//! ⚙ `remove-rail-derailment` diff: removes the addressed rail derailment row.

use crate::remove_rail_derailment::RemoveRailDerailment;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDerailmentDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveRailDerailment, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.rail_derailments.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail derailment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { rail_derailments: En1991RailDerailmentDelta::removal(&base.rail_derailments, index), ..Default::default() })
}
