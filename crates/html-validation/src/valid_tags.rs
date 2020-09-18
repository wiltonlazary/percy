use lazy_static::lazy_static;
use std::collections::hash_set::HashSet;

use super::svg_namespace::is_svg_namespace;

lazy_static! {
    static ref VALID_TAGS: HashSet<&'static str> = [
        "a","abbr","address","area","article","aside","audio","b","base","bdi","bdo","big",
        "blockquote","body","br","button","canvas","caption","cite","code","col","colgroup",
        "command", "data","datalist","dd","del","details","dfn","dialog","div","dl","dt","em","embed",
        "fieldset","figcaption","figure","footer","form","h1","h2","h3","h4","h5","h6","head",
        "header","hr","html","i","iframe","img","input","ins","kbd","keygen","label","legend",
        "li","link","main","map","mark","menu","menuitem","meta","meter","nav","noscript","object",
        "ol","optgroup","option","output","p","param","picture","pre","progress","q","rp","rt",
        "ruby","s","samp","script","section","select","small","source","span","strong","style",
        "sub","summary","sup","table","tbody","td","textarea","tfoot","th","thead","time","title",
        "tr","track","u","ul","var","video","wbr",
    ]
    .iter()
    .cloned()
    .collect();
}

/// Whether or not this tag is valid
///
/// ```
/// use html_validation::is_valid_tag;
///
/// assert_eq!(is_valid_tag("br"), true);
///
/// assert_eq!(is_valid_tag("random"), false);
/// ```
pub fn is_valid_tag(tag: &str) -> bool {
    VALID_TAGS.contains(tag) || is_svg_namespace(tag)
}
