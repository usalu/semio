# 📓️ Wave W1-B: Puzzle 3d Fill as a Framework Tool Run

Lane W1-B of `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`. The lane moves the puzzle 3d fill from a tick command, a session registry and preview JSON onto the framework ToolRun ledger. It also closes every W1-F red→green row.

`E3` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`, and `T` = this ticket folder. Logs are in `T/🗑️generated/W1-B/`.

## 1. Result

- **Manifest:** fill is declared as a mutating `instance3d` ToolRun: `rebase: revalidate`, `reconfigure: resume`, with stages, counters and EN/DE reasons. The framework builds its run and revalidate jobs through `ArtifactEditor::build_tool_run_job`.
- **Removed:**
  - the tick, cancel-toggle and preview paths (`fillBuildTick`, `cancelFillBuild`, the `fillBuild` interaction block, the tried ring, the 16 KiB preview caps);
  - the process-global fill session and the `fill-count` amend key.
- **Count:** it stays a `WindowMeasure::Number`, and a `setFillCount` is config only.
- **Escape:** it maps to `toolRunAbort` while a fill run is non-terminal.
- **Checks:**
  - every W1-F row is green at `--test-threads=4` and at `=1`;
  - the W1-D probe reports no puzzle 3d fill finding;
  - the 3d crate, the 5d crate (native and wasm32-wasip2) and the wgpu tests all compile.

## 2. Changes as Landed

### 2.1 Fill Tool: `E3/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (rewritten)

- **Job kinds** (`:24`, `:26`):
  - `RUN_JOB_KIND = "s.puzzle.puzzle3d.fill.run"`
  - `REVALIDATE_JOB_KIND = "s.puzzle.puzzle3d.fill.revalidate"`
- **Declaration:**
  - `definition(label)` (`:31`) is `ToolDefinition { run: Some(run_definition()), .. }`.
  - `run_definition()` (`:37`) sets `mutating: true`, `ToolRunRebasePolicy::Revalidate`, `ToolRunReconfigurePolicy::Resume`, `ToolRunTraceKind::Instance3d`, `run_job` and `revalidate_job`.
- **Count measure:** `count_measure(envelope, labels, tool_run)` (`:56`) and `measures(..)` (`:74`). The `Number` entry has no maximum, and `loading` is set only while a fill run is live.
- **Abort:** `abort_action(tool_run)` (`:84`) returns `toolRunAbort {runId: string, generation}` for a live run, or `None`.
- **Job builder:** `build_run_job(ToolRunJobRequest)` (`:93`) works as follows.
  - It rebuilds the scene from the request snapshot plus config.
  - It builds the lane with `main::mesh_lane`.
  - `Run` decodes the checkpoint; `Revalidate` rebuilds the placements through `fill_run_placements`.
- **`Puzzle3dFillToolRunJob`** (`:139`) is a bounded `InteractiveJob`.
  1. **Preparing** installs one mesh identity per unit, from the shared derived store or the scaled box fallback.
  2. It digests base revision, overlap, seed, weights and mesh sources into the 32-byte `inputs`.
  3. It then becomes `FillRunJob::start(..)` or `FillRevalidateJob::new(..)`.

### 2.2 Planner: `E3/⏳️precompute/🪣️fill/🦀️.rs`

- **Construction:** `FillRunJob::start(roots, operation, identity, lane, inputs, requested, checkpoint, provisional)` (`:2591`) is the one production construction path. It picks one of three branches:
  - replay (`:2617`), when the checkpoint `inputs` match;
  - restart (`:2607`), when there are provisional ops or `generation > 0`;
  - otherwise a fresh run.
  - This also keeps `FillBuilder::begin_preparation(` inside the planner file, as the P4e predicate requires.
- **Capacity refusal:** `capacity_outcome(context, writer)` (`:2253`) publishes a `ToolRunStepKind::Danger` step `DocumentCapacity` once, then faults with `preparation-capacity:{branch}:{cap}`.
- **Replay:** `settle_replay` (`:2891`) is the silent replay to a checkpoint.
  - On arrival it rebases the writer onto the ledger's provisional ops and retracts past the checkpoint.
  - An overshoot restarts with `clear_trace` and `retract_to(0)`.
