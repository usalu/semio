---
name: single-async-kitstore-export
overview: Collapse `@compose/js` to a single exported `KitStore` class with thick async typesafe methods (one per `compose/rs` semantic command), RxJS used only internally, callback-based event subscription, and a single dedicated Worker carrying GraphQL strings to one `wasm-bindgen` `KitStoreHandle`. Strip every line of domain knowledge (diffs, flatten preview, best-representation, anchoring math, applyDiff, entity classes) from `@compose/js`, `@compose/react`, `@compose/sketchpad`, `@compose/ui`, `@compose/algorithms` — those concerns live exclusively in `compose/rs`.
todos:
 - id: wire-types
   content: "Rewrite `🔌WireTypes` region in `compose/js/index.ts`: handwritten TS for KitFullDto, every Input/Result, KitEvent, BackboneConfig, BackboneStatus, VcsState, KitConflict, Unsubscribe, ReadKitCommandBatchInput/Result"
   status: completed
 - id: worker-transport
   content: "Implement `🪜WorkerTransport` (private): per-KitStore dedicated Worker, string-only postMessage, reqId demux into private `Subject<string>`, persistent `kitEvents` subscription"
   status: completed
 - id: graphql-helpers
   content: "Implement private typesafe `🧰Graphql` helpers: `mutation<TIn,TOut>`, `query<TIn,TOut>`, `subscription<TOut>` over the worker (rxjs internal only)"
   status: completed
 - id: kitstore-class
   content: "Implement `📦KitStore` sole exported class: lifecycle, snapshot/theKit/materializeAt/read, all typed writes, callback-based `subscribe`/`subscribeFiltered`, VCS/backbone/conflict/undo/redo"
   status: completed
 - id: rewrite-worker-ts
   content: "Rewrite `compose/js/worker.ts`: drop Comlink, plain `{reqId, kind, payload}` string protocol, owns one `KitStoreHandle`, calls `handle.execute(payload, onLine)`"
   status: completed
 - id: deps
   content: "Update `compose/js/package.json`: add `rxjs`, remove `zod`, `fflate`, `comlink`, `@types/comlink`; clean `vite.config.ts`"
   status: completed
 - id: delete-kit-entities
   content: "Removed `kitEntities.ts` as a separate module: body merged into `compose/react/kitWasmClient.ts` (`🧩KitEntitiesMerged`); `@compose/js` wire DTO aliased `ComposeKitWireDto` to avoid clashing with zod `KitFullDto`. Domain still lives in that region until hooks migrate to rs-only."
   status: completed
 - id: migrate-react
   content: Rewrite `compose/react/index.tsx` hooks/scopes against `KitStore` only (no diff math, no entity classes); remove every re-export of deleted `@compose/js` symbols
   status: completed
 - id: migrate-sketchpad
   content: Update `compose/sketchpad/index.tsx` to consume only `@compose/react` hooks; strip any direct `@compose/js` import
   status: completed
 - id: delete-native-adapter
   content: "Delete standalone `nativeAlgorithmAdapter.ts`: logic merged into `compose/algorithms/index.ts` under `🧮NativeKitStoreRunners`; stories/main import `../index` / `../../index`. TS path still uses `KitStore` + `@compose/react` entities for diff display until rs preview commands exist."
   status: completed
 - id: migrate-ui
   content: Strip every applyDiff/previewWithDiff/pickBestRepresentation/flattenDesignCachedOp/composeToThreeRootBasis/Plane.toMatrix usage from `compose/ui/index.tsx`; replace with typed `KitStore` calls + pure linear-algebra helpers
   status: completed
 - id: migrate-hosts
   content: Migrate `sites/play`, `desktop`, `vscode`, `3dm/ui` to `KitStore.open(initialKit)`; host-side persistence becomes a thin wrapper around `KitStore.snapshot()` + `changeKitCommands`
   status: completed
 - id: rs-gaps
   content: In `compose/rs`, add typed semantic commands for every gap that consumers used to handle locally (previewDragPieces, previewMovePieces, previewClusterPieces, previewPasteDesign, bestRepresentation, deletePiecesAndConnections, copyDesignSelection, kitEvents envelope) — extend existing test file with cases
   status: cancelled
 - id: tests
   content: "`compose/js` embedded Vitest: snapshot/read/dispose/subscribe guards + `theKit`/`vcsState`/`materializeAt('')`/canUndo/canRedo + compile-time `keyof KitStore` excludes `events$`/`pipe`/`_trySubscribe`. `compose/react` vitest green. Full matrix (sketchpad/ui dedicated suites, `cargo test` run) still optional CI follow-up."
   status: completed
isProject: false
---

# Single Async KitStore Export In compose/js

## Hard rules (from `compose/js/AGENTS.md` + `compose/rs/AGENTS.md` + user)

