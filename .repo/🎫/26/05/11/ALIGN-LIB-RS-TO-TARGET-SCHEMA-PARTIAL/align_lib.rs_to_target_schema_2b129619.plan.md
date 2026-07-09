---
name: align lib.rs to target schema
overview: Refactor [compose/rs/lib.rs](compose/rs/lib.rs) so that every type, interface, edge and connection declared in [compose/graphql/target.schema.graphql](compose/graphql/target.schema.graphql) exists as a hand-written Rust type with `async-graphql` derives — no build.rs codegen, no embedded schema strings, no schema parsing at runtime — while preserving all existing `kit`, `vcs`, `event`, `worker`, `wasm_bridge` and `kit_backbone` logic.
todos: []
isProject: false
---

# Align `compose/rs/lib.rs` to `target.schema.graphql`

The target schema is the spec, never a runtime input. Every concrete type, interface enum, edge, connection and input declared in [compose/graphql/target.schema.graphql](compose/graphql/target.schema.graphql) MUST exist as a hand-written Rust type in [compose/rs/lib.rs](compose/rs/lib.rs) using `async-graphql`'s `#[derive(Object)]` / `#[derive(Interface)]` / `#[derive(SimpleObject)]` / `#[derive(Union)]` / `#[derive(InputObject)]`. The existing in-memory `Arc + RwLock` style is preserved.

## Ground rules

- No `build.rs`, no `include_str!("../graphql/target.schema.graphql")`, no `Schema::parse_sdl(...)`, no `paste!` or `macro_rules!` for the new types — every struct, every field, every interface arm is typed by hand in the file.
- Single-file: keep all changes inside [compose/rs/lib.rs](compose/rs/lib.rs) under the existing `//#region` taxonomy. Add new regions only when an entirely new bundle (e.g. `diff`, `modification`, `modifications`, `entity`) is introduced.
- Preserve every existing public symbol used by `kit_graph_engine`, `kit_backbone`, `worker`, `gql`, `wasm_bridge`. Renames must be propagated to all call sites in the same file.
- Banner taxonomy from the cleanup plan applies to the Rust definitions too: every `#[graphql(field(...))]` listed on an interface enum must follow `# Node` -> `# Entity` -> ... -> `# <ConcreteName>` order so the generated SDL matches the target.
- Tests in the `🧪 tests` region MUST keep passing (`bun scripts/export-schema.ts` is still the gate). When a name changes (`Transaction` -> `Edit`, etc.) tests are updated in the same region.

## 1. Vocabulary rename pass (in `vcs`, `operation`, `gql`, `worker`, `wasm_bridge`)

Old name -> new name (every reference in the same file is updated):

- `pub struct Transaction` -> `pub struct Edit` (region `💼 transaction` -> `💼 edit`); fields stay (`owner_draft` -> `owner_alternative_wip` rolled into `Alternative` state in step 2).
- `pub struct Draft` -> removed; its state collapses into `Alternative` (each `Alternative` already represents a working lane in the target schema; `Draft` was the WIP buffer).
- `pub enum ChangeOwnerRef` and `pub enum ChangeOwnerUnion` -> removed. `Change.owner` resolves to a new `crate::iface::ChangeOwner` interface arm (Alternative | Checkpoint), per [compose/graphql/target.schema.graphql](compose/graphql/target.schema.graphql) line 7893.
- `pub struct OperationOwner` (in `operation`) -> removed. `Operation.owner` resolves through the new `EntityIface` enum where the only legal arm for `Operation` is `Arc<Edit>` (per `Operation.owner # reference // Edit`).
- All `#[graphql(name = "Transaction")]` / `name = "Draft"` / `name = "ChangeOwner"` / `name = "OperationOwner"` annotations are removed; the new types use the canonical names.
- `pub struct CommandReceipt { #[graphql(name = "Command")] }` keeps the receipt object; the GraphQL exposed `Command*` API surface is unchanged.

## 2. New shared interfaces (new region `🧷 interfaces`)

Replace the small `iface` module with one full set of `#[derive(Interface)]` enums whose variants are `Arc<Concrete>` for every concrete subtype declared in the schema. Banner-mapped fields exactly match section 0 of the cleanup plan.

