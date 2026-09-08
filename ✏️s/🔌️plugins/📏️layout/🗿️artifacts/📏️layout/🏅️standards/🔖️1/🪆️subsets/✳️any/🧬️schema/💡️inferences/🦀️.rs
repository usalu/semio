//! 💡️ Layout inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::LayoutSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_layout_topology};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a layout snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout.inference")]
pub struct LayoutInference {
    #[derived]
    pub topology: LayoutTopology,
}

// 🧷️ `LayoutSnapshot` has no `Default` impl of its own (its only "empty-ish" constructor,
// `engine::default_document`, actually seeds a full demo document) — so unlike the sibling
// forms/playbook/mathematical inference facets, this can't be written as
// `infer(&LayoutSnapshot::default())`. An empty layout document (no pages/spreads/parent pages) has
// an unambiguous empty topology, so that's what this hand-written `Default` returns directly.
impl Default for LayoutInference {
    fn default() -> Self {
        Self { topology: LayoutTopology::empty() }
    }
}

impl protocol::Inference<LayoutSnapshot> for LayoutInference {
    fn infer(snapshot: &LayoutSnapshot) -> Self {
        Self { topology: compute_layout_topology(&snapshot.parent_pages, &snapshot.spreads, &snapshot.pages) }
    }
}

impl protocol::InferenceSpec<LayoutSnapshot> for LayoutInference {
    fn inference_schema_id() -> &'static str {
        "s.layout.layout.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.layout.layout.inference.topology", reads: &["parentPages", "spreads", "pages"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::LayoutBuilder {
    type Snapshot = LayoutSnapshot;
    type Inference = LayoutInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.layout.layout.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `layout_artifact_schema_descriptor`'s registration.
pub fn layout_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.layout.layout.inference",
        inference: schema::FacetLeaves {
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::topology::LayoutTopology;
//#endregion 🔁️Re-exports
