/// 🐢️ The long hostile-input laws module: bounded admin pages, the socket grant ledger and process-isolated saturation.

mod long {
use semio_hub_directory_contract::{ADMIN_PAGE_MAX, ADMIN_RESPONSE_MAX_BYTES, SocketGrantCapability};
use semio_hub_ledgers::guard::AdminPrincipalV1;
use std::sync::Arc;
use semio_hub_runtime::startup::{ArtifactCasMaintenanceCheckpoint, StartupCatalogControl};
use semio_hub_artifact_creation_service::ArtifactCreationServiceV1;
use semio_hub_directory_contract::model::{AuthSessionKind, SpaceRole};
use semio_hub_artifact_authority::{AuthorityLimits, AuthorityOperationControl, OperationContext};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, STUDIO, announce_document_for_test, bearer_headers, client_binary, create_space_for_test, document_open_plan_test_authority, document_socket_request, issue_test_session, next_server_frame, next_server_frame_at, next_server_frame_within, publish_checkpoint_for_test, raw_http_request, run_socket_test, sample_envelope, seed_author_token, seed_genesis_document_for_test, socket_hello, spawn_restartable_server, spawn_server, test_state, upsert_member_for_test};
use directory::os_directory::DocumentScope;
use semio_hub_ledgers::socket_grants::{SOCKET_GRANT_BINDING_PENDING_CAPACITY, SOCKET_GRANT_LEDGER_CAPACITY, SocketAudienceV1, SocketGrantKeyV1, SocketGrantLedgerErrorV1, SocketGrantLedgerV1, SocketSubjectV1};
use protocol::ArtifactId as WireArtifactId;
use semio_hub_route_models::admin::{admin_cursor_encode, admin_fit_connection_snapshot, admin_fit_page, admin_fit_space_detail};
use semio_hub_route_models::artifact_creation::artifact_creation_request_id_v1;
use tokio_tungstenite::connect_async;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_runtime::support::now_ms;
use crate::tests::process_isolated_law;
#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use directory::os_directory::{AdminRecordedConnectionV1, DirectorySpaceRole, DirectorySpaceVisibility, MemberView, SpaceView};
#[cfg(test)]
use axum::extract::{Path, State};
#[cfg(test)]
use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationStatusV1};
#[cfg(test)]
use axum::http::StatusCode;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::issue_document_socket_grant_fixture;

    /// 📈️ A document socket keeps acknowledging commands however large its document grows: 150
    /// chained writes over one live socket, every one `Accepted`, past each former wall of the
    /// document store (the eighth index entry, the 128th replaced value). A hub used to go silent — no
    /// Ack, no error — once a document grew past one of them.
    #[test]
    fn a_document_socket_keeps_acknowledging_commands_as_its_document_grows() {
        run_socket_test(|| async {
            const EDITS: usize = 150;
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-growth").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-growth".to_string())), bearer_headers(&token), State(state.clone())).await.expect("growth socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-growth/document/ws"), &token)).await.expect("growth socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("growth hello");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            let mut previous: Option<protocol::MutationId> = None;
            for index in 0..EDITS {
                let mut envelope = sample_envelope(&format!("growth-{index}"), &WireArtifactId("socket-growth".into())).await;
                envelope.actor = ActorId(receipt.actor_id.clone());
                envelope.diff.payload = db::document::encode_pathmap_json(&serde_json::json!({ format!("feature-{}", index % 8): format!("{index}:{}", "g".repeat(2048)) })).await.unwrap();
                envelope.dependencies = previous.iter().cloned().collect();
                let batch = 10_000 + index as u64;
                socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![envelope.clone()] }, Lane::Command).await).await.expect("growth command");
                let outcome = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                    loop {
                        if let ServerFrame::Ack { batch_id, stages, .. } = next_server_frame_at(&mut socket, "growth ack").await {
                            if batch_id == batch {
                                return stages;
                            }
                        }
                    }
                })
                .await
                .unwrap_or_else(|_| panic!("the hub never acknowledged growth edit {index}"));
                assert!(matches!(outcome.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)), "growth edit {index}: {outcome:?}");
                previous = Some(envelope.mutation_id);
            }
            let frontier = state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "socket-growth"))).await.expect("growth handle").frontier().await.expect("growth frontier");
            assert_eq!(frontier.commit_seq, EDITS as u64);
        });
    }

    /// 📨️ Every command a document socket receives is answered — accepted or refused with a reason —
    /// even when the database refuses writes: twelve sockets on twelve documents write concurrently
    /// until the process's shared credit refuses some of them, and no socket is ever left without the
    /// Ack for a batch it sent. A refused write that left its sender waiting forever looked, to a
    /// client, exactly like a hub that stopped listening.
    #[test]
    fn every_command_is_answered_when_the_database_refuses_writes() {
        if !process_isolated_law("tests::long::every_command_is_answered_when_the_database_refuses_writes") {
            return;
        }
        run_socket_test(|| async {
            const SOCKETS: usize = 12;
            const EDITS: usize = 16;
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            let addr = spawn_server(state.clone()).await;
            let mut sockets = Vec::with_capacity(SOCKETS);
            for index in 0..SOCKETS {
                let document = format!("refusal-{index:02}");
                announce_document_for_test(&state, STUDIO, &document).await;
                let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.clone())), bearer_headers(&token), State(state.clone())).await.expect("refusal socket grant").0;
                let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2F{document}/document/ws"), &token)).await.expect("refusal socket");
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("refusal hello");
                assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
                assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
                sockets.push((document, receipt.actor_id, socket));
            }
            let outcomes = futures::future::join_all(sockets.into_iter().map(|(document, actor, mut socket)| async move {
                let mut accepted = 0usize;
                let mut refused = Vec::new();
                let mut previous: Option<protocol::MutationId> = None;
                for edit in 0..EDITS {
                    let mut envelope = sample_envelope(&format!("{document}-{edit}"), &WireArtifactId(document.clone())).await;
                    envelope.actor = ActorId(actor.clone());
                    envelope.diff.payload = db::document::encode_pathmap_json(&serde_json::json!({ format!("path-{edit:02}"): format!("{edit}:{}", "r".repeat(1024)) })).await.unwrap();
                    envelope.dependencies = previous.iter().cloned().collect();
                    let batch = 20_000 + edit as u64;
                    socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![envelope.clone()] }, Lane::Command).await).await.expect("refusal command");
                    let stages = loop {
                        if let ServerFrame::Ack { batch_id, stages, .. } = next_server_frame_within(&mut socket, &format!("{document} edit {edit} ack"), std::time::Duration::from_secs(40)).await {
                            if batch_id == batch {
                                break stages;
                            }
                        }
                    };
                    match stages.last() {
                        Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted) => {
                            accepted += 1;
                            previous = Some(envelope.mutation_id);
                        }
                        Some(AckStage::Applied { outcome }) => match outcome.as_ref() {
                            ApplyOutcome::Rejected { reason, .. } => refused.push(reason.clone()),
                            other => panic!("{document} edit {edit}: {other:?}"),
                        },
                        other => panic!("{document} edit {edit}: {other:?}"),
                    }
                }
                (document, accepted, refused)
            }))
            .await;
            let answered: usize = outcomes.iter().map(|(_, accepted, refused)| accepted + refused.len()).sum();
            assert_eq!(answered, SOCKETS * EDITS, "every command was answered: {outcomes:?}");
        });
    }

    /// 🌱️ Server-owned creation on the GIS plugin's real release component: the catalog is author-only,
    /// a stale generation or unknown kind claims nothing, two concurrent duplicates own one factory, and
    /// the guest's own genesis reaches `ready` with a descriptor and a genesis-backed active checkpoint.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed() {
        use semio_hub_artifact_creation::ArtifactCreationOperationV1;
        use semio_hub_trusted_catalog_fixtures as trusted_catalog_fixture;
        use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1;

        let profile = trusted_catalog_fixture::verified_gis_map_release_profile(&trusted_catalog_fixture::unique_profile_root("artifact-creation-http")).await.expect("verified GIS Map creation profile on the real release component");
        let mut state = test_state().await;
        state.verified_catalog = Some(profile.catalog().clone());
        state.artifact_creation = Some(Arc::new(ArtifactCreationServiceV1::new(state.directory_service.clone(), profile.catalog().clone(), state.artifact_cas.clone())));
        let author = issue_test_session(&state, "creation-author@example.test").await;
        let peer = issue_test_session(&state, "creation-peer@example.test").await;
        let spectator = issue_test_session(&state, "creation-spectator@example.test").await;
        let space_id = create_space_for_test(&state, &author.user_id, "Artifact Creation", directory::os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, "creation-author@example.test", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space_id, "creation-peer@example.test", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space_id, "creation-spectator@example.test", DirectorySpaceRole::Spectator).await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let route = format!("/spaces/{space_id}/artifact-creations");
        let author_bearer = format!("Bearer {}", author.token);
        let peer_bearer = format!("Bearer {}", peer.token);
        let spectator_bearer = format!("Bearer {}", spectator.token);
        let catalog = raw_http_request(addr, "GET", &route, &[("Authorization", author_bearer.as_str())], &[]).await;
        assert_eq!(catalog.status, 200);
        let catalog_source = std::str::from_utf8(&catalog.body).expect("creation catalog UTF-8");
        let catalog = SpaceArtifactCreationCatalogV1::parse_canonical_json(catalog_source).expect("canonical selected creation catalog");
        assert_eq!(catalog.space_id, space_id);
        assert_eq!(catalog.kinds.iter().map(|kind| kind.kind_id.as_str()).collect::<Vec<_>>(), vec!["s.gis.gismap"]);
        assert_eq!(raw_http_request(addr, "GET", &route, &[], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "GET", &route, &[("Authorization", spectator_bearer.as_str())], &[]).await.status, 403);

        let request = SpaceArtifactCreateV1 {
            schema: "semio.hub.space-artifact-create/v1".into(),
            request_id: "1234567890abcdef1234567890abcdef".into(),
            expected_catalog_generation_id: catalog.catalog_generation_id.clone(),
            kind_id: "s.gis.gismap".into(),
            name: "Shared Map".into(),
        };
        let body = semio_framework_pack_json::to_json_string(&request);
        let malformed = body.replacen("{", "{\"documentId\":\"caller-owned\",", 1);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], malformed.as_bytes()).await.status, 400);
        let unknown = SpaceArtifactCreateV1 { request_id: "2234567890abcdef1234567890abcdef".into(), kind_id: "s.gis.unknown".into(), ..request.clone() };
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], semio_framework_pack_json::to_json_string(&unknown).as_bytes()).await.status, 409);
        let stale_generation = SpaceArtifactCreateV1 { request_id: "3234567890abcdef1234567890abcdef".into(), expected_catalog_generation_id: "9".repeat(64), ..request.clone() };
        assert_ne!(stale_generation.expected_catalog_generation_id, catalog.catalog_generation_id);
        let directory_head = state.directory.head_seq().await.expect("directory head before stale catalog creation");
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], semio_framework_pack_json::to_json_string(&stale_generation).as_bytes()).await.status, 409);
        assert!(state.directory.read_artifact_creation(&author.user_id, &stale_generation.request_id).await.expect("stale catalog creation facts").is_empty());
        assert_eq!(state.directory.head_seq().await.expect("directory head after stale catalog creation"), directory_head, "a stale catalog generation cannot claim an operation or append directory events");
        let creation_headers = [("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")];
        let first = raw_http_request(addr, "POST", &route, &creation_headers, body.as_bytes());
        let duplicate = raw_http_request(addr, "POST", &route, &creation_headers, body.as_bytes());
        let (first, duplicate) = tokio::join!(first, duplicate);
        assert!([200, 202].contains(&first.status) && [200, 202].contains(&duplicate.status), "exact concurrent duplicate never reports capacity or owns a second factory");
        for response in [&first, &duplicate] {
            let source = std::str::from_utf8(&response.body).expect("creation acceptance UTF-8");
            let status = SpaceArtifactCreationStatusV1::parse_canonical_json(source).expect("creation acceptance is canonical");
            assert_eq!(status.catalog_generation_id, request.expected_catalog_generation_id);
        }
        let facts = state.directory.read_artifact_creation(&author.user_id, &request.request_id).await.expect("durable creation facts");
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts).expect("one durable creation operation");
        assert!(operation.intent.scope.document_id.strip_prefix("artifact-").is_some_and(artifact_creation_request_id_v1));
        let created_document_id = operation.intent.scope.document_id;

        let status_route = format!("{route}/{}", request.request_id);
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(600);
        let ready = loop {
            let response = raw_http_request(addr, "GET", &status_route, &[("Authorization", author_bearer.as_str())], &[]).await;
            let status = SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("creation status UTF-8")).expect("canonical creation status");
            let running = matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing);
            assert_eq!(response.status, if running { 202 } else { 200 }, "a running creation answers 202, a terminal one 200");
            assert_eq!(status.catalog_generation_id, request.expected_catalog_generation_id);
            if status.phase == SpaceArtifactCreationPhaseV1::Ready {
                break status;
            }
            assert!(matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing | SpaceArtifactCreationPhaseV1::Indeterminate), "creation reached an unexpected terminal phase");
            assert!(tokio::time::Instant::now() < deadline, "genesis on the real GIS guest did not become Ready");
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        };
        let ready_scope = DocumentScope::new(&space_id, &ready.ready.as_ref().expect("Ready coordinates").artifact_id);
        assert_eq!(ready.catalog_generation_id, request.expected_catalog_generation_id);
        assert_eq!(ready_scope.document_id, created_document_id, "both concurrent requests retain one server-minted document");
        assert!(state.directory.get_document_descriptor(&ready_scope).await.expect("created descriptor read").is_some());
        assert!(state.directory.get_active_artifact_checkpoint(&ready_scope).await.expect("created checkpoint read").is_some_and(|checkpoint| checkpoint.baseline_frontier.is_genesis_for(&ready_scope)));
        assert_eq!(raw_http_request(addr, "GET", &status_route, &[("Authorization", peer_bearer.as_str())], &[]).await.status, 404, "another Author cannot read the private request key");
        let cancelled = raw_http_request(addr, "POST", &format!("{status_route}/cancel"), &[("Authorization", author_bearer.as_str())], &[]).await;
        assert_eq!(cancelled.status, 200);
        assert_eq!(SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&cancelled.body).expect("cancel status UTF-8")).expect("canonical cancel status").phase, SpaceArtifactCreationPhaseV1::Ready, "cancel cannot overwrite Ready");
        assert_eq!(state.artifact_creation_tasks.task_count(), 0);
        let _ = shutdown.send(());
        server.await.expect("creation HTTP server stop");
        state.artifact_creation_tasks.shutdown().await;
    }

    #[test]
    fn admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row() {
        let cursor_key = [0x5a; 32];
        let principal = AdminPrincipalV1 {
            user_id: "user:admin".into(),
            auth_session_id: "session:admin".into(),
            authorization_generation: 7,
            identity_provider: "test".into(),
            identity_subject_digest: [7; 32],
            expires_at_ms: now_ms() + 60_000,
            correlation_id: "correlation:admin".into(),
            peer_class: "admin-rest",
            session_kind: AuthSessionKind::DevelopmentLocal,
            device_instance_id: "admin-device".into(),
        };
        let rows = (0..ADMIN_PAGE_MAX).map(|index| directory::os_directory::UserView { id: format!("user:{index}:{}", "i".repeat(4_000)), email: format!("{index}@{}", "e".repeat(4_000)), display_name: "n".repeat(4_000), created_at_ms: 0 }).collect();
        let page = admin_fit_page(rows, false, 7, |rows| admin_cursor_encode(&cursor_key, &principal, 2, rows.len())).expect("byte-bounded user page");
        assert!(page.rows.len() < ADMIN_PAGE_MAX);
        assert!(page.next_cursor.is_some());
        assert!(semio_framework_pack_json::to_json_string(&page).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let connections = (0..ADMIN_PAGE_MAX)
            .map(|index| AdminRecordedConnectionV1 {
                sync_session_id: format!("sync:{index}:{}", "s".repeat(4_000)),
                scope: DocumentScope::new("space", format!("document:{index}:{}", "d".repeat(4_000))),
                authenticated_user_id: Some(format!("user:{index}:{}", "u".repeat(4_000))),
                email: Some("admin@example.com".into()),
                role: Some(DirectorySpaceRole::Author),
                connected_at_ms: 0,
                source: "recorded-sync-session".into(),
            })
            .collect();
        let snapshot = admin_fit_connection_snapshot(connections, false, 7, 9, &cursor_key, &principal, 0).expect("byte-bounded connection snapshot");
        assert!(snapshot.rows.len() < ADMIN_PAGE_MAX);
        assert!(snapshot.next_cursor.is_some());
        assert!(semio_framework_pack_json::to_json_string(&snapshot).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let view = SpaceView {
            id: "space:one".into(),
            name: "Space".into(),
            kind: directory::os_directory::DirectorySpaceKind::Studio,
            visibility: DirectorySpaceVisibility::Private,
            owner_user_id: "user:owner".into(),
            role: None,
            member_count: ADMIN_PAGE_MAX as u32,
            document_count: 0,
            active_connections: 0,
            created_at_ms: 0,
            updated_at_ms: 0,
        };
        let members = (0..ADMIN_PAGE_MAX).map(|index| MemberView { user_id: format!("user:{index}:{}", "u".repeat(4_000)), email: format!("{index}@example.com"), display_name: "n".repeat(4_000), role: DirectorySpaceRole::Author }).collect();
        let detail = admin_fit_space_detail(view, members, false, 7, &cursor_key, &principal, "space:one", 0).expect("byte-bounded member detail");
        assert!(detail.members.rows.len() < ADMIN_PAGE_MAX);
        assert!(detail.members.next_cursor.is_some());
        assert!(semio_framework_pack_json::to_json_string(&detail).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let oversized = vec![directory::os_directory::UserView { id: "i".repeat(ADMIN_RESPONSE_MAX_BYTES), email: "e@example.com".into(), display_name: "name".into(), created_at_ms: 0 }];
        assert_eq!(admin_fit_page(oversized, false, 7, |_| Ok("a".repeat(84))), Err(StatusCode::PAYLOAD_TOO_LARGE));
    }

    #[tokio::test]
    async fn socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe() {
        let ledger = Arc::new(SocketGrantLedgerV1::default());
        let audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-a"));
        let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-a".into() };
        let capability = SocketGrantCapability::mint().expect("socket grant");
        ledger.issue(&capability, audience.clone(), "hub.v1.actor".into(), subject.clone(), 1, 9_000).expect("issue grant");
        assert!(ledger.pending(&capability, &SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-b")), 2).is_err(), "audience mismatch never consumes");
        let candidate = ledger.pending(&capability, &audience, 2).expect("pending grant");
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let attempts = (0..2)
            .map(|_| {
                let ledger = ledger.clone();
                let candidate = candidate.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    ledger.consume(&candidate, 3).is_ok()
                })
            })
            .collect::<Vec<_>>();
        barrier.wait();
        assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("consume race")).filter(|won| *won).count(), 1, "exactly one concurrent upgrade consumes");
        assert!(ledger.pending(&capability, &audience, 4).is_err(), "consumed grants never replay");
        assert!(SocketGrantLedgerV1::default().pending(&capability, &audience, 4).is_err(), "grants are process-bound and disappear on restart");
        let (_, live_notify) = ledger.register_live(&candidate).expect("register consumed grant live");

        let pending = SocketGrantCapability::mint().expect("pending socket grant");
        ledger.issue(&pending, audience.clone(), "hub.v1.pending".into(), subject.clone(), 4, 9_000).expect("issue pending grant");
        let stale = ledger.pending(&pending, &audience, 5).expect("candidate before revoke");
        ledger.invalidate_binding(subject.binding());
        tokio::time::timeout(std::time::Duration::from_secs(1), live_notify.notified()).await.expect("live revoke notification");
        assert!(ledger.consume(&stale, 6).is_err(), "revoke between durable revalidation and consume fails closed");
        assert!(ledger.register_live(&candidate).is_err(), "consume then revoke then late register fails closed");

        let ttl_ledger = SocketGrantLedgerV1::default();
        let ttl_capability = SocketGrantCapability::mint().expect("TTL grant");
        ttl_ledger.issue(&ttl_capability, audience.clone(), "hub.v1.ttl".into(), subject.clone(), 1, 10).expect("issue TTL grant");
        let ttl_candidate = ttl_ledger.pending(&ttl_capability, &audience, 2).expect("pending TTL grant");
        let ttl_consumed = ttl_ledger.consume(&ttl_candidate, 3).expect("consume TTL grant");
        let (ttl_live_id, _) = ttl_ledger.register_live(&ttl_consumed).expect("register TTL grant live");
        let sweep_trigger = SocketGrantCapability::mint().expect("sweep trigger");
        ttl_ledger.issue(&sweep_trigger, audience.clone(), "hub.v1.sweep".into(), subject.clone(), 11, 100).expect("trigger grant sweep");
        assert!(ttl_ledger.is_live(&ttl_consumed, &ttl_live_id), "grant TTL applies to dial/consume, not a durably-authorized live socket");
        ttl_ledger.unregister_live(&ttl_consumed, &ttl_live_id);
        assert!(!ttl_ledger.inner.lock().expect("ledger").records.contains_key(ttl_capability.selector()), "last live lease reclaims its consumed grant record");

        let abandoned = SocketGrantLedgerV1::default();
        let mut first_abandoned = None;
        for index in 0..SOCKET_GRANT_LEDGER_CAPACITY {
            let capability = SocketGrantCapability::mint().expect("abandoned grant");
            abandoned.issue(&capability, audience.clone(), format!("hub.v1.abandoned.{index}"), subject.clone(), 1, 10).expect("fill ledger");
            let candidate = abandoned.pending(&capability, &audience, 2).expect("abandoned pending");
            abandoned.consume(&candidate, 3).expect("abandoned consume");
            first_abandoned.get_or_insert(capability);
        }
        assert!(abandoned.pending(first_abandoned.as_ref().expect("first abandoned"), &audience, 4).is_err(), "consumed failed-pre-live grant never replays");
        let recovered = SocketGrantCapability::mint().expect("recovered grant");
        abandoned.issue(&recovered, audience.clone(), "hub.v1.recovered".into(), subject.clone(), 11, 100).expect("expired pre-live tombstones reclaim full ledger capacity");

        let bounded = SocketGrantLedgerV1::default();
        for index in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
            let capability = SocketGrantCapability::mint().expect("bounded grant");
            bounded.issue(&capability, audience.clone(), format!("hub.v1.{index}"), subject.clone(), 1, 9_000).expect("within per-binding bound");
        }
        let overflow = SocketGrantCapability::mint().expect("overflow grant");
        assert_eq!(bounded.issue(&overflow, audience, "hub.v1.overflow".into(), subject.clone(), 1, 9_000), Err(SocketGrantLedgerErrorV1::Capacity));
        bounded.invalidate_binding(subject.binding());
        assert!(bounded.issue(&overflow, SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-a")), "hub.v1.after-revoke".into(), subject, 2, 9_000).is_ok());
    }

    #[tokio::test]
    async fn document_socket_grants_are_credential_bound_secret_free_oldest_first_and_single_consume() {
        let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
        let plan = Arc::new(document_open_plan_test_authority(&fixture));
        let ledger = SocketGrantLedgerV1::default();
        let audience = SocketAudienceV1::Document(DocumentScope::new("space-a", "document-a"));
        let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-a".into() };
        let other = SocketSubjectV1::Session { session_id: "session-b".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-b".into() };
        assert_eq!(ledger.issue(&SocketGrantCapability::mint().expect("capability"), audience.clone(), "hub.v1.capability".into(), subject.clone(), 1, 9_000), Err(SocketGrantLedgerErrorV1::Rejected), "document audiences never answer to a capability");
        assert_eq!(ledger.admit_document(SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-a")), "hub.v1.scoped".into(), subject.clone(), 1, 9_000, plan.clone()), Err(SocketGrantLedgerErrorV1::Rejected), "only document audiences are credential-bound");
        let older = ledger.admit_document(audience.clone(), "hub.v1.older".into(), subject.clone(), 1, 9_000, plan.clone()).expect("older admission");
        let newer = ledger.admit_document(audience.clone(), "hub.v1.newer".into(), subject.clone(), 2, 9_000, plan.clone()).expect("newer admission");
        assert_ne!(older, newer);
        assert!(ledger.pending_document_binding(&audience, &other.binding(), 3).is_err(), "another credential never consumes this binding's grant");
        assert!(ledger.pending_document_binding(&SocketAudienceV1::Document(DocumentScope::new("space-a", "document-b")), &subject.binding(), 3).is_err(), "audience is exact");
        let first = ledger.pending_document_binding(&audience, &subject.binding(), 3).expect("oldest pending");
        assert_eq!((first.selector.as_str(), first.actor_id.as_str(), first.key), (older.as_str(), "hub.v1.older", SocketGrantKeyV1::CredentialBinding));
        ledger.consume(&first, 4).expect("single consume");
        assert!(ledger.consume(&first, 5).is_err(), "consumed admissions never replay");
        let second = ledger.pending_document_binding(&audience, &subject.binding(), 5).expect("next pending");
        assert_eq!(second.selector, newer);
        ledger.invalidate_binding(subject.binding());
        assert!(ledger.pending_document_binding(&audience, &subject.binding(), 6).is_err(), "revoke drops pending admissions");
        assert!(ledger.admit_document(audience.clone(), "hub.v1.expired".into(), subject.clone(), 7, 8, plan).is_ok());
        assert!(ledger.pending_document_binding(&audience, &subject.binding(), 9).is_err(), "expired admissions never consume");
    }

    #[tokio::test]
    async fn artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests() {
        let state = test_state().await;
        for index in 0..5 {
            let space_id = create_space_for_test(&state, "seed", &format!("CAS {index}"), directory::os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let (document_id, _) = seed_genesis_document_for_test(&state, "cas-maintenance-author@example.com", &space_id, &format!("cas-maintenance-{index}")).await;
            publish_checkpoint_for_test(&state, &space_id, &document_id).await;
        }
        let control = StartupCatalogControl::silent();
        let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
        let mut checkpoint = ArtifactCasMaintenanceCheckpoint::default();
        let mut requests = 0usize;
        let mut examined = 0u64;
        loop {
            let result = state.directory_service.sweep_artifact_cas(state.artifact_cas.as_ref(), checkpoint.request(false, 1), &context).await.expect("bounded maintenance page");
            requests += 1;
            examined += result.examined_objects;
            if checkpoint.accept(&result) {
                break;
            }
            assert!(requests < 128, "maintenance cursor converges");
        }
        assert!(requests > 16);
        assert!(examined > 16);
    }
}
