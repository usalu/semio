//! ↩️ `change-schema` inverse — self-inverse: the pre-state schema string captured from `base`.

use crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(_payload: &ChangeSchema, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: base.host_snapshot.schema.clone() })]

    })())
}
