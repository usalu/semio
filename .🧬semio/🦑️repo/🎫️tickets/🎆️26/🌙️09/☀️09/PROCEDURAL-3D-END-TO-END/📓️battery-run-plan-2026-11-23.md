# 🔋 Battery run plan (2026-11-23)

Read-only audit. No source edited, no server started, no probe or battery run. Only this file was written. Verified by reading the files named below, by `--list` on both batteries, by `🐍️example-oracle-count.mjs` (offline, exit 0), and by `curl` / `nc` / `screen -ls` against the live ports.

Abbreviations: `TF` = ticket folder `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END`. `WGPU` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`. `VARIANT` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 0. Live state at audit time

| Port | State | Evidence |
|---|---|---|
| 6018 (React) | closed | `nc -z` refused. `🗑️generated/serve-native-6018.txt` ends with `ENOENT … .cache/b/p/m/browser/prototype/runtime/react/generation3d/activation/🔣️receipt.json` |
| 6118 (WGPU) | listening, served | screen `73167.g3dwgpu` detached. `/?plugin=generation3d` returns 200, `/renderer-modules/wgpu/semio-renderer-wgpu-component.js` returns 200 |

Staged plugin modules: `.cache/b/p/m/browser/prototype/modules/` holds only `🖨️raster`, `🧵️shard`, `🪞️vendor`. No `procedural` / `generation3d` module exists. `find` for `plugin-modules` and for activation receipts returns nothing in the repo.

## 1. Commands

### 1.1 Environment (both batteries)

```sh
TF="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
BUN="$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)"   # bun 1.4.2
export PATH="$(dirname "$BUN"):$PATH"   # both batteries spawn a bare `bun`; `command -v bun` is empty by default
cd "$TF"
```

Alternative runtime with the same bun layout: `/Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun` (exists; the readiness audit's path). Either works.

Browser library: `playwright` 1.64.1 at `/Users/ueli/Documents/semio/node_modules/playwright` (present; symlink into `.cache/b/d/m/j/307631934f0e2b89ee1f/node_modules`). 143 probe files `import { chromium } from "playwright"`. Chromium revision 1248 is installed at `~/Library/Caches/ms-playwright/chromium-1248` and `chromium_headless_shell-1248`, which matches playwright-core's `browsers.json`. Every browser probe launches with `--enable-unsafe-webgpu --ignore-gpu-blocklist --use-angle=metal` (macOS only).

Offline pre-check (no server, no browser, exit 0, 8 of 8 rows):

```sh
"$BUN" 🐍️example-oracle-count.mjs
```

### 1.2 Servers

React, port 6018 (currently blocked, see §4 B2):

```sh
screen -dmS semio-g3d-react "$TF/serve-generation3d-native.sh"
curl -s -o /dev/null -w '%{http_code}\n' 'http://127.0.0.1:6018/?plugin=generation3d'
```

WGPU, port 6118 (already running as screen `73167.g3dwgpu`; restart only if needed):

```sh
screen -S g3dwgpu -X quit 2>/dev/null; screen -dmS g3dwgpu "$TF/serve-generation3d-wgpu-native.sh"
curl -s -o /dev/null -w '%{http_code}\n' 'http://127.0.0.1:6118/?plugin=generation3d'
```

Logs: `TF/🗑️generated/serve-native-6018.txt` (React), `TF/🗑️generated/wgpu-native-serve.txt` (WGPU).

### 1.3 Batteries

```sh
cd "$TF" && "$BUN" 🐍️wgpu-battery.mjs --only=boot        # gate: must be green before the rest
cd "$TF" && "$BUN" 🐍️react-battery.mjs --only=boot       # gate
cd "$TF" && "$BUN" 🐍️react-battery.mjs                   # all 26 rows, about 8.3 h worst case
cd "$TF" && "$BUN" 🐍️wgpu-battery.mjs                    # all 16 rows, about 7.5 h worst case
```

Subsets: `--only=a,b` with the names in §3 (`--list` prints them). Sequential by design: one browser at a time, do not run both batteries concurrently against the same evidence root.

Defaults that match the servers: React `SEMIO_BATTERY_URL=http://127.0.0.1:6018/?plugin=generation3d`, WGPU `http://127.0.0.1:6118/?plugin=generation3d`. Roots: `🗑️generated/react-verify/scoreboard.json`, `🗑️generated/wgpu-verify/scoreboard.json` (override with `SEMIO_BATTERY_ROOT`).

