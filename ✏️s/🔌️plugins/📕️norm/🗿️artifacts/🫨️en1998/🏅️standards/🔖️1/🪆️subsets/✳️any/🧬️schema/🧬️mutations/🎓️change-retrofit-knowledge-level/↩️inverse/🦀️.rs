//! ↩️ `change-retrofit-knowledge-level` inverse — restores the pre-change `retrofit_knowledge_level` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_retrofit_knowledge_level::ChangeRetrofitKnowledgeLevel;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRetrofitKnowledgeLevel, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeRetrofitKnowledgeLevel(ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: base.retrofit_knowledge_level.clone() })]
}
//#endregion 🔖️Inverse
