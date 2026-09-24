"""📌️ One-off: the hub's in-process Check In laws and their stdio JSON ledger fixture."""
p = "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:120], s.count(old))
    s = s.replace(old, new)

helpers = r'''
/// 📌️ One stdio JSON document on a hub with the linked native codec: a genesis checkpoint, an
/// author, a spectator, and the document's live actor.
#[cfg(feature = "native-artifact-execution")]
struct CheckInFixture {
    state: HubState,
    author: TestIssuedSession,
    spectator: TestIssuedSession,
    author_email: String,
    scope: DocumentScope,
    handle: db::ArtifactHandle,
    genesis: os_directory::ArtifactCheckpoint,
    pack: Vec<u8>,
    spr: Vec<u8>,
    catalog_root: std::path::PathBuf,
}

#[cfg(feature = "native-artifact-execution")]
async fn check_in_fixture(label: &str) -> CheckInFixture {
    let catalog_root = native_openable_stdio_bundle();
    let providers = NativeCodecProviderSetV1::linked();
    let configured = configured_artifact_authority(&catalog_root, Some(&providers), &Tracer::disabled()).await.expect("load stdio check-in catalog").configured().expect("configured check-in catalog");
    let selection = configured.catalog.selected_document_open().expect("selected stdio JSON target").clone();
    let mut state = test_state().await;
    let author_email = format!("check-in-{label}-author@example.test");
    let author = issue_test_session(&state, &author_email).await;
    let spectator = issue_test_session(&state, &format!("check-in-{label}-spectator@example.test")).await;
    let space_id = create_space_for_test(&state, &author.user_id, &format!("Check In {label}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, &author_email, DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, &format!("check-in-{label}-spectator@example.test"), DirectorySpaceRole::Spectator).await;
    let request_id = os_directory::hex_lower(&Sha256::digest(format!("check-in-{label}").as_bytes()))[..32].to_string();
    let scope = DocumentScope::new(space_id, format!("artifact-{request_id}"));
    let mut descriptor = DocumentDescriptor {
        space_id: scope.space_id.clone(),
        document_id: scope.document_id.clone(),
        artifact_kind: selection.artifact.kind,
        artifact_schema: selection.artifact.schema,
        owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id, package_id: selection.package.package_id, version: selection.package.version, package_hash: selection.package.component_sha256 },
        pack_schema_hash: selection.artifact.pack_schema_hash,
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: String::new(),
    };
    state.artifact_authority = Some(configured.authority);
    let catalog_generation = configured.catalog.generation_id().to_string();
    let parent_dialect = directory::os_io::ArtifactDialect { artifact_kind: selection.parent_dialect.artifact_kind, standard: selection.parent_dialect.standard, subset: selection.parent_dialect.subset };
    state.openable_catalog = Some(configured.catalog);
    let document = db_artifact_id(&scope);
    let pack = <semio_s_artifact_stdio_json::JsonSnapshot as directory::os_store::ArtifactPack>::encode_pack(&semio_s_artifact_stdio_json::schema::snapshot::demo_json_snapshot());
    let spr = directory::os_store::empty_document_spr(&document.0, &descriptor.artifact_schema).await;
    descriptor.bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(&pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(&author.token).expect("check-in author capability")).await.expect("check-in author session read").expect("check-in author session");
    let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: session.id, authorization_generation: session.authorization_generation };
    let genesis = publish_genesis_checkpoint_for_test(&state, actor, catalog_generation, parent_dialect, descriptor, &pack, &spr).await;
    let handle = state.ensure_document(&document).await.expect("check-in document actor");
    CheckInFixture { state, author, spectator, author_email, scope, handle, genesis, pack, spr, catalog_root }
}

/// ✍️ The ledger a real stdio JSON editor emits for `values.len()` one-operation edits made on the
/// genesis pair: one envelope per edit, exactly its store's own event log, addressed to the hub's
/// document key.
#[cfg(feature = "native-artifact-execution")]
async fn check_in_json_edits(fixture: &CheckInFixture, values: &[&str]) -> Vec<MutationEnvelope> {
    use directory::os_store::{ArtifactCommand, ArtifactStore, SnapshotRetirementStep, SpaceMember};
    use semio_s_artifact_stdio_json::schema::mutations::{JsonMutation, SetMemberMutation, SetMemberPayload};
    use semio_s_artifact_stdio_json::schema::snapshot::JsonValue;
    use semio_s_artifact_stdio_json::JsonSnapshot;
    let parsed = directory::os_store::parse_document_pack::<JsonSnapshot, JsonMutation>(&fixture.pack, &fixture.spr).await.expect("genesis pair parses");
    let mut store = ArtifactStore::<JsonSnapshot, JsonMutation>::new(parsed.into_envelope()).await.expect("editor store");
    store.install_document_store_owners_exact(directory::os_store::bounded_artifact_store_owners());
    let mut applied = Ok(());
    for value in values {
        let mutation = JsonMutation::SetMember(SetMemberMutation::Apply(SetMemberPayload { path: Vec::new(), key: "count".into(), value: JsonValue::Number { lexeme: (*value).into() } }));
        if let Err(error) = store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], description: None }).await {
            applied = Err(error);
            break;
        }
    }
    let events = store.event_log();
    loop {
        match store.close_owned_step(1, directory::os_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("editor store closes") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { .. } => {}
            SnapshotRetirementStep::Blocked => panic!("editor store close blocked"),
        }
    }
    drop(store);
    applied.expect("editor edits apply");
    let document = WireArtifactId(db_artifact_id(&fixture.scope).0);
    events.expect("editor ledger").into_iter().map(|mut envelope| {
        envelope.document_id = document.clone();
        envelope
    }).collect()
}

/// 📥️ Commits one envelope as its own ledger transaction and names the head it produced.
#[cfg(feature = "native-artifact-execution")]
async fn check_in_commit(fixture: &CheckInFixture, envelope: MutationEnvelope) -> EditedArtifactFrontierV1 {
    let batch = db::document::CommandBatch::new(vec![envelope]).await.expect("check-in command batch");
    fixture.handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("check-in actor response").expect("check-in edit accepted");
    let snapshot = fixture.handle.checkpoint_publication_snapshot().await.expect("check-in actor snapshot");
    EditedArtifactFrontierV1::of_artifact_frontier(&ledger_artifact_frontier(&fixture.scope, &snapshot).expect("an edited ledger point")).expect("edited wire frontier")
}

/// 🧮️ The pair a remote replica holds after folding `envelopes` onto the genesis pair — the oracle a
/// hub-materialized checkpoint must equal byte for byte.
#[cfg(feature = "native-artifact-execution")]
async fn check_in_replica_pair(fixture: &CheckInFixture, envelopes: &[MutationEnvelope]) -> (Vec<u8>, Vec<u8>) {
    use semio_s_artifact_stdio_json::schema::mutations::JsonMutation;
    use semio_s_artifact_stdio_json::JsonSnapshot;
    let files = directory::os_store::replay_envelopes_onto_pair::<JsonSnapshot, JsonMutation>(&fixture.pack, &fixture.spr, &directory::os_spr::encode_envelopes(envelopes), directory::os_store::bounded_artifact_store_owners).await.expect("replica fold");
    (files.pack, files.spr)
}

/// 📮️ Posts one Check In and follows it to a terminal status.
#[cfg(feature = "native-artifact-execution")]
async fn check_in_until_terminal(addr: SocketAddr, fixture: &CheckInFixture, token: &str, request_id: &str, head: &EditedArtifactFrontierV1) -> (u16, Option<DocumentCheckInStatusV1>) {
    let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
    let request = DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head: head.clone() }.canonical_json().expect("canonical check-in");
    let authorization = format!("Bearer {token}");
    let posted = raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], request.as_bytes()).await;
    if posted.status != 200 && posted.status != 202 {
        return (posted.status, None);
    }
    let mut status = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&posted.body).expect("status UTF-8")).expect("canonical status");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !status.phase.is_terminal() {
        assert!(std::time::Instant::now() < deadline, "check-in {request_id} never reached a terminal status: {status:?}");
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let polled = raw_http_request(addr, "GET", &format!("{route}/{request_id}"), &[("Authorization", authorization.as_str())], &[]).await;
        assert!(polled.status == 200 || polled.status == 202, "check-in poll answered {}", polled.status);
        let next = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&polled.body).expect("status UTF-8")).expect("canonical status");
        assert!(next.progress.completed_units >= status.progress.completed_units, "check-in progress went back: {status:?} -> {next:?}");
        status = next;
    }
    (posted.status, Some(status))
}

/// 🧾️ The hub's active checkpoint pair as a cold opener reads it.
#[cfg(feature = "native-artifact-execution")]
async fn check_in_cold_pair(addr: SocketAddr, fixture: &CheckInFixture) -> semio_hub::lag_rebootstrap::VerifiedActiveCheckpointPair {
    let authorization = format!("Bearer {}", fixture.author.token);
    let response = raw_http_request(addr, "GET", &format!("/spaces/{}/documents/{}/active-checkpoint/pair", fixture.scope.space_id, fixture.scope.document_id), &[("Authorization", authorization.as_str()), ("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE)], &[]).await;
    assert_eq!(response.status, 200, "cold open answered {}", String::from_utf8_lossy(&response.body));
    decode_canonical_checkpoint_pair(&response.body).expect("canonical cold pair")
}
'''
anchor = '''#[cfg(all(feature = "sqlite", feature = "integration-fixtures"))]
#[tokio::test]
async fn check_in_process_fixture_emits_verified_gis_ledger_and_catalog() {'''
rep(anchor, helpers.lstrip("\n") + "\n" + anchor)

