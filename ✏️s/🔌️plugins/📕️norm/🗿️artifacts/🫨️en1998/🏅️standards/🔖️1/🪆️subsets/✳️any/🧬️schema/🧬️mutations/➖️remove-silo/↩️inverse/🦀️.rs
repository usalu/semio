//! ↩️ Inverse for `remove-silo`.
use crate::remove_silo::RemoveSilo;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_silo;

pub fn inverse<Op>(payload: &RemoveSilo, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_silo::InsertSilo> {
    Ok((|| {
    match base.silos.get(payload.index) {
        Some(item) => vec![Op::from(insert_silo::InsertSilo { index: Some(payload.index), silo: item.clone() })],
        None => Vec::new(),
    }

    })())
}
