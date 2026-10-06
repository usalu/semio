//! 🧬️ Lowpoly snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
//! `dsl::DslRecord` spec, which carries each object's composed `mesh` child handle.

use crate::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;




//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "lowpoly")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
}

impl Default for LowpolySnapshot {
    fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️DocumentHelpers
/// 🏗️ Builds an object with its authored mesh source and content-addressed child handle.
pub fn snapshot_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolySnapshot {
    LowpolySnapshot {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject { mesh_state:None,
            id: object_id.into(),
            name: object_name.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh: Some(crate::mesh_child_handle(object_id, mesh_json)),
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
            mesh_content: mesh_json.into(),
        }],
    }
}
//#endregion 🔖️DocumentHelpers
