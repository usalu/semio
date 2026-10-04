//! 💡️ Wires inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::WiresSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from a wires snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir, read off the
/// `board_fixture`'s `nodes`/`edges` — the actual graph a wires board renders).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires.inference")]
pub struct WiresInference {
    #[derived]
    pub topology: WiresTopology,
}

/// 🧭️ The parent alone carries no board (§20.15: it lives in the composed `content` child), so a parent-only inference reads
/// the empty topology; the composed topology is [`super::topology::compute_wires_topology`] over `crate::wires_composed` (the
/// `ArchiveChildren` reader seam hands parent-only readers the child).
impl protocol::Inference<WiresSnapshot> for WiresInference {
    fn infer(_snapshot: &WiresSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self::default())
    }
}

/// 🌱 The topology of the empty board.
impl Default for WiresInference {
    fn default() -> Self {
        Self { topology: WiresTopology::default() }
    }
}

impl protocol::InferenceSpec<WiresSnapshot> for WiresInference {
    fn inference_schema_id() -> &'static str {
        "s.reasoning.wires.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.reasoning.wires.inference.topology", reads: &["content"] }]
    }
}
//#endregion 🔖️Inference


//#region 🔖️Descriptor
/// 💡️ Registers `s.reasoning.wires.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `wires_artifact_schema_descriptor`'s registration.
pub fn wires_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.reasoning.wires.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
pub use super::topology::WiresTopology;
//#endregion 🔁️Re-exports
