//! 💡️ Generation2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use super::topology::compute_generation2d_topology;
use crate::Generation2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a generation2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
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

/// 🌱 Hand-fixed to agree with `infer(&Generation2dSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — the snapshot's `fixture` is `semio_framework_artifact_flow_flow::FlowFixture::default()`,
/// which ships a non-empty three-widget starter graph, so a structural default would name a
/// topology no document ever has. Same trick as `FlowInference`'s hand-written `Default`
/// (`🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/💡️inferences/🦀️.rs`).
impl Default for Generation2dInference {
    fn default() -> Self {
        <Self as protocol::Inference<Generation2dSnapshot>>::infer(&Generation2dSnapshot::default())
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
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
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
pub use super::topology::Generation2dTopology;
//#endregion 🔁️Re-exports
