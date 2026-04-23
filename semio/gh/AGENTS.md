---
technology: semio
bundle:
 name: gh
 emoji: 🐙
 description: The gh bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

- **Kit persistence (Grasshopper)**: `LoadKit` / `SaveKit` / `Update Kit` use [`StoreKitIO`](../../net/Semio/Store/StoreKitIO.cs) (semio-store JSON-RPC), same as the net bundle. Ship `semio-store.exe` next to the built `.gha` (or set `SEMIO_STORE_BIN`); the repo’s `Semio.csproj` and this project copy `target/release/semio-store.exe` when present.

## 📛 Entities
