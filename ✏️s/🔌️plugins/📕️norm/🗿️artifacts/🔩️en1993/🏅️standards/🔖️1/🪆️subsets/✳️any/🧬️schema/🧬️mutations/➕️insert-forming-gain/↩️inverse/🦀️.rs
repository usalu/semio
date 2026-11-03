use crate::insert_forming_gain::InsertFormingGain;
use crate::remove_forming_gain;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFormingGain, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_forming_gain::RemoveFormingGain> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.forming_gains.len());
    vec![Op::from(remove_forming_gain::RemoveFormingGain { index: at })]

    })())
}
