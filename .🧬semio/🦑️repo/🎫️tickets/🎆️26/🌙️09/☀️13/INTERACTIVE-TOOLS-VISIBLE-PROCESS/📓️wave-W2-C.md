# 🖌️ Wave W2-C: Puzzle 3d Brush Suggestions as a Read-Only Tool Run

Lane W2-C of `📋️tool-run-contract.md` (§2.4, §3.2, §3.7, §5 W2-C), following `📋️wave-3-lane-brief.md`.
Status: **landed.** The lane's own suites are green, the crate compiles native and on wasm32-wasip2, and the React suites are green.

Paths:
- `E3` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `A` = `…/✳️any`
- `WH` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `T` = this ticket folder

Logs are in `T/🗑️generated/W2-C/`.

## 1. Result

- **Brush is now a read-only run.** The brush utility declares a `mutating: false` `instance3d` ToolRun (`rebase: restart`, `reconfigure: restart`). The run reads the settings `/overlapBudget`, `/objectKindWeights` and `/vortexKindWeights`.
- **One job serves all vortices.** `BrushSuggestionsRunJob` follows the instance's *brush suggestions link* instead of being built per vortex.
  - The link holds the vortex the user points at: an open popup wins over the armed brush's hover.
  - Every compatible candidate becomes a trace record: `testing`, then `success/free` or `danger/collision` (or `warning/pose-unavailable`).
  - One fuel unit is one candidate verdict.
  - The verdicts are also published to the link, which the popup, cycle and accept read.
  - A new target clears the trace and searches again. A settled search waits on its `ToolRunJobPort`, so it costs no turns.
- **Starting and aborting.** `ArtifactEditor::pending_effects` (`utilities::brush::run_effects`) compares the link with `doc.tool_run()`:
  - hovering a vortex or opening a popup → `toolRunStart {toolId: brush}`;
  - a new target while live → wake;
  - leaving the vortex or closing the popup → `toolRunAbort {runId, generation}`.
  - Each request is latched until the run view answers it, so a refresh never repeats it and an abort-then-restart cannot race.
- **Accepting stays one-shot.** `acceptSuggestion` places the indexed *free* candidate the run found, with its real brush pose and source vortex. That is one edit and one undo.
- **Deleted:**
  - the `suggestionsTick` command and loop, including host interval and hover tick;
  - the `BrushSearchProgress` readout plumbing (schema type, `FillCandidateVerdict`, precompute readers, `advance_brush_search`, `brush_live_target`, popup `progress` block, placement picker measure, captions and labels);
  - the popup warm stage;
  - the brush ghost path in `WH`, plus its host-side leftover-refresh subsystem in ShellHost, PluginRuntime and ShellHelpers (see §5);
  - the `[DEBUG]` traces on these paths.
- **Real defect fixed** (the two red brush precompute tests):
  - Placed bodies were resolved **by kind** (the first fixture object of that kind), not by the object's own `meshUrl`. A body carrying its own mesh therefore never became a collision body, and every candidate docking into it read "free".
  - New `resolve_placed_object_mesh_url` gives own mesh first, then kind, which is the renderer's law. It is used by the run and by the precompute lane that puzzle 5d still reads.
  - The two old tests tested deleted API; their successor laws are green (§3).

## 2. Changes

### 2.1 Schema-first vocabulary

- **`A/🧬️schema/🔣️.json`:** new `$defs` `Puzzle3dBrushSuggestionsRun` (with an `x-semio-toolRun` table), `…RunStage` and `…RunCounter`.
  - Stages: `prepare`, `target`, `test`, `idle`.
  - Counters: `tested`, `free`, `collisions`.
  - Reasons: 0 `free` (success), 1 `collision` (danger), 2 `pose-unavailable`, 3 `target-missing`, 4 `suggestions-blocked` (warning), 5 `search-complete` (success, args `[free, tested]`).
- **`A/🧬️schema/🦀️.rs:631`:** `BrushSuggestionsRunStage`, `BrushSuggestionsRunCounter` and `BrushSuggestionsRunReason`, each with `ALL`, `index`/`code`, `id` and `verdict`. `BrushSearchProgress` and `FillCandidateVerdict` are removed.
- **`E3/🗣️terminology/🦀️.rs:216`:** region `🔖️BrushSuggestionsRun` with unit, stages, counters and reasons, all in EN and DE. The `brush_search*` labels are removed.