- **Revalidation helpers:** `FILL_REVALIDATE_KEY_BASE = 1 << 63` (`:2433`) and `fill_run_placements(provisional, lane)` (`:2438`).
- **Deleted:**
  - the preview, census and retirement cursor, `retire_fill_preview`, the tried ring and the preview JSON;
  - `snapshot`, `base_fixture` and `begin_soft_replan`;
  - `inject_nested_owner_page_plus_one_for_test` and the builder `fixed_backing_witness_for_test`;
  - the `FillRunJob` methods `identity`, `rebind` and `stage`.
- **Renamed:** `FillJobStage::DiscardTail` is now `RetractTail`, with label `"retract-tail"`. The old `"discard-tail"` literal tripped the local-lifecycle predicate.
- **Test-only API:** these are now `#[cfg(test)]`, because production uses replay rather than resident resume:
  - `builder`, `operation`, `mesh_lane` and `provisional_placements` on `FillRunJob`;
  - `resume` and `FillRunResumeError`;
  - `conflicts` on `FillRevalidateJob`.

### 2.3 Precompute and Geometry

- **`E3/⏳️precompute/🦀️.rs`** is now brush only.
  - Deleted: the fill job bridge, `initialize()`, the fill session, `FillApplyChunk` and the lane enum.
  - `precompute_step(budget)` is brush only.
  - `Puzzle3dCollision::set_scene` (test-only now), `scene_is_synced` (deleted) and `brush_candidate_cache_len` (now `#[cfg(test)]`) are no longer used in production.
- **`E3/⏳️precompute/📐️geometry/🦀️.rs`:**
  - The owner census (`CollisionIndexOwnerCensus*`, `census_one_owner`, the credit helpers, `backing_credit`, `values`, `truncated`) is now `#[cfg(test)]`. It is kept as the measuring instrument of the spatial-close law.
  - `CollisionShape` lost `retained_items`, `retained_bytes` and `page_bounded`.
  - `CollisionBody::retained_part(s)_credit` is deleted.

### 2.4 App: `E3/🦀️.rs`

- **Tool-run hook:** `ArtifactEditor::build_tool_run_job` (`:7696`) delegates to `fill_tool::build_run_job`. Finalize uses the existing `Puzzle3dArtifactStorePreparationFactory` (`:7279`).
- **`setFillCount`:**
  - It is Decode → Publish with a config-only Emit (`:6788`).
  - Its publication contract is `Config`.
  - Its scope class is `FillOptions` (`:2248`).
  - It is now declared as a `view_action` (`:8348`), like `setObjectKindWeight`. It no longer edits the document.
  - This fixes `command_scope_classes_name_the_panels_they_change`.
- **Removed from the app:**
  - `Puzzle3dFillSession`, `"fillBuildTick"` and `"cancelFillBuild"` everywhere (dispatch arms, contracts, proofs, job arms, interactive-job and view declarations, retained tool ids);
  - the FillLock stage, `puzzle3d_operations_from_values` and `with_puzzle3d_app_mut`.
- **Added:** `puzzle3d_fallback_mesh_buffers()` (`:1565`). `collect_mesh_urls` is now sorted.
- **Tool run view:** tool measures and window engagements receive `doc.tool_run()`.

### 2.5 Main Window: `E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`

- Added `VORTEX_MARKER_MESH_KIND` (`:568`) and `mesh_lane(fixture)` (`:575`). `meshesJson` is published as `[box, vortex-marker, sorted deduped urls]`, which is exactly the lane the trace `mesh` field indexes.
- `engagement(envelope, labels, tool_run)` (`:970`) sets `on_abort` from `fill_tool::abort_action`.
- Removed: the `fillBuild` JSON, `world_fill_preview_json` and the `labels` parameter of `render`.

### 2.6 Commands

- **Deleted:** `🎮️commands/🪣️fill-build-tick/`.
- **`🧮️set-fill-count`:** only `request(count)` and `parse_count(args)` remain.
- **`🛑️engagement-abort`:** when the fill tool is active it dispatches `toolRunAbort` with the renderer-echoed trace cursor (`tool_run_trace_cursor_by_window_id[window]`), plus `SetActiveTool ""`.
- **`📨️engagement-submit`:** `fill n` sets the count, then dispatches `toolRunStart {toolId: fill}`.
- **`engagement-repeat-last`:** uses `fill_tool::TOOL_ID`.

### 2.7 Terminology, Schema and Fixtures

