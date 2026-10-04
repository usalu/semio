use super::ChangeFireRei;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeFireRei, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeFireRei(ChangeFireRei { index: payload.index, new_fire_rei_min: base.walls[payload.index].fire_rei_min })] }

    })())
}
