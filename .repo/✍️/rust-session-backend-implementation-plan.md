# Implementation Plan: Rust Session-Backend Service

## 1. Scope and hard constraints

This plan assumes the backend is a **collaborative session service** for one session-scoped root object (`Kit`) with nested entities such as `Type`, `Design`, `Piece`, `Connection`, `Group`, `Layer`, `Prop`, `Port`, `Quality`, `File`, `Folder`, `Author`, `Tag`, and `Concept`.

The uploaded schema shows:

- a `Kit` root containing `types`, `designs`, `tags`, `concepts`, `ports`, `qualities`, `files`, `folders`, and `authors`
- `Design` containing `pieces`, `connections`, `stats`, `props`, `layers`, and `groups`
- `Piece` referencing both `Design` and `Type`
- several self-references (`Type.parent`, `Design.parent`, `Folder.parent`)
- sparse diff inputs where most changed properties are represented by nullable fields

This plan is intentionally shaped by your requirements:

- **maximum type safety**
- **proper relational SQL**
- **no JSON state storage**
- **no peer-to-peer or multi-writer design**
- **exactly one logical writer per session on the server**
- **support for cyclic dependencies**
- **presentational / ephemeral data stored as first-class semio entities**
- **no backward-compatibility constraints**

## 2. Recommended top-level architecture

Build a **single Rust service** with four internal subsystems:

1. **API layer**
   - HTTP for commands and snapshots
   - WebSocket (or SSE) for live updates
   - authentication / authorization
   - connection/session attachment

2. **Session runtime**
   - exactly one `SessionActor` task per active session
   - sole writer for that session
   - keeps typed in-memory state
   - serializes command application and persistence

3. **Persistence layer**
   - PostgreSQL only
   - fully normalized relational schema
   - append-only typed history tables
   - typed semio tables for ephemeral/presentational state

4. **Read / stream layer**
   - loads full typed session snapshot
   - broadcasts accepted domain and semio changes
   - supports reconnect and catch-up by version

This is a **modular monolith**, not a microservice system.

That is the right level of ambition here: one deployable, one database, one writer task per session, one strongly typed domain model.

---

## 3. Hard design decisions

### 3.1 One writer actor per session

Every mutation for a session goes through one in-process actor:

```text
frontend -> API -> SessionDirectory -> SessionActor(session_id) -> PostgreSQL
```

Properties:

- commands are processed strictly in arrival order at the actor
- no concurrent writes inside a session
- conflict resolution is deterministic
- no distributed consensus is needed
- no row-level race handling is pushed into business logic

A session actor is a **logical process**, implemented as a Tokio task with an inbox.

### 3.2 PostgreSQL as the only durable store

Use PostgreSQL for:

- canonical domain state
- typed history / audit records
- semio/presentational state
- session metadata
- command idempotency
- reconnect/catch-up

Do **not** introduce Redis for core correctness. You do not need it for a clean first implementation.

### 3.3 Strongly typed relational model, not document storage

Do **not** store the session or diffs as JSON blobs.

Use:

- one table per entity
- proper join tables for many-to-many relations
- typed history tables
- explicit foreign keys
- explicit enums / check constraints
- strong Rust newtypes for IDs

### 3.4 Explicit command model, not sparse GraphQL diff objects internally

The uploaded GraphQL diff model is useful as a description of change shape, but it is not the right internal command model because sparse nullable inputs make these three states hard to distinguish cleanly:

- field not touched
- field set to a value
- optional field explicitly cleared to `null`

For the new backend, use explicit commands and explicit field operations.

### 3.5 Soft deletion with tombstones, not physical deletion in the write path

To support concurrent stale writes and cyclic references safely:

- domain deletes become **tombstones**
- reads hide tombstoned rows
- cleanup is a later administrative operation
- references to tombstoned rows are rejected (unless a property explicitly allows nulling or placeholder substitution)

---

## 4. Session boundary and ownership model

The **session** is the consistency boundary.

Everything mutable in a collaborative editing flow is scoped by `session_id`.

That means all canonical rows should include `session_id`, for example:

- `type_entity(session_id, type_id, ...)`
- `design_entity(session_id, design_id, ...)`
- `piece_entity(session_id, piece_id, ...)`

This gives you:

- strict isolation between sessions
- simple actor ownership
- simple conflict/version tracking
- simple SQL partitioning/indexing strategy later if needed

A session can optionally be associated with a business identity such as `kit_id`, but the write model is session-scoped.

---

## 5. Rust project layout

Use a single workspace:

```text
backend/
  crates/
    app/               # binary
    api/               # HTTP/WS handlers, DTOs
    session_runtime/   # session directory + actor runtime
    domain/            # entity types, IDs, invariants, commands, conflict policies
    persistence/       # sqlx repositories, migrations, row mapping
    semio/             # presence/cursor/look domain
    auth/              # authn/authz
    observability/     # tracing, metrics
    test_support/      # fixtures, builders, integration helpers
```

### 5.1 Core libraries

Use:

- `tokio` for async runtime
- `axum` for HTTP + WebSocket
- `sqlx` with compile-time checked queries for PostgreSQL
- `serde` for API serialization
- `uuid` for stable IDs
- `time` for timestamps
- `thiserror` for domain and application errors
- `tracing` + `tracing-subscriber` for structured logs

