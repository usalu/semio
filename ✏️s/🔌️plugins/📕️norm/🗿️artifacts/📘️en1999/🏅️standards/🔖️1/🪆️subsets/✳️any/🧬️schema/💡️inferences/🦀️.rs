//! 💡️ En1999 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1999::En1999Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1999Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1999 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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
        &[protocol::InferenceFieldSpec { id: "s.norm.en1999.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1999::standards::v1::subsets::any::schema::En1999Builder {
    type Snapshot = En1999Snapshot;
    type Inference = En1999Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1999.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1999_artifact_schema_descriptor`'s registration.
pub fn en1999_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1999.inference",
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
        let snapshot = En1999Snapshot::default();
        assert_eq!(En1999Inference::infer(&snapshot), En1999Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    fn inference_default_law() {
        assert_eq!(En1999Inference::infer(&En1999Snapshot::default()), En1999Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::en1999::standards::v1::subsets::any::schema::{check_aluminium_member, na_de, part_1_1, part_1_2, part_1_3, part_1_4, part_1_5};
/// 📋️ Full EN 1999 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1999Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport};

/// 📋️ Full EN 1999 check spanning every remaining part: 1-1 (cross-section, buckling, bending, welds), 1-2 (fire), 1-3 (fatigue), 1-4 (cold-formed sheeting), 1-5 (shell buckling).
#[allow(clippy::too_many_arguments)]
pub fn check_full_aluminium(
    n_ed_kn: f64,
    m_ed_knm: f64,
    a_mm2: f64,
    w_el_mm3: f64,
    alloy: part_1_1::Alloy,
    chi: f64,
    i_t_mm4: f64,
    l_cr_mm: f64,
    theta_c: f64,
    delta_sigma_ed: f64,
    delta_sigma_c: f64,
    fatigue_m: f64,
    n_cycles: f64,
    v_weld_ed_kn: f64,
    weld_throat_mm: f64,
    weld_length_mm: f64,
    beta_w: f64,
    sheet_b_mm: f64,
    sheet_t_mm: f64,
    sheet_k_sigma: f64,
    sheet_w_el_mm3: f64,
    sheet_m_ed_knm: f64,
    shell_t_mm: f64,
    shell_r_mm: f64,
    sigma_ed_shell_mpa: f64,
    annex: AnnexChoice,
) -> CheckReport {
    let params = na_de::AnnexParams::for_choice(annex);
    let mut report = check_aluminium_member(n_ed_kn, m_ed_knm, a_mm2, w_el_mm3, alloy, chi, i_t_mm4, l_cr_mm, annex);
    let theta_cr = part_1_2::critical_temperature_c(alloy.f_0_2_mpa());
    report.push(part_1_2::check_fire_protection(theta_c, theta_cr, annex));
    let delta_sigma_rd = part_1_3::fatigue_strength_mpa(delta_sigma_c, fatigue_m, n_cycles);
    report.push(part_1_3::check_fatigue(delta_sigma_ed, delta_sigma_rd, annex));
    let a_w = part_1_1::weld_throat_area_mm2(weld_throat_mm, weld_length_mm);
    let v_weld_rd = part_1_1::weld_resistance_kn(a_w, alloy.f_u_mpa(), beta_w, params.gamma_m2);
    report.push(part_1_1::check_welded_joint(v_weld_ed_kn, v_weld_rd, annex));
    let lambda_p = part_1_4::plate_slenderness(sheet_b_mm, sheet_t_mm, sheet_k_sigma, alloy);
    let rho = part_1_4::effective_width_factor(lambda_p);
    let w_eff = part_1_4::effective_section_modulus_mm3(sheet_w_el_mm3, rho);
    report.push(part_1_4::check_cold_formed_sheeting(sheet_m_ed_knm, w_eff, alloy, params.gamma_m1, annex));
    let sigma_cr = part_1_5::critical_axial_stress_mpa(shell_t_mm, shell_r_mm);
    let lambda_bar = part_1_5::relative_slenderness(alloy.f_0_2_mpa(), sigma_cr);
    let chi_shell = part_1_5::buckling_reduction_factor(lambda_bar);
    let sigma_rd_shell = part_1_5::design_buckling_stress_mpa(chi_shell, alloy.f_0_2_mpa(), params.gamma_m1);
    report.push(part_1_5::check_shell_buckling(sigma_ed_shell_mpa, sigma_rd_shell, annex));
    report
}

fn parse_alloy(value: &str) -> part_1_1::Alloy {
    match value.to_ascii_lowercase().as_str() {
        "aw6082t6" => part_1_1::Alloy::Aw6082T6,
        _ => part_1_1::Alloy::Aw6060T6,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1999Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &En1999Snapshot) -> CheckReport {
    check_full_aluminium(
        document.n_ed_kn,
        document.m_ed_knm,
        document.a_mm2,
        document.w_el_mm3,
        parse_alloy(&document.alloy),
        document.chi,
        document.i_t_mm4,
        document.l_cr_mm,
        document.theta_c,
        document.delta_sigma_ed,
        document.delta_sigma_c,
        document.fatigue_m,
        document.n_cycles,
        document.v_weld_ed_kn,
        document.weld_throat_mm,
        document.weld_length_mm,
        document.beta_w,
        document.sheet_b_mm,
        document.sheet_t_mm,
        document.sheet_k_sigma,
        document.sheet_w_el_mm3,
        document.sheet_m_ed_knm,
        document.shell_t_mm,
        document.shell_r_mm,
        document.sigma_ed_shell_mpa,
        document.annex,
    )
}

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn full_aluminium_worked_example() {
        let alloy = part_1_1::Alloy::Aw6060T6;
        let report = check_full_aluminium(80.0, 4.0, 1200.0, 24_000.0, alloy, 0.85, 5000.0, 3000.0, 200.0, 45.0, 71.0, 8.0, 500_000.0, 25.0, 4.0, 120.0, 0.63, 200.0, 2.0, 4.0, 8000.0, 0.5, 4.0, 500.0, 150.0, AnnexChoice::De);
        assert_eq!(report.checks.len(), 8);
        assert!(report.checks[4].utilization < 1.0);
    }

    #[semio_framework_async_macros::async_test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&En1999Snapshot::default());
        assert_eq!(report.checks.len(), 8);
    }

    #[semio_framework_async_macros::async_test]
    fn annex_en_de_documented_equality() {
        // 📖️ DIN EN 1999-1-1/NA does not override γ_M1/γ_M2, so EN and DE-NA must yield identical utilization.
        let en_doc = En1999Snapshot { annex: AnnexChoice::En, ..En1999Snapshot::default() };
        let de_doc = En1999Snapshot { annex: AnnexChoice::De, ..En1999Snapshot::default() };
        let en_report = evaluate(&en_doc);
        let de_report = evaluate(&de_doc);
        assert_eq!(en_report.checks.len(), de_report.checks.len());
        for (en_check, de_check) in en_report.checks.iter().zip(de_report.checks.iter()) {
            assert!((en_check.utilization - de_check.utilization).abs() < 1e-9);
        }
    }
}
//#endregion 🧪️ComplianceReportTests
