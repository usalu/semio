# 🛰️ wgpu DOCK LAYOUT → WORLD3D — every window of the active mode is laid out, and the preview surface exists

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu dock layout + World3d", 2026-09-12.
Resumes `📓️wgpu-command-frames-mesh-chain-2026-09-12.md` §8 (the one named remaining hop).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened — bookkeeping is on disk. Evidence under
`🗑️generated/wgpu-dock/run-1 … run-10`, `edit`, `edit-final`, `generate`, `generate-2`,
`generate-3`, `viewer`; nothing under any `🗑️generated` folder was swept. The react serve on 6018 was
not touched, the procedural guest was NOT rebuilt or restaged, and no git-state-modifying command was
run.

---

## 1. TL;DR

| question | answer |
|---|---|
| **1 — does the wgpu dock apply the app's authored mode layout?** | **Yes.** `http://127.0.0.1:6118/?plugin=generation3d` lays out **two** windows at the authored `[68, 32]` row split — `procedural-main@975×814+3 procedural-preview@459×814+978` of a `1434×836` canvas — each with its own dock tab chrome, its own arena subtree and its own viewport (§4.1). Generate mode's named `[22, 43, 35]` layout and the viewer role's single full-pane stack are proven the same way (§4.3, §4.4). |
| **1 — what was actually wrong?** | **The wgpu shell never consulted the dock at all.** `DockState` — layout, tab chrome, splitters, drop zones, silhouettes — was complete and had **zero production call sites**: `paint_chrome`, `register_hits`, `register_resize_hits`, `stack_body_rects_with_silhouettes` and `stack_corner_tab_bar_rects` were never called, and `dock_drop_bodies`/`dock_drop_tab_bars`/`dock_canvas_bounds`/`window_content_rects`/`window_silhouettes` had no writer outside a test. `render_main_window_step` painted `active_window_id` alone over the whole body (§2). |
| **1 — shared law?** | One language-neutral fixture (`🧫️fixtures/🪟️app-mode-layouts/🔣️.json`, 6 cases), a Rust law driving the real `DockState` (**4/4**) and a TypeScript twin driving React's own `resolveLayoutForMode`/`resolveFrameworkLayoutSeed` on the same fixture (**6/6**) — both run, both quoted (§5). |
| **2 — does the `procedural-preview` World3d engine surface exist?** | **Yes.** `world3d_states` now gets its `procedural-preview` entry, the surface paints its own scene pass into the preview window's draw list, and the window-transient lanes reach it fully merged: `lanes=[meshes:593, instances:473, selection:172, environment:92, status:659]`, all byte-exact (§4.2). |
| **2 — is the hexagonal column on it?** | **No, and it is not claimed.** The chain's 6 round trips deliver, and the guest's own status says why: at the moment the shell renders the preview the job is at **`phase: "meshingFaces", facesDone: 6/8, inFlight: 1`**, and the payload the surface receives is the WIRE preview only (`eval-profile@wire#0` with empty `positions`/`indices`, `eval-extrusion-axis@vector#0`). No solid has been published yet. The remaining hop is named with console evidence and a measured failed attempt at closing it in §6. |
| Defects fixed on the way | **Three**, each measured: the dock had no call site (§2); one window body whose retained paint never terminates held the ENTIRE chrome frame — and with it the navbar, the panels and the GPU present — hostage, which is what a blank canvas with a correct dock plan looks like (§3.2); and `dumpStructure`/`dumpFrameStats` could not answer for anything but the largest window, so the smaller pane of a two-pane layout was unmeasurable by construction (§3.1). |
| Also added | `?mode=` on the wgpu boot beside the existing `?role=`, and the role PROJECTION that `?role=viewer` needed to reach `…#viewer` at all (§3.3). |

---

## 2. Deliverable 1 — the defect, root-caused

`📓️wgpu-command-frames-mesh-chain` §8 reads: "`refresh_ui` renders the preview window's document, but
`self.dock.root` places only `procedural-play-main`". The first half is right, the second is one level
off. `self.dock.root` was always correct — `DockState::from_app` has read `app.default_layout` through
`dock_from_window_layout` since it was written, and `refresh_ui` walks `self.dock.window_instances()`,
which is why BOTH surfaces rendered every round.

What did not exist is the **consumer**. `ShellState::render_main_window_step`
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) resolved exactly one window —

