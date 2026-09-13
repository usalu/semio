# ⏯️ Wave W1-A: puzzle 3d fill run job

Lane W1-A of `📋️tool-run-contract.md` (§2.7, §3.2, §3.3 resume policy, §3.7, §5 wave 1). Status: **landed. All new
laws are green. Native and wasm32-wasip2 checks reach warnings.**

Paths are relative to the repo root.

- `A` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `F` = `A/✏️editor/⏳️precompute/🪣️fill`

## 1. What changed

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | New dependency `semio-framework-tool-run = { workspace = true }`. New dev-dependency `parry3d = "0.17"` (the oracle, test-only) |
| `A/🧬️schema/🦀️.rs:825-1025` | New schema types: `FillRunStage`, `FillRunCounter`, `FillRunReason`, `FillRunCheckpoint` |
| `A/🧬️schema/🔣️.json:534` | New `$defs`: `Puzzle3dFillRun` (source of record: the `x-semio-toolRun` table of stages, counters, reason codes with verdicts, and the checkpoint layout), `Puzzle3dFillRunStage`, `Puzzle3dFillRunCounter`, `Puzzle3dFillRunReason`, `Puzzle3dFillRunCheckpoint` |
| `F/🦀️.rs:42-86` | New `FillStepContext` trait (implemented for `StepContext`), `FillRunEvent`, `fill_run_scale` |
| `F/🦀️.rs` (builder) | See the builder bullets below |
| `F/🦀️.rs:4825-5539` | New region `⏯️FillRunJob`: `FillRunJob`, `FillRevalidateJob`, the op and entity helpers, `FillRunTransitionContext` |
| `A/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` | `CollisionAabb::intersects` is now `pub(crate)` (the revalidate job uses it) |
| `F/🧫️fixtures/🎞️fill-run.json` (new) | Language-neutral run law: 3 seeded example cases plus the oracle, delivery and interactive law parameters |
| `F/🧪️tests/🔬️unit/🦀️.rs:1460-2095` | 8 new tests and their helpers, described in §3 |

Builder changes in `F/🦀️.rs`:

- `InteractiveJob::step` now delegates to the generic `FillBuilder::advance<C: FillStepContext>` (`:4717`). `test_collision`, `publish_preview` and `stall_on_capacity` are generic over the same trait.
- A new `run_events: Option<Vec<FillRunEvent>>` field records observations only while a run job observes the builder. The session lane leaves it `None`, so it records nothing. The field takes part in the close walk (field 29) and in `terminal_owner_debt`.
- Events are emitted at these points:
  - after a pose is built (`Constructed`);
  - in `reject_candidate` and `reject_target` (`Refused`);
  - at commit (`Accepted`);
  - in `stall` (`Refused` for a live candidate, then `Stalled`);
  - in `discard_tail_one` (`Discarded`);
  - in `set_requested_count` when lowering and in `begin_soft_replan` (`Abandoned`).
- **Behaviour fix:** `round_constructed` is now set when the pose is built, before the target-volume check (`:4161`). Before, a round whose poses were all outside the target volume stalled as `no-compatible-kind` instead of `no-free-placement`.

No other file was touched. `E3/🦀️.rs`, commands, tools and windows are unchanged. The existing session API (`preview_json_*`, the tried ring, `take_fill_locked_chunk`, …) still compiles and behaves as before.

## 2. Public API as landed

### Schema (`A/🧬️schema/🦀️.rs`)

```rust
pub enum FillRunStage { Prepare, Search, Test, Lock, Retract }          // ALL, index() -> u16, id() -> "prepare"|"search"|"test"|"lock"|"retract"
pub enum FillRunCounter { Tested, Locked, Collisions, Rejected }        // ALL, index() -> u16, id()
pub enum FillRunReason {                                               // code() = ordinal (u16), from_code, id(), of_id, of_refusal, verdict() -> ToolRunVerdict
    Fits /*0 success*/, SolidOverlap /*1 danger*/, OutsideTargetVolume, MeshUnavailable, MissingPreview, MissingTarget,
    BroadPhaseEntryMissing, PlacedMeshUnavailable, StaleSpatialQuery, PlacementKindMissing, PlacementVortexMissing,
    PlacementMeshMissing, PlacementRejected, PlacementStateMissing, PlacementSpatialStateMissing, StaleSpatialMutation,
    Rejected /*16, any undeclared refusal*/,                            // 2..=16 warning
    NoOpenVortex /*17*/, NoCompatibleKind, NoFreePlacement, DocumentCapacity /*20*/,  // stall steps, warning
    RequestedReached /*21 success step*/, Retracted /*22 info step, verdict testing*/,
}
pub struct FillRunCheckpoint { pub requested: u64, pub placements: u64, pub provisional_ops: u32, pub tested: u64, pub next_key: u64 }
impl FillRunCheckpoint { pub const BYTES: usize = 36; pub fn encode(self) -> [u8; 36]; pub fn decode(bytes: &[u8]) -> Option<Self> } // LE, field order as listed
```

