# 🎮️ Wave B2 — commands / config / editor integration

Ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`, master plan §1 decisions 1, 2, 4.
`E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 1. What changed

### 1.1 No ceiling on the count (decision 1)

| file:line | change |
|---|---|
| `E/🦀️.rs:73` | `pub const PUZZLE3D_FILL_COUNT_MAX: u32 = 1000;` **deleted** |
| `E/🎮️commands/🧮️set-fill-count/🦀️.rs:19-21` | `parse_count` no longer `.min(PUZZLE3D_FILL_COUNT_MAX)` — it clamps only to the `u32` range (`.clamp(0.0, f64::from(u32::MAX))`) |
| `E/🎮️commands/📨️engagement-submit/🦀️.rs:19` | `fill <n>` verb keeps the typed number verbatim |
| `E/🎮️commands/🔂️engagement-repeat-last/🦀️.rs:8` | `fill_count.saturating_add(1)`, no ceiling |
| `E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` | (wave C, already landed) count control is `WindowMeasure::Number { max: None }` |

### 1.2 Default 100 (decision 4)

| file:line | change |
|---|---|
| `E/🎚️config/🦀️.rs:23-31` | new `default_fill_count() -> u32 { 100 }` with its docstring |
| `E/🎚️config/🦀️.rs:150,227` | `Puzzle3dRuntime::fill_count` — `#[value(default = "default_fill_count")]` + `Default` |
| `E/🎚️config/🦀️.rs:261,282` | `Puzzle3dConfig::fill_count` — same |
| `E/🎚️config/🧬️schema/🔣️.json:9` | `"fillCount": { …, "default": 100, … }` |
| `E/🎚️config/🧬️schema/🟦️.ts:8` | new `export const PUZZLE3D_CONFIG_FILL_COUNT_DEFAULT = 100;` (guard logic unchanged) |
| `E/🪟️window/🦀️.rs:286,318` | unchanged — `runtime()`/`shared()` already pass `fill_count` straight through, verified no clamp on either leg |

### 1.3 `setFillCount` drives the LIVE session (decision 2)

`E/🦀️.rs`, `Puzzle3dPrecomputeCommandWork`:

- stages `FillPrepare` / `FillPlan` / `FillApply` **deleted** (they minted a fresh
  `Puzzle3dPrecomputeSession::new()` per dispatch and replanned synchronously);
  replaced by `FillRequest` → `FillLock` (`E/🦀️.rs` ≈6980-7000).
- the `fill_precompute: Option<Puzzle3dPrecomputeSession>` field, its `close_step` drain and its
  `terminal_is_empty` clause are gone.
- new `Puzzle3dPrecomputeCommandWork::runtime_for(&self, config) -> Result<Puzzle3dRuntime, Fault>`
  (`E/🦀️.rs` ≈6850) — the composed window-owner runtime both fill stages need.
- `FillRequest`: `with_puzzle3d_app_for(session, &runtime, |app| app.precompute.borrow_mut().set_fill_requested_count(self.requested_count))`
  — the app's own retained session, checked out of the process-global session registry, exactly the
  one `fillBuildTick` drives.
- `FillLock`: `set_fill_count::take_locked_mutations(&mut app.precompute.borrow_mut())` per turn,
  one bounded `FILL_LOCK_PLACEMENTS_PER_TICK` chunk, looping until it yields nothing. Lowering
  therefore emits its `delete_object` tail immediately; raising yields nothing (the tick locks new
  placements in as they arrive).
- `Publish` (`setFillCount` branch) unchanged in shape: accumulated `fill_mutations` +
  `Puzzle3dConfigMutation::SetFillCount { count }` when the value moved, `coalesce_key: "fill-count"`.

`E/🎮️commands/🧮️set-fill-count/🦀️.rs` rewritten:

- `MAX_PLACEMENTS_PER_STEP` and `apply_chunk` (which called the removed-from-use
  `apply_fill_count_chunk`) are gone.
- `take_locked_mutations(precompute) -> Vec<Puzzle3dMutation>` — the command-work shape
  (delete_object, then create_object, then connect_vortices).
