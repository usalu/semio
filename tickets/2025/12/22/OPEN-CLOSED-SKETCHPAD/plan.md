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
