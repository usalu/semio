//! ↩️ `change-pile-elastic-modulus` inverse: restores the previous `elastic_modulus`.

use crate::change_pile_elastic_modulus::ChangePileElasticModulus;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileElasticModulus, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_elastic_modulus::ChangePileElasticModulus> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileElasticModulus { id: payload.id.clone(), new_elastic_modulus: row.elastic_modulus })],
        None => Vec::new(),
    })
}
