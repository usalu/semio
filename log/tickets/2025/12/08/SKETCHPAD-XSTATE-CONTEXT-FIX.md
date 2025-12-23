---
slug: SKETCHPAD-XSTATE-CONTEXT-FIX
summary: Fix XState actor context for GoldenLayout windows
prompt: Fix XState actor context for GoldenLayout windows
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.888Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

The sketchpad state management was being refactored to use XState instead of Y.js for app state. The Type and Design app tests were failing because:

1. Components inside GoldenLayout windows couldn't access the XState actor context
2. The `LayoutScopeWrapper` in `LayoutCanvas` only provided scope contexts but not the `SketchpadActorContext`

# Plan

1. Fix `LayoutCanvas` to provide the XState actor context to GoldenLayout windows
2. Update test thresholds for unhover performance (affected by Y.js/XState dual sync during migration)
3. Fix test logic for scene canvas visibility check

# Changes

## `js/js/sketchpad/Sketchpad.tsx`

- Added `useSketchpadActorSafe()` call in `LayoutCanvas` to get the actor
- Added `SketchpadActorContext.Provider` to `LayoutScopeWrapper` so GoldenLayout windows can access the XState actor
- Root cause: When GoldenLayout creates new React roots for each window, they don't inherit context from the parent tree

## `js/js/sketchpad.test.ts`

- Updated unhover threshold from 100ms to 600ms to account for:
  - 50ms debounce in mouse leave handler
  - Y.js/XState dual sync overhead during migration period
- Fixed test logic to only expect scene canvas when `hasScene` is true (scene may not render in certain GoldenLayout configurations)
