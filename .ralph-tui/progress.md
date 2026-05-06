# Ralph Progress Log

This file tracks progress across iterations. Agents update this file
after each iteration and it's included in prompts for context.

## Codebase Patterns (Study These First)

- **Kit store assets:** Canonical shape is `semio.kit_store.bundle` with `rootSnapshot`, ordered `semanticOpLog`, optional `histories` (checkpoint/draft/transaction metadata over the same op model), and `backbonePointers`. Document the intent in `semio/assets/semio/kit-store.contract.semio.json`; pair `kit-store.golden.ops.semio.json` with `kit-store.golden.expected.semio.json` for RS replay tests (`projectionFingerprint` = blake3-style `hash::h` over sorted piece centers) and lightweight JS fixture parses.
- **Root pnpm for semio slice:** A minimal `pnpm-workspace.yaml` including only `semio/js`, `semio/react`, and `semio/assets` avoids `pnpm install` pulling packages that depend on `file:../rs/pkg` before `wasm-pack build` populates `semio/rs/pkg`.
- **GraphQL SDL source of truth:** Integrators read `semio/graphql/schema.graphql`, but it is **generated** from `semio/rs` (`async_graphql` `Schema::sdl`) via `pnpm exec nx build semio/graphql` (runs the ignored `export_semio_graphql_schema_file` test with `SEMIO_GRAPHQL_SCHEMA_OUT`). Edit the Rust schema, then rebuild—do not hand-edit the SDL long-term.
- **Kit graph engine (RS):** `crate::kit_graph_engine` owns `projection_fingerprint_for_kit` (golden-compatible), `deterministic_semantic_diff`, and async `apply_semantic_op_json`. `Kit`/`Design` use `design_id_to_index` and `piece_id_to_index` for O(1) slot resolve after a single `bind_external_design_id` at the boundary; GraphQL `Graph.projectionFingerprint` delegates to the engine.
- **Attachable backbones (native RS):** `crate::kit_backbone` implements `BackboneStoreKind::DEV_JSON` (single file, `*.tmp.semio-write` + `rename(2)`) and `LOCAL_DOT_SEMIO` (`.semio/{wip,staged,authoritative,conflicts}.db` + `blobs/`). `worker::ChildRuntime::backbone` replays persisted ops via `apply_semantic_op_json` after `Kit::clear_piece_projections_for_backbone_replay`; `createFixedPiece` appends `{draftId,transactionId,kind,input}`. Wasm attach resolves to `invalid`/`NotSupported` style errors (no SQLite on wasm).
- **GraphQL SDL parity check:** After changing `async_graphql` resolvers or types, export with `SEMIO_GRAPHQL_SCHEMA_OUT` + ignored `export_semio_graphql_schema_file` test and `diff` the output against `semio/graphql/schema.graphql`; an empty diff means the committed integrator surface matches RS.

---

## 2026-05-06 - US-005

- **What was implemented:** Confirmed **byte-for-byte SDL parity** between `crate::gql::build_schema().sdl()` and `semio/graphql/schema.graphql` (US-002 kit-store surface + full entity graph). Clarified **caching semantics in Rustdoc** on `Graph.semanticOpLog` / `projectionFingerprint` / `rootSnapshotHash` (no server memo; invalidate on live kit / backbone / replay) and on `op::Diff` (ephemeral `deterministic_semantic_diff`, not bundle-persisted; clients read via `Operation.diff`). Opened/closed ticket `graphql-target-semio-rs-us-005`.
- **Files changed:** `semio/rs/lib.rs`, `.ralph-tui/progress.md`, `.repo/🎫/26/05/06/graphql-target-semio-rs-us-005/ticket.json`.
- **Learnings:**
  - **Patterns discovered:** Prior US-002–004 already wired the schema export path; US-005 is primarily **verification + explicit compute/memo docs** so integrators do not assume hidden caches on fingerprints or diffs.
  - **Gotchas encountered:** `async_graphql` only exports types **reachable from the schema roots**; Rust-internal unions (e.g. `OperationInput` enums) that are never referenced from `Query`/`Mutation`/`SubscriptionRoot` do not appear in the emitted SDL — avoid assuming every `derive`d GraphQL type shows up in `schema.graphql`.
---


- **What was implemented:** Kit asset contracts aligned to **one root snapshot + ordered semantic ops** with checkpoint/draft/transaction wrappers documented in JSON; golden ops/expected pair; `metabolism.new.kit.semio.json` replaced with a minimal bundle exemplar; RS tests replay golden ops and assert invariants/fingerprint; `@semio/js` embedded tests load golden + bundle paths for structural checks; root `pnpm typecheck` / `pnpm lint` validate the touched packages.
- **Files changed:** `semio/assets/semio/kit-store.contract.semio.json`, `kit-store.golden.*.semio.json`, `metabolism.new.kit.semio.json`, `semio/rs/lib.rs`, `semio/js/index.ts`, root `package.json`, `pnpm-workspace.yaml`, `.npmrc`, `eslint.config.mjs`, plus prior workspace/JS fixes from this epic (see git status for full set).
- **Learnings:**
  - **Patterns discovered:** Same ordered op log underlies snapshot projection and history wrappers—difference is metadata/lifecycle, not a second persistence shape. Golden fixtures should encode **invariants** (`sortedPieceCenters`, counts) plus a stable **fingerprint** for deterministic CI.
  - **Gotchas encountered:** Full pnpm workspace that includes `semio/algorithms` breaks install until `semio/rs/pkg` exists; narrow the workspace or document wasm-pack as a prereq. Legacy `KitStoreHandle` / `eventStream` GraphQL expectations in JS need a follow-up (e.g. US-006) rather than half-wiring old APIs.