- `take_locked_into_fixture(precompute, fixture) -> bool` — the ctx/reducer shape (see §2).
- `document_object` / `document_attraction` are the two engine→document twins; `document_object`
  keeps the proven `FixtureObject → Value → crate::Puzzle3dObject` bridge.

### 1.4 `fillBuildTick` commits (decision 2)

`E/🎮️commands/🪣️fill-build-tick/🦀️.rs`: after `poll_fill_job()` the arm calls
`set_fill_count::take_locked_into_fixture(&mut precompute, &mut ctx.scene.fixture)` and folds
`committed` into the dirty test. `UiDirtyScope` is untouched: still `puzzle3d_fill_build_scope()`
(Partial: main body + tools) and still `UiDirtyScope::None` when nothing moved.

### 1.5 The render-time ghost tail is gone (decision 2)

Deleted from `E/🦀️.rs`: `Puzzle3dFillDisplayPayload`, `FillDisplayMemo`,
`fill_display_payload_from_fixture`, `append_fill_display_tail`,
`puzzle3d_fixture_with_fill_display_memo` (≈1649-1705), the `fill_display` field on
`Puzzle3dSessionState` and its `bytes()` term, the `fill_display_memo: Mutex<…>` field on
`Puzzle3dPlayApp`, and both session check-out/check-in legs.

`render_body` (`E/🦀️.rs` ≈8239) now composes `app.render_fixture(&puzzle3d_projection_value(…))`
directly — no `compose_fill_display`, no `fill_available`.

`reveal_index` removed from the editor's own `Puzzle3dObject` (nothing else needed it):

- `E/🦀️.rs:208-211` field, `:574` `fixture_object_from_snapshot`, `:1522` `engine_fixture_object`
  (now hands the engine `reveal_index: None`).
- readers in `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` (`instance["revealIndex"]` emission at ≈231
  and both `reveal_index.hash(…)` terms at ≈258/558) plus the stale doc line at ≈204 — **wave C's
  file, edited only because the field they read no longer exists** (see §5).
- struct-literal sites in `E/📌️panels/{🗿️artifact,🔍️inspection}/🧪️tests/🔬️unit/🦀️.rs`,
  `E/🎮️commands/🌱️add-object-kind/🦀️.rs`, `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs`,
  `E/🧪️tests/🔬️unit/🦀️.rs`.

The schema-side `FixtureObject.reveal_index` and `compose_fill_display` are untouched (waves A/B1);
with the editor twin's field gone, no instance can carry `revealIndex` any more, so the host's
`RevealCutoffStore` is already inert before wave C removes it.

`set_fill_applied_count(config.fill_count)` removed from all three `E/🦀️.rs` call sites (prologue
`sync_step`, `render_body`, `tool_measures`/`window_measures`): the applied cursor is now
session-owned and moved only by `take_fill_locked_chunk`. Requested and applied legitimately
diverge now, so restoring the cursor from the *requested* count would have told the session the
whole run was already committed.

## 2. How the tick emits mutations — the exact path

`fillBuildTick` is a retained tool (`PUZZLE3D_RETAINED_TOOL_IDS`) routed to
`Puzzle3dPrecomputeCommandWork` (`E/🦀️.rs` ≈7900), which for a non-`setFillCount` verb runs
`Decode → Objects/Vortices/Attractions/Catalog* → PrologueScene → PrologueSync → Publish`.
`Publish` calls `Puzzle3dActionPrologue::dispatch_step`, which:

1. hands the arm a `Puzzle3dActionCtx` over a mutable `Puzzle3dScene` (`E/🦀️.rs` ≈3458);
2. after the arm returns, derives `artifact_mutations` with
   `puzzle3d_operations_from_fixture_change(before, &scene.fixture)` — **only** when
   `puzzle3d_action_document_intent(action)` is true.

So the tick emits mutations the same way `addBrushObject`/`deleteSelection` do: by editing
`ctx.scene.fixture`. Two registry edits make that real:

