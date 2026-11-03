//! ↩️ `change-pile-group-single-res` inverse: restores the previous `single_resistance`.

use crate::change_pile_group_single_res::ChangePileGroupSingleRes;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupSingleRes, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_single_res::ChangePileGroupSingleRes> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupSingleRes { id: payload.id.clone(), new_single_resistance: row.single_resistance })],
        None => Vec::new(),
    })
}
