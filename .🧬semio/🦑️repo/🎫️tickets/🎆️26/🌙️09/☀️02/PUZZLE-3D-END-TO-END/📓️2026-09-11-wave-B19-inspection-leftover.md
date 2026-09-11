# Wave B19 — Inspection leftover.ids snapshot

Ticket: `26/09/02/PUZZLE-3D-END-TO-END`

## Symptom

On `:6014` leftover now publishes `selectedIds:["seed-left-001"]` (B15). B13 hash-bust omits the cached Inspection hash when leftover ids are nonempty. Inspection still rendered `id=null` / empty summary (`expect-42`). Leftover parse is not dropping ids.

## Hop

`inspection::render` already paints object fields + `object.locked` `flag_row` when the snapshot carries leftover-shaped ids (empty or unresolved granularity). The live hop was: leftover.ids exist on the host, `Puzzle3dInteractionSnapshot` still read empty persist `selection(vortex)`.

`leftoverInteractionStateV1` overlays leftover.ids onto `vortex` when that domain is empty (including an empty `vortex.ids` entry). Host `INTERACTION_STATE_OBSERVED` does not reach `inspection::render`. Guest `from_interaction` / `from_state` used to read `selection(vortex)` only. Persist can be empty after topology prune while leftover.ids still name the pick (leftover is pre-revalidate).

B19 probe on `:6014` (pre-rebuild wasm) confirmed the host hop: leftover Inspection refresh fired `{epoch:1, selectedIds:["seed-left-001"], hashBust:true}` and leftover InteractionView published `selectedIds:["seed-left-001"]`. Guest Inspection body stayed `puzzle3d-play-inspector.empty`.

`:6014` is `workspace:dev -- 5d` (vite profile `dev`). It serves `dist/dev/plugin-modules/puzzle/semio_s_plugin_puzzle_component.core.wasm`. That file was dated 2026-09-10 20:16 — B15 leftover `selectedIds` was in it; leftover overlay + leftover.ids snapshot were not.

## Fix (do not revert B9 / B13 / B15)

1. `leftoverInteractionStateV1` overlays leftover.ids into `vortex` when `vortex.ids` is empty, even if the leftover selection map already has an empty vortex entry.
2. `InteractionView::leftover_selected_ids` flattens leftover.ids from every domain.
3. `Puzzle3dInteractionSnapshot::from_interaction` / `from_state` use leftover.ids when `selection(vortex)` is empty, and object granularity when leftover.ids land with empty granularity. That snapshot is what `inspection::render` uses.
4. `interaction_selection_snapshot` leftover overlay + leftoverInteractionStateV1 twin: leftover.ids land on `vortex` when persist vortex is empty.
5. `overlay_leftover_ids_into_vortex` runs on the just-computed InteractionView before leftover publication and leftover overlay retain, so leftover.ids are the snapshot Inspection reads after prune.
6. Forced `bun nx run @semio-tech/puzzle-plugin:materialize-dev --skip-nx-cache` so nx would not replay the Sep-10 `component-dev` cache (framework `plugin.rs` is outside puzzle namedInputs). Staged `dist/component-dev` then materialized into the vite `plugin-modules` path `:6014` serves.

## Laws

- cargo `leftover_ids_overlay_when_vortex_selection_empty_wires_object_fields_and_lock_row` **ok**
- cargo `leftover_browser_shaped_snapshot_wires_namespaced_object_fields_and_lock_row` **ok**
- cargo `leftover_vortex_granularity_unresolved_falls_back_to_object_fields` **ok**
- cargo `leftover_selected_vortex_uuid_falls_through_to_object_fields` **ok**
- cargo `hover_after_first_pick_keeps_the_object_and_its_lock_row` **ok**
- vitest engine-contract `SEMIO_TEST_LEVEL=contract`:
  `first leftover pick keeps selection across hover leftover and busts the Inspection hash skip` **ok**
  (includes leftoverInteractionStateV1 overlay when leftover `selection.vortex.ids` is `[]`)
  `empty-target interactionSelect leftover selectedIds names the hovered object` **ok**

## Probe

- First probe (pre-rebuild wasm): `:6014` HTTP 200. `bun browser-probe.ts --only=selection-surfaces --port=6014`.
  Log: `generated/probe-2026-09-11T17-39-24.md`.
  Leftover taps: `selectedIds:["seed-left-001"]`, gumball true, leftover Inspection refresh hash-bust.
  Step `selection-surfaces: ok`. Battery `PASS=4 FAIL=2`. Guest alive.
  Inspection still `populated=false` / empty summary (`expect-42`).
- Wasm rebuild: `generated/component-dev-materialize-2026-09-11.log`. NX Successfully ran `materialize-dev` in 4m 18s (`--skip-nx-cache`). Deliverable mtimes 2026-09-11 20:07:
  - `✏️s/.../puzzle/.../dist/component-dev/semio_s_plugin_puzzle.wasm`
  - `framework/.../plugin/.../dist/dev/plugin-modules/puzzle/semio_s_plugin_puzzle_component.core.wasm`
- Second probe: `:6014` answered HTTP 200 once, then vite pid 18776 disappeared. `bun browser-probe.ts --only=selection-surfaces --port=6014` got `ERR_CONNECTION_REFUSED` at `/?plugin=puzzle3d`, `booted=false`, verdicts=0.
  Log: `generated/probe-2026-09-11T18-13-18.md`. Did not restart `:6014`. Did not touch `:6013`. When `:6014` is back, re-run the same probe and expect Inspection object fields + lock `flag_row` (`populated=true`, leftover.ids `seed-left-001`).

## Constraints held

- B9 guest topology not reverted.
- B13 leftoverInspectionPanelHash omit-when-ids-nonempty not reverted.
- B15 SELECT-arm hover fallback and leftoverOverlayCarryingSelectionV1 hover-is-not-a-pick not reverted.
- No git. No `:6013`.
