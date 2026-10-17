
use super::*;
use crate::contract::{CommandId, PolicyGrant, QueryConsistency, QueryId, TraceContext};
use crate::test_instance::{CountingModule, SilentSaga, TestInstance, TestSagas, CREATION};
use crate::throttle::{Allowance, Rate};
use futures::channel::mpsc::unbounded;
use std::time::Duration;

async fn state() -> ServerState<TestInstance> {
    server().await.state().clone()
}

async fn server() -> Server<TestInstance> {
    Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).build().await.expect("the test instance opens its stores")
}

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "counter".into(), id: id.into() }
}

fn event(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock::default(), kind: "counter.incremented".into(), payload: vec![seq as u8] }
}

fn grant(state: &ServerState<TestInstance>, point: PolicyPoint, action: &str) {
    let mut engine = state.policy.write().unwrap();
    engine.register_template(PolicyTemplate { name: format!("{point:?}-{action}"), auto_apply: false, grants: vec![PolicyGrant { point, resource: "*".into(), action: action.to_string() }] });
    engine.assign("anonymous".to_string(), format!("{point:?}-{action}"));
}

fn scratch(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("semio-gateway-{name}-{unique}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn loopback() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4242))
}

fn caller() -> ClientKey {
    ClientKey::of(loopback().ip())
}

fn address(last: u8) -> ClientKey {
    ClientKey::of(IpAddr::V4(Ipv4Addr::new(203, 0, 113, last)))
}

//#region 🔖️Presence
#[test]
fn a_colour_lease_takes_the_lowest_free_slot() {
    let presence = Presence::new();
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
    assert_eq!(presence.acquire_colour("space-2", "dave"), 0);
}

#[test]
fn a_colour_is_freed_only_on_the_last_disconnect() {
    let presence = Presence::new();
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.colour_of("space-1", "alice"), Some(0));
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.colour_of("space-1", "alice"), None);
    assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
}

#[test]
fn joining_and_leaving_maintains_the_roster() {
    let presence = Presence::new();
    presence.join("space-1", "bob", "canvas");
    presence.join("space-1", "alice", "canvas");
    presence.publish_peer("space-1", "alice", vec![7]);
    let roster = presence.roster("space-1");
    assert_eq!(roster.iter().map(|session| session.actor.as_str()).collect::<Vec<_>>(), vec!["alice", "bob"]);
    assert_eq!(roster[0].peer, Some(vec![7]));
    presence.leave("space-1", "alice");
    assert_eq!(presence.roster("space-1").len(), 1);
    assert_eq!(presence.colour_of("space-1", "alice"), None);
}
//#endregion 🔖️Presence

//#region 🔖️Fanout
#[tokio::test]
async fn a_lane_reaches_every_subscriber_and_is_dropped_when_empty() {
    let fanout = Fanout::new();
    assert_eq!(fanout.publish("stream:a", vec![1]), 0);
    let mut first = fanout.subscribe("stream:a");
    let mut second = fanout.subscribe("stream:a");
    assert_eq!(fanout.lanes(), 1);
    assert_eq!(fanout.publish("stream:a", vec![9]), 2);
    assert_eq!(first.recv().await, Some(vec![9]));
    assert_eq!(second.recv().await, Some(vec![9]));
    drop(first);
    assert_eq!(fanout.lanes(), 1);
    drop(second);
    assert_eq!(fanout.lanes(), 0);
}

#[test]
fn lane_keys_keep_the_durable_and_ephemeral_lanes_apart() {
    let scope = Scope("space-1".into());
    assert_eq!(document_lane(&scope), "document:space-1");
    assert_eq!(ephemeral_lane(&scope), "ephemeral:space-1");
    assert_eq!(stream_lane(&actor("c1")), "stream:t1/counter/c1");
}
//#endregion 🔖️Fanout

//#region 🔖️Kick
#[tokio::test]
async fn a_kick_fired_before_the_wait_is_still_observed() {
    let kicks = KickMap::new();
    kicks.kick("session-1");
    tokio::time::timeout(Duration::from_millis(200), kicks.kicked("session-1")).await.expect("kick must be observed");
    assert_eq!(kicks.tracked(), 1);
    kicks.forget("session-1");
    assert_eq!(kicks.tracked(), 0);
    assert!(tokio::time::timeout(Duration::from_millis(20), kicks.kicked("session-2")).await.is_err());
}
//#endregion 🔖️Kick

//#region 🔖️StaticApp
#[test]
fn a_static_host_refuses_traversal() {
    let host = StaticAppHost::new("/srv/app");
    assert!(host.resolve("../secret").is_none());
    assert!(host.resolve("nested/../../secret").is_none());
    assert!(host.resolve("nested\\secret").is_none());
    assert_eq!(host.resolve("/etc/passwd"), Some(PathBuf::from("/srv/app/etc/passwd")));
    assert_eq!(host.resolve("assets/app.js"), Some(PathBuf::from("/srv/app/assets/app.js")));
}

#[tokio::test]
async fn a_client_route_falls_back_to_index_html() {
    let root = scratch("spa");
    std::fs::write(root.join("index.html"), b"<!doctype html>shell").unwrap();
    std::fs::write(root.join("app.js"), b"export {}").unwrap();
    let host = StaticAppHost::new(&root);

    let asset = host.serve("app.js");
    assert_eq!(asset.status(), StatusCode::OK);
    assert_eq!(asset.headers().get(header::CONTENT_TYPE).unwrap(), "text/javascript");

    let route = host.serve("spaces/sp-1");
    assert_eq!(route.status(), StatusCode::OK);
    let body = axum::body::to_bytes(route.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body.as_ref(), b"<!doctype html>shell");

    assert_eq!(host.serve("../secret").status(), StatusCode::BAD_REQUEST);
    assert_eq!(StaticAppHost::new(root.join("missing")).serve("index.html").status(), StatusCode::SERVICE_UNAVAILABLE);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn installs_are_scanned_and_ordered() {
    let root = scratch("apps");
    for id in ["beta", "alpha"] {
        std::fs::create_dir_all(root.join(id)).unwrap();
        std::fs::write(root.join(id).join("install.json"), format!("{{\"extensionId\":\"{id}\"}}")).unwrap();
    }
    std::fs::create_dir_all(root.join("empty")).unwrap();
    let registry = AppRegistry::new();
    registry.register("extensions", &root);
    let installs = registry.installs("extensions");
    assert_eq!(installs.iter().map(|install| install.id.as_str()).collect::<Vec<_>>(), vec!["alpha", "beta"]);
    assert_eq!(installs[0].manifest["extensionId"], "alpha");
    assert_eq!(registry.names(), vec!["extensions".to_string()]);
    assert!(registry.installs("nothing").is_empty());
    std::fs::remove_dir_all(&root).unwrap();
}
//#endregion 🔖️StaticApp

//#region 🔖️Cors
#[test]
fn cors_reflects_the_callers_own_origin_and_never_allows_credentials() {
    let mut headers = HeaderMap::new();
    apply_cors_headers(&mut headers, Some(&HeaderValue::from_static("http://127.0.0.1:6072")));
    assert_eq!(headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "http://127.0.0.1:6072");
    assert!(headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS).is_none());
    assert_eq!(headers.get(header::VARY).unwrap(), "origin");

    let mut anonymous = HeaderMap::new();
    apply_cors_headers(&mut anonymous, None);
    assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none());
    assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_METHODS).is_some());
}
//#endregion 🔖️Cors

//#region 🔖️Credential
#[test]
fn a_bearer_and_the_loopback_fact_are_read_from_the_transport() {
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer tok"));
    headers.insert(CAPABILITY_HEADER, HeaderValue::from_static("share-1"));
    let local = credential(&headers, Some(loopback()));
    assert_eq!(local.bearer.as_deref(), Some("tok"));
    assert_eq!(local.capability.map(|proof| proof.0), Some("share-1".to_string()));
    assert!(local.loopback);
    assert!(!credential(&headers, Some(SocketAddr::from(([10, 0, 0, 7], 80)))).loopback);
    assert!(!credential(&HeaderMap::new(), None).loopback);
    assert!(credential(&HeaderMap::new(), None).bearer.is_none());
}
//#endregion 🔖️Credential

