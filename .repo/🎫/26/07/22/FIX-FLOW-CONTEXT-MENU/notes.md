# Rich Context Menus — verification

## Vitest

- `@semio-tech/ui-react` ContextMenu: 4 passed
- `@semio-tech/framework-renderer-react` menu / mapContextMenuSpecs / puzzle2d selection: 5 passed

## Cargo (`--config 'build.rustc-wrapper=""'`, target `/tmp/semio-rich-menus3`)

- `flow-plugin` `context_menu*`: 2 passed (icons, destructive, hide/show preview, `previewOffJson`)
- `puzzle-plugin` `context_menu_at_selects_vortex*`: 1 passed (sparkles + zoom + delete)
- `puzzle-plugin` `hover_suggestion_updates*`: 1 passed (candidate/brush `color` + `icon`)

## Runtime `[DEBUG]` logs

Hosts log menu open (`logContextMenuOpen`) and `hoverSuggestion` / `brushPreview` color in `framework/renderer/react/index.tsx`.
