//! ↩️ `change-bridge-lambda-max` inverse: restores the previous `lambda_max`.

use crate::change_bridge_lambda_max::ChangeBridgeLambdaMax;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeLambdaMax, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_lambda_max::ChangeBridgeLambdaMax> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeLambdaMax { id: payload.id.clone(), new_lambda_max: row.lambda_max })],
        None => Vec::new(),
    })
}
