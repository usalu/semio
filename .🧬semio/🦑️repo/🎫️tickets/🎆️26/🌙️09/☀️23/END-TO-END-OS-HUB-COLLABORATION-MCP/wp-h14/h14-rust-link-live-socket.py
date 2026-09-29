#!/usr/bin/env python3
"""🔌 H14 14c (coordinator 17:1x: "make sure the wgpu shell's Rust document link handles the stale/re-plan answer the
same way the worker does"): findings on the Rust document link (kernel `🏪️store/🔄️sync`, GUEST-LINKED → a prepared set for
an L1 train, never landed by H14 during the guest freeze).
(1) A `409 socket-grant-stale` at the socket upgrade is already transient: the native actor maps every
`connect_async` failure to `DocumentConnectFailure::Short` and every attempt runs a fresh open-plan exchange; the
browser (wasm) actor redials on any close. A live document socket's `4401` was never terminal there either (the native
actor ignores the Close frame and reconnects when the stream ends; the wasm actor redials on `Closed(_)`); the terminal
`4401` is the scoped DIRECTORY stream's membership revocation, which stays.
(2) The real defect is the Rust twin of the React worker's 09-22 fix: both Rust actors bound a LIVE socket by the plan's
`expires_at_unix_ms` — the ≤ 30 s window in which the one-shot plan receipt may be exchanged (`DOCUMENT_OPEN_PLAN_MAX_TTL_MS`),
not a lifetime. The native actor armed `socket_authority_deadline` at admission and closed + requeued + re-planned the
socket at it (on the next frame, send or drive turn); the wasm actor's `pump_socket` disconnected once it passed. So
every wgpu/native document link dropped and reconnected ~30 s after each admission, for ever. The hub re-proves a live
socket's authority every second, so the client-side bound adds nothing but the break.
Fix: the plan window bounds the admission only (both actors still refuse an authority that expired before the socket was
dialled or finished connecting); `socket_authority_deadline`, `invalidate_socket_authority` and the test hook go; the
native law's expiry tail is dropped and a mock-hub law pins the contract: with a 3 s plan window, an edit sent 3.5 s
after Session goes out on the SAME socket and is accepted. Idempotent, region-guarded; `--dry-run` reports."""
import sys

R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync"
SRC = f"{R}/🦀️.rs"
TESTS = f"{R}/🧪️tests/🔬️unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {path: open(path, encoding="utf-8").read() for path in (SRC, TESTS)}
problems, states = [], []


def edit(path, old, new, label):
    text = files[path]
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit(SRC, """        socket_actor_confirmed: bool,
        socket_authority: Option<crate::os_directory::client::DocumentSocketAuthorityV1>,
        socket_authority_deadline: Option<Instant>,
""", """        socket_actor_confirmed: bool,
""", "native fields")

edit(SRC, """                socket_actor_confirmed: false,
                socket_authority: None,
                socket_authority_deadline: None,
                session_color: None,
                semio_hub: None,""", """                socket_actor_confirmed: false,
                session_color: None,
                semio_hub: None,""", "native init")

edit(SRC, """                    self.tick_link().await;
                    if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                        self.invalidate_socket_authority().await;
                        return ArtifactDrive::MoreWork;
                    }
                    self.emit_status_if_changed().await;""", """                    self.tick_link().await;
                    self.emit_status_if_changed().await;""", "status turn")

edit(SRC, """                    return ArtifactDrive::Idle { deadline: [self.reconnect_at, self.link_expires_at, self.fs_deadline, self.socket_authority_deadline].into_iter().flatten().min() };""",
     """                    return ArtifactDrive::Idle { deadline: [self.reconnect_at, self.link_expires_at, self.fs_deadline].into_iter().flatten().min() };""", "idle deadline")

edit(SRC, """            self.socket_actor_confirmed = false;
            self.socket_authority = None;
            self.socket_authority_deadline = None;
            self.session_color = None;
        }

        async fn invalidate_socket_authority(&mut self) {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            if let Some(mut connection) = self.semio_hub.take() {
                let _ = tokio::time::timeout(Duration::from_millis(4), connection.write.close()).await;
            }
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
        }
""", """            self.socket_actor_confirmed = false;
            self.session_color = None;
        }
""", "native clear + invalidate")

