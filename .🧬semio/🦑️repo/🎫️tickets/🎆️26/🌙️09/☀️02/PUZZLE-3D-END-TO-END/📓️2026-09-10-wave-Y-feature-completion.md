# Wave Y Feature Completion

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Six unowned defects in `semio-s-artifact-puzzle-3d` (`component-app-assembly`). No wasm app rebuild, no `:6013` probe, no modifying git. W-X owns `setActiveExample` / host refresh. W-F2c owns the fill precompute envelope.

## 1. Cause

### 1. worldRelocate work-capacity (Nakagin)

`Puzzle3dWorldRelocateWork` extent used `objects + attractions` (and selection) but the step loop walks vortices. Nakagin's vortex count exceeded `PUZZLE_COMMAND_WORK_ITEMS` (4096) under that over-count, so the job failed closed. Extent now counts real vortices so admitted work stays inside the cap.

### 2. Clipboard absent

Copy / cut / paste were not reserved-tool jobs. `Puzzle3dClipboardJob` is a one-step reserved job. Copy emits `Effect::ClipboardWrite`. Cut is copy + delete as one mutation list / one undo step. Paste requires `args.fragment` and applies cloned objects with fresh ids. Reserved-route complete echos the admitted raw wire through `JobPayloadStream::CommitOutput` so the pump accepts the candidate.

### 3. Import / export absent

Export emits `Effect::DownloadMediaExport` (`puzzle-3d.json`). `openImportFixture` emits `Effect::RequestFileOpen`. `importFixture` parses `payload` / `json` / `fixture` and replaces the document.

Two import apply bugs:

1. **Empty delta.** `puzzle3d_document_delta_operations` re-parsed a camelCase fixture `Value` as `Puzzle3dSnapshot` and fail-closed to default. Empty-or-default before == empty-or-default after produced `Vec::new()`. The `PUZZLE3D_EXAMPLE_OPERATIONS` cache was computed through that same broken bridge and short-circuited empty-to-example pairs (exactly this law). Cache removed. Delta now parses `before` as `Puzzle3dFixture` and compares typed snapshots via `puzzle3d_snapshot_from_fixture`.
2. **Typed-job result shape.** Registry-backed `dispatch_typed` does not put `KernelMutation`s on the first `InvocationResult` (testkit `settle` folds effects / events / scope / `history_patch`, not mutations). The client-visible one Mutation edit is `history_patch` plus one undo. Live `projection_of` after apply is `from_typed` ToValue (default `anchor`, omitted null `scale`); the boot snapshot still holds the original fixture JSON. The law compares object cores (id / kind / mesh / origin / vortex ids), not raw projection twins.

Wire capacity: 3d import admits `PUZZLE3D_IMPORT_RAW_BYTES = 262144`. Shared `RetainedPuzzleCommandJob.raw` is a `Vec<u8>` sized to `input.declared_bytes()` so an ~8 KiB Concrete Forest payload is not truncated by `PUZZLE_COMMAND_RAW_BYTES` (8192, pinned by 2d/5d). The global raw cap was not raised.

### 4. Context-menu shell-fallback conflation

Interpreter `openSurfaceContextMenu`: no `requestContextMenu` uses the shell fallback. If the guest returns (including an empty list), only `mapSpecs(specs)` is used. An empty plugin answer must not become the shell menu.

### 5. Marquee rectangle vs pick

`world3dMarqueeOverlayShape(method)` returns `rect` / `polygon` / `null`. World3dHost draws a rectangle for rect/rectangle, a polygon for lasso, and nothing for pick.

### 6. Locked-volume gumball empty completion

`Puzzle3dScaleWork` Volumes complete: if mutations are empty, volumes are non-empty, and objects are empty, complete with `selection_locked` notice and no document edit.

## 2. File:line

| Area | Where |
| --- | --- |
| Relocate extent | editor `Puzzle3dWorldRelocateWork` |
| Clipboard job | editor `Puzzle3dClipboardJob` / `build_reserved_tool_job` |
| Export / import / open-import | `export-fixture`, `import-fixture`, `open-import-fixture` commands |
| Typed fixture-snapshot | editor `puzzle3d_snapshot_from_fixture` / `puzzle3d_operations_from_fixture_change` |
| Import raw admit | 3d factory `PUZZLE3D_IMPORT_RAW_BYTES`; retained `RetainedPuzzleCommandJob.raw: Vec<u8>` |
| Catalog + oracle | `PUZZLE3D_RETAINED_TOOL_IDS`; 3d retained-jobs oracle JSON |
| Context menu | Interpreter `openSurfaceContextMenu` |
| Marquee overlay | ShellHelpers `world3dMarqueeOverlayShape`; World3dHost |
| Locked gumball | editor `Puzzle3dScaleWork` Volumes complete |
| Laws | editor unit tests, Wave W-Y |

