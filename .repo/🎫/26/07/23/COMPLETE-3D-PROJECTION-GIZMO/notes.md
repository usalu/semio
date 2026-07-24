# Complete 3D Projection Gizmo — mode switch + window title

## Root cause
`Window` mounted empty `<PaneHost />` as a **sibling** of window body children, so `usePaneSlot` in `WorldOrbitProjectionSwitchPane` never saw `PaneHostContext` and rendered nothing. Only the R3F navigation cube was visible.

## Fixes
1. **PaneHost** — children are siblings of the `pointer-events-none` portal mount under a relative root (`pane-host-root`), so canvas hit-testing stays intact while deep hosts receive context.
2. **Window** — wraps body content in `<PaneHost>` instead of a sibling empty host.
3. **WorldOrbitProjectionSwitchPane** (framework + CAD) — overlay fallback when no portal container is ready / available.
4. **SET_WINDOW_TITLE** + `windowTitlesById` — projection kind switch and gizmo snaps call `worldProjectionSpecLabel` and retitle the Mode window.

## Verification
- `framework/renderer/react` vitest: 250 passed (includes SET_WINDOW_TITLE + kind-switch markup).
- `infinite/world/r3f` vitest: 122 passed.
