---
name: Puzzle2d Overlay Repaint Caching
overview: Make hover/zoom/selection as fast as drag by repainting the text-overlay canvas only when its visual inputs change, and by keeping marquee preselect local to each pane instead of fanning out through triptych React state.
todos:
  - id: epoch
    content: Add textOverlayContentEpoch and bump it in markDirty() and markSceneDescriptorDirty()
    status: in_progress
  - id: dirty-helper
    content: Add textOverlayDirty() comparing camera/size/dpr/lod/selection/preselect/epoch/theme to last painted snapshot; store lastOverlay* fields
    status: pending
  - id: paint-skip
    content: In paintTextOverlays(), early-return when not dirty and when isDraggingAreaSelect(); paint+store otherwise
    status: pending
  - id: preselect-local
    content: Stop routing marquee preselect through shell React state in the play pane (drop controlled preselection/onPreselect); add puzzle2dBroadcastPreselectSilent peer mirror and call it from updatePreselection emit path
    status: pending
  - id: tests
    content: "Extend vitest: textOverlayDirty cases (camera/selection/epoch/theme dirty, hover-only not dirty) and preselect peer broadcast without full syncDescriptorJson; run bun ./script.ts test"
    status: pending
isProject: false
---

# Puzzle2d Overlay Repaint Caching

## Why
`render()` runs `paintTextOverlays()` every frame ([puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx) line 3535). That per-node `measureText`/`fitFontPx`/`fillText` loop ([lines 4058-4159](puzzle/2d/react/index.tsx)) is the cost. Drag is fast only because it early-returns from that method via the `defersDescriptorSyncFromJs()` gate (line 4062). Hover, zoom, and select do not, so they repaint all node/handle text on all 3 panes every frame. Selection is worse because marquee preselect pushes into shell React state (`onPreselect` -> `setPreselection`) re-rendering every pane per frame.

## Part 1 - Repaint the text overlay only when it changes (primary fix)
In `Puzzle2dRenderer`:
- Add `private textOverlayContentEpoch = 0;` and bump it inside `markDirty()` ([~line 3419](puzzle/2d/react/index.tsx)) and `markSceneDescriptorDirty()` ([~line 2619](puzzle/2d/react/index.tsx)). These already fire on node position/text/visibility/topology changes; hover does NOT call them (it uses `scheduleInputInvalidate`), so hover frames leave the epoch unchanged.
- Add a small pure helper `private textOverlayDirty(): boolean` that compares the current overlay inputs to the last painted snapshot:
  - `camera.x`, `camera.y`, `camera.zoom`
  - `width`, `height`, `dpr`
  - `effectiveDrawLodLabel()`
  - `selectionStore.getSnapshot()` (ref equality; stable until changed)
  - `preselectStore.getSnapshot()` (ref)
  - `textOverlayContentEpoch`
  - `lastVelloThemeJson` (covers theme/appearance via the existing field)
- In `paintTextOverlays()`, after the existing early-returns, `if (!this.textOverlayDirty()) return;` then paint and store the new snapshot in `lastOverlay*` fields.
- Result: hover and idle frames skip the overlay entirely; zoom/select repaint exactly once when inputs change. The text loop only runs when text visually moves or recolors.

## Part 2 - Drag-like behavior for marquee selection (secondary)
- Extend the gate at line 4062 to also skip the overlay while `this.session.isDraggingAreaSelect()` is true (text positions are static during a marquee).
- Keep preselect local per pane instead of fanning out via shell React:
  - In the play pane ([framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)), stop driving preselect through controlled shell state during the gesture: drop the per-frame `onPreselect -> setPreselection` (line 1912) and the controlled `preselection` prop (line 1985) so changing the marquee no longer re-renders all 3 panes. Each pane already updates its own marquee chrome from its own WASM drain.
  - Mirror the existing selection peer broadcast: add `puzzle2dBroadcastPreselectSilent(source, snapshot)` next to `puzzle2dBroadcastSelectionSilent` (MultiViewAuthoring region, [~line 7163](puzzle/2d/react/index.tsx)) calling `peer.syncPreselectionSilent(snapshot)` on other panes, and invoke it from the emit path in `updatePreselection`. This keeps cross-pane parity without React.
- Zoom: leave `syncBaselineFromViewportCamera` (needed for redraw features); it is already coalesced to one camera emit per RAF. The Part 1 overlay cache removes the remaining per-frame zoom cost. `Puzzle2dHostSubtree.setCamera` already no-ops on equal cameras.

## Testing
- Extract the comparison into `textOverlayDirty()` so it is unit-testable without a canvas (headless-test mode early-returns from `paintTextOverlays`). Extend the existing vitest block in [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx):
  - dirty after a camera change, after a selection change, after a content-epoch bump, after a theme change;
  - NOT dirty when only a hover drain occurred (no camera/selection/epoch change).
- Add a peer test that a `preselect` drain mirrors onto a peer pane via `syncPreselectionSilent` without a full `syncDescriptorJson` (mirrors the existing selection-broadcast test).
- Run `bun ./script.ts test` in `puzzle/2d/react`; no Rust changes required.

## Out of scope
No Rust/WASM changes. `build_vector_scene()` is not the bottleneck (drag proves the GPU path is smooth).