//! 🔺️ `change-forming-gain-gross-area` diff: patches `gross_area` of the addressed forming_gain block row.

use crate::change_forming_gain_gross_area::ChangeFormingGainGrossArea;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FormingGainDelta, En1993FormingGainPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFormingGainGrossArea, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_gross_area.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.forming_gains.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "forming-gain", Vec::<String>::new());
    };
    if row.gross_area == payload.new_gross_area {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The forming_gain block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { forming_gains: En1993FormingGainDelta::modification(payload.id.clone(), En1993FormingGainPatch { gross_area: Some(payload.new_gross_area), ..Default::default() }), ..Default::default() })
}
