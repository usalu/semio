//! 📥️ EN 1991 play app — structured inputs editor over the design-load subject.

use crate::En1991Snapshot;
use semio_framework_plugin::{LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-en1991-inputs";
pub const BODY_INPUTS: &str = "norm.en1991.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::en1991::create_en1991_app`.
pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🗂️ Projects the flat SI subject into ≤32-child groups so `UiFixedList` (cap 32) can admit the tree.
pub fn render(
    document: &En1991Snapshot,
    locale: semio_framework_plugin::Locale,
    controller_id: &'static str,
    windows: &semio_framework_plugin::TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let grouped = grouped_editor_value(document);
    crate::app_surface::render_document_editor(&grouped, locale, controller_id, Some(crate::field_meta::en1991_field_meta), windows)
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct GroupedEditor {
    annex: crate::document::AnnexChoice,
    site: SiteGroup,
    building: BuildingGroup,
    thermal: ThermalGroup,
    fire: FireGroup,
    execution: ExecutionGroup,
    bridge: BridgeGroup,
    crane: CraneGroup,
    silo: SiloGroup,
    floors: Vec<crate::FloorArea>,
    self_weight_elements: Vec<crate::SelfWeightElement>,
    roofs: Vec<crate::RoofArea>,
    wind_faces: Vec<crate::WindFace>,
    accidental_cases: Vec<crate::AccidentalCase>,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct SiteGroup {
    snow_zone: String,
    altitude: f64,
    en_sk: f64,
    exceptional_snow_north_german_lowlands: bool,
    wind_zone: u8,
    en_vb: f64,
    terrain_category: u8,
    mixed_terrain_upwind: u8,
    mixed_terrain_distance: f64,
    orography_factor: f64,
    coast_or_island: bool,
    air_density: f64,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct BuildingGroup {
    height: f64,
    width: f64,
    depth: f64,
    storey_count: u8,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct ThermalGroup {
    assumed_delta_t: f64,
    t_max: f64,
    t_min: f64,
    t_0: f64,
    thermal_element_type: String,
    thermal_bridge_type: u8,
    delta_t_m: f64,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct FireGroup {
    fire_mode: crate::FireMode,
    fire_curve: crate::part_1_2::FireCurve,
    fire_duration: f64,
    assumed_gas_temperature: f64,
    assumed_h_net: f64,
    fire_compartment_area: f64,
    fire_compartment_height: f64,
    fire_opening_factor: f64,
    fire_thermal_inertia: f64,
    fire_occupancy: String,
    fire_load_density_qf: f64,
    assumed_qf_d: f64,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct ExecutionGroup {
    construction_activity: String,
    assumed_construction_qk: f64,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct BridgeGroup {
    structure_kind: crate::StructureKind,
    bridge_lane: u8,
    bridge_span: f64,
    bridge_lane_width: f64,
    assumed_bridge_tandem: f64,
    assumed_bridge_udl: f64,
    assumed_bridge_lm2: f64,
    assumed_bridge_footway: f64,
    assumed_bridge_lm3: f64,
    assumed_bridge_lm4: f64,
    bridge_load_group: String,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct CraneGroup {
    crane_claimed: bool,
    crane_class: String,
    hoist_class: String,
    hoisting_speed: f64,
    assumed_crane_wheel: f64,
    assumed_crane_horizontal: f64,
}

#[derive(Clone, Debug, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct SiloGroup {
    silo_claimed: bool,
    silo_kind: String,
    silo_bulk_density: f64,
    silo_height: f64,
    silo_hydraulic_radius: f64,
    silo_mu: f64,
    silo_k: f64,
    assumed_silo_pressure: f64,
    assumed_silo_patch: f64,
    assumed_silo_wall_friction: f64,
}

fn grouped_editor_value(document: &En1991Snapshot) -> GroupedEditor {
    GroupedEditor {
        annex: document.annex,
        site: SiteGroup {
            snow_zone: document.snow_zone.clone(),
            altitude: document.altitude,
            en_sk: document.en_sk,
            exceptional_snow_north_german_lowlands: document.exceptional_snow_north_german_lowlands,
            wind_zone: document.wind_zone,
            en_vb: document.en_vb,
            terrain_category: document.terrain_category,
            mixed_terrain_upwind: document.mixed_terrain_upwind,
            mixed_terrain_distance: document.mixed_terrain_distance,
            orography_factor: document.orography_factor,
            coast_or_island: document.coast_or_island,
            air_density: document.air_density,
        },
        building: BuildingGroup { height: document.height, width: document.width, depth: document.depth, storey_count: document.storey_count },
        thermal: ThermalGroup {
            assumed_delta_t: document.assumed_delta_t,
            t_max: document.t_max,
            t_min: document.t_min,
            t_0: document.t_0,
            thermal_element_type: document.thermal_element_type.clone(),
            thermal_bridge_type: document.thermal_bridge_type,
            delta_t_m: document.delta_t_m,
        },
        fire: FireGroup {
            fire_mode: document.fire_mode,
            fire_curve: document.fire_curve,
            fire_duration: document.fire_duration,
            assumed_gas_temperature: document.assumed_gas_temperature,
            assumed_h_net: document.assumed_h_net,
            fire_compartment_area: document.fire_compartment_area,
            fire_compartment_height: document.fire_compartment_height,
            fire_opening_factor: document.fire_opening_factor,
            fire_thermal_inertia: document.fire_thermal_inertia,
            fire_occupancy: document.fire_occupancy.clone(),
            fire_load_density_qf: document.fire_load_density_qf,
            assumed_qf_d: document.assumed_qf_d,
        },
        execution: ExecutionGroup {
            construction_activity: document.construction_activity.clone(),
            assumed_construction_qk: document.assumed_construction_qk,
        },
        bridge: BridgeGroup {
            structure_kind: document.structure_kind,
            bridge_lane: document.bridge_lane,
            bridge_span: document.bridge_span,
            bridge_lane_width: document.bridge_lane_width,
            assumed_bridge_tandem: document.assumed_bridge_tandem,
            assumed_bridge_udl: document.assumed_bridge_udl,
            assumed_bridge_lm2: document.assumed_bridge_lm2,
            assumed_bridge_footway: document.assumed_bridge_footway,
            assumed_bridge_lm3: document.assumed_bridge_lm3,
            assumed_bridge_lm4: document.assumed_bridge_lm4,
            bridge_load_group: document.bridge_load_group.clone(),
        },
        crane: CraneGroup {
            crane_claimed: document.crane_claimed,
            crane_class: document.crane_class.clone(),
            hoist_class: document.hoist_class.clone(),
            hoisting_speed: document.hoisting_speed,
            assumed_crane_wheel: document.assumed_crane_wheel,
            assumed_crane_horizontal: document.assumed_crane_horizontal,
        },
        silo: SiloGroup {
            silo_claimed: document.silo_claimed,
            silo_kind: document.silo_kind.clone(),
            silo_bulk_density: document.silo_bulk_density,
            silo_height: document.silo_height,
            silo_hydraulic_radius: document.silo_hydraulic_radius,
            silo_mu: document.silo_mu,
            silo_k: document.silo_k,
            assumed_silo_pressure: document.assumed_silo_pressure,
            assumed_silo_patch: document.assumed_silo_patch,
            assumed_silo_wall_friction: document.assumed_silo_wall_friction,
        },
        floors: document.floors.clone(),
        self_weight_elements: document.self_weight_elements.clone(),
        roofs: document.roofs.clone(),
        wind_faces: document.wind_faces.clone(),
        accidental_cases: document.accidental_cases.clone(),
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
