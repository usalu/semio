---
technology: compose
bundle:
 name: gh
 emoji: 🐙
 description: The gh bundle for compose.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

- **Kit persistence (Grasshopper)**: `LoadKit` / `SaveKit` / `Update Kit` use [`StoreKitIO`](../../net/Compose/Store/StoreKitIO.cs) (compose-store JSON-RPC), same as the net bundle. Ship `compose-store.exe` next to the built `.gha` (or set `COMPOSE_STORE_BIN`); the repo’s `Compose.csproj` and this project copy `target/release/compose-store.exe` when present.

## 📛 Entities
