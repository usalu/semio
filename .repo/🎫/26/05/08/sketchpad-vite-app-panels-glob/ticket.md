# Sketchpad Vite App Panels Glob + Kit Rename Architecture End-To-End

**Status:** Done in-session (repo MCP unavailable — no `ticket_open` / `ticket_close`).

## Problems addressed

1. `loadAppPanels` used a fully dynamic template `import()` — esbuild dependency scan failed.
2. Embedded vitest gate referenced `process.env` without checking for `process` — browser threw `ReferenceError`.
3. `SketchpadScopeWithKitRegistry` referenced an undefined `piecesMetadata` global in a `window` assignment.
4. `SketchpadStore.hasKitApp` checked `typeof kitAppStore?.id === "function"` but `Store.id` is a string field, leaving the UI on “Preparing kit app…”.
5. `useKitName` infinite-loop in browser fallback path: `subscribeKitName` / `subscribeRenameStatus` in `FallbackKitClient` invoked the React subscriber callback synchronously, and `getRenameStatusSnapshot` returned a fresh `{ kind: "idle" }` object on every call (`useSyncExternalStore` requires referential stability).
6. Dedicated WASM worker init silently timed out: a Blob worker can't resolve the bare specifier `@semio/rs-wasm`, so the new rename architecture fell all the way back to `FallbackKitClient` (no real rename).

## Changes

- `semio/sketchpad/index.tsx`
  - `import.meta.glob("./apps/*/panels.ts")` instead of a dynamic template import.
  - Removed `(window as any).__piecesMetadata = piecesMetadata;` (undefined identifier).
  - `SketchpadStore.hasKitApp(kitApp)` now uses `this.kitApps.has(kitApp.kit)` (the Map is keyed by kit uuid). `kitAppIds()` rebuilds `{ kit }` from `this.kitApps.keys()`.
- `semio/js/index.ts`
  - Added exported `KIT_RENAME_STATUS_IDLE` (frozen) used by `KitStore.renameStatus$` and the fallback snapshot for stable identity.
  - `FallbackKitClient.subscribeKitName` / `subscribeRenameStatus` no longer call the subscriber synchronously.
  - `FallbackKitClient.getRenameStatusSnapshot` returns the cached `KIT_RENAME_STATUS_IDLE`.
  - Embedded test gate now guards `process` existence: `typeof process !== "undefined" && !!process.env && process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1"`.
  - `WorkerStringTransport.init` now rejects fast on the worker's `error` op / `error` event (was only resolving on `ready`, then waiting 30s).
  - `KitStore.open` falls back from the dedicated Blob worker to the inline main-thread WASM transport when the worker init throws (still real rust authority, so the new rename architecture works through real `KitStoreHandle` even without a worker).
- `semio/react/index.tsx`
  - `useKitName` snapshot for kit name and rename status now use stable references (`runtime.store.getSnapshot()` instead of identity-changing `runtime.snapshot`; `KIT_RENAME_STATUS_IDLE` for the no-client branch).
  - Imported `KIT_RENAME_STATUS_IDLE` from `@semio/js`.
  - Replaced the `{ kind: "idle" } as const` test-stub literals with `KIT_RENAME_STATUS_IDLE`.

## Verification

- `cd semio/react && npm test` → 15 / 15 passed (includes “useKitName rejects empty required name via kit client” and the kit-metadata write test).
- `cd semio/js && npm test -- --testNamePattern=rename` → rename test passes (other failures pre-existing and unrelated: WASM-asset / SDL fixture tests).
- `cd semio/sketchpad && npx vite --host 127.0.0.1 --port 5210` → server boots cleanly (no dependency-scan error, no `process` reference error, no “Preparing kit app…” render-time crash).

## Files

- `semio/js/index.ts`
- `semio/react/index.tsx`
- `semio/sketchpad/index.tsx`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`
