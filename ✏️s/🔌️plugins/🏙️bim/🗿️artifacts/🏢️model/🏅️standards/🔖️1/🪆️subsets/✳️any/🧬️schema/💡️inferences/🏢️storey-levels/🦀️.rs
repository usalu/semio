//! 🪜️ `storey-levels`: the elevation of every storey, derived from stored heights and level indices only.
//!
//! Level 0 is the building datum. Storeys with a positive level stack upward from it, storeys with a negative level stack
//! downward. Each storey has at most one parent: the storey directly below it (positive side) or directly above it (negative
//! side), so changing one storey height re-infers exactly the storeys that stack on it. Elevations are building-relative;
//! the absolute pair adds the site and building elevations. This module is the pure arithmetic (`resolve`, `top_of`, `vertical_of`);
//! the `Storey` nodes of the model graph (`model-graph`) call it, and so does every consumer that needs a vertical extent.

use crate::standards::v1::subsets::any::schema::authored::storeys::{stacking, step};
use crate::{ModelSnapshot, TopConstraint};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

//#region 🔖️Value
/// 🪜️ Resolved elevations of one storey, in metres.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct StoreyLevel {
    pub elevation: f64,
    pub top_elevation: f64,
    pub absolute_elevation: f64,
    pub absolute_top_elevation: f64,
}
//#endregion 🔖️Value

//#region 🔖️Stacking
/// 🧭️ Every storey of the model in stacking order (buildings by id, each building bottom-up from level 0, then downward) with the id of its parent.
pub fn stackings(snapshot: &ModelSnapshot) -> Vec<(String, Option<String>)> {
    let buildings: std::collections::BTreeSet<&String> = snapshot.storeys.values().map(|storey| &storey.building).collect();
    buildings.into_iter().flat_map(|building| stacking(snapshot, building)).collect()
}

/// 📍️ The parent of a storey in the stacking of its building.
pub fn parent_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    let storey = snapshot.storeys.get(id)?;
    stacking(snapshot, &storey.building).into_iter().find(|(row, _)| row == id).and_then(|(_, parent)| parent)
}

/// 🧮️ The levels of a storey from its own parameters and its parent's already inferred levels.
pub fn resolve(snapshot: &ModelSnapshot, id: &str, parent: Option<&StoreyLevel>) -> StoreyLevel {
    let Some(storey) = snapshot.storeys.get(id) else { return StoreyLevel::default() };
    let datum = snapshot.buildings.get(&storey.building).map_or(0.0, |building| building.elevation + snapshot.sites.get(&building.site).map_or(0.0, |site| site.elevation));
    let elevation = step(storey.level, storey.height, parent.map(|level| (level.elevation, level.top_elevation)));
    let top = elevation + storey.height;
    StoreyLevel { elevation, top_elevation: top, absolute_elevation: datum + elevation, absolute_top_elevation: datum + top }
}

/// 🔑️ Everything `resolve` reads for `id` apart from the parent value.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(storey) = snapshot.storeys.get(id) else { return DslValue::Null };
    let building = snapshot.buildings.get(&storey.building);
    let site = building.and_then(|row| snapshot.sites.get(&row.site));
    DslValue::object([
        ("level".to_string(), semio_framework_value::ToValue::to_value(&storey.level)),
        ("height".to_string(), semio_framework_value::ToValue::to_value(&storey.height)),
        ("building_elevation".to_string(), semio_framework_value::ToValue::to_value(&building.map(|row| row.elevation))),
        ("site_elevation".to_string(), semio_framework_value::ToValue::to_value(&site.map(|row| row.elevation))),
    ])
}
//#endregion 🔖️Stacking

