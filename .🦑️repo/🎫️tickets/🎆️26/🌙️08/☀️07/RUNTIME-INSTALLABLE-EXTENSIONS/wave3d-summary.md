# Wave 3.d — flow-extension-bim → ExtensionBundle

## Outcome

Converted `semio-s-plugin-flow-extension-bim` from `PluginBundle` / `plugin_exports!` + `standalone-wasm` to `ExtensionBundle` / `extension_exports!` with an `evaluate` invoke handler.

## Changes

### `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs`

- Replaced `#region PluginGuest` + entire `#region WasmExt` (wasm-bindgen surface) with `#region ExtensionGuest`.
- Bundle: `ExtensionBundle::new("bim", "Bim", "0.1.0").extends("flow")`.
- Dual `Contribution::FlowExtension` entries (one `app_id` each): `flow-play` and `procedural3d-play` — both hosts still consume contributions via `setContributions`.
- Handler `evaluate` parses `{ operatorId, inputJson, nodeHash? }` and runs `evaluate_json` against `module_registry()`.
- Domain `register()` + existing `build_manifest_json` / operator tests kept; added `extension_bundle_extends_flow_and_evaluates`.

### `📦️packages/🦀️rust/Cargo.toml`

- Removed `standalone-wasm` feature and `wasm-bindgen` dependency / wasm-pack metadata.
- Kept `component-guest` (default) gating optional `semio-framework-plugin` + `semio-framework-core`.
- Semio metadata: `role = "extension"`, `extends = "flow"`, `contributes = ["flow.extension"]`.

### Build / workspace

- `📜️script.ts`: dropped `WasmScript` / `runWasmPackWebBuild`; test-only router remains.
- `📋️project.json`: removed `wasm` target.
- Root `package.json`: removed workspace `…/bim/📦️packages/🦀️rust/pkg` (`@semio-tech/flow-module-bim`).

## Confirmations

- ShellHost / renderer: no `flow-module-bim` import (Wave 0 already cleared).
- No remaining `standalone-wasm`, `WasmExt`, `plugin_exports!`, or `PluginBundle` in bim.

## Verification

- `cargo test` / `cargo check -p semio-s-plugin-flow-extension-bim` blocked in this environment by:
  1. Xcode license agreement (cc/clang exit 69; blake3 neon rebuild + link fail).
  2. Concurrent Wave 3.a churn in `semio-framework-os-flow` (extern crate / extension wiring mid-migration).
- Logs: `🧪cargo-test-bim.txt`, `🧪cargo-check-bim*.txt`, `🧪cargo-test-bim-blocked.txt`.

## Follow-ups

- Re-run `cargo test -p semio-s-plugin-flow-extension-bim` once Xcode license + flow SDK stabilize.
- Wave 5: scrub remaining doc/test mentions of `@semio-tech/flow-module-bim` (e.g. workspaces helper comments, vite-elements-assets allowlist) and delete stale `pkg/` if still present on disk.