### 2.2 Run job and link (`E3/⏳️precompute/🖌️brush/🦀️.rs`)

- **`:313` `resolve_placed_object_mesh_url`.**
- **`:595` region `⏯️BrushSuggestionsRun`:**
  - `BrushSuggestionVerdict`
  - `BrushSuggestionsFound`: `writer (run, generation)`, `target`, `previews`, `verdicts`, `done`, and `free()`
  - `BrushSuggestionsRequest`
  - `BrushSuggestionsLink`: `target`, `open_menu`, `close_menu`, `hover`, `wake`, `found`, `clear`, `is_empty`
  - `BrushSuggestionsOwner` (trait)
  - `BrushSuggestionsMeshSource`
  - `BrushSuggestionsRunJob<O>`: implements `InteractiveJob`
- **Job phases**, each a bounded unit:
  1. Mesh bodies, one per lane entry, from the process-wide derived store or the app's scaled box fallback, the same law as fill.
  2. Placed bodies, one per object, each with its AABB.
  3. Target listing: compatible candidates filtered by weight, and their brush poses.
  4. Collision units: `CollisionOverlapState` (1 024 samples), resumable per placed pair, host excluded, AABB prefilter.
- **Wall budget.** Collision units spend the step's wall clock (the interactive lane's 1 ms) but never its fuel.
- **Close.** The link result is retired only if this job wrote it, so a restarted successor keeps its own.

### 2.3 Precompute session (`E3/⏳️precompute/🦀️.rs`, brush regions)

- Removed: `BRUSH_SEARCH_*`, the `brush_progress` field and all its writes, `publish_brush_progress`, `brush_search_progress` (engine and session), `advance_brush_search`, and `brush_live_target`/`set_brush_live_target`.
- `brush_collision_free_until` no longer tracks progress.
- The broad-phase sync uses `resolve_placed_object_mesh_url`.
- The background lane (`precompute_step`, `brush_candidates`, `brush_preview`) stays, because puzzle 5d's brush still reads it (§6).

### 2.4 Brush utility (`E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs`)

- `definition` now carries `run`.
- `run_definition` (`:32`), with `RUN_JOB_KIND = "s.puzzle.puzzle3d.brush.suggestions"`.
- `options(envelope, labels)` (`:51`): overlap budget and the distribution tree. There is no picker.
- `build_run_job` (`:87`).
- `run_effects` (`:104`): the reconciler.
- New test module `🧪️tests/🔬️unit/🦀️.rs`.

### 2.5 Editor (`E3/🦀️.rs`, foreign: W1-B owner)

- **`Puzzle3dInstanceOperationOwner`** (`:3090`) holds the link and implements the owner close protocol.
  - `ArtifactEditor::build_instance_operation_owner` (`:7703`) and `pending_effects` (`:7709`).
  - `build_tool_run_job` (`:7690`) dispatches brush vs fill.
- **Owner threading.**
  - `Puzzle3dActionCtx.instance_owner` plus `ctx.brush_suggestions(|link| …)` (`:2553`).
  - `Puzzle3dActionPrologue::dispatch_step(app, instance_owner, …)`.
  - `bind_instance_owner` on `Puzzle3dWindowCommandWork`, `Puzzle3dPrecomputeCommandWork` and `Puzzle3dAcceptSuggestionWork`, bound in `build_tool_job`.
  - `render_body(owner, …)` passes the menu vortex's `BrushSuggestionsFound` to `main::render`.
- **`suggestionsTick` replaced by `targetBrushSuggestions`** (`:2418`) everywhere:
  - command enum, action list, dispatch arm, retained tool ids, proofs, manifest `view_action`, `Migrated` classification;
  - publication contract `HostOnly`, scope class `Quiet`.
  - The `SuggestionsTick` scope class is removed.
