//! ↩️ Inverse for `ChangeSchema` — restores the captured BASE schema. The schema field always
//! exists (it is not optional on the fixture), so this is unconditional.

use crate::change_schema;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use crate::change_schema::component::ChangeSchema;

pub fn inverse<Op: From<ChangeSchema>>(_payload: &super::ChangeSchema, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![change_schema(base.host_snapshot.schema.clone())]

    })())
}
