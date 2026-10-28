//! 🧪️ Portable CMap endpoint and supplementary-plane witnesses.
use super::*;
#[test]
fn pdf_unicode_cmap_ranges_cover_scalar_planes_without_surrogates() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    for row in vectors["ranges"].as_array().unwrap() { assert_eq!(unicode_range(row["at"].as_u64().unwrap() as usize).unwrap().to_string(), format!("1 beginbfrange\n<{}> <{}> <{}>\nendbfrange\n", row["from"].as_str().unwrap(), row["to"].as_str().unwrap(), row["target"].as_str().unwrap())); }
    assert!(unicode_range(1026).is_none());
    for row in vectors["texts"].as_array().unwrap() { for scalar in row.as_str().unwrap().chars() { let code=scalar as u32;let at=if code<0xe000{0}else if code<=0xffff{1}else{2+(code as usize-0x10000)/1024};assert!(unicode_range(at).is_some()); } }
}
