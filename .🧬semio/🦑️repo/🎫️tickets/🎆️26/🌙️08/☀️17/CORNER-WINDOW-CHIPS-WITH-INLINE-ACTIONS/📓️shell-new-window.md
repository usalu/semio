# ShellHost `onWindowOpenInNewWindow` Wiring

## Summary

Wired `<Mode onWindowOpenInNewWindow={...} />` in `ShellHost` so the dock chip / hotkey opens a **second live instance** of the same window kind in its own stack (split to the right), not a native OS window.

## Behavior

1. Resolve `windowKindId` from `extraWindowInstances` (extra instance id) or `session.app.windowKinds` (primary kind id).
2. Mint `instanceId` via `extraWindowCounterRef` as `` `${windowKindId}-${counter}` `` (same pattern as `handleTemplateDrop`).
3. Append `{ id, windowKindId, title }` with `SET_EXTRA_WINDOW_INSTANCES`.
4. `refreshUi(session, { kind: "full" }, nextExtraInstances)` so the new pane gets body/measures/engagement.
5. Update shell layout:
   - `resolveStackPathForWindowId(layout, windowId)` locates the source stack.
   - `splitWithWindow(..., "right")` inserts the new window in an adjacent stack (safe because the new id is not yet in the tree, so the internal remove is a no-op).
   - If the source window is missing from layout, fall back to `insertWindowAtDropZone(..., { kind: "root-split", side: "right" })`.
6. `SET_ACTIVE_WINDOW_ID` to the new instance.
7. `noteShellCommand("shell.windowOpenInNewWindow", ...)`.

## Supporting Changes

| Area | Change |
|------|--------|
| Canvas | Exported `resolveStackPathForWindowId` |
| `@semio-tech/ui-react` | Re-exported helper; EN/DE `ui.shellCommand.windowOpenInNewWindow` |
| I18n schema | Added `shellCommand.windowOpenInNewWindow` |
| ShellHost | Import `splitWithWindow` + `resolveStackPathForWindowId`; Mode prop handler |

## Labels

- EN: `Open in New Window`
- DE: `In neuem Fenster öffnen`

## Files

- `ShellHost/🟦️component.tsx`
- `Canvas/🟦️component.tsx`
- `ui/.../⚛️react/📦️index.tsx`
- `I18n/🟦️component.tsx`
