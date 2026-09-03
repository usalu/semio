//! 💡️ Equation inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (`🧭topology/`, and `🌱roots/` — wave M3a,
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS).

use crate::artifacts::equation::EquationSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

use super::roots::{compute_equation_roots, EquationRoot};
use super::topology::{compute_equation_topology, EquationTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a equation snapshot. One field per named inference under
/// `💡️inferences/` (`topology`, backed by `🧭topology/`; `roots`, backed by `🌱roots/`).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.equation.inference")]
pub struct EquationInference {
    #[derived]
    pub topology: EquationTopology,
    #[derived]
    pub roots: Vec<EquationRoot>,
}

impl Default for EquationInference {
    fn default() -> Self {
        <Self as protocol::Inference<EquationSnapshot>>::infer(&EquationSnapshot::default())
    }
}

impl protocol::Inference<EquationSnapshot> for EquationInference {
    async fn infer(snapshot: &EquationSnapshot) -> Self {
        Self { topology: compute_equation_topology(&crate::artifacts::equation::equation_graph(snapshot)), roots: compute_equation_roots(snapshot) }
    }
}

impl protocol::InferenceSpec<EquationSnapshot> for EquationInference {
    async fn inference_schema_id() -> &'static str {
        "s.mathematical.equation.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.mathematical.equation.inference.topology", reads: &["notation", "results", "computed"] }, protocol::InferenceFieldSpec { id: "s.mathematical.equation.inference.roots", reads: &["equation"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🌱 Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM: the old impl target
/// (`derive_artifact_facets!`-generated `EquationBuilder`, deleted in the same pass) is gone.
/// `semio_framework_plugin::app::SnapshotBuilder<S, M>` (the recipe's literally-suggested
/// replacement) does NOT work here — it is a foreign, non-`#[fundamental]` generic struct, so
/// `impl ArtifactInferrer for SnapshotBuilder<EquationSnapshot, EquationMutation>` is an
/// orphan-rule violation (E0117) regardless of the type parameters being local (confirmed by
/// `🎬️sequence`'s identical W4 pass, `📓️w4-sequence-report.md` `## recipeGaps` #1).
/// `ArtifactInferrer::infer` takes `&Self::Snapshot`, never `&self`, so the impl target is a pure
/// type-level anchor with zero live callers repo-wide (grepped) — a trivial local zero-sized
/// marker struct is the real fix.
pub struct EquationInferrer;

impl ArtifactInferrer for EquationInferrer {
    type Snapshot = EquationSnapshot;
    type Inference = EquationInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.mathematical.equation.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `equation_artifact_schema_descriptor`'s
/// registration.
pub async fn equation_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.mathematical.equation.inference",
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
mod tests {
    use super::*;
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = EquationSnapshot::default();
        assert_eq!(EquationInference::infer(&snapshot), EquationInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(EquationInference::infer(&EquationSnapshot::default()), EquationInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn default_graph_diamond_is_cycle_free_with_two_roots() {
        // 🔷 The default graph is a diamond: a->b, a->c, b->d, c->d — acyclic, `a` is the only root.
        let inferred = EquationInference::infer(&EquationSnapshot::default());
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.node_count, 4);
        assert_eq!(inferred.topology.depth["a"], 0);
        assert_eq!(inferred.topology.depth["d"], 2);
        let a_index = inferred.topology.topo_order.iter().position(|id| id == "a").unwrap();
        let d_index = inferred.topology.topo_order.iter().position(|id| id == "d").unwrap();
        assert!(a_index < d_index);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_equation_is_the_zero_polynomial_with_no_roots() {
        // 🔎️ `EquationExprSnapshot::default()` is the integer literal `0` — the zero polynomial has no
        // isolated real roots, an empty `Vec`, never a panic (see `🌱roots`'s own scope tests).
        let inferred = EquationInference::infer(&EquationSnapshot::default());
        assert!(inferred.roots.is_empty());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
