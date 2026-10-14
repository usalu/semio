//! ↩️ `change-outdoor-co2` inverse.
use crate::change_outdoor_co2::ChangeOutdoorCo2;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeOutdoorCo2, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_outdoor_co2::ChangeOutdoorCo2> {
    Ok((|| {
    vec![Op::from(ChangeOutdoorCo2 { new_outdoor_co2_ppm: base.outdoor_co2_ppm })]

    })())
}