### 5.2 Type-safety rules in Rust

Use newtypes for every domain ID:

```rust
pub struct SessionId(Uuid);
pub struct KitId(Uuid);
pub struct TypeId(Uuid);
pub struct DesignId(Uuid);
pub struct PieceId(Uuid);
pub struct GroupId(Uuid);
pub struct ConnectionId(Uuid);
pub struct PortId(Uuid);
pub struct QualityId(Uuid);
```

Do not use raw `Uuid` or `String` in domain APIs after the transport boundary.

Model all mutable fields with explicit patch semantics:

```rust
pub enum FieldPatch<T> {
    NoChange,
    Set(T),
    Clear,
}
```

For non-nullable fields, use:

```rust
pub enum RequiredFieldPatch<T> {
    NoChange,
    Set(T),
}
```

This removes ambiguity and is substantially safer than sparse nullable diff objects.

---

## 6. Domain model strategy

## 6.1 Current-state model

Maintain a fully typed in-memory session state inside the actor:

```rust
pub struct SessionState {
    pub session_id: SessionId,
    pub version: DomainVersion,
    pub semio_version: SemioVersion,
    pub kit: KitState,
    pub types: BTreeMap<TypeId, TypeState>,
    pub designs: BTreeMap<DesignId, DesignState>,
    pub pieces: BTreeMap<PieceId, PieceState>,
    pub groups: BTreeMap<GroupId, GroupState>,
    pub connections: BTreeMap<ConnectionId, ConnectionState>,
    // ...
    pub semio_people: BTreeMap<PersonId, SemioPersonState>,
}
```

The actor loads this from SQL on session activation.

Given the schema size (around 23 entity kinds, max composition depth 4), loading the full session into memory is reasonable and simplifies correctness.

## 6.2 Entity identity versus entity state

For each entity kind, distinguish:

- **identity**
- **live state**
- **lifecycle**

Example:

```rust
pub enum Lifecycle {
    Active,
    Tombstoned { at: DomainVersion, by: CommandId },
}
```

Do not physically remove rows during ordinary mutation processing.

---

## 7. SQL schema design

Use separate PostgreSQL schemas:

- `runtime` — session metadata and versions
- `core` — current canonical domain state
- `history` — append-only typed audit/change records
- `semio` — ephemeral/presentational state
- `auth` — optional session membership / roles

## 7.1 Session runtime tables

### `runtime.session`
Tracks session lifecycle.

Columns:

- `session_id uuid primary key`
- `root_kit_id uuid not null`
- `domain_version bigint not null default 0`
- `semio_version bigint not null default 0`
- `status session_status not null`
- `writer_instance_id uuid null`
- `writer_lease_expires_at timestamptz null`
- `created_at timestamptz not null`
- `updated_at timestamptz not null`

### `runtime.session_command`
Idempotency and command envelope.

Columns:

- `command_id uuid primary key`
- `session_id uuid not null`
- `client_id uuid not null`
- `request_id uuid not null`
- `base_domain_version bigint not null`
- `accepted_domain_version bigint null`
- `accepted_semio_version bigint null`
- `command_kind command_kind not null`
- `actor_person_id uuid not null`
- `received_at timestamptz not null`
- `applied_at timestamptz null`
- `status command_status not null`
- unique `(session_id, client_id, request_id)`

This table lets the server safely accept retries.

### `runtime.property_clock`
The key table for property-level conflict resolution.

Columns:

- `session_id uuid not null`
- `entity_kind entity_kind not null`
- `entity_id uuid not null`
- `property_key property_key not null`
- `last_changed_domain_version bigint not null`
- `last_command_id uuid not null`
- primary key `(session_id, entity_kind, entity_id, property_key)`

This table says: “what is the latest accepted domain version that changed this exact property?”

That means conflict checks do not need to replay an event log to learn whether a field changed after a client’s base version.

### `runtime.session_subscription` (optional)
For live connections.

Columns:

- `connection_id uuid primary key`
- `session_id uuid not null`
- `person_id uuid not null`
- `frontend_id text not null`
- `connected_at timestamptz not null`
- `last_seen_at timestamptz not null`

---

## 7.2 Canonical domain tables (`core` schema)

The exact table count will be high because the schema is rich and type-safe by design. That is acceptable.

Below is the recommended pattern.

### Root and top-level entities

#### `core.kit`
- `session_id uuid not null`
- `kit_id uuid not null`
- `name text not null`
- `version text null`
- `remote text null`
- `homepage text null`
- `license text null`
- `preview text null`
- `icon text null`
- `image text null`
- `description text null`
- `created_at timestamptz null`
- `updated_at timestamptz null`
- `lifecycle lifecycle_status not null default 'active'`
- primary key `(session_id, kit_id)`

#### `core.author`
- `session_id`
- `author_id`
- `name text not null`
- `email text not null`
- `lifecycle`
- PK `(session_id, author_id)`

#### `core.location`
- `session_id`
- `location_id`
- `longitude double precision not null`
- `latitude double precision not null`
- `altitude double precision null`
- `lifecycle`
- PK `(session_id, location_id)`

