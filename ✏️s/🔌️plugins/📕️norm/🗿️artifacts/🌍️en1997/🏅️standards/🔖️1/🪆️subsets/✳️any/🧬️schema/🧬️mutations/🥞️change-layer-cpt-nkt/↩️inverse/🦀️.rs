//! ↩️ `change-layer-cpt-nkt` inverse: restores the previous `cpt_nkt`.

use crate::change_layer_cpt_nkt::ChangeLayerCptNkt;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLayerCptNkt, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_cpt_nkt::ChangeLayerCptNkt> {
    Ok(match base.layers.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeLayerCptNkt { id: payload.id.clone(), new_cpt_nkt: row.cpt_nkt })],
        None => Vec::new(),
    })
}
