---
name: kit vcs schema consolidation
overview: "Align the entire monorepo (schemas, rust, store, persistence, client bundles, UI, assets, tests, docs) with the version-controlled kit model: Kit → Alternatives → Checkpoints → (Session → Draft → Transaction → ChangeCommands); Kit owns first-class Families (with Ports) that Types/Designs reference; Artifacts drop parent/variant/view; Kits drop version/release; KitChange becomes forward/inverse command lists; snapshots are parameterized by (alternativeId?, checkpointHash?, sessionId?, draftId?, transactionId?)."
todos:
 - id: canonical_spec
   content: "Rewrite canonical spec: compose/AGENTS.md, compose/SPECS.md, compose/DOCS.md — new Kit/Family/Port/Connector/Type/Design shape + VCS entities"
   status: completed
 - id: canonical_schemas
   content: Update compose/sqlite/schema.sql, compose/graphql/schema.graphql, compose/openapi, compose/jsonschema, compose/rdf, compose/owl to the new entity graph
   status: completed
 - id: rs_entities
   content: "compose/rs/lib.rs: add FamilyStore; reshape PortStore to family-scoped with geometry; slim ConnectorStore to (id, name, port_id, desc, attrs); remove parent/variant/view/isAbstract from Type/Design; remove version/release from Kit"
   status: completed
 - id: rs_vcs
   content: "compose/rs/lib.rs: add alternative/checkpoint/session/draft/transaction modules + stores + Dtos; checkpoint hash (blake3 of parent+changes); snapshot(ref) materialization; remove legacy wasm applyKitDiff/applyDesignDiff/setField etc."
   status: completed
 - id: rs_commands
   content: "compose/rs/lib.rs: add Family/Port commands; remove Type/Design parent/variant/view/isAbstract commands and Connector geometry commands; add Type/Design AddFamilyRef/RemoveFamilyRef/SetFamilies; add Connector::Port"
   status: completed
 - id: rs_io
   content: "compose/rs/lib.rs io::sqlite + io::json: rewrite readers/writers to the new schema including VCS tables and command blobs"
   status: completed
 - id: store_sidecar
   content: "compose/store: update jsonrpc.rs method catalog, AGENTS.md, and tests/rpc.rs to new VCS/snapshot surface"
   status: completed
 - id: graphql_postgres_go
   content: "compose/postgres + compose/graphql resolvers + compose/go: mirror sqlite schema; wire graphql queries/mutations/subscriptions to the sidecar"
   status: pending
 - id: clients_parallel
   content: "Parallel subagents: compose/py, compose/net, compose/rb, compose/js, compose/react, compose/liveblocks — regen DTOs, update store clients, extend existing tests"
   status: pending
 - id: apps_parallel
   content: "Parallel subagents: compose/sketchpad, compose/desktop, compose/vscode, compose/hub, compose/gh, compose/3dm — UI + integrations to the new snapshot-based API"
   status: pending
 - id: assets
   content: Regenerate assets/compose/metabolism/.compose/kit.db and metabolism.zip against the new schema; update compose/examples kits; fix assets/index.ts
   status: pending
 - id: tests
   content: Extend existing test files across every bundle to cover family ops, snapshot(ref), session/draft/transaction, release, alternatives, checkpoint hashing, round-trips
   status: pending
 - id: docs
   content: Update every AGENTS.md listed in section 10 to match the new model
   status: pending
isProject: false
---

# Kit VCS Schema Consolidation

End-to-end alignment. No legacy / backwards compat. Every bundle converges on one schema + one API.

## 1. Canonical model (source of truth)

### 1.1 Entity shape (all ids are uuid-v7, read-only, never change)

