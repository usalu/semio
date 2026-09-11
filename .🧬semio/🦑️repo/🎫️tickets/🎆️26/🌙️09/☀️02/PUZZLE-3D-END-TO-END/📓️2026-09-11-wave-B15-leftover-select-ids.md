# Wave B15 — leftover selectedIds on first pick

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Continues B13: leftover after `interactionSelect` was
`selectedIds:[]` while `hoverTarget.id` was `seed-left-001` (`probe-2026-09-11T15-24-52.md`).

## Root cause

`dispatch_interaction_action` `INTERACTION_SELECT` runs `protocol::next_selection` on `args.targets`.
A click that hits as hover but publishes `targets: []` (or a topology prune of those targets) leaves
`state.selection` empty. Leftover is built from that state *before* persist, so the host hash-bust
(B13) never sees ids. Hover leftover carry only runs on `INTERACTION_HOVER`, not on this empty select.

## Fix (already landed — do not revert)

When `next.ids` is empty after `next_selection`, the SELECT arm copies the first `interaction_hover`
id for that domain into the selection (granularity `object` if unset). Leftover `selectedIds` then
matches the pointer.

File: `framework/os/plugin/rs` SELECT arm.

`leftoverOverlayCarryingSelectionV1` still must **not** treat `hoveredId` as a selection when
`prior.ids` is empty — that leftover is hover, not a pick.

## Law

Fails if `interactionSelect` with empty `targets` leaves leftover `selectedIds` empty while hover
names an object:

- cargo `empty_target_interaction_select_leftover_selected_ids_name_the_hovered_object`
  in puzzle-3d unit + plugin-builder-contract
- vitest `empty-target interactionSelect leftover selectedIds names the hovered object`
  (`leftoverSelectIdsMustNameHoverPickV1`; hover leftover with empty prior stays `ids: []`)

## Verify

- cargo `semio-s-artifact-puzzle-3d` `--features component-app-assembly --lib`
  `empty_target_interaction_select_leftover_selected_ids_name_the_hovered_object` **ok**
- cargo `semio-framework-plugin` leftover filter (`--test-threads=1`):
  `empty_target_interaction_select_leftover_selected_ids_name_the_hovered_object` **ok**,
  `interaction_hover_job_completion_publishes_hover_target_on_leftover` **ok**,
  `interaction_select_job_completion_publishes_interaction_view_on_leftover` **ok**
- cargo leftover peers (name-filtered, skip B16 fixture overflow):
  `leftover_copy_paste_clones_*` **ok**,
  `hover_after_first_pick_keeps_the_object_and_its_lock_row` **ok**,
  `interaction_hover_leftover_carries_vortex_full_id` **ok**
- cargo `leftover` filter also matches B16 `leftover_open_import_fixture` /
  `leftover_export_fixture` / `leftover_import_fixture` — those **stack-overflow**
  (peer, not B15). Do not treat as this law failing.
- vitest `SEMIO_TEST_LEVEL=contract` engine-contract
  `empty-target interactionSelect leftover selectedIds names the hovered object`
  **ok** (`selectedIds=["seed-left-001"]`; hover-only carry stays `ids=[]`)
- `:6014` HTTP 200. `bun browser-probe.ts --only=selection-surfaces --port=6014`.
  Log: `generated/probe-2026-09-11T16-48-30.md`.
  Leftover taps: empty + hover `seed-left-001` (hover leftover; prior ids empty,
  overlay must not invent a pick), then
  `selectedIds:["seed-left-001"]` + `gumball:true` on later `interactionSelect`.
  Step `selection-surfaces: ok`. Battery `PASS=4 FAIL=2`. Guest alive.
  Inspection still `populated=false` / empty summary (`expect-42`) — leftover
  parse is not dropping ids. Host Inspection bind/hash hop remains.

## Constraints held

- SELECT-arm hover fallback in plugin `rs` not reverted.
- `leftoverOverlayCarryingSelectionV1` still carries **prior** ids only; hover
  leftover with empty prior stays `ids: []`.
- No git. No `:6013`.
