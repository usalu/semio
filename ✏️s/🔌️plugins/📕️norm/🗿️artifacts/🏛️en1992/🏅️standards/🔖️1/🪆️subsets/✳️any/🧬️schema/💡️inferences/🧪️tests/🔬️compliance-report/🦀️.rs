use semio_s_plugin_norm_compliance::CheckStatus;
use crate::evaluate;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[test]
fn default_evaluate_covers_parts() {
    let report = evaluate(&En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(report.checks.len() >= 10);
    let parts: std::collections::BTreeSet<_> = report.checks.iter().map(|c| c.part.as_str()).collect();
    assert!(parts.iter().any(|p| p.contains("1992-1-1")));
}

#[test]
fn failing_example_not_compliant() {
    let report = evaluate(&En1992Snapshot::failing_under_reinforced());
    assert!(!report.complies());
    assert!(report.summary.fail >= 3);
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::Fail && !c.remedies.is_empty()));
}

fn stations_of<'a>(report: &'a semio_s_plugin_norm_compliance::CheckReport, prefix: &str) -> Vec<&'a semio_s_plugin_norm_compliance::CheckResult> {
    report.checks.iter().filter(|c| c.id.starts_with(prefix) && !c.stations.is_empty()).collect()
}

#[test]
fn flexure_shear_crack_and_deflection_emit_member_stations_that_follow_the_closed_forms() {
    let report = evaluate(&En1992Snapshot::compliant_office_frame());
    for prefix in ["en1992.6.1.flexure.", "en1992.6.2.shear.", "en1992.7.3.crack.", "en1992.7.4.deflection."] {
        let checks = stations_of(&report, prefix);
        assert!(!checks.is_empty(), "{prefix} has no stations");
        for c in checks {
            assert!(c.stations_consistent(), "{} breaks the station law", c.id);
            assert_eq!(c.stations.len(), 11);
            assert!(c.station_field.is_empty());
        }
    }
    let flexure = stations_of(&report, "en1992.6.1.flexure.")[0];
    let (lo, hi) = flexure.stations.iter().fold((f64::MAX, 0.0_f64), |(lo, hi), s| (lo.min(s.utilization), hi.max(s.utilization)));
    assert!((hi - flexure.utilization).abs() < 1e-9 && lo < 0.9 * hi);
}

#[test]
fn punching_emits_a_control_perimeter_field() {
    let mut doc = En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.members.retain(|m| matches!(m.kind, semio_s_artifact_norm_en1992_model::MemberKind::FlatSlab));
    let report = evaluate(&doc);
    for c in report.checks.iter().filter(|c| c.id.starts_with("en1992.6.4.punching.")) {
        assert_eq!(c.station_field, "punching-perimeter");
        assert!(c.stations.len() >= 8 && c.stations.iter().all(|s| s.at.len() == 2));
        assert!(c.stations_consistent());
    }
}
