# Ralph Progress Log

This file tracks progress across iterations. Agents update this file
after each iteration and it's included in prompts for context.

## Codebase Patterns (Study These First)

- **Kit store assets:** Canonical shape is `compose.kit_store.bundle` with `rootSnapshot`, ordered `semanticOpLog`, optional `histories` (checkpoint/draft/transaction metadata over the same operation model), and `backbonePointers`. Document the intent in `compose/asset/compose/kit-store.contract.compose.json`; pair `kit-store.golden.operations.compose.json` with `kit-store.golden.expected.compose.json` for RS replay tests (`projectionFingerprint` = blake3-style `hash::h` over sorted piece centers) and lightweight JS fixture parses.
- **Root pnpm for compose slice:** A minimal `pnpm-workspace.yaml` including only `compose/js`, `compose/react`, and `compose/asset` avoids `pnpm install` pulling packages that depend on `file:../rs/pkg` before `wasm-pack build` populates `compose/rs/pkg`.
- **GraphQL SDL source of truth:** Integrators read `compose/graphql/schema.graphql`, but it is **generated** from `compose/rs` (`async_graphql` `Schema::sdl`) via `pnpm exec nx build compose/graphql` (runs the ignored `export_compose_graphql_schema_file` test with `COMPOSE_GRAPHQL_SCHEMA_OUT`). Edit the Rust schema, then rebuild—do not hand-edit the SDL long-term.
- **Kit graph engine (RS):** `crate::kit_graph_engine` owns `projection_fingerprint_for_kit` (golden-compatible), `deterministic_semantic_diff`, and async `apply_semantic_op_json`. `Kit`/`Design` use `design_id_to_index` and `piece_id_to_index` for O(1) slot resolve after a single `bind_external_design_id` at the boundary; GraphQL `Graph.projectionFingerprint` delegates to the engine.
- **Attachable backbones (native RS):** `crate::kit_backbone` implements `BackboneStoreKind::DEV_JSON` (single file, `*.tmp.compose-write` + `rename(2)`) and `LOCAL_DOT_COMPOSE` (`.compose/{wip,staged,authoritative,conflicts}.db` + `blobs/`). `worker::ChildRuntime::backbone` replays persisted operations via `apply_semantic_op_json` after `Kit::clear_piece_projections_for_backbone_replay`; `createFixedPiece` appends `{draftId,transactionId,kind,input}`. Wasm attach resolves to `invalid`/`NotSupported` style errors (no SQLite on wasm).
- **GraphQL SDL parity check:** After changing `async_graphql` resolvers or types, export with `COMPOSE_GRAPHQL_SCHEMA_OUT` + ignored `export_compose_graphql_schema_file` test and `diff` the output against `compose/graphql/schema.graphql`; an empty diff means the committed integrator surface matches RS.
- **`@semio-tech/compose-js` kit reads vs Integrator SDL:** Full kit DTO is always `wip.theKit.fullSnapshot` (RS `kit_full_snapshot_value`); granular reads use fields that exist on `Kit` / `Design` / `Piece` in `schema.graphql` (e.g. `design { piece(id:) { flatPosition { plane { xAxis yAxis } } } }`). Hydration accepts **camelCase** plane keys from JS via `#[serde(alias)]` on `Plane`; golden / semantic-operation JSON keeps **snake_case** `x_axis` / `y_axis`.
- **`@semio-tech/compose-react` kit live reads:** `getComposeKitLiveReadStore` and related classes (`ComposeKitViewStore`, `ComposeKitDesignReadStore`, `ComposeKitShallowListReadStore`) live in `compose/js` (🪜ComposeKitLiveReadHub). They pair `KitStoreClient.subscribe` with per-key async fetches and `KitStoreReadSnap`; hooks call `useSyncExternalStore` with the hub’s `subscribe`/`getSnapshot` pattern. Narrow invalidation uses `kitEventTouchesPiece` / `kitEventTouchesDesign` / `kitEventAffects*` filters so e.g. `usePieceFlatPlane` only repolls when the RS-backed event stream says that projection may have changed.

---

## 2026-05-06 - US-005

- **What was implemented:** Confirmed **byte-for-byte SDL parity** between `crate::gql::build_schema().sdl()` and `compose/graphql/schema.graphql` (US-002 kit-store surface + full entity graph). Clarified **caching semantics in Rustdoc** on `Graph.semanticOpLog` / `projectionFingerprint` / `rootSnapshotHash` (no server memo; invalidate on live kit / backbone / replay) and on `operation::Diff` (ephemeral `deterministic_semantic_diff`, not bundle-persisted; clients read via `Operation.diff`). Opened/closed ticket `graphql-target-compose-rs-us-005`.
- **Files changed:** `compose/rs/lib.rs`, `.ralph-tui/progress.md`, `.repo/🎫/26/05/06/graphql-target-compose-rs-us-005/ticket.json`.
- **Learnings:**
  - **Patterns discovered:** Prior US-002–004 already wired the schema export path; US-005 is primarily **verification + explicit compute/memo docs** so integrators do not assume hidden caches on fingerprints or diffs.
  - **Gotchas encountered:** `async_graphql` only exports types **reachable from the schema roots**; Rust-internal unions (e.g. `OperationInput` enums) that are never referenced from `Query`/`Mutation`/`SubscriptionRoot` do not appear in the emitted SDL — avoid assuming every `derive`d GraphQL type shows up in `schema.graphql`.

