//! 💡️ Forms inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::{forms_steps, FormsSnapshot};
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_forms_topology, FormsTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a forms snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.inference")]
pub struct FormsInference {
    #[derived]
    pub topology: FormsTopology,
}

impl Default for FormsInference {
    fn default() -> Self {
        <Self as protocol::Inference<FormsSnapshot>>::infer(&FormsSnapshot::default())
    }
}

impl protocol::Inference<FormsSnapshot> for FormsInference {
    fn infer(snapshot: &FormsSnapshot) -> Self {
        Self { topology: compute_forms_topology(&forms_steps(snapshot)) }
    }
}

impl protocol::InferenceSpec<FormsSnapshot> for FormsInference {
    fn inference_schema_id() -> &'static str {
        "s.forms.forms.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.forms.forms.inference.topology", reads: &["steps"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🎯️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM: `ArtifactInferrer::infer` takes
/// `&Self::Snapshot` (never `&self`), so the impl target is a pure type-level anchor — a local
/// zero-sized marker, not the deleted `derive_artifact_facets!`-generated `FormsBuilder`
/// (retargeting onto `semio_framework_plugin::app::SnapshotBuilder<S, M>` is an orphan-rule
/// violation: it is a foreign, non-`#[fundamental]` generic struct — confirmed by the
/// `🎬️sequence` fan-out pass, `📓️w4-sequence-report.md` `## recipeGaps` #1).
pub struct FormsInferrer;
impl ArtifactInferrer for FormsInferrer {
    type Snapshot = FormsSnapshot;
    type Inference = FormsInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.forms.forms.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `forms_artifact_schema_descriptor`'s registration.
pub fn forms_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.forms.forms.inference",
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
