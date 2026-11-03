//! ↩️ `change-pile-neg-friction-beta` inverse: restores the previous `negative_friction_beta`.

use crate::change_pile_neg_friction_beta::ChangePileNegFrictionBeta;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileNegFrictionBeta, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_neg_friction_beta::ChangePileNegFrictionBeta> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileNegFrictionBeta { id: payload.id.clone(), new_negative_friction_beta: row.negative_friction_beta })],
        None => Vec::new(),
    })
}
