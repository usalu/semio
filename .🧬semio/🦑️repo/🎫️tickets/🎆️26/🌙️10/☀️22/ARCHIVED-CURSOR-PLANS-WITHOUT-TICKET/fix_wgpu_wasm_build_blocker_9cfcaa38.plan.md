---
name: Fix wgpu wasm build blocker
overview: The edge/face hover/select/highlight fixes from previous sessions are already implemented and covered by passing unit tests in Rust, but the wasm bundle that the browser actually loads has never rebuilt successfully with them, because an unrelated Cargo dependency-gating bug breaks the entire wasm32 build of the renderer.
todos:
 - id: fix-cargo-toml
   content: Move semio-framework-plugin-host dependency to the wasm32-excluded target section in framework/renderer/wgpu/rs/Cargo.toml
   status: completed
 - id: cargo-check
   content: cargo check the renderer crate for both wasm32 and native targets to confirm the fix and no regressions
   status: completed
 - id: rebuild-wasm
   content: Rebuild the wgpu renderer wasm bundle via bun framework/renderer/wgpu/script.ts wasm and confirm trunk succeeds with fresh artifacts
   status: completed
 - id: reverify-browser
   content: Reload the dev:lowpoly server/browser and re-verify vertex/edge/face hover, select, and highlight behavior
   status: completed
isProject: false
---

## Root cause found

The wasm build of the `semio-framework-renderer-wgpu` crate is currently broken, which means the browser is running a stale build from `Jul 7 20:17` that predates every edge/face fix made in this ticket (both today's regression fix and the earlier round). This explains why vertices still look correct (older, already-deployed behavior) while edges/faces look unchanged (fixes never shipped).

Evidence:

- The dev server terminal running `bun run dev:lowpoly` (terminal `16.txt`) shows a hard build failure ending in:

```
cargo:warning=error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-unknown"'
...
4: cargo call to executable 'cargo' with args: '["build", "--target=wasm32-unknown-unknown", ...]' returned a bad status: exit status: 101
```

- `cargo tree -i zstd-sys --target wasm32-unknown-unknown` traces this straight to `wasmtime-cache -> wasmtime -> semio-framework-plugin-host -> semio-framework-renderer-wgpu`. `zstd-sys` compiles native C code via `sccache`/`clang`, and there is no wasm32 C sysroot available, so it hard-fails.
- `wasmtime`/`semio-framework-plugin-host` is a native-only plugin runtime. Its actual Rust usage in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) is already correctly gated everywhere:

```4486:4487:framework/renderer/wgpu/rs/lib.rs
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::WasmPluginRuntime;
```

- But the Cargo dependency declaration itself in [framework/renderer/wgpu/rs/Cargo.toml](framework/renderer/wgpu/rs/Cargo.toml) is unconditional:

```22:23:framework/renderer/wgpu/rs/Cargo.toml
semio-framework-plugin = { path = "../../../plugin/rs" }
semio-framework-plugin-host = { path = "../../../plugin/host/rs" }
```

so Cargo still tries to compile `semio-framework-plugin-host` (and transitively `wasmtime`/`zstd-sys`) for `wasm32-unknown-unknown` even though nothing on that target path uses it. `semio-framework-plugin-host` has exactly one consumer in the whole workspace (verified via grep across all `Cargo.toml` files), so this is safe to move.

## Fix

1. In [framework/renderer/wgpu/rs/Cargo.toml](framework/renderer/wgpu/rs/Cargo.toml), move `semio-framework-plugin-host` out of the unconditional `[dependencies]` block into the existing `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` block (next to `ureq`, `rfd`), leaving `semio-framework-plugin` (the wasm-safe, wasmtime-free SDK crate) where it is.
2. Run `cargo check -p semio-framework-renderer-wgpu --target wasm32-unknown-unknown` to confirm the dependency graph no longer pulls in `wasmtime`/`zstd-sys`, and run `cargo check -p semio-framework-renderer-wgpu` (native, with `native-bin` feature) to confirm the native path (which still needs `WasmPluginRuntime`) keeps compiling.
3. Rebuild the actual browser bundle with `bun framework/renderer/wgpu/script.ts wasm`, confirming trunk succeeds and produces fresh artifacts (new hashed `.js`/`.wasm` files with a current timestamp) under `framework/product/os/dev/renderer-modules/wgpu/`.
4. Reload/verify the running `dev:lowpoly` server picks up the new build (trunk watch should auto-rebuild; otherwise restart it), then hard-reload the browser tab to bypass any cached wasm.

## Re-verify the original bug report

Once a real, current build is deployed:

- Vertex mode: confirm hover/select/highlight still work (regression check).
- Edge mode: hover under cursor, click-select, and rectangle marquee preview/commit all highlight the correct edge.
- Face mode: hover under cursor, click-select, and rectangle marquee preview/commit all highlight the correct face (fill + outline).

If any of these still fail after the wasm bundle is confirmed fresh, that will be a genuine logic issue distinct from the build pipeline, and will need separate targeted debugging in [infinite/world/rs/lib.rs](infinite/world/rs/lib.rs) (`pick_component_at`, `append_component_overlays`, `sync_world3d_state`) — but this is expected to already be correct based on the passing `infinite_world`/`kernel_3d_scene` test suites from the prior session.
