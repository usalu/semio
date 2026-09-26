#[semio_framework_async_macros::async_test]
async fn applies_remove_permanent() {
    let base = crate::En1990Snapshot::default();
    if base.permanents.is_empty() { return; }
    let mutation = crate::En1990Mutation::RemovePermanent(crate::standards::v1::subsets::any::schema::mutations::remove_permanent::RemovePermanent { index: 0 });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.permanents.len(), base.permanents.len() - 1);
}