```mermaid
erDiagram
  Kit ||--o{ Family : owns
  Kit ||--o{ Type : owns
  Kit ||--o{ Design : owns
  Kit ||--o{ File : owns
  Kit ||--o{ Folder : owns
  Kit ||--o{ Quality : owns
  Kit ||--o{ Author : owns
  Kit ||--o{ Concept : owns
  Kit ||--o{ Tag : owns
  Kit ||--o{ Location : owns
  Kit ||--o{ Alternative : owns
  Kit ||--o{ Checkpoint : owns
  Kit ||--o{ Session : owns
  Family ||--o{ Port : owns
  Port ||--o{ Connector : "is filled by"
  Type ||--o{ Connector : owns
  Type }o--o{ Family : references
  Design }o--o{ Family : references
  Design ||--o{ Piece : owns
  Design ||--o{ Connection : owns
  Design ||--o{ Layer : owns
  Design ||--o{ Group : owns
  Design ||--o{ Stat : owns
  Piece }o--|| Type : references
  Piece }o--o| Design : "references (nested)"
  Connection }o--|| Piece : connected
  Connection }o--|| Piece : connecting
  Connection }o--|| Connector : connected
  Connection }o--|| Connector : connecting
  Alternative ||--o{ Checkpoint : "ordered chain"
  Checkpoint ||--o{ KitChange : contains
  Session ||--o{ Draft : owns
  Draft ||--o{ Transaction : "undo/redo stack"
  Transaction ||--o{ ChangeKitCommand : forward
  Transaction ||--o{ ChangeKitCommand : inverse
```

Key shifts from today:

- **Kit**: drop `version`, `release`. Keep `id`, `name`, `uri`, `remote`, `homepage`, `license`, `icon`, `image`, `preview`, `description`, `createdAt`, `updatedAt`.
- **Family** (NEW, kit-level): `id`, `name`, `description`, `icon`, `ports[]`, `attributes`.
- **Port**: owned by Family (not Type). `id`, `name`, `description`, `icon`, `mandatory`, `t`, `point`, `direction`, `compatibleFamilies[]` (by id), `compatiblePorts[]` (by id), `attributes`.
- **Connector**: owned by Type, references Port by id (like Connection → Piece). Fields: `id`, `name`, `portId` (required — picks which family port is filled), `description`, `attributes`. Geometry now lives on Port.
- **Type**: drop `parent`, `variant`, `isAbstract`; replace with `families: FamilyId[]` (plural, matches metabolism DB). Keep `name`, `representations`, `connectors`, `props`, `stock`, `virtual`, `unit`, `location`, `authors`, `concepts`, `icon`, `image`, `description`, `attributes`, `createdAt`, `updatedAt`.
- **Design**: drop `parent`, `variant`, `view`, `isAbstract`; replace with `families: FamilyId[]`. Keep rest (pieces, connections, layers, groups, stats, etc.).
- **Alternative** (NEW): `id`, `name`, `description`, `branchFromCheckpointId?`, `checkpointIds: CheckpointId[]` (ordered). If `null` / default alternative = main line = "the kit".
- **Checkpoint** (NEW): `id` (content hash of `parentHash + changes`), `parentCheckpointId?`, `changes: KitChange`, `message?`, `time?`, `authorIds[]`, `isRelease: Boolean`, `materializedKitHash?` (set when release).
- **KitChange**: `forward: ChangeKitCommand[]`, `inverse: ChangeKitCommand[]`, `kind: KitChangeKind`, `authorId?`, `time?`.
- **Session** (NEW, persisted): `id`, `kitId`, `clientId`, `openedAt`, `lastSeenAt`, `drafts: Draft[]`.
- **Draft** (NEW, persisted): `id`, `sessionId`, `checkpointId` (base), `alternativeId?`, `transactions: Transaction[]`, `cursor: Int` (undo/redo cursor). Only allowed on tail checkpoint of an alternative or of "the kit".
- **Transaction** (NEW, persisted): `id`, `draftId`, `forward: ChangeKitCommand[]`, `inverse: ChangeKitCommand[]`, `time`, `committed: Boolean`.
- **KitSnapshot** is NOT an entity — it is a **query parameter**: `{ kitId, alternativeId?, checkpointId?, sessionId?, draftId?, transactionId? }` that materializes to a `KitFullDto`.
  - No `checkpointId` → root snapshot (initial kit).
  - No `alternativeId` → "the kit" (main line tail).
  - `sessionId/draftId/transactionId` → include uncommitted work on top of base.

### 1.2 `compose/AGENTS.md` and `compose/SPECS.md`

