# Layout Editor Context Menus

## Problem

`LayoutPlayApp` implemented `ArtifactEditor` without `context_menu` / `context_menu_with_request_context`, so the shell always received an empty menu.

## Solution

- Added `layout_context_menu_items` with surface-specific branches:
  - **Page tree row** (`layout-document.page.*`): `setActivePage`
  - **Link tree row** (`layout-document.link.*`): `interactionSelect` over referencing image frames
  - **Preflight issue row** (`layout-preflight.*`): `focusPreflightIssue`
  - **Empty canvas / tree** (authoring): create group (`addFrame` rect/text/image), `addPage`, `selectAll`, `paste`, disabled `clearSelection`; optional **Select** when right-click hits an unselected frame
  - **Frame selection** (1 or many): clipboard verbs, `deleteSelection` (destructive, before `clearSelection`); single selection adds a create row for the same frame kind
  - **Preview surface**: `selectAll` only (read-only)
- Added `deleteSelection` command (`delete-selection` wire) mapping to `DeleteFrame` mutations for the live `"elements"` selection.

## Tests

- `editor/🧪️tests/🔬️unit/🦀️.rs` (`context_menu_*`): empty blueprint canvas, unselected frame hit, single/multi frame selection, same-kind create for text frames, page tree row, link tree row, preflight issue row, preview read-only surface, live interaction fallback when surface selection is empty
- `🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs`: retained `deleteSelection` removes the live `"elements"` selection

## Verification

```bash
CARGO_TARGET_DIR=/tmp/semio-layout-test-target cargo test -p semio-s-artifact-layout-layout context_menu delete_selection_removes
```

Last run: 10/10 `context_menu` tests and 1/1 `delete_selection_removes` passed.
