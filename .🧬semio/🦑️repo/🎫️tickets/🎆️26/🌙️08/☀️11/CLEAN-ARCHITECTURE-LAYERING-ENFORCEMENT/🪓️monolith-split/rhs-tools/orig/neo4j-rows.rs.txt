//! 🧰️ Neo4j directory rows: the Cypher row decoders and writers, the retry classification and the effect-commit guards every `Neo4jDirectory` operation is built from.

use semio_hub_directory_contract::model::{AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AuthAuditRecord, AuthSessionKind, AuthSessionRecord, CheckpointPublicationDispositionV1, CheckpointPublicationReceiptRecordV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptRecord, InviteRecord, NewAdminOperationAuditRecord, ShareTokenRecord, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord};
use semio_hub_artifact_authority::chunk_cas::{ArtifactCasObjectKey, ArtifactCasReservation, encode_artifact_cas_ownership_v1};
use semio_hub_artifact_creation::{ArtifactCreationActorV1, ArtifactCreationFactBodyV1, ArtifactCreationFactV1, ArtifactCreationIntentV1};
use directory::os_directory::{ArtifactHash, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DocumentScope, Hlc};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use neo4rs::{Txn, query};
use semio_hub_directory_contract::{decode_auth_digest_hex, directory_command_result_kind_from_str, validate_admin_operation_effect_receipt};
use directory::os_directory::io::binary::artifact_hash::hex_lower;

pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

pub fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

/// 🔒️ How many times a contended Neo4j write is re-attempted after the server aborts it as
/// transient. Forseti breaks a lock cycle by killing one participant, so a two-writer race needs one
/// retry and a wider fan-out needs a few; beyond this the contention is not transient and the caller
/// must see the fault.
const NEO4J_TRANSIENT_RETRY_MAX: u32 = 6;

/// ⏳️ First back-off step in milliseconds, doubled per attempt. The loser of a Forseti cycle must not
/// re-enter the same lock order immediately or the two writers simply rebuild the cycle.
const NEO4J_TRANSIENT_RETRY_BASE_MS: u64 = 10;

/// 🔁️ Whether a directory fault is one Neo4j classifies as transient — the server aborted an
/// otherwise-valid transaction to break a deadlock, serialization cycle or leader change, and rolled
/// it back whole. Unlike SQLite (`BEGIN IMMEDIATE` plus a busy handler) and PostgreSQL (`SELECT …
/// FOR UPDATE`), Bolt has no server-side wait: the retry is the client's half of the contract, so a
/// backend that surfaces `Neo.TransientError.*` to its caller is reporting a failure the database
/// never meant as one.
fn is_neo4j_transient(error: &DirectoryError) -> bool {
    matches!(error, DirectoryError::Backend(message) if message.contains("Neo.TransientError."))
}

/// 😴️ Consumes one retry budget unit for a transient fault and sleeps the back-off, answering
/// whether the caller should attempt the write again.
pub async fn neo4j_transient_retry_pause(error: &DirectoryError, attempt: &mut u32) -> bool {
    if !is_neo4j_transient(error) || *attempt >= NEO4J_TRANSIENT_RETRY_MAX {
        return false;
    }
    *attempt += 1;
    tokio::time::sleep(std::time::Duration::from_millis(NEO4J_TRANSIENT_RETRY_BASE_MS << (*attempt - 1))).await;
    true
}

pub fn artifact_creation_request_key(actor_user_id: &str, request_id: &str) -> String {
    format!("v1:{}:{}:{}{}", actor_user_id.len(), request_id.len(), actor_user_id, request_id)
}

fn artifact_creation_phase(body: &ArtifactCreationFactBodyV1) -> &'static str {
    match body {
        ArtifactCreationFactBodyV1::Accepted { .. } => "accepted",
        ArtifactCreationFactBodyV1::Prepared { .. } => "prepared",
        ArtifactCreationFactBodyV1::Committed { .. } => "committed",
        ArtifactCreationFactBodyV1::Cancelled => "cancelled",
        ArtifactCreationFactBodyV1::Failed => "failed",
    }
}

