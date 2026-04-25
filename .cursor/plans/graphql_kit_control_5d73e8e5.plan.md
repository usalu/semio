---
name: graphql kit control
overview: Migrate the kit store control plane in `semio/rs` and `semio/js` so GraphQL is the only runtime boundary for reads, mutations, and event streams, replacing enum-command dispatch and direct WASM mutation helpers instead of layering on top of them.
todos:
 - id: map-coverage
   content: Map every existing `KitStoreCommand`, direct WASM helper, and JS client method to its replacement GraphQL query, mutation, or subscription field.
   status: completed
 - id: rust-schema
   content: Refactor `semio/rs/lib.rs` so existing stores expose GraphQL objects directly and mutations no longer dispatch through `KitStoreCommand`.
   status: in_progress
 - id: actor-bus
   content: Implement the inbound mutation queue and outbound subscription stream as separate async channels with boot/handle lifecycle wiring.
   status: pending
 - id: js-client
   content: Rewrite `semio/js/index.ts` client and worker paths so GraphQL `execute` is the only kit-store boundary.
   status: pending
 - id: schema-align
   content: Update `semio/graphql/schema.graphql` to match the new runtime schema and remove selector/id command-plane fields.
   status: pending
 - id: tests
   content: Extend existing Rust and JS test blocks, then run the relevant Rust, WASM, JS, and type-check commands.
   status: pending
isProject: false
---

# GraphQL Kit Control Plane Migration

## Target Shape

The current split is not aligned with the requested architecture:

- Rust WASM GraphQL lives in [`semio/rs/lib.rs`](semio/rs/lib.rs) under `kit_graphql`, but mutations still enqueue `GraphWork::KitStoreCommand` and execute the old `KitStoreCommand` enum.
- Native Rust `kit_store::KitStore` in [`semio/rs/lib.rs`](semio/rs/lib.rs) is still the backbone/coordinator control plane and exposes `execute(KitStoreCommand)`.
- JS `KitStoreClient` in [`semio/js/index.ts`](semio/js/index.ts) already has a streaming `kitGraphql().execute(...)`, but writes still call direct WASM helpers like `executeChangeKitCommands`, and VCS/backbone calls still use tagged command JSON via `kitGraphqlExecuteStoreCommand`.
- The checked-in schema [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) still exposes selector/id shapes that do not match the “pointers only, resolve in memory” directive.

The implementation should make GraphQL the control plane, not a transport for the old enum API.

## Rust Plan

1. Convert the Rust schema from WASM-only to the canonical kit-store schema used by both WASM and native paths.
   - Keep the single `execute(request_json, on_message)` WASM boundary on `KitStoreHandle`.
   - Move the schema builder and root `Query` / `Mutation` / `Subscription` out from `#[cfg(target_arch = "wasm32")]` where needed so native `kit_store::KitStore` can also execute GraphQL internally or expose it to semio-store.

2. Replace enum-command mutation dispatch with typed GraphQL mutation resolvers.
   - Remove `GraphWork::KitStoreCommand` and `run_kit_store(...)` from the active GraphQL path.
   - Turn VCS/session/backbone operations into mutation fields that enqueue typed actor work or call the existing store methods directly behind the mutation resolver.
   - Remove `kit_store_execute`, `kit_store_batch`, and JSON command passthrough fields from the GraphQL schema.

3. Collapse the actor model into the requested dual-channel bus.
   - Keep one inbound `async_channel` queue for mutation work.
   - Add a separate outbound event channel for mutation results, errors, and spontaneous store events.
   - Make subscriptions consume only the outbound stream; reads never use queues.
   - Keep actor startup in `boot()` / handle creation and avoid polling by awaiting channel receivers.