#### `core.folder`
- `session_id`
- `folder_id`
- `name text not null`
- `parent_folder_id uuid null`
- `description text null`
- created/updated columns
- `lifecycle`
- PK `(session_id, folder_id)`
- FK `(session_id, parent_folder_id)` -> `core.folder`
- mark FK as `DEFERRABLE INITIALLY DEFERRED`

#### `core.file`
- `session_id`
- `file_id`
- `name text not null`
- `remote text null`
- `folder_id uuid null`
- `size_bytes bigint null`
- `hash text null`
- `blob_ref text null`
- created/updated/by columns
- `lifecycle`
- PK `(session_id, file_id)`
- FK to folder

#### `core.tag`, `core.concept`, `core.port`, `core.quality`, `core.benchmark`
Use direct entity tables with typed scalar columns.

### Important relationships

#### `core.port_compatibility`
- `session_id`
- `port_id`
- `compatible_port_id`
- PK `(session_id, port_id, compatible_port_id)`

#### `core.quality_benchmark`
- `session_id`
- `quality_id`
- `benchmark_id`
- PK `(session_id, quality_id, benchmark_id)`

### Type/model/connector hierarchy

#### `core.type_entity`
- `session_id`
- `type_id`
- `name text not null`
- `parent_type_id uuid null`
- `is_abstract boolean null`
- `folder text null`
- `stock integer null`
- `virtual boolean null`
- `unit text null`
- `location_id uuid null`
- `icon text null`
- `image text null`
- `description text null`
- created/updated columns
- `lifecycle`
- PK `(session_id, type_id)`
- self-FK on parent is deferred

#### `core.type_author`
- `session_id`
- `type_id`
- `author_id`
- PK `(session_id, type_id, author_id)`

#### `core.type_concept`
- `session_id`
- `type_id`
- `concept_id`
- PK `(session_id, type_id, concept_id)`

#### `core.model`
- `session_id`
- `model_id`
- `type_id`
- `name text null`
- `file_id uuid not null`
- `description text null`
- `lifecycle`
- PK `(session_id, model_id)`

#### `core.model_tag`
- `session_id`
- `model_id`
- `tag_id`
- PK `(session_id, model_id, tag_id)`

#### `core.connector`
- `session_id`
- `connector_id`
- `type_id`
- `name text null`
- `t double precision not null`
- `point_x double precision not null`
- `point_y double precision not null`
- `point_z double precision not null`
- `direction_x double precision not null`
- `direction_y double precision not null`
- `direction_z double precision not null`
- `description text null`
- `port_id uuid null`
- `mandatory boolean null`
- `lifecycle`
- PK `(session_id, connector_id)`

### Prop and attribute strategy

#### `core.prop`
- `session_id`
- `prop_id`
- `quality_id uuid not null`
- `value text not null`
- `unit text null`
- `owner_kind prop_owner_kind not null`
- `owner_id uuid not null`
- `lifecycle`
- PK `(session_id, prop_id)`

A single typed `prop` table works well because ownership varies (`Type`, `Piece`, `Connector`, `Design`).

#### `core.attribute`
- `session_id`
- `attribute_id uuid not null`
- `key text not null`
- `value text null`
- `definition text null`
- `owner_kind attribute_owner_kind not null`
- `owner_id uuid not null`
- `lifecycle`
- PK `(session_id, attribute_id)`

This is still proper SQL and strongly typed because owner kind is explicit.

### Design and nested entities

#### `core.design`
- `session_id`
- `design_id`
- `name text not null`
- `parent_design_id uuid null`
- `is_abstract boolean null`
- `folder text null`
- `active_layer_id uuid null`
- `can_scale boolean null`
- `can_mirror boolean null`
- `unit text null`
- `location_id uuid null`
- `icon text null`
- `image text null`
- `description text null`
- created/updated columns
- `lifecycle`
- PK `(session_id, design_id)`
- self-FK deferred

#### `core.design_author`
- `session_id`
- `design_id`
- `author_id`
- PK `(session_id, design_id, author_id)`

#### `core.design_concept`
- `session_id`
- `design_id`
- `concept_id`
- PK `(session_id, design_id, concept_id)`

#### `core.layer`
- `session_id`
- `layer_id`
- `design_id uuid not null`
- `path text not null`
- `is_hidden boolean null`
- `is_locked boolean null`
- `color text null`
- `description text null`
- `lifecycle`
- PK `(session_id, layer_id)`

#### `core.piece`
- `session_id`
- `piece_id`
- `design_id uuid not null`
- `name text null`
- `type_id uuid null`
- `plane_origin_x double precision null`
- `plane_origin_y double precision null`
- `plane_origin_z double precision null`
- `plane_x_axis_x double precision null`
- `plane_x_axis_y double precision null`
- `plane_x_axis_z double precision null`
- `plane_y_axis_x double precision null`
- `plane_y_axis_y double precision null`
- `plane_y_axis_z double precision null`
- `center_u double precision null`
- `center_v double precision null`
- `scale double precision null`
- `mirror_plane_origin_x double precision null`
- `mirror_plane_origin_y double precision null`
- `mirror_plane_origin_z double precision null`
- `mirror_plane_x_axis_x double precision null`
- `mirror_plane_x_axis_y double precision null`
- `mirror_plane_x_axis_z double precision null`
- `mirror_plane_y_axis_x double precision null`
- `mirror_plane_y_axis_y double precision null`
- `mirror_plane_y_axis_z double precision null`
- `is_hidden boolean null`
- `is_locked boolean null`
- `color text null`
- `description text null`
- `lifecycle`
- PK `(session_id, piece_id)`

