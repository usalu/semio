//! 🔺️ `change-form-title` — sparse diff construction.

use super::mutation::ChangeFormTitle;
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_change_form_title(payload: &ChangeFormTitle, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if payload.new_title == base.title {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Form title already has this value.");
    }
    protocol::MutationOutcome::new(FormsDiff { title: Some(payload.new_title.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
