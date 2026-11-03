use crate::remove_lab_test::RemoveLabTest;
use crate::insert_lab_test::InsertLabTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveLabTest, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_lab_test::InsertLabTest> {
    Ok((|| {
    match base.lab_tests.get(payload.index).cloned() {
        Some(lab_test) => vec![Op::from(InsertLabTest { index: payload.index, lab_test })],
        None => Vec::new(),
    }

    })())
}