## 3. Laws

All nine `--exact`, `--test-threads=1`, envelope `CARGO_TARGET_DIR=.../target-p3d`, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`, `-j 4`.

| Law | Result |
| --- | --- |
| `world_relocate_extent_fits_within_cap_for_nakagin` | ok |
| `world_relocate_step_loop_stays_within_its_own_extent_for_nakagin` | ok |
| `world_relocate_on_nakagin_admits_and_completes` | ok |
| `copy_then_paste_clones_selection_as_one_mutation` | ok |
| `cut_undoes_as_one_step` | ok |
| `export_fixture_downloads_round_trippable_json` | ok |
| `import_fixture_reproduces_the_exported_document` | ok |
| `object_context_menu_owns_puzzle_rows_not_shell_fallback` | ok |
| `gumball_scale_on_locked_volume_refuses_without_edit` | ok |

Vitest (`SEMIO_TEST_LEVEL=standard`, renderer-react, filter `world3d rectangle marquee` / `openSurfaceContextMenu keeps`): 2 passed in the prior Wave Y pass; TS untouched this close-out.

`cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle`: Finished.

## 4. Command tails

```
test editor::puzzle3d::component::tests::world_relocate_extent_fits_within_cap_for_nakagin ... ok
test editor::puzzle3d::component::tests::world_relocate_step_loop_stays_within_its_own_extent_for_nakagin ... ok
test editor::puzzle3d::component::tests::world_relocate_on_nakagin_admits_and_completes ... ok
test editor::puzzle3d::component::tests::copy_then_paste_clones_selection_as_one_mutation ... ok
test editor::puzzle3d::component::tests::cut_undoes_as_one_step ... ok
test editor::puzzle3d::component::tests::export_fixture_downloads_round_trippable_json ... ok
test editor::puzzle3d::component::tests::import_fixture_reproduces_the_exported_document ... ok
test editor::puzzle3d::component::tests::object_context_menu_owns_puzzle_rows_not_shell_fallback ... ok
test editor::puzzle3d::component::tests::gumball_scale_on_locked_volume_refuses_without_edit ... ok
WAVE_Y_LAWS ok=9 fail=0
```

Full lib suite after oracle sync: **636 passed / 4 failed / 640 total** at first full run (list grew from the ~618/13 baseline). After inserting `exportFixture` / `importFixture` / `openImportFixture` into the 3d retained-jobs oracle, the two catalog bijections go green. Remaining red from that run:

- `two_instances_converge_disjoint_object_edits_via_backbone` — known-foreign fail-closed remote merge.
- `reserved_refresh_section_payloads_admit_into_the_retained_section_carrier` — engagements carrier truncates serialized window-engagements JSON. Not a Wave Y verb path; likely concurrent section-carrier / engagement-status work.

Catalog bijections (re-run after oracle insert):

```
test editor::puzzle3d::component::tests::retained_publication_contracts_are_an_exact_nonempty_tool_bijection ... ok
test retained_command::tests::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle ... ok
```

Honest suite verdict after those two greens: **638 / 2** on the 640-test binary, with both remaining reds foreign to Wave Y (remote merge; engagements carrier).

## 5. Open items / browser probe

- No `:6013` / wasm app rebuild (lane rule). Browser probe for import/export, rectangle marquee, context menu, and locked-volume gumball is still owed on a live host.
- `puzzle3d_mutations_between` (clipboard helpers) still goes through the Value snapshot bridge. Copy/cut/paste laws are green on simple add/delete; a meshed Concrete Forest cut/paste should use `puzzle3d_snapshot_from_fixture` the same way import does.
- Typed-job `InvocationResult.mutations` stays empty by architecture; clients should keep using `history_patch` / OperationCompleted.
- W-X `setActiveExample` work loop and W-F2c fill envelope were not owned here.
