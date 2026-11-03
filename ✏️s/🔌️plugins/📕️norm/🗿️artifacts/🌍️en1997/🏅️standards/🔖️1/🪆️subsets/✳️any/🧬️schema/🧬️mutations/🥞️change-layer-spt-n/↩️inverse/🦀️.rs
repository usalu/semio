//! ↩️ `change-layer-spt-n` inverse: restores the previous `spt_n`.

use crate::change_layer_spt_n::ChangeLayerSptN;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerSptN, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_spt_n::ChangeLayerSptN> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerSptN { id: payload.id.clone(), new_spt_n: row.spt_n })],
        None => Vec::new(),
    })
}
