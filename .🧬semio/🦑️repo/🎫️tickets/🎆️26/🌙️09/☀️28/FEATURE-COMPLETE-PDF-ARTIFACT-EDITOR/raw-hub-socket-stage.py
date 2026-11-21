"""📡 Stages authenticated Hub socket commit and atomic live fanout using the canonical WAL owner."""

import hashlib
import json
import runpy
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
SOCKET = Path("🌎️hub/🏗️bootstrap/🔌️document-socket/🦀️.rs")
MODELS = SOCKET.with_name("🧱️types") / "🦀️.rs"
CHECK_IN = SOCKET.parent.with_name("📌️check-in") / "🦀️.rs"
OUT = TICKET / "🗑️generated/raw-hub-socket-overlay.json"


def stage():
    lower = runpy.run_path(str(TICKET / "raw-hub-atomic-stage.py"))
    lower["stage"]()
    data = json.loads(lower["OUT"].read_text(encoding="utf8"))
    pre, post = data["preimages"], data["candidate"]

    def edit(path, anchor, replacement):
        raw = (ROOT / path).read_bytes()
        pre[str(path)] = hashlib.sha256(raw).hexdigest()
        current = post.get(str(path), raw.decode("utf8"))
        assert current.count(anchor) == 1, (path, anchor, current.count(anchor))
        post[str(path)] = current.replace(anchor, replacement)

    edit(MODELS, "    match handle.submit(batch, db::document::SubmitOptions", """    submit_admitted_batch(handle, gate, actor, batch_id, committed_before, batch, policy).await
}

/// 🧾️ Publishes one durable receipt, selected only after the native WAL frontier advances.
pub async fn submit_admitted_batch(handle: &db::ArtifactHandle, gate: &db::security::SecurityGate, actor: &ActorId, batch_id: u64, committed_before: Option<u64>, batch: db::document::CommandBatch, policy: protocol::MergePolicy) -> (ServerFrame, Option<ServerFrame>) {
    let group_id = batch.atomic_edit().map(|group| group.edit_id.clone());
    let envelopes = batch.envelopes.clone();
    match handle.submit(batch, db::document::SubmitOptions""")
    edit(MODELS, "engine_frontier_to_wire(&receipt.frontier, receipt.command_id.0.clone())",
         "engine_frontier_to_wire(&receipt.frontier, group_id.unwrap_or_else(|| receipt.command_id.0.clone()))")
    edit(MODELS, "pub async fn error_frame", """/// 🪢 Preserves packet ordering and the original edit identity on document fanout and hello replay.
pub async fn encode_document_fanout(frame: &ServerFrame, document_id: &str) -> Result<Vec<Message>, &'static str> {
    let ServerFrame::Commands { envelopes, .. } = frame else { return Ok(vec![encode(frame, document_id).await]); };
    if !envelopes.iter().any(|row| row.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool))) {
        return Ok(vec![encode(frame, document_id).await]);
    }
    let mut projected = frame.clone();
    project_server_frame(&mut projected, document_id);
    let ServerFrame::Commands { envelopes, frontier, .. } = projected else { unreachable!() };
    let group = protocol::atomic_hub::AtomicWalEdit { edit_id: frontier.head_edit_id.clone(), rows: envelopes.len() };
    let metadata = ServerFrame::Ack { batch_id: 0, stages: Vec::new(), frontier };
    let metadata = encode_server_frame(&metadata, Lane::Command).await;
    group.relay(&envelopes, &metadata).map(|frames| frames.into_iter().map(|frame| Message::Binary(frame.into())).collect())
}

pub async fn error_frame""")
    edit(SOCKET, "use crate::socket_grants::socket_live_authority;", """use crate::socket_grants::socket_live_authority;
use semio_hub_route_models::document_socket::{encode_document_fanout, submit_admitted_batch};
use protocol::atomic_hub::{AtomicSocketEdit, client_atomic_part};

const ATOMIC_SOCKET_PERSISTENCE_ENABLED: bool = false;""")
    edit(SOCKET, "    loop {\n        tokio::select! {", "    let mut atomic_ingress = AtomicSocketEdit::default();\n    loop {\n        tokio::select! {")
    edit(SOCKET, """                        let Ok((_lane, frame)) = decode_client_frame(&bytes).await else {""", """                        let atomic = match client_atomic_part(&bytes) {
                            Ok(None) => None,
                            Ok(Some((batch_id, part))) if ATOMIC_SOCKET_PERSISTENCE_ENABLED => Some((batch_id, part)),
                            _ => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "invalid-atomic-document-packet".into() }))).await;
                                break;
                            }
                        };
                        if let Some((batch_id, physical)) = atomic {
                            let page_authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                                Ok(admission) => admission,
                                Err(_) => break,
                            };
                            let group = match atomic_ingress.push(batch_id, physical, &document_id, &actor.0) {
                                Ok(None) => continue,
                                Ok(Some(group)) => group,
                                Err(_) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "invalid-atomic-document-group".into() }))).await;
                                    break;
                                }
                            };
                            drop(page_authority);
                            if let Some(subject) = agent_command_subject {
                                let paced = state.rate_limits.admit_paced(RateLimitClassV1::AgentCommand, &[subject], AGENT_COMMAND_PATIENCE_MS, |ms| tokio::time::sleep(std::time::Duration::from_millis(ms))).await;
                                if let RateLimitDecisionV1::Refused { retry_after_ms } = paced {
                                    let reason = format!("rate-limited: retry after {retry_after_ms} ms");
                                    let messages = transient_apply_refusal_messages(&reason);
                                    let frontier = best_effort_frontier(&handle).await;
                                    let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason, messages }) }], frontier };
                                    if sender.send(encode(&ack, &document_id).await).await.is_err() { break; }
                                    continue;
                                }
                            }
                            let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                                Ok(admission) => admission,
                                Err(_) => break,
                            };
                            let batch = match db::document::CommandBatch::from_verified_atomic(group, &db_id) {
                                Ok(batch) => batch,
                                Err(_) => break,
                            };
                            let admitted = tokio::time::timeout(document_socket_frame_deadline(&state), async {
                                admit_writes(&gate, &principal, &tenant, &db_id, &batch.envelopes, now_ms().max(0) as u64).await?;
                                Ok::<_, db::DbError>(state.socket_binding_gates.document_write(&scope).lock_owned().await)
                            }).await;
                            let document_write = match admitted {
                                Ok(Ok(guard)) => guard,
                                Ok(Err(error)) => {
                                    let frontier = best_effort_frontier(&handle).await;
                                    let outcome = ApplyOutcome::Rejected { reason: error.to_string(), messages: messages_for_error(&error) };
                                    let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(outcome) }], frontier };
                                    if sender.send(encode(&ack, &document_id).await).await.is_err() { break; }
                                    continue;
                                }
                                Err(_) => break,
                            };
                            let prior = handle.frontier().await.map(|frontier| frontier.commit_seq).ok();
                            let (ack, relay) = submit_admitted_batch(&handle, &gate, &actor, batch_id, prior, batch, state.merge_policy).await;
                            if let Some(frame) = relay {
                                if let ServerFrame::Commands { envelopes, .. } = &frame {
                                    observe_checkpoint_policy(&state, &scope, &socket_grant.subject, &handle, envelopes);
                                }
                                let _ = fanout.send(frame);
                            }
                            drop(document_write);
                            if sender.send(encode(&ack, &document_id).await).await.is_err() { break; }
                            continue;
                        }
                        let Ok((_lane, frame)) = decode_client_frame(&bytes).await else {""")
    edit(SOCKET, "                        let frame = encode(&frame, &document_id).await;", """                        let Ok(frames) = encode_document_fanout(&frame, &document_id).await else { break; };""")
    edit(SOCKET, """            match db::sync::handle_frontier_advertise(&state.db.storage().await.wal().await, core_document, &frontier, ActorId(HUB_CATCH_UP_ORIGIN.into())).await {
                Ok(Some(catch_up)) => ClientFrameStepV1::after_send(sender.send(encode(&catch_up, document_id).await).await.is_ok()),
                Ok(None) | Err(_) => ClientFrameStepV1::Continue,
            }""", """            if !ATOMIC_SOCKET_PERSISTENCE_ENABLED {
                return match db::sync::handle_frontier_advertise(&state.db.storage().await.wal().await, core_document, &frontier, ActorId(HUB_CATCH_UP_ORIGIN.into())).await {
                    Ok(Some(catch_up)) => ClientFrameStepV1::after_send(sender.send(encode(&catch_up, document_id).await).await.is_ok()),
                    Ok(None) | Err(_) => ClientFrameStepV1::Continue,
                };
            }
            let wal = state.db.storage().await.wal().await;
            let Ok(replayed) = db::sync::replay_sync_state(&wal, core_document).await else { return ClientFrameStepV1::End; };
            let Ok(tail) = db::sync::grouped_frontier_frames(&replayed, &db::sync::from_frontier_summary(&frontier), ActorId(HUB_CATCH_UP_ORIGIN.into())).await else { return ClientFrameStepV1::End; };
            for batch in tail {
                let Ok(packets) = encode_document_fanout(&batch, document_id).await else { return ClientFrameStepV1::End; };
                for packet in packets {
                    if sender.send(packet).await.is_err() { return ClientFrameStepV1::End; }
                }
            }
            ClientFrameStepV1::Continue""")
    edit(SOCKET, """                        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame)).await, Ok(Ok(()))) {
                            break;
                        }""", """                        let send = async {
                            for frame in frames { sender.send(frame).await?; }
                            Ok::<_, axum::Error>(())
                        };
                        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), send).await, Ok(Ok(()))) { break; }""")
    catchup = runpy.run_path(str(TICKET / "raw-hub-atomic-catchup.py"))
    catchup["apply"](ROOT, pre, post)
    edit(CHECK_IN, "    let envelopes: Vec<MutationEnvelope> = commits.into_iter().flat_map(|commit| commit.envelopes).collect();",
         """    let envelopes = if commits.iter().any(|commit| commit.original_edit.is_some()) {
        let mut commits = commits;
        for commit in &mut commits {
            for row in &mut commit.envelopes {
                row.document_id = protocol::ArtifactId(scope.document_id.as_str().to_owned());
            }
        }
        directory::os_spr::atomic_hub::encode_checkpoint_replay(
            commits.iter().map(|commit| (commit.original_edit.as_ref(), commit.envelopes.as_slice()))
        ).map_err(|error| CheckInEndV1 { refusal: Some(DocumentCheckInRefusalV1::CodecRefused), cause: Some(error.into()) })?
    } else {
        let rows: Vec<MutationEnvelope> = commits.into_iter().flat_map(|commit| commit.envelopes).collect();
        directory::os_spr::encode_envelopes(&rows)
    };""")
    edit(CHECK_IN, "envelopes: directory::os_spr::encode_envelopes(&envelopes) },",
         "envelopes },")
    edit(SOCKET, "                    Ok(owner) => encode(owner, &document_id).await,", """                    Ok(owner) => match encode_document_fanout(owner, &document_id).await {
                        Ok(frames) => frames,
                        Err(error) => {
                            let _ = sender.send(error_frame("storage", error).await).await;
                            hello_session.cancel();
                            state.release_color(&space_id, &actor.0);
                            return;
                        }
                    },""")
    edit(SOCKET, "                let sent = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame_bytes)).await;", """                let send = async {
                    for bytes in frame_bytes { sender.send(bytes).await?; }
                    Ok::<_, axum::Error>(())
                };
                let sent = tokio::time::timeout(std::time::Duration::from_secs(2), send).await;""")
    OUT.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} authenticated Hub group and socket postimages")


if __name__ == "__main__":
    stage()
