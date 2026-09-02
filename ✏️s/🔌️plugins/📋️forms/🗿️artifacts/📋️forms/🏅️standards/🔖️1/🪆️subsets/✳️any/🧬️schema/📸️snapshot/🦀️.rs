//! 🧬️ Forms snapshot schema — artifact-lane fields only.

use crate::artifacts::forms::{forms_snapshot_with_state, FormsResultsChild, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted forms document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the inline
/// `steps: Vec<FormStep>` field is replaced by two fixed composed CHILD slots — this plugin no
/// longer defines its own bespoke document tree, it composes stdio's `value`/`table` subsets
/// instead. See `crate::artifacts::forms::🔖️Composition` (`🗿️artifacts/📋️forms/🦀️.rs`)
/// for the converters/working-scene this slot pair is built and read through. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: FormsResultsChild,
}

impl Default for FormsSnapshot {
    fn default() -> Self {
        forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, Vec::new())
    }
}
//#endregion 🔖️Snapshot

// 🧬️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): the native
// `store::ArtifactDsl`/`store::ArtifactPack` codec impls (and their hex/LEB128 primitives) moved to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}` — this facet root keeps only the struct + pure defaults, no
// codecs (design.md rule: `🧬️schema` is types + pure transforms only). Their round-trip tests moved
// with them.
