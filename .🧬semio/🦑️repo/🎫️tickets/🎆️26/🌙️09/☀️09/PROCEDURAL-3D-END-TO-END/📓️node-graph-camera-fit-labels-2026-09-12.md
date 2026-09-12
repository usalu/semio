# Flow node-graph — the opening camera and the node captions (2026-09-12)

Closes the two defects `📓️node-graph-paint-restart-2026-09-12.md` left on a canvas that finally paints:
the Flow window opened on a stored camera that framed **one node of seven**, and every node title was
drawn as a **single glyph** (`E` for `ExtrudeCurve`).

Lane: the node-graph camera/layout/label path — `canvas::camera`, `canvas::text`, `DagHost`'s caption
rows, the React overlay and the wgpu overlay row painter. Two further defects of the *same family* as
the paint lane's `renderCanvas` pre-emption were found on the way and had to be fixed to see anything
at all at runtime (§3). `semio-framework-os-flow-core:wasm` was rebuilt (twice) and the runtime proof
runs on a **second serve on port 6048** — 6018 was never restarted and the procedural plugin was never
restaged.

---

## 0. Headline

| | before | after |
|---|---|---|
| opening camera, 483×814 pane | the document camera `x=94.75 y=-97.5 zoom=1.78`, which shows **0.570** of the graph — one node on screen, the minimap showing the rest | the fit `x=20.11 y=-136.74 zoom=0.9165`, coverage **1.0**, all seven widgets inside the pane |
| node captions at the boot zoom | `E` / `V` / `P` — the draw tier served `DagNodeSpec::abbreviation`, which generation3d derives as the kind name's first letter | `Column Heig…` `Profile Radius` `Side Count` `Polygon` `Vector` `Extrude` |
| a caption wider than its budget | the font was binary-searched down to **4 px** — an unreadable smear | clipped at its **measured** width with one `…`, font never below 8 px |
| a caption drawn ABOVE the node body | budgeted at one node width (it is not inside the body at all) | budgeted at `NODE_TITLE_WIDTH_FACTOR` node widths, published by the host as the row's own `maxScreenW` |
| `Fit graph` | did not exist | a focusable, localized (en/de) control on the Flow window, `F` as a shortcut, persisting through `nodeGraphViewport` like any gesture |

Evidence: `🗑️generated/camera-fit/{after-no-webgpu,after-webgpu,fit-action}/`.

---

## 1. The camera — a decision, not a copy

### 1.1 The law

`canvas::camera` (`♾️infinite/🖼️canvas/🦀️.rs`) gained the framing half it never had:

```rust
pub fn content_coverage(content: &ContentBounds, camera: &Camera, viewport: &Viewport) -> f64
pub fn fit_camera(content: &ContentBounds, viewport: &Viewport, padding_px: f64) -> Camera
pub fn startup_camera(stored: Option<&Camera>, content: Option<&ContentBounds>, viewport: &Viewport,
                      padding_px: f64, min_coverage: f64) -> (Camera, bool /* fitted */)
```

* **A stored camera is honoured only when it already frames the graph it was stored for** —
  `content_coverage >= CONTENT_FRAMED_MIN_COVERAGE` of the graph's own area.
* Otherwise (and whenever there is no stored camera) the graph is **fitted**: centred, zoomed to
  `min(vw/cw, vh/ch)` with `CONTENT_FIT_PADDING_PX` of screen margin per side, clamped to the canvas
  camera range so a graph past the zoom-out floor is centred rather than over-zoomed.
* A **fit is persisted exactly the way a pan or a zoom gesture is** (`nodeGraphViewport`), so the next
  open honours it and the taxonomy stays "persisted local-only" as CLAUDE.md requires.
* An **example switch** is the one case where an ALREADY LIVE camera is re-framed, and only when the
  new graph left the view entirely (`content_coverage <= CONTENT_REFIT_MAX_COVERAGE`). A camera the
  viewer set themselves is never yanked back for an ordinary edit.

Degenerate content is handled on the axes it has, so a single node or a row of nodes at one `y` never
reads as zero coverage and re-fits forever.

### 1.2 The one judgement call, calibrated against the defect

