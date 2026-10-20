#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = semio_s_artifact_norm_vdi3805_examples::nonconforming::PRIMARY_TEXT;
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_to_nonconforming_valve_dataset() {
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_artifact_norm_vdi3805_model::nonconforming_valve_dataset;
    let text = semio_s_artifact_norm_vdi3805_examples::nonconforming::PRIMARY_TEXT;
    let decoded = decode_vdi3805_dsl(text).expect("nonconforming dsl");
    assert_eq!(decoded, nonconforming_valve_dataset());
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
