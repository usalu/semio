//! ↩️ Inverse for `change-fire-load-density-qf`.
use crate::change_fire_load_density_qf::ChangeFireLoadDensityQf;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireLoadDensityQf, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_load_density_qf::ChangeFireLoadDensityQf> {
    Ok((|| {
    vec![Op::from(ChangeFireLoadDensityQf { new_fire_load_density_qf: base.fire_load_density_qf })]

    })())
}
