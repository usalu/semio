use crate::insert_frame_bar::InsertFrameBar;
use crate::remove_frame_bar;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFrameBar, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_frame_bar::RemoveFrameBar> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.frame_bars.len());
    vec![Op::from(remove_frame_bar::RemoveFrameBar { index: at })]

    })())
}
