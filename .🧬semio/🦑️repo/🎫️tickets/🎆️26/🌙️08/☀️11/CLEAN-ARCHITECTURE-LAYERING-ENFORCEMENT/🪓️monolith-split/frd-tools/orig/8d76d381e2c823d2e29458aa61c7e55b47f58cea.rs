//! 🧰️ Energy mutation fixture laws: the committed-vector case shape and the eight laws every vector is held to, the
//! roster the language-neutral adapter mirrors, and the codec probe corpus. Its `[[test]]` targets run them.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_artifact_energy_model_mut_air_loop as air_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_battery as battery_mutations;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_mut_construction as construction_mutations;
extern crate semio_s_artifact_energy_model_mut_day_schedule as day_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_daylight_zone as daylight_zone_mutations;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_mut_enclosure as enclosure_mutations;
extern crate semio_s_artifact_energy_model_mut_equipment_gain as equipment_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_fault as fault_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_finish as finish_mutations;
extern crate semio_s_artifact_energy_model_mut_fixed_schedule as fixed_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_glazing as glazing_mutations;
extern crate semio_s_artifact_energy_model_mut_humidistat as humidistat_mutations;
extern crate semio_s_artifact_energy_model_mut_ideal_loads as ideal_loads_mutations;
extern crate semio_s_artifact_energy_model_mut_infiltration as infiltration_mutations;
extern crate semio_s_artifact_energy_model_mut_leakage as leakage_mutations;
extern crate semio_s_artifact_energy_model_mut_lighting_gain as lighting_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_load_center as load_center_mutations;
extern crate semio_s_artifact_energy_model_mut_material as material_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mut_model as model_mutations;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_outdoor_air as outdoor_air_mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain as people_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_plant_loop as plant_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system as pv_system_mutations;
extern crate semio_s_artifact_energy_model_mut_refrigeration as refrigeration_mutations;
extern crate semio_s_artifact_energy_model_mut_room_air as room_air_mutations;
extern crate semio_s_artifact_energy_model_mut_run as run_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_setpoint as setpoint_mutations;
extern crate semio_s_artifact_energy_model_mut_shading_device as shading_device_mutations;
extern crate semio_s_artifact_energy_model_mut_shading as shading_mutations;
extern crate semio_s_artifact_energy_model_mut_shw_system as shw_system_mutations;
extern crate semio_s_artifact_energy_model_mut_site as site_mutations;
extern crate semio_s_artifact_energy_model_mut_sizing as sizing_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_solar_thermal as solar_thermal_mutations;
extern crate semio_s_artifact_energy_model_mut_space_list as space_list_mutations;
extern crate semio_s_artifact_energy_model_mut_space as space_mutations;
extern crate semio_s_artifact_energy_model_mut_surface as surface_mutations;
extern crate semio_s_artifact_energy_model_mut_thermostat as thermostat_mutations;
extern crate semio_s_artifact_energy_model_mut_time_series as time_series_mutations;
extern crate semio_s_artifact_energy_model_mut_ventilation as ventilation_mutations;
extern crate semio_s_artifact_energy_model_mut_water_system as water_system_mutations;
extern crate semio_s_artifact_energy_model_mut_week_schedule as week_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer as window_layer_mutations;
extern crate semio_s_artifact_energy_model_mut_year_schedule as year_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_zone_equipment as zone_equipment_mutations;
extern crate semio_s_artifact_energy_model_mut_zone as zone_mutations;

use crate::air_loop_mutations::{add_air_loop_terminal_zone, change_air_loop_design_supply_air_flow, change_air_loop_return_node, change_air_loop_supply_node, create_air_loop, delete_air_loop, remove_air_loop_terminal_zone, rename_air_loop};
use crate::battery_mutations::{change_battery_capacity, change_battery_max_charge, change_battery_max_discharge, change_battery_round_trip_efficiency, create_battery, delete_battery};
use crate::boundary_mutations::{change_surface_boundary_condition, change_surface_sun_exposed, change_surface_wind_exposed, connect_surfaces, disconnect_surfaces};
use crate::construction_mutations::{add_construction_layer, create_construction, delete_construction, remove_construction_layer, rename_construction, reorder_construction_layers};
use crate::day_schedule_mutations::{change_daily_schedule_interpolation, change_daily_schedule_limits, create_daily_schedule, delete_daily_schedule, replace_daily_schedule_hourly_values};
use crate::daylight_zone_mutations::{change_daylight_zone_glare_limit, change_daylight_zone_illuminance_target, change_daylight_zone_window_transmittance, change_daylight_zone_zone, create_daylight_zone, delete_daylight_zone};
use crate::enclosure_mutations::{add_thermal_enclosure_zone, create_thermal_enclosure, delete_thermal_enclosure, remove_thermal_enclosure_zone, rename_thermal_enclosure};
use crate::equipment_gain_mutations::{change_equipment_gain_latent_fraction, change_equipment_gain_radiant_fraction, change_equipment_gain_schedule, change_equipment_gain_watts_per_area, change_equipment_gain_zone, create_equipment_gain, delete_equipment_gain};
use crate::fault_mutations::{change_fault_severity, change_fault_start_schedule, change_fault_target_equipment, change_fault_type, create_fault, delete_fault};
use crate::fenestration_mutations::{change_fenestration_area, change_fenestration_height, change_fenestration_sill_height, change_fenestration_surface, create_fenestration, delete_fenestration, rename_fenestration, replace_fenestration_vertices};
use crate::finish_mutations::{change_material_roughness, change_material_solar_absorptance, change_material_thermal_absorptance, change_material_visible_absorptance};
use crate::fixed_schedule_mutations::{change_constant_schedule_value, create_constant_schedule, delete_constant_schedule};
use crate::glazing_mutations::{bind_fenestration_glazing_construction, change_fenestration_divider_conductance, change_fenestration_frame_conductance, change_fenestration_shgc, change_fenestration_u_value, change_fenestration_vlt, clear_fenestration_glazing_construction};
use crate::humidistat_mutations::{change_humidistat_dehumidifying_setpoint_schedule, change_humidistat_dehumidifying_throttle_range, change_humidistat_humidifying_setpoint_schedule, change_humidistat_humidifying_throttle_range, change_humidistat_zone, create_humidistat, delete_humidistat};
use crate::ideal_loads_mutations::{change_ideal_loads_system_max_cooling_capacity, change_ideal_loads_system_max_heating_capacity, change_ideal_loads_system_max_heating_supply_air_temp, change_ideal_loads_system_min_cooling_supply_air_temp, change_ideal_loads_system_outdoor_air_per_area, change_ideal_loads_system_outdoor_air_per_person, change_ideal_loads_system_zone, create_ideal_loads_system, delete_ideal_loads_system};
use crate::infiltration_mutations::{change_infiltration_design_flow_ach, change_infiltration_flow_per_exterior_area, change_infiltration_method, change_infiltration_schedule, change_infiltration_zone, create_infiltration, delete_infiltration};
use crate::leakage_mutations::{change_infiltration_constant_term_coefficient, change_infiltration_discharge_coefficient, change_infiltration_effective_leakage_area, change_infiltration_stack_height, change_infiltration_temperature_term_coefficient, change_infiltration_velocity_squared_term_coefficient, change_infiltration_velocity_term_coefficient};
use crate::lighting_gain_mutations::{change_lighting_gain_radiant_fraction, change_lighting_gain_return_air_fraction, change_lighting_gain_schedule, change_lighting_gain_visible_fraction, change_lighting_gain_watts_per_area, change_lighting_gain_zone, create_lighting_gain, delete_lighting_gain};
use crate::load_center_mutations::{add_electrical_load_center_battery, add_electrical_load_center_pv, create_electrical_load_center, delete_electrical_load_center, remove_electrical_load_center_battery, remove_electrical_load_center_pv, rename_electrical_load_center};
use crate::material_mutations::{change_material_conductivity, change_material_density, change_material_specific_heat, change_material_thickness, create_material, delete_material, rename_material};
use crate::model_mutations::{add_output_variable, bind_weather_file, change_model_version, connect_referenced_model, disconnect_referenced_model, remove_output_variable, rename_model, replace_airflow_network, unbind_weather_file};
use crate::outdoor_air_mutations::{change_outdoor_air_system_air_loop, change_outdoor_air_system_economizer_enabled, change_outdoor_air_system_min_oa_flow, create_outdoor_air_system, delete_outdoor_air_system};
use crate::people_gain_mutations::{change_people_gain_activity_schedule, change_people_gain_latent_fraction, change_people_gain_people_per_area, change_people_gain_radiant_fraction, change_people_gain_schedule, change_people_gain_sensible_fraction, change_people_gain_zone, create_people_gain, delete_people_gain};
use crate::plant_loop_mutations::{add_plant_loop_equipment, change_plant_loop_design_flow, change_plant_loop_return_temperature, change_plant_loop_supply_temperature, change_plant_loop_type, create_plant_loop, delete_plant_loop, remove_plant_loop_equipment, rename_plant_loop};
use crate::pv_system_mutations::{change_pv_system_area, change_pv_system_azimuth, change_pv_system_dc_capacity, change_pv_system_inverter_efficiency, change_pv_system_module_efficiency, change_pv_system_tilt, create_pv_system, delete_pv_system};
use crate::refrigeration_mutations::{change_refrigeration_system_case_count, change_refrigeration_system_defrost_schedule, change_refrigeration_system_design_load, create_refrigeration_system, delete_refrigeration_system};
use crate::room_air_mutations::{change_room_air_model, create_room_air_model_assignment, delete_room_air_model_assignment};
use crate::run_mutations::{change_run_end_day, change_run_end_month, change_run_start_day, change_run_start_month, change_run_year};
use crate::setpoint_mutations::{change_setpoint_manager_schedule, create_setpoint_manager, delete_setpoint_manager, rename_setpoint_manager, replace_setpoint_manager_kind};
use crate::shading_mutations::{change_shading_surface_transmittance_schedule, create_shading_surface, delete_shading_surface, rename_shading_surface, replace_shading_surface_vertices};
use crate::shading_device_mutations::{change_fenestration_fin_depth, change_fenestration_fin_offset, change_fenestration_overhang_depth, change_fenestration_overhang_offset};
use crate::shw_system_mutations::{change_shw_system_heater_capacity, change_shw_system_schedule, change_shw_system_setpoint, change_shw_system_storage_volume, create_shw_system, delete_shw_system};
use crate::site_mutations::{change_ground_building, change_ground_deep, change_ground_shallow, change_site_elevation, change_site_latitude, change_site_longitude, change_site_north_axis, change_site_time_zone};
use crate::sizing_mutations::{change_sizing_object_design_day_type, change_sizing_object_sizing_type, change_sizing_object_zone, create_sizing_object, delete_sizing_object};
use crate::solar_thermal_mutations::{change_solar_thermal_system_azimuth, change_solar_thermal_system_collector_area, change_solar_thermal_system_efficiency, change_solar_thermal_system_storage_volume, change_solar_thermal_system_tilt, create_solar_thermal_system, delete_solar_thermal_system};
use crate::space_mutations::{change_space_floor_area, change_space_zone, create_space, delete_space, rename_space};
use crate::space_list_mutations::{add_space_list_member, create_space_list, delete_space_list, remove_space_list_member, rename_space_list};
use crate::surface_mutations::{change_surface_class, change_surface_construction, change_surface_multiplier, change_surface_zone, create_surface, delete_surface, rename_surface, replace_surface_vertices};
use crate::thermostat_mutations::{change_thermostat_cooling_setpoint_schedule, change_thermostat_cooling_throttle_range, change_thermostat_heating_setpoint_schedule, change_thermostat_heating_throttle_range, change_thermostat_zone, create_thermostat, delete_thermostat};
use crate::time_series_mutations::{change_time_series_schedule_timestep, create_time_series_schedule, delete_time_series_schedule, replace_time_series_schedule_values};
use crate::ventilation_mutations::{change_mechanical_ventilation_design_flow, change_mechanical_ventilation_fan_delta_pressure, change_mechanical_ventilation_fan_total_efficiency, change_mechanical_ventilation_schedule, change_mechanical_ventilation_zone, create_mechanical_ventilation, delete_mechanical_ventilation};
use crate::water_system_mutations::{change_water_system_fixture_count, change_water_system_peak_flow, change_water_system_schedule, create_water_system, delete_water_system};
use crate::week_schedule_mutations::{change_weekly_schedule_day, create_weekly_schedule, delete_weekly_schedule};
use crate::window_layer_mutations::{change_gas_material_gas, change_gas_material_thickness, change_glazing_material_conductivity, change_glazing_material_infrared_emissivity, change_glazing_material_solar_transmittance, change_glazing_material_thickness, change_glazing_material_visible_transmittance, rename_gas_material, rename_glazing_material};
use crate::year_schedule_mutations::{add_annual_schedule_holiday, change_annual_schedule_default_daily_schedule, change_annual_schedule_holiday_daily_schedule, create_annual_schedule, delete_annual_schedule, insert_annual_schedule_rule, remove_annual_schedule_holiday, remove_annual_schedule_rule, reorder_annual_schedule_rules};
use crate::zone_mutations::{change_zone_conditioned, change_zone_floor_area_participation, change_zone_multiplier, change_zone_volume, create_zone, delete_zone, rename_zone};
use crate::zone_equipment_mutations::{change_zone_equipment_cooling_capacity, change_zone_equipment_heating_capacity, change_zone_equipment_priority, change_zone_equipment_type, change_zone_equipment_zone, create_zone_equipment, delete_zone_equipment};
use crate::mutations::EnergyModelMutation;
use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use protocol::{Mutation, SemanticMutation};
use semio_framework_value::ToValue;

