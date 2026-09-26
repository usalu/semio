//! 💡️ Din18599 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::Din18599Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din18599 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599.inference")]
pub struct Din18599Inference {
    #[derived]
    pub outline: Din18599Outline,
}

impl protocol::Inference<Din18599Snapshot> for Din18599Inference {
    fn infer(snapshot: &Din18599Snapshot) -> Self {
        Self { outline: Din18599Outline::compute(snapshot) }
    }
}

impl Default for Din18599Inference {
    fn default() -> Self {
        <Self as protocol::Inference<Din18599Snapshot>>::infer(&Din18599Snapshot::default())
    }
}

impl protocol::InferenceSpec<Din18599Snapshot> for Din18599Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.din18599.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.din18599.inference.outline",
            reads: &[
                "buildingCategory",
                "attachment",
                "useClass",
                "method",
                "netFloorAreaM2",
                "heatedVolumeM3",
                "gegQpFactor",
                "deltaUWbWM2k",
                "automationClass",
                "zones",
                "elements",
                "heating",
                "dhw",
                "ventilation",
                "cooling",
                "lighting",
                "renewables",
                "climate",
            ],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Din18599Builder {
    type Snapshot = Din18599Snapshot;
    type Inference = Din18599Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din18599.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `din18599_artifact_schema_descriptor`'s registration.
pub fn din18599_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.din18599.inference",
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
use crate::standards::v1::subsets::any::schema::evaluate_document;

/// 📋️ Full annual balancing / GEG evaluation per DIN V 18599.
pub fn balance_annual(inputs: &Din18599Snapshot) -> CheckReport {
    evaluate_document(inputs)
}

/// � tang `Din18599Snapshot -> CheckReport` conformance law — building energy / GEG evaluation.
pub fn evaluate(document: &Din18599Snapshot) -> CheckReport {
    evaluate_document(document)
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::Din18599Outline;
//#endregion 🔁️Re-exports
