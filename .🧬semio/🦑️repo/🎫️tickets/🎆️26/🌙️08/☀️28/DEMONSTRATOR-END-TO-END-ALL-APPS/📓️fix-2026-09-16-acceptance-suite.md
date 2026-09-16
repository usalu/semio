# 🎭️ Demonstrator acceptance suite — 8 failures, root causes + fixes (2026-09-16)

Ticket: `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`.
Suite: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`, run through
`bun nx run @semio-tech/mit-bestand-demonstrator:test-e2e`.
Baseline: `🗑️generated/test-e2e-2026-09-16-run1.txt` — 1 passed / 8 failed.

All paths below are relative to `/Users/ueli/Documents/semio`.

## 0. Method

Every root cause below was reproduced and measured against the live `:6029` dev serve with read-only
Playwright probes (kept in the ticket folder) before anything was changed, so no fix is a guess:

- `🔍️acceptance-dom-probe.ts` — dumps a pane's window containers, the surface host classes they carry
  and every `data-*-json` on them. Output: `🗑️generated/probe-{aussuchen,generator}-dom.txt`.
- `🔍️aussuchen-preview-probe.ts` — clicks a stock row and re-reads the preview window.
  Output: `🗑️generated/probe-aussuchen-preview.txt`.
- Canvas screenshots: `🗑️generated/probe-verfolgen-{swiftshader,metal}.png`,
  `🗑️generated/probe-aussuchen-grid-canvas.png`.

Fast iteration then ran the suite straight against `:6029`
(`PLAYWRIGHT_BASE_URL=http://127.0.0.1:6029 bunx playwright test --config ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts`)
— logs `🗑️generated/spec-live-6029-run{1,2}.txt`, `🗑️generated/spec-live-6029-verfolgen.txt` — before the
full Nx target.

## 1. Failure-by-failure

### 1.1 Drift guard — `ENOENT … 🧪️tests/🎭️acceptance/🪧️brand.ts` (fixed by the coordinator)

`brandPaneIds()` read `🪧️brand.ts` next to the spec instead of two levels up. Already fixed before this
pass; it passes in every run below.

### 1.2 `demonstrator overview: pane cards …` — `click: Timeout 5000ms exceeded … waiting for element to be visible, enabled and stable`

**Root cause.** Not the card chrome — the card contract is fully satisfied (probe: 6 cards, each with
exactly 1 `[data-window-silhouette]`, 1 title-chip `svg`, 0 drag handles). The intro overlay's
`ui.introduction.skip` anchors itself to whatever surface the current tour step highlights
(`registerIntroductionSurfaceResolver`), so while a COLD landing page is still booting its six shells the
box re-anchors and Playwright's actionability *stable* gate never settles inside 5 s. Measured: on a warm
`:6029` the button's box is byte-identical for 300 consecutive frames and the click lands in ~230 ms; on
run 1's cold server the same click never became stable. It is a boot-time layout race, not an app defect.

**Fix.** `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts:117` — new `dismissIntroduction(page, skip)`:
one generous (20 s) click, then `Escape` — the keyboard parity route this very control declares
(`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:164`,
`"ui.introduction.skip": "escape"`), which needs no actionability at all — and finally
`expect(skip).toHaveCount(0)`, so a genuinely undismissable overlay still fails loudly. Both the overview
test and `dismissIntroductionIfPresent` now go through it (the pane path previously swallowed failures
with a bare `.catch(() => {})`, which was weaker than this).

### 1.3 generator — `framework.window.proceduralMain`: `widgets: []`

