---
name: Clean Store Tasks
overview: Implement a stateless `compose/js` store facade layer over the single Rust `KitStoreHandle.execute` boundary, and extend `compose/rs` so every execute call creates a task id with lifecycle/result events.
todos:
 - id: ticket
   content: Open or reopen the repo ticket for this follow-up under the Running Sketchpad goal.
   status: completed
 - id: rs-task-execute
   content: Extend Rust execute to allocate task ids and emit task lifecycle events for every command.
   status: cancelled
 - id: js-stateless-stores
   content: Refactor compose/js store exports into stateless facades over the single execute boundary.
   status: cancelled
 - id: react-task-events
   content: Update compose/react task event naming and hook consumption where needed.
   status: completed
 - id: tests
   content: Extend existing Rust, JS, and React tests and run focused validation.
   status: completed
isProject: false
---

# Clean Stateless Stores Over Rust Execute

## Scope

- Work under the `Running Sketchpad` goal. Since the exact prior hook-refactor ticket is closed and this is a follow-up slice, open a new ticket after plan approval unless the user explicitly wants the open umbrella kit refactor ticket reused.
- Primary files:
  - [compose/rs/lib.rs](compose/rs/lib.rs)
  - [compose/js/index.ts](compose/js/index.ts)
  - [compose/js/worker.ts](compose/js/worker.ts)
  - [compose/react/index.tsx](compose/react/index.tsx) only where public types/hooks need the renamed task semantics

## Current Boundary

`compose/js` already has a raw execute adapter, but higher-level clients still expose cached snapshots, generic `any` payloads, and direct field mutation helpers that need to be removed:

```ts
export interface KitStoreClient {
 getDto(): any;
 getSnapshot(): Promise<any>;
 setField(kind: string, id: string, field: string, value: unknown): Promise<SetResult>;
 // ...many mutation/read helpers...
 execute(cmd: unknown): Promise<KitStoreExecuteResult>;
}
```

Rust currently has `kit_store::KitStore::execute(cmd) -> KitStoreCommandResult` and a GraphQL semantic shell that returns `requestId` for submitted commands, but not every `execute` call is modeled as a task.

## Hard Type-Safety Rules

- Do not introduce generic `any`. Existing `any` at this boundary must be replaced with explicit wire DTOs, discriminated unions, `unknown` narrowed by parsers, or generic parameters constrained by concrete command/result maps.
- Remove `setField` from the public store surface. Domain stores must expose typed operations such as `design.rename(...)`, `type.addAttribute(...)`, or typed command objects instead of `(kind, id, field, value)`.
- The single `execute` entry point must remain fully typed: command `kind` determines its exact input and result shape at compile time.
- Rust wire enums and TypeScript command/result unions must stay aligned, with tests asserting representative command/result typing rather than relying on dynamic string fields.

## Implementation Plan

1. Add task semantics to `compose/rs` execute.
   - Introduce a typed `TaskId`/`KitStoreTask` model and a `KitStoreTaskEvent` lifecycle shape: accepted, succeeded, failed.
   - Make `KitStoreHandle.execute` allocate a task id for every command, including read/control commands, and emit task lifecycle events with result or error.
   - Collapse the existing `requestId`/semantic command shell naming into `taskId` at the Rust wire boundary.
   - Keep command execution single-writer through the existing actor/coordinator paths; only the execute return contract changes to task receipt.

2. Refactor `compose/js` into clean stateless stores.
   - Add a small `KitStore` wrapper that only owns the Rust/worker handle plus a pending task id registry.
   - Add domain-scoped facades like `DesignStore`, `TypeStore`, `PieceStore`, etc. as stateless command builders over the owning `KitStore.execute` method.
   - Define a typed `KitCommandMap`/`KitCommandResultMap` (or equivalent) so every domain facade operation is statically tied to one command and one result shape.
   - Remove local authoritative kit state from exported store APIs: no cached `Kit`, no `replace`, no mutable snapshot mirror in stores.
   - Reads become execute commands or event/result subscriptions; UI-level caching may live in React hooks if needed, not in the JS store objects.

3. Replace the current JS client convenience mutation surface.
   - Delete the generic `setField` API and replace it with typed command builders for every supported mutation.
   - Route add-child, drag/move/fix, undo/redo, backbone commands, and read batches through the single typed `execute` call.
   - Convert `KitStoreExecuteResult`/`SetResult` to task-aware results: immediate `{ ok: true, taskId }` plus task lifecycle events for completion/result/error.
   - Update worker API methods in `compose/js/worker.ts` to expose execute-only behavior instead of parallel command-specific methods.

4. Update `compose/react` consumers.
   - Rename request-id concepts to task-id concepts where the public API leaks them.
   - Keep hooks consuming lifecycle events, but adapt to the task event shape and the stateless store wrappers.
   - Avoid reintroducing store-owned state in React-facing `@compose/js` classes.

5. Extend existing tests in place.
   - In `compose/rs/lib.rs`, add Rust tests that `execute` always returns a task id and emits accepted/succeeded/failed events for read, write, and error cases.
   - In `compose/js/index.ts`, extend embedded Vitest coverage to assert the stores keep only pending task ids, every store operation calls typed `execute`, no public `setField` remains, and no public `replace`/snapshot mutation path remains.
   - Add compile-time TypeScript checks in existing test/type-check locations for representative commands so invalid payload/result pairings fail before runtime.
   - In `compose/react/index.tsx`, extend embedded coverage for task event consumption and error propagation if hook APIs change.

## Verification

- Run focused Rust tests for the new execute/task behavior.
- Run `compose/js` embedded Vitest tests.
- Run `compose/react` build/tests if public hook types change.
- Run lints on touched files.
- Close the ticket with `compose/rs/lib.rs`, `compose/js/index.ts`, `compose/js/worker.ts`, and any `compose/react/index.tsx` changes listed.
