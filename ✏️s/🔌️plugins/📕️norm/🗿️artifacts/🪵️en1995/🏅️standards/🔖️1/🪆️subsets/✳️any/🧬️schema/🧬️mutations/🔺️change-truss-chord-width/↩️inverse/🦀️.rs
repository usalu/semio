use crate::change_truss_chord_width::ChangeTrussChordWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTrussChordWidth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_truss_chord_width::ChangeTrussChordWidth> {
    Ok(match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => vec![Op::from(ChangeTrussChordWidth { truss_id: payload.truss_id.clone(), new_value: item.chord_width_m })],
        None => Vec::new(),
    })
}
