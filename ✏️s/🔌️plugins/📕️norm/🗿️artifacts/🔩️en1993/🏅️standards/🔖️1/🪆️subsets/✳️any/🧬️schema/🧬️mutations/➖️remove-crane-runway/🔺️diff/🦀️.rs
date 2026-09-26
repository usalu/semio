use super::RemoveCraneRunway;
use crate::diff::En1993CraneList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveCraneRunway, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.crane_runways.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("crane-runway index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.crane_runways.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { crane_runways: Some(En1993CraneList { values }), ..Default::default() })
}
