//! 💡️ Wires inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_wires_topology, WiresTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a wires snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir, read off the
/// `board_fixture`'s `nodes`/`edges` — the actual graph a wires board renders).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires.inference")]
pub struct WiresInference {
    #[state(inferred)]
    pub topology: WiresTopology,
}

impl protocol::Inference<WiresSnapshot> for WiresInference {
    fn infer(snapshot: &WiresSnapshot) -> Self {
        Self { topology: compute_wires_topology(&snapshot.board_fixture) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&empty_wires_snapshot())` rather than a naive
/// `#[derive(Default)]` — `WiresSnapshot` itself has no `Default` impl (its two fields are opaque
/// `DslValue` blobs), so this mirrors the "match `infer` of the canonical empty document, don't
/// derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️component.rs`, anchored on
/// this artifact's own `empty_wires_snapshot()` rather than `Default::default()`.
impl Default for WiresInference {
    fn default() -> Self {
        <Self as protocol::Inference<WiresSnapshot>>::infer(&crate::artifacts::wires::empty_wires_snapshot())
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
        &[protocol::InferenceFieldSpec { id: "s.reasoning.wires.inference.topology", reads: &["board_fixture"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️LookupHelpers
/// 🔎️ Reads of `&WiresSnapshot` — dissolved from the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): each fn here takes the whole document
/// snapshot (not just a `DslValue`), so it's derived compute over the artifact rather than a generic
/// document helper — those instead live in `🧬️schema/🦀️component.rs`'s `🔖️DocumentHelpers` region.
pub fn find_board_node<'a>(document: &'a WiresSnapshot, node_id: &str) -> Option<&'a DslValue> {
    document.board_fixture.get("nodes").and_then(|value| value.as_array()).into_iter().flatten().find(|node| crate::artifacts::wires::standards::v1::subsets::any::schema::entity_id(node, "id") == Some(node_id))
}

pub fn find_board_edge<'a>(document: &'a WiresSnapshot, edge_id: &str) -> Option<&'a DslValue> {
    document.board_fixture.get("edges").and_then(|value| value.as_array()).into_iter().flatten().find(|edge| crate::artifacts::wires::standards::v1::subsets::any::schema::entity_id(edge, "id") == Some(edge_id))
}

pub fn find_relationship<'a>(document: &'a WiresSnapshot, edge_id: &str) -> Option<&'a DslValue> {
    document.wires_fixture.get("relationships").and_then(|value| value.as_array()).into_iter().flatten().find(|relationship| crate::artifacts::wires::standards::v1::subsets::any::schema::entity_id(relationship, "edgeId") == Some(edge_id))
}
//#endregion 🔖️LookupHelpers

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::wires::standards::v1::subsets::any::schema::WiresBuilder {
    type Snapshot = WiresSnapshot;
    type Inference = WiresInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.reasoning.wires.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `wires_artifact_schema_descriptor`'s registration.
pub fn wires_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.reasoning.wires.inference",
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
    use crate::artifacts::wires::empty_wires_snapshot;
    use dsl::DslValue;
    use protocol::Inference;

    fn chain_snapshot() -> WiresSnapshot {
        let mut snapshot = empty_wires_snapshot();
        snapshot.board_fixture = DslValue::object([
            ("schema".into(), DslValue::String("reasoning.mindmap.fixture".into())),
            ("camera".into(), DslValue::object([("x".into(), DslValue::Number(0.0)), ("y".into(), DslValue::Number(0.0)), ("zoom".into(), DslValue::Number(1.0))])),
            ("nodes".into(), DslValue::Array(vec![DslValue::object([("id".into(), DslValue::String("a".into()))]), DslValue::object([("id".into(), DslValue::String("b".into()))])])),
            ("edges".into(), DslValue::Array(vec![DslValue::object([("id".into(), DslValue::String("e1".into())), ("source".into(), DslValue::String("a".into())), ("target".into(), DslValue::String("b".into()))])])),
            ("wires".into(), DslValue::Array(vec![])),
        ]);
        snapshot
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(WiresInference::infer(&snapshot), WiresInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(WiresInference::infer(&empty_wires_snapshot()), WiresInference::default());
    }

    #[test]
    fn topology_counts_the_board_graph() {
        let inferred = WiresInference::infer(&chain_snapshot());
        assert_eq!(inferred.topology.node_count, 2);
        assert_eq!(inferred.topology.edge_count, 1);
        assert!(inferred.topology.cycle_free);
    }
}
//#endregion 🧪️Tests
