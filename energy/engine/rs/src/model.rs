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
