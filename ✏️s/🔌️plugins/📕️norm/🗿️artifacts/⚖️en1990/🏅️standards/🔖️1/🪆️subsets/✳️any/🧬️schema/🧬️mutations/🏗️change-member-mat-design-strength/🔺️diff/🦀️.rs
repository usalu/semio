//! 🏗️ `change-member-mat-design-strength` diff: patches `material_design_strength` of the addressed member row.

use crate::change_member_mat_design_strength::ChangeMemberMatDesignStrength;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990MemberDelta, En1990MemberPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeMemberMatDesignStrength, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_material_design_strength.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.material_design_strength == payload.new_material_design_strength {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { members: En1990MemberDelta::modification(&payload.id, En1990MemberPatch { material_design_strength: Some(payload.new_material_design_strength), ..Default::default() }), ..Default::default() })
}
