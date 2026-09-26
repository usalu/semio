#[semio_framework_async_macros::async_test]
async fn applies_insert_accidental() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::InsertAccidental(crate::standards::v1::subsets::any::schema::mutations::insert_accidental::InsertAccidental {
        index: base.accidentals.len(), item: crate::AccidentalAction { id: "A-new".into(), ad: 0.0 },
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.accidentals.len(), base.accidentals.len() + 1);
}
