#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = super::PRIMARY_TEXT;
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_to_nonconforming_valve_dataset() {
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    use semio_s_artifact_norm_vdi3805_model::nonconforming_valve_dataset;
    let text = super::PRIMARY_TEXT;
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

#[semio_framework_async_macros::async_test]
async fn primary_asset_fails_named_checks_with_remedies() {
    use semio_s_artifact_norm_vdi3805_inferences::evaluate;
    use semio_s_artifact_norm_vdi3805_io::text::snapshot::decode_vdi3805_dsl;
    let report = evaluate(&decode_vdi3805_dsl(super::PRIMARY_TEXT).expect("nonconforming dsl"));
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 5, "expected >=5 fails, got {}", fails.len());
    for fail in &fails {
        assert!(!fail.remedies.is_empty(), "fail {} must carry a remedy", fail.id);
    }
    assert!(fails.iter().any(|c| c.id.contains("dn")));
    assert!(fails.iter().any(|c| c.id.contains("kvs")));
    assert!(fails.iter().any(|c| c.id.contains("structure")));
    assert!(fails.iter().any(|c| c.id.contains("historical")));
}
