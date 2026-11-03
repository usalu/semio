//! ↩️ `change-construction-load-kind` inverse: restores the previous `kind`.

use crate::change_construction_load_kind::ChangeConstructionLoadKind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeConstructionLoadKind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_construction_load_kind::ChangeConstructionLoadKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConstructionLoadKind, base: &En1991Snapshot) -> Option<ChangeConstructionLoadKind> {
    let r0 = base.construction_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConstructionLoadKind { id: payload.id.clone(), new_kind: r0.kind })
}
