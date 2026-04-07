# Semio Refactor Plan: Replaceable Kit Store Architecture

## Executive summary

This refactor must **remove all storage-engine knowledge from `semio/sketchpad` and public core APIs** so the kit backend can be swapped without changing Sketchpad. The current code still mixes UI state, kit editing logic, synchronization, persistence, and storage-engine primitives in the same bundle. The end state is:

- `semio/sketchpad` becomes a **storage-agnostic editor UI**.
- `semio/js` becomes **domain/core only**.
- `semio/studio` becomes the **provider bundle** that contains the collaborative kit provider and the other store implementations.
- a **JSON file store** and a **folder/sqlite store** are added.
- the VS Code integration becomes a **separate extension in a separate repository**, not inside `repo`.
- the string and dependency for the current collaboration engine disappear from `semio/sketchpad` after the refactor.

This is a structural refactor, not a rename exercise. The abstraction must express **what a kit store does** rather than exposing **how one specific engine stores data internally**.

---

## Non-negotiable constraints

1. **No engine-specific imports in `semio/sketchpad`.**
2. **No engine-specific imports in public `semio/js` APIs.**
3. **No engine-specific primitives as abstraction**:
   - no map/array/doc abstractions copied from a collaboration engine
   - no wrapper types that are just renamed engine types
4. **All kit providers live in `semio/studio`** and are exported from `studio.ts`.
5. `Sketchpad` receives the backend via a root prop, e.g.:
   - `<Sketchpad kitStore={kitStore} />`
6. Add these store implementations:
   - **CollaborativeKitStore** in `semio/studio`
   - **JsonFileKitStore** in `semio/studio`
   - **FolderKitStore** in `semio/studio`
7. The **VS Code bundle is a separate extension** and **must not live in `repo`**.
8. After the refactor, the collaboration engine name must **not appear anywhere in `semio/sketchpad`**.

---

## Why the current architecture must change

The current `Sketchpad.tsx` shows that the editor bundle still owns storage-engine concerns directly:

- it imports the collaboration engine and IndexedDB persistence directly
- it defines stores in terms of engine primitives such as map/array/doc
- `SketchpadStore` owns the document, persistence, and provider wiring
- undo/redo and state observation are tightly coupled to engine-backed containers
- file providers and non-engine code already exist, but only as partial escape hatches rather than as the main architecture

This means changing the kit store today requires changing Sketchpad internals instead of just swapping a provider.

---

## Goals

### Primary goals

- Make the kit store completely replaceable.
- Decouple editor behavior from synchronization technology.
- Move all provider implementations into `semio/studio`.
- Keep the domain model centered on `Kit`, `KitDiff`, import/export, commands, and synchronization state.
- Support three backends with the same public capability surface:
  - collaborative synchronized store
  - single JSON file store
  - folder-backed store using `.semio/` and sqlite

### Secondary goals

- keep import/export and domain operations reusable across web, desktop, and VS Code
- preserve undo/redo semantics
- preserve current editing features while changing storage underneath
- make integration tests backend-agnostic

### Anti-goals

- renaming engine types and pretending they are abstractions
- leaving “temporary” engine types in shared interfaces
- letting `semio/sketchpad` choose storage strategy internally
- keeping VS Code code inside the existing repository because it is convenient

---

## Target package topology

```text
semio/js
  domain model only
  diffs, import/export, commands, selectors, serializers
  storage-agnostic interfaces and adapters

semio/sketchpad
  React editor shell
  UI app state
  editor commands
  storage-agnostic hooks bound to KitStore interfaces
  no provider implementations
  no storage-engine imports

semio/studio
  provider bundle
  exports from studio.ts:
    createCollaborativeKitStore(...)
    createJsonFileKitStore(...)
    createFolderKitStore(...)
    createBlobStore(...)
    provider utilities

semio-vscode (separate repository)
  VS Code extension host
  webview host for Sketchpad
  opens .json kit files
  uses JsonFileKitStore
  writes back to the file system
```

---

## Architectural direction

### 1. Separate domain, editor, and provider responsibilities

#### `semio/js`

Owns only domain and storage-agnostic contracts:

- `Kit`, `KitDiff`, import/export, serialization
- pure selectors and mutators
- validation
- command contracts
- store interfaces
- synchronization status types

It must not own:

- collaboration documents
- provider clients
- IndexedDB persistence
- file system adapters
- VS Code APIs

#### `semio/sketchpad`

Owns editor behavior only:

- rendering
- command dispatch
- navigation
- layout
- selection state
- tool state
- hooks that consume `KitStore`

It must not know:

- whether the kit is backed by collaborative sync, JSON, sqlite, files, or memory
- whether assets are blobs, files, remote URLs, or embedded payloads

#### `semio/studio`

Owns every kit provider implementation:

- collaborative provider
- json file provider
- folder/sqlite provider
- provider composition and bootstrapping
- persistence bridges
- transport adapters

---

## The abstraction to introduce

The abstraction must express capabilities, not internal data structures.

### Core interfaces

```ts
export type KitStoreStatus = "idle" | "loading" | "ready" | "saving" | "syncing" | "offline" | "error";

export type KitSyncState = {
 status: KitStoreStatus;
 dirty: boolean;
 readonly: boolean;
 lastSyncedAt?: string;
 error?: Error;
};

export type KitStoreSnapshot = {
 kit: Kit;
 sync: KitSyncState;
};

export interface KitStore {
 getSnapshot(): KitStoreSnapshot;
 subscribe(listener: () => void): () => void;

 transact<T>(label: string, run: () => T): T;
 apply(diff: KitDiff, meta?: { origin?: string }): void;
 replace(next: Kit, meta?: { origin?: string }): void;

 save(): Promise<void>;
 reload(): Promise<void>;
 dispose(): Promise<void> | void;
}
```

### Optional capability interfaces

```ts
export interface UndoableKitStore extends KitStore {
 canUndo(): boolean;
 canRedo(): boolean;
 undo(): void;
 redo(): void;
}

export interface BlobAssetStore {
 put(file: Blob, meta?: { path?: string; mimeType?: string }): Promise<{ id: string; url?: string }>;
 get(id: string): Promise<Blob>;
 remove(id: string): Promise<void>;
}

export interface ObservablePathStore {
 subscribePath(path: readonly string[], listener: () => void): () => void;
}
```

### Key rule

`semio/sketchpad` may depend on `KitStore`, `UndoableKitStore`, and other storage-agnostic capability interfaces. It may **not** depend on any provider implementation or provider primitive.

---

## Refactor strategy

Do this as a staged migration, not a big-bang rewrite.

# Phase 0 — freeze the target and create guardrails

## Outcomes

- lock the architecture before touching implementation
- stop new storage-engine leaks from entering Sketchpad

## Tasks

- create an ADR: **Replaceable Kit Store Architecture**
- add lint rules / forbidden imports:
  - forbid the collaboration engine package in `semio/sketchpad`
  - forbid provider packages in `semio/js`
  - forbid importing `semio/studio` from `semio/js`
- add CI grep checks:
  - the collaboration engine name must not appear in `semio/sketchpad/**`
- define public interfaces in `semio/js`
- identify every current import/export surface that leaks provider details

## Exit criteria

- architecture doc approved
- forbidden import rules active
- target interfaces committed before migration work starts

---

# Phase 1 — extract storage-agnostic contracts into `semio/js`

## Outcomes

- `semio/js` becomes the home of the portable contracts
- editor and provider can depend on the same neutral interfaces

## Tasks

### 1.1 Create new storage-neutral modules

Suggested modules:

```text
semio/js/store/KitStore.ts
semio/js/store/UndoStore.ts
semio/js/store/AssetStore.ts
semio/js/store/SyncState.ts
semio/js/store/StoreEvents.ts
semio/js/store/transactions.ts
```

### 1.2 Move domain logic out of provider-backed store classes

Move pure logic into `semio/js`:

- kit diff application
- inverse diff computation
- import/export
- merge policies
- selectors/finders
- validation
- serialization

### 1.3 Define transaction semantics independently of backend

Create a neutral transaction model:

- transaction label
- user/system origin
- undo grouping behavior
- optimistic vs persisted state

### 1.4 Remove provider primitives from shared types

Delete or replace types that expose internal provider details from shared/public APIs.

Examples to remove from public editor-facing contracts:

- document types
- map/array aliases
- path types that assume provider-specific containers
- observer types that require a specific storage engine

## Exit criteria

- `semio/js` exports only neutral store contracts and domain logic
- no provider implementation exists in `semio/js`
- a dummy in-memory store can satisfy the new interfaces

---

# Phase 2 — decontaminate `semio/sketchpad`

## Outcomes

- Sketchpad becomes backend-agnostic
- engine-specific state holders disappear from the editor bundle

## Tasks

### 2.1 Change the root API

Current direction:

```tsx
<Sketchpad kitStore={kitStore} />
```

Optional extension:

```tsx
<Sketchpad kitStore={kitStore} assetStore={assetStore} services={services} />
```

### 2.2 Split `SketchpadStore`

Break the current giant store into:

- `EditorShellStore`
  - window/layout/navigation/panels
- `EditorSessionStore`
  - command routing, current kit binding, selection
- `KitBinding`
  - adapter layer that maps `KitStore` snapshots into editor state

`SketchpadStore` should stop owning backend documents and persistence.

### 2.3 Replace provider-backed store inheritance

Current store classes are heavily tied to engine-backed base classes. Replace inheritance with one of these approaches:

- composition over inheritance, or
- adapter + reducer pattern

Recommended:

- plain editor reducers for UI state
- `KitStore` subscription for domain state
- `CommandExecutor` that reads from `KitStore` and writes diffs back via `kitStore.apply(...)`

### 2.4 Rewrite observers around neutral subscriptions

Replace logic like field/path observers that assume a specific nested container model with one of:

- snapshot subscription + selectors
- memoized selectors + equality checks
- optional `subscribePath` capability implemented only by stores that can optimize it

Important: optimized subscriptions are allowed, but the editor must not require a provider’s container primitives.

### 2.5 Move any remaining provider utilities out of Sketchpad

Anything in Sketchpad that creates, persists, or manages a concrete backend must move to `semio/studio`.

## Exit criteria

- no collaboration engine import in `semio/sketchpad`
- no persistence import in `semio/sketchpad`
- `Sketchpad` boots from injected `kitStore`
- the editor runs against a fake in-memory `KitStore`

---

# Phase 3 — build `semio/studio`

## Outcomes

- all kit providers live in one provider bundle
- the editor consumes them by injection only

## Public surface

`studio.ts` should export only provider factories and provider-facing helper types.

Suggested API:

```ts
export { createCollaborativeKitStore, createJsonFileKitStore, createFolderKitStore, createMemoryAssetStore, createFileSystemAssetStore, createRemoteAssetStore } from "./providers";
```

## Tasks

### 3.1 Create provider package structure

```text
semio/studio
  studio.ts
  providers/
    collaborative/
    json-file/
    folder/
  assets/
  bridges/
  tests/
```

### 3.2 Implement `CollaborativeKitStore`

Responsibilities:

- current collaborative backend integration
- remote sync / local persistence
- translating provider events into neutral `KitStore` updates
- translating neutral operations into provider mutations

Strict rule:

- provider internals stay inside `semio/studio/providers/collaborative/**`
- no provider type leaks through the factory return type

### 3.3 Implement `JsonFileKitStore`

Responsibilities:

- load one kit from one JSON file
- keep in-memory working snapshot
- serialize full kit back to file on save or autosave
- support file-changed-on-disk reload logic where host supports it

Use cases:

- local single-file workflows
- VS Code json editing workflow
- tests and fixtures

### 3.4 Implement `FolderKitStore`

Responsibilities:

- synchronize a kit to a folder
- keep metadata/state in `.semio/`
- use sqlite in `.semio/` in the same model as the python engine
- keep large assets as real files rather than stuffing them into sqlite unless explicitly required

Suggested folder layout:

```text
my-kit/
  kit.json                # optional materialized export or manifest
  assets/
  .semio/
    kit.db                # sqlite
    metadata.json
    locks/
```

Suggested storage split:

- sqlite: normalized kit records, indices, metadata, sync bookkeeping
- filesystem: binary assets
- optional `kit.json`: import/export snapshot for interoperability

### 3.5 Keep provider factory APIs host-friendly

Examples:

```ts
createCollaborativeKitStore(config): Promise<KitStore>
createJsonFileKitStore(config): Promise<KitStore>
createFolderKitStore(config): Promise<KitStore>
```

Host-specific details stay in config adapters, not in the editor.

## Exit criteria

- `semio/studio` is the only bundle with provider implementations
- all providers return the same neutral interface
- `studio.ts` is the single public export point for kit providers

---

# Phase 4 — migrate current collaborative backend into `semio/studio`

## Outcomes

- current collaborative support still works
- it no longer contaminates Sketchpad or `semio/js`

## Tasks

### 4.1 Extract provider bridge code

Move all code that directly touches the collaborative engine into the `collaborative/` provider.

This includes:

- document creation
- indexed persistence integration
- remote provider wiring
- engine observers
- engine mutation helpers

### 4.2 Build a translation layer, not a type alias layer

Bad approach:

- `RMap = Y.Map`
- `RArray = Y.Array`

Correct approach:

- provider owns an internal document model
- provider emits `KitStoreSnapshot`
- provider accepts `KitDiff` / `replace(Kit)`
- provider keeps its own mapping logic private

### 4.3 Move engine-aware undo/redo into provider internals or neutral adapters

If undo/redo is provider-specific today, expose only neutral commands to Sketchpad:

- `canUndo`
- `undo`
- `redo`

Implementation stays private.

### 4.4 Define synchronization semantics explicitly

Standardize these states across all providers:

- loading
- ready
- dirty
- syncing
- saved
- offline
- error
- readonly

## Exit criteria

- collaborative backend works through `KitStore`
- Sketchpad cannot tell which provider is used
- no engine-specific symbol escapes the provider package

---

# Phase 5 — implement JSON file store

## Outcomes

- single-file kit editing works
- this becomes the foundation for the VS Code experience

## Behavior

- open JSON file
- parse into `Kit`
- edits update in-memory snapshot immediately
- save serializes the full kit back to the source JSON file
- external file changes can trigger reload or conflict flow

## Design choices

### Save model

Preferred:

- in-memory working copy
- debounced save
- explicit `save()` support

### Conflict model

For first version:

- detect file mtime/hash change
- if dirty and external change detected, show conflict dialog
- allow reload / overwrite / save-as

### Asset model

For JSON-backed store, choose one explicit policy and document it:

- embedded blobs only for small assets, or
- asset references only, or
- hybrid with configurable thresholds

Do not leave this ambiguous.

## Exit criteria

- `JsonFileKitStore` passes full editor integration tests
- open/edit/save/reload works from a host bridge

---

# Phase 6 — implement folder/sqlite store

