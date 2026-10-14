use crate::remove_anchor::RemoveAnchor;
use crate::insert_anchor::InsertAnchor;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveAnchor, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_anchor::InsertAnchor> {
    Ok((|| {
    let Some((index, anchor)) = base.anchors.iter().enumerate().find(|(_, a)| a.id == payload.anchor_id) else { return vec![]; };
    vec![Op::from(InsertAnchor { index, anchor: anchor.clone() })]

    })())
}
