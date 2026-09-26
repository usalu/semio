use super::ChangeGeotechnicalCategory;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(_payload: &ChangeGeotechnicalCategory, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeGeotechnicalCategory(ChangeGeotechnicalCategory { new_geotechnical_category: base.geotechnical_category })]
}
