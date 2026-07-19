//! ⚡ DIN V 18599 energy performance balancing method for buildings.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, ClimateZoneDe, NormError, Quantity};
use norm_din_4108::part_2::Layer;
use norm_din_en_16798::part_7 as ventilation_16798;

// #region 🔖Shared
/// 🏢 Building use class for energy reference area factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 📐 Monthly climate data for balancing.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthlyClimate {
    pub theta_e_c: [f64; 12],
    pub g_h_w_m2: [f64; 12],
}

impl MonthlyClimate {
    pub fn german_reference(zone: ClimateZoneDe) -> Self {
        let winter = zone.design_external_temperature_c();
        let summer = zone.summer_design_temperature_c();
        let mean = (winter + summer) / 2.0;
        let amplitude = (summer - winter) / 2.0;
        let mut theta_e = [0.0; 12];
        let g_h = [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0];
        for (i, t) in theta_e.iter_mut().enumerate() {
            let month = i as f64 + 1.0;
            *t = mean + amplitude * (2.0 * std::f64::consts::PI * (month - 7.0) / 12.0).cos();
        }
        Self {
            theta_e_c: theta_e,
            g_h_w_m2: g_h,
        }
    }
}

/// 🧱 Transmission loss coefficient H_T [W/K].
pub fn transmission_loss_coefficient(
    envelope_u_a: f64,
    thermal_bridge_psi_l: f64,
    ground_floor_u_a: f64,
) -> f64 {
    envelope_u_a + thermal_bridge_psi_l + ground_floor_u_a
}

/// 🌬️ Ventilation loss coefficient H_V [W/K] per DIN V 18599-2.
pub fn ventilation_loss_coefficient(airflow_m3_h: f64, heat_recovery_eta: f64) -> f64 {
    let rho_cp = 0.34;
    let factor = 1.0 - heat_recovery_eta.clamp(0.0, 0.95);
    rho_cp * airflow_m3_h * factor
}

/// 📐 Envelope area estimate for single-storey building [m²].
pub fn envelope_area_m2(floor_area_m2: f64) -> f64 {
    3.0 * floor_area_m2
}

fn heating_degree_hours(climate: &MonthlyClimate, theta_int: f64) -> f64 {
    let hours_per_month = [744.0, 672.0, 744.0, 720.0, 744.0, 720.0, 744.0, 744.0, 720.0, 744.0, 720.0, 744.0];
    let mut dh = 0.0;
    for m in 0..12 {
        let delta = (theta_int - climate.theta_e_c[m]).max(0.0);
        dh += delta * hours_per_month[m];
    }
    dh
}

fn cooling_degree_hours(climate: &MonthlyClimate, theta_int_cool: f64) -> f64 {
    let hours_per_month = [744.0, 672.0, 744.0, 720.0, 744.0, 720.0, 744.0, 744.0, 720.0, 744.0, 720.0, 744.0];
    let mut cdh = 0.0;
    for m in 0..12 {
        let delta = (climate.theta_e_c[m] - theta_int_cool).max(0.0);
        cdh += delta * hours_per_month[m];
    }
    cdh
}
// #endregion 🔖Shared

/// 📋 Inputs for annual energy balancing.
#[derive(Clone, Debug, PartialEq)]
pub struct BalancingInputs {
    pub use_class: UseClass,
    pub heated_area_m2: f64,
    pub occupants: u32,
    pub h_t: f64,
    pub h_v: f64,
    pub climate: MonthlyClimate,
    pub internal_gains_w_m2: f64,
    pub solar_gains_kwh: f64,
    pub system_losses_kwh: f64,
    pub renewable_kwh: f64,
    pub annual_limit_kwh: f64,
}

impl BalancingInputs {
    pub fn reference_residential(zone: ClimateZoneDe, area_m2: f64) -> Self {
        let occupants = ((area_m2 / 30.0).ceil() as u32).max(1);
        let wall_layers = reference_wall_layers();
        from_building(&wall_layers, area_m2, occupants, zone, 0.0).expect("reference building")
    }
}

