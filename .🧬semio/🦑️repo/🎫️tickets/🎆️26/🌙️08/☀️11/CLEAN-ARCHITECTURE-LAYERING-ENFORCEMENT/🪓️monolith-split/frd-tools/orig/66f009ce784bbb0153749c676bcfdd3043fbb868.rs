//! 🔗️ Exact native graph admission preserves referential invariants before a snapshot can reach its codec.
use super::*;
use semio_framework_value::{FromValue, DslValue as Value};
use protocol::MutationDiff;

#[test]
fn history_edit_step_reference_closure_preserves_cycles_and_refuses_invalid_candidate_before_serialization() {
    let corpus=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/📸️snapshot/🔗️references/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let cases=corpus.get("cases").and_then(Value::as_array).unwrap();
    assert_eq!(cases.len(),8);
    for case in cases {
        let entities=Vec::<crate::standards::v_ap214::subsets::base::schema::snapshot::StepEntity>::from_value(case.get("entities").unwrap().clone()).unwrap();
        let snapshot=StepSnapshot{entities,..StepSnapshot::default()};
        let original=snapshot.clone();
        let result=validate_step_references(&snapshot);
        let expected=case.get("error").unwrap();
        if matches!(expected,Value::Null) {assert!(result.is_ok(),"{}",case.get("id").and_then(Value::as_str).unwrap());} else {
            let error=result.unwrap_err();
            assert_eq!(error.code,expected.get("code").and_then(Value::as_str).unwrap());
            assert_eq!(error.target,Vec::<String>::from_value(expected.get("target").unwrap().clone()).unwrap());
        }
        assert_eq!(snapshot,original);
    }
    let base=StepSnapshot::default();
    let diff=crate::standards::v_ap214::subsets::base::schema::diff::StepDiff{entities:Some(crate::standards::v_ap214::subsets::base::schema::diff::StepEntitiesDiff{added:vec![crate::standards::v_ap214::subsets::base::schema::diff::StepEntityAdded{index:0,entity:crate::schema::snapshot::StepEntity{id:10,name:"THING".into(),args:vec![StepValue::Reference(99)],complex:Vec::new()}}],..Default::default()}),..Default::default()};
    let error=protocol::apply_diff(&diff, &base).unwrap_err();
    assert_eq!(error.code,"mutation.apply.dangling-reference");
    assert_eq!(error.target,["entities","10","args","0"]);
    assert_eq!(base,StepSnapshot::default());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
