//! 💡️ Cad inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::cad::CadSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::bounds::{object_count, scene_bounds, vertex_count, CadBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a cad snapshot. Today: object/brep-vertex counts and the 3d
/// bounding box across every pane's object origins and vertex positions (see
/// `📦bounds/🦀️component.rs`). A simple whole-snapshot scalar — no `InferredField` caching, a full
/// scan over the document is cheap at cad scale.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad.inference")]
pub struct CadInference {
    #[state(inferred)]
    pub object_count: usize,
    #[state(inferred)]
    pub vertex_count: usize,
    #[state(inferred)]
    pub bounds: Option<CadBounds>,
}

impl protocol::Inference<CadSnapshot> for CadInference {
    fn infer(snapshot: &CadSnapshot) -> Self {
        Self { object_count: object_count(snapshot), vertex_count: vertex_count(snapshot), bounds: scene_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<CadSnapshot> for CadInference {
    fn inference_schema_id() -> &'static str {
        "s.cad.cad.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.objectCount", reads: &["objects", "buildingObjects", "energyObjects", "structureClassicObjects"] },
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.vertexCount", reads: &["shapeGeometry", "buildingGeometry", "energyGeometry", "structureClassicGeometry"] },
            protocol::InferenceFieldSpec {
                id: "s.cad.cad.inference.bounds",
                reads: &["objects", "buildingObjects", "energyObjects", "structureClassicObjects", "shapeGeometry", "buildingGeometry", "energyGeometry", "structureClassicGeometry"],
            },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::cad::standards::v1::subsets::any::schema::CadBuilder {
    type Snapshot = CadSnapshot;
    type Inference = CadInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.cad.cad.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `cad_artifact_schema_descriptor`'s registration.
pub fn cad_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.cad.cad.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::cad::{empty_cad_snapshot, CadObject};
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let mut snapshot = empty_cad_snapshot();
        snapshot.objects.push(CadObject {
            id: "o1".into(),
            label: "O1".into(),
            typology: "generic".into(),
            visible: true,
            locked: false,
            origin: [1.0, 2.0, 3.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: None,
            primitives: Vec::new(),
        });
        assert_eq!(CadInference::infer(&snapshot), CadInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(CadInference::infer(&empty_cad_snapshot()), CadInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
