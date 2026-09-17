# Context Menu Fused Submenu

## Goal

Nested context menus share one window-chrome outline (no gap, no second title chip repeating the parent row).

## Change (2026-09-17)

- Submenu columns are `w-max`, absolutely placed at the parent row (not a full-height sibling column).
- Cap row shrink-wraps (`capFitContent`) so the title chip does not span the wing.
- `WindowSilhouette` builds a stepped polygon outline from measured primary + wing rects (`contextMenuFusion` on metrics).

- `ContextMenuController` renders open submenu columns as sibling flex columns inside the same `ContextMenuChrome`.
- Submenu columns align vertically with their parent row via measured `marginTop`.
- Removed portaled nested `ContextMenuChrome` per submenu parent.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/🧩️component/🟦️.tsx`
