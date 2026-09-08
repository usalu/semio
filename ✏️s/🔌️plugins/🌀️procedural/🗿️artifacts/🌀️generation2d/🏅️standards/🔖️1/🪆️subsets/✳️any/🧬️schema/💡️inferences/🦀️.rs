//! 💡️ Generation2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::Generation2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use semio_framework_value_derive::{FromValue, ToValue};
use super::topology::{compute_generation2d_topology, Generation2dTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a generation2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation2d.inference")]
pub struct Generation2dInference {
    #[derived]
    pub topology: Generation2dTopology,
}

impl protocol::Inference<Generation2dSnapshot> for Generation2dInference {
    fn infer(snapshot: &Generation2dSnapshot) -> Self {
        Self { topology: compute_generation2d_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<Generation2dSnapshot> for Generation2dInference {
    fn inference_schema_id() -> &'static str {
        "s.procedural.generation2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.procedural.generation2d.inference.topology", reads: &["fixture"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ `topology` is a whole-snapshot scalar (see `🧭topology/🦀️.rs`), so the default
/// `ArtifactInferrer::infer_cached` passthrough (plain `infer`, no `InferenceCache`/`InferenceSession`
/// involvement) is exactly right — nothing here benefits from per-entity incremental caching.
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Generation2dBuilder {
    type Snapshot = Generation2dSnapshot;
    type Inference = Generation2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.procedural.generation2d.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `generation2d_artifact_schema_descriptor`'s
/// registration.
pub fn generation2d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.procedural.generation2d.inference",
        inference: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
