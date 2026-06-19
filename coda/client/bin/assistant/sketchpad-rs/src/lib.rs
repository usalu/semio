use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;
use serde_json::Value;

// Embed the Tabula database
static TABULA_DATA: &str = include_str!("../../tabula_data/extracted_data.json");

lazy_static::lazy_static! {
    static ref TABULA_DB: HashMap<String, Value> = {
        serde_json::from_str(TABULA_DATA).unwrap_or_default()
    };
}

// -----------------------------------------------------------------------------
// Data Structures
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EnvelopeDirectionData {
    pub gross_wall_area: f64,
    pub window_area: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EnvelopeData {
    #[serde(rename = "N")]
    pub n: EnvelopeDirectionData,
    #[serde(rename = "E")]
    pub e: EnvelopeDirectionData,
    #[serde(rename = "S")]
    pub s: EnvelopeDirectionData,
    #[serde(rename = "W")]
    pub w: EnvelopeDirectionData,
    #[serde(rename = "H")]
    #[serde(default)]
    pub h: EnvelopeDirectionData,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BuildingGeometry {
    pub total_conditioned_volume: f64,
    pub total_floor_area: f64,
    pub total_roof_area: f64,
    pub total_ground_area: f64,
    pub exterior_perimeter: f64,
    #[serde(default)]
    pub envelope_data: EnvelopeData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CustomInsulation {
    pub thickness_m: f64,
    pub lambda: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameters {
    pub building_type: String,
    pub year_class: String,
    pub scenario: String,
    pub story_height: f64,
    pub num_stories: i32,
    pub window_to_wall_ratio: f64,
    pub building_rotation_deg: f64,
    pub heating_system: String,
    pub custom_wall_insulation: Option<CustomInsulation>,
    
    // Explicit physics parameters fetched from Neo4j (DIN V 18599 & TABULA)
    // (Removed graph parameters)
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            building_type: "SFH".to_string(),
            year_class: "2016-...".to_string(),
            scenario: "Existing State".to_string(),
            story_height: 2.8,
            num_stories: 1,
            window_to_wall_ratio: 0.15,
            building_rotation_deg: 0.0,
            heating_system: "Gas Condensing Boiler".to_string(),
            custom_wall_insulation: None,
            // (Removed graph parameters)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct State {
    pub geometry: Option<BuildingGeometry>,
    pub params: Parameters,
    pub ui_state: Option<Value>,
}

// -----------------------------------------------------------------------------
// Action Logging & Event Sourcing
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum Action {
    Init,
    UpdateGeometry(BuildingGeometry),
    UpdateParameters(Parameters),
    GenericUpdate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogEntry {
    pub timestamp: f64,
    pub action: Action,
    pub description: String,
}

impl LogEntry {
    pub fn new(action: Action, description: &str) -> Self {
        Self {
            timestamp: js_sys::Date::now(),
            action,
            description: description.to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// History Store
// -----------------------------------------------------------------------------

pub struct HistoryStore {
    past: Vec<(State, LogEntry)>,
    current: State,
    current_log: LogEntry,
    future: Vec<(State, LogEntry)>,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self {
            past: Vec::new(),
            current: State::default(),
            current_log: LogEntry::new(Action::Init, "Initialized engine"),
            future: Vec::new(),
        }
    }

    pub fn apply_action(&mut self, new_state: State, action: Action, description: &str) {
        let entry = LogEntry::new(action, description);
        self.past.push((self.current.clone(), self.current_log.clone()));
        self.current = new_state;
        self.current_log = entry;
        self.future.clear();
    }

    pub fn set_state(&mut self, new_state: State) {
        self.apply_action(new_state, Action::GenericUpdate, "Generic state update");
    }

    pub fn undo(&mut self) -> bool {
        if let Some((prev_state, prev_log)) = self.past.pop() {
            self.future.push((self.current.clone(), self.current_log.clone()));
            self.current = prev_state;
            self.current_log = prev_log;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some((next_state, next_log)) = self.future.pop() {
            self.past.push((self.current.clone(), self.current_log.clone()));
            self.current = next_state;
            self.current_log = next_log;
            true
        } else {
            false
        }
    }

    pub fn current(&self) -> &State {
        &self.current
    }

    pub fn current_log(&self) -> &LogEntry {
        &self.current_log
    }

    pub fn get_log_history(&self) -> Vec<LogEntry> {
        let mut history: Vec<LogEntry> = self.past.iter().map(|(_, log)| log.clone()).collect();
        history.push(self.current_log.clone());
        history
    }
}

// Global Store
thread_local! {
    static STORE: RefCell<HistoryStore> = RefCell::new(HistoryStore::new());
}

// -----------------------------------------------------------------------------
// Calculation Engine
// -----------------------------------------------------------------------------

// We do a simplified steady-state calculation replicating the ISO 13790 logic from Python.
// In a full implementation, Shapely would be used for exact union area. 
// Since this is Rust WASM, we calculate a bounding box or simple sum of areas.
// We assume simple non-overlapping rectangular zones for the area calculation to match the sketchpad.

fn calculate_energy(state: &State) -> Value {
    let geometry = match &state.geometry {
        Some(g) => g,
        None => return serde_json::json!({ "status": "error", "message": "No geometry defined." }),
    };

    let a_floor_total = geometry.total_floor_area;
    let a_roof = geometry.total_roof_area;
    let a_ground = geometry.total_ground_area;
    let perimeter = geometry.exterior_perimeter;
    let conditioned_volume = geometry.total_conditioned_volume;

    let win_n = geometry.envelope_data.n.window_area;
    let win_e = geometry.envelope_data.e.window_area;
    let win_s = geometry.envelope_data.s.window_area;
    let win_w = geometry.envelope_data.w.window_area;
    let win_h = geometry.envelope_data.h.window_area;

    let a_wall = geometry.envelope_data.n.gross_wall_area
        + geometry.envelope_data.e.gross_wall_area
        + geometry.envelope_data.s.gross_wall_area
        + geometry.envelope_data.w.gross_wall_area;

    let a_window = win_n + win_e + win_s + win_w + win_h;
    let net_wall = a_wall - (win_n + win_e + win_s + win_w); // Only vertical windows reduce wall area


    // Lookup TABULA
    let year_class = &state.params.year_class;
    let age_code = match year_class.as_str() {
        "...1859" => "01", "1860-1918" => "02", "1919-1948" => "03", "1949-1957" => "04",
        "1958-1968" => "05", "1969-1978" => "06", "1979-1983" => "07", "1984-1994" => "08",
        "1995-2001" => "09", "2002-2009" => "10", "2010-2015" => "11", "2016-..." | "2016+" => "12",
        _ => "01"
    };

    let suffix = match state.params.scenario.as_str() {
        "Existing State" => ".001",
        "Usual Refurbishment" => ".002",
        "Advanced Refurbishment" => ".003",
        _ => ".001"
    };

    let prefix = format!("DE_{}_{}", state.params.building_type, age_code);
    
    // Find archetype
    let mut u_wall = 1.0;
    let mut u_roof = 1.0;
    let mut u_floor = 1.0;
    let mut u_window = 1.3;
    let mut g_value = 0.7;

    for (k, v) in TABULA_DB.iter() {
        if k.starts_with(&prefix) && k.ends_with(suffix) {
            u_wall = v.get("u_wall").and_then(Value::as_f64).unwrap_or(1.0);
            u_roof = v.get("u_roof").and_then(Value::as_f64).unwrap_or(1.0);
            u_floor = v.get("u_floor").and_then(Value::as_f64).unwrap_or(1.0);
            u_window = v.get("u_window").and_then(Value::as_f64).unwrap_or(1.3);
            g_value = v.get("g_value").and_then(Value::as_f64).unwrap_or(0.7);
            break;
        }
    }
    
    // Always find existing state u_wall for the breakdown
    let mut u_wall_existing = u_wall;
    for (k, v) in TABULA_DB.iter() {
        if k.starts_with(&prefix) && k.ends_with(".001") {
            u_wall_existing = v.get("u_wall").and_then(Value::as_f64).unwrap_or(1.0);
            break;
        }
    }

    let r_base = 1.0 / u_wall_existing;
    let mut r_ins = 0.0;

    // Apply custom wall insulation override if specified
    if let Some(ref custom_ins) = state.params.custom_wall_insulation {
        r_ins = custom_ins.thickness_m / custom_ins.lambda;
        u_wall = 1.0 / (r_base + r_ins);
    }

    let delta_u_tbr = match state.params.scenario.as_str() {
        "Advanced Refurbishment" => 0.0,
        "Usual Refurbishment" => 0.05,
        _ => 0.10,
    };

    let sum_a = net_wall + a_roof + a_ground + a_window;
    let sum_bau = (net_wall * u_wall * 1.0) + (a_roof * u_roof * 1.0) + (a_ground * u_floor * 0.5) + (a_window * u_window * 1.0);
    let h_tr = sum_bau + (sum_a * delta_u_tbr);
    
    // Infiltration
    let is_old = match year_class.as_str() {
        "...1859" | "1860-1918" | "1919-1948" | "1949-1957" | "1958-1968" | "1969-1978" => true,
        _ => false,
    };

    let n_infiltr = match state.params.scenario.as_str() {
        "Advanced Refurbishment" => 0.05,
        "Usual Refurbishment" => 0.1,
        _ => if is_old { 0.4 } else { 0.2 },
    };

    let h_ve = 0.34 * (0.4 + n_infiltr) * conditioned_volume;
    
    // ISO 13790
    let theta_int = 20.0;
    let theta_e = 4.4;
    let d_hs = 222.0;

    let q_ht = 0.024 * (h_tr + h_ve) * 1.0 * (theta_int - theta_e) * d_hs;
    let q_ht_tr = 0.024 * h_tr * 1.0 * (theta_int - theta_e) * d_hs;
    let q_ht_ve = 0.024 * h_ve * 1.0 * (theta_int - theta_e) * d_hs;
    
    
    // win_n, win_e, win_s, win_w are already calculated.
    
    let sol_factor_vertical = 0.6 * (1.0 - 0.3) * 0.9 * g_value;
    let sol_factor_horizontal = 0.8 * (1.0 - 0.3) * 0.9 * g_value;
    
    let q_sol_n = sol_factor_vertical * win_n * 160.0;
    let q_sol_e = sol_factor_vertical * win_e * 271.0;
    let q_sol_s = sol_factor_vertical * win_s * 392.0;
    let q_sol_w = sol_factor_vertical * win_w * 271.0;
    let q_sol_h = sol_factor_horizontal * win_h * 392.0;
    
    let q_sol = q_sol_n + q_sol_e + q_sol_s + q_sol_w + q_sol_h;
    
    // Internal heat gains according to equation (9)
    let phi_int = 3.0; // average thermal output of internal heat sources [W/m²]
    let q_int = 0.024 * phi_int * d_hs * a_floor_total;
    let q_gn = q_sol + q_int;
    
    let q_h_nd = f64::max(0.0, q_ht - 0.95 * q_gn);
    
    // 1. Determine System Properties: (e_g_h, q_d_h_specific, q_s_h_specific)
    let (e_g_h, q_d_h_spec, q_s_h_spec) = match state.params.heating_system.as_str() {
        "Gas Condensing Boiler" => (1.05, 15.0, 0.0),      // High efficiency, typical pipes
        "Gas Non-Condensing Boiler" => (1.18, 15.0, 5.0),  // Older tech, has storage tank
        "Air Source Heat Pump" => (0.35, 10.0, 5.0),       // COP ~2.8, modern pipes
        "Biomass Pellet Boiler" => (1.25, 15.0, 10.0),     // Lower efficiency, large buffer tank
        "Direct Electric Heating" => (1.00, 0.0, 0.0),     // 100% efficient at point of use, no pipes
        _ => (1.10, 15.0, 0.0), // Default fallback
    };

    // 2. Calculate absolute losses (Specific Loss * Floor Area)
    let q_d_h_total = q_d_h_spec * a_floor_total;
    let q_s_h_total = q_s_h_spec * a_floor_total;

    // 3. Apply TABULA Equation 20 (Simplified): Heat Output of Generator
    // Q_g_h_out = Q_h_nd + Q_d_h + Q_s_h
    let q_g_h_out = q_h_nd + q_d_h_total + q_s_h_total;

    // 4. Apply TABULA Equation 19: Delivered Energy
    // Q_del_h = Q_g_h_out * e_g_h
    let q_final = q_g_h_out * e_g_h;

    serde_json::json!({
        "status": "success",
        "envelope_areas_m2": {
            "net_wall": net_wall,
            "roof": a_roof,
            "floor": a_ground,
            "window": a_window,
            "total_floor": a_floor_total,
            "exterior_perimeter_m": perimeter
        },
        "tabula_u_values": {
            "wall_W_m2K": u_wall,
            "roof_W_m2K": u_roof,
            "floor_W_m2K": u_floor,
            "window_W_m2K": u_window
        },
        "heat_losses": {
            "Q_ht_kWh_a": q_ht,
            "transmission_loss_kWh_a": q_ht_tr,
            "ventilation_loss_kWh_a": q_ht_ve
        },
        "heat_gains": {
            "solar_gains_kWh_a": q_sol,
            "internal_gains_kWh_a": q_int,
            "total_gains_kWh_a": q_gn
        },
        "heating_demand": {
            "Q_H_nd_kWh_a": q_h_nd,
            "specific_Q_H_nd_kWh_m2a": q_h_nd / a_floor_total
        },
        "final_energy": {
            "Q_final_kWh_a": q_final,
            "specific_Q_final_kWh_m2a": q_final / a_floor_total
        },
        "wall_insulation_breakdown": {
            "u_wall_base": u_wall_existing,
            "r_wall_base": r_base,
            "r_insulation": r_ins,
            "u_wall_final": u_wall
        }
    })
}

// -----------------------------------------------------------------------------
// WASM Exports
// -----------------------------------------------------------------------------

#[wasm_bindgen]
pub fn init_engine() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_state() -> String {
    STORE.with(|store| {
        let store = store.borrow();
        serde_json::to_string(store.current()).unwrap()
    })
}

#[wasm_bindgen]
pub fn get_history_log() -> String {
    STORE.with(|store| {
        let store = store.borrow();
        serde_json::to_string(&store.get_log_history()).unwrap()
    })
}

#[wasm_bindgen]
pub fn update_state(state_json: &str) -> String {
    match serde_json::from_str::<State>(state_json) {
        Ok(new_state) => {
            STORE.with(|store| {
                let mut store = store.borrow_mut();
                store.set_state(new_state);
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            })
        }
        Err(e) => {
            serde_json::json!({ "status": "error", "message": format!("Invalid state payload: {}", e) }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn update_geometry(geom_json: &str) -> String {
    match serde_json::from_str::<BuildingGeometry>(geom_json) {
        Ok(geometry) => {
            STORE.with(|store| {
                let mut store = store.borrow_mut();
                let mut new_state = store.current().clone();
                new_state.geometry = Some(geometry.clone());
                store.apply_action(new_state, Action::UpdateGeometry(geometry), "Updated building geometry");
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            })
        }
        Err(e) => {
            serde_json::json!({ "status": "error", "message": format!("Invalid geometry payload: {}", e) }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn update_parameters(params_json: &str) -> String {
    match serde_json::from_str::<Parameters>(params_json) {
        Ok(params) => {
            STORE.with(|store| {
                let mut store = store.borrow_mut();
                let mut new_state = store.current().clone();
                new_state.params = params.clone();
                store.apply_action(new_state, Action::UpdateParameters(params), "Updated parameters");
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            })
        }
        Err(e) => {
            serde_json::json!({ "status": "error", "message": format!("Invalid parameters payload: {}", e) }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn undo() -> String {
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        if store.undo() {
            let res = calculate_energy(store.current());
            serde_json::json!({
                "status": "success",
                "state": store.current(),
                "energy": res,
                "log": store.current_log()
            }).to_string()
        } else {
            serde_json::json!({ "status": "error", "message": "No undo history" }).to_string()
        }
    })
}

#[wasm_bindgen]
pub fn redo() -> String {
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        if store.redo() {
            let res = calculate_energy(store.current());
            serde_json::json!({
                "status": "success",
                "state": store.current(),
                "energy": res,
                "log": store.current_log()
            }).to_string()
        } else {
            serde_json::json!({ "status": "error", "message": "No redo history" }).to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_state() {
        let json_payload = r#"{
            "geometry": null,
            "params": {
                "building_type": "SFH",
                "year_class": "2016-...",
                "scenario": "Existing State",
                "story_height": 2.8,
                "num_stories": 1,
                "window_to_wall_ratio": 0.15,
                "building_rotation_deg": 0.0,
                "heating_system": "Gas Condensing Boiler",
                "custom_wall_insulation": null
            },
            "ui_state": null
        }"#;
        let res = update_state(json_payload);
        println!("{}", res);
        assert!(res.contains("success"));
    }
}
