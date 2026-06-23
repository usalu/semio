---
technology: compose
bundle:
 name: net
 emoji: 🔷
 description: The net bundle for compose.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

- **Rust kit store (sidecar)**: [`compose/store`](../../store) — `compose-store` (HTTP `POST /install` + `POST /graphql`, same wire as [`compose/js`](../js)).
  - **GraphQL documents**: [`Compose/Store/StoreGraphql.cs`](Compose/Store/StoreGraphql.cs) (`StoreGraphqlWire` + `StoreGraphql` + `StoreGraphqlJson`).
  - **Reads**: `session { stores { edges { cursor node { wip { theKit { kit { … } } } } } } } }` — never `Query.store` (RS-only test helper).
  - **Mutations**: `session { store(id: $storeId) { theKit { startNewChange | unsavedChange(id:) { kit { … } } } } } }` with `Response` selection (`ok`, `errors`, `result { … on IdResult { value } }`).
  - **Store id**: `stores.edges[].cursor` (relay cursor, typically `e0`), passed as `store(id:)` on mutations.
  - **Session API**: [`StoreSession`](Compose/Store/StoreClient.cs) (`OpenHttp`, `StartNewChange`, `RenameKit`, `ReadWipMaterialization`). Legacy alias: [`KitStoreSession`](Compose/Store/KitStore.cs).
  - **I/O**: [`StoreKitIO.cs`](Compose/Store/StoreKitIO.cs) validates install/import via GraphQL materialized `wip.theKit.kit.name`, then reads/writes JSON locally.
  - **Tests**: [`StoreClientTests.cs`](../Compose.Tests/StoreClientTests.cs) (wire + rename materialization roundtrip). Build `cargo build -p compose-store --release` before live sidecar tests.

## 📛 Entities
