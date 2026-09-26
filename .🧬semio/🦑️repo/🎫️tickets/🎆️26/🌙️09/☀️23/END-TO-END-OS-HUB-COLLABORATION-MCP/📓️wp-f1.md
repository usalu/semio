# F1 — frontend runtime performance of the os `s` shell and its plugin surfaces

Slice F1, session 12 (2026-09-26 01:4x). Coordinator = main Claude Code chat. Serves 6620–6629, hubs 8120–8129 (none needed).
Captures `wp-f1/generated/` (expendable); durable logs `.🧬semio/🌐hub/s12-f1-logs/`. Guests = W2's restage4 (`consistent=60`).
Budget: an idle surface paints ≈ 0 frames and keeps the main thread < 2 % busy; interaction input → paint p95 under one frame budget.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Status

| # | item | state | evidence |
|---|---|---|---|
| F1-0 | own `dev s` serve (6620), census | **running (kept for the post-publish live proofs)**: serve pid **5419** (`bun ./📜️script.ts serve s react dev`, local-only, HMR off) → vite 5423; boot `ready:s`, 60/60 loaded, 148 programs | `f1-idle-trial1.json` |
| F1-1 | idle-cost census: every staged kind (editor + viewer) — busy %, frames/s over 10 s idle, JS heap, wasm memory | **measured, complete: 146 programs (76 editors + 70 viewers).** 4 continuous loops found (raster editor, puzzle2d editor, puzzle5d editor + viewer: 60 frames/s) → fixed (F1-2); after the fixes **every kind: max 0.2 main frames/s, max 1.5 % main busy (measured under fleet load 25–70; the 6 hung rows and the 2 borderline rows re-measured at load 4: all 0 frames/s, ≤ 0.6 %)** — tables §1 | `f1-idle-r1-{editors,viewers,loops,loops-after}.json`, `f1-idle-r2-*.json`, `f1-idle-tables.md` |
| F1-2 | root-fix every idle offender (demand-driven through the ONE shared scheduler; bounded explicit animations) | **landed (TS host, live on 6620)**: the one scheduler paints a frame only when dirty / continuous / inside an EXPLICIT trailing window; `paintNow` satisfies pending demand; owner calls before a frame coalesce into one paint; the implicit 250 ms trailing window is gone (NodeGraph + TiledMap declare `EASED_SURFACE_TRAILING_WINDOW_MS` explicitly). Live: jack typing 34 → **16** paints / 16 keys; dag drag 157 → **67** paints. **Idle offenders found by the census and fixed (same scheduler):** `Paint2dWasmCanvas` (raster editor: 60 frames/s, 120 rAF/s, 6.5 % busy → **0 frames/s, 0.8 %**) and `Board2dHost` (puzzle2d editor 60 frames/s, 180 rAF/s, 15.1 % → **0, 0.8 %**; puzzle5d editor/viewer 60 frames/s, 8.6/7.8 % → **0, 0.02/0.7 %**); matrix rows raster/puzzle2d/puzzle5d/jack/dag/gismap **6/6 PASS** with the new layer/node painted. Laws demand-frames v2 **10/10** (red: double paint → 2 FAIL, continuous loop → 6 FAIL), engine-contract **671/671**, NodeGraph/TiledMap/Canvas2dHost suites **96/96**, renderer `tsc` rc 0 (edited files in the program) | `f1-law-demand-frames-*.txt`, `f1-engine-contract-1.txt`, `f1-element-suites-1.txt` |
| F1-2a | `EditorSession` re-parses the font face + reshapes text every paint (framework_editor wasm) — cache parsed faces + shaping | **landed + LIVE (both browser bundles rebuilt by F1 in the `wasmshort` lane, rc 0): jack keydown→paint p50 217 → 11.3 ms, p95 346 → 55 ms; EditorSession paint p50 91.8 → 5.9 ms; gismap paint 354 → 22.9 ms.** Two root causes: (1) no reuse → content-keyed shaped-label cache (schema-first bounds); (2) the label hot path (usvg/ttf-parser/rustybuzz/fontdb/…) ran at `opt-level = 0` → `BROWSER_CANVAS_HOT_CRATES` at opt-level 3 in dev for the two bundles only. Before (browser, jack query editor typing): paint p50 **84.5 ms**, 98 % in `usvg::Tree::from_str`; keydown→paint p50 213 / p95 846 ms. Native: cold `build_scene` 8.2 ms → warm **0.12 ms** (68×). Laws: canvas label-shapes 5/5 + canvas text/label/draw-list 21/21, editor 85/85 (83/83 after F1-5) incl. `an_unchanged_frame_reshapes_no_label`; wasm32-unknown-unknown = both bundles built rc 0; `cargo check -p semio-framework-os-infinite --target wasm32-wasip2` rc 0 (05:20, `wasmshort`) | `f1-latency-jack-type-before.json`, `f1-test-infinite-1.txt`, `f1-test-editor-1.txt` |
| F1-3 | interaction latency input → paint p50/p95 per family (text edit, canvas drag, graph node move, 3D orbit) + fix worst | **measured (table §3)**; worst two = text editor (EditorSession, 217 ms p50) and map (MapSession, 354 ms paint), both usvg-per-paint → F1-2a; TS part live (jack 217 → 132 ms p50) | `f1-latency-*.json` |
| F1-4 | budget laws (idle frames = 0 after settle) per surface family, red on a continuous loop; engine/renderer suites green | **landed + green**: (a) canvas package `🪶️demand-frames` v2 (GraphWasmCanvas + the one scheduler) **10/10**, red 2 / 6 FAIL; (b) renderer `🪶️surface-idle-frames` (new language-agnostic fixture `🧑‍🎨engine/🧫️fixtures/🪶️surface-idle-frames/🔣️.json`; the REAL Board2dHost + Paint2dHost mounted by React DOM with a stub session on a faked clock: mount ≤ 8, idle 0, change 1–4, idle 0, unmount 0) **2/2**, red with the pre-F1 loops (board 77, paint 64 renders on mount); (c) canvas `🏷️label-shapes` 5/5 + editor unchanged-frame law. Suites: engine-contract 672 + element suites 119 + surface-idle 2 = **793/793**, renderer `tsc` rc 0 | `f1-law-*.txt`, `f1-suites-3.txt` |
| F1-5 | text-input model of the canvas text editor (coordinator 05:2x): caret-relative insertion, token boundaries never implied by input; fixture law slow + fast, start/middle/end of a token, IME; live proof at real typing speed | **landed (Rust `✍️editor` 05:28, before the freeze; TS host) + laws + live PASS**: fixture `✍️editor/🧫️fixtures/⌨️text-input/🔣️.json` (18 sequences, 5 echo schedules); Rust `EditorHost` law under both token schedules **83/83** (red on the old code: `RETURN n a m e`); Chromium native `<textarea>` oracle **18/18** + echo + refusal laws **26/26**; live `f1-typing-proof.mjs`: jack editor = guest buffer = native textarea at **20 / 60 / 150 / 400 / 1000 ms per key** | `f1-test-editor-2.txt`, `f1-law-text-input-oracle-3.txt`, `f1-typing-proof-jack-*.json` |
| F1-6 | every typed run coalesces into one amendable ledger edit (coordinator 06:0x, P0): census of TextEditor/EditorHost editors, fixes + ≥ 1000-character laws as prepared patches (guest freeze) | **host side landed + live PASS on writer; guest side prepared** — census: editable TextEditor windows = writer (already coalesces) + trinity jack (does not: refused from the 65th key); 5 derived read-only TextEditor windows now turn read-only after one refusal (was 300 refusals / 158 keys). Host defects found by a 1379-key run and fixed: `queue-full` (> 256 pending turns) → one latest-wins delivery channel; echoed selections threw the caret back → echoes of the editor's own state never apply text or selection; keyless text input (dead keys, emoji picker) was lost → `beforeinput insertText`; Cmd+Z/Cmd+Shift+Z did nothing → document history chords. **Live writer: 1137 typed characters with pauses/caret moves/corrections saved exactly (1065 = model = Chromium oracle), 0 refusals, ONE undo → start, ONE redo → run.** Guest patch set (jack coalesce_key, vcs `Emit::amend`, SDK `typing_run()` + fixture, writer + jack ≥ 1000-char laws): `apply.py --dry-run` clean, written not compiled | `f1-typing-run-live-writer-7.json`, `f1-typing-census-r{3,5,6}.json`, `wp-f1/patches/typing-coalescing/` |

