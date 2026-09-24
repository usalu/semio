# U4 — pinch/pan, Diagram arrow navigation, theme contrast warning wired live

Slice U4 of ticket 26/09/18 (`OS-HUB-COLLABORATION-AI-END-TO-END`). Source rows: `📓️g10-goal-gap-reaudit.md`
§B uncovered O1-19b, O1-22b, O1-21; prior art `📓️u2-touch-tablet-contrast-diagram-a11y.md`, `📓️g8-wgpu-parity-spec.md`.
Constraints: TypeScript/React only (no cargo, no wasm), serves 6400–6409, no hub.

Status: **all three items wired and observed at runtime inside one `s` session (en + de)**; nx target run PASS
(8 PASS, 1 UNAVAILABLE = puzzle2d board: its `core.wasm` is not staged). Typecheck of the renderer-react project
0 errors; every touched suite green. One Rust hand-off (dag horizontal two-finger pan) written below, not built.

## 0. Inherited state

No prior U4 work (`wp-u4/` did not exist). Re-read of the live source at 18:10 (not the reports):
- `👆️gesture/🟦️.ts` (U2) was pure math only; three hosts re-assembled pointer set + pinch frame by hand:
  `🖥️Board2dHost` (pinch → `setCameraSilent`), `🌐️World3dHost` (suppression only; Three `OrbitControls`
  `TOUCH.DOLLY_PAN` owned the actual 3D pinch — a second, third-party recognizer). The dag and flow node-graph
  surfaces (`🕸️NodeGraph` `WasmGraphSurface` / `FlowGraphCanvasHost`, the live `dag` viewport inside `s`) had
  NO multi-touch at all and no `touch-action: none`.
- Latent defect in both hand-assembled copies: "multi-touch" was decided per event, so after a pinch the finger
  left down resumed the single-pointer lane — its moves drove marquee/drag and its release was replayed as a
  board click (`Board2dHost` `pointerUpScreen`) / pick (`World3dHost`).
- `🕸️Diagram` (React Flow) had U2's keyboard law; the dag/flow `NodeGraphHost` wrapper handled only `Escape`.
- `📌️ChromePanels` theme tree had U2's contrast badge on appearance paints only.

## 1. Item 1 — one gesture recognizer for every viewport

**Law** (`🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts`): `GestureRecognizer` (mutable cell) over pure
`recognizeGesturePointerDown/Move/Up` transitions returning a `GestureVerdict`
(`single` | `pinchBegin` | `pinch{step}` | `held` | `pinchEnd`). A second contact LATCHES the surface until the
last contact lifts, which fixes the "remaining finger resumes the single lane" defect in every host at once.
Two new pure laws beside it: `pinchZoomNotches` (pinch scale → whole wheel notches with a carried log-space
remainder, for engines that zoom a fixed factor per wheel event) and `applyPinchToOrbit` (target-centred
dolly / orthographic zoom + camera-plane pan, renderer-neutral tuples). Exported from `@semio-tech/framework`.

**Hosts now wired through the one recognizer** (all hand-assembled pointer sets removed):

