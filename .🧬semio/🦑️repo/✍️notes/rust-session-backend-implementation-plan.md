# Rust Session-Backend Service Implementation Plan

## 1. Goal

Build a single Rust backend service that hosts collaborative session state for a kit-shaped domain with strongly typed relational persistence, deterministic conflict handling, support for cyclic references, and persisted compose/presentational state.

The service has these non-negotiable properties:

- exactly one logical writer on the server per session
- PostgreSQL as the only durable store
- one PostgreSQL schema only
- one Rust source file only: `bin.rs`
- no JSON blobs for canonical state or diffs
- explicit relational modeling with foreign keys, enums, check constraints, and typed history
- tombstone-based deletion in the write path
- compose/presentational data persisted as first-class entities
- clean support for historical artifact views for **Type** and **Design** at these lookback points:
  - `1min`
  - `5min`
  - `10min`
  - `30min`
  - `1h`
  - `5h`
  - `1d`
  - `3d`
  - `7d`
  - `1mo`
  - `6mo`
  - `1y`

The implementation target is a clean new system, not a compatibility layer.

---

## 2. High-level architecture

The service is a single deployable process with four internal parts, all implemented inside `bin.rs`:

1. HTTP/WebSocket API
2. Session directory and session actors
3. Domain and compose command application
4. PostgreSQL persistence and historical reads

The core runtime shape is:

```text
frontend -> HTTP/WS -> SessionDirectory -> SessionActor(session_id) -> PostgreSQL
```

For each active session, exactly one in-process actor owns mutation ordering. That actor:

- receives domain and compose commands
- validates them against the current session state
- resolves conflicts deterministically
- persists accepted changes in one SQL transaction
- updates in-memory state
- broadcasts accepted changes to subscribers

This keeps the design simple and correct:

- no peer-to-peer synchronization
- no multi-writer database race logic in the domain layer
- no distributed locking protocol beyond the actor itself

---

## 3. Single-file Rust structure (`bin.rs`)

Everything lives in one file. The file is still organized into sections so it remains readable.

Recommended order inside `bin.rs`:

1. imports, constants, configuration structs
2. ID newtypes and shared scalar/value objects
3. enums for entity kinds, property keys, conflict policies, lifecycle, and command kinds
4. domain structs for current in-memory state
5. transport DTOs for HTTP input/output
6. command enums and patch types
7. SQL row structs and query helper functions
8. session directory
9. session actor implementation
10. conflict engine and validation functions
11. snapshot and history query functions
12. WebSocket event streaming
13. HTTP route handlers
14. startup, migration runner, and `main`

Inline submodules are allowed only if they remain inside `bin.rs`, for example:

```rust
mod api { /* inline */ }
mod domain { /* inline */ }
mod db { /* inline */ }
```

No external crate split is required. The point is operational and structural simplicity, not premature modularization.

Recommended dependencies:

- `tokio`
- `axum`
- `sqlx`
- `serde`
- `uuid`
- `time`
- `thiserror`
- `tracing`
- `tracing-subscriber`

---

## 4. Type-safety rules

### 4.1 Newtypes for every identity

Never pass raw `Uuid` values through the domain layer. Define strong newtypes for every entity identity.

Example:

```rust
struct SessionId(Uuid);
struct KitId(Uuid);
struct TypeId(Uuid);
struct DesignId(Uuid);
struct PieceId(Uuid);
struct GroupId(Uuid);
struct ConnectionId(Uuid);
struct LayerId(Uuid);
struct PropId(Uuid);
struct QualityId(Uuid);
struct PortId(Uuid);
struct PersonId(Uuid);
struct CommandId(Uuid);
struct ClientId(Uuid);
struct RequestId(Uuid);
```

### 4.2 Explicit field patch semantics

Use explicit patch enums instead of sparse nullable DTO semantics.

```rust
enum FieldPatch<T> {
    NoChange,
    Set(T),
    Clear,
}

enum RequiredFieldPatch<T> {
    NoChange,
    Set(T),
}
```

This removes ambiguity between:

- untouched field
- field assigned a value
- nullable field explicitly cleared

### 4.3 Explicit property registry

Every mutable property gets a compile-time `PropertyKey` enum value.

Example:

```rust
enum PropertyKey {
    TypeName,
    TypeParentType,
    TypeDescription,
    DesignName,
    DesignParentDesign,
    DesignActiveLayer,
    PieceType,
    PiecePlane,
    PieceCenter,
    PieceScale,
    GroupMembership,
    ConnectionEndpoints,
    ComposeCursor,
    ComposeLook,
}
```

This registry drives:

- conflict policy lookup
- property clock updates
- audit/history generation
- testing coverage

---

## 5. Domain boundary

The consistency boundary is the **session**.

All canonical domain data and all compose data are scoped by `session_id`.

Each active session actor owns an in-memory state shaped roughly like this:

```rust
struct SessionState {
    session_id: SessionId,
    domain_version: i64,
    compose_version: i64,
    kit: KitState,
    types: BTreeMap<TypeId, TypeState>,
    designs: BTreeMap<DesignId, DesignState>,
    pieces: BTreeMap<PieceId, PieceState>,
    layers: BTreeMap<LayerId, LayerState>,
    groups: BTreeMap<GroupId, GroupState>,
    connections: BTreeMap<ConnectionId, ConnectionState>,
    props: BTreeMap<PropId, PropState>,
    compose_people: BTreeMap<(PersonId, String), ComposePersonState>,
}
```

The full current session state is loaded from PostgreSQL when the actor starts and is rebuilt from relational rows only.

---

## 6. PostgreSQL layout: one schema only

Use exactly one PostgreSQL schema, for example `app`.

Everything lives under that schema:

- runtime tables
- domain tables
- historical/version tables
- compose tables
- enums

There are no separate SQL schemas such as `runtime`, `core`, `history`, or `compose`.

To keep the schema readable, use table naming prefixes:

- `session_*`
- `commit_*`
- `type_*`
- `design_*`
- `piece_*`
- `group_*`
- `compose_*`

Example DDL style:

```sql
create schema if not exists app;
set search_path to app;
```

---

## 7. Persistence model: temporal relational tables

The cleanest way to support historical artifact viewing without JSON is to make the canonical domain tables **temporal**.

That means each mutable domain row carries version validity:

- `valid_from_version bigint not null`
- `valid_to_version bigint null`
- `recorded_at timestamptz not null`
- `recorded_by_command_id uuid not null`

A row is current when `valid_to_version is null`.

When a property changes:

1. close the current row by setting `valid_to_version = next_domain_version`
2. insert a new row with updated values and `valid_from_version = next_domain_version`

Deletes are represented as tombstones, not hard deletes.

This gives three important benefits:

1. current-state reads remain relational and strongly typed
2. historical reads become simple `as-of version` queries
3. Type and Design artifact views at named lookback points become first-class, not bolted on

### 7.1 Global domain commit table

Create a table that maps every accepted domain version to commit metadata.

### `app.domain_commit`

Columns:

- `session_id uuid not null`
- `domain_version bigint not null`
- `command_id uuid not null`
- `committed_at timestamptz not null`
- `actor_person_id uuid not null`
- primary key `(session_id, domain_version)`
- unique `(command_id)`

This table is the anchor for all historical lookups.

### 7.2 Session table

### `app.session`

Columns:

- `session_id uuid primary key`
- `kit_id uuid not null`
- `domain_version bigint not null default 0`
- `compose_version bigint not null default 0`
- `status text not null`
- `writer_instance_id uuid null`
- `writer_lease_expires_at timestamptz null`
- `created_at timestamptz not null`
- `updated_at timestamptz not null`

The initial implementation can run on one service instance and still keep the writer columns for future safety.

### 7.3 Idempotency and request tracking

### `app.session_command`

Columns:

- `command_id uuid primary key`
- `session_id uuid not null`
- `client_id uuid not null`
- `request_id uuid not null`
- `command_kind text not null`
- `base_domain_version bigint null`
- `base_compose_version bigint null`
- `accepted_domain_version bigint null`
- `accepted_compose_version bigint null`
- `actor_person_id uuid not null`
- `received_at timestamptz not null`
- `applied_at timestamptz null`
- `status text not null`
- unique `(session_id, client_id, request_id)`

### 7.4 Property clock table

### `app.property_clock`

Columns:

- `session_id uuid not null`
- `entity_kind text not null`
- `entity_id uuid not null`
- `property_key text not null`
- `last_changed_domain_version bigint not null`
- `last_command_id uuid not null`
- primary key `(session_id, entity_kind, entity_id, property_key)`

This table is the main index for property-level stale-write detection.

---

## 8. Canonical domain tables