- **Also removed:** the `Warm` stage and its constants from the window command work, the session-slot `brush_live_target`, and the `window_measures_with_request_context` override (the framework default delegates to `window_measures`).
- **Retained `Puzzle3dAcceptSuggestionWork`.**
  - Its `Candidate` stage reads the indexed free candidate from the link (`requested % free.len()`) and closes the link's menu.
  - It publishes `create_object` at the candidate's origin, orientation, scale and mesh, plus `connect_vortices` from `object:v{sourceVortexIndex}`.
  - The `Representation` stage and `target_position` are removed.
  - Before this lane the retained accept placed catalog kind `index % kinds` at the vortex with identity orientation and `v0`, ignoring the search entirely. That was a second real defect.

### 2.6 Commands

| Command | Change |
|---|---|
| `🎮️commands/🎣️target-brush-suggestions/🦀️.rs` (new; replaces the deleted `⏱️suggestions-tick`) | Writes `link.hover(fullId or the brush target)` while the brush is armed, `None` otherwise |
| `🔓️open-vortex-suggestions` | Sets the menu and calls `link.open_menu` |
| `🔒️close-vortex-suggestions` | Clears the menu and calls `link.close_menu` |
| `✅️accept-suggestion` (unstaged path) | The free candidate comes from the link |
| `🔁️cycle-candidate` | The free count comes from the link |

The crate root `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs` swaps the module declaration.

### 2.7 Main window (`E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`, foreign: W1-B owner)

- `world_interaction_json(…, suggestions: Option<&BrushSuggestionsFound>)`:
  - The popup lists the free candidates (first 8) of its own vortex.
  - `pending` = nothing free yet and the search is not done.
  - The `progress` block is removed.
- `render(…, suggestions)`.
- `window_measures(envelope, labels)`.
- Removed: `world_brush_preview_target`, `world_brush_preview_json`, the `brushPreviewJson` interaction key, the `brush_preview_json` scene lane write, and the `[DEBUG] vortices.publish` eprintln.

### 2.8 React (surgical)

- **`WH`**, removed:
  - `WorldBrushVerdict`, `WorldBrushPreviewRecord`, `parseWorldBrushPreview`, `retainWorldBrushPreviewJsonV1`, `brushObjectPlacementArgs`, `scaleTuple`, `brushGhostPaint`, `BrushPreviewGhost`;
  - the `danger` mesh style kind, which only that ghost used;
  - the leftover `brushPreviewJson` field and its carry;
  - the 120 ms suggestions interval, the hover tick and the `[DEBUG]` hops;
  - the interactive-action in-flight counter (`begin`/`endInteractivePluginAction`), which only yielded to that interval.
- **`WH`**, added or changed:
  - A coalescing `targetBrushSuggestions` lane that sends only a *change* of the armed brush's hovered vortex.
  - A brush click dispatches `acceptSuggestion {fullId}`.
  - `brushPreviewGhostMeshUrl` is renamed `worldGhostMeshUrl`; the catalogue drop still uses it.
  - Collision mesh upload uses the scene lane's meshes, which already include catalogue kinds.

## 3. Tests (TDD: fixture and laws first, then implementation)

### 3.1 New laws

**`E3/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs`**, region `⏯️BrushSuggestionsRun`, with the language-neutral fixture `🧫️fixtures/🎞️brush-suggestions-run.json`:

| Test | Asserts |
|---|---|
| `brush_suggestions_run_matches_the_language_neutral_fixture` | Four cases (clear, blocked, missing target, vortex kind without suggestions): exact `verdict:reason` sequence, steps with args, counters, idle stage; every candidate is `testing` first and decided exactly once; the link lists exactly the free kinds |
| `a_body_carrying_its_own_mesh_where_every_candidate_docks_collides_them_all` | Red→green for the old `…blocked…` and `…ghost_json…verdict` reds, on the run **and** the 5d-facing precompute lane |
| `brush_suggestions_run_step_with_one_unit_of_fuel_decides_at_most_one_candidate` | `toolRunStep` = one candidate |
| `brush_suggestions_run_follows_the_link_target_and_waits_while_settled` | A settled search emits nothing and waits; a gesture wakes it; a new target clears the trace; no target clears the trace and the result |
| `a_closing_brush_suggestions_job_retires_only_its_own_result` | See the close rule in §2.2 |
| `brush_suggestions_run_collision_verdicts_agree_with_the_parry3d_oracle` | **Oracle `parry3d`** (existing dev-dependency). All Concrete Forest vortices plus 3 Nakagin vortices, box fallback, exact convex hulls, overlap volume by parry point containment: 0 disagreements among decisive verdicts, both collisions and frees decided, ≤ 10 % ambiguous |
| `brush_suggestions_run_step_stays_below_the_interactive_ceiling_for_nakagin` | Real clock, interactive lane budget, 12 targets, best of 5 cold runs, worst step < 2 000 µs. Solo steps measured 0.13–1.6 ms |

