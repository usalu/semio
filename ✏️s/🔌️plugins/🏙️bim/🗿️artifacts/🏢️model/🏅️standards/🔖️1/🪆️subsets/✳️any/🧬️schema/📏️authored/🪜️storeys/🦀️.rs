//! 🪜️ `storeys`: the stacking of the storeys of a building and the elevation of a storey as the pure sum of authored heights. Level 0 is the building datum; storeys with a positive level stack upward
//! from it, storeys with a negative level stack downward, and each storey has at most one parent, the storey directly below it (positive side) or directly above it (negative side).

use crate::ModelSnapshot;

/// 🧭️ The storeys of one building ordered by level, with the id of each storey's single parent.
pub fn stacking(snapshot: &ModelSnapshot, building: &str) -> Vec<(String, Option<String>)> {
    let mut rows: Vec<(i32, &String)> = snapshot.storeys.iter().filter(|(_, storey)| storey.building == building).map(|(id, storey)| (storey.level, id)).collect();
    rows.sort();
    let upward: Vec<&(i32, &String)> = rows.iter().filter(|(level, _)| *level >= 0).collect();
    let downward: Vec<&(i32, &String)> = rows.iter().rev().filter(|(level, _)| *level < 0).collect();
    let chain = |ordered: Vec<&(i32, &String)>| -> Vec<(String, Option<String>)> {
        ordered.iter().enumerate().map(|(index, (_, id))| ((*id).clone(), index.checked_sub(1).map(|previous| ordered[previous].1.clone()))).collect()
    };
    let mut plan = chain(upward);
    plan.extend(chain(downward));
    plan
}

/// 📐️ The elevation of a storey above the building datum from its own level and height and its parent's `(elevation, top)`: the top of the parent below a positive level, the elevation of the parent above less the own height below the datum.
pub fn step(level: i32, height: f64, parent: Option<(f64, f64)>) -> f64 {
    match (level >= 0, parent) {
        (true, Some((_, top_below))) => top_below,
        (true, None) => 0.0,
        (false, Some((elevation_above, _))) => elevation_above - height,
        (false, None) => -height,
    }
}

/// 🪜️ The elevation of storey `id` above its building datum, the pure sum of the authored heights along the stacking of its building; none when the storey is absent.
pub fn elevation(snapshot: &ModelSnapshot, id: &str) -> Option<f64> {
    let storey = snapshot.storeys.get(id)?;
    let mut spans: std::collections::BTreeMap<String, (f64, f64)> = std::collections::BTreeMap::new();
    for (key, parent) in stacking(snapshot, &storey.building) {
        let row = snapshot.storeys.get(&key)?;
        let at = step(row.level, row.height, parent.as_ref().and_then(|parent| spans.get(parent)).copied());
        if key == id {
            return Some(at);
        }
        spans.insert(key, (at, at + row.height));
    }
    None
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
