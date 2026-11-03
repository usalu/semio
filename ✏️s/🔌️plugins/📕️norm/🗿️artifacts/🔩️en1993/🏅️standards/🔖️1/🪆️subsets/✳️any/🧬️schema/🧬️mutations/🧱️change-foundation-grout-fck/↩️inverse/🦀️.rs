//! ↩️ `change-foundation-grout-fck` inverse: restores the previous `grout_fck`.

use crate::change_foundation_grout_fck::ChangeFoundationGroutFck;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationGroutFck, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_grout_fck::ChangeFoundationGroutFck> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationGroutFck { id: payload.id.clone(), new_grout_fck: row.grout_fck })],
        None => Vec::new(),
    })
}
