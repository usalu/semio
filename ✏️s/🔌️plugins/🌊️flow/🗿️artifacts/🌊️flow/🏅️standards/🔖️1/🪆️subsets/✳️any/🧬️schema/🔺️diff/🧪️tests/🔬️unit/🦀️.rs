use super::*;

#[test]
fn flow_diff_codecs_preserve_the_neutral_schema_and_child_handles() {
    use protocol::{DiffBinary, DiffText};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️codec/🔣️.json")).expect("neutral Flow diff codec fixture");
    let cases = fixture["cases"].as_array().expect("diff cases");
    assert_eq!(cases.len(), 3);
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
    println!("[DEBUG] Flow diff text and binary preserve all three independent JSON cases");
}

#[semio_framework_async_macros::async_test]
async fn a_diff_applies_slot_wise_inverts_exactly_and_absorbs_later_slots_over_earlier_ones() {
    use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law};
    use protocol::DiffAlgebra;
    let base = FlowSnapshot::default();
    let renamed = FlowDiff { schema: Some("flow.renamed".into()), ..Default::default() };
    let rescened = diff_replace_content(Vec::new(), Vec::new(), Default::default());
    let mut sum = renamed.clone();
    sum.absorb(rescened.clone());
    sum.absorb(FlowDiff { schema: Some("flow.final".into()), ..Default::default() });
    let after = protocol::apply_diff(&sum, &base).expect("valid parent diff");
    assert_eq!(after.schema, "flow.final");
    assert_eq!(Some(&after.content), rescened.content.as_ref());
    assert_diff_algebra_inverse_law(&base, &sum).await;
    assert_eq!(protocol::apply_diff(&sum.inverse(&base), &after).expect("valid inverse diff"), base);
    assert!(FlowDiff::default().is_empty() && !renamed.is_empty());
}
