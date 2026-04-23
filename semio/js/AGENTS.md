---
technology: semio
bundle:
 name: js
 emoji: 📜
 description: The js bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

### Kit store client (WASM) and control-plane commands

- **`KitStoreClient`** (worker + `FallbackKitStoreClient`) talks to **`KitStoreHandle`** from `@semio/rs-wasm`. CRUD/undo paths use the existing `changeKitCommands*` / `executeChangeKitCommands` helpers; structured VCS and **backbone / conflict** commands go through **`execute`**, **`executeReadKitCommands`**, **`vcsState`**, **`theKitDto`**, and **`materializeAt`**.
- **Wire types** (serde-compatible with `semio::kit_store_command` / `kit_backbone_wire`): `KitStoreWireBackboneConfig` (`dev` / `local` / `remote` keys), `KitStoreWireConflictResolution` (`dropWip` / `forceOverwriteBackbone` as `{ variant: null }`), `KitStoreWireBackboneStatus`, `KitStoreWireKitConflict`, `KitStoreExecuteResult`.
- **WASM graph handle**: `KitStoreHandle` wraps a plain in-memory **`KitGraphRef`**. Backbone/coordinator commands require **`semio::kit_store::KitStore`** (native / `semio-store`); on WASM they fail with an invalid-operation style error. Use **`vcsState`** for the Git-style tree without a backbone.
- **Convenience methods**: `attachBackbone`, `detachBackbone`, `backboneStatus`, `listConflicts`, `resolveConflict`, `syncNow` build the same JSON shapes as Storybook `HistoryControls` (`newSession: null`–style tagged commands).
- **`InMemoryKitStore`**: `execute` / `vcsState` / `materializeAt` reject; backbone/conflict mutators return **`NotSupported`**.

## 📛 Entities

### Kit
