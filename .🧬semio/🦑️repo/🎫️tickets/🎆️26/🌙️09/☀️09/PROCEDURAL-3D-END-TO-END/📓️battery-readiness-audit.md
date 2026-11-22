# 🔋 Battery readiness audit (2026-11-22)

Read-only audit of `🐍️react-battery.mjs` and `🐍️wgpu-battery.mjs` against the open-journey notes and the current source. No build, no server, no repo edits. Only this file was written.

Status at audit time: ports 6018 and 6118 both closed (`nc -z`). `bun` is NOT installed (`command -v bun` empty). Node v24.15.0 is present. `/System/Volumes/Data` shows 31 GiB free (the ENOSPC of 2026-09-15 may no longer hold; unverified). `.cache/b/r/` holds `fixed.dmg` (103 GB, modified 2026-11-22 15:21) and `image.json`; the `m` entry named in the open-journey note was not seen.

## 1. How to run

Prerequisites (both batteries):
- **bun is required.** Both batteries and every probe they spawn run under `bun`: they use `import.meta.dir` (`🐍️react-battery.mjs` and `🐍️wgpu-battery.mjs`, the probes too) and the battery spawns `bun <probe>`. Node cannot run them. `bun` is missing on this Mac.
- **Playwright and Chromium.** Probes `import { chromium } from "playwright"`. `node_modules/playwright` exists at `/Users/ueli/Documents/semio/node_modules/playwright`, and `~/Library/Caches/ms-playwright` holds `chromium-1248` and `chromium_headless_shell-1248`. The probes launch headless Chromium with `--enable-unsafe-webgpu --ignore-gpu-blocklist --use-angle=metal` (macOS-only flags; not cross-platform).
- **Servers.** React on 6018: `repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype` (from the open-journey note; the target name was not found by a grep of repo JSON/TS/TOML, so unverified). WGPU on 6118: `repo run @semio-tech/framework-os-dev:dev-generation3d-wgpu-prototype`. React serving needs an activation receipt and staged wasm (react-interact note). Start each server before its battery.
- **URL and plugin.** React: `http://127.0.0.1:6018/?plugin=generation3d`. WGPU: `http://127.0.0.1:6118/?plugin=generation3d`. Override with `SEMIO_BATTERY_URL`.

Commands (run from the ticket folder, sequentially, one browser at a time):
```
cd "<ticket>" && bun 🐍️react-battery.mjs                       # all 25 probes
cd "<ticket>" && bun 🐍️react-battery.mjs --only=interact,keyboard-verbs
cd "<ticket>" && bun 🐍️react-battery.mjs --list
cd "<ticket>" && bun 🐍️wgpu-battery.mjs                        # all 16 probes
cd "<ticket>" && bun 🐍️wgpu-battery.mjs --only=examples,port-fit
```
Env: `SEMIO_BATTERY_ROOT` (default `react-verify` / `wgpu-verify`, the folder under `🗑️generated/`). Scoreboards: `🗑️generated/react-verify/scoreboard.json` and `🗑️generated/wgpu-verify/scoreboard.json`.

Time budget (sum of per-probe `minutes`): React about 496 min (about 8.3 h worst case); WGPU about 452 min (about 7.5 h worst case).

Every probe's pass or fail is decided from its own published evidence in its folder (`results.json`, `verdicts.json`, `console.txt`, and so on). A missing or unreadable file makes the step red, not skipped.

## 2. Checks each battery asserts

