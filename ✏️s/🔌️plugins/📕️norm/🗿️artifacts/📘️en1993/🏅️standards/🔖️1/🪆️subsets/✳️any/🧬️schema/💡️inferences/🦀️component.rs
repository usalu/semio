//! 💡️ En1993 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1993::En1993Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1993Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1993 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1993.inference")]
pub struct En1993Inference {
    #[derived]
    pub outline: En1993Outline,
}

impl protocol::Inference<En1993Snapshot> for En1993Inference {
    fn infer(snapshot: &En1993Snapshot) -> Self {
        Self { outline: En1993Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1993Snapshot> for En1993Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1993.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1993.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1993::standards::v1::subsets::any::schema::En1993Builder {
    type Snapshot = En1993Snapshot;
    type Inference = En1993Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1993.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1993_artifact_schema_descriptor`'s registration.
pub fn en1993_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1993.inference",
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

    #[semio_framework_async_macros::async_test]
    fn inference_determinism_law() {
        let snapshot = En1993Snapshot::default();
        assert_eq!(En1993Inference::infer(&snapshot), En1993Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    fn inference_default_law() {
        assert_eq!(En1993Inference::infer(&En1993Snapshot::default()), En1993Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::en1993::standards::v1::subsets::any::schema::{
    check_steel_member, part_1_1, part_1_10, part_1_11, part_1_12, part_1_2, part_1_3, part_1_4, part_1_5, part_1_6, part_1_8, part_1_9, part_2, part_3, part_4, part_5, part_6, AnnexParams,
};
/// 📋️ Full EN 1993 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1993Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::CheckReport;

fn parse_fire_rating(value: &str) -> part_1_2::FireRating {
    match value.to_ascii_lowercase().as_str() {
        "r30" => part_1_2::FireRating::R30,
        "r90" => part_1_2::FireRating::R90,
        "r120" => part_1_2::FireRating::R120,
        _ => part_1_2::FireRating::R60,
    }
}

fn parse_fatigue_method(value: &str) -> part_1_9::AssessmentMethod {
    match value.to_ascii_lowercase().as_str() {
        "low_consequence" => part_1_9::AssessmentMethod::LowConsequence,
        "safe_life" => part_1_9::AssessmentMethod::SafeLife,
        _ => part_1_9::AssessmentMethod::DamageTolerant,
    }
}

/// 📋️ Full steel member check across all sixteen EN 1993 parts (1-1 through 6), each reached from `evaluate`.
pub fn check_full_steel_member(document: &En1993Snapshot) -> CheckReport {
    let annex = document.annex;
    let params = AnnexParams { gamma_mf: parse_fatigue_method(&document.fatigue_method).gamma_mf(), ..AnnexParams::for_choice(annex) };

    let mut report = check_steel_member(document.n_ed_kn, document.m_ed_knm, document.a_mm2, document.w_pl_mm3, document.f_y_mpa, document.chi);

    // #region 🔖️Part1_1
    let v_rd = part_1_1::shear_resistance_kn(document.a_v_mm2, document.f_y_mpa, params);
    report.push(part_1_1::check_shear(document.v_ed_kn, v_rd, annex));
    let n_t_rd = part_1_1::net_tension_resistance_kn(document.a_mm2, document.a_net_mm2, document.f_y_mpa, document.f_u_mpa, params);
    report.push(part_1_1::check_net_tension(document.tension_n_ed_kn, n_t_rd, annex));
    // #endregion 🔖️Part1_1

    // #region 🔖️Part1_2
    report.push(part_1_2::check_fire_protection(document.fire_thickness_mm, parse_fire_rating(&document.fire_rating), document.fire_massivity, annex));
    report.push(part_1_2::check_critical_temperature(document.fire_mu_0, document.fire_design_temperature_c, annex));
    // #endregion 🔖️Part1_2

    // #region 🔖️Part1_3
    let cf_lambda_p = part_1_3::lambda_p(document.cf_b_bar_mm, document.cf_t_mm, document.f_y_mpa, document.cf_k_sigma);
    let cf_rho = part_1_3::reduction_factor(cf_lambda_p, document.cf_psi);
    let cf_n_eff_rd = part_1_3::effective_resistance_kn(cf_rho, document.cf_gross_resistance_kn);
    report.push(part_1_3::check_cold_formed_effective_section(document.cf_n_ed_kn, cf_n_eff_rd, annex));
    // #endregion 🔖️Part1_3

    // #region 🔖️Part1_4
    let stainless_m_rd = part_1_4::bending_resistance_knm(document.stainless_w_pl_mm3, document.stainless_f_y_mpa);
    report.push(part_1_4::check_stainless_steel(document.stainless_m_ed_knm, stainless_m_rd, annex));
    // #endregion 🔖️Part1_4

    // #region 🔖️Part1_5
    let plated_sigma_rd = part_1_5::local_buckling_stress_rd_mpa(document.f_y_mpa, document.plated_lambda_p, params);
    report.push(part_1_5::check_plated_buckling(document.plated_sigma_ed_mpa, plated_sigma_rd, annex));
    // #endregion 🔖️Part1_5

    // #region 🔖️Part1_6
    let shell_sigma_x_rcr = part_1_6::sigma_x_rcr_mpa(document.silo_t_mm, document.silo_r_mm, 210_000.0);
    let shell_lambda_bar = part_1_6::lambda_bar(document.f_y_mpa, shell_sigma_x_rcr);
    let shell_alpha = part_1_6::alpha_imperfection(document.silo_r_mm, document.silo_t_mm);
    let shell_chi = part_1_6::chi(shell_lambda_bar, shell_alpha);
    let shell_sigma_x_rd = part_1_6::design_resistance_mpa(document.f_y_mpa, shell_chi, params);
    report.push(part_1_6::check_shell_buckling(document.shell_sigma_x_ed_mpa, shell_sigma_x_rd, annex));
    // #endregion 🔖️Part1_6

    // #region 🔖️Part1_8
    let bearing_alpha_b = part_1_8::bearing_alpha_b(document.bolt_e1_mm, document.bolt_d0_mm, document.bolt_f_ub_mpa, document.bolt_f_u_mpa);
    let bearing_k1 = part_1_8::bearing_k1(document.bolt_e2_mm, document.bolt_d0_mm);
    let bolt_v_rd = part_1_8::bolt_shear_resistance_kn(document.bolt_n_bolts, document.bolt_a_s_mm2, document.bolt_f_ub_mpa, params.gamma_m2);
    let bolt_b_rd = part_1_8::bolt_bearing_resistance_kn(bearing_k1, bearing_alpha_b, document.bolt_f_u_mpa, document.bolt_d_mm, document.bolt_t_mm, params.gamma_m2);
    report.push(part_1_8::check_bolt_shear(document.bolt_f_ed_kn, bolt_v_rd, annex));
    report.push(part_1_8::check_bolt_bearing(document.bolt_f_ed_kn, bolt_b_rd, annex));
    let weld_beta_w = part_1_8::beta_w(&document.weld_steel_grade);
    let weld_w_rd = part_1_8::fillet_weld_resistance_kn(document.weld_a_mm, document.weld_l_mm, document.weld_f_u_mpa, weld_beta_w, params.gamma_m2);
    report.push(part_1_8::check_fillet_weld(document.weld_f_ed_kn, weld_w_rd, annex));
    // #endregion 🔖️Part1_8

    // #region 🔖️Part1_9
    report.push(part_1_9::check_fatigue_range(document.delta_sigma_mpa, document.fatigue_category, params.gamma_mf, annex));
    // #endregion 🔖️Part1_9

    // #region 🔖️Part1_10
    report.push(part_1_10::check_through_thickness(document.t10_actual_thickness_mm, &document.t10_steel_subgrade, document.t10_t_ed_c, annex));
    // #endregion 🔖️Part1_10

    // #region 🔖️Part1_11
    let tension_component_rd = part_1_11::tension_component_resistance_kn(document.tension_component_f_uk_kn, document.tension_component_f_k_kn);
    report.push(part_1_11::check_tension_component(document.tension_component_n_ed_kn, tension_component_rd, annex));
    // #endregion 🔖️Part1_11

    // #region 🔖️Part1_12
    let hss_m_rd = part_1_12::elastic_bending_resistance_knm(document.hss_w_el_mm3, document.hss_f_y_mpa, document.hss_section_class, params);
    report.push(part_1_12::check_high_strength_bending(document.hss_m_ed_knm, hss_m_rd, annex));
    // #endregion 🔖️Part1_12

    // #region 🔖️Part2
    let n_rd = part_1_1::axial_resistance_kn(document.a_mm2, document.f_y_mpa, params);
    let m_rd = part_1_1::bending_resistance_knm(document.w_pl_mm3, document.f_y_mpa, params);
    report.push(part_2::check_steel_bridge(document.n_ed_kn, n_rd, document.m_ed_knm, m_rd, annex));
    report.push(part_2::check_bridge_fatigue(document.bridge_lambda, document.bridge_phi_2, document.bridge_delta_sigma_p_mpa, document.fatigue_category, params.gamma_mf, annex));
    // #endregion 🔖️Part2

    // #region 🔖️Part3
    let tower_n_b_rd = part_3::tower_buckling_kn(document.a_mm2, document.f_y_mpa, document.chi, document.tower_wind_factor, params);
    report.push(part_3::check_tower_buckling(document.tower_n_ed_kn, tower_n_b_rd, annex));
    // #endregion 🔖️Part3

    // #region 🔖️Part4
    let silo_p_h = part_4::janssen_pressure_kpa(document.silo_k, document.silo_gamma_kn_m3, document.silo_depth_m);
    let silo_sigma_ed = part_4::membrane_hoop_stress_mpa(silo_p_h, document.silo_r_mm, document.silo_t_mm);
    report.push(part_4::check_silo_wall(silo_sigma_ed, shell_sigma_x_rd, annex));
    // #endregion 🔖️Part4

    // #region 🔖️Part5
    report.push(part_5::check_pile_driving_stress(document.pile_sigma_mpa, document.f_y_mpa, annex));
    let pile_n_rd = part_5::pile_compression_kn(document.a_mm2, document.f_y_mpa, document.pile_k_red, params);
    report.push(part_5::check_pile_foundation_steel(document.pile_n_ed_kn, pile_n_rd, annex));
    // #endregion 🔖️Part5

    // #region 🔖️Part6
    let crane_l_eff = part_6::effective_length_mm(document.crane_wheel_contact_length_mm, document.crane_dispersion_mm);
    let crane_sigma_oz = part_6::wheel_load_web_stress_mpa(document.crane_f_z_ed_kn, crane_l_eff, document.crane_t_w_mm);
    report.push(part_6::check_crane_runway_web(crane_sigma_oz, document.f_y_mpa, params, annex));
    // #endregion 🔖️Part6

    report
}

/// 📋️ `En1993Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &En1993Snapshot) -> CheckReport {
    check_full_steel_member(document)
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn full_steel_member_e2e() {
        let report = check_full_steel_member(&En1993Snapshot::default());
        assert_eq!(report.checks.len(), 25);
    }

    #[semio_framework_async_macros::async_test]
    fn every_part_reaches_evaluate() {
        let report = check_full_steel_member(&En1993Snapshot::default());
        let families: std::collections::BTreeSet<&str> = report.checks.iter().map(|c| c.clause.family.as_str()).collect();
        for expected in
            ["EN 1993-1-1", "EN 1993-1-2", "EN 1993-1-3", "EN 1993-1-4", "EN 1993-1-5", "EN 1993-1-6", "EN 1993-1-8", "EN 1993-1-9", "EN 1993-1-10", "EN 1993-1-11", "EN 1993-1-12", "EN 1993-2", "EN 1993-3-1", "EN 1993-4-1", "EN 1993-5", "EN 1993-6"]
        {
            assert!(families.contains(expected), "missing checks sourced from {expected}");
        }
    }
}
//#endregion 🧪️ComplianceReportTests
