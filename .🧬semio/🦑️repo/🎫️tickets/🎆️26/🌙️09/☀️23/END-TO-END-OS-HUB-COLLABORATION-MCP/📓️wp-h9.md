# WP-H9 — Hub Backend: Check In Command, Fallible Stores, Declared Authorization, db Gate, Live Trace Leg

Slice: H9 (session 11). Ports: hubs 8010–8019, serves 6510–6519. Private cargo target: `.tmp-ticket/wp-h9/target`. Captures: `wp-h9/generated/`.

## Status

| # | Item | Status |
|---|------|--------|
| 1 | Check In as a hub command (H8 items 1–7) | DONE (native + wasm32 compile): ABI leg, schema (JSON + Rust + TS + fixture, Ajv **2/2**), upload route deleted with every caller, hub job + routes, os React shell + native wgpu shell wiring (en + de). Hub laws **3/3 PASS** (advance + cold open, stale/unknown/foreign, idempotent/cancel/revoke), job laws **3/3**, `check-in-check source` **32 checks** (`generated/laws-run-7.txt`, `check-in-source-1.txt`). Process phase (approval after edit + check-in, `proveGisMapProposalProcess`) needs a current-tree catalog → waits on W2's `w2-catalog-all` |
| 1a | `gis_map_abandoned_pre_witness…` flake root cause | DONE: root cause = the Preflight commit turn was the only turn that did not yield, so the approval could race through assembly, journal and publication inside one manual poll; the law then saw the test publisher's injected attempt-0 `Storage`. Fix: `Preflight` yields like every other turn. Before: 1/1000 alone, 6/3000 under load. After: **0/3000** under load (see Item 1a) |
| 2 | Store writes that can fail (O2-5b) | DONE: verified already fixed (W3d); one silent caller fixed (agent-delegation revoke → `503 directory-unavailable`); law `credential_sign_in_reports_a_failing_instance_session_store` **PASS** |
| 3 | Declared authorization (O2-10) | DONE: one schema-first policy (`HubAccessPolicyV1`), one authority `hub_access_permits`, every hub gate routed through it; Rust laws **2/2** (113 vectors) + TS/Ajv **2/2**; removed member → 403 on the live job and `authority-changed` on the running Check In |
| 4 | db gates (in-process `--lib` + nextest) | DONE: root cause found + fixed. In-process `--lib` failed 2 of 4 runs under load (19 and 1 failures); cause = lost wake in artifact-runner retirement (see Item 4). After fix: in-process **705/705 ×6**, nextest **705/705** |
| 5 | Observability live leg (document socket open/close + presence in two-client e2e) | DONE: two-client e2e on my hub **:8010** (W2's binary, catalog A copy in `wp-h9/catalog-a`, own temp data root) **2/2 PASS** in 347 s; 27 trace lines Ajv-valid + declared; document socket open (`upgrade`, ok) / close (`closed`, cancelled) share `requestId r000000000011`; presence join 2 / leave 2 / expiry 1; `server.shutdown:ok` (`generated/two-client-sqlite-1.txt`, `two-client-sqlite-receipt.json`) |
| 6 | Gates: hub quick + long, os-hub-ts vitest, pg/neo4j | DONE: hub nextest `long` **346/346** (1 skipped), `quick` **337/337** (10 skipped), `os-hub-ts` vitest **19 passed / 2 skipped** (5 files), live WAL-writer fence sqlite/postgres/neo4j **3/3**, directory live lanes postgres **12/12**, neo4j **7/7**, corpus 6/6 (inside the all-features run) |
| a | P0-1 creation law on a real genesis-capable guest; DB4 document lane pg/neo4j | PARTIAL: **P1-1 verified stale** (live: my e2e created and co-edited an `s.note.note` document — note is not in the native provider table — on catalog A via its guest codec; unlinked packages get an empty native closure). No hardcoded gate blocks a kind (the only literal fence is W2's `local-stdio-gis-open-v1` profile-id check, which only constrains a profile of that exact id). **P0-1 BLOCKED:** a real genesis-capable guest from this tree does not exist yet. My 14-export ABI means the 09-24 release components are refused; the only current-tree GIS component is the 226 MB dev build (03:11), too heavy to compile inside a debug unit law. It needs W2's `component-release` rebuild. The DB4 pg/neo4j document lane is blocked the same way (needs a current-tree catalog plus a pg/neo4j binary) |
| b | P2-2 `Database::shutdown` on every exit path; restart < 1 s on all backends | PARTIAL: code DONE (every exit after `connect_db` closes the database, see Item b). Live, db level: WAL-writer fence `release-admits-contender` + `process-exit-releases` **3/3 backends**. Live, hub level sqlite: SIGTERM→exit **298 ms**, close code 1012, `server.shutdown:ok database=closed`, reopen on the first attempt. Hub-level pg/neo4j restart probe needs a current-tree catalog (same block as a) |
| c | P2-1 `HubSagas` decision (real saga or delete) | DECIDED — keep the drain, empty saga set (see Item c) |
| d | P2-4 hub suite with postgres/neo4j features (live-database-lanes) full count | DONE: `cargo test -p semio-hub --all-features` full count **372/375** (lib 225/227, bin 147/148; Docker live lanes included). Fixed in this pass: approval retry 409 (Item d), trusted-publication crash law ENOTDIR (Item d). Remaining 3: the P0-1 creation law (blocked, row a) and 2 trusted-catalog laws that pass alone but race on the process-wide codec registry in-process (W2's area, already flagged in `wp-w2.md` §2.1) |
| e | Local-bootstrap pipe: 64 exchanges per run, then the hub exits (W2) | DONE (law): the replay set is now bounded per 15 s validity window, and every per-request refusal is answered with a signed `reject` — see Item e |
| f | A hub still loading its catalog ignores pipe EOF (W2) | DONE (law): a pump owns the pipe from hello onward and raises `closed` on EOF; startup cancels the catalog load and exits cleanly — see Item e |

## Landing (guest ABI, 00:56 → 01:13)

H8's frozen `codec.replay-envelopes` set, re-derived on the current tree (not the frozen text verbatim):
- WIT `codec.replay-envelopes(artifact-kind, pair, envelopes) -> result<document-pair, plugin-error>`.
- Kernel `store::replay_envelopes_onto_pair(pack, spr, envelopes, owners: impl FnOnce() -> DocumentStoreOwners)` — folds
  every envelope through `ArtifactStore::ingest_remote`; refuses a quarantined conflict (`!accepted || conflict`), an
  envelope left pending on an unknown dependency (`MutationDag::pending_is_empty`, new), a corrupt stream, a missing
  baseline. Owners are a factory (an unused `ArtifactStoreCursorDisposer` asserts in `Drop`). The close cursor shared with
  `apply_ops_binary` moved into `close_codec_reduction_store`. `ArtifactCodec::replay_envelopes` thunk.
- Plugin crate: `PluginApp::artifact_replay_envelopes`, `artifact_app_replay_envelopes::<A>` (app owner catalogue),
  `plugin_artifact_replay_envelopes`, component `codec::Guest::replay_envelopes`, owned `semio_owned_replay_envelopes_v1`
  (`OwnedSemioExport::ALL` 13 → 14). Plugin host: `OwnedOperation::ReplayEnvelopes`, owned
  `codec_replay_envelopes_observed` (fuel progress), wasmtime `codec_replay_envelopes`, `GuestRuntimes::codec_replay_envelopes`.
- MCP guest-backed codec `guest_replay_envelopes`; hub `TrustedArtifactReplayCodec` (+ impls for the native/guest
  catalog codec and the plugin-host codec); hub fixture codecs.
- Consequence recorded in `📓️landing.md`: a host built from this tree refuses components built before it.

## Item 1 — Check In design (as landed)

- **Contract** (`📇️directory/🧬️schema`): `DocumentCheckInV1 {schema, requestId, head: EditedArtifactFrontierV1}` and
  `DocumentCheckInStatusV1 {phase accepted|materializing|publishing|ready|failed|cancelled, progress {completedUnits,
  totalUnits = 8}, ready?, refusal?}`; refusals `unknown-head | stale-head | active-checkpoint-changed |
  ledger-not-replayable | codec-refused | authority-changed | unavailable`. `CheckpointPublicationFrontierV1` renamed
  `EditedArtifactFrontierV1` everywhere (GIS inference, MCP dispatch, wgpu test, TS). H8's `label` dropped: the version
  message is the ledger's own `CommitCheckpoint` transition; an echo the hub never stores would be decoration.
  Fixture `📌️document-check-in-v1/🧫️fixtures/🔣️.json` (9 valid, 18 invalid, `schemaValid` flags for the canonical-form
  and progress laws JSON Schema cannot express).
- **Job** (`🌎️hub/🗿️artifact-authority/📌️check-in`): `ReplayingArtifactAuthority::materialize_check_in` (resolve codec →
  validate input → `TrustedArtifactReplayCodec::replay_envelopes` → validate output → candidate at baseline = head,
  parent = active checkpoint), `DocumentCheckInJob` (monotonic progress, one terminal outcome, `cancel`, `revoke` →
  `authority-changed`, it is the `AuthorityOperationControl`), `DocumentCheckInJobs` (join by author/document/request,
  256 live, 1024 retained, conflict on a different request under the same id).
- **Hub** (`🏗️bootstrap` `📌️CheckIn`): author-only (401/403), durable idempotency through the existing
  checkpoint-publication claim ledger (request id = correlation id, SHA-256 of the canonical body), stall-bounded
  (30 s, guest fuel counts), 50 ms author revalidation monitor, active pair from `VerifiedRebootstrapSource`, ledger from
  `db::document::artifact_ledger_tail`, fenced publication (author still writes, descriptor unchanged, active
  checkpoint still the parent) completing the claim in the checkpoint event's transaction; `head == baseline` answers
  the active checkpoint (no-op), a head behind it `stale-head`, a head not on the ledger `unknown-head`; trace
  `server.document.check-in` (vocabulary 23 → 24).
- **Deleted** (no legacy): `POST …/checkpoint-publications`, `CheckpointPublicationCommandV1/CurrentV1/BlobV1/ReceiptV1`,
  `CHECKPOINT_PUBLICATION_*` consts, the command JSON defs, `📣️checkpoint-publication-command-v1/`,
  `🌱️artifact-genesis-v1/📤️current.json`, the two upload-route laws, `publishCheckpointPublicationProcessPairV1` and
  `proveCheckpointPublicationCommandV1`. Process probes (MCP cold mount, GIS proposal, two-author shell) now commit a
  real GIS Map ledger edit and Check In its head (`checkInProcessHeadV1`); gate renamed `check-in-check`
  (`os-hub:check-in-check|check-in-native-check|check-in-process-check`, launch.json `⚖️gate📌️check-in…` 411.11–13).

## Item 2 — Store writes that can fail (O2-5b)

Verified on the current tree: `server::storage::{ProjectionStore::put/set_checkpoint/clear, SessionStore::create/delete}`
all return `Result<(), StorageError>` (W3d, ticket 26/09/18) and the hub's `HubProjectionStore`/`HubSessionStore` append +
flush + `sync_data` before folding, with fault laws (`a_projection_write_reports_a_failing_sink`,
`a_session_write_reports_a_failing_sink`). Callers: sign-in (`record_instance_session` → refuse + withdraw the directory
issuance), `DELETE /auth/sessions/me` and credential change (→ 500/503) already surfaced it; **the agent-delegation revoke
route discarded it (`let _ =`)** and now answers `503 directory-unavailable` (`🏗️bootstrap` `delete_agent_delegation`).
New route-level law: a sign-in whose instance session directory refuses writes answers 503 `directory-unavailable` and
records nothing.

## Item 3 — Declared authorization (O2-10)

- **Policy** `🌎️hub/🔐️auth/🛡️access-policy/🔣️.json` (schema `schema://hub.auth/HubAccessPolicyV1`, defs `HubAccessRoleV1`,
  `HubAccessActionV1`, `HubAccessGrantV1`, `HubAccessDecisionVectorV1`): roles `admin | owner | author | spectator | share |
  authenticated`; actions `space.create|rename|visibility|archive|delete, member.upsert|remove, invite.create|revoke,
  document.announce|read|write|check-in, agent.delegate, artifact.create, blob.read|write`; archive denies every write.
- **Authority** `semio_hub::auth::access_policy::hub_access_permits(roles, action, space_kind)`; roles are derived
  (membership, ownership, share token, operator subject), decisions never hand-written.
- **Routed through it:** directory commands (`authorize_directory_command` = one `permits` call), document read
  (`authorized`, canonical pair), space blobs (`authorized_for_blob(…, BlobRead|BlobWrite)` — writes now need the write
  grant, a spectator can no longer PUT a blob), document-socket `SecurityGate` (compiled from `document.read`/`write` in the
  space's kind; a share now maps to its own `share` role), open-plan surface writability, Check In, artifact creation
  (route + final commit authority), agent delegation, GIS approval delivery. `db::security::space_grants` (the second,
  divergent copy) deleted with its two laws.
- **Laws:** Rust `declared_access_policy_matches_the_language_neutral_truth_table` (113 vectors in `🧫️fixtures`),
  `access_policy_is_closed_by_default_and_deny_overrides_allow`; TS twin + Ajv `🧪️tests/🛡️access-policy` **2/2**
  (`generated/hub-ts-access-1.txt`).

## Item 1 — os clients (as landed)

- **Worker protocol** (`💻️os/🟦️.ts`, `🏪️store/👷️worker/🟦️.ts` `📌️DocumentCheckIn`): `document-check-in {requestId,
  clientInstanceId, scope}` / `document-check-in-cancel`, answered by `document-check-in-status {status}`. The worker
  waits (≤ 15 s) until the replica is quiescent (no pending batches, empty outbox, no pending mutations) and names its
  acknowledged frontier as the head, submits, polls every 100 ms (deadline 180 s, 8 concurrent), cancels on request.
- **React shell** (`🏛️ShellHost`): a checkpoint this shell dispatched that lands in history triggers `requestHubCheckIn`;
  History panel item `framework.history.checkin-status` (`role=status`) + abort button; labels `checkinStatusText` /
  `checkinAbortText` en + de (`🛠️ShellHelpers`).
- **Native wgpu shell** (`🐚️Shell/🎯️targets/🧊️wgpu`, not wasm32): `ArtifactSyncStatus.acknowledged_head` (new; only when
  outbox and pending set are empty) is the head; `request_hub_check_in` on the landed checkpoint, `pump_hub_check_in`
  per frame from `pump_directory_events` (submit → poll → cancel, 401/403 → `authority-changed`), hub footer badge shows
  the phase via `shell_chrome_string` `checkIn.*` en + de.

## Item c — HubSagas decision

Keep `HubSagas` as the CQRS outbox drain with an empty saga set: it is the only retirement path for `/commands` outbox
events; every cross-aggregate consequence (membership removal → socket revocation, delegation revoke) is synchronous
under the membership fence, so there is no multi-step saga to run. Recorded on the type (`🌎️hub/🗄️stores/🦀️.rs`).

## Item 4 — db gate: lost wake in artifact-runner retirement

- **Symptom:** `cargo test -p semio-framework-os-kernel-db --lib` in-process failed under load: run 1 had 19 failures
  (engine laws failing at `Database::open(...).unwrap()`), and a later run had 1 failure:
  `artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity` reported "retirement did not release its slot:
  driver=5 (ClosingReady) terminal=false turns=0 maintenance=true" after its 60 s watchdog (`generated/db-lib-loop-3.txt`).
  nextest (process per law) was green.
- **Cause (`🛢️db/🗿️artifact/🦀️.rs`):**
  - `ArtifactRunnerRetirementReservation::commit` published the maintenance ticket to the handoff *before* it wrote the
    cursor into its global slot.
  - A runner transition in that window (the close-poll drop reaching `ClosingReady`) requested maintenance. A pool worker
    ran `artifact_runner_retirement_step`, found the row empty, and returned `Retire`, so the hook was removed.
  - The commit's own request then hit a stale ticket (ignored). The cursor, its slot (1 of 64, process-global) and its pool
    use stayed stranded for the life of the process (`turns=0`).
  - Stranded slots exhaust the 64-slot table, and every later mount fails. That is the 19-failure cascade.
- **Fix:**
  - `commit` writes the cursor first and publishes the ticket last.
  - The step answers `Idle` (keeps the hook) for a reserved-but-uncommitted slot. Only a generation that no longer owns
    the slot retires it.
  - New law `retirement_turn_before_its_commit_keeps_the_hook_for_the_committed_cursor` (deterministic, calls the step
    between reserve and commit).
- **Measured after the fix:** in-process 705/705 on 6 consecutive runs under load (`generated/db-lib-fixed-{1..6}.txt`);
  nextest 705/705 (`db-nextest-2.txt`).

## Item b — `Database::shutdown` on every exit path

- **Before:** every `?` in `main` after `connect_db` returned without shutting the database down. That covers the
  directory, CAS and coordinator setup, the extension dir, the GIS binding, the inference ledger, session-key minting,
  `compose_hub_server`, and a port-in-use `TcpListener::bind`. The storage close was skipped, so the Neo4j lease, the
  Postgres advisory session and the sqlite sidecar were only released by process death or TTL.
- **Now:**
  - Everything after the open runs inside one `served` block. `close_hub_database` runs unconditionally after it, on
    serve, on a refused startup and on a bootstrap-pipe close.
  - `hub_shutdown_record` emits one `server.shutdown` record for every exit (`retained-sockets=N` or
    `startup-refused=<error>`, plus `database=closed|<error>`).
  - Check In jobs, which hold `Arc<Database>`, are cancelled and drained before the close, so the database's `try_unwrap`
    can succeed.
- **Live restart-within-1 s probe:** still to run. It needs an os-hub with the postgres + neo4j drivers from this tree
  and a current-tree catalog (catalog A is refused by 14-export hosts).

## Item 1a — `gis_map_abandoned_pre_witness…` flake

- **Instrumentation:** every `GisMapApprovalCommitErrorV1::Storage` construction in the runtime (42 sites) now goes
  through one `#[track_caller] gis_map_storage_refusal(&error)`. It records `file:line: error` in test builds, which
  replaces H8's `[DEBUG]` print and the six hand-named sites (codemod `wp-h9/codemods/gis-storage-refusal-sites.py`).
- **Measurement:** failures kept reporting `last storage refusal None`, so the `Storage` came from outside the runtime.
  The only other constructor on that path is the law's own `OrderedApprovalCheckpointPublisherV1`, whose attempt 0
  returns `Storage` by design.
- **Mechanism:**
  - `poll_approval_to_phase` polls the approval by hand and checks the retained state between polls.
  - `Preflight` was the only commit turn with no yield. When the actor's snapshot reply was already there (the test
    thread preempted right after the ask, which is common under load), one poll ran Preflight → Assembly.
  - The harness never saw `Ready{pending}`. It kept polling through journal and publication until the injected failure.
- **Fix (`🏃️runtime/🦀️.rs`):** the `Preflight` arm of both commit loops (approval and undo) yields once before the
  snapshot, like `Continue` and `Committed`. The rule is documented on `GisMapCommitTurnV1`.
- **Measured** on a copied test binary, one law per process, under load (two loops plus a cargo build):
  - before: 1/1000 alone (`flake-exact` run 644) and 6/3000;
  - after: **0/3000** (`generated/flake-fixed-{a,b}.txt`).

## Item d — all-features fixes

- **Approval retries answered 409.**
  - Each retry of a prepared approval re-stamped a fresh command with the wall clock. This happens whenever
    `document_clock` is `None`: fail-closed committer, or document not mounted.
  - The ledger then refused the new command hash against the prepared outbox (`prepare_approval` → `Conflict`).
  - `approve_gis_map_job` now reuses the durable prepared command (`approval_recovery_by_mutation`, uncommitted, same
    job + proposal).
  - `gis_map_approval_is_idempotent_across_duplicate_requests_and_restart`: **PASS**.
- **`trusted_publication_owner_process_crash_releases_exact_lock` failed with ENOTDIR.**
  - Server-owned roots are opened per component with `O_NOFOLLOW|O_DIRECTORY`, so a `SEMIO_TEST_ARTIFACT_DIR` reached
    through a symlink (`.tmp-ticket`, or macOS `/tmp`) is refused.
  - `test_artifact_root()` (`🌎️hub/🧪️tests/🗂️artifact-root`) now returns the canonical path. **PASS**.

## Item e — local-bootstrap pipe (W2's two defects)

- **64 per run → per window.**
  - `consumed` was a non-evicting 64-slot set. The 65th exchange failed `insert` → `Unauthorized` → `serve_local_bootstrap`
    returned `Err` → the hub exited (W2 measured this at 11:41).
  - It is now `ExchangeWindow<LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX = 64>`. Entries are evicted once their own `expiresAt` is
    past; no frame carrying them can validate again, and the strict sequence already forbids replays. The fixture
    declares `replayWindowExchangesMax: 64`.
- **Refusal = answer.**
  - Integrity is still channel-fatal: an unparseable frame, a wrong run id, a wrong sequence or a bad proof.
  - Everything after authentication is answered with a signed `reject` and the loop continues:
    - an expired window → `expired`;
    - a device-text, profile or client-class violation → `denied`;
    - a replayed live id → `denied`;
    - a full window or a full pending set → `resource-limit`.
  - Stale cancel or shutdown frames are ignored. A cancel only marks a still-pending exchange, so the cancelled set can
    never overflow.
  - Request-task failures (issue, delivery, reject write) stay per-request; only a panicking task ends the service.
  - The TS launcher now raises `LocalBootstrapRefusedError(code)` on a verified reject, instead of reporting a binding
    mismatch.
- **EOF during loading.**
  - A pump task owns the read half from hello onward. It feeds `accept` through a bounded channel (8) and raises
    `closed` on EOF or a broken frame. `LocalBootstrapTransport::closed()` is new.
  - In `main`, a watcher turns `closed` into `StartupCancellationV1`. `StartupCatalogControl::is_cancelled` observes it,
    and so does the CAS handshake control.
  - A cancelled catalog load returns `Cancelled` without being retried as a stall. `main` emits
    `server.shutdown: cancelled launcher-closed-during-catalog-load database=unopened` and exits `0`.
  - A launcher that closes or shuts down the pipe while the hub is serving is now a clean `Ok` exit. It used to be
    `Err("local bootstrap endpoint closed")`.
- **Laws:**
  - `local_bootstrap_refusals_are_answers_and_the_replay_window_frees_with_time`: 64 exchanges fill one window, then
    resource-limit / replay / expired / unknown-profile each get a signed reject while the pipe stays ready. After the
    window passes, the next exchange is admitted. A forged proof is the one frame that ends the pipe.
  - `local_bootstrap_closed_resolves_when_the_launcher_leaves_before_anyone_accepts`.
  - `launcher_close_cancels_the_startup_catalog_load_instead_of_retrying_it`.
  - All **PASS**, with the existing local-bootstrap laws still green: 5/5 lib + 1 bin (`generated/laws-run-12.txt`).
- **Consequence of W2's one open-target rule (12:23):** the synthetic stdio JSON viewer can no longer be an open target.
  A manifest-level kind must equal its app's dialect, and dialects must be canonical `s.…`. The three Check In laws
  therefore moved onto the verified GIS Map profile (`check_in_map_edits`), under `integration-fixtures`. All 3 pass
  again, and the `check-in-check` native phase now builds with `integration-fixtures`.

## Log
- 00:56 landing row announced; 01:13 all native + wasm32 checks green (`generated/abi-*.txt`).
- 05:2x resumed after usage reset; wgpu Check In already landed + compiled (native + 4× wasm32 green, `generated/wasm-check-2.txt`); Docker answers.

- 05:25–06:37:
  - Check In lifecycle law fixed (publication failure under a revoked author → `authority-changed`; a removed member
    gets 403, not 401). All three hub laws pass.
  - Two-client e2e on :8010 **2/2**.
  - db retirement lost-wake fixed.
  - Flake root-caused and fixed.
  - Every exit now closes the database.
  - All-features count **372/375**; gates green.
  - Cleaned up: binary copies, the catalog A copy, test-artifact dirs, the private target and the `*-before-*` backups.
- 12:16–12:35: an external low-disk cleanup deleted `wp-h9/generated` (captures cited above up to 06:37 are gone; the results stand as recorded, re-measured where noted). Data roots and catalog copies now live under `.🧬semio/🌐hub/s11-h9-*` (revised rule 15).
