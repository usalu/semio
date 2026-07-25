# PRD: Kit Store Architecture (Contracts-First, Multi-Backbone, Pointer-Based RS Core)

## Overview

Replace the current incomplete kit persistence and layering across `compose/asset/compose`, `compose/graphql`, `compose/rs`, `compose/js`, `compose/react`, and `compose/sketchpad` with a single coherent **kit store architecture**:

- **One root kit snapshot per kit graph**; all history is modeled as an **ordered list of semantic kit changes** (operations + inputs).
- **Everything is async** end-to-end in `compose/rs`, including backbone IO and GraphQL resolution paths that touch persistence or replay.
- **Persist only operations + inputs** on backbones (never persist computed kit diffs); **every operation produces a kit diff** in-memory when applied/replayed.
- **Multiple attachable/detachable backbones at runtime**, notably:
  - **Dev backbone**: single JSON file.
  - **Local backbone**: a folder containing `.compose/` with `wip.db`, `staged.db`, `authoritative.db`, `conflicts.db`, and `blobs/BLOBHASH.EXT` content-addressed blobs.
- **`compose/rs` is the in-memory source of truth** for kit graphs while attached; UI reads/writes through **`compose/js` → `compose/react`**, with **`useSyncExternalStore` only for kit state** and **granular invalidation** aligned to RS events (no React-wide blind refetch when a narrower projection updates).
- IPC/schema boundary remains **`compose/graphql/schema.graphql`** (Relay-style extension with owners/children/hashes as already established in the schema direction).

This PRD is **contracts-first**: finalize contracts (assets + GraphQL) before locking Rust internals, then implement RS, then JS/React, then prove it in sketchpad.

## Goals

- Establish an **unambiguous kit versioning/history model**: root snapshot + ordered semantic changes for **checkpoints, drafts, and transactions**, using **one persisted ordered semantic-operation sequence** for all three, distinguished by **metadata and lifecycle**, not by incompatible persistence shapes.
- Provide **two backbone implementations** (dev JSON + local multi-db/blobs) with **attach/detach** runtime switching without leaking backbone details into React.
- Ensure **`compose/graphql` covers snapshots, diffs, connections, operations, and backbone lifecycle** end-to-end as the primary “done” signal.
- Ensure **`compose/js` exposes RS capabilities as conventional OO APIs** using RxJS internally, **without duplicating kit caching/state ownership** in JS.
- Ensure **`compose/react` composes only `compose/js`**, exporting **`[STATE, SETSTATE, STATUS] = use*(…)`** hooks for settable surfaces, and **`useSyncExternalStore`** kit subscriptions that update **only when the subscribed projection changes**.
- Deliver **`compose/sketchpad` as the integration acceptance surface** demonstrating checkpoints/drafts/transactions + backbone attach/detach.

## Quality Gates

These commands must pass for **every** user story:

**JavaScript/TypeScript monorepo gates (repo root):**

- ```bash
  pnpm typecheck && pnpm lint
  ```

````

**Tooling prerequisite (must be true before claiming any story is complete):**
- The repository root MUST define `pnpm typecheck` and `pnpm lint` such that they typecheck and lint **all packages/files touched by the story** (this epic may introduce/adjust root pnpm workspace config + scripts so the above invocation is real and reproducible).

**Rust (`compose/rs`) when Rust code or GraphQL export wiring changes:**
- ```bash
cargo test --manifest-path compose/rs/Cargo.toml
cargo clippy --manifest-path compose/rs/Cargo.toml --all-targets -- -D warnings
````

**GraphQL schema drift gate when the GraphQL contract changes:**

- ```bash
  pnpm exec nx build compose/graphql
  ```

````

**Embedded/unit tests when behavior changes in TS layers (run when the story touches that package):**
- ```bash
pnpm --filter @semio-tech/compose-js test
pnpm --filter @semio-tech/compose-react test
````

**Sketchpad build gate when sketchpad changes:**

- ```bash
  pnpm --filter @semio-tech/compose-sketchpad build
  ```

