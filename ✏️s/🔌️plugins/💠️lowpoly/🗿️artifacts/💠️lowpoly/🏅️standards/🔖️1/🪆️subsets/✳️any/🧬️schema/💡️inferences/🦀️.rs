//! 💡️ Lowpoly inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::LowpolySnapshot;
use framework_schema::ArtifactSchema;

use super::bounds::scene_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a lowpoly snapshot. Today: object count and the 3d bounding box
/// across every object's `transform.position` (see `📦bounds/🦀️.rs`). A simple
/// whole-snapshot scalar — no `InferredField` caching, the object list is small.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly.inference")]
pub struct LowpolyInference {
    #[derived]
    pub object_count: usize,
    #[derived]
    pub bounds: Option<LowpolyBounds>,
}

impl protocol::Inference<LowpolySnapshot> for LowpolyInference {
    fn infer(snapshot: &LowpolySnapshot) -> Self {
        Self { object_count: snapshot.objects.len(), bounds: scene_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<LowpolySnapshot> for LowpolyInference {
    fn inference_schema_id() -> &'static str {
        "s.lowpoly.lowpoly.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.lowpoly.lowpoly.inference.objectCount", reads: &["objects"] }, protocol::InferenceFieldSpec { id: "s.lowpoly.lowpoly.inference.bounds", reads: &["objects"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::standards::v1::subsets::any::schema::LowpolyBuilder {
    type Snapshot = LowpolySnapshot;
    type Inference = LowpolyInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.lowpoly.lowpoly.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `lowpoly_artifact_schema_descriptor`'s registration.
pub fn lowpoly_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.lowpoly.lowpoly.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::LowpolyBounds;
//#endregion 🔁️Re-exports
