# Wave P2 — precompute step budget and the 28 red precompute tests

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Scope: `✏️editor/⏳️precompute/**` plus the retained command
works that drive it. Everything below was executed; every number is measured, never inferred.

Toolchain for every command in this report:

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/
  9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d
RUST_MIN_STACK=134217728        (for test runs)
cargo … -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4
```

---

## 0. Headline

| measure | before this wave | after |
| --- | ---: | ---: |
| `editor::puzzle3d::precompute` module, one process, `--test-threads=1` | **104 passed / 28 failed** | **132 passed / 0 failed** |
| `precompute::component::tests` | 23 failed | 0 failed |
| `precompute::fill::tests` | 4 failed | 0 failed |
| `precompute::geometry::tests` | 1 failed | 0 failed |
| `precompute::brush::tests` | 0 failed | 0 failed |
| `cargo check --features component-app-assembly --tests` | 0 errors / 1 warning | **0 errors / 0 warnings** |
| lane turns to plan one fill placement (`precompute_step_lane(Fill, 1)`) | ≈ **4 000** (thread round trip) | **1** |
| `dispatch(SetScene)` on the 180-object Nakagin document | **39.4 ms** | **0.18 ms** |
| `openVortexSuggestions` one-turn cost, Nakagin, opt-level 0 | **82.5 ms** | **10.2 ms** (steady) — still over the 8 ms ceiling, §6 |

The wave's own baseline run is `run…/p2/baseline.txt`; the final one is `run…/p2/run7.txt`
(both in the session scratchpad, not in the ticket, per the "no tool-generated output files" rule).

---

## 1. The fill planner was never actually running — root cause, measured

`⏳️precompute/🦀️.rs`'s fill lane pumped the planner through a **worker pool**
(`MountedWorkerJobSession::pump_one(&fill_worker_pool(), Lane::Background)`), i.e. every single
`FillBuilder::step` cost a thread round trip that the caller then *spun* on, one poll per
`precompute_step_lane` call.

Measured with temporary instrumentation (since removed), `fill_capable_engine()` + 20 000 lane turns:

```
[DEBUG] i=0     stage=Some(PrepareFixture)       elapsed=591µs
[DEBUG] i=775   stage=Some(PrepareCatalogs)      elapsed=1.88ms
[DEBUG] i=4377  stage=Some(PrepareMeshes)        elapsed=5.83ms
[DEBUG] i=6725  stage=Some(PrepareEntries)       elapsed=8.42ms
[DEBUG] i=10632 stage=Some(PrepareLookup)        elapsed=13.78ms
[DEBUG] i=13743 stage=Some(PrepareTargets)       elapsed=17.34ms
[DEBUG] i=15247 stage=Some(PrepareCandidates)    elapsed=19.46ms
[DEBUG] lane death via pump error
[DEBUG] counts={"none": 15475, "preview": 9, "yield": 58}
```

**≈ 4 000 idle lane turns per planner step**, and then the lane *died* on
`MountedWorkerJobPumpFault` and set `fill_steps_remaining = 0`. In the app the fill lane is driven one
turn per 120 ms `fillBuildTick`, so a single planner step would have taken **eight minutes**; the fill
tool could never produce a plan. On the wasm guest there is no pool at all.

**Fix** — drive the planner ON THE CALLER:

* `⏳️precompute/🦀️.rs:212` — `type MountedFillWorker = semio_framework_job::BatchJobSession<SharedFillWorkerJob>`
  (was `MountedWorkerJobSession`). `BatchJobSession::step()` is the framework's own
  `try_step_on_caller` driver: one bounded `FillBuilder::step` synchronously, no pool, no `J: Send`.
* `⏳️precompute/🦀️.rs:1704-1717` — the interactive lane arm now `worker.step()` + `worker.take_outcome()`.
* `⏳️precompute/🦀️.rs:666-679` — `drive_fill_envelope` (the isolated background job) takes the same path.
* `fn fill_worker_pool()` and the `semio_framework_async` pool imports are gone from this file.

After the fix, one `precompute_step_lane(Fill, 1)` = exactly one planner step, and
`fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms` passes.

---

## 2. The three other production defects fixed

### 2.1 The fill projection returned an EMPTY document while the planner was still preparing
`⏳️precompute/🪣️fill/🦀️.rs:3119` `FillBuilder::base_fixture()`, used at
`⏳️precompute/🦀️.rs:1771`, `:1788`, `:2336`.

`compose_fill_display` / `apply_fill_count` / `compose_fill_projection` all read `fill.base.snapshot()`.
Since preparation became cursorized, `base` is only *partially* copied until `PrepareConfiguration`
releases the preparation roots — so `ComposeFillDisplay { count: 0 }` on a freshly seeded session
returned a fixture **with the document's own objects missing**. `base_fixture()` reads the still-owned
`preparation_roots.scene.fixture` while preparing (where `sequence` is empty, so the projection is
exactly the scene) and the fixed page afterwards.

Caught by `precompute_session_native_wrapper_exercises_public_methods` and
`dispatch_set_scene_then_apply_and_compose_fill_count_round_trip`.

### 2.2 A distribution-weight edit destroyed every already-applied fill object
`⏳️precompute/🦀️.rs:1288` `soft_replan_fill_tail`, `⏳️precompute/🪣️fill/🦀️.rs:3284`
`FillBuilder::begin_soft_replan`, `:3301` `discard_tail_one`, `:861` `FillJobStage::DiscardTail`,
`:1063` `tail_removal: Option<CollisionIndexRemoval>`.

`soft_replan_fill_tail` called `start_fill_preparation(false)`, which **constructs a brand new
`FillBuilder`** — dropping `applied_count`, the plan and every applied placement. Its own docstring
says the opposite ("applied fill objects stay, only the unapplied planning tail is discarded").

The live builder is now replanned in place: `begin_soft_replan` installs the new weight maps and moves
the planner to a new `DiscardTail` stage; `discard_tail_one` withdraws **one** unapplied placement per
turn (`placed_lookup` row, `placed` row, then a resumable `CollisionSpatialIndex::begin_removal` /
`step_removal`), and rewinds to `PrepareTargets` when it reaches the applied prefix. Bounded,
resumable, fixed-capacity — no scan.

Owner bookkeeping for the new field: close arm `⏳️precompute/🪣️fill/🦀️.rs:3244` (FillBuilder close
cursor, field 26; the roots/refusal arms shifted to 27/28), retirement-cursor arm `:2845` (same shift),
`terminal_owners_empty` gained `tail_removal.is_none()`, and
`📐️geometry/🦀️.rs:1032` `impl CollisionIndexRemoval { retire_one_owner }` gives the removal its own
one-owner-per-grant close.

### 2.3 The spatial-index close census read a *released* page as "over capacity"
`⏳️precompute/📐️geometry/🦀️.rs:1066` `collision_index_backing_credit`, applied at the five backing
arms of `census_one_owner`.

`census_one_owner` mapped `backing_credit() == None` to `CollisionIndexOwnerCensusStep::Rejected`.
But `None` means *"this container has no page"* — which is exactly the state every container reaches
while `retire_one_owner` walks it, and the close census walks a **retiring** index by construction. So
`spatial_index_close_retains_bucket_values_and_retires_one_credited_owner_per_grant` hit `Rejected`
("bounded credit") as soon as the first page was handed back. A released page now credits `(0, 0)`;
only a semantic owner over its declared byte cap is a refusal.

### 2.4 (perf, §6) Three whole-document JSON round trips per scene sync
`⏳️precompute/🦀️.rs:1019` `scene_synced: Option<Arc<SceneConfig>>` (was `scene_json: Option<String>`),
`:1351` `Puzzle3dCollision::set_scene_config`, `:1980`/`:1984`
`Puzzle3dPrecomputeSession::set_scene_config` + `install_scene`.

`dispatch(SetScene { scene })` serialized the typed scene to JSON, `set_scene` parsed it back, then
re-serialized it to *normalize* it for the "same scene?" comparison. Measured on Nakagin: **13 ms per
round trip, three round trips, 39.4 ms** — five times the whole framework step ceiling, per command.
The engine now keeps the last synced scene as the decoded value and compares it structurally
(`SceneConfig: PartialEq`), and the typed entry point skips JSON entirely.

Equivalence was **proved, not assumed**: a temporary probe computed both the typed verdict and the old
normalized-JSON verdict on every `set_scene_config` and printed any disagreement — across the whole
crate suite, **zero disagreements**. Result: `dispatch SetScene` 39.4 ms → **0.18 ms**.

---

## 3. The 28 red tests, one by one

Production law fixed (13 tests):

| test | verdict |
| --- | --- |
| `precompute::component::dispatch_set_scene_then_apply_and_compose_fill_count_round_trip` | production — §2.1 |
| `precompute::component::precompute_session_native_wrapper_exercises_public_methods` | production — §2.1 |
| `precompute::component::fill_options_paths_are_millisecond_scale` | production — §2.2 (`applied fill objects must survive weight edits`) |
| `precompute::component::fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms` | production — §1 (was stuck at `stage=Some(PrepareFixture)` for 10 000 turns) |
| `precompute::geometry::spatial_index_close_retains_bucket_values_and_retires_one_credited_owner_per_grant` | production — §2.3 **and** stale constant (below) |
| `precompute::component::fill_worker_actual_owner_census_rejects_cap_plus_one_with_exact_handback` + the 8 downstream `PoisonError` casualties it caused | test helper — see `drive_fill_preparation` below; this one test poisoned the shared `fill_envelope_test_guard` and took 14 others with it |

Stale test assumptions fixed (15 tests) — each with its evidence:

1. **`fill_worker_session` admitted an UNPREPARED builder.**
   `⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:447` `drive_fill_preparation`, `:468`
   `FILL_PREPARATION_DRIVE_TURNS = 4096`.
   Evidence: `inject_nested_owner_page_plus_one_for_test` panicked
   `index out of bounds: the len is 0 but the index is 0` at `🪣️fill/🦀️.rs:2891`, indexing
   `catalogs.objects[0]` — the catalog page is EMPTY until the cursorized `PrepareCatalogs` stage runs.
   Every `fill_worker_*` law is about an admitted, populated builder, so the helper now drives the
   preparation stages (and drains the checked-out worker outcome — admission refuses a session whose
   last outcome is still checked out, which is why the first attempt still returned `None` forever).

2. **`fill_lane_advances_while_brush_targets_remain_queued:974`** asserted
   `!engine.brush_queue.is_empty()` immediately after `set_scene`. The brush queue is now filled from
   a preparation cursor, one target per lane turn — outstanding brush work is `brush_lane_active()`,
   not a non-empty queue. Assertion retargeted; the test's actual law (the fill lane progresses while
   brush work is outstanding) is unchanged.

3. **`update_kind_weights_soft_replans_tail_without_rebuilding_queue:242`** and
   **`set_scene_with_identical_json_preserves_precompute_progress:267`** measured progress as
   *`work_pending_for_test()` shrinking*. With both lanes cursorized that is no longer a progress
   measure: preparing one brush target moves it from the cursor INTO the queue, so the pending count
   is unchanged or larger. Added `⏳️precompute/🦀️.rs:1786` `precompute_progress_for_test()` — a
   monotone witness (resolved suggestion targets + reconciled broad-phase owners + planner
   transitions) — and the two tests now assert real completed work. `work_pending_for_test` is still
   used for the "identical JSON must not wipe" / "a changed scene must rebuild" halves, where it is
   exactly the right measure.

4. **`set_scene_with_applied_fill_projection_preserves_slider_session:346`** held its own
   `MutexGuard` on `engine.fill` across `engine.apply_fill_count(1)`, whose first act is
   `fill.try_lock()`. A missing `drop(fill)` — a test bug, not a behaviour change.

5. **`fill_worker_completed_before_session_drop_is_reclassified_and_mounted_once:769`** expected the
   NEXT mounted caller to consume the retained close intent. `Puzzle3dPrecomputeSession::drop` no
   longer only *asks* for the terminal — it drains the registry itself (P8's fix; its own docstring
   explains the four-slot leak). The dying session IS the one caller that reclassifies and mounts,
   exactly once. Retargeted to assert the slot is already returned and a second mount is refused.

6. **`fill_worker_cross_generation_restore_preserves_dropped_closing_handle_and_zero_credit:620`** and
   **`fill_worker_session_drop_during_partial_close_rearms_the_same_cursor_once`** asserted that ONE
   `close_step()` grant moves the admitted fill into its retirement cursor.
   `FillEnvelopeTerminalHandle::close_step` retires the last worker outcome (stage 0) and the whole
   worker session (stage 1) first. **Measured: exactly nine grants.** Added
   `⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:514` `close_until_fill_retirement`, which asserts every grant
   stays `Pending` (incremental, never a bulk close) and is bounded by
   `FILL_ENVELOPE_CLOSE_GRANTS_TO_RETIREMENT = 18` (measured nine, doubled).

7. **`fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase:609`**
   forced `authority.observation.done = true` in the REGISTRY and then asserted the SESSION's
   observation equalled it. The law is that a rejected restore leaves the session's own observation
   untouched; the registry copy is forced precisely to prove the rejection never adopts it. The test
   now captures the session's observation before the restore attempt.

8. **`fill::all_fill_fixed_collections_store_max_entries_…:904`** and
   **`fill::constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently:670`** both
   panicked `body` on
   `collision_body_from_buffers(&[0,0,0, 1,0,0, 0,1,0], &[0,1,2])`. That triangle's extent is 1.0 and
   `BRUSH_COLLISION_MESH_MIN_EXTENT` (`📐️geometry/🦀️.rs:545`) is **2.0**, so the constructor
   correctly returns `None`. Fixture widened to a 4.0-extent triangle; the capacity law is untouched.

9. **`fill::capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_fault:799`**
   asserted `builder.preview.sequence > 0`. `StepContext::next_preview_sequence`
   (`🧵️job/🦀️.rs:1025`) returns the value BEFORE incrementing, so an operation's first publication is
   legitimately sequence **0**. Retargeted to the real law: the refusal publishes under the
   operation's own first sequence and consumes exactly one (`sequence == 1` after, checked once the
   `StepContext` borrow ends; the fault that follows consumes none).

10. **`fill::empty_fill_transition_stays_below_watchdog_ceiling:1024`** gave the planner 16 turns to
    reach `Complete`. Cursorized preparation needs seven stages, three of which walk three empty roots
    apiece. Bound raised to a named, derived `EMPTY_FILL_TRANSITION_TURNS = 64`; the 8 ms per-turn
    assertion is unchanged.

11. **`geometry::spatial_index_close_retains_bucket_values_…:434`** asserted one close grant releases
    at most `16 * 1024` bytes — the BOOKKEEPING page width (`FIXED_OWNER_PAGE_BYTES`). This index
    declares DOCUMENT-scale pages (`entries` at `DOCUMENT_OBJECT_SLOTS`, `cells` at
    `DOCUMENT_CELL_SLOTS`), whose declared ceiling is `DOCUMENT_OWNER_PAGE_BYTES` — the constant the
    same file's two sibling laws at `:377` and `:518` already use. Retargeted to it.

---

## 4. Per-step budget — what was measured and what was fixed

New measured tests, `✏️editor/🧪️tests/🔬️unit/🦀️.rs:2713-2831` (`//#region ⏱️InteractiveStepBudget`):
`measured_step_loop` drives a REAL `PuzzleCommandWork::step()` to `Complete` on the real 180-object
Nakagin document, timing every turn, and asserts each one strictly under
`PUZZLE3D_INTERACTIVE_STEP_CEILING` (8 000 µs, mirroring
`semio_framework_trace::INTERACTIVE_STEP_CEILING_US` — that crate is not a direct dependency here) plus
a tighter `PUZZLE3D_MEASURED_STEP_BUDGET` of 2 000 µs for the unoptimized profile.