---

## 2026-05-06 - US-002

- **What was implemented:** Finalized the **kit-store GraphQL contract** in `compose/rs` (exported SDL): `Query.readableKitGraph` + `backboneCapabilities` with `ReadableGraphSelector` (`KitGraphWorkspace` + optional checkpoint/draft/transaction anchors); `Graph.semanticOpLog`, `projectionFingerprint`, `rootSnapshotHash`; lifecycle linkage fields on `Change` / `Checkpoint` / `Transaction` / `Draft`; `BackboneStoreKind`, `backboneAttach` / `backboneDetach`; mutations return `Command` (`requestId` + `kind`) and take `workspace` for wip vs authoritative routing; `Diff.summary`; `SemanticOpRecord` type. Regenerated `compose/graphql/schema.graphql`; documented `graphqlSurface` on `kit-store.contract.compose.json`; root `pnpm typecheck` now runs `nx build compose/graphql`.
- **Files changed:** `compose/rs/lib.rs`, `compose/graphql/schema.graphql`, `compose/asset/compose/kit-store.contract.compose.json`, `package.json`, `.ralph-tui/progress.md`, `.repo/🎫/26/05/06/graphql-kit-store-contract-us-002/ticket.json`.
- **Learnings:**
  - **Patterns discovered:** Object-typed mutation payloads (`Command`) require selection sets in GraphQL documents—integration tests and clients must request `{ requestId kind }`. Enum variables (e.g. `KitGraphWorkspace`) flow through `async_graphql::value!` as string labels (`"WIP"`).
  - **Gotchas encountered:** `target.schema.graphql` remains a separate Relay-style design draft; runtime SDL is only what `gql::sdl()` emits—do not assume parity without an explicit codegen/link step.

---

## 2026-05-06 - US-003

- **What was implemented:** Core **kit graph engine** in `compose/rs`: `crate::kit_graph_engine` with `DesignHandle`, `projection_fingerprint_for_kit` (same algorithm as kit-store golden), `deterministic_semantic_diff` (ephemeral, from operation kind + payload JSON + fp before/after), and async `apply_semantic_op_json` for bundle-shaped replay. `Kit`/`Design` now keep **slot maps** (`design_id_to_index`, `piece_id_to_index`) so hot paths avoid linear Id scans; `bind_external_design_id` is the single translation from external design `Id` to internal handle + `Arc`. `Graph::apply_create_fixed_piece` returns `(piece, diff)` and uses `apply_create_fixed_piece_on_design_node` for pointer-only mutation; GraphQL `projectionFingerprint` calls the engine. `CreatedFixedPiece` events carry computed diffs. Contract JSON documents `kitGraphEngine`.
- **Files changed:** `compose/rs/lib.rs`, `compose/asset/compose/kit-store.contract.compose.json`, `compose/graphql/schema.graphql` (SDL doc comment from resolver), `.ralph-tui/progress.md`, `.repo/🎫/26/05/06/core-kit-graph-engine-us-003/ticket.json`.
- **Learnings:**
  - **Patterns discovered:** Treat **two `Arc<Graph>` instances** (wip vs authoritative) as the multi-state primitive; semantic apply + fp/diff logic stays identical per graph. Deterministic diff should key on **canonical input JSON + fp transition** so replay and live mutation agree without persisting diffs.
  - **Gotchas encountered:** `apply_create_fixed_piece` must **clone** fields passed to the inner node helper when building `CreatedFixedPieceInput` for serde, or Rust move analysis fails; serde field names in golden JSON must match `#[serde(rename)]` on payload DTOs.

---

## 2026-05-06 - US-004

