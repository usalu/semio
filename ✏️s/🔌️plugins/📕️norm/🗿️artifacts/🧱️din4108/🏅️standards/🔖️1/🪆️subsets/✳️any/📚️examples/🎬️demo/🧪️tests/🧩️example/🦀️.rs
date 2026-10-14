use semio_s_artifact_norm_din4108_inferences::evaluate;
use semio_s_artifact_norm_din4108_io::text::snapshot::decode_din4108_dsl;

#[semio_framework_async_macros::async_test]
async fn demo_dsl_decodes_and_complies() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snap = decode_din4108_dsl(text).expect("demo dsl");
    let report = evaluate(&snap);
    assert!(report.complies(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_din4108_inferences::Din4108Inference;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    use protocol::Inference;
    let snapshot = Din4108Snapshot::default();
    assert_eq!(Din4108Inference::infer(&snapshot).expect("valid materialized inference fixture"), Din4108Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_norm_din4108_inferences::Din4108Inference;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    use protocol::Inference;
    assert_eq!(Din4108Inference::infer(&Din4108Snapshot::default()).expect("valid materialized inference fixture"), Din4108Inference::default());
}
