use crate::remove_anchor_test::RemoveAnchorTest;
use crate::insert_anchor_test::InsertAnchorTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveAnchorTest, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_anchor_test::InsertAnchorTest> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.anchor_id).and_then(|parent| parent.tests.get(payload.index).cloned()) {
        Some(anchor_test) => vec![Op::from(InsertAnchorTest { anchor_id: payload.anchor_id.clone(), index: payload.index, anchor_test })],
        None => Vec::new(),
    })
}
