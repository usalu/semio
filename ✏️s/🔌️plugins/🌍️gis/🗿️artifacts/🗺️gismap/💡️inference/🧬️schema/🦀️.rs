//! 🗺️ Owner-authored map proposal wire contracts, lifecycle and presentation vocabulary.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_os_kernel::os_directory::schema::{EditedArtifactFrontierV1, DocumentExecutionTargetLocaleV1, DOCUMENT_OPEN_MAX_SAFE_INTEGER};

//#region 💡️InferencePort
/// 🧯 Exact maximum accepted bytes for one inference request or approval body.
pub const GIS_MAP_INFERENCE_REQUEST_MAX_BYTES: usize = 1024;
/// 🧯 Exact maximum accepted bytes for one bounded owner-private response body.
pub const GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES: usize = 16 * 1024;
/// 📈 Highest progress cursor the hub's append-only bounded progress table admits.
pub const GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR: u64 = 16;
/// 📃 Highest number of lifecycle events one owner-private page may carry.
pub const GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS: usize = 8;
/// ⏳ Highest job lifetime the hub admits for one submitted job.
pub const GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS: u64 = 120_000;
/// 🔖 The one inference service the GIS Map port may name.
pub const GIS_MAP_INFERENCE_SERVICE_ID: &str = "s.gis.gismap.inference";

/// 📤 The closed client intent one submit carries — a service and a lifetime and nothing else.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceJobRequestV1 {
    pub schema: String,
    pub version: u32,
    pub request_id: String,
    pub service_id: String,
    pub policy_version: u32,
    pub lifetime_ms: u64,
}

/// ✅ The closed body one approval carries; the hash is echoed, never computed by a client.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceApprovalRequestV1 {
    pub schema: String,
    pub version: u32,
    pub job_id: String,
    pub proposal_hash: String,
}

/// 🖥 The hub's own job lifecycle vocabulary, mirrored exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferenceJobStateV1 {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// 🖥 The hub's own proposal lifecycle vocabulary, mirrored exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferenceProposalStateV1 {
    None,
    Offered,
    Approved,
    Stale,
    Cancelled,
}

fn valid_gis_map_inference_hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

/// 🧾 The closed receipt one accepted submit returns.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceJobReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    #[value(required)]
    pub proposal_hash: Option<String>,
    pub cursor: u64,
    pub expires_at_ms: u64,
}

impl GisMapInferenceJobReceiptV1 {
    /// 🛡️ Refuses malformed, substituted, or unbounded Hub receipt coordinates.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.inference-job-receipt/v1"
            && valid_gis_map_inference_hex(&self.job_id, 32)
            && self.proposal_hash.as_deref().is_none_or(|value| valid_gis_map_inference_hex(value, 64))
            && self.cursor <= GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR
            && (1..=DOCUMENT_OPEN_MAX_SAFE_INTEGER).contains(&self.expires_at_ms)
    }
}

/// 📈 One owner-private progress row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceProgressV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 🗓 One owner-private lifecycle event.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📃 The owner-private bounded page one events, cancel or poll read returns.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceEventPageV1 {
    pub schema: String,
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    pub cancel_requested: bool,
    pub stale: bool,
    #[value(required)]
    pub proposal_hash: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub events: Vec<GisMapInferenceEventV1>,
    pub progress: Vec<GisMapInferenceProgressV1>,
    pub next_cursor: u64,
}