## Log

- 01:41 read AGENTS.md, session-12/11 preambles, `📓️wp-s15.md` §Session 12. Report skeleton created.
- 01:4x `nx run …:serve-s-react-dev` waited forever ("Waiting for … in another nx process": a peer's nx serve holds the same target) →
  stopped my nx tree by pid (3969/3971/3972 + 4 plugin workers) and started the serve directly in the dev package dir:
  `S_OS_PORT=6620 S_LOCAL_ONLY=1 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev` (pid **5419**, vite 5423, nice 0), log
  `.🧬semio/🌐hub/s12-f1-logs/serve-6620.txt`. It prints `[stale] <plugin>: source-changed` for most plugins (peers edited sources
  after restage4); the staged guests are restage4.
- 01:4x probe `wp-f1/f1-lib.mjs` + `f1-idle-census.mjs` (new). Metric = a CDP trace of 10 s with no input: main-thread busy = merged
  top-level task intervals of `CrRendererMain` (wall) and their thread time (CPU); frames = `BeginMainThreadFrame`/`FireAnimationFrame`/
  `Paint`/`UpdateLayoutTree` on main, `DrawFrame` on the compositor; worker busy per `DedicatedWorker thread`; in-page hooks name every
  rAF/timer requester; heap after a forced GC (`Performance.getMetrics`), main-thread wasm memory (instantiate hook), worker heap +
  backing store (`Runtime.getHeapUsage` through a non-flat auto-attach). **Positive control** (`f1-control.mjs`, `generated/f1-control.txt`):
  Home idle 0 frames/s, 0.01 % busy; + an injected empty rAF loop → **60 main frames/s, 60 rAF/s** (only 1.2 % busy: busy % alone misses
  cheap loops, frames/s does not); + a CSS spinner → **60 compositor draws/s**, 4.75 style recalcs/s. Note: S15's CDP-*Profiler* "9–12 %
  busy after the fix" is the sampling profiler's own `(program)` cost — the trace reads the same idle Home at 0.02 %.
- 01:48 Home baseline (`f1-pilot`): main 0.02 % busy, 0 frames/s, only ShellHost's 5 s interval + AgentBridge's 30 s poll; JS heap 83–109 MB,
  4 shard workers (heap 4.9 MB, backing 9 MB). Trial (`f1-idle-trial1.json`): dag editor 0.79 %, trinity jack 0.67 %, note 0.03 %, all
  **0 frames/s** (S15's GraphWasmCanvas fix holds), nothing survives close.
- 01:5x full census launched detached: editors pid **7404**, viewers pid **7440** (logs `s12-f1-logs/idle-r1-{editors,viewers}.txt`).
- 03:40 resumed after the usage cut (coordinator). The two census runs had stalled since ~01:56: their Chromium children were gone
  (bun alive, waiting on a dead browser) — stopped them by pid (7404, 7440), added a per-row watchdog (240 s) + browser-disconnect
  reject to `f1-idle-census.mjs`, resumed with `--resume`: editors pid **32043**, viewers pid **32070** (logs `idle-r1-*-2.txt`).
- 03:4x `f1-latency.mjs` (new): input → end of the first frame after it (capture listener → rAF → MessageChannel), every `renderFrame`
  of the wasm canvas modules timed by wrapping the exported classes (`framework_editor` DagSession/EditorSession, `framework_surface`
  DagSession/GraphSession/MapSession/RasterSession), Event Timing entries, a 3 s CPU profile. (Resource timing needed
  `performance.setResourceTimingBufferSize` — the default 250 entries were exhausted by the dev module graph.)
  **Before, trinity jack query editor, 16 keystrokes** (`f1-latency-jack-type-before.json`): keydown → painted frame **p50 213 ms,
  p95 846 ms**; Event Timing keydown p50 192 ms; 35 `EditorSession.renderFrame` for 16 keys, each **p50 84.5 ms / p95 109.5 ms**; CPU
  profile: 1846 ms of 3 s in `EditorSession::render_frame` → `EditorHost::build_scene` → **1798 ms in `usvg::Tree::from_str`**.
- 03:5x **F1-2a root fix (Rust, `♾️infinite/🖼️canvas/🦀️.rs` `text`):** every label paint/measure went through `usvg::Tree::from_str`
  (SVG parse + fontdb face parse (`ttf_parser::Face` / `collect_tables`) + rustybuzz shaping + outlining) on every frame — the editor's
  gutter numbers, every line, every caret/selection measure (`label_byte_world_x` = 2 shapings), the DAG's node captions and the tiled
  map's place labels. Now ONE content-keyed cache: the markup (text, size, family, fill, halo = every input of the shaping) → usvg content
  bounds + outline scene, shaped once, reused by every later paint and measure (`shaped_label`); the four markup builders are shared
  (`label_{measure,paint,tspans}_markup`), `append_shaped_label` places a cached outline. Bounded LRU (`BoundedLru`: entries, total bytes,
  per-entry bytes; accounted = key + record + `Scene::retained_bytes()` (new: command list + path/stroke/image backing)). **Schema-first
  bounds:** `🖼️canvas/🧬️schema/🏷️label-shapes/🔣️.json` (`$id semio.infinite.canvas.label-shapes/v1`) declares them as `const`s
  (4096 entries, 16 MiB, 1 MiB per entry) and `LabelShapeBounds::declared()` reads them from it (no second source). Per-thread
  (`thread_local!`: the browser session is one thread; each native test thread owns a deterministic cache); `label_shape_stats()`
  (hits / shapes / evictions / bypasses / entries / bytes). Host/browser-only like every usvg path (wasip2 arms unchanged).
  Measured: a 30-char label holds ~60 KB of outlines (fill + halo), so 16 MiB ≈ 270 long lines / 5000 gutter numbers per module.
  **Laws** (language-agnostic fixture `🖼️canvas/🧫️fixtures/🏷️label-shapes/🔣️.json` + `🧪️tests/🏷️label-shapes/🦀️.rs`): bounds == schema
  consts (+ 3 refusals); 3 LRU cases (entry bound, byte bound, re-admission/oversize bypass) with exact eviction order; every label and
  tspan row painted twice draws exactly the draw list usvg produces for the same markup directly (**third-party oracle: usvg**),
  shaped once, and every caret measure equals the usvg-direct measure and reshapes nothing on repeat; a 64-label working set is accounted
  by its outline bytes within the bounds → **5/5**; the neighbouring canvas laws 21/21 (`f1-test-infinite-1.txt`). Editor law
  `an_unchanged_frame_reshapes_no_label` (unchanged frame: 0 shapings, hits > 0; one typed char reshapes ≤ 4) → editor **85/85**.
  `cargo check -p semio-framework-os-infinite --tests` rc 0 (158 warnings, all pre-existing peers'; the only canvas warning is line 375,
  pre-existing). Native timing (dev profile, temporary print removed): cold `build_scene` **8.2 ms** → warm **0.119 ms**.
- 04:00 wasm32 checks queued detached through the mutex (behind W2 + WG8): `f1-wasm-check.sh` = editor `wasm32-unknown-unknown` +
  infinite `wasm32-wasip2`, mutex pid **39813**, log `s12-f1-logs/wasm-check-1.txt`.
- 04:0x **measurement artifact found + fixed in my own probe:** starting the CDP `Profiler` right before the input costs the main thread
  ~0.5 s (code-event logging of the whole dev heap), which read as a 459–493 ms `pointerdown` on the dag and the 846 ms keydown p95 of the
  first jack run; a `--trace` run showed pointerdown 5.6 ms. The probe now starts the profiler/trace 2 s before the first input and clears
  the rows. Numbers below are from the fixed probe.
- 04:1x **F1-2 host fix (TS, `♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx`):** measured waste on top of the 85 ms paints: the owner's
  `renderFrame` painted synchronously AND the proxied calls before it had already scheduled a frame, which painted the same state again,
  then kept painting for the implicit 250 ms trailing window (jack: 34 paints for 16 keys); a node-graph drag painted every frame twice
  (NodeGraph's own clock + GraphWasmCanvas's). The ONE scheduler now tracks dirt: a frame paints only when dirty, a continuous reason is held
  or an explicit trailing window runs; `paintNow()` (new) paints synchronously and clears the dirt; `frameDemandingSessionV1(session,
  scheduler)` maps `renderFrame` → `paintNow`, every other call → `invalidate` (coalesced); GraphWasmCanvas's pointer/resize paths use
  `paintNow` and no longer hold a `pointer` continuous reason (every event paints once; a held, still pointer paints nothing). The
  trailing window is an explicit option (default 0); NodeGraph and TiledMapHost declare `EASED_SURFACE_TRAILING_WINDOW_MS` (250, exported
  beside the scheduler) — their behaviour is unchanged. engine-contract's scheduler mock gains `paintNow` (one line, needed by the type).
  **Law** demand-frames fixture **v2** (`🖼️canvas/🧫️fixtures/🪶️demand-frames/🔣️.json`, S15's v1 extended): 6 canvas cases (idle 0; one call =
  one frame; three calls before a frame = one; `renderFrame` paints at once; calls + `renderFrame` = exactly one; unmount 0) + 3 scheduler
  cases (no trailing = one paint per demand; a declared 250 ms window is bounded ≤ 17 frames then 0; a continuous reason paints every frame
  until it ends, then one closing paint, then 0) → **10/10**; red: the pre-F1 always-paint tick → 2 FAIL; a continuous loop → 6 FAIL
  (`f1-law-demand-frames-red-{double,loop}.txt`). engine-contract **671/671**, NodeGraph (3) + TiledMap + Canvas2dHost (3) suites **96/96**,
  renderer `tsc --noEmit` rc 0 with all edited files in the program (`f1-tsc-files.txt`). Functional: `f1-editor-echo.mjs` types
  ` RETURN zq` into the jack query → the session text ends with it and the LAST paint shows the current text (`f1-editor-echo-after-ts.json`);
  the dag node drag moves `Combine` (+90,+60) and paints it selected (`f1-latency-dag-node-move-after-ts.png`).
- 04:2x census browsers died again at ~03:53 (both runs, same minute; no jetsam/crash line in the unified log for the window; rows
  gis/gisterrain editor + norm/din16798 viewer hit the row watchdog, then the reboot hung with no Chromium child). Stopped 32043/32070 by pid;
  the census now has a process watchdog (no progress 360 s → the row is kept as `hung`, exit 3) and `f1-idle-supervise.sh` (new) resumes it
  until done: editors supervisor pid **47222**, viewers **47267** (logs `idle-r1-*-3.txt`).

## 3. Interaction latency (measured, serve 6620, Metal ANGLE, fleet load 25–30)

Input → end of the first frame after it (capture listener → rAF → MessageChannel), per event; paint = one wasm `renderFrame`.

| family | program / window / gesture | before (S15 scheduler) | after F1-2 (TS, live) | after F1-2a (wasm) | capture |
|---|---|---|---|---|---|
| text edit | trinity jack / query editor / 16 keys | keydown **p50 217, p95 346 ms**; 34 paints × p50 **91.8 ms** (98 % `usvg::Tree::from_str`) | keydown **p50 132, p95 225 ms**; **16** paints × p50 87.7 ms | **keydown p50 11.3, p95 54.9 ms; paint p50 5.9 ms** (cache + hot crates at opt 3) | `f1-latency-jack-type-{before-2,after-ts}.json` |
| graph node move | dag / DAG / drag `Combine` 30 moves | (centre drag) move p50 10, p95 21 ms; 157 paints × 0.8 ms | move **p50 6.1, p95 21.4 ms**, down 14.5, up 18.2 ms; 67 paints × 0.9 ms | n/a (GraphSession paints no usvg label per frame) | `f1-latency-dag-{drag-before-3,node-move-after-ts}.json` |
| canvas drag | draw / drawing / drag 30 moves | move p50 10.3, p95 27.8 ms; down 22.7 ms; React 25 % busy (no wasm paint) | — | — | `f1-latency-draw-drag-before.json` |
| map pan | gis / gismap / drag 30 moves | move p50 1.6, p95 13.8 ms; ONE `MapSession` paint of **354 ms** at the gesture end (**299 ms in `canvas::text::append_label`**, `ttf_parser` table parse per label) | — | **paint p50 22.9 ms** (34.6 with the cache alone) | `f1-latency-gismap-pan-before.json` |
| 3D orbit | cad / Shape / drag 30 moves | move p50 7.4, p95 21.3, max 83 ms; up 104 ms (R3F events) | — | — | `f1-latency-cad-orbit-before.json` |
| 3D orbit | block3d / world / drag 30 moves | move p50 8.0, p95 20.7, max 37 ms; up 41 ms | — | — | `f1-latency-block3d-orbit.json` |
| 3D orbit | puzzle3d / Perspective / drag 30 moves | move p50 8.6–10.3, **p95 34–74 ms**, up 76 ms; 31 % busy: retained UI patch intake (`acceptOwnedUiPatches` 217 ms / 3 s) + shell re-render (`FrameworkOsShellInner` 143 ms, `World3dHost` 119 ms) — the guest republishes UI while the camera moves | — (open: the orbit should not round-trip UI per move; guest-side, see the hover-latency anatomy) | — | `f1-latency-puzzle3d-orbit-2.json` |
| canvas drag | raster / composite / drag 30 moves (after F1-2) | — | move p50 4.4, p95 18 ms | — | `f1-latency-raster-drag-after.json` |
| board drag | puzzle2d / overview / drag 30 moves (after F1-2) | — | move p50 8.6, p95 17.8 ms | — | `f1-latency-puzzle2d-drag-after.json` |
| text edit (long run) | writer / main / 1379 keys incl. pauses, caret moves, ü/ß | — | 1165 refusals (queue-full + retirement saturation) → **0**, saved exactly, one undo/redo moves the run (F1-6) | — | `f1-typing-run-live-writer-{1,7}.json` |
- 04:3x **idle offenders found** (targeted run `--only raster,puzzle`, `f1-idle-r1-loops.json`): raster editor **60 frames/s, 120 rAF/s, 6.5 %**
  (`Paint2dHost` `Paint2dWasmCanvas`: an unconditional `tick → renderFrame → requestAnimationFrame(tick)` per window, composite + navigator);
  puzzle2d editor **60 frames/s, 180 rAF/s, 15.1 %** and puzzle5d editor/viewer **60 frames/s, 8.6 / 7.8 %** (`Board2dHost`: the same
  unconditional loop per board window, 3 windows in puzzle2d). Neither wasm session animates on its own (checked: `RasterSession::render_frame`
  and the board `build_vector_scene` read no clock), so both became plain demand surfaces on the ONE scheduler:
  `Paint2dWasmCanvas` hands its owner `frameDemandingSessionV1(session, scheduler)` (every call invalidates, `renderFrame` = `paintNow`), resize
  paints once, the loop is gone, unmount disposes. `Board2dHost`: its own rAF coalescer (`scheduleRender`) and its per-frame loop are replaced by
  the scheduler (the paint republishes the probe vitals when a `scheduleRender` asked for them); `sessionRef` and the peer handle are the
  demand handle (peer mirror mutations repaint the peer), the probe vitals read the raw session (a read through the handle from inside a paint
  would re-invalidate every frame), resize invalidates, release/unmount dispose. Laws: renderer `tsc` rc 0; Board2dHost + Paint2dHost +
  NodeGraph + TiledMap + Canvas2dHost suites **119/119** (12 files); engine-contract **672/672** (a peer added one). **Live after**
  (`f1-idle-r1-loops-after.json`): raster editor **0 frames/s, 0.84 %**, puzzle2d editor **0, 0.82 %**, puzzle5d editor **0, 0.02 %**, viewer
  **0, 0.74 %**; matrix (`f1-matrix.mjs` = S15's probe writing into my folder) raster addLayer, puzzle2d/5d addNode, jack patchNodes, dag addNode,
  gismap addFeature **6/6 PASS** `[0,1,0,1]`, screenshots show the new raster layer painted in composite + navigator and the new puzzle node
  (`s15-paint-hosts-*.png`); drags: raster move p50 4.4 / p95 18 ms, puzzle2d move p50 8.6 / p95 17.8 ms (`f1-latency-{raster,puzzle2d}-drag-after.json`).
- 04:4x **F1-4 surface-family idle law (new):** `🧑‍🎨engine/🧪️tests/🪶️surface-idle-frames/🟦️.tsx` over the fixture
  `🧑‍🎨engine/🧫️fixtures/🪶️surface-idle-frames/🔣️.json` (`semio.renderer.surface-idle-frames/v1`, registered in the renderer vitest config
  beside `⏱️command-stall`): per family (board2d = `Board2dHost`, paint2d = `Paint2dHost`) the REAL host mounted by React DOM with a stub wasm
  session (a renderFrame counter; every other method a neutral no-op) on a faked 16 ms frame + wall clock: mount ≤ 8 renders, 5 s idle **0**,
  one scene change 1–4, idle 0, unmount 0. First green run caught a real cost: ONE board scene change repainted the board **16×
  synchronously** (every scene-sync effect's `applyToSession` ended with `renderFrame()`); `applyToSession` now only applies through the
  demand handle, so a commit's syncs coalesce into one frame → **2/2**. Red: the pre-F1 `Board2dHost` → 77 renders on mount, the pre-F1
  `Paint2dHost` (HEAD) → 64 (`f1-law-surface-idle-red-{board,paint}.txt`). Everything together: engine-contract 672 + NodeGraph/TiledMap/
  Canvas2dHost/Board2dHost/Paint2dHost suites 119 + surface-idle 2 = **793/793** (`f1-suites-3.txt`), renderer `tsc` rc 0. Live after the
  coalescing: matrix raster + puzzle2d + puzzle5d editors and viewers **6/6 PASS** (`f1-matrix-board-coalesce.json`; the puzzle5d board shows
  the added part painted).
- 04:5x coordinator: cancel the `wasm` ticket, use the new short lane (rule 19) and build the two browser bundles myself. Stopped my waiter
  (pid 39813 ignored TERM inside its sleep loop → `kill -9` on my own pid, removed my own ticket file). Built `@semio-tech/framework-editor-rs:wasm`
  (3 m 28 s) and `@semio-tech/framework-surface-rs:wasm` (20 s) via `wasmshort` with `--skip-nx-cache` (`f1-build-browser-wasm.sh`, new; logs
  `s12-f1-logs/build-{editor,surface}-rs-1.txt`) — both rc 0 = the wasm32-unknown-unknown compile proof of the cache; `.d.ts` unchanged (no API change).
- 05:0x **measured with the cache alone:** gismap `MapSession` paint **354 → 34.6 ms** (labels now hit the cache during a pan; the rest is vello
  replay). jack typing **unchanged per keystroke (EditorSession paint ~80 ms)**: in a one-line query every keystroke changes the only line, so its
  tspans, its layout box and the caret prefix are new content and are shaped again (≈3 shapings per key). CPU profile: the time is ttf-parser table
  reads + rustybuzz GSUB coverage binary searches — i.e. the shaping itself, compiled at **opt-level 0** (the browser bundles build `--dev`).
- 05:0x **root fix 2 (build config, scoped):** library `runWasmPackWebBuild` gains `devOptimizedCrates` → `--config profile.dev.package.<crate>.opt-level=3`
  in dev mode only; `BROWSER_CANVAS_HOT_CRATES` (library, documented with the measurement) = ttf-parser, rustybuzz, usvg, fontdb, roxmltree,
  svgtypes, simplecss, strict-num, tiny-skia-path, kurbo, peniko, vello_encoding; the editor and surface bundle scripts pass it. Scoped to these two
  builds: no root-manifest profile changes, so no other unit (native, wasip2 guests, peers' builds) changes fingerprint. Rebuilt editor (87 s) +
  surface (4 m 11 s), rc 0 (`build-{editor,surface}-rs-2.txt`).
  **Live after both fixes** (`f1-latency-jack-type-after-hot.json`): jack keydown → painted frame **p50 11.3 ms, p95 54.9 ms** (was 217 / 346);
  `EditorSession.renderFrame` **p50 5.9 ms, p95 9.7 ms** (was 91.8 / 115); the remaining keystroke cost is React (dev) re-rendering the shell.
  gismap pan: `MapSession` paint **p50 22.9 ms** (was 354); dag node move: move p50 5.9 / p95 19.6 ms, GraphSession paint 0.8 ms.
- 05:1x **found on the way (editor UX, not perf; routed, not fixed):** with paints now fast, the jack query editor's token logic shows at normal
  typing speed: (a) `EditorHost::insert_text` auto-prefixes a space whenever the caret sits at the end of a semantic token — once the guest's tokens
  for a half-typed word arrive, the next letter becomes a new word (` return` typed at 1 s/key → ` r e t u r n`); (b) a character inserted INSIDE a
  token replaces the whole token (`start > token.start && start < token.end` → the token range), so keys that land while a stale token range is
  applied drop letters (` abcdefgh` at 20 ms/key → ` ab ef h`). With the old ~90 ms synchronous paints the guest's tokens never arrived between
  scripted keystrokes, which hid (a)/(b) from the 60 ms echo probe (`f1-editor-echo-*.json`). Owner: framework editor / trinity (T12).
- 08:40 resumed after the second usage cut. Hard guest freeze (rule 20) active; the trinity jack coalesce fix stays a prepared patch (the
  file equals HEAD, verified by the coordinator). Census supervisors finished at 04:57/05:03 (both attempt 4, exit 0): editors 76 rows (73
  measured, 3 hung), viewers 70 (67 + 3 hung). Re-measured at load 4 (`f1-idle-r2-hung-{editors,viewers}.json`): norm/en1992, puzzle3d, stdio/html
  editors, norm/en1995, shooting, vcs viewers — **all clean, 0 frames/s**; the two borderline rows (demonstrator/curation editor 2.2 %, norm/en1996
  viewer 2.12 %, both GC / a websocket reconnect under fleet load) → 0.01 %, 0 frames/s (`f1-idle-r2-curation.json`).

## 1. Idle-cost census (measured, serve 6620, 10 s without input after settle)

Columns: main-thread busy % (wall / thread CPU) from a CDP trace; main frames/s (`BeginMainThreadFrame`), rAF callbacks/s, compositor draws/s;
dedicated-worker busy %; after a forced GC: JS heap, wasm linear memory of the main thread / backing store of the workers. Rows re-measured after a
fix or at low load replace the first run (noted). Budget: ≈ 0 frames and < 2 % busy.

#### editors (76 kinds)

| kind | main busy % (wall / cpu) | main frames/s | rAF/s | compositor draws/s | workers busy % | JS heap MB | wasm MB (main / workers) | verdict |
|---|---|---|---|---|---|---|---|---|
| animate/presentation | 0.01 / 0.01 | 0 | 0 | 0 | 0.09 | 90.5 | 40.8 / 2.1 | clean |
| architect/program | 1.19 / 1.11 | 0 | 0 | 0 | 0.1 | 100.9 | 61.1 / 12.5 | clean |
| block/block2d | 0.01 / 0.01 | 0 | 0 | 0 | 0.13 | 90.9 | 40.8 / 2.6 | clean |
| block/block3d | 0.02 / 0.02 | 0 | 0 | 0 | 0.21 | 98.1 | 40.8 / 3.1 | clean |
| block/block5d | 0.93 / 0.86 | 0 | 0 | 0 | 0.38 | 96.7 | 40.8 / 3.6 | clean |
| cad/cad | 1.23 / 1.14 | 0 | 0 | 0 | 0.41 | 107.8 | 61.2 / 4.5 | clean |
| dag/dag | 0.03 / 0.02 | 0 | 0 | 0 | 0.49 | 103.5 | 62.1 / 5 | clean |
| demonstrator/cad | 1.1 / 1.06 | 0 | 0 | 0 | 0.04 | 106.6 | 41.5 / 3.3 | clean |
| demonstrator/curation | 0.01 / 0 | 0 | 0 | 0 | 0.05 | 90.5 | 0 / 17.9 | clean |
| demonstrator/generation3d | 0.61 / 0.56 | 0 | 0 | 0 | 0.04 | 98.7 | 41.5 / 11 | clean |
| demonstrator/gismap | 0.76 / 0.73 | 0 | 0 | 0 | 0.23 | 109 | 168.7 / 5 | clean |
| demonstrator/playground | 0.04 / 0.04 | 0 | 0 | 0 | 0.29 | 104.4 | 62.1 / 6.5 | clean |
| demonstrator/process3d | 0.48 / 0.47 | 0.2 | 0.1 | 0.1 | 0.23 | 107.4 | 41.5 / 4.5 | clean |
| demonstrator/puzzle3d | 0.01 / 0.01 | 0 | 0 | 0 | 0.24 | 108.5 | 41.5 / 3.6 | clean |
| draw/drawing | 0.01 / 0.01 | 0 | 0 | 0 | 0.19 | 103.4 | 62.1 / 5.5 | clean |
| energy/model | 0.01 / 0.01 | 0 | 0 | 0 | 0.04 | 108.3 | 62.1 / 14.4 | clean |
| fem/fem2d | 0.02 / 0.01 | 0 | 0 | 0 | 0.37 | 85 | 0 / 9.6 | clean |
| fem/fem3d | 0.71 / 0.63 | 0 | 0 | 0 | 0.1 | 106.3 | 0 / 1.6 | clean |
| flow/flow | 0.54 / 0.45 | 0.1 | 0 | 0 | 0.16 | 96.7 | 41.4 / 2.1 | clean |
| forms/forms | 0.06 / 0.05 | 0 | 0 | 0 | 0.08 | 110.6 | 168.7 / 13.9 | clean |
| gis/gismap | 0.55 / 0.49 | 0 | 0 | 0 | 0.27 | 86.6 | 127.1 / 9.6 | clean |
| gis/gisterrain | 0.37 / 0.35 | 0 | 0 | 0 | 0.29 | 88.9 | 27.8 / 9.5 | clean |
| imperative/procedure | 0.03 / 0.03 | 0 | 0 | 0 | 0.04 | 109 | 168.7 / 6 | clean |
| layout/layout | 0.02 / 0.02 | 0 | 0 | 0 | 0.04 | 109.8 | 168.7 / 15 | clean |
| lowpoly/lowpoly | 0.01 / 0.01 | 0 | 0 | 0 | 0.04 | 92.9 | 27.8 / 2.3 | clean |
| mathematical/equation | 0.65 / 0.61 | 0 | 0 | 0 | 0.11 | 90.2 | 27.8 / 1.6 | clean |
| norm/din16798 | 0.01 / 0.01 | 0 | 0 | 0 | 0.13 | 96.6 | 27.8 / 5.1 | clean |
| norm/din18599 | 0.01 / 0.01 | 0 | 0 | 0 | 0.18 | 96.4 | 27.8 / 5.5 | clean |
| norm/din4108 | 0.01 / 0.01 | 0 | 0 | 0 | 0.15 | 96.1 | 27.8 / 4.6 | clean |
| norm/en1990 | 0.03 / 0.02 | 0 | 0 | 0 | 0.31 | 96.8 | 27.8 / 6 | clean |
| norm/en1991 | 0.05 / 0.04 | 0 | 0 | 0 | 0.32 | 96.8 | 27.8 / 6.5 | clean |
| norm/en1992 | 0.01 / 0.01 | 0 | 0 | 0 | 0.04 | 85.2 | 0 / 17.9 | clean |
| norm/en1993 | 0.97 / 0.99 | 0 | 0 | 0 | 0.12 | 85.3 | 0 / 17.9 | clean |
| norm/en1994 | 0.02 / 0.01 | 0 | 0 | 0 | 0.28 | 85.8 | 0 / 1.6 | clean |
| norm/en1995 | 0.02 / 0.01 | 0 | 0 | 0 | 0.2 | 86.4 | 0 / 2.1 | clean |
| norm/en1996 | 0.01 / 0.01 | 0 | 0 | 0 | 0.12 | 91.2 | 0 / 2.1 | clean |
| norm/en1997 | 0.02 / 0.01 | 0 | 0 | 0 | 0.15 | 87.1 | 0 / 3.1 | clean |
| norm/en1998 | 0.07 / 0.04 | 0 | 0 | 0 | 0.3 | 87.5 | 0 / 3.6 | clean |
| norm/en1999 | 0.02 / 0.02 | 0 | 0 | 0 | 0.26 | 87.5 | 0 / 4 | clean |
| norm/iso16757 | 0.01 / 0.01 | 0 | 0 | 0 | 0.26 | 88 | 0 / 4.5 | clean |
| norm/vdi3805 | 0.01 / 0.01 | 0 | 0 | 0 | 0.04 | 89.3 | 0 / 13.4 | clean |
| note/note | 0.01 / 0.01 | 0 | 0 | 0 | 0.1 | 93.4 | 27.8 / 3.1 | clean |
| playbook-module-procedural/procedural | 0.02 / 0.01 | 0 | 0 | 0 | 0.19 | 94.9 | 27.8 / 3.6 | clean |
| playbook/playbook | 0.02 / 0.01 | 0 | 0 | 0 | 0.18 | 92.4 | 27.8 / 2.6 | clean |
| procedural/generation2d | 0.52 / 0.51 | 0 | 0 | 0 | 0.18 | 91.4 | 20.7 / 5.5 | clean |
| procedural/generation3d | 0.02 / 0.01 | 0 | 0 | 0 | 0.05 | 95.6 | 20.8 / 14.4 | clean |
| process/process3d | 0.01 / 0.01 | 0 | 0 | 0 | 0.13 | 95.5 | 27.8 / 4.1 | clean |
| puzzle/puzzle2d | 0.82 / 0.74 | 0 | 0 | 0 | 0.54 | 91.9 | 43.3 / 2.1 | clean — before F1: 60.1 fps, 15.13 % → fixed |
| puzzle/puzzle3d | 0.6 / 0.56 | 0 | 0 | 0 | 0.09 | 103.1 | 0 / 1.6 | clean |
| puzzle/puzzle5d | 0.02 / 0.01 | 0 | 0 | 0 | 0.17 | 108.1 | 43.3 / 4 | clean — before F1: 60 fps, 8.64 % → fixed |
| raster/raster | 0.84 / 0.8 | 0 | 0 | 0 | 0.4 | 86.9 | 20.3 / 9.5 | clean — before F1: 60 fps, 6.46 % → fixed |
| reasoning/wires | 0.49 / 0.44 | 0 | 0 | 0 | 0.25 | 83.9 | 0 / 9.5 | clean |
| remodel/remodeling | 0.03 / 0.02 | 0 | 0 | 0 | 0.13 | 89.4 | 0 / 1.6 | clean |
| sequence/sequence | 0.87 / 0.78 | 0 | 0 | 0 | 0.17 | 98.4 | 63.5 / 1.6 | clean |
| shooting/shooting | 0.02 / 0.02 | 0 | 0 | 0 | 0.12 | 106.9 | 63.6 / 2.1 | clean |
| sourcing/curation | 0.02 / 0.02 | 0 | 0 | 0 | 0.14 | 95 | 0 / 3.1 | clean |
| space/space | 0.02 / 0.01 | 0 | 0 | 0 | 0.03 | 83.7 | 0 / 17.9 | clean |
| space/studio | 0.55 / 0.52 | 0.1 | 0 | 0 | 0.26 | 90.5 | 40.8 / 1.6 | clean |
| stdio/csv | 0.02 / 0.02 | 0 | 0 | 0 | 0.2 | 103.6 | 63.6 / 3.1 | clean |
| stdio/html | 0.27 / 0.25 | 0 | 0 | 0 | 0.14 | 100.5 | 20.3 / 2.1 | clean |
| stdio/json | 0.02 / 0.01 | 0 | 0 | 0 | 0.36 | 104.2 | 63.6 / 4.5 | clean |
| stdio/json/i-json | 0.02 / 0.01 | 0 | 0 | 0 | 0.35 | 104.4 | 63.6 / 5 | clean |
| stdio/md | 0.06 / 0.05 | 0 | 0 | 0 | 0.31 | 104.7 | 63.6 / 6.4 | clean |
| stdio/tsv | 0.06 / 0.05 | 0 | 0 | 0 | 0.29 | 103.7 | 63.6 / 3.6 | clean |
| stdio/txt | 0.03 / 0.03 | 0 | 0 | 0 | 0.21 | 103.9 | 63.6 / 4.1 | clean |
| stdio/xml | 0.78 / 0.76 | 0 | 0 | 0 | 0.39 | 104.6 | 63.6 / 5.5 | clean |
| stdio/xml/valid | 0.05 / 0.05 | 0 | 0 | 0 | 0.36 | 104.7 | 63.6 / 6 | clean |
| trinity/jack | 0.55 / 0.52 | 0.1 | 0 | 0 | 0.21 | 89.8 | 40.8 / 9.6 | clean |
| trinity/rewriting | 0.64 / 0.6 | 0 | 0 | 0 | 0.32 | 93.6 | 41.1 / 1.6 | clean |
| vcs/vcs | 0.02 / 0.02 | 0 | 0 | 0 | 0.13 | 92.4 | 41.3 / 2.2 | clean |
| wfc/bitmap | 0.02 / 0.01 | 0 | 0 | 0 | 0.12 | 92.7 | 41.3 / 2.6 | clean |
| wfc/grid2d | 0.02 / 0.02 | 0 | 0 | 0 | 0.2 | 92.9 | 41.3 / 3.1 | clean |
| wfc/grid3d | 0.72 / 0.69 | 0 | 0 | 0 | 0.17 | 101 | 41.3 / 4.1 | clean |
| wfc/wfc2d | 0.79 / 0.74 | 0 | 0 | 0 | 0.21 | 93.2 | 41.3 / 3.6 | clean |
| wfc/wfc3d | 0.3 / 0.29 | 0 | 0 | 0 | 0.15 | 98.5 | 41.3 / 4.5 | clean |
| writer/writer | 0.03 / 0.02 | 0 | 0 | 0 | 0.18 | 97.9 | 41.3 / 5 | clean |

#### viewers (70 kinds)

| kind | main busy % (wall / cpu) | main frames/s | rAF/s | compositor draws/s | workers busy % | JS heap MB | wasm MB (main / workers) | verdict |
|---|---|---|---|---|---|---|---|---|
| animate/presentation | 0.43 / 0.41 | 0 | 0 | 0 | 0.27 | 84.3 | 0 / 1.6 | clean |
| architect/program | 0.01 / 0.01 | 0 | 0 | 0 | 0.24 | 91.4 | 0 / 4.1 | clean |
| block/block2d | 0.02 / 0.02 | 0 | 0 | 0 | 0.13 | 88 | 0 / 2.6 | clean |
| block/block3d | 0.01 / 0.01 | 0 | 0 | 0 | 0.18 | 92.7 | 0 / 3.1 | clean |
| block/block5d | 0.7 / 0.66 | 0 | 0 | 0 | 0.18 | 91.1 | 0 / 3.6 | clean |
| cad/cad | 0.37 / 0.35 | 0 | 0 | 0 | 0.13 | 88.1 | 0 / 2.1 | clean |
| dag/dag | 0.96 / 0.9 | 0 | 0 | 0 | 0.26 | 93.2 | 20.3 / 4.6 | clean |
| demonstrator/curation | 0.01 / 0 | 0 | 0 | 0 | 0.12 | 90 | 0 / 1.6 | clean |
| demonstrator/playground | 1.12 / 1.02 | 0 | 0 | 0 | 0.33 | 95.1 | 40.6 / 5 | clean |
| demonstrator/process3d | 0.02 / 0.01 | 0 | 0 | 0 | 0.24 | 96.8 | 40.6 / 6 | clean |
| draw/drawing | 0.67 / 0.62 | 0 | 0 | 0 | 0.03 | 83.5 | 0 / 17.9 | clean |
| energy/model | 0.01 / 0.01 | 0 | 0 | 0 | 0.04 | 100.7 | 40.6 / 14.9 | clean |
| fem/fem2d | 0.01 / 0.01 | 0 | 0 | 0 | 0.08 | 87.6 | 20.7 / 2.1 | clean |
| fem/fem3d | 1 / 0.72 | 0 | 0 | 0 | 0.2 | 99 | 20.7 / 2.6 | clean |
| flow/flow | 0.4 / 0.38 | 0 | 0 | 0 | 0.26 | 87.2 | 20.7 / 1.6 | clean |
| forms/forms | 0.05 / 0.05 | 0 | 0 | 0 | 0.04 | 92.4 | 20.7 / 3.1 | clean |
| gis/gismap | 1.06 / 1 | 0 | 0 | 0 | 0.21 | 96.3 | 167.9 / 4.1 | clean |
| gis/gisterrain | 0.02 / 0.02 | 0 | 0 | 0 | 0.16 | 97.3 | 167.9 / 4.6 | clean |
| imperative/procedure | 0.03 / 0.03 | 0 | 0 | 0 | 0.08 | 92.6 | 20.7 / 3.6 | clean — after close 3.19 % |
| layout/layout | 0.02 / 0.02 | 0 | 0 | 0 | 0.25 | 96.5 | 167.9 / 5.2 | clean |
| lowpoly/lowpoly | 0.07 / 0.06 | 0 | 0 | 0 | 0.19 | 98 | 167.9 / 6.2 | clean |
| mathematical/equation | 0.06 / 0.05 | 0 | 0 | 0 | 0.19 | 96.7 | 167.9 / 5.7 | clean |
| norm/din16798 | 0.02 / 0.02 | 0 | 0 | 0 | 0.2 | 89.5 | 0 / 3.6 | clean |
| norm/din18599 | 0.01 / 0.01 | 0 | 0 | 0 | 0.11 | 89.5 | 0 / 4.1 | clean |
| norm/din4108 | 1.5 / 1 | 0 | 0 | 0 | 0.44 | 97.5 | 167.9 / 6.6 | clean — after close 1.96 % |
| norm/en1990 | 0.02 / 0.01 | 0 | 0 | 0 | 0.47 | 89.7 | 0 / 4.6 | clean |
| norm/en1991 | 0.27 / 0.25 | 0 | 0 | 0 | 0.17 | 89.7 | 0 / 5 | clean |
| norm/en1992 | 0.01 / 0.01 | 0 | 0 | 0 | 0.19 | 89.9 | 0 / 5.5 | clean |
| norm/en1993 | 0.01 / 0.01 | 0 | 0 | 0 | 0.15 | 90.2 | 0 / 6 | clean |
| norm/en1994 | 0.02 / 0.02 | 0 | 0 | 0 | 0.19 | 90.1 | 0 / 6.5 | clean |
| norm/en1995 | 0.01 / 0.01 | 0 | 0 | 0 | 0.02 | 83.8 | 0 / 17.9 | clean |
| norm/en1996 | 0.02 / 0.01 | 0 | 0 | 0 | 0.13 | 90.9 | 0 / 2.6 | clean |
| norm/en1997 | 0.04 / 0.03 | 0 | 0 | 0 | 0.21 | 85.3 | 0 / 1.6 | clean |
| norm/en1998 | 0.05 / 0.03 | 0 | 0 | 0 | 0.34 | 85.8 | 0 / 2.1 | clean |
| norm/en1999 | 0.02 / 0.02 | 0 | 0 | 0 | 0.32 | 86.1 | 0 / 2.6 | clean |
| norm/iso16757 | 0.02 / 0.01 | 0 | 0 | 0 | 0.36 | 86.4 | 0 / 3.1 | clean |
| norm/vdi3805 | 0.02 / 0.01 | 0 | 0 | 0 | 0.39 | 88.5 | 0 / 3.5 | clean |
| note/note | 0.02 / 0.02 | 0 | 0 | 0 | 0.35 | 84.3 | 0 / 1.6 | clean |
| playbook/playbook | 0.38 / 0.36 | 0 | 0 | 0 | 0.05 | 83.4 | 0 / 17.9 | clean |
| procedural/generation2d | 0.02 / 0.01 | 0 | 0 | 0 | 0.16 | 88 | 0 / 2.6 | clean |
| procedural/generation3d | 0.47 / 0.44 | 0 | 0 | 0 | 0.1 | 89.9 | 0 / 3.1 | clean |
| process/process3d | 0.84 / 0.58 | 0 | 0 | 0 | 0.12 | 88.2 | 0 / 2.1 | clean |
| puzzle/puzzle2d | 0.86 / 0.74 | 0 | 0 | 0 | 0.3 | 94.4 | 43.3 / 2.6 | clean |
| puzzle/puzzle3d | 0.67 / 0.62 | 0 | 0 | 0 | 0.18 | 105.1 | 43.3 / 3.6 | clean |
| puzzle/puzzle5d | 0.74 / 0.69 | 0 | 0 | 0 | 0.2 | 107.3 | 43.3 / 4.5 | clean — before F1: 60 fps, 7.75 % → fixed |
| raster/raster | 0.06 / 0.05 | 0 | 0 | 0 | 0.58 | 87.5 | 20.3 / 1.6 | clean |
| reasoning/wires | 0.02 / 0.01 | 0 | 0 | 0 | 0.21 | 86.8 | 0 / 4 | clean |
| remodel/remodeling | 0.7 / 0.66 | 0 | 0 | 0 | 0.22 | 90.4 | 0 / 4.5 | clean |
| sequence/sequence | 0.02 / 0.01 | 0 | 0 | 0 | 0.03 | 86 | 20.3 / 17.9 | clean |
| shooting/shooting | 0.48 / 0.46 | 0 | 0 | 0 | 0.18 | 94.3 | 0 / 1.6 | clean |
| sourcing/curation | 0.25 / 0.22 | 0 | 0 | 0 | 0.15 | 93 | 0 / 3.6 | clean |
| space/space | 0.01 / 0.01 | 0 | 0 | 0 | 0.03 | 83.4 | 0 / 17.9 | clean |
| stdio/csv | 0.01 / 0.01 | 0 | 0 | 0 | 0.11 | 87.1 | 20.3 / 2.1 | clean |
| stdio/html | 1.01 / 0.88 | 0 | 0 | 0 | 0.49 | 90.5 | 40.6 / 5.9 | clean |
| stdio/json | 0.01 / 0.01 | 0 | 0 | 0 | 0.18 | 89.7 | 40.5 / 3.6 | clean |
| stdio/json/i-json | 0.01 / 0.01 | 0 | 0 | 0 | 0.12 | 89.9 | 40.5 / 4 | clean |
| stdio/md | 0.02 / 0.01 | 0 | 0 | 0 | 0.21 | 90.3 | 40.6 / 5.5 | clean |
| stdio/tsv | 0.03 / 0.03 | 0 | 0 | 0 | 0.12 | 87.4 | 20.3 / 2.6 | clean |
| stdio/txt | 0.51 / 0.51 | 0 | 0 | 0 | 0.12 | 89.3 | 40.5 / 3.1 | clean |
| stdio/xml | 0.02 / 0.01 | 0 | 0 | 0 | 0.18 | 90 | 40.5 / 4.5 | clean |
| stdio/xml/valid | 0.06 / 0.05 | 0 | 0 | 0 | 0.22 | 90.2 | 40.5 / 5 | clean |
| trinity/jack | 0.04 / 0.02 | 0 | 0 | 0 | 0.24 | 90.7 | 40.6 / 6.4 | clean |
| trinity/rewriting | 0.02 / 0.01 | 0 | 0 | 0 | 0.05 | 85.4 | 20.3 / 17.9 | clean |
| vcs/vcs | 0.01 / 0.01 | 0 | 0 | 0 | 0.12 | 94.1 | 0 / 2.1 | clean |
| wfc/bitmap | 0.02 / 0.02 | 0 | 0 | 0 | 0.04 | 84.3 | 0 / 17.9 | clean |
| wfc/grid2d | 0.01 / 0.01 | 0 | 0 | 0 | 0.13 | 84.9 | 0 / 1.6 | clean |
| wfc/grid3d | 1.44 / 0.72 | 0 | 0 | 0 | 0.23 | 90.7 | 0 / 2.6 | clean |
| wfc/wfc2d | 0.12 / 0.11 | 0 | 0 | 0 | 0.11 | 85.3 | 0 / 2.1 | clean |
| wfc/wfc3d | 0.59 / 0.58 | 0 | 0 | 0 | 0.14 | 90.7 | 0 / 3.1 | clean |
| writer/writer | 0.7 / 0.66 | 0 | 0 | 0 | 0.23 | 91.4 | 20.2 / 3.6 | clean |

- 05:2x–06:0x **F1-5 text-input model** (coordinator: take the editor bug). Root causes, all in the input model, none in the renderer:
  (a) `EditorHost::insert_text` prefixed a space whenever the caret sat at a token end and REPLACED the whole token when the caret sat inside one;
  `backspace`/`delete_forward` removed whole tokens; arrows jumped token boundaries; clicks snapped the caret to token edges; selections widened to
  token/span edges (`normalize_edit_range`, `snap_offset_for_atomic`, `should_prefix_auto_space`, `token_{left,right}_boundary`,
  `allowed_composite_selection`). (b) The TextEditor host applied every scene buffer, so an echo of an OLDER edit reverted typing still in flight
  and the next keystroke sent the reverted text (letters lost). (c) Backspace/Delete never reached the session: `tagName === "TEXTAREA" &&
  key.length !== 1 → return` made both branches dead, and the hidden textarea's native deletion + `onChange` sent its whole value (deleting at the
  textarea's caret, i.e. the end). (d) IME: composing keydowns were inserted as letters; paste/cut/copy used the textarea's own (empty) selection.
  **Fix (Rust, landed 05:28 — before the freeze notice):** `insert_text` replaces the exact selection or inserts at the caret; Backspace/Delete
  remove one character or the exact selection; arrows move one character (a selection collapses to its edge); clicks/drags place the exact
  offset; `set_selection_range` is exact (`char_boundary_at_or_before`); the six helpers and the unused `headEnd`/`tailStart` span fields removed;
  double-click span selection (explicit gesture) unchanged. **Fix (TS host):** `reconcileTextEditorEchoV1` (optimistic local echo: pending
  edits + last acknowledged text; stale echoes and re-published acknowledged texts are not applied, external changes are),
  `refuseTextEditorEditV1` (a refused edit leaves the pending set; nothing pending → re-apply the guest's buffer, so a keystroke the guest did not
  take is never shown as saved), `sendEdit` awaits the input-ledger outcome; the textarea is an input sink (`onChange` ignored),
  `onCompositionEnd` / `onPaste` / `onCopy` / `onCut` go through the session, composing keydowns (`isComposing`, `Process`) are left to the IME,
  the dead TEXTAREA guard removed. **Laws:** fixture above; Rust `every_typing_sequence_ends_where_the_fixture_says_under_both_token_schedules`
  + tokenizer law; 9 old unit tests that pinned the token-implied behaviour rewritten to the new law (e.g. `backspace_inside_a_keyword_removes_one_character`),
  4 tests of removed helpers deleted → editor **83/83**; red run on the old model: `RETURN n a m e`. Renderer engine suite `⌨️text-input-oracle`
  (registered): Chromium native textarea with real key events answers all 18 sequences (third-party oracle), 5 echo schedules, 3 refusal cases
  (`✏️TextEditor/🧫️fixtures/🚫️refused-edits/🔣️.json`) → **26/26**; renderer `tsc` rc 0. Editor bundle rebuilt in `wasmshort` (05:35, rc 0).
  **Live** (`f1-typing-proof.mjs`, new: type ` return`, 3× ArrowLeft, `X`, End, ` a.name`, Backspace, `E` into the editor and into a native textarea
  holding the same text/caret; compare editor text, guest buffer and oracle after the echoes settle): **PASS at 20, 60, 150, 400 and 1000 ms/key**
  (`f1-typing-proof-jack-{3,400,1000}.json`). A 4-burst run in one page FAILED on the 4th burst — not the model: after 64 keystrokes every
  `textEdit` is refused with `module.vcsvalidation failed: batched publication requires preinstalled fixed applied and revision capacity`
  (`f1-typing-proof-jack-ledger.json`): the jack query's `text-edit` emits one window-config edit per keystroke with no coalesce key, and every
  store's applied-edit ledger holds 64 (memory: engagement coalescing). → F1-6.
- 05:5x rule 20 (hard guest freeze): my trinity `text-edit` coalesce fix + its law (written minutes before the notice) reverted to the exact
  index bytes (git diff empty) and kept as `wp-f1/patches/jack-text-edit-coalesced.rs` + `jack-unit-tests-with-law.rs`; the 05:28 editor change
  stays in the tree (coordinator: no revert); record `patches/editor-text-input-model.diff`.
- 08:5x–09:2x **F1-6 typing census** (`f1-typing-census.mjs`, new: every window hosting the TextEditor, 158–403 keys incl. pauses, a caret move back
  into the run and Backspace corrections, then ONE undo chord and ONE redo chord; buffer = the guest's text). 12 candidate programs: editable
  TextEditor windows = **writer** main and **trinity jack** editor; TextEditor windows that declare no `textEdit` (derived views) = dag compiled,
  flow compiled, sequence script + compiled, trinity rewriting jack; forms, imperative, note, reasoning, vcs, space: no TextEditor window in `s`.
  Host defects found and fixed (TS, live): (1) **Cmd+Z / Cmd+Shift+Z did nothing inside any text editor** — the shell's chords stand down for a
  focused textarea, and the editor's hidden input sink is one → `documentHistoryChord` dispatches the document's `undo`/`redo`; writer: one undo
  reverts the whole 403-key run, redo restores it (`f1-typing-census-r3.json`). (2) **Read-only text windows accepted typing and refused every
  keystroke twice** (`textEdit` + `textSelect`: 300 refusals for 158 keys, each a shell notice) → a refusal whose reason says the window takes no
  edits (`undeclared-action`, `viewer-read-only`) turns the editor read-only (`aria-readonly`, `data-read-only`; editing keys, paste, cut and
  compositions are swallowed; navigation, selection, copy and history chords keep working), and an undeclared `textSelect` stops the selection
  echo → **1 refusal** (`f1-typing-census-r6.json`). Laws: refused-edits fixture + read-only case → oracle suite **27/27**; renderer `tsc` rc 0;
  engine-contract + text-input + surface-idle + Board2dHost/Paint2dHost/NodeGraph/TiledMap/Canvas2dHost suites **820/821** — the one failure is a
  peer's in-flight edit (`🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json` gained a `waiting` locale string at 04:45 without its schema;
  `renders the schema-owned English and German lifecycle…`), not F1's; demand-frames **10/10**.
  Guest defect (frozen, prepared): trinity jack `text-edit` emits one window-config edit per keystroke with no coalesce key → from the 65th key
  every keystroke is refused `module.vcsvalidation failed: batched publication requires preinstalled fixed applied and revision capacity`, undo
  is dead (`f1-typing-census-r2.json`: 403 keys, 291 refusals). vcs `text-edit` has the same shape (`Emit::mutations`, no key) though no vcs
  window hosts the TextEditor in `s` today. Writer already uses `Emit::amend(…, "writer-text-edit")`.
- 09:2x–09:4x **long-run proof on writer** (`f1-typing-run-live.mjs`, new; fixture `patches/typing-run/typing-run.json` from
  `f1-generate-typing-run.py`: 1379 keys = 1137 typed characters incl. `Zürich`/`Straße`, 180+ Backspace/Delete, caret moves back into the run,
  Home/End, Enter, ~90 pauses of 500 ms; Chromium's native textarea answers the fixture's text exactly: `f1-typing-run-oracle.mjs` PASS).
  First run on writer (already coalesced in the guest) FAILED: 1166 refusals — `queue-full` (the per-actor command-ingress queue, > 256 pending
  turns: every key sent a `textEdit` + a `textSelect` at ~80 commands/s) and, downstream, `artifact store displaced-owner fixed retirement
  authority is saturated`. **Fix (host):** ONE latest-wins delivery channel per editor on the shared `createCoalescingActionDispatcher` — at most
  one round trip in flight, the newest `{text, selection}` delivered when it settles (`textEdit` only when the guest lacks the text, then
  `textSelect`). Second run: 0 refusals but a scrambled text → the host applied the echo of its newest SENT text while the local text was
  already ahead, and applied echoed selections (the caret jumped back mid-word). **Fix:** `reconcileTextEditorEchoV1` now answers `external`:
  a scene that only echoes the editor's own state (a pending sent text, or the last acknowledged text re-published) applies neither buffer nor
  selection; only an external change applies both; a refusal resync is an explicit forced apply. Third run: `ß`/`ü` missing — text inserted
  without a key of its own (`insertText`) went to the ignored textarea value → native `beforeinput` (`insertText`, `insertFromDrop`) inserts
  at the session caret. **Final run `writer-7`: PASS — typed 1065 characters = model = oracle, 0 refusals, one undo → start, one redo → run,
  69 s of typing.** Laws: host fixture consolidated as `✏️TextEditor/🧫️fixtures/🔁️local-echo/🔣️.json` (11 cases incl. unsent typing and
  re-published state; the refused-edits fixture merged into it) → oracle suite **29/29**; all renderer suites **822/823** (the one failure is
  the peer's artifact-creation fixture, unchanged); renderer `tsc` rc 0; typing proofs jack 20/150/1000 ms + writer 20/150 ms PASS.
- 09:4x guest side prepared: `patches/typing-coalescing/` (`apply.py --dry-run`: 5 files + 1 new fixture, clean) — see `patches/README-f1-patches.txt`;
  crates listed in `wp-w1/requests/f1.txt`.
- 09:5x idle re-census of every kind F1 touched after all TS changes (`f1-idle-r3-text.json`: dag, puzzle2d, raster, sequence, trinity jack,
  writer — editors + viewers): **12/12 clean, max 0.1 frames/s, max 1.1 % busy**; main-thread wasm memory 41 → 64 MB with the rebuilt bundles
  (the shaped-label cache, bounded at 16 MiB per bundle). puzzle3d orbit re-profiled: p95 up to 74 ms, the cost is the guest's UI republish during
  the gesture (retained patch intake + shell re-render), not the canvas — recorded as open (guest side). `nx show project` confirms the editor
  bundle's wasm target hashes the canvas crate's sources (the `--skip-nx-cache` precaution was not needed; `requests/f1.txt` corrected).
- 09:5x **state / blocker:** everything host-side is landed and measured. Open, blocked on the coordinator's publish-done announcement (rule 20):
  apply `patches/typing-coalescing/` (jack + vcs guest fixes, SDK typing-run helper + fixture, writer/jack laws), run the laws, wasip2-check
  both guests, then (after W2's consolidated restage) prove the jack query live with `f1-typing-run-live.mjs --plugin trinity --app
  "s.trinity.jack@1/*#editor" --window trinity-jack-editor`. Serve 6620 (pid 5419 → vite 5423) stays up for that; I stop it afterwards.
  Also open, guest/framework side (not started, frozen): the dag computing indicator is a frame-counted pulse (`DagHost::tick_computing_animation`)
  that only advances on demand paints — it should become an explicit bounded animation (the session answering "animating" while a node
  computes, the host holding a `beginContinuous` for exactly that time); puzzle3d orbit republishes the retained UI during the gesture.