- **`E3/🗣️terminology/🦀️.rs`:** the `🔖️FillRun` region adds `puzzle3d_fill_run_unit`, `puzzle3d_fill_run_stages`, `puzzle3d_fill_run_counters` and `puzzle3d_fill_run_reasons` (`:163`–`:191`). The reasons table has 23 EN/DE entries with verdict literals. A law test compares it with the schema `x-semio-toolRun` table.
- **`✳️any/🧬️schema/🦀️.rs` and `🔣️.json`:**
  - Removed: `reveal_index`, `PrecomputeLane`, `FILL_TRIED_RING`, the `FillTried*` / `FillBuildPreview` / `FillBuildProgress` / `FillProgressSummary` types, `ApplyFillCount`, `ComposeFillDisplay` and every `Puzzle3dFillPreview*` `$def`.
  - `FillRunCheckpoint` (`:870`) is 68 bytes: `requested u64 | placements u64 | provisionalOps u32 | tested u64 | nextKey u64 | inputs bytes32`.
- **Fixtures:**
  - `🪣️fill/🧫️fixtures/🔣️.json` is deleted, and its `documentCapacities` moved into `🎞️fill-run.json`.
  - `🧫️fixtures/🗄️retained-jobs/🔣️.json` no longer contains `fillBuildTick` or `cancelFillBuild` (in `toolIds`, `evidenceToolIds`, `semanticCursors` or vectors).

### 2.8 TypeScript Leftovers

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`: `"fillBuildTick"` is dropped from the framework action set, and the interval docstring is reworded.
- `…/🏛️ShellHost/🟦️.tsx`: `interactiveAction` only exempts `suggestionsTick`.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`: the comment now reads "polling ticks".

## 3. Tests

### 3.1 W1-F Red→Green Table

All rows live in `E3/🧪️tests/🔬️unit/🦀️.rs` or the fill planner tests.

| Row | Status |
|---|---|
| **Nakagin tick law.** `fill_build_tick_every_step_…` is deleted. Its successor `fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin` checks both the worst job step and the worst `fold_overlay_op` append under 2 ms, on the same Nakagin document. | ✅ passed at 4 and 1 threads |
| **Cancel.** `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run` checks that abort is present while starting and after the first provisional piece, that the run reaches `aborted`, that no provisional piece remains, that the document is byte-identical, and that no undo entry is left. | ✅ |
| **Escape.** `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one` checks that Escape yields `toolRunAbort` with the run id and disarms the tool, that the state is `aborting` inside the call, that teardown takes at least one driver turn, and that the document is unchanged. | ✅ |
| **Count.** `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count` checks the default of 100, that the count is config only, and that a change during a run keeps the same `runId` with a higher `generation`. | ✅ |
| **Finalize.** `fill_build_tick_locks_…` is deleted. Its successor `fill_run_finalize_publishes_one_edit_with_every_provisional_placement` checks that a complete run commits nothing, that the viewport shows committed ⊕ provisional, that finalize publishes every placement, that one undo removes all of them and that one redo restores them. | ✅ at 4 and 1 threads |

### 3.2 Added Tests

- **Brief obligations:**
  - Start → complete → finalize is one undo entry, and undo after finalize removes every piece: covered in `fill_run_finalize_publishes_…`.
  - Abort leaves the document byte-identical: covered in `cancelling_…`.
  - Raising and lowering the count during a run: `raising_and_lowering_the_fill_count_during_a_run_continues_and_retracts_the_same_sequence` (2 ⊂ 4 ⊂ 6, one run id, finalize commits the lowered count).
- **Declaration and scope:** `the_fill_tool_declares_its_tool_run_through_the_manifest` and `set_fill_count_declares_the_fill_options_ui_scope`.
- **Main window:** `world_interaction_json_carries_no_fill_run_state` and `meshes_json_is_published_in_exactly_the_mesh_lane_order_trace_subjects_index`.
- **Planner:**
  - `fill_run_job_rebuilt_from_a_checkpoint_replays_silently_and_continues_like_the_resident_job`
  - `fill_run_job_capacity_refusal_publishes_a_danger_step_before_faulting`
  - `fill_run_placements_rebuild_the_provisional_placements_from_their_ops`
- **Terminology:** `the_fill_run_vocabulary_equals_the_schema_table_and_is_authored_in_english_and_german`.

### 3.3 Commands and Counts