```rust
let window = if self.space_mode && self.spawned_ui.is_some() { UiText::try_from_str("spawned") }
             else { self.active_window_id.as_deref().and_then(UiText::try_from_str) };
```

— and painted its document into `bounds.inset(theme.panel_inset)`, the whole body. Everything the dock
knows was unreachable from there. Grepped across the crate before any edit:

| dock entry | production callers before |
|---|---|
| `DockState::stack_body_rects` / `stack_body_rects_with_silhouettes` | **0** |
| `DockState::stack_corner_tab_bar_rects` | **0** |
| `DockState::paint_chrome` | **0** |
| `DockState::register_hits` / `register_resize_hits` | **0** |
| `ShellState::window_content_rects` | **0** writers (one test) |
| `ShellState::window_silhouettes` | **0** writers |
| `ShellState::dock_drop_bodies` / `dock_drop_tab_bars` / `dock_canvas_bounds` | **0** writers, several readers (the drag/resize handlers, which could therefore never fire) |

So the second window of a two-window mode had no rect, and with no rect it had no document paint, no
arena subtree, and no `render_component_scene_step` — which is the only thing that creates a
`World3dState` in `world3d_states`. That is the whole of §8's "no World3d engine surface exists".

---

## 3. Deliverable 1 — the fix, at the owning layer

### 3.1 The chrome walk places every window the dock names

`render_main_window_step` becomes a dock walk with four phases and a per-window cursor:

| phase | one unit |
|---|---|
| `0` | body background, `cursor.rect = bounds.inset(panel_inset)` |
| `1` | **`plan_dock_windows(rect, theme, atlas)`** — solve the layout once for this frame |
| `2` | one scissor over the whole dock canvas |
| `3` | next window: its hit region, its `canvas_clear` ground, its `UiText` identity |
| `4` | that window's document paint, stepped, then `cursor.item += 1` back to `3` |
| `5` | `dock.paint_chrome(ctx, canvas, body_fill = false)` + `dock.register_resize_hits` — tab caps, tab actions, splitters, join corners, all on top of the bodies |
| `6`/`7` | pop the scissor, done |

`plan_dock_windows` is the single writer the five registries never had: it fills `dock_canvas_bounds`,
`dock_drop_bodies`, `dock_drop_tab_bars`, `window_content_rects`, `window_silhouettes` and the new
ordered `dock_window_plan`. `dock_chrome_maps` resolves each INSTANCE's tab label and icon off the
app manifest's own `LocalizedLabel`/`IconName` through the live terminology and locale — no shell
dictionary, no default language, the rule React's `withLocalizedWindowKindLabels` already follows.

`paint_chrome` is called with `body_fill = false` deliberately: the bodies are painted by the loop
above it, so the dock only owns the chrome. `register_hits` is not called at all — it is
`paint_chrome(body_fill = true)` under another name, and calling both would double-paint every tab.

A spawned studio surface is the one non-dock case and stays a single full-bounds pane.

### 3.2 One window body must not be able to hold the frame hostage

Placing every window immediately exposed a second defect that a single-window walk structurally could
not show. On `?plugin=generation3d&mode=generate` the dock plan was correct and the arena was
populated, and **the canvas stayed uniformly blank — no navbar, no panels, nothing**
(`🗑️generated/wgpu-dock/generate-2/shot-015s.png`, `shot-030s.png`, `shot-045s.png`, all three).

The cause: `generation3d-generate-form`'s retained document paint never terminates. The walk's
contract is "advance on complete or fault", so it returned `false` forever, the chrome cursor never
reached `Navbar`/`Footer`/`Overlay`, and no frame ever presented.

`SHELL_WINDOW_PAINT_OPPORTUNITIES = 1 << 20` bounds it: a body that has not finished within its own
budget is recorded as that surface's paint fault and the walk moves to the next window — the same
"one surface's failure is that surface's failure, never the renderer's" rule
`ShellSurfaceFault` already states. Measured, `🗑️generated/wgpu-dock/generate-3/console.txt`:

```
19578  [DEBUG] wgpu-shell window paint generation3d-generate-form exhausted 1048576 opportunities
38662  [DEBUG] wgpu-shell window paint generation3d-generations exhausted 1048576 opportunities
```

and with it, generate mode paints (§4.3). **The non-terminating `Form` body itself is NOT fixed here**
— it is named, bounded and left for its owner; §6.3.

