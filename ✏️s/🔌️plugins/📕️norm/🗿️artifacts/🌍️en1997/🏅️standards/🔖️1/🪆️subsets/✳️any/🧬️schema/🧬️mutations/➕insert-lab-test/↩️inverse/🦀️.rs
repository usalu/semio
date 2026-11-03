use crate::insert_lab_test::InsertLabTest;
use crate::remove_lab_test::RemoveLabTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertLabTest, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_lab_test::RemoveLabTest> {
    Ok((|| {
    let at = payload.index.min(base.lab_tests.len());
    vec![Op::from(RemoveLabTest { index: at })]

    })())
}
