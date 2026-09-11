# Wave B17 — Brush and Volume Brush arm

Ticket: `26/09/02/PUZZLE-3D-END-TO-END` (moon). Vite-live host leftover. Does not revert B9–B16.

## User-visible land

Click Brush → leftover / world lane `activeUtility=brush`. Hover leftover that republishes `activeUtility=select` must not disarm it. Volume Brush arms the same way. Alt+click still adds a target volume (B11 host gesture, unchanged).

C2 / #45b:

| Verdict | Before | Hop |
| --- | --- | --- |
| `volume-brush-arm` | FAIL `activeUtility=select` | hover leftover `select` wiped the armed utility |
| `brush-preview-place` | FAIL `preview=null` | same wipe; preview gate needs `utility=brush` |
| `engagement-brush-verb` | FAIL `activeUtility=select` | same leftover overlay |

## Hop

B6 `leftoverOverlayCarryingUtilityV1` only carried when `next.activeUtility === undefined`. An InteractionView leftover that **publishes** `activeUtility=select` (guest default / hover republish) replaced the host-owned `brush` / `volumeBrush` the same way an empty-`ids` hover leftover used to drop `selectedIds` before B15.

`leftoverOverlayCarryingSelectionV1` already keeps `prior.ids` on hover leftover. Armed brush / volumeBrush now use that same rule: hover leftover `select` is not a disarm.

`SET_ACTIVE_UTILITY` still publishes the armed utility directly. Clicking Select still publishes `select` on that path — carry only refuses hover/`InteractionView` leftover `select` over an armed brush or volumeBrush.

## Fix

Host, vite-live:

- `leftoverOverlayArmedBrushUtilityV1` — `brush` | `volumeBrush`.
- `leftoverOverlayCarryingUtilityV1` — `next.activeUtility === "select"` + prior armed brush/volumeBrush → keep prior.
- `leftoverOverlayCarryingSelectionV1` — hover leftover (`hoveredId` set) with `select` keeps prior armed brush/volumeBrush, same shape as the ids carry.
- `LeftoverInteractionViewV1` / `interactionViewFromLeftoverOutput` parse optional leftover `activeUtility`.
- `applyLeftoverInteractionView` forwards that field so a guest leftover `select` hits the carry instead of being dropped as `undefined`.

B9 guest `puzzle3d_addressed_window_id` / `puzzle3d_scene_active_utility` (#46) is unchanged. B11 Volume Brush Alt+click host gesture is unchanged.

## Laws

- engine-contract `test-long`: 4 passed (882 skipped)
  - `carries the armed utility across an InteractionView leftover republish`
  - `hover leftover select keeps an armed brush or volumeBrush` (new)
  - `leftover activeUtility overlays guest select without a hover id`
  - `leftover fill tool overlays guest select so fillBuildTick still arms`
- cargo `@semio-tech/puzzle-3d-rs:test`: 4 passed, 703 skipped
  - `each_window_instance_publishes_its_own_armed_utility_into_its_world_lane`
  - `hover_committed_in_brush_publishes_preview`
  - `set_active_utility_emits_no_ops_and_no_history_entry`
  - `set_active_utility_dirties_the_world_body`

## :6014 (HTTP 200; not :6013)

- `--only=volume-brush` → `generated/probe-2026-09-11T17-02-02.md`
  - `volume-brush-arm PASS` `found=1 active=volumeBrush` (C2 was `activeUtility=select`)
  - `volume-brush-target-volume-attribute PASS`
  - `volume-brush-voxel-dims PASS`
  - `volume-brush-add-target-volume FAIL before=0 after=0` — arm is not the hop; Alt+click / ground-ray still miss on this census
- `--only=brush-stroke,engagement-bar` → `generated/probe-2026-09-11T17-02-39.md`
  - click `#brush` → poll 0 `utility=brush`; hover storm keeps `utility=brush` (never `select`)
  - 11 vortices publish after arm (`seed-left-001:v0`…`v10`)
  - leftover hover `seed-left-001:v*` overlays while utility stays `brush`
  - `brush-preview-place FAIL preview=null` — host `brush-place hop preview=null utility=brush hover=seed-left-001:v0`; leftover no longer disarms. Preview JSON is the next guest `suggestionsTick` / `brushPreview` hop (B9 #46), not this leftover wipe.
  - `engagement-input-present FAIL` — engagement pane did not expose the input; `engagement-brush-verb` not reached. World lane already reads `utility=brush`.

## Constraints held

- No git.
- No `:6013`. `:6014` only if HTTP 200.
- B9–B16 leftover / fill / selection / window-instance carry not reverted.
- Fill leftover (`activeToolId=fill` + leftover `select`) still wins via `leftoverOverlayArmedUtilityV1`.
- Hover leftover still does not invent a pick (`ids` stay empty when prior ids are empty).
