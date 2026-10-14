#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/❌️failing-under-reinforced/❌️failing-under-reinforced/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <semio_s_artifact_norm_en1992_model::En1992Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("parse hierarchical example");
    assert!(!snap.members.is_empty());
    assert!(semio_s_artifact_norm_en1992_inferences::evaluate(&snap).failing().count() >= 2);
}
