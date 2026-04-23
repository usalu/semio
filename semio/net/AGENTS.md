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

- **Rust kit store (sidecar)**: [`semio/store`](../../store) — `semio-store` (stdio NDJSON JSON-RPC 2.0). Primary .NET entry points: [`Semio/Store/StoreKitIO.cs`](Semio/Store/StoreKitIO.cs) (import/export + `KitsEqual` via `kit.equals` when the binary is present, otherwise full JSON deep-compare), and [`Semio/Store/StoreClient.cs`](Semio/Store/StoreClient.cs) for ad-hoc calls. Resolve the binary with `StorePaths.ResolveStoreBinary()` / `SEMIO_STORE_BIN`; the build copies `semio-store.exe` next to `Semio.dll` when `target/release/semio-store.exe` exists. `FileKit` / `FolderKit` / `ArchiveKit` / `ZipRoundtrip` use the sidecar for materialization. `ZipRoundtrip.ImportKit` can load a zip that contains only a root `kit.json` if the sidecar is absent. In-memory `KitDiff` application uses [`KitInPlaceDiff.cs`](Semio/KitInPlaceDiff.cs). For integration tests and benchmarks that read full on-disk kit archives, run `cargo build -p semio-store --release` so the binary is on disk before `dotnet test`.

## 📛 Entities
