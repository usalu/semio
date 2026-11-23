# WGPU generation3d native serve (2026-11-23)

## State
- 6118 serves HTTP 200 (`screen -S g3dwgpu`, log `🗑️generated/wgpu-native-serve.txt`). Renderer wasm (natively built, new frame-loop edits compiled), boot/worker bundles and activation receipt are in place.
- Page boots to `worker-boot-failed: plugin-registry-invalid: selected module dependency graph`: the plugin modules for generation3d are NOT staged (only `raster` exists in `.cache/b/p/m/browser/prototype/modules`). Battery NOT run (before: not runnable; after: not runnable). Smoke evidence: `🗑️generated/wgpu-native-smoke/console.txt`.

## Start
`screen -dmS g3dwgpu "$TF/serve-generation3d-wgpu-native.sh"` (Vite host via `native-launch.ts` envelope; launch config `generation3d-wgpu-native` added). Wasm dir: `.cache/g3d-native/wasm-dev` (`S_G3D_WASM`).

## Built / fixed
- `wgpu-native-wasm-build.sh`: cargo (`~/.cargo/bin`, `CARGO_HOME=.cache/b/r/m/cargo/home`, own target `.cache/g3d-native/t`, wasm-bindgen from the cargo home). Re-run after source edits; `repo` graph not needed. Target dir deleted for disk space (29 GB), so a rebuild is cold (~1 h).
- `wgpu-native-closure.ts` + `wgpu-native-generate.ts`: graph-free `generate-browser-closure` and boot/renderer-boot/frame-worker generators (updated `🔣️taxonomy.json` and the wgpu `📋️project.json` inputs).
- `wgpu-native-activate.ts`: activation receipt. `wgpu-native-modules.ts` + `wgpu-native-components-build.sh`: native component build and jco staging for the 11-component closure (flow, 9 flow extensions, procedural); built hub crates only partly (stopped at ~800 crates, disk), modules script untested.
- Source fixes: natural-file input moved out of the React-importing tutorial module (`🛠️ShellHelpers/🎞️media-import`, consumers: wgpu plugin-bridge, document-transfer scope, test); shell-plugins and shell-actions got the `semio-ui-retained-tree` dependency (+ Cargo.lock); `LocalFileRow` import (shell-prefs); `'static` key (doc-open route-not-found); window-actions `TaskManagerToolRun*` imports and display-ready `Label`; `local_document_folder_uri` path conversion; tree hit domain const and `document_key` (shell-pointer); `directory_now_ms` import; un-gated `context_menu_paths_equal/submenu_open` and `context_menu_item_at_path` import (view-overlays); `has_pending_settle` uses `interaction_mut`; `MailboxRuntime: JobThreadTransfer` (wasm Rc); `host_unsaved` moved to inherent impl; `pub use RendererAssetDecodeTurn(Kind)`; `CanvasClipPathJson` export; chrome_text/chrome_icon imports and borrow fix (view-panels). Several look like stale-overwrite regressions of commit 752 vs 751.

## Blockers / next
1. `repo`'s own component driver (`wgpu-native-components.sh`) waited ~80 min on the exclusive `.cache/b/d` lease held by another lane's `tools setup all` (pid 17436), then started; its result is unknown (log `🗑️generated/wgpu-native-components.txt`). Check `ls .cache/b/p/m/browser/prototype/modules` for `🌀️procedural`; if present, run `bun wgpu-native-activate.ts`, reload 6118, run `bun 🐍️wgpu-battery.mjs`.
2. Otherwise run `wgpu-native-components-build.sh` (needs ~25 GB free; check `df`), then `bun wgpu-native-modules.ts`, then `wgpu-native-activate.ts`. The 9 extensions are separate isolated workspaces; confirm the browser needs them.
3. Disk was full; I deleted only my own `.cache/g3d-native/{t,tp2,stage}`.
