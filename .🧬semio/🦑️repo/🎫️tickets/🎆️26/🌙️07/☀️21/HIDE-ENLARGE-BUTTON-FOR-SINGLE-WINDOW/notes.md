# Hide Enlarge Button for Single Window

## Decision
Hide Mode's Focus/Unfocus (`data-slot="mode-dock-maximize"`) whenever the layout has at most one window. Enlarge only makes sense when there is another window to focus away from.

## Notes
- Wired through `ModeDockContext.canMaximize` from `modeCollectWindowIds(layoutState).length > 1`.
- Mobile already hid Focus; single-window desktop now matches that intent for Close-only chrome.
- Clearing maximize when the last peer window disappears drops any stale `maximizedStackPath`.
