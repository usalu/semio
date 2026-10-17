//! 💡️ En1990 inference schema — outline + full basis-of-design evaluate().

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1990 snapshot.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990.inference")]
pub struct En1990Inference {
    #[derived]
    pub outline: En1990Outline,
}

impl protocol::Inference<En1990Snapshot> for En1990Inference {
    fn infer(snapshot: &En1990Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: En1990Outline::compute(snapshot) }

        })
    }
}

impl protocol::InferenceSpec<En1990Snapshot> for En1990Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1990.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.en1990.inference.outline",
            reads: &[
                "annex",
                "projectId",
                "structureKind",
                "altitudeM",
                "consequenceClass",
                "reliabilityClass",
                "designWorkingLifeCategory",
                "designWorkingLifeYears",
                "referencePeriodYears",
                "supervisionLevel",
                "inspectionLevel",
                "kFiDeclared",
                "betaComputed",
                "permanents",
                "variables",
                "accidentals",
                "seismics",
                "members",
                "bridgeSls",
                "effects",
            ],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1990.inference`'s facet leaves into the OS-wide inference catalog.
pub fn en1990_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.en1990.inference",
        inference: semio_framework_schema_registry::FacetLeaves {
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

//#region 🔖️ComplianceReport
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
#[cfg(test)] use semio_s_plugin_norm_compliance::DesignSituation;
use semio_s_artifact_norm_en1990_checks::{check_reliability_index_for, combination_6_11, combination_6_12b, combination_equ_for, combination_geo_a24c, combination_sls_char, combination_sls_frequent, combination_sls_quasi_permanent, combination_str_geo_610a_for, combination_str_geo_610b_for, k_fi, resolve_psi_category, ActionSet, NaDe, NaEn, NationalAnnexes};
use semio_s_plugin_norm_compliance::NationalAnnex;
use semio_s_artifact_norm_en1990_model::{Member, MemberEffect};

#[path = "🧰️report-helpers/🦀️.rs"]
mod report_helpers;
pub(crate) use report_helpers::*;

#[path = "🏗️ultimate-limit-state/🦀️.rs"]
mod ultimate_limit_state;
use ultimate_limit_state::*;

#[path = "📏️serviceability-and-fatigue/🦀️.rs"]
mod serviceability_and_fatigue;
use serviceability_and_fatigue::*;

#[path = "🌍️working-life-and-bridge-annexes/🦀️.rs"]
mod working_life_and_bridge_annexes;
use working_life_and_bridge_annexes::*;

#[path = "🧮️partial-factor-sets/🦀️.rs"]
mod partial_factor_sets;
use partial_factor_sets::*;

#[path = "📋️compliance-evaluation/🦀️.rs"]
mod compliance_evaluation;
pub use compliance_evaluation::*;

#[path = "⛓️referential-integrity/🦀️.rs"]
mod referential_integrity;
use referential_integrity::*;

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
pub use crate::outline::En1990Outline;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts
