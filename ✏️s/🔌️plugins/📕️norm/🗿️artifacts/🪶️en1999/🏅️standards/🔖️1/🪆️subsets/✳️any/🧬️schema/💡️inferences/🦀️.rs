//! 💡️ En1999 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1999Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1999 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1999.inference")]
pub struct En1999Inference {
    #[derived]
    pub outline: En1999Outline,
}

impl protocol::Inference<En1999Snapshot> for En1999Inference {
    fn infer(snapshot: &En1999Snapshot) -> Self {
        Self { outline: En1999Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1999Snapshot> for En1999Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1999.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.en1999.inference.outline",
            reads: &["annex", "materials", "sections", "members", "connections", "fireScenarios", "fatigueDetails"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1999Builder {
    type Snapshot = En1999Snapshot;
    type Inference = En1999Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1999.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1999_artifact_schema_descriptor`'s registration.
pub fn en1999_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1999.inference",
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
/// 📋️ Full EN 1999 compliance-report — hierarchical aluminium-structure evaluate.
use crate::document::CheckReport;
use crate::standards::v1::subsets::any::schema::evaluate_structure;

/// 🧮️ Headless per-document evaluation for EN 1999.
pub fn evaluate(document: &En1999Snapshot) -> CheckReport {
    evaluate_structure(document)
}

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1999Outline;
//#endregion 🔁️Re-exports
