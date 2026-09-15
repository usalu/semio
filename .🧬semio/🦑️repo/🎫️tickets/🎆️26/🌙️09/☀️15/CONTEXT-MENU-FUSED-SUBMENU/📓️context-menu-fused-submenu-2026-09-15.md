# Context Menu Fused Submenu

## Goal

Nested context menus share one window-chrome outline (no gap, no second title chip repeating the parent row).

## Change

- `ContextMenuController` renders open submenu columns as sibling flex columns inside the same `ContextMenuChrome`.
- Submenu columns align vertically with their parent row via measured `marginTop`.
- Removed portaled nested `ContextMenuChrome` per submenu parent.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/🧩️component/🟦️.tsx`