impl GisMapInferenceEventPageV1 {
    /// 🛡️ Refuses substituted owner coordinates and non-monotone bounded pages.
    pub fn validate(&self, expected_job_id: &str) -> bool {
        if self.schema != "semio.hub.inference-job-events/v1"
            || self.job_id != expected_job_id
            || !valid_gis_map_inference_hex(&self.job_id, 32)
            || !self.proposal_hash.as_deref().is_none_or(|value| valid_gis_map_inference_hex(value, 64))
            || self.events.len() > GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS
            || self.progress.len() > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR as usize
            || self.next_cursor > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR
        {
            return false;
        }
        let mut ordinal = 0;
        if self.events.iter().any(|event| {
            let invalid = event.ordinal <= ordinal || event.ordinal > DOCUMENT_OPEN_MAX_SAFE_INTEGER || event.at_ms == 0 || event.at_ms > DOCUMENT_OPEN_MAX_SAFE_INTEGER;
            ordinal = event.ordinal;
            invalid
        }) {
            return false;
        }
        let mut cursor = 0;
        let mut completed = 0;
        if self.progress.iter().any(|progress| {
            let invalid = progress.cursor <= cursor
                || progress.cursor > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR
                || progress.run_epoch > DOCUMENT_OPEN_MAX_SAFE_INTEGER
                || progress.completed < completed
                || progress.completed > progress.total
                || progress.completed > DOCUMENT_OPEN_MAX_SAFE_INTEGER
                || progress.total == 0
                || progress.total > DOCUMENT_OPEN_MAX_SAFE_INTEGER
                || progress.at_ms == 0
                || progress.at_ms > DOCUMENT_OPEN_MAX_SAFE_INTEGER;
            cursor = progress.cursor;
            completed = progress.completed;
            invalid
        }) {
            return false;
        }
        self.preview.as_ref().is_none_or(|preview| {
            preview.validate()
                && preview.job_id == self.job_id
                && self.proposal_hash.as_deref() == Some(preview.proposal_hash.as_str())
                && self.state == GisMapInferenceJobStateV1::Succeeded
                && self.proposal_state == GisMapInferenceProposalStateV1::Offered
                && !self.cancel_requested
                && !self.stale
        })
    }
}

/// 🗺 The bounded Hub-validated geometry an owner may inspect before approving a proposal.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferencePreviewV1 {
    pub schema: String,
    pub job_id: String,
    pub proposal_hash: String,
    pub region_id: String,
    pub ring: [[f64; 2]; 5],
}

impl GisMapInferencePreviewV1 {
    /// 🛡️ Refuses any noncanonical rectangular preview or substituted owner.
    pub fn validate(&self) -> bool {
        if self.schema != "semio.hub.gis-map-inference-preview/v1"
            || !valid_gis_map_inference_hex(&self.job_id, 32)
            || !valid_gis_map_inference_hex(&self.proposal_hash, 64)
            || self.region_id != format!("inference-{}", self.job_id)
            || self.ring.iter().flatten().any(|coordinate| !coordinate.is_finite())
        {
            return false;
        }
        let [lon_min, lat_min] = self.ring[0];
        let [lon_max, lat_max] = self.ring[2];
        (-180.0..=180.0).contains(&lon_min)
            && (-180.0..=180.0).contains(&lon_max)
            && (-90.0..=90.0).contains(&lat_min)
            && (-90.0..=90.0).contains(&lat_max)
            && lon_min <= lon_max
            && lat_min <= lat_max
            && self.ring == [[lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min]]
    }
}

/// ✅ The closed approval outcome; `applied` is true only after a real committed-WAL witness.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceApprovalReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub applied: bool,
    pub undo: GisMapApprovalUndoHandleV1,
}

impl GisMapInferenceApprovalReceiptV1 {
    /// 🛡️ Refuses an uncommitted or substituted approval outcome.
    pub fn validate(&self, expected_job_id: &str, expected_proposal_hash: &str) -> bool {
        self.schema == "semio.hub.inference-approval-receipt/v1"
            && self.job_id == expected_job_id
            && self.proposal_hash == expected_proposal_hash
            && valid_gis_map_inference_hex(&self.job_id, 32)
            && valid_gis_map_inference_hex(&self.mutation_id, 32)
            && valid_gis_map_inference_hex(&self.command_hash, 64)
            && valid_gis_map_inference_hex(&self.proposal_hash, 64)
            && self.applied
            && valid_gis_map_inference_hex(&self.undo.target_id, 32)
            && self.undo.expected_current.validate()
    }
}

/// ↩️ Owner-bound durable undo locator minted only from a committed GIS approval witness.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapApprovalUndoHandleV1 {
    pub target_id: String,
    pub expected_current: EditedArtifactFrontierV1,
}

/// 📨️ Closed undo intent: a server-minted target, exact tail expectation and retry identity.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapApprovalUndoRequestV1 {
    pub schema: String,
    pub version: u32,
    pub target_id: String,
    pub idempotency_key: String,
    pub expected_current: EditedArtifactFrontierV1,
}

impl GisMapApprovalUndoRequestV1 {
    /// 🛡️ Refuses guessed targets, unbounded identities and noncanonical frontiers.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.gis-map-approval-undo/v1"
            && self.version == 1
            && self.target_id.len() == 32
            && self.target_id.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            && self.idempotency_key.len() == 32
            && self.idempotency_key.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            && self.expected_current.validate()
    }
}

/// 🧾️ Durable inverse receipt published only after the second verified WAL decision and pair.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapApprovalUndoReceiptV1 {
    pub schema: String,
    pub target_id: String,
    pub original_job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub applied: bool,
    pub replayed: bool,
    pub frontier: EditedArtifactFrontierV1,
}

/// 🔎 Owner-authored recovered page kept distinct from its live event envelope.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapReconciledPageV1 {
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    #[value(required)]
    pub proposal_hash: Option<String>,
    pub cancel_requested: bool,
    pub events: Vec<GisMapInferenceEventV1>,
    pub progress: Vec<GisMapInferenceProgressV1>,
    pub next_cursor: u64,
    pub expired: bool,
}

/// ↩️ Durable approval recovery status published by the owner's ledger.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapReconciledApprovalStateV1 { Available, UndoPrepared, Undone }

/// 🧾 Recovered approval status with its original committed receipt when available.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapReconciledApprovalV1 {
    pub state: GisMapReconciledApprovalStateV1,
    #[value(required)]
    pub receipt: Option<GisMapInferenceApprovalReceiptV1>,
}

/// 📦 One recovered job selected by the server from an exact retry identity.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapReconciledJobV1 {
    pub receipt: GisMapInferenceJobReceiptV1,
    pub page: GisMapReconciledPageV1,
    #[value(required)]
    pub approval: Option<GisMapReconciledApprovalV1>,
}

impl GisMapReconciledJobV1 {
    /// 🛡 Refuses substituted receipts, pages, previews and approval document frontiers.
    pub fn validate(&self, document_id: &str) -> bool {
        let page = GisMapInferenceEventPageV1 { schema: "semio.hub.inference-job-events/v1".into(), job_id: self.page.job_id.clone(), state: self.page.state, proposal_state: self.page.proposal_state, cancel_requested: self.page.cancel_requested, stale: self.page.proposal_state == GisMapInferenceProposalStateV1::Stale, proposal_hash: self.page.proposal_hash.clone(), preview: None, events: self.page.events.clone(), progress: self.page.progress.clone(), next_cursor: self.page.next_cursor };
        self.receipt.validate() && page.validate(&self.receipt.job_id)
            && self.receipt.state == page.state && self.receipt.proposal_state == page.proposal_state
            && self.receipt.proposal_hash == page.proposal_hash && self.receipt.cursor == page.next_cursor
            && ((page.proposal_state == GisMapInferenceProposalStateV1::Approved) == self.approval.is_some())
            && self.approval.as_ref().is_none_or(|approval| {
                self.receipt.proposal_hash.is_some() && match approval.state {
                    GisMapReconciledApprovalStateV1::Available => approval.receipt.as_ref().is_some_and(|receipt| receipt.validate(&self.receipt.job_id, self.receipt.proposal_hash.as_deref().unwrap_or("")) && receipt.undo.expected_current.document_id == document_id),
                    GisMapReconciledApprovalStateV1::UndoPrepared | GisMapReconciledApprovalStateV1::Undone => approval.receipt.is_none(),
                }
            })
    }
}

/// 🚦 The complete published failure vocabulary the four authenticated routes may answer with, plus
/// the two a client itself may reach: an indeterminate call and a port refused before any request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferencePortCodeV1 {
    #[value(rename = "inference.unavailable")]
    Unavailable,
    #[value(rename = "inference.denied")]
    Denied,
    #[value(rename = "inference.not-found")]
    NotFound,
    #[value(rename = "inference.invalid")]
    Invalid,
    #[value(rename = "inference.bounds")]
    Bounds,
    #[value(rename = "inference.conflict")]
    Conflict,
    #[value(rename = "inference.capacity")]
    Capacity,
    #[value(rename = "inference.expired")]
    Expired,
    #[value(rename = "inference.cancelled")]
    Cancelled,
    #[value(rename = "approval.commit-unavailable")]
    CommitUnavailable,
    #[value(rename = "inference.storage")]
    Storage,
    #[value(rename = "inference.transport")]
    Transport,
    #[value(rename = "inference.lease-unverified")]
    LeaseUnverified,
}

