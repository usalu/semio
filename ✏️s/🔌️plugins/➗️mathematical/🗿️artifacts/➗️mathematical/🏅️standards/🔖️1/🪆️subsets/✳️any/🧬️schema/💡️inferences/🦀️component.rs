//! 💡️ Mathematical inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::mathematical::MathematicalSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_mathematical_topology, MathematicalTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a mathematical snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical.inference")]
pub struct MathematicalInference {
    #[state(inferred)]
    pub topology: MathematicalTopology,
}

impl Default for MathematicalInference {
    fn default() -> Self {
        <Self as protocol::Inference<MathematicalSnapshot>>::infer(&MathematicalSnapshot::default())
    }
}

impl protocol::Inference<MathematicalSnapshot> for MathematicalInference {
    fn infer(snapshot: &MathematicalSnapshot) -> Self {
        Self { topology: compute_mathematical_topology(&snapshot.graph) }
    }
}

impl protocol::InferenceSpec<MathematicalSnapshot> for MathematicalInference {
    fn inference_schema_id() -> &'static str {
        "s.mathematical.mathematical.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.mathematical.mathematical.inference.topology", reads: &["graph"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::mathematical::standards::v1::subsets::any::schema::MathematicalBuilder {
    type Snapshot = MathematicalSnapshot;
    type Inference = MathematicalInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.mathematical.mathematical.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `mathematical_artifact_schema_descriptor`'s
/// registration.
pub fn mathematical_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.mathematical.mathematical.inference",
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
    fn inference_determinism_law() {
        let snapshot = MathematicalSnapshot::default();
        assert_eq!(MathematicalInference::infer(&snapshot), MathematicalInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(MathematicalInference::infer(&MathematicalSnapshot::default()), MathematicalInference::default());
    }

    #[test]
    fn default_graph_diamond_is_cycle_free_with_two_roots() {
        // 🔷 The default graph is a diamond: a->b, a->c, b->d, c->d — acyclic, `a` is the only root.
        let inferred = MathematicalInference::infer(&MathematicalSnapshot::default());
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.node_count, 4);
        assert_eq!(inferred.topology.depth["a"], 0);
        assert_eq!(inferred.topology.depth["d"], 2);
        let a_index = inferred.topology.topo_order.iter().position(|id| id == "a").unwrap();
        let d_index = inferred.topology.topo_order.iter().position(|id| id == "d").unwrap();
        assert!(a_index < d_index);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
