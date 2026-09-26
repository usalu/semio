#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_to_conforming_valve_dataset() {
    use crate::standards::v1::subsets::any::schema::snapshot::decode_vdi3805_dsl;
    use crate::{conforming_valve_dataset, reference_fixture};
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let decoded = decode_vdi3805_dsl(text).expect("demo dsl");
    assert_eq!(decoded, conforming_valve_dataset());
    assert_eq!(decoded, reference_fixture());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::Vdi3805Inference;
    use crate::Vdi3805Snapshot;
    use protocol::Inference;
    let snapshot = Vdi3805Snapshot::default();
    assert_eq!(Vdi3805Inference::infer(&snapshot), Vdi3805Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifact_schema::inferences::Vdi3805Inference;
    use crate::Vdi3805Snapshot;
    use protocol::Inference;
    assert_eq!(Vdi3805Inference::infer(&Vdi3805Snapshot::default()), Vdi3805Inference::default());
}