#### `core.group_entity`
- `session_id`
- `group_id`
- `design_id uuid not null`
- `color text null`
- `name text null`
- `description text null`
- `lifecycle`
- PK `(session_id, group_id)`

#### `core.group_piece`
- `session_id`
- `group_id`
- `piece_id`
- `ordinal integer not null`
- PK `(session_id, group_id, piece_id)`

#### `core.connection`
- `session_id`
- `connection_id`
- `design_id uuid not null`
- `connected_piece_id uuid not null`
- `connected_design_piece_id uuid null`
- `connected_connector_id uuid null`
- `connecting_piece_id uuid not null`
- `connecting_design_piece_id uuid null`
- `connecting_connector_id uuid null`
- `gap double precision null`
- `shift double precision null`
- `rise double precision null`
- `rotation double precision null`
- `turn double precision null`
- `tilt double precision null`
- `u double precision null`
- `v double precision null`
- `description text null`
- `lifecycle`
- PK `(session_id, connection_id)`

#### `core.stat`
- `session_id`
- `stat_id`
- `design_id uuid not null`
- `quality_id uuid not null`
- `unit text null`
- `min double precision null`
- `min_excluded boolean null`
- `max double precision null`
- `max_excluded boolean null`
- `lifecycle`
- PK `(session_id, stat_id)`

### Tombstone columns

Every mutable entity table should have at least:

- `lifecycle lifecycle_status not null`
- `deleted_at_domain_version bigint null`
- `deleted_by_command_id uuid null`

Use `lifecycle_status` enum:

- `active`
- `tombstoned`

Queries for current state always filter `lifecycle = 'active'`.

---

## 7.3 Typed history tables (`history` schema)

The history model should be append-only and queryable without reconstructing from JSON.

### `history.command`
- `command_id uuid primary key`
- `session_id uuid not null`
- `base_domain_version bigint not null`
- `accepted_domain_version bigint null`
- `actor_person_id uuid not null`
- `command_kind command_kind not null`
- `recorded_at timestamptz not null`

### `history.entity_create`
- `session_id`
- `command_id`
- `entity_kind`
- `entity_id`
- PK `(session_id, command_id, entity_kind, entity_id)`

### `history.entity_delete`
- `session_id`
- `command_id`
- `entity_kind`
- `entity_id`
- `delete_reason delete_reason not null`
- PK `(session_id, command_id, entity_kind, entity_id)`

### `history.scalar_change`
For simple scalar properties.

Columns:

- `session_id`
- `command_id`
- `entity_kind`
- `entity_id`
- `property_key`
- typed before/after columns:
  - `before_text text null`, `after_text text null`
  - `before_bool boolean null`, `after_bool boolean null`
  - `before_int bigint null`, `after_int bigint null`
  - `before_float double precision null`, `after_float double precision null`
  - `before_uuid uuid null`, `after_uuid uuid null`
  - `before_timestamptz timestamptz null`, `after_timestamptz timestamptz null`
- one `value_kind` enum column
- check constraints ensure only the correct typed pair is populated

### `history.coord_change`
For `Coord` properties such as `Piece.center` or semio cursor.

### `history.point_change`
For `Point` properties.

### `history.vector_change`
For `Vector` properties.

### `history.plane_change`
For `Plane` properties.

### `history.camera_change`
For `Camera` properties such as semio look.

### `history.membership_add`
For many-to-many and ordered membership edges.

Columns:

- `session_id`
- `command_id`
- `edge_kind`
- `owner_id`
- `member_id`
- `ordinal integer null`

### `history.membership_remove`
Same shape as add.

This history design is deliberately boring and explicit. That is good. It is easy to inspect, index, debug, and reason about.

---

## 7.4 Semio tables (`semio` schema)

Semio data is ephemeral in meaning, but still persisted as typed state.

It should **not** share the domain version clock because cursor movement must not create domain conflicts.

Use a separate `semio_version` per session.

### `semio.person`
- `session_id uuid not null`
- `person_id uuid not null`
- `display_name text null`
- `color text null`
- `frontend_id text not null`
- `is_present boolean not null`
- `last_seen_at timestamptz not null`
- `expires_at timestamptz not null`
- primary key `(session_id, person_id, frontend_id)`

### `semio.cursor`
- `session_id`
- `person_id`
- `frontend_id`
- `u double precision not null`
- `v double precision not null`
- `updated_at timestamptz not null`
- `semio_version bigint not null`
- PK `(session_id, person_id, frontend_id)`

### `semio.look`
- `session_id`
- `person_id`
- `frontend_id`
- `position_x`, `position_y`, `position_z`
- `forward_x`, `forward_y`, `forward_z`
- `up_x`, `up_y`, `up_z`
- `updated_at`
- `semio_version`
- PK `(session_id, person_id, frontend_id)`

### `semio.selection_piece`
- `session_id`
- `person_id`
- `frontend_id`
- `piece_id`
- PK `(session_id, person_id, frontend_id, piece_id)`

### `semio.selection_design`
- `session_id`
- `person_id`
- `frontend_id`
- `design_id`
- PK `(session_id, person_id, frontend_id, design_id)`

You can add more semio entities later (viewport, hovered entity, drag state, active tool), but keep the same pattern:

- typed table
- typed columns
- session scoped
- semio versioned
- expiry aware
- not part of canonical domain conflict resolution

---

## 8. Conflict resolution model

## 8.1 Why this can be simple

Because there is exactly one server-side writer actor per session, the hard problem is **not** concurrent DB mutation.

The hard problem is:

- commands arrive with stale `base_domain_version`
- different properties need different merge behavior
- references may be deleted while another client creates new referencing entities

The clean design is:

1. client sends command with `base_domain_version`
2. actor computes touched property set
3. actor checks `runtime.property_clock` for any touched property changed after `base_domain_version`
4. actor applies per-property conflict policies
5. actor persists result + history + property clocks in one SQL transaction
6. actor increments session version and broadcasts outcome

## 8.2 Compile-time property registry

Create a Rust enum covering all mutable properties:

```rust
pub enum PropertyKey {
    KitName,
    KitVersion,
    TypeName,
    TypeParent,
    TypeIsAbstract,
    TypeFolder,
    TypeStock,
    TypeVirtual,
    TypeUnit,
    TypeLocation,
    TypeIcon,
    TypeImage,
    TypeDescription,
    DesignName,
    DesignParent,
    DesignActiveLayer,
    PieceType,
    PiecePlane,
    PieceCenter,
    PieceScale,
    PieceMirrorPlane,
    GroupPieces,
    ConnectionConnectedSide,
    ConnectionConnectingSide,
    // ...
}
```

Create a compile-time mapping:

```rust
pub struct ConflictSpec {
    pub policy: ConflictPolicy,
    pub reference_behavior: ReferenceBehavior,
}

pub fn conflict_spec(key: PropertyKey) -> ConflictSpec { ... }
```

This is far safer than storing executable policy definitions in the database.

## 8.3 Conflict policies

Use a small fixed set of policy kinds:

- `RejectIfChanged`
- `LastWriterWins`
- `AdditiveNumeric`
- `ReplaceSet`
- `UnionSet`
- `OrderedMembershipReplace`
- `ReferenceMustExistAndBeActive`
- `ReferenceMayBecomeNull`
- `TombstoneAwareReject`
- `SemioLastWriterWins`

Examples:

- `Kit.name` -> `RejectIfChanged`
- `Type.description` -> `LastWriterWins`
- `Group.pieces` -> `OrderedMembershipReplace`
- `Piece.type_id` -> `ReferenceMustExistAndBeActive`
- `semio.cursor` -> `SemioLastWriterWins`
- `semio.look` -> `SemioLastWriterWins`

## 8.4 Property clocks

After a command changes a property, update:

```text
(session_id, entity_kind, entity_id, property_key) -> current_domain_version
```

If a new command comes in with base version 120 and `Piece.type_id` was changed at version 123, the actor sees that immediately.

## 8.5 Command outcome model

Do not return only “success/failure”.

Return structured outcomes:

- accepted fully
- accepted with merged fields
- rejected with conflicts
- partially accepted only if your product explicitly wants that behavior

For the first implementation, keep it simple:

- domain commands are **atomic**
- semio commands may be coalesced but are individually acknowledged

That means one domain command either commits entirely or fails entirely.

---

## 9. Cyclic dependency support

This is one of the most important parts of the design.

### 9.1 What cyclic support means here

From your example:

- kit contains types and designs
- designs contain pieces
- pieces reference types

There are also self-references like:

- `Type.parent -> Type`
- `Design.parent -> Design`
- `Folder.parent -> Folder`

You may also have command bundles where multiple new entities reference each other in the same request.

### 9.2 Two-phase apply inside one SQL transaction

When a command creates or updates multiple interdependent entities, apply in this order:

1. **Reserve identities**
   - insert missing entity rows with minimal required columns and `lifecycle = active`
   - this makes IDs exist before all references are connected

2. **Apply scalar fields**
   - names, booleans, numbers, text, timestamps, geometry columns

3. **Apply references and membership edges**
   - parent pointers
   - piece -> type
   - design -> active layer
   - group -> piece memberships
   - connection side references

4. **Run semantic validation**
   - referenced targets exist
   - referenced targets are active, not tombstoned
   - no prohibited ownership crossings
   - no invalid layer/design mismatches
   - no duplicate membership/order violations

5. **Write history + property clocks + bump version**
6. **Commit**

### 9.3 Deferred foreign keys

Any FK involved in same-transaction cyclic or self-referential creation should be created as:

- `DEFERRABLE INITIALLY DEFERRED`

That allows the transaction to insert mutually dependent rows before final FK validation at commit.

### 9.4 Delete semantics for cyclic safety

Never hard-delete in the main write path.

Instead:

- mark target as tombstoned
- reject future references to tombstoned targets
- keep old history and stale references inspectable
- optionally run offline cleanup when safe

