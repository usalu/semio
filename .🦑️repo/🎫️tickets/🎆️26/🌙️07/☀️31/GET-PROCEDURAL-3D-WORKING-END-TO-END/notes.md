# Notes

## Root cause
`Render error: session.clearGhostWidget is not a function` happened because Vite loaded the **playground WASM stub** for `@semio-tech/flow-core` instead of the real `FlowSession`.

Chain:
1. 53 `@semio-tech/*` node_modules symlinks were broken (FE0E text-style targets vs FE0F filesystem paths).
2. Engine packages use the wasm-pack `pkg/` folder as the package root, but React imported `…/pkg/foo.js` (nested `pkg/` that does not exist).
3. The stub resolver found the real file after stripping `pkg/`, then returned `undefined`, so Vite still failed the nested path and fell through to `PLAYGROUND_WASM_JS_STUB`.
4. Stub `FlowSession` only had `lodScaleJson()` — no `clearGhostWidget`.

## Fixes
- Repaired broken `@semio-tech/*` symlinks (VS-insensitive path match).
- Pointed `@semio-tech/flow-core` workspace + symlink at `flow/core/⚡implementation/🦀rust/pkg` (wasm-pack output).
- Changed engine session imports to package mains (`@semio-tech/flow-core`, `@semio-tech/framework-surface-node-graph-rs`, …).
- Stub resolver now returns the absolute hit path when a stripped `pkg/` candidate exists; stub `FlowSession` includes ghost APIs.
- Stripped FE0F/FE0E from Rust `char` literals that blocked wasm builds; completed `TokenKind`/`Shape` match arms needed for those builds.

## Verify
`bun e2e-verify.mts` on http://127.0.0.1:6018 → title `semio · procedural · 3d`, Flow+Preview, 3 canvases, no render/ghost errors.


## 2026-07-31 regression (compose→workspace rename)

### Failures
1. `getWorkspaceRoot()` still matched root `package.json` name `"compose"`; under nx cwd it returned the package dir and doubled the plugin-registry path.
2. `UiTreeNode` gained `selected_ids`/`highlighted_ids` without updating all struct initializers → `semio-framework-ui-wgpu` (and others) failed to compile.
3. CAD engine `Cargo.toml` had one-too-many `../` to math-graph-dsl → resolved to `/Users/ueli/Documents/🧰framework/...` and broke flow-core wasm.

### Fixes
- `getWorkspaceRoot`: match `"workspace"`.
- `UiTreeNode`: `Default` impl + fill missing fields across wgpu/workflow/plugins; `PanelTreeBuilder::build` stamps without moving id lists.
- CAD engine path: 10×`../` → 9×`../`.

### Verify
`bun e2e-verify.mts` on http://127.0.0.1:6018 → ok: title `semio · procedural · 3d`, Flow+Preview, 3 canvases, no render/ghost errors.
