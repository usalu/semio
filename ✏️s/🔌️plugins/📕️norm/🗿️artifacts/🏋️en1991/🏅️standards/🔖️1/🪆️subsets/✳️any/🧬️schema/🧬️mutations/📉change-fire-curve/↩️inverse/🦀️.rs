//! Inverse for `change-fire-curve`.
use super::ChangeFireCurve;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireCurve, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeFireCurve(ChangeFireCurve { new_fire_curve: base.fire_curve })]

    })())
}
