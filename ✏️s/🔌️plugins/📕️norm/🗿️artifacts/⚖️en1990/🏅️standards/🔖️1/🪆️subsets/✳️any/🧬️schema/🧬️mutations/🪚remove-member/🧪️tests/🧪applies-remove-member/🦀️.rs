#[semio_framework_async_macros::async_test]
async fn applies_remove_member() {
    let base = crate::En1990Snapshot::default();
    if base.members.is_empty() { return; }
    let mutation = crate::En1990Mutation::RemoveMember(crate::standards::v1::subsets::any::schema::mutations::remove_member::RemoveMember { index: 0 });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.members.len(), base.members.len() - 1);
}
