//! 🎯 Ideal loads air system: unlimited or capacity-limited zone conditioning.

use crate::props::{humidity_ratio_from_rh, moist_air_enthalpy_j_per_kg, saturation_pressure_pa};
use crate::units::{CP_DRY_AIR, H_FG_0C, P_STD};
use serde::{Deserialize, Serialize};

// #region 🔖IdealLoads
/// 🎯 Ideal loads system configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdealLoadsSystem {
    pub max_heating_w: Option<f64>,
    pub max_cooling_w: Option<f64>,
    pub max_heating_humidification_kg_s: Option<f64>,
    pub max_dehumidification_kg_s: Option<f64>,
    pub outdoor_air_flow_m3_s: f64,
    pub economizer: EconomizerControl,
    pub humidity_control: HumidityControl,
}

/// 🌬️ Economizer control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomizerControl {
    None,
    DifferentialDryBulb,
    DifferentialEnthalpy,
    FixedDryBulb { lockout_c: f64 },
}

/// 💧 Humidity control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumidityControl {
    None,
    HumidifyAndDehumidify,
    HumidifyOnly,
    DehumidifyOnly,
}

/// 📤 Ideal loads delivery result per zone timestep.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdealLoadsOutput {
    pub sensible_heating_w: f64,
    pub sensible_cooling_w: f64,
    pub latent_heating_w: f64,
    pub latent_cooling_w: f64,
    pub outdoor_air_mass_flow_kg_s: f64,
    pub supply_temperature_c: f64,
    pub supply_humidity_ratio: f64,
    pub economizer_active: bool,
    pub humidification_kg_s: f64,
    pub dehumidification_kg_s: f64,
}

/// 🌡️ Zone and outdoor boundary conditions for ideal loads.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdealLoadsRequest {
    pub zone_temperature_c: f64,
    pub zone_humidity_ratio: f64,
    pub heating_setpoint_c: f64,
    pub cooling_setpoint_c: f64,
    pub humidifying_setpoint: f64,
    pub dehumidifying_setpoint: f64,
    pub outdoor_temperature_c: f64,
    pub outdoor_humidity_ratio: f64,
    pub outdoor_pressure_pa: f64,
    pub zone_volume_m3: f64,
    pub infiltration_ach: f64,
}
// #endregion 🔖IdealLoads

// #region 🔖Deliver
/// 🎯 Deliver ideal heating/cooling/humidity to meet zone setpoints.
pub fn ideal_loads_deliver(system: &IdealLoadsSystem, request: &IdealLoadsRequest) -> IdealLoadsOutput {
    let rho = 1.2;
    let oa_m_dot = system.outdoor_air_flow_m3_s * rho;
    let economizer_active = economizer_allows_oa(system, request);
    let effective_oa = if economizer_active { oa_m_dot } else { oa_m_dot.min(0.01) };

    let zone_cp = CP_DRY_AIR;
    let zone_mass = request.zone_volume_m3 * rho;
    let infil_m_dot = zone_mass * request.infiltration_ach / 3600.0;

    let mut sensible_heating = 0.0;
    let mut sensible_cooling = 0.0;
    let mut latent_heating = 0.0;
    let mut latent_cooling = 0.0;

    if request.zone_temperature_c < request.heating_setpoint_c {
        let load = zone_mass * zone_cp * (request.heating_setpoint_c - request.zone_temperature_c) / 3600.0;
        sensible_heating = apply_capacity(load, system.max_heating_w);
    } else if request.zone_temperature_c > request.cooling_setpoint_c {
        let load = zone_mass * zone_cp * (request.zone_temperature_c - request.cooling_setpoint_c) / 3600.0;
        sensible_cooling = apply_capacity(load, system.max_cooling_w);
    }

    let mut w_zone = request.zone_humidity_ratio;
    let mut humidification = 0.0_f64;
    let mut dehumidification = 0.0_f64;

    match system.humidity_control {
        HumidityControl::None => {}
        HumidityControl::HumidifyOnly | HumidityControl::HumidifyAndDehumidify => {
            if w_zone < request.humidifying_setpoint {
                let w_needed = request.humidifying_setpoint - w_zone;
                let m_w = w_needed * zone_mass / 3600.0;
                humidification = apply_capacity(m_w, system.max_heating_humidification_kg_s);
                latent_heating += humidification * H_FG_0C;
                w_zone += humidification * 3600.0 / zone_mass;
            }
        }
        _ => {}
    }

    match system.humidity_control {
        HumidityControl::DehumidifyOnly | HumidityControl::HumidifyAndDehumidify => {
            if w_zone > request.dehumidifying_setpoint {
                let w_remove = w_zone - request.dehumidifying_setpoint;
                let m_w = w_remove * zone_mass / 3600.0;
                dehumidification = apply_capacity(m_w, system.max_dehumidification_kg_s);
                latent_cooling += dehumidification * H_FG_0C;
                w_zone -= dehumidification * 3600.0 / zone_mass;
            }
        }
        _ => {}
    }

    let oa_effect = effective_oa + infil_m_dot;
    let supply_t = if sensible_heating > 0.0 {
        request.heating_setpoint_c
    } else if sensible_cooling > 0.0 {
        request.cooling_setpoint_c
    } else {
        request.zone_temperature_c
    };
    let supply_w = if humidification > 0.0 {
        request.humidifying_setpoint
    } else if dehumidification > 0.0 {
        request.dehumidifying_setpoint
    } else if economizer_active {
        request.outdoor_humidity_ratio
    } else {
        w_zone
    };

    let _ = (oa_effect, supply_t, supply_w);

    IdealLoadsOutput {
        sensible_heating_w: sensible_heating,
        sensible_cooling_w: sensible_cooling,
        latent_heating_w: latent_heating,
        latent_cooling_w: latent_cooling,
        outdoor_air_mass_flow_kg_s: effective_oa,
        supply_temperature_c: supply_t,
        supply_humidity_ratio: supply_w,
        economizer_active,
        humidification_kg_s: humidification,
        dehumidification_kg_s: dehumidification,
    }
}

