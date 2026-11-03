//! ⚙ `change-rail-derailment-assumed-line-load` diff: patches `assumed_line_load` of the addressed rail derailment row.

use crate::change_rail_derailment_assumed_line_load::ChangeRailDerailmentAssumedLineLoad;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDerailmentDelta, En1991RailDerailmentPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailDerailmentAssumedLineLoad, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_line_load.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.rail_derailments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail derailment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_line_load == payload.new_assumed_line_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The rail derailment already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_derailments: En1991RailDerailmentDelta::modification(&payload.id, En1991RailDerailmentPatch { assumed_line_load: Some(payload.new_assumed_line_load), ..Default::default() }), ..Default::default() })
}
