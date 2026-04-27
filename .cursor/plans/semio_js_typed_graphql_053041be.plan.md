---
name: semio js typed graphql
overview: "Refactor `semio/js` into a stateless typed GraphQL bridge to `semio/rs`: remove JS-side kit DTO caches and snapshot reads, keep only request ids for transport/event correlation, and update direct React consumers to read through the typed async surface."
todos:
 - id: reopen-ticket
   content: Reopen or continue the existing Semio JS Exact GraphQL And Wire Typing ticket and record this refinement there.
   status: completed
 - id: remove-snapshot-channel
   content: Delete snapshot transport operations and route all full-kit reads through typed GraphQL `Query.kit(scope)`.
   status: completed
 - id: remove-js-caches
   content: Remove `WasmKitStoreClient.lastDto`, DTO bridge cache APIs, fallback fixed DTO reads, and semio/js read snapshot maps/weak maps.
   status: completed
 - id: typed-graphql-layer
   content: Add typed GraphQL operation helpers and convert read/mutation/subscription call sites to typed variables and typed response data.
   status: completed
 - id: update-react-consumers
   content: Update semio/react to consume async typed reads/subscriptions without relying on semio/js cached DTO snapshots.
   status: completed
 - id: extend-tests
   content: Extend embedded semio/js tests and affected React tests to enforce no JS kit cache and typed GraphQL-only reads.
   status: completed
 - id: verify
   content: Run semio/js and semio/react build/test commands plus layering check if affected.
   status: completed
isProject: false
---

# Semio JS Stateless Typed GraphQL Plan

## Scope

- Continue the existing open ticket `[.repo/🎫/26/04/27/SEMIO-JS-EXACT-GRAPH-QL-AND-WIRE-TYPING/ticket.json](.repo/🎫/26/04/27/SEMIO-JS-EXACT-GRAPH-QL-AND-WIRE-TYPING/ticket.json)` after plan approval.
- Main implementation files: `[semio/js/index.ts](semio/js/index.ts)` and direct consumer updates in `[semio/react/index.tsx](semio/react/index.tsx)`.
- Contract authority remains `[semio/rs/lib.rs](semio/rs/lib.rs)` and `[semio/graphql/schema.graphql](semio/graphql/schema.graphql)`.

## Implementation

- Remove the non-GraphQL read channel from `semio/js`: delete worker/transport `snapshot` operations, remove `InlineWasmTransport.snapshotJson`, remove worker `snapshotResult`, and replace public full-kit reads with typed `Query.kit(scope: $scope) { fullDto }` operations.
- Make every `KitStore.read(...)` branch forward directly to `semio/rs` GraphQL. `readKitFullCommand`, `theKit()`, `materializedLiveJsonForReadScope(...)`, entity `full()` helpers, and catalog reads must all use typed GraphQL operations without falling back to WASM `snapshot()` or a local DTO.
- Remove JS-side kit state/caches from `WasmKitStoreClient` and related bridge types: eliminate `lastDto`, `getDto()`, `getSnapshot()` cache semantics, `SemioKitBridge`, `FallbackKitClient` fixed DTO reads, and the `WeakMap`/`Map` snapshot hubs that cache kit/read DTOs in `semio/js` (`liveReadHubs`, `viewStores`, design/shallow snap caches). Keep only request ids for worker execute/subscribe correlation and GraphQL/command lifecycle `requestId` matching.
- Introduce a typed GraphQL operation layer inside `[semio/js/index.ts](semio/js/index.ts)`: operation constants paired with exact variable/data/result TypeScript types, a generic `kitGraphqlRunTyped<TVariables, TData>()`, and typed helpers for `kit(scope)` query, batch mutation, and event subscription. Inline query strings may remain as operation documents, but all call sites must pass typed variables and unwrap typed response data instead of untyped `JsonObject` casts.
- Update `[semio/react/index.tsx](semio/react/index.tsx)` to stop depending on `semio/js` cached DTO snapshots. React may keep UI-local `useSyncExternalStore` snapshots if needed, but `semio/js` must expose only async reads, typed stores, mutations, subscriptions, and request/event correlation.
- Keep `semio/rs` as the only owner of kit graph state and caching. Add Rust/schema changes only if a JS read currently cannot be expressed as a typed `Query.kit(...)` field.

## Verification

- Extend existing embedded tests in `[semio/js/index.ts](semio/js/index.ts)` rather than adding new test files. Cover: no public `snapshot`/`getDto` cache surface, full reads use GraphQL, worker messages only use request ids for execute/subscribe, and typed operation constants align with `[semio/graphql/schema.graphql](semio/graphql/schema.graphql)`.
- Update existing React tests only where import/API changes require it.
- Run focused checks: `npm run build` and `npm run test` in `[semio/js](semio/js)`, then `npm run build` and `npm run test` in `[semio/react](semio/react)`. If layering imports change, run the root dependency-cruiser layer check as well.
