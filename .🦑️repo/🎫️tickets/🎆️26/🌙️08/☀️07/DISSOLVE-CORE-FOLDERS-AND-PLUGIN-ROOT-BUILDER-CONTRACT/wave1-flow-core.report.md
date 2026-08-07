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
- ticket `deferred-flow-core.json` (72 external files: `flow_core::` → `flow::`)
- ticket `wave1-flow-core.report.md`

## Updated
- `🌊️flow/📦️packages/🦀️rust/📦️glue.rs`
  - removed `pub mod core`
  - wired document / catalogue / registry / bridge / host / drawing / wasm_session / vcs / brep_geometry via `#[path]`
  - `extern crate self as flow_core` → `extern crate self as flow`
  - added `KernelModuleAliases` (`os_store` / `os_dsl` / `os_spr` / `os_vcs` / `os_pack`) matching `♾️infinite`
  - added `InfiniteAlias` + crate-root `dag` / `canvas` / `neural` re-exports
- `🌊️flow/📦️packages/🦀️rust/Cargo.toml`
  - description
  - dep `semio-framework-os-infinite`
  - dev-dep path `…/🟡️core` → `…/🔤️primitive` (parallel rename already landed)
- `🖥️host` tests: `semio_s_plugin_flow_extension_core` → `semio_s_plugin_flow_extension_primitive`

## Removed
- `🌊️flow/🙀️core/` (including nested `pkg/`)
- corrupted duplicate `🌊️flow/📦️packages/🦀️rust/�📦️glue.rs`

## Lifted
- `🌊️flow/🙀️core/📐️brep-geometry/` → `🌊️flow/📐️brep-geometry/` (as-is)

## Modules
| Folder | Rust mod | Former region lines |
|--------|----------|---------------------|
| 📄️document | document | Document 22–1038 |
| 📚️catalogue | catalogue | Catalogue 1040–1337 |
| 📔️registry | registry | ExtensionRegistry 1339–1575 |
| 🌉️bridge | bridge | EvalBridge+ChannelEval 1577–1976 |
| 🖥️host | host | Errors+FlowHost 1978–4010 + Tests 6011–7915 |
| 🖍️drawing | drawing | DrawingKernel 4013–4215 |
| 🌉️wasm | wasm_session | WasmSession 4217–4886 |
| 🌿️vcs | vcs | DocumentVcs 4888–6009 |
| 📐️brep-geometry | brep_geometry | lifted sibling |

## Naming
- Used `🌿️vcs` (no collision under `🌊️flow/`)

## Deferred (Wave 2)
- `deferred-flow-core.json` — 72 files outside the flow tree still using `flow_core::`
- Root `Cargo.toml` alias `semio-framework-os-kernel-flow-core` (shared)
- Plugin `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` still points at deleted `🧩️extensions/🙀️core` (blocks workspace `cargo check`; owned by plugin agent / Wave 2)

## Verify
- Structural: core gone; 8 concept modules + brep present; single glue file; balanced braces/regions; no pub-name collisions across modules
- `cargo check -p semio-framework-os-flow` not runnable until plugin Cargo.toml path is fixed (outside ownership)

## Out of scope (untouched)
- pack / db / spr / dsl cores
- plugins (except deferred manifest + local test crate rename for primitive)
- framework `🧩core`
