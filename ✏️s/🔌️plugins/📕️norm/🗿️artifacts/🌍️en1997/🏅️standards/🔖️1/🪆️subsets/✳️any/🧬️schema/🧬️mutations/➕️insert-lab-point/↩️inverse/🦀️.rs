use crate::insert_lab_point::InsertLabPoint;
use crate::remove_lab_point::RemoveLabPoint;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertLabPoint, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_lab_point::RemoveLabPoint> {
    let at = base.lab_tests.iter().find(|row| row.id == payload.lab_test_id).map_or(0, |parent| payload.index.min(parent.points.len()));
    Ok(vec![Op::from(RemoveLabPoint { lab_test_id: payload.lab_test_id.clone(), index: at })])
}
