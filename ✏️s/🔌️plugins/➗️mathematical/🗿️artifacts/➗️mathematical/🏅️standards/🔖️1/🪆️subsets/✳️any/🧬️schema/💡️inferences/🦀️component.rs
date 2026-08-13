//! 💡️ Mathematical inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (`🧭topology/`, and `🌱roots/` — wave M3a,
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS).

use crate::artifacts::mathematical::MathematicalSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::roots::{compute_mathematical_roots, MathematicalRoot};
use super::topology::{compute_mathematical_topology, MathematicalTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a mathematical snapshot. One field per named inference under
/// `💡️inferences/` (`topology`, backed by `🧭topology/`; `roots`, backed by `🌱roots/`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical.inference")]
pub struct MathematicalInference {
    #[state(inferred)]
    pub topology: MathematicalTopology,
    #[state(inferred)]
    pub roots: Vec<MathematicalRoot>,
}

impl Default for MathematicalInference {
    fn default() -> Self {
        <Self as protocol::Inference<MathematicalSnapshot>>::infer(&MathematicalSnapshot::default())
    }
}

impl protocol::Inference<MathematicalSnapshot> for MathematicalInference {
    fn infer(snapshot: &MathematicalSnapshot) -> Self {
        Self { topology: compute_mathematical_topology(&crate::artifacts::mathematical::mathematical_graph(snapshot)), roots: compute_mathematical_roots(snapshot) }
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
        &[
            protocol::InferenceFieldSpec { id: "s.mathematical.mathematical.inference.topology", reads: &["notation", "results", "computed"] },
            protocol::InferenceFieldSpec { id: "s.mathematical.mathematical.inference.roots", reads: &["equation"] },
        ]
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

    #[test]
    fn default_equation_is_the_zero_polynomial_with_no_roots() {
        // 🔎️ `EquationSnapshot::default()` is the integer literal `0` — the zero polynomial has no
        // isolated real roots, an empty `Vec`, never a panic (see `🌱roots`'s own scope tests).
        let inferred = MathematicalInference::infer(&MathematicalSnapshot::default());
        assert!(inferred.roots.is_empty());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