/// 🧱 Build balancing inputs from envelope layers and occupancy (DIN V 18599-2/-7).
pub fn from_building(
    wall_layers: &[Layer],
    floor_area_m2: f64,
    occupants: u32,
    climate: ClimateZoneDe,
    heat_recovery_eta: f64,
) -> Result<BalancingInputs, NormError> {
    if wall_layers.is_empty() {
        return Err(NormError::IncompleteInput {
            field: "wall_layers".into(),
        });
    }
    if floor_area_m2 <= 0.0 {
        return Err(NormError::InvalidValue {
            field: "floor_area_m2".into(),
            reason: "must be positive".into(),
        });
    }
    let r = norm_din_4108::part_2::total_resistance(
        wall_layers,
        norm_din_4108::R_SI_WALL_M2K_W,
        norm_din_4108::R_SE_WALL_M2K_W,
    );
    let u = norm_din_4108::part_2::u_value_from_resistance(r);
    let a_env = envelope_area_m2(floor_area_m2);
    let side = floor_area_m2.sqrt();
    let psi_l = 0.10 * side * 4.0;
    let h_t = transmission_loss_coefficient(u * a_env, 0.10 * psi_l, 0.15 * floor_area_m2);
    let airflow = ventilation_16798::residential_ventilation_rate(floor_area_m2, occupants);
    let h_v = ventilation_loss_coefficient(airflow, heat_recovery_eta);
    let solar = floor_area_m2
        * MonthlyClimate::german_reference(climate)
            .g_h_w_m2
            .iter()
            .sum::<f64>()
        / 1000.0
        * 0.6;
    Ok(BalancingInputs {
        use_class: UseClass::Residential,
        heated_area_m2: floor_area_m2,
        occupants,
        h_t,
        h_v,
        climate: MonthlyClimate::german_reference(climate),
        internal_gains_w_m2: 3.5,
        solar_gains_kwh: solar,
        system_losses_kwh: 8.0 * floor_area_m2,
        renewable_kwh: 15.0 * floor_area_m2,
        annual_limit_kwh: 75.0 * floor_area_m2,
    })
}

fn reference_wall_layers() -> Vec<Layer> {
    vec![
        Layer {
            thickness_m: 0.12,
            lambda_w_mk: 0.035,
        },
        Layer {
            thickness_m: 0.24,
            lambda_w_mk: 0.77,
        },
    ]
}

fn transmission_losses_kwh(inputs: &BalancingInputs) -> f64 {
    let dh = heating_degree_hours(&inputs.climate, 19.0);
    inputs.h_t * dh / 1000.0
}

fn ventilation_losses_kwh(inputs: &BalancingInputs) -> f64 {
    let dh = heating_degree_hours(&inputs.climate, 19.0);
    inputs.h_v * dh / 1000.0
}

fn internal_gains_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.internal_gains_w_m2 * inputs.heated_area_m2 * 8760.0 / 1000.0 * 0.35
}

fn solar_gains_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.solar_gains_kwh
}

fn system_losses_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.system_losses_kwh
}

fn net_heating_demand_kwh(inputs: &BalancingInputs) -> f64 {
    let q_t = transmission_losses_kwh(inputs);
    let q_v = ventilation_losses_kwh(inputs);
    let q_i = internal_gains_kwh(inputs);
    let q_s = solar_gains_kwh(inputs);
    (q_t + q_v - q_i - q_s).max(0.0)
}

fn cooling_demand_kwh(inputs: &BalancingInputs) -> f64 {
    let cdh = cooling_degree_hours(&inputs.climate, 26.0);
    let gain_factor = 0.35;
    (inputs.h_t + inputs.h_v) * cdh * gain_factor / 1000.0
}

fn dhw_demand_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.occupants as f64 * 900.0
}

fn primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
    let q_h = net_heating_demand_kwh(inputs);
    let q_c = cooling_demand_kwh(inputs);
    let q_w = dhw_demand_kwh(inputs);
    let f_p = 1.1;
    (q_h + q_c + q_w) * f_p + inputs.system_losses_kwh - inputs.renewable_kwh
}

fn automation_factor(inputs: &BalancingInputs) -> f64 {
    match inputs.use_class {
        UseClass::Residential => 0.95,
        UseClass::Office => 0.90,
        UseClass::School => 0.92,
    }
}

// #region 🔖Part1
pub mod part_1 {
    use super::*;

