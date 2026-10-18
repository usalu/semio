// 🎭️ Native actor tests module: the native actor tests with the mock hub, their fixtures and the actor laws.

//#region 🧪️Actor
#[cfg(not(target_arch = "wasm32"))]
mod actor_tests {
    use semio_framework_test_fixtures::fixture_path;

    use super::*;
    use semio_framework_os_store_sync::{ARTIFACT_MAILBOX_BYTES, ARTIFACT_MAILBOX_ITEMS, ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactMailboxSendError, ArtifactSyncStatus, CommandAckOutcome, DocumentBackboneRetentionV1, HISTORY_TRANSITION_REFUSED_CODE, OpenDocument, PersistenceBinding, PresenceHeartbeatProducer, RemoteState, SyncError, SyncSession, artifact_actor_message_bytes, artifact_mailbox_pair, backbone_worker_wire, decode_document_backbone_message_exact, envelopes_from_history_edit, frontier_reaches, hub_ws_url, presence_to_bytes, rollback_envelope, spr_op_ids, stamp_session};
use semio_framework_os_spr::{AckStage, ActorId, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapPair, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationId, PresencePeer, PresenceToolRun, RuntimeFrontierSummary, ServerFrame, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes};
use semio_framework_os_store_sync::fixtures::{ActorFixture, FixtureInbound, load_fixtures};
use semio_framework_os_store_sync_native::native_actor::ArtifactHost;
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_backbone::{BackboneMessage, Backbones, ChannelBackbone, decode_hot_backbone_message_exact};
use semio_framework_os_store_sync_native::lifted_sync::{FOLDER_EVENT_MAGIC, FolderEventLogStorage, FolderTextStorage};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_value::ValueError;
use tokio::sync::broadcast;
    use semio_framework_os_spr::{decode_client_frame, encode_server_frame};
    use futures::{SinkExt, StreamExt};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::{broadcast as tokio_broadcast, Mutex};
    use tokio_tungstenite::tungstenite::Message as WsMessage;

