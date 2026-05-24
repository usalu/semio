---
technology: semio
bundle:
  name: react
  emoji: ÔÜø´©Å
  description: React hooks bundle for semio.
  kind: library
---

# Specification

## Strict layering

- **Up**: `semio/sketchpad` (and other hosts) import **only** `@semio/react` for kit hooks, command facades, and DTO helpers re-exported here.
- **Down**: this package uses **`@semio/js`** for `Session`, `Store.installProjection`, and wire types. You MUST NOT import `semio/rs` or raw WASM from React code. CI: `npm run depcruise:layers` (`.dependency-cruiser.cjs`).

Kit domain logic and caching remain in **`semio/rs`**; the merged legacy entity region in **`@semio/js`** exists only as a **temporary UI/DTO bridge** and MUST shrink over time.

## Hooks and external store

- Any hook that mirrors **live kit state** from the wasm client MUST prefer **`React.useSyncExternalStore`** with the client's `subscribe` + `getSnapshot` pattern (see existing kit view / live-read hooks in `index.tsx`).
- Hooks MUST treat `KitStoreClient` / `KitStore` as the authority; local React state is view-only (selection, layout), not a second kit graph.
- **Entity CQRS:** One id hook per entity (`useDesign`, `useType`, ÔÇª), one field hook per scalar/collection field (`useDesignName`, ÔÇª), and one command hook per entity (`useDesignCommand`, `useKitCommand`, ÔÇª) exposing `{ run, status }` with a shared `OperationStatus` across all mutations on that scope.
- **Collection reads:** `useKitDesigns`, `useKitTypes`, `useDesignPieces`, ÔÇª return stable **entity handles** (`Design[]`, `Type[]`, ÔÇª), not bare id strings. Sketchpad tuple bundles (`useTypes`, `useTagsFull`, ÔÇª) live in `// #region ­ƒÄ¿SketchpadFacade` (legacy kit-store bridge WIP).

## Mechanisms

### Kit backbone / coordinator hooks (WASM `KitStoreClient`)

These hooks call into `@semio/js` **`KitStore`** (via `KitStoreClient`) methods that forward to **`KitStoreHandle.execute`** (and related WASM APIs). They report failures via hook-local `lastError` and **`pushSetRejection`** on the kit runtime (same path as schema write errors).

- **`useBackboneStatus(pollMs?)`**: polls `backboneStatus`; `refresh()` for manual updates.
- **`useAttachBackbone()`**: `{ attach, detach, pending, lastError }` ÔÇö `attach(cfg)` uses `BackboneConfig`.
- **`useDetachBackbone()`**: `{ detach, pending, lastError }` only.
- **`useListConflicts()`**: `{ conflicts, refresh, pending, lastError }`.
- **`useResolveConflict()`**: `{ resolve(id, strategy), pending, lastError }` with `ConflictResolution`.
- **`useSyncNow()`**: `{ sync, pending, lastError }`.

Re-exported wire types: `BackboneConfig`, `ConflictResolution`, `BackboneStatusDto`, `KitConflict`, `KitStoreExecuteResult`, `ReadWireBatch`, `ReadWireItem`, `ReadWireBatchResult` (from `@semio/js`).

Per-kit UI is wrapped in **`KitScope`**. The **persistence** shape passed when opening a kit is **`KitBackboneConfig`**. Entity panels use **`DesignScope`**, **`PieceScope`**, **`TypeScope`**, and the other `*Scope` components.

On a plain browser WASM graph (no native control plane), backbone/coordinator commands are expected to fail; use **`useKitStoreClient`**.`vcsState()` for VCS tree UI where appropriate.