pub async fn lock_artifact_creation_request(txn: &mut Txn, actor_user_id: &str, request_id: &str) -> DirectoryResult<String> {
    let key = artifact_creation_request_key(actor_user_id, request_id);
    let mut result = txn
        .execute(
            query("MERGE (r:ArtifactCreationRequest {key: $key}) ON CREATE SET r.actorUserId = $actor_user_id, r.requestId = $request_id, r.lockNonce = 0 SET r.lockNonce = r.lockNonce + 1 RETURN r.key AS key")
                .param("key", key.clone())
                .param("actor_user_id", actor_user_id)
                .param("request_id", request_id),
        )
        .await
        .map_err(backend)?;
    result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact creation request lock returned no row".into()))?;
    drop(result);
    Ok(key)
}

pub async fn artifact_creation_facts(txn: &mut Txn, request_key: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
    let mut result = txn.execute(query("MATCH (:ArtifactCreationRequest {key: $key})-[:HAS_FACT]->(f:ArtifactCreationFact) RETURN f.payload AS payload ORDER BY f.revision LIMIT 4").param("key", request_key)).await.map_err(backend)?;
    let mut facts = Vec::new();
    while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
        facts.push(semio_framework_pack_json::from_json_str(&row.get::<String>("payload").map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?);
    }
    Ok(facts)
}

pub async fn validate_artifact_creation_authority(txn: &mut Txn, actor: &ArtifactCreationActorV1, space_id: &str, observed_now_override: Option<u64>) -> DirectoryResult<u64> {
    let mut result = txn
        .execute(
            query(
                "MATCH (a:AuthSession {id: $session_id})-[:BELONGS_TO]->(u:User {id: $user_id}), (s:Space {id: $space_id})
         OPTIONAL MATCH (u)-[m:MEMBER_OF]->(s)
         SET u.creationLockNonce = coalesce(u.creationLockNonce, 0) + 1, a.creationLockNonce = coalesce(a.creationLockNonce, 0) + 1,
             s.creationLockNonce = coalesce(s.creationLockNonce, 0) + 1
         FOREACH (_ IN CASE WHEN m IS NULL THEN [] ELSE [1] END | SET m.creationLockNonce = coalesce(m.creationLockNonce, 0) + 1)
         RETURN a.authorizationGeneration AS generation, a.revokedAt AS revokedAt, a.expiresAt AS expiresAt, s.kind AS spaceKind, m.role AS role",
            )
            .param("session_id", actor.session_id.clone())
            .param("user_id", actor.user_id.clone())
            .param("space_id", space_id),
        )
        .await
        .map_err(backend)?;
    let row = result.next(txn.handle()).await.map_err(backend)?.ok_or(DirectoryError::Unauthorized)?;
    let generation: i64 = row.get("generation").map_err(backend)?;
    let revoked: Option<i64> = row.get("revokedAt").map_err(backend)?;
    let expires_at: i64 = row.get("expiresAt").map_err(backend)?;
    let space_kind: String = row.get("spaceKind").map_err(backend)?;
    let role: Option<String> = row.get("role").map_err(backend)?;
    drop(result);
    let observed_now = observed_now_override.unwrap_or(u64::try_from(now_ms()).map_err(backend)?);
    if generation != i64::try_from(actor.authorization_generation).map_err(backend)? || revoked.is_some() || space_kind == "archive" || role.as_deref() != Some("author") || expires_at <= i64::try_from(observed_now).map_err(backend)? {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(observed_now)
}

pub async fn insert_artifact_creation_fact(txn: &mut Txn, request_key: &str, intent: &ArtifactCreationIntentV1, fact: &ArtifactCreationFactV1) -> DirectoryResult<()> {
    let payload = semio_framework_pack_json::to_json_string(fact);
    if payload.len() > 8 * 1024 * 1024 {
        return Err(DirectoryError::Conflict("artifact creation fact exceeds its bounded envelope".into()));
    }
    let revision = i64::try_from(fact.revision).map_err(backend)?;
    txn.run(query(
        "MATCH (r:ArtifactCreationRequest {key: $request_key})
         CREATE (f:ArtifactCreationFact {requestRevisionKey: $request_revision_key, actorUserId: $actor_user_id, requestId: $request_id, revision: $revision, phase: $phase, spaceId: $space_id, documentId: $document_id, deadlineMs: $deadline_ms, recordedAtMs: $recorded_at_ms, payload: $payload})
         CREATE (r)-[:HAS_FACT]->(f)")
        .param("request_key", request_key)
        .param("request_revision_key", format!("{request_key}:{revision}"))
        .param("actor_user_id", fact.actor_user_id.clone())
        .param("request_id", fact.request_id.clone())
        .param("revision", revision)
        .param("phase", artifact_creation_phase(&fact.body))
        .param("space_id", intent.scope.space_id.clone())
        .param("document_id", intent.scope.document_id.clone())
        .param("deadline_ms", i64::try_from(intent.deadline_ms).map_err(backend)?)
        .param("recorded_at_ms", i64::try_from(fact.recorded_at_ms).map_err(backend)?)
        .param("payload", payload)).await.map_err(backend)?;
    Ok(())
}

enum AdminEffectPreflightFailure {
    RejectedBeforeCommit,
    Indeterminate,
}

impl<T> From<AdminEffectPreflightFailure> for AdminEffectCommitV1<T> {
    fn from(value: AdminEffectPreflightFailure) -> Self {
        match value {
            AdminEffectPreflightFailure::RejectedBeforeCommit => Self::RejectedBeforeCommit,
            AdminEffectPreflightFailure::Indeterminate => Self::Indeterminate,
        }
    }
}

pub fn admin_effect_preflight<T>(result: DirectoryResult<T>) -> Result<T, AdminEffectPreflightFailure> {
    match result {
        Ok(value) => Ok(value),
        Err(DirectoryError::Backend(_)) => Err(AdminEffectPreflightFailure::Indeterminate),
        Err(_) => Err(AdminEffectPreflightFailure::RejectedBeforeCommit),
    }
}

pub async fn admin_effect_rollback<T>(txn: Txn) -> AdminEffectCommitV1<T> {
    match txn.rollback().await {
        Ok(()) => AdminEffectCommitV1::RejectedBeforeCommit,
        Err(_) => AdminEffectCommitV1::Indeterminate,
    }
}

pub async fn insert_auth_audit(txn: &mut Txn, event: &AuthAuditRecord) -> DirectoryResult<()> {
    txn.run(
        query("CREATE (:AuthAudit {id: $id, occurredAt: $occurred_at, eventKind: $event_kind, authSessionId: $auth_session_id, targetUserId: $target_user_id, actorUserId: $actor_user_id, provider: $provider, outcomeCode: $outcome_code, reasonCode: $reason_code, correlationId: $correlation_id, peerClass: $peer_class})")
            .param("id", event.id.clone())
            .param("occurred_at", event.occurred_at)
            .param("event_kind", event.event_kind.clone())
            .param("auth_session_id", event.auth_session_id.clone().unwrap_or_default())
            .param("target_user_id", event.target_user_id.clone().unwrap_or_default())
            .param("actor_user_id", event.actor_user_id.clone().unwrap_or_default())
            .param("provider", event.provider.clone().unwrap_or_default())
            .param("outcome_code", event.outcome_code.clone())
            .param("reason_code", event.reason_code.clone().unwrap_or_default())
            .param("correlation_id", event.correlation_id.clone())
            .param("peer_class", event.peer_class.clone()),
    )
    .await
    .map_err(backend)?;
    Ok(())
}

pub async fn insert_admin_operation_effect_receipt(txn: &mut Txn, receipt: &AdminOperationEffectReceiptV1) -> DirectoryResult<()> {
    validate_admin_operation_effect_receipt(receipt)?;
    let mut result = txn
        .execute(
            query(
                "MERGE (r:AdminOperationEffectReceipt {operationId: $operation_id})
                 ON CREATE SET r.intentDigest = $intent_digest, r.committedAt = $committed_at, r.outcomeCode = $outcome_code, r.eventSeqFirst = $event_seq_first, r.eventSeqLast = $event_seq_last
                 RETURN r AS r",
            )
            .param("operation_id", receipt.operation_id.clone())
            .param("intent_digest", receipt.intent_digest.clone())
            .param("committed_at", receipt.committed_at)
            .param("outcome_code", receipt.outcome_code.clone())
            .param("event_seq_first", receipt.event_seq_first.map(|value| i64::try_from(value).map_err(backend)).transpose()?.unwrap_or(0))
            .param("event_seq_last", receipt.event_seq_last.map(|value| i64::try_from(value).map_err(backend)).transpose()?.unwrap_or(0)),
        )
        .await
        .map_err(backend)?;
    let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("admin operation effect receipt disappeared".into()))?;
    let node: neo4rs::Node = row.get("r").map_err(backend)?;
    let established = AdminOperationEffectReceiptV1 {
        operation_id: node.get("operationId").map_err(backend)?,
        intent_digest: node.get("intentDigest").map_err(backend)?,
        committed_at: node.get("committedAt").map_err(backend)?,
        outcome_code: node.get("outcomeCode").map_err(backend)?,
        event_seq_first: match node.get::<i64>("eventSeqFirst").map_err(backend)? {
            0 => None,
            value => Some(u64::try_from(value).map_err(backend)?),
        },
        event_seq_last: match node.get::<i64>("eventSeqLast").map_err(backend)? {
            0 => None,
            value => Some(u64::try_from(value).map_err(backend)?),
        },
    };
    drop(result);
    if &established == receipt { Ok(()) } else { Err(DirectoryError::Conflict("admin operation effect receipt identity changed".into())) }
}

pub fn actor_kind_to_str(kind: DirectoryActorKind) -> &'static str {
    match kind {
        DirectoryActorKind::User => "user",
        DirectoryActorKind::Admin => "admin",
        DirectoryActorKind::System => "system",
    }
}

