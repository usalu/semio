use super::*;

#[test]
fn rejects_out_of_range_bearing_widths_before_target_resolution() {
    let base = semio_framework_pack_json::from_json_str::<En1992Snapshot>(include_str!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/💧change-member-bearing-width/✅apply/🔣️.before.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native before fixture");
    let wire: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/💧change-member-bearing-width/⛔range/🔣️.mutation.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("negative wire fixture");
    let payload: ChangeMemberBearingWidth = semio_framework_value::FromValue::from_value(wire.get("ChangeMemberBearingWidth").expect("mutation tag").clone()).expect("native mutation payload");
    assert!(matches!(diff(&payload, &base).admit(), protocol::MutationAdmission::Refused(_)));
    for value in [-0.1, 1.5, f64::INFINITY, f64::NAN] {
        let payload = ChangeMemberBearingWidth { member_id: "missing".into(), new_value: value };
        let outcome = diff(&payload, &base);
        assert!(outcome.messages().iter().any(|message| message.code.as_str() == "mutation.invariant"));
        assert!(matches!(outcome.admit(), protocol::MutationAdmission::Refused(_)));
    }
}