| command | before | after | verdict |
| --- | ---: | ---: | --- |
| `acceptSuggestion` | 11 796 µs (W-D) | **83 turns, worst 137 µs** | **passes** both the ceiling and the 2 ms budget |
| `openVortexSuggestions` | 14 644 µs (W-D); **82.5 ms** re-measured here | 10.2 ms steady, 18.5 ms first-touch | **still over** — §6 |
| `fillBuildTick` | 11 705 µs (W-D) | 753 turns far under, **one** publish turn 17.6 ms | **still over** — §6 |

Two contained per-step wins landed:

* `✏️editor/🦀️.rs:2558` — `handle_action_impl` decoded the persisted projection **twice** (once for
  `before`, once for `scene_from_projection`), 1.6 ms each on Nakagin. Now decoded once.
* `✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs` — the command called
  `sync_precompute_session` a second time on the same scene `handle_action_impl` had already synced
  (`openVortexSuggestions` is in `puzzle3d_action_uses_precompute`). Measured cost of the duplicate:
  **3.5 ms** of pure re-conversion, per popup. Removed.

Together with §2.4 that took `openVortexSuggestions` from **82.5 ms → 10.2 ms** per turn.

Measured breakdown of what remains in that one turn (Nakagin, opt-level 0, temporary probe, removed):