fn actor_kind_from_str(value: &str) -> DirectoryActorKind {
    match value {
        "admin" => DirectoryActorKind::Admin,
        "system" => DirectoryActorKind::System,
        _ => DirectoryActorKind::User,
    }
}

pub fn document_scope_key_v1(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
}

pub fn checkpoint_key_v1(scope: &DocumentScope, checkpoint_id: ArtifactHash) -> String {
    let scope_key = document_scope_key_v1(scope);
    format!("v1:{}:{}{}", scope_key.len(), scope_key, hex_lower(&checkpoint_id.0))
}

pub fn cas_object_token(key: &ArtifactCasObjectKey) -> String {
    format!("{}:{}", key.kind.name(), hex_lower(&key.digest.0))
}

pub async fn cas_generation(txn: &mut Txn) -> DirectoryResult<i64> {
    let mut result = txn.execute(query("MERGE (h:ArtifactCasLedgerHead {id: 'singleton'}) ON CREATE SET h.generation = 0 SET h.generation = h.generation + 1 RETURN h.generation AS generation")).await.map_err(backend)?;
    let generation = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS ledger counter returned no row".into()))?.get("generation").map_err(backend)?;
    drop(result);
    Ok(generation)
}

pub async fn cas_lock_space(txn: &mut Txn, space_id: &str) -> DirectoryResult<i64> {
    let mut result = txn
        .execute(query("MERGE (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) ON CREATE SET b.lockNonce = 0 SET b.lockNonce = b.lockNonce + 1 RETURN coalesce(b.leaseExpiresAtMs, 0) AS leaseExpiresAtMs").param("space_id", space_id))
        .await
        .map_err(backend)?;
    let expires_at_ms = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS space barrier returned no row".into()))?.get("leaseExpiresAtMs").map_err(backend)?;
    drop(result);
    Ok(expires_at_ms)
}

pub async fn cas_reservation_barrier(txn: &mut Txn, space_id: &str, now_ms: i64) -> DirectoryResult<([u8; 32], u64)> {
    let lease_expires_at_ms = cas_lock_space(txn, space_id).await?;
    if lease_expires_at_ms > now_ms {
        return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
    }
    let mut epoch_result = txn
        .execute(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) SET b.fenceEpoch = coalesce(b.fenceEpoch, 0) + 1 REMOVE b.leaseToken, b.leaseExpiresAtMs RETURN b.fenceEpoch AS epoch").param("space_id", space_id))
        .await
        .map_err(backend)?;
    let epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier epoch returned no row".into()))?.get("epoch").map_err(backend)?;
    drop(epoch_result);
    let mut identity = txn.execute(query("MATCH (b:ArtifactCasBarrierIdentity {id: 'singleton'}) RETURN b.coordinatorId AS coordinator")).await.map_err(backend)?;
    let row = identity.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier coordinator identity is missing".into()))?;
    let coordinator: neo4rs::BoltBytes = row.get("coordinator").map_err(backend)?;
    drop(identity);
    Ok((coordinator.value.to_vec().try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?, u64::try_from(epoch).map_err(backend)?))
}

pub async fn cas_project_reserve(txn: &mut Txn, reservation: &ArtifactCasReservation) -> DirectoryResult<()> {
    let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
    txn.run(query("MERGE (r:ArtifactCasReservation {scopeCheckpointKey: $key}) SET r.spaceId = $space_id, r.documentId = $document_id, r.checkpointId = $checkpoint_id, r.generation = $generation, r.writeEpoch = $write_epoch, r.expiresAtMs = $expires_at, r.plan = $plan, r.objects = $objects")
        .param("key", scope_key).param("space_id", reservation.plan.scope.space_id.clone()).param("document_id", reservation.plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0)).param("generation", i64::try_from(reservation.generation).map_err(backend)?).param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?).param("expires_at", i64::try_from(reservation.expires_at_ms).map_err(backend)?).param("plan", encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?).param("objects", reservation.plan.objects.iter().map(cas_object_token).collect::<Vec<_>>())).await.map_err(backend)?;
    Ok(())
}

