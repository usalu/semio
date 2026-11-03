//! ↩️ `insert-factor-override` inverse: removes the inserted row again.

use crate::insert_factor_override::InsertFactorOverride;
use crate::remove_factor_override::RemoveFactorOverride;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertFactorOverride, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_factor_override::RemoveFactorOverride> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertFactorOverride, base: &En1990Snapshot) -> Option<RemoveFactorOverride> {
    let index = payload.index.unwrap_or(usize::MAX).min(base.factor_overrides.len());
    Some(RemoveFactorOverride { index })
}
