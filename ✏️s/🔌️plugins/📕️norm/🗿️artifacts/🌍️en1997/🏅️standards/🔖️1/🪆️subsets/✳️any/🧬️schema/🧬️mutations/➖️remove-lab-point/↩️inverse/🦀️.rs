use crate::remove_lab_point::RemoveLabPoint;
use crate::insert_lab_point::InsertLabPoint;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveLabPoint, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_lab_point::InsertLabPoint> {
    Ok(match base.lab_tests.iter().find(|row| row.id == payload.lab_test_id).and_then(|parent| parent.points.get(payload.index).cloned()) {
        Some(lab_point) => vec![Op::from(InsertLabPoint { lab_test_id: payload.lab_test_id.clone(), index: payload.index, lab_point })],
        None => Vec::new(),
    })
}
