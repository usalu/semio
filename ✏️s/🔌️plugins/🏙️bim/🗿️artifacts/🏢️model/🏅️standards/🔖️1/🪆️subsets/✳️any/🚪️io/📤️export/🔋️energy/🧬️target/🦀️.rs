//! 🧬️ The target of the energy bridge: the part of the `Model` of `s.energy.model@1` the BIM export writes, member for member and in the order the energy engine declares it, with the same derives, so a value of
//! these types renders to the same value tree (numbers, unit variants as text, `Interzone(n)` as `{ "Interzone": n }`, absent options as `null`) as the engine's own. The engine's `Model` is not linked: the bridge owns this
//! mirror, the schema `../🔣️.json` states the contract and the energy artifact's loader is the test of it. Members the bridge never writes are empty arrays.
//! 📎 ../../../../../../../../../🔋️energy/🗿️artifacts/🔋️model/🦀️.rs

use semio_framework_value::DslValue;

/// 🌍️ Site location and orientation.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct Site {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
    pub north_axis_deg: f64,
}

/// 🏠️ A thermal zone.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Zone {
    pub id: u32,
    pub name: String,
    pub volume_m3: f64,
    pub multiplier: u32,
    pub conditioned: bool,
    pub part_of_total_floor_area: bool,
}

/// 🪑️ A space of a zone.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Space {
    pub id: u32,
    pub name: String,
    pub zone_id: u32,
    pub floor_area_m2: f64,
}

/// 🧱️ The class of a surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue)]
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

/// 🌡️ What lies behind a surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue)]
pub enum OutsideBoundary {
    OutdoorAir,
    Ground,
    OtherSideTemperature,
    Adiabatic,
    Interzone(u32),
}

/// 📐️ A planar polygon surface, counter-clockwise seen from outside.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Surface {
    pub id: u32,
    pub name: String,
    pub zone_id: u32,
    pub class: SurfaceClass,
    pub vertices_m: Vec<[f64; 3]>,
    pub construction_id: u32,
    pub outside_boundary_condition: OutsideBoundary,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub multiplier: u32,
}

/// 🪟️ A window or door in an exterior surface.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Fenestration {
    pub id: u32,
    pub name: String,
    pub surface_id: u32,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
    pub vlt: f64,
    pub area_m2: f64,
    pub height_m: f64,
    pub sill_height_m: f64,
    pub frame_conductance_w_k: f64,
    pub divider_conductance_w_k: f64,
    pub overhang_depth_m: f64,
    pub overhang_offset_m: f64,
    pub fin_depth_m: f64,
    pub fin_offset_m: f64,
    pub glazing_construction_id: Option<u32>,
    pub vertices_m: Vec<[f64; 3]>,
}

/// 🪨️ The roughness of an exterior face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue)]
pub enum SurfaceRoughness {
    VeryRough,
    Rough,
    MediumRough,
    MediumSmooth,
    Smooth,
    VerySmooth,
}

/// 🧱️ An opaque material layer.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Material {
    pub id: u32,
    pub name: String,
    pub roughness: SurfaceRoughness,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kg_k: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
}

/// 🧱️ A layered construction, the layers outside first.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Construction {
    pub id: u32,
    pub name: String,
    pub layer_material_ids: Vec<u32>,
}

/// 👤️ People as an internal gain.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct PeopleGain {
    pub id: u32,
    pub zone_id: u32,
    pub schedule_id: u32,
    pub activity_schedule_id: u32,
    pub people_per_area: f64,
    pub sensible_fraction: f64,
    pub latent_fraction: f64,
    pub radiant_fraction: f64,
}

/// 💡️ Lighting as an internal gain.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct LightingGain {
    pub id: u32,
    pub zone_id: u32,
    pub schedule_id: u32,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub visible_fraction: f64,
    pub return_air_fraction: f64,
}

/// 🔌️ Electric equipment as an internal gain.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct EquipmentGain {
    pub id: u32,
    pub zone_id: u32,
    pub schedule_id: u32,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub latent_fraction: f64,
}

/// 🌡️ A thermostat.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Thermostat {
    pub id: u32,
    pub zone_id: u32,
    pub heating_setpoint_schedule_id: u32,
    pub cooling_setpoint_schedule_id: u32,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
}

/// ❄️ An ideal loads air system.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct IdealLoadsSystem {
    pub id: u32,
    pub zone_id: u32,
    pub max_heating_supply_air_temp_c: f64,
    pub min_cooling_supply_air_temp_c: f64,
    pub max_heating_capacity_w: Option<f64>,
    pub max_cooling_capacity_w: Option<f64>,
    pub outdoor_air_per_person_m3_s: f64,
    pub outdoor_air_per_area_m3_s_m2: f64,
}

/// 🏠️ Zones grouped by the zone of the BIM model they belong to.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct ThermalEnclosure {
    pub id: u32,
    pub name: String,
    pub zone_ids: Vec<u32>,
}

/// 🔗️ Two surfaces that face each other.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct AdjacencyPair {
    pub surface_a_id: u32,
    pub surface_b_id: u32,
}

/// 🌡️ The ground temperature by month.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct GroundTemperature {
    pub building_surface_c: [f64; 12],
    pub shallow_c: [f64; 12],
    pub deep_c: f64,
}

