//! 🌉 `remove-wind-bridge` diff: removes the addressed bridge wind case row.

use crate::remove_wind_bridge::RemoveWindBridge;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindBridgeDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveWindBridge, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.wind_bridges.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bridge wind case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { wind_bridges: En1991WindBridgeDelta::removal(&base.wind_bridges, index), ..Default::default() })
}
