---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed all TS compilation errors across semio/js (4→0), semio/play (14→0), semio/sketchpad (19→0) with minimal type-level fixes: arrow function wrappers in Design.tsx, ReactNode cast in elements.tsx, any[] cast in sketchpad.test.ts, @semio/js path mapping in tsconfig.json, vite/client types in play+sketchpad tsconfigs. 14/14 unit tests pass, both dev servers return 200.
## Changes
- Design.tsx: Wrapped FC components in arrow functions for `content` field type compatibility
- elements.tsx: Cast `activeTab.content` as `React.ReactNode` in SidePanel fallback branch
- sketchpad.test.ts: Added `as any[]` cast for `Object.values(typeApps)` activeTool access
- tsconfig.json (semio/js): Added `@semio/js` self-referencing path mapping for dependent projects
- tsconfig.json (semio/play): Added `vite/client` types for Vite-specific globals
- tsconfig.json (semio/sketchpad): Added `vite/client` types for Vite-specific globals

## Log
- semio/js: 4 errors → 0 errors
- semio/play: 2 (+12 inherited) errors → 0 errors
- semio/sketchpad: 19 errors → 0 errors
- Sketchpad dev server (port 5173): HTTP 200 ✓
- Play dev server (port 4000): HTTP 200 ✓
- Unit tests: 14/14 pass ✓

## Todos
- [x] Fix Design.tsx FC type errors
- [x] Fix elements.tsx i18n type error
- [x] Fix sketchpad.test.ts activeTool error
- [x] Fix play/sketchpad module resolution and Vite types
- [x] Verify TS compilation passes (0 errors across all 3 projects)
- [x] Verify dev servers run correctly (both 200)
- [x] Run tests to verify nothing broken (14/14 pass)

## Plan
Minimal type-level fixes only: casts, arrow function wrappers, and tsconfig path/type additions. No functional changes.
