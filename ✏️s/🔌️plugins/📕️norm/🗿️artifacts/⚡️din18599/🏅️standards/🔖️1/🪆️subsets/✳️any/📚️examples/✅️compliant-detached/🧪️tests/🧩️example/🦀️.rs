#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_evaluates_claimed_verdict() {
    use semio_s_artifact_norm_din18599_checks::evaluate_document;
    use pack::snapshot::ArtifactDsl;
    let text = crate::compliant_detached::PRIMARY_TEXT;
    let doc = <semio_s_artifact_norm_din18599_model::Din18599Snapshot as ArtifactDsl>::parse_dsl(text).expect("parse compliant-detached");
    let report = evaluate_document(&doc);
    assert!(report.complies(), "fails={:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_din18599_inferences::Din18599Inference;
    use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
    use protocol::Inference;
    let snapshot = Din18599Snapshot::default();
    assert_eq!(Din18599Inference::infer(&snapshot).expect("valid materialized inference fixture"), Din18599Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_norm_din18599_inferences::Din18599Inference;
    use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
    use protocol::Inference;
    assert_eq!(Din18599Inference::infer(&Din18599Snapshot::default()).expect("valid materialized inference fixture"), Din18599Inference::default());
}
