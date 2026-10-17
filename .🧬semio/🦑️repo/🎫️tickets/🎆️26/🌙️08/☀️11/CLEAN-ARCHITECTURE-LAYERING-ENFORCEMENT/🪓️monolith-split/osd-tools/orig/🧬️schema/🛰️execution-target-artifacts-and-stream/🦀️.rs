//! 🛰️ Execution target, artifacts and stream of the directory schema: the execution target lease fields and status codes, the descriptor digest validation, the artifact frontier, checkpoint and retention records, the invite and document views and the directory stream message. See [`super`].

use super::*;

//#region 🔖️Views
//#region 🪪️ExecutionTargetLease
/// 🧯️ Exact maximum accepted bytes for one verified execution-target component: the shipped `release` cap, because only
/// shipped bytes execute and every shipped component's receipt is held to it.
pub const DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES: u64 = semio_framework_plugin_component_receipt::SHIPPED_COMPONENT_MAX_BYTES;

/// 🧯️ Exact maximum accepted bytes for one verified raw package descriptor.
pub const DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES: u64 = 4 * 1024 * 1024;

/// 🧱️ Exact byte identity of one verified component, bound to the package projection.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionTargetComponentV1 {
    pub sha256: String,
    pub blake3: String,
    pub byte_length: u64,
}

/// 📜️ Exact byte identity of one verified raw package descriptor.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionTargetDescriptorV1 {
    pub sha256: String,
    pub byte_length: u64,
}

/// 🪪️ Receipt-free public fields of one document execution-target lease. It never carries a plan
/// receipt, socket grant, session token, hub origin, local path or module URL.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionTargetLeaseFieldsV1 {
    pub schema: String,
    pub version: u32,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub component: DocumentExecutionTargetComponentV1,
    pub descriptor: DocumentExecutionTargetDescriptorV1,
    pub browser_actor: DocumentExecutionTargetBrowserActorV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: DocumentOpenCheckpointV1,
    pub revalidation: DocumentOpenRevalidationV1,
}

impl DocumentExecutionTargetLeaseFieldsV1 {
    /// ✅ Validates every identity, byte and grant invariant of one receipt-free lease projection.
    ///
    /// 🪢 `artifact.kind` (manifest taxonomy space) and `parent_dialect.artifact_kind` (plugin
    /// dialect space) are bounded separately for the reason `DocumentOpenPlanV1::validate` records;
    /// the lease is projected from a plan that already carries both from one catalog selection.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        self.browser_actor
            .validate(DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str())
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
        let ids = [
            self.scope.space_id.as_str(),
            self.scope.document_id.as_str(),
            self.package.plugin_id.as_str(),
            self.package.package_id.as_str(),
            self.package.version.as_str(),
            self.artifact.kind.as_str(),
            self.artifact.schema.as_str(),
            self.surface.surface_id.as_str(),
            self.surface.app_id.as_str(),
            self.surface.window_kind_id.as_str(),
        ];
        if self.schema != "semio.os.document-execution-target-lease/v1"
            || self.version != 1
            || ids.iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES))
            || [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset].into_iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES) || value.trim() != value.as_str())
            || !valid_document_open_hash(&self.descriptor_digest_v1)
            || !valid_document_open_hash(&self.catalog.generation_id)
            || !valid_document_open_hash(&self.package.component_sha256)
            || !valid_document_open_hash(&self.package.component_blake3)
            || !valid_document_open_hash(&self.package.descriptor_byte_sha256)
            || !self.package.receipt_binds()
            || self.package.execution_protocol.app_channel_version != semio_framework_os_channel::CHANNEL_VERSION
            || !valid_document_open_hash(&self.artifact.pack_schema_hash)
            || !valid_document_open_hash(&self.component.sha256)
            || !valid_document_open_hash(&self.component.blake3)
            || !valid_document_open_hash(&self.descriptor.sha256)
            || self.component.sha256 != self.package.component_sha256
            || self.component.blake3 != self.package.component_blake3
            || self.descriptor.sha256 != self.package.descriptor_byte_sha256
            || self.component.byte_length == 0
            || self.component.byte_length > DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES
            || self.component.byte_length != self.package.component_receipt.wasm_bytes
            || self.descriptor.byte_length == 0
            || self.descriptor.byte_length > DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES
            || !self.grant.read
            || !self.grant.observe
            || self.grant.write != matches!(self.surface.role, DocumentOpenSurfaceRoleV1::Editor)
            || self.revalidation.directory_revision == 0
            || self.revalidation.directory_revision > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.revalidation.membership_generation == 0
            || self.revalidation.membership_generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || (self.revalidation.session_generation.is_some() == self.revalidation.share_generation.is_some())
            || self.revalidation.session_generation == Some(0)
            || self.revalidation.session_generation.is_some_and(|generation| generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
            || self.revalidation.share_generation == Some(0)
            || self.revalidation.share_generation.is_some_and(|generation| generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let checkpoint = &self.checkpoint;
        if !valid_document_open_hash(&checkpoint.checkpoint_id)
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
            || !valid_document_open_hash(&checkpoint.aggregate_sha256)
            || checkpoint.baseline_frontier.document_id != self.scope.document_id
            || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
            || !(checkpoint.baseline_frontier.is_genesis_for(&self.scope) || checkpoint.baseline_frontier.is_edited_for(&self.scope))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(())
    }
}

