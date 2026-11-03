//! ↩️ `change-pile-group-shaft-coh-sum` inverse: restores the previous `shaft_cohesion_sum`.

use crate::change_pile_group_shaft_coh_sum::ChangePileGroupShaftCohSum;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupShaftCohSum, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_shaft_coh_sum::ChangePileGroupShaftCohSum> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupShaftCohSum { id: payload.id.clone(), new_shaft_cohesion_sum: row.shaft_cohesion_sum })],
        None => Vec::new(),
    })
}