For W1-B, the `ToolRunDefinition` is assembled like this:

- **stages:** `FillRunStage::ALL`.
- **counters:** `FillRunCounter::ALL`.
- **reasons:** `FillRunReason::ALL`, each with `code()`, `id()` and `verdict()`. W1-B adds the EN/DE templates in terminology.
- **trace:** `instance3d`.
- **policies:** `rebase: revalidate`, `reconfigure: resume`.

### Run job (`F/🦀️.rs`, `editor::puzzle3d::precompute::fill`)

```rust
pub(crate) const FILL_RUN_TICK_FLUSH_BYTES: usize = 8 * 1024;   // estimated tick bytes before a flush (one 16 KiB job payload page)
pub(crate) const FILL_RUN_OPS_PER_PLACEMENT: u32 = 2;           // create_object, connect_vortices
pub(crate) fn fill_run_entity(object_id: &str) -> u64;          // first 8 LE bytes of semio_framework_hash::hash(id)
pub(crate) fn fill_run_ops(object: &FixtureObject, attraction: &AttractionProps) -> Option<[Vec<u8>; 2]>; // OpBinary encoded
pub(crate) struct FillRunPlacement { pub key: u64, pub subject: ToolRunTraceSubject, pub entity: u64, pub object: FixtureObject, pub attraction: AttractionProps }
pub(crate) enum FillRunResumeError { Malformed, Foreign }
pub(crate) struct FillRunJob;   // impl InteractiveJob
impl FillRunJob {
    pub(crate) fn new(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>) -> Self;
    pub(crate) fn builder(&self) -> &FillBuilder;
    pub(crate) fn operation(&self) -> Operation;
    pub(crate) fn identity(&self) -> ToolRunIdentity;
    pub(crate) fn rebind(&mut self, identity: ToolRunIdentity);
    pub(crate) fn mesh_lane(&self) -> &[String];                // caller lane plus any url it lacked, appended
    pub(crate) fn counters(&self) -> [u64; 4];                  // FillRunCounter::ALL order
    pub(crate) fn stage(&self) -> FillRunStage;
    pub(crate) fn checkpoint(&self) -> FillRunCheckpoint;
    pub(crate) fn provisional_placements(&self) -> Vec<FillRunPlacement>;
    pub(crate) fn resume(&mut self, checkpoint: &[u8], requested: usize) -> Result<(), FillRunResumeError>;
}
pub(crate) struct FillRevalidateJob;   // impl InteractiveJob
impl FillRevalidateJob {
    pub(crate) fn new(operation: Operation, identity: ToolRunIdentity, head: FillPreparationRoots, placements: Vec<FillRunPlacement>, first_sequence: u64) -> Self;
    pub(crate) fn conflicts(&self) -> &[bool];
}
// builder seams
pub(crate) trait FillStepContext: CollisionStepContext { fn operation; fn generation; fn set_stage; fn next_preview_sequence; fn fault_payload }
pub(crate) enum FillRunEvent { Constructed { mesh_url, origin, orientation, scale }, Refused(FillRunReason), Accepted, Abandoned, Stalled(FillRunReason), Discarded }
impl FillBuilder { pub(crate) fn advance<C: FillStepContext>(&mut self, &mut C) -> StepOutcome; pub(crate) fn observe_run(&mut self); pub(crate) fn swap_run_events(&mut self, into: &mut Vec<FillRunEvent>) }
```

### Run job step semantics

**Fuel.** One unit of fuel is one candidate verdict (`consume_fuel(1)` per verdict). Planner transitions and collision samples do not consume the step's fuel. They run in a `FillRunTransitionContext` that forwards the deadline, cancellation and identity.

**Trace.**

