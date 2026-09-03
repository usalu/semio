//! ↩️ Inverse for `ChangeSchema` — restores the captured BASE schema. The schema field always
//! exists (it is not optional on the fixture), so this is unconditional.

use crate::artifacts::generation2d::mutations::{change_schema, Generation2dMutation};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn inverse(_payload: &super::ChangeSchema, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    vec![change_schema(base.fixture.schema.clone())]
}
