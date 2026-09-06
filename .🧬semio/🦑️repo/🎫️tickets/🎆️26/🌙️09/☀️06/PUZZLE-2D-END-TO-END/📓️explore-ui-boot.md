# Puzzle 2D UI Boot — Exploration Report

Read-only exploration. No cargo/build/test run, no edits, no git actions. All paths repo-relative.

---

## 1. `dev 2d` — the exact boot chain

### 1.1 Entry chain

- `package.json:78` `"dev:puzzle:2d": "bun ./📜️script.ts dev 2d"`.
- Root router: `📜️script.ts:22606-22613` `ScriptRouter(...).register("dev", DevScript)`; `import.meta.main` block at `📜️script.ts:35339-35342` only intercepts `policy`, otherwise calls `runWorkspaceScriptMain(router)`, which dispatches `dev` → `DevScript.run(["2d"])` (`📜️script.ts:476-516`).
- `DevScript.run`: `"2d"` matches none of `storybook`/`storybook-static`/`s`/`multi`/`mcp` (`📜️script.ts:478-514`), falls to `resolvePlaygroundDevApp(["2d"])` (`📜️script.ts:502-506`, defined `177-181`), which calls `resolveFrameworkOsPlaygroundPlugin(catalog, ["2d"])`.
- Catalog row: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts:66`:
  `{ variant: "puzzle2d", pluginId: "puzzle", cratePath: "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust", app: "s.puzzle.puzzle2d@1/*#editor", aliases: ["2d","puzzle 2d"], ports: { react: 6012, wgpu: 6112 }, examples: ["🌙️capsule-dream","🌲️concrete-forest","🎬️demo-session","🏗️nakagin-capsule-tower"], engines: ["./✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"], assets: [] }`.
  `resolveFrameworkOsPlaygroundPlugin` matches alias `"2d"` (`🧰️framework/…/📚️library/📦️packages/🟦️typescript/🟦️.ts:2506-2517`) → `plugin = "puzzle2d"`.
- `runFrameworkOsPlaygroundDev("puzzle2d", [])` (`📜️script.ts:196-204`) shells `bun nx run @semio-tech/framework-os-dev:dev -- puzzle2d`, env from `frameworkOsPlaygroundDevEnv` (`🟦️.ts:2519-2530`): `SEMIO_PLUGIN=puzzle2d`, `SEMIO_RENDERER` defaults to `"wgpu"` unless `SEMIO_RENDERER=react` is already set (launch.json's `🛠️dev🧩️puzzle🩻️2d⚛️react` entry sets it), `S_OS_PORT` defaults per-renderer from the catalog row (`react:6012`, `wgpu:6112`).

### 1.2 What `@semio-tech/framework-os-dev:dev` (`DevScript` in `🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1215-1364`) actually runs for `puzzle2d`

`filterPlugin = "puzzle2d"` (a variant id, not a host filter — `isHostPlaygroundFilter` only matches `s`/`note`/etc.), so every plugin-crate build below is **scoped to the single `puzzle` plugin id**, not the ~20–37/59-crate whole-studio build memory notes describe for `dev s`. That whole-studio number does **not** apply to `dev 2d`/`dev 3d`/`dev 5d`.

**react renderer** (`SEMIO_RENDERER=react`, i.e. the `⚛️react` launch.json entry, port 6012):
1. `publishShardWorker()` (`📜️script.ts:1222`) — unconditional, writes `/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js`.
2. `streamPluginBuilds = true` (react + `SKIP_PLUGIN_BUILD!=1`, line 1252) ⇒ acquires a per-variant build lease (`acquirePluginBuildLease`, lines 1257-1282) so two concurrent `dev puzzle2d` processes (e.g. a peer session) don't both build; a follower just serves.
3. Holder path: `ensurePluginRegistry("puzzle2d")` (line 1286, def. `509-516`) → `bun 🧰️framework/…/🔌️plugin/📇️registry/📜️script.ts generate` (no cargo) + `writePlaygroundSession`.
4. `buildEngineWasm("puzzle2d", "react")` (line 1287, def. `934-958`) — **wasm-pack/cargo, budgeted `buildBudgetMs()` = 1,200,000 ms (20 min, `SEMIO_BUILD_BUDGET_MS` override) per sub-build**, skipped entirely if `SKIP_ENGINE_BUILD=1` or renderer≠react (line 942):
   - `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts wasm` → `semio-framework-surface` (shared, every react-renderer app).
   - `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts wasm` → `semio-framework-editor` (shared).
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📜️script.ts wasm` → `semio-framework-os-flow` (shared "flow-core", every app pulls this even if unrelated to flow).
   - Then, per the catalog row's own `engines: [...]` (line 953): `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts wasm` → `WasmScript` (`✏️s/…/rust/📜️script.ts:9-16`) → `runWasmPackWebBuild({ rsDir: this.root, skipEnvVar: "PUZZLE_BOARD_SKIP_WASM_BUILD", wasmBaseName: "semio_puzzle", shipProfile: "wasm-release", noDefaultFeatures: true, pkg: { name: "@semio-tech/puzzle-wasm", ... } })` — a `wasm-pack build --target web` (dev profile in dev mode) of the **same crate** `semio-s-plugin-puzzle`, producing `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle_bg.wasm` + `semio_puzzle.js` (already present on disk from a prior build). **Env skip:** `PUZZLE_BOARD_SKIP_WASM_BUILD=1`. It also short-circuits (`🟦️.ts:2892-2903`) if the wasm output is newer than every crate source file (`wasmPackInputsStale`), logging `"pkg/semio_puzzle_bg.wasm up to date → skipping wasm-pack build"`.
