//! 🔺️ `change-forming-gain-forming` diff: patches `forming` of the addressed forming_gain row.

use crate::change_forming_gain_forming::ChangeFormingGainForming;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FormingGainDelta, En1993FormingGainPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFormingGainForming, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.forming_gains.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "forming_gain", Vec::<String>::new());
    };
    if row.forming == payload.new_forming {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The forming_gain already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { forming_gains: En1993FormingGainDelta::modification(payload.id.clone(), En1993FormingGainPatch { forming: Some(payload.new_forming), ..Default::default() }), ..Default::default() })
}
