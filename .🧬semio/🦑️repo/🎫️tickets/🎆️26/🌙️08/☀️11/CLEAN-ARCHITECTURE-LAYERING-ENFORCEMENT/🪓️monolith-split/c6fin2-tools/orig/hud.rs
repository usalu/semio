
use semio_hub_route_models::admin::{ADMIN_EFFECT_ADMITTED, ADMIN_EFFECT_PRE_EFFECT, ADMIN_OPERATION_DEADLINE, AdminIntentMetadata, AdminIntentSecretResult, AdminOperationCleanup, AdminOperationRuntime, AdminOperationTaskOwner, AdminPageQuery, admin_audit_receipt, admin_audit_visibility, admin_create_space_id, admin_cursor_decode, admin_cursor_decode_scoped, admin_cursor_encode_scoped, admin_intent_bindings, admin_page_limit, new_admin_audit_fact};
use semio_hub_directory_contract::{ADMIN_PAGE_MAX, InviteCapability, SessionCapability, SocketGrantCapability};
use semio_hub_directory_contract::model::{AdminEffectCommitV1, AuthSessionKind, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, DirectoryCommandResultKindV1, NewAdminOperationEffectReceiptV1, NewDirectoryCommandReceipt, SpaceRole};
use semio_hub_ledgers::guard::AdminPrincipalV1;
use std::sync::Arc;
use directory::os_directory::{DirectoryCommandOutcomeV1, DirectoryCommandResultV1, DocumentScope};
use semio_hub_directory_contract::error::DirectoryError;
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, LawHangWatchdogV1, RawHttpResponse, SOCKET_RENDEZVOUS_HANG_GUARD, STUDIO, TEST_EXECUTION_TARGET_COMPONENT_BYTES, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES, announce_document_for_test, authorize_test_admin, bearer_headers, bounded_law_step, client_binary, create_space_for_test, document_open_catalog_for_descriptor, document_open_plan_test_authority, document_socket_request, issue_and_exchange_document_open_plan_for_test, issue_test_session, next_close_code, next_close_without_authority, next_directory_message, next_server_frame_at, post_directory_command_for_test, presence_hex_bytes, presence_normalization_fixture, raw_http_get, raw_http_request, run_socket_test, seed_author_token, seed_genesis_for_document_for_test, socket_hello, socket_request, spawn_restartable_server, spawn_server, stop_recovery_server, tempdir, test_state, test_state_with_directory, upsert_member_for_test};
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_runtime::readiness::{HubMode, hub_readiness};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SOCKET_GRANT_BINDING_PENDING_CAPACITY, SOCKET_GRANT_LEDGER_CAPACITY, SocketAudienceV1, SocketBindingKeyV1, SocketBindingValidityV1, SocketGrantKeyV1, SocketGrantLedgerV1, SocketGrantRecordV1, SocketGrantStateV1, SocketSubjectV1};
use semio_framework_async::ShardedMap;
use semio_hub_directory_sqlite::SqliteDirectory;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_session_routes::guard::authenticate_admin_principal;
use tokio_tungstenite::connect_async;
use semio_hub_route_models::directory_events::{directory_access_change_for_reader, directory_message_bindings, directory_stream_message_space};
use semio_hub_directory_service::{directory_command_result_kind, replay_directory_command_receipt};
use {semio_framework_os_kernel as directory_command_sha256, semio_framework_os_kernel as validate_directory_event_page_event};
use semio_hub_runtime::presence::document_scope_key_v1;
use semio_hub_session_routes::session_agent::{get_session_me, post_session_sign_out};
use semio_hub_session_routes::access_grants::issue_directory_socket_grant;
use semio_hub_document_routes::admin::reconcile_stale_admin_acceptance;
use semio_hub_session_routes::directory_commands::resolve_bearer_user;
use semio_hub_document_routes::directory_events::socket_directory_membership_visibility;
use semio_hub_directory::{AdminOperationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog, InviteDirectory, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::admin::ADMIN_SECRET_WIPE_BYTES;
#[cfg(test)]
use directory::os_directory::{AdminIntentReceiptV1, AdminIntentStateV1, AdminIntentV1, ConnectionView, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryConnectionPhase, DirectoryEvent, DirectoryEventPageErrorV1, DirectoryEventPageV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenIntentV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanErrorV1, DocumentOpenPlanV1, DocumentPlanSocketGrantIntentV1};
#[cfg(test)]
use axum::body::Bytes;
#[cfg(test)]
use protocol::{ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use axum::http::{HeaderMap, StatusCode};
#[cfg(test)]
use axum::extract::State;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::{TestLiveGate, last_document_plan_refusal};
#[cfg(test)]
use directory::os_directory;
/// ⏳️ The single wall-clock bound every `test_state()` open in this suite answers to. Opening a hub
/// state is bounded I/O, not a rendezvous, so the bound exists only to name a genuinely wedged open
/// instead of letting nextest kill it 300 s later with no site. It replaces the per-call 5 s bounds,
/// which measured fleet scheduling latency and failed `directory_invite_redemption_admitted_fence_
/// precedes_archive` on load alone (2026-09-20).
const TEST_STATE_OPEN_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(60);

fn assert_recovery_denied(response: RawHttpResponse, expected: &serde_json::Value) {
    assert_eq!(u64::from(response.status), expected["removedStatus"].as_u64().unwrap());
    assert!(response.headers.lines().any(|line| line.eq_ignore_ascii_case("content-type: application/json")));
    let error: DocumentOpenPlanErrorV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("strict bounded error, not selected asset bytes");
    assert_eq!(error.schema, "semio.hub.document-open-plan-error/v1");
    assert_eq!(error.code, DocumentOpenPlanErrorCodeV1::Denied);
    assert_eq!(expected["removedCode"], "denied");
}

#[test]
fn admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json")).expect("admin recovery fixture");
        let expected = &fixture["expected"];
        let scope = DocumentScope::new(fixture["scope"]["spaceId"].as_str().unwrap(), fixture["scope"]["documentId"].as_str().unwrap());
        let dir = tempdir("admin-presence-target-recovery");
        std::fs::create_dir_all(&dir).expect("recovery fixture directory");
        let path = dir.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("file directory");
        let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
        let admin_headers = authorize_test_admin(&mut state, "recovery-admin@example.com").await;
        let removed = issue_test_session(&state, "recovery-removed@example.com").await;
        let observer = issue_test_session(&state, "recovery-observer@example.com").await;
        for member in fixture["members"].as_array().unwrap() {
            let email = format!("recovery-{}@example.com", member["id"].as_str().unwrap());
            assert_eq!(member["role"], "author");
            upsert_member_for_test(&state, &scope.space_id, &email, DirectorySpaceRole::Author).await;
        }
        seed_genesis_for_document_for_test(&state, &removed.token, &scope.space_id, &scope.document_id, "admin-presence-recovery").await;
        let descriptor = state.directory.get_document_descriptor(&scope).await.unwrap().unwrap();
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        let (plan_b, grant_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:removed").await;
        let (plan_c, grant_c) = issue_and_exchange_document_open_plan_for_test(&state, &observer.token, &scope, "client:observer").await;
        assert_eq!(plan_b.surface.surface_id, fixture["surfaceId"].as_str().unwrap());
        assert_eq!(plan_b.surface.surface_id, plan_c.surface.surface_id);
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let root = format!("/spaces/{}/documents/{}", scope.space_id, scope.document_id);
        let url =         format!(
            "ws://{addr}/scopes/{}%2F{}/document/ws?surface={}",
            scope.space_id,
            scope.document_id,
            plan_b.surface.surface_id
        );
        let (mut b, _) = connect_async(document_socket_request(&url, &removed.token)).await.expect("member document socket");
        let (mut c, _) = connect_async(document_socket_request(&url, &observer.token)).await.expect("observer document socket");
        b.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
        c.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
        let welcome_b = match tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, b.next()).await {
            Ok(Some(Ok(WsMessage::Binary(bytes)))) => protocol::decode_server_frame(&bytes).await.expect("member welcome frame").1,
            other => panic!("the member plan socket never reached its welcome: {other:?} refused as {:?}", last_document_plan_refusal()),
        };
        assert!(matches!(&welcome_b, ServerFrame::Welcome { .. }), "member welcome: {welcome_b:?}");
        let ServerFrame::Session { actor: actor_b, color: color_b } = next_server_frame_at(&mut b, "member session frame after the sync-hello bootstrap drain").await else { panic!("member session") };
        let welcome_c = next_server_frame_at(&mut c, "observer welcome").await;
        assert!(matches!(&welcome_c, ServerFrame::Welcome { .. }), "observer welcome: {welcome_c:?}");
        let ServerFrame::Session { actor: actor_c, .. } = next_server_frame_at(&mut c, "observer session frame after the sync-hello bootstrap drain").await else { panic!("observer session") };
        let raw = presence_hex_bytes(presence_normalization_fixture()["vectors"][0]["rawPeerHex"].as_str().unwrap());
        b.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.unwrap();
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut c, "observer normalized presence of the member").await else { panic!("observer normalized presence") };
        assert_eq!(peers.len(), 1);
        let peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
        assert_eq!(peer.actor, actor_b);
        assert_eq!(peer.user_id.as_deref(), Some(removed.user_id.as_str()));
        assert_eq!(peer.label.as_deref(), Some("recovery-removed@example.com"));
        assert_eq!(peer.role.as_deref(), Some("author"));
        assert_eq!(peer.color, Some(color_b));
        assert_eq!(peer.surface.as_deref(), Some(plan_b.surface.surface_id.as_str()));
        assert!(matches!(next_server_frame_at(&mut b, "member echo of its own normalized presence").await, ServerFrame::Presence { .. }));
        let intent = semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some(plan_b.surface.surface_id.clone()),
            client_instance_id: "client:recovery-target".into(),
        });
        let authorization_b = format!("Bearer {}", removed.token);
        let authorization_c = format!("Bearer {}", observer.token);
        let headers_b = [("Authorization", authorization_b.as_str()), ("Content-Type", "application/json")];
        let headers_c = [("Authorization", authorization_c.as_str()), ("Content-Type", "application/json")];
        assert_eq!(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_b, intent.as_bytes()).await.status, 200);
        let (_, pending_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:pending-removed").await;
        let remove = semio_framework_pack_json::to_json_string(&AdminIntentV1::RemoveSpaceMember { request_id: "request:admin-presence-recovery".into(), space_id: scope.space_id.clone(), user_id: removed.user_id.clone() });
        let admin_authorization = admin_headers.get(axum::http::header::AUTHORIZATION).unwrap().to_str().unwrap();
        let receipt = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", admin_authorization), ("Content-Type", "application/json")], remove.as_bytes()).await;
        assert_eq!(receipt.status, 200);
        let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&receipt.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
        assert_eq!(u64::from(next_close_without_authority(&mut b).await), expected["removedCloseCode"].as_u64().unwrap());
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut c, "observer withdrawal presence after the admin removal").await else { panic!("removed member withdrawal") };
        assert_eq!(peers.len() as u64, expected["visibleAfterRemoval"].as_u64().unwrap());
        assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
        let exchange = semio_framework_pack_json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan_b.receipt.clone() });
        let denied = raw_http_request(addr, "POST", &format!("{root}/socket-grants"), &headers_b, exchange.as_bytes()).await;
        assert_recovery_denied(denied, expected);
        assert!(connect_async(document_socket_request(&url, &removed.token)).await.is_err(), "removed member cannot reuse its consumed grant");
        assert!(connect_async(document_socket_request(&url, &removed.token)).await.is_err(), "removal invalidates a previously unused member grant");
        for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
            let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
            assert_recovery_denied(denied, expected);
        }
        c.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.unwrap();
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut c, "observer presence after the removal, still live").await else { panic!("observer remains live") };
        assert_eq!(peers.len(), 1);
        let observer_peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
        assert_eq!(observer_peer.actor, actor_c);
        assert_eq!(observer_peer.user_id.as_deref(), Some(observer.user_id.as_str()));
        assert_eq!(observer_peer.role.as_deref(), Some("author"));
        assert_eq!(observer_peer.surface.as_deref(), Some(plan_c.surface.surface_id.as_str()));
        assert_eq!(u64::from(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_c, intent.as_bytes()).await.status), expected["observerStatus"].as_u64().unwrap());
        c.close(None).await.unwrap();
        drop(c);
        drop(b);
        stop_recovery_server(state, shutdown, server).await;
        assert_eq!(expected["sqliteReopen"], true);
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("reopen exact file directory");
        let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
        assert!(state.socket_grants.inner.lock().unwrap().records.is_empty());
        assert!(state.document_open_plans.inner.lock().unwrap().records.is_empty());
        for session in [&removed, &observer] {
            assert!(state.directory.authenticate_session(&SessionCapability::parse(&session.token).unwrap()).await.unwrap().is_some(), "membership removal must not be mistaken for session loss");
        }
        assert_eq!(state.directory.get_role(&scope.space_id, &removed.user_id).await.unwrap(), None);
        assert_eq!(state.directory.get_role(&scope.space_id, &observer.user_id).await.unwrap(), Some(SpaceRole::Author));
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
            let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
            assert_recovery_denied(denied, expected);
            let admitted = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_c, intent.as_bytes()).await;
            assert_eq!(u64::from(admitted.status), expected["observerStatus"].as_u64().unwrap(), "reopened observer {route}");
            match route {
                "execution-target/component" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES),
                "execution-target/descriptor" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES),
                "execution-target/manifest" => {
                    let manifest: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&admitted.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
                    manifest.validate().unwrap();
                    assert_eq!(manifest.scope, scope);
                }
                "open-plan" => {
                    let plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&admitted.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
                    plan.validate(now_ms() as u64).unwrap();
                    assert_eq!(plan.scope, scope);
                }
                _ => unreachable!(),
            }
        }
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn admin_intent_binding_wire_matrix_is_exact_sorted_and_self_deduplicated() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).unwrap();
    let principal = AdminPrincipalV1 {
        user_id: "admin".into(),
        auth_session_id: "admin-session".into(),
        authorization_generation: 1,
        identity_provider: "fixture".into(),
        identity_subject_digest: [0; 32],
        expires_at_ms: i64::MAX,
        correlation_id: "binding-fixture".into(),
        peer_class: "test",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    for row in fixture["bindings"].as_array().unwrap() {
        let intent: AdminIntentV1 = semio_framework_pack_json::from_json_str(row["intentJson"].as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("actual closed administrator intent wire");
        let keys = admin_intent_bindings(&principal, &intent).map(|bindings| {
            bindings
                .into_iter()
                .map(|binding| match binding {
                    SocketBindingKeyV1::User(id) => format!("user:{id}"),
                    SocketBindingKeyV1::Session(id) => format!("session:{id}"),
                    SocketBindingKeyV1::DirectorySpaceAuthority { space_id } => format!("space:{space_id}"),
                    SocketBindingKeyV1::Membership { user_id, space_id } => format!("membership:{user_id}/{space_id}"),
                    SocketBindingKeyV1::Share(id) => format!("share:{id}"),
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(serde_json::to_value(&keys).unwrap(), row["keys"], "exact sorted binding union: {}", row["name"]);
    }
}

#[test]
fn admin_short_effects_retain_principal_until_their_actual_side_effect() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin short effect fixture");
        let root = tempdir("admin-short-authority");
        std::fs::create_dir_all(&root).expect("physical directory parent");
        let path = root.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("physical directory");
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state_with_directory(root.join("db"), directory, 1024, 256)).await.expect("admin effect state open deadline");
        let physical = rusqlite::Connection::open(&path).expect("independent physical directory reader");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "admin-short-authority@example.test";
        let _headers = authorize_test_admin(&mut state, email).await;
        let owner = issue_test_session(&state, "admin-short-owner@example.test").await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["shortActions"].as_array().unwrap().iter().enumerate() {
            let admin = issue_test_session(&state, email).await;
            let target = issue_test_session(&state, &format!("admin-short-target-{index}@example.test")).await;
            let space = create_space_for_test(&state, &owner.user_id, "Short Authority", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let scope = DocumentScope::new(space, format!("admin-short-document-{index}"));
            announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
            let request_id = format!("request:admin-short-{index}");
            let sync_id = format!("sync:admin-short-{index}");
            let kick = Arc::new(tokio::sync::Notify::new());
            state.session_kicks.insert(sync_id.clone(), kick.clone());
            let action = row["action"].as_str().unwrap();
            let existing_share = if action == "revoke-share" { Some(state.directory.issue_share_token(&scope, 600, "fixture:existing-share").await.expect("existing target share")) } else { None };
            let intent = match action {
                "issue-share" => AdminIntentV1::IssueDocumentShare { request_id, scope: scope.clone(), ttl_secs: 600 },
                "revoke-share" => AdminIntentV1::RevokeDocumentShare { request_id, scope: scope.clone(), share_id: existing_share.as_ref().unwrap().record.id.clone(), reason_code: "test-revoke".into() },
                "revoke-user" => AdminIntentV1::RevokeUserSessions { request_id, user_id: target.user_id.clone(), reason_code: "test-revoke".into() },
                "kick" => AdminIntentV1::KickConnection { request_id, sync_session_id: sync_id.clone(), reason_code: "test-kick".into() },
                other => panic!("unknown short action {other}"),
            };
            let principal = authenticate_admin_principal(&state, &bearer_headers(&admin.token), None).await.expect("short action principal");
            let bindings = admin_intent_bindings(&principal, &intent).expect("short effect owns an authority union");
            assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "closed short action authority keys");
            let action_first = row["first"] == "action";
            *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), action_first));
            let command = tokio::spawn({
                let authorization = format!("Bearer {}", admin.token);
                let body = semio_framework_pack_json::to_json_string(&intent);
                async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("short action pause deadline").expect("short action pause").forget();
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = admin.token.clone();
                async move { post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("short action revoke attempt deadline").expect("short action revoke attempt").forget();
            if action_first {
                assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted {} must own the principal authority", row["name"]);
                for binding in bindings {
                    assert!(state.socket_binding_gates.gate(binding).try_write_owned().is_err(), "every short effect key stays owned through its physical side effect");
                }
            } else {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning short action revoke deadline").expect("winning short action revoke"), StatusCode::NO_CONTENT);
            }
            *gate.directory_command_pause_user.lock().unwrap() = None;
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), command).await.expect("short action completion deadline").expect("short action task");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("short action receipt");
            assert_eq!(receipt.state, if row["effect"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled }, "principal authority decides {}", row["name"]);
            if action_first {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing short action revoke deadline").expect("trailing short action revoke"), StatusCode::NO_CONTENT);
            }
            let effect: i64 = match action {
                "issue-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
                "revoke-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE id = ?1 AND revoked_at IS NOT NULL", [existing_share.as_ref().unwrap().record.id.as_str()], |row| row.get(0)).unwrap(),
                "revoke-user" => physical.query_row("SELECT count(*) FROM hub_auth_session WHERE user_id = ?1 AND revoked_at IS NOT NULL", [target.user_id.as_str()], |row| row.get(0)).unwrap(),
                "kick" => i64::from(tokio::time::timeout(std::time::Duration::from_millis(30), kick.notified()).await.is_ok()),
                _ => unreachable!(),
            };
            assert_eq!(effect, i64::from(row["effect"].as_bool().unwrap()), "exact physical effect: {}", row["name"]);
            let audit_count: i64 = physical.query_row("SELECT count(*) FROM hub_auth_audit WHERE correlation_id = ?1", [&receipt.correlation_id], |row| row.get(0)).unwrap();
            assert_eq!(audit_count, if action == "kick" { 0 } else { effect }, "no auth side-effect audit under revoked authority");
            let secret = receipt.result.as_ref().and_then(|result| result.share_token.as_ref());
            assert_eq!(secret.is_some(), row["secret"].as_bool().unwrap(), "plaintext share result follows the exact admitted effect");
            if let Some(secret) = secret {
                let capability = semio_hub_directory_contract::ShareCapability::parse(secret).expect("minted share capability");
                assert!(state.directory.authenticate_share(&scope, &capability).await.expect("minted share authentication"));
            }
            state.session_kicks.remove(&sync_id);
        }
        drop(physical);
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn admin_directory_commands_hold_exact_principal_without_confusing_space_role() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin authority fixture");
        let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("admin authority state open deadline");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "directory-admin-authority@example.test";
        let _headers = authorize_test_admin(&mut state, email).await;
        let owner = issue_test_session(&state, "directory-admin-target-owner@example.test").await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
            let admin = issue_test_session(&state, email).await;
            let headers = bearer_headers(&admin.token);
            let principal = authenticate_admin_principal(&state, &headers, None).await.expect("configured administrator");
            let space = create_space_for_test(&state, &owner.user_id, "Admin Target", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Author).await;
            let request_id = format!("request:admin-directory-authority-{index}");
            let name = format!("Admin Mutation {index}");
            let create = row["command"] == "create";
            let command_first = row["first"] == "command";
            let intent = if create {
                AdminIntentV1::CreateSpace { request_id: request_id.clone(), name: name.clone(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }
            } else {
                AdminIntentV1::RenameSpace { request_id: request_id.clone(), space_id: space.clone(), name: name.clone() }
            };
            let target = if create { admin_create_space_id(&request_id) } else { space.clone() };
            let before = state.directory.head_seq().await.expect("admin head before intent");
            *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), command_first));
            let mut command = tokio::spawn({
                let authorization = format!("Bearer {}", admin.token);
                let body = semio_framework_pack_json::to_json_string(&intent);
                async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("admin pause deadline").expect("admin pause").forget();
            let mut trailing_revoke = None;
            if row["transition"] == "session" {
                let mut revoke = tokio::spawn({
                    let state = state.clone();
                    let token = admin.token.clone();
                    async move { post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await }
                });
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("admin revoke attempt deadline").expect("admin revoke attempt").forget();
                if command_first {
                    assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted admin command retains the exact session");
                    assert!(state.socket_binding_gates.gate(SocketBindingKeyV1::User(principal.user_id.clone())).try_write_owned().is_err(), "admitted admin command also retains user-wide revocation authority");
                    trailing_revoke = Some(revoke);
                } else {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("admin revoke deadline").expect("admin revoke"), StatusCode::NO_CONTENT);
                }
            } else {
                upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Spectator).await;
                assert_eq!(state.directory.get_role(&space, &admin.user_id).await.unwrap(), Some(SpaceRole::Spectator));
            }
            *gate.directory_command_pause_user.lock().unwrap() = None;
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut command).await.expect("admin intent completion deadline").expect("admin intent task");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("admin authority receipt");
            let expected = if row["mutated"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled };
            assert_eq!(receipt.state, expected, "exact principal, not ordinary role, decides {}", row["name"]);
            if let Some(revoke) = trailing_revoke {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing admin revoke deadline").expect("trailing admin revoke"), StatusCode::NO_CONTENT);
            }
            let events = state.directory.events_since(before, 50).await.expect("admin authority durable events");
            let own_events: Vec<_> = events.iter().filter(|event| event.actor == principal.event_actor()).collect();
            assert_eq!(
                own_events.len(),
                if row["mutated"] == true {
                    if create {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                },
                "no mutation escapes a revoked principal"
            );
            let persisted = state.directory.get_space(&target).await.expect("admin target projection");
            assert_eq!(persisted.as_ref().is_some_and(|space| space.name == name), row["mutated"].as_bool().unwrap());
            if row["mutated"] == false {
                assert!(receipt.event_seq_first.is_none() && receipt.event_seq_last.is_none());
            }
        }

        let self_session = issue_test_session(&state, email).await;
        let authorization = format!("Bearer {}", self_session.token);
        let body = semio_framework_pack_json::to_json_string(&AdminIntentV1::RevokeUserSessions { request_id: "request:admin-directory-self-revoke".into(), user_id: self_session.user_id.clone(), reason_code: "test-self-revoke".into() });
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()))
            .await
            .expect("self-revocation cannot nest an administrator User fence");
        assert_eq!(response.status, 200);
        let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("self-revocation receipt");
        assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
        let capability = SessionCapability::parse(&self_session.token).unwrap();
        assert!(state.directory.authenticate_session(&capability).await.unwrap().is_none());
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn directory_global_message_bindings_decode_wire_without_indexing_unrelated_memberships() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
    let ledger = SocketGrantLedgerV1::default();
    let capability = SocketGrantCapability::mint().expect("global capability");
    let audience = SocketAudienceV1::Directory { auth_session_id: "session".into(), authorization_generation: 1 };
    let subject = SocketSubjectV1::Session { session_id: "session".into(), user_id: "recipient".into(), authorization_generation: 1, role: None, expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-recipient".into() };
    ledger.issue(&capability, audience.clone(), "hub.v1.global".into(), subject, 1, 9_000).expect("global issue");
    let pending = ledger.pending(&capability, &audience, 2).expect("pending global");
    let record = ledger.consume(&pending, 3).expect("consumed global");
    let (live_id, _) = ledger.register_live(&record).expect("global lease");
    let principal_bindings = vec![SocketBindingKeyV1::User("recipient".into()), SocketBindingKeyV1::Session("session".into())];
    for row in fixture["messages"].as_array().unwrap() {
        let message: DirectoryStreamMessage = semio_framework_pack_json::from_json_str(row["messageJson"].as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("wire message {}: {error:?}", row["name"]));
        assert_eq!(directory_stream_message_space(&message), row["spaceId"].as_str(), "wire-derived space");
        let bindings = directory_message_bindings(&record, &message);
        assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "complete transient union");
        let mut expected = principal_bindings.clone();
        if let Some(space_id) = row["spaceId"].as_str() {
            expected.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
            expected.push(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: space_id.to_owned() });
        }
        assert_eq!(bindings, expected, "sorted exact principal and recipient membership union");
        assert_eq!(record.bindings(), principal_bindings, "transient keys never become permanent global indices");
        let mut scoped = record.clone();
        scoped.audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new("scoped", "doc"));
        assert_eq!(directory_message_bindings(&scoped, &message), scoped.bindings(), "scoped audiences do not borrow unrelated scopes");
    }
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: "space-a".into() });
    ledger.invalidate_binding(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() });
    assert!(ledger.is_live(&record, &live_id), "space A revocation cannot invalidate a global lease for B");
    ledger.invalidate_binding(SocketBindingKeyV1::Session("session".into()));
    assert!(!ledger.is_live(&record, &live_id), "principal revocation remains terminal");
}

