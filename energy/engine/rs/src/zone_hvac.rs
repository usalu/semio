//! 🏠 Zone equipment catalog: baseboards, radiant, fan coils, PTAC, VRF, ERV.

use crate::coils::{cooling_coil_output_w, heating_coil_output_w, CoilAirState, CoolingCoil, HeatingCoil};
use crate::fans::{fan_mass_flow_kg_s, fan_operating_point, fan_power_w, Fan};
use crate::heat_recovery::{heat_recovery_exchange_w, HeatRecoveryUnit, HxAirstream};
use crate::units::{CP_DRY_AIR, RHO_AIR_REF};
use serde::{Deserialize, Serialize};

// #region 🔖ZoneEquipment
/// 🏠 Zone-level HVAC equipment catalog.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ZoneEquipment {
    Baseboard { heating: HeatingCoil },
    Radiant { heating: HeatingCoil, cooling: Option<CoolingCoil>, surface_area_m2: f64 },
    FanCoil { heating: Option<HeatingCoil>, cooling: Option<CoolingCoil>, fan: Fan, max_flow_m3_s: f64 },
    Ptac { heating: HeatingCoil, cooling: CoolingCoil, fan: Fan, oa_fraction: f64 },
    VrfTerminal { heating_cap_w: f64, cooling_cap_w: f64, cop_heating: f64, cop_cooling: f64 },
    Erv { unit: HeatRecoveryUnit, supply_fan: Fan, exhaust_fan: Fan },
    UnitHeater { heating: HeatingCoil, fan: Fan },
    WaterToAirHp { heating_cap_w: f64, cooling_cap_w: f64, cop_heating: f64, cop_cooling: f64 },
}

/// 📥 Zone equipment simulation request.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneEquipmentRequest {
    pub zone_temperature_c: f64,
    pub zone_humidity_ratio: f64,
    pub heating_load_w: f64,
    pub cooling_load_w: f64,
    pub outdoor_temperature_c: f64,
    pub outdoor_humidity_ratio: f64,
    pub outdoor_pressure_pa: f64,
    pub supply_air_temp_c: f64,
    pub supply_air_humidity_ratio: f64,
    pub supply_mass_flow_kg_s: f64,
}

/// 📤 Zone equipment simulation result.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneEquipmentOutput {
    pub delivered_heating_w: f64,
    pub delivered_cooling_w: f64,
    pub supply_temperature_c: f64,
    pub supply_humidity_ratio: f64,
    pub supply_mass_flow_kg_s: f64,
    pub fan_power_w: f64,
    pub compressor_power_w: f64,
    pub gas_consumption_w: f64,
}
// #endregion 🔖ZoneEquipment