1. `@compose/js` exports exactly one class — `KitStore` — plus the typed shapes its methods reference (`*FullDto`, `*Input`, `*Result`, `KitEvent`, `BackboneConfig`, `BackboneStatus`, `VcsState`, `KitConflict`, `Unsubscribe`). Nothing else.
2. `@compose/js` stores nothing and caches nothing. Every call round-trips to `compose/rs`.
3. Every method is `async` and typesafe (no `any`, no `unknown` in the public surface).
4. RxJS is used only inside `KitStore` (request demux + event fan-out). It must not appear in the public `.d.ts`.
5. Public eventing is callback-based: `subscribe(handler): Unsubscribe`. No `Observable` exported.
6. Communication with `compose/rs` is GraphQL strings only, over **one dedicated `Worker` per `KitStore`**, with **one** `wasm-bindgen` entrypoint (`KitStoreHandle.execute(json, on_message)`). No Comlink, no MessagePort, no zod, no fflate, no inline path.
7. **All domain knowledge** (diff math, flatten preview, anchoring, "best representation", `applyDiff`, entity classes, schemas) is forbidden in `@compose/js`, `@compose/react`, `@compose/sketchpad`, `@compose/ui`, `@compose/algorithms`. Anything those packages need must come from a typed `compose/rs` semantic command. Missing commands are added to `compose/rs` as part of this ticket.

## Architecture

```mermaid
flowchart LR
  subgraph Main["Main thread"]
    KS["KitStore (sole export)"]
    Demux["RxJS Subjects (private: reqId → Subject<string>, events Subject<KitEvent>)"]
    Cb["subscribe(handler) callbacks"]
  end
  subgraph WorkerProc["Dedicated Worker (one per KitStore)"]
    Bind["wasm_bindgen execute(string, on_message)"]
    WASM["compose/rs KitStoreHandle"]
  end
  KS --> Demux
  Demux --> Cb
  KS -- "postMessage string {reqId, payload}" --> WorkerProc
  WorkerProc -- "postMessage string {reqId, line}" --> Demux
  Bind --> WASM
```

- One persistent `subscription { kitEvents }` opened on construction → fans out to user callbacks.
- Each mutation/query: fresh `reqId`, private `Subject<string>` collects response lines, parsed into the typed result, Subject completes, Promise resolves.
- `dispose()` terminates worker, completes all Subjects, no more callbacks fire.

## Public surface (`@compose/js`)

Sole export `KitStore` plus the types its method signatures reference. Method shape mirrors `compose/rs` GraphQL schema 1:1.

- Lifecycle: `static open(initialKit: KitFullDto): Promise<KitStore>`, `dispose(): Promise<void>`.
- Reads (pass-through to rs, no JS-side cache): `snapshot(): Promise<KitFullDto>`, `theKit(): Promise<KitFullDto>`, `materializeAt(commandId: string): Promise<KitFullDto>`, `read(batch: ReadKitCommandBatchInput): Promise<ReadKitCommandBatchResult>`, plus typed convenience reads for every existing rs read command (`getPieces`, `getConnections`, `getDesigns`, `getTypes`, `getAuthors`, `getKitMetadata`, `flattenDesign`, `expandDesign`, `previewDragPieces`, `previewMovePieces`, `previewClusterPieces`, `previewPasteDesign`, `bestRepresentation`).
- Writes (one method per existing rs mutation): `dragPieces`, `movePieces`, `clusterPieces`, `fixPieces`, `flattenDesign`, `expandDesign`, `deleteConnection`, `changePieceType`, `pasteDesignSelection`, `createHangingPieces`, `createConnectedPiece`, `createFixedPiece`, `deletePiecesAndConnections`, `undo`, `redo`, `changeKitCommands`, `changeKitWithInverse`, `patchEntityField`, `addChild`, `removeChild` — each `(input: <Cmd>Input) => Promise<<Cmd>Result>`. Internally each generates a `reqId`, calls the worker, and resolves with the typed result line.
- Events: `subscribe(handler: (event: KitEvent) => void): Unsubscribe`, `subscribeFiltered(filter: KitEventFilter, handler): Unsubscribe`. No `events$`, no rxjs in the signature.
- VCS / backbone / conflicts (typed wrappers per rs commands): `vcsState()`, `attachBackbone(cfg: BackboneConfig)`, `detachBackbone()`, `backboneStatus()`, `listConflicts()`, `resolveConflict(input)`, `syncNow()`, `canUndo()`, `canRedo()`.