`CONTENT_FRAMED_MIN_COVERAGE = 0.85`. The live generation3d boot camera covers **0.570** of its own
graph (measured, §4.1), so a 0.6 threshold would have decided the defect by 0.03 — a slightly wider
pane would have adopted the bad camera again. 0.85 decides it by 0.28 and still adopts a camera with a
sliver of the graph off the edge (the `a-stored-camera-with-a-small-overhang-is-still-adopted` row, at
0.9). All three constants are styling tokens (`metrics.camera.{contentFitPaddingPx,
contentFramedMinCoverage,contentRefitMaxCoverage}`), regenerated through
`@semio-tech/ui-styling-tokens:generate`.

### 1.3 Two implementations, one fixture

| implementation | used by |
|---|---|
| `canvas::camera::{content_coverage,fit_camera,startup_camera}` + `DagHost::{content_world_bounds,camera_content_coverage,fit_camera_to_content,adopt_camera_or_fit,refit_camera_if_content_left_view}` | the wgpu shell (`sync_node_graph_engine`), and every other consumer of the shared board |
| `dagContentBounds`/`dagContentCoverage`/`dagFitCamera`/`dagStartupCamera` in `🕸️NodeGraph/🟦️.tsx` | the React shell, which owns the pane's pixel size and the scene's node records |

They are the same shape as the existing `dagWorldToScreen`/`canvas::camera::world_to_screen` pair that
already lives on both sides, and they are pinned to one file:
`♾️infinite/🖼️canvas/🧪️tests/📷️camera-fit/🔣️.json` (10 rows, §5.1).

`DagHost::minimap_widget_content_bounds` is now a padded view of `content_world_bounds` — the minimap
and the fit frame the *same* union by construction rather than by two copies of the same loop.

### 1.4 Where the decision is made

* React: `applyFlowStartupCamera` in the attach `.then()`, once per surface, against the pane the
  surface actually got. `refitFlowCameraIfContentLeftView` runs on a scene pass whose **layout
  signature** changed (`nodeGraphContentSignature` — node ids and boxes only, so hover/selection/eval
  churn never triggers it).
* wgpu: `sync_node_graph_engine` uses `adopt_camera_or_fit` for the FIRST viewport it sees and
  `set_camera` for every later one (those are viewports this renderer itself persisted), plus
  `refit_camera_if_content_left_view` when the fixture changed under an unchanged viewport.

`syncFlowSessionStructureFromScene` no longer takes `applyCamera` at all: the camera is never copied
from `scene.viewport` on a resync, and the one moment a stored camera is considered is the first
attach.

### 1.5 `Fit graph`

A `<button>` on the Flow node-graph surface: `aria-label`/`title` from `ui.nodeGraph.fitGraph`
(en `Fit graph` / beginner `Show the whole graph`; de `Graph einpassen` / `Ganzen Graph zeigen`,
added to both locale tables in `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` and to the
`UiTranslations` type), `aria-keyshortcuts="F"`, and an `F` handler on the surface container that
ignores editable targets and modified keys. It fits, then dispatches `nodeGraphViewport` — the same
action a wheel gesture dispatches, so persistence is identical. Proven by keyboard at runtime (§4.3).

---

## 2. The captions — clipped by measure, never shrunk into a smear

### 2.1 Root cause, measured not guessed

A probe over the real payload (`🗑️generated/camera-fit/probe-2.txt`, since replaced by the laws in
§5.2) walked the LOD ladder:

```
zoom=0.9  lod=normal   titles=[… "Brep.curve.polygon" "Math.vector" "Brep.solid.extrude"]
zoom=1.78 lod=detail   titles=[… same …]      ← with kind infos applied, these are "P" "V" "E"
zoom=3.0  lod=micro    titles=[… same …]
```

Two independent halves, both confirmed:

1. **`DagDrawLod::Detail` served `DagNodeSpec::abbreviation`.** The ladder was
   `Compact|Detail → Abbreviation`, `Normal|Micro → Name` — **non-monotone**: zooming IN past the
   `Normal` band (floor 1.50 on the shifted scale; the document camera is 1.784) replaced the title
   with the abbreviation, and zooming further in to `Micro` brought the name back.
