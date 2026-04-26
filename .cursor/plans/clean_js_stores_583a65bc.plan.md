---
name: clean js stores
overview: Refactor `semio/js` to expose stateless, typed TypeScript store facades over the single Rust execute boundary while preserving the existing request/response lifecycle semantics.
todos:
 - id: ticket
   content: Reopen the existing stateless store ticket after approval and track changed files there.
   status: completed
 - id: rename-host-store
   content: Rename snapshot-backed host persistence store types away from the public `KitStore` name.
   status: in_progress
 - id: typed-execute
   content: Introduce typed command/result/request receipt maps around the single Rust execute boundary.
   status: pending
 - id: store-facades
   content: Implement stateless `KitStore` plus domain facades as the only public semio/js store API.
   status: in_progress
 - id: remove-backbone-js
   content: Remove backbone command/details from semio/js public APIs and leave attachment ownership in semio/rs.
   status: pending
 - id: react-hooks
   content: Update React hooks to use typed facades and request lifecycle events.
   status: pending
 - id: tests
   content: Extend existing Rust, JS, and React tests and run focused validation.
   status: pending
isProject: false
---

# Clean Stateless Semio JS Stores

## Current State

`semio/js/index.ts` still exposes two different store ideas under the same names: host persistence stores (`InMemoryKitStore`, `JsonFileKitStore`, `FolderKitStore`) with `getSnapshot()`/`replace()`, and `KitStoreClient` as the Rust/WASM bridge. The bridge still exposes generic field and child helpers (`patchEntityField`, `addChild`, `removeChild`) and backbone-shaped commands/details. Lifecycle events already use the preferred `requestId` request/response correlation.

`semio/rs/lib.rs` already has the single `KitStoreHandle.execute`/GraphQL boundary and a semantic command shell that emits `SemioKitCommand { requestId, commandKind, phase }`. `semio/react/index.tsx` still routes public writes through field/patch helpers instead of typed semio store facades.

## Implementation

1. Reopen the existing matching ticket `CLEAN-STATELESS-KIT-STORES-AND-KIT-COMMAND-REQUESTS` after approval, under the Running Sketchpad goal, instead of creating a duplicate ticket.

2. In [semio/js/index.ts](semio/js/index.ts), rename the host persistence contract away from `KitStore` to avoid colliding with the requested API, for example `KitHostStore` or `KitPersistenceStore`. Keep its snapshot behavior only for file/folder/temporary host integration, not as the native semio store facade, and do not export it as an alternative public semio store path.

3. Add the new stateless TypeScript-native facade layer in [semio/js/index.ts](semio/js/index.ts):

- `KitStore` owns only a Rust execute handle/client, an event subscription, and a pending request map keyed by request id.
- Domain facades such as `DesignStore`, `TypeStore`, `PieceStore`, `ConnectionStore`, `FamilyStore`, `FileStore`, and `FolderStore` are lightweight command builders that hold only a parent `KitStore` reference and optional entity ids.
- Reads and writes go through one typed `execute(command)` method; no facade stores kit data, DTO snapshots, mutable mirrors, field names, patches, or generic diffs.
- Remove or make internal any old public `KitStoreClient` mutation helpers so the facade layer is the only supported semio/js write API.

4. Replace the `KitStoreClient` public mutation surface with discriminated command/result maps:

- Define `KitExecuteCommandMap` and `KitExecuteResultMap` so a command kind determines its exact input and output.
- Change `execute(cmd: unknown): Promise<KitStoreExecuteResult>` into a generic typed method that returns a request receipt.
- Replace `patchEntityField`, `addChild`, `removeChild`, and patch-shaped update hooks with semantic methods such as `kit.rename`, `design.rename`, `design.createFixedPiece`, `design.dragPieces`, `type.rename`, `type.addConnector`, `family.addPort`, `folder.moveArtifact`, etc.
- Remove backbone attach/detach/status/sync command builders from the semio/js public API. If a UI needs to trigger backbone behavior later, it must do so through a Rust-owned semantic request that does not expose JS-side backbone configuration/details.

5. Preserve Rust request/response lifecycle naming in [semio/rs/lib.rs](semio/rs/lib.rs):

- Keep `SemioKitCommand { requestId, commandKind, phase }` and `submitKitCommand { requestId, commandKind, accepted }` as the wire contract.
- Keep the single `execute` boundary and semantic command shell as the only mutation entry point.
- Keep attach/detach/status/sync ownership in Rust. JS may receive high-level request lifecycle events, but it must not construct or interpret backbone-specific payloads.
- If Rust still accepts GraphQL JSON for command execution internally, keep that hidden behind typed JS command builders rather than exposing JSON/field semantics.

6. Update [semio/react/index.tsx](semio/react/index.tsx) to consume the new facade API:

- Replace generic `setFieldValue`, `setObjectValue`, and `useUpdate*({ patch })` internals with calls into the typed store facades.
- Keep React hook state and UI caching in React where needed; do not reintroduce kit state inside `semio/js` store classes.

7. Extend existing tests in place:

- In [semio/js/index.ts](semio/js/index.ts), add compile-time and runtime tests proving store facades only track pending requests, command/result typing rejects invalid pairs, no public `patchEntityField`/field-update surface remains, and no public backbone API remains in semio/js.
- In [semio/rs/lib.rs](semio/rs/lib.rs), keep or extend lifecycle tests for request accepted/succeeded/failed events through the execute path.
- In [semio/react/index.tsx](semio/react/index.tsx), update embedded tests/stubs for request events and typed write hooks.

## Verification

Run focused checks after implementation:

- `cargo test` for the relevant `semio/rs` tests.
- The `semio/js` Vitest/type-check path for embedded tests.
- The `semio/react` type/test path if hook signatures change.
- `ReadLints` on touched files before closing the ticket.
