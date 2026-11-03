//! ↩️ `change-anchor-bond-stress` inverse: restores the previous `bond_stress`.

use crate::change_anchor_bond_stress::ChangeAnchorBondStress;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorBondStress, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_bond_stress::ChangeAnchorBondStress> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorBondStress { id: payload.id.clone(), new_bond_stress: row.bond_stress })],
        None => Vec::new(),
    })
}
