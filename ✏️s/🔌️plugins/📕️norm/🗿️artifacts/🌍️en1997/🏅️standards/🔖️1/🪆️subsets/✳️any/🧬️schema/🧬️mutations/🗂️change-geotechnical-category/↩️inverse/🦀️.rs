use crate::change_geotechnical_category::ChangeGeotechnicalCategory;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeGeotechnicalCategory, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_geotechnical_category::ChangeGeotechnicalCategory> {
    Ok((|| {
    vec![Op::from(ChangeGeotechnicalCategory { new_geotechnical_category: base.geotechnical_category })]

    })())
}
