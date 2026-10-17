//! ↩️ Inverse reconstruction for `resize-source-frame` — reads the BASE frame, never the diff.
use super::ResizeSourceFrame;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source.frame` — captured from pre-state, never from the applied diff.
pub fn inverse<Op>(_payload: &ResizeSourceFrame, base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_source_frame::ResizeSourceFrame>,
{
    Ok((|| {
    let (source, _) = crate::presentation_working_scene(base);
    vec![Op::from(ResizeSourceFrame { new_frame: source.frame })]

    })())
}
//#endregion 🔹Inverse
