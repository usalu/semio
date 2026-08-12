//! 🔺️ `change-retrofit-knowledge-level` sparse diff construction — writes only `En1998Diff.retrofit_knowledge_level` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_knowledge_level::mutation::ChangeRetrofitKnowledgeLevel;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitKnowledgeLevel, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { retrofit_knowledge_level: Some(payload.new_retrofit_knowledge_level.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
