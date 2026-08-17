# W2 TypeScript CORE State Migration

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Owner:** Wave 2 — framework-ts + s-plugins-ts

## Goal

Move durable UI chrome and CAD catalog registries off direct browser globals / mutable class stores toward `StoragePort` injection and immutable registry maps.

## Delivered

### Framework UI chrome (`@semio-tech/ui-react`)

- `readStoredComputeWorkerCount` / `writeStoredComputeWorkerCount` / `effectiveComputeWorkerCount` now require `StoragePort` (same contract as other `UI_CHROME_*` helpers).
- `readStoredIntroductionSeen` / `writeStoredIntroductionSeen` now require `(storage, appId)`.
- Vitest layout cases use `createBrowserStoragePort()` only (no direct `localStorage` in tests).
- **OS shell:** `ShellHost` introduction auto-start/dismiss passes `scope.storage` (required for the new signatures).

### Flow compute (`✏️s/…/🧮️compute`)

- `initFlowThreadPool(init, storage?, requested?)` accepts `StoragePort` (defaults to `createBrowserStoragePort()` for standalone callers).

### CAD core (`@semio-tech/cad-js/core`)

- `ActionRegistry` / `InteractionRegistry` are `ReadonlyMap` types with pure factories:
  - `modelDefinitionActionRegistry()`, `registerActionDef`, `listActionDefs`, `runRegisteredAction`
  - `modelDefinitionInteractionRegistry()`, `registerInteractionSpec`, `listInteractionSpecs`
- Model-definition asset indexes documented as derived caches (cleared on `registerModelDefinitionAssets`).
- `COMPILED_INTERACTION_BY_ID` documented as derived compile cache.

### CAD query (`@semio-tech/cad-js/query`)

- Construct runner uses `runRegisteredAction` + `modelDefinitionActionRegistry()`.

## Inventory

See `🧪w2-typescript-inventory.md` (rg snapshot of `localStorage` / module-level `let` in scope).

## Verification

| Gate | Result |
|---|---|
| `bun nx run @semio-tech/ui-react:test-quick` | **fail** — pre-existing duplicate export `Cursor` in `📦️index.tsx:7937` (transform error; not introduced by this wave) |
| `bun test ./🟦️component.ts` (cad core) | **fail** — `ENOENT` `@semio-tech/kernel-3d-js` in workspace link |
| `bun test ./🟦️component.ts` (flow compute) | **fail** — same `Cursor` duplicate export via `@semio-tech/ui-react` import chain |

Logs: `🧪w2-typescript-ui-react-test.err`, `🧪w2-typescript-cad-core-test.err`, `🧪w2-typescript-flow-compute-test.err`.

## Remaining violations (in scope)

| Area | Violation |
|---|---|
| `framework-core` `createBrowserStoragePort` | Sole browser `localStorage` adapter (intended until OS chrome document projection lands) |
| `ui-react` chrome keys | `UI_CHROME_*` / dock layout stores still persist via `StoragePort` → browser, not OS `DocumentStore` |
| `ui/styling` | Module-level `_activeUiTheme`, `_builtinThemesCache`, per-root CSS var maps; `setActiveUiTheme` page-global |
| `ShellScope` / `UiDriver` / `Tree` | Ephemeral session globals (drag sessions, selection mode store, driver provider) |
| `vite-elements-assets` boot scripts | Inline `localStorage` in playground HTML boot snippets |
| `AttributeStore` / `Model` | Document metadata still in TS `Map` on `Model.metadata` (projection to OS document ops not done) |
| `InteractionRuntime` / `StatelyStateEngine` | Live interaction session state in TS (draft lane / host session pending) |
| `brepjs` | Kernel implementation state inside compute session |
| CAD runtime/renderer caches | `shippedModelDefinitionAssetsCache`, `spatialSceneColorCache`, etc. |

## Follow-ups

- OS chrome prefs: bind `ui.chrome.*` through OS config document / `ChromePrefsState` Rust path instead of `StoragePort` → `localStorage`.
- CAD: move `AttributeStore` rows into document projection; interaction session into OS draft lane.
- Fix duplicate `Cursor` export blocking vitest for `ui-react`.
