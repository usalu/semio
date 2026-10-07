//! 📝️ Physical text diff representation.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::En1998Diff);

#[cfg(test)]
#[test]
fn history_edit_en1998_empty_diff_records_agree_with_the_neutral_json_oracle() {
    use protocol::{DiffBinary, DiffText};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🔺️diff/🧫️fixtures/🔣️.json")).unwrap();
    let expected: crate::artifact_schema::diff::En1998Diff = serde_json::from_value(corpus["empty"].clone()).unwrap();
    let text = expected.print_diff();
    let actual = crate::artifact_schema::diff::En1998Diff::parse_diff(&text).unwrap();
    assert_eq!(actual, expected);
    let binary = expected.encode_diff().unwrap();
    assert_eq!(crate::artifact_schema::diff::En1998Diff::decode_diff(&binary).unwrap(), expected);
}