Rewrite the `SQL`, `Interface` (JSON), `InMemory` (mermaid classDiagram), and `📛️ Entities` sections to mirror 1.1. Add a new `🧬️ Version Control` section with the entities above and the snapshot-query rule. Remove every occurrence of `parent`, `variant`, `view`, `isAbstract`, `version`, `release` on Type/Design/Kit. Move `Port` fields from Type-scope to Family-scope; reshape `Connector` to carry only `id`, `name`, `portId`, `description`, `attributes`.

Update [`compose/DOCS.md`](compose/DOCS.md) to match.

## 2. Canonical schemas

### 2.1 [`compose/sqlite/schema.sql`](compose/sqlite/schema.sql)

- Add `family`, `type_family`, `design_family` tables.
- Rewrite `port`: move from `type` child to `family` child; add columns `name`, `description`, `icon`, `mandatory`, `t`, `point_*`, `direction_*`, `family_id`. Drop `family` TEXT column; drop `port_compatible_family`; add `port_compatible_port`.
- Rewrite `connector`: columns `id`, `ordinal`, `name`, `description`, `port_id` (required), `type_id`. Drop `t/point/direction/mandatory` (now on port).
- Drop columns `type.variant`, `type.parent_id`, `type.is_abstract`; `design.variant`, `design.view`, `design.parent_id`, `design.is_abstract`; `kit.version`; any `release` columns.
- Add `alternative`, `checkpoint`, `kit_change`, `change_kit_command` (serialized), `alternative_checkpoint` (order), `session`, `draft`, `transaction`, `transaction_forward_command`, `transaction_inverse_command` tables.
- `kit_change` stores JSON blobs `forward` and `inverse`; checkpoint `id` is TEXT (hash); `parent_checkpoint_id` nullable for root.
- Remove `compose.release` from `compose_schema` (keep `schema_version`, `engine`, `created_at`).

### 2.2 [`compose/graphql/schema.graphql`](compose/graphql/schema.graphql)

- Remove `parent`, `variant`, `view`, `isAbstract` from `Type`/`Design`. Remove `version`, `release` from `Kit`.
- Add `Family` type with `id`, `name`, `description`, `icon`, `ports`, `attributes`; add `families: [Family!]!` on `Type` and `Design`.
- Move port fields (`point`, `direction`, `t`, `mandatory`, `compatiblePorts`, `compatibleFamilies`) to `Port`; reshape `Connector` to `{ id, name, portId: ID!, description, attributes }`.
- Add `Alternative`, `Checkpoint`, `KitChange`, `ChangeKitCommand` (input union over per-entity command inputs), `ReadKitCommand`, `Session`, `Draft`, `Transaction`.
- Queries: `kitSnapshot(kitId, alternativeId, checkpointId, sessionId, draftId, transactionId): Kit`, `kit: Kit` (= "the kit"), `checkpoint(id)`, `alternative(id)`, `session(id)`, `kitTree(kitId): [Alternative!]!`.
- Mutations: `openSession`, `closeSession`, `openDraft`, `closeDraft`, `beginTransaction`, `commitTransaction`, `abortTransaction`, `executeChangeKitCommands(transactionId, commands): [ChangeKitCommand!]! /* inverse */`, `checkpoint(draftId, message, time, authors): Checkpoint`, `openAlternative(fromCheckpointId, name): Alternative`, `switchAlternative`, `promoteAlternative`, `markAsRelease(checkpointId)`.
- Subscriptions: `kitEvents(kitSnapshotRef)` forwarding `KitEvent` from the rust store.

### 2.3 [`compose/openapi/`](compose/openapi/) and [`compose/jsonschema/`](compose/jsonschema/)

