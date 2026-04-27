---
name: Rust Kit Session Graph Store
overview: Refactor `semio/rs` and the GraphQL surface to a clean `KitSession`/`KitGraph`/`KitAlternative`/`KitStore`/`KitCheckpoint`/`KitDraft`/`KitTransaction`/`KitConflict` object graph. Drafts move onto alternatives, the per-client `kit_session::Session` is removed, mutations and reads are reorganized under `Query.session` / `Mutation.session`, and JS/React/sketchpad consumers are updated end-to-end.
todos:
  - id: ticket
    content: Reopen RUST-GRAPH-QL-STORE-DTO-CLEANUP ticket (or open new under r2603) and update its description to cover the session/graph/alternative/store/checkpoint/draft/transaction/conflict refactor.
    status: pending
  - id: domain-model
    content: "Phase 1 — semio/rs/lib.rs: delete kit_session per-client container, move drafts onto KitAlternative, refactor KitReadScope (no sessionId), rebuild KitStoreCommand into Alternative/Draft/Transaction/Checkpoint/Backbone tree, update KitStore/wip_kit/coordinator for stable session id."
    status: pending
  - id: graphql-layer
    content: "Phase 2 — kit_graphql module: implement KitSession/KitGraph/KitAlternative/KitStore/KitCheckpoint/KitDraft/KitTransaction/KitConflict resolvers, build KitSessionMutation tree, drop KitStoreMutation/KitStorePayload/coloredConnectors, regenerate semio/graphql/schema.graphql + local.schema.graphql."
    status: pending
  - id: js-client
    content: "Phase 3 — semio/js/index.ts: rewrite all queries/mutations to session.wip/authorative tree and the nested mutation API; drop sessionId bookkeeping; update generated contract assertions."
    status: pending
  - id: react-sketchpad
    content: "Phase 4 — semio/react/index.tsx + semio/sketchpad/index.tsx + semio/algorithms storybook + any semio/ui consumers: switch hooks to KitSession/KitGraph/KitAlternative/KitStore, remove sessionId state, address drafts by alternativeId only."
    status: pending
  - id: tests-verify
    content: Phase 5 — extend Rust kit_graphql_smoke + end-to-end tests, JS/React vitest, sketchpad typecheck, run cargo + nx, add temporary [DEBUG] logs to confirm a create/finalize cycle, then strip and close ticket with full file list.
    status: pending
isProject: false
---

# Rust Kit Session/Graph/Store Refactor

## Goal & Ticket

Reopen the `Rust GraphQL Store Dto Cleanup` ticket at [.repo/🎫/26/04/27/RUST-GRAPH-QL-STORE-DTO-CLEANUP/ticket.json](.repo/%F0%9F%8E%AB/26/04/27/RUST-GRAPH-QL-STORE-DTO-CLEANUP/ticket.json) and continue under goal `r2603` (verify via `repo://goals` first; open a new ticket only if that goal no longer fits). Work end-to-end in one ticket; close with a summary listing every touched file.

## Target schema (authoritative)

The exposed SDL becomes the user's spec, normalized so every checkpoint reference uses `KitCheckpoint` (no `Checkpoint` alias). Every navigable parent is named `container` and is nullable only when the entity legitimately can be detached.

```graphql
type Query   { session: KitSession! }
type Mutation { session: KitSessionMutation! }
type Subscription { eventStream: KitEvent! }

type KitSession {
  id: ID!
  wip: KitGraph!
  authorative: KitGraph
  conflicts: [KitConflict!]!
}

type KitGraph {
  container: KitSession
  id: ID!
  theKit: KitStore
  alternative(id: KitAlternativeIdDto!): KitAlternative
  alternatives: [KitAlternative!]!
  checkpoint(id: KitCheckpointIdDto!): KitCheckpoint
  checkpoints: [KitCheckpoint!]!
}

type KitAlternative {
  container: KitGraph
  id: ID!
  name: String!
  start: KitCheckpoint!
  checkpoints: [KitCheckpoint!]!
  store: KitStore!
  draft: KitDraft
  transaction: KitTransaction
}

type KitStore {
  container: KitGraph
  checkpoint: KitCheckpoint!
  draft: KitDraft
  transaction: KitTransaction
  full: KitFullDto!
  metadata: KitMetadataDto!
  shallow: KitShallowDto!
  id: ID
  name: String!
  description: String
  # ...all current scalar kit fields...
  design(id: DesignIdDto!): DesignStore
  designs: [DesignStore!]!
  type(id: TypeIdDto!): TypeStore
  types: [TypeStore!]!
  # ...all other store-graph fields kept from current KitStore...
}

type KitCheckpoint   { id: ID!, parent: KitCheckpoint, message, time, authors: [AuthorIdDto!]!, hash, isRelease, changeCount, store: KitStore!, container: KitGraph }
type KitDraft        { id: ID!, alternative: KitAlternative!, parent: KitCheckpoint, transactions: [KitTransaction!]!, openTransaction: KitTransaction, canUndo, canRedo, store: KitStore! }
type KitTransaction  { id: ID!, draft: KitDraft!, state: TransactionState!, changeCount, redoChangeCount, canUndo, canRedo, store: KitStore! }
type KitConflict     { id: ID!, reason, createdAt, wipCheckpoint: KitCheckpoint!, backboneTip: KitCheckpoint }
```

