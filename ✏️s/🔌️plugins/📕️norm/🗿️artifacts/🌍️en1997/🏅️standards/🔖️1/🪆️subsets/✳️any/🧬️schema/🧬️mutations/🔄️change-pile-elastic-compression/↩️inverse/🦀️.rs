//! ↩️ `change-pile-elastic-compression` inverse: restores the previous `elastic_compression`.

use crate::change_pile_elastic_compression::ChangePileElasticCompression;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileElasticCompression, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_elastic_compression::ChangePileElasticCompression> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileElasticCompression { id: payload.id.clone(), new_elastic_compression: row.elastic_compression })],
        None => Vec::new(),
    })
}
