#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = crate::demo::PRIMARY_TEXT;
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_to_conforming_valve_dataset() {
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_artifact_norm_vdi3805_model::{conforming_valve_dataset, reference_fixture};
    let text = crate::demo::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("demo dsl");
    assert_eq!(decoded, conforming_valve_dataset());
    assert_eq!(decoded, reference_fixture());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_vdi3805_inferences::Vdi3805Inference;
    use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
    use protocol::Inference;
    let snapshot = Vdi3805Snapshot::default();
    assert_eq!(Vdi3805Inference::infer(&snapshot).expect("valid materialized inference fixture"), Vdi3805Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_norm_vdi3805_inferences::Vdi3805Inference;
    use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
    use protocol::Inference;
    assert_eq!(Vdi3805Inference::infer(&Vdi3805Snapshot::default()).expect("valid materialized inference fixture"), Vdi3805Inference::default());
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_complies() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_plugin_norm_compliance::CheckStatus;
    let report = evaluate(&decode_vdi3805_dsl(crate::demo::PRIMARY_TEXT).expect("demo dsl"));
    assert!(report.complies(), "fails={:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    assert!(report.checks.iter().any(|c| c.id.starts_with("vdi3805.1.structure") && c.status == CheckStatus::Pass));
    assert!(report.checks.iter().any(|c| c.id.contains("vdi3805.2.kvs") && c.status == CheckStatus::Pass));
}
