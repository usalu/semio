//! 💡️ Playground inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🧭️Topology
/// 🧭️ Playground topology is honestly empty because its snapshot owns only schema metadata.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaygroundTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

async fn infer_topology(_snapshot: &PlaygroundSnapshot) -> PlaygroundTopology {
    PlaygroundTopology { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
}
//#endregion 🧭️Topology

//#region 🔖️Inference
/// 💡️ Everything inferable from a playground snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — `PlaygroundSnapshot`
/// is today's minimal schema stub (`schema: String` only, see its own doc comment), so `topology`
/// here is honestly always the vacuous empty graph until this artifact grows real domain entities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.demonstrator.playground.inference")]
pub struct PlaygroundInference {
    #[derived]
    pub topology: PlaygroundTopology,
}

impl protocol::Inference<PlaygroundSnapshot> for PlaygroundInference {
    async fn infer(snapshot: &PlaygroundSnapshot) -> Self {
        Self { topology: infer_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) so this stays correct regardless of what
/// `PlaygroundSnapshot::default()` happens to contain.
impl Default for PlaygroundInference {
    async fn default() -> Self {
        <Self as protocol::Inference<PlaygroundSnapshot>>::infer(&PlaygroundSnapshot::default())
    }
}

impl protocol::InferenceSpec<PlaygroundSnapshot> for PlaygroundInference {
    async fn inference_schema_id() -> &'static str {
        "s.demonstrator.playground.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.demonstrator.playground.inference.topology", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::playground::standards::v1::subsets::any::schema::PlaygroundBuilderFacets {
    type Snapshot = PlaygroundSnapshot;
    type Inference = PlaygroundInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.demonstrator.playground.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `playground_artifact_schema_descriptor`'s registration.
pub async fn playground_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.demonstrator.playground.inference",
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

    #[test]
    async fn inference_determinism_law() {
        let snapshot = PlaygroundSnapshot::default();
        assert_eq!(PlaygroundInference::infer(&snapshot), PlaygroundInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(PlaygroundInference::infer(&PlaygroundSnapshot::default()), PlaygroundInference::default());
    }

    #[test]
    async fn topology_is_the_vacuous_empty_graph() {
        let topology = infer_topology(&PlaygroundSnapshot::default());
        assert!(topology.topo_order.is_empty());
        assert!(topology.depth.is_empty());
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 0);
    }
}
//#endregion 🧪️Tests
