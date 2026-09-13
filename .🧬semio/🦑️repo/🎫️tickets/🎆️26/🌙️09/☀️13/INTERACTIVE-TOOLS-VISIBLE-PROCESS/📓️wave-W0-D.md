# ⏯️ Wave W0-D: tool run ledger and driver (OS plugin runtime)

Lane W0-D of `📋️tool-run-contract.md` (§2.7, §2.8, §3.3, §4.1 layers 1 and 3, §5, §6). Status: **landed, green.**

Abbreviations: `P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `R` = `🔌️plugin/⏯️tool-run/🦀️.rs` (new),
`S` = `🏪️store/🦀️.rs`. Logs are in `T/🗑️generated/W0-D/`.

## 1. What changed

### New files

| File | Content |
|---|---|
| `R` | `ToolRunLedger<A>`, `ToolRunDriver`, the driver turn (`impl VcsArtifactApp`), finalize publication, panel `ComponentTree`, presence and trace-delta API |
| `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` | 10 runtime laws over a toy app (`ToyRunApp`, registered manifest with two `run` tools, toy run and revalidate job) |
| `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json` | Language-agnostic expectations the laws read (counts, ids, chords, codes, generations, bench budget) |

### `P` (owned)

- `app` module mounts `tool_run` (`#[path = "⏯️tool-run/🦀️.rs"]`) and the test module, with explicit re-exports in `app` and at the crate root.
- `AppActionRegistry.tool_runs: HashMap<id, (LocalizedLabel, ToolRunDefinition)>`, filled from `AppDefinition.tools`/`utilities` with `run` (the manifest stays the source of record), retired in `close_step`, checked in `terminal_is_empty`; accessor `tool_run(id)`.
- `VcsArtifactApp.tool_runs: ToolRunLedger<A>`.
- `is_framework_reserved_action_id` includes the seven ids. `dispatch_action` routes them to `dispatch_tool_run_action` **before** every other branch: host-applied, never a spawned job, never queued behind the run. Kind stays `History` (W0-B's choice).
- `ArtifactView` gains `tool_run: Option<ToolRunView>` with `pub fn tool_run()`. `render`, `window_engagements`, `window_measures` and `tool_measures` pass `tool_runs.overlay_or(committed)` plus the view. Commands, `interaction_topology`, `pending_effects` and the `snapshot_override_json` branch keep reading committed, so provisional entities are never selectable.
- `render` serves `FRAMEWORK_TOOL_RUN_BODY_KEY` right after `FRAMEWORK_HISTORY_BODY_KEY`.
- `dispatch_emit` has a freeze guard (`toolRun.busy` Fault for artifact mutations under `rebase: freeze`). Window-config publications (in `dispatch_emit` and the typed-op receipt) call `note_window_config_published()`.
- `PluginApp`:
  - new default methods `tool_run_presence()` and `tool_run_trace_delta(cursor)`, implemented by `VcsArtifactApp`;
  - `advance_typed_operation_publication` runs one driver turn when work is pending;
  - `has_pending_typed_operations` and `has_runnable_typed_operations` include `tool_run_has_pending_work()`.
- Close: the ledger's close stage runs right after `pending_reserved`, before any store disposal; `close_terminal_is_empty` requires the ledger to be empty.
- `ArtifactApp::build_tool_run_job` (default `Ok(None)`), mirrored on `ArtifactEditor` and forwarded by `EditorApp<E>`.
- W0-B's one-line `tool_run_action_definitions` injection in the app builder is kept.

### `S` (minimal hooks, both unavoidable)

1. **Outbound batch publication.** `begin_apply_batch` rejects any store with a backbone attached, and production attaches backbones. So finalize could not publish at all, and there is no path to "one `Mutations` batch".
   - Added `pub fn begin_outbound_apply_batch(…)`.
   - Added `pub async fn flush_published_apply_batch(&mut self, &mut publication) -> Result<bool, VcsError>`.
   - Added a private `outbound: bool` on `ArtifactStoreBatchPublication`. `acknowledge()` refuses until the flush has run.
   - `begin_apply_batch_owned` and the preflight backbone check honour the flag. Existing callers pass `false`, so their behaviour is unchanged.
2. **Unique per-op mutation ids in a multi-item batch.** `fold_batch_item` now stamps each folded item's `mutation_id` as `"<edit id>#<forward index>"`.
   - Before, every item kept its factory id `"<prefix>-<seq>#0"`, so N items shared one id. The outbound DAG seed then failed with `duplicate mutation id`.
   - Single-item batches keep the same id as before.

## 2. Public API as landed (the code W1-B and W2 build on)

```rust
pub const FRAMEWORK_TOOL_RUN_BODY_KEY: &str = "framework.body.toolRun";
pub const TOOL_RUN_TURN_WALL_US: u64 = 4_000;             // one driver turn
pub const TOOL_RUN_TRACE_DELTA_BYTES: usize = 262_144;    // per refresh
pub enum ToolRunJobPurpose { Run, Revalidate }
pub type ToolRunJob = Box<dyn semio_framework_job::InteractiveJob>;
pub struct ToolRunJobRequest<'a, A: ArtifactApp> {
    pub tool_id: &'a str, pub definition: &'a ToolRunDefinition, pub purpose: ToolRunJobPurpose, pub identity: ToolRunIdentity,
    pub snapshot: Arc<A::Snapshot>,   // Run: the run's base; Revalidate: committed head
    pub config: Arc<A::Config>, pub checkpoint: Option<&'a [u8]>, pub provisional: &'a [A::Mutation],
}
pub struct ToolRunView { pub tool_id: String, pub identity: ToolRunIdentity, pub state: ToolRunState, pub provisional_entities: Arc<BTreeSet<u64>> }
pub fn is_tool_run_action_id(action: &str) -> bool;
pub struct ToolRunTickReceipt { pub stale: bool, pub appended: u32, pub retracted: u32, pub evicted: u32, pub capped: bool }
pub struct ToolRunDriver { pub turn_wall_us: u64, /* window-config publication counter */ }
pub enum ToolRunActionOutcome { Applied(ToolRunEffect), Rejected(ToolRunRejection) }

impl<A: ArtifactApp> ToolRunLedger<A> {   // Default
    pub fn state(&self) -> Option<ToolRunState>; pub fn slot(&self) -> Option<ToolRunSlot>; pub fn identity(&self) -> Option<ToolRunIdentity>;
    pub fn tool_id(&self) -> Option<&str>; pub fn provisional(&self) -> &[A::Mutation]; pub fn provisional_entities(&self) -> Option<&BTreeSet<u64>>;
    pub fn steps(&self) -> Option<&ToolRunStepRing>; pub fn trace(&self) -> Option<&ToolRunTraceStore>; pub fn conflicts(&self) -> u32;
    pub fn is_refolding(&self) -> bool; pub fn progress(&self) -> Option<ToolRunProgress>;
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot>;
    pub fn view(&self) -> Option<ToolRunView>; pub fn freezes_local_emits(&self) -> bool; pub fn has_pending_work(&self) -> bool;
    pub fn apply_tick(&mut self, tick: ToolRunTick) -> Result<ToolRunTickReceipt, Fault>;   // O(k) overlay append
    pub fn note_window_config_published(&mut self);
    pub fn presence(&self) -> Option<protocol::PresenceToolRun>;                           // completed ≤ total
    pub fn trace_delta(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String>; // base64url, unpadded
    pub fn begin_close(&mut self); pub fn close_step(&mut self, store: &mut ArtifactStore<..>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;
    pub fn terminal_is_empty(&self) -> bool;
    pub fn panel(&mut self, controller_id: &str, locale: Locale, tool_label: Option<String>) -> UiAssemblyResult<BuiltNode>;
}
impl VcsArtifactApp<A, M> {
    pub async fn apply_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault>;
    pub(crate) async fn dispatch_tool_run_action(..) -> Result<InvocationResult, Fault>; // output {"toolRun": effect} | {"rejected": code}
    pub(crate) async fn drive_tool_run_turn(&mut self) -> Result<(), Fault>;
    pub(crate) fn tool_run_has_pending_work(&self) -> bool;
}
// ArtifactApp / ArtifactEditor (EditorApp forwards)
fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> { Ok(None) }
// PluginApp (defaults None; VcsArtifactApp implements)
fn tool_run_presence(&self) -> Option<protocol::PresenceToolRun>;
fn tool_run_trace_delta(&self, cursor: Option<ToolRunTraceCursor>) -> Option<String>;
// S
pub fn begin_outbound_apply_batch(&self, operation, expected_generation, expected_revision, actor, mutations, description, factory) -> Result<ArtifactStoreBatchPublication<P, M>, ArtifactStoreBatchAdmissionRejected<M>>;
pub async fn flush_published_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, M>) -> Result<bool, VcsError>;
```

### Behaviour the plugin lanes rely on

- **`toolRunStart`**
  - Takes `toolId`. Without it, it falls back to `ViewModel.active_utility_id` when that utility declares `run`, otherwise to `active_tool_id`.
  - Faults `toolRun.unknown-tool` when the manifest declares no `run`.
  - Answers `rejected: toolRun.busy` while a run is non-terminal.
- **`toolRunPause`** on a paused run **resumes** it (W0-B binds the shared chord to Pause).
- **`toolRunDismiss`** acts only through its button or action; no key is bound.
- **Stale, busy and illegal actions** are silent no-ops. They return `Ok` with `output.rejected`.
- **The run job**:
  - Reports `ToolRunTick`s via `StepOutcome::PreviewReady`. The payload may be multi-page, up to 256 KiB.
  - Reports checkpoints via `CheckpointReady`. The bytes are stored and handed back as `request.checkpoint` on reconfigure `resume`.
  - Ends with `Complete`.
  - `Fault`/`Cancelled` outside abort → `faulted`, and provisional ops are discarded.
- **Single step** is `drive_step` with `fuel = 1`. `CheckpointReady` or `Yield` outcomes do not use up the step, so one step is always one unit (one tick).
- **Watch.**
  - A store generation change → `baseChanged`. The base becomes the head, a bounded refold starts (the old overlay stays rendered) and a `warning` rebasing step is added. `restart` also closes the job and discards provisional ops.
  - A config-store generation change or a window-config publication → `settingsChanged`. The job closes and is rebuilt next turn: from its checkpoint under `resume`, from scratch with provisional ops discarded under `restart`.
- **Finalize.**
  1. Refold on the head if it moved.
  2. Build a `Revalidate` job when `revalidate_job` is declared and ops exist. Its ticks may `retractTo`. Any retraction → `revalidationConflicts`: back to `complete`, generation+1, `conflicts += n`, and a `danger` conflict step.
  3. `begin_outbound_apply_batch`, advanced one item per grant inside the turn wall.
  4. On `Published`: `stamp_tail_group_id("toolRun:<run>")`, then `flush_published_apply_batch` (one `Mutations`), `acknowledge`, one `record_command(tool_id, Mutation, label, edit_id)`, interaction revalidation, bounded publication close, then `publicationComplete` → `finalized`.
  5. A missing factory, admission rejection or advance error → `storeRejected`: generation+1 and a `danger` step.
- **Abort** is legal until the batch is at or past `Publishing`. It cancels the job token, runs `begin_close`, `cancel_apply_batch`, bounded close turns, then `abortComplete` → `aborted`. Provisional ops are cold-retired (`Mutation::retire_cold`) at most 4 096 per turn.
- **Panel** (`FRAMEWORK_TOOL_RUN_BODY_KEY`), under `column` `framework.toolRun` labelled with the manifest tool label:
  - `framework.toolRun.status`: text, `Polite` (`Assertive` for faulted or conflicts), refreshed at most every 2 s or on a state change;
  - `framework.toolRun.progress`: `Component::Progress` with the §2.5 valuetext;
  - `framework.toolRun.actions`: row of real buttons `framework.toolRun.<actionId>` with `{runId (text), generation}` args, `aria-keyshortcuts` = chord, disabled when illegal; a disabled Finalize carries the "Available once the run is complete" description;
  - `framework.toolRun.steps`: column, live off, newest 16;
  - `framework.toolRun.trace`: tree, newest 32 upserted records, tone by verdict.
- **Scene lane (W0-E).** Call `PluginApp::tool_run_trace_delta(cursor)` with the renderer-echoed `ToolRunTraceCursor` and put the returned base64url string into lane `toolRunTrace`. `None` means nothing is pending.

## 3. Tests run (exact commands, all foreground)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-plugin --features artifact-app-testing -- tool_run` | **10 passed, 0 failed** (`verify-test-final.txt`). 9 are this lane's laws; the 10th is W0-B's builder injection test |
| `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2` | Finished; the plugin lib reached 28 warnings, none in `R` (`verify-wasip2.txt`) |
| `cargo check -p semio-framework-os-kernel` | Finished, 3 warnings (`verify-kernel.txt`) |
| `cargo check -p semio-framework-plugin --features artifact-app-testing --tests` | 0 errors; no warnings in `R` or the test file (`check-tests-3.txt`) |

The nine laws:
1. overlay rendered (`count=5`) while committed stays 0 and the store generation does not move, plus panel roles, presence clamp and base64url delta;
2. the abort invariant: store generation, `vcs.edits` length, command log, and memory-backbone outbox `(0 Mutations, 0 Snapshot)` all equal their start values;
3. finalize: store generation +1, one `Edit` holding all 10 ops with `group_id toolRun:1`, one command row, exactly `(1 Mutations, 0 Snapshot)`, and `Undo` restores count 0 and label "";
4. remote ingest → rebase (generation 1) → revalidation conflict → `complete` with generation 2, 10 retracted, no publish;
5. stale generation and run no-ops, finalize `toolRun.illegal` while running, start `toolRun.busy`, disabled-finalize description;
6. pause, then step three times, is exactly +2 ops per step and nothing while paused; Pause toggles to resume;
7. reconfigure 5→8 resumes from checkpoint 5 (same run, generation 1); 8→3 retracts (generation 2) with the overlay at `count=3`;
8. the `freeze` policy makes local emits fault with `toolRun.busy`;
9. **bench**: 771 ticks of 2 ops (`SetCount` plus a 96-byte `SetLabel`) through `apply_tick`, with at most 3 ticks allowed at or above 2 ms for preemption under concurrent builds. The whole run took about 0.02 s.

**Mutation check.** Making `overlay_or` always return committed turned 3 laws red (overlay, reconfigure, bench) (`test-mutation-overlay.txt`). The file was restored and verified byte-identical with `cmp`.

**Neighbour checks.**
- `artifact_store_batch_*` store tests: 5 pass. 1 fails (`…commit_refuses_an_under_declared_multi_item_gesture…`), and it fails identically with my fold edit reverted, so it is **pre-existing** (`verify-store-batch-without-hook.txt`).
- `testkit_law_chronological_determinism…` fails on the peer "edit history insertion requires its exact mutation retirement factory" (already reported by W0-F).
- The plugin `mutation_fixture::transaction::*` tests fail on the pre-existing non-canonical `testkit-txn` app id (the baseline run before my edits failed the same way).

TDD note: the fixture and the laws were written before the first test run. The first runs were red:
- `duplicate mutation id` led to S hook 2;
- the rebase watch was not pumped in `complete`;
- the command-log backfill capture, the probe queue holding store close, and single-step checkpoint consumption were also found this way.

## 4. Commands to register in launch.json

- `cargo test -p semio-framework-plugin --features artifact-app-testing -- tool_run`. The plugin `📜️script.ts` has no filtered target, so the coordinator should add one, for example `test-tool-run` in the plugin package `📜️script.ts`.

## 5. Deviations from the contract, with reasons

1. **No job admission through ActionBus or `Effect::SpawnJob`.** The ledger owns the `Box<dyn InteractiveJob>` and steps it with `drive_step` (interactive lane budget, 8 ms watchdog) inside `advance_typed_operation_publication`, which the host already pumps each turn. A spawned tool job completes into one `Emit` publication; it cannot stream preview ticks, be paused or single-stepped, or be aborted by the host without queueing behind itself.
2. **The job is built by the `ArtifactApp::build_tool_run_job` hook,** not by `ArtifactToolFactoryRegistry` keyed on `JobKindId`. The registry's factories are typed-command factories with Emit completion. `request.definition.run_job` / `revalidate_job` are passed through so an app can dispatch on them. The **definition itself comes only from the manifest** (`registry.tool_run`); there is no second source.
3. **S hooks** (§1): the outbound batch variant, plus unique per-op ids in multi-item batches. The contract expected no S edit.
4. **The step log is a `column` group and the panel root a labelled `column`.** The UI contract has no `log` role and no public `group` container builder. Liveness is Off on the log and Polite/Assertive on the status line, as in the contract.
5. **The trace list is not bound to a camera action.** Framing a record is plugin-specific (window config), so W1-B binds it.
6. **Entities are tagged with the provisional length at the end of their tick** (W0-A open item). On `retractTo(n)`, an entity whose tick ended after `n` is dropped. This is conservative for partially retracted ticks.
7. **`storeRejected` uses reason `TOOL_RUN_REASON_CONFLICT` with `args = [provisional len]`.** Steps carry no prose, and no reserved "store rejected" reason exists.
8. **Refold conflicts** (an op no longer applicable to the new base) are skipped in the overlay and counted in `conflicts`. Their semantic check stays with the revalidate job.
9. **Tick UI dirtiness** pushes one `UiDirtyScope::Full` into the typed UI outbox when the outbox is empty. That is coarse but coalesced; see open item 4.
10. **The overlay and base `Arc`s drop normally;** they are not retired through app snapshot retirement factories. `DocumentStoreOwners` exposes no factory accessor. Provisional ops do use `retire_cold`.

## 6. Foreign edits

- `S` (listed as conditional in the lane row): the two hooks in §1.
- None elsewhere. W0-B's line in `P` was kept untouched.

## 7. Open items

1. **W0-A.** `ToolRunTickWriter` has no way to start from an existing provisional length. A resumed or revalidate job cannot use `retract_to` below its own appends (the toy builds `ToolRunTick` literals). Add `ToolRunTickWriter::with_provisional_base(identity, len)`.
2. **Shell / W0-E.**
   - Fill the local `PresencePeer.tool_run` from `PluginApp::tool_run_presence()`. The wgpu Shell builds that peer, outside this lane.
   - Put `tool_run_trace_delta(cursor)` into the `toolRunTrace` scene lane.
   - The contract's `(instance, cursor)` signature lives on the per-instance `PluginApp`; the host resolves the instance.
3. **W1-B.**
   - Implement `build_tool_run_job` for fill (Run and Revalidate).
   - Supply `build_artifact_store_one_item_preparation_factory`; without it, finalize answers `storeRejected`.
   - Use `ArtifactView::tool_run()` for the `provisional` style.
4. **Performance.** A per-tick `Full` UI scope repaints everything. Narrow it to the world window body and the panel once hosts expose a per-body scope for scene lanes.
5. **`restart` rebase policy while `complete`** refolds nothing and leaves the reducer in `complete` with the provisional ops discarded. The reducer has no `complete → running` edge for `baseChanged`; W0-A should decide whether one is needed.
6. **Pre-existing reds needing owners:**
   - `artifact_store_batch_commit_refuses_an_under_declared_multi_item_gesture_and_accepts_the_invertible_one` (fold inverse capacity);
   - the plugin transaction fixture's non-canonical app id.