pub async fn cas_project_publish(txn: &mut Txn, reservation: &ArtifactCasReservation, generation: i64) -> DirectoryResult<()> {
    let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
    txn.run(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) DELETE r").param("key", scope_key.clone())).await.map_err(backend)?;
    txn.run(
        query("CREATE (r:ArtifactCasReference {scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, generation: $generation, writeEpoch: $write_epoch, plan: $plan, objects: $objects})")
            .param("key", scope_key)
            .param("space_id", reservation.plan.scope.space_id.clone())
            .param("document_id", reservation.plan.scope.document_id.clone())
            .param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0))
            .param("generation", generation)
            .param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?)
            .param("plan", encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?)
            .param("objects", reservation.plan.objects.iter().map(cas_object_token).collect::<Vec<_>>()),
    )
    .await
    .map_err(backend)?;
    Ok(())
}

pub async fn cas_project_release(txn: &mut Txn, operation: &str, space_id: &str, scope: Option<&DocumentScope>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
    match operation {
        "retention" => {
            let scope = scope.ok_or_else(|| DirectoryError::Backend("artifact CAS retention scope missing".into()))?;
            let floor_key = checkpoint_key_v1(scope, checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?);
            txn.run(query("MATCH (floor:ArtifactAuthorityEvent {scopeCheckpointKey: $floor_key}) MATCH (r:ArtifactCasReference {spaceId: $space_id, documentId: $document_id}) MATCH (published:ArtifactAuthorityEvent {scopeCheckpointKey: r.scopeCheckpointKey}) WHERE published.eventSeq < floor.eventSeq DETACH DELETE r")
                .param("floor_key", floor_key.clone()).param("space_id", space_id).param("document_id", scope.document_id.clone())).await.map_err(backend)?;
            txn.run(
                query("MATCH (floor:ArtifactAuthorityEvent {scopeCheckpointKey: $floor_key}) MATCH (p:ArtifactCheckpointPrivate {spaceId: $space_id, documentId: $document_id}) WHERE p.eventSeq < floor.eventSeq DETACH DELETE p")
                    .param("floor_key", floor_key)
                    .param("space_id", space_id)
                    .param("document_id", scope.document_id.clone()),
            )
            .await
            .map_err(backend)?;
        }
        "space-delete" => {
            txn.run(query("MATCH (r:ArtifactCasReservation {spaceId: $space_id}) DETACH DELETE r").param("space_id", space_id)).await.map_err(backend)?;
            txn.run(query("MATCH (r:ArtifactCasReference {spaceId: $space_id}) DETACH DELETE r").param("space_id", space_id)).await.map_err(backend)?;
        }
        _ => return Err(DirectoryError::Backend("invalid artifact CAS release operation".into())),
    }
    Ok(())
}

pub const CONSTRAINTS: &[&str] = &[
    "CREATE CONSTRAINT IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:Space) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (d:DocumentDescriptor) REQUIRE d.scopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:DocumentIndex) REQUIRE i.scopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (c:ArtifactCheckpoint) REQUIRE c.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactRetention) REQUIRE r.scopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (p:ArtifactCheckpointPrivate) REQUIRE p.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:ArtifactAuthorityEvent) REQUIRE a.eventSeq IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:ArtifactAuthorityEvent) REQUIRE a.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:ArtifactCasLedgerEvent) REQUIRE e.generation IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCasReservation) REQUIRE r.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCasReference) REQUIRE r.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (b:ArtifactCasSpaceBarrier) REQUIRE b.spaceId IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (b:ArtifactCasBarrierIdentity) REQUIRE b.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (g:ShareGrant) REQUIRE g.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (g:ShareGrant) REQUIRE g.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:SyncSession) REQUIRE s.id IS UNIQUE",
    "CREATE INDEX sync_session_space IF NOT EXISTS FOR (s:SyncSession) ON (s.spaceId)",
    "CREATE INDEX directory_event_space IF NOT EXISTS FOR (e:DirectoryEvent) ON (e.spaceId)",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:DirectoryCommandReceipt) REQUIRE r.key IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:CheckpointPublicationReceipt) REQUIRE r.key IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthAudit) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AdminOperationAudit) REQUIRE a.sequence IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AdminOperationAudit) REQUIRE a.requestTerminalKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:AdminOperationEffectReceipt) REQUIRE r.operationId IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCreationRequest) REQUIRE r.key IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCreationRequest) REQUIRE r.documentScopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (f:ArtifactCreationFact) REQUIRE f.requestRevisionKey IS UNIQUE",
    "CREATE INDEX IF NOT EXISTS FOR (f:ArtifactCreationFact) ON (f.phase, f.deadlineMs)",
    "CREATE INDEX IF NOT EXISTS FOR (a:AdminOperationAudit) ON (a.operationId)",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (c:AdminOperationAuditCounter) REQUIRE c.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.seq IS UNIQUE",
];

