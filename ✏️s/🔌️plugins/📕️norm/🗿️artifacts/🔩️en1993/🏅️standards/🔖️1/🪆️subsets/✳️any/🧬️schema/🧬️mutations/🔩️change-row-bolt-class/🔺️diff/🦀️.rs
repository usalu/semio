//! 🔺️ `change-row-bolt-class` diff: patches `bolt_class` of the addressed joint row row.

use crate::change_row_bolt_class::ChangeRowBoltClass;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointRowDelta, En1993JointRowPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRowBoltClass, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joint_rows.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-row", Vec::<String>::new());
    };
    if row.bolt_class == payload.new_bolt_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint row already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joint_rows: En1993JointRowDelta::modification(payload.id.clone(), En1993JointRowPatch { bolt_class: Some(payload.new_bolt_class.clone()), ..Default::default() }), ..Default::default() })
}
