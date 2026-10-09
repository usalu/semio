//! 🖼️ Shared BIM rendering: everything the editor and the viewer draw identically lives here, once. `world` builds the World3d scene from the
//! `element-solids` inference, `plan` builds the Canvas2d layers from the `plan-linework` inference, `window_config` is the window-config
//! macro, and this root holds the model queries both need (storeys in stacking order, placed element ids, the storey and the name of an
//! element) used by both. The viewer must not import the editor,
//! so this module is the single place both surfaces import.

use crate::ModelSnapshot;

#[path = "🪟️window-config/🦀️.rs"]
pub mod window_config;
#[path = "🧊️world/🦀️.rs"]
pub mod world;
#[path = "🗺️plan/🦀️.rs"]
pub mod plan;

//#region 🔖️Storeys
/// 🏢️ One storey in stacking order for pickers, plan selection and visibility toggles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreyRow {
    pub id: String,
    pub building: String,
    pub name: String,
    pub level: i32,
}

/// 🏢️ Every storey ordered by building, then level, then id; level 0 is the ground storey and negative levels sit below it.
pub fn storeys(snapshot: &ModelSnapshot) -> Vec<StoreyRow> {
    let mut rows: Vec<StoreyRow> = snapshot.storeys.iter().map(|(id, storey)| StoreyRow { id: id.clone(), building: storey.building.clone(), name: storey.name.clone(), level: storey.level }).collect();
    rows.sort_by(|a, b| (&a.building, a.level, &a.id).cmp(&(&b.building, b.level, &b.id)));
    rows
}

/// 🏢️ The storey a plan window shows: the stored one while it still exists, otherwise the lowest storey of the model.
pub fn plan_storey(snapshot: &ModelSnapshot, stored: &str) -> Option<String> {
    if snapshot.storeys.contains_key(stored) {
        return Some(stored.to_string());
    }
    storeys(snapshot).into_iter().next().map(|row| row.id)
}
//#endregion 🔖️Storeys

//#region 🔖️Elements
/// 🧱️ Ids of every placed element that can be drawn or picked (walls, curtain walls, columns, beams, slabs, roofs, openings, wall sweeps, stairs, railings, ramps).
pub fn element_ids(snapshot: &ModelSnapshot) -> Vec<String> {
    let keys = |ids: Vec<&String>| ids.into_iter().cloned().collect::<Vec<String>>();
    [
        keys(snapshot.walls.keys().collect()),
        keys(snapshot.curtain_walls.keys().collect()),
        keys(snapshot.columns.keys().collect()),
        keys(snapshot.beams.keys().collect()),
        keys(snapshot.slabs.keys().collect()),
        keys(snapshot.ceilings.keys().collect()),
        keys(snapshot.roofs.keys().collect()),
        keys(snapshot.openings.keys().collect()),
        keys(snapshot.wall_sweeps.keys().collect()),
        keys(snapshot.stairs.keys().collect()),
        keys(snapshot.railings.keys().collect()),
        keys(snapshot.ramps.keys().collect()),
    ]
    .concat()
}

/// 🏢️ The storey an element stands on; an opening stands on the storey of its host wall.
pub fn storey_of<'a>(snapshot: &'a ModelSnapshot, element: &str) -> Option<&'a str> {
    let storey = |id: &'a String| Some(id.as_str());
    if let Some(wall) = snapshot.walls.get(element) {
        return storey(&wall.storey);
    }
    if let Some(opening) = snapshot.openings.get(element) {
        return snapshot.walls.get(&opening.host).and_then(|wall| storey(&wall.storey)).or_else(|| snapshot.curtain_walls.get(&opening.host).and_then(|wall| storey(&wall.storey)));
    }
    if let Some(sweep) = snapshot.wall_sweeps.get(element) {
        return snapshot.walls.get(&sweep.host).and_then(|wall| storey(&wall.storey));
    }
    snapshot
        .curtain_walls
        .get(element)
        .map(|item| &item.storey)
        .or_else(|| snapshot.columns.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.beams.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.slabs.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.ceilings.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.roofs.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.stairs.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.railings.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.ramps.get(element).map(|item| &item.storey))
        .and_then(storey)
}

/// 🏷️ The authored name of an element, empty when it has none.
pub fn element_name<'a>(snapshot: &'a ModelSnapshot, element: &str) -> &'a str {
    snapshot
        .walls
        .get(element)
        .map(|item| item.name.as_str())
        .or_else(|| snapshot.curtain_walls.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.columns.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.beams.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.slabs.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.ceilings.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.roofs.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.openings.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.wall_sweeps.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.stairs.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.railings.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.ramps.get(element).map(|item| item.name.as_str()))
        .unwrap_or_default()
}
//#endregion 🔖️Elements

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
