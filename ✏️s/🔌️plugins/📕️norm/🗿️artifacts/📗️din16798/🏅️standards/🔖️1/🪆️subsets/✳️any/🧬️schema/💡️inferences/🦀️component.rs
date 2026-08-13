//! 💡️ Din16798 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::din16798::Din16798Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Din16798Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din16798 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din16798.inference")]
pub struct Din16798Inference {
    #[derived]
    pub outline: Din16798Outline,
}

impl protocol::Inference<Din16798Snapshot> for Din16798Inference {
    fn infer(snapshot: &Din16798Snapshot) -> Self {
        Self { outline: Din16798Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Din16798Snapshot> for Din16798Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.din16798.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.din16798.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::din16798::standards::v1::subsets::any::schema::Din16798Builder {
    type Snapshot = Din16798Snapshot;
    type Inference = Din16798Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din16798.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `din16798_artifact_schema_descriptor`'s registration.
pub fn din16798_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.din16798.inference",
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
        let snapshot = Din16798Snapshot::default();
        assert_eq!(Din16798Inference::infer(&snapshot), Din16798Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Din16798Inference::infer(&Din16798Snapshot::default()), Din16798Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
/// 📋️ Full DIN EN 16798 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `Din16798Snapshot -> CheckReport` projection; everything it composes
/// (`part_N`/`annex_params`) is a pure helper living in the parent `🧬️schema`.
use crate::document::CheckReport;
use crate::artifacts::din16798::standards::v1::subsets::any::schema::{part_1, part_3, part_5_1, part_5_2, part_7, part_9, part_13, part_15, part_17, annex_params};

/// 📋️ End-to-end residential indoor environment check.
pub fn check_residential_environment(floor_area_m2: f64, occupants: u32, ventilation_m3_h: f64, t_op_c: f64, l_aeq_db: f64) -> CheckReport {
    let mut report = CheckReport::default();
    report.push(part_1::check_operative_temperature(crate::document::OccupancyType::Residential, t_op_c));
    report.push(part_3::check_residential_ventilation(floor_area_m2, occupants, ventilation_m3_h));
    report.push(part_1::check_acoustic_category(part_1::ComfortCategory::II, l_aeq_db));
    report
}

fn parse_occupancy(occupancy: &str) -> crate::document::OccupancyType {
    match occupancy.to_ascii_lowercase().as_str() {
        "office" => crate::document::OccupancyType::Office,
        "meeting" => crate::document::OccupancyType::Meeting,
        "classroom" => crate::document::OccupancyType::Classroom,
        "retail" => crate::document::OccupancyType::Retail,
        "kitchen" => crate::document::OccupancyType::Kitchen,
        "corridor" => crate::document::OccupancyType::Corridor,
        _ => crate::document::OccupancyType::Residential,
    }
}

fn parse_comfort_category(category: &str) -> part_1::ComfortCategory {
    match category.to_ascii_uppercase().as_str() {
        "I" => part_1::ComfortCategory::I,
        "III" => part_1::ComfortCategory::III,
        _ => part_1::ComfortCategory::II,
    }
}

fn parse_ida_class(class: &str) -> part_3::IdaClass {
    match class {
        "1" => part_3::IdaClass::Ida1,
        "3" => part_3::IdaClass::Ida3,
        "4" => part_3::IdaClass::Ida4,
        _ => part_3::IdaClass::Ida2,
    }
}

fn parse_duct_class(class: &str) -> part_17::DuctLeakageClass {
    match class.to_ascii_uppercase().as_str() {
        "A" => part_17::DuctLeakageClass::A,
        "C" => part_17::DuctLeakageClass::C,
        "D" => part_17::DuctLeakageClass::D,
        _ => part_17::DuctLeakageClass::B,
    }
}

fn parse_chiller_type(chiller_type: &str) -> part_13::ChillerType {
    match chiller_type.to_ascii_lowercase().as_str() {
        "water_cooled" => part_13::ChillerType::WaterCooled,
        "absorption" => part_13::ChillerType::Absorption,
        _ => part_13::ChillerType::AirCooled,
    }
}

/// 📋️ Full EN 16798 normative parts (1, 3, 5-1, 5-2, 7, 9, 13, 15, 17) plus DE-NA divergent checks.
pub fn check_full_environment(document: &Din16798Snapshot) -> CheckReport {
    let occupancy = parse_occupancy(&document.occupancy);
    let category = parse_comfort_category(&document.comfort_category);
    let ida_class = parse_ida_class(&document.ida_class);
    let sfp_required_class = part_3::sfp_class_from_number(document.sfp_required_class);
    let duct_class = parse_duct_class(&document.duct_class);
    let chiller_type = parse_chiller_type(&document.chiller_type);
    let annex = annex_params::AnnexParams::for_choice(document.annex);

    let mut report = CheckReport::default();

    report.push(part_1::check_operative_temperature(occupancy, document.t_op_c));
    report.push(part_1::check_pmv_comfort(document.t_op_c, document.rh_percent, document.air_speed_m_s));
    report.push(part_1::check_adaptive_comfort(document.theta_rm_c, document.t_op_c, category));
    report.push(part_1::check_co2_level(occupancy, document.co2_ppm, &annex));
    report.push(part_1::check_daylight_factor(category, document.df_percent));
    report.push(part_1::check_acoustic_category(category, document.l_aeq_db));

    report.push(part_3::check_ventilation_rate(occupancy, document.persons, ida_class, document.ventilation_m3_h));
    report.push(part_3::check_dwelling_ventilation(document.floor_area_m2, document.bedrooms, document.dwelling_ventilation_m3_h));
    report.push(part_3::check_residential_ventilation(document.floor_area_m2, document.occupants, document.residential_ventilation_m3_h));
    report.push(part_3::check_design_sfp(document.sfp_w_m3_s, sfp_required_class));
    report.push(part_3::check_heat_recovery_efficiency(document.heat_recovery_eta, document.heat_recovery_eta_min));
    report.push(part_3::check_inspection_due(&document.system_type, document.years_since_inspection));
    report.push(part_3::check_humidification_capacity(document.humidification_required_kg_h, document.humidification_provided_kg_h));

    report.push(part_5_1::check_building_fan_energy(document.sfp_w_m3_s, document.fan_q_v_m3_s, document.fan_t_run_h, document.fan_energy_reference_kwh));
    report.push(part_5_1::check_night_setback(occupancy, document.night_setback_k));

    report.push(part_5_2::check_heat_recovery_savings(document.heat_recovery_eta, document.hr_m_dot_kg_s, document.hr_cp_j_kgk, document.hr_delta_t_c, document.hr_t_h, document.hr_savings_reference_kwh));

    report.push(part_7::check_infiltration(document.n50_h_inv, document.volume_m3, document.infiltration_allowance_m3_h));
    report.push(part_7::check_cellar_ventilation(document.cellar_area_m2, document.cellar_ventilation_m3_h));

    report.push(part_9::check_cooling_energy_need(
        document.h_tr_w_k,
        document.h_ve_w_k,
        document.theta_e_c,
        document.theta_set_c,
        document.cooling_delta_t_h,
        document.cooling_gains_kwh,
        document.cooling_utilization_factor,
        document.cooling_reference_kwh,
    ));

    report.push(part_13::check_chiller_eer(chiller_type, document.eer_actual));
    report.push(part_13::check_generation_energy(document.q_c_kwh, document.eer_actual, document.generation_reference_kwh));
    report.push(part_13::check_supply_air_temperature(document.data_center_supply_c));

    report.push(part_15::check_storage_losses(document.h_st_w_k, document.theta_st_c, document.theta_amb_c, document.storage_t_h, document.storage_allowance_kwh));
    report.push(part_15::check_dhw_temperature(document.dhw_delivery_c));

    report.push(part_17::check_duct_leakage(duct_class, document.duct_test_pressure_pa, document.duct_leakage_m3_s_m2));

    report
}

/// 📋️ `Din16798Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &Din16798Snapshot) -> CheckReport {
    check_full_environment(document)
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[test]
    fn residential_environment_e2e_with_acoustic() {
        let report = check_residential_environment(85.0, 3, 40.0, 21.0, 24.0);
        assert!(report.all_pass());
        assert_eq!(report.checks.len(), 3);
    }

    #[test]
    fn full_environment_evaluate_covers_all_nine_parts() {
        let document = Din16798Snapshot::default();
        let report = evaluate(&document);
        assert_eq!(report.checks.len(), 25, "checks: {:?}", report.checks);
        assert!(report.all_pass(), "checks: {:?}", report.checks);
        assert_eq!(document.annex, crate::document::AnnexChoice::De);
        let pmv = part_1::pmv_iso7730(document.t_op_c, document.rh_percent, document.air_speed_m_s);
        assert!(pmv.abs() < 0.5);
    }
}
//#endregion 🧪️ComplianceReportTests
