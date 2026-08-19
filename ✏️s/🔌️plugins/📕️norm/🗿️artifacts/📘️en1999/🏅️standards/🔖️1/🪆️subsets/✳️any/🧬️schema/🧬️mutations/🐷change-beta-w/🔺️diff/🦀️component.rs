//! 🔺️ `change-beta-w` sparse diff construction — writes only `En1999Diff.beta_w` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_beta_w::mutation::ChangeBetaW;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeBetaW, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_beta_w.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Correlation factor beta_w must be a finite number, got {}.", payload.new_beta_w), Vec::<String>::new());
    }
    if base.beta_w == payload.new_beta_w {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Correlation factor beta_w is already {}.", payload.new_beta_w));
    }
    protocol::MutationOutcome::new(En1999Diff { beta_w: Some(payload.new_beta_w.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
