//! 🔺️ `change-form-title` — sparse diff construction.

use crate::standards::v1::subsets::any::schema::mutations::change_form_title::mutation::ChangeFormTitle;
use crate::schema::diff::FormsOptionalText;
use crate::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_change_form_title(payload: &ChangeFormTitle, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if payload.new_title == base.title {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Form title already has this value.");
    }
    protocol::MutationOutcome::new(FormsDiff { title: Some(FormsOptionalText { value: payload.new_title.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
