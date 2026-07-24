# Fix Puzzle 3D Stuck After Context Menu Suggestion

## Symptoms
After clicking a suggestion in the context-menu picker, the 3D view can stick:
- Vortex remains selected / hover stays
- Group (marquee) selection still works
- Context menu on another vortex does nothing
- Background click does nothing

## Root causes
1. `acceptSuggestion` only cleared `runtime.suggestion_menu` on full placement success. Any failed preview/place left `suggestionMenu.open: true`, which globally suppressed the regular `ContextMenuController` on **every** pane.
2. Suggestion menu renders only for the opening `windowId`, so the sibling split pane had the global gate but no menu and no outside-dismiss listener — felt stuck with no visible popup.
3. `closeOnSelect={false}` (from the finalize fix) made closing 100% dependent on that Rust clear path.
4. Accept/close never cleared `hovered_vortex_full_id`; accept re-bound vortex selection and left it after place.
5. Right-click on a vortex armed connect-drag (no button filter); clicking the portaled menu never delivered host `pointerUp` to cancel it.
6. `wasMarqueeDragRef` was set on pointer-up but only cleared on a later non-drag pointer-up — a stale `true` made the next `onPointerMissed` empty-click no-op.

## Fixes
- Rust `acceptSuggestion`: always dismiss menu + clear hover first; clear selection after successful one-shot place.
- Rust `closeVortexSuggestions`: also clear sticky vortex hover.
- React: scope regular context-menu suppression to owning window via `worldSuggestionMenuOwnsWindow`.
- Sibling panes get Escape / outside-dismiss while suggestion is open elsewhere.
- Empty-click closes suggestions; consume `wasMarqueeDragRef` correctly.
- Cancel connect-arm when opening/accepting/closing suggestions; vortex `pointerDown` primary button only.
- Unrelated compile unblock: `IntroductionAdvance::Tool` match arm in framework plugin validation.

## Verification
- `cargo test -p puzzle-plugin accept_suggestion_` → 3 passed (incl. failed-place still closes menu)
- `cargo test -p puzzle-plugin close_vortex_suggestions` → 2 passed (incl. sticky hover clear)
- `vitest` `scopes suggestion menu ownership` → passed
