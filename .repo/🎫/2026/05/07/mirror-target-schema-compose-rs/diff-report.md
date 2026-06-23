# Schema / target parity (2026-05-07)

- `compose/graphql/schema.graphql` and `compose/graphql/target.schema.graphql`: **identical** (SHA256 match).
- `parses_target_schema` test: **passes** (target SDL loads into dynamic schema builder).

## Pointer sweep (Phase 2)

- `Connector.port_id` → `port: RwLock<Weak<Port>>`; resolver upgrades weak only.
- `Design.piece_id_to_index` → `piece_weak_by_external_id: HashMap<Id, Weak<Piece>>` + `Vec`; `piece_by_external_id` / GraphQL `piece(id:)` use the weak map.
- `Kit.design_id_to_index` → `design_weak_by_id`; `ensure_design` / `design(id:)` use weak entries; `bind_external_design_id` slot = vec position.
- `Kit.types` hydration maintains `type_weak_by_id` for GraphQL `type(id:)`.
- `Type`: `connector_weak_by_id` / `port_weak_by_id` / `representation_weak_by_id` refreshed before `connector` / `representation` field resolution.
- `Design.connection`: inlined vec lookup (no `connection_by_id` helper).

### Deferred

- `meta::Group.piece_ids` → `Vec<Weak<Piece>>` would introduce `meta` ↔ `kit::design::piece` module cycle; left as `Vec<Id>` until groups are moved or an indirection trait is introduced.

## Relay / per-entity scaffolding (Phases 3a–3f)

Implemented via **`gql_target`**: target SDL is parsed and registered dynamically with overlays, rather than `entity_relay!` / `entity_diffs!` macros in `lib.rs` (single-file macro expansion at ~650 entity scale was superseded by this path).