//#region 🔖️Blob
#[tokio::test]
async fn a_blob_address_that_does_not_match_its_bytes_is_a_conflict() {
    let state = state().await;
    grant(&state, PolicyPoint::BlobWrite, "write");
    grant(&state, PolicyPoint::BlobRead, "read");

    let honest = content_hash(b"hello").to_string();
    let receipt = put_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello")).await.expect("an honest address is accepted");
    assert_eq!(receipt.0.size, 5);

    let lie = content_hash(b"world").to_string();
    let error = put_blob(Path(lie), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello")).await.expect_err("a mismatched address is refused");
    assert_eq!(error.status(), StatusCode::CONFLICT);

    assert_eq!(head_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::OK);
    let missing = content_hash(b"world").to_string();
    assert_eq!(head_blob(Path(missing), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::NOT_FOUND);
    let short = put_blob(Path("beef".to_string()), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await;
    assert_eq!(short.err().map(|error| error.status()), Some(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn a_blob_write_without_a_grant_is_forbidden() {
    let state = state().await;
    let hash = content_hash(b"hello").to_string();
    let error = put_blob(Path(hash), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await.expect_err("closed by default");
    assert_eq!(error.status(), StatusCode::FORBIDDEN);
}
//#endregion 🔖️Blob

//#region 🔖️EventStream
#[tokio::test]
async fn the_replay_live_seam_has_no_gap_and_no_duplicate() {
    let state = state().await;
    let actor = actor("c1");
    state.store.write().await.append_events(&actor, &[event(&actor, 1), event(&actor, 2), event(&actor, 3)], &[]).await.unwrap();

    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    state.fanout.publish(&stream_lane(&actor), serde_json::to_vec(&event(&actor, 3)).unwrap());
    state.fanout.publish(&stream_lane(&actor), serde_json::to_vec(&event(&actor, 4)).unwrap());

    let (mut sink, stream) = unbounded::<Message>();
    let pump = pump_events(&state, &actor, 0, &mut live, &mut sink);
    assert!(tokio::time::timeout(Duration::from_millis(120), pump).await.is_err());
    drop(sink);

    let delivered: Vec<u64> = stream
        .collect::<Vec<Message>>()
        .await
        .iter()
        .filter_map(|message| match message {
            Message::Text(text) => serde_json::from_str::<EventRecord>(text.as_str()).ok(),
            _ => None,
        })
        .map(|record| record.seq)
        .collect();
    assert_eq!(delivered, vec![1, 2, 3, 4]);
}

#[tokio::test]
async fn a_resuming_subscriber_skips_what_it_already_holds() {
    let state = state().await;
    let actor = actor("c1");
    state.store.write().await.append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap();
    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    let (mut sink, stream) = unbounded::<Message>();
    let pump = pump_events(&state, &actor, 1, &mut live, &mut sink);
    assert!(tokio::time::timeout(Duration::from_millis(80), pump).await.is_err());
    drop(sink);
    assert_eq!(stream.collect::<Vec<Message>>().await.len(), 1);
}

#[tokio::test]
async fn history_is_read_while_the_bus_is_held_by_a_turn() {
    let state = state().await;
    let actor = actor("c1");
    state.store.write().await.append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap();
    let turns = state.authority.lock().await;
    let replayed = tokio::time::timeout(Duration::from_secs(5), state.replay_events(&actor, 0)).await.expect("a reader never waits for the bus").expect("read");
    assert_eq!(replayed.len(), 2);
    drop(turns);
}

#[test]
fn the_seam_admits_every_sequence_once() {
    let actor = actor("c1");
    let mut seam = EventSeam::new(0);
    assert!(seam.admit(&event(&actor, 1)));
    assert!(seam.admit(&event(&actor, 2)));
    assert!(!seam.admit(&event(&actor, 2)));
    assert!(!seam.admit(&event(&actor, 1)));
    assert!(seam.admit(&event(&actor, 3)));
    assert_eq!(seam.delivered(), 3);
}
//#endregion 🔖️EventStream

//#region 🔖️Relay
#[test]
fn a_relayed_frame_names_the_session_that_produced_it() {
    let wrapped = wrap_relay("session-7", b"frame");
    assert_eq!(unwrap_relay(&wrapped), Some(("session-7", b"frame".as_slice())));
    assert_eq!(unwrap_relay(b"\x40\x00short"), None);
    assert_eq!(unwrap_relay(&[]), None);
}
//#endregion 🔖️Relay

//#region 🔖️Server
#[tokio::test]
async fn build_collects_every_module_contribution() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() })
        .identity("hub", "0.1.0")
        .module(CountingModule)
        .app("admin", "/srv/admin")
        .admin_token(Some("secret".to_string()))
        .build()
        .await
        .expect("the test instance opens its stores");

    assert_eq!(server.definition().modules.len(), 1);
    assert_eq!(server.definition().id, "hub");
    assert!(server.state().admin.is_configured());
    assert_eq!(server.state().apps.names(), vec!["admin".to_string()]);
    assert!(server.state().documents.is_none());

    let request = admission_request(&envelope());
    assert!(server.state().authorize(&request).is_err());
    server.state().policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    assert!(server.state().authorize(&request).is_ok());
    let _ = server.router();
}

#[tokio::test]
async fn an_unroutable_command_is_rejected_and_publishes_nothing() {
    let state = state().await;
    let mut live = state.fanout.subscribe(&stream_lane(&actor("c1")));
    let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), WireJson(envelope())).await.unwrap();
    assert!(matches!(outcome.0, CommandOutcome::Rejected { .. }));
    assert!(tokio::time::timeout(Duration::from_millis(20), live.recv()).await.is_err());
}

#[tokio::test]
async fn the_clock_never_goes_backwards() {
    let state = state().await;
    let first = state.now();
    let second = state.now();
    assert!(second > first);
}

#[tokio::test]
async fn the_instance_supplies_the_storage_the_builder_wires_in() {
    let state = state().await;
    state.projections.write().await.put("roster", "space-1", vec![7]).await.expect("written");
    assert_eq!(state.projections.read().await.get("roster", "space-1").await, Some(vec![7]));
    assert_eq!(state.projections.read().await.checkpoint("roster").await, 0);
    assert!(state.sessions.lock().await.get(&crate::contract::SessionId("nobody".into())).await.is_none());
    assert_eq!(state.profile.data_dir(), Some("/tmp/semio-gateway"));
}

#[tokio::test]
async fn an_instance_that_registers_no_query_handler_answers_not_found() {
    let state = state().await;
    grant(&state, PolicyPoint::QueryAccess, "read");
    assert_eq!(state.queries.len(), 0);
    let error = post_query(HeaderMap::new(), ConnectInfo(loopback()), State(state), WireJson(query())).await.expect_err("no handler is registered");
    assert_eq!(error.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_module_registers_its_deciders_on_the_bus_it_was_built_into() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).build().await.expect("built");
    let state = server.state().clone();
    state.policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state), WireJson(envelope())).await.unwrap();
    match outcome.0 {
        CommandOutcome::Accepted { events, .. } => assert_eq!(events[0].kind, "counter.incremented"),
        other => panic!("expected acceptance, got {other:?}"),
    }
}

#[tokio::test]
async fn a_committed_event_reaches_the_instance_saga_runner_exactly_once() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).saga(TestSagas::Silent(SilentSaga)).build().await.expect("built");
    let state = server.state().clone();
    {
        let mut engine = state.policy.write().unwrap();
        engine.assign("anonymous".to_string(), "author".to_string());
        engine.assign(crate::policy::principal_key(&Principal::User { id: "alice".into() }), "author".to_string());
    }
    assert_eq!(state.sagas.lock().await.sagas.len(), 2, "the module's workflow and the builder's are both on the one runner");

    let committed = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), WireJson(envelope())).await.unwrap();
    assert!(matches!(committed.0, CommandOutcome::Accepted { .. }));
    let reactions = state.drain_sagas(64).await;
    assert_eq!(reactions.len(), 1, "one committed event, one follow-up turn");
    assert!(matches!(reactions[0], CommandOutcome::Accepted { .. }));

    let replayed = state.drain_sagas(64).await;
    assert!(replayed.iter().all(|outcome| matches!(outcome, CommandOutcome::Accepted { ref events, .. } if events.is_empty())), "a re-issued follow-up is deduplicated by its idempotency key rather than committing twice");
    assert!(state.store.read().await.pending_outbox(64).await.unwrap().is_empty(), "every drained row is acknowledged");
}

#[tokio::test]
async fn the_lane_has_counted_what_a_command_committed_when_it_answers() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).build().await.expect("built");
    let state = server.state().clone();
    state.policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    assert_eq!(state.committed(), Committed::default());
    let CommandOutcome::Accepted { events, .. } = state.submit(envelope()).await else { panic!("accepted") };
    let owed = state.store.read().await.pending_outbox(64).await.unwrap().len() as u64;
    assert_eq!(state.committed(), Committed { events: events.len() as u64, outbox: owed });
    assert!(owed > 0, "the counting module queues an outbox row per event");
    let refused = CommandEnvelope { kind: "counter.unknown".into(), command_id: CommandId("cmd-2".into()), ..envelope() };
    assert!(matches!(state.submit(refused).await, CommandOutcome::Rejected { .. }));
    assert_eq!(state.committed().events, events.len() as u64, "a refused command commits nothing");
}

fn query() -> QueryEnvelope {
    QueryEnvelope { query_id: QueryId("q-1".into()), kind: "counter.value".into(), version: 1, scope: Scope("space-1".into()), principal: Principal::Anonymous, arguments: Vec::new(), consistency: QueryConsistency::Local, cursor: None }
}

fn envelope() -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId("cmd-1".into()),
        kind: "counter.increment".into(),
        version: 1,
        target: actor("c1"),
        scope: Scope("space-1".into()),
        principal: Principal::Anonymous,
        session: None,
        device: None,
        payload: vec![1],
        causal_frontier: None,
        client_hlc: HybridLogicalClock::default(),
        expected_revision: None,
        idempotency_key: None,
        capability_proof: None,
        trace: TraceContext::default(),
    }
}
//#endregion 🔖️Server

#[test]
fn document_socket_identity_binds_from_resolved_actor_never_query() {
    let resolved = Resolved {
        principal: Principal::User { id: "alice".into() },
        session: Some(crate::contract::SessionId("session-1".into())),
        device: None,
        via: "test".into(),
        actor: Some("hub.v1.alice".into()),
    };
    let identity = document_socket_identity(&resolved).expect("binds");
    assert_eq!(identity.actor, "hub.v1.alice");
    assert_eq!(identity.session, "session-1");
}

#[test]
fn document_socket_identity_rejects_anonymous_without_grant() {
    let resolved = Resolved { principal: Principal::Anonymous, session: None, device: None, via: "anonymous".into(), actor: None };
    assert!(document_socket_identity(&resolved).is_err());
}

//#region 🔖️PresenceStream
type ClientFrames = futures::channel::mpsc::UnboundedSender<Result<Message, std::convert::Infallible>>;
type ServerFrames = futures::channel::mpsc::UnboundedReceiver<Message>;

fn quick_presence(idle: Duration) -> PresenceSettings {
    PresenceSettings { tick: Duration::from_millis(20), idle, keepalive: Duration::from_secs(30), ..PresenceSettings::default() }
}

async fn presence_server(settings: PresenceSettings) -> ServerState<TestInstance> {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).presence(settings).build().await.expect("built");
    let state = server.state().clone();
    grant(&state, PolicyPoint::Subscription, PRESENCE_JOIN);
    grant(&state, PolicyPoint::Subscription, PRESENCE_PUBLISH);
    state
}

fn connect(state: &ServerState<TestInstance>, scope: &str, surface: &str) -> (ClientFrames, ServerFrames, tokio::task::JoinHandle<()>) {
    let (client, mut inbound) = unbounded::<Result<Message, std::convert::Infallible>>();
    let (mut outbound, frames) = unbounded::<Message>();
    let (state, scope, surface) = (state.clone(), Scope(scope.to_string()), surface.to_string());
    let task = tokio::spawn(async move { run_presence(&state, &scope, &surface, &PresenceGrant { principal: Principal::Anonymous, may_publish: true, client: caller() }, &mut outbound, &mut inbound).await });
    (client, frames, task)
}

async fn next_frame(frames: &mut ServerFrames) -> PresenceFrame {
    loop {
        match tokio::time::timeout(Duration::from_secs(5), frames.next()).await.expect("a frame arrives").expect("the socket is open") {
            Message::Text(text) => return serde_json::from_str(text.as_str()).expect("a presence frame"),
            Message::Ping(_) => continue,
            other => panic!("unexpected message {other:?}"),
        }
    }
}

fn say(client: &ClientFrames, text: &str) {
    client.unbounded_send(Ok(Message::Text(text.to_string().into()))).expect("the session is open");
}

const ROOM: &str = "presence:s";

fn entry(session: &str, state: OpaqueJson) -> PresenceEntry {
    PresenceEntry { session: session.into(), colour: 0, surface: "home".into(), state }
}

