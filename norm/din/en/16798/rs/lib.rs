//! 🌬️ DIN EN 16798 indoor environmental input parameters and ventilation / HVAC energy.
//!
//! 🗺️ Only the normative parts are modelled as modules: `part_1`, `part_3`, `part_5_1`,
//! `part_5_2`, `part_7`, `part_9`, `part_13`, `part_15`, `part_17`. The informative
//! Technical Reports (the even-numbered 2, 4, 6, 8, 10, 12, 14, 16) are not separate
//! normative parts, so their real guidance was folded into the adjacent normative
//! sibling instead of being kept as dedicated thin modules (e.g. duct-leakage classes
//! now live in `part_17`; dwelling/residential ventilation rates, SFP occupancy limits
//! and inspection-interval guidance now live in `part_3`; DHW delivery temperature now
//! lives alongside storage losses in `part_15`; night-setback control now lives
//! alongside building-level fan energy in `part_5_1`; data-center supply air and
//! acoustic/daylight IEQ criteria now live alongside cooling generation and comfort
//! category checks in `part_13`/`part_1` respectively).

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, OccupancyType, Quantity};
use serde::{Deserialize, Serialize};

// #region 🔖Part1
pub mod part_1 {
    use super::annex_params::AnnexParams;
    use super::*;

    /// 🏷️ EN 16798-1 IEQ comfort category (I strictest .. III loosest).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComfortCategory {
        I,
        II,
        III,
    }

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

    fn saturation_pressure_pa(t_c: f64) -> f64 {
        611.2 * (17.67 * t_c / (t_c + 243.5)).exp()
    }

    fn solve_clothing_temp_c(t_a: f64, t_r: f64, m: f64, w: f64, i_cl: f64, f_cl: f64, v: f64) -> f64 {
        let mut t_cl = t_a + (35.5 - t_a) / (3.5 * i_cl + 1.0);
        for _ in 0..50 {
            let h_c = if v < 0.1 { 2.38 * (t_cl - t_a).abs().powf(0.25) } else { 12.1 * v.sqrt() };
            let t_cl_k = t_cl + 273.15;
            let t_r_k = t_r + 273.15;
            let rad = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
            let conv = f_cl * h_c * (t_cl - t_a);
            let t_new = 35.7 - 0.028 * (m - w) - i_cl * (rad + conv);
            if (t_new - t_cl).abs() < 0.001 {
                return t_new;
            }
            t_cl = t_new;
        }
        t_cl
    }

    /// 😌 ISO 7730 PMV with default activity 1.2 met and clothing 0.5 clo.
    pub fn pmv_iso7730(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64) -> f64 {
        pmv_iso7730_with_activity(t_op_c, rh_percent, air_speed_m_s, 1.2, 0.5)
    }

    /// 😌 ISO 7730 PMV with explicit metabolic rate [met] and clothing [clo].
    pub fn pmv_iso7730_with_activity(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64, metabolic_rate_met: f64, clothing_clo: f64) -> f64 {
        let m = metabolic_rate_met * 58.15;
        let w = 0.0;
        let i_cl = 0.155 * clothing_clo;
        let f_cl = if i_cl <= 0.078 { 1.0 + 1.29 * i_cl } else { 1.05 + 0.645 * i_cl };
        let t_a = t_op_c;
        let t_r = t_op_c;
        let v = air_speed_m_s.max(0.0);
        let p_a = (rh_percent / 100.0).clamp(0.0, 1.0) * saturation_pressure_pa(t_a);
        let t_cl = solve_clothing_temp_c(t_a, t_r, m, w, i_cl, f_cl, v);
        let h_c = if v < 0.1 { 2.38 * (t_cl - t_a).abs().powf(0.25) } else { 12.1 * v.sqrt() };
        let t_cl_k = t_cl + 273.15;
        let t_r_k = t_r + 273.15;
        let e_r = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
        let e_c = f_cl * h_c * (t_cl - t_a);
        let e_sw = 3.05e-3 * (5733.0 - 6.99 * m - p_a).max(0.0);
        let e_diff = if m > 58.15 { 0.42 * (m - 58.15) } else { 0.0 };
        let e = e_sw + e_diff;
        let c_res = 1.7e-5 * m * (34.0 - t_a);
        let l = m - w - e - e_r - e_c - c_res;
        ((0.303 * (-0.035 * m).exp() + 0.028) * l).clamp(-3.0, 3.0)
    }

    /// 📊 PPD [%] from PMV per ISO 7730.
    pub fn ppd_from_pmv(pmv: f64) -> f64 {
        let pmv_c = pmv.clamp(-3.0, 3.0);
        100.0 - 95.0 * (-0.03353 * pmv_c.powi(4) - 0.2179 * pmv_c.powi(2)).exp()
    }

    /// ✅ Category II comfort band: PMV within ±0.5 (EN 16798-1).
    pub fn check_pmv_comfort(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64) -> CheckResult {
        let pmv = pmv_iso7730(t_op_c, rh_percent, air_speed_m_s);
        let limit = 0.5;
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-1", "§7", "7.2.2"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, pmv.abs()),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "ISO 7730 PMV comfort",
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

    /// 🌤️ Adaptive comfort operative temperature centre [°C]: θ_c,operation = 0.33·θ_rm + 18.8 (EN 16798-1 Annex A).
    pub fn adaptive_comfort_temperature_c(theta_rm_c: f64) -> f64 {
        0.33 * theta_rm_c + 18.8
    }

    /// 📏 Adaptive comfort acceptable band half-width [K] by category.
    pub fn adaptive_band_k(category: ComfortCategory) -> f64 {
        match category {
            ComfortCategory::I => 2.0,
            ComfortCategory::II => 3.0,
            ComfortCategory::III => 4.0,
        }
    }

    /// ✅ Check operative temperature against the adaptive comfort model (EN 16798-1 Annex A, free-running buildings).
    pub fn check_adaptive_comfort(theta_rm_c: f64, t_op_c: f64, category: ComfortCategory) -> CheckResult {
        let centre = adaptive_comfort_temperature_c(theta_rm_c);
        let band = adaptive_band_k(category);
        let deviation = (t_op_c - centre).abs();
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-1", "Annex A", "A.2"),
            Quantity::new(norm_core::QuantityKind::Temperature, deviation),
            Quantity::new(norm_core::QuantityKind::Temperature, band),
            "adaptive comfort deviation",
            AnnexChoice::En,
        )
    }

    /// 🫁 Design CO₂ limit [ppm] branching on the selected national annex (folded TR EN 16798-2).
    pub fn design_co2_limit_ppm(occupancy: OccupancyType, annex: &AnnexParams) -> f64 {
        match occupancy {
            OccupancyType::Residential => annex.co2_limit_residential_ppm,
            OccupancyType::Classroom => annex.co2_limit_classroom_ppm,
            _ => annex.co2_limit_other_ppm,
        }
    }

    /// ✅ Check indoor CO₂ concentration against the annex-specific category threshold.
    pub fn check_co2_level(occupancy: OccupancyType, co2_ppm: f64, annex: &AnnexParams) -> CheckResult {
        let limit = design_co2_limit_ppm(occupancy, annex);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-1", annex.choice.label(), "6.2"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, co2_ppm),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "indoor CO₂ concentration",
            annex.choice,
        )
    }

    /// ☀️ Minimum daylight factor [%] by IEQ category (folded TR EN 16798-10 daylight guidance).
    pub fn daylight_factor_min_percent(category: ComfortCategory) -> f64 {
        match category {
            ComfortCategory::I => 3.0,
            ComfortCategory::II => 2.0,
            ComfortCategory::III => 1.5,
        }
    }

    /// ✅ Check daylight factor against the IEQ category minimum.
    pub fn check_daylight_factor(category: ComfortCategory, df_percent: f64) -> CheckResult {
        let minimum = daylight_factor_min_percent(category);
        CheckResult::from_minimum(
            ClauseId::new("EN 16798-1", "Annex B", "B.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, df_percent),
            Quantity::new(norm_core::QuantityKind::Dimensionless, minimum),
            "daylight factor category",
            AnnexChoice::En,
        )
    }

    /// 🔊 Acoustic L_Aeq limit [dB] by IEQ category (folded TR EN 16798-11 acoustic guidance).
    pub fn acoustic_limit_db_by_category(category: ComfortCategory) -> f64 {
        match category {
            ComfortCategory::I => 30.0,
            ComfortCategory::II => 35.0,
            ComfortCategory::III => 40.0,
        }
    }

    /// ✅ Check ventilation sound level against the IEQ category limit.
    pub fn check_acoustic_category(category: ComfortCategory, l_aeq_db: f64) -> CheckResult {
        let limit = acoustic_limit_db_by_category(category);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-1", "Annex B", "B.2"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, l_aeq_db),
            Quantity::new(norm_core::QuantityKind::Dimensionless, limit),
            "acoustic category limit",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 🏷️ Indoor air quality (IDA) category per EN 16798-3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IdaClass {
        Ida1,
        Ida2,
        Ida3,
        Ida4,
    }

    impl IdaClass {
        /// 📈 Outdoor airflow multiplier relative to the IDA2 reference rate.
        pub fn outdoor_air_multiplier(self) -> f64 {
            match self {
                Self::Ida1 => 1.35,
                Self::Ida2 => 1.0,
                Self::Ida3 => 0.7,
                Self::Ida4 => 0.4,
            }
        }
    }

    /// 🏷️ Outdoor air (ODA) quality category per EN 16798-3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OdaClass {
        Oda1,
        Oda2,
        Oda3,
    }

    impl OdaClass {
        /// 🏷️ Human-readable outdoor air quality label.
        pub fn label(self) -> &'static str {
            match self {
                Self::Oda1 => "pure",
                Self::Oda2 => "moderate pollution",
                Self::Oda3 => "high pollution",
            }
        }
    }

    /// 📊 Specific outdoor airflow per person [m³/(h·person)] at IDA2 reference (EN 16798-3 Table 1).
    pub fn outdoor_air_per_person(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => 36.0,
            OccupancyType::Retail => 20.0,
            OccupancyType::Kitchen => 60.0,
            OccupancyType::Residential => 30.0,
            OccupancyType::Corridor => 10.0,
        }
    }

    /// 📐 Required outdoor airflow [m³/h] for the given IDA class.
    pub fn required_outdoor_air_m3_h(occupancy: OccupancyType, persons: u32, ida_class: IdaClass) -> f64 {
        outdoor_air_per_person(occupancy) * persons as f64 * ida_class.outdoor_air_multiplier()
    }

    /// ✅ Check ventilation rate for non-residential spaces against the IDA class requirement.
    pub fn check_ventilation_rate(occupancy: OccupancyType, persons: u32, ida_class: IdaClass, supplied_m3_h: f64) -> CheckResult {
        let required = required_outdoor_air_m3_h(occupancy, persons, ida_class);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "Table 1", "q"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "outdoor air supply",
            AnnexChoice::De,
        )
    }

    /// ⚡ Specific Fan Power class bounds [W/(m³/s)] (EN 16798-3 Table 10).
    pub const SFP_CLASS_BOUNDS_W_M3_S: [f64; 6] = [500.0, 750.0, 1250.0, 2000.0, 3000.0, 4500.0];

    /// 🏷️ Specific Fan Power class (SFP1 tightest .. SFP6 loosest).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SfpClass {
        Sfp1,
        Sfp2,
        Sfp3,
        Sfp4,
        Sfp5,
        Sfp6,
    }

    impl SfpClass {
        /// 📐 Upper SFP bound [W/(m³/s)] for this class.
        pub fn bound_w_m3_s(self) -> f64 {
            SFP_CLASS_BOUNDS_W_M3_S[self as usize]
        }

        fn from_ordinal(n: usize) -> Self {
            match n {
                0 => Self::Sfp1,
                1 => Self::Sfp2,
                2 => Self::Sfp3,
                3 => Self::Sfp4,
                4 => Self::Sfp5,
                _ => Self::Sfp6,
            }
        }
    }

    /// 🔢 Parse an SFP class number (1–6) into its `SfpClass`.
    pub fn sfp_class_from_number(n: u8) -> SfpClass {
        match n {
            1 => SfpClass::Sfp1,
            2 => SfpClass::Sfp2,
            3 => SfpClass::Sfp3,
            4 => SfpClass::Sfp4,
            5 => SfpClass::Sfp5,
            _ => SfpClass::Sfp6,
        }
    }

    /// 🔍 Classify a design Specific Fan Power [W/(m³/s)] into its SFP class (SFP = P_el / q_v).
    pub fn classify_sfp(sfp_w_m3_s: f64) -> SfpClass {
        let idx = SFP_CLASS_BOUNDS_W_M3_S.iter().position(|&bound| sfp_w_m3_s <= bound).unwrap_or(SFP_CLASS_BOUNDS_W_M3_S.len() - 1);
        SfpClass::from_ordinal(idx)
    }

    /// ✅ Check design SFP against the required class bound.
    pub fn check_design_sfp(sfp_w_m3_s: f64, required_class: SfpClass) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "Table 10", "SFP"),
            Quantity::new(norm_core::QuantityKind::Power, sfp_w_m3_s),
            Quantity::new(norm_core::QuantityKind::Power, required_class.bound_w_m3_s()),
            "specific fan power class",
            AnnexChoice::De,
        )
    }

    /// ✅ Check heat-recovery system efficiency against the minimum required (EN 16798-3 §7.3).
    pub fn check_heat_recovery_efficiency(eta_delivered: f64, eta_min: f64) -> CheckResult {
        CheckResult::from_minimum(
            ClauseId::new("EN 16798-3", "§7", "7.3"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eta_delivered),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eta_min),
            "minimum heat-recovery efficiency",
            AnnexChoice::De,
        )
    }

    /// 🏠 Dwelling whole-building ventilation rate [m³/h] (folded TR EN 16798-4).
    pub fn dwelling_ventilation_rate(floor_area_m2: f64, bedrooms: u32) -> f64 {
        let by_area = 0.5 * floor_area_m2;
        let by_bedroom = 21.0 * bedrooms.max(1) as f64;
        by_area.max(by_bedroom)
    }

    /// ✅ Check dwelling ventilation adequacy.
    pub fn check_dwelling_ventilation(floor_area_m2: f64, bedrooms: u32, supplied_m3_h: f64) -> CheckResult {
        let required = dwelling_ventilation_rate(floor_area_m2, bedrooms);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "§7", "7.1"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "dwelling ventilation rate",
            AnnexChoice::De,
        )
    }

    /// 🏠 Residential whole-building ventilation rate [m³/h] (relocated from the former part_7; consumed by DIN V 18599).
    pub fn residential_ventilation_rate(floor_area_m2: f64, occupants: u32) -> f64 {
        let by_area = 0.4 * floor_area_m2;
        let by_person = 30.0 * occupants.max(1) as f64;
        by_area.max(by_person)
    }

    /// ✅ Check residential ventilation adequacy.
    pub fn check_residential_ventilation(floor_area_m2: f64, occupants: u32, supplied_m3_h: f64) -> CheckResult {
        let required = residential_ventilation_rate(floor_area_m2, occupants);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "§7", "7.2"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "residential ventilation rate",
            AnnexChoice::De,
        )
    }

    /// 🔍 Inspection interval [years] for ventilation systems (folded TR EN 16798-10).
    pub fn inspection_interval_years(system_type: &str) -> u32 {
        match system_type {
            "central_mech" => 3,
            "decentral" => 5,
            _ => 3,
        }
    }

    /// ✅ Check whether the last inspection is within the required interval.
    pub fn check_inspection_due(system_type: &str, years_since_inspection: u32) -> CheckResult {
        let interval = inspection_interval_years(system_type);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "§8", "8.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, years_since_inspection as f64),
            Quantity::new(norm_core::QuantityKind::Dimensionless, interval as f64),
            "ventilation inspection interval",
            AnnexChoice::De,
        )
    }

    /// 💧 Humidification capacity check (folded TR EN 16798-9 humidification guidance).
    pub fn check_humidification_capacity(required_kg_h: f64, provided_kg_h: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-3", "§7", "7.4"),
            Quantity::new(norm_core::QuantityKind::Mass, provided_kg_h),
            Quantity::new(norm_core::QuantityKind::Mass, required_kg_h),
            "humidification capacity",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part3

// #region 🔖Part5_1
pub mod part_5_1 {
    use super::*;

    /// ⚡ Building-level fan electrical energy [kWh]: E_fan = SFP · q_v · t_run (EN 16798-5-1).
    pub fn fan_energy_kwh(sfp_w_m3_s: f64, q_v_m3_s: f64, t_run_h: f64) -> f64 {
        sfp_w_m3_s * q_v_m3_s * t_run_h / 1000.0
    }

    /// ✅ Check building-level ventilation fan energy against a reference allowance.
    pub fn check_building_fan_energy(sfp_w_m3_s: f64, q_v_m3_s: f64, t_run_h: f64, reference_kwh: f64) -> CheckResult {
        let computed = fan_energy_kwh(sfp_w_m3_s, q_v_m3_s, t_run_h);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-5-1", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Energy, computed),
            Quantity::new(norm_core::QuantityKind::Energy, reference_kwh),
            "building-level ventilation fan energy",
            AnnexChoice::De,
        )
    }

    /// 🎛️ Minimum night setback [K] for heating controls (folded TR EN 16798-12).
    pub fn night_setback_k(occupancy: OccupancyType) -> f64 {
        match occupancy {
            OccupancyType::Residential => 3.0,
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => 4.0,
            OccupancyType::Retail | OccupancyType::Kitchen | OccupancyType::Corridor => 2.0,
        }
    }

    /// ✅ Check that the night setback is deep enough to count as a building-level energy-saving measure.
    pub fn check_night_setback(occupancy: OccupancyType, configured_k: f64) -> CheckResult {
        let required = night_setback_k(occupancy);
        CheckResult::from_minimum(
            ClauseId::new("EN 16798-5-1", "§6", "6.2"),
            Quantity::new(norm_core::QuantityKind::Temperature, configured_k),
            Quantity::new(norm_core::QuantityKind::Temperature, required),
            "night setback depth",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part5_1

// #region 🔖Part5_2
pub mod part_5_2 {
    use super::*;

    /// 🌬️ Dry-air specific heat capacity [J/(kg·K)], the default c_p for heat-recovery energy.
    pub const AIR_CP_J_KGK: f64 = 1005.0;

    /// ♻️ System-level heat-recovery energy savings [kWh]: Q_hr = η_t · ṁ · c_p · ΔT · t (EN 16798-5-2).
    pub fn heat_recovery_savings_kwh(eta_t: f64, m_dot_kg_s: f64, cp_j_kgk: f64, delta_t_c: f64, t_h: f64) -> f64 {
        let power_w = m_dot_kg_s * cp_j_kgk * delta_t_c;
        eta_t * power_w * t_h / 1000.0
    }

    /// ✅ Check that heat-recovery energy savings meet a minimum reference requirement.
    pub fn check_heat_recovery_savings(eta_t: f64, m_dot_kg_s: f64, cp_j_kgk: f64, delta_t_c: f64, t_h: f64, reference_kwh: f64) -> CheckResult {
        let computed = heat_recovery_savings_kwh(eta_t, m_dot_kg_s, cp_j_kgk, delta_t_c, t_h);
        CheckResult::from_minimum(
            ClauseId::new("EN 16798-5-2", "§6", "6.3"),
            Quantity::new(norm_core::QuantityKind::Energy, computed),
            Quantity::new(norm_core::QuantityKind::Energy, reference_kwh),
            "system-level heat-recovery energy savings",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part5_2

// #region 🔖Part7
pub mod part_7 {
    use super::*;

    /// 💨 Infiltration airflow [m³/h]: q_inf = n₅₀ · V / 20 (EN 16798-7 simplified shielding-corrected method).
    pub fn infiltration_rate_m3_h(n50_h_inv: f64, volume_m3: f64) -> f64 {
        n50_h_inv * volume_m3 / 20.0
    }

    /// ✅ Check infiltration airflow against a design allowance.
    pub fn check_infiltration(n50_h_inv: f64, volume_m3: f64, design_allowance_m3_h: f64) -> CheckResult {
        let computed = infiltration_rate_m3_h(n50_h_inv, volume_m3);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-7", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, computed),
            Quantity::new(norm_core::QuantityKind::VentilationRate, design_allowance_m3_h),
            "infiltration airflow",
            AnnexChoice::De,
        )
    }

    /// 🏚️ Cellar ventilation rate [m³/h] per floor area (folded TR EN 16798-6).
    pub fn cellar_ventilation_rate(cellar_area_m2: f64) -> f64 {
        0.3 * cellar_area_m2
    }

    /// ✅ Check cellar ventilation adequacy.
    pub fn check_cellar_ventilation(cellar_area_m2: f64, supplied_m3_h: f64) -> CheckResult {
        let required = cellar_ventilation_rate(cellar_area_m2);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-7", "§6", "6.2"),
            Quantity::new(norm_core::QuantityKind::VentilationRate, supplied_m3_h),
            Quantity::new(norm_core::QuantityKind::VentilationRate, required),
            "cellar ventilation rate",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part7

// #region 🔖Part9
pub mod part_9 {
    use super::*;

    /// 🌡️ Raw cooling degree-hour energy demand [kWh]: (H_tr + H_ve) · max(θ_e − θ_set, 0) · Δt (EN 16798-9).
    pub fn cooling_degree_hours_energy_kwh(h_tr_w_k: f64, h_ve_w_k: f64, theta_e_c: f64, theta_set_c: f64, delta_t_h: f64) -> f64 {
        let h_total = h_tr_w_k + h_ve_w_k;
        let delta_theta = (theta_e_c - theta_set_c).max(0.0);
        h_total * delta_theta * delta_t_h / 1000.0
    }

    /// ❄️ Net cooling energy need [kWh]: raw degree-hour demand minus utilised free/internal gains.
    pub fn cooling_energy_need_kwh(h_tr_w_k: f64, h_ve_w_k: f64, theta_e_c: f64, theta_set_c: f64, delta_t_h: f64, gains_kwh: f64, utilization_factor: f64) -> f64 {
        let raw = cooling_degree_hours_energy_kwh(h_tr_w_k, h_ve_w_k, theta_e_c, theta_set_c, delta_t_h);
        (raw - utilization_factor * gains_kwh).max(0.0)
    }

    /// ✅ Check net cooling energy need against a reference value.
    pub fn check_cooling_energy_need(h_tr_w_k: f64, h_ve_w_k: f64, theta_e_c: f64, theta_set_c: f64, delta_t_h: f64, gains_kwh: f64, utilization_factor: f64, reference_kwh: f64) -> CheckResult {
        let computed = cooling_energy_need_kwh(h_tr_w_k, h_ve_w_k, theta_e_c, theta_set_c, delta_t_h, gains_kwh, utilization_factor);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-9", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Energy, computed),
            Quantity::new(norm_core::QuantityKind::Energy, reference_kwh),
            "net cooling energy need",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part9

// #region 🔖Part13
pub mod part_13 {
    use super::*;

    /// 🏷️ Chiller generation technology (EN 16798-13 Table 5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ChillerType {
        AirCooled,
        WaterCooled,
        Absorption,
    }

    /// 📊 Minimum EER by chiller type.
    pub fn eer_min(chiller_type: ChillerType) -> f64 {
        match chiller_type {
            ChillerType::AirCooled => 2.5,
            ChillerType::WaterCooled => 3.0,
            ChillerType::Absorption => 0.7,
        }
    }

    /// ✅ Check chiller EER against the minimum required for its type.
    pub fn check_chiller_eer(chiller_type: ChillerType, eer_actual: f64) -> CheckResult {
        CheckResult::from_minimum(
            ClauseId::new("EN 16798-13", "Table 5", "EER"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eer_actual),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eer_min(chiller_type)),
            "chiller EER",
            AnnexChoice::De,
        )
    }

    /// ⚡ Cooling generation electrical energy [kWh]: E = Q_C / EER.
    pub fn generation_energy_kwh(q_c_kwh: f64, eer: f64) -> f64 {
        q_c_kwh / eer
    }

    /// ✅ Check cooling generation energy against a reference value.
    pub fn check_generation_energy(q_c_kwh: f64, eer: f64, reference_kwh: f64) -> CheckResult {
        let computed = generation_energy_kwh(q_c_kwh, eer);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-13", "§6", "6.2"),
            Quantity::new(norm_core::QuantityKind::Energy, computed),
            Quantity::new(norm_core::QuantityKind::Energy, reference_kwh),
            "cooling generation energy",
            AnnexChoice::De,
        )
    }

    /// 🖥️ Data center supply air temperature band [°C] (folded TR EN 16798-16).
    pub fn supply_air_temperature_band_c() -> (f64, f64) {
        (18.0, 27.0)
    }

    /// ✅ Check data center supply air temperature.
    pub fn check_supply_air_temperature(t_supply_c: f64) -> CheckResult {
        let (t_min, t_max) = supply_air_temperature_band_c();
        let within = t_supply_c >= t_min && t_supply_c <= t_max;
        let status = if within { norm_core::CheckStatus::Pass } else { norm_core::CheckStatus::Fail };
        CheckResult {
            clause: ClauseId::new("EN 16798-13", "§7", "7.2"),
            status,
            computed: Quantity::new(norm_core::QuantityKind::Temperature, t_supply_c),
            limit: Quantity::new(norm_core::QuantityKind::Temperature, t_max),
            utilization: if within { 0.0 } else { 1.1 },
            message: "data center supply air temperature".into(),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖Part13

// #region 🔖Part15
pub mod part_15 {
    use super::*;

    /// 🔥 Storage losses [kWh]: Q_st = H_st · (θ_st − θ_amb) · t (EN 16798-15).
    pub fn storage_losses_kwh(h_st_w_k: f64, theta_st_c: f64, theta_amb_c: f64, t_h: f64) -> f64 {
        h_st_w_k * (theta_st_c - theta_amb_c) * t_h / 1000.0
    }

    /// ✅ Check storage losses against an allowance.
    pub fn check_storage_losses(h_st_w_k: f64, theta_st_c: f64, theta_amb_c: f64, t_h: f64, allowance_kwh: f64) -> CheckResult {
        let computed = storage_losses_kwh(h_st_w_k, theta_st_c, theta_amb_c, t_h);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-15", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Energy, computed),
            Quantity::new(norm_core::QuantityKind::Energy, allowance_kwh),
            "storage losses",
            AnnexChoice::De,
        )
    }

    /// 🚿 DHW delivery temperature band [°C] (folded TR EN 16798-14).
    pub fn dhw_delivery_temperature_band_c() -> (f64, f64) {
        (55.0, 60.0)
    }

    /// ✅ Check DHW delivery temperature.
    pub fn check_dhw_temperature(t_delivery_c: f64) -> CheckResult {
        let (t_min, t_max) = dhw_delivery_temperature_band_c();
        let within = t_delivery_c >= t_min && t_delivery_c <= t_max;
        let status = if within { norm_core::CheckStatus::Pass } else { norm_core::CheckStatus::Fail };
        CheckResult {
            clause: ClauseId::new("EN 16798-15", "§7", "7.1"),
            status,
            computed: Quantity::new(norm_core::QuantityKind::Temperature, t_delivery_c),
            limit: Quantity::new(norm_core::QuantityKind::Temperature, t_max),
            utilization: if within { 0.0 } else { 1.1 },
            message: "DHW delivery temperature".into(),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖Part15

// #region 🔖Part17
pub mod part_17 {
    use super::*;

    /// 🏷️ Ductwork/AHU air-leakage class (EN 16798-17 / EN 12237, absorbed from the former part_8).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DuctLeakageClass {
        A,
        B,
        C,
        D,
    }

    /// 📐 Leakage coefficient c in f_max = c · p^0.65.
    pub fn leakage_coefficient_c(class: DuctLeakageClass) -> f64 {
        match class {
            DuctLeakageClass::A => 0.027,
            DuctLeakageClass::B => 0.009,
            DuctLeakageClass::C => 0.003,
            DuctLeakageClass::D => 0.001,
        }
    }

    /// 🌀 Duct leakage limit [m³/(s·m²)] at the given test pressure: f_max = c · p^0.65.
    pub fn leakage_limit_m3_s_m2(class: DuctLeakageClass, test_pressure_pa: f64) -> f64 {
        leakage_coefficient_c(class) * test_pressure_pa.powf(0.65)
    }

    /// ✅ Check ductwork leakage at test pressure against the class limit.
    pub fn check_duct_leakage(class: DuctLeakageClass, test_pressure_pa: f64, measured_m3_s_m2: f64) -> CheckResult {
        let limit = leakage_limit_m3_s_m2(class, test_pressure_pa);
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-17", "§8", "8.2"),
            Quantity::new(norm_core::QuantityKind::AirPermeability, measured_m3_s_m2),
            Quantity::new(norm_core::QuantityKind::AirPermeability, limit),
            "ductwork leakage class",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part17

// #region 🔖AnnexParams
pub mod annex_params {
    use super::*;

    /// 🇪🇺🇩🇪 DE-NA divergence parameters for EN 16798 category thresholds.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub co2_limit_residential_ppm: f64,
        pub co2_limit_classroom_ppm: f64,
        pub co2_limit_other_ppm: f64,
        pub acoustic_limit_residential_db: f64,
    }

    impl AnnexParams {
        /// 🇪🇺 Base EN category thresholds (no national divergence).
        pub fn en() -> Self {
            Self { choice: AnnexChoice::En, co2_limit_residential_ppm: 1500.0, co2_limit_classroom_ppm: 1000.0, co2_limit_other_ppm: 1000.0, acoustic_limit_residential_db: 30.0 }
        }

        /// 🇩🇪 DIN EN 16798 NA-DE tightened thresholds.
        pub fn de() -> Self {
            Self { choice: AnnexChoice::De, co2_limit_residential_ppm: 1200.0, co2_limit_classroom_ppm: 800.0, co2_limit_other_ppm: 900.0, acoustic_limit_residential_db: 25.0 }
        }

        /// 🔀 Select annex parameters by `AnnexChoice`.
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice {
                AnnexChoice::En => Self::en(),
                AnnexChoice::De => Self::de(),
            }
        }
    }
}
// #endregion 🔖AnnexParams

/// 📋 End-to-end residential indoor environment check.
pub fn check_residential_environment(floor_area_m2: f64, occupants: u32, ventilation_m3_h: f64, t_op_c: f64, l_aeq_db: f64) -> CheckReport {
    let mut report = CheckReport::default();
    report.push(part_1::check_operative_temperature(OccupancyType::Residential, t_op_c));
    report.push(part_3::check_residential_ventilation(floor_area_m2, occupants, ventilation_m3_h));
    report.push(part_1::check_acoustic_category(part_1::ComfortCategory::II, l_aeq_db));
    report
}

fn parse_occupancy(occupancy: &str) -> OccupancyType {
    match occupancy.to_ascii_lowercase().as_str() {
        "office" => OccupancyType::Office,
        "meeting" => OccupancyType::Meeting,
        "classroom" => OccupancyType::Classroom,
        "retail" => OccupancyType::Retail,
        "kitchen" => OccupancyType::Kitchen,
        "corridor" => OccupancyType::Corridor,
        _ => OccupancyType::Residential,
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

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOperation};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "din16798", layout = "lines")]
pub struct Document {
    pub annex: AnnexChoice,
    pub occupancy: String,
    pub comfort_category: String,
    pub t_op_c: f64,
    pub rh_percent: f64,
    pub air_speed_m_s: f64,
    pub theta_rm_c: f64,
    pub co2_ppm: f64,
    pub df_percent: f64,
    pub l_aeq_db: f64,

    pub persons: u32,
    // Not `#[dsl(ident)]`: values like `"2"` are bare digits, which the lexer always tokenizes as
    // an integer, never as an identifier — quoted `Text` (the default String shape) has no such
    // ambiguity.
    pub ida_class: String,
    pub ventilation_m3_h: f64,
    pub floor_area_m2: f64,
    pub bedrooms: u32,
    pub dwelling_ventilation_m3_h: f64,
    pub occupants: u32,
    pub residential_ventilation_m3_h: f64,
    pub sfp_w_m3_s: f64,
    pub sfp_required_class: u8,
    pub heat_recovery_eta: f64,
    pub heat_recovery_eta_min: f64,
    pub system_type: String,
    pub years_since_inspection: u32,
    pub humidification_required_kg_h: f64,
    pub humidification_provided_kg_h: f64,

    pub fan_q_v_m3_s: f64,
    pub fan_t_run_h: f64,
    pub fan_energy_reference_kwh: f64,
    pub night_setback_k: f64,

    pub hr_m_dot_kg_s: f64,
    pub hr_cp_j_kgk: f64,
    pub hr_delta_t_c: f64,
    pub hr_t_h: f64,
    pub hr_savings_reference_kwh: f64,

    pub n50_h_inv: f64,
    pub volume_m3: f64,
    pub infiltration_allowance_m3_h: f64,
    pub cellar_area_m2: f64,
    pub cellar_ventilation_m3_h: f64,

    pub h_tr_w_k: f64,
    pub h_ve_w_k: f64,
    pub theta_e_c: f64,
    pub theta_set_c: f64,
    pub cooling_delta_t_h: f64,
    pub cooling_gains_kwh: f64,
    pub cooling_utilization_factor: f64,
    pub cooling_reference_kwh: f64,

    pub chiller_type: String,
    pub eer_actual: f64,
    pub q_c_kwh: f64,
    pub generation_reference_kwh: f64,
    pub data_center_supply_c: f64,

    pub h_st_w_k: f64,
    pub theta_st_c: f64,
    pub theta_amb_c: f64,
    pub storage_t_h: f64,
    pub storage_allowance_kwh: f64,
    pub dhw_delivery_c: f64,

    pub duct_class: String,
    pub duct_test_pressure_pa: f64,
    pub duct_leakage_m3_s_m2: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            occupancy: "residential".into(),
            comfort_category: "II".into(),
            t_op_c: 22.0,
            rh_percent: 50.0,
            air_speed_m_s: 0.1,
            theta_rm_c: 15.0,
            co2_ppm: 800.0,
            df_percent: 2.5,
            l_aeq_db: 24.0,

            persons: 10,
            ida_class: "2".into(),
            ventilation_m3_h: 280.0,
            floor_area_m2: 90.0,
            bedrooms: 3,
            dwelling_ventilation_m3_h: 63.0,
            occupants: 3,
            residential_ventilation_m3_h: 80.0,
            sfp_w_m3_s: 1500.0,
            sfp_required_class: 4,
            heat_recovery_eta: 0.75,
            heat_recovery_eta_min: 0.70,
            system_type: "central_mech".into(),
            years_since_inspection: 1,
            humidification_required_kg_h: 2.0,
            humidification_provided_kg_h: 2.0,

            fan_q_v_m3_s: 1.0,
            fan_t_run_h: 8.0,
            fan_energy_reference_kwh: 15.0,
            night_setback_k: 3.5,

            hr_m_dot_kg_s: 0.5,
            hr_cp_j_kgk: 1005.0,
            hr_delta_t_c: 15.0,
            hr_t_h: 10.0,
            hr_savings_reference_kwh: 50.0,

            n50_h_inv: 1.5,
            volume_m3: 500.0,
            infiltration_allowance_m3_h: 45.0,
            cellar_area_m2: 50.0,
            cellar_ventilation_m3_h: 15.0,

            h_tr_w_k: 200.0,
            h_ve_w_k: 100.0,
            theta_e_c: 32.0,
            theta_set_c: 26.0,
            cooling_delta_t_h: 10.0,
            cooling_gains_kwh: 5.0,
            cooling_utilization_factor: 0.8,
            cooling_reference_kwh: 20.0,

            chiller_type: "air_cooled".into(),
            eer_actual: 3.0,
            q_c_kwh: 1000.0,
            generation_reference_kwh: 400.0,
            data_center_supply_c: 22.0,

            h_st_w_k: 5.0,
            theta_st_c: 60.0,
            theta_amb_c: 20.0,
            storage_t_h: 24.0,
            storage_allowance_kwh: 6.0,
            dhw_delivery_c: 58.0,

            duct_class: "C".into(),
            duct_test_pressure_pa: 400.0,
            duct_leakage_m3_s_m2: 0.10,
        }
    }
}

pub type Operation = SetDocumentOperation<Document>;
pub type Host = NormHost<DinEn16798Family>;

/// 📋 Full EN 16798 normative parts (1, 3, 5-1, 5-2, 7, 9, 13, 15, 17) plus DE-NA divergent checks.
pub fn check_full_environment(document: &Document) -> CheckReport {
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

pub fn evaluate(document: &Document) -> CheckReport {
    check_full_environment(document)
}

pub struct DinEn16798Family;

impl NormFamily for DinEn16798Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::DinEn16798
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖Session

// #region 🔖Dsl
// `Document`'s `vcs::DocumentDsl` impl is now generated by `#[derive(dsl::DslDocument)]` on the
// type definition above — the engine's `dsl_schema` grammar replaces this crate's own hand-rolled
// `norm_core::dsl_kv` printer/parser.
// #endregion 🔖Dsl

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
    fn pmv_iso7730_neutral_at_comfort_conditions() {
        let pmv = part_1::pmv_iso7730(22.0, 50.0, 0.1);
        assert!(pmv.abs() < 0.5, "pmv={pmv}");
        let ppd = part_1::ppd_from_pmv(pmv);
        assert!(ppd < 10.0, "ppd={ppd}");
    }

    #[test]
    fn adaptive_comfort_center_and_category_band() {
        let centre = part_1::adaptive_comfort_temperature_c(20.0);
        assert!((centre - 25.4).abs() < 1e-9, "centre={centre}");
        let check = part_1::check_adaptive_comfort(20.0, 24.0, part_1::ComfortCategory::II);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn co2_annex_divergence_de_stricter_than_en() {
        let en = annex_params::AnnexParams::en();
        let de = annex_params::AnnexParams::de();
        assert!((en.co2_limit_classroom_ppm - 1000.0).abs() < 1e-9);
        assert!((de.co2_limit_classroom_ppm - 800.0).abs() < 1e-9);
        let en_check = part_1::check_co2_level(OccupancyType::Classroom, 850.0, &en);
        let de_check = part_1::check_co2_level(OccupancyType::Classroom, 850.0, &de);
        assert_eq!(en_check.status, norm_core::CheckStatus::Pass);
        assert_eq!(de_check.status, norm_core::CheckStatus::Fail);
    }

    #[test]
    fn daylight_factor_category_ii_minimum() {
        assert!((part_1::daylight_factor_min_percent(part_1::ComfortCategory::II) - 2.0).abs() < 1e-9);
        let pass = part_1::check_daylight_factor(part_1::ComfortCategory::II, 2.5);
        assert_eq!(pass.status, norm_core::CheckStatus::Pass);
        let fail = part_1::check_daylight_factor(part_1::ComfortCategory::II, 1.0);
        assert_eq!(fail.status, norm_core::CheckStatus::Fail);
    }

    #[test]
    fn acoustic_category_ii_limit() {
        let check = part_1::check_acoustic_category(part_1::ComfortCategory::II, 24.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn ventilation_rates_per_room_type_at_ida2() {
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Office), 36.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Meeting), 36.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Classroom), 36.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Retail), 20.0);
        assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Kitchen), 60.0);
        let office = part_3::check_ventilation_rate(OccupancyType::Office, 10, part_3::IdaClass::Ida2, 360.0);
        assert_eq!(office.status, norm_core::CheckStatus::Pass);
        assert!((office.limit.value - 360.0).abs() < 1e-9);
    }

    #[test]
    fn sfp_1500w_1m3s_falls_in_class_4() {
        let sfp_w_m3_s = 1500.0_f64 / 1.0;
        assert!((sfp_w_m3_s - 1500.0).abs() < 1e-9);
        assert_eq!(part_3::classify_sfp(sfp_w_m3_s), part_3::SfpClass::Sfp4);
        let check = part_3::check_design_sfp(sfp_w_m3_s, part_3::SfpClass::Sfp4);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn dwelling_ventilation_85m2_3_bedrooms() {
        let rate = part_3::dwelling_ventilation_rate(85.0, 3);
        assert!((rate - 63.0).abs() < 1e-9);
    }

    #[test]
    fn residential_ventilation_rate_100m2_4_occupants() {
        let rate = part_3::residential_ventilation_rate(100.0, 4);
        assert!((rate - 120.0).abs() < 1e-9);
    }

    #[test]
    fn heat_recovery_efficiency_minimum() {
        let check = part_3::check_heat_recovery_efficiency(0.75, 0.70);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn inspection_due_within_interval() {
        let check = part_3::check_inspection_due("central_mech", 1);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn humidification_capacity_meets_requirement() {
        let check = part_3::check_humidification_capacity(2.0, 2.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn fan_energy_1500w_1m3s_8h_is_12kwh() {
        let energy = part_5_1::fan_energy_kwh(1500.0, 1.0, 8.0);
        assert!((energy - 12.0).abs() < 1e-9, "energy={energy}");
        let check = part_5_1::check_building_fan_energy(1500.0, 1.0, 8.0, 15.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn night_setback_residential_minimum() {
        let check = part_5_1::check_night_setback(OccupancyType::Residential, 3.5);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn heat_recovery_savings_worked_example() {
        let savings = part_5_2::heat_recovery_savings_kwh(0.75, 0.5, part_5_2::AIR_CP_J_KGK, 15.0, 10.0);
        assert!((savings - 56.53125).abs() < 1e-6, "savings={savings}");
        let check = part_5_2::check_heat_recovery_savings(0.75, 0.5, part_5_2::AIR_CP_J_KGK, 15.0, 10.0, 50.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn infiltration_n50_1_5_volume_500m3_is_37_5() {
        let rate = part_7::infiltration_rate_m3_h(1.5, 500.0);
        assert!((rate - 37.5).abs() < 1e-9, "rate={rate}");
        let check = part_7::check_infiltration(1.5, 500.0, 45.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn cellar_ventilation_50m2() {
        let rate = part_7::cellar_ventilation_rate(50.0);
        assert!((rate - 15.0).abs() < 1e-9);
    }

    #[test]
    fn cooling_energy_need_worked_example() {
        let net = part_9::cooling_energy_need_kwh(200.0, 100.0, 32.0, 26.0, 10.0, 5.0, 0.8);
        assert!((net - 14.0).abs() < 1e-9, "net={net}");
        let check = part_9::check_cooling_energy_need(200.0, 100.0, 32.0, 26.0, 10.0, 5.0, 0.8, 20.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn chiller_eer_table_lookup_air_cooled() {
        assert!((part_13::eer_min(part_13::ChillerType::AirCooled) - 2.5).abs() < 1e-9);
        let check = part_13::check_chiller_eer(part_13::ChillerType::AirCooled, 3.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
        let generation = part_13::generation_energy_kwh(1000.0, 3.0);
        assert!((generation - 333.3333333333333).abs() < 1e-6, "generation={generation}");
    }

    #[test]
    fn data_center_supply_air_22c_passes() {
        let check = part_13::check_supply_air_temperature(22.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn storage_losses_worked_example() {
        let losses = part_15::storage_losses_kwh(5.0, 60.0, 20.0, 24.0);
        assert!((losses - 4.8).abs() < 1e-9, "losses={losses}");
        let check = part_15::check_storage_losses(5.0, 60.0, 20.0, 24.0, 6.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn dhw_delivery_temperature_58c_passes() {
        let check = part_15::check_dhw_temperature(58.0);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn duct_leakage_class_c_400pa_worked_example() {
        let limit = part_17::leakage_limit_m3_s_m2(part_17::DuctLeakageClass::C, 400.0);
        assert!((limit - 0.1473873631338949).abs() < 1e-6, "limit={limit}");
        let check = part_17::check_duct_leakage(part_17::DuctLeakageClass::C, 400.0, 0.10);
        assert_eq!(check.status, norm_core::CheckStatus::Pass);
    }

    #[test]
    fn residential_environment_e2e_with_acoustic() {
        let report = check_residential_environment(85.0, 3, 40.0, 21.0, 24.0);
        assert!(report.all_pass());
        assert_eq!(report.checks.len(), 3);
    }

    #[test]
    fn full_environment_evaluate_covers_all_nine_parts() {
        let document = Document::default();
        let report = evaluate(&document);
        assert_eq!(report.checks.len(), 25, "checks: {:?}", report.checks);
        assert!(report.all_pass(), "checks: {:?}", report.checks);
        assert_eq!(document.annex, AnnexChoice::De);
        let pmv = part_1::pmv_iso7730(document.t_op_c, document.rh_percent, document.air_speed_m_s);
        assert!(pmv.abs() < 0.5);
    }

    #[test]
    fn document_dsl_round_trips() {
        vcs::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn set_document_op_text_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = vcs::create_document_vcs_envelope("norm.din16798/v1", "din16798", Document::default(), None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        store
            .dispatch(vcs::DocumentVcsCommand::Apply {
                operations: vec![Operation::SetDocument { document: Document::default() }],
                description: None,
            })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
}
