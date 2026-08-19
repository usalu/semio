//! ↩️ Inverse for `ChangeSchema` — restores the captured BASE schema. The schema field always
//! exists (it is not optional on the fixture), so this is unconditional.

use crate::artifacts::procedural2d::mutations::{change_schema, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub async fn inverse(_payload: &super::mutation::ChangeSchema, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    vec![change_schema(base.fixture.schema.clone())]
}
