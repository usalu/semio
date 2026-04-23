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

- **Rust kit store (sidecar)**: [`semio/store`](../../store) — `semio-store` (stdio NDJSON JSON-RPC 2.0). Primary .NET entry points: [`Semio/Store/StoreKitIO.cs`](Semio/Store/StoreKitIO.cs) (import/export + `KitsEqual` via `kit.equals`), and [`Semio/Store/StoreClient.cs`](Semio/Store/StoreClient.cs) for ad-hoc calls. Resolve the binary with `StorePaths.ResolveStoreBinary()` / `SEMIO_STORE_BIN` / `runtimes/**` when built. `FileKit` / `FolderKit` / `ArchiveKit` / `ZipRoundtrip` delegate to the sidecar; in-memory `KitDiff` application uses [`KitInPlaceDiff.cs`](Semio/KitInPlaceDiff.cs). Authoritative graph mutations in production are `ChangeKitCommand` JSON on the same process.

## 📛 Entities