fn rooms(departures_kept: usize) -> Arc<PresenceRooms> {
    Arc::new(PresenceRooms::new(&PresenceSettings { departures_kept, max_bytes_per_second: None, ..PresenceSettings::default() }, Instant::now()))
}

/// 🧾️ What a page carries, read back the way a client reads its frame: the sessions with their
/// states in order, the sessions that left, and whether it replaces.
fn carried(page: &Page) -> (Vec<(String, OpaqueJson)>, Vec<String>, bool) {
    let PresenceFrame::Batch { entries, left } = serde_json::from_str(&page.batch()).expect("a page is a frame") else { panic!("a batch") };
    (entries.into_iter().map(|entry| (entry.session, entry.state)).collect(), left, page.replace)
}

fn sessions(page: &Page) -> Vec<String> {
    carried(page).0.into_iter().map(|(session, _)| session).collect()
}

#[test]
fn a_reader_is_sent_the_newest_state_of_each_session_that_changed_and_who_left() {
    let rooms = rooms(8);
    rooms.join(ROOM, &entry("a", OpaqueJson::Null));
    rooms.join(ROOM, &entry("b", OpaqueJson::Null));
    let mut reader = rooms.read(ROOM, Some("b"), caller());
    assert!(reader.fresh() && reader.has_unsent());
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![("b".into(), OpaqueJson::Null), ("a".into(), OpaqueJson::Null)], vec![], true), "the first page replaces and is led by the reader's own session");
    assert!(!reader.fresh() && !reader.has_unsent());
    assert!(reader.page(4096, Instant::now()).is_empty(), "a room that did not change has nothing to send");

    for n in 0..5 {
        rooms.update(ROOM, &entry("a", serde_json::json!({ "n": n })));
    }
    rooms.update(ROOM, &entry("nobody", serde_json::json!(1)));
    assert!(reader.has_unsent());
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![("a".into(), serde_json::json!({ "n": 4 }))], vec![], false), "five states are one entry: the newest");
    assert!(reader.page(4096, Instant::now()).is_empty());

    rooms.update(ROOM, &entry("b", serde_json::json!({ "x": 1 })));
    rooms.leave(ROOM, "b");
    rooms.leave(ROOM, "b");
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![], vec!["b".into()], false), "a session that left is not sent, its departure is");
    assert_eq!(rooms.roster(ROOM), [entry("a", serde_json::json!({ "n": 4 }))]);

    let mut late = rooms.read(ROOM, None, caller());
    assert_eq!(carried(&late.page(4096, Instant::now())), (vec![("a".into(), serde_json::json!({ "n": 4 }))], vec![], true), "a later reader starts with the roster and no departure");
}

