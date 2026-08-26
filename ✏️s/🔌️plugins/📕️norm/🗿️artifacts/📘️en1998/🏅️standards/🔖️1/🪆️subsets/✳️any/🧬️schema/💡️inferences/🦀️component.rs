//! 💡️ En1998 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1998::En1998Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1998Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1998 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998.inference")]
pub struct En1998Inference {
    #[derived]
    pub outline: En1998Outline,
}

impl protocol::Inference<En1998Snapshot> for En1998Inference {
    fn infer(snapshot: &En1998Snapshot) -> Self {
        Self { outline: En1998Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1998Snapshot> for En1998Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1998.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1998.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1998::standards::v1::subsets::any::schema::En1998Builder {
    type Snapshot = En1998Snapshot;
    type Inference = En1998Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1998.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1998_artifact_schema_descriptor`'s registration.
pub fn en1998_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1998.inference",
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
        let snapshot = En1998Snapshot::default();
        assert_eq!(En1998Inference::infer(&snapshot), En1998Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    fn inference_default_law() {
        assert_eq!(En1998Inference::infer(&En1998Snapshot::default()), En1998Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::en1998::standards::v1::subsets::any::schema::{check_building_seismic_with_annex, na_de, part_1, part_2, part_3, part_4, part_5, part_6, AnnexParams};
/// 📋️ Full EN 1998 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1998Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport};

/// 📋️ Full seismic check across EN 1998 parts 1 through 6.
pub fn check_full_seismic(document: &En1998Snapshot) -> CheckReport {
    let zone = parse_seismic_zone(document.seismic_zone);
    let ground = parse_ground_type(&document.ground_type);
    let importance = parse_importance(&document.importance_class);
    let system = parse_structural_system(&document.structural_system);
    let annex_choice = parse_annex(&document.annex);

    let annex = match annex_choice {
        AnnexChoice::En => AnnexParams::En { a_gr: document.en_a_gr, ground: parse_en_ground_type(&document.en_ground_type), spectrum: parse_spectrum_type(&document.en_spectrum_type) },
        AnnexChoice::De => AnnexParams::De { zone, ground },
    };
    let (a_g, s, tb, tc, td) = annex.ground_params();
    let gamma_i = importance.gamma_i();
    let q = system.q();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, document.t1_s);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let v_b = part_1::base_shear_from_design_kn(s_d, document.mass_t);

    let mut report = check_building_seismic_with_annex(&annex, importance, system, document.t1_s, document.mass_t, document.v_rd_kn, document.drift_mm, document.height_m, document.multiple_resisting_systems);

    let q_isol = part_2::isolation_reduction_factor(document.period_ratio);
    let s_d_isol = part_2::isolated_spectrum_sd(s_e, gamma_i, q_isol);
    let v_bridge = part_1::base_shear_from_design_kn(s_d_isol, document.mass_t);
    report.push(part_2::check_bridge_seismic(v_bridge, document.bridge_v_rd_kn));
    report.push(part_2::check_isolation_bearing(document.bearing_d_ed_mm, document.bearing_d_rd_mm));

    let kl = parse_knowledge_level(&document.retrofit_knowledge_level);
    let limit_state = parse_retrofit_limit_state(&document.retrofit_limit_state);
    report.push(part_3::check_element_capacity(document.retrofit_e_d_kn, document.retrofit_r_k_kn, kl.confidence_factor(), document.retrofit_gamma_el, limit_state, annex_choice));

    let h_over_r_silo = document.silo_height_m / document.silo_radius_m;
    let mu_i = part_4::impulsive_mass_ratio(h_over_r_silo);
    let mu_c = part_4::convective_mass_ratio(h_over_r_silo);
    let v_i = part_1::base_shear_from_design_kn(s_d, document.mass_t * mu_i);
    let t_c_silo = part_4::convective_period_s(document.silo_radius_m);
    let s_e_c_silo = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c_silo);
    let s_d_c_silo = part_1::design_spectrum_sd(s_e_c_silo, gamma_i, q);
    let v_c = part_1::base_shear_from_design_kn(s_d_c_silo, document.mass_t * mu_c);
    let v_silo = part_4::silo_base_shear_kn(v_i, v_c);
    report.push(part_4::check_silo_wall(v_silo, document.silo_n_rd_kn));
    report.push(part_4::check_silo_anchor(document.silo_v_ed_kn, document.silo_v_rd_kn));
    let _ = part_4::silo_behaviour_factor(document.silo_q_nominal);

    let h_over_r_tank = document.tank_height_m / document.tank_radius_m;
    let mu_i_tank = part_4::impulsive_mass_ratio(h_over_r_tank);
    let mu_c_tank = part_4::convective_mass_ratio(h_over_r_tank);
    let t_i_tank = part_4::impulsive_period_s(document.tank_height_m, document.tank_radius_m);
    let t_c_tank = part_4::convective_period_s(document.tank_radius_m);
    let s_e_i_tank = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_i_tank);
    let s_e_c_tank = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c_tank);
    let v_tank = part_4::tank_base_shear_kn(document.tank_mass_t * mu_i_tank, s_e_i_tank, document.tank_mass_t * mu_c_tank, s_e_c_tank);
    report.push(part_4::check_tank_base_shear(v_tank, document.tank_v_rd_kn));

    let bearing_red = part_5::bearing_reduction_factor(a_g);
    let p_rd = document.foundation_p_rd_kpa * bearing_red;
    let p_ed = part_5::seismic_bearing_pressure_kpa(v_b, document.foundation_area_m2);
    report.push(part_5::check_foundation_bearing(p_ed, p_rd));
    report.push(part_5::check_foundation_sliding(document.foundation_h_ed_kn, document.foundation_h_rd_kn));
    let _ = part_5::radiation_damping(part_5::stiffness_ratio(document.k_foundation, document.k_soil));

    let k_h = part_5::horizontal_seismic_coefficient(a_g, s, document.wall_r);
    let k_ae = part_5::mononobe_okabe_k_ae(document.wall_phi_deg, k_h);
    let h_ed_wall = part_5::retaining_wall_thrust_kn_m(document.wall_soil_gamma_kn_m3, document.wall_height_m, k_ae);
    report.push(part_5::check_retaining_wall_sliding(h_ed_wall, document.wall_h_rd_kn));

    let q_tower = part_6::tower_behaviour_factor(document.tower_q_nominal, document.tower_is_chimney);
    let s_d_tower = part_1::design_spectrum_sd(s_e, gamma_i, q_tower);
    let gamma_modal = part_6::cantilever_modal_participation_factor();
    let _v_tower = part_6::tower_base_shear_kn(gamma_modal, s_d_tower, document.tower_mass_t);
    report.push(part_6::check_tower_overturning(document.tower_m_ed_knm, document.tower_m_rd_knm));

    report
}

fn parse_seismic_zone(value: u8) -> na_de::SeismicZone {
    match value {
        0 => na_de::SeismicZone::Zone0,
        1 => na_de::SeismicZone::Zone1,
        3 => na_de::SeismicZone::Zone3,
        _ => na_de::SeismicZone::Zone2,
    }
}

fn parse_ground_type(value: &str) -> na_de::GroundType {
    match value.to_ascii_lowercase().as_str() {
        "a" => na_de::GroundType::A,
        "c" => na_de::GroundType::C,
        "d" => na_de::GroundType::D,
        "e" => na_de::GroundType::E,
        _ => na_de::GroundType::B,
    }
}

fn parse_importance(value: &str) -> part_1::ImportanceClass {
    match value.to_ascii_lowercase().as_str() {
        "cc1" => part_1::ImportanceClass::Cc1,
        "cc3" => part_1::ImportanceClass::Cc3,
        "cc4" => part_1::ImportanceClass::Cc4,
        _ => part_1::ImportanceClass::Cc2,
    }
}

fn parse_structural_system(value: &str) -> part_1::StructuralSystem {
    match value.to_ascii_lowercase().as_str() {
        "moment_frame_dcm" => part_1::StructuralSystem::MomentFrameDcm,
        "moment_frame_dcl" => part_1::StructuralSystem::MomentFrameDcl,
        "shear_wall" => part_1::StructuralSystem::ShearWall,
        "braced_frame" => part_1::StructuralSystem::BracedFrame,
        "inverted_pendulum" => part_1::StructuralSystem::InvertedPendulum,
        "dual_system" => part_1::StructuralSystem::DualSystem,
        _ => part_1::StructuralSystem::MomentFrameDch,
    }
}

fn parse_annex(value: &str) -> AnnexChoice {
    match value.to_ascii_lowercase().as_str() {
        "en" => AnnexChoice::En,
        _ => AnnexChoice::De,
    }
}

fn parse_en_ground_type(value: &str) -> part_1::EnGroundType {
    match value.to_ascii_lowercase().as_str() {
        "a" => part_1::EnGroundType::A,
        "c" => part_1::EnGroundType::C,
        "d" => part_1::EnGroundType::D,
        "e" => part_1::EnGroundType::E,
        _ => part_1::EnGroundType::B,
    }
}

fn parse_spectrum_type(value: &str) -> part_1::SpectrumType {
    match value.to_ascii_lowercase().as_str() {
        "type2" => part_1::SpectrumType::Type2,
        _ => part_1::SpectrumType::Type1,
    }
}

fn parse_knowledge_level(value: &str) -> part_3::KnowledgeLevel {
    match value.to_ascii_lowercase().as_str() {
        "kl1" => part_3::KnowledgeLevel::Kl1,
        "kl3" => part_3::KnowledgeLevel::Kl3,
        _ => part_3::KnowledgeLevel::Kl2,
    }
}

fn parse_retrofit_limit_state(value: &str) -> part_3::RetrofitLimitState {
    match value.to_ascii_lowercase().as_str() {
        "damage_limitation" => part_3::RetrofitLimitState::DamageLimitation,
        "near_collapse" => part_3::RetrofitLimitState::NearCollapse,
        _ => part_3::RetrofitLimitState::SignificantDamage,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1998Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &En1998Snapshot) -> CheckReport {
    check_full_seismic(document)
}

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn full_seismic_e2e() {
        let report = check_full_seismic(&En1998Snapshot::default());
        assert_eq!(report.checks.len(), 12);
    }

    #[semio_framework_async_macros::async_test]
    fn full_seismic_en_annex_e2e() {
        let document = En1998Snapshot { annex: "en".into(), ..En1998Snapshot::default() };
        let report = check_full_seismic(&document);
        assert_eq!(report.checks.len(), 12);
    }
}
//#endregion 🧪️ComplianceReportTests
