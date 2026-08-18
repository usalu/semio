//! 🧬️ VCS snapshot schema — artifact-lane fields only.

use crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted VCS demo document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.vcs.vcs")]
#[dsl(extension = "vcs")]
#[dsl(layout = "lines")]
pub struct VcsSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub title: String,
    #[state(artifact)]
    pub counter: i64,
    #[state(artifact)]
    pub notes: String,
    #[state(artifact)]
    pub status: String,
    #[state(artifact)]
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Default for VcsSnapshot {
    fn default() -> Self {
        Self {
            schema: VCS_DOCUMENT_SCHEMA.into(),
            title: "VCS Demo".into(),
            counter: 0,
            notes: String::new(),
            status: "new".into(),
            tags: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

// 🚪️ `store::ArtifactDsl`/`store::ArtifactPack for VcsSnapshot` — the real codec impls — moved to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}` (design.md §1 CORRECTION; `🧬️schema` keeps types + pure
// transforms only, no codecs, per design.md rule 3/`🔖️Exclusivity`).
