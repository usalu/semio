//! ↩️ Inverse reconstruction for `replace-source` — reads the BASE source, never the diff.
use super::ReplaceSource;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source` — captured from pre-state, never from the applied diff.
pub fn inverse<Op>(_payload: &ReplaceSource, base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_source::ReplaceSource>,
{
    Ok((|| {
    let (source, _) = crate::presentation_working_scene(base);
    vec![Op::from(ReplaceSource { new_source: source })]

    })())
}
//#endregion 🔹Inverse