/// 🧾 Projects one plan into receipt-free lease fields. The plan constrains every identity but no
/// byte length, so both lengths come from the installation under comparison and are independently
/// enforced against the exact verified bytes before a lease exists.
pub fn lease_fields_from_plan_v1(plan: &DocumentOpenPlanV1, component_byte_length: u64, descriptor_byte_length: u64, browser_actor_byte_length: Option<u64>) -> Result<DocumentExecutionTargetLeaseFieldsV1, DocumentOpenPlanErrorCodeV1> {
    let fields = DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".to_string(),
        version: 1,
        scope: plan.scope.clone(),
        descriptor_digest_v1: plan.descriptor_digest_v1.clone(),
        catalog: plan.catalog.clone(),
        package: plan.package.clone(),
        component: DocumentExecutionTargetComponentV1 { sha256: plan.package.component_sha256.clone(), blake3: plan.package.component_blake3.clone(), byte_length: component_byte_length },
        descriptor: DocumentExecutionTargetDescriptorV1 { sha256: plan.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
        browser_actor: plan
            .browser_actor
            .to_lease(DocumentBrowserActorSourceV1 { component_sha256: &plan.package.component_sha256, descriptor_byte_sha256: &plan.package.descriptor_byte_sha256 }, plan.surface.renderer_target.as_str(), browser_actor_byte_length)
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?,
        artifact: plan.artifact.clone(),
        parent_dialect: plan.parent_dialect.clone(),
        surface: plan.surface.clone(),
        grant: plan.grant,
        checkpoint: plan.checkpoint.clone(),
        revalidation: plan.revalidation,
    };
    fields.validate()?;
    Ok(fields)
}

/// ⚖️ The one shared full-field lease relation. No transport is permitted a subset comparison.
pub fn same_lease_fields_v1(left: &DocumentExecutionTargetLeaseFieldsV1, right: &DocumentExecutionTargetLeaseFieldsV1) -> bool {
    left == right
}

/// 🌐 Complete localized execution-target status vocabulary, free of origin, path, receipt, grant,
/// digest and user identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum DocumentExecutionTargetStatusCodeV1 {
    Verifying,
    Retrying,
    CatchingUp,
    IntegrityFailed,
    Stale,
    Cancelled,
    RendererUnavailable,
    LinkExpired,
    AccessRevoked,
}

impl DocumentExecutionTargetStatusCodeV1 {
    /// 🗣️ Explicit English and German text; there is no default language.
    pub const fn text(self, locale: DocumentExecutionTargetLocaleV1) -> &'static str {
        match (self, locale) {
            (Self::Verifying, DocumentExecutionTargetLocaleV1::En) => "Verifying document component…",
            (Self::Verifying, DocumentExecutionTargetLocaleV1::De) => "Dokumentkomponente wird überprüft…",
            (Self::Retrying, DocumentExecutionTargetLocaleV1::En) => "The hub is busy. Asking again for the document component…",
            (Self::Retrying, DocumentExecutionTargetLocaleV1::De) => "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…",
            (Self::CatchingUp, DocumentExecutionTargetLocaleV1::En) => "Catching up with the hub…",
            (Self::CatchingUp, DocumentExecutionTargetLocaleV1::De) => "Gleiche mit dem Hub ab…",
            (Self::IntegrityFailed, DocumentExecutionTargetLocaleV1::En) => "The document component could not be verified. Reopen the document.",
            (Self::IntegrityFailed, DocumentExecutionTargetLocaleV1::De) => "Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::En) => "The document target changed. Reopen the document.",
            (Self::Stale, DocumentExecutionTargetLocaleV1::De) => "Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::En) => "Opening the document was cancelled.",
            (Self::Cancelled, DocumentExecutionTargetLocaleV1::De) => "Das Öffnen des Dokuments wurde abgebrochen.",
            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::En) => "The verified document component is ready, but this renderer is unavailable.",
            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::De) => "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar.",
            (Self::LinkExpired, DocumentExecutionTargetLocaleV1::En) => "The connection was lost for too long. Reconnect to keep editing this document.",
            (Self::LinkExpired, DocumentExecutionTargetLocaleV1::De) => "Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten.",
            (Self::AccessRevoked, DocumentExecutionTargetLocaleV1::En) => "Your access to this document was removed.",
            (Self::AccessRevoked, DocumentExecutionTargetLocaleV1::De) => "Ihr Zugriff auf dieses Dokument wurde entfernt.",
        }
    }

    /// 🔊 Progress announces; every terminal outcome asserts.
    pub const fn aria_role(self) -> &'static str {
        match self {
            Self::Verifying | Self::Retrying | Self::CatchingUp => "status",
            _ => "alert",
        }
    }
}