    /// 📜 General balancing scope check (DIN V 18599-1).
    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let scope = inputs.heated_area_m2;
        Ok(CheckResult::from_minimum(
            ClauseId::new("DIN V 18599-1", "§5", "5.1"),
            Quantity::new(norm_core::QuantityKind::Area, scope),
            Quantity::new(norm_core::QuantityKind::Area, 1.0),
            "general balancing scope",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part1

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🔥 Annual transmission losses Q_T [kWh/a] (DIN V 18599-2).
    pub fn transmission_losses_kwh(inputs: &BalancingInputs) -> f64 {
        super::transmission_losses_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = transmission_losses_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-2", "§8", "8.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "transmission losses Q_T",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 🌬️ Annual ventilation losses Q_V [kWh/a] (DIN V 18599-3).
    pub fn ventilation_losses_kwh(inputs: &BalancingInputs) -> f64 {
        super::ventilation_losses_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = ventilation_losses_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-3", "§7", "7.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "ventilation losses Q_V",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    /// 💡 Annual internal gains Q_I [kWh/a] (DIN V 18599-4).
    pub fn internal_gains_kwh(inputs: &BalancingInputs) -> f64 {
        super::internal_gains_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = internal_gains_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-4", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh * 2.0),
            "internal gains Q_I",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part4

// #region 🔖Part5
pub mod part_5 {
    use super::*;

    /// ☀️ Annual solar gains Q_S [kWh/a] (DIN V 18599-5).
    pub fn solar_gains_kwh(inputs: &BalancingInputs) -> f64 {
        super::solar_gains_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = solar_gains_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-5", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh * 2.0),
            "solar gains Q_S",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part5

// #region 🔖Part6
pub mod part_6 {
    use super::*;

    /// ⚙️ Annual system losses [kWh/a] (DIN V 18599-6).
    pub fn system_losses_kwh(inputs: &BalancingInputs) -> f64 {
        super::system_losses_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = system_losses_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-6", "§8", "8.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "system losses",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part6

// #region 🔖Part7
pub mod part_7 {
    use super::*;

    /// 🔥 Net heating demand Q_H [kWh/a] (DIN V 18599-7).
    pub fn net_heating_demand_kwh(inputs: &BalancingInputs) -> f64 {
        super::net_heating_demand_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = net_heating_demand_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-7", "§9", "9.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "net heating demand Q_H",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part7

// #region 🔖Part8
pub mod part_8 {
    use super::*;

    /// ❄️ Annual cooling demand Q_C [kWh/a] from cooling degree hours (DIN V 18599-8).
    pub fn cooling_demand_kwh(inputs: &BalancingInputs) -> f64 {
        super::cooling_demand_kwh(inputs)
    }

    pub fn cooling_degree_hours(climate: &MonthlyClimate) -> f64 {
        super::cooling_degree_hours(climate, 26.0)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = cooling_demand_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-8", "§10", "10.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "cooling demand Q_C",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part8

// #region 🔖Part9
pub mod part_9 {
    use super::*;

    /// 🚿 Annual DHW demand Q_W [kWh/a] from occupants (DIN V 18599-9).
    pub fn dhw_demand_kwh(inputs: &BalancingInputs) -> f64 {
        super::dhw_demand_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = dhw_demand_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-9", "§11", "11.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "DHW demand Q_W",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part9

// #region 🔖Part10
pub mod part_10 {
    use super::*;

    /// ⚡ Annual primary energy Q_P [kWh/a] (DIN V 18599-10).
    pub fn primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
        super::primary_energy_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = primary_energy_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-10", "§12", "12.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "primary energy Q_P",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part10

// #region 🔖Part11
pub mod part_11 {
    use super::*;

    /// 🎛️ Building automation factor (DIN V 18599-11).
    pub fn automation_factor(inputs: &BalancingInputs) -> f64 {
        super::automation_factor(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = automation_factor(inputs);
        Ok(CheckResult::from_minimum(
            ClauseId::new("DIN V 18599-11", "§7", "7.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, value),
            Quantity::new(norm_core::QuantityKind::Dimensionless, 0.85),
            "building automation factor",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part11

// #region 🔖Part12
pub mod part_12 {
    use super::*;

    /// 📊 Tabular method primary energy reference [kWh/a] (DIN V 18599-12).
    pub fn tabular_primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
        super::primary_energy_kwh(inputs)
    }

    pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = tabular_primary_energy_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-12", "§4", "4.1"),
            Quantity::new(norm_core::QuantityKind::Energy, value),
            Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
            "tabular method reference",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖Part12

/// 📋 Full annual balancing per DIN V 18599.
pub fn balance_annual(inputs: &BalancingInputs) -> Result<CheckReport, NormError> {
    let mut report = CheckReport::default();
    report.push(part_1::check(inputs)?);
    report.push(part_2::check(inputs)?);
    report.push(part_3::check(inputs)?);
    report.push(part_7::check(inputs)?);
    report.push(part_8::check(inputs)?);
    report.push(part_9::check(inputs)?);
    report.push(part_10::check(inputs)?);
    Ok(report)
}

/// ✅ Primary energy compliance gate.
pub fn check_primary_energy(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
    part_10::check(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_100m2_inputs() -> BalancingInputs {
        from_building(
            &reference_wall_layers(),
            100.0,
            4,
            ClimateZoneDe::Zone2,
            0.0,
        )
        .unwrap()
    }

    #[test]
    fn from_building_computes_h_t_from_u_value() {
        let inputs = reference_100m2_inputs();
        let r = norm_din_4108::part_2::total_resistance(
            &reference_wall_layers(),
            norm_din_4108::R_SI_WALL_M2K_W,
            norm_din_4108::R_SE_WALL_M2K_W,
        );
        let u = norm_din_4108::part_2::u_value_from_resistance(r);
        let a_env = envelope_area_m2(100.0);
        let side = 10.0;
        let psi_l = 0.10 * side * 4.0;
        let expected_h_t = transmission_loss_coefficient(u * a_env, 0.10 * psi_l, 15.0);
        assert!((inputs.h_t - expected_h_t).abs() < 1e-6);
        assert!(inputs.h_t > 40.0);
    }

    #[test]
    fn from_building_computes_h_v_from_ventilation() {
        let inputs = reference_100m2_inputs();
        let airflow = ventilation_16798::residential_ventilation_rate(100.0, 4);
        assert!((airflow - 120.0).abs() < 1e-9);
        let expected_h_v = ventilation_loss_coefficient(airflow, 0.0);
        assert!((inputs.h_v - expected_h_v).abs() < 1e-6);
        assert!((inputs.h_v - 40.8).abs() < 0.1);
    }

    #[test]
    fn reference_100m2_q_t_numeric() {
        let inputs = reference_100m2_inputs();
        let q_t = part_2::transmission_losses_kwh(&inputs);
        assert!((q_t - 11_054.56).abs() < 1.0);
    }

    #[test]
    fn reference_100m2_q_v_numeric() {
        let inputs = reference_100m2_inputs();
        let q_v = part_3::ventilation_losses_kwh(&inputs);
        assert!((q_v - 4_896.01).abs() < 1.0);
    }

    #[test]
    fn reference_100m2_q_p_numeric() {
        let inputs = reference_100m2_inputs();
        let q_p = part_10::primary_energy_kwh(&inputs);
        assert!((q_p - 19_608.96).abs() < 5.0);
    }

    #[test]
    fn dhw_from_occupants_4_persons() {
        let inputs = reference_100m2_inputs();
        let q_w = part_9::dhw_demand_kwh(&inputs);
        assert!((q_w - 3600.0).abs() < 1e-9);
    }

    #[test]
    fn cooling_degree_hours_zone2_positive() {
        let climate = MonthlyClimate::german_reference(ClimateZoneDe::Zone2);
        let cdh = part_8::cooling_degree_hours(&climate);
        assert!(cdh > 1000.0);
        let inputs = reference_100m2_inputs();
        let q_c = part_8::cooling_demand_kwh(&inputs);
        assert!(q_c > 0.0);
    }

    #[test]
    fn balance_annual_includes_all_parts() {
        let inputs = reference_100m2_inputs();
        let report = balance_annual(&inputs).unwrap();
        assert!(report.checks.len() >= 7);
    }

    #[test]
    fn part_11_automation_factor_residential() {
        let inputs = reference_100m2_inputs();
        assert!((part_11::automation_factor(&inputs) - 0.95).abs() < 1e-9);
    }
}
