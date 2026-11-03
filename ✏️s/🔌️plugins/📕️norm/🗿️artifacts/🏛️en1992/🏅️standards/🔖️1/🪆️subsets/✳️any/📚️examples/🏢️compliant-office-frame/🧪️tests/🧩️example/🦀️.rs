#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🏢compliant-office-frame/🏢compliant-office-frame/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <semio_s_artifact_norm_en1992_model::En1992Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("parse hierarchical example");
    assert!(!snap.members.is_empty());
    assert_eq!(semio_s_artifact_norm_en1992_inferences::evaluate(&snap).failing().count(), 0, "no fail; unverified NDP basis keeps the verdict from being a compliance claim");
}