### 3.3 The two boot axes, and the role projection `?role=viewer` was missing

`dumpStructure()`/`dumpFrameStats()` were zero-arg and picked "the window with the largest last-known
viewport area". With one window that is total; with a two-pane mode layout the smaller pane was
**unmeasurable by construction** — the exact situation this lane had to measure. Both exports now take
an optional `windowId` (`dump_window_id`), and both dumps carry `windowId` + `windowIds` so a caller
can see what else it could have asked for. The plumbing follows the existing introspection seam:
`BrowserFrameWorkerIntrospect.windowId` → `transport.introspect(probe, windowId)` →
`bindings.dumpStructure(windowId)`.

`UiFramePaintCensus` gained `scene_passes`/`scene_draws`/`scene_instances`, and a layer a scene pass
targets now counts as non-empty. A `World3d` window's entire output is `ScenePass3d`s — instanced mesh
draws, translucent draws, textured draws, the grid's line draws — and none of it is a `UiInstance`, so
a census counting only quads answered `0` for a surface that was drawing. Same shape as
`📓️wgpu-document-reconcile` §3.4, one layer down.

**`?role=`**: `semioWgpuSetAppRole` already reached Rust and `boot_app_role()` already answered — with
**zero readers**. The registry maps every variant to an `#editor` app id (`generation3d →
s.procedural.generation3d@1/*#editor`), so a viewer boot names the right DIALECT and the wrong role.
`select_boot_program` now takes the boot role and `project_boot_app_role` resolves the sibling surface
of the anchor's dialect — the Rust twin of React's `resolveBootPrimaryAppV1`, rule for rule: a manifest
that declares the sibling wins, one that does not keeps the anchor (a viewer request on an editor-only
artifact is a downgrade to the surface that exists, never a dead boot).

**`?mode=`**: new, the second axis. `semioWgpuSetBootMode` mirrors `semioWgpuSetAppRole` exactly
(`thread_local`, set once from the boot descriptor); `ShellState::apply_boot_mode` sets the session's
`active_mode_id` and installs `resolve_layout_for_mode`'s answer as the `layout_override` — the same
two lines `handle_control_command`'s `playground.navbar.modes.*` arm runs for a click. A mode the open
app does not declare is logged and ignored. This is what makes generate mode reachable on 6118 at all:
the wgpu navbar has no mode group yet (§6.4).

---

## 4. What 6118 does now, measured

`🐍️wgpu-dock-probe.mjs` (new) reads the dock: per-window `dumpStructure`/`dumpFrameStats` for every
window id it is given, the engine scene nodes and their rects, the surfaces the shell says it rendered,
and it classifies the console itself so a run's verdict is the log rather than a reading of it.

### 4.1 Edit mode — the authored `[68, 32]` row split (`🗑️generated/wgpu-dock/edit-final`)

```
[DEBUG] wgpu-shell dock plan canvas=1434x836 windows=2
        procedural-main@975x814+3   procedural-preview@459x814+978
```

975/1434 = 0.680, 459/1434 = 0.320 — the fractions `…/🎭️modes/✏️edit/🦀️.rs:14-16` authors.

```
procedural-main     nodes 34  {drawCalls: 2, quadCount: 594, glyphCount: 549}
                    scene  stack[0]#procedural-play-main.body/stack[1]#…canvas/componentScene[0]#procedural-main
                           rect [497.0, 0, 477.8, 813.6]
procedural-preview  nodes  1  viewport 458.75 × 813.6
                    scene  componentScene[0]#procedural-preview  rect [0, 0, 458.75, 813.6]
windowIds           ["procedural-main", "procedural-preview"]     ← two engine windows, was one
alert               null
```

`edit-final/shot-035s.png` shows it: two dock windows with their own tab chrome (`Flow` and `Preview`,
each with focus/new/close controls), the Flow window carrying the outline tree (`Vector — Evaluated`,
`ExtrudeCurve — Evaluated`, `Preview — Evaluated`, all seven nodes, all six wires) and the node-graph
canvas with its value chips `0.5 / 6.0 / 6.0` and its wires, and the Preview window carrying the
World3d LOD grid.

Baseline for contrast — the same probe, the previous build (`📓️wgpu-command-frames-mesh-chain` §8):
34 nodes, **every one** under `procedural-play-main.body`, `dumpFrameStats` answering for
`procedural-main` only, and no node under `procedural-preview` at all.

