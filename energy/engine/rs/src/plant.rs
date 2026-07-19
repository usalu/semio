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
