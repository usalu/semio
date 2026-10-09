//! 🏘️ The zoning rows of the entity table: how a zone, an area scheme and the zone and finish references of a space read off the snapshot, how an edited value becomes a `set-zone`, `set-area-scheme`
//! or `set-space` mutation, what a new zone or area scheme is, and the pickers (a zone, a finish material, an area measure) the properties panel offers for the reference rows.

use super::{number, parse_number, parse_text, partial, rename_element, variant, Created, FieldRow, InferredRow};
use crate::editor::bim::terminology::BimLabels;
use crate::mutations::set_space::SetSpace;
use crate::{AreaMeasure, AreaScheme, Assigned, ModelInference, ModelMutation, ModelSnapshot, SpacePatch, Zone};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

pub use crate::standards::v1::subsets::any::schema::inferences::finishes::FinishSurface;

//#region 🔖️Choices
/// 🏘️ The zones a space can join: every zone of the model by its id and name.
pub fn zone_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.zones.iter().map(|(id, zone)| (id.clone(), zone.name.clone())).collect()
}

/// 🎨️ The materials a finish can name: every material of the model by its id and name.
pub fn material_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.materials.iter().map(|(id, material)| (id.clone(), material.name.clone())).collect()
}

/// 📐️ The measures an area scheme adds up, by their stored name and their localized label.
pub fn measure_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![("Gross".to_string(), labels.choice_gross.as_str().to_string()), ("Net".to_string(), labels.choice_net.as_str().to_string())]
}
//#endregion 🔖️Choices

//#region 🔖️Writes
fn reference(value: &str) -> Option<String> {
    let text = value.trim();
    (!text.is_empty()).then(|| text.to_string())
}

fn list(value: &str) -> Vec<String> {
    value.split(',').map(str::trim).filter(|part| !part.is_empty()).map(str::to_string).collect()
}

fn space(id: &str, patch: SpacePatch) -> Option<ModelMutation> {
    Some(ModelMutation::SetSpace(SetSpace::from_patch(id.into(), patch)))
}

/// 🏘️ The mutation that moves a space into the zone a text names (empty leaves its zone).
pub fn write_zone(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    space(id, SpacePatch { zone: Some(Assigned::new(reference(value))), ..Default::default() })
}

/// 🎨️ The mutation that finishes the floor of a space with the material a text names (empty removes the finish).
pub fn write_floor_finish(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    space(id, SpacePatch { floor_finish: Some(Assigned::new(reference(value))), ..Default::default() })
}

/// 🎨️ The mutation that finishes the walls of a space with the material a text names (empty removes the finish).
pub fn write_wall_finish(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    space(id, SpacePatch { wall_finish: Some(Assigned::new(reference(value))), ..Default::default() })
}

/// 🎨️ The mutation that finishes the ceiling of a space with the material a text names (empty removes the finish).
pub fn write_ceiling_finish(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    space(id, SpacePatch { ceiling_finish: Some(Assigned::new(reference(value))), ..Default::default() })
}

fn parse_measure(text: &str) -> Option<AreaMeasure> {
    variant(text, &[AreaMeasure::Gross, AreaMeasure::Net])
}

fn distinct(items: Vec<String>) -> Vec<String> {
    items.into_iter().fold(Vec::new(), |mut kept, item| {
        if !kept.contains(&item) {
            kept.push(item);
        }
        kept
    })
}

/// 🗃️ The mutation that sets the usages an area scheme counts to the comma separated list a text holds (duplicates dropped, empty counts every usage).
pub fn write_scheme_usages(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    partial::<crate::mutations::set_area_scheme::SetAreaScheme, _>(id, "usages", &distinct(list(value))).map(ModelMutation::SetAreaScheme)
}

/// 🗃️ The mutation that sets the zones an area scheme counts to the comma separated zone ids or zone names a text holds; a token that names no zone refuses the whole edit.
pub fn write_scheme_zones(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let resolve = |token: String| snapshot.zones.get(&token).map(|_| token.clone()).or_else(|| snapshot.zones.iter().find(|(_, zone)| zone.name.to_lowercase() == token.to_lowercase()).map(|(zone, _)| zone.clone()));
    let zones = list(value).into_iter().map(resolve).collect::<Option<Vec<String>>>()?;
    partial::<crate::mutations::set_area_scheme::SetAreaScheme, _>(id, "zones", &distinct(zones)).map(ModelMutation::SetAreaScheme)
}
//#endregion 🔖️Writes

//#region 🔖️Fields
/// 🧾️ The authored parameters of a zone.
pub static ZONE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.zones.get(id).map(|row| row.name.clone()), rename),
    field!("category", field_category, Text, |s, id| s.zones.get(id).map(|row| row.category.clone()), parse_text => set_zone::SetZone),
    field!("occupancy_density", field_occupancy_density, Number, |s, id| s.zones.get(id).map(|row| number(row.occupancy_density)), parse_number => set_zone::SetZone),
];

