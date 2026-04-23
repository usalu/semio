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

- **Rust kit store (sidecar)**: [`semio/store`](../../store) — `semio-store` (stdio NDJSON JSON-RPC 2.0). The .NET client is [`Semio/Store/StoreClient.cs`](Semio/Store/StoreClient.cs) (`Semio.Store.StoreClient`); resolve the binary with `StorePaths.ResolveStoreBinary()` / `SEMIO_STORE_BIN` / `runtimes/**` content when built.
- **Persistence**: the legacy in-file `KitSqlite` / `SemioDiff` / `FileKit` / `FolderKit` surface remains for now; new host code should talk to the sidecar. Follow-up: route import/export and graph edits through `ChangeKitCommand` wire JSON.

## 📛 Entities
