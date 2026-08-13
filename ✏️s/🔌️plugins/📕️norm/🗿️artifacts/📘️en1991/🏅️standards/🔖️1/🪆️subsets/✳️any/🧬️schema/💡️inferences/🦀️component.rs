//! 💡️ En1991 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1991::En1991Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1991Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1991 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991.inference")]
pub struct En1991Inference {
    #[derived]
    pub outline: En1991Outline,
}

impl protocol::Inference<En1991Snapshot> for En1991Inference {
    fn infer(snapshot: &En1991Snapshot) -> Self {
        Self { outline: En1991Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1991Snapshot> for En1991Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1991.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1991.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1991::standards::v1::subsets::any::schema::En1991Builder {
    type Snapshot = En1991Snapshot;
    type Inference = En1991Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1991.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1991_artifact_schema_descriptor`'s registration.
pub fn en1991_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1991.inference",
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
    fn inference_determinism_law() {
        let snapshot = En1991Snapshot::default();
        assert_eq!(En1991Inference::infer(&snapshot), En1991Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(En1991Inference::infer(&En1991Snapshot::default()), En1991Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
/// 📋️ Full EN 1991 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1991Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, ImposedCategory, NationalAnnex, Quantity};
use crate::artifacts::en1990::standards::v1::subsets::any::schema::{NaDe, NaEn};
use crate::artifacts::en1991::standards::v1::subsets::any::schema::{part_1_1, part_1_2, part_1_3, part_1_4, part_1_5, part_1_6, part_1_7, part_2, part_3, part_4};

/// 📋️ Aggregate action checks for a typical floor bay.
pub fn check_floor_actions(area_m2: f64, category: ImposedCategory, wind_zone_vb: f64, snow_zone: u8, use_de_na: bool) -> CheckReport {
    let annex: &dyn NationalAnnex = if use_de_na { &NaDe } else { &NaEn };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(area_m2, category, annex));
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, wind_zone_vb, c_e);
    report.push(part_1_4::check_wind(part_1_4::wind_pressure(q_p, 0.8, 0.2), 1.5, annex));
    let s = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(snow_zone), 0.8);
    report.push(part_1_3::check_snow(s, 1.2, annex));
    report
}

/// 📋️ Full EN 1991 action checks across parts 1-1 through 1-7 and parts 2–4.
pub fn check_full_actions(document: &En1991Snapshot) -> CheckReport {
    let annex: &dyn NationalAnnex = if document.annex == AnnexChoice::De { &NaDe } else { &NaEn };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(document.area_m2, document.category, annex));
    report.push(part_1_1::check_self_weight(&document.self_weight_material, document.self_weight_thickness_m, document.assumed_g_k_kn_m2, document.annex));
    report.push(part_1_2::check_fire_action(document.fire_curve, document.fire_resistance_min, document.fire_member_capacity_c, document.annex));
    let s_k = part_1_3::design_ground_snow_load(document.annex, document.snow_zone, document.snow_altitude_m, document.en_s_k_kn_m2);
    let s = part_1_3::roof_snow_load(s_k, 0.8);
    report.push(part_1_3::check_snow(s, 1.2, annex));
    let v_b = part_1_4::design_basic_wind_velocity(document.annex, document.wind_zone, document.en_v_b_m_s);
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, v_b, c_e);
    let c_sc_d = part_1_4::structural_factor(document.c_s, document.c_d);
    let w_p = part_1_4::wind_pressure(q_p, 0.8, 0.2) * c_sc_d;
    report.push(part_1_4::check_wind(w_p, 1.5, annex));
    report.push(part_1_5::check_temperature_action(document.delta_t_k, 50.0));
    let q_const = part_1_6::construction_load_kn_m2(&document.construction_activity);
    report.push(part_1_6::check_construction_load(q_const, 5.0));
    let impact = part_1_7::impact_force_kn(document.accidental_mass_t, document.accidental_speed_km_h);
    report.push(CheckResult::from_utilization(ClauseId::new("EN 1991-1-7", "Annex B", "B.2"), Quantity::force_kn(impact), Quantity::force_kn(500.0), "accidental impact", annex.choice()));
    report.push(part_2::check_lm1_moment(document.annex, document.bridge_span_m, document.bridge_lane, document.bridge_lane_width_m, document.bridge_moment_resistance_knm));
    let wheel = part_3::design_vertical_wheel_load(&document.crane_class, &document.hoist_class, document.hoisting_speed_m_s);
    report.push(part_3::check_crane_load(wheel, wheel * 1.2));
    let silo_p = part_4::janssen_horizontal_pressure_kpa(document.silo_bulk_density_kn_m3, document.silo_hydraulic_radius_m, document.silo_mu, document.silo_k, document.silo_height_m);
    report.push(part_4::check_silo_pressure(silo_p, 100.0));
    report
}

/// 📋️ `En1991Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &En1991Snapshot) -> CheckReport {
    check_full_actions(document)
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[test]
    fn full_actions_de_na_numeric() {
        let doc = En1991Snapshot::default();
        let annex = NaDe;
        let report = check_full_actions(&doc);
        assert_eq!(report.checks.len(), 11);
        let imposed_q = part_1_1::imposed_load_kn_m2(ImposedCategory::B) * doc.area_m2 * annex.psi_0("office");
        assert!((report.checks[0].computed.value / 1000.0 - imposed_q).abs() < 1e-6);
        assert!((report.checks[1].computed.value / 1000.0 - 5.0).abs() < 1e-6);
        let theta_g = part_1_2::standard_gas_temperature_c(30.0);
        assert!((theta_g - 841.79588).abs() < 1e-4);
        assert!((report.checks[2].computed.value - theta_g).abs() < 1e-6);
        let snow = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(2), 0.8);
        assert!((report.checks[3].computed.value - snow * 1000.0).abs() < 1e-6);
        let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
        let q_p = part_1_4::peak_velocity_pressure(1.25, 25.0, c_e);
        let w_p = part_1_4::wind_pressure(q_p, 0.8, 0.2) * part_1_4::structural_factor(1.0, 1.0);
        assert!((report.checks[4].computed.value - w_p * 1000.0).abs() < 1e-3);
        assert!((report.checks[5].computed.value - doc.delta_t_k).abs() < 1e-6);
        assert!((report.checks[6].computed.value / 1000.0 - 1.0).abs() < 1e-6);
        let impact = part_1_7::impact_force_kn(30.0, 80.0);
        assert!((report.checks[7].computed.value / 1000.0 - impact).abs() < 1e-6);
        assert!((impact - 7.407407407407407).abs() < 1e-6);
        assert!((report.checks[8].computed.value / 1000.0 - 2700.0).abs() < 1e-6);
        let silo_p = part_4::janssen_horizontal_pressure_kpa(8.0, 1.5, 0.4, 0.4, 12.0);
        assert!((silo_p - 54.147).abs() < 1e-2);
        assert!((report.checks[10].computed.value - silo_p * 1000.0).abs() < 1e-6);
        assert!(report.all_pass());
    }

    #[test]
    fn evaluate_reaches_every_part_module() {
        let report = evaluate(&En1991Snapshot::default());
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-1")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-2")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-3")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-4")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-5")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-6")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-7")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-2")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-3")));
        assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-4")));
    }
}
//#endregion 🧪️ComplianceReportTests
