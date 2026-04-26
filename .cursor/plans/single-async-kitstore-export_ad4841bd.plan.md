---
name: single-async-kitstore-export
overview: Collapse `semio/js/index.ts` to a single exported `KitStore` class with thick async typesafe methods, RxJS internally, and one wasm-bindgen-string GraphQL transport per dedicated Web Worker. Migrate `semio/react`, `sketchpad`, and other consumers off the removed exports.
todos:
 - id: open-ticket
   content: Open ticket `Single Async KitStore Export In semio/js` under goal r2602/runningsketchpad
   status: completed
 - id: wire-types
   content: "Write `🔌WireTypes` region: handwritten TS for KitFullDto, every GraphQL Input/Output, KitEvent, SetError/SetResult, Read/Write/Backbone/Conflict types"
   status: completed
 - id: graphql-worker
   content: Implement `🪜GraphQLWorker` (per-store dedicated Worker, string-only postMessage, request-id demultiplexing into RxJS Subjects)
   status: completed
 - id: graphql-helpers
   content: "Implement private typesafe `🧰GraphQL` helpers: `mutation<T>`, `query<T>`, `subscription<T>` over the worker"
   status: pending
 - id: kitstore-class
   content: "Implement `📦KitStore` thick API: lifecycle, snapshot/theKit/materializeAt, events$, all writes, reads, VCS/backbone/conflict, undo/redo"
   status: pending
 - id: rewrite-worker-ts
   content: "Rewrite `semio/js/worker.ts`: drop Comlink, plain string postMessage protocol calling `KitStoreHandle.create/execute/snapshot`"
   status: pending
 - id: deps
   content: "Update `semio/js/package.json`: add `rxjs`, remove `zod`/`fflate`/`comlink`"
   status: pending
 - id: migrate-react
   content: Rewrite `semio/react/index.tsx` hooks/scopes against the new `KitStore` only; remove all references to deleted `@semio/js` exports
   status: pending
 - id: migrate-sketchpad
   content: Update `semio/sketchpad/index.tsx` to consume only `semio/react` hooks; remove direct `@semio/js` usage
   status: pending
 - id: migrate-others
   content: Migrate `sites/play`, `desktop`, `vscode`, `3dm/ui`, `algorithms` (delete `nativeAlgorithmAdapter.ts`), and `ui` + storybook stories to the new API
   status: completed
 - id: tests
   content: Extend `🧪EmbeddedTests` to cover the new `KitStore` end-to-end; run `npm test` in `semio/js`, `semio/react`, `semio/sketchpad` and fix until green
   status: pending
 - id: close-ticket
   content: Close ticket with summary and full file list
   status: cancelled
isProject: false
---

## Architecture

```mermaid
flowchart LR
  subgraph Main["Main thread (semio/react, sketchpad)"]
    KS["KitStore (sole export)"]
    Bus["RxJS Subjects (responses, events)"]
  end
  subgraph WorkerProc["Dedicated Worker (per KitStore)"]
    Bind["wasm_bindgen execute(string, on_message)"]
    WASM["semio/rs KitStoreHandle"]
  end
  KS --> Bus
  KS -- "postMessage string (GraphQL request JSON + reqId)" --> WorkerProc
  WorkerProc -- "postMessage string (GraphQL response JSON + reqId)" --> Bus
  Bind --> WASM
```

- Per-store dedicated `Worker` (no Comlink, no MessagePort, no `zod`, no `fflate`). Strings only.
- One persistent GraphQL `subscription { eventStream }` opened on construction; demultiplexed by request id into RxJS `Subject`s.
- Every public method is `async` and typesafe; every argument is a typed TS shape from the new internal `🔌WireTypes` region.

## Public surface (only export)

`semio/js/index.ts` exports **only** `KitStore` plus the TS types referenced by its method signatures. Thick API, one method per existing rs GraphQL operation.

- lifecycle: `static open(initialKit: KitFullDto): Promise<KitStore>`, `dispose(): Promise<void>`
- snapshot / observe: `snapshot(): Promise<KitFullDto>`, `theKit(): Promise<KitFullDto>`, `materializeAt(id: string): Promise<KitFullDto>`, `events$: Observable<KitEvent>`
- writes (typed wrappers per `semio/rs` mutation): `dragPieces`, `movePieces`, `clusterPieces`, `fixPieces`, `flattenDesign`, `expandDesign`, `deleteConnection`, `changePieceType`, `pasteDesignSelection`, `createHangingPieces`, `createConnectedPiece`, `createFixedPiece`, `undo`, `redo`, `changeKitCommands`, `changeKitWithInverse`, `patchEntityField`, `addChild`, `removeChild` — each returns `Promise<SetResult>` (typed, no `unknown`)
- reads: `read(batch: ReadCommandBatch): Promise<ReadCommandBatchResult>` plus typed convenience reads (`getPieces`, `getConnections`, `getDesigns`, `getTypes`, `getAuthors`, `getKitMetadata`)
- VCS / backbone: `vcsState`, `attachBackbone(cfg: BackboneConfig)`, `detachBackbone`, `backboneStatus`, `listConflicts`, `resolveConflict`, `syncNow`, `canUndo`, `canRedo`

## File-level changes

