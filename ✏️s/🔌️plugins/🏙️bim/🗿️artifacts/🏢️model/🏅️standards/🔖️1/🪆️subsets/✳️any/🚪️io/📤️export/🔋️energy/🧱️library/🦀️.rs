//! 🧱️ What the energy model shares between its surfaces: the materials and layered constructions (a layer of the BIM model is a material with a thickness, an energy material has its thickness built in, so every
//! pair of BIM material and thickness is one energy material and every distinct stack one construction) and the schedules the loads and set points refer to. Every table interns: asking twice for the same thing
//! answers the same id, so the file is as small as the model is repetitive and its order is the order of first use.
//! 📎 ISO 6946 <https://www.iso.org/standard/65708.html>

use super::target::{ConstantSchedule, Construction, DailySchedule, Material, ScheduleInterpolation, Schedules, SurfaceRoughness, WeeklySchedule};
use crate::Layer;
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 🌡️ The thermal absorptance of every surface, the default of EnergyPlus.
pub const THERMAL_ABSORPTANCE: f64 = 0.9;
/// ☀️ The solar and visible absorptance of every surface, the default of EnergyPlus.
pub const SOLAR_ABSORPTANCE: f64 = 0.7;

/// 🧱️ The materials and constructions of an energy model being written.
#[derive(Clone, Debug, Default)]
pub struct Library {
    pub materials: Vec<Material>,
    pub constructions: Vec<Construction>,
    material_ids: BTreeMap<(String, u64), u32>,
    construction_ids: BTreeMap<Vec<u32>, u32>,
}

/// 🔢️ The shortest text of a length in millimetres: `200`, `37.5`.
pub fn millimetres(metres: f64) -> String {
    let text = format!("{:.3}", metres * 1000.0);
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

impl Library {
    /// 🧱️ The energy material of `material` at `thickness`: `None` when the snapshot has no such material or its data cannot be a layer (a thickness or conductivity that is not positive).
    pub fn material(&mut self, model: &ModelSnapshot, material: &str, thickness: f64, next: &mut u32) -> Option<u32> {
        let row = model.materials.get(material)?;
        if !(thickness.is_finite() && thickness > 0.0 && row.conductivity.is_finite() && row.conductivity > 0.0) {
            return None;
        }
        let key = (material.to_string(), thickness.to_bits());
        if let Some(id) = self.material_ids.get(&key) {
            return Some(*id);
        }
        *next += 1;
        self.material_ids.insert(key, *next);
        self.materials.push(Material {
            id: *next,
            name: format!("{} {} mm", row.name, millimetres(thickness)),
            roughness: SurfaceRoughness::MediumRough,
            thickness_m: thickness,
            conductivity_w_m_k: row.conductivity,
            density_kg_m3: row.density.max(0.0),
            specific_heat_j_kg_k: row.specific_heat.max(0.0),
            thermal_absorptance: THERMAL_ABSORPTANCE,
            solar_absorptance: SOLAR_ABSORPTANCE,
            visible_absorptance: SOLAR_ABSORPTANCE,
        });
        Some(*next)
    }

    /// 🪟️ The host of a curtain wall: one massless layer (a millimetre at 1000 W/(m K), a resistance of one millionth) so the construction adds nothing to the fenestration that fills the whole surface.
    pub fn curtain_host(&mut self, next: &mut u32) -> u32 {
        let key = ("curtain-wall host".to_string(), 1.0f64.to_bits());
        let material = match self.material_ids.get(&key) {
            Some(id) => *id,
            None => {
                *next += 1;
                self.material_ids.insert(key, *next);
                self.materials.push(Material { id: *next, name: "Curtain wall host".to_string(), roughness: SurfaceRoughness::Smooth, thickness_m: 0.001, conductivity_w_m_k: 1000.0, density_kg_m3: 0.0, specific_heat_j_kg_k: 0.0, thermal_absorptance: THERMAL_ABSORPTANCE, solar_absorptance: SOLAR_ABSORPTANCE, visible_absorptance: SOLAR_ABSORPTANCE });
                *next
            }
        };
        if let Some(id) = self.construction_ids.get(&vec![material]) {
            return *id;
        }
        *next += 1;
        self.construction_ids.insert(vec![material], *next);
        self.constructions.push(Construction { id: *next, name: "Curtain wall host (massless)".to_string(), layer_material_ids: vec![material] });
        *next
    }

    /// 🧱️ The construction of `layers` listed outside first, `None` when a layer is not a material (the stack has no id then). The first construction of a stack names it.
    pub fn construction(&mut self, model: &ModelSnapshot, name: &str, layers: &[Layer], next: &mut u32) -> Option<u32> {
        let ids: Option<Vec<u32>> = layers.iter().map(|layer| self.material(model, &layer.material, layer.thickness, next)).collect();
        let ids = ids.filter(|ids| !ids.is_empty())?;
        if let Some(id) = self.construction_ids.get(&ids) {
            return Some(*id);
        }
        *next += 1;
        self.construction_ids.insert(ids.clone(), *next);
        self.constructions.push(Construction { id: *next, name: name.to_string(), layer_material_ids: ids });
        Some(*next)
    }
}

/// 📅️ What a schedule name of the BIM model stands for: the whole year, or the weekdays between two hours.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum Profile {
    Always,
    Weekdays { from: u32, to: u32 },
}

