use crate::change_groundwater_level::ChangeGroundwaterLevel;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeGroundwaterLevel, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_groundwater_level::ChangeGroundwaterLevel> {
    Ok((|| {
    vec![Op::from(ChangeGroundwaterLevel { new_groundwater_level: base.groundwater_level })]

    })())
}