Every domain entity table is temporal and uses one row version per accepted domain change.

The pattern is:

- logical identity columns (`session_id`, entity id)
- business columns
- lifecycle/tombstone columns
- validity window columns (`valid_from_version`, `valid_to_version`)
- audit columns (`recorded_at`, `recorded_by_command_id`)

### 8.1 Root and simple entities

Representative examples:

### `app.kit_row`

- `session_id`
- `kit_id`
- `name text not null`
- `version_label text null`
- `remote text null`
- `homepage text null`
- `license text null`
- `preview text null`
- `icon text null`
- `image text null`
- `description text null`
- `lifecycle text not null`
- `valid_from_version bigint not null`
- `valid_to_version bigint null`
- `recorded_at timestamptz not null`
- `recorded_by_command_id uuid not null`
- primary key `(session_id, kit_id, valid_from_version)`

### `app.author_row`

- `session_id`
- `author_id`
- `name text not null`
- `email text not null`
- `lifecycle text not null`
- validity columns
- audit columns

### `app.tag_row`, `app.concept_row`, `app.quality_row`, `app.port_row`, `app.file_row`, `app.folder_row`

Use the same pattern.

### 8.2 Type artifact tables

A **Type artifact** is reconstructed from these temporal tables:

- `app.type_row`
- `app.type_author_row`
- `app.type_concept_row`
- `app.type_attribute_row`
- `app.type_prop_row`
- `app.model_row`
- `app.model_tag_row`
- `app.connector_row`
- `app.connector_prop_row`

#### `app.type_row`

Columns:

- `session_id uuid not null`
- `type_id uuid not null`
- `name text not null`
- `parent_type_id uuid null`
- `is_abstract boolean null`
- `folder_id uuid null`
- `stock integer null`
- `is_virtual boolean null`
- `unit text null`
- `location_id uuid null`
- `icon text null`
- `image text null`
- `description text null`
- `lifecycle text not null`
- `deleted_at_version bigint null`
- `valid_from_version bigint not null`
- `valid_to_version bigint null`
- `recorded_at timestamptz not null`
- `recorded_by_command_id uuid not null`
- primary key `(session_id, type_id, valid_from_version)`

Important constraints:

- self-FK on `parent_type_id` is `DEFERRABLE INITIALLY DEFERRED`
- referenced folder/location rows must exist in the same session
- `lifecycle` uses `active` or `tombstoned`

#### `app.connector_row`

- `session_id`
- `connector_id`
- `type_id`
- `name text null`
- `port_id uuid null`
- `mandatory boolean null`
- `t double precision not null`
- `point_x`, `point_y`, `point_z`
- `direction_x`, `direction_y`, `direction_z`
- `description text null`
- lifecycle + validity + audit columns

### 8.3 Design artifact tables

A **Design artifact** is reconstructed from these temporal tables:

- `app.design_row`
- `app.design_author_row`
- `app.design_concept_row`
- `app.design_prop_row`
- `app.layer_row`
- `app.piece_row`
- `app.piece_prop_row`
- `app.group_row`
- `app.group_piece_row`
- `app.connection_row`
- `app.stat_row`

#### `app.design_row`

Columns:

- `session_id uuid not null`
- `design_id uuid not null`
- `name text not null`
- `parent_design_id uuid null`
- `is_abstract boolean null`
- `folder_id uuid null`
- `active_layer_id uuid null`
- `can_scale boolean null`
- `can_mirror boolean null`
- `unit text null`
- `location_id uuid null`
- `icon text null`
- `image text null`
- `description text null`
- `lifecycle text not null`
- `deleted_at_version bigint null`
- `valid_from_version bigint not null`
- `valid_to_version bigint null`
- `recorded_at timestamptz not null`
- `recorded_by_command_id uuid not null`
- primary key `(session_id, design_id, valid_from_version)`

#### `app.piece_row`

- `session_id`
- `piece_id`
- `design_id uuid not null`
- `name text null`
- `type_id uuid null`
- plane columns
- center columns
- `scale double precision null`
- mirror plane columns
- `is_hidden boolean null`
- `is_locked boolean null`
- `color text null`
- `description text null`
- `lifecycle text not null`
- `deleted_at_version bigint null`
- `valid_from_version bigint not null`
- `valid_to_version bigint null`
- `recorded_at timestamptz not null`
- `recorded_by_command_id uuid not null`
- primary key `(session_id, piece_id, valid_from_version)`

