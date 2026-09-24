//! 📌️ Hub-materialized Check In: a client names a committed ledger head and a label; the hub builds
//! the checkpoint from its own document log and advances the document's active checkpoint.
//!
//! See `🌎️hub/🗿️artifact-authority/📌️check-in` for the hub job and `🔣️.json` for the schema.

use super::{valid_document_open_hash, valid_document_open_text, ArtifactFrontier, ArtifactHash, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧯️ Maximum canonical Check In request or status bytes.
pub const DOCUMENT_CHECK_IN_MAX_BYTES: usize = 4096;
/// 🏷️ Maximum Check In label characters.
pub const DOCUMENT_CHECK_IN_LABEL_MAX_CHARS: usize = 256;
/// 📛️ Request schema identifier.
pub const DOCUMENT_CHECK_IN_SCHEMA_V1: &str = "semio.hub.document-check-in/v1";
/// 📛️ Status schema identifier.
pub const DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1: &str = "semio.hub.document-check-in-status/v1";

fn request_id(value: &str) -> bool {
    value.len() == 32 && value.bytes().any(|byte| byte != b'0') && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn label(value: &str) -> bool {
    !value.is_empty() && value.chars().count() <= DOCUMENT_CHECK_IN_LABEL_MAX_CHARS && value.trim() == value && !value.chars().any(char::is_control)
}

/// 🌊️ One edited, committed ledger head with a canonical hexadecimal chain hash.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct EditedArtifactFrontierV1 {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_sha256: String,
}

impl EditedArtifactFrontierV1 {
    /// 🛡️ Checks the cross-runtime integer, text, and hash boundary.
    pub fn validate(&self) -> bool {
        valid_document_open_text(&self.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && (1..=DOCUMENT_OPEN_MAX_SAFE_INTEGER).contains(&self.head_edit_ordinal)
            && valid_document_open_text(&self.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && (1..=DOCUMENT_OPEN_MAX_SAFE_INTEGER).contains(&self.last_commit_seq)
            && valid_document_open_hash(&self.chain_sha256)
    }

    /// 🧬️ The directory authority frontier this grammar names.
    pub fn artifact_frontier(&self) -> Option<ArtifactFrontier> {
        if !self.validate() {
            return None;
        }
        Some(ArtifactFrontier {
            document_id: self.document_id.clone(),
            head_edit_ordinal: self.head_edit_ordinal,
            head_edit_id: self.head_edit_id.clone(),
            last_commit_seq: self.last_commit_seq,
            chain_hash: ArtifactHash::parse_hex(&self.chain_sha256)?,
        })
    }

    /// 🔁️ The grammar of an edited directory frontier; `None` for genesis.
    pub fn of_artifact_frontier(frontier: &ArtifactFrontier) -> Option<Self> {
        let wire = Self {
            document_id: frontier.document_id.clone(),
            head_edit_ordinal: frontier.head_edit_ordinal,
            head_edit_id: frontier.head_edit_id.clone(),
            last_commit_seq: frontier.last_commit_seq,
            chain_sha256: frontier.chain_hash.hex(),
        };
        wire.validate().then_some(wire)
    }
}

/// 📥️ A client names the head it checks in and a label; scope and author come from the route
/// and the session, and no pair, hash of a pair, or parent checkpoint is accepted from a client.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInV1 {
    pub schema: String,
    pub request_id: String,
    pub head: EditedArtifactFrontierV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl DocumentCheckInV1 {
    /// 🛡️ Validates the schema-owned bounds before any ledger or catalog access.
    pub fn validate(&self) -> bool {
        self.schema == DOCUMENT_CHECK_IN_SCHEMA_V1 && request_id(&self.request_id) && self.head.validate() && self.label.as_deref().is_none_or(label)
    }

    /// 📦️ Rejects duplicate fields, unknown authority inputs, padding, and noncanonical JSON.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > DOCUMENT_CHECK_IN_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }

    /// 📤️ Emits only a canonical bounded request.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = crate::os_pack::json::to_json_string(self);
        (source.len() <= DOCUMENT_CHECK_IN_MAX_BYTES).then_some(source)
    }
}

/// 🚦️ A Check In is ready only once its checkpoint is the document's active checkpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum DocumentCheckInPhaseV1 {
    Accepted,
    Materializing,
    Publishing,
    Ready,
    Failed,
    Cancelled,
}

impl DocumentCheckInPhaseV1 {
    /// 🏁️ Ready, Failed and Cancelled never change again.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Ready | Self::Failed | Self::Cancelled)
    }
}

/// ⛔️ Why a Check In ended without advancing the active checkpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum DocumentCheckInRefusalV1 {
    /// The named head is not the hub's current committed head.
    StaleHead,
    /// The named head precedes or equals the active checkpoint's baseline.
    BehindActiveCheckpoint,
    /// The active checkpoint changed while the checkpoint was materialized.
    ActiveCheckpointChanged,
    /// The ledger between the active checkpoint and the head holds an approval decision.
    LedgerNotReplayable,
    /// The trusted catalog's codec refused the replayed pair.
    CodecRefused,
    /// The member lost its write grant or the document its descriptor.
    AuthorityChanged,
    /// Storage or the trusted catalog was unavailable.
    Unavailable,
}

/// 📈️ Monotonic bounded materialization progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInProgressV1 {
    pub completed_units: u64,
    pub total_units: u64,
}

/// 🚩️ The checkpoint a ready Check In made active.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInReadyV1 {
    pub checkpoint_id: String,
    pub parent_checkpoint_id: String,
    pub baseline: EditedArtifactFrontierV1,
}

/// 📣️ One Check In's observable state; only Ready names a checkpoint and only Failed a refusal.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInStatusV1 {
    pub schema: String,
    pub request_id: String,
    pub phase: DocumentCheckInPhaseV1,
    pub progress: DocumentCheckInProgressV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<DocumentCheckInReadyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<DocumentCheckInRefusalV1>,
}

impl DocumentCheckInStatusV1 {
    /// 🔐️ Phase, checkpoint and refusal agree; progress never runs past its total.
    pub fn validate(&self) -> bool {
        self.schema == DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1
            && request_id(&self.request_id)
            && self.progress.completed_units <= self.progress.total_units
            && self.progress.total_units <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.label.as_deref().is_none_or(label)
            && match (&self.phase, &self.ready, &self.refusal) {
                (DocumentCheckInPhaseV1::Ready, Some(ready), None) => valid_document_open_hash(&ready.checkpoint_id) && valid_document_open_hash(&ready.parent_checkpoint_id) && ready.baseline.validate(),
                (DocumentCheckInPhaseV1::Failed, None, Some(_)) => true,
                (DocumentCheckInPhaseV1::Ready | DocumentCheckInPhaseV1::Failed, _, _) => false,
                (_, None, None) => true,
                _ => false,
            }
    }

    /// 📤️ Emits only a canonical bounded status.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = crate::os_pack::json::to_json_string(self);
        (source.len() <= DOCUMENT_CHECK_IN_MAX_BYTES).then_some(source)
    }

    /// 🧾️ Reads one exact status, withholding malformed or overposted results.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > DOCUMENT_CHECK_IN_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }
}
