# Verify Log — Fix Lowpoly End To End Boot

## Root cause

jco-transpiled plugin component JS imports `@bytecodealliance/preview2-shim/*` bare specifiers. Trunk serves plugin modules as static ESM without a bundler, so the plugin worker failed during `init` with:

`Failed to resolve module specifier "@bytecodealliance/preview2-shim/cli"`

## Fix

- Vendor `@bytecodealliance/preview2-shim` browser shims into `plugin-modules/_vendor/@bytecodealliance/preview2-shim/`
- Rewrite component imports to relative paths with `.js` extensions after jco transpile
- Watch `plugin-modules` in wgpu `Trunk.toml` so hot reload picks up plugin rebuilds
- Add missing `wasm_bindgen::JsCast` import in wgpu shell region (blocked trunk rebuild)

## Runtime verification

- `http://127.0.0.1:6178/?plugin=lowpoly` loads without preview2-shim module errors
- UI renders: semio lowpoly chrome, 3D viewport with default table mesh, Details panel, bottom tool tabs