impl GisMapInferencePortCodeV1 {
    /// 🏷 The exact published wire code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "inference.unavailable",
            Self::Denied => "inference.denied",
            Self::NotFound => "inference.not-found",
            Self::Invalid => "inference.invalid",
            Self::Bounds => "inference.bounds",
            Self::Conflict => "inference.conflict",
            Self::Capacity => "inference.capacity",
            Self::Expired => "inference.expired",
            Self::Cancelled => "inference.cancelled",
            Self::CommitUnavailable => "approval.commit-unavailable",
            Self::Storage => "inference.storage",
            Self::Transport => "inference.transport",
            Self::LeaseUnverified => "inference.lease-unverified",
        }
    }

    /// 🚦 Maps one exact HTTP status onto the vocabulary; an unmapped status is indeterminate.
    pub const fn from_status(status: u16) -> Self {
        match status {
            400 => Self::Invalid,
            403 => Self::Denied,
            404 => Self::NotFound,
            409 => Self::Conflict,
            410 => Self::Expired,
            413 => Self::Bounds,
            429 => Self::Capacity,
            503 => Self::Unavailable,
            _ => Self::Transport,
        }
    }

    /// 🗣 Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Unavailable, DocumentExecutionTargetLocaleV1::En) => "Proposals are unavailable for this document.",
            (Self::Unavailable, DocumentExecutionTargetLocaleV1::De) => "Für dieses Dokument sind keine Vorschläge verfügbar.",
            (Self::Denied, DocumentExecutionTargetLocaleV1::En) => "You may not request proposals for this document.",
            (Self::Denied, DocumentExecutionTargetLocaleV1::De) => "Sie dürfen für dieses Dokument keine Vorschläge anfordern.",
            (Self::NotFound, DocumentExecutionTargetLocaleV1::En) => "This proposal no longer exists.",
            (Self::NotFound, DocumentExecutionTargetLocaleV1::De) => "Dieser Vorschlag existiert nicht mehr.",
            (Self::Invalid, DocumentExecutionTargetLocaleV1::En) => "The request was rejected as malformed.",
            (Self::Invalid, DocumentExecutionTargetLocaleV1::De) => "Die Anfrage wurde als fehlerhaft abgelehnt.",
            (Self::Bounds, DocumentExecutionTargetLocaleV1::En) => "The request exceeded its accepted size.",
            (Self::Bounds, DocumentExecutionTargetLocaleV1::De) => "Die Anfrage hat die zulässige Größe überschritten.",
            (Self::Conflict, DocumentExecutionTargetLocaleV1::En) => "The document changed; request a new proposal.",
            (Self::Conflict, DocumentExecutionTargetLocaleV1::De) => "Das Dokument hat sich geändert; fordern Sie einen neuen Vorschlag an.",
            (Self::Capacity, DocumentExecutionTargetLocaleV1::En) => "Too many proposals are running. Try again shortly.",
            (Self::Capacity, DocumentExecutionTargetLocaleV1::De) => "Es laufen zu viele Vorschläge. Versuchen Sie es in Kürze erneut.",
            (Self::Expired, DocumentExecutionTargetLocaleV1::En) => "This proposal expired before it was approved.",
            (Self::Expired, DocumentExecutionTargetLocaleV1::De) => "Dieser Vorschlag ist vor der Freigabe abgelaufen.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "The proposal was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlag wurde abgebrochen.",
            (Self::CommitUnavailable, DocumentExecutionTargetLocaleV1::En) => "The approved proposal could not be committed and was not applied.",
            (Self::CommitUnavailable, DocumentExecutionTargetLocaleV1::De) => "Der freigegebene Vorschlag konnte nicht übernommen werden und wurde nicht angewendet.",
            (Self::Storage, DocumentExecutionTargetLocaleV1::En) => "The proposal service is temporarily unavailable.",
            (Self::Storage, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlagsdienst ist vorübergehend nicht verfügbar.",
            (Self::Transport, DocumentExecutionTargetLocaleV1::En) => "The outcome is unknown. Close retries checking the original request without submitting another.",
            (Self::Transport, DocumentExecutionTargetLocaleV1::De) => "Das Ergebnis ist unbekannt. Schließen prüft die ursprüngliche Anfrage erneut, ohne eine weitere zu senden.",
            (Self::LeaseUnverified, DocumentExecutionTargetLocaleV1::En) => "This document has no verified execution target, so no proposal can start.",
            (Self::LeaseUnverified, DocumentExecutionTargetLocaleV1::De) => "Dieses Dokument hat kein verifiziertes Ausführungsziel, daher kann kein Vorschlag starten.",
        }
    }
}

