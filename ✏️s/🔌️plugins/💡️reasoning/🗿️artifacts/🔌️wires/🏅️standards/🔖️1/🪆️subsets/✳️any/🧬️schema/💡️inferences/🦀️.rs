//! 💡️ Wires inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::WiresSnapshot;
use dsl::DslValue;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_wires_topology, WiresTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a wires snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir, read off the
/// `board_fixture`'s `nodes`/`edges` — the actual graph a wires board renders).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires.inference")]
pub struct WiresInference {
    #[derived]
    pub topology: WiresTopology,
}

impl protocol::Inference<WiresSnapshot> for WiresInference {
    fn infer(snapshot: &WiresSnapshot) -> Self {
        Self { topology: compute_wires_topology(&crate::wires_working_board(snapshot)) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&empty_wires_snapshot())` rather than a naive
/// `#[derive(Default)]` — `WiresSnapshot` itself has no `Default` impl (its two fields are opaque
/// `DslValue` blobs), so this mirrors the "match `infer` of the canonical empty document, don't
/// derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️.rs`, anchored on
/// this artifact's own `empty_wires_snapshot()` rather than `Default::default()`.
impl Default for WiresInference {
    fn default() -> Self {
        <Self as protocol::Inference<WiresSnapshot>>::infer(&crate::empty_wires_snapshot())
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

//#region 🔖️LookupHelpers
/// 🔎️ Reads of `&WiresSnapshot` — dissolved from the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): each fn here takes the whole document
/// snapshot (not just a `DslValue`), so it's derived compute over the artifact rather than a generic
/// document helper — those instead live in `🧬️schema/🦀️component.rs`'s `🔖️DocumentHelpers` region.
///
/// `find_board_node`/`find_board_edge` return OWNED `DslValue` (not `&'a DslValue` tied to
/// `document`'s lifetime) since UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: the real node/edge data no longer
/// lives inside `WiresSnapshot` itself, it's read through [`crate::wires_working_board`]
/// (the working-scene accessor), which materializes a fresh `DslValue` every call.
pub fn find_board_node(document: &WiresSnapshot, node_id: &str) -> Option<DslValue> {
    crate::wires_working_board(document)
        .get("nodes")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|node| crate::standards::v1::subsets::any::schema::entity_id(node, "id") == Some(node_id))
        .cloned()
}

pub fn find_board_edge(document: &WiresSnapshot, edge_id: &str) -> Option<DslValue> {
    crate::wires_working_board(document)
        .get("edges")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|edge| crate::standards::v1::subsets::any::schema::entity_id(edge, "id") == Some(edge_id))
        .cloned()
}

pub fn find_relationship<'a>(document: &'a WiresSnapshot, edge_id: &str) -> Option<&'a DslValue> {
    document.wires_fixture.get("relationships").and_then(|value| value.as_array()).into_iter().flatten().find(|relationship| crate::standards::v1::subsets::any::schema::entity_id(relationship, "edgeId") == Some(edge_id))
}
//#endregion 🔖️LookupHelpers

//#region 🔖️ArtifactInferrer
/// 🪪️ Zero-sized marker struct anchoring the `ArtifactInferrer` impl — `ArtifactInferrer::infer`
/// takes `&Self::Snapshot`, never `&self`, so the impl target is a pure type-level anchor, not a
/// real value. Ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`'s recipe (§2) suggests
/// retargeting a deleted `derive_artifact_facets!`-generated builder type onto the generic
/// `semio_framework_plugin::app::SnapshotBuilder<S, M>` when the macro is removed — that is a genuine
/// orphan-rule violation (E0117: `SnapshotBuilder` is a foreign, non-`#[fundamental]` generic struct),
/// confirmed by `📓️w4-sequence-report.md` `## recipeGaps` #1. This marker struct is that report's
/// documented fix.
pub struct WiresInferrer;

impl ArtifactInferrer for WiresInferrer {
    type Snapshot = WiresSnapshot;
    type Inference = WiresInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.reasoning.wires.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `wires_artifact_schema_descriptor`'s registration.
pub fn wires_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.reasoning.wires.inference",
        inference: framework_schema::FacetLeaves {
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
