#!/usr/bin/env python3
"""📌 H14 14c P1 (B1): the hub checks a document in by itself once the edits (or diff/inverse payload bytes) committed over
its sockets since it last did reach the declared policy (`LocalBootstrapReadinessV1.features.checkpointPolicy`, configured
by `OS_HUB_CHECKPOINT_POLICY_EDITS` / `OS_HUB_CHECKPOINT_POLICY_BYTES`), on behalf of the author whose batch reached it —
the ordinary event-sourced Check In (claim, fold, fenced publication). With (B2) a fresh client's tail is then bounded by
the policy instead of the document's history. Schema, pipe fixture, README, hub bootstrap, bin laws. Idempotent;
`--dry-run` reports only."""
import sys

R = "/Users/ueli/Documents/semio"
BOOT = f"{R}/🌎️hub/🏗️bootstrap/🦀️.rs"
TESTS = f"{R}/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
SCHEMA = f"{R}/🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json"
FIXTURE = f"{R}/🌎️hub/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json"
README = f"{R}/🌎️hub/README.md"
DRY = "--dry-run" in sys.argv
files = {path: open(path, encoding="utf-8").read() for path in (BOOT, TESTS, SCHEMA, FIXTURE, README)}
problems, states = [], []


def edit(path, old, new, label, count=1):
    text = files[path]
    if new in text and (count != 1 or old not in text):
        states.append("done")
        return
    if text.count(old) != count:
        problems.append(f"{label}: expected {count}, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


# schema: features.checkpointPolicy (required)
edit(SCHEMA, '''"openPlan",
            "openPlanExchange",
            "rebootstrap",
            "mcpWorkspace",
            "inferenceServices"
          ],''', '''"openPlan",
            "openPlanExchange",
            "rebootstrap",
            "mcpWorkspace",
            "inferenceServices",
            "checkpointPolicy"
          ],''', "schema required")

# fixture: every readiness features object carries the declared default policy
old_features = '"inferenceServices": [] }'
new_features = '"inferenceServices": [], "checkpointPolicy": { "edits": 1024, "payloadBytes": 524288 } }'
count = files[FIXTURE].count(old_features)
edit(FIXTURE, old_features, new_features, "fixture features", count=count if count else 1)


edit(SCHEMA, """            "inferenceServices": {
              "type": "array",
              "maxItems": 16,""", """            "checkpointPolicy": {
              "type": "object",
              "additionalProperties": false,
              "required": [
                "edits",
                "payloadBytes"
              ],
              "description": "📌️ When the hub checks a document in by itself: once the edits, or the diff and inverse payload bytes, committed over its document sockets since it last did reach either bound, on behalf of the author whose batch reached it. A client that seeds from the checkpoint then receives only the tail since it (`OS_HUB_CHECKPOINT_POLICY_EDITS`, `OS_HUB_CHECKPOINT_POLICY_BYTES`).",
              "properties": {
                "edits": {
                  "type": "integer",
                  "minimum": 1,
                  "maximum": 1048576
                },
                "payloadBytes": {
                  "type": "integer",
                  "minimum": 4096,
                  "maximum": 1073741824
                }
              }
            },
            "inferenceServices": {
              "type": "array",
              "maxItems": 16,""", "schema property")

edit(README, """| `OS_HUB_MERGE_POLICY` |""", """| `OS_HUB_CHECKPOINT_POLICY_EDITS` | `1024` | Edits committed over a document's sockets after which the hub checks the document in by itself, on behalf of the author whose batch reached the bound (1 … 1048576; anything else fails boot). A client that seeds from the active checkpoint receives only the tail since it, so this bounds what opening a document replays. Declared at `/readyz` → `features.checkpointPolicy`. |
| `OS_HUB_CHECKPOINT_POLICY_BYTES` | `524288` (512 KiB) | Diff and inverse payload bytes committed after which the hub checks a document in by itself (4096 … 1073741824; anything else fails boot) — whichever of the two bounds a document reaches first. |
| `OS_HUB_MERGE_POLICY` |""", "README env rows")

edit(BOOT, """#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubFeatureReadinessV1 {
    open_plan: bool,
    open_plan_exchange: bool,
    rebootstrap: bool,
    mcp_workspace: bool,
    inference_services: Vec<HubInferenceServiceReadinessV1>,
}
""", """#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubFeatureReadinessV1 {
    open_plan: bool,
    open_plan_exchange: bool,
    rebootstrap: bool,
    mcp_workspace: bool,
    inference_services: Vec<HubInferenceServiceReadinessV1>,
    checkpoint_policy: HubCheckpointPolicyV1,
}

/// @emoji 📌️ `features.checkpointPolicy` (`LocalBootstrapReadinessV1`): the hub checks a document in by itself once the edits,
/// or the diff and inverse payload bytes, committed over its document sockets since it last did reach either bound — on behalf
/// of the author whose batch reached it, through the ordinary Check In. A client that seeds from the active checkpoint then
/// replays at most about one policy's worth of tail when it opens the document (ticket 26/09/23 session 14c: a 13 201-edit
/// document without a checkpoint could no longer be opened).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubCheckpointPolicyV1 {
    edits: u64,
    payload_bytes: u64,
}

/// 📌️ The declared default policy: 1 024 edits or 512 KiB of payload, well inside a browser client's pre-activation tail
/// retention (64 frames, 1 MiB).
const HUB_CHECKPOINT_POLICY: HubCheckpointPolicyV1 = HubCheckpointPolicyV1 { edits: 1_024, payload_bytes: 512 * 1024 };
const CHECKPOINT_POLICY_EDITS_ENV: &str = "OS_HUB_CHECKPOINT_POLICY_EDITS";
const CHECKPOINT_POLICY_BYTES_ENV: &str = "OS_HUB_CHECKPOINT_POLICY_BYTES";

impl HubCheckpointPolicyV1 {
    /// 🎚️ The operator's policy: each bound decimal within the schema's range, absent means the declared default, anything else
    /// fails boot.
    fn configured(edits: Option<&str>, payload_bytes: Option<&str>) -> Result<Self, String> {
        let bounded = |value: Option<&str>, name: &str, bounds: std::ops::RangeInclusive<u64>, default: u64| match value.map(str::trim).filter(|text| !text.is_empty()) {
            None => Ok(default),
            Some(text) => text.parse::<u64>().ok().filter(|bound| bounds.contains(bound)).ok_or_else(|| format!("{name} must be a decimal integer from {} to {}", bounds.start(), bounds.end())),
        };
        Ok(Self { edits: bounded(edits, CHECKPOINT_POLICY_EDITS_ENV, 1..=1_048_576, HUB_CHECKPOINT_POLICY.edits)?, payload_bytes: bounded(payload_bytes, CHECKPOINT_POLICY_BYTES_ENV, 4_096..=1_073_741_824, HUB_CHECKPOINT_POLICY.payload_bytes)? })
    }
}

/// @emoji 🧮️ What one written document committed since the hub last weighed its checkpoint policy, and whether a policy
/// Check In of it runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CheckpointPolicyTallyV1 {
    edits: u64,
    payload_bytes: u64,
    running: bool,
}

/// 🧮️ How many documents one hub tallies at once; past it the tallies of documents with no Check In running restart.
const CHECKPOINT_POLICY_DOCUMENTS_MAX: usize = 65_536;

/// @emoji 🧮️ The per-document tallies of the checkpoint policy.
#[derive(Default)]
struct CheckpointPolicyLedgerV1 {
    documents: Mutex<std::collections::HashMap<DocumentScope, CheckpointPolicyTallyV1>>,
}

impl CheckpointPolicyLedgerV1 {
    /// ➕️ Adds one committed batch to `scope`'s tally: `true` when the tally reached `policy` and no policy Check In of the
    /// document runs — the caller starts one, the tally restarts and stays running until [`Self::finish`].
    fn observe(&self, scope: &DocumentScope, edits: u64, payload_bytes: u64, policy: HubCheckpointPolicyV1) -> bool {
        let mut documents = self.documents.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !documents.contains_key(scope) && documents.len() >= CHECKPOINT_POLICY_DOCUMENTS_MAX {
            documents.retain(|_, tally| tally.running);
            if documents.len() >= CHECKPOINT_POLICY_DOCUMENTS_MAX {
                return false;
            }
        }
        let tally = documents.entry(scope.clone()).or_default();
        tally.edits = tally.edits.saturating_add(edits);
        tally.payload_bytes = tally.payload_bytes.saturating_add(payload_bytes);
        if tally.running || (tally.edits < policy.edits && tally.payload_bytes < policy.payload_bytes) {
            return false;
        }
        *tally = CheckpointPolicyTallyV1 { edits: 0, payload_bytes: 0, running: true };
        true
    }

    /// 🏁️ Ends `scope`'s policy Check In; the edits committed meanwhile stay tallied.
    fn finish(&self, scope: &DocumentScope) {
        if let Some(tally) = self.documents.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get_mut(scope) {
            tally.running = false;
        }
    }

    #[cfg(test)]
    fn tally(&self, scope: &DocumentScope) -> Option<CheckpointPolicyTallyV1> {
        self.documents.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(scope).copied()
    }
}
""", "policy types")

edit(BOOT, """            mcp_workspace: mcp_workspace_ready(agent_delegation_ready, open_plan_ready),
            inference_services,
        },""", """            mcp_workspace: mcp_workspace_ready(agent_delegation_ready, open_plan_ready),
            inference_services,
            checkpoint_policy: HUB_CHECKPOINT_POLICY,
        },""", "readiness default")

edit(BOOT, """    merge_policy: protocol::MergePolicy,
    /// @emoji 🛣️ Every matched route's answers""", """    merge_policy: protocol::MergePolicy,
    /// @emoji 📌️ The checkpoint policy's per-document tallies ([`HubCheckpointPolicyV1`], declared in `readiness`).
    checkpoint_policy: Arc<CheckpointPolicyLedgerV1>,
    /// @emoji 🛣️ Every matched route's answers""", "state field")

edit(BOOT, """            check_ins: check_ins.clone(),
            directory_service,""", """            check_ins: check_ins.clone(),
            checkpoint_policy: Arc::default(),
            directory_service,""", "state init")

edit(BOOT, """    let guest_residency = TrustedCatalogGuestResidencyV1::configured(std::env::var(GUEST_RESIDENCY_BYTES_ENV).ok().as_deref()).map_err(|detail| HubError::ArtifactAuthority(AuthorityError::Catalog(detail)))?;""", """    let guest_residency = TrustedCatalogGuestResidencyV1::configured(std::env::var(GUEST_RESIDENCY_BYTES_ENV).ok().as_deref()).map_err(|detail| HubError::ArtifactAuthority(AuthorityError::Catalog(detail)))?;
    let checkpoint_policy = HubCheckpointPolicyV1::configured(std::env::var(CHECKPOINT_POLICY_EDITS_ENV).ok().as_deref(), std::env::var(CHECKPOINT_POLICY_BYTES_ENV).ok().as_deref()).map_err(|detail| HubError::ArtifactAuthority(AuthorityError::Catalog(detail)))?;""", "env parse")

edit(BOOT, """        let readiness = Arc::new(declare_public_session_issuance(
            hub_readiness(mode, bind_scope, run_id, bootstrap_ready, artifact_authority_ready, open_plan_ready, agent_delegation_ready, admin_dir.is_dir(), true, artifact_cas_sweep_execute, inference_ready, artifact_authority_reason),
            credential_sign_in.is_enabled(),
        ));""", """        let readiness = Arc::new(declare_public_session_issuance(
            with_checkpoint_policy(
                hub_readiness(mode, bind_scope, run_id, bootstrap_ready, artifact_authority_ready, open_plan_ready, agent_delegation_ready, admin_dir.is_dir(), true, artifact_cas_sweep_execute, inference_ready, artifact_authority_reason),
                checkpoint_policy,
            ),
            credential_sign_in.is_enabled(),
        ));""", "readiness wrap")

edit(BOOT, """fn declare_public_session_issuance(readiness: HubReadinessV1, credential_sign_in_enabled: bool) -> HubReadinessV1 {""", """/// @emoji 📌️ Declares the operator's checkpoint policy in `features.checkpointPolicy`.
fn with_checkpoint_policy(mut readiness: HubReadinessV1, policy: HubCheckpointPolicyV1) -> HubReadinessV1 {
    readiness.features.checkpoint_policy = policy;
    readiness
}

fn declare_public_session_issuance(readiness: HubReadinessV1, credential_sign_in_enabled: bool) -> HubReadinessV1 {""", "with_checkpoint_policy fn")

edit(BOOT, """/// @emoji 🧾️ Commits one admitted batch and answers it: the engine's receipt (or refusal) always reaches the socket as
/// the batch's `Ack` — never cut by the frame deadline, the engine's own bounds end the wait — and an advanced
/// frontier's relay reaches every peer. Returns `false` when the Ack could not be sent.
#[allow(clippy::too_many_arguments)]
async fn commit_admitted_commands(state: &HubState, handle: &db::ArtifactHandle, document_id: &str, fanout: &broadcast::Sender<ServerFrame>, actor: &ActorId, gate: &db::security::SecurityGate, admitted: AdmittedCommandsV1, sender: &mut SplitSink<WebSocket, Message>) -> bool {
    let AdmittedCommandsV1 { batch_id, envelopes, _document_write } = admitted;
    let (ack, relay) = submit_commands(handle, gate, actor, batch_id, envelopes, state.merge_policy).await;""", """/// @emoji 🧾️ Commits one admitted batch and answers it: the engine's receipt (or refusal) always reaches the socket as
/// the batch's `Ack` — never cut by the frame deadline, the engine's own bounds end the wait — and an advanced
/// frontier's relay reaches every peer; a committed batch is tallied for the checkpoint policy. Returns `false` when the
/// Ack could not be sent.
#[allow(clippy::too_many_arguments)]
async fn commit_admitted_commands(state: &HubState, handle: &db::ArtifactHandle, scope: &DocumentScope, subject: &SocketSubjectV1, fanout: &broadcast::Sender<ServerFrame>, actor: &ActorId, gate: &db::security::SecurityGate, admitted: AdmittedCommandsV1, sender: &mut SplitSink<WebSocket, Message>) -> bool {
    let document_id = scope.document_id.as_str();
    let AdmittedCommandsV1 { batch_id, envelopes, _document_write } = admitted;
    let (ack, relay) = submit_commands(handle, gate, actor, batch_id, envelopes, state.merge_policy).await;
    if let Some(ServerFrame::Commands { envelopes, .. }) = relay.as_ref() {
        observe_checkpoint_policy(state, scope, subject, handle, envelopes);
    }""", "commit tally")

edit(BOOT, """                                Ok(ClientFrameStepV1::Commit(admitted)) => {
                                    if !commit_admitted_commands(&state, &handle, &document_id, &fanout, &actor, &gate, admitted, sender).await {""", """                                Ok(ClientFrameStepV1::Commit(admitted)) => {
                                    if !commit_admitted_commands(&state, &handle, &DocumentScope::new(space_id.as_str(), document_id.as_str()), &socket_grant.subject, &fanout, &actor, &gate, admitted, sender).await {""", "socket call")

edit(BOOT, """async fn run_document_check_in(state: HubState, subject: SocketSubjectV1, scope: DocumentScope, request: DocumentCheckInV1, job: Arc<DocumentCheckInJob>, mut claim: CheckInClaimGuardV1) {""", """/// @emoji 📌️ Tallies one committed batch for the checkpoint policy and, when it reaches the policy, starts the policy Check In
/// of the document on behalf of `subject` in the background.
fn observe_checkpoint_policy(state: &HubState, scope: &DocumentScope, subject: &SocketSubjectV1, handle: &db::ArtifactHandle, envelopes: &[MutationEnvelope]) {
    let payload_bytes = envelopes.iter().map(|envelope| (envelope.diff.payload.len() + envelope.inverse.payload.len()) as u64).sum();
    if state.checkpoint_policy.observe(scope, envelopes.len() as u64, payload_bytes, state.readiness.features.checkpoint_policy) {
        tokio::spawn(run_policy_check_in(state.clone(), subject.clone(), scope.clone(), handle.clone()));
    }
}

/// @emoji 📌️ The checkpoint policy's Check In: `subject` — the author whose batch reached the policy, still allowed to check
/// in — checks `scope` in at its committed head through the ordinary claim, fold and fenced publication; answers the
/// job's terminal status (`None` when it could not start: no author session, no head, a claim of the same head exists).
async fn run_policy_check_in(state: HubState, subject: SocketSubjectV1, scope: DocumentScope, handle: db::ArtifactHandle) -> Option<DocumentCheckInStatusV1> {
    let status = start_policy_check_in(&state, subject, &scope, &handle).await;
    state.checkpoint_policy.finish(&scope);
    status
}

async fn start_policy_check_in(state: &HubState, subject: SocketSubjectV1, scope: &DocumentScope, handle: &db::ArtifactHandle) -> Option<DocumentCheckInStatusV1> {
    let SocketSubjectV1::Session { user_id, .. } = &subject else { return None };
    let user_id = user_id.clone();
    if state.artifact_authority.is_none() || !access_permits_in_space(state, &subject.access_roles(), HubAccessActionV1::DocumentCheckIn, &scope.space_id).await {
        return None;
    }
    let snapshot = handle.checkpoint_publication_snapshot().await.ok()?;
    let head = EditedArtifactFrontierV1::of_artifact_frontier(&ledger_artifact_frontier(scope, &snapshot)?)?;
    let request_id = os_directory::hex_lower(&Sha256::digest(format!("checkpoint-policy\\0{}\\0{}\\0{}", scope.space_id, scope.document_id, head.head_edit_ordinal).as_bytes()))[..32].to_string();
    let request = DocumentCheckInV1 { schema: os_directory::DOCUMENT_CHECK_IN_SCHEMA_V1.into(), request_id: request_id.clone(), head };
    let source = request.canonical_json()?;
    let command_sha256 = os_directory::hex_lower(&Sha256::digest(source.as_bytes()));
    let key = DocumentCheckInKey { user_id: user_id.clone(), space_id: scope.space_id.clone(), document_id: scope.document_id.clone(), request_id: request_id.clone() };
    let DocumentCheckInAdmission::Owner(job) = state.check_ins.admit(key.clone(), &command_sha256) else { return None };
    let claim = NewCheckpointPublicationClaimV1 { actor_user_id: user_id.clone(), correlation_id: request_id.clone(), command_sha256: command_sha256.clone(), claimed_at: now_ms() };
    if !matches!(state.directory_service.claim_or_read_checkpoint_publication(&claim).await, Ok(CheckpointPublicationClaimV1::Claimed(_))) {
        state.check_ins.forget(&key);
        return None;
    }
    let guard = CheckInClaimGuardV1 { service: state.directory_service.clone(), actor_user_id: user_id, correlation_id: request_id, command_sha256, complete: false };
    let policy = state.readiness.features.checkpoint_policy;
    let mut record = TraceRecord::new("server.document.check-in", TraceOutcome::Started);
    record.level = TraceLevel::Info;
    record.principal = Some(subject.trace_principal());
    record.space = Some(scope.space_id.clone());
    record.artifact = Some(scope.document_id.clone());
    record.detail = Some(format!("checkpoint-policy head={} edits={} payloadBytes={}", request.head.head_edit_ordinal, policy.edits, policy.payload_bytes));
    state.tracer.emit(record);
    run_document_check_in(state.clone(), subject, scope.clone(), request, job.clone(), guard).await;
    Some(job.status())
}

async fn run_document_check_in(state: HubState, subject: SocketSubjectV1, scope: DocumentScope, request: DocumentCheckInV1, job: Arc<DocumentCheckInJob>, mut claim: CheckInClaimGuardV1) {""", "policy runner")


edit(TESTS, """    /// 📌️ Check In folds the hub's own ledger onto the active checkpoint and publishes it: after two
    /// edits the active checkpoint's baseline is exactly the named head, its parent is genesis, and a""", """    /// 📌️ The checkpoint policy is the schema's: absent bounds are the declared default the readiness body carries, each
    /// bound is a decimal within its range, anything else fails boot.
    #[test]
    fn the_checkpoint_policy_is_declared_in_readiness_and_bounded_like_its_schema() {
        assert_eq!(HubCheckpointPolicyV1::configured(None, None), Ok(HUB_CHECKPOINT_POLICY));
        assert_eq!(HubCheckpointPolicyV1::configured(Some(" 1 "), Some("4096")), Ok(HubCheckpointPolicyV1 { edits: 1, payload_bytes: 4_096 }));
        assert_eq!(HubCheckpointPolicyV1::configured(Some("1048576"), Some("1073741824")), Ok(HubCheckpointPolicyV1 { edits: 1_048_576, payload_bytes: 1_073_741_824 }));
        for (edits, bytes) in [(Some("0"), None), (Some("1048577"), None), (Some("ten"), None), (None, Some("4095")), (None, Some("1073741825")), (None, Some("-1"))] {
            assert!(HubCheckpointPolicyV1::configured(edits, bytes).is_err(), "{edits:?} {bytes:?} must fail boot");
        }
        let readiness = with_checkpoint_policy(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"), HubCheckpointPolicyV1 { edits: 7, payload_bytes: 8_192 });
        let body = serde_json::to_value(&readiness).expect("readiness JSON");
        assert_eq!(body["features"]["checkpointPolicy"], serde_json::json!({ "edits": 7, "payloadBytes": 8192 }));
        let declared = serde_json::to_value(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root")).expect("readiness JSON");
        assert_eq!(declared["features"]["checkpointPolicy"], serde_json::json!({ "edits": 1024, "payloadBytes": 524288 }));
    }

    /// 🧮️ The policy's tally starts one Check In when a document reaches either bound, keeps tallying — but starts no second
    /// one — while it runs, and tallies documents apart.
    #[test]
    fn the_checkpoint_policy_starts_one_check_in_per_bound_and_tallies_while_it_runs() {
        let ledger = CheckpointPolicyLedgerV1::default();
        let policy = HubCheckpointPolicyV1 { edits: 4, payload_bytes: 1_000 };
        let (first, second) = (DocumentScope::new("space", "first"), DocumentScope::new("space", "second"));
        assert!(!ledger.observe(&first, 3, 10, policy));
        assert!(ledger.observe(&first, 1, 10, policy), "the fourth edit reaches the edit bound");
        assert_eq!(ledger.tally(&first), Some(CheckpointPolicyTallyV1 { edits: 0, payload_bytes: 0, running: true }));
        assert!(!ledger.observe(&first, 9, 5_000, policy), "no second policy Check In while one runs");
        assert!(ledger.observe(&second, 1, 1_000, policy), "the byte bound alone starts one, for another document");
        ledger.finish(&first);
        assert_eq!(ledger.tally(&first), Some(CheckpointPolicyTallyV1 { edits: 9, payload_bytes: 5_000, running: false }), "edits committed meanwhile stay tallied");
        assert!(ledger.observe(&first, 0, 0, policy), "the tally that grew meanwhile starts the next one");
    }

    /// 📌️ Once the edits committed since the active checkpoint reach the policy the hub checks the document in by itself, on
    /// behalf of the author whose batch reached it: the active checkpoint's baseline becomes the committed head and a cold
    /// open reads the replica fold of the ledger, byte for byte; a frontier-less hello from a seeding plan then resumes there.
    #[cfg(all(feature = "native-artifact-execution", feature = "integration-fixtures"))]
    #[tokio::test]
    async fn the_checkpoint_policy_checks_a_document_in_at_its_committed_head() {
        let mut fixture = check_in_fixture("policy").await;
        let mut readiness = (*fixture.state.readiness).clone();
        readiness.features.checkpoint_policy = HubCheckpointPolicyV1 { edits: 2, payload_bytes: 1_073_741_824 };
        fixture.state.readiness = Arc::new(readiness);
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["61", "62"]).await;
        check_in_commit(&fixture, ledger[0].clone()).await;
        let head = check_in_commit(&fixture, ledger[1].clone()).await;
        let (subject, _) = check_in_author(&fixture.state, &fixture.scope, &bearer_headers(&fixture.author.token)).await.expect("the author checks in");
        assert!(fixture.state.checkpoint_policy.observe(&fixture.scope, 2, 0, fixture.state.readiness.features.checkpoint_policy), "two committed edits reach the policy");
        let status = run_policy_check_in(fixture.state.clone(), subject, fixture.scope.clone(), fixture.handle.clone()).await.expect("the policy Check In starts");
        assert_eq!(status.phase, DocumentCheckInPhaseV1::Ready, "policy check-in ended {status:?}");
        assert_eq!(fixture.state.checkpoint_policy.tally(&fixture.scope).map(|tally| tally.running), Some(false));
        let active = fixture.state.directory.get_active_artifact_checkpoint(&fixture.scope).await.expect("active read").expect("active checkpoint");
        assert_eq!(EditedArtifactFrontierV1::of_artifact_frontier(&active.baseline_frontier), Some(head), "the active checkpoint is the committed head");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(cold.selection.active_checkpoint_id, active.checkpoint_id);
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), check_in_replica_pair(&fixture, &ledger).await, "the policy checkpoint is the replica fold, byte for byte");
    }

    /// 📌️ Check In folds the hub's own ledger onto the active checkpoint and publishes it: after two
    /// edits the active checkpoint's baseline is exactly the named head, its parent is genesis, and a""", "laws")
print(f"states-mid {states}")
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
