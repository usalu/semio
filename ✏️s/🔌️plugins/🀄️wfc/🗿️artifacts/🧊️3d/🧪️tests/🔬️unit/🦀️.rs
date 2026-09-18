//! 🧪️ `wfc3d` artifact root — identity, the language roster, and the inline media helpers every
//! example builds its tile catalogue from.

use crate::schema::snapshot::{TileMedia3d, Wfc3dSnapshot};
use crate::{artifact_kind, definition, wfc3d_languages, WFC3D_DIALECT, WFC3D_DOCUMENT_SCHEMA};

#[test]
fn the_dialect_coordinate_matches_the_document_schema_and_this_files_own_location() {
    assert_eq!(WFC3D_DOCUMENT_SCHEMA, "s.wfc.wfc3d");
    assert_eq!(WFC3D_DIALECT.artifact_kind, WFC3D_DOCUMENT_SCHEMA);
    assert_eq!(WFC3D_DIALECT.standard.0, "1");
    assert_eq!(WFC3D_DIALECT.subset.0, "*");
}

/// 🗂️ The OS kind id is a DIFFERENT namespace from the dialect — `<dimension>.<name>`, never the
/// `s.<plugin>.<artifact>` string.
#[test]
fn the_os_artifact_kind_is_the_dimension_namespace() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "3d.wfc3d");
    assert_eq!(kind.component_kind, "wfc3d");
    assert_eq!(kind.dimension, "3d");
    assert_eq!(kind.schema, WFC3D_DOCUMENT_SCHEMA);
    assert_eq!(kind.name, "3D");
}

#[test]
fn the_definition_declares_every_capability_row_without_error() {
    let definition = definition().expect("wfc3d definition builds");
    assert_eq!(definition.identity().as_str(), "s.wfc.wfc3d");
}

/// 📌️ `io()` indexes this roster positionally (0 document, 1 op, 2 diff, 3 pack, 4 spr), so the
/// order is a contract, not a convenience.
#[test]
fn the_language_roster_is_document_op_diff_pack_spr_in_that_order() {
    let languages = wfc3d_languages();
    let ids: Vec<&str> = languages.iter().map(|language| language.id).collect();
    assert_eq!(ids, vec!["wfc3d.snapshot", "wfc3d.mutations", "wfc3d.diff", "wfc3d.pack", "wfc3d.spr"]);
    assert_eq!(languages[0].extension, Some("wfc3d"));
    assert!(languages[0].grammar.is_some() && languages[0].protocol.is_some());
    assert!(languages[3].grammar.is_none() && languages[3].protocol.is_some(), "a pack language carries a protocol and never a grammar");
}

/// 📦️ The inline unit box is what a slot with no resolvable tile media still gets to draw, so its
/// triangle list has to be real geometry, not an empty placeholder.
#[test]
fn the_unit_box_helper_is_a_closed_twelve_triangle_body() {
    match crate::unit_box_media(None) {
        TileMedia3d::Mesh { positions, indices, color } => {
            assert_eq!(positions.len(), 24, "eight corners, three floats each");
            assert_eq!(indices.len(), 36, "six quads, two triangles each");
            assert!(indices.iter().all(|index| (*index as usize) < positions.len() / 3), "every index addresses a real corner");
            assert_eq!(color, None);
        }
        TileMedia3d::MeshChild { .. } => panic!("the inline helper must not mint a child handle"),
    }
}

#[test]
fn the_unit_wedge_helper_is_a_closed_body_with_fewer_corners_than_the_box() {
    match crate::unit_wedge_media(None) {
        TileMedia3d::Mesh { positions, indices, .. } => {
            assert_eq!(positions.len(), 18, "six corners, three floats each");
            assert!(!indices.is_empty());
            assert!(indices.iter().all(|index| (*index as usize) < positions.len() / 3));
        }
        TileMedia3d::MeshChild { .. } => panic!("the inline helper must not mint a child handle"),
    }
}

/// 🥽️ A mesh CHILD handle is content-addressed by the tile id on both sides, which is what the
/// composed-child load contract requires (`child_id == target.artifact_id`, unique per document).
#[test]
fn a_mesh_child_handle_names_the_tile_on_both_sides() {
    let handle = crate::mesh_child_handle("stair");
    assert_eq!(handle.child_id, "stair");
    assert_eq!(handle.target.artifact_id, "stair");
    assert_eq!(handle.target.dialect.artifact_kind, "s.stdio.semio");
    assert_eq!(handle.target.dialect.subset, "mesh");
}

#[test]
fn the_empty_document_carries_its_own_schema_string() {
    assert_eq!(Wfc3dSnapshot::default().schema, WFC3D_DOCUMENT_SCHEMA);
}