#[test]
fn a_page_is_bounded_and_a_round_reaches_every_session_before_any_twice() {
    let rooms = rooms(8);
    let names: Vec<String> = (0..20).map(|n| format!("s{n:02}")).collect();
    for name in &names {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    let one = serde_json::to_string(&entry("s00", serde_json::json!({ "round": 0 }))).expect("an entry").len();
    let budget = one * 5 + 4;
    let mut reader = rooms.read(ROOM, Some("s07"), caller());
    let mut roster = Vec::new();
    while reader.has_unsent() {
        assert!(reader.arriving(), "the roster is still on its way");
        let page = reader.page(budget, Instant::now());
        assert!(page.bytes() <= budget, "{} bytes in a page of {budget}", page.bytes());
        roster.extend(sessions(&page));
    }
    assert_eq!(roster.first().map(String::as_str), Some("s07"));
    roster.sort();
    assert_eq!(roster, names, "the roster arrives whole, every session once");

    let mut sent = Vec::new();
    for round in 1..=4 {
        for name in &names {
            rooms.update(ROOM, &entry(name, serde_json::json!({ "round": round })));
        }
        let page = reader.page(budget, Instant::now());
        assert!(page.bytes() <= budget && !page.replace);
        sent.extend(sessions(&page));
    }
    assert_eq!(sent.len(), 20, "five entries fit a page");
    sent.sort();
    assert_eq!(sent, names, "with every session changing before every page, four pages still reach all twenty once");

    rooms.update(ROOM, &entry("s03", serde_json::json!("x".repeat(budget * 2))));
    let mut fresh = rooms.read(ROOM, None, caller());
    let mut large = 0;
    while fresh.has_unsent() {
        let page = fresh.page(budget, Instant::now());
        if page.bytes() > budget {
            assert_eq!(sessions(&page), ["s03"], "an entry larger than a page travels alone");
            large += 1;
        }
    }
    assert_eq!(large, 1);
}

#[test]
fn a_vacated_seat_is_taken_again_and_readers_keep_their_place() {
    let rooms = rooms(4096);
    let names: Vec<String> = (0..600).map(|n| format!("s{n:03}")).collect();
    for name in &names {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    let mut readers: Vec<RoomReader> = (0..8).map(|_| rooms.read(ROOM, None, caller())).collect();
    for reader in &mut readers {
        let mut roster = 0;
        while reader.has_unsent() {
            roster += sessions(&reader.page(4096, Instant::now())).len();
        }
        assert_eq!(roster, names.len());
    }
    for name in names.iter().skip(1).step_by(2) {
        rooms.leave(ROOM, name);
    }
    for name in ["late-a", "late-b"] {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    for reader in &mut readers {
        let (entries, left, replace) = carried(&reader.page(1 << 20, Instant::now()));
        assert_eq!((entries.iter().map(|(session, _)| session.as_str()).collect::<BTreeSet<_>>(), left.len(), replace), (BTreeSet::from(["late-a", "late-b"]), 300, false));
    }
    for round in 0..50 {
        let changing = &names[(round * 7 % 300) * 2];
        rooms.update(ROOM, &entry(changing, serde_json::json!(round)));
        for reader in &mut readers {
            assert_eq!(carried(&reader.page(8192, Instant::now())), (vec![(changing.clone(), serde_json::json!(round))], vec![], false));
        }
    }
    assert_eq!(rooms.roster(ROOM).len(), 302);
}

#[test]
fn the_tournament_finds_the_next_changed_seat_like_a_walk_of_the_seats_does() {
    let mut room = Room::default();
    let mut random = 0x9E37_79B9_7F4A_7C15_u64;
    let mut draw = move || {
        random ^= random << 13;
        random ^= random >> 7;
        random ^= random << 17;
        random
    };
    for step in 0..3000 {
        let session = format!("s{}", draw() % 97);
        if draw() % 4 == 0 {
            room.vacate(&session, 8);
        } else {
            room.seat(Member { session, wire: "{}".into() });
        }
        if step % 40 == 0 {
            let seats = room.seats.len();
            for clock in [0, room.clock / 2, room.clock.saturating_sub(3), room.clock] {
                for from in 0..=seats {
                    assert_eq!(room.next_changed(from, clock), (from..seats).find(|seat| room.changed[seats + seat] > clock), "from seat {from} after clock {clock} at step {step}");
                }
            }
        }
    }
    let seats = room.seats.len();
    assert!(seats.is_power_of_two() && seats <= 128, "97 sessions never need more than 128 seats, the room has {seats}");
    assert!(room.seats.iter().enumerate().all(|(seat, member)| member.is_some() == (room.changed[seats + seat] > 0)), "exactly the taken seats hold a clock");
    assert_eq!(room.seated.len(), room.seats.iter().flatten().count());
    assert!(room.seated.iter().all(|(session, seat)| room.seats[*seat].as_ref().is_some_and(|member| &member.session == session)));
}

#[test]
fn all_rooms_together_send_their_budget_and_readers_wait_their_turn() {
    let started = Instant::now();
    let rooms = Arc::new(PresenceRooms::new(&PresenceSettings { max_bytes_per_second: Some(4000), ..PresenceSettings::default() }, started));
    let state = serde_json::json!("x".repeat(400));
    for name in ["a", "b", "c", "d", "e", "f"] {
        rooms.join(ROOM, &entry(name, state.clone()));
        rooms.join("presence:other", &entry(name, state.clone()));
    }
    let one = serde_json::to_string(&entry("a", state)).expect("an entry").len();
    let mut readers: Vec<RoomReader> = (0..4).map(|reader| rooms.read(if reader % 2 == 0 { ROOM } else { "presence:other" }, None, caller())).collect();
    let mut sent = 0;
    let mut round = |at: Duration, sent: &mut usize| {
        let before = *sent;
        for reader in &mut readers {
            let page = reader.page(4096, started + at);
            assert!(page.is_empty() || reader.has_unsent() || page.bytes() >= 6 * one, "a page is whole or its reader knows it is behind");
            *sent += page.bytes();
        }
        *sent - before
    };
    let at_once = round(Duration::ZERO, &mut sent);
    assert!(at_once >= 1000 && at_once < 1000 + 6 * one + 5, "a quarter of a second's worth goes at once and at most one page more: {at_once} bytes");
    assert_eq!(round(Duration::ZERO, &mut sent), 0, "the budget is spent: every reader waits, in every room");
    assert_eq!(round(Duration::from_millis(100), &mut sent), 0, "and waits until what was sent beyond the budget has flowed back");
    let mut at = Duration::from_millis(100);
    while sent < 4 * (6 * one + 5) {
        at += Duration::from_millis(100);
        round(at, &mut sent);
        let allowed = 1000 + 4000 * at.as_millis() as usize / 1000 + 6 * one + 5;
        assert!(sent <= allowed, "{sent} bytes after {at:?}, the budget allows {allowed}");
        assert!(at < Duration::from_secs(10), "every reader is sent its roster in the end");
    }
    assert!(at >= Duration::from_millis(1500), "four rosters of {} bytes take their time at 4000 bytes a second, not {at:?}", 6 * one + 5);

}

#[test]
fn a_joining_member_is_welcomed_at_once_and_its_welcome_counts_against_the_budget() {
    let started = Instant::now();
    let rooms = Arc::new(PresenceRooms::new(&PresenceSettings { max_bytes_per_second: Some(4000), ..PresenceSettings::default() }, started));
    let state = serde_json::json!("x".repeat(400));
    for name in ["a", "b", "c", "d", "e", "f"] {
        rooms.join(ROOM, &entry(name, state.clone()));
    }
    let roster = 6 * serde_json::to_string(&entry("a", state)).expect("an entry").len() + 5;
    let mut watcher = rooms.read(ROOM, None, caller());
    assert_eq!(watcher.page(4096, started).bytes(), roster, "the quarter second the rooms may send at once is spent, and more");

    let mut member = rooms.read(ROOM, Some("c"), caller());
    let welcome = member.page(4096, started);
    assert_eq!((sessions(&welcome).first().map(String::as_str), welcome.bytes(), welcome.replace), (Some("c"), roster, true), "a joining member is told at once that it joined, whatever the budget");
    rooms.update(ROOM, &entry("a", OpaqueJson::Null));
    assert!(member.page(4096, started).is_empty() && member.has_unsent(), "what follows its welcome waits like everything else");

    let owed_millis = u64::try_from((2 * roster - 1000) * 1000 / 4000).expect("milliseconds");
    assert!(watcher.page(4096, started + Duration::from_millis(owed_millis - 20)).is_empty(), "the welcome was counted: both rosters have to flow back first");
    assert_eq!(sessions(&watcher.page(4096, started + Duration::from_millis(owed_millis + 20))), ["a"]);
}

#[test]
fn a_reader_is_not_held_back_by_what_another_address_took() {
    let started = Instant::now();
    let rooms = Arc::new(PresenceRooms::new(&PresenceSettings { max_bytes_per_second: Some(4000), ..PresenceSettings::default() }, started));
    let state = serde_json::json!("x".repeat(400));
    for name in ["a", "b", "c", "d", "e", "f"] {
        rooms.join(ROOM, &entry(name, state.clone()));
    }
    let roster = 6 * serde_json::to_string(&entry("a", state)).expect("an entry").len() + 5;
    let mut flood: Vec<RoomReader> = (0..4).map(|_| rooms.read(ROOM, None, address(66))).collect();
    let mut hall = rooms.read(ROOM, None, address(1));
    assert_eq!(flood[0].page(4096, started).bytes(), roster, "more than the quarter second the rooms may send at once");
    for reader in &mut flood[1..] {
        assert!(reader.page(4096, started).is_empty() && reader.has_unsent(), "the address that took it waits, through every socket it reads with");
    }
    assert_eq!(hall.page(4096, started).bytes(), roster, "another address reads at once: its even split of the budget is its own");
    let mut sent = 2 * roster;
    let mut at = Duration::ZERO;
    while flood.iter().any(RoomReader::has_unsent) {
        at += Duration::from_millis(100);
        sent += flood.iter_mut().map(|reader| reader.page(4096, started + at).bytes()).sum::<usize>();
        let allowed = 2000 + 4000 * at.as_millis() as usize / 1000 + 2 * roster;
        assert!(sent <= allowed, "{sent} bytes after {at:?}, the budget, half a second's worth and a page per address allow {allowed}");
        assert!(at < Duration::from_secs(10), "every reader is sent its roster in the end");
    }
    assert_eq!(sent, 5 * roster);
    assert!(at >= Duration::from_secs(2), "four rosters of {roster} bytes take one address its time at 4000 bytes a second, not {at:?}");
}

/// 🧮️ Play `addresses` against one budget for `span`: every address asks for `demand` bytes per
/// second through `readers` readers that each try one page of `page` bytes every ten milliseconds;
/// the bytes each address drew in every half second.
fn played(outflow: &mut Outflow, started: Instant, from: Duration, span: Duration, page: usize, addresses: &[(ClientKey, u64, usize)]) -> Vec<Vec<usize>> {
    let mut drawn = vec![vec![0; (span.as_millis() / 500) as usize]; addresses.len()];
    let mut owed = vec![0u64; addresses.len()];
    for step in 0..span.as_millis() as u64 / 10 {
        let now = started + from + Duration::from_millis(step * 10);
        for (index, (client, demand, readers)) in addresses.iter().enumerate() {
            owed[index] = owed[index].saturating_add(demand / 100);
            for _ in 0..*readers {
                if owed[index] >= page as u64 && outflow.open(*client, now) {
                    outflow.draw(*client, page);
                    owed[index] -= page as u64;
                    drawn[index][(step / 50) as usize] += page;
                }
            }
        }
    }
    drawn
}

#[test]
fn the_budget_is_dealt_fairly_among_addresses_however_many_readers_each_has() {
    let started = Instant::now();
    let (hall, flood) = (address(1), address(66));
    let mut outflow = Outflow::new(16_000, started);
    for _ in 0..3 {
        outflow.attend(hall);
    }
    let alone = played(&mut outflow, started, Duration::ZERO, Duration::from_secs(2), 100, &[(hall, 12_000, 3)]);
    assert!(alone[0].iter().all(|half| (5_900..=6_100).contains(half)), "alone, an address is sent what it asks for: {alone:?}");

    for _ in 0..200 {
        outflow.attend(flood);
    }
    let beside = played(&mut outflow, started, Duration::from_secs(2), Duration::from_secs(4), 100, &[(hall, 12_000, 3), (flood, u64::MAX / 2, 200)]);
    assert!(beside[0].iter().all(|half| *half >= 3_900), "from the first moment of a flood the other address keeps an even split of the budget: {beside:?}");
    assert!(beside[0][2..].iter().all(|half| (3_900..=4_300).contains(half)) && beside[1][2..].iter().all(|half| (3_900..=4_300).contains(half)), "two addresses that both ask for more than half are each sent half: {beside:?}");
    assert!((0..8).all(|half| beside[0][half] + beside[1][half] <= 8_000 + 8_000), "the rooms never send more than their budget and half a second's worth: {beside:?}");

    let modest = played(&mut outflow, started, Duration::from_secs(6), Duration::from_secs(4), 100, &[(hall, 5_000, 3), (flood, u64::MAX / 2, 200)]);
    assert!(modest[0].iter().all(|half| (2_400..=2_600).contains(half)), "an address that asks for less than an even split is sent all of it: {modest:?}");
    assert!(modest[1][2..].iter().all(|half| (5_200..=5_700).contains(half)), "and what it leaves goes to the one that asks for more: {modest:?}");

    for _ in 0..200 {
        outflow.depart(flood);
    }
    let after = played(&mut outflow, started, Duration::from_secs(10), Duration::from_secs(2), 100, &[(hall, 12_000, 3)]);
    assert!(after[0][1..].iter().all(|half| (5_900..=6_100).contains(half)), "the flood gone, the address is sent what it asks for again: {after:?}");
    assert_eq!(outflow.shares.len(), 1, "an address without a reader holds no share");
}

#[test]
fn what_modest_addresses_leave_is_split_evenly_among_those_that_ask_for_more() {
    let started = Instant::now();
    let (hall, flood) = (address(1), address(66));
    let homes: Vec<ClientKey> = (0..100u32).map(|home| ClientKey::of(IpAddr::V4(Ipv4Addr::from(0x0A00_0000 + home)))).collect();
    let mut outflow = Outflow::new(64_000, started);
    let mut addresses: Vec<(ClientKey, u64, usize)> = homes.iter().map(|home| (*home, 200, 1)).collect();
    addresses.extend([(hall, 36_000, 9), (flood, u64::MAX / 2, 200)]);
    for (client, _, readers) in &addresses {
        for _ in 0..*readers {
            outflow.attend(*client);
        }
    }
    let drawn = played(&mut outflow, started, Duration::ZERO, Duration::from_secs(6), 100, &addresses);
    for home in &drawn[..100] {
        assert_eq!(home.iter().sum::<usize>(), 1_200, "an address that asks for little is sent all of it, flood or not: {home:?}");
    }
    let (hall, flood) = (&drawn[100], &drawn[101]);
    assert!(hall.iter().all(|half| *half >= 300), "the even split of a hundred and two addresses is the least an address is ever sent: {hall:?}");
    assert!(hall[4..].iter().all(|half| (10_000..=11_500).contains(half)) && flood[4..].iter().all(|half| (10_000..=11_500).contains(half)), "what the hundred leave of 64000 bytes a second — 44000 — is halved between the two that ask for more: hall {hall:?}, flood {flood:?}");
}

#[test]
fn a_reader_that_missed_a_forgotten_departure_starts_over() {
    let rooms = rooms(2);
    for name in ["a", "b", "c", "d", "e"] {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    let mut reader = rooms.read(ROOM, Some("a"), caller());
    assert_eq!(sessions(&reader.page(4096, Instant::now())).len(), 5);
    rooms.leave(ROOM, "b");
    rooms.leave(ROOM, "c");
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![], vec!["b".into(), "c".into()], false), "two departures are remembered");
    for name in ["d", "e"] {
        rooms.leave(ROOM, name);
    }
    rooms.join(ROOM, &entry("f", OpaqueJson::Null));
    rooms.leave(ROOM, "f");
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![("a".into(), OpaqueJson::Null)], vec![], true), "three departures are one too many: the roster replaces what the reader held");
    assert!(reader.page(4096, Instant::now()).is_empty());
}

#[test]
fn a_room_lives_while_it_has_a_member_or_a_reader() {
    let rooms = rooms(8);
    assert_eq!(rooms.rooms(), 0);
    let watching = rooms.read(ROOM, None, caller());
    assert_eq!(rooms.rooms(), 1, "a watched room exists before anybody joins it");
    rooms.join(ROOM, &entry("a", OpaqueJson::Null));
    drop(watching);
    assert_eq!(rooms.rooms(), 1);
    let reading = rooms.read(ROOM, Some("a"), caller());
    rooms.leave(ROOM, "a");
    assert_eq!(rooms.rooms(), 1, "its departure is still to be read");
    drop(reading);
    assert_eq!(rooms.rooms(), 0);
    assert!(rooms.roster(ROOM).is_empty());
}

#[test]
fn a_page_is_written_as_the_frame_serde_writes() {
    let rooms = rooms(8);
    let first = entry("a\"1", serde_json::json!({ "cursor": { "x": 0.5 }, "text": "é\n" }));
    rooms.join(ROOM, &first);
    rooms.join(ROOM, &entry("b", OpaqueJson::Null));
    let mut reader = rooms.read(ROOM, Some("a\"1"), caller());
    let page = reader.page(4096, Instant::now());
    let roster = vec![first.clone(), entry("b", OpaqueJson::Null)];
    assert_eq!(page.welcome("a\"1", 7), serde_json::to_string(&PresenceFrame::Welcome { session: "a\"1".into(), colour: 7, roster: roster.clone() }).expect("serde"));
    assert_eq!(page.watched("room \"x\""), serde_json::to_string(&PresenceFrame::Watched { scope: Scope("room \"x\"".into()), entries: roster, left: vec![], snapshot: true }).expect("serde"));
    rooms.leave(ROOM, "b");
    rooms.update(ROOM, &first);
    let page = reader.page(4096, Instant::now());
    assert_eq!(page.batch(), serde_json::to_string(&PresenceFrame::Batch { entries: vec![first.clone()], left: vec!["b".into()] }).expect("serde"));
    assert_eq!(page.watched("r"), serde_json::to_string(&PresenceFrame::Watched { scope: Scope("r".into()), entries: vec![first], left: vec!["b".into()], snapshot: false }).expect("serde"));
}

