//! 🎞️ Protocol semio_hub wire frames: the lane-tagged `ClientFrame`/`ServerFrame` envelopes a
//! browser/native sync client exchanges with the collaboration semio_hub, plus their binary codec. Frozen
//! contract: `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment`
//! §`protocol_wire`.
//!
//! 🎯️ W5: the byte encoding is now a fully hand-rolled binary layout — `lane: u8` followed by
//! `frame tag: u8` (the frame enum's variant declaration order) and its fields in declaration
//! order, with no body-length prefix (one frame per WS message) and no per-field tags. This
//! matches `crate::wire::🔖️WireCodec`'s convention (also used by `crate::causal::🔖️EnvelopeCodec`
//! and `os_dsl::op_rt`). `ArtifactDiff`/`InverseMutation` payloads are opaque `Vec<u8>` (never
//! `serde_json::Value`). `ClientFrame::Presence`/`ServerFrame::Presence` carry opaque presence
//! payload bytes (`peer: Vec<u8>` / `peers: Vec<Vec<u8>>`) — this crate has no dependency on
//! `framework_core` (where the concrete `PresencePeer` type and its binary codec live), so the
//! frame only ever moves the already-encoded blob a caller supplies. `protocol_core` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_hash32`/`write_bool` and
//! their `read_*` twins); this crate adds only the option/vec combinators and the frame/nested-enum
//! tag dispatch below.

use semio_framework_replication_wire_core::malformed;
use semio_framework_replication_wire_core::read_opt_bytes;
use semio_framework_replication_wire_core::read_opt_str;
use semio_framework_replication_wire_core::read_vec_bytes;
use semio_framework_replication_wire_core::write_opt_bytes;
use semio_framework_replication_wire_core::write_opt_str;
use semio_framework_replication_wire_core::write_vec_bytes;

//#region 🔖️Lane
/// 🛣️ Which logical channel a wire frame travels on: `Command` for causally-ordered,
/// durable operation batches; `Preview` for ephemeral, best-effort UI-state broadcast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Command = 0,
    Preview = 1,
}

impl Lane {
    async fn to_byte(self) -> u8 {
        self as u8
    }

    async fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Lane::Command),
            1 => Some(Lane::Preview),
            _ => None,
        }
    }
}
//#endregion 🔖️Lane

//#region 🔖️ClientFrame
/// 📨️ One frame a client sends to the semio_hub.
#[derive(Clone, Debug, PartialEq)]
pub enum ClientFrame {
    SocketHelloV1 { wire_version: u32, protocol_version: u32, schema: String, pack_schema_hash: [u8; 32], resume_token: Option<String>, frontier: Option<semio_framework_replication_causal::FrontierSummary> },
    Commands { batch_id: u64, envelopes: Vec<semio_framework_replication_causal::MutationEnvelope> },
    FrontierAdvertise { frontier: semio_framework_replication_causal::FrontierSummary },
    PreviewPublish { key: String, seq: u64, payload: Vec<u8> },
    Presence { peer: Vec<u8> },
    CreditGrant { n: u32 },
    Bye,
}
//#endregion 🔖️ClientFrame

//#region 🔖️ServerFrame
/// 🚀️ How a `ServerFrame::Welcome` seeds a freshly (re)connected client's local state.
#[derive(Clone, Debug, PartialEq)]
pub enum Bootstrap {
    None,
    Snapshot { pack_hash: [u8; 32], inline: Option<Vec<u8>> },
    Tail,
    ArtifactBootstrap(Box<ArtifactBootstrap>),
}

pub const ARTIFACT_BOOTSTRAP_FORMAT_VERSION: u32 = 1;
pub const ARTIFACT_BOOTSTRAP_CHUNK_BYTES: usize = 4 * 1024;
pub const ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES: u64 = 64 * 1024 * 1024;
pub const ARTIFACT_BOOTSTRAP_MAX_CHUNKS: u32 = 16 * 1024;
pub const REBOOTSTRAP_SCOPE_MAX_BYTES: usize = 256;

/// 🧩️ The indivisible canonical artifact payload: pack state plus SPR history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactBootstrapPair {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 📦️ Descriptor-bound metadata and optional inline pair for one public artifact bootstrap.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactBootstrap {
    pub format_version: u32,
    pub descriptor_hash: [u8; 32],
    pub artifact_schema: String,
    pub artifact_kind: String,
    pub pack_schema_hash: [u8; 32],
    pub baseline_frontier: semio_framework_replication_causal::FrontierSummary,
    pub pack_hash: [u8; 32],
    pub spr_hash: [u8; 32],
    pub pack_length: u64,
    pub spr_length: u64,
    pub chunk_count: u32,
    pub aggregate_hash: [u8; 32],
    pub required_tail_frontier: semio_framework_replication_causal::FrontierSummary,
    pub inline: Option<ArtifactBootstrapPair>,
}

/// 🛟️ Storage-key-free checkpoint identity sent before a lagged live stream closes.
#[derive(Clone, Debug, PartialEq)]
pub struct RebootstrapRequired {
    pub space_id: String,
    pub document_id: String,
    pub checkpoint_id: [u8; 32],
    pub descriptor_hash: [u8; 32],
    pub baseline_frontier: semio_framework_replication_causal::FrontierSummary,
}

/// 🌱️ Whether `baseline` is a document's genesis — no edit and no commit yet — the one baseline that names no head edit: a
/// document created and rebuilt before its first check-in has only its creation checkpoint.
fn rebootstrap_baseline_is_genesis(baseline: &semio_framework_replication_causal::FrontierSummary) -> bool {
    baseline.head_edit_ordinal == 0 && baseline.last_commit_seq == 0
}

fn validate_rebootstrap_required(control: &RebootstrapRequired) -> Result<(), semio_framework_replication_error::ProtocolError> {
    if control.space_id.is_empty()
        || control.document_id.is_empty()
        || control.space_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        || control.document_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        || !artifact_bootstrap_nonzero(&control.checkpoint_id)
        || !artifact_bootstrap_nonzero(&control.descriptor_hash)
        || control.baseline_frontier.document_id.0 != control.document_id
        || (control.baseline_frontier.head_edit_id.is_empty() && !rebootstrap_baseline_is_genesis(&control.baseline_frontier))
    {
        return Err(artifact_bootstrap_error("rebootstrap control identity is invalid"));
    }
    Ok(())
}

/// 🛡️ Caller-selected assembly ceilings, always constrained by the wire chunk maximum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactBootstrapLimits {
    pub max_total_bytes: u64,
    pub max_chunks: u32,
    pub max_chunk_bytes: usize,
}

impl Default for ArtifactBootstrapLimits {
    fn default() -> Self {
        Self { max_total_bytes: ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, max_chunks: ARTIFACT_BOOTSTRAP_MAX_CHUNKS, max_chunk_bytes: ARTIFACT_BOOTSTRAP_CHUNK_BYTES }
    }
}

/// 📈️ Observable transfer progress; byte and chunk counts never decrease within one assembler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactBootstrapProgress {
    pub received_bytes: u64,
    pub total_bytes: u64,
    pub received_chunks: u32,
    pub total_chunks: u32,
}

/// ⏱️ Host-provided cancellation, monotonic clock, and progress boundary.
pub trait ArtifactBootstrapControl {
    fn is_cancelled(&mut self) -> bool;
    fn now_ms(&mut self) -> u64;
    fn on_progress(&mut self, progress: ArtifactBootstrapProgress);
}

#[derive(Debug)]
enum ArtifactBootstrapStorage {
    Inline(ArtifactBootstrapPair),
    Chunked(Vec<u8>),
}

/// 🧱️ Bounded, cancellable staging owner that yields a pair only after complete integrity validation.
#[derive(Debug)]
pub struct ArtifactBootstrapAssembler {
    bootstrap: ArtifactBootstrap,
    limits: ArtifactBootstrapLimits,
    deadline_ms: Option<u64>,
    storage: Option<ArtifactBootstrapStorage>,
    received_bytes: u64,
    next_index: u32,
    inline: bool,
}

fn artifact_bootstrap_error(detail: impl Into<String>) -> semio_framework_replication_error::ProtocolError {
    semio_framework_replication_error::ProtocolError::Malformed { what: "artifact bootstrap", offset: 0, detail: detail.into() }
}

fn artifact_bootstrap_nonzero(hash: &[u8; 32]) -> bool {
    hash.iter().any(|byte| *byte != 0)
}

fn artifact_bootstrap_total(bootstrap: &ArtifactBootstrap) -> Result<u64, semio_framework_replication_error::ProtocolError> {
    if bootstrap.pack_length == 0 || bootstrap.spr_length == 0 {
        return Err(artifact_bootstrap_error("pack and SPR bytes must both be present"));
    }
    bootstrap.pack_length.checked_add(bootstrap.spr_length).ok_or_else(|| artifact_bootstrap_error("total bytes overflow"))
}

fn validate_artifact_bootstrap_header(bootstrap: &ArtifactBootstrap, inline: bool, limits: ArtifactBootstrapLimits) -> Result<u64, semio_framework_replication_error::ProtocolError> {
    if limits.max_total_bytes == 0 || limits.max_chunks == 0 || limits.max_chunk_bytes == 0 || limits.max_chunk_bytes > ARTIFACT_BOOTSTRAP_CHUNK_BYTES {
        return Err(artifact_bootstrap_error("assembler limits are invalid"));
    }
    if bootstrap.format_version != ARTIFACT_BOOTSTRAP_FORMAT_VERSION {
        return Err(artifact_bootstrap_error(format!("version {} is unsupported", bootstrap.format_version)));
    }
    if !artifact_bootstrap_nonzero(&bootstrap.descriptor_hash) || !artifact_bootstrap_nonzero(&bootstrap.pack_schema_hash) || !artifact_bootstrap_nonzero(&bootstrap.pack_hash) || !artifact_bootstrap_nonzero(&bootstrap.spr_hash) || !artifact_bootstrap_nonzero(&bootstrap.aggregate_hash) {
        return Err(artifact_bootstrap_error("required hash must be nonzero"));
    }
    if bootstrap.artifact_schema.is_empty() || bootstrap.artifact_schema.len() > 256 || bootstrap.artifact_kind.is_empty() || bootstrap.artifact_kind.len() > 256 {
        return Err(artifact_bootstrap_error("artifact schema or kind length is invalid"));
    }
    if bootstrap.baseline_frontier.document_id.0.is_empty() || bootstrap.baseline_frontier.document_id != bootstrap.required_tail_frontier.document_id {
        return Err(artifact_bootstrap_error("frontier document mismatch"));
    }
    if bootstrap.required_tail_frontier.head_edit_ordinal < bootstrap.baseline_frontier.head_edit_ordinal || bootstrap.required_tail_frontier.last_commit_seq < bootstrap.baseline_frontier.last_commit_seq {
        return Err(artifact_bootstrap_error("required tail frontier precedes baseline"));
    }
    let total = artifact_bootstrap_total(bootstrap)?;
    if total > limits.max_total_bytes || usize::try_from(total).is_err() {
        return Err(artifact_bootstrap_error("total bytes exceed assembler budget"));
    }
    if inline {
        if bootstrap.chunk_count != 0 {
            return Err(artifact_bootstrap_error("inline pair must declare zero chunks"));
        }
    } else {
        let chunk_count = u64::from(bootstrap.chunk_count);
        if bootstrap.chunk_count == 0 || bootstrap.chunk_count > limits.max_chunks || chunk_count > total || total > chunk_count.saturating_mul(limits.max_chunk_bytes as u64) {
            return Err(artifact_bootstrap_error("chunk count cannot cover declared bytes within budget"));
        }
    }
    Ok(total)
}

fn validate_artifact_bootstrap(bootstrap: &ArtifactBootstrap, limits: ArtifactBootstrapLimits) -> Result<u64, semio_framework_replication_error::ProtocolError> {
    let inline = bootstrap.inline.is_some();
    let total = validate_artifact_bootstrap_header(bootstrap, inline, limits)?;
    if let Some(pair) = &bootstrap.inline {
        if pair.pack.len() as u64 != bootstrap.pack_length || pair.spr.len() as u64 != bootstrap.spr_length {
            return Err(artifact_bootstrap_error("inline pair lengths do not match metadata"));
        }
    }
    Ok(total)
}

/// 🔗️ SHA-256 over the exact declared `pack || spr` content stream.
pub fn artifact_bootstrap_aggregate_hash(pack: &[u8], spr: &[u8]) -> [u8; 32] {
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(pack);
    hash.update(spr);
    hash.finalize()
}

impl ArtifactBootstrapAssembler {
    /// 🆕️ Validates metadata and reserves no more than the caller's explicit budget.
    pub fn new(mut bootstrap: ArtifactBootstrap, expected_descriptor_hash: [u8; 32], limits: ArtifactBootstrapLimits, deadline_ms: Option<u64>, control: &mut impl ArtifactBootstrapControl) -> Result<Self, semio_framework_replication_error::ProtocolError> {
        let total = validate_artifact_bootstrap(&bootstrap, limits)?;
        if !artifact_bootstrap_nonzero(&expected_descriptor_hash) || bootstrap.descriptor_hash != expected_descriptor_hash {
            return Err(artifact_bootstrap_error("descriptor mismatch"));
        }
        if control.is_cancelled() {
            return Err(artifact_bootstrap_error("cancelled"));
        }
        if deadline_ms.is_some_and(|deadline| control.now_ms() >= deadline) {
            return Err(artifact_bootstrap_error("deadline exceeded"));
        }
        let inline_pair = bootstrap.inline.take();
        let inline = inline_pair.is_some();
        let storage = match inline_pair {
            Some(mut pair) => {
                pair.pack.shrink_to_fit();
                pair.spr.shrink_to_fit();
                if pair.pack.capacity().saturating_add(pair.spr.capacity()) as u64 > limits.max_total_bytes {
                    return Err(artifact_bootstrap_error("inline allocation exceeds assembler budget"));
                }
                ArtifactBootstrapStorage::Inline(pair)
            }
            None => {
                let storage = Vec::with_capacity(total as usize);
                if storage.capacity() as u64 > limits.max_total_bytes {
                    return Err(artifact_bootstrap_error("chunk allocation exceeds assembler budget"));
                }
                ArtifactBootstrapStorage::Chunked(storage)
            }
        };
        let mut assembler = Self { bootstrap, limits, deadline_ms, storage: Some(storage), received_bytes: 0, next_index: 0, inline };
        control.on_progress(assembler.progress());
        if inline {
            assembler.received_bytes = total;
            control.on_progress(assembler.progress());
        }
        Ok(assembler)
    }

    /// 💾️ Exact payload allocation debit currently owned by the assembler.
    pub fn retained_bytes(&self) -> usize {
        match self.storage.as_ref() {
            Some(ArtifactBootstrapStorage::Inline(pair)) => pair.pack.capacity().saturating_add(pair.spr.capacity()),
            Some(ArtifactBootstrapStorage::Chunked(bytes)) => bytes.capacity(),
            None => 0,
        }
    }

    /// 📈️ Current monotonic progress snapshot.
    pub fn progress(&self) -> ArtifactBootstrapProgress {
        ArtifactBootstrapProgress { received_bytes: self.received_bytes, total_bytes: self.bootstrap.pack_length + self.bootstrap.spr_length, received_chunks: self.next_index, total_chunks: self.bootstrap.chunk_count }
    }

    /// 🧹️ Drops all staged bytes without producing a completion value.
    pub fn abort(&mut self) {
        self.storage = None;
    }

    fn fail<T>(&mut self, detail: impl Into<String>) -> Result<T, semio_framework_replication_error::ProtocolError> {
        self.abort();
        Err(artifact_bootstrap_error(detail))
    }

    fn guard(&mut self, control: &mut impl ArtifactBootstrapControl) -> Result<(), semio_framework_replication_error::ProtocolError> {
        if control.is_cancelled() {
            return self.fail("cancelled");
        }
        if self.deadline_ms.is_some_and(|deadline| control.now_ms() >= deadline) {
            return self.fail("deadline exceeded");
        }
        if self.storage.is_none() {
            return Err(artifact_bootstrap_error("is not active"));
        }
        Ok(())
    }

    /// 🧩️ Appends exactly the next descriptor-bound, nonempty bounded chunk.
    pub fn push(&mut self, descriptor_hash: [u8; 32], index: u32, bytes: &[u8], control: &mut impl ArtifactBootstrapControl) -> Result<ArtifactBootstrapProgress, semio_framework_replication_error::ProtocolError> {
        self.guard(control)?;
        if self.inline {
            return self.fail("inline transfer cannot accept chunks");
        }
        if descriptor_hash != self.bootstrap.descriptor_hash {
            return self.fail("chunk descriptor mismatch");
        }
        if index != self.next_index {
            return self.fail(format!("chunk index {index} does not equal expected {}", self.next_index));
        }
        if bytes.is_empty() || bytes.len() > self.limits.max_chunk_bytes {
            return self.fail("chunk bytes exceed assembler budget");
        }
        if index >= self.bootstrap.chunk_count {
            return self.fail("chunk index exceeds declared count");
        }
        let Some(end) = self.received_bytes.checked_add(bytes.len() as u64) else { return self.fail("received bytes overflow") };
        if end > self.progress().total_bytes {
            return self.fail("chunk bytes exceed declared total");
        }
        match self.storage.as_mut() {
            Some(ArtifactBootstrapStorage::Chunked(storage)) => storage.extend_from_slice(bytes),
            _ => return self.fail("chunk storage is unavailable"),
        }
        self.received_bytes = end;
        self.next_index += 1;
        let progress = self.progress();
        control.on_progress(progress);
        Ok(progress)
    }

    /// ✅️ Verifies completion and all hashes, transfers ownership of the pair, then retires staging.
    pub fn finish(&mut self, done: Option<([u8; 32], u32)>, control: &mut impl ArtifactBootstrapControl) -> Result<ArtifactBootstrapPair, semio_framework_replication_error::ProtocolError> {
        self.guard(control)?;
        if !self.inline {
            let Some((descriptor_hash, chunk_count)) = done else { return self.fail("is incomplete without done frame") };
            if descriptor_hash != self.bootstrap.descriptor_hash {
                return self.fail("done descriptor mismatch");
            }
            if chunk_count != self.bootstrap.chunk_count {
                return self.fail("done chunk count mismatch");
            }
        } else if done.is_some_and(|(descriptor_hash, chunk_count)| descriptor_hash != self.bootstrap.descriptor_hash || chunk_count != 0) {
            return self.fail("inline done metadata mismatch");
        }
        if self.next_index != self.bootstrap.chunk_count || self.received_bytes != self.progress().total_bytes {
            return self.fail("is incomplete");
        }
        let (pack_hash, spr_hash, aggregate_hash) = match self.storage.as_ref() {
            Some(ArtifactBootstrapStorage::Inline(pair)) => (semio_framework_hash::Sha256::digest(&pair.pack), semio_framework_hash::Sha256::digest(&pair.spr), artifact_bootstrap_aggregate_hash(&pair.pack, &pair.spr)),
            Some(ArtifactBootstrapStorage::Chunked(bytes)) => {
                let split = self.bootstrap.pack_length as usize;
                let (pack, spr) = bytes.split_at(split);
                (semio_framework_hash::Sha256::digest(pack), semio_framework_hash::Sha256::digest(spr), artifact_bootstrap_aggregate_hash(pack, spr))
            }
            None => return Err(artifact_bootstrap_error("is not active")),
        };
        self.guard(control)?;
        if pack_hash != self.bootstrap.pack_hash {
            return self.fail("pack hash mismatch");
        }
        if spr_hash != self.bootstrap.spr_hash {
            return self.fail("SPR hash mismatch");
        }
        if aggregate_hash != self.bootstrap.aggregate_hash {
            return self.fail("aggregate hash mismatch");
        }
        let storage = self.storage.take().ok_or_else(|| artifact_bootstrap_error("is not active"))?;
        match storage {
            ArtifactBootstrapStorage::Inline(pair) => Ok(pair),
            ArtifactBootstrapStorage::Chunked(mut bytes) => {
                let spr = bytes.split_off(self.bootstrap.pack_length as usize);
                Ok(ArtifactBootstrapPair { pack: bytes, spr })
            }
        }
    }
}

/// ⚖️ How the semio_hub resolved one submitted operation against concurrent history.
#[derive(Clone, Debug, PartialEq)]
pub enum ApplyOutcome {
    Accepted,
    // 🔒️ Boxed: MutationEnvelope is far larger than the other variants, and clippy's
    // large_enum_variant lint (a real per-instance cost, not just style) applies at -D warnings.
    Transformed {
        envelope: Box<semio_framework_replication_causal::MutationEnvelope>,
    },
    /// 🧾 `messages` (trailing addition, tag unchanged) is one packed `Vec<MutationMessage>` blob —
    /// opaque here (this crate stays decoupled from `os_spr::command`'s concrete type, matching
    /// `ArtifactDiff`/`InverseMutation`'s opaque-bytes convention above), packed by the caller with
    /// `pack::encode_record_body` before construction. See contract-freeze.md §C8 of
    /// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    Rejected {
        reason: String,
        messages: Vec<u8>,
    },
}

/// 🪜️ One stage of a submitted batch's lifecycle, from `Received` to `Applied`.
#[derive(Clone, Debug, PartialEq)]
pub enum AckStage {
    Received,
    Persisted,
    // 🔒️ Boxed for the same reason as ApplyOutcome::Transformed above.
    Applied { outcome: Box<ApplyOutcome> },
}

pub const SNAPSHOT_CHUNK_BACKING_BYTES: usize = 4 * 1024;

/// 🧱️ One fixed-capacity snapshot frame backing whose owned allocation can never exceed one wire page.
#[derive(Clone)]
pub struct SnapshotChunkBytes {
    backing: Option<Box<[u8; SNAPSHOT_CHUNK_BACKING_BYTES]>>,
    len: u16,
}

impl SnapshotChunkBytes {
    /// 🆕️ Allocates exactly one fixed snapshot backing.
    pub fn allocate_fixed() -> Self {
        Self { backing: Some(Box::new([0; SNAPSHOT_CHUNK_BACKING_BYTES])), len: 0 }
    }

    /// 📥️ Copies one bounded source into a fixed snapshot backing.
    pub fn try_from_slice(source: &[u8]) -> Option<Self> {
        if source.len() > SNAPSHOT_CHUNK_BACKING_BYTES {
            return None;
        }
        let mut owner = Self::allocate_fixed();
        owner.try_extend_from_slice(source).then_some(owner)
    }

    /// ➕️ Appends bytes only while the fixed backing can retain the complete source.
    pub fn try_extend_from_slice(&mut self, source: &[u8]) -> bool {
        let start = usize::from(self.len);
        let Some(end) = start.checked_add(source.len()).filter(|end| *end <= SNAPSHOT_CHUNK_BACKING_BYTES) else { return false };
        let Some(backing) = self.backing.as_mut() else { return false };
        backing[start..end].copy_from_slice(source);
        self.len = end as u16;
        true
    }

    /// 📏️ Returns the initialized snapshot byte count.
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    /// 🩹️ Reports whether the initialized snapshot range is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 🧩️ Borrows the initialized snapshot byte range.
    pub fn as_slice(&self) -> &[u8] {
        self.backing.as_ref().map_or(&[], |backing| &backing[..usize::from(self.len)])
    }

    /// 🧮️ Returns the exact retained fixed backing debit.
    pub fn backing_bytes(&self) -> usize {
        self.backing.as_ref().map_or(0, |backing| size_of_val(backing.as_ref()))
    }

    /// 🧹️ Retires the single fixed backing in one explicit close opportunity.
    pub fn close_one(&mut self) -> bool {
        if self.backing.take().is_none() {
            return false;
        }
        self.len = 0;
        true
    }

    /// 🏁️ Reports whether the fixed backing owner has been retired.
    pub fn terminal_is_empty(&self) -> bool {
        self.backing.is_none()
    }
}

impl std::fmt::Debug for SnapshotChunkBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("SnapshotChunkBytes").field(&self.as_slice()).finish()
    }
}

impl PartialEq for SnapshotChunkBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for SnapshotChunkBytes {}

/// 🧱️ Artifact-bootstrap name for the same fixed-capacity wire backing, without changing private snapshot bytes.
pub type ArtifactBootstrapChunkBytes = SnapshotChunkBytes;

/// 📬️ One frame the semio_hub sends to a client.
#[derive(Clone, Debug, PartialEq)]
pub enum ServerFrame {
    Welcome {
        session_id: String,
        resume_token: String,
        server_frontier: semio_framework_replication_causal::FrontierSummary,
        bootstrap: Bootstrap,
    },
    SnapshotChunk {
        seq: u32,
        bytes: SnapshotChunkBytes,
    },
    SnapshotDone {
        seq_count: u32,
    },
    Commands {
        envelopes: Vec<semio_framework_replication_causal::MutationEnvelope>,
        origin: semio_framework_replication_ids::ActorId,
        frontier: semio_framework_replication_causal::FrontierSummary,
    },
    Ack {
        batch_id: u64,
        stages: Vec<AckStage>,
        frontier: semio_framework_replication_causal::FrontierSummary,
    },
    Preview {
        actor: semio_framework_replication_ids::ActorId,
        key: String,
        seq: u64,
        payload: Vec<u8>,
    },
    Presence {
        peers: Vec<Vec<u8>>,
    },
    CreditGrant {
        n: u32,
    },
    Error {
        code: String,
        message: String,
    },
    /// 🎨️ The hub's one-time session assignment for this connection: `color` is the
    /// hub-assigned palette index (`HubState.session_colors`, §C7.3), leased per `(space, actor)` and
    /// stamped by the client actor onto every outbound `PresencePeer` (never filled by a shell). Sent
    /// exactly once per connection, after `Welcome` (and its follow-up bootstrap frames) and before
    /// any `Presence` frame.
    Session {
        actor: String,
        color: u8,
    },
    ArtifactBootstrapChunk {
        descriptor_hash: [u8; 32],
        index: u32,
        bytes: ArtifactBootstrapChunkBytes,
    },
    ArtifactBootstrapDone {
        descriptor_hash: [u8; 32],
        chunk_count: u32,
    },
    RebootstrapRequired {
        control: RebootstrapRequired,
    },
}
//#endregion 🔖️ServerFrame

//#region 🔖️Codec

//#region 🔖️Combinators

async fn write_opt_frontier(out: &mut Vec<u8>, value: &Option<semio_framework_replication_causal::FrontierSummary>) {
    semio_framework_replication_wire_core::write_bool(out, value.is_some());
    if let Some(f) = value {
        semio_framework_replication_causal::binary::encode_frontier(f, out);
    }
}

async fn read_opt_frontier(bytes: &[u8], pos: &mut usize) -> Result<Option<semio_framework_replication_causal::FrontierSummary>, semio_framework_replication_error::ProtocolError> {
    if semio_framework_replication_wire_core::read_bool(bytes, pos)? {
        Ok(Some(semio_framework_replication_causal::binary::decode_frontier(bytes, pos)?))
    } else {
        Ok(None)
    }
}

async fn write_vec_envelope(out: &mut Vec<u8>, values: &[semio_framework_replication_causal::MutationEnvelope]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        semio_framework_replication_causal::binary::encode_envelope(value, out);
    }
}

async fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<semio_framework_replication_causal::MutationEnvelope>, semio_framework_replication_error::ProtocolError> {
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity((count as usize).min(bytes.len()));
    for _ in 0..count {
        out.push(semio_framework_replication_causal::binary::decode_envelope(bytes, pos)?);
    }
    Ok(out)
}
//#endregion 🔖️Combinators

//#region 🔖️NestedEnums
fn encode_artifact_bootstrap(bootstrap: &ArtifactBootstrap, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.format_version as u64);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.descriptor_hash);
    semio_framework_replication_wire_core::write_str(out, &bootstrap.artifact_schema);
    semio_framework_replication_wire_core::write_str(out, &bootstrap.artifact_kind);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.pack_schema_hash);
    semio_framework_replication_causal::binary::encode_frontier(&bootstrap.baseline_frontier, out);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.pack_hash);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.spr_hash);
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.pack_length);
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.spr_length);
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.chunk_count as u64);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.aggregate_hash);
    semio_framework_replication_causal::binary::encode_frontier(&bootstrap.required_tail_frontier, out);
    semio_framework_replication_wire_core::write_bool(out, bootstrap.inline.is_some());
    if let Some(pair) = &bootstrap.inline {
        semio_framework_replication_wire_core::write_bytes(out, &pair.pack);
        semio_framework_replication_wire_core::write_bytes(out, &pair.spr);
    }
}

fn read_artifact_bootstrap_string(bytes: &[u8], pos: &mut usize, name: &'static str) -> Result<String, semio_framework_replication_error::ProtocolError> {
    let length = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if length == 0 || length > 256 {
        return Err(artifact_bootstrap_error(format!("{name} length is invalid")));
    }
    let length = length as usize;
    let end = pos.checked_add(length).ok_or_else(|| artifact_bootstrap_error(format!("{name} length overflow")))?;
    let value = bytes.get(*pos..end).ok_or_else(|| artifact_bootstrap_error(format!("{name} is truncated")))?;
    let value = std::str::from_utf8(value).map_err(|_| artifact_bootstrap_error(format!("{name} is not UTF-8")))?.to_string();
    *pos = end;
    Ok(value)
}

fn read_artifact_bootstrap_bytes(bytes: &[u8], pos: &mut usize, expected: u64, name: &'static str) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
    let length = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if length != expected || length > ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES {
        return Err(artifact_bootstrap_error(format!("{name} length does not match metadata")));
    }
    let length = length as usize;
    let end = pos.checked_add(length).ok_or_else(|| artifact_bootstrap_error(format!("{name} length overflow")))?;
    let value = bytes.get(*pos..end).ok_or_else(|| artifact_bootstrap_error(format!("{name} is truncated")))?.to_vec();
    *pos = end;
    Ok(value)
}

fn decode_artifact_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<ArtifactBootstrap, semio_framework_replication_error::ProtocolError> {
    let format_version = u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?).map_err(|_| artifact_bootstrap_error("format version exceeds u32"))?;
    let mut bootstrap = ArtifactBootstrap {
        format_version,
        descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        artifact_schema: read_artifact_bootstrap_string(bytes, pos, "artifact schema")?,
        artifact_kind: read_artifact_bootstrap_string(bytes, pos, "artifact kind")?,
        pack_schema_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        baseline_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, pos)?,
        pack_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        spr_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        pack_length: semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?,
        spr_length: semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?,
        chunk_count: u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?).map_err(|_| artifact_bootstrap_error("chunk count exceeds u32"))?,
        aggregate_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        required_tail_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, pos)?,
        inline: None,
    };
    let inline = semio_framework_replication_wire_core::read_bool(bytes, pos)?;
    validate_artifact_bootstrap_header(&bootstrap, inline, ArtifactBootstrapLimits::default())?;
    if inline {
        bootstrap.inline = Some(ArtifactBootstrapPair { pack: read_artifact_bootstrap_bytes(bytes, pos, bootstrap.pack_length, "inline pack")?, spr: read_artifact_bootstrap_bytes(bytes, pos, bootstrap.spr_length, "inline SPR")? });
    }
    validate_artifact_bootstrap(&bootstrap, ArtifactBootstrapLimits::default())?;
    Ok(bootstrap)
}

async fn encode_bootstrap(bootstrap: &Bootstrap, out: &mut Vec<u8>) {
    match bootstrap {
        Bootstrap::None => out.push(0),
        Bootstrap::Snapshot { pack_hash, inline } => {
            out.push(1);
            semio_framework_replication_wire_core::write_hash32(out, pack_hash);
            write_opt_bytes(out, inline).await;
        }
        Bootstrap::Tail => out.push(2),
        Bootstrap::ArtifactBootstrap(bootstrap) => {
            out.push(3);
            encode_artifact_bootstrap(bootstrap, out);
        }
    }
}

