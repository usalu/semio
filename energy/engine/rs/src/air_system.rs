//! 🏭 Air systems: CAV, VAV, dual-duct, multizone, DOAS, and VRF with OA mixing.

use crate::coils::{cooling_coil_output_w, heating_coil_output_w, CoilAirState, CoolingCoil, HeatingCoil};
use crate::fans::{fan_mass_flow_kg_s, fan_operating_point, fan_power_w, Fan};
use crate::ideal_hvac::EconomizerControl;
use crate::props::moist_air_enthalpy_j_per_kg;
use crate::terminal::{AirTerminal, TerminalRequest};
use crate::units::RHO_AIR_REF;
use serde::{Deserialize, Serialize};

// #region 🔖AirSystem
/// 🏭 Central air system configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AirSystem {
    Cav { supply_fan: Fan, return_fan: Option<Fan>, cooling: CoolingCoil, heating: Option<HeatingCoil>, design_flow_m3_s: f64 },
    Vav { supply_fan: Fan, return_fan: Option<Fan>, cooling: CoolingCoil, heating: Option<HeatingCoil>, min_flow_m3_s: f64, max_flow_m3_s: f64 },
    DualDuct { hot_fan: Fan, cold_fan: Fan, hot_heating: HeatingCoil, cold_cooling: CoolingCoil },
    Multizone { supply_fan: Fan, cooling: CoolingCoil, zone_dampers: Vec<f64> },
    Doas { supply_fan: Fan, cooling: CoolingCoil, heating: Option<HeatingCoil>, erv_effectiveness: f64, design_oa_m3_s: f64 },
    Vrf { outdoor_unit_cap_w: f64, cop_cooling: f64, cop_heating: f64, num_terminals: u32 },
}

/// 📥 Air system simulation boundary conditions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AirSystemRequest {
    pub outdoor_temperature_c: f64,
    pub outdoor_humidity_ratio: f64,
    pub outdoor_pressure_pa: f64,
    pub return_temperature_c: f64,
    pub return_humidity_ratio: f64,
    pub total_cooling_load_w: f64,
    pub total_heating_load_w: f64,
    pub oa_fraction: f64,
    pub economizer: EconomizerControl,
    pub zone_terminals: Vec<(AirTerminal, TerminalRequest)>,
    pub requested_supply_flow_m3_s: f64,
}

/// 📤 Air system simulation result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AirSystemOutput {
    pub supply_temperature_c: f64,
    pub supply_humidity_ratio: f64,
    pub supply_mass_flow_kg_s: f64,
    pub outdoor_air_mass_flow_kg_s: f64,
    pub mixed_air_temperature_c: f64,
    pub mixed_air_humidity_ratio: f64,
    pub total_cooling_w: f64,
    pub total_heating_w: f64,
    pub supply_fan_power_w: f64,
    pub return_fan_power_w: f64,
    pub compressor_power_w: f64,
    pub economizer_active: bool,
    pub terminal_outputs: Vec<crate::terminal::TerminalOutput>,
}
// #endregion 🔖AirSystem