2. **generation3d abbreviates to one letter.** `document_operator_records`
   (`🧊️generation3d/…/🪟️windows/🕸️flow/🦀️.rs:238`) derives a record for every operator kind its
   catalogue does not carry, with
   `abbreviation: name.chars().next().map(|ch| ch.to_uppercase().to_string())`. `brep.solid.extrude`
   → name `extrude`, abbreviation `E`.

So neither the glyph atlas, nor the draw list, nor the 2D replay was at fault — which is why the
canvas-paint lane's pixel statistics passed while the window read as unlabelled.

### 2.2 The fix, structurally

**A draw tier decides WHETHER a node is captioned, never WHAT the caption says.**
`DagNodeLabel` is now `{ None, Name }`; `node_label()` returns `None` only for `Minimap`/`Overview`.
The caption is always `DagNodeSpec::name`, and width is answered by clipping:

```rust
pub const LABEL_ELLIPSIS: &str = "…";
pub fn ellipsize_by_measure(text: &str, max_width: f64, measure: impl FnMut(&str) -> f64) -> String
```

— the longest prefix that still fits once `…` is appended, measured by **the caller's own measure**:
the wgpu glyph atlas (`fit_overlay_label_text`), the browser canvas `measureText`
(`dagEllipsizeOverlayLabel`), a synthetic advance in the laws. Empty text and a non-positive budget
draw nothing; a budget too narrow for one glyph plus the ellipsis draws the bare `…`, so a clipped
caption is always visibly clipped.

The font clamp in both presentations now shrinks **only to fit the row's height**, never below
`metrics.label.legibleMinPx` (8). It used to binary-search on width down to 4 px, which is the second
half of the same defect: a caption wider than its node became a smear rather than a word.

### 2.3 The budget belongs to the host

A computation node's title is drawn **above** the body (`computation_name_world_center`), so clipping
it to the body's width truncated every operator name to three or four glyphs (measured live:
`Pol…` `Ve…` `Ext…`). Both presentations were deriving that budget privately from `nodeW`. The host
now publishes the row's own `maxScreenW`, because only it knows where the caption sits:

| row | budget |
|---|---|
| title above the body (computation/slider layout) | `node.width * NODE_TITLE_WIDTH_FACTOR` (2.0) |
| title inside the body (compact tier) | `node.width` |
| rotated title (io widget) | `node.height` |
| port label | its own port column width |

all × `zoom` × `DAG_LABEL_SCREEN_INSET` (0.88 — the number both presentations already used, now named
once). Both presentations prefer the published budget and keep their old derivation as the fallback.
Result at the fit zoom: `Profile Radius`, `Side Count`, `Polygon`, `Vector`, `Extrude` whole, only
`Column Heig…` clipped.

---

## 3. Two pre-emption defects found on the way (without which nothing was visible)

Both are the family the paint lane named for `renderCanvas`: `observeFlowTask` keeps one task per
feature key and **cancels the previous**, which is right for a superseded query and fatal for anything
the guest must apply or paint.

### 3.1 The document sync never landed

The scene changes on every `flowEvalTick` (≈700 ms on a live graph), so `synchronizeDocumentJson` was
re-issued and cancelled before the guest ever applied it. Measured live — the issue line repeating
every ~700 ms with no settle, and:

```
[DEBUG] node-graph surface ready surface=window:procedural-main 7 6     ← the scene had 7 nodes
[DEBUG] flow surface context created 2d 483x814 dpr=1 widgets=3 commands=16   ← the guest had 3
```

The window therefore painted the **default three-widget flow fixture** at the generation3d camera for
the whole session. `setNeuronKindInfosJson` had the identical problem, which is why the captions read
`Bre…`/`Ma…` (the kind-derived fallback names) even after the document landed.

Fix: `sendFlowPayloadOnce(session, feature, payload, issue)` — an identical payload is never re-sent,
and a send that did **not** deliver clears the record so a cancelled or failed send is retried by the
next scene pass rather than lost. Applied to `synchronizeDocumentJson`, `setNeuronKindInfosJson` and
`setCatalogueJson`. `observeFlowTask` gained an optional `settled(delivered)` hook for it.

### 3.2 The overlay painted exactly once per session