### 9.5 The exact example: delete type while another client creates a design with a piece using that type

Suppose:

- client A deletes `Type T`
- client B, based on an older version, creates `Design D` and `Piece P(type = T)`

Because there is one writer actor, these commands are serialized.

#### Case 1: delete arrives first

1. actor tombstones `Type T`
2. property clocks for `Type.lifecycle` are updated
3. actor later processes the create-design command
4. semantic validation sees `Piece.type_id = T`, but `T` is tombstoned
5. policy for `Piece.type_id` is `ReferenceMustExistAndBeActive`
6. command is rejected atomically with a structured conflict:

```text
conflict:
  property = Piece.type_id
  target = Type(T)
  reason = target_tombstoned_after_base_version
```

No partial design is created.

#### Case 2: create-design arrives first

1. actor creates `Design D` and `Piece P(type = T)`
2. actor later processes delete-type command
3. delete-type validation finds active references from `Piece P`
4. policy for deleting a referenced type should be:
   - either reject delete
   - or require explicit force-delete plus cascading null/reject semantics

Recommended default:

- **reject delete while active references exist**

That is the cleanest invariant-preserving rule.

### 9.6 Delete policy matrix

For referenced entities such as `Type`, `Quality`, `Port`, `File`, `Folder`, and `Layer`, choose one of these delete policies explicitly:

- `RejectIfReferenced`
- `TombstoneAndNullifyReferences`
- `TombstoneAndCascadeDeleteChildren`
- `TombstoneAndKeepDangling` (**do not use**)

Recommended defaults:

- `Type` -> `RejectIfReferenced`
- `Quality` -> `RejectIfReferenced`
- `Port` -> `RejectIfReferenced`
- `Layer` -> `RejectIfReferenced` if active references exist
- `Folder` -> `RejectIfReferenced` or explicit subtree delete command
- `Design` -> `CascadeDeleteOwnedChildren`
- `Group` -> `CascadeDeleteMemberships`
- `Connection` -> direct tombstone allowed

---

## 10. Command model

Do not expose “apply arbitrary sparse diff to giant object” as the authoritative write contract.

Instead use explicit semantic commands.

## 10.1 Domain commands

Examples:

```rust
pub enum DomainCommand {
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
    // ...
}
```

Each command contains:

- `command_id`
- `client_id`
- `request_id`
- `actor_person_id`
- `base_domain_version`
- payload

## 10.2 Batch command support

You will need batch commands for transactional multi-entity changes:

```rust
pub struct DomainBatch {
    pub commands: Vec<DomainCommand>,
}
```

The whole batch is atomic and processed in one actor turn and one SQL transaction.

That is how you support “create design + create pieces + set references” cleanly.

## 10.3 Semio commands

Examples:

```rust
pub enum SemioCommand {
    UpsertPresence(UpsertPresence),
    SetCursor(SetCursor),
    SetLook(SetLook),
    ReplacePieceSelection(ReplacePieceSelection),
    ReplaceDesignSelection(ReplaceDesignSelection),
    ClearPresence(ClearPresence),
}
```

These use `base_semio_version`, not `base_domain_version`.

---

## 11. API design

## 11.1 Write API

Use HTTP JSON commands. Keep transport boring.

### Endpoints

- `POST /sessions`
- `POST /sessions/{session_id}/attach`
- `GET /sessions/{session_id}/snapshot`
- `POST /sessions/{session_id}/commands/domain`
- `POST /sessions/{session_id}/commands/semio`
- `GET /sessions/{session_id}/events?after_domain_version=...&after_semio_version=...`
- `GET /sessions/{session_id}/ws`

### Why not GraphQL mutations as the main write path?

Because the main difficulty is deterministic patch semantics and conflict handling, not nested query flexibility.

You can still add:

- GraphQL read API later
- generated read adapters later

But the **authoritative write path** should be explicit commands.

## 11.2 Read API

Provide:

- full session snapshot
- lightweight summary endpoints
- catch-up events since version N
- live stream on WebSocket

The snapshot should include:

- full canonical domain state
- current `domain_version`
- current semio snapshot
- current `semio_version`

## 11.3 Live stream protocol

Broadcast three event categories:

- `domain_command_accepted`
- `domain_command_rejected`
- `semio_updated`

For reconnect:

1. client reconnects with `last_domain_version` and `last_semio_version`
2. server returns missed events from history/current semio tables
3. client resynchronizes deterministically

---

## 12. Session actor implementation

## 12.1 Session directory

A process-global registry maps `session_id` to active actor handle:

```rust
DashMap<SessionId, SessionHandle>
```

A `SessionHandle` contains:

- command sender
- broadcast sender
- liveness / refcount metadata

## 12.2 Actor lifecycle

### Activation
- load session metadata
- load canonical tables into `SessionState`
- load semio state
- start actor loop

### Active processing
- receive domain and semio commands
- coalesce high-frequency semio updates
- persist accepted changes
- broadcast outcomes

### Passivation
- after inactivity timeout
- actor flushes pending semio writes
- drops in-memory state
- session remains fully reconstructable from SQL

## 12.3 Single-writer guarantee

Within one service instance, the directory guarantees one actor per session.

