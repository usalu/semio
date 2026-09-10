# Wave U — Editor Correctness (2026-09-10)

Claim timestamp: **2026-09-10 00:25 CEST** (wave start). W-U3 verification closed **2026-09-10 ~02:25 CEST**.
Repo MCP (`repo://goals`, ticket tools) is not mounted; work stayed inside `26/09/02/PUZZLE-3D-END-TO-END`.

Assignment: five editor-correctness defects in `semio-s-artifact-puzzle-3d`.

| # | Defect | Verdict |
|---|---|---|
| 1 | Gumball undo coalescing | **fixed** (unmodified `gumball_translate_drag_coalesces_into_one_edit` passes; rotate/scale twins pass) |
| 2 | ActionKind honesty for `relocateTargetVolume` / `worldRelocate` | **fixed / already honest** — both `.mutation(...)`; Document scope names inspector + outliner + history + world; undo/redo laws pass |
| 3 | Outliner continuation-row E2E | **fixed** — `setPanelPage` + `runtime.panel_pages` cloned into window config; continuation law passes |
| 4 | Inspection `ids` paging | **fixed** — `IDS_ROWS = 16` + same `setPanelPage` cursor; law passes |
| 5 | Stale fixture id `setFillCountStep` | **proven-not-a-defect** — absent from 3d retained-jobs fixture and the puzzle tree; live ids are `setFillCount` / `fillBuildTick` / `cancelFillBuild`; bijection law passes |

Hard constraints: no modifying git; no edits to precompute (W-F / W-F2); no wasm rebuild; no :6013; no kill of foreign processes.

---

## 1 Causes

### 1 Gumball undo

`Puzzle3dScaleWork` emits absolute `move_object` / rotate / scale per tick with `coalesce_key` `gumball-translate|rotate|scale`. Retained-tool completions apply through `store.begin_apply_batch`, **not** `dispatch_emit`/`AmendLast`.

Two host/store holes (already closed in-tree before this verification turn):

1. Plugin success path now does `publication.set_coalesce_key(emit.coalesce_key.take())`.
2. Store `fold_batch_item` stamps `publication.coalesce_key` onto the staged edit; `Publishing` calls `batch_amend_target` and **extends** the tail uncommitted edit (forwards + inverses) without replacing `tail_undo_cache`. One undo therefore restores gesture start (`1+2+3` → `[0,0,0]`).

Without the stamp, the first tick minted `coalesce_key: None` (`puzzle3d_artifact_store_edit` still hardcodes `None` on the per-item candidate; the batch stage is what carries the key). Later ticks then created new edits; one undo reversed only `+3`.

### 2 ActionKind

Both verbs are `.mutation(...)` at stitch (`worldRelocate` ~7753, `relocateTargetVolume` ~7803). Scope table maps both to `Puzzle3dScopeClass::Document`. `puzzle3d_document_panel_bodies` = selection bodies (inspector, artifact outliner, history) + catalogue; world is the window body. `command_scope_classes_name_the_panels_they_change` already requires inspector + outliner + history for every narrowed Mutation.

### 3 Outliner continuation

`set_panel_page` writes `runtime.panel_pages`. `Puzzle3dWindowCommandWork` runs `dispatch_step`, which emits `window_config_mutations` when `Puzzle3dWindowConfig::from_runtime` (clones `panel_pages`) differs. Artifact panel binds `+N` to `setPanelPage`. Persist follows grid/LOD (window config lane).

### 4 Inspection ids

`IDS_SECTION` / `IDS_ROWS = 16`; `push_ids` pages via `runtime.panel_pages` and dispatches `setPanelPage`. Last page drops `+N`.

### 5 `setFillCountStep`

No hit under the puzzle plugin. 3d `retained-jobs` JSON lists `setFillCount`, `fillBuildTick`, `cancelFillBuild`, `setPanelPage`. Demonstrator still has the stale id; that is not this crate's fixture.

---

## 2 Changes this turn (W-U3)

Store/plugin/editor stitch for defects 1–4 were already on disk (peer Wave U / prior instance). This turn:

- Re-read ScaleWork, store `advance_apply_batch` / `fold_batch_item`, plugin `set_coalesce_key`, relocate stitch, `set_panel_page`, window `from_runtime`, inspection paging, retained-jobs fixture.
- Removed leftover `[DEBUG]` `eprintln`s from the artifact outliner unit file (5 lines).
- Prefixed unused `more` in the continuation law (`_more`) so the `expect` stay is the assertion.

No precompute edits. No main `🦀️.rs` command/scope edits this turn (file was cold; the stitch was already correct).

---

## 3 Laws

- `gumball_translate_drag_coalesces_into_one_edit` (unmodified)
- `gumball_rotate_drag_coalesces_into_one_edit`
- `gumball_scale_drag_coalesces_into_one_edit`
- `command_scope_classes_name_the_panels_they_change`
- `relocate_target_volume_undoes_and_redoes_as_one_mutation`
- `world_relocate_undoes_and_redoes_as_one_mutation`
- `pressing_the_outliner_continuation_reveals_the_next_page`
- `inspection_ids_page_is_bounded_and_the_continuation_advances`
- `retained_publication_contracts_are_an_exact_nonempty_tool_bijection`

---

## 4 Commands + tails (W-U3, this machine)

Envelope:

```
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d
export RUSTC_WRAPPER=""
export RUST_MIN_STACK=134217728
```

`-j 4`, `--test-threads=1`, crate `semio-s-artifact-puzzle-3d`, feature `component-app-assembly`.

### 4.1 cargo check

```
1618 |     pub(crate) fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
     |                   ^^^^^^^^^
...
1625 |     pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) {
     |                   ^^^^^^^^^^^^^^^^

warning: `semio-s-artifact-puzzle-3d` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s) in 31.70s
```

Exit 0.

### 4.2 Targeted laws

**gumball_**

```
test editor::puzzle3d::component::tests::gumball_active_only_for_transform_utilities_with_object_selection ... ok
test editor::puzzle3d::component::tests::gumball_gesture_commits_one_absolute_delta_between_its_host_brackets ... ok
test editor::puzzle3d::component::tests::gumball_inactive_when_every_handle_flag_is_off ... ok
test editor::puzzle3d::component::tests::gumball_rotate_drag_coalesces_into_one_edit ... ok
test editor::puzzle3d::component::tests::gumball_scale_drag_coalesces_into_one_edit ... ok
test editor::puzzle3d::component::tests::gumball_translate_drag_coalesces_into_one_edit ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 625 filtered out; finished in 5.17s
```

**command_scope_classes_name_the_panels_they_change**

```
test editor::puzzle3d::component::tests::command_scope_classes_name_the_panels_they_change ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 630 filtered out; finished in 0.12s
```

**undoes_and_redoes_as_one_mutation**

```
test editor::puzzle3d::component::tests::relocate_target_volume_undoes_and_redoes_as_one_mutation ... ok
test editor::puzzle3d::component::tests::world_relocate_undoes_and_redoes_as_one_mutation ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 629 filtered out; finished in 3.82s
```

**pressing_the_outliner_continuation_reveals_the_next_page**

```
test editor::puzzle3d::panels::document::tests::pressing_the_outliner_continuation_reveals_the_next_page ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 630 filtered out; finished in 0.01s
```

**inspection_ids_page_is_bounded_and_the_continuation_advances**

```
test editor::puzzle3d::panels::inspection::tests::inspection_ids_page_is_bounded_and_the_continuation_advances ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 630 filtered out; finished in 0.00s
```

**retained_publication_contracts_are_an_exact_nonempty_tool_bijection**

```
test editor::puzzle3d::component::tests::retained_publication_contracts_are_an_exact_nonempty_tool_bijection ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 630 filtered out; finished in 0.19s
```

### 4.3 Full lib suite vs 613/9 baseline

```
failures:
    editor::puzzle3d::component::tests::a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh
    editor::puzzle3d::component::tests::fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances
    editor::puzzle3d::component::tests::fill_build_tick_only_plans_available_slider_range
    editor::puzzle3d::component::tests::fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job
    editor::puzzle3d::component::tests::fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances
    editor::puzzle3d::component::tests::fill_render_reveals_the_full_available_plan_tagged_with_reveal_index
    editor::puzzle3d::component::tests::probe_fill_law_stack_peaks
    editor::puzzle3d::component::tests::set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up
    editor::puzzle3d::component::tests::two_instances_converge_disjoint_object_edits_via_backbone
    editor::puzzle3d::precompute::component::tests::bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint
    editor::puzzle3d::precompute::component::tests::fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close
    editor::puzzle3d::precompute::component::tests::fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase
    editor::puzzle3d::precompute::component::tests::fill_worker_session_drop_during_measurement_mounts_the_same_terminal_once

test result: FAILED. 618 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out; finished in 93.38s
```

**Honest verdict: 618 passed / 13 failed** (631 tests). Baseline was **613 / 9**.

Wave U's five defects are **not** among the 13. Gumball translate/rotate/scale all passed in the targeted run.

The 13 failures are foreign:

- W-F / W-F2 fill job + slider laws (6 `fill_*` / `set_fill_count_*` in `component::tests`)
- `probe_fill_law_stack_peaks` — testkit `unknown puzzle3d action id: setSpacing` (live verb is `setGridSpacing`)
- `two_instances_converge_disjoint_object_edits_via_backbone` — fail-closed remote snapshot merge (same as baseline)
- `a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh` — Wave P interaction-lane republish (same as baseline)
- 4 `precompute::component::tests::fill_worker_*` / `bounded_fill_job_*` — W-F2 owns `⏳️precompute` (hot / in-flight)

Not repaired: W-F2 owns those files; `setSpacing` breakage is a peer rename, not a Wave U defect.

---

## 5 Open

- W-F2 fill-job suite still red; do not edit precompute from this wave.
- `puzzle3d_artifact_store_edit` still hardcodes per-item `coalesce_key: None`; the batch stage stamp is the live path. Leave it unless a one-item (non-batch) publisher needs the key.
- Demonstrator still mentions `setFillCountStep`; out of crate scope.
- Generated cargo logs were copied under the ticket `generated` folder for this write, then deleted.

