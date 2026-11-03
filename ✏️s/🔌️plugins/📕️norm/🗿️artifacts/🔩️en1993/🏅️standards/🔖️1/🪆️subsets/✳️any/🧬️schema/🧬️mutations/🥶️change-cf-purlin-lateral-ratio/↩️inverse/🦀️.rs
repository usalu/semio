//! ↩️ `change-cf-purlin-lateral-ratio` inverse: restores the previous `purlin_lateral_ratio`.

use crate::change_cf_purlin_lateral_ratio::ChangeCfPurlinLateralRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfPurlinLateralRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_purlin_lateral_ratio::ChangeCfPurlinLateralRatio> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfPurlinLateralRatio { id: payload.id.clone(), new_purlin_lateral_ratio: row.purlin_lateral_ratio })],
        None => Vec::new(),
    })
}