## Outcomes

- folder-backed kits work for larger local workflows
- layout aligns with the python engine’s storage model

## Recommended design

### Data ownership

- sqlite is the source of truth for structured kit data
- filesystem stores blobs/assets
- optional materialized exports are derived artifacts

### Important tables

Suggested logical tables:

- `kits`
- `types`
- `qualities`
- `designs`
- `pieces`
- `connections`
- `files`
- `metadata`
- `transactions` or `revisions`

### Store capabilities

- open folder
- bootstrap `.semio/` if missing
- load kit from sqlite
- write diffs transactionally
- write assets to folder
- rebuild export snapshot when needed
- recover from partial writes

### Reliability requirements

- transaction-safe writes
- schema migrations
- lock or journal strategy
- backup/repair flow for corrupted local state

## Exit criteria

- folder-backed editing works end-to-end
- sqlite schema versioning exists
- python engine compatibility is documented and tested

---

# Phase 7 — separate the VS Code extension from `repo`

## Outcomes

- VS Code delivery is decoupled from the main repository
- file editing workflow uses the new store abstraction cleanly

## Repository split

Create a new repository, for example:

```text
semio-vscode
  extension/
  webview/
  shared/
```

## Responsibilities of the extension

### Extension host

- register for `.json` kit files
- launch webview editor on open
- provide file read/write bridge
- handle save/reload/conflict prompts
- contribute commands and file associations

### Webview

- host `semio/sketchpad`
- instantiate `JsonFileKitStore` via a host bridge
- keep webview/editor state isolated from VS Code APIs

### Contract between extension and webview

Use a thin RPC layer:

- `readFile(uri)`
- `writeFile(uri, contents)`
- `watchFile(uri)`
- `showError(message)`
- `requestSave()`

The webview must not know about VS Code internals directly.

## Important rule

The extension package is not a new bundle inside `repo`. It is a separate project consuming published Semio packages.

## Exit criteria

- opening a JSON kit file launches Sketchpad
- edits round-trip back to the JSON file
- external file changes are handled safely
- extension can version independently from `repo`

---

## Parallel workstream plan

This work is large enough to run in parallel. Treat these as separate implementation streams.

### Workstream A — core contracts and domain cleanup

Scope:

- `semio/js` neutral interfaces
- pure command/diff/domain extraction
- forbidden import rules

Deliverables:

- stable `KitStore` contracts
- domain tests
- migration codemods where helpful

### Workstream B — Sketchpad decontamination

Scope:

- inject `kitStore`
- split giant `SketchpadStore`
- remove provider imports and provider assumptions

Deliverables:

- editor boots from fake store
- no engine reference in `semio/sketchpad`

### Workstream C — collaborative provider in `semio/studio`

Scope:

- extract current collaborative backend
- implement bridge to neutral contracts
- preserve current collaboration behavior

Deliverables:

- `createCollaborativeKitStore`
- migration coverage for existing collaborative workflows

### Workstream D — JSON and folder stores

Scope:

- `JsonFileKitStore`
- `FolderKitStore`
- asset persistence strategy
- sqlite schema

Deliverables:

- two local providers with tests
- migration/import/export docs

### Workstream E — separate VS Code extension

Scope:

- new repo
- webview host
- file-open workflow
- json kit editing integration

Deliverables:

- installable extension
- end-to-end file editing demo

### Workstream F — test, migration, release

Scope:

- CI
- smoke tests across providers
- package publishing and versioning
- deprecation plan

Deliverables:

- migration guide
- rollout checklist
- release notes

---

## Migration details

### Replace inheritance-heavy stores with adapters

Current code suggests multiple store classes inherit from engine-backed bases. Replace that with:

- plain editor state reducers
- a `KitStoreAdapter` that exposes only the neutral interfaces
- pure command handlers operating on snapshots and diffs

### Keep commands pure where possible

Target pattern:

1. read `kitStore.getSnapshot().kit`
2. compute `KitDiff`
3. call `kitStore.apply(diff)`
4. store updates notify subscribers

This makes commands reusable across all providers.

### Introduce host bridges for environment-specific IO

Examples:

- browser local persistence bridge
- remote sync bridge
- Node filesystem bridge
- VS Code file bridge

The store provider may use host bridges, but the editor must not.

---

## Testing strategy

## 1. Contract tests for all `KitStore` implementations

Every provider must pass the same suite:

- load initial kit
- apply diff
- replace full kit
- subscribe notifications
- transaction grouping
- undo/redo if supported
- save/reload
- dispose
- dirty/sync/error state transitions

## 2. Editor integration tests

Run the same editor test matrix against:

- fake in-memory store
- collaborative provider
- JSON file store
- folder/sqlite store

## 3. Persistence tests

### JSON file store

- invalid json
- external change conflict
- large file save
- autosave recovery

### Folder store

- initial bootstrap
- schema migration
- rollback after write failure
- asset add/remove/rename
- sqlite corruption handling

## 4. Regression tests for feature parity

- open existing kits
- create/edit/delete domain entities
- undo/redo
- import/export
- asset attachment workflows
- collaboration workflow parity for the collaborative provider

## 5. Static checks

- forbidden import checks
- grep check for engine string inside `semio/sketchpad`
- bundle boundary checks

---

## Risks and mitigations

### Risk 1 — abstraction leaks provider details

Mitigation:

- review all public interfaces against the question: “Could this exist if the backend were sqlite only?”
- if not, it does not belong in the neutral API

### Risk 2 — giant `SketchpadStore` rewrite causes regressions

Mitigation:

- split by responsibility first
- keep compatibility adapters temporarily
- migrate feature by feature behind integration tests

### Risk 3 — collaborative semantics get lost

Mitigation:

- document required sync semantics explicitly
- add contract tests for remote update propagation and conflict handling

### Risk 4 — file and folder stores diverge too much

Mitigation:

- keep the same `KitStore` contract
- document capability differences separately
- use optional capabilities instead of leaking implementation assumptions

### Risk 5 — VS Code extension blocks core refactor

Mitigation:

- make VS Code depend on the new JSON store
- do not let extension requirements shape core editor abstractions

---

## Concrete code moves

## Move out of `semio/sketchpad`

- all direct imports of the collaboration engine
- all direct imports of indexed persistence for collaboration
- any direct document/map/array types
- provider creation / provider bootstrapping
- provider-specific observer helpers
- provider-specific mutation helpers

## Move out of `semio/js`

- any concrete provider implementation
- provider persistence code
- provider wiring and environment adapters

## Add to `semio/studio`

- `providers/collaborative/*`
- `providers/json-file/*`
- `providers/folder/*`
- provider-specific observation/mutation bridges
- factory exports in `studio.ts`

## Keep in `semio/js`

- domain types
- diffs
- serializers
- validators
- pure command logic
- neutral store contracts

## Keep in `semio/sketchpad`

- UI
- app/session state
- command invocation
- view models/selectors against snapshots

---

## Recommended implementation order

1. add neutral contracts and import guards
2. make Sketchpad boot from injected `kitStore`
3. introduce fake/in-memory store and pass editor tests
4. extract collaborative provider into `semio/studio`
5. remove remaining provider references from Sketchpad
6. implement JSON file store
7. implement folder/sqlite store
8. create separate VS Code extension using JSON file store
9. delete old provider code from `semio/sketchpad` and `semio/js`
10. publish migration guide and cut release

---

## Definition of done

The refactor is only done when all of the following are true:

- `semio/sketchpad` contains no collaboration-engine import, symbol, helper, or terminology
- `semio/js` contains no concrete kit provider implementation
- `semio/studio` exports all kit providers from `studio.ts`
- Sketchpad accepts a root `kitStore` prop and works with all three backends
- JSON file editing works end-to-end
- folder/sqlite editing works end-to-end
- the VS Code extension lives in a separate repository
- a test matrix proves provider interchangeability
- swapping the kit store requires changing provider creation only, not Sketchpad internals

---

## Acceptance checklist

- [ ] `semio/js` exports only neutral store contracts and core domain logic
- [ ] `semio/sketchpad` has no provider imports
- [ ] `semio/sketchpad` has no collaboration-engine terminology
- [ ] `semio/studio/studio.ts` exports all kit providers
- [ ] `CollaborativeKitStore` implemented
- [ ] `JsonFileKitStore` implemented
- [ ] `FolderKitStore` implemented
- [ ] fake/in-memory store used in editor tests
- [ ] contract test suite shared by all providers
- [ ] separate `semio-vscode` repository created
- [ ] opening a JSON kit file launches Sketchpad in VS Code
- [ ] file edits save back safely
- [ ] migration and release docs written

---

## Bottom line

The right refactor is to make `KitStore` the boundary and move every concrete backend behind it in `semio/studio`. `semio/sketchpad` must become a pure editor shell that consumes a kit store by injection. `semio/js` must become pure portable core. The VS Code editor must be delivered as a separate extension using the JSON file store, not by embedding extension code into the main repository.

---

Agreed. That changes the design in two important ways:

First, **undo/redo is not an optional capability**. It has to be part of the core `KitStore` contract, because today undo/redo is already baked into editor behavior and even reaches into `kit().change(...)` from Sketchpad-side transaction stacks. That logic needs to move down into the store so every backend behaves the same.

Second, **files are first-class kit entities**, not a separate asset subsystem. Your current domain already models `files` and `folders` directly on `Kit`, has `File` and `Folder` ids like every other entity, and even uses `file.blob` from `kit.files` during export. So the public architecture should remove `AssetStore` / `createBlobStore` entirely and treat file content as part of the kit state.

## The clean general solution

Make the store a **technology-neutral, collaborative, undoable, path-addressable kit state engine**.

That means:

- one public `KitStore` API for all backends
- all entities, including `File` and `Folder`, live in the same kit graph
- every read/subscription is addressed by a **domain path**
- every local or external update flows through the same mutation pipeline
- undo/redo is mandatory and consistent everywhere

## 1. Public API

```ts
export type Guid = string;

export type EntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Model" | "Layer" | "Group" | "Stat";

export type KitStoreStatus = "loading" | "ready" | "saving" | "syncing" | "offline" | "error" | "readonly";

export type KitSyncState = {
 status: KitStoreStatus;
 dirty: boolean;
 readonly: boolean;
 localRevision: string;
 externalRevision?: string;
 lastSyncedAt?: string;
 conflict?: "none" | "external-change" | "merge-required";
 error?: string;
};

export type PathSegment = { kind: "entity"; entityKind: EntityKind; id: Guid } | { kind: "collection"; field: string } | { kind: "field"; field: string };

export type DomainPath = readonly PathSegment[];

export type ApplyOrigin = "user" | "undo" | "redo" | "remote" | "file-watch" | "database-watch" | "system";

export type TransactionOptions = {
 label: string;
 origin?: ApplyOrigin;
 undoGroup?: "merge" | "separate";
};

export interface KitStore {
 // snapshot
 getKit(): Kit;
 getSyncState(): KitSyncState;

 // path-addressable reads
 read<T>(path: DomainPath): T | undefined;

 // subscriptions
 subscribe(listener: () => void): () => void;
 subscribePath(path: DomainPath, listener: () => void): () => void;

 // writes
 transact<T>(options: TransactionOptions, run: () => T): T;
 apply(mutation: KitMutation | readonly KitMutation[], meta?: { origin?: ApplyOrigin }): { revision: string };
 replace(next: Kit, meta?: { origin?: ApplyOrigin }): { revision: string };

 // persistence
 save(): Promise<void>;
 reload(): Promise<void>;

 // mandatory undo/redo
 canUndo(): boolean;
 canRedo(): boolean;
 undo(): void;
 redo(): void;

 // lifecycle
 dispose(): void | Promise<void>;
}
```

This is the right public shape because it is:

- storage-agnostic
- collaborative
- path-addressable
- undoable by definition
- compatible with the current domain model where files/folders are part of the kit itself.

## 2. Path-based subscription

The cleanest abstraction is **domain path**, not provider path and not backend primitives.

Examples:

```ts
const typeNamePath: DomainPath = [
 { kind: "entity", entityKind: "Type", id: typeGuid },
 { kind: "field", field: "name" },
];

const typeConnectorsPath: DomainPath = [
 { kind: "entity", entityKind: "Type", id: typeGuid },
 { kind: "collection", field: "connectors" },
];

const fileBlobPath: DomainPath = [
 { kind: "entity", entityKind: "File", id: fileGuid },
 { kind: "field", field: "blob" },
];

const folderNamePath: DomainPath = [
 { kind: "entity", entityKind: "Folder", id: folderGuid },
 { kind: "field", field: "name" },
];
```

That satisfies “arbitrarily deep through entities and collections” without leaking storage layout. Your existing model already has stable ids across entities, including files and folders, so this is a natural fit.

## 3. React hooks

Every property hook should be built on `read(path)` + `subscribePath(path)`.

```ts
function useKitPath<T>(store: KitStore, path: DomainPath, fallback?: T): T | undefined {
 const subscribe = React.useCallback((notify: () => void) => store.subscribePath(path, notify), [store, path]);

 const getSnapshot = React.useCallback(() => store.read<T>(path) ?? fallback, [store, path, fallback]);

 return React.useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}
```

Then every property hook is trivial:

```ts
function useTypeName(store: KitStore, typeGuid: string) {
 return useKitPath<string>(store, [
  { kind: "entity", entityKind: "Type", id: typeGuid },
  { kind: "field", field: "name" },
 ]);
}

function useFileBlob(store: KitStore, fileGuid: string) {
 return useKitPath<string | Uint8Array | undefined>(store, [
  { kind: "entity", entityKind: "File", id: fileGuid },
  { kind: "field", field: "blob" },
 ]);
}
```

That gives you the “one hook per property, only rerender on that property” guarantee.

## 4. Collections and deep traversal

For collections, subscribe to the collection first, then drill into the chosen entity.

Example: “second type → first connector → name”.

```ts
function useCollectionIds(store: KitStore, ownerKind: EntityKind, ownerId: string, field: string) {
 return (
  useKitPath<readonly string[]>(store, [
   { kind: "entity", entityKind: ownerKind, id: ownerId },
   { kind: "collection", field },
  ]) ?? []
 );
}

function useSecondTypeFirstConnectorName(store: KitStore, kitGuid: string) {
 const typeIds =
  useKitPath<readonly string[]>(store, [
   { kind: "entity", entityKind: "Kit", id: kitGuid },
   { kind: "collection", field: "types" },
  ]) ?? [];

 const secondTypeId = typeIds[1];

 const connectorIds =
  useKitPath<readonly string[]>(
   store,
   secondTypeId
    ? [
       { kind: "entity", entityKind: "Type", id: secondTypeId },
       { kind: "collection", field: "connectors" },
      ]
    : [],
  ) ?? [];

 const firstConnectorId = connectorIds[0];

 return firstConnectorId
  ? useKitPath<string>(store, [
     { kind: "entity", entityKind: "Connector", id: firstConnectorId },
     { kind: "field", field: "name" },
    ])
  : undefined;
}
```

So the rule is:

- collections are subscribed structurally
- properties are subscribed by entity id
- identity is always by guid, never by backend path
- index is only used as a structural selection step, not as long-lived identity

## 5. Mandatory undo/redo semantics

Because every store must support undo/redo, the semantics must be uniform:

- `undo()` and `redo()` are always present
- undo history is built from `transact(...)`
- each transaction stores inverse mutations
- undo/redo is **session-local**, not globally collaborative
- external changes are never pushed onto the local undo stack

That last point matters. In a collaborative store, or when a file/sqlite store changes from another process, those external updates still enter the store, but they are not “my undo steps”. They must rebase against the current local history. If rebasing fails, the store can invalidate affected undo entries and surface conflict state through `getSyncState()`. That gives the same mental model across remote, file, and sqlite backends. The refactor plan already calls out the need to preserve undo/redo while making the provider replaceable, and the current Sketchpad code shows undo is still wrongly living above the store boundary.

## 6. Collaboration model

All backends should feed the same core runtime.

- Collaborative backend: remote operations come in as external mutations.
- JSON file backend: file watcher notices disk change and emits external mutations or a replace.
- Folder/sqlite backend: db/file watchers do the same.

So the store core sees only:

- local transaction
- undo
- redo
- external mutation
- replace

That is what makes the store independent of storage technology while still collaborative.

## 7. Files are part of the kit, not a side channel

This is the most important correction to my earlier proposal.

There should be **no** public `AssetStore`, and `semio/studio` should not export `createBlobStore(...)`. The refactor plan needs to be corrected there. Files and folders already belong in the kit domain, and the store should expose them exactly like types, designs, qualities, and authors.

That does **not** force all backends to persist bytes the same way.

It only means the **public semantics** are the same:

- `File` is a kit entity
- `Folder` is a kit entity
- file content is a file field
- file metadata is file fields
- file moves, renames, content edits, and folder membership are normal kit mutations
- all of them are subscribable and undoable

Examples:

- JSON store can serialize file bytes inline
- folder/sqlite store can keep bytes in real files and structured metadata in sqlite
- collaborative store can chunk or sync bytes however it wants

But Sketchpad only sees kit entities and kit mutations. That is exactly the separation your plan calls for.

## 8. What to change in the plan

I would make these corrections:

- remove `AssetStore` from `semio/js`
- remove `createBlobStore(...)` from `semio/studio`
- make undo/redo mandatory in `KitStore`
- make `File` and `Folder` normal first-class entities in the path model
- make `subscribePath(path)` part of the core contract, not optional

## Bottom line

The clean general solution is:

- **one mandatory `KitStore`**
- **mandatory undo/redo**
- **files and folders inside the kit**
- **domain-path reads and subscriptions**
- **local and external mutations through one pipeline**

That is the smallest design that satisfies all your requirements without leaking storage technology.

I can rewrite the store contract into a concrete `semio/js/store/KitStore.ts` draft next.

---

Here is the concrete JSON store I would ship for Electron.

It matches the refactor direction you set: the JSON file store lives in `semio/studio`, Sketchpad only sees a neutral store, and JSON is a one-kit-per-file backend. It also treats files and folders as normal kit entities, which your current command layer and import/export already do, and it relies on stable `guid` identity across entities for subscriptions and merge targeting.

I am assuming the JSON file contains the full kit and that file payloads are JSON-safe values inside the kit, such as base64 or data-URI strings on file fields. That is consistent with the existing file/folder handling and the current blob decoding logic in `semio.ts`.

### 1) Electron host bridge

```ts
// semio/studio/providers/json-file/JsonFileHost.ts
export interface FileStat {
 mtimeMs: number;
 size: number;
}

export interface JsonFileHost {
 readText(path: string): Promise<string>;
 writeTextAtomic(path: string, contents: string): Promise<void>;
 stat(path: string): Promise<FileStat | null>;
 watch(path: string, onChange: () => void): () => void;
}
```

```ts
// semio/studio/providers/json-file/electronJsonFileHost.ts
import { mkdir, open, readFile, rename, stat as fsStat } from "node:fs/promises";
import { dirname } from "node:path";
import { watchFile, unwatchFile } from "node:fs";
import type { JsonFileHost, FileStat } from "./JsonFileHost";

export const electronJsonFileHost: JsonFileHost = {
 async readText(path: string): Promise<string> {
  return await readFile(path, "utf8");
 },

 async writeTextAtomic(path: string, contents: string): Promise<void> {
  await mkdir(dirname(path), { recursive: true });

  const tmpPath = `${path}.tmp-${process.pid}-${Date.now()}`;
  const fh = await open(tmpPath, "w");

  try {
   await fh.writeFile(contents, "utf8");
   await fh.sync();
  } finally {
   await fh.close();
  }

  await rename(tmpPath, path);
 },

 async stat(path: string): Promise<FileStat | null> {
  try {
   const s = await fsStat(path);
   return { mtimeMs: s.mtimeMs, size: s.size };
  } catch {
   return null;
  }
 },

 watch(path: string, onChange: () => void): () => void {
  const listener = (curr: { mtimeMs: number; size: number }, prev: { mtimeMs: number; size: number }) => {
   if (curr.mtimeMs !== prev.mtimeMs || curr.size !== prev.size) {
    onChange();
   }
  };

  watchFile(path, { interval: 250 }, listener);
  return () => unwatchFile(path, listener);
 },
};
```

### 2) JSON store implementation

This version gives you:

- one in-memory working kit
- property-level subscription with guid-addressed paths
- mandatory undo/redo
- atomic saves
- external change detection
- three-way merge when disk changed in another process
- conservative history invalidation when rebasing against external changes