edit(SRC, """                    self.hub_surface = Some(authority.surface.surface_id.clone());
                    self.socket_authority_deadline = Some(Instant::now() + Duration::from_millis(authority.expires_at_unix_ms.saturating_sub(now)));
                    self.socket_authority = Some(authority);
""", """                    self.hub_surface = Some(authority.surface.surface_id.clone());
""", "native arm")

edit(SRC, """        socket_actor_confirmed: bool,
        socket_authority: Option<crate::os_directory::client::DocumentSocketAuthorityV1>,
        /// 🎨️ See the native actor's matching field — same role, browser side.""", """        socket_actor_confirmed: bool,
        /// 🎨️ See the native actor's matching field — same role, browser side.""", "wasm field")

edit(SRC, """            self.hello = Some(document_socket_hello(&self.schema, pack_schema_hash, self.resume_token.clone(), self.server_frontier.clone()));
            self.socket_authority = Some(authority);
            self.session_color = None;""", """            self.hello = Some(document_socket_hello(&self.schema, pack_schema_hash, self.resume_token.clone(), self.server_frontier.clone()));
            self.session_color = None;""", "wasm arm")

edit(SRC, """            self.socket_actor_confirmed = false;
            self.socket_authority = None;
            self.session_color = None;
            self.hello = None;""", """            self.socket_actor_confirmed = false;
            self.session_color = None;
            self.hello = None;""", "wasm clear")

edit(SRC, """            socket_actor_confirmed: false,
            socket_authority: None,
            session_color: None,
            server_frontier: hub.seed""", """            socket_actor_confirmed: false,
            session_color: None,
            server_frontier: hub.seed""", "wasm init")

edit(SRC, """        async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
            if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                self.invalidate_socket_authority().await;
                return;
            }
            match message {""", """        async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
            match message {""", "receive")

edit(SRC, """            note_authored_envelopes(&mut self.applied_op_ids, envelopes);
            if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                self.queue_outbox(envelopes.iter().cloned());
                self.invalidate_socket_authority().await;
                return;
            }
            let Some(socket_actor) = self.socket_actor.clone() else {""", """            note_authored_envelopes(&mut self.applied_op_ids, envelopes);
            let Some(socket_actor) = self.socket_actor.clone() else {""", "relay")

edit(SRC, """        async fn send_raw(&mut self, message: Message) {
            if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                self.invalidate_socket_authority().await;
                return;
            }
            let mut failed = false;""", """        async fn send_raw(&mut self, message: Message) {
            let mut failed = false;""", "send")

edit(SRC, """
        #[cfg(test)]
        pub(super) fn expire_test_socket_authority(&mut self) {
            self.socket_authority_deadline = Some(Instant::now());
        }
""", "", "test hook")

edit(SRC, """        /// 📥️ One socket turn: greet the hub once the page reports the socket open, then hand at most one
        /// page of frames to the protocol. An expired authority, a close or any lost frame reconnects.
        async fn pump_socket(&mut self) {
            if self.socket_authority.as_ref().is_some_and(|authority| authority.expires_at_unix_ms <= wall_ms()) {
                self.disconnect();
                return;
            }
            let Some(socket) = self.socket.as_mut() else { return };""", """        /// 📥️ One socket turn: greet the hub once the page reports the socket open, then hand at most one
        /// page of frames to the protocol. A close or any lost frame reconnects; the admission plan's
        /// expiry bounds when its grant may be exchanged, never the live socket (the hub re-proves the
        /// socket's authority every second).
        async fn pump_socket(&mut self) {
            let Some(socket) = self.socket.as_mut() else { return };""", "wasm pump")

edit(TESTS, """async fn native_terminal_connection_failure_clears_receipt_actor_before_reissue() {
    use futures::StreamExt;
    use tokio_tungstenite::tungstenite::Message;
""", """async fn native_terminal_connection_failure_clears_receipt_actor_before_reissue() {
    use futures::StreamExt;
""", "law imports")

edit(TESTS, """    assert_eq!(actor.socket_epoch_test_state(), (Some(reconnected.into()), true, 1, Vec::new()));

    actor.expire_test_socket_authority();
    let after_expiry = sample_operation_envelope("after-authority-expiry", 5).await;
    actor.relay_test_envelope(after_expiry).await;
    let (socket_actor, confirmed, pending, queued) = actor.socket_epoch_test_state();
    assert_eq!((socket_actor, confirmed, pending), (None, false, 0));
    assert_eq!(queued.len(), 2, "unacknowledged and post-expiry mutations stay queued for a fresh plan");
    let terminal = tokio::time::timeout(std::time::Duration::from_secs(1), reconnected_socket.next()).await.expect("expired authority closes promptly");
    assert!(!matches!(terminal, Some(Ok(Message::Binary(_)))), "expired plan authority cannot carry another command");
}""", """    assert_eq!(actor.socket_epoch_test_state(), (Some(reconnected.into()), true, 1, Vec::new()));
}""", "law expiry tail")

