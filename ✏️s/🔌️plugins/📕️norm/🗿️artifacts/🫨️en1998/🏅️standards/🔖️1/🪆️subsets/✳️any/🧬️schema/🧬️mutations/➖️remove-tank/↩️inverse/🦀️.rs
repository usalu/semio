//! Inverse for `remove-tank`.
use crate::remove_tank::RemoveTank;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_tank;

pub fn inverse<Op>(payload: &RemoveTank, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tank::InsertTank> {
    Ok((|| {
    match base.tanks.get(payload.index) {
        Some(item) => vec![Op::from(insert_tank::InsertTank { index: Some(payload.index), tank: item.clone() })],
        None => Vec::new(),
    }

    })())
}
