//! 💡️ En1990 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1990Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;


//#region 🔖️Inference
/// 💡️ Everything inferable from a en1990 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
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
    fn infer(snapshot: &En1990Snapshot) -> Self {
        Self { outline: En1990Outline::compute(snapshot) }
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
        &[protocol::InferenceFieldSpec { id: "s.norm.en1990.inference.outline", reads: &["g_k", "q_k", "resistance_kn", "consequence_class", "annex", "seismic_a_ed_kn"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1990Builder {
    type Snapshot = En1990Snapshot;
    type Inference = En1990Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1990.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1990_artifact_schema_descriptor`'s registration.
pub fn en1990_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1990.inference",
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

//#region 🔖️ComplianceReport
use crate::standards::v1::subsets::any::schema::{append_combination_set, check_reliability_index, check_seismic_situation, ActionSet, NaDe, NaEn, NationalAnnexes};
use crate::En1990QkEntry;
/// 📋️ Full EN 1990 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1990Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport, DesignSituation};
/// 🔁️ Convert a `En1990Snapshot`'s `q_k` entries (read through the `en1990_qk` working-scene
/// accessor — `q_k` is a composed `s.stdio.semio.table` child slot, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) into the plain `(category, value)` pairs
/// `ActionSet` expects.
fn action_set_from_document(document: &En1990Snapshot) -> ActionSet {
    ActionSet { g_k: document.g_k, q_k: crate::en1990_qk(document).iter().map(|entry: &En1990QkEntry| (entry.category.clone(), entry.value)).collect() }
}

/// 📋️ `En1990Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &En1990Snapshot) -> CheckReport {
    let actions = action_set_from_document(document);
    // 🔀️ O1 de-dyn: runtime-chosen concrete type (was `&dyn NationalAnnex`) — the closed-set enum
    // `NationalAnnexes` (`dyn_enum_close!` in en1990's schema module) replaces the trait object.
    let annex: NationalAnnexes = if document.annex == AnnexChoice::De { NaDe.into() } else { NaEn.into() };
    let mut report = CheckReport::default();
    append_combination_set(&mut report, &annex, DesignSituation::Persistent, &actions, document.resistance_kn);
    append_combination_set(&mut report, &annex, DesignSituation::Accidental, &actions, document.resistance_kn);
    report.push(check_seismic_situation(&annex, &actions, document.seismic_a_ed_kn, document.resistance_kn));
    report.push(check_reliability_index(3.9, document.consequence_class));
    report
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1990Outline;
//#endregion 🔁️Re-exports
