//! ⛰️ GIS terrain artifact — the document entity the 🧊️3d app edits (constitutional: general).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🗄️ VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).
pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔖️Constants

//#region 🔖️Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gisterrain", keyword = "gisterrain")]
pub struct Gis3dTerrainDocument {
    pub exaggeration: f64,
    /// 🔌️ `map:in`'s insertion point (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
    /// Wave 2 port recipe): the last-imported `2d.map` descriptor JSON (`{positions,routes,regions}`,
    /// same shape `crate::artifacts::gismap::engine::gis_map_descriptor_json` produces), rendered as an
    /// extra pin layer alongside the read-only fixture-text positions (see the 🏔️terrain window's
    /// `instances_json`) — real, undoable document state, not view-only scratch, since importing a map
    /// overlay is a document edit.
    pub imported_features_json: String,
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🔌️ `3d.mesh` — the interchange kind `scene:out` produces; canonically declared by `lowpoly`
/// (`mesh_from_mesh_document`'s registration). Re-declared here as an identical-shape duplicate so the
/// 🧊️3d manifest is self-describing on both sides of the edge (the registry dedupes by id).
pub fn mesh_artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.mesh".into(),
        name: "3D Mesh".into(),
        source_format: "mesh.reference".into(),
        component_kind: "mesh".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "mesh.reference".into(),
        export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl],
        import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_artifact_kind_matches_the_scene_out_interchange_kind() {
        let kind = mesh_artifact_kind();
        assert_eq!(kind.id, "3d.mesh");
        assert_eq!(kind.schema, "mesh.reference");
    }

    #[test]
    fn the_terrain_document_defaults_to_a_flat_unimported_terrain() {
        let document = Gis3dTerrainDocument::default();
        assert_eq!(document.exaggeration, 0.0);
        assert!(document.imported_features_json.is_empty());
    }
}
//#endregion 🧪️Tests
