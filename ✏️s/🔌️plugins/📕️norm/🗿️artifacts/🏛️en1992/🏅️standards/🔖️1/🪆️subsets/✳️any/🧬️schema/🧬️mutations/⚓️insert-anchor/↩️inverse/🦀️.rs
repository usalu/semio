use crate::insert_anchor::InsertAnchor;
use crate::remove_anchor::RemoveAnchor;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertAnchor, _base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_anchor::RemoveAnchor> {
    Ok((|| {
    vec![Op::from(RemoveAnchor { anchor_id: payload.anchor.id.clone() })]

    })())
}
