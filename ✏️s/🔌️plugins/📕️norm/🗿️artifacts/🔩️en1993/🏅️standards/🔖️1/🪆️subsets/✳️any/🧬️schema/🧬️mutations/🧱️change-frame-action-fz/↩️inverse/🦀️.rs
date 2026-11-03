//! ↩️ `change-frame-action-fz` inverse: restores the previous `fz`.

use crate::change_frame_action_fz::ChangeFrameActionFz;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameActionFz, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_action_fz::ChangeFrameActionFz> {
    Ok(match base.frame_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameActionFz { id: payload.id.clone(), new_fz: row.fz })],
        None => Vec::new(),
    })
}
