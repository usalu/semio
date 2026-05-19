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

- **Rust kit store (sidecar)**: [`semio/store`](../../store) — `semio-store` (HTTP `POST /install` + `POST /graphql`, same wire as [`semio/js`](../js)). Primary .NET entry points: [`Semio/Store/StoreKitIO.cs`](Semio/Store/StoreKitIO.cs) (install-validated import/export; `KitsEqual` via normalized JSON), and [`Semio/Store/StoreClient.cs`](Semio/Store/StoreClient.cs) for GraphQL queries/mutations. Resolve the binary with `StorePaths.ResolveStoreBinary()` / `SEMIO_STORE_BIN`; the build copies `semio-store.exe` next to `Semio.dll` when `target/release/semio-store.exe` exists. `FileKit` / `FolderKit` / `ArchiveKit` / `ZipRoundtrip` use the sidecar for install validation. `ZipRoundtrip.ImportKit` can load a zip that contains only a root `kit.json` if the sidecar is absent. In-memory `KitDiff` application uses [`KitInPlaceDiff.cs`](Semio/KitInPlaceDiff.cs). For integration tests and benchmarks that read full on-disk kit archives, run `cargo build -p semio-store --release` so the binary is on disk before `dotnet test`.

## 📛 Entities
