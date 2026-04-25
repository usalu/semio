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

- **`KitStoreClient`** (worker + `FallbackKitStoreClient`) talks to **`KitStoreHandle`** from `@semio/rs-wasm`. CRUD/undo paths use the existing `changeKitCommands*` / `executeChangeKitCommands` helpers; structured VCS and **backbone / conflict** commands go through **`execute`**, **`executeRead`**, **`vcsState`**, **`theKitDto`**, and **`materializeAt`**. **`executeRead(cmds)`** is typed as **`ReadCommandBatch` → `Promise<ReadCommandBatchResult>`** (see [`readCommandTypes.ts`](readCommandTypes.ts), generated from [`../rs/read_module.rs`](../rs/read_module.rs) via [`gen_read_command_types.py`](gen_read_command_types.py)); it forwards to wasm **`executeReadKitCommands`** (same JSON shape).
- **Wire types** (serde-compatible with `semio::kit_store_command` / `kit_backbone_wire`): `BackboneConfig` (`dev` / `local` / `remote` keys), `ConflictResolution` (`dropWip` / `forceOverwriteBackbone` as `{ variant: null }`), `BackboneStatusDto`, `KitConflict`, `KitStoreExecuteResult`.
- **WASM graph handle**: `KitStoreHandle` wraps a plain in-memory **`KitGraphRef`**. Backbone/coordinator commands require **`semio::kit_store::KitStore`** (native / `semio-store`); on WASM they fail with an invalid-operation style error. Use **`vcsState`** for the Git-style tree without a backbone.
- **Convenience methods**: `attachBackbone`, `detachBackbone`, `backboneStatus`, `listConflicts`, `resolveConflict`, `syncNow` build the same JSON shapes as Storybook `HistoryControls` (`newSession: null`–style tagged commands).
- **`InMemoryKitStore` / `JsonFileKitStore` / `FolderKitStore` / `createSessionKitStore`**: host-facing kit containers (`getSnapshot` includes `kit` + `sync` with **`DEFAULT_KIT_SYNC`**). Session store is an in-memory placeholder until hub sync is wired.
- **`getSemioKitViewStore`**: per-`KitStoreClient` cache; subscribes to the event stream and only notifies when a catalog key’s JSON snapshot changes (used by `@semio/react` catalog hooks).

## 📛 Entities

### Kit
