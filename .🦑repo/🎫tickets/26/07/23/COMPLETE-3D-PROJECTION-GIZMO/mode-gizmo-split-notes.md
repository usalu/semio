# Mode switcher vs gizmo — no duplication

## Request
Projection modes were repeating gizmo angles. They must work in conjunction: gizmo = angle, projection pane = mode.

## Split
| Control | Owns |
|---------|------|
| Projection pane | Kind (Ortho/Axo/Obl/1Pt/2Pt/3Pt/Fish) + non-spatial variants (Iso/Di/Tri, Cab/Cav/Mil, Fish/Pan) |
| Navigation cube | Spatial angle within active mode (ortho faces → view, axo corners → quadrant/hemisphere, 1Pt faces → axis, perspective center → re-snap) |

## Code
- `worldProjectionModeOptions`: dropped Top/Bottom/Front/… and one-point X/Y/Z
- `resolveProjectionGizmoSpec`: never switches kind when a mode is active; inapplicable hits are no-ops
- `worldProjectionKindSwitchSpec("onePoint")`: emits full default including `axis` (gizmo owns axis after that)
- Gizmo `onHitSelect` skips when resolved spec equals current

## Verify
- vitest → `vitest-mode-gizmo-split.txt`
