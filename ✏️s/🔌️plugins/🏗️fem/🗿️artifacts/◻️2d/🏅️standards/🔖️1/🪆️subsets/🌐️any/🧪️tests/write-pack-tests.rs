//! 🧪️ Actual persistent FEM Pack publication uses the shared original codec and final wire owner.
use super::*;

#[test]
fn actual_fem_native_pack_publication_matches_the_canonical_reference_document() {
    let source = Fem2dSnapshot::default();
    let expected = pack::snapshot::ArtifactPack::encode_pack(&source);
    let payload = original_owners::with_encoding(|owner| {
        let mut published = None;
        let turn = source.encode_exchange_pack(&mut published, owner);
        assert!(turn.work <= 2_000_000);
        turn.result.unwrap();
        assert!(owner.progress().copied_items > 0);
        published.unwrap()
    });
    let store::io_schema::IoPayload::Binary(wire) = &payload else { panic!("binary native Pack") };
    assert_eq!(wire, &expected);
    semio_s_fem_receiving::testkit::retire((source, payload));
}
