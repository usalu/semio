use crate::change_truss_web_width::ChangeTrussWebWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTrussWebWidth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_truss_web_width::ChangeTrussWebWidth> {
    Ok(match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => vec![Op::from(ChangeTrussWebWidth { truss_id: payload.truss_id.clone(), new_value: item.web_width_m })],
        None => Vec::new(),
    })
}
