use crate::insert_footing::InsertFooting;
use crate::remove_footing::RemoveFooting;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertFooting, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_footing::RemoveFooting> {
    Ok((|| {
    let at = payload.index.min(base.footings.len());
    vec![Op::from(RemoveFooting { index: at })]

    })())
}
