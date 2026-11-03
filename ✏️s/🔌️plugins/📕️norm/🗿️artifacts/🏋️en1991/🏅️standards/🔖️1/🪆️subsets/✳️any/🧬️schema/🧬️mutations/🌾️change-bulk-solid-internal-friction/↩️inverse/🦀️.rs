//! ↩️ `change-bulk-solid-internal-friction` inverse: restores the previous `internal_friction`.

use crate::change_bulk_solid_internal_friction::ChangeBulkSolidInternalFriction;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeBulkSolidInternalFriction, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bulk_solid_internal_friction::ChangeBulkSolidInternalFriction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBulkSolidInternalFriction, base: &En1991Snapshot) -> Option<ChangeBulkSolidInternalFriction> {
    let r0 = base.bulk_solids.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBulkSolidInternalFriction { id: payload.id.clone(), new_internal_friction: r0.internal_friction })
}
