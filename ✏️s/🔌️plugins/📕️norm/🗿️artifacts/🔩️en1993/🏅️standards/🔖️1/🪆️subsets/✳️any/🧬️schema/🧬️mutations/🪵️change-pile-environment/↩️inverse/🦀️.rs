//! ↩️ `change-pile-environment` inverse: restores the previous `environment`.

use crate::change_pile_environment::ChangePileEnvironment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileEnvironment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_environment::ChangePileEnvironment> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileEnvironment { id: payload.id.clone(), new_environment: row.environment })],
        None => Vec::new(),
    })
}
