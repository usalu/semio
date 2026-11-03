//! 🏗 `remove-crane-hoist` diff: removes the addressed crane hoist row.

use crate::remove_crane_hoist::RemoveCraneHoist;
use semio_s_artifact_norm_en1991_diff::{En1991CraneHoistDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveCraneHoist, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.crane_hoists.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Crane hoist {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { crane_hoists: En1991CraneHoistDelta::removal(&base.crane_hoists, index), ..Default::default() })
}
