#!/usr/bin/env python3
"""🧾 H14 14c: the document socket's frame deadline bounds a Commands batch's admission only; a batch handed to the engine
is always answered with its Ack (hub bootstrap). Idempotent, region-guarded edits; `--dry-run` reports only.
usage: python3 h14-ack-every-commit.py [--dry-run]"""
import sys

PATH = "/Users/ueli/Documents/semio/🌎️hub/🏗️bootstrap/🦀️.rs"
DRY = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
problems = []


def region(start_marker, end_marker="\n}\n"):
    start = text.index(start_marker)
    end = text.index(end_marker, start) + len(end_marker)
    return start, end


def replace_in(span, old, new, label, count=1):
    global text
    start, end = span
    body = text[start:end]
    if new in body and old not in body:
        return "done"
    found = body.count(old)
    if found != count:
        problems.append(f"{label}: expected {count} match(es), found {found}")
        return "problem"
    text = text[:start] + body.replace(old, new) + text[end:]
    return "replace"


states = []

# 1. TestLiveGate: commit-phase pause + frame deadline override.
states.append(replace_in(region("#[cfg(test)]\nstruct TestLiveGate {"), "    check_in_release: tokio::sync::Semaphore,\n}", "    check_in_release: tokio::sync::Semaphore,\n    socket_commit_pause_enabled: std::sync::atomic::AtomicBool,\n    socket_commit_admitted: tokio::sync::Semaphore,\n    socket_commit_release: tokio::sync::Semaphore,\n    socket_frame_deadline: Mutex<Option<std::time::Duration>>,\n}", "TestLiveGate fields"))
states.append(replace_in(region("impl Default for TestLiveGate {", "\n}\n"), "            check_in_release: tokio::sync::Semaphore::new(0),\n", "            check_in_release: tokio::sync::Semaphore::new(0),\n            socket_commit_pause_enabled: std::sync::atomic::AtomicBool::new(false),\n            socket_commit_admitted: tokio::sync::Semaphore::new(0),\n            socket_commit_release: tokio::sync::Semaphore::new(0),\n            socket_frame_deadline: Mutex::new(None),\n", "TestLiveGate default"))

# 2. Deadline doc + resolver.
states.append(replace_in(region("/// ⏱️ How long one document-socket frame", "\n\n"), """/// ⏱️ How long one document-socket frame — a command's admission, document write and durable commit
/// included — may take before its socket closes `1013 frame-deadline` and its client resynchronizes.
/// It bounds how long the frame holds its authority bindings (so how long a revocation of them can
/// wait), and it is far above a loaded host's commit latency: at 2 s a busy hub closed sockets whose
/// commands were merely waiting for their fsync, as `authorization-unavailable`, and never sent their Ack.
const DOCUMENT_SOCKET_FRAME_DEADLINE: std::time::Duration = std::time::Duration::from_secs(30);""", """/// ⏱️ How long one document-socket frame's admission may take — authority, the batch's own refusals, the
/// security gate and the document's write gate — before its socket closes `1013 frame-deadline` and its
/// client resynchronizes; nothing of such a frame reached the engine. A `Commands` batch the engine received
/// is never cut by it: its commit is awaited to the engine's answer and acknowledged, since a batch that
/// committed after a cut used to land durably with its socket closed and its Ack lost (ticket 26/09/23
/// session 14c, H13's transient probe: batch 24 committed at head 6144, no Ack).
const DOCUMENT_SOCKET_FRAME_DEADLINE: std::time::Duration = std::time::Duration::from_secs(30);

/// ⏱️ The frame deadline this socket applies: [`DOCUMENT_SOCKET_FRAME_DEADLINE`], or a law's shorter one.
fn document_socket_frame_deadline(state: &HubState) -> std::time::Duration {
    #[cfg(test)]
    if let Some(deadline) = state.live_gate.as_ref().and_then(|gate| *gate.socket_frame_deadline.lock().unwrap_or_else(std::sync::PoisonError::into_inner)) {
        return deadline;
    }
    let _ = state;
    DOCUMENT_SOCKET_FRAME_DEADLINE
}""", "deadline doc"))