// #region 🔖Simulate
/// 🏭 Simulate central air system including OA mixing, coils, fans, and terminals.
pub fn simulate_air_system(system: &AirSystem, request: &AirSystemRequest) -> AirSystemOutput {
    let oa_frac = request.oa_fraction.clamp(0.0, 1.0);
    let economizer_active = economizer_active(request);
    let effective_oa_frac = if economizer_active { oa_frac.max(0.2) } else { oa_frac };

    let return_m_dot = fan_mass_flow_kg_s(request.requested_supply_flow_m3_s, RHO_AIR_REF);
    let oa_m_dot = return_m_dot * effective_oa_frac;
    let ra_m_dot = return_m_dot - oa_m_dot;

    let mixed_t = (oa_m_dot * request.outdoor_temperature_c + ra_m_dot * request.return_temperature_c) / return_m_dot.max(1e-6);
    let mixed_w = (oa_m_dot * request.outdoor_humidity_ratio + ra_m_dot * request.return_humidity_ratio) / return_m_dot.max(1e-6);

    match system {
        AirSystem::Cav { supply_fan, return_fan, cooling, heating, design_flow_m3_s } => {
            let flow = *design_flow_m3_s;
            let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
            let coil_result = run_coils(cooling, heating.as_ref(), mixed_t, mixed_w, m_dot, request, request.outdoor_pressure_pa);
            let sup_op = fan_operating_point(supply_fan, flow, 500.0);
            let ret_power = return_fan.as_ref().map_or(0.0, |f| {
                let op = fan_operating_point(f, flow * 0.9, 300.0);
                fan_power_w(f, &op)
            });
            let terminals = simulate_terminals(&request.zone_terminals, coil_result.outlet.temperature_c, coil_result.outlet.humidity_ratio);
            AirSystemOutput {
                supply_temperature_c: coil_result.outlet.temperature_c,
                supply_humidity_ratio: coil_result.outlet.humidity_ratio,
                supply_mass_flow_kg_s: m_dot,
                outdoor_air_mass_flow_kg_s: oa_m_dot,
                mixed_air_temperature_c: mixed_t,
                mixed_air_humidity_ratio: mixed_w,
                total_cooling_w: coil_result.cooling_w,
                total_heating_w: coil_result.heating_w,
                supply_fan_power_w: fan_power_w(supply_fan, &sup_op),
                return_fan_power_w: ret_power,
                compressor_power_w: coil_result.compressor_w,
                economizer_active,
                terminal_outputs: terminals,
            }
        }
        AirSystem::Vav { supply_fan, return_fan, cooling, heating, min_flow_m3_s, max_flow_m3_s } => {
            let load_frac = (request.total_cooling_load_w / 50_000.0).clamp(0.0, 1.0);
            let flow = min_flow_m3_s + load_frac * (max_flow_m3_s - min_flow_m3_s);
            let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
            let coil_result = run_coils(cooling, heating.as_ref(), mixed_t, mixed_w, m_dot, request, request.outdoor_pressure_pa);
            let sup_op = fan_operating_point(supply_fan, flow, 600.0);
            let ret_power = return_fan.as_ref().map_or(0.0, |f| fan_power_w(f, &fan_operating_point(f, flow * 0.85, 350.0)));
            let terminals = simulate_terminals(&request.zone_terminals, coil_result.outlet.temperature_c, coil_result.outlet.humidity_ratio);
            AirSystemOutput {
                supply_temperature_c: coil_result.outlet.temperature_c,
                supply_humidity_ratio: coil_result.outlet.humidity_ratio,
                supply_mass_flow_kg_s: m_dot,
                outdoor_air_mass_flow_kg_s: oa_m_dot,
                mixed_air_temperature_c: mixed_t,
                mixed_air_humidity_ratio: mixed_w,
                total_cooling_w: coil_result.cooling_w,
                total_heating_w: coil_result.heating_w,
                supply_fan_power_w: fan_power_w(supply_fan, &sup_op),
                return_fan_power_w: ret_power,
                compressor_power_w: coil_result.compressor_w,
                economizer_active,
                terminal_outputs: terminals,
            }
        }
        AirSystem::DualDuct { hot_fan, cold_fan, hot_heating, cold_cooling } => {
            let hot_flow = request.requested_supply_flow_m3_s * 0.4;
            let cold_flow = request.requested_supply_flow_m3_s * 0.6;
            let hot_m = fan_mass_flow_kg_s(hot_flow, RHO_AIR_REF);
            let cold_m = fan_mass_flow_kg_s(cold_flow, RHO_AIR_REF);
            let hot_in = CoilAirState { temperature_c: mixed_t, humidity_ratio: mixed_w, mass_flow_kg_s: hot_m, pressure_pa: request.outdoor_pressure_pa };
            let cold_in = hot_in;
            let hot_out = heating_coil_output_w(hot_heating, &hot_in, request.total_heating_load_w);
            let cold_out = cooling_coil_output_w(cold_cooling, &cold_in, request.total_cooling_load_w, 0.08);
            let total_m = hot_m + cold_m;
            let t_sup = (hot_m * hot_out.outlet.temperature_c + cold_m * cold_out.outlet.temperature_c) / total_m.max(1e-6);
            let w_sup = (hot_m * hot_out.outlet.humidity_ratio + cold_m * cold_out.outlet.humidity_ratio) / total_m.max(1e-6);
            let sup_power = fan_power_w(hot_fan, &fan_operating_point(hot_fan, hot_flow, 400.0)) + fan_power_w(cold_fan, &fan_operating_point(cold_fan, cold_flow, 400.0));
            let terminals = simulate_terminals(&request.zone_terminals, t_sup, w_sup);
            AirSystemOutput {
                supply_temperature_c: t_sup,
                supply_humidity_ratio: w_sup,
                supply_mass_flow_kg_s: total_m,
                outdoor_air_mass_flow_kg_s: oa_m_dot,
                mixed_air_temperature_c: mixed_t,
                mixed_air_humidity_ratio: mixed_w,
                total_cooling_w: cold_out.total_cooling_w,
                total_heating_w: hot_out.total_heating_w,
                supply_fan_power_w: sup_power,
                return_fan_power_w: 0.0,
                compressor_power_w: cold_out.compressor_power_w,
                economizer_active,
                terminal_outputs: terminals,
            }
        }
        AirSystem::Multizone { supply_fan, cooling, zone_dampers } => {
            let total_flow = request.requested_supply_flow_m3_s;
            let m_dot = fan_mass_flow_kg_s(total_flow, RHO_AIR_REF);
            let coil_result = run_coils(cooling, None, mixed_t, mixed_w, m_dot, request, request.outdoor_pressure_pa);
            let sup_op = fan_operating_point(supply_fan, total_flow, 550.0);
            let terminals = simulate_terminals(&request.zone_terminals, coil_result.outlet.temperature_c, coil_result.outlet.humidity_ratio);
            let _ = zone_dampers;
            AirSystemOutput {
                supply_temperature_c: coil_result.outlet.temperature_c,
                supply_humidity_ratio: coil_result.outlet.humidity_ratio,
                supply_mass_flow_kg_s: m_dot,
                outdoor_air_mass_flow_kg_s: oa_m_dot,
                mixed_air_temperature_c: mixed_t,
                mixed_air_humidity_ratio: mixed_w,
                total_cooling_w: coil_result.cooling_w,
                total_heating_w: coil_result.heating_w,
                supply_fan_power_w: fan_power_w(supply_fan, &sup_op),
                return_fan_power_w: 0.0,
                compressor_power_w: coil_result.compressor_w,
                economizer_active,
                terminal_outputs: terminals,
            }
        }
        AirSystem::Doas { supply_fan, cooling, heating, erv_effectiveness, design_oa_m3_s } => {
            let oa_flow = design_oa_m3_s.min(request.requested_supply_flow_m3_s);
            let m_dot = fan_mass_flow_kg_s(oa_flow, RHO_AIR_REF);
            let t_erv = request.outdoor_temperature_c + erv_effectiveness * (request.return_temperature_c - request.outdoor_temperature_c);
            let w_erv = request.outdoor_humidity_ratio + erv_effectiveness * (request.return_humidity_ratio - request.outdoor_humidity_ratio);
            let coil_result = run_coils(cooling, heating.as_ref(), t_erv, w_erv, m_dot, request, request.outdoor_pressure_pa);
            let sup_op = fan_operating_point(supply_fan, oa_flow, 450.0);
            let terminals = simulate_terminals(&request.zone_terminals, coil_result.outlet.temperature_c, coil_result.outlet.humidity_ratio);
            AirSystemOutput {
                supply_temperature_c: coil_result.outlet.temperature_c,
                supply_humidity_ratio: coil_result.outlet.humidity_ratio,
                supply_mass_flow_kg_s: m_dot,
                outdoor_air_mass_flow_kg_s: m_dot,
                mixed_air_temperature_c: t_erv,
                mixed_air_humidity_ratio: w_erv,
                total_cooling_w: coil_result.cooling_w,
                total_heating_w: coil_result.heating_w,
                supply_fan_power_w: fan_power_w(supply_fan, &sup_op),
                return_fan_power_w: 0.0,
                compressor_power_w: coil_result.compressor_w,
                economizer_active: true,
                terminal_outputs: terminals,
            }
        }
        AirSystem::Vrf { outdoor_unit_cap_w, cop_cooling, cop_heating, .. } => {
            let q_cool = request.total_cooling_load_w.min(*outdoor_unit_cap_w);
            let q_heat = request.total_heating_load_w.min(*outdoor_unit_cap_w);
            let comp = q_cool / cop_cooling.max(0.5) + q_heat / cop_heating.max(0.5);
            let terminals = simulate_terminals(&request.zone_terminals, request.return_temperature_c, request.return_humidity_ratio);
            AirSystemOutput {
                supply_temperature_c: request.return_temperature_c,
                supply_humidity_ratio: request.return_humidity_ratio,
                supply_mass_flow_kg_s: return_m_dot,
                outdoor_air_mass_flow_kg_s: oa_m_dot,
                mixed_air_temperature_c: mixed_t,
                mixed_air_humidity_ratio: mixed_w,
                total_cooling_w: q_cool,
                total_heating_w: q_heat,
                supply_fan_power_w: 0.0,
                return_fan_power_w: 0.0,
                compressor_power_w: comp,
                economizer_active,
                terminal_outputs: terminals,
            }
        }
    }
}
// #endregion 🔖Simulate