/// 📅️ The run period.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct RunPeriod {
    pub start_month: u32,
    pub start_day: u32,
    pub end_month: u32,
    pub end_day: u32,
    pub year: u32,
}

/// 📅️ A constant schedule.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct ConstantSchedule {
    pub id: u32,
    pub value: f64,
}

/// 📆️ How a daily schedule is read between its hours.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue)]
pub enum ScheduleInterpolation {
    Continuous,
    Discrete,
}

/// 📅️ A daily schedule of 24 hourly values.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct DailySchedule {
    pub id: u32,
    pub hourly_values: [f64; 24],
    pub interpolation: ScheduleInterpolation,
    pub limits: Option<DslValue>,
}

/// 📅️ A weekly schedule: the daily schedule of each day from Sunday to Saturday.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct WeeklySchedule {
    pub id: u32,
    pub daily_schedule_ids: [u32; 7],
}

/// 📚️ All schedules of the model.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct Schedules {
    pub constants: Vec<ConstantSchedule>,
    pub daily: Vec<DailySchedule>,
    pub weekly: Vec<WeeklySchedule>,
    pub annual: Vec<DslValue>,
    pub time_series: Vec<DslValue>,
}

/// 🏢️ The model of `s.energy.model@1`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct EnergyModel {
    pub name: String,
    pub version: String,
    pub site: Site,
    pub zones: Vec<Zone>,
    pub spaces: Vec<Space>,
    pub surfaces: Vec<Surface>,
    pub fenestrations: Vec<Fenestration>,
    pub materials: Vec<Material>,
    pub glazing_materials: Vec<DslValue>,
    pub gas_materials: Vec<DslValue>,
    pub constructions: Vec<Construction>,
    pub people: Vec<PeopleGain>,
    pub lighting: Vec<LightingGain>,
    pub equipment: Vec<EquipmentGain>,
    pub thermostats: Vec<Thermostat>,
    pub humidistats: Vec<DslValue>,
    pub setpoint_managers: Vec<DslValue>,
    pub ideal_loads: Vec<IdealLoadsSystem>,
    pub zone_equipment: Vec<DslValue>,
    pub air_loops: Vec<DslValue>,
    pub plant_loops: Vec<DslValue>,
    pub outdoor_air_systems: Vec<DslValue>,
    pub infiltrations: Vec<DslValue>,
    pub mechanical_ventilations: Vec<DslValue>,
    pub shading_surfaces: Vec<DslValue>,
    pub space_lists: Vec<DslValue>,
    pub thermal_enclosures: Vec<ThermalEnclosure>,
    pub adjacency_pairs: Vec<AdjacencyPair>,
    pub airflow_network: Option<DslValue>,
    pub electrical_load_centers: Vec<DslValue>,
    pub pv_systems: Vec<DslValue>,
    pub battery_storage: Vec<DslValue>,
    pub shw_systems: Vec<DslValue>,
    pub solar_thermal_systems: Vec<DslValue>,
    pub refrigeration_systems: Vec<DslValue>,
    pub water_systems: Vec<DslValue>,
    pub faults: Vec<DslValue>,
    pub output_variables: Vec<DslValue>,
    pub sizing_objects: Vec<DslValue>,
    pub daylight_zones: Vec<DslValue>,
    pub room_air_models: Vec<DslValue>,
    pub ground_temperature: GroundTemperature,
    pub run_period: RunPeriod,
    pub schedules: Schedules,
}

impl EnergyModel {
    /// 🌱️ A model with a name, a version and a site and nothing else: the ground at 18 degrees all year and the whole of 2026 as the run period, as the engine's defaults.
    pub fn new(name: &str, version: &str, site: Site) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            site,
            zones: Vec::new(),
            spaces: Vec::new(),
            surfaces: Vec::new(),
            fenestrations: Vec::new(),
            materials: Vec::new(),
            glazing_materials: Vec::new(),
            gas_materials: Vec::new(),
            constructions: Vec::new(),
            people: Vec::new(),
            lighting: Vec::new(),
            equipment: Vec::new(),
            thermostats: Vec::new(),
            humidistats: Vec::new(),
            setpoint_managers: Vec::new(),
            ideal_loads: Vec::new(),
            zone_equipment: Vec::new(),
            air_loops: Vec::new(),
            plant_loops: Vec::new(),
            outdoor_air_systems: Vec::new(),
            infiltrations: Vec::new(),
            mechanical_ventilations: Vec::new(),
            shading_surfaces: Vec::new(),
            space_lists: Vec::new(),
            thermal_enclosures: Vec::new(),
            adjacency_pairs: Vec::new(),
            airflow_network: None,
            electrical_load_centers: Vec::new(),
            pv_systems: Vec::new(),
            battery_storage: Vec::new(),
            shw_systems: Vec::new(),
            solar_thermal_systems: Vec::new(),
            refrigeration_systems: Vec::new(),
            water_systems: Vec::new(),
            faults: Vec::new(),
            output_variables: Vec::new(),
            sizing_objects: Vec::new(),
            daylight_zones: Vec::new(),
            room_air_models: Vec::new(),
            ground_temperature: GroundTemperature { building_surface_c: [18.0; 12], shallow_c: [18.0; 12], deep_c: 18.0 },
            run_period: RunPeriod { start_month: 1, start_day: 1, end_month: 12, end_day: 31, year: 2026 },
            schedules: Schedules::default(),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
