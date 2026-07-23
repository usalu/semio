# Fix Flow Context Menu

## Bugs

1. Right-click moved nodes: `BoardEngine::pointer_down_screen` started drag on any button when hitting a node. DAG skipped rectangle-drag for `button != 0`, then fell through to the engine which still dragged.
2. Preview/zoom/clear missing: menu rows were selection-gated and opened from stale `contextMenuJson` before `contextMenuAt` round-tripped.

## Fixes

- `infinite/board`: secondary/middle only select/hover — never drag; empty secondary keeps selection.
- `flow/plugin`: always emit preview / zoom / clear / delete (disabled when empty).
- `FlowGraphCanvasHost`: ignore secondary pointerDown/Up; pick target; enrich menu for effective selection immediately.
- Inspector `UiInspectorFieldGroup` missing `presence` (compile break).

## Verification

- `infinite_board` `secondary_pointer*`: 2 passed
- `flow-plugin` `context_menu*`: 3 passed
- vitest enrich/map context menu: 2 passed
