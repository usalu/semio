//! ➖️ `remove-bearing` diff: removes the addressed bearing row.

use crate::remove_bearing::RemoveBearing;
use semio_s_artifact_norm_en1998_diff::{En1998BearingDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveBearing, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.bearings.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bearing {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { bearings: En1998BearingDelta::removal(&base.bearings, index), ..Default::default() })
}
