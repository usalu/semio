use super::*;
use semio_framework_value::ToValue;

fn json(value: &impl ToValue) -> String {
    semio_framework_pack_json::to_json_string(value)
}

#[test]
fn a_unit_variant_is_its_name_and_an_interzone_boundary_names_its_partner() {
    assert_eq!(json(&OutsideBoundary::OutdoorAir), "\"OutdoorAir\"");
    assert_eq!(json(&OutsideBoundary::Interzone(3)), "{\"Interzone\":3}");
    assert_eq!(json(&SurfaceClass::ExteriorWall), "\"ExteriorWall\"");
    assert_eq!(json(&SurfaceRoughness::MediumRough), "\"MediumRough\"");
}

#[test]
fn an_empty_model_has_every_member_of_the_engine_model_in_its_order() {
    let text = json(&EnergyModel::new("M", "v", Site::default()));
    let members = ["name", "version", "site", "zones", "spaces", "surfaces", "fenestrations", "materials", "glazing_materials", "gas_materials", "constructions", "people", "lighting", "equipment", "thermostats", "humidistats", "setpoint_managers", "ideal_loads", "zone_equipment", "air_loops", "plant_loops", "outdoor_air_systems", "infiltrations", "mechanical_ventilations", "shading_surfaces", "space_lists", "thermal_enclosures", "adjacency_pairs", "airflow_network", "electrical_load_centers", "pv_systems", "battery_storage", "shw_systems", "solar_thermal_systems", "refrigeration_systems", "water_systems", "faults", "output_variables", "sizing_objects", "daylight_zones", "room_air_models", "ground_temperature", "run_period", "schedules"];
    let positions: Vec<usize> = members.iter().map(|member| text.find(&format!("\"{member}\":")).unwrap_or_else(|| panic!("{member} is written"))).collect();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "the members follow the engine's order");
    assert!(text.contains("\"airflow_network\":null"), "an absent option is null");
}

#[test]
fn a_fenestration_without_a_glazing_construction_writes_null() {
    let window = Fenestration { id: 1, name: "w".into(), surface_id: 2, u_value_w_m2k: 1.1, shgc: 0.4, vlt: 0.4, area_m2: 1.0, height_m: 1.0, sill_height_m: 0.9, frame_conductance_w_k: 0.0, divider_conductance_w_k: 0.0, overhang_depth_m: 0.0, overhang_offset_m: 0.0, fin_depth_m: 0.0, fin_offset_m: 0.0, glazing_construction_id: None, vertices_m: vec![[0.0, 0.0, 0.0]] };
    let text = json(&window);
    assert!(text.contains("\"glazing_construction_id\":null") && text.contains("\"vertices_m\":[[0"), "{text}");
}
