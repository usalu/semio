//! 💡️ Presentation inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::PresentationSnapshot;
use schema::ArtifactSchema;

use super::topology::compute_presentation_topology;
//#region 🔖️Inference
/// 💡️ Everything inferable from a presentation snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — the tile
/// filmstrip has no dependency edges of its own, so "topology" here is the honest degenerate case:
/// a linear chain in persisted tile order (`topoOrder` == tile ids in order, `depth` == each tile's
/// index, always `cycleFree`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.presentation.inference")]
pub struct PresentationInference {
    #[derived]
    pub topology: PresentationTopology,
}

impl protocol::Inference<PresentationSnapshot> for PresentationInference {
    fn infer(snapshot: &PresentationSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_presentation_topology(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) so this stays correct even if `PresentationSnapshot`'s
/// own `Default` ever stops being the empty tile list — see the sibling artifacts' inference
/// families for the same trick where the default snapshot is NOT the zero value.
impl Default for PresentationInference {
    fn default() -> Self {
        let snapshot = &PresentationSnapshot::default();

        Self { topology: compute_presentation_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<PresentationSnapshot> for PresentationInference {
    fn inference_schema_id() -> &'static str {
        "s.animate.presentation.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.animate.presentation.inference.topology", reads: &["tiles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.animate.presentation.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `presentation_artifact_schema_descriptor`'s registration.
pub fn presentation_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.animate.presentation.inference",
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
pub use super::topology::PresentationTopology;
//#endregion 🔁️Re-exports