impl Profile {
    /// 📖️ The profile a schedule name states: a name that ends with `HH-HH` (for example `Office 08-18`) is the weekdays from the first to the second hour, any other name, and no name, is always on.
    /// The second answer is whether the name was understood (`false` for a name that states something else than hours).
    pub fn of(name: Option<&str>) -> (Self, bool) {
        let Some(name) = name.map(str::trim).filter(|name| !name.is_empty()) else { return (Self::Always, true) };
        let last = name.rsplit(' ').next().unwrap_or(name);
        let mut parts = last.split('-');
        let hours = (parts.next().and_then(hour), parts.next().and_then(hour), parts.next());
        match hours {
            (Some(from), Some(to), None) if from < to => (Self::Weekdays { from, to }, true),
            _ => (Self::Always, name.eq_ignore_ascii_case("always") || name.eq_ignore_ascii_case("24/7")),
        }
    }
}

fn hour(text: &str) -> Option<u32> {
    (!text.is_empty() && text.len() <= 2 && text.bytes().all(|byte| byte.is_ascii_digit())).then(|| text.parse().ok()).flatten().filter(|hour| *hour <= 24)
}

/// 📅️ The schedules of an energy model being written, each kind interned.
#[derive(Clone, Debug, Default)]
pub struct ScheduleBook {
    pub schedules: Schedules,
    constants: BTreeMap<u64, u32>,
    profiles: BTreeMap<(u32, u32), u32>,
}

impl ScheduleBook {
    /// 📅️ The constant schedule of `value`.
    pub fn constant(&mut self, value: f64, next: &mut u32) -> u32 {
        if let Some(id) = self.constants.get(&value.to_bits()) {
            return *id;
        }
        *next += 1;
        self.constants.insert(value.to_bits(), *next);
        self.schedules.constants.push(ConstantSchedule { id: *next, value });
        *next
    }

    /// 📅️ The schedule of a profile: the constant 1 for the whole year, for the weekdays a weekly schedule of one on-day (1 between the hours) and one off-day.
    pub fn profile(&mut self, profile: Profile, next: &mut u32) -> u32 {
        let Profile::Weekdays { from, to } = profile else { return self.constant(1.0, next) };
        if let Some(id) = self.profiles.get(&(from, to)) {
            return *id;
        }
        let mut hourly = [0.0; 24];
        for slot in hourly.iter_mut().take(to as usize).skip(from as usize) {
            *slot = 1.0;
        }
        let (on, off) = (*next + 1, *next + 2);
        *next += 3;
        self.schedules.daily.push(DailySchedule { id: on, hourly_values: hourly, interpolation: ScheduleInterpolation::Discrete, limits: None });
        self.schedules.daily.push(DailySchedule { id: off, hourly_values: [0.0; 24], interpolation: ScheduleInterpolation::Discrete, limits: None });
        self.schedules.weekly.push(WeeklySchedule { id: *next, daily_schedule_ids: [off, on, on, on, on, on, off] });
        self.profiles.insert((from, to), *next);
        *next
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
