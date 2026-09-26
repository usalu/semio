#[semio_framework_async_macros::async_test]
async fn applies_remove_effect() {
    let base = crate::En1990Snapshot::default();
    if base.effects.is_empty() { return; }
    let mutation = crate::En1990Mutation::RemoveEffect(crate::standards::v1::subsets::any::schema::mutations::remove_effect::RemoveEffect { index: 0 });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.effects.len(), base.effects.len() - 1);
}
