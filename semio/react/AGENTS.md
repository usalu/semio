---
technology: semio
bundle:
  name: react
  emoji: ⚛️
  description: React hooks bundle for semio.
  kind: library
---

# Specification

## Mechanisms

### Kit backbone / coordinator hooks (WASM `KitStoreClient`)

These hooks call into `@semio/js` **`KitStoreClient`** methods that forward to **`KitStoreHandle.execute`** (and related WASM APIs). They report failures via hook-local `lastError` and **`pushSetRejection`** on the kit runtime (same path as schema write errors).

- **`useBackboneStatus(pollMs?)`**: polls `backboneStatus`; `refresh()` for manual updates.
- **`useAttachBackbone()`**: `{ attach, detach, pending, lastError }` — `attach(cfg)` uses `KitStoreWireBackboneConfig`.
- **`useDetachBackbone()`**: `{ detach, pending, lastError }` only.
- **`useListConflicts()`**: `{ conflicts, refresh, pending, lastError }`.
- **`useResolveConflict()`**: `{ resolve(id, strategy), pending, lastError }` with `KitStoreWireConflictResolution`.
- **`useSyncNow()`**: `{ sync, pending, lastError }`.

Re-exported wire types: `KitStoreWireBackboneConfig`, `KitStoreWireConflictResolution`, `KitStoreWireBackboneStatus`, `KitStoreWireKitConflict`, `KitStoreExecuteResult` (from `@semio/js`).

On a plain browser WASM graph (no native control plane), backbone/coordinator commands are expected to fail; use **`useKitStoreClient`**.`vcsState()` for VCS tree UI where appropriate.
