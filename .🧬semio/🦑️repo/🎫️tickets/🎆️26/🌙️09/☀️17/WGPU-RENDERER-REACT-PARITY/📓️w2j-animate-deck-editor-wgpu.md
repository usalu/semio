# W2j — Animate presentation-deck editor: wgpu parity + presentation-product scope

Packet **W2j (L)** of `26/09/17/WGPU-RENDERER-REACT-PARITY`. Reads
`📓️audit-plugin-react-only-surfaces.md` §2 rows 2 and 4, §7 `P-animate` / `P-presentation-scope`, and the
sibling packet `📓️w2f-cad-spatial-editor-wgpu.md` (same census-premise correction, one packet earlier).
Logs under `🗑️generated/w2j-*.txt`.

---

## 1. Census correction — what the animate "React-only surface" actually is

§2 row 2 sizes P-animate as "add a wgpu paint layer over an existing Rust engine (~7.5k React/CSS/TS
lines, 34 Rust engine files)". Verified against disk, both halves of that sentence describe two
**different, unrelated** things, and the React half is not an OS surface at all.

| Claim | Verdict | Evidence |
|---|---|---|
| `✏️editor/📺️renderer/⚛️react/🟦️.tsx` (5 559 lines) is the animate presentation-deck editor the OS shell renders | **No** | Its own header: *"React + reveal.js renderer for `@semio-tech/animate-presentation-core` declarative decks."* It mounts reveal.js (`mountPresentation`/`PresentationDeck`/`usePresentationInteraction`, auto-animate morph runs, slide-URL hash sync, pdf.js canvas port, markdown→HTML). Repo-wide consumers of those exports outside the file itself: its own vitest suite, `📖️stories/🎭️presentation-deck/🧪️.story.tsx`, and the **`🎤️presentation` product's own renderer** (§4). Zero references from `💻️os`, from `🗣️Interpreter`, from `🔌️PluginRuntime`, or from any `SurfaceKind` dispatch. |
| The animate deck editor has no wgpu path | **No** | The shell-facing animate editor is the Rust play app `crate::editor::animate` (`✏️editor/🦀️.rs`, 844 lines + `🎮️commands/*` 17 dirs + `📌️panels/*` 3 + `🎭️modes/🖊️main/🪟️windows/🖼️tile-editor` + `👥️presence` + `🎚️config` + `🗣️terminology`), plus a read-only sibling play app `👁️viewer/` with its own `🎭️modes/👁️view/🪟️windows/🖼️tile-editor`. Its single window is `surface_kind: SurfaceKind::Canvas2d` rendering a `Canvas2dScene`; its three panel tabs are plain `UiNode` trees. React paints that wire with `📐️Canvas2dHost`; wgpu paints the **same** wire with `render_canvas_2d_step` (Tier 1 of `render_component_scene_step`, closed by W1b). |
| The 34-file `⚙️engine` (rate/video/animation/text/scene/geometry/config/camera) is the deck editor's engine awaiting a paint layer | **No** | That engine is the Manim-class scene/keyframe/video core used at **render/export** time (`exportVideoFromDeck`), not by the editor window. The artifact's own doc says it outright: *"this artifact's persisted document has no time-based data at all — the Manim-class scene/keyframe engine under `✏️editor/⚙️engine` constructs its scenes in Rust code at render/export time, never from persisted document state"* (`🗿️artifacts/🎬️presentation/🦀️.rs`, `animation_child_handle` region). There is no timeline/keyframe UI on either target to reach parity between. |

So there is **no deck/slide list, no timeline/keyframe editor, no inspector** in the OS animate editor on
either target: the OS editor is a **tile editor** — one shared source figure plus named crop rectangles
over it — and both targets already paint it from one Rust wire. W2j is therefore not "build the wgpu
target beside the React one"; it is:

> find and close the divergences between the React and wgpu **canvas-2d hosts** in the exact wire this
> plugin emits, and decide the `🎤️presentation` product's scope.

### 1.1 What the OS animate editor is, precisely