Path length: the deepest battery output (for example `…/🗑️generated/wgpu-verify/world3d-viewer/sphere-cut-with-torus/console.txt`) is about 205 bytes, under the 256 limit. Measured 186 bytes for a React interact path.

## 2. Boot contract (what the serve must provide)

- Query: `plugin=generation3d` (required), `role=viewer` (viewer rows), `example=<slug>` (example rows). Parser (shared by React and WGPU): `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts:44-66`. Unknown `role`/`example` values fall back, they do not fail. `mode` is not used by any battery row.
- Routes: `/?plugin=generation3d` (HTML page, both ports), `/renderer-modules/wgpu/semio-renderer-wgpu-component.js` and `…_bg.wasm` (WGPU; served from `.cache/g3d-native/wasm-dev`, present), boot bundle `/🚀️boot.js/🟨️.js` (WGPU; page loads it, 200 path is the one the HTML references). `/🚀️boot.js` alone returns 404, which is expected.
- Example slugs (8, identical in both batteries and in `🐍️wgpu-example-matrix-probe.mjs:55`): `hexagonal-mushroom-column`, `rectangle-extrude-volume`, `rectangle-wire-preview`, `box-shell-preview`, `box-fillet-preview`, `sphere-cut-with-torus`, `sphere-box-fuse`, `face-sweep-extrude`. All 8 directories under `VARIANT/📚️examples/` exist with `🧪️tests/🧫️fixtures/🧩️example/🔣️.json`.

## 3. Battery rows and what they cover

### 3.1 React (`🐍️react-battery.mjs`, 26 rows, not 25 as the readiness audit says)

| Row | Probe | Examples / lanes covered |
|---|---|---|
| boot | console-dump-probe | ≥2 surfaces mount, starter document |
| journey | journey-probe | `edit:<label>` and `view:<label>` for every `EXPECTED_WIDGETS` key (`journey-probe.mjs:81`): Hexagonal Mushroom Column, Rectangle Extrude Volume, Sphere Cut With Torus, Box Fillet Preview, Sphere Box Fuse, Face Sweep Extrude, Rectangle Wire Preview, Box Shell Preview, No example |
| interact | interaction-matrix-probe | all 8 examples (from `Object.values(ORACLES)`) × payload, selection-reset, hover, select, inspector, orbit, fit |
| slider-live-preview | slider-live-preview-probe | 8 examples (env list, `react-battery.mjs:118`) × `graph` slider |
| generate-mode | generate-mode-probe | generate mode steps |
| flow-window | flow-window-probe (`default`) | node graph paints |
| flow-wire | flow-window-probe (`wire`) | wire publish, cut, redraw |
| flow-reorganize | flow-window-probe (`reorganize`) | mod+alt+L |
| cancel-preview | cancel-preview-probe | Sphere Cut With Torus (default pick) |
| keyboard-verbs | editor-verbs-keyboard-probe | chords on Sphere Cut With Torus: baseline, edit-target, edit-arms-history, undo-armed-history, redo-armed-history, generate-mode, add-generation, cancellable-evaluation-armed, cancel-preview-eval, cancel-affordance-discriminator |
| io-surface | io-surface-probe | export and import rows |
| export-encoding | export-encoding-probe | `dwg`, `stl` |
| status-parity | status-parity-probe | edit:sphere-cut, edit:no-example, edit:box-fillet, generate-mode, generate-added, back-to-edit, viewer-role, view:sphere-box-fuse, view:no-example |
| role-switch | role-switch-runtime-probe | boot, mid-chain, post-editor, post-viewer, generate (plus `mid-chain-immediate`, ungraded, see M9) |
| i18n-a11y | i18n-a11y-customization-probe | tags 2-edit-de, 4-generate-de, 5-viewer-de, 6-after-reload |
| customization-persistence | customization-persistence-probe | locale and appearance survive reload |
| gaps | react-gap-probe | probe-owned steps |
| preview-chrome | preview-chrome-probe | probe-owned steps |
| status-states | status-states-probe | probe-owned, en and de passes |
| menus | menu-dump-probe | probe-owned steps |
| viewer-actions | viewer-actions-probe | probe-owned steps |
| flow-scroll | flow-scroll-render-perf-probe | hexagonal-mushroom-column, sphere-cut-with-torus × four gestures |
| graph-keyboard | graph-keyboard-nav-probe | arrow chords and Enter on the node graph (default example); 9 graded chords |
| outline-selection | outline-selection-probe | outline steps |
| panel-i18n | panel-i18n-probe | every `Generation3dLabels` field reached |
| inspection-i18n | inspection-i18n-probe | inspection catalogue fields |