**Root cause.** Stale attribute name in the spec. Commit `8773331d23` ("Align plugin schemas, mutations,
panels, and tests to snapshot-fixture-asset terminology") renamed the node-graph scene field
`fixtureJson` → `hostSnapshotJson`, and with it the DOM attribute
`data-fixture-json` → `data-host-snapshot-json`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1322`). The spec still
read the old name, so it always saw `null` → 0 widgets. Probe evidence: `proceduralMain` carries
`data-status-json=193B`, `data-host-snapshot-json=1867B`, `data-selection-json=73B` and **no**
`data-fixture-json`; the snapshot parses to `{schema, camera, widgets(7), synapses, layout}` with widgets
`inputSlider height`, `inputSlider radius`, … — i.e. the real Polygon → Vector → Extrude graph.

**Fix.** `🎭️acceptance/🟦️.ts:258` `nodeGraphWidgetCount` reads `data-host-snapshot-json`; the flow engine
is a wasm canvas with no per-node DOM, so that attribute is the only DOM-level evidence of graph content
and the assertion is unchanged in strength (`widgets.length > 0`).

### 1.4 aussuchen — `framework.window.sourcingPreview` has no `.semio-world-3d-host`/`-empty`

**Root cause.** Not an id/alias drift: the window attaches under exactly the expected id. Its window kind
declares `SurfaceKind::World3d`
(`✏️s/🔌️plugins/🪵️sourcing/…/🪟️windows/👁️preview/🦀️.rs:22`), but with nothing selected its `render`
deliberately returns `built_text_node(labels.no_selection)` (same file, line 46) — a plain **text** body,
so no world-3d host element exists to grade. Probe: the window's `innerText` is `Keine Auswahl`, host
classes `[]`, while its three siblings are healthy (`sourcingPool`/`sourcingCurated` →
`.semio-table-host`, `sourcingGrid` → `.semio-world-3d-host` with `data-meshes-json=804B`).

The `📓️app-aussuchen.md §5` gap the spec cited is **closed**: `✏️s/🔌️plugins/🪵️sourcing/…/✏️editor/🦀️.rs:1091`
now feeds `preview::render(doc.snapshot, &selected_ids, …)` from
`interaction.selection(SOURCING_ROWS_DOMAIN)`. Probe proof (`probe-aussuchen-preview.txt`):

```
before      {"text":"Keine Auswahl","host":false,"meshes":0,"instances":0}
pool rows: 10
after-click {"text":"","host":true,"meshes":804,"instances":197}
```

**Fix.** Two changes, and the net effect is a *stronger* suite, not a relaxed one:
- `🎭️acceptance/🟦️.ts` — new `surface: "placeholder"` kind + `placeholderPattern`; the preview window is
  graded against its documented `Keine Auswahl|No selection` text instead of a host element that the app
  never renders in that state.
- `🎭️acceptance/🟦️.ts:480` — new test `demonstrator pane "aussuchen": selecting a stock row renders the
  preview scene`: clicks the first `[data-row-id]` in the stock pool and then requires
  `.semio-world-3d-host` with non-empty meshes/instances. This is the first **served** proof of the
  interaction-view threading wave (`📓️fix-2026-09-16-interaction-view-threading.md`).

### 1.5 verfolgen — `expected the map canvas to paint more than one flat color`

Two independent defects, both real.

**(a) The map never boots under `--use-angle=swiftshader`.** The e2e Playwright config launched Chromium
with the software stack. The r3f/WebGL World3d panes are fine under it (aussuchen's grid paints its beams —
see `probe-aussuchen-grid-canvas.png`), but the wasm/wgpu `TiledMapHost` never finishes `attachCanvas`, so
`refreshTiles()` never runs. Measured on the same serve, same page, 30 s settle:

| launch args | `/osm` + `/vt` requests | canvas |
|---|---|---|
| `--use-angle=swiftshader --enable-unsafe-swiftshader --enable-unsafe-webgpu` | **0** | blank (`probe-verfolgen-swiftshader.png`) |
| `--use-angle=metal --enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu` | **65** (`/osm/0/0/0.png`, `/vt/3/0/1.pbf`, …) | continents + labels + marker (`probe-verfolgen-metal.png`) |

Tiles themselves were never the problem: the dev server's proxy answers offline out of the 158 MB
`.🧬semio/🗺️map` cache (`curl /osm/12/2200/1343.png` → 200, 40 144 B; `/vt/12/2200/1343.pbf` → 200,
221 740 B), so no network egress is needed and no fixture-tile mode had to be invented.

**Fix.** `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts:27` — `browserLaunchArgs()` defaults to
the machine's real adapter (`--use-angle=metal` on darwin, `--use-angle=gl` elsewhere, plus
`--enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu`); `DEMONSTRATOR_E2E_GPU=swiftshader` forces
the software stack back for a host without a GPU, knowing the map pane cannot be graded there.

**(b) The pixel probe could never observe paint.** `drawImage(mapCanvas, …)` + `getImageData` answers a
fully transparent `0,0,0,0` buffer even when the map is visibly drawn — the wgpu swap-chain is not a
preserved drawing buffer. Verified with ANGLE-Metal *and* a visibly-painted map: readback `flat 0,0,0,0`.

**Fix.** `🎭️acceptance/🟦️.ts:328` `tiledMapHasVisibleContent` now grades a **composited** Playwright element
screenshot, decoded back to `ImageData` in the page (`countCanvasColors`), counting distinct RGBA only in
the centre 60 % box so the window's own chrome at the edges can never stand in for map content.

### 1.6 koordinator / aggregator / bearbeiten — `unexpected console errors`

Every entry in run 1 was `[DEBUG] `-prefixed host instrumentation (`buildShardClientOptions …`,
`contributions document sources …`, `contributions push skipped unresolved document operators …`). The
coordinator's `significantConsoleErrors` filter already drops those, and all three panes pass in the runs
below. No further cause to fix.

### 1.7 Cross-cutting: single-sample content probes

Once 1.3–1.5 were fixed, generator and verfolgen still failed at 2.8 s / 4.2 s with `meshes=0` /
`distinctColors=1` — because the helpers sampled the surface ONCE, the instant the host element became
visible. A shell reports `data-shell-ready` as soon as the plugin's UI tree mounts, which is strictly
before the guest's first scene crosses the wire (generator's preview reads `meshes=0` at 2.8 s and a
3689-byte `data-meshes-json` once its `previewEval` run lands).

**Fix.** `🎭️acceptance/🟦️.ts:206` `settleContentCount` re-reads until the count reaches its floor or
`SURFACE_CONTENT_TIMEOUT_MS` (60 s) elapses, and every helper takes a `requireContent` flag. It only ever
waits on windows that are *expected* to carry content, so a genuinely empty surface still fails — just
60 s later instead of instantly. `TEST_TIMEOUT_MS` 180 s → 240 s to leave room for koordinator's four
windows. The tiled map passes `minimum = 2`, since a flat grab still counts one colour.

## 2. Files changed

| file | what |
|---|---|
| `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` | robust intro dismissal; `data-host-snapshot-json`; `surface: "placeholder"`; new aussuchen selection test; screenshot-based map paint probe; `settleContentCount` |
| `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts` | `browserLaunchArgs()` — real GPU adapter by default, `DEMONSTRATOR_E2E_GPU=swiftshader` escape hatch |

No application/framework source was changed: every demonstrator defect this suite was pointing at turned
out to be either already fixed in the app (aussuchen selection threading) or a stale/impossible assertion
in the spec, except the map, which was an e2e **launch-args** defect.

## 3. Results

Final full-target run: `bun nx run @semio-tech/mit-bestand-demonstrator:test-e2e`
→ `🗑️generated/test-e2e-2026-09-16-run2.txt` (87 Nx tasks, 80 m 29 s — it rebuilt every plugin wasm
because peers had touched their sources; the Playwright phase itself was 46 s). **7 passed / 3 failed**
(baseline run 1: 1 passed / 8 failed).

| # | test | run 1 | run 2 | note |
|---|---|---|---|---|
| 1 | `DEMONSTRATOR_PANES … (drift guard)` | ✘ | ✓ | coordinator's `../../` fix |
| 2 | `demonstrator overview: pane cards …` | ✘ | ✓ | §1.2 |
| 3 | pane `generator` | ✘ | ✓ | §1.3 + §1.7 |
| 4 | pane `koordinator` | ✘ | ✘ | §4 — `setContributions` admission |
| 5 | pane `aggregator` | ✘ | ✓ | `[DEBUG]` filter |
| 6 | pane `aussuchen` | ✘ | ✘ | §1.4 fixed; fails on §4 only |
| 7 | pane `bearbeiten` | ✘ | ✘ | §4 — `setContributions` admission |
| 8 | pane `verfolgen` | ✘ | ✓ | §1.5 |
| 9 | pane `aussuchen`: selecting a stock row renders the preview scene | — | ✓ | NEW, §1.4 |
| 10 | `landing Förderhinweis …` | ✓ | ✓ | |

Every window-content and chrome assertion in the suite now passes, including koordinator's four CAD
windows, aggregator's split puzzle3d instances, generator's edit-mode preview and verfolgen's map. The
three remaining failures are all the SAME single console error, on the last assertion of their test.

## 4. What remains — one peer-owned regression, NOT in this pass's scope

```
setContributions command failed demonstrator typed command raw JSON exceeds its registered retained-page admission
```

- Raised by `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21405`
  (`FaultCode "interactive-job.raw-wire-limit"` in `admit_command_json_with_proof`) — the app's scoped
  contributions pack is larger than the `max_raw_wire_bytes` its `setContributions` contract registers.
- Logged by `…/🧱️elements/🏛️ShellHost/🟦️.tsx:4647`.
- Hits exactly **koordinator (cad), aussuchen (sourcing), bearbeiten (process)** and no other pane.

It is a peer's live in-flight work, not something this pass introduced or should race:

- It was **absent** from every probe taken before ~15:00 today (`probe-aussuchen-dom.txt`,
  `probe-generator-dom.txt` — their only console errors were `[DEBUG] ` lines) and **present** from the
  ~15:55 live run on.
- `git status` shows `🧰️framework/…/🔌️plugin/🦀️.rs` plus the cad, process and sourcing plugin crates all
  dirty, with mtimes after 14:00 — the same three apps, and only those three.
- The ticket folder already carries in-flight peer notes on exactly this seam:
  `📓️fix-2026-09-16-sourcing-contributions-envelope.md`, `📓️fix-2026-09-16-cad-sourcing-extension-topics.md`.

Whoever owns the contributions-envelope wave should either widen those three apps' registered
`max_raw_wire_bytes` for `setContributions` or page the pack; nothing in this suite needs to change for
it, and the three tests turn green the moment the pack fits.

Two notes for the next runner:

- `:6029` went unresponsive right at the end of this pass (peers regenerating the plugin registry — the
  known "release serve wedges after host edit bursts" behaviour). It was not touched, restarted or killed.
- Probe scripts left in the ticket folder for re-use: `🔍️acceptance-dom-probe.ts` (`PROBE_PANE=<pane>`)
  and `🔍️aussuchen-preview-probe.ts`. Both need
  `PLAYWRIGHT_BROWSERS_PATH=$(pwd)/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`.
