# Window Options Tree Icon Checkbox

## Goal
`R26-02/RUNNING-SKETCHPAD` (repo MCP unavailable; ticket folder created manually).

## Change
Window-option toggle rows now render:
- semantic `iconId` before the label (`data-slot="tree-icon"`)
- a compact `TreeCheckbox` on the right instead of an icon `Toggle`

Shared `TreeCheckbox` extracted from tree header checkbox actions.

## Verify
- `bunx vitest run -c 🧪️vitest.config.ts -t "puts the toggle icon before the label"` → pass
- `bunx vitest run -c 🧪️vitest.config.ts -t "WindowMeasuresTree"` → see log
- renderer suite currently fails to load (`framework-surface-paint-rs` pkg missing); coverage lives in UI leaf test + `renderWindowMeasuresTree` unit (blocked by env)
