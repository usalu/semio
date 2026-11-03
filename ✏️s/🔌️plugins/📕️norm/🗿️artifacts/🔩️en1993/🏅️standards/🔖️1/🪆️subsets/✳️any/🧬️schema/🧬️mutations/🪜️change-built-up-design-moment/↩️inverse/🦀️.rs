//! ↩️ `change-built-up-design-moment` inverse: restores the previous `design_moment`.

use crate::change_built_up_design_moment::ChangeBuiltUpDesignMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpDesignMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_design_moment::ChangeBuiltUpDesignMoment> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpDesignMoment { id: payload.id.clone(), new_design_moment: row.design_moment })],
        None => Vec::new(),
    })
}