**Other new laws:**

| File | Test |
|---|---|
| `🪛️utilities/🖌️brush/🧪️tests/🔬️unit/🦀️.rs` | `the_brush_suggestions_reconciler_starts_wakes_and_aborts_exactly_once_per_answer` |
| `E3/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` | `the_brush_suggestions_run_vocabulary_equals_the_schema_table_and_is_authored_in_english_and_german` |
| `🧊️main/🧪️tests/🔬️unit/🦀️.rs` | `the_suggestion_popup_lists_the_free_candidates_the_brush_run_published_for_its_vortex` |

**App-level laws in `E3/🧪️tests/🔬️unit/🦀️.rs`**, using the new probes `pump_brush_suggestions`, `brush_suggestions_settled`, `target_brush_suggestions`, `tool_run_trace_records` and `tool_run_trace_run`:

| Test | Asserts |
|---|---|
| `the_armed_brush_runs_a_read_only_search_that_traces_every_candidate_and_leaving_aborts_it` | Arming alone starts nothing. Every candidate is on the trace, and the document is unchanged. A retarget keeps the same run id. Leaving → `aborted`, byte-identical document, no undo entry. Re-hovering → a new run |
| `the_popup_search_is_aborted_on_close_and_an_accept_is_one_undoable_placement` | Close aborts. A reopen starts a new run. Accept adds one object with **one command row**, and one undo restores the document |
| `the_brush_utility_declares_a_read_only_tool_run_through_the_manifest` | The declaration, reasons equal to the schema, settings reads, no `suggestionsTick` action, no picker |
| `vortex_hover_storm_admits_then_the_brush_run_searches_and_a_click_places` | See the rewrites below |
| `hover_suggestion_moves_the_candidate_index_over_the_free_candidates_the_run_found` | See the rewrites below |

**Rewrites of existing tests.**
- The two tests above replace the ghost/tick-based `vortex_hover_storm_…brush_preview_and_place` and `hover_suggestion_updates_…live_preview`.
- Accept and open tests now pump the run before accepting.
- `accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin` binds an owner holding one free candidate.
- Static route markers are updated.
- The scope-class law no longer lists `SuggestionsTick`.

**Deleted** (they pinned the tick, the ghost or the picker):
- `hover_committed_in_brush_publishes_preview`
- `suggestions_tick_at_the_window_kind_…`
- `leftover_refresh_after_suggestions_tick_…`
- `one_hover_warms_the_brush_preview_…`
- `the_tick_rewarms_the_latched_brush_target_…`
- `hover_committed_click_places_via_published_preview`
- `brush_placement_picker_appears_only_for_a_live_brush_target`
- the brush search precompute laws and their `world_brush_preview_target` window law

**React.**
- `engine-contract`: the ghost and placement-args tests are deleted. The awaitable-twin law now pins the `targetBrushSuggestions` lane. A new source law checks that `WH` contains no `suggestionsTick`, interval, `brushPreviewJson`, ghost or `addBrushObject`, and that the trace layer is mounted.
- The leftover-overlay law is without the preview field.
- `🔌️plugin-runtime`: the hash-bust test for the deleted functions is removed.
- `⏯️tool-run-trace` component test: the `danger` style row is removed.