| Surface | Wire | React painter | wgpu painter |
|---|---|---|---|
| `tile-editor` window (`animate.presentation.play`) | `Canvas2dScene { camera_x: 0, camera_y: 0, zoom: 1, layers_json }` — one `kind: "image"`/`"source"` backdrop layer (`data_url = source.src`) plus one `kind: "tile"` rect per crop, all scaled ×1000 | `📐️Canvas2dHost` | `render_canvas_2d_step` → `render_canvas_2d` |
| viewer `animate-view-tile-editor` (`animate.presentation.view`) | same shape, read-only (no engagement bar, no selection overlay) | `📐️Canvas2dHost` | same |
| `artifact` / `catalogue` / `details` panel tabs | `UiNode`/`UiTree` | `🗣️Interpreter` react | `🗣️Interpreter` wgpu |
| 17 commands, one interaction domain `"tiles"` (`HierarchyProvider::Flat`, granularity `tile`) | framework action bus | — | — |

---

## 2. Design

One engine, N paint targets: the parity work belongs in the **framework canvas-2d seam**, not in a new
animate-only wgpu file. Confirmed by three independent plugin readers of the same wire (§3.1) — a fix
there fixes animate, draw and layout at once, which is what makes this packet tractable.

Reused rather than rebuilt: `Canvas2dScene`/`scene_surface` (the wire), `render_canvas_2d` (the wgpu
painter), `ui_wgpu`'s `BoundedActionBatchReservation` (the action wire), the framework `"tiles"`
interaction domain (selection/hover), `🌳️Tree`/`PanelTreeBuilder` (the three panels). No new drawing
primitive was added.

---

## 3. Findings — React vs wgpu, in the wire animate emits

### 3.1 G1 (landed) — the canvas pointer wire was the wrong shape on wgpu

React's `📐️Canvas2dHost` gesture lane (`🟦️.tsx:426-446`) dispatches:

| action | React payload |
|---|---|
| `canvasPointerDown` | `{ x, y, button, shift, ctrl, meta, alt, width, height }` |
| `canvasPointerMove` | `{ x, y, width, height, samples: [[x,y], …] }` |
| `canvasPointerUp` | `{ x, y, shift, ctrl, meta, alt, width, height, cancelled }` |

`x`/`y` are **screen-logical pixels inside the canvas element**, `width`/`height` the logical viewport
size — that is the contract, and three plugins encode it independently:

- `🖍️draw` `CanvasPointerDown/Move/Up` declare `x, y, width, height, shift, ctrl, meta` (+ `cancelled`,
  + `samples`, + optional `world_x`/`world_y`) and call
  `canvas_point_to_world(&session.window_config.viewport, payload.x, payload.y, payload.width, payload.height)`.
- `📏️layout` `CanvasPointerDown` declares `surface_id, button, extend, x, y, width, height` and calls
  `hit_test_at(doc, cfg, payload.x, payload.y, payload.width, payload.height)`.
- `💡️reasoning` `CanvasPointerDown` declares `id, x, y`.

wgpu published instead `{ surfaceId, x, y, button, extend }` with `x`/`y` already converted to **world**
coordinates (`write_canvas_pointer_action`, `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1075-1102`), no
`width`/`height`, no `ctrl`/`meta`/`alt`, no `cancelled`, no `samples`. Consequences on wgpu before this
packet: `draw` converted world coordinates a second time against a 0×0 viewport (every press landed in
the wrong place), `layout`'s blueprint pick hit-tested against a 0×0 viewport (no row could ever match),
and `shift` never arrived under the name React uses.

**Landed** (see §5 I1): wgpu now publishes React's payload verbatim, keeps `surfaceId`/`extend` as
additive fields its own existing readers (`📏️layout`) already consume, and adds `worldX`/`worldY` as the
optional world lane `🖍️draw` already declares — nothing loses information, and every field React sends is
present under React's name and in React's coordinate space.

### 3.2 G2 (landed) — the empty-canvas placeholder

React draws `"Empty canvas"` inside the canvas when zero layers render (`🟦️.tsx:632-635`) and an
`ui.host.emptyScene` label when the scene itself is absent (`:942`). wgpu drew the absent-scene
placeholder (`render_placeholder`) but nothing for a present-but-empty layer list. Landed in §5 I2.