### 3.2 WGPU (`🐍️wgpu-battery.mjs`, 16 rows)

| Row | Probe | Examples / lanes covered |
|---|---|---|
| boot | wgpu-journey-probe (`SEMIO_PROBE_ONLY=boot`) | shell reaches a live window, dock plans, no surface fault |
| examples | wgpu-example-matrix-probe | 8 examples × lanes edit, viewer (16 rows, each with an oracle row) |
| no-example | wgpu-example-matrix-probe (`(none)`) | edit, viewer: starter solid, no `?example=` trace |
| generate-add | wgpu-add-generation-probe | hexagonal-mushroom-column, box-shell-preview |
| generation-roster | wgpu-generation-publication-probe | hexagonal-mushroom-column (retained row, roster, form, journey) |
| deferred-commit | wgpu-deferred-commit-probe | hexagonal-mushroom-column (rename, slider) |
| world3d-editor | wgpu-world3d-interaction-probe | all 8 examples, editor role; h1..h9 gestures plus camera framing |
| world3d-viewer | wgpu-world3d-interaction-probe (`role=viewer`) | all 8 examples, viewer role |
| spawn-job | wgpu-spawn-job-probe | hexagonal-mushroom-column, s1..s6 gestures |
| frame-loop | wgpu-frame-loop-probe | hexagonal-mushroom-column, 20 gestures and idle window |
| node-gestures | wgpu-node-gestures-probe | hexagonal-mushroom-column graph sweep |
| chrome | wgpu-input-deliverables-probe | box-fillet-preview, chords d1..d6a |
| catalogue | wgpu-catalogue-probe | app-static catalogue |
| status-a11y-i18n | wgpu-status-a11y-i18n-probe | sphere-cut-with-torus, accessibility mirror, German, status pill |
| io | wgpu-io-probe | hexagonal-mushroom-column, export STL, import picker |
| port-fit | port-fit-probe (`SEMIO_PROBE_TARGET=wgpu`) | hexagonal-mushroom-column, fit findings |

## 4. Mismatches (fix list, file:line)

Paths are relative to `TF` unless shown. `WGPU/` is the path above.

### B. Blockers (the serves do not render)

- **B1. WGPU serve passes an empty plugin catalog.** `TF/serve-generation3d-wgpu-native.ts:13` passes `catalog: { version: 1, targets: [], hosts: [], playgrounds: [] }`. The same empty literal is the default in `WGPU/🌐️server/🎚️config/🟦️.ts:62` and `:65`, and `nativeCatalog` falls back to empty at `:45`. The config injects this catalog as `#semio-plugin-catalog` (`WGPU/🌐️server/🟦️.ts:146`). The boot reads it at `WGPU/🚀️browser-boot/🤖️generated/🟨️.js:27968`. With `?plugin=generation3d`, `pluginVariant` is set, so `pluginRegistrySelection` is `"variant"` (`WGPU/🚀️browser-boot/🟦️.ts:25`). The plan is then empty and `assertWgpuPluginPlan` refuses at `WGPU/🧩️plugin-modules/🟦️.ts:87` with `plugin-registry-invalid: selected module dependency graph`. That is the exact error in `TF/📓️wgpu-native-serve-2026-11-23.md:5`. Fix: feed the generation3d catalog rows into `serve-generation3d-wgpu-native.ts:13` (and the `:62`/`:65` defaults), not the empty literal. Staging the module alone will not fix this. Code reading only, not re-run.
- **B2. React serve cannot start: no staged `procedural` module, no activation receipt.** `TF/serve-generation3d-native.ts:17` calls `createServer`, whose config load reads `.cache/b/p/m/browser/prototype/runtime/react/generation3d/activation/🔣️receipt.json`. The receipt is absent and `…/modules/🌀️procedural/` is absent (`TF/📓️react-native-serve-2026-11-23.md:7-8`). Log: `TF/🗑️generated/serve-native-6018.txt` (ENOENT). Fix: stage the component, then run `repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies` (per the note). Not runnable from this audit.
- **B3. Stale comment about the WGPU guest.** `TF/📜️serve-generation3d-wgpu.sh:24-25` says the guest is "already staged under `🧑‍💻dev/🔌️plugin-modules`". That directory does not exist anywhere in the repo. Fix the comment or stage the module.

