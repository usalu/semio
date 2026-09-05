//! 🧬️ VCS snapshot schema — artifact-lane fields only.

use crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted VCS demo document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    #[value(default)]
    pub tags: Vec<String>,
}

impl Default for VcsSnapshot {
    fn default() -> Self {
        Self { schema: VCS_DOCUMENT_SCHEMA.into(), title: "VCS Demo".into(), counter: 0, notes: String::new(), status: "new".into(), tags: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

// 🚪️ `store::ArtifactDsl`/`store::ArtifactPack for VcsSnapshot` — the real codec impls — moved to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}` (design.md §1 CORRECTION; `🧬️schema` keeps types + pure
// transforms only, no codecs, per design.md rule 3/`🔖️Exclusivity`).

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`VcsSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🌿️mutate-vcs-1`'s scenarios are measured through, and the same shape the committed
/// `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in.
///
/// A thin `dsl::json` wrapper (this facet's own first-party `DslValue` JSON codec, used behind
/// this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_vcs_snapshot_json(snapshot: &VcsSnapshot) -> String {
    dsl::json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_vcs_snapshot_json`] — decodes those committed specification vectors
/// into real [`VcsSnapshot`] values, so `🌿️mutate-vcs-1`'s adapter reads the committed fixture rather
/// than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that adapter is
/// impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_vcs_snapshot_json(text: &str) -> Result<VcsSnapshot, String> {
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.vcs.dsl.semio` text into a [`VcsSnapshot`] — a named, non-async pass-through of this
/// type's own `store::ArtifactDsl` impl (`../../🚪️io/📸️snapshot/📝️text/🦀️.rs`), whose trait
/// and error type are both unnameable outside this crate, so `🌿️mutate-vcs-1`'s `identity-round-trip`
/// scenario reaches the real committed artifact (`../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`)
/// through this instead.
pub fn parse_vcs_dsl(text: &str) -> Result<VcsSnapshot, String> {
    <VcsSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`VcsSnapshot`] back as `.vcs.dsl.semio` text — the inverse of [`parse_vcs_dsl`],
/// preamble included, which is what makes a printed document comparable to the committed one
/// byte for byte.
pub fn print_vcs_dsl(snapshot: &VcsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
