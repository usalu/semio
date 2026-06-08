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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Window {
    pub id: String,
    pub wall_id: String,
    pub u: f64,
    pub v: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Zone {
    pub id: String,
    pub room_type: String,
    pub width: f64,
    pub length: f64,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub windows: Vec<Window>,
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
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct State {
    pub zones: Vec<Zone>,
    pub params: Parameters,
}

// -----------------------------------------------------------------------------
// Action Logging & Event Sourcing
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum Action {
    Init,
    AddZone(Zone),
    RemoveZone(String),
    UpdateZone(Zone),
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
    if state.zones.is_empty() {
        return serde_json::json!({ "status": "error", "message": "No zones defined." });
    }

    use geo::{polygon, MultiPolygon};
    use geo::algorithm::area::Area;

    use geo::BooleanOps;

    let mut unioned: MultiPolygon<f64> = MultiPolygon::new(vec![]);
    for z in &state.zones {
        let poly = polygon![
            (x: z.x, y: z.y),
            (x: z.x + z.width, y: z.y),
            (x: z.x + z.width, y: z.y + z.length),
            (x: z.x, y: z.y + z.length),
            (x: z.x, y: z.y),
        ];
        unioned = unioned.union(&poly);
    }
    
    let a_floor = unioned.unsigned_area();
    let mut perimeter = 0.0;
    
    let mut walls_n = 0.0;
    let mut walls_e = 0.0;
    let mut walls_s = 0.0;
    let mut walls_w = 0.0;

    for p in unioned.iter() {
        let coords: Vec<_> = p.exterior().0.iter().collect();
        for i in 0..(coords.len().saturating_sub(1)) {
            let dx = coords[i+1].x - coords[i].x;
            let dy = coords[i+1].y - coords[i].y;
            let length = (dx * dx + dy * dy).sqrt();
            perimeter += length;
            
            if length < 0.001 { continue; }
            
            let rot_rad = -state.params.building_rotation_deg.to_radians();
            let dx_rot = dx * rot_rad.cos() - dy * rot_rad.sin();
            let dy_rot = dx * rot_rad.sin() + dy * rot_rad.cos();
            
            let normal_x = dy_rot;
            let normal_y = -dx_rot;
            
            let mut angle_deg = normal_x.atan2(normal_y).to_degrees();
            if angle_deg < 0.0 { angle_deg += 360.0; }
            
            if angle_deg >= 315.0 || angle_deg < 45.0 {
                walls_n += length;
            } else if angle_deg >= 45.0 && angle_deg < 135.0 {
                walls_e += length;
            } else if angle_deg >= 135.0 && angle_deg < 225.0 {
                walls_s += length;
            } else {
                walls_w += length;
            }
        }
    }
    
    let a_floor_total = a_floor * state.params.num_stories as f64;
    let a_wall = perimeter * state.params.story_height * state.params.num_stories as f64;

    let mut a_window = 0.0;
    let mut win_n = 0.0;
    let mut win_e = 0.0;
    let mut win_s = 0.0;
    let mut win_w = 0.0;

    let rot_rad = -state.params.building_rotation_deg.to_radians();

    for z in &state.zones {
        for w in &z.windows {
            let w_area = w.width * w.height * state.params.num_stories as f64; // assuming windows span stories or are defined per story? Let's just multiply by num_stories to be consistent with walls. Or maybe windows are absolute. Let's just use w_area. Wait, if it's 3D, and the user draws on one face... Let's just take w.width * w.height.
            let mut true_w_area = w.width * w.height;
            a_window += true_w_area;
            
            // Base normal of the local wall
            let (nx, ny) = match w.wall_id.as_str() {
                "N" => (0.0, 1.0),
                "E" => (1.0, 0.0),
                "S" => (0.0, -1.0),
                "W" => (-1.0, 0.0),
                _ => (0.0, 1.0), // default to N
            };
            
            // Rotate the normal
            let nx_rot = nx * rot_rad.cos() - ny * rot_rad.sin();
            let ny_rot = nx * rot_rad.sin() + ny * rot_rad.cos();
            
            let mut angle_deg = nx_rot.atan2(ny_rot).to_degrees();
            if angle_deg < 0.0 { angle_deg += 360.0; }
            
            if angle_deg >= 315.0 || angle_deg < 45.0 {
                win_n += true_w_area;
            } else if angle_deg >= 45.0 && angle_deg < 135.0 {
                win_e += true_w_area;
            } else if angle_deg >= 135.0 && angle_deg < 225.0 {
                win_s += true_w_area;
            } else {
                win_w += true_w_area;
            }
        }
    }

    let net_wall = a_wall - a_window;
    let a_roof = a_floor;
    let a_ground = a_floor;

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

    let h_ve = 0.34 * (0.4 + n_infiltr) * a_floor_total * 2.5;
    
    // ISO 13790
    let theta_int = 20.0;
    let theta_e = 4.4;
    let d_hs = 222.0;

    let q_ht = 0.024 * (h_tr + h_ve) * 1.0 * (theta_int - theta_e) * d_hs;
    let q_ht_tr = 0.024 * h_tr * 1.0 * (theta_int - theta_e) * d_hs;
    let q_ht_ve = 0.024 * h_ve * 1.0 * (theta_int - theta_e) * d_hs;
    
    let h_story = state.params.story_height * state.params.num_stories as f64;
    // win_n, win_e, win_s, win_w are already calculated.
    
    let sol_factor = 0.6 * (1.0 - 0.3) * 0.9 * g_value;
    let q_sol_n = sol_factor * win_n * 160.0;
    let q_sol_e = sol_factor * win_e * 271.0;
    let q_sol_s = sol_factor * win_s * 392.0;
    let q_sol_w = sol_factor * win_w * 271.0;
    let q_sol = q_sol_n + q_sol_e + q_sol_s + q_sol_w;
    
    let q_int = 0.0528 * d_hs * a_floor_total;
    let q_gn = q_sol + q_int;
    let q_h_nd = f64::max(0.0, q_ht - 0.95 * q_gn);
    
    let eff = 0.9;
    let q_final = q_h_nd / eff;

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
            "solar_gains_kWh_a": q_sol
        },
        "heating_demand": {
            "Q_H_nd_kWh_a": q_h_nd,
            "specific_Q_H_nd_kWh_m2a": q_h_nd / a_floor_total
        },
        "final_energy": {
            "Q_final_kWh_a": q_final,
            "specific_Q_final_kWh_m2a": q_final / a_floor_total
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
    if let Ok(new_state) = serde_json::from_str::<State>(state_json) {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            store.set_state(new_state);
            let res = calculate_energy(store.current());
            serde_json::to_string(&res).unwrap()
        })
    } else {
        serde_json::json!({ "status": "error", "message": "Invalid state payload" }).to_string()
    }
}

