use crate::remove_restraint::RemoveRestraint;
use crate::insert_restraint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveRestraint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_restraint::InsertRestraint> {
    Ok((|| {
    if payload.index >= base.restraints.len() { return Vec::new(); }
    vec![Op::from(insert_restraint::InsertRestraint { index: Some(payload.index), restraint: base.restraints[payload.index].clone() })]

    })())
}
