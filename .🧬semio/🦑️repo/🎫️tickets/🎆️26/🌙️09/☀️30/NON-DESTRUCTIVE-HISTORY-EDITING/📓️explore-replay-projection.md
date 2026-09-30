# Explore: materialization, projection, replay, jobs, ABI and state lanes for non-destructive history editing

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit, 2026-09-30. No source edited, nothing compiled or run, no git write. Every statement is from reading the source at the cited place. The runtime files churn under concurrent edits (`S` and `P` shifted by 5-15 lines while I read), so anchor on symbol names; line numbers are "as read".

Peer reports on the same ticket (read, not repeated here): `📓️explore-event-sourcing-core.md` (mutation contract, fold, `ingest_remote`, hooks H1-H10), `📓️explore-alternatives-vcs.md` (alternatives, hub, `Supersede`), `📓️explore-tools-transactions.md` (tools, `ToolRun`), `📓️explore-puzzle2d.md`. This report covers what they do not: how the running guest materializes and renders state, the bounded/retained regime every live write lives in, the job/progress/cancel machinery, the host-guest boundary for dry-run evaluation, the state-lane placement of a session, and a concrete replay-engine design.

## Path aliases

| Alias | Path |
|---|---|
| `FW` | `🧰️framework/🔨️modules` |
| `OS` | `🧰️framework/🛍️products/💻️os/🔨️modules` |
| `S` | `OS/🏪️store/🦀️.rs` (`ArtifactStore`, ~23.6k lines) |
| `V` | `OS/🌿️vcs/🦀️.rs` (ledger, `ArtifactVcs`, `apply_mutation`) |
| `P` | `OS/🔌️plugin/🦀️.rs` (`VcsArtifactApp`, ~45k lines) |
| `PT` | `OS/🔌️plugin/⏯️tool-run/🦀️.rs` (`ToolRunLedger` driver) |
| `OP` | `OS/🔌️plugin/⏳️operation-progress/🦀️.rs` |
| `RJ` | `OS/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs` (cold job kinds `semio.*`) |
| `WIT` | `OS/🔌️plugin/🧬️schema/📜️.wit` |
| `CH` | `OS/📡️spr/🧵️channel/🦀️.rs` (`AppCommand` / `AppFrame`) |
| `CMD` | `OS/📡️spr/🎮️command/🦀️.rs` |
| `SPRM` | `OS/📡️spr/💎️materialize/🦀️.rs` |
| `INF` | `OS/💡️inference/🦀️.rs` |
| `DB` | `OS/🛢️db` (hub-side artifact engine) |
| `SYNC` | `OS/🏪️store/🔄️sync/🦀️.rs` |
| `MUT` | `FW/📡️replication/🎮️mutation/🦀️.rs` |
| `TRANS` | `FW/📡️replication/🔗️causal/🔀️transition/🦀️.rs` |
| `J` | `FW/🧵️job/🦀️.rs` |
| `AB` | `FW/🎯️action-bus/🦀️.rs` |
| `PRT` | `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` |
| `SH` | `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` |
| `TRC` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` |

## 0. Verdict in ten lines

1. The guest holds exactly one materialized position: the head, `ArtifactStore.current: Arc<P>` (`S:15651`), plus one pre-tail snapshot (`tail_undo_cache`, `S:15659`). Every other position is a fold from `initial_snapshot` (`fold_history` `S:19672`, `materialize_document_snapshot` `S:11440`): O(total ops), no checkpoints, all `MutationMessage`s discarded.
2. Three replay shapes exist: the silent fold, the atomic local apply (`replay_mutations` `S:18473`) and the merge replay (`replay_suffix_partitioned` `S:17038`, used only by `ingest_remote`/`resolve_conflict`). Only the last recomputes diff, inverse and messages of a suffix. It is synchronous, clones the whole state per edit, has no cancel and no progress.
3. There is no read-only historic view. Nearest existing seams: the `ToolRunLedger` overlay read at four render sites (`P:34168/34197/34234/34265`), the cold JSON `snapshot_override_json` render parameter, the hub-side `db_projection::state_at` / `preview_augmented`, and the unwired `.spr` `resolve_plan` + `REC_PROJECTION` snapshots.
4. "Revert to here" is repeated `Undo`: n-k `Revert` transitions appended forever, each one refolding the whole event log and (after the first) folding the document from genesis. It must not be the scrubbing mechanism.
5. The live app runs in a bounded/retained regime: writes go through app-owned one-item preparation factories, per-turn grants, fixed-capacity slots, cold retirement of every scratch root, a 64-edit ledger, and a publication that only ever appends at the tail. `replay_suffix_partitioned` and `dispatch` are the unbounded generic path, not the live one.
6. The job system already offers cancellable, budgeted, progress-reporting work with a stale-result guard, a host-driven abort, a latest-wins lane and a localized accessible progress panel. A replay job must be classified `Migrated` or its verb is dead (`interactive-job.not-ui-safe`).
7. No base-parametric "evaluate this mutation/command against snapshot X" exists across the boundary. `preview()`, `preview_wire`, the agent-lane preview and `transaction_prepare` all fold over the head; `AppCommand::PureCommand` takes packs but hydrates (overwrites) the live instance's lanes; `codec.apply-ops` is pure but whole-pair and returns no messages. Inside one guest instance no ABI change is needed, because roots are `Arc<Snapshot>`.
8. The four state classes map cleanly, and the earlier tool-run decision (TRC section 1.1) already rejected the DraftStore lane. A time-travel session belongs in a framework-owned ephemeral local-only ledger next to `tool_runs` (`P:23509`); only the finalized amendment becomes a persisted-shared event.
9. Inference is pure and content-addressed (`DepHash`), so it is snapshot-agnostic, but the cache and session have no production caller and the projection stamps are used by tests only.
10. Recommendation: session ledger + prefix-snapshot ring + resumable replay stepper extracted from `replay_suffix_partitioned` (keep-and-record semantics, no per-edit clone) + latest-wins replay job + convergence early exit + atomic finalize modeled on `publish_tool_run`, tests first.

## 1. How the running app holds, updates and derives document state

### 1.1 Lanes held by one app instance (`VcsArtifactApp`, `P:23215`)

| StateClass (`FW/📡️replication/🧾️wire/🦀️.rs:294`) | Field | Shape | History | Persistence / sharing |
|---|---|---|---|---|
| Artifact (persisted shared) | `store: ArtifactStore<Snapshot, Mutation>`; owned children in `children` | event-sourced | 64-edit ledger + unbounded `transitions` (`S:2698`) | `PersistenceBinding::Folder` -> `PersistedLocalOnly`, `Hub` -> `PersistedShared` (`SYNC:90-170`); backbone `Mutations` batches |
| Config (persisted local-only) | `config_store` (`P:23218`), `window_config_store` (`P:23249`), `interaction_store` (selection, `HistoryLane::Interaction`) | event-sourced `ArtifactStore` alias | same 64-slot ledger, hence coalescing is mandatory | `LoadConfig/ReadConfig`, `LoadWindowConfig/ReadWindowConfigs` (`CH`), never shared |
| Draft ("ephemeral artifact") | `draft_store` (`P:23221`), `DraftStore` alias (`S:3965`) | event-sourced, volatile | ledger | no backbone attached at construction (`P:24285`); `PruneDrafts` returns `Err(not implemented)` in `dispatch_inner`; all 150 editor impls declare `type Draft = NoDraft`; no render path reads it (TRC 0.2) |
| Presence (ephemeral shared) | `presence_store` (`P:23225`), `peer_presence` | last-writer-wins roster | none | presence/preview frames, `wire_lane_data_class` -> `EphemeralShared` (`SYNC:125`) |
| Transient (ephemeral local-only) | `transient_store` (`P:23248`), `window_transient_store` (`P:23250`) | one `Arc` root | none | none |
| Tool run (framework-owned, ephemeral local-only) | `tool_runs: ToolRunLedger<A>` (`P:23509`) | provisional ops + overlay | none | dies with the page; only a `PresenceToolRun` summary is shared (`P:13968`) |

`PersistenceDataClass { PersistedLocalOnly, PersistedShared, EphemeralLocalOnly, EphemeralShared }` (`SYNC:90`) is the wire/binding form of the four AGENTS.md categories. Empty bindings resolve to `EphemeralLocalOnly` ("in-memory draft / studio with no backbone", `SYNC:135`); the fixture `OS/🏪️store/🧫️fixtures/persistence-data-class-v1` names the promotion commands (`promote-to-hub-space`, `persist-locally`).

### 1.2 The store's materialization (`ArtifactStore`, `S:15605`)

- `current: Arc<P>` (`S:15651`): the raw fold of `initial_snapshot` over every `forwards` op of `applied_edit_ids`, maintained incrementally so `snapshot()`/`Apply`/`AmendLast` are O(new work). Cold paths (checkout, switch, `set_state`, load) recompute with `fold_current` (`S:17016`).
- `tail_undo_cache: Option<(edit_id, Arc<P>)>` (`S:15659`): the snapshot right before the current tail edit. Refreshed by `apply_command`, `amend_command` (fresh branch), redo. It powers an O(1) undo of exactly the tail. `take_tail_snapshot_for_current` (`S:16645`) consumes it, so a second consecutive undo has no cache.
- `revision_accumulator` (`S:13763`): per applied position `CursorRevisionRecord{id_digest, edit_digest, prefix_digest}`, a prefix-authenticated hash chain. `edit_digest` (`S:13860`) hashes forwards, **inverse** and meta. A rebased inverse therefore changes the revision of every later prefix.
- `generation`, `content_revision`, `artifact_revision()` (`S:16016`), `ArtifactRevision`/`ArtifactProjectionStamp` (`S:3837/3847`), `SnapshotRead<T>` leases (`S:347`, registry of 1,024 slots at `S:83`) with `commit_authority_matches(generation, revision)`: a lease notices any commit in O(1) and fails closed.
- `reproject()` (`S:18020`) is the single convergence path for local and remote history transitions: `fold_envelope_history` (`S:19631`, walks **all** edits and **all** transitions) -> `adopt_history_facts` (`S:18065`) -> only if the applied id list changed, `project_applied` (`S:18042`) with three modes: tail undo O(1) through the cache, exactly one edit appended at the tail O(edit), otherwise a full fold from genesis. It records no messages.

### 1.3 How a mutation is applied

| Path | Where | What it does | Bounded / cancellable |
|---|---|---|---|
| Pure core | `os_vcs::apply_mutation` (`V:1147`) | `operation.diff(&P).into_parts()`, `diff.apply(&P)`, `MutationDiff::retire_cold(diff)`; returns `(P', Vec<MutationMessage>)`. `apply` returns a **new** `P` (`MUT:103`); there is no in-place apply | pure |
| Local apply | `apply_command` (`S:18286`) -> `replay_mutations` (`S:18473`) | per op: `inverse(&running)` reversed, HLC tick, mint id/meta (label, semantic_kind, group_id always defaulted), `diff(&running).stamp_op_index`, `apply`, retire the displaced root. Atomic vs the local `MergePolicy` (`Rejected` before any store field changes). Then edit into the ledger, `record_edit_messages`, tail cache, `reproject`, `bump` | no (sync) |
| Coalesced | `amend_command` (`S:18335`) | extends the tail edit's forwards/inverse/meta (same key, uncommitted) | no |
| Merge replay | `ingest_remote` (`S:18835`) -> `replay_suffix_partitioned` (`S:17038`) | fold `order[..k]` for the base (`fold_history`), clone every edit into `edits_by_id` (`S:~18959`), then per edit: `candidate = state.clone()`, per op `diff`/`inverse`/`apply`, per-edit accept-or-quarantine by policy; an `apply` error aborts the whole replay | no |
| **Live bounded path** | `begin_apply_batch` (`S:17243`) -> `advance_apply_batch` (`S:17501`) -> `fold_batch_item` (`S:17747`) | phases `Preparing` (one app-prepared item per grant; base for item i+1 is item i's post root through a lease, `batch_item_base` `S:17470`) -> `PreparingCursor` -> `PreflightingCommit` -> `Publishing` (one atomic swap, appends at the tail or amends a coalesced tail, `batch_amend_target` `S:17484`) -> `AwaitingAck`. `cancel_apply_batch` (`S:17710`) sets `cancel_requested` | yes: grants, checkpoints, generation+revision freshness guard |

Glossary of the terms the ticket asked about:

- **Diff application / inverse.** `Mutation::diff(&self, &P) -> MutationOutcome<Diff>` and `Mutation::inverse(&self, &P) -> Vec<Self>` (`MUT:174-182`). The inverse is state-dependent, so a stored `Edit.inverse` is stale as soon as anything before it changes. `ingest_remote` rewrites it from `rebased_inverse` in its write-back block (`S:~18990-19075`); `Edit.forwards` are never rewritten.
- **Scratch projections.** Every intermediate `P` a fold displaces must go through `MutationDiff::retire_projection` (`retire_replayed_projection` `S:19538`, the self-retiring `ReplayProjection` guard `S:19555`, `retire_scratch_edits/operations`). A bare drop of a fail-closed root (an `OrderedMap`, a neural `Dictionary`) aborts the guest. Any replay engine must hold displaced roots in an "owed to retirement" list, as `ToolRunEntry.displaced` does (`PT:399-440`).
- **Retained rows.** The footprint counts staged edit ROWS, forwards plus inverse (`ArtifactStoreOneItemFootprint` `S:14691`, `for_one_invertible_item` = 2 rows `S:14698`, hard caps 65,536 rows and 1 MiB per gesture `S:14671-14672`). A mutation whose inverse expands to several rows fails `fold_batch_item`; the fix is to emit point-invertible rows. A replayed edit's inverse is recomputed against a shifted state and can expand: the engine must turn that into an outcome, not a crash.
- **Reproject.** Section 1.2.

### 1.4 Rendering the state

- The render input is `AppProjectionCache = ((store_gen, config_gen, log_gen, filter), Arc<Snapshot>, Arc<Config>, Arc<HistoryView>)` (`P:22824`), refreshed by `refresh_cache` (`P:26948`) when any key changes. `HistoryView` (`P:12046`) is rebuilt per generation; rows carry `edit_id`, a bounded op preview (last 8 ops), `applied`, `revertible`. The host gets `HistoryPatch`/`HistoryEntry` (`FW/🎠️kernel/🦀️.rs:1795/1833`); a row has **no severity or messages** field.
- `render` (`P:34124`) builds `ArtifactView::with_render_context(tool_runs.overlay_or(snapshot), history, ...)` at four seams: render `P:34168`, `window_engagements` `P:34197`, measures `P:34234`, labels/menu `P:34265`. `ToolRunLedger::overlay_or` (`PT:839`) returns the mutating run's overlay while it holds provisional state, else the committed root. **This is the existing, performant "render a non-head document" seam**; `ArtifactView.with_tool_run(...)` gives the app the provisional entity set for styling.
- Commands are reduced against `TypedCommandRoots.snapshot` (`P:16036`, built by `capture_typed_command_roots` `P:31161`), which is the committed cache, not the overlay (TRC 2.7).
- `render(body_key, snapshot_override_json, view_state)` (`P:14249/34124/41214`) accepts a JSON snapshot: it decodes the whole document on every call, rebuilds an empty `HistoryView`, reads config/transient live. A cold escape hatch, not a viewer.

### 1.5 Inference and dependency-aware caching

- Contract: `Inference<P>::infer(&P)` is pure, deterministic and total (`CMD:33`); `InferenceSpec` carries id/version/field reads (`CMD:99`); `DiffRegions::touches()` is the tier-1 write-region gate (`CMD:84`, adopted per type, not universal).
- Engine (`INF`): `DepHash` merkle chain per entity (`INF:19`), `InferredField {plan, dep_input, compute}` (`INF:83`), byte-budget LRU `InferenceCache` (`INF:148`, **disabled by default**, `touch` is an O(n) `VecDeque::position` scan, `get` clones the bytes), `infer_field` (`INF:291`), `infer_field_after_diff` (`INF:326`) whose session gate remembers one previous result per field. Keys are content hashes, so entries are valid for any snapshot: head, candidate overlay, replay intermediate.
- Live wiring: `ArtifactInferrer::infer_cached` (`P:1439`) defaults to a passthrough. `InferenceCache::new` and `InferenceSession::new` have **no non-test caller** in `FW`, `OS`, `✏️s` or `🌎️hub` (grep 2026-09-30). `accept_projection_result`, `projection_event`, `invalidate_after_replay` (`S:16091/16054/16066`) are called by tests only.
- What does run live: the wire inference service (`P:1545-1600`, `wire_artifact_infer` `P:2114`) takes the document as `canonical_payload` plus `previous_state`, `requested_cache_mode` (Rebuild / ReusePrevious / ValidatePrevious, the twin of `ArtifactProjectionCacheMode` `S:3870`), `budgets`, `cancellation_id`, and runs as the `semio.infer` cold job through a `MountedWorkerJobSession` (`RJ` `💡️infer/🦀️.rs`: previews coalesce, checkpoints lossless, diagnostics in a bounded ring, `WORK_UNITS_PUMP = 64`). Apps otherwise compute derived data inline while rendering.
- Hub read models (`DB/📽️projection/🦀️.rs`): `ProjectionEngine::state_at(projection, frontier)` and `preview_augmented(projection, base_frontier, preview_envelope)`, query modes `Consistency::{Historical, Speculative, PreviewAugmented}` (`DB/🔍️query`), an ephemeral `PreviewStore` with `Active -> Superseded|Withdrawn|Committed|Rejected|Expired` and rebase-or-stale reconciliation (`DB/🔮️preview/🦀️.rs`). It is the CQRS template for "historic base plus candidate that never enters the WAL", but at the hub, over opaque bytes, and not connected to the guest render.

## 2. A snapshot at an arbitrary history position

### 2.1 Inventory of every existing "state as of X"

| Site | Base and target | Cost | Messages | Cancel/progress | Callers |
|---|---|---|---|---|---|
| `current` / `tail_undo_cache` | head / before tail | O(1) | - | - | render, undo of the tail |
| `fold_history` (`S:19672`), `materialize_document_snapshot` (`S:11440`) | genesis -> any `applied_edit_ids` list | O(M), one fresh `P` per op, linear `edits.iter().find` per edit | dropped | none | `project_applied` fallback, `ingest_remote` base, `pack_at_checkpoint` (`S:20963`), host `materialize_backbone_snapshot` (`OS/🖥️host/🦀️.rs:526`, only ever called with `&[]`) |
| `replay_suffix_partitioned` (`S:17038`) | caller-supplied `&P` -> suffix | O(M_suffix) + one `P::clone` per edit | kept per edit | none | `ingest_remote`, `resolve_conflict` |
| `.spr` `resolve_plan` / `materialize_with` (`SPRM:398/477`) | `MaterializeTarget::{LatestOnActive, AtCheckpoint, AtEditOrdinal}`; base = newest valid `REC_PROJECTION` at or before the target, tail replayed; `CheckpointPolicy{every_edits 512, every_bytes 4 MiB, on_checkpoint_commit}` (`SPRM:177`) | O(tail) | - | - | tests and CLI only; the append-mode reopen (`open_append`) re-appends through `HistoryLog`, which has no slot for `REC_PROJECTION`/`REC_INDEX`, so it drops them (`OS/📡️spr/🔌️io/🦀️.rs:98-108`); the ordinal is file order, not applied (HLC) order |
| `db_projection::state_at` / `preview_augmented` | hub, per frontier | index lookup | - | - | hub queries |
| `ToolRunLedger` overlay (`PT:839`) | committed head + provisional ops | O(k) append, bounded refold | dropped (`conflicts` counter only) | yes | four render seams |
| `snapshot_override_json` | any JSON document | full decode per call | - | - | tests, legacy |

The `.spr` snapshot design is complete and unused: the persisted side never gets a checkpoint cache from the store.

### 2.2 Cost model

- n <= 64 edits per store (`ARTIFACT_HISTORY_LEDGER_CAPACITY = 64`, `V:196`; `reserve_slot` `V:344-356`; a 65th edit fails and nothing compacts; every other ledger is 64 too). What is unbounded is the number of **ops** M: a coalesced drag stores one op per tick in one edit (`amend_command` extends `forwards`).
- One op costs one `diff` plus one `apply` that builds a new `P`. How much a new `P` shares with the old one is per-type (the store has bounded retained-clone owners, `OS/🏪️store/🧬️retained-clone`). I did not measure any type (see section 7). `replay_suffix_partitioned` additionally pays a full `P::clone` per edit (`S:17057`), and `ingest_remote` clones every edit into `edits_by_id`.
- The guest has one fixed linear memory whose allocator never returns pages and a 64 KiB contiguous-request ceiling (project notes), and the displaced-owner queue has 1,024 slots (`S:1760`, pressure drain at a quarter full). Unbounded scratch is therefore a correctness risk, not just a speed risk.

### 2.3 How revert and undo execute today

- `Undo` -> `undo_with_policy` (`S:18210`) -> `undo_lane_position` (`S:18238`) -> `commit_transition(Revert{mutation_ids})` (`S:18000`) -> `reproject` -> `project_applied`. The first tail undo is O(1); a mid-history undo or any second consecutive undo folds from genesis.
- `revertToCommand` (reserved job route 6, `framework_reserved_job!` `P:17877`) runs `FrameworkReservedCommitStage::Revert` (`P:19402`): `framework_revert_unit` (`P:29260`) dispatches one `Undo` per unit until the target edit is the tail, with `FrameworkReservedCommitProgress{operation, applied, total}` (`P:19420`). Cost: n-k appended transitions that never shrink, each `reproject` walks the whole event log, all but the first undo do a genesis fold: O((n-k)·(n+T) + (n-k)·M). Redo re-applies forwards.
- `CheckoutCheckpoint` is a `Checkout` transition that moves the shared head for every replica (peer report 2.2).
- Conclusion: scrubbing or "view as of #k" cannot be built on undo or checkout. It must be a non-mutating overlay.

## 3. The job system and how a replay could run as a cancellable job

### 3.1 Primitives (all verified in `J`, `AB`, `P`)

- `InteractiveJob { step(&mut StepContext) -> StepOutcome, begin_close, close_step, terminal_is_empty }` (`J:1307`). `StepOutcome = Yield | PreviewReady(payload) | CheckpointReady(Checkpoint) | Complete(CommitCandidate) | Cancelled | Fault` (`J:1241`). Every job releases its owners through a bounded close protocol, never a plain drop.
- Budgets: `StepBudget{fuel, deadline_us}` (`J:377`), lanes Interactive 1 ms / 2 M fuel, UserVisible 2 ms / 6 M, Background 4 ms / 20 M, Maintenance 4 ms / 80 M (`J:394-401`), hard ceiling 8 ms (`ToolExecutionContract::INTERACTIVE_MAX_STEP_MICROS`, `AB`). `StepContext` exposes `should_yield`, `is_cancelled`, `consume_fuel`, `set_stage`, `next_preview_sequence` (`J:990`); `drive_step` (`J:1330`) pre-checks cancellation, runs under a watchdog and records the trace stage.
- Identity and staleness: `Operation{operation, base_revision, generation, preview_sequence, seed}` and `validate_commit` (`J:121/165`): "a stale candidate must be rebased or discarded, never silently applied". `JobScope::child_of(token)` (`J:1558`, 64 child slots) gives parent-to-child cancellation.
- Progress vocabulary: `ProgressEvent` with ten variants (`J:1825`: `Started`, `StageChanged`, `CandidateTested`, `PreviewPatch{completed_units, total_units, ...}`, `Diagnostic`, `Checkpoint`, `CommitCandidate`, `Completed`, `Cancelled`, `Failed`) and channel policies (`J:1945-1990`): preview coalesced (64 items / 4 MiB), commit and checkpoint lossless (256 / 16 MiB), diagnostics ring (128 / 512 KiB). Payload pages are 16 KiB, 256 pages per operation (`J:405-442`).
- Command jobs: `ToolExecutionContract::resumable(max_raw_wire_bytes, max_decoded_items, max_work_units_per_step, max_output_bytes, max_step_micros, checkpoint_every_steps, progress_every_steps)` with `PerOperation` cancellation and `ValidateImmediatelyBeforeExposure` (`AB:216-260`); `ToolJobFactory` (`AB:332`); `ArtifactOwnedToolJobFactory` with `latest_wins_target(command)` (`P:15191/15197`); publication lanes `HostOnly, Artifact, Config, Draft, Presence, Transient, WindowConfig, WindowTransient, Child, Interaction` (`P:15167`); result lanes add `Ui, Terminal, Fault, ...` (`P:15863`).
- Cold job kinds: `register_bounded_job_kind(kind, factory)` (`RJ:172`); `BoundedJob { step(JobBudget{fuel, deadline_ms}) -> Running(progress bytes)|Done|Failed, cancel, checkpoint }` (`RJ:126`); builtin kinds `semio.io-run|io-sniff|infer|mutation-plan|migrate` (`RJ:73-84`); a stall guard fails a job that reports no progress with an unchanged budget; `checkpoint()` bytes restore the job after a guest trap (WIT `checkpoint` interface).
- Latest-wins: a factory that returns a target key from `latest_wins_target` gets a coalescing lane; a newer command with the same key restarts the pending one (`PendingLatestWinsCommand`, `P:19727`, fields `restarting`, `lease`).

### 3.2 Scheduling on the guest

- One reactor turn calls `PluginApp::advance_typed_operation_publication` (`P:~33255`): reserved commits (revert), then the tool-run driver `drive_tool_run_turn` (`PT:1517`, 4 ms per turn `PT:24`), then the envelope-decode worker, the local interaction query, then typed-operation publication (256 units per turn, `P:15748`; worker slice at most 4 ms and 256 pumps per turn, `P:15743-15745`). Fixed slots: 64 live typed operations (`ARTIFACT_LIVE_OUTPUT_SLOTS`, `P:19153`).
- The wasm guest is one logical cooperative worker. Native hosts have a pool of threads with lanes Interactive/UserVisible/Background/Maintenance/Io/Timer and deficit-round-robin weights 8/4/2/1/4/3 (`FW/⏳️async/🦀️.rs:651-705`); `Lane::Interactive` steps run on the caller (`J:2235`). The web shell has `min(hardwareConcurrency-1, 4)` guest shards (`PRT:385`).

### 3.3 Host-side driving, progress and cancellation (web)

- Spawned jobs: `driveSpawnedJob` (`PRT:2888`) calls `startJob`/`stepJob(jobStepBudget)`/`cancelJob` on the spawning instance's actor under `serializeCommandIngressForActor`, in bounded batches with `yieldPluginUiContinuation` between them, so a job interleaves with other commands for that actor. Progress bytes reach the shell through `publishSpawnedJobProgress`, coalesced to one refresh per 120 ms per job (`PRT:2865-2887`), `subscribeSpawnedJobProgress` (`PRT:278`) and a full refresh in `SH:7323`. A cooperative job ledger feeds the task manager (`💼️job-ledger`, 250 ms publish; cancel = `cancelJob`, the guest releases the id and the next step answers `job.unknown`).
- Typed operations: unsolicited `AppFrame::Invocation`/`OperationCompleted` frames carry `UiDirtyScope`; `subscribeOperationProgress` (`PRT:275/4092`) feeds the shell's coalescing refresh lane (`SH:7315-7322`).
- Visible progress and cancel are guest-rendered UI: `ArtifactOperationProgress{operation_id, generation, label, completed_units, cancelling}` (`OP:7`), `operation_progress_controls` builds a `Liveness::Polite` text, a `progress` node and a Cancel button, localized en/de, keyboard accessible. Cancel is the reserved action `cancelTypedOperation{operationId, generation}` (16 hex digits each) -> `dispatch_operation_cancellation` validates instance, actor and generation, then `lease.cancel()`. A user cancel maps to the `Terminal` result lane, a worker fault to `Fault` (`cancellation_result_lane`). Only `completed_units` exists; there is no total (the tool-run `ToolRunProgress` has `total?`).
- Tool runs add host-driven reserved actions (`toolRunAbort`, ...) precisely so that an abort never queues behind the guest work it wants to stop (TRC 2.5).

### 3.4 Gates and traps that will bite a replay job

- `InteractiveJobClassification::BatchOnlyPendingRewrite` (and `Unclassified`) is hard-dead in the live app: `validate_ui_dispatch_classification` (`P:14394`) returns `interactive-job.not-ui-safe` before the handler runs. The replay verb must be `Migrated`, with a `ToolExecutionContract`, a publication contract and a proof entry.
- A typed result page is a fixed 4,096 bytes (`TYPED_OPERATION_RESULT_PAGE_BYTES`, `P:15650`). A per-edit outcome table for up to 64 edits with messages cannot ride one result page; use coalesced preview pages (16 KiB) or session-owned state the panel reads.
- Retained-wire command pages decode with `OpBinary::decode_op`; a hand-written validator faults at the first byte (project notes).
- Placement is declarative only. `JobPlacement` documents `Inline` = the instance's own turn budget, `Isolated` = own pooled actor, `Exclusive` = dedicated actor (`FW/🎠️kernel/🦀️.rs:720`), but the web runtime never reads `placement` (`PRT`) and the native async import `spawn-job` is a fault stub (`OS/🔌️plugin/🖥️host/🦀️.rs:2624`). Do not rely on it for parallelism.

### 3.5 Three ways to place the replay

| Option | Where it runs | Pros | Cons | Verdict |
|---|---|---|---|---|
| A. Session driver turn (the `ToolRunLedger` pattern) | same guest, bounded turn slice | shares `Arc` roots, no hydrate, no wire copy, retirement and overlay already solved | single cooperative worker, no parallel gain | phase 1 |
| B. `semio.history-replay` bounded job kind | same actor via `spawn-job`/`step-job` | checkpointable across a trap, task-manager row for free | ingress-serialized with other commands, progress only as bytes, same memory | optional wrapper around A |
| C. Throwaway second instance | another shard, hydrated from packs | real parallelism off the interactive actor | hydrate is O(pack); `PureCommand` hydration overwrites the live lanes, so only a spawned instance is safe; result is ops + diagnostics only | phase 2, only for very large `P` |

## 4. Host-guest boundary and dry-run evaluation

### 4.1 The WIT surface (`WIT`, `world actor` `:1445`)

- `reactor` (`:1088`): `poll(events, budget) -> turn-result{ui-patches, effects, presence, next-wake, status, fuel-used, ...}`, `stage-command-page`, `stage-cold-pair-page`. Everything is turn-based and budgeted (`budget{fuel, deadline-ms, max-effects, max-patch-bytes, max-frames}`); `turn-status::checkpoint-ready(job-checkpoint)` exists.
- `jobs` (`:1335`): `start-job`, `step-job(job, job-budget) -> running|done|failed`, `cancel-job`, `take-segmented-download-chunk`. `checkpoint` (`:1361`): `checkpoint()`/`restore(state)` (instances with document/config/draft packs, view state, ephemeral). `describe` (`:1376`). `codec` (`:1396`): pure `genesis`, `print-mirror`, `apply-ops(kind, pair, ops)`, `replay-envelopes(kind, pair, envelopes)`.
- `host-async` imports (`:1255`): storage, blob, http, artifact-read/write, `spawn-plugin-instance`, `emit`, ...; `spawn-job` here is a stub (3.4).
- Host-to-guest command channel (`CH:2501`): `Command`, `ArtifactCommand`, `ApplyEnvelopes`, `LoadDocument`/`ReadDocument`, `ReadHistory` (`:2593`, "complete history projection after resynchronization"), `PureCommand` (`:2570`), `TransactionPrepare/Commit/Rollback/Undo/Redo` (`:2601`), `SetMergePolicy`, `ResolveConflict`, `ReadConflicts`, and a retained-operation family with `PollDocumentArchiveLoad`/`CancelDocumentArchiveLoad` (`:2711/2716`), the wire precedent for poll+cancel of a long guest operation.

### 4.2 Evaluate-without-commit today

| Entry point | Base | Returns | Bounded / cancellable | Fit for history editing |
|---|---|---|---|---|
| `VcsArtifactApp::preview(&[Mutation])` (`P:27055`) -> `SpaceMember::preview_wire` (`S:20861`) | current head (`self.snapshot()`), ops threaded forward | `DispatchReport{policy, worst, messages}` with `op_index` stamps; never applies, never touches `dispatch_report` | sync, no | the closest pure evaluator, but base is fixed to head; also clones the whole head |
| `os_vcs::apply_mutation(&P, &Mutation)` (`V:1147`) | any `&P` | `(P', messages)` | pure | the primitive to build on |
| Agent-lane preview `preview_addressed_action` (`P:29604`) / `preview_typed_command_job` (`P:29721`) | `capture_typed_command_roots` = head | emitted ops + counts, lane carriage verdict (`agent_lane_carriage`); refuses lanes an agent transaction cannot carry | yes: wall 2 s (`AGENT_LANE_PREVIEW_WALL_US` `P:19429`), keyed cancellation lease | evaluates whole *commands*, publishes nothing; head-based |
| `transaction_prepare` (`P:14122`, impl `P:33618`) | private `running` fold over head, generation-checked commit | foreign steps, rejection fault | no | cross-artifact 2PC, not reusable |
| `AppCommand::PureCommand` (`CH:2570`, guest arm `P:43042`) | host-supplied document/config/draft packs, **hydrated into the live instance** (`hydrate_document_lane`) | `AppFrame::Emit{document_ops, config_ops, draft_ops, diagnostics, child_ops}` | no | host-authoritative evaluate; overwrites the live lanes, so only for a throwaway instance |
| WIT `codec.apply-ops` / `replay-envelopes` (`S:10830/10704`) | a `(pack, spr)` pair on a throwaway `ArtifactStore` | next pair only; `replay-envelopes` refuses a quarantined conflict | fuel-capped, whole-pair O(size) | hub materialization, not an interactive evaluator |
| Tool-run overlay fold `fold_one` (`PT:526`) | any `&Snapshot` | `Option<Snapshot>`; messages discarded, `MergePolicy::default()` rejects Error | bounded refold `refold_turn` (`PT:536`) | pattern to copy, not to reuse (drops outcomes) |

Gap: nothing evaluates a mutation or a command against a caller-chosen base and returns the full `MutationOutcome`/messages. Inside one guest instance the base is just an `Arc<Snapshot>`, so this is a small refactor, not a new wire verb.

### 4.3 Outcome vocabulary already in place

`MutationOutcome{diff, messages}` (`MUT:1066`) with the laws: `Fatal` => empty diff, `Error` => no change to the named target; `Severity` order Info < Warning < Error < Fatal; `MutationMessage{level, code, message, target, op_index}` (`MUT:953`); `worst_level` (`MUT:1042`); `MergePolicy` `LaissezFaire` rejects Fatal, `Normal` Error+, `Vigilant` Warning+ (`FW/📡️replication/🧾️wire/🦀️.rs:212`, local, never on the wire); `EditMessages`, `DispatchReport`, `MergeReport{accepted, insertion_index, replayed, worst, conflict}` (`FW/📡️replication/⚔️conflict/🦀️.rs:240/275/322`); durable per-edit ledger `edit_messages` (8,192 slots x 4,096 bytes, `S:14266-14269`) read by `messages_for_edit`; `MutationOutcome::apply_to` (`MUT:1138`) converts an apply failure into a Fatal message instead of an error. The history row projection (`HistoryEntry`) is the only place these do not reach yet.

## 5. State classes and where a time-travel session lives

### 5.1 Requirements mapped to lanes

| Datum | Class | Reason |
|---|---|---|
| Amendment history (which edit's inputs were superseded, by whom) | persisted shared | it is the event log; replicated, hub-materialized through `codec.replay-envelopes` |
| An alternative not yet shared, or a session kept across reload | persisted local-only | only as tiny coalesced inputs in the config lane, or as a local-only document (empty bindings, `promote-to-hub-space`); never the overlay |
| Session state: target edit, candidate inputs, prefix snapshots, overlay, replay cursor, outcome table | **ephemeral local-only** | zero trace on abort is the invariant (TRC 2.2 invariant 3); the overlay is a read model |
| "X is editing history at #k" awareness | ephemeral shared | the `PresenceToolRun` precedent rides the presence heartbeat (`P:13916`); preview lane is `EphemeralShared` (`SYNC:125`) |
| Which edit is selected in the scrub UI, panel expansion | ephemeral local-only, per window | window-transient lane (`P:23250`) |

### 5.2 Recommended location

A framework-owned `HistorySessionLedger<A>` field of `VcsArtifactApp` beside `tool_runs` (`P:23509`), with a domain-neutral pure lifecycle module beside `FW/⏯️tool-run` (reducer, identity, fixtures, TS twin). Rejected alternatives, with the evidence:

- **DraftStore** (`P:23221`): separate `Draft`/`DraftMutation` types cannot express "committed document plus provisional ops"; no render path reads it; it is a second full `ArtifactStore` minting edits (64-slot ledger) per batch; `PruneDrafts` is unimplemented; all 150 apps use `NoDraft` (TRC 0.2 and 1.1).
- **Config lane**: 64-slot ledger dies under per-tick amends (project note on coalescing); good only for a handful of coalesced input records.
- **App `Transient`**: typed by the app, one `Arc` root; it cannot own snapshot rings, retirement, or the overlay seam, and `WindowTransient` is per window while an overlay is per instance.
- **The tool run itself**: its ledger is a forward overlay (`base` = head, `provisional` = ops after it). A history session is a backward overlay (`base` = snapshot before edit k, candidate replaces k, downstream replayed later). Reuse the driver mechanics and pure pieces (bounded `refold_turn`, `displaced` retirement, identity guard, freeze guard, finalize pattern), not the type, so the single-responsibility of each ledger holds.

## 6. Recommendations

### R1. Session as a read model, event only at finalize

States (pure reducer, lifecycle-law fixture, modeled on `ToolRunMachine`): `editing(k)` -> `replaying` -> `settled{blocking?}` -> `finalizing` -> `finalized | aborted | faulted`; `settled -> editing(j)` repeats. In `editing(k)` the overlay is `fold(prefix_{k-1}, candidate_k)` with downstream **not** applied; in `settled` the table lists every replayed edit. Identity `(sessionId, generation, baseRevision)`; a mismatch is a silent no-op `historySession.stale` (TRC 2.1). Freeze local artifact emits while a session is non-terminal (`freezes_local_emits` `PT:851`); remote ingests keep arriving, the base is refolded at finalize exactly like `watch_tool_run_generations`/`finalize_tool_run_turn` (`PT:1450/1774`). Generalize `overlay_or` into one `render_snapshot_or(committed)` used at the four render seams so a session and a tool run cannot both claim the overlay.

### R2. Prefix-snapshot ring in the store

- Keep `Arc<P>` checkpoints of `S_0..S_n` (n <= 64) keyed by a **forwards-only** prefix digest. Do not key by `CursorRevisionRecord.prefix_digest` (`S:13860`), because it also hashes the inverse and meta that a replay rewrites.
- Population is free: `apply_command` and `project_applied` mode 2 already hold `pre_snapshot`/`pre`; a full `fold_history` pass can emit stride checkpoints as a by-product. Editing the last edit is then O(1) (today's tail cache, generalized to a ring).
- Budget by retained bytes with an adaptive stride K, re-fold at most K-1 edits; retire displaced roots through the store retirement (`S:1760`), never a bare drop; read through `SnapshotRead` leases so a session notices a head commit in O(1).

### R3. Resumable replay stepper (extract from `replay_suffix_partitioned`)

`EditReplay<P, Mutation>` with cursor `(edit_index, op_index)`, `running: Option<P>`, per-edit accumulators, `step(deadline)`, `cancel`, `finish()`. Semantic decisions (policy-independent, identical on every replica):

- per op `diff(&running)` then `apply`; an `apply` error becomes a Fatal message (`MutationOutcome::apply_to`, `MUT:1138`) instead of aborting the replay;
- keep-and-record, no quarantine: Fatal => empty diff so state continues, Error => no change for the target. This also removes the per-edit `state.clone()` (`S:17057`), so per-edit cost is O(ops), not O(|P|);
- `inverse = op.inverse(&running)` reversed, staged as `rebased_inverse` and written only at finalize; account its rows against the footprint caps and turn an over-cap or multi-row inverse into an `Error` outcome;
- `EditOutcome{edit_id, status: Applied|Warning|Error|Fatal, worst, messages (<= 4 KiB, op_index stamped)}`;
- checkpoint state `{session, generation, prefix_digest(k), cursor}` well under 512 bytes (precedent `ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES = 512`, `OS/🔌️plugin/🧵️retained-command/🦀️.rs:10`);
- keep `replay_suffix_partitioned` as a drive-to-completion wrapper under a `MergePolicy` so `ingest_remote` laws stay green, removing the duplicated loop.

### R4. Convergence early exit

After each replayed edit j compare the running state with the previous `S_j`: `Arc::ptr_eq` first, then `PartialEq` (already required of every snapshot, `type Snapshot: Clone + PartialEq + ...` at `P:13423`). If equal, splice the previous `S_{j+1..n}` and outcomes and stop. An input tweak that a later edit overwrites, or that no later edit reads, then costs O(affected), not O(n-k). `Mutation::conflict_target()`/`DiffRegions::touches()` can act as a hint to try the equality check first, never as a reason to skip evaluation (`touches()` may over-approximate but must never under-approximate, and adoption is partial).

### R5. Scheduling, cancellation, progress

- Run the editing preview (candidate op plus inference) on the Interactive lane and the suffix replay on UserVisible or Background (`J:394-401`), so scrubbing never starves pointer input. Each turn is one 4 ms driver slice (`PT:24` pattern) or one bounded step.
- Slider/stepper inputs: `latest_wins_target = "historySession:<id>:<mutationId>"` (`P:15197`) so each tick restarts the in-flight job (`PendingLatestWinsCommand.restarting`).
- Cancel: a session-level `JobScope` (`J:1558`) with one child per replay; host-driven reserved abort (never queued behind guest work) plus the existing `cancelTypedOperation`; generation guard drops stale results (`validate_commit` `J:165`, and wire the unused `ArtifactProjectionStamp` freshness gate `accept_projection_result` `S:16091` with the session generation).
- Progress: `completed` = edits replayed, `total` = n-k. Add `total_units: Option<u64>` to `ArtifactOperationProgress` (`OP:7`). Stream per-edit outcome rows as coalesced `PreviewReady` pages (not through the 4 KiB result page) and refresh with `UiDirtyScope::Partial{window_bodies, panel_bodies}` rather than `Full`. Reuse `operation_progress_controls` for `Liveness::Polite`, en/de labels with no default locale, a real Cancel button, and severity chips that carry text, not colour alone.

### R6. Derived data

Wire one enabled `InferenceCache` plus a separate `InferenceSession` per snapshot lineage (head vs session) into the instance and route overlay renders through `infer_cached`; content-addressed `DepHash` keys then share entries between head, candidate and replay intermediates, so an overlay pays only for the changed dependency cones. Make `InferenceCache::touch` O(1) first. Long inferences stay `semio.infer` jobs with `cancellation_id`. Until wired, every scrub recomputes all derived data.

### R7. Finalize as one atomic publication

Model on `publish_tool_run` (`PT:1836`) and `advance_apply_batch` `Publishing` (`S:17628-17705`): freshness refold if `store.generation()` moved; then one swap that installs the amendment event, the rebased inverses, `replace_edit_messages` per edit (`S:18422`), `replace_current_retained(post)`, tail-cache reset, a rebuilt `revision_accumulator`, `bump()` and `snapshot_read_leases.publish_authority`. Constraints found:

- the ledger has 64 slots and forwards are immutable, so finalize must not mint n-k+1 replacement edits; use the peer reports' `Supersede`/`Amend` event plus an override map;
- the fold orders by the first op's HLC, not by ledger index (`TRANS:343`), so a replacement must keep the original edit's HLC position (or the fold must place it by reference);
- `content_addressed_checkpoint_id` hashes `Change.edit_ids` (`V:1210`), so amending an edit inside an already committed change forks the checkpoint chain: that is the "new alternative" case, consistent with the ticket;
- a hub materializes only what is expressible as causal envelopes: `replay_envelopes_onto_pair` refuses a quarantined conflict (`S:10704`). Keep the single fold implementation in `FW/📡️replication` so guest, hub and CLI agree (`codec.replay-envelopes`).
- Fatal outcomes make the session `settled{blocking}` and disable finalize; Warnings do not. Persist outcomes in the existing `edit_messages` ledger and add a severity/messages field to `HistoryEntry` (`FW/🎠️kernel/🦀️.rs:1795`) so history rows can show them.

### R8. Ordering of work (each slice testable alone)

1. `EditReplay` stepper + `EditOutcome` + wrapper refactor of `replay_suffix_partitioned` (store-level, sync tests, no UI).
2. Prefix ring in `ArtifactStore`.
3. Session ledger, lifecycle module, overlay seam, freeze guard.
4. Latest-wins replay job, progress/cancel UI, severity in `HistoryEntry`.
5. Finalize event + atomic publication (with the peer reports' event design).
6. Inference wiring.
7. Phase 2: throwaway-instance replay for very large `P`.

### R9. Tests first (AGENTS.md: language-agnostic plus an independent oracle)

- Replay equivalence law: incremental (prefix ring, stepper with arbitrary yield points, resume from checkpoint) == full fold, on JSON fixtures shared with an independent fold in another language (Python or TypeScript) for `(S_k, per-edit outcomes)`.
- Cancellation law: cancel at every step boundary leaves store generation, edit count, command log and outbox unchanged (TRC 2.2 invariant 3).
- Lifecycle-law fixture: every legal and illegal `(state, event)` pair, stale generations.
- Convergence law: early exit == no early exit.
- Budget law: every step within its lane wall; `terminal_is_empty` after close (scratch roots retired, none dropped).
- Classification law: Info/Warning/Error/Fatal vs the finalize gate, independent of `MergePolicy`.

## 7. Risks, open points, unverified

- **Not measured.** I did not compile, run or benchmark anything. Per-op `apply` cost and how much a new `P` shares with the old one is unknown per artifact; if some `P` deep-copies per op, replay is O(M·|P|) and the ring stride and a diff-based overlay matter more. Measure first with the typed-operation census (`P:15650-15738`) and the heap witness.
- **Wasm timing** of a 4 ms slice under fixed linear memory, and the guest's contiguous-allocation ceiling with a ring of up to 64 snapshots, are unverified.
- **Addressing.** Edits are multi-op (coalesced) locally but single-op on remote replicas (peer report 1.2); the session must address ops by `MutationId`, and which op of a 60-tick drag is "the mutation" is a product decision (puzzle 2d stores absolute final-position `MoveNode` ops, so the editable input is the last op's value).
- **`.spr` snapshot facts** come from doc comments and code reading of `SPRM`/`spr/🔌️io`; I did not run them. The claim "no production caller" for `InferenceCache`, `resolve_plan`, `accept_projection_result` is a repo grep on 2026-09-30.
- **Placement semantics** are read from doc comments and from the absence of any branch on `placement` in `PRT` and the native import; the shard runtime itself was not read.
- **Native/hub side.** How the Rust hub schedules replays (thread pool lanes) was not traced beyond `replay_envelopes_onto_pair`.
- **wgpu shell.** Whether the wgpu renderer shows the guest progress node is not verified (TRC 0.7 said it did not on 2026-09-13; later waves may have changed it).