### 3.3 G3 (shared gap, NOT a wgpu-only fix) — no canvas layer picking on either target

Animate's `canvasPointerDown` payload is `{ layer_id: Option<String> }` and its handler selects that tile
through `interaction_select_effect` — but **neither host hit-tests canvas layers**, so `layer_id` is
always `None` and the canvas tile pick is dead on React and wgpu alike. React self-documents it:

> `/** @emoji 🖱️ No layer pick/selection is tracked at this level (Canvas2dScene carries only camera + layersJson) — hits/selection stay empty per surface convention until layer picking lands here. */`
> — `📐️Canvas2dHost/🟦️.tsx`, `onContextMenu`

Adding the hit test to wgpu alone would *create* divergence, so this packet deliberately does not. It
cannot be fixed inside the animate plugin either: the host owns the live camera (wgpu keeps it in
`SCENE_STATE`, React in `cameraRef`) and animate's `PresentationConfig` carries only `engagement_input`,
so the plugin cannot convert a screen point to the ×1000 world space its own layers live in, and no
`setCamera` reaches it. Needs a framework packet on both hosts — see §7 `P-canvas-layer-pick`.

### 3.4 G4 (shared gap) — URL-sourced raster layers never paint on wgpu

The tile-editor backdrop is `kind: "image"` with `data_url = source.src`, and
`default_figure_tile_source()` sets `src = "/🖼️bauteilbörse.png"` — a plain URL path, not a `data:` URI.

- React: `preloadImages` does `new Image(); image.src = key; await image.decode()`, so any URL the page
  can fetch paints.
- wgpu: `render_canvas_2d` filters `source.filter(|src| src.starts_with("data:"))`, and the layer below it
  (`admit_ui_image`) answers `UiImageAdmission::Deferred` for every non-`data:` source with the comment
  *"the host asset pipeline owns it"* — and **no such pipeline exists** in the os wgpu renderer (zero
  `UiImageAdmission::Deferred` consumers repo-wide outside that function and its own tests).

So any served or relative image in a canvas-2d layer silently does not paint on wgpu. Not landed here:
it needs an image-fetch/asset-transport seam in the renderer (the analogue of the mesh
`meshAssetTransportUrl` lane), which is a framework packet, not a plugin one — §7 `P-canvas-url-images`.
Note the animate default `src` is itself dangling (no `🖼️bauteilbörse.png` exists in the repo; the nearest
file is `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/♻️bauteilbörse.png`, a different name), so
today both targets happen to show nothing — the divergence bites the moment a real source is set through
`setSource`.

### 3.5 G5 (shared gap, target-neutral) — 12 of animate's 17 verbs are `BatchOnlyPendingRewrite`

`create_animate_presentation_app` classifies only `setActiveExample`, `engagementInput` and `noMutation`
as `InteractiveJobClassification::Migrated` (the three tool ids
`AnimatePresentationRetainedCommandJobFactory` publishes); the other fourteen —
`seedGrid`, `addTile`, `deleteTile`, `deleteSelection`, `renameTiles`, `patchTileCrops`, `setSource`,
`setFrame`, `clearTiles`, `engagementSubmit`, `resetGrid`, `canvasPointerDown`, `copyPrompt`,
`exportVideoFromDeck` — are `BatchOnlyPendingRewrite`. `ActionBus::register`/`register_once` refuse any
factory whose classification is not `Migrated`, and `validate_interactive_job_classification` rejects only
`Unclassified`, so these verbs pass catalog validation while having no interactive dispatch route. This is
identical on both targets (it is manifest data, not paint), so it is **not** a wgpu-parity item — but it
does mean the animate editor cannot be exercised end-to-end on *either* target beyond the three migrated
verbs, which caps how much of §6's table can be verified by interaction rather than by wire inspection.
Recorded for the owner of the verb-migration lane; not touched here.

---

## 4. `🎤️presentation` product scope — **web-only, like `📓️print`. Not an OS surface.**

