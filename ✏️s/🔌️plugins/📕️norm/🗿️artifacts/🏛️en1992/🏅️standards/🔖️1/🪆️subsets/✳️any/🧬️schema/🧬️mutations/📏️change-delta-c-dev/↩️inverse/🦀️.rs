use crate::change_delta_c_dev::ChangeDeltaCDev;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(_payload: &ChangeDeltaCDev, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_delta_c_dev::ChangeDeltaCDev> {
    Ok((|| {
    vec![Op::from(ChangeDeltaCDev { new_delta_c_dev: base.delta_c_dev })]

    })())
}
