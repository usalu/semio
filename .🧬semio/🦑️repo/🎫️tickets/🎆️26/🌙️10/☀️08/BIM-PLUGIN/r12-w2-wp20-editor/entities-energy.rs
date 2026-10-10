//! 🌡️ The energy rows of the entity table: how the conditions of a space (set points, occupancy, ventilation, power densities, schedule profile) read off the snapshot as the rows of the space, what the envelope of a
//! space and the totals of its zone, its building and its zone itself add up to as read-only rows, and the one pure function that condenses an `EnvelopeSpace` into the facts the panel shows. Writing is the table's
//! business (`set-space-conditions`, `set-type-thermal-data` through `partial`); nothing derived is ever stored.
//!
//! Related: ISO 6946 <https://www.iso.org/standard/65708.html>, GEG <https://www.gesetze-im-internet.de/geg/>.

use super::{number, InferredRow};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{Boundary, EnergyScope, EnergyTotals, EnvelopeSpace, SurfaceKind};
use crate::{ModelInference, ModelSnapshot, SpaceConditions};

//#region 🔖️Conditions
/// 🌡️ The text of one condition of a space: empty while the space states no conditions or not this one; `None` for a space that does not exist.
pub fn condition(snapshot: &ModelSnapshot, id: &str, pick: fn(&SpaceConditions) -> Option<String>) -> Option<String> {
    snapshot.spaces.contains_key(id).then(|| snapshot.space_conditions.get(id).and_then(pick).unwrap_or_default())
}

/// 🌡️ The text of an optional number: empty when absent.
pub fn amount(value: Option<f64>) -> Option<String> {
    Some(value.map(number).unwrap_or_default())
}

/// 🌡️ The condition a text states: trimmed, blank clears it.
pub fn stated(text: &str) -> Option<String> {
    Some(text.trim().to_string()).filter(|text| !text.is_empty())
}
/// 🌡️ The conditions of a space in one line for the outliner: the set points as a range or a bound in degrees Celsius, then the occupancy type and the schedule profile.
pub fn summary(conditions: &SpaceConditions) -> String {
    let setpoints = match (conditions.heating_setpoint, conditions.cooling_setpoint) {
        (Some(heating), Some(cooling)) => Some(format!("{}–{} °C", number(heating), number(cooling))),
        (Some(heating), None) => Some(format!("≥ {} °C", number(heating))),
        (None, Some(cooling)) => Some(format!("≤ {} °C", number(cooling))),
        (None, None) => None,
    };
    [setpoints, conditions.occupancy.clone(), conditions.schedule.clone()].into_iter().flatten().collect::<Vec<_>>().join(" · ")
}
//#endregion 🔖️Conditions

//#region 🔖️Facts
/// 📊️ What the envelope of one space adds up to, as the panel shows it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EnvelopeFacts {
    pub surfaces: usize,
    pub envelope_area: f64,
    pub glazing_area: f64,
    pub glazing_ratio: f64,
}

/// 📊️ The facts of an envelope: the number of surfaces, the area that borders the exterior or the ground, the area of the windows on the exterior and their share of the exterior facade (walls, curtain walls, windows
/// and doors on the exterior).
pub fn facts(space: &EnvelopeSpace) -> EnvelopeFacts {
    let outside = |boundary: Boundary| matches!(boundary, Boundary::Exterior | Boundary::Ground);
    let envelope_area = space.surfaces.iter().filter(|surface| outside(surface.boundary)).map(|surface| surface.area).sum();
    let exterior = space.surfaces.iter().filter(|surface| surface.boundary == Boundary::Exterior);
    let glazing_area: f64 = exterior.clone().filter(|surface| surface.kind == SurfaceKind::Window).map(|surface| surface.area).sum();
    let facade: f64 = exterior.filter(|surface| surface.vertical()).map(|surface| surface.area).sum();
    EnvelopeFacts { surfaces: space.surfaces.len(), envelope_area, glazing_area, glazing_ratio: if facade > 0.0 { glazing_area / facade } else { 0.0 } }
}

/// 📊️ A measure with two decimals.
pub fn fixed(value: f64) -> String {
    format!("{value:.2}")
}

/// 📊️ One fact of the envelope of a space, `None` while the space states no conditions.
pub fn envelope(inference: &ModelInference, id: &str, read: fn(&EnvelopeSpace, EnvelopeFacts) -> String) -> Option<String> {
    inference.energy_envelopes.get(id).map(|space| read(space, facts(space)))
}

/// 🏘️ The totals of the zone a space belongs to, read by `pick`.
pub fn zone_of_space(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, pick: fn(&EnergyTotals) -> f64) -> Option<String> {
    let zone = snapshot.spaces.get(id)?.zone.as_ref()?;
    inference.energy_totals.get(&EnergyScope::Zone(zone.clone()).key()).map(|totals| fixed(pick(totals)))
}

/// 📊️ One total of a scope with two decimals, `None` while the scope has no totals.
pub fn scope_total(inference: &ModelInference, scope: EnergyScope, pick: fn(&EnergyTotals) -> f64) -> Option<String> {
    inference.energy_totals.get(&scope.key()).map(|totals| fixed(pick(totals)))
}
//#endregion 🔖️Facts

//#region 🔖️Rows
/// 🏘️ One total of a zone, `None` while the zone does not exist or has no totals.
pub fn zone_total(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, pick: fn(&EnergyTotals) -> f64) -> Option<String> {
    snapshot.zones.contains_key(id).then(|| scope_total(inference, EnergyScope::Zone(id.into()), pick)).flatten()
}

/// 🏢️ One total of a building, `None` while the building does not exist or has no totals.
pub fn building_total(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, pick: fn(&EnergyTotals) -> f64) -> Option<String> {
    snapshot.buildings.contains_key(id).then(|| scope_total(inference, EnergyScope::Building(id.into()), pick)).flatten()
}

/// 💡️ The thermal totals of a building.
pub static BUILDING_INFERRED: &[InferredRow] = &[
    inferred!("envelope_area", field_envelope_area, |s, inference, id| building_total(s, inference, id, |totals| totals.envelope_area)),
    inferred!("h_t_prime", field_h_t_prime, |s, inference, id| building_total(s, inference, id, |totals| totals.h_t_prime)),
    inferred!("a_over_v", field_a_over_v, |s, inference, id| building_total(s, inference, id, |totals| totals.a_over_v)),
    inferred!("glazing_ratio", field_glazing_ratio, |s, inference, id| building_total(s, inference, id, |totals| totals.glazing_ratio)),
];
//#endregion 🔖️Rows

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