---

## 2026-05-06 - US-002

- **What was implemented:** Finalized the **kit-store GraphQL contract** in `semio/rs` (exported SDL): `Query.readableKitGraph` + `backboneCapabilities` with `ReadableGraphSelector` (`KitGraphWorkspace` + optional checkpoint/draft/transaction anchors); `Graph.semanticOpLog`, `projectionFingerprint`, `rootSnapshotHash`; lifecycle linkage fields on `Change` / `Checkpoint` / `Transaction` / `Draft`; `BackboneStoreKind`, `backboneAttach` / `backboneDetach`; mutations return `Command` (`requestId` + `kind`) and take `workspace` for wip vs authoritative routing; `Diff.summary`; `SemanticOpRecord` type. Regenerated `semio/graphql/schema.graphql`; documented `graphqlSurface` on `kit-store.contract.semio.json`; root `pnpm typecheck` now runs `nx build semio/graphql`.
- **Files changed:** `semio/rs/lib.rs`, `semio/graphql/schema.graphql`, `semio/assets/semio/kit-store.contract.semio.json`, `package.json`, `.ralph-tui/progress.md`, `.repo/🎫/26/05/06/graphql-kit-store-contract-us-002/ticket.json`.
- **Learnings:**
  - **Patterns discovered:** Object-typed mutation payloads (`Command`) require selection sets in GraphQL documents—integration tests and clients must request `{ requestId kind }`. Enum variables (e.g. `KitGraphWorkspace`) flow through `async_graphql::value!` as string labels (`"WIP"`).
  - **Gotchas encountered:** `target.schema.graphql` remains a separate Relay-style design draft; runtime SDL is only what `gql::sdl()` emits—do not assume parity without an explicit codegen/link step.
---

## 2026-05-06 - US-003

- **What was implemented:** Core **kit graph engine** in `semio/rs`: `crate::kit_graph_engine` with `DesignHandle`, `projection_fingerprint_for_kit` (same algorithm as kit-store golden), `deterministic_semantic_diff` (ephemeral, from op kind + payload JSON + fp before/after), and async `apply_semantic_op_json` for bundle-shaped replay. `Kit`/`Design` now keep **slot maps** (`design_id_to_index`, `piece_id_to_index`) so hot paths avoid linear Id scans; `bind_external_design_id` is the single translation from external design `Id` to internal handle + `Arc`. `Graph::apply_create_fixed_piece` returns `(piece, diff)` and uses `apply_create_fixed_piece_on_design_node` for pointer-only mutation; GraphQL `projectionFingerprint` calls the engine. `CreatedFixedPiece` events carry computed diffs. Contract JSON documents `kitGraphEngine`.
- **Files changed:** `semio/rs/lib.rs`, `semio/assets/semio/kit-store.contract.semio.json`, `semio/graphql/schema.graphql` (SDL doc comment from resolver), `.ralph-tui/progress.md`, `.repo/🎫/26/05/06/core-kit-graph-engine-us-003/ticket.json`.
- **Learnings:**
  - **Patterns discovered:** Treat **two `Arc<Graph>` instances** (wip vs authoritative) as the multi-state primitive; semantic apply + fp/diff logic stays identical per graph. Deterministic diff should key on **canonical input JSON + fp transition** so replay and live mutation agree without persisting diffs.
  - **Gotchas encountered:** `apply_create_fixed_piece` must **clone** fields passed to the inner node helper when building `CreatedFixedPieceInput` for serde, or Rust move analysis fails; serde field names in golden JSON must match `#[serde(rename)]` on payload DTOs.
---

## 2026-05-06 - US-004

- **What was implemented:** Native **`crate::kit_backbone`** with dev JSON backbone (canonical `semanticOpLog` payload + atomic temp/rename persistence notes) and local **`.semio/`** backbone (SQLite `semantic_op_log` in `wip`/`staged`/`authoritative`/`conflicts` dbs initialized together, **`blobs/`** directory ensured for `HASH.EXT`). **`worker::BackboneNativeCell`** on each async child: **`backboneAttach`/`Detach`** hydrate or drop the persistence handle; **`createFixedPiece`** appends **`createdFixedPiece`** rows while attached; replay runs **`replay_stored_ops`** → clears piece projections then **`apply_semantic_op_json`**. RS tests replay **US-001 golden ops from Dev JSON file and from `wip.db`**. Contract JSON **`attachableBackbones`** block documents atomic rewrite, crash safety, detach semantics, and `.semio` layout. **`@semio/js`** fixture assertion for dev backbone JSON shape.
- **Files changed:** `semio/rs/lib.rs`, `semio/assets/semio/kit-store.contract.semio.json`, `semio/js/index.ts`, `tasks/prd.json`, `.repo/🎫/26/05/06/attachable-backbones-us-004/ticket.json`, `.ralph-tui/progress.md`.
- **Learnings:**
  - **Patterns discovered:** Keep **`apply_semantic_op_json`** as the single replay oracle; persisted rows are **`kind + input`** (plus draft/tx ids) so Dev JSON and SQLite stay aligned. **Attach** should **clear piece projections** before replay to avoid double-applying when reusing a live graph.
  - **Gotchas encountered:** **WASM** builds must not reference `rusqlite`; gate **backbone IO** with `#[cfg(not(target_arch = "wasm32"))]` and return **`SemioError::invalid(...)`** on attach from wasm workers. **Detach URI** must **match** the mounted URI or integrators could think persistence stopped when it did not.
---