- `puzzle3d_action_document_intent` (`E/🦀️.rs:701-725`) += `"fillBuildTick"` — this is what
  materializes `before` and runs the diff;
- the `coalesce_key` match in `dispatch_step` (`E/🦀️.rs` ≈3480):
  `"setFillCount" | "fillBuildTick" => Some("fill-count")` — `AmendLast` on the artifact lane
  (`🧰️framework/…/🔌️plugin/🦀️.rs:22916`, and `publication.set_coalesce_key(…)` on the interactive-job
  completion path at `:25335`), so a whole fill run is ONE undo entry shared with `setFillCount`.

`take_locked_into_fixture` mirrors `delete-selection`'s own retain predicate for orphaned
attractions (`{objectId}:` prefix on `attracting`/`attracted`), so a lowered tail leaves no dangling
attraction for the diff to trip over.

## 3. Lane / classification changes

- `ArtifactToolPublicationContract { tool_id: "fillBuildTick", … }` (`E/🦀️.rs` ≈7215):
  `HostOnly` → `Artifact`. Config/window-config/interaction are NOT declared: the tick writes only
  the document (`window_ownership::shared(&scene.runtime)` is unchanged by it, so no config lane is
  produced — the framework faults on an undeclared lane, `🔌️plugin/🦀️.rs:25307`).
- `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json`: `fillBuildTick` moved from the
  `host-only` group to the `artifact` group of `Puzzle3dPlayApp` (the TS oracle
  `exactContracts`/`ownerOracle` in `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:56,181`
  compares this fixture route-by-route against the Rust contracts).
- **No classification change needed.** `fillBuildTick` is already
  `InteractiveJobClassification::Migrated` (`E/🦀️.rs` ≈8689), which is the only thing
  `validate_ui_dispatch_classification` (`🔌️plugin/🦀️.rs:12138`) checks.
- **`ActionKind::View` stays.** The framework explicitly records a `View`-kind dispatch that reaches
  the document (`🔌️plugin/🦀️.rs:22896-22903` — only a `View` whose *only* emission is window-config is
  skipped), so the tick still gets its history row and its coalesced edit. The existing law
  `fill_build_tick_is_a_view_action_with_narrow_ui_scope` therefore stays valid unchanged.

## 4. Tests

### 4.1 `E/🎚️config/🧪️tests/🔬️unit/🦀️.rs` (new)

- `default_fill_count_is_a_hundred_and_survives_the_json_round_trip` — `Puzzle3dConfig::default()`
  and `Puzzle3dRuntime::default()` are 100, the serialized config carries `"fillCount":100`, and an
  omitted `fillCount` decodes to 100 (the schema default), not 0.

### 4.2 `E/🧪️tests/🔬️unit/🦀️.rs`

Removed (they pinned semantics decisions 1/2 delete):

- `fill_build_tick_only_plans_available_slider_range` (slider-max pin + "planning must not append")
- `set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up` (the clamp is gone)
- `fill_render_reveals_the_full_available_plan_tagged_with_reveal_index` (ghost tail)
- `seeded_objects_omit_reveal_index_so_the_boot_cutoff_cannot_hide_them` (`revealIndex` is gone)
- `fill_count_measure_shows_planning_progress_while_precompute_incomplete` (replaced, see below)

Added / rewritten:

| test | asserts |
|---|---|
| `fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks` | the tick grows the document monotonically, never more than `FILL_LOCK_PLACEMENTS_PER_TICK` (8) per tick, never past the requested count; the count entry's `ready` == the locked count; `instance_count(render) == object_count` (no ghost) |
| `a_whole_fill_run_coalesces_into_one_undo_entry` | one `undo` restores the pre-run document |
| `lowering_the_fill_count_deletes_the_committed_tail_and_publishes_the_count` | the document shrinks to the new count and the config lane carries the lowered request verbatim; still no ghost |
| `the_fill_count_has_no_ceiling_and_the_entry_declares_none` | a 5 000 request survives verbatim, the `Number` entry's `max` is `None`, `parse_count` carries the whole `u32` range |
| `fill_count_is_shared_across_split_panes` | (rewritten without `revealCutoffs`) both panes emit the same instance ids after a commit on either pane |
| `fill_count_measure_is_an_unbounded_number_entry_reporting_the_locked_count` | `WindowMeasure::Number` with `min: Some(0.0)`, `max: None`, `value` == requested, `ready` == locked ≤ requested |