/// 🧫️ One committed specification vector and the typed scenario it was generated from.
pub struct Case {
    pub kind: &'static str,
    pub directory: &'static str,
    pub before: String,
    pub after: String,
    pub mutation: String,
    pub diff: String,
    pub outcome: String,
    pub scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation),
}

impl Case {
    /// 🧫️ Reads one committed `(before, mutation, after, diff, outcome)` vector from its fixture bundle at run time; a leaf
    /// not yet materialized reads empty documents, which every law then refuses by name.
    pub fn load(kind: &'static str, directory: &'static str, scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation)) -> Self {
        let bundle = fixture_root().join(directory);
        let read = |leaf: &str| std::fs::read_to_string(bundle.join(leaf)).unwrap_or_default();
        Case {
            kind,
            directory,
            before: read("📸️snapshot/⬅️before/🔣️.json"),
            after: read("📸️snapshot/➡️after/🔣️.json"),
            mutation: read("🦠️mutation/🔣️.json"),
            diff: read("🔺️diff/🔣️.json"),
            outcome: read("🎯️outcome/🔣️.json"),
            scenario,
        }
    }
}

/// 📁️ The committed `🧫️fixtures/🧬️mutations/<domain>/<operation>/<case>` tree, resolved from this crate's manifest.
pub fn fixture_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations")
}

/// 🏠️ One conditioned zone with the BESTEST 600 envelope's own air volume.
pub fn zone(id: u32, name: &str) -> crate::model::Zone {
    crate::model::Zone { id: crate::model::EntityId(id), name: name.to_string(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

/// 📸️ The persisted snapshot a typed model lives in, both composed child handles minted.
pub fn snapshot(model: crate::model::Model) -> EnergyModelSnapshot {
    crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)
}

/// 🔗️ A head-pinned forward link to another artifact.
pub fn link(uri: &str, role: &str) -> store::ArtifactLink {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    store::ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef::parse_uri(uri).expect("fixture link uri parses"), pin: store::LinkPin::Head, role: role.to_string() }
}

//#region 🧰️G1Constructors
/// 🪑️ One space inside a zone.
pub fn space(id: u32, name: &str, zone_id: u32) -> crate::model::Space {
    crate::model::Space { id: crate::model::EntityId(id), name: name.to_string(), zone_id: crate::model::EntityId(zone_id), floor_area_m2: 48.0 }
}

/// 🧱️ One opaque material layer — the BESTEST 600 lightweight wall's plasterboard.
pub fn material(id: u32, name: &str) -> crate::model::Material {
    crate::model::Material {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        roughness: crate::model::SurfaceRoughness::MediumRough,
        thickness_m: 0.012,
        conductivity_w_m_k: 0.16,
        density_kg_m3: 950.0,
        specific_heat_j_kg_k: 840.0,
        thermal_absorptance: 0.9,
        solar_absorptance: 0.6,
        visible_absorptance: 0.6,
    }
}

/// 🪟️ One uncoated 3 mm clear pane, the outer sheet of ANSI/ASHRAE 140 §5.2's double glazing.
pub fn glazing_material(id: u32, name: &str) -> crate::model::GlazingMaterial {
    crate::model::GlazingMaterial {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        thickness_m: 0.003,
        conductivity_w_m_k: 1.06,
        solar_transmittance: 0.837,
        solar_reflectance_front: 0.075,
        solar_reflectance_back: 0.075,
        visible_transmittance: 0.898,
        visible_reflectance_front: 0.081,
        visible_reflectance_back: 0.081,
        infrared_transmittance: 0.0,
        infrared_emissivity_front: 0.84,
        infrared_emissivity_back: 0.84,
    }
}

/// 🌫️ One 13 mm air gap between two panes.
pub fn gas_material(id: u32, name: &str) -> crate::model::GasMaterial {
    crate::model::GasMaterial { id: crate::model::EntityId(id), name: name.to_string(), thickness_m: 0.013, gas: crate::model::GasKind::Air }
}

/// 🧱️ One single-layer construction over `material_id`.
pub fn construction(id: u32, name: &str, material_id: u32) -> crate::model::Construction {
    crate::model::Construction { id: crate::model::EntityId(id), name: name.to_string(), layer_material_ids: vec![crate::model::EntityId(material_id)] }
}

/// 📐️ One sun- and wind-exposed 8 m × 2.7 m exterior wall on `zone_id`.
pub fn surface(id: u32, name: &str, zone_id: u32, construction_id: u32) -> crate::model::Surface {
    crate::model::Surface {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        zone_id: crate::model::EntityId(zone_id),
        class: crate::model::SurfaceClass::ExteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]],
        construction_id: crate::model::EntityId(construction_id),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    }
}

/// 🪟️ One 3 m × 2 m window carrying ANSI/ASHRAE 140 §5.2's own quoted optics.
pub fn window(id: u32, name: &str, surface_id: u32) -> crate::model::Fenestration {
    crate::model::Fenestration {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        surface_id: crate::model::EntityId(surface_id),
        u_value_w_m2k: 3.0,
        shgc: 0.787,
        vlt: 0.86,
        area_m2: 6.0,
        height_m: 2.0,
        sill_height_m: 0.5,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
        vertices_m: Vec::new(),
    }
}

/// 🌳️ One free-standing site shading surface.
pub fn shading(id: u32, name: &str) -> crate::model::ShadingSurface {
    crate::model::ShadingSurface { id: crate::model::EntityId(id), name: name.to_string(), vertices_m: vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]], transmittance_schedule_id: None }
}
//#endregion 🧰️G1Constructors

fn committed(case: &Case, label: &str, text: &str) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}/{}: committed {label} is not valid JSON: {error}", case.kind, case.directory))
}

fn decode(case: &Case, label: &str, text: &str) -> EnergyModelSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}/{}: committed {label} does not decode: {error}", case.kind, case.directory))
}

fn built(case: &Case) -> protocol::MutationOutcome<EnergyModelDiff> {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&mutation, &base)
}

fn level_name(level: semio_framework_diagnostic::Severity) -> &'static str {
    match level {
        semio_framework_diagnostic::Severity::Info => "info",
        semio_framework_diagnostic::Severity::Warning => "warning",
        semio_framework_diagnostic::Severity::Error => "error",
        semio_framework_diagnostic::Severity::Fatal => "fatal",
    }
}

/// 🎯️ The declared outcome document a produced `MutationOutcome` corresponds to, in the outcome-class vocabulary
/// `applied | no-op | rejected`: a refusal names one fault code and the offending address, an application or a no-op
/// (the `mutation.no-op` warning) names its ordered message list.
fn outcome_document(outcome: &protocol::MutationOutcome<EnergyModelDiff>) -> semio_framework_pack_json::Value {
    let rejected = outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error);
    if rejected {
        let first = outcome.messages().iter().find(|message| message.level >= semio_framework_diagnostic::Severity::Error).expect("a refusal carries its own message");
        return semio_framework_pack_json::object([
            ("status".to_string(), semio_framework_pack_json::Value::String("rejected".to_string())),
            ("code".to_string(), semio_framework_pack_json::Value::String(first.code.0.clone())),
            ("path".to_string(), semio_framework_pack_json::Value::Array(first.target.iter().map(|entry| semio_framework_pack_json::Value::String(entry.clone())).collect())),
        ]);
    }
    let no_op = outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op");
    semio_framework_pack_json::object([
        ("status".to_string(), semio_framework_pack_json::Value::String(if no_op { "no-op" } else { "applied" }.to_string())),
        (
            "messages".to_string(),
            semio_framework_pack_json::Value::Array(
                outcome.messages().iter().map(|message| semio_framework_pack_json::object([("level".to_string(), semio_framework_pack_json::Value::String(level_name(message.level).to_string())), ("code".to_string(), semio_framework_pack_json::Value::String(message.code.0.clone()))])).collect(),
            ),
        ),
    ])
}

fn write_file(path: std::path::PathBuf, text: &str) {
    std::fs::create_dir_all(path.parent().expect("fixture file has a parent")).expect("fixture directory is writable");
    std::fs::write(path, format!("{text}\n")).expect("fixture file is writable");
}

/// 🏗️ Materializes the five committed JSON files from the typed scenario. Off unless
/// `SEMIO_ENERGY_WRITE_FIXTURES=1`, so an ordinary test run never touches the source tree.
/// The root is the SAME `🧫️fixtures/🧬️mutations/<kind>/<case>` tree every leaf test
/// `include_str!`s from (`../../../../../🧫️fixtures/…` relative to the leaf) — the writer once
/// pointed at `🧬️schema/🧬️mutations/…`, which left every committed quintet a `{}` stub.
pub fn write_when_requested(case: &Case) {
    if std::env::var("SEMIO_ENERGY_WRITE_FIXTURES").ok().as_deref() != Some("1") {
        return;
    }
    let (before, mutation) = (case.scenario)();
    let outcome = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&mutation, &before);
    let after = protocol::apply_diff(outcome.diff(), &before).expect("the scenario's forward diff applies");
    let root = fixture_root().join(case.directory);
    let json = |value: semio_framework_pack_json::Value| semio_framework_pack_json::to_string_pretty(&value);
    write_file(root.join("📸️snapshot/⬅️before/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&before.to_value())));
    write_file(root.join("📸️snapshot/➡️after/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&after.to_value())));
    write_file(root.join("🦠️mutation/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&mutation.to_value())));
    write_file(root.join("🔺️diff/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&outcome.diff().to_value())));
    let mut document = outcome_document(&outcome);
    if let (Some(invariant), Some(object)) = (committed_outcome(case).1, document.as_object_mut()) {
        object.insert("invariant", semio_framework_pack_json::Value::String(invariant));
    }
    write_file(root.join("🎯️outcome/🔣️.json"), &json(document));
}

/// 📖️ The committed before-snapshot and mutation of one vector, decoded — the inputs a leaf hands the framework's inverse-sum law.
pub fn committed(case: &Case) -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    (decode(case, "before-snapshot", case.before.as_str()), mutation)
}

/// ▶️ Applying the committed mutation to the committed before-snapshot reproduces the committed
/// after-snapshot exactly, composed child handles included.
pub fn assert_forward(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let applied = protocol::apply_diff(built(case).diff(), &base).expect("committed mutation applies to its committed before-snapshot");
    assert_eq!(applied, decode(case, "after-snapshot", case.after.as_str()), "{}/{}: the applied document is not the committed after-snapshot", case.kind, case.directory);
    assert_eq!((applied.structure.child_id.as_str(), applied.zones.child_id.as_str()), (base.structure.child_id.as_str(), base.zones.child_id.as_str()), "{}/{}: structure and zones must stay one scene", case.kind, case.directory);
}

/// ↩️ Applying the committed mutation and then its OWN computed inverse restores the committed
/// before-snapshot; an inverse step that is itself refused fails here rather than passing quietly.
/// The steps are replayed in REVERSE order — exactly as `ArtifactStore::replay_mutations`
/// (`back.reverse()`) and the framework's `assert_mutation_inverse_law` do — so a multi-step inverse
/// that only works forwards (parent re-created first) is caught here, not in the shell's undo.
pub fn assert_inverse(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let mut snapshot = protocol::apply_diff(built(case).diff(), &base).expect("committed mutation applies");
    for step in <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").into_iter().rev() {
        let outcome = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&step, &snapshot);
        assert!(!outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error), "{}/{}: an inverse step was itself refused", case.kind, case.directory);
        snapshot = protocol::apply_diff(outcome.diff(), &snapshot).expect("the inverse step applies");
    }
    assert_eq!(snapshot, base, "{}/{}: undoing did not land back on the committed before-snapshot", case.kind, case.directory);
}

/// 🔣️ Both committed documents and the committed payload re-encode to themselves.
pub fn assert_canonical(case: &Case) {
    for (label, text) in [("before", case.before.as_str()), ("after", case.after.as_str())] {
        let decoded = decode(case, label, text);
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &committed(case, label, text)), "{}/{}: committed {label} snapshot is not canonical", case.kind, case.directory);
    }
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&mutation.to_value());
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &committed(case, "mutation", case.mutation.as_str())), "{}/{}: committed mutation payload is not canonical", case.kind, case.directory);
}

