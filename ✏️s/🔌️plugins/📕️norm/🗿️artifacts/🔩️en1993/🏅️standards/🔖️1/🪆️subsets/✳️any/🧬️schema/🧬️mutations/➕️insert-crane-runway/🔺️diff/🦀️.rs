use super::InsertCraneRunway;
use crate::diff::En1993CraneList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertCraneRunway, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.crane_runways.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.crane_runway.clone());
    protocol::MutationOutcome::new(En1993Diff { crane_runways: Some(En1993CraneList { values }), ..Default::default() })
}
