//! 💡️ Lowpoly inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::lowpoly::LowpolySnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::bounds::{scene_bounds, LowpolyBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a lowpoly snapshot. Today: object count and the 3d bounding box
/// across every object's `transform.position` (see `📦bounds/🦀️component.rs`). A simple
/// whole-snapshot scalar — no `InferredField` caching, the object list is small.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly.inference")]
pub struct LowpolyInference {
    #[derived]
    pub object_count: usize,
    #[derived]
    pub bounds: Option<LowpolyBounds>,
}

impl protocol::Inference<LowpolySnapshot> for LowpolyInference {
    async fn infer(snapshot: &LowpolySnapshot) -> Self {
        Self { object_count: snapshot.objects.len(), bounds: scene_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<LowpolySnapshot> for LowpolyInference {
    async fn inference_schema_id() -> &'static str {
        "s.lowpoly.lowpoly.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.lowpoly.lowpoly.inference.objectCount", reads: &["objects"] },
            protocol::InferenceFieldSpec { id: "s.lowpoly.lowpoly.inference.bounds", reads: &["objects"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::lowpoly::standards::v1::subsets::any::schema::LowpolyBuilder {
    type Snapshot = LowpolySnapshot;
    type Inference = LowpolyInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.lowpoly.lowpoly.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `lowpoly_artifact_schema_descriptor`'s registration.
pub async fn lowpoly_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.lowpoly.lowpoly.inference",
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
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[test]
    async fn inference_determinism_law() {
        let snapshot = crate::artifacts::lowpoly::snapshot_from_mesh_json("{}", "o1", "Object 1");
        assert_eq!(LowpolyInference::infer(&snapshot), LowpolyInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(LowpolyInference::infer(&LowpolySnapshot::default()), LowpolyInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
