//! ↩️ `change-schema` inverse — self-inverse: the pre-state schema string captured from `base`.

use crate::artifacts::generation3d::mutations::change_schema::ChangeSchema;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;

pub fn inverse(_payload: &ChangeSchema, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: base.fixture.schema.clone() })]
}