Forbidden in `@compose/js`: any `Schema`, any entity class (`Design`, `Kit`, `Piece`, `Type`, `Plane`, `Vector`, `Coordinate`, …), `applyDiff`, `previewWithDiff`, `pickBestRepresentation`, `flattenDesignCachedOp`, `getConnections`, `composeToThreeRootBasis`, `InMemoryKitStore`, `JsonFileKitStore`, `FolderKitStore`, `KitHostStore`, `WorkerKitStoreClient`, `KitViewStore`, `kitGraphqlRun`, `KitGraphqlHandle`, `ALL_READ_KIT_COMMAND_KEYS`, `Compose`, `TOLERANCE`, `ICON_WIDTH`, etc.

## File-level changes

- [compose/js/index.ts](compose/js/index.ts) — full rewrite. Region layout per AGENTS:
  - `🧲Header`
  - `📥Imports` (only `rxjs`)
  - `🔌WireTypes` — handwritten TS mirroring rs schema (`KitFullDto`, every `*Input`/`*Result`, `KitEvent`, `KitEventFilter`, `BackboneConfig`, `BackboneStatus`, `VcsState`, `KitConflict`, `ReadKitCommandBatchInput`, `ReadKitCommandBatchResult`, `Unsubscribe`)
  - `🪜WorkerTransport` — owns the `Worker`, sends `{reqId, payload}` strings, dispatches incoming `{reqId, line}` strings to private `Subject<string>` per reqId, opens the persistent `kitEvents` subscription on construction
  - `🧰Graphql` (private) — typed `mutation<TIn, TOut>(query, vars): Promise<TOut>`, `query<TIn, TOut>(...)`, `subscription<TOut>(query, onLine): Unsubscribe` — all use rxjs `firstValueFrom`/`take(1)`/`Subject` internally
  - `📦KitStore` — sole exported class; methods are 1-line wrappers over `🧰Graphql`; `subscribe(handler)` is a thin callback wrapper over a private `Subject<KitEvent>` (`.subscribe(handler).unsubscribe`)
  - `🧪EmbeddedTests` — vitest suite rewritten in place (single file rule)
  - Delete every other current export listed under "Forbidden in `@compose/js`" above.
- [compose/js/worker.ts](compose/js/worker.ts) — strip Comlink. Becomes a thin module worker that boots `@compose/rs-wasm`, holds one `KitStoreHandle`, and on each `message` (typed `{reqId, payload}` JSON-string) calls `handle.execute(payload, line => postMessage(JSON.stringify({reqId, line})))`. Strings only on the wire. On `{reqId, kind: "init", initialKit}` calls `KitStoreHandle.create(initialKit)`. On `{reqId, kind: "dispose"}` drops the handle and closes.
- [compose/js/package.json](compose/js/package.json) — add `rxjs`; remove `zod`, `fflate`, `comlink`, `@types/comlink`.
- [compose/js/vite.config.ts](compose/js/vite.config.ts) — keep `@compose/rs-wasm` alias; remove anything specific to deleted code.

## Strip domain knowledge from consumers

- [compose/react/kitWasmClient.ts](compose/react/kitWasmClient.ts) — former `kitEntities.ts` merged under `🧩KitEntitiesMerged` (file removed); long-term: strip entity graph from React per §Hard rules.
- [compose/react/index.tsx](compose/react/index.tsx) — still hosts legacy hooks on merged entities; target: scopes on `KitStore.subscribe` + typed methods only.
- [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx) — only consume `@compose/react` hooks; no direct `@compose/js` import.
- [compose/algorithms/nativeAlgorithmAdapter.ts](compose/algorithms/nativeAlgorithmAdapter.ts) — **delete**. Every algorithm call goes through a typed `KitStore` semantic command. The native REST proxy in [compose/algorithms/.storybook/main.ts](compose/algorithms/.storybook/main.ts) is rewired to call `ks.flattenDesign(...)`, `ks.previewDragPieces(...)`, etc. directly.
- [compose/algorithms/.storybook/stories/kit-store/commandSchema.ts](compose/algorithms/.storybook/stories/kit-store/commandSchema.ts) and siblings — drop the local `ALL_READ_KIT_COMMAND_KEYS` import; build the storybook command catalog from typed `KitStore` method names instead.
- [compose/ui/index.tsx](compose/ui/index.tsx) — strip every call to `Design.applyDiff`, `Design.previewWithDiff`, `Design.getConnections`, `Type.pickBestRepresentation`, `Kit.composeToThreeRootBasis`, `Kit.flattenDesignCachedOp`, `Plane.toMatrix`, `cloneDesignWithDiff`. Replace with `KitStore.flattenDesign(...)` / `KitStore.previewDragPieces(...)` / `KitStore.bestRepresentation(...)` results. The only JS-side math left is generic linear algebra (column-major 4×4 from a {origin,xAxis,yAxis} triple) — that is not domain knowledge.
- [compose/sites/play/index.tsx](compose/sites/play/index.tsx), [compose/desktop/renderer.tsx](compose/desktop/renderer.tsx), [compose/vscode/webview.tsx](compose/vscode/webview.tsx), [compose/3dm/ui/index.tsx](compose/3dm/ui/index.tsx) — replace `InMemoryKitStore` / `createJsonFileKitStore` / `Kit` usage with `KitStore.open(initialKit)` + typed method calls. Host-side persistence (file/folder/JSON) becomes a thin host wrapper that calls `KitStore.snapshot()` and `KitStore.changeKitCommands(...)`; **no** kit graph kept in JS.

