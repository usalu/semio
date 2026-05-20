---
technology: semio
bundle:
 name: net
 emoji: 🔷
 description: The net bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

- **Rust kit store (sidecar)**: [`semio/store`](../../store) — `semio-store` (HTTP `POST /install` + `POST /graphql`, same wire as [`semio/js`](../js)).
  - **GraphQL documents**: [`Semio/Store/StoreGraphql.cs`](Semio/Store/StoreGraphql.cs) (`StoreGraphqlWire` + `StoreGraphql` + `StoreGraphqlJson`).
  - **Reads**: `session { stores { edges { cursor node { wip { theKit { kit { … } } } } } } } }` — never `Query.store` (RS-only test helper).
  - **Mutations**: `session { store(id: $storeId) { theKit { startNewChange | unsavedChange(id:) { kit { … } } } } } }` with `Response` selection (`ok`, `errors`, `result { … on IdResult { value } }`).
  - **Store id**: `stores.edges[].cursor` (relay cursor, typically `e0`), passed as `store(id:)` on mutations.
  - **Session API**: [`StoreSession`](Semio/Store/StoreClient.cs) (`OpenHttp`, `StartNewChange`, `RenameKit`, `ReadWipMaterialization`). Legacy alias: [`KitStoreSession`](Semio/Store/KitStore.cs).
  - **I/O**: [`StoreKitIO.cs`](Semio/Store/StoreKitIO.cs) validates install/import via GraphQL materialized `wip.theKit.kit.name`, then reads/writes JSON locally.
  - **Tests**: [`StoreClientTests.cs`](../Semio.Tests/StoreClientTests.cs) (wire + rename materialization roundtrip). Build `cargo build -p semio-store --release` before live sidecar tests.

## 📛 Entities
