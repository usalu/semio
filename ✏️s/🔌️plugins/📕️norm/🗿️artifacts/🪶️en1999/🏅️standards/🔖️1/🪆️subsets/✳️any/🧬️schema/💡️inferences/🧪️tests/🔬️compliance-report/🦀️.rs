//! 🔬️ evaluate() report shape for hierarchical EN 1999 subject.

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::evaluate;

#[test]
fn evaluate_emits_member_and_connection_checks() {
    let report = evaluate(&En1999Snapshot::compliant_roof_purlin());
    assert!(report.checks.len() >= 8);
    assert!(report.checks.iter().any(|c| c.id.contains("6.3.1")));
    assert!(report.complies(), "worst={} fails={:?}", report.worst_utilization(), report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[test]
fn noncompliant_evaluate_fails() {
    let report = evaluate(&En1999Snapshot::noncompliant_multi_fail());
    assert!(!report.complies());
    assert!(report.failing().next().unwrap().remedies.len() >= 1);
}

#[test]
fn member_checks_emit_stations_that_follow_the_support_and_load() {
    let mut document = En1999Snapshot::compliant_roof_purlin();
    document.members[0].support = semio_s_artifact_norm_en1999_model::SupportCondition::SimplySupported;
    let report = evaluate(&document);
    let bending = report.checks.iter().find(|c| c.id.starts_with("en1999.6.2.5.m.")).expect("bending check");
    assert_eq!(bending.stations.len(), 11);
    assert!(bending.stations_consistent());
    if document.members[0].actions.iter().any(|a| a.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl) {
        assert!(bending.stations[0].utilization < 1e-12 && bending.stations[10].utilization < 1e-12);
        let mid = &bending.stations[5];
        assert!((mid.utilization - bending.utilization).abs() < 1e-9);
    }
    let sampled = report.checks.iter().filter(|c| c.id.starts_with("en1999.6.") && !c.stations.is_empty()).count();
    assert!(sampled >= 5, "resistance, shear, buckling and interaction checks carry stations");
    assert!(report.checks.iter().all(|c| c.stations.is_empty() || c.stations_consistent()));
}
