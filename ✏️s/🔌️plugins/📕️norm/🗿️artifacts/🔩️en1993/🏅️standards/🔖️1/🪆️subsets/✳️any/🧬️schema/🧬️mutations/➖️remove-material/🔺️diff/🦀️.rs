use super::RemoveMaterial;
use crate::diff::En1993MaterialList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.materials.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("material index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.materials.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { materials: Some(En1993MaterialList { values }), ..Default::default() })
}