- [semio/js/index.ts](semio/js/index.ts) — full rewrite. New region layout (per AGENTS):
  - `🧲Header`
  - `📥Imports` (only `rxjs`)
  - `🔌WireTypes` — handwritten TS mirroring `semio/rs` schema (`KitFullDto`, every Input/Output, `KitEvent`, `SetError`, `SetResult`, `ReadCommandBatch`, `ReadCommandBatchResult`, `BackboneConfig`, `BackboneStatusDto`, `ConflictResolution`, `KitConflict`, `VcsState`, …)
  - `🪜GraphQLWorker` — owns `Worker`, request-id → `Subject<string>` map, persistent `eventStream` subscription
  - `🧰GraphQL` — private typed helpers `mutation<T>`, `query<T>`, `subscription<T>`
  - `📦KitStore` — sole exported class
  - `🧪EmbeddedTests` — vitest suite rewritten in place (no new test files)
  - Remove every other current export: `Semio`, `Coordinate`/`Vec`/`Point`/`Vector`/`Plane`/`Camera`, `Attribute`/`Author`/`File`/`Folder`/`Benchmark`/`Quality`/`Port`/`Family`/`Prop`/`Tag`/`Concept`/`Representation`/`Connector`/`Type`/`Layer`/`Piece`/`Group`/`Side`/`Connection`/`Stat`/`Design`/`Kit` (classes, zod schemas, `*Plain`/`*Diff`/`*Shallow`/`*MetadataDto`), `InMemoryKitStore`, `JsonFileKitStore`, `FolderKitStore`, `createSessionKitStore`, `KitHostStore`, `KitBinaryStore`, `DEFAULT_KIT_SYNC`, `FallbackKitStoreClient`, `WorkerKitStoreClient`, `createKitStoreClient`, `KitWorkerApi`, `KitViewStore`, `getSemioKitViewStore`, `kitGraphqlExecuteStoreCommand`, `kitGraphqlRun`, `KitGraphqlHandle`, `readCommandTypes`, all `*Schema` / `*Id` / `DiffStatus` / `ICON_WIDTH` / `TOLERANCE`.
- [semio/js/worker.ts](semio/js/worker.ts) — strip Comlink. Becomes a thin module worker that calls `boot()` from `@semio/rs-wasm`, holds one `KitStoreHandle`, and on each `message` (typed `{ reqId, kind, payload }`) calls `handle.execute(json, onLine => postMessage({ reqId, line }))` / `handle.snapshot()` / `KitStoreHandle.create(initialKit)`. Strings only on the wire.
- [semio/js/package.json](semio/js/package.json) — add `rxjs`; remove `zod`, `fflate`, `comlink`.
- [semio/js/vite.config.ts](semio/js/vite.config.ts) — keep `@semio/rs-wasm` alias.

## Dependent migrations (greenfield, no compat)

- [semio/react/index.tsx](semio/react/index.tsx) — drop every removed import. Rebuild hooks (`useKit`, `useDesign`, `usePiece`, `useType`, `useBackboneStatus`, `useAttachBackbone`, `useDetachBackbone`, `useListConflicts`, `useResolveConflict`, `useSyncNow`, `KitScope`, `DesignScope`, `PieceScope`, `TypeScope`, …) directly on `KitStore.events$` + typed methods. Remove all `export type/{…} from "@semio/js"` re-exports of deleted symbols.
- [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx) — only consume `semio/react` hooks (its AGENTS rule); strip any direct `@semio/js` imports.
- [semio/sites/play/index.tsx](semio/sites/play/index.tsx), [semio/desktop/renderer.tsx](semio/desktop/renderer.tsx), [semio/vscode/webview.tsx](semio/vscode/webview.tsx), [semio/3dm/ui/index.tsx](semio/3dm/ui/index.tsx) — replace `InMemoryKitStore` / `createJsonFileKitStore` / `Kit` usage with `KitStore.open(initialKit)` + typed method calls (host-side persistence becomes a thin wrapper around `KitStore.snapshot()` + `KitStore.changeKitCommands(...)`; no host kit graph kept in JS).
- [semio/algorithms/nativeAlgorithmAdapter.ts](semio/algorithms/nativeAlgorithmAdapter.ts) — delete; algorithm calls go through `KitStore.flattenDesign` / `read(...)` (rs AGENTS forbids JS-side domain logic).
- [semio/algorithms/.storybook/stories/\*](semio/algorithms/.storybook/stories), [semio/ui/index.tsx](semio/ui/index.tsx), [semio/ui/.storybook/stories/\*](semio/ui/.storybook/stories) — rewrite imports against `KitStore` types only; remove uses of `Design`/`Kit`/`DesignEntity`/`KitRuntime`/`getKitPorts`/etc.

## Tests

Extend `🧪EmbeddedTests` in [semio/js/index.ts](semio/js/index.ts) (no new test files):

- `KitStore.open` boots worker, exposes typed `snapshot()`
- Each typed write round-trips and resolves with `SetResult`
- `events$` emits typed `KitEvent` for `accepted` / `succeeded` / `failed`
- `dispose()` terminates worker, completes `events$`
- Typed `read(batch)` returns typed result (no `unknown`)
- `vcsState` / `theKit` / `materializeAt` round-trip
- `attachBackbone` / `listConflicts` / `resolveConflict` typed paths

Run `npm test` in `semio/js`, `semio/react`, `semio/sketchpad` — all green before closing the ticket.

## Ticket

`ticket_open` → goal `🎯r2602🎯runningsketchpad`, title `Single Async KitStore Export In semio/js`. All temporary files inside the ticket folder. Close with summary + file list when tests pass.

## Out of scope

No `semio/rs` schema changes — we consume the existing single bindgen entrypoint `KitStoreHandle.execute(request_json, on_message)` as-is.
