use crate::artifact_schema::{decode_en1995_dsl, evaluate_structure};
use crate::document::CheckReport;
use crate::En1995Snapshot;

fn decoded() -> En1995Snapshot {
    decode_en1995_dsl(crate::multi_fail_timber::PRIMARY_TEXT).expect("multi-fail-timber DSL decodes")
}

fn report() -> CheckReport {
    let snap = decoded();
    evaluate_structure(snap.annex, &snap.members, &snap.connections)
}

#[test]
fn primary_asset_decodes_to_the_noncompliant_building() {
    assert_eq!(decoded(), En1995Snapshot::noncompliant_building());
}

#[test]
fn decoded_example_fails_several_checks_with_expected_ids() {
    let report = report();
    let failing: Vec<String> = report.failing().map(|c| c.id.clone()).collect();
    assert!(failing.len() >= 2, "fail_count={} ids={failing:?}", failing.len());
    for pattern in [
        "en1995.6.1.6.bending.beam-B2",
        "en1995.6.1.7.shear.beam-B2",
        "en1995.6.3.2.compression.col-C1",
        "en1995.8.spacing.a1.conn-C2",
        "en1995.8.spacing.a4t.conn-C2",
    ] {
        assert!(failing.iter().any(|id| id == pattern), "{pattern} must fail, failing={failing:?}");
    }
    for id in &failing {
        assert!(
            ["en1995.6.", "en1995.7.", "en1995.8.", "en1995.1-2."].iter().any(|p| id.starts_with(p)),
            "unexpected failing id {id}"
        );
    }
}

#[test]
fn every_failure_carries_a_remedy() {
    for check in report().failing() {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
