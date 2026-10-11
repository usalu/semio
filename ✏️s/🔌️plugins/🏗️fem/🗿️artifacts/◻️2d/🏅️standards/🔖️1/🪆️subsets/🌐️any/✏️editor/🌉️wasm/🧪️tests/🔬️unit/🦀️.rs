mod tests {
    /// 🧪️ The store type aliases live in `crate::standards::v1::subsets::any::schema::mutations` (`Fem2dEnvelope`/`Fem2dStore`) and
    /// are exercised by that node's own tests plus `crate::standards::v1::subsets::any::io::binary::mutations`'s
    /// `fem2d_document_text_round_trips_through_the_store`.
    #[test]
    fn fem2d_store_type_alias_constructs_from_an_empty_envelope() {
        let mut store = ::semio_framework_async::poll::resolve_ready(crate::standards::v1::subsets::any::schema::mutations::Fem2dStore::new(store::create_document_envelope(
            crate::FEM_2D_SCHEMA,
            "fem2d",
            crate::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot(),
            None,
        ), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())))
        .expect("valid store");
        store.install_document_store_owners_exact(semio_framework_os_kernel::os_store::funded_bounded_artifact_store_owners::<crate::Fem2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
        assert!(store.snapshot().expect("snapshot").nodes.is_empty());
        while !store.close_owned_terminal_is_empty() {
            store.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("fem2d document store closes through its exact bounded owners");
        }
    }
}