/// 🔑️ Law over the shared `🔑️directory-access-changed-v1` fixture: exactly the membership events naming the reader owe it
/// an `access-changed` frame — a grant or redemption only for a space this socket never delivered the creation of, every
/// removal — and each frame borrows no space authority (it is addressed to its reader alone).
#[test]
fn a_membership_event_naming_the_reader_owes_it_exactly_the_fixture_access_change() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔑️access-changed-v1/🔣️.json")).expect("access-changed fixture");
    let reader = fixture["readerUserId"].as_str().expect("fixture reader");
    for row in fixture["cases"].as_array().expect("fixture cases") {
        let message: DirectoryStreamMessage = semio_framework_pack_json::from_json_str(&row["message"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("fixture message {}: {error:?}", row["id"]));
        let created = row["createdOnSocket"].as_array().expect("created spaces").iter().map(|space| space.as_str().expect("space id").to_owned()).collect::<std::collections::BTreeSet<_>>();
        let expected = (!row["frame"].is_null()).then(|| semio_framework_pack_json::from_json_str::<DirectoryStreamMessage>(&row["frame"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture frame"));
        let owed = directory_access_change_for_reader(reader, &created, &message);
        assert_eq!(owed, expected, "{}", row["id"]);
        if let Some(frame) = owed {
            assert_eq!(directory_stream_message_space(&frame), None, "an access change borrows no space authority: {}", row["id"]);
            assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&frame)).expect("frame JSON"), row["frame"], "wire shape: {}", row["id"]);
        }
    }
}

