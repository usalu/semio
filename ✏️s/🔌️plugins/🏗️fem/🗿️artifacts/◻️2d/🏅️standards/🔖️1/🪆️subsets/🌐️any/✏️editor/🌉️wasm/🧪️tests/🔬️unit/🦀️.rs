
/// 🧪️ The store type aliases live in `crate::standards::v1::subsets::any::schema::mutations` (`Fem2dEnvelope`/`Fem2dStore`) and
/// are exercised by that node's own tests plus `crate::standards::v1::subsets::any::schema::mutations::binary`'s
/// `fem2d_document_text_round_trips_through_the_store`.
#[test]
fn fem2d_store_type_alias_constructs_from_an_empty_envelope() {
    let store = semio_framework_plugin::resolve_ready(crate::standards::v1::subsets::any::schema::mutations::Fem2dStore::new(store::create_document_envelope(crate::FEM_2D_SCHEMA, "fem2d", crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot(), None))).expect("valid store");
    assert!(store.snapshot().expect("snapshot").nodes.is_empty());
}
