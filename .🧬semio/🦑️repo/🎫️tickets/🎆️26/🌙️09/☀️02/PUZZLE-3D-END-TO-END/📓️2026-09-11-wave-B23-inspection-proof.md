# Wave B23 — First-pick Inspection leftover.ids

Ticket: `26/09/02/PUZZLE-3D-END-TO-END` (moon). Did not write to the bridge `🌉09` duplicate.

## Probe (coordinator — do not re-run)

`:6014` vite **dev**, wasm mtime **20:53**. Coordinator already ran `bun 🔍️browser-probe.ts --only=selection-surfaces --port=6014`.

Logs:

- `🗑️generated/b23-selection-surfaces-6014.txt`
- `🗑️generated/probe-2026-09-11T19-04-04.md` / `.ndjson`

Evidence after first viewport pick of leftover.ids `seed-left-001`:

- leftover InteractionView: `selectedIds:["seed-left-001"]`, `publishedIds:["seed-left-001"]`, `gumball:true`
- leftover Inspection refresh: `epoch:1, selectedIds:["seed-left-001"], hashBust:true, cached:"5bd00904:1"`
- Inspection after: `populated=false empty=true id=null` — body stayed `puzzle3d-play-inspector.empty`
- Battery `PASS=4 FAIL=2` (`inspection-object-fields`, `inspection-locked-flag-row`)

C3 ranks this #1. B19 overlay did not reach `inspection::render` on this guest (or hash-bust still served the empty body). Hover-only is not treated as a pick.

## Hop

Leftover.ids already published on leftover InteractionView. Persist `selection(vortex)` stays empty after topology prune. Host leftover Inspection refresh hash-busts. Guest `inspection::render` still reads an empty `Puzzle3dInteractionSnapshot`.

Landed (merge, do not revert B9 / B13 / B15 / B19 / B20 chips / B21 refresh_cache):

1. Guest leftover retain (`plugin.rs`): `interaction_leftover_ids` updates only when leftover selectedIds are nonempty. Empty leftover (hover or miss) overlays those leftover.ids back onto `vortex` — never invents leftover.ids from `hoveredId`.
2. `interaction_selection_snapshot` overlays `interaction_leftover_ids` into `vortex` when persist vortex is empty. That snapshot is what `Puzzle3dInteractionSnapshot::from_interaction` / `inspection::render` reads.
3. Host `leftoverOverlayCarryingSelectionV1`: keep prior leftover.ids on empty leftover even without `hoveredId`. Still does not invent leftover.ids from hover.
4. Host leftover Inspection refresh captures leftover.ids at the epoch bump (so a later empty leftover cannot skip the refresh) and deletes Inspection/inspector cache keys when hash-busting.

## Laws

- cargo `leftover_ids_name_object_empty_persist_vortex_from_state_wires_inspection_snapshot` **ok**
- cargo `leftover_ids_name_object_empty_persist_vortex_paints_object_fields_and_lock_row` **ok**
- cargo `leftover_ids_overlay_when_vortex_selection_empty_wires_object_fields_and_lock_row` **ok**
- cargo `leftover_browser_shaped_snapshot_wires_namespaced_object_fields_and_lock_row` **ok**

## Rematerialize

Guest rust changed (`plugin.rs` leftover retain). A `materialize-dev --skip-nx-cache` was already running — did not start a second compile. Did not recycle `:6014`. Did not touch `:6013`. Did not re-run selection-surfaces.

Browser proof of the hop requires the next `materialize-dev` to include leftover retain, then a first-pick Inspection `populated=true` + lock `flag_row` for `seed-left-001`.

## Constraints held

- B9 guest topology not reverted.
- B13 leftoverInspectionPanelHash omit-when-ids-nonempty not reverted.
- B15 SELECT-arm hover fallback and leftoverOverlayCarryingSelectionV1 hover-is-not-a-pick not reverted.
- B19 leftover overlay / leftover.ids snapshot not reverted.
- B20 `Window` `data-slot="window-engagement-quick-actions"` not reverted.
- B21 `take_typed_operation_completion` refresh_cache-before-dirty-guard not reverted.
- No git. No worktrees. No `:6013`.
