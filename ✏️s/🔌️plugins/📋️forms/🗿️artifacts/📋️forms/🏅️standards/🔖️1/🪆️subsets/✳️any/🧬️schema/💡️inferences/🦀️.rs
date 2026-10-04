//! 💡️ Forms inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::{forms_steps, FormsSnapshot};
use framework_schema::ArtifactSchema;

use super::topology::compute_forms_topology;
//#region 🔖️Inference
/// 💡️ Everything inferable from a forms snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.inference")]
pub struct FormsInference {
    #[derived]
    pub topology: FormsTopology,
}

impl Default for FormsInference {
    fn default() -> Self {
        let snapshot = &FormsSnapshot::default();

        Self { topology: compute_forms_topology(&forms_steps(snapshot)) }
    }
}

impl protocol::Inference<FormsSnapshot> for FormsInference {
    fn infer(snapshot: &FormsSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_forms_topology(&forms_steps(snapshot)) }
    
        })
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

//#region 🔖️Descriptor
/// 💡️ Registers `s.forms.forms.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `forms_artifact_schema_descriptor`'s registration.
pub fn forms_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.forms.forms.inference",
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
pub use super::topology::FormsTopology;
//#endregion 🔁️Re-exports
