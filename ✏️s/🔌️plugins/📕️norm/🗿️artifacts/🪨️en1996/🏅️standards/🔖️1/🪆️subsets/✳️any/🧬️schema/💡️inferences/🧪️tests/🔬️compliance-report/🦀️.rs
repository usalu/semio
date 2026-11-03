#[test]
fn evaluate_default_complies() {
    let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = crate::evaluate(&doc);
    assert!(report.checks.len() >= 5);
    assert!(report.complies());
}

#[test]
fn evaluate_noncompliant_has_fails() {
    let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::noncompliant_multi_fail();
    let report = crate::evaluate(&doc);
    assert!(!report.complies());
    assert!(report.summary.fail >= 3);
}

#[test]
fn compression_emits_phi_stations_at_the_bottom_mid_and_top_sections_and_flexure_a_height_profile() {
    let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::noncompliant_multi_fail();
    let report = crate::evaluate(&doc);
    let compression: Vec<_> = report.checks.iter().filter(|c| c.id.starts_with("en1996.6.1.2.compression.") && !c.id.contains(".mid.")).collect();
    assert!(!compression.is_empty());
    for c in compression {
        assert_eq!(c.station_field, "wall-phi");
        assert!(c.stations_consistent(), "{}", c.id);
        let at: Vec<f64> = c.stations.iter().map(|s| s.at[0]).collect();
        assert_eq!(at, vec![0.0, 0.5, 1.0]);
        assert!(c.stations.iter().all(|s| s.values.len() == 3 && s.values[2].value > 0.0 && s.values[2].value <= 1.0));
        let n_bottom = c.stations[0].values[0].value;
        let n_top = c.stations[2].values[0].value;
        assert!(n_bottom >= n_top, "the self-weight makes the normal force grow towards the base");
    }
    for c in report.checks.iter().filter(|c| c.id.starts_with("en1996.6.3.flexure.") && !c.stations.is_empty()) {
        assert_eq!(c.station_field, "wall-lateral");
        assert!(c.stations_consistent());
        assert_eq!(c.stations.len(), 11);
        let (first, mid) = (c.stations[0].utilization, c.stations[5].utilization);
        assert!(first.abs() < 1e-12 && (mid - c.utilization).abs() < 1e-9);
        let quarter = c.stations[2].utilization;
        assert!((quarter - mid * 4.0 * 0.2 * 0.8).abs() < 1e-9);
    }
}

/// 🔮️ EN 1996 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and independent formulations and fails on any mismatch beyond the tolerance profile or between references.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 3, "expected at least 3 verified oracle cases, got {verified}");
}
