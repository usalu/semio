# Wave 1 — flow core dissolution

## Summary
Dissolved `🌊️flow/🙀️core/` into concept siblings under `🌊️flow/`, lifted `📐️brep-geometry`, rewired `📦️glue.rs` (`flow_core` → `flow`), deleted the corrupted duplicate glue file, and recorded external `flow_core::` renames for Wave 2.

## Created
- `🌊️flow/📄️document/🦀️component.rs`
- `🌊️flow/📚️catalogue/🦀️component.rs`
- `🌊️flow/📔️registry/🦀️component.rs`
- `🌊️flow/🌉️bridge/🦀️component.rs`
- `🌊️flow/🖥️host/🦀️component.rs` (Errors + FlowHost + Tests)
- `🌊️flow/🖍️drawing/🦀️component.rs`
- `🌊️flow/🌉️wasm/🦀️component.rs`
- `🌊️flow/🌿️vcs/🦀️component.rs`
- ticket `deferred-flow-core.json` (72 external files)
- ticket `wave1-flow-core.report.md`

## Updated
- `🌊️flow/📦️packages/🦀️rust/📦️glue.rs` — removed `pub mod core`; wired document/catalogue/registry/bridge/host/drawing/wasm_session/vcs/brep_geometry via `#[path]`; `extern crate self as flow`
- `🌊️flow/📦️packages/🦀️rust/Cargo.toml` — description; dev-dep path `…/🟡️core` → `…/🔤️primitive` (parallel rename already landed)
- `🖥️host` test helpers: `semio_s_plugin_flow_extension_core` → `semio_s_plugin_flow_extension_primitive`

## Removed
- `🌊️flow/🙀️core/` (including nested `pkg/` wasm artifact leftover)
- `🌊️flow/📦️packages/🦀️rust/�📦️glue.rs` (corrupted duplicate)

## Lifted
- `🌊️flow/🙀️core/📐️brep-geometry/` → `🌊️flow/📐️brep-geometry/` (as-is)

## Modules
| Folder | Rust mod | Former region lines |
|--------|----------|---------------------|
| 📄️document | document | Document/Widget 22–1038 |
| 📚️catalogue | catalogue | Catalogue 1040–1337 |
| 📔️registry | registry | ExtensionRegistry 1339–1575 |
| 🌉️bridge | bridge | EvalBridge+ChannelEval 1577–1976 |
| 🖥️host | host | Errors+FlowHost 1978–4010 + Tests 6011–7915 |
| 🖍️drawing | drawing | DrawingKernel 4013–4215 |
| 🌉️wasm | wasm_session | WasmSession 4217–4886 |
| 🌿️vcs | vcs | DocumentVcs 4888–6009 |
| 📐️brep-geometry | brep_geometry | lifted sibling |

## Alias
- `extern crate self as flow_core` → `extern crate self as flow`
- Removed `pub mod core` / `pub use core::*`

## Naming
- Used `🌿️vcs` (no collision under `🌊️flow/`; OS-level `✨️modules/🌿️vcs` is unrelated)

## Deferred (Wave 2)
- See `deferred-flow-core.json`: 72 files with `flow_core::` → `flow::`
- Root `Cargo.toml` workspace alias `semio-framework-os-kernel-flow-core` (shared; not owned here)
- Plugin `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` still points at deleted `🧩️extensions/🙀️core` (plugin agent / Wave 2)

## Verify
- Structural: core gone, 8 concept modules + brep present, single glue file, no pub-name collisions across modules
- `cargo check -p semio-framework-os-flow` blocked by workspace member plugin Cargo.toml still referencing `…/🧩️extensions/🙀️core` (outside this agent's ownership)

## Out of scope (untouched)
- pack/db/spr/dsl cores
- plugins (except deferred manifest)
- framework `🧩core`
