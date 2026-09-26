use super::RemoveTowerLeg;
use crate::diff::En1993TowerList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveTowerLeg, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.tower_legs.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tower-leg index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.tower_legs.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { tower_legs: Some(En1993TowerList { values }), ..Default::default() })
}
