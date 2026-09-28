#!/usr/bin/env python3
"""🧾 H14 14c: bin-unit law — a batch the engine committed past its socket's frame deadline is still acknowledged.
Idempotent insertion after `socket_grant_revoke_before_command_admission_has_no_storage_effect`. usage: [--dry-run]"""
import sys

PATH = "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
NAME = "a_batch_committed_past_the_frame_deadline_is_still_acknowledged"
ANCHOR = "            assert_eq!(frontier.head_seq, 1, \"the revoked actor-matching command never reaches durable storage\");\n        });\n    }\n"
LAW = '''
    /// 🧾️ A `Commands` batch the engine committed after its socket's frame deadline elapsed is still acknowledged on that
    /// socket, its relay still reaches the socket, and the socket stays open for the next batch: the deadline bounds a
    /// frame's admission only (ticket 26/09/23 session 14c — H13's transient probe saw batch 24 commit at head 6144 while
    /// the deadline closed its socket `1013` and its Ack never came).
    #[test]
    fn a_batch_committed_past_the_frame_deadline_is_still_acknowledged() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let live_gate = Arc::new(TestLiveGate::default());
            *live_gate.socket_frame_deadline.lock().expect("frame deadline override") = Some(std::time::Duration::from_millis(200));
            live_gate.socket_commit_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-commit-deadline").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-commit-deadline".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-commit-deadline/document/ws");
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
            live_gate.document_release.add_permits(1);

            let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-commit-deadline"));
            let wire_document = WireArtifactId("socket-commit-deadline".into());
            for (batch_id, id, head) in [(70u64, "late-commit-op", 1u64), (71, "next-commit-op", 2)] {
                let mut envelope = sample_envelope(id, &wire_document).await;
                envelope.actor = ActorId(receipt.actor_id.clone());
                socket.send(client_binary(&ClientFrame::Commands { batch_id, envelopes: vec![envelope] }, Lane::Command).await).await.expect("command sent");
                tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary").forget();
                live_gate.socket_command_release.add_permits(1);
                tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_commit_admitted.acquire()).await.expect("commit boundary deadline").expect("commit boundary").forget();
                let committed = state.db.document(&document).await.expect("document handle").frontier().await.expect("committed frontier");
                assert_eq!(committed.head_seq, head, "the engine committed batch {batch_id} before its answer");
                tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                live_gate.socket_commit_release.add_permits(1);
                match next_server_frame_at(&mut socket, "Ack of a batch committed past the frame deadline").await {
                    ServerFrame::Ack { batch_id: acked, stages, .. } => {
                        assert_eq!(acked, batch_id);
                        assert!(stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted))), "batch {batch_id} is acknowledged as accepted: {stages:?}");
                    }
                    other => panic!("batch {batch_id} answered {other:?} instead of its Ack"),
                }
                assert!(matches!(next_server_frame_at(&mut socket, "relay").await, ServerFrame::Commands { .. }), "the committed batch's relay follows its Ack");
            }
        });
    }
'''
text = open(PATH, encoding="utf-8").read()
if f"fn {NAME}(" in text:
    print("done")
    sys.exit(0)
if text.count(ANCHOR) != 1:
    print(f"PROBLEM: anchor found {text.count(ANCHOR)} times")
    sys.exit(1)
if "--dry-run" in sys.argv:
    print("dry-run clean")
    sys.exit(0)
open(PATH, "w", encoding="utf-8").write(text.replace(ANCHOR, ANCHOR + LAW))
print("applied")
