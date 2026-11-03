//! 🔺️ `change-crane-crane-kq` diff: patches `crane_kq` of the addressed crane runway row.

use crate::change_crane_crane_kq::ChangeCraneCraneKq;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993CraneRunwayDelta, En1993CraneRunwayPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCraneCraneKq, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_crane_kq.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.crane_runways.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "crane", Vec::<String>::new());
    };
    if row.crane_kq == payload.new_crane_kq {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane runway already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::modification(payload.id.clone(), En1993CraneRunwayPatch { crane_kq: Some(payload.new_crane_kq), ..Default::default() }), ..Default::default() })
}