async fn decode_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<Bootstrap, semio_framework_replication_error::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire bootstrap tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(Bootstrap::None),
        1 => {
            let pack_hash = semio_framework_replication_wire_core::read_hash32(bytes, pos)?;
            let inline = read_opt_bytes(bytes, pos).await?;
            Ok(Bootstrap::Snapshot { pack_hash, inline })
        }
        2 => Ok(Bootstrap::Tail),
        3 => Ok(Bootstrap::ArtifactBootstrap(Box::new(decode_artifact_bootstrap(bytes, pos)?))),
        other => Err(malformed("wire bootstrap tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn encode_apply_outcome(outcome: &ApplyOutcome, out: &mut Vec<u8>) {
    match outcome {
        ApplyOutcome::Accepted => out.push(0),
        ApplyOutcome::Transformed { envelope } => {
            out.push(1);
            semio_framework_replication_causal::binary::encode_envelope(envelope, out);
        }
        ApplyOutcome::Rejected { reason, messages } => {
            out.push(2);
            semio_framework_replication_wire_core::write_str(out, reason);
            semio_framework_replication_wire_core::write_bytes(out, messages);
        }
    }
}

async fn decode_apply_outcome(bytes: &[u8], pos: &mut usize) -> Result<ApplyOutcome, semio_framework_replication_error::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire apply-outcome tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(ApplyOutcome::Accepted),
        1 => Ok(ApplyOutcome::Transformed { envelope: Box::new(semio_framework_replication_causal::binary::decode_envelope(bytes, pos)?) }),
        2 => Ok(ApplyOutcome::Rejected { reason: semio_framework_replication_wire_core::read_str(bytes, pos)?, messages: semio_framework_replication_wire_core::read_bytes(bytes, pos)? }),
        other => Err(malformed("wire apply-outcome tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn encode_ack_stage(stage: &AckStage, out: &mut Vec<u8>) {
    match stage {
        AckStage::Received => out.push(0),
        AckStage::Persisted => out.push(1),
        AckStage::Applied { outcome } => {
            out.push(2);
            encode_apply_outcome(outcome, out).await;
        }
    }
}

async fn decode_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<AckStage, semio_framework_replication_error::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire ack-stage tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(AckStage::Received),
        1 => Ok(AckStage::Persisted),
        2 => Ok(AckStage::Applied { outcome: Box::new(decode_apply_outcome(bytes, pos).await?) }),
        other => Err(malformed("wire ack-stage tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn write_vec_ack_stage(out: &mut Vec<u8>, values: &[AckStage]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_ack_stage(value, out).await;
    }
}

async fn read_vec_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<Vec<AckStage>, semio_framework_replication_error::ProtocolError> {
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(decode_ack_stage(bytes, pos).await?);
    }
    Ok(out)
}
//#endregion 🔖️NestedEnums

/// 📤️ Encodes one `ClientFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub async fn encode_client_frame(frame: &ClientFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte().await);
    match frame {
        ClientFrame::SocketHelloV1 { wire_version, protocol_version, schema, pack_schema_hash, resume_token, frontier } => {
            out.push(7);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *wire_version as u64);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *protocol_version as u64);
            semio_framework_replication_wire_core::write_str(&mut out, schema);
            semio_framework_replication_wire_core::write_hash32(&mut out, pack_schema_hash);
            write_opt_str(&mut out, resume_token).await;
            write_opt_frontier(&mut out, frontier).await;
        }
        ClientFrame::Commands { batch_id, envelopes } => {
            out.push(1);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *batch_id);
            write_vec_envelope(&mut out, envelopes).await;
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            out.push(2);
            semio_framework_replication_causal::binary::encode_frontier(frontier, &mut out);
        }
        ClientFrame::PreviewPublish { key, seq, payload } => {
            out.push(3);
            semio_framework_replication_wire_core::write_str(&mut out, key);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq);
            semio_framework_replication_wire_core::write_bytes(&mut out, payload);
        }
        ClientFrame::Presence { peer } => {
            out.push(4);
            semio_framework_replication_wire_core::write_bytes(&mut out, peer);
        }
        ClientFrame::CreditGrant { n } => {
            out.push(5);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *n as u64);
        }
        ClientFrame::Bye => out.push(6),
    }
    out
}

/// 📥️ Decodes one `ClientFrame`, returning the `Lane` it was tagged with.
pub async fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), semio_framework_replication_error::ProtocolError> {
    let lane_byte = match bytes.first() {
        Some(b) => *b,
        None => return Err(malformed("wire frame", 0, "empty frame").await),
    };
    let lane = match Lane::from_byte(lane_byte).await {
        Some(l) => l,
        None => return Err(malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")).await),
    };
    let mut pos = 1usize;
    let tag = match bytes.get(pos) {
        Some(b) => *b,
        None => return Err(malformed("wire client-frame tag", pos as u64, "truncated").await),
    };
    pos += 1;
    let frame = match tag {
        1 => ClientFrame::Commands { batch_id: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos).await? },
        2 => ClientFrame::FrontierAdvertise { frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)? },
        3 => ClientFrame::PreviewPublish { key: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?, seq: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, payload: semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)? },
        4 => ClientFrame::Presence { peer: semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)? },
        5 => ClientFrame::CreditGrant { n: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32 },
        6 => ClientFrame::Bye,
        7 => ClientFrame::SocketHelloV1 {
            wire_version: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32,
            protocol_version: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32,
            schema: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?,
            pack_schema_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?,
            resume_token: read_opt_str(bytes, &mut pos).await?,
            frontier: read_opt_frontier(bytes, &mut pos).await?,
        },
        other => return Err(malformed("wire client-frame tag", pos as u64, &format!("unknown tag {other:#x}")).await),
    };
    Ok((lane, frame))
}

/// 📤️ Encodes one `ServerFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub async fn encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte().await);
    match frame {
        ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap } => {
            out.push(0);
            semio_framework_replication_wire_core::write_str(&mut out, session_id);
            semio_framework_replication_wire_core::write_str(&mut out, resume_token);
            semio_framework_replication_causal::binary::encode_frontier(server_frontier, &mut out);
            encode_bootstrap(bootstrap, &mut out).await;
        }
        ServerFrame::SnapshotChunk { seq, bytes } => {
            out.push(1);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq as u64);
            semio_framework_replication_wire_core::write_bytes(&mut out, bytes.as_slice());
        }
        ServerFrame::SnapshotDone { seq_count } => {
            out.push(2);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq_count as u64);
        }
        ServerFrame::Commands { envelopes, origin, frontier } => {
            out.push(3);
            write_vec_envelope(&mut out, envelopes).await;
            semio_framework_replication_wire_core::write_str(&mut out, &origin.0);
            semio_framework_replication_causal::binary::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Ack { batch_id, stages, frontier } => {
            out.push(4);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *batch_id);
            write_vec_ack_stage(&mut out, stages).await;
            semio_framework_replication_causal::binary::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Preview { actor, key, seq, payload } => {
            out.push(5);
            semio_framework_replication_wire_core::write_str(&mut out, &actor.0);
            semio_framework_replication_wire_core::write_str(&mut out, key);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq);
            semio_framework_replication_wire_core::write_bytes(&mut out, payload);
        }
        ServerFrame::Presence { peers } => {
            out.push(6);
            write_vec_bytes(&mut out, peers).await;
        }
        ServerFrame::CreditGrant { n } => {
            out.push(7);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *n as u64);
        }
        ServerFrame::Error { code, message } => {
            out.push(8);
            semio_framework_replication_wire_core::write_str(&mut out, code);
            semio_framework_replication_wire_core::write_str(&mut out, message);
        }
        ServerFrame::Session { actor, color } => {
            out.push(9);
            semio_framework_replication_wire_core::write_str(&mut out, actor);
            out.push(*color);
        }
        ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
            out.push(10);
            semio_framework_replication_wire_core::write_hash32(&mut out, descriptor_hash);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *index as u64);
            semio_framework_replication_wire_core::write_bytes(&mut out, bytes.as_slice());
        }
        ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
            out.push(11);
            semio_framework_replication_wire_core::write_hash32(&mut out, descriptor_hash);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *chunk_count as u64);
        }
        ServerFrame::RebootstrapRequired { control } => {
            out.push(12);
            semio_framework_replication_wire_core::write_str(&mut out, &control.space_id);
            semio_framework_replication_wire_core::write_str(&mut out, &control.document_id);
            semio_framework_replication_wire_core::write_hash32(&mut out, &control.checkpoint_id);
            semio_framework_replication_wire_core::write_hash32(&mut out, &control.descriptor_hash);
            semio_framework_replication_causal::binary::encode_frontier(&control.baseline_frontier, &mut out);
        }
    }
    out
}

