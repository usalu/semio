# Ralph Progress Log

This file tracks progress across iterations. Agents update this file
after each iteration and it's included in prompts for context.

## Codebase Patterns (Study These First)

- **Kit store assets:** Canonical shape is `semio.kit_store.bundle` with `rootSnapshot`, ordered `semanticOpLog`, optional `histories` (checkpoint/draft/transaction metadata over the same op model), and `backbonePointers`. Document the intent in `semio/assets/semio/kit-store.contract.semio.json`; pair `kit-store.golden.ops.semio.json` with `kit-store.golden.expected.semio.json` for RS replay tests (`projectionFingerprint` = blake3-style `hash::h` over sorted piece centers) and lightweight JS fixture parses.
- **Root pnpm for semio slice:** A minimal `pnpm-workspace.yaml` including only `semio/js`, `semio/react`, and `semio/assets` avoids `pnpm install` pulling packages that depend on `file:../rs/pkg` before `wasm-pack build` populates `semio/rs/pkg`.
- **GraphQL SDL source of truth:** Integrators read `semio/graphql/schema.graphql`, but it is **generated** from `semio/rs` (`async_graphql` `Schema::sdl`) via `pnpm exec nx build semio/graphql` (runs the ignored `export_semio_graphql_schema_file` test with `SEMIO_GRAPHQL_SCHEMA_OUT`). Edit the Rust schema, then rebuild—do not hand-edit the SDL long-term.

---

## 2026-05-06 - US-001

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

