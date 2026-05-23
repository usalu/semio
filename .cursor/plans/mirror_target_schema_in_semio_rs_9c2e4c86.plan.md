---
name: Mirror Target Schema In semio/rs
overview: Bring `semio/rs`'s generated GraphQL SDL to full structural parity with the extended `semio/graphql/target.schema.graphql` (Relay scaffolding, owner/owned unions, hashing, Modification/Diff/Diffs per entity) and refactor `semio/rs` internals to resolve resources strictly via `Arc`/`Weak` pointers — no `Id` lookups inside resolvers.
todos:
  - id: ticket
    content: Open ticket in repo MCP for this work and record working tree under .repo/🎫/YY/MM/DD/
    status: completed
  - id: foundation
    content: "Phase 1 (sequential): add Node/Entity/WeakEntity/StrongEntity/Artifact/Document/Modification/Diff interfaces, PageInfo, EntityEdge/EntityConnectionInterface, global OwnerEntity/OwnedEntityConnection/ChangeOwned/DiffOwner/DiffsOwner/Input unions, plus entity_relay!/entity_diffs!/entity_owner! macros. Single edit pass on lib.rs."
    status: completed
  - id: pointer-sweep
    content: "Phase 2 (sequential): replace Connector.port_id, Group.piece_ids, Design.piece_id_to_index, Kit.design_id_to_index with Weak/Arc; delete *_by_id from resolvers; keep id→Arc indexes only at command-apply boundary."
    status: completed
  - id: geom-relay
    content: "Phase 3a (parallel subagent): apply entity_relay!+entity_diffs!+entity_owner! to Vector/Point/Coordinate/Offset/Plane/Position/Place/Location; add hash. Lift current SimpleObject geom types to Arc-shared structs."
    status: completed
  - id: meta-relay
    content: "Phase 3b (parallel subagent): same scaffolding + hash for Author/Attribute/Benchmark/Quality/Prop/Tag/Concept/Stat/Layer/Group/File/Folder."
    status: completed
  - id: type-relay
    content: "Phase 3c (parallel subagent): same scaffolding for Type/Port/Connector/Representation; wire owner unions Type→Kit, Connector/Port/Representation→Type."
    status: completed
  - id: design-relay
    content: "Phase 3d (parallel subagent): same scaffolding for Design/Piece/Connection/Side/Clump; wire DesignOwner=Kit, PieceOwner=Blueprint, ConnectionOwner=Design, SideOwner=Connection, ClumpOwner=Design."
    status: completed
  - id: kit-vcs-relay
    content: "Phase 3e (parallel subagent): same scaffolding for Kit/Graph/Session/Draft/Transaction/Checkpoint/Alternative/Change."
    status: completed
  - id: ops-relay
    content: "Phase 3f (parallel subagent): per-entity Operations + per-entity Operation unions per the existing extend_graphql_operations plan; rename CreatedFixedPiece→AddedFixedPieceToDesign, FixedPiece→FixedPieceInDesign, DraggedPiece→DraggedPiecesInDesign; add Mutation + Subscription field per op."
    status: completed
  - id: validate
    content: "Phase 4 (sequential): regenerate semio/graphql/schema.graphql, diff vs target.schema.graphql, fix unmatched types, ensure cargo build passes for native + wasm32, close ticket."
    status: completed
isProject: false
---

## Mirror Target Schema In semio/rs

### Goal
- The SDL emitted by [`crate::gql::sdl`](semio/rs/lib.rs:3762) (written to `semio/graphql/schema.graphql`) MUST match `semio/graphql/target.schema.graphql` structurally: same interfaces, same `<Entity>` + `<Entity>Edge` + `<Entity>Connection` triples, same `<Entity>Modification` + `<Entity>ModificationEdge` + `<Entity>ModificationConnection` triples, same `<Entity>Diff`/`<Entity>Diffs` triples, same owner/owned unions, same `hash: String!` field on every entity, same `entityOwner`/`ownedEntities` fields.
- `semio/rs` MUST resolve every entity relationship through stored `Arc`/`Weak` pointers. No `Id`-keyed slot tables, no `*_by_id` linear scans inside `#[Object]` resolvers, no `port_id: Option<Id>` style child slots. `Id` is allowed only on the entity itself and on inbound command DTOs (translated `Id → Arc` once at command-apply time).

### Target shape (from `target.schema.graphql`)
- 11 interfaces: `Node`, `Entity`, `WeakEntity`, `StrongEntity`, `Artifact`, `Document`, `Modification`, `Diff`, `Operation`, `EntityEdge`, `EntityConnectionInterface`.
- ~653 object types, ~389 unions, 17 inputs, 1 scalar (`Timestamp`) plus existing `Id`.
- Per-entity scaffolding pattern (12 types per concrete entity X):
  - `X` implementing `Entity`/`WeakEntity`/`StrongEntity` with `hash`, `owner: XOwner!`, `entityOwner: OwnerEntity`, `ownedEntities: OwnedEntityConnection`.
  - `XEdge` / `XConnection`.
  - `XModification` / `XModificationEdge` / `XModificationConnection`.
  - `XDiff` / `XDiffEdge` / `XDiffConnection`.
  - `XDiffs` / `XDiffsEdge` / `XDiffsConnection`.
  - Unions: `XOwner`, `XOwned`, `XModificationOwner`, `XModificationOwned`, `XDiffsOwner`, `XDiffsOwned`.

### Concrete entities to lift (extracted from `target.schema.graphql`)
- Geometry: `Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Position`, `Place`, `Location`.
- Meta: `Author`, `Attribute`, `Benchmark`, `Quality`, `Prop`, `Tag`, `Concept`, `Stat`, `Layer`, `Group`, `File`, `Folder`.
- Type tree: `Type`, `Port`, `Connector`, `Representation`.
- Design tree: `Design`, `Piece`, `Connection`, `Side`, `Clump`.
- Kit + VCS: `Kit`, `Graph`, `Session`, `Draft`, `Transaction`, `Checkpoint`, `Alternative`, `Change`.
- Operations (per `[.cursor/plans/extend_graphql_operations_per_entity_ce90680a.plan.md](.cursor/plans/extend_graphql_operations_per_entity_ce90680a.plan.md)`): `CreatedTag/RenamedTag/UpdatedTagDescription/UpdatedTagIcon/AddedAttributeToTag/.../DeletedTags`, mirrored for `Concept`, `Port`, `Quality`, `Type`, `Connector`, `Design`; full `Piece` op set (`AddedFixedPieceToDesign`, `MovedPieceInDesign`, `DraggedPieceInDesign`, `ChangedPieceToTypeInDesign`, etc.); `Kit` ops (`RenamedKit`, `ChangedDescription`).

### Mechanism in Rust (async-graphql 7)
- Interfaces: declare with `#[derive(Interface)]` enum wrappers (e.g. `EntityIface { Tag(Arc<Tag>), Concept(Arc<Concept>), … }`) and add `#[graphql(field(name = "id", type = "Id"), field(name = "hash", type = "String"), field(name = "entityOwner", type = "OwnerEntity"), field(name = "ownedEntities", type = "OwnedEntityConnection"))]`. The same enum doubles as the `EntityConnection` `nodes` element (Relay accepts interface-typed connections).
- Per-entity Edge/Connection: macro `entity_relay!(Tag)` to expand to `TagEdge { cursor, node }`, `TagConnection { edges, pageInfo, hash }`. New file region `//#region 🪢 relay` in `lib.rs` with the macro definition (per CLAUDE.md no new files).
- Modification/Diff/Diffs: macro `entity_diffs!(Tag, fields = [name, description, icon, order])` expanding to `TagModification { name: Option<String>, description: Option<String>, removeDescription: Option<bool>, … }`, `TagDiff { before, modification, after }`, `TagDiffs { removed, diffs, added }` plus their edge/connection. Diff/Modification structs are `WeakEntity` (`id` = blake3 hash of contents, `hash` = same).
- Owner/owned unions: `#[derive(Union)] enum TagOwner { Kit(Arc<Kit>), Type(Arc<Type>), Representation(Arc<Representation>) }`. Global `OwnerEntity` and `OwnedEntityConnection` unions list every concrete entity (auto-generated from the same source list as `entity_relay!`).
- `hash` everywhere: extend the existing [`crate::hash::h`](semio/rs/lib.rs:340) helper. Every `#[Object]` impl gets `async fn hash(&self) -> String { self.compute_hash().await }`. Add `compute_hash` to entities currently missing it (`Concept`, `Tag`, `Quality`, `Author`, `Attribute`, `Benchmark`, `Prop`, `Stat`, `Layer`, `Group`, `File`, `Folder`, `Location`, `Side`, `Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Position`).

### Pointer refactor (strict)
- Replace `Connector.port_id: RwLock<Option<Id>>` ([lib.rs:428](semio/rs/lib.rs:428)) with `port: RwLock<Weak<Port>>`. Resolver `port()` upgrades the `Weak` directly; no parent-type id scan.
- Replace `Group.piece_ids: Vec<Id>` ([lib.rs:328](semio/rs/lib.rs:328)) with `pieces: RwLock<Vec<Weak<Piece>>>` (and add the corresponding GraphQL `pieces: PieceConnection!` resolver).
- Drop `Design.piece_id_to_index: HashMap<Id, usize>` ([lib.rs:1083](semio/rs/lib.rs:1083)) and `Kit.design_id_to_index` ([lib.rs:1323](semio/rs/lib.rs:1323)). Internal traversal uses `pieces: Vec<Arc<Piece>>` directly. Where `O(1)` lookup is needed at command-apply time, keep a `HashMap<Id, Weak<Piece>>` adjacent to the `Vec` and treat it strictly as a write-side index, never read from a resolver.
- Delete `*_by_id` from all `#[Object]` resolvers ([lib.rs:619, 622, 625, 1150, 1156, 1391, 1396](semio/rs/lib.rs:619)). Field args like `design.piece(id: Id!): Piece` keep the external `Id` arg but resolve via the new write-side index (since this is the command boundary, not internal traversal).
- `parent_piece` / `parent_connection` / `child_pieces` / `path` already use `Weak`/`Arc` ([lib.rs:770-775](semio/rs/lib.rs:770)) — keep as is; verify no `Id`-based fallbacks remain.
- Audit list: every `_by_id` and `_id_to_` symbol in `lib.rs` must either be deleted or marked `#[doc(hidden)]` and only callable from `crate::op::*` command-apply paths. Add a `#[deny]` lint comment in the `gql` region forbidding `_by_id` in resolvers.

### Schema region layout in `lib.rs`
Extend the existing `//#region 🌐 gql` ([lib.rs:3467](semio/rs/lib.rs:3467)) with hierarchical sub-regions matching `target.schema.graphql`'s region tree:
- `//#region 🪪 interfaces` — `Node`, `Entity`, `WeakEntity`, `StrongEntity`, `Artifact`, `Document`, `Modification`, `Diff`, `Operation` interface enums + `EntityEdge`, `EntityConnectionInterface`, `PageInfo`.
- `//#region 🌐 unions` — global `EntityOwner`/`OwnerEntity`, `OwnedEntityConnection`, `ChangeOwned`, `DiffOwner`, `DiffsOwner`, `Input`, `AnyOperation`.
- `//#region 🪢 relay-macros` — `entity_relay!`, `entity_diffs!`, `entity_owner!` macros.
- `//#region 📐 geom-relay`, `🏷️ meta-relay`, `🏠 type-relay`, `🏘 design-relay`, `📦 kit-relay`, `🌿 vcs-relay`, `⚙️ op-relay` — invoke macros for each entity group.

### Phasing (delegated)
Because the work spans ~1000 GraphQL types and ~5000 lines of Rust, after the foundation is in place I will fan out to parallel `generalPurpose` subagents (one per entity group), each given the macro contract and the exact target-schema slice they must reproduce. Foundation MUST land before the parallel wave because every per-entity macro depends on `EntityIface`, `OwnerEntity`, `OwnedEntityConnection`, `Modification`, `Diff`, and the `entity_relay!` macros.

### Validation
- After each phase: `cargo check -p semio --target wasm32-unknown-unknown --no-default-features` plus native `cargo build -p semio` to confirm both targets compile.
- After each phase: regenerate `semio/graphql/schema.graphql` via the existing test harness (`crate::gql::sdl`) and `diff` against `semio/graphql/target.schema.graphql` ignoring whitespace/comments. Track the unmatched-type count in the ticket folder under `.repo/🎫/.../diff-report.md` after every wave.
- Final pass: zero unmatched named types between `schema.graphql` and `target.schema.graphql`; zero `*_by_id` calls inside any `#[Object]` impl (verified with `rg "_by_id\(" semio/rs/lib.rs` scoped to resolver blocks).

### Out of scope
- Operation business logic (resolvers can return placeholder shapes or `unimplemented!()` for `before`/`after`/`modification` until a follow-up ticket — but every type MUST exist and compile).
- Backbone persistence changes.
- `semio/js`, `semio/react`, hosts.