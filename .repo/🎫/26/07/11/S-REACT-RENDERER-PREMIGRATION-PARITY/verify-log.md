# S React Renderer Premigration Parity — Verification Log (2026-07-11)

## Environment
- `bun run dev:s` via new `.claude/launch.json` entry `s-react-dev` (port 6070, react renderer)
- Heavy concurrent-session churn during verification: RASTER/FORMS/CAD parity + window-layout refactor sessions editing `os-shell.tsx`, `ui/js/react/index.tsx` live (repeated HMR full reloads).

## Root causes found and fixed at runtime
1. **s plugin aborted in browser** — `js-sys 0.3.103` panics "cannot access imported statics on non-wasm targets" under wasm32-wasip2. Every `LocalStorageBackbonePort` read (studio catalog!) and every `vcs` timestamp (`js_sys::Date::now`) aborted the component. Fixed by gating web_sys/js_sys to non-p2 wasm and routing p2 through the WIT host: new `host_port` module in `semio-framework-plugin` (backbone-read/write + now-ms bindings, `HostBackbonePort`, registered into `vcs::set_host_backbone_port` at plugin init), generated `host-shim.js` per plugin module (localStorage first, sync `/semio-backbone` fallback) mapped via `jco transpile --map semio:framework/host=./host-shim.js`.
2. **Studio booted with wrong plugin registry** — `framework/plugin/registry/script.ts generate` wrote a per-session FILTERED catalog into the shared generated `plugins.ts`; concurrent playground sessions (draw/raster) kept overwriting it, so studio boots saw a single plugin. Fixed: generator always emits the full catalog (runtime filtering already exists via `expandPluginRegistry`; build scope filtering is separate).
3. **Studio crashed with "Maximum update depth exceeded"** — two independent loops:
   - Radix + React 19.2.6 unstable composed refs (radix-ui/primitives#3799/#3963) in the navbar example `Select`. Fixed via `@radix-ui/react-slot` override `^1.2.5` (resolves 1.3.0) + `@radix-ui/react-select` `^2.3.3`, pruning stale nested copies.
   - `useWindowMeasuresReservePx` (ui/js/react) re-ran its layout effect every render because the `measures` ReactNode identity was a dependency → synchronous measure/setState feedback. Fixed by depending on `Boolean(measures)`.

## Verified live (http://localhost:6070/studios/default)
- Boot: home app → studio via `/studios/default`, browser title + URI routing, example select "Demo Studio".
- Studio chrome: Catalogue tree (all programs/apps, drag data), Parameters tab, Inspector panel, sync toolbar (Temporary/File/Folder/Remote), Undo/Redo/Checkpoint tools.
- Media VFS: all 5 demo instances (Semio Emblem, Emblem Copy, Jack Notes, Raster Board, Note Board) with source.json/inputs/outputs and the Brush Size parameter row.
- Compiled DAG window renders (text-editor scene, line numbers).
- Media Graph window mounts through the FLOW engine path (`engine:"flow"` capabilities + fixtureJson; flow wasm session + label canvases created).
- Presence end-to-end: per-tab identity in sessionStorage (`semio.presence.client`), `presenceHeartbeat` dispatched on 5s interval, peer record written through WIT host → `localStorage["semio.backbone.presence:default"]` with clientId/name/selection/updatedAtMs.

## Blocked (external, concurrent work)
- Interactive canvas verification (node drag/connect persistence, catalogue drop-to-spawn, camera round-trip, two-tab presence overlay): window CONTENT areas currently collapse to 0×0 repo-wide — reproduced identically on `?plugin=flow` — caused by the in-flight COMPOSABLE-WINDOW-CONTENT-LAYOUT / PER-WINDOW-TOOLBAR-STRIP refactor in concurrent sessions. The s-side logic for these behaviors is covered by unit tests (below); re-run the interactive walkthrough once the layout refactor lands.

## Tests
- `cargo test -p s-plugin -p semio-framework-os -p semio-framework-plugin -p vcs` — 82 passed (new: flow-fixture camera round-trip + diff to ops, connect/disconnect/remove diffs, nodeGraphEdit setFixture moves + camera persistence, nodeGraphViewport persistence, presence upsert/prune/self-exclusion, heartbeat publishes peer).
- `bunx vitest run` in framework/renderer/react — 64 passed (new: flow-engine capability routing, presence peers overlay rendering).
- `cargo check -p s-plugin --target wasm32-wasip2` — clean.
