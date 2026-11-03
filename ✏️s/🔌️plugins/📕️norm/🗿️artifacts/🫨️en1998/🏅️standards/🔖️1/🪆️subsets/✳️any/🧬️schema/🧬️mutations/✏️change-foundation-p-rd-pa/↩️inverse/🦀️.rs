//! ↩️ `change-foundation-p-rd-pa` inverse: restores the previous `p_rd_pa`.

use crate::change_foundation_p_rd_pa::ChangeFoundationPRdPa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPRdPa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_p_rd_pa::ChangeFoundationPRdPa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPRdPa, base: &En1998Snapshot) -> Option<ChangeFoundationPRdPa> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPRdPa { id: payload.id.clone(), new_p_rd_pa: r0.p_rd_pa })
}