/// 🔑️ Live on a global directory socket: a reader added to a space created before its socket is told its access
/// changed right after the membership event (so its Home re-reads the space's earlier events); a space it creates on
/// that socket is not an access change (it read the space from its first event); its removal is.
#[test]
fn a_reader_added_to_an_older_space_is_told_its_access_changed_and_its_own_new_space_is_not() {
    run_socket_test(|| async {
        let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("access-changed state open deadline");
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let owner = issue_test_session(&state, "access-owner@example.test").await;
        let reader = issue_test_session(&state, "access-reader@example.test").await;
        let older = create_space_for_test(&state, &owner.user_id, "Older", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&reader.token), State(state.clone()), Bytes::new()).await.expect("reader grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut socket, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &receipt.grant)).await.expect("reader socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("reader hello");
        upsert_member_for_test(&state, &older, "access-reader@example.test", DirectorySpaceRole::Author).await;
        assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id, user_id, .. } if *space_id == older && *user_id == reader.user_id)), "the membership event itself");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: older.clone(), change: os_directory::DirectoryAccessChange::Granted }, "then the reader's access change");
        let own = create_space_for_test(&state, &reader.user_id, "Own", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let mut own_frames = Vec::new();
        loop {
            let message = next_directory_message(&mut socket).await;
            let settled = matches!(&message, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id, .. } if *space_id == own));
            own_frames.push(message);
            if settled {
                break;
            }
        }
        assert!(own_frames.iter().any(|message| matches!(message, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } if *space_id == own))), "the creator reads its space from its first event: {own_frames:?}");
        state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#test", owner.user_id) }, DirectoryCommand::RemoveMember { space_id: older.clone(), user_id: reader.user_id.clone() }).await.expect("remove reader");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: older.clone(), change: os_directory::DirectoryAccessChange::Revoked }, "no AccessChanged for its own space; then its removal from the older one");
        socket.close(None).await.expect("close reader socket");
        stop_recovery_server(state, shutdown, server).await;
    });
}

/// 🗑️ Live on a global directory socket: a member of a space its owner deletes is told its access was revoked (its own
/// `member.removed` of the deletion), so its Home re-reads the directory from the origin without the space.
#[test]
fn a_member_of_a_deleted_space_is_told_its_access_was_revoked() {
    run_socket_test(|| async {
        let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("deleted-space state open deadline");
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let owner = issue_test_session(&state, "deleted-owner@example.test").await;
        let reader = issue_test_session(&state, "deleted-reader@example.test").await;
        let doomed = create_space_for_test(&state, &owner.user_id, "Doomed", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &doomed, "deleted-reader@example.test", DirectorySpaceRole::Spectator).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&reader.token), State(state.clone()), Bytes::new()).await.expect("reader grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut socket, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &receipt.grant)).await.expect("reader socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("reader hello");
        state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#test", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: doomed.clone() }).await.expect("delete space");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: doomed.clone(), change: os_directory::DirectoryAccessChange::Revoked }, "the deletion revokes the member's access");
        socket.close(None).await.expect("close reader socket");
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn directory_global_socket_delivery_and_revocation_share_one_transient_authority_order() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
        let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("global directory state open deadline");
        let gate = Arc::new(TestLiveGate::default());
        gate.socket_directory_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
            let owner_email = format!("global-owner-{index}@example.test");
            let recipient_email = format!("global-recipient-{index}@example.test");
            let owner = issue_test_session(&state, &owner_email).await;
            let recipient = issue_test_session(&state, &recipient_email).await;
            let space_a = create_space_for_test(&state, &owner.user_id, "A", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let space_b = create_space_for_test(&state, &owner.user_id, "B", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            for space in [&space_a, &space_b] {
                upsert_member_for_test(&state, space, &recipient_email, DirectorySpaceRole::Spectator).await;
            }
            let receipt = issue_directory_socket_grant(bearer_headers(&recipient.token), State(state.clone()), Bytes::new()).await.expect("global grant").0;
            let since = state.directory.head_seq().await.expect("global directory head");
            let url = format!("ws://{addr}/directory/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("global directory socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("global socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_directory_admitted.acquire()).await.expect("global admission deadline").expect("global admission").forget();
            gate.socket_directory_release.add_permits(1);
            let delivery_first = row["first"] == "send";
            *gate.socket_global_send_pause.lock().expect("global pause lock") = Some((recipient.user_id.clone(), if delivery_first { 2 } else { 1 }));
            let event_a = state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_a.clone(), name: format!("A-{index}") })
                .await
                .expect("durable A event")
                .0
                .into_iter()
                .next()
                .expect("A event");
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_global_send_admitted.acquire()).await.expect("message pause deadline").expect("message pause").forget();
            let membership = row["revocation"] == "membership";
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = if membership { owner.token.clone() } else { recipient.token.clone() };
                let command = DirectoryCommand::RemoveMember { space_id: space_a.clone(), user_id: recipient.user_id.clone() };
                async move {
                    if membership {
                        post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await.status
                    } else {
                        post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await.as_u16()
                    }
                }
            });
            let attempted = if membership { &gate.directory_command_attempted } else { &gate.socket_session_revoke_attempted };
            tokio::time::timeout(std::time::Duration::from_secs(5), attempted.acquire()).await.expect("revocation fence attempt deadline").expect("revocation fence attempt").forget();
            if delivery_first {
                assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted delivery must own the same authority keys as revocation: {}", row["name"]);
            } else {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning revocation deadline").expect("winning revocation"), if membership { 202 } else { 204 });
            }
            *gate.socket_global_send_pause.lock().expect("clear global pause") = None;
            gate.socket_global_send_release.add_permits(1);
            if row["a"] == true {
                assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if *event == event_a), "one exact A event wins before revocation");
            }
            if delivery_first {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing revocation deadline").expect("trailing revocation"), if membership { 202 } else { 204 });
            }
            if row["b"] == true {
                assert_eq!(
                    next_directory_message(&mut socket).await,
                    DirectoryStreamMessage::AccessChanged { space_id: space_a.clone(), change: os_directory::DirectoryAccessChange::Revoked },
                    "the removed reader is told its access to A moved before any later frame: {}",
                    row["name"]
                );
                let event_b = state
                    .directory_service
                    .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_b.clone(), name: format!("B-{index}") })
                    .await
                    .expect("durable B event")
                    .0
                    .into_iter()
                    .next()
                    .expect("B event");
                assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if *event == event_b), "removed A membership neither leaks A nor closes unrelated B");
                socket.close(None).await.expect("close unaffected global socket");
            } else {
                assert_eq!(u64::from(next_close_code(&mut socket, false).await), row["close"].as_u64().unwrap(), "revoked principal closes without a later message");
            }
        }
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn socket_directory_revoke_after_admission_suppresses_replay_without_deadlock() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        gate.socket_directory_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let token = seed_author_token(&state).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone()), Bytes::new()).await.expect("issue directory grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since=0");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_directory_admitted.acquire()).await.expect("directory admission deadline").expect("directory admission");
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), post_session_sign_out(bearer_headers(&token), State(state), Bytes::new())).await.expect("bounded revoke"), StatusCode::NO_CONTENT,);
        gate.socket_directory_release.add_permits(1);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "no replay text crosses a winning revoke");
    });
}

#[tokio::test]
async fn socket_directory_visibility_requires_membership_even_for_public_spaces() {
    let state = test_state().await;
    let token = seed_author_token(&state).await;
    let capability = SessionCapability::parse(&token).expect("session capability");
    let session = state.directory.authenticate_session(&capability).await.expect("authenticate session").expect("active session");
    let other = issue_test_session(&state, "public-owner@example.com").await;
    let public_space = create_space_for_test(&state, &other.user_id, "Public Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
    let event = state.directory.events_since(0, 100).await.expect("directory events").into_iter().find(|event| event.space_id.as_deref() == Some(public_space.as_str())).expect("public-space event");
    let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
    let record = SocketGrantRecordV1 {
        selector: "visibility".into(),
        key: SocketGrantKeyV1::Capability([0; 32]),
        audience,
        actor_id: "hub.v1.visibility".into(),
        subject: SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: None, expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id },
        document_plan: None,
        issued_at_ms: session.issued_at,
        expires_at_ms: session.expires_at,
        state: SocketGrantStateV1::Consumed,
    };
    assert_eq!(socket_directory_membership_visibility(&state, &record, &DirectoryStreamMessage::Event { event: Box::new(event) }).await, SocketBindingValidityV1::Unauthorized);
}

fn assert_public_projection_has_no_private_keys(value: &serde_json::Value) {
    const FORBIDDEN: &[&str] = &[
        "ownerUserId",
        "role",
        "activeConnections",
        "connections",
        "presence",
        "members",
        "invites",
        "email",
        "userId",
        "displayName",
        "actor",
        "hlc",
        "cursor",
        "headSeq",
        "commitSeq",
        "epoch",
        "bootstrapVersion",
        "bootstrapFrontier",
        "bootstrapSnapshotHash",
        "checkpointId",
        "storageKey",
    ];
    match value {
        serde_json::Value::Object(entries) => {
            for (key, value) in entries {
                assert!(!FORBIDDEN.contains(&key.as_str()), "public projection disclosed forbidden key {key}");
                assert_public_projection_has_no_private_keys(value);
            }
        }
        serde_json::Value::Array(entries) => entries.iter().for_each(assert_public_projection_has_no_private_keys),
        _ => {}
    }
}

/// 🪞️ Strips the three fields `DirectorySpaceAdministrationPageV1` binds to its individual caller —
/// `sessionBindingSha256`, `authorizationGeneration` and `receiptSha256` — so two readers of the same
/// public space can be compared on the projection they are meant to share.
fn public_projection_without_caller_binding(value: &serde_json::Value) -> serde_json::Value {
    let mut stripped = value.clone();
    let entries = stripped.as_object_mut().expect("administration page object");
    for key in ["sessionBindingSha256", "authorizationGeneration", "receiptSha256"] {
        assert!(entries.remove(key).is_some(), "administration page carries its caller binding field {key}");
    }
    stripped
}

