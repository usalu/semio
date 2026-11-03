use crate::insert_anchor_test::InsertAnchorTest;
use crate::remove_anchor_test::RemoveAnchorTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertAnchorTest, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_anchor_test::RemoveAnchorTest> {
    let at = base.anchors.iter().find(|row| row.id == payload.anchor_id).map_or(0, |parent| payload.index.min(parent.tests.len()));
    Ok(vec![Op::from(RemoveAnchorTest { anchor_id: payload.anchor_id.clone(), index: at })])
}
