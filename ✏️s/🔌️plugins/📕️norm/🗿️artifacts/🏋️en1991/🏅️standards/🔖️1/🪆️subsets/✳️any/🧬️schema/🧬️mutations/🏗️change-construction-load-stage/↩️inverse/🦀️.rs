//! ↩️ `change-construction-load-stage` inverse: restores the previous `stage`.

use crate::change_construction_load_stage::ChangeConstructionLoadStage;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeConstructionLoadStage, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_construction_load_stage::ChangeConstructionLoadStage> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConstructionLoadStage, base: &En1991Snapshot) -> Option<ChangeConstructionLoadStage> {
    let r0 = base.construction_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConstructionLoadStage { id: payload.id.clone(), new_stage: r0.stage.clone() })
}