Mutations move under the session entity (object-oriented; no `KitStoreMutation`):

```graphql
type KitSessionMutation {
  createAlternative(input: CreateKitAlternativeInput!): KitAlternative!
  alternative(id: KitAlternativeIdDto!): KitAlternativeMutation!
  checkpoint(id: KitCheckpointIdDto!): KitCheckpointMutation!
  backbone: KitBackboneMutation!
}
type KitAlternativeMutation {
  createDraft(parentCheckpointId: ID): KitDraft!
  draft: KitDraftMutation
  unify(message: String!): KitCheckpoint!
}
type KitDraftMutation {
  finalize(message: String!): KitCheckpoint!
  abort: Boolean!
  undo(count: Int): Boolean!
  redo(count: Int): Boolean!
  startTransaction: KitTransaction!
  transaction: KitTransactionMutation
}
type KitTransactionMutation {
  changeKit(commands: [ChangeKitCommand!]!): Int!
  changeKitWithInverse(commands: [ChangeKitCommand!]!): KitChangeWithInverseDto!
  design(id: DesignIdDto!): DesignMutation!
  finalize: KitCheckpoint!
  abort: Boolean!
  undo(count: Int): Boolean!
  redo(count: Int): Boolean!
}
type KitCheckpointMutation {
  markRelease: Boolean!
  setActive: Boolean!
}
type KitBackboneMutation {
  attach(config: BackboneConfigInput!): BackboneStatus!
  detach: Boolean!
  status: BackboneStatus!
  syncNow: Boolean!
  resolveConflict(id: ID!, strategy: ConflictResolutionInput!): Boolean!
}
```

`Subscription.eventStream: KitEvent!` stays as a sibling.

## Phase 1 — Rust domain model ([semio/rs/lib.rs](semio/rs/lib.rs))

- Delete `pub mod kit_session` entirely (the per-client multi-draft container). Remove every `Session::*` use, `SessionCommand*`, `SessionCommandResult*`, and `Session*` field. The single master process IS the GraphQL session.
- Move drafts to alternatives: extend `kit_alternative::KitAlternative` with `pub draft: Option<crate::kit_draft::Draft>` and remove `KitGraph.sessions`. Add a `the_kit_draft: Option<Draft>` next to `the_kit_head` for drafts on the kit line.
- Trim `kit_draft::Draft`: drop `parent_checkpoint`/`target_alternative` (derived from owning alternative tip / kit head); keep `id`, `before`, `transactions`, `redo_transactions`, `open_transaction`. Add helper `parent_checkpoint(&self, owner: DraftOwner)` if needed.
- Replace `KitReadScope` with a draft/transaction model keyed by alternative (or none for the kit line):
  ```rust
  pub enum KitReadScope {
      TheKit,
      Checkpoint { checkpoint_id: Id },
      Alternative { alternative_id: Id },
      Draft { alternative_id: Option<Id> },
      Transaction { alternative_id: Option<Id>, transaction_id: Id },
  }
  ```
  All `session_id` fields disappear from `KitReadScope*`, `KitStoreCommand*`, `kit_read_scope::resolve_read_graph`, and IO/persistence.
