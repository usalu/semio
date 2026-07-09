---
technology: compose
bundle:
 name: rs
 emoji: 🦀
 description: The rs bundle for compose.
 kind: library
---

# 🧾 Specification

All compose domain logic MUST exclusively live in `compose/rs`.
All kit graph caching and invalidation MUST exclusively live in `compose/rs`.

## Strict layering

Consumers talk to this bundle only through the **GraphQL control plane** (WASM `KitStoreHandle` / native `compose-store`). Downstream bundles (`compose/js`, then `compose/react`, then hosts) MUST NOT re-implement kit math, diff merge, or flatten previews.

## 🛠️ Mechanisms

### CQRS dual-bus actor model

- Communication MUST be symmetrical: inbound work and outbound events share one contract surface (GraphQL execute + subscription stream).
- You MUST NOT rely on client-side polling for kit truth; clients subscribe and react to events.
- You MUST use a **single FFI boundary** per host process (one `KitStoreHandle` in WASM worker or one native adapter).
- You MUST use an **inbound work queue** for commands (non-blocking accept).
- You MUST use an **outbound event stream** for lifecycle + data (including command outcomes).
- You MUST NOT depend on OS-only async runtimes where the target is `wasm32` (no tokio on wasm paths).

## 🕸️ Systems

## 🧮 Algorithms

## 📛 Entities
