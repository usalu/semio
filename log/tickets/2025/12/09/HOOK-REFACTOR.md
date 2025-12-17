---
slug: HOOK-REFACTOR
summary: >-
  State management refactor: remove Triadic/Safe/Granular suffixes, redirect
  writes to state machine
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.895Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The codebase had inconsistent hook patterns:

- Non-triadic hooks: `useTheme()`, `useLanguage()`, `useMode()`, etc. returned single values
- Triadic hooks: `useThemeTriadic()` etc. returned `[value, setter, canSet]` tuples
- Safe hooks: `useSketchpadActorSafe()`, `useFocusSafe()` wrapped null-checks
- Commands hooks: `useDesignAppCommands()` etc. exposed command dispatch

AGENTS.md specifies that all UI component hooks should be triadic `[value, setter, canSet]`.

# Plan

1. ✅ Replace non-triadic global settings hooks with triadic implementations
2. ✅ Add deprecated aliases for backward compatibility
3. ✅ Update all usages to destructure from triadic returns
4. ✅ Run tests to verify changes don't break functionality
5. ✅ Analyze Safe hooks - keep as valid pattern for optional context access
6. ✅ Analyze Commands hooks - verify writes go through state machine

# Changes

## Phase 1: Triadic Hooks (COMPLETE)

**Sketchpad.tsx:**

- Replaced `useTheme()`, `useLanguage()`, `useMode()`, `useExpertise()`, `useLayout()` with triadic implementations
- Added `useFullscreen()` triadic hook
- Updated dependent hooks (`useTooltip()`, `useSemioTooltip()`, `useIsNavbarExpanded()`, `useIsFooterExpanded()`) to destructure
- Updated `LayoutWrapper` to use triadic destructuring pattern
- Updated `Navigation` component to use `const [mode] = useMode()`
- Created deprecated aliases: `useThemeTriadic = useTheme`, etc.

**Tutorials.tsx:**

- Updated `RecordingControls` and `RecordButton` to use `const [mode] = useMode()`

**Tests:**
All 5 playwright tests pass: Home, Kit, Type, Design, Docs (32.5s total)

## Phase 2: Safe Hooks Analysis (COMPLETE - NO CHANGES)

**Safe hooks serve a valid purpose:**

- `useSketchpadActorSafe()` returns `null` when outside `SketchpadActorContext`
- `useFocusSafe()` returns `null` when outside `FocusContext`

Used in components that can render before full context is available (e.g., `LayoutWrapper`) or conditionally based on context existence. Removing them would require React Error Boundaries which adds complexity.

**Decision:** Keep Safe hooks as valid pattern.

## Phase 3: Commands Hooks Analysis (COMPLETE - ALREADY CORRECT)

**Commands hooks already route writes through state machine:**

- `actor.send(...)` for XState events (selection, hover, panels, etc.)
- `store.execute(...)` for mutations with undo/redo (integrates with XState)

No Granular suffix exists on hook names - hooks already use `GranularHookResult<T>` type for triadic returns.
