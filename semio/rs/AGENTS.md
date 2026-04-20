---
technology: semio
bundle:
 name: rs
 emoji: 🦀
 description: The rs bundle for semio.
 kind: library
---

# 🧾 Specification

All semio domain logic MUST exclusively live in `semio/rs`.
All kit graph caching and invalidation MUST exclusively live in `semio/rs`.

## Strict layering

Consumers talk to this bundle only through the **GraphQL control plane** (WASM `KitStoreHandle` / native `semio-store`). Downstream bundles (`semio/js`, then `semio/react`, then hosts) MUST NOT re-implement kit math, diff merge, or flatten previews.

## 🛠️ Mechanisms

### CQRS dual-bus actor model

- Communication MUST be symmetrical: inbound work and outbound events share one contract surface (GraphQL execute + subscription stream).
- You MUST NOT rely on client-side polling for kit truth; clients subscribe and react to events.
- You MUST use a **single FFI boundary** per host process (one `KitStoreHandle` in WASM worker or one native adapter).
- You MUST use an **inbound work queue** for commands (non-blocking accept).
- You MUST use an **outbound event stream** for lifecycle + data (including command outcomes).
- You MUST NOT depend on OS-only async runtimes where the target is `wasm32` (no tokio on wasm paths).

### Command / diff contract (target)

- **External** kit mutations MUST be expressed only as **semantic kit change commands** (no ad-hoc field surgery from clients).
- Each change command MUST define:
  1. A pure **forward** function: concrete parameters → **`KitDiff`** (or typed fragment) describing the state transition.
  2. A pure **inverse** function: ordered list of concrete commands → inverse **`KitDiff`** (or stack of inverse commands) so undo is data-defined.
- **Internal** application MUST be central: apply diffs to the live graph, invalidate caches, emit events — **commands MUST NOT** scatter direct `KitGraph` edits outside the central apply path.
- **Async command surface (target)**: accepting a command returns only an **execution / request id**; success, merged DTOs, and errors are delivered on the **event stream** so clients correlate by id. (Transitional GraphQL helpers that return immediate receipts MUST converge on this model.)

## 🕸️ Systems

## 🧮 Algorithms

## 📛 Entities
