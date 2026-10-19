mod tests {
    /// 🧪️ The store type aliases live in `semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations` (`Fem2dEnvelope`/`Fem2dStore`) and
    /// are exercised by that node's own tests plus `semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::binary::mutations`'s
    /// `fem2d_document_text_round_trips_through_the_store`.
    #[test]
    fn fem2d_store_type_alias_constructs_from_an_empty_envelope() {
        let mut store = ::semio_framework_async::poll::resolve_ready(semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(store::os_store::document_materialize::create_document_envelope(
            semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA,
            "fem2d",
            semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot(),
            None,
         &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())))
        .expect("valid store");
        store.install_document_store_owners_exact(semio_framework_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot, semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
        assert!(store.snapshot().expect("snapshot").nodes.is_empty());
        store.close_owned_unscheduled().expect("fem2d document store closes through its exact bounded owners");
    }
}