laws = r'''
    /// 📌️ Check In folds the hub's own ledger onto the active checkpoint and publishes it: after two
    /// edits the active checkpoint's baseline is exactly the named head, its parent is genesis, and a
    /// cold open reads the pair a remote replica holds after the same ledger, byte for byte. A third
    /// edit and a second Check In advance it again from the first checkpoint.
    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it() {
        let fixture = check_in_fixture("advances").await;
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_json_edits(&fixture, &["43", "44", "45"]).await;
        assert_eq!(ledger.len(), 3, "one envelope per one-operation edit");
        check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;

        let (code, status) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "11111111111111111111111111111111", &second).await;
        let status = status.unwrap_or_else(|| panic!("check-in refused with {code}"));
        assert_eq!(code, 202, "a fresh check-in answers accepted");
        assert_eq!(status.phase, DocumentCheckInPhaseV1::Ready, "check-in ended {status:?}");
        let ready = status.ready.clone().expect("ready names its checkpoint");
        assert_eq!(ready.baseline, second);
        assert_eq!(ready.parent_checkpoint_id, fixture.genesis.checkpoint_id.hex());
        let active = fixture.state.directory.get_active_artifact_checkpoint(&fixture.scope).await.expect("active read").expect("active checkpoint");
        assert_eq!(active.checkpoint_id.hex(), ready.checkpoint_id);
        assert_eq!(EditedArtifactFrontierV1::of_artifact_frontier(&active.baseline_frontier), Some(second.clone()), "the head advanced to the named head");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(cold.selection.active_checkpoint_id.hex(), ready.checkpoint_id, "a cold open starts from the new checkpoint");
        let expected = check_in_replica_pair(&fixture, &ledger[..2]).await;
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), expected, "the checkpoint is the replica fold of the ledger, byte for byte");

        let third = check_in_commit(&fixture, ledger[2].clone()).await;
        let (_, next) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "22222222222222222222222222222222", &third).await;
        let next = next.expect("second check-in status");
        let next_ready = next.ready.clone().unwrap_or_else(|| panic!("second check-in ended {next:?}"));
        assert_eq!(next_ready.parent_checkpoint_id, ready.checkpoint_id, "the second checkpoint extends the first");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(cold.selection.active_checkpoint_id.hex(), next_ready.checkpoint_id);
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), check_in_replica_pair(&fixture, &ledger).await);
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 3, "genesis plus exactly two check-ins");
        std::fs::remove_dir_all(fixture.catalog_root).expect("remove check-in catalog fixture");
    }

    /// ⛔️ Only an author checks in, only a committed head of this document is materialized, a head the
    /// active checkpoint already passed is stale, and checking in the active baseline again is the
    /// same checkpoint rather than a second one.
    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn check_in_refuses_stale_unknown_and_foreign_heads_and_is_author_owned() {
        let fixture = check_in_fixture("refusals").await;
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_json_edits(&fixture, &["43", "44"]).await;
        let first = check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;
        let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
        let request = |request_id: &str, head: &EditedArtifactFrontierV1| DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head: head.clone() }.canonical_json().expect("canonical check-in");
        let body = request("33333333333333333333333333333333", &second);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], body.as_bytes()).await.status, 401, "no credential");
        let spectator = format!("Bearer {}", fixture.spectator.token);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", spectator.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await.status, 403, "a spectator never writes a checkpoint");
        let author = format!("Bearer {}", fixture.author.token);
        let mut foreign = second.clone();
        foreign.document_id = "artifact-other".into();
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], request("33333333333333333333333333333333", &foreign).as_bytes()).await.status, 400, "a head of another document");
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], format!(" {body}").as_bytes()).await.status, 400, "a noncanonical body");

        let mut forged = second.clone();
        forged.chain_sha256 = format!("{}{}", if forged.chain_sha256.starts_with('a') { "b" } else { "a" }, &forged.chain_sha256[1..]);
        let (_, unknown) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "44444444444444444444444444444444", &forged).await;
        assert_eq!(unknown.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::UnknownHead))), "a forged chain is not a ledger point");
        let mut beyond = second.clone();
        beyond.head_edit_ordinal += 1;
        beyond.last_commit_seq += 1;
        let (_, unknown) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "55555555555555555555555555555555", &beyond).await;
        assert_eq!(unknown.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::UnknownHead))), "a head the ledger never reached");

        let (_, ready) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "66666666666666666666666666666666", &second).await;
        let ready = ready.and_then(|status| status.ready).expect("the head checks in");
        let (_, stale) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "77777777777777777777777777777777", &first).await;
        assert_eq!(stale.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::StaleHead))), "a head the active checkpoint passed is stale");
        let (_, again) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "88888888888888888888888888888888", &second).await;
        assert_eq!(again.and_then(|status| status.ready), Some(ready), "checking in the active baseline answers the same checkpoint");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 2, "refusals and the repeat publish nothing");
        std::fs::remove_dir_all(fixture.catalog_root).expect("remove check-in catalog fixture");
    }

    /// 🔁️ A retried request answers its own job, the same request id naming another head conflicts,
    /// a Check In the author cancels before publication publishes nothing, and an author removed
    /// from the space mid-flight ends it `authority-changed` with nothing published.
    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn check_in_is_idempotent_per_request_cancellable_and_revoked_with_the_author() {
        let mut fixture = check_in_fixture("lifecycle").await;
        let gate = Arc::new(TestLiveGate::default());
        fixture.state.live_gate = Some(gate.clone());
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_json_edits(&fixture, &["43", "44"]).await;
        let first = check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;
        let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
        let author = format!("Bearer {}", fixture.author.token);
        let post = |request_id: &'static str, head: EditedArtifactFrontierV1| {
            let route = route.clone();
            let author = author.clone();
            async move {
                let body = DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head }.canonical_json().expect("canonical check-in");
                raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await
            }
        };

        gate.check_in_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let accepted = post("99999999999999999999999999999999", second.clone()).await;
        assert_eq!(accepted.status, 202);
        tokio::time::timeout(std::time::Duration::from_secs(10), gate.check_in_admitted.acquire()).await.expect("check-in reaches its publication gate").expect("gate admission").forget();
        let joined = post("99999999999999999999999999999999", second.clone()).await;
        assert_eq!(joined.status, 202, "a retry joins the running job");
        let joined = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&joined.body).unwrap()).expect("joined status");
        assert_eq!(joined.phase, DocumentCheckInPhaseV1::Materializing);
        assert_eq!(joined.progress.completed_units, 5, "materialized, not yet published");
        assert_eq!(post("99999999999999999999999999999999", first.clone()).await.status, 409, "one request id names one request");
        let cancelled = raw_http_request(addr, "POST", &format!("{route}/99999999999999999999999999999999/cancel"), &[("Authorization", author.as_str())], &[]).await;
        assert!(cancelled.status == 200 || cancelled.status == 202);
        gate.check_in_release.add_permits(1);
        let (_, status) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "99999999999999999999999999999999", &second).await;
        assert_eq!(status.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Cancelled, None)));
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1, "a cancelled check-in publishes nothing");

        assert_eq!(post("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", second.clone()).await.status, 202);
        tokio::time::timeout(std::time::Duration::from_secs(10), gate.check_in_admitted.acquire()).await.expect("second check-in reaches its gate").expect("gate admission").forget();
        execute_directory_command_fenced(&fixture.state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:check-in-removal".into() }, DirectoryCommand::RemoveMember { space_id: fixture.scope.space_id.clone(), user_id: fixture.author.user_id.clone() }).await.expect("remove the author");
        gate.check_in_release.add_permits(1);
        let key = DocumentCheckInKey { user_id: fixture.author.user_id.clone(), space_id: fixture.scope.space_id.clone(), document_id: fixture.scope.document_id.clone(), request_id: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into() };
        let job = fixture.state.check_ins.get(&key).expect("the revoked job is retained");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !job.is_terminal() {
            assert!(std::time::Instant::now() < deadline, "the revoked check-in never ended");
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert_eq!((job.status().phase, job.status().refusal), (DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged)));
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1, "a revoked author publishes nothing");
        assert_eq!(raw_http_request(addr, "GET", &format!("{route}/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), &[("Authorization", author.as_str())], &[]).await.status, 403, "the removed member lost read access to the job too");
        let _ = &fixture.author_email;
        std::fs::remove_dir_all(fixture.catalog_root).expect("remove check-in catalog fixture");
    }
'''
anchor = "    async fn native_openable_stdio_provider_is_the_only_atomic_readiness_transition() {"
i = s.index(anchor)
j = s.index("\n    }\n", i) + len("\n    }\n")
s = s[:j] + laws + s[j:]
open(p, "w", encoding="utf-8").write(s)
print("ok")