//#region 🔖️Vertical
/// 🔝️ The z of a top constraint, the one resolver of the model: `Unconnected` adds its height to the base, `StoreyTop` the offset to the own storey top, `Storey` the offset to the elevation of the target storey (the own storey when it is missing); an attached top (`Roof`, `Slab`, `Ceiling`) is the reference height of the flat model, the own storey top plus the offset: the wall layout replaces it by the surface of the target (`wall-layout/attach`).
pub fn top_of(top: &TopConstraint, base_z: f64, own: &StoreyLevel, target: Option<&StoreyLevel>) -> f64 {
    match top {
        TopConstraint::Unconnected { height } => base_z + height,
        TopConstraint::StoreyTop { offset } => own.top_elevation + offset,
        TopConstraint::Storey { offset, .. } => target.unwrap_or(own).elevation + offset,
        TopConstraint::Roof { offset, .. } | TopConstraint::Slab { offset, .. } | TopConstraint::Ceiling { offset, .. } => own.top_elevation + offset,
    }
}

/// 🔝️ Base and top `z` of an element from its base offset and top constraint.
pub fn vertical_of(base_offset: f64, top: &TopConstraint, own: &StoreyLevel, target: Option<&StoreyLevel>) -> (f64, f64) {
    let base_z = own.elevation + base_offset;
    (base_z, top_of(top, base_z, own, target))
}

/// 🪜️ The storeys an element is resolved by: its own and, for a `Storey` top constraint, the target (in this order).
pub fn constraint_storeys(storey: &str, top: &TopConstraint) -> Vec<String> {
    let mut storeys = vec![storey.to_string()];
    if let TopConstraint::Storey { storey: target, .. } = top {
        if target != storey {
            storeys.push(target.clone());
        }
    }
    storeys
}

/// 🧭️ The level a top constraint targets, from a set of levels.
pub fn target_of<'a>(top: &TopConstraint, levels: &'a BTreeMap<String, StoreyLevel>) -> Option<&'a StoreyLevel> {
    if let TopConstraint::Storey { storey, .. } = top {
        levels.get(storey)
    } else {
        None
    }
}
//#endregion 🔖️Vertical

//#region 🔖️Projection
/// 🪜️ The levels of every storey (the `Storey` nodes of the model graph).
#[cfg(test)]
pub fn compute_storey_levels(snapshot: &ModelSnapshot) -> BTreeMap<String, StoreyLevel> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::LEVELS }>(snapshot).storey_levels)
}
//#endregion 🔖️Projection

/// ⚖️ The differences between a committed third-party oracle table and a produced one: numbers within `1e-9` (relative above 1), arrays in order, objects by member, everything else equal.
#[cfg(test)]
pub fn table_problems(expected: &str, actual: &str) -> Vec<String> {
    fn walk(path: &str, expected: &serde_json::Value, actual: &serde_json::Value, problems: &mut Vec<String>) {
        use serde_json::Value;
        match (expected, actual) {
            (Value::Object(left), Value::Object(right)) => {
                let (names, others): (std::collections::BTreeSet<&String>, std::collections::BTreeSet<&String>) = (left.keys().collect(), right.keys().collect());
                if names != others {
                    problems.push(format!("{path}: members differ: oracle {names:?}, subject {others:?}"));
                }
                left.iter().filter(|(key, _)| right.contains_key(*key)).for_each(|(key, value)| walk(&format!("{path}.{key}"), value, &right[key], problems));
            }
            (Value::Array(left), Value::Array(right)) if left.len() == right.len() => left.iter().zip(right).enumerate().for_each(|(index, (a, b))| walk(&format!("{path}[{index}]"), a, b, problems)),
            (Value::Number(left), Value::Number(right)) => {
                let (want, got) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
                if !((want - got).abs() <= 1e-9 * want.abs().max(1.0)) {
                    problems.push(format!("{path}: oracle {want}, subject {got}"));
                }
            }
            _ if expected == actual => {}
            _ => problems.push(format!("{path}: oracle {expected}, subject {actual}")),
        }
    }
    let (Ok(expected), Ok(actual)) = (serde_json::from_str(expected), serde_json::from_str(actual)) else { return vec!["a table is not JSON".to_string()] };
    let mut problems = Vec::new();
    walk("$", &expected, &actual, &mut problems);
    problems
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
