# Get S Working End to End

## Status (2026-07-23 reopen)

Studio build (`SEMIO_PLUGIN=s`) failed while packaging **all** plugin crates; e2e then needed a stale selector fix.

### Failures found

1. **flow** — `flow_grid_measures_group` truncated to empty `WindowMeasure::Group { }`; test had orphaned Group fields injected into assertions.
2. **lowpoly** — `lowpoly_selection_utility_options` truncated the same way; test match arm had orphaned `value/min/...` fields.
3. **note** — five `WindowMeasure::Group` constructors missing `value/min/max/step/ready/loading/waiting/on_change`.
4. **e2e** — search toggle id moved from `s-media-graph-window-search-toggle` to `framework.window.sMediaGraph.search.toggle`.

### Fixes

- Restored flow grid group (visible/snap/factor) and cleaned the test.
- Restored lowpoly selection utility options; cleaned match-arm corruption.
- Injected missing Group Option fields in note.
- Updated S studio e2e engagement expand selector.

### Verification

- `cargo test -p flow-plugin --lib window_measures_surface_lod_proximity_and_grid` — ok
- `cargo build -p flow-plugin --target wasm32-wasip2 --release` — ok
- `cargo build -p lowpoly-plugin --target wasm32-wasip2 --release` — ok
- `cd framework/product/os/dev && SEMIO_PLUGIN=s bun ./📜️script.ts build` — exit 0 (33 plugin crates incl. `s`, vite dist)
- `SKIP_PLUGIN_BUILD=1 SEMIO_PLUGIN=s bun ./📜️script.ts dev` — Vite on `http://127.0.0.1:6070/`
- `bun ./📜️script.ts verify e2e` — PASS (home → studio → windows → draw spawn via engagement → undo/palette/home)


## Status (2026-08-03 continue)

Home → Demo Studio → studio workbench confirmed via Playwright on `http://127.0.0.1:6070/`.

### Root causes fixed

1. **Missing `command_from_action`** on Home + Space — shell `{action,args}` never reached typed `handle`.
2. **`VcsDocumentApp` body_key:`windowId` suffix** — Home/Space render now strips it (puzzle3d pattern).
3. **Host effects wire shape** — DslValue `{kind,value}` normalized to externally-tagged `HostEffect`.
4. **`parseSpaceShellPath`** expected `/studios/` but apps navigate to `/spaces/`.
5. **`LoadDocument` pack/spr** — wired through `AppChannelClient.loadDocument` / `loadAppDocumentPack`.
6. **Scene pack codec** — editor/node-graph `sync_from_scene_pack` now accepts wire-body (`encodePackValue`) with pack-shell fallback; surface WASMs rebuilt.

### Still noisy (non-blocking)

- `flow-extension-bim` / `trinity` plugin-modules 404.
- `setContributions` push skipped on home (uncaught opt-in; try/catch).
- Full unskipped `framework-os-dev:dev` (all plugins + wasm-opt) still slow; use `SEMIO_WASM_OPT=0` / `SEMIO_PLUGIN_ONLY=<id>`.

### Verify

- Double-click Demo Studio → title `semio · s · studio`, URL `/spaces/default`, Workflow / Media VFS / Compiled DAG visible with demo nodes.
