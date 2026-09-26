#[semio_framework_async_macros::async_test]
async fn applies_remove_variable() {
    let base = crate::En1990Snapshot::default();
    if base.variables.is_empty() { return; }
    let mutation = crate::En1990Mutation::RemoveVariable(crate::standards::v1::subsets::any::schema::mutations::remove_variable::RemoveVariable { index: 0 });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.variables.len(), base.variables.len() - 1);
}
