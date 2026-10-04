use super::ChangeEccentricityBottom;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeEccentricityBottom, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeEccentricityBottom(ChangeEccentricityBottom { index: payload.index, new_eccentricity_bottom_m: base.walls[payload.index].eccentricity_bottom_m })] }

    })())
}