- Each constructed pose is traced as `Upsert { key, Testing, 0, Instance3d { mesh, position, rotation, scale } }`.
- `key` is monotone from 0. `mesh` is the index of the pose's mesh url in `mesh_lane`. `scale` is uniform: a number, else the first vector component, else 1.
- The same key then gets exactly one final upsert:
  - `Danger`/`solid-overlap` for a collision;
  - `Warning`/`<refusal code>` for a rule refusal;
  - `Success`/`fits` for a placement.

**Placements.** Each placement appends `create_object` then `connect_vortices` and one entity.

**Flushing.** The job flushes a tick (`PreviewReady`) in these cases:

- the fuel is exhausted;
- the deadline is exceeded;
- the estimated pending bytes reach 8 KiB;
- a placement was made. The next call then returns `CheckpointReady` with the 36-byte state and `applied_progress = placements`.
- the planner completed. The next call then returns `Complete`.

`CheckpointReady` and `Complete` never carry a tick.

**Progress.** Every tick carries progress:

- `state` is `Running`, or `Complete` on the last tick.
- `stage` is the `FillRunStage`.
- `completed` is the number of placements and `total` is the requested count.
- There are 4 counters.
- `steps` is an empty ring, because the ledger accumulates `tick.steps`.

**Completion.** When the run completes, one step is emitted:

- `warning` with the stall reason and `args=[placements]`; or
- `success` with `requested-reached` and `args=[placements]`.

**Resume.**

- A completed job stays resident. `resume(own checkpoint, n)`:
  - rejects a checkpoint from the future as `Foreign` and bytes that are not 36 long as `Malformed`;
  - calls `set_requested_count(n)`.
- A raise continues the RNG stream: the ops and verdicts equal one run to `n`.
- A lower walks `DiscardTail`. Each discarded placement retires its key, emits `retract_to(2·remaining)` and one `info`/`retracted` step (consecutive ones coalesce).
- A candidate that was under test when the count was lowered is retired.

**Provisional cap.** If a placement would exceed `TOOL_RUN_PROVISIONAL_OPS_MAX`, the job turns that candidate `warning`/`TOOL_RUN_REASON_PROVISIONAL_CAP`, adds a warning step, lowers the count and completes.

### Revalidate job semantics

1. **Head preparation.** The job reads one head object per transition: its id, its vortex full ids → owner, and its mesh body.
2. **Per placement** (one unit of fuel each), a placement conflicts when:
   - its id already exists in the head; or
   - its host vortex is gone (neither in the head nor on a surviving earlier placement); or
   - its body overlaps a non-host head body beyond `scene.overlap_budget`. The overlap uses an AABB prefilter, then `CollisionOverlapState`.
3. **Trace.** Each placement is upserted `Testing`, then `Success`/`fits` or `Danger`/`TOOL_RUN_REASON_CONFLICT`, using the run's own key and subject.
4. **Final tick.**
   - If nothing conflicts, the final tick carries no ops.
   - Otherwise it carries `retract_to = 2·first_conflict`, re-appends the ops and entities of every later survivor, and adds one `danger` step `TOOL_RUN_REASON_CONFLICT args=[conflicts]`.
   - Progress has `state: Finalizing`.
   - Ticks are numbered from `first_sequence`.
5. `Complete` follows on the next call.

## 3. Tests (all foreground; logs in `T/🗑️generated/W1-A/`)

### New laws (`F/🧪️tests/🔬️unit/🦀️.rs`)

