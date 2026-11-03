//! 🛤 `change-rail-derailment-situation` diff: patches `situation` of the addressed fatigue situation row.

use crate::change_rail_derailment_situation::ChangeRailDerailmentSituation;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDerailmentDelta, En1991RailDerailmentPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailDerailmentSituation, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.rail_derailments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue situation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.situation == payload.new_situation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue situation already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_derailments: En1991RailDerailmentDelta::modification(&payload.id, En1991RailDerailmentPatch { situation: Some(payload.new_situation.clone()), ..Default::default() }), ..Default::default() })
}