```

For **UI / sketchpad** stories, also include:
- Verify in browser using the **dev-browser skill** (at least one happy path and one failure/conflict path).

## User Stories

### US-001: Align Kit Asset Contracts With Snapshot + Change History Model

As a contributor, I want the canonical kit asset shapes under `compose/asset/compose` (including patterns exemplified by `compose/asset/compose/metabolism.new.kit.compose.json`) to match the target architecture so that contracts and fixtures are unambiguous for codegen, tests, and GraphQL mapping.

**Acceptance Criteria:**
- [ ] Document and encode the intended kit graph model as **one root snapshot + ordered semantic changes**, consistent with how checkpoints/drafts/transactions reference history (**same ordered operation persistence model**, differing by **metadata/lifecycle**).
- [ ] Update or replace incompatible example assets/schemas in `compose/asset/compose` needed for the new model (**greenfield replacement**, no legacy compatibility requirement).
- [ ] Provide at least **one minimal golden fixture** pair: **( persisted operation log inputs ) → ( expected derived snapshot hash or structural invariant checks )** suitable for automated testing (exact mechanism chosen in RS tests).

### US-002: Finalize `compose/graphql` Contract For Kits, Changes, Diffs, Connections, Operations, And Backbone Lifecycle

As an integrator, I want `compose/graphql/schema.graphql` to fully specify the kit store surface (Relay-style patterns already present: owners/children/hashes) so `compose/rs` can implement a single GraphQL target without ad hoc gaps.

**Acceptance Criteria:**
- [ ] GraphQL supports querying **readable states** broadly (not “one global active kit”): selectors/parameters explicitly identify which readable state/graph projection is queried.
- [ ] GraphQL supports initiating writes against explicitly identified **writable states** (draft/transaction flows), producing **async-command style outcomes** consistent with existing patterns like `Command` / `commandSucceeded` already present in the schema direction.
- [ ] Schema covers **snapshots**, **diffs**, **connections**, **semantic operations**, **ordered change sequences**, and **checkpoint/draft/transaction** linkage consistent with the asset model.
- [ ] Schema includes backbone operations needed for this epic: **attach**, **detach**, and **capability discovery** sufficient for dev JSON vs local `.compose/` backends (without exposing backbone-internal SQL details to clients).

### US-003: Implement `compose/rs` Core Kit Graph Engine (Async, Pointer-Internal, Multi-State)

As a library author, I want `compose/rs` to represent kit graphs internally with **stable pointers/handles** (not ID-string resolution as the primary internal mechanism) and to allow **reads on any readable state** and **writes on any writable state**, so hosts can implement complex parallel editing workflows safely.

**Acceptance Criteria:**
- [ ] Internal graph resolution uses **pointer/handle-based referencing** for hot paths; any externally-facing IDs are translated at GraphQL/IPC boundaries only.
- [ ] Applying a semantic operation computes a **kit diff** deterministically from **operation + input** (diff not persisted).
- [ ] Core APIs are **async** for operations that may touch execution queues, IO readiness, or backbone-mediated persistence scheduling.
- [ ] Remove or rewrite incompatible surfaces aggressively per greenfield rule (**no legacy API parity**).

### US-004: Implement Attachable Backbones — Dev JSON And Local `.compose/` Multi-DB + Blobs

As a user/developer, I want two backbone implementations so kits can be developed locally with a single file while still supporting a realistic on-disk layout for local workspaces.

**Acceptance Criteria:**
- [ ] **Dev backbone** persists and replays **operation + input** records from **one JSON file** (atomic rewrite strategy documented; crash safety expectations documented).
- [ ] **Local backbone** persists under `.compose/` using separate **SQLite** databases: `wip.db`, `staged.db`, `authoritative.db`, `conflicts.db`, plus `blobs/BLOBHASH.EXT` for referenced payloads.
- [ ] Backbones can be **attached/detached at runtime** without restarting sketchpad in dev workflows (detach semantics define what remains cached vs cleared).
- [ ] Replay from persisted operations reproduces the same derived snapshot/diff behavior as in-memory application (**golden test hooked to US-001 fixture**).

### US-005: Implement GraphQL Target In `compose/rs` Matching `compose/graphql/schema.graphql`

As a host integrator, I want `compose/rs` to implement the GraphQL target directly so all IPC semantics remain in schema, not bespoke channels.

**Acceptance Criteria:**
- [ ] `async-graphql` schema/types implement the updated `compose/graphql/schema.graphql` surface required by US-002 for this epic’s scope.
- [ ] Computed fields (**diffs, derived views, caches**) resolve without requiring clients to persist diffs; caching semantics remain explicit (what is memoized, what invalidates).
- [ ] GraphQL export/build wiring remains consistent with `pnpm exec nx build compose/graphql`.

### US-006: Rebuild `compose/js` Facade Over `compose/rs` Using RxJS Without JS-Layer Kit Caching

As an app developer, I want `compose/js` to expose clean OO APIs that hide RS’s internal architecture while preserving correct async behavior.

**Acceptance Criteria:**
- [ ] Public JS API does not introduce a second source of truth for kit graphs (**no kit state machine duplicated in JS** beyond ergonomic wrappers).
- [ ] Uses RxJS internally for eventing/streams where appropriate; external API remains conventional OO ergonomics (methods + objects), not “Rx-first required” for consumers.
- [ ] Extend existing embedded tests in `compose/js` (do not add new test files) to cover new facade behaviors related to subscriptions/progress/error channels as applicable.

### US-007: Rebuild `compose/react` Hooks Over `compose/js` With `useSyncExternalStore` Kit Subscriptions And Tuple Hooks

As a UI developer, I want React bindings that avoid over-fetching and match the repo’s hook conventions.

**Acceptance Criteria:**
- [ ] Hooks consume **only** `compose/js` (no direct RS imports from React code).
- [ ] Settable hooks follow **`[STATE, SETSTATE, STATUS] = use*(…)`** naming/shape where applicable.
- [ ] Kit state uses **`useSyncExternalStore`** wired to **subscribe + snapshot** callbacks provided by `compose/js`.
- [ ] Demonstrably granular updates: reading a narrow projection (example: a single piece “flat plane”) triggers React updates **only** when that projection’s backing RS signal reports an update (test strategy may use deterministic RS emitters + React testing utilities).
- [ ] Extend existing embedded tests in `compose/react` (do not add new test files).

### US-008: Sketchpad Integration Demonstrating End-To-End Store Workflows

As a reviewer, I want `compose/sketchpad` to prove checkpoints/drafts/transactions workflows plus backbone attach/detach against real GraphQL/Relay-shaped usage patterns.

**Acceptance Criteria:**
- [ ] Sketchpad demonstrates creating/navigating **checkpoint/draft/transaction** timelines using ordered semantic changes (UX minimal but real; avoid decorative mocks).
- [ ] Sketchpad demonstrates switching between **dev JSON backbone** and **local `.compose/` backbone** (attach/detach), without violating layering rules.
- [ ] Extend existing sketchpad tests (Playwright embedded tests) rather than introducing new test harness files.
- [ ] Browser verification performed via **dev-browser skill** for at least one happy path + one failure path (e.g., conflict surfaced via `conflicts.db` semantics).

## Functional Requirements

- **FR-1:** The system must persist **only** `{operationKind, inputPayload}` records (plus necessary indexing metadata) on backbones; it must **never** require persisted kit diffs for correctness.
- **FR-2:** Applying any semantic operation must yield a **kit diff** available to readers/query fields defined by GraphQL.
- **FR-3:** The kit graph must maintain exactly **one root snapshot reference per graph**; all mutations must be expressible as appends/replays of ordered semantic changes scoped to checkpoint/draft/transaction containers, with **checkpoints/drafts/transactions sharing the same underlying ordered-operation persistence approach** distinguished by **metadata/lifecycle**.
- **FR-4:** `compose/rs` must support **multiple readable states** concurrently and **writes targeting explicitly selected writable states**.
- **FR-5:** Backbone attach/detach must be safe against ongoing async operations; semantics must define cancellation vs draining vs error propagation.
- **FR-6:** Dev backbone must operate from **one JSON file** containing the persisted operation log (implementation chooses compaction strategy; must be documented).
- **FR-7:** Local backbone must create/use `.compose/` containing **`wip.db`**, **`staged.db`**, **`authoritative.db`**, **`conflicts.db`**, and **`blobs/BLOBHASH.EXT`**.
- **FR-8:** IPC must remain **`compose/graphql/schema.graphql`**-based; no parallel IPC channel is introduced for kit store functionality.
- **FR-9:** `compose/js` must not cache kit graphs independently of `compose/rs` ownership rules; caching belongs in RS’s computed/cache layers only.
- **FR-10:** `compose/react` must use **`useSyncExternalStore`** for kit-derived UI state subscription boundaries aligned with RS granular invalidation signals.
- **FR-11:** Repo root tooling must provide **`pnpm typecheck`** and **`pnpm lint`** that are sufficient to validate each story’s touched surfaces (agents must not “hand wave” gates while those scripts are missing).

## Non-Goals

- Backwards compatibility with pre-refactor kit APIs or interim bridges.
- Supporting multiple technologies mixing (**elements**, **coda**, **compose**) beyond existing repo separation rules.
- Remote/network synchronization protocols, CRDT merges, or multi-user realtime collaboration (unless already explicitly present in schema—otherwise defer).
- Final UX polish pass across sketchpad beyond what’s needed to prove correctness.

## Technical Considerations

- **Contracts-first sequencing:** US-001 → US-002 precede deep RS implementation work to reduce rework.
- **SQLite** is the assumed engine for `*.db` files; keep backbone SQL schemas internal to RS/backbone modules.
- **`async-graphql` dependency already exists in `compose/rs`**; GraphQL implementation should align with its async execution model.
- **Relay-shaped caching**: hashes/owners/children patterns must remain coherent between GraphQL schema and RS resolver caching keys.
- **WASM vs native**: preserve existing crate constraints (`cdylib`/`rlib`) where relevant; GraphQL surface should remain identical across targets where feasible.
- **`wip.db` vs `staged.db`**: treat these as separate durability stages for **pending vs promoted** operational material on the local backbone (exact promotion rules and transactional boundaries are implementation-defined, but must be documented in-module and must preserve **authoritative replay** correctness).

## Success Metrics

- **Primary:** GraphQL supports snapshots/diffs/connections/semantic operations/backbone lifecycle end-to-end with sketchpad exercising realistic flows.
- **Secondary:** RS replay tests demonstrate persisted operation logs reproduce deterministic derived structure/hash invariants from golden fixtures.
- **Secondary:** React tests demonstrate subscription granularity avoids unnecessary rerenders for narrow projections.

## Open Questions

- Exact **promotion pipeline** details between `wip.db` and `staged.db` (batching, atomicity, and how conflicts route into `conflicts.db`)—to be specified in the RS backbone module docs during US-004.
```
