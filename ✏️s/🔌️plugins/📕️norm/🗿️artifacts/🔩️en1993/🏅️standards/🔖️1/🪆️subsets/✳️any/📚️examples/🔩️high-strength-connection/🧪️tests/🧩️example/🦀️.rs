#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_fails_compliance() {
    use semio_s_artifact_norm_en1993_checks::check_full_steel_structure;
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;
    use pack::snapshot::ArtifactDsl;
    let text = include_str!("../../../../🖼️assets/🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio");
    let snapshot = En1993Snapshot::parse_dsl(text).expect("decode non-compliant DSL");
    let report = check_full_steel_structure(&snapshot);
    assert!(!report.complies());
    assert!(report.failing().count() >= 2, "expected ≥2 fails, got {}", report.failing().count());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_en1993_inferences::En1993Inference;
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;
    use protocol::Inference;
    let snapshot = En1993Snapshot::default();
    assert_eq!(En1993Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1993Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_norm_en1993_inferences::En1993Inference;
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;
    use protocol::Inference;
    assert_eq!(En1993Inference::infer(&En1993Snapshot::default()).expect("valid materialized inference fixture"), En1993Inference::default());
}
