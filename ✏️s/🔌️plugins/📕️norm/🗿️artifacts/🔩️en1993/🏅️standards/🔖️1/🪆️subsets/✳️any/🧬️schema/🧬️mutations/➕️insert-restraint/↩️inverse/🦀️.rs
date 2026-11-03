use crate::insert_restraint::InsertRestraint;
use crate::remove_restraint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertRestraint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_restraint::RemoveRestraint> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.restraints.len());
    vec![Op::from(remove_restraint::RemoveRestraint { index: at })]

    })())
}