//#region 🏛️SpaceAdministration
/// 🏛️ An author receives both bounded windows, a canonical receipt over the exact response
/// bytes, and the server's own capability flags — and no credential column anywhere.
#[tokio::test]
async fn space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-author@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Administered", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-guest@example.invalid", DirectorySpaceRole::Spectator).await;
    state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
        .await
        .expect("author invite fixture");
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", author.token);
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(response.status, 200);
    let canonical = std::str::from_utf8(&response.body).expect("administration page UTF-8").to_string();
    assert!(canonical.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES);
    let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical author page");
    assert_eq!(page.space_id(), space);
    let DirectorySpaceAdministrationPageV1::Author { members, invites, capabilities, .. } = &page else { panic!("author projection") };
    assert_eq!(members.rows.len(), 2);
    assert!(members.rows.windows(2).all(|pair| pair[0].user_id < pair[1].user_id), "member rows are keyset-ordered");
    assert!(members.rows.iter().any(|row| row.owner), "the owner row is marked so removal can be disabled before dispatch");
    assert_eq!(invites.rows.len(), 1);
    assert!(capabilities.remove_member && capabilities.create_invite);
    for secret in ["selector", "secretDigest", "passwordHash", "ssoSubject", "ssoProvider", "inviteToken"] {
        assert!(!canonical.contains(secret), "administration page leaked {secret}");
    }
}

/// 🛂️ A spectator receives the member shape only: invites and capability flags are structurally
/// absent, so no renderer can mis-gate them into existence.
#[tokio::test]
async fn space_administration_page_v1_route_denies_a_spectator_the_author_windows() {
    let _watchdog = LawHangWatchdogV1::arm("space_administration_page_v1_route_denies_a_spectator_the_author_windows");
    let state = bounded_law_step("spectator law: hub state open", test_state()).await;
    let author = bounded_law_step("spectator law: author session", issue_test_session(&state, "administration-owner@example.invalid")).await;
    let spectator = bounded_law_step("spectator law: spectator session", issue_test_session(&state, "administration-spectator@example.invalid")).await;
    let outsider = bounded_law_step("spectator law: outsider session", issue_test_session(&state, "administration-outsider@example.invalid")).await;
    let space = bounded_law_step("spectator law: space creation", create_space_for_test(&state, &author.user_id, "Spectated", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private)).await;
    bounded_law_step("spectator law: spectator membership", upsert_member_for_test(&state, &space, "administration-spectator@example.invalid", DirectorySpaceRole::Spectator)).await;
    bounded_law_step(
        "spectator law: author invite fixture",
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-spectator-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 }),
    )
    .await
    .expect("author invite fixture");
    let addr = spawn_server(state).await;
    let spectator_authorization = format!("Bearer {}", spectator.token);
    let response = bounded_law_step("spectator law: spectator page read", raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", spectator_authorization.as_str())])).await;
    assert_eq!(response.status, 200);
    let canonical = std::str::from_utf8(&response.body).expect("spectator page UTF-8").to_string();
    let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical member page");
    assert!(matches!(page, DirectorySpaceAdministrationPageV1::Member { .. }));
    assert!(page.capabilities().is_none());
    assert!(!canonical.contains("\"invites\"") && !canonical.contains("\"capabilities\""));
    let outsider_authorization = format!("Bearer {}", outsider.token);
    assert_eq!(bounded_law_step("spectator law: outsider page read", raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", outsider_authorization.as_str())])).await.status, 404);
    assert_eq!(bounded_law_step("spectator law: anonymous page read", raw_http_get(addr, &format!("/directory/spaces/{space}"), &[])).await.status, 404);
}

/// 🧯️ A membership removal takes effect on the very next page read: the response is a denial with
/// no page bytes at all, so no member or invite row can leak past the revocation.
#[tokio::test]
async fn space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-remover@example.invalid").await;
    let removed = issue_test_session(&state, "administration-removed@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Revoked", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-removed@example.invalid", DirectorySpaceRole::Spectator).await;
    let service = state.directory_service.clone();
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", removed.token);
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
    service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-remove-law", author.user_id) }, DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: removed.user_id.clone() })
        .await
        .expect("member removal");
    let denied = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(denied.status, 404, "a removed member cannot even enumerate the private space");
    assert!(!String::from_utf8_lossy(&denied.body).contains("administration-remover@example.invalid"));
}

/// 🛡️ The query grammar is exact and every cursor is MAC-bound: a foreign, tampered, or
/// differently shaped cursor is a 400 with no page bytes, never a page for another identity.
#[tokio::test]
async fn space_administration_page_v1_route_rejects_a_noncanonical_query_and_a_foreign_cursor() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-cursor@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Cursored", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", author.token);
    for query in ["?cursor=", "?cursor=a&cursor=b", "?cursor=%6d", "?cursor=m+1", "?section=members", "?cursor=m.7573.zz"] {
        let response = raw_http_get(addr, &format!("/directory/spaces/{space}{query}"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(response.status, 400, "query {query} must be refused before any read");
        assert!(response.body.is_empty() || !String::from_utf8_lossy(&response.body).contains("receiptSha256"));
    }
    let forged = format!("m.{}.{}", os_directory::io::binary::artifact_hash::hex_lower(b"user-forged"), "ab".repeat(32));
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}?cursor={forged}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(response.status, 400, "a cursor with a forged MAC is refused");
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
}
//#endregion 🏛️SpaceAdministration

#[test]
fn space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404() {
    run_socket_test(|| async {
        let state = test_state().await;
        let author = issue_test_session(&state, "public-author@example.invalid").await;
        let spectator = issue_test_session(&state, "public-spectator@example.invalid").await;
        let outsider = issue_test_session(&state, "public-outsider@example.invalid").await;
        let public_space = create_space_for_test(&state, &author.user_id, "Discoverable", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        upsert_member_for_test(&state, &public_space, "public-spectator@example.invalid", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, &public_space, "catalog-document").await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#route-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: public_space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
            .await
            .expect("author invite fixture");
        let private_space = create_space_for_test(&state, &author.user_id, "Private", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state).await;

        let anonymous = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[]).await;
        assert_eq!(anonymous.status, 200);
        let anonymous: serde_json::Value = serde_json::from_slice(&anonymous.body).expect("anonymous public detail JSON");
        assert_eq!(anonymous["access"], "public");
        assert_eq!(anonymous["schema"], DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA);
        assert_eq!(anonymous["space"]["visibility"], "public");
        assert_eq!(anonymous["documents"]["rows"][0]["documentId"], "catalog-document");
        assert!(anonymous["documents"]["rows"][0].get("descriptor").is_none());
        assert_public_projection_has_no_private_keys(&anonymous);

        let outsider_authorization = format!("Bearer {}", outsider.token);
        let public_nonmember = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
        assert_eq!(public_nonmember.status, 200);
        let nonmember: serde_json::Value = serde_json::from_slice(&public_nonmember.body).expect("nonmember public detail");
        assert_eq!(
            public_projection_without_caller_binding(&nonmember),
            public_projection_without_caller_binding(&anonymous),
            "an authenticated nonmember reads exactly the anonymous public projection"
        );
        assert_eq!(anonymous["sessionBindingSha256"], "0".repeat(64), "the anonymous public projection is bound to no session");
        assert_eq!(anonymous["authorizationGeneration"], 0, "the anonymous public projection carries no authorization generation");
        assert_ne!(nonmember["sessionBindingSha256"], anonymous["sessionBindingSha256"], "an authenticated reader's public page is bound to that reader's own session");
        assert!(nonmember["authorizationGeneration"].as_u64().expect("nonmember authorization generation") >= 1, "an authenticated reader's public page carries that reader's generation");
        assert_public_projection_has_no_private_keys(&nonmember);
        let private_nonmember = raw_http_get(addr, &format!("/directory/spaces/{private_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
        assert_eq!(private_nonmember.status, 404);

        let public_document_status = format!("/spaces/{public_space}/documents/catalog-document");
        assert_eq!(raw_http_get(addr, &public_document_status, &[]).await.status, 401, "public discovery is not document-currentness authority");
        assert_eq!(raw_http_get(addr, &public_document_status, &[("Authorization", outsider_authorization.as_str())]).await.status, 401);
        let blob = format!("/spaces/{public_space}/blobs/{}", "11".repeat(32));
        assert_eq!(raw_http_request(addr, "GET", &blob, &[], &[]).await.status, 401, "public discovery is not blob read authority");
        assert_eq!(raw_http_request(addr, "HEAD", &blob, &[], &[]).await.status, 401, "public discovery is not blob existence authority");
        assert_eq!(raw_http_request(addr, "PUT", &blob, &[], b"private").await.status, 401, "public discovery is not blob write authority");
        assert_eq!(raw_http_request(addr, "GET", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "HEAD", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "PUT", &blob, &[("Authorization", outsider_authorization.as_str())], b"private").await.status, 401);

        let spectator_authorization = format!("Bearer {}", spectator.token);
        let member = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", spectator_authorization.as_str())]).await;
        assert_eq!(member.status, 200);
        let member: serde_json::Value = serde_json::from_slice(&member.body).expect("member detail");
        assert_eq!(member["access"], "member");
        assert!(member.get("members").is_some());
        assert!(member.get("invites").is_none(), "a member page structurally omits invites");
        assert!(member.get("capabilities").is_none(), "a member page structurally omits capability flags");
        assert_eq!(member["documents"]["rows"][0]["headSeq"], 0);

        let author_authorization = format!("Bearer {}", author.token);
        let authored = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await;
        assert_eq!(authored.status, 200);
        let authored: serde_json::Value = serde_json::from_slice(&authored.body).expect("author detail");
        assert_eq!(authored["access"], "author");
        assert_eq!(authored["space"]["role"], "author");
        assert_eq!(authored["invites"]["rows"].as_array().map(Vec::len), Some(1));
        assert_eq!(authored["capabilities"]["removeMember"], true);
        let authored_bytes = std::str::from_utf8(&raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await.body).expect("author page UTF-8").to_string();
        assert!(!authored_bytes.contains("selector") && !authored_bytes.contains("secretDigest") && !authored_bytes.contains("passwordHash"));
        assert_eq!(DirectorySpaceAdministrationPageV1::parse_canonical_json(&authored_bytes).map(|page| page.space_id().to_string()), Ok(public_space.clone()));

        let list = raw_http_get(addr, "/directory/spaces", &[]).await;
        assert_eq!(list.status, 200);
        let list: serde_json::Value = serde_json::from_slice(&list.body).expect("public list");
        let rows = list.as_array().expect("list rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["access"], "public");
        assert_public_projection_has_no_private_keys(&rows[0]);
    });
}

#[test]
fn space_public_boundary_public_event_route_denies_raw_directory_events() {
    run_socket_test(|| async {
        let state = test_state().await;
        let outsider = issue_test_session(&state, "event-outsider@example.invalid").await;
        let since = state.directory.head_seq().await.expect("pre-public head");
        let owner = issue_test_session(&state, "event-owner@example.invalid").await;
        let public_space = create_space_for_test(&state, &owner.user_id, "Event Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        announce_document_for_test(&state, &public_space, "event-document").await;
        let addr = spawn_server(state).await;
        let anonymous = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[]).await;
        assert_eq!(anonymous.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&anonymous.body).expect("anonymous events"), serde_json::json!([]));
        let authorization = format!("Bearer {}", outsider.token);
        let nonmember = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(nonmember.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&nonmember.body).expect("nonmember events"), serde_json::json!([]));
    });
}

#[test]
fn space_public_boundary_real_socket_denies_public_raw_events_and_member_telemetry() {
    run_socket_test(|| async {
        let state = test_state().await;
        let outsider = issue_test_session(&state, "socket-public-outsider@example.invalid").await;
        let owner = issue_test_session(&state, "socket-public-owner@example.invalid").await;
        let public_space = create_space_for_test(&state, &owner.user_id, "Socket Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let since = state.directory.head_seq().await.expect("head");
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-replay-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Replay".into() })
            .await
            .expect("replayed raw public event");
        let receipt = issue_directory_socket_grant(bearer_headers(&outsider.token), State(state.clone()), Bytes::new()).await.expect("outsider directory grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("public outsider directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free directory hello");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-live-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Live".into() })
            .await
            .expect("live raw public event");
        state.directory_service.publish(DirectoryStreamMessage::Connection {
            phase: DirectoryConnectionPhase::Opened,
            connection: ConnectionView {
                sync_session_id: "private-sync".into(),
                space_id: public_space,
                document_id: "private-document".into(),
                surface: "private-surface".into(),
                actor: "private-actor".into(),
                user_id: Some(owner.user_id),
                email: Some("socket-public-owner@example.invalid".into()),
                role: DirectorySpaceRole::Author,
                connected_at_ms: 1,
                presence_known: true,
            },
        });
        let head = state.directory.head_seq().await.expect("post-event head");
        assert!(head > since);
        state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: head });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(250), socket.next()).await.is_err(), "public nonmember received a raw event, member telemetry, or global progress cursor");
    });
}

