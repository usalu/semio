# Runtime verification checklist — procedural 3d React target (2026-09-09)

For the coordinator to run in the Claude browser pane against
`http://127.0.0.1:6018/?plugin=generation3d` (`s.procedural.generation3d@1`, `serve-generation3d-react-dev`
per `📓️wave1-boot-report-2026-09-09.md`). Format per step: **step → how → expected → evidence**.

**Blocking precondition (as of 19:46, `📓️runtime-verification-2026-09-09.md`):** the plugin actor
trapped on its first `pending_effects` (`ordered-map root must be explicitly retired before drop` +
`surface context exceeds its wire bound`), dispatched to a `runtime-defects` lane not yet landed per
`📓️status.md`. Steps below assume that fix has landed; if step 4 reproduces the same trap, stop and
report it rather than working around it.

All file:line citations below were read directly from source during this audit (2026-09-09); re-check
if the peer-churn windows noted in other audits have since moved these lines.

---

## 1. Boot

| # | Step | How | Expected | Evidence |
|---|---|---|---|---|
| 1 | Size the pane | `resize_window` custom 1440×900 (pane may stay hidden while driven — see §6) | viewport reports 1440×900 | tool result |
| 2 | Open the URL | `navigate` → `http://127.0.0.1:6018/?plugin=generation3d` | title starts `semio · os`, then transitions | `read_page` title / `get_page_text` |
| 3 | Confirm shell settle | `read_page` title again after ~10-40s | title becomes `semio · procedural · 3d` (pattern `semio · <shellLabel> · <variant>`, per puzzle3d's observed `semio · puzzle · 3d` — exact string not independently re-derived here, verify) | title text |
| 4 | Watch JS-side boot markers | `read_console_messages` (no filter first) | transient `[DEBUG] local interaction observation failed …` (🏛️`ShellHost/🟦️.tsx:3606`) and `[DEBUG] action failed …` (🏛️`ShellHost/🟦️.tsx:5477`) for ~10-30s while cooperative-maintenance catches up — this exact pattern was a real framework bug fixed across puzzle3d rebuilds #4-#8 (`…PUZZLE-3D-END-TO-END/📓️2026-09-09-runtime-verification.md`); expect it NOT to reproduce past ~30s, but verify, don't assume | console text + timestamps |
| 5 | Watch Rust-side trap/fault markers | `read_console_messages` again, grep for `trapped`, `more-work`, `plugin.internal`, `unreachable`, `ordered-map root must be explicitly retired`, `surface context exceeds its wire bound` | **none present** after settle | full console text (these strings do NOT appear anywhere in `🏛️ShellHost/🟦️.tsx` or `🔌️PluginRuntime/🟦️.tsx` — confirmed by grep this audit; they are Rust `eprintln!` from the wasm worker, visible ONLY via `read_console_messages`, never via a page-injected `console.*` hook) |
| 6 | Confirm DOM/canvas | `read_page` / `javascript_tool`: `document.getElementById("root")` populated; count `document.querySelectorAll("canvas").length` | 0 canvases at t+10s → ≥2 canvases once both `procedural-preview` and (in Generate mode) `generation3d-generate-preview` mount | count + screenshot |
| 7 | Confirm chrome | `read_page` | panel tabs Artifact/Catalogue/Inspection, Fullscreen control, mode tabs Edit/Generate, example picker showing a default example (Hexagonal Mushroom Column expected per prior boot) | accessibility tree |

**Fiber-read technique** (used for every `shellState.*` read in §2-§5 below): the puzzle3d ticket's own
runtime-verification doc only *describes* this in prose (`…PUZZLE-3D-END-TO-END/📓️2026-09-09-runtime-verification.md:8`:
"React-fiber reads of `shellState`") with no embedded snippet; the actual verbatim technique lives in a
sibling ticket's probe script, `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️21/FIX-AGGREGATOR-ABBAU-AUFBAU-EXAMPLE-CRASH/probe-scene.ts:63-76`:

```ts
const rootKey = Object.keys(document.getElementById("root") || {}).find((k) => k.startsWith("__reactContainer$") || k.startsWith("__reactFiber$"));
const rootEl = document.getElementById("root") as any;
if (rootEl && rootKey) {
  const fiber = rootEl[rootKey];
  visit(fiber.stateNode ? fiber : fiber, 0);
  // react 19 container
  if (fiber?.current) visit(fiber.current, 0);
}
// fallback: scan all nodes
if (found.length === 0) {
  document.querySelectorAll("*").forEach((el) => {
    const k = Object.keys(el).find((x) => x.startsWith("__reactFiber$"));
    if (k) visit((el as any)[k], 0);
  });
}
```
`visit(fiber, depth)` walks `fiber.child`/`fiber.sibling`, reading `fiber.memoizedProps || fiber.pendingProps`
at each node (same file, lines 36-61) — for generation3d, look for `session`, `onAction`, or the world3d
scene props (`statusJson`, `meshesJson`, `instancesJson`) instead of the puzzle-specific fields shown there.

---

## 2. Windows and chrome

DOM window ids follow `framework.window.{elementIdSegment(windowId)}` (lossy kebab→camelCase, mirrored
Rust/TS, `🆔️ElementId/🟦️.tsx:27-40`); confirmed pattern at `🏛️ShellHost/🟦️.tsx:8023-8038`. Panel tab ids
follow `framework.panelTab.framework.panel.{anchor}` (`owned-locale-detector-retirement/🟦️.tsx:482`).

| # | Step | How | Expected | Evidence |
|---|---|---|---|---|
| 8 | Edit mode → Flow window | find `#framework.window.proceduralMain` | window id `procedural-main`, body `procedural.play.main`, `SurfaceKind::NodeGraph` (`…✏️edit/🪟️windows/🕸️flow/🦀️.rs:11-12,20-21`) | element present |
| 9 | Edit mode → 3D Preview window | find `#framework.window.proceduralPreview` | window id `procedural-preview`, body `procedural.play.preview`, `SurfaceKind::World3d` (`…✏️edit/🪟️windows/👁️preview/🦀️.rs:13-14,22-23`) | canvas present |
| 10 | Switch to Generate mode | click `#playground.navbar.modes` → `generate` item (`🏛️ShellHost/🟦️.tsx:7671`, mode id from `…🎭️modes/🧬️generate/🦀️.rs:6`) | layout swaps to `generation3d-generate` named layout | 3 new windows below mount |
| 11 | Generate → Generations list | find `#framework.window.generation3dGenerations` | window id `generation3d-generations`, body `procedural.play.generations` (`…🧬️generate/🪟️windows/🗂️generations/🦀️.rs:8-9,17-18`) | element present |
| 12 | Generate → Form window | find `#framework.window.generation3dGenerateForm` | window id `generation3d-generate-form`, body `procedural.play.generate-form` (`…🧬️generate/🪟️windows/📝️form/🦀️.rs:11-12,20-21`) | element present |
| 13 | Generate → output Preview window | find `#framework.window.generation3dGeneratePreview` | window id `generation3d-generate-preview`, body `procedural.play.generate-preview` (`…🧬️generate/🪟️windows/👁️preview/🦀️.rs:16-17,25-26`) | canvas present |
| 14 | Back to Edit mode | click `#playground.navbar.modes` → `edit` item (mode id `…✏️edit/🦀️.rs:7`) | Flow + 3D Preview windows return | element present |
| 15 | Panels | find/click panel tabs | Artifact → body `procedural.play.document` (`📌️panels/🗿️artifact/🦀️.rs:10`); Catalogue → `procedural.play.catalogue` (`📌️panels/🛍️catalogue/🦀️.rs:9`); Inspection → `procedural.play.inspection` (`📌️panels/🔍️inspection/🦀️.rs:11`) | each panel body renders without a `[DEBUG]` fault |
| 16 | Preview-window utilities | find utility bar under `#framework.window.proceduralPreview` | three "transform"-group utilities: `move` (default active, `active_utility_id.unwrap_or("move")`, `✏️editor/🦀️.rs:230`), `rotate`, `scale` (`✏️editor/🦀️.rs:1581-1584`) | toggle buttons present, `move` pressed by default |
| 17 | Preview-window measures | `javascript_tool` fiber-read of the preview window's measures prop, or the chrome's Show/Sun controls | Show select `generation3d-measure-show` (values `shaded`/`shaded+edges`/`wireframe`/`points` → action `setShowMode`, `…✏️edit/🪟️windows/👁️preview/🦀️.rs:40-51`); Sun group `generation3d-measure-sun` with toggle `generation3d-measure-sun-enabled` (→`toggleSun`) + sliders `-sun-azimuth` (0-360→`setSunAzimuth`), `-sun-elevation` (0-90→`setSunElevation`), `-sun-intensity` (→`setSunIntensity`) (`🔌️plugin/🦀️.rs:32461-32500`) | measure ids present in shellState/DOM |
| 18 | Debug status (not DOM — scene payload) | fiber-read the World3d scene's `statusJson` prop for the preview window | nested `debug: {evalLen, meshesLen, instancesLen, evalHead}` object (`…✏️edit/🪟️windows/👁️preview/🦀️.rs:68-83`) | parsed JSON fields non-empty once a fixture is loaded |

---

## 3. Examples

Picker: `NavbarExampleSelect` id `playground.navbar.fixture` (`🏛️ShellHost/🟦️.tsx:7658-7668`), its
`onValueChange` calls `dispatchActiveExample` → `onAction({..., action: "setActiveExample", args: {exampleId}})`
(`🏛️ShellHost/🟦️.tsx:7638-7649`). The 8 example ids (each a top-level `pub const ID` at line 5 of its
`🦀️.rs`, under `…✳️any/📚️examples/`) and expected geometry stats from `📓️example-geometry-tests-2026-09-09.md`
§2 (as corrected in its §8.1):

| Example id | Expected volume | BBox | `t` | `kernelStatus` |
|---|---|---|---|---|
| `rectangle-wire-preview` | — (edge-only; perimeter 2(w+h) = **7.0**) | 2 × 1.5 × 0 | 0.05 | green |
| `rectangle-extrude-volume` | **12.0** (w·h·d) | 2 × 2 × 3 | 0.05 | blocked-on-extrude-orientation |
| `face-sweep-extrude` | **12.0** (w·h·d) | 2 × 1.5 × 4 | 0.05 | blocked-on-extrude-orientation |
| `hexagonal-mushroom-column` | **3.897114317030** | z-span 6, xy between inradius/circumradius | 0.05 | blocked-on-extrude-orientation |
| `box-shell-preview` | **3.904** | 2 × 2 × 2 | 0.05 | green |
| `box-fillet-preview` | **7.888634924** | 2 × 2 × 2 | 0.01 | blocked-on-fillet-kernel |
| `sphere-box-fuse` | **7.245424930456** | 2.7³ (`[-1.2,-1.2,-1.2]`–`[1.5,1.5,1.5]`) | 0.01 | blocked-on-boolean-kernel |
| `sphere-cut-with-torus` | **37.841142613316** | 4.4³ | 0.01 | blocked-on-boolean-kernel |

| # | Step | How | Expected | Evidence |
|---|---|---|---|---|
| 19 | Cycle all 8 examples | select each option in `#playground.navbar.fixture` in turn | scene rebuilds (Flow + Preview windows), no `[DEBUG] action failed setActiveExample`; per-example numbers above should match once `blocked-on-*` kernel defects are fixed — a `blocked-on-*` row failing its expected volume today is a **known, already-diagnosed** kernel gap, not a new bug (see `📓️boolean-kernel-2026-09-09.md`, `📓️sweep-kernel-2026-09-09.md`, `📓️blend-kernel-2026-09-09.md`) | scene screenshot + fiber-read `statusJson.debug` (§2 step 18) per example |
| 20 | Cross-check bbox/volume | fiber-read the preview window's `statusJson`/mesh payload, or use the Inspection panel | fields align with the table (allow the kernel-status caveats above) | numeric readout |

Note: `demo-session` under `…✳️any/✏️editor/📚️examples/🎬️demo-session` is a 9th directory but is
deliberately excluded from `examples()` (`📓️example-geometry-tests-2026-09-09.md` §4) — do not expect it
in the picker.

---

## 4. Hover / selection

Interaction domain/channel/granularity constants (`✏️editor/🦀️.rs:1698-1707`): domain `"graph"`, hover
channel `"pointer"`, granularity `"handle"`. `PreviewInteractionMarks::from_interaction` (`✏️editor/🦀️.rs:1728-1733`)
reads `interaction.hover(domain, channel).ids` for hover and `interaction.selection(domain).ids` for
selection — both framework-owned (`InteractionView`), the app stores neither itself. Marks match at three
levels: bare widget id, `{widgetId}@{channel}`, `{widgetId}@{channel}#{index}` (`✏️editor/🦀️.rs:1735-1736`).

| # | Step | How | Expected | Evidence |
|---|---|---|---|---|
| 21 | Hover an instance | move pointer over a mesh instance in `#framework.window.proceduralPreview`'s canvas | `shellState.interaction` (fiber-read, §1) reports one hovered id under domain `graph`/channel `pointer` | fiber-read snapshot before/after |
| 22 | Click-select the same instance | click it | selection under domain `graph` carries the same widget id, `granularity="handle"` | fiber-read `interaction.selection("graph")` |
| 23 | Read the derived selection JSON | fiber-read the preview window's `statusJson`/selection payload (`preview_selection_json`, `✏️editor/🦀️.rs:1809-1825`) | `transformMode` = active utility (`move`/`rotate`/`scale`); `gumballActive` = true iff selection non-empty AND `transformMode` non-empty; `showEdges` true for `shaded+edges`/`wireframe`; `selectionMode`/`granularity` = `"mesh"` | parsed JSON fields |
| 24 | Transitive hover (Cluster) | hover a clustered/grouped node in the Flow window if one exists in the loaded example | every member instance in the 3D Preview highlights too (`hovered_graph_target`/transitive hover ids, `✏️editor/🦀️.rs:1763-1772`) | visual highlight across both windows |
| 25 | Clear selection | click empty space in the preview canvas | selection ids empty again; `gumballActive` false | fiber-read |

The viewer (read-only) side of this app duplicates the identical mechanism at
`…✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:115-122` (`Generation3dViewMarks::from_interaction`,
`📓️viewer-2026-09-09.md` §2.5) — the same fiber-read technique applies there if the viewer is reachable
from this playground.

---

## 5. Actions

`GENERATION3D_RETAINED_TOOL_IDS` (`✏️editor/🦀️.rs:249-277`) — **28** retained action ids (task brief said
29; actual count read from source is 28):

`setActiveExample`, `nodeGraphEdit`, `deleteSelection`, `removeWidget`, `moveMediaNode`, `addWidget`,
`patchFlowWidgets`, `reorganize`, `translateSelection`, `rotateSelection`, `scaleSelection`,
`addGeneration`, `removeGeneration`, `renameGeneration`, `updateGenerationValues`, `nodeGraphViewport`,
`setLodMode`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`,
`setCamera`, `selectGeneration`, `flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`,
`cancelPreviewEval`.

**Dispatch mechanism**: no dev-console dispatch hook exists in `🔌️PluginRuntime/🟦️.tsx` or
`🏛️ShellHost/🟦️.tsx` — both files were grepped this audit for `window.__`/`globalThis.`/`dispatchAction`
and none exists. The only sanctioned path is a real DOM control bound to `onAction`
(`{controllerId, action, args}`, built at `🏛️ShellHost/🟦️.tsx:7649`). To script an action id that has no
visible control yet, fall back to the §1 fiber-walk to reach the component instance holding `onAction`
and invoke it directly from the console — the puzzle3d ticket used the same class of technique (Playwright
`page.evaluate` + fiber walk) for its own scripted probes.

| # | Step | How | Expected | Evidence |
|---|---|---|---|---|
| 26 | `setActiveExample` | example picker (§3) | covered above | — |
| 27 | `setShowMode` | Show measure select (§2 step 17) | `config.show_mode` changes, mesh payload updates (edges/wireframe/points channels dropped per mode) | fiber-read `statusJson.debug` |
| 28 | `toggleSun` / `setSunAzimuth` / `setSunElevation` / `setSunIntensity` | Sun measure group (§2 step 17) | sun toggle flips; sliders move 0-360 / 0-90 / intensity range; scene lighting changes | fiber-read sun measures + visual |
| 29 | `translateSelection` / `rotateSelection` / `scaleSelection` | select an instance (§4), pick `move`/`rotate`/`scale` utility (§2 step 16), drag the gumball | one absolute-delta commit on gesture release (puzzle3d's documented gumball semantics: `transformBegin`/`transformEnd` are deliberate no-ops, the drag itself is host/canvas-local) | mesh pose updates once on release; no `[DEBUG] action failed` |
| 30 | Generate-mode actions | `selectGeneration`/`addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues` via the Generations/Form windows (§2 steps 11-12) | generations list mutates; Form reflects the selected generation's values | list/form DOM diff |
| 31 | Flow-graph actions | `nodeGraphEdit`/`nodeGraphViewport`/`addWidget`/`removeWidget`/`moveMediaNode`/`patchFlowWidgets`/`reorganize`/`deleteSelection` in the Flow window (§2 step 8) | graph mutates, no fault | fiber-read graph scene |
| 32 | Evaluation pipeline | `flowEvalTick`/`flowEvalResolve`/`flowTessellateResolve`/`cancelPreviewEval` | preview mesh recomputes; watch for the extension round-trip fix (`📓️extension-round-trip-2026-09-09.md`, landed 19:26) actually taking effect — this was the root cause of "no 3d preview" earlier in this ticket | console (`[DEBUG] extension invocation completed`, `🏛️ShellHost/🟦️.tsx:1556`) + preview mesh changes |
| 33 | `setCamera` | drag/orbit the 3D preview | camera state persists across a re-render | fiber-read camera JSON |

Evidence baseline for every action above: no new `[DEBUG] action failed <id>` (`🏛️ShellHost/🟦️.tsx:5477`)
after dispatch, and the relevant `shellState` slice changed.

---

## 6. Pane pitfalls

The exact "hidden pane → resize; worker eprintln; console buffer survives reload" trio is not verbatim
in `📓️boot-path-audit-2026-09-09.md` §6 (that section is a build/serve recipe, not pane-driving notes) —
sourced instead from the puzzle3d runtime-verification doc's own header and repo memory, cited per line.

| # | Step | How | Expected | Evidence |
|---|---|---|---|---|
| 34 | Hidden-pane driving | `resize_window` 1440×900 once at the start (§1 step 1); the pane can then stay hidden for the rest of the session | tool calls succeed without needing the pane fronted (puzzle3d ran its whole session this way — `…PUZZLE-3D-END-TO-END/📓️2026-09-09-runtime-verification.md:4-5`: "Browser pane emulated at 1440×900 (the pane itself stays hidden)") | no viewport-related failures |
| 35 | Worker `eprintln!` visibility | never rely on page-injected `console.*` hooks alone for reactor/trap traces (§1 step 5) | those lines appear ONLY via `read_console_messages` (worker stdio capture) — confirmed this audit: `trapped`/`more-work` are absent from both `🏛️ShellHost/🟦️.tsx` and `🔌️PluginRuntime/🟦️.tsx` | `read_console_messages` output |
| 36 | Console buffer persistence | after any `navigate` reload, note the timestamp of your last read before assuming a fresh boot's messages | `read_console_messages` keeps prior-load entries in its buffer (repo memory: "Browser Console Buffer Survives Reload") — a stale trap from an earlier boot can still be sitting there; don't mistake it for a new one | compare message ordering/timestamps across the reload boundary |
| 37 | Build freshness | before trusting any runtime observation, confirm the served wasm is the one you expect | port-listening is not proof of a rebuild (`📓️boot-path-audit-2026-09-09.md:419-420`) — `strings <staged .wasm> \| grep <a string unique to your target commit>` | grep hit |
