//! 🧪️ Portable editable-font property byte laws.
use super::*;
#[test]
fn pdf_text_properties_preserve_unicode_family_and_scalar_style() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        assert_eq!(case["family"].as_str().unwrap().chars().map(|value|font_family_scalar(value).to_string()).collect::<String>(), case["hex"].as_str().unwrap());
        assert_eq!(text_properties(case["weight"].as_u64().unwrap() as u32, case["italic"].as_bool().unwrap()).to_string(), format!("> /SemioFontWeight {} /SemioItalic {} >> BDC\n", case["weight"], case["italic"]));
    }
}
