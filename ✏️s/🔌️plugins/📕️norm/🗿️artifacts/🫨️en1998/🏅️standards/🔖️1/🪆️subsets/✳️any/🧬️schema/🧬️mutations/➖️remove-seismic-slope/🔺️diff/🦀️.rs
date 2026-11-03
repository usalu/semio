//! ➖️ `remove-seismic-slope` diff: removes the addressed seismic_slope row.

use crate::remove_seismic_slope::RemoveSeismicSlope;
use semio_s_artifact_norm_en1998_diff::{En1998SlopeDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveSeismicSlope, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.slopes.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("seismic_slope {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { slopes: En1998SlopeDelta::removal(&base.slopes, index), ..Default::default() })
}