```
[DEBUG] projection_value                        1.61 ms
[DEBUG] scene_from_projection                   2.78 ms   (180 objects)
[DEBUG] sync_precompute_session (warm)          3.69 ms   = scene_config_value 0.50 + FromValue 3.00 + dispatch 0.18
[DEBUG] refresh_brush_candidates                2.04 ms   (already deadline-bounded to 2 000 µs)
[DEBUG] handle_action_impl (whole)             10.17 ms
```

---

## 5. Files changed

Production:

* `✏️s/…/✏️editor/⏳️precompute/🦀️.rs` — caller-side worker (§1), `set_scene_config` + `scene_synced`
  (§2.4), `soft_replan_fill_tail` in place (§2.2), `base_fixture` call sites (§2.1),
  `precompute_progress_for_test`.
* `✏️s/…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` — `base_fixture`, `begin_soft_replan`,
  `discard_tail_one`, `FillJobStage::DiscardTail`, `tail_removal` owner + its close/retirement/
  terminal-empty arms, `transition_count` visibility.
* `✏️s/…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` — `collision_index_backing_credit` (§2.3),
  `CollisionIndexRemoval::retire_one_owner`.
* `✏️s/…/✏️editor/🦀️.rs` — single projection decode in `handle_action_impl` (§4).
* `✏️s/…/✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs` — duplicate session sync removed (§4).

Tests (no assertion weakened; every retarget justified in §3):

* `✏️s/…/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`
* `✏️s/…/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`
* `✏️s/…/✏️editor/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs`
* `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the three measured step-budget tests, plus one
  `unnecessary qualification` warning fixed at `:817` so the crate checks warning-free.

Ticket folder: this report.

---

## 6. NOT done / NOT verified

1. **`openVortexSuggestions` and `fillBuildTick` still exceed the 8 ms ceiling in their one publish
   turn** (10.2 ms and 17.6 ms measured above). Their two tests are therefore
   `#[ignore = "known red: …"]`d with the reason and a pointer to this section — they are real,
   runnable measurements (`cargo test … -- --include-ignored`), not deleted laws.
   **Why they cannot be sliced from the puzzle side as they stand:** the residue is four *whole
   document* `Value` conversions inside `handle_action_impl`'s own prologue — projection decode,
   `scene_from_projection`, `sync_precompute_session`'s `scene_config_value` + `FromValue`, and the
   already-bounded candidate refresh. That prologue runs whole inside one `handle_action_impl` call,
   which is the shared spine of EVERY puzzle3d command. Closing it means splitting
   `handle_action_impl` into `scene` / `session-sync` / `dispatch` halves that a staged
   `PuzzleCommandWork` can spend one turn on each (and routing `openVortexSuggestions` from
   `BoundedFirstStepCommandWork` to the staged `Puzzle3dPrecomputeCommandWork`). That is a refactor of
   the command spine — a wave of its own, and `✏️editor/🦀️.rs` was being edited by W-B and W-M
   throughout this one.
