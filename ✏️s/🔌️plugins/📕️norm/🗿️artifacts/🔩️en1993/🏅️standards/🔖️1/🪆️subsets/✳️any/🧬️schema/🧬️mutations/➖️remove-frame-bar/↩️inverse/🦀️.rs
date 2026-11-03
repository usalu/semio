use crate::remove_frame_bar::RemoveFrameBar;
use crate::insert_frame_bar;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFrameBar, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_frame_bar::InsertFrameBar> {
    Ok((|| {
    if payload.index >= base.frame_bars.len() { return Vec::new(); }
    vec![Op::from(insert_frame_bar::InsertFrameBar { index: Some(payload.index), frame_bar: base.frame_bars[payload.index].clone() })]

    })())
}