- Rebuild `kit_store_command::KitStoreCommand` as a tree shaped to mutations:
  - `Alternative { id, command: KitAlternativeOp }` with `Create`, `Unify`, `CreateDraft`, `Draft { command }`.
  - `Draft { command: KitDraftOp }` with `StartTransaction`, `Finalize { message }`, `Abort`, `Undo { count }`, `Redo { count }`, `Transaction { id, command }`.
  - `Transaction { command: TransactionOp }` with `ChangeKit { commands }`, `ChangeKitWithInverse { commands }`, `Design { id, command }`, `Finalize`, `Abort`, `Undo`, `Redo`.
  - `Checkpoint { id, command: KitCheckpointOp }` with `MarkRelease`, `SetActive`.
  - `Backbone { command: KitBackboneOp }` with `Attach`, `Detach`, `Status`, `SyncNow`, `ResolveConflict`.
    Drop every `NewSession` / `EndSession` / `ExecuteSessionCommands` variant and result. The native [semio-store](semio/rs/lib.rs) and `KitStoreHandle` execute the new tree.
- Update SQLite/JSON/folder persistence in `pub mod io { json/sqlite/folder/zip }` to stop reading/writing per-client sessions. Migrate alternatives to embed an optional draft blob; bump on-disk schema versioning markers if any. No backwards-compat preserved (greenfield rule from [AGENTS.md](AGENTS.md)).
- Update `kit_store::KitStore` and `kit_coordinator` so `wip_kit` / `backbone_kit_stub` expose `KitGraphRef`s and the master `KitStore` (Rust) exposes a stable `id: Id` for the GraphQL `KitSession.id`.

## Phase 2 — Rust GraphQL layer (`pub mod kit_graphql` in [semio/rs/lib.rs](semio/rs/lib.rs))

- Rename internal wrappers consistently: `KitSessionGraphql`, `KitGraphGraphql`, `KitAlternativeGraphql`, `KitCheckpointGraphql`, `KitDraftGraphql`, `KitTransactionGraphql`, `KitConflictGraphql`. Keep existing `*StoreGraphql` wrappers (`KitStoreGraphql`, `DesignStoreGraphql`, …) but the `KitStoreGraphql` is now a materialized view scoped to a checkpoint/draft/transaction, not the WIP container itself.
- Replace `Query` with `session: KitSession!` resolved from the master `KitStore` Arc held in `GraphQlOverride` / actor context. The current `Query.kit(scope)` is removed entirely.
- Implement `KitGraph` resolvers backed by a `KitGraphRef`. `theKit` returns `Some(KitStore)` materialized at `the_kit_head`, `None` if no head yet. `alternative(id)` and `checkpoint(id)` lookups, plus list resolvers.
- Implement `KitAlternative.store` by materializing at `tip()` (or `start` if empty). Drafts on alts back the new `draft` / `transaction` fields.
- `KitStore` keeps the existing rich resolver set (designs/types/replaceableCatalog/flattenMap/...). Add `checkpoint`, `draft`, `transaction`, `container`. Replace `coloredConnectors` and any leftover \*Row scalars per the standing connector-color refactor at [.cursor/plans/connector_color_store_f823838c.plan.md](.cursor/plans/connector_color_store_f823838c.plan.md) (do this in this slice; the rule "fix unrelated problems if no other ticket is currently covering it" applies and that ticket can be closed by this work).
- Add `KitCheckpointIdDto`, `KitAlternativeIdDto`, `DraftIdDto`, `TransactionIdDto`, `KitConflictIdDto` if missing, mirroring existing `*IdDto` shapes. Keep all DTO names \*Dto.
- Build `Mutation` resolver tree exactly as listed above. Each layer takes the parent ID, dispatches a `KitStoreCommand` through the existing actor/native bridge, returns the touched entity (e.g. `KitTransactionMutation.changeKit` returns the integer applied count, while `finalize` returns the new `KitCheckpoint`). Remove `KitStoreMutation`, `KitStorePayload`, `KitStoreResult`, `KitStoreResultKind`, `KitStoreCommandInput`, and the giant batch dispatcher.
- Subscription unchanged: `event_stream` still streams `KitEvent`.
- Regenerate SDL via `npx nx build semio/graphql` after schema settles; commit both [semio/graphql/schema.graphql](semio/graphql/schema.graphql) and [semio/graphql/local.schema.graphql](semio/graphql/local.schema.graphql).

## Phase 3 — JS client ([semio/js/index.ts](semio/js/index.ts))

