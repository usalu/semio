//! ⚡ DIN V 18599 energy performance balancing method for buildings.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, ClimateZoneDe, NormError, Quantity};

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
        let t_base = zone.design_external_temperature_c();
        let mut theta_e = [0.0; 12];
        let g_h = [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0];
        for (i, t) in theta_e.iter_mut().enumerate() {
            let month_factor = [0.0, 0.2, 0.5, 0.8, 1.0, 1.0, 1.0, 1.0, 0.7, 0.4, 0.1, -0.1][i];
            *t = t_base + 8.0 * month_factor;
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
// #endregion 🔖Shared

macro_rules! part_module {
    ($name:ident, $part:expr, $section:expr, $msg:expr, $compute:expr) => {
        pub mod $name {
            use super::*;
            pub fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
                let value = $compute(inputs);
                Ok(CheckResult::from_utilization(
                    ClauseId::new("DIN V 18599", $part, $section),
                    Quantity::new(norm_core::QuantityKind::Energy, value),
                    Quantity::new(norm_core::QuantityKind::Energy, inputs.annual_limit_kwh),
                    $msg,
                    AnnexChoice::De,
                ))
            }
        }
    };
}

/// 📋 Inputs for annual energy balancing.
#[derive(Clone, Debug, PartialEq)]
pub struct BalancingInputs {
    pub use_class: UseClass,
    pub heated_area_m2: f64,
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
        Self {
            use_class: UseClass::Residential,
            heated_area_m2: area_m2,
            h_t: 120.0,
            h_v: 45.0,
            climate: MonthlyClimate::german_reference(zone),
            internal_gains_w_m2: 3.5,
            solar_gains_kwh: 25.0 * area_m2,
            system_losses_kwh: 8.0 * area_m2,
            renewable_kwh: 15.0 * area_m2,
            annual_limit_kwh: 75.0 * area_m2,
        }
    }
}

fn degree_hours(climate: &MonthlyClimate, theta_int: f64) -> f64 {
    let hours_per_month = [744.0, 672.0, 744.0, 720.0, 744.0, 720.0, 744.0, 744.0, 720.0, 744.0, 720.0, 744.0];
    let mut dh = 0.0;
    for m in 0..12 {
        let delta = (theta_int - climate.theta_e_c[m]).max(0.0);
        dh += delta * hours_per_month[m];
    }
    dh
}

fn transmission_losses_kwh(inputs: &BalancingInputs) -> f64 {
    let dh = degree_hours(&inputs.climate, 19.0);
    inputs.h_t * dh / 1000.0
}

fn ventilation_losses_kwh(inputs: &BalancingInputs) -> f64 {
    let dh = degree_hours(&inputs.climate, 19.0);
    inputs.h_v * dh / 1000.0
}

fn internal_gains_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.internal_gains_w_m2 * inputs.heated_area_m2 * 8760.0 / 1000.0 * 0.35
}

fn net_heating_demand_kwh(inputs: &BalancingInputs) -> f64 {
  let q_t = transmission_losses_kwh(inputs);
  let q_v = ventilation_losses_kwh(inputs);
  let q_i = internal_gains_kwh(inputs);
  let q_s = inputs.solar_gains_kwh;
  (q_t + q_v - q_i - q_s).max(0.0)
}

fn primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
  let q_h = net_heating_demand_kwh(inputs);
  let f_p = 1.1;
  q_h * f_p + inputs.system_losses_kwh - inputs.renewable_kwh
}

part_module!(part_1, "-1", "§5", "general balancing scope", |_| 0.0);
part_module!(part_2, "-2", "§8", "transmission losses Q_T", transmission_losses_kwh);
part_module!(part_3, "-3", "§7", "ventilation losses Q_V", ventilation_losses_kwh);
part_module!(part_4, "-4", "§6", "internal gains Q_I", internal_gains_kwh);
part_module!(part_5, "-5", "§6", "solar gains Q_S", |i: &BalancingInputs| i.solar_gains_kwh);
part_module!(part_6, "-6", "§8", "system losses", |i: &BalancingInputs| i.system_losses_kwh);
part_module!(part_7, "-7", "§9", "net heating demand Q_H", net_heating_demand_kwh);
part_module!(part_8, "-8", "§10", "cooling demand Q_C", |_| 0.0);
part_module!(part_9, "-9", "§11", "DHW demand Q_W", |i: &BalancingInputs| 12.0 * i.heated_area_m2);
part_module!(part_10, "-10", "§12", "primary energy Q_P", primary_energy_kwh);
part_module!(part_11, "-11", "§7", "building automation factor", |_| 0.95);
part_module!(part_12, "-12", "§4", "tabular method reference", primary_energy_kwh);

/// 📋 Full annual balancing per DIN V 18599.
pub fn balance_annual(inputs: &BalancingInputs) -> Result<CheckReport, NormError> {
    let mut report = CheckReport::default();
    report.push(part_2::check(inputs)?);
    report.push(part_3::check(inputs)?);
    report.push(part_7::check(inputs)?);
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

    #[test]
    fn residential_balancing_e2e() {
        let inputs = BalancingInputs::reference_residential(ClimateZoneDe::Zone2, 100.0);
        let report = balance_annual(&inputs).unwrap();
        assert!(!report.checks.is_empty());
        let q_p = primary_energy_kwh(&inputs);
        assert!(q_p > 0.0);
    }
}
