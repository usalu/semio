//! 🧪️ Every bundled example is a real, decodable, non-empty document the picker can offer.

use crate::schema::snapshot::Grid2dSnapshot;

#[test]
fn the_registry_offers_two_distinct_examples() {
    let sources = super::sources();
    assert_eq!(sources.len(), 2);
    assert_ne!(sources[0].id(), sources[1].id());
    assert_eq!(super::example_source_slice().len(), 2);
}

#[test]
fn every_example_decodes_into_a_document_with_tiles_and_rules() {
    for source in super::sources() {
        let id = source.id().to_string();
        assert!(!source.document().trim().is_empty(), "{id}");
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(&source.document()).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(document.schema, crate::WFC_GRID2D_DOCUMENT_SCHEMA, "{id}");
        assert!(!document.tiles.is_empty(), "{id}");
        assert!(!document.rules.is_empty(), "{id}");
        assert!(document.width > 0 && document.height > 0, "{id}");
        let pack = <Grid2dSnapshot as store::ArtifactPack>::encode_pack(&document);
        assert!(pack.len() > 64, "{id}");
    }
}