#### `app.group_piece_row`

This is also temporal.

- `session_id`
- `group_id`
- `piece_id`
- `ordinal integer not null`
- `lifecycle text not null`
- validity + audit columns
- primary key `(session_id, group_id, piece_id, valid_from_version)`

#### `app.connection_row`

- `session_id`
- `connection_id`
- `design_id uuid not null`
- connected-side columns
- connecting-side columns
- `gap`, `shift`, `rise`, `rotation`, `turn`, `tilt`, `u`, `v`
- `description text null`
- lifecycle + validity + audit columns

### 8.4 Current reads

Current-state reads are always:

```sql
... where valid_to_version is null and lifecycle = 'active'
```

Historical reads are always:

```sql
... where valid_from_version <= $as_of_version
  and (valid_to_version is null or valid_to_version > $as_of_version)
  and lifecycle = 'active'
```

If a deleted artifact should still be viewable historically, the read is performed at a version before its tombstone row becomes active.

---

## 9. Historical artifact views

Historical artifact viewing is a first-class feature of the persistence model.

### 9.1 Supported lookback points

The service must support these named lookbacks for **Type** and **Design** views:

- `1min`
- `5min`
- `10min`
- `30min`
- `1h`
- `5h`
- `1d`
- `3d`
- `7d`
- `1mo`
- `6mo`
- `1y`

Use these exact tokens in the public API to avoid ambiguity between minute and month.

### 9.2 Resolution algorithm

For any request like:

```text
GET /sessions/{session_id}/artifacts/designs/{design_id}?at=5h
```

resolve it as follows:

1. map the token to an interval
2. compute `target_ts = now_utc - interval`
3. query `app.domain_commit` for the latest version at or before `target_ts`
4. use that `as_of_version` to reconstruct the artifact from temporal rows

Example SQL for step 3:

```sql
select max(domain_version)
from app.domain_commit
where session_id = $1
  and committed_at <= $2;
```

If no version exists that far back, return the earliest available artifact state together with metadata indicating that the target precedes the session history.

### 9.3 Artifact reconstruction rules

#### Type artifact at version `V`

Load:

- the `type_row` active at `V`
- all active `connector_row`, `model_row`, prop rows, attribute rows, author links, and concept links for that type at `V`
- referenced supporting rows such as `port_row`, `quality_row`, and `file_row` at `V`

#### Design artifact at version `V`

Load:

- the `design_row` active at `V`
- all active `piece_row`, `connection_row`, `group_row`, `group_piece_row`, `layer_row`, `stat_row`, and prop rows for that design at `V`
- referenced `type_row` versions for piece type references at `V`

This means a historical design view always resolves references against the same historical boundary. If a type was later changed or deleted, the design view for `1d ago` still shows the type as it existed one day ago.

### 9.4 Named lookback helper table or code constant

The simplest implementation is a code constant in `bin.rs`:

```rust
const SUPPORTED_LOOKBACKS: &[(&str, time::Duration)] = &[
    ("1min", time::Duration::minutes(1)),
    ("5min", time::Duration::minutes(5)),
    ("10min", time::Duration::minutes(10)),
    ("30min", time::Duration::minutes(30)),
    ("1h", time::Duration::hours(1)),
    ("5h", time::Duration::hours(5)),
    ("1d", time::Duration::days(1)),
    ("3d", time::Duration::days(3)),
    ("7d", time::Duration::days(7)),
    ("1mo", time::Duration::days(30)),
    ("6mo", time::Duration::days(182)),
    ("1y", time::Duration::days(365)),
];
```

No extra table is needed.

---

## 10. Compose / presentational persistence

Compose state is persisted as first-class relational entities in the same `app` schema.

Compose data is not part of canonical artifact history and must not advance the domain version. It uses its own monotonic `compose_version` per session.

### 10.1 Tables

### `app.compose_person`

- `session_id uuid not null`
- `person_id uuid not null`
- `frontend_id text not null`
- `display_name text null`
- `color text null`
- `is_present boolean not null`
- `last_seen_at timestamptz not null`
- `expires_at timestamptz not null`
- primary key `(session_id, person_id, frontend_id)`

### `app.compose_cursor`

- `session_id`
- `person_id`
- `frontend_id`
- `coord_x double precision not null`
- `coord_y double precision not null`
- `coord_z double precision null`
- `compose_version bigint not null`
- `updated_at timestamptz not null`
- primary key `(session_id, person_id, frontend_id)`