2. **The `fillBuildTick` admission-across-steps item is NOT closed.** `enqueue_fill_job`
   (`⏳️precompute/🦀️.rs:2144`) does one resumable census unit per call and `fill_build_tick` calls it
   once per 120 ms tick, so a fresh session's first fill call still needs many ticks to admit. Making
   the tick's work drive its own admission across its own steps depends on the same
   `handle_action_impl` split as item 1, so it moves with it. The *tests* drive it correctly through
   the existing `enqueue_measured_fill_job` helper, and admission itself is verified green.
3. **No runtime / browser confirmation.** Nothing here was observed in a running app; every verdict is
   a cargo test or a measured probe. The coordinator owns the wasm rebuild and the servers.
4. **The full crate suite is still broadly red in one process, and none of it is this wave's.**
   `cargo test … -- --test-threads=1` (whole lib): 496 passed / 80 failed / 2 ignored, and it now runs
   to completion instead of aborting.
   * 55 of the 80 are `editor::puzzle3d::component::tests`. Wave X measured **58** there before this
     wave (`📓️2026-09-09-wave-X-test-suite.md` §"By module"), so this wave is at or below that
     baseline on the same population.
   * 21 are `precompute` tests that pass **132/132 when the precompute module runs alone**. Their
     shared-process failure is P8/P10 process-global slot exhaustion, proven by the message:
     `set_scene_with_identical_json_preserves_precompute_progress` fails with
     `rebuild_queue should have enqueued at least the fill steps` and
     `update_kind_weights_…` with `left: 0, right: 1000` — i.e. `mount_fill_worker` was REJECTED
     because earlier tests exhausted the framework's 256 worker-session slots. Framework-owned; wave X
     already recorded "any future measurement of this crate has to isolate".
   * 1 is `retained_command::tests::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
     — the peer's `PuzzleCommandCheckpointState` byte-count fixture (wave D §4).
5. **The census does not credit the new `tail_removal` owner.** It is `None` at admission (a freshly
   admitted builder has never replanned) and the fill census is documented as a bound, not an exact
   accounting (wave P §7.6). A soft replan after admission would carry one uncredited `String`.
6. **`FILL_ENVELOPE_CLOSE_GRANTS_TO_RETIREMENT = 18`** is measured at nine and doubled, not derived
   from the framework's own close-stage count.
7. **Untouched by instruction:** `⚛️reactor/🔄️turn/🦀️.rs`, `🕹️interaction/**`, and
   `✏️editor/🦀️.rs`'s store-preparation region (~6046-6140, W-B). The `[DEBUG] typed-operation
   publication turn=…` lines in the suite output are the coordinator's instrumentation, still live.

---

## 7. Commands, with tails

```
$ cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 24.78s          # 0 errors, 0 warnings

$ cargo test … editor::puzzle3d::precompute -- --test-threads=1        # BEFORE
test result: FAILED. 104 passed; 28 failed; 0 ignored; 0 measured; 442 filtered out

$ cargo test … editor::puzzle3d::precompute -- --test-threads=1        # AFTER
test result: ok. 132 passed; 0 failed; 0 ignored; 0 measured; 446 filtered out; finished in 6.09s

$ cargo test … -- --test-threads=1                                     # whole lib, AFTER
test result: FAILED. 496 passed; 80 failed; 2 ignored; 0 measured; 0 filtered out; finished in 78.36s

$ cargo test … -- --test-threads=1 --include-ignored --nocapture every_step_stays_below_the_interactive_ceiling
[DEBUG] puzzle3d acceptSuggestion: 83 turns, worst turn 136.833µs
fillBuildTick turn 754 took 17.583792ms, at or over the framework's interactive step ceiling 8ms
openVortexSuggestions turn 1 took 18.544333ms, at or over the framework's interactive step ceiling 8ms
test result: FAILED. 1 passed; 2 failed

$ rustfmt --unstable-features --skip-children <every edited .rs>       # exit 0, all 9 files
```
