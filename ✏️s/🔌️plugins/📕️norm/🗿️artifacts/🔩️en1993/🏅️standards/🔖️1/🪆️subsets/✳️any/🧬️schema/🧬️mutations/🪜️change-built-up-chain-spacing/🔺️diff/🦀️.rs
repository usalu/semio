//! 🔺️ `change-built-up-chain-spacing` diff: patches `chain_spacing` of the addressed built-up member row.

use crate::change_built_up_chain_spacing::ChangeBuiltUpChainSpacing;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BuiltUpMemberDelta, En1993BuiltUpMemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBuiltUpChainSpacing, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_chain_spacing.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.built_up_members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "built-up", Vec::<String>::new());
    };
    if row.chain_spacing == payload.new_chain_spacing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The built-up member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { built_up_members: En1993BuiltUpMemberDelta::modification(payload.id.clone(), En1993BuiltUpMemberPatch { chain_spacing: Some(payload.new_chain_spacing), ..Default::default() }), ..Default::default() })
}
