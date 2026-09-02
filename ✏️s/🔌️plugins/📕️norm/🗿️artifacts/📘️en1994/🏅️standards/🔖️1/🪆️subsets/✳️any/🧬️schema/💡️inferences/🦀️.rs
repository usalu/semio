//! 💡️ En1994 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1994::En1994Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1994Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1994 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994.inference")]
pub struct En1994Inference {
    #[derived]
    pub outline: En1994Outline,
}

impl protocol::Inference<En1994Snapshot> for En1994Inference {
    fn infer(snapshot: &En1994Snapshot) -> Self {
        Self { outline: En1994Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1994Snapshot> for En1994Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1994.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1994.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1994::standards::v1::subsets::any::schema::En1994Builder {
    type Snapshot = En1994Snapshot;
    type Inference = En1994Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1994.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1994_artifact_schema_descriptor`'s registration.
pub fn en1994_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1994.inference",
        inference: schema::FacetLeaves {
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
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    fn inference_determinism_law() {
        let snapshot = En1994Snapshot::default();
        assert_eq!(En1994Inference::infer(&snapshot), En1994Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    fn inference_default_law() {
        assert_eq!(En1994Inference::infer(&En1994Snapshot::default()), En1994Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::en1994::standards::v1::subsets::any::schema::{check_composite_beam, part_1_1, part_1_2, part_2};
/// 📋️ Full EN 1994 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1994Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

fn parse_fire_rating(value: &str) -> part_1_2::FireRating {
    match value.to_ascii_lowercase().as_str() {
        "r30" => part_1_2::FireRating::R30,
        "r90" => part_1_2::FireRating::R90,
        "r120" => part_1_2::FireRating::R120,
        _ => part_1_2::FireRating::R60,
    }
}

/// 📋️ Full EN 1994 check across composite bending/shear, stud resistance, shear connection degree, fire, and bridge fatigue parts.
#[allow(clippy::too_many_arguments)]
pub fn check_full_composite(
    m_ed_knm: f64,
    v_ed_kn: f64,
    m_pla: f64,
    m_pl_rd: f64,
    eta: f64,
    v_l_rd: f64,
    insulation_thickness_mm: f64,
    fire_rating: &str,
    deck_type: &str,
    delta_sigma_mpa: f64,
    fatigue_detail: &str,
    annex: AnnexChoice,
    d_mm: f64,
    h_sc_mm: f64,
    f_ck_mpa: f64,
    f_u_mpa: f64,
    e_cm_mpa: f64,
    v_ed_per_stud_kn: f64,
    span_m: f64,
    f_y_mpa: f64,
    n_cycles_stud: f64,
    delta_tau_stud_mpa: f64,
) -> CheckReport {
    let mut report = check_composite_beam(m_ed_knm, v_ed_kn, m_pla, m_pl_rd, eta, v_l_rd, annex);
    report.push(part_1_1::check_stud_resistance(v_ed_per_stud_kn, d_mm, h_sc_mm, f_ck_mpa, f_u_mpa, e_cm_mpa, annex));
    report.push(part_1_1::check_shear_connection_degree(eta, span_m, f_y_mpa, annex));
    report.push(part_1_2::check_fire_composite(insulation_thickness_mm, parse_fire_rating(fire_rating), deck_type));
    let category = part_2::bridge_fatigue_category(fatigue_detail);
    report.push(CheckResult::from_utilization(ClauseId::new("EN 1994-2", "§8", "8.1"), Quantity::stress_mpa(delta_sigma_mpa), Quantity::stress_mpa(category as f64), "bridge composite fatigue", AnnexChoice::En));
    report.push(part_2::check_stud_fatigue(delta_tau_stud_mpa, n_cycles_stud));
    report
}

/// 📋️ `En1994Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &En1994Snapshot) -> CheckReport {
    check_full_composite(
        document.m_ed_knm,
        document.v_ed_kn,
        document.m_pla,
        document.m_pl_rd,
        document.eta,
        document.v_l_rd,
        document.insulation_thickness_mm,
        &document.fire_rating,
        &document.deck_type,
        document.delta_sigma_mpa,
        &document.fatigue_detail,
        document.annex,
        document.d_mm,
        document.h_sc_mm,
        document.f_ck_mpa,
        document.f_u_mpa,
        document.e_cm_mpa,
        document.v_ed_per_stud_kn,
        document.span_m,
        document.f_y_mpa,
        document.n_cycles_stud,
        document.delta_tau_stud_mpa,
    )
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn full_composite_worked_example() {
        let report = check_full_composite(180.0, 110.0, 80.0, 250.0, 0.75, 150.0, 20.0, "r60", "trapezoidal", 55.0, "stud_welded", AnnexChoice::De, 19.0, 95.0, 30.0, 450.0, 33_000.0, 40.0, 8.0, 355.0, 2_000_000.0, 40.0);
        assert_eq!(report.checks.len(), 7);
        let m_rd = part_1_1::plastic_moment_partial_knm(80.0, 250.0, 0.75);
        assert!((m_rd - 207.5).abs() < 0.1);
        assert!(report.checks[0].utilization < 1.0);
        assert!(report.checks[2].utilization < 1.0, "stud resistance check should pass");
        assert!(report.checks[3].utilization < 1.0, "shear connection degree check should pass");
        assert!(report.checks[4].utilization < 1.0, "fire check should pass");
        assert!(report.checks[6].utilization < 1.0, "stud fatigue check should pass");
    }

    #[semio_framework_async_macros::async_test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&En1994Snapshot::default());
        assert_eq!(report.checks.len(), 7);
    }
}
//#endregion 🧪️ComplianceReportTests
