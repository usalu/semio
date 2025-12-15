---
date:
  created: '2025-12-12T20:23:25.478Z'
  updated: '2025-12-12T20:23:25.478Z'
slug: APP-PLUGIN-REFACTOR
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Refactor sketchpad apps to use open/closed plugin architecture with triadic
  hooks
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

The sketchpad architecture has grown organically with several issues:

- `Sketchpad.tsx` is a monolithic file (~17k lines) containing all app-specific XState events, actions, and guards
- Apps (Home, Kit, Type, Design, Quality, Docs) have mixed patterns for state management
- `Kit.tsx` still uses `controller.execute()` pattern instead of XState events
- `Type.tsx` correctly uses triadic hooks with `actor.send()` and `snapshot.can()`
- Store factory registration exists in `shared.ts` but no plugin mechanism for machine contributions
- Adding a new app requires editing Sketchpad.tsx, violating open/closed principle

# Plan

## Phase 0: Add AppPlugin Registry

- Add `AppPlugin` interface to `shared.ts`
- Add `registerAppPlugin()` and `getAppPlugins()` functions
- Plugin provides: events, actions, guards, selectors for machine composition

## Phase 1: Fix Kit.tsx Violations

- Replace `controller.execute()` calls with `actor.send()` events
- Convert hooks to triadic pattern with `snapshot.can()`

## Phase 2-5: Migrate Each App

- Extract machine contributions (events, actions, guards) into plugin
- Register plugin on module import
- Ensure triadic hook pattern throughout

## Phase 6: Clean Sketchpad.tsx

- Remove hardcoded app events/actions/guards
- Compose machine from registered plugins
- Keep only platform-level concerns

## Phase 7: Create apps/index.ts

- Single import point for all app modules
- Side-effect imports register plugins + stores

# Changes

## Phase 0: AppPlugin Registry (shared.ts)

- Added `AppMachineContribution` interface for events, actions, guards, selectors
- Added `AppPlugin` interface with id, namespace, machine contributions
- Added `registerAppPlugin()`, `getAppPlugins()`, `getAppPlugin()`, `hasAppPlugin()` functions
- Added `composePluginContributions()` to merge all plugin contributions

## Phase 1: Kit.tsx Refactoring

- Refactored `useKitAppSelection` to use XState `actor.send()` instead of `controller.execute()`
- Refactored `useKitAppTransaction` to use XState events (KIT.TRANSACTION.\*)
- Added `conditionalHookResult` and `readonlyHookResult` imports
- Added Kit app plugin registration (`kitAppPlugin`)
- TODO: Add KIT.SET_FULLSCREEN event to XState machine (useKitAppFullscreen still uses controller)
- Note: Pre-existing lint errors in Kit.tsx remain (not related to this refactor)

## Phase 2: Type.tsx Plugin Registration

- Added `typeAppPlugin` with namespace "TYPE"
- Plugin provides default state factory for TypeAppState

## Phase 3: Design.tsx Plugin Registration

- Added `designAppPlugin` with namespace "DESIGN"
- Plugin provides default state factory for DesignAppState
- Plugin registers Design store factory via `registerStores` callback

## Phase 4: Home.tsx Plugin Registration

- Added `homeAppPlugin` with namespace "HOME"
- Plugin provides default state factory for HomeState

## Phase 5: Docs.tsx and Quality.tsx Plugin Registration

- Added `docsAppPlugin` with namespace "DOCS" to Docs.tsx
- Added `qualityAppPlugin` with namespace "QUALITY" to Quality.tsx
- Both plugins provide default state factories

## Phase 6: Sketchpad.tsx Integration (Partial)

- Plugin infrastructure in place, apps register on import
- Full machine composition from plugins deferred for incremental migration

## Phase 7: apps/index.ts Created

- Single import point for all app modules
- Re-exports plugin utilities for external use
- Importing triggers all plugin registrations

## Documentation

- Added "Sketchpad App Plugin Architecture" section to AGENTS.md
- Documents plugin structure, registration, triadic hook pattern, file layout