```ts
// semio/studio/providers/json-file/JsonFileKitStore.ts
import { createHash } from "node:crypto";
import { deepEqual, type Kit } from "../../../js/semio";
import type { JsonFileHost } from "./JsonFileHost";

export type ApplyOrigin = "user" | "undo" | "redo" | "remote" | "file-watch" | "database-watch" | "system";

export type KitStoreStatus = "loading" | "ready" | "saving" | "syncing" | "offline" | "error" | "readonly";

export type KitSyncState = {
 status: KitStoreStatus;
 dirty: boolean;
 readonly: boolean;
 localRevision: string;
 externalRevision?: string;
 lastSyncedAt?: string;
 conflict?: "none" | "external-change" | "merge-required";
 error?: string;
};

export type DomainPathSegment = string | { guid: string };
export type DomainPath = readonly DomainPathSegment[];

export interface JsonFileKitStoreConfig {
 filePath: string;
 host: JsonFileHost;
 autosaveMs?: number;
 initialKit?: Kit;
}

export interface JsonFileKitStore {
 getSnapshot(): { kit: Kit; sync: KitSyncState };
 getKit(): Kit;
 getSyncState(): KitSyncState;

 read<T>(path: DomainPath): T | undefined;

 subscribe(listener: () => void): () => void;
 subscribePath(path: DomainPath, listener: () => void): () => void;

 update(label: string, updater: (draft: Kit) => void): void;
 replace(next: Kit, meta?: { origin?: ApplyOrigin }): { revision: string };

 save(): Promise<void>;
 reload(): Promise<void>;

 canUndo(): boolean;
 canRedo(): boolean;
 undo(): void;
 redo(): void;

 dispose(): void;
}

type HistoryEntry = {
 label: string;
 before: Kit;
 after: Kit;
};

type MergeConflict = {
 path: DomainPath;
 base: unknown;
 local: unknown;
 remote: unknown;
};

type LoadedFile = {
 kit: Kit;
 text: string;
 hash: string;
};

export async function createJsonFileKitStore(config: JsonFileKitStoreConfig): Promise<JsonFileKitStore> {
 return await JsonFileKitStoreImpl.open(config);
}

class JsonFileKitStoreImpl implements JsonFileKitStore {
 private readonly filePath: string;
 private readonly host: JsonFileHost;
 private readonly autosaveMs: number;

 private kit: Kit;
 private baseKit: Kit;
 private sync: KitSyncState;

 private baseHash: string;
 private diskHash: string;

 private readonly listeners = new Set<() => void>();
 private readonly pathListeners = new Map<string, Set<() => void>>();

 private readonly undoStack: HistoryEntry[] = [];
 private readonly redoStack: HistoryEntry[] = [];

 private unwatch?: () => void;
 private autosaveTimer?: ReturnType<typeof setTimeout>;
 private changeHandling = false;
 private suppressWatchUntil = 0;

 static async open(config: JsonFileKitStoreConfig): Promise<JsonFileKitStoreImpl> {
  const loaded = await loadOrBootstrap(config);
  const revision = loaded.hash;

  const store = new JsonFileKitStoreImpl(config.filePath, config.host, config.autosaveMs ?? 600, loaded.kit, revision);

  store.unwatch = config.host.watch(config.filePath, () => {
   void store.handleExternalFileChange();
  });

  return store;
 }

 private constructor(filePath: string, host: JsonFileHost, autosaveMs: number, initialKit: Kit, initialHash: string) {
  this.filePath = filePath;
  this.host = host;
  this.autosaveMs = autosaveMs;

  this.kit = clone(initialKit);
  this.baseKit = clone(initialKit);

  this.baseHash = initialHash;
  this.diskHash = initialHash;

  this.sync = {
   status: "ready",
   dirty: false,
   readonly: false,
   localRevision: initialHash,
   externalRevision: initialHash,
   lastSyncedAt: new Date().toISOString(),
   conflict: "none",
  };
 }

 getSnapshot() {
  return { kit: this.kit, sync: this.sync };
 }

 getKit(): Kit {
  return this.kit;
 }

 getSyncState(): KitSyncState {
  return this.sync;
 }

 read<T>(path: DomainPath): T | undefined {
  return readAtPath(this.kit, path) as T | undefined;
 }

 subscribe(listener: () => void): () => void {
  this.listeners.add(listener);
  return () => this.listeners.delete(listener);
 }

 subscribePath(path: DomainPath, listener: () => void): () => void {
  const key = pathKey(path);
  let set = this.pathListeners.get(key);
  if (!set) {
   set = new Set();
   this.pathListeners.set(key, set);
  }
  set.add(listener);

  return () => {
   const current = this.pathListeners.get(key);
   if (!current) return;
   current.delete(listener);
   if (current.size === 0) this.pathListeners.delete(key);
  };
 }

 update(label: string, updater: (draft: Kit) => void): void {
  const before = clone(this.kit);
  const next = clone(this.kit);

  updater(next);

  if (deepEqual(before, next)) return;

  this.undoStack.push({ label, before, after: clone(next) });
  this.redoStack.length = 0;

  const changedPaths = collectChangedPaths(before, next);
  this.commitCurrent(next, changedPaths, {
   origin: "user",
   dirty: true,
   clearHistory: false,
   lastSyncedAt: this.sync.lastSyncedAt,
  });

  this.scheduleAutosave();
 }

 replace(next: Kit, meta?: { origin?: ApplyOrigin }): { revision: string } {
  const origin = meta?.origin ?? "system";
  const before = clone(this.kit);
  const after = clone(next);

  if (deepEqual(before, after)) {
   return { revision: this.sync.localRevision };
  }

  const changedPaths = collectChangedPaths(before, after);

  if (origin === "user") {
   this.undoStack.push({ label: "replace", before, after: clone(after) });
   this.redoStack.length = 0;
  }

  this.commitCurrent(after, changedPaths, {
   origin,
   dirty: origin === "user" || origin === "undo" || origin === "redo",
   clearHistory: false,
   lastSyncedAt: this.sync.lastSyncedAt,
  });

  return { revision: this.sync.localRevision };
 }

 async save(): Promise<void> {
  if (!this.sync.dirty) return;

  this.setSync({
   status: "saving",
   error: undefined,
   conflict: "none",
  });

  try {
   const disk = await loadFile(this.host, this.filePath);

   // Normal save: disk still matches our merge base.
   if (disk.hash === this.baseHash) {
    const writtenHash = await writeKitFile(this.host, this.filePath, this.kit);
    this.baseKit = clone(this.kit);
    this.baseHash = writtenHash;
    this.diskHash = writtenHash;
    this.setSync({
     status: "ready",
     dirty: false,
     localRevision: writtenHash,
     externalRevision: writtenHash,
     lastSyncedAt: new Date().toISOString(),
     conflict: "none",
     error: undefined,
    });
    return;
   }

   // Concurrent disk change: merge base vs local vs current disk.
   const merge = threeWayMergeKit(this.baseKit, this.kit, disk.kit);

   if (merge.conflicts.length > 0) {
    this.setSync({
     status: "ready",
     dirty: true,
     externalRevision: disk.hash,
     conflict: "merge-required",
     error: `Concurrent change conflict at ${formatConflictPath(merge.conflicts[0].path)}`,
    });
    throw new Error(`Concurrent JSON-store change conflict (${merge.conflicts.length} conflicts).`);
   }

   const before = clone(this.kit);
   const merged = merge.merged;
   const changedPaths = collectChangedPaths(before, merged);

   const writtenHash = await writeKitFile(this.host, this.filePath, merged);
   this.baseKit = clone(merged);
   this.baseHash = writtenHash;
   this.diskHash = writtenHash;

   // Conservative: when we had to rebase local edits against another writer,
   // clear local history instead of pretending old undo steps are still valid.
   this.undoStack.length = 0;
   this.redoStack.length = 0;

   this.commitCurrent(merged, changedPaths, {
    origin: "system",
    dirty: false,
    clearHistory: false,
    lastSyncedAt: new Date().toISOString(),
   });

   this.setSync({
    status: "ready",
    dirty: false,
    localRevision: writtenHash,
    externalRevision: writtenHash,
    lastSyncedAt: new Date().toISOString(),
    conflict: "none",
    error: undefined,
   });
  } catch (error) {
   this.setSync({
    status: "error",
    error: error instanceof Error ? error.message : String(error),
   });
   throw error;
  }
 }

 async reload(): Promise<void> {
  const disk = await loadFile(this.host, this.filePath);

  if (disk.hash === this.diskHash) return;

  if (!this.sync.dirty) {
   const before = clone(this.kit);
   const after = clone(disk.kit);
   const changedPaths = collectChangedPaths(before, after);

   this.baseKit = clone(after);
   this.baseHash = disk.hash;
   this.diskHash = disk.hash;

   this.undoStack.length = 0;
   this.redoStack.length = 0;

   this.commitCurrent(after, changedPaths, {
    origin: "file-watch",
    dirty: false,
    clearHistory: false,
    lastSyncedAt: new Date().toISOString(),
   });

   this.setSync({
    status: "ready",
    dirty: false,
    localRevision: disk.hash,
    externalRevision: disk.hash,
    lastSyncedAt: new Date().toISOString(),
    conflict: "none",
    error: undefined,
   });
   return;
  }

  const merge = threeWayMergeKit(this.baseKit, this.kit, disk.kit);

  if (merge.conflicts.length > 0) {
   this.diskHash = disk.hash;
   this.setSync({
    status: "ready",
    dirty: true,
    externalRevision: disk.hash,
    conflict: "merge-required",
    error: `External process changed file. Conflict at ${formatConflictPath(merge.conflicts[0].path)}`,
   });
   this.emitGlobal();
   return;
  }

  const before = clone(this.kit);
  const merged = merge.merged;
  const changedPaths = collectChangedPaths(before, merged);

  // Disk changed externally. We rebase local unsaved state on top of new disk
  // and invalidate local history conservatively.
  this.baseKit = clone(disk.kit);
  this.baseHash = disk.hash;
  this.diskHash = disk.hash;

  this.undoStack.length = 0;
  this.redoStack.length = 0;

  this.commitCurrent(merged, changedPaths, {
   origin: "file-watch",
   dirty: true,
   clearHistory: false,
   lastSyncedAt: this.sync.lastSyncedAt,
  });

  this.setSync({
   status: "ready",
   dirty: true,
   localRevision: hashKit(merged),
   externalRevision: disk.hash,
   conflict: "none",
   error: undefined,
  });
 }

 canUndo(): boolean {
  return this.undoStack.length > 0;
 }

 canRedo(): boolean {
  return this.redoStack.length > 0;
 }

 undo(): void {
  const entry = this.undoStack.pop();
  if (!entry) return;

  const current = clone(this.kit);
  this.redoStack.push({
   label: `${entry.label}:redo`,
   before: current,
   after: clone(entry.after),
  });

  const changedPaths = collectChangedPaths(this.kit, entry.before);

  this.commitCurrent(clone(entry.before), changedPaths, {
   origin: "undo",
   dirty: true,
   clearHistory: false,
   lastSyncedAt: this.sync.lastSyncedAt,
  });

  this.scheduleAutosave();
 }

 redo(): void {
  const entry = this.redoStack.pop();
  if (!entry) return;

  const current = clone(this.kit);
  this.undoStack.push({
   label: `${entry.label}:undo`,
   before: current,
   after: clone(entry.after),
  });

  const changedPaths = collectChangedPaths(this.kit, entry.after);

  this.commitCurrent(clone(entry.after), changedPaths, {
   origin: "redo",
   dirty: true,
   clearHistory: false,
   lastSyncedAt: this.sync.lastSyncedAt,
  });

  this.scheduleAutosave();
 }

 dispose(): void {
  if (this.autosaveTimer) clearTimeout(this.autosaveTimer);
  this.unwatch?.();
  this.listeners.clear();
  this.pathListeners.clear();
 }

 private scheduleAutosave(): void {
  if (this.autosaveMs <= 0) return;
  if (this.autosaveTimer) clearTimeout(this.autosaveTimer);
  this.autosaveTimer = setTimeout(() => {
   void this.save().catch(() => {
    // sync state already updated in save()
   });
  }, this.autosaveMs);
 }

 private commitCurrent(
  next: Kit,
  changedPaths: DomainPath[],
  opts: {
   origin: ApplyOrigin;
   dirty: boolean;
   clearHistory: boolean;
   lastSyncedAt?: string;
  },
 ): void {
  this.kit = next;

  if (opts.clearHistory) {
   this.undoStack.length = 0;
   this.redoStack.length = 0;
  }

  this.sync = {
   ...this.sync,
   status: "ready",
   dirty: opts.dirty,
   localRevision: hashKit(next),
   conflict: this.sync.conflict ?? "none",
   lastSyncedAt: opts.lastSyncedAt,
   error: undefined,
  };

  this.emitPaths(changedPaths);
  this.emitGlobal();
 }

 private setSync(patch: Partial<KitSyncState>): void {
  this.sync = { ...this.sync, ...patch };
  this.emitGlobal();
 }

 private async handleExternalFileChange(): Promise<void> {
  if (Date.now() < this.suppressWatchUntil) return;
  if (this.changeHandling) return;

  this.changeHandling = true;
  try {
   await this.reload();
  } finally {
   this.changeHandling = false;
  }
 }

 private emitGlobal(): void {
  for (const listener of this.listeners) listener();
 }

 private emitPaths(paths: DomainPath[]): void {
  const emitted = new Set<string>();

  for (const path of paths) {
   const key = pathKey(path);
   if (emitted.has(key)) continue;
   emitted.add(key);

   const listeners = this.pathListeners.get(key);
   if (!listeners) continue;

   for (const listener of listeners) listener();
  }
 }
}

// ---------- file loading / writing ----------

async function loadOrBootstrap(config: JsonFileKitStoreConfig): Promise<LoadedFile> {
 try {
  return await loadFile(config.host, config.filePath);
 } catch {
  if (!config.initialKit) throw new Error(`JSON kit file not found: ${config.filePath}`);
  const text = serializeKit(config.initialKit);
  await config.host.writeTextAtomic(config.filePath, text);
  return {
   kit: clone(config.initialKit),
   text,
   hash: sha256(text),
  };
 }
}

async function loadFile(host: JsonFileHost, path: string): Promise<LoadedFile> {
 const text = await host.readText(path);
 const parsed = JSON.parse(text) as Kit;
 return {
  kit: parsed,
  text,
  hash: sha256(text),
 };
}

async function writeKitFile(host: JsonFileHost, path: string, kit: Kit): Promise<string> {
 const text = serializeKit(kit);
 await host.writeTextAtomic(path, text);
 return sha256(text);
}

function serializeKit(kit: Kit): string {
 return `${stableStringify(kit)}\n`;
}

function hashKit(kit: Kit): string {
 return sha256(serializeKit(kit));
}

function sha256(text: string): string {
 return createHash("sha256").update(text).digest("hex");
}

// ---------- path read / diff ----------

function readAtPath(value: unknown, path: DomainPath): unknown {
 let current: any = value;

 for (const segment of path) {
  if (current == null) return undefined;

  if (typeof segment === "string") {
   current = current[segment];
   continue;
  }

  if (!Array.isArray(current)) return undefined;
  current = current.find((item) => item && typeof item === "object" && item.guid === segment.guid);
 }

 return current;
}

function collectChangedPaths(before: unknown, after: unknown, path: DomainPath = [], out: DomainPath[] = []): DomainPath[] {
 if (deepEqual(before, after)) return out;

 if (isGuidObjectArray(before) && isGuidObjectArray(after)) {
  const beforeIds = before.map((x) => x.guid);
  const afterIds = after.map((x) => x.guid);

  if (!arrayShallowEqual(beforeIds, afterIds)) {
   out.push(path);
  }

  const ids = new Set([...beforeIds, ...afterIds]);
  for (const id of ids) {
   const b = before.find((x) => x.guid === id);
   const a = after.find((x) => x.guid === id);
   collectChangedPaths(b, a, [...path, { guid: id }], out);
  }
  return out;
 }

 if (Array.isArray(before) || Array.isArray(after)) {
  out.push(path);
  return out;
 }

 if (isPlainObject(before) && isPlainObject(after)) {
  const keys = new Set([...Object.keys(before), ...Object.keys(after)]);
  for (const key of keys) {
   collectChangedPaths((before as any)[key], (after as any)[key], [...path, key], out);
  }
  return out;
 }

 out.push(path);
 return out;
}

// ---------- merge ----------

function threeWayMergeKit(base: Kit, local: Kit, remote: Kit): { merged: Kit; conflicts: MergeConflict[] } {
 const conflicts: MergeConflict[] = [];
 const merged = mergeValue(base, local, remote, [], conflicts) as Kit;
 return { merged, conflicts };
}

function mergeValue(base: unknown, local: unknown, remote: unknown, path: DomainPath, conflicts: MergeConflict[]): unknown {
 if (deepEqual(local, remote)) return clone(local);
 if (deepEqual(base, local)) return clone(remote);
 if (deepEqual(base, remote)) return clone(local);

 if (isGuidObjectArray(base) || isGuidObjectArray(local) || isGuidObjectArray(remote)) {
  return mergeGuidObjectArray(asGuidObjectArray(base), asGuidObjectArray(local), asGuidObjectArray(remote), path, conflicts);
 }

 if (isPlainObject(base) || isPlainObject(local) || isPlainObject(remote)) {
  const b = asRecord(base);
  const l = asRecord(local);
  const r = asRecord(remote);

  const keys = new Set([...Object.keys(b), ...Object.keys(l), ...Object.keys(r)]);
  const result: Record<string, unknown> = {};

  for (const key of keys) {
   result[key] = mergeValue(b[key], l[key], r[key], [...path, key], conflicts);
  }

  return result;
 }

 if (Array.isArray(base) || Array.isArray(local) || Array.isArray(remote)) {
  conflicts.push({ path, base, local, remote });
  return clone(local);
 }

 conflicts.push({ path, base, local, remote });
 return clone(local);
}

function mergeGuidObjectArray(
 base: Array<{ guid: string; [k: string]: unknown }>,
 local: Array<{ guid: string; [k: string]: unknown }>,
 remote: Array<{ guid: string; [k: string]: unknown }>,
 path: DomainPath,
 conflicts: MergeConflict[],
): Array<{ guid: string; [k: string]: unknown }> {
 const baseById = new Map(base.map((x) => [x.guid, x]));
 const localById = new Map(local.map((x) => [x.guid, x]));
 const remoteById = new Map(remote.map((x) => [x.guid, x]));

 const allIds = new Set<string>([...baseById.keys(), ...localById.keys(), ...remoteById.keys()]);

 const mergedById = new Map<string, { guid: string; [k: string]: unknown }>();

 for (const id of allIds) {
  const b = baseById.get(id);
  const l = localById.get(id);
  const r = remoteById.get(id);

  if (!b && l && !r) {
   mergedById.set(id, clone(l));
   continue;
  }
  if (!b && !l && r) {
   mergedById.set(id, clone(r));
   continue;
  }
  if (b && !l && !r) {
   continue;
  }
  if (b && !l && r) {
   if (deepEqual(b, r)) continue; // local removed, remote unchanged
   conflicts.push({ path: [...path, { guid: id }], base: b, local: l, remote: r });
   continue;
  }
  if (b && l && !r) {
   if (deepEqual(b, l)) continue; // remote removed, local unchanged
   conflicts.push({ path: [...path, { guid: id }], base: b, local: l, remote: r });
   mergedById.set(id, clone(l));
   continue;
  }
  if (!b && l && r) {
   const mergedAdded = mergeValue(undefined, l, r, [...path, { guid: id }], conflicts);
   if (mergedAdded && typeof mergedAdded === "object") {
    mergedById.set(id, mergedAdded as { guid: string; [k: string]: unknown });
   }
   continue;
  }

  const mergedExisting = mergeValue(b, l, r, [...path, { guid: id }], conflicts);
  if (mergedExisting && typeof mergedExisting === "object") {
   mergedById.set(id, mergedExisting as { guid: string; [k: string]: unknown });
  }
 }

 const baseOrder = base.map((x) => x.guid);
 const localOrder = local.map((x) => x.guid);
 const remoteOrder = remote.map((x) => x.guid);

 const chosenOrder = chooseMergedOrder(baseOrder, localOrder, remoteOrder);

 return chosenOrder.filter((id) => mergedById.has(id)).map((id) => mergedById.get(id)!);
}

function chooseMergedOrder(base: string[], local: string[], remote: string[]): string[] {
 if (arrayShallowEqual(local, remote)) return local;
 if (arrayShallowEqual(base, local)) return remote;
 if (arrayShallowEqual(base, remote)) return local;

 const seen = new Set<string>();
 const out: string[] = [];

 for (const id of local) {
  if (!seen.has(id)) {
   seen.add(id);
   out.push(id);
  }
 }

 for (const id of remote) {
  if (!seen.has(id)) {
   seen.add(id);
   out.push(id);
  }
 }

 return out;
}

// ---------- utils ----------

function pathKey(path: DomainPath): string {
 return JSON.stringify(path);
}

function formatConflictPath(path: DomainPath): string {
 return path.map((segment) => (typeof segment === "string" ? segment : `{guid:${segment.guid}}`)).join(".");
}

function clone<T>(value: T): T {
 return structuredClone(value);
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
 return !!value && typeof value === "object" && !Array.isArray(value);
}

function asRecord(value: unknown): Record<string, unknown> {
 return isPlainObject(value) ? value : {};
}

function isGuidObjectArray(value: unknown): value is Array<{ guid: string; [k: string]: unknown }> {
 return Array.isArray(value) && value.every((item) => item && typeof item === "object" && typeof (item as any).guid === "string");
}

function asGuidObjectArray(value: unknown): Array<{ guid: string; [k: string]: unknown }> {
 return isGuidObjectArray(value) ? value : [];
}

function arrayShallowEqual(a: readonly string[], b: readonly string[]): boolean {
 return a.length === b.length && a.every((x, i) => x === b[i]);
}

function stableStringify(value: unknown): string {
 return JSON.stringify(sortRecursively(value), null, 2);
}

function sortRecursively(value: unknown): unknown {
 if (Array.isArray(value)) return value.map(sortRecursively);
 if (!isPlainObject(value)) return value;

 const out: Record<string, unknown> = {};
 for (const key of Object.keys(value).sort()) {
  out[key] = sortRecursively((value as Record<string, unknown>)[key]);
 }
 return out;
}
```