/// 🧾️ The authored parameters of an area scheme: the rule of which spaces it adds up.
pub static AREA_SCHEME_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.area_schemes.get(id).map(|row| row.name.clone()), rename),
    field!("measure", field_measure, Text, |s, id| s.area_schemes.get(id).map(|row| format!("{:?}", row.measure)), choices: measure_choices, parse_measure => set_area_scheme::SetAreaScheme),
    field!("usages", field_usages, Text, |s, id| s.area_schemes.get(id).map(|row| row.usages.join(", ")), |s, id, value| write_scheme_usages(s, id, value)),
    field!("zones", field_zones, Text, |s, id| s.area_schemes.get(id).map(|row| row.zones.join(", ")), |s, id, value| write_scheme_zones(s, id, value)),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 🎨️ The area the finish of one surface of a resolved space covers, as text.
pub fn finish_area(inference: &ModelInference, id: &str, surface: FinishSurface) -> Option<String> {
    inference.quantities.elements.get(id).and_then(|row| row.finishes.iter().find(|finish| finish.surface == surface)).map(|finish| number(finish.area))
}

/// 💡️ The totals of a zone, read off the `zones` inference.
pub static ZONE_INFERRED: &[InferredRow] = &[
    inferred!("spaces", field_spaces, |_, inference, id| inference.zone_totals.get(id).map(|row| row.spaces.to_string())),
    inferred!("resolved", field_resolved, |_, inference, id| inference.zone_totals.get(id).map(|row| row.resolved.to_string())),
    inferred!("area", field_area, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.area))),
    inferred!("net_area", field_net_area, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.net_area))),
    inferred!("volume", field_volume, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.volume))),
    inferred!("occupancy", field_occupancy, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.occupancy))),
    inferred!("floor_finish_area", field_floor_finish_area, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.floor_finish_area))),
    inferred!("wall_finish_area", field_wall_finish_area, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.wall_finish_area))),
    inferred!("ceiling_finish_area", field_ceiling_finish_area, |_, inference, id| inference.zone_totals.get(id).map(|row| number(row.ceiling_finish_area))),
];

/// 🗃️ The distinct usages of the spaces of the model, so an area scheme can be written by what exists.
pub fn available_usages(snapshot: &ModelSnapshot) -> String {
    snapshot.spaces.values().map(|space| space.usage.as_str()).collect::<std::collections::BTreeSet<_>>().into_iter().collect::<Vec<_>>().join(", ")
}

/// 🏘️ The zones of the model as `id (name)`, so an area scheme can be written by what exists.
pub fn available_zones(snapshot: &ModelSnapshot) -> String {
    snapshot.zones.iter().map(|(id, zone)| format!("{id} ({})", zone.name)).collect::<Vec<_>>().join(", ")
}

/// 💡️ The totals of an area scheme, read off the `zones` inference, and the usages and zones the model offers to count.
pub static AREA_SCHEME_INFERRED: &[InferredRow] = &[
    inferred!("available_usages", field_available_usages, |s, _, id| s.area_schemes.contains_key(id).then(|| available_usages(s))),
    inferred!("available_zones", field_available_zones, |s, _, id| s.area_schemes.contains_key(id).then(|| available_zones(s))),
    inferred!("spaces", field_spaces, |_, inference, id| inference.scheme_totals.get(id).map(|row| row.spaces.to_string())),
    inferred!("resolved", field_resolved, |_, inference, id| inference.scheme_totals.get(id).map(|row| row.resolved.to_string())),
    inferred!("area", field_area, |_, inference, id| inference.scheme_totals.get(id).map(|row| number(row.area))),
    inferred!("volume", field_volume, |_, inference, id| inference.scheme_totals.get(id).map(|row| number(row.volume))),
    inferred!("occupancy", field_occupancy, |_, inference, id| inference.scheme_totals.get(id).map(|row| number(row.occupancy))),
];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 🏘️ A new zone: named, with no category and no occupancy.
pub fn create_zone(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateZone(crate::mutations::create_zone::CreateZone { id: id.into(), zone: Zone { name: name.into(), category: String::new(), occupancy_density: 0.0 } }))
}

/// 🗃️ A new area scheme: a net area counting every space.
pub fn create_area_scheme(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateAreaScheme(crate::mutations::create_area_scheme::CreateAreaScheme { id: id.into(), area_scheme: AreaScheme { name: name.into(), measure: AreaMeasure::Net, usages: Vec::new(), zones: Vec::new() } }))
}
//#endregion 🔖️Create

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
