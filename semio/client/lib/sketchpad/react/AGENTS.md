---
technology: semio
bundle:
 name: sketchpad
 emoji: ✏️
 description: The sketchpad bundle for semio.
 kind: ui
---

# 🧾 Specification

## Strict layering

Sketchpad sits at the **top** of the client kit stack:

`semio/sketchpad` → `semio/react` (`logic/index.tsx` + optional wasm kit host) → `semio/js` → GraphQL → `semio/rs`

- You MUST **not** import `@semio/js` or `semio/rs` from sketchpad sources (Vite may still alias `@semio/js` for transitive bundles; do not use it directly). CI enforces this via `npm run depcruise:layers` at the repo root (see `.dependency-cruiser.cjs`).
- You MUST **not** implement semio business logic here. All business logic is centralized in `semio/rs`.
- You MUST interact with kits only through **`@semio/react`** hooks and helpers (`executeSemioKitCommand`, `createKitStoreClient`, scopes, etc.). Those helpers forward to `@semio/js` / WASM.
- You MUST NOT construct or apply `KitDiff` / DTO merge / replace graphs in sketchpad; external kit changes go through the **react** command surface → **js** `KitStore` → **rs**.
- You MUST NOT keep any local kit authority. The live kit is only in `semio/rs` (mirrored through the client stack).
- You MUST NOT cache or memoize authoritative kit state; subscribe via react hooks / store clients instead.

## Reads vs operations (`@semio/react`)

- **Reads:** Prefer CQRS field hooks from `semio/client/lib/react/logic` (`useKitName`, `useDesignName`, `useWipKit`, …). Avoid widening `useKitStoreSnapshot()` except where the wasm registry host still materializes a full kit DTO.
- **Operations / side effects:** Use `[run, OperationStatus]` operation hooks (e.g. `useRenameKit`, `useCreateFolder`) or `executeSemioKitCommand` / `applyKitHostGraphOperation`; keep mutations out of render and out of unrelated `useMemo` read paths.
- **Native store shell:** When targeting `semio-store` over HTTP, compose `SemioStoreKitLineHost` (from `@semio/react`) above kit UI so session/store/WIP contexts match the GraphQL line; wasm kit table code may run beside it until the registry is fully migrated.

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities
