//! 🧱️ The `document_socket` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use semio_hub_directory_contract::{AUTH_TEXT_MAX_BYTES, SocketGrantCapability};
use protocol::{AckStage, ActorId, ApplyOutcome, Lane, MutationEnvelope, ArtifactId as ProtocolArtifactId, RuntimeFrontierSummary, ServerFrame, encode_server_frame};
use std::sync::Arc;
use serde::Deserialize;
use semio_hub_ledgers::open_plan::DocumentOpenPlanAuthorityV1;
use directory::{DslValue, ToValue};
use axum::http::{HeaderMap, StatusCode};
use axum::extract::ws::{Message, WebSocket};
use semio_hub_lag_rebootstrap::{RebootstrapProgress, RebootstrapTransferControl};
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SOCKET_PROTOCOL_V1};
use futures::stream::{SplitSink, SplitStream};
use semio_hub_runtime::support::now_ms;

//#region 🔖️WebSocket
pub fn socket_grant_from_protocol_header(headers: &HeaderMap) -> Result<SocketGrantCapability, StatusCode> {
    if headers.contains_key(axum::http::header::AUTHORIZATION) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let values = headers.get_all(axum::http::header::SEC_WEBSOCKET_PROTOCOL);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let offered = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if offered.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (protocol, grant) = offered.split_once(", ").ok_or(StatusCode::UNAUTHORIZED)?;
    if protocol != SOCKET_PROTOCOL_V1 || grant.contains(',') {
        return Err(StatusCode::UNAUTHORIZED);
    }
    SocketGrantCapability::parse(grant).map_err(|_| StatusCode::UNAUTHORIZED)
}

/// 🌱️ Whether a document-open plan's bootstrap facts are still the hub's current ones.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentPlanBootstrapV1 {
    Current,
    Stale,
    Unavailable,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentWsV1Query {
    pub surface: Option<String>,
}

pub fn credential_from_protocol_header(headers: &HeaderMap) -> Result<String, StatusCode> {
    if headers.contains_key(axum::http::header::AUTHORIZATION) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let values = headers.get_all(axum::http::header::SEC_WEBSOCKET_PROTOCOL);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let offered = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if offered.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (protocol, token) = offered.split_once(", ").ok_or(StatusCode::UNAUTHORIZED)?;
    if protocol != SESSION_PROTOCOL_V1 || token.is_empty() || token.contains(',') {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(token.to_string())
}

pub async fn encode(frame: &ServerFrame, document_id: &str) -> Message {
    if frame_carries_frontier(frame) {
        let mut projected = frame.clone();
        project_server_frame(&mut projected, document_id);
        return Message::Binary(encode_server_frame(&projected, Lane::Command).await.into());
    }
    Message::Binary(encode_server_frame(frame, Lane::Command).await.into())
}

pub async fn error_frame(code: &str, message: impl Into<String>) -> Message {
    encode(&ServerFrame::Error { code: code.to_string(), message: message.into() }, "").await
}

/// 🧭️ Best-effort `RuntimeFrontierSummary` for an `Ack` when the triggering `submit` itself
/// failed — re-reads the document's current (unaffected) frontier so the client still learns
/// "where the server actually is", falling back to an all-zero genesis summary only if even that
/// read fails (a document wedged badly enough that this happens has bigger problems than one Ack).
pub async fn best_effort_frontier(handle: &db::ArtifactHandle) -> RuntimeFrontierSummary {
    match handle.frontier().await {
        Ok(frontier) => engine_frontier_to_wire(&frontier, String::new()),
        Err(_) => RuntimeFrontierSummary { document_id: handle.document_id().await.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] },
    }
}

fn engine_frontier_to_wire(frontier: &db::engine::Frontier, head_edit_id: String) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: frontier.document.clone(), head_edit_ordinal: frontier.head_seq, head_edit_id, last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
}

/// 🔁️ The origin this hub stamps on every catch-up tail (hello and `FrontierAdvertise`): a declared hub identity, never the receiving socket's actor — a
/// tail carries anyone's edits, and a replica that read its own actor there discarded the whole history as its own echo (ticket
/// 26/09/23 session 12, run s12i). Replicas suppress echoes by operation identity; this names the tail. Pinned to
/// `🏪️store/🧫️fixtures/document-echo-suppression-v1` `hubCatchUpOrigin`.
pub const HUB_CATCH_UP_ORIGIN: &str = "hub.catch-up";

/// 📨️ Submits one batch and returns its `Ack` plus the `Commands` relay for peers. The caller
/// holds the document's write gate, so the frontier read first is exactly the one the submit starts
/// from: a receipt that does not advance `commit_seq` is the engine's idempotent replay of an
/// already-committed `command_id` — acknowledged again for the resending client, never relayed as new.
/// An accepted batch's operations join `gate`'s replay guard; a refused one's never do, so its resend
/// is admitted again. A transient refusal (`db::DbError::is_transient`) says so with the transient code, and its author
/// resends the batch.
pub async fn submit_commands(handle: &db::ArtifactHandle, gate: &db::security::SecurityGate, actor: &ActorId, batch_id: u64, envelopes: Vec<MutationEnvelope>, policy: protocol::MergePolicy) -> (ServerFrame, Option<ServerFrame>) {
    let committed_before = handle.frontier().await.map(|frontier| frontier.commit_seq).ok();
    let batch = match db::document::CommandBatch::new(envelopes.clone()).await {
        Ok(batch) => batch,
        Err(error) => {
            let frontier = best_effort_frontier(handle).await;
            return (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages: Vec::new() }) }], frontier }, None);
        }
    };
    match handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy }).await {
        Ok(Ok(receipt)) => {
            gate.record_committed(&envelopes.iter().map(|envelope| (&envelope.actor, &envelope.mutation_id)).collect::<Vec<_>>(), now_ms().max(0) as u64);
            let frontier = engine_frontier_to_wire(&receipt.frontier, receipt.command_id.0.clone());
            let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() };
            let advanced = committed_before.is_some_and(|before| receipt.frontier.commit_seq > before);
            (ack, advanced.then(|| ServerFrame::Commands { envelopes, origin: actor.clone(), frontier }))
        }
        Ok(Err(error)) | Err(error) => {
            let frontier = best_effort_frontier(handle).await;
            let messages = messages_for_error(&error);
            (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages }) }], frontier }, None)
        }
    }
}

/// 🚪️ Admits `envelopes` as one batch through `gate.admit_commands` (tenant isolation, then
/// `Action::Write` authz on `AuthzScope::CommandKind`, the DoS budget charged for the whole batch at
/// once, replay dedupe — see `SecurityGate::admit_commands`' own doc) before any of them reach
/// `db::ArtifactHandle::submit`; admission records nothing, [`submit_commands`] records the committed
/// operations. `kind` is a constant ("write") rather than a per-envelope command-kind string: this
/// crate sits above `db_artifact`'s pipeline and never interprets an operation's schema/diff semantics
/// (matches `db_security`'s own module doc — payload interpretation stays out of this layer), so
/// command-kind granularity inside one document is not this wave's concern.
pub async fn admit_writes(gate: &db::security::SecurityGate, principal: &db::security::Principal, tenant: &db::security::TenantId, document: &ProtocolArtifactId, envelopes: &[MutationEnvelope], physical_ms: u64) -> Result<(), db::DbError> {
    let commands: Vec<(&ActorId, &protocol::MutationId)> = envelopes.iter().map(|envelope| (&envelope.actor, &envelope.mutation_id)).collect();
    gate.admit_commands(principal, tenant, document, "write", &commands, physical_ms).await
}

/// 🪣️ A document socket's DoS budget, in commands: its capacity is the document backbone's declared
/// batch maximum, so a full bucket admits every batch the wire declares legal — a link cut's whole
/// outbox arrives as one (ticket 26/09/23 session 14, C12 P1) — and an emptied one refuses
/// transiently (`hub.unavailable`) until it refills at [`DOCUMENT_SOCKET_COMMAND_REFILL_PER_SECOND`].
const DOCUMENT_SOCKET_COMMAND_BUDGET: u32 = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES as u32;

/// 🪣️ Commands per second a document socket's budget regains.
pub const DOCUMENT_SOCKET_COMMAND_REFILL_PER_SECOND: u32 = 60;

/// 🚧️ One document socket's security gate over `policy`: a 60 s replay window of 256 committed
/// operations per actor and the declared-batch-sized command budget.
pub fn document_socket_gate(policy: db::security::RoleBasedPolicy) -> db::security::SecurityGate {
    db::security::SecurityGate::new(policy, db::security::ReplayGuard::new(60_000, 256), db::security::BudgetRegistry::new(DOCUMENT_SOCKET_COMMAND_BUDGET, DOCUMENT_SOCKET_COMMAND_REFILL_PER_SECOND), Arc::new(db::NullEmit))
}

/// ⏱️ How long one document-socket frame's admission may take — authority, the batch's own refusals, the
/// security gate and the document's write gate — before its socket closes `1013 frame-deadline` and its
/// client resynchronizes; nothing of such a frame reached the engine. A `Commands` batch the engine received
/// is never cut by it: its commit is awaited to the engine's answer and acknowledged, since a batch that
/// committed after a cut used to land durably with its socket closed and its Ack lost (ticket 26/09/23
/// session 14c, H13's transient probe: batch 24 committed at head 6144, no Ack).
pub const DOCUMENT_SOCKET_FRAME_DEADLINE: std::time::Duration = std::time::Duration::from_secs(30);

/// ⏳️ How long an agent's document `Commands` frame waits for its session's `agent-command` budget before it is refused
/// (the MCP gateway's relay-acknowledgement wait): pacing keeps an honest agent's commits whole, and the wait sits before the
/// socket re-reads its authority, so a revocation that lands while a frame waits still fences it.
pub const AGENT_COMMAND_PATIENCE_MS: u64 = 10_000;

/// 🚦️ What one decoded frame leaves its socket to do once its admission finished within the frame deadline. A refused
/// batch never ends or rebuilds its socket: its author rolls refused operations back by their inverses and retracts
/// refused history transitions (`BackboneMessage::Retract`, ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING), so it
/// converges with the log without a rebuild.
pub enum ClientFrameStepV1 {
    Continue,
    End,
    Commit(AdmittedCommandsV1),
}

impl ClientFrameStepV1 {
    /// 📤️ `Continue` when the frame's answer was sent, `End` when the socket is gone.
    pub fn after_send(sent: bool) -> Self {
        if sent { Self::Continue } else { Self::End }
    }
}

/// ✍️ One `Commands` batch admitted within the frame deadline, holding its document's write gate until its
/// commit is answered.
pub struct AdmittedCommandsV1 {
    pub batch_id: u64,
    pub envelopes: Vec<MutationEnvelope>,
    pub _document_write: tokio::sync::OwnedMutexGuard<()>,
}

/// ⏳️ How long a socket the hub is closing keeps reading the client's remaining frames for its closing reply.
const SOCKET_CLOSE_DRAIN: std::time::Duration = std::time::Duration::from_secs(1);

/// 🚪️ Ends a socket the way the WebSocket closing handshake asks: `close` sent — refused, and harmless, when a
/// typed close already went out — then [`drain_closing_socket`].
pub async fn close_socket(sender: &mut SplitSink<WebSocket, Message>, receiver: &mut SplitStream<WebSocket>, close: Message) {
    let _ = sender.send(close).await;
    drain_closing_socket(receiver).await;
}

/// 🧽️ Reads and discards what the client still sends until its own close, its disconnect or
/// [`SOCKET_CLOSE_DRAIN`]. Dropping a socket with unread frames resets the TCP connection, and a reset can
/// discard the close frame the client has not read yet: a client that kept sending after a refusal saw a bare
/// `Connection reset by peer` instead of the hub's typed close (found by the generative hostile-input law).
async fn drain_closing_socket(receiver: &mut SplitStream<WebSocket>) {
    let _ = tokio::time::timeout(SOCKET_CLOSE_DRAIN, async {
        while let Some(Ok(message)) = receiver.next().await {
            if matches!(message, Message::Close(_)) {
                break;
            }
        }
    })
    .await;
}
//#endregion 🔖️WebSocket

//#region 🔖️Directory
pub fn wire_rebootstrap(control: &directory::os_directory::RebootstrapRequired) -> protocol::RebootstrapRequired {
    protocol::RebootstrapRequired {
        space_id: control.scope.space_id.clone(),
        document_id: control.scope.document_id.clone(),
        checkpoint_id: control.checkpoint_id.0,
        descriptor_hash: control.descriptor_digest_v1.0,
        baseline_frontier: RuntimeFrontierSummary {
            document_id: ProtocolArtifactId(control.baseline_frontier.document_id.clone()),
            head_edit_ordinal: control.baseline_frontier.head_edit_ordinal,
            head_edit_id: control.baseline_frontier.head_edit_id.clone(),
            last_commit_seq: control.baseline_frontier.last_commit_seq,
            chain_hash: control.baseline_frontier.chain_hash.0,
        },
    }
}

pub struct SocketRebootstrapControl;

impl RebootstrapTransferControl for SocketRebootstrapControl {
    fn now_ms(&self) -> u64 {
        now_ms().max(0) as u64
    }

    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, _progress: RebootstrapProgress) {}
}

/// 🧾️ `ApplyOutcome::Rejected.messages`'s canonical JSON payload, encoded from the
/// first-party `ToValue` shape shared by every replication wire consumer.
pub fn encode_messages(messages: &[protocol::MutationMessage]) -> Vec<u8> {
    let value = DslValue::Array(messages.iter().map(ToValue::to_value).collect());
    semio_framework_pack_json::to_json_string(&value).into_bytes()
}

/// 🧾️ Every `protocol::MutationMessage` `error` carries, if any: `db::DbError::Rejected`'s own (the outcome-step
/// gate `db_artifact::ArtifactEngine::submit` returns per contract §C9), and for `db::DbError::Unavailable` — the engine
/// could not admit the batch NOW (DB I/O admission or capacity exhausted) — the declared transient refusal, so the client
/// resends the batch instead of discarding it; every other `DbError` variant has nothing to add here.
pub fn messages_for_error(error: &db::DbError) -> Vec<u8> {
    match error {
        db::DbError::Rejected { messages, .. } => encode_messages(messages),
        db::DbError::Unavailable(reason) => transient_apply_refusal_messages(reason),
        error if error.is_transient() => transient_apply_refusal_messages(&error.to_string()),
        db::DbError::LimitExceeded(limit) => batch_limit_refusal_messages(&format!("limit exceeded: {limit}")),
        _ => Vec::new(),
    }
}

/// 📏️ `ApplyOutcome::Rejected.messages` of a batch the hub can never admit: the one `HubBatchLimitRefusalMessageV1`
/// (`🚧️refusal`), level error, code `hub.batch-limit`, the exceeded limit bounded — permanent, never the transient code.
fn batch_limit_refusal_messages(reason: &str) -> Vec<u8> {
    encode_messages(&[protocol::MutationMessage::error(semio_hub_refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, semio_hub_refusal::hub_batch_limit_refusal_message(reason))])
}

/// 📏️ The permanent refusal of a `Commands` batch the document backbone does not declare legal
/// (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`, judged by the wire's own exact decoder over the batch's canonical
/// encoding); `None` for every declared batch, which the socket budget and the db engine admit as one submit.
pub fn undeclared_batch_refusal(envelopes: &[MutationEnvelope]) -> Option<ApplyOutcome> {
    let limit = protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(envelopes)).err()?;
    let reason = format!("batch exceeds the document backbone's declaration: {limit}");
    Some(ApplyOutcome::Rejected { messages: batch_limit_refusal_messages(&reason), reason })
}

/// ⏳️ `ApplyOutcome::Rejected.messages` of a batch refused for a transient reason: the one
/// `HubTransientApplyRefusalMessageV1` (`🚧️refusal`), level warning, code `hub.unavailable`, the reason bounded.
pub fn transient_apply_refusal_messages(reason: &str) -> Vec<u8> {
    encode_messages(&[protocol::MutationMessage::warning(semio_hub_refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE, semio_hub_refusal::hub_transient_apply_refusal_message(reason))])
}

