//! ↩️ `change-cellar-area` inverse.
use crate::change_cellar_area::ChangeCellarArea;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeCellarArea, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cellar_area::ChangeCellarArea> {
    Ok((|| {
    vec![Op::from(ChangeCellarArea { new_cellar_area_m2: base.cellar_area_m2 })]

    })())
}
