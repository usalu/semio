//! 💰 Utility tariffs and life-cycle costing (non-physics post-pass).

use crate::meters::{FuelType, MeterStore};
use serde::{Deserialize, Serialize};

// #region 🔖Tariff
/// 💰 Time-of-use period.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TouPeriod {
    pub name: String,
    pub start_hour: u8,
    pub end_hour: u8,
    pub months: Vec<u8>,
    pub energy_rate_per_kwh: f64,
    pub demand_rate_per_kw: f64,
}

/// 💰 Utility tariff definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilityTariff {
    pub name: String,
    pub fuel: FuelType,
    pub periods: Vec<TouPeriod>,
    pub fixed_monthly_charge: f64,
    pub ratchet_percent: f64,
}

impl UtilityTariff {
    pub fn energy_cost(&self, energy_kwh: f64, hour: u8, month: u8) -> f64 {
        let rate = self.periods.iter().find(|p| p.months.contains(&month) && hour >= p.start_hour && hour < p.end_hour).map_or(0.1, |p| p.energy_rate_per_kwh);
        energy_kwh * rate
    }
}
// #endregion 🔖Tariff

// #region 🔖Lcca
/// 💰 Life-cycle cost parameters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LccaParameters {
    pub study_period_years: u32,
    pub discount_rate: f64,
    pub inflation_rate: f64,
    pub initial_cost: f64,
    pub annual_maintenance: f64,
    pub replacement_cost: f64,
    pub replacement_interval_years: u32,
}

/// 💰 Life-cycle cost result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LccaResult {
    pub present_value_energy: f64,
    pub present_value_maintenance: f64,
    pub present_value_total: f64,
    pub simple_payback_years: f64,
}

/// 💰 Compute present value of annual cost over study period.
pub fn present_value(annual_cost: f64, discount_rate: f64, years: u32) -> f64 {
    let mut pv = 0.0;
    for y in 1..=years {
        pv += annual_cost / (1.0 + discount_rate).powi(y as i32);
    }
    pv
}

/// 💰 Run LCCA from annual energy cost and parameters.
pub fn compute_lcca(annual_energy_cost: f64, params: &LccaParameters) -> LccaResult {
    let pv_energy = present_value(annual_energy_cost, params.discount_rate, params.study_period_years);
    let pv_maint = present_value(params.annual_maintenance, params.discount_rate, params.study_period_years);
    let pv_total = params.initial_cost + pv_energy + pv_maint;
    let simple_payback = if annual_energy_cost > 0.0 { params.initial_cost / annual_energy_cost } else { f64::INFINITY };
    LccaResult { present_value_energy: pv_energy, present_value_maintenance: pv_maint, present_value_total: pv_total, simple_payback_years: simple_payback }
}
// #endregion 🔖Lcca

// #region 🔖Economics
/// 💰 Economics post-pass over meter results.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EconomicsResult {
    pub annual_energy_cost: f64,
    pub annual_demand_cost: f64,
    pub lcca: Option<LccaResult>,
}

/// 💰 Apply tariffs to meter store (annual run).
pub fn apply_tariffs(meters: &MeterStore, tariffs: &[UtilityTariff]) -> EconomicsResult {
    let mut annual_energy_cost = 0.0;
    let mut annual_demand_cost = 0.0;
    for meter in meters.meters.values() {
        let kwh = meter.energy_kwh();
        if let Some(tariff) = tariffs.iter().find(|t| t.fuel == meter.fuel) {
            annual_energy_cost += tariff.energy_cost(kwh, 12, 7);
            if let Some(period) = tariff.periods.first() {
                annual_demand_cost += meter.peak_demand_w / 1000.0 * period.demand_rate_per_kw;
            }
        }
    }
    EconomicsResult { annual_energy_cost, annual_demand_cost, lcca: None }
}
// #endregion 🔖Economics

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_value_positive() {
        let pv = present_value(1000.0, 0.05, 10);
        assert!(pv > 0.0 && pv < 10_000.0);
    }

    #[test]
    fn lcca_computes_payback() {
        let params = LccaParameters { study_period_years: 20, discount_rate: 0.03, inflation_rate: 0.02, initial_cost: 10_000.0, annual_maintenance: 500.0, replacement_cost: 0.0, replacement_interval_years: 0 };
        let lcca = compute_lcca(2000.0, &params);
        assert!((lcca.simple_payback_years - 5.0).abs() < 1e-6);
    }
}