#[test]
fn admin_intent_wire_taxonomy_rejects_generic_and_unknown_commands() {
    let valid = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private"}"#;
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(valid, semio_framework_pack_json::JsonMemberPolicy::Reject).is_ok());
    let generic = r#"{"kind":"directory","requestId":"request:one","command":{"kind":"create-space","name":"Studio","spaceKind":"studio","visibility":"private"}}"#;
    let forbidden = r#"{"kind":"announce-document","requestId":"request:one","descriptor":{}}"#;
    let unknown = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private","actor":"admin"}"#;
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(generic, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(forbidden, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(unknown, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

#[test]
fn admin_document_cursor_is_principal_route_and_exact_page_bound() {
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
    let cursor = admin_cursor_encode_scoped(&cursor_key, &principal, 5, Some("space:one"), ADMIN_PAGE_MAX).expect("document cursor");
    assert_eq!(cursor.len(), 84);
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:one"), Some(&cursor)), Ok(ADMIN_PAGE_MAX));
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:two"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_cursor_decode(&cursor_key, &principal, 2, Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    let mut other_principal = principal.clone();
    other_principal.auth_session_id = "session:other".into();
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &other_principal, 5, Some("space:one"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX) }), Ok(ADMIN_PAGE_MAX));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(0) }), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX + 1) }), Err(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn retained_short_admin_shutdown_drains_before_bounded_abort_and_receipt_reconciliation_is_exact() {
    let cancelled = AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    };
    assert!(cancelled.request_cancel());
    assert!(!cancelled.admit_effect(), "cancellation linearized before admission refuses the effect");
    let admitted = AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    };
    assert!(admitted.admit_effect());
    assert!(!admitted.request_cancel(), "admission linearized before cancellation cannot claim rollback");
    ADMIN_SECRET_WIPE_BYTES.store(0, std::sync::atomic::Ordering::SeqCst);
    let invite = "invite.v1.selector.secret".to_string();
    let share = "share.v1.selector.secret".to_string();
    let expected_wipes = invite.len() + share.len();
    drop(AdminIntentSecretResult::Invite(invite));
    drop(AdminIntentSecretResult::Share(share));
    assert_eq!(ADMIN_SECRET_WIPE_BYTES.load(std::sync::atomic::Ordering::SeqCst), expected_wipes);

    let state = test_state().await;
    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:retained".into(),
        peer_class: "admin-rest",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    let metadata = AdminIntentMetadata { intent_kind: "delete-space", target_kind: "space", target_id: "space:one".into(), reason_code: None };
    let mut accepted = new_admin_audit_fact(&principal, "request:stale-accepted", &"11".repeat(32), "operation:stale-accepted", &metadata, "accepted", None, "accepted");
    accepted.occurred_at = now_ms() - 60_000;
    state.directory.append_admin_operation_audit(&accepted).await.expect("stale accepted audit");
    let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&accepted.request_id).await.expect("stale audit read")).await.expect("stale reconciliation");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].fact.phase, "accepted", "age alone cannot prove cancellation or rollback");
    let unresolved = admin_audit_visibility(admin_audit_receipt(&rows).expect("stale accepted receipt"), false);
    assert_eq!(unresolved.state, AdminIntentStateV1::Indeterminate);
    assert_eq!(unresolved.outcome.code, "admin-effect-outcome-indeterminate");
    assert_eq!(admin_audit_visibility(admin_audit_receipt(&rows).expect("live accepted receipt"), true).state, AdminIntentStateV1::Accepted);

    let owner = issue_test_session(&state, "retained-reconcile-owner@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Receipt Reconciliation", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let scope = DocumentScope::new(&space, "receipt-reconciliation-document");
    announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
    let committed_digest = "22".repeat(32);
    let committed_metadata = AdminIntentMetadata { intent_kind: "issue-document-share", target_kind: "document", target_id: format!("{}/{}", scope.space_id, scope.document_id), reason_code: None };
    let mut committed = new_admin_audit_fact(&principal, "request:committed-accepted", &committed_digest, "operation:committed-accepted", &committed_metadata, "accepted", None, "accepted");
    committed.occurred_at = now_ms() - 60_000;
    state.directory.append_admin_operation_audit(&committed).await.expect("committed acceptance audit");
    let effect = NewAdminOperationEffectReceiptV1 { operation_id: committed.operation_id.clone(), intent_digest: committed.intent_digest.clone(), committed_at: now_ms(), outcome_code: "share-issued".into() };
    let AdminEffectCommitV1::Applied(issued) = state.directory.issue_share_token_as_with_admin_effect(&scope, 600, Some(&principal.user_id), &principal.correlation_id, &effect).await else {
        panic!("atomic share and effect receipt");
    };
    drop(issued);
    assert!(tokio::time::timeout(std::time::Duration::ZERO, std::future::pending::<()>()).await.is_err(), "effect commit can precede acknowledgement deadline");
    assert!(state.directory.admin_operation_effect_receipt(&committed.operation_id, &"33".repeat(32)).await.expect("mismatched receipt query").is_none());
    let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&committed.request_id).await.expect("committed audit read")).await.expect("factual receipt reconciliation");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].fact.phase, "succeeded");
    assert_eq!(rows[1].fact.outcome_code, "share-issued");
    assert_eq!(rows[1].fact.event_seq_first, None);
    assert_eq!(rows[1].fact.event_seq_last, None);

    let owner = AdminOperationTaskOwner::new(std::time::Duration::from_secs(1));
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    let drained = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(owner
        .spawn("operation:drain".into(), runtime.clone(), {
            let drained = drained.clone();
            let runtime = runtime.clone();
            async move {
                while !runtime.progress().cancel_requested {
                    tokio::task::yield_now().await;
                }
                drained.store(true, std::sync::atomic::Ordering::Release);
            }
        })
        .is_ok());
    owner.shutdown().await;
    assert!(drained.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(owner.task_count(), 0);
    assert!(owner.spawn("operation:after-close".into(), runtime.clone(), async {}).is_err(), "shutdown refuses later operation tasks");

    struct DropSignal(Option<tokio::sync::oneshot::Sender<()>>);
    impl Drop for DropSignal {
        fn drop(&mut self) {
            if let Some(signal) = self.0.take() {
                let _ = signal.send(());
            }
        }
    }
    let aborting = AdminOperationTaskOwner::new(std::time::Duration::from_millis(10));
    let abort_runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_ADMITTED),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel();
    assert!(aborting
        .spawn("operation:bounded-abort".into(), abort_runtime, async move {
            let _signal = DropSignal(Some(dropped_tx));
            std::future::pending::<()>().await;
        })
        .is_ok());
    aborting.shutdown().await;
    tokio::time::timeout(std::time::Duration::from_secs(1), dropped_rx).await.expect("aborted task drop deadline").expect("aborted task dropped");
    assert_eq!(aborting.task_count(), 0);
}

#[tokio::test]
async fn admin_rebuild_slots_are_atomic_and_abort_closes_once() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    directory.seed().await.expect("seed directory");
    let directory = Arc::new(HubDirectories::from(directory));
    let operations = Arc::new(ShardedMap::new());
    let operation_slots = Arc::new(tokio::sync::Semaphore::new(64));
    let barrier = Arc::new(tokio::sync::Barrier::new(129));
    let release = Arc::new(tokio::sync::Notify::new());
    let acquired = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut contenders = Vec::new();
    for _ in 0..128 {
        let barrier = barrier.clone();
        let release = release.clone();
        let acquired = acquired.clone();
        let slots = operation_slots.clone();
        contenders.push(tokio::spawn(async move {
            barrier.wait().await;
            if let Ok(_permit) = slots.try_acquire_owned() {
                acquired.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                release.notified().await;
            }
        }));
    }
    barrier.wait().await;
    for _ in 0..128 {
        if acquired.load(std::sync::atomic::Ordering::Acquire) == 64 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(acquired.load(std::sync::atomic::Ordering::Acquire), 64);
    assert_eq!(operation_slots.available_permits(), 0);
    release.notify_waiters();
    for contender in contenders {
        contender.await.expect("slot contender");
    }
    assert_eq!(operation_slots.available_permits(), 64);

    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:admin".into(),
        peer_class: "admin-rest",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    let metadata = AdminIntentMetadata { intent_kind: "rebuild-directory-projections", target_kind: "directory", target_id: "directory".into(), reason_code: None };
    let request_id = "request:abort";
    let digest = "11".repeat(32);
    let operation_id = "operation:abort";
    let accepted = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "accepted", None, "accepted");
    directory.append_admin_operation_audit(&accepted).await.expect("accepted audit");
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + std::time::Duration::from_secs(10),
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    operations.insert(operation_id.into(), runtime);
    let cleanup = AdminOperationCleanup { operations: operations.clone(), operation_id: operation_id.into(), _permit: operation_slots.clone().try_acquire_owned().expect("cleanup slot") };
    let task = tokio::spawn(async move {
        let _cleanup = cleanup;
        std::future::pending::<()>().await;
    });
    tokio::task::yield_now().await;
    task.abort();
    let _ = task.await;
    let mut rows = Vec::new();
    for _ in 0..128 {
        rows = directory.admin_operation_audit_for_request(request_id).await.expect("operation audit");
        if operations.get_cloned(operation_id).is_none() {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].fact.phase, "accepted");
    assert_eq!(rows[0].fact.outcome_code, "accepted");
    assert!(operations.get_cloned(operation_id).is_none());
    assert_eq!(operation_slots.available_permits(), 64);
    let later = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "succeeded", None, "late-success");
    assert_eq!(directory.append_admin_operation_audit(&later).await.expect("factual late terminal").fact.phase, "succeeded", "task abortion cannot invent rollback before the factual result is known");
}

async fn append_directory_page_test_events(state: &HubState, rows: &[(String, String)]) -> Vec<DirectoryEvent> {
    let events = rows
        .iter()
        .map(|(space_id, name)| semio_hub_directory_contract::NewDirectoryEvent {
            hlc: os_directory::Hlc { physical_ms: now_ms(), logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: format!("system:event-page:{space_id}") },
            space_id: Some(space_id.clone()),
            user_id: None,
            body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: space_id.clone(), name: name.clone() },
        })
        .collect::<Vec<_>>();
    state.directory.append_events(&events).await.expect("append event-page fixture")
}

fn event_page_authorization(token: &str) -> String {
    format!("Bearer {token}")
}

fn parse_directory_command_receipt_for_test(response: &RawHttpResponse, request_id: &str, command: &DirectoryCommand) -> DirectoryCommandReceiptV1 {
    let canonical = std::str::from_utf8(&response.body).expect("receipt UTF-8");
    DirectoryCommandReceiptV1::parse_canonical_json(canonical, &DirectoryCommandRequestV1::new(request_id, command.clone())).expect("canonical receipt")
}

#[tokio::test]
async fn directory_command_authority_revalidates_after_durable_revocation_before_fence() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["appended"] == 0) {
        let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("authority state open deadline");
        let owner = issue_test_session(&state, "authority-owner@example.com").await;
        let author = issue_test_session(&state, "authority-author@example.com").await;
        let target = issue_test_session(&state, "authority-target@example.com").await;
        let space = create_space_for_test(&state, &owner.user_id, "Authority Race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
        let gate = Arc::new(TestLiveGate::default());
        *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), false));
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
        let request_id = "a00102030405060708090a0b0c0d0e0f";
        let pending = {
            let token = author.token.clone();
            let command = command.clone();
            tokio::spawn(async move { post_directory_command_for_test(addr, &token, request_id, command).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        match row["revocation"].as_str().unwrap() {
            "demote" | "remove" => {
                let revocation = if row["revocation"] == "demote" {
                    DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator }
                } else {
                    DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: author.user_id.clone() }
                };
                assert_eq!(post_directory_command_for_test(addr, &owner.token, "b00102030405060708090a0b0c0d0e0f", revocation).await.status, 202);
            }
            "session" => assert_eq!(post_session_sign_out(bearer_headers(&author.token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT),
            _ => unreachable!(),
        }
        let head = state.directory.head_seq().await.unwrap();
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["id"]);
        assert!(response.body.is_empty());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), Some(SpaceRole::Spectator));
        let claim = NewDirectoryCommandReceipt { actor_user_id: author.user_id.clone(), request_id: request_id.into(), command_sha256: directory_command_sha256(&command), result_kind: directory_command_result_kind(&command), claimed_at: now_ms() };
        assert!(matches!(state.directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Claimed(_)), "denied command must leave no durable claim");
        state.directory.release_directory_command_receipt(&author.user_id, request_id, &claim.command_sha256).await.unwrap();
        stop_recovery_server(state, shutdown, server).await;
    }
}