/// 🧭️ Stamps the DOCUMENT's own id onto one outbound wire frontier.
///
/// The hub keys documents internally by [`db_artifact_id`] — `v1:<len>:<len>:<space><document>` —
/// because the db and fanout catalogs are flat and a bare document id is not unique across spaces.
/// The db engine stamps that key into every `Frontier` it returns and the replication wire carries
/// it through unchanged, so until this projection existed every `Welcome`, `Ack`, `Commands` and
/// rebootstrap control named the hub's private key where the client's own `documentId` belongs. A
/// client that checks the identity it asked for against the identity it was handed — the store
/// worker's `validateArtifactBootstrapIdentity` does exactly that — refuses a pair the hub itself
/// produced. This is the single boundary where the internal key becomes the client's identity
/// again, and [`wire_frontier_to_db`] is its exact inverse on the way in.
fn project_wire_frontier(frontier: &mut RuntimeFrontierSummary, document_id: &str) {
    frontier.document_id = ProtocolArtifactId(document_id.to_string());
}

/// 🧭️ Every frontier one outbound frame carries, projected onto the document's own id. A
/// frame with no frontier is returned untouched and uncloned by the caller.
pub fn project_server_frame(frame: &mut ServerFrame, document_id: &str) {
    match frame {
        ServerFrame::Welcome { server_frontier, bootstrap, .. } => {
            project_wire_frontier(server_frontier, document_id);
            if let protocol::Bootstrap::ArtifactBootstrap(artifact) = bootstrap {
                project_wire_frontier(&mut artifact.baseline_frontier, document_id);
                project_wire_frontier(&mut artifact.required_tail_frontier, document_id);
            }
        }
        ServerFrame::Commands { frontier, envelopes, .. } => {
            project_wire_frontier(frontier, document_id);
            for envelope in envelopes {
                envelope.document_id = ProtocolArtifactId(document_id.to_string());
            }
        }
        ServerFrame::Ack { frontier, .. } => project_wire_frontier(frontier, document_id),
        ServerFrame::RebootstrapRequired { control } => project_wire_frontier(&mut control.baseline_frontier, document_id),
        ServerFrame::SnapshotChunk { .. }
        | ServerFrame::SnapshotDone { .. }
        | ServerFrame::Preview { .. }
        | ServerFrame::Presence { .. }
        | ServerFrame::CreditGrant { .. }
        | ServerFrame::Error { .. }
        | ServerFrame::Session { .. }
        | ServerFrame::ArtifactBootstrapChunk { .. }
        | ServerFrame::ArtifactBootstrapDone { .. } => {}
    }
}

/// 🧭️ Whether one frame carries a frontier at all, so the projection clones only when it has
/// something to rewrite.
pub const fn frame_carries_frontier(frame: &ServerFrame) -> bool {
    matches!(frame, ServerFrame::Welcome { .. } | ServerFrame::Commands { .. } | ServerFrame::Ack { .. } | ServerFrame::RebootstrapRequired { .. })
}

/// 🧭️ The inverse of [`project_wire_frontier`]: one frontier a CLIENT sent, named by the
/// document id it opened, re-keyed onto this hub's internal db id before any db layer compares it
/// (`db_sync` refuses a hello whose advertised frontier does not name its own document). A frontier
/// naming anything other than the socket's own document is refused rather than re-keyed — the hub
/// must never accept an identity claim it then overwrites.
/// 🪢️ Where a hello that names no frontier resumes: at the baseline of its plan's checkpoint when the plan's client
/// seeds its document from that checkpoint's canonical pair (a `closed-browser-actor` plan) and the checkpoint holds edits —
/// the pair already carries every edit up to it, so the tail starts after it and stays bounded by the checkpoint instead of
/// the document's whole history. `None` (the tail from the start) otherwise. Keyed by the hub's internal document key.
pub fn plan_seeded_hello_frontier(plan: Option<&DocumentOpenPlanAuthorityV1>, db_id: &ProtocolArtifactId) -> Option<RuntimeFrontierSummary> {
    let plan = plan?;
    let baseline = &plan.checkpoint.baseline_frontier;
    if !matches!(plan.browser_actor, directory::os_directory::DocumentOpenBrowserActorV1::ClosedBrowserActor { .. }) || !baseline.is_edited_for(&plan.scope) {
        return None;
    }
    Some(RuntimeFrontierSummary { document_id: db_id.clone(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: baseline.chain_hash.0 })
}

pub fn wire_frontier_to_db(frontier: &mut RuntimeFrontierSummary, document_id: &str, db_id: &ProtocolArtifactId) -> bool {
    if frontier.document_id.0 != document_id {
        return false;
    }
    frontier.document_id = db_id.clone();
    true
}
//#endregion 🔖️Directory
