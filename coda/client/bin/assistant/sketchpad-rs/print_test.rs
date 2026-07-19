use sketchpad_rs::*;
use serde_json::json;

fn main() {
    let mut params = Parameters::default();
    params.building_type = "SFH".to_string();
    params.year_class = "2016-...".to_string();
    params.scenario = "Existing State".to_string();
    params.story_height = 2.8;
    params.num_stories = 1;
    params.window_to_wall_ratio = 0.15;
    params.building_rotation_deg = 0.0;
    params.heating_system = "Gas Condensing Boiler".to_string();
    params.climate_region = "Potsdam".to_string();
    params.usage_profile = "Residential".to_string();

    let mut geo = BuildingGeometry::default();
    geo.total_floor_area = 100.0;
    geo.total_roof_area = 100.0;
    geo.total_ground_area = 100.0;
    geo.total_conditioned_volume = 280.0;
    geo.exterior_perimeter = 40.0;
    geo.envelope_data.n.gross_wall_area = 28.0;
    geo.envelope_data.e.gross_wall_area = 28.0;
    geo.envelope_data.s.gross_wall_area = 28.0;
    geo.envelope_data.w.gross_wall_area = 28.0;
    geo.windows.push(WindowGeometry {
        area: 4.2,
        orientation: 180.0,
        tilt: 90.0,
        g_value: 0.6,
        frame_factor: 0.3,
        shading_factor: 1.0,
    });

    let ui_state = json!({
        "raw_zones": [{
            "type": "Residential",
            "geometry": { "x": 0.0, "y": 0.0, "width": 10.0, "length": 10.0 },
            "profile": "Residential"
        }]
    });

    let state = State { geometry: Some(geo), params, ui_state: Some(ui_state) };
    let result = calculate_energy(&state).unwrap();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
