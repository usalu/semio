---
name: Kit Hook Refactor
overview: Refactor the kit data path so sketchpad consumes only `compose/react` hooks, `compose/react` consumes only `compose/js` stores, and `compose/js` reaches Rust only through `KitStoreHandle.execute`. All kit mutations will be represented as semantic commands, with no sketchpad-level manual diff or patch authority.
todos:
 - id: ticket
   content: Open or reopen the appropriate repo ticket under Running Sketchpad before code edits.
   status: completed
 - id: react-api
   content: Add missing `compose/react` hook surface for sketchpad kit reads and semantic commands.
   status: completed
 - id: js-boundary
   content: Refactor `compose/js` so all Rust communication uses the single `KitStoreHandle.execute` wrapper.
   status: completed
 - id: sketchpad-migration
   content: Remove direct kit stores, caches, snapshots, and manual diff/patch paths from sketchpad.
   status: completed
 - id: tests
   content: Extend existing embedded and Playwright tests, then run focused validation.
   status: completed
isProject: false
---

# Refactor Sketchpad Kit State Management

## Scope

- Work inside a new or reopened repo ticket under the `Running Sketchpad` goal before implementation.
- Primary files:
  - [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx)
  - [compose/react/index.tsx](compose/react/index.tsx)
  - [compose/js/index.ts](compose/js/index.ts)
  - [compose/js/worker.ts](compose/js/worker.ts)
  - Existing embedded tests in those same files.

## Target Flow

```mermaid
flowchart LR
  Sketchpad[compose/sketchpad] --> ReactHooks[compose/react hooks]
  ReactHooks --> JsStores[compose/js stores]
  JsStores --> Execute[KitStoreHandle.execute]
  Execute --> RustStore[compose/rs KitStore]
  RustStore --> Events[semantic command events]
  Events --> JsStores
  JsStores --> ReactHooks
```

## Implementation Steps

1. Remove sketchpad-owned kit authority.
   - Delete direct `@compose/js` kit imports from [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx), including `executeComposeKitCommand`, `createKitCommandEngineExplicitOrigin`, concrete store constructors, local kit file state helpers, and direct DTO import/export helpers.
   - Replace `SketchpadStore` kit registry/shallow caches, browser persistence caches, `kitStore.getSnapshot()` reads, and direct `kitStore.subscribe()` reads with `compose/react` hooks.
   - Remove the window bridge `__COMPOSE_EXECUTE_COMPOSE_KIT_COMMAND__` and the legacy `sketchpadAttachKitReadShell` snapshot mutation path.

2. Promote missing hook APIs into `compose/react`.
   - Add narrow hooks for sketchpad needs that are currently implemented locally: open kit list, active kit, kit kind/source, kit shallow list, kit persistence/file URLs, command dispatchers, import/export/open/create, undo/redo, and backbone/conflict operations where UI needs them.
   - Keep hooks focused: each hook returns only the field or command surface needed, backed by `useSyncExternalStore` or existing store selectors.
   - Ensure `compose/react` imports only `@compose/js` plus React.

3. Collapse `compose/js` to the execute-only Rust boundary.
   - Refactor `kitGraphqlRun`, `kitGraphqlExecuteStoreCommand`, read helpers, and command shell helpers so all Rust interaction routes through one internal `execute` wrapper over `KitStoreHandle.execute`.
   - Remove or internalize public store `replace(kit)` escape hatches for authoritative mutation. Host stores may mirror snapshots, but consumers must not mutate kit state through `replace` or manual patch calls.
   - Replace string bridge commands like `compose.kit.patchTypes`, `patchDesigns`, and `__patch` field writes with typed semantic command calls that return request ids and reconcile through lifecycle events.

4. Convert sketchpad mutations to semantic commands.
   - Replace `kit.fixPiecesInDesignDiff`, `dragPiecesInDesignDiff`, `compose.kit.setField(..., "__patch", diff)`, `patchTypes`, `patchDesigns`, and `KitDiffAppStore` mutation paths with `compose/react` semantic command hooks.
   - Keep app UI state in sketchpad state machine/store dispatch, but every kit change must flow through a hook command and receive its result from the `compose/react`/`compose/js` event path.
   - Preserve preview-only UI state where needed, but do not apply manual kit diffs or mutate local kit snapshots in sketchpad.

5. Update tests in place.
   - Extend [compose/js/index.ts](compose/js/index.ts) embedded tests to assert all kit store operations use the execute wrapper and no public manual patch path remains.
   - Extend [compose/react/index.tsx](compose/react/index.tsx) embedded tests for new hook APIs, command lifecycle events, and error propagation.
   - Extend existing [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx) Playwright/embedded coverage for import/open, kit list rendering, design drag/delete/undo, and details panel command edits.

## Verification

- Run the focused JS/react/sketchpad checks first, then the relevant Playwright slice.
- Run lints on touched files after edits.
- Close the repo ticket with changed files and a concise implementation summary.
