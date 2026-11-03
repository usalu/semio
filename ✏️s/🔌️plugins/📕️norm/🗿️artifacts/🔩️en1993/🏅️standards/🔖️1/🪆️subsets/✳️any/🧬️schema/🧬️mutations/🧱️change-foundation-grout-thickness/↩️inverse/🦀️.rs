//! ↩️ `change-foundation-grout-thickness` inverse: restores the previous `grout_thickness`.

use crate::change_foundation_grout_thickness::ChangeFoundationGroutThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationGroutThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_grout_thickness::ChangeFoundationGroutThickness> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationGroutThickness { id: payload.id.clone(), new_grout_thickness: row.grout_thickness })],
        None => Vec::new(),
    })
}
