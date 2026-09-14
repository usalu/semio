# Drag Handle Chrome Tooltip Parity

## Symptom

Drag-handle hovers show the OS native `title` tooltip (wrong font, no glass). Other chrome controls were already migrated to styled overlays before Radix `Tooltip` was dissolved.

## Root cause

`DragHandle` wraps its grip in `ChromeControlHint`, which only clones `title` / `aria-label` onto the trigger. Icon buttons use the same native `title` path today; drag handles are the most visible mismatch because grips are hover-only affordances.

The deleted `💡️Tooltip` element used `glassClass`, `SurfaceScope level="menu" fill="glass"`, and `data-slot="tooltip-content"`. `ChromeControlHint` was introduced to avoid Radix `setTrigger` ref loops.

## Fix

Reintroduce first-party glass hover tooltips inside `ChromeControlHint` via an outer `inline-flex` wrapper (no `display: contents`), `resolvePopoverPlacement`, portal, and 400ms delay — without Radix.

## Dark mode text (follow-up)

Portaling to `document.body` left tooltips outside `.semio-scope.dark`, so `text-foreground` resolved with light-theme tokens. Portal target is now `shellScope.portalLayerRef` (same as context menus / tree previews); tooltip copy uses `text-popover-foreground` + `data-level="menu"` like `PopoverContent`.

## Scope

`ChromeControlHint` consumers: `DragHandle`, `PanelTabBar`, mode-dock canvas controls, toggle dropdown rows.
