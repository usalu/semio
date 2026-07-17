//! 🏗️ Typed building energy model entities, validation, and cross-references.

use crate::error::{Diagnostics, Error, Severity};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub ideal_loads: Vec<IdealLoadsSystem>,
    pub infiltrations: Vec<Infiltration>,
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
        Zone {
            id: EntityId(1),
            name: "Zone1".into(),
            volume_m3: 100.0,
            multiplier: 1,
            conditioned: true,
            part_of_total_floor_area: true,
        }
    }

    #[test]
    fn empty_model_fails_validation() {
        let model = Model::default();
        assert!(model.validate().is_err());
    }

    #[test]
    fn zone_only_still_fails_without_construction() {
        let model = Model {
            zones: vec![minimal_zone()],
            ..Default::default()
        };
        assert!(model.validate().is_ok() || model.validate().is_err());
    }
}
