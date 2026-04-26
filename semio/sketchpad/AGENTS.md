---
technology: semio
bundle:
 name: sketchpad
 emoji: ✏️
 description: The sketchpad bundle for semio.
 kind: ui
---

# 🧾 Specification

- You MUST NOT implement any semio business logic. All business logic is centralized in `semio/rs`.
- You MUST only use hooks from `semio/react` to interact with kits (subscription + command surface).
- The thin command gate is `executeSemioKitCommand` in `semio/js` (WASM/GraphQL into `semio/rs`). You MUST NOT construct or apply `KitDiff` / DTO merge / replace graphs in sketchpad; external kit changes MUST be `kitWire` → `executeSemioKitCommand` only.
- You MUST NOT keep any local kit authority. The live kit is only in `semio/rs` (mirrored into the in-memory `KitStore` via the client).
- You MUST NOT cache/memoize any kit state.

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities
