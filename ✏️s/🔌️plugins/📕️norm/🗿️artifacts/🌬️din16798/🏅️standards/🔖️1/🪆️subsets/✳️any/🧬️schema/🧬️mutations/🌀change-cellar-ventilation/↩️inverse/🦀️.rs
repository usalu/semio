//! ↩️ `change-cellar-ventilation` inverse.
use crate::change_cellar_ventilation::ChangeCellarVentilation;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeCellarVentilation, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cellar_ventilation::ChangeCellarVentilation> {
    Ok((|| {
    vec![Op::from(ChangeCellarVentilation { new_cellar_ventilation_m3_h: base.cellar_ventilation_m3_h })]

    })())
}