/// 🧷️ The payload-intrinsic invariant ids the committed mutation's own leaf payload schema declares in `x-semio-invariant`.
fn declared_invariants(case: &Case) -> Vec<String> {
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let schema = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::input_schema(&mutation).expect("every energy leaf publishes its payload schema");
    let document = semio_framework_pack_json::parse(schema, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("leaf payload schema is valid JSON");
    document.get("x-semio-invariant").and_then(semio_framework_pack_json::Value::as_array).map(|rows| rows.iter().filter_map(|row| row.get("id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string)).collect()).unwrap_or_default()
}

/// 🧷️ The committed outcome split into the part the implementation produces and the `invariant` id a payload-intrinsic
/// refusal names — a rule the produced message cannot carry, which the leaf schema declares in `x-semio-invariant`.
fn committed_outcome(case: &Case) -> (semio_framework_pack_json::Value, Option<String>) {
    let mut declared = committed(case, "outcome", case.outcome.as_str());
    let invariant = declared.as_object_mut().and_then(|object| object.remove("invariant")).map(|value| value.as_str().expect("an outcome's invariant is an id").to_string());
    (declared, invariant)
}

/// 🎯️ The diagnostics the implementation raises are the ones the committed outcome declares; a named invariant is a
/// `mutation.invariant` refusal whose id the leaf's payload schema declares.
pub fn assert_outcome(case: &Case) {
    let produced = outcome_document(&built(case));
    let (declared, invariant) = committed_outcome(case);
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced outcome {produced:?} differs from the committed one {declared:?}", case.kind, case.directory);
    if let Some(invariant) = invariant {
        assert_eq!(produced["code"].as_str(), Some("mutation.invariant"), "{}/{}: only a mutation.invariant refusal names an invariant", case.kind, case.directory);
        assert!(declared_invariants(case).contains(&invariant), "{}/{}: the leaf payload schema declares no x-semio-invariant {invariant:?}", case.kind, case.directory);
    }
}

/// 🔺️ The produced delta is the committed delta — which pins WHICH fields the kind may touch,
/// not merely where the document ended up.
pub fn assert_diff(case: &Case) {
    let produced = semio_framework_pack_json::from_dsl_value(&built(case).diff().to_value());
    let declared = committed(case, "diff", case.diff.as_str());
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced diff {produced:?} differs from the committed 🔺️diff/🔣️.json {declared:?}", case.kind, case.directory);
}

/// 🔣️ The committed delta decodes to the real `EnergyModelDiff` and re-encodes unchanged.
pub fn assert_diff_canonical(case: &Case) {
    let decoded: EnergyModelDiff = semio_framework_pack_json::from_json_str(case.diff.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &committed(case, "diff", case.diff.as_str())), "{}/{}: committed diff is not canonical", case.kind, case.directory);
}

/// 🩹 The committed delta ALONE carries the before-document to the after-document.
pub fn assert_diff_applies(case: &Case) {
    let decoded: EnergyModelDiff = semio_framework_pack_json::from_json_str(case.diff.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &decode(case, "before-snapshot", case.before.as_str())).expect("committed diff applies to the before-document");
    assert_eq!(produced, decode(case, "after-snapshot", case.after.as_str()), "{}/{}: the committed diff did not carry before to after", case.kind, case.directory);
}

/// 🧭️ The kind's semantic descriptor uses an approved verb and its inverse closes the loop.
pub async fn assert_semantics(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let descriptor = SemanticMutation::semantics(&mutation);
    assert!(protocol::is_approved_verb(descriptor.verb), "{}: {:?} is not an approved verb", case.kind, descriptor.verb);
    assert_eq!(descriptor.kind, case.kind);
    assert_eq!(<EnergyModelMutation as SemanticMutation<EnergyModelSnapshot>>::kinds().len(), super::KINDS.len());
    assert_inverse(case);
    let _ = base;
}

/// 🎯️ Whether the committed outcome declares this vector a refusal (`{"status":"rejected",…}`).
fn committed_is_rejected(case: &Case) -> bool {
    matches!(committed(case, "outcome", case.outcome.as_str()).get("status"), Some(semio_framework_pack_json::Value::String(status)) if status == "rejected")
}

/// ⚖️ The framework's own laws, run in role against this vector: the sequential inverse law, the L3 inverse-sum law (the concrete
/// inverse steps' diffs, absorbed, equal the negative of the forward diff) and the diff-absorb law over the forward diff followed
/// by the diff of the first inverse step applied. A vector the committed outcome declares REJECTED is exempt from the inverse
/// laws by construction — the framework laws panic on any Error/Fatal forward outcome, and a refusal IS that outcome; what a
/// refusal owes instead (the exact fault code + path, an empty delta, before == after) is pinned by
/// `assert_outcome`/`assert_diff`/`assert_forward`. The absorb law still holds for it.
pub async fn assert_laws(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let first = built(case).diff().clone();
    if committed_is_rejected(case) {
        assert!(built(case).worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error), "{}/{}: the committed outcome is a refusal but the implementation applied the mutation", case.kind, case.directory);
        assert_eq!(decode(case, "after-snapshot", case.after.as_str()), base, "{}/{}: a refused vector must leave the document untouched", case.kind, case.directory);
        protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first.clone(), first).await;
        return;
    }
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let after = protocol::apply_diff(&first, &base).expect("the committed forward diff applies");
    let undo = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").pop();
    let second = undo.map_or_else(EnergyModelDiff::default, |step| <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&step, &after).diff().clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first, second).await;
}