- Replace every `Query.kit(scope: ...)` operation with `query Session { session { wip { ...KitGraphFields } authorative { ...KitGraphFields } conflicts { ... } } }` plus targeted fragment queries for alternatives/checkpoints/drafts/transactions/stores.
- Replace `kitStore.batch(...)` mutation send-paths with the nested `mutation { session { alternative(id) { draft { transaction { changeKit(commands: ...) } } } } }` style. Provide a fluent helper API on the JS-side `KitStore` so callers express intent: `store.alternative(id).draft.transaction.changeKit(commands)` etc.
- Drop `sessionId` and per-client session bookkeeping from the JS `KitStore` and from `kitEventAffects*` selectors. Drafts are addressed by `alternativeId` only.
- Update generated GraphQL contract assertions and any vitest snapshots for the new SDL types.

## Phase 4 — React, sketchpad, algorithms

- [semio/react/index.tsx](semio/react/index.tsx): rename hooks (`useKitSession`, `useKitGraph`, `useKitAlternative`, `useKitDraft`, `useKitTransaction`, `useKitStore`) and rewrite the `useSyncExternalStore` paths to subscribe to `session.wip` instead of the old WIP `KitStore`. Remove `useKitSession`-style multi-draft hooks tied to client sessions if any.
- [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx): switch all kit reads/mutations to `wipKitGraph.theKit` / `alternative(id).store` / `alternative(id).draft.transaction`. Remove sessionId state, prompts, and session URL params; keep alternative + draft selection.
- [semio/algorithms/.storybook/stories/kit-store/commandSchema.ts](semio/algorithms/.storybook/stories/kit-store/commandSchema.ts) and any other `semio/algorithms` story rewires to the new mutation tree. Audit `semio/ui` and switch any kit reads that are still on legacy contracts.

## Phase 5 — Tests & verification

- Extend (do not add) Rust tests in [semio/rs/lib.rs](semio/rs/lib.rs):
  - `kit_graphql_smoke` to assert the new SDL shape: `Query.session: KitSession!`, presence of `KitGraph`, `KitAlternative`, `KitStore`, `KitCheckpoint`, `KitDraft`, `KitTransaction`, `KitConflict`, `KitSessionMutation`; absence of `Query.kit`, `KitStoreMutation`, `KitStorePayload`, `KitColoredConnectorDto`, `sessionId` fields.
  - End-to-end: create alternative → draft → transaction → changeKit → finalize → checkpoint, all via `KitStoreCommand` directly and via `Schema::execute` of the new mutation tree.
  - Read-side: navigate `session.wip.alternative(id).store.full`, `session.wip.checkpoint(id).store.metadata`, draft/transaction `store` materialization correctness.
- Run `cargo check -p semio`, `cargo test -p semio`, `cargo test -p semio-store --bin semio-store`, then `npx nx build semio/graphql`, `npx nx test semio-js`, `npx nx test semio-react`, and a focused sketchpad typecheck/test command if available.
- Confirm runtime behaviour with `[DEBUG]`-prefixed logs in the new resolver tree for one create/finalize cycle, then strip the temporary logs before close.

## Delegation

Delegate three independent ~1h slices in parallel after Phase 1 lands; each works on the same files with regions, no `git` mutations:

- Slice A — JS client + algorithms storybook ([semio/js/index.ts](semio/js/index.ts), [semio/algorithms/.storybook/stories/kit-store/commandSchema.ts](semio/algorithms/.storybook/stories/kit-store/commandSchema.ts)).
- Slice B — React adapter + sketchpad UI rewire ([semio/react/index.tsx](semio/react/index.tsx), [semio/sketchpad/index.tsx](semio/sketchpad/index.tsx)).
- Slice C — Rust tests, SDL regeneration, persistence migration sweep ([semio/rs/lib.rs](semio/rs/lib.rs) `io::*`, [semio/graphql/schema.graphql](semio/graphql/schema.graphql), [semio/graphql/local.schema.graphql](semio/graphql/local.schema.graphql)).

Phase 1 + Phase 2 stay with the lead generalist; merging happens with the file-region pattern from [AGENTS.md](AGENTS.md). Close the ticket once `cargo`, `nx`, and runtime smoke logs all confirm the new tree.

## Out of scope (explicitly)

- No backwards-compat shims for the removed `Query.kit(scope)` / `Mutation.kitStore.batch` / `sessionId` paths.
- No new test files; only extend existing ones.
- Domain math in [semio/AGENTS.md](semio/AGENTS.md) is not edited (per workspace rule).
