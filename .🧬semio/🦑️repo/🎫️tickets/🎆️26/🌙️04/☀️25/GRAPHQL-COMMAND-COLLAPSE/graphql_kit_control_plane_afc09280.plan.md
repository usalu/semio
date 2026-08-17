---
name: Graphql Kit Control Plane
overview: "Rust-owned async-graphql execute stream is the WASM control plane; compose/js maps many ReadKitCommand batches to kitStore field queries via kitGraphLive. Full removal of read-command JSON and selector-heavy SDL is tracked as follow-up (see todos)."
todos:
 - id: rs-graphql-boundary
   content: Replace Rust WASM command/read exports with boot plus single async-graphql execute stream and dual actor channels.
   status: completed
 - id: rs-object-resolvers
   content: Add async-graphql Object impls on kit graph/store refs for stored and computed fields (incremental coverage).
   status: in_progress
 - id: rs-command-collapse
   content: Convert remaining read/change/session/backbone semantics from command IDs to pure GraphQL fields/mutations/events.
   status: in_progress
 - id: schema-rewrite
   content: "Rewrite compose/graphql/schema.graphql toward traversal-first kitStore (reduce EntityIdInput / *SelectorInput where resolver graph allows)."
   status: in_progress
 - id: js-graphql-only
   content: "Replace executeRead + readCommandTypes batch with typed GraphQL operations only (remove kitGraphqlMapReadCommand shim)."
   status: pending
 - id: downstream-alignment
   content: Rewire compose/react, compose/sketchpad, compose/algorithms to consume only the JS GraphQL document surface (no ReadCommandBatch).
   status: pending
 - id: verification
   content: Extend tests; run cargo test, wasm tests, pnpm -F @semio-tech/compose-js|react|sketchpad (note Windows LNK1104 if linker locks test exe).
   status: in_progress
isProject: false
---

# GraphQL Kit Control Plane Migration

## Completion notes (synced with repo)

- **Done:** WASM `execute` streams GraphQL; `KitStoreClient` uses `kitGraphqlExecuteRead` / `kitGraphqlExecuteStoreCommand`; `kitGraphLive.ts` maps a subset of `ReadKitCommand` to `kitStore { … }` queries; SDL grew to a full selector/Node surface (`compose/graphql/schema.graphql`).
- **In progress:** Schema still uses `*SelectorInput` / `Node` patterns; JS still exposes `executeRead(ReadCommandBatch)` and `readCommandTypes.ts`; React/sketchpad still depend on that batch path indirectly.
- **Verification:** `cargo check` / `npm run build` for js+react+sketchpad are the usual green gates; `cargo test -p compose` may hit **LNK1104** on Windows if the test `.exe` is locked—retry or exclude AV interference.

## Target Shape

- `compose/rs` owns the only kit control plane: `#[wasm_bindgen(js_name = execute)]` accepts a GraphQL request JSON/document and streams GraphQL responses.
- `async-graphql` is added directly to existing store types in `compose/rs/lib.rs`: `KitGraph`, `kit_store::KitStore`, `KitStoreHandle`, `DesignStore`, `PieceStore`, `TypeStore`, `ConnectionStore`, and child stores expose `#[Object]` resolver impls on the existing structs/refs.
- Public GraphQL read traversal uses in-memory store refs, not command IDs. Root starts at the live/current kit store, and nested fields return `Arc<RwLock<T>>`/existing `*StoreRef` wrappers resolved under short locks.
- Mutations enqueue typed command payloads into the WASM actor inbound channel. The actor applies them against the live `KitGraphRef`, then publishes typed events to the outbound subscription channel.
- `compose/js`, `compose/react`, `compose/sketchpad`, and `compose/algorithms` stop using `executeRead`, string commands, `KitStoreCommand` JSON, and local derived data. They call the same GraphQL `execute` boundary.

```mermaid
graph LR
  ReactHooks[compose/react hooks] --> JsClient[compose/js GraphQL client]
  JsClient -->|single execute| WasmExecute[compose/rs wasm execute]
  WasmExecute --> Schema[async-graphql schema]
  Schema -->|Query refs| ExistingStores[existing Arc RwLock stores]
  Schema -->|Mutation enqueue| Inbound[Inbound command queue]
  Inbound --> Actor[WASM kit actor]
  Actor --> ExistingStores
  Actor --> Outbound[Outbound event stream]
  Outbound -->|Subscription| JsClient
```

