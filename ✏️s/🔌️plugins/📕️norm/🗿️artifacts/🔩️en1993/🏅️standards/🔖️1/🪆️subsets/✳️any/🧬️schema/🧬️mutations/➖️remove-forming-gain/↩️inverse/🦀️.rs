use crate::remove_forming_gain::RemoveFormingGain;
use crate::insert_forming_gain;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFormingGain, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_forming_gain::InsertFormingGain> {
    Ok((|| {
    if payload.index >= base.forming_gains.len() { return Vec::new(); }
    vec![Op::from(insert_forming_gain::InsertFormingGain { index: Some(payload.index), forming_gain: base.forming_gains[payload.index].clone() })]

    })())
}
