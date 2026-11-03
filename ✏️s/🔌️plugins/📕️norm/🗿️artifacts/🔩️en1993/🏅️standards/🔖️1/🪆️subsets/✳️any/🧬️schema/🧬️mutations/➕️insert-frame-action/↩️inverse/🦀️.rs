use crate::insert_frame_action::InsertFrameAction;
use crate::remove_frame_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFrameAction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_frame_action::RemoveFrameAction> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.frame_actions.len());
    vec![Op::from(remove_frame_action::RemoveFrameAction { index: at })]

    })())
}