### T. Battery-side mismatches

- **T1. Catalogue row grades stale evidence and ignores the lane root.** `TF/🐍️wgpu-battery.mjs:331` sets `SEMIO_PROBE_OUT: "verify"` and `:332` sets `from: join(GEN, "wgpu-catalogue", "verify")`. The probe writes `🗑️generated/wgpu-catalogue/<OUT>` (`TF/🐍️wgpu-catalogue-probe.mjs:14`). The battery never clears `from`; `rmSync` at `:518` clears only `dir`. If the probe dies before writing `report.json`, the previous run's `report.json` and `console.txt` are copied at `:529` and graded green. `SEMIO_BATTERY_ROOT` is also not honoured for this row. Fix: `SEMIO_PROBE_OUT: "../${ROOT_NAME}/catalogue"` (same pattern as `outUp` in `react-battery.mjs:33`), drop `from` at `:332`, and drop the copy at `:529`.
- **T2. `examples` and `no-example` never read their consoles; page errors are hard-coded to zero.** `TF/🐍️wgpu-battery.mjs:112` and `:142` return `pageerrors: 0, faults: []`. The per-row console is at `🗑️generated/wgpu-verify/examples/<example>/<lane>/console.txt` (`TF/🐍️wgpu-example-matrix-probe.mjs:318`). Fix: read it and count `pageerror` and `FAULT_RE` lines, as the React battery does.
- **T3. WGPU has no central page-error or fault gate.** React appends "no page errors" and "no shell faults" to every probe (`TF/🐍️react-battery.mjs:529-533`). The WGPU verdict loop (`TF/🐍️wgpu-battery.mjs:533-534`) decides only on the probe's own steps, so a probe can be green while its log carries `pageerror`. Fix: mirror the React gate at `wgpu-battery.mjs:533`.
- **T4. Four graph-keyboard rows are never graded.** `TF/🐍️react-battery.mjs:422-432` maps only the nine traversal chords. `escape-clear`, `delete-selection`, `undo` and `baseline` (`TF/🐍️graph-keyboard-nav-probe.mjs:126-140`) get `ok: undefined`, so `decided` (`react-battery.mjs:534`) drops them. Fix: add expectations for `escape-clear` (`Escape`), `delete-selection` (`Delete`) and `undo` (`Control+z`) to the `want` map.
- **T5. Chrome row reads a step the probe never emits.** `TF/🐍️wgpu-battery.mjs:320` tests `s.d5_cancel ?? s.cancel`. `TF/🐍️wgpu-input-deliverables-probe.mjs` emits `d5_retained_row` (line 293) and no cancel step. The check reduces to a console regex. Fix: read `d5_retained_row` or drop the step.
- **T6. Matrix writes its evidence only at the end; the row budget is at the edge.** `TF/🐍️wgpu-battery.mjs:80` sets `minutes: 60` for `examples`. Worst case is 16 rows × `SEMIO_PROBE_BUDGET` 200 s, about 53 min, before browser start. `TF/🐍️wgpu-example-matrix-probe.mjs:324` writes `results.json` only after the loop, so a timeout leaves no evidence. Fix: write `results.json` after each row, or raise `minutes`.
- **T7. Readiness audit row count is stale.** `TF/📓️battery-readiness-audit.md` says "all 25 probes" and "25 probes". `bun 🐍️react-battery.mjs --list` prints 26 names. The audit's time budget (496 min) matches the 26-row array, so only the count text is wrong.
- **T8. Role-switch row `mid-chain-immediate` is graded only on "windows mounted".** The probe records `mid-chain-immediate` (`TF/🐍️role-switch-runtime-probe.mjs:86`). `TF/🐍️react-battery.mjs:298` has no `expected` entry for it, so it falls back to `windows.length > 0`. Low. Fix: add the entry or drop the record.

