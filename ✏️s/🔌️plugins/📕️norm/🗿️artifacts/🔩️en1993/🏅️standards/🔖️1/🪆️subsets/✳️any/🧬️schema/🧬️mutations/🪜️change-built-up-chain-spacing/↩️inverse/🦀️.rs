//! ↩️ `change-built-up-chain-spacing` inverse: restores the previous `chain_spacing`.

use crate::change_built_up_chain_spacing::ChangeBuiltUpChainSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpChainSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_chain_spacing::ChangeBuiltUpChainSpacing> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpChainSpacing { id: payload.id.clone(), new_chain_spacing: row.chain_spacing })],
        None => Vec::new(),
    })
}