Regenerate/rewrite to mirror the GraphQL/SQL shape. Every entity `X` exposes `XIdDto`, `XInputDto`, `XMetadataDto`, `XShallowDto`, `XFullDto` (as already established in `compose/AGENTS.md`'s `InMemory` section). Add DTOs for `Family`, `Alternative`, `Checkpoint`, `KitChange`, `ChangeKitCommand`, `ReadKitCommand`, `Session`, `Draft`, `Transaction`, `KitSnapshotRef`.

### 2.4 [`compose/rdf/`](compose/rdf/) / [`compose/owl/`](compose/owl/) / [`compose/xmi/`](compose/xmi/) / [`compose/peg/`](compose/peg/) / [`compose/antlr/`](compose/antlr/)

Update ontology + grammar sources to mirror the same entity graph.

## 3. Rust store ([`compose/rs/lib.rs`](compose/rs/lib.rs))

Single-file crate — edit in-place, using existing region structure.

### 3.1 Entity DTOs + stores

- Add `FamilyStore` + `Family{Id,Input,Metadata,Shallow,Full}Dto`; move port ownership: `FamilyStore` owns `Arc<RwLock<PortStore>>[]`.
- Rewrite `PortStore` fields to the new geometry-bearing shape; keep `Weak<FamilyStore>` back-ref.
- Rewrite `ConnectorStore`: drop geometry, add required `Weak<PortStore>` reference via `port_id`.
- Rewrite `TypeStore`: remove `parent_id`, `variant`, `is_abstract`; add `families: Vec<FamilyIdDto>` (weak refs resolved via `KitStore`).
- Rewrite `DesignStore`: remove `parent_id`, `variant`, `view`, `is_abstract`; add `families: Vec<FamilyIdDto>`.
- Rewrite `KitStore`: remove `version`, `release`; add child collection `families: Vec<Arc<RwLock<FamilyStore>>>`, plus VCS-owned children (see 3.2).

### 3.2 VCS entities

Add `pub mod alternative`, `pub mod checkpoint`, `pub mod session`, `pub mod draft`, `pub mod transaction`. Each with its own store + `*Dto` family. All persisted.

- `CheckpointStore { id: Hash, parent: Option<Hash>, changes: KitChange, message, time, authors, is_release, materialized_kit_hash? }`.
- `AlternativeStore { id, name, branch_from: Option<Hash>, checkpoint_ids: Vec<Hash> }`. Default alternative id = `ALTERNATIVE_MAIN`.
- `SessionStore { id, kit_id, client_id, opened_at, last_seen_at, drafts: Vec<Arc<DraftStore>> }`.
- `DraftStore { id, session: Weak<SessionStore>, checkpoint_id, alternative_id?, transactions: Vec<Arc<TransactionStore>>, cursor: usize }`.
- `TransactionStore { id, draft: Weak<DraftStore>, forward: Vec<ChangeKitCommand>, inverse: Vec<ChangeKitCommand>, time, committed: bool }`.

### 3.3 Commands

- `ChangeKitCommand` (existing): add variants for Family: `AddFamily`, `RemoveFamily`, `ChangeFamilyCommands { id, commands }`; `ChangeFamilyCommand` with `Name`, `Description`, `Icon`, `AddPort`, `RemovePort`, `ChangePortCommands`.
- Remove `ChangeTypeCommand::{Parent, Variant, IsAbstract}` / `ChangeDesignCommand::{Parent, Variant, View, IsAbstract}`. Add `ChangeTypeCommand::{AddFamilyRef, RemoveFamilyRef, SetFamilies}` and same on `ChangeDesignCommand`.
- Remove `ChangeConnectorCommand::{T, Point, Direction, Mandatory}` (moved to Port); add `ChangeConnectorCommand::Port { value: PortIdDto }`.
- Add `ChangePortCommand::{Name, Description, Icon, Mandatory, T, Point, Direction, CompatiblePorts, CompatibleFamilies, AddAttribute, RemoveAttribute, ChangeAttributeCommands}`.
- Remove `ChangeKitCommand::{Version, Release}`. Keep `Name, Description, Icon, Image, Preview, Remote, Homepage, License, Uri, Created, Updated`.

### 3.4 KitChange / materialization

- Keep `KitChange` as already reshaped in [commands return diffs plan](.cursor/plans/commands_return_diffs,_central_apply_08e68b1c.plan.md).
- Add `Checkpoint::compute_hash(parent: Option<Hash>, changes: &KitChange) -> Hash` (content-addressable, blake3 over canonical JSON).
- `KitStore::materialize(initial: &KitFullDto, checkpoints: &[&CheckpointStore]) -> KitFullDto`: apply each checkpoint's `forward` command-list starting from `initial`.
- `KitStore::the_kit()` = materialize from root over default (main) alternative tail.
- `KitStore::snapshot(ref: KitSnapshotRef) -> KitFullDto`: resolve (alt?, checkpoint?, session?, draft?, transaction?) and materialize accordingly; apply pending transaction inverses if still open.

### 3.5 `pub mod wasm`

Expose `executeChangeKitCommands`, `executeReadKitCommands`, `snapshot(ref)`, `theKit()`, `openSession/closeSession/openDraft/closeDraft/beginTx/commitTx/abortTx/undo/redo`, `checkpoint`, `openAlternative/switchAlternative/promoteAlternative`, `markAsRelease`, and `kitTree()`. All others (legacy `applyKitDiff`, `applyDesignDiff`, `setField`, `addChild`, `removeChild`) are removed.

## 4. Store sidecar ([`compose/store/`](compose/store/))

Update [`compose/store/jsonrpc.rs`](compose/store/jsonrpc.rs) and [`compose/store/AGENTS.md`](compose/store/AGENTS.md) method catalog to match 3.5 verbatim (same camelCase JS names). Event notification payload remains `KitEvent` from [`compose/rs/lib.rs`](compose/rs/lib.rs). Persist sessions/drafts/transactions through the existing sqlite io path (3.6).

## 5. Persistence (rust io + postgres + graphql resolvers)

### 5.1 [`compose/rs/lib.rs`](compose/rs/lib.rs) `pub mod io::sqlite`

Rewrite the readers/writers to match the new `compose/sqlite/schema.sql`. Include checkpoints, alternatives, sessions, drafts, transactions, and command blobs. Materialized kit is not stored unless `isRelease = true` (then in the `materialized_kit` table keyed by checkpoint hash).

### 5.2 [`compose/postgres/`](compose/postgres/) and [`compose/graphql/`](compose/graphql/)

Mirror the sqlite schema in postgres. GraphQL resolvers delegate to the compose-store sidecar (same method catalog as 4.).

### 5.3 [`compose/go/`](compose/go/)

Update go graphql server / REST surface to the new schema. Re-generate any schema-derived code.

## 6. Client bundles

Every client re-generates its DTOs from the canonical schema and routes mutations through `compose-store` (or the wasm `KitStoreHandle` in-browser).

- [`compose/py/`](compose/py/): regenerate DTOs; update `StoreClient` to new method names; update tests in-place (no new test files).
- [`compose/net/`](compose/net/): same; update `StoreClient`, `StoreKitIO`, `KitInPlaceDiff`. Remove Kit.version/Release.
- [`compose/rb/`](compose/rb/), [`compose/go/`](compose/go/): regenerate.
- [`compose/js/`](compose/js/) (see [`compose/js/index.ts`](compose/js/index.ts)): regenerate TS types from graphql/jsonschema; update the `Kit`, `Type`, `Design`, `Port`, `Connector`, `Family`, snapshot + VCS types.
- [`compose/react/`](compose/react/) (see [`compose/react/index.tsx`](compose/react/index.tsx)): update components and hooks that consumed `variant`/`view`/`parent`/`version`/`release` or type-owned ports.
- [`compose/liveblocks/`](compose/liveblocks/): update sync model to include alternative/checkpoint/session/draft/transaction ids.

## 7. UI + apps

- [`compose/sketchpad/index.tsx`](compose/sketchpad/index.tsx) and [`compose/sketchpad/`](compose/sketchpad/): replace variant/view selectors with family-set pickers; wire session/draft/transaction lifecycle to the store sidecar; materialized kit view uses `snapshot(ref)`.
- [`compose/desktop/`](compose/desktop/), [`compose/vscode/`](compose/vscode/), [`compose/hub/`](compose/hub/): same schema updates.
- [`compose/gh/`](compose/gh/) (Grasshopper): update `Compose/Store/*` bindings; Kit LoadKit/SaveKit components use the new snapshot-based API. Drop any version/release parameters.
- [`compose/3dm/`](compose/3dm/): reshape Rhino import/export to the new entity graph.
- [`elements/ui/`](elements/ui/) and [`compose/ui/`](compose/ui/): update generic UI atoms only if they referenced removed fields.

## 8. Assets + examples

- Regenerate [`assets/compose/metabolism/.compose/kit.db`](assets/compose/metabolism/.compose/kit.db) against the new [`compose/sqlite/schema.sql`](compose/sqlite/schema.sql). Remove all `variant`/`view`/`parent`/`version`/`release` columns.
- Regenerate [`assets/compose/metabolism.zip`](assets/compose/metabolism.zip) from the folder.
- Update [`compose/examples/`](compose/examples/) kits to the new schema.
- [`assets/index.ts`](assets/index.ts): update any exported asset helpers that assumed old fields.

## 9. Tests (extend existing files only — no new test files)

- [`compose/rs/tests/`](compose/rs/tests/): add cases for family add/remove/rename, connector-port binding, snapshot(ref) materialization, session/draft/transaction lifecycle, release marking, alternative branch/promote, checkpoint hash stability, forward+inverse round-trips including family commands.
- [`compose/store/tests/rpc.rs`](compose/store/tests/rpc.rs): VCS method coverage end-to-end over NDJSON.
- [`compose/py/`](compose/py/), [`compose/net/Compose.Tests/`](compose/net/Compose.Tests/), [`compose/js/`](compose/js/), [`compose/react/`](compose/react/), [`compose/sketchpad/`](compose/sketchpad/): extend existing test files to cover new DTO shapes + VCS flows.
- [`compose/algorithms/.storybook/stories/kit-store/`](compose/algorithms/.storybook/stories/kit-store/): the Kit/Store story dropdowns regenerate from the updated `commandSchema.ts`.

## 10. Docs + agent rules

- [`compose/AGENTS.md`](compose/AGENTS.md), [`compose/SPECS.md`](compose/SPECS.md), [`compose/DOCS.md`](compose/DOCS.md): rewritten to the new entity model (sections 1.1-1.2).
- [`compose/rs/AGENTS.md`](compose/rs/AGENTS.md), [`compose/store/AGENTS.md`](compose/store/AGENTS.md), [`compose/sqlite/AGENTS.md`](compose/sqlite/AGENTS.md), [`compose/graphql/AGENTS.md`](compose/graphql/AGENTS.md), [`compose/py/AGENTS.md`](compose/py/AGENTS.md), [`compose/net/AGENTS.md`](compose/net/AGENTS.md), [`compose/gh/AGENTS.md`](compose/gh/AGENTS.md), [`compose/js/AGENTS.md`](compose/js/AGENTS.md), [`compose/react/AGENTS.md`](compose/react/AGENTS.md), [`compose/sketchpad/AGENTS.md`](compose/sketchpad/AGENTS.md), [`assets/AGENTS.md`](assets/AGENTS.md): update each `Systems / Mechanisms / Entities` sections.
- [`.repo/💬️/ueli.md`](.repo/💬️/ueli.md): leave untouched (it is the spec source).

## Execution order (to keep the tree compilable)

1. Section 1-2: canonical shape lands in [`compose/AGENTS.md`](compose/AGENTS.md), [`compose/sqlite/schema.sql`](compose/sqlite/schema.sql), [`compose/graphql/schema.graphql`](compose/graphql/schema.graphql), [`compose/openapi/`](compose/openapi/), [`compose/jsonschema/`](compose/jsonschema/).
2. Section 3: [`compose/rs/lib.rs`](compose/rs/lib.rs) stores + commands + VCS + wasm + io::sqlite in one pass (single crate, must compile atomically).
3. Section 4: [`compose/store/jsonrpc.rs`](compose/store/jsonrpc.rs) method catalog + tests green.
4. Section 5.2-5.3: postgres + graphql resolvers + go green.
5. Section 6: clients — py, net, rb, js, react, liveblocks — in parallel subagents (each takes ~1h).
6. Section 7: sketchpad, desktop, vscode, hub, gh, 3dm — in parallel subagents.
7. Section 8: regenerate metabolism asset via the new sqlite writer.
8. Section 9: tests green in each bundle.
9. Section 10: docs.

Delegate sections 6 and 7 to parallel generalist subagents (one per bundle) because each is roughly independent once the canonical schema + rust store + sidecar are locked.