### P. Probe defaults (only matter when a probe runs standalone)

- `TF/🐍️interaction-matrix-probe.mjs:34` and `TF/🐍️flow-scroll-render-perf-probe.mjs:31` default to port 6022. `TF/🐍️outline-selection-probe.mjs:14` defaults to 6023. The battery overrides all three (`react-battery.mjs:34`, `:112`, `:445`), so this affects standalone runs only.

### Verified as matching (no action)

- Battery-to-probe env names: every `SEMIO_PROBE_*` the batteries pass is read by the probe (checked for all 26 React and 16 WGPU rows).
- Output roots: React rows use `outUp` and `out` consistently with the probes' `join(import.meta.dir, "🗑️generated", …)` and fixed parents. `panel-i18n` and `inspection-i18n` pass `../react-verify/<x>`, which resolves correctly.
- Verdict file names: every `results.json` / `verdicts.json` / `verdict.json` / `steps.json` / `summary.json` / `scroll.json` / `wire-result.json` / `reorganize-result.json` / `status-frames.json` / `result.json` / `findings.json` / `report.json` / `german.json` the battery reads is written by its probe at the same directory.
- Oracle path: `example-fixtures.mjs` resolves `EXAMPLE_FIXTURE_PATH` from the examples' own TS lane. `example-oracle-count.mjs` returns 8 of 8 with `matrixFixture: true`. The readiness audit's oracle-path blocker (its §3 row 4 and §6 items 1-4) is resolved offline.
- Example slugs: the battery lists match the eight example directories, and the matrix probe's `ALL_EXAMPLES` (`wgpu-example-matrix-probe.mjs:55`).
- `plugin`, `role`, `example` query keys: parsed by the shared boot-query module (see §2).
- `i18n-a11y` tags 2, 4, 5 and 6 all exist in the probe (`i18n-a11y-customization-probe.mjs:142, 167, 181, 201`).

## 5. Predicted reds

Predictions come from the audit, the open-journey note, and code reading. None of them were measured in this audit.

### 5.1 Now (serve-level, every row)

- React, 26 rows: all red. 6018 is closed, and the serve dies on the missing activation receipt (B2). Reason: no listener, `run.status` non-zero, no evidence.
- WGPU, 16 rows: boot is red, with reason `plugin-registry-invalid: selected module dependency graph` (B1, `TF/📓️wgpu-native-serve-2026-11-23.md:5`). Rows that need a live window cascade to red: examples (16 rows), no-example (2), generate-add (2), generation-roster (5), deferred-commit (3), world3d-editor, world3d-viewer, spawn-job, frame-loop, node-gestures, chrome, io, status-a11y-i18n, port-fit. The catalogue row depends on the catalogue route, not on a booted shell, but T1 means a stale green is possible.

### 5.2 After the serves boot (from the notes)

- React `interact`: Rectangle Wire Preview `select` and `inspector`, and Hexagonal Mushroom Column `inspector` (open-journey P0 3; `TF/📓️open-journey-items-2026-11-22.md` item 3). Fixes R4 and R5 are in source but not re-run.
- React `keyboard-verbs`: `undo-armed-history` red unless the re-arm fix (R2) is live. Rust law not run. `add-generation` unverified (R3).
- WGPU `chrome`: "a chord replans the dock" and the control-row click, which the open-journey note says misses (P0 5).
- WGPU `status-a11y-i18n`: ARIA, German and status pill (P0 5; R9 browser re-run pending).
- WGPU `io`: "the import opens a real file picker" and "the picked file replaces the graph" (P1 wgpu import/export).
- WGPU `frame-loop`: "the frame wire never quarantines" (P0 5; R12 not executed).
- WGPU `world3d-viewer`: at risk for the gumball and authority hops (R7 law not run).
- WGPU `node-gestures`: "Fit graph publishes a camera" predicted green (R10 emitter restored, audit §6.5). "The dense sweep never faults" at risk.
- WGPU `port-fit`: wgpu fit finding predicted green (audit §6.6), not re-run.
- WGPU `examples` and `world3d-*`: the oracle rows can now grade (fixed offline). Any row where the published payload differs from the committed box or mesh roles is red. No per-example prediction exists in the notes.

