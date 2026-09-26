#[semio_framework_async_macros::async_test]
async fn applies_remove_accidental() {
    let base = crate::En1990Snapshot::default();
    if base.accidentals.is_empty() { return; }
    let mutation = crate::En1990Mutation::RemoveAccidental(crate::standards::v1::subsets::any::schema::mutations::remove_accidental::RemoveAccidental { index: 0 });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.accidentals.len(), base.accidentals.len() - 1);
}
