//! ➖️ `remove-material-ductility` diff — removes the row at the index.

use crate::remove_material_ductility::RemoveMaterialDuctility;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MaterialDuctilityDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveMaterialDuctility, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.material_ductility.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("material-ductility index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { material_ductility: En1993MaterialDuctilityDelta::removal(&base.material_ductility, payload.index), ..Default::default() })
}
