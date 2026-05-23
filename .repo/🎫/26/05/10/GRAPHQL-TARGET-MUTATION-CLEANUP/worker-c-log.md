# Worker C (target schema) — 2026-05-10

- Scope: Type/Connector/Representation, design tree, Kit-facing relay types, Side `Arc` for `piece`, Clump stub, BlueprintEdge/Connection, connector/representation connections.
- Repo MCP `repo://goals` / `search` unavailable in this agent environment; used existing ticket folder per parent instructions.

## Done

- `gql_relay`: `ConnectorConnection`/`RepresentationConnection`/`SideConnection`/`BlueprintEdge`/`BlueprintConnection` (+ `Side::compute_hash`).
- `Connector` / `Representation`: Artifact-shaped `name`/`icon`/string fields; `port` as `Option<Arc<Port>>`; attributes → `AttributeConnection`.
- `Type`: string slots for description/icon/image/unit; `connectors`/`representations`/`authors`/props/attributes/stats return relay connections; `KitGraphEngine` entity_* helpers wrapped `Some` for `Type`.
- `Side`: `piece`/`port`/`connector`/`design_piece` use `Arc`/optional `Arc`; `owner_connection` for `SideOwner`; `Connection` artifact name/description/icon + `AttributeConnection`.
- `Piece`: `replaceableBlueprints` → `BlueprintConnection`.
- `meta::Layer`: `icon` field for Artifact alignment.
- `design`: `Clump` stub object.

## Blocked / parallel

- `cargo check` still fails on unrelated in-flight gql iface / `wip_kit_scope` / subscription JSON wiring (Workers A/D regions). Worker C edits are incremental on top.
