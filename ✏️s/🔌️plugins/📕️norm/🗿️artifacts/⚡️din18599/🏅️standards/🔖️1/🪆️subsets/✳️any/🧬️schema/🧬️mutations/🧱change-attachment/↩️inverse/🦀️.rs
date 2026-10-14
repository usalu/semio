//! ↩️ `change-attachment` inverse.

use crate::change_attachment::ChangeAttachment;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeAttachment, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_attachment::ChangeAttachment> {
    Ok((|| {
    vec![Op::from(ChangeAttachment { new_attachment: base.attachment })]

    })())
}
