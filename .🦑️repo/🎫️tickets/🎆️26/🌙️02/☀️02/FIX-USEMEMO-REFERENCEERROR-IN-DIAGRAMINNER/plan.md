# Plan - Fix useMemo ReferenceError in DiagramInner

## Problem

The `DiagramInner` component in `js/compose/sketchpad/elements.tsx` throws a `ReferenceError: useMemo is not defined`. This is because the file uses a namespace import for React (`import * as React from "react"`) but calls `useMemo` directly on line 4854.

## Proposed Changes

- Update `js/compose/sketchpad/elements.tsx` to use `React.useMemo` instead of `useMemo`.
- Audit the rest of the file for similar issues with standard React hooks.
- Audit other files in `js/compose/sketchpad/` to ensure they use correct hook scoping based on their import style.

## Tasks

- [x] Identify the exact location of the error in `js/compose/sketchpad/elements.tsx`.
- [x] Fix the bug by prefixing `useMemo` with `React.`.
- [x] Verify the fix using `grep`.
- [x] Audit `elements.tsx` for other non-prefixed built-in hooks.
- [x] Audit neighboring files (`Design.tsx`, `Kit.tsx`, etc.) for import/hook consistency.
- [ ] Close the ticket with a summary.