/// 💡 The complete rendered lifecycle of one host-owned ephemeral inference port. `Idle` and
/// `Submitting` have no server counterpart, `Approving` is the server's `approval-prepared`, and
/// the four terminals are exactly `Applied | Cancelled | Stale | Failed`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferencePortPhaseV1 {
    Idle,
    Submitting,
    Running,
    Offered,
    Approving,
    Indeterminate,
    Applied,
    Cancelled,
    Stale,
    Failed,
}

impl GisMapInferencePortPhaseV1 {
    /// 🏁 A terminal phase accepts no further server answer — only an explicit clear.
    pub const fn terminal(self) -> bool {
        matches!(self, Self::Applied | Self::Cancelled | Self::Stale | Self::Failed)
    }

    /// 🔊 Work in flight announces politely; every terminal asserts.
    pub const fn aria_role(self) -> &'static str {
        if self.terminal() { "alert" } else { "status" }
    }

    /// 🗣 Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Idle, DocumentExecutionTargetLocaleV1::En) => "No proposal requested.",
            (Self::Idle, DocumentExecutionTargetLocaleV1::De) => "Kein Vorschlag angefordert.",
            (Self::Submitting, DocumentExecutionTargetLocaleV1::En) => "Requesting a bounds proposal…",
            (Self::Submitting, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag wird angefordert…",
            (Self::Running, DocumentExecutionTargetLocaleV1::En) => "Computing the bounds proposal…",
            (Self::Running, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag wird berechnet…",
            (Self::Offered, DocumentExecutionTargetLocaleV1::En) => "A bounds proposal is ready for review.",
            (Self::Offered, DocumentExecutionTargetLocaleV1::De) => "Ein Begrenzungsvorschlag liegt zur Prüfung bereit.",
            (Self::Approving, DocumentExecutionTargetLocaleV1::En) => "Waiting for the server to commit the approved proposal…",
            (Self::Approving, DocumentExecutionTargetLocaleV1::De) => "Warten auf die Freigabe des Vorschlags durch den Server…",
            (Self::Indeterminate, DocumentExecutionTargetLocaleV1::En) => "The outcome is unknown. The original request is retained while its server state is checked.",
            (Self::Indeterminate, DocumentExecutionTargetLocaleV1::De) => "Das Ergebnis ist unbekannt. Die ursprüngliche Anfrage bleibt erhalten, während ihr Serverstatus geprüft wird.",
            (Self::Applied, DocumentExecutionTargetLocaleV1::En) => "The approved proposal was committed to the document.",
            (Self::Applied, DocumentExecutionTargetLocaleV1::De) => "Der freigegebene Vorschlag wurde im Dokument übernommen.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "The proposal was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlag wurde abgebrochen.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::En) => "The document changed while the proposal ran. Request a new one.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::De) => "Das Dokument hat sich während des Vorschlags geändert. Fordern Sie einen neuen an.",
            (Self::Failed, DocumentExecutionTargetLocaleV1::En) => "The proposal did not complete.",
            (Self::Failed, DocumentExecutionTargetLocaleV1::De) => "Der Vorschlag wurde nicht abgeschlossen.",
        }
    }
}

/// 🎛 The complete localized control and region labels; EN and DE are both explicit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GisMapInferencePortControlV1 {
    Heading,
    Request,
    Cancel,
    Reject,
    Approve,
    Close,
    Progress,
    Overlay,
}