`paintOverlays` reads ten session queries through `observeFlowTask`. A second pass starting while the
first was in flight cancelled the first pass's own reads, its `Promise.all` rejected, and **nothing
was painted**. Since every invalidation calls it, the overlay painted once: measured as **three
`fillText` calls in a 60 s run**, all in the first 100 ms, showing the pre-sync picture forever.

Fix: `paintOverlays` coalesces exactly like `renderFlow` — an in-flight pass is left alone, a request
arriving during one is re-issued when it settles. At most one extra pass, never a dropped one.

Both fixes are in `🕸️NodeGraph/🟦️.tsx` and carry the measurement in their docstrings.

### 3.3 One more trap worth recording

`dispatch` is a `useCallback` over the `onAction` prop, so depending on it in the attach effect
**re-attaches the canvas on every parent render** and in the scene effect re-issues
`synchronizeDocumentJson` per render (caught by the React suite: `expected […] to have a length of 5
but got 6`). Effects reach the dispatcher through `dispatchRef` instead.

---

## 4. Runtime — port 6048, both modes

Probe: `🐍️camera-fit-probe.mjs` (new). It wraps `CanvasRenderingContext2D.prototype.fillText` in an
init script, so "the titles are full words" is the **recorded strings** rather than an impression, and
"all seven nodes are framed" is every widget having a caption anchored inside the overlay canvas.
Launch args copied from `🐍️canvas-paint-probe.mjs`. Serve: `📜️serve-generation3d-camera-fit.sh`
under `screen -dmS g3dfit`, stopped at the end of the lane.

| run | result |
|---|---|
| `after-no-webgpu` (no flags, `present=2d`) | captions `["Column Heig…","Profile Radius","Side Count","Polygon","Vector","Extrude"]`, ports `["! Num","! …","! x","! y","! z"]`, **0 single-glyph captions**, **0 off-screen anchors**, fit logged — **PASSED** |
| `after-webgpu` (`--enable-unsafe-webgpu --ignore-gpu-blocklist --use-angle=metal`, `present=webgpu`) | byte-identical caption set, same verdict — **PASSED** |
| `fit-action` | see §4.3 — **PASSED** |

`after-no-webgpu/2-surface.png` and `after-webgpu/2-surface.png` are the same picture (the scene on the
GPU vs the 2D replay, captions from the shared overlay): three sliders, `Polygon`, `Vector`, `Extrude`
and the preview node, all inside the 483×814 pane, with the `Fit graph` control at the top left.

### 4.1 The opening camera, live

```
[DEBUG] node-graph fit on open surface=window:procedural-main {"x":20.114…,"y":-136.741…,"zoom":0.9165…}
```

The stored camera it replaced covers `0.5703074829530029` of the graph (§5.2's falsification prints
exactly that number), against a `0.85` threshold.

### 4.2 Example switch

Picking another bundled example does **not** re-fit, and must not: the new graph is still in view, so
`refit_camera_if_content_left_view` correctly declines (`CONTENT_REFIT_MAX_COVERAGE`). The re-fit case
is pinned where it can be exact — `a_graph_that_left_the_view_refits_and_one_that_did_not_is_left_alone`
over the real payload, and the `…left-the-graph-entirely…` fixture row on both implementations.

### 4.3 `Fit graph`, by keyboard

Wheel the camera far in, then **Tab focus + Enter**:

```
keyboardFocused: "Fit graph"
awayCaptions:    ["? X","? Y","? Z","! Vector","! X","! Y","! Z","* Errors","ExtrudeCurve","! Wire",…]
afterFit:        ["? x","? y","? z","! …","! x","! y","! z","*…","Extrude…","! …",…]
viewportDispatches: 3
```

`3-after-fit-action.png` shows the focus ring on the control and the whole graph framed again.
`ExtrudeCurve` drawn in full at the zoomed-in camera is the same law from the other end: the caption is
the whole name, and the clipping at the fit zoom is purely the measured budget.

### 4.4 What needs a restart or a rebuild