Not predicted (no evidence either way): React boot, journey, slider-live-preview, generate-mode, flow-window, flow-wire, flow-reorganize, cancel-preview, io-surface, export-encoding, status-parity, role-switch, i18n-a11y, customization-persistence, gaps, preview-chrome, status-states, menus, viewer-actions, flow-scroll, outline-selection, panel-i18n, inspection-i18n; WGPU deferred-commit, spawn-job, generation-roster, generate-add, no-example, catalogue (apart from T1).

## 6. 2026-11-23 harness fixes

Harness-only edits, no server and no Rust build touched. Checked with `node --check` and `bun build --no-bundle` on all three files (green), `--list` (React 26 rows, WGPU 16 rows) and `bun 🐍️example-oracle-count.mjs` (8 of 8 oracles, exit 0). No battery was run.

- **T1** `🐍️wgpu-battery.mjs`: the catalogue row now sets `SEMIO_PROBE_OUT` to `../<SEMIO_BATTERY_ROOT>/catalogue` (the React `outUp` pattern), so the lane root is honoured. The `from` field and the `cp -R` of a foreign `report.json` are gone; the row dir is cleared before each run, so a dead probe leaves no report and grades red. The unused `existsSync` import is dropped.
- **T2** `examples` and `no-example` read every `<example>/<lane>/console.txt` (`matrixConsole`) and report real `pageerrors` and `faults`. Both rows gained a completeness step (`examples`: `complete === true` and 16 rows; `no-example`: 2 rows), so a partial or missing `results.json` cannot grade green on the rows it happens to hold.
- **T3** central gate in the verdict loop, mirroring React: appended steps "no page errors" and "no shell faults", plus "the probe finished inside its budget" (a SIGTERM/ETIMEDOUT kill is red). Unlike React the exit code is not required to be 0 (wgpu probes were never held to it); say so if you want that.
- **T4** `🐍️react-battery.mjs` graph-keyboard: `escape-clear` must invoke `clearSelection`, `delete-selection` `deleteSelection`, `undo` `undo`, per `⌨️keyboard-reachability.json` (`mod+z` undo, `delete` deleteSelection) and the interaction domain's `escape` mapping. `baseline` presses nothing, so it is green only when nothing was invoked. Every row is now decided. These three verbs were derived from the spec and not measured: the first run may show them red for a real reason (for example `clearSelection` being minted by the framework under another id).
- **T5** the probe never presses a cancel control: `d5` is a retained-row click and the cancel control is only a console declaration, and cancellation semantics are graded by `cancel-preview` and `spawn-job`. So the battery reads the field the probe does emit. The dead `d5_cancel ?? cancel` read is dropped; the declaration step stays console-only, and a new step grades `d5_retained_row` (a row was found, no pointer failure, a render or command followed).
- **T6** `🐍️wgpu-example-matrix-probe.mjs` writes `results.json` after every row (`complete: false`, flipped to `true` at the end) and the per-example budget is wall clock from the row's start (`at() < SEMIO_PROBE_BUDGET`, the `MIN_SECONDS` floor likewise) instead of a loop-turn count, which `measure()` stretched far past 200 s. The battery's `examples` timeout is now 16 rows × (budget + 30 s) rounded up to whole minutes + 5 = 69 min.
- **T7** `📓️battery-readiness-audit.md`: "25 probes" corrected to 26 (two places).
- **T8** `role-switch`: `mid-chain-immediate` is graded as windows mounted and exactly one of editor/viewer pressed (never a torn role group), since the snapshot is taken 150 ms after the chord, before the switch is expected to land.
