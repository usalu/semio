//! ↩️ `change-foundation-passive-resistance-n` inverse: restores the previous `passive_resistance_n`.

use crate::change_foundation_passive_resistance_n::ChangeFoundationPassiveResistanceN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPassiveResistanceN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_passive_resistance_n::ChangeFoundationPassiveResistanceN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPassiveResistanceN, base: &En1998Snapshot) -> Option<ChangeFoundationPassiveResistanceN> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPassiveResistanceN { id: payload.id.clone(), new_passive_resistance_n: r0.passive_resistance_n })
}
