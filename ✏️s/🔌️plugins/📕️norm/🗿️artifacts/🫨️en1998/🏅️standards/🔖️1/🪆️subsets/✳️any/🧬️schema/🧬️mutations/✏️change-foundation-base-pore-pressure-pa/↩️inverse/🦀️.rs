//! ↩️ Inverse for `change-foundation-base-pore-pressure-pa`.
use crate::change_foundation_base_pore_pressure_pa::ChangeFoundationBasePorePressurePa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBasePorePressurePa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_base_pore_pressure_pa::ChangeFoundationBasePorePressurePa> {
    Ok(match base.foundations.get(payload.index) {
        Some(row) => vec![Op::from(ChangeFoundationBasePorePressurePa { index: payload.index, new_base_pore_pressure_pa: row.base_pore_pressure_pa })],
        None => Vec::new(),
    })
}