| host | before | after |
|---|---|---|
| `🖥️Board2dHost` | own pointer set + frame | `gestureRecognizer.down/move/up`; pinch → `board2dPinchCamera` → `setCameraSilent`; latched moves/ups ignored |
| `🌐️World3dHost` | own pointer set (suppression) | recognizer verdicts gate marquee/pick/relocate/face-drag |
| `♾️infinite/🌍️world/🎨️r3f` `WorldOrbitControlsBridge` | Three `OrbitControls` `TOUCH.DOLLY_PAN` (third-party recognizer) | capture-phase listeners feed a `GestureRecognizer`; `pinchBegin` suspends `OrbitControls`, `pinch` → `applyWorldOrbitPinch` (= `applyPinchToOrbit`), `pinchEnd` restores; OrbitControls keeps mouse + one-finger orbit and still emits `start`/`end` |
| `🕸️NodeGraph` `WasmGraphSurface` (dag, the live `s` viewport) | NO multi-touch, no `touch-action` | `touch-none`; `pinchBegin` cancels the first finger's gesture and captures both pointers; `pinch` → `graphPinchWheelPlan` → the session's own `wheelScreen` (zoom notches at the centroid + one non-zoom pan); `pinchEnd` → `emitInteractionState` |
| `🕸️NodeGraph` `FlowGraphCanvasHost` (flow engine) | NO multi-touch | same plan through `issueFlowGestureStep(session.wheelScreen…)` + `wheelGesture.tick()` (one viewport publish when settled) |
| `♾️infinite/🖼️canvas/🎨️react-renderer` `GraphWasmCanvas` (→ `📐️Canvas2dHost`) | NO multi-touch | recognizer in its native listeners; `pinchBegin` → `session.pointerCancel`; `pinch` → new optional `GraphWasmSession.pinch(step)`; `Canvas2dHost`'s `JsonLayersCanvasSession.pinch` = `canvasPinchCamera` (exact law, `CANVAS_CAMERA_ZOOM_BOUNDS`) |
| `🖌️Paint2dHost` | NO multi-touch; overlay without `touch-action` | recognizer on the React handlers; `canvasPinchCamera` → `session.setCamera` per step, ONE `setCamera` dispatch at `pinchEnd`; navigator mode pinches the content camera; overlay `touch-none` |
| `🧭️TiledMapHost` | NO multi-touch | recognizer on its native listeners; `applyPinchToCamera` in the map's canvas-space camera with `getTiledMapCameraLimits`; continuous tile polling (`beginContinuousInteraction("pinch")`) for the gesture, `mirrorSessionCameraToReact` once at `pinchEnd` |
| `🖋️InkCanvasHost` | NO multi-touch | recognizer fed in the CAPTURE phase (a finger on a block still starts the pinch); `applyPinchToOffsetCamera` (ink's `screen = world·zoom + camera` transform, `INK_CAMERA_ZOOM_BOUNDS` shared with the wheel) → draft camera per step, ONE `setCamera` at `pinchEnd` |

**Law correction.** U2's `applyPinchToCamera` zoomed about the NEW centroid and then panned, which drifts the
content from under the fingers by `pan·(1−scale)` per step. It is now the exact finger-anchored move (the world point
under the previous centroid lands under the new centroid at the new zoom); U2's unit expectation was updated to the
exact numbers and an invariant assertion added. `applyPinchToOffsetCamera` is the same law for the ink transform.

`GRAPH_WHEEL_ZOOM_NOTCH` reads the schema tokens `STYLING_METRICS.camera.wheelZoomIn/OutFactor` (1.1/0.9);
a parity test reads both Rust engines' sources. `@semio-tech/infinite-world-r3f` gained the
`@semio-tech/framework` workspace dependency (package.json + the matching `bun.lock` workspace row, hand-written
exactly as `bun install` writes it; the root `node_modules/@semio-tech/framework` link already existed).

**Rust hand-off (not built — no cargo in this slice).** The dag session drops horizontal pan: its wasm binding
`wheelScreen(sx, sy, _delta_x, delta_y, zoom)` discards `delta_x` and `GraphHost::plan_wheel` pans only y.
The TS plan already sends `deltaX = panX`, so this hunk alone completes two-finger pan on the dag with no TS
change (`🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`):

```rust
    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        let plan = self.plan_wheel(sx, sy, delta_x, delta_y, zoom_gesture);
        let _ = self.commit_wheel(plan);
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) -> GraphWheelPlan {
        let cam = &self.dag.host_snapshot.camera;
        let expected = [cam.x, cam.y, cam.zoom];
        let next = if !zoom_gesture {
            [cam.x - delta_x / cam.zoom.max(1e-9), cam.y - delta_y / cam.zoom.max(1e-9), cam.zoom.max(1e-9)]
        } else {
            /* unchanged zoom branch */
        };
        GraphWheelPlan { revision: self.interaction_revision, expected, next }
    }
```
and in the `#[wasm_bindgen(js_name = wheelScreen)]` binding: rename `_delta_x` → `delta_x` and call
`host.wheel_screen(sx, sy, delta_x, delta_y, zoom_gesture)`; every other `plan_wheel`/`wheel_screen` caller
passes `0.0` for `delta_x`.

## 2. Item 2 — Diagram arrow navigation + focus + localized announcements

The live `dag` window inside `s` is NOT the React Flow `🕸️Diagram` — it is `🕸️NodeGraph`'s `NodeGraphHost` →
`WasmGraphSurface` (two canvases). Its wrapper handled only `Escape`. So the law was lifted to ONE place and both
surfaces run it:

- `🖱️ui/🧱️elements/🕸️Diagram/🟦️.tsx`: `diagramKeyboardStep(nodes, focusedId, selectedIds, key, shift, editable)`
  (arrows → focus, Enter/Space → replace, Shift+Enter → toggle, Escape → clear; selection keys inert when
  read-only), `diagramReadingOrderPosition`, `diagramKeyboardAnnouncement(step, nodes, labelOf, translate)`,
  `useDiagramTranslate` (active scope locale + label tier), `DiagramLiveRegion` (`role=status`,
  `aria-live=polite`, `aria-atomic`), `DIAGRAM_KEYBOARD_FOCUS_CLASS`. Re-exported from `@semio-tech/ui-react`.
- React Flow `Diagram`: handler now calls `diagramKeyboardStep`; speaks through `DiagramLiveRegion`; the focused
  node gets the focus ring class + `data-keyboard-focused`. U2's `aria-activedescendant` pointed at a node id no
  element carries (React Flow reserves the node DOM `id`; `domAttributes` omits it by type) — an invalid IDREF;
  removed, the live region is the AT channel.
- `NodeGraphHost` (dag + flow): `role="application"`, always a tab stop (reading a read-only graph is allowed),
  localized `aria-roledescription`/`aria-label` (node + edge counts)/`aria-describedby` key help,
  `DiagramLiveRegion`; keys through `diagramKeyboardStep`; focus → `publishNodeGraphHover` + local
  `session.setHover` (visible highlight on the canvas); Enter → `publishNodeGraphSelection` (the pointer-pick
  lane) or `clearSelection`; a focused node that disappears from the scene drops focus; a key the React Flow
  fallback already handled (`defaultPrevented`) is ignored.
- i18n: `ui.diagram.focusedNode` (`{{node}}, {{position}} of {{count}}` / `… von …`), `selectedNode`,
  `deselectedNode`, `selectionCleared` in the compile-checked schema + en + de bundles (U2's unused noun
  labels replaced).
- Language-agnostic fixture `🕸️Diagram/🧫️fixtures/⌨️keyboard.json` (3 sequences, 19 presses) replayed by the
  component suite.

## 3. Item 3 — theme contrast live warning

`📌️ChromePanels` → Settings → Theme → Appearances → <light|dark> → <group> rows (the only place a user picks a
paint that has a foreground to contrast with):
- `themeContrastBadgeText(paint, fg, gradeLabel, formatRatio, warningLabel)` now also returns `ratio` and a
  `warning` sentence when the pair is below WCAG AA 4.5:1 (`WCAG_AA_CONTRAST` from `@semio-tech/ui-styling`).
- `themeContrastRatioFormatter(locale)` — `Intl.NumberFormat` of the ACTIVE shell locale (`shellLabelLocale()`),
  so the ratio reads `4.48` in en and `4,48` in de; no default language.
- Row: colour input gets `aria-label`, `aria-describedby` → badge (+ warning), `aria-invalid` when failing;
  badge keeps grade text + `data-contrast-grade` + new `data-contrast-ratio`; a failing pair renders
  `<p role="alert" data-contrast-warning>` under the row: en `Low contrast 4.48:1 against the text colour — WCAG AA
  needs at least 4.50:1`, de `Geringer Kontrast 4,48:1 zur Textfarbe — WCAG AA verlangt mindestens 4,50:1`.
- i18n key `ui.settings.theme.contrast.warning` (`{{ratio}}`, `{{minimum}}`, `{{counterpart}}`) in schema + en + de.

**Pairing law corrected (found live, run r4).** U2's badge measured EVERY chrome paint against `foreground`,
so on the untouched default theme `activeForeground`, `accentForeground`, `borderEmphasized` (= foreground) printed
`1.00:1 · Below AA` — text against text, a border against text (screenshot `wp-u4/generated/u4-r4-contrast-en.png`).
New law in `🎨️styling/🌓️theme/🟦️.ts`: `THEME_CHROME_CONTRAST_PAIRS` (9 text-on-surface pairs: `foreground` on
base/panel/muted/hoverInteractiveFill, `mutedForeground` on base/panel, `accentForeground` on accent,
`activeForeground` on activeBase/activeHover; borders unpaired) + `themePaintContrastPairs(palette, key)` (every
pair the key takes part in, as text OR surface, worst first). The editor row shows the worst pair and names the
other paint in the warning. Language-agnostic fixture `🎨️styling/🧫️fixtures/♿️chrome-contrast-pairs.json` whose
expected ratios were computed by an independent Python WCAG implementation (cross-language oracle).

## 4. Runtime proof (measured)

Serve: `bun ./📜️script.ts serve s react dev` in `🧑‍💻dev/📦️packages/🟦️typescript` with `S_OS_PORT=6400 S_LOCAL_ONLY=1`
(no hub, no activation; W1's staged dist, vite serves my TS edits from source). Wrapper pid 50668, vite 50674.
Probe `🐍️u4-pinch-diagram-contrast-probe.mjs` (playwright chromium `--use-angle=metal`, `hasTouch: true`,
touches through CDP `Input.dispatchTouchEvent` so Chrome mints real `pointerType: "touch"` events). One page,
one `s` session: dag opened from the palette (`spawn.dag` → windows `dag-3::dag-main`, `dag-3::dag-compiled-dag`).
Capture of record: `wp-u4/generated/u4-probe-r5.txt` (r4 identical for items 1–2; r5 is after the pair-law fix).

| item | observed (r5) | expected | verdict |
|---|---|---|---|
| pinch spread 100→300 px about the centre | camera `{x:0,y:0,zoom:1}` → `{zoom:2.853116706110003}`; ratio 2.853116706110003; 56 touch pointer events on the surface | ⌊ln 3 / ln 1.1⌋ = 11 notches → 1.1¹¹ = 2.8531167061100025 | PASS |
| two-finger drag (+40, +80 px) | Δy = −28.03951195851137 world, Δx = 0, zoom ×1 | Δy = −80/2.8531 = −28.039511958511373; Δx needs the Rust hand-off (§1) | PASS for y; x = documented gap |
| latch: 2 down, lift one, drag the other 8 steps | camera unchanged (`true`), 0 marquee elements, selection `[]` → `[]` | no single-lane replay | PASS (r1 had a probe bug: CDP `touchEnd` lists the RELEASED points) |
| keyboard en | `application` "Node graph — 5 Nodes, 4 Connections", `tabindex=0`, help via `aria-describedby`; ArrowRight → `mode` "Mode, 5 of 5"; → `scale` "Scale, 2 of 5"; Enter → session selection `["scale"]` "Scale selected, 1 in selection"; ArrowDown → `combine` "Combine, 3 of 5"; Shift+Enter → `["scale","combine"]` "Combine selected, 2 in selection"; Escape → `[]` "Selection cleared"; live region `role=status aria-live=polite`; `document.activeElement` = the surface | law + localized sentences | PASS |
| keyboard de (Settings → General → Language → Deutsch, `lang=de`) | "Mode, 5 von 5", "Scale ausgewählt, 1 in der Auswahl", "Combine ausgewählt, 2 in der Auswahl", "Auswahl aufgehoben"; ARIA snapshot `application "Knotengraph — 5 Knoten, 4 Verbindungen"` / text "Pfeiltasten: Knoten wechseln · …" / `status: Auswahl aufgehoben` | de bundle | PASS |
| contrast en: `light.chrome.panel` set to the foreground `#001117` | badge `1.00:1 · Below AA — contrast too low`, `data-contrast-grade=fail`, swatch `aria-invalid=true`, `aria-describedby` → badge + `<p role="alert">` "Low contrast 1.00:1 with foreground — WCAG AA needs at least 4.50:1" | localized warning with ratio | PASS |
| contrast de | "1,00:1 · Unter AA — zu geringer Kontrast", alert "Geringer Kontrast 1,00:1 zu foreground — WCAG AA verlangt mindestens 4,50:1" | de number format + prose | PASS |

Why the scene selection column stays `[]`: the dag program publishes no `interactionDomain`, so a pointer pick is
session-local too; keyboard selection goes through the SAME lane (`syncInteraction` + the surface's own
`emitInteractionState`) and is read back from the new `data-session-selection-json` mirror.

**Measured default-palette findings (true WCAG failures, NOT fixed — palette generation is out of scope):**
light chrome `mutedForeground` on `panel` 2.34:1 (fail), `mutedForeground` on `base` 3.54:1 (AA large only),
`activeForeground` on `activeHover` 4.45:1 (AA large only). The editor now warns about exactly these on an
untouched light theme.

Console faults: r4 0; r5 5× `WebSocket connection to 'ws://127.0.0.1:50191/bridge' failed … ERR_CONNECTION_REFUSED`
(the shell's MCP bridge port of a peer process that was gone during r5; not touched by this slice).

## 5. Tests and typecheck (all executed; captures in `wp-u4/generated/`)

| suite | command (cwd) | result |
|---|---|---|
| gesture law (in-source) | `SEMIO_TEST_LEVEL=fundamental bun x vitest run --config ./🧪️tests/🎚️config/🟦️.ts 🔨️modules/🕹️interaction/👆️gesture/🟦️.ts` (`🧰️framework`) | **46 passed** (22 U2 + 24 new: 12 fixture replays, latch, 6 notch rows, 4 orbit rows validated against three.js camera basis, offset law) — `u4-gesture-vitest.txt` |
| Diagram (ui-react) | `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🕸️Diagram` | **55 passed** (was 51; 4 fixture/announcer tests new, 2 U2 tests rewritten from the invalid `aria-activedescendant` to the live region + focus ring) — `u4-diagram-vitest.txt` |
| renderer-react: Board2d pinch, World3d multi-touch, ChromePanels, NodeGraph (incl. new `🤏️pinch-wheel`), Ink, TiledMap, Paint2d, Canvas2d | same runner, 8 filters | **12 files, 124 passed** — `u4-renderer-vitest.txt` |
| canvas react-renderer package | `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts` | 1 passed — `u4-canvas-renderer-vitest.txt` |
| styling suite | `bun test ../../🧪️tests/🧩️suite/🟦️.ts` | **65 pass / 1 fail** — the fail is U2's documented pre-existing `panel-tab toggle dividers` CSS assertion; 3 new pair-law tests pass — `u4-styling-test-full.txt` |
| typecheck renderer-react | `bun ./📜️script.ts typecheck` | **exit 0, 0 errors** — `u4-typecheck-renderer.txt` |
| typecheck ui-react / framework | same verb | 44 / 16 errors, **0 in any file this slice touched** (same counts before and after my edits; the 4 styling-suite hits are pre-existing vite-config typing lines 101–574) |
| chrome i18n lint | `bun ./📜️script.ts check-chrome-i18n` (ui-react) | 0 violations |
| lockfile | `bun install --frozen-lockfile --dry-run` | exit 0, `bun.lock` byte-identical (the two hand-written workspace rows are exactly what bun resolves) |
| nx target | `NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s -- http://127.0.0.1:6400/ --tag nx --out wp-u4/generated` | `Successfully ran target`; verdicts in `u4-probe-nx.txt` / `u4-nx-run.txt` |

Not runnable: the r3f package's own vitest config fails at startup (`ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` in
`⏯️tool-run/🟦️.ts` parameter properties, loaded through `🛠️build-tooling` by the CONFIG file under node strip-only) —
pre-existing, not caused by this slice; the r3f bridge is typechecked through the renderer project and observed live.

## 6. Measured vs unverified, honest gaps

Measured live (nx run, `u4-probe-nx.txt`): dag pinch zoom 2.853116706110003 = 1.1¹¹; dag two-finger pan y exact;
dag latch; **World3d (block3d) pinch dolly: orbit distance 17.9795 → 5.9932, ratio 0.333334 (expected 1/3)** through
the r3f bridge recognizer; keyboard walk/selection/announcements en + de; contrast warning en + de.

Unverified at runtime (tests + typecheck only):
1. **Board2dHost pinch in a browser** — the only `s` program on Board2dHost is puzzle2d, whose `core.wasm` is not in
   W1's staged dist (`🧩️puzzle/` has no `*.core.wasm`); block2d renders UiNodes, not a board. Covered by the 8
   component tests (incl. the new latch regression). Needs W1's restage (already requested by S3 for 15 guests).
2. **Flow, Canvas2d, Paint2d, TiledMap, Ink pinch** — wired and typechecked, suites green, not driven live (no
   single healthy `s` program was proven for each today; raster/paint guest also unstaged per S3).
3. **dag horizontal two-finger pan** — engine drops `delta_x` (Rust hand-off in §1). Vertical pan and zoom measured.
4. **Real hardware feel** (trackpad/phone) — only CDP-synthesized touches.
5. **dag keyboard focus is not scrolled into view** — the dag session exposes no camera setter to JS (same hand-off
   family); focus is visible as the engine hover highlight and spoken by the live region.
6. **wgpu parity** (G8 WG-8/10/11) — out of scope (no cargo); the recognizer, keyboard and pair fixtures are
   language-agnostic so a Rust twin can replay them.
7. The live `s` light default palette itself fails WCAG AA on three measured pairs (§4); reported, not changed.

## 7. Files changed

New:
- `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🧫️fixtures/🤏️recognizer.json` — recognizer + notch law fixture
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧫️fixtures/⌨️keyboard.json` — keyboard law fixture
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧫️fixtures/♿️chrome-contrast-pairs.json` — pair law fixture (Python oracle)
- `…/🧱️elements/🕸️NodeGraph/🧪️tests/🤏️pinch-wheel/🟦️.ts` — pinch→wheel plan + Rust factor parity
- ticket: `🐍️u4-pinch-diagram-contrast-probe.mjs`, `🐍️u4-settings-diagnose.mjs`, `🐍️u4-palette-diagnose.mjs`, `🐍️u4-chrome-contrast-census.ts`

Edited:
- `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts` (+ `🧪️tests/🔬️unit/🟦️.ts`) — `GestureRecognizer`, pure transitions, `pinchZoomNotches`, exact `applyPinchToCamera`, `applyPinchToOffsetCamera`, `applyPinchToOrbit`
- `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` — exports
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🟦️.tsx` (+ component tests) — shared keyboard law, announcer, live region, focus ring; invalid `aria-activedescendant` removed
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — diagram announcement + contrast warning keys (en + de), Diagram re-exports
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` (+ `🧪️tests/🧩️suite/🟦️.ts`) — `THEME_CHROME_CONTRAST_PAIRS`, `themePaintContrastPairs`
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/`: `🖥️Board2dHost` (+ pinch test), `🌐️World3dHost`, `🕸️NodeGraph`, `📌️ChromePanels` (+ component test), `📐️Canvas2dHost`, `🖌️Paint2dHost`, `🧭️TiledMapHost` (+ component test harness), `🖋️InkCanvasHost`
- `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — registers `🕸️NodeGraph/🤏️pinch-wheel`
- `…/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` + its `package.json`; `…/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx` + its `package.json`; `bun.lock` (two workspace rows)
- `…/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` — target `s-host-pinch-diagram-contrast-s`
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — row `🛠️dev🪐️os-s🤏️pinch-a11y` (group `3_dev`, order `387.067`, right after `🔭️foreign-kind`)

## 8. Processes started

Serve on 6400: wrapper **50668**, vite **50674**, esbuild **50677** (`wp-u4/generated/u4-serve.txt`) — all killed by
pid at the end of the slice (port 6400 free, 0 survivors). Probe browsers were playwright-owned and closed by each
run. No hub, no cargo, no wasm build, no activation. No other process touched.

Cosmetic note left open: in the narrow Settings dock the grade badge's long `fail` label is clipped on the left
(`…inger Kontrast · 1,00`, screenshot `u4-nx-world3d-after-pinch.png`); the full sentence is in the inline
warning and the badge `aria-label`.
