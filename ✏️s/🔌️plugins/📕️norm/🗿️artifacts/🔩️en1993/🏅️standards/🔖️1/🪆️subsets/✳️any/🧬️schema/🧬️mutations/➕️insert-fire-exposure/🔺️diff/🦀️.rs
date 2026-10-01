use super::InsertFireExposure;
use crate::diff::En1993FireList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertFireExposure, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fire_exposures.iter().any(|existing| existing.id == payload.fire_exposure.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fire exposure id {} already exists.", payload.fire_exposure.id), [payload.fire_exposure.id.clone()]);
    }
    let mut values = base.fire_exposures.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.fire_exposure.clone());
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: Some(En1993FireList { values }), ..Default::default() })
}
