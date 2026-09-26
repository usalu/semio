#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_and_evaluates_claimed_verdict() {
    use crate::standards::v1::subsets::any::schema::evaluate_document;
    use store::ArtifactDsl;
    let text = crate::examples::demo::PRIMARY_TEXT;
    let doc = <crate::Din18599Snapshot as ArtifactDsl>::parse_dsl(text).expect("parse demo");
    let report = evaluate_document(&doc);
    assert!(report.complies(), "fails={:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::Din18599Inference;
    use crate::Din18599Snapshot;
    use protocol::Inference;
    let snapshot = Din18599Snapshot::default();
    assert_eq!(Din18599Inference::infer(&snapshot), Din18599Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifact_schema::inferences::Din18599Inference;
    use crate::Din18599Snapshot;
    use protocol::Inference;
    assert_eq!(Din18599Inference::infer(&Din18599Snapshot::default()), Din18599Inference::default());
}
