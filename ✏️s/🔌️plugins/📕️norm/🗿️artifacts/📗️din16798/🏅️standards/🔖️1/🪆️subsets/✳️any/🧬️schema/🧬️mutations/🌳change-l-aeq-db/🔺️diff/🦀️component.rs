//! 🔺️ `change-l-aeq-db` sparse diff construction — writes only `Din16798Diff.l_aeq_db` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_l_aeq_db::mutation::ChangeLAeqDb;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLAeqDb, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_l_aeq_db.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Equivalent sound pressure level must be a finite number, got {}.", payload.new_l_aeq_db), Vec::<String>::new());
    }
    if base.l_aeq_db == payload.new_l_aeq_db {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Equivalent sound pressure level is already {}.", payload.new_l_aeq_db));
    }
    protocol::MutationOutcome::new(Din16798Diff { l_aeq_db: Some(payload.new_l_aeq_db.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
