//! ↩️ `change-roof-multi-span` inverse: restores the previous `multi_span`.

use crate::change_roof_multi_span::ChangeRoofMultiSpan;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofMultiSpan, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_multi_span::ChangeRoofMultiSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofMultiSpan, base: &En1991Snapshot) -> Option<ChangeRoofMultiSpan> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofMultiSpan { id: payload.id.clone(), new_multi_span: r0.multi_span })
}