edit(TESTS, """    struct MockHubSocketGrantSource {
        hub_origin: String,
        actor_id: String,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }""", """    struct MockHubSocketGrantSource {
        hub_origin: String,
        actor_id: String,
        plan_window_ms: u64,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }""", "mock field")

edit(TESTS, """            let expires_at_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("system clock after epoch").as_millis().saturating_add(60_000).min(i64::MAX as u128) as i64;""",
     """            let expires_at_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("system clock after epoch").as_millis().saturating_add(u128::from(self.plan_window_ms)).min(i64::MAX as u128) as i64;""", "mock expiry")

edit(TESTS, """    fn configure_mock_hub(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub) {
        host.set_local_hub_credential(Arc::new(crate::os_directory::client::LocalHubCredential::test(hub_origin, &format!("session.v1.{}.{}", actor_fill.to_string().repeat(32), actor_fill.to_string().repeat(64)))));
        host.set_hub_socket_grant_source(Arc::new(MockHubSocketGrantSource { hub_origin: hub_origin.into(), actor_id: format!("hub.v1.{}", actor_fill.to_string().repeat(64)), trace: hub.trace.clone() }));
    }""", """    fn configure_mock_hub(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub) {
        configure_mock_hub_with_plan_window(host, hub_origin, actor_fill, hub, 60_000);
    }

    /// ⏳️ [`configure_mock_hub`] with the admission plan's exchange window the grant source answers.
    fn configure_mock_hub_with_plan_window(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub, plan_window_ms: u64) {
        host.set_local_hub_credential(Arc::new(crate::os_directory::client::LocalHubCredential::test(hub_origin, &format!("session.v1.{}.{}", actor_fill.to_string().repeat(32), actor_fill.to_string().repeat(64)))));
        host.set_hub_socket_grant_source(Arc::new(MockHubSocketGrantSource { hub_origin: hub_origin.into(), actor_id: format!("hub.v1.{}", actor_fill.to_string().repeat(64)), plan_window_ms, trace: hub.trace.clone() }));
    }""", "mock configure")

edit(TESTS, """        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    // 🔬️ Reconnect with `since` catch-up:""", """        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    /// 🔌️ A live document socket outlives its admission plan's exchange window: the plan's expiry bounds when its grant
    /// may be exchanged (the hub's `DOCUMENT_OPEN_PLAN_MAX_TTL_MS`, 30 s), never the socket, which the hub re-proves every
    /// second. With a 3 s window, an edit sent 3.5 s after Session goes out on the SAME socket and is accepted — the React
    /// worker's contract since 09-22 (ticket 26/09/23 session 14c: both Rust actors re-planned every live socket at it).
    #[tokio::test]
    async fn a_live_socket_outlives_its_admission_plan_window() {
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let host = ArtifactHost::new(test_pool());
        configure_mock_hub_with_plan_window(&host, &base_url, 'a', &hub, 3_000);
        let channels = host
            .open(ArtifactActorConfig {
                document_id: "plan-window".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let key = channels.document_key.clone();
        let mut events = host.subscribe_key(&key).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("Session", &hub, &mut events, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));
        tokio::time::sleep(std::time::Duration::from_millis(3_500)).await;
        let edit = document_backbone_envelope("after-the-plan-window", "plan-window");
        channels.cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: document_backbone_message(std::slice::from_ref(&edit)) }).expect("edit after the plan window");
        assert!(matches!(wait_for_mock_hub_event("CommandOutcome", &hub, &mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await, ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }));
        assert_eq!(hub.connections.load(Ordering::SeqCst), 1, "the edit went out on the socket admitted inside the plan window");
        host.close_key(&key);
    }

    // 🔬️ Reconnect with `since` catch-up:""", "live socket law")

if any(marker in files[SRC] for marker in ("self.socket_authority", "socket_authority: ", "socket_authority_deadline")) or "invalidate_socket_authority" in files[SRC] or "expire_test_socket_authority" in files[TESTS]:
    problems.append("a socket authority deadline reference survived")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
