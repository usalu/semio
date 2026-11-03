//! ↩️ `change-dynamic-assumed-cscd` inverse: restores the previous `assumed_cscd`.

use crate::change_dynamic_assumed_cscd::ChangeDynamicAssumedCscd;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicAssumedCscd, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_assumed_cscd::ChangeDynamicAssumedCscd> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicAssumedCscd, base: &En1991Snapshot) -> Option<ChangeDynamicAssumedCscd> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicAssumedCscd { id: payload.id.clone(), new_assumed_cscd: r0.assumed_cscd })
}
