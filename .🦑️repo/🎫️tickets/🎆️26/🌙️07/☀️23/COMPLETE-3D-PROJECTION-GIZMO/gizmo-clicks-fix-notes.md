# Fix gizmo clicks no-op

## Cause
`resolveProjectionGizmoSpec` was changed to never switch kind. Default view is `threePoint`, so face/corner hits returned the same spec and the gizmo early-returned — clicks did nothing.

## Fix
Restore navigation-cube snaps:
- face → orthographic view
- corner → axonometric (keeps Iso/Di/Tri from pane)
- center → active perspective kind or 3-Point

Pane tree still owns taxonomy/mode selection; cube owns spatial snaps again.

## Verify
- `vitest-gizmo-clicks-fix.txt`
