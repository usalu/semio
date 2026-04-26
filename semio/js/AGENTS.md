---
technology: semio
bundle:
 name: js
 emoji: 📜
 description: The js bundle for semio.
 kind: library
---

# 🧾 Specification

`semio/js` is a **thin GraphQL client** to `semio/rs` (`KitStoreHandle` over a dedicated Worker, or inline handle in Node tests). It exposes **`KitStore`**, **`openKit`**, plus wire types required for method signatures.

The **`@semio/js/kitWasmBridge`** subpath holds zod/UI DTO helpers consumed by **`@semio/react`**; the root entry MUST NOT import that module (avoids cycles).

You MUST only export **`KitStore`**, **`openKit`**, and the types needed for the public API from the root entry.
You MUST NOT store authoritative kit data.
You MUST NOT cache kit graph data locally (DTO snapshots returned from `snapshot` / reads are rs materializations, not a second source of truth).

## Strict layering

- **Up** (toward UI): `semio/react` imports this package for `KitStore` + wire types.
- **Down** (toward domain): this package speaks **only GraphQL** into `semio/rs`. No imports from `semio/react` or `semio/sketchpad`. CI: `npm run depcruise:layers` (`.dependency-cruiser.cjs`).

## Bidirectional actor model

- **Inbound**: async `KitStore` methods → GraphQL `query` / `mutation` / command shell payloads → rs execute.
- **Outbound**: persistent GraphQL **subscription** → callback-based `subscribe` / `subscribeFiltered` (RxJS is an **internal** implementation detail; it MUST NOT leak into the public `.d.ts`).

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities

### KitStore

Every argument of every public method MUST be typesafe (no `any`; prefer wire DTOs and opaque `ReadWireBatch` for rs-owned read shapes).
