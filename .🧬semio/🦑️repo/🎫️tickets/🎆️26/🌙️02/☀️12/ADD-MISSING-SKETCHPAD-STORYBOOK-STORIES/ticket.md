---
goal: R26-02/RUNNING-SKETCHPAD
---

# Ticket

## Summary

Fixed Storybook indexing failure by removing duplicate Panel declaration collision in Resizable stories and validating with a successful Storybook build.

## Changes

### Files modified:

1. **Avatar.stories.tsx** → Added `DraggableAvatar` (Default, Selected, Hovered, Faded states), `TableAvatar` (image, fallback, selected, hovered states)
2. **Band.stories.tsx** → Added `Strip` (horizontal scrollable, non-scrollable variants)
3. **Canvas.stories.tsx** → Added `DiagramNode` (default, selected, hovered, placeholder, clickable), `DiagramSkeleton` (default skeleton)
4. **HoverCard.stories.tsx** → Added `Aside` (note, tip, caution, danger variants), `Card` (with emoji icon), `CardGrid` (4-card grid)
5. **Icons.stories.tsx** → Added `Spinner` (small, medium, large sizes), `LoadingRow` (with/without icon)
6. **Collapsible.stories.tsx** → Added `Section` (titled content sections), `Steps` (3-step sequence)
7. **Navbar.stories.tsx** → Added `ToolbarZone` with `ToolbarGroup`, `ToolbarDivider`, `ToolbarItem` (buttons + toggles + dividers)
8. **Table.stories.tsx** → Added `TableSkeleton` (default 5-row, minimal 3-row)
9. **Layout.stories.tsx** → Added `Page` (frontmatter + prose), `SidePanel` (3-tab left panel), `HudPanel` (2-tab floating panel)
10. **Breadcrumb.stories.tsx** → Added `NotFound` (full + minimal), `PageNavigation` (prev+next, next-only) with MemoryRouter wrapper
11. **Resizable.stories.tsx** → Added `Panel` (resizable panel with 2 sections)

### Components added (22):

DraggableAvatar, TableAvatar, Strip, DiagramNode, DiagramSkeleton, Aside, Card, CardGrid, Spinner, LoadingRow, Section, Steps, ToolbarZone, ToolbarGroup, ToolbarDivider, ToolbarItem, TableSkeleton, Page, SidePanel, HudPanel, NotFound, PageNavigation, Panel

### Intentionally skipped (with reasons):

- **InteractionProvider, LevelProvider, TransactionProvider** → React context providers, not visual components
- **TreeStateProvider, TreeContent, SortableTreeItems, TreeItems, FileTree** → Tree internal sub-components, fully covered by Tree.stories
- **PlaceholderDiagramNode** → Thin wrapper around DiagramNode with useLabel; DiagramNode story covers its visual appearance
- **Geometry, Scene, SceneSkeleton** → Require Three.js/React Three Fiber context; complex 3D setup beyond Storybook scope
- **PanelGroup** → Trivial flex div wrapper, already demonstrated in Layout.stories
- **Window** → Already extensively used in Canvas.stories and Layout.stories
- **LeftPanel, RightPanel, MiddlePanel, BottomPanel** → Thin wrappers around Panel with fixed resizeSide prop; Panel story covers the base component

### Storybook main.ts:

Verified globs cover all story locations. No changes needed.

## Log

## Todos

- [x] Add DraggableAvatar, TableAvatar to Avatar.stories.tsx
- [x] Add Strip to Band.stories.tsx
- [x] Add DiagramNode, DiagramSkeleton to Canvas.stories.tsx
- [x] Add Aside, Card, CardGrid to HoverCard.stories.tsx
- [x] Add Spinner, LoadingRow to Icons.stories.tsx
- [x] Add Section, Steps to Collapsible.stories.tsx
- [x] Add ToolbarZone/Group/Divider/Item to Navbar.stories.tsx
- [x] Add TableSkeleton to Table.stories.tsx
- [x] Add Page, SidePanel, HudPanel to Layout.stories.tsx
- [x] Add NotFound, PageNavigation to Breadcrumb.stories.tsx (with MemoryRouter)
- [x] Add Panel to Resizable.stories.tsx
- [x] Verify main.ts globs

## Plan

### Components to add stories for (grouped by target file):

1. **Avatar.stories.tsx**: DraggableAvatar, TableAvatar
2. **Band.stories.tsx**: Strip
3. **Canvas.stories.tsx**: DiagramNode, DiagramSkeleton
4. **HoverCard.stories.tsx**: Aside, Card, CardGrid
5. **Icons.stories.tsx**: Spinner, LoadingRow
6. **Collapsible.stories.tsx**: Section, Steps
7. **Navbar.stories.tsx**: ToolbarZone, ToolbarGroup, ToolbarDivider, ToolbarItem
8. **Table.stories.tsx**: TableSkeleton
9. **Layout.stories.tsx**: Page, SidePanel, HudPanel
10. **Breadcrumb.stories.tsx**: NotFound, PageNavigation (wrapped in MemoryRouter)
11. **Resizable.stories.tsx**: Panel

### Intentionally skipped:

- InteractionProvider, LevelProvider, TransactionProvider → Context providers, not visual
- TreeStateProvider, TreeContent, SortableTreeItems, TreeItems, FileTree → Tree internal sub-components, covered by Tree.stories
- PlaceholderDiagramNode → Wraps DiagramNode with useLabel; DiagramNode story covers visuals
- Geometry, Scene, SceneSkeleton → Need Three.js/R3F context, complex 3D setup
- PanelGroup → Trivial div wrapper, already demonstrated in Layout.stories
- Window → Already extensively in Canvas.stories and Layout.stories
- LeftPanel, RightPanel, MiddlePanel, BottomPanel → Thin wrappers around Panel with fixed resizeSide

## Reopen 2026-02-25

### Summary

Fix Storybook indexing crash in `Resizable.stories.tsx` caused by duplicate module-scope declaration of `Panel`.

### Plan

1. Reproduce or statically verify the duplicate symbol collision in the story module.
2. Resolve the declaration collision without removing story coverage.
3. Run Storybook build/indexing command to validate `/index.json` generation.
4. Close the ticket with an explicit file list and verification result.

### Todos

- [x] Reopen ticket and capture failure context from Storybook stack trace.
- [x] Patch duplicate `Panel` declaration in `Resizable.stories.tsx`.
- [x] Run Storybook build to validate indexing.
- [x] Close ticket with summary and touched files.

### Changes

- Aliased imported component `Panel` to `ResizableSidePanel` in `Resizable.stories.tsx`.
- Updated `PanelDefault` story render to use `ResizableSidePanel`.

### Log

- 2026-02-25: Applied collision fix, validation pending.
- 2026-02-25: Ran `cd compose/js && npm run build`; Storybook build completed successfully and emitted `Resizable.stories-*.js`.
