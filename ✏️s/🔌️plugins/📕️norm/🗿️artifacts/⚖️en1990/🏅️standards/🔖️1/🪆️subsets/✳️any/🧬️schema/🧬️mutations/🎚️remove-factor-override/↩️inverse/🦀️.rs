//! ↩️ `remove-factor-override` inverse: inserts the removed row back at its position.

use crate::remove_factor_override::RemoveFactorOverride;
use crate::insert_factor_override::InsertFactorOverride;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveFactorOverride, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_factor_override::InsertFactorOverride> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFactorOverride, base: &En1990Snapshot) -> Option<InsertFactorOverride> {
    let index = payload.index;
    let row = base.factor_overrides.get(index)?;
    Some(InsertFactorOverride { index: Some(index), item: row.clone() })
}
