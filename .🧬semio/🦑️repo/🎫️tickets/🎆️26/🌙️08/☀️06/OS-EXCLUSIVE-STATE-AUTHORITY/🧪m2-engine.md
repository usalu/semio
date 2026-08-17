# M2 Engine Module — Wave 1a

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Owner:** Wave 1a M2 ENGINE MODULE

## Delivered

### New module
`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs`

- `EngineKey` / `EngineHandle` — content-addressed (`blake3(engine_id || 0 || input)`)
- `EngineFault` — `UnknownEngine` / `Compute` / `Evicted` / `InvalidInput`
- `Engine` trait with `const ENGINE_ID` + `compute`
- Private `DynEngine` object-safe adapter (`Box<dyn Engine>` is illegal with associated consts)
- `EngineHost` trait defined (object-safe `&self` wrapper deferred)
- `EngineHandles` bag for handle()/render
- `EngineCache` with inherent API:
  - `new(budget_bytes)`
  - `register<E: Engine>`
  - `engine_key`
  - `derive(&mut self, …)` — cache hit touches LRU; miss computes, inserts, evicts
  - `read(&self, …)` — `Evicted` on miss

### Glue
`📦️packages/🦀️rust/📦️glue.rs`:

```rust
pub mod os_engine { … path to ⚙️engine/🦀️component.rs … }
pub use crate::os_engine::*;
```

Placed next to `os_store`; re-exported at crate root like `os_vcs`.

### Deps
Kernel `Cargo.toml` already has `blake3 = "1"` and `thiserror = "2.0.12"`. No workspace / Cargo.toml change required.

### Unit tests (same file, `#[cfg(test)]`)
- `register_and_derive_echoes_input`
- `derive_twice_same_key_is_cache_hit`
- `eviction_when_budget_exceeded`
- `unknown_engine_fault`
- `engine_key_is_stable`

### ArtifactKind
`ArtifactKind` in framework core currently: `Document | Projection | Window | Asset | Network | Backbone` — **no `Engine`**.

Appended open request to ticket `📥️integration-requests.md` (do not edit framework core from this wave):

> Append `Engine` variant after `Backbone` for future `engine-derive` / `engine-read` capability grants.

## Verification

| Gate | Result |
|---|---|
| Isolated `rustc --emit=metadata` on engine component | **pass** → `🧪m2-engine-rustc-check.log` + `🧪m2-engine-check.rmeta` |
| Isolated `rustc --emit=metadata --cfg test` (typechecks unit tests) | **pass** → `🧪m2-engine-rustc-check-test.log` |
| `cargo check -p semio-framework-os-kernel --lib` (earlier, ~15:16) | **pass** (`Finished … in 4.02s`) while store was still green |
| `cargo check -p semio-framework-os-kernel --lib` (latest) | **fail** — 13 errors **all in `🏪️store`** (`CommandReceipt`, `IngestRemote`/`PruneDrafts` match arms, `OperationEnvelope` serde). **Zero** `⚙️engine` / `os_engine` errors. Log: `🧪m2-engine-cargo-check.log` |
| `cargo test -p semio-framework-os-kernel --lib` | **blocked** by same store errors (+ pre-existing fixture-sweep / grammar `include_str` under cfg(test)) |
| Standalone harness / `rustc --test` link | **blocked** on this host by Xcode license (`cc` exit 69) when rebuilding proc-macro build scripts |

## Notes for integrator / peer waves

1. Store wave owns the current kernel `cargo check` red — not engine.
2. Once store compiles again, re-run:
   - `cargo check -p semio-framework-os-kernel --lib`
   - `cargo test -p semio-framework-os-kernel --lib -- register_and_derive_echoes_input derive_twice_same_key eviction_when_budget`
3. Apply `ArtifactKind::Engine` from `📥️integration-requests.md` when WIT imports land (Wave 1b).