# 3. handle_client_frame → admission only.
span = region("/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session.")
start, end = span
body = text[start:end]
if "ClientFrameStepV1::Commit(AdmittedCommandsV1" not in body:
    new = body
    new = new.replace("""/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session.
/// session. Returns `false` when the session should close (`Bye`, or a send failure).
""", """/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session within the frame
/// deadline: every frame but an admitted `Commands` batch completes here; an admitted batch leaves holding its
/// document's write gate as [`ClientFrameStepV1::Commit`] for [`commit_admitted_commands`].
""")
    new = new.replace(") -> bool {\n    match frame {", ") -> ClientFrameStepV1 {\n    match frame {")
    new = new.replace("return sender.send(encode(&ack, document_id).await).await.is_ok();", "return ClientFrameStepV1::after_send(sender.send(encode(&ack, document_id).await).await.is_ok());")
    new = new.replace("""            let _document_write = state.socket_binding_gates.document_write(&DocumentScope::new(space_id, document_id)).lock_owned().await;
            let (ack, relay) = submit_commands(handle, gate, actor, batch_id, envelopes, state.merge_policy).await;
            if let Some(commands_frame) = relay {
                let _ = fanout.send(commands_frame);
            }
            sender.send(encode(&ack, document_id).await).await.is_ok()
        }""", """            let document_write = state.socket_binding_gates.document_write(&DocumentScope::new(space_id, document_id)).lock_owned().await;
            ClientFrameStepV1::Commit(AdmittedCommandsV1 { batch_id, envelopes, _document_write: document_write })
        }""")
    new = new.replace("""                return sender.send(error_frame("frontier-document-mismatch", "advertised frontier names a different document than this socket").await).await.is_ok();""", """                return ClientFrameStepV1::after_send(sender.send(error_frame("frontier-document-mismatch", "advertised frontier names a different document than this socket").await).await.is_ok());""")
    new = new.replace("""                Ok(Some(catch_up)) => sender.send(encode(&catch_up, document_id).await).await.is_ok(),
                Ok(None) => true,
                Err(_) => true,""", """                Ok(Some(catch_up)) => ClientFrameStepV1::after_send(sender.send(encode(&catch_up, document_id).await).await.is_ok()),
                Ok(None) | Err(_) => ClientFrameStepV1::Continue,""")
    new = new.replace("""            let _ = fanout.send(ServerFrame::Preview { actor: actor.clone(), key: preview_key, seq, payload });
            true""", """            let _ = fanout.send(ServerFrame::Preview { actor: actor.clone(), key: preview_key, seq, payload });
            ClientFrameStepV1::Continue""")
    new = new.replace("""            let _ = state.refresh_document_presence(key, space_id, document_id, &actor.0, socket_live_id, peer, state.presence_now()).await;
            true""", """            let _ = state.refresh_document_presence(key, space_id, document_id, &actor.0, socket_live_id, peer, state.presence_now()).await;
            ClientFrameStepV1::Continue""")
    new = new.replace("""        ClientFrame::CreditGrant { .. } => true,
        ClientFrame::Bye => false,""", """        ClientFrame::CreditGrant { .. } => ClientFrameStepV1::Continue,
        ClientFrame::Bye => ClientFrameStepV1::End,""")
    new = new.replace("""            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            false
        }
    }
}""", """            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            ClientFrameStepV1::End
        }
    }
}

/// @emoji 🚦️ What one decoded frame leaves its socket to do once its admission finished within the frame deadline.
enum ClientFrameStepV1 {
    Continue,
    End,
    Commit(AdmittedCommandsV1),
}

impl ClientFrameStepV1 {
    /// 📤️ `Continue` when the frame's answer was sent, `End` when the socket is gone.
    fn after_send(sent: bool) -> Self {
        if sent { Self::Continue } else { Self::End }
    }
}

/// @emoji ✍️ One `Commands` batch admitted within the frame deadline, holding its document's write gate until its
/// commit is answered.
struct AdmittedCommandsV1 {
    batch_id: u64,
    envelopes: Vec<MutationEnvelope>,
    _document_write: tokio::sync::OwnedMutexGuard<()>,
}

/// @emoji 🧾️ Commits one admitted batch and answers it: the engine's receipt (or refusal) always reaches the socket as
/// the batch's `Ack` — never cut by the frame deadline, the engine's own bounds end the wait — and an advanced
/// frontier's relay reaches every peer. Returns `false` when the Ack could not be sent.
#[allow(clippy::too_many_arguments)]
async fn commit_admitted_commands(state: &HubState, handle: &db::ArtifactHandle, document_id: &str, fanout: &broadcast::Sender<ServerFrame>, actor: &ActorId, gate: &db::security::SecurityGate, admitted: AdmittedCommandsV1, sender: &mut SplitSink<WebSocket, Message>) -> bool {
    let AdmittedCommandsV1 { batch_id, envelopes, _document_write } = admitted;
    let (ack, relay) = submit_commands(handle, gate, actor, batch_id, envelopes, state.merge_policy).await;
    #[cfg(test)]
    if let Some(live_gate) = state.live_gate.as_ref().filter(|gate| gate.socket_commit_pause_enabled.load(std::sync::atomic::Ordering::Acquire)) {
        live_gate.socket_commit_admitted.add_permits(1);
        live_gate.socket_commit_release.acquire().await.expect("socket commit test release").forget();
    }
    drop(_document_write);
    if let Some(commands_frame) = relay {
        let _ = fanout.send(commands_frame);
    }
    sender.send(encode(&ack, document_id).await).await.is_ok()
}""")
    if new.count("ClientFrameStepV1::after_send(") < 6 or "-> bool {\n    match frame" in new or "\n            true\n" in new:
        problems.append("handle_client_frame: not every bool exit was converted")
    else:
        text = text[:start] + new + text[end:]
        states.append("replace")
else:
    states.append("done")

# 4. Socket loop: admission under the deadline, commit after it.
states.append(replace_in(region("async fn serve_document_socket("), """                            match tokio::time::timeout(
                                DOCUMENT_SOCKET_FRAME_DEADLINE,
                                handle_client_frame(&state, &handle, &db_id, &key, &space_id, &document_id, &fanout, &actor, &socket_live.id, &gate, &principal, &tenant, frame, sender),
                            )
                            .await
                            {
                                Ok(true) => {}
                                Ok(false) => break,
                                Err(_) => {""", """                            match tokio::time::timeout(
                                document_socket_frame_deadline(&state),
                                handle_client_frame(&state, &handle, &db_id, &key, &space_id, &document_id, &fanout, &actor, &socket_live.id, &gate, &principal, &tenant, frame, sender),
                            )
                            .await
                            {
                                Ok(ClientFrameStepV1::Continue) => {}
                                Ok(ClientFrameStepV1::End) => break,
                                Ok(ClientFrameStepV1::Commit(admitted)) => {
                                    if !commit_admitted_commands(&state, &handle, &document_id, &fanout, &actor, &gate, admitted, sender).await {
                                        break;
                                    }
                                }
                                Err(_) => {""", "socket loop"))

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if not DRY and "replace" in states:
    open(PATH, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("dry-run clean" if DRY else "nothing to apply")