#[tokio::test]
async fn a_reader_waits_for_its_room_to_change() {
    let rooms = rooms(8);
    rooms.join(ROOM, &entry("a", OpaqueJson::Null));
    let mut reader = rooms.read(ROOM, None, caller());
    tokio::time::timeout(Duration::from_secs(5), reader.unsent()).await.expect("a fresh reader has its roster to read");
    reader.page(4096, Instant::now());
    assert!(tokio::time::timeout(Duration::from_millis(60), reader.unsent()).await.is_err(), "nothing changed");
    let writer = Arc::clone(&rooms);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        writer.update(ROOM, &entry("a", serde_json::json!(1)));
    });
    tokio::time::timeout(Duration::from_secs(5), reader.unsent()).await.expect("the change wakes the reader");
    tokio::time::timeout(Duration::from_secs(5), reader.unsent()).await.expect("and it stays due until it is read");
    assert_eq!(sessions(&reader.page(4096, Instant::now())), ["a"]);
}

#[test]
fn a_state_frame_is_bounded_by_size_rate_and_shape() {
    let settings = PresenceSettings::default();
    let now = Instant::now();
    let mut budget = StateBudget::new(settings.max_states_per_second);
    assert_eq!(admit_frame(r#"{"type":"state","state":{"x":1}}"#, &settings, &mut budget, now), Ok(Some(PresenceFrame::State { state: serde_json::json!({ "x": 1 }) })));
    let large = format!(r#"{{"type":"state","state":"{}"}}"#, "x".repeat(settings.max_state_bytes));
    assert_eq!(admit_frame(&large, &settings, &mut budget, now), Err(REFUSED_TOO_LARGE.to_string()));
    assert_eq!(admit_frame(r#"{"type":"batch","entries":[],"left":[]}"#, &settings, &mut budget, now), Err(REFUSED_INVALID.to_string()));
    assert_eq!(admit_frame("not json", &settings, &mut budget, now), Err(REFUSED_INVALID.to_string()));
    let mut spent = StateBudget::new(settings.max_states_per_second);
    let start = Instant::now();
    let admitted = (0..40).filter(|_| admit_frame(r#"{"type":"state","state":1}"#, &settings, &mut spent, start) == Ok(Some(PresenceFrame::State { state: serde_json::json!(1) }))).count();
    assert_eq!(admitted, settings.max_states_per_second as usize);
    assert_eq!(admit_frame(r#"{"type":"state","state":1}"#, &settings, &mut spent, start + Duration::from_secs(1)), Ok(Some(PresenceFrame::State { state: serde_json::json!(1) })));
}

#[test]
fn presence_session_ids_are_unique_hex() {
    let first = presence_session_id();
    let second = presence_session_id();
    assert_ne!(first, second);
    assert!(first.len() == 32 && first.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
}

#[tokio::test]
async fn presence_admission_needs_the_join_grant_a_short_surface_and_an_admitted_origin() {
    let state = state().await;
    let scope = Scope("space-1".into());
    let refused = admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect_err("closed by default");
    assert_eq!(refused.status(), StatusCode::FORBIDDEN);
    grant(&state, PolicyPoint::Subscription, PRESENCE_JOIN);
    assert!(!admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect("joins").may_publish, "join alone does not publish");
    grant(&state, PolicyPoint::Subscription, PRESENCE_PUBLISH);
    assert_eq!(admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect("joins and publishes"), PresenceGrant { principal: Principal::Anonymous, may_publish: true, client: caller() });
    let long = "x".repeat(65);
    assert_eq!(admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, &long).await.expect_err("too long").status(), StatusCode::BAD_REQUEST);

    let mut gated = state.clone();
    gated.origin_admission = Some(Arc::new(|origin: &str| origin == "https://quizzes.example"));
    let mut foreign = HeaderMap::new();
    foreign.insert(header::ORIGIN, HeaderValue::from_static("https://evil.example"));
    assert_eq!(admit_presence(&gated, &foreign, Some(loopback()), &scope, "home").await.expect_err("foreign origin").status(), StatusCode::FORBIDDEN);
    let mut site = HeaderMap::new();
    site.insert(header::ORIGIN, HeaderValue::from_static("https://quizzes.example"));
    assert!(admit_presence(&gated, &site, Some(loopback()), &scope, "home").await.is_ok());
    assert!(admit_presence(&gated, &HeaderMap::new(), Some(loopback()), &scope, "home").await.is_ok(), "a non-browser client presents no origin");
}

#[tokio::test]
async fn two_sessions_share_coalesced_state_and_see_each_other_leave() {
    let state = presence_server(quick_presence(Duration::from_secs(30))).await;
    let (alice, mut alice_frames, _alice_task) = connect(&state, "room-1", "home");
    let PresenceFrame::Welcome { session: alice_session, colour: alice_colour, roster } = next_frame(&mut alice_frames).await else { panic!("welcome first") };
    assert_eq!(roster.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), [alice_session.as_str()]);

    let (bob, mut bob_frames, bob_task) = connect(&state, "room-1", "leaderboard");
    let PresenceFrame::Welcome { session: bob_session, colour: bob_colour, roster } = next_frame(&mut bob_frames).await else { panic!("welcome first") };
    assert_ne!(alice_colour, bob_colour);
    assert_eq!(roster.len(), 2);
    assert!(roster.iter().any(|entry| entry.session == alice_session && entry.surface == "home"));

    for n in 0..5 {
        say(&alice, &format!(r#"{{"type":"state","state":{{"n":{n}}}}}"#));
    }
    let mut seen = Vec::new();
    while seen.last() != Some(&serde_json::json!({ "n": 4 })) {
        if let PresenceFrame::Batch { entries, .. } = next_frame(&mut bob_frames).await {
            seen.extend(entries.into_iter().filter(|entry| entry.session == alice_session).map(|entry| entry.state).filter(|state| !state.is_null()));
        }
    }
    assert!(seen.len() <= 2, "five states sent within one tick arrive coalesced: {seen:?}");

    say(&alice, r#"{"type":"state","state":{"refuse":"not-here"}}"#);
    let mut refusal = next_frame(&mut alice_frames).await;
    while let PresenceFrame::Batch { .. } = refusal {
        refusal = next_frame(&mut alice_frames).await;
    }
    assert_eq!(refusal, PresenceFrame::Refused { reason: "not-here".into() });
    alice.unbounded_send(Ok(Message::Binary(vec![1, 2].into()))).expect("open");
    assert_eq!(next_frame(&mut alice_frames).await, PresenceFrame::Refused { reason: REFUSED_INVALID.into() });

    drop(bob);
    tokio::time::timeout(Duration::from_secs(5), bob_task).await.expect("bob's session ends").expect("no panic");
    loop {
        if let PresenceFrame::Batch { left, .. } = next_frame(&mut alice_frames).await {
            if left.contains(&bob_session) {
                break;
            }
        }
    }
    assert_eq!(state.presence_rooms.roster(&presence_lane(&Scope("room-1".into()))).len(), 1);
    assert_eq!(state.presence.colour_of(&presence_lane(&Scope("room-1".into())), &bob_session), None);
}

#[tokio::test]
async fn an_idle_session_is_closed_and_leaves_the_room() {
    let state = presence_server(quick_presence(Duration::from_millis(150))).await;
    let (_silent, mut frames, task) = connect(&state, "room-idle", "home");
    assert!(matches!(next_frame(&mut frames).await, PresenceFrame::Welcome { .. }));
    tokio::time::timeout(Duration::from_secs(5), task).await.expect("the idle session is closed").expect("no panic");
    let rest: Vec<Message> = frames.collect().await;
    assert!(matches!(rest.last(), Some(Message::Close(_))), "{rest:?}");
    assert!(state.presence_rooms.roster(&presence_lane(&Scope("room-idle".into()))).is_empty());
}

fn scopes(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn watched(scope: &str, entries: Vec<PresenceEntry>, left: &[&str], snapshot: bool) -> PresenceFrame {
    PresenceFrame::Watched { scope: Scope(scope.into()), entries, left: left.iter().map(|session| (*session).to_string()).collect(), snapshot }
}

/// 📏️ The bytes of entries and departures a server frame carries.
fn carried_bytes(frame: &PresenceFrame) -> usize {
    let (entries, left) = match frame {
        PresenceFrame::Welcome { roster, .. } => (roster, None),
        PresenceFrame::Batch { entries, left } | PresenceFrame::Watched { entries, left, .. } => (entries, Some(left)),
        _ => return 0,
    };
    let listed = |texts: Vec<String>| texts.iter().map(String::len).sum::<usize>() + texts.len().saturating_sub(1);
    listed(entries.iter().map(|entry| serde_json::to_string(entry).expect("an entry")).collect()) + listed(left.into_iter().flatten().map(|session| serde_json::to_string(session).expect("a session")).collect())
}

#[tokio::test]
async fn a_roster_larger_than_a_frame_arrives_over_several_frames_and_so_does_a_watched_one() {
    let frame_bytes = 400;
    let state = presence_server(PresenceSettings { max_frame_bytes: frame_bytes, ..quick_presence(Duration::from_secs(30)) }).await;
    grant_scoped(&state, PRESENCE_WATCH, &["hall"]);
    let mut members = Vec::new();
    for _ in 0..12 {
        let (client, mut frames, _task) = connect(&state, "hall", "home");
        assert!(matches!(next_frame(&mut frames).await, PresenceFrame::Welcome { .. }));
        members.push((client, frames));
    }
    let mut everyone: Vec<String> = state.presence_rooms.roster(&presence_lane(&Scope("hall".into()))).into_iter().map(|entry| entry.session).collect();

    let (_late, mut late_frames, _late_task) = connect(&state, "hall", "home");
    let PresenceFrame::Welcome { session: late_session, roster, .. } = next_frame(&mut late_frames).await else { panic!("welcome first") };
    assert_eq!(roster.first().map(|entry| entry.session.as_str()), Some(late_session.as_str()), "the welcome is led by the joiner");
    assert!(roster.len() < 13, "thirteen entries do not fit a frame of {frame_bytes} bytes");
    let mut known: BTreeSet<String> = roster.into_iter().map(|entry| entry.session).collect();
    let mut frames = 1;
    while known.len() < 13 {
        let frame = next_frame(&mut late_frames).await;
        assert!(carried_bytes(&frame) <= frame_bytes, "{frame:?}");
        let PresenceFrame::Batch { entries, .. } = frame else { panic!("the roster continues in batches") };
        known.extend(entries.into_iter().map(|entry| entry.session));
        frames += 1;
    }
    everyone.push(late_session);
    assert_eq!(known, everyone.into_iter().collect::<BTreeSet<_>>());
    assert!(frames >= 3, "the roster took {frames} frames");

    let (watcher, mut watcher_frames, _watcher_task) = connect(&state, "lobby", "home");
    assert!(matches!(next_frame(&mut watcher_frames).await, PresenceFrame::Welcome { .. }));
    say(&watcher, r#"{"type":"watch","scopes":["hall"],"intervalMs":40}"#);
    let PresenceFrame::Watched { entries, snapshot: true, .. } = watched_frame(&mut watcher_frames).await else { panic!("a snapshot first") };
    let mut seen: BTreeSet<String> = entries.into_iter().map(|entry| entry.session).collect();
    while seen.len() < 13 {
        let frame = watched_frame(&mut watcher_frames).await;
        assert!(carried_bytes(&frame) <= frame_bytes, "{frame:?}");
        let PresenceFrame::Watched { entries, snapshot: false, .. } = frame else { panic!("the snapshot continues in changes") };
        seen.extend(entries.into_iter().map(|entry| entry.session));
    }
    assert_eq!(seen, known);
}

#[tokio::test]
async fn a_peer_that_takes_no_frame_is_let_go_and_holds_nothing() {
    let state = presence_server(quick_presence(Duration::from_millis(300))).await;
    let (client, mut inbound) = unbounded::<Result<Message, std::convert::Infallible>>();
    let (mut outbound, _never_read) = futures::channel::mpsc::channel::<Message>(0);
    let scope = Scope("stuck".to_string());
    let session = state.clone();
    let stuck = tokio::spawn(async move { run_presence(&session, &scope, "home", &PresenceGrant { principal: Principal::Anonymous, may_publish: true, client: caller() }, &mut outbound, &mut inbound).await });
    let chatter = tokio::spawn(async move {
        let mut n = 0;
        while client.unbounded_send(Ok(Message::Text(format!(r#"{{"type":"state","state":{n}}}"#).into()))).is_ok() {
            n += 1;
            tokio::time::sleep(Duration::from_millis(40)).await;
        }
    });
    tokio::time::timeout(Duration::from_secs(5), stuck).await.expect("a session that keeps talking and takes none of its own echoes ends").expect("no panic");
    tokio::time::timeout(Duration::from_secs(5), chatter).await.expect("its peer finds the socket closed").expect("no panic");
    assert_eq!(state.presence_rooms.rooms(), 0, "and it left the room");
}

#[test]
fn the_watch_interval_is_clamped_between_the_tick_and_the_slowest_interval() {
    let settings = PresenceSettings::default();
    assert_eq!(watch_interval(0, &settings), settings.tick);
    assert_eq!(watch_interval(40, &settings), settings.tick);
    assert_eq!(watch_interval(250, &settings), Duration::from_millis(250));
    assert_eq!(watch_interval(u64::MAX, &settings), settings.max_watch_interval);
    let mut budget = StateBudget::new(settings.max_states_per_second);
    assert_eq!(admit_frame(r#"{"type":"watch","scopes":["a"],"intervalMs":250}"#, &settings, &mut budget, Instant::now()), Ok(Some(PresenceFrame::Watch { scopes: vec![Scope("a".into())], interval_ms: 250 })));
    assert_eq!(admit_frame(r#"{"type":"watched","scope":"a","entries":[],"left":[]}"#, &settings, &mut budget, Instant::now()), Err(REFUSED_INVALID.to_string()), "watched travels server → client only");
}

fn grant_scoped(state: &ServerState<TestInstance>, action: &str, scopes: &[&str]) {
    let mut engine = state.policy.write().unwrap();
    let name = format!("presence-{action}");
    engine.register_template(PolicyTemplate { name: name.clone(), auto_apply: false, grants: vec![PolicyGrant { point: PolicyPoint::Subscription, resource: PRESENCE_RESOURCE.into(), action: action.to_string() }] });
    for scope in scopes {
        engine.assign_scoped("anonymous".to_string(), Scope((*scope).to_string()), name.clone());
    }
}

#[tokio::test]
async fn a_watch_is_admitted_per_scope_and_bounded() {
    let state = state().await;
    let wanted = [Scope("room-a".into()), Scope("room-b".into()), Scope("room-a".into())];
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Err("forbidden room-a".to_string()), "closed by default");
    grant_scoped(&state, PRESENCE_WATCH, &["room-a"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Err("forbidden room-b".to_string()));
    grant_scoped(&state, PRESENCE_WATCH, &["room-b"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Ok(scopes(&["room-a", "room-b"])));
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &[]), Ok(BTreeSet::new()), "an empty watch stops watching");
    let many: Vec<Scope> = (0..=state.presence_settings.max_watch_scopes).map(|_| Scope("room-a".into())).collect();
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &many), Err(REFUSED_WATCH_TOO_MANY.to_string()));
    grant_scoped(&state, PRESENCE_JOIN, &["room-c"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &[Scope("room-c".into())]), Err("forbidden room-c".to_string()), "joining is not watching");
}

async fn watched_frame(frames: &mut ServerFrames) -> PresenceFrame {
    loop {
        let frame = next_frame(frames).await;
        if matches!(frame, PresenceFrame::Watched { .. } | PresenceFrame::Refused { .. }) {
            return frame;
        }
    }
}

#[tokio::test]
async fn a_socket_watches_other_rooms_read_only_at_its_own_interval() {
    let state = presence_server(quick_presence(Duration::from_secs(30))).await;
    grant_scoped(&state, PRESENCE_WATCH, &["room-a", "room-b"]);
    let (alice, mut alice_frames, _alice_task) = connect(&state, "room-a", "home");
    let PresenceFrame::Welcome { session: alice_session, .. } = next_frame(&mut alice_frames).await else { panic!("welcome") };
    let (bob, mut bob_frames, bob_task) = connect(&state, "room-b", "home");
    let PresenceFrame::Welcome { session: bob_session, .. } = next_frame(&mut bob_frames).await else { panic!("welcome") };
    let (carol, mut carol_frames, carol_task) = connect(&state, "room-c", "home");
    let PresenceFrame::Welcome { session: carol_session, .. } = next_frame(&mut carol_frames).await else { panic!("welcome") };

    say(&carol, r#"{"type":"watch","scopes":["room-b","room-a"],"intervalMs":80}"#);
    let PresenceFrame::Watched { scope, entries, left, snapshot: true } = watched_frame(&mut carol_frames).await else { panic!("a snapshot") };
    assert_eq!((scope.0.as_str(), entries.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), left.len()), ("room-a", vec![alice_session.as_str()], 0));
    let PresenceFrame::Watched { scope, entries, snapshot: true, .. } = watched_frame(&mut carol_frames).await else { panic!("a snapshot") };
    assert_eq!((scope.0.as_str(), entries.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>()), ("room-b", vec![bob_session.as_str()]));

    let started = Instant::now();
    for n in 0..8 {
        say(&alice, &format!(r#"{{"type":"state","state":{{"n":{n}}}}}"#));
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let intervals = started.elapsed().as_millis() / 80 + 2;
    let mut seen = Vec::new();
    while seen.last() != Some(&serde_json::json!({ "n": 7 })) {
        let PresenceFrame::Watched { scope, entries, snapshot: false, .. } = watched_frame(&mut carol_frames).await else { panic!("a coalesced change") };
        if scope.0 == "room-a" {
            seen.extend(entries.into_iter().filter(|entry| entry.session == alice_session && !entry.state.is_null()).map(|entry| entry.state));
        }
    }
    assert!(seen.len() < 8 && seen.len() as u128 <= intervals, "eight states reach an 80 ms watcher at most once per interval ({intervals}): {seen:?}");

    say(&carol, r#"{"type":"state","state":{"here":"room-c"}}"#);
    say(&carol, r#"{"type":"watch","scopes":["room-a","room-z"],"intervalMs":80}"#);
    assert_eq!(watched_frame(&mut carol_frames).await, PresenceFrame::Refused { reason: "forbidden room-z".into() });
    assert!(state.presence_rooms.roster(&presence_lane(&Scope("room-a".into()))).iter().all(|entry| entry.session != carol_session), "watching never joins");

    drop(bob);
    tokio::time::timeout(Duration::from_secs(5), bob_task).await.expect("bob's session ends").expect("no panic");
    assert_eq!(watched_frame(&mut carol_frames).await, watched("room-b", Vec::new(), &[bob_session.as_str()], false), "a refused watch kept the previous set");

    say(&carol, r#"{"type":"watch","scopes":[],"intervalMs":80}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;
    say(&alice, r#"{"type":"state","state":{"n":8}}"#);
    let quiet = tokio::time::timeout(Duration::from_millis(400), watched_frame(&mut carol_frames)).await;
    assert!(quiet.is_err(), "an empty watch stops watching: {quiet:?}");

    drop((alice, carol));
    tokio::time::timeout(Duration::from_secs(5), carol_task).await.expect("carol's session ends").expect("no panic");
    let deadline = Instant::now() + Duration::from_secs(5);
    while state.presence_rooms.rooms() > 0 && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert_eq!(state.presence_rooms.rooms(), 0, "closing releases every room the socket joined or watched");
}
//#endregion 🔖️PresenceStream

//#region 🔖️Edge
use tower::ServiceExt;

fn remote(last: u8) -> SocketAddr {
    SocketAddr::from(([198, 51, 100, last], 40_000))
}

fn forwarded(value: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(FORWARDED_FOR, HeaderValue::from_static(value));
    headers
}

/// 📨️ One request as the listener hands it to the router: with the peer's address attached.
fn request(method: Method, path: &str, peer: SocketAddr, headers: &[(&'static str, &'static str)], body: Vec<u8>) -> Request {
    let mut request = Request::new(axum::body::Body::from(body));
    *request.method_mut() = method;
    *request.uri_mut() = path.parse().expect("a path");
    for (name, value) in headers {
        request.headers_mut().insert(*name, HeaderValue::from_static(value));
    }
    request.extensions_mut().insert(ConnectInfo(peer));
    request
}

async fn answer(router: &Router, request: Request) -> (StatusCode, HeaderMap, serde_json::Value) {
    let response = router.clone().oneshot(request).await.expect("the router answers");
    let (status, headers) = (response.status(), response.headers().clone());
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("a body");
    (status, headers, serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null))
}

fn command_body() -> Vec<u8> {
    serde_json::to_vec(&envelope()).expect("an envelope")
}

const JSON: (&str, &str) = ("content-type", "application/json");

#[test]
fn the_client_address_is_the_peer_unless_a_trusted_proxy_names_it() {
    let proxy = SocketAddr::from(([10, 0, 0, 2], 55_000));
    let named = ClientKey::of("203.0.113.7".parse().expect("an address"));
    assert_eq!(client_key(ClientAddressing::Peer, &forwarded("203.0.113.7"), Some(proxy)), ClientKey::of(proxy.ip()), "an untrusted header is never read");
    assert_eq!(client_key(ClientAddressing::Forwarded, &forwarded("203.0.113.7"), Some(proxy)), named);
    assert_eq!(client_key(ClientAddressing::Forwarded, &forwarded("192.0.2.1, 203.0.113.7"), Some(proxy)), named, "the proxy's own entry is the last one");
    let mut twice = forwarded("192.0.2.1");
    twice.append(FORWARDED_FOR, HeaderValue::from_static("203.0.113.7"));
    assert_eq!(client_key(ClientAddressing::Forwarded, &twice, Some(proxy)), named, "of several header lines the last is the proxy's");
    assert_eq!(client_key(ClientAddressing::Forwarded, &HeaderMap::new(), Some(proxy)), ClientKey::of(proxy.ip()), "a probe beside the proxy is its own peer");
    assert_eq!(client_key(ClientAddressing::Forwarded, &forwarded("unknown"), Some(proxy)), ClientKey::of(proxy.ip()));
    assert_eq!(client_key(ClientAddressing::Peer, &HeaderMap::new(), None), ClientKey::of(IpAddr::V4(Ipv4Addr::UNSPECIFIED)));
}

#[test]
fn a_request_spends_from_the_bucket_of_its_class() {
    let plain = HeaderMap::new();
    assert_eq!(request_class(&Method::POST, "/commands", &plain), RequestClass::Write);
    assert_eq!(request_class(&Method::PUT, "/blobs/00", &plain), RequestClass::Write);
    assert_eq!(request_class(&Method::POST, QUERIES_PATH, &plain), RequestClass::Read);
    assert_eq!(request_class(&Method::GET, "/actors/t/k/i/events", &plain), RequestClass::Read);
    assert_eq!(request_class(&Method::OPTIONS, "/commands", &plain), RequestClass::Read);
    let mut upgrade = HeaderMap::new();
    upgrade.insert(header::UPGRADE, HeaderValue::from_static("WebSocket"));
    assert_eq!(request_class(&Method::GET, "/scopes/s/presence/ws", &upgrade), RequestClass::Upgrade);
}

#[tokio::test]
async fn a_refusal_names_its_wait_in_the_header_and_in_the_gateways_own_body() {
    let throttled = ServerError::from(Refusal::Throttled { retry_after: Duration::from_millis(1500) }).into_response();
    assert_eq!((throttled.status(), throttled.headers().get(header::RETRY_AFTER).map(|value| value.to_str().unwrap().to_string())), (StatusCode::TOO_MANY_REQUESTS, Some("2".to_string())));
    let body: ErrorBody = serde_json::from_slice(&axum::body::to_bytes(throttled.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, ErrorBody { kind: "throttled".into(), message: "too many requests".into(), retry_after_ms: Some(1500), allowance: None });
    let spent = ServerError::Throttled { wait: Duration::from_secs(36), allowance: Some("sign-up") };
    assert_eq!(spent.to_string(), "the sign-up allowance of this address is spent: retry in 36000 ms");
    let spent = spent.into_response();
    assert_eq!((spent.status(), spent.headers()[header::RETRY_AFTER].to_str().unwrap()), (StatusCode::TOO_MANY_REQUESTS, "36"));
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(spent.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, serde_json::json!({ "kind": "throttled", "message": "the sign-up allowance of this address is spent", "retryAfterMs": 36000, "allowance": "sign-up" }), "a spent named allowance is named");
    let overloaded = ServerError::from(Refusal::Overloaded { retry_after: Duration::from_secs(1) }).into_response();
    assert_eq!((overloaded.status(), overloaded.headers()[header::RETRY_AFTER].to_str().unwrap()), (StatusCode::SERVICE_UNAVAILABLE, "1"));
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(overloaded.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, serde_json::json!({ "kind": "overloaded", "message": "overloaded", "retryAfterMs": 1000 }));
    let missing = ServerError::NotFound("no blob".into()).into_response();
    assert!(missing.headers().get(header::RETRY_AFTER).is_none());
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&axum::body::to_bytes(missing.into_body(), usize::MAX).await.unwrap()).unwrap(), serde_json::json!({ "kind": "notFound", "message": "not found: no blob" }), "an error that does not pass names no wait");
}

#[tokio::test]
async fn a_fault_of_the_server_and_a_denial_of_its_policy_tell_the_caller_nothing() {
    for (error, status, kind, message) in [
        (ServerError::from(StorageError::Backend("disk I/O error at /srv/quiz/data/proctor.sqlite".into())), StatusCode::INTERNAL_SERVER_ERROR, "internal", "internal error"),
        (ServerError::from(StorageError::Conflict("idempotency key enroll:architecture:roster:7 is already bound to command enroll:architecture:roster:7".into())), StatusCode::CONFLICT, "conflict", "conflict: the write contradicts what is stored"),
        (ServerError::from(StorageError::SequenceGap { expected: 3, got: 5 }), StatusCode::CONFLICT, "conflict", "conflict: the write contradicts what is stored"),
        (ServerError::Forbidden("closed by default: no grant lets anonymous do 'read' on 'stream:t/quiz-roster/roster'".into()), StatusCode::FORBIDDEN, "forbidden", "forbidden"),
    ] {
        let response = error.into_response();
        assert_eq!(response.status(), status);
        let body: ErrorBody = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(body, ErrorBody { kind: kind.into(), message: message.into(), retry_after_ms: None, allowance: None });
    }
    let state = state().await;
    let denied = state.authorize(&PolicyRequest { point: PolicyPoint::EventDelivery, principal: Principal::Anonymous, scope: None, resource: "stream:t1/counter/c1".into(), action: "read".into() }).expect_err("closed by default");
    assert!(denied.to_string().contains("no grant lets anonymous"), "the operator's account keeps the reason: {denied}");
    assert_eq!(denied.message(), "forbidden");
}

async fn limited(limits: Limits, addressing: ClientAddressing, routes: RouteGroups, disclosure: InstanceDisclosure) -> (Server<TestInstance>, Router) {
    let server = Server::<TestInstance>::builder(StorageProfile::Ephemeral).module(CountingModule).limits(limits).addressing(addressing).routes(routes).disclosure(disclosure).build().await.expect("built");
    server.state().policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    let router = server.throttled(server.router());
    (server, router)
}

#[tokio::test]
async fn a_body_past_the_limit_or_of_another_type_is_refused_in_the_gateways_own_shape() {
    let (_, router) = limited(Limits { body_bytes: Some(2048), ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[JSON], command_body())).await;
    assert_eq!((status, body["status"].as_str()), (StatusCode::OK, Some("accepted")), "a command within the limit is served: {body}");
    let mut large = envelope();
    large.payload = vec![7; 4096];
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[JSON], serde_json::to_vec(&large).unwrap())).await;
    assert_eq!((status, body), (StatusCode::PAYLOAD_TOO_LARGE, serde_json::json!({ "kind": "payloadTooLarge", "message": "payload too large" })));
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[("content-type", "text/plain")], command_body())).await;
    assert_eq!((status, body["kind"].as_str()), (StatusCode::BAD_REQUEST, Some("badRequest")), "a form or text post never reaches a handler: {body}");
    let (status, _, body) = answer(&router, request(Method::POST, "/queries", remote(1), &[JSON], b"{ not json".to_vec())).await;
    assert_eq!((status, body["kind"].as_str()), (StatusCode::BAD_REQUEST, Some("badRequest")));
}

#[tokio::test]
async fn a_body_is_taken_whole_within_its_limit_and_its_patience() {
    type Chunk = Result<Bytes, std::convert::Infallible>;
    let streamed = |chunks: Vec<&'static [u8]>| {
        let mut request = request(Method::POST, "/commands", remote(1), &[JSON], Vec::new());
        *request.body_mut() = axum::body::Body::from_stream(futures::stream::iter(chunks.into_iter().map(|chunk| Chunk::Ok(Bytes::from_static(chunk)))));
        request
    };
    let patient = Duration::from_secs(5);
    let whole = body_taken(streamed(vec![b"12345", b"67890"]), 10, patient).await.expect("ten bytes fit a limit of ten");
    assert_eq!(whole.extensions().get::<ConnectInfo<SocketAddr>>().map(|info| info.0), Some(remote(1)), "the request is the same but for its body");
    assert_eq!(axum::body::to_bytes(whole.into_body(), 64).await.expect("in hand").as_ref(), b"1234567890");
    assert!(matches!(body_taken(streamed(vec![b"12345", b"678901"]), 10, patient).await, Err(ServerError::PayloadTooLarge)), "a body that delivers more than it may is not taken");
    let declared = request(Method::POST, "/commands", remote(1), &[("content-length", "11")], Vec::new());
    assert!(matches!(body_taken(declared, 10, patient).await, Err(ServerError::PayloadTooLarge)), "a body that declares more than it may is not read at all");
    let mut silent = request(Method::POST, "/commands", remote(1), &[JSON], Vec::new());
    *silent.body_mut() = axum::body::Body::from_stream(futures::stream::once(async { Chunk::Ok(Bytes::from_static(b"{")) }).chain(futures::stream::pending()));
    let stalled = body_taken(silent, 10, Duration::from_millis(60)).await.expect_err("a body that stops arriving is given up on");
    assert_eq!((stalled.status(), stalled.kind()), (StatusCode::REQUEST_TIMEOUT, "stalled"));
}

#[tokio::test]
async fn a_refused_request_is_still_read_so_its_connection_can_go_on() {
    let rate = Rate::per_second(1, 1);
    let (_, router) = limited(Limits { writes: rate, ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let read = Arc::new(AtomicU64::new(0));
    let counted = |read: &Arc<AtomicU64>| {
        let read = Arc::clone(read);
        let mut request = request(Method::POST, "/commands", remote(1), &[JSON], Vec::new());
        *request.body_mut() = axum::body::Body::from_stream(futures::stream::iter(command_body().chunks(16).map(<[u8]>::to_vec).collect::<Vec<_>>()).map(move |chunk| {
            read.fetch_add(chunk.len() as u64, Ordering::Relaxed);
            Ok::<_, std::convert::Infallible>(Bytes::from(chunk))
        }));
        request
    };
    assert_eq!(answer(&router, counted(&read)).await.0, StatusCode::OK);
    let served = read.swap(0, Ordering::Relaxed);
    assert_eq!(served, command_body().len() as u64);
    assert_eq!(answer(&router, counted(&read)).await.0, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(read.load(Ordering::Relaxed), served, "the refused command's body was read to its end before the refusal was sent");
}

#[tokio::test]
async fn the_throttle_refuses_before_a_handler_runs_and_keys_on_the_address_the_proxy_names() {
    let rate = Rate::per_second(1, 2);
    let (server, router) = limited(Limits { writes: rate, ..Limits::default() }, ClientAddressing::Forwarded, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let proxy = remote(1);
    let hall = [JSON, ("x-forwarded-for", "203.0.113.7")];
    for _ in 0..2 {
        assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &hall, command_body())).await.0, StatusCode::OK);
    }
    let (status, headers, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, b"never read".to_vec())).await;
    assert_eq!((status, headers[header::RETRY_AFTER].to_str().unwrap(), body["kind"].as_str()), (StatusCode::TOO_MANY_REQUESTS, "1", Some("throttled")), "{body}");
    assert!(body["retryAfterMs"].as_u64().is_some_and(|wait| (1..=1000).contains(&wait)), "{body}");
    assert_eq!(server.state().store.read().await.last_seq(&actor("c1")).await.unwrap(), 2, "the refused command reached no turn");
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command_body())).await.0, StatusCode::OK, "another address behind the same proxy has its own bucket");
    assert_eq!(answer(&router, request(Method::GET, "/instance", proxy, &[("x-forwarded-for", "203.0.113.7")], Vec::new())).await.0, StatusCode::OK, "a read spends from another bucket");
    assert_eq!(server.state().throttle.in_flight(), 0, "an answered request is no longer in flight");

    let (_, direct) = limited(Limits { writes: rate, ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    for claimed in ["203.0.113.1", "203.0.113.2"] {
        assert_eq!(answer(&direct, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", claimed)], command_body())).await.0, StatusCode::OK);
    }
    assert_eq!(answer(&direct, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.3")], command_body())).await.0, StatusCode::TOO_MANY_REQUESTS, "an address a client claims buys no bucket where no proxy is trusted");
}

#[tokio::test]
async fn a_classed_command_spends_its_named_allowance_and_is_handed_back_what_kept_nothing() {
    let limits = Limits { allowances: vec![Allowance { name: CREATION, rate: Rate::per_hour(60, 2) }], ..Limits::default() };
    let (server, router) = limited(limits, ClientAddressing::Forwarded, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let proxy = remote(1);
    let hall = [JSON, ("x-forwarded-for", "203.0.113.7")];
    let command = |target: &str, kind: &str, key: &str| {
        let mut command = envelope();
        (command.target, command.kind, command.command_id, command.idempotency_key) = (actor(target), kind.into(), CommandId(key.into()), Some(crate::contract::IdempotencyKey(key.into())));
        serde_json::to_vec(&command).expect("an envelope")
    };
    for attempt in 0..5 {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command("new-a", "counter.forbid", &format!("refused-{attempt}")))).await;
        assert_eq!((status, body["status"].as_str()), (StatusCode::OK, Some("rejected")), "{body}");
    }
    for created in ["new-a", "new-b"] {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command(created, "counter.increment", created))).await;
        assert_eq!((status, body["status"].as_str(), body["events"].as_array().map(Vec::len)), (StatusCode::OK, Some("accepted"), Some(1)), "five rejected creations spent nothing: {body}");
    }
    let (status, headers, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command("new-c", "counter.increment", "new-c"))).await;
    assert_eq!((status, headers[header::RETRY_AFTER].to_str().unwrap()), (StatusCode::TOO_MANY_REQUESTS, "60"));
    assert_eq!((body["kind"].as_str(), body["allowance"].as_str(), body["message"].as_str()), (Some("throttled"), Some(CREATION), Some("the creation allowance of this address is spent")), "{body}");
    assert!(body["retryAfterMs"].as_u64().is_some_and(|wait| (59_000..=60_000).contains(&wait)), "the wait is the time the next token takes: {body}");
    assert_eq!(server.state().store.read().await.last_seq(&actor("new-c")).await.unwrap(), 0, "the refused creation reached no turn");
    for _ in 0..3 {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command("new-a", "counter.increment", "new-a"))).await;
        assert_eq!((status, body["status"].as_str(), body["events"].as_array().map(Vec::len)), (StatusCode::TOO_MANY_REQUESTS, None, None), "with the allowance spent even a replay waits: {body}");
    }
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &hall, command("c1", "counter.increment", "plain"))).await.0, StatusCode::OK, "a command of no class spends no named allowance");
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-d", "counter.increment", "new-d"))).await.0, StatusCode::OK, "another address has its own allowance");
    for _ in 0..3 {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-d", "counter.increment", "new-d"))).await;
        assert_eq!((status, body["status"].as_str(), body["events"].as_array().map(Vec::len)), (StatusCode::OK, Some("accepted"), Some(0)), "a replay is answered and spends nothing: {body}");
    }
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-e", "counter.increment", "new-e"))).await.0, StatusCode::OK, "three replays left the second token of that address");
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-f", "counter.increment", "new-f"))).await.0, StatusCode::TOO_MANY_REQUESTS);
}

#[test]
fn a_command_kept_something_when_it_committed_an_event_or_was_deferred() {
    let receipt = CommandReceipt { command_id: CommandId("c".into()), actor: actor("c1"), revision: Revision(1), accepted_at: HybridLogicalClock::default() };
    assert!(kept(&CommandOutcome::Accepted { receipt: receipt.clone(), events: vec![event(&actor("c1"), 1)], frontier: None }));
    assert!(!kept(&CommandOutcome::Accepted { receipt: receipt.clone(), events: Vec::new(), frontier: None }), "a replay and an effect-only turn commit no event");
    assert!(kept(&CommandOutcome::Transformed { receipt: receipt.clone(), canonical_events: vec![event(&actor("c1"), 1)], frontier: None, notices: Vec::new() }));
    assert!(kept(&CommandOutcome::Pending { receipt: receipt.clone(), process: crate::contract::ProcessId("p".into()) }));
    assert!(!kept(&CommandOutcome::Rejected { receipt, reason: Rejection::Invalid { detail: "no".into() }, notices: Vec::new() }));
}

#[tokio::test]
async fn the_instance_turns_requests_away_at_its_in_flight_cap() {
    let (server, router) = limited(Limits { in_flight: 1, ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let held = server.state().throttle.enter().expect("one request in flight");
    let (status, headers, body) = answer(&router, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await;
    assert_eq!((status, headers[header::RETRY_AFTER].to_str().unwrap(), body["kind"].as_str()), (StatusCode::SERVICE_UNAVAILABLE, "1", Some("overloaded")));
    drop(held);
    assert_eq!(answer(&router, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await.0, StatusCode::OK);
}

#[tokio::test]
async fn an_instance_mounts_only_the_route_groups_it_asked_for() {
    let (_, core) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let (_, all) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let hash = "00".repeat(32);
    let unused = [(Method::GET, "/apps".to_string()), (Method::GET, format!("/blobs/{hash}")), (Method::PUT, format!("/blobs/{hash}")), (Method::POST, "/scopes/space-1/ephemeral".to_string())];
    for (method, path) in &unused {
        assert_eq!(answer(&core, request(method.clone(), path, remote(1), &[JSON], b"{}".to_vec())).await.0, StatusCode::NOT_FOUND, "{method} {path} does not exist on a core instance");
        assert_ne!(answer(&all, request(method.clone(), path, remote(1), &[JSON], b"{}".to_vec())).await.0, StatusCode::NOT_FOUND, "{method} {path} exists where every group is mounted");
    }
    for path in ["/instance", "/counting/health"] {
        assert_eq!(answer(&core, request(Method::GET, path, remote(1), &[], Vec::new())).await.0, StatusCode::OK, "{path}");
    }
    assert_eq!(answer(&core, request(Method::POST, "/commands", remote(1), &[JSON], command_body())).await.0, StatusCode::OK);
    assert_eq!(answer(&core, request(Method::GET, "/actors/t1/counter/c1/events", remote(1), &[], Vec::new())).await.0, StatusCode::FORBIDDEN, "the durable lane is mounted and closed by default");
}

#[tokio::test]
async fn a_public_instance_document_says_what_to_address_and_not_how_it_is_authorized() {
    let (server, router) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Public).await;
    let (status, _, body) = answer(&router, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await;
    assert_eq!(status, StatusCode::OK);
    let disclosed: ServerInstanceDefinition = serde_json::from_value(body).expect("the definition's own shape");
    assert_eq!(disclosed, server.definition().public());
    assert_eq!((disclosed.modules[0].id.as_str(), disclosed.modules[0].policies.len()), ("counting", 0));
    assert_eq!(server.definition().modules[0].policies.len(), 1, "in process the definition is whole");
    let (_, whole) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    assert_eq!(answer(&whole, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await.2["modules"][0]["policies"][0]["name"], "author");
}

#[tokio::test]
async fn a_socket_is_counted_against_its_address_and_the_instance_until_it_closes() {
    let (server, _) = limited(Limits { sockets_per_client: 1, sockets: 2, ..Limits::default() }, ClientAddressing::Forwarded, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let state = server.state();
    let first = state.open_socket(&forwarded("203.0.113.7"), Some(remote(1))).expect("the first socket of an address");
    assert_eq!(state.open_socket(&forwarded("203.0.113.7"), Some(remote(1))).err().map(|error| error.status()), Some(StatusCode::TOO_MANY_REQUESTS));
    let _second = state.open_socket(&forwarded("203.0.113.8"), Some(remote(1))).expect("another address");
    assert_eq!(state.open_socket(&forwarded("203.0.113.9"), Some(remote(1))).err().map(|error| error.status()), Some(StatusCode::SERVICE_UNAVAILABLE));
    drop(first);
    assert!(state.open_socket(&forwarded("203.0.113.7"), Some(remote(1))).is_ok());
    assert_eq!(PresenceSettings::default().inbound_bytes(), 4096, "the presence socket reads no message past twice its largest state");
}

#[tokio::test]
async fn a_module_refuses_a_malformed_target_before_the_bus_places_anything() {
    let (server, router) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let mut malformed = envelope();
    malformed.target = actor("no such id");
    malformed.idempotency_key = Some(crate::contract::IdempotencyKey("k1".into()));
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[JSON], serde_json::to_vec(&malformed).unwrap())).await;
    assert_eq!((status, body["status"].as_str(), &body["reason"]), (StatusCode::OK, Some("rejected"), &serde_json::json!({ "kind": "invalid", "detail": "target-malformed" })), "{body}");
    assert_eq!(server.state().authority.lock().await.directory().placed(), 0);
}
//#endregion 🔖️Edge