All commands run from the repo root in the foreground. Every run below was repeated after the last edit.

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --lib --tests` | ok, 0 errors; lib warnings went from 110 to 88 | `check-3d-native-3.txt` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --target wasm32-wasip2` | ok, 0 errors | `check-3d-wasm.txt` |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- fill --test-threads=4` | **48 passed, 0 failed** | `test-fill-t4-2.txt` |
| same with `--test-threads=1` | **48 passed, 0 failed** (169 s) | `test-fill-t1-2.txt` |
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly -j 4 --lib --tests` | ok | `check-5d-3.txt` |
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly -j 4 --target wasm32-wasip2` | ok | `check-5d-wasm.txt` |
| `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -j 4 -- fill` | 3 passed, 0 failed | `test-5d-fill.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --tests -j 4` | ok | `check-wgpu.txt` |
| `cargo test … puzzle-3d --lib -- retained engagement geometry terminology scope windows::main precompute::tests brush distribution publication set_fill --skip fill_run --test-threads=4` | 140 passed, 3 failed (see §5) | `test-touched-t4-2.txt` |
| `bun T/🐍️w1d-policy-probe.ts` | see below | `probe-final.txt` |

The W1-D probe results:

- The self-tests pass: tool-run 38, run-job 32, trace 13, p4e 27.
- `p4e`, `run-job` and `trace` report no failures.
- `amend`, `legacy-trace` and `reserved-action` report 0 findings.
- `local-lifecycle` reports 5 findings, all in generation3d `preview-eval` (lane W3-2).
- `declaration` reports 25 findings. None is puzzle 3d fill: the 5d fill row belongs to W2-B, and the rest are unassigned or other lanes.

**launch.json:** `.vscode/launch.json` has no entry for these crate checks, crate tests or the W1-D probe. Registering them is outside this lane, because the lane rules forbid editing launch.json.

## 4. Deviations

1. **Replay instead of resident resume.** The driver closes the job on `SettingsChanged` and rebuilds it with a checkpoint. The rebuilt job replays silently to the checkpoint when `inputs` match, and restarts when they do not. `FillRunJob::resume` survives only as test API.
2. **Capacity refusal.** It publishes one Danger step, yields, and faults on the next step, so the step reaches the panel before the terminal fault.
3. **Revalidation keys.** They use `1 << 63 | index`, because `ToolRunJobRequest` carries no trace keys.
4. **Checkpoint size.** `FillRunCheckpoint` grew from 36 to 68 bytes (`inputs: bytes32`), and schema and fixture are updated.
5. **`setFillCount` action kind.** It is now a `view_action` rather than a Mutation, because it only writes config.
6. **Precompute test API.** `Puzzle3dCollision::set_scene` is `#[cfg(test)]`, since only precompute tests parse JSON scenes.

## 5. Foreign Edits

**Outside the owned list**

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs`: removed the `fill_build_tick` module declaration.
- `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs`: removed the `precompute::initialize()` call.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️wire-format-guard/🦀️.rs`: removed the fill engine command rows.
- `…/✳️any/🧬️schema/🧪️tests/🔬️precompute-model/🦀️.rs`:
  - removed `reveal_index` and the dead `fill_plan_object`, `fill_plan_attraction` and `fill_plan_payload`;
  - the brush test fixtures in the same folder lost `reveal_index`.
- `E3/🧪️tests/🔬️mutation-latency/🦀️.rs`: switched to `precompute_step`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json`: dropped `fillBuildTick` and `cancelFillBuild`.
- `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json`, `Puzzle3dPlayApp` entry:
  - dropped `cancelFillBuild` from host-only and `fillBuildTick` from artifact;
  - moved `setFillCount` from `artifact+config` to `config`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs`: removed the `cancel_fill_build` module declaration.

**5d crate: minimal changes (W2-B owns the proper conversion)**

5d fill now has no planner until W2-B wires the 3d `runJob` in.

- **`…/🖐️5d/…/✏️editor/🧠️precompute/🦀️.rs`:** removed `Puzzle5dFillProgress`, `fill_preview_object_kind`, `fill_progress`, `fill_requested_count`, `set_fill_requested_count`, `fill_job_identity`, `cancel_fill_job_for`, `fill_preview_json_page` and both `apply_fill_count(_rust)` impls (native/p2 and browser wasm).
- **`…/🎮️commands/🛑️cancel-fill-build/`:** deleted.
  - In the same crate, `✏️editor/🦀️.rs` lost the `CancelFillBuild` enum variant, the dispatch arm, the `ActionDefinition`, the interactive-job line and the import.