- **What was implemented:** Native **`crate::kit_backbone`** with dev JSON backbone (canonical `semanticOpLog` payload + atomic temp/rename persistence notes) and local **`.compose/`** backbone (SQLite `semantic_op_log` in `wip`/`staged`/`authoritative`/`conflicts` dbs initialized together, **`blobs/`** directory ensured for `HASH.EXT`). **`worker::BackboneNativeCell`** on each async child: **`backboneAttach`/`Detach`** hydrate or drop the persistence handle; **`createFixedPiece`** appends **`createdFixedPiece`** rows while attached; replay runs **`replay_stored_operations`** → clears piece projections then **`apply_semantic_op_json`**. RS tests replay **US-001 golden operations from Dev JSON file and from `wip.db`**. Contract JSON **`attachableBackbones`** block documents atomic rewrite, crash safety, detach semantics, and `.compose` layout. **`@semio-tech/compose-js`** fixture assertion for dev backbone JSON shape.
- **Files changed:** `compose/rs/lib.rs`, `compose/asset/compose/kit-store.contract.compose.json`, `compose/js/index.ts`, `tasks/prd.json`, `.repo/🎫/26/05/06/attachable-backbones-us-004/ticket.json`, `.ralph-tui/progress.md`.
- **Learnings:**
  - **Patterns discovered:** Keep **`apply_semantic_op_json`** as the single replay oracle; persisted rows are **`kind + input`** (plus draft/tx ids) so Dev JSON and SQLite stay aligned. **Attach** should **clear piece projections** before replay to avoid double-applying when reusing a live graph.
  - **Gotchas encountered:** **WASM** builds must not reference `rusqlite`; gate **backbone IO** with `#[cfg(not(target_arch = "wasm32"))]` and return **`ComposeError::invalid(...)`** on attach from wasm workers. **Detach URI** must **match** the mounted URI or integrators could think persistence stopped when it did not.

---

## 2026-05-06 - US-006

- **What was implemented:** `@semio-tech/compose-js` **KitStore** now **awaits** `KitStoreHandle.create` (inline WASM + blob worker) so GraphQL `execute`/`subscribe` bind to a real handle; **read path** uses `materializedLiveJsonForReadScope` for design piece/connection bulk reads (no duplicate kit graph in JS), **`mapPieceRead`** / **`getPiecesMetadata`** query **`flatPosition`** per integrator SDL, and design commands that **are not** on `Design` in SDL return **empty stubs** (cluster / included / replaceable catalog) until RS exposes them. **Embedded test** for **`subscribeFiltered` / `subscribeComposeKitCommandLifecycle`** (RxJS behind conventional unsubscribe). **RS:** `Plane` serde **`alias`** for `xAxis`/`yAxis` hydration from JS; **`kit_full_snapshot_value`** emits **camelCase** `xAxis`/`yAxis` in `plane` for `KitFullDto` Zod round-trip. Regenerated **`compose/graphql/schema.graphql`** (`Kit.fullSnapshot`). Rebuilt **`compose/rs/pkg`** with `wasm-pack`. Fixed metadata read **TypeScript** narrowing for `readKitTypesMetadataCommand` / `readKitDesignsMetadataCommand`.
- **Files changed:** `compose/js/index.ts`, `compose/rs/lib.rs`, `compose/rs/pkg/*`, `compose/graphql/schema.graphql`, `.ralph-tui/progress.md`.
- **Learnings:**
  - **Patterns discovered:** **`KitStoreHandle.create` is Promise-shaped** in wasm-bindgen output—treat as async at every callsite (Vitest inline path and worker `init`). **`fullSnapshot`** is the single full-kit channel; mix targeted `theKit { … }` queries only where the SDL exposes fields.
  - **Gotchas encountered:** **Plane JSON** has two conventions: persisted / golden **snake_case** vs **camelCase** kit DTO / GraphQL field names—use **aliases** on serde and **explicit** snapshot JSON for `plane` keys so both tests and JS parse stay green.

---

## 2026-05-06 - US-007

- **What was implemented:** Moved **`ComposeKitLiveReadStore`** and related **view / design / shallow list** read hubs from `@semio-tech/compose-react` into **`compose/js`** (`🪜ComposeKitLiveReadHub` after `kitStoreFromKitStoreClient`), so **`subscribe` + `getSnapshot`** for async materialized reads are owned by the JS kit layer; React hooks keep **`useSyncExternalStore`** (`useComposeReadSnap`, catalog hooks, `usePieceFlatPlane`, etc.) and **re-export** the hub API from `@semio-tech/compose-react`. Added an embedded test that **`usePieceFlatPlane`** rerenders only the probe for a **piece-targeted** `FlattenInvalidated` event (narrow RS-style emission). Root **`pnpm typecheck`** / **`pnpm lint`** unchanged (still validate `compose/js` + `compose/react` + GraphQL build).
- **Files changed:** `compose/js/index.ts`, `compose/react/index.tsx`, `.ralph-tui/progress.md`.
- **Learnings:**
  - **Patterns discovered:** Centralizing **`getComposeKitLiveReadStore`** in `compose/js` keeps one **WeakMap** hub per `KitStoreClient` and matches the PRD: external store **callbacks** from JS, **`useSyncExternalStore`** in React only.
  - **Gotchas encountered:** **`internalKs`** must be cast via **`unknown`** when tests stub a minimal `piece().readFlatPlane` stand-in for `KitStore`. `pnpm exec nx build compose/graphql` is part of root **typecheck**—run it after schema-touching RS work.

---
