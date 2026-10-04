use super::ChangeEccentricityTop;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeEccentricityTop, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeEccentricityTop(ChangeEccentricityTop { index: payload.index, new_eccentricity_top_m: base.walls[payload.index].eccentricity_top_m })] }

    })())
}
