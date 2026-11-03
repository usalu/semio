use crate::change_taper_side::ChangeTaperSide;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTaperSide, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_taper_side::ChangeTaperSide> {
    Ok(match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => vec![Op::from(ChangeTaperSide { taper_id: payload.taper_id.clone(), new_value: item.tension_side })],
        None => Vec::new(),
    })
}
