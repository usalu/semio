//! 🏋️ Complete EN1991 snapshot capability and independent domain evidence.

#[test]
fn sqlite_snapshot_en1991_actual_bare_factory_exposes_owned_relational_capability() {
    let codec = store::ArtifactCodec::bare::<super::En1991Snapshot, crate::En1991Mutation>(crate::EN1991_DOCUMENT_SCHEMA);
    assert!(codec.snapshot_sqlite.is_some(), "EN1991 has no owned relational capability");
}