// #region 🔖Helpers
struct CoilRunResult {
    outlet: CoilAirState,
    cooling_w: f64,
    heating_w: f64,
    compressor_w: f64,
}

fn run_coils(cooling: &CoolingCoil, heating: Option<&HeatingCoil>, mixed_t: f64, mixed_w: f64, m_dot: f64, request: &AirSystemRequest, pressure_pa: f64) -> CoilRunResult {
    let inlet = CoilAirState { temperature_c: mixed_t, humidity_ratio: mixed_w, mass_flow_kg_s: m_dot, pressure_pa };
    let cool = cooling_coil_output_w(cooling, &inlet, request.total_cooling_load_w, 0.08);
    let heat = heating.map_or(crate::coils::HeatingCoilOutput { outlet: cool.outlet, total_heating_w: 0.0, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 }, |h| heating_coil_output_w(h, &cool.outlet, request.total_heating_load_w));
    CoilRunResult { outlet: heat.outlet, cooling_w: cool.total_cooling_w, heating_w: heat.total_heating_w, compressor_w: cool.compressor_power_w }
}

fn simulate_terminals(terminals: &[(AirTerminal, TerminalRequest)], supply_t: f64, supply_w: f64) -> Vec<crate::terminal::TerminalOutput> {
    terminals
        .iter()
        .map(|(term, req)| {
            let mut r = *req;
            r.supply_temperature_c = supply_t;
            r.supply_humidity_ratio = supply_w;
            term.simulate(&r)
        })
        .collect()
}

