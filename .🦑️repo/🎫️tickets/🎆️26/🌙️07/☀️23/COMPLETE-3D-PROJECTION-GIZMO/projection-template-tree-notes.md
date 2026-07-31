# Projection pane = template tree; folded by default

## Request
1. Projection chrome must show the same taxonomy tree as Display window templates (`createWorldProjectionTemplates`).
2. Pane should start toggled off (folded).

## Change
- `WorldProjectionKindSwitch` now renders a `Tree` built from `createWorldProjectionTemplates` (Parallel → Orthographic/Plan/…, Axonometric, Oblique; Perspective → 1/2/3-Point, Curvilinear).
- Helpers: `worldProjectionTemplateSelectionId`, `worldProjectionTemplateApplySpec` (keeps gizmo angles when staying in-family), `worldProjectionSwitchTreeItems`.
- Framework + CAD `WorldOrbitProjectionSwitchPane`: `folded` defaults to `true`.

## Verify
- `vitest-projection-template-tree.txt` — 136 passed
