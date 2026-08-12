//! 🔺️ `change-l-aeq-db` sparse diff construction — writes only `Din16798Diff.l_aeq_db` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_l_aeq_db::mutation::ChangeLAeqDb;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLAeqDb, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { l_aeq_db: Some(payload.new_l_aeq_db.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
