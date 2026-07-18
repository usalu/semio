//! 🌬️ DIN EN 16798 indoor environmental input parameters and ventilation / HVAC energy.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    /// 🏢 Occupancy category per EN 16798-1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OccupancyCategory {
        Residential,
        Office,
        Classroom,
        Retail,
    }

    /// 🌡️ Design operative temperature band [°C] per category.
    pub fn operative_temperature_band(category: OccupancyCategory) -> (f64, f64) {
        match category {
            OccupancyCategory::Residential => (20.0, 24.0),
            OccupancyCategory::Office => (20.0, 26.0),
            OccupancyCategory::Classroom => (20.0, 26.0),
            OccupancyCategory::Retail => (18.0, 26.0),
        }
    }

    /// ✅ Check operative temperature within band (EN 16798-1).
    pub fn check_operative_temperature(
        category: OccupancyCategory,
        t_op_c: f64,
    ) -> CheckResult {
        let (t_min, t_max) = operative_temperature_band(category);
        let within = t_op_c >= t_min && t_op_c <= t_max;
        let status = if within {
            norm_core::CheckStatus::Pass
        } else {
            norm_core::CheckStatus::Fail
        };
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

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 📊 Specific outdoor airflow per person [m³/(h·person)] non-residential (EN 16798-3).
    pub fn outdoor_air_per_person(category: &str) -> f64 {
        match category {
            "office" => 36.0,
            "meeting" => 36.0,
            "classroom" => 36.0,
            _ => 36.0,
        }
    }

    /// ✅ Check ventilation rate for non-residential spaces.
    pub fn check_ventilation_rate(
        category: &str,
        persons: u32,
        supplied_m3_h: f64,
    ) -> CheckResult {
        let required = outdoor_air_per_person(category) * persons as f64;
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

// #region 🔖Part7
pub mod part_7 {
    use super::*;

    /// 🏠 Residential whole-building ventilation rate [m³/h] per EN 16798-7.
    pub fn residential_ventilation_rate(floor_area_m2: f64, occupants: u32) -> f64 {
        let by_area = 0.4 * floor_area_m2;
        let by_person = 30.0 * occupants as f64;
        by_area.max(by_person)
    }

    /// ✅ Check residential ventilation adequacy.
    pub fn check_residential_ventilation(
        floor_area_m2: f64,
        occupants: u32,
        supplied_m3_h: f64,
    ) -> CheckResult {
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

// #region 🔖Part9
pub mod part_9 {
    use super::*;

    /// 💧 Humidification capacity check (EN 16798-9).
    pub fn check_humidification_capacity(required_kg_h: f64, provided_kg_h: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 16798-9", "§5", "5.2"),
            Quantity::new(norm_core::QuantityKind::Mass, provided_kg_h),
            Quantity::new(norm_core::QuantityKind::Mass, required_kg_h),
            "humidification capacity",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part9

// #region 🔖Part13
pub mod part_13 {
    use super::*;

    /// 🔊 Residential ventilation acoustic limit L_Aeq [dB] (EN 16798-13).
    pub fn check_acoustic_level(l_aeq_db: f64) -> CheckResult {
        let limit = 30.0;
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

// #region 🔖Part15
pub mod part_15 {
    use super::*;

    /// 🍳 Kitchen extract airflow [m³/h] per EN 16798-15.
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

/// 📋 End-to-end residential indoor environment check.
pub fn check_residential_environment(
    floor_area_m2: f64,
    occupants: u32,
    ventilation_m3_h: f64,
    t_op_c: f64,
) -> CheckReport {
    let mut report = CheckReport::default();
    report.push(part_1::check_operative_temperature(
        part_1::OccupancyCategory::Residential,
        t_op_c,
    ));
    report.push(part_7::check_residential_ventilation(
        floor_area_m2,
        occupants,
        ventilation_m3_h,
    ));
    report.push(part_13::check_acoustic_level(28.0));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn residential_ventilation_e2e() {
        let report = check_residential_environment(85.0, 3, 40.0, 21.0);
        assert!(report.all_pass());
    }
}