    async fn demo_envelope(document_id: &str) -> semio_framework_os_store_space_schema::ArtifactEnvelope<DemoSnapshot, DemoMutation> {
        create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)
    }

    async fn wait_until<Fut>(label: &str, mut predicate: impl FnMut() -> Fut)
    where
        Fut: std::future::Future<Output = bool>,
    {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while !predicate().await {
            if tokio::time::Instant::now() >= deadline {
                panic!("{label} not satisfied before 5s deadline");
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_until_value<T, Fut>(label: &str, mut predicate: impl FnMut() -> Fut) -> T
    where
        Fut: std::future::Future<Output = Option<T>>,
    {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(value) = predicate().await {
                return value;
            }
            if tokio::time::Instant::now() >= deadline {
                panic!("{label} not satisfied before 5s deadline");
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_for_event(events: &mut broadcast::Receiver<ArtifactEvent>, predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        wait_for_named_event("event", events, predicate).await
    }

    async fn wait_for_named_event(label: &str, events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, events.recv()).await {
                Ok(Ok(event)) => {
                    if predicate(&event) {
                        return event;
                    }
                }
                Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                other => panic!("no matching {label} before deadline: {other:?}"),
            }
        }
    }

    async fn wait_for_mock_hub_event(label: &str, hub: &MockHub, events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        let mut observed = Vec::new();
        loop {
            match tokio::time::timeout_at(deadline, events.recv()).await {
                Ok(Ok(event)) => {
                    if predicate(&event) {
                        return event;
                    }
                    observed.push(format!("{event:?}"));
                }
                Ok(Err(broadcast::error::RecvError::Lagged(count))) => observed.push(format!("lagged:{count}")),
                other => panic!("no matching {label} before deadline: {other:?}; actor-events={observed:?}; mock-hub={:?}", hub.trace()),
            }
        }
    }

    // 🔬️ External folder edit → RemoteMutations event + the store timeline grows on tick().
    /// 📁️ A local apply establishes a persisted edit on disk.
    ///
    /// Wait until the actor has persisted the local edit to the folder db as real pack+spr bytes.
    ///
    /// Out-of-band: append a foreign edit directly to the spr bytes (real binary op
    /// payloads, no codec, no JSON) before writing pack+spr back.
    ///
    /// Deterministically poke the actor to re-read (notify also wired, but timing-independent here).
    ///
    /// The store ingests the pushed operation on tick(); the timeline grows and snapshot updates.
    #[tokio::test]
    async fn folder_external_edit_delivers_remote_operations() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        ensure_demo_codec_registered().await;
        let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
        let host = ArtifactHost::new(test_pool());
        let channels = host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "doc-a".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
        let mut events = host.subscribe("doc-a").await;
        let mut store = semio_framework_os_store_test_support::plain_test_store(demo_envelope("doc-a").await).await;
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply");
        channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        let mut archive = wait_until_value("persisted edit on disk", || async {
            let bytes = storage.read_archive("doc-a").await.expect("read archive")?;
            let archive = semio_framework_os_spr::decode_document_archive_bytes(&bytes).await.ok()?;
            if spr_op_ids(&archive.parent_spr).await.ok()?.is_empty() {
                None
            } else {
                Some(archive)
            }
        })
        .await;

        let external_edit = semio_framework_os_spr::HistoryEdit { line: None,
            id: "external-1".into(),
            actor: Some("peer".into()),
            started_at: "0".into(),
            finished_at: None, verb: None,
            ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
            inverse: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 1 }.encode_op().expect("encode")) }],
            meta: None, lane: None,
        };
        archive.parent_spr = semio_framework_os_store_text_io::text_format::append_history_events_to_spr(&archive.parent_spr, &[external_edit], &[]).await.expect("append external edit");
        let bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode externally changed archive");
        storage.write_archive("doc-a", "demo/v1", &bytes).await.expect("out-of-band archive write");

        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");

        let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
        match event {
            ArtifactEvent::RemoteMutations { envelopes } => {
                assert_eq!(envelopes.len(), 1);
                assert_eq!(envelopes[0].mutation_id.0, "external-1#0", "single-op edit -> mutation_id is edit.id#0 (crate::os_spr::mutation_envelope_from_edit's ordinal-suffix convention)");
            }
            other => panic!("expected RemoteMutations, got {other:?}"),
        }

        store.tick().await.expect("tick");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "external edit joined the timeline");
        assert_eq!(store.snapshot().expect("snapshot").n, 42);
        host.close("doc-a");
        semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    }

    //#region 🔖️MockHub
    /// 🧪️ A minimal in-process semio_hub speaking the real, binary `crate::os_spr::wire::ClientFrame`/
    /// `ServerFrame` protocol, so the semio_hub endpoint is exercised end-to-end without linking a real
    /// `db`-backed semio_hub (that's CW6's job — this mock never touches `db`). Ordinal-indexed log,
    /// mirroring `db_sync`'s replica-catch-up shape (`Hello.frontier` -> filtered backlog ->
    /// `Welcome` then a follow-up `Commands`), but with a placeholder `chain_hash`/`resume_token`
    /// (this mock has no durable log to derive a real chain hash from).
    struct MockHub {
        log: Arc<Mutex<Vec<(u64, MutationEnvelope)>>>,
        broadcast: tokio_broadcast::Sender<ServerFrame>,
        connections: AtomicUsize,
        session_gate: Option<Arc<tokio::sync::Semaphore>>,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl MockHub {
        fn record(&self, event: &'static str) {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(event);
        }

        fn trace(&self) -> Vec<&'static str> {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
        }
    }

    struct MockHubSocketGrantSource {
        hub_origin: String,
        actor_id: String,
        plan_window_ms: u64,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl semio_framework_os_directory_client::HubSocketGrantSource for MockHubSocketGrantSource {
        fn admit_document_socket(
            &self,
            _ctx: &semio_framework_async::OperationContext,
            space_id: &str,
            document_id: &str,
            expectation: &semio_framework_os_directory_client::DocumentSocketExpectationV1,
            _client_instance_id: &str,
            _timeout_ms: u64,
        ) -> Result<semio_framework_os_directory_client::DocumentSocketAdmissionV1, semio_framework_os_directory_client::DirectoryClientError> {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("grant.entered");
            let expires_at_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("system clock after epoch").as_millis().saturating_add(u128::from(self.plan_window_ms)).min(i64::MAX as u128) as i64;
            let admission = semio_framework_os_directory_client::DocumentSocketAdmissionV1 {
                socket: semio_framework_os_directory_client::DocumentSocketGrantReceiptV1 {
                    schema: "semio.hub.document-socket-grant/v1".into(),
                    protocol: "semio.session.v1".into(),
                    actor_id: self.actor_id.clone(),
                    expires_at_ms,
                },
                authority: semio_framework_os_directory_client::DocumentSocketAuthorityV1 {
                    admitted_lease: expectation.lease.clone(),
                    hub_origin: self.hub_origin.clone(),
                    browser_actor: semio_framework_os_directory::DocumentOpenBrowserActorV1::None,
                    expires_at_unix_ms: u64::try_from(expires_at_ms).expect("positive mock expiry"),
                    scope: semio_framework_os_directory::DocumentScope::new(space_id, document_id),
                    descriptor_digest_v1: "4".repeat(64),
                    catalog: semio_framework_os_directory::DocumentOpenCatalogV1 { generation_id: "5".repeat(64) },
                    package: semio_framework_os_directory::DocumentOpenPackageV1 {
                        plugin_id: "mock-plugin".into(),
                        package_id: "semio:mock-plugin".into(),
                        version: "1.0.0".into(),
                        component_sha256: "6".repeat(64),
                        component_blake3: "7".repeat(64),
                        descriptor_byte_sha256: "8".repeat(64),
                        execution_protocol: semio_framework_os_directory::DocumentExecutionProtocolV1 { app_channel_version: semio_framework_os_spr::CHANNEL_VERSION },
                        component_receipt: semio_framework_plugin_component_receipt::ComponentReceiptV1 {
                            schema: semio_framework_plugin_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
                            plugin_id: "mock-plugin".into(),
                            package_id: "semio:mock-plugin".into(),
                            profile: semio_framework_plugin_component_receipt::ComponentProfileV1::Release,
                            target: semio_framework_plugin_component_receipt::COMPONENT_TARGET.into(),
                            wasm_sha256: "6".repeat(64),
                            wasm_bytes: 1024,
                            semantic_descriptor_sha256: "8".repeat(64),
                            toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
                            source_tree_digest: "3".repeat(64),
                            provenance_sha256: "4".repeat(64),
                        },
                    },
                    artifact: semio_framework_os_directory::DocumentOpenArtifactV1 { kind: "mock.document".into(), schema: expectation.artifact_schema.clone(), pack_schema_hash: "1".repeat(64) },
                    parent_dialect: semio_framework_os_directory::DocumentOpenParentDialectV1 { artifact_kind: "mock.document".into(), standard: "1".into(), subset: "*".into() },
                    pack_schema_hash: expectation.pack_schema_hash,
                    surface: semio_framework_os_directory::DocumentOpenSurfaceV1 {
                        surface_id: expectation.requested_surface_id.clone().unwrap_or_else(|| "mock.surface".into()),
                        app_id: "mock.app".into(),
                        window_kind_id: "mock.window".into(),
                        role: semio_framework_os_directory::DocumentOpenSurfaceRoleV1::Editor,
                        renderer_target: semio_framework_os_directory::DocumentOpenRendererTargetV1::Wgpu,
                    },
                    grant: semio_framework_os_directory::DocumentOpenGrantV1 { read: true, write: true, observe: true },
                    checkpoint: semio_framework_os_directory::DocumentOpenCheckpointV1 {
                        checkpoint_id: "9".repeat(64),
                        descriptor_digest_v1: "4".repeat(64),
                        baseline_frontier: semio_framework_os_directory::ArtifactFrontier {
                            document_id: document_id.to_string(),
                            head_edit_ordinal: 0,
                            head_edit_id: String::new(),
                            last_commit_seq: 0,
                            chain_hash: semio_framework_os_directory::ArtifactHash::new([0; 32]),
                        },
                        aggregate_sha256: "a".repeat(64),
                    },
                    revalidation: semio_framework_os_directory::DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
                },
            };
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("grant.returned");
            Ok(admission)
        }
    }

    fn configure_mock_hub(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub) {
        configure_mock_hub_with_plan_window(host, hub_origin, actor_fill, hub, 60_000);
    }

    /// ⏳️ [`configure_mock_hub`] with the admission plan's exchange window the grant source answers.
    fn configure_mock_hub_with_plan_window(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub, plan_window_ms: u64) {
        host.set_local_hub_credential(Arc::new(semio_framework_os_directory_client::LocalHubCredential::test(hub_origin, &format!("session.v1.{}.{}", actor_fill.to_string().repeat(32), actor_fill.to_string().repeat(64)))));
        host.set_hub_socket_grant_source(Arc::new(MockHubSocketGrantSource { hub_origin: hub_origin.into(), actor_id: format!("hub.v1.{}", actor_fill.to_string().repeat(64)), plan_window_ms, trace: hub.trace.clone() }));
    }

    async fn mock_frontier(ordinal: u64) -> RuntimeFrontierSummary {
        RuntimeFrontierSummary { document_id: ArtifactId("mock".to_string()), head_edit_ordinal: ordinal, head_edit_id: format!("edit-{ordinal}"), last_commit_seq: ordinal, chain_hash: [0u8; 32] }
    }

    async fn spawn_mock_hub() -> (std::net::SocketAddr, Arc<MockHub>) {
        spawn_mock_hub_with_session_gate(false).await
    }

    /// 🎭️ Framework document WS (C4b): client offers `semio.session.v1, <token>`;
    /// server selects the protocol token only (mirrors gateway `.protocols([SESSION_PROTOCOL_V1])`).
    ///
    /// Session actor is derived from the session credential (last segment of
    /// `session.v1.<selector>.<secret>` → `hub.v1.<secret>`), never from a
    /// caller-supplied `?actor=` query — mirrors the real hub binding.
    async fn spawn_mock_hub_with_session_gate(gated: bool) -> (std::net::SocketAddr, Arc<MockHub>) {
        ensure_demo_codec_registered().await;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let (broadcast, _rx) = tokio_broadcast::channel(256);
        let semio_hub = Arc::new(MockHub { log: Arc::new(Mutex::new(Vec::new())), broadcast, connections: AtomicUsize::new(0), session_gate: gated.then(|| Arc::new(tokio::sync::Semaphore::new(0))), trace: Arc::new(std::sync::Mutex::new(Vec::new())) });
        let accept_hub = semio_hub.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else { break };
                let conn_hub = accept_hub.clone();
                conn_hub.record("tcp.accepted");
                tokio::spawn(async move {
                    let socket_actor = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));
                    let socket_actor_for_hdr = socket_actor.clone();
                    let accepted = tokio_tungstenite::accept_hdr_async(
                        stream,
                        move |request: &tokio_tungstenite::tungstenite::handshake::server::Request,
                         mut response: tokio_tungstenite::tungstenite::handshake::server::Response|
                         -> Result<_, tokio_tungstenite::tungstenite::handshake::server::ErrorResponse> {
                            const SESSION_PROTOCOL_V1: &str = "semio.session.v1";
                            let offered = request
                                .headers()
                                .get("Sec-WebSocket-Protocol")
                                .and_then(|value| value.to_str().ok())
                                .unwrap_or("");
                            let selected = offered.split(',').next().map(str::trim).filter(|token| *token == SESSION_PROTOCOL_V1);
                            let Some(selected) = selected else {
                                return Err(tokio_tungstenite::tungstenite::http::Response::builder().status(400).body(None).expect("static refuse"));
                            };
                            response.headers_mut().insert("Sec-WebSocket-Protocol", selected.parse().expect("static protocol header"));
                            let actor = offered.split_once(", ").and_then(|(_, token)| {
                                let secret = token.rsplit('.').next()?;
                                (!secret.is_empty()).then(|| format!("hub.v1.{secret}"))
                            });
                            *socket_actor_for_hdr.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = actor;
                            Ok(response)
                        },
                    )
                    .await;
                    match accepted {
                        Ok(ws) => {
                            conn_hub.record("websocket.accepted");
                            let actor = match socket_actor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                                Some(actor) => {
                                    let _ = conn_hub.connections.fetch_add(1, Ordering::SeqCst);
                                    actor
                                }
                                None => {
                                    let n = conn_hub.connections.fetch_add(1, Ordering::SeqCst);
                                    let fill = if n == 0 { 'a' } else { 'b' };
                                    format!("hub.v1.{}", fill.to_string().repeat(64))
                                }
                            };
                            mock_hub_connection(ws, conn_hub, actor).await;
                        }
                        Err(_) => conn_hub.record("websocket.refused"),
                    }
                });
            }
        });
        (addr, semio_hub)
    }

    /// 🤝️ Expect Hello first.
    ///
    /// 👻️ Best-effort fan-out on the uncredited preview lane — this mock
    /// semio_hub doesn't track per-connection actor identity beyond `Hello`, so
    /// it stamps a fixed sentinel origin (fine for the round-trip test
    /// this drives, which only asserts the *other* peer receives it).
    async fn mock_hub_connection(ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, semio_hub: Arc<MockHub>, actor: String) {
        let (mut write, mut read) = ws.split();
        let requested_ordinal = match read.next().await {
            Some(Ok(WsMessage::Binary(bytes))) => match decode_client_frame(&bytes).await {
                Ok((_, ClientFrame::SocketHelloV1 { frontier, .. })) => {
                    semio_hub.record("client.socket-hello");
                    frontier.map_or(0, |frontier| frontier.head_edit_ordinal)
                }
                Ok(_) => {
                    semio_hub.record("client.unexpected-first-frame");
                    return;
                }
                Err(_) => {
                    semio_hub.record("client.malformed-first-frame");
                    return;
                }
            },
            Some(Ok(_)) => {
                semio_hub.record("client.nonbinary-first-frame");
                return;
            }
            Some(Err(_)) => {
                semio_hub.record("client.first-frame-error");
                return;
            }
            None => {
                semio_hub.record("client.closed-before-hello");
                return;
            }
        };
        let (frontier, backlog) = {
            let log = semio_hub.log.lock().await;
            let ordinal = log.last().map_or(0, |(ordinal, _)| *ordinal);
            let backlog: Vec<MutationEnvelope> = log.iter().filter(|(ordinal, _)| *ordinal > requested_ordinal).map(|(_, envelope)| envelope.clone()).collect();
            (mock_frontier(ordinal).await, backlog)
        };
        if let Some(gate) = &semio_hub.session_gate {
            semio_hub.record("server.session-gate-waiting");
            gate.acquire().await.expect("mock session gate remains live").forget();
            semio_hub.record("server.session-gate-released");
        }
        let welcome = ServerFrame::Welcome { session_id: "mock-session".to_string(), resume_token: "mock-resume".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
        if write.send(WsMessage::Binary(encode_server_frame(&welcome, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.welcome-send-failed");
            return;
        }
        semio_hub.record("server.welcome-sent");
        let session = ServerFrame::Session { actor, color: 1 };
        if write.send(WsMessage::Binary(encode_server_frame(&session, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.session-send-failed");
            return;
        }
        semio_hub.record("server.session-sent");
        let commands = ServerFrame::Commands { envelopes: backlog, origin: ActorId("semio_hub-backlog".into()), frontier: frontier.clone() };
        if write.send(WsMessage::Binary(encode_server_frame(&commands, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.commands-send-failed");
            return;
        }
        semio_hub.record("server.commands-sent");
        let mut broadcast_rx = semio_hub.broadcast.subscribe();
        loop {
            tokio::select! {
                incoming = read.next() => {
                    match incoming {
                        Some(Ok(WsMessage::Binary(bytes))) => {
                            match decode_client_frame(&bytes).await {
                                Ok((_, ClientFrame::Commands { batch_id, envelopes })) => {
                                    semio_hub.record("client.commands");
                                    let mut assigned_frontier = frontier.clone();
                                    for envelope in envelopes {
                                        let (ordinal, origin) = {
                                            let mut log = semio_hub.log.lock().await;
                                            let next = log.last().map_or(0, |(ordinal, _)| *ordinal) + 1;
                                            log.push((next, envelope.clone()));
                                            (next, envelope.actor.clone())
                                        };
                                        assigned_frontier = mock_frontier(ordinal).await;
                                        let _ = semio_hub.broadcast.send(ServerFrame::Commands { envelopes: vec![envelope], origin, frontier: assigned_frontier.clone() });
                                    }
                                    let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: assigned_frontier };
                                    let _ = write.send(WsMessage::Binary(encode_server_frame(&ack, Lane::Command).await.into())).await;
                                }
                                Ok((_, ClientFrame::PreviewPublish { key, seq, payload })) => {
                                    semio_hub.record("client.preview-publish");
                                    let _ = semio_hub.broadcast.send(ServerFrame::Preview { actor: ActorId("mock-semio_hub-peer".into()), key, seq, payload });
                                }
                                Ok((_, ClientFrame::Bye)) => semio_hub.record("client.bye"),
                                Ok(_) => semio_hub.record("client.other"),
                                Err(_) => semio_hub.record("client.malformed-frame"),
                            }
                        }
                        Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => break,
                        Some(Ok(_)) => {}
                    }
                }
                frame = broadcast_rx.recv() => {
                    match frame {
                        Ok(frame) => {
                            if write.send(WsMessage::Binary(encode_server_frame(&frame, Lane::Command).await.into())).await.is_err() {
                                break;
                            }
                        }
                        Err(tokio_broadcast::error::RecvError::Lagged(_)) => {}
                        Err(tokio_broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }
    }
    //#endregion 🔖️MockHub

    /// 🔬️ Two stores, each behind its own ArtifactHost, converge through a semio_hub on edits AND
    /// structural history: every step A authors (apply, undo, checkpoint, redo) reaches B only as
    /// events relayed by the hub, and B's fold lands on exactly A's position after each one.
    #[tokio::test]
    async fn two_hosts_converge_through_hub() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let config = |actor: &str| ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
            document_id: "shared".into(),
            schema: "demo/v1".into(),
            bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
            watch_external: false,
            actor: actor.into(),
        };
        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a.open(config("A")).await;
        let key_a = channels_a.document_key.clone();
        let mut events_a = host_a.subscribe_key(&key_a).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;
        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b.open(config("B")).await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        let mut store_a = semio_framework_os_store_test_support::plain_test_store(demo_envelope("shared").await).await;
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        let mut store_b = semio_framework_os_store_test_support::plain_test_store(demo_envelope("shared").await).await;
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");
        let position = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| (store.snapshot().expect("snapshot"), store.applied_edit_ids().len(), store.redo_edit_ids().len(), store.current_checkpoint_id().map(str::to_string));

        let steps: Vec<ArtifactCommand<DemoMutation>> = vec![
            ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], transaction: None },
            ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], transaction: None },
            ArtifactCommand::Undo,
            ArtifactCommand::CommitCheckpoint { message: Some("hub checkpoint".into()), authors: Vec::new() },
            ArtifactCommand::Redo,
        ];
        for command in steps {
            store_a.dispatch(command, &mut identity).await.expect("a authors the step");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
            wait_for_mock_hub_event("B DocumentBackbone", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
            store_b.tick().await.expect("b folds the relayed events");
            assert_eq!(position(&store_b), position(&store_a), "B sits exactly where A sits after every hub-relayed step");
        }
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 9, "the redo reinstated A's second edit on B too");

        semio_framework_os_store_test_support::close_plain_test_store(&mut store_a);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_b);
        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    #[tokio::test]
    async fn raw_document_backbone_reaches_hub_once_and_returns_one_canonical_event() {
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "raw-shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let key_a = channels_a.document_key.clone();
        let mut events_a = host_a.subscribe_key(&key_a).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));
        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "raw-shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));

        let original = document_backbone_envelope("raw-mutation", "raw-shared");
        channels_a.cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: document_backbone_message(std::slice::from_ref(&original)) }).expect("exact raw owner admitted");
        let event = wait_for_mock_hub_event("B DocumentBackbone", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        let delivered = document_backbone_event_envelopes(&event).expect("canonical raw event");
        assert_eq!(delivered.len(), 1);
        assert_eq!(delivered[0].mutation_id, original.mutation_id);
        assert_eq!(delivered[0].diff, original.diff);
        assert_eq!(delivered[0].inverse, original.inverse);
        assert_ne!(delivered[0].actor, original.actor, "Hub authority stamps its actor without rewriting opaque mutation fields");
        assert!(matches!(wait_for_mock_hub_event("A CommandOutcome", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await, ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }));
        while let Ok(next) = events_b.try_recv() {
            assert!(!matches!(next, ArtifactEvent::RemoteMutations { .. }), "one server Commands frame cannot emit a duplicate legacy mutation event");
        }

        host_a.close_key(&key_a);
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
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
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

    // 🔬️ Reconnect with `since` catch-up: after A appends operations while B is offline, B reconnects and
    // its Welcome backlog carries only the operations it missed.
    /// 📬️ A applies two operations while nobody else is connected.
    ///
    /// B connects fresh (since_version 0) and its Welcome backlog replays both operations.
    #[tokio::test]
    async fn reconnect_since_catch_up_replays_backlog() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "catchup".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let mut events_a = host_a.subscribe_key(&channels_a.document_key).await;
        let mut store_a = semio_framework_os_store_test_support::plain_test_store(demo_envelope("catchup").await).await;
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        for n in [3, 4] {
            store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n }], transaction: None }, &mut identity).await.expect("apply on a");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
            tokio::time::sleep(Duration::from_millis(80)).await;
        }

        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "catchup".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "B".into() })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = semio_framework_os_store_test_support::plain_test_store(demo_envelope("catchup").await).await;
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact backlog event").len(), 2, "backlog replays both missed operations");
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 4);

        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_a);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_b);
    }

    // 🔬️ Detach drains the outbox: an operation applied right before close still reaches the semio_hub (and B).
    /// 🚿️ Observer B stays connected to witness A's last operation.
    ///
    /// Immediately close A without waiting for the poll tick: Detach must flush the outbox first.
    #[tokio::test]
    async fn detach_drains_pending_outbound_operations() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "drain".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = semio_framework_os_store_test_support::plain_test_store(demo_envelope("drain").await).await;
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a =
            host_a.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "drain".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "A".into() }).await;
        let mut events_a = host_a.subscribe_key(&channels_a.document_key).await;
        let mut store_a = semio_framework_os_store_test_support::plain_test_store(demo_envelope("drain").await).await;
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], transaction: None }, &mut identity).await.expect("apply on a");
        host_a.close_key(&key_a);

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact drain event").len(), 1, "the operation applied before detach was not lost");
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 5);
        host_b.close_key(&key_b);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_b);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_a);
    }

    // 🔬️ The mock semio_hub always Acks `Accepted` — confirms the new `ServerFrame::Ack` ->
    // `ArtifactEvent::CommandOutcome` wiring actually fires (not just that it compiles).
    #[tokio::test]
    async fn command_outcome_accepted_fires_after_hub_ack() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");
        let host = ArtifactHost::new(test_pool());
        configure_mock_hub(&host, &base_url, 'a', &hub);
        let channels =
            host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "outcome".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "A".into() }).await;
        let key = channels.document_key.clone();
        let mut events = host.subscribe_key(&key).await;
        let mut store = semio_framework_os_store_test_support::plain_test_store(demo_envelope("outcome").await).await;
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
        wait_for_mock_hub_event("A Session", &hub, &mut events, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply");
        channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

        let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await;
        match event {
            ArtifactEvent::CommandOutcome { outcome, .. } => assert_eq!(outcome, CommandAckOutcome::Accepted),
            other => panic!("expected CommandOutcome, got {other:?}"),
        }
        host.close_key(&key);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    }

    // 🔬️ `SyncSession::publish_preview` -> `ClientFrame::PreviewPublish` -> the mock semio_hub's
    // preview-lane fan-out -> `ServerFrame::Preview` -> `ArtifactEvent::Preview` on another peer.
    #[tokio::test]
    async fn publish_preview_round_trips_through_hub() {
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "preview".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let mut events_a = host_a.subscribe_key(&channels_a.document_key).await;

        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "preview".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "B".into() })
            .await;
        let key_a = channels_a.document_key.clone();
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        channels_a.cmd_tx.send(ArtifactActorMsg::PublishPreview { key: "cursor".into(), seq: 1, payload: vec![1, 2, 3] }).expect("publish preview");

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::Preview { .. })).await;
        match event {
            ArtifactEvent::Preview { key, seq, payload, .. } => {
                assert_eq!(key, "cursor");
                assert_eq!(seq, 1);
                assert_eq!(payload, vec![1, 2, 3]);
            }
            other => panic!("expected Preview, got {other:?}"),
        }
        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    // 🔬️ Shared fixtures replay: each fixture's inbound stimuli produce the expected ArtifactEvent
    // sequence and final timeline. The same fixtures drive WS-E's vitest harness against the TS twin.
    #[tokio::test]
    async fn fixtures_replay_matches_expected_events() {
        let fixtures_dir = fixture_path!("../../../../🧫️fixtures");
        let fixtures = load_fixtures(&fixtures_dir).await;
        assert!(!fixtures.is_empty(), "expected fixtures in {fixtures_dir:?}");
        for fixture in fixtures {
            replay_fixture(&fixture).await;
        }
    }

    /// 🎞️ Lockstep: apply each stimulus, then wait for its paired expected event before the next
    /// (removes any write/poke race). Folder-replayable fixtures pair inbound 1:1 with events.
    async fn replay_fixture(fixture: &ActorFixture) {
        ensure_demo_codec_registered().await;
        let codec = semio_framework_os_store_codec_registry::document_codec(&fixture.schema).await.expect("codec registry available").unwrap_or_else(|| panic!("no codec registered for fixture schema {:?}", fixture.schema));
        let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
        let host = ArtifactHost::new(test_pool());
        let channels =
            host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: fixture.document_id.clone(), schema: fixture.schema.clone(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
        let mut events = host.subscribe(&fixture.document_id).await;
        let mut store = semio_framework_os_store_test_support::plain_test_store(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        wait_until(&format!("seed snapshot for {} on disk", fixture.document_id), || async { storage.read_archive(&fixture.document_id).await.expect("read archive").is_some() }).await;

        assert_eq!(fixture.inbound.len(), fixture.expected_events.len(), "fixture {} must pair each inbound stimulus with one expected event", fixture.name);
        let mut observed: Vec<String> = Vec::new();
        for (inbound, expected) in fixture.inbound.iter().zip(fixture.expected_events.iter()) {
            match inbound {
                FixtureInbound::ExternalEdits { ops_text } => {
                    let archive_bytes = storage.read_archive(&fixture.document_id).await.expect("read archive").expect("some archive");
                    let mut archive = semio_framework_os_spr::decode_document_archive_bytes(&archive_bytes).await.expect("decode current archive");
                    let parsed = semio_framework_os_spr::parse_ops_text(ops_text).unwrap_or_else(|error| panic!("fixture {} parse_ops_text: {error}", fixture.name));
                    let mut new_edits: Vec<semio_framework_os_spr::HistoryEdit> = Vec::new();
                    for edit in parsed.edits {
                        let mut ops: Vec<semio_framework_os_spr::OpPayload> = Vec::new();
                        for op in &edit.ops {
                            let text = op.text.as_deref().unwrap_or_else(|| panic!("fixture {} op line has no text", fixture.name));
                            let concrete = DemoMutation::parse_op(text).unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
                            ops.push(semio_framework_os_spr::OpPayload { text: None, binary: Some(concrete.encode_op().expect("encode demo op")) });
                        }
                        new_edits.push(semio_framework_os_spr::HistoryEdit { ops, meta: None, ..edit });
                    }
                    archive.parent_spr = semio_framework_os_store_text_io::text_format::append_history_events_to_spr(&archive.parent_spr, &new_edits, &[]).await.expect("append fixture edits");
                    let archive_bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode fixture archive");
                    storage.write_archive(&fixture.document_id, &fixture.schema, &archive_bytes).await.expect("write archive");
                    channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                }
                FixtureInbound::ReplaceDocument { dsl_text, ops_text } => {
                    let (pack_files, _dsl_mirror) = (codec.compile_dsl)(dsl_text, ops_text).await.unwrap_or_else(|error| panic!("fixture {} compile_dsl: {error}", fixture.name));
                    let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pack_files.pack, parent_spr: pack_files.spr, members: Vec::new() };
                    let archive_bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode replacement archive");
                    storage.write_archive(&fixture.document_id, &fixture.schema, &archive_bytes).await.expect("replace archive write");
                    channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                }
                FixtureInbound::HubFrame { .. } => {
                    panic!("fixture {} uses a HubFrame stimulus not supported by the Rust harness", fixture.name);
                }
            }
            let event = wait_for_event(&mut events, |event| document_event_tag(event) == expected.as_str()).await;
            observed.push(document_event_tag(&event).to_string());
            store.tick().await.expect("tick");
        }
        assert_eq!(&observed, &fixture.expected_events, "fixture {} event sequence", fixture.name);
        let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        for expected_id in &fixture.expected_edit_ids {
            assert!(timeline_ids.contains(expected_id), "fixture {} expected edit id {expected_id} in timeline {timeline_ids:?}", fixture.name);
        }
        host.close(&fixture.document_id);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    }

    // 🚫️async: E1-adjacent — pure match with no suspension point, consumed by
    // `wait_for_event`'s sync `FnMut(&ArtifactEvent) -> bool` predicate bound — see R9.
    fn document_event_tag(event: &ArtifactEvent) -> &'static str {
        match event {
            ArtifactEvent::RemoteMutations { .. } => "remoteMutations",
            ArtifactEvent::DocumentBackbone { .. } => "documentBackbone",
            ArtifactEvent::DocumentArchiveReplaced { .. } => "documentArchiveReplaced",
            ArtifactEvent::DocumentArchiveAbsent => "documentArchiveAbsent",
            ArtifactEvent::BootstrapProgress { .. } => "bootstrapProgress",
            ArtifactEvent::Status(_) => "status",
            ArtifactEvent::Presence { .. } => "presence",
            ArtifactEvent::Session { .. } => "session",
            ArtifactEvent::Preview { .. } => "preview",
            ArtifactEvent::CommandOutcome { .. } => "commandOutcome",
            ArtifactEvent::Conflict(_) => "conflict",
            ArtifactEvent::RebootstrapRequired { .. } => "rebootstrapRequired",
        }
    }
}
//#endregion 🧪️Actor
