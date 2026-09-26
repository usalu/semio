use super::super::*;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use store::ArtifactDsl;

#[test]
fn example_id_is_stable() {
    assert!(!ID.is_empty());
    let _ = label();
    assert!(!PRIMARY_TEXT.is_empty());
}

#[test]
fn demo_dsl_decodes_and_complies() {
    let document = <crate::Iso16757Snapshot as ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("decode demo DSL");
    let report = evaluate(&document);
    assert!(report.complies(), "demo must comply: {:?}", report.failing().map(|c| c.id.as_str()).collect::<Vec<_>>());
}
