//! ↩️ `change-roof-parapet-height` inverse: restores the previous `parapet_height`.

use crate::change_roof_parapet_height::ChangeRoofParapetHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofParapetHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_parapet_height::ChangeRoofParapetHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofParapetHeight, base: &En1991Snapshot) -> Option<ChangeRoofParapetHeight> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofParapetHeight { id: payload.id.clone(), new_parapet_height: r0.parapet_height })
}
