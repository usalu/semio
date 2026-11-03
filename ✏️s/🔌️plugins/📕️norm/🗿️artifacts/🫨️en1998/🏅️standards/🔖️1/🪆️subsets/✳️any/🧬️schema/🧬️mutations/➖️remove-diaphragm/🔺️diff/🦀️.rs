//! ➖️ `remove-diaphragm` diff: removes the addressed diaphragm row.

use crate::remove_diaphragm::RemoveDiaphragm;
use semio_s_artifact_norm_en1998_diff::{En1998DiaphragmDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveDiaphragm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.diaphragms.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("diaphragm {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { diaphragms: En1998DiaphragmDelta::removal(&base.diaphragms, index), ..Default::default() })
}