pub fn user_from_node(row: &neo4rs::Row) -> DirectoryResult<UserRecord> {
    let node: neo4rs::Node = row.get("u").map_err(backend)?;
    Ok(UserRecord {
        id: node.get("id").map_err(backend)?,
        email: node.get("email").map_err(backend)?,
        display_name: node.get("displayName").map_err(backend)?,
        password_hash: node.get::<String>("passwordHash").ok().filter(|s| !s.is_empty()),
        sso_subject: node.get::<String>("ssoSubject").ok().filter(|s| !s.is_empty()),
        sso_provider: node.get::<String>("ssoProvider").ok().filter(|s| !s.is_empty()),
        created_at: node.get("createdAt").map_err(backend)?,
    })
}

pub fn space_from_node(row: &neo4rs::Row) -> DirectoryResult<SpaceRecord> {
    let node: neo4rs::Node = row.get("s").map_err(backend)?;
    Ok(SpaceRecord {
        id: node.get("id").map_err(backend)?,
        name: node.get("name").map_err(backend)?,
        owner_user_id: node.get("ownerUserId").map_err(backend)?,
        created_at: node.get("createdAt").map_err(backend)?,
        kind: node.get("kind").map_err(backend)?,
        visibility: node.get("visibility").map_err(backend)?,
    })
}

pub fn invite_from_node(row: &neo4rs::Row) -> DirectoryResult<InviteRecord> {
    let node: neo4rs::Node = row.get("i").map_err(backend)?;
    let role: String = node.get("role").map_err(backend)?;
    Ok(InviteRecord {
        id: node.get("id").map_err(backend)?,
        selector: node.get("selector").map_err(backend)?,
        secret_digest: decode_auth_digest_hex(&node.get::<String>("secretDigest").map_err(backend)?)?,
        space_id: node.get("spaceId").map_err(backend)?,
        role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator),
        created_at: node.get("createdAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
        revoked_reason: node.get::<String>("revokedReason").ok().filter(|value| !value.is_empty()),
        accepted_at: node.get::<i64>("acceptedAt").ok(),
        accepted_event_id: node.get::<String>("acceptedEventId").ok().filter(|value| !value.is_empty()),
    })
}

