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
fn broken_dsl_decodes_and_fails() {
    let document = <crate::Iso16757Snapshot as ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("decode broken DSL");
    let report = evaluate(&document);
    assert!(!report.complies(), "broken must not comply");
    assert!(report.failing().count() >= 2, "broken must yield ≥2 fails");
    assert!(
        report.failing().all(|c| c.remedies.iter().any(|r| r.applicable)),
        "every broken fail needs an applicable remedy"
    );
}