#[tokio::test]
async fn directory_command_authority_holds_admitted_command_until_receipt_before_demotion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
    let mut state = test_state().await;
    let owner = issue_test_session(&state, "authority-owner@example.com").await;
    let author = issue_test_session(&state, "authority-author@example.com").await;
    let target = issue_test_session(&state, "authority-target@example.com").await;
    let space = create_space_for_test(&state, &owner.user_id, "Authority Order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
    let gate = Arc::new(TestLiveGate::default());
    *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), true));
    state.live_gate = Some(gate.clone());
    let head = state.directory.head_seq().await.unwrap();
    let addr = spawn_server(state.clone()).await;
    let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
    let pending = {
        let token = author.token.clone();
        let command = command.clone();
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
    gate.directory_command_attempted.acquire().await.unwrap().forget();
    let demotion = DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
    let revoking = {
        let token = owner.token.clone();
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "d00102030405060708090a0b0c0d0e0f", demotion).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
    assert!(!revoking.is_finished());
    assert_eq!(state.directory.head_seq().await.unwrap(), head);
    gate.directory_command_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
    assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
    let receipt = parse_directory_command_receipt_for_test(&response, "c00102030405060708090a0b0c0d0e0f", &command);
    assert_eq!(receipt.outcome, DirectoryCommandOutcomeV1::Accepted);
    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
    let events = state.directory.events_since(head, 8).await.unwrap();
    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::MemberRemoved { user_id, .. } if user_id == &target.user_id));
    assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), None);
    assert_eq!(state.directory.get_role(&space, &author.user_id).await.unwrap(), Some(SpaceRole::Spectator));
}

#[tokio::test]
async fn directory_command_authority_invite_revocation_requires_the_exact_owned_space() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).expect("authority fixture");
    let state = test_state().await;
    let caller = issue_test_session(&state, "invite-scope-owner@example.test").await;
    let other = issue_test_session(&state, "invite-scope-other@example.test").await;
    let owned = create_space_for_test(&state, &caller.user_id, "Owned space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let foreign = create_space_for_test(&state, &other.user_id, "Foreign space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    for (index, row) in fixture["inviteScopes"].as_array().expect("invite scope rows").iter().enumerate() {
        let issued = state.directory.issue_invite(&foreign, SpaceRole::Spectator, 600, "invite-scope-law").await.expect("foreign invite");
        let invite = issued.record.clone();
        if row["accepted"].as_bool().expect("accepted fixture") {
            let recipient = issue_test_session(&state, &format!("invite-recipient-{index}@example.test")).await;
            state.directory_service.redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-scope", recipient.user_id) }, &issued.capability, &recipient.user_id).await.expect("accepted invite fixture");
        }
        let exact = row["commandSpace"] == row["inviteSpace"];
        let (space, token) = if exact { (&foreign, &other.token) } else { (&owned, &caller.token) };
        let head = state.directory.head_seq().await.expect("before revoke head");
        let request_id = format!("{:032x}", 1000 + index);
        let response = post_directory_command_for_test(addr, token, &request_id, DirectoryCommand::RevokeInvite { space_id: space.clone(), invite_id: invite.id.clone() }).await;
        assert_eq!(u64::from(response.status), row["status"].as_u64().expect("expected status"), "{}", row["id"]);
        let current = state.directory.list_invites(&foreign).await.expect("foreign invite after request").into_iter().find(|candidate| candidate.id == invite.id).expect("retained foreign invite");
        assert_eq!(current.revoked_at.is_some(), row["revoked"].as_bool().expect("expected revocation"));
        assert_eq!(state.directory.head_seq().await.expect("after revoke head"), head, "invite revocation changes capability authority, not membership events");
        if !exact {
            assert!(response.body.is_empty(), "cross-space response carries no foreign metadata");
        }
    }
}

/// 🎟️ Uses real space commands and HTTP redemption to preserve archive reads without restoring authorship.
#[tokio::test]
async fn directory_invite_redemption_obeys_current_space_state_and_readonly_replay() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).expect("invite state fixture");
    let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite state open deadline");
    let owner = issue_test_session(&state, "invite-state-owner@example.test").await;
    let addr = spawn_server(state.clone()).await;
    for (index, row) in fixture["spaceStates"].as_array().expect("state rows").iter().enumerate() {
        let caller = issue_test_session(&state, &format!("invite-state-{index}@example.test")).await;
        let other = issue_test_session(&state, &format!("invite-state-other-{index}@example.test")).await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite state", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let role = if row["inviteRole"] == "author" { SpaceRole::Author } else { SpaceRole::Spectator };
        let issued = state.directory.issue_invite(&space, role, 600, "invite-state-law").await.expect("pending invite");
        let accepted_user = if row["accepted"] == "other" { &other } else { &caller };
        let accepted = if row["accepted"] != "none" {
            state
                .directory_service
                .redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-state", accepted_user.user_id) }, &issued.capability, &accepted_user.user_id)
                .await
                .map(|commit| vec![commit.into_event()])
                .expect("initial acceptance")
        } else {
            Vec::new()
        };
        let transition = match row["spaceState"].as_str().unwrap() {
            "archived" => Some(DirectoryCommand::ArchiveSpace { space_id: space.clone() }),
            "deleted" => Some(DirectoryCommand::DeleteSpace { space_id: space.clone() }),
            "writable" => None,
            _ => unreachable!(),
        };
        if let Some(command) = transition {
            assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 2000 + index), command).await.status, 202);
        }
        let replay_grant = if row["accepted"] == "same" && row["spaceState"] != "deleted" {
            let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
            let capability = SocketGrantCapability::mint().unwrap();
            let audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new(&space, "invite-replay"));
            let subject = SocketSubjectV1::Session { session_id: session.session_id, user_id: caller.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Spectator), expires_at_ms: session.expires_at, session_kind: AuthSessionKind::External, device_instance_id: "device-spectator".into() };
            state.socket_grants.issue(&capability, audience.clone(), "hub.v1.invite-replay".into(), subject, now_ms(), now_ms() + 30_000).unwrap();
            Some((capability, audience))
        } else {
            None
        };
        let before = state.directory.head_seq().await.unwrap();
        let mut stream = state.directory_service.subscribe();
        let token = issued.capability.expose_once();
        let authorization = format!("Bearer {}", caller.token);
        let response = raw_http_request(addr, "POST", &format!("/directory/invites/{token}/redeem"), &[("Authorization", &authorization)], &[]).await;
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
        if let Some((capability, audience)) = replay_grant {
            assert!(state.socket_grants.pending(&capability, &audience, now_ms()).is_ok(), "read-only replay preserves fresh admission");
        }
        let appended = state.directory.events_since(before, 16).await.unwrap();
        assert_eq!(appended.len() as u64, row["appended"].as_u64().unwrap(), "{}", row["name"]);
        assert!(appended.iter().all(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })));
        let current = state.directory.get_role(&space, &caller.user_id).await.unwrap();
        let expected_role = match row["membershipRole"].as_str().unwrap() {
            "author" => Some(SpaceRole::Author),
            "spectator" => Some(SpaceRole::Spectator),
            "none" => None,
            _ => unreachable!(),
        };
        assert_eq!(current, expected_role, "{}", row["name"]);
        let invitations = state.directory.list_invites(&space).await.unwrap();
        if row["spaceState"] == "deleted" {
            assert!(invitations.is_empty());
        } else {
            let retained = invitations.iter().find(|invite| invite.id == issued.record.id).expect("retained invitation");
            assert_eq!(retained.accepted_at.is_some(), !accepted.is_empty() || !appended.is_empty());
            if !accepted.is_empty() {
                assert_eq!(retained.accepted_event_id.as_deref(), Some(accepted[0].id.as_str()));
            }
        }
        if row["appended"] == 0 {
            assert!(matches!(stream.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "replay and denial publish no new event");
        } else {
            assert!(stream.try_recv().is_ok());
        }
        if response.status == 200 && !accepted.is_empty() {
            let expected = axum::body::to_bytes(DirectoryJson(accepted).into_response().into_body(), 1 << 20).await.unwrap();
            assert_eq!(response.body.as_slice(), expected.as_ref(), "replay returns the exact original immutable event");
        } else if response.status != 200 {
            assert!(response.body.is_empty());
        }
    }
}

/// 🧭️ Scope selection accepts only the exact stored secret and existing authenticated subject.
#[tokio::test]
async fn directory_invite_redemption_scope_hint_is_capability_bound() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite hint state deadline");
    let owner = issue_test_session(&state, "invite-hint-owner@example.test").await;
    let caller = issue_test_session(&state, "invite-hint-caller@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Invite hint", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-hint").await.unwrap();
    let other = InviteCapability::mint().unwrap();
    let other_encoded = other.expose_once();
    let wrong_secret = InviteCapability::parse(&other_encoded.replacen(other.selector(), issued.capability.selector(), 1)).unwrap();
    let head = state.directory.head_seq().await.unwrap();
    for row in fixture["vectors"].as_array().unwrap().iter().filter(|row| ["fresh-single", "wrong-selector", "wrong-secret", "actor-mismatch", "missing-user"].contains(&row["name"].as_str().unwrap())) {
        let user_id = if row["name"] == "missing-user" { "nonexistent-user" } else { &caller.user_id };
        let actor_user = if row["name"] == "actor-mismatch" { &owner.user_id } else { user_id };
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{actor_user}#hint") };
        let capability = match row["name"].as_str().unwrap() {
            "wrong-selector" => &other,
            "wrong-secret" => &wrong_secret,
            _ => &issued.capability,
        };
        let result = state.directory.invite_redemption_scope_hint(capability, &actor, user_id).await;
        if row["expected"]["outcomes"][0] == "newly-committed" {
            assert_eq!(result.unwrap().space_id(), space);
        } else {
            assert!(matches!(result, Err(DirectoryError::Unauthorized)), "{}", row["name"]);
        }
        assert_eq!(state.directory.head_seq().await.unwrap(), head, "a hint writes no event");
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None, "a hint grants no membership");
        assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint", caller.user_id) };
    state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint-owner", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: space }).await.unwrap();
    assert!(matches!(state.directory.invite_redemption_scope_hint(&issued.capability, &actor, &caller.user_id).await, Err(DirectoryError::Unauthorized)));
}

/// 🔒️ A paused redemption cannot spend the identity captured before a durable revocation.
#[tokio::test]
async fn directory_invite_redemption_revalidates_after_hint_before_fence() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite race state deadline");
    let owner = issue_test_session(&state, "invite-race-owner@example.test").await;
    let gate = Arc::new(TestLiveGate::default());
    state.live_gate = Some(gate.clone());
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    for (index, row) in fixture["authorityRaces"].as_array().unwrap().iter().filter(|row| row["appended"] == 0).enumerate() {
        let caller = issue_test_session(&state, &format!("invite-race-{index}@example.test")).await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-race").await.unwrap();
        *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), false));
        let pending = {
            let token = caller.token.clone();
            let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
            tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        if row["revocation"] == "session" {
            assert_eq!(post_session_sign_out(bearer_headers(&caller.token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
        } else {
            let command = if row["revocation"] == "archive" { DirectoryCommand::ArchiveSpace { space_id: space.clone() } } else { DirectoryCommand::DeleteSpace { space_id: space.clone() } };
            assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 3000 + index), command).await.status, 202);
        }
        let head = state.directory.head_seq().await.unwrap();
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
        assert!(response.body.is_empty());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None);
        assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
    }
    shutdown.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
}

/// 🧱️ All session and space keys stay owned through one irreversible invitation event.
#[tokio::test]
async fn directory_invite_redemption_admitted_fence_precedes_archive() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let row = fixture["authorityRaces"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
    let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite order state deadline");
    let owner = issue_test_session(&state, "invite-order-owner@example.test").await;
    let caller = issue_test_session(&state, "invite-order-caller@example.test").await;
    let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
    let space = create_space_for_test(&state, &owner.user_id, "Invite order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-order").await.unwrap();
    let gate = Arc::new(TestLiveGate::default());
    *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), true));
    state.live_gate = Some(gate.clone());
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    let head = state.directory.head_seq().await.unwrap();
    let pending = {
        let token = caller.token.clone();
        let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
        tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
    for key in [
        SocketBindingKeyV1::User(caller.user_id.clone()),
        SocketBindingKeyV1::Session(session.session_id.clone()),
        SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space.clone() },
        SocketBindingKeyV1::Membership { space_id: space.clone(), user_id: caller.user_id.clone() },
    ] {
        assert!(state.socket_binding_gates.gate(key).try_write_owned().is_err(), "the admitted redemption must own every exact authority key");
    }
    gate.directory_command_attempted.acquire().await.unwrap().forget();
    let revoking = {
        let token = owner.token.clone();
        let command = DirectoryCommand::ArchiveSpace { space_id: space.clone() };
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "f10102030405060708090a0b0c0d0e0f", command).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
    assert!(!revoking.is_finished());
    assert_eq!(state.directory.head_seq().await.unwrap(), head);
    gate.directory_command_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
    assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
    let events = state.directory.events_since(head, 16).await.unwrap();
    assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::InviteRedeemed { user_id, role: DirectorySpaceRole::Author, .. } if user_id == &caller.user_id));
    assert!(matches!(&events.last().unwrap().body, os_directory::DirectoryEventBody::SpaceArchived { .. }));
    assert_eq!(events.iter().filter(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })).count(), row["appended"].as_u64().unwrap() as usize);
    assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), Some(SpaceRole::Spectator));
    shutdown.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
}