| change | reaches a running serve how |
|---|---|
| `🕸️NodeGraph/🟦️.tsx`, `🖱️ui` locale tables | watched source modules — a page **reload**, except that a serve started with `SEMIO_VITE_HMR=0` caches transforms and needs a **serve restart** to pick up a later edit (measured: an edit made after the previous request was served from cache) |
| `♾️infinite` (`canvas::camera`, `canvas::text`, `DagHost` rows, the LOD ladder) | inside the flow-core guest — needs **`bun nx run semio-framework-os-flow-core:wasm`** and then a serve restart, because vite never invalidates `node_modules/@semio-tech/flow-core` |
| `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | the wgpu shell only — a native renderer build |
| styling tokens | `bun nx run @semio-tech/ui-styling-tokens:generate` (run; `🦀️tokens.generated.rs` and the TS/CSS/C#/Python twins are regenerated) |

The wasm was rebuilt twice in this lane (77 s, then 32 s). **6018 was not restarted**, so it still
serves the previous guest; its next restart picks the new one up.

---

## 5. Tests — all run, all quoted

### 5.1 Shared fixtures (language-agnostic)

* `♾️infinite/🖼️canvas/🧪️tests/📷️camera-fit/🔣️.json` — 10 rows of
  `graph bounds + viewport + stored camera → {camera, fitted, coverage, refits}`, with the law, the
  calibration note and the constants in `provenance`.
* `♾️infinite/🖼️canvas/🧪️tests/🏷️label-fit/🔣️.json` — 8 rows of `text + maxWidth + charWidth →
  clipped string`, measured with the synthetic advance `charWidth × characters` so both
  implementations are driven by the **same** measure and a row pins the algorithm, not a font file.

### 5.2 Rust

```
cargo test -p semio-framework-os-infinite --lib -- camera_fit_laws label_fit_laws dag_paint_scene_keeps_labels
test canvas::camera_fit_laws::every_fixture_row_opens_on_the_camera_the_law_names ... ok
test canvas::camera_fit_laws::a_fitted_camera_frames_the_whole_graph_it_was_fitted_to ... ok
test canvas::camera_fit_laws::the_fit_leaves_the_padding_it_promises ... ok
test canvas::camera_fit_laws::a_camera_that_frames_nothing_is_never_reported_as_framing_something ... ok
test canvas::camera_fit_laws::the_shared_constants_are_the_ones_the_fixture_was_pinned_against ... ok
test canvas::label_fit_laws::every_fixture_row_clips_to_the_string_the_law_names ... ok
test canvas::label_fit_laws::a_clipped_caption_keeps_the_longest_prefix_that_fits ... ok
test canvas::label_fit_laws::the_measure_the_caller_passes_is_the_one_that_decides ... ok
test canvas::label_fit_laws::no_draw_tier_serves_a_shorter_caption_than_the_tier_below_it ... ok
test board::ports::directed_dag::tests::dag_paint_scene_keeps_labels_when_lod_forced_at_low_zoom ... ok
test result: ok. 10 passed; 0 failed
```

```
cargo test -p semio-framework-os-flow --lib draw_list_laws
test wasm_session::draw_list_laws::the_generation_3d_document_camera_does_not_frame_its_own_graph_and_loses_to_the_fit ... ok
test wasm_session::draw_list_laws::a_graph_that_left_the_view_refits_and_one_that_did_not_is_left_alone ... ok
test wasm_session::draw_list_laws::every_captioned_zoom_band_names_the_node_instead_of_abbreviating_it ... ok
test wasm_session::draw_list_laws::a_title_above_the_node_body_is_budgeted_wider_than_the_body ... ok
… (the four pre-existing draw-list laws) …
test result: ok. 8 passed; 0 failed
```

The first drives the REAL generation3d payload at 483×814 and asserts every one of the seven widgets
is inside the pane after the fit (`entity_screen_json`), not just that a number came out right.

**Falsified, not assumed** (`🗑️generated/camera-fit/falsify-*.txt`):

| temporary change | result |
|---|---|
| `startup_camera` always adopts a stored camera | `camera_fit_laws` 2 FAILED, and the real-payload law fails with `the opening camera kept a stored camera that shows 0.5703074829530029 of the graph` |
| `Compact`/`Detail` serve `node.abbreviation` again (HEAD's content) | `at zoom 1.78443256160111 (lod detail) the caption "P" is a single glyph — the abbreviation tier is back` |
| the 2D overlay draws `row.text` instead of the clipped string | the overlay law FAILED |

All three reverted; the passing runs above are after the revert.

### 5.3 JavaScript twin (the 2D replay's text measure) — `🔬️engine-contract/🟦️.ts`

`bun nx run @semio-tech/framework-renderer-react:test-long -t "node-graph caption clipping"` → **3
passed**; `-t "node-graph opening camera"` → **3 passed**. They read the same two fixture files from
where the Rust laws live (not a copy), and the overlay row asserts what `paintDagLabelOverlays`
actually draws through a recording 2D context: one `fillText`, ending in `…`, starting with `Brep`, at
a font **≥ 8 px**, within the published budget — and the same overlay drawing `Extrude` whole when it
fits.

### 5.4 Neighbouring suites

| command | result |
|---|---|
| `bun nx run @semio-tech/framework-renderer-react:test-long` | **991 passed, 9 failed** — 6 × the extension-SDK evaluate/dispatch lane, 1 × `noteShellCommand`, 1 × `PluginRuntime`, 1 × `readAppDocumentPack`: all peer lanes, the same shape the paint lane recorded (it saw 970/9). My 6 new rows pass. |
| `bun nx run semio-framework-os-flow-core:test-browser` | passed |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | 0 errors, 131 pre-existing warnings |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 engine_canvas::node_graph_attach_tests engine_canvas::engine_surface_attach_tests` | 9 passed, 1 failed (`saturated_graph_and_board_wheel_queues_preserve_cameras`, a **board2d** `defers_descriptor_sync_from_js` assertion — the peer's `➕️normal` lane). Run in parallel, three of these fail on thread-local registry contention; serially they pass. |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | 861 pre-existing errors repo-wide (`import.meta.dir`, peer test files), **none** in `🕸️NodeGraph`, `📚️I18n` or the engine barrel |
| `cargo test -p semio-framework-os-infinite --lib` | 324 passed, 15 failed — 11 × `world::tests` and 3 × `board_host` (the peers' uncommitted `🌍️world/🦀️.rs` and `➕️normal/🦀️.rs` diffs), plus 2 × dag JSON (`Float(0.0)` vs `UInt(0)`, and a cursor census of 20 vs 16 bytes on escaped ids) which belong to the peer's `os_pack::json` lane |
| `cargo test -p semio-framework-os-flow --lib` | 156 passed, 62 failed — **26** panic `ordered-map root must be explicitly retired before drop` in the value layer the peer is editing (`📡️replication/📡️wire/🦀️.rs`) and the rest cascade from the poisoned `Once` that leaves behind. The 8 `draw_list_laws` pass in the same binary. |

### 5.5 One pre-existing red test repaired rather than left lying

`dag_paint_scene_keeps_labels_when_lod_forced_at_low_zoom` asserted `scene.path_count() > 12` and
measured **10** — proven pre-existing by re-running it with HEAD's exact caption source
(`🗑️generated/camera-fit/pre-existing-path-count.txt`: still 10). A captioned tier *delegates* its
caption to the overlay, so the scene's path count is node chrome only, two paths per node. The test
now asserts what the architecture guarantees: every node's chrome painted, and one overlay row per
node carrying that node's **name**.

---

## 6. Findings handed on (not this lane)

1. **`observeFlowTask` pre-emption is a repo-wide shape, not three incidents.** `renderCanvas` (paint
   lane), `synchronizeDocumentJson`/`setNeuronKindInfosJson` and `paintOverlays` (this lane) were all
   silently starved by it. Every remaining `observeFlowTask` call whose payload the guest must *apply*
   — `setSelection`, `setHover`, `applyEvalOutputsJson`, `setNodeStatuses`, `setPreviewOff`,
   `setComputingProgress`, `setCanvasThemeJson` — has the same exposure; they are cheap and idempotent
   today, which is the only reason they are not visibly broken.
2. **A serve started with `SEMIO_VITE_HMR=0` does not pick up later source edits** — it serves the
   transform cached at the previous request. Costed this lane three probe runs before it was noticed;
   worth a line in the serve scripts.
3. **generation3d's document-derived operator names are the kind's last segment** (`extrude`,
   `vector`, `polygon`) while the outline shows `ExtrudeCurve`. Two different display names for the
   same node in two windows of the same app.
4. **The wgpu overlay row painter ignores `layout: "vertical"`** — it never rotates, so an io-widget
   caption is drawn horizontally there and along the node's height in the browser. Pre-existing;
   `maxScreenW` now at least gives both the same budget.
5. **The flow window's title bar has no `Fit graph` entry**, only the surface control. If the plugin
   wants it in the window's action list (and therefore in the command palette and key bindings), it
   needs a `nodeGraphFitGraph` action whose command carries the fit — which requires the plugin to
   learn the pane's pixel size, since a fit is not computable from the document alone.

---

## 7. Files

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` (+ regenerated `🦀️tokens.generated.rs` and twins) | `metrics.camera.{contentFitPaddingPx,contentFramedMinCoverage,contentRefitMaxCoverage}`, `metrics.label.legibleMinPx`, `metrics.dag.nodeTitleWidthFactor` |
| `♾️infinite/🖼️canvas/🦀️.rs` | **new** `camera::{ContentBounds,content_coverage,fit_camera,startup_camera}` + the three constants; **new** `text::{LABEL_ELLIPSIS,ellipsize_by_measure}`; two test mounts |
| `♾️infinite/🖼️canvas/🧪️tests/📷️camera-fit/{🔣️.json,🦀️.rs}` | **new** — the opening-camera fixture and its 5 laws |
| `♾️infinite/🖼️canvas/🧪️tests/🏷️label-fit/{🔣️.json,🦀️.rs}` | **new** — the caption-clipping fixture and its 4 laws |
| `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` | **new** `🔖️ContentFraming` region on `DagHost`; `minimap_widget_content_bounds` built on it; `node_label_text` serves the name; every caption row publishes `maxScreenW`; `DAG_LABEL_SCREEN_INSET` |
| `♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs` | `DagNodeLabel` is `{None,Name}`; `node_label()` captions every tier above `Overview` |
| `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs` | ladder assertions follow the new tier; the low-zoom label test asserts the overlay row instead of a path count |
| `🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs` | **new** `📷️OpeningCamera` (2 laws) and `🏷️NodeCaptions` (2 laws) regions over the real generation3d payload |
| `📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` | **new** `📷️ContentFraming` region (bounds/coverage/fit/startup + `nodeGraphContentSignature`); `dagEllipsizeByMeasure`/`dagEllipsizeOverlayLabel`; height-only font clamp with a legible floor; published-budget clipping in `paintDagLabelOverlays`; `applyFlowStartupCamera`/`refitFlowCameraIfContentLeftView`; `sendFlowPayloadOnce` + `observeFlowTask`'s `settled` hook; `paintOverlays` coalescing; `dispatchRef`; the `Fit graph` control and its `F` shortcut |
| `📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | the new camera/label exports added to both barrels |
| `📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | `clamp_label_font_px` clamps height only; **new** `fit_overlay_label_text`; the published `maxScreenW`; `sync_node_graph_engine` adopts-or-fits the first viewport and re-fits a graph that left the view |
| `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | `ui.nodeGraph.fitGraph` in the type and in both locale tables |
| `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | **new** `📷️CameraAndLabelFitTwins` region — 6 rows driving both shared fixtures and the real overlay painter |
| `<ticket>/🐍️camera-fit-probe.mjs` | **new** — the `fillText` recorder, the framing/caption checks, the `Fit graph` keyboard step |
| `<ticket>/📜️serve-generation3d-camera-fit.sh` | **new** — the port-6048 serve this lane measured on |

Evidence: `🗑️generated/camera-fit/` — `after-no-webgpu/`, `after-webgpu/`, `fit-action/`,
`cross-6018/`, `diag-fixture/` (each with `captions.json`, `camera-fit-check.json`, `console.txt`,
`1-settled.png`, `2-surface.png`, and `3-after-fit-action.png` for the action run), plus
`falsify-camera.txt`, `falsify-caption.txt`, `falsify-js.txt`, `pre-existing-path-count.txt`,
`rust-infinite-laws.txt`, `rust-flow-laws.txt`, `react-test-long.txt`, `typecheck.txt`,
`wgpu-check.txt`, `wgpu-node-graph-tests.txt`, `flow-core-wasm-build.txt`, `serve-6048.txt`.
