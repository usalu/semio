# UI WGPU DslValue Serde Seam

## Decision

`DslValue` needs a JSON-shaped serde implementation at this host UI boundary. An already-staged implementation delegates through the existing `DslValue` ↔ `serde_json::Value` bridge, so it exactly matches the established wire shape. The duplicate direct visitor implementation drafted during this investigation was removed.

## Evidence

- The `wgpu` component's `ActionDescriptor`, `UiMenuRef`, and `ContextMenuItemSpec` directly own `DslValue` fields.
- Those types are in turn embedded throughout serde-derived UI trees, measures, engagements, and table cells.
- Runtime helpers serialize `ActionDescriptor` into `settings_json` and serialize table cells through `serde_json::to_value`; the JSON golden tests also exercise the same public wire contract.
- Replacing only the three direct types with `ToValue`/`FromValue` would make every serde-derived parent fail. Replacing the full UI contract would be a separate UI-boundary migration, not a repair of this missing implementation.

## Shape

A `serde_json` parity test compares serde serialization to the existing `DslValue` ↔ `serde_json::Value` bridge and verifies round-tripping.

## Verification

- `cargo check -p semio-framework-ui --message-format=short` completed successfully with zero errors. The original `DslValue: Serialize`/`Deserialize` E0277 diagnostics are absent.
- The focused OS-kernel test command could not compile because of unrelated live serde regressions on `PresencePeer` and `MutationLeafDescriptor`.
- `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-raster` reaches the OS kernel and fails only on two unrelated `MutationLeafDescriptor: serde::Serialize` E0277 diagnostics; no `DslValue` diagnostic remains.
- The focused UI golden test could not compile because of two unrelated `E0308` test-compilation errors in `🎯️targets/🧊️wgpu/🦀️prepared.rs`.
