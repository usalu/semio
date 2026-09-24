"""Moves the Check In laws from the synthetic stdio JSON viewer (no longer openable under the one open-target rule) onto the verified GIS Map profile."""
import re
p = "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
s = open(p, encoding="utf-8").read()

def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)

start = s.index("#[cfg(feature = \"native-artifact-execution\")]\nstruct CheckInFixture {")
end = s.index("/// 📮️ Posts one Check In and follows it to a terminal status.")
block = s[start:end]
new_block = '''#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]
struct CheckInFixture {
    state: HubState,
    author: TestIssuedSession,
    spectator: TestIssuedSession,
    scope: DocumentScope,
    handle: db::ArtifactHandle,
    genesis: os_directory::ArtifactCheckpoint,
    pack: Vec<u8>,
    spr: Vec<u8>,
    _profile: semio_hub::artifact_authority::trusted_catalog::trusted_catalog_fixture::VerifiedGisMapIntegrationProfileV1,
}

/// 🗺️ One GIS Map document on the verified GIS profile (the kind a hub opens and checks in), its
/// genesis checkpoint published, one Author, one Spectator and a separate owner.
#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]
async fn check_in_fixture(label: &str) -> CheckInFixture {
    use semio_hub::artifact_authority::trusted_catalog::trusted_catalog_fixture;
    let profile = trusted_catalog_fixture::verified_gis_map_integration_profile(&trusted_catalog_fixture::unique_profile_root(&format!("check-in-{label}"))).await.expect("verified GIS Map check-in profile");
    let selection = profile.binding().selection().clone();
    let mut state = test_state().await;
    let author_email = format!("check-in-{label}-author@example.test");
    let author = issue_test_session(&state, &author_email).await;
    let spectator = issue_test_session(&state, &format!("check-in-{label}-spectator@example.test")).await;
    let owner = issue_test_session(&state, &format!("check-in-{label}-owner@example.test")).await;
    let space_id = create_space_for_test(&state, &owner.user_id, &format!("Check In {label}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, &author_email, DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, &format!("check-in-{label}-spectator@example.test"), DirectorySpaceRole::Spectator).await;
    let request_id = os_directory::hex_lower(&Sha256::digest(format!("check-in-{label}").as_bytes()))[..32].to_string();
    let scope = DocumentScope::new(space_id, format!("artifact-{request_id}"));
    let mut descriptor = DocumentDescriptor {
        space_id: scope.space_id.clone(),
        document_id: scope.document_id.clone(),
        artifact_kind: selection.artifact.kind.clone(),
        artifact_schema: selection.artifact.schema.clone(),
        owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id.clone(), package_id: selection.package.package_id.clone(), version: selection.package.version.clone(), package_hash: selection.package.component_sha256.clone() },
        pack_schema_hash: selection.artifact.pack_schema_hash.clone(),
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: String::new(),
    };
    state.artifact_authority = Some(Arc::new(ValidatingCanonicalArtifactAuthority::new(profile.catalog().clone())));
    state.verified_catalog = Some(profile.catalog().clone());
    state.openable_catalog = Some(profile.catalog().clone());
    let document = db_artifact_id(&scope);
    let pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::os_store::ArtifactPack>::encode_pack(&semio_s_artifact_gis_gismap::GisMapSnapshot::default());
    let spr = directory::os_store::empty_document_spr(&document.0, &descriptor.artifact_schema).await;
    descriptor.bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(&pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(&author.token).expect("check-in author capability")).await.expect("check-in author session read").expect("check-in author session");
    let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: session.id, authorization_generation: session.authorization_generation };
    let genesis = publish_genesis_checkpoint_for_test(&state, actor, profile.catalog().generation_id().to_string(), selection.parent_dialect.clone(), descriptor, &pack, &spr).await;
    let handle = state.ensure_document(&document).await.expect("check-in document actor");
    CheckInFixture { state, author, spectator, scope, handle, genesis, pack, spr, _profile: profile }
}

/// ✍️ The ledger a real GIS Map editor emits for `ids.len()` one-operation edits made on the genesis
/// pair — each places one position — as one envelope per edit, exactly its store's own event log,
/// addressed to the hub's document key.
#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]
async fn check_in_map_edits(fixture: &CheckInFixture, ids: &[&str]) -> Vec<MutationEnvelope> {
    use directory::os_store::{ArtifactCommand, ArtifactStore, SnapshotRetirementStep};
    use semio_s_artifact_gis_gismap::mutations::create_position::CreatePosition;
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot, MapFeature};
    let parsed = directory::os_store::parse_document_pack::<GisMapSnapshot, GisMapMutation>(&fixture.pack, &fixture.spr).await.expect("genesis pair parses");
    let mut store = ArtifactStore::<GisMapSnapshot, GisMapMutation>::new(parsed.into_envelope()).await.expect("editor store");
    store.install_document_store_owners_exact(directory::os_store::bounded_artifact_store_owners());
    let mut applied = Ok(());
    for (index, id) in ids.iter().enumerate() {
        let point = directory::DslValue::object([("lon".into(), directory::DslValue::float(7.0 + index as f64)), ("lat".into(), directory::DslValue::float(47.0))]);
        let mutation = GisMapMutation::CreatePosition(CreatePosition { index, item: MapFeature { id: format!("check-in-{id}"), data: point } });
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
#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]
async fn check_in_commit(fixture: &CheckInFixture, envelope: MutationEnvelope) -> EditedArtifactFrontierV1 {
    let batch = db::document::CommandBatch::new(vec![envelope]).await.expect("check-in command batch");
    fixture.handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("check-in actor response").expect("check-in edit accepted");
    let snapshot = fixture.handle.checkpoint_publication_snapshot().await.expect("check-in actor snapshot");
    EditedArtifactFrontierV1::of_artifact_frontier(&ledger_artifact_frontier(&fixture.scope, &snapshot).expect("an edited ledger point")).expect("edited wire frontier")
}

/// 🧮️ The pair a remote replica holds after folding `envelopes` onto the genesis pair — the oracle a
/// hub-materialized checkpoint must equal byte for byte.
#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]
async fn check_in_replica_pair(fixture: &CheckInFixture, envelopes: &[MutationEnvelope]) -> (Vec<u8>, Vec<u8>) {
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot};
    let files = directory::os_store::replay_envelopes_onto_pair::<GisMapSnapshot, GisMapMutation>(&fixture.pack, &fixture.spr, &directory::os_spr::encode_envelopes(envelopes), directory::os_store::bounded_artifact_store_owners).await.expect("replica fold");
    (files.pack, files.spr)
}

'''
s = s[:start] + new_block + s[end:]
# the remaining helpers and three laws: widen their cfg
for name in ["async fn check_in_until_terminal(", "async fn check_in_cold_pair("]:
    i = s.index(name)
    head = s.rfind('#[cfg(feature = "native-artifact-execution")]', 0, i)
    assert i - head < 200, name
    s = s[:head] + '#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]' + s[head + len('#[cfg(feature = "native-artifact-execution")]'):]
for law in ["async fn check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it()", "async fn check_in_refuses_stale_unknown_and_foreign_heads_and_is_author_owned()", "async fn check_in_is_idempotent_per_request_cancellable_and_revoked_with_the_author()"]:
    i = s.index(law)
    head = s.rfind('#[cfg(feature = "native-artifact-execution")]', 0, i)
    assert i - head < 120, law
    s = s[:head] + '#[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]' + s[head + len('#[cfg(feature = "native-artifact-execution")]'):]
n = s.count("check_in_json_edits(")
s = s.replace("check_in_json_edits(", "check_in_map_edits(")
print("edits renamed", n)
n = s.count('        std::fs::remove_dir_all(fixture.catalog_root).expect("remove check-in catalog fixture");\n')
s = s.replace('        std::fs::remove_dir_all(fixture.catalog_root).expect("remove check-in catalog fixture");\n', "")
print("catalog removals", n)
open(p, "w", encoding="utf-8").write(s)
