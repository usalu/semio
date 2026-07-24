# Fix Puzzle 3D Stuck After Context Menu Suggestion

## Symptoms
After clicking a suggestion in the context-menu picker, the 3D view can stick:
- Vortex remains selected / hover stays
- Group (marquee) selection still works
- Context menu on another vortex does nothing
- Background click does nothing

## Root causes
1. `acceptSuggestion` only cleared `runtime.suggestion_menu` on full placement success. Any failed preview/place left `suggestionMenu.open: true`, which globally suppresses the regular `ContextMenuController` on **every** pane.
2. Suggestion menu renders only for the opening `windowId`, so the sibling split pane has the global gate but no menu and no outside-dismiss listener — feels stuck with no visible popup.
3. `closeOnSelect={false}` (from the finalize fix) made closing 100% dependent on that Rust clear path.
4. Accept/close never cleared `hovered_vortex_full_id`; accept re-bound vortex selection.
5. Right-click on a vortex arms connect-drag (no button filter); clicking the portaled menu never delivers host `pointerUp`, so connect/orbit gate can linger until a later host gesture.
6. `wasMarqueeDragRef` was set on pointer-up but only cleared on a later non-drag pointer-up — a stale `true` made the next `onPointerMissed` empty-click no-op.

## Fix plan
- Always clear `suggestion_menu` (+ hover) on accept attempt; clear selection after successful one-shot place.
- Clear hover on `closeVortexSuggestions`.
- Scope regular context-menu suppression to the owning suggestion `windowId`.
- Empty-click closes suggestions; consume `wasMarqueeDragRef` correctly.
- Cancel connect-arm when suggestions open; vortex `pointerDown` primary button only.
- Regression tests for failed accept still closing the menu.
