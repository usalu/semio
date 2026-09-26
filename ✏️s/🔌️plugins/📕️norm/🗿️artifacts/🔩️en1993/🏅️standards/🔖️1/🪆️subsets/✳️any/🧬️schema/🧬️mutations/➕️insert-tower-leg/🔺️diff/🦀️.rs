use super::InsertTowerLeg;
use crate::diff::En1993TowerList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertTowerLeg, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.tower_legs.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.tower_leg.clone());
    protocol::MutationOutcome::new(En1993Diff { tower_legs: Some(En1993TowerList { values }), ..Default::default() })
}
