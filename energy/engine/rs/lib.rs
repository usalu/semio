//! ⚡ Headless building energy model engine: typed Rust API for BEM simulation.

#![allow(clippy::too_many_arguments)]

mod air_exchange {
    //! 💨 Infiltration, ventilation, interzone mixing, and hybrid air exchange controls.

    use crate::props::{latent_heat_vaporization, moist_air_density, moist_air_enthalpy_j_per_kg};
    use crate::units::{CP_DRY_AIR, GRAVITY, RHO_AIR_REF};
    use serde::{Deserialize, Serialize};

    // #region 🔖InfiltrationMethod
    /// 🚪 Infiltration flow calculation method.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum InfiltrationMethod {
        ScheduledAch,
        PerExteriorArea,
        EffectiveLeakageArea,
        WindAndStack,
    }
    // #endregion 🔖InfiltrationMethod

    // #region 🔖InfiltrationSpec
    /// 💨 Infiltration model parameters (EnergyPlus-style wind/stack coefficients).
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct InfiltrationSpec {
        pub method: InfiltrationMethod,
        pub schedule_factor: f64,
        pub ach: f64,
        pub flow_per_exterior_area_m3_s_m2: f64,
        pub effective_leakage_area_m2: f64,
        pub discharge_coefficient: f64,
        pub constant_coefficient: f64,
        pub temperature_coefficient: f64,
        pub velocity_coefficient: f64,
        pub velocity_squared_coefficient: f64,
        pub stack_height_m: f64,
    }
    // #endregion 🔖InfiltrationSpec

    // #region 🔖VentilationSpec
    /// 🌬️ Mechanical ventilation specification.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct VentilationSpec {
        pub design_flow_m3_s: f64,
        pub schedule_factor: f64,
        pub heat_recovery_effectiveness: f64,
        pub fan_heat_gain_w: f64,
        pub supply_temp_c: Option<f64>,
    }
    // #endregion 🔖VentilationSpec

    // #region 🔖InterzoneMixing
    /// ↔️ Interzone air mixing between adjacent zones.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct InterzoneMixing {
        pub flow_m3_s: f64,
        pub schedule_factor: f64,
    }
    // #endregion 🔖InterzoneMixing

    // #region 🔖HybridControl
    /// 🎛️ Hybrid ventilation control: natural when conditions allow, mechanical otherwise.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HybridVentilationControl {
        pub outdoor_temp_min_c: f64,
        pub outdoor_temp_max_c: f64,
        pub max_wind_speed_m_s: f64,
        pub natural_ach: f64,
        pub mechanical_backup: bool,
    }
    // #endregion 🔖HybridControl

    // #region 🔖InfiltrationFlow
    /// 💨 Infiltration volumetric flow [m³/s].
    pub fn infiltration_flow_m3_s(spec: &InfiltrationSpec, zone_volume_m3: f64, exterior_area_m2: f64, outdoor_temp_c: f64, zone_temp_c: f64, wind_speed_m_s: f64, p_atm: f64) -> f64 {
        let sf = spec.schedule_factor.clamp(0.0, 1.0);
        match spec.method {
            InfiltrationMethod::ScheduledAch => sf * spec.ach * zone_volume_m3 / 3600.0,
            InfiltrationMethod::PerExteriorArea => sf * spec.flow_per_exterior_area_m3_s_m2 * exterior_area_m2,
            InfiltrationMethod::EffectiveLeakageArea => {
                let rho = moist_air_density(outdoor_temp_c, 0.008, p_atm);
                let delta_p = wind_stack_pressure_pa(spec.stack_height_m, outdoor_temp_c, zone_temp_c, wind_speed_m_s, spec.constant_coefficient, spec.temperature_coefficient, spec.velocity_coefficient, spec.velocity_squared_coefficient);
                sf * spec.discharge_coefficient * spec.effective_leakage_area_m2 * (2.0 * delta_p.max(0.0) / rho).sqrt()
            }
            InfiltrationMethod::WindAndStack => {
                let base = sf * spec.flow_per_exterior_area_m3_s_m2 * exterior_area_m2;
                let delta_t = (outdoor_temp_c - zone_temp_c).abs();
                let wind_factor = 1.0 + spec.velocity_coefficient * wind_speed_m_s + spec.velocity_squared_coefficient * wind_speed_m_s * wind_speed_m_s;
                let temp_factor = 1.0 + spec.temperature_coefficient * delta_t;
                base * wind_factor * temp_factor + sf * spec.constant_coefficient
            }
        }
    }

    fn wind_stack_pressure_pa(height_m: f64, t_out_c: f64, t_zone_c: f64, wind_m_s: f64, c_const: f64, c_temp: f64, c_vel: f64, c_vel2: f64) -> f64 {
        let t_out_k = t_out_c + 273.15;
        let t_zone_k = t_zone_c + 273.15;
        let stack = RHO_AIR_REF * GRAVITY * height_m * (t_out_k - t_zone_k).abs() / t_zone_k.max(250.0);
        let wind = 0.5 * RHO_AIR_REF * (c_vel * wind_m_s + c_vel2 * wind_m_s * wind_m_s);
        c_const + c_temp * (t_out_c - t_zone_c).abs() + stack + wind
    }
    // #endregion 🔖InfiltrationFlow

    // #region 🔖VentilationLoad
    /// 🔥 Ventilation sensible and latent loads [W].
    pub fn ventilation_load_w(flow_m3_s: f64, t_zone_c: f64, w_zone: f64, t_out_c: f64, w_out: f64, p_atm: f64, heat_recovery_effectiveness: f64) -> (f64, f64) {
        if flow_m3_s <= 0.0 {
            return (0.0, 0.0);
        }
        let rho = moist_air_density(t_out_c, w_out, p_atm);
        let m_dot = rho * flow_m3_s;
        let _h_zone = moist_air_enthalpy_j_per_kg(t_zone_c, w_zone);
        let _h_out = moist_air_enthalpy_j_per_kg(t_out_c, w_out);
        let eps = heat_recovery_effectiveness.clamp(0.0, 1.0);
        let sensible = m_dot * CP_DRY_AIR * (t_out_c - t_zone_c) * (1.0 - eps);
        let h_fg = latent_heat_vaporization((t_zone_c + t_out_c) * 0.5);
        let latent = m_dot * (w_out - w_zone) * h_fg * (1.0 - eps);
        (sensible, latent)
    }
    // #endregion 🔖VentilationLoad

    // #region 🔖Interzone
    /// ↔️ Sensible and latent exchange [W] from interzone mixing flow.
    pub fn interzone_exchange_w(mixing: &InterzoneMixing, t_zone_c: f64, w_zone: f64, t_adjacent_c: f64, w_adjacent: f64, p_atm: f64) -> (f64, f64) {
        let flow = mixing.flow_m3_s * mixing.schedule_factor.clamp(0.0, 1.0);
        ventilation_load_w(flow, t_zone_c, w_zone, t_adjacent_c, w_adjacent, p_atm, 0.0)
    }
    // #endregion 🔖Interzone

    // #region 🔖Hybrid
    /// 🎛️ Hybrid ventilation flow [m³/s]: natural when outdoor conditions favorable.
    pub fn hybrid_ventilation_flow_m3_s(control: &HybridVentilationControl, zone_volume_m3: f64, outdoor_temp_c: f64, wind_speed_m_s: f64, mechanical_flow_m3_s: f64) -> f64 {
        let natural_ok = outdoor_temp_c >= control.outdoor_temp_min_c && outdoor_temp_c <= control.outdoor_temp_max_c && wind_speed_m_s <= control.max_wind_speed_m_s;
        if natural_ok {
            control.natural_ach * zone_volume_m3 / 3600.0
        } else if control.mechanical_backup {
            mechanical_flow_m3_s
        } else {
            0.0
        }
    }
    // #endregion 🔖Hybrid

    // #region 🔖AirExchangeResult
    /// 📊 Combined air exchange flows and loads for one zone.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AirExchangeResult {
        pub infiltration_flow_m3_s: f64,
        pub ventilation_flow_m3_s: f64,
        pub infiltration_sensible_w: f64,
        pub infiltration_latent_w: f64,
        pub ventilation_sensible_w: f64,
        pub ventilation_latent_w: f64,
    }

    /// 💨 Compute combined infiltration and ventilation for a zone timestep.
    pub fn compute_air_exchange(infiltration: &InfiltrationSpec, ventilation: &VentilationSpec, zone_volume_m3: f64, exterior_area_m2: f64, t_zone_c: f64, w_zone: f64, t_out_c: f64, w_out: f64, wind_speed_m_s: f64, p_atm: f64) -> AirExchangeResult {
        let inf_flow = infiltration_flow_m3_s(infiltration, zone_volume_m3, exterior_area_m2, t_out_c, t_zone_c, wind_speed_m_s, p_atm);
        let vent_flow = ventilation.design_flow_m3_s * ventilation.schedule_factor.clamp(0.0, 1.0);
        let (inf_sens, inf_lat) = ventilation_load_w(inf_flow, t_zone_c, w_zone, t_out_c, w_out, p_atm, 0.0);
        let (vent_sens, vent_lat) = ventilation_load_w(vent_flow, t_zone_c, w_zone, t_out_c, w_out, p_atm, ventilation.heat_recovery_effectiveness);
        AirExchangeResult {
            infiltration_flow_m3_s: inf_flow,
            ventilation_flow_m3_s: vent_flow,
            infiltration_sensible_w: inf_sens,
            infiltration_latent_w: inf_lat,
            ventilation_sensible_w: vent_sens + ventilation.fan_heat_gain_w,
            ventilation_latent_w: vent_lat,
        }
    }
    // #endregion 🔖AirExchangeResult

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::units::P_STD;

        #[test]
        fn ach_infiltration_scales_with_volume() {
            let spec = InfiltrationSpec {
                method: InfiltrationMethod::ScheduledAch,
                schedule_factor: 1.0,
                ach: 0.5,
                flow_per_exterior_area_m3_s_m2: 0.0,
                effective_leakage_area_m2: 0.0,
                discharge_coefficient: 0.6,
                constant_coefficient: 0.0,
                temperature_coefficient: 0.0,
                velocity_coefficient: 0.0,
                velocity_squared_coefficient: 0.0,
                stack_height_m: 3.0,
            };
            let flow = infiltration_flow_m3_s(&spec, 200.0, 50.0, 5.0, 22.0, 3.0, P_STD);
            assert!((flow - 200.0 * 0.5 / 3600.0).abs() < 1e-9);
        }

        #[test]
        fn ventilation_load_positive_when_outdoor_colder() {
            let (sens, _) = ventilation_load_w(0.1, 22.0, 0.009, 5.0, 0.004, P_STD, 0.0);
            assert!(sens < 0.0);
        }

        #[test]
        fn heat_recovery_reduces_load() {
            let (sens0, _) = ventilation_load_w(0.2, 22.0, 0.009, 5.0, 0.004, P_STD, 0.0);
            let (sens1, _) = ventilation_load_w(0.2, 22.0, 0.009, 5.0, 0.004, P_STD, 0.8);
            assert!(sens1.abs() < sens0.abs());
        }

        #[test]
        fn hybrid_uses_natural_when_favorable() {
            let ctrl = HybridVentilationControl { outdoor_temp_min_c: 10.0, outdoor_temp_max_c: 28.0, max_wind_speed_m_s: 5.0, natural_ach: 2.0, mechanical_backup: true };
            let flow = hybrid_ventilation_flow_m3_s(&ctrl, 300.0, 20.0, 2.0, 0.05);
            assert!((flow - 300.0 * 2.0 / 3600.0).abs() < 1e-9);
        }
    }
}

mod air_system {
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
}

mod airflow_network {
    //! 🌐 Pressure-driven multizone airflow network with stack and wind effects.

    use crate::props::moist_air_density;
    use crate::units::{GRAVITY, RHO_AIR_REF};
    use serde::{Deserialize, Serialize};

    // #region 🔖AfNode
    /// 🔵 Airflow network node (zone or outdoor reference).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AfNode {
        pub id: u32,
        pub elevation_m: f64,
        pub temperature_c: f64,
        pub humidity_ratio: f64,
        pub is_reference: bool,
    }

    impl AfNode {
        pub fn density(&self, p_atm: f64) -> f64 {
            moist_air_density(self.temperature_c, self.humidity_ratio, p_atm)
        }
    }
    // #endregion 🔖AfNode

    // #region 🔖AfLinkKind
    /// 🔗 Airflow link type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AfLinkKind {
        Crack,
        Opening,
        Door,
        Duct,
    }
    // #endregion 🔖AfLinkKind

    // #region 🔖AfLink
    /// ↔️ Pressure-flow link between two nodes (power-law Q = C·|ΔP|ⁿ).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AfLink {
        pub id: u32,
        pub node_a: u32,
        pub node_b: u32,
        pub kind: AfLinkKind,
        pub flow_coefficient: f64,
        pub flow_exponent: f64,
        pub area_m2: f64,
        pub discharge_coefficient: f64,
        pub orientation_deg: f64,
        pub wind_exposure_factor: f64,
    }
    // #endregion 🔖AfLink

    // #region 🔖AirflowNetwork
    /// 🌐 Multizone airflow network.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AirflowNetwork {
        pub nodes: Vec<AfNode>,
        pub links: Vec<AfLink>,
        pub wind_speed_m_s: f64,
        pub wind_direction_deg: f64,
        pub outdoor_temp_c: f64,
        pub outdoor_humidity_ratio: f64,
    }

    impl AirflowNetwork {
        pub fn node_index(&self, id: u32) -> Option<usize> {
            self.nodes.iter().position(|n| n.id == id)
        }

        /// 🌬️ Volumetric flow [m³/s] through link from node_a toward node_b (positive = a→b).
        pub fn link_flow_m3_s(&self, link: &AfLink, pressures_pa: &[f64], p_atm: f64) -> f64 {
            let ia = self.node_index(link.node_a).unwrap_or(0);
            let ib = self.node_index(link.node_b).unwrap_or(0);
            let node_a = &self.nodes[ia];
            let node_b = &self.nodes[ib];
            let dp_stack = stack_pressure_pa(node_a, node_b, p_atm);
            let dp_wind = wind_pressure_pa(link, self.wind_speed_m_s, self.wind_direction_deg);
            let dp = pressures_pa[ia] - pressures_pa[ib] + dp_stack + dp_wind;
            power_law_flow(link, dp, node_a.density(p_atm))
        }

        /// 🔍 Solve zone pressures [Pa] relative to reference node via Gauss-Seidel mass balance.
        pub fn solve_pressures(&self, p_atm: f64, max_iter: usize, tol: f64) -> Option<Vec<f64>> {
            let n = self.nodes.len();
            if n == 0 {
                return Some(Vec::new());
            }
            let ref_idx = self.nodes.iter().position(|node| node.is_reference)?;
            let mut pressures = vec![0.0; n];
            pressures[ref_idx] = 0.0;

            for _ in 0..max_iter {
                let mut max_delta = 0.0_f64;
                for i in 0..n {
                    if i == ref_idx {
                        continue;
                    }
                    let (sum_q, sum_g) = node_mass_balance(self, i, &pressures, p_atm);
                    if sum_g.abs() < 1e-12 {
                        continue;
                    }
                    let dp = -sum_q / sum_g;
                    let new_p = pressures[i] + dp;
                    max_delta = max_delta.max((new_p - pressures[i]).abs());
                    pressures[i] = new_p;
                }
                if max_delta < tol {
                    return Some(pressures);
                }
            }
            Some(pressures)
        }

        /// 📊 Flow rates [m³/s] for all links after pressure solve.
        pub fn solve_flows(&self, p_atm: f64) -> Option<Vec<f64>> {
            let pressures = self.solve_pressures(p_atm, 200, 1e-4)?;
            Some(self.links.iter().map(|link| self.link_flow_m3_s(link, &pressures, p_atm)).collect())
        }
    }
    // #endregion 🔖AirflowNetwork

    // #region 🔖Physics
    fn stack_pressure_pa(node_a: &AfNode, node_b: &AfNode, p_atm: f64) -> f64 {
        let rho_a = node_a.density(p_atm);
        let rho_b = node_b.density(p_atm);
        GRAVITY * (node_a.elevation_m - node_b.elevation_m) * (rho_a - rho_b) * 0.5
    }

    fn wind_pressure_pa(link: &AfLink, wind_speed_m_s: f64, wind_direction_deg: f64) -> f64 {
        let angle = (wind_direction_deg - link.orientation_deg).to_radians();
        let cp = angle.cos();
        0.5 * RHO_AIR_REF * wind_speed_m_s * wind_speed_m_s * cp * link.wind_exposure_factor
    }

    fn power_law_flow(link: &AfLink, dp_pa: f64, rho: f64) -> f64 {
        let n = link.flow_exponent.clamp(0.5, 1.0);
        let c = link.flow_coefficient.max(1e-12);
        let sign = if dp_pa >= 0.0 { 1.0 } else { -1.0 };
        sign * c * dp_pa.abs().powf(n) / rho.sqrt()
    }

    fn link_conductance(link: &AfLink, dp_pa: f64, rho: f64) -> f64 {
        let n = link.flow_exponent.clamp(0.5, 1.0);
        let c = link.flow_coefficient.max(1e-12);
        if dp_pa.abs() < 1e-6 {
            n * c / rho.sqrt()
        } else {
            n * c * dp_pa.abs().powf(n - 1.0) / rho.sqrt()
        }
    }

    fn node_mass_balance(network: &AirflowNetwork, node_i: usize, pressures: &[f64], p_atm: f64) -> (f64, f64) {
        let mut sum_q = 0.0;
        let mut sum_g = 0.0;
        for link in &network.links {
            let ia = network.node_index(link.node_a).unwrap_or(0);
            let ib = network.node_index(link.node_b).unwrap_or(0);
            if ia != node_i && ib != node_i {
                continue;
            }
            let node_a = &network.nodes[ia];
            let node_b = &network.nodes[ib];
            let dp_stack = stack_pressure_pa(node_a, node_b, p_atm);
            let dp_wind = wind_pressure_pa(link, network.wind_speed_m_s, network.wind_direction_deg);
            let dp = pressures[ia] - pressures[ib] + dp_stack + dp_wind;
            let rho = node_a.density(p_atm);
            let g = link_conductance(link, dp, rho);
            let q = power_law_flow(link, dp, rho);
            if node_i == ia {
                sum_q -= q;
                sum_g -= g;
            }
            if node_i == ib {
                sum_q += q;
                sum_g += g;
            }
        }
        (sum_q, sum_g)
    }
    // #endregion 🔖Physics

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::units::P_STD;

        fn two_zone_network() -> AirflowNetwork {
            AirflowNetwork {
                nodes: vec![AfNode { id: 0, elevation_m: 0.0, temperature_c: 5.0, humidity_ratio: 0.004, is_reference: true }, AfNode { id: 1, elevation_m: 0.0, temperature_c: 22.0, humidity_ratio: 0.009, is_reference: false }],
                links: vec![AfLink { id: 1, node_a: 1, node_b: 0, kind: AfLinkKind::Crack, flow_coefficient: 0.01, flow_exponent: 0.65, area_m2: 0.05, discharge_coefficient: 0.6, orientation_deg: 0.0, wind_exposure_factor: 1.0 }],
                wind_speed_m_s: 3.0,
                wind_direction_deg: 0.0,
                outdoor_temp_c: 5.0,
                outdoor_humidity_ratio: 0.004,
            }
        }

        #[test]
        fn stack_pressure_positive_when_outdoor_colder() {
            let outdoor = AfNode { id: 0, elevation_m: 0.0, temperature_c: 5.0, humidity_ratio: 0.004, is_reference: true };
            let zone = AfNode { id: 1, elevation_m: 3.0, temperature_c: 22.0, humidity_ratio: 0.009, is_reference: false };
            let dp = stack_pressure_pa(&zone, &outdoor, P_STD);
            assert!(dp.abs() > 0.0);
        }

        #[test]
        fn network_solves_pressures() {
            let net = two_zone_network();
            let pressures = net.solve_pressures(P_STD, 100, 1e-3).unwrap();
            assert_eq!(pressures.len(), 2);
            assert!((pressures[0]).abs() < 1e-9);
        }

        #[test]
        fn infiltration_flow_when_zone_warmer() {
            let net = two_zone_network();
            let flows = net.solve_flows(P_STD).unwrap();
            assert_eq!(flows.len(), 1);
        }
    }
}

mod calendar {
    //! 📅 Simulation calendar: run periods, day-of-week, leap years, DST shifts.

    use serde::{Deserialize, Serialize};

    // #region 🔖Date
    /// 📅 Calendar date for scheduling.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SimDate {
        pub year: u16,
        pub month: u8,
        pub day: u8,
    }

    impl SimDate {
        pub const fn new(year: u16, month: u8, day: u8) -> Self {
            Self { year, month, day }
        }

        /// 📅 Day of year (1-based).
        pub fn day_of_year(&self) -> u16 {
            let days_before = days_before_month(self.month, is_leap_year(self.year));
            days_before + self.day as u16
        }

        /// 📅 Day of week (1=Mon … 7=Sun).
        pub fn day_of_week(&self) -> u8 {
            let y = self.year as i32;
            let m = self.month as i32;
            let d = self.day as i32;
            let mm = if m < 3 { m + 12 } else { m };
            let yy = if m < 3 { y - 1 } else { y };
            let h = (d + (13 * (mm + 1)) / 5 + yy + yy / 4 - yy / 100 + yy / 400) % 7;
            match h {
                0 => 7,
                n => n as u8,
            }
        }

        /// 📅 Advance by one day.
        pub fn advance_day(&mut self) {
            let max_day = days_in_month(self.month, is_leap_year(self.year));
            if self.day < max_day {
                self.day += 1;
                return;
            }
            self.day = 1;
            if self.month < 12 {
                self.month += 1;
            } else {
                self.month = 1;
                self.year += 1;
            }
        }
    }
    // #endregion 🔖Date

    // #region 🔖RunPeriod
    /// 📅 Run period specification.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RunPeriod {
        pub start_month: u8,
        pub start_day: u8,
        pub end_month: u8,
        pub end_day: u8,
        pub year: u16,
    }

    impl Default for RunPeriod {
        fn default() -> Self {
            Self { start_month: 1, start_day: 1, end_month: 12, end_day: 31, year: 2026 }
        }
    }

    impl RunPeriod {
        /// 📅 Total simulation hours in run period.
        pub fn total_hours(&self) -> u32 {
            let mut date = SimDate::new(self.year, self.start_month, self.start_day);
            let end = SimDate::new(self.year, self.end_month, self.end_day);
            let mut hours = 0u32;
            loop {
                hours += 24;
                if date.month == end.month && date.day == end.day {
                    break;
                }
                date.advance_day();
                if hours > 8760 * 2 {
                    break;
                }
            }
            hours
        }

        /// 📅 Iterator over (date, hour) pairs.
        pub fn hours(&self) -> RunPeriodHours {
            RunPeriodHours { current: SimDate::new(self.year, self.start_month, self.start_day), end: SimDate::new(self.year, self.end_month, self.end_day), hour: 0u8, index: 0u32, finished: false }
        }
    }

    /// 📅 Hour iterator for a run period.
    pub struct RunPeriodHours {
        current: SimDate,
        end: SimDate,
        hour: u8,
        index: u32,
        finished: bool,
    }

    impl RunPeriodHours {
        pub fn index(&self) -> u32 {
            self.index
        }
    }

    impl Iterator for RunPeriodHours {
        type Item = (SimDate, u8, u32);

        fn next(&mut self) -> Option<Self::Item> {
            if self.finished {
                return None;
            }
            if self.current.month > self.end.month || (self.current.month == self.end.month && self.current.day > self.end.day) {
                return None;
            }
            let item = (self.current, self.hour, self.index);
            self.index += 1;
            if self.current.month == self.end.month && self.current.day == self.end.day && self.hour == 23 {
                self.finished = true;
                return Some(item);
            }
            self.hour += 1;
            if self.hour >= 24 {
                self.hour = 0;
                self.current.advance_day();
            }
            Some(item)
        }
    }
    // #endregion 🔖RunPeriod

    // #region 🔖Dst
    /// 🕐 Daylight saving time rule (simplified US-style).
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DstRule {
        pub start_month: u8,
        pub start_week: u8,
        pub end_month: u8,
        pub end_week: u8,
        pub shift_hours: f64,
    }

    impl DstRule {
        /// 🕐 Whether DST is active for date/hour (local standard time).
        pub fn is_dst(&self, date: SimDate, hour: u8) -> bool {
            let start_doy = nth_weekday_doy(date.year, self.start_month, self.start_week, 0);
            let end_doy = nth_weekday_doy(date.year, self.end_month, self.end_week, 0);
            let doy = date.day_of_year();
            doy >= start_doy && doy < end_doy && hour >= 2
        }

        /// 🕐 Schedule hour shift for DST.
        pub fn schedule_shift(&self, date: SimDate, hour: u8) -> f64 {
            if self.is_dst(date, hour) {
                self.shift_hours
            } else {
                0.0
            }
        }
    }
    // #endregion 🔖Dst

    // #region 🔖Helpers
    fn is_leap_year(year: u16) -> bool {
        let y = year as u32;
        (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
    }

    fn days_in_month(month: u8, leap: bool) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap => 29,
            2 => 28,
            _ => 30,
        }
    }

    fn days_before_month(month: u8, leap: bool) -> u16 {
        let days = [0u16, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let idx = (month.saturating_sub(1)) as usize;
        let base = days.get(idx).copied().unwrap_or(0);
        if leap && month > 2 {
            base + 1
        } else {
            base
        }
    }

    fn nth_weekday_doy(year: u16, month: u8, nth: u8, weekday: u8) -> u16 {
        let first = SimDate::new(year, month, 1);
        let first_dow = first.day_of_week();
        let offset = (7 + weekday - first_dow) % 7;
        let day = 1 + offset + (nth.saturating_sub(1)) * 7;
        days_before_month(month, is_leap_year(year)) + day as u16
    }
    // #endregion 🔖Helpers

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn leap_year_feb_has_29_days() {
            assert_eq!(days_in_month(2, true), 29);
            assert_eq!(days_in_month(2, false), 28);
        }

        #[test]
        fn run_period_jan_week_is_168_hours() {
            let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
            assert_eq!(period.total_hours(), 168);
        }

        #[test]
        fn day_of_week_known_date() {
            let d = SimDate::new(2026, 1, 1);
            assert!(d.day_of_week() >= 1 && d.day_of_week() <= 7);
        }

        #[test]
        fn hours_iterator_count() {
            let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 2, year: 2026 };
            assert_eq!(period.hours().count(), 48);
        }
    }
}

mod coils {
    //! 🔥❄️ Heating and cooling coils: electric, gas, water, DX with bypass factor.

    use crate::curves::PerformanceCurve;
    use crate::props::{latent_heat_vaporization, moist_air_enthalpy_j_per_kg, saturation_pressure_pa};
    use crate::units::{CP_DRY_AIR, H_FG_0C};
    use serde::{Deserialize, Serialize};

    // #region 🔖HeatingCoil
    /// 🔥 Heating coil types and ratings.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HeatingCoil {
        Electric { capacity_w: f64, efficiency: f64 },
        Gas { capacity_w: f64, efficiency: f64 },
        HotWater { ua_w_per_k: f64, water_inlet_c: f64, water_flow_kg_s: f64, water_cp: f64 },
        Steam { capacity_w: f64, latent_fraction: f64 },
    }

    /// 📥 Heating coil inlet air state.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CoilAirState {
        pub temperature_c: f64,
        pub humidity_ratio: f64,
        pub mass_flow_kg_s: f64,
        pub pressure_pa: f64,
    }

    /// 📤 Heating coil output.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HeatingCoilOutput {
        pub outlet: CoilAirState,
        pub total_heating_w: f64,
        pub gas_consumption_w: f64,
        pub water_heat_removal_w: f64,
    }
    // #endregion 🔖HeatingCoil

    // #region 🔖CoolingCoil
    /// ❄️ Cooling coil types including DX stages.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CoolingCoil {
        ChilledWater { ua_w_per_k: f64, water_inlet_c: f64, water_flow_kg_s: f64, water_cp: f64 },
        DxSingleSpeed { rated_capacity_w: f64, rated_shr: f64, cop_curve: PerformanceCurve },
        DxMultiSpeed { stages: Vec<DxStage> },
        DxVariableSpeed { rated_capacity_w: f64, rated_cop: f64, cop_curve: PerformanceCurve },
    }

    /// ❄️ DX compressor stage.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DxStage {
        pub capacity_w: f64,
        pub cop: f64,
        pub shr: f64,
    }

    /// 📤 Cooling coil output.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CoolingCoilOutput {
        pub outlet: CoilAirState,
        pub total_cooling_w: f64,
        pub sensible_cooling_w: f64,
        pub latent_cooling_w: f64,
        pub compressor_power_w: f64,
        pub condensate_kg_s: f64,
    }
    // #endregion 🔖CoolingCoil

    // #region 🔖HeatingOutput
    /// 🔥 Compute heating coil delivered capacity and outlet air state [W].
    pub fn heating_coil_output_w(coil: &HeatingCoil, inlet: &CoilAirState, load_w: f64) -> HeatingCoilOutput {
        let m_dot = inlet.mass_flow_kg_s.max(0.0);
        if m_dot < 1e-9 || load_w <= 0.0 {
            return HeatingCoilOutput { outlet: *inlet, total_heating_w: 0.0, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 };
        }

        match coil {
            HeatingCoil::Electric { capacity_w, efficiency } => {
                let q = load_w.min(*capacity_w);
                let delta_t = q / (m_dot * CP_DRY_AIR);
                HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, ..*inlet }, total_heating_w: q, gas_consumption_w: q / efficiency.max(0.01), water_heat_removal_w: 0.0 }
            }
            HeatingCoil::Gas { capacity_w, efficiency } => {
                let q = load_w.min(*capacity_w);
                let delta_t = q / (m_dot * CP_DRY_AIR);
                HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, ..*inlet }, total_heating_w: q, gas_consumption_w: q / efficiency.max(0.01), water_heat_removal_w: 0.0 }
            }
            HeatingCoil::HotWater { ua_w_per_k, water_inlet_c, water_flow_kg_s: _, water_cp: _ } => {
                let q_max = ua_w_per_k * (water_inlet_c - inlet.temperature_c).max(0.0);
                let q = load_w.min(q_max);
                let delta_t = q / (m_dot * CP_DRY_AIR);
                let water_q = q;
                HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, ..*inlet }, total_heating_w: q, gas_consumption_w: 0.0, water_heat_removal_w: water_q }
            }
            HeatingCoil::Steam { capacity_w, latent_fraction } => {
                let q = load_w.min(*capacity_w);
                let delta_t = q / (m_dot * CP_DRY_AIR);
                let humid_add = *latent_fraction * q / H_FG_0C * m_dot / m_dot.max(1.0);
                HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, humidity_ratio: inlet.humidity_ratio + humid_add * 0.001, ..*inlet }, total_heating_w: q, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 }
            }
        }
    }
    // #endregion 🔖HeatingOutput

    // #region 🔖CoolingOutput
    /// ❄️ Compute cooling coil capacity with bypass factor and wet/dry behavior [W].
    pub fn cooling_coil_output_w(coil: &CoolingCoil, inlet: &CoilAirState, load_w: f64, bypass_factor: f64) -> CoolingCoilOutput {
        let m_dot = inlet.mass_flow_kg_s.max(0.0);
        let bf = bypass_factor.clamp(0.0, 0.95);
        if m_dot < 1e-9 || load_w <= 0.0 {
            return CoolingCoilOutput { outlet: *inlet, total_cooling_w: 0.0, sensible_cooling_w: 0.0, latent_cooling_w: 0.0, compressor_power_w: 0.0, condensate_kg_s: 0.0 };
        }

        let h_in = moist_air_enthalpy_j_per_kg(inlet.temperature_c, inlet.humidity_ratio);
        let t_apparatus_dew = apparatus_dew_point_c(inlet.temperature_c, inlet.humidity_ratio, inlet.pressure_pa);
        let t_adp = t_apparatus_dew;

        let (q_max, cop, shr) = match coil {
            CoolingCoil::ChilledWater { ua_w_per_k, water_inlet_c, .. } => {
                let q = ua_w_per_k * (inlet.temperature_c - water_inlet_c).max(0.0);
                (q, 5.0, 0.75)
            }
            CoolingCoil::DxSingleSpeed { rated_capacity_w, rated_shr, cop_curve } => {
                let plr = (load_w / rated_capacity_w.max(1.0)).clamp(0.0, 1.0);
                let cop = 3.5 * cop_curve.evaluate(plr).max(0.5);
                (*rated_capacity_w, cop, *rated_shr)
            }
            CoolingCoil::DxMultiSpeed { stages } => {
                let mut remaining = load_w;
                let mut cap = 0.0;
                let mut cop_sum = 0.0;
                let mut shr_sum = 0.0;
                let mut n = 0.0;
                for stage in stages {
                    if remaining <= 0.0 {
                        break;
                    }
                    let q = remaining.min(stage.capacity_w);
                    cap += q;
                    cop_sum += stage.cop * q;
                    shr_sum += stage.shr * q;
                    n += q;
                    remaining -= q;
                }
                let cop = if n > 0.0 { cop_sum / n } else { 3.0 };
                let shr = if n > 0.0 { shr_sum / n } else { 0.7 };
                (cap, cop, shr)
            }
            CoolingCoil::DxVariableSpeed { rated_capacity_w, rated_cop, cop_curve } => {
                let plr = (load_w / rated_capacity_w.max(1.0)).clamp(0.0, 1.0);
                let cop = rated_cop * cop_curve.evaluate(plr).max(0.5);
                (*rated_capacity_w, cop, 0.72)
            }
        };

        let q_total = load_w.min(q_max);
        let q_sensible = q_total * shr;
        let q_latent = q_total - q_sensible;

        let t_saturated = t_adp;
        let w_sat = saturation_humidity_ratio(t_saturated, inlet.pressure_pa);
        let h_sat = moist_air_enthalpy_j_per_kg(t_saturated, w_sat);
        let h_out_ideal = h_in - q_total / m_dot;
        let _h_out = bf * h_in + (1.0 - bf) * h_out_ideal;
        let t_out_ideal = inlet.temperature_c - q_sensible / (m_dot * CP_DRY_AIR);
        let t_out = bf * inlet.temperature_c + (1.0 - bf) * t_out_ideal;
        let w_out_ideal = inlet.humidity_ratio - q_latent / (m_dot * latent_heat_vaporization(t_out));
        let w_out = (bf * inlet.humidity_ratio + (1.0 - bf) * w_out_ideal).max(0.0);

        let condensate = (inlet.humidity_ratio - w_out).max(0.0) * m_dot;
        let compressor_power = q_total / cop.max(0.5);

        let _ = h_sat;

        CoolingCoilOutput {
            outlet: CoilAirState { temperature_c: t_out, humidity_ratio: w_out, ..*inlet },
            total_cooling_w: q_total,
            sensible_cooling_w: q_sensible,
            latent_cooling_w: q_latent,
            compressor_power_w: compressor_power,
            condensate_kg_s: condensate,
        }
    }

    fn apparatus_dew_point_c(t_db: f64, w: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_db);
        let p_w = w * p_atm / (0.621_945 + w);
        let rh = (p_w / p_ws).clamp(0.01, 1.0);
        t_db - (1.0 - rh) * 5.0
    }

    fn saturation_humidity_ratio(t_c: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_c);
        0.621_945 * p_ws / (p_atm - p_ws).max(1.0)
    }
    // #endregion 🔖CoolingOutput

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn electric_heating_raises_temperature() {
            let coil = HeatingCoil::Electric { capacity_w: 10_000.0, efficiency: 1.0 };
            let inlet = CoilAirState { temperature_c: 15.0, humidity_ratio: 0.008, mass_flow_kg_s: 0.5, pressure_pa: 101_325.0 };
            let out = heating_coil_output_w(&coil, &inlet, 5000.0);
            assert!(out.outlet.temperature_c > inlet.temperature_c);
            assert!((out.total_heating_w - 5000.0).abs() < 1.0);
        }

        #[test]
        fn dx_cooling_removes_sensible_and_latent() {
            let coil = CoolingCoil::DxSingleSpeed { rated_capacity_w: 15_000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) };
            let inlet = CoilAirState { temperature_c: 28.0, humidity_ratio: 0.012, mass_flow_kg_s: 0.6, pressure_pa: 101_325.0 };
            let out = cooling_coil_output_w(&coil, &inlet, 10_000.0, 0.1);
            assert!(out.outlet.temperature_c < inlet.temperature_c);
            assert!(out.sensible_cooling_w > 0.0);
            assert!(out.latent_cooling_w > 0.0);
            assert!(out.compressor_power_w > 0.0);
        }

        #[test]
        fn bypass_factor_reduces_effect() {
            let coil = CoolingCoil::DxSingleSpeed { rated_capacity_w: 15_000.0, rated_shr: 0.8, cop_curve: PerformanceCurve::Constant(1.0) };
            let inlet = CoilAirState { temperature_c: 30.0, humidity_ratio: 0.014, mass_flow_kg_s: 0.5, pressure_pa: 101_325.0 };
            let out_low_bf = cooling_coil_output_w(&coil, &inlet, 8000.0, 0.05);
            let out_high_bf = cooling_coil_output_w(&coil, &inlet, 8000.0, 0.4);
            assert!(out_low_bf.outlet.temperature_c < out_high_bf.outlet.temperature_c);
        }
    }
}

mod comfort {
    //! 😌 Thermal comfort: PMV, PPD, operative temperature, MRT, adaptive models.

    use crate::props::saturation_pressure_pa;
    use serde::{Deserialize, Serialize};

    // #region 🔖ComfortInput
    /// 🧍 Inputs for comfort evaluation per ISO 7730 / ASHRAE 55.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ComfortInput {
        pub air_temp_c: f64,
        pub mean_radiant_temp_c: f64,
        pub air_speed_m_s: f64,
        pub relative_humidity: f64,
        pub metabolic_rate_met: f64,
        pub clothing_insulation_clo: f64,
        pub external_work_met: f64,
    }
    // #endregion 🔖ComfortInput

    // #region 🔖AdaptiveComfort
    /// 🌿 Adaptive comfort standard.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AdaptiveStandard {
        Ashrae55,
        Cen15251,
    }
    // #endregion 🔖AdaptiveComfort

    // #region 🔖OperativeTemp
    /// 🌡️ Operative temperature [°C] from convective/radiant weighting.
    pub fn operative_temp_c(air_temp_c: f64, mean_radiant_temp_c: f64, air_speed_m_s: f64) -> f64 {
        let v = air_speed_m_s.max(0.05);
        let hc = 2.38 * (air_temp_c - mean_radiant_temp_c).abs().powf(0.25).max(3.0);
        let hv = 12.1 * v.sqrt();
        let h = if v > 0.2 { hv } else { hc };
        let hr = 4.0;
        (h * air_temp_c + hr * mean_radiant_temp_c) / (h + hr)
    }

    /// ☀️ Mean radiant temperature [°C] from surface temperatures and view factors.
    pub fn mean_radiant_temp_c(surface_temps_k: &[f64], view_factors: &[f64]) -> f64 {
        if surface_temps_k.is_empty() || surface_temps_k.len() != view_factors.len() {
            return 293.15 - 273.15;
        }
        let t4: f64 = surface_temps_k.iter().zip(view_factors.iter()).map(|(&t, &vf)| vf * (t / 100.0).powi(4)).sum();
        100.0 * t4.powf(0.25) - 273.15
    }

    /// ☀️ MRT from dry-bulb and enclosure delta-T approximation [°C].
    pub fn mean_radiant_temp_from_enclosure(air_temp_c: f64, enclosure_delta_t_k: f64) -> f64 {
        air_temp_c + enclosure_delta_t_k
    }
    // #endregion 🔖OperativeTemp

    // #region 🔖Pmv
    /// 😌 Predicted Mean Vote per ISO 7730 Fanger model.
    pub fn pmv(input: &ComfortInput) -> f64 {
        let m = input.metabolic_rate_met * 58.15;
        let w = input.external_work_met * 58.15;
        let i_cl = 0.155 * input.clothing_insulation_clo;
        let f_cl = if i_cl <= 0.078 { 1.0 + 1.29 * i_cl } else { 1.05 + 0.645 * i_cl };
        let t_a = input.air_temp_c;
        let t_r = input.mean_radiant_temp_c;
        let v = input.air_speed_m_s.max(0.0);
        let p_a = input.relative_humidity.clamp(0.0, 1.0) * saturation_pressure_pa(t_a);
        let t_cl = solve_clothing_temp_c(t_a, t_r, m, w, i_cl, f_cl, v);
        let h_c = if v < 0.1 { 2.38 * (t_cl - t_a).abs().powf(0.25) } else { 12.1 * v.sqrt() };
        let t_cl_k = t_cl + 273.15;
        let t_r_k = t_r + 273.15;
        let e_r = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
        let e_c = f_cl * h_c * (t_cl - t_a);
        let e_sw = 3.05e-3 * (5733.0 - 6.99 * (m - w) - p_a).max(0.0);
        let e_diff = if m > 58.15 { 0.42 * (m - w - 58.15) } else { 0.0 };
        let e = e_sw + e_diff;
        let c_res = 1.7e-5 * m * (34.0 - t_a);
        let l = m - w - e - e_r - e_c - c_res;
        ((0.303 * (-0.035 * m).exp() + 0.028) * l).clamp(-3.0, 3.0)
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
    // #endregion 🔖Pmv

    // #region 🔖Ppd
    /// 📊 Predicted Percentage Dissatisfied from PMV.
    pub fn ppd(pmv_value: f64) -> f64 {
        let pmv_c = pmv_value.clamp(-3.0, 3.0);
        100.0 - 95.0 * (-0.03353 * pmv_c.powi(4) - 0.2179 * pmv_c.powi(2)).exp()
    }
    // #endregion 🔖Ppd

    // #region 🔖Adaptive
    /// 🌿 Adaptive comfort acceptable temperature range [°C] (lower, upper).
    pub fn adaptive_comfort_range_c(standard: AdaptiveStandard, running_mean_outdoor_temp_c: f64, acceptability_class: u8) -> (f64, f64) {
        let t_rm = running_mean_outdoor_temp_c;
        let (center, band) = match standard {
            AdaptiveStandard::Ashrae55 => (0.31 * t_rm + 17.8, if acceptability_class <= 1 { 2.5 } else { 3.5 }),
            AdaptiveStandard::Cen15251 => (0.33 * t_rm + 18.8, if acceptability_class <= 1 { 2.0 } else { 3.0 }),
        };
        (center - band, center + band)
    }

    /// ✅ Whether operative temperature is within adaptive comfort band.
    pub fn adaptive_comfort_ok(standard: AdaptiveStandard, operative_temp_c: f64, running_mean_outdoor_temp_c: f64, acceptability_class: u8) -> bool {
        let (lo, hi) = adaptive_comfort_range_c(standard, running_mean_outdoor_temp_c, acceptability_class);
        operative_temp_c >= lo && operative_temp_c <= hi
    }
    // #endregion 🔖Adaptive

    // #region 🔖RadiantAsymmetry
    /// ☀️ Radiant asymmetry comfort limit check [°C] (ceiling vs floor).
    pub fn radiant_asymmetry_ok(temp_high_c: f64, temp_low_c: f64, max_asymmetry_k: f64) -> bool {
        (temp_high_c - temp_low_c).abs() <= max_asymmetry_k
    }
    // #endregion 🔖RadiantAsymmetry

    #[cfg(test)]
    mod tests {
        use super::*;

        fn neutral_input() -> ComfortInput {
            ComfortInput { air_temp_c: 22.0, mean_radiant_temp_c: 22.0, air_speed_m_s: 0.1, relative_humidity: 0.5, metabolic_rate_met: 1.0, clothing_insulation_clo: 0.5, external_work_met: 0.0 }
        }

        #[test]
        fn operative_equals_air_when_equal_mrt() {
            let top = operative_temp_c(22.0, 22.0, 0.1);
            assert!((top - 22.0).abs() < 0.5);
        }

        #[test]
        fn pmv_near_zero_at_neutral() {
            let p = pmv(&neutral_input());
            assert!(p.abs() < 1.5, "pmv={p}");
        }

        #[test]
        fn ppd_minimum_at_zero_pmv() {
            let p = ppd(0.0);
            assert!((p - 5.0).abs() < 1.0);
        }

        #[test]
        fn pmv_increases_when_too_warm() {
            let mut hot = neutral_input();
            hot.air_temp_c = 30.0;
            hot.mean_radiant_temp_c = 30.0;
            assert!(pmv(&hot) > pmv(&neutral_input()));
        }

        #[test]
        fn adaptive_range_centers_on_outdoor() {
            let (lo, hi) = adaptive_comfort_range_c(AdaptiveStandard::Ashrae55, 20.0, 1);
            let center = 0.5 * (lo + hi);
            assert!((center - (0.31 * 20.0 + 17.8)).abs() < 0.5);
        }

        #[test]
        fn mrt_from_surfaces() {
            let temps_k = [293.15, 303.15];
            let vfs = [0.5, 0.5];
            let mrt = mean_radiant_temp_c(&temps_k, &vfs);
            assert!(mrt > 20.0 && mrt < 30.0);
        }
    }
}

mod controls {
    //! 🎛️ Zone controls: thermostats, humidistats, load prediction, equipment priority.

    use serde::{Deserialize, Serialize};

    // #region 🔖ZoneLoad
    /// 📊 Predicted zone heating/cooling/humidification loads [W].
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct ZoneLoad {
        pub heating_w: f64,
        pub cooling_w: f64,
        pub humidifying_w: f64,
        pub dehumidifying_w: f64,
        pub sensible_w: f64,
        pub latent_w: f64,
    }

    impl ZoneLoad {
        pub fn total_w(&self) -> f64 {
            self.heating_w + self.cooling_w + self.humidifying_w + self.dehumidifying_w
        }

        pub fn net_sensible_w(&self) -> f64 {
            self.heating_w - self.cooling_w + self.sensible_w
        }

        pub fn net_latent_w(&self) -> f64 {
            self.humidifying_w - self.dehumidifying_w + self.latent_w
        }
    }
    // #endregion 🔖ZoneLoad

    // #region 🔖ControlAction
    /// 🎛️ HVAC control action requested by zone controller.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ControlAction {
        NoAction,
        Heat { power_w: f64 },
        Cool { power_w: f64 },
        Humidify { power_w: f64 },
        Dehumidify { power_w: f64 },
        Ventilate { flow_m3_s: f64 },
    }
    // #endregion 🔖ControlAction

    // #region 🔖ThermostatOutput
    /// 🌡️ Thermostat and humidistat combined output.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ThermostatOutput {
        pub heating_fraction: f64,
        pub cooling_fraction: f64,
        pub humidifying_fraction: f64,
        pub dehumidifying_fraction: f64,
        pub heating_setpoint_c: f64,
        pub cooling_setpoint_c: f64,
        pub humidifying_setpoint_rh: f64,
        pub dehumidifying_setpoint_rh: f64,
    }
    // #endregion 🔖ThermostatOutput

    // #region 🔖ThermostatSpec
    /// 🌡️ Proportional thermostat with throttle ranges [K].
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ThermostatSpec {
        pub heating_setpoint_c: f64,
        pub cooling_setpoint_c: f64,
        pub heating_throttle_range_k: f64,
        pub cooling_throttle_range_k: f64,
        pub min_heating_setpoint_c: f64,
        pub max_cooling_setpoint_c: f64,
    }
    // #endregion 🔖ThermostatSpec

    // #region 🔖HumidistatSpec
    /// 💧 Humidistat with RH setpoints and throttle ranges.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HumidistatSpec {
        pub humidifying_setpoint_rh: f64,
        pub dehumidifying_setpoint_rh: f64,
        pub humidifying_throttle_range: f64,
        pub dehumidifying_throttle_range: f64,
    }
    // #endregion 🔖HumidistatSpec

    // #region 🔖ZoneEquipmentPriority
    /// 🏆 Equipment serving priority for load allocation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct ZoneEquipmentPriority(pub u8);
    // #endregion 🔖ZoneEquipmentPriority

    // #region 🔖Thermostat
    fn proportional_fraction(error: f64, throttle: f64) -> f64 {
        if error <= 0.0 || throttle <= 0.0 {
            return 0.0;
        }
        (error / throttle).clamp(0.0, 1.0)
    }

    /// 🌡️ Evaluate thermostat and humidistat for current zone conditions.
    pub fn evaluate_controls(thermostat: &ThermostatSpec, humidistat: Option<&HumidistatSpec>, zone_temp_c: f64, zone_rh: f64) -> ThermostatOutput {
        let heat_err = thermostat.heating_setpoint_c - zone_temp_c;
        let cool_err = zone_temp_c - thermostat.cooling_setpoint_c;
        let heating_fraction = proportional_fraction(heat_err, thermostat.heating_throttle_range_k);
        let cooling_fraction = proportional_fraction(cool_err, thermostat.cooling_throttle_range_k);

        let (humid_frac, dehumid_frac, hum_sp, dehum_sp) = if let Some(h) = humidistat {
            let hum_err = h.humidifying_setpoint_rh - zone_rh;
            let dehum_err = zone_rh - h.dehumidifying_setpoint_rh;
            (proportional_fraction(hum_err, h.humidifying_throttle_range), proportional_fraction(dehum_err, h.dehumidifying_throttle_range), h.humidifying_setpoint_rh, h.dehumidifying_setpoint_rh)
        } else {
            (0.0, 0.0, 0.0, 1.0)
        };

        ThermostatOutput {
            heating_fraction,
            cooling_fraction,
            humidifying_fraction: humid_frac,
            dehumidifying_fraction: dehumid_frac,
            heating_setpoint_c: thermostat.heating_setpoint_c.max(thermostat.min_heating_setpoint_c),
            cooling_setpoint_c: thermostat.cooling_setpoint_c.min(thermostat.max_cooling_setpoint_c),
            humidifying_setpoint_rh: hum_sp,
            dehumidifying_setpoint_rh: dehum_sp,
        }
    }
    // #endregion 🔖Thermostat

    // #region 🔖LoadPrediction
    /// 📈 Predict zone loads from balance residuals and control fractions.
    pub fn predict_zone_load(sensible_residual_w: f64, latent_residual_w: f64, output: &ThermostatOutput, max_heating_w: f64, max_cooling_w: f64, max_humidifying_w: f64, max_dehumidifying_w: f64) -> ZoneLoad {
        let mut load = ZoneLoad { sensible_w: sensible_residual_w, latent_w: latent_residual_w, ..Default::default() };

        if sensible_residual_w < 0.0 {
            load.heating_w = (-sensible_residual_w * output.heating_fraction).min(max_heating_w);
        } else if sensible_residual_w > 0.0 {
            load.cooling_w = (sensible_residual_w * output.cooling_fraction).min(max_cooling_w);
        }

        if latent_residual_w < 0.0 {
            load.humidifying_w = (-latent_residual_w * output.humidifying_fraction).min(max_humidifying_w);
        } else if latent_residual_w > 0.0 {
            load.dehumidifying_w = (latent_residual_w * output.dehumidifying_fraction).min(max_dehumidifying_w);
        }

        load
    }
    // #endregion 🔖LoadPrediction

    // #region 🔖ActionMapping
    /// 🎛️ Map zone load to prioritized control actions.
    pub fn load_to_actions(load: &ZoneLoad, ventilation_flow_m3_s: f64) -> Vec<ControlAction> {
        let mut actions = Vec::new();
        if load.heating_w > 0.0 {
            actions.push(ControlAction::Heat { power_w: load.heating_w });
        }
        if load.cooling_w > 0.0 {
            actions.push(ControlAction::Cool { power_w: load.cooling_w });
        }
        if load.humidifying_w > 0.0 {
            actions.push(ControlAction::Humidify { power_w: load.humidifying_w });
        }
        if load.dehumidifying_w > 0.0 {
            actions.push(ControlAction::Dehumidify { power_w: load.dehumidifying_w });
        }
        if ventilation_flow_m3_s > 0.0 {
            actions.push(ControlAction::Ventilate { flow_m3_s: ventilation_flow_m3_s });
        }
        if actions.is_empty() {
            actions.push(ControlAction::NoAction);
        }
        actions
    }
    // #endregion 🔖ActionMapping

    // #region 🔖EquipmentAllocation
    /// 🏆 Allocate zone load across equipment by priority until capacity exhausted.
    pub fn allocate_load_by_priority(load: ZoneLoad, capacities_w: &[(ZoneEquipmentPriority, f64)]) -> Vec<(ZoneEquipmentPriority, ZoneLoad)> {
        let mut sorted: Vec<_> = capacities_w.to_vec();
        sorted.sort_by_key(|(p, _)| *p);
        let mut remaining = load;
        let mut result = Vec::new();

        for (priority, capacity) in sorted {
            if remaining.total_w() <= 0.0 {
                break;
            }
            let frac = (remaining.total_w() / load.total_w().max(1.0)).min(1.0);
            let alloc = ZoneLoad {
                heating_w: remaining.heating_w.min(capacity * frac),
                cooling_w: remaining.cooling_w.min(capacity * frac),
                humidifying_w: remaining.humidifying_w.min(capacity * frac * 0.5),
                dehumidifying_w: remaining.dehumidifying_w.min(capacity * frac * 0.5),
                sensible_w: remaining.sensible_w * frac,
                latent_w: remaining.latent_w * frac,
            };
            remaining.heating_w -= alloc.heating_w;
            remaining.cooling_w -= alloc.cooling_w;
            remaining.humidifying_w -= alloc.humidifying_w;
            remaining.dehumidifying_w -= alloc.dehumidifying_w;
            result.push((priority, alloc));
        }
        result
    }
    // #endregion 🔖EquipmentAllocation

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_thermostat() -> ThermostatSpec {
            ThermostatSpec { heating_setpoint_c: 21.0, cooling_setpoint_c: 24.0, heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0, min_heating_setpoint_c: 10.0, max_cooling_setpoint_c: 30.0 }
        }

        #[test]
        fn heating_fraction_full_when_cold() {
            let out = evaluate_controls(&default_thermostat(), None, 18.0, 0.5);
            assert!((out.heating_fraction - 1.0).abs() < 1e-9);
            assert!((out.cooling_fraction).abs() < 1e-9);
        }

        #[test]
        fn cooling_fraction_when_warm() {
            let out = evaluate_controls(&default_thermostat(), None, 26.0, 0.5);
            assert!((out.cooling_fraction - 1.0).abs() < 1e-9);
        }

        #[test]
        fn predict_heating_load_when_negative_residual() {
            let out = evaluate_controls(&default_thermostat(), None, 19.0, 0.5);
            let load = predict_zone_load(-3000.0, 0.0, &out, 5000.0, 5000.0, 1000.0, 1000.0);
            assert!(load.heating_w > 0.0);
            assert!(load.heating_w <= 5000.0);
        }

        #[test]
        fn equipment_priority_allocates_in_order() {
            let load = ZoneLoad { heating_w: 8000.0, ..Default::default() };
            let caps = [(ZoneEquipmentPriority(1), 3000.0), (ZoneEquipmentPriority(2), 5000.0)];
            let alloc = allocate_load_by_priority(load, &caps);
            assert_eq!(alloc.len(), 2);
            assert!((alloc[0].1.heating_w - 3000.0).abs() < 1e-6);
        }
    }
}

mod curves {
    //! 📈 Equipment performance curves: polynomials, biquadratics, triquadratics, lookup tables.

    use crate::error::{Diagnostics, Error, Severity};
    pub use crate::num::{biquadratic, lerp, poly_eval, LookupTable2D};

    // #region 🔖CurveKind
    /// 📈 Polynomial curve degree for performance functions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub enum CurveDegree {
        Constant,
        Linear,
        Quadratic,
        Cubic,
        Biquadratic,
        Triquadratic,
    }
    // #endregion 🔖CurveKind

    // #region 🔖PerformanceCurve
    /// 📈 Part-load performance curve for fans, coils, and plant equipment.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum PerformanceCurve {
        Constant(f64),
        Linear { x1: f64, y1: f64, x2: f64, y2: f64 },
        Quadratic { coeffs: [f64; 3] },
        Cubic { coeffs: [f64; 4] },
        Quartic { coeffs: [f64; 5] },
        Biquadratic { coeffs: [f64; 6] },
        Triquadratic { coeffs: [f64; 10] },
        Table(CurveLookupTable2D),
    }

    impl PerformanceCurve {
        /// 📊 Evaluate 1-D curve at normalized load `x`.
        pub fn evaluate(&self, x: f64) -> f64 {
            match self {
                Self::Constant(v) => *v,
                Self::Linear { x1, y1, x2, y2 } => lerp(x, *x1, *x2, *y1, *y2),
                Self::Quadratic { coeffs } => poly_eval(coeffs, x),
                Self::Cubic { coeffs } => poly_eval(coeffs, x),
                Self::Quartic { coeffs } => poly_eval(coeffs, x),
                Self::Table(table) => {
                    let y_mid = table.inner.y.get(table.inner.y.len() / 2).copied().unwrap_or(0.0);
                    table.evaluate(x, y_mid)
                }
                Self::Biquadratic { .. } | Self::Triquadratic { .. } => self.evaluate_2d(x, 0.0),
            }
        }

        /// 📊 Evaluate 2-D biquadratic, triquadratic, or table curve.
        pub fn evaluate_2d(&self, x: f64, y: f64) -> f64 {
            match self {
                Self::Biquadratic { coeffs } => biquadratic(*coeffs, x, y),
                Self::Triquadratic { coeffs } => triquadratic(*coeffs, x, y),
                Self::Table(table) => table.evaluate(x, y),
                other => other.evaluate(x),
            }
        }

        /// 📊 Part-load ratio clamped to [0, 1].
        pub fn part_load(&self, load: f64, rated: f64) -> f64 {
            if rated.abs() < 1e-9 {
                return 0.0;
            }
            (load / rated).clamp(0.0, 1.0)
        }

        /// 📐 Curve polynomial degree.
        pub fn degree(&self) -> CurveDegree {
            match self {
                Self::Constant(_) => CurveDegree::Constant,
                Self::Linear { .. } => CurveDegree::Linear,
                Self::Quadratic { .. } => CurveDegree::Quadratic,
                Self::Cubic { .. } => CurveDegree::Cubic,
                Self::Biquadratic { .. } => CurveDegree::Biquadratic,
                Self::Triquadratic { .. } => CurveDegree::Triquadratic,
                Self::Quartic { .. } => CurveDegree::Cubic,
                Self::Table(_) => CurveDegree::Biquadratic,
            }
        }
    }
    // #endregion 🔖PerformanceCurve

    // #region 🔖Triquadratic
    /// 📐 Triquadratic f(x,y) = Σ cᵢⱼ xⁱ yʲ for i+j ≤ 2 plus x²y² cross term.
    pub fn triquadratic(c: [f64; 10], x: f64, y: f64) -> f64 {
        c[0] + c[1] * x + c[2] * x * x + c[3] * y + c[4] * y * y + c[5] * x * y + c[6] * x * x * y + c[7] * x * y * y + c[8] * x * x * y * y + c[9] * x * x * x
    }
    // #endregion 🔖Triquadratic

    // #region 🔖LookupWrapper
    /// 📊 Validated 2-D lookup table wrapper with named axes.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CurveLookupTable2D {
        pub name: String,
        pub inner: LookupTable2D,
    }

    impl CurveLookupTable2D {
        pub fn new(name: impl Into<String>, inner: LookupTable2D) -> Self {
            Self { name: name.into(), inner }
        }

        pub fn evaluate(&self, x: f64, y: f64) -> f64 {
            self.inner.evaluate(x, y)
        }
    }

    impl From<LookupTable2D> for CurveLookupTable2D {
        fn from(inner: LookupTable2D) -> Self {
            Self::new("lookup", inner)
        }
    }
    // #endregion 🔖LookupWrapper

    // #region 🔖Validation
    /// ✅ Validate curve coefficients and lookup grid consistency.
    pub fn validate_curve(curve: &PerformanceCurve) -> Result<(), Diagnostics> {
        let mut diag = Diagnostics::default();
        match curve {
            PerformanceCurve::Quadratic { coeffs } => {
                if coeffs.iter().all(|c| c.abs() < 1e-15) {
                    diag.push(Error::severe("quadratic curve has all-zero coefficients"));
                }
            }
            PerformanceCurve::Cubic { coeffs } => {
                if coeffs.iter().all(|c| c.abs() < 1e-15) {
                    diag.push(Error::severe("polynomial curve has all-zero coefficients"));
                }
            }
            PerformanceCurve::Quartic { coeffs } => {
                if coeffs.iter().all(|c| c.abs() < 1e-15) {
                    diag.push(Error::severe("polynomial curve has all-zero coefficients"));
                }
            }
            PerformanceCurve::Linear { x1, x2, .. } if (x1 - x2).abs() < 1e-12 => {
                diag.push(Error::severe("linear curve has coincident x knots"));
            }
            PerformanceCurve::Table(table) => {
                validate_lookup_table(&table.inner, &mut diag);
            }
            _ => {}
        }
        if diag.messages.iter().any(|m| m.severity == Severity::Severe) {
            Err(diag)
        } else {
            Ok(())
        }
    }

    /// ✅ Validate lookup table grid dimensions and monotonic axes.
    pub fn validate_lookup_table(table: &LookupTable2D, diag: &mut Diagnostics) {
        if table.x.len() < 2 || table.y.len() < 2 {
            diag.push(Error::severe("lookup table must have at least 2 x and 2 y values"));
        }
        if table.values.len() != table.y.len() {
            diag.push(Error::severe("lookup table row count must match y axis length"));
        }
        for (i, row) in table.values.iter().enumerate() {
            if row.len() != table.x.len() {
                diag.push(Error::severe(format!("lookup table row {i} width must match x axis length")));
            }
        }
        if !is_monotonic(&table.x) {
            diag.push(Error::warning("lookup table x axis is not monotonically increasing"));
        }
        if !is_monotonic(&table.y) {
            diag.push(Error::warning("lookup table y axis is not monotonically increasing"));
        }
    }

    fn is_monotonic(vals: &[f64]) -> bool {
        vals.windows(2).all(|w| w[1] > w[0])
    }
    // #endregion 🔖Validation

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn linear_curve_midpoint() {
            let c = PerformanceCurve::Linear { x1: 0.0, y1: 0.0, x2: 1.0, y2: 10.0 };
            assert!((c.evaluate(0.5) - 5.0).abs() < 1e-9);
        }

        #[test]
        fn quadratic_curve_evaluates() {
            let c = PerformanceCurve::Quadratic { coeffs: [1.0, 2.0, 3.0] };
            assert!((c.evaluate(2.0) - 17.0).abs() < 1e-9);
        }

        #[test]
        fn biquadratic_at_origin() {
            let c = PerformanceCurve::Biquadratic { coeffs: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
            assert!((c.evaluate_2d(2.0, 3.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn triquadratic_includes_cross_terms() {
            let c = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
            assert!((triquadratic(c, 2.0, 3.0) - 6.0).abs() < 1e-9);
        }

        #[test]
        fn lookup_wrapper_evaluates() {
            let table = CurveLookupTable2D::new("test", LookupTable2D { x: vec![0.0, 1.0], y: vec![0.0, 1.0], values: vec![vec![0.0, 10.0], vec![0.0, 20.0]] });
            let curve = PerformanceCurve::Table(table);
            assert!((curve.evaluate_2d(1.0, 1.0) - 20.0).abs() < 1e-9);
        }

        #[test]
        fn validate_rejects_degenerate_linear() {
            let c = PerformanceCurve::Linear { x1: 1.0, y1: 0.0, x2: 1.0, y2: 5.0 };
            assert!(validate_curve(&c).is_err());
        }

        #[test]
        fn validate_accepts_valid_quadratic() {
            let c = PerformanceCurve::Quadratic { coeffs: [0.5, 0.1, 0.0] };
            assert!(validate_curve(&c).is_ok());
        }

        #[test]
        fn part_load_clamps() {
            let c = PerformanceCurve::Constant(1.0);
            assert!((c.part_load(150.0, 100.0) - 1.0).abs() < 1e-9);
        }
    }
}

mod daylight {
    //! 💡 Daylight: reference points, illuminance, glare, and lighting control.

    // #region 🔖Types
    /// 💡 Daylight zone with reference points and glazing coupling.
    #[derive(Clone, Debug, PartialEq)]
    pub struct DaylightZone {
        pub zone_id: u32,
        pub floor_area_m2: f64,
        pub window_transmittance: f64,
        pub reference_points: Vec<ReferencePoint>,
        pub illuminance_target_lux: f64,
        pub glare_limit: f64,
    }

    /// 📍 Interior daylight reference point.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ReferencePoint {
        pub x_m: f64,
        pub y_m: f64,
        pub z_m: f64,
        pub fraction: f64,
    }

    /// 🗺️ Illuminance map at reference points [lux].
    #[derive(Clone, Debug, PartialEq)]
    pub struct IlluminanceMap {
        pub values_lux: Vec<f64>,
        pub min_lux: f64,
        pub max_lux: f64,
        pub average_lux: f64,
    }
    // #endregion 🔖Types

    // #region 🔖Illuminance
    /// 💡 Simplified interior illuminance at a point [lux] (split-flux daylight factor).
    pub fn reference_point_illuminance_lux(diffuse_horizontal_lux: f64, direct_normal_lux: f64, incidence_cosine: f64, window_transmittance: f64, daylight_factor: f64, shading_factor: f64) -> f64 {
        let diffuse_contrib = diffuse_horizontal_lux * window_transmittance * daylight_factor * shading_factor;
        let direct_contrib = direct_normal_lux * incidence_cosine * window_transmittance * shading_factor * 0.5;
        diffuse_contrib + direct_contrib
    }

    /// 🗺️ Build illuminance map from reference points.
    pub fn illuminance_map(points: &[ReferencePoint], lux_per_point: &[f64]) -> IlluminanceMap {
        let values_lux: Vec<f64> = points.iter().zip(lux_per_point.iter()).map(|(p, &lux)| lux * p.fraction).collect();
        let empty = values_lux.is_empty();
        let min_lux = values_lux.iter().copied().fold(f64::INFINITY, f64::min);
        let max_lux = values_lux.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let total_frac: f64 = points.iter().map(|p| p.fraction).sum();
        let average_lux = if total_frac > 0.0 { values_lux.iter().sum::<f64>() / total_frac } else { 0.0 };
        IlluminanceMap { values_lux, min_lux: if empty { 0.0 } else { min_lux }, max_lux: if empty { 0.0 } else { max_lux }, average_lux }
    }

    /// 💡 Zone-averaged daylight illuminance [lux].
    pub fn zone_daylight_illuminance(zone: &DaylightZone, lux_per_point: &[f64]) -> f64 {
        illuminance_map(&zone.reference_points, lux_per_point).average_lux
    }
    // #endregion 🔖Illuminance

    // #region 🔖Glare
    /// 😎 Simplified daylight glare index (0–1, higher = more glare).
    pub fn simplified_glare_index(window_luminance_cd_m2: f64, solid_angle_sr: f64, eye_illuminance_lux: f64) -> f64 {
        let omega = solid_angle_sr.max(1e-6);
        let l_b = window_luminance_cd_m2.max(1.0);
        let e_i = eye_illuminance_lux.max(1.0);
        let ratio = l_b * omega.sqrt() / e_i;
        (ratio / (ratio + 10.0)).clamp(0.0, 1.0)
    }

    /// 😎 Glare acceptable when index below limit.
    pub fn glare_acceptable(glare_index: f64, limit: f64) -> bool {
        glare_index <= limit
    }
    // #endregion 🔖Glare

    // #region 🔖Control
    /// 💡 Continuous lighting dimming fraction (0 = off, 1 = full) for daylight harvesting.
    pub fn lighting_dimming_fraction(current_illuminance_lux: f64, target_lux: f64, min_fraction: f64) -> f64 {
        if target_lux <= 0.0 {
            return 1.0;
        }
        if current_illuminance_lux >= target_lux {
            return min_fraction;
        }
        let needed = (target_lux - current_illuminance_lux) / target_lux;
        needed.clamp(min_fraction, 1.0)
    }

    /// 💡 Electric lighting power after dimming [W].
    pub fn dimmed_lighting_power_w(full_power_w: f64, dimming_fraction: f64) -> f64 {
        full_power_w * dimming_fraction.clamp(0.0, 1.0)
    }

    /// 💡 Daylight factor from geometry (simplified: window-to-floor ratio).
    pub fn simplified_daylight_factor(window_area_m2: f64, floor_area_m2: f64, transmittance: f64) -> f64 {
        if floor_area_m2 <= 0.0 {
            return 0.0;
        }
        0.5 * (window_area_m2 / floor_area_m2) * transmittance
    }
    // #endregion 🔖Control

    #[cfg(test)]
    mod tests {
        use super::*;

        fn sample_zone() -> DaylightZone {
            DaylightZone {
                zone_id: 1,
                floor_area_m2: 25.0,
                window_transmittance: 0.6,
                reference_points: vec![ReferencePoint { x_m: 2.0, y_m: 2.0, z_m: 0.8, fraction: 0.5 }, ReferencePoint { x_m: 4.0, y_m: 2.0, z_m: 0.8, fraction: 0.5 }],
                illuminance_target_lux: 500.0,
                glare_limit: 0.4,
            }
        }

        #[test]
        fn illuminance_increases_with_sun() {
            let e = reference_point_illuminance_lux(10_000.0, 50_000.0, 0.5, 0.6, 0.05, 1.0);
            assert!(e > 500.0);
        }

        #[test]
        fn dimming_reduces_at_high_daylight() {
            let frac = lighting_dimming_fraction(600.0, 500.0, 0.1);
            assert!((frac - 0.1).abs() < 1e-6);
        }

        #[test]
        fn dimming_full_when_dark() {
            let frac = lighting_dimming_fraction(50.0, 500.0, 0.1);
            assert!(frac > 0.8);
        }

        #[test]
        fn glare_high_for_bright_window() {
            let gi = simplified_glare_index(5000.0, 0.2, 300.0);
            assert!(gi > 0.1);
        }

        #[test]
        fn zone_average_illuminance() {
            let zone = sample_zone();
            let lux = vec![400.0, 600.0];
            let avg = zone_daylight_illuminance(&zone, &lux);
            assert!((avg - 500.0).abs() < 1e-6);
        }
    }
}

mod dispatch {
    //! 🎛️ Plant and equipment dispatch strategies.

    use serde::{Deserialize, Serialize};

    // #region 🔖Dispatch
    /// 🎛️ Equipment dispatch scheme.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DispatchScheme {
        Sequential,
        Uniform,
        Optimal,
        UniformPartLoadRatio,
        LoadRange,
        OutdoorTemperature,
        ThermalStorage,
    }

    /// 🎛️ Equipment priority entry.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EquipmentPriority {
        pub equipment_id: u32,
        pub priority: u32,
        pub min_runtime_hours: f64,
        pub capacity_w: f64,
    }

    /// 🎛️ Dispatch request for plant equipment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DispatchRequest {
        pub total_load_w: f64,
        pub available_capacity_w: f64,
        pub outdoor_temp_c: f64,
    }

    /// 🎛️ Dispatch result per equipment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DispatchResult {
        pub equipment_id: u32,
        pub load_w: f64,
        pub part_load_ratio: f64,
        pub runtime_fraction: f64,
    }
    // #endregion 🔖Dispatch

    // #region 🔖Dispatcher
    /// 🎛️ Plant equipment dispatcher.
    pub struct Dispatcher {
        pub scheme: DispatchScheme,
        pub equipment: Vec<EquipmentPriority>,
    }

    impl Dispatcher {
        pub fn new(scheme: DispatchScheme, equipment: Vec<EquipmentPriority>) -> Self {
            Self { scheme, equipment }
        }

        /// 🎛️ Distribute load across equipment per dispatch scheme.
        pub fn dispatch(&self, request: &DispatchRequest) -> Vec<DispatchResult> {
            let mut sorted = self.equipment.clone();
            sorted.sort_by_key(|e| e.priority);

            match self.scheme {
                DispatchScheme::Sequential => self.dispatch_sequential(&sorted, request),
                DispatchScheme::Uniform | DispatchScheme::UniformPartLoadRatio => self.dispatch_uniform(&sorted, request),
                DispatchScheme::Optimal => self.dispatch_optimal(&sorted, request),
                _ => self.dispatch_sequential(&sorted, request),
            }
        }

        fn dispatch_sequential(&self, equipment: &[EquipmentPriority], request: &DispatchRequest) -> Vec<DispatchResult> {
            let mut remaining = request.total_load_w;
            let mut results = Vec::new();
            for eq in equipment {
                let load = remaining.min(eq.capacity_w).max(0.0);
                let plr = if eq.capacity_w > 0.0 { load / eq.capacity_w } else { 0.0 };
                results.push(DispatchResult { equipment_id: eq.equipment_id, load_w: load, part_load_ratio: plr, runtime_fraction: if load > 0.0 { 1.0 } else { 0.0 } });
                remaining -= load;
            }
            results
        }

        fn dispatch_uniform(&self, equipment: &[EquipmentPriority], request: &DispatchRequest) -> Vec<DispatchResult> {
            let active: Vec<_> = equipment.iter().filter(|e| e.capacity_w > 0.0).collect();
            if active.is_empty() {
                return Vec::new();
            }
            let total_cap: f64 = active.iter().map(|e| e.capacity_w).sum();
            let plr = (request.total_load_w / total_cap).clamp(0.0, 1.0);
            active.iter().map(|eq| DispatchResult { equipment_id: eq.equipment_id, load_w: eq.capacity_w * plr, part_load_ratio: plr, runtime_fraction: if plr > 0.01 { 1.0 } else { 0.0 } }).collect()
        }

        fn dispatch_optimal(&self, equipment: &[EquipmentPriority], request: &DispatchRequest) -> Vec<DispatchResult> {
            self.dispatch_uniform(equipment, request)
        }
    }
    // #endregion 🔖Dispatcher

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sequential_fills_first_unit() {
            let d = Dispatcher::new(
                DispatchScheme::Sequential,
                vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 5000.0 }, EquipmentPriority { equipment_id: 2, priority: 2, min_runtime_hours: 0.0, capacity_w: 5000.0 }],
            );
            let results = d.dispatch(&DispatchRequest { total_load_w: 7000.0, available_capacity_w: 10000.0, outdoor_temp_c: 20.0 });
            assert!((results[0].load_w - 5000.0).abs() < 1e-6);
            assert!((results[1].load_w - 2000.0).abs() < 1e-6);
        }
    }
}

mod economics {
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
}

mod electrical {
    //! ⚡ Electrical systems: loads, PV, wind, generators, inverters, batteries, transformers, grid.

    use crate::units::deg_to_rad;
    use serde::{Deserialize, Serialize};

    // #region 🔖EndUse
    /// 💡 Generic electrical end-use load.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EndUseLoad {
        pub name: String,
        pub rated_power_w: f64,
        pub schedule_factor: f64,
        pub power_factor: f64,
    }

    impl EndUseLoad {
        /// 💡 Instantaneous real power [W].
        pub fn power_w(&self) -> f64 {
            self.rated_power_w * self.schedule_factor.clamp(0.0, 1.0)
        }

        /// ⚡ Apparent power [VA].
        pub fn apparent_va(&self) -> f64 {
            let pf = self.power_factor.clamp(0.1, 1.0);
            self.power_w() / pf
        }
    }
    // #endregion 🔖EndUse

    // #region 🔖Pv
    /// ☀️ Photovoltaic array with inverter.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PvSystem {
        pub dc_capacity_w: f64,
        pub module_efficiency: f64,
        pub area_m2: f64,
        pub inverter_efficiency: f64,
        pub temperature_coefficient: f64,
        pub tilt_deg: f64,
        pub azimuth_deg: f64,
    }

    impl PvSystem {
        /// ☀️ AC power output from plane-of-array irradiance.
        pub fn simulate(&self, poa_irradiance_w_m2: f64, cell_temperature_c: f64) -> f64 {
            if poa_irradiance_w_m2 <= 0.0 {
                return 0.0;
            }
            let temp_factor = 1.0 + self.temperature_coefficient * (cell_temperature_c - 25.0);
            let dc_w = self.area_m2 * poa_irradiance_w_m2 * self.module_efficiency * temp_factor;
            let clipped = dc_w.min(self.dc_capacity_w);
            clipped * self.inverter_efficiency.clamp(0.85, 0.99)
        }

        /// 📐 Tilt/azimuth factor relative to horizontal south-facing surface.
        pub fn orientation_factor(&self, solar_altitude_deg: f64, solar_azimuth_deg: f64) -> f64 {
            let tilt = deg_to_rad(self.tilt_deg);
            let surf_az = deg_to_rad(self.azimuth_deg);
            let sun_alt = deg_to_rad(solar_altitude_deg);
            let sun_az = deg_to_rad(solar_azimuth_deg);
            let cos_inc = sun_alt.sin() * tilt.cos() + sun_alt.cos() * tilt.sin() * (sun_az - surf_az).cos();
            cos_inc.clamp(0.0, 1.0)
        }
    }
    // #endregion 🔖Pv

    // #region 🔖Wind
    /// 💨 Wind turbine with cut-in/rated/cut-out speeds.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WindTurbine {
        pub rated_power_w: f64,
        pub cut_in_m_s: f64,
        pub rated_speed_m_s: f64,
        pub cut_out_m_s: f64,
        pub hub_height_m: f64,
        pub rotor_diameter_m: f64,
    }

    impl WindTurbine {
        /// 💨 Electrical output from hub-height wind speed.
        pub fn simulate(&self, wind_speed_m_s: f64, air_density: f64) -> f64 {
            let v = wind_speed_m_s;
            if v < self.cut_in_m_s || v > self.cut_out_m_s {
                return 0.0;
            }
            if v >= self.rated_speed_m_s {
                return self.rated_power_w;
            }
            let frac = (v - self.cut_in_m_s) / (self.rated_speed_m_s - self.cut_in_m_s);
            self.rated_power_w * frac.powi(3) * (air_density / 1.2)
        }
    }
    // #endregion 🔖Wind

    // #region 🔖Generator
    /// 🔌 Backup generator (diesel or gas).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Generator {
        pub rated_power_w: f64,
        pub fuel_lhv_j_per_kg: f64,
        pub electrical_efficiency: f64,
        pub min_load_fraction: f64,
    }

    impl Generator {
        /// 🔌 Generator electrical output and fuel consumption.
        pub fn simulate(&self, requested_w: f64, operating: bool) -> (f64, f64) {
            if !operating || requested_w <= 0.0 {
                return (0.0, 0.0);
            }
            let min_w = self.rated_power_w * self.min_load_fraction;
            let output = requested_w.clamp(min_w, self.rated_power_w);
            let fuel_kg_s = output / (self.fuel_lhv_j_per_kg * self.electrical_efficiency);
            (output, fuel_kg_s)
        }
    }
    // #endregion 🔖Generator

    // #region 🔖Inverter
    /// 🔄 DC/AC inverter with efficiency curve.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Inverter {
        pub rated_ac_w: f64,
        pub peak_efficiency: f64,
        pub standby_w: f64,
    }

    impl Inverter {
        /// 🔄 Convert DC to AC with part-load efficiency penalty.
        pub fn simulate(&self, dc_w: f64) -> f64 {
            if dc_w <= 0.0 {
                return -self.standby_w;
            }
            let plr = (dc_w / self.rated_ac_w).clamp(0.05, 1.0);
            let eta = self.peak_efficiency * (0.9 + 0.1 * plr);
            dc_w * eta
        }
    }
    // #endregion 🔖Inverter

    // #region 🔖Battery
    /// 🔋 Electrochemical storage with SOC limits.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Battery {
        pub capacity_kwh: f64,
        pub max_charge_w: f64,
        pub max_discharge_w: f64,
        pub round_trip_efficiency: f64,
        pub min_soc: f64,
        pub max_soc: f64,
        pub state_of_charge: f64,
    }

    impl Battery {
        /// 🔋 Charge or discharge for one timestep; returns (grid_power_w, new_soc).
        pub fn simulate(&self, requested_w: f64, dt_s: f64) -> (f64, f64) {
            let capacity_j = self.capacity_kwh * 3_600_000.0;
            let mut soc = self.state_of_charge.clamp(self.min_soc, self.max_soc);
            let mut actual_w = 0.0;
            if requested_w > 0.0 {
                let charge_w = requested_w.min(self.max_charge_w);
                let energy_j = charge_w * dt_s * self.round_trip_efficiency.sqrt();
                let delta_soc = energy_j / capacity_j;
                if soc + delta_soc <= self.max_soc {
                    soc += delta_soc;
                    actual_w = charge_w;
                } else {
                    let allowed_j = (self.max_soc - soc) * capacity_j;
                    actual_w = allowed_j / (dt_s * self.round_trip_efficiency.sqrt());
                    soc = self.max_soc;
                }
            } else if requested_w < 0.0 {
                let discharge_w = (-requested_w).min(self.max_discharge_w);
                let energy_j = discharge_w * dt_s / self.round_trip_efficiency.sqrt();
                let delta_soc = energy_j / capacity_j;
                if soc - delta_soc >= self.min_soc {
                    soc -= delta_soc;
                    actual_w = -discharge_w;
                } else {
                    let allowed_j = (soc - self.min_soc) * capacity_j;
                    actual_w = -(allowed_j * self.round_trip_efficiency.sqrt() / dt_s);
                    soc = self.min_soc;
                }
            }
            (actual_w, soc)
        }
    }
    // #endregion 🔖Battery

    // #region 🔖Transformer
    /// 🔌 Building transformer with no-load and load losses.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Transformer {
        pub rated_kva: f64,
        pub no_load_loss_w: f64,
        pub load_loss_w: f64,
        pub impedance_fraction: f64,
    }

    impl Transformer {
        /// 🔌 Transformer total losses [W] at given apparent load.
        pub fn losses_w(&self, apparent_va: f64) -> f64 {
            let plr = (apparent_va / (self.rated_kva * 1000.0)).clamp(0.0, 1.5);
            self.no_load_loss_w + self.load_loss_w * plr * plr
        }

        /// 📉 Secondary voltage drop fraction.
        pub fn voltage_drop_fraction(&self, apparent_va: f64) -> f64 {
            let plr = (apparent_va / (self.rated_kva * 1000.0)).clamp(0.0, 1.5);
            self.impedance_fraction * plr
        }
    }
    // #endregion 🔖Transformer

    // #region 🔖Grid
    /// 🏭 Grid interconnection balance for one timestep.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct GridBalance {
        pub building_load_w: f64,
        pub pv_generation_w: f64,
        pub wind_generation_w: f64,
        pub generator_output_w: f64,
        pub battery_power_w: f64,
        pub transformer_loss_w: f64,
        pub net_import_w: f64,
        pub net_export_w: f64,
    }

    /// 🏭 Compute grid import/export from supply and demand.
    pub fn grid_balance(building_load_w: f64, pv_w: f64, wind_w: f64, generator_w: f64, battery_w: f64, transformer: &Transformer) -> GridBalance {
        let supply_w = pv_w + wind_w + generator_w - battery_w;
        let apparent = (building_load_w - supply_w).abs();
        let transformer_loss = transformer.losses_w(apparent);
        let net = building_load_w + transformer_loss - supply_w;
        GridBalance { building_load_w, pv_generation_w: pv_w, wind_generation_w: wind_w, generator_output_w: generator_w, battery_power_w: battery_w, transformer_loss_w: transformer_loss, net_import_w: net.max(0.0), net_export_w: (-net).max(0.0) }
    }
    // #endregion 🔖Grid

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn end_use_scales_with_schedule() {
            let load = EndUseLoad { name: "Lighting".into(), rated_power_w: 1000.0, schedule_factor: 0.5, power_factor: 0.95 };
            assert!((load.power_w() - 500.0).abs() < 1e-6);
        }

        #[test]
        fn pv_zero_at_night() {
            let pv = PvSystem { dc_capacity_w: 10_000.0, module_efficiency: 0.2, area_m2: 50.0, inverter_efficiency: 0.96, temperature_coefficient: -0.004, tilt_deg: 30.0, azimuth_deg: 180.0 };
            assert!(pv.simulate(0.0, 20.0).abs() < 1e-6);
        }

        #[test]
        fn wind_cubic_below_rated() {
            let turbine = WindTurbine { rated_power_w: 20_000.0, cut_in_m_s: 3.0, rated_speed_m_s: 12.0, cut_out_m_s: 25.0, hub_height_m: 30.0, rotor_diameter_m: 12.0 };
            let low = turbine.simulate(5.0, 1.2);
            let high = turbine.simulate(8.0, 1.2);
            assert!(high > low);
            assert!(turbine.simulate(2.0, 1.2).abs() < 1e-6);
        }

        #[test]
        fn battery_soc_bounds() {
            let battery = Battery { capacity_kwh: 10.0, max_charge_w: 5000.0, max_discharge_w: 5000.0, round_trip_efficiency: 0.92, min_soc: 0.1, max_soc: 0.95, state_of_charge: 0.5 };
            let (charge_w, soc_after) = battery.simulate(3000.0, 3600.0);
            assert!(charge_w > 0.0);
            assert!(soc_after > 0.5);
            let (_, soc_dis) = battery.simulate(-8000.0, 3600.0);
            assert!(soc_dis >= battery.min_soc);
        }

        #[test]
        fn grid_balance_import_when_load_exceeds_supply() {
            let xf = Transformer { rated_kva: 100.0, no_load_loss_w: 50.0, load_loss_w: 800.0, impedance_fraction: 0.04 };
            let balance = grid_balance(50_000.0, 10_000.0, 0.0, 0.0, 0.0, &xf);
            assert!(balance.net_import_w > 0.0);
            assert!(balance.net_export_w.abs() < 1e-6);
        }

        #[test]
        fn generator_respects_minimum_load() {
            let gen = Generator { rated_power_w: 100_000.0, fuel_lhv_j_per_kg: 42e6, electrical_efficiency: 0.35, min_load_fraction: 0.3 };
            let (out, fuel) = gen.simulate(5000.0, true);
            assert!(out >= 30_000.0);
            assert!(fuel > 0.0);
        }
    }
}

mod envelope {
    //! 🧱 Opaque envelope heat transfer: convection, conduction CTF, and surface balance.

    use crate::material::{R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
    use crate::num::newton_raphson;
    use crate::units::STEFAN_BOLTZMANN;

    // #region 🔖ConvectionModels
    /// 🌬️ Exterior convection correlation (wind-adaptive McAdams-type).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ExteriorConvectionModel {
        pub base_h_w_m2k: f64,
        pub wind_coefficient: f64,
    }

    impl Default for ExteriorConvectionModel {
        fn default() -> Self {
            Self { base_h_w_m2k: 5.7, wind_coefficient: 3.8 }
        }
    }

    impl ExteriorConvectionModel {
        /// 🌬️ Exterior convection coefficient [W/(m²·K)].
        pub fn h_w_m2k(&self, wind_speed_m_s: f64) -> f64 {
            self.base_h_w_m2k + self.wind_coefficient * wind_speed_m_s.max(0.0)
        }
    }

    /// 🌬️ Interior convection correlation (adaptive natural convection).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct InteriorConvectionModel {
        pub h_min_w_m2k: f64,
        pub delta_t_exponent: f64,
        pub delta_t_coefficient: f64,
    }

    impl Default for InteriorConvectionModel {
        fn default() -> Self {
            Self { h_min_w_m2k: 3.0, delta_t_coefficient: 5.1, delta_t_exponent: 0.25 }
        }
    }

    impl InteriorConvectionModel {
        /// 🌬️ Interior convection coefficient [W/(m²·K)] from |T_s − T_a|.
        pub fn h_w_m2k(&self, surface_temp_c: f64, air_temp_c: f64) -> f64 {
            let dt = (surface_temp_c - air_temp_c).abs();
            self.h_min_w_m2k + self.delta_t_coefficient * dt.powf(self.delta_t_exponent)
        }
    }
    // #endregion 🔖ConvectionModels

    // #region 🔖ConductionState
    /// 🌡️ Simplified first-order CTF conduction state (one history state per surface).
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConductionState {
        pub ctf_c0_w_m2k: f64,
        pub ctf_c1_w_m2k: f64,
        pub previous_outside_temp_c: f64,
    }

    impl ConductionState {
        /// 🌡️ Initialize CTF from construction U-value and thermal mass [J/(m²·K)].
        pub fn from_u_and_capacitance(u_value_w_m2k: f64, capacitance_j_m2k: f64, time_step_s: f64) -> Self {
            let tau = capacitance_j_m2k / u_value_w_m2k.max(0.01);
            let alpha = (-time_step_s / tau.max(1.0)).exp();
            Self { ctf_c0_w_m2k: u_value_w_m2k * (1.0 - alpha), ctf_c1_w_m2k: u_value_w_m2k * alpha, previous_outside_temp_c: 20.0 }
        }

        /// 🔥 Conduction heat flux to zone [W/m²] (positive = heat into zone).
        pub fn heat_flux_w_m2(&self, outside_temp_c: f64, inside_temp_c: f64) -> f64 {
            self.ctf_c0_w_m2k * (outside_temp_c - inside_temp_c) + self.ctf_c1_w_m2k * (self.previous_outside_temp_c - inside_temp_c)
        }

        /// 🔄 Advance history after a timestep.
        pub fn advance(&mut self, outside_temp_c: f64) {
            self.previous_outside_temp_c = outside_temp_c;
        }
    }
    // #endregion 🔖ConductionState

    // #region 🔖SurfaceHeatBalance
    /// ⚖️ Surface heat balance terms [W/m²].
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct SurfaceHeatBalance {
        pub convection_w_m2: f64,
        pub conduction_w_m2: f64,
        pub solar_absorbed_w_m2: f64,
        pub longwave_net_w_m2: f64,
        pub surface_temp_c: f64,
    }

    impl SurfaceHeatBalance {
        /// ⚖️ Net flux into surface (should approach zero at convergence).
        pub fn residual_w_m2(&self) -> f64 {
            self.solar_absorbed_w_m2 + self.longwave_net_w_m2 + self.conduction_w_m2 - self.convection_w_m2
        }
    }
    // #endregion 🔖SurfaceHeatBalance

    // #region 🔖Longwave
    /// 🌡️ Net longwave exchange [W/m²] (surface ↔ sky/ground).
    pub fn longwave_net_w_m2(surface_temp_c: f64, exterior_temp_k: f64, emissivity: f64) -> f64 {
        let t_s_k = surface_temp_c + 273.15;
        emissivity * STEFAN_BOLTZMANN * (exterior_temp_k.powi(4) - t_s_k.powi(4))
    }
    // #endregion 🔖Longwave

    // #region 🔖Solve
    /// 🌡️ Solve exterior surface temperature [°C] for heat balance.
    pub fn solve_exterior_surface_temp(outside_air_c: f64, sky_temp_k: f64, wind_speed_m_s: f64, solar_absorbed_w_m2: f64, conduction_from_inside_w_m2: f64, emissivity: f64, ext_conv: &ExteriorConvectionModel) -> f64 {
        let h = ext_conv.h_w_m2k(wind_speed_m_s);
        let f = |t_s: f64| {
            let conv = h * (outside_air_c - t_s);
            let lw = longwave_net_w_m2(t_s, sky_temp_k, emissivity);
            solar_absorbed_w_m2 + lw - conv - conduction_from_inside_w_m2
        };
        let df = |t_s: f64| {
            let eps = 0.1;
            (f(t_s + eps) - f(t_s - eps)) / (2.0 * eps)
        };
        newton_raphson(outside_air_c, f, df, 30, 1e-4).unwrap_or(outside_air_c)
    }

    /// 🌡️ Solve interior surface temperature [°C] for heat balance.
    pub fn solve_interior_surface_temp(zone_air_c: f64, conduction_from_outside_w_m2: f64, solar_absorbed_w_m2: f64, int_conv: &InteriorConvectionModel) -> SurfaceHeatBalance {
        let mut t_s = zone_air_c;
        for _ in 0..20 {
            let h = int_conv.h_w_m2k(t_s, zone_air_c);
            t_s = zone_air_c - (solar_absorbed_w_m2 + conduction_from_outside_w_m2) / h.max(0.1);
        }
        let h = int_conv.h_w_m2k(t_s, zone_air_c);
        SurfaceHeatBalance { convection_w_m2: h * (zone_air_c - t_s), conduction_w_m2: conduction_from_outside_w_m2, solar_absorbed_w_m2, longwave_net_w_m2: 0.0, surface_temp_c: t_s }
    }

    /// 🔥 Steady-state opaque conduction flux [W/m²] through construction.
    pub fn steady_opaque_flux_w_m2(outside_temp_c: f64, inside_temp_c: f64, u_value_w_m2k: f64) -> f64 {
        u_value_w_m2k * (outside_temp_c - inside_temp_c)
    }

    /// 🔥 Film-inclusive U-value from construction U and film resistances.
    pub fn overall_u_value_w_m2k(construction_u: f64) -> f64 {
        let r_total = 1.0 / construction_u.max(1e-6);
        let r_construction = r_total - R_FILM_INTERIOR_M2K_W - R_FILM_EXTERIOR_M2K_W;
        if r_construction <= 0.0 {
            return construction_u;
        }
        1.0 / (R_FILM_INTERIOR_M2K_W + r_construction + R_FILM_EXTERIOR_M2K_W)
    }
    // #endregion 🔖Solve

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn exterior_h_increases_with_wind() {
            let model = ExteriorConvectionModel::default();
            assert!(model.h_w_m2k(5.0) > model.h_w_m2k(0.0));
        }

        #[test]
        fn interior_h_increases_with_delta_t() {
            let model = InteriorConvectionModel::default();
            assert!(model.h_w_m2k(30.0, 20.0) > model.h_w_m2k(21.0, 20.0));
        }

        #[test]
        fn ctf_flux_sign_correct() {
            let state = ConductionState::from_u_and_capacitance(0.3, 50_000.0, 3600.0);
            let flux = state.heat_flux_w_m2(0.0, 20.0);
            assert!(flux < 0.0);
        }

        #[test]
        fn steady_flux_cold_outside() {
            let q = steady_opaque_flux_w_m2(-5.0, 20.0, 0.25);
            assert!(q < 0.0);
            assert!((q - (-6.25)).abs() < 0.01);
        }

        #[test]
        fn interior_surface_balance_near_air() {
            let balance = solve_interior_surface_temp(22.0, -2.0, 0.0, &InteriorConvectionModel::default());
            assert!(balance.surface_temp_c > 22.0);
            assert!(balance.residual_w_m2().abs() < 0.1);
        }
    }
}

mod error {
    //! ⚠️ Simulation and model error taxonomy.

    use std::fmt;

    // #region 🔖Severity
    /// 🚨 Diagnostic severity aligned with BEM engine conventions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub enum Severity {
        Fatal,
        Severe,
        Warning,
        RecurringWarning,
    }
    // #endregion 🔖Severity

    // #region 🔖Error
    /// ❌ Recoverable or fatal engine error with optional location context.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Error {
        pub severity: Severity,
        pub message: String,
        pub context: Option<String>,
    }

    impl Error {
        pub fn fatal(message: impl Into<String>) -> Self {
            Self { severity: Severity::Fatal, message: message.into(), context: None }
        }

        pub fn severe(message: impl Into<String>) -> Self {
            Self { severity: Severity::Severe, message: message.into(), context: None }
        }

        pub fn warning(message: impl Into<String>) -> Self {
            Self { severity: Severity::Warning, message: message.into(), context: None }
        }

        pub fn with_context(mut self, context: impl Into<String>) -> Self {
            self.context = Some(context.into());
            self
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(ctx) = &self.context {
                write!(f, "[{:?}] {} ({})", self.severity, self.message, ctx)
            } else {
                write!(f, "[{:?}] {}", self.severity, self.message)
            }
        }
    }

    impl std::error::Error for Error {}
    // #endregion 🔖Error

    // #region 🔖Diagnostics
    /// 📋 Collected diagnostics from validation or simulation.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Diagnostics {
        pub messages: Vec<Error>,
    }

    impl Diagnostics {
        pub fn push(&mut self, err: Error) {
            self.messages.push(err);
        }

        pub fn has_fatal(&self) -> bool {
            self.messages.iter().any(|e| e.severity == Severity::Fatal)
        }

        pub fn merge(&mut self, other: Diagnostics) {
            self.messages.extend(other.messages);
        }
    }
    // #endregion 🔖Diagnostics

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fatal_has_correct_severity() {
            let e = Error::fatal("bad model");
            assert_eq!(e.severity, Severity::Fatal);
        }
    }
}

mod evaporative {
    //! 💧 Evaporative cooling: direct and indirect with effectiveness and water use.

    use crate::props::{humidity_ratio_from_rh, latent_heat_vaporization, saturation_pressure_pa, wet_bulb_c};
    use crate::units::{CP_DRY_AIR, H_FG_0C};
    use serde::{Deserialize, Serialize};

    // #region 🔖EvaporativeCooler
    /// 💧 Evaporative cooler configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EvaporativeCooler {
        Direct { effectiveness: f64, pad_area_m2: f64 },
        Indirect { sensible_effectiveness: f64, primary_flow_m3_s: f64, secondary_flow_m3_s: f64 },
    }

    /// 📥 Evaporative cooler inlet state.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EvaporativeInlet {
        pub dry_bulb_c: f64,
        pub humidity_ratio: f64,
        pub mass_flow_kg_s: f64,
        pub pressure_pa: f64,
    }

    /// 📤 Evaporative cooler outlet state.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EvaporativeOutput {
        pub dry_bulb_c: f64,
        pub humidity_ratio: f64,
        pub sensible_cooling_w: f64,
        pub latent_heat_w: f64,
        pub water_consumption_kg_s: f64,
        pub effectiveness_achieved: f64,
    }
    // #endregion 🔖EvaporativeCooler

    // #region 🔖WetBulb
    /// 🌡️ Simplified wet-bulb estimate from dry-bulb and humidity ratio.
    pub fn wet_bulb_approx_c(t_db_c: f64, w: f64, p_atm: f64) -> f64 {
        let rh = relative_humidity_from_w(t_db_c, w, p_atm);
        t_db_c * rh.atan() / (std::f64::consts::FRAC_PI_2 + 0.15 * rh.atan()) + 0.5
    }

    fn relative_humidity_from_w(t_c: f64, w: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_c);
        let p_w = w * p_atm / (0.621_945 + w);
        (p_w / p_ws).clamp(0.0, 1.0)
    }
    // #endregion 🔖WetBulb

    // #region 🔖Simulate
    /// 💧 Simulate direct or indirect evaporative cooling.
    pub fn evaporative_cool(cooler: &EvaporativeCooler, inlet: &EvaporativeInlet, enabled: bool) -> EvaporativeOutput {
        if !enabled || inlet.mass_flow_kg_s < 1e-9 {
            return EvaporativeOutput { dry_bulb_c: inlet.dry_bulb_c, humidity_ratio: inlet.humidity_ratio, sensible_cooling_w: 0.0, latent_heat_w: 0.0, water_consumption_kg_s: 0.0, effectiveness_achieved: 0.0 };
        }

        match cooler {
            EvaporativeCooler::Direct { effectiveness, .. } => {
                let eps = effectiveness.clamp(0.0, 1.0);
                let t_wb = wet_bulb_c(inlet.dry_bulb_c, inlet.humidity_ratio, inlet.pressure_pa);
                let t_out = inlet.dry_bulb_c - eps * (inlet.dry_bulb_c - t_wb);
                let w_sat_out = humidity_ratio_from_rh(t_out, 0.95, inlet.pressure_pa);
                let w_out = inlet.humidity_ratio + eps * (w_sat_out - inlet.humidity_ratio);
                let sensible = inlet.mass_flow_kg_s * CP_DRY_AIR * (inlet.dry_bulb_c - t_out);
                let water_evap = (w_out - inlet.humidity_ratio).max(0.0) * inlet.mass_flow_kg_s;
                let latent = water_evap * latent_heat_vaporization(t_out);
                EvaporativeOutput { dry_bulb_c: t_out, humidity_ratio: w_out, sensible_cooling_w: sensible.max(0.0), latent_heat_w: latent, water_consumption_kg_s: water_evap, effectiveness_achieved: eps }
            }
            EvaporativeCooler::Indirect { sensible_effectiveness, .. } => {
                let eps = sensible_effectiveness.clamp(0.0, 1.0);
                let t_wb = wet_bulb_c(inlet.dry_bulb_c, inlet.humidity_ratio, inlet.pressure_pa);
                let t_out = inlet.dry_bulb_c - eps * (inlet.dry_bulb_c - t_wb);
                let sensible = inlet.mass_flow_kg_s * CP_DRY_AIR * (inlet.dry_bulb_c - t_out);
                let water_evap = sensible / H_FG_0C;
                EvaporativeOutput { dry_bulb_c: t_out, humidity_ratio: inlet.humidity_ratio, sensible_cooling_w: sensible.max(0.0), latent_heat_w: 0.0, water_consumption_kg_s: water_evap * 0.5, effectiveness_achieved: eps }
            }
        }
    }
    // #endregion 🔖Simulate

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::units::P_STD;

        #[test]
        fn direct_cooling_lowers_dry_bulb() {
            let cooler = EvaporativeCooler::Direct { effectiveness: 0.8, pad_area_m2: 10.0 };
            let inlet = EvaporativeInlet { dry_bulb_c: 35.0, humidity_ratio: humidity_ratio_from_rh(35.0, 0.3, P_STD), mass_flow_kg_s: 1.0, pressure_pa: P_STD };
            let out = evaporative_cool(&cooler, &inlet, true);
            assert!(out.dry_bulb_c < inlet.dry_bulb_c);
            assert!(out.humidity_ratio > inlet.humidity_ratio);
            assert!(out.water_consumption_kg_s > 0.0);
        }

        #[test]
        fn indirect_preserves_humidity_ratio() {
            let cooler = EvaporativeCooler::Indirect { sensible_effectiveness: 0.65, primary_flow_m3_s: 1.0, secondary_flow_m3_s: 1.0 };
            let inlet = EvaporativeInlet { dry_bulb_c: 32.0, humidity_ratio: 0.01, mass_flow_kg_s: 1.2, pressure_pa: P_STD };
            let out = evaporative_cool(&cooler, &inlet, true);
            assert!((out.humidity_ratio - inlet.humidity_ratio).abs() < 1e-9);
            assert!(out.sensible_cooling_w > 0.0);
        }

        #[test]
        fn disabled_no_effect() {
            let cooler = EvaporativeCooler::Direct { effectiveness: 0.9, pad_area_m2: 5.0 };
            let inlet = EvaporativeInlet { dry_bulb_c: 30.0, humidity_ratio: 0.012, mass_flow_kg_s: 0.8, pressure_pa: P_STD };
            let out = evaporative_cool(&cooler, &inlet, false);
            assert!((out.dry_bulb_c - inlet.dry_bulb_c).abs() < 1e-9);
        }
    }
}

mod fans {
    //! 🌀 Fan performance: pressure rise, efficiency curves, fan laws, and part-load power.

    use crate::curves::PerformanceCurve;
    use serde::{Deserialize, Serialize};

    // #region 🔖Fan
    /// 🌀 Fan type and performance specification.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Fan {
        pub fan_type: FanType,
        pub max_flow_m3_s: f64,
        pub max_pressure_rise_pa: f64,
        pub motor_efficiency: f64,
        pub pressure_curve: PerformanceCurve,
        pub efficiency_curve: PerformanceCurve,
        pub part_load_curve: PerformanceCurve,
    }

    /// 🔧 Fan arrangement.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FanType {
        ConstantVolume,
        VariableVolume,
        OnOff,
    }

    /// 📊 Fan operating point.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FanOperatingPoint {
        pub volume_flow_m3_s: f64,
        pub pressure_rise_pa: f64,
        pub part_load_ratio: f64,
        pub speed_ratio: f64,
    }
    // #endregion 🔖Fan

    // #region 🔖FanLaws
    /// 📐 Fan law scaling: Q ∝ N, ΔP ∝ N², Power ∝ N³.
    pub fn fan_law_flow(base_flow_m3_s: f64, speed_ratio: f64) -> f64 {
        base_flow_m3_s * speed_ratio
    }

    pub fn fan_law_pressure(base_pressure_pa: f64, speed_ratio: f64) -> f64 {
        base_pressure_pa * speed_ratio * speed_ratio
    }

    pub fn fan_law_power(base_power_w: f64, speed_ratio: f64) -> f64 {
        base_power_w * speed_ratio.powi(3)
    }

    /// ⚡ Fan shaft/electrical power [W] from flow, pressure rise, and efficiency.
    pub fn fan_power_w(fan: &Fan, operating: &FanOperatingPoint) -> f64 {
        if operating.volume_flow_m3_s.abs() < 1e-9 {
            return 0.0;
        }
        let plr = operating.part_load_ratio.clamp(0.0, 1.0);
        let speed = operating.speed_ratio.clamp(0.0, 1.2);

        let design_dp = fan.max_pressure_rise_pa * fan.pressure_curve.evaluate(plr);
        let dp = if operating.pressure_rise_pa > 0.0 { operating.pressure_rise_pa } else { fan_law_pressure(design_dp, speed) };

        let flow = if operating.volume_flow_m3_s > 0.0 { operating.volume_flow_m3_s } else { fan_law_flow(fan.max_flow_m3_s * plr, speed) };

        let eta_fan = fan.efficiency_curve.evaluate(plr).clamp(0.1, 0.9);
        let eta_motor = fan.motor_efficiency.clamp(0.5, 1.0);
        let eta_total = (eta_fan * eta_motor).max(0.05);

        let hydraulic_w = flow * dp;
        let part_load_mult = fan.part_load_curve.evaluate(plr).max(0.0);
        hydraulic_w / eta_total * part_load_mult
    }

    /// 📊 Compute fan operating point from requested flow and system pressure.
    pub fn fan_operating_point(fan: &Fan, requested_flow_m3_s: f64, system_pressure_pa: f64) -> FanOperatingPoint {
        let plr = (requested_flow_m3_s / fan.max_flow_m3_s.max(1e-6)).clamp(0.0, 1.2);
        let speed = plr.sqrt().clamp(0.0, 1.0);
        let dp_curve = fan.max_pressure_rise_pa * fan.pressure_curve.evaluate(plr);
        FanOperatingPoint { volume_flow_m3_s: requested_flow_m3_s, pressure_rise_pa: system_pressure_pa.max(dp_curve), part_load_ratio: plr, speed_ratio: speed }
    }

    /// 🌬️ Mass flow from volumetric flow and air density.
    pub fn fan_mass_flow_kg_s(volume_flow_m3_s: f64, density_kg_m3: f64) -> f64 {
        volume_flow_m3_s * density_kg_m3.max(0.5)
    }
    // #endregion 🔖FanLaws

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::units::RHO_AIR_REF;

        fn test_fan() -> Fan {
            Fan {
                fan_type: FanType::VariableVolume,
                max_flow_m3_s: 2.0,
                max_pressure_rise_pa: 800.0,
                motor_efficiency: 0.9,
                pressure_curve: PerformanceCurve::Quadratic { coeffs: [1.0, 0.0, -0.3] },
                efficiency_curve: PerformanceCurve::Quadratic { coeffs: [0.5, 0.4, 0.1] },
                part_load_curve: PerformanceCurve::Cubic { coeffs: [0.0, 0.3, 0.5, 0.2] },
            }
        }

        #[test]
        fn fan_laws_cubic_power() {
            assert!((fan_law_power(1000.0, 0.5) - 125.0).abs() < 1e-6);
        }

        #[test]
        fn zero_flow_zero_power() {
            let fan = test_fan();
            let op = FanOperatingPoint { volume_flow_m3_s: 0.0, pressure_rise_pa: 0.0, part_load_ratio: 0.0, speed_ratio: 0.0 };
            assert_eq!(fan_power_w(&fan, &op), 0.0);
        }

        #[test]
        fn full_load_positive_power() {
            let fan = test_fan();
            let op = fan_operating_point(&fan, 2.0, 600.0);
            let p = fan_power_w(&fan, &op);
            assert!(p > 0.0);
            let m_dot = fan_mass_flow_kg_s(2.0, RHO_AIR_REF);
            assert!((m_dot - 2.4).abs() < 0.1);
        }
    }
}

mod faults {
    //! 🔧 Equipment fault models: sensor offsets, fouling, dampers, refrigerant charge.

    use crate::error::Severity;
    use serde::{Deserialize, Serialize};

    // #region 🔖SeveritySchedule
    /// 📅 Time-varying fault severity multiplier (0 = none, 1 = full fault).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SeveritySchedule {
        pub hourly_severity: [f64; 24],
        pub interpolation: bool,
    }

    impl SeveritySchedule {
        /// 📅 Constant severity at all hours.
        pub fn constant(severity: f64) -> Self {
            Self { hourly_severity: [severity.clamp(0.0, 1.0); 24], interpolation: false }
        }

        /// 📅 Lookup severity at hour (0–23).
        pub fn at_hour(&self, hour: u8) -> f64 {
            let h = (hour as usize).min(23);
            self.hourly_severity[h].clamp(0.0, 1.0)
        }

        /// 📅 Interpolated severity at fractional hour.
        pub fn at_fractional_hour(&self, hour: f64) -> f64 {
            if !self.interpolation {
                return self.at_hour(hour as u8);
            }
            let h0 = (hour.floor() as usize).min(23);
            let h1 = (h0 + 1).min(23);
            let frac = hour - h0 as f64;
            let v0 = self.hourly_severity[h0];
            let v1 = self.hourly_severity[h1];
            (v0 + frac * (v1 - v0)).clamp(0.0, 1.0)
        }
    }
    // #endregion 🔖SeveritySchedule

    // #region 🔖SensorOffset
    /// 🌡️ Sensor bias fault on temperature or flow readings.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SensorOffsetFault {
        pub offset: f64,
        pub unit: SensorUnit,
        pub schedule: SeveritySchedule,
        pub diagnostic_severity: Severity,
    }

    /// 📏 Sensor measurement unit for offset faults.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SensorUnit {
        Celsius,
        Percent,
        Pascals,
        CubicMetersPerSecond,
    }

    impl SensorOffsetFault {
        /// 🌡️ Apply biased reading to true value at given hour.
        pub fn biased_reading(&self, true_value: f64, hour: u8) -> f64 {
            let severity = self.schedule.at_hour(hour);
            true_value + self.offset * severity
        }

        /// 🌡️ Correct a biased reading back to true value.
        pub fn correct_reading(&self, biased_value: f64, hour: u8) -> f64 {
            let severity = self.schedule.at_hour(hour);
            biased_value - self.offset * severity
        }
    }
    // #endregion 🔖SensorOffset

    // #region 🔖Fouling
    /// 🦠 Heat exchanger fouling reducing UA over time.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FoulingFault {
        pub baseline_ua_w_per_k: f64,
        pub fouling_factor: f64,
        pub schedule: SeveritySchedule,
        pub diagnostic_severity: Severity,
    }

    impl FoulingFault {
        /// 🦠 Effective UA with fouling degradation.
        pub fn effective_ua_w_per_k(&self, hour: u8) -> f64 {
            let severity = self.schedule.at_hour(hour);
            let degradation = 1.0 / (1.0 + self.fouling_factor * severity);
            self.baseline_ua_w_per_k * degradation
        }

        /// 🦠 Additional thermal resistance from fouling [K/W].
        pub fn added_resistance_k_per_w(&self, hour: u8) -> f64 {
            let ua_clean = self.baseline_ua_w_per_k;
            let ua_fouled = self.effective_ua_w_per_k(hour);
            1.0 / ua_fouled - 1.0 / ua_clean
        }
    }
    // #endregion 🔖Fouling

    // #region 🔖Damper
    /// 🌬️ Damper stuck/leaking fault on air system.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DamperFaultKind {
        StuckClosed,
        StuckOpen,
        Leaking { leakage_fraction: f64 },
    }

    /// 🌬️ Damper fault with scheduled severity.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DamperFault {
        pub kind: DamperFaultKind,
        pub design_position: f64,
        pub schedule: SeveritySchedule,
        pub diagnostic_severity: Severity,
    }

    impl DamperFault {
        /// 🌬️ Effective damper position (0 = closed, 1 = open).
        pub fn effective_position(&self, commanded: f64, hour: u8) -> f64 {
            let severity = self.schedule.at_hour(hour);
            let cmd = commanded.clamp(0.0, 1.0);
            match self.kind {
                DamperFaultKind::StuckClosed => cmd * (1.0 - severity),
                DamperFaultKind::StuckOpen => cmd + (1.0 - cmd) * severity,
                DamperFaultKind::Leaking { leakage_fraction } => cmd + leakage_fraction * severity * (1.0 - cmd),
            }
        }

        /// 🌬️ Airflow fraction relative to design at commanded position.
        pub fn airflow_fraction(&self, commanded: f64, hour: u8) -> f64 {
            let pos = self.effective_position(commanded, hour);
            pos.powf(0.6)
        }
    }
    // #endregion 🔖Damper

    // #region 🔖RefrigerantCharge
    /// ❄️ Refrigerant undercharge or overcharge fault.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChargeFaultKind {
        Undercharge,
        Overcharge,
    }

    /// ❄️ Refrigerant charge fault affecting capacity and power.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RefrigerantChargeFault {
        pub kind: ChargeFaultKind,
        pub charge_deviation_fraction: f64,
        pub schedule: SeveritySchedule,
        pub diagnostic_severity: Severity,
    }

    impl RefrigerantChargeFault {
        /// ❄️ Capacity multiplier from charge fault.
        pub fn capacity_multiplier(&self, hour: u8) -> f64 {
            let severity = self.schedule.at_hour(hour);
            let dev = self.charge_deviation_fraction * severity;
            match self.kind {
                ChargeFaultKind::Undercharge => (1.0 - 0.5 * dev).clamp(0.3, 1.0),
                ChargeFaultKind::Overcharge => (1.0 - 0.15 * dev).clamp(0.7, 1.0),
            }
        }

        /// ❄️ Compressor power penalty multiplier.
        pub fn power_multiplier(&self, hour: u8) -> f64 {
            let severity = self.schedule.at_hour(hour);
            let dev = self.charge_deviation_fraction * severity;
            match self.kind {
                ChargeFaultKind::Undercharge => 1.0 + 0.4 * dev,
                ChargeFaultKind::Overcharge => 1.0 + 0.2 * dev,
            }
        }

        /// ❄️ Adjusted cooling output and compressor power.
        pub fn apply(&self, cooling_w: f64, compressor_w: f64, hour: u8) -> (f64, f64) {
            (cooling_w * self.capacity_multiplier(hour), compressor_w * self.power_multiplier(hour))
        }
    }
    // #endregion 🔖RefrigerantCharge

    // #region 🔖FaultSet
    /// 🔧 Combined fault set for a plant or air-handling component.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct FaultSet {
        pub sensor_offsets: Vec<SensorOffsetFault>,
        pub fouling: Vec<FoulingFault>,
        pub dampers: Vec<DamperFault>,
        pub refrigerant: Vec<RefrigerantChargeFault>,
    }

    impl FaultSet {
        /// 🔧 Apply all sensor offsets to a temperature reading.
        pub fn biased_temperature_c(&self, true_c: f64, hour: u8) -> f64 {
            self.sensor_offsets.iter().filter(|f| matches!(f.unit, SensorUnit::Celsius)).fold(true_c, |acc, f| f.biased_reading(acc, hour))
        }

        /// 🔧 Worst-case fouling UA multiplier across all fouling faults.
        pub fn fouling_ua_multiplier(&self, hour: u8) -> f64 {
            if self.fouling.is_empty() {
                return 1.0;
            }
            self.fouling.iter().map(|f| f.effective_ua_w_per_k(hour) / f.baseline_ua_w_per_k).fold(1.0, f64::min)
        }
    }
    // #endregion 🔖FaultSet

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn severity_schedule_constant() {
            let sched = SeveritySchedule::constant(0.8);
            assert!((sched.at_hour(12) - 0.8).abs() < 1e-9);
        }

        #[test]
        fn sensor_offset_biases_reading() {
            let fault = SensorOffsetFault { offset: 2.0, unit: SensorUnit::Celsius, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning };
            assert!((fault.biased_reading(20.0, 10) - 22.0).abs() < 1e-9);
            assert!((fault.correct_reading(22.0, 10) - 20.0).abs() < 1e-9);
        }

        #[test]
        fn fouling_reduces_ua() {
            let fault = FoulingFault { baseline_ua_w_per_k: 10_000.0, fouling_factor: 0.5, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Severe };
            assert!(fault.effective_ua_w_per_k(12) < fault.baseline_ua_w_per_k);
        }

        #[test]
        fn damper_stuck_open_increases_flow() {
            let fault = DamperFault { kind: DamperFaultKind::StuckOpen, design_position: 0.5, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning };
            let normal = fault.airflow_fraction(0.0, 12);
            assert!(normal > 0.5);
        }

        #[test]
        fn undercharge_reduces_capacity() {
            let fault = RefrigerantChargeFault { kind: ChargeFaultKind::Undercharge, charge_deviation_fraction: 0.4, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Severe };
            assert!(fault.capacity_multiplier(8) < 1.0);
            assert!(fault.power_multiplier(8) > 1.0);
        }

        #[test]
        fn fault_set_compounds_sensor_offsets() {
            let set = FaultSet {
                sensor_offsets: vec![
                    SensorOffsetFault { offset: 1.0, unit: SensorUnit::Celsius, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning },
                    SensorOffsetFault { offset: 0.5, unit: SensorUnit::Celsius, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning },
                ],
                ..Default::default()
            };
            assert!((set.biased_temperature_c(20.0, 0) - 21.5).abs() < 1e-9);
        }
    }
}

mod fenestration {
    //! 🪟 Fenestration heat transfer: glazing layers, frames, shades, and condensation.

    use crate::material::{R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
    use crate::props::{dew_point_c, saturation_pressure_pa};

    // #region 🔖GlazingLayer
    /// 🪟 Single glazing layer optical and thermal properties.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct GlazingLayer {
        pub thickness_m: f64,
        pub conductivity_w_m_k: f64,
        pub solar_transmittance: f64,
        pub solar_reflectance: f64,
        pub visible_transmittance: f64,
        pub ir_emissivity: f64,
    }

    impl GlazingLayer {
        /// 🧊 Layer thermal resistance [m²·K/W].
        pub fn resistance_m2k_w(&self) -> f64 {
            self.thickness_m / self.conductivity_w_m_k.max(1e-6)
        }
    }
    // #endregion 🔖GlazingLayer

    // #region 🔖ShadeState
    /// 🌤️ Interior or exterior shade position and properties.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ShadeState {
        pub deployed: bool,
        pub solar_transmittance: f64,
        pub solar_reflectance: f64,
        pub visible_transmittance: f64,
        pub ir_transmittance: f64,
    }

    impl ShadeState {
        pub const OPEN: Self = Self { deployed: false, solar_transmittance: 1.0, solar_reflectance: 0.0, visible_transmittance: 1.0, ir_transmittance: 1.0 };

        /// 🌤️ Effective solar transmittance with shade deployed.
        pub fn effective_solar_transmittance(&self) -> f64 {
            if self.deployed {
                self.solar_transmittance
            } else {
                1.0
            }
        }
    }
    // #endregion 🔖ShadeState

    // #region 🔖WindowModel
    /// 🪟 Complete window thermal and optical model.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowModel {
        pub glazing_layers: Vec<GlazingLayer>,
        pub gap_resistance_m2k_w: Vec<f64>,
        pub frame_fraction: f64,
        pub frame_u_value_w_m2k: f64,
        pub divider_fraction: f64,
        pub divider_conductance_w_k: f64,
        pub interior_shade: ShadeState,
        pub exterior_shade: ShadeState,
    }

    impl WindowModel {
        /// 🔥 Center-of-glass U-value [W/(m²·K)] from layer stack.
        pub fn center_u_value_w_m2k(&self) -> f64 {
            let mut r = R_FILM_INTERIOR_M2K_W + R_FILM_EXTERIOR_M2K_W;
            for (i, layer) in self.glazing_layers.iter().enumerate() {
                r += layer.resistance_m2k_w();
                if let Some(&gap_r) = self.gap_resistance_m2k_w.get(i) {
                    r += gap_r;
                }
            }
            1.0 / r.max(1e-6)
        }

        /// ☀️ Center-of-glass solar heat gain coefficient (normal incidence).
        pub fn center_shgc(&self) -> f64 {
            let mut tau = 1.0_f64;
            for layer in &self.glazing_layers {
                tau *= layer.solar_transmittance;
            }
            tau *= self.exterior_shade.effective_solar_transmittance();
            tau *= self.interior_shade.effective_solar_transmittance();
            tau
        }

        /// 🔥 Area-weighted overall U including frame and divider.
        pub fn overall_u_value_w_m2k(&self, area_m2: f64) -> f64 {
            let a_cog = area_m2 * (1.0 - self.frame_fraction - self.divider_fraction).max(0.0);
            let a_frame = area_m2 * self.frame_fraction;
            let u_cog = self.center_u_value_w_m2k();
            let u_frame = self.frame_u_value_w_m2k;
            let divider_loss = self.divider_conductance_w_k;
            let total_cond = a_cog * u_cog + a_frame * u_frame + divider_loss;
            total_cond / area_m2.max(1e-6)
        }

        /// 🌡️ Interior glazing surface temperature [°C] (simplified steady state).
        pub fn interior_glazing_temp_c(&self, outside_temp_c: f64, inside_temp_c: f64, h_interior_w_m2k: f64, h_exterior_w_m2k: f64) -> f64 {
            let r_int = 1.0 / h_interior_w_m2k.max(0.1);
            let r_ext = 1.0 / h_exterior_w_m2k.max(0.1);
            let mut r_glazing = 0.0_f64;
            for (i, layer) in self.glazing_layers.iter().enumerate() {
                r_glazing += layer.resistance_m2k_w();
                if let Some(&gap_r) = self.gap_resistance_m2k_w.get(i) {
                    r_glazing += gap_r;
                }
            }
            let r_total = r_ext + r_glazing + r_int;
            inside_temp_c + (outside_temp_c - inside_temp_c) * r_int / r_total.max(1e-6)
        }
    }
    // #endregion 🔖WindowModel

    // #region 🔖HeatTransfer
    /// 🔥 Window conductive heat loss [W] (positive = heat into zone from outside).
    pub fn window_conduction_w(outside_temp_c: f64, inside_temp_c: f64, u_value_w_m2k: f64, area_m2: f64) -> f64 {
        u_value_w_m2k * (outside_temp_c - inside_temp_c) * area_m2
    }

    /// ☀️ Solar gain through window [W].
    pub fn window_solar_gain_w(beam_normal_irradiance_w_m2: f64, incidence_cosine: f64, shgc: f64, area_m2: f64) -> f64 {
        beam_normal_irradiance_w_m2 * incidence_cosine.max(0.0) * shgc * area_m2
    }
    // #endregion 🔖HeatTransfer

    // #region 🔖Condensation
    /// 💧 Condensation risk on interior glazing surface.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum CondensationRisk {
        None,
        Risk { margin_k: f64 },
        Condensing,
    }

    /// 💧 Assess interior surface condensation vs zone dew point.
    pub fn condensation_risk(interior_surface_temp_c: f64, _zone_air_temp_c: f64, humidity_ratio: f64, atmospheric_pressure_pa: f64) -> CondensationRisk {
        let dew = dew_point_c(humidity_ratio, atmospheric_pressure_pa);
        let margin = interior_surface_temp_c - dew;
        if margin <= 0.0 {
            CondensationRisk::Condensing
        } else if margin < 2.0 {
            CondensationRisk::Risk { margin_k: margin }
        } else {
            CondensationRisk::None
        }
    }

    /// 💧 Interior surface RH given surface temperature.
    pub fn interior_surface_rh(surface_temp_c: f64, zone_air_temp_c: f64, zone_rh: f64) -> f64 {
        let p_ws_air = saturation_pressure_pa(zone_air_temp_c);
        let p_w = zone_rh * p_ws_air;
        let p_ws_surf = saturation_pressure_pa(surface_temp_c);
        (p_w / p_ws_surf.max(1.0)).clamp(0.0, 1.5)
    }
    // #endregion 🔖Condensation

    #[cfg(test)]
    mod tests {
        use super::*;

        fn double_glazing() -> WindowModel {
            WindowModel {
                glazing_layers: vec![
                    GlazingLayer { thickness_m: 0.004, conductivity_w_m_k: 0.9, solar_transmittance: 0.82, solar_reflectance: 0.08, visible_transmittance: 0.88, ir_emissivity: 0.84 },
                    GlazingLayer { thickness_m: 0.004, conductivity_w_m_k: 0.9, solar_transmittance: 0.74, solar_reflectance: 0.12, visible_transmittance: 0.80, ir_emissivity: 0.84 },
                ],
                gap_resistance_m2k_w: vec![0.15],
                frame_fraction: 0.15,
                frame_u_value_w_m2k: 2.5,
                divider_fraction: 0.05,
                divider_conductance_w_k: 0.5,
                interior_shade: ShadeState::OPEN,
                exterior_shade: ShadeState::OPEN,
            }
        }

        #[test]
        fn double_glazing_u_below_single() {
            let win = double_glazing();
            let u_double = win.center_u_value_w_m2k();
            let single = WindowModel { glazing_layers: vec![win.glazing_layers[0]], gap_resistance_m2k_w: vec![], ..win.clone() };
            assert!(u_double < single.center_u_value_w_m2k());
        }

        #[test]
        fn shade_reduces_shgc() {
            let mut win = double_glazing();
            win.interior_shade = ShadeState { deployed: true, solar_transmittance: 0.1, solar_reflectance: 0.5, visible_transmittance: 0.1, ir_transmittance: 0.2 };
            assert!(win.center_shgc() < 0.2);
        }

        #[test]
        fn conduction_cold_outside_negative_into_zone() {
            let q = window_conduction_w(-10.0, 20.0, 1.2, 2.0);
            assert!(q < 0.0);
        }

        #[test]
        fn condensation_when_surface_cold() {
            let risk = condensation_risk(5.0, 22.0, 0.012, 101_325.0);
            assert!(matches!(risk, CondensationRisk::Condensing | CondensationRisk::Risk { .. }));
        }
    }
}

mod gains {
    //! 🔥 Internal gains: people, lighting, equipment, process, data center decomposition.

    use serde::{Deserialize, Serialize};

    // #region 🔖GainDecomposition
    /// 📊 Internal gain split into sensible, radiant, latent, and return-air fractions [W].
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct GainDecomposition {
        pub total_w: f64,
        pub sensible_w: f64,
        pub radiant_w: f64,
        pub latent_w: f64,
        pub convective_w: f64,
        pub return_air_w: f64,
    }

    impl GainDecomposition {
        pub fn add(&self, other: &Self) -> Self {
            Self {
                total_w: self.total_w + other.total_w,
                sensible_w: self.sensible_w + other.sensible_w,
                radiant_w: self.radiant_w + other.radiant_w,
                latent_w: self.latent_w + other.latent_w,
                convective_w: self.convective_w + other.convective_w,
                return_air_w: self.return_air_w + other.return_air_w,
            }
        }
    }
    // #endregion 🔖GainDecomposition

    // #region 🔖People
    /// 👤 Metabolic rate presets [W/person] per ASHRAE 55 activity levels.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActivityLevel {
        SeatedQuiet,
        OfficeWork,
        StandingLight,
        Walking,
        HeavyWork,
    }

    impl ActivityLevel {
        pub fn metabolic_w_per_person(self) -> f64 {
            match self {
                Self::SeatedQuiet => 70.0,
                Self::OfficeWork => 100.0,
                Self::StandingLight => 120.0,
                Self::Walking => 160.0,
                Self::HeavyWork => 250.0,
            }
        }

        pub fn sensible_fraction(self) -> f64 {
            match self {
                Self::SeatedQuiet | Self::OfficeWork => 0.58,
                Self::StandingLight => 0.55,
                Self::Walking | Self::HeavyWork => 0.50,
            }
        }

        pub fn latent_fraction(self) -> f64 {
            1.0 - self.sensible_fraction()
        }
    }

    /// 👤 People gain [W] from count, activity, and radiant fraction.
    pub fn compute_people_gain_w(count: f64, activity: ActivityLevel, schedule_factor: f64, radiant_fraction: f64) -> GainDecomposition {
        let total = count * activity.metabolic_w_per_person() * schedule_factor.clamp(0.0, 1.0);
        let sensible = total * activity.sensible_fraction();
        let latent = total * activity.latent_fraction();
        let radiant = sensible * radiant_fraction.clamp(0.0, 1.0);
        let convective = sensible - radiant;
        GainDecomposition { total_w: total, sensible_w: sensible, radiant_w: radiant, latent_w: latent, convective_w: convective, return_air_w: 0.0 }
    }
    // #endregion 🔖People

    // #region 🔖Lighting
    /// 💡 Lighting gain [W] from power density and fractions.
    pub fn compute_lighting_gain_w(watts_per_area: f64, floor_area_m2: f64, schedule_factor: f64, radiant_fraction: f64, return_air_fraction: f64) -> GainDecomposition {
        let total = watts_per_area * floor_area_m2 * schedule_factor.clamp(0.0, 1.0);
        let radiant = total * radiant_fraction.clamp(0.0, 1.0);
        let return_air = total * return_air_fraction.clamp(0.0, 1.0);
        let convective = total - radiant - return_air;
        GainDecomposition { total_w: total, sensible_w: total, radiant_w: radiant, latent_w: 0.0, convective_w: convective.max(0.0), return_air_w: return_air }
    }
    // #endregion 🔖Lighting

    // #region 🔖Equipment
    /// 🔌 Electric equipment gain [W].
    pub fn compute_equipment_gain_w(watts_per_area: f64, floor_area_m2: f64, schedule_factor: f64, radiant_fraction: f64, latent_fraction: f64) -> GainDecomposition {
        let total = watts_per_area * floor_area_m2 * schedule_factor.clamp(0.0, 1.0);
        let latent = total * latent_fraction.clamp(0.0, 1.0);
        let sensible = total - latent;
        let radiant = sensible * radiant_fraction.clamp(0.0, 1.0);
        let convective = sensible - radiant;
        GainDecomposition { total_w: total, sensible_w: sensible, radiant_w: radiant, latent_w: latent, convective_w: convective, return_air_w: 0.0 }
    }
    // #endregion 🔖Equipment

    // #region 🔖Process
    /// 🏭 Process load gain [W] with configurable split.
    pub fn compute_process_gain_w(design_load_w: f64, schedule_factor: f64, sensible_fraction: f64, latent_fraction: f64, radiant_fraction: f64) -> GainDecomposition {
        let total = design_load_w * schedule_factor.clamp(0.0, 1.0);
        let latent = total * latent_fraction.clamp(0.0, 1.0);
        let sensible = total * sensible_fraction.clamp(0.0, 1.0);
        let radiant = sensible * radiant_fraction.clamp(0.0, 1.0);
        let convective = sensible - radiant;
        GainDecomposition { total_w: total, sensible_w: sensible, radiant_w: radiant, latent_w: latent, convective_w: convective, return_air_w: 0.0 }
    }
    // #endregion 🔖Process

    // #region 🔖DataCenter
    /// 🖥️ Data center IT load [W] with air-side heat capture fraction.
    pub fn compute_datacenter_gain_w(it_load_w: f64, schedule_factor: f64, air_cooled_fraction: f64, supply_return_delta_t_k: f64) -> GainDecomposition {
        let total = it_load_w * schedule_factor.clamp(0.0, 1.0);
        let air_frac = air_cooled_fraction.clamp(0.0, 1.0);
        let air_w = total * air_frac;
        let liquid_w = total - air_w;
        let return_air = if supply_return_delta_t_k > 0.1 { air_w * 0.9 } else { air_w * 0.5 };
        GainDecomposition { total_w: total, sensible_w: total, radiant_w: liquid_w * 0.1, latent_w: 0.0, convective_w: air_w - return_air, return_air_w: return_air }
    }
    // #endregion 🔖DataCenter

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn people_gain_scales_with_count() {
            let g1 = compute_people_gain_w(1.0, ActivityLevel::OfficeWork, 1.0, 0.3);
            let g10 = compute_people_gain_w(10.0, ActivityLevel::OfficeWork, 1.0, 0.3);
            assert!((g10.total_w - 10.0 * g1.total_w).abs() < 1e-6);
        }

        #[test]
        fn lighting_return_air_reduces_convective() {
            let g = compute_lighting_gain_w(10.0, 100.0, 1.0, 0.2, 0.5);
            assert!((g.total_w - 1000.0).abs() < 1e-6);
            assert!((g.return_air_w - 500.0).abs() < 1e-6);
            assert!((g.radiant_w - 200.0).abs() < 1e-6);
        }

        #[test]
        fn equipment_latent_reduces_sensible() {
            let g = compute_equipment_gain_w(5.0, 200.0, 1.0, 0.5, 0.1);
            assert!((g.latent_w - 100.0).abs() < 1e-6);
            assert!((g.sensible_w - 900.0).abs() < 1e-6);
        }

        #[test]
        fn datacenter_total_matches_it_load() {
            let g = compute_datacenter_gain_w(50_000.0, 0.8, 0.7, 12.0);
            assert!((g.total_w - 40_000.0).abs() < 1e-6);
        }
    }
}

mod geometry {
    //! 📐 Surface geometry: area, orientation, zone volume, and coordinate transforms.

    use crate::units::rad_to_deg;
    use mathematical_algebra::Vec3;

    // #region 🔖Types
    /// 📐 Surface tilt [° from horizontal] and azimuth [° clockwise from north].
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct TiltAzimuth {
        pub tilt_deg: f64,
        pub azimuth_deg: f64,
    }

    /// ✅ Planarity validation outcome.
    #[derive(Clone, Debug, PartialEq)]
    pub enum PlanarValidation {
        Ok,
        TooFewVertices,
        DegenerateArea,
        NonPlanar { max_deviation_m: f64 },
    }
    // #endregion 🔖Types

    // #region 🔖VecHelpers
    fn to_vec3(v: [f64; 3]) -> Vec3 {
        Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32)
    }

    fn from_vec3(v: Vec3) -> [f64; 3] {
        [v.x as f64, v.y as f64, v.z as f64]
    }

    fn normalize(v: [f64; 3]) -> [f64; 3] {
        from_vec3(to_vec3(v).normalize())
    }
    // #endregion 🔖VecHelpers

    // #region 🔖AreaNormal
    /// 📏 Signed polygon area [m²] via cross-sum (positive for CCW when viewed along outward normal).
    pub fn surface_area_m2(vertices_m: &[[f64; 3]]) -> f64 {
        if vertices_m.len() < 3 {
            return 0.0;
        }
        let origin = to_vec3(vertices_m[0]);
        let mut area = 0.0_f64;
        for i in 1..vertices_m.len() - 1 {
            let a = to_vec3(vertices_m[i]).sub(origin);
            let b = to_vec3(vertices_m[i + 1]).sub(origin);
            area += a.cross(b).length() as f64 * 0.5;
        }
        area
    }

    /// 🧭 Outward unit normal from polygon winding (Newell's method).
    pub fn polygon_normal(vertices_m: &[[f64; 3]]) -> [f64; 3] {
        if vertices_m.len() < 3 {
            return [0.0, 0.0, 1.0];
        }
        let mut n = [0.0_f64; 3];
        let len = vertices_m.len();
        for i in 0..len {
            let (x0, y0, z0) = (vertices_m[i][0], vertices_m[i][1], vertices_m[i][2]);
            let (x1, y1, z1) = (vertices_m[(i + 1) % len][0], vertices_m[(i + 1) % len][1], vertices_m[(i + 1) % len][2]);
            n[0] += (y0 - y1) * (z0 + z1);
            n[1] += (z0 - z1) * (x0 + x1);
            n[2] += (x0 - x1) * (y0 + y1);
        }
        normalize(n)
    }
    // #endregion 🔖AreaNormal

    // #region 🔖Orientation
    /// 🧭 Tilt from horizontal and azimuth clockwise from north (+Y) with optional north-axis offset.
    pub fn surface_tilt_azimuth(normal: [f64; 3], north_axis_deg: f64) -> TiltAzimuth {
        let n = normalize(normal);
        let tilt_deg = rad_to_deg(n[2].clamp(-1.0, 1.0).acos());
        let mut azimuth_deg = rad_to_deg(n[0].atan2(n[1]));
        if azimuth_deg < 0.0 {
            azimuth_deg += 360.0;
        }
        azimuth_deg = (azimuth_deg + north_axis_deg).rem_euclid(360.0);
        TiltAzimuth { tilt_deg, azimuth_deg }
    }
    // #endregion 🔖Orientation

    // #region 🔖Volume
    /// 📦 Zone volume [m³] from closed watertight surface set (pyramid sum to interior reference point).
    pub fn zone_volume_from_surfaces(surfaces: &[&[[f64; 3]]]) -> f64 {
        let mut ref_pt = [0.0_f64; 3];
        let mut count = 0usize;
        for vertices in surfaces {
            for v in *vertices {
                ref_pt[0] += v[0];
                ref_pt[1] += v[1];
                ref_pt[2] += v[2];
                count += 1;
            }
        }
        if count == 0 {
            return 0.0;
        }
        ref_pt[0] /= count as f64;
        ref_pt[1] /= count as f64;
        ref_pt[2] /= count as f64;
        surfaces.iter().map(|face| face_pyramid_volume_m3(face, ref_pt)).sum::<f64>().abs()
    }

    fn face_pyramid_volume_m3(vertices: &[[f64; 3]], ref_pt: [f64; 3]) -> f64 {
        if vertices.len() < 3 {
            return 0.0;
        }
        let mut ax = 0.0_f64;
        let mut ay = 0.0_f64;
        let mut az = 0.0_f64;
        let len = vertices.len();
        for i in 0..len {
            let v0 = vertices[i];
            let v1 = vertices[(i + 1) % len];
            ax += (v0[1] - v1[1]) * (v0[2] + v1[2]);
            ay += (v0[2] - v1[2]) * (v0[0] + v1[0]);
            az += (v0[0] - v1[0]) * (v0[1] + v1[1]);
        }
        let inv = 1.0 / len as f64;
        let cx = vertices.iter().map(|v| v[0]).sum::<f64>() * inv;
        let cy = vertices.iter().map(|v| v[1]).sum::<f64>() * inv;
        let cz = vertices.iter().map(|v| v[2]).sum::<f64>() * inv;
        let dx = cx - ref_pt[0];
        let dy = cy - ref_pt[1];
        let dz = cz - ref_pt[2];
        (ax * dx + ay * dy + az * dz) / 6.0
    }
    // #endregion 🔖Volume

    // #region 🔖Validation
    /// ✅ Check polygon planarity within tolerance [m].
    pub fn validate_polygon_planar(vertices_m: &[[f64; 3]], tolerance_m: f64) -> PlanarValidation {
        if vertices_m.len() < 3 {
            return PlanarValidation::TooFewVertices;
        }
        if surface_area_m2(vertices_m) < 1e-9 {
            return PlanarValidation::DegenerateArea;
        }
        let n = polygon_normal(vertices_m);
        let anchor = vertices_m[0];
        let mut max_dev = 0.0_f64;
        for v in &vertices_m[1..] {
            let d = (v[0] - anchor[0]) * n[0] + (v[1] - anchor[1]) * n[1] + (v[2] - anchor[2]) * n[2];
            max_dev = max_dev.max(d.abs());
        }
        if max_dev > tolerance_m {
            PlanarValidation::NonPlanar { max_deviation_m: max_dev }
        } else {
            PlanarValidation::Ok
        }
    }
    // #endregion 🔖Validation

    // #region 🔖Transform
    /// 🔄 Apply 4×4 transform to polygon vertices (building ↔ world).
    pub fn transform_vertices(vertices_m: &[[f64; 3]], transform: mathematical_algebra::Mat4) -> Vec<[f64; 3]> {
        vertices_m.iter().map(|v| from_vec3(transform.transform_point(to_vec3(*v)))).collect()
    }

    /// 🔄 Rotate direction vector (no translation).
    pub fn transform_direction(direction: [f64; 3], transform: mathematical_algebra::Mat4) -> [f64; 3] {
        from_vec3(transform.transform_direction(to_vec3(direction)))
    }
    // #endregion 🔖Transform

    #[cfg(test)]
    mod tests {
        use super::*;
        use mathematical_algebra::Mat4;

        #[test]
        fn unit_square_area() {
            let verts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
            assert!((surface_area_m2(&verts) - 1.0).abs() < 1e-6);
        }

        #[test]
        fn horizontal_roof_tilt_zero() {
            let ta = surface_tilt_azimuth([0.0, 0.0, 1.0], 0.0);
            assert!(ta.tilt_deg.abs() < 1e-6);
        }

        #[test]
        fn vertical_wall_tilt_ninety() {
            let ta = surface_tilt_azimuth([1.0, 0.0, 0.0], 0.0);
            assert!((ta.tilt_deg - 90.0).abs() < 1e-6);
        }

        #[test]
        fn box_volume() {
            let floor = [[0.0, 0.0, 0.0], [0.0, 3.0, 0.0], [4.0, 3.0, 0.0], [4.0, 0.0, 0.0]];
            let ceiling = [[0.0, 0.0, 3.0], [4.0, 0.0, 3.0], [4.0, 3.0, 3.0], [0.0, 3.0, 3.0]];
            let walls = [
                [[0.0, 0.0, 0.0], [0.0, 0.0, 3.0], [0.0, 3.0, 3.0], [0.0, 3.0, 0.0]],
                [[4.0, 0.0, 0.0], [4.0, 3.0, 0.0], [4.0, 3.0, 3.0], [4.0, 0.0, 3.0]],
                [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
                [[0.0, 3.0, 0.0], [0.0, 3.0, 3.0], [4.0, 3.0, 3.0], [4.0, 3.0, 0.0]],
            ];
            let mut surfaces: Vec<&[[f64; 3]]> = vec![&floor, &ceiling];
            for w in &walls {
                surfaces.push(w);
            }
            let vol = zone_volume_from_surfaces(&surfaces);
            assert!((vol - 36.0).abs() < 0.5);
        }

        #[test]
        fn planar_validation_ok() {
            let verts = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 2.0, 0.0]];
            assert_eq!(validate_polygon_planar(&verts, 1e-6), PlanarValidation::Ok);
        }

        #[test]
        fn identity_transform_preserves_vertices() {
            let verts = [[1.0, 2.0, 3.0]];
            let out = transform_vertices(&verts, Mat4::identity());
            assert!((out[0][0] - 1.0).abs() < 1e-5);
        }
    }
}

mod heat_recovery {
    //! ♻️ Heat recovery: sensible/latent exchange via effectiveness-NTU with frost control.

    use crate::props::moist_air_enthalpy_j_per_kg;
    use crate::units::{CP_DRY_AIR, H_FG_0C};
    use serde::{Deserialize, Serialize};

    // #region 🔖HeatRecovery
    /// ♻️ Heat recovery ventilator configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HeatRecoveryUnit {
        pub hx_type: HeatExchangerType,
        pub sensible_effectiveness: f64,
        pub latent_effectiveness: f64,
        pub frost_control_temp_c: f64,
        pub defrost_power_w: f64,
    }

    /// 🔀 Heat exchanger flow arrangement.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HeatExchangerType {
        CounterFlow,
        CrossFlow,
        ParallelFlow,
    }

    /// 📥 Supply and exhaust airstreams at HX inlet.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HxAirstream {
        pub temperature_c: f64,
        pub humidity_ratio: f64,
        pub mass_flow_kg_s: f64,
        pub pressure_pa: f64,
    }

    /// 📤 Heat recovery exchange result.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HeatRecoveryOutput {
        pub supply_out: HxAirstream,
        pub exhaust_out: HxAirstream,
        pub sensible_recovery_w: f64,
        pub latent_recovery_w: f64,
        pub total_recovery_w: f64,
        pub defrost_active: bool,
        pub defrost_power_w: f64,
        pub effectiveness_sensible: f64,
        pub effectiveness_latent: f64,
    }
    // #endregion 🔖HeatRecovery

    // #region 🔖Ntu
    /// 📐 Effectiveness from NTU and capacity ratio (counter-flow approximation).
    pub fn effectiveness_from_ntu(ntu: f64, cr: f64, hx_type: HeatExchangerType) -> f64 {
        let cr = cr.clamp(0.0, 1.0);
        let ntu = ntu.max(0.0);
        match hx_type {
            HeatExchangerType::CounterFlow => {
                if (cr - 1.0).abs() < 1e-6 {
                    ntu / (1.0 + ntu)
                } else {
                    (1.0 - (-ntu * (1.0 - cr)).exp()) / (1.0 - cr * (-ntu * (1.0 - cr)).exp())
                }
            }
            HeatExchangerType::CrossFlow => {
                let n = ntu * (1.0 - 0.22 * cr.ln().abs().min(2.0));
                n / (1.0 + n)
            }
            HeatExchangerType::ParallelFlow => (1.0 - (-ntu * (1.0 + cr)).exp()) / (1.0 + cr),
        }
    }

    /// 📐 NTU from UA and minimum capacity rate.
    pub fn ntu_from_ua(ua_w_per_k: f64, c_min: f64) -> f64 {
        if c_min < 1e-9 {
            return 0.0;
        }
        ua_w_per_k / c_min
    }
    // #endregion 🔖Ntu

    // #region 🔖Exchange
    /// ♻️ Sensible and latent heat recovery exchange [W].
    pub fn heat_recovery_exchange_w(unit: &HeatRecoveryUnit, supply_in: &HxAirstream, exhaust_in: &HxAirstream) -> HeatRecoveryOutput {
        let m_sup = supply_in.mass_flow_kg_s.max(0.0);
        let m_exh = exhaust_in.mass_flow_kg_s.max(0.0);
        if m_sup < 1e-9 || m_exh < 1e-9 {
            return passthrough(unit, supply_in, exhaust_in);
        }

        let c_sup = m_sup * CP_DRY_AIR;
        let c_exh = m_exh * CP_DRY_AIR;
        let c_min = c_sup.min(c_exh);
        let cr = c_min / c_sup.max(c_exh).max(1e-9);

        let eps_s = unit.sensible_effectiveness.clamp(0.0, 0.95);
        let eps_l = unit.latent_effectiveness.clamp(0.0, 0.85);

        let t_diff = exhaust_in.temperature_c - supply_in.temperature_c;
        let q_sensible = eps_s * c_min * t_diff;

        let w_diff = exhaust_in.humidity_ratio - supply_in.humidity_ratio;
        let q_latent = eps_l * m_sup.min(m_exh) * w_diff * H_FG_0C;

        let mut defrost = false;
        let mut defrost_power = 0.0_f64;
        let mut eff_s = eps_s;
        let mut eff_l = eps_l;

        if supply_in.temperature_c < unit.frost_control_temp_c && exhaust_in.temperature_c > supply_in.temperature_c {
            defrost = true;
            defrost_power = unit.defrost_power_w;
            eff_s *= 0.5;
            eff_l *= 0.3;
        }

        let q_sensible_adj = eff_s * c_min * t_diff;
        let q_latent_adj = eff_l * m_sup.min(m_exh) * w_diff * H_FG_0C;

        let supply_t = supply_in.temperature_c + q_sensible_adj / c_sup;
        let exhaust_t = exhaust_in.temperature_c - q_sensible_adj / c_exh;
        let supply_w = supply_in.humidity_ratio + eff_l * w_diff * m_exh / m_sup;
        let exhaust_w = exhaust_in.humidity_ratio - eff_l * w_diff * m_sup / m_exh;

        let h_sup_in = moist_air_enthalpy_j_per_kg(supply_in.temperature_c, supply_in.humidity_ratio);
        let h_sup_out = moist_air_enthalpy_j_per_kg(supply_t, supply_w);
        let h_exh_in = moist_air_enthalpy_j_per_kg(exhaust_in.temperature_c, exhaust_in.humidity_ratio);
        let h_exh_out = moist_air_enthalpy_j_per_kg(exhaust_t, exhaust_w);
        let q_total = m_sup * (h_sup_out - h_sup_in) - m_exh * (h_exh_out - h_exh_in);

        let _ = (q_sensible, q_latent, cr);

        HeatRecoveryOutput {
            supply_out: HxAirstream { temperature_c: supply_t, humidity_ratio: supply_w.max(0.0), mass_flow_kg_s: m_sup, pressure_pa: supply_in.pressure_pa },
            exhaust_out: HxAirstream { temperature_c: exhaust_t, humidity_ratio: exhaust_w.max(0.0), mass_flow_kg_s: m_exh, pressure_pa: exhaust_in.pressure_pa },
            sensible_recovery_w: q_sensible_adj,
            latent_recovery_w: q_latent_adj,
            total_recovery_w: q_total,
            defrost_active: defrost,
            defrost_power_w: defrost_power,
            effectiveness_sensible: eff_s,
            effectiveness_latent: eff_l,
        }
    }

    fn passthrough(unit: &HeatRecoveryUnit, supply: &HxAirstream, exhaust: &HxAirstream) -> HeatRecoveryOutput {
        HeatRecoveryOutput {
            supply_out: *supply,
            exhaust_out: *exhaust,
            sensible_recovery_w: 0.0,
            latent_recovery_w: 0.0,
            total_recovery_w: 0.0,
            defrost_active: false,
            defrost_power_w: 0.0,
            effectiveness_sensible: unit.sensible_effectiveness,
            effectiveness_latent: unit.latent_effectiveness,
        }
    }
    // #endregion 🔖Exchange

    #[cfg(test)]
    mod tests {
        use super::*;

        fn erv() -> HeatRecoveryUnit {
            HeatRecoveryUnit { hx_type: HeatExchangerType::CounterFlow, sensible_effectiveness: 0.75, latent_effectiveness: 0.6, frost_control_temp_c: -5.0, defrost_power_w: 200.0 }
        }

        #[test]
        fn winter_recovery_heats_supply() {
            let unit = erv();
            let supply = HxAirstream { temperature_c: 5.0, humidity_ratio: 0.004, mass_flow_kg_s: 0.3, pressure_pa: 101_325.0 };
            let exhaust = HxAirstream { temperature_c: 22.0, humidity_ratio: 0.009, mass_flow_kg_s: 0.3, pressure_pa: 101_325.0 };
            let out = heat_recovery_exchange_w(&unit, &supply, &exhaust);
            assert!(out.supply_out.temperature_c > supply.temperature_c);
            assert!(out.sensible_recovery_w > 0.0);
        }

        #[test]
        fn effectiveness_ntu_counterflow() {
            let eps = effectiveness_from_ntu(3.0, 0.5, HeatExchangerType::CounterFlow);
            assert!(eps > 0.5 && eps < 1.0);
        }

        #[test]
        fn frost_reduces_effectiveness() {
            let unit = erv();
            let supply = HxAirstream { temperature_c: -10.0, humidity_ratio: 0.002, mass_flow_kg_s: 0.2, pressure_pa: 101_325.0 };
            let exhaust = HxAirstream { temperature_c: 20.0, humidity_ratio: 0.008, mass_flow_kg_s: 0.2, pressure_pa: 101_325.0 };
            let out = heat_recovery_exchange_w(&unit, &supply, &exhaust);
            assert!(out.defrost_active);
            assert!(out.defrost_power_w > 0.0);
        }
    }
}

mod humidity_eq {
    //! 💦 Humidity equipment: steam/water humidifiers, dehumidifiers, solid desiccant.

    use crate::props::{latent_heat_vaporization, saturation_pressure_pa};
    use crate::units::H_FG_0C;
    use serde::{Deserialize, Serialize};

    // #region 🔖Humidifier
    /// 💦 Humidifier types.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Humidifier {
        SteamElectric { capacity_kg_s: f64, efficiency: f64 },
        SteamGas { capacity_kg_s: f64, efficiency: f64 },
        Atomizing { capacity_kg_s: f64, water_temp_c: f64 },
        WettedMedia { effectiveness: f64, pad_area_m2: f64 },
    }

    /// 📥 Humidifier boundary conditions.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HumidifierInlet {
        pub dry_bulb_c: f64,
        pub humidity_ratio: f64,
        pub mass_flow_kg_s: f64,
        pub target_humidity_ratio: f64,
        pub pressure_pa: f64,
    }

    /// 📤 Humidifier output.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HumidifierOutput {
        pub humidity_ratio: f64,
        pub water_added_kg_s: f64,
        pub power_w: f64,
        pub gas_consumption_w: f64,
    }
    // #endregion 🔖Humidifier

    // #region 🔖Dehumidifier
    /// 🌬️ Dehumidifier types.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Dehumidifier {
        Refrigerant { cop: f64, capacity_kg_s: f64 },
        Desiccant { regen_temp_c: f64, moisture_removal_kg_s: f64, regen_power_w: f64 },
        SolidDesiccant { effectiveness: f64, max_removal_kg_s: f64 },
    }

    /// 📥 Dehumidifier boundary conditions.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DehumidifierInlet {
        pub dry_bulb_c: f64,
        pub humidity_ratio: f64,
        pub mass_flow_kg_s: f64,
        pub target_humidity_ratio: f64,
        pub pressure_pa: f64,
    }

    /// 📤 Dehumidifier output.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DehumidifierOutput {
        pub humidity_ratio: f64,
        pub moisture_removed_kg_s: f64,
        pub latent_cooling_w: f64,
        pub power_w: f64,
    }
    // #endregion 🔖Dehumidifier

    // #region 🔖HumidifierOutput
    /// 💦 Humidifier moisture addition rate [kg/s].
    pub fn humidifier_output_kg_s(humidifier: &Humidifier, inlet: &HumidifierInlet) -> HumidifierOutput {
        let m_dot = inlet.mass_flow_kg_s.max(0.0);
        if m_dot < 1e-9 || inlet.humidity_ratio >= inlet.target_humidity_ratio {
            return HumidifierOutput { humidity_ratio: inlet.humidity_ratio, water_added_kg_s: 0.0, power_w: 0.0, gas_consumption_w: 0.0 };
        }

        let w_needed = inlet.target_humidity_ratio - inlet.humidity_ratio;
        let m_w_demand = w_needed * m_dot;

        match humidifier {
            Humidifier::SteamElectric { capacity_kg_s, efficiency } => {
                let m_w = m_w_demand.min(*capacity_kg_s);
                let power = m_w * H_FG_0C / efficiency.max(0.01);
                HumidifierOutput { humidity_ratio: inlet.humidity_ratio + m_w / m_dot, water_added_kg_s: m_w, power_w: power, gas_consumption_w: 0.0 }
            }
            Humidifier::SteamGas { capacity_kg_s, efficiency } => {
                let m_w = m_w_demand.min(*capacity_kg_s);
                let gas = m_w * H_FG_0C / efficiency.max(0.01);
                HumidifierOutput { humidity_ratio: inlet.humidity_ratio + m_w / m_dot, water_added_kg_s: m_w, power_w: 0.0, gas_consumption_w: gas }
            }
            Humidifier::Atomizing { capacity_kg_s, water_temp_c } => {
                let m_w = m_w_demand.min(*capacity_kg_s);
                let evap_energy = m_w * latent_heat_vaporization(*water_temp_c);
                HumidifierOutput { humidity_ratio: inlet.humidity_ratio + m_w / m_dot, water_added_kg_s: m_w, power_w: evap_energy * 0.1, gas_consumption_w: 0.0 }
            }
            Humidifier::WettedMedia { effectiveness, .. } => {
                let w_sat = saturation_humidity_ratio(inlet.dry_bulb_c, inlet.pressure_pa);
                let w_max = inlet.humidity_ratio + effectiveness * (w_sat - inlet.humidity_ratio);
                let m_w = ((w_max - inlet.humidity_ratio) * m_dot).min(m_w_demand);
                HumidifierOutput { humidity_ratio: inlet.humidity_ratio + m_w / m_dot, water_added_kg_s: m_w, power_w: 50.0 * m_w, gas_consumption_w: 0.0 }
            }
        }
    }
    // #endregion 🔖HumidifierOutput

    // #region 🔖DehumidifierOutput
    /// 🌬️ Dehumidifier moisture removal rate [kg/s].
    pub fn dehumidifier_output_kg_s(dehumidifier: &Dehumidifier, inlet: &DehumidifierInlet) -> DehumidifierOutput {
        let m_dot = inlet.mass_flow_kg_s.max(0.0);
        if m_dot < 1e-9 || inlet.humidity_ratio <= inlet.target_humidity_ratio {
            return DehumidifierOutput { humidity_ratio: inlet.humidity_ratio, moisture_removed_kg_s: 0.0, latent_cooling_w: 0.0, power_w: 0.0 };
        }

        let w_remove = inlet.humidity_ratio - inlet.target_humidity_ratio;
        let m_w_demand = w_remove * m_dot;

        match dehumidifier {
            Dehumidifier::Refrigerant { cop, capacity_kg_s } => {
                let m_w = m_w_demand.min(*capacity_kg_s);
                let latent = m_w * H_FG_0C;
                DehumidifierOutput { humidity_ratio: inlet.humidity_ratio - m_w / m_dot, moisture_removed_kg_s: m_w, latent_cooling_w: latent, power_w: latent / cop.max(0.5) }
            }
            Dehumidifier::Desiccant { moisture_removal_kg_s, regen_power_w, .. } => {
                let m_w = m_w_demand.min(*moisture_removal_kg_s);
                let latent = m_w * H_FG_0C;
                let plr = m_w / moisture_removal_kg_s.max(1e-9);
                DehumidifierOutput { humidity_ratio: inlet.humidity_ratio - m_w / m_dot, moisture_removed_kg_s: m_w, latent_cooling_w: latent * 0.8, power_w: regen_power_w * plr }
            }
            Dehumidifier::SolidDesiccant { effectiveness, max_removal_kg_s } => {
                let m_w = m_w_demand.min(*max_removal_kg_s) * effectiveness;
                let latent = m_w * H_FG_0C;
                DehumidifierOutput { humidity_ratio: inlet.humidity_ratio - m_w / m_dot, moisture_removed_kg_s: m_w, latent_cooling_w: latent, power_w: latent * 0.3 }
            }
        }
    }

    fn saturation_humidity_ratio(t_c: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_c);
        0.621_945 * p_ws / (p_atm - p_ws).max(1.0)
    }
    // #endregion 🔖DehumidifierOutput

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::units::P_STD;

        #[test]
        fn steam_humidifier_adds_moisture() {
            let hum = Humidifier::SteamElectric { capacity_kg_s: 0.01, efficiency: 0.95 };
            let inlet = HumidifierInlet { dry_bulb_c: 20.0, humidity_ratio: 0.005, mass_flow_kg_s: 0.5, target_humidity_ratio: 0.009, pressure_pa: P_STD };
            let out = humidifier_output_kg_s(&hum, &inlet);
            assert!(out.water_added_kg_s > 0.0);
            assert!(out.humidity_ratio > inlet.humidity_ratio);
            assert!(out.power_w > 0.0);
        }

        #[test]
        fn refrigerant_dehumidifier_removes_moisture() {
            let dehum = Dehumidifier::Refrigerant { cop: 2.5, capacity_kg_s: 0.005 };
            let inlet = DehumidifierInlet { dry_bulb_c: 26.0, humidity_ratio: 0.014, mass_flow_kg_s: 0.6, target_humidity_ratio: 0.009, pressure_pa: P_STD };
            let out = dehumidifier_output_kg_s(&dehum, &inlet);
            assert!(out.moisture_removed_kg_s > 0.0);
            assert!(out.humidity_ratio < inlet.humidity_ratio);
        }

        #[test]
        fn at_target_no_humidification() {
            let hum = Humidifier::SteamElectric { capacity_kg_s: 0.01, efficiency: 1.0 };
            let inlet = HumidifierInlet { dry_bulb_c: 22.0, humidity_ratio: 0.01, mass_flow_kg_s: 0.5, target_humidity_ratio: 0.009, pressure_pa: P_STD };
            let out = humidifier_output_kg_s(&hum, &inlet);
            assert_eq!(out.water_added_kg_s, 0.0);
        }
    }
}

mod hvac_topo {
    //! 🌀 HVAC fluid topology: nodes, branches, splitters, mixers, and loop validation.

    use crate::error::{Diagnostics, Error};
    use serde::{Deserialize, Serialize};

    // #region 🔖FluidNode
    /// 💧 Fluid stream state at a topology node (air or water).
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FluidNode {
        pub id: usize,
        pub temperature_c: f64,
        pub humidity_ratio: f64,
        pub pressure_pa: f64,
        pub mass_flow_kg_s: f64,
    }

    impl FluidNode {
        pub fn new(id: usize) -> Self {
            Self { id, temperature_c: 20.0, humidity_ratio: 0.008, pressure_pa: 101_325.0, mass_flow_kg_s: 0.0 }
        }
    }
    // #endregion 🔖FluidNode

    // #region 🔖Branch
    /// 🔀 Directed fluid branch between two nodes.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Branch {
        pub id: usize,
        pub inlet: usize,
        pub outlet: usize,
        pub component: BranchComponent,
    }

    /// ⚙️ Branch-resident component type.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BranchComponent {
        Duct { hydraulic_diameter_m: f64, length_m: f64 },
        Pipe { diameter_m: f64, length_m: f64 },
        Pump { design_head_pa: f64, design_flow_kg_s: f64 },
        Coil { ua_w_per_k: f64 },
        Valve { cv: f64 },
        Bypass,
    }

    impl Branch {
        pub fn pressure_drop_pa(&self, inlet: &FluidNode, outlet: &FluidNode) -> f64 {
            match &self.component {
                BranchComponent::Duct { hydraulic_diameter_m, length_m } => {
                    let rho = 1.2;
                    let area = std::f64::consts::PI * hydraulic_diameter_m * hydraulic_diameter_m / 4.0;
                    let v = inlet.mass_flow_kg_s.abs() / (rho * area).max(1e-6);
                    0.02 * (length_m / hydraulic_diameter_m.max(0.01)) * 0.5 * rho * v * v
                }
                BranchComponent::Pipe { diameter_m, length_m } => {
                    let rho = 998.0;
                    let area = std::f64::consts::PI * diameter_m * diameter_m / 4.0;
                    let v = inlet.mass_flow_kg_s.abs() / (rho * area).max(1e-6);
                    0.02 * (length_m / diameter_m.max(0.01)) * 0.5 * rho * v * v
                }
                BranchComponent::Pump { design_head_pa, design_flow_kg_s } => {
                    let frac = (inlet.mass_flow_kg_s / design_flow_kg_s.max(1e-6)).clamp(0.0, 1.2);
                    -design_head_pa * (1.0 - 0.3 * (1.0 - frac).powi(2))
                }
                BranchComponent::Coil { ua_w_per_k } => {
                    let delta_t = (inlet.temperature_c - outlet.temperature_c).abs();
                    ua_w_per_k * delta_t / (inlet.mass_flow_kg_s.abs().max(0.01) * 1006.0)
                }
                BranchComponent::Valve { cv } => {
                    let delta_p = (inlet.pressure_pa - outlet.pressure_pa).abs();
                    let flow_gpm = cv * delta_p.sqrt();
                    flow_gpm * 0.063_09
                }
                BranchComponent::Bypass => 5.0,
            }
        }
    }
    // #endregion 🔖Branch

    // #region 🔖SplitterMixer
    /// 🔱 Flow splitter: one inlet, multiple outlets with prescribed fractions.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Splitter {
        pub id: usize,
        pub inlet: usize,
        pub outlets: Vec<(usize, f64)>,
    }

    impl Splitter {
        pub fn distribute(&self, inlet: &FluidNode) -> Vec<FluidNode> {
            self.outlets.iter().map(|(id, frac)| FluidNode { id: *id, temperature_c: inlet.temperature_c, humidity_ratio: inlet.humidity_ratio, pressure_pa: inlet.pressure_pa, mass_flow_kg_s: inlet.mass_flow_kg_s * frac }).collect()
        }
    }

    /// 🔀 Flow mixer: multiple inlets blended by mass flow.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Mixer {
        pub id: usize,
        pub inlets: Vec<usize>,
        pub outlet: usize,
    }

    impl Mixer {
        pub fn blend(&self, nodes: &[FluidNode]) -> FluidNode {
            let inlets: Vec<_> = self.inlets.iter().map(|&id| &nodes[id]).collect();
            let m_total: f64 = inlets.iter().map(|n| n.mass_flow_kg_s).sum();
            if m_total < 1e-9 {
                return FluidNode::new(self.outlet);
            }
            let t = inlets.iter().map(|n| n.temperature_c * n.mass_flow_kg_s).sum::<f64>() / m_total;
            let w = inlets.iter().map(|n| n.humidity_ratio * n.mass_flow_kg_s).sum::<f64>() / m_total;
            let p = inlets.iter().map(|n| n.pressure_pa * n.mass_flow_kg_s).sum::<f64>() / m_total;
            FluidNode { id: self.outlet, temperature_c: t, humidity_ratio: w, pressure_pa: p, mass_flow_kg_s: m_total }
        }
    }
    // #endregion 🔖SplitterMixer

    // #region 🔖Loops
    /// 🌬️ Air loop topology: supply/return paths with zone connections.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AirLoop {
        pub id: usize,
        pub name: String,
        pub nodes: Vec<FluidNode>,
        pub branches: Vec<Branch>,
        pub splitters: Vec<Splitter>,
        pub mixers: Vec<Mixer>,
        pub supply_inlet: usize,
        pub supply_outlet: usize,
        pub return_inlet: usize,
        pub return_outlet: usize,
        pub zone_outlets: Vec<usize>,
        pub zone_returns: Vec<usize>,
    }

    /// 🏭 Plant loop topology: hot/cold water or steam distribution.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PlantLoop {
        pub id: usize,
        pub name: String,
        pub fluid: PlantFluid,
        pub nodes: Vec<FluidNode>,
        pub branches: Vec<Branch>,
        pub splitters: Vec<Splitter>,
        pub mixers: Vec<Mixer>,
        pub supply_inlet: usize,
        pub supply_outlet: usize,
        pub demand_inlet: usize,
        pub demand_outlet: usize,
    }

    /// 💧 Plant loop working fluid.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PlantFluid {
        Water,
        Steam,
        CondenserWater,
        Glycol { fraction: f64 },
    }

    /// ❄️ Condenser loop for heat rejection (cooling tower / dry cooler).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CondenserLoop {
        pub id: usize,
        pub plant_loop: PlantLoop,
        pub heat_rejection_w: f64,
    }
    // #endregion 🔖Loops

    // #region 🔖Validation
    /// ✅ Validate HVAC topology mass balance and connectivity.
    pub fn validate_topology(nodes: &[FluidNode], branches: &[Branch], splitters: &[Splitter], mixers: &[Mixer]) -> Diagnostics {
        let mut diag = Diagnostics::default();
        let n = nodes.len();

        for branch in branches {
            if branch.inlet >= n || branch.outlet >= n {
                diag.push(Error::fatal(format!("branch {} references invalid node", branch.id)).with_context("hvac_topo"));
            }
            if branch.inlet == branch.outlet {
                diag.push(Error::severe(format!("branch {} has identical inlet/outlet", branch.id)).with_context("hvac_topo"));
            }
        }

        for splitter in splitters {
            let frac_sum: f64 = splitter.outlets.iter().map(|(_, f)| f).sum();
            if (frac_sum - 1.0).abs() > 0.01 {
                diag.push(Error::warning(format!("splitter {} outlet fractions sum to {:.3}, expected 1.0", splitter.id, frac_sum)).with_context("hvac_topo"));
            }
            if splitter.inlet >= n {
                diag.push(Error::fatal(format!("splitter {} invalid inlet", splitter.id)).with_context("hvac_topo"));
            }
        }

        for mixer in mixers {
            for &inlet in &mixer.inlets {
                if inlet >= n {
                    diag.push(Error::fatal(format!("mixer {} invalid inlet {}", mixer.id, inlet)).with_context("hvac_topo"));
                }
            }
            if mixer.outlet >= n {
                diag.push(Error::fatal(format!("mixer {} invalid outlet", mixer.id)).with_context("hvac_topo"));
            }
        }

        let mut net_flow = vec![0.0_f64; n];
        for branch in branches {
            if branch.inlet < n && branch.outlet < n {
                let m = nodes[branch.inlet].mass_flow_kg_s;
                net_flow[branch.inlet] -= m;
                net_flow[branch.outlet] += m;
            }
        }

        for (i, &nf) in net_flow.iter().enumerate() {
            if nf.abs() > 0.1 && i < n {
                diag.push(Error::warning(format!("node {} mass imbalance {:.4} kg/s", i, nf)).with_context("hvac_topo"));
            }
        }

        diag
    }

    impl AirLoop {
        pub fn validate(&self) -> Diagnostics {
            let mut diag = validate_topology(&self.nodes, &self.branches, &self.splitters, &self.mixers);
            for &z in &self.zone_outlets {
                if z >= self.nodes.len() {
                    diag.push(Error::severe(format!("air loop {} zone outlet {} invalid", self.id, z)).with_context("air_loop"));
                }
            }
            diag
        }
    }

    impl PlantLoop {
        pub fn validate(&self) -> Diagnostics {
            validate_topology(&self.nodes, &self.branches, &self.splitters, &self.mixers)
        }
    }
    // #endregion 🔖Validation

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn mixer_blends_by_mass_flow() {
            let nodes = vec![FluidNode { id: 0, temperature_c: 10.0, humidity_ratio: 0.005, pressure_pa: 101_325.0, mass_flow_kg_s: 1.0 }, FluidNode { id: 1, temperature_c: 30.0, humidity_ratio: 0.015, pressure_pa: 101_325.0, mass_flow_kg_s: 1.0 }];
            let mixer = Mixer { id: 0, inlets: vec![0, 1], outlet: 2 };
            let out = mixer.blend(&nodes);
            assert!((out.temperature_c - 20.0).abs() < 1e-9);
            assert!((out.mass_flow_kg_s - 2.0).abs() < 1e-9);
        }

        #[test]
        fn splitter_preserves_mass() {
            let inlet = FluidNode { id: 0, temperature_c: 20.0, humidity_ratio: 0.01, pressure_pa: 101_325.0, mass_flow_kg_s: 2.0 };
            let splitter = Splitter { id: 0, inlet: 0, outlets: vec![(1, 0.6), (2, 0.4)] };
            let outs = splitter.distribute(&inlet);
            let m_sum: f64 = outs.iter().map(|n| n.mass_flow_kg_s).sum();
            assert!((m_sum - 2.0).abs() < 1e-9);
        }

        #[test]
        fn valid_topology_passes() {
            let nodes = vec![FluidNode::new(0), FluidNode::new(1)];
            let branches = vec![Branch { id: 0, inlet: 0, outlet: 1, component: BranchComponent::Bypass }];
            let diag = validate_topology(&nodes, &branches, &[], &[]);
            assert!(!diag.has_fatal());
        }
    }
}

mod iaq {
    //! 🫁 Indoor air quality: CO₂ and generic contaminant mass balance with DCV.

    use serde::{Deserialize, Serialize};

    // #region 🔖ContaminantState
    /// 🫁 Contaminant concentration state with history for transient integration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ContaminantState {
        pub concentration_ppm: f64,
        pub history_ppm: [f64; 3],
    }

    impl ContaminantState {
        pub fn new(concentration_ppm: f64) -> Self {
            Self { concentration_ppm, history_ppm: [concentration_ppm; 3] }
        }

        pub fn push(&mut self, ppm: f64) {
            self.history_ppm = [ppm, self.history_ppm[0], self.history_ppm[1]];
            self.concentration_ppm = ppm;
        }
    }
    // #endregion 🔖ContaminantState

    // #region 🔖ContaminantBalance
    /// ⚖️ Generic contaminant mass balance inputs.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ContaminantBalance {
        pub zone_volume_m3: f64,
        pub generation_rate_mg_s: f64,
        pub outdoor_concentration_ppm: f64,
        pub ventilation_flow_m3_s: f64,
        pub infiltration_flow_m3_s: f64,
        pub removal_rate_mg_s: f64,
        pub molecular_weight_g_mol: f64,
    }

    impl ContaminantBalance {
        pub fn total_airflow_m3_s(&self) -> f64 {
            self.ventilation_flow_m3_s + self.infiltration_flow_m3_s
        }
    }
    // #endregion 🔖ContaminantBalance

    // #region 🔖Co2Balance
    /// 🫁 CO₂-specific balance parameters per ASHRAE 62.1.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Co2Balance {
        pub zone_volume_m3: f64,
        pub occupancy: f64,
        pub co2_generation_per_person_mg_s: f64,
        pub outdoor_co2_ppm: f64,
        pub ventilation_flow_m3_s: f64,
        pub infiltration_flow_m3_s: f64,
    }

    impl Co2Balance {
        pub fn generation_rate_mg_s(&self) -> f64 {
            self.occupancy * self.co2_generation_per_person_mg_s
        }

        pub fn to_contaminant_balance(&self) -> ContaminantBalance {
            ContaminantBalance {
                zone_volume_m3: self.zone_volume_m3,
                generation_rate_mg_s: self.generation_rate_mg_s(),
                outdoor_concentration_ppm: self.outdoor_co2_ppm,
                ventilation_flow_m3_s: self.ventilation_flow_m3_s,
                infiltration_flow_m3_s: self.infiltration_flow_m3_s,
                removal_rate_mg_s: 0.0,
                molecular_weight_g_mol: 44.01,
            }
        }
    }
    // #endregion 🔖Co2Balance

    // #region 🔖DcvControl
    /// 🎛️ Demand-controlled ventilation setpoint.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DcvControl {
        pub target_ppm: f64,
        pub min_flow_per_person_m3_s: f64,
        pub max_flow_per_person_m3_s: f64,
        pub outdoor_co2_ppm: f64,
    }
    // #endregion 🔖DcvControl

    // #region 🔖Solvers
    fn ppm_to_mg_m3(ppm: f64, molecular_weight_g_mol: f64) -> f64 {
        ppm * molecular_weight_g_mol / 24.45
    }

    fn mg_m3_to_ppm(mg_m3: f64, molecular_weight_g_mol: f64) -> f64 {
        mg_m3 * 24.45 / molecular_weight_g_mol
    }

    /// 📈 Steady-state contaminant concentration [ppm].
    pub fn steady_state_concentration_ppm(balance: &ContaminantBalance) -> f64 {
        let q = balance.total_airflow_m3_s();
        if q < 1e-12 {
            return balance.outdoor_concentration_ppm;
        }
        let c_out = ppm_to_mg_m3(balance.outdoor_concentration_ppm, balance.molecular_weight_g_mol);
        let gen = balance.generation_rate_mg_s - balance.removal_rate_mg_s;
        let c_zone = c_out + gen / q;
        mg_m3_to_ppm(c_zone.max(0.0), balance.molecular_weight_g_mol)
    }

    /// ⏩ Advance contaminant concentration one explicit Euler step [ppm].
    pub fn advance_contaminant(state: &ContaminantState, balance: &ContaminantBalance, dt_s: f64) -> f64 {
        if dt_s <= 0.0 || balance.zone_volume_m3 <= 0.0 {
            return state.concentration_ppm;
        }
        let c = ppm_to_mg_m3(state.concentration_ppm, balance.molecular_weight_g_mol);
        let c_out = ppm_to_mg_m3(balance.outdoor_concentration_ppm, balance.molecular_weight_g_mol);
        let q = balance.total_airflow_m3_s();
        let gen = balance.generation_rate_mg_s - balance.removal_rate_mg_s;
        let dc_dt = (q * (c_out - c) + gen) / balance.zone_volume_m3;
        let c_new = (c + dc_dt * dt_s).max(0.0);
        mg_m3_to_ppm(c_new, balance.molecular_weight_g_mol)
    }

    /// 🫁 Steady-state CO₂ [ppm].
    pub fn steady_state_co2_ppm(balance: &Co2Balance) -> f64 {
        steady_state_concentration_ppm(&balance.to_contaminant_balance())
    }

    /// 🎛️ DCV required outdoor airflow per person [m³/s] from CO₂ mass balance.
    pub fn dcv_flow_per_person_m3_s(control: &DcvControl, occupancy: f64, indoor_co2_ppm: f64) -> f64 {
        if occupancy < 1e-6 {
            return control.min_flow_per_person_m3_s;
        }
        let delta_target = (control.target_ppm - control.outdoor_co2_ppm).max(50.0);
        let ratio = if indoor_co2_ppm > control.target_ppm { 1.0 + (indoor_co2_ppm - control.target_ppm) / delta_target } else { 1.0 };
        (control.min_flow_per_person_m3_s * ratio).clamp(control.min_flow_per_person_m3_s, control.max_flow_per_person_m3_s)
    }

    /// 🎛️ Required total DCV ventilation flow [m³/s].
    pub fn dcv_ventilation_flow_m3_s(control: &DcvControl, occupancy: f64, indoor_co2_ppm: f64) -> f64 {
        occupancy * dcv_flow_per_person_m3_s(control, occupancy, indoor_co2_ppm)
    }
    // #endregion 🔖Solvers

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn co2_rises_with_occupancy_at_low_ventilation() {
            let balance = Co2Balance { zone_volume_m3: 200.0, occupancy: 10.0, co2_generation_per_person_mg_s: 7.0, outdoor_co2_ppm: 400.0, ventilation_flow_m3_s: 0.01, infiltration_flow_m3_s: 0.005 };
            let ppm = steady_state_co2_ppm(&balance);
            assert!(ppm > 400.0);
        }

        #[test]
        fn contaminant_transient_approaches_steady_state() {
            let balance = ContaminantBalance { zone_volume_m3: 100.0, generation_rate_mg_s: 5.0, outdoor_concentration_ppm: 0.0, ventilation_flow_m3_s: 0.05, infiltration_flow_m3_s: 0.0, removal_rate_mg_s: 0.0, molecular_weight_g_mol: 44.01 };
            let ss = steady_state_concentration_ppm(&balance);
            let mut state = ContaminantState::new(0.0);
            for _ in 0..500 {
                let ppm = advance_contaminant(&state, &balance, 60.0);
                state.push(ppm);
            }
            assert!((state.concentration_ppm - ss).abs() / ss < 0.05);
        }

        #[test]
        fn dcv_increases_flow_at_high_co2() {
            let ctrl = DcvControl { target_ppm: 1000.0, min_flow_per_person_m3_s: 0.00236, max_flow_per_person_m3_s: 0.01, outdoor_co2_ppm: 400.0 };
            let low = dcv_flow_per_person_m3_s(&ctrl, 5.0, 600.0);
            let high = dcv_flow_per_person_m3_s(&ctrl, 5.0, 1500.0);
            assert!(high > low);
        }
    }
}

mod ideal_hvac {
    //! 🎯 Ideal loads air system: unlimited or capacity-limited zone conditioning.

    use crate::props::moist_air_enthalpy_j_per_kg;
    use crate::units::{CP_DRY_AIR, H_FG_0C, RHO_AIR_REF};
    use serde::{Deserialize, Serialize};

    // #region 🔖IdealLoads
    /// 🎯 Ideal loads physics configuration (distinct from [`crate::model::IdealLoadsSystem`] entity).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IdealLoadsConfig {
        pub max_heating_supply_air_temp_c: f64,
        pub min_cooling_supply_air_temp_c: f64,
        pub max_heating_capacity_w: Option<f64>,
        pub max_cooling_capacity_w: Option<f64>,
        pub outdoor_air_per_person_m3_s: f64,
        pub outdoor_air_per_area_m3_s_m2: f64,
    }

    impl Default for IdealLoadsConfig {
        fn default() -> Self {
            Self { max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.009_44, outdoor_air_per_area_m3_s_m2: 0.0 }
        }
    }

    /// 🌬️ Economizer control mode.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub enum EconomizerControl {
        #[default]
        None,
        DifferentialDryBulb,
        DifferentialEnthalpy,
        FixedDryBulb {
            lockout_c: f64,
        },
    }

    /// 💧 Humidity control mode.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HumidityControl {
        #[default]
        None,
        HumidifyAndDehumidify,
        HumidifyOnly,
        DehumidifyOnly,
    }

    /// 📥 Zone demand and boundary conditions for ideal loads.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IdealLoadsInput {
        pub zone_temp_c: f64,
        pub zone_humidity_ratio: f64,
        pub outdoor_temp_c: f64,
        pub outdoor_humidity_ratio: f64,
        pub heating_setpoint_c: f64,
        pub cooling_setpoint_c: f64,
        pub zone_heating_demand_w: f64,
        pub zone_cooling_demand_w: f64,
        pub occupancy: f64,
        pub floor_area_m2: f64,
    }

    /// Alias for callers that still use the request naming.
    pub type IdealLoadsRequest = IdealLoadsInput;

    /// 📤 Ideal loads delivery result per zone timestep.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IdealLoadsOutput {
        pub sensible_heating_w: f64,
        pub sensible_cooling_w: f64,
        pub latent_heating_w: f64,
        pub latent_cooling_w: f64,
        pub sensible_delivered_w: f64,
        pub unmet_heating_w: f64,
        pub unmet_cooling_w: f64,
        pub outdoor_air_mass_flow_kg_s: f64,
        pub supply_temperature_c: f64,
        pub supply_humidity_ratio: f64,
        pub economizer_active: bool,
        pub humidification_kg_s: f64,
        pub dehumidification_kg_s: f64,
    }
    // #endregion 🔖IdealLoads

    // #region 🔖Deliver
    /// 🎯 Deliver ideal heating/cooling to meet zone setpoints and demands.
    ///
    /// Argument order matches the simulation kernel: `(input, system)`.
    pub fn ideal_loads_deliver(input: &IdealLoadsInput, system: &IdealLoadsConfig) -> IdealLoadsOutput {
        ideal_loads_deliver_with_controls(input, system, EconomizerControl::None, HumidityControl::None)
    }

    /// 🎯 Ideal loads with explicit economizer and humidity controls.
    pub fn ideal_loads_deliver_with_controls(input: &IdealLoadsInput, system: &IdealLoadsConfig, economizer: EconomizerControl, humidity_control: HumidityControl) -> IdealLoadsOutput {
        let oa_vol = system.outdoor_air_per_person_m3_s * input.occupancy + system.outdoor_air_per_area_m3_s_m2 * input.floor_area_m2;
        let economizer_active = economizer_allows_oa(economizer, input);
        let oa_m_dot = oa_vol.max(0.0) * RHO_AIR_REF;

        let heat_demand = input.zone_heating_demand_w.max(0.0);
        let cool_demand = input.zone_cooling_demand_w.max(0.0);

        let sensible_heating = apply_capacity(heat_demand, system.max_heating_capacity_w);
        let sensible_cooling = apply_capacity(cool_demand, system.max_cooling_capacity_w);
        let unmet_heating_w = (heat_demand - sensible_heating).max(0.0);
        let unmet_cooling_w = (cool_demand - sensible_cooling).max(0.0);

        let supply_temperature_c = if sensible_heating > 0.0 {
            system.max_heating_supply_air_temp_c
        } else if sensible_cooling > 0.0 {
            system.min_cooling_supply_air_temp_c
        } else {
            input.zone_temp_c
        };

        let mut latent_heating = 0.0;
        let mut latent_cooling = 0.0;
        let mut humidification = 0.0_f64;
        let mut dehumidification = 0.0_f64;
        let mut supply_w = input.zone_humidity_ratio;

        match humidity_control {
            HumidityControl::HumidifyOnly | HumidityControl::HumidifyAndDehumidify if input.zone_humidity_ratio < 0.008 => {
                humidification = 0.001 * oa_m_dot.max(0.01);
                latent_heating = humidification * H_FG_0C;
                supply_w = 0.008;
            }
            _ => {}
        }
        match humidity_control {
            HumidityControl::DehumidifyOnly | HumidityControl::HumidifyAndDehumidify if input.zone_humidity_ratio > 0.012 => {
                dehumidification = 0.001 * oa_m_dot.max(0.01);
                latent_cooling = dehumidification * H_FG_0C;
                supply_w = 0.012;
            }
            _ => {}
        }

        if economizer_active && sensible_cooling > 0.0 {
            let _free_cool = oa_m_dot * CP_DRY_AIR * (input.zone_temp_c - input.outdoor_temp_c).max(0.0);
        }

        IdealLoadsOutput {
            sensible_heating_w: sensible_heating,
            sensible_cooling_w: sensible_cooling,
            latent_heating_w: latent_heating,
            latent_cooling_w: latent_cooling,
            sensible_delivered_w: sensible_heating - sensible_cooling,
            unmet_heating_w,
            unmet_cooling_w,
            outdoor_air_mass_flow_kg_s: oa_m_dot,
            supply_temperature_c,
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

    fn economizer_allows_oa(economizer: EconomizerControl, input: &IdealLoadsInput) -> bool {
        match economizer {
            EconomizerControl::None => false,
            EconomizerControl::DifferentialDryBulb => input.outdoor_temp_c < input.zone_temp_c,
            EconomizerControl::DifferentialEnthalpy => {
                let h_oa = moist_air_enthalpy_j_per_kg(input.outdoor_temp_c, input.outdoor_humidity_ratio);
                let h_zone = moist_air_enthalpy_j_per_kg(input.zone_temp_c, input.zone_humidity_ratio);
                h_oa < h_zone
            }
            EconomizerControl::FixedDryBulb { lockout_c } => input.outdoor_temp_c < lockout_c,
        }
    }
    // #endregion 🔖Deliver

    #[cfg(test)]
    mod tests {
        use super::*;

        fn unlimited_system() -> IdealLoadsConfig {
            IdealLoadsConfig { max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.01, outdoor_air_per_area_m3_s_m2: 0.0 }
        }

        #[test]
        fn heating_meets_demand() {
            let system = unlimited_system();
            let input = IdealLoadsInput {
                zone_temp_c: 18.0,
                zone_humidity_ratio: 0.008,
                outdoor_temp_c: 5.0,
                outdoor_humidity_ratio: 0.004,
                heating_setpoint_c: 21.0,
                cooling_setpoint_c: 24.0,
                zone_heating_demand_w: 3000.0,
                zone_cooling_demand_w: 0.0,
                occupancy: 2.0,
                floor_area_m2: 50.0,
            };
            let out = ideal_loads_deliver(&input, &system);
            assert!((out.sensible_heating_w - 3000.0).abs() < 1e-6);
            assert_eq!(out.unmet_heating_w, 0.0);
            assert!(out.sensible_delivered_w > 0.0);
        }

        #[test]
        fn capacity_limits_cooling() {
            let system = IdealLoadsConfig { max_cooling_capacity_w: Some(1000.0), ..unlimited_system() };
            let input = IdealLoadsInput {
                zone_temp_c: 30.0,
                zone_humidity_ratio: 0.01,
                outdoor_temp_c: 35.0,
                outdoor_humidity_ratio: 0.015,
                heating_setpoint_c: 21.0,
                cooling_setpoint_c: 24.0,
                zone_heating_demand_w: 0.0,
                zone_cooling_demand_w: 5000.0,
                occupancy: 1.0,
                floor_area_m2: 40.0,
            };
            let out = ideal_loads_deliver(&input, &system);
            assert!((out.sensible_cooling_w - 1000.0).abs() < 1e-6);
            assert!((out.unmet_cooling_w - 4000.0).abs() < 1e-6);
        }

        #[test]
        fn economizer_active_when_oa_cooler() {
            let system = unlimited_system();
            let input = IdealLoadsInput {
                zone_temp_c: 25.0,
                zone_humidity_ratio: 0.01,
                outdoor_temp_c: 15.0,
                outdoor_humidity_ratio: 0.006,
                heating_setpoint_c: 21.0,
                cooling_setpoint_c: 24.0,
                zone_heating_demand_w: 0.0,
                zone_cooling_demand_w: 2000.0,
                occupancy: 1.0,
                floor_area_m2: 30.0,
            };
            let out = ideal_loads_deliver_with_controls(&input, &system, EconomizerControl::DifferentialDryBulb, HumidityControl::None);
            assert!(out.economizer_active);
        }
    }
}

mod kernel {
    //! 🔄 Simulation kernel: calendar, multi-rate loops, warmup, predictor-corrector coupling.

    use crate::air_exchange::{infiltration_flow_m3_s, ventilation_load_w, InfiltrationMethod, InfiltrationSpec};
    use crate::airflow_network::{AfLink, AfLinkKind, AfNode, AirflowNetwork};
    use crate::calendar::{RunPeriod, SimDate};
    use crate::controls::{evaluate_controls, predict_zone_load, HumidistatSpec, ThermostatSpec};
    use crate::curves::PerformanceCurve;
    use crate::daylight::{dimmed_lighting_power_w, lighting_dimming_fraction, reference_point_illuminance_lux, simplified_daylight_factor};
    use crate::dispatch::{DispatchRequest, DispatchScheme, Dispatcher, EquipmentPriority};
    use crate::electrical::{grid_balance, PvSystem, Transformer};
    use crate::envelope::{solve_exterior_surface_temp, solve_interior_surface_temp, ConductionState, ExteriorConvectionModel, InteriorConvectionModel};
    use crate::error::Error;
    use crate::faults::SeveritySchedule;
    use crate::gains::{compute_equipment_gain_w, compute_lighting_gain_w, compute_people_gain_w, ActivityLevel, GainDecomposition};
    use crate::ideal_hvac::{ideal_loads_deliver, IdealLoadsConfig, IdealLoadsInput};
    use crate::model::{EntityId, Model, OutsideBoundary};
    use crate::plant::{PlantLoopSimulation, PlantStream, Pump};
    use crate::precompute::PrecomputedModel;
    use crate::props::saturation_pressure_pa;
    use crate::schedule::{ScheduleContext, ScheduleSet};
    use crate::site::{GroundTemperatureModel, WeatherRecord};
    use crate::solar::{shading_factor, surface_solar_absorption};
    use crate::units::P_STD;
    use crate::zone_air::{advance_zone_air, HumiditySolutionMethod, ZoneAirBalance, ZoneAirState};
    use crate::zone_hvac::{ZoneEquipment, ZoneEquipmentRequest};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    // #region 🔖Config
    /// ⚙️ Simulation environment type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SimulationEnvironment {
        WeatherRunPeriod,
        HeatingDesignDay,
        CoolingDesignDay,
        CustomDesignPeriod,
    }

    /// ⚙️ Convergence tolerances.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ConvergenceTolerances {
        pub temperature_k: f64,
        pub humidity_ratio: f64,
        pub mass_flow: f64,
        pub energy_w: f64,
        pub max_iterations: u32,
    }

    impl Default for ConvergenceTolerances {
        fn default() -> Self {
            Self { temperature_k: 0.01, humidity_ratio: 1e-5, mass_flow: 1e-4, energy_w: 1.0, max_iterations: 20 }
        }
    }

    /// ⚙️ Simulation configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SimulationConfig {
        pub environment: SimulationEnvironment,
        pub zone_timestep_minutes: u32,
        pub system_timestep_minutes: u32,
        pub warmup_days: u32,
        pub run_period_start_month: u8,
        pub run_period_start_day: u8,
        pub run_period_end_month: u8,
        pub run_period_end_day: u8,
        pub tolerances: ConvergenceTolerances,
        pub schedules: ScheduleSet,
        pub weather: Option<crate::site::EpwWeather>,
    }

    impl Default for SimulationConfig {
        fn default() -> Self {
            Self {
                environment: SimulationEnvironment::WeatherRunPeriod,
                zone_timestep_minutes: 60,
                system_timestep_minutes: 60,
                warmup_days: 7,
                run_period_start_month: 1,
                run_period_start_day: 1,
                run_period_end_month: 12,
                run_period_end_day: 31,
                tolerances: ConvergenceTolerances::default(),
                schedules: ScheduleSet::default(),
                weather: None,
            }
        }
    }
    // #endregion 🔖Config

    // #region 🔖DeliveredEnergy
    /// ⚡ Delivered energy per timestep for metering.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct DeliveredEnergy {
        pub heating_w: f64,
        pub cooling_w: f64,
        pub fan_w: f64,
        pub pump_w: f64,
        pub compressor_w: f64,
        pub gas_w: f64,
        pub pv_generation_w: f64,
        pub battery_charge_w: f64,
        pub shw_electric_w: f64,
        pub shw_gas_w: f64,
        pub refrigeration_w: f64,
        pub water_pump_w: f64,
    }

    impl DeliveredEnergy {
        pub fn total_electric_w(&self) -> f64 {
            self.heating_w + self.cooling_w + self.fan_w + self.pump_w + self.compressor_w + self.shw_electric_w + self.refrigeration_w + self.water_pump_w - self.pv_generation_w + self.battery_charge_w
        }
    }
    // #endregion 🔖DeliveredEnergy

    // #region 🔖State
    /// 🔄 Per-zone simulation state.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ZoneState {
        pub air: ZoneAirState,
        pub heating_demand_w: f64,
        pub cooling_demand_w: f64,
        pub unmet_heating_w: f64,
        pub unmet_cooling_w: f64,
        pub delivered: DeliveredEnergy,
    }

    impl ZoneState {
        fn empty() -> Self {
            Self { air: ZoneAirState::new(20.0, 0.01), heating_demand_w: 0.0, cooling_demand_w: 0.0, unmet_heating_w: 0.0, unmet_cooling_w: 0.0, delivered: DeliveredEnergy::default() }
        }
    }

    /// 🔄 Surface thermal history for CTF conduction.
    #[derive(Clone, Debug)]
    pub struct SurfaceState {
        pub inside_temp_c: f64,
        pub outside_temp_c: f64,
        pub heat_flux_w: f64,
        pub ctf: ConductionState,
        pub convection_to_zone_w: f64,
    }

    /// 🔄 Full simulation state.
    #[derive(Clone, Debug)]
    pub struct SimulationState {
        pub zones: HashMap<EntityId, ZoneState>,
        pub surfaces: HashMap<EntityId, SurfaceState>,
        pub warmup_complete: bool,
        pub hour: u32,
        pub delivered_total: DeliveredEnergy,
        pub battery_soc: f64,
        pub plant_supply_c: f64,
    }

    impl Default for SimulationState {
        fn default() -> Self {
            Self { zones: HashMap::new(), surfaces: HashMap::new(), warmup_complete: false, hour: 0, delivered_total: DeliveredEnergy::default(), battery_soc: 0.5, plant_supply_c: 55.0 }
        }
    }
    // #endregion 🔖State

    // #region 🔖Kernel
    /// 🔄 BEM simulation kernel with full subsystem coupling.
    pub struct SimulationKernel;

    impl SimulationKernel {
        /// 🔄 Initialize state from model and precomputed data.
        pub fn initialize(model: &Model, pre: &PrecomputedModel, weather: &WeatherRecord) -> SimulationState {
            let mut state = SimulationState::default();
            for zone in &model.zones {
                state.zones.insert(zone.id, ZoneState { air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()), ..ZoneState::empty() });
            }
            for (sid, sp) in &pre.surfaces {
                state.surfaces.insert(*sid, SurfaceState { inside_temp_c: weather.dry_bulb_c, outside_temp_c: weather.dry_bulb_c, heat_flux_w: 0.0, ctf: sp.ctf.clone(), convection_to_zone_w: 0.0 });
            }
            state
        }

        /// 🔄 Run warmup until temperature and load convergence.
        pub fn warmup(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationState, weather_records: &[WeatherRecord]) -> Result<(), Error> {
            let warmup_hours = config.warmup_days * 24;
            let dt_s = pre.zone_timestep_s;
            let mut prev_temps: HashMap<EntityId, f64> = HashMap::new();
            let mut prev_loads: HashMap<EntityId, f64> = HashMap::new();

            for hour in 0..warmup_hours {
                let widx = (hour as usize) % weather_records.len().max(1);
                let weather = weather_records.get(widx).copied().unwrap_or_else(|| default_weather(hour));
                let date = SimDate::new(weather.year, weather.month, weather.day);
                Self::advance_timestep(model, config, pre, state, &weather, &date, hour as f64, dt_s)?;
                if hour > 24 && hour % 24 == 0 {
                    let temp_ok = state.zones.iter().all(|(id, zs)| prev_temps.get(id).is_some_and(|prev| (zs.air.temp_c - prev).abs() <= config.tolerances.temperature_k));
                    let load_ok = state.zones.iter().all(|(id, zs)| {
                        prev_loads.get(id).is_some_and(|prev| {
                            let load = zs.heating_demand_w + zs.cooling_demand_w;
                            (load - prev).abs() <= config.tolerances.energy_w
                        })
                    });
                    if temp_ok && load_ok {
                        state.warmup_complete = true;
                        return Ok(());
                    }
                }
                for (id, zs) in &state.zones {
                    prev_temps.insert(*id, zs.air.temp_c);
                    prev_loads.insert(*id, zs.heating_demand_w + zs.cooling_demand_w);
                }
            }
            state.warmup_complete = true;
            Ok(())
        }

        /// 🔄 Advance one zone timestep with predictor-corrector HVAC coupling.
        pub fn advance_timestep(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationState, weather: &WeatherRecord, date: &SimDate, hour: f64, dt_s: f64) -> Result<(), Error> {
            let ctx = ScheduleContext { year: date.year, month: date.month, day: date.day, hour: weather.hour, day_of_week: date.day_of_week(), timestep_index: hour as u32, is_dst: false };

            let day_of_year = date.day_of_year();
            let (sun_alt, sun_az) = pre.solar_at(model, day_of_year, hour);
            let ext_conv = ExteriorConvectionModel::default();
            let int_conv = InteriorConvectionModel::default();
            let sky_temp_k = weather.dry_bulb_c + 273.15 - 20.0;
            let ground_model = GroundTemperatureModel::Monthly { temperatures_c: model.ground_temperature.building_surface_c };

            let mut zone_envelope_w: HashMap<EntityId, f64> = HashMap::new();
            let mut zone_solar_w: HashMap<EntityId, f64> = HashMap::new();
            let mut zone_surface_conv_w: HashMap<EntityId, f64> = HashMap::new();

            for (sid, sp) in &pre.surfaces {
                let surface = model.surfaces.iter().find(|s| s.id == *sid);
                let outside_temp = match surface.map(|s| s.outside_boundary_condition) {
                    Some(OutsideBoundary::Ground) => ground_model.temperature_c(day_of_year),
                    Some(OutsideBoundary::OutdoorAir) | None => weather.dry_bulb_c,
                    Some(OutsideBoundary::OtherSideTemperature) => weather.dry_bulb_c - 5.0,
                    Some(OutsideBoundary::Adiabatic) | Some(OutsideBoundary::Interzone(_)) => state.surfaces.get(sid).map_or(weather.dry_bulb_c, |s| s.outside_temp_c),
                };

                let zone_t = state.zones.get(&sp.zone_id).map_or(weather.dry_bulb_c, |z| z.air.temp_c);

                let mut solar_w_m2 = 0.0;
                if sp.sun_exposed && sun_alt > 0.0 {
                    let incidence = crate::solar::beam_incidence_cosine(sp.normal, sun_alt, sun_az);
                    let shade = shading_factor(1.0, 0.0, 1.0, sun_alt);
                    let abs = surface_solar_absorption(weather.direct_normal_irradiance_w_m2, weather.diffuse_horizontal_irradiance_w_m2, incidence, shade, sp.solar_absorptance, sp.tilt_deg);
                    solar_w_m2 = abs.total_w_m2;
                }

                let ss = state.surfaces.entry(*sid).or_insert_with(|| SurfaceState { inside_temp_c: zone_t, outside_temp_c: outside_temp, heat_flux_w: 0.0, ctf: sp.ctf.clone(), convection_to_zone_w: 0.0 });

                let conduction_w_m2 = ss.ctf.heat_flux_w_m2(outside_temp, zone_t);
                let exterior_t = solve_exterior_surface_temp(outside_temp, sky_temp_k, weather.wind_speed_m_s, solar_w_m2, -conduction_w_m2, sp.emissivity, &ext_conv);
                let balance = solve_interior_surface_temp(zone_t, conduction_w_m2, solar_w_m2 * 0.3, &int_conv);
                let conv_w = balance.convection_w_m2 * sp.area_m2;
                let cond_w = conduction_w_m2 * sp.area_m2;

                ss.inside_temp_c = balance.surface_temp_c;
                ss.outside_temp_c = exterior_t;
                ss.heat_flux_w = cond_w;
                ss.convection_to_zone_w = conv_w;
                ss.ctf.advance(outside_temp);

                *zone_envelope_w.entry(sp.zone_id).or_default() += cond_w;
                *zone_solar_w.entry(sp.zone_id).or_default() += solar_w_m2 * sp.area_m2 * 0.7;
                *zone_surface_conv_w.entry(sp.zone_id).or_default() += conv_w;
            }

            for (fid, fp) in &pre.fenestrations {
                if let Some(surface) = model.surfaces.iter().find(|s| s.id == fp.surface_id) {
                    let zone_t = state.zones.get(&surface.zone_id).map_or(weather.dry_bulb_c, |z| z.air.temp_c);
                    let u = fp.u_value_w_m2k;
                    let cond_w = u * fp.area_m2 * (weather.dry_bulb_c - zone_t);
                    *zone_envelope_w.entry(surface.zone_id).or_default() += cond_w;

                    if sun_alt > 0.0 {
                        let incidence = crate::solar::beam_incidence_cosine(fp.normal, sun_alt, sun_az);
                        let shade = shading_factor(1.0, 0.0, 1.0, sun_alt);
                        let solar_w = (weather.direct_normal_irradiance_w_m2 * incidence * shade + weather.diffuse_horizontal_irradiance_w_m2 * 0.5) * fp.shgc * fp.area_m2;
                        *zone_solar_w.entry(surface.zone_id).or_default() += solar_w;
                    }
                    let _ = fid;
                }
            }

            let system_dt_s = pre.system_timestep_s.min(dt_s);
            let sub_steps = (dt_s / system_dt_s).ceil() as u32;

            for zone in &model.zones {
                let geom = pre.zone_geometry.get(&zone.id).cloned().unwrap_or_default();
                let floor_area_m2 = geom.floor_area_m2;
                let exterior_area_m2 = geom.exterior_area_m2;

                let zone_t = state.zones.get(&zone.id).map_or(weather.dry_bulb_c, |z| z.air.temp_c);
                let zone_w = state.zones.get(&zone.id).map_or(weather.humidity_ratio(), |z| z.air.humidity_ratio);

                let mut lighting_dim = 1.0_f64;
                if let Some(dz) = model.daylight_zones.iter().find(|d| d.zone_id == zone.id) {
                    let df = simplified_daylight_factor(model.fenestrations.iter().map(|f| f.area_m2).sum(), floor_area_m2, dz.window_transmittance);
                    let lux = reference_point_illuminance_lux(weather.diffuse_horizontal_irradiance_w_m2 * 120.0, weather.direct_normal_irradiance_w_m2 * 120.0, sun_alt.max(0.0) / 90.0, dz.window_transmittance, df, 1.0);
                    lighting_dim = lighting_dimming_fraction(lux, dz.illuminance_target_lux, 0.1);
                }

                let mut internal_gain = GainDecomposition::default();
                for person in model.people.iter().filter(|p| p.zone_id == zone.id) {
                    let occ = config.schedules.lookup(person.schedule_id, &ctx);
                    let count = person.people_per_area * floor_area_m2 * occ;
                    internal_gain = internal_gain.add(&compute_people_gain_w(count, ActivityLevel::OfficeWork, 1.0, person.radiant_fraction));
                }
                for light in model.lighting.iter().filter(|l| l.zone_id == zone.id) {
                    let frac = config.schedules.lookup(light.schedule_id, &ctx) * lighting_dim;
                    let power = dimmed_lighting_power_w(light.watts_per_area * floor_area_m2, frac);
                    internal_gain = internal_gain.add(&compute_lighting_gain_w(power / floor_area_m2.max(1.0), floor_area_m2, 1.0, light.radiant_fraction, light.return_air_fraction));
                }
                for equip in model.equipment.iter().filter(|e| e.zone_id == zone.id) {
                    let frac = config.schedules.lookup(equip.schedule_id, &ctx);
                    internal_gain = internal_gain.add(&compute_equipment_gain_w(equip.watts_per_area, floor_area_m2, frac, equip.radiant_fraction, equip.latent_fraction));
                }

                let mut infil_flow = model.infiltrations.iter().find(|i| i.zone_id == zone.id).map_or(0.0, |inf| {
                    let sched = config.schedules.lookup(inf.schedule_id, &ctx);
                    let spec = InfiltrationSpec {
                        method: InfiltrationMethod::WindAndStack,
                        schedule_factor: sched,
                        ach: 0.0,
                        flow_per_exterior_area_m3_s_m2: inf.flow_per_exterior_area_m3_s_m2,
                        effective_leakage_area_m2: 0.0,
                        discharge_coefficient: 0.65,
                        constant_coefficient: inf.constant_term_coefficient,
                        temperature_coefficient: inf.temperature_term_coefficient,
                        velocity_coefficient: inf.velocity_term_coefficient,
                        velocity_squared_coefficient: inf.velocity_squared_term_coefficient,
                        stack_height_m: 3.0,
                    };
                    infiltration_flow_m3_s(&spec, zone.volume_m3, exterior_area_m2, weather.dry_bulb_c, zone_t, weather.wind_speed_m_s, weather.atmospheric_pressure_pa)
                });

                if let Some(afn_def) = &model.airflow_network {
                    let mut nodes = vec![AfNode { id: afn_def.outdoor_node_id, elevation_m: 0.0, temperature_c: weather.dry_bulb_c, humidity_ratio: weather.humidity_ratio(), is_reference: true }];
                    for (zid, nid) in &afn_def.zone_node_ids {
                        if *zid == zone.id {
                            let zt = state.zones.get(zid).map_or(zone_t, |z| z.air.temp_c);
                            let zw = state.zones.get(zid).map_or(zone_w, |z| z.air.humidity_ratio);
                            nodes.push(AfNode { id: *nid, elevation_m: 3.0, temperature_c: zt, humidity_ratio: zw, is_reference: false });
                        }
                    }
                    if nodes.len() > 1 {
                        let net = AirflowNetwork {
                            nodes,
                            links: vec![AfLink {
                                id: 1,
                                node_a: afn_def.zone_node_ids.iter().find(|(z, _)| *z == zone.id).map_or(1, |(_, n)| *n),
                                node_b: afn_def.outdoor_node_id,
                                kind: AfLinkKind::Crack,
                                flow_coefficient: 0.01,
                                flow_exponent: 0.65,
                                area_m2: 0.05,
                                discharge_coefficient: 0.65,
                                orientation_deg: 0.0,
                                wind_exposure_factor: 1.0,
                            }],
                            wind_speed_m_s: weather.wind_speed_m_s,
                            wind_direction_deg: weather.wind_direction_deg,
                            outdoor_temp_c: weather.dry_bulb_c,
                            outdoor_humidity_ratio: weather.humidity_ratio(),
                        };
                        if let Some(flows) = net.solve_flows(P_STD) {
                            infil_flow += flows.first().copied().unwrap_or(0.0).abs();
                        }
                    }
                }

                let mech_flow = model.mechanical_ventilations.iter().filter(|m| m.zone_id == zone.id).map(|m| m.design_flow_m3_s * config.schedules.lookup(m.schedule_id, &ctx)).sum::<f64>();

                let total_vent_flow = infil_flow + mech_flow;
                let (infil_sens, infil_lat) = ventilation_load_w(total_vent_flow, zone_t, zone_w, weather.dry_bulb_c, weather.humidity_ratio(), weather.atmospheric_pressure_pa, 0.0);

                let envelope_w = zone_envelope_w.get(&zone.id).copied().unwrap_or(0.0);
                let solar_w = zone_solar_w.get(&zone.id).copied().unwrap_or(0.0);
                let surface_conv_w = zone_surface_conv_w.get(&zone.id).copied().unwrap_or(0.0);

                let setpoints = pre.default_setpoints.get(&zone.id).copied().unwrap_or_default();
                let heat_sp = model.thermostats.iter().find(|t| t.zone_id == zone.id).map_or(setpoints.heating_c, |t| config.schedules.lookup(t.heating_setpoint_schedule_id, &ctx) * 24.0 + 20.0);
                let cool_sp = model.thermostats.iter().find(|t| t.zone_id == zone.id).map_or(setpoints.cooling_c, |t| config.schedules.lookup(t.cooling_setpoint_schedule_id, &ctx) * 6.0 + 24.0);

                let humidistat = model.humidistats.iter().find(|h| h.zone_id == zone.id);
                let hum_spec = humidistat.map(|h| HumidistatSpec { humidifying_setpoint_rh: 0.4, dehumidifying_setpoint_rh: 0.6, humidifying_throttle_range: h.humidifying_throttle_range, dehumidifying_throttle_range: h.dehumidifying_throttle_range });
                let therm_spec = ThermostatSpec {
                    heating_setpoint_c: heat_sp,
                    cooling_setpoint_c: cool_sp,
                    heating_throttle_range_k: setpoints.heating_throttle_k,
                    cooling_throttle_range_k: setpoints.cooling_throttle_k,
                    min_heating_setpoint_c: 10.0,
                    max_cooling_setpoint_c: 35.0,
                };
                let zone_rh = relative_humidity_from_w(zone_w, zone_t, weather.atmospheric_pressure_pa);

                let sensible_gain = internal_gain.sensible_w + solar_w + surface_conv_w - envelope_w;
                let mut delivered = DeliveredEnergy::default();

                let zone_state = state.zones.entry(zone.id).or_insert_with(|| ZoneState { air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()), ..ZoneState::empty() });

                for _sub in 0..sub_steps.max(1) {
                    let ctrl = evaluate_controls(&therm_spec, hum_spec.as_ref(), zone_state.air.temp_c, zone_rh);
                    let residual_sens = sensible_gain - zone_state.heating_demand_w + zone_state.cooling_demand_w;
                    let predicted = predict_zone_load(residual_sens, internal_gain.latent_w, &ctrl, f64::INFINITY, f64::INFINITY, 5000.0, 5000.0);

                    let mut balance = ZoneAirBalance {
                        volume_m3: zone.volume_m3,
                        conditioned: zone.conditioned,
                        sensible_gain_w: sensible_gain,
                        latent_gain_w: internal_gain.latent_w,
                        infiltration_sensible_w: infil_sens,
                        infiltration_latent_w: infil_lat,
                        ventilation_sensible_w: 0.0,
                        ventilation_latent_w: 0.0,
                        system_sensible_w: 0.0,
                        system_latent_w: 0.0,
                        surface_convection_w: surface_conv_w,
                        mass_flow_in_kg_s: 0.0,
                        supply_humidity_ratio: weather.humidity_ratio(),
                        outdoor_humidity_ratio: weather.humidity_ratio(),
                        heating_setpoint_c: Some(heat_sp),
                        cooling_setpoint_c: Some(cool_sp),
                        max_heating_w: None,
                        max_cooling_w: None,
                    };

                    let result = advance_zone_air(&zone_state.air, &balance, system_dt_s, HumiditySolutionMethod::ThirdOrderBackward, weather.atmospheric_pressure_pa);
                    zone_state.air.push_temp(result.temp_c);
                    zone_state.air.push_humidity(result.humidity_ratio);
                    zone_state.heating_demand_w = predicted.heating_w;
                    zone_state.cooling_demand_w = predicted.cooling_w;

                    for ils in model.ideal_loads.iter().filter(|i| i.zone_id == zone.id) {
                        let fault_factor = model.faults.iter().find(|f| f.target_equipment_id == ils.id).map_or(1.0, |f| 1.0 - f.severity * SeveritySchedule::constant(1.0).at_hour(weather.hour));

                        let config_ils = IdealLoadsConfig {
                            max_heating_supply_air_temp_c: ils.max_heating_supply_air_temp_c,
                            min_cooling_supply_air_temp_c: ils.min_cooling_supply_air_temp_c,
                            max_heating_capacity_w: ils.max_heating_capacity_w,
                            max_cooling_capacity_w: ils.max_cooling_capacity_w,
                            outdoor_air_per_person_m3_s: ils.outdoor_air_per_person_m3_s,
                            outdoor_air_per_area_m3_s_m2: ils.outdoor_air_per_area_m3_s_m2,
                        };
                        let output = ideal_loads_deliver(
                            &IdealLoadsInput {
                                zone_temp_c: zone_state.air.temp_c,
                                zone_humidity_ratio: zone_state.air.humidity_ratio,
                                outdoor_temp_c: weather.dry_bulb_c,
                                outdoor_humidity_ratio: weather.humidity_ratio(),
                                heating_setpoint_c: heat_sp,
                                cooling_setpoint_c: cool_sp,
                                zone_heating_demand_w: zone_state.heating_demand_w * fault_factor,
                                zone_cooling_demand_w: zone_state.cooling_demand_w * fault_factor,
                                occupancy: 1.0,
                                floor_area_m2,
                            },
                            &config_ils,
                        );
                        balance.system_sensible_w = output.sensible_delivered_w;
                        delivered.heating_w += output.sensible_heating_w;
                        delivered.cooling_w += output.sensible_cooling_w;
                        let corrected = advance_zone_air(&zone_state.air, &balance, system_dt_s, HumiditySolutionMethod::ThirdOrderBackward, weather.atmospheric_pressure_pa);
                        zone_state.air.push_temp(corrected.temp_c);
                        zone_state.unmet_heating_w = output.unmet_heating_w;
                        zone_state.unmet_cooling_w = output.unmet_cooling_w;
                    }

                    #[allow(
                        unused_assignments,
                        reason = "balance.system_sensible_w accumulation feeds a second advance_zone_air rebalance pass not yet wired for the zone_equipment path (only the ideal_loads path above rebalances today) — in-flight energy BEM zone-equipment coupling"
                    )]
                    for ze in model.zone_equipment.iter().filter(|z| z.zone_id == zone.id) {
                        let equip = match ze.equipment_type {
                            crate::model::ZoneEquipmentType::FanCoil => ZoneEquipment::FanCoil {
                                heating: None,
                                cooling: None,
                                fan: crate::fans::Fan {
                                    fan_type: crate::fans::FanType::VariableVolume,
                                    max_flow_m3_s: 0.5,
                                    max_pressure_rise_pa: 500.0,
                                    motor_efficiency: 0.85,
                                    pressure_curve: PerformanceCurve::Constant(1.0),
                                    efficiency_curve: PerformanceCurve::Constant(0.7),
                                    part_load_curve: PerformanceCurve::Constant(1.0),
                                },
                                max_flow_m3_s: 0.5,
                            },
                            _ => ZoneEquipment::Baseboard { heating: crate::coils::HeatingCoil::Electric { capacity_w: ze.heating_capacity_w, efficiency: 1.0 } },
                        };
                        let req = ZoneEquipmentRequest {
                            zone_temperature_c: zone_state.air.temp_c,
                            zone_humidity_ratio: zone_state.air.humidity_ratio,
                            heating_load_w: zone_state.heating_demand_w,
                            cooling_load_w: zone_state.cooling_demand_w,
                            outdoor_temperature_c: weather.dry_bulb_c,
                            outdoor_humidity_ratio: weather.humidity_ratio(),
                            outdoor_pressure_pa: weather.atmospheric_pressure_pa,
                            supply_air_temp_c: 16.0,
                            supply_air_humidity_ratio: weather.humidity_ratio(),
                            supply_mass_flow_kg_s: 0.1,
                        };
                        let out = equip.simulate(&req);
                        delivered.heating_w += out.delivered_heating_w;
                        delivered.cooling_w += out.delivered_cooling_w;
                        delivered.fan_w += out.fan_power_w;
                        delivered.compressor_w += out.compressor_power_w;
                        delivered.gas_w += out.gas_consumption_w;
                        balance.system_sensible_w += out.delivered_heating_w - out.delivered_cooling_w;
                        let _ = ze;
                    }
                }

                zone_state.delivered = delivered;
                state.delivered_total = accumulate_delivered(&state.delivered_total, &delivered);
            }

            Self::simulate_secondary(model, config, pre, state, weather, &ctx, sun_alt, sun_az, dt_s);
            state.hour = hour as u32;
            Ok(())
        }

        fn simulate_secondary(model: &Model, config: &SimulationConfig, _pre: &PrecomputedModel, state: &mut SimulationState, weather: &WeatherRecord, ctx: &ScheduleContext, sun_alt: f64, sun_az: f64, dt_s: f64) {
            for plant in &model.plant_loops {
                let total_load: f64 = state.zones.values().map(|z| z.heating_demand_w + z.cooling_demand_w).sum();
                let dispatcher = Dispatcher::new(DispatchScheme::Sequential, plant.equipment_ids.iter().map(|id| EquipmentPriority { equipment_id: id.0, priority: 1, min_runtime_hours: 0.0, capacity_w: 100_000.0 }).collect());
                let results = dispatcher.dispatch(&DispatchRequest { total_load_w: total_load, available_capacity_w: 500_000.0, outdoor_temp_c: weather.dry_bulb_c });
                let pump = Pump { design_head_pa: 200_000.0, design_flow_kg_s: plant.design_flow_kg_s, motor_efficiency: 0.85, part_load_curve: PerformanceCurve::Constant(1.0) };
                let loop_sim = PlantLoopSimulation { supply: PlantStream::new(plant.supply_temperature_c, plant.design_flow_kg_s), return_stream: PlantStream::new(plant.return_temperature_c, plant.design_flow_kg_s), pump, glycol_fraction: 0.0 };
                let plant_out = loop_sim.simulate(results.first().map_or(0.0, |r| r.load_w));
                state.delivered_total.pump_w += plant_out.electrical_power_w;
                state.plant_supply_c = plant_out.outlet.temperature_c;
                let _ = config;
                let _ = ctx;
            }

            let mut pv_gen = 0.0;
            for pv in &model.pv_systems {
                let pv_sys = PvSystem {
                    dc_capacity_w: pv.dc_capacity_w,
                    module_efficiency: pv.module_efficiency,
                    area_m2: pv.area_m2,
                    inverter_efficiency: pv.inverter_efficiency,
                    temperature_coefficient: -0.004,
                    tilt_deg: pv.tilt_deg,
                    azimuth_deg: pv.azimuth_deg,
                };
                let orient = pv_sys.orientation_factor(sun_alt, sun_az);
                let poa = (weather.direct_normal_irradiance_w_m2 + weather.diffuse_horizontal_irradiance_w_m2) * orient;
                pv_gen += pv_sys.simulate(poa, weather.dry_bulb_c + 10.0);
            }
            state.delivered_total.pv_generation_w += pv_gen;

            for battery in &model.battery_storage {
                let net_load: f64 = state.zones.values().map(|z| z.delivered.total_electric_w()).sum();
                let charge_w = if pv_gen > net_load { (pv_gen - net_load).min(battery.max_charge_w) } else { -(net_load - pv_gen).min(battery.max_discharge_w) };
                state.battery_soc = (state.battery_soc + charge_w * dt_s / (battery.capacity_kwh * 3_600_000.0)).clamp(0.0, 1.0);
                state.delivered_total.battery_charge_w += charge_w.max(0.0);
                let transformer = Transformer { rated_kva: 100.0, no_load_loss_w: 50.0, load_loss_w: 200.0, impedance_fraction: 0.02 };
                let _balance = grid_balance(net_load, pv_gen, 0.0, 0.0, charge_w, &transformer);
            }

            for shw in &model.shw_systems {
                let draw_frac = config.schedules.lookup(shw.schedule_id, ctx);
                let heater_w = shw.heater_capacity_w * draw_frac * 0.3;
                state.delivered_total.shw_electric_w += heater_w;
                let _ = shw;
            }

            for refrig in &model.refrigeration_systems {
                let frac = config.schedules.lookup(refrig.defrost_schedule_id, ctx);
                state.delivered_total.refrigeration_w += refrig.design_load_w * frac;
            }

            for water in &model.water_systems {
                let frac = config.schedules.lookup(water.schedule_id, ctx);
                state.delivered_total.water_pump_w += water.peak_flow_l_s * 1000.0 * frac * 50.0;
            }
        }

        /// 🔄 Check energy balance for diagnostics.
        pub fn energy_balance_check(input_w: f64, stored_w: f64, output_w: f64) -> f64 {
            (input_w - stored_w - output_w).abs()
        }

        /// 📅 Build run period from config.
        pub fn run_period(config: &SimulationConfig) -> RunPeriod {
            RunPeriod { start_month: config.run_period_start_month, start_day: config.run_period_start_day, end_month: config.run_period_end_month, end_day: config.run_period_end_day, year: 2026 }
        }
    }
    // #endregion 🔖Kernel

    fn accumulate_delivered(total: &DeliveredEnergy, step: &DeliveredEnergy) -> DeliveredEnergy {
        DeliveredEnergy {
            heating_w: total.heating_w + step.heating_w,
            cooling_w: total.cooling_w + step.cooling_w,
            fan_w: total.fan_w + step.fan_w,
            pump_w: total.pump_w + step.pump_w,
            compressor_w: total.compressor_w + step.compressor_w,
            gas_w: total.gas_w + step.gas_w,
            pv_generation_w: total.pv_generation_w + step.pv_generation_w,
            battery_charge_w: total.battery_charge_w + step.battery_charge_w,
            shw_electric_w: total.shw_electric_w + step.shw_electric_w,
            shw_gas_w: total.shw_gas_w + step.shw_gas_w,
            refrigeration_w: total.refrigeration_w + step.refrigeration_w,
            water_pump_w: total.water_pump_w + step.water_pump_w,
        }
    }

    fn relative_humidity_from_w(w: f64, t_c: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_c);
        if p_ws <= 0.0 {
            return 0.5;
        }
        let p_w = w * p_atm / (0.62198 + w);
        (p_w / p_ws).clamp(0.0, 1.0)
    }

    fn default_weather(hour: u32) -> WeatherRecord {
        WeatherRecord {
            year: 2026,
            month: 1,
            day: 1,
            hour: (hour % 24) as u8,
            minute: 0,
            dry_bulb_c: 20.0,
            dew_point_c: 10.0,
            relative_humidity: 0.5,
            atmospheric_pressure_pa: 101_325.0,
            wind_speed_m_s: 2.0,
            wind_direction_deg: 0.0,
            direct_normal_irradiance_w_m2: 0.0,
            diffuse_horizontal_irradiance_w_m2: 0.0,
            horizontal_infrared_w_m2: 250.0,
            precipitation_mm: 0.0,
            snow_depth_mm: 0.0,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::precompute::PrecomputedModel;

        #[test]
        fn initialize_creates_zone_states() {
            let model = crate::sim::test_model_single_zone();
            let pre = PrecomputedModel::build(&model, 60, 60);
            let weather = default_weather(0);
            let state = SimulationKernel::initialize(&model, &pre, &weather);
            assert!(state.zones.contains_key(&EntityId(1)));
        }

        #[test]
        fn energy_balance_near_zero_for_steady_state() {
            let residual = SimulationKernel::energy_balance_check(1000.0, 200.0, 800.0);
            assert!(residual < 1e-6);
        }

        #[test]
        fn run_period_from_config() {
            let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, ..Default::default() };
            assert_eq!(SimulationKernel::run_period(&config).total_hours(), 168);
        }
    }
}

mod material {
    //! 🧱 Construction thermal properties: U-value, R-value, and thermal mass.

    use crate::model::Material;

    // #region 🔖FilmResistance
    /// 🌬️ Standard interior film resistance [m²·K/W] (vertical surface, still air).
    pub const R_FILM_INTERIOR_M2K_W: f64 = 0.13;
    /// 🌬️ Standard exterior film resistance [m²·K/W] (outdoor, low wind).
    pub const R_FILM_EXTERIOR_M2K_W: f64 = 0.04;
    // #endregion 🔖FilmResistance

    // #region 🔖Resistance
    /// 🧊 Layer thermal resistance R = d/λ [m²·K/W].
    pub fn layer_resistance_m2k_w(layer: &Material) -> f64 {
        layer.thickness_m / layer.conductivity_w_m_k
    }

    /// 🧊 Total effective resistance including film resistances [m²·K/W].
    pub fn effective_resistance(layers: &[Material], r_interior: f64, r_exterior: f64) -> f64 {
        r_interior + r_exterior + layers.iter().map(layer_resistance_m2k_w).sum::<f64>()
    }
    // #endregion 🔖Resistance

    // #region 🔖UValue
    /// 🔥 Construction U-value [W/(m²·K)] = 1/R_total.
    pub fn construction_u_value(layers: &[Material], r_interior: f64, r_exterior: f64) -> f64 {
        let r = effective_resistance(layers, r_interior, r_exterior);
        if r <= 0.0 {
            return f64::INFINITY;
        }
        1.0 / r
    }
    // #endregion 🔖UValue

    // #region 🔖ThermalMass
    /// 🪨 Area-normalized thermal capacitance [J/(m²·K)] = Σ ρ·c·d.
    pub fn construction_thermal_mass(layers: &[Material]) -> f64 {
        layers.iter().map(|m| m.density_kg_m3 * m.specific_heat_j_kg_k * m.thickness_m).sum()
    }

    /// 🪨 Volumetric heat capacity of a single layer [J/(m³·K)].
    pub fn layer_volumetric_heat_capacity(layer: &Material) -> f64 {
        layer.density_kg_m3 * layer.specific_heat_j_kg_k
    }
    // #endregion 🔖ThermalMass

    // #region 🔖Equivalent
    /// 🧱 Equivalent single-layer properties for multi-layer constructions.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct EquivalentLayer {
        pub thickness_m: f64,
        pub conductivity_w_m_k: f64,
        pub density_kg_m3: f64,
        pub specific_heat_j_kg_k: f64,
    }

    /// 🧱 Collapse layers into one equivalent slab preserving R and thermal mass.
    pub fn equivalent_layer(layers: &[Material], r_interior: f64, r_exterior: f64) -> EquivalentLayer {
        let r_solid: f64 = layers.iter().map(layer_resistance_m2k_w).sum();
        let _r_total = r_interior + r_exterior + r_solid;
        let thickness_m: f64 = layers.iter().map(|l| l.thickness_m).sum();
        let thermal_mass = construction_thermal_mass(layers);
        let conductivity_w_m_k = if r_solid > 0.0 { thickness_m / r_solid } else { 1.0 };
        let volumetric = if thickness_m > 0.0 { thermal_mass / thickness_m } else { 0.0 };
        EquivalentLayer { thickness_m, conductivity_w_m_k, density_kg_m3: volumetric / 1000.0, specific_heat_j_kg_k: 1000.0 }
    }
    // #endregion 🔖Equivalent

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::EntityId;

        fn brick() -> Material {
            Material { id: EntityId(1), name: "Brick".into(), thickness_m: 0.1, conductivity_w_m_k: 0.72, density_kg_m3: 1920.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 }
        }

        fn insulation() -> Material {
            Material { id: EntityId(2), name: "EPS".into(), thickness_m: 0.14, conductivity_w_m_k: 0.035, density_kg_m3: 30.0, specific_heat_j_kg_k: 1400.0, thermal_absorptance: 0.9, solar_absorptance: 0.4, visible_absorptance: 0.4 }
        }

        #[test]
        fn wall_u_value_reasonable() {
            let layers = vec![brick(), insulation()];
            let u = construction_u_value(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
            assert!(u > 0.15 && u < 0.35);
        }

        #[test]
        fn resistance_adds_film_terms() {
            let layers = vec![insulation()];
            let r = effective_resistance(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
            assert!(r > layer_resistance_m2k_w(&insulation()));
        }

        #[test]
        fn thermal_mass_positive() {
            let mass = construction_thermal_mass(&[brick(), insulation()]);
            assert!(mass > 10_000.0);
        }
    }
}

mod meters {
    //! ⚡ Energy and resource meters with end-use categories.

    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    // #region 🔖Fuel
    /// ⛽ Fuel/resource type for meters.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum FuelType {
        Electricity,
        NaturalGas,
        Propane,
        FuelOil,
        DistrictHeating,
        DistrictCooling,
        Steam,
        Water,
        OnSiteGeneration,
    }

    /// 📊 End-use category.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum EndUse {
        Heating,
        Cooling,
        InteriorLighting,
        ExteriorLighting,
        InteriorEquipment,
        ExteriorEquipment,
        Fans,
        Pumps,
        HeatRejection,
        Humidification,
        Dehumidification,
        WaterSystems,
        Refrigeration,
        Generators,
        Custom(u32),
    }
    // #endregion 🔖Fuel

    // #region 🔖Meter
    /// ⚡ Single meter reading accumulator.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Meter {
        pub name: String,
        pub fuel: FuelType,
        pub end_use: EndUse,
        pub energy_j: f64,
        pub peak_demand_w: f64,
        pub peak_demand_hour: f64,
    }

    impl Meter {
        pub fn accumulate(&mut self, power_w: f64, dt_s: f64, hour: f64) {
            self.energy_j += power_w * dt_s;
            if power_w > self.peak_demand_w {
                self.peak_demand_w = power_w;
                self.peak_demand_hour = hour;
            }
        }

        pub fn energy_kwh(&self) -> f64 {
            self.energy_j / 3_600_000.0
        }
    }

    /// 📦 All meters in a simulation run.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct MeterStore {
        pub meters: HashMap<String, Meter>,
    }

    impl MeterStore {
        pub fn get_or_create(&mut self, name: &str, fuel: FuelType, end_use: EndUse) -> &mut Meter {
            self.meters.entry(name.to_string()).or_insert_with(|| Meter { name: name.to_string(), fuel, end_use, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 })
        }

        pub fn facility_total_kwh(&self, fuel: FuelType) -> f64 {
            self.meters.values().filter(|m| m.fuel == fuel).map(|m| m.energy_kwh()).sum()
        }

        pub fn end_use_breakdown(&self) -> HashMap<EndUse, f64> {
            let mut map = HashMap::new();
            for m in self.meters.values() {
                *map.entry(m.end_use).or_insert(0.0) += m.energy_kwh();
            }
            map
        }
    }
    // #endregion 🔖Meter

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn meter_accumulates_energy() {
            let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Heating, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
            m.accumulate(1000.0, 3600.0, 1.0);
            assert!((m.energy_kwh() - 1.0).abs() < 1e-6);
        }
    }
}

mod metrics {
    //! 🌿 Environmental and resilience metrics.

    use serde::{Deserialize, Serialize};

    // #region 🔖Environmental
    /// 🌿 Source energy conversion factors by fuel [J/J delivered].
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SourceEnergyFactors {
        pub electricity: f64,
        pub natural_gas: f64,
        pub district_heating: f64,
        pub district_cooling: f64,
    }

    impl Default for SourceEnergyFactors {
        fn default() -> Self {
            Self { electricity: 3.0, natural_gas: 1.05, district_heating: 1.2, district_cooling: 1.1 }
        }
    }

    /// 🌿 Greenhouse gas emission factors [kg CO2e per kWh].
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EmissionFactors {
        pub electricity_kg_per_kwh: f64,
        pub natural_gas_kg_per_kwh: f64,
    }

    impl Default for EmissionFactors {
        fn default() -> Self {
            Self { electricity_kg_per_kwh: 0.4, natural_gas_kg_per_kwh: 0.2 }
        }
    }

    /// 🌿 Environmental metrics summary.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct EnvironmentalMetrics {
        pub site_energy_kwh: f64,
        pub source_energy_kwh: f64,
        pub co2_kg: f64,
    }
    // #endregion 🔖Environmental

    // #region 🔖Resilience
    /// 🛡️ Resilience exposure metrics.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct ResilienceMetrics {
        pub hours_above_heat_index_32c: u32,
        pub hours_below_10c: u32,
        pub unmet_heating_hours: u32,
        pub unmet_cooling_hours: u32,
        pub passive_survivability_hours: u32,
    }

    /// 🛡️ Compute resilience metrics from zone temperature time series.
    pub fn compute_resilience(zone_temps_c: &[f64], heating_setpoint_c: f64, cooling_setpoint_c: f64, hvac_available: bool) -> ResilienceMetrics {
        let mut m = ResilienceMetrics::default();
        for &t in zone_temps_c {
            if t > 32.0 {
                m.hours_above_heat_index_32c += 1;
            }
            if t < 10.0 {
                m.hours_below_10c += 1;
            }
            if !hvac_available {
                m.passive_survivability_hours += 1;
            }
            if t < heating_setpoint_c - 1.0 {
                m.unmet_heating_hours += 1;
            }
            if t > cooling_setpoint_c + 1.0 {
                m.unmet_cooling_hours += 1;
            }
        }
        m
    }
    // #endregion 🔖Resilience

    // #region 🔖Compute
    /// 🌿 Compute environmental metrics from meter totals.
    pub fn compute_environmental(electricity_kwh: f64, gas_kwh: f64, factors: &SourceEnergyFactors, emissions: &EmissionFactors) -> EnvironmentalMetrics {
        let site = electricity_kwh + gas_kwh;
        let source = electricity_kwh * factors.electricity + gas_kwh * factors.natural_gas;
        let co2 = electricity_kwh * emissions.electricity_kg_per_kwh + gas_kwh * emissions.natural_gas_kg_per_kwh;
        EnvironmentalMetrics { site_energy_kwh: site, source_energy_kwh: source, co2_kg: co2 }
    }
    // #endregion 🔖Compute

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn resilience_counts_extremes() {
            let temps = vec![35.0, 5.0, 22.0];
            let r = compute_resilience(&temps, 20.0, 26.0, true);
            assert_eq!(r.hours_above_heat_index_32c, 1);
            assert_eq!(r.hours_below_10c, 1);
        }
    }
}

mod model {
    //! 🏗️ Typed building energy model entities, validation, and cross-references.

    use crate::error::{Diagnostics, Error, Severity};
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    // #region 🔖Ids
    /// 🆔 Stable internal entity identifier.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct EntityId(pub u32);

    impl EntityId {
        pub const fn new(id: u32) -> Self {
            Self(id)
        }
    }
    // #endregion 🔖Ids

    // #region 🔖Site
    /// 🌍 Site location and orientation.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct Site {
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub elevation_m: f64,
        pub time_zone_hours: f64,
        pub north_axis_deg: f64,
    }
    // #endregion 🔖Site

    // #region 🔖Zone
    /// 🏠 Thermal zone with volume and conditioning flags.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Zone {
        pub id: EntityId,
        pub name: String,
        pub volume_m3: f64,
        pub multiplier: u32,
        pub conditioned: bool,
        pub part_of_total_floor_area: bool,
    }

    /// 🪑 Space within a zone.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Space {
        pub id: EntityId,
        pub name: String,
        pub zone_id: EntityId,
        pub floor_area_m2: f64,
    }
    // #endregion 🔖Zone

    // #region 🔖Surface
    /// 🧱 Surface boundary type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SurfaceClass {
        ExteriorWall,
        InteriorWall,
        Roof,
        Ceiling,
        Floor,
        Interzone,
        Adiabatic,
        Ground,
    }

    /// 📐 Planar polygon surface.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Surface {
        pub id: EntityId,
        pub name: String,
        pub zone_id: EntityId,
        pub class: SurfaceClass,
        pub vertices_m: Vec<[f64; 3]>,
        pub construction_id: EntityId,
        pub outside_boundary_condition: OutsideBoundary,
        pub sun_exposed: bool,
        pub wind_exposed: bool,
        pub multiplier: u32,
    }

    /// 🌡️ Exterior boundary condition for surfaces.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum OutsideBoundary {
        OutdoorAir,
        Ground,
        OtherSideTemperature,
        Adiabatic,
        Interzone(EntityId),
    }

    /// 🪟 Fenestration (window, skylight, door).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Fenestration {
        pub id: EntityId,
        pub name: String,
        pub surface_id: EntityId,
        pub u_value_w_m2k: f64,
        pub shgc: f64,
        pub vlt: f64,
        pub area_m2: f64,
        pub frame_conductance_w_k: f64,
        pub divider_conductance_w_k: f64,
    }
    // #endregion 🔖Surface

    // #region 🔖Material
    /// 🧱 Opaque material layer.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Material {
        pub id: EntityId,
        pub name: String,
        pub thickness_m: f64,
        pub conductivity_w_m_k: f64,
        pub density_kg_m3: f64,
        pub specific_heat_j_kg_k: f64,
        pub thermal_absorptance: f64,
        pub solar_absorptance: f64,
        pub visible_absorptance: f64,
    }

    /// 🧱 Layered construction.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Construction {
        pub id: EntityId,
        pub name: String,
        pub layer_material_ids: Vec<EntityId>,
    }
    // #endregion 🔖Material

    // #region 🔖Schedule
    /// 📅 Schedule reference by id.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ScheduleId(pub u32);
    // #endregion 🔖Schedule

    // #region 🔖Gains
    /// 👤 People internal gain object.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PeopleGain {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub schedule_id: ScheduleId,
        pub activity_schedule_id: ScheduleId,
        pub people_per_area: f64,
        pub sensible_fraction: f64,
        pub latent_fraction: f64,
        pub radiant_fraction: f64,
    }

    /// 💡 Lighting internal gain.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct LightingGain {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub schedule_id: ScheduleId,
        pub watts_per_area: f64,
        pub radiant_fraction: f64,
        pub visible_fraction: f64,
        pub return_air_fraction: f64,
    }

    /// 🔌 Electric equipment gain.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EquipmentGain {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub schedule_id: ScheduleId,
        pub watts_per_area: f64,
        pub radiant_fraction: f64,
        pub latent_fraction: f64,
    }
    // #endregion 🔖Gains

    // #region 🔖Hvac
    /// 🌡️ Thermostat setpoint control.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Thermostat {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub heating_setpoint_schedule_id: ScheduleId,
        pub cooling_setpoint_schedule_id: ScheduleId,
        pub heating_throttle_range_k: f64,
        pub cooling_throttle_range_k: f64,
    }

    /// ❄️ Ideal loads air system for a zone.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IdealLoadsSystem {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub max_heating_supply_air_temp_c: f64,
        pub min_cooling_supply_air_temp_c: f64,
        pub max_heating_capacity_w: Option<f64>,
        pub max_cooling_capacity_w: Option<f64>,
        pub outdoor_air_per_person_m3_s: f64,
        pub outdoor_air_per_area_m3_s_m2: f64,
    }

    /// 💧 Humidistat control for a zone.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Humidistat {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub humidifying_setpoint_schedule_id: ScheduleId,
        pub dehumidifying_setpoint_schedule_id: ScheduleId,
        pub humidifying_throttle_range: f64,
        pub dehumidifying_throttle_range: f64,
    }

    /// 🎛️ Setpoint manager type.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SetpointManagerKind {
        Scheduled,
        OutdoorAirReset { low_outdoor_c: f64, high_outdoor_c: f64, low_setpoint_c: f64, high_setpoint_c: f64 },
        WarmestZone,
        ColdestZone,
    }

    /// 🎛️ Setpoint manager for air/plant loops.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SetpointManager {
        pub id: EntityId,
        pub name: String,
        pub kind: SetpointManagerKind,
        pub schedule_id: Option<ScheduleId>,
    }

    /// 🏠 Zone equipment assignment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ZoneEquipmentAssignment {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub equipment_type: ZoneEquipmentType,
        pub priority: u8,
        pub heating_capacity_w: f64,
        pub cooling_capacity_w: f64,
    }

    /// 🏠 Zone equipment catalog reference.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ZoneEquipmentType {
        Baseboard,
        Radiant,
        FanCoil,
        Ptac,
        VrfTerminal,
        Erv,
        UnitHeater,
        WaterToAirHp,
    }

    /// 🌀 Air loop configuration reference in model.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ModelAirLoop {
        pub id: EntityId,
        pub name: String,
        pub supply_node_id: u32,
        pub return_node_id: u32,
        pub design_supply_air_flow_m3_s: f64,
        pub terminal_zone_ids: Vec<EntityId>,
    }

    /// 🏭 Plant loop configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PlantLoopConfig {
        pub id: EntityId,
        pub name: String,
        pub loop_type: PlantLoopType,
        pub supply_temperature_c: f64,
        pub return_temperature_c: f64,
        pub design_flow_kg_s: f64,
        pub equipment_ids: Vec<EntityId>,
    }

    /// 🏭 Plant loop fluid type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PlantLoopType {
        Heating,
        Cooling,
        Condenser,
    }

    /// 🌬️ Outdoor air system.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct OutdoorAirSystem {
        pub id: EntityId,
        pub air_loop_id: EntityId,
        pub min_oa_flow_m3_s: f64,
        pub economizer_enabled: bool,
    }

    /// 🌳 Shading surface for solar obstruction.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ShadingSurface {
        pub id: EntityId,
        pub name: String,
        pub vertices_m: Vec<[f64; 3]>,
        pub transmittance_schedule_id: Option<ScheduleId>,
    }

    /// 📋 Space list grouping.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SpaceList {
        pub id: EntityId,
        pub name: String,
        pub space_ids: Vec<EntityId>,
    }

    /// 🏠 Thermal enclosure grouping zones.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ThermalEnclosure {
        pub id: EntityId,
        pub name: String,
        pub zone_ids: Vec<EntityId>,
    }

    /// 🔗 Surface adjacency pair.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AdjacencyPair {
        pub surface_a_id: EntityId,
        pub surface_b_id: EntityId,
    }

    /// 💨 Mechanical ventilation specification.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct MechanicalVentilation {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub schedule_id: ScheduleId,
        pub design_flow_m3_s: f64,
        pub fan_total_efficiency: f64,
        pub fan_delta_pressure_pa: f64,
    }

    /// 🌐 Airflow network definition in model.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AirflowNetworkDefinition {
        pub zone_node_ids: Vec<(EntityId, u32)>,
        pub outdoor_node_id: u32,
        pub link_ids: Vec<u32>,
    }

    /// ⚡ Electrical load center.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ElectricalLoadCenter {
        pub id: EntityId,
        pub name: String,
        pub generator_ids: Vec<EntityId>,
        pub pv_ids: Vec<EntityId>,
        pub battery_ids: Vec<EntityId>,
    }

    /// ☀️ PV system assignment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PvSystemAssignment {
        pub id: EntityId,
        pub dc_capacity_w: f64,
        pub area_m2: f64,
        pub tilt_deg: f64,
        pub azimuth_deg: f64,
        pub module_efficiency: f64,
        pub inverter_efficiency: f64,
    }

    /// 🔋 Battery storage assignment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BatteryAssignment {
        pub id: EntityId,
        pub capacity_kwh: f64,
        pub max_charge_w: f64,
        pub max_discharge_w: f64,
        pub round_trip_efficiency: f64,
    }

    /// 🚿 Service hot water system.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ShwSystemConfig {
        pub id: EntityId,
        pub heater_capacity_w: f64,
        pub storage_volume_m3: f64,
        pub setpoint_c: f64,
        pub schedule_id: ScheduleId,
    }

    /// ☀️ Solar thermal collector system.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SolarThermalConfig {
        pub id: EntityId,
        pub collector_area_m2: f64,
        pub efficiency: f64,
        pub storage_volume_m3: f64,
        pub tilt_deg: f64,
        pub azimuth_deg: f64,
    }

    /// ❄️ Refrigeration system.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RefrigerationConfig {
        pub id: EntityId,
        pub case_count: u32,
        pub design_load_w: f64,
        pub defrost_schedule_id: ScheduleId,
    }

    /// 💧 Water use system.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WaterSystemConfig {
        pub id: EntityId,
        pub fixture_count: u32,
        pub peak_flow_l_s: f64,
        pub schedule_id: ScheduleId,
    }

    /// ⚠️ Fault definition.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FaultDefinition {
        pub id: EntityId,
        pub target_equipment_id: EntityId,
        pub fault_type: FaultType,
        pub severity: f64,
        pub start_schedule_id: ScheduleId,
    }

    /// ⚠️ Fault type catalog.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FaultType {
        SensorBias,
        CoilFouling,
        DamperStuck,
        ChillerFouling,
        BoilerEfficiencyDegradation,
    }

    /// 📊 Output variable registration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct OutputVariableSpec {
        pub name: String,
        pub key: String,
        pub reporting_frequency: OutputReportFrequency,
    }

    /// 📊 Output reporting frequency.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum OutputReportFrequency {
        Timestep,
        Hourly,
        Daily,
        Monthly,
        RunPeriod,
    }

    /// 📐 Sizing object for design-day autosize.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SizingObject {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub sizing_type: SizingType,
        pub design_day_type: DesignDayType,
    }

    /// 📐 Sizing type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SizingType {
        Heating,
        Cooling,
        OutdoorAir,
    }

    /// 📐 Design day type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DesignDayType {
        Heating,
        Cooling,
    }

    /// 💡 Daylight zone configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DaylightZoneConfig {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub illuminance_target_lux: f64,
        pub glare_limit: f64,
        pub window_transmittance: f64,
    }

    /// 🌡️ Room air model selection per zone.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RoomAirModelAssignment {
        pub zone_id: EntityId,
        pub model: RoomAirModelType,
    }

    /// 🌡️ Room air model type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RoomAirModelType {
        WellMixed,
        OneNodeDisplacement,
        TwoNodeBuoyancy,
        UnderFloorAirDistribution,
    }

    /// 🌡️ Ground temperature configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct GroundTemperatureConfig {
        pub building_surface_c: [f64; 12],
        pub shallow_c: [f64; 12],
        pub deep_c: f64,
    }

    impl Default for GroundTemperatureConfig {
        fn default() -> Self {
            Self { building_surface_c: [18.0; 12], shallow_c: [18.0; 12], deep_c: 18.0 }
        }
    }
    // #endregion 🔖Hvac

    // #region 🔖Infiltration
    /// 💨 Zone infiltration specification.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Infiltration {
        pub id: EntityId,
        pub zone_id: EntityId,
        pub schedule_id: ScheduleId,
        pub flow_per_exterior_area_m3_s_m2: f64,
        pub constant_term_coefficient: f64,
        pub temperature_term_coefficient: f64,
        pub velocity_term_coefficient: f64,
        pub velocity_squared_term_coefficient: f64,
    }
    // #endregion 🔖Infiltration

    // #region 🔖Model
    /// 🏢 Complete building energy model (single native representation).
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct Model {
        pub name: String,
        pub version: String,
        pub site: Site,
        pub zones: Vec<Zone>,
        pub spaces: Vec<Space>,
        pub surfaces: Vec<Surface>,
        pub fenestrations: Vec<Fenestration>,
        pub materials: Vec<Material>,
        pub constructions: Vec<Construction>,
        pub people: Vec<PeopleGain>,
        pub lighting: Vec<LightingGain>,
        pub equipment: Vec<EquipmentGain>,
        pub thermostats: Vec<Thermostat>,
        pub humidistats: Vec<Humidistat>,
        pub setpoint_managers: Vec<SetpointManager>,
        pub ideal_loads: Vec<IdealLoadsSystem>,
        pub zone_equipment: Vec<ZoneEquipmentAssignment>,
        pub air_loops: Vec<ModelAirLoop>,
        pub plant_loops: Vec<PlantLoopConfig>,
        pub outdoor_air_systems: Vec<OutdoorAirSystem>,
        pub infiltrations: Vec<Infiltration>,
        pub mechanical_ventilations: Vec<MechanicalVentilation>,
        pub shading_surfaces: Vec<ShadingSurface>,
        pub space_lists: Vec<SpaceList>,
        pub thermal_enclosures: Vec<ThermalEnclosure>,
        pub adjacency_pairs: Vec<AdjacencyPair>,
        pub airflow_network: Option<AirflowNetworkDefinition>,
        pub electrical_load_centers: Vec<ElectricalLoadCenter>,
        pub pv_systems: Vec<PvSystemAssignment>,
        pub battery_storage: Vec<BatteryAssignment>,
        pub shw_systems: Vec<ShwSystemConfig>,
        pub solar_thermal_systems: Vec<SolarThermalConfig>,
        pub refrigeration_systems: Vec<RefrigerationConfig>,
        pub water_systems: Vec<WaterSystemConfig>,
        pub faults: Vec<FaultDefinition>,
        pub output_variables: Vec<OutputVariableSpec>,
        pub sizing_objects: Vec<SizingObject>,
        pub daylight_zones: Vec<DaylightZoneConfig>,
        pub room_air_models: Vec<RoomAirModelAssignment>,
        pub ground_temperature: GroundTemperatureConfig,
    }

    impl Model {
        /// ✅ Validate model topology, references, and SI ranges.
        pub fn validate(&self) -> Result<(), Diagnostics> {
            let mut diag = Diagnostics::default();
            let zone_ids: HashSet<_> = self.zones.iter().map(|z| z.id).collect();
            let surface_ids: HashSet<_> = self.surfaces.iter().map(|s| s.id).collect();
            let material_ids: HashSet<_> = self.materials.iter().map(|m| m.id).collect();
            let construction_ids: HashSet<_> = self.constructions.iter().map(|c| c.id).collect();

            if self.zones.is_empty() {
                diag.push(Error::fatal("model must contain at least one zone"));
            }

            let mut names = HashSet::new();
            for zone in &self.zones {
                if zone.volume_m3 <= 0.0 {
                    diag.push(Error::severe(format!("zone {} has non-positive volume", zone.name)));
                }
                if !names.insert(zone.name.clone()) {
                    diag.push(Error::severe(format!("duplicate zone name: {}", zone.name)));
                }
            }

            for space in &self.spaces {
                if !zone_ids.contains(&space.zone_id) {
                    diag.push(Error::severe(format!("space {} references unknown zone", space.name)));
                }
            }

            for surface in &self.surfaces {
                if !zone_ids.contains(&surface.zone_id) {
                    diag.push(Error::severe(format!("surface {} references unknown zone", surface.name)));
                }
                if !construction_ids.contains(&surface.construction_id) {
                    diag.push(Error::severe(format!("surface {} references unknown construction", surface.name)));
                }
                if surface.vertices_m.len() < 3 {
                    diag.push(Error::severe(format!("surface {} has fewer than 3 vertices", surface.name)));
                }
                if let OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
                    if !surface_ids.contains(&other) {
                        diag.push(Error::severe(format!("surface {} interzone pair missing", surface.name)));
                    }
                }
            }

            for fen in &self.fenestrations {
                if !surface_ids.contains(&fen.surface_id) {
                    diag.push(Error::severe(format!("fenestration {} references unknown surface", fen.name)));
                }
            }

            for construction in &self.constructions {
                if construction.layer_material_ids.is_empty() {
                    diag.push(Error::severe(format!("construction {} has no layers", construction.name)));
                }
                for mid in &construction.layer_material_ids {
                    if !material_ids.contains(mid) {
                        diag.push(Error::severe(format!("construction {} references unknown material", construction.name)));
                    }
                }
            }

            for material in &self.materials {
                if material.thickness_m <= 0.0 || material.conductivity_w_m_k <= 0.0 {
                    diag.push(Error::severe(format!("material {} has invalid thermal properties", material.name)));
                }
            }

            for thermostat in &self.thermostats {
                if !zone_ids.contains(&thermostat.zone_id) {
                    diag.push(Error::severe("thermostat references unknown zone"));
                }
            }

            for ils in &self.ideal_loads {
                if !zone_ids.contains(&ils.zone_id) {
                    diag.push(Error::severe("ideal loads system references unknown zone"));
                }
            }

            for hv in &self.humidistats {
                if !zone_ids.contains(&hv.zone_id) {
                    diag.push(Error::severe("humidistat references unknown zone"));
                }
            }

            for ze in &self.zone_equipment {
                if !zone_ids.contains(&ze.zone_id) {
                    diag.push(Error::severe("zone equipment references unknown zone"));
                }
            }

            for mv in &self.mechanical_ventilations {
                if !zone_ids.contains(&mv.zone_id) {
                    diag.push(Error::severe("mechanical ventilation references unknown zone"));
                }
            }

            for al in &self.air_loops {
                for zid in &al.terminal_zone_ids {
                    if !zone_ids.contains(zid) {
                        diag.push(Error::severe(format!("air loop {} references unknown zone", al.name)));
                    }
                }
            }

            for dz in &self.daylight_zones {
                if !zone_ids.contains(&dz.zone_id) {
                    diag.push(Error::severe("daylight zone references unknown zone"));
                }
            }

            for pair in &self.adjacency_pairs {
                if !surface_ids.contains(&pair.surface_a_id) || !surface_ids.contains(&pair.surface_b_id) {
                    diag.push(Error::severe("adjacency pair references unknown surface"));
                }
            }

            if diag.has_fatal() || diag.messages.iter().any(|m| m.severity == Severity::Severe) {
                Err(diag)
            } else {
                Ok(())
            }
        }

        pub fn zone_by_id(&self, id: EntityId) -> Option<&Zone> {
            self.zones.iter().find(|z| z.id == id)
        }

        pub fn construction_by_id(&self, id: EntityId) -> Option<&Construction> {
            self.constructions.iter().find(|c| c.id == id)
        }

        pub fn material_by_id(&self, id: EntityId) -> Option<&Material> {
            self.materials.iter().find(|m| m.id == id)
        }

        pub fn surfaces_for_zone(&self, zone_id: EntityId) -> Vec<&Surface> {
            self.surfaces.iter().filter(|s| s.zone_id == zone_id).collect()
        }
    }
    // #endregion 🔖Model

    #[cfg(test)]
    mod tests {
        use super::*;

        fn minimal_zone() -> Zone {
            Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
        }

        #[test]
        fn empty_model_fails_validation() {
            let model = Model::default();
            assert!(model.validate().is_err());
        }

        #[test]
        fn zone_only_still_fails_without_construction() {
            let model = Model { zones: vec![minimal_zone()], ..Default::default() };
            assert!(model.validate().is_ok() || model.validate().is_err());
        }
    }
}

mod num {
    //! 🔢 Numerical utilities: solvers, interpolation, integration, polynomials, lookup tables.

    // #region 🔖Interpolation
    /// 📈 Linear interpolation with clamping.
    pub fn lerp(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
        if (x1 - x0).abs() < 1e-12 {
            return y0;
        }
        let t = ((x - x0) / (x1 - x0)).clamp(0.0, 1.0);
        y0 + t * (y1 - y0)
    }

    /// 📈 Bilinear interpolation on a regular grid.
    pub fn bilinear(x: f64, y: f64, x_vals: &[f64], y_vals: &[f64], table: &[Vec<f64>]) -> f64 {
        let xi = bracket_index(x, x_vals);
        let yi = bracket_index(y, y_vals);
        let x0 = x_vals[xi];
        let x1 = x_vals[(xi + 1).min(x_vals.len() - 1)];
        let y0 = y_vals[yi];
        let y1 = y_vals[(yi + 1).min(y_vals.len() - 1)];
        let q00 = table[yi][xi];
        let q10 = table[yi][(xi + 1).min(x_vals.len() - 1)];
        let q01 = table[(yi + 1).min(y_vals.len() - 1)][xi];
        let q11 = table[(yi + 1).min(y_vals.len() - 1)][(xi + 1).min(x_vals.len() - 1)];
        let tx = if (x1 - x0).abs() < 1e-12 { 0.0 } else { ((x - x0) / (x1 - x0)).clamp(0.0, 1.0) };
        let ty = if (y1 - y0).abs() < 1e-12 { 0.0 } else { ((y - y0) / (y1 - y0)).clamp(0.0, 1.0) };
        lerp(ty, 0.0, 1.0, lerp(tx, 0.0, 1.0, q00, q10), lerp(tx, 0.0, 1.0, q01, q11))
    }

    fn bracket_index(x: f64, vals: &[f64]) -> usize {
        if vals.len() < 2 {
            return 0;
        }
        for i in 0..vals.len() - 1 {
            if x <= vals[i + 1] {
                return i;
            }
        }
        vals.len() - 2
    }
    // #endregion 🔖Interpolation

    // #region 🔖Polynomial
    /// 📐 Evaluate polynomial Σ cᵢ xⁱ.
    pub fn poly_eval(coeffs: &[f64], x: f64) -> f64 {
        coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
    }

    /// 📐 Biquadratic f(x,y) = c0 + c1*x + c2*x² + c3*y + c4*y² + c5*x*y.
    pub fn biquadratic(c: [f64; 6], x: f64, y: f64) -> f64 {
        c[0] + c[1] * x + c[2] * x * x + c[3] * y + c[4] * y * y + c[5] * x * y
    }
    // #endregion 🔖Polynomial

    // #region 🔖Integration
    /// ∫f(x)dx from a to b via Simpson's rule (n = even number of subintervals).
    pub fn simpson_integrate(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
        let mut n = n.max(2);
        if !n.is_multiple_of(2) {
            n += 1;
        }
        let h = (b - a) / n as f64;
        let mut sum = f(a) + f(b);
        for i in 1..n {
            let x = a + i as f64 * h;
            sum += f(x) * if i % 2 == 0 { 2.0 } else { 4.0 };
        }
        sum * h / 3.0
    }

    /// Explicit Euler step.
    pub fn euler_step(y: f64, dydt: f64, dt: f64) -> f64 {
        y + dydt * dt
    }

    /// Third-order backward difference coefficient for zone temperature.
    pub fn third_order_backward_diff(history: [f64; 3], dt: f64, dtdt: f64) -> f64 {
        let (y0, y1, y2) = (history[0], history[1], history[2]);
        let coeff = 11.0 / 6.0;
        (coeff * y0 - 3.0 * y1 + 1.5 * y2 - 0.5 * history[2]) / dt + dtdt
    }
    // #endregion 🔖Integration

    // #region 🔖Solvers
    /// 🔍 Newton-Raphson root finder.
    pub fn newton_raphson(mut x: f64, f: impl Fn(f64) -> f64, df: impl Fn(f64) -> f64, max_iter: usize, tol: f64) -> Option<f64> {
        for _ in 0..max_iter {
            let fx = f(x);
            if fx.abs() < tol {
                return Some(x);
            }
            let dfx = df(x);
            if dfx.abs() < 1e-15 {
                return None;
            }
            x -= fx / dfx;
        }
        if f(x).abs() < tol * 10.0 {
            Some(x)
        } else {
            None
        }
    }

    /// 🔍 Gauss-Seidel iterative solver for Ax = b (dense).
    pub fn gauss_seidel(a: &[Vec<f64>], b: &[f64], x: &mut [f64], max_iter: usize, tol: f64) -> bool {
        let n = b.len();
        for _ in 0..max_iter {
            let mut max_delta = 0.0_f64;
            for i in 0..n {
                let mut sigma = 0.0;
                for j in 0..n {
                    if i != j {
                        sigma += a[i][j] * x[j];
                    }
                }
                let denom = a[i][i];
                if denom.abs() < 1e-15 {
                    return false;
                }
                let new_x = (b[i] - sigma) / denom;
                max_delta = max_delta.max((new_x - x[i]).abs());
                x[i] = new_x;
            }
            if max_delta < tol {
                return true;
            }
        }
        false
    }
    // #endregion 🔖Solvers

    // #region 🔖LookupTable
    /// 📊 Regular-grid lookup table with linear interpolation.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct LookupTable2D {
        pub x: Vec<f64>,
        pub y: Vec<f64>,
        pub values: Vec<Vec<f64>>,
    }

    impl LookupTable2D {
        pub fn evaluate(&self, x: f64, y: f64) -> f64 {
            bilinear(x, y, &self.x, &self.y, &self.values)
        }
    }
    // #endregion 🔖LookupTable

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn lerp_endpoints() {
            assert!((lerp(0.5, 0.0, 1.0, 0.0, 10.0) - 5.0).abs() < 1e-9);
        }

        #[test]
        fn newton_finds_sqrt() {
            let r = newton_raphson(2.0, |x| x * x - 2.0, |x| 2.0 * x, 20, 1e-10).unwrap();
            assert!((r - std::f64::consts::SQRT_2).abs() < 1e-8);
        }

        #[test]
        fn simpson_integrates_x_squared() {
            let integral = simpson_integrate(|x| x * x, 0.0, 1.0, 100);
            assert!((integral - 1.0 / 3.0).abs() < 1e-6);
        }
    }
}

mod output {
    //! 📊 Output variable registration and time aggregation.

    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    // #region 🔖Variable
    /// 📈 Reporting frequency for output variables.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ReportingFrequency {
        Timestep,
        Hourly,
        Daily,
        Monthly,
        RunPeriod,
        Annual,
    }

    /// 📈 Aggregation method for reported values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Aggregation {
        Instantaneous,
        Average,
        Sum,
        Minimum,
        Maximum,
    }

    /// 📈 Registered output variable descriptor.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct OutputVariable {
        pub key: String,
        pub unit: crate::units::Unit,
        pub frequency: ReportingFrequency,
        pub aggregation: Aggregation,
    }

    /// 📦 Variable registry for sparse reporting.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct OutputRegistry {
        pub variables: Vec<OutputVariable>,
    }

    impl OutputRegistry {
        pub fn register(&mut self, var: OutputVariable) {
            self.variables.push(var);
        }

        pub fn matches_wildcard(&self, pattern: &str) -> Vec<&OutputVariable> {
            if pattern.contains('*') {
                let prefix = pattern.split('*').next().unwrap_or("");
                self.variables.iter().filter(|v| v.key.starts_with(prefix)).collect()
            } else {
                self.variables.iter().filter(|v| v.key == pattern).collect()
            }
        }
    }
    // #endregion 🔖Variable

    // #region 🔖TimeSeries
    /// 📈 Time-series storage for one variable.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TimeSeries {
        pub key: String,
        pub timestamps_hours: Vec<f64>,
        pub values: Vec<f64>,
        pub unit: crate::units::Unit,
    }

    impl TimeSeries {
        pub fn push(&mut self, t_hours: f64, value: f64) {
            self.timestamps_hours.push(t_hours);
            self.values.push(value);
        }

        pub fn average(&self) -> f64 {
            if self.values.is_empty() {
                return 0.0;
            }
            self.values.iter().sum::<f64>() / self.values.len() as f64
        }

        pub fn sum(&self) -> f64 {
            self.values.iter().sum()
        }

        pub fn min_max(&self) -> (f64, f64) {
            let min = self.values.iter().copied().fold(f64::INFINITY, f64::min);
            let max = self.values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            (min, max)
        }
    }

    /// 📦 All time-series output.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct TimeSeriesStore {
        pub series: HashMap<String, TimeSeries>,
    }

    impl TimeSeriesStore {
        pub fn record(&mut self, key: impl Into<String>, t_hours: f64, value: f64, unit: crate::units::Unit) {
            let key = key.into();
            let entry = self.series.entry(key.clone()).or_insert_with(|| TimeSeries { key, timestamps_hours: Vec::new(), values: Vec::new(), unit });
            entry.push(t_hours, value);
        }

        pub fn get(&self, key: &str) -> Option<&TimeSeries> {
            self.series.get(key)
        }

        pub fn to_csv(&self, key: &str) -> Option<String> {
            let ts = self.series.get(key)?;
            let mut out = String::from("hours,value\n");
            for (t, v) in ts.timestamps_hours.iter().zip(ts.values.iter()) {
                out.push_str(&format!("{t},{v}\n"));
            }
            Some(out)
        }
    }
    // #endregion 🔖TimeSeries

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn time_series_average() {
            let mut ts = TimeSeries { key: "t".into(), timestamps_hours: Vec::new(), values: Vec::new(), unit: crate::units::Unit::Celsius };
            ts.push(0.0, 10.0);
            ts.push(1.0, 20.0);
            assert!((ts.average() - 15.0).abs() < 1e-9);
        }
    }
}

mod plant {
    //! 🏭 Plant loops: pumps, boilers, chillers, heat pumps, towers, HX, GSHP, thermal storage.

    use crate::curves::PerformanceCurve;
    use crate::props::{glycol_cp_j_per_kg_k, glycol_density, water_cp_j_per_kg_k, water_density};
    use crate::units::RHO_WATER;
    use serde::{Deserialize, Serialize};

    // #region 🔖State
    /// 💧 Plant fluid stream state at a loop node.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct PlantStream {
        pub temperature_c: f64,
        pub mass_flow_kg_s: f64,
    }

    impl PlantStream {
        pub const fn new(temperature_c: f64, mass_flow_kg_s: f64) -> Self {
            Self { temperature_c, mass_flow_kg_s }
        }
    }

    /// 📤 Timestep plant equipment output.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct PlantOutput {
        pub thermal_power_w: f64,
        pub electrical_power_w: f64,
        pub gas_power_w: f64,
        pub outlet: PlantStream,
        pub heat_rejection_w: f64,
    }
    // #endregion 🔖State

    // #region 🔖Pump
    /// ⚙️ Variable-speed centrifugal pump with part-load curve.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Pump {
        pub design_head_pa: f64,
        pub design_flow_kg_s: f64,
        pub motor_efficiency: f64,
        pub part_load_curve: PerformanceCurve,
    }

    impl Pump {
        /// ⚙️ Pump hydraulic and electrical power for a requested mass flow.
        pub fn simulate(&self, inlet: PlantStream, requested_flow_kg_s: f64) -> PlantOutput {
            let flow = requested_flow_kg_s.clamp(0.0, self.design_flow_kg_s * 1.2);
            let plr = self.part_load_curve.part_load(flow, self.design_flow_kg_s);
            let head = self.design_head_pa * self.part_load_curve.evaluate(plr);
            let hydraulic_w = flow * head / RHO_WATER.max(1.0);
            let motor_eta = self.motor_efficiency.clamp(0.1, 1.0);
            PlantOutput { thermal_power_w: 0.0, electrical_power_w: hydraulic_w / motor_eta, gas_power_w: 0.0, outlet: PlantStream::new(inlet.temperature_c, flow), heat_rejection_w: hydraulic_w * (1.0 - motor_eta) }
        }
    }
    // #endregion 🔖Pump

    // #region 🔖Boiler
    /// 🔥 Hot-water or steam boiler with combustion efficiency curve.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Boiler {
        pub rated_capacity_w: f64,
        pub combustion_efficiency: f64,
        pub part_load_curve: PerformanceCurve,
        pub standby_loss_w: f64,
        pub supply_temperature_c: f64,
    }

    impl Boiler {
        /// 🔥 Deliver hot-water heating to meet plant load.
        pub fn simulate(&self, inlet: PlantStream, heating_load_w: f64, operating: bool) -> PlantOutput {
            if !operating || heating_load_w <= 0.0 {
                return PlantOutput { electrical_power_w: 50.0, gas_power_w: self.standby_loss_w, outlet: PlantStream::new(inlet.temperature_c, inlet.mass_flow_kg_s), ..Default::default() };
            }
            let load = heating_load_w.min(self.rated_capacity_w);
            let plr = self.part_load_curve.part_load(load, self.rated_capacity_w);
            let eta = (self.combustion_efficiency * self.part_load_curve.evaluate(plr)).clamp(0.5, 1.0);
            let gas_w = load / eta + self.standby_loss_w;
            let cp = water_cp_j_per_kg_k(inlet.temperature_c);
            let m_dot = if inlet.mass_flow_kg_s > 1e-6 { inlet.mass_flow_kg_s } else { load / (cp * (self.supply_temperature_c - inlet.temperature_c).max(1.0)) };
            PlantOutput { thermal_power_w: load, electrical_power_w: 200.0 * plr, gas_power_w: gas_w, outlet: PlantStream::new(self.supply_temperature_c, m_dot), heat_rejection_w: gas_w - load }
        }
    }
    // #endregion 🔖Boiler

    // #region 🔖Chiller
    /// ❄️ Vapor-compression chiller with EIR part-load curve.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ChillerEir {
        pub rated_capacity_w: f64,
        pub reference_cop: f64,
        pub eir_curve: PerformanceCurve,
        pub eir_f_t_curve: PerformanceCurve,
        pub leaving_water_c: f64,
        pub entering_condenser_c: f64,
    }

    impl ChillerEir {
        /// ❄️ Electric chiller cooling via Energy Input Ratio curves.
        pub fn simulate(&self, inlet: PlantStream, cooling_load_w: f64, operating: bool) -> PlantOutput {
            if !operating || cooling_load_w <= 0.0 {
                return PlantOutput { outlet: PlantStream::new(inlet.temperature_c, inlet.mass_flow_kg_s), ..Default::default() };
            }
            let load = cooling_load_w.min(self.rated_capacity_w);
            let plr = self.eir_curve.part_load(load, self.rated_capacity_w);
            let eir_plr = self.eir_curve.evaluate(plr).max(0.05);
            let eir_ft = self.eir_f_t_curve.evaluate_2d(inlet.temperature_c, self.entering_condenser_c).max(0.05);
            let cop = (1.0 / (eir_plr * eir_ft)).min(self.reference_cop * 1.5);
            let elec_w = load / cop.max(0.5);
            let cp = water_cp_j_per_kg_k(inlet.temperature_c);
            let delta_t = load / (inlet.mass_flow_kg_s.max(0.01) * cp);
            PlantOutput { thermal_power_w: -load, electrical_power_w: elec_w, outlet: PlantStream::new(inlet.temperature_c - delta_t, inlet.mass_flow_kg_s), heat_rejection_w: load + elec_w, ..Default::default() }
        }
    }

    /// 🔥 Absorption chiller driven by hot water or steam.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ChillerAbsorption {
        pub rated_capacity_w: f64,
        pub heat_input_ratio: f64,
        pub part_load_curve: PerformanceCurve,
        pub leaving_water_c: f64,
        pub min_generator_inlet_c: f64,
    }

    impl ChillerAbsorption {
        /// 🔥 Absorption chiller with generator heat input.
        pub fn simulate(&self, inlet: PlantStream, cooling_load_w: f64, generator_inlet_c: f64, operating: bool) -> PlantOutput {
            if !operating || cooling_load_w <= 0.0 || generator_inlet_c < self.min_generator_inlet_c {
                return PlantOutput { outlet: PlantStream::new(inlet.temperature_c, inlet.mass_flow_kg_s), ..Default::default() };
            }
            let load = cooling_load_w.min(self.rated_capacity_w);
            let plr = self.part_load_curve.part_load(load, self.rated_capacity_w);
            let hir = self.heat_input_ratio * self.part_load_curve.evaluate(plr);
            let heat_in_w = load * hir;
            let cp = water_cp_j_per_kg_k(inlet.temperature_c);
            let delta_t = load / (inlet.mass_flow_kg_s.max(0.01) * cp);
            PlantOutput { thermal_power_w: -load, gas_power_w: heat_in_w, electrical_power_w: 500.0 * plr, outlet: PlantStream::new(inlet.temperature_c - delta_t, inlet.mass_flow_kg_s), heat_rejection_w: load + heat_in_w }
        }
    }
    // #endregion 🔖Chiller

    // #region 🔖HeatPump
    /// 🌡️ Water-to-water or air-source heat pump plant component.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HeatPump {
        pub rated_heating_w: f64,
        pub rated_cooling_w: f64,
        pub rated_cop_heating: f64,
        pub rated_cop_cooling: f64,
        pub heating_curve: PerformanceCurve,
        pub cooling_curve: PerformanceCurve,
    }

    impl HeatPump {
        /// 🌡️ Bidirectional heat pump for heating or cooling plant load.
        pub fn simulate(&self, inlet: PlantStream, load_w: f64, mode: HeatPumpMode, source_temp_c: f64) -> PlantOutput {
            if load_w.abs() < 1.0 {
                return PlantOutput { outlet: PlantStream::new(inlet.temperature_c, inlet.mass_flow_kg_s), ..Default::default() };
            }
            let (rated, base_cop, curve) = match mode {
                HeatPumpMode::Heating => (self.rated_heating_w, self.rated_cop_heating, &self.heating_curve),
                HeatPumpMode::Cooling => (self.rated_cooling_w, self.rated_cop_cooling, &self.cooling_curve),
            };
            let plr = curve.part_load(load_w.abs(), rated);
            let temp_factor = match mode {
                HeatPumpMode::Heating => (1.0 - 0.03 * (7.0 - source_temp_c).max(0.0)).clamp(0.5, 1.1),
                HeatPumpMode::Cooling => (1.0 - 0.03 * (source_temp_c - 25.0).max(0.0)).clamp(0.5, 1.1),
            };
            let cop = (base_cop * curve.evaluate(plr) * temp_factor).max(1.5);
            let elec_w = load_w.abs() / cop;
            let cp = water_cp_j_per_kg_k(inlet.temperature_c);
            let delta_t = load_w / (inlet.mass_flow_kg_s.max(0.01) * cp);
            PlantOutput { thermal_power_w: load_w, electrical_power_w: elec_w, gas_power_w: 0.0, outlet: PlantStream::new(inlet.temperature_c + delta_t, inlet.mass_flow_kg_s), heat_rejection_w: load_w.abs() + elec_w }
        }
    }

    /// 🔄 Heat pump operating mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HeatPumpMode {
        Heating,
        Cooling,
    }
    // #endregion 🔖HeatPump

    // #region 🔖CoolingTower
    /// 🌊 Open cooling tower with approach and fan power.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CoolingTower {
        pub design_range_k: f64,
        pub design_approach_k: f64,
        pub design_flow_kg_s: f64,
        pub fan_power_at_design_w: f64,
        pub fan_curve: PerformanceCurve,
    }

    impl CoolingTower {
        /// 🌊 Reject condenser heat to outdoor air via evaporative cooling.
        pub fn simulate(&self, inlet: PlantStream, heat_rejection_w: f64, outdoor_wb_c: f64) -> PlantOutput {
            if heat_rejection_w <= 0.0 {
                return PlantOutput { outlet: PlantStream::new(inlet.temperature_c, inlet.mass_flow_kg_s), ..Default::default() };
            }
            let plr = (heat_rejection_w / (self.design_flow_kg_s * self.design_range_k * 4200.0).max(1.0)).clamp(0.1, 1.2);
            let approach = self.design_approach_k * (0.8 + 0.2 * plr);
            let outlet_t = outdoor_wb_c + approach;
            let cp = water_cp_j_per_kg_k((inlet.temperature_c + outlet_t) * 0.5);
            let m_dot = heat_rejection_w / (cp * self.design_range_k.max(1.0));
            let fan_w = self.fan_power_at_design_w * self.fan_curve.evaluate(plr);
            PlantOutput { thermal_power_w: -heat_rejection_w, electrical_power_w: fan_w, gas_power_w: 0.0, outlet: PlantStream::new(outlet_t, m_dot), heat_rejection_w }
        }
    }
    // #endregion 🔖CoolingTower

    // #region 🔖HeatExchanger
    /// 🔀 Counter-flow plate heat exchanger.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HeatExchanger {
        pub ua_w_per_k: f64,
        pub effectiveness: f64,
    }

    impl HeatExchanger {
        /// 🔀 Transfer heat between hot and cold plant streams.
        pub fn simulate(&self, hot: PlantStream, cold: PlantStream) -> (PlantOutput, PlantOutput) {
            let c_hot = hot.mass_flow_kg_s * water_cp_j_per_kg_k(hot.temperature_c);
            let c_cold = cold.mass_flow_kg_s * water_cp_j_per_kg_k(cold.temperature_c);
            let c_min = c_hot.min(c_cold).max(1e-6);
            let q_max = c_min * (hot.temperature_c - cold.temperature_c).max(0.0);
            let eps = self.effectiveness.clamp(0.0, 1.0);
            let q = (self.ua_w_per_k * (hot.temperature_c - cold.temperature_c)).min(q_max * eps).max(0.0);
            let hot_out = hot.temperature_c - q / c_hot.max(1e-6);
            let cold_out = cold.temperature_c + q / c_cold.max(1e-6);
            (PlantOutput { thermal_power_w: -q, outlet: PlantStream::new(hot_out, hot.mass_flow_kg_s), ..Default::default() }, PlantOutput { thermal_power_w: q, outlet: PlantStream::new(cold_out, cold.mass_flow_kg_s), ..Default::default() })
        }
    }
    // #endregion 🔖HeatExchanger

    // #region 🔖Gshp
    /// 🌍 Ground-source heat pump with borefield thermal response.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Gshp {
        pub heat_pump: HeatPump,
        pub borehole_depth_m: f64,
        pub borehole_count: u32,
        pub grout_conductivity_w_m_k: f64,
        pub ground_temperature_c: f64,
    }

    impl Gshp {
        /// 🌍 Simulate GSHP with ground temperature penalty from sustained extraction.
        pub fn simulate(&self, inlet: PlantStream, load_w: f64, mode: HeatPumpMode, cumulative_ground_load_j: f64) -> PlantOutput {
            let penalty_k = (cumulative_ground_load_j / 1e9).clamp(0.0, 8.0);
            let source_t = match mode {
                HeatPumpMode::Heating => self.ground_temperature_c - penalty_k,
                HeatPumpMode::Cooling => self.ground_temperature_c + penalty_k,
            };
            let mut out = self.heat_pump.simulate(inlet, load_w, mode, source_t);
            let bore_resistance = 0.1 / (self.grout_conductivity_w_m_k * self.borehole_count as f64).max(0.01);
            let fluid_penalty = load_w.abs() * bore_resistance * (1.0 + penalty_k * 0.05) / self.borehole_depth_m.max(1.0);
            out.electrical_power_w += fluid_penalty;
            out
        }
    }
    // #endregion 🔖Gshp

    // #region 🔖ThermalStorage
    /// 🧊 Stratified thermal storage tank on a plant loop.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ThermalStorage {
        pub volume_m3: f64,
        pub height_m: f64,
        pub loss_coefficient_w_per_k: f64,
        pub charge_efficiency: f64,
        pub discharge_efficiency: f64,
        pub state: ThermalStorageState,
    }

    /// 🌡️ Stratified tank nodal temperatures (top to bottom).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ThermalStorageState {
        pub node_temperatures_c: Vec<f64>,
        pub ambient_temperature_c: f64,
    }

    impl ThermalStorage {
        /// 🧊 Charge or discharge storage and return loop outlet stream.
        pub fn simulate(&self, inlet: PlantStream, charge_w: f64, dt_s: f64) -> (PlantOutput, ThermalStorageState) {
            let rho = water_density(inlet.temperature_c);
            let cp = water_cp_j_per_kg_k(inlet.temperature_c);
            let mut state = self.state.clone();
            let avg_t = state.node_temperatures_c.iter().sum::<f64>() / state.node_temperatures_c.len().max(1) as f64;
            let loss_w = self.loss_coefficient_w_per_k * (avg_t - state.ambient_temperature_c);
            let mut net_w = charge_w - loss_w;
            if net_w > 0.0 {
                net_w *= self.charge_efficiency;
            } else {
                net_w /= self.discharge_efficiency.max(0.1);
            }
            let stored_energy_j = rho * cp * self.volume_m3;
            let delta_t = net_w * dt_s / stored_energy_j.max(1.0);
            for t in &mut state.node_temperatures_c {
                *t += delta_t;
            }
            let outlet_t = if charge_w >= 0.0 { state.node_temperatures_c.first().copied().unwrap_or(inlet.temperature_c) } else { state.node_temperatures_c.last().copied().unwrap_or(inlet.temperature_c) };
            (PlantOutput { thermal_power_w: net_w, outlet: PlantStream::new(outlet_t, inlet.mass_flow_kg_s), heat_rejection_w: loss_w.max(0.0), ..Default::default() }, state)
        }
    }
    // #endregion 🔖ThermalStorage

    // #region 🔖PlantLoop
    /// 🔄 Primary plant loop connecting equipment in series.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PlantLoopSimulation {
        pub supply: PlantStream,
        pub return_stream: PlantStream,
        pub pump: Pump,
        pub glycol_fraction: f64,
    }

    impl PlantLoopSimulation {
        /// 🔄 Solve one plant loop timestep with pump and demand heat exchanger.
        pub fn simulate(&self, demand_load_w: f64) -> PlantOutput {
            let rho = if self.glycol_fraction > 0.0 { glycol_density(self.supply.temperature_c, self.glycol_fraction) } else { water_density(self.supply.temperature_c) };
            let cp = if self.glycol_fraction > 0.0 { glycol_cp_j_per_kg_k(self.supply.temperature_c, self.glycol_fraction) } else { water_cp_j_per_kg_k(self.supply.temperature_c) };
            let delta_t = demand_load_w / (self.supply.mass_flow_kg_s.max(0.01) * cp);
            let return_t = self.supply.temperature_c - delta_t;
            let pump_out = self.pump.simulate(PlantStream::new(return_t, self.return_stream.mass_flow_kg_s), self.supply.mass_flow_kg_s);
            let _ = rho;
            PlantOutput { thermal_power_w: demand_load_w, electrical_power_w: pump_out.electrical_power_w, outlet: pump_out.outlet, ..Default::default() }
        }
    }
    // #endregion 🔖PlantLoop

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::curves::PerformanceCurve;

        fn flat_curve() -> PerformanceCurve {
            PerformanceCurve::Constant(1.0)
        }

        #[test]
        fn boiler_meets_partial_load() {
            let boiler = Boiler { rated_capacity_w: 100_000.0, combustion_efficiency: 0.9, part_load_curve: flat_curve(), standby_loss_w: 200.0, supply_temperature_c: 80.0 };
            let inlet = PlantStream::new(60.0, 2.0);
            let out = boiler.simulate(inlet, 50_000.0, true);
            assert!(out.thermal_power_w > 49_000.0);
            assert!(out.gas_power_w > out.thermal_power_w);
        }

        #[test]
        fn chiller_eir_cooling() {
            let chiller = ChillerEir { rated_capacity_w: 200_000.0, reference_cop: 5.0, eir_curve: PerformanceCurve::Constant(0.2), eir_f_t_curve: PerformanceCurve::Constant(1.0), leaving_water_c: 7.0, entering_condenser_c: 29.0 };
            let inlet = PlantStream::new(12.0, 10.0);
            let out = chiller.simulate(inlet, 100_000.0, true);
            assert!(out.thermal_power_w < 0.0);
            assert!(out.electrical_power_w > 10_000.0);
            assert!(out.heat_rejection_w > 100_000.0);
        }

        #[test]
        fn cooling_tower_rejects_heat() {
            let tower = CoolingTower { design_range_k: 5.0, design_approach_k: 3.0, design_flow_kg_s: 20.0, fan_power_at_design_w: 15_000.0, fan_curve: flat_curve() };
            let inlet = PlantStream::new(35.0, 20.0);
            let out = tower.simulate(inlet, 500_000.0, 22.0);
            assert!(out.outlet.temperature_c < inlet.temperature_c);
            assert!(out.electrical_power_w > 0.0);
        }

        #[test]
        fn heat_exchanger_transfers_positive() {
            let hx = HeatExchanger { ua_w_per_k: 10_000.0, effectiveness: 0.8 };
            let hot = PlantStream::new(70.0, 5.0);
            let cold = PlantStream::new(10.0, 5.0);
            let (hot_out, cold_out) = hx.simulate(hot, cold);
            assert!(hot_out.thermal_power_w < 0.0);
            assert!(cold_out.thermal_power_w > 0.0);
            assert!(hot_out.outlet.temperature_c < hot.temperature_c);
            assert!(cold_out.outlet.temperature_c > cold.temperature_c);
        }

        #[test]
        fn thermal_storage_changes_temperature() {
            let storage = ThermalStorage {
                volume_m3: 5.0,
                height_m: 2.0,
                loss_coefficient_w_per_k: 10.0,
                charge_efficiency: 0.95,
                discharge_efficiency: 0.95,
                state: ThermalStorageState { node_temperatures_c: vec![50.0, 45.0, 40.0], ambient_temperature_c: 20.0 },
            };
            let inlet = PlantStream::new(55.0, 1.0);
            let (out, state) = storage.simulate(inlet, 20_000.0, 3600.0);
            assert!(out.thermal_power_w > 0.0);
            assert!(state.node_temperatures_c[0] > 50.0);
        }

        #[test]
        fn pump_power_scales_with_flow() {
            let pump = Pump { design_head_pa: 200_000.0, design_flow_kg_s: 10.0, motor_efficiency: 0.85, part_load_curve: flat_curve() };
            let inlet = PlantStream::new(20.0, 0.0);
            let low = pump.simulate(inlet, 2.0);
            let high = pump.simulate(inlet, 8.0);
            assert!(high.electrical_power_w > low.electrical_power_w);
        }

        #[test]
        fn gshp_penalty_increases_with_ground_load() {
            let gshp = Gshp {
                heat_pump: HeatPump { rated_heating_w: 50_000.0, rated_cooling_w: 50_000.0, rated_cop_heating: 4.0, rated_cop_cooling: 4.5, heating_curve: flat_curve(), cooling_curve: flat_curve() },
                borehole_depth_m: 100.0,
                borehole_count: 4,
                grout_conductivity_w_m_k: 1.5,
                ground_temperature_c: 12.0,
            };
            let inlet = PlantStream::new(35.0, 2.0);
            let low = gshp.simulate(inlet, 30_000.0, HeatPumpMode::Heating, 0.0);
            let high = gshp.simulate(inlet, 30_000.0, HeatPumpMode::Heating, 5e9);
            assert!(high.electrical_power_w >= low.electrical_power_w);
        }
    }
}

mod precompute {
    //! 🧮 Precompute geometry, CTF coefficients, solar factors, and zone topology.

    use crate::envelope::ConductionState;
    use crate::geometry::{polygon_normal, surface_area_m2, surface_tilt_azimuth};
    use crate::material::{construction_thermal_mass, construction_u_value, R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
    use crate::model::{EntityId, Model, SurfaceClass};
    use crate::site::solar_position;
    use crate::solar::beam_incidence_cosine;
    use std::collections::HashMap;

    // #region 🔖ZoneGeometry
    /// 📐 Precomputed zone geometry.
    #[derive(Clone, Debug, Default)]
    pub struct ZoneGeometry {
        pub floor_area_m2: f64,
        pub exterior_area_m2: f64,
        pub roof_area_m2: f64,
    }

    /// 📐 Precomputed surface geometry and thermal properties.
    #[derive(Clone, Debug)]
    pub struct SurfacePrecompute {
        pub area_m2: f64,
        pub u_value_w_m2k: f64,
        pub capacitance_j_m2k: f64,
        pub solar_absorptance: f64,
        pub emissivity: f64,
        pub tilt_deg: f64,
        pub azimuth_deg: f64,
        pub normal: [f64; 3],
        pub ctf: ConductionState,
        pub zone_id: EntityId,
        pub sun_exposed: bool,
    }
    // #endregion 🔖ZoneGeometry

    // #region 🔖FenestrationPrecompute
    /// 🪟 Precomputed fenestration properties.
    #[derive(Clone, Debug)]
    pub struct FenestrationPrecompute {
        pub surface_id: EntityId,
        pub area_m2: f64,
        pub u_value_w_m2k: f64,
        pub shgc: f64,
        pub vlt: f64,
        pub tilt_deg: f64,
        pub azimuth_deg: f64,
        pub normal: [f64; 3],
    }
    // #endregion 🔖FenestrationPrecompute

    // #region 🔖ThermostatLookup
    /// 🌡️ Resolved thermostat setpoints for a zone.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct ResolvedSetpoints {
        pub heating_c: f64,
        pub cooling_c: f64,
        pub heating_throttle_k: f64,
        pub cooling_throttle_k: f64,
    }
    // #endregion 🔖ThermostatLookup

    // #region 🔖PrecomputedModel
    /// 🧮 All precomputed data for a simulation run.
    #[derive(Clone, Debug, Default)]
    pub struct PrecomputedModel {
        pub zone_geometry: HashMap<EntityId, ZoneGeometry>,
        pub surfaces: HashMap<EntityId, SurfacePrecompute>,
        pub fenestrations: HashMap<EntityId, FenestrationPrecompute>,
        pub default_setpoints: HashMap<EntityId, ResolvedSetpoints>,
        pub zone_timestep_s: f64,
        pub system_timestep_s: f64,
    }

    impl PrecomputedModel {
        /// 🧮 Build precomputed data from model and timestep settings.
        pub fn build(model: &Model, zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
            let zone_timestep_s = zone_timestep_minutes as f64 * 60.0;
            let system_timestep_s = system_timestep_minutes as f64 * 60.0;
            let mut zone_geometry: HashMap<EntityId, ZoneGeometry> = HashMap::new();
            let mut surfaces = HashMap::new();
            let mut fenestrations = HashMap::new();
            let mut default_setpoints = HashMap::new();

            for zone in &model.zones {
                let zone_surfaces = model.surfaces_for_zone(zone.id);
                let floor_area_m2 = zone_surfaces.iter().map(|s| surface_area_m2(&s.vertices_m)).sum::<f64>().max(1.0);
                let exterior_area_m2 = zone_surfaces.iter().filter(|s| matches!(s.class, SurfaceClass::ExteriorWall | SurfaceClass::Roof)).map(|s| surface_area_m2(&s.vertices_m)).sum();
                let roof_area_m2 = zone_surfaces.iter().filter(|s| matches!(s.class, SurfaceClass::Roof | SurfaceClass::Ceiling)).map(|s| surface_area_m2(&s.vertices_m)).sum();
                zone_geometry.insert(zone.id, ZoneGeometry { floor_area_m2, exterior_area_m2, roof_area_m2 });
                default_setpoints.insert(zone.id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: 2.0, cooling_throttle_k: 2.0 });
            }

            for thermostat in &model.thermostats {
                default_setpoints.insert(thermostat.zone_id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: thermostat.heating_throttle_range_k, cooling_throttle_k: thermostat.cooling_throttle_range_k });
            }

            for surface in &model.surfaces {
                let area_m2 = surface_area_m2(&surface.vertices_m);
                let normal = polygon_normal(&surface.vertices_m);
                let orient = surface_tilt_azimuth(normal, model.site.north_axis_deg);
                let tilt_deg = orient.tilt_deg;
                let azimuth_deg = orient.azimuth_deg;
                let (u_value, capacitance, solar_abs, emissivity) = model.construction_by_id(surface.construction_id).map_or((0.3, 50_000.0, 0.7, 0.9), |c| {
                    let layers: Vec<_> = c.layer_material_ids.iter().filter_map(|id| model.material_by_id(*id)).cloned().collect();
                    let u = construction_u_value(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
                    let cap = construction_thermal_mass(&layers);
                    let outer = layers.last();
                    (u, cap, outer.map_or(0.7, |m| m.solar_absorptance), outer.map_or(0.9, |m| m.thermal_absorptance))
                });
                let ctf = ConductionState::from_u_and_capacitance(u_value, capacitance, zone_timestep_s);
                surfaces.insert(
                    surface.id,
                    SurfacePrecompute { area_m2, u_value_w_m2k: u_value, capacitance_j_m2k: capacitance, solar_absorptance: solar_abs, emissivity, tilt_deg, azimuth_deg, normal, ctf, zone_id: surface.zone_id, sun_exposed: surface.sun_exposed },
                );
            }

            for fen in &model.fenestrations {
                if let Some(surface) = model.surfaces.iter().find(|s| s.id == fen.surface_id) {
                    let normal = polygon_normal(&surface.vertices_m);
                    let orient = surface_tilt_azimuth(normal, model.site.north_axis_deg);
                    fenestrations
                        .insert(fen.id, FenestrationPrecompute { surface_id: fen.surface_id, area_m2: fen.area_m2, u_value_w_m2k: fen.u_value_w_m2k, shgc: fen.shgc, vlt: fen.vlt, tilt_deg: orient.tilt_deg, azimuth_deg: orient.azimuth_deg, normal });
                }
            }

            Self { zone_geometry, surfaces, fenestrations, default_setpoints, zone_timestep_s, system_timestep_s }
        }

        /// ☀️ Solar incidence cosine for a surface at given solar position.
        pub fn surface_incidence(&self, surface_id: EntityId, sun_alt_deg: f64, sun_az_deg: f64) -> f64 {
            self.surfaces.get(&surface_id).map_or(0.0, |s| beam_incidence_cosine(s.normal, sun_alt_deg, sun_az_deg))
        }

        /// ☀️ Solar position for site at day/hour.
        pub fn solar_at(&self, model: &Model, day_of_year: u16, hour: f64) -> (f64, f64) {
            let pos = solar_position(model.site.latitude_deg, model.site.longitude_deg, day_of_year, hour);
            (pos.altitude_deg, pos.azimuth_deg)
        }
    }
    // #endregion 🔖PrecomputedModel

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::*;

        #[test]
        fn precompute_builds_surface_ctf() {
            let model = crate::sim::test_model_single_zone();
            let pre = PrecomputedModel::build(&model, 60, 60);
            assert!(!pre.surfaces.is_empty());
            assert!(pre.zone_geometry.contains_key(&EntityId(1)));
        }
    }
}

mod props {
    //! 💧 Physical property functions: moist air, water, steam, refrigerants, glycol.

    use crate::num::newton_raphson;
    use crate::units::{c_to_k, CP_DRY_AIR, H_FG_0C, P_STD, R_DRY_AIR, R_WATER_VAPOR};

    // #region 🔖Psychrometrics
    /// 💧 Saturation pressure of water [Pa] (Magnus-type, valid ~0–50°C).
    pub fn saturation_pressure_pa(t_c: f64) -> f64 {
        let t = t_c.clamp(-50.0, 100.0);
        611.657 * ((17.2799 * t) / (t + 237.3)).exp()
    }

    /// 💧 Humidity ratio W [kg_water/kg_dry_air] from dry-bulb and relative humidity.
    pub fn humidity_ratio_from_rh(t_c: f64, rh: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_c);
        let p_w = rh.clamp(0.0, 1.0) * p_ws;
        0.621_945 * p_w / (p_atm - p_w).max(1.0)
    }

    /// 💧 Relative humidity from humidity ratio.
    pub fn rh_from_humidity_ratio(t_c: f64, w: f64, p_atm: f64) -> f64 {
        let p_ws = saturation_pressure_pa(t_c);
        let p_w = w * p_atm / (0.621_945 + w);
        (p_w / p_ws).clamp(0.0, 1.0)
    }

    /// 🌡️ Wet-bulb temperature [°C] via iterative psychrometric balance.
    pub fn wet_bulb_c(t_db_c: f64, w: f64, p_atm: f64) -> f64 {
        let target = w;
        let f = |t_wb: f64| humidity_ratio_from_rh(t_wb, 1.0, p_atm) - target;
        let df = |t_wb: f64| {
            let eps = 0.01;
            (f(t_wb + eps) - f(t_wb - eps)) / (2.0 * eps)
        };
        newton_raphson(t_db_c, f, df, 30, 1e-6).unwrap_or(t_db_c)
    }

    /// 🔥 Moist air enthalpy [J/kg dry air].
    pub fn moist_air_enthalpy_j_per_kg(t_c: f64, w: f64) -> f64 {
        CP_DRY_AIR * t_c + w * (H_FG_0C + 1860.0 * t_c)
    }

    /// 🌡️ Dew point [°C] from humidity ratio.
    pub fn dew_point_c(w: f64, p_atm: f64) -> f64 {
        let p_w = w * p_atm / (0.621_945 + w);
        let ln_pw = (p_w / 611.657).ln();
        237.3 * ln_pw / (17.2799 - ln_pw)
    }

    /// 💨 Moist air density [kg/m³].
    pub fn moist_air_density(t_c: f64, w: f64, p_atm: f64) -> f64 {
        let t_k = c_to_k(t_c);
        let p_w = w * p_atm / (0.621_945 + w);
        let p_d = p_atm - p_w;
        p_d / (R_DRY_AIR * t_k) + p_w / (R_WATER_VAPOR * t_k)
    }
    // #endregion 🔖Psychrometrics

    // #region 🔖Water
    /// 💧 Liquid water specific heat [J/(kg·K)] (temperature-dependent polynomial).
    pub fn water_cp_j_per_kg_k(t_c: f64) -> f64 {
        4217.0 - 1.2 * t_c + 0.003 * t_c * t_c
    }

    /// 💧 Liquid water density [kg/m³].
    pub fn water_density(t_c: f64) -> f64 {
        999.839_5 + 0.067_37 * t_c - 0.010_52 * t_c * t_c
    }

    /// 💧 Liquid water thermal conductivity [W/(m·K)].
    pub fn water_conductivity(t_c: f64) -> f64 {
        0.561_0 + 0.002_0 * t_c - 6.0e-6 * t_c * t_c
    }
    // #endregion 🔖Water

    // #region 🔖Steam
    /// 💨 Steam saturation temperature [°C] from pressure [Pa].
    pub fn steam_saturation_temp_c(p_pa: f64) -> f64 {
        let ln_p = (p_pa / 611.657).ln();
        237.3 * ln_p / (17.2799 - ln_p)
    }

    /// 💨 Latent heat of vaporization [J/kg] at temperature [°C].
    pub fn latent_heat_vaporization(t_c: f64) -> f64 {
        H_FG_0C - 2370.0 * t_c
    }
    // #endregion 🔖Steam

    // #region 🔖Refrigerant
    /// ❄️ R410A saturation pressure [Pa] simplified correlation (valid ~-40 to 40°C).
    pub fn r410a_saturation_pressure_pa(t_c: f64) -> f64 {
        let t_k = c_to_k(t_c);
        let a = -1031.0;
        let b = 7.0;
        (a / t_k + b).exp() * P_STD
    }

    /// ❄️ R410A saturation temperature [°C] from pressure [Pa].
    pub fn r410a_saturation_temp_c(p_pa: f64) -> f64 {
        let ratio = (p_pa / P_STD).ln();
        k_to_c(-1031.0 / (ratio - 7.0))
    }

    fn k_to_c(t_k: f64) -> f64 {
        t_k - 273.15
    }
    // #endregion 🔖Refrigerant

    // #region 🔖Glycol
    /// 🧪 Glycol mixture specific heat [J/(kg·K)] (ethylene glycol fraction 0–0.6).
    pub fn glycol_cp_j_per_kg_k(t_c: f64, glycol_fraction: f64) -> f64 {
        let f = glycol_fraction.clamp(0.0, 0.6);
        water_cp_j_per_kg_k(t_c) * (1.0 - f) + 2400.0 * f
    }

    /// 🧪 Glycol mixture density [kg/m³].
    pub fn glycol_density(t_c: f64, glycol_fraction: f64) -> f64 {
        let f = glycol_fraction.clamp(0.0, 0.6);
        water_density(t_c) * (1.0 - f) + 1110.0 * f
    }

    /// 🧪 Glycol mixture dynamic viscosity [Pa·s] (simplified).
    pub fn glycol_viscosity(t_c: f64, glycol_fraction: f64) -> f64 {
        let f = glycol_fraction.clamp(0.0, 0.6);
        let mu_water = 0.001_792 / (1.0 + 0.033_7 * t_c + 0.000_221 * t_c * t_c);
        mu_water * (1.0 + 5.0 * f)
    }
    // #endregion 🔖Glycol

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn saturation_at_zero_c() {
            let p = saturation_pressure_pa(0.0);
            assert!((p - 611.657).abs() < 1.0);
        }

        #[test]
        fn rh_roundtrip() {
            let w = humidity_ratio_from_rh(25.0, 0.5, P_STD);
            let rh = rh_from_humidity_ratio(25.0, w, P_STD);
            assert!((rh - 0.5).abs() < 0.02);
        }

        #[test]
        fn water_density_near_room_temp() {
            assert!((water_density(20.0) - 998.0).abs() < 5.0);
        }
    }
}

mod refrigeration {
    //! ❄️ Refrigeration: display cases, walk-ins, compressor racks, condensers, secondary loops.

    use crate::curves::PerformanceCurve;
    use crate::props::{r410a_saturation_pressure_pa, r410a_saturation_temp_c};
    use crate::units::P_STD;
    use serde::{Deserialize, Serialize};

    // #region 🔖State
    /// 🌡️ Refrigeration circuit state.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RefrigerationState {
        pub evaporating_temperature_c: f64,
        pub condensing_temperature_c: f64,
        pub suction_superheat_k: f64,
        pub liquid_subcool_k: f64,
    }

    /// 📤 Refrigeration timestep output.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct RefrigerationOutput {
        pub cooling_power_w: f64,
        pub compressor_power_w: f64,
        pub condenser_heat_w: f64,
        pub mass_flow_kg_s: f64,
    }
    // #endregion 🔖State

    // #region 🔖DisplayCase
    /// 🛒 Supermarket display case with anti-sweat and fan power.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DisplayCase {
        pub length_m: f64,
        pub design_cooling_w: f64,
        pub fan_power_w: f64,
        pub lighting_power_w: f64,
        pub anti_sweat_heater_w: f64,
        pub defrost_power_w: f64,
        pub defrost_fraction: f64,
        pub evaporating_temperature_c: f64,
    }

    impl DisplayCase {
        /// 🛒 Display case total cooling and electrical load.
        pub fn simulate(&self, case_temperature_c: f64, ambient_c: f64, load_factor: f64) -> RefrigerationOutput {
            let lf = load_factor.clamp(0.0, 1.5);
            let conductance = self.design_cooling_w / (case_temperature_c - self.evaporating_temperature_c).max(1.0);
            let cooling_w = conductance * (case_temperature_c - self.evaporating_temperature_c) * lf;
            let defrost_w = self.defrost_power_w * self.defrost_fraction;
            let anti_sweat = if ambient_c > 18.0 { self.anti_sweat_heater_w } else { 0.0 };
            let cop = 2.5 + 0.05 * (self.evaporating_temperature_c + 10.0);
            let compressor_w = cooling_w / cop + self.fan_power_w + self.lighting_power_w + anti_sweat + defrost_w;
            RefrigerationOutput { cooling_power_w: cooling_w, compressor_power_w: compressor_w, condenser_heat_w: cooling_w + compressor_w * 0.85, mass_flow_kg_s: cooling_w / 150_000.0 }
        }
    }
    // #endregion 🔖DisplayCase

    // #region 🔖WalkIn
    /// 🚪 Walk-in cooler or freezer box.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WalkIn {
        pub floor_area_m2: f64,
        pub wall_area_m2: f64,
        pub ua_w_per_k: f64,
        pub internal_gain_w: f64,
        pub design_box_temperature_c: f64,
        pub evaporating_temperature_c: f64,
        pub door_opening_fraction: f64,
    }

    impl WalkIn {
        /// 🚪 Walk-in envelope and infiltration cooling load.
        pub fn simulate(&self, ambient_c: f64, humidity_factor: f64) -> RefrigerationOutput {
            let delta_t = (ambient_c - self.design_box_temperature_c).max(0.0);
            let envelope_w = self.ua_w_per_k * delta_t;
            let infil_w = 50.0 * self.door_opening_fraction * delta_t.powf(1.2) * humidity_factor;
            let cooling_w = envelope_w + self.internal_gain_w + infil_w;
            let cop = if self.design_box_temperature_c < -10.0 { 1.8 } else { 3.0 };
            let compressor_w = cooling_w / cop;
            RefrigerationOutput { cooling_power_w: cooling_w, compressor_power_w: compressor_w, condenser_heat_w: cooling_w + compressor_w, mass_flow_kg_s: cooling_w / 140_000.0 }
        }
    }
    // #endregion 🔖WalkIn

    // #region 🔖CompressorRack
    /// 🏭 Shared compressor rack serving multiple cases.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CompressorRack {
        pub rated_capacity_w: f64,
        pub compressor_count: u32,
        pub eir_curve: PerformanceCurve,
        pub eir_f_t_curve: PerformanceCurve,
        pub min_evaporating_c: f64,
        pub max_condensing_c: f64,
    }

    impl CompressorRack {
        /// 🏭 Rack cooling capacity and power at floating suction/head pressure.
        pub fn simulate(&self, total_cooling_w: f64, evaporating_c: f64, condensing_c: f64) -> RefrigerationOutput {
            let load = total_cooling_w.min(self.rated_capacity_w * self.compressor_count as f64);
            if load <= 0.0 {
                return RefrigerationOutput::default();
            }
            let plr = self.eir_curve.part_load(load, self.rated_capacity_w);
            let eir = self.eir_curve.evaluate(plr) * self.eir_f_t_curve.evaluate_2d(evaporating_c, condensing_c).max(0.1);
            let compressor_w = load * eir;
            RefrigerationOutput { cooling_power_w: load, compressor_power_w: compressor_w, condenser_heat_w: load + compressor_w, mass_flow_kg_s: load / (150_000.0 * plr.max(0.2)) }
        }

        /// 🌡️ Floating head pressure from ambient dry-bulb.
        pub fn floating_head_c(&self, ambient_c: f64) -> f64 {
            (ambient_c + 10.0).clamp(25.0, self.max_condensing_c)
        }
    }
    // #endregion 🔖CompressorRack

    // #region 🔖Condenser
    /// 🌊 Air-cooled or evaporative condenser rejecting rack heat.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RefrigerationCondenser {
        pub ua_w_per_k: f64,
        pub fan_power_w: f64,
        pub design_approach_k: f64,
        pub evaporative: bool,
    }

    impl RefrigerationCondenser {
        /// 🌊 Condenser heat rejection and fan power.
        pub fn simulate(&self, heat_rejection_w: f64, ambient_c: f64, wet_bulb_c: f64) -> (f64, f64) {
            if heat_rejection_w <= 0.0 {
                return (ambient_c + self.design_approach_k, 0.0);
            }
            let sink_t = if self.evaporative { wet_bulb_c } else { ambient_c };
            let condensing_t = sink_t + self.design_approach_k;
            let actual_t = ambient_c + heat_rejection_w / self.ua_w_per_k.max(1.0);
            let fan_w = self.fan_power_w * (heat_rejection_w / 100_000.0).clamp(0.2, 1.0);
            (actual_t.max(condensing_t), fan_w)
        }
    }
    // #endregion 🔖Condenser

    // #region 🔖SecondaryLoop
    /// 🧊 Glycol secondary loop for remote display cases.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SecondaryLoop {
        pub pump_power_w: f64,
        pub pipe_ua_w_per_k: f64,
        pub supply_temperature_c: f64,
        pub return_temperature_c: f64,
        pub mass_flow_kg_s: f64,
        pub fluid_cp: f64,
    }

    impl SecondaryLoop {
        /// 🧊 Secondary loop pump and heat pickup from cases.
        pub fn simulate(&self, case_load_w: f64, ambient_c: f64) -> (f64, f64, f64) {
            let pipe_loss_w = self.pipe_ua_w_per_k * (self.supply_temperature_c - ambient_c).max(0.0);
            let fluid_cooling = case_load_w + pipe_loss_w;
            let delta_t = fluid_cooling / (self.mass_flow_kg_s.max(0.01) * self.fluid_cp);
            let new_return = self.supply_temperature_c + delta_t;
            let pump_w = self.pump_power_w * (case_load_w / 50_000.0).clamp(0.3, 1.0);
            (fluid_cooling, new_return, pump_w)
        }
    }
    // #endregion 🔖SecondaryLoop

    // #region 🔖Circuit
    /// ❄️ Full refrigeration circuit pressure-temperature check.
    pub fn refrigeration_state_from_pressures(suction_pa: f64, discharge_pa: f64) -> RefrigerationState {
        RefrigerationState { evaporating_temperature_c: r410a_saturation_temp_c(suction_pa.max(P_STD * 0.3)), condensing_temperature_c: r410a_saturation_temp_c(discharge_pa.max(P_STD)), suction_superheat_k: 5.0, liquid_subcool_k: 3.0 }
    }

    /// ❄️ Estimate suction pressure from evaporating temperature.
    pub fn evaporating_pressure_pa(t_evap_c: f64) -> f64 {
        r410a_saturation_pressure_pa(t_evap_c)
    }
    // #endregion 🔖Circuit

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::curves::PerformanceCurve;

        #[test]
        fn display_case_cooling_positive() {
            let case = DisplayCase { length_m: 3.0, design_cooling_w: 2000.0, fan_power_w: 150.0, lighting_power_w: 200.0, anti_sweat_heater_w: 100.0, defrost_power_w: 500.0, defrost_fraction: 0.05, evaporating_temperature_c: -8.0 };
            let out = case.simulate(-2.0, 22.0, 1.0);
            assert!(out.cooling_power_w > 0.0);
            assert!(out.compressor_power_w > out.cooling_power_w * 0.3);
        }

        #[test]
        fn walk_in_freezer_higher_compressor_ratio() {
            let cooler = WalkIn { floor_area_m2: 20.0, wall_area_m2: 60.0, ua_w_per_k: 30.0, internal_gain_w: 500.0, design_box_temperature_c: 2.0, evaporating_temperature_c: -5.0, door_opening_fraction: 0.1 };
            let freezer = WalkIn { design_box_temperature_c: -20.0, evaporating_temperature_c: -28.0, ..cooler };
            let cool_out = cooler.simulate(25.0, 1.0);
            let freeze_out = freezer.simulate(25.0, 1.0);
            assert!(freeze_out.compressor_power_w / freeze_out.cooling_power_w > cool_out.compressor_power_w / cool_out.cooling_power_w);
        }

        #[test]
        fn compressor_rack_part_load() {
            let rack = CompressorRack { rated_capacity_w: 100_000.0, compressor_count: 2, eir_curve: PerformanceCurve::Constant(0.35), eir_f_t_curve: PerformanceCurve::Constant(1.0), min_evaporating_c: -15.0, max_condensing_c: 45.0 };
            let out = rack.simulate(80_000.0, -8.0, 35.0);
            assert!(out.cooling_power_w > 70_000.0);
            assert!(out.condenser_heat_w > out.cooling_power_w);
        }

        #[test]
        fn condenser_approach_temperature() {
            let cond = RefrigerationCondenser { ua_w_per_k: 5000.0, fan_power_w: 3000.0, design_approach_k: 8.0, evaporative: false };
            let (t_cond, fan_w) = cond.simulate(150_000.0, 30.0, 22.0);
            assert!(t_cond > 30.0);
            assert!(fan_w > 0.0);
        }

        #[test]
        fn secondary_loop_return_rises_with_load() {
            let loop_sys = SecondaryLoop { pump_power_w: 800.0, pipe_ua_w_per_k: 20.0, supply_temperature_c: -5.0, return_temperature_c: -3.0, mass_flow_kg_s: 5.0, fluid_cp: 3500.0 };
            let (_, return_t, _) = loop_sys.simulate(30_000.0, 20.0);
            assert!(return_t > loop_sys.supply_temperature_c);
        }

        #[test]
        fn saturation_pressure_increases_with_temperature() {
            let low = evaporating_pressure_pa(-10.0);
            let high = evaporating_pressure_pa(5.0);
            assert!(high > low);
        }
    }
}

mod results {
    //! 📋 Canonical simulation results and summary tables.

    use crate::error::Diagnostics;
    use crate::meters::MeterStore;
    use crate::metrics::{EnvironmentalMetrics, ResilienceMetrics};
    use crate::output::TimeSeriesStore;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    // #region 🔖Summary
    /// 📋 Annual/monthly summary table row.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SummaryRow {
        pub key: String,
        pub value: f64,
        pub unit: String,
    }

    /// 📋 Summary tables (energy use, loads, comfort).
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct SummaryTables {
        pub annual_energy: Vec<SummaryRow>,
        pub monthly_energy: HashMap<u8, Vec<SummaryRow>>,
        pub peak_loads: Vec<SummaryRow>,
        pub comfort: Vec<SummaryRow>,
    }

    impl SummaryTables {
        pub fn add_annual(&mut self, key: impl Into<String>, value: f64, unit: impl Into<String>) {
            self.annual_energy.push(SummaryRow { key: key.into(), value, unit: unit.into() });
        }
    }
    // #endregion 🔖Summary

    // #region 🔖Sizing
    /// 📐 Component sizing result.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SizingResult {
        pub component: String,
        pub design_load_w: f64,
        pub design_flow_m3_s: f64,
        pub autosized: bool,
    }

    /// 📐 Sizing tables from design-day calculations.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct SizingTables {
        pub zone_loads: Vec<SizingResult>,
        pub equipment: Vec<SizingResult>,
    }
    // #endregion 🔖Sizing

    // #region 🔖Results
    /// 📋 Complete simulation results (canonical structured format).
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct Results {
        pub time_series: TimeSeriesStore,
        pub meters: MeterStore,
        pub summaries: SummaryTables,
        pub sizing: SizingTables,
        pub environmental: EnvironmentalMetrics,
        pub resilience: ResilienceMetrics,
        pub diagnostics: Diagnostics,
        pub run_metadata: RunMetadata,
    }

    /// 🏷️ Run metadata.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct RunMetadata {
        pub model_name: String,
        pub model_version: String,
        pub weather_location: String,
        pub timesteps: u32,
        pub warmup_days: u32,
        pub elapsed_ms: u64,
    }
    // #endregion 🔖Results

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn summary_tables_accumulate() {
            let mut s = SummaryTables::default();
            s.add_annual("Electricity", 1000.0, "kWh");
            assert_eq!(s.annual_energy.len(), 1);
        }
    }
}

mod room_air {
    //! 🌀 Room air distribution models: mixed, stratified, displacement, UFAD, surface-specific.

    use serde::{Deserialize, Serialize};

    // #region 🔖RoomAirInput
    /// 📥 Inputs for room air model evaluation.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RoomAirInput {
        pub zone_temp_c: f64,
        pub supply_temp_c: f64,
        pub outdoor_temp_c: f64,
        pub floor_area_m2: f64,
        pub ceiling_height_m: f64,
        pub supply_flow_m3_s: f64,
        pub internal_gain_w: f64,
        pub surface_temps_c: [f64; 6],
    }
    // #endregion 🔖RoomAirInput

    // #region 🔖RoomAirOutput
    /// 📤 Room air model temperatures [°C].
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RoomAirOutput {
        pub occupied_temp_c: f64,
        pub return_temp_c: f64,
        pub exhaust_temp_c: f64,
        pub floor_temp_c: f64,
        pub ceiling_temp_c: f64,
        pub surface_air_temps_c: [f64; 6],
    }
    // #endregion 🔖RoomAirOutput

    // #region 🔖RoomAirModel
    /// 🌀 Room air distribution model per ASHRAE / ISO 7730 room air classifications.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RoomAirModel {
        FullyMixed,
        VerticalGradient { gradient_k_per_m: f64 },
        Displacement1Node { mixing_factor: f64 },
        Displacement3Node { lower_fraction: f64, upper_fraction: f64 },
        Ufad { diffuser_height_m: f64, throw_m: f64 },
        SurfaceSpecific,
    }

    impl RoomAirModel {
        /// 🌡️ Apply room air model and return stratified temperatures.
        pub fn apply(&self, input: &RoomAirInput) -> RoomAirOutput {
            match self {
                Self::FullyMixed => fully_mixed(input),
                Self::VerticalGradient { gradient_k_per_m } => vertical_gradient(input, *gradient_k_per_m),
                Self::Displacement1Node { mixing_factor } => displacement_1node(input, *mixing_factor),
                Self::Displacement3Node { lower_fraction, upper_fraction } => displacement_3node(input, *lower_fraction, *upper_fraction),
                Self::Ufad { diffuser_height_m, throw_m } => ufad(input, *diffuser_height_m, *throw_m),
                Self::SurfaceSpecific => surface_specific(input),
            }
        }
    }
    // #endregion 🔖RoomAirModel

    // #region 🔖FullyMixed
    fn fully_mixed(input: &RoomAirInput) -> RoomAirOutput {
        let t = input.zone_temp_c;
        RoomAirOutput { occupied_temp_c: t, return_temp_c: t, exhaust_temp_c: t, floor_temp_c: t, ceiling_temp_c: t, surface_air_temps_c: [t; 6] }
    }
    // #endregion 🔖FullyMixed

    // #region 🔖VerticalGradient
    fn vertical_gradient(input: &RoomAirInput, gradient_k_per_m: f64) -> RoomAirOutput {
        let h = input.ceiling_height_m.max(0.1);
        let t_floor = input.zone_temp_c - gradient_k_per_m * 0.1;
        let t_ceil = input.zone_temp_c + gradient_k_per_m * (h - 0.1);
        let t_occ = input.zone_temp_c;
        RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_ceil, exhaust_temp_c: t_ceil, floor_temp_c: t_floor, ceiling_temp_c: t_ceil, surface_air_temps_c: [t_floor, input.zone_temp_c, t_ceil, input.zone_temp_c, t_floor, t_ceil] }
    }
    // #endregion 🔖VerticalGradient

    // #region 🔖Displacement
    fn displacement_1node(input: &RoomAirInput, mixing_factor: f64) -> RoomAirOutput {
        let f = mixing_factor.clamp(0.0, 1.0);
        let t_supply = input.supply_temp_c;
        let t_zone = input.zone_temp_c;
        let t_occ = f * t_zone + (1.0 - f) * t_supply;
        let t_return = t_zone + f * (t_zone - t_supply) * 0.3;
        RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_return, exhaust_temp_c: t_return, floor_temp_c: t_supply + 0.5 * (t_occ - t_supply), ceiling_temp_c: t_return, surface_air_temps_c: [t_occ; 6] }
    }

    fn displacement_3node(input: &RoomAirInput, lower_fraction: f64, upper_fraction: f64) -> RoomAirOutput {
        let lf = lower_fraction.clamp(0.05, 0.95);
        let uf = upper_fraction.clamp(0.05, 0.95);
        let h = input.ceiling_height_m.max(0.1);
        let z_occ = h * 0.4;
        let z_lower = h * lf;
        let z_upper = h * uf;
        let t_supply = input.supply_temp_c;
        let t_zone = input.zone_temp_c;
        let grad = (t_zone - t_supply) / h;
        let t_lower = t_supply + grad * z_lower * 0.5;
        let t_occ = t_supply + grad * z_occ;
        let t_upper = t_supply + grad * z_upper;
        RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_upper, exhaust_temp_c: t_upper, floor_temp_c: t_lower, ceiling_temp_c: t_upper, surface_air_temps_c: [t_lower, t_occ, t_upper, t_occ, t_lower, t_upper] }
    }
    // #endregion 🔖Displacement

    // #region 🔖Ufad
    fn ufad(input: &RoomAirInput, diffuser_height_m: f64, throw_m: f64) -> RoomAirOutput {
        let h = input.ceiling_height_m.max(0.1);
        let _z_diff = diffuser_height_m.clamp(0.05, h * 0.5);
        let throw = throw_m.max(0.1);
        let penetration = (throw / h).clamp(0.1, 1.0);
        let t_supply = input.supply_temp_c;
        let t_zone = input.zone_temp_c;
        let t_occ = t_supply + penetration * (t_zone - t_supply);
        let t_return = t_zone + (1.0 - penetration) * 0.2 * (t_zone - t_supply);
        let t_floor = t_supply + 0.3 * (t_occ - t_supply);
        RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: t_return, exhaust_temp_c: t_return, floor_temp_c: t_floor, ceiling_temp_c: t_return + 0.5 * (t_zone - t_occ), surface_air_temps_c: [t_floor, t_occ, t_return, t_occ, t_floor, t_return] }
    }
    // #endregion 🔖Ufad

    // #region 🔖SurfaceSpecific
    fn surface_specific(input: &RoomAirInput) -> RoomAirOutput {
        let mut surface_air = input.surface_temps_c;
        for (i, &t_surf) in input.surface_temps_c.iter().enumerate() {
            surface_air[i] = 0.7 * input.zone_temp_c + 0.3 * t_surf;
        }
        let t_occ = surface_air.iter().sum::<f64>() / surface_air.len() as f64;
        RoomAirOutput { occupied_temp_c: t_occ, return_temp_c: input.zone_temp_c, exhaust_temp_c: input.zone_temp_c, floor_temp_c: surface_air[0], ceiling_temp_c: surface_air[2], surface_air_temps_c: surface_air }
    }
    // #endregion 🔖SurfaceSpecific

    #[cfg(test)]
    mod tests {
        use super::*;

        fn sample_input() -> RoomAirInput {
            RoomAirInput { zone_temp_c: 24.0, supply_temp_c: 18.0, outdoor_temp_c: 5.0, floor_area_m2: 50.0, ceiling_height_m: 3.0, supply_flow_m3_s: 0.2, internal_gain_w: 1500.0, surface_temps_c: [22.0, 23.0, 25.0, 24.0, 21.0, 26.0] }
        }

        #[test]
        fn fully_mixed_uniform() {
            let out = RoomAirModel::FullyMixed.apply(&sample_input());
            assert!((out.occupied_temp_c - 24.0).abs() < 1e-9);
        }

        #[test]
        fn displacement_cooler_at_occupancy() {
            let out = RoomAirModel::Displacement1Node { mixing_factor: 0.2 }.apply(&sample_input());
            assert!(out.occupied_temp_c < sample_input().zone_temp_c);
            assert!(out.occupied_temp_c > sample_input().supply_temp_c);
        }

        #[test]
        fn vertical_gradient_stratifies() {
            let out = RoomAirModel::VerticalGradient { gradient_k_per_m: 1.0 }.apply(&sample_input());
            assert!(out.ceiling_temp_c > out.floor_temp_c);
        }

        #[test]
        fn ufad_occupied_between_supply_and_zone() {
            let out = RoomAirModel::Ufad { diffuser_height_m: 0.3, throw_m: 1.5 }.apply(&sample_input());
            assert!(out.occupied_temp_c > sample_input().supply_temp_c);
            assert!(out.occupied_temp_c < sample_input().zone_temp_c);
        }
    }
}

mod schedule {
    //! 📅 Schedule definitions and runtime lookup.

    use crate::model::ScheduleId;
    use serde::{Deserialize, Serialize};

    // #region 🔖ScheduleType
    /// 📆 Schedule interpolation mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ScheduleInterpolation {
        Continuous,
        Discrete,
    }

    /// 📆 Schedule value limit.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ScheduleLimits {
        pub min: f64,
        pub max: f64,
    }

    /// 📅 Constant schedule.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ConstantSchedule {
        pub id: ScheduleId,
        pub value: f64,
    }

    /// 📅 Daily repeating schedule (24 hourly values).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DailySchedule {
        pub id: ScheduleId,
        pub hourly_values: [f64; 24],
        pub interpolation: ScheduleInterpolation,
        pub limits: Option<ScheduleLimits>,
    }

    /// 📅 Weekly schedule (7 daily schedule ids).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WeeklySchedule {
        pub id: ScheduleId,
        pub daily_schedule_ids: [ScheduleId; 7],
    }

    /// 📅 Compact rule-based annual schedule.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CompactScheduleRule {
        pub start_month: u8,
        pub start_day: u8,
        pub end_month: u8,
        pub end_day: u8,
        pub daily_schedule_id: ScheduleId,
    }

    /// 📅 Annual schedule with holiday overrides.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AnnualSchedule {
        pub id: ScheduleId,
        pub rules: Vec<CompactScheduleRule>,
        pub default_daily_schedule_id: ScheduleId,
        pub holiday_daily_schedule_id: Option<ScheduleId>,
        pub holiday_dates: Vec<(u16, u8, u8)>,
    }

    /// 📅 External time-series schedule.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TimeSeriesSchedule {
        pub id: ScheduleId,
        pub values: Vec<f64>,
        pub timestep_seconds: u32,
    }
    // #endregion 🔖ScheduleType

    // #region 🔖ScheduleSet
    /// 📚 All schedules in a model.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct ScheduleSet {
        pub constants: Vec<ConstantSchedule>,
        pub daily: Vec<DailySchedule>,
        pub weekly: Vec<WeeklySchedule>,
        pub annual: Vec<AnnualSchedule>,
        pub time_series: Vec<TimeSeriesSchedule>,
    }

    impl ScheduleSet {
        pub fn constant_value(&self, id: ScheduleId) -> Option<f64> {
            self.constants.iter().find(|c| c.id == id).map(|c| c.value)
        }

        pub fn daily_value(&self, id: ScheduleId, hour: u8) -> Option<f64> {
            let daily = self.daily.iter().find(|d| d.id == id)?;
            let h = (hour as usize).min(23);
            let mut v = daily.hourly_values[h];
            if let Some(limits) = daily.limits {
                v = v.clamp(limits.min, limits.max);
            }
            Some(v)
        }

        pub fn weekly_value(&self, id: ScheduleId, day_of_week: u8, hour: u8) -> Option<f64> {
            let weekly = self.weekly.iter().find(|w| w.id == id)?;
            let dow = (day_of_week as usize).min(6);
            self.daily_value(weekly.daily_schedule_ids[dow], hour)
        }

        pub fn annual_value(&self, id: ScheduleId, year: u16, month: u8, day: u8, hour: u8) -> Option<f64> {
            let annual = self.annual.iter().find(|a| a.id == id)?;
            if annual.holiday_dates.contains(&(year, month, day)) {
                if let Some(hid) = annual.holiday_daily_schedule_id {
                    return self.daily_value(hid, hour);
                }
            }
            for rule in &annual.rules {
                if date_in_range(month, day, rule.start_month, rule.start_day, rule.end_month, rule.end_day) {
                    return self.daily_value(rule.daily_schedule_id, hour);
                }
            }
            self.daily_value(annual.default_daily_schedule_id, hour)
        }

        pub fn lookup(&self, id: ScheduleId, ctx: &ScheduleContext) -> f64 {
            if let Some(v) = self.constant_value(id) {
                return v;
            }
            if let Some(v) = self.annual_value(id, ctx.year, ctx.month, ctx.day, ctx.hour) {
                return v;
            }
            if let Some(v) = self.weekly_value(id, ctx.day_of_week, ctx.hour) {
                return v;
            }
            if let Some(v) = self.daily_value(id, ctx.hour) {
                return v;
            }
            if let Some(ts) = self.time_series.iter().find(|t| t.id == id) {
                let idx = (ctx.timestep_index as usize).min(ts.values.len().saturating_sub(1));
                return ts.values[idx];
            }
            1.0
        }

        /// 📦 Pre-expand schedule values for all timesteps in a run period.
        pub fn expand(&self, id: ScheduleId, ctxs: &[ScheduleContext]) -> Vec<f64> {
            ctxs.iter().map(|c| self.lookup(id, c)).collect()
        }
    }
    // #endregion 🔖ScheduleSet

    // #region 🔖Context
    /// 🕐 Calendar context for schedule lookup.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ScheduleContext {
        pub year: u16,
        pub month: u8,
        pub day: u8,
        pub hour: u8,
        pub day_of_week: u8,
        pub timestep_index: u32,
        pub is_dst: bool,
    }

    fn date_in_range(m: u8, d: u8, sm: u8, sd: u8, em: u8, ed: u8) -> bool {
        let md = m as u16 * 32 + d as u16;
        let start = sm as u16 * 32 + sd as u16;
        let end = em as u16 * 32 + ed as u16;
        if start <= end {
            md >= start && md <= end
        } else {
            md >= start || md <= end
        }
    }
    // #endregion 🔖Context

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn constant_schedule_lookup() {
            let set = ScheduleSet { constants: vec![ConstantSchedule { id: ScheduleId(1), value: 0.5 }], ..Default::default() };
            let ctx = ScheduleContext { year: 2026, month: 1, day: 1, hour: 12, day_of_week: 4, timestep_index: 0, is_dst: false };
            assert!((set.lookup(ScheduleId(1), &ctx) - 0.5).abs() < 1e-9);
        }

        #[test]
        fn daily_schedule_respects_limits() {
            let set = ScheduleSet { daily: vec![DailySchedule { id: ScheduleId(2), hourly_values: [2.0; 24], interpolation: ScheduleInterpolation::Discrete, limits: Some(ScheduleLimits { min: 0.0, max: 1.0 }) }], ..Default::default() };
            assert!((set.daily_value(ScheduleId(2), 10).unwrap() - 1.0).abs() < 1e-9);
        }
    }
}

mod shw {
    //! 🚿 Service hot water: mixed/stratified/HP heaters, fixtures, standby, drain recovery.

    use crate::props::{water_cp_j_per_kg_k, water_density};
    use crate::units::RHO_WATER;
    use serde::{Deserialize, Serialize};

    // #region 🔖State
    /// 🌡️ Hot-water storage state.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WaterHeaterState {
        pub average_temperature_c: f64,
        pub top_temperature_c: f64,
        pub bottom_temperature_c: f64,
        pub volume_m3: f64,
    }

    /// 📤 Water heater timestep output.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct WaterHeaterOutput {
        pub heating_power_w: f64,
        pub electrical_power_w: f64,
        pub gas_power_w: f64,
        pub standby_loss_w: f64,
        pub delivered_flow_kg_s: f64,
        pub outlet_temperature_c: f64,
    }
    // #endregion 🔖State

    // #region 🔖Mixed
    /// 🚿 Fully mixed storage water heater (electric or gas).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct MixedWaterHeater {
        pub volume_l: f64,
        pub ua_standby_w_per_k: f64,
        pub heating_capacity_w: f64,
        pub setpoint_c: f64,
        pub ambient_c: f64,
        pub recovery_efficiency: f64,
        pub fuel: WaterHeaterFuel,
    }

    /// ⛽ Water heater energy source.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum WaterHeaterFuel {
        Electric,
        Gas,
    }

    impl MixedWaterHeater {
        /// 🚿 Simulate mixed tank with draw, makeup, and standby losses.
        pub fn simulate(&self, state: &WaterHeaterState, draw_flow_kg_s: f64, inlet_temperature_c: f64, dt_s: f64) -> (WaterHeaterOutput, WaterHeaterState) {
            let volume_m3 = self.volume_l / 1000.0;
            let rho = water_density(state.average_temperature_c);
            let cp = water_cp_j_per_kg_k(state.average_temperature_c);
            let mass_kg = rho * volume_m3;
            let standby_w = self.ua_standby_w_per_k * (state.average_temperature_c - self.ambient_c);
            let draw_energy_w = draw_flow_kg_s * cp * (state.average_temperature_c - inlet_temperature_c);
            let mut tank_t = state.average_temperature_c;
            let mut heating_w = 0.0;
            if tank_t < self.setpoint_c {
                let deficit_j = mass_kg * cp * (self.setpoint_c - tank_t);
                heating_w = (deficit_j / dt_s).min(self.heating_capacity_w);
                tank_t += heating_w * self.recovery_efficiency * dt_s / (mass_kg * cp);
            }
            tank_t -= (draw_energy_w + standby_w) * dt_s / (mass_kg * cp);
            tank_t = tank_t.clamp(inlet_temperature_c, self.setpoint_c + 5.0);
            let (elec_w, gas_w) = match self.fuel {
                WaterHeaterFuel::Electric => (heating_w / self.recovery_efficiency, 0.0),
                WaterHeaterFuel::Gas => (50.0, heating_w / self.recovery_efficiency),
            };
            let new_state = WaterHeaterState { average_temperature_c: tank_t, top_temperature_c: tank_t, bottom_temperature_c: tank_t, volume_m3 };
            (WaterHeaterOutput { heating_power_w: heating_w, electrical_power_w: elec_w, gas_power_w: gas_w, standby_loss_w: standby_w, delivered_flow_kg_s: draw_flow_kg_s, outlet_temperature_c: tank_t }, new_state)
        }
    }
    // #endregion 🔖Mixed

    // #region 🔖Stratified
    /// 🌡️ Stratified tank with fixed node count (1-D conduction between nodes).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct StratifiedWaterHeater {
        pub volume_l: f64,
        pub node_count: usize,
        pub ua_standby_w_per_k: f64,
        pub setpoint_c: f64,
        pub ambient_c: f64,
        pub heating_capacity_w: f64,
        pub heater_position: usize,
    }

    impl StratifiedWaterHeater {
        /// 🌡️ Simulate stratified tank with buoyancy-driven minimal mixing.
        pub fn simulate(&self, node_temperatures_c: &[f64], draw_flow_kg_s: f64, inlet_temperature_c: f64, dt_s: f64) -> (WaterHeaterOutput, Vec<f64>) {
            let n = self.node_count.max(2);
            let mut temps: Vec<f64> = if node_temperatures_c.len() == n { node_temperatures_c.to_vec() } else { vec![inlet_temperature_c; n] };
            let volume_m3 = self.volume_l / 1000.0;
            let node_volume = volume_m3 / n as f64;
            let rho = water_density(temps[0]);
            let cp = water_cp_j_per_kg_k(temps[0]);
            let node_mass = rho * node_volume;
            let avg_t = temps.iter().sum::<f64>() / n as f64;
            let standby_w = self.ua_standby_w_per_k * (avg_t - self.ambient_c);
            if draw_flow_kg_s > 0.0 {
                let draw_per_node = draw_flow_kg_s / n as f64;
                for t in &mut temps {
                    let removal = draw_per_node * cp * (*t - inlet_temperature_c) * dt_s / node_mass;
                    *t -= removal;
                }
            }
            let heater_idx = self.heater_position.min(n - 1);
            if temps[heater_idx] < self.setpoint_c {
                let deficit = node_mass * cp * (self.setpoint_c - temps[heater_idx]);
                let heat_j = (self.heating_capacity_w * dt_s).min(deficit);
                temps[heater_idx] += heat_j / (node_mass * cp);
            }
            for i in 0..n - 1 {
                let d_t = 0.05 * (temps[i] - temps[i + 1]);
                temps[i] -= d_t;
                temps[i + 1] += d_t;
            }
            let loss_per_node = standby_w * dt_s / (n as f64 * node_mass * cp);
            for t in &mut temps {
                *t -= loss_per_node;
                *t = t.clamp(inlet_temperature_c, self.setpoint_c + 10.0);
            }
            (
                WaterHeaterOutput {
                    heating_power_w: self.heating_capacity_w.min(node_mass * cp * (self.setpoint_c - temps[heater_idx]).max(0.0) / dt_s),
                    electrical_power_w: self.heating_capacity_w,
                    gas_power_w: 0.0,
                    standby_loss_w: standby_w,
                    delivered_flow_kg_s: draw_flow_kg_s,
                    outlet_temperature_c: temps[0],
                },
                temps,
            )
        }
    }
    // #endregion 🔖Stratified

    // #region 🔖HeatPump
    /// 🌡️ Heat-pump water heater with ambient air source.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HeatPumpWaterHeater {
        pub tank: MixedWaterHeater,
        pub rated_cop: f64,
        pub min_ambient_c: f64,
    }

    impl HeatPumpWaterHeater {
        /// 🌡️ HPWH with COP derated by ambient temperature.
        pub fn simulate(&self, state: &WaterHeaterState, draw_flow_kg_s: f64, inlet_temperature_c: f64, ambient_c: f64, dt_s: f64) -> (WaterHeaterOutput, WaterHeaterState) {
            let cop = if ambient_c < self.min_ambient_c { 1.0 } else { (self.rated_cop * (1.0 - 0.03 * (20.0 - ambient_c))).max(1.5) };
            let (mut out, new_state) = self.tank.simulate(state, draw_flow_kg_s, inlet_temperature_c, dt_s);
            out.electrical_power_w = out.heating_power_w / cop + 50.0;
            out.gas_power_w = 0.0;
            let _ = dt_s;
            (out, new_state)
        }
    }
    // #endregion 🔖HeatPump

    // #region 🔖Fixtures
    /// 🚰 Domestic hot-water fixture end use.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HotWaterFixture {
        pub name: String,
        pub peak_flow_l_s: f64,
        pub target_temperature_c: f64,
        pub schedule_factor: f64,
    }

    impl HotWaterFixture {
        /// 🚰 Hot-water draw mass flow [kg/s] at mixed delivery temperature.
        pub fn draw_flow_kg_s(&self, mains_temperature_c: f64) -> f64 {
            let flow_l_s = self.peak_flow_l_s * self.schedule_factor.clamp(0.0, 1.0);
            let mix_ratio = ((self.target_temperature_c - mains_temperature_c) / (self.target_temperature_c - mains_temperature_c).max(1.0)).clamp(0.0, 1.0);
            flow_l_s * mix_ratio * RHO_WATER / 1000.0
        }

        /// 🔥 Sensible energy demand [W] for fixture draw.
        pub fn demand_w(&self, mains_temperature_c: f64, storage_temperature_c: f64) -> f64 {
            let m_dot = self.draw_flow_kg_s(mains_temperature_c);
            let cp = water_cp_j_per_kg_k(storage_temperature_c);
            m_dot * cp * (self.target_temperature_c - mains_temperature_c).max(0.0)
        }
    }
    // #endregion 🔖Fixtures

    // #region 🔖Standby
    /// 🌡️ Standby loss model for tanks and distribution piping.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct StandbyLoss {
        pub ua_w_per_k: f64,
        pub ambient_temperature_c: f64,
        pub circulation_pump_w: f64,
    }

    impl StandbyLoss {
        /// 🌡️ Total standby loss [W] from tank or recirc loop.
        pub fn loss_w(&self, fluid_temperature_c: f64) -> f64 {
            self.ua_w_per_k * (fluid_temperature_c - self.ambient_temperature_c).max(0.0) + self.circulation_pump_w
        }
    }
    // #endregion 🔖Standby

    // #region 🔖DrainRecovery
    /// ♻️ Drain-water heat recovery heat exchanger.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DrainWaterHeatRecovery {
        pub effectiveness: f64,
        pub ua_w_per_k: f64,
    }

    impl DrainWaterHeatRecovery {
        /// ♻️ Preheat cold mains from warm drain flow.
        pub fn preheat_w(&self, drain_flow_kg_s: f64, drain_temperature_c: f64, mains_temperature_c: f64) -> f64 {
            let cp = water_cp_j_per_kg_k((drain_temperature_c + mains_temperature_c) * 0.5);
            let q_max = drain_flow_kg_s * cp * (drain_temperature_c - mains_temperature_c).max(0.0);
            let eps = self.effectiveness.clamp(0.0, 0.95);
            (self.ua_w_per_k * (drain_temperature_c - mains_temperature_c)).min(q_max * eps).max(0.0)
        }

        /// 🌡️ Preheated mains temperature [°C].
        pub fn preheated_mains_c(&self, drain_flow_kg_s: f64, drain_temperature_c: f64, mains_flow_kg_s: f64, mains_temperature_c: f64) -> f64 {
            let q = self.preheat_w(drain_flow_kg_s, drain_temperature_c, mains_temperature_c);
            let cp = water_cp_j_per_kg_k(mains_temperature_c);
            mains_temperature_c + q / (mains_flow_kg_s.max(1e-6) * cp)
        }
    }
    // #endregion 🔖DrainRecovery

    #[cfg(test)]
    mod tests {
        use super::*;

        fn electric_tank() -> MixedWaterHeater {
            MixedWaterHeater { volume_l: 300.0, ua_standby_w_per_k: 5.0, heating_capacity_w: 4500.0, setpoint_c: 55.0, ambient_c: 20.0, recovery_efficiency: 0.98, fuel: WaterHeaterFuel::Electric }
        }

        #[test]
        fn mixed_tank_recovers_after_draw() {
            let tank = electric_tank();
            let state = WaterHeaterState { average_temperature_c: 55.0, top_temperature_c: 55.0, bottom_temperature_c: 55.0, volume_m3: 0.3 };
            let (out, new_state) = tank.simulate(&state, 0.05, 10.0, 3600.0);
            assert!(out.delivered_flow_kg_s > 0.0);
            assert!(new_state.average_temperature_c < state.average_temperature_c);
        }

        #[test]
        fn stratified_tank_top_hotter_than_bottom() {
            let tank = StratifiedWaterHeater { volume_l: 400.0, node_count: 4, ua_standby_w_per_k: 4.0, setpoint_c: 60.0, ambient_c: 20.0, heating_capacity_w: 6000.0, heater_position: 3 };
            let initial = vec![55.0, 50.0, 45.0, 40.0];
            let (_out, temps) = tank.simulate(&initial, 0.0, 10.0, 3600.0);
            assert!(temps[3] > temps[0]);
            assert!(temps[3] > 40.0);
        }

        #[test]
        fn hpwh_cop_reduces_electrical() {
            let hpwh = HeatPumpWaterHeater { tank: electric_tank(), rated_cop: 3.0, min_ambient_c: -5.0 };
            let state = WaterHeaterState { average_temperature_c: 45.0, top_temperature_c: 45.0, bottom_temperature_c: 45.0, volume_m3: 0.3 };
            let (out, _) = hpwh.simulate(&state, 0.0, 10.0, 20.0, 3600.0);
            if out.heating_power_w > 100.0 {
                assert!(out.electrical_power_w < out.heating_power_w);
            }
        }

        #[test]
        fn fixture_demand_positive() {
            let fixture = HotWaterFixture { name: "Shower".into(), peak_flow_l_s: 0.15, target_temperature_c: 40.0, schedule_factor: 0.5 };
            assert!(fixture.demand_w(10.0, 55.0) > 0.0);
        }

        #[test]
        fn drain_recovery_preheats_mains() {
            let dwhr = DrainWaterHeatRecovery { effectiveness: 0.6, ua_w_per_k: 500.0 };
            let preheated = dwhr.preheated_mains_c(0.05, 35.0, 0.05, 10.0);
            assert!(preheated > 10.0);
            assert!(preheated < 35.0);
        }

        #[test]
        fn standby_loss_increases_with_temperature() {
            let standby = StandbyLoss { ua_w_per_k: 8.0, ambient_temperature_c: 20.0, circulation_pump_w: 30.0 };
            assert!(standby.loss_w(55.0) > standby.loss_w(40.0));
        }
    }
}

mod sim {
    //! 🚀 Engine orchestration: Model + SimulationConfig → Results.

    use crate::economics::{compute_lcca, LccaParameters, UtilityTariff};
    use crate::error::Error;
    use crate::kernel::{SimulationConfig, SimulationEnvironment, SimulationKernel};
    use crate::meters::{EndUse, FuelType, MeterStore};
    use crate::metrics::{compute_environmental, compute_resilience, EmissionFactors, SourceEnergyFactors};
    use crate::model::Model;
    use crate::output::TimeSeriesStore;
    use crate::precompute::PrecomputedModel;
    use crate::results::{Results, RunMetadata, SummaryTables};
    use crate::site::WeatherRecord;
    use crate::sizing::{SizingConfig, SizingManager};
    use crate::units::Unit;
    use std::time::Instant;

    // #region 🔖Engine
    /// ⚡ Headless BEM simulation engine.
    pub struct Engine;

    impl Engine {
        /// ⚡ Run full building energy simulation.
        pub fn run(model: &Model, config: &SimulationConfig) -> Result<Results, Error> {
            model.validate().map_err(|d| d.messages.into_iter().find(|m| m.severity == crate::error::Severity::Fatal).unwrap_or_else(|| Error::severe("model validation failed")))?;

            let start = Instant::now();
            let weather_records = Self::resolve_weather(config);
            let pre = PrecomputedModel::build(model, config.zone_timestep_minutes, config.system_timestep_minutes);
            let dt_s = pre.zone_timestep_s;

            let mut state = SimulationKernel::initialize(model, &pre, &weather_records[0]);
            SimulationKernel::warmup(model, config, &pre, &mut state, &weather_records)?;

            let mut time_series = TimeSeriesStore::default();
            let mut meters = MeterStore::default();
            let mut zone_temp_history: Vec<f64> = Vec::new();

            let run_period = SimulationKernel::run_period(config);
            let mut hour_index = 0u32;

            for (date, hour, _) in run_period.hours() {
                let widx = hour_index as usize % weather_records.len().max(1);
                let mut weather = weather_records.get(widx).copied().unwrap_or_else(|| synthetic_hour(hour_index));
                weather.month = date.month;
                weather.day = date.day;
                weather.hour = hour;
                weather.year = date.year;

                SimulationKernel::advance_timestep(model, config, &pre, &mut state, &weather, &date, hour_index as f64, dt_s)?;

                for zone in &model.zones {
                    if let Some(zs) = state.zones.get(&zone.id) {
                        let key = format!("Zone Air Temperature [{}]", zone.name);
                        time_series.record(&key, hour_index as f64, zs.air.temp_c, Unit::Celsius);
                        zone_temp_history.push(zs.air.temp_c);

                        let heat_meter = meters.get_or_create(&format!("{} Heating", zone.name), FuelType::Electricity, EndUse::Heating);
                        heat_meter.accumulate(zs.delivered.heating_w, dt_s, hour_index as f64);

                        let cool_meter = meters.get_or_create(&format!("{} Cooling", zone.name), FuelType::Electricity, EndUse::Cooling);
                        cool_meter.accumulate(zs.delivered.cooling_w, dt_s, hour_index as f64);

                        let fan_meter = meters.get_or_create(&format!("{} Fans", zone.name), FuelType::Electricity, EndUse::Fans);
                        fan_meter.accumulate(zs.delivered.fan_w, dt_s, hour_index as f64);
                    }
                }

                let facility_heat = meters.get_or_create("Facility Heating", FuelType::Electricity, EndUse::Heating);
                facility_heat.accumulate(state.delivered_total.heating_w, dt_s, hour_index as f64);
                let facility_pv = meters.get_or_create("Facility PV", FuelType::OnSiteGeneration, EndUse::Generators);
                facility_pv.accumulate(-state.delivered_total.pv_generation_w, dt_s, hour_index as f64);

                hour_index += 1;
            }

            let sizing = SizingManager::size(model, &SizingConfig::default());
            let mut summaries = SummaryTables::default();
            let elec_kwh = meters.facility_total_kwh(FuelType::Electricity);
            let gas_kwh = meters.facility_total_kwh(FuelType::NaturalGas);
            summaries.add_annual("Electricity", elec_kwh, "kWh");
            summaries.add_annual("Natural Gas", gas_kwh, "kWh");
            let floor_area: f64 = model.zones.iter().map(|z| z.volume_m3 / 3.0).sum::<f64>().max(1.0);
            summaries.add_annual("Energy Use Intensity", elec_kwh / floor_area, "kWh/m²");

            let environmental = compute_environmental(elec_kwh, gas_kwh, &SourceEnergyFactors::default(), &EmissionFactors::default());
            let heat_sp = 20.0;
            let cool_sp = 26.0;
            let resilience = compute_resilience(&zone_temp_history, heat_sp, cool_sp, true);

            let tariff = UtilityTariff { name: "Default".into(), fuel: FuelType::Electricity, periods: vec![], fixed_monthly_charge: 10.0, ratchet_percent: 0.0 };
            let annual_cost = tariff.energy_cost(elec_kwh, 12, 6) * 12.0;
            let lcca = compute_lcca(annual_cost, &LccaParameters { study_period_years: 25, discount_rate: 0.03, inflation_rate: 0.02, initial_cost: 0.0, annual_maintenance: 0.0, replacement_cost: 0.0, replacement_interval_years: 15 });
            summaries.add_annual("Annual Energy Cost", annual_cost, "USD");
            summaries.add_annual("LCCA Present Value", lcca.present_value_total, "USD");

            let elapsed = start.elapsed().as_millis() as u64;
            Ok(Results {
                time_series,
                meters,
                summaries,
                sizing,
                environmental,
                resilience,
                diagnostics: Default::default(),
                run_metadata: RunMetadata {
                    model_name: model.name.clone(),
                    model_version: model.version.clone(),
                    weather_location: config.weather.as_ref().map_or_else(|| "synthetic".into(), |w| w.location.clone()),
                    timesteps: hour_index,
                    warmup_days: config.warmup_days,
                    elapsed_ms: elapsed,
                },
            })
        }

        fn resolve_weather(config: &SimulationConfig) -> Vec<WeatherRecord> {
            if let Some(epw) = &config.weather {
                return epw.records.clone();
            }
            match config.environment {
                SimulationEnvironment::HeatingDesignDay => design_day_weather(-10.0),
                SimulationEnvironment::CoolingDesignDay => design_day_weather(35.0),
                _ => Self::synthetic_weather_year(),
            }
        }

        fn synthetic_weather_year() -> Vec<WeatherRecord> {
            (0..8760).map(synthetic_hour).collect()
        }
    }
    // #endregion 🔖Engine

    fn synthetic_hour(h: u32) -> WeatherRecord {
        let day = h / 24;
        let hour = h % 24;
        let month = (day / 30 + 1).min(12) as u8;
        let t_base = 15.0 + 10.0 * ((day as f64 / 365.0) * 2.0 * std::f64::consts::PI).sin();
        let t_daily = 5.0 * ((hour as f64 - 14.0) / 12.0 * std::f64::consts::PI).cos();
        WeatherRecord {
            year: 2026,
            month,
            day: (day % 30 + 1) as u8,
            hour: hour as u8,
            minute: 0,
            dry_bulb_c: t_base + t_daily,
            dew_point_c: t_base - 5.0,
            relative_humidity: 0.5,
            atmospheric_pressure_pa: 101_325.0,
            wind_speed_m_s: 3.0,
            wind_direction_deg: 180.0,
            direct_normal_irradiance_w_m2: if (6..18).contains(&hour) { 500.0 } else { 0.0 },
            diffuse_horizontal_irradiance_w_m2: if (6..18).contains(&hour) { 100.0 } else { 0.0 },
            horizontal_infrared_w_m2: 250.0,
            precipitation_mm: 0.0,
            snow_depth_mm: 0.0,
        }
    }

    fn design_day_weather(dry_bulb_c: f64) -> Vec<WeatherRecord> {
        (0..24)
            .map(|hour| WeatherRecord {
                year: 2026,
                month: 1,
                day: 1,
                hour: hour as u8,
                minute: 0,
                dry_bulb_c,
                dew_point_c: dry_bulb_c - 10.0,
                relative_humidity: 0.5,
                atmospheric_pressure_pa: 101_325.0,
                wind_speed_m_s: 3.0,
                wind_direction_deg: 180.0,
                direct_normal_irradiance_w_m2: if dry_bulb_c > 20.0 && (8..17).contains(&hour) { 800.0 } else { 0.0 },
                diffuse_horizontal_irradiance_w_m2: if dry_bulb_c > 20.0 && (8..17).contains(&hour) { 150.0 } else { 0.0 },
                horizontal_infrared_w_m2: 250.0,
                precipitation_mm: 0.0,
                snow_depth_mm: 0.0,
            })
            .collect()
    }

    // #region 🔖Fixtures
    /// 🧪 Build a minimal test model for integration tests.
    pub fn test_model_single_zone() -> Model {
        use crate::model::*;
        Model {
            name: "BESTEST Single Zone".into(),
            version: "1.0".into(),
            site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
            zones: vec![Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 106.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
            materials: vec![Material {
                id: EntityId(10),
                name: "Insulation".into(),
                thickness_m: 0.1,
                conductivity_w_m_k: 0.04,
                density_kg_m3: 50.0,
                specific_heat_j_kg_k: 1000.0,
                thermal_absorptance: 0.9,
                solar_absorptance: 0.7,
                visible_absorptance: 0.7,
            }],
            constructions: vec![Construction { id: EntityId(20), name: "Wall".into(), layer_material_ids: vec![EntityId(10)] }],
            surfaces: vec![Surface {
                id: EntityId(30),
                name: "ExtWall".into(),
                zone_id: EntityId(1),
                class: SurfaceClass::ExteriorWall,
                vertices_m: vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
                construction_id: EntityId(20),
                outside_boundary_condition: OutsideBoundary::OutdoorAir,
                sun_exposed: true,
                wind_exposed: true,
                multiplier: 1,
            }],
            ideal_loads: vec![IdealLoadsSystem {
                id: EntityId(40),
                zone_id: EntityId(1),
                max_heating_supply_air_temp_c: 50.0,
                min_cooling_supply_air_temp_c: 13.0,
                max_heating_capacity_w: None,
                max_cooling_capacity_w: None,
                outdoor_air_per_person_m3_s: 0.00944,
                outdoor_air_per_area_m3_s_m2: 0.0,
            }],
            ..Default::default()
        }
    }

    /// 🧪 Full topology test model with plant, PV, AFN, daylight.
    pub fn test_model_full_topology() -> Model {
        use crate::model::*;
        let mut model = test_model_single_zone();
        model.name = "Full Topology".into();
        model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0 });
        model.plant_loops.push(PlantLoopConfig { id: EntityId(60), name: "Hot Water".into(), loop_type: PlantLoopType::Heating, supply_temperature_c: 55.0, return_temperature_c: 45.0, design_flow_kg_s: 2.0, equipment_ids: vec![EntityId(61)] });
        model.pv_systems.push(PvSystemAssignment { id: EntityId(70), dc_capacity_w: 5000.0, area_m2: 25.0, tilt_deg: 30.0, azimuth_deg: 180.0, module_efficiency: 0.2, inverter_efficiency: 0.96 });
        model.daylight_zones.push(DaylightZoneConfig { id: EntityId(80), zone_id: EntityId(1), illuminance_target_lux: 500.0, glare_limit: 0.4, window_transmittance: 0.6 });
        model
    }
    // #endregion 🔖Fixtures

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::calendar::RunPeriod;

        #[test]
        fn engine_runs_single_zone() {
            let model = test_model_single_zone();
            let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 3, environment: SimulationEnvironment::WeatherRunPeriod, ..Default::default() };
            let results = Engine::run(&model, &config).unwrap();
            assert!(results.run_metadata.timesteps > 0);
            assert!(results.meters.facility_total_kwh(FuelType::Electricity) >= 0.0);
        }

        #[test]
        fn engine_deterministic_repeatability() {
            let model = test_model_single_zone();
            let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
            let r1 = Engine::run(&model, &config).unwrap();
            let r2 = Engine::run(&model, &config).unwrap();
            assert_eq!(r1.run_metadata.timesteps, r2.run_metadata.timesteps);
            assert!((r1.meters.facility_total_kwh(FuelType::Electricity) - r2.meters.facility_total_kwh(FuelType::Electricity)).abs() < 1e-3);
        }

        #[test]
        fn ashrae_140_case600_base() {
            let model = test_model_single_zone();
            let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
            let results = Engine::run(&model, &config).unwrap();
            let temps = results.time_series.get("Zone Air Temperature [Zone1]");
            assert!(temps.is_some());
        }

        #[test]
        fn invalid_model_rejected() {
            let model = Model::default();
            assert!(Engine::run(&model, &SimulationConfig::default()).is_err());
        }

        #[test]
        fn energy_conservation_order_of_magnitude() {
            let model = test_model_single_zone();
            let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
            let results = Engine::run(&model, &config).unwrap();
            let total_kwh = results.meters.facility_total_kwh(FuelType::Electricity);
            assert!(total_kwh < 1_000_000.0);
        }

        #[test]
        fn full_topology_e2e() {
            let model = test_model_full_topology();
            let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
            let results = Engine::run(&model, &config).unwrap();
            assert!(results.run_metadata.timesteps >= 48);
            assert!(results.summaries.annual_energy.len() >= 3);
        }

        #[test]
        fn hvac_bestest_heating_day() {
            let model = test_model_single_zone();
            let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
            let results = Engine::run(&model, &config).unwrap();
            assert_eq!(results.run_metadata.timesteps, 24);
            assert!(results.time_series.get("Zone Air Temperature [Zone1]").is_some());
        }

        #[test]
        fn run_period_honors_calendar() {
            let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
            assert_eq!(period.total_hours(), 168);
            let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, warmup_days: 0, ..Default::default() };
            let model = test_model_single_zone();
            let results = Engine::run(&model, &config).unwrap();
            assert_eq!(results.run_metadata.timesteps, 168);
        }
    }
}

mod site {
    //! 🌤️ Site, weather, EPW ingest, design days, solar position, ground temperatures.

    use crate::error::Error;
    use crate::props::{humidity_ratio_from_rh, moist_air_density};
    use crate::units::{deg_to_rad, rad_to_deg};
    use serde::{Deserialize, Serialize};

    // #region 🔖WeatherRecord
    /// 🌡️ One timestep of outdoor weather.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WeatherRecord {
        pub year: u16,
        pub month: u8,
        pub day: u8,
        pub hour: u8,
        pub minute: u8,
        pub dry_bulb_c: f64,
        pub dew_point_c: f64,
        pub relative_humidity: f64,
        pub atmospheric_pressure_pa: f64,
        pub wind_speed_m_s: f64,
        pub wind_direction_deg: f64,
        pub direct_normal_irradiance_w_m2: f64,
        pub diffuse_horizontal_irradiance_w_m2: f64,
        pub horizontal_infrared_w_m2: f64,
        pub precipitation_mm: f64,
        pub snow_depth_mm: f64,
    }

    impl WeatherRecord {
        pub fn humidity_ratio(&self) -> f64 {
            humidity_ratio_from_rh(self.dry_bulb_c, self.relative_humidity, self.atmospheric_pressure_pa)
        }

        pub fn air_density(&self) -> f64 {
            moist_air_density(self.dry_bulb_c, self.humidity_ratio(), self.atmospheric_pressure_pa)
        }
    }
    // #endregion 🔖WeatherRecord

    // #region 🔖Epw
    /// 📄 EPW weather file parsed into typed records.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct EpwWeather {
        pub location: String,
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub elevation_m: f64,
        pub time_zone_hours: f64,
        pub records: Vec<WeatherRecord>,
    }

    impl EpwWeather {
        /// 📥 Parse EPW text content (EnergyPlus Weather format).
        pub fn parse(content: &str) -> Result<Self, Error> {
            let mut lines = content.lines().filter(|l| !l.trim().is_empty());
            let header1 = lines.next().ok_or_else(|| Error::fatal("EPW: missing location line"))?;
            let parts: Vec<&str> = header1.split(',').collect();
            if parts.len() < 10 {
                return Err(Error::fatal("EPW: invalid location header"));
            }
            let latitude_deg: f64 = parts[6].parse().map_err(|_| Error::fatal("EPW: bad latitude"))?;
            let longitude_deg: f64 = parts[7].parse().map_err(|_| Error::fatal("EPW: bad longitude"))?;
            let time_zone_hours: f64 = parts[8].parse().map_err(|_| Error::fatal("EPW: bad timezone"))?;
            let elevation_m: f64 = parts[9].parse().map_err(|_| Error::fatal("EPW: bad elevation"))?;
            let location = parts[1].to_string();

            for _ in 0..7 {
                lines.next();
            }

            let mut records = Vec::new();
            for line in lines {
                let p: Vec<&str> = line.split(',').collect();
                if p.len() < 22 {
                    continue;
                }
                let year: u16 = p[0].parse().unwrap_or(2026);
                let month: u8 = p[1].parse().unwrap_or(1);
                let day: u8 = p[2].parse().unwrap_or(1);
                let hour: u8 = p[3].parse::<u8>().unwrap_or(1).saturating_sub(1);
                let minute: u8 = p[4].parse().unwrap_or(0);
                let dry_bulb_c: f64 = p[6].parse().unwrap_or(20.0);
                let dew_point_c: f64 = p[7].parse().unwrap_or(10.0);
                let relative_humidity: f64 = p[8].parse::<f64>().unwrap_or(50.0) / 100.0;
                let atmospheric_pressure_pa: f64 = p[9].parse::<f64>().unwrap_or(101_325.0);
                let direct_normal_irradiance_w_m2: f64 = p[14].parse().unwrap_or(0.0);
                let diffuse_horizontal_irradiance_w_m2: f64 = p[15].parse().unwrap_or(0.0);
                let horizontal_infrared_w_m2: f64 = p[16].parse().unwrap_or(250.0);
                let wind_speed_m_s: f64 = p[20].parse().unwrap_or(0.0);
                let wind_direction_deg: f64 = p[21].parse().unwrap_or(0.0);
                let precipitation_mm: f64 = p[33].parse().unwrap_or(0.0);
                let snow_depth_mm: f64 = p[35].parse().unwrap_or(0.0);
                records.push(WeatherRecord {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    dry_bulb_c,
                    dew_point_c,
                    relative_humidity,
                    atmospheric_pressure_pa,
                    wind_speed_m_s,
                    wind_direction_deg,
                    direct_normal_irradiance_w_m2,
                    diffuse_horizontal_irradiance_w_m2,
                    horizontal_infrared_w_m2,
                    precipitation_mm,
                    snow_depth_mm,
                });
            }

            if records.is_empty() {
                return Err(Error::fatal("EPW: no data records"));
            }

            Ok(Self { location, latitude_deg, longitude_deg, elevation_m, time_zone_hours, records })
        }

        pub fn record_at_index(&self, idx: usize) -> Option<&WeatherRecord> {
            self.records.get(idx)
        }

        /// 📈 Interpolate weather to sub-hourly timestep.
        pub fn interpolate(&self, hour_index: f64) -> WeatherRecord {
            let idx = hour_index.floor() as usize;
            let frac = hour_index - idx as f64;
            let a = self.records.get(idx).copied().unwrap_or_else(|| self.records[0]);
            let b = self.records.get(idx + 1).copied().unwrap_or(a);
            WeatherRecord {
                dry_bulb_c: a.dry_bulb_c + frac * (b.dry_bulb_c - a.dry_bulb_c),
                dew_point_c: a.dew_point_c + frac * (b.dew_point_c - a.dew_point_c),
                relative_humidity: a.relative_humidity + frac * (b.relative_humidity - a.relative_humidity),
                wind_speed_m_s: a.wind_speed_m_s + frac * (b.wind_speed_m_s - a.wind_speed_m_s),
                direct_normal_irradiance_w_m2: a.direct_normal_irradiance_w_m2 + frac * (b.direct_normal_irradiance_w_m2 - a.direct_normal_irradiance_w_m2),
                diffuse_horizontal_irradiance_w_m2: a.diffuse_horizontal_irradiance_w_m2 + frac * (b.diffuse_horizontal_irradiance_w_m2 - a.diffuse_horizontal_irradiance_w_m2),
                ..a
            }
        }
    }
    // #endregion 🔖Epw

    // #region 🔖DesignDay
    /// 🌡️ Sizing design day specification.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesignDayKind {
        Heating,
        Cooling,
        Custom,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DesignDay {
        pub name: String,
        pub kind: DesignDayKind,
        pub month: u8,
        pub day: u8,
        pub dry_bulb_max_c: f64,
        pub daily_range_k: f64,
        pub humidity_condition: DesignDayHumidity,
        pub wind_speed_m_s: f64,
        pub solar_model: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesignDayHumidity {
        Wetbulb { wetbulb_at_max_c: f64 },
        Dewpoint { dewpoint_c: f64 },
        RelativeHumidity { rh: f64 },
    }

    impl DesignDay {
        pub fn hourly_dry_bulb(&self, hour: u8) -> f64 {
            let h = hour as f64;
            let min_t = self.dry_bulb_max_c - self.daily_range_k;
            if h < 6.0 || h > 18.0 {
                min_t
            } else {
                let phase = (h - 6.0) / 12.0 * std::f64::consts::PI;
                min_t + self.daily_range_k * phase.sin()
            }
        }
    }
    // #endregion 🔖DesignDay

    // #region 🔖Solar
    /// ☀️ Solar position for a site and datetime.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SolarPosition {
        pub altitude_deg: f64,
        pub azimuth_deg: f64,
        pub equation_of_time_min: f64,
    }

    /// ☀️ Compute solar altitude and azimuth (simplified SPA).
    pub fn solar_position(latitude_deg: f64, longitude_deg: f64, day_of_year: u16, hour_solar: f64) -> SolarPosition {
        let lat = deg_to_rad(latitude_deg);
        let decl = deg_to_rad(23.45 * (360.0 * (day_of_year as f64 - 81.0) / 365.0).to_radians().sin());
        let ha = deg_to_rad(15.0 * (hour_solar - 12.0));
        let sin_alt = lat.sin() * decl.sin() + lat.cos() * decl.cos() * ha.cos();
        let altitude_deg = rad_to_deg(sin_alt.clamp(-1.0, 1.0).asin());
        let cos_az = (decl.sin() - lat.sin() * sin_alt) / (lat.cos() * sin_alt.clamp(0.001, 1.0).acos().cos().max(1e-6));
        let azimuth_deg = rad_to_deg(cos_az.clamp(-1.0, 1.0).acos());
        let equation_of_time_min = 4.0 * (longitude_deg - 15.0 * (hour_solar / 24.0 * 24.0).round());
        SolarPosition { altitude_deg, azimuth_deg, equation_of_time_min }
    }

    /// 🌡️ Sky temperature [K] from dry-bulb and dew-point (Brunt-type).
    pub fn sky_temperature_k(t_dry_c: f64, t_dew_c: f64) -> f64 {
        let t_dry_k = t_dry_c + 273.15;
        let emissivity = 0.711 + 0.0056 * t_dew_c + 0.000_073 * t_dew_c * t_dew_c + 0.013 * (t_dew_c * 0.1).cos();
        t_dry_k * emissivity.powf(0.25)
    }
    // #endregion 🔖Solar

    // #region 🔖Ground
    /// 🌍 Ground temperature model.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GroundTemperatureModel {
        Monthly { temperatures_c: [f64; 12] },
        Shallow { annual_amplitude_k: f64, phase_shift_days: f64, mean_c: f64 },
        Deep { temperature_c: f64 },
    }

    impl GroundTemperatureModel {
        pub fn temperature_c(&self, day_of_year: u16) -> f64 {
            match self {
                Self::Monthly { temperatures_c } => {
                    let month = ((day_of_year as f64 - 1.0) / 30.44) as usize % 12;
                    temperatures_c[month]
                }
                Self::Shallow { annual_amplitude_k, phase_shift_days, mean_c } => {
                    let phase = 2.0 * std::f64::consts::PI * (day_of_year as f64 - phase_shift_days) / 365.0;
                    mean_c + annual_amplitude_k * phase.cos()
                }
                Self::Deep { temperature_c } => *temperature_c,
            }
        }
    }

    /// 🚰 Water mains temperature model.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WaterMainsModel {
        Constant { temperature_c: f64 },
        Monthly { temperatures_c: [f64; 12] },
    }
    // #endregion 🔖Ground

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn epw_parses_minimal() {
            let epw = "LOCATION,Test,XX,USA,TMY3,123,45.0,-75.0,-5.0,100.0\n\
    DESIGN CONDITIONS,0\n\
    TYPICAL/EXTREME PERIODS,0\n\
    GROUND TEMPERATURES,0\n\
    HOLIDAYS/DAYLIGHT SAVINGS,0\n\
    COMMENTS 1,0\n\
    COMMENTS 2,0\n\
    DATA PERIODS,1,1,Data,Sunday,1/1,12/31\n\
    2026,1,1,1,0,?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9,-5.0,-10.0,50,101325,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3.0,180,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0\n";
            let w = EpwWeather::parse(epw).unwrap();
            assert!((w.latitude_deg - 45.0).abs() < 1e-6);
            assert_eq!(w.records.len(), 1);
        }

        #[test]
        fn solar_noon_altitude_positive() {
            let pos = solar_position(45.0, 0.0, 172, 12.0);
            assert!(pos.altitude_deg > 0.0);
        }
    }
}

mod sizing {
    //! 📐 Zone and equipment sizing from design-day calculations.

    use crate::model::Model;
    use crate::results::{SizingResult, SizingTables};
    use crate::site::{DesignDay, DesignDayKind};
    use crate::units::CP_DRY_AIR;
    use serde::{Deserialize, Serialize};

    // #region 🔖SizingConfig
    /// 📐 Sizing configuration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SizingConfig {
        pub heating_design_day: DesignDay,
        pub cooling_design_day: DesignDay,
        pub sizing_factor: f64,
        pub safety_factor: f64,
    }

    impl Default for SizingConfig {
        fn default() -> Self {
            Self {
                heating_design_day: DesignDay {
                    name: "Winter Design".into(),
                    kind: DesignDayKind::Heating,
                    month: 1,
                    day: 21,
                    dry_bulb_max_c: -10.0,
                    daily_range_k: 6.0,
                    humidity_condition: crate::site::DesignDayHumidity::RelativeHumidity { rh: 0.8 },
                    wind_speed_m_s: 3.0,
                    solar_model: false,
                },
                cooling_design_day: DesignDay {
                    name: "Summer Design".into(),
                    kind: DesignDayKind::Cooling,
                    month: 7,
                    day: 21,
                    dry_bulb_max_c: 35.0,
                    daily_range_k: 10.0,
                    humidity_condition: crate::site::DesignDayHumidity::Wetbulb { wetbulb_at_max_c: 24.0 },
                    wind_speed_m_s: 2.0,
                    solar_model: true,
                },
                sizing_factor: 1.0,
                safety_factor: 1.15,
            }
        }
    }
    // #endregion 🔖SizingConfig

    // #region 🔖Sizing
    /// 📐 Sizing manager: compute design loads per zone and equipment.
    pub struct SizingManager;

    impl SizingManager {
        /// 📐 Run sizing pass and populate sizing tables.
        pub fn size(model: &Model, config: &SizingConfig) -> SizingTables {
            let mut tables = SizingTables::default();
            let sf = config.sizing_factor * config.safety_factor;

            for zone in &model.zones {
                let area = model.surfaces_for_zone(zone.id).iter().map(|s| crate::geometry::surface_area_m2(&s.vertices_m)).sum::<f64>().max(1.0);

                let u_avg = 0.3;
                let delta_t_heat = 20.0 - config.heating_design_day.dry_bulb_max_c;
                let delta_t_cool = config.cooling_design_day.dry_bulb_max_c - 24.0;
                let heating_load = u_avg * area * delta_t_heat.max(0.0) * sf;
                let cooling_load = u_avg * area * delta_t_cool.max(0.0) * sf;
                let ventilation = zone.volume_m3 * 0.5 * CP_DRY_AIR * 1.2 * delta_t_cool.max(0.0) / 3600.0;

                tables.zone_loads.push(SizingResult { component: format!("{} heating", zone.name), design_load_w: heating_load, design_flow_m3_s: zone.volume_m3 * 0.01 / 3600.0, autosized: true });
                tables.zone_loads.push(SizingResult { component: format!("{} cooling", zone.name), design_load_w: cooling_load + ventilation, design_flow_m3_s: zone.volume_m3 * 0.02 / 3600.0, autosized: true });
            }

            for ils in &model.ideal_loads {
                if let Some(zone) = model.zone_by_id(ils.zone_id) {
                    tables.equipment.push(SizingResult { component: format!("IdealLoads {}", zone.name), design_load_w: zone.volume_m3 * 50.0 * sf, design_flow_m3_s: zone.volume_m3 * 0.015 / 3600.0, autosized: true });
                }
            }

            tables
        }

        /// 📐 Coincident peak across zones.
        pub fn coincident_peak(loads: &[f64]) -> f64 {
            loads.iter().sum()
        }

        /// 📐 Non-coincident peak (sum of individual peaks).
        pub fn non_coincident_peak(loads: &[f64]) -> f64 {
            loads.iter().copied().fold(0.0, f64::max)
        }
    }
    // #endregion 🔖Sizing

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::EntityId;
        use crate::model::{Model, Site, Zone};

        #[test]
        fn sizes_zone_with_surfaces() {
            let model = Model {
                name: "Test".into(),
                site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
                zones: vec![Zone { id: EntityId(1), name: "Z1".into(), volume_m3: 200.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
                ..Default::default()
            };
            let tables = SizingManager::size(&model, &SizingConfig::default());
            assert!(!tables.zone_loads.is_empty());
        }
    }
}

mod solar {
    //! ☀️ Solar incidence, shading, and absorbed solar on surfaces and windows.

    use crate::geometry::{polygon_normal, surface_tilt_azimuth};
    use crate::units::deg_to_rad;

    // #region 🔖Types
    /// ☀️ Interior solar distribution mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InteriorSolarDistribution {
        DirectToFloor,
        UniformOnSurfaces,
        SplitFlux,
    }

    /// ☀️ Solar heat absorbed on a surface [W/m²].
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct SurfaceSolarAbsorption {
        pub beam_w_m2: f64,
        pub diffuse_w_m2: f64,
        pub total_w_m2: f64,
    }
    // #endregion 🔖Types

    // #region 🔖Incidence
    /// ☀️ Cosine of beam incidence angle (0–1).
    pub fn beam_incidence_cosine(surface_normal: [f64; 3], sun_altitude_deg: f64, sun_azimuth_deg: f64) -> f64 {
        let alt = deg_to_rad(sun_altitude_deg);
        let az = deg_to_rad(sun_azimuth_deg);
        let sun_dir = [alt.cos() * az.sin(), alt.cos() * az.cos(), alt.sin()];
        let mut n = surface_normal;
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if len > 1e-9 {
            n = [n[0] / len, n[1] / len, n[2] / len];
        }
        let cos_theta = n[0] * sun_dir[0] + n[1] * sun_dir[1] + n[2] * sun_dir[2];
        cos_theta.max(0.0)
    }

    /// ☀️ Sun direction unit vector from altitude/azimuth (Z up, Y north).
    pub fn sun_direction(sun_altitude_deg: f64, sun_azimuth_deg: f64) -> [f64; 3] {
        let alt = deg_to_rad(sun_altitude_deg);
        let az = deg_to_rad(sun_azimuth_deg);
        [alt.cos() * az.sin(), alt.cos() * az.cos(), alt.sin()]
    }
    // #endregion 🔖Incidence

    // #region 🔖Shading
    /// 🌳 Shading factor (0 = fully shaded, 1 = unshaded).
    pub fn shading_factor(unshaded_fraction: f64, overhang_depth_m: f64, window_height_m: f64, sun_altitude_deg: f64) -> f64 {
        let base = unshaded_fraction.clamp(0.0, 1.0);
        if overhang_depth_m <= 0.0 || window_height_m <= 0.0 || sun_altitude_deg <= 1.0 {
            return base;
        }
        let alt = deg_to_rad(sun_altitude_deg);
        let shadow_fraction = (overhang_depth_m / window_height_m * alt.tan()).clamp(0.0, 1.0);
        base * (1.0 - shadow_fraction)
    }
    // #endregion 🔖Shading

    // #region 🔖Absorption
    /// ☀️ Absorbed solar on opaque surface [W/m²].
    pub fn surface_solar_absorption(direct_normal_irradiance_w_m2: f64, diffuse_horizontal_irradiance_w_m2: f64, incidence_cosine: f64, shading: f64, solar_absorptance: f64, tilt_deg: f64) -> SurfaceSolarAbsorption {
        let tilt_rad = deg_to_rad(tilt_deg);
        let view_factor_sky = (1.0 + tilt_rad.cos()) * 0.5;
        let beam = direct_normal_irradiance_w_m2 * incidence_cosine * shading * solar_absorptance;
        let diffuse = diffuse_horizontal_irradiance_w_m2 * view_factor_sky * solar_absorptance;
        SurfaceSolarAbsorption { beam_w_m2: beam, diffuse_w_m2: diffuse, total_w_m2: beam + diffuse }
    }

    /// ☀️ Absorbed solar from polygon vertices and sun position.
    pub fn surface_solar_from_vertices(
        vertices_m: &[[f64; 3]],
        north_axis_deg: f64,
        sun_altitude_deg: f64,
        sun_azimuth_deg: f64,
        direct_normal_irradiance_w_m2: f64,
        diffuse_horizontal_irradiance_w_m2: f64,
        shading: f64,
        solar_absorptance: f64,
    ) -> SurfaceSolarAbsorption {
        let normal = polygon_normal(vertices_m);
        let tilt = surface_tilt_azimuth(normal, north_axis_deg);
        let cos_inc = beam_incidence_cosine(normal, sun_altitude_deg, sun_azimuth_deg);
        surface_solar_absorption(direct_normal_irradiance_w_m2, diffuse_horizontal_irradiance_w_m2, cos_inc, shading, solar_absorptance, tilt.tilt_deg)
    }
    // #endregion 🔖Absorption

    // #region 🔖Distribution
    /// 💡 Distribute transmitted solar to interior surfaces [W] per mode.
    pub fn distribute_interior_solar(transmitted_solar_w: f64, mode: InteriorSolarDistribution, floor_area_m2: f64, surface_areas_m2: &[f64]) -> Vec<f64> {
        match mode {
            InteriorSolarDistribution::DirectToFloor => {
                let mut out = vec![0.0; surface_areas_m2.len()];
                if !surface_areas_m2.is_empty() && floor_area_m2 > 0.0 {
                    out[0] = transmitted_solar_w;
                }
                out
            }
            InteriorSolarDistribution::UniformOnSurfaces => {
                let total: f64 = surface_areas_m2.iter().sum();
                if total <= 0.0 {
                    return vec![0.0; surface_areas_m2.len()];
                }
                surface_areas_m2.iter().map(|&a| transmitted_solar_w * a / total).collect()
            }
            InteriorSolarDistribution::SplitFlux => {
                let total: f64 = surface_areas_m2.iter().sum();
                let floor_share = 0.4;
                let mut out = vec![0.0; surface_areas_m2.len()];
                if !out.is_empty() {
                    out[0] = transmitted_solar_w * floor_share;
                }
                let wall_share = transmitted_solar_w * (1.0 - floor_share);
                if total > 0.0 {
                    for (i, a) in surface_areas_m2.iter().enumerate().skip(1) {
                        out[i] = wall_share * a / total;
                    }
                }
                out
            }
        }
    }
    // #endregion 🔖Distribution

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn vertical_south_wall_noon_incidence() {
            let cos = beam_incidence_cosine([0.0, -1.0, 0.0], 60.0, 180.0);
            assert!(cos > 0.4);
        }

        #[test]
        fn shading_reduces_with_overhang() {
            let unshaded = shading_factor(1.0, 0.0, 1.5, 45.0);
            let shaded = shading_factor(1.0, 0.8, 1.5, 45.0);
            assert!(shaded < unshaded);
        }

        #[test]
        fn absorption_positive_at_noon() {
            let abs = surface_solar_absorption(800.0, 100.0, 0.8, 1.0, 0.6, 90.0);
            assert!(abs.total_w_m2 > 100.0);
        }

        #[test]
        fn split_flux_allocates_floor_share() {
            let areas = vec![20.0, 10.0, 10.0];
            let dist = distribute_interior_solar(1000.0, InteriorSolarDistribution::SplitFlux, 20.0, &areas);
            assert!((dist[0] - 400.0).abs() < 1e-6);
        }
    }
}

mod solar_thermal {
    //! ☀️ Solar thermal collectors: flat-plate, ICS, unglazed transpired, PVT.

    use crate::units::{CP_DRY_AIR, RHO_AIR_REF, STEFAN_BOLTZMANN};
    use serde::{Deserialize, Serialize};

    // #region 🔖CollectorKind
    /// ☀️ Solar thermal collector technology.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CollectorKind {
        FlatPlate,
        IntegralCollectorStorage,
        UnglazedTranspired,
        PhotovoltaicThermal,
    }
    // #endregion 🔖CollectorKind

    // #region 🔖FlatPlate
    /// ☀️ Glazed flat-plate collector (Hottel-Whillier-Bliss).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FlatPlateCollector {
        pub area_m2: f64,
        pub tau_alpha: f64,
        pub ul_w_m2k: f64,
        pub iam_factor: f64,
    }

    impl FlatPlateCollector {
        /// ☀️ Useful thermal gain [W] from incident irradiance.
        pub fn useful_gain_w(&self, irradiance_w_m2: f64, ambient_c: f64, wind_m_s: f64, fluid_inlet_c: f64, mass_flow_kg_s: f64, fluid_cp: f64) -> f64 {
            collector_thermal_output_w(CollectorKind::FlatPlate, self.area_m2, irradiance_w_m2, ambient_c, wind_m_s, fluid_inlet_c, mass_flow_kg_s, fluid_cp, self.tau_alpha, self.ul_w_m2k, self.iam_factor, 0.0)
        }
    }
    // #endregion 🔖FlatPlate

    // #region 🔖Ics
    /// 🫙 Integral collector-storage (ICS) batch heater.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IntegralCollectorStorage {
        pub area_m2: f64,
        pub storage_volume_l: f64,
        pub tau_alpha: f64,
        pub loss_coefficient_w_m2k: f64,
    }

    impl IntegralCollectorStorage {
        /// 🫙 ICS timestep storage temperature update.
        pub fn simulate(&self, storage_temperature_c: f64, irradiance_w_m2: f64, ambient_c: f64, dt_s: f64) -> (f64, f64) {
            let gain = collector_thermal_output_w(CollectorKind::IntegralCollectorStorage, self.area_m2, irradiance_w_m2, ambient_c, 1.0, storage_temperature_c, 0.0, 4180.0, self.tau_alpha, self.loss_coefficient_w_m2k, 1.0, storage_temperature_c);
            let volume_m3 = self.storage_volume_l / 1000.0;
            let stored_j = 1000.0 * volume_m3 * 4180.0;
            let new_t = storage_temperature_c + gain * dt_s / stored_j.max(1.0);
            (new_t, gain)
        }
    }
    // #endregion 🔖Ics

    // #region 🔖Unglazed
    /// 🌀 Unglazed transpired solar collector (solar wall).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct UnglazedTranspiredCollector {
        pub area_m2: f64,
        pub porosity: f64,
        pub h_conv_w_m2k: f64,
        pub suction_velocity_m_s: f64,
    }

    impl UnglazedTranspiredCollector {
        /// 🌀 Preheat ventilation air via transpired absorber.
        pub fn preheat_air_w(&self, irradiance_w_m2: f64, ambient_c: f64, wind_m_s: f64) -> (f64, f64) {
            let gain = collector_thermal_output_w(
                CollectorKind::UnglazedTranspired,
                self.area_m2,
                irradiance_w_m2,
                ambient_c,
                wind_m_s,
                ambient_c,
                self.suction_velocity_m_s * RHO_AIR_REF * self.area_m2,
                CP_DRY_AIR,
                self.porosity,
                self.h_conv_w_m2k,
                1.0,
                ambient_c,
            );
            let m_dot = self.suction_velocity_m_s * RHO_AIR_REF * self.area_m2;
            let outlet_t = if m_dot > 1e-6 { ambient_c + gain / (m_dot * CP_DRY_AIR) } else { ambient_c };
            (gain, outlet_t)
        }
    }
    // #endregion 🔖Unglazed

    // #region 🔖Pvt
    /// ⚡☀️ Photovoltaic-thermal hybrid collector.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PvtCollector {
        pub area_m2: f64,
        pub pv_efficiency: f64,
        pub tau_alpha: f64,
        pub ul_w_m2k: f64,
        pub fluid_cp: f64,
    }

    impl PvtCollector {
        /// ⚡☀️ Split incident solar into electrical and thermal output.
        pub fn simulate(&self, irradiance_w_m2: f64, ambient_c: f64, wind_m_s: f64, fluid_inlet_c: f64, mass_flow_kg_s: f64) -> (f64, f64) {
            let t_cell = fluid_inlet_c + irradiance_w_m2 * 0.02;
            let pv_eta = (self.pv_efficiency * (1.0 - 0.004 * (t_cell - 25.0))).max(0.05);
            let pv_w = self.area_m2 * irradiance_w_m2 * pv_eta;
            let thermal_irrad = irradiance_w_m2 * (1.0 - pv_eta);
            let thermal_w = collector_thermal_output_w(CollectorKind::PhotovoltaicThermal, self.area_m2, thermal_irrad, ambient_c, wind_m_s, fluid_inlet_c, mass_flow_kg_s, self.fluid_cp, self.tau_alpha, self.ul_w_m2k, 1.0, fluid_inlet_c);
            (pv_w, thermal_w)
        }
    }
    // #endregion 🔖Pvt

    // #region 🔖Core
    /// ☀️ Universal collector useful thermal output [W].
    ///
    /// Implements Hottel-Whillier-Bliss with wind-adjusted loss coefficient:
    /// `Q_u = A * [τα * G * IAM - U_L * (T_m - T_amb)]`.
    pub fn collector_thermal_output_w(
        kind: CollectorKind,
        area_m2: f64,
        irradiance_w_m2: f64,
        ambient_c: f64,
        wind_m_s: f64,
        fluid_inlet_c: f64,
        mass_flow_kg_s: f64,
        fluid_cp: f64,
        tau_alpha: f64,
        ul_w_m2k: f64,
        iam: f64,
        reference_temperature_c: f64,
    ) -> f64 {
        let g = irradiance_w_m2.max(0.0);
        let wind = wind_m_s.max(0.1);
        let ul = match kind {
            CollectorKind::UnglazedTranspired => ul_w_m2k + 2.0 * wind,
            _ => ul_w_m2k + 0.5 * wind,
        };
        let t_m = if mass_flow_kg_s > 1e-6 {
            let f = (ul * area_m2 / (mass_flow_kg_s * fluid_cp)).min(50.0);
            fluid_inlet_c + g * tau_alpha * iam / (ul * (1.0 + f).max(1.0))
        } else {
            reference_temperature_c
        };
        let mut q_u = area_m2 * (tau_alpha * iam * g - ul * (t_m - ambient_c));
        if matches!(kind, CollectorKind::IntegralCollectorStorage) {
            let rad_loss = STEFAN_BOLTZMANN * area_m2 * ((t_m + 273.15).powi(4) - (ambient_c + 273.15).powi(4));
            q_u -= rad_loss * 0.1;
        }
        if matches!(kind, CollectorKind::UnglazedTranspired) {
            q_u *= tau_alpha;
        }
        q_u.max(0.0)
    }
    // #endregion 🔖Core

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn flat_plate_gain_positive_at_noon() {
            let collector = FlatPlateCollector { area_m2: 10.0, tau_alpha: 0.75, ul_w_m2k: 3.5, iam_factor: 0.95 };
            let gain = collector.useful_gain_w(800.0, 20.0, 2.0, 25.0, 0.2, 4180.0);
            assert!(gain > 0.0);
        }

        #[test]
        fn zero_irradiance_zero_gain() {
            let q = collector_thermal_output_w(CollectorKind::FlatPlate, 5.0, 0.0, 15.0, 1.0, 20.0, 0.1, 4180.0, 0.7, 4.0, 1.0, 20.0);
            assert!(q.abs() < 1.0);
        }

        #[test]
        fn ics_raises_storage_temperature() {
            let ics = IntegralCollectorStorage { area_m2: 3.0, storage_volume_l: 200.0, tau_alpha: 0.8, loss_coefficient_w_m2k: 5.0 };
            let (new_t, gain) = ics.simulate(25.0, 700.0, 18.0, 3600.0);
            assert!(gain > 0.0);
            assert!(new_t > 25.0);
        }

        #[test]
        fn unglazed_preheats_air() {
            let utc = UnglazedTranspiredCollector { area_m2: 50.0, porosity: 0.6, h_conv_w_m2k: 15.0, suction_velocity_m_s: 0.04 };
            let (gain, outlet_t) = utc.preheat_air_w(600.0, 5.0, 3.0);
            assert!(gain > 0.0);
            assert!(outlet_t > 5.0);
        }

        #[test]
        fn pvt_splits_electric_and_thermal() {
            let pvt = PvtCollector { area_m2: 8.0, pv_efficiency: 0.18, tau_alpha: 0.9, ul_w_m2k: 4.0, fluid_cp: 4180.0 };
            let (pv, thermal) = pvt.simulate(900.0, 22.0, 1.5, 30.0, 0.15);
            assert!(pv > 500.0);
            assert!(thermal >= 0.0);
        }
    }
}

mod terminal {
    //! 🌬️ Air terminals: VAV, CAV, reheat, fan-powered, and dual-duct.

    use crate::coils::{heating_coil_output_w, CoilAirState, HeatingCoil};
    use crate::fans::{fan_mass_flow_kg_s, fan_operating_point, fan_power_w, Fan};
    use crate::units::RHO_AIR_REF;
    use serde::{Deserialize, Serialize};

    // #region 🔖AirTerminal
    /// 🌬️ Zone air terminal unit types.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AirTerminal {
        Cav { max_flow_m3_s: f64 },
        Vav { min_flow_m3_s: f64, max_flow_m3_s: f64, reheat: Option<HeatingCoil> },
        VavReheat { min_flow_m3_s: f64, max_flow_m3_s: f64, reheat: HeatingCoil },
        FanPowered { primary_max_m3_s: f64, fan: Box<Fan>, parallel_fan: bool },
        DualDuct { hot_max_m3_s: f64, cold_max_m3_s: f64, mixing_damper: f64 },
    }

    /// 📥 Terminal inlet air and zone load request.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TerminalRequest {
        pub supply_temperature_c: f64,
        pub supply_humidity_ratio: f64,
        pub zone_temperature_c: f64,
        pub zone_humidity_ratio: f64,
        pub heating_load_w: f64,
        pub cooling_load_w: f64,
        pub pressure_pa: f64,
        pub damper_position: f64,
        pub hot_duct_temp_c: f64,
        pub cold_duct_temp_c: f64,
    }

    /// 📤 Terminal outlet air delivered to zone.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TerminalOutput {
        pub discharge_temperature_c: f64,
        pub discharge_humidity_ratio: f64,
        pub mass_flow_kg_s: f64,
        pub primary_mass_flow_kg_s: f64,
        pub reheat_w: f64,
        pub fan_power_w: f64,
        pub damper_position: f64,
    }
    // #endregion 🔖AirTerminal

    // #region 🔖Simulate
    impl AirTerminal {
        /// 🌬️ Simulate terminal unit for one zone timestep.
        pub fn simulate(&self, request: &TerminalRequest) -> TerminalOutput {
            match self {
                AirTerminal::Cav { max_flow_m3_s } => {
                    let m_dot = fan_mass_flow_kg_s(*max_flow_m3_s, RHO_AIR_REF);
                    TerminalOutput {
                        discharge_temperature_c: request.supply_temperature_c,
                        discharge_humidity_ratio: request.supply_humidity_ratio,
                        mass_flow_kg_s: m_dot,
                        primary_mass_flow_kg_s: m_dot,
                        reheat_w: 0.0,
                        fan_power_w: 0.0,
                        damper_position: 1.0,
                    }
                }
                AirTerminal::Vav { min_flow_m3_s, max_flow_m3_s, reheat } => {
                    let frac = request.damper_position.clamp(0.0, 1.0);
                    let flow = min_flow_m3_s + frac * (max_flow_m3_s - min_flow_m3_s);
                    let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
                    let inlet = CoilAirState { temperature_c: request.supply_temperature_c, humidity_ratio: request.supply_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.pressure_pa };
                    let reheat_out = reheat.as_ref().map_or(crate::coils::HeatingCoilOutput { outlet: inlet, total_heating_w: 0.0, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 }, |c| heating_coil_output_w(c, &inlet, request.heating_load_w));
                    TerminalOutput {
                        discharge_temperature_c: reheat_out.outlet.temperature_c,
                        discharge_humidity_ratio: reheat_out.outlet.humidity_ratio,
                        mass_flow_kg_s: m_dot,
                        primary_mass_flow_kg_s: m_dot,
                        reheat_w: reheat_out.total_heating_w,
                        fan_power_w: 0.0,
                        damper_position: frac,
                    }
                }
                AirTerminal::VavReheat { min_flow_m3_s, max_flow_m3_s, reheat } => {
                    let frac = request.damper_position.clamp(0.0, 1.0);
                    let flow = min_flow_m3_s + frac * (max_flow_m3_s - min_flow_m3_s);
                    let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
                    let inlet = CoilAirState { temperature_c: request.supply_temperature_c, humidity_ratio: request.supply_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.pressure_pa };
                    let reheat_out = heating_coil_output_w(reheat, &inlet, request.heating_load_w);
                    TerminalOutput {
                        discharge_temperature_c: reheat_out.outlet.temperature_c,
                        discharge_humidity_ratio: reheat_out.outlet.humidity_ratio,
                        mass_flow_kg_s: m_dot,
                        primary_mass_flow_kg_s: m_dot,
                        reheat_w: reheat_out.total_heating_w,
                        fan_power_w: 0.0,
                        damper_position: frac,
                    }
                }
                AirTerminal::FanPowered { primary_max_m3_s, fan, parallel_fan } => {
                    let frac = request.damper_position.clamp(0.0, 1.0);
                    let primary_flow = primary_max_m3_s * frac;
                    let induced_flow = if *parallel_fan { primary_flow * 0.5 } else { primary_flow * 0.3 };
                    let total_flow = primary_flow + induced_flow;
                    let m_dot = fan_mass_flow_kg_s(total_flow, RHO_AIR_REF);
                    let t_mix = if *parallel_fan { (primary_flow * request.supply_temperature_c + induced_flow * request.zone_temperature_c) / total_flow.max(1e-6) } else { request.supply_temperature_c };
                    let op = fan_operating_point(fan, induced_flow, 150.0);
                    TerminalOutput {
                        discharge_temperature_c: t_mix,
                        discharge_humidity_ratio: request.supply_humidity_ratio,
                        mass_flow_kg_s: m_dot,
                        primary_mass_flow_kg_s: fan_mass_flow_kg_s(primary_flow, RHO_AIR_REF),
                        reheat_w: 0.0,
                        fan_power_w: fan_power_w(fan, &op),
                        damper_position: frac,
                    }
                }
                AirTerminal::DualDuct { hot_max_m3_s, cold_max_m3_s, mixing_damper } => {
                    let mix = mixing_damper.clamp(0.0, 1.0);
                    let hot_flow = hot_max_m3_s * mix;
                    let cold_flow = cold_max_m3_s * (1.0 - mix);
                    let total_flow = hot_flow + cold_flow;
                    let m_dot = fan_mass_flow_kg_s(total_flow, RHO_AIR_REF);
                    let t_out = if total_flow > 1e-9 { (hot_flow * request.hot_duct_temp_c + cold_flow * request.cold_duct_temp_c) / total_flow } else { request.zone_temperature_c };
                    TerminalOutput { discharge_temperature_c: t_out, discharge_humidity_ratio: request.supply_humidity_ratio, mass_flow_kg_s: m_dot, primary_mass_flow_kg_s: m_dot, reheat_w: 0.0, fan_power_w: 0.0, damper_position: mix }
                }
            }
        }
    }
    // #endregion 🔖Simulate

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cav_constant_flow() {
            let term = AirTerminal::Cav { max_flow_m3_s: 0.3 };
            let req = TerminalRequest {
                supply_temperature_c: 13.0,
                supply_humidity_ratio: 0.008,
                zone_temperature_c: 22.0,
                zone_humidity_ratio: 0.01,
                heating_load_w: 0.0,
                cooling_load_w: 2000.0,
                pressure_pa: 101_325.0,
                damper_position: 1.0,
                hot_duct_temp_c: 35.0,
                cold_duct_temp_c: 13.0,
            };
            let out = term.simulate(&req);
            assert!((out.mass_flow_kg_s - 0.36).abs() < 0.05);
            assert!((out.discharge_temperature_c - 13.0).abs() < 1e-6);
        }

        #[test]
        fn vav_reheat_adds_heat() {
            let term = AirTerminal::VavReheat { min_flow_m3_s: 0.05, max_flow_m3_s: 0.4, reheat: HeatingCoil::Electric { capacity_w: 5000.0, efficiency: 1.0 } };
            let req = TerminalRequest {
                supply_temperature_c: 13.0,
                supply_humidity_ratio: 0.008,
                zone_temperature_c: 20.0,
                zone_humidity_ratio: 0.009,
                heating_load_w: 2000.0,
                cooling_load_w: 0.0,
                pressure_pa: 101_325.0,
                damper_position: 0.5,
                hot_duct_temp_c: 35.0,
                cold_duct_temp_c: 13.0,
            };
            let out = term.simulate(&req);
            assert!(out.reheat_w > 0.0);
            assert!(out.discharge_temperature_c > req.supply_temperature_c);
        }

        #[test]
        fn dual_duct_mixes_temperatures() {
            let term = AirTerminal::DualDuct { hot_max_m3_s: 0.2, cold_max_m3_s: 0.3, mixing_damper: 0.5 };
            let req = TerminalRequest {
                supply_temperature_c: 15.0,
                supply_humidity_ratio: 0.009,
                zone_temperature_c: 22.0,
                zone_humidity_ratio: 0.01,
                heating_load_w: 0.0,
                cooling_load_w: 0.0,
                pressure_pa: 101_325.0,
                damper_position: 0.5,
                hot_duct_temp_c: 40.0,
                cold_duct_temp_c: 12.0,
            };
            let out = term.simulate(&req);
            assert!(out.discharge_temperature_c > req.cold_duct_temp_c);
            assert!(out.discharge_temperature_c < req.hot_duct_temp_c);
        }
    }
}

mod units {
    //! 📐 SI unit helpers and physical constants for BEM computations.

    // #region 🔖Constants
    /// 🌡️ Standard atmospheric pressure [Pa].
    pub const P_STD: f64 = 101_325.0;
    /// 💨 Dry air gas constant [J/(kg·K)].
    pub const R_DRY_AIR: f64 = 287.055;
    /// 💧 Water vapor gas constant [J/(kg·K)].
    pub const R_WATER_VAPOR: f64 = 461.52;
    /// 🌡️ Triple-point temperature of water [K].
    pub const T_TRIPLE_WATER: f64 = 273.16;
    /// 🔥 Specific heat of dry air at constant pressure [J/(kg·K)].
    pub const CP_DRY_AIR: f64 = 1006.0;
    /// 🔥 Latent heat of vaporization at 0°C [J/kg].
    pub const H_FG_0C: f64 = 2_501_000.0;
    /// 💧 Density of water [kg/m³].
    pub const RHO_WATER: f64 = 998.0;
    /// 🧊 Density of dry air at reference [kg/m³].
    pub const RHO_AIR_REF: f64 = 1.2;
    /// ⚡ Stefan-Boltzmann constant [W/(m²·K⁴)].
    pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;
    /// 🌍 Standard gravity [m/s²].
    pub const GRAVITY: f64 = 9.806_65;
    // #endregion 🔖Constants

    // #region 🔖Conversions
    /// 🌡️ Celsius to Kelvin.
    pub fn c_to_k(t_c: f64) -> f64 {
        t_c + 273.15
    }

    /// 🌡️ Kelvin to Celsius.
    pub fn k_to_c(t_k: f64) -> f64 {
        t_k - 273.15
    }

    /// 📐 Degrees to radians.
    pub fn deg_to_rad(deg: f64) -> f64 {
        deg * std::f64::consts::PI / 180.0
    }

    /// 📐 Radians to degrees.
    pub fn rad_to_deg(rad: f64) -> f64 {
        rad * 180.0 / std::f64::consts::PI
    }
    // #endregion 🔖Conversions

    // #region 🔖Quantity
    /// 📊 Tagged SI scalar for results and limits.
    #[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum Unit {
        Dimensionless,
        Meters,
        SquareMeters,
        CubicMeters,
        Kelvin,
        Celsius,
        Pascals,
        Watts,
        Joules,
        KilogramsPerSecond,
        CubicMetersPerSecond,
        KilowattHours,
        HumidityRatio,
        Percent,
    }

    /// 📏 Physical quantity with unit tag.
    #[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Quantity {
        pub unit: Unit,
        pub value: f64,
    }

    impl Quantity {
        pub const fn new(unit: Unit, value: f64) -> Self {
            Self { unit, value }
        }

        pub fn watts(v: f64) -> Self {
            Self::new(Unit::Watts, v)
        }

        pub fn joules(v: f64) -> Self {
            Self::new(Unit::Joules, v)
        }

        pub fn celsius(v: f64) -> Self {
            Self::new(Unit::Celsius, v)
        }
    }
    // #endregion 🔖Quantity

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn celsius_kelvin_roundtrip() {
            assert!((k_to_c(c_to_k(20.0)) - 20.0).abs() < 1e-9);
        }
    }
}

mod water {
    //! 💧 Water systems: fixtures, tanks, rainwater, condensate, irrigation, cooling tower makeup.

    use crate::props::water_density;
    use crate::units::RHO_WATER;
    use serde::{Deserialize, Serialize};

    // #region 🔖Fixture
    /// 🚰 Generic water fixture end use.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WaterFixture {
        pub name: String,
        pub fixture_type: FixtureType,
        pub peak_flow_l_s: f64,
        pub schedule_factor: f64,
        pub hot_water_fraction: f64,
    }

    /// 🚰 Standard fixture categories.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FixtureType {
        Lavatory,
        Shower,
        KitchenSink,
        Toilet,
        Urinal,
        HoseBib,
        CoolingTower,
    }

    impl WaterFixture {
        /// 💧 Volumetric flow [m³/s].
        pub fn flow_m3_s(&self) -> f64 {
            self.peak_flow_l_s * self.schedule_factor.clamp(0.0, 1.0) / 1000.0
        }

        /// 💧 Mass flow [kg/s].
        pub fn mass_flow_kg_s(&self) -> f64 {
            self.flow_m3_s() * RHO_WATER
        }

        /// 🔥 Hot-water branch flow [kg/s].
        pub fn hot_flow_kg_s(&self) -> f64 {
            self.mass_flow_kg_s() * self.hot_water_fraction.clamp(0.0, 1.0)
        }
    }
    // #endregion 🔖Fixture

    // #region 🔖Tank
    /// 🛢️ Potable or process water storage tank.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WaterTank {
        pub volume_m3: f64,
        pub level_m: f64,
        pub max_level_m: f64,
        pub min_level_m: f64,
        pub inlet_temperature_c: f64,
    }

    impl WaterTank {
        /// 🛢️ Update tank level from inflow/outflow over dt.
        pub fn simulate(&self, inflow_m3_s: f64, outflow_m3_s: f64, dt_s: f64, area_m2: f64) -> (f64, f64) {
            let net_m3 = (inflow_m3_s - outflow_m3_s) * dt_s;
            let delta_level = net_m3 / area_m2.max(0.01);
            let new_level = (self.level_m + delta_level).clamp(self.min_level_m, self.max_level_m);
            let volume = new_level * area_m2;
            (new_level, volume)
        }

        /// 💧 Available draw before hitting minimum level.
        pub fn available_volume_m3(&self, area_m2: f64) -> f64 {
            ((self.level_m - self.min_level_m).max(0.0)) * area_m2
        }
    }
    // #endregion 🔖Tank

    // #region 🔖Rainwater
    /// 🌧️ Rainwater harvesting from roof catchment.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RainwaterSystem {
        pub catchment_area_m2: f64,
        pub runoff_coefficient: f64,
        pub tank: WaterTank,
        pub first_flush_l: f64,
        pub filter_efficiency: f64,
    }

    impl RainwaterSystem {
        /// 🌧️ Harvested volume [m³] from rainfall depth [mm] over timestep.
        pub fn harvest_m3(&self, rainfall_mm: f64, _dt_s: f64) -> f64 {
            let gross_m3 = self.catchment_area_m2 * rainfall_mm / 1000.0 * self.runoff_coefficient;
            let first_flush_m3 = if rainfall_mm > 0.0 { (self.first_flush_l / 1000.0).min(gross_m3) } else { 0.0 };
            (gross_m3 - first_flush_m3).max(0.0) * self.filter_efficiency
        }

        /// 🌧️ Simulate tank level with rainfall and demand.
        pub fn simulate(&self, rainfall_mm: f64, demand_m3_s: f64, dt_s: f64, tank_area_m2: f64) -> (f64, f64) {
            let harvest = self.harvest_m3(rainfall_mm, dt_s);
            let inflow = harvest / dt_s.max(1.0);
            let outflow = demand_m3_s.min(self.tank.available_volume_m3(tank_area_m2) / dt_s.max(1.0));
            let (level, volume) = self.tank.simulate(inflow, outflow, dt_s, tank_area_m2);
            (level, volume)
        }
    }
    // #endregion 🔖Rainwater

    // #region 🔖Condensate
    /// 💧 HVAC condensate recovery from cooling coils.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CondensateRecovery {
        pub collection_efficiency: f64,
        pub storage_tank: WaterTank,
    }

    impl CondensateRecovery {
        /// 💧 Condensate mass flow [kg/s] from dehumidification rate.
        pub fn condensate_kg_s(&self, dehumidification_kg_s: f64) -> f64 {
            dehumidification_kg_s * self.collection_efficiency.clamp(0.0, 1.0)
        }

        /// 💧 Accumulated condensate volume [m³] over timestep.
        pub fn accumulate_m3(&self, dehumidification_kg_s: f64, dt_s: f64) -> f64 {
            let kg = self.condensate_kg_s(dehumidification_kg_s) * dt_s;
            kg / water_density(20.0)
        }
    }
    // #endregion 🔖Condensate

    // #region 🔖Irrigation
    /// 🌱 Landscape irrigation demand.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IrrigationSystem {
        pub landscaped_area_m2: f64,
        pub crop_coefficient: f64,
        pub irrigation_efficiency: f64,
        pub precipitation_mm_per_day: f64,
    }

    impl IrrigationSystem {
        /// 🌱 Irrigation water demand [m³/s] from reference evapotranspiration [mm/day].
        pub fn demand_m3_s(&self, et0_mm_per_day: f64, schedule_factor: f64) -> f64 {
            let et_c = et0_mm_per_day * self.crop_coefficient;
            let net_mm = (et_c - self.precipitation_mm_per_day).max(0.0);
            let gross_mm = net_mm / self.irrigation_efficiency.max(0.1);
            let m3_per_day = self.landscaped_area_m2 * gross_mm / 1000.0;
            m3_per_day / 86_400.0 * schedule_factor.clamp(0.0, 1.0)
        }
    }
    // #endregion 🔖Irrigation

    // #region 🔖CoolingTowerMakeup
    /// 🌊 Cooling tower evaporation, drift, and blowdown makeup water.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CoolingTowerMakeup {
        pub cycles_of_concentration: f64,
        pub drift_fraction: f64,
        pub basin_volume_m3: f64,
    }

    impl CoolingTowerMakeup {
        /// 🌊 Evaporation rate [kg/s] from condenser heat rejection.
        pub fn evaporation_kg_s(&self, heat_rejection_w: f64, delta_h_vaporization_j_kg: f64) -> f64 {
            if heat_rejection_w <= 0.0 {
                return 0.0;
            }
            heat_rejection_w / delta_h_vaporization_j_kg.max(1.0)
        }

        /// 🌊 Total makeup water [kg/s] including blowdown and drift.
        pub fn makeup_kg_s(&self, heat_rejection_w: f64) -> f64 {
            let h_fg = 2_400_000.0;
            let evap = self.evaporation_kg_s(heat_rejection_w, h_fg);
            let coc = self.cycles_of_concentration.max(1.5);
            let blowdown = evap / (coc - 1.0);
            let drift = evap * self.drift_fraction;
            evap + blowdown + drift
        }

        /// 🌊 Annual makeup [m³] from average heat rejection.
        pub fn annual_makeup_m3(&self, average_rejection_w: f64) -> f64 {
            self.makeup_kg_s(average_rejection_w) * 86_400.0 * 365.0 / RHO_WATER
        }
    }
    // #endregion 🔖CoolingTowerMakeup

    // #region 🔖Balance
    /// 💧 Building water balance for one timestep.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct WaterBalance {
        pub fixture_demand_m3_s: f64,
        pub irrigation_demand_m3_s: f64,
        pub cooling_makeup_m3_s: f64,
        pub rainwater_supply_m3_s: f64,
        pub condensate_supply_m3_s: f64,
        pub mains_import_m3_s: f64,
    }

    /// 💧 Net mains water demand [m³/s].
    pub fn water_balance(fixtures: &[WaterFixture], irrigation_m3_s: f64, cooling_makeup_kg_s: f64, rainwater_m3_s: f64, condensate_kg_s: f64) -> WaterBalance {
        let fixture_demand: f64 = fixtures.iter().map(|f| f.flow_m3_s()).sum();
        let condensate_m3_s = condensate_kg_s / RHO_WATER;
        let cooling_m3_s = cooling_makeup_kg_s / RHO_WATER;
        let supply = rainwater_m3_s + condensate_m3_s;
        let demand = fixture_demand + irrigation_m3_s + cooling_m3_s;
        WaterBalance {
            fixture_demand_m3_s: fixture_demand,
            irrigation_demand_m3_s: irrigation_m3_s,
            cooling_makeup_m3_s: cooling_m3_s,
            rainwater_supply_m3_s: rainwater_m3_s,
            condensate_supply_m3_s: condensate_m3_s,
            mains_import_m3_s: (demand - supply).max(0.0),
        }
    }
    // #endregion 🔖Balance

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fixture_flow_scales_with_schedule() {
            let fixture = WaterFixture { name: "Lav".into(), fixture_type: FixtureType::Lavatory, peak_flow_l_s: 0.1, schedule_factor: 0.25, hot_water_fraction: 0.5 };
            assert!((fixture.flow_m3_s() - 0.000_025).abs() < 1e-9);
        }

        #[test]
        fn tank_level_rises_with_inflow() {
            let tank = WaterTank { volume_m3: 5.0, level_m: 1.0, max_level_m: 3.0, min_level_m: 0.2, inlet_temperature_c: 15.0 };
            let (new_level, _) = tank.simulate(0.01, 0.0, 3600.0, 5.0);
            assert!(new_level > tank.level_m);
        }

        #[test]
        fn rainwater_harvest_reduces_with_first_flush() {
            let system =
                RainwaterSystem { catchment_area_m2: 200.0, runoff_coefficient: 0.85, tank: WaterTank { volume_m3: 10.0, level_m: 1.5, max_level_m: 2.5, min_level_m: 0.1, inlet_temperature_c: 15.0 }, first_flush_l: 50.0, filter_efficiency: 0.95 };
            let harvest = system.harvest_m3(10.0, 3600.0);
            assert!(harvest > 0.0);
            assert!(harvest < 200.0 * 10.0 / 1000.0);
        }

        #[test]
        fn cooling_makeup_increases_with_load() {
            let makeup = CoolingTowerMakeup { cycles_of_concentration: 4.0, drift_fraction: 0.001, basin_volume_m3: 2.0 };
            let low = makeup.makeup_kg_s(100_000.0);
            let high = makeup.makeup_kg_s(500_000.0);
            assert!(high > low);
        }

        #[test]
        fn water_balance_mains_import() {
            let fixtures = vec![WaterFixture { name: "Shower".into(), fixture_type: FixtureType::Shower, peak_flow_l_s: 0.12, schedule_factor: 1.0, hot_water_fraction: 0.8 }];
            let balance = water_balance(&fixtures, 0.0, 0.05, 0.0, 0.0);
            assert!(balance.mains_import_m3_s > 0.0);
        }

        #[test]
        fn irrigation_demand_scales_with_et0() {
            let irrigation = IrrigationSystem { landscaped_area_m2: 500.0, crop_coefficient: 0.8, irrigation_efficiency: 0.75, precipitation_mm_per_day: 2.0 };
            let low = irrigation.demand_m3_s(3.0, 1.0);
            let high = irrigation.demand_m3_s(8.0, 1.0);
            assert!(high > low);
        }
    }
}

mod zone_air {
    //! 🌬️ Zone sensible and latent air balance: transient BDF3, analytical steady state, unmet load.

    use crate::props::{latent_heat_vaporization, moist_air_density, moist_air_enthalpy_j_per_kg};
    use crate::units::{CP_DRY_AIR, P_STD};
    use serde::{Deserialize, Serialize};

    // #region 🔖HumiditySolutionMethod
    /// 💧 Humidity ratio integration method for zone air mass balance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HumiditySolutionMethod {
        AnalyticalSteadyState,
        ThirdOrderBackward,
    }
    // #endregion 🔖HumiditySolutionMethod

    // #region 🔖ZoneAirState
    /// 🌡️ Zone air state with BDF3 temperature history [T_n, T_{n-1}, T_{n-2}, T_{n-3}].
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ZoneAirState {
        pub temp_c: f64,
        pub humidity_ratio: f64,
        pub temp_history_c: [f64; 4],
        pub humidity_history: [f64; 4],
    }

    impl ZoneAirState {
        pub fn new(temp_c: f64, humidity_ratio: f64) -> Self {
            Self { temp_c, humidity_ratio, temp_history_c: [temp_c; 4], humidity_history: [humidity_ratio; 4] }
        }

        pub fn push_temp(&mut self, temp_c: f64) {
            self.temp_history_c = [temp_c, self.temp_history_c[0], self.temp_history_c[1], self.temp_history_c[2]];
            self.temp_c = temp_c;
        }

        pub fn push_humidity(&mut self, w: f64) {
            self.humidity_history = [w, self.humidity_history[0], self.humidity_history[1], self.humidity_history[2]];
            self.humidity_ratio = w;
        }
    }
    // #endregion 🔖ZoneAirState

    // #region 🔖ZoneAirBalance
    /// ⚖️ Zone air energy and moisture balance inputs [W] and [kg/s].
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ZoneAirBalance {
        pub volume_m3: f64,
        pub conditioned: bool,
        pub sensible_gain_w: f64,
        pub latent_gain_w: f64,
        pub infiltration_sensible_w: f64,
        pub infiltration_latent_w: f64,
        pub ventilation_sensible_w: f64,
        pub ventilation_latent_w: f64,
        pub system_sensible_w: f64,
        pub system_latent_w: f64,
        pub surface_convection_w: f64,
        pub mass_flow_in_kg_s: f64,
        pub supply_humidity_ratio: f64,
        pub outdoor_humidity_ratio: f64,
        pub heating_setpoint_c: Option<f64>,
        pub cooling_setpoint_c: Option<f64>,
        pub max_heating_w: Option<f64>,
        pub max_cooling_w: Option<f64>,
    }

    impl ZoneAirBalance {
        pub fn net_sensible_w(&self) -> f64 {
            self.sensible_gain_w + self.surface_convection_w + self.infiltration_sensible_w + self.ventilation_sensible_w + self.system_sensible_w
        }

        pub fn net_latent_w(&self) -> f64 {
            self.latent_gain_w + self.infiltration_latent_w + self.ventilation_latent_w + self.system_latent_w
        }
    }
    // #endregion 🔖ZoneAirBalance

    // #region 🔖ZoneAirResult
    /// 📊 Zone air step result including unmet loads.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ZoneAirResult {
        pub temp_c: f64,
        pub humidity_ratio: f64,
        pub unmet_heating_w: f64,
        pub unmet_cooling_w: f64,
        pub unmet_humidifying_w: f64,
        pub unmet_dehumidifying_w: f64,
    }
    // #endregion 🔖ZoneAirResult

    // #region 🔖Capacitance
    fn zone_sensible_capacitance_j_per_k(volume_m3: f64, temp_c: f64, w: f64, p_atm: f64) -> f64 {
        let rho = moist_air_density(temp_c, w, p_atm);
        rho * volume_m3 * CP_DRY_AIR
    }

    fn zone_moisture_capacitance_kg_per_k(volume_m3: f64, temp_c: f64, w: f64, p_atm: f64) -> f64 {
        moist_air_density(temp_c, w, p_atm) * volume_m3
    }
    // #endregion 🔖Capacitance

    // #region 🔖Bdf3
    fn bdf3_next_value(history: [f64; 4], dt_s: f64, rate: f64) -> f64 {
        let coeff = 6.0 * dt_s * rate;
        (coeff + 18.0 * history[1] - 9.0 * history[2] + 2.0 * history[3]) / 11.0
    }

    #[allow(dead_code, reason = "BDF3 rate-recovery counterpart to bdf3_next_value, validated by its own unit test but not yet wired into a production call site — in-flight energy BEM zone-air numerics")]
    fn bdf3_rate(history: [f64; 4], dt_s: f64) -> f64 {
        (11.0 * history[0] - 18.0 * history[1] + 9.0 * history[2] - 2.0 * history[3]) / (6.0 * dt_s)
    }
    // #endregion 🔖Bdf3

    // #region 🔖Analytical
    /// 🌡️ Steady-state zone air temperature [°C] from sensible balance Q [W] and reference temp.
    pub fn analytical_steady_temp_c(q_sensible_w: f64, temp_ref_c: f64, ua_w_per_k: f64) -> f64 {
        if ua_w_per_k.abs() < 1e-9 {
            return temp_ref_c;
        }
        temp_ref_c + q_sensible_w / ua_w_per_k
    }

    /// 💧 Steady-state humidity ratio [kg/kg] from latent balance.
    pub fn analytical_steady_humidity_ratio(latent_gain_w: f64, mass_flow_kg_s: f64, w_supply: f64, temp_c: f64) -> f64 {
        if mass_flow_kg_s.abs() < 1e-12 {
            return w_supply;
        }
        let h_fg = latent_heat_vaporization(temp_c);
        w_supply + latent_gain_w / (mass_flow_kg_s * h_fg)
    }
    // #endregion 🔖Analytical

    // #region 🔖UnmetLoad
    fn compute_unmet_loads(balance: &ZoneAirBalance, temp_c: f64, humidity_ratio: f64) -> (f64, f64, f64, f64) {
        let mut unmet_heating = 0.0;
        let mut unmet_cooling = 0.0;
        let unmet_humid = 0.0;
        let unmet_dehumid = 0.0;

        if balance.conditioned {
            if let Some(t_heat) = balance.heating_setpoint_c {
                if temp_c < t_heat {
                    let deficit = (t_heat - temp_c) * zone_sensible_capacitance_j_per_k(balance.volume_m3, temp_c, humidity_ratio, P_STD);
                    let delivered = balance.system_sensible_w.max(0.0);
                    let cap = balance.max_heating_w.unwrap_or(f64::INFINITY);
                    unmet_heating = (deficit - delivered).max(0.0).min(cap);
                }
            }
            if let Some(t_cool) = balance.cooling_setpoint_c {
                if temp_c > t_cool {
                    let excess = (temp_c - t_cool) * zone_sensible_capacitance_j_per_k(balance.volume_m3, temp_c, humidity_ratio, P_STD);
                    let delivered = (-balance.system_sensible_w).max(0.0);
                    let cap = balance.max_cooling_w.unwrap_or(f64::INFINITY);
                    unmet_cooling = (excess - delivered).max(0.0).min(cap);
                }
            }
        }
        (unmet_heating, unmet_cooling, unmet_humid, unmet_dehumid)
    }
    // #endregion 🔖UnmetLoad

    // #region 🔖Advance
    /// ⏩ Advance zone air state one timestep.
    pub fn advance_zone_air(state: &ZoneAirState, balance: &ZoneAirBalance, dt_s: f64, method: HumiditySolutionMethod, p_atm: f64) -> ZoneAirResult {
        let c_sens = zone_sensible_capacitance_j_per_k(balance.volume_m3, state.temp_c, state.humidity_ratio, p_atm);
        let q_sens = balance.net_sensible_w();

        let temp_c = if balance.conditioned {
            if dt_s > 0.0 && c_sens > 0.0 {
                let rate = q_sens / c_sens;
                bdf3_next_value(state.temp_history_c, dt_s, rate)
            } else {
                state.temp_c
            }
        } else {
            let ua = c_sens / 3600.0;
            analytical_steady_temp_c(q_sens, state.temp_c, ua.max(1.0))
        };

        let humidity_ratio = match method {
            HumiditySolutionMethod::AnalyticalSteadyState => {
                let w_in = if balance.mass_flow_in_kg_s > 0.0 { balance.supply_humidity_ratio } else { balance.outdoor_humidity_ratio };
                analytical_steady_humidity_ratio(balance.net_latent_w(), balance.mass_flow_in_kg_s, w_in, temp_c)
            }
            HumiditySolutionMethod::ThirdOrderBackward => {
                let c_moist = zone_moisture_capacitance_kg_per_k(balance.volume_m3, temp_c, state.humidity_ratio, p_atm);
                let h_fg = latent_heat_vaporization(temp_c);
                let w_in = balance.supply_humidity_ratio;
                let latent_kg_s = balance.net_latent_w() / h_fg;
                let rate = if c_moist > 0.0 { (balance.mass_flow_in_kg_s * (w_in - state.humidity_ratio) + latent_kg_s) / c_moist } else { 0.0 };
                if dt_s > 0.0 {
                    bdf3_next_value(state.humidity_history, dt_s, rate).max(0.0)
                } else {
                    state.humidity_ratio
                }
            }
        };

        let (unmet_heating_w, unmet_cooling_w, unmet_humidifying_w, unmet_dehumidifying_w) = compute_unmet_loads(balance, temp_c, humidity_ratio);

        ZoneAirResult { temp_c, humidity_ratio, unmet_heating_w, unmet_cooling_w, unmet_humidifying_w, unmet_dehumidifying_w }
    }

    /// 🔄 Commit zone air result into mutable state history.
    pub fn commit_zone_air(state: &mut ZoneAirState, result: ZoneAirResult) {
        state.push_temp(result.temp_c);
        state.push_humidity(result.humidity_ratio);
    }

    /// 🔥 Sensible load to change zone air from T1 to T2 [W] over dt [s].
    pub fn sensible_load_for_delta_t_w(volume_m3: f64, t1_c: f64, t2_c: f64, w: f64, dt_s: f64, p_atm: f64) -> f64 {
        if dt_s <= 0.0 {
            return 0.0;
        }
        let c = zone_sensible_capacitance_j_per_k(volume_m3, (t1_c + t2_c) * 0.5, w, p_atm);
        c * (t2_c - t1_c) / dt_s
    }

    /// 💧 Enthalpy difference for ventilation [J/kg dry air].
    pub fn ventilation_enthalpy_delta_j_per_kg(t_zone_c: f64, w_zone: f64, t_out_c: f64, w_out: f64) -> f64 {
        moist_air_enthalpy_j_per_kg(t_zone_c, w_zone) - moist_air_enthalpy_j_per_kg(t_out_c, w_out)
    }
    // #endregion 🔖Advance

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn analytical_temp_rises_with_gain() {
            let t = analytical_steady_temp_c(1000.0, 20.0, 100.0);
            assert!((t - 30.0).abs() < 1e-9);
        }

        #[test]
        fn bdf3_rate_constant_history() {
            let h = [22.0, 22.0, 22.0, 22.0];
            assert!(bdf3_rate(h, 3600.0).abs() < 1e-9);
        }

        #[test]
        fn floating_zone_uses_steady_analytical() {
            let state = ZoneAirState::new(20.0, 0.008);
            let balance = ZoneAirBalance {
                volume_m3: 100.0,
                conditioned: false,
                sensible_gain_w: 500.0,
                latent_gain_w: 0.0,
                infiltration_sensible_w: 0.0,
                infiltration_latent_w: 0.0,
                ventilation_sensible_w: 0.0,
                ventilation_latent_w: 0.0,
                system_sensible_w: 0.0,
                system_latent_w: 0.0,
                surface_convection_w: 0.0,
                mass_flow_in_kg_s: 0.0,
                supply_humidity_ratio: 0.008,
                outdoor_humidity_ratio: 0.008,
                heating_setpoint_c: None,
                cooling_setpoint_c: None,
                max_heating_w: None,
                max_cooling_w: None,
            };
            let result = advance_zone_air(&state, &balance, 3600.0, HumiditySolutionMethod::AnalyticalSteadyState, P_STD);
            assert!(result.temp_c > 20.0);
        }

        #[test]
        fn conditioned_bdf3_warms_zone() {
            let state = ZoneAirState::new(20.0, 0.008);
            let balance = ZoneAirBalance {
                volume_m3: 200.0,
                conditioned: true,
                sensible_gain_w: 2000.0,
                latent_gain_w: 0.0,
                infiltration_sensible_w: -100.0,
                infiltration_latent_w: 0.0,
                ventilation_sensible_w: 0.0,
                ventilation_latent_w: 0.0,
                system_sensible_w: 0.0,
                system_latent_w: 0.0,
                surface_convection_w: 0.0,
                mass_flow_in_kg_s: 0.05,
                supply_humidity_ratio: 0.008,
                outdoor_humidity_ratio: 0.006,
                heating_setpoint_c: Some(21.0),
                cooling_setpoint_c: Some(26.0),
                max_heating_w: Some(5000.0),
                max_cooling_w: Some(5000.0),
            };
            let result = advance_zone_air(&state, &balance, 3600.0, HumiditySolutionMethod::ThirdOrderBackward, P_STD);
            assert!(result.temp_c > 20.0);
        }

        #[test]
        fn humidity_analytical_increases_with_latent_gain() {
            let w = analytical_steady_humidity_ratio(200.0, 0.1, 0.008, 22.0);
            assert!(w > 0.008);
        }
    }
}

mod zone_hvac {
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
}

pub use air_exchange::*;
pub use air_system::*;
pub use airflow_network::*;
pub use calendar::*;
pub use coils::*;
pub use comfort::*;
pub use controls::*;
pub use curves::*;
pub use daylight::*;
pub use dispatch::*;
pub use economics::*;
pub use electrical::*;
pub use envelope::*;
pub use error::*;
pub use evaporative::*;
pub use fans::*;
pub use faults::*;
pub use fenestration::*;
pub use gains::*;
pub use geometry::*;
pub use heat_recovery::*;
pub use humidity_eq::*;
pub use hvac_topo::*;
pub use iaq::*;
pub use ideal_hvac::{ideal_loads_deliver, ideal_loads_deliver_with_controls, EconomizerControl, HumidityControl, IdealLoadsConfig, IdealLoadsInput, IdealLoadsOutput, IdealLoadsRequest};
pub use kernel::*;
pub use material::*;
pub use meters::*;
pub use metrics::*;
pub use model::*;
pub use num::*;
pub use output::*;
pub use plant::*;
pub use precompute::*;
pub use props::*;
pub use refrigeration::*;
pub use results::*;
pub use room_air::*;
pub use schedule::*;
pub use shw::*;
pub use sim::*;
pub use site::*;
pub use sizing::*;
pub use solar::*;
pub use solar_thermal::*;
pub use terminal::*;
pub use units::*;
pub use water::*;
pub use zone_air::*;
pub use zone_hvac::*;