### 3.2 Commands run (foreground, repo root unless noted)

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --lib --tests` | ok, no warnings in lane files | `check-tests-3.txt` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --target wasm32-wasip2` | ok (95 crate warnings, none new) | `check-3d-wasip2.txt` |
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly -j 4 --lib --tests` | ok | `check-5d.txt` |
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly -j 4 --lib` | ok (shared retained trait) | `check-2d.txt` |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- precompute:: brush suggestion vortex_suggestions the_armed_brush the_popup_search retained terminology windows::main scope --skip brush_suggestions_run_collision_verdicts --skip brush_suggestions_run_step_stays --skip fill_ --test-threads=4` | **156 passed, 0 failed** | `test-final-brush.txt` |
| `… -- brush_suggestions_run_collision_verdicts --test-threads=1` | 1 passed (31.5 s) | `test-brush-4.txt` |
| `… -- the_brush_suggestions_reconciler brush_suggestions_run_step_stays --test-threads=1` | 2 passed | `test-reconciler-1.txt` |
| `… -- accept_suggestion_step_loop --test-threads=4` | 1 passed | `test-accept-loop.txt` |
| `… --lib -j 4 -- --test-threads=4 --skip brush_suggestions_run_collision_verdicts` (whole crate) | 710 passed, 12 failed (see below) | `test-lib-full-1.txt` |
| `bun ./📜️script.ts test long "engine-contract"` (react package) | **603 passed** | `ts-engine-contract-1.txt` |
| `bun ./📜️script.ts test long "🌐️World3dHost" "world3d-interaction" "world3d-instance-delta" "world3d-pick-bounds" "tool-run-trace"` | 43 passed, 1 failed (the `danger` row, fixed), then `tool-run-trace` 8/8 | `ts-world3d-1.txt`, `ts-trace-runtime-1.txt` |
| `bunx tsc --noEmit -p tsconfig.json` (react package) | Only the pre-existing errors in `WH` (4) and ShellHost (same set as W0-H/W1-C) | `react-tsc-1.txt` |
| `bun T/🐍️w1d-policy-probe.ts` | amend 0, local-lifecycle 0, legacy-trace 0, reserved-action 0; no brush finding | `probe-1.txt` |

**The 12 failures in the whole-crate run are outside this lane.**
- They are not in lane files, and their signatures are unrelated:
  - catalogue panel tests (4: "catalogue declares object kinds");
  - `wire_format_guard::engine_command_rows_…` (engine command tag 02→04, from W1-B's removal of the fill engine rows);
  - `every_context_menu_row_…` (zoom row count);
  - `the_settings_panel_is_addressed_at_the_focused_pane…`;
  - `window_options_are_local_…` (a reloaded window config pack loses `gridSpacing`);
  - `two_instances_converge_…backbone` ("remote snapshot merge is fail-closed");
  - `mutation_latency::one_mutation_publishes_…` (store units 953 on Nakagin);
  - `an_id_only_announcement_…` (a parallel-run flake on the process-wide mesh store: it **passes at `--test-threads=1`**, `test-suspects-1.txt`).
- **No clean baseline exists**, because the repo moves concurrently. They need their owners.

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- precompute::brush:: brush_suggestions the_armed_brush the_popup_search` (brush run laws)
- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- brush_suggestions_run_collision_verdicts --test-threads=1` (parry3d oracle)
- `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --target wasm32-wasip2`

## 5. Deviations from the contract, with reasons

1. **One long-lived run that follows a target, instead of one run per hovered vortex.**
   - `toolRunStart` while the previous run is `aborting` answers `toolRun.busy`, and a teardown takes driver turns. A per-vortex run would drop every fast hover.
   - The link target plus port wake retargets in O(1). The run is started on the first target and aborted when no target remains ("leave or close").
   - Consequence: the brush run never reaches `complete`. It stays `running` in stage `idle` with `completed == total` once the target is searched.
2. **The run start and target come from the app instance owner, not from `toolRunStart` arguments.**
   - `ToolRunJobRequest` carries no start arguments, and commands cannot read `tool_run()`.
   - The owner is the framework-sanctioned per-instance ephemeral state that jobs and commands share, the same pattern as generation3d.
   - The reconciler lives in `pending_effects`, like generation3d's `preview_eval_run_effects`.
3. **Results are published twice: trace plus link.** The popup, cycle and accept need candidate identities, and renders, measures and commands have no read access to the ledger trace.
4. **A pose-unavailable candidate is traced with an `Entity { entity: key }` subject.** It has no pose to draw.
5. **No selected-candidate emphasis in 3d.** The host ghost was the only paint of the *current* free candidate. The trace shows all candidates (free in success, collisions in danger), and the popup marks the current index. See open item 3.
6. **The placement picker measure is removed.** The framework ToolRun panel's trace list is the accessible readout, and measures have no instance-owner access.
7. **A trace subject whose mesh URL is not in the published lane uses index 0 (box).**
8. **Host leftover-brush-preview refresh subsystem deleted** (ShellHost, PluginRuntime, ShellHelpers). It forced world-body refreshes keyed on `suggestionsTick` and a leftover preview JSON that no guest publishes. The trace lane refreshes through the ledger's own dirty scope.

## 6. Foreign edits

| File | Edit |
|---|---|
| `E3/🦀️.rs` (W1-B) | §2.5 |
| `E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` and its unit tests (W1-B) | §2.7 |
| `E3/🧪️tests/🔬️unit/🦀️.rs` (W1-B) | Brush tests only (§3) |
| `E3/🗣️terminology/🦀️.rs` and tests | Brush region |
| `A/🧬️schema/{🦀️.rs,🔣️.json}` | Brush vocabulary; removed `BrushSearchProgress` and `FillCandidateVerdict` |
| `E3/🎮️commands/{✅️accept-suggestion,🔁️cycle-candidate}/🦀️.rs` | Read the link |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs` | Module declaration |
| `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` | Default method `bind_instance_owner` |
| `A/🧫️fixtures/🗄️retained-jobs/🔣️.json`, `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` | `suggestionsTick` → `targetBrushSuggestions` (host-only) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | Member `⏱️suggestions-tick` → `🎣️target-brush-suggestions` |
| React: `🛠️ShellHelpers/🟦️.tsx`, `🔌️PluginRuntime/🟦️.tsx`, `🏛️ShellHost/🟦️.tsx`, `🎯️targets/⚛️react/🟦️.tsx`, `🧪️tests/🔬️engine-contract/🟦️.ts`, `🧪️tests/🔌️plugin-runtime/🟦️.tsx`, `🌐️World3dHost/⏯️tool-run-trace/🧪️tests/🧩️component/🟦️.ts` | §2.8, §5.8 |

