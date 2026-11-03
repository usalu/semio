//! ↩️ `change-cf-spring-factor` inverse: restores the previous `spring_factor`.

use crate::change_cf_spring_factor::ChangeCfSpringFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfSpringFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_spring_factor::ChangeCfSpringFactor> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfSpringFactor { id: payload.id.clone(), new_spring_factor: row.spring_factor })],
        None => Vec::new(),
    })
}
