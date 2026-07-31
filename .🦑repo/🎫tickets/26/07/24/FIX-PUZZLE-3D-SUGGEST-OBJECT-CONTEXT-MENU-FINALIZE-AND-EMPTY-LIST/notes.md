# Fix Puzzle 3D Suggest Object Context Menu Finalize and Empty List

## Symptoms
- Choosing a suggestion sometimes does not place the object (menu closes, nothing added).
- Clicking "Suggest objects" sometimes shows no candidate list; retrying sometimes works.

## Root causes
1. Default layout mounts two `World3dHost` panes (top + perspective). Both rendered the suggestion `ContextMenuController` when `suggestionMenu.open`. Each pane's outside-`pointerdown` listener treated the *other* pane's menu as outside → `closeVortexSuggestions` on the click's `pointerdown`, often unmounting the menu before `click`/`acceptSuggestion` ran.
2. Selecting a suggestion always called `onClose` → `closeVortexSuggestions` in parallel with `acceptSuggestion`, amplifying the race.
3. Opening suggestions could reuse a stale empty brush-candidate cache entry (`free: []`, `unknownPending: false`), so the popup showed "No placement" / felt empty even when kinds existed; a later retry after cache invalidation worked.

## Fixes
- `ContextMenuController`: ignore outside dismiss when the event targets any `[role=menu]` (sibling panes); arm outside-dismiss after a 0ms timeout so the opening gesture cannot dismiss; `closeOnSelect={false}` for the suggestion popup so accept owns closing.
- Suggestion menu renders only for the opening `windowId`.
- `openVortexSuggestions` invalidates + refreshes brush candidates for the vortex; menu JSON includes `windowId` + `vortexFullId`.
- `acceptSuggestion` accepts `fullId` and re-binds selection + refreshes candidates before placing.

## Verification
- cargo puzzle-plugin suggestion tests
- vitest ContextMenuController sibling / closeOnSelect tests
