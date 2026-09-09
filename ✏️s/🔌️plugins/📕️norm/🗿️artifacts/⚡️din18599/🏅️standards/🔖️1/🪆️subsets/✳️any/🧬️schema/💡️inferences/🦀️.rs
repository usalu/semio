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
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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

impl protocol::InferenceSpec<Din18599Snapshot> for Din18599Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.din18599.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.din18599.inference.outline", reads: &[] }]
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
/// 📋️ Full DIN V 18599 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `Din18599Snapshot -> CheckReport` projection; `balance_annual`
/// composes every `part_N::check` (pure helpers living in the parent `🧬️schema`).
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, NormError, Quantity};
use crate::standards::v1::subsets::any::schema::{part_1, part_10, part_11, part_12, part_2, part_3, part_4, part_5, part_6, part_7, part_8, part_9};
use crate::BalancingInputs;
/// 📋️ Full annual balancing per DIN V 18599.
pub fn balance_annual(inputs: &BalancingInputs) -> Result<CheckReport, NormError> {
    let mut report = CheckReport::default();
    report.push(part_1::check(inputs)?);
    report.push(part_2::check(inputs)?);
    report.push(part_3::check(inputs)?);
    report.push(part_4::check(inputs)?);
    report.push(part_5::check(inputs)?);
    report.push(part_6::check(inputs)?);
    report.push(part_7::check(inputs)?);
    report.push(part_8::check(inputs)?);
    report.push(part_9::check(inputs)?);
    report.push(part_10::check(inputs)?);
    report.push(part_11::check(inputs)?);
    report.push(part_12::check(inputs)?);
    Ok(report)
}

/// 📋️ `Din18599Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &Din18599Snapshot) -> CheckReport {
    balance_annual(document).unwrap_or_else(|err| {
        let mut report = CheckReport::default();
        report.push(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599", "input", "1"),
            Quantity::new(crate::document::QuantityKind::Dimensionless, 2.0),
            Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            err.to_string(),
            AnnexChoice::De,
        ));
        report
    })
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
