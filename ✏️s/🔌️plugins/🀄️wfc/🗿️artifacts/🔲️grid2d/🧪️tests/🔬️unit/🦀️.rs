//! 🧪️ Artifact-root laws — identity, the capability roster, and the native codec round trip.

use crate::schema::snapshot::Grid2dSnapshot;
use crate::{artifact_kind, definition, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};

#[test]
fn the_document_schema_is_the_dialect_artifact_kind() {
    assert_eq!(WFC_GRID2D_DOCUMENT_SCHEMA, "s.wfc.grid2d");
    assert_eq!(WFC_GRID2D_DIALECT.artifact_kind, WFC_GRID2D_DOCUMENT_SCHEMA);
    assert_eq!(WFC_GRID2D_DIALECT.standard.0, "1");
    assert_eq!(WFC_GRID2D_DIALECT.subset, semio_framework_plugin::SubsetId::ANY);
}

#[test]
fn the_os_artifact_kind_is_a_separate_dimension_scoped_id() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "2d.wfcgrid2d");
    assert_eq!(kind.dimension, "2d");
    assert_eq!(kind.component_kind, "wfcgrid2d");
    assert_eq!(kind.schema, WFC_GRID2D_DOCUMENT_SCHEMA);
    assert_ne!(kind.id, WFC_GRID2D_DOCUMENT_SCHEMA, "the OS kind id and the dialect id are different namespaces");
}

#[test]
fn the_definition_builds_with_every_declared_capability() {
    definition().expect("the capability roster is well formed");
}

#[test]
fn the_document_round_trips_through_its_own_dsl_and_pack_codecs() {
    let document = Grid2dSnapshot { seed: 5, width: 2, height: 3, cell_width: 1.5, cell_height: 2.5, periodic_x: true, ..Default::default() };
    let text = <Grid2dSnapshot as store::ArtifactDsl>::print_dsl(&document);
    assert_eq!(<Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("dsl parses"), document);
    let bytes = <Grid2dSnapshot as store::ArtifactPack>::encode_pack(&document);
    assert_eq!(<Grid2dSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("pack decodes"), document);
}

#[test]
fn the_document_round_trips_through_json() {
    let document = Grid2dSnapshot { seed: 11, width: 4, height: 4, ..Default::default() };
    let json = dsl::json::to_json_string(&document);
    assert_eq!(dsl::json::from_json_str::<Grid2dSnapshot>(&json).expect("json decodes"), document);
}
