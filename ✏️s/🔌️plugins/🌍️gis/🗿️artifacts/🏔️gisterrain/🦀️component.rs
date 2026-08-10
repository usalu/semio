// GIS terrain artifact — the document entity the 3d app edits (constitutional: general).

pub use crate::artifacts::gisterrain::schema::snapshot::GisTerrainSnapshot;
pub use crate::artifacts::gisterrain::schema::mutations::GisTerrainMutation;
pub use crate::artifacts::gisterrain::schema::diff::GisTerrainDiff;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };

//#region 🔹Constants
/// VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).


pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔹Constants

//#region 🔹Types
/// 📸️ Persisted GIS terrain snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
//#endregion 🔹Types

//#region 🔹ArtifactKind
/// 🔌️ `3d.mesh` — the interchange kind `scene:out` produces; canonically declared by `lowpoly`
/// (`mesh_from_mesh_document`'s registration). Re-declared here as an identical-shape duplicate so the
/// 3d manifest is self-describing on both sides of the edge (the registry dedupes by id).
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
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
    }
}
//#endregion 🔹ArtifactKind

//#region 🔹Tests
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
    fn the_terrain_snapshot_defaults_to_a_flat_unimported_terrain() {
        let document = GisTerrainSnapshot::default();
        assert_eq!(document.exaggeration, 0.0);
        assert!(document.imported_features_json.is_empty());
    }
}
//#endregion 🔹Tests
