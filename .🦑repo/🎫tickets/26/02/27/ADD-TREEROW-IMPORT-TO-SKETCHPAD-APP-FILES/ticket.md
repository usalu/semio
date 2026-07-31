---
goal: SKETCHPAD
---

# Ticket

## Summary

Added TreeRow to imports from ./elements in Type.tsx, Quality.tsx, Kit.tsx, Home.tsx, and Docs.tsx. All 5 files use TreeRow in JSX. HelperRow not added as it is unused.

## Changes

- Type.tsx L32: Added TreeRow to import from ./elements
- Quality.tsx L31-52: Added TreeRow to import from ./elements
- Kit.tsx L131-171: Added TreeRow to import from ./elements
- Home.tsx L52: Added TreeRow to import from ./elements
- Docs.tsx L44: Added TreeRow to import from ./elements

## Log

- All 5 files use <TreeRow> in JSX but none import it
- No files use <HelperRow>, so only TreeRow is added

## Todos

- [x] Check TreeRow usage in all 5 files
- [x] Check HelperRow usage (not used)
- [x] Add TreeRow import to Type.tsx
- [x] Add TreeRow import to Quality.tsx
- [x] Add TreeRow import to Kit.tsx
- [x] Add TreeRow import to Home.tsx
- [x] Add TreeRow import to Docs.tsx

## Plan

1. Add TreeRow next to TreeItem in each file's import from ./elements
