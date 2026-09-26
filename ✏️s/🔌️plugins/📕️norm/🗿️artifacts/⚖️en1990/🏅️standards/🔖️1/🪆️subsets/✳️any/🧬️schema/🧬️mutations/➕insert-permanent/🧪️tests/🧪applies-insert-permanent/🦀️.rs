#[semio_framework_async_macros::async_test]
async fn applies_insert_permanent() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::InsertPermanent(crate::standards::v1::subsets::any::schema::mutations::insert_permanent::InsertPermanent {
        index: base.permanents.len(), item: crate::PermanentAction { id: "G-new".into(), kind: "g_sup".into(), gk: 0.0 },
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.permanents.len(), base.permanents.len() + 1);
}