5. `buildPluginsStreaming("puzzle2d")` (line 1356, only for the lease holder, called AFTER Vite is already serving) → `preparePluginBuildTargets` → `resolvePluginBuildTargets` filters the generated catalog to `pluginId === "puzzle"` (one crate) unless `SEMIO_PLUGIN_ONLY=<id>` overrides the filter (`📜️script.ts:518-534`) → `buildPluginCatalog` runs, per target, `cargo rustc -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile wasm-dev -- -C link-arg=-zstack-size=<N>` (`pluginCargoArgs`, `🧑‍💻dev/…/📜️script.ts:85,103-109`; `wasm-release` instead of `wasm-dev` when `SEMIO_BUILD_MODE=ship`, override via `SEMIO_PLUGIN_PROFILE`) — **this is the "plugin wasm32-wasip2 component"** the ticket asked about, the OS-hosted editor/panels/windows/actions logic.
6. Vite dev server (`runViteBunxDev`) is started on `S_OS_PORT` (6012) with `SEMIO_PLUGIN=puzzle2d`, `VITE_SEMIO_PLUGIN="puzzle"`, `VITE_SEMIO_APP_ID="s.puzzle.puzzle2d@1/*#editor"` — **before** step 5 finishes; the plugin materializes and streams into the already-open shell (comment `📜️script.ts:1245-1251`).

