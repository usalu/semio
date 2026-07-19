//! 🌬️ DIN EN 16798 indoor environmental input parameters and ventilation / HVAC energy.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, OccupancyType, Quantity};
use serde::{Deserialize, Serialize};

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    /// 🌡️ Design operative temperature band [°C] per occupancy type (EN 16798-1).
    pub fn operative_temperature_band(occupancy: OccupancyType) -> (f64, f64) {
        match occupancy {
            OccupancyType::Residential => (20.0, 24.0),
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => (20.0, 26.0),
            OccupancyType::Retail | OccupancyType::Corridor => (18.0, 26.0),
            OccupancyType::Kitchen => (18.0, 28.0),
        }
    }

    /// 🧮 Simplified PMV index from operative temperature, RH [%], and air speed [m/s].
    pub fn pmv_simplified(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64) -> f64 {
        let t_ref = 25.0;
        let v = air_speed_m_s.max(0.0);
        0.28 * (t_op_c - t_ref) + 0.001 * (rh_percent - 50.0) * (t_op_c - t_ref) - 0.15 * (v - 0.1)
    }

    /// ✅ Category II comfort band: PMV within ±0.5 (EN 16798-1).
    pub fn check_pmv_comfort(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64) -> CheckResult {
        let pmv = pmv_simplified(t_op_c, rh_percent, air_speed_m_s);
        let limit = 0.5;
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-1", "§7", "7.2.2"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, pmv.abs()),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "simplified PMV comfort",
            AnnexChoice::De,
        )
    }

    /// ✅ Check operative temperature within band (EN 16798-1).
    pub fn check_operative_temperature(occupancy: OccupancyType, t_op_c: f64) -> CheckResult {
        let (t_min, t_max) = operative_temperature_band(occupancy);
        let within = t_op_c >= t_min && t_op_c <= t_max;
        let status = if within { norm_core::CheckStatus::Pass } else { norm_core::CheckStatus::Fail };
        CheckResult {
            clause: ClauseId::new("EN 16798-1", "§7", "7.2.2"),
            status,
            computed: Quantity::new(norm_core::QuantityKind::Temperature, t_op_c),
            limit: Quantity::new(norm_core::QuantityKind::Temperature, t_max),
            utilization: if within { 0.0 } else { 1.1 },
            message: "operative temperature band".into(),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖Part1

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🫁 Design CO₂ limit [ppm] for non-residential spaces (EN 16798-2).
    pub fn design_co2_limit_ppm(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Residential => 1500.0,
            _ => 1000.0,
        }
    }

    /// ✅ Check indoor CO₂ concentration.
    pub fn check_co2_level(occupancy: OccupancyType, co2_ppm: f64) -> CheckResult {
        let limit = design_co2_limit_ppm(occupancy);
        CheckResult::from_utilization(ClauseId::new("EN 16798-2", "§6", "6.2"), Quantity::new(norm_core::QuantityKind::Dimensionless, co2_ppm), Quantity::new(norm_core::QuantityKind::Dimensionless, limit), "indoor CO₂ concentration", AnnexChoice::De)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 📊 Specific outdoor airflow per person [m³/(h·person)] (EN 16798-3 Table 1).
    pub fn outdoor_air_per_person(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => 36.0,
            OccupancyType::Retail => 20.0,
            OccupancyType::Kitchen => 60.0,
            OccupancyType::Residential => 30.0,
            OccupancyType::Corridor => 10.0,
        }
    }

    /// ✅ Check ventilation rate for non-residential spaces.
    pub fn check_ventilation_rate(occupancy: OccupancyType, persons: u32, supplied_m3_h: f64) -> CheckResult {
        let required = outdoor_air_per_person(occupancy) * persons as f64;
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "Table 1", "q"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "outdoor air supply",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    /// 🏠 Dwelling whole-building ventilation rate [m³/h] (EN 16798-4).
    pub fn dwelling_ventilation_rate(floor_area_m2: f64, bedrooms: u32) -> f64 {
        let by_area = 0.5 * floor_area_m2;
        let by_bedroom = 21.0 * bedrooms.max(1) as f64;
        by_area.max(by_bedroom)
    }

    /// ✅ Check dwelling ventilation adequacy.
    pub fn check_dwelling_ventilation(floor_area_m2: f64, bedrooms: u32, supplied_m3_h: f64) -> CheckResult {
        let required = dwelling_ventilation_rate(floor_area_m2, bedrooms);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-4", "§7", "7.1"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "dwelling ventilation rate",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part4

// #region 🔖Part5
pub mod part_5 {
    use super::*;

    /// ⚡ Seasonal system efficiency for heating (EN 16798-5 simplified).
    pub fn check_heating_efficiency(eta_delivered: f64, eta_min: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-5", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eta_delivered),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eta_min),
            "heating system seasonal efficiency",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part5

// #region 🔖Part6
pub mod part_6 {
    use super::*;

    /// 🏚️ Cellar ventilation rate [m³/h] per floor area (EN 16798-6).
    pub fn cellar_ventilation_rate(cellar_area_m2: f64) -> f64 {
        0.3 * cellar_area_m2
    }

    /// ✅ Check cellar ventilation adequacy.
    pub fn check_cellar_ventilation(cellar_area_m2: f64, supplied_m3_h: f64) -> CheckResult {
        let required = cellar_ventilation_rate(cellar_area_m2);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-6", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "cellar ventilation rate",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part6

// #region 🔖Part7
pub mod part_7 {
    use super::*;

    /// 🏠 Residential whole-building ventilation rate [m³/h] (EN 16798-7).
    pub fn residential_ventilation_rate(floor_area_m2: f64, occupants: u32) -> f64 {
        let by_area = 0.4 * floor_area_m2;
        let by_person = 30.0 * occupants.max(1) as f64;
        by_area.max(by_person)
    }

    /// ✅ Check residential ventilation adequacy.
    pub fn check_residential_ventilation(floor_area_m2: f64, occupants: u32, supplied_m3_h: f64) -> CheckResult {
        let required = residential_ventilation_rate(floor_area_m2, occupants);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-7", "§7", "7.1"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "residential ventilation rate",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part7

// #region 🔖Part8
pub mod part_8 {
    use super::*;

    /// 🌀 Duct leakage class limit [m³/(s·m²) at 400 Pa] (EN 16798-8).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DuctLeakageClass {
        A,
        B,
        C,
    }

    pub fn leakage_limit_m3_s_m2(class: DuctLeakageClass) -> f64 {
        match class {
            DuctLeakageClass::A => 0.027,
            DuctLeakageClass::B => 0.009,
            DuctLeakageClass::C => 0.003,
        }
    }

    /// ✅ Check ductwork leakage at test pressure.
    pub fn check_duct_leakage(class: DuctLeakageClass, measured_m3_s_m2: f64) -> CheckResult {
        let limit = leakage_limit_m3_s_m2(class);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-8", "§7", "7.3"),
            Quantity::new(norm_core::QuantityKind::AirPermeability, measured_m3_s_m2),
            Quantity::new(norm_core::QuantityKind::AirPermeability, limit),
            "ductwork leakage class",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part8

// #region 🔖Part9
pub mod part_9 {
    use super::*;

    /// 💧 Humidification capacity check (EN 16798-9).
    pub fn check_humidification_capacity(required_kg_h: f64, provided_kg_h: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 16798-9", "§5", "5.2"), Quantity::new(norm_core::QuantityKind::Mass, provided_kg_h), Quantity::new(norm_core::QuantityKind::Mass, required_kg_h), "humidification capacity", AnnexChoice::De)
    }
}
// #endregion 🔖Part9

// #region 🔖Part10
pub mod part_10 {
    use super::*;

    /// 🔍 Inspection interval [years] for ventilation systems (EN 16798-10).
    pub fn inspection_interval_years(system_type: &str) -> u32 {
        match system_type {
            "central_mech" => 3,
            "decentral" => 5,
            _ => 3,
        }
    }

    /// ✅ Check whether last inspection is within required interval.
    pub fn check_inspection_due(system_type: &str, years_since_inspection: u32) -> CheckResult {
        let interval = inspection_interval_years(system_type);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-10", "§5", "5.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, years_since_inspection as f64),
            Quantity::new(norm_core::QuantityKind::Dimensionless, interval as f64),
            "ventilation inspection interval",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part10

// #region 🔖Part11
pub mod part_11 {
    use super::*;

    /// ⚡ Specific fan power limit [W/(m³/s)] for air-handling units (EN 16798-11).
    pub fn specific_fan_power_limit_w_m3_s(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Residential => 1.8,
            OccupancyType::Office | OccupancyType::Meeting => 1.5,
            OccupancyType::Classroom | OccupancyType::Retail => 1.6,
            OccupancyType::Kitchen => 2.0,
            OccupancyType::Corridor => 2.2,
        }
    }

    /// ✅ Check specific fan power of ventilation unit.
    pub fn check_specific_fan_power(occupancy: OccupancyType, sfp_w_m3_s: f64) -> CheckResult {
        let limit = specific_fan_power_limit_w_m3_s(occupancy);
        CheckResult::from_utilization(ClauseId::new("EN 16798-11", "§6", "6.2"), Quantity::new(norm_core::QuantityKind::Power, sfp_w_m3_s), Quantity::new(norm_core::QuantityKind::Power, limit), "specific fan power", AnnexChoice::De)
    }
}
// #endregion 🔖Part11

// #region 🔖Part12
pub mod part_12 {
    use super::*;

    /// 🎛️ Minimum night setback [K] for heating controls (EN 16798-12).
    pub fn night_setback_k(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Residential => 3.0,
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => 4.0,
            OccupancyType::Retail | OccupancyType::Kitchen | OccupancyType::Corridor => 2.0,
        }
    }

    /// ✅ Check control setback is configured.
    pub fn check_night_setback(occupancy: OccupancyType, configured_k: f64) -> CheckResult {
        let required = night_setback_k(occupancy);
        CheckResult::from_minimum(ClauseId::new("EN 16798-12", "§5", "5.3"), Quantity::new(norm_core::QuantityKind::Temperature, configured_k), Quantity::new(norm_core::QuantityKind::Temperature, required), "night setback depth", AnnexChoice::De)
    }
}
// #endregion 🔖Part12

// #region 🔖Part13
pub mod part_13 {
    use super::*;

    /// 🔊 Residential ventilation acoustic limit L_Aeq [dB] (EN 16798-13).
    pub fn acoustic_limit_db() -> f64 {
        30.0
    }

    /// ✅ Check ventilation unit sound level.
    pub fn check_acoustic_level(l_aeq_db: f64) -> CheckResult {
        let limit = acoustic_limit_db();
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-13", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, l_aeq_db),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "ventilation unit sound level",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part13

// #region 🔖Part14
pub mod part_14 {
    use super::*;

    /// 🚿 DHW delivery temperature band [°C] (EN 16798-14).
    pub fn dhw_delivery_temperature_band_c() -> (f64, f64) {
        (55.0, 60.0)
    }

    /// ✅ Check DHW delivery temperature.
    pub fn check_dhw_temperature(t_delivery_c: f64) -> CheckResult {
        let (t_min, t_max) = dhw_delivery_temperature_band_c();
        let within = t_delivery_c >= t_min && t_delivery_c <= t_max;
        let status = if within { norm_core::CheckStatus::Pass } else { norm_core::CheckStatus::Fail };
        CheckResult {
            clause: ClauseId::new("EN 16798-14", "§6", "6.1"),
            status,
            computed: Quantity::new(norm_core::QuantityKind::Temperature, t_delivery_c),
            limit: Quantity::new(norm_core::QuantityKind::Temperature, t_max),
            utilization: if within { 0.0 } else { 1.1 },
            message: "DHW delivery temperature".into(),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖Part14

// #region 🔖Part15
pub mod part_15 {
    use super::*;

    /// 🍳 Kitchen extract airflow [m³/h] (EN 16798-15).
    pub fn kitchen_extract_rate(stove_type: &str) -> f64 {
        match stove_type {
            "domestic" => 140.0,
            "commercial_light" => 400.0,
            _ => 140.0,
        }
    }

    pub fn check_kitchen_extract(stove_type: &str, supplied_m3_h: f64) -> CheckResult {
        let required = kitchen_extract_rate(stove_type);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-15", "§7", "7.1"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "kitchen extract airflow",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part15

// #region 🔖Part16
pub mod part_16 {
    use super::*;

    /// 🖥️ Data center supply air temperature band [°C] (EN 16798-16).
    pub fn supply_air_temperature_band_c() -> (f64, f64) {
        (18.0, 27.0)
    }

    /// ✅ Check data center supply air temperature.
    pub fn check_supply_air_temperature(t_supply_c: f64) -> CheckResult {
        let (t_min, t_max) = supply_air_temperature_band_c();
        let within = t_supply_c >= t_min && t_supply_c <= t_max;
        let status = if within { norm_core::CheckStatus::Pass } else { norm_core::CheckStatus::Fail };
        CheckResult {
            clause: ClauseId::new("EN 16798-16", "§7", "7.2"),
            status,
            computed: Quantity::new(norm_core::QuantityKind::Temperature, t_supply_c),
            limit: Quantity::new(norm_core::QuantityKind::Temperature, t_max),
            utilization: if within { 0.0 } else { 1.1 },
            message: "data center supply air temperature".into(),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖Part16

// #region 🔖Part17
pub mod part_17 {
    use super::*;

    /// 🍽️ Commercial kitchen capture velocity [m/s] (EN 16798-17).
    pub fn check_capture_velocity(v_m_s: f64) -> CheckResult {
        let limit = 0.5;
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-17", "§8", "8.2"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, v_m_s),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "hood capture face velocity",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part17

// #region 🔖NaDe
pub mod na_de {
    use super::*;

    /// 🇩🇪 German national CO₂ design limit [ppm] (DIN EN 16798 NA-DE).
    pub fn co2_design_limit_ppm(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Residential => 1200.0,
            OccupancyType::Classroom => 800.0,
            _ => 900.0,
        }
    }

    /// 🇩🇪 German national acoustic limit L_Aeq [dB] for residential ventilation.
    pub fn residential_acoustic_limit_db() -> f64 {
        25.0
    }

    /// ✅ Check CO₂ against German national annex limits.
    pub fn check_co2_level(occupancy: OccupancyType, co2_ppm: f64) -> CheckResult {
        let limit = co2_design_limit_ppm(occupancy);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-2", "NA-DE", "6.2"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, co2_ppm),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "German national CO₂ limit",
            AnnexChoice::De,
        )
    }

    /// ✅ Check acoustic level against German national annex.
    pub fn check_acoustic_level(l_aeq_db: f64) -> CheckResult {
        let limit = residential_acoustic_limit_db();
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-13", "NA-DE", "6.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, l_aeq_db),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "German national ventilation sound level",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖NaDe

/// 📋 End-to-end residential indoor environment check.
pub fn check_residential_environment(floor_area_m2: f64, occupants: u32, ventilation_m3_h: f64, t_op_c: f64, l_aeq_db: f64) -> CheckReport {
    let mut report = CheckReport::default();
    report.push(part_1::check_operative_temperature(OccupancyType::Residential, t_op_c));
    report.push(part_7::check_residential_ventilation(floor_area_m2, occupants, ventilation_m3_h));
    report.push(na_de::check_acoustic_level(l_aeq_db));
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub floor_area_m2: f64,
    pub occupants: u32,
    pub ventilation_m3_h: f64,
    pub t_op_c: f64,
    pub l_aeq_db: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self { floor_area_m2: 90.0, occupants: 3, ventilation_m3_h: 120.0, t_op_c: 21.0, l_aeq_db: 30.0 }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<DinEn16798Family>;

pub fn evaluate(document: &Document) -> CheckReport {
    check_residential_environment(document.floor_area_m2, document.occupants, document.ventilation_m3_h, document.t_op_c, document.l_aeq_db)
}

pub struct DinEn16798Family;

impl NormFamily for DinEn16798Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::DinEn16798
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖Session

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pmv_simplified_neutral_at_reference() {
        let pmv = part_1::pmv_simplified(25.0, 50.0, 0.1);
        assert!(pmv.abs() < 0.01);
    }

    #[test]
    fn pmv_comfort_passes_for_office_conditions() {
        let check = part_1::check_pmv_comfort(24.0, 50.0, 0.1);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
        assert!(check.utilization < 1.0);
    }

    #[test]
    fn co2_limit_office_1000_ppm() {
        assert_eq!(part_2::design_co2_limit_ppm(OccupancyType::Office), 1000.0);
        let check = part_2::check_co2_level(OccupancyType::Office, 950.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn ventilation_rates_per_room_type() {
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Office), 36.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Meeting), 36.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Classroom), 36.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Retail), 20.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Kitchen), 60.0);
        let office = part_3::check_ventilation_rate(OccupancyType::Office, 10, 360.0);
        assert_eq!(office.status, norm_core::CheckStatus::Pass);
        assert!((office.limit.value - 360.0).abs() < 1e-9);
    }

    #[test]
    fn residential_ventilation_rate_100m2_4_occupants() {
        let rate = part_7::residential_ventilation_rate(100.0, 4);
        assert!((rate - 120.0).abs() < 1e-9);
    }

    #[test]
    fn dwelling_ventilation_85m2_3_bedrooms() {
        let rate = part_4::dwelling_ventilation_rate(85.0, 3);
        assert!((rate - 63.0).abs() < 1e-9);
    }

    #[test]
    fn cellar_ventilation_50m2() {
        let rate = part_6::cellar_ventilation_rate(50.0);
        assert!((rate - 15.0).abs() < 1e-9);
    }

    #[test]
    fn duct_leakage_class_b_limit() {
        assert!((part_8::leakage_limit_m3_s_m2(part_8::DuctLeakageClass::B) - 0.009).abs() < 1e-9);
        let check = part_8::check_duct_leakage(part_8::DuctLeakageClass::B, 0.008);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn data_center_supply_air_22c_passes() {
        let check = part_16::check_supply_air_temperature(22.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn na_de_classroom_co2_800_ppm() {
        assert_eq!(na_de::co2_design_limit_ppm(OccupancyType::Classroom), 800.0);
        let check = na_de::check_co2_level(OccupancyType::Classroom, 750.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn residential_environment_e2e_with_acoustic() {
        let report = check_residential_environment(85.0, 3, 40.0, 21.0, 24.0);
        assert!(report.all_pass());
        assert_eq!(report.checks.len(), 3);
    }
}
