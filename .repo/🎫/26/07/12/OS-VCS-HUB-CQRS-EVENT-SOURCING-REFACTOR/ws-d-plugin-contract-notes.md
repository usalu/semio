# WS-D — Plugin Contract (framework/plugin)

## Scope
- framework/plugin/rs/lib.rs: two-layer contract (DocumentApp typed / PluginApp runtime), VcsDocumentApp wrapper, generation-op vocabulary, TestApp.
- framework/wit/world.wit: new plugin exports + host backbone-status import.
- framework/plugin/rs component module: wasm WIT glue for new exports.
- framework/plugin/host/rs/lib.rs: native host — backbone registry placeholder (resolve_backbone gone), backbone-status import impl, caller methods for new exports.
- framework/core/rs/lib.rs: HostEffect variants + IconRenderExportItem. (DONE)
- framework/core/js/index.ts: typed ActionResponse, delete patchOpsFromActionResponse, new handle methods.

## Verification results (2026-07-12)
- `cargo build -p semio-framework-core` — green (2 pre-existing unrelated warnings).
- `cargo build -p semio-framework-plugin` — green.
- `cargo test -p semio-framework-plugin` — 27 passed / 0 failed (8 new contract tests).
- `cargo check -p semio-framework-plugin --target wasm32-wasip2` — green (component Guest + WIT bindings regenerate cleanly).
- plugin-host built standalone (temp `[workspace]`, reverted) — green (wasmtime bindgen! regenerated the 5 new export callers + backbone-status host import).
- `tsc -p tsconfig.json`: zero errors in framework/core/js/index.ts from WS-D regions. (One unrelated pre-existing/concurrent error at index.ts:916 in `uiDeclarativeChildToTreeItem`, not WS-D.)
- WS-F worklist confirmed via sequence-plugin: E0407/E0046/E0277 — all ~24 `semio_plugin!` crates need `DocumentApp` migration.

Codegen tooling IS available; WIT bindings auto-regenerate (wasmtime `bindgen!` native, `wit-bindgen::generate!` wasm). No hand-written bindings.

## Env blocker
Concurrent mathematical-crates refactor left a dangling path-dep in framework/graph/rs/Cargo.toml
(-> deleted mathematical/graph/port/directed/dag/rs). Blocks ALL cargo manifest loading.
Verification via temporary stub of the missing crate, removed afterward.
