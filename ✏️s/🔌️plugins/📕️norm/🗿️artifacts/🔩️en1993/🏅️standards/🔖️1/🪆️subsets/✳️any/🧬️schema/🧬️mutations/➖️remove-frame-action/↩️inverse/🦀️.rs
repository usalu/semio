use crate::remove_frame_action::RemoveFrameAction;
use crate::insert_frame_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFrameAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_frame_action::InsertFrameAction> {
    Ok((|| {
    if payload.index >= base.frame_actions.len() { return Vec::new(); }
    vec![Op::from(insert_frame_action::InsertFrameAction { index: Some(payload.index), frame_action: base.frame_actions[payload.index].clone() })]

    })())
}
