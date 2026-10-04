//! 💡️ Equation inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (`🧭topology/`, and `🌱roots/` — wave M3a,
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS).

use crate::EquationSnapshot;
use framework_schema::ArtifactSchema;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use super::roots::{compute_equation_roots, EquationRoot};
use super::topology::compute_equation_topology;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
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
        let snapshot = &EquationSnapshot::default();

        Self { topology: compute_equation_topology(&snapshot.graph), roots: compute_equation_roots(snapshot) }
    }
}

impl protocol::Inference<EquationSnapshot> for EquationInference {
    fn infer(snapshot: &EquationSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_equation_topology(&snapshot.graph), roots: compute_equation_roots(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<EquationSnapshot> for EquationInference {
    fn inference_schema_id() -> &'static str {
        "s.mathematical.equation.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.mathematical.equation.inference.topology", reads: &["notation", "results", "computed"] }, protocol::InferenceFieldSpec { id: "s.mathematical.equation.inference.roots", reads: &["equation"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.mathematical.equation.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `equation_artifact_schema_descriptor`'s
/// registration.
pub fn equation_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.mathematical.equation.inference",
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
pub use super::topology::EquationTopology;
//#endregion 🔁️Re-exports
