---
date: "2025-12-08T15:43:43.165Z"
slug: STATE-MACHINE-REFACTOR
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Refactor sketchpad state machine for proper state transitions
model: claude-sonnet-4.5
---

# Previously

The codebase has a dual state management system:

1. **Y.js stores** - For persistence and synchronization of kit data
2. **XState machine** - For app state (selection, hover, panel visibility, etc.)

Current issues:

- XState machine only uses reflexive transitions (all events just trigger actions, no state transitions)
- Some hooks don't follow the triadic pattern `[state, setState, canSetState]`
- App stores still use Y.js for app state which should be pure XState

Triadic hook pattern already exists in:

- `useTypeAppSelection(): GranularHookResult<TypeAppSelection>`
- `usePieceCenterU(): GranularHookResult<number>`
- `useConnectionGap(): GranularHookResult<number>`

All playwright tests pass (5/5).

# Plan

1. [x] Analyze current state management structure
2. [x] Ensure all sketchpad hooks follow triadic pattern
3. [ ] Verify XState machine handles all app state correctly
4. [ ] Remove Y.js usage from app state (keep only for kit data sync)
5. [x] Run tests to verify nothing breaks

# Changes

## Converted sketchpad hooks to triadic pattern

Updated the following hooks in `Sketchpad.tsx` to return `GranularHookResult<T>` (triadic tuple `[value, setter, canSet]`):

- `useTheme()` → `GranularHookResult<Theme>`
- `useLanguage()` → `GranularHookResult<string>`
- `useLayout()` → `GranularHookResult<Layout>`
- `useMode()` → `GranularHookResult<Mode>`
- `useExpertise()` → `GranularHookResult<Expertise>`
- `useIsFullscreen()` → `GranularHookResult<boolean>`

Each triadic hook:

1. Gets the value from Y.js store via `useSketchpad`
2. Uses `useOrigin()` to get the current origin from context
3. Creates a setter that sends events to both XState actor and Y.js store with the origin
4. Returns `[value, setter, canSet]` as a readonly tuple

## Added OriginProvider/Context pattern

Created in `Sketchpad.tsx`:

- `OriginContext` - React context for tracking command origin
- `OriginProvider` - Wraps children with specified origin id
- `useOrigin()` - Returns current origin from context (defaults to "semio.sketchpad.unknown")
- `useOriginSafe()` - Returns origin or null if not in provider

Components with an `id` should wrap their children in `<OriginProvider id={...}>` to provide origin for triadic hooks.

## Updated all consumers to use destructuring and OriginProvider

Updated all files using these hooks:

- `Home.tsx` - Extracted `ThemeToggle`, `LanguageSelect`, `LayoutToggle`, `ExpertiseToggle`, `ModeToggle` components wrapped with `OriginProvider`
- `Design.tsx` - Added `OriginProvider` import and settings helper components
- `Type.tsx` - Added `OriginProvider` import and settings helper components
- `Kit.tsx` - Added `OriginProvider` import
- `Sketchpad.tsx` - `LayoutWrapper` destructures triadic hooks

All 5 playwright tests pass after the refactor.
