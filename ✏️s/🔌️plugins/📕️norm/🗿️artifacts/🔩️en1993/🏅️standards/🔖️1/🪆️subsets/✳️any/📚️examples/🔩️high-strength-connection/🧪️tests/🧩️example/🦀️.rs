#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_fails_compliance() {
    use crate::standards::v1::subsets::any::schema::check_full_steel_structure;
    use crate::En1993Snapshot;
    use store::ArtifactDsl;
    let text = include_str!("../../../../🖼️assets/🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio");
    let snapshot = En1993Snapshot::parse_dsl(text).expect("decode non-compliant DSL");
    let report = check_full_steel_structure(&snapshot);
    assert!(!report.complies());
    assert!(report.failing().count() >= 2, "expected ≥2 fails, got {}", report.failing().count());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::En1993Inference;
    use crate::En1993Snapshot;
    use protocol::Inference;
    let snapshot = En1993Snapshot::default();
    assert_eq!(En1993Inference::infer(&snapshot), En1993Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifact_schema::inferences::En1993Inference;
    use crate::En1993Snapshot;
    use protocol::Inference;
    assert_eq!(En1993Inference::infer(&En1993Snapshot::default()), En1993Inference::default());
}
