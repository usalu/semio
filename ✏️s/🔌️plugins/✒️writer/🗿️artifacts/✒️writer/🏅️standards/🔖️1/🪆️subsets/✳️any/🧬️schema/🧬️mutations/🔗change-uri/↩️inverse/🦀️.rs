//! ↩️ Inverse for `ChangeUri` — reads the BASE uri, never the diff.
use super::ChangeUri;
use crate::WriterSnapshot;

//#region 🔖️Inverse
/// ↩️ Undo restores `base.uri`.
pub fn inverse<Op>(_payload: &ChangeUri, base: &WriterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_uri::ChangeUri>,
{
    Ok((|| {
    vec![Op::from(ChangeUri { new_uri: base.uri.clone() })]

    })())
}
//#endregion 🔖️Inverse
