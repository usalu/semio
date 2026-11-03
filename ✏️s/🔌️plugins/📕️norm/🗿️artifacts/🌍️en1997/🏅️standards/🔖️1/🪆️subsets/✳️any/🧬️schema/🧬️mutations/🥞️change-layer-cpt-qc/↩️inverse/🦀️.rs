//! ↩️ `change-layer-cpt-qc` inverse: restores the previous `cpt_qc`.

use crate::change_layer_cpt_qc::ChangeLayerCptQc;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerCptQc, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_cpt_qc::ChangeLayerCptQc> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerCptQc { id: payload.id.clone(), new_cpt_qc: row.cpt_qc })],
        None => Vec::new(),
    })
}