impl GisMapInferencePortControlV1 {
    /// 🗣 Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Heading, DocumentExecutionTargetLocaleV1::En) => "Bounds proposal",
            (Self::Heading, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag",
            (Self::Request, DocumentExecutionTargetLocaleV1::En) => "Request bounds proposal",
            (Self::Request, DocumentExecutionTargetLocaleV1::De) => "Begrenzungsvorschlag anfordern",
            (Self::Cancel, DocumentExecutionTargetLocaleV1::En) => "Cancel proposal",
            (Self::Cancel, DocumentExecutionTargetLocaleV1::De) => "Vorschlag abbrechen",
            (Self::Reject, DocumentExecutionTargetLocaleV1::En) => "Reject proposal",
            (Self::Reject, DocumentExecutionTargetLocaleV1::De) => "Vorschlag ablehnen",
            (Self::Approve, DocumentExecutionTargetLocaleV1::En) => "Approve proposal",
            (Self::Approve, DocumentExecutionTargetLocaleV1::De) => "Vorschlag freigeben",
            (Self::Close, DocumentExecutionTargetLocaleV1::En) => "Close proposal",
            (Self::Close, DocumentExecutionTargetLocaleV1::De) => "Vorschlag schließen",
            (Self::Progress, DocumentExecutionTargetLocaleV1::En) => "Proposal progress",
            (Self::Progress, DocumentExecutionTargetLocaleV1::De) => "Fortschritt des Vorschlags",
            (Self::Overlay, DocumentExecutionTargetLocaleV1::En) => "Proposed bounds on the map",
            (Self::Overlay, DocumentExecutionTargetLocaleV1::De) => "Vorgeschlagene Grenzen auf der Karte",
        }
    }
}

/// 💡 Complete renderer-visible state of one document's port — never a receipt, bearer, origin,
/// path, base pack, proposal body or user identity, and never anything persisted into a document.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferencePortStatusV1 {
    pub phase: GisMapInferencePortPhaseV1,
    pub job_id: Option<String>,
    pub cursor: u64,
    pub completed: u64,
    pub total: u64,
    pub proposal_hash: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub cancel_requested: bool,
    pub code: Option<GisMapInferencePortCodeV1>,
}

impl Default for GisMapInferencePortStatusV1 {
    /// 💤 The one starting value; a document with no port has exactly this.
    fn default() -> Self {
        Self { phase: GisMapInferencePortPhaseV1::Idle, job_id: None, cursor: 0, completed: 0, total: 0, proposal_hash: None, preview: None, cancel_requested: false, code: None }
    }
}

/// 🎬 Every input the port's state machine accepts.
#[derive(Clone, Debug, PartialEq)]
pub enum GisMapInferencePortEventV1 {
    Start,
    LeaseUnverified,
    Receipt(GisMapInferenceJobReceiptV1),
    Page(GisMapInferenceEventPageV1),
    Approve,
    Approval(GisMapInferenceApprovalReceiptV1),
    Cancel,
    Indeterminate(GisMapInferencePortCodeV1),
    Failed(GisMapInferencePortCodeV1),
    Clear,
}

/// 🗺 Projects one exact server page onto a rendered phase. Staleness outranks everything, then the
/// job's own terminal states, then the proposal's.
fn gis_map_inference_server_phase_v1(state: GisMapInferenceJobStateV1, proposal_state: GisMapInferenceProposalStateV1, stale: bool) -> GisMapInferencePortPhaseV1 {
    if stale || proposal_state == GisMapInferenceProposalStateV1::Stale {
        GisMapInferencePortPhaseV1::Stale
    } else if state == GisMapInferenceJobStateV1::Cancelled || proposal_state == GisMapInferenceProposalStateV1::Cancelled {
        GisMapInferencePortPhaseV1::Cancelled
    } else if state == GisMapInferenceJobStateV1::Failed {
        GisMapInferencePortPhaseV1::Failed
    } else if proposal_state == GisMapInferenceProposalStateV1::Approved {
        GisMapInferencePortPhaseV1::Applied
    } else if proposal_state == GisMapInferenceProposalStateV1::Offered || state == GisMapInferenceJobStateV1::Succeeded {
        GisMapInferencePortPhaseV1::Offered
    } else {
        GisMapInferencePortPhaseV1::Running
    }
}