Asked by §7 `P-presentation-scope`. Evidence, all from the product's own taxonomy/README/tickets:

| Evidence | Source |
|---|---|
| Declared responsibility is the model **and its React + reveal.js renderer** — no shell/surface role | `🧰️framework/🛍️products/🔣️.json`: `framework.product.presentation` → *"Provides the render-independent declarative presentation model and its React + reveal.js renderer for slide decks."* (cf. `framework.product.os`, the only member that names a shell/runtime) |
| The product ships **zero Rust** — no `SurfaceKind`, no `WindowKindDefinition`, no play app, nothing a wgpu target could paint | `find 🧰️framework/🛍️products/🎤️presentation -name '*.rs'` → 0 files (identical to `📓️print`: also 0) |
| The deliverable is a **website**, addressed by URL, driven by reveal.js keyboard/overview | README: *"The presentation is website under site `/{columnIndex}/{rowIndex}?chapter=…`"*, *"When `escape` is pressed, the overview is shown"* |
| Its only consumer is a **static vite site** outside the OS | README: *"Consumers such as `♻️mit-bestand/🎤️präsentation/📅️33.projektetage` declare their deck as a glob of slide files and mount it through the renderer"*; `bun nx run …projektetage:build   # its static site` |
| The restoration ticket explicitly **did not** restore the OS-side host | `26/09/06/RESTORE-REVEAL-JS-PRESENTATION-PRODUCT-FRAMEWORK` (closed): *"Not restored: the July Rust deck crate and playground host, which depended on deleted products."* Verification was *"the dev server renders the deck (39 sections …, no console errors)"* — a browser, not a shell pane. |
| The README's own renderer-independence line is about **future** targets, not an existing OS surface | *"The model knows nothing about the DOM; the renderer is one target among possible others."* |

**Decision: `🎤️presentation` (9 415 + 1 406 lines) is out of scope for wgpu parity**, the same category as
`📓️print`'s LaTeX/PDF output — a web product with no OS surface, no Rust, and no wire for a second target
to paint. The animate plugin's own 5 559-line reveal.js renderer (§1) is the **same lineage and the same
category**: the restoration ticket's summary says the product was restored *from* it. No packet is owed
for either, and no packet is listed below for them. If the deck viewer is ever wanted *inside* the OS,
that is a new product decision (a declarative deck `SurfaceKind` + a Rust deck play app), not a
paint-parity packet — which is why none is listed in §7.

---

## 5. Landed increments

### I1 — canvas-2d pointer wire, wgpu → React shape — **landed, green**

`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, region `🔖️SceneInput`. `write_canvas_pointer_action`'s five
positional arguments are replaced by one `CanvasPointerWire` value built from the surface rect and the
live `Viewport`, so the screen-logical and world coordinates can never be confused again at a call site:

| field | before (wgpu) | after (wgpu) | React |
|---|---|---|---|
| `x`, `y` | world coordinates | **screen-logical inside the surface rect** (`x - inner.x`) | screen-logical |
| `width`, `height` | absent | surface rect size | logical viewport size |
| `shift` | absent (`extend` only) | present | present |
| `ctrl`, `meta`, `alt` | absent | present (see gap below) | present |
| `button` | down and up | down only | down only |
| `cancelled` | absent | `false` on up | `false` on up (`true` on a host-closed gesture) |
| `samples` | absent | `[[x, y]]` on move | the whole coalesced batch on move |
| `worldX`, `worldY` | — | added (the optional world lane `🖍️draw` declares) | absent (React converts host-side) |
| `surfaceId`, `extend` | present | kept (additive; `📏️layout`'s `CanvasPointerDown` reads both) | absent |

Byte accounting moved to `canvas_pointer_action_bytes` over one `CANVAS_POINTER_WIRE_KEYS` list, so the
reservation and the written keys are the same list (`checked_action_string_bytes` charges keys and string
values only — numbers and booleans cost node slots, of which 256 are available per action).

Residual, deliberately not faked: `ctrl`/`meta`/`alt` are emitted as `false` because
`ui_wgpu::wgpu::UiEvent::PointerDown/Up/Scroll` carries no modifier fields at all and the Interpreter's
scene route passes `shift: false` verbatim (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1153-1157`) — the same
modifier-plumbing gap `📓️audit-plugin-react-only-surfaces.md` §5 already lists as cross-cutting #3. The
keys are now present under React's names, so closing that gap becomes a value change, not a wire change.