fn apply_capacity(load: f64, cap: Option<f64>) -> f64 {
    match cap {
        Some(c) => load.min(c),
        None => load,
    }
}

fn economizer_allows_oa(system: &IdealLoadsSystem, req: &IdealLoadsRequest) -> bool {
    match system.economizer {
        EconomizerControl::None => false,
        EconomizerControl::DifferentialDryBulb => req.outdoor_temperature_c < req.zone_temperature_c,
        EconomizerControl::DifferentialEnthalpy => {
            let h_oa = moist_air_enthalpy_j_per_kg(req.outdoor_temperature_c, req.outdoor_humidity_ratio);
            let h_zone = moist_air_enthalpy_j_per_kg(req.zone_temperature_c, req.zone_humidity_ratio);
            h_oa < h_zone
        }
        EconomizerControl::FixedDryBulb { lockout_c } => req.outdoor_temperature_c < lockout_c,
    }
}
// #endregion 🔖Deliver

#[cfg(test)]
mod tests {
    use super::*;

    fn unlimited_system() -> IdealLoadsSystem {
        IdealLoadsSystem {
            max_heating_w: None,
            max_cooling_w: None,
            max_heating_humidification_kg_s: None,
            max_dehumidification_kg_s: None,
            outdoor_air_flow_m3_s: 0.05,
            economizer: EconomizerControl::DifferentialDryBulb,
            humidity_control: HumidityControl::HumidifyAndDehumidify,
        }
    }

    #[test]
    fn heating_when_below_setpoint() {
        let system = unlimited_system();
        let req = IdealLoadsRequest {
            zone_temperature_c: 18.0,
            zone_humidity_ratio: 0.008,
            heating_setpoint_c: 21.0,
            cooling_setpoint_c: 24.0,
            humidifying_setpoint: 0.008,
            dehumidifying_setpoint: 0.012,
            outdoor_temperature_c: 5.0,
            outdoor_humidity_ratio: humidity_ratio_from_rh(5.0, 0.8, P_STD),
            outdoor_pressure_pa: P_STD,
            zone_volume_m3: 100.0,
            infiltration_ach: 0.5,
        };
        let out = ideal_loads_deliver(&system, &req);
        assert!(out.sensible_heating_w > 0.0);
        assert_eq!(out.sensible_cooling_w, 0.0);
    }

    #[test]
    fn capacity_limits_cooling() {
        let system = IdealLoadsSystem {
            max_heating_w: None,
            max_cooling_w: Some(1000.0),
            max_heating_humidification_kg_s: None,
            max_dehumidification_kg_s: None,
            outdoor_air_flow_m3_s: 0.0,
            economizer: EconomizerControl::None,
            humidity_control: HumidityControl::None,
        };
        let req = IdealLoadsRequest {
            zone_temperature_c: 30.0,
            zone_humidity_ratio: 0.01,
            heating_setpoint_c: 21.0,
            cooling_setpoint_c: 24.0,
            humidifying_setpoint: 0.008,
            dehumidifying_setpoint: 0.012,
            outdoor_temperature_c: 35.0,
            outdoor_humidity_ratio: 0.015,
            outdoor_pressure_pa: P_STD,
            zone_volume_m3: 500.0,
            infiltration_ach: 0.0,
        };
        let out = ideal_loads_deliver(&system, &req);
        assert!((out.sensible_cooling_w - 1000.0).abs() < 1e-6);
    }

    #[test]
    fn economizer_active_when_oa_cooler() {
        let system = unlimited_system();
        let req = IdealLoadsRequest {
            zone_temperature_c: 25.0,
            zone_humidity_ratio: 0.01,
            heating_setpoint_c: 21.0,
            cooling_setpoint_c: 24.0,
            humidifying_setpoint: 0.008,
            dehumidifying_setpoint: 0.012,
            outdoor_temperature_c: 15.0,
            outdoor_humidity_ratio: 0.006,
            outdoor_pressure_pa: P_STD,
            zone_volume_m3: 100.0,
            infiltration_ach: 0.0,
        };
        let out = ideal_loads_deliver(&system, &req);
        assert!(out.economizer_active);
    }
}
