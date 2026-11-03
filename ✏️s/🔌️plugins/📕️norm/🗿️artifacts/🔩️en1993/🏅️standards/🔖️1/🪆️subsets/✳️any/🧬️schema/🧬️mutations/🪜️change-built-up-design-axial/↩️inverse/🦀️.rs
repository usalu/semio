//! ↩️ `change-built-up-design-axial` inverse: restores the previous `design_axial`.

use crate::change_built_up_design_axial::ChangeBuiltUpDesignAxial;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpDesignAxial, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_design_axial::ChangeBuiltUpDesignAxial> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpDesignAxial { id: payload.id.clone(), new_design_axial: row.design_axial })],
        None => Vec::new(),
    })
}
