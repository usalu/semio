//! 🖼️ Shared BIM rendering: everything the editor and the viewer draw identically lives here, once. `world` builds the World3d scene from the
//! `element-solids` inference, `plan` builds the Canvas2d layers from the `plan-linework` inference, `window_config` is the window-config
//! macro, and this root holds the model queries both need (storeys in stacking order, placed element ids, the storey and the name of an
//! element) plus the one-entry inference memo that keeps a camera move from re-inferring the model. The viewer must not import the editor,
//! so this module is the single place both surfaces import.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, ElementSolid};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{compute_plan_linework, PlanLinework};
use crate::ModelSnapshot;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

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
/// 🧱️ Ids of every placed element that can be drawn or picked (walls, curtain walls, columns, beams, slabs, roofs, openings, stairs, railings).
pub fn element_ids(snapshot: &ModelSnapshot) -> Vec<String> {
    let keys = |ids: Vec<&String>| ids.into_iter().cloned().collect::<Vec<String>>();
    [
        keys(snapshot.walls.keys().collect()),
        keys(snapshot.curtain_walls.keys().collect()),
        keys(snapshot.columns.keys().collect()),
        keys(snapshot.beams.keys().collect()),
        keys(snapshot.slabs.keys().collect()),
        keys(snapshot.roofs.keys().collect()),
        keys(snapshot.openings.keys().collect()),
        keys(snapshot.stairs.keys().collect()),
        keys(snapshot.railings.keys().collect()),
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
    snapshot
        .curtain_walls
        .get(element)
        .map(|item| &item.storey)
        .or_else(|| snapshot.columns.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.beams.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.slabs.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.roofs.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.stairs.get(element).map(|item| &item.storey))
        .or_else(|| snapshot.railings.get(element).map(|item| &item.storey))
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
        .or_else(|| snapshot.roofs.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.openings.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.stairs.get(element).map(|item| item.name.as_str()))
        .or_else(|| snapshot.railings.get(element).map(|item| item.name.as_str()))
        .unwrap_or_default()
}
//#endregion 🔖️Elements

//#region 🔖️Inference
struct Memo {
    snapshot: ModelSnapshot,
    solids: Option<Rc<BTreeMap<String, ElementSolid>>>,
    plans: Option<Rc<BTreeMap<String, PlanLinework>>>,
}

thread_local! {
    static MEMO: RefCell<Option<Memo>> = const { RefCell::new(None) };
}

fn memoised<R>(snapshot: &ModelSnapshot, read: impl FnOnce(&mut Memo) -> R) -> R {
    MEMO.with(|memo| {
        let mut memo = memo.borrow_mut();
        if memo.as_ref().is_none_or(|cached| cached.snapshot != *snapshot) {
            *memo = Some(Memo { snapshot: snapshot.clone(), solids: None, plans: None });
        }
        read(memo.as_mut().expect("the memo was just filled"))
    })
}

/// 🧊️ The `element-solids` inference of `snapshot`, memoised for the last snapshot only: re-rendering for a camera or selection change reuses
/// it, a document change replaces it. The memo holds exactly one model, so it never grows.
pub fn solids(snapshot: &ModelSnapshot) -> Rc<BTreeMap<String, ElementSolid>> {
    memoised(snapshot, |memo| Rc::clone(memo.solids.get_or_insert_with(|| Rc::new(compute_element_solids(snapshot)))))
}

/// 🗺️ The `plan-linework` inference of `snapshot` (one plan per storey id), memoised like [`solids`].
pub fn plans(snapshot: &ModelSnapshot) -> Rc<BTreeMap<String, PlanLinework>> {
    memoised(snapshot, |memo| Rc::clone(memo.plans.get_or_insert_with(|| Rc::new(compute_plan_linework(snapshot)))))
}
//#endregion 🔖️Inference

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
