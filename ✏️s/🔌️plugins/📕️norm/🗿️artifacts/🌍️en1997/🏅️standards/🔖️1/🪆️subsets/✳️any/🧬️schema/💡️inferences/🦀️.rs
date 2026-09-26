//! 💡️ En1997 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1997Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1997 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1997.inference")]
pub struct En1997Inference {
    #[derived]
    pub outline: En1997Outline,
}

impl Default for En1997Inference {
    fn default() -> Self {
        use protocol::Inference;
        Self::infer(&En1997Snapshot::default())
    }
}

impl protocol::Inference<En1997Snapshot> for En1997Inference {
    fn infer(snapshot: &En1997Snapshot) -> Self {
        Self { outline: En1997Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1997Snapshot> for En1997Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1997.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1997.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1997Builder {
    type Snapshot = En1997Snapshot;
    type Inference = En1997Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1997.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1997_artifact_schema_descriptor`'s registration.
pub fn en1997_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1997.inference",
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
use crate::document::CheckReport;
use crate::standards::v1::subsets::any::schema::check_project;

/// 🧮️ Headless per-document evaluation for EN 1997.
pub fn evaluate(document: &En1997Snapshot) -> CheckReport {
    check_project(document)
}

/// 📋️ Compatibility alias used by older call sites.
pub fn check_full_geotechnical(document: &En1997Snapshot) -> CheckReport {
    check_project(document)
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1997Outline;
//#endregion 🔁️Re-exports