/// 🌍 Explicit UI language for one execution-target status; callers must choose one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentExecutionTargetLocaleV1 {
    En,
    De,
}
//#endregion 🪪️ExecutionTargetLease

/// 🚨️ Descriptor values that cannot participate in canonical authority hashing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptorDigestError {
    EmptyField(&'static str),
    InvalidHash(&'static str),
    InvalidFrontier,
    InvalidBootstrapVersion,
    LengthOverflow(&'static str),
}

impl std::fmt::Display for DescriptorDigestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "descriptor field `{field}` is empty"),
            Self::InvalidHash(field) => write!(formatter, "descriptor field `{field}` is not a nonzero lowercase SHA-256"),
            Self::InvalidFrontier => formatter.write_str("descriptor bootstrap commit exceeds head"),
            Self::InvalidBootstrapVersion => formatter.write_str("descriptor bootstrap version must be positive"),
            Self::LengthOverflow(field) => write!(formatter, "descriptor field `{field}` exceeds the u64 byte-length encoding"),
        }
    }
}

impl std::error::Error for DescriptorDigestError {}

/// 🪪️ Validates immutable descriptor authority metadata without encoding or hashing.
pub fn validate_document_descriptor_v1(descriptor: &DocumentDescriptor) -> Result<(), DescriptorDigestError> {
    if descriptor.bootstrap_version == 0 {
        return Err(DescriptorDigestError::InvalidBootstrapVersion);
    }
    if descriptor.bootstrap_frontier.commit_seq > descriptor.bootstrap_frontier.head_seq {
        return Err(DescriptorDigestError::InvalidFrontier);
    }
    for (field, value) in [
        ("space_id", descriptor.space_id.as_str()),
        ("document_id", descriptor.document_id.as_str()),
        ("artifact_kind", descriptor.artifact_kind.as_str()),
        ("artifact_schema", descriptor.artifact_schema.as_str()),
        ("owner.plugin_id", descriptor.owner.plugin_id.as_str()),
        ("owner.package_id", descriptor.owner.package_id.as_str()),
        ("owner.version", descriptor.owner.version.as_str()),
    ] {
        if value.is_empty() { return Err(DescriptorDigestError::EmptyField(field)); }
    }
    for (field, value) in [
        ("owner.package_hash", descriptor.owner.package_hash.as_str()),
        ("pack_schema_hash", descriptor.pack_schema_hash.as_str()),
        ("bootstrap_snapshot_hash", descriptor.bootstrap_snapshot_hash.as_str()),
    ] {
        if value.len() != 64 || !value.chars().all(|digit| matches!(digit, '0'..='9' | 'a'..='f')) || value.chars().all(|digit| digit == '0') {
            return Err(DescriptorDigestError::InvalidHash(field));
        }
    }
    Ok(())
}

/// 🏔️ Exact public checkpoint frontier, structurally identical to the replication wire frontier.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactFrontier {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: ArtifactHash,
}