4. Add `#[Object]` to existing store structs instead of wrapper GraphQL node structs.
   - Remove `KitStoreNode`, `DesignNode`, `PieceNode`, `TypeNode`, `ConnectionNode`, `ConnectorNode`, and `RepresentationNode` from `kit_graphql`.
   - Implement GraphQL fields on the existing graph/store kinds (`KitGraph`, `DesignStore`, `PieceStore`, `TypeStore`, etc.) in their existing regions.
   - Where async-graphql cannot expose `Arc<RwLock<T>>` directly, add local resolver helper functions only, not new public wrapper structs.

5. Remove id-based GraphQL navigation from the control plane.
   - Drop fields such as `designForId`, `typeForId`, `pieceForId`, and selector inputs from the runtime schema.
   - Expose nested object traversal from already-resolved in-memory pointers, for example `kitStore { designs { pieces { refType { ... } } } }`.
   - Keep ids only as persisted DTO data where the graph format requires them, not as the live GraphQL navigation mechanism.

6. Rework native `kit_store::KitStore` as a GraphQL object/control-plane host.
   - Add `#[Object]` resolvers for native-only backbone/coordinator capabilities directly on the existing store.
   - Remove or retire `KitStoreCommand` / `KitStoreCommandResult` as the control-plane surface after GraphQL mutations cover all operations.
   - Preserve the underlying coordinator/wip/backbone mechanics, but make them implementation details behind GraphQL mutations and subscriptions.

## JS Plan

1. Make `KitStoreClient` call GraphQL for every store operation.
   - Replace direct calls to `handle.changeKitCommandsForFieldPatch`, `handle.executeChangeKitCommands`, `applyDesignDiff`, `undo`, `redo`, and other direct mutation helpers with typed GraphQL mutation documents.
   - Keep `kitGraphqlRun`, `kitGraphqlFirstData`, and the single `KitGraphqlHandle.execute` stream as the only WASM/worker store boundary.

2. Delete the tagged command mapper as an active API.
   - Remove `kitGraphqlExecuteStoreCommand` and `execute(cmd: unknown)` from the primary control plane.
   - Replace `attachBackbone`, `detachBackbone`, `backboneStatus`, `listConflicts`, `resolveConflict`, `syncNow`, session, draft, checkpoint, and alternative calls with direct GraphQL mutation/query/subscription helpers.

3. Update live reads to pointer-style GraphQL traversal.
   - Replace `LiveKitRoot.design(id)`, `LiveKitRoot.type(id)`, and `LivePieceView(designId, pieceId)` with facades created from GraphQL result objects or index-based/nested traversal where the UI already has the parent pointer context.
   - Keep UI convenience methods if needed, but implement them by resolving from in-memory graph traversal in Rust rather than sending ids as selectors.

4. Update worker lifecycle.
   - Keep `kitWorkerApi.graphqlExecute` as the only worker call for store control.
   - Remove worker methods that mirror direct WASM mutations and instead call shared GraphQL helper functions.
   - Start the subscription once after `boot()` and route outbound store events through the existing listener map.

5. Align schema artifacts.
   - Update [`semio/graphql/schema.graphql`](semio/graphql/schema.graphql) to the actual runtime schema: `Query` for live reads, `Mutation` for queued updates, `Subscription` for outbound events.
   - Remove old selector/id inputs and enum-command-oriented fields.
   - Keep any persisted DTO id fields only where the GraphQL object is exposing serialized data, not as lookup arguments.

## Validation Plan

- Extend existing Rust tests in [`semio/rs/lib.rs`](semio/rs/lib.rs); do not add new test files.
- Extend embedded JS/Vitest tests in [`semio/js/index.ts`](semio/js/index.ts); do not add new test files.
- Cover at least:
  - GraphQL query reads live in-memory stores without command enums.
  - GraphQL mutations enqueue work and emit outbound subscription events.
  - Direct WASM mutation helpers and `kitStoreExecute` no longer exist on the client path.
  - Backbone/coordinator operations work through GraphQL mutations on native store paths.
  - Schema file and runtime query strings match.
- Run the relevant Rust native tests, WASM tests/build if available, JS Vitest suite, and TypeScript checks for affected packages.
