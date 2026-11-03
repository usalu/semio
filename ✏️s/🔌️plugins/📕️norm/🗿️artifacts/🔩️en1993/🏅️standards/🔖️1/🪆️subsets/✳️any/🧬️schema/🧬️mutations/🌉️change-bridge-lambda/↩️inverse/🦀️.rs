//! ↩️ `change-bridge-lambda` inverse: restores the previous `lambda`.

use crate::change_bridge_lambda::ChangeBridgeLambda;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeLambda, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_lambda::ChangeBridgeLambda> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeLambda { id: payload.id.clone(), new_lambda: row.lambda })],
        None => Vec::new(),
    })
}
