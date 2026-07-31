# Detail Panel Tree Layout System

## Goal

R26-02/RUNNING-SKETCHPAD

## Status

CLOSED

## Plan

1. [x] Add TreeRow and HelperRow convenience components to elements.tsx
2. [x] Add vertical spacing (gap-y-[2px]) to TreeSection, SortableTreeItem, TreeItem children containers
3. [x] Migrate Design.tsx TreeItem+TreeContent patterns to TreeRow (90 patterns)
4. [x] Migrate Kit.tsx TreeItem+TreeContent patterns to TreeRow (6 patterns with key props)
5. [x] Migrate Quality.tsx TreeItem+TreeContent patterns to TreeRow (16 patterns with id props)
6. [x] Migrate Type.tsx, Home.tsx, Docs.tsx TreeItem+TreeContent patterns to TreeRow
7. [x] Fix TreeRow component to accept id, onClick, onDoubleClick, actions props
8. [x] Fix Quality.tsx collapsible TreeItem+TreeContent closing tags (QualityTree)
9. [x] Verify TypeScript compilation (only pre-existing errors remain)
10. [x] Run tests (13/13 pass)

## Changes

### elements.tsx

- Added `TreeRow` component: convenience wrapper combining `TreeItem` + `TreeContent`
- Added `HelperRow` component: informational text row with muted styling
- Added `flex flex-col gap-y-[2px]` vertical spacing wrappers in:
  - TreeSection's CollapsibleContent children
  - SortableTreeItem's expandable children
  - TreeItem's expandable children
- TreeRow accepts: children, className, id, onClick, onDoubleClick, actions

### Design.tsx

- Migrated 90 `<TreeItem><TreeContent>...</TreeContent></TreeItem>` patterns to `<TreeRow>...</TreeRow>`
- Added TreeRow and HelperRow imports

### Kit.tsx

- Migrated 6 `<TreeItem key={...}><TreeContent>...</TreeContent></TreeItem>` patterns to `<TreeRow key={...}>...</TreeRow>`
- Added TreeRow import

### Quality.tsx

- Migrated 16 `<TreeItem id="..."><TreeContent>...</TreeContent></TreeItem>` patterns to `<TreeRow id="...">...</TreeRow>`
- Preserved structural TreeItem+TreeContent pairs in QualityTree component
- Fixed closing tag mismatch in QualityTree collapsible node

### Type.tsx, Home.tsx, Docs.tsx

- Migrated all bare `<TreeItem><TreeContent>...</TreeContent></TreeItem>` patterns to `<TreeRow>...</TreeRow>`
- Added TreeRow imports

## Summary

Implemented TreeRow and HelperRow convenience components in elements.tsx, migrated 100+ TreeItem+TreeContent patterns to TreeRow across 6 files (Design.tsx, Kit.tsx, Quality.tsx, Type.tsx, Home.tsx, Docs.tsx), added consistent vertical spacing to tree containers, and updated TreeRow to support id/onClick/onDoubleClick/actions props. All TypeScript compilation passes (only 3 pre-existing errors) and all 13 tests pass.
