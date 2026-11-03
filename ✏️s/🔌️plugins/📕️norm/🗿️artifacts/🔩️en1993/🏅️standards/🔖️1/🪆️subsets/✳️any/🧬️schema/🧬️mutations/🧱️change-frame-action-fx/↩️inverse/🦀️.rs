//! ↩️ `change-frame-action-fx` inverse: restores the previous `fx`.

use crate::change_frame_action_fx::ChangeFrameActionFx;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionFx, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_fx::ChangeFrameActionFx> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionFx { id: payload.id.clone(), new_fx: row.fx })],
        None => Vec::new(),
    })
}