Support changes in the same file: new `find_measure_number` / `find_measure_number_max` /
`find_measure_number_ready` helpers; `fill_ready` now reads the `Number` entry's `ready`
(= LOCKED count, docstring updated) instead of the deleted slider's planned extent;
`drive_fill_until_ready` documented as "until the document holds `target` locked placements";
two prose references to `PUZZLE3D_FILL_COUNT_MAX` rewritten.

### 4.3 Results

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --message-format short` (lib):

```
warning: `semio-s-artifact-puzzle-3d` (lib) generated 96 warnings (run `cargo fix --lib -p semio-s-artifact-puzzle-3d` to apply 91 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 5m 43s
```

→ **0 errors** (baseline 0 errors / 97 warnings; the 96 include new never-used warnings for
`Puzzle3dPrecomputeSession::apply_fill_count_chunk` and `set_fill_applied_count`, which B1 now owns
as dead code, see §5).

`--all-targets` (test build) + `cargo test … --lib -- fill`: see §6 below — the run is gated on
peer-owned compile errors outside this wave.

## 5. Files touched outside wave B2's declared ownership (and why)

1. `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` (**wave C**) — three `reveal_index` reader lines and one
   doc line. Deleting the field on `Puzzle3dObject` is decision 2; these were its only readers, and
   wave C removes the host half of the same mechanism. Nothing else in that file was touched.
2. `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs` (**wave C**) — one struct-literal line.
3. `E/📌️panels/{🗿️artifact,🔍️inspection}/🧪️tests/🔬️unit/🦀️.rs`, `E/🎮️commands/🌱️add-object-kind/🦀️.rs`
   (unassigned) — struct-literal lines only.
4. `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` — the lane move of §3. (The
   master plan named `🧪️publication-authority`; the file actually lives at `🧫️fixtures/🔏️publication-authority`.)

## 6. Open issues

1. **Peer-owned compile errors block the test run.** At the time of writing, `--all-targets` fails
   with six errors, none in a B2 file:
   - `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs:106,107` and
     `E/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:685,686` — `Puzzle3dLabels::labels` no longer exists
     (wave C's terminology refactor, mid-flight).
   - `E/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:668` — `Puzzle3dPrecomputeSession.engine` is private
     (wave B1/G).
   - `E/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:608` — `WorkerJobSessionAdmissionRejected<SharedFillWorkerJob>`
     does not implement `Debug` (wave B1).
   The B2 lib and the B2 test file themselves compile clean; the fill test results must be re-run
   once those land.
2. **Dead code now owned by B1**: `Puzzle3dPrecomputeSession::apply_fill_count_chunk` and
   `set_fill_applied_count` have no callers left (B2 removed the last ones). They should be deleted
   with the rest of the old apply path.
3. **A `[DEBUG]` eprintln survives in peer code** — `E/⏳️precompute/🦀️.rs` `brush_preview`
   (`[DEBUG] puzzle3d.brushPreview.compute …`) and `E/🦀️.rs:3385`
   (`[DEBUG] puzzle3d.utility.publish …`). Neither is B2's and neither is fill-related; flagged for
   whoever owns the cleanup pass.
4. **Per-tick diff cost.** Making `fillBuildTick` a document-intent action means every 120 ms tick
   now materializes the projection `before` and runs `puzzle3d_operations_from_fixture_change` over
   the whole document. That is the sanctioned ctx path (`addBrushObject` pays the same), and the
   existing law `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` is the
   gate on it — re-run it once the tree compiles.
5. **`E/🎚️config/🧬️schema/{🔗️.graphql,🛰️.proto}`** carry no default notion, so only `🔣️.json` and the
   TS mirror express the 100. Flagged in case the schema-first gate wants a uniform expression.
