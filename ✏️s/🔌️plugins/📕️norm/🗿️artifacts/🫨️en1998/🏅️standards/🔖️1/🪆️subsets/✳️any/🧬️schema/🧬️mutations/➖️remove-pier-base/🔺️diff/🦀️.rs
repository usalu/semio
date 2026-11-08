//! ➖️ `remove-pier-base` diff: removes the addressed pier_base row.

use crate::remove_pier_base::RemovePierBase;
use semio_s_artifact_norm_en1998_diff::{En1998PierBaseDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemovePierBase, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.pier_bases.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier_base {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { pier_bases: En1998PierBaseDelta::removal(&base.pier_bases, index), ..Default::default() })
}
