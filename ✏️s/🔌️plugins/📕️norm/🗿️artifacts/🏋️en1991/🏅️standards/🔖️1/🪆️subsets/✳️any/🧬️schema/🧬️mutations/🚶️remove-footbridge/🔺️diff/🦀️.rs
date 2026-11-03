//! 🚶 `remove-footbridge` diff: removes the addressed footbridge row.

use crate::remove_footbridge::RemoveFootbridge;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FootbridgeDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveFootbridge, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.footbridges.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Footbridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { footbridges: En1991FootbridgeDelta::removal(&base.footbridges, index), ..Default::default() })
}
