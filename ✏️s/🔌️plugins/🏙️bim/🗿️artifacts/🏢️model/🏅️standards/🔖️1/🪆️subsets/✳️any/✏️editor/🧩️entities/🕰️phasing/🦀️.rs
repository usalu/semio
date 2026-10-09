//! 🕰️ The phase and storey rows of the entity table: the pickers of the construction phase of a phasable element and of the storey of a storey-placed element, and how a picked value becomes the
//! `set-element-phase` or `set-element-storey` mutation. One definition serves every kind that carries the field, so the properties panel offers the same combobox on a wall, a slab or a space.

use crate::editor::bim::terminology::BimLabels;
use crate::mutations::set_element_phase::SetElementPhase;
use crate::mutations::set_element_storey::SetElementStorey;
use crate::{ModelMutation, ModelSnapshot, Phase};

//#region 🔖️Phase
/// 🕰️ The four construction phases by their localized names, in the order of a project's life.
pub fn phase_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    [(Phase::Existing, &labels.phase_existing), (Phase::New, &labels.phase_new), (Phase::Demolished, &labels.phase_demolished), (Phase::Temporary, &labels.phase_temporary)]
        .into_iter()
        .map(|(phase, label)| (format!("{phase:?}"), label.as_str().to_string()))
        .collect()
}

/// 🕰️ The mutation that puts an element into the phase a picked value names; none for anything else.
pub fn write_phase(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Phase::parse(value).map(|phase| ModelMutation::SetElementPhase(SetElementPhase { id: id.into(), phase }))
}
//#endregion 🔖️Phase

//#region 🔖️Storey
/// 🪜️ The storeys an element can stand on: every storey of the model by its id and name, by building and level; a model with several buildings names the building too.
pub fn storey_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    let several = snapshot.buildings.len() > 1;
    let mut rows: Vec<(&str, i32, &str, String)> = snapshot
        .storeys
        .iter()
        .map(|(id, storey)| {
            let building = snapshot.buildings.get(&storey.building).map_or(storey.building.as_str(), |row| row.name.as_str());
            (storey.building.as_str(), storey.level, id.as_str(), if several { format!("{} ({building})", storey.name) } else { storey.name.clone() })
        })
        .collect();
    rows.sort();
    rows.into_iter().map(|(_, _, id, label)| (id.to_string(), label)).collect()
}

/// 🪜️ The mutation that stands an element on the storey a picked value names.
pub fn write_storey(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetElementStorey(SetElementStorey { id: id.into(), storey: value.trim().to_string() }))
}
//#endregion 🔖️Storey

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
