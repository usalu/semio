# Wave 3.d — Flow BIM extension conversion

## Done
- ExtensionBundle + extension_exports! (no PluginBundle)
- invoke handler capability `evaluate` via evaluate_json
- removed standalone-wasm / wasm_ext / pkg/
- Cargo.toml: role=extension, extends=flow, non-optional component-guest deps
- app_id: contributes to both `flow-play` and `procedural3d-play`
- extension_id wire id: `flow-extension-bim` (manifest id remains `bim` for operator pending-eval lookup)

## Verify
`cargo test -p semio-s-plugin-flow-extension-bim`