#[tokio::test]
async fn directory_command_authority_demotion_invalidates_only_affected_scope_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    let shared = SocketGrantLedgerV1::default();
    let capacity = &fixture["capacity"];
    assert_eq!(capacity["subjectLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_BINDING_PENDING_CAPACITY);
    assert_eq!(capacity["ledgerLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_LEDGER_CAPACITY);
    for index in 0..capacity["subjects"].as_u64().unwrap() {
        let subject = SocketSubjectV1::Session { session_id: format!("capacity-session-{index}"), user_id: format!("capacity-user-{index}"), authorization_generation: 1, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: format!("capacity-device-{index}") };
        shared.issue(&SocketGrantCapability::mint().unwrap(), SocketAudienceV1::DirectoryScoped(DocumentScope::new("shared-space", "document")), format!("hub.v1.capacity-{index}"), subject, 1, 9_000).unwrap();
    }
    assert_eq!(shared.inner.lock().unwrap().records.len() as u64, capacity["accepted"].as_u64().unwrap());
    let state = test_state().await;
    let owner = issue_test_session(&state, "authority-owner@example.com").await;
    let author = issue_test_session(&state, "authority-author@example.com").await;
    let space = create_space_for_test(&state, &owner.user_id, "Authority Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let other = create_space_for_test(&state, &author.user_id, "Other Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
    let plan_fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let plan = Arc::new(document_open_plan_test_authority(&plan_fixture));
    let issue_pending = |audience: &SocketAudienceV1, actor_id: String, subject: SocketSubjectV1| -> String {
        if matches!(audience, SocketAudienceV1::Document(_)) {
            return state.socket_grants.admit_document(audience.clone(), actor_id, subject, now_ms(), now_ms() + 30_000, plan.clone()).unwrap();
        }
        let capability = SocketGrantCapability::mint().unwrap();
        state.socket_grants.issue(&capability, audience.clone(), actor_id, subject, now_ms(), now_ms() + 30_000).unwrap();
        capability.selector().to_string()
    };
    let still_pending = |selector: &str| state.socket_grants.inner.lock().unwrap().records.get(selector).is_some_and(|record| record.state == SocketGrantStateV1::Pending);
    let mut records = Vec::new();
    for row in fixture["bindings"].as_array().unwrap() {
        let user = if row["user"] == "author" { &author } else { &owner };
        let session = state.directory.authenticate_session(&SessionCapability::parse(&user.token).unwrap()).await.unwrap().unwrap();
        let scope = DocumentScope::new(if row["space"] == "changed" { &space } else { &other }, "authority-document");
        let subject = SocketSubjectV1::Session { session_id: session.id.clone(), user_id: user.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Author), expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id.clone() };
        let audience = match row["audience"].as_str().unwrap() {
            "document" => SocketAudienceV1::Document(scope),
            "scoped" => SocketAudienceV1::DirectoryScoped(scope),
            "global" => SocketAudienceV1::Directory { auth_session_id: session.id, authorization_generation: session.authorization_generation },
            _ => unreachable!(),
        };
        let actor_id = format!("hub.v1.{}", row["id"].as_str().unwrap());
        let live = issue_pending(&audience, actor_id.clone(), subject.clone());
        let pending = issue_pending(&audience, actor_id, subject.clone());
        let record = state.socket_grants.inner.lock().unwrap().records.get(&live).cloned().unwrap();
        let record = state.socket_grants.consume(&record, now_ms()).unwrap();
        let (live_id, notify) = state.socket_grants.register_live(&record).unwrap();
        records.push((row, pending, record, live_id, notify));
    }
    let addr = spawn_server(state.clone()).await;
    let command = DirectoryCommand::UpsertMember { space_id: space, email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
    let request_id = "e00102030405060708090a0b0c0d0e0f";
    assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
    for (row, pending, record, live_id, notify) in &records {
        let invalidated = row["invalidated"].as_bool().unwrap();
        assert_eq!(!still_pending(pending), invalidated, "pending {}", row["id"]);
        assert_eq!(!state.socket_grants.is_live(record, live_id), invalidated, "live {}", row["id"]);
        if invalidated {
            tokio::time::timeout(std::time::Duration::from_secs(1), notify.notified()).await.unwrap();
            let mut subject = record.subject.clone();
            if let SocketSubjectV1::Session { role, .. } = &mut subject {
                *role = Some(SpaceRole::Spectator);
            }
            let fresh = issue_pending(&record.audience, record.actor_id.clone(), subject);
            assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
            assert!(still_pending(&fresh), "receipt replay must not invalidate fresh admission");
        }
    }
}

#[tokio::test]
async fn directory_command_receipt_v1_route_is_request_idempotent_for_concurrent_identical_ids() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-author@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Receipt Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let mut live = state.directory_service.subscribe();
    let command = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";
    let (first, second) = tokio::join!(post_directory_command_for_test(addr, &author.token, request_id, command.clone()), post_directory_command_for_test(addr, &author.token, request_id, command.clone()),);
    assert_eq!((first.status, second.status), (202, 202), "an idempotent duplicate is answered, never failed");
    let receipts = [parse_directory_command_receipt_for_test(&first, request_id, &command), parse_directory_command_receipt_for_test(&second, request_id, &command)];
    let tokens: Vec<String> = receipts
        .iter()
        .filter_map(|receipt| match &receipt.result {
            os_directory::DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
            os_directory::DirectoryCommandResultV1::None => None,
        })
        .collect();
    assert_eq!(tokens.len(), 1, "exactly one response carries the one-shot capability");
    assert_eq!(receipts.iter().filter(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::Accepted).count(), 1);
    assert!(receipts.iter().any(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::SecretUndeliverable && receipt.result == os_directory::DirectoryCommandResultV1::None));
    let token = tokens.into_iter().next().expect("issued capability");

    let invites = state.directory.list_invites(&space_id).await.expect("invite rows");
    assert_eq!(invites.len(), 1, "two concurrent identical request ids mint exactly one invitation");
    assert!(!format!("{invites:?}").contains(&token), "no invite row retains the capability plaintext");

    let retry = post_directory_command_for_test(addr, &author.token, request_id, command.clone()).await;
    let replayed = parse_directory_command_receipt_for_test(&retry, request_id, &command);
    assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
    assert!(!std::str::from_utf8(&retry.body).expect("retry UTF-8").contains(&token), "a later resolution of the same id is redacted");
    assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows after retry").len(), 1);

    let events = state.directory.events_since(0, 1_000).await.expect("durable log");
    assert!(!format!("{events:?}").contains(&token), "the capability never enters the durable event log");
    let mut broadcast = Vec::new();
    while let Ok(message) = live.try_recv() {
        broadcast.push(format!("{message:?}"));
    }
    assert!(!broadcast.join("\n").contains(&token), "the capability never enters the live broadcast");
}

#[tokio::test]
async fn directory_command_receipt_v1_route_denies_cross_user_spectator_and_digest_substitution() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-owner@example.com").await;
    let spectator = issue_test_session(&state, "command-receipt-spectator@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Denial Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, "command-receipt-spectator@example.com", DirectorySpaceRole::Spectator).await;
    let addr = spawn_server(state.clone()).await;
    let invite = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request_id = "0a1b2c3d4e5f60718293a4b5c6d7e8f9";

    let accepted = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
    assert_eq!(accepted.status, 202);
    let token = match parse_directory_command_receipt_for_test(&accepted, request_id, &invite).result {
        DirectoryCommandResultV1::Invite { invite_token } => invite_token,
        DirectoryCommandResultV1::None => panic!("the first accepted create-invite carries its capability"),
    };

    let denied = post_directory_command_for_test(addr, &spectator.token, request_id, invite.clone()).await;
    assert_eq!(denied.status, 403, "a spectator is denied before any stored completion is consulted");
    assert!(denied.body.is_empty(), "a denial carries no body, so no receipt or capability leaks");

    let cross_user =
        post_directory_command_for_test(addr, &spectator.token, request_id, DirectoryCommand::CreateSpace { name: "Spectator Space".into(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await;
    assert_eq!(cross_user.status, 202, "the idempotency key is scoped to the authenticated user");
    assert!(!std::str::from_utf8(&cross_user.body).expect("cross-user UTF-8").contains(&token), "another user's equal request id never discovers a stored capability");

    let substituted = post_directory_command_for_test(addr, &author.token, request_id, DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Substituted".into() }).await;
    assert_eq!(substituted.status, 409, "an equal key with an unequal command digest is a generic conflict");
    assert!(substituted.body.is_empty());
    assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows").len(), 1, "no denial or conflict executed a second time");

    let capability = SessionCapability::parse(&author.token).expect("author capability");
    let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    state.directory.revoke_auth_session(&record.id, "command-receipt-test", None, "command-receipt-test").await.expect("revoke session").expect("revoked row");
    let revoked = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
    assert_eq!(revoked.status, 401, "authentication re-runs before any stored completion is returned");
    assert!(revoked.body.is_empty());
}

#[tokio::test]
async fn directory_command_receipt_v1_route_bounds_request_and_receipt_bytes() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-bounds@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Bounds Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let authorization = format!("Bearer {}", author.token);

    let fitting = DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "n".repeat(64) };
    let request = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a0", fitting.clone());
    assert!(request.canonical_json().len() <= DIRECTORY_COMMAND_REQUEST_MAX_BYTES);
    let accepted = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], request.canonical_json().as_bytes()).await;
    assert_eq!(accepted.status, 202);
    assert!(accepted.body.len() <= os_directory::DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, "an admitted request can never produce an over-ceiling receipt");

    let padding = DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1 - DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: String::new() }).canonical_json().len();
    let oversize = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "x".repeat(padding) });
    assert_eq!(oversize.canonical_json().len(), DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1);
    let rejected = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], oversize.canonical_json().as_bytes()).await;
    assert!(matches!(rejected.status, 400 | 413), "one byte past the request ceiling is refused, got {}", rejected.status);

    for hostile in [
        "{\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"schema\":\"semio.directory.command-request.v1\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9F8E7D6C5B4A39281706F5E4D3C2B1A2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"00000000000000000000000000000000\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"},\"extra\":1}",
        "{\"kind\":\"archive-space\",\"spaceId\":\"x\"}",
    ] {
        let response = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], hostile.as_bytes()).await;
        assert_eq!(response.status, 400, "a noncanonical, mis-cased, zero-id, unknown-field, or bare-command body is refused: {hostile}");
        assert!(response.body.is_empty());
    }
}

