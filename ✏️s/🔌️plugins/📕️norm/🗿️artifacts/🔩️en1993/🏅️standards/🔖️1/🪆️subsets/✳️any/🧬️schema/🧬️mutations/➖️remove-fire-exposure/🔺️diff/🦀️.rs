use super::RemoveFireExposure;
use crate::diff::En1993FireList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveFireExposure, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.fire_exposures.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fire-exposure index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.fire_exposures.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: Some(En1993FireList { values }), ..Default::default() })
}