| Test | What it proves |
|---|---|
| `fill_run_job_matches_the_language_neutral_fill_run_fixture` | See the case list below |
| `fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle` | See the oracle description below |
| `fill_run_job_delivers_every_trace_record_of_a_5000_candidate_run` | Concrete Forest, run until ≥ 5 000 tested. Three key→verdict maps are equal: the one the job reported, the ledger `ToolRunTraceStore`, and a renderer store fed only by `delta_after(cursor, 64 KiB)` |
| `fill_run_job_step_stays_below_the_interactive_ceiling_for_nakagin` | Red→green row 1, part (a). Real Nakagin, `drive_step` under `INTERACTIVE_LANE_WALL_US`. It takes the per-turn best of 5 cold runs; run 1 slices by the real clock and runs 2-5 replay its exact expiry reads. The same method as the artifact's `measured_cold_runs` was needed, because a single wall-clock run under load average 17 spiked to 18-46 ms. The last run measured **2 270 turns, worst 1.106 ms < 2 ms** |
| `fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict` | With fuel 1, every tick carries exactly one final verdict, whose key equals the key tested in that tick. The only exception is the final completion tick. The number of verdicts equals `tested` |
| `fill_run_job_resume_raise_continues_the_sequence_and_lower_retracts_the_tail` | Raise 30→45 from the last checkpoint gives ops, entities, verdicts and counters equal to a fresh run to 45. Lower 30→12 retracts to exactly op 24, keeps the op prefix, and retires the 18 success records. `Foreign` and `Malformed` are refused |
| `fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors` | With a clean head there are no conflicts, no retract and no ops. With an intruder body on placement 3, that placement conflicts, the final tick retracts to the first conflict, re-appends the exact survivor ops and entities, adds the conflict step with its count, and upserts `danger` for exactly the conflicting keys |
| `fill_run_job_reports_rule_refusals_as_warnings_and_the_stall_as_a_warning_step` | With a target volume far away, every verdict is `warning`/`outside-target-volume`, nothing is appended, the last step is `warning`/`no-free-placement` with `[0]`, and the state is `Complete` |

The fixture test runs three cases:

- Nakagin seed 1, requested 12;
- Concrete Forest seed 7, requested 8;
- Concrete Forest seed 3, requested 40.

Each case builds the real example document through `scene_from_snapshot` → `scene_config_value`, with the app's 4× box fallback mesh for every mesh identity. It checks:

- the exact verdict prefix (up to 24 entries), `tested`, `locked`, `collisions`, `rejected`, `appendOps`, `appendEntities`, `checkpoints` and `stall`;
- that the schema `x-semio-toolRun` table equals the Rust enums;
- the per-run laws: two ops and one entity per placement; verdict counts equal the counters; every tested key stays resident; ops alternate `CreateObject`/`ConnectVortices`; each entity equals `fill_run_entity(created id)`; final progress is `Complete`.

The parry3d oracle runs Concrete Forest seed 7, requested 60. It recomputes every `danger`/`success` candidate against every body placed before it, excluding the docking host:

- Each body is an exact `ConvexPolyhedron` hull.
- Pairs are prefiltered by AABB and parry `distance`.
- Overlap volume comes from parry `contains_local_point` on a 16³ grid.
- A verdict is decisive when the overlap is ≥ 2× the budget with ≥ 16 expected samples (collision), or ≤ ½ the budget with the threshold at ≥ 16 samples (fit).

Result: **204 decisive, 0 disagreements, 4 ambiguous, 148 collisions, 60 fits.**

Observation: on Nakagin with the box fallback, only 1 of 3 344 candidates fits; the rest collide. parry agreed on all 3 344 in an earlier depth-based oracle run. That is why the oracle and delivery laws use Concrete Forest.

### Commands run

The logs are in `T/🗑️generated/W1-A/`. `RUST_MIN_STACK=134217728` was set on every test run.

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --message-format short` | Finished, 0 errors, warnings reached in `🪣️fill/🦀️.rs` (`check-1.txt`, rerun after the last edit in `check-final.txt`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --target wasm32-wasip2 --message-format short` | Finished in 2 m 39 s, 0 errors, 111 warnings including `🪣️fill/🦀️.rs` (`check-wasip2-1.txt`) |
| `cargo test … --lib -j 4 -- fill_run_job fill_revalidate --test-threads=4` | **8 passed, 0 failed** (`test-run-10.txt`) |
| `cargo test … --lib -j 4 -- fill --test-threads=4` | **106 passed, 2 failed** (`test-fill-threads4.txt`) |
| `cargo test … --lib -j 4 -- fill --test-threads=1` | **106 passed, 2 failed** (the same two), 690 s (`test-fill-threads1.txt`) |

The two failures in the `--test-threads=4` run are both §0.9 reds in `E3/🧪️tests/🔬️unit/🦀️.rs`, which W1-B/W1-F own and this lane may not edit:

- `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` (`fillBuildTick` gets deleted);
- `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count` (the stale literal 0 vs 100).

Every test under `precompute::fill` passes.

### 3.1 Single-threaded run

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- fill --test-threads=1` finished in 690 s with **106 passed, 2 failed**. The two failures are the same §0.9 reds in W1-B's `E3/🧪️tests` as in the 4-thread run. All 8 W1-A laws pass single-threaded as well.

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill_run_job fill_revalidate`: the W1-A laws, about 70 s at opt-level 0.
- `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --target wasm32-wasip2`

