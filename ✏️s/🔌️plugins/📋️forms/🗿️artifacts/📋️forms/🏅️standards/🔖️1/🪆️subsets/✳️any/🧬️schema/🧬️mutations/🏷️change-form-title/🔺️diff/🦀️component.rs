//! 🔺️ `change-form-title` — sparse diff construction.

use super::mutation::ChangeFormTitle;
use crate::artifacts::forms::FormsDiff;

//#region 🔖️Diff
pub fn diff_change_form_title(payload: &ChangeFormTitle) -> protocol::MutationOutcome<FormsDiff> {
    protocol::MutationOutcome::new(FormsDiff { title: Some(payload.new_title.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