// #region 🔖Simulate
impl ZoneEquipment {
    /// 🏠 Simulate zone equipment for one timestep.
    pub fn simulate(&self, request: &ZoneEquipmentRequest) -> ZoneEquipmentOutput {
        match self {
            ZoneEquipment::Baseboard { heating } => {
                let inlet = CoilAirState { temperature_c: request.zone_temperature_c, humidity_ratio: request.zone_humidity_ratio, mass_flow_kg_s: 0.1, pressure_pa: request.outdoor_pressure_pa };
                let out = heating_coil_output_w(heating, &inlet, request.heating_load_w);
                ZoneEquipmentOutput {
                    delivered_heating_w: out.total_heating_w,
                    delivered_cooling_w: 0.0,
                    supply_temperature_c: request.zone_temperature_c,
                    supply_humidity_ratio: request.zone_humidity_ratio,
                    supply_mass_flow_kg_s: 0.0,
                    fan_power_w: 0.0,
                    compressor_power_w: 0.0,
                    gas_consumption_w: out.gas_consumption_w,
                }
            }
            ZoneEquipment::Radiant { heating, cooling, surface_area_m2 } => {
                let q_rad_factor = surface_area_m2 / 10.0;
                let inlet = CoilAirState { temperature_c: request.zone_temperature_c, humidity_ratio: request.zone_humidity_ratio, mass_flow_kg_s: 0.05, pressure_pa: request.outdoor_pressure_pa };
                let heat = heating_coil_output_w(heating, &inlet, request.heating_load_w);
                let cool = cooling.as_ref().map_or(crate::coils::CoolingCoilOutput { outlet: inlet, total_cooling_w: 0.0, sensible_cooling_w: 0.0, latent_cooling_w: 0.0, compressor_power_w: 0.0, condensate_kg_s: 0.0 }, |c| {
                    cooling_coil_output_w(c, &inlet, request.cooling_load_w, 0.05)
                });
                ZoneEquipmentOutput {
                    delivered_heating_w: heat.total_heating_w * q_rad_factor.min(1.0),
                    delivered_cooling_w: cool.total_cooling_w * q_rad_factor.min(1.0),
                    supply_temperature_c: request.zone_temperature_c,
                    supply_humidity_ratio: request.zone_humidity_ratio,
                    supply_mass_flow_kg_s: 0.0,
                    fan_power_w: 0.0,
                    compressor_power_w: cool.compressor_power_w,
                    gas_consumption_w: heat.gas_consumption_w,
                }
            }
            ZoneEquipment::FanCoil { heating, cooling, fan, max_flow_m3_s } => {
                let flow = max_flow_m3_s.min(request.supply_mass_flow_kg_s / RHO_AIR_REF.max(0.5));
                let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
                let inlet = CoilAirState { temperature_c: request.supply_air_temp_c, humidity_ratio: request.supply_air_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.outdoor_pressure_pa };
                let heat = heating.as_ref().map_or(crate::coils::HeatingCoilOutput { outlet: inlet, total_heating_w: 0.0, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 }, |h| heating_coil_output_w(h, &inlet, request.heating_load_w));
                let cool_inlet = heat.outlet;
                let cool = cooling.as_ref().map_or(crate::coils::CoolingCoilOutput { outlet: cool_inlet, total_cooling_w: 0.0, sensible_cooling_w: 0.0, latent_cooling_w: 0.0, compressor_power_w: 0.0, condensate_kg_s: 0.0 }, |c| {
                    cooling_coil_output_w(c, &cool_inlet, request.cooling_load_w, 0.08)
                });
                let op = fan_operating_point(fan, flow, 300.0);
                ZoneEquipmentOutput {
                    delivered_heating_w: heat.total_heating_w,
                    delivered_cooling_w: cool.total_cooling_w,
                    supply_temperature_c: cool.outlet.temperature_c,
                    supply_humidity_ratio: cool.outlet.humidity_ratio,
                    supply_mass_flow_kg_s: m_dot,
                    fan_power_w: fan_power_w(fan, &op),
                    compressor_power_w: cool.compressor_power_w,
                    gas_consumption_w: heat.gas_consumption_w,
                }
            }
            ZoneEquipment::Ptac { heating, cooling, fan, oa_fraction } => {
                let oa_m_dot = request.supply_mass_flow_kg_s * oa_fraction;
                let ra_m_dot = request.supply_mass_flow_kg_s - oa_m_dot;
                let t_mix = (oa_m_dot * request.outdoor_temperature_c + ra_m_dot * request.zone_temperature_c) / request.supply_mass_flow_kg_s.max(1e-6);
                let w_mix = (oa_m_dot * request.outdoor_humidity_ratio + ra_m_dot * request.zone_humidity_ratio) / request.supply_mass_flow_kg_s.max(1e-6);
                let inlet = CoilAirState { temperature_c: t_mix, humidity_ratio: w_mix, mass_flow_kg_s: request.supply_mass_flow_kg_s, pressure_pa: request.outdoor_pressure_pa };
                let heat = heating_coil_output_w(heating, &inlet, request.heating_load_w);
                let cool = cooling_coil_output_w(cooling, &heat.outlet, request.cooling_load_w, 0.1);
                let op = fan_operating_point(fan, request.supply_mass_flow_kg_s / RHO_AIR_REF, 250.0);
                ZoneEquipmentOutput {
                    delivered_heating_w: heat.total_heating_w,
                    delivered_cooling_w: cool.total_cooling_w,
                    supply_temperature_c: cool.outlet.temperature_c,
                    supply_humidity_ratio: cool.outlet.humidity_ratio,
                    supply_mass_flow_kg_s: request.supply_mass_flow_kg_s,
                    fan_power_w: fan_power_w(fan, &op),
                    compressor_power_w: cool.compressor_power_w,
                    gas_consumption_w: heat.gas_consumption_w,
                }
            }
            ZoneEquipment::VrfTerminal { heating_cap_w, cooling_cap_w, cop_heating, cop_cooling } => {
                let q_heat = request.heating_load_w.min(*heating_cap_w);
                let q_cool = request.cooling_load_w.min(*cooling_cap_w);
                let comp = q_heat / cop_heating.max(0.5) + q_cool / cop_cooling.max(0.5);
                ZoneEquipmentOutput {
                    delivered_heating_w: q_heat,
                    delivered_cooling_w: q_cool,
                    supply_temperature_c: request.supply_air_temp_c,
                    supply_humidity_ratio: request.supply_air_humidity_ratio,
                    supply_mass_flow_kg_s: request.supply_mass_flow_kg_s,
                    fan_power_w: 50.0,
                    compressor_power_w: comp,
                    gas_consumption_w: 0.0,
                }
            }
            ZoneEquipment::Erv { unit, supply_fan, exhaust_fan } => {
                let m_dot = request.supply_mass_flow_kg_s;
                let supply = HxAirstream { temperature_c: request.outdoor_temperature_c, humidity_ratio: request.outdoor_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.outdoor_pressure_pa };
                let exhaust = HxAirstream { temperature_c: request.zone_temperature_c, humidity_ratio: request.zone_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.outdoor_pressure_pa };
                let hx = heat_recovery_exchange_w(unit, &supply, &exhaust);
                let sup_op = fan_operating_point(supply_fan, m_dot / RHO_AIR_REF, 200.0);
                let exh_op = fan_operating_point(exhaust_fan, m_dot / RHO_AIR_REF, 200.0);
                ZoneEquipmentOutput {
                    delivered_heating_w: hx.sensible_recovery_w.max(0.0),
                    delivered_cooling_w: (-hx.sensible_recovery_w).max(0.0),
                    supply_temperature_c: hx.supply_out.temperature_c,
                    supply_humidity_ratio: hx.supply_out.humidity_ratio,
                    supply_mass_flow_kg_s: m_dot,
                    fan_power_w: fan_power_w(supply_fan, &sup_op) + fan_power_w(exhaust_fan, &exh_op) + hx.defrost_power_w,
                    compressor_power_w: 0.0,
                    gas_consumption_w: 0.0,
                }
            }
            ZoneEquipment::UnitHeater { heating, fan } => {
                let m_dot = request.supply_mass_flow_kg_s.max(0.2);
                let inlet = CoilAirState { temperature_c: request.zone_temperature_c, humidity_ratio: request.zone_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.outdoor_pressure_pa };
                let heat = heating_coil_output_w(heating, &inlet, request.heating_load_w);
                let op = fan_operating_point(fan, m_dot / RHO_AIR_REF, 150.0);
                ZoneEquipmentOutput {
                    delivered_heating_w: heat.total_heating_w,
                    delivered_cooling_w: 0.0,
                    supply_temperature_c: heat.outlet.temperature_c,
                    supply_humidity_ratio: heat.outlet.humidity_ratio,
                    supply_mass_flow_kg_s: m_dot,
                    fan_power_w: fan_power_w(fan, &op),
                    compressor_power_w: 0.0,
                    gas_consumption_w: heat.gas_consumption_w,
                }
            }
            ZoneEquipment::WaterToAirHp { heating_cap_w, cooling_cap_w, cop_heating, cop_cooling } => {
                let q_heat = request.heating_load_w.min(*heating_cap_w);
                let q_cool = request.cooling_load_w.min(*cooling_cap_w);
                let t_out = if q_heat > q_cool { request.zone_temperature_c + q_heat / (request.supply_mass_flow_kg_s.max(0.1) * CP_DRY_AIR) } else { request.zone_temperature_c - q_cool / (request.supply_mass_flow_kg_s.max(0.1) * CP_DRY_AIR) };
                ZoneEquipmentOutput {
                    delivered_heating_w: q_heat,
                    delivered_cooling_w: q_cool,
                    supply_temperature_c: t_out,
                    supply_humidity_ratio: request.zone_humidity_ratio,
                    supply_mass_flow_kg_s: request.supply_mass_flow_kg_s,
                    fan_power_w: 80.0,
                    compressor_power_w: q_heat / cop_heating.max(0.5) + q_cool / cop_cooling.max(0.5),
                    gas_consumption_w: 0.0,
                }
            }
        }
    }
}
// #endregion 🔖Simulate

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::PerformanceCurve;
    use crate::fans::FanType;
    use crate::units::P_STD;

    #[test]
    fn baseboard_delivers_heat() {
        let eq = ZoneEquipment::Baseboard { heating: HeatingCoil::Electric { capacity_w: 5000.0, efficiency: 1.0 } };
        let req = ZoneEquipmentRequest {
            zone_temperature_c: 18.0,
            zone_humidity_ratio: 0.008,
            heating_load_w: 3000.0,
            cooling_load_w: 0.0,
            outdoor_temperature_c: 5.0,
            outdoor_humidity_ratio: 0.005,
            outdoor_pressure_pa: P_STD,
            supply_air_temp_c: 20.0,
            supply_air_humidity_ratio: 0.008,
            supply_mass_flow_kg_s: 0.0,
        };
        let out = eq.simulate(&req);
        assert!(out.delivered_heating_w > 0.0);
    }

    #[test]
    fn vrf_respects_capacity() {
        let eq = ZoneEquipment::VrfTerminal { heating_cap_w: 2000.0, cooling_cap_w: 2500.0, cop_heating: 3.5, cop_cooling: 3.0 };
        let req = ZoneEquipmentRequest {
            zone_temperature_c: 24.0,
            zone_humidity_ratio: 0.01,
            heating_load_w: 0.0,
            cooling_load_w: 5000.0,
            outdoor_temperature_c: 32.0,
            outdoor_humidity_ratio: 0.015,
            outdoor_pressure_pa: P_STD,
            supply_air_temp_c: 16.0,
            supply_air_humidity_ratio: 0.009,
            supply_mass_flow_kg_s: 0.3,
        };
        let out = eq.simulate(&req);
        assert!((out.delivered_cooling_w - 2500.0).abs() < 1.0);
        assert!(out.compressor_power_w > 0.0);
    }

    #[test]
    fn fan_coil_runs_coils_and_fan() {
        let eq = ZoneEquipment::FanCoil {
            heating: None,
            cooling: Some(CoolingCoil::DxSingleSpeed { rated_capacity_w: 8000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) }),
            fan: Fan {
                fan_type: FanType::OnOff,
                max_flow_m3_s: 0.4,
                max_pressure_rise_pa: 400.0,
                motor_efficiency: 0.85,
                pressure_curve: PerformanceCurve::Constant(1.0),
                efficiency_curve: PerformanceCurve::Constant(0.6),
                part_load_curve: PerformanceCurve::Constant(1.0),
            },
            max_flow_m3_s: 0.35,
        };
        let req = ZoneEquipmentRequest {
            zone_temperature_c: 26.0,
            zone_humidity_ratio: 0.012,
            heating_load_w: 0.0,
            cooling_load_w: 4000.0,
            outdoor_temperature_c: 30.0,
            outdoor_humidity_ratio: 0.014,
            outdoor_pressure_pa: P_STD,
            supply_air_temp_c: 18.0,
            supply_air_humidity_ratio: 0.009,
            supply_mass_flow_kg_s: 0.35,
        };
        let out = eq.simulate(&req);
        assert!(out.delivered_cooling_w > 0.0);
        assert!(out.fan_power_w > 0.0);
    }
}