/// 🧮 Total, pure transition — the Rust twin of `reduceGisMapInferencePortV1`. It never fabricates a
/// phase the server has not reported: `Submitting` is only left on an exact receipt, `Cancelled`
/// only on an exact server answer (a Cancel click is recorded as `cancel_requested`, never as an
/// optimistic terminal), `Approving` is only reachable from `Offered`, and an answer for a different
/// job id or after a terminal is ignored outright.
pub fn reduce_gis_map_inference_port_v1(current: &GisMapInferencePortStatusV1, event: &GisMapInferencePortEventV1) -> GisMapInferencePortStatusV1 {
    if matches!(event, GisMapInferencePortEventV1::Clear) {
        return GisMapInferencePortStatusV1::default();
    }
    if current.phase.terminal() {
        return current.clone();
    }
    let mut next = current.clone();
    match event {
        GisMapInferencePortEventV1::Clear => unreachable!(),
        GisMapInferencePortEventV1::Start => {
            if current.phase == GisMapInferencePortPhaseV1::Idle {
                next = GisMapInferencePortStatusV1 { phase: GisMapInferencePortPhaseV1::Submitting, ..GisMapInferencePortStatusV1::default() };
            }
        }
        GisMapInferencePortEventV1::LeaseUnverified => {
            if matches!(current.phase, GisMapInferencePortPhaseV1::Idle | GisMapInferencePortPhaseV1::Submitting) {
                next.phase = GisMapInferencePortPhaseV1::Failed;
                next.preview = None;
                next.code = Some(GisMapInferencePortCodeV1::LeaseUnverified);
            }
        }
        GisMapInferencePortEventV1::Receipt(receipt) => {
            if current.phase == GisMapInferencePortPhaseV1::Submitting || (current.phase == GisMapInferencePortPhaseV1::Indeterminate && current.job_id.is_none()) {
                next.phase = gis_map_inference_server_phase_v1(receipt.state, receipt.proposal_state, false);
                next.job_id = Some(receipt.job_id.clone());
                next.cursor = receipt.cursor;
                next.proposal_hash = receipt.proposal_hash.clone();
                next.preview = None;
                next.code = None;
            }
        }
        GisMapInferencePortEventV1::Page(page) => {
            if current.job_id.as_deref() == Some(page.job_id.as_str()) {
                let server = gis_map_inference_server_phase_v1(page.state, page.proposal_state, page.stale);
                let phase = if current.phase == GisMapInferencePortPhaseV1::Approving && !server.terminal() { GisMapInferencePortPhaseV1::Approving } else { server };
                if let Some(latest) = page.progress.last() {
                    next.completed = latest.completed;
                    next.total = latest.total;
                }
                next.phase = phase;
                next.cursor = current.cursor.max(page.next_cursor);
                next.proposal_hash = page.proposal_hash.clone();
                next.preview = if matches!(phase, GisMapInferencePortPhaseV1::Offered | GisMapInferencePortPhaseV1::Approving) { page.preview.clone() } else { None };
                next.cancel_requested = current.cancel_requested || page.cancel_requested;
                next.code = if phase == GisMapInferencePortPhaseV1::Failed { Some(GisMapInferencePortCodeV1::Storage) } else { None };
            }
        }
        GisMapInferencePortEventV1::Approve => {
            let preview_matches = current.preview.as_ref().zip(current.job_id.as_ref()).zip(current.proposal_hash.as_ref()).is_some_and(|((preview, job_id), proposal_hash)| preview.job_id == *job_id && preview.proposal_hash == *proposal_hash);
            if current.phase == GisMapInferencePortPhaseV1::Offered && preview_matches && !current.cancel_requested {
                next.phase = GisMapInferencePortPhaseV1::Approving;
            }
        }
        GisMapInferencePortEventV1::Approval(receipt) => {
            if matches!(current.phase, GisMapInferencePortPhaseV1::Approving | GisMapInferencePortPhaseV1::Indeterminate)
                && current.job_id.as_deref() == Some(receipt.job_id.as_str())
                && current.proposal_hash.as_deref() == Some(receipt.proposal_hash.as_str())
            {
                if receipt.applied {
                    next.phase = GisMapInferencePortPhaseV1::Applied;
                    next.code = None;
                } else {
                    next.phase = GisMapInferencePortPhaseV1::Failed;
                    next.code = Some(GisMapInferencePortCodeV1::CommitUnavailable);
                }
                next.preview = None;
            }
        }
        GisMapInferencePortEventV1::Cancel => {
            if current.phase != GisMapInferencePortPhaseV1::Idle {
                next.cancel_requested = true;
            }
        }
        GisMapInferencePortEventV1::Indeterminate(code) => {
            next.phase = GisMapInferencePortPhaseV1::Indeterminate;
            next.preview = None;
            next.code = Some(*code);
        }
        GisMapInferencePortEventV1::Failed(code) => {
            next.phase = if *code == GisMapInferencePortCodeV1::Cancelled { GisMapInferencePortPhaseV1::Cancelled } else { GisMapInferencePortPhaseV1::Failed };
            next.preview = None;
            next.code = Some(*code);
        }
    }
    next
}
//#endregion 💡️InferencePort

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