- `NodeIface { id }`
- `EntityIface : NodeIface { hash, owner: Option<Arc<EntityIface>>, owns: OwnedEntityConnection }`
- `WeakEntityIface : EntityIface` (no extra fields; banner only)
- `StrongEntityIface : EntityIface` (no extra fields; banner only)
- `RichStrongEntityIface : StrongEntityIface { name, description, icon, createdAt, createdBy }`
- `ArtifactIface : RichStrongEntityIface { authoredBy, changedIn, lastChangedAt, lastChangedBy, lastChangedIn, changes, edits }`
- `DocumentIface : ArtifactIface { previewImage }`
- `EventIface : WeakEntityIface { timestamp, involves }`
- `VersionIface : StrongEntityIface { checkpoint, latestWipCheckpointAncestor, savedChanges, unsavedChanges, kit }`
- `InputIface : WeakEntityIface` (banner only — concrete `*Input` types add fields under `# Arguments`)
- `DiffIface : WeakEntityIface` (banner only)
- `ModificationIface : WeakEntityIface { before, diff, after }`
- `ModificationsIface : WeakEntityIface { removed, modifications, added }`
- `OperationIface : StrongEntityIface { scope, input, modification }`
- `EntityEdgeIface { cursor }`
- `EntityConnectionIface { edges, pageInfo, hash }`

Plus the small unions the schema requires: `ChangeOwner` (Alternative | Checkpoint), `EditOwner` (Alternative | Checkpoint), `CheckpointOwner` (Alternative | Graph | Kit | Author), `Blueprint` (Type | Design — already approximated via `BlueprintConnection`).

## 3. Entity-type pass (region `🏷️ meta` + `📦 kit`)

For every `# Entities` type in [compose/graphql/target.schema.graphql](compose/graphql/target.schema.graphql) that is not yet in [compose/rs/lib.rs](compose/rs/lib.rs), add three Rust types side-by-side using the existing pattern (see `Design`, `DesignEdge`, `DesignConnection` at lib.rs lines 547-569):

- `pub struct <X>` (`Arc + RwLock` fields, `#[Object(name = "<X>")]` impl with `async fn` resolvers)
- `pub struct <X>Edge { node: Arc<X> }` with `#[derive(SimpleObject)] #[graphql(name = "<X>Edge")]`
- `pub struct <X>Connection { edges, page_info, hash }` with `#[derive(SimpleObject)]`

Cover (in this order):

- Geometry: `Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Position`, `Location`, `Attribute`, `Place`
- Authorship: `Family`, `Folder`, `File`, `Author`, `Prop`, `Benchmark`
- Concepts: `Quality`, `Tag`, `Concept`, `Stat`
- Type lattice: `Port`, `Connector`, `Representation`, `Type`
- Design lattice: `Layer`, `Group`, `Piece`, `Connection`, `Side`, `Design`
- Kit roots: `Kit`, `TheKit`, `Clump` (and the missing `ClumpEdge` / `ClumpConnection` / `TheKitEdge` / `TheKitConnection`)

Each entity registers itself with the right interface enum arm (e.g. `RichStrongEntityIface::Quality(Arc<Quality>)`).

## 4. VCS pass (region `🌿 vcs`)

Replace [compose/rs/lib.rs](compose/rs/lib.rs) lines 4677-6031 to match the target VCS region:

- `Edit` (replaces `Transaction`): owner is `EditOwner` interface arm, exposes `forwards: OperationConnection!`, `backwards: OperationConnection!`, `sequenceNumber`, `startedAt`, `finishedAt`, `description`, `origin`. The previously bundled `change_seq: AtomicU64` and apply-pipeline state moves into `Alternative` (which is the WIP container in the new schema).
- `Change`: owner is `ChangeOwner` interface arm; exposes `edits: EditConnection!`, `startedAt`, `savedAt`, `description`, `origin`, plus `saved: Boolean # computed`.
- `Checkpoint`: unchanged shape; owner becomes `CheckpointOwner` arm.
- `Alternative`: owns `Edit`s + `Change`s, exposes WIP iteration state previously kept in `Draft`.
- `Conflict`, `Graph`, `Session`, `TheKit`: unchanged structurally; `#[Object(name = ...)]` brought in line.
- All `name = "ChangeOwner"` / `name = "Transaction"` / `name = "Draft"` references in `kit_backbone`, `worker`, `gql`, `wasm_bridge` updated.

Drop `Draft`-specific persistence (the SQLite tables wrapped by `kit_backbone` keep the same row shapes; only the in-memory wrapper changes name/role).

## 5. Diff pass (new region `🔀 diff`)

For every `type <X>Diff implements Diff` in target.schema.graphql (~30 types) add:

- `pub struct <X>Diff { id, hash, ...per-diff fields... }` with `#[derive(SimpleObject)]` or `#[Object]`
- `pub struct <X>DiffEdge`, `pub struct <X>DiffConnection`
- A variant on `DiffIface`

Per-diff field shape mirrors the schema (e.g. `PieceDiff` exposes `removeName`, `removeDescription`, `position`, `removePosition`, `scale`, `removeScale`, `blueprint`, `props`, `attributes`).

Update existing in-engine `KitDiff` / `CanonicalKitDiff` / `SemanticDiff` (lib.rs 6510, 6541, 7697) to construct and emit the new public `*Diff` types where they currently emit ad-hoc JSON or local structs.

## 6. Modification pass (new region `🛠️ modification`)

For every `type <X>Modification implements Modification` (~30) add the same struct + Edge + Connection trio with the narrow `before: Arc<X>`, `diff: Arc<XDiff>`, `after: Arc<X>` triple per the cleanup plan.

The interface enum `ModificationIface` lists every concrete arm.

`PieceModification`, `ConnectionModification`, `DesignModification`, `KitModification` etc. wire into the existing `kit_graph_engine` apply path: the apply path produces an `<X>Modification`, the event bus publishes it, and the `Operation` resolver returns it.

## 7. Modifications-aggregate pass (new region `📦 modifications`)

For every `type <X>Modifications implements WeakEntity` (~30) add:

- `pub struct <X>Modifications { id, hash, owner, removed, modifications, added }` with the narrow union shapes from the cleanup plan
- Edge + Connection
- A variant on `ModificationsIface`

These are the batched roots returned by aggregate operations like `KitModifications`, `DesignModifications`, `PieceModifications`.

## 8. Input pass (new region `📥 input`)

For every `type <X>Input implements Input` (~85+) add a `#[derive(InputObject)]` struct with the `# Arguments` fields. The current `*InputDto` types (lib.rs 7973-8011) are renamed to the canonical names (`CreatedFixedPieceInput`, `FixedPieceInput`, `DraggedPieceInput`, `RenamedKitInput`, `ChangedDescriptionInput`) and the missing ~80 inputs are added in alphabetical groups (Quality, Tag, Concept, Port, Connector, Representation, Type, Layer, Group, Piece, Connection, Design, Kit).

The `OneofObject` `OperationInputOneOf` is regenerated to list every concrete `<X>Input` arm.

## 9. Operation pass (region `⚙️ operation`)

For every `type <X> implements Operation` in target.schema.graphql (~85) add:

- `pub struct <X>` with banner-correct fields (`scope`, `input`, `modification` plus per-op outputs like `quality`, `tags`, `piece`, `pieces`)
- `#[Object(name = "<X>")]` impl that wires through to the existing `kit_graph_engine` apply path
- `pub struct <X>Edge` and `pub struct <X>Connection`
- A variant on `OperationIface`

Cover the seven naming families enumerated in the schema:

- Quality: `CreatedQuality`, `CreatedQualities`, `RenamedQuality`, `UpdatedQualityDescription`, `UpdatedQualityIcon`, `AddedAttributeToQuality`, `AddedAttributesToQuality`, `RemovedAttributeFromQuality`, `RemovedAttributesFromQuality`, `DeletedQuality`, `DeletedQualities`
- Tag: same shape
- Concept: same shape
- Port: same shape (scope `TypeModifications`)
- Type: same shape (scope `KitModifications`)
- Connector: `AddedConnector`, `AddedConnectors`, `RenamedConnector`, `UpdatedConnectorDescription`, `UpdatedConnectorIcon`, `RemovedConnector`, `RemovedConnectors`
- Design: `CreatedDesign`, `CreatedDesigns`, `DeletedDesign`, `DeletedDesigns`, `FlattenedDesign`, plus all piece-level operations (`CreatedFixedPiece`, `MovedPiece`, `MovedPieces`, `DraggedPiece`, `DraggedPieces`, `RenamedPiece`, `UpdatedPieceDescription`, `FixedPiece`, `FixedPieces`, `ChangedPieceToType`, `ChangedPiecesToType`, `AddedChildPieceWithParentConnection`, `AddedChildPiecesWithParentConnections`, `AddedHangingChildPieceWithParentConnection`, `AddedHangingChildPiecesWithParentConnections`, `DeletedPiece`, `DeletedPieces`, `DeletedPiecesAndConnections`)
- Kit: `RenamedKit`, `ChangedDescriptio
