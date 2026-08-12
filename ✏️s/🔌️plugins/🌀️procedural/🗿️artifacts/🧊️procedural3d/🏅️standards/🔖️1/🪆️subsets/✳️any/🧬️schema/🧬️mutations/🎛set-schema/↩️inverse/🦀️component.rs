//! ↩️ `change-schema` inverse — self-inverse: the pre-state schema string captured from `base`.

use crate::artifacts::procedural3d::mutations::set_schema::mutation::ChangeSchema;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub fn inverse(_payload: &ChangeSchema, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    vec![Procedural3dMutation::ChangeSchema(ChangeSchema { new_schema: base.fixture.schema.clone() })]
}
