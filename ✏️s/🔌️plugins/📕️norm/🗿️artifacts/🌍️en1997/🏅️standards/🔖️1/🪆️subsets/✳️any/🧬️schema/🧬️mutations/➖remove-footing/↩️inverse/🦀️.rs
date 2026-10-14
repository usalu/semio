use crate::remove_footing::RemoveFooting;
use crate::insert_footing::InsertFooting;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveFooting, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_footing::InsertFooting> {
    Ok((|| {
    let footing = base.footings.get(payload.index).cloned().unwrap_or_else(|| base.footings[0].clone());
    vec![Op::from(InsertFooting { index: payload.index, footing })]

    })())
}
