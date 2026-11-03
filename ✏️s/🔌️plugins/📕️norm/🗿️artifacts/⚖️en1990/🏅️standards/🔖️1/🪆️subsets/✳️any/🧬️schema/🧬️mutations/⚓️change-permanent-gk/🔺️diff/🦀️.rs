//! ⚓️ `change-permanent-gk` diff: patches `gk` of the addressed permanent action row.

use crate::change_permanent_gk::ChangePermanentGk;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990PermanentDelta, En1990PermanentPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangePermanentGk, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_gk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.permanents.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Permanent action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.gk == payload.new_gk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The permanent action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta::modification(&payload.id, En1990PermanentPatch { gk: Some(payload.new_gk), ..Default::default() }), ..Default::default() })
}
