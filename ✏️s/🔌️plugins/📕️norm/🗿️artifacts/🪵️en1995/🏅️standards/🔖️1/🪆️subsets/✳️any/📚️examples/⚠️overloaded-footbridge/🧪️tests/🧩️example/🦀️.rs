use crate::artifact_schema::{decode_en1995_dsl, evaluate_structure};
use crate::En1995Snapshot;

fn decoded() -> En1995Snapshot {
    decode_en1995_dsl(crate::overloaded_footbridge::PRIMARY_TEXT).expect("overloaded-footbridge DSL decodes")
}

#[test]
fn primary_asset_decodes_to_the_noncompliant_bridge() {
    assert_eq!(decoded(), En1995Snapshot::noncompliant_bridge());
}

#[test]
fn decoded_example_fails_bridge_checks_with_remedies() {
    let snap = decoded();
    let report = evaluate_structure(snap.annex, &snap.members, &snap.connections);
    let failing: Vec<_> = report.failing().collect();
    assert!(failing.len() >= 2, "ids={:?}", failing.iter().map(|c| c.id.clone()).collect::<Vec<_>>());
    for check in &failing {
        assert!(check.id.starts_with("en1995.2.") && check.id.ends_with(".bridge-G2"), "unexpected failing id {}", check.id);
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
    assert!(failing.iter().any(|c| c.id == "en1995.2.b.avert.bridge-G2"));
}
