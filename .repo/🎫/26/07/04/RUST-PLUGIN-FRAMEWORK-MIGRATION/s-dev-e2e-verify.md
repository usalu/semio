# S Dev End-to-End Verification

Date: 2026-07-04

## Command
`SEMIO_PLUGIN=s bun nx run @semio-tech/framework-os-dev:dev`

## Result: PASS

- S Studio boots with all 25 WASM plugins loaded
- Header shows `25 programs · N spawned`
- Demo studio media graph document renders (node-graph host stub)
- Catalogue panel lists all plugin programs
- Spawn (+ Draw) creates instance and shows spawned preview panel
- `cargo test -p s-plugin` passes (2 tests)
- `@semio-tech/framework-renderer-react:test` passes

## Fixes applied for E2E
- Studio mode (`SEMIO_PLUGIN=s`) loads all plugins, boots `s-play` shell
- Full `s/plugin/rs` studio implementation with demo document
- `wasm_bindgen(start)` forces `_PLUGIN_INIT` so manifests register
- Plugin output moved from `public/plugins` to `plugin-modules` (Vite 7 import restriction)
- Restored minimal `framework/core/js` canvas pick exports for `ui-react`
- Vite alias for `@semio-tech/framework-renderer-react`
