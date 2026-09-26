use super::ChangeGeotechnicalCategory;
use crate::diff::En1997Diff;
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeGeotechnicalCategory, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !(1..=3).contains(&payload.new_geotechnical_category) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "geotechnical category must be 1..3", Vec::<String>::new());
    }
    if base.geotechnical_category == payload.new_geotechnical_category {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "category unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { geotechnical_category: Some(payload.new_geotechnical_category), ..Default::default() })
}