## Implementation Phases

1. **Replace the Rust control boundary in [compose/rs/lib.rs](compose/rs/lib.rs).**
   - Add `async-graphql`, `async-stream` if needed, and `console_error_panic_hook` for wasm.
   - Introduce one schema module/region inside the existing file, not a new file: schema root objects, `boot()`, `execute(request_json, on_message)`, dual `async_channel` queues, and the single actor spawn.
   - Remove/retire WASM exports that expose JSON command/read control (`executeReadKitCommands`, command-specific helpers) instead of layering GraphQL next to them.

2. **Put `#[Object]` on existing stores and resolve by pointer.**
   - Implement resolver impls on the existing `KitGraph`/`KitStore` and entity stores; expose every stored and computed field as GraphQL fields.
   - Replace selector inputs in [compose/graphql/schema.graphql](compose/graphql/schema.graphql) that currently use `EntityIdInput`/`Node` semantics with traversal-first fields and typed mutation inputs.
   - Where a mutation needs a target, resolve it inside the actor from the already traversed in-memory store path used by the JS store object, not by public entity ID selectors.
   - Keep internal IDs only as persisted entity data where the domain already owns them; they stop being the control mechanism.

3. **Convert commands into complete GraphQL fields.**
   - Map every `Read*Command` variant/output in [compose/rs/read_module.rs](compose/rs/read_module.rs) and [compose/rs/read_impl.rs](compose/rs/read_impl.rs) to a statically typed GraphQL field on the matching store.
   - Map `ChangeKitCommand` and VCS/session/backbone commands into root mutations that only validate/enqueue; heavy work runs in the actor.
   - Port the remaining `executeComposeKitCommand` string handlers from [compose/js/index.ts](compose/js/index.ts) into typed Rust mutations/events, then delete the string dispatcher.

4. **Make [compose/js](compose/js) a GraphQL-only client.**
   - Replace `KitStoreClient.executeRead`, command JSON helpers, `kitGraphLive.ts`, and worker RPC methods with a single `executeGraphql`/stream API over the WASM `execute` function.
   - Rewrite per-entity TS stores to issue typed GraphQL documents/fragments. They may hold typed pointer paths/store handles, but no domain logic, caches, or ID-based command construction.
   - Regenerate or replace [compose/js/readCommandTypes.ts](compose/js/readCommandTypes.ts) with GraphQL operation/result types sourced from [compose/graphql/schema.graphql](compose/graphql/schema.graphql).

5. **Align downstream packages.**
   - [compose/react/index.tsx](compose/react/index.tsx): hooks and Scopes call the JS GraphQL stores only; subscriptions update hook state from `eventStream`.
   - [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx): remove local kit command strings and direct `@semio-tech/compose-js` control imports; use `@semio-tech/compose-react` hooks/Scopes.
   - [compose/algorithms](compose/algorithms): replace any direct command/client calls with the shared JS GraphQL client surface.

6. **Testing and verification.**
   - Extend existing Rust tests in [compose/rs/lib.rs](compose/rs/lib.rs) for GraphQL queries, queued mutations, subscription events, pointer traversal, and computed fields.
   - Extend existing JS/React/sketchpad tests rather than creating new test files: verify operation typing, no `executeRead`/string command imports, hook reads/writes, and event-driven updates.
   - Run `cargo test` in `compose/rs`, wasm tests where available, `pnpm -F @semio-tech/compose-js test`, `pnpm -F @semio-tech/compose-react test`, and relevant sketchpad/algorithm tests.

## Key Risks

- GraphQL object refs must avoid holding write locks across nested resolver awaits. Each resolver should clone child refs under a short read lock, then release before resolving deeper fields.
- Native `kit_store::KitStore` and WASM `KitStoreHandle` currently diverge; the migration should make the actor/schema the shared semantic boundary while native-only backbone I/O remains behind the same mutations.
- The existing SDL is ID/selector-heavy and must be replaced, not patched, to satisfy pointer-only control semantics.
