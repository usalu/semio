//! ↩️ `change-frame-action-kind` inverse: restores the previous `kind`.

use crate::change_frame_action_kind::ChangeFrameActionKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_kind::ChangeFrameActionKind> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
