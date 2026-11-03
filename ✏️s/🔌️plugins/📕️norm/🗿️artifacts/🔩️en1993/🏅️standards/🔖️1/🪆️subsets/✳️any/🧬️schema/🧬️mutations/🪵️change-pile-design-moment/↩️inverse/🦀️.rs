//! ↩️ `change-pile-design-moment` inverse: restores the previous `design_moment`.

use crate::change_pile_design_moment::ChangePileDesignMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileDesignMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_design_moment::ChangePileDesignMoment> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDesignMoment { id: payload.id.clone(), new_design_moment: row.design_moment })],
        None => Vec::new(),
    })
}