### 4.2 The World3d engine surface for `procedural-preview` (`edit-final`, `run-5`)

```
[DEBUG] world3d surface=procedural-preview pane=Some("procedural.play.preview") bounds=459x814
        draws=0 translucent=0 instances=0 lines=1 textured=0 passes=1
        meshes=593b instances=473b delta=0b
        lanes=[meshes:593, instances:473, selection:172, environment:92, status:659]
        snapshot=false camera=80b
```

The surface EXISTS (`world3d_states` has its entry, `register_engine_surface` recorded it, a
`ScenePass3d` was pushed into the preview window's own draw list), its bounds are the dock's, and every
declared window-transient lane arrived at its declared byte length and merged. `lines=1` is the LOD
grid, visible in the screenshots.

### 4.3 Generate mode — the named `[22, 43, 35]` layout (`🗑️generated/wgpu-dock/generate-3`)

`http://127.0.0.1:6118/?plugin=generation3d&mode=generate`:

```
[DEBUG] wgpu-shell boot: program=procedural app=s.procedural.generation3d@1/*#editor mode=Some("generate")
[DEBUG] wgpu-shell dock plan canvas=1434x836 windows=3
        generation3d-generations@315x814+3
        generation3d-generate-form@616x814+319
        generation3d-generate-preview@502x814+935
```

315/1434 = 0.220, 616/1434 = 0.430, 502/1434 = 0.350 — the fractions
`…/🎭️modes/🧬️generate/🦀️.rs:21-33` authors, through the NAMED layout `generation3d-generate` its
mode's `layout_id` points at. `generate-3/shot-040s.png` shows three dock windows: `Generations`
(with its `(no generations)` row and its `Add Generation` action), `Form`, and `Preview` with the
World3d grid. All three have engine windows
(`windowIds: [generation3d-generations, generation3d-generate-form, generation3d-generate-preview]`).

### 4.4 Viewer role — one full-pane stack (`🗑️generated/wgpu-dock/viewer`)

`http://127.0.0.1:6118/?plugin=generation3d&role=viewer`:

```
[DEBUG] wgpu-shell boot: program=procedural app=s.procedural.generation3d@1/*#viewer mode=Some("view")
[DEBUG] wgpu-shell dock plan canvas=1434x836 windows=1 procedural-view-preview@1434x814+3
[DEBUG] world3d surface=procedural-view-preview pane=Some("procedural.view.preview") bounds=1434x814
        lanes=[meshes:593, instances:473, …]
```

`viewer/shot-070s.png` shows the title chip `s.procedural.generation3d@1/*#viewer`, one full-width
`Preview` window with its dock tab chrome, and the World3d grid. Before the role projection of §3.3
this url booted the EDITOR app — the viewer surface was unreachable on this target.

---

## 5. The shared law, both halves run, verbatim

### 5.1 The fixture

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🪟️app-mode-layouts/🔣️.json`
— one canvas (`1000 × 800`) and six cases, each declaring an app (modes with `layoutId`, window kinds,
`defaultLayout`, `namedLayouts`), an active mode, and the expected resolution: the named layout's
identity, the root axis, the window INSTANCES in layout order, the focused window, and per stack its
path, its windows, its active window, its fraction and its solved rect. A `provenance` block names the
authoring site of each real layout it carries.

The six cases: generation3d edit (`row [68,32]`), generation3d generate (the named `row [22,43,35]`),
the viewer's single stack, a mode naming an undeclared layout (falls back to the default), two
INSTANCES of one window kind (`instanceId` — two laid-out windows, one extra instance), and a tabbed
stack with an explicit `activeWindowKindId` (two instances, one laid-out body, focus on the declared
one rather than the first).

### 5.2 Rust, driving the real `DockState`

`🧱️elements/🛰️Dock/🧪️tests/🪟️app-mode-layouts/🦀️.rs`, mounted from
`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`.

```
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- app_mode_layout
running 4 tests
test dock::app_mode_layout_tests::the_shared_fixture_declares_a_closed_layout_for_every_case ... ok
test dock::app_mode_layout_tests::a_mode_change_reseeds_the_dock_with_the_new_modes_windows ... ok
test dock::app_mode_layout_tests::every_window_instance_of_the_active_mode_is_laid_out_at_its_authored_fraction ... ok
test dock::app_mode_layout_tests::every_painted_body_is_its_solved_frame_minus_the_tab_cap ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 502 filtered out
```

Four laws: every window instance of the active mode is laid out at its authored fraction, with the
authored focus and the authored solved rect (driving `resolve_layout_for_mode` → `DockState::from_app`
→ `apply_layout_diff` → `stack_frame_rects`, all production); a mode change re-seeds (edit → generate
→ edit, asserting generate REPLACES edit's windows outright and edit restores them); every painted
body is its solved frame minus the tab cap and nothing else (driving the real
`stack_body_rects_with_silhouettes` against a real `Theme` + `FontAtlas`); and the fixture's own
arithmetic closes.

`DockState::stack_frame_rects` is new and is why the fixture can state geometry without also pinning a
theme metric: it is the pure axis arithmetic, and `stack_body_rects` is that minus each stack's cap.

### 5.3 The TypeScript twin, same fixture, React's own resolvers

`🧑‍🎨engine/🧪️tests/🪟️app-mode-layouts/🟦️.ts` (new, registered in the React `vitest.config.ts`).

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts 🧪️tests/🪟️app-mode-layouts/🟦️.ts
 Test Files  1 passed (1)
      Tests  6 passed (6)
```

Six laws over `resolveLayoutForMode` (`🎠️kernel`) and `resolveFrameworkLayoutSeed` (`🛠️ShellHelpers`):
the layout each mode names (named ahead of default), every window instance seeded in layout order with
`extraInstances` exactly the instance-id-bearing ones, each authored fraction carried into the seeded
tree, the declared rects solved from the declared fractions, the fixture's own closure, and the
edit → generate → edit re-seed. Every row also runs through an independent in-file oracle that
re-derives the same answer from the fixture's declarations, so a shipped regression cannot move both
sides together.

One note for whoever writes the next suite here: `🐚️Shell` and `🛠️ShellHelpers` are mutually recursive
modules (Shell calls `shellLabel` at its own module scope), so entering the pair through ShellHelpers
leaves Shell reading a half-initialised binding (`Cannot access '__vite_ssr_import_10__' before
initialization`). Every production entry reaches them through `🏛️ShellHost` → `🐚️Shell`; the suite now
does the same with one side-effect import, commented as the evaluation order it is.

### 5.4 The boot-descriptor oracle

The `?mode=` axis is pinned by the existing boot fixture
(`🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json`), which gains `appMode` on every row plus two
new rows (`?plugin=generation3d&mode=generate`, `?plugin=generation3d&role=viewer&mode=view`). Its
owning suite is the repo's cache-contracts test, which also packages the vscode extension and runs two
bundlers; `<ticket>/🔍️boot-descriptor-oracle.ts` runs the `bootDescriptor()` half of it on its own —
the same TypeScript AST extraction, the same bare `node:vm`, the same fixture:

```
$ bun 🔍️boot-descriptor-oracle.ts
boot-descriptor oracle: 7/7 fixture rows agree with the shipped bootDescriptor()
  "" -> {"pluginVariant":"s","appRole":"editor","appMode":""}
  "?plugin=note&role=viewer" -> {"pluginVariant":"note","appRole":"viewer","appMode":""}
  "?plugin=puzzle3d&role=unknown" -> {"pluginVariant":"puzzle3d","appRole":"editor","appMode":""}
  "?plugin=note&plugin=puzzle3d" -> {"pluginVariant":"note","appRole":"editor","appMode":""}
  "?hub=…&user=Fixture+User&dataDir=fixture" -> {…,"appRole":"editor","hub":{…},"appMode":""}
  "?plugin=generation3d&mode=generate" -> {"pluginVariant":"generation3d","appRole":"editor","appMode":"generate"}
  "?plugin=generation3d&role=viewer&mode=view" -> {"pluginVariant":"generation3d","appRole":"viewer","appMode":"view"}
```

The full cache-contracts suite was **not** run.

### 5.5 The neighbouring suites

```
$ bunx vitest run --config vitest.config.ts         # @semio-tech/framework-renderer-wgpu
 Test Files  1 failed | 11 passed (12)
      Tests  6 failed | 127 passed (133)
```

All six failures are in `🧪️tests/🧩️package-integration/🟦️.ts` and all six are the same
`ReferenceError: Bun is not defined` from `⚙️browser-build/🟦️.ts`'s `assertPinnedBunVersion` — that
suite needs the Bun runtime and vitest runs it on node. Nothing in it touches this lane's files, and
it is the same file `📓️wgpu-command-frames-mesh-chain` §6.4 already reported failing for its own
environmental reason. 127 passed.

---

## 6. What is NOT claimed, and exactly where it stands

### 6.1 The hexagonal column is not on the wgpu World3d surface

The surface now exists, is laid out, receives its window-transient lanes byte-exact and paints a scene
pass. What it receives is the WIRE preview, because that is all the guest has published:

```
meshesHead   [{"id":"eval-profile@wire#0","data":{"positions":[],"normals":[],"colors":[],"indices":[],
              "edgePositions":[0.5,0.0,0.0, 0.25,0.433…]}},
             {"id":"eval-extrusion-axis@vector#0","data":{"positions":[0,0,0, 0,0,6], …}}]
instancesHead [{"id":"profile@wire#0","meshId":"eval-profile@wire#0", …},
               {"id":"extrusion-axis@vector#0", …}]
status        {"phase":"meshingFaces","phaseLabel":{"en":"Meshing faces","de":"Flächen werden vernetzt"},
               "progress":{"unitsDone":24,"unitsTotal":44,"facesDone":6,"facesTotal":8,"inFlight":1,
                           "ratio":0.545},"cancellable":true,"cancelAction":"cancelPreviewEval", …}
```

The guest's own status is the answer: **6 of 8 faces meshed, 1 request in flight**, at the moment the
shell renders the preview for the last time. `extrude@solid` is not in `meshes_json` because it has
not been produced yet. `📓️wgpu-engine-surfaces-2026-09-10.md` §6.1's stated contract — "wire-only
previews still do not paint (`profile@wire`, `extrusion-axis@vector`) — the bridge drops them … the
solid does paint" — is therefore consistent with `draws=0`: there is no solid to drop or draw.

The chain itself is unchanged and still converges exactly as `📓️tick-arming-latch` §4 derives
(`edit-final`): 5 `flowEvalTick`s, 6 extension round trips (1 math evaluate, 2 brep evaluate,
3 brep tessellate), every one `status: ok`, the last carrying 2265 B, `invokeExtension faulted` 0,
`unknown tag 115` 0.

### 6.2 The hop that remains, and a measured attempt at closing it

**A guest's preview job is driven BY its own window render.** Rendering `procedural-preview` is what
arms the next `flowEvalTick` and each tick's extension answer arms one more. `settle_boot` does
`refresh_ui()` once and `flush_deferred_actions()` once, so the shell stops the moment the chain THAT
one render armed quiesces — at 6/8 faces. React has no equivalent hole because a mutation re-renders
the shell, which re-enters every window body and re-arms.

The obvious fix — a bounded `settle_ui_chain` that loops `refresh_ui` + `flush_deferred_actions` until
a render arms nothing new — was written, built, served and measured, and **it does not work today**:

```
run-6  [DEBUG] wgpu-shell ui chain settled after 2 round(s)
       [DEBUG] wgpu-shell surface fault surface=procedural-main       detail=retained document permit failed: Capacity
       [DEBUG] wgpu-shell surface fault surface=procedural-preview    detail=retained document permit failed: Capacity
       [DEBUG] wgpu-shell surface fault surface=framework.panel.*     detail=retained document permit failed: Capacity   (×4)
```

Three variants were measured, all on 6118 (`run-6` … `run-9`):

| variant | result |
|---|---|
| loop as written | round two fails **all six** surfaces on the resident-document permit |
| retire-then-drain instead of drain-then-retire inside `refresh_ui` | 8 Capacity faults — worse |
| round ≥ 1 re-renders **window bodies only** (a `UiDirtyScope::Partial` reading) | 4 Capacity faults, and the engine ends with **no windows at all** |

`UiResidentPermit::try_reserve` is a fixed process-wide aggregate and the six live documents of one
full refresh already sit near it; a second refresh cannot open its documents before the first set has
been fully retired, and `drain_retained_document_arenas` — driven to idle, 65 536 steps, both the
document registry and the `UiValue` pages — does not free enough. **All of it was reverted**; the
shell is byte-for-byte back on one refresh + one flush, `Capacity` count 0, and `run-10`/`edit-final`
are that build.

So the next lane's first move is **the resident-document budget**, not the chain: either
`UI_RESIDENT_AGGREGATE_BYTES` sized against a real six-surface refresh, or a retirement that a
convergence round can actually drive to completion without a frame in between. With that closed, the
settle loop above is ~20 lines and its shape is already measured. This is the same fixed-arena family
as `📓️wgpu-command-frames-mesh-chain` §8's `UI_VALUE_ADMISSION_SLOTS` residue, which is still present
here (`framework.panel.catalogue` transiently reports `headroom collections=0` and recovers).

### 6.3 `generation3d-generate-form`'s body paint does not terminate

Named in §3.2 and bounded there, not fixed. It burns its full 1 048 576-opportunity budget (~19 s of
frames) on every boot into generate mode and is then reported as that surface's paint fault.
`generation3d-generations` does the same on a later frame. Neither is in this lane's files; both are
now survivable instead of fatal.

### 6.4 No mode or role group in the wgpu navbar

`handle_control_command` has handled `playground.navbar.modes.<id>` and `shell.layout.<id>` all along;
`render_navbar_step` renders neither group, so the only way to reach a non-default mode on this target
is the `?mode=` url this lane added. The role axis is boot-time only for the same reason — React's
own role switcher runs through `switchToPluginApp`'s create→retire→publish→seed→refresh ladder
(`📓️role-switch-keyboard-2026-09-12.md` §2.3), which has no wgpu counterpart at all. Both are named
rather than guessed at.

### 6.5 A repaint flicker

`edit-final/shot-035s.png` shows the Flow body fully painted; `shot-070s.png`, a later frame of the
same run, shows the dock chrome with an empty Flow body while `dumpFrameStats` still reports
`drawCalls 2, quadCount 594, glyphCount 549` for it. A later frame publishes a paint that does not
reach the present. Observed, not diagnosed.

---

## 7. Checks and builds

```
$ cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown
warning: `semio-framework-os-renderer-wgpu` (lib) generated 24 warnings     # unchanged baseline
    Finished `dev` profile
```

The renderer wasm was rebuilt and the served bundle re-verified **nine** times; every probe run
checked that the served `_bg.wasm` carried the build under test (by a string marker unique to that
build) before it was trusted. Shared cargo build dir throughout,
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`, every build in the foreground. The wgpu serve on
6118 (`screen g3dwgpu`, trunk) was restarted by nobody — trunk's own watcher rebuilt it, and
`generate-browser-boot`/`generate-frame-worker` were re-run by hand whenever the TS side of the
introspection or boot seam changed.

### 7.1 The native Rust suites this lane touches

```
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- dock introspection wgpu_engine_surfaces render_entry wgpu_node_graph document_tree
test result: FAILED. 54 passed; 1 failed; 0 ignored; 0 measured; 451 filtered out
```

The one failure is `dock::tests::dock_stack_content_fills_full_bounds_through_one_silhouette_clip`
(`🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:247`, a file dated Sep 9 that this lane does not touch):
`clip.scissors.len()` is `Some(2)`, the test declares `Some(3)`. It drives `paint_chrome` →
`layout_stack_cap` → `stack_window_silhouette` → `content_clip_rects` → `ClipRegion::from_rects` over
`Theme::default()` + `FontAtlas::builtin()`, and **none of those six is edited here** — this lane's
only Dock change is the additive `stack_frame_rects`/`collect_stack_frames` and the new test mount.
Three scissors means the tab chip leaves a gap to the stack's right edge (body + chip + gap); two
means the two chips now span the full 600 px, i.e. `dock_tab_chip_width` grew — which is a theme or
font metric. `🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs` (11:06) and `📝️text/🦀️.rs` (11:14) were both
rewritten by peers today, ~7 hours before this lane's first edit. Attributed, not touched.

`cargo test -p semio-framework-os-renderer-wgpu --lib` with NO filter cannot be run to completion at
all: `async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`
aborts the whole binary with `panic in a destructor during cleanup` inside
`<semio_framework_os_infinite::world::WorldAssetFetchOwner as Drop>::drop` (SIGABRT). Also
pre-existing, also nothing this lane touches.

---

## 8. Observed, not mine

| red | peer work in flight |
|---|---|
| `semio-framework-os-kernel` — `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`: `let chains are only allowed in Rust 2024 or later`, `ArtifactStoreDecodedEditRetirement`/`mutation_meta_from_history_op_meta`/`conflict_from_history_conflict`/`CompositionPin` not found (8 errors) | the store composition-open lane, ~18:20–19:00. Blocked the trunk serve rebuild for one cycle; went green on its own, nothing reverted |
| `semio_framework::Viewport2d` no longer in the root (`⚙️EngineCanvas`, `🎞️Scenes`, `🕸️wgpu-node-graph` test) | a re-export move to `semio_framework_os_kernel`, live at 19:44 (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` mtime). Blocks the NATIVE `--lib` test build only; the **wasm32** target, which is what 6118 runs, is green throughout |

