//! ↩️ `change-connections` inverse.

use crate::change_connections::ChangeConnections;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeConnections, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_connections::ChangeConnections> {
    Ok((|| {
    vec![Op::from(ChangeConnections { connections: base.connections.clone() })]

    })())
}
