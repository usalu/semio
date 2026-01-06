---
slug: OPEN-CLOSED-SKETCHPAD
prompt: >-
  Sketchpad.tsx, elements.tsx and APP.tsx (Home.tsx, Kit.tsx, Design.tsx,
  Type.tsx, Quality.tsx, Docs.tsx, Feedback.tsx) should be refactored to follow
  the open/closed principle. All app specific logic should be part of the
  APP.tsx files. elements.tsx should not import anything from sketchpad or any
  app. There should be no design, type, etc logic part of Sketchpad.tsx file.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-22T20:43:22.250Z"
iterations:
  - prompt: Refactor to follow open/closed principle
    model: claude-sonnet-4-20250514
    date: "2025-12-22T20:51:00.000Z"
    files:
      updated:
        - js/js/sketchpad/shared.ts
        - js/js/sketchpad/Sketchpad.tsx
        - js/js/sketchpad/Design.tsx
        - js/js/sketchpad/Kit.tsx
        - js/js/sketchpad/Docs.tsx
        - js/js/sketchpad/elements.tsx
---

# Previously

Sketchpad.tsx contained app-specific caches and hook getters:

- `designAppModuleCache`, `kitAppModuleCache` - module caches for lazy loading
- `getDesignAppHooks()`, `getKitAppHooks()` - functions returning app-specific hooks

elements.tsx contained:

- `getDocsRegistry()` with `require("../apps/docs/App")` - direct import from docs app

These violated the open/closed principle: adding/removing an app required modifying Sketchpad.tsx.

# Plan

1. Add registry system to shared.ts for app hooks and registries
2. Remove app-specific caches from Sketchpad.tsx
3. Each APP.tsx registers its own hooks on module load
4. Move domain-specific SectionTree from elements.tsx to Docs.tsx
5. Docs.tsx registers docsRegistry via the registry system

# Changes

## shared.ts

- Added `DesignAppHooks` and `KitAppHooks` interfaces
- Added `registerDesignAppHooks()`, `getDesignAppHooks()` functions
- Added `registerKitAppHooks()`, `getKitAppHooks()` functions
- Added `DocsRegistryInterface` and `registerDocsRegistry()`, `getDocsRegistry()` functions

## Sketchpad.tsx

- Removed `designAppModuleCache`, `kitAppModuleCache` variables
- Removed inline `getDesignAppHooks()` and `getKitAppHooks()` implementations
- Now imports `getDesignAppHooks`, `getKitAppHooks` from shared.ts

## Design.tsx

- Added `registerDesignAppHooks` import
- Registers hooks via `registerDesignAppHooks()` on module load

## Kit.tsx

- Added `registerKitAppHooks` import
- Registers hooks via `registerKitAppHooks()` on module load

## Docs.tsx

- Added `registerDocsRegistry` import and `FileTree` import
- Moved `SectionTree` component from elements.tsx (domain-specific)
- Registers docsRegistry via `registerDocsRegistry()` on module load

## elements.tsx

- Removed `SectionTree` component and `getDocsRegistry()` function
- No longer imports from any app module
