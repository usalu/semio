use super::InsertMaterial;
use crate::diff::En1993MaterialList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertMaterial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.materials.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.material.clone());
    protocol::MutationOutcome::new(En1993Diff { materials: Some(En1993MaterialList { values }), ..Default::default() })
}
