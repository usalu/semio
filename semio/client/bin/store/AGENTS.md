---
technology: semio
bundle:
 name: store
 emoji: 🏪
 description: The store bundle for semio (HTTP GraphQL + install to native KitStore).
 kind: application
---

# 🧾 Specification

## 🕸️ Systems

- **Kit HTTP service**: one long-running `semio-store` process per kit workspace. The process holds a native [`kit_store::KitStore`](https://github.com/usalu/semio/blob/main/semio/rs/lib.rs) (WIP + coordinator + optional backbone + conflict registry), in the same role as a wasm `KitStoreHandle` over a single graph, but with **GraphQL** as the command surface (aligned with `semio::kit_graphql`).

## 🛠️ Mechanisms

- **Transport**: **Axum** HTTP. **Install** with `POST /install` (JSON body: `create` / `importFile` / `importFromFolder` / `importFromZip` per [`bin.rs`](bin.rs) `InstallBody`). **GraphQL** with `POST /graphql` using the same schema as `semio::kit_graphql` — root mutation `kitStore { batch(input: KitStoreBatchInput!) { … } }` (live / session / checkpoint / alternative / backbone oneofs). **GraphiQL** at `GET /graphiql` (and browser `GET /graphql`). Optional **CORS** is permissive for local dev.
- **Binary**: `semio-store` (crate `semio-store`), `[[bin]]` in [`Cargo.toml`](Cargo.toml). Entry, server, and tests are consolidated in [`bin.rs`](bin.rs).
- **Control plane**: `execute_with_control_plane` sets `GraphQlVcsOverride { native: Some(store) }` so backbone / coordinator commands run through the real `KitStore::execute` (not only the in-graph actor path). A per-graph **actor** still serializes `ChangeKitCommands` and undo/redo.
- **Lifecycle**: the first successful `POST /install` wins; a second install returns `409`. Shutdown: `POST /server/shutdown` or process signal.
- **Events**: unless `SEMIO_STORE_NO_EVENTS` is set, a background thread logs kit events to `tracing` (target `semio_store_event`).

## 📛 Entities

- Wire DTOs and command enums are the same [`serde` types as `semio`](../rs/lib.rs) (`KitFullDto`, `ChangeKitCommand` as the `ChangeKitCommand` GraphQL scalar, `KitStoreBatchInput`, …). There is no second RPC schema.
