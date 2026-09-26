#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/❌️failing-under-reinforced/❌️failing-under-reinforced/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <crate::En1992Snapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse hierarchical example");
    assert!(!snap.members.is_empty());
    assert!(crate::standards::v1::subsets::any::schema::inferences::evaluate(&snap).failing().count() >= 2);
}
