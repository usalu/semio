use super::ChangeWallLabelEn;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeWallLabelEn, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeWallLabelEn(ChangeWallLabelEn { index: payload.index, new_label_en: base.walls[payload.index].label_en.clone() })] }

    })())
}
