//! 🔺️ `change-hollow-brace1-width` diff: patches `brace1_width` of the addressed hollow joint row.

use crate::change_hollow_brace1_width::ChangeHollowBrace1Width;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993HollowJointDelta, En1993HollowJointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeHollowBrace1Width, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_brace1_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.hollow_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "hollow-joint", Vec::<String>::new());
    };
    if row.brace1_width == payload.new_brace1_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The hollow joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { hollow_joints: En1993HollowJointDelta::modification(payload.id.clone(), En1993HollowJointPatch { brace1_width: Some(payload.new_brace1_width), ..Default::default() }), ..Default::default() })
}