If you later need multi-instance deployment, add a DB lease on `runtime.session.writer_instance_id`, but do not build that now unless you actually need it.

For the initial system:

- one service instance
- one actor per active session
- one DB
- one source of truth

That is enough.

---

## 13. Persistence flow per accepted domain command

Inside one SQL transaction:

1. verify idempotency (`runtime.session_command`)
2. load current session version row `FOR UPDATE`
3. compare `base_domain_version` against `runtime.property_clock` for touched properties
4. execute conflict policies
5. apply canonical row changes in `core.*`
6. write `history.command`
7. write `history.entity_create` / `history.entity_delete` / `history.*_change`
8. upsert `runtime.property_clock`
9. increment `runtime.session.domain_version`
10. mark `runtime.session_command` accepted
11. commit

Only after commit:

12. update in-memory state if not already mutated as source of truth
13. broadcast accepted event to subscribers

If commit fails, broadcast nothing.

---

## 14. Semio persistence flow

Semio is high-frequency, so treat it differently while keeping one writer actor.

### 14.1 Coalescing

Inside the actor, coalesce repeated semio updates per `(person_id, frontend_id)`:

- cursor: keep latest only
- look/camera: keep latest only
- selection: keep latest full replacement only

Flush at a fixed cadence such as every 50-100 ms, or immediately on important transitions.

### 14.2 Separate semio version

Domain state and semio state must not share the same version counter.

Use:

- `domain_version` for canonical data
- `semio_version` for presence/presentation state

This prevents cursor chatter from invalidating domain commands.

### 14.3 Expiry

Each semio row has `expires_at`.

A periodic cleanup task:

- marks stale presence as absent
- removes stale selections/cursor/look if needed
- emits semio updates

Persistence rule:

- semio survives process restarts
- semio expires automatically
- semio is queryable and streamable
- semio does not participate in domain referential invariants except for obvious FK validity where needed

---

## 15. Validation rules

Validation should be split into four layers.

## 15.1 Transport validation
- required fields
- enum decoding
- UUID parsing
- command shape

## 15.2 Domain validation
- names not empty where required
- numeric ranges
- ownership consistency
- no impossible connection side combinations

## 15.3 Referential validation
- referenced row exists in same session
- referenced row is active
- referenced row belongs to correct owning design/type

## 15.4 Conflict validation
- touched properties changed after base version?
- if yes, apply property policy
- if policy says reject, reject atomically

---

## 16. Testing strategy

## 16.1 Unit tests
Cover:

- command decoding
- property touched-set computation
- conflict policy application
- delete policy checks
- semio coalescing
- domain invariants

## 16.2 SQL integration tests
For each entity kind:

- create
- update
- tombstone
- reject invalid reference
- reject invalid delete
- load snapshot

For cyclic behavior:

- self-parent creation in one batch
- cross-reference creation in one batch
- deferred FK validation

## 16.3 Session actor tests
Use deterministic actor tests for:

- idempotent retry
- stale command rejection
- conflict merge outcomes
- command ordering
- snapshot + catch-up after restart

## 16.4 Concurrency tests
Even with one writer actor, test:

- many API callers racing to same session
- actor mailbox backpressure
- reconnect + replay
- semio storm behavior

## 16.5 Property-policy matrix tests
For every mutable property, add a table-driven test asserting:

- touched property key
- conflict policy
- behavior on stale base version
- behavior on target tombstone

This is one of the highest-value test suites in the system.

---

## 17. Observability

Instrument every command with:

- `session_id`
- `command_id`
- `command_kind`
- `actor_person_id`
- `base_domain_version`
- `accepted_domain_version`
- `conflict_count`
- `duration_ms`

Metrics:

- active session actors
- actor mailbox depth
- domain commands/sec
- semio commands/sec
- conflict rejects/sec
- idempotent retries/sec
- snapshot load time
- SQL transaction duration
- semio flush batch size

Logs should never be the only source of truth. History tables remain the audit source.

---

## 18. Security and authorization

Keep auth simple:

- authenticate frontend/user
- authorize session membership
- map authenticated principal to `person_id`
- record `actor_person_id` on every command

Add role checks if needed:

- viewer
- editor
- owner

Semio writes require at least session attachment; domain writes require editor role.

---

## 19. Migration from the uploaded GraphQL schema

Because you explicitly want a clean new solution, do not preserve the current GraphQL diff shape as the backend’s internal contract.

Use the uploaded schema only as a **domain inventory** and **field/reference map**.

Recommended approach:

1. inventory all entities and fields from the schema
2. define Rust domain structs and commands from scratch
3. define SQL tables from domain ownership and references
4. define read DTOs separately
5. only after that, decide whether to expose:
   - REST/JSON reads only
   - GraphQL reads
   - or generated typed frontend clients

The current schema is best treated as analysis input, not architecture.

---

## 20. Concrete implementation phases

## Phase 0 — Domain inventory and property map
Deliverables:

- entity catalog
- ownership map
- reference map
- mutable property list
- `PropertyKey` enum draft
- delete policy matrix draft

Acceptance criteria:

- every mutable field in the schema mapped to:
  - owning entity
  - SQL column(s)
  - property key
  - conflict policy
  - delete/reference rule

## Phase 1 — Rust scaffolding
Deliverables:

