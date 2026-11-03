use crate::insert_anchor::InsertAnchor;
use crate::remove_anchor::RemoveAnchor;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertAnchor, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_anchor::RemoveAnchor> {
    Ok((|| {
    let at = payload.index.min(base.anchors.len());
    vec![Op::from(RemoveAnchor { index: at })]

    })())
}