### `app.compose_look`

- `session_id`
- `person_id`
- `frontend_id`
- `camera_pos_x`, `camera_pos_y`, `camera_pos_z`
- `camera_forward_x`, `camera_forward_y`, `camera_forward_z`
- `camera_up_x`, `camera_up_y`, `camera_up_z`
- `compose_version bigint not null`
- `updated_at timestamptz not null`
- primary key `(session_id, person_id, frontend_id)`

### `app.compose_selection_piece`

- `session_id`
- `person_id`
- `frontend_id`
- `piece_id`
- `compose_version bigint not null`
- primary key `(session_id, person_id, frontend_id, piece_id)`

Additional compose tables can be added for hovered entity, active tool, drag state, viewport, and panel focus using the same pattern.

### 10.2 Compose behavior

Compose writes are:

- persisted
- streamable
- scoped to session + person + frontend
- expiry-aware
- excluded from canonical domain conflict logic

Compose update coalescing is allowed inside the actor for high-frequency updates such as cursor movement.

---

## 11. Command model

The authoritative write contract is explicit commands, not generic JSON patches and not raw GraphQL-shaped sparse diffs.

### 11.1 Domain commands

Examples:

```rust
enum DomainCommand {
    PatchKit(PatchKit),
    AddType(AddType),
    PatchType(PatchType),
    DeleteType(DeleteType),
    AddDesign(AddDesign),
    PatchDesign(PatchDesign),
    DeleteDesign(DeleteDesign),
    AddPiece(AddPiece),
    PatchPiece(PatchPiece),
    DeletePiece(DeletePiece),
    AddConnection(AddConnection),
    PatchConnection(PatchConnection),
    DeleteConnection(DeleteConnection),
    AddGroup(AddGroup),
    PatchGroup(PatchGroup),
    DeleteGroup(DeleteGroup),
}
```

Each command envelope includes:

- `command_id`
- `client_id`
- `request_id`
- `actor_person_id`
- `base_domain_version`
- typed payload

### 11.2 Batch command support

Multi-entity graph edits must be atomic. Use:

```rust
struct DomainBatch {
    commands: Vec<DomainCommand>,
}
```

A single batch can create a design, create multiple pieces, attach them to groups, and connect them in one actor turn and one SQL transaction.

### 11.3 Compose commands

Examples:

```rust
enum ComposeCommand {
    UpsertPresence(UpsertPresence),
    SetCursor(SetCursor),
    SetLook(SetLook),
    ReplacePieceSelection(ReplacePieceSelection),
    ClearPresence(ClearPresence),
}
```

Compose commands carry `base_compose_version` rather than `base_domain_version`.

---

## 12. Conflict handling

Conflict handling is deterministic and property-based.

### 12.1 Why property-level conflict checks are enough

Because there is exactly one writer actor per session, no two commands can commit concurrently for that session. The only conflict problem is stale client intent relative to a newer committed version.

The actor handles this by:

1. calculating the set of touched properties for the incoming command
2. consulting `app.property_clock`
3. comparing each touched property against the command’s `base_domain_version`
4. applying the compile-time policy for that property

### 12.2 Conflict policy kinds

Keep the policy set small and explicit:

- `RejectIfChanged`
- `LastWriterWins`
- `AdditiveNumeric`
- `ReplaceOrderedMembership`
- `UnionMembership`
- `ReferenceMustExistAndBeActive`
- `ReferenceMayBecomeNull`
- `TombstoneAwareReject`
- `ComposeLastWriterWins`

Recommended defaults:

- names, parents, active layer, and most identity-defining fields: `RejectIfChanged`
- free-form description fields: `LastWriterWins`
- piece type references: `ReferenceMustExistAndBeActive`
- group membership ordering: `ReplaceOrderedMembership`
- compose cursor/look: `ComposeLastWriterWins`

### 12.3 Property clock update

After an accepted domain command, update the property clock for every changed property:

```text
(session_id, entity_kind, entity_id, property_key) -> accepted_domain_version
```

This avoids replaying the full history log for conflict checks.

---

## 13. Cyclic dependency support

The data model supports self-references and cross-references such as:

- `Type.parent -> Type`
- `Design.parent -> Design`
- `Piece.type -> Type`
- `Piece.design -> Design`
- `Group -> Piece[]`
- future mutually dependent graphs created in one batch

