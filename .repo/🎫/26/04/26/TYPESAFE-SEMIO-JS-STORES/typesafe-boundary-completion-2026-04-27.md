# Typesafe semio boundary (completion log)

- **JS (`semio/js/index.ts`)**  
  - `KitStore.submitShellJson`: backbone VCS path returns `BackboneStatusDto` (defaults), `KitConflict[]` for listConflicts, `SetResult` for attach/detach/resolve/sync.  
  - `KitStore` + `KitStoreClient` + `FallbackKitClient` + `WasmKitStoreClient`: `attachBackbone` / `detachBackbone` / `resolveConflict` / `syncNow` → `Promise<SetResult>`; `listConflicts` → `Promise<KitConflict[]>`; `backboneStatus` uses normalized shell return.

- **React (`semio/react/index.tsx`)**  
  - Node adapters: `read()` / `readKit` / `readFile` return `string` / `undefined` (not `null`) to match `KitJsonFileAdapter` / `KitFolderAdapter`.  
  - `createStoreFromBackbone`: `remote` branch only for `createSessionKitStore`; unsupported kind throws.  
  - `useOpenKitShallows` → `Kit[]`. `useSchemaHook` indexes `schemaHooks` with a record cast.  
  - Embedded tests: `KitScope` `children` prop, `ChangeKitCommandWire` typing, `KitStoreClient` stubs (backbone + read helpers + `SetResult`), `Kit` instead of `KitShallowDto`.

- **Validation** (run 2026-04-27)  
  - `npx tsc --noEmit` in `semio/js`, `semio/react`  
  - `npm test` in `semio/js` (19) and `semio/react` (13)  
  - `cargo test` in `semio/rs` (124 + 0 failed, 1 ignored)