## 5. Deviations from the contract, with reasons

1. **Resume is resident, not a rewind.** A `FillBuilder` (spatial index, fixed pages) cannot be serialized into one 16 KiB checkpoint page.
   - The checkpoint is therefore a 36-byte position.
   - `resume` validates that the checkpoint belongs to this resident job and retargets it.
   - The prefix property makes this equal to "restart from checkpoint with new settings" (proved against a fresh run).
2. **The revalidate job takes typed placements.** It consumes `FillRunPlacement` values (key, subject, entity, object, attraction) from `FillRunJob::provisional_placements()`, not raw op bytes. The reason is that trace keys and subjects are not recoverable from ops.
   - The retract-then-re-append shape expresses removing middle ops with the contract's truncate-only `retractTo`.
3. **The revalidate job builds its ticks directly.** `ToolRunTickWriter` has no constructor with a non-zero provisional base, so the job builds `ToolRunTick` values from their public fields. It is not a foreign edit.
4. **Ticks and outcomes are separate.** `CheckpointReady` and `Complete` come one call after their tick. `StepOutcome` carries one payload, and the contract routes ticks only through `PreviewReady`.
5. **Mesh lane.** The caller supplies the mesh lane.
   - Today's `collect_mesh_urls` iterates a `HashSet`, so the order of `meshesJson` is not stable.
   - W1-B must publish the lane in the same order it passes, and read `mesh_lane()` back for any appended url.
6. **Testing records carry reason 0 (`fits`).** The contract has no "testing" reason code; renderers key off the verdict.
7. **Interactive-law name.** The law is named `fill_run_job_step_stays_below_the_interactive_ceiling_for_nakagin` because part (b), the overlay append, belongs to W0-D.
8. **Order of work.** The fixture skeleton and the tests were written before the first run. The fixture's exact expected values were then filled in from that run. They are cross-validated by the count laws and by the parry3d oracle.

## 6. Foreign edits

None. Only lane-owned files were changed; the `CollisionAabb::intersects` visibility change is in `📐️geometry`, which this lane owns.

## 7. Open items, for W1-B unless noted

1. **The tried ring and the 16 KiB preview JSON cap were not deleted.** They are still used outside this lane's files:
   - `A/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:885-889` (`fill_preview_object_kind`, `fill_preview_json_page`);
   - `A/✏️editor/⏳️precompute/🦀️.rs:3354-3383`;
   - `WH` (`FillTriedGhosts`, `WORLD_FILL_PREVIEW_JSON_MAX_BYTES`, W1-C);
   - the engine-contract TS test (`…/🧪️tests/🔬️engine-contract/🟦️.ts:5762-5784`);
   - root `📜️script.ts:9723,9789,9799,9814` and `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts` (W1-D).

   Once those are gone, delete in lane files: `FILL_TRIED_RING`, `FillTriedCandidate`, `FillBuildPreview.tried`, `push_tried`/`tried_*`, the whole `🔭️RetainedPreviewJson` region with `FILL_PREVIEW_JSON_MAX_BYTES`, and the JSON `$defs` `Puzzle3dFillPreview*`, together with their tests. Keeping this API compiling was the lane rule.
2. **Wiring (W1-B):**
   - Register `FillRunJob` as `runJob` and `FillRevalidateJob` as `revalidateJob`.
   - Build the `ToolRunDefinition` from §2 and add EN/DE templates for the 23 reasons, 5 stages and 4 counters. The 3d terminology already has `fill_tested`, `fill_collision` and `fill_rejected` labels.
   - Feed `runtime.fill_count` through `resume`.
3. **W0-D:**
   - Decode ticks from `PreviewReady` pages and close each page afterwards.
   - Treat `CheckpointReady` as position-only (§5.1).
   - Accumulate `tick.steps` into the ring, because progress carries an empty ring.
   - Recompute the provisional entity set on `retractTo`. This job keeps entities index-aligned: 1 entity per 2 ops.
4. **Coverage gap.** No shipped document exercises a `warning` verdict under the box fallback (`rejected = 0` in every fixture case). The rule-refusal law covers it with a synthetic target volume.
5. **Dead-code warnings.** The new `pub(crate)` run API reports dead-code warnings (about 20) until W1-B uses it outside tests.