### 3) Example usage from Electron

```ts
import { createJsonFileKitStore } from "@semio/studio/providers/json-file/JsonFileKitStore";
import { electronJsonFileHost } from "@semio/studio/providers/json-file/electronJsonFileHost";

const kitStore = await createJsonFileKitStore({
 filePath: "/absolute/path/to/my-kit.json",
 host: electronJsonFileHost,
 autosaveMs: 800,
});

// then
// <Sketchpad kitStore={kitStore} />
```

### 4) What this implementation guarantees

The important guarantees are these:

`subscribePath()` is property-level and guid-addressed. A hook can subscribe to exactly:

```ts
["types", { guid: typeGuid }, "connectors", { guid: connectorGuid }, "name"];
```

and it will only fire when that property changes, not when unrelated parts of the kit change.

All writes are atomic at the file level. Another process will see either the old JSON or the new JSON, never half-written JSON.

External changes from another process are watched and loaded back into the store. If the local store is clean, it just updates. If the local store is dirty, it performs a three-way merge against the last synced disk state. If that merge is ambiguous, the store marks `conflict: "merge-required"` instead of silently dropping either side.

Undo/redo is always available, but this implementation intentionally clears local history after an external rebase. That is the conservative choice for the first JSON-store version; it avoids pretending old undo entries are still valid after another process changed the same file.

The next useful step is to wire your React hooks directly to guid-addressed paths like the one above.