fn read_snapshot_chunk_bytes(bytes: &[u8], pos: &mut usize) -> Result<SnapshotChunkBytes, semio_framework_replication_error::ProtocolError> {
    let len = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if len > SNAPSHOT_CHUNK_BACKING_BYTES as u64 {
        return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("snapshot chunk fixed backing"));
    }
    let len = len as usize;
    let end = pos.checked_add(len).ok_or(semio_framework_replication_error::ProtocolError::Malformed { what: "wire snapshot chunk", offset: *pos as u64, detail: "length overflow".to_string() })?;
    let source = bytes.get(*pos..end).ok_or(semio_framework_replication_error::ProtocolError::Malformed { what: "wire snapshot chunk", offset: *pos as u64, detail: "truncated".to_string() })?;
    let owner = SnapshotChunkBytes::try_from_slice(source).ok_or(semio_framework_replication_error::ProtocolError::LimitExceeded("snapshot chunk fixed backing"))?;
    *pos = end;
    Ok(owner)
}

fn read_artifact_bootstrap_chunk_bytes(bytes: &[u8], pos: &mut usize) -> Result<ArtifactBootstrapChunkBytes, semio_framework_replication_error::ProtocolError> {
    let bytes = read_snapshot_chunk_bytes(bytes, pos)?;
    if bytes.is_empty() {
        return Err(artifact_bootstrap_error("chunk bytes must be nonempty"));
    }
    Ok(bytes)
}

/// 📥️ Decodes one `ServerFrame`, returning the `Lane` it was tagged with.
pub async fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), semio_framework_replication_error::ProtocolError> {
    let lane_byte = match bytes.first() {
        Some(b) => *b,
        None => return Err(malformed("wire frame", 0, "empty frame").await),
    };
    let lane = match Lane::from_byte(lane_byte).await {
        Some(l) => l,
        None => return Err(malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")).await),
    };
    let mut pos = 1usize;
    let tag = match bytes.get(pos) {
        Some(b) => *b,
        None => return Err(malformed("wire server-frame tag", pos as u64, "truncated").await),
    };
    pos += 1;
    let frame = match tag {
        0 => ServerFrame::Welcome {
            session_id: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?,
            resume_token: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?,
            server_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)?,
            bootstrap: decode_bootstrap(bytes, &mut pos).await?,
        },
        1 => ServerFrame::SnapshotChunk { seq: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32, bytes: read_snapshot_chunk_bytes(bytes, &mut pos)? },
        2 => ServerFrame::SnapshotDone { seq_count: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32 },
        3 => ServerFrame::Commands { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: semio_framework_replication_ids::ActorId(semio_framework_replication_wire_core::read_str(bytes, &mut pos)?.into()), frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)? },
        4 => ServerFrame::Ack { batch_id: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, stages: read_vec_ack_stage(bytes, &mut pos).await?, frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)? },
        5 => ServerFrame::Preview { actor: semio_framework_replication_ids::ActorId(semio_framework_replication_wire_core::read_str(bytes, &mut pos)?.into()), key: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?, seq: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, payload: semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)? },
        6 => ServerFrame::Presence { peers: read_vec_bytes(bytes, &mut pos).await? },
        7 => ServerFrame::CreditGrant { n: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32 },
        8 => ServerFrame::Error { code: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?, message: semio_framework_replication_wire_core::read_str(bytes, &mut pos)? },
        9 => {
            let actor = semio_framework_replication_wire_core::read_str(bytes, &mut pos)?;
            let color = semio_framework_replication_wire_core::read_u8(bytes, &mut pos)?;
            ServerFrame::Session { actor, color }
        }
        10 => ServerFrame::ArtifactBootstrapChunk { descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?, index: u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?).map_err(|_| artifact_bootstrap_error("chunk index exceeds u32"))?, bytes: read_artifact_bootstrap_chunk_bytes(bytes, &mut pos)? },
        11 => ServerFrame::ArtifactBootstrapDone { descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?, chunk_count: u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?).map_err(|_| artifact_bootstrap_error("done chunk count exceeds u32"))? },
        12 => {
            let control = RebootstrapRequired {
                space_id: read_artifact_bootstrap_string(bytes, &mut pos, "rebootstrap space")?,
                document_id: read_artifact_bootstrap_string(bytes, &mut pos, "rebootstrap document")?,
                checkpoint_id: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?,
                descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?,
                baseline_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)?,
            };
            validate_rebootstrap_required(&control)?;
            ServerFrame::RebootstrapRequired { control }
        }
        other => return Err(malformed("wire server-frame tag", pos as u64, &format!("unknown tag {other:#x}")).await),
    };
    Ok((lane, frame))
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