fn economizer_active(request: &AirSystemRequest) -> bool {
    match request.economizer {
        EconomizerControl::None => false,
        EconomizerControl::DifferentialDryBulb => request.outdoor_temperature_c < request.return_temperature_c,
        EconomizerControl::DifferentialEnthalpy => {
            let h_oa = moist_air_enthalpy_j_per_kg(request.outdoor_temperature_c, request.outdoor_humidity_ratio);
            let h_ra = moist_air_enthalpy_j_per_kg(request.return_temperature_c, request.return_humidity_ratio);
            h_oa < h_ra
        }
        EconomizerControl::FixedDryBulb { lockout_c } => request.outdoor_temperature_c < lockout_c,
    }
}
// #endregion 🔖Helpers

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::PerformanceCurve;
    use crate::fans::FanType;
    use crate::props::humidity_ratio_from_rh;
    use crate::units::P_STD;

    fn test_cooling() -> CoolingCoil {
        CoolingCoil::DxSingleSpeed { rated_capacity_w: 50_000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) }
    }

    fn test_fan() -> Fan {
        Fan {
            fan_type: FanType::VariableVolume,
            max_flow_m3_s: 5.0,
            max_pressure_rise_pa: 800.0,
            motor_efficiency: 0.9,
            pressure_curve: PerformanceCurve::Constant(1.0),
            efficiency_curve: PerformanceCurve::Constant(0.65),
            part_load_curve: PerformanceCurve::Cubic { coeffs: [0.0, 0.2, 0.5, 0.3] },
        }
    }

    #[test]
    fn vav_system_cools_mixed_air() {
        let system = AirSystem::Vav { supply_fan: test_fan(), return_fan: None, cooling: test_cooling(), heating: None, min_flow_m3_s: 1.0, max_flow_m3_s: 4.0 };
        let req = AirSystemRequest {
            outdoor_temperature_c: 32.0,
            outdoor_humidity_ratio: humidity_ratio_from_rh(32.0, 0.5, P_STD),
            outdoor_pressure_pa: P_STD,
            return_temperature_c: 24.0,
            return_humidity_ratio: 0.01,
            total_cooling_load_w: 30_000.0,
            total_heating_load_w: 0.0,
            oa_fraction: 0.2,
            economizer: EconomizerControl::None,
            zone_terminals: vec![],
            requested_supply_flow_m3_s: 3.0,
        };
        let out = simulate_air_system(&system, &req);
        assert!(out.supply_temperature_c < out.mixed_air_temperature_c);
        assert!(out.total_cooling_w > 0.0);
        assert!(out.supply_fan_power_w > 0.0);
    }

    #[test]
    fn economizer_detected_when_oa_cooler() {
        let system = AirSystem::Cav { supply_fan: test_fan(), return_fan: None, cooling: test_cooling(), heating: None, design_flow_m3_s: 2.0 };
        let req = AirSystemRequest {
            outdoor_temperature_c: 15.0,
            outdoor_humidity_ratio: 0.006,
            outdoor_pressure_pa: P_STD,
            return_temperature_c: 24.0,
            return_humidity_ratio: 0.01,
            total_cooling_load_w: 5000.0,
            total_heating_load_w: 0.0,
            oa_fraction: 0.15,
            economizer: EconomizerControl::DifferentialDryBulb,
            zone_terminals: vec![],
            requested_supply_flow_m3_s: 2.0,
        };
        let out = simulate_air_system(&system, &req);
        assert!(out.economizer_active);
    }

    #[test]
    fn doas_conditions_outdoor_air() {
        let system = AirSystem::Doas { supply_fan: test_fan(), cooling: test_cooling(), heating: Some(HeatingCoil::Electric { capacity_w: 10_000.0, efficiency: 1.0 }), erv_effectiveness: 0.7, design_oa_m3_s: 0.5 };
        let req = AirSystemRequest {
            outdoor_temperature_c: 30.0,
            outdoor_humidity_ratio: humidity_ratio_from_rh(30.0, 0.6, P_STD),
            outdoor_pressure_pa: P_STD,
            return_temperature_c: 22.0,
            return_humidity_ratio: 0.009,
            total_cooling_load_w: 8000.0,
            total_heating_load_w: 0.0,
            oa_fraction: 1.0,
            economizer: EconomizerControl::None,
            zone_terminals: vec![],
            requested_supply_flow_m3_s: 0.5,
        };
        let out = simulate_air_system(&system, &req);
        assert!(out.mixed_air_temperature_c < req.outdoor_temperature_c);
        assert!((out.outdoor_air_mass_flow_kg_s - out.supply_mass_flow_kg_s).abs() < 0.01);
    }
}
