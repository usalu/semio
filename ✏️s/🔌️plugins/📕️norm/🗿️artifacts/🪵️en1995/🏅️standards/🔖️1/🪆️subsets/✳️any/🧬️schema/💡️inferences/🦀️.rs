//! 💡️ En1995 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1995Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1995 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995.inference")]
pub struct En1995Inference {
    #[derived]
    pub outline: En1995Outline,
}

impl protocol::Inference<En1995Snapshot> for En1995Inference {
    fn infer(snapshot: &En1995Snapshot) -> Self {
        Self { outline: En1995Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1995Snapshot> for En1995Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1995.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1995.inference.outline", reads: &["members", "connections", "annex"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1995Builder {
    type Snapshot = En1995Snapshot;
    type Inference = En1995Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1995.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1995_artifact_schema_descriptor`'s registration.
pub fn en1995_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1995.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
/// 📋️ `En1995Snapshot -> CheckReport` — full timber-structure evaluation.
pub fn evaluate(document: &En1995Snapshot) -> crate::document::CheckReport {
    crate::artifact_schema::evaluate_structure(document.annex, &document.members, &document.connections)
}
//#endregion 🔖️ComplianceReport

#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;

//#region 🔁️Re-exports
/// 🔁️ Outline type lives in the sibling `🧾outline/` slug.
pub use super::outline::En1995Outline;
//#endregion 🔁️Re-exports