- workspace
- CI
- formatting/linting
- error model
- ID newtypes
- shared time/UUID utilities
- basic axum/sqlx app skeleton

Acceptance criteria:

- service boots
- health endpoint works
- DB migrations run
- compile-time SQL checking is enabled

## Phase 2 — Core SQL schema
Deliverables:

- `runtime`, `core`, `history`, `semio` schemas
- base enums
- canonical entity tables
- join tables
- deferred FKs where required
- essential indexes

Acceptance criteria:

- full canonical schema migrates cleanly
- all FKs and unique constraints are in place
- snapshot load queries compile and run

## Phase 3 — Session runtime
Deliverables:

- session directory
- actor lifecycle
- snapshot loader
- actor passivation
- in-memory `SessionState`

Acceptance criteria:

- one actor created per active session
- repeated attaches reuse same actor
- actor reconstructs state from DB

## Phase 4 — Domain command model
Deliverables:

- command enums
- field patch types
- transport DTOs
- touched-property computation
- command validation framework

Acceptance criteria:

- commands decode unambiguously
- touched properties are deterministic
- validation errors are structured

## Phase 5 — Conflict engine
Deliverables:

- `PropertyKey` enum
- conflict policy registry
- property clock queries
- conflict outcome formatter

Acceptance criteria:

- stale writes are detected per property
- configured policies behave correctly
- command outcomes are deterministic

## Phase 6 — Canonical persistence and history
Deliverables:

- repositories for canonical writes
- history writers
- property clock upserts
- version increment logic
- idempotency checks

Acceptance criteria:

- accepted command updates canonical tables
- history rows are written
- version increments exactly once
- retried command is idempotent

## Phase 7 — Semio subsystem
Deliverables:

- semio commands
- semio tables
- semio versioning
- coalescing logic
- expiry cleanup

Acceptance criteria:

- cursor/look/presence survive restart
- semio updates do not change domain version
- stale semio data expires correctly

## Phase 8 — API and streaming
Deliverables:

- snapshot endpoint
- domain command endpoint
- semio command endpoint
- catch-up endpoint
- WebSocket stream

Acceptance criteria:

- client can attach, load snapshot, issue commands, reconnect, and catch up

## Phase 9 — Invariant and scenario test suite
Deliverables:

- delete/reference tests
- cyclic create tests
- stale write tests
- type-delete versus piece-create tests
- restart/recovery tests

Acceptance criteria:

- all critical scenarios are automated
- no known ambiguous merge behavior remains

## Phase 10 — Performance and hardening
Deliverables:

- indexes tuned
- N+1 query review
- actor backpressure handling
- metrics dashboards
- operational runbook

Acceptance criteria:

- target session size loads within agreed SLA
- target command throughput is met
- observability is sufficient for production debugging

---

## 21. Recommended first milestone

The first production-worthy milestone should support only:

- one service instance
- one session actor per active session
- one session root (`Kit`)
- core domain entities:
  - `Type`
  - `Design`
  - `Piece`
  - `Connection`
  - `Layer`
  - `Group`
  - `Quality`
  - `Prop`
  - `Port`
- semio entities:
  - `Person`
  - `Cursor`
  - `Look`
  - `PieceSelection`

Plus these guarantees:

- typed SQL schema
- no JSON persistence
- typed commands
- per-property conflict detection
- tombstone deletes
- deterministic rejection of invalid stale references
- restart-safe semio persistence

That is enough to prove the architecture without overshooting.

---

## 22. Key rules to keep the implementation clean

1. **One session actor is the only writer.**
2. **No JSON state or JSON diff persistence.**
3. **Every mutable field has a `PropertyKey`.**
4. **Every delete has an explicit policy.**
5. **No hard deletes in normal mutation flow.**
6. **Semio has its own version clock.**
7. **Transport DTOs are not the domain model.**
8. **Batch commands are the unit for cyclic graph edits.**
9. **History is append-only and typed.**
10. **Queries hide tombstones by default.**

---

## 23. Final recommendation

Implement the service as a **single Rust application with a session actor runtime and PostgreSQL-backed normalized storage**.

The key architecture choices are:

- **session-scoped canonical state**
- **one logical writer actor per session**
- **explicit typed commands**
- **typed relational current state**
- **typed relational history**
- **property-clock-based conflict detection**
- **tombstone deletion**
- **deferred FKs for same-transaction cyclic graph creation**
- **separate semio subsystem with independent versioning**

That combination gives you:

- maximum type safety
- deterministic conflict handling
- support for cyclic references
- persisted canonical and semio state
- no unnecessary distributed systems complexity

## 24. Suggested next concrete artifacts

After this plan, the next three implementation artifacts should be produced in order:

1. **Property map spreadsheet**
   - every mutable field
   - owning entity
   - SQL columns
   - conflict policy
   - delete/reference policy

2. **SQL migration set**
   - enums
   - core tables
   - history tables
   - semio tables
   - indexes
   - deferred FKs

3. **Rust command and domain type skeleton**
   - ID newtypes
   - entity structs
   - `FieldPatch<T>`
   - `DomainCommand`
   - `SemioCommand`
   - `PropertyKey`
   - `ConflictPolicy`
