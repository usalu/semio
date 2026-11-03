//! ↩️ `change-tower-lattice-kind` inverse: restores the previous `lattice_kind`.

use crate::change_tower_lattice_kind::ChangeTowerLatticeKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerLatticeKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_lattice_kind::ChangeTowerLatticeKind> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerLatticeKind { id: payload.id.clone(), new_lattice_kind: row.lattice_kind })],
        None => Vec::new(),
    })
}