Nothing was reverted and nothing was fought.

**A peer is already on §6.2.** At 20:17 the serve rebuilt with a `settle_ui_chain` that is not this
lane's (reverted at 19:40) plus a new diagnostic this lane did not write:

```
[DEBUG] wgpu-shell ui chain settled after 1 round(s) resident-roots=6 resident-bytes=1311424/33554432
```

That number is worth carrying forward: the resident aggregate is at **1.3 MB of 33.5 MB**, so the
`retained document permit failed: Capacity` of §6.2 is the permit ledger's **item/root** limit, not
its byte budget — six resident roots is the ceiling, and a convergence round needs twelve. Confirmed
on that live build (`🗑️generated/wgpu-dock/edit-confirm-2`): this lane's dock plan and World3d surface
still hold on top of it — `dock plan canvas=1434x836 windows=2 procedural-main@975x814+3
procedural-preview@459x814+978`, `world3dSurface 3`, `alert null`, `paintFault 0`.

---

## 9. Files

**New**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🪟️app-mode-layouts/🔣️.json` — the shared fixture (§5.1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🪟️app-mode-layouts/🦀️.rs` — the Rust law (§5.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪟️app-mode-layouts/🟦️.ts` — the TS twin (§5.3).
- `<ticket>/🐍️wgpu-dock-probe.mjs` — the dock probe (§4).
- `<ticket>/🔍️boot-descriptor-oracle.ts` — the boot-descriptor oracle (§5.4).

**Changed**
- `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `plan_dock_windows`, `dock_chrome_maps`,
  `dock_window_plan`, the dock walk in `render_main_window_step`, `SHELL_WINDOW_PAINT_OPPORTUNITIES`,
  `select_boot_program`'s role parameter + `project_boot_app_role`, `Self::boot_app_role`,
  `apply_boot_mode` (§3).
- `…/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` — `stack_frame_rects` + `collect_stack_frames`; the new
  law's mount.
- `…/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `dump_window_id`/`dump_window_ids`, the optional
  `windowId` on both exports, `windowIds` + the three scene counters on the dumps (§3.3).
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` — `UiFramePaintCensus` counts scene
  passes, scene draws and scene instances, and a scene-pass layer is non-empty (§3.3).
- `…/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` — `world3d_surface_debug_log` (§4.2).
- `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — region `🎭️ModeBoot`: `BOOT_APP_MODE`,
  `semioWgpuSetBootMode`, `boot_app_mode()` (§3.3).
- `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` — `appMode` on the boot message; `windowId` on
  the introspect message and on `introspect()`.
- `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` — `semioWgpuSetBootMode`; the introspection hooks take a
  window id.
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` — `?mode=` in `bootDescriptor`; the introspection shim
  forwards a window id.
- `…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` + `…/🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json`
  — the boot role axis: the fixture app's `(dialect, role)` is parsed out of its id, four role cases
  added.
- `…/🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json` — `appMode` on every row, two `?mode=` rows.
- `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts` — registers the TS twin.

**Temporary logs left in the tree** — `[DEBUG] wgpu-shell dock plan …` (prints on change),
`[DEBUG] world3d surface=… draws=… meshesHead=… status=…`,
`[DEBUG] wgpu-shell window paint … exhausted …`, and `mode=…` on the existing
`[DEBUG] wgpu-shell boot:` line. They are the entire evidence trail of §4 and §6.1. The `windowIds`
and scene counters on the introspection dumps are **not** temporary — they are the only way to measure
a multi-window layout or a World3d surface at all, which is why §8 of the previous report could not
be checked.

No `launch.json` entry was added: `procedural3d-wgpu` already exists and is the row this lane ran, and
both new suites run inside existing targets (`@semio-tech/framework-renderer-react:test`,
`cargo test -p semio-framework-os-renderer-wgpu`).