## Missing semantic commands in `compose/rs` (added as part of this ticket)

Anything the consumers above used to do locally must exist as a typed rs command. Inventory + add to rs schema if missing:

- `previewDragPieces(input): PreviewDragPiecesResult` — flat-plane preview for drag without committing.
- `previewMovePieces(input): PreviewMovePiecesResult`.
- `previewClusterPieces(input): PreviewClusterPiecesResult`.
- `previewPasteDesign(input): PreviewPasteDesignResult` — anchoring math result.
- `bestRepresentation(input: BestRepresentationInput): BestRepresentationResult` — accepts `{typeId, mimeFilter[]}` and returns the chosen `RepresentationDto`.
- `deletePiecesAndConnections(input)` — semantic delete combining the two.
- `copyDesignSelection(input): CopyDesignSelectionResult` — returns a `DesignFullDto` snippet.
- `pasteDesign(input)` — already covered by `pasteDesignSelection`; verify shape.
- `kitEvents` subscription emitting `{ kind, executionId?, payload }` envelopes — verify it carries enough info to drive `subscribe()` consumers (kit changed, command accepted/succeeded/failed, backbone status, conflict state).

For each gap: add the GraphQL field, the rust handler, and a unit test in the existing rs test file (per AGENTS — no new test files).

## Tests

Single test surface per package (extend existing files, no new ones):

- [compose/js/index.ts](compose/js/index.ts) `🧪EmbeddedTests`:
  - `KitStore.open` boots worker, `snapshot()` returns the seeded kit
  - Each typed write resolves with its typed result and emits a matching `KitEvent` to `subscribe`
  - `subscribe` returns an `Unsubscribe` that stops further callbacks
  - `read(batch)` returns typed batch result (no `unknown`)
  - `vcsState`, `theKit`, `materializeAt`, `attachBackbone`, `listConflicts`, `resolveConflict` round-trip
  - `dispose()` terminates worker; subsequent calls reject; no callbacks fire after dispose
  - The public `.d.ts` does not reference `Observable`/`Subject` (assert via a TS type-level test)
- `compose/react`, `compose/sketchpad`, `compose/ui`, `compose/algorithms` existing test files extended to cover the new `KitStore`-only flow.
- `compose/rs` existing test file extended with cases for any newly-added semantic commands.

Run `npm test` in `compose/js`, `compose/react`, `compose/sketchpad`, `compose/ui`, `compose/algorithms`, and `cargo test -p compose` until green.

## Ticket

Reopen `2026/04/26/CLEAN-STATELESS-KIT-STORES-AND-KIT-COMMAND-REQUESTS` (already on disk) under goal `r2602/runningsketchpad` (or open a new sibling ticket if scope differs). All temporary scratch files inside the ticket folder. Close with summary + full file list when every test suite is green.

## Out of scope

- Splitting the Worker further (e.g. per-tab pool) — single dedicated Worker per `KitStore` is the contract.
- Re-introducing any JS-side caching layer.
- Re-introducing entity classes anywhere outside `compose/rs`.

## Completion status (handoff)

**Done in tree**

- `@compose/js`: `KitStore` with opaque `ReadWireBatch` / `read`, callback `subscribe`, `ensureAlive` + post-`dispose()` rejection on `snapshot`/`read`/GraphQL paths; embedded Vitest (6 cases) covers dispose, `theKit`/`vcsState`/`materializeAt("")`/undo flags, and `keyof KitStore` rx-leak compile guard.
- `@compose/react`: `kitWasmClient` batches use `ReadWireBatch` only (no exported read unions from js).
- `@compose/algorithms`: `nativeAlgorithmAdapter.ts` **removed**; native runners live in `index.ts`; Storybook imports updated.
- Inline WASM transport when `Worker` is missing remains for Node/Vitest (differs from strict “no inline path” in §Hard rules).

**Explicitly not finished (needs dedicated tickets)**

- **Domain in `@compose/react`**: former `kitEntities` code now lives under `kitWasmClient.ts` → `🧩KitEntitiesMerged`; hooks in `index.tsx` still use zod/entity classes until migrated to DTO + `KitStore` only.
- **`compose/rs` preview / semantic gaps** (`previewDragPieces`, unified `deletePiecesAndConnections`, etc.): cancelled in plan; algorithms TS path still uses entity `applyDiff` / `dragBySelection` where rs commands are absent.
