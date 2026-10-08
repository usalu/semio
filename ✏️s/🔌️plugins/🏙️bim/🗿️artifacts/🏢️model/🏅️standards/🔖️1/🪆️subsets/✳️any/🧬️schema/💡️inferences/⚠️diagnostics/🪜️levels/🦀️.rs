//! 🪜️ Storey levels of one building: levels that skip a number leave a gap, two storeys on one level overlap, a building without level 0 has no
//! datum; spaces of the building that share a number are ambiguous.

use super::{Diagnostic, DiagnosticCode};
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 🪜️ The level and number findings of one building.
pub fn building(snapshot: &ModelSnapshot, building: &str) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    let mut by_level: BTreeMap<i32, Vec<&str>> = BTreeMap::new();
    for (id, storey) in snapshot.storeys.iter().filter(|(_, storey)| storey.building == building) {
        by_level.entry(storey.level).or_default().push(id);
    }
    for (level, ids) in by_level.iter().filter(|(_, ids)| ids.len() > 1) {
        found.push(Diagnostic::new(DiagnosticCode::StoreyLevelDuplicate, &ids).with("level", f64::from(*level)));
    }
    if !by_level.is_empty() && !by_level.contains_key(&0) {
        found.push(Diagnostic::new(DiagnosticCode::StoreyNoDatum, &[building]));
    }
    let levels: Vec<(&i32, &Vec<&str>)> = by_level.iter().collect();
    for pair in levels.windows(2) {
        let ((from, below), (to, above)) = (pair[0], pair[1]);
        if to - from > 1 {
            found.push(Diagnostic::new(DiagnosticCode::StoreyLevelGap, &[below[0], above[0]]).with("from", f64::from(*from)).with("to", f64::from(*to)));
        }
    }
    let mut numbers: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (id, space) in snapshot.spaces.iter().filter(|(_, space)| snapshot.storeys.get(&space.storey).is_some_and(|storey| storey.building == building)) {
        if !space.number.is_empty() {
            numbers.entry(&space.number).or_default().push(id);
        }
    }
    found.extend(numbers.values().filter(|ids| ids.len() > 1).map(|ids| Diagnostic::new(DiagnosticCode::SpaceDuplicateNumber, ids)));
    found
}
