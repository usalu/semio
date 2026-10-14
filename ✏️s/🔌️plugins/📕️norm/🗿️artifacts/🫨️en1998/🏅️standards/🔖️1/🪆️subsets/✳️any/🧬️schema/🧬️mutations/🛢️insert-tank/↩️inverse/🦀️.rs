//! Inverse for `insert-tank`.
use crate::insert_tank::InsertTank;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_tank;

pub fn inverse<Op>(payload: &InsertTank, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tank::RemoveTank> {
    Ok((|| {
    vec![Op::from(remove_tank::RemoveTank { index: payload.index.unwrap_or(usize::MAX).min(base.tanks.len()) })]

    })())
}
