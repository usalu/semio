use super::ChangeWallLabelDe;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeWallLabelDe, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeWallLabelDe(ChangeWallLabelDe { index: payload.index, new_label_de: base.walls[payload.index].label_de.clone() })] }

    })())
}
