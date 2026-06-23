---
technology: compose
bundle:
 name: js
 emoji: 📜
 description: The js bundle for compose.
 kind: library
---

# 🧾 Specification

`compose/js` is a **thin GraphQL client** to `compose/rs` (`Session` + `Store.installProjection` over a dedicated Worker). It exposes **`Session`**, **`Store`**, **`openSessionInMemory`**, plus wire types required for method signatures.

The root **`@compose/js`** entry holds the WASM transport plus zod/UI DTO helpers consumed by **`@compose/react`**;

You MUST NOT store authoritative kit data.
You MUST NOT cache kit graph data locally (DTO snapshots returned from `snapshot` / reads are rs materializations, not a second source of truth).

## Strict layering

- **Up** (toward UI): `compose/react` imports this package for `KitStore` + wire types.
- **Down** (toward domain): this package speaks **only GraphQL** into `compose/rs` (wire + WASM in `index.ts` regions `GraphqlContract` / `RsWasmTransport`; `kit-store.worker.ts` with `dev://empty` only). Kit JSON enters via `Store.installProjection`, never `Session.open` URIs. No imports from `compose/react` or `compose/sketchpad`. CI: `npm run depcruise:layers` (`.dependency-cruiser.cjs`).

## Bidirectional actor model

- **Inbound**: async `KitStore` methods → GraphQL `query` / `mutation` / command shell payloads → rs execute.
- **Outbound**: persistent GraphQL **subscription** → callback-based `subscribe` (RxJS is an **internal** implementation detail; it MUST NOT leak into the public `.d.ts`).

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities

### KitStore

Every argument of every public method MUST be typesafe (no generic `any` or `json`).
