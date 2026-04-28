---
name: Refactor semio/rs to in-memory Arc references (no value cloning)
overview: Refactor [semio/rs/lib.rs](semio/rs/lib.rs) so every entity lives behind a single shared `Arc<Entity>` with interior `async_lock::RwLock` per mutable field. GraphQL resolvers take `&self` on the entity (deref'd through the Arc) and return `Arc<Child>` for relationships, so a query like `wip.theKit.design(id).piece(id).pose` only acquires the locks it actually needs and never deep-copies an aggregate Vec.
todos:
 - id: field-rwlocks
   content: Convert every mutable field on every entity (Connector, Representation, Type, Side, Connection, Piece, Design, Kit, Change, Transaction, Draft, Checkpoint, Alternative, Graph, Session, Conflict, Op*, meta entities) into async_lock::RwLock<FieldT>; switch owned-child Vec<T> to RwLock<Vec<Arc<T>>> and back-pointers to Weak<Parent>.
   status: completed
 - id: ctor-mut-rewrite
   content: Rewrite every constructor to return Arc<Self>; rewrite every &mut self mutator to take &self and acquire its own interior write-lock. No &mut self anywhere on entities.
   status: completed
 - id: resolvers-arc
   content: "Update every #[Object] resolver: relationships return Arc<Child> / Option<Arc<Child>> / Vec<Arc<Child>>; value leaves dereference the per-field read-guard once; owners upgrade Weak to Arc."
   status: completed
 - id: graph-root
   content: Drop Arc<RwLock<Graph>> in favour of Arc<Graph> in worker::ParentRuntime. Delete snapshot_wip_graph/snapshot_auth_graph. Make Query.wip / Query.authoritative return Arc<Graph> directly. Make Graph::apply_create_fixed_piece take &self via interior locks.
   status: completed
 - id: ops-arc-payload
   content: "Change Operation structs to carry Arc<Entity> payloads (e.g. CreatedFixedPiece.piece: Arc<Piece>). Update KitEvent variants to wrap Arc<Op>. Update Subscription filter macro and OperationKind/OperationIface derives to operate over Arc<*> variants."
   status: completed
 - id: tests-no-clone
   content: "Update lib.rs tests: keep the existing 4 green; add no_deep_clone_on_traversal (Arc::strong_count guard around a deep GraphQL query) and mutation_visible_without_resnapshotting (two reads across a mutation prove in-place mutability through interior locks)."
   status: completed
 - id: compile-and-pass
   content: cargo check (native + wasm32-unknown-unknown) and cargo test --lib must pass with all 6 tests green.
   status: completed
isProject: false
---

# Refactor semio/rs to in-memory Arc references (no value cloning)

All edits land in [semio/rs/lib.rs](semio/rs/lib.rs). No new files.

## 1. Storage shape per entity

Every entity that has an `id: Id` becomes immutable in identity but interior-mutable in fields. The struct itself is shared as `Arc<Self>`; each mutable field gets its own `async_lock::RwLock`. Owned children are `RwLock<Vec<Arc<Child>>>`; back-references are `Weak<Parent>`.

Example shape (Piece):

```rust
pub struct Piece {
    pub id: Id,
    pub owner_design: Weak<Design>,
    name: RwLock<Option<String>>,
    description: RwLock<Option<String>>,
    pose: RwLock<Option<Position>>,
    scale: RwLock<Option<f64>>,
    blueprint: RwLock<Option<kit::r#type::Blueprint>>,
    connection_kind: RwLock<Option<PieceConnectionKind>>,
    parent_piece: RwLock<Option<Weak<Piece>>>,
    parent_connection: RwLock<Option<Weak<Connection>>>,
    child_pieces: RwLock<Vec<Arc<Piece>>>,
    child_connections: RwLock<Vec<Arc<Connection>>>,
    props: RwLock<Vec<Arc<Prop>>>,
    attributes: RwLock<Vec<Arc<Attribute>>>,
    bus: Weak<EventBus>,
}
```

The same pattern applies uniformly to: `Connector`, `Representation`, `Type`, `Side`, `Connection`, `Design`, `Kit`, `Change`, `Transaction`, `Draft`, `Checkpoint`, `Alternative`, `Graph`, `Session`, `Conflict`, every `Op*` struct, and the strong meta entities (`Location`, `File`, `Folder`, `Author`, `Attribute`, `Prop`, `Benchmark`, `Quality`, `Tag`, `Concept`, `Stat`, `Layer`, `Group`).

Pure value types (`Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Position`, `Id`, `Timestamp`) stay plain `Copy`/`Clone` values — they are only ever leaf bytes.

## 2. Constructor + mutation pattern

Constructors return `Arc<Self>`; mutators take `&self` and use interior locks.

```rust
impl Piece {
    pub async fn new_fixed(owner_design: Weak<Design>, blueprint: kit::r#type::Blueprint, pose: Position, bus: Weak<EventBus>) -> Arc<Self> { /* RwLock::new(...) for each field */ }
    pub async fn set_pose(&self, pose: Position) { *self.pose.write().await = Some(pose); }
    pub async fn add_child(&self, child: Arc<Piece>) { self.child_pieces.write().await.push(child); }
}
```

No `&mut self` anywhere on entities — mutations always go through `&self` + lock acquire. The single-threaded `worker::ChildRuntime` loop already serializes mutations, so write-locks are uncontended in steady state.

## 3. Resolver pattern (the rule)

`#[Object] impl Entity` is unchanged structurally — still on the bare entity type. Returns are split as:

- Value-typed leaves return owned values: `async fn id(&self) -> Id`, `async fn pose(&self) -> Option<Position> { *self.pose.read().await }`.
- Single related entities return `Arc<Child>` (or `Option<Arc<Child>>` / `Vec<Arc<Child>>`):

```rust
#[Object(name = "Design")]
impl Design {
    async fn pieces(&self) -> Vec<Arc<Piece>> { self.pieces.read().await.clone() } // Vec<Arc> clone = refcount bumps only
    async fn piece(&self, id: Id) -> Option<Arc<Piece>> {
        self.pieces.read().await.iter().find(|p| p.id == id).cloned()
    }
    async fn owner(&self) -> Option<Arc<crate::kit::Kit>> { self.owner_kit.upgrade() }
}
```

`Vec<Arc<Piece>> as OutputType` is fine — `async-graphql` derefs through `Arc`. The path `wip.theKit.design(id).piece(id).pose` therefore touches:

```mermaid
flowchart LR
  Q[Query.wip] -->|"ArcSwap.load"| G[Arc Graph]
  G -->|"&kit field"| K[Arc Kit]
  K -->|"designs.read.await scan"| D[Arc Design]
  D -->|"pieces.read.await scan"| P[Arc Piece]
  P -->|"pose.read.await deref"| V[Position value]
```

No `Vec<Piece>` clone, no `Design` clone, no `Kit` clone — only one `RwLockReadGuard` per hop and one `Arc::clone` per matched child.

## 4. Owner / back-reference wiring

- Forward (owner -> child) is `Vec<Arc<Child>>` inside the owner's `RwLock`.
- Back (child -> owner) is `Weak<Owner>` set at construction time (or via a `init_owner` setter once the Arc exists). On resolve, `weak.upgrade()` produces an `Arc<Owner>`. Cycles are broken by Weak everywhere on the back path.
- For `Connection.connected: Side` / `connecting: Side` (declared `Side` not `Side!`-of-piece), `Side` itself becomes `Arc<Side>` with `piece: Weak<Piece>`.

## 5. Graph root + ParentRuntime

`worker::ParentRuntime` drops `Arc<RwLock<Graph>>` in favour of plain `Arc<Graph>`:

```rust
pub struct ParentRuntime {
    pub bus: Arc<EventBus>,
    pub wip: ChildPort,
    pub auth: ChildPort,
    pub wip_graph: Arc<Graph>,
    pub auth_graph: Arc<Graph>,
    pub sessions: RwLock<Vec<Arc<Session>>>,
    pub conflicts: RwLock<Vec<Arc<Conflict>>>,
}
```

The two `snapshot_*_graph` methods (which `.read().await.clone()` today, copying the entire `Graph` per query) are removed. `Query.wip` becomes:

```rust
async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
    Ok(rt(ctx)?.wip_graph.clone()) // Arc::clone = refcount bump
}
```

`Query.authoritative` and `Query.authorative` mirror this with `auth_graph`.

## 6. Mutation path stays single-emit

`worker::ChildRuntime::apply` already routes through `EventBus::emit_event`. The new shape:

```rust
let piece: Arc<Piece> = self.graph.apply_create_fixed_piece(draft_id, transaction_id, design_id, blueprint_id, pose, name, description).await?;
let op = Arc::new(CreatedFixedPiece { id: request_id, owner_change: Weak::new(), input, diff: Diff::default(), piece: piece.clone() });
self.bus.emit_event(KitEvent::CreatedFixedPiece(op)).await; // op is Arc, not deep-cloned
```

`KitEvent` variants carry `Arc<Op>` instead of owned `Op`. The single `EventBus::emit_event` definition is unchanged — guard test still asserts uniqueness of the canonical signature.

`Graph::apply_create_fixed_piece` becomes `&self` (interior mutability), constructs `Arc<Piece>`, write-locks the design's `pieces` Vec, pushes the Arc, returns it.

## 7. Op carrier shape

Operation structs carry `Arc<Entity>` payloads:

```rust
pub struct CreatedFixedPiece {
    pub id: Id,
    pub owner_change: Weak<Change>,
    pub input: CreatedFixedPieceInput,
    pub diff: Diff,
    pub piece: Arc<crate::kit::design::piece::Piece>,
}
```

`#[Object] impl CreatedFixedPiece` returns `Arc<Piece>` for `piece(&self)`. `OperationKind` and `OperationIface` derive `Union` / `Interface` over `Arc<*>` variants (`async-graphql`'s derives accept `Arc<T>` variants).

## 8. Test updates ([semio/rs/lib.rs](semio/rs/lib.rs) `mod tests`)

- Existing tests stay green: `parses_target_schema`, `single_emit_event_in_codebase`, `create_fixed_piece_end_to_end`, `wip_and_authoritative_are_isolated`.
- Add `no_deep_clone_on_traversal()`: build the schema, manually grab `Arc::strong_count(&piece)` after insertion, run a deep GraphQL query (`{ wip { theKit { designs { pieces { pose { center { u } } } } } } }`), then re-check `strong_count` — it must not have grown beyond a small bound proportional to the resolver chain depth (proves the resolver returns Arc clones, not value copies).
- Add `mutation_visible_without_resnapshotting()`: run two GraphQL `wip` reads back-to-back across a mutation; the second read must reflect the mutation without anyone calling a snapshot/clone helper (proves in-place mutation through interior locks).

## 9. Mechanical conversion table

For each entity in the file, the conversion is mechanical:

- Move every mutable field into `async_lock::RwLock<FieldT>`.
- Move every owned-child `Vec<T>` into `RwLock<Vec<Arc<T>>>`.
- Move every back-pointer into `Weak<Parent>` (immutable, set once at construction).
- Replace all `pub async fn ...(&mut self, ...)` with `pub async fn ...(&self, ...)` + interior write-lock.
- Update every `#[Object]` resolver:
  - Owned children -> `Vec<Arc<Child>>` / `Option<Arc<Child>>` / `Arc<Child>`.
  - Owner -> `Option<Arc<Owner>>` (`weak.upgrade()`).
  - Value leaves -> deref the read-guard once: `*self.pose.read().await` for `Copy`, `self.name.read().await.clone()` for owned `String`/`Option<String>`.
- Replace `Arc<RwLock<Graph>>` with `Arc<Graph>` in `worker::ParentRuntime`; delete `snapshot_wip_graph` / `snapshot_auth_graph`; rewrite `Query.wip` / `Query.authoritative` to return `Arc<Graph>` directly.
- Update `KitEvent::*` variants to carry `Arc<Op>`; update `Subscription` filter macro to yield `Arc<Op>`.

## 10. Out of scope

- No new caches/indexes (linear scan inside locked Vec stays for the skeleton; HashMap indexes can be added later in a separate ticket).
- No `arc_swap` adoption (the user picked plain `Arc<RwLock>` per entity; that's enough).
- Cargo deps unchanged — `async_lock`, `async_broadcast`, `async_channel` are already pulled in.
