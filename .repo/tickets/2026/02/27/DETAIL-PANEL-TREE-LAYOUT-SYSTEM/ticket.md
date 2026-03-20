---
goal: R26-02/RUNNING-SKETCHPAD
---

# Ticket

## Summary

Implemented TreeRow/HelperRow components in elements.tsx, migrated 100+ TreeItem+TreeContent patterns to TreeRow across 6 files, added vertical spacing (gap-y-[2px]) to tree containers, updated TreeRow to support id/onClick/onDoubleClick/actions props. TypeScript compiles (only pre-existing errors). All 13 JS tests pass.

## Changes

- elements.tsx: Added TreeRow (TreeItem+TreeContent wrapper), HelperRow (info text row), vertical spacing in tree containers
- Design.tsx: Migrated 90 TreeItem+TreeContent patterns to TreeRow
- Kit.tsx: Migrated 6 key-prop TreeItem patterns to TreeRow
- Quality.tsx: Migrated 16 id-prop TreeItem patterns to TreeRow, fixed QualityTree closing tags
- Type.tsx, Home.tsx, Docs.tsx: Migrated bare TreeItem patterns to TreeRow

## Log

- TreeRow initially only accepted children+className, extended to support id/onClick/onDoubleClick/actions
- Perl regex migration missed TreeItem tags with props (key, id), required targeted fixes
- Quality.tsx QualityTree has structural TreeItem+TreeContent that must remain (collapsible nodes)
- Design.tsx has 30 structural TreeItems (with id/defaultOpen) that correctly remain as TreeItem

## Todos

- [x] Add TreeRow, HelperRow to elements.tsx
- [x] Add vertical spacing to tree child containers
- [x] Migrate Design.tsx (90 patterns)
- [x] Migrate Kit.tsx (6 key-prop patterns)
- [x] Migrate Quality.tsx (16 id-prop patterns)
- [x] Migrate Type.tsx, Home.tsx, Docs.tsx
- [x] Fix TreeRow props to include id
- [x] Fix Quality.tsx QualityTree closing tags
- [x] Verify TypeScript compilation
- [x] Run tests (13/13 pass)

## Plan

1. Add TreeRow and HelperRow to elements.tsx Tree region
2. Add consistent spacing to tree child containers via CollapsibleContent gap
3. Migrate all files verbose TreeItem+TreeContent patterns to TreeRow
4. Fix mismatches from regex migration
5. Verify compilation and test pass
