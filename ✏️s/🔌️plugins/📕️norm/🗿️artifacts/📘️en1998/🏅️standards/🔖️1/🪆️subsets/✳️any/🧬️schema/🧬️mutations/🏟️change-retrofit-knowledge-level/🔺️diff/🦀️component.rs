//! 🔺️ `change-retrofit-knowledge-level` sparse diff construction — writes only `En1998Diff.retrofit_knowledge_level` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_knowledge_level::mutation::ChangeRetrofitKnowledgeLevel;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeRetrofitKnowledgeLevel, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.retrofit_knowledge_level == payload.new_retrofit_knowledge_level {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Retrofit knowledge level is already \"{}\".", payload.new_retrofit_knowledge_level));
    }
    protocol::MutationOutcome::new(En1998Diff { retrofit_knowledge_level: Some(payload.new_retrofit_knowledge_level.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