### 13.1 SQL support

Any foreign key that may participate in same-transaction graph creation should be declared:

```sql
DEFERRABLE INITIALLY DEFERRED
```

That includes at least:

- `type_row.parent_type_id -> type_row.type_id`
- `design_row.parent_design_id -> design_row.design_id`
- `folder_row.parent_folder_id -> folder_row.folder_id`
- references from nested tables created together in one batch

### 13.2 Actor apply order for cyclic graph edits

Inside one SQL transaction:

1. reserve identities for newly created entities
2. insert initial temporal rows with minimal required data
3. apply scalar fields
4. apply references and membership edges
5. run semantic validation
6. close old temporal rows and insert new temporal rows as needed
7. write domain commit row, command row, and property clocks
8. commit

### 13.3 Delete behavior under cyclic references

The write path never hard-deletes domain rows.

Deletes become tombstones. The default policy for referenced entities is:

- `Type`: reject delete if referenced by any active piece at the candidate next version
- `Quality`: reject delete if referenced by active props/stats
- `Port`: reject delete if referenced by active connectors
- `Layer`: reject delete if active design state depends on it
- `Design`: allow tombstone only when owned children are tombstoned in the same batch

This keeps the graph valid at every committed version.

### 13.4 Example: delete a type while another user creates a design piece referencing that type

Assume client A sends `DeleteType(T)` and client B sends `AddDesign(D) + AddPiece(P { type_id = T })` based on an older version.

Because one actor serializes all writes:

#### If delete commits first

- the actor writes a tombstone revision for `Type T`
- the next command resolves `Piece.type_id = T`
- the policy `ReferenceMustExistAndBeActive` fails
- the whole batch creating `D` and `P` is rejected atomically

#### If create commits first

- the actor writes the design and piece rows
- the delete command then sees active references from piece `P`
- the delete policy for `Type` rejects the delete

No dangling current-state reference is ever committed.

---

## 14. SQL transaction flow for accepted domain commands

For each accepted domain batch:

1. verify idempotency from `app.session_command`
2. `select ... for update` the `app.session` row
3. compute `next_domain_version = current_domain_version + 1`
4. check touched properties against `app.property_clock`
5. apply conflict policies
6. perform semantic validation against the in-memory state and, when needed, SQL uniqueness checks
7. close affected temporal rows by setting `valid_to_version = next_domain_version`
8. insert replacement temporal rows with `valid_from_version = next_domain_version`
9. insert into `app.domain_commit`
10. upsert `app.property_clock`
11. update `app.session.domain_version`
12. mark the command accepted in `app.session_command`
13. commit

Broadcast to subscribers only after commit succeeds.

---

## 15. API surface

Keep the transport surface minimal and explicit.

### 15.1 Write endpoints

- `POST /sessions`
- `POST /sessions/{session_id}/attach`
- `POST /sessions/{session_id}/commands/domain`
- `POST /sessions/{session_id}/commands/compose`

### 15.2 Read endpoints

- `GET /sessions/{session_id}/snapshot`
- `GET /sessions/{session_id}/events?after_domain_version=...&after_compose_version=...`
- `GET /sessions/{session_id}/artifacts/types/{type_id}`
- `GET /sessions/{session_id}/artifacts/types/{type_id}?at=1d`
- `GET /sessions/{session_id}/artifacts/designs/{design_id}`
- `GET /sessions/{session_id}/artifacts/designs/{design_id}?at=5h`
- `GET /sessions/{session_id}/ws`

### 15.3 Historical artifact response metadata

Historical artifact responses should include:

- `artifact_id`
- `artifact_kind`
- `as_of_version`
- `as_of_committed_at`
- `requested_lookback`
- `resolved_lookback`
- `is_current`

This makes the historical view explicit and debuggable.

---

## 16. Snapshot loading and catch-up

### 16.1 Current snapshot

A current snapshot loads all current domain and compose rows:

- domain rows where `valid_to_version is null`
- compose rows keyed by current primary keys and not expired

### 16.2 Event catch-up

For reconnect support, provide:

- domain catch-up using `app.domain_commit` and rows changed after the client’s `after_domain_version`
- compose catch-up using current compose rows newer than `after_compose_version`

The implementation does not need a separate broker for correctness.

---

## 17. In-memory state update strategy

The session actor is the authoritative in-process mutator.

For each accepted command:

1. validate against the current in-memory state
2. compute the next in-memory state
3. persist the change transactionally
4. only after success, publish the state transition

The in-memory state should mirror the current SQL view only. Historical views are reconstructed on demand from temporal tables.

---

## 18. Indexing strategy

Required indexes:

- `session_command(session_id, client_id, request_id)` unique
- `domain_commit(session_id, committed_at)`
- `property_clock(session_id, entity_kind, entity_id, property_key)` primary key
- per temporal table:
  - `(session_id, entity_id, valid_to_version)`
  - `(session_id, valid_from_version)`
  - `(session_id, parent_id, valid_to_version)` where hierarchical lookups are common
  - `(session_id, owner_id, valid_to_version)` for owned child tables

For artifact history reads, the most important access path is:

```text
(session_id, owner_id, valid_from_version, valid_to_version)
```

for child tables such as pieces, connectors, props, and group membership.

---

## 19. Validation layers

Validation should be separated into four layers.

### 19.1 Transport validation

- UUID parsing
- enum decoding
- required JSON shape
- supported lookback token validation

### 19.2 Domain validation

- required names
- numeric ranges
- allowed combinations of flags
- connection endpoint consistency
- group membership ordering constraints

### 19.3 Referential validation

- referenced entity exists in the same session at current version
- referenced entity is active
- referenced entity belongs to the correct owner

### 19.4 Conflict validation

- touched property changed after `base_domain_version`
- policy outcome for that property
- delete policy outcome for referenced targets

---

## 20. Testing strategy

### 20.1 Unit tests in `bin.rs`

Keep a large inline `#[cfg(test)]` section with focused test helpers.

Test:

- `FieldPatch<T>` behavior
- touched-property computation
- conflict policy matrix
- lookback token parsing
- as-of version resolution
- temporal row close/open logic
- delete policy logic

### 20.2 Integration tests against PostgreSQL

Test these scenarios:

- create and patch Type
- create and patch Design
- create piece referencing type
- reject delete of referenced type
- accept delete of unreferenced type
- reconstruct design at `1min`, `1h`, `1d`, and `1y` lookbacks
- historical design read resolves historical referenced type versions
- tombstoned artifact still viewable before tombstone point
- batch create with deferred self-reference

### 20.3 Session actor tests

Test:

- idempotent retries
- stale base version rejection
- serialized command ordering
- current snapshot after restart
- compose coalescing under load

---

## 21. Recommended implementation order

### Phase 1: foundation

- create `bin.rs`
- wire `axum`, `tokio`, `sqlx`, logging, config
- create migrations for the single `app` schema
- implement `app.session`, `app.session_command`, `app.domain_commit`, `app.property_clock`

### Phase 2: current domain essentials

- implement temporal tables for `kit_row`, `type_row`, `design_row`, `piece_row`, `layer_row`, `group_row`, `group_piece_row`, `connection_row`
- implement snapshot loader for current state
- implement session actor and session directory

### Phase 3: command path

- add explicit command DTOs and domain types
- implement touched-property computation
- implement conflict policy registry
- implement transactional temporal writes

### Phase 4: compose

- add `compose_person`, `compose_cursor`, `compose_look`, and `compose_selection_piece`
- add compose versioning and coalescing
- add WebSocket push

### Phase 5: historical artifacts

- implement `as_of_version` resolution using `app.domain_commit`
- implement Type artifact reconstruction at any supported lookback
- implement Design artifact reconstruction at any supported lookback
- add response metadata describing the resolved historical point

### Phase 6: hardening

- indexes
- performance review of artifact reads
- backpressure behavior on actor mailboxes
- operational metrics and structured logs

---

## 22. Final solution summary

The service is a single Rust application implemented in one source file, `bin.rs`, backed by one PostgreSQL schema, `app`.

The key design choices are:

- one logical writer actor per session
- explicit typed commands
- strongly typed relational persistence only
- temporal domain tables with `valid_from_version` / `valid_to_version`
- tombstone deletes
- compile-time property conflict registry
- persisted compose entities with a separate `compose_version`
- historical Type and Design artifact reconstruction through `as_of_version` queries resolved from `domain_commit`

This design is intentionally conservative in moving parts and ambitious in correctness. It avoids distributed-systems overshoot, keeps the current-state model strongly typed, preserves deterministic conflict handling, supports cyclic dependencies cleanly, and makes historical artifact viewing a native capability rather than an afterthought.
