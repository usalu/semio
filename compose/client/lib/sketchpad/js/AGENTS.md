---
technology: compose
path: 🏘️compose✍️sketchpad
bundle:
 name: sketchpad
 emoji: ✍️
 description: The sketchpad bundle for compose.
 kind: ui
---

# 🧾 Specification

## Strict layering

Sketchpad sits at the **top** of the client kit stack:

`compose/sketchpad` → `compose/react` (`logic/index.tsx` + optional wasm kit host) → `compose/js` → GraphQL → `compose/rs`

- You MUST **not** import `@compose/js` or `compose/rs` from sketchpad sources (Vite may still alias `@compose/js` for transitive bundles; do not use it directly). CI enforces this via `npm run depcruise:layers` at the repo root (see `.dependency-cruiser.cjs`).
- You MUST **not** implement compose business logic here. All business logic is centralized in `compose/rs`.
- You MUST interact with kits only through **`@compose/react`** hooks and helpers (`executeComposeKitCommand`, `createKitStoreClient`, scopes, etc.). Those helpers forward to `@compose/js` / WASM.
- You MUST NOT construct or apply `KitDiff` / DTO merge / replace graphs in sketchpad; external kit changes go through the **react** command surface → **js** `KitStore` → **rs**.
- You MUST NOT keep any local kit authority. The live kit is only in `compose/rs` (mirrored through the client stack).
- You MUST NOT cache or memoize authoritative kit state; subscribe via react hooks / store clients instead.

## Reads vs operations (`@compose/react`)

- **Reads:** Prefer CQRS field hooks from `compose/client/lib/react/logic` (`useKitName`, `useDesignName`, `useWipKit`, …). Avoid widening `useKitStoreSnapshot()` except where the wasm registry host still materializes a full kit DTO.
- **Operations / side effects:** Use `[run, OperationStatus]` operation hooks (e.g. `useRenameKit`, `useCreateFolder`) or `executeComposeKitCommand` / `applyKitHostGraphOperation`; keep mutations out of render and out of unrelated `useMemo` read paths.
- **Native store shell:** When targeting `compose-store` over HTTP, compose `ComposeStoreKitLineHost` (from `@compose/react`) above kit UI so session/store/WIP contexts match the GraphQL line; wasm kit table code may run beside it until the registry is fully migrated.

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities
