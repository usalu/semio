//! 💡️ Din18599 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::din18599::Din18599Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Din18599Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din18599 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599.inference")]
pub struct Din18599Inference {
    #[derived]
    pub outline: Din18599Outline,
}

impl protocol::Inference<Din18599Snapshot> for Din18599Inference {
    async fn infer(snapshot: &Din18599Snapshot) -> Self {
        Self { outline: Din18599Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Din18599Snapshot> for Din18599Inference {
    async fn inference_schema_id() -> &'static str {
        "s.norm.din18599.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.din18599.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::din18599::standards::v1::subsets::any::schema::Din18599Builder {
    type Snapshot = Din18599Snapshot;
    type Inference = Din18599Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din18599.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `din18599_artifact_schema_descriptor`'s registration.
pub async fn din18599_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.din18599.inference",
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

    #[test]
    async fn inference_determinism_law() {
        let snapshot = Din18599Snapshot::default();
        assert_eq!(Din18599Inference::infer(&snapshot), Din18599Inference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(Din18599Inference::infer(&Din18599Snapshot::default()), Din18599Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::din18599::standards::v1::subsets::any::schema::{part_1, part_10, part_11, part_12, part_2, part_3, part_4, part_5, part_6, part_7, part_8, part_9};
use crate::artifacts::din18599::BalancingInputs;
/// 📋️ Full DIN V 18599 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `Din18599Snapshot -> CheckReport` projection; `balance_annual`
/// composes every `part_N::check` (pure helpers living in the parent `🧬️schema`).
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, NormError, Quantity};

/// 📋️ Full annual balancing per DIN V 18599.
pub async fn balance_annual(inputs: &BalancingInputs) -> Result<CheckReport, NormError> {
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
pub async fn evaluate(document: &Din18599Snapshot) -> CheckReport {
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
mod compliance_report_tests {
    use super::*;
    use crate::artifacts::din18599::standards::v1::subsets::any::schema::{from_building, reference_wall_layers};
    use crate::document::ClimateZoneDe;

    async fn reference_100m2_inputs() -> BalancingInputs {
        from_building(&reference_wall_layers(), 100.0, 4, ClimateZoneDe::Zone2, 0.0).unwrap()
    }

    #[test]
    async fn balance_annual_includes_all_parts() {
        let inputs = reference_100m2_inputs();
        let report = balance_annual(&inputs).unwrap();
        assert_eq!(report.checks.len(), 12);
    }

    #[test]
    async fn part_1_check_reached_via_balance_annual() {
        let inputs = reference_100m2_inputs();
        let check = part_1::check(&inputs).unwrap();
        assert_eq!(check.clause.family, "DIN V 18599-1");
        let report = balance_annual(&inputs).unwrap();
        assert!(report.checks.iter().any(|c| c.clause.family == "DIN V 18599-1" && c.clause.part == "§6"));
    }
}
//#endregion 🧪️ComplianceReportTests
