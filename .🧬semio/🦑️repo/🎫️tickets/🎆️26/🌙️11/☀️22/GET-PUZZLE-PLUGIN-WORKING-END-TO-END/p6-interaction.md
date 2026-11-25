IN WORK
# p6-interaction: 5D bugs of 09/23 and the four 2D battery failures

Logs: 🗑️generated/p6-*. Script: p6-run.sh. Nothing below counts as green unless a log line states it.

## 2D battery failures (E7, 2026-09-17), current-code verdict by reading
1. engagement-move: framework `WindowSearch` `applyDraft` (🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔍️WindowSearch/🟦️.tsx:354) passes the typed line verbatim; the law test "Search keeps an argument line verbatim" (🧪️tests/🧪️owned-locale-detector-retirement/📚️shell-components-8) already exists. Fixed in source, TS test NOT run by me.
2. undo-changes-document: fill finalize is one history entry; laws `fill_run_start_complete_finalize_is_one_undo_entry` and `a_hundred_placement_fill_is_one_history_entry_that_undoes_and_redoes` exist in 2D fill tests. Fixed in source, NOT run by me.
3. import-round-trip: probe-only; probe `actionsOpen(windowId)` is window-scoped and FAULT_RE excludes the stale banner. Fixed in probe, not run.
4. inspector-fresh-on-open: only `sync_descriptor_with(.., false)` (snapshot parse) is used by the host; `syncDescriptorJson` (announces edges) has no TS caller outside doubles. Not run.

Runs are masked (brief rule 12, mask-dbg.sh sourced first; sha lines are at the top of each p6-*.log). Queued with 3500 MB.

## 5D bugs
(see below, updated after the runs)

### Tests written (not yet compiled or run, no green claim)
- `background_click_clears_the_world_pane_selection` in ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/☑️option-groups-and-edits/🦀️.rs (package semio-s-artifact-puzzle-5d-v1-unit-tests). Two clicks: the world host's empty `interactionSelect` (replace, `[]`) and the board's `applyBoardEvents` `select` with `ids:[]`; after each, the shared selection and the world scene (`selectionJson.ids`, `instancesJson[].selected`) must be empty.
- `a_catalogue_row_payload_is_admitted_by_the_board_and_the_world_ghost` in 🖐️5d …/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs (package semio-s-artifact-puzzle-5d-host-admission-tests): one drag payload must satisfy the board parser (`kindId`, ghost extent) and the world parser (`objectKind`).

### Findings from reading (current code)
- Bug 1 (background click): Rust side looks correct. Framework `apply_interaction_verb` clears the domain on empty `Replace`; the 5D board `select` event pushes `InteractionWrite::replace(..., [])` and `apply_interaction_writes` clears on empty Replace; `Puzzle5dInteractionSnapshot::from_interaction` falls back to `leftover_selected_ids` only when the vortex selection is empty, same as 2D/3D. The 5D world scene binds `domainId`/`domainGranularityId`. The oracle `world3d-gestures-and-selection` pins the host's empty-click. Not proven fixed or broken: the queued test decides. If it passes, the bug lives in the browser host (needs a browser run).
- Bug 2 (drag preview in both windows): both hosts read the same payload from `getActiveCatalogueDragPayload`; the board host registers pointer/drag listeners and pushes `setDropPreviewJson` to all peers, 5D payload carries `kindId`+`objectKind`. No defect found by reading; needs a browser run to see the ghost. The contract test above is the only evidence prepared.
- Residual: wasm `syncDescriptorJson` still announces new edges (event-credit family of E7 #4) but has no TS caller outside test doubles.

### Run state
`p6-run.sh` (masked, rule 12; 3500 MB, queued behind other agents for over 10 minutes: "not enough free memory") for `-p semio-s-artifact-puzzle-5d-v1-unit-tests --lib background_click`, log 🗑️generated/p6-5d-bg.log (absent until it starts). No result yet. The launcher may have died with my session; rerun with:
`./p6-run.sh "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d" p6-5d-bg.log "-p semio-s-artifact-puzzle-5d-v1-unit-tests --lib background_click"` and the same for `-p semio-s-artifact-puzzle-5d-host-admission-tests --lib a_catalogue_row_payload`.
