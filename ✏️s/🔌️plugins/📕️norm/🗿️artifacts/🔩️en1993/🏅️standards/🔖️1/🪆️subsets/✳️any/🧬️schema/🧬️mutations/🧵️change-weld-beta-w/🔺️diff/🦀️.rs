//! 🔺️ `change-weld-beta-w` diff: patches `beta_w` of the addressed weld group row.

use crate::change_weld_beta_w::ChangeWeldBetaW;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993WeldGroupDelta, En1993WeldGroupPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeWeldBetaW, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_beta_w.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.weld_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "weld-group", Vec::<String>::new());
    };
    if row.beta_w == payload.new_beta_w {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The weld group already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { weld_groups: En1993WeldGroupDelta::modification(payload.id.clone(), En1993WeldGroupPatch { beta_w: Some(payload.new_beta_w), ..Default::default() }), ..Default::default() })
}