/// 📝️ Every declared kind survives the text and binary op codecs the `dsl::DslEnum` derive
/// generates — one loop, no per-kind round-trip test to forget.
#[semio_framework_async_macros::async_test]
async fn every_kind_round_trips_through_text_and_binary() {
    for operation in super::wire_probes() {
        store::os_store::test_support::assert_op_line_round_trip(&operation);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}

//#region 🔖️Roster
/// 🏷️ Direct semantic roster exported for the language-neutral test adapter, in aggregate
/// declaration order — which is also the binary ordinal order `dsl::variants_binary` writes.
pub const KINDS: &[&str] = &[
    "rename-model",
    "change-model-version",
    "change-site-latitude",
    "change-site-longitude",
    "change-site-elevation",
    "change-site-time-zone",
    "change-site-north-axis",
    "change-ground-building",
    "change-ground-shallow",
    "change-ground-deep",
    "change-run-start-month",
    "change-run-start-day",
    "change-run-end-month",
    "change-run-end-day",
    "change-run-year",
    "replace-airflow-network",
    "add-output-variable",
    "remove-output-variable",
    "bind-weather-file",
    "unbind-weather-file",
    "connect-referenced-model",
    "disconnect-referenced-model",
    "rename-zone",
    "change-zone-volume",
    "change-zone-multiplier",
    "change-zone-conditioned",
    "change-zone-floor-area-participation",
    "create-zone",
    "delete-zone",
    "create-space",
    "delete-space",
    "rename-space",
    "change-space-floor-area",
    "change-space-zone",
    "create-surface",
    "delete-surface",
    "rename-surface",
    "change-surface-zone",
    "change-surface-class",
    "replace-surface-vertices",
    "change-surface-construction",
    "change-surface-boundary-condition",
    "change-surface-sun-exposed",
    "change-surface-wind-exposed",
    "change-surface-multiplier",
    "create-fenestration",
    "delete-fenestration",
    "rename-fenestration",
    "change-fenestration-surface",
    "change-fenestration-u-value",
    "change-fenestration-shgc",
    "change-fenestration-vlt",
    "change-fenestration-area",
    "change-fenestration-frame-conductance",
    "change-fenestration-divider-conductance",
    "create-shading-surface",
    "delete-shading-surface",
    "rename-shading-surface",
    "replace-shading-surface-vertices",
    "change-shading-surface-transmittance-schedule",
    "connect-surfaces",
    "disconnect-surfaces",
    "bind-fenestration-glazing-construction",
    "clear-fenestration-glazing-construction",
    "change-fenestration-height",
    "change-fenestration-sill-height",
    "change-fenestration-overhang-depth",
    "change-fenestration-overhang-offset",
    "change-fenestration-fin-depth",
    "change-fenestration-fin-offset",
    "create-material",
    "delete-material",
    "rename-material",
    "change-material-thickness",
    "change-material-conductivity",
    "change-material-density",
    "change-material-specific-heat",
    "change-material-thermal-absorptance",
    "change-material-solar-absorptance",
    "change-material-visible-absorptance",
    "create-construction",
    "delete-construction",
    "rename-construction",
    "add-construction-layer",
    "remove-construction-layer",
    "reorder-construction-layers",
    "create-people-gain",
    "delete-people-gain",
    "change-people-gain-zone",
    "change-people-gain-schedule",
    "change-people-gain-activity-schedule",
    "change-people-gain-people-per-area",
    "change-people-gain-sensible-fraction",
    "change-people-gain-latent-fraction",
    "change-people-gain-radiant-fraction",
    "create-lighting-gain",
    "delete-lighting-gain",
    "change-lighting-gain-zone",
    "change-lighting-gain-schedule",
    "change-lighting-gain-watts-per-area",
    "change-lighting-gain-radiant-fraction",
    "change-lighting-gain-visible-fraction",
    "change-lighting-gain-return-air-fraction",
    "create-equipment-gain",
    "delete-equipment-gain",
    "change-equipment-gain-zone",
    "change-equipment-gain-schedule",
    "change-equipment-gain-watts-per-area",
    "change-equipment-gain-radiant-fraction",
    "change-equipment-gain-latent-fraction",
    "create-infiltration",
    "delete-infiltration",
    "change-infiltration-zone",
    "change-infiltration-schedule",
    "change-infiltration-flow-per-exterior-area",
    "change-infiltration-constant-term-coefficient",
    "change-infiltration-temperature-term-coefficient",
    "change-infiltration-velocity-term-coefficient",
    "change-infiltration-velocity-squared-term-coefficient",
    "create-mechanical-ventilation",
    "delete-mechanical-ventilation",
    "change-mechanical-ventilation-zone",
    "change-mechanical-ventilation-schedule",
    "change-mechanical-ventilation-design-flow",
    "change-mechanical-ventilation-fan-total-efficiency",
    "change-mechanical-ventilation-fan-delta-pressure",
    "change-infiltration-method",
    "change-infiltration-design-flow-ach",
    "change-infiltration-effective-leakage-area",
    "change-infiltration-discharge-coefficient",
    "change-infiltration-stack-height",
    "create-thermostat",
    "delete-thermostat",
    "change-thermostat-zone",
    "change-thermostat-heating-setpoint-schedule",
    "change-thermostat-cooling-setpoint-schedule",
    "change-thermostat-heating-throttle-range",
    "change-thermostat-cooling-throttle-range",
    "create-humidistat",
    "delete-humidistat",
    "change-humidistat-zone",
    "change-humidistat-humidifying-setpoint-schedule",
    "change-humidistat-dehumidifying-setpoint-schedule",
    "change-humidistat-humidifying-throttle-range",
    "change-humidistat-dehumidifying-throttle-range",
    "create-ideal-loads-system",
    "delete-ideal-loads-system",
    "change-ideal-loads-system-zone",
    "change-ideal-loads-system-max-heating-supply-air-temp",
    "change-ideal-loads-system-min-cooling-supply-air-temp",
    "change-ideal-loads-system-max-heating-capacity",
    "change-ideal-loads-system-max-cooling-capacity",
    "change-ideal-loads-system-outdoor-air-per-person",
    "change-ideal-loads-system-outdoor-air-per-area",
    "create-zone-equipment",
    "delete-zone-equipment",
    "change-zone-equipment-zone",
    "change-zone-equipment-type",
    "change-zone-equipment-priority",
    "change-zone-equipment-heating-capacity",
    "change-zone-equipment-cooling-capacity",
    "create-daylight-zone",
    "delete-daylight-zone",
    "change-daylight-zone-zone",
    "change-daylight-zone-illuminance-target",
    "change-daylight-zone-glare-limit",
    "change-daylight-zone-window-transmittance",
    "create-sizing-object",
    "delete-sizing-object",
    "change-sizing-object-zone",
    "change-sizing-object-sizing-type",
    "change-sizing-object-design-day-type",
    "create-room-air-model-assignment",
    "delete-room-air-model-assignment",
    "change-room-air-model",
    "create-setpoint-manager",
    "delete-setpoint-manager",
    "rename-setpoint-manager",
    "replace-setpoint-manager-kind",
    "change-setpoint-manager-schedule",
    "create-air-loop",
    "delete-air-loop",
    "rename-air-loop",
    "change-air-loop-supply-node",
    "change-air-loop-return-node",
    "change-air-loop-design-supply-air-flow",
    "add-air-loop-terminal-zone",
    "remove-air-loop-terminal-zone",
    "create-plant-loop",
    "delete-plant-loop",
    "rename-plant-loop",
    "change-plant-loop-type",
    "change-plant-loop-supply-temperature",
    "change-plant-loop-return-temperature",
    "change-plant-loop-design-flow",
    "add-plant-loop-equipment",
    "remove-plant-loop-equipment",
    "create-outdoor-air-system",
    "delete-outdoor-air-system",
    "change-outdoor-air-system-air-loop",
    "change-outdoor-air-system-min-oa-flow",
    "change-outdoor-air-system-economizer-enabled",
    "create-electrical-load-center",
    "delete-electrical-load-center",
    "rename-electrical-load-center",
    "add-electrical-load-center-pv",
    "remove-electrical-load-center-pv",
    "add-electrical-load-center-battery",
    "remove-electrical-load-center-battery",
    "create-pv-system",
    "delete-pv-system",
    "change-pv-system-dc-capacity",
    "change-pv-system-area",
    "change-pv-system-tilt",
    "change-pv-system-azimuth",
    "change-pv-system-module-efficiency",
    "change-pv-system-inverter-efficiency",
    "create-battery",
    "delete-battery",
    "change-battery-capacity",
    "change-battery-max-charge",
    "change-battery-max-discharge",
    "change-battery-round-trip-efficiency",
    "create-shw-system",
    "delete-shw-system",
    "change-shw-system-heater-capacity",
    "change-shw-system-storage-volume",
    "change-shw-system-setpoint",
    "change-shw-system-schedule",
    "create-solar-thermal-system",
    "delete-solar-thermal-system",
    "change-solar-thermal-system-collector-area",
    "change-solar-thermal-system-efficiency",
    "change-solar-thermal-system-storage-volume",
    "change-solar-thermal-system-tilt",
    "change-solar-thermal-system-azimuth",
    "create-refrigeration-system",
    "delete-refrigeration-system",
    "change-refrigeration-system-case-count",
    "change-refrigeration-system-design-load",
    "change-refrigeration-system-defrost-schedule",
    "create-water-system",
    "delete-water-system",
    "change-water-system-fixture-count",
    "change-water-system-peak-flow",
    "change-water-system-schedule",
    "create-fault",
    "delete-fault",
    "change-fault-target-equipment",
    "change-fault-type",
    "change-fault-severity",
    "change-fault-start-schedule",
    "create-space-list",
    "delete-space-list",
    "rename-space-list",
    "add-space-list-member",
    "remove-space-list-member",
    "create-thermal-enclosure",
    "delete-thermal-enclosure",
    "rename-thermal-enclosure",
    "add-thermal-enclosure-zone",
    "remove-thermal-enclosure-zone",
    "create-constant-schedule",
    "delete-constant-schedule",
    "change-constant-schedule-value",
    "create-daily-schedule",
    "delete-daily-schedule",
    "replace-daily-schedule-hourly-values",
    "change-daily-schedule-interpolation",
    "change-daily-schedule-limits",
    "create-weekly-schedule",
    "delete-weekly-schedule",
    "change-weekly-schedule-day",
    "create-annual-schedule",
    "delete-annual-schedule",
    "insert-annual-schedule-rule",
    "remove-annual-schedule-rule",
    "reorder-annual-schedule-rules",
    "change-annual-schedule-default-daily-schedule",
    "change-annual-schedule-holiday-daily-schedule",
    "add-annual-schedule-holiday",
    "remove-annual-schedule-holiday",
    "create-time-series-schedule",
    "delete-time-series-schedule",
    "replace-time-series-schedule-values",
    "change-time-series-schedule-timestep",
    "replace-fenestration-vertices",
    "change-glazing-material-thickness",
    "change-glazing-material-conductivity",
    "change-glazing-material-solar-transmittance",
    "change-glazing-material-visible-transmittance",
    "change-glazing-material-infrared-emissivity",
    "rename-glazing-material",
    "change-gas-material-thickness",
    "change-gas-material-gas",
    "rename-gas-material",
    "change-material-roughness",
];

/// 🗂️ `(semanticKind, domain/operation owner directory)` for every declared kind — the single place the
/// emoji-carrying directory names are stated in Rust.
pub const DIRECTORIES: &[(&str, &str)] = &[
    ("rename-model", "🔋️model/🏷️rename"),
    ("change-model-version", "🔋️model/🔢️change-version"),
    ("change-site-latitude", "📍️site/🌍️change-latitude"),
    ("change-site-longitude", "📍️site/🌏️change-longitude"),
    ("change-site-elevation", "📍️site/⛰️change-elevation"),
    ("change-site-time-zone", "📍️site/🕰️change-time-zone"),
    ("change-site-north-axis", "📍️site/🔝️change-north-axis"),
    ("change-ground-building", "📍️site/🌡️change-ground-building"),
    ("change-ground-shallow", "📍️site/🌱️change-ground-shallow"),
    ("change-ground-deep", "📍️site/⛏️change-ground-deep"),
    ("change-run-start-month", "⏯️run/🛫️change-start-month"),
    ("change-run-start-day", "⏯️run/▶️change-start-day"),
    ("change-run-end-month", "⏯️run/🛬️change-end-month"),
    ("change-run-end-day", "⏯️run/⏹️change-end-day"),
    ("change-run-year", "⏯️run/📅️change-year"),
    ("replace-airflow-network", "🔋️model/🫧️replace-airflow-network"),
    ("add-output-variable", "🔋️model/📊️add-output-variable"),
    ("remove-output-variable", "🔋️model/📉️remove-output-variable"),
    ("bind-weather-file", "🔋️model/🌦️bind-weather-file"),
    ("unbind-weather-file", "🔋️model/🌤️unbind-weather-file"),
    ("connect-referenced-model", "🔋️model/🪢️connect-referenced-model"),
    ("disconnect-referenced-model", "🔋️model/✂️disconnect-referenced-model"),
    ("rename-zone", "🏡️zone/🏠️rename"),
    ("change-zone-volume", "🏡️zone/🧊️change-volume"),
    ("change-zone-multiplier", "🏡️zone/✖️change-multiplier"),
    ("change-zone-conditioned", "🏡️zone/🌬️change-conditioned"),
    ("change-zone-floor-area-participation", "🏡️zone/📐️change-floor-area-participation"),
    ("create-zone", "🏡️zone/🏘️create"),
    ("delete-zone", "🏡️zone/🏚️delete"),
    ("create-space", "🛋️space/🪑️create"),
    ("delete-space", "🛋️space/🧹️delete"),
    ("rename-space", "🛋️space/🔤️rename"),
    ("change-space-floor-area", "🛋️space/🧮️change-floor-area"),
    ("change-space-zone", "🛋️space/🚚️change-zone"),
    ("create-surface", "🧱️surface/🟫️create"),
    ("delete-surface", "🧱️surface/🪚️delete"),
    ("rename-surface", "🧱️surface/🏳️rename"),
    ("change-surface-zone", "🧱️surface/🗜️change-zone"),
    ("change-surface-class", "🧱️surface/🧩️change-class"),
    ("replace-surface-vertices", "🧱️surface/🔺️replace-vertices"),
    ("change-surface-construction", "🧱️surface/🧰️change-construction"),
    ("change-surface-boundary-condition", "🔲️boundary/🚧️change-boundary-condition"),
    ("change-surface-sun-exposed", "🔲️boundary/🌅️change-sun-exposed"),
    ("change-surface-wind-exposed", "🔲️boundary/🍃️change-wind-exposed"),
    ("change-surface-multiplier", "🧱️surface/🔁️change-multiplier"),
    ("create-fenestration", "🪟️fenestration/🪟️create"),
    ("delete-fenestration", "🪟️fenestration/🚪️delete"),
    ("rename-fenestration", "🪟️fenestration/🏁️rename"),
    ("change-fenestration-surface", "🪟️fenestration/🧲️change-surface"),
    ("change-fenestration-u-value", "🔍️glazing/🌐️change-u-value"),
    ("change-fenestration-shgc", "🔍️glazing/🌇️change-shgc"),
    ("change-fenestration-vlt", "🔍️glazing/🌈️change-vlt"),
    ("change-fenestration-area", "🪟️fenestration/🟥️change-area"),
    ("change-fenestration-frame-conductance", "🔍️glazing/🖼️change-frame-conductance"),
    ("change-fenestration-divider-conductance", "🔍️glazing/🧷️change-divider-conductance"),
    ("create-shading-surface", "🌳️shading/🌳️create"),
    ("delete-shading-surface", "🌳️shading/🪵️delete"),
    ("rename-shading-surface", "🌳️shading/🏕️rename"),
    ("replace-shading-surface-vertices", "🌳️shading/🗺️replace-vertices"),
    ("change-shading-surface-transmittance-schedule", "🌳️shading/⛱️change-transmittance-schedule"),
    ("connect-surfaces", "🔲️boundary/🤝️connect"),
    ("disconnect-surfaces", "🔲️boundary/💔️disconnect"),
    ("bind-fenestration-glazing-construction", "🔍️glazing/🧊️bind-glazing-construction"),
    ("clear-fenestration-glazing-construction", "🔍️glazing/🫗️clear-glazing-construction"),
    ("change-fenestration-height", "🪟️fenestration/⬆️change-height"),
    ("change-fenestration-sill-height", "🪟️fenestration/⬇️change-sill-height"),
    ("change-fenestration-overhang-depth", "⛱️shading-device/🧢️change-overhang-depth"),
    ("change-fenestration-overhang-offset", "⛱️shading-device/🎩️change-overhang-offset"),
    ("change-fenestration-fin-depth", "⛱️shading-device/🐬️change-fin-depth"),
    ("change-fenestration-fin-offset", "⛱️shading-device/🐋️change-fin-offset"),
    ("create-material", "🪨️material/🧱️create"),
    ("delete-material", "🪨️material/🪨️delete"),
    ("rename-material", "🪨️material/🪧️rename"),
    ("change-material-thickness", "🪨️material/📏️change-thickness"),
    ("change-material-conductivity", "🪨️material/🔥️change-conductivity"),
    ("change-material-density", "🪨️material/⚖️change-density"),
    ("change-material-specific-heat", "🪨️material/♨️change-specific-heat"),
    ("change-material-thermal-absorptance", "🎨️finish/🔆️change-thermal-absorptance"),
    ("change-material-solar-absorptance", "🎨️finish/☀️change-solar-absorptance"),
    ("change-material-visible-absorptance", "🎨️finish/👁️change-visible-absorptance"),
    ("create-construction", "🏗️construction/🏗️create"),
    ("delete-construction", "🏗️construction/🧨️delete"),
    ("rename-construction", "🏗️construction/🪪️rename"),
    ("add-construction-layer", "🏗️construction/➕️add-layer"),
    ("remove-construction-layer", "🏗️construction/➖️remove-layer"),
    ("reorder-construction-layers", "🏗️construction/🔀️reorder-layers"),
    ("create-people-gain", "👥️people-gain/👤️create"),
    ("delete-people-gain", "👥️people-gain/🚷️delete"),
    ("change-people-gain-zone", "👥️people-gain/🚶️change-zone"),
    ("change-people-gain-schedule", "👥️people-gain/⏰️change-schedule"),
    ("change-people-gain-activity-schedule", "👥️people-gain/🏃️change-activity-schedule"),
    ("change-people-gain-people-per-area", "👥️people-gain/👥️change-people-per-area"),
    ("change-people-gain-sensible-fraction", "👥️people-gain/🌞️change-sensible-fraction"),
    ("change-people-gain-latent-fraction", "👥️people-gain/💧️change-latent-fraction"),
    ("change-people-gain-radiant-fraction", "👥️people-gain/📡️change-radiant-fraction"),
    ("create-lighting-gain", "💡️lighting-gain/💡️create"),
    ("delete-lighting-gain", "💡️lighting-gain/🕯️delete"),
    ("change-lighting-gain-zone", "💡️lighting-gain/🔦️change-zone"),
    ("change-lighting-gain-schedule", "💡️lighting-gain/⏱️change-schedule"),
    ("change-lighting-gain-watts-per-area", "💡️lighting-gain/🔌️change-watts-per-area"),
    ("change-lighting-gain-radiant-fraction", "💡️lighting-gain/🌟️change-radiant-fraction"),
    ("change-lighting-gain-visible-fraction", "💡️lighting-gain/🔅️change-visible-fraction"),
    ("change-lighting-gain-return-air-fraction", "💡️lighting-gain/🎐️change-return-air-fraction"),
    ("create-equipment-gain", "🖥️equipment-gain/🖥️create"),
    ("delete-equipment-gain", "🖥️equipment-gain/🧯️delete"),
    ("change-equipment-gain-zone", "🖥️equipment-gain/🖨️change-zone"),
    ("change-equipment-gain-schedule", "🖥️equipment-gain/⌛️change-schedule"),
    ("change-equipment-gain-watts-per-area", "🖥️equipment-gain/⚡️change-watts-per-area"),
    ("change-equipment-gain-radiant-fraction", "🖥️equipment-gain/🌠️change-radiant-fraction"),
    ("change-equipment-gain-latent-fraction", "🖥️equipment-gain/💦️change-latent-fraction"),
    ("create-infiltration", "💨️infiltration/💨️create"),
    ("delete-infiltration", "💨️infiltration/🧽️delete"),
    ("change-infiltration-zone", "💨️infiltration/🌀️change-zone"),
    ("change-infiltration-schedule", "💨️infiltration/⏳️change-schedule"),
    ("change-infiltration-flow-per-exterior-area", "💨️infiltration/🌫️change-flow-per-exterior-area"),
    ("change-infiltration-constant-term-coefficient", "🕳️leakage/🅰️change-constant-term-coefficient"),
    ("change-infiltration-temperature-term-coefficient", "🕳️leakage/🅱️change-temperature-term-coefficient"),
    ("change-infiltration-velocity-term-coefficient", "🕳️leakage/🆎️change-velocity-term-coefficient"),
    ("change-infiltration-velocity-squared-term-coefficient", "🕳️leakage/🆑️change-velocity-squared-term-coefficient"),
    ("create-mechanical-ventilation", "🌪️ventilation/🌪️create"),
    ("delete-mechanical-ventilation", "🌪️ventilation/🚫️delete"),
    ("change-mechanical-ventilation-zone", "🌪️ventilation/🧭️change-zone"),
    ("change-mechanical-ventilation-schedule", "🌪️ventilation/📆️change-schedule"),
    ("change-mechanical-ventilation-design-flow", "🌪️ventilation/🚿️change-design-flow"),
    ("change-mechanical-ventilation-fan-total-efficiency", "🌪️ventilation/💠️change-fan-total-efficiency"),
    ("change-mechanical-ventilation-fan-delta-pressure", "🌪️ventilation/🎈️change-fan-delta-pressure"),
    ("change-infiltration-method", "💨️infiltration/🔬️change-method"),
    ("change-infiltration-design-flow-ach", "💨️infiltration/🔄️change-design-flow-ach"),
    ("change-infiltration-effective-leakage-area", "🕳️leakage/🕳️change-effective-leakage-area"),
    ("change-infiltration-discharge-coefficient", "🕳️leakage/🚰️change-discharge-coefficient"),
    ("change-infiltration-stack-height", "🕳️leakage/🏭️change-stack-height"),
    ("create-thermostat", "🌡️thermostat/🩺️create"),
    ("delete-thermostat", "🌡️thermostat/🛑️delete"),
    ("change-thermostat-zone", "🌡️thermostat/🛖️change-zone"),
    ("change-thermostat-heating-setpoint-schedule", "🌡️thermostat/🥵️change-heating-setpoint-schedule"),
    ("change-thermostat-cooling-setpoint-schedule", "🌡️thermostat/🐧️change-cooling-setpoint-schedule"),
    ("change-thermostat-heating-throttle-range", "🌡️thermostat/🎚️change-heating-throttle-range"),
    ("change-thermostat-cooling-throttle-range", "🌡️thermostat/🎛️change-cooling-throttle-range"),
    ("create-humidistat", "💧️humidistat/🌂️create"),
    ("delete-humidistat", "💧️humidistat/🏜️delete"),
    ("change-humidistat-zone", "💧️humidistat/🏙️change-zone"),
    ("change-humidistat-humidifying-setpoint-schedule", "💧️humidistat/☔️change-humidifying-setpoint-schedule"),
    ("change-humidistat-dehumidifying-setpoint-schedule", "💧️humidistat/🏝️change-dehumidifying-setpoint-schedule"),
    ("change-humidistat-humidifying-throttle-range", "💧️humidistat/🌧️change-humidifying-throttle-range"),
    ("change-humidistat-dehumidifying-throttle-range", "💧️humidistat/🧻️change-dehumidifying-throttle-range"),
    ("create-ideal-loads-system", "💯️ideal-loads/🫁️create"),
    ("delete-ideal-loads-system", "💯️ideal-loads/🫥️delete"),
    ("change-ideal-loads-system-zone", "💯️ideal-loads/🏢️change-zone"),
    ("change-ideal-loads-system-max-heating-supply-air-temp", "💯️ideal-loads/🔴️change-max-heating-supply-air-temp"),
    ("change-ideal-loads-system-min-cooling-supply-air-temp", "💯️ideal-loads/🔵️change-min-cooling-supply-air-temp"),
    ("change-ideal-loads-system-max-heating-capacity", "💯️ideal-loads/⛽️change-max-heating-capacity"),
    ("change-ideal-loads-system-max-cooling-capacity", "💯️ideal-loads/🟧️change-max-cooling-capacity"),
    ("change-ideal-loads-system-outdoor-air-per-person", "💯️ideal-loads/🧍️change-outdoor-air-per-person"),
    ("change-ideal-loads-system-outdoor-air-per-area", "💯️ideal-loads/🔳️change-outdoor-air-per-area"),
    ("create-zone-equipment", "🔧️zone-equipment/🛠️create"),
    ("delete-zone-equipment", "🔧️zone-equipment/🗑️delete"),
    ("change-zone-equipment-zone", "🔧️zone-equipment/🏬️change-zone"),
    ("change-zone-equipment-type", "🔧️zone-equipment/🔧️change-type"),
    ("change-zone-equipment-priority", "🔧️zone-equipment/🎗️change-priority"),
    ("change-zone-equipment-heating-capacity", "🔧️zone-equipment/🧇️change-heating-capacity"),
    ("change-zone-equipment-cooling-capacity", "🔧️zone-equipment/🍧️change-cooling-capacity"),
    ("create-daylight-zone", "🌞️daylight-zone/🔭️create"),
    ("delete-daylight-zone", "🌞️daylight-zone/🌗️delete"),
    ("change-daylight-zone-zone", "🌞️daylight-zone/🏫️change-zone"),
    ("change-daylight-zone-illuminance-target", "🌞️daylight-zone/🪔️change-illuminance-target"),
    ("change-daylight-zone-glare-limit", "🌞️daylight-zone/🕶️change-glare-limit"),
    ("change-daylight-zone-window-transmittance", "🌞️daylight-zone/🥃️change-window-transmittance"),
    ("create-sizing-object", "📏️sizing/📶️create"),
    ("delete-sizing-object", "📏️sizing/🪒️delete"),
    ("change-sizing-object-zone", "📏️sizing/🏨️change-zone"),
    ("change-sizing-object-sizing-type", "📏️sizing/🧾️change-sizing-type"),
    ("change-sizing-object-design-day-type", "📏️sizing/🌥️change-design-day-type"),
    ("create-room-air-model-assignment", "🛏️room-air/🛏️create-assignment"),
    ("delete-room-air-model-assignment", "🛏️room-air/🧺️delete-assignment"),
    ("change-room-air-model", "🛏️room-air/🪭️change"),
    ("create-setpoint-manager", "🎚️setpoint/📌️create"),
    ("delete-setpoint-manager", "🎚️setpoint/🍄️delete"),
    ("rename-setpoint-manager", "🎚️setpoint/🖇️rename"),
    ("replace-setpoint-manager-kind", "🎚️setpoint/🔃️replace-kind"),
    ("change-setpoint-manager-schedule", "🎚️setpoint/🎼️change-schedule"),
    ("create-air-loop", "🌬️air-loop/🛞️create"),
    ("delete-air-loop", "🌬️air-loop/🥀️delete"),
    ("rename-air-loop", "🌬️air-loop/📇️rename"),
    ("change-air-loop-supply-node", "🌬️air-loop/↗️change-supply-node"),
    ("change-air-loop-return-node", "🌬️air-loop/↘️change-return-node"),
    ("change-air-loop-design-supply-air-flow", "🌬️air-loop/🍥️change-design-supply-air-flow"),
    ("add-air-loop-terminal-zone", "🌬️air-loop/🪺️add-terminal-zone"),
    ("remove-air-loop-terminal-zone", "🌬️air-loop/🪹️remove-terminal-zone"),
    ("create-plant-loop", "🏭️plant-loop/⚗️create"),
    ("delete-plant-loop", "🏭️plant-loop/💣️delete"),
    ("rename-plant-loop", "🏭️plant-loop/📛️rename"),
    ("change-plant-loop-type", "🏭️plant-loop/♻️change-type"),
    ("change-plant-loop-supply-temperature", "🏭️plant-loop/☕️change-supply-temperature"),
    ("change-plant-loop-return-temperature", "🏭️plant-loop/🫖️change-return-temperature"),
    ("change-plant-loop-design-flow", "🏭️plant-loop/🚤️change-design-flow"),
    ("add-plant-loop-equipment", "🏭️plant-loop/🔩️add-equipment"),
    ("remove-plant-loop-equipment", "🏭️plant-loop/⚙️remove-equipment"),
    ("create-outdoor-air-system", "🍃️outdoor-air/🌲️create"),
    ("delete-outdoor-air-system", "🍃️outdoor-air/🍂️delete"),
    ("change-outdoor-air-system-air-loop", "🍃️outdoor-air/⛓️change-air-loop"),
    ("change-outdoor-air-system-min-oa-flow", "🍃️outdoor-air/🦋️change-min-oa-flow"),
    ("change-outdoor-air-system-economizer-enabled", "🍃️outdoor-air/💰️change-economizer-enabled"),
    ("create-electrical-load-center", "🔌️load-center/🏦️create"),
    ("delete-electrical-load-center", "🔌️load-center/🔻️delete"),
    ("rename-electrical-load-center", "🔌️load-center/🖊️rename"),
    ("add-electrical-load-center-pv", "🔌️load-center/☄️add-pv"),
    ("remove-electrical-load-center-pv", "🔌️load-center/🌘️remove-pv"),
    ("add-electrical-load-center-battery", "🔌️load-center/🔋️add-battery"),
    ("remove-electrical-load-center-battery", "🔌️load-center/🪝️remove-battery"),
    ("create-pv-system", "☀️pv-system/✨️create"),
    ("delete-pv-system", "☀️pv-system/🌒️delete"),
    ("change-pv-system-dc-capacity", "☀️pv-system/⚛️change-dc-capacity"),
    ("change-pv-system-area", "☀️pv-system/🟨️change-area"),
    ("change-pv-system-tilt", "☀️pv-system/📈️change-tilt"),
    ("change-pv-system-azimuth", "☀️pv-system/🧿️change-azimuth"),
    ("change-pv-system-module-efficiency", "☀️pv-system/🎖️change-module-efficiency"),
    ("change-pv-system-inverter-efficiency", "☀️pv-system/♌️change-inverter-efficiency"),
    ("create-battery", "🪫️battery/🪙️create"),
    ("delete-battery", "🪫️battery/♒️delete"),
    ("change-battery-capacity", "🪫️battery/🥫️change-capacity"),
    ("change-battery-max-charge", "🪫️battery/⏫️change-max-charge"),
    ("change-battery-max-discharge", "🪫️battery/⏬️change-max-discharge"),
    ("change-battery-round-trip-efficiency", "🪫️battery/🥉️change-round-trip-efficiency"),
    ("create-shw-system", "🚿️shw-system/🛀️create"),
    ("delete-shw-system", "🚿️shw-system/🚱️delete"),
    ("change-shw-system-heater-capacity", "🚿️shw-system/🍵️change-heater-capacity"),
    ("change-shw-system-storage-volume", "🚿️shw-system/🛢️change-storage-volume"),
    ("change-shw-system-setpoint", "🚿️shw-system/🏹️change-setpoint"),
    ("change-shw-system-schedule", "🚿️shw-system/🕐️change-schedule"),
    ("create-solar-thermal-system", "🔥️solar-thermal/🌄️create"),
    ("delete-solar-thermal-system", "🔥️solar-thermal/🌆️delete"),
    ("change-solar-thermal-system-collector-area", "🔥️solar-thermal/🟩️change-collector-area"),
    ("change-solar-thermal-system-efficiency", "🔥️solar-thermal/🏅️change-efficiency"),
    ("change-solar-thermal-system-storage-volume", "🔥️solar-thermal/🧃️change-storage-volume"),
    ("change-solar-thermal-system-tilt", "🔥️solar-thermal/🔼️change-tilt"),
    ("change-solar-thermal-system-azimuth", "🔥️solar-thermal/⛵️change-azimuth"),
    ("create-refrigeration-system", "❄️refrigeration/❄️create"),
    ("delete-refrigeration-system", "❄️refrigeration/🫠️delete"),
    ("change-refrigeration-system-case-count", "❄️refrigeration/🗄️change-case-count"),
    ("change-refrigeration-system-design-load", "❄️refrigeration/🏋️change-design-load"),
    ("change-refrigeration-system-defrost-schedule", "❄️refrigeration/🕑️change-defrost-schedule"),
    ("create-water-system", "🚰️water-system/🚽️create"),
    ("delete-water-system", "🚰️water-system/🧼️delete"),
    ("change-water-system-fixture-count", "🚰️water-system/🪣️change-fixture-count"),
    ("change-water-system-peak-flow", "🚰️water-system/🚾️change-peak-flow"),
    ("change-water-system-schedule", "🚰️water-system/🕒️change-schedule"),
    ("create-fault", "⚠️fault/⚠️create"),
    ("delete-fault", "⚠️fault/🩹️delete"),
    ("change-fault-target-equipment", "⚠️fault/🎣️change-target-equipment"),
    ("change-fault-type", "⚠️fault/🐛️change-type"),
    ("change-fault-severity", "⚠️fault/🌶️change-severity"),
    ("change-fault-start-schedule", "⚠️fault/🕓️change-start-schedule"),
    ("create-space-list", "📋️space-list/📋️create"),
    ("delete-space-list", "📋️space-list/🗒️delete"),
    ("rename-space-list", "📋️space-list/🪶️rename"),
    ("add-space-list-member", "📋️space-list/➡️add-member"),
    ("remove-space-list-member", "📋️space-list/🪤️remove-member"),
    ("create-thermal-enclosure", "🔒️enclosure/🏟️create"),
    ("delete-thermal-enclosure", "🔒️enclosure/🏯️delete"),
    ("rename-thermal-enclosure", "🔒️enclosure/🖋️rename"),
    ("add-thermal-enclosure-zone", "🔒️enclosure/🔒️add-zone"),
    ("remove-thermal-enclosure-zone", "🔒️enclosure/🔓️remove-zone"),
    ("create-constant-schedule", "🕐️fixed-schedule/🕜️create"),
    ("delete-constant-schedule", "🕐️fixed-schedule/📍️delete"),
    ("change-constant-schedule-value", "🕐️fixed-schedule/🕝️change-value"),
    ("create-daily-schedule", "🕒️day-schedule/🕞️create"),
    ("delete-daily-schedule", "🕒️day-schedule/🌓️delete"),
    ("replace-daily-schedule-hourly-values", "🕒️day-schedule/🕔️replace-hourly-values"),
    ("change-daily-schedule-interpolation", "🕒️day-schedule/🕕️change-interpolation"),
    ("change-daily-schedule-limits", "🕒️day-schedule/🕟️change-limits"),
    ("create-weekly-schedule", "🗓️week-schedule/🗓️create"),
    ("delete-weekly-schedule", "🗓️week-schedule/🕖️delete"),
    ("change-weekly-schedule-day", "🗓️week-schedule/🕗️change-day"),
    ("create-annual-schedule", "📆️year-schedule/📚️create"),
    ("delete-annual-schedule", "📆️year-schedule/📕️delete"),
    ("insert-annual-schedule-rule", "📆️year-schedule/📗️insert-rule"),
    ("remove-annual-schedule-rule", "📆️year-schedule/📙️remove-rule"),
    ("reorder-annual-schedule-rules", "📆️year-schedule/🗂️reorder-rules"),
    ("change-annual-schedule-default-daily-schedule", "📆️year-schedule/🎌️change-default-daily-schedule"),
    ("change-annual-schedule-holiday-daily-schedule", "📆️year-schedule/🎄️change-holiday-daily-schedule"),
    ("add-annual-schedule-holiday", "📆️year-schedule/🎉️add-holiday"),
    ("remove-annual-schedule-holiday", "📆️year-schedule/🎊️remove-holiday"),
    ("create-time-series-schedule", "📈️time-series/🪗️create"),
    ("delete-time-series-schedule", "📈️time-series/🎞️delete"),
    ("replace-time-series-schedule-values", "📈️time-series/🕘️replace-values"),
    ("change-time-series-schedule-timestep", "📈️time-series/🕙️change-timestep"),
    ("replace-fenestration-vertices", "🪟️fenestration/🔶️replace-vertices"),
    ("change-glazing-material-thickness", "🥂️window-layer/🔷️change-glazing-thickness"),
    ("change-glazing-material-conductivity", "🥂️window-layer/🟠️change-glazing-conductivity"),
    ("change-glazing-material-solar-transmittance", "🥂️window-layer/🟡️change-glazing-solar-transmittance"),
    ("change-glazing-material-visible-transmittance", "🥂️window-layer/🥽️change-glazing-visible-transmittance"),
    ("change-glazing-material-infrared-emissivity", "🥂️window-layer/🩻️change-glazing-infrared-emissivity"),
    ("rename-glazing-material", "🥂️window-layer/🟢️rename-glazing"),
    ("change-gas-material-thickness", "🥂️window-layer/🟣️change-gas-thickness"),
    ("change-gas-material-gas", "🥂️window-layer/🟤️change-gas-type"),
    ("rename-gas-material", "🥂️window-layer/🔘️rename-gas"),
    ("change-material-roughness", "🎨️finish/🗻️change-roughness"),
];
//#endregion 🔖️Roster

//#region 🧵️WireProbes
/// 🧵️ One representative value per declared variant — the codec round-trip corpus, so wire coverage
/// grows with the enum instead of with a hand-maintained list of tests.
pub fn wire_probes() -> Vec<EnergyModelMutation> {
    vec![
        rename_model("Probe".to_string()),
        change_model_version("2".to_string()),
        change_site_latitude(52.4),
        change_site_longitude(9.7),
        change_site_elevation(55.0),
        change_site_time_zone(1.0),
        change_site_north_axis(15.0),
        change_ground_building(7, 21.5),
        change_ground_shallow(7, 21.5),
        change_ground_deep(8.0),
        change_run_start_month(2),
        change_run_start_day(3),
        change_run_end_month(11),
        change_run_end_day(30),
        change_run_year(2027),
        replace_airflow_network(true, vec![1], vec![1], 0, vec![7]),
        add_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string(), crate::model::OutputReportFrequency::Hourly, None),
        remove_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string()),
        bind_weather_file(semio_framework_artifact_reference::ArtifactRef { artifact_id: "hannover".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "epw".into() } }),
        unbind_weather_file(),
        connect_referenced_model(semio_framework_artifact_reference::ArtifactRef { artifact_id: "doc-2".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } }),
        disconnect_referenced_model(),
        rename_zone(crate::model::EntityId(1), "ZONE 1".to_string()),
        change_zone_volume(crate::model::EntityId(1), 129.6),
        change_zone_multiplier(crate::model::EntityId(1), 4),
        change_zone_conditioned(crate::model::EntityId(1), false),
        change_zone_floor_area_participation(crate::model::EntityId(1), false),
        create_zone(crate::model::EntityId(2), "ZONE TWO".to_string(), 129.6, 1, true, true, None),
        delete_zone(crate::model::EntityId(2)),
        create_space(crate::model::EntityId(2), "SPACE ONE".to_string(), crate::model::EntityId(1), 48.0, None),
        delete_space(crate::model::EntityId(2)),
        rename_space(crate::model::EntityId(2), "SPACE 1".to_string()),
        change_space_floor_area(crate::model::EntityId(2), 48.0),
        change_space_zone(crate::model::EntityId(2), crate::model::EntityId(3)),
        create_surface(
            crate::model::EntityId(3),
            "WALL SOUTH".to_string(),
            crate::model::EntityId(1),
            crate::model::SurfaceClass::ExteriorWall,
            vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7]],
            crate::model::EntityId(2),
            crate::model::OutsideBoundaryKind::OutdoorAir,
            None,
            true,
            true,
            1, None,
        ),
        delete_surface(crate::model::EntityId(3)),
        rename_surface(crate::model::EntityId(3), "WALL S".to_string()),
        change_surface_zone(crate::model::EntityId(3), crate::model::EntityId(7)),
        change_surface_class(crate::model::EntityId(3), crate::model::SurfaceClass::Roof),
        replace_surface_vertices(crate::model::EntityId(3), vec![[0.0, 0.0, 0.0], [6.0, 0.0, 0.0], [6.0, 0.0, 2.7]]),
        change_surface_construction(crate::model::EntityId(3), crate::model::EntityId(8)),
        change_surface_boundary_condition(crate::model::EntityId(3), crate::model::OutsideBoundaryKind::Ground, None),
        change_surface_sun_exposed(crate::model::EntityId(3), false),
        change_surface_wind_exposed(crate::model::EntityId(3), false),
        change_surface_multiplier(crate::model::EntityId(3), 4),
        create_fenestration(crate::model::EntityId(5), "WINDOW SOUTH".to_string(), crate::model::EntityId(3), 3.0, 0.787, 0.86, 6.0, 2.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None),
        delete_fenestration(crate::model::EntityId(5)),
        rename_fenestration(crate::model::EntityId(5), "WINDOW S".to_string()),
        change_fenestration_surface(crate::model::EntityId(5), crate::model::EntityId(6)),
        change_fenestration_u_value(crate::model::EntityId(5), 2.74),
        change_fenestration_shgc(crate::model::EntityId(5), 0.76),
        change_fenestration_vlt(crate::model::EntityId(5), 0.84),
        change_fenestration_area(crate::model::EntityId(5), 12.0),
        change_fenestration_frame_conductance(crate::model::EntityId(5), 1.2),
        change_fenestration_divider_conductance(crate::model::EntityId(5), 0.4),
        create_shading_surface(crate::model::EntityId(11), "SITE AWNING".to_string(), vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0]], None, None),
        delete_shading_surface(crate::model::EntityId(11)),
        rename_shading_surface(crate::model::EntityId(11), "AWNING".to_string()),
        replace_shading_surface_vertices(crate::model::EntityId(11), vec![[0.0, -3.0, 3.0], [8.0, -3.0, 3.0], [8.0, 0.0, 3.0]]),
        change_shading_surface_transmittance_schedule(crate::model::EntityId(11), Some(crate::schedule::ScheduleId(1))),
        connect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6), None),
        disconnect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6)),
        bind_fenestration_glazing_construction(crate::model::EntityId(5), crate::model::EntityId(7)),
        clear_fenestration_glazing_construction(crate::model::EntityId(5)),
        change_fenestration_height(crate::model::EntityId(5), 2.4),
        change_fenestration_sill_height(crate::model::EntityId(5), 0.9),
        change_fenestration_overhang_depth(crate::model::EntityId(5), 1.0),
        change_fenestration_overhang_offset(crate::model::EntityId(5), 0.5),
        change_fenestration_fin_depth(crate::model::EntityId(5), 1.0),
        change_fenestration_fin_offset(crate::model::EntityId(5), 0.2),
        create_material(2, crate::model::EntityId(3), "CONCRETE SLAB".into(), crate::model::SurfaceRoughness::MediumRough, 0.08, 1.13, 1400.0, 1000.0, 0.9, 0.6, 0.6),
        delete_material(crate::model::EntityId(1)),
        rename_material(crate::model::EntityId(1), "TIMBER FLOORING".to_string()),
        change_material_thickness(crate::model::EntityId(1), 0.02),
        change_material_conductivity(crate::model::EntityId(1), 0.25),
        change_material_density(crate::model::EntityId(1), 1200.0),
        change_material_specific_heat(crate::model::EntityId(1), 1000.0),
        change_material_thermal_absorptance(crate::model::EntityId(1), 0.85),
        change_material_solar_absorptance(crate::model::EntityId(1), 0.7),
        change_material_visible_absorptance(crate::model::EntityId(1), 0.75),
        create_construction(2, crate::model::EntityId(12), "ROOF".into(), vec![crate::model::EntityId(2), crate::model::EntityId(1)]),
        delete_construction(crate::model::EntityId(11)),
        rename_construction(crate::model::EntityId(10), "LTWALL INSULATED".to_string()),
        add_construction_layer(crate::model::EntityId(10), 2, crate::model::EntityId(1)),
        remove_construction_layer(crate::model::EntityId(10), 1),
        reorder_construction_layers(crate::model::EntityId(10), vec![crate::model::EntityId(2), crate::model::EntityId(1)]),
        create_people_gain(1, crate::model::EntityId(21), crate::model::EntityId(2), crate::schedule::ScheduleId(1), crate::schedule::ScheduleId(2), 0.1, 0.6, 0.4, 0.3),
        delete_people_gain(crate::model::EntityId(20)),
        change_people_gain_zone(crate::model::EntityId(20), crate::model::EntityId(2)),
        change_people_gain_schedule(crate::model::EntityId(20), crate::schedule::ScheduleId(3)),
        change_people_gain_activity_schedule(crate::model::EntityId(20), crate::schedule::ScheduleId(3)),
        change_people_gain_people_per_area(crate::model::EntityId(20), 0.1),
        change_people_gain_sensible_fraction(crate::model::EntityId(20), 0.7),
        change_people_gain_latent_fraction(crate::model::EntityId(20), 0.3),
        change_people_gain_radiant_fraction(crate::model::EntityId(20), 0.5),
        create_lighting_gain(1, crate::model::EntityId(31), crate::model::EntityId(2), crate::schedule::ScheduleId(1), 8.0, 0.6, 0.2, 0.0),
        delete_lighting_gain(crate::model::EntityId(30)),
        change_lighting_gain_zone(crate::model::EntityId(30), crate::model::EntityId(2)),
        change_lighting_gain_schedule(crate::model::EntityId(30), crate::schedule::ScheduleId(3)),
        change_lighting_gain_watts_per_area(crate::model::EntityId(30), 12.5),
        change_lighting_gain_radiant_fraction(crate::model::EntityId(30), 0.7),
        change_lighting_gain_visible_fraction(crate::model::EntityId(30), 0.25),
        change_lighting_gain_return_air_fraction(crate::model::EntityId(30), 0.1),
        create_equipment_gain(1, crate::model::EntityId(41), crate::model::EntityId(2), crate::schedule::ScheduleId(1), 2.5, 0.5, 0.0),
        delete_equipment_gain(crate::model::EntityId(40)),
        change_equipment_gain_zone(crate::model::EntityId(40), crate::model::EntityId(2)),
        change_equipment_gain_schedule(crate::model::EntityId(40), crate::schedule::ScheduleId(3)),
        change_equipment_gain_watts_per_area(crate::model::EntityId(40), 5.0),
        change_equipment_gain_radiant_fraction(crate::model::EntityId(40), 0.4),
        change_equipment_gain_latent_fraction(crate::model::EntityId(40), 0.2),
        create_infiltration(1, crate::model::EntityId(51), crate::model::EntityId(2), crate::schedule::ScheduleId(1), crate::air_exchange::InfiltrationMethod::ScheduledAch, 0.5, 0.0, 0.0, 1.0, 2.7, 1.0, 0.0, 0.0, 0.0),
        delete_infiltration(crate::model::EntityId(50)),
        change_infiltration_zone(crate::model::EntityId(50), crate::model::EntityId(2)),
        change_infiltration_schedule(crate::model::EntityId(50), crate::schedule::ScheduleId(3)),
        change_infiltration_flow_per_exterior_area(crate::model::EntityId(50), 0.0003),
        change_infiltration_constant_term_coefficient(crate::model::EntityId(50), 0.606),
        change_infiltration_temperature_term_coefficient(crate::model::EntityId(50), 0.03636),
        change_infiltration_velocity_term_coefficient(crate::model::EntityId(50), 0.1177),
        change_infiltration_velocity_squared_term_coefficient(crate::model::EntityId(50), 0.000103),
        create_mechanical_ventilation(1, crate::model::EntityId(61), crate::model::EntityId(2), crate::schedule::ScheduleId(1), 0.08, 0.65, 250.0),
        delete_mechanical_ventilation(crate::model::EntityId(60)),
        change_mechanical_ventilation_zone(crate::model::EntityId(60), crate::model::EntityId(2)),
        change_mechanical_ventilation_schedule(crate::model::EntityId(60), crate::schedule::ScheduleId(3)),
        change_mechanical_ventilation_design_flow(crate::model::EntityId(60), 0.12),
        change_mechanical_ventilation_fan_total_efficiency(crate::model::EntityId(60), 0.8),
        change_mechanical_ventilation_fan_delta_pressure(crate::model::EntityId(60), 500.0),
        change_infiltration_method(crate::model::EntityId(50), crate::air_exchange::InfiltrationMethod::PerExteriorArea),
        change_infiltration_design_flow_ach(crate::model::EntityId(50), 1.0),
        change_infiltration_effective_leakage_area(crate::model::EntityId(50), 0.05),
        change_infiltration_discharge_coefficient(crate::model::EntityId(50), 0.65),
        change_infiltration_stack_height(crate::model::EntityId(50), 3.0),
        create_thermostat(crate::model::EntityId(10), crate::model::EntityId(1), crate::schedule::ScheduleId(1), crate::schedule::ScheduleId(2), 1.0, 1.0, None),
        delete_thermostat(crate::model::EntityId(10)),
        change_thermostat_zone(crate::model::EntityId(10), crate::model::EntityId(2)),
        change_thermostat_heating_setpoint_schedule(crate::model::EntityId(10), crate::schedule::ScheduleId(2)),
        change_thermostat_cooling_setpoint_schedule(crate::model::EntityId(10), crate::schedule::ScheduleId(1)),
        change_thermostat_heating_throttle_range(crate::model::EntityId(10), 2.0),
        change_thermostat_cooling_throttle_range(crate::model::EntityId(10), 2.0),
        create_humidistat(crate::model::EntityId(11), crate::model::EntityId(1), crate::schedule::ScheduleId(1), crate::schedule::ScheduleId(2), 5.0, 5.0, None),
        delete_humidistat(crate::model::EntityId(11)),
        change_humidistat_zone(crate::model::EntityId(11), crate::model::EntityId(2)),
        change_humidistat_humidifying_setpoint_schedule(crate::model::EntityId(11), crate::schedule::ScheduleId(2)),
        change_humidistat_dehumidifying_setpoint_schedule(crate::model::EntityId(11), crate::schedule::ScheduleId(1)),
        change_humidistat_humidifying_throttle_range(crate::model::EntityId(11), 10.0),
        change_humidistat_dehumidifying_throttle_range(crate::model::EntityId(11), 10.0),
        create_ideal_loads_system(crate::model::EntityId(12), crate::model::EntityId(1), 50.0, 13.0, false, 0.0, false, 0.0, 0.0, 0.0, None),
        delete_ideal_loads_system(crate::model::EntityId(12)),
        change_ideal_loads_system_zone(crate::model::EntityId(12), crate::model::EntityId(2)),
        change_ideal_loads_system_max_heating_supply_air_temp(crate::model::EntityId(12), 45.0),
        change_ideal_loads_system_min_cooling_supply_air_temp(crate::model::EntityId(12), 12.0),
        change_ideal_loads_system_max_heating_capacity(crate::model::EntityId(12), true, 4000.0),
        change_ideal_loads_system_max_cooling_capacity(crate::model::EntityId(12), true, 5000.0),
        change_ideal_loads_system_outdoor_air_per_person(crate::model::EntityId(12), 0.0025),
        change_ideal_loads_system_outdoor_air_per_area(crate::model::EntityId(12), 0.0003),
        create_zone_equipment(crate::model::EntityId(13), crate::model::EntityId(1), crate::model::ZoneEquipmentType::Baseboard, 1, 2000.0, 0.0, None),
        delete_zone_equipment(crate::model::EntityId(13)),
        change_zone_equipment_zone(crate::model::EntityId(13), crate::model::EntityId(2)),
        change_zone_equipment_type(crate::model::EntityId(13), crate::model::ZoneEquipmentType::FanCoil),
        change_zone_equipment_priority(crate::model::EntityId(13), 2),
        change_zone_equipment_heating_capacity(crate::model::EntityId(13), 3000.0),
        change_zone_equipment_cooling_capacity(crate::model::EntityId(13), 1500.0),
        create_daylight_zone(crate::model::EntityId(14), crate::model::EntityId(1), 500.0, 22.0, 0.6, None),
        delete_daylight_zone(crate::model::EntityId(14)),
        change_daylight_zone_zone(crate::model::EntityId(14), crate::model::EntityId(2)),
        change_daylight_zone_illuminance_target(crate::model::EntityId(14), 300.0),
        change_daylight_zone_glare_limit(crate::model::EntityId(14), 20.0),
        change_daylight_zone_window_transmittance(crate::model::EntityId(14), 0.5),
        create_sizing_object(crate::model::EntityId(15), crate::model::EntityId(1), crate::model::SizingType::Heating, crate::model::DesignDayType::Heating, None),
        delete_sizing_object(crate::model::EntityId(15)),
        change_sizing_object_zone(crate::model::EntityId(15), crate::model::EntityId(2)),
        change_sizing_object_sizing_type(crate::model::EntityId(15), crate::model::SizingType::Cooling),
        change_sizing_object_design_day_type(crate::model::EntityId(15), crate::model::DesignDayType::Cooling),
        create_room_air_model_assignment(crate::model::EntityId(1), crate::model::RoomAirModelType::WellMixed, None),
        delete_room_air_model_assignment(crate::model::EntityId(1)),
        change_room_air_model(crate::model::EntityId(1), crate::model::RoomAirModelType::TwoNodeBuoyancy),
        create_setpoint_manager(crate::model::EntityId(16), "SUPPLY SPM".to_string(), "Scheduled".to_string(), 0.0, 0.0, 0.0, 0.0, true, crate::schedule::ScheduleId(1), None),
        delete_setpoint_manager(crate::model::EntityId(16)),
        rename_setpoint_manager(crate::model::EntityId(16), "RESET SPM".to_string()),
        replace_setpoint_manager_kind(crate::model::EntityId(16), "OutdoorAirReset".to_string(), -10.0, 20.0, 16.0, 12.0),
        change_setpoint_manager_schedule(crate::model::EntityId(16), true, crate::schedule::ScheduleId(2)),
        create_air_loop(crate::model::EntityId(17), "MAIN AIR LOOP".to_string(), 1, 2, 1.2, vec![crate::model::EntityId(1)], None),
        delete_air_loop(crate::model::EntityId(17)),
        rename_air_loop(crate::model::EntityId(17), "PRIMARY AIR LOOP".to_string()),
        change_air_loop_supply_node(crate::model::EntityId(17), 5),
        change_air_loop_return_node(crate::model::EntityId(17), 6),
        change_air_loop_design_supply_air_flow(crate::model::EntityId(17), 1.6),
        add_air_loop_terminal_zone(crate::model::EntityId(17), crate::model::EntityId(2), None),
        remove_air_loop_terminal_zone(crate::model::EntityId(17), crate::model::EntityId(1)),
        create_plant_loop(crate::model::EntityId(18), "HOT WATER LOOP".to_string(), crate::model::PlantLoopType::Heating, 80.0, 60.0, 2.0, Vec::new(), None),
        delete_plant_loop(crate::model::EntityId(18)),
        rename_plant_loop(crate::model::EntityId(18), "BOILER LOOP".to_string()),
        change_plant_loop_type(crate::model::EntityId(18), crate::model::PlantLoopType::Cooling),
        change_plant_loop_supply_temperature(crate::model::EntityId(18), 70.0),
        change_plant_loop_return_temperature(crate::model::EntityId(18), 55.0),
        change_plant_loop_design_flow(crate::model::EntityId(18), 3.0),
        add_plant_loop_equipment(crate::model::EntityId(18), crate::model::EntityId(50), None),
        remove_plant_loop_equipment(crate::model::EntityId(18), crate::model::EntityId(50)),
        create_outdoor_air_system(crate::model::EntityId(19), crate::model::EntityId(17), 0.2, false, None),
        delete_outdoor_air_system(crate::model::EntityId(19)),
        change_outdoor_air_system_air_loop(crate::model::EntityId(19), crate::model::EntityId(20)),
        change_outdoor_air_system_min_oa_flow(crate::model::EntityId(19), 0.35),
        change_outdoor_air_system_economizer_enabled(crate::model::EntityId(19), true),
        create_electrical_load_center(2, crate::model::EntityId(322), "ROOF PANEL".into(), Vec::new(), vec![crate::model::EntityId(301)], Vec::new()),
        delete_electrical_load_center(crate::model::EntityId(321)),
        rename_electrical_load_center(crate::model::EntityId(320), "MAIN DISTRIBUTION".to_string()),
        add_electrical_load_center_pv(crate::model::EntityId(320), 1, crate::model::EntityId(301)),
        remove_electrical_load_center_pv(crate::model::EntityId(320), crate::model::EntityId(300)),
        add_electrical_load_center_battery(crate::model::EntityId(320), 0, crate::model::EntityId(310)),
        remove_electrical_load_center_battery(crate::model::EntityId(321), crate::model::EntityId(310)),
        create_pv_system(2, crate::model::EntityId(302), 8000.0, 44.0, 25.0, 200.0, 0.19, 0.97),
        delete_pv_system(crate::model::EntityId(300)),
        change_pv_system_dc_capacity(crate::model::EntityId(300), 7000.0),
        change_pv_system_area(crate::model::EntityId(300), 42.0),
        change_pv_system_tilt(crate::model::EntityId(300), 35.0),
        change_pv_system_azimuth(crate::model::EntityId(300), 170.0),
        change_pv_system_module_efficiency(crate::model::EntityId(300), 0.21),
        change_pv_system_inverter_efficiency(crate::model::EntityId(300), 0.98),
        create_battery(1, crate::model::EntityId(311), 27.0, 7000.0, 7000.0, 0.92),
        delete_battery(crate::model::EntityId(310)),
        change_battery_capacity(crate::model::EntityId(310), 20.0),
        change_battery_max_charge(crate::model::EntityId(310), 7000.0),
        change_battery_max_discharge(crate::model::EntityId(310), 6000.0),
        change_battery_round_trip_efficiency(crate::model::EntityId(310), 0.95),
        create_shw_system(1, crate::model::EntityId(401), 6000.0, 0.2, 60.0, crate::schedule::ScheduleId(2)),
        delete_shw_system(crate::model::EntityId(400)),
        change_shw_system_heater_capacity(crate::model::EntityId(400), 6000.0),
        change_shw_system_storage_volume(crate::model::EntityId(400), 0.2),
        change_shw_system_setpoint(crate::model::EntityId(400), 60.0),
        change_shw_system_schedule(crate::model::EntityId(400), crate::schedule::ScheduleId(2)),
        create_solar_thermal_system(1, crate::model::EntityId(411), 6.0, 0.6, 0.4, 40.0, 190.0),
        delete_solar_thermal_system(crate::model::EntityId(410)),
        change_solar_thermal_system_collector_area(crate::model::EntityId(410), 6.0),
        change_solar_thermal_system_efficiency(crate::model::EntityId(410), 0.62),
        change_solar_thermal_system_storage_volume(crate::model::EntityId(410), 0.5),
        change_solar_thermal_system_tilt(crate::model::EntityId(410), 35.0),
        change_solar_thermal_system_azimuth(crate::model::EntityId(410), 200.0),
        create_refrigeration_system(1, crate::model::EntityId(421), 2, 8000.0, crate::schedule::ScheduleId(2)),
        delete_refrigeration_system(crate::model::EntityId(420)),
        change_refrigeration_system_case_count(crate::model::EntityId(420), 6),
        change_refrigeration_system_design_load(crate::model::EntityId(420), 15000.0),
        change_refrigeration_system_defrost_schedule(crate::model::EntityId(420), crate::schedule::ScheduleId(2)),
        create_water_system(1, crate::model::EntityId(431), 4, 0.15, crate::schedule::ScheduleId(2)),
        delete_water_system(crate::model::EntityId(430)),
        change_water_system_fixture_count(crate::model::EntityId(430), 9),
        change_water_system_peak_flow(crate::model::EntityId(430), 0.4),
        change_water_system_schedule(crate::model::EntityId(430), crate::schedule::ScheduleId(2)),
        create_fault(1, crate::model::EntityId(441), crate::model::EntityId(501), crate::model::FaultType::SensorBias, 0.5, crate::schedule::ScheduleId(2)),
        delete_fault(crate::model::EntityId(440)),
        change_fault_target_equipment(crate::model::EntityId(440), crate::model::EntityId(501)),
        change_fault_type(crate::model::EntityId(440), crate::model::FaultType::SensorBias),
        change_fault_severity(crate::model::EntityId(440), 0.6),
        change_fault_start_schedule(crate::model::EntityId(440), crate::schedule::ScheduleId(2)),
        create_space_list(2, crate::model::EntityId(212), "ATRIUM".into(), vec![crate::model::EntityId(201)]),
        delete_space_list(crate::model::EntityId(211)),
        rename_space_list(crate::model::EntityId(210), "CORE SPACES".to_string()),
        add_space_list_member(crate::model::EntityId(210), 1, crate::model::EntityId(201)),
        remove_space_list_member(crate::model::EntityId(210), crate::model::EntityId(200)),
        create_thermal_enclosure(2, crate::model::EntityId(222), "ROOF BLOCK".into(), vec![crate::model::EntityId(2)]),
        delete_thermal_enclosure(crate::model::EntityId(221)),
        rename_thermal_enclosure(crate::model::EntityId(220), "NORTH ENCLOSURE".to_string()),
        add_thermal_enclosure_zone(crate::model::EntityId(220), 1, crate::model::EntityId(2)),
        remove_thermal_enclosure_zone(crate::model::EntityId(220), crate::model::EntityId(1)),
        create_constant_schedule(2, crate::schedule::ScheduleId(3), 0.75),
        delete_constant_schedule(crate::schedule::ScheduleId(2)),
        change_constant_schedule_value(crate::schedule::ScheduleId(1), 0.25),
        create_daily_schedule(3, crate::schedule::ScheduleId(13), vec![21.0; 24], crate::schedule::ScheduleInterpolation::Continuous, None, None),
        delete_daily_schedule(crate::schedule::ScheduleId(12)),
        replace_daily_schedule_hourly_values(crate::schedule::ScheduleId(10), vec![22.0; 24]),
        change_daily_schedule_interpolation(crate::schedule::ScheduleId(10), crate::schedule::ScheduleInterpolation::Discrete),
        change_daily_schedule_limits(crate::schedule::ScheduleId(10), Some(0.0), Some(40.0)),
        create_weekly_schedule(1, crate::schedule::ScheduleId(21), vec![crate::schedule::ScheduleId(11); 7]),
        delete_weekly_schedule(crate::schedule::ScheduleId(20)),
        change_weekly_schedule_day(crate::schedule::ScheduleId(20), 2, crate::schedule::ScheduleId(11)),
        create_annual_schedule(1, crate::schedule::ScheduleId(31), crate::schedule::ScheduleId(10), None),
        delete_annual_schedule(crate::schedule::ScheduleId(30)),
        insert_annual_schedule_rule(crate::schedule::ScheduleId(30), 1, 7, 1, 8, 31, crate::schedule::ScheduleId(12)),
        remove_annual_schedule_rule(crate::schedule::ScheduleId(30), 0),
        reorder_annual_schedule_rules(crate::schedule::ScheduleId(30), 0, 1),
        change_annual_schedule_default_daily_schedule(crate::schedule::ScheduleId(30), crate::schedule::ScheduleId(10)),
        change_annual_schedule_holiday_daily_schedule(crate::schedule::ScheduleId(30), Some(crate::schedule::ScheduleId(11))),
        add_annual_schedule_holiday(crate::schedule::ScheduleId(30), 1, 2026, 1, 1),
        remove_annual_schedule_holiday(crate::schedule::ScheduleId(30), 2026, 12, 25),
        create_time_series_schedule(1, crate::schedule::ScheduleId(41), vec![1.0, 0.9], 900),
        delete_time_series_schedule(crate::schedule::ScheduleId(40)),
        replace_time_series_schedule_values(crate::schedule::ScheduleId(40), vec![1.0, 0.8, 0.6, 0.4]),
        change_time_series_schedule_timestep(crate::schedule::ScheduleId(40), 900),
        replace_fenestration_vertices(crate::model::EntityId(4), vec![[1.0, 0.0, 0.5], [4.0, 0.0, 0.5], [4.0, 0.0, 2.0], [2.5, 0.0, 2.5], [1.0, 0.0, 2.0]]),
        change_glazing_material_thickness(crate::model::EntityId(1), 0.004),
        change_glazing_material_conductivity(crate::model::EntityId(1), 1.06),
        change_glazing_material_solar_transmittance(crate::model::EntityId(1), 0.61),
        change_glazing_material_visible_transmittance(crate::model::EntityId(1), 0.74),
        change_glazing_material_infrared_emissivity(crate::model::EntityId(1), 0.84, 0.04),
        rename_glazing_material(crate::model::EntityId(1), "LOW-E 3MM".to_string()),
        change_gas_material_thickness(crate::model::EntityId(2), 0.016),
        change_gas_material_gas(crate::model::EntityId(2), crate::model::GasKind::Argon),
        rename_gas_material(crate::model::EntityId(2), "ARGON GAP 16MM".to_string()),
        change_material_roughness(crate::model::EntityId(1), crate::model::SurfaceRoughness::VeryRough),
    ]
}
//#endregion 🧵️WireProbes