Common to every React probe: "no page errors" (count of `pageerror` lines) and "no shell faults" (FAULT_RE match). Common to every WGPU probe: the same fault and page-error gates are NOT appended centrally (only the probe's own steps, plus `pageerrors`/`faults` in the key).

React (`🐍️react-battery.mjs`, 25 probes):
- boot: ≥2 surfaces mount.
- journey: one row per journey label, `converged`.
- interact: per example × {payload, selection-reset, hover, select, inspector, orbit, fit}.
- slider-live-preview: per example × kind (graph): no dropped actions, one history entry per press, coalesced dispatch, settled geometry.
- generate-mode: probe steps.
- flow-window: node graph paints (`dag draw` > 0).
- flow-wire: graph publishes wires; cut removes; redraw restores.
- flow-reorganize: mod+alt+L moves widgets.
- cancel-preview: a cancellable frame is published; cancel affordance exists.
- keyboard-verbs: one row per chord, decided by the probe's own oracle (includes `undo-armed-history`, `generate-mode`, `add-generation`).
- io-surface: probe `ok` per row.
- export-encoding: `export dwg`, `export stl` (bytes > 0, no error).
- status-parity: preview hosts carry a status.
- role-switch: boot, mid-chain, post-editor, post-viewer, generate; no `no actor for instance`.
- i18n-a11y: tags 2-edit-de, 4-generate-de, 5-viewer-de, 6-after-reload (German labels, named/focusable shells, `lang=de`).
- customization-persistence: locale survives reload; appearance survives reload.
- gaps, preview-chrome, status-states, menus, viewer-actions, outline-selection: probe-owned steps.
- panel-i18n, inspection-i18n: one `de:<field>` row per Generation3d label.
- flow-scroll: per gesture: paint median ≤16 ms, p95 ≤50 ms, 0 guest hops during, 1 camera publication at settle.
- graph-keyboard: 9 traversal chords reach their verb; Inspection panel mounted; selection moves.

WGPU (`🐍️wgpu-battery.mjs`, 16 probes):
- boot: shell reaches a live window; dock plans windows; no surface fault.
- examples: per lane (edit, viewer) × 8 examples: `verdict === pass`, and a second row per example that the published payload matches the committed `example-geometry` fixture (`meshRoles`, bounding box).
- no-example: the starter document is a solid; no `?example=` trace.
- generate-add: Add Generation → mesh for hexagonal-mushroom-column and box-shell-preview.
- generation-roster: addGeneration dispatched; roster grows; Form binds; generate preview carries a mesh; second add lands.
- deferred-commit: rename commits and guest runs it; Form slider reaches guest; preview pixels move.
- world3d-editor, world3d-viewer: per example: publishes committed meshes; boot camera frames the bounding box; h1 hover centre reports the example's target (or none for wire-only); h2 empty corner none; h3 click selects (guest, published payload and shell inspector all change); h4 shift-add; h5 empty click clears; h6 marquee takes the body; h7 wheel, h8 alt-right-drag orbit, h9 shift-right pan each publish setCamera; camera still frames body; no authority fault, panic, or unmapped effect.
- spawn-job: each step pumps a spawn-job; no dropped `spawn-job`; no panic.
- frame-loop: ≥20 gestures; zero wire quarantines; wire at most one frame behind; actions still published.
- node-gestures: census > 0; interactionSelect; nodeGraphEdit; dag edge connected and removed; nodeGraphViewport; `shell node-graph fit`; no ItemCredits, BoundedActionFault, graph move fault or panic.
- chrome: navbar control ids; example picker reachable; mode and role buttons reachable; a chord replans the dock; world3d cancel control declared.
- catalogue: app-static catalogue parses; no UiFixedMap refusal; catalogue panel renders.
- status-a11y-i18n: probe steps (accessibility mirror, German via palette and boot, `status:pill-while-computing`).
- io: mod+shift+e reaches guest; mod+o reaches guest; no dispatch failure; export hands real STL bytes; import opens a file picker; picked file replaces graph (3 nodes).
- port-fit: probe findings (Fit graph publishes the computed camera).

## 3. Reds recorded in the notes: fix present or missing (evidence)

Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` unless shown otherwise; generation3d root is `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

| # | Red (note) | Current source | Verdict |
|---|---|---|---|
| R1 | Rust image ENOSPC blocks activation (open-journey P0 1) | Not a source defect. df shows 31 GiB free; `.cache/b/r/fixed.dmg` present. | Unverified; may be stale |
| R2 | Undo does not re-arm preview (open-journey P0 2; undo note §1) | Present. `PreviewEvalDocumentStamp` at `generation3d/…/🧵️preview-eval/⏯️tool-run/🦀️.rs:318`, `preview_eval_run_effects` at `:383`. Viewer feeds `content_revision: doc.source_version().canonical_revision` at `…/👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs:121`; editor at `…/✏️editor/🪟️artifact-editor-impl/🦀️.rs:588`. The digest (`applied_document_edits_digest`) is gone (grep empty). Law at `…/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs:281`. | Fixed in source. Rust law NOT RUN; runtime row `undo-armed-history` (`editor-verbs-keyboard-probe.mjs:321`) unconfirmed |
| R3 | Add Generation `mod+shift+g` unverified (09-13 §5.2) | Present. `🏛️ShellHost/⌨️window-scope/🟦️.ts:38` `dockSeedActiveWindowIdV1`, `:77` `resolveKeybindingTargetWindowV1`; used by `🏛️ShellHost/🎚️shell-model/🟦️.tsx:152`. | Fixed in source; runtime row `add-generation` (probe `:345`) unconfirmed |
| R4 | React interact: Rectangle Wire Preview `select` | Present. `🌐️World3dHost/🧱️instance-layers/🧊️instance-node/🟦️.tsx:166` `isCurveOnly`; `:269` and `:280` `world3dLinePick(…, isCurveOnly && meshReachable)` on hover and click, same target. Curve-only test per note, not re-run. | Fixed in source (not reproduced); the `interact` row is still red for another reason (§4) |
| R5 | React interact: Hexagonal Mushroom inspector | Probe-only change: `🐍️interaction-matrix-probe.mjs:123` `sweepForHover` returns unrounded point; `:147` `inspectionRows` polls the panel. No product change. | Probe hardened; runtime unconfirmed |
| R6 | WGPU wheel zoom in edit (wgpu note row 1) | Cause removed. No `AppWheel`. Per-event `handle_pointer_wheel` at `…/🎮️interaction-state/🦀️.rs:304` → `apply_scene_wheel` at `:323`. | Fixed in source; browser re-run pending |
| R7 | WGPU viewer `translateSelection` (row 2) | Present. `♾️infinite/🌍️world/🧩️components/🦀️.rs:584` `world3d_offers_transform_gumball`; host gate `♾️infinite/🌍️world/🎛️host/🦀️.rs:8`; sync `🐚️Shell/…/⚓️dock/⚙️settings-and-dock-persistence/🦀️.rs:24`. | Fixed in source; law not run |
| R8 | WGPU picker, modes, roles (row 3) | Present. `🐚️Shell/…/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs:475` `shell_example_control`, `:485` `shell_mode_controls`, `:505` `shell_role_controls`. | Fixed in source |
| R9 | WGPU status pill, German, ARIA (row 4) | Present. `…/chrome-paint-and-navbar/🦀️.rs:521` `world3d_status_pill_for`; `…/🧭️navbar-footer-and-overlays/🦀️.rs:392` computing trace; `:601` `shell.world3d.cancel`. | Fixed in source; browser re-run pending |
| R10 | WGPU Fit camera (row 5) | `framework_fit_node_graph` logs `[DEBUG] shell node-graph fit {"x":N,"y":N,"zoom":N}` on both arms. The unchanged viewport (`node_graph_fit_action` returns `None`) reads `node_graph_current_camera` and logs that camera, so `port-fit-probe.mjs` can `JSON.parse` from `{` and the gesture/battery counters that only require the substring still match. | Emitter restored in source; battery not re-run (no `repo cargo`, no server) |
| R11 | React Fit graph (node-graph note) | `🕸️NodeGraph/🎯️fit-graph/🟦️.ts:8,19`; input funnel `🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🟦️.ts:94` → `🔲️fit-graph/🟦️.ts:27`; registered by `🕸️NodeGraph/🔌️flow-session-attach/🟦️.ts:83-86`. | Present; runtime unverified. The node-graph note's Rust canvas and manifest tests passed; the wgpu test and TS tests did not run |
| R12 | WGPU frame-loop quarantine, chrome row click misses (open-journey P0 5) | Owner-based admission is recorded in `wgpu-frame-loop-after-selection-2026-09-13.md` §4. The time-based fatal lane and capacity refusals are in `wgpu-frame-loop-quarantine-audit.md`. The pre-step `frame_maintenance_terminal_fault(..., deadline_exceeded)` lane is retired in `wgpu-frame-loop-fix.md`: a passed deadline while the maintenance owner is live sets `shell_maintenance = false` and does not publish a surface fault. A missing deadline is not treated as expired. | Recorded in source; this row was not executed (no `repo cargo`, no server on 6118) |

## 4. Why rows are red regardless of the fixes (example enumeration)

Example list sources:
- **Source of truth.** Eight directories under `✏️s/…/📚️examples/` (for example `🍄️hexagonal-mushroom-column`, `🪢️rectangle-wire-preview`). Each carries `🧪️tests/🧫️fixtures/🧩️example/🔣️.json` with schema `s.procedural.generation3d.example-geometry/v1`, and each fixture's `example` field matches the slug. Verified by a scratchpad script: 8 of 8 OK.
- **WGPU battery.** Hard-coded `EXAMPLES` constant, `🐍️wgpu-battery.mjs:53`, with the same 8 slugs. Used by `examples` (line 85), `world3d-editor`, `world3d-viewer`, `spawn-job` (single example).
- **React battery.** `interact` gets its list from `🐍️interaction-matrix-probe.mjs:38`, `Object.values(ORACLES)`. `slider-live-preview` takes an explicit env list (`🐍️react-battery.mjs:118`).
- **Oracle loader (BUG).** `🐍️example-oracle.mjs:44` hard-codes `EXAMPLES_DIR` as an absolute `/Users/ueli/Documents/semio/...` path. Line 56 reads `<dir>/🧫️fixtures/🧩️example/🔣️.json`, but the fixture lives at `<dir>/🧪️tests/🧫️fixtures/🧩️example/🔣️.json` (the native twin uses this path: `…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts:27`). Result: `ORACLES` is empty. I confirmed `node -e import(...example-oracle.mjs)` gives `oracles: 0 []`.
- **Same wrong path** in `🐍️wgpu-example-matrix-probe.mjs` (`fixtureFor`, around line 72) and `🐍️wgpu-world3d-interaction-probe.mjs` (`fixtureFor`, around line 58). Both return `null` for all 8 examples.

Consequences:
- React `interact`: 0 examples enumerated, so `steps` is `[]` and the battery marks it red (`decided.length > 0` fails). Every interact row is red until the path is fixed.
- React `slider-live-preview`: `loadExampleOracles()` returns `{}` (`🐍️slider-live-preview-probe.mjs:476` `exampleOracles[example] ?? null`), so oracle-dependent grading degrades. Unverified beyond that.
- WGPU `examples`: every `publishes the committed meshes and box` row is red with "no fixture found for this example" (8 × 2 lanes = 16 red).
- WGPU `world3d-editor` / `world3d-viewer`: with `oracle.fixture` null, `targets` is `[]`, so the probe grades every example as wire-only. Solid bodies then fail `h1` (hover should be empty), `h3` (click should select nothing) and `h4`-`h6`. "publishes exactly its committed meshes" and "boot camera frames the committed bounding box" are red. So most world3d rows are red by construction for solid examples.

Also: the oracle header (`🐍️example-oracle.mjs`, lines 1-8) documents the wrong path, so fixing the code alone won't fix the docs.

## 5. Portability note (CLAUDE.md zero-touch rule)

- Absolute `/Users/ueli/...` path in `🐍️example-oracle.mjs:44`.
- macOS-only Chromium flags (`--use-angle=metal`) in every browser probe.
- `import.meta.dir` (Bun-only) in the batteries and probes.

## 6. Likely still red after this audit

1. React `interact`: all rows (0 examples; oracle path bug).
2. React `slider-live-preview`: oracle-dependent rows (oracle path bug).
3. WGPU `examples`: 16 oracle rows (oracle path bug).
4. WGPU `world3d-editor` and `world3d-viewer`: committed-mesh and boot-camera rows for every example; hover/select rows for solid examples (oracle path bug).
5. WGPU `node-gestures`: "Fit graph publishes a camera" — the `[DEBUG] shell node-graph fit` emitter is back, including the already-fitted arm. Not re-run.
6. WGPU `port-fit`: the same line is now `{x,y,zoom}` JSON on both arms, which is what `port-fit-probe.mjs` parses. Not re-run.
7. WGPU `frame-loop`: quarantine cause and the deadline terminal fault are recorded (`wgpu-frame-loop-after-selection-2026-09-13.md` §4, `wgpu-frame-loop-quarantine-audit.md`, `wgpu-frame-loop-fix.md`). The row itself was not executed.
8. Any row, if `bun` is absent: all of them (no runner).

## 2026-11-22 update: oracle fixture-path fix and runnable commands

**Fixed (the "oracle path bug", §3 row 4).** `🐍️example-oracle.mjs`, `🐍️wgpu-example-matrix-probe.mjs` and `🐍️wgpu-world3d-interaction-probe.mjs` read `<example>/🧫️fixtures/…`; the real location is `<example>/🧪️tests/🧫️fixtures/🧩️example/🔣️.json`, so every oracle was empty and every grade vacuous or red. Now one shared resolver, `🐍️example-fixtures.mjs`, serves all three. It does not restate the path: it imports `EXAMPLE_FIXTURE_PATH` from the examples' own TypeScript lane (`📚️examples/🧪️tests/🧩️geometry/🟦️.ts`, which the Rust lane mirrors with `include_str!`) and derives the examples root from its own location (the old absolute `/Users/ueli/...` in `EXAMPLES_DIR` is gone, so it is cross-platform).

**Same mistake elsewhere.** Checked every probe of both batteries. Only `🐍️wgpu-node-gestures-probe.mjs` had it: `⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/…` is really `⚙️EngineCanvas/🧪️tests/🧫️fixtures/🕸️wgpu-node-graph/…`; fixed. No other battery probe reads a fixture file (the remaining `🧫️fixtures` mentions are comments).

**Offline proof** (`🐍️example-oracle-count.mjs`, no server or browser). Oracle count is 8 of 8 examples (previously 0); each also resolves through the matrix/interaction resolver:

| example | meshes | minTriangles | minEdgeSegments |
|---|---|---|---|
| box-shell-preview | 1 | 24 | 24 |
| face-sweep-extrude | 1 | 12 | 12 |
| hexagonal-mushroom-column | 3 | 20 | 18 |
| sphere-cut-with-torus | 1 | 1114 | 45 |
| sphere-box-fuse | 1 | 398 | 25 |
| rectangle-wire-preview | 1 | 0 | 4 |
| rectangle-extrude-volume | 1 | 12 | 12 |
| box-fillet-preview | 1 | 92 | 72 |

**Run both batteries with the repo-provided bun** (the batteries spawn `bun` from PATH, so put the repo bun first; macOS/Linux shell):
```
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
BUN_DIR="$(dirname "$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)")"
cd "$T" && PATH="$BUN_DIR:$PATH" bun 🐍️example-oracle-count.mjs      # offline oracle proof, expect 8 rows
cd "$T" && PATH="$BUN_DIR:$PATH" bun 🐍️react-battery.mjs              # needs the React server on 6018
cd "$T" && PATH="$BUN_DIR:$PATH" bun 🐍️wgpu-battery.mjs               # needs the WGPU server on 6118
```
Equivalent via the tool-neutral interface: `repo tools exec --light bun -- <script>` for a single script (not used for the batteries, which spawn the bare `bun` name). Servers still must be started first (§1).