Test: `canvas2d_pointer_payload_is_react_shaped_screen_logical_with_a_world_lane`
(`🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`) drives a real
`UiCommand::Scene { event: PointerDown { x: 50, y: 70 } }` over a surface at `Rect(30, 40, 200, 120)` and
asserts `x == 20`, `y == 30`, `width == 200`, `height == 120`, `button == 0`, all five flags present, and
a world lane. It fails on the old wire by construction (the old `x` was the world coordinate).

The `#[cfg(test)]` mirror `🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs`'s `canvas_world_pointer_json` was
moved to the same shape in lockstep, so the fixture oracle and production agree.

### I2 — empty-canvas placeholder on wgpu — **landed, green**

`render_canvas_2d` now paints `CANVAS_2D_EMPTY_LABEL` (`"Empty canvas"`, the literal from
`📐️Canvas2dHost/🟦️.tsx:634`) at the surface centre minus React's own 36 px x-offset when no `role !=
"meta"` record renders — the emptiness predicate is the same `role !== "meta"` filter React counts.

Test: `empty_layer_list_paints_the_react_empty_canvas_label` (`🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs`)
renders `"[]"` and asserts the glyph-instance count equals the label's character count (the grid and
checkerboard chrome push vectors and solids, never glyphs, so the count is exact).

### I3 — animate retained-tool proof/factory contract drift — **landed, green**

`✏️editor/🦀️.rs`: the `bounded_first_step_tool_proofs!` row declared
`ToolExecutionContract::bounded_first_step(8_192, 64, 1, 65_536, 7_500)` while the registered factory's
`execution_contract()` returns `ToolExecutionContract::resumable(8_192, 64, 1, 65_536, 7_500, 1, 1)`.
`validate_tool_job_rows` compares the two for equality (`registration.contract == row.contract`), so
`typed_join` was **false** and `interactive-job.catalog-authority` rejected the WHOLE catalog — including
the three verbs that *are* migrated. The row now reads `contract: animate_presentation_retained_contract()`,
the one function the factory itself returns, so the two cannot drift again; only the proof declaration
changed, never the contract the bus stores, so runtime behaviour is untouched.

Verified by executing, not by reading: the same failures now report `typed_join=true` (27/27), and the
registry-backed tests advance past the catalog gate to the honest per-verb fault
`interactive-job.missing-factory: typed command 'addTile' has no exact controller/owner/factory/tool/schema
proof` — which is §3.5's `BatchOnlyPendingRewrite` gap, now proven by execution rather than inferred from
the manifest. `📓️audit-plugin-react-only-surfaces.md`'s §7 list gains nothing here; the animate verb
migration is an existing lane's work.

---

## 6. Per-feature status vs React — the animate OS surface

React reference = `📐️Canvas2dHost/🟦️.tsx` + `🗣️Interpreter/🟦️.tsx`; wgpu = `render_canvas_2d_step` +
`🗣️Interpreter/🎯️targets/🧊️wgpu`. "Wire" means the feature is decided in the Rust play app and both
targets read the same field.

| Feature | React | wgpu | Verdict |
|---|---|---|---|
| tile-editor window dispatch (`SurfaceKind::Canvas2d`) | `resolveComponentSceneHost("canvas-2d")` | Tier 1 `render_canvas_2d_step` | **parity** |
| viewer window (`animate-view-tile-editor`) | same host | same painter | **parity** |
| source-figure backdrop, `data:` source | `drawImage` after `image.decode()` | `push_raster_quad` after `queue_canvas_image_upload` | **parity** |
| source-figure backdrop, URL source | paints | **does not paint** | gap G4, §3.4 |
| `kind: "source"` placeholder rect (pdf / empty src) | bounds rect + label | bounds rect + label | **parity** |
| crop tiles (`kind: "tile"`) | bounds rect + name label | bounds rect + name label | **parity** |
| infinite grid chrome | `drawInfiniteCanvasGrid` | `draw_canvas_infinite_grid` | **parity** |
| selection ring/glow on a `selected` layer | amber glow + ring | same two literals (`CANVAS2D_SELECTION_*`) | **parity** (pre-existing) |
| empty-canvas label | `"Empty canvas"` | `"Empty canvas"` | **parity — I2** |
| absent-scene placeholder | `ui.host.emptyScene` label | `render_placeholder` | parity in kind, not in string (wgpu prints `"canvas-2d host"`) |
| `canvasPointerDown/Up/Move` payload | React's shape | React's shape | **parity — I1** |
| pointer modifiers (`ctrl`/`meta`/`alt`, real values) | real | always `false` | keys present, values missing — framework `UiEvent` gap |
| pointer-move coalescing (`samples` batch) | full batch per owed dispatch | one sample per event | wgpu is per-event by design; the field shape matches |
| camera pan / wheel zoom | local camera + debounced `setCamera` | local `SCENE_STATE` viewport + settled `setCamera` | **parity** (pre-existing) |
| canvas layer pick → `layerId` | **not implemented** | **not implemented** | gap G3, §3.3 — equal and broken |
| surface context menu | `requestContextMenu` with empty hits | nothing | `W2b` (`scene.menu` unreferenced in wgpu) |
| catalogue drag → `canvasDragOver`/`canvasDrop` | dispatched | not dispatched | animate declares neither verb; a framework gap for the plugins that do |
| `artifact` / `catalogue` / `details` panel trees | `🗣️Interpreter` react | `🗣️Interpreter` wgpu | **parity** (`UiNode`, covered by the interpreter audit) |
| 3 migrated verbs (`setActiveExample`, `engagementInput`, `noMutation`) | dispatch | dispatch | **parity — unblocked by I3** |
| 14 `BatchOnlyPendingRewrite` verbs | `interactive-job.missing-factory` | same | gap G5, §3.5 — target-neutral |
| deck / slide list, timeline, keyframes, inspector | **do not exist on this surface** | — | not a gap (§1: reveal.js web renderer, no OS surface) |

---

## 7. Remaining gaps / hand-offs

Ordered by what blocks animate end-to-end, not by size.

1. **`P-animate-verb-migration` (M, target-neutral, blocks everything else).** The 14
   `BatchOnlyPendingRewrite` verbs (§3.5) fault with `interactive-job.missing-factory` on both targets.
   Every document-mutating gesture in the animate editor is in that set. Needs the same treatment
   `📓️audit-plugin-react-only-surfaces.md`-adjacent plugins already had (per-verb retained factories,
   proofs, publication lanes) — not a paint packet.
2. **`P-animate-registryless-harness` (S, target-neutral).** 27 of the artifact crate's editor tests call
   `presentation_app()` (`artifact_app_laws::new_app`, no registry), whose catalog has zero action
   definitions, so `migrated` is empty and any declared tool proof is rejected outright
   (`generated_migrated=false`). Either those tests move to `presentation_app_with_registry()` or
   `new_app` must build its catalog from the manifest. Separately,
   `standards::…::io::mutations::binary::tests` has two tests that panic **in `Drop`**
   (`PresentationEnvelopeMaterializeRegistry reached Drop before every retained caller was closed`),
   which SIGABRTs the whole binary and reports every concurrently-running test as failed — that
   cascade is why the suite looks like 40+ failures instead of a handful.
3. **`P-canvas-url-images` (M, framework).** §3.4: `admit_ui_image` defers every non-`data:` source to a
   "host asset pipeline" that does not exist in the wgpu renderer, and `render_canvas_2d` filters those
   layers out. The mesh lane's `meshAssetTransportUrl` is the shape to copy. Until then no served or
   relative image paints on wgpu, in any plugin's canvas-2d surface.
4. **`P-canvas-layer-pick` (M, framework, BOTH targets).** §3.3: neither host hit-tests canvas layers, so
   every plugin whose pointer payload wants a picked id (`🎞️animate`'s `layerId`, `💡️reasoning`'s `id`)
   gets nothing. Must land on React and wgpu together or it becomes a new divergence; cannot be done in
   the plugin, which has no access to the live camera.
5. **`UiEvent` pointer modifiers (S, framework).** I1 put `ctrl`/`meta`/`alt` on the wire; they are
   constant `false` until `UiEvent::PointerDown/Up/Scroll` carries modifiers. Same item as
   `📓️audit-plugin-react-only-surfaces.md` §5 cross-cutting #3.
6. **Absent-scene placeholder string (XS).** wgpu prints `"canvas-2d host"` where React prints the
   localized `ui.host.emptyScene` label. Cosmetic, and the wgpu side has no label lookup at that seam.
7. **No packet for the reveal.js renderers.** §1 and §4: the animate plugin's 5 559-line React deck
   renderer and the `🎤️presentation` product's 9 415-line one are the same web-only lineage as
   `📓️print`'s PDF output — no Rust, no `SurfaceKind`, no OS consumer, addressed as a website. They are
   removed from this ticket's scope rather than deferred.

### Descriptor / registry consistency

No plugin manifest surface changed (I3 touched a test-visible proof declaration only, not an action,
window, panel or interaction definition), so `✏️s/🔌️plugins/🎞️animate/🔣️.json` and `🛂️.descriptor.semio`
stay valid; the regeneration target is `nx run @semio-tech/animate-plugin:describe`
(`📦️packages/🦀️rust/📜️script.ts`'s `DescribeScript` → `describePluginComponent`). The descriptor already
lists all 35 editor-window actions and the `resetGrid` app command, matching the manifest.

---

## 8. Verification

All foreground, `-j 4`, plain env. Logs under `🗑️generated/w2j-*.txt`.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **exit 0**, reached the crate (`semio-framework-os-renderer-wgpu (lib) generated 53 warnings`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 canvas2d_pointer` | **2 passed** (the new law + the pre-existing dispatch test) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 interpreter::ui_command_wiring_tests` | **20 passed, 0 failed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 scenes::canvas -- --test-threads=1` | **6 passed, 0 failed**, including I2's `empty_layer_list_paints_the_react_empty_canvas_label`. Serial matters: `canvas2d_pan_drag_…` and `scene_camera_deadlines_saturate_…` share the `SCENE_CAMERA_DISPATCH_DEADLINES_MS` thread-local and flake against each other in parallel (pre-existing; each passes alone) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 scenes:: -- --test-threads=1` | 141 passed, **2 failed, both peer-owned**: `production_action_ingress_…` asserts `text_editor_apply_key` is test-only in `⚙️EngineCanvas`'s wgpu target, and `virtual_file_system_tests::chevron_press_…`; `git status` shows both files under concurrent edit by other agents, and neither is on any path this packet touched |
| `cargo check -p semio-s-plugin-animate -j 4` | **exit 0**, reached `semio-s-plugin-animate` |
| `cargo check -p semio-s-plugin-animate --target wasm32-wasip2 -j 4` | **exit 0**, reached `semio-s-plugin-animate` |
| `cargo test -p semio-s-plugin-animate -j 4` | **3 passed, 0 failed** |
| `cargo test -p semio-s-artifact-animate-presentation -j 4 --lib editor::` | 152 passed, 40 failed — all pre-existing, diagnosed in §7 item 2; I3 changed every `typed_join=false` to `typed_join=true` |

Notes for whoever re-runs this: the workspace disk hit 100% mid-packet and cargo failed with
`No space left on device`; pruning `⚡️cache/nx` entries older than 12 h freed 42 GB (the incremental
prune from `📓️`-memory's recipe freed nothing — every session was younger than two hours under the live
fleet). Two separate peer edits to `semio-framework-ui` and `🖼️IconRenderHost` broke the shared build
mid-run; both cleared on a re-poll, which is the expected concurrent-workspace behaviour, not a defect
in this packet.
