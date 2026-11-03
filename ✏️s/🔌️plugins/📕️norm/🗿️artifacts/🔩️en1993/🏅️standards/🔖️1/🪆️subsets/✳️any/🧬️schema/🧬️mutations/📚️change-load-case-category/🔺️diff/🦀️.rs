//! 🔺️ `change-load-case-category` diff: patches `category` of the addressed load case row.

use crate::change_load_case_category::ChangeLoadCaseCategory;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993LoadCaseDelta, En1993LoadCasePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeLoadCaseCategory, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.load_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "load-case", Vec::<String>::new());
    };
    if row.category == payload.new_category {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The load case already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { load_cases: En1993LoadCaseDelta::modification(payload.id.clone(), En1993LoadCasePatch { category: Some(payload.new_category.clone()), ..Default::default() }), ..Default::default() })
}