pub fn share_from_node(row: &neo4rs::Row) -> DirectoryResult<ShareTokenRecord> {
    let node: neo4rs::Node = row.get("g").map_err(backend)?;
    Ok(ShareTokenRecord {
        id: node.get("id").map_err(backend)?,
        selector: node.get("selector").map_err(backend)?,
        secret_digest: decode_auth_digest_hex(&node.get::<String>("secretDigest").map_err(backend)?)?,
        scope: DocumentScope::new(node.get::<String>("spaceId").map_err(backend)?, node.get::<String>("documentId").map_err(backend)?),
        created_at: node.get("createdAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
        revoked_reason: node.get::<String>("revokedReason").ok().filter(|value| !value.is_empty()),
    })
}

pub fn auth_session_from_node(row: &neo4rs::Row) -> DirectoryResult<AuthSessionRecord> {
    let node: neo4rs::Node = row.get("a").map_err(backend)?;
    let session_kind: String = node.get("sessionKind").map_err(backend)?;
    Ok(AuthSessionRecord {
        id: node.get("id").map_err(backend)?,
        selector: node.get("selector").map_err(backend)?,
        secret_digest: decode_auth_digest_hex(&node.get::<String>("secretDigest").map_err(backend)?)?,
        user_id: row.get("userId").map_err(backend)?,
        identity_provider: node.get("identityProvider").map_err(backend)?,
        identity_subject_digest: decode_auth_digest_hex(&node.get::<String>("identitySubjectDigest").map_err(backend)?)?,
        issued_at: node.get("issuedAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
        revoked_reason: node.get::<String>("revokedReason").ok().filter(|value| !value.is_empty()),
        authorization_generation: u64::try_from(node.get::<i64>("authorizationGeneration").map_err(backend)?).map_err(backend)?,
        device_instance_id: node.get("deviceInstanceId").map_err(backend)?,
        session_kind: AuthSessionKind::parse(&session_kind).ok_or_else(|| DirectoryError::Backend("stored session kind is invalid".into()))?,
    })
}

pub fn auth_audit_from_node(row: &neo4rs::Row) -> DirectoryResult<AuthAuditRecord> {
    let node: neo4rs::Node = row.get("a").map_err(backend)?;
    let optional = |name| node.get::<String>(name).ok().filter(|value| !value.is_empty());
    Ok(AuthAuditRecord {
        id: node.get("id").map_err(backend)?,
        occurred_at: node.get("occurredAt").map_err(backend)?,
        event_kind: node.get("eventKind").map_err(backend)?,
        auth_session_id: optional("authSessionId"),
        target_user_id: optional("targetUserId"),
        actor_user_id: optional("actorUserId"),
        provider: optional("provider"),
        outcome_code: node.get("outcomeCode").map_err(backend)?,
        reason_code: optional("reasonCode"),
        correlation_id: node.get("correlationId").map_err(backend)?,
        peer_class: node.get("peerClass").map_err(backend)?,
    })
}

/// 🔑️ The one durable `(actor, request id)` idempotency key this backend indexes receipts by.
pub fn directory_command_receipt_key(actor_user_id: &str, request_id: &str) -> String {
    format!("{}:{}", hex_lower(actor_user_id.as_bytes()), request_id)
}

pub fn checkpoint_publication_receipt_key(actor_user_id: &str, correlation_id: &str) -> String {
    format!("{}:{}", hex_lower(actor_user_id.as_bytes()), correlation_id)
}

pub fn checkpoint_publication_receipt_from_node(row: &neo4rs::Row) -> DirectoryResult<CheckpointPublicationReceiptRecordV1> {
    let node: neo4rs::Node = row.get("r").map_err(backend)?;
    let disposition = match node.get::<String>("disposition").map_err(backend)?.as_str() {
        "pending" => CheckpointPublicationDispositionV1::Pending,
        "completed" => CheckpointPublicationDispositionV1::Completed,
        other => return Err(DirectoryError::Backend(format!("unknown checkpoint publication disposition '{other}'"))),
    };
    let checkpoint_id = node.get::<String>("checkpointId").ok().filter(|value| !value.is_empty()).map(|value| semio_hub_directory_contract::decode_auth_digest_hex(&value).map(ArtifactHash)).transpose()?;
    Ok(CheckpointPublicationReceiptRecordV1 {
        actor_user_id: node.get("actorUserId").map_err(backend)?,
        correlation_id: node.get("correlationId").map_err(backend)?,
        command_sha256: node.get("commandSha256").map_err(backend)?,
        disposition,
        checkpoint_id,
        claimed_at: node.get("claimedAt").map_err(backend)?,
        completed_at: node.get::<i64>("completedAt").ok().filter(|value| *value > 0),
    })
}

pub fn directory_command_receipt_from_node(row: &neo4rs::Row) -> DirectoryResult<DirectoryCommandReceiptRecord> {
    let node: neo4rs::Node = row.get("r").map_err(backend)?;
    let disposition_text: String = node.get("disposition").map_err(backend)?;
    let disposition = match disposition_text.as_str() {
        "pending" => DirectoryCommandDispositionV1::Pending,
        "completed" => DirectoryCommandDispositionV1::Completed,
        other => return Err(DirectoryError::Backend(format!("unknown command disposition '{other}'"))),
    };
    let result_kind_text: String = node.get("resultKind").map_err(backend)?;
    let completed_at = node.get::<i64>("completedAt").ok().filter(|value| *value > 0);
    Ok(DirectoryCommandReceiptRecord {
        actor_user_id: node.get("actorUserId").map_err(backend)?,
        request_id: node.get("requestId").map_err(backend)?,
        command_sha256: node.get("commandSha256").map_err(backend)?,
        result_kind: directory_command_result_kind_from_str(&result_kind_text)?,
        disposition,
        event_seq_first: node.get::<i64>("eventSeqFirst").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?,
        event_seq_last: node.get::<i64>("eventSeqLast").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?,
        receipt_sha256: node.get::<String>("receiptSha256").ok().filter(|value| !value.is_empty()),
        claimed_at: node.get("claimedAt").map_err(backend)?,
        completed_at,
    })
}

pub fn admin_operation_audit_from_node(row: &neo4rs::Row) -> DirectoryResult<AdminOperationAuditRecord> {
    let node: neo4rs::Node = row.get("a").map_err(backend)?;
    let event_seq_first = node.get::<i64>("eventSeqFirst").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
    let event_seq_last = node.get::<i64>("eventSeqLast").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
    Ok(AdminOperationAuditRecord {
        sequence: u64::try_from(node.get::<i64>("sequence").map_err(backend)?).map_err(backend)?,
        fact: NewAdminOperationAuditRecord {
            request_id: node.get("requestId").map_err(backend)?,
            intent_digest: node.get("intentDigest").map_err(backend)?,
            operation_id: node.get("operationId").map_err(backend)?,
            occurred_at: node.get("occurredAt").map_err(backend)?,
            phase: node.get("phase").map_err(backend)?,
            intent_kind: node.get("intentKind").map_err(backend)?,
            target_kind: node.get("targetKind").map_err(backend)?,
            target_id: node.get("targetId").map_err(backend)?,
            principal_user_id: node.get("principalUserId").map_err(backend)?,
            principal_session_id: node.get("principalSessionId").map_err(backend)?,
            principal_generation: u64::try_from(node.get::<i64>("principalGeneration").map_err(backend)?).map_err(backend)?,
            correlation_id: node.get("correlationId").map_err(backend)?,
            event_seq_first,
            event_seq_last,
            outcome_code: node.get("outcomeCode").map_err(backend)?,
            reason_code: node.get::<String>("reasonCode").ok().filter(|value| !value.is_empty()),
        },
    })
}

/// 🧭️ Shared by `list_sync_sessions_for_document` (caller already knows `document_id`) and
/// `list_active_sync_sessions` (reads `documentId` off the row itself, since it spans documents).
pub fn sync_session_from_row(row: &neo4rs::Row, document_id: &str) -> DirectoryResult<SyncSessionRecord> {
    let role: String = row.get("role").unwrap_or_default();
    Ok(SyncSessionRecord {
        id: row.get("id").map_err(backend)?,
        auth_session_id: row.get::<String>("authSessionId").ok().filter(|value| !value.is_empty()),
        authorization_generation: u64::try_from(row.get::<i64>("generation").unwrap_or(0)).unwrap_or(0),
        actor_id: row.get("actorId").unwrap_or_default(),
        space_id: row.get("spaceId").map_err(backend)?,
        document_id: document_id.to_string(),
        surface: row.get("surface").unwrap_or_default(),
        user_id: row.get::<String>("userId").ok(),
        authenticated_email: row.get::<String>("authenticatedEmail").ok().filter(|value| !value.is_empty()),
        space_role: SpaceRole::parse(&role),
        client_label: row.get("clientLabel").map_err(backend)?,
        connected_at: row.get("connectedAt").map_err(backend)?,
        disconnected_at: row.get::<i64>("disconnectedAt").ok(),
    })
}

pub fn event_from_node(row: &neo4rs::Row) -> DirectoryResult<DirectoryEvent> {
    let node: neo4rs::Node = row.get("e").map_err(backend)?;
    let payload: String = node.get("payload").map_err(backend)?;
    let body = DirectoryEventBody::from_value(directory::DslValue::from(serde_json::from_str::<serde_json::Value>(&payload).map_err(backend)?)).map_err(backend)?;
    let actor_kind: String = node.get("actorKind").map_err(backend)?;
    let seq = u64::try_from(node.get::<i64>("seq").map_err(backend)?).map_err(backend)?;
    let logical = u32::try_from(node.get::<i64>("hlcLogical").map_err(backend)?).map_err(backend)?;
    Ok(DirectoryEvent {
        seq,
        id: node.get("id").map_err(backend)?,
        hlc: Hlc { physical_ms: node.get("hlcPhysical").map_err(backend)?, logical },
        actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: node.get("actorId").map_err(backend)? },
        space_id: node.get::<String>("spaceId").ok(),
        user_id: node.get::<String>("userId").ok(),
        body,
        recorded_at_ms: node.get("recordedAt").map_err(backend)?,
    })
}
