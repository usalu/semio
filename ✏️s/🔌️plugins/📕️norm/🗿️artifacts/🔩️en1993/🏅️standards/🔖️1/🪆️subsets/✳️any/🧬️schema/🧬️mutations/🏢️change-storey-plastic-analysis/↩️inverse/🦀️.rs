//! ↩️ `change-storey-plastic-analysis` inverse: restores the previous `plastic_analysis`.

use crate::change_storey_plastic_analysis::ChangeStoreyPlasticAnalysis;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyPlasticAnalysis, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_plastic_analysis::ChangeStoreyPlasticAnalysis> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyPlasticAnalysis { id: payload.id.clone(), new_plastic_analysis: row.plastic_analysis })],
        None => Vec::new(),
    })
}
