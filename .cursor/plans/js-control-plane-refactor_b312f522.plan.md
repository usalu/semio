---
name: js-control-plane-refactor
overview: Refactor `semio/js` so store, GraphQL, event, and change handling follow the Rust control plane as the source of truth, removing the old shell-style paths and duplicated event/read logic.
todos:
 - id: ticket
   content: Use the existing open JS store/control-plane ticket, and link any Rust schema/control-plane prerequisites before editing.
   status: completed
 - id: contract
   content: Codify the Rust/GraphQL wire contract in JS tests and type boundaries.
   status: completed
 - id: writes
   content: Replace legacy shell mutation paths with one typed batch executor.
   status: completed
 - id: events
   content: Simplify event normalization and typed event touch filters.
   status: completed
 - id: reads
   content: Deduplicate scoped GraphQL read mapping into one adapter.
   status: completed
 - id: consumers
   content: Update React/sketchpad consumers to stay behind the JS store boundary.
   status: completed
 - id: verify
   content: Run JS, React, Rust/schema, and targeted sketchpad checks relevant to changed paths.
   status: completed
isProject: false
---

# JS Control Plane Refactor

## Scope

- Work primarily in [`semio/js/index.ts`](semio/js/index.ts), with compatibility updates in [`semio/react/index.tsx`](semio/react/index.tsx) and only necessary schema alignment checks against [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) and [`semio/rs/lib.rs`](semio/rs/lib.rs).
- Continue the existing open JS store/control-plane work instead of creating a parallel architecture; keep the Rust control-plane refactor as the naming and behavior authority.
- Preserve the current package entry point, but reorganize the internal regions so transport, GraphQL, reads, writes, events, stores, and tests are cleanly separated inside the existing file.

## Current Problems

- [`semio/js/index.ts`](semio/js/index.ts) has multiple overlapping control paths: direct `kitStore.batch`, `submitChangeKitCommands`, and legacy `submitShell` wrappers.
- GraphQL strings, result parsing, event normalization, read mapping, store facades, fallback clients, and tests live together with repeated shape knowledge.
- Events are normalized through several layers: `semioKitCommand`/`SemioKitCommand`, classified mutation rows, field invalidation rows, and JSON-subtree fallback checks.
- React still exposes several bridge paths that call schema-field helpers or graph ops directly, while the intended layering is `sketchpad -> react -> js -> GraphQL -> rs`.

## Implementation Plan

1. Start ticketed implementation against the existing open JS store/control-plane ticket, and coordinate with the open Rust control-plane ticket before changing JS wire assumptions.
2. Make the Rust/GraphQL contract explicit in JS:
   - Keep `KitStoreBatchInput`, `KitStoreBatchResult`, `KitReadScopeInput`, `KitEventScalar`, `ChangeKitCommandWire`, and `KitChangeKindWire` as the only accepted wire shapes.
   - Add focused embedded tests that assert JS GraphQL operation expectations match [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) for batch, event stream, and scoped reads.
3. Collapse write handling in [`semio/js/index.ts`](semio/js/index.ts):
   - Route all mutations through one typed batch executor.
   - Replace `submitShell`/`submitShellJson` callers with typed command helpers or explicit VCS/backbone batch commands.
   - Remove shell wording and any command-kind string router that duplicates Rust semantic command handling.
4. Collapse event handling:
   - Normalize subscription output once at the GraphQL boundary.
   - Remove dual casing shims once Rust emits one stable event scalar shape.
   - Replace broad JSON-subtree touch detection with typed classified event filters where Rust provides enough information.
5. Refactor reads into one table-driven scoped read adapter:
   - Keep `KitStore.read(scope, batch)` as the public JS read facade.
   - Deduplicate `mapReadCommand`, `mapDesignRead`, and entity-row helpers behind shared `gqlRunWithReadScope` parsing utilities.
   - Keep full DTO refresh only where React still needs it, and narrow refreshes where typed events can invalidate specific stores.
6. Clean React/sketchpad boundaries:
   - Keep [`semio/react/index.tsx`](semio/react/index.tsx) as a consumer of `KitStoreClient` promises plus subscription only.
   - Remove or quarantine remaining direct schema-field write paths after equivalent typed commands exist.
   - Confirm [`semio/sketchpad/index.tsx`](semio/sketchpad/index.tsx) continues to use React hooks rather than JS internals.
7. Extend existing tests only:
   - Add embedded `semio/js` tests for typed batch execution, event normalization, scoped read mapping, removed shell paths, and React boundary behavior.
   - Extend existing React/sketchpad tests only where public behavior changes.

## Verification

- Run `npm test` in [`semio/js`](semio/js).
- Run the relevant `semio/react` validation command from its package scripts or monorepo target.
- Run focused sketchpad/play tests for the kit mutation and read paths touched by the refactor.
- If Rust or SDL changes are needed, run the Rust schema/test checks first and regenerate [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) via the existing schema build path.
