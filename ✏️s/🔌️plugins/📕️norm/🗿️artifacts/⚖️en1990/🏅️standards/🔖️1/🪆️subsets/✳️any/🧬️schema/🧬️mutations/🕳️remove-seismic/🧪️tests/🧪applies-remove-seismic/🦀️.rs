#[semio_framework_async_macros::async_test]
async fn applies_remove_seismic() {
    let base = crate::En1990Snapshot::default();
    if base.seismics.is_empty() { return; }
    let mutation = crate::En1990Mutation::RemoveSeismic(crate::standards::v1::subsets::any::schema::mutations::remove_seismic::RemoveSeismic { index: 0 });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.seismics.len(), base.seismics.len() - 1);
}
