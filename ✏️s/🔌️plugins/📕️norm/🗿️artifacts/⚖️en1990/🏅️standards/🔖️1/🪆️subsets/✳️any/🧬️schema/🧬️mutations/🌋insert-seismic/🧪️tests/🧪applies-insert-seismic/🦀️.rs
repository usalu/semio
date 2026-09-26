#[semio_framework_async_macros::async_test]
async fn applies_insert_seismic() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::InsertSeismic(crate::standards::v1::subsets::any::schema::mutations::insert_seismic::InsertSeismic {
        index: base.seismics.len(), item: crate::SeismicAction { id: "E-new".into(), a_ek: 0.0, importance_class: crate::ImportanceClass::II },
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.seismics.len(), base.seismics.len() + 1);
}
