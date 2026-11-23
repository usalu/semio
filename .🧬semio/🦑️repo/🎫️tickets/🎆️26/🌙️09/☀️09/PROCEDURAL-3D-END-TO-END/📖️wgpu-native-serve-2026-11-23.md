# WGPU Native Serve 2026-11-23

URL: `http://127.0.0.1:6118/?plugin=generation3d`

Curl (no listener): exit 7, `http_code` 000, `Couldn't connect to server`. Port 6118 was not in `LISTEN`.

`serve-generation3d-wgpu-native.sh` was not started. The script's own header requires renderer wasm in `.cache/g3d-native/wasm-dev`, generated boot/worker bundles, the procedural plugin module, and an activation receipt. None of those are present. `🗑️generated/wgpu-native-serve.txt` was not created.

## Command

```sh
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/serve-generation3d-wgpu-native.sh
```

It exports `SEMIO_PLUGIN=generation3d`, `SEMIO_RENDERER=wgpu`, `S_OS_PORT=6118`, `S_G3D_WASM=/Users/ueli/Documents/semio/.cache/g3d-native/wasm-dev`, then runs `serve-generation3d-wgpu-native.ts`. That file builds a Vite middleware server from `createCompletedWgpuConfigurationV1` and serves `/renderer-modules/wgpu/*` from `S_G3D_WASM`. Log target: `🗑️generated/wgpu-native-serve.txt`. Ready line: `WGPU generation3d native host ready`.

## Renderer wasm

`wgpu-native-wasm-build.sh` (pid 27300) finished. Log `🗑️generated/wgpu-native-wasm-build.txt` ends with `CARGO_EXIT=101` (line 117564). Pid 27305 (`cargo build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown -j 2 --offline --frozen`, target dir `.cache/g3d-native/t`) is gone. `.cache/g3d-native/wasm-dev` is empty (created 2026-11-23 00:49, no bindgen files). `BUILD_DONE` was not printed. wasm-bindgen was not reached.

Last compile failure: `semio-renderer-wgpu-shell-pointer` (lib), `error: could not compile` after `E0609` `no field document_id on type ShellSyncOwner` in `elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🔁️deferred-menus/🦀️.rs:196`. Earlier in the same log: `E0521` borrowed `key` escapes in `route-not-found/🦀️.rs` (`semio-renderer-wgpu-shell-doc-open`), and `TaskManagerToolRun` / `Label::resolve` missing in `semio-renderer-wgpu-shell-window-actions`. Those are renderer sources other agents are editing. They were not patched here (serve-path fixes only: receipt writer or env, and only when a real module exists).

Intended command, already in the script:

```sh
cargo build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown -j 1 --offline --frozen \
  --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' \
  --config 'build.target-dir=".cache/g3d-native/t"' \
  --config 'build.build-dir=".cache/g3d-native/t/b"'
# then wasm-bindgen on $TGT/wasm32-unknown-unknown/debug/semio_renderer_wgpu_component.wasm
# --target web --no-typescript --out-name semio-renderer-wgpu-component --out-dir .cache/g3d-native/wasm-dev
```

A retry was not started: 3 other cargos were already running (pids 34300, 37689, 97721) while memory free was 54% and 1-minute load was about 7.7. The gate allows a new cargo only under 3 compiling lanes, free memory at least 40%, and 1-minute load at most 20.

## Plugin module and receipt

Same gap as the React host. `.cache/b/p/m/browser/prototype/modules` has no `🌀️procedural`. `.cache/b/p/m/browser/prototype/runtime/react/generation3d` does not exist. WGPU activation would live at `developmentRuntimeRoot(workspace, "generation3d", "prototype", "wgpu")/activation/🔣️receipt.json`. `publishActivationReceipt` / `nextActivationReceipt` stay unused until staged plugin bytes exist.

`publishComponentBrowserV1` is the module publisher to copy, not a new layout. It stages under the prototype plugin-modules root and the component deployment directory `🌀️procedural`, copying preview2 vendor, assembling transpiled cores, probing `describe()`, then writing `🛂️.descriptor.semio`, `🔣️.json`, the bridge, and `🧾️receipt.json`. The component crate is a WIT cdylib (`semio:procedural`, `semio-hub-procedural-component`), target `wasm32-wasip2`, not a wasm-bindgen crate. The native component driver never got there: see the React note. It is blocked in `fixedOptionsGuardV1` → `acquireQueuedResourceLease` (shared) on `native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d`, whose sqlite is held by setup-all bun pid 17436. No `[native-build]` line in `🗑️generated/wgpu-native-components.txt` or `native-component-procedural-3.txt`.

Boot and worker bundles were not generated (`wgpu-native-generate.ts` was not run). Nothing to serve under the Vite config.

## Remaining blockers

- Renderer wasm build exited 101; `.cache/g3d-native/wasm-dev` is empty.
- Procedural plugin module and both activation receipts are absent.
- Component driver is stuck on the setup-all b/d lease, before `runNativeBuildDriverV1`.
- Direct cargo is waiting on a free compile lane (3 cargos already).
- WGPU listener stays down until wasm-dev, the plugin module, and a real receipt exist.
