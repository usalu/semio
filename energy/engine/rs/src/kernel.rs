//! 🔄 Simulation kernel: calendar, multi-rate loops, warmup, predictor-corrector coupling.

use crate::air_exchange::{infiltration_flow_m3_s, ventilation_load_w, InfiltrationMethod, InfiltrationSpec};
use crate::airflow_network::{AfLink, AfLinkKind, AfNode, AirflowNetwork};
use crate::calendar::{RunPeriod, SimDate};
use crate::controls::{
    evaluate_controls, predict_zone_load, HumidistatSpec, ThermostatSpec,
};
use crate::daylight::{dimmed_lighting_power_w, lighting_dimming_fraction, reference_point_illuminance_lux, simplified_daylight_factor};
use crate::dispatch::{DispatchRequest, DispatchScheme, Dispatcher, EquipmentPriority};
use crate::electrical::{grid_balance, PvSystem, Transformer};
use crate::envelope::{
    solve_exterior_surface_temp, solve_interior_surface_temp,
    ConductionState, ExteriorConvectionModel, InteriorConvectionModel,
};
use crate::error::Error;
use crate::faults::SeveritySchedule;
use crate::gains::{compute_equipment_gain_w, compute_lighting_gain_w, compute_people_gain_w, ActivityLevel, GainDecomposition};
use crate::ideal_hvac::{ideal_loads_deliver, IdealLoadsConfig, IdealLoadsInput};
use crate::model::{EntityId, Model, OutsideBoundary, SurfaceClass};
use crate::plant::{PlantLoopSimulation, PlantStream, Pump};
use crate::precompute::PrecomputedModel;
use crate::props::saturation_pressure_pa;
use crate::schedule::{ScheduleContext, ScheduleSet};
use crate::site::{GroundTemperatureModel, WeatherRecord};
use crate::solar::{shading_factor, surface_solar_absorption};
use crate::units::P_STD;
use crate::zone_air::{advance_zone_air, HumiditySolutionMethod, ZoneAirBalance, ZoneAirState};
use crate::zone_hvac::{ZoneEquipment, ZoneEquipmentRequest};
use crate::curves::PerformanceCurve;
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
        Self {
            temperature_k: 0.01,
            humidity_ratio: 1e-5,
            mass_flow: 1e-4,
            energy_w: 1.0,
            max_iterations: 20,
        }
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
        self.heating_w + self.cooling_w + self.fan_w + self.pump_w + self.compressor_w
            + self.shw_electric_w + self.refrigeration_w + self.water_pump_w
            - self.pv_generation_w
            + self.battery_charge_w
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
        Self {
            air: ZoneAirState::new(20.0, 0.01),
            heating_demand_w: 0.0,
            cooling_demand_w: 0.0,
            unmet_heating_w: 0.0,
            unmet_cooling_w: 0.0,
            delivered: DeliveredEnergy::default(),
        }
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
        Self {
            zones: HashMap::new(),
            surfaces: HashMap::new(),
            warmup_complete: false,
            hour: 0,
            delivered_total: DeliveredEnergy::default(),
            battery_soc: 0.5,
            plant_supply_c: 55.0,
        }
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
            state.zones.insert(
                zone.id,
                ZoneState {
                    air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()),
                    ..ZoneState::empty()
                },
            );
        }
        for (sid, sp) in &pre.surfaces {
            state.surfaces.insert(
                *sid,
                SurfaceState {
                    inside_temp_c: weather.dry_bulb_c,
                    outside_temp_c: weather.dry_bulb_c,
                    heat_flux_w: 0.0,
                    ctf: sp.ctf.clone(),
                    convection_to_zone_w: 0.0,
                },
            );
        }
        state
    }

    /// 🔄 Run warmup until temperature and load convergence.
    pub fn warmup(
        model: &Model,
        config: &SimulationConfig,
        pre: &PrecomputedModel,
        state: &mut SimulationState,
        weather_records: &[WeatherRecord],
    ) -> Result<(), Error> {
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
                let temp_ok = state.zones.iter().all(|(id, zs)| {
                    prev_temps
                        .get(id)
                        .map(|prev| (zs.air.temp_c - prev).abs() <= config.tolerances.temperature_k)
                        .unwrap_or(false)
                });
                let load_ok = state.zones.iter().all(|(id, zs)| {
                    prev_loads
                        .get(id)
                        .map(|prev| {
                            let load = zs.heating_demand_w + zs.cooling_demand_w;
                            (load - prev).abs() <= config.tolerances.energy_w
                        })
                        .unwrap_or(false)
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
    pub fn advance_timestep(
        model: &Model,
        config: &SimulationConfig,
        pre: &PrecomputedModel,
        state: &mut SimulationState,
        weather: &WeatherRecord,
        date: &SimDate,
        hour: f64,
        dt_s: f64,
    ) -> Result<(), Error> {
        let ctx = ScheduleContext {
            year: date.year,
            month: date.month,
            day: date.day,
            hour: weather.hour,
            day_of_week: date.day_of_week(),
            timestep_index: hour as u32,
            is_dst: false,
        };

        let day_of_year = date.day_of_year();
        let (sun_alt, sun_az) = pre.solar_at(model, day_of_year, hour);
        let ext_conv = ExteriorConvectionModel::default();
        let int_conv = InteriorConvectionModel::default();
        let sky_temp_k = weather.dry_bulb_c + 273.15 - 20.0;
        let ground_model = GroundTemperatureModel::Monthly {
            temperatures_c: model.ground_temperature.building_surface_c,
        };

        let mut zone_envelope_w: HashMap<EntityId, f64> = HashMap::new();
        let mut zone_solar_w: HashMap<EntityId, f64> = HashMap::new();
        let mut zone_surface_conv_w: HashMap<EntityId, f64> = HashMap::new();

        for (sid, sp) in &pre.surfaces {
            let surface = model.surfaces.iter().find(|s| s.id == *sid);
            let outside_temp = match surface.map(|s| s.outside_boundary_condition) {
                Some(OutsideBoundary::Ground) => ground_model.temperature_c(day_of_year),
                Some(OutsideBoundary::OutdoorAir) | None => weather.dry_bulb_c,
                Some(OutsideBoundary::OtherSideTemperature) => weather.dry_bulb_c - 5.0,
                Some(OutsideBoundary::Adiabatic) | Some(OutsideBoundary::Interzone(_)) => {
                    state.surfaces.get(sid).map(|s| s.outside_temp_c).unwrap_or(weather.dry_bulb_c)
                }
            };

            let zone_t = state
                .zones
                .get(&sp.zone_id)
                .map(|z| z.air.temp_c)
                .unwrap_or(weather.dry_bulb_c);

            let mut solar_w_m2 = 0.0;
            if sp.sun_exposed && sun_alt > 0.0 {
                let incidence = crate::solar::beam_incidence_cosine(sp.normal, sun_alt, sun_az);
                let shade = shading_factor(1.0, 0.0, 1.0, sun_alt);
                let abs = surface_solar_absorption(
                    weather.direct_normal_irradiance_w_m2,
                    weather.diffuse_horizontal_irradiance_w_m2,
                    incidence,
                    shade,
                    sp.solar_absorptance,
                    sp.tilt_deg,
                );
                solar_w_m2 = abs.total_w_m2;
            }

            let ss = state.surfaces.entry(*sid).or_insert_with(|| SurfaceState {
                inside_temp_c: zone_t,
                outside_temp_c: outside_temp,
                heat_flux_w: 0.0,
                ctf: sp.ctf.clone(),
                convection_to_zone_w: 0.0,
            });

            let conduction_w_m2 = ss.ctf.heat_flux_w_m2(outside_temp, zone_t);
            let exterior_t = solve_exterior_surface_temp(
                outside_temp,
                sky_temp_k,
                weather.wind_speed_m_s,
                solar_w_m2,
                -conduction_w_m2,
                sp.emissivity,
                &ext_conv,
            );
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
                let zone_t = state
                    .zones
                    .get(&surface.zone_id)
                    .map(|z| z.air.temp_c)
                    .unwrap_or(weather.dry_bulb_c);
                let u = fp.u_value_w_m2k;
                let cond_w = u * fp.area_m2 * (weather.dry_bulb_c - zone_t);
                *zone_envelope_w.entry(surface.zone_id).or_default() += cond_w;

                if sun_alt > 0.0 {
                    let incidence = crate::solar::beam_incidence_cosine(fp.normal, sun_alt, sun_az);
                    let shade = shading_factor(1.0, 0.0, 1.0, sun_alt);
                    let solar_w = (weather.direct_normal_irradiance_w_m2 * incidence * shade
                        + weather.diffuse_horizontal_irradiance_w_m2 * 0.5)
                        * fp.shgc
                        * fp.area_m2;
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

            let zone_t = state.zones.get(&zone.id).map(|z| z.air.temp_c).unwrap_or(weather.dry_bulb_c);
            let zone_w = state.zones.get(&zone.id).map(|z| z.air.humidity_ratio).unwrap_or(weather.humidity_ratio());

            let mut lighting_dim = 1.0_f64;
            if let Some(dz) = model.daylight_zones.iter().find(|d| d.zone_id == zone.id) {
                let df = simplified_daylight_factor(
                    model.fenestrations.iter().map(|f| f.area_m2).sum(),
                    floor_area_m2,
                    dz.window_transmittance,
                );
                let lux = reference_point_illuminance_lux(
                    weather.diffuse_horizontal_irradiance_w_m2 * 120.0,
                    weather.direct_normal_irradiance_w_m2 * 120.0,
                    sun_alt.max(0.0) / 90.0,
                    dz.window_transmittance,
                    df,
                    1.0,
                );
                lighting_dim = lighting_dimming_fraction(lux, dz.illuminance_target_lux, 0.1);
            }

            let mut internal_gain = GainDecomposition::default();
            for person in model.people.iter().filter(|p| p.zone_id == zone.id) {
                let occ = config.schedules.lookup(person.schedule_id, &ctx);
                let count = person.people_per_area * floor_area_m2 * occ;
                internal_gain = internal_gain.add(&compute_people_gain_w(
                    count,
                    ActivityLevel::OfficeWork,
                    1.0,
                    person.radiant_fraction,
                ));
            }
            for light in model.lighting.iter().filter(|l| l.zone_id == zone.id) {
                let frac = config.schedules.lookup(light.schedule_id, &ctx) * lighting_dim;
                let power = dimmed_lighting_power_w(light.watts_per_area * floor_area_m2, frac);
                internal_gain = internal_gain.add(&compute_lighting_gain_w(
                    power / floor_area_m2.max(1.0),
                    floor_area_m2,
                    1.0,
                    light.radiant_fraction,
                    light.return_air_fraction,
                ));
            }
            for equip in model.equipment.iter().filter(|e| e.zone_id == zone.id) {
                let frac = config.schedules.lookup(equip.schedule_id, &ctx);
                internal_gain = internal_gain.add(&compute_equipment_gain_w(
                    equip.watts_per_area,
                    floor_area_m2,
                    frac,
                    equip.radiant_fraction,
                    equip.latent_fraction,
                ));
            }

            let mut infil_flow = model
                .infiltrations
                .iter()
                .find(|i| i.zone_id == zone.id)
                .map(|inf| {
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
                    infiltration_flow_m3_s(
                        &spec,
                        zone.volume_m3,
                        exterior_area_m2,
                        weather.dry_bulb_c,
                        zone_t,
                        weather.wind_speed_m_s,
                        weather.atmospheric_pressure_pa,
                    )
                })
                .unwrap_or(0.0);

            if let Some(afn_def) = &model.airflow_network {
                let mut nodes = vec![AfNode {
                    id: afn_def.outdoor_node_id,
                    elevation_m: 0.0,
                    temperature_c: weather.dry_bulb_c,
                    humidity_ratio: weather.humidity_ratio(),
                    is_reference: true,
                }];
                for (zid, nid) in &afn_def.zone_node_ids {
                    if *zid == zone.id {
                        let zt = state.zones.get(zid).map(|z| z.air.temp_c).unwrap_or(zone_t);
                        let zw = state.zones.get(zid).map(|z| z.air.humidity_ratio).unwrap_or(zone_w);
                        nodes.push(AfNode {
                            id: *nid,
                            elevation_m: 3.0,
                            temperature_c: zt,
                            humidity_ratio: zw,
                            is_reference: false,
                        });
                    }
                }
                if nodes.len() > 1 {
                    let net = AirflowNetwork {
                        nodes,
                        links: vec![AfLink {
                            id: 1,
                            node_a: afn_def.zone_node_ids.iter().find(|(z, _)| *z == zone.id).map(|(_, n)| *n).unwrap_or(1),
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

            let mech_flow = model
                .mechanical_ventilations
                .iter()
                .filter(|m| m.zone_id == zone.id)
                .map(|m| m.design_flow_m3_s * config.schedules.lookup(m.schedule_id, &ctx))
                .sum::<f64>();

            let total_vent_flow = infil_flow + mech_flow;
            let (infil_sens, infil_lat) = ventilation_load_w(
                total_vent_flow,
                zone_t,
                zone_w,
                weather.dry_bulb_c,
                weather.humidity_ratio(),
                weather.atmospheric_pressure_pa,
                0.0,
            );

            let envelope_w = zone_envelope_w.get(&zone.id).copied().unwrap_or(0.0);
            let solar_w = zone_solar_w.get(&zone.id).copied().unwrap_or(0.0);
            let surface_conv_w = zone_surface_conv_w.get(&zone.id).copied().unwrap_or(0.0);

            let setpoints = pre.default_setpoints.get(&zone.id).copied().unwrap_or_default();
            let heat_sp = model
                .thermostats
                .iter()
                .find(|t| t.zone_id == zone.id)
                .map(|t| config.schedules.lookup(t.heating_setpoint_schedule_id, &ctx) * 24.0 + 20.0)
                .unwrap_or(setpoints.heating_c);
            let cool_sp = model
                .thermostats
                .iter()
                .find(|t| t.zone_id == zone.id)
                .map(|t| config.schedules.lookup(t.cooling_setpoint_schedule_id, &ctx) * 6.0 + 24.0)
                .unwrap_or(setpoints.cooling_c);

            let humidistat = model.humidistats.iter().find(|h| h.zone_id == zone.id);
            let hum_spec = humidistat.map(|h| HumidistatSpec {
                humidifying_setpoint_rh: 0.4,
                dehumidifying_setpoint_rh: 0.6,
                humidifying_throttle_range: h.humidifying_throttle_range,
                dehumidifying_throttle_range: h.dehumidifying_throttle_range,
            });
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

            let zone_state = state.zones.entry(zone.id).or_insert_with(|| ZoneState {
                air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()),
                ..ZoneState::empty()
            });

            for _sub in 0..sub_steps.max(1) {
                let ctrl = evaluate_controls(&therm_spec, hum_spec.as_ref(), zone_state.air.temp_c, zone_rh);
                let residual_sens = sensible_gain - zone_state.heating_demand_w + zone_state.cooling_demand_w;
                let predicted = predict_zone_load(
                    residual_sens,
                    internal_gain.latent_w,
                    &ctrl,
                    f64::INFINITY,
                    f64::INFINITY,
                    5000.0,
                    5000.0,
                );

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

                let result = advance_zone_air(
                    &zone_state.air,
                    &balance,
                    system_dt_s,
                    HumiditySolutionMethod::ThirdOrderBackward,
                    weather.atmospheric_pressure_pa,
                );
                zone_state.air.push_temp(result.temp_c);
                zone_state.air.push_humidity(result.humidity_ratio);
                zone_state.heating_demand_w = predicted.heating_w;
                zone_state.cooling_demand_w = predicted.cooling_w;

                for ils in model.ideal_loads.iter().filter(|i| i.zone_id == zone.id) {
                    let fault_factor = model
                        .faults
                        .iter()
                        .find(|f| f.target_equipment_id == ils.id)
                        .map(|f| 1.0 - f.severity * SeveritySchedule::constant(1.0).at_hour(weather.hour))
                        .unwrap_or(1.0);

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
                    let corrected = advance_zone_air(
                        &zone_state.air,
                        &balance,
                        system_dt_s,
                        HumiditySolutionMethod::ThirdOrderBackward,
                        weather.atmospheric_pressure_pa,
                    );
                    zone_state.air.push_temp(corrected.temp_c);
                    zone_state.unmet_heating_w = output.unmet_heating_w;
                    zone_state.unmet_cooling_w = output.unmet_cooling_w;
                }

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
                        _ => ZoneEquipment::Baseboard {
                            heating: crate::coils::HeatingCoil::Electric {
                                capacity_w: ze.heating_capacity_w,
                                efficiency: 1.0,
                            },
                        },
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

    fn simulate_secondary(
        model: &Model,
        config: &SimulationConfig,
        _pre: &PrecomputedModel,
        state: &mut SimulationState,
        weather: &WeatherRecord,
        ctx: &ScheduleContext,
        sun_alt: f64,
        sun_az: f64,
        dt_s: f64,
    ) {
        for plant in &model.plant_loops {
            let total_load: f64 = state.zones.values().map(|z| z.heating_demand_w + z.cooling_demand_w).sum();
            let dispatcher = Dispatcher::new(
                DispatchScheme::Sequential,
                plant
                    .equipment_ids
                    .iter()
                    .map(|id| EquipmentPriority {
                        equipment_id: id.0,
                        priority: 1,
                        min_runtime_hours: 0.0,
                        capacity_w: 100_000.0,
                    })
                    .collect(),
            );
            let results = dispatcher.dispatch(&DispatchRequest {
                total_load_w: total_load,
                available_capacity_w: 500_000.0,
                outdoor_temp_c: weather.dry_bulb_c,
            });
            let pump = Pump {
                design_head_pa: 200_000.0,
                design_flow_kg_s: plant.design_flow_kg_s,
                motor_efficiency: 0.85,
                part_load_curve: PerformanceCurve::Constant(1.0),
            };
            let loop_sim = PlantLoopSimulation {
                supply: PlantStream::new(plant.supply_temperature_c, plant.design_flow_kg_s),
                return_stream: PlantStream::new(plant.return_temperature_c, plant.design_flow_kg_s),
                pump,
                glycol_fraction: 0.0,
            };
            let plant_out = loop_sim.simulate(results.first().map(|r| r.load_w).unwrap_or(0.0));
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
            let net_load: f64 = state
                .zones
                .values()
                .map(|z| z.delivered.total_electric_w())
                .sum();
            let charge_w = if pv_gen > net_load {
                (pv_gen - net_load).min(battery.max_charge_w)
            } else {
                -(net_load - pv_gen).min(battery.max_discharge_w)
            };
            state.battery_soc = (state.battery_soc + charge_w * dt_s / (battery.capacity_kwh * 3_600_000.0))
                .clamp(0.0, 1.0);
            state.delivered_total.battery_charge_w += charge_w.max(0.0);
            let transformer = Transformer {
                rated_kva: 100.0,
                no_load_loss_w: 50.0,
                load_loss_w: 200.0,
                impedance_fraction: 0.02,
            };
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
        RunPeriod {
            start_month: config.run_period_start_month,
            start_day: config.run_period_start_day,
            end_month: config.run_period_end_month,
            end_day: config.run_period_end_day,
            year: 2026,
        }
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
        let config = SimulationConfig {
            run_period_start_month: 1,
            run_period_start_day: 1,
            run_period_end_month: 1,
            run_period_end_day: 7,
            ..Default::default()
        };
        assert_eq!(SimulationKernel::run_period(&config).total_hours(), 168);
    }
}
