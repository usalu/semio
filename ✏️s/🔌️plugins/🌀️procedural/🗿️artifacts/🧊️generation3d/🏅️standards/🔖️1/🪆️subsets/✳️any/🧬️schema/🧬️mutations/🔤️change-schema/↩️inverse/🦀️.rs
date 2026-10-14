//! ↩️ `change-schema` inverse — self-inverse: the pre-state schema string captured from `base`.

use crate::change_schema::ChangeSchema;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse<Op: From<ChangeSchema>>(_payload: &ChangeSchema, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(ChangeSchema { new_schema: base.host_snapshot.schema.clone() })]

    })())
}
