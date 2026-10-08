use super::*;

#[test]
fn flow_diff_codecs_preserve_the_neutral_schema_and_child_handles() {
    use protocol::{DiffBinary, DiffText};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️codec/🔣️.json")).expect("neutral Flow diff codec fixture");
    let cases = fixture["cases"].as_array().expect("diff cases");
    assert_eq!(cases.len(), 4);
    for expected in cases {
        let independent = serde_json::to_string(expected).expect("independent JSON encoder");
        let diff: FlowDiff = semio_framework_pack_json::from_json_str(&independent, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("schema-first diff input");
        let text = diff.print_diff();
        let text_diff = FlowDiff::parse_diff(&text).expect("text diff decoder");
        let binary = diff.encode_diff().expect("binary diff encoder");
        let binary_diff = FlowDiff::decode_diff(&binary).expect("binary diff decoder");
        assert_eq!(text_diff, diff);
        assert_eq!(binary_diff, diff);
        assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&text_diff)), *expected);
        assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&binary_diff)), *expected);
    }
    println!("[DEBUG] Flow diff text and binary preserve all four independent JSON cases");
}

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_content_diff() {
    let base = FlowSnapshot::default();
    let mut replacement = base.clone();
    replacement.schema = "flow.replaced".into();
    let mut diff = diff_replace_content(Vec::new(), Vec::new(), Default::default());
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
}