#[wasm_bindgen]
pub fn add_zone(zone_json: &str) -> String {
    if let Ok(zone) = serde_json::from_str::<Zone>(zone_json) {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let mut new_state = store.current().clone();
            new_state.zones.push(zone.clone());
            store.apply_action(new_state, Action::AddZone(zone), "Added new zone");
            let res = calculate_energy(store.current());
            serde_json::to_string(&res).unwrap()
        })
    } else {
        serde_json::json!({ "status": "error", "message": "Invalid zone payload" }).to_string()
    }
}

#[wasm_bindgen]
pub fn remove_zone(zone_id: &str) -> String {
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        let mut new_state = store.current().clone();
        new_state.zones.retain(|z| z.id != zone_id);
        store.apply_action(new_state, Action::RemoveZone(zone_id.to_string()), &format!("Removed zone {}", zone_id));
        let res = calculate_energy(store.current());
        serde_json::to_string(&res).unwrap()
    })
}

#[wasm_bindgen]
pub fn update_zone(zone_json: &str) -> String {
    if let Ok(updated_zone) = serde_json::from_str::<Zone>(zone_json) {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let mut new_state = store.current().clone();
            if let Some(pos) = new_state.zones.iter().position(|z| z.id == updated_zone.id) {
                new_state.zones[pos] = updated_zone.clone();
                store.apply_action(new_state, Action::UpdateZone(updated_zone), "Updated zone");
                let res = calculate_energy(store.current());
                serde_json::to_string(&res).unwrap()
            } else {
                serde_json::json!({ "status": "error", "message": "Zone not found" }).to_string()
            }
        })
    } else {
        serde_json::json!({ "status": "error", "message": "Invalid zone payload" }).to_string()
    }
}

#[wasm_bindgen]
pub fn update_parameters(params_json: &str) -> String {
    if let Ok(params) = serde_json::from_str::<Parameters>(params_json) {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let mut new_state = store.current().clone();
            new_state.params = params.clone();
            store.apply_action(new_state, Action::UpdateParameters(params), "Updated parameters");
            let res = calculate_energy(store.current());
            serde_json::to_string(&res).unwrap()
        })
    } else {
        serde_json::json!({ "status": "error", "message": "Invalid parameters payload" }).to_string()
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
