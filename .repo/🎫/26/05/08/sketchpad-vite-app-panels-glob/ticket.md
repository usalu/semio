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

---

## Follow-up: Triad field rows + stable `WriteStatus` (general architecture)

**Goal:** Apply the same UX pattern as kit rename (inline spinner + error under control, stable subscription semantics) across sketchpad detail panels and schema hooks.

### React (`semio/react/index.tsx`)

- Added `writeStatusEquivalent()` plus `USE_KIT_NAME_PENDING_STATUS` (frozen). `useKitName` now derives status with **primitive deps** (`renameKind`, `renameErrorMessage`) instead of the whole `renameSnap` object, and caches error `{ kind: "error", … }` by message so **`WriteStatus` identity stays stable** across renders when nothing changed.
- `useSchemaFieldState` wraps computed status in a ref: reuse the previous **`WriteStatus` reference** when semantically equal (pending count + `lastError` ref, error ref, idle/readonly frozen singletons).

### Sketchpad (`semio/sketchpad/index.tsx`)

- New region **`SketchpadTriadFieldRows`**: `SketchpadTriadInputRow`, `SketchpadTriadTextareaRow`, `SketchpadTriadToggleRow` — each consumes a **`HookTriad`** + **`useWriteIndicator`** (spinner + destructive error text).
- **Kit detail**: all kit metadata rows use triad components + `mapCommit` where optional strings trim to `null`.
- **Type detail (`SingleTypeSection`)**: wired to `useType*` hooks; parent id remains read-only display with shared write-indicator line (graph reference is not edited as a plain string).
- **Design detail (`DesignSectionForm`)**: name/description/icon/image/unit use schema triads + triad rows; **variant/view** stay on `runUpdateDesign` (client-only extensions, not in GraphQL `Design`).

### Verification (this pass)

- `cd semio/react && npm test` → **15 / 15 passed**.

---

## Follow-up: Strip JS-side knowledge of the on-disk kit-store bundle envelope (Rust-owned format)

**Constraint reaffirmed by the dev:** *"Everything kit state related MUST be only in semio/rs. The dev kit backbone (json file) is only interacted by rust."*

### Why this changed

A previous in-session attempt added a JS bundle codec
(`KIT_STORE_BUNDLE_SCHEMA` / `encodeKitStoreBundle` / `decodeKitStoreBundle`) and used it
in `JsonFileKitStore`, `FolderKitStore`, `importKitToDto`, and the sketchpad
`importKit` to read/write `{ schema, wip: { id, root: <KitFullDto> } }` envelopes
matching `semio/assets/semio/metabolism.new.kit.semio.json`. That violated the
layering: **JS reshaped/persisted the on-disk bundle** that is owned by `semio/rs`.

In addition, the *real* dev-json backbone (Rust `DevJsonBackboneFile` in `semio/rs/lib.rs`)
uses an entirely different on-disk shape — `kind` / `schema = "2026-05-06"` /
`connectionUri` / `persistence` / `semanticOpLog[]` — so **the JS bundle envelope was
both architecturally wrong and format-wrong.**

### Reverts in this pass

- `semio/js/index.ts`
  - **Removed** `KIT_STORE_BUNDLE_SCHEMA`, `KitStoreBundle`, `encodeKitStoreBundle`,
    `decodeKitStoreBundle`. Replaced with a `🚧` block-comment explaining JS does not
    speak the on-disk kit bundle format.
  - `JsonFileKitStore.create` / `replace`: stops parsing or writing the file. The store
    seeds an in-memory empty kit and `replace` only updates listeners — disk persistence
    is Rust's responsibility once the dev-json backbone is wired through a host adapter.
  - `FolderKitStore.create` / `replace`: same treatment (probes `readKit()` to keep the
    adapter contract alive but does not decode bytes).
  - `importKitToDto`: parses bytes strictly as the flat `KitFullDto`. Wrapped files
    must reach the host through Rust.
- `semio/sketchpad/index.tsx`
  - `importKit`: removed bundle-shape unwrapping. Parses the flat `KitFullDto` only.

### Open Rust-side gap (next ticket)

- Wire the JS file/folder adapter callbacks (`KitJsonFileAdapter`, `KitFolderAdapter`)
  through to a Rust host hook so `AttachedBackbone::DevJson` (and the future folder
  variant) reads/writes the on-disk bundle. After that, `JsonFileKitStore` /
  `FolderKitStore` can become pure projections of the rs `KitStore` snapshot, and the
  empty-seed bootstrap above can be replaced with the rs-projected DTO.
- Decide which on-disk shape is canonical (the `metabolism.new.kit.semio.json`
  wip-projection envelope vs. the `DevJsonBackboneFile` op-log envelope) and converge
  the rs serializer.

### Files (this pass)

- `semio/js/index.ts`
- `semio/sketchpad/index.tsx`
- `.repo/🎫/26/05/08/sketchpad-vite-app-panels-glob/ticket.md`

