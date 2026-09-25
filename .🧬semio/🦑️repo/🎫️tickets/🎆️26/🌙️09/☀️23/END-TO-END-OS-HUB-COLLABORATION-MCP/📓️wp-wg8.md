# WP-WG8 — wgpu Native Collaboration: Kernel Turn (B1), Guest-Owned Codec (B2), Live Check, Creation Door

Slice: WG8 (session 11). Ports: hubs 8090–8099, serves 6590–6599. Private cargo target: `.tmp-ticket/wp-wg8/target`.
Captures: `wp-wg8/generated/`. Inheritance: `.tmp-ticket-0918/📓️g7w-…md` (B1, B2, B3, §8), `📓️n2-…md` §6.4,
`📓️wg6-…md`, `📓️tc3b-catalog-genesis-landed.md`; sibling `📓️wp-wg7.md` (N2 relay, `ureq` move, wasm32 `connect`).

## Status

| # | Item | Status |
|---|------|--------|
| 1 | B1 — native kernel turn never returns after a real guest boots | **FIXED, law green (measured 05:39)**: `a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub ... ok` — open 14.6 s (debug interpreter), every surface admitted, backbone bind receipt accepted, `addHandleKind` once in 3.6 s |
| 1b | §1.4 — native reserved-tool jobs (undo, redo, selection, clipboard) spawned but never stepped | **FIXED, one mechanism with React + guest (07:3x)**: 4/4 native journey laws green in 5 consecutive runs (edit, undo, redo, select-all + copy + paste; `generated/journey-{17..21}.raw.txt`), new shard law, kernel fixture laws (Rust 7/7, TS) — §1.4 |
| 2 | B2 — native document actor resolves a guest-owned kind through the mounted component's `codec` interface | **LANDED, laws green** (store resolution law, sync `componentIdentity` fixture law, 190 kernel sync/channel/client laws); **live `Live` not run** — needs a hub catalog carrying block (§2, §6) |
| 3 | `hub-live-collaboration-check` steps 1–12 green (runner: **WG8**, per WG7's scope split) | **GREEN (17:0x), all 12 steps, test exits 0 in 209 s** on hub 7800 catalog B (runId `345ceda4…`, generation `e8167ce8…`): `two_live_wgpu_shells_collaborate_on_one_hub_document ... ok` (`.🧬semio/🌐hub/s11-wg8-captures/collab-live-18.raw.txt`). Five native defects fixed on the way, each measured first (§3) |
| 4 | G-P1-3 — wgpu artifact-creation door (schema-first, progress + cancel, en + de) | **DONE, live-proven (05:42)**: a native wgpu shell created a hub artifact from its own door on hub 7800 — catalog ready (gis, note), `accepted → preparing → ready` in 53.7 s, `artifact-db290b13…`; 4 fixture laws green (§4) |

## Coordination (read me, WG7 / coordinator)

- **Runner of `hub-live-collaboration-check`: WG8** (WG7's scope split, `📓️wp-wg7.md`).
- **Kind identity — one mechanism.** New in the kernel store (`🏪️store/🦀️.rs`, region `CodecRegistry`):
  `ComponentDocumentCodec` (trait: `schema`, `pack_schema_hash`, `print_mirror`), `register_component_document_codec`,
  `DocumentKindCodec { Linked, Component }` and `document_kind_codec(schema)` — linked Rust codec first, else the
  codec of the mounted component that owns the kind (the hub's trusted-catalog order: linked `codec`, else `guest`).
  WG7's shared helper `document_pack_schema_hash(schema, lease)` (region `DocumentSocketConnect`) now asks
  `document_kind_codec` first, then the lease; the native actor's `start_connect_hub`, `finish_connect_hub` and
  bootstrap identity use that one helper; bootstrap validation and folder mirror persistence use
  `DocumentKindCodec::print_mirror`. The browser host can register a jco-backed component codec the same way.
- New fixture cases `componentIdentity` in WG7's `🏪️store/🧫️fixtures/document-socket-connect-v1/🔣️.json`
  (+ runner `a_mounted_component_codec_is_the_kind_identity_before_any_lease`); every case owns a distinct schema
  because the component registry is process-wide — WG7's `packIdentity` cases (schema `block.2d`) are untouched.

## 1. B1 — root cause and fix

Measured inheritance (G7w runs 10–13, `.tmp-ticket-0918/wp-g7w/generated/g7w-live-1{0..3}.txt`):

- run 10/11: the first post-boot turn was **cut on its 100 ms wall grant** (owned interpreter → shard
  `DeadlineExceeded` → reported as `MoreWork`); the next ordinary event reached a guest still mid-call →
  `owned turn is mid-flight` trap → failure ladder → `tick` grants nothing → "shard produced no outcome".
- run 12/13 (G7w's resumed settle loop: continue every `MoreWork` with `Event::Wake` until `Idle`): the open
  settles, but an authored edit spins **30 138 `MoreWork` turns** until the 30 s budget
  ("turn did not settle within 30s", steps 8/10/11/12).

Root cause (two conflated meanings of `MoreWork`):

1. A **preempted** owned turn (cut mid-call, owns the next call) and a guest's **completed** turn saying it has
   more work were the same outcome, so the host could not know when resuming was mandatory.
2. The settle loop swallowed the **command-ingress protocol**: `exchange_commands` drives one page per turn
   and must observe every `CommandIngressStatus`; the settle loop absorbed a `PageAccepted`/`CommandPending`
   into later `Idle`-ingress Wake turns (`ExchangeOutcome::absorb` keeps the LAST ingress), so the driver
   re-offered the page / never saw completion, and a guest waiting on a host round trip (command page,
   typed-operation ACK) answered `MoreWork` forever.

Fix (source landed 01:5x, `cargo check` green, see §5):

- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs`: new `ShardOutcome::Preempted { actor }` (pack tag 6) — emitted when a turn
  faults `DeadlineExceeded | FuelExhausted` **and** the instance is still `turn_in_flight()` (owned
  interpreter). Wasmtime's non-resumable cut keeps its old `MoreWork` shape. `FuelExhausted` on a resumable
  instance was a latent trap (kernel fault ladder) — now a preemption too.
- `🧊️renderer/🦀️.rs` `run_turn_once`: any granted actor reported `Preempted` gets an `Event::Wake` resume
  envelope inside the same tick loop, bounded by `RUN_TURN_SETTLE_BUDGET` — a request never hands a mid-flight
  guest back to its caller.
- `run_turn`: the React host's settle rules (`settlePluginTurn`): lifecycle receipt → ack; completed `MoreWork`
  with **idle command and cold-pair ingress** → continue; non-idle ingress → return to the caller's page
  driver; stop after `run_turn_quiescent_continuations()` (= `max_patch_bytes / max_text_bytes × 8` = 128,
  the same derivation as React's `PLUGIN_UI_QUIESCENT_CONTINUATIONS`) continuations that carried nothing
  (`ExchangeOutcome::carries_nothing`, native twin of `shardTurnCarriesNothingV1`).
- `🏃️run/🦀️.rs` (`semio-framework-os-run`): the same `Preempted` outcome is resumed there (its own tick loop).

Laws: `a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub` (green) and
`a_native_guest_undoes_its_authored_edit_without_a_hub` (green since §1.4) (`🐚️Shell/🧪️tests/🔗️hub-projection-workspace`),
driven by the new language-agnostic fixture `🧫️fixtures/⏯️native-guest-journey/🔣️.json` (open relay args, verb,
undo, expected ledger deltas); verb `native-guest-journey-check` (renderer package, nx target of the same name,
launch row `⚖️gate🧊️wgpu⏯️native-guest-journey` after `⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration` in both
launch files). Hub-less, so B1 can never hide behind a hub failure again. The temporary `[DEBUG] g7w` turn
logging (committed by G7w) is removed.
Run 1 (01:2x, pre-fix tree + pre-H9 guest): `component has no core module implementing the owned Semio actor ABI`
— the staged block component predates H9's 14th owned export (coordinator: not B1); block2d rebuild launched
through the wasm mutex (`wp-wg8/block-release.sh`, `generated/block-release-1.txt`).

### 1.2 Two more native-only defects found by the hub-less law (fixed)

Journey run 2 (01:5x, rebuilt post-H9 block2d): preemption now resumes (`outcome other` = `Preempted`, then the
turn completes), but every `SurfaceVisible` answered **no patch** → "retained surface … is not admitted".

- **Surface identity.** The kernel names an instance's surface `"<instance>:<surface>"` (the guest's
  `parse_surface_instance`; the wasmtime host builds the same from WIT `surface-ref`; `📥️ui-patch` names patches
  so). The native `ProgramBridge::render_with_document` sent a bare `"block2d-board"` — the owned interpreter
  hands the kernel event to the guest verbatim, the guest parsed no instance and mounted nothing — and looked the
  retained document up by the bare id. Fix: `kernel_surface_id(instance, surface)` for the event, the targeted
  advance and the lookup (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`).
- **Actor 0 had no patch transport.** Run 3: the first reconcile turn with patches faulted `fixed turn patch
  transport admission refused the exact owner`. The shard's transport session is the granted actor id, and
  `ActorId(0)` is real (first app of the first plugin: ordinal, kind, index, generation all 0), but the kernel's
  `UiTurnPatchTransportArena::reserve` and `UiTurnPatchTransportLease::try_from_token` refused session 0 as a
  sentinel. Every lookup already keys on the slot state, so 0 needs no sentinel. Fix in `🎠️kernel/🦀️.rs`
  (`semio-framework`, target-neutral) + law `ui_turn_patch_transport_admits_the_zero_actor_as_its_session`
  (`🎠️kernel/🧪️tests/🔬️ui-turn-patch`). This is also why G7w's actor A never painted while B (actor 16384) only
  lacked the surface identity.

Journey run 5 (02:4x): open 46.4 s (debug interpreter), `error=None` (every surface admitted), `addHandleKind`
`Ok(())` in 1.36 s, ledger `[] → [("addHandleKind", true)]`; commands now answer `commandComplete` per page.

### 1.3 Three more native defects on the edit/undo path (03:1x–05:3x)

- **Frames were prefix-decoded.** `decode_app_frame` accepted any payload that merely STARTED like a frame; a
  document-backbone binding receipt (pack-value tag 3 = `AppFrame::Document`) was swallowed as a frame, so the
  open relay failed with "binding returned 0 shell receipts". Fix: `decode_app_frame` refuses trailing bytes
  (`📡️spr/🧵️channel/🦀️.rs`, kernel crate, target-neutral) + law `decode_app_frame_refuses_trailing_bytes`.
- **Typed-operation pages were never acknowledged in time.** Reserved tool jobs (undo, redo, checkpoint, copy…)
  publish a `semio.typed-operation-page.v1` page and wait for its ACK; the renderer parked the page in a
  process-wide exchange no native consumer ever read and queued the ACK behind the running request, so undo spun
  516 silent `MoreWork` turns and never applied. Fix (React's `settlePluginTurn` rule): `apply_turn_result` puts
  the pages on `ExchangeOutcome::typed_results`, `run_turn_once` owes one ACK event per page, `run_turn` delivers
  every owed event (lifecycle ACK, typed ACK) on a turn of its own inside the same settle. The dead
  `TypedOperationResultExchange`/`MountedTypedOperationResultExchange`, `KernelRequest::AcknowledgeTypedOperationResult`,
  `deliver_typed_operation_result_ack`, both `acknowledge_typed_operation_result` accessors, their test file and the
  fixed-slot budget row (`⏳️async/🧫️fixtures/🧱️boxed-fixed-slots`) are deleted; the page's wire-bound law moved to
  `🧪️tests/🗞️typed-result-page`.
- **`absorb` let an `Idle` ingress overwrite `CommandComplete`.** With ACK turns inside the settle, the page
  turn's `CommandComplete` was replaced by the ACK turn's `Idle`, so `exchange_commands` resent the page: run 9
  applied one `addHandleKind` 64 times; run 10 resent it 6 466 times in 83 min (coordinator-reported 99 % CPU; `sample`
  in `generated/journey-10-sample.txt`: test thread in `author_edit → dispatch_action → exchange_commands`; run
  killed, pids 88067/88069/88099). Fix: `absorb` keeps the latest NON-idle ingress.

### 1.4 Native reserved-tool jobs — root cause and fix (landed 07:3x)

Two faults, both measured on the native journey (block2d release, no hub):

1. **Nothing stepped the job** (runs 11–14): the undo page answers `commandComplete` with the admission `Invocation`
   plus `SpawnJob { kind: "framework.reserved.tool" }`. The shard admitted the spawn as a replay seed and started it,
   but steps a job only on a host `Payload::JobStep { turn }` carrying the exact shard-minted `JobTurn`, which the host
   never learned; the renderer sent `JobStep` only for product-replay entries. `JobCompleted` never reached the guest.
2. **The seed closed silently** (runs 15–16, once the host stepped): `ShardLoop::pump: job 71 has no independently
   admitted operation authority (retained shard failure: plugin: ShardLoop::replay: checkpoint page admission refused
   for actor 16384)`. Every spawn's seed checkpoints the whole guest before it starts the job (so a host can replay
   it on another worker); block2d's checkpoint outgrows the fixed checkpoint pages, the seed closed, and the executor
   retained the failure where no host ever read it.

Fix (one mechanism; the guest half is C8's `admit_reserved_spawned_job` + one commit unit per turn, unchanged):

- **Kernel contract** `semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND` (next to `SPAWNED_JOB_STEP_CEILING`), TS twin
  in `🎠️kernel/🟦️.ts`, both asserted from the fixture `🧫️fixtures/🧵️spawned-job-drive` (`reservedKind`). The plugin's
  `app::FRAMEWORK_RESERVED_JOB_KIND` is a re-export; React's `PluginRuntime` uses the TS constant (3 literals gone).
  Pure additions — no ABI, pack-schema or codec-hash change (soft freeze).
- **Shard** (`🔌️plugin/🖥️host/🧵️shard`): `ShardOutcome::Turn { jobs }` reports each admitted spawn's exact `JobTurn`;
  `ShardOutcome::actor()`; a reserved seed is live-only (`MountedReplaySeed.replayable == false`: capture kind and input,
  then start — no guest checkpoint) and `validate_replay_request` refuses it. Law
  `a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay` (69/69 `shard::`).
- **Renderer kernel thread** (`kernel_runtime`): a fixed `ReservedToolJob` registry (64, like every job table there),
  filled by `admit_reserved_jobs` from each settled turn's reserved spawns and reported turns. `settle_reserved_jobs`
  runs after `exchange` and after `exchange_commands` completes (React: `driveReservedToolJob` serialized after the
  command's ingress): `run_reserved_job_once` grants one `Payload::JobStep` per turn, the step publication yields the next
  turn or ends the job, and the shard's deferred `JobCompleted` turn — an outcome no grant asked for — is counted as owed
  and awaited by `dispatch_turn` (per-outcome grant/owed tally), then settled by the same `settle_turn` rules (typed-page
  ACKs, `MoreWork`). Reserved spawns are host work, never `ExchangeOutcome.effects` (so never a product replay); every
  turn result of one dispatch is applied (a second used to overwrite the first). `ParallelRuntime::take_shard_failure`
  names a retained shard failure in the fault (that is how fault 2 was found).

Measured (runs 17–21, `generated/journey-{17..21}.raw.txt`): undo `Ok`, ledger `addHandleKind` → `false`, 3.8–4.3 s
(debug interpreter; the verb's own exchange 0.43 s, the rest is the shell's refresh exchanges); redo `Ok`, → `true`;
`selectAll`/`copy`/`paste` `Ok`, ledger unchanged (block2d has no clipboard producer, so the framework clipboard routes
answer empty — the fixture declares it). The reserved drive takes 2 steps per job (`Yield`, `Complete`). Two latency
outliers of ~30 s (run 18 undo, run 19 `selectAll`) did not recur in runs 20–21 under `[DEBUG]` timing (no kernel
request > 1.6 s, no outcome wait > 0.5 s, no foreign grant); the machine was running W2's release builds. Watch item.
Runs 22–23 (same timing, 07:5x–08:0x): no outlier; run 22's 4th open failed with `os.open-artifact could not switch to
block: plugin: wasm decode: byte range exceeds input` — W2's batch B was rewriting block's `component-release` under the
staged runtime at that moment (not a kernel fault; `run-collab-live.sh` republishes the runtime from W2's finished
component). One more guard from reading the drive: a submitted `JobStep` that no grant answered used to return an
empty `Ok` (the loop would resubmit the same turn); `dispatch_turn` now counts an empty reserved turn as settled only
when a step publication arrived, else it is the loud "shard produced no outcome".

### 1.5 Earlier notes (resolved)

- The undo spin (516 silent `MoreWork`) was the unacknowledged typed-operation page (§1.3, fixed); the undo job
  itself is §1.4.
- The backbone-bind "0 shell receipts" was the prefix frame decode (§1.3, fixed).

## 2. B2 — design and landing

- Kernel store: trait + registry + `document_kind_codec` (above).
- Plugin host: `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` — `OwnedComponentDocumentCodec` (owned interpreter +
  compiled component + schema; `codec.pack-schema-hash` asked once and kept; `codec.print-mirror` per call) and
  `GUEST_CODEC_BUDGET` (4 G fuel, 30 s no-progress wall — the hub's values; the hub still carries its own copy,
  follow-up: import this one).
- Renderer: `KernelClient::create_app(.., artifact_schema)`; `ProgramBridgeEntry::create_app` passes
  `app_document_schema(app_id)` — the app's `io.artifact_schema`, or for a viewer (which declares none) its
  package's schema for the same dialect, so a spectator's mount registers the codec too; the kernel thread
  registers the compiled component as that kind's codec right after compile (`CreateAppRequestOwner` gained the
  field inside its bounded close; its shutdown law updated).
- Follow-up (not done): the hub's `GuestArtifactCodecBinding` could adopt `OwnedComponentDocumentCodec` once the
  kernel trait grows genesis/apply-ops with fuel-progress observation; it keeps its own copy of the budget today.
- Laws (written, not run yet): store `a_kind_resolves_to_its_linked_codec_before_its_mounted_component`; sync
  `a_mounted_component_codec_is_the_kind_identity_before_any_lease` (fixture `componentIdentity`).

## 3. The 12-step gate on catalog B — runs 1–18 and five native fixes

Runner `wp-wg8/run-collab-live.sh` (block2d release runtime staged from W2's catalog component `0d1a9bcd…`; the release
descriptor was rematerialized at 12:18, so the publish verb accepts it). Captures moved to the durable
`.🧬semio/🌐hub/s11-wg8-captures/` after the 12:19 cleanup deleted `generated/` (runs 1–4 lost; their findings are below).

| run | result | measured cause → fix |
|---|---|---|
| 1 | 1–5 ✓, 6 ✗ (sockets stay `Connecting`) | `there is no reactor running` panic on a pool worker at the hub dial (`🏪️store/🔄️sync`): the native document actor took whatever Tokio runtime its spawner was inside, and a wgpu shell spawns from a plain thread. **Fix:** `document_socket_io_reactor()` — one current-thread I/O+time reactor on its own `semio-document-io` thread, entered only while an actor is polled. Law `a_hub_dial_polled_off_any_runtime_is_driven_by_the_document_socket_reactor` (kernel `os_store::sync` 67/67) |
| 5–8 | 1–6 ✓ (**B2 `Live` proven**: native codec hash `1869126a…` = hub pin), 8 ✓, 11 ✓; 7, 9, 10, 12 ✗ | every authored edit refused by the document actor as `document backbone scope mismatch`: the guest's envelopes carried `document_id = s.block.block2d@1/*#editor` (its app id) — a fresh door artifact answers the socket `Bootstrap::None`, so nothing ever gave the guest the document's identity. **Fix:** the store contract gains `ComponentDocumentCodec::genesis` + `component_document_genesis(schema, id)` (zero-history check identical to the hub's `initial_pair`); the host adapter calls `codec.genesis`; `open_document` loads that genesis into the guest before its actor exists — the hub seeds the artifact from the same export. Law `a_document_opens_on_the_genesis_its_owning_component_mints_for_its_identity` |
| 13 | 1–6, 8, 9, 11 ✓; 7, 10, 12 ✗ | no presence heartbeat ever reached the hub: the native shell beats in the chrome walk's presence phase, which a headless law never runs, so the hub closed each idle socket after its presence lease (reconnect every 30 s) and B's edit waited out A's backoff. **Fix (law):** `frame_pump` = what one painted frame does for a document (sync pump + the presence phase → `advance_presence_preview_step`) |
| 14–15 | 1–11 ✓; 12 ✗ (pump 31 s / 7 s for a 3 s window) | `[DEBUG]` timing: every pump carrying a presence or status event called `refresh_ui(Full)` — every guest body re-rendered ten times a second per peer, 3.4–5.6 s per pump in debug. **Fix:** `pump_sync_events` refreshes guest bodies only when the guest's document changed (remote mutations, archive, backbone effects, terminal fault); status/bootstrap/conflict republish the host-owned Sync panel; presence is footer state painted next frame |
| 16 | 1–11 ✓; 12 ✗ (A's ledger read 0.8 s after relive) | the law read A before B's outbox flushed; now it pumps 10 s after relive, as steps 9–10 do. "Not frozen" is measured against the same shell's online edit (≤ 2×), not a 2 s constant a debug interpreter never meets |
| 5–17 | 8 of these runs never exited | sampled (run 17): the test thread parked for 23 min in `block_on(handle_hub_workspace_action)`; `hub_verb` now uses `drive` (pumps renderer I/O + worker retirements like the frame does). The hung test processes were mine; stopped by pid |
| **18** | **12/12 ✓, exit 0** | step 6 Live in 3.4 s, 7 both rosters online, 8 A authors (3.9 s), 9 B ingests, 10 B authors + A ingests, 11 per-actor undo propagates (`apply` false on B), 12 offline edit 4.6 s (online 6.4 s), pump 3.0 s, Stale → Ready, relive 0.1 s, A ingests the offline edit |

Also measured, not fixed here (routed): opening a document runs inside the frame pump (the door's `Ready` open held one
pump for ~20 s in debug: guest mount + genesis + manifest) — the native `os.open-artifact` relay should become a retained
operation like the creation itself; the native presence peer carries no app presence pack (no bounded ephemeral snapshot
API natively yet, the state React publishes when its snapshot misses the bound).

## 4. G-P1-3 — the wgpu artifact-creation door

- Directory client (kernel, target-neutral): `🔌️client/🌱️space-artifact-creation/🦀️.rs` —
  `space_artifact_creation_catalog`, `create_space_artifact`, `space_artifact_creation_status`,
  `cancel_space_artifact_creation` over the schema-owned `SpaceArtifactCreation*V1` types; exact routes
  (`SpaceArtifactCreationRoute`), bounded answers, canonical parse, space/request identity checked.
- Hub workspace (`🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs`): `HubArtifactCreationState` (catalog phase, catalog,
  chosen kind, name, pre-minted idempotency key, one creation), `hub_artifact_creation_intent`, the door section
  in the open space (catalog line + role, one choice per kind with the hub's en/de label, name, Create, the phase
  line + role, Cancel while cancellable, "Cancellation requested…", opening / failed + "Open artifact"); every text
  byte-identical to React's `ARTIFACT_CREATION_PROGRESS_TEXT_V1`.
- Shell: `hubSelectArtifactKind`, `hubSetArtifactName`, `hubCreateArtifact`, `hubCancelArtifactCreation`,
  `hubOpenCreatedArtifact`; `open_hub_artifact_creation` on open space; `pump_hub_artifact_creation` (one bounded
  hub request per frame: submit / cancel / poll ≥ 100 ms apart, 120 s → `Indeterminate`, `409` → catalog
  re-read) from both targets' `pump_directory_events`; `Ready` opens the artifact through the ordinary
  `os.open-artifact` relay (dialect coordinate, schema, space) and records `Opened`/`Failed`.
- Laws (read React's own language-agnostic fixtures `🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/{🔣️.json,
  🪪️catalog-authority/🔣️.json}`, so React's `ArtifactCreationProgressNotice` is the independent twin):
  `the_creation_door_speaks_the_fixture_texts_in_both_tongues`, `every_fixture_creation_phase_paints_its_role_and_controls`,
  `every_fixture_catalog_phase_paints_its_line_and_choices`, `the_door_seals_one_intent_for_a_chosen_kind_and_a_valid_name`.

Live (05:42, `generated/door-live-1.txt`): law `a_live_hub_artifact_is_created_through_the_wgpu_creation_door`
against W2's hub 7800 (catalog A, `user1@semio.dev`): catalog `ready` with kinds `s.gis.gismap`, `s.note.note`;
`s.gis.gismap` → trail `accepted → preparing → ready` in 53.7 s, ready `artifact-db290b13c9dd5c1b223e3d38e156edfd`
(`gis.map`); the door's tree reports phase `ready` in en and de; opening `Failed` as expected (no native gis guest
staged in this shell). Verb `hub-live-creation-check` (nx target + launch row `⚖️gate🔐️hub-auth🧊️wgpu-live-creation`
before `…-wgpu-live-collaboration`, both launch files). The two-user law now creates its block2d document through
the door (step `4a-create-through-the-door`) instead of inventing a document id.

Open: `os.create-space-artifact` (the Space-index guest's own create dialog, React's route) is not yet wired to
the same operation on wgpu — it needs the Rust twin of React's `encodeArtifactKindChoice`/dialog kind choices.

## 5. Checks

| when | command | result |
|---|---|---|
| 01:5x | `cargo check -p semio-framework-os-kernel --features sync,ureq -p semio-framework-plugin-host -p semio-framework-os-run --lib` | green (kernel 4 pre-existing warnings; my 4 `unused_qualifications` fixed) |
| 01:5x | same with bins | `semio-framework-os-run` **bin** red in a peer file (`🏗️bootstrap/🦀️.rs:43/50` `store::Media: Serialize` missing) — not mine |
| 02:0x | `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` | green (1 own break in the create-request shutdown law fixed; 129 / 281 pre-existing warnings) |
| 02:2x | `cargo check -p semio-framework --lib --tests` (transport session-0 fix + law) | green |
| 02:3x | `cargo check -p semio-framework-os-kernel --lib --features sync,ureq` (creation client) | green |
| 02:4x | `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (door + surface identity) | green (1 own missing import fixed) |
| 03:08 | wasm32: `semio-framework` + kernel `--target wasm32-wasip2`, kernel `--features sync --target wasm32-unknown-unknown` (store codec, transport session 0, creation client) | green (`generated/check-wasm-2.txt`) |
| 04:03 | same, after the exact frame decode | green (`generated/check-wasm-3.txt`) |
| 05:39 | same + renderer `--lib --target wasm32-unknown-unknown` (door, shell, typed ACK, all renderer edits up to 05:37) | green (`generated/check-wasm-4.txt`) |
| 05:28 | laws: kernel `--features sync,ureq` — B2 store + sync fixture laws, `decode_app_frame_refuses_trailing_bytes`, WG7's connect laws | 16 / 16 passed (`generated/laws-kernel-1.txt`) |
| 05:29 | kernel suites `os_spr::channel`, `os_store::sync`, `os_directory::client` | 190 / 190 passed (`laws-kernel-2.txt`) |
| 05:30 | `semio-framework` `ui_turn_patch_transport` (incl. the actor-0 law) | 10 / 10 with `--test-threads=1`; 8 fail in parallel — the suite shares the process-wide arena under `try_lock` (pre-existing, not the session-0 change) (`laws-framework-{1,2}.txt`) |
| 05:30 | plugin host `shard::` (incl. `Preempted` codec round trip) | 68 / 68 passed (`laws-plugin-host-1.txt`) |
| 05:31 | renderer `hub_connection::` (4 door laws + the verb-closure law updated for the 5 door verbs), typed-page laws, create-request shutdown law, slot-table budget | 37 + 14 passed (`laws-renderer-{1,2}.txt`) |
| 05:39 | native journey (staged post-H9 block2d release) | edit law **ok**, undo law red (§1.4) (`generated/journey-14.raw.txt`) |
| 05:42 | live door law on hub 7800 | **1 passed** (`generated/door-live-1.txt`) |
| 06:5x | `cargo check -p semio-framework-os-renderer-wgpu -p semio-framework-plugin-host --lib --tests` (§1.4 host drive) | green (only pre-existing warnings) |
| 07:0x | native journey runs 15–16 | undo red: first no step, then fault 2 of §1.4 named by the new retained-failure report |
| 07:1x | `cargo check -p semio-framework -p semio-framework-plugin -p semio-framework-plugin-host -p semio-framework-os-renderer-wgpu --lib --tests` (kernel constant, live-only seed) | green |
| 07:1x | `cargo test -p semio-framework --lib -- spawned_job` + TS `testSpawnedJobDriveContract` (bun) | 7/7 + TS pass (`reservedKind` asserted on both) |
| 07:1x–07:3x | native journey runs 17–21 (edit, undo, redo, select-all/copy/paste) | **4/4 green, 5 runs in a row** |
| 07:3x | `cargo test -p semio-framework-plugin-host --lib -- shard::` | 69/69 (`generated/laws-plugin-host-3.txt`) |
| 07:3x | `cargo check -p semio-framework-os-run -p semio-framework-os-renderer-wgpu --lib --tests` | green |
| 07:3x | renderer `kernel_runtime program_bridge typed_result` laws | parallel: 8 product-replay laws abort on a poisoned process-wide registry (pre-existing: shared registries); serially `kernel_runtime::semantic_document_tests` **34/34** (`laws-renderer-{3,4}.txt`); program-bridge + typed-page laws green |
| 07:3x | React package `typecheck` (PluginRuntime uses the kernel constant) | exit 0 |
| 08:0x | native journey runs 22–23 (outlier hunt, `[DEBUG] wg8` timing, reverted after) | 23: 4/4; 22: 3/4, the 4th open hit W2 rewriting block's component (above) |
| 12:1x–17:1x | two-user gate runs 1–18 on 7800 catalog B (captures from run 5 in `.🧬semio/🌐hub/s11-wg8-captures/`) | **run 18: 12/12, exit 0** (§3) |
| 16:5x | `cargo test -p semio-framework-os-kernel --lib --features sync,ureq -- os_store::sync os_store::component` | 448/448 ×2 (one earlier run: `retained_readiness_wake_after_turn_release_is_observed_once` missed its 1 s deadline under load; alone and ×3 in the suite green) |
| 17:1x | native journey run 24 (genesis-seeded opens) | 4/4 (`s11-wg8-captures/journey-24.raw.txt`) |
| 17:1x | renderer `chrome_maintenance sync_card hub_projection_workspace_tests hub_connection:: sync_panel presence` | 71/71 (`laws-renderer-5.txt`) |
| 17:2x | plugin host `shard:: component` | `shard::` 69/69; `component::` 220/225 — the 5 failures sweep stale staged components on disk (`target-g8/…/semio_s_plugin_note.wasm`: pre-H9 owned ABI; `replay-envelopes` missing), not these edits (`laws-plugin-host-4.txt`) |
| 17:2x | wasm32 gates (`wasm-checks-2.sh`: framework+kernel wasip2, kernel `sync` unknown-unknown, renderer unknown-unknown) | queued in the wasm mutex behind W2's `warm rest` hold and WG7 (`s11-wg8-captures/check-wasm-5.txt`) |

## Files (WG8)

Paths relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` unless absolute.

- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` — `ShardOutcome::Preempted` (+ codec round-trip law row in `🧪️tests/🔬️unit`);
  §1.4: `ShardOutcome::Turn { jobs }`, `ShardOutcome::actor()`, live-only reserved seeds (`replayable`), replay refusal;
  law `a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay` in `🧪️tests/🔬️unit`.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/{🦀️.rs,🟦️.ts}` — `FRAMEWORK_RESERVED_JOB_KIND` (Rust + TS twin);
  fixture `🧫️fixtures/🧵️spawned-job-drive/🔣️.json` (`reservedKind`) + both runners in `🧪️tests/🧵️spawned-job-drive`.
- `🔌️plugin/🦀️.rs` — `app::FRAMEWORK_RESERVED_JOB_KIND` re-exports the kernel constant.
- Gate (§3): `🏪️store/🔄️sync/🦀️.rs` — `document_socket_io_reactor` (+ law in `🧪️tests/🔬️native-actor-retained-turn-fixtures`);
  `🏪️store/🦀️.rs` — `ComponentDocumentCodec::genesis`, `ComponentDocumentGenesis`, `component_document_genesis` (+ law in
  `🧪️tests/🔬️unit`, `FixtureComponentCodec::genesis` in `🔄️sync/🧪️tests/🔬️document-socket-connect`); `🔌️plugin/🖥️host/🧬️component-codec`
  — `genesis`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `open_document` opens on the component genesis, `pump_sync_events`
  refreshes guest bodies only on document changes; `🐚️Shell/🧪️tests/🔗️hub-projection-workspace` — `frame_pump`, `hub_verb`
  via `drive`, step 12 ingest window and relative freeze bound.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — the React reserved drive reads the kernel constant.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs` — `ParallelRuntime::take_shard_failure`.
- `🏃️run/🦀️.rs` — resumes a preempted actor.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `run_turn` settle rules, preemption resume, quiescence,
  `carries_nothing`, `absorb` keeps non-idle ingress, typed-operation pages on `ExchangeOutcome::typed_results` + in-settle
  ACKs (dead exchange/request/ACK plumbing deleted), `create_app(.., artifact_schema)` + component codec registration;
  G7w's `[DEBUG] g7w` logging removed; §1.4: `ReservedToolJob`, `settle_turn`, `settle_reserved_jobs`, `admit_reserved_jobs`,
  `run_reserved_job_once`, `dispatch_turn` (grant/owed tally, every turn result applied), reserved spawns out of `effects`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — `kernel_surface_id`, `create_app` passes the
  app's `io.artifact_schema`.
- `🎠️kernel/🦀️.rs` (`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`) — transport session 0 admitted;
  law in `🧪️tests/🔬️ui-turn-patch`.
- `📡️spr/🧵️channel/🦀️.rs` — exact `decode_app_frame`; law in `🧪️tests/🔬️unit`.
- `🏪️store/🦀️.rs` — `ComponentDocumentCodec`, `DocumentKindCodec`, `register_component_document_codec`,
  `document_kind_codec`; law in `🧪️tests/🔬️unit`.
- `🏪️store/🔄️sync/🦀️.rs` — `document_pack_schema_hash` asks `document_kind_codec`; native actor connect/finish/bootstrap
  identity + bootstrap validation + folder mirror persistence use it; fixture `🏪️store/🧫️fixtures/document-socket-connect-v1`
  (`componentIdentity`) + runner law.
- `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` (new) + mount in `🔌️plugin/🖥️host/🦀️.rs` — `OwnedComponentDocumentCodec`,
  `GUEST_CODEC_BUDGET`.
- `📇️directory/🔌️client/🌱️space-artifact-creation/🦀️.rs` (new) + mount in `📇️directory/🔌️client/🦀️.rs`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` — creation door state, texts, UI; laws in
  `🔗️HubConnection/🧪️tests/🔬️wgpu-unit`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — door verbs, catalog load, frame-pumped creation
  operation, opening through the relay.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — native journey laws (edit, undo, redo,
  select-all/copy/paste), live door law, two-user law creates through the door and undoes with the fixture's verb.
- `📺️renderer/🧑‍🎨engine/🧫️fixtures/⏯️native-guest-journey/🔣️.json` (new; `redo`, `clipboard`, expected ledgers added in §1.4).
- Tests adjusted: `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs` (create-request
  shutdown law, `typed_results`, budget rows), `🗞️typed-result-page/🦀️.rs` (page bound law moved in),
  `🧪️tests/🔬️wgpu-renderer-kernel-runtime-typed-operation-result-exchange/` (deleted with the exchange),
  `🧱️elements/🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs`; fixture row removed from
  `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}` — verbs
  `native-guest-journey-check`, `hub-live-creation-check`; `/Users/ueli/Documents/semio/.vscode/{launch.json,🧩️launch.seed.jsonc}`
  rows `⚖️gate🔐️hub-auth🧊️wgpu-live-creation`, `⚖️gate🧊️wgpu⏯️native-guest-journey`.
- `native-guest-journey-check` (renderer package `📜️script.ts`) runs the four journey laws.
- Ticket-local: `wp-wg8/{run-native-journey.sh,run-door-live.sh,block-release.sh,wasm-checks.sh,wasm-checks-2.sh,edit-*.py}`
  (`edit-debug-*.py` apply/revert the temporary `[DEBUG] wg8` timing; reverted, 0 lines left).

## Processes

- `75765` journey run 1, `24132`/`33624`/`44282`/`52569`/`62155` journey runs 2–6 (each exits by itself).
- `87049` block2d `materialize-release` (exited 01:55, rc 0; component sha256 `7d4bb1ed…`, native runtime republished).
- `44189`, `73318`, `9716` wasm32 checks (all exited green). `2629`, `3335`, `6859`, `9609` journey runs 11–14 (exited).
- Journey runs 15–24 (each exited by itself). Gate runs 5, 7, 8, 12, 13, 14, 16, 17 hung after their ledger (test harness
  `block_on`, §3); stopped by pid 17:0x (zsh + cargo + test binary of each). Run 18 exited by itself. My block
  `materialize-release` mutex waiter (11:52) was cancelled after the descriptor turned out rematerialized at 12:18.
- `wasm-checks-2.sh` queued in the wasm mutex (17:17, detached).
- No hub or serve started by WG8 (the live door law used W2's hub 7800 as a client).

## Log

- 00:5x start; read preambles, audit §3/§6, G7w, N2 §6.4, TC3b, WG7 skeleton.
- 01:05 guest request `wp-w1/requests/wg8.txt` (block in the full catalog + post-H9 release component).
- 01:2x journey run 1 → owned-ABI refusal (pre-H9 guest). 01:36 block2d rebuild launched (coordinator allowed).
- 01:4x–02:0x B1 + B2 source landed, native checks green.
- 05:2x resumed after the outage/usage cut; journey run 10 (spinning 83 min, coordinator notice) sampled and killed (my pids).
- 05:3x–05:4x typed ACK in settle, absorb ingress fix, frame decode, laws run, `[DEBUG] g7w` removed, B1 edit law green,
  live creation door green on hub 7800.
- 06:0x coordinator notice on note's `dist/component-dev` rebuilt outside the mutex at 05:39: **not WG8** — WG8's only guest
  build is block `materialize-release` through the wasm mutex (01:36–01:55, `generated/block-release-1.txt`); WG8's journey
  runs build only the renderer test binary. The native runtime needs the block **release** component (the 88 MB dev one
  exceeds the 64 MiB execution-target bound); WG8 republishes it from W2's `component-release` once W2's final pass lands it.
- 10:2x–17:2x gate: W2 catalog B publish failed twice (10:39 codec probe, 11:11 space-creation JSON), published 11:44, 7800
  ready 11:50, killed by the 12:19 low-disk cleanup, back 12:45 (runId `345ceda4…`); gate runs 1–18, five fixes, run 18 green.
- 06:4x–07:4x §1.4: shard reports admitted job turns, renderer drives reserved jobs (runs 15–16 red: seed checkpoint
  overflow found through the new retained-failure report), live-only reserved seeds + kernel constant, redo and
  select-all/copy/paste laws; runs 17–21 all green; 2 latency outliers timed, not recurring.

## 6. Next (for the coordinator)

1. **Done: the gate is green on catalog B** (§3). Was: **Catalog with block, early.** Ask W2 to publish a catalog that carries block as soon as block's `component-release`
   is built (e.g. `--packages stdio,gis,note,draw,writer,puzzle,block` into a copy served on WG8's hub 8090, or on
   7800) instead of after all 34. Then: `@semio-tech/block-plugin:materialize-release` through the wasm mutex (so the
   release descriptor matches W2's component) and `zsh .tmp-ticket/wp-wg8/run-collab-live.sh` (optionally
   `SEMIO_HUB_LIVE_ORIGIN=http://127.0.0.1:8090`). Expected: steps 1–12 measurable, 11 green since §1.4.
2. **§1.4 landed.** Left open by design: a product (non-reserved) spawn of a guest whose checkpoint outgrows the fixed
   replay checkpoint pages (block2d's does; other guests not measured) still closes its seed — the product-replay owners
   should size or page that checkpoint; the failure is now named in the fault instead of lost. Pre-existing: the renderer's product-replay
   laws share process-wide registries and abort in parallel (serially green); an unmounted stray copy of the shard unit
   tests lives at `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` (compiled by no crate).
3. `semio-framework` `ui_turn_patch_transport` laws fail when run in parallel (shared process-wide arena under
   `try_lock`); they pass with `--test-threads=1`. Pre-existing; worth a serial guard.
4. `os.create-space-artifact` (the Space-index guest's create dialog) on wgpu → the same door operation (needs the Rust
   twin of React's kind-choice encoding).
5. `wp-wg8/target` is kept (coordinator); it is no longer needed for the gate.
6. **WG7 / React parity (not measured by WG8):** a fresh door artifact answers the document socket `Bootstrap::None`, so any
   host whose guest opens without the document's genesis authors under the wrong document id. Worth one check on the wasm32
   wgpu shell (WG7) — `open_document` is target-neutral, so it now seeds there too once a component codec is registered —
   and on React. Presence is timer-driven in React already. The native `os.open-artifact`
   open inside the frame pump (~20 s in debug) wants a retained operation (§3).

