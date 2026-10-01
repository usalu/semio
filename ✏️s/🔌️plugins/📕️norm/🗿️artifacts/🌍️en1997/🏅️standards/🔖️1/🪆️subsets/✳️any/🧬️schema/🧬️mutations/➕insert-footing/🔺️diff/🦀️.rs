use super::InsertFooting;
use crate::diff::{En1997Diff, En1997FootingList};
use crate::En1997Snapshot;
pub fn diff(payload: &InsertFooting, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.footings.iter().any(|existing| existing.id == payload.footing.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Footing id {} already exists.", payload.footing.id), [payload.footing.id.clone()]);
    }
    let mut footings = base.footings.clone();
    let at = payload.index.min(footings.len());
    footings.insert(at, payload.footing.clone());
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingList { values: footings }), ..Default::default() })
}
