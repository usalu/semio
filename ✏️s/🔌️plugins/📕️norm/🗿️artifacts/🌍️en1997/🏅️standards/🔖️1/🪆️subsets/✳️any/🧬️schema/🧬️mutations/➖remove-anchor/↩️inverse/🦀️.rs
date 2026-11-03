use crate::remove_anchor::RemoveAnchor;
use crate::insert_anchor::InsertAnchor;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveAnchor, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_anchor::InsertAnchor> {
    Ok((|| {
    match base.anchors.get(payload.index).cloned() {
        Some(anchor) => vec![Op::from(InsertAnchor { index: payload.index, anchor })],
        None => Vec::new(),
    }

    })())
}
