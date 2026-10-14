//! Inverse for `change-silo-kind`.
use crate::change_silo_kind::ChangeSiloKind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloKind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_kind::ChangeSiloKind> {
    Ok((|| {
    vec![Op::from(ChangeSiloKind { new_silo_kind: base.silo_kind.clone() })]

    })())
}
