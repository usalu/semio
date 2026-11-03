//! ↩️ `change-roof-has-parapet` inverse: restores the previous `has_parapet`.

use crate::change_roof_has_parapet::ChangeRoofHasParapet;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofHasParapet, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_has_parapet::ChangeRoofHasParapet> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofHasParapet, base: &En1991Snapshot) -> Option<ChangeRoofHasParapet> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofHasParapet { id: payload.id.clone(), new_has_parapet: r0.has_parapet })
}