impl ArtifactFrontier {
    /// 🌱️ Empty history is exact, scope-bound, and never represented by an invented edit.
    pub fn is_genesis_for(&self, scope: &DocumentScope) -> bool {
        valid_document_open_text(&scope.space_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && valid_document_open_text(&scope.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.document_id == scope.document_id
            && self.head_edit_ordinal == 0
            && self.head_edit_id.is_empty()
            && self.last_commit_seq == 0
            && self.chain_hash.0 == [0; 32]
    }

    /// 🌿️ An edited frontier has positive counters and a real authenticated history head.
    pub fn is_edited_for(&self, scope: &DocumentScope) -> bool {
        valid_document_open_text(&scope.space_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.document_id == scope.document_id
            && valid_document_open_text(&self.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && valid_document_open_text(&self.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            && self.head_edit_ordinal > 0
            && self.head_edit_ordinal <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.last_commit_seq > 0
            && self.last_commit_seq <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.chain_hash.0 != [0; 32]
    }
}

/// 🫧️ Integrity and private storage identity for one staged immutable artifact blob.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactBlobRef {
    pub sha256: ArtifactHash,
    pub byte_length: u64,
    pub storage_key: String,
}

/// 🪞️ Public integrity metadata for one staged blob; private storage keys never enter events.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PublishedArtifactBlob {
    pub sha256: ArtifactHash,
    pub byte_length: u64,
}

/// 📡️ Storage-key-free checkpoint metadata published through the append-only directory log.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PublishedArtifactCheckpoint {
    pub scope: DocumentScope,
    pub checkpoint_id: CheckpointId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<CheckpointId>,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: PublishedArtifactBlob,
    pub spr: PublishedArtifactBlob,
    pub aggregate_sha256: ArtifactHash,
    pub published_at_ms: u64,
}

/// 📍️ One server-derived checkpoint including backend-private immutable blob locators.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactCheckpoint {
    pub scope: DocumentScope,
    pub checkpoint_id: CheckpointId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<CheckpointId>,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: ArtifactBlobRef,
    pub spr: ArtifactBlobRef,
    pub aggregate_sha256: ArtifactHash,
    pub published_at_ms: u64,
}

/// 🧹️ Public retention selection vocabulary; advancement is P2-B and pruning remains P2-D.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactRetention {
    pub scope: DocumentScope,
    pub retained_checkpoint_id: CheckpointId,
    pub retained_floor: ArtifactFrontier,
    pub checkpoint_lineage_head: CheckpointId,
}

/// 🧾️ One document inside a space's durable artifact index plus live sync bookkeeping.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentView {
    pub descriptor: DocumentDescriptor,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
}

/// 🔗️ One outstanding (or revoked) space invite. Not event-sourced itself (secret token lives
/// outside the log) — only its `invite.redeemed` outcome is a `DirectoryEvent`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InviteView {
    pub id: String,
    pub space_id: String,
    pub role: DirectorySpaceRole,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
}
//#endregion 🔖️Views

//#region 🔖️Stream
/// 🔌️ `connection` stream message phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectoryConnectionPhase {
    Opened,
    Closed,
}

/// 👥️ One live presence actor in a document's roster (Amendment 3 to C1) — the hub knows all four
/// fields without ever decoding the actor's opaque `PresencePeer` bytes: `surface`/`color` are
/// stamped at hub-handshake time (`?surface=`, `HubState.session_colors`), `user_id` from auth.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DirectoryPresenceActor {
    pub actor: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub surface: String,
    pub color: u8,
}

/// 🛟️ Public checkpoint identity that makes a lagged client discard its discontinuous live state.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RebootstrapRequired {
    pub scope: DocumentScope,
    pub checkpoint_id: CheckpointId,
    pub descriptor_digest_v1: ArtifactHash,
    pub baseline_frontier: ArtifactFrontier,
}

/// 🔑️ Which way one reader's own access to a space moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DirectoryAccessChange {
    Granted,
    Revoked,
}

/// 📡️ One `/directory/socket/v1` text frame (contract C1/C2) — subscribe, then gap-free replay.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "lowercase", rename_all_fields = "camelCase")]
pub enum DirectoryStreamMessage {
    Event {
        event: Box<DirectoryEvent>,
    },
    Connection {
        phase: DirectoryConnectionPhase,
        connection: ConnectionView,
    },
    /// 👥️ Amendment 3 to C1: the document-wide roster, published on every roster change.
    Presence {
        space_id: String,
        document_id: String,
        actors: Vec<DirectoryPresenceActor>,
    },
    Heartbeat {
        head_seq: u64,
    },
    #[value(rename = "rebootstrap-required")]
    RebootstrapRequired {
        control: RebootstrapRequired,
    },
    /// 🔑️ The reader's OWN access to one space moved (a membership granted, an invite redeemed, a membership revoked):
    /// the directory events it may read changed retroactively — a space's earlier events became visible or invisible —
    /// so its projection re-reads the directory from the origin. Derived per reader by the hub's directory socket from
    /// the committed membership event; never published on the shared bus.
    #[value(rename = "access-changed")]
    AccessChanged {
        space_id: String,
        change: DirectoryAccessChange,
    },
}
//#endregion 🔖️Stream