#[tokio::test]
async fn directory_command_receipt_v1_store_resolves_a_lost_reply_and_survives_restart() {
    let path = tempdir("command-receipt-store");
    std::fs::create_dir_all(&path).expect("receipt store dir");
    let database = path.join("directory.sqlite");
    let database_path = database.to_str().expect("receipt store path").to_string();
    let claim = |request_id: &str, digest: &str, kind: DirectoryCommandResultKindV1, actor: &str| NewDirectoryCommandReceipt {
        actor_user_id: actor.to_string(),
        request_id: request_id.to_string(),
        command_sha256: digest.to_string(),
        result_kind: kind,
        claimed_at: 1_700_000_000_000,
    };
    let invite_digest = directory_command_sha256(&DirectoryCommand::CreateInvite { space_id: "space".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 });
    let rename_digest = directory_command_sha256(&DirectoryCommand::RenameSpace { space_id: "space".into(), name: "Renamed".into() });
    let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";

    let directory = SqliteDirectory::connect(&database_path).await.expect("connect receipt store");
    directory.seed().await.expect("seed receipt store");
    let pending_claim = claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-a");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("first claim"), DirectoryCommandClaimV1::Claimed(_)));
    let lost = match directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("lost-reply resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a claimed key never re-executes: {other:?}"),
    };
    assert_eq!(lost.disposition, DirectoryCommandDispositionV1::Pending);
    assert_eq!(replay_directory_command_receipt(&lost).outcome, DirectoryCommandOutcomeV1::SecretUndeliverable, "a reply lost between durable command and response is honestly undeliverable");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &rename_digest, DirectoryCommandResultKindV1::None, "user-a")).await.expect("digest substitution"), DirectoryCommandClaimV1::Conflict));
    assert!(
        matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-b")).await.expect("cross-user claim"), DirectoryCommandClaimV1::Claimed(_)),
        "the key is scoped to the authenticated user"
    );

    let replay_digest = replay_directory_command_receipt(&DirectoryCommandReceiptRecord { disposition: DirectoryCommandDispositionV1::Completed, ..lost.clone() }).receipt_sha256;
    let completed = directory
        .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
            actor_user_id: "user-a".into(),
            request_id: request_id.into(),
            event_seq_first: None,
            event_seq_last: None,
            receipt_sha256: replay_digest.clone(),
            completed_at: 1_700_000_000_001,
        })
        .await
        .expect("durable completion");
    assert_eq!(completed.disposition, DirectoryCommandDispositionV1::Completed);
    assert!(
        directory
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: "user-a".into(),
                request_id: request_id.into(),
                event_seq_first: None,
                event_seq_last: None,
                receipt_sha256: replay_digest.clone(),
                completed_at: 1_700_000_000_002
            })
            .await
            .is_err(),
        "one claim completes exactly once"
    );
    drop(directory);

    let restarted = SqliteDirectory::connect(&database_path).await.expect("reconnect receipt store");
    restarted.seed().await.expect("reseed receipt store");
    let resolved = match restarted.claim_or_read_directory_command_receipt(&pending_claim).await.expect("restart resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a completed key survives restart: {other:?}"),
    };
    assert_eq!(resolved.disposition, DirectoryCommandDispositionV1::Completed);
    assert_eq!(resolved.receipt_sha256.as_deref(), Some(replay_digest.as_str()), "the durable row carries the canonical redacted receipt digest");
    let replayed = replay_directory_command_receipt(&resolved);
    assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
    assert_eq!(replayed.receipt_sha256, replay_digest);
    assert!(replayed.validate().is_ok() && replayed.events.is_empty() && replayed.result == DirectoryCommandResultV1::None);

    let plain = NewDirectoryCommandReceipt { result_kind: DirectoryCommandResultKindV1::None, command_sha256: rename_digest.clone(), request_id: "9f8e7d6c5b4a39281706f5e4d3c2b1a0".into(), ..pending_claim.clone() };
    assert!(matches!(restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain claim"), DirectoryCommandClaimV1::Claimed(_)));
    restarted
        .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
            actor_user_id: plain.actor_user_id.clone(),
            request_id: plain.request_id.clone(),
            event_seq_first: Some(7),
            event_seq_last: Some(8),
            receipt_sha256: "0".repeat(64),
            completed_at: 1_700_000_000_003,
        })
        .await
        .expect("plain completion");
    let plain_record = match restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a completed plain key replays: {other:?}"),
    };
    assert_eq!((plain_record.event_seq_first, plain_record.event_seq_last), (Some(7), Some(8)));
    assert_eq!(replay_directory_command_receipt(&plain_record).outcome, DirectoryCommandOutcomeV1::PreviouslyAccepted, "a completed secret-free command resolves as previously accepted");
    drop(restarted);
    let _ = std::fs::remove_dir_all(&path);
}

#[tokio::test]
async fn directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility() {
    let state = test_state().await;
    let caller = issue_test_session(&state, "event-page-member@example.com").await;
    upsert_member_for_test(&state, STUDIO, "event-page-member@example.com", DirectorySpaceRole::Spectator).await;
    let outsider = issue_test_session(&state, "event-page-owner@example.com").await;
    let hidden_space = create_space_for_test(&state, &outsider.user_id, "Hidden", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let after = state.directory.head_seq().await.expect("event-page start");
    let appended = append_directory_page_test_events(&state, &[(hidden_space.clone(), "hidden-11".into()), (STUDIO.into(), "visible-12".into()), (hidden_space.clone(), "hidden-13".into()), (STUDIO.into(), "visible-14".into())]).await;
    let addr = spawn_server(state.clone()).await;
    let authorization = event_page_authorization(&caller.token);
    let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
    assert_eq!(response.status, 200);
    assert!(response.headers.to_ascii_lowercase().contains("content-type: application/json"));
    let canonical = std::str::from_utf8(&response.body).expect("event-page UTF-8");
    let page = DirectoryEventPageV1::parse_canonical_json(canonical).expect("canonical event page");
    assert_eq!(page.after_seq_exclusive, after);
    assert_eq!(page.through_seq_inclusive, appended[3].seq);
    assert_eq!(page.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq, appended[3].seq]);
    assert!(!canonical.contains(&hidden_space));
    assert!(!canonical.contains("hidden-11"));
    assert!(!canonical.contains("hidden-13"));

    let hidden_after = state.directory.head_seq().await.expect("hidden scan start");
    let hidden = (0..DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).map(|index| (hidden_space.clone(), format!("hidden-saturated-{index}"))).collect::<Vec<_>>();
    let hidden_events = append_directory_page_test_events(&state, &hidden).await;
    let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={hidden_after}"), &[("Authorization", &authorization)]).await;
    let page = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("hidden page UTF-8")).expect("hidden page");
    assert!(page.events.is_empty());
    assert_eq!(page.through_seq_inclusive, hidden_events.last().expect("hidden tail").seq);
    assert!(page.has_more, "a saturated raw scan advertises the bounded follow-up even when every row is hidden");
}

#[tokio::test]
async fn directory_event_page_v1_route_revalidates_session_generation_after_read_before_response() {
    let mut state = test_state().await;
    let session = issue_test_session(&state, "event-page-revoked@example.com").await;
    upsert_member_for_test(&state, STUDIO, "event-page-revoked@example.com", DirectorySpaceRole::Spectator).await;
    append_directory_page_test_events(&state, &[(STUDIO.into(), "before-revoke".into())]).await;
    let capability = SessionCapability::parse(&session.token).expect("event-page capability");
    let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    let gate = Arc::new(TestLiveGate::default());
    gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
    state.live_gate = Some(gate.clone());
    let addr = spawn_server(state.clone()).await;
    let authorization = event_page_authorization(&session.token);
    let request = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("event-page read fence deadline").expect("event-page read fence");
    admitted.forget();
    state.directory.revoke_auth_session(&record.id, "event-page-test", None, "event-page-test").await.expect("revoke session").expect("revoked row");
    gate.directory_event_page_read_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(2), request).await.expect("revalidation response deadline").expect("revalidation request");
    assert_eq!(response.status, 401);
    assert!(response.body.is_empty());

    let session = issue_test_session(&state, "event-page-cancel@example.com").await;
    let authorization = event_page_authorization(&session.token);
    let cancelled = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("cancel read fence deadline").expect("cancel read fence");
    admitted.forget();
    let control = gate.directory_event_page_control.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().expect("request-owned cancellation control");
    cancelled.abort();
    assert!(cancelled.await.expect_err("request task cancelled").is_cancelled());
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while control.active.load(std::sync::atomic::Ordering::Acquire) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("server request cancellation deadline");
    assert!(control.cancelled.load(std::sync::atomic::Ordering::Acquire));
    gate.directory_event_page_read_release.add_permits(1);
}

#[tokio::test]
async fn directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq() {
    let state = test_state().await;
    let token = seed_author_token(&state).await;
    let after = state.directory.head_seq().await.expect("byte-prefix start");
    let appended = append_directory_page_test_events(&state, &[(STUDIO.into(), "a".repeat(32 * 1024)), (STUDIO.into(), "b".repeat(32 * 1024))]).await;
    let addr = spawn_server(state).await;
    let authorization = event_page_authorization(&token);
    let first = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
    assert_eq!(first.status, 200);
    assert!(first.body.len() <= DIRECTORY_EVENT_PAGE_MAX_BYTES);
    let first = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&first.body).expect("first page UTF-8")).expect("first page");
    assert_eq!(first.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[0].seq]);
    assert_eq!(first.through_seq_inclusive, appended[0].seq);
    assert!(first.has_more);
    let second = raw_http_get(addr, &format!("/directory/event-page/v1?after={}", first.through_seq_inclusive), &[("Authorization", &authorization)]).await;
    let second = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&second.body).expect("second page UTF-8")).expect("second page");
    assert_eq!(second.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq]);
    assert_eq!(second.through_seq_inclusive, appended[1].seq);
}

#[tokio::test]
async fn directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j() {
    let state = test_state().await;
    let mut exact = DirectoryEvent {
        seq: 1,
        id: "event-boundary".into(),
        hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-boundary".into() },
        space_id: Some(STUDIO.into()),
        user_id: None,
        body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: String::new() },
        recorded_at_ms: 1,
    };
    let base = semio_framework_pack_json::to_json_string(&exact).len();
    let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body else { unreachable!() };
    *name = "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
    assert_eq!(semio_framework_pack_json::to_json_string(&exact).len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
    assert_eq!(validate_directory_event_page_event(&exact), Ok(()));
    if let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body {
        name.push('x');
    }
    assert_eq!(validate_directory_event_page_event(&exact), Err(DirectoryEventPageErrorV1::Invalid));

    let head = state.directory.head_seq().await.expect("head before rejected append");
    let rejected = semio_hub_directory_contract::NewDirectoryEvent {
        hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-reject".into() },
        space_id: Some(STUDIO.into()),
        user_id: None,
        body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES) },
    };
    assert!(matches!(state.directory.append_events(&[rejected]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(state.directory.head_seq().await.expect("head after rejected append"), head, "SQLite rolls back both row and dense sequence");
    let space = state.directory.get_space(STUDIO).await.expect("space projection").expect("seed space");
    assert_ne!(space.name.len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES, "rejected event never reaches projection");

    let postgres = semio_hub_testing::hub_module_tree("📇️directory/🐘️postgres/🦀️.rs");
    let neo4j = semio_hub_testing::hub_module_tree("📇️directory/🌐️neo4j/🦀️.rs");
    let postgres_writes = postgres.matches("INSERT INTO hub_directory_event(seq").count() + postgres.matches("INSERT INTO hub_directory_event (seq").count();
    let neo4j_writes = neo4j.matches("CREATE (e:DirectoryEvent {seq:").count();
    assert!(postgres_writes >= 3 && neo4j_writes >= 3, "both remote backends still persist directory events on every append seam");
    assert_eq!(postgres.matches("validate_directory_event_page_event(&").count(), postgres_writes, "all PostgreSQL full-event append seams admit before persistence");
    assert_eq!(neo4j.matches("validate_directory_event_page_event(&").count(), neo4j_writes, "all Neo4j full-event append seams admit before persistence");
}

#[tokio::test]
async fn directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body() {
    let mut state = test_state().await;
    let token = seed_author_token(&state).await;
    let stale = issue_test_session(&state, "event-page-stale@example.com").await;
    let stale_capability = SessionCapability::parse(&stale.token).expect("stale capability");
    let stale_record = state.directory.authenticate_session(&stale_capability).await.expect("stale lookup").expect("stale session");
    state.directory.revoke_auth_session(&stale_record.id, "stale", None, "stale").await.expect("stale revoke").expect("stale row");
    let gate = Arc::new(TestLiveGate::default());
    gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
    state.live_gate = Some(gate.clone());
    let addr = spawn_server(state).await;
    let authorization = event_page_authorization(&token);
    for path in [
        "/directory/event-page/v1",
        "/directory/event-page/v1?",
        "/directory/event-page/v1?after=",
        "/directory/event-page/v1?after=00",
        "/directory/event-page/v1?after=1&after=2",
        "/directory/event-page/v1?since=0",
        "/directory/event-page/v1?after=%30",
        "/directory/event-page/v1?after=9007199254740992",
    ] {
        let response = raw_http_get(addr, path, &[("Authorization", &authorization)]).await;
        assert_eq!(response.status, 400, "query {path}");
        assert!(response.body.is_empty(), "query rejection is body-free");
    }
    let missing = raw_http_get(addr, "/directory/event-page/v1?after=0", &[]).await;
    assert_eq!(missing.status, 401);
    assert!(missing.body.is_empty());
    let stale_authorization = event_page_authorization(&stale.token);
    let stale = raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &stale_authorization)]).await;
    assert_eq!(stale.status, 401);
    assert!(stale.body.is_empty());
    assert_eq!(gate.directory_event_page_read_admitted.available_permits(), 0, "bad query and pre-read authentication failures perform no directory event scan");
}

// 🔬️ `GET /auth/sessions/me` + `POST /auth/sessions/me/sign-out`: a live session resolves the
// caller's identity; signing out makes the SAME token unauthorized on a subsequent call.
#[tokio::test]
async fn auth_sessions_me_roundtrip() {
    let state = test_state().await;
    let session = issue_test_session(&state, "me@example.com").await;
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().unwrap());

    let me = get_session_me(headers.clone(), State(state.clone())).await.expect("session me");
    assert_eq!(me.0.user_id, session.user_id);
    assert_eq!(me.0.email, "me@example.com");
    assert_eq!(me.0.schema, "semio.directory.session-authority.v1");
    assert_eq!(me.0.authorization_generation, 1);
    assert!(me.0.session_binding_sha256.len() == 64 && me.0.session_binding_sha256.bytes().any(|byte| byte != b'0'));
    assert_eq!(me.0.canonical_json().as_deref().and_then(os_directory::schema::session_authority::DirectorySessionAuthorityV1::parse_canonical_json), Some(me.0.clone()));

    assert_eq!(post_session_sign_out(headers.clone(), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
    assert_eq!(get_session_me(headers, State(state)).await.err(), Some(StatusCode::UNAUTHORIZED));
}
