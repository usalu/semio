//! ↩️ `change-bridge-lambda1` inverse: restores the previous `lambda_1`.

use crate::change_bridge_lambda_1::ChangeBridgeLambda1;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeLambda1, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_lambda_1::ChangeBridgeLambda1> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeLambda1 { id: payload.id.clone(), new_lambda_1: row.lambda_1 })],
        None => Vec::new(),
    })
}