**Skip flags** (`🧑‍💻dev/…/📜️script.ts`):
- `SKIP_PLUGIN_BUILD=1` — skip the ~1-crate `buildPluginCatalog` cargo step entirely (`streamPluginBuilds` branch collapses to registry+engine-wasm only, line 1285); used by `served`/hub multi-user launches.
- `SKIP_ENGINE_BUILD=1` — skip `buildEngineWasm` wholesale (surface/editor/flow-core AND puzzle's wasm-pack build), line 942.
- `PUZZLE_BOARD_SKIP_WASM_BUILD=1` — skip only the puzzle crate's wasm-pack (`@semio-tech/puzzle-wasm`) build.
- `SEMIO_PLUGIN_ONLY=<pluginId>` — force-rebuild one plugin crate even under a host (`s`) filter; irrelevant to `dev 2d` since it's already single-plugin-scoped.
- `SEMIO_BUILD_MODE=ship` — switches plugin cargo profile `wasm-dev`→`wasm-release` and the puzzle wasm-pack profile to `wasm-release`.

**wgpu renderer** (`SEMIO_RENDERER=wgpu`, i.e. the `🧊️wgpu🌐️wasm` launch.json entry, port 6112): same registry+`buildEngineWasm`/`buildPlugins` sequence (non-streaming: `buildPlugins` not `buildPluginsStreaming`, line 1290, because `streamPluginBuilds` requires `renderer === "react"`), then instead of Vite it shells `bun 🧰️framework/…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts serve` (`📜️script.ts:1321-1335`) — the Trunk-served native-Rust wgpu shell — with `SEMIO_PLUGIN=puzzle2d`. This is the whole-shell wgpu renderer target (Rust `winit`/`wgpu` window via Trunk+WASM), a different thing from the puzzle-specific `@semio-tech/puzzle-wasm` `BoardSession` (see §2). The `🖥️native` launch.json entry instead runs that same wgpu-targets crate's `📜️script.ts native puzzle2d`, a native (non-browser) binary, not a browser dev server at all.

### 1.3 Output locations / ports
- Plugin wasm32-wasip2 component + registry catalog: materialized under `🧰️framework/…/🧑‍💻dev/🔌️plugin-modules/` (served at `/plugin-modules/` — same static path Storybook's `staticDirs` entry serves, `.storybook/main.ts:82-87`).
- `@semio-tech/puzzle-wasm`: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/{semio_puzzle_bg.wasm,semio_puzzle.js,semio_puzzle.d.ts}` (confirmed present on disk).
- Framework engines (`surface`, `editor`, `flow-core`): each crate's own `pkg/` (same `runWasmPackWebBuild`/wasm-bindgen pattern).
- Ports: react 6012, wgpu(browser/Trunk) 6112 (per catalog row, overridable via `PUZZLE_2D_PLAY_PORT`/`S_OS_PORT` in launch.json envs — note `PUZZLE_2D_PLAY_PORT` is set in `.vscode/launch.json:1259,1279` but `DevScript` actually reads `S_OS_PORT`, not `PUZZLE_2D_PLAY_PORT` — that launch.json var appears to be informational/legacy, not consumed by `frameworkOsPlaygroundDevEnv`, which only checks `S_OS_PORT`/`extra.S_OS_PORT`; worth flagging but not chased further here).

### 1.4 Timing expectations (from the sibling `26/09/02/PUZZLE-3D-END-TO-END` ticket, since no build was run here)
- `📓️engine-wasm-status.md` (that ticket): an **isolated, mostly-cold** `CARGO_TARGET_DIR` check of `surface`/`editor`/`flow-core` alone was still running native+wasm32-wasip2 checks 8 minutes in when it stopped for other reasons — explicitly flagged as "not representative of steady-state." The 20-minute `BUILD_BUDGET_MS` ceiling (`🧰️framework/…/📚️library/🏃️process/🟦️.ts:12-16`) is the hard timeout every individual cargo/wasm-pack sub-build in §1.2 is billed against; a cold `dev 2d` (surface + editor + flow-core + puzzle's wasm-pack + puzzle's wasip2 component, cargo-serialized against a possibly shared/locked `target/`) can plausibly consume several of those 20-minute budgets end-to-end if the shared `target/` is contended — this ticket's own `📓️status.md` log records exactly that ("Host load 220, 97 cargo/rustc processes from peers... seeding private `target-p2d-e2e` from a peer's rmeta"). Memory's "~20 WASM plugins, slow cold boot" note is about `dev s` (whole studio), not `dev 2d` — `dev 2d` only ever builds one plugin crate (`puzzle`) plus the three shared framework engines, so it should be materially faster once the target dir isn't lock-contended.

---

## 2. Editor windows/panels/modes and the viewer — what renders what

App booted by `dev 2d`: `s.puzzle.puzzle2d@1/*#editor` (the **editor** role; `#viewer` is a separate, not-launched-by-default app — see §2.4).

Artifact root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

### 2.1 Default layout — one mode, three windows, all Canvas2d/Board2d

`✏️editor/🎭️modes/✏️edit/🦀️.rs` is the only mode (`PUZZLE2D_PLAY_MODE_EDIT = "edit"`, line 15). Its `layout()` (lines 28-30) is a "triptych":
```
create_default_layout(&[overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID], "row", Some(&[50.0, 25.0, 25.0]), Some(&["Overview","Detail","Selection"]))
```
Three window **instances**, all opened by default, all `SurfaceKind::Canvas2d` at the `WindowKindDefinition` level:

| Window kind id | Path | Width | `ZOOM_SCALE` | `BODY_KEY` |
|---|---|---|---|---|
| `2d-overview` | `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️.rs:14-29` | 50% | 0.68 | `puzzle2d.play.overview` |
| `2d-detail` | `.../🔍️detail/🦀️.rs:12-24` | 25% | 2.15 | `puzzle2d.play.detail` |
| `2d-selection` | `.../🎯️selection/🦀️.rs:12-24` | 25% | 0.36 | `puzzle2d.play.selection` |

Each window's `render()` (e.g. `overview/🦀️.rs:51-52`) delegates to the shared `edit::render_canvas(document_json, envelope, WINDOW_KIND_ID)` (`✏️edit/🦀️.rs:154-162`), which builds a `Board2dScene` (`✏️edit/🦀️.rs:117-146`) and encodes it as `semio_framework_ui_contract::SurfaceKind::**Board2d**` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs:113`) — a different, more specific surface-kind enum than the `WindowKindDefinition`'s `SurfaceKind::Canvas2d` declaration; `Board2d` is the wire surface kind the React `Board2dHost` element actually keys off.

**All three windows render via the React `Board2dHost` component + a live WASM `BoardSession`, never via the wgpu whole-scene renderer** — see §2.2. Only `2d-overview` is `interactive: true` in its `Board2dScene` (`✏️edit/🦀️.rs:136`, `interactive: pane == overview::WINDOW_KIND_ID`); detail/selection are read-only framed views of the same fixture (camera math in `puzzle2d_pane_camera`, `✏️edit/🦀️.rs:80-101` — detail zooms into the last-placed node, selection frames a fixed lower-left-ish quadrant, mirroring the pre-migration TS behavior per the doc comment).

Each window also emits a `WindowEngagement` HUD (`puzzle2d_engagement`, `✏️edit/🦀️.rs:167-199`): a text input (`engagementInput`/`engagementSubmit`/`engagementAbort` actions) plus a status line `"{node_count} Nodes · {edge_count} Edges · {lod} LOD"`. This is a small piece of genuinely non-empty React chrome even when the board itself is empty (see §3).

Per-window `WindowMeasure`s: `options::lod::measure` + `options::brush::measure` (each window's lines ~40-46) — LOD dropdown and brush-weight controls, sourced from `✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs` and `.../🖌️brush/🦀️.rs`.

### 2.2 The rendering engine: `BoardHost` (native Rust, server/plugin-side) vs `BoardSession` (wasm-pack, browser-side)

Two distinct "board" pieces exist, easy to conflate:

1. **`BoardHost`** — a native Rust struct from the shared framework board module `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:2261`. Puzzle wraps it via `puzzle_board_host()`/`puzzle_board_host_normal()` (`✏️editor/⚙️engine/🎲️board-host/🦀️.rs:9-18`), wiring in puzzle's own themed icon table (`⚙️engine/🔣️icons/🦀️.rs:10-13`, generated at compile time by `build.rs` from `🌱️metabolism/🔣️icons` SVGs — **no runtime asset fetch needed for icons**, they're baked into the wasm binary). This `BoardHost` is compiled into BOTH the plugin's own `wasm32-wasip2` component (server/OS-hosted logic — hit-testing math shared with the mutation/command layer) AND into `@semio-tech/puzzle-wasm` below (same source, different build target).
2. **`BoardSession`** — a `#[wasm_bindgen]` struct at `✏️editor/🌉️wasm/🦀️.rs:102-137`, exported from the crate's wasm-pack `--target web` build (`@semio-tech/puzzle-wasm`, §1.2 step 4). `BoardSession::new()` holds one `BoardHost` plus a `CanvasGpuSession` and exposes `attach_canvas(canvas, w, h, dpr)` to bind WebGPU presentation directly (`🌉️wasm/🦀️.rs:135-137`).
3. **Wiring to React:** `✏️editor/🌉️wasm/🟦️.ts:1-18` dynamically imports `../../../../../../../../📦️packages/🦀️rust/pkg/semio_puzzle.js` (the same wasm-pack output) and registers `PUZZLE_BOARD_SESSION_FACTORIES = [{ kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#editor", create: createPuzzleBoardSession }, { ...#viewer... }]` — an `AppSurfaceSessionFactory[]` consumed by `resolveAppSurfaceSessionFactory` (`🧰️framework/…/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:344-350`). `Board2dHost` (`…/🧱️elements/🖥️Board2dHost/🟦️.tsx`, 1044 lines) reads `BoardSessionFactoryContext` (its `🟦️.tsx:27,348`) to construct one live `BoardSession` per pane and drives it with real pointer/wheel/keyboard events, mounting a `<canvas>` under class `.semio-board-2d-host` (confirmed by the existing Playwright spec, §4).

**Conclusion: all three puzzle2d windows are rendered by React (`Board2dHost`), which itself embeds a genuine client-side WebAssembly+WebGPU engine (`@semio-tech/puzzle-wasm`'s `BoardSession`) — independent of whether the top-level `SEMIO_RENDERER` is `react` or `wgpu`.** The `SEMIO_RENDERER=wgpu` launch.json variant swaps the *outer OS shell chrome* (Trunk-served native Rust winit/wgpu window vs. Vite-served React DOM shell); it does not change how a Board2d surface itself paints, since `Board2dHost` is a React element either way — worth independently confirming the wgpu shell actually mounts a React host at all before relying on this for the wgpu launch entries specifically (not chased further here; the react entry is unambiguous).

### 2.3 Panels

`✏️editor/📌️panels/`:
- `🗿️artifact/🦀️.rs:19,56` — a `PanelTabDefinition` + `render(envelope, labels)`.
- `🔍️inspection/🦀️.rs:15,33` — details/inspector panel (the one `PUZZLE-2D-INSPECTOR-KIND-DROPDOWNS`, §5, added kind dropdowns to).
- `🛍️catalogue/🦀️.rs:21,89` — node/handle kind catalogue browser, `render(fixture, labels)`.

All three are plain React panel content (`UiAssemblyResult<BuiltNode>`), not board/canvas surfaces — no wasm session involved.

### 2.4 Viewer (not launched by `dev 2d`)

`👁️viewer/🦀️.rs:79` `create_puzzle2d_viewer()` builds a separate `AppDefinition` for app id `s.puzzle.puzzle2d@1/*#viewer`, mode `👁️view` (`👁️viewer/🎭️modes/👁️view/`). It shares the same `PUZZLE_BOARD_SESSION_FACTORIES` registration (§2.2 item 3, second array entry) so its board surfaces would also mount `BoardSession`, but `dev 2d`'s catalog row (`🎠️playgrounds.ts:66`) only names `app: "s.puzzle.puzzle2d@1/*#editor"` — the viewer role is not part of this boot path and would need a separate `SEMIO_APP`/`VITE_SEMIO_APP_ID` override to reach.

### 2.5 Terminology / config / presence

- `✏️editor/🗣️terminology/🦀️.rs` — `Puzzle2dLabels`, native/reuse × en/de (same 4-combination pattern as puzzle3d's terminology file the 3d ticket documented).
- `✏️editor/🎚️config/🦀️.rs` — `Puzzle2dConfig`/`Puzzle2dPlayRuntime`, including `example_load_generation`/`example_load_id` (§3) and `lod_mode_by_pane`.
- `✏️editor/👥️presence/🦀️.rs` — `Puzzle2dPresence` schema for multi-user cursors/selection; not chased further (out of scope for a single-dev boot check).

---

## 3. Examples: loading, switching, defaults, and asset routing

### 3.1 Default boot state is **empty**, not concrete-forest

Unlike puzzle3d (`default_fixture()` = `CONCRETE_FOREST_EXAMPLE_FIXTURE`, per the 3d ticket's `📓️window-example-render-path.md`), puzzle2d's `ArtifactEditor::initial_snapshot()` is:
```rust
fn initial_snapshot() -> Puzzle2dPlaySnapshot {
    set_active_example::warm_examples();
    Puzzle2dPlaySnapshot(serde_json::to_value(default_empty_fixture()).unwrap_or(Value::Null))
}
```
(`✏️editor/🦀️.rs:1871-1874`), and:
```rust
pub fn default_empty_fixture() -> Value {
    json!({ "schema": PUZZLE2D_FIXTURE_SCHEMA, "nodes": [], "edges": [] })
}
```
(`✏️editor/🦀️.rs:90-96`). `Puzzle2dConfig::default()` also leaves `example_load_id: None`, `example_load_generation: 0` (`✏️editor/🎚️config/🦀️.rs:256,286-287`), and `pending_effects` (the only startup-effect hook, `✏️editor/🦀️.rs:1891`) wires to `set_fill_count::reconcile_snapshot_read`, unrelated to examples. **No effect auto-loads an example on boot.** So on a fresh `dev 2d`, all three windows legitimately show an empty board (0 nodes, 0 edges, grid/canvas chrome only) until something dispatches `setActiveExample` — this is a real gap against ticket definition-of-done #5 ("every window renders non-empty content") that a verification pass must account for by explicitly triggering an example load, not by just loading the page.

### 3.2 `setActiveExample` is a real, resumable, already-migrated action

`✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs` implements a **generation-tagged, chunked, resumable** load (not a synchronous swap like puzzle3d's): `begin_active_example` resets runtime + enqueues a `setActiveExampleStep` effect; `step_active_example` walks stages `clearEdges → clearNodes → manifest → clearCompatibility → addCompatibility → catalogs → nodes → edges`, at most `MAX_MUTATIONS_PER_STEP = 4` mutations per dispatched step (line 16), each step self-queueing the next via `Effect::DispatchAction` until done. This is exactly the shape of a properly `Migrated` interactive action, and indeed: `PUZZLE2D_RETAINED_TOOL_IDS = ["setActiveExample", "forceLayout", "addNode", "applyBoardEvents"]` (`✏️editor/🦀️.rs:1072`) — `setActiveExample` is one of the ticket's "6 Migrated" actions (`📓️status.md` baseline), so **example switching should genuinely dispatch and complete at runtime**, unlike puzzle3d's `setActiveExample`/`setFillCount` which the 3d ticket documented as still `BatchOnlyPendingRewrite` (expected `interactive-job.not-ui-safe` fault there). A real boot check should NOT expect that fault code on puzzle2d's `setActiveExample` — seeing it there would itself be a regression finding.

Only two example ids resolve to real content:
```rust
pub(crate) fn canonical_example_id(example_id: &str) -> &'static str {
    match example_id {
        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID | "concrete" => PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID | "nakagin" => PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID,
        _ => "",
    }
}
```
(`🛍️set-active-example/🦀️.rs:33-39`) — anything else (including the catalog's own `"capsule-dream"`/`"demo-session"` ids, see §3.3) resolves to `""`, i.e. `target("")` returns the static `EMPTY` snapshot (line 45-46 target dispatch), so selecting either of those two extra catalog-advertised examples would **silently reset the board to empty**, not error.

### 3.3 Registry-vs-disk example mismatch

`🎠️playgrounds.ts:66` lists 4 examples for `puzzle2d`: `🌙️capsule-dream, 🌲️concrete-forest, 🎬️demo-session, 🏗️nakagin-capsule-tower`. On disk under this artifact:
- `📚️examples/🌲️concrete-forest/` and `📚️examples/🏗️nakagin-capsule-tower/` (top-level artifact examples — real DSL fixtures wired via `crate::examples::puzzle2d::{concrete_forest,nakagin_capsule_tower}::SOURCE`, referenced by `set-active-example/🦀️.rs:26-27`).
- `✏️editor/📚️examples/🎬️demo-session/` (a **different kind** of "example": a recorded command-replay fixture, `🖼️assets/🎮️.cmd.semio` + its own `🧪️tests/`, used for scripted test scenarios, not a `setActiveExample` target).
- **No `capsule-dream` directory anywhere under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/`** (confirmed via `find … -iname "*capsule-dream*"`, no hits) — the registry's example list for `puzzle2d` appears to have been generated/copied from puzzle3d/5d's catalog rows (which DO have a real `capsule-dream` — puzzle3d/5d launch.json even has dedicated `…🎛️capsule🌙️dream…` entries) rather than reflecting puzzle2d's actual example set. Net: of 4 advertised, 2 are real switchable examples, 1 (`demo-session`) is a same-named but functionally different artifact (test fixture, not a UI-selectable example), 1 (`capsule-dream`) doesn't exist for 2d at all.

### 3.4 Asset routing: `assets: []` for puzzle2d is very likely correct, not a gap

Puzzle3d/5d catalog rows declare `assets: [{ kind: "mesh-collection", route: "/mesh", ... }, { kind: "static-dir", route: "/infinite-fixture", ... }]` because their 3D scenes instance GLB meshes served at runtime. Puzzle2d's `Board2dScene` (§2.2) draws nodes/edges/handles procedurally through `BoardHost`'s vector canvas plus **compile-time-embedded** SVG icons (`⚙️engine/🔣️icons/🦀️.rs`, `build.rs`-generated `include!`) — there is no glTF/mesh asset to serve, so an empty `assets: []` for the 2D variant is architecturally consistent rather than a missing-route bug. (Flagging this explicitly since the ticket brief asked whether 2D is "missing" asset routes relative to 3D — the honest answer is "not missing anything it needs," not "yes, gap.")

---

## 4. Runtime verification conventions in this repo

### 4.1 `[DEBUG]` logging convention

Root `CLAUDE.md`/AGENTS.md rule: temporary logs get a `[DEBUG] ` prefix so they're easy to strip later. Existing dev-script logging already follows a bracketed-tag convention (`[dev] …`, `[puzzle/board] …`, `[policy] …`) that any added `[DEBUG]` instrumentation should sit alongside, not replace.

### 4.2 The 3d ticket's browser-probe method (transferable to 2d)

`26/09/02/PUZZLE-3D-END-TO-END/📓️runtime-verification-plan.md` defines the template: per-window DOM/id checks, canvas non-blank checks (`getImageData`), a documented fault-code table (`interactive-job.not-ui-safe`, `RawWireLimit`, `runtime live cleanup faulted for instance …`, `puzzle command exceeds fixed semantic work capacity`), and an explicit go/no-go checklist. `📓️window-example-render-path.md` traces the exact Rust call chain from `default_fixture()`→`render()`→scene JSON, and flags the "silent empty fixture" trap (`from_json_str(...).unwrap_or_else(|_| empty_fixture())`) as the main false-negative risk for "is it actually rendering." Puzzle2d's own default-empty-boot behavior (§3.1) is a related but distinct trap: an empty board here is **the correct default state**, not a silent parse failure — a verification script must not conflate "0 nodes because nothing was loaded yet" with "0 nodes because JSON parsing silently failed."

`📓️live-cleanup-verdict.md` source-traces puzzle3d's "runtime live cleanup faulted for instance 1" fault to a structurally-closed gap (both `build_document_store_owners`/`build_config_store_owners` return `Some(...)` for `Puzzle3dPlayApp`, `✏️editor/🦀️.rs:6710-6716` there) and concludes the fault is **not currently reproducible against current source** for puzzle3d, though never empirically re-run. This exact same check (does `Puzzle2dPlayApp` override both hooks?) was **not done here** — worth a quick grep (`build_document_store_owners\|build_config_store_owners` in puzzle2d's `✏️editor/🦀️.rs`) before assuming puzzle2d is or isn't exposed to the same fault class; not chased in this pass since it's plugin-migration scope, not UI-boot scope.

### 4.3 Storybook: a REAL runtime verification channel exists for puzzle2d — but with two broken references

Unlike puzzle3d (whose only story, `World.stories.tsx`, was found completely broken by `📓️storybook-render-probe.md` — dead fixture-DSL import paths + a nonexistent `@semio-tech/puzzle-3d-ui-rs` package), **puzzle2d has a genuine, more sophisticated Playwright spec already written**: `.storybook/puzzle-2d.spec.ts` (180 lines, 10 `test(...)` cases) drives the real `BoardSession` WASM engine through actual pointer clicks, wheel-zoom, and Delete-key input against story ids like `puzzle-2d--overview-select`, asserting against a `data-testid="puzzle2d-board-debug"` JSON readout (`{selection, camera, activeUtility, nodeCount, edgeCount}`) and a visible `.semio-board-2d-host canvas` element (`.storybook/puzzle-2d.spec.ts:47-58`) — this is exactly the kind of no-full-OS-boot, no-cargo-plugin-build verification channel that would let a dev iterate on rendering without waiting through §1's wasm build chain.

**However, two stale-taxonomy issues (same class the 3d ticket found in `World.stories.tsx`) currently block it, neither touched here:**
1. `.storybook/stories/puzzle/2d/Board.stories.tsx:10-11` imports `"../../../../framework/product/os/module/renderer/js/react/index.tsx"` — an **ASCII-transliterated path that does not exist anywhere on disk** (confirmed: `ls framework/product/os/module/renderer/js/react/index.tsx` → No such file or directory; no alias for it found in `.storybook/main.ts`'s `buildStorybookAliases`/`buildScopeAliases`). The real file is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`. This will fail to bundle exactly like `World.stories.tsx` did for the 3d ticket (Vite/Rollup "Could not resolve" at build time) — not verified by an actual Storybook build in this pass (forbidden — no builds), but the path is unambiguously absent, so the failure mode is not in doubt.
2. `.storybook/scopes.ts:92` — the `puzzle/2d` scope's `sourceRoots` points at `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻️2d`, a directory that **does not exist** (only `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d` exists on disk, confirmed via `find` at the plugin root). This affects `buildScopeWatchIgnores` (stale root just means an always-empty ignore-set entry, low impact) but is the same "taxonomy sweep renamed the directory, this reference wasn't updated" pattern flagged repo-wide in the 3d ticket.

Net: whoever picks up runtime verification for puzzle2d should fix these two references first (both are pure path corrections, no cargo/plugin-build dependency, same conclusion the 3d ticket reached about `World.stories.tsx`) — then `.storybook/puzzle-2d.spec.ts` becomes a fast, cargo-free way to prove the `BoardSession` engine genuinely paints and responds to input, independent of the full `dev 2d` OS-boot path in §1.

### 4.4 "Every window renders non-empty content" — how to actually measure it here

Given §2.1 (three windows, all Board2d) and §3.1 (empty by default): the operational definition should be (a) `.semio-board-2d-host canvas` is present and has a non-zero bounding box for each of the three window instances, AND (b) after dispatching `setActiveExample({exampleId:"concrete-forest"})` and waiting for its multi-step chain to settle (poll the engagement HUD's `"{n} Nodes · {m} Edges"` status text, §2.1, until n>0), each window's fixture-derived content (node/edge counts) matches across all three panes (same underlying `envelope.fixture`, different camera framing per §2.1's zoom table) — not "canvas pixels are non-blank" alone, since an empty-but-correctly-rendering board is also non-blank (grid/background) yet has 0 semantic content.

---

## 5. Known closed puzzle2d UI tickets (`26/06/02/...`)

All five are `status: "closed"` in their `🎫️ticket.json`; none carry a closing "note" field, so "what was left open" is inferred from each ticket's own artifact file where one exists (most don't retain a status/log beyond the ticket.json description — consistent with being fully resolved, one-shot fixes):

| Ticket | What it did | What's left open |
|---|---|---|
| `PUZZLE2D-BRUSH-TOOL` | Implemented the paint-style Brush tool (offset slot hitboxes, compatible-node preview, auto-flush on leave, Tab-cycle candidates, flush-distance window measure) per its retained `puzzle2d_brush_tool_a806828f.plan.md` plan. | Plan file's own step 6 called for temporary `[DEBUG]` logs during validation "before removing them" — not verifiable here whether they were actually removed (no build run); worth a quick grep for stray `[DEBUG]` strings in the brush tool source (`✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs`, `🛠️tools/🪣️fill/🦀️.rs`) if picking this up. |
| `PUZZLE-2D-INSPECTOR-KIND-DROPDOWNS` | Added node/handle/edge kind-catalog dropdowns to the details/inspection panel, mirroring puzzle3d's inspector. | None recorded; only a one-line ticket.json description survives, no artifact files. |
| `FIX-PUZZLE-2D-DUPLICATE-IMPORT` | Fixed a duplicate import of `puzzle2dFixtureMergedKindCatalogs` in the (then-TS) playground renderer `react/index.tsx` that broke the Vite/Babel build for the puzzle-2d entry. | This was a TS-era bug in a file that has since been ported to Rust (`edit::render_canvas` etc., §2) — the specific broken file/import no longer exists under that name; low residual risk, but a reminder that this class of "duplicate import breaks the puzzle-2d Vite entry" bug has precedent. |
| `CENTER-NODE-TEXT-LABELS-IN-PUZZLE-2D` | Centered (previously left-aligned) text labels on puzzle2d board nodes. | None recorded. |
| `PUZZLE2D-THEME-CHROME-SYNC` | Fixed dark/light theme desync (puzzle2d sometimes showed light chrome while the rest of the UI was dark) via a reference-counted chrome-lease stack in `@semio-tech/ui-react`, `useLayoutEffect` instead of `useEffect` for theme subscribers, `color-scheme` on `html`/`body`, and a puzzle2d CSS-probe mirror of the `.dark` class; plus a documented "play follow-up" fix for a dark→light boot flash (`bootstrapElementsSurfaceChromeDocument` + inline `index.html` script + sync calls in `puzzle/2d/play/index.ts`/`mountPlaygroundApp`). | The retained `theme-sync-fix.md` explicitly documents this as landed, not speculative — but a real `dev 2d` boot check should still visually confirm no dark→light flash on first paint, since that follow-up fix touches boot-order-sensitive code (`useLayoutEffect` timing, inline `<script>` in `index.html`) that a taxonomy rename elsewhere could silently break without any type error. |

---

## 6. Concrete boot-verification plan for the main session

### 6.1 Build (foreground, isolated target dir, nohup'd logs to the scratchpad — never `/tmp`, per this ticket's own §📌️important conventions and the "Preview Servers Vanish" / "Subagents Must Build In Foreground" memory notes)

```bash
cd /Users/ueli/Documents/semio
mkdir -p .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🗑️generated

# Isolated target dir avoids the shared target/ lock other live sessions hold (see 📓️status.md log).
export CARGO_TARGET_DIR=/private/tmp/claude-.../scratchpad/target-p2d-dev   # use this session's real scratchpad path
export SEMIO_RENDERER=react
export S_OS_PORT=6012

nohup bun ./📜️script.ts dev 2d \
  > /private/tmp/claude-.../scratchpad/dev-puzzle2d-react.log 2>&1 &
disown
```
Poll readiness (don't sleep-loop blindly — poll the port):
```bash
until curl -fsS http://127.0.0.1:6012/ >/dev/null 2>&1; do sleep 2; done
```
Watch for these log lines specifically (from the traced chain in §1.2): `"plugin registry generation failed"` (fatal, would exit(1) before Vite starts), `"[puzzle/board] pkg/semio_puzzle_bg.wasm up to date → skipping wasm-pack build"` or a fresh `"wasm build done in Xs"` line, `"program build scope: puzzle"`, and eventually Vite's own "ready in Xms" / "Local: http://127.0.0.1:6012/" banner.

### 6.2 Open and drive the browser

- Navigate to `http://127.0.0.1:6012/`.
- Confirm three `.semio-board-2d-host canvas` elements are present (one per `2d-overview`/`2d-detail`/`2d-selection` instance) with non-zero bounding boxes — read_page/find, not just a screenshot, to get real geometry.
- Read each window's engagement-HUD status text (`"{n} Nodes · {m} Edges · {lod} LOD"`, §2.1) — expect `0 Nodes · 0 Edges` on fresh boot (§3.1 — this is correct, not a bug).
- Dispatch the concrete-forest example: locate and use whatever UI control invokes `setActiveExample` (examples dropdown/selector — not located by file in this pass; grep `✏️editor/🦀️.rs`/panels for where `puzzle2d_action("setActiveExample", ...)` is wired into a control, or use the network/action-dispatch inspector to fire it directly) with `args: { exampleId: "concrete-forest" }`.
- Poll (network tab or the HUD status text) until the chained `setActiveExampleStep` dispatches stop firing (§3.2 — multiple sequential dispatches, `MAX_MUTATIONS_PER_STEP=4` per step, so a fixture with many nodes/edges takes several round-trips) and the HUD shows `N Nodes · M Edges` with `N,M > 0`.
- Confirm all three windows now show the SAME node/edge counts (shared fixture, different camera framing) — this is the actual "non-empty content" proof requested by the ticket's definition of done.
- Switch to `nakagin-capsule-tower` the same way; confirm counts change and then switch back to `concrete-forest`; confirm counts return to the original values — proves example switching round-trips.
- Trigger one editor action (e.g. click a node in the interactive `2d-overview` pane to select it, or drag the LOD/brush `WindowMeasure` controls) and confirm its dispatch appears in the network/action log with a success response (not `interactive-job.not-ui-safe` — that fault would be a real regression here per §3.2, unlike on puzzle3d where it's an accepted known gap).
- Check server-side stdout/stderr (the nohup log) for `"runtime live cleanup faulted for instance"` after the above interactions — per §4.2, unresolved whether puzzle2d exhibits the same class of fault puzzle3d's ticket source-traced as closed; a clean log here would be a positive signal, a repeating fault would need the same `build_document_store_owners`/`build_config_store_owners` override check §4.2 flags as not yet done.

### 6.3 Optional faster iteration path (once §4.3's two path fixes land)

```bash
STORYBOOK_SCOPE=puzzle/2d bunx storybook build -c .storybook --output-dir <scratchpad>/storybook-static-puzzle2d
```
then run `.storybook/puzzle-2d.spec.ts` against it — no cargo, no plugin-fleet materialize, exercises the same `BoardSession` WASM engine directly. Useful for isolating "does the board engine itself work" from "does the full OS/plugin-dispatch boot work," exactly the distinction the 3d ticket's `📓️storybook-render-probe.md` drew.

---

## Files referenced (non-exhaustive index)

- `📜️script.ts:177-204,476-516,22606-22613,35339-35342`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2499-2530,2860-2963`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:12-40`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts:66`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:85-109,509-608,895-958,1215-1364`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:1-17`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:50-51,90-96,1072,1871-1874,1891,2189`
- `.../✏️editor/🎭️modes/✏️edit/🦀️.rs:1-30,80-101,117-199`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/{👁️overview,🔍️detail,🎯️selection}/🦀️.rs`
- `.../✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs`
- `.../✏️editor/⚙️engine/🦀️.rs`, `.../⚙️engine/🎲️board-host/🦀️.rs`, `.../⚙️engine/🔣️icons/🦀️.rs`
- `.../✏️editor/🌉️wasm/🦀️.rs:24,75-137`, `.../✏️editor/🌉️wasm/🟦️.ts`
- `.../✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue}/🦀️.rs`
- `.../👁️viewer/🦀️.rs:79`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs:83-113`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:322-350`
- `.storybook/main.ts`, `.storybook/scopes.ts:90-92`, `.storybook/puzzle-2d.spec.ts`, `.storybook/stories/puzzle/2d/{Board,Fixtures}.stories.tsx`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/{📓️runtime-verification-plan.md,📓️window-example-render-path.md,📓️storybook-render-probe.md,📓️engine-wasm-status.md,📓️live-cleanup-verdict.md}`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️02/{PUZZLE2D-BRUSH-TOOL,PUZZLE-2D-INSPECTOR-KIND-DROPDOWNS,FIX-PUZZLE-2D-DUPLICATE-IMPORT,CENTER-NODE-TEXT-LABELS-IN-PUZZLE-2D,PUZZLE2D-THEME-CHROME-SYNC}/🎫️ticket.json`