No fill file was touched. W0-I added `ToolRunDefinition.settings` mid-lane, and the brush declares it.

## 7. Open items

1. **Puzzle 5d brush (W2-B or next).**
   - 5d still reads the 3d precompute background lane (`precompute_step`, `brush_candidates`, `brush_preview`).
   - It still publishes `brush_preview_json`, which `WH` no longer paints, so **5d's React brush ghost is gone**.
   - Convert 5d onto `BrushSuggestionsRunJob`, then delete the lane and the `brushPreview` scene lane (framework `🎬️scenes`, TS `🎬️scene`, wgpu board port).
2. **The same kind-instead-of-own-mesh placed-body resolution exists in the fill planner** (`E3/⏳️precompute/🪣️fill/🦀️.rs` `prepare_entry_one`). It is not touched here per brief. Switch it to `resolve_placed_object_mesh_url`.
3. **Framework (W0-I).** A domain-neutral "emphasize record" affordance, for example a selected-key cursor on the trace layer, would restore the "this is what a click places" highlight. `toolRunStart` could preempt a `mutating: false` run instead of answering busy, so arming fill while the brush run is live starts fill immediately. Today the host disarms brush, the next refresh aborts, and the fill start must be retried.
4. **Stale generated manifests.** `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` and `✏️s/🔌️plugins/🎪️demonstrator/🔣️.json` still list `suggestionsTick`, as they already listed `fillBuildTick`. Regenerate them.
5. **Orphaned test suite.** `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` is not registered in the react suite config, so it was not run.
6. **Runtime browser evidence** (deploy chain, `data-tool-run-*` on a hovered vortex) was not captured in this lane. It belongs to the verification lane.
7. **Twelve whole-crate reds outside this lane** (§3.2), which need their owners.