- **`…/🎮️commands/🧮️set-fill-count/🦀️.rs`:** only records `runtime.fill_count` and arms `fill`. It no longer touches the document.
- **`…/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs`:**
  - only the count `Number` remains, with no ready or loading state;
  - `fill_cancel_measure` is removed;
  - `measure(envelope, labels)` lost its precompute parameter, and both window callers were updated.
- **`…/🪟️windows/🧊️3d/🦀️.rs`:** removed `world_fill_preview_json` and `part_color`; `render(envelope, precompute)` lost `labels`, and both `✏️editor/🦀️.rs` callers were updated.
- **`…/🧪️tests/🔬️unit/🦀️.rs`:** deleted `set_fill_count_carries_a_large_count_and_retargets_the_planner`, `fill_cancel_toggle_reflects_the_wrapped_session_summary` and `cancel_fill_build_pins_the_count_to_what_is_locked`.
- **Left for W2-B:** the now-unused 5d labels `fill_cancel`, `fill_locked`, `fill_stage_*` and `fill_stall_*`, plus `puzzle5d_fill_stage_label` and `puzzle5d_fill_stall_reason_label` in `🗣️terminology/🦀️.rs`.

## 6. Open Items

- **Pre-existing brush failures (W2-C owner, already reported in `📓️wave-W0-C2.md`):**
  - `precompute::brush::tests::brush_search_progress_counts_blocked_candidates_as_a_verdict_not_a_silence`
  - `precompute::brush::tests::brush_ghost_json_carries_the_verdict_of_the_candidate_it_shows` (same "blocker does not collide" cause)
- **Timing-sensitive suggestion tests:**
  - `hover_suggestion_updates_the_brush_candidate_index_and_live_preview` failed once at `--test-threads=4`.
  - `the_tick_rewarms_the_latched_brush_target_after_an_edit_invalidated_its_candidates` failed once at `--test-threads=4` and passed on the rerun.
  - Both pass alone at `=1` (`test-suspects-t1.txt`). They are wall-clock bounded brush slices.
- **Publication authority audit.** `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit Puzzle3dPlayApp` currently aborts before the owners are audited: `Puzzle2dWindowConfig` still lists `fillCount`, from W2-A's in-flight 2d edit. Rerun it once W2-A lands.
- **Stale `fillBuildTick` references outside this lane:**
  - `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` (generated plugin manifest snapshot, lines ~15829 and ~15929);
  - `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1026` (comment);
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:278-312` (comments);
  - the root `📜️script.ts` `INTERACTIVITY_TOOL_RUN_REQUIREMENTS` fill row still lists scope `🪣️fill-build-tick` and action `fillBuildTick`. This is harmless but stale (W1-D owner).
- **Provisional stamp.** Instance records are not stamped `provisional: true` from `doc.tool_run().provisional_entities`. The overlay renders provisional pieces as ordinary instances; the testing and verdict styling comes from the W0-E trace layer.
- **Pre-existing debug logs, not from this lane:** `[DEBUG]` `eprintln!`s remain in `E3/🦀️.rs` (openImport, prologue, utility.publish) and in `🧊️main/🦀️.rs:939` (vortices.publish).

## 7. API Gaps (for W0-H; no edits were made to `🔌️plugin/🦀️.rs` or `🔌️plugin/⏯️tool-run/**`)

1. **Stale ticks after a base change.** A running job is not rebound on `BaseChanged`, so the ticks it emits carry a stale identity. A `rebind` hook, or a close and rebuild, is needed.
2. **No tool run in command context.** Commands (`Puzzle3dActionCtx`) cannot read `ArtifactView::tool_run()`, so Escape uses the renderer-echoed trace cursor instead.
3. **Chords lose the run identity.** Keyboard chords carry no run identity, so a chord-dispatched `toolRunAbort` without args is a stale no-op. The framework should resolve the instance's live run when args are absent.
4. **No trace access at build time.** `ToolRunJobRequest` exposes no trace or placement keys, which forces revalidation into the `1 << 63` key space.
5. **Too-broad reconfigure trigger.** `SettingsChanged` fires on every window-config publication, including camera, and each one forces a close plus replay of a live run. It should fire only when the config read by the run's job changes.
6. **Ledger not readable from app tests.** `tool_runs` is `pub(crate)`, so the tests read state through `PluginApp::tool_run_presence()` and engagement `on_abort` instead.
