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

Sketchpad sits at the **top** of the wasm host stack:

`semio/sketchpad` → `semio/react` → `semio/js` → GraphQL → `semio/rs`

- You MUST **not** import `@semio/js` or `semio/rs` from sketchpad sources (Vite may still alias `@semio/js` for transitive bundles; do not use it directly).
- You MUST **not** implement semio business logic here. All business logic is centralized in `semio/rs`.
- You MUST interact with kits only through **`@semio/react`** hooks and helpers (`executeSemioKitCommand`, `createKitStoreClient`, scopes, etc.). Those helpers forward to `@semio/js` / WASM.
- You MUST NOT construct or apply `KitDiff` / DTO merge / replace graphs in sketchpad; external kit changes go through the **react** command surface → **js** `KitStore` → **rs**.
- You MUST NOT keep any local kit authority. The live kit is only in `semio/rs` (mirrored through the client stack).
- You MUST NOT cache or memoize authoritative kit state; subscribe via react hooks / store clients instead.

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities
