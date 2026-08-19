//! 🔺️ `change-en-a-gr` sparse diff construction — writes only `En1998Diff.en_a_gr` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_en_a_gr::mutation::ChangeEnAGr;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeEnAGr, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_en_a_gr.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Reference ground acceleration a_gr must be a finite number, got {}.", payload.new_en_a_gr), Vec::<String>::new());
    }
    if base.en_a_gr == payload.new_en_a_gr {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Reference ground acceleration a_gr is already {}.", payload.new_en_a_gr));
    }
    protocol::MutationOutcome::new(En1998Diff { en_a_gr: Some(payload.new_en_a_gr.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
