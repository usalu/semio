//! ↩️ `remove-imperfection` inverse: inserts the removed row back at its position.

use crate::remove_imperfection::RemoveImperfection;
use crate::insert_imperfection::InsertImperfection;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveImperfection, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_imperfection::InsertImperfection> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveImperfection, base: &En1991Snapshot) -> Option<InsertImperfection> {
    let (index, row) = base.execution_imperfections.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertImperfection { index, item: row.clone() })
}
