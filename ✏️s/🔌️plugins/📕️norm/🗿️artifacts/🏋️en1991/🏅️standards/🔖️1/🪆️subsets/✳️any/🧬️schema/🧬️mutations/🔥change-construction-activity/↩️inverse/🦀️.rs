//! Inverse for `change-construction-activity`.
use super::ChangeConstructionActivity;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeConstructionActivity, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeConstructionActivity(ChangeConstructionActivity { new_construction_activity: base.construction_activity.clone() })]

    })())
}
