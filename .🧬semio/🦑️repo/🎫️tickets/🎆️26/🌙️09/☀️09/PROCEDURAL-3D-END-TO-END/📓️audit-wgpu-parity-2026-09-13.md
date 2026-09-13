# wgpu ↔ React Parity Audit — Beyond the Two Live Lanes (2026-09-13)

Read-only audit, lane `audit-wgpu-parity`. No source file was edited, no build/server started, no git
state changed. Sources: direct reading of current code as of this writing, cross-checked against (and
in several places correcting) the 2026-09-12/13 prior reports in this ticket folder. Two other lanes are
live today on `http://127.0.0.1:6118/?plugin=generation3d`: **input/hit runtime**
(`📓️wgpu-server-input-present-2026-09-13.md`, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`,
`📓️wgpu-retained-hit-registry-2026-09-13.md`) and **chrome parity**
(`📓️wgpu-chrome-parity-2026-09-13.md` — example picker, mode/role groups, preview cancel, `Fit graph`).
This report does **not** re-report anything those five own; it goes one level deeper or sideways from
each, per the brief's seven questions.

Abbreviations: `reconcile.rs` = `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`;
`paint.rs` = `…/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`; `events.rs` = `…/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`;
`wgpu-shell.rs` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`;
`renderer.rs` = `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (the product-level renderer, distinct from the shared
`🧑‍🎨engine`); `engine-canvas.rs` = `…/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`;
`scenes.rs` = `…/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`; `dag.rs` =
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`; `world.rs` =
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`.

---

## 0. Ranked gaps, highest user impact first

| # | gap | owning file(s) | law/probe to prove it |
|---|---|---|---|
| 1 | **Editing a Form field never reaches the guest** — typing into a generation's Input/NumberStepper is locally real (caret, IME, clipboard) but two divergent commit systems exist and it is unconfirmed either one fires from a *retained-document* click; the retained EventRouter explicitly documents no commit at all | `events.rs:719-726`, `renderer.rs` `commit_focused_input`+`stepper_metas` (~line 7402) | a browser probe: click a Generation's numeric field via the retained-hit-registry's `HitKind::Input`, type a digit, blur, and grep console for `on_absolute`/`on_change` dispatch — first proof needed once input-routing lands |
| 2 | **Wire (edge) creation/deletion on the Flow canvas is dead code from wgpu's side** — `DagHost::pointer_down_screen`'s port/handle hit-test and `DrawEdge` interaction mode exist and are reachable in principle, but wgpu's dispatch chain calls only `plan_pointer`/`commit_pointer` (`Idle\|Pan\|Drag\|Select`), never `pointer_down_screen` | `engine-canvas.rs:2747,2773,2786` (`node_graph_pointer_{down,move,up}_into`) vs `dag.rs:5054-5198` | native law over `GraphHost`/`FlowHost::plan_pointer` asserting it never enters `InteractionMode::DrawEdge`; browser probe: drag from a node's output port and assert no `pending_edge` preview paints |
| 3 | **Accessibility metadata is parsed but dropped** — `AccessibilitySpec` (label/description/live-region/shortcut/hidden) is a first-class field on every `UiNodeRecord`, React turns it into real ARIA attributes, wgpu's `reconcile.rs`/`paint.rs` never reference it (0 hits) | contract: `🧬️contract/♿️accessibility/🦀️.rs`; gap: `reconcile.rs`, `🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | a fixture-driven law comparing `AccessibilitySpec` fields against whatever wgpu surfaces today (answer: nothing) — proves by absence |
| 4 | **World3d compute status has no progress UI** — only a bare cancel "x" exists; phase/phaseLabel/ratio/unitsDone/facesDone/spinner are never read, unlike React's full `WorldComputeStatusPane` | `engine-canvas.rs:1780`, `wgpu-shell.rs:9282-9299` (`world3d_cancel_affordance`); `world.rs` never references `status_json` | fixture already exists (`🛑️surface-controls/🔣️.json`, today's chrome lane) — extend it with `phase`/`ratio` rows and assert a progress element paints |
| 5 | **Minimap and node/edge captions paint correctly on wgpu (`DagHost::paint_scene` is shared and IS called), but the minimap's own click-to-navigate is unconfirmed reachable** — no direct evidence either way was found; `minimap_widget_pointer_hit` is called only from inside `dag.rs`'s own `pointer_down_screen`/`pointer_move` (same method as gap #2), so it inherits gap #2's dispatch problem rather than being a separate gap | `dag.rs:3575,5061,5166` | once gap #2 is closed, a probe click inside the minimap panel rect and assert the main viewport re-centers |
| 6 | **Toggle/Slider/NumberStepper/Ring/IconSelect share Input's non-commit** — only `Button`, `Select`, and a `Stack` with `activate` are "concretely-wired" per the code's own docstring; the rest paint live state but a press updates nothing server-side | `events.rs:719-726,288` | per-kind fixture asserting a `PointerUp` on each kind produces the kind's own guest action (`on_change`/`toggle`/etc.) — today all but Button/Select/Stack fail it |
| 7 | **No `?example=` boot query** — but this is **not** a wgpu-only gap: React has no query-time example override either (`VITE_SEMIO_DEFAULT_EXAMPLE` is build-time-only). The real parity gap is the clickable picker, already fully owned by today's chrome-parity lane and closed at the source level (not yet runtime-proven, blocked on an unrelated build failure per that report's §6) | n/a — re-scoped, not a new finding | — |
| 8 | **Shell chrome i18n has real gaps outside the audited dictionary** — the 32-key `shell_chrome_string` table is 1:1 EN/DE and has a safe fallback, but the Command Palette's ~19 strings (`Set Appearance`, `Light`, `Dark`, `Set Locale`, …) are hardcoded English literals via `Label::data(...)`, bypassing the table entirely | `wgpu-shell.rs:9602-9644` | grep-based law: every `Label::data("...")` call site in chrome code must instead route through `shell_chrome_string` |
| 9 | **Resident-document budget is not the binding constraint on mesh size** — clarifying, not a bug: `meshes_json`/`instances_json` sit on a resident `UiComponentSceneNode` (governed by `UI_RESIDENT_SURFACE_BYTES` = 8 MiB / `UI_RESIDENT_AGGREGATE_BYTES` = 32 MiB), but geometry is bridged out into a *separate* `Mesh3dSchema` pool (16 MiB per mesh, 64 MiB pool cap, `🖱️ui/🎬️scene/📐️math/🦀️.rs:286-290`). All 8 bundled examples are simple analytic BREP shapes (single-digit-to-low-double-digit KB transferred per mesh per `📓️preview-mesh-delivery-2026-09-12.md`), so neither ceiling is at plausible risk today — flagged low-confidence-but-low, not a live concern | contract: `🎟️resident/🦀️.rs:11-14`; mesh pool: `📐️math/🦀️.rs:286-290` | none needed now; revisit if a denser (imported/high-poly) example is added |

---

## 1. UiNode/`Component` kind coverage — wgpu reconcile+paint vs React

Canonical enum: `Component` (18 variants) at
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:495-511` — `Container, Text, Button,
Separator, Input, Select, Toggle, KeyValueList, Slider, NumberStepper, Ring, IconSelect, Tree,
TreeSection, TreeItem, Image, Surface, Extension`. wgpu's own post-reconcile `UiNode` enum
(`🧩️component/🦀️.rs:3154-3172`, wgpu-target) has 19 variants (`Container` fans into
`Stack/Field/Section/Group` by role; `Surface`→`ComponentScene`; `Extension`→`ExternalSlot`).

**Reconcile (`reconcile.rs:495-780`) and paint (`paint.rs:591-1075`) are exhaustive Rust matches — every
kind produces a retained node and a real (non-placeholder) paint call.** This is a materially better
state than the 2026-09-11/12 "unknown-kind" audits found; those were investigating a *wire/deserialization*
gap, not a reconcile/paint fallback — wgpu has no runtime "unknown kind" branch at all (an unrecognized
`type` tag fails at `serde` deserialization, before reconcile ever runs), unlike React's explicit
`UnknownComponentView` (`🗣️Interpreter/🟦️.tsx:1458-1463`).

| kind | wgpu paint | dispatch (press → guest action) |
|---|---|---|
| Container/Stack/Field/Section/Group, Text, Separator, Image, KeyValueList, Tree/TreeSection/TreeItem | real | n/a (display-only, or row dispatch — covered by the hit-registry lane) |
| **Button** | real | **wired** (`button.action`, `events.rs:1259`) |
| **Select** | real | **wired** (`toggle_select_popup`, `events.rs:1253-1256`) |
| **Input** | real (border, live caret/selection) | **not wired** through the retained `EventRouter` — `events.rs:719-726` names it explicitly as "a documented gap for a later milestone." A *separate*, older shell-level system (`renderer.rs`'s `AppInteractionState.input: InputState<ActionDescriptor>` + `commit_focused_input`, ~line 7402) does parse and dispatch `on_absolute`/`on_change` for ids matching `stepper_metas` — but whether a retained-document click ever sets that system's `focused_id` (as opposed to only the `EventRouter`'s own, separate focus state) was not confirmed either way this pass. See ranked gap #1. |
| **Toggle, Slider, NumberStepper, Ring, IconSelect** | real | **not wired** — appear only in `is_focusable` (`events.rs:288`); same caveat as Input applies if the older shell-level system happens to reach them, unconfirmed |
| **Surface→ComponentScene** | real, deferred to `scene_slots`/engine-canvas per `SurfaceKind` | see §2/§3 below for World3d/NodeGraph specifically |
| **Extension→ExternalSlot** | placeholder border + `body_key` label (correct — plugin body is host concern) | n/a |

`SurfaceKind` (15 kinds, `reconcile.rs:410-424`): generation3d uses only `World3d` and `NodeGraph`, both
routed through `engine_canvas::sync_engine_scene`/`surface_kind_uses_engine_canvas`
(`scenes.rs:1223,1396`, the set is `World3d | NodeGraph | TiledMap | Board2d`) — real content, not
placeholders, for all four. The remaining 11 `SurfaceKind`s are unverified this pass and unused by
generation3d.

---

## 2. Window transient lanes (meshes, instances, selection, hover, environment, status, camera)

Shared schema: `World3dSceneLane` (17 lanes) at `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:491-511`.
React reads the same `World3dScene` shape via `🖱️ui/🎬️scene/🟦️.ts`'s `WORLD3D_SCENE_LANES`
(18-lane manifest) in `🌐️World3dHost/🟦️.tsx`.

| lane | wgpu receives/renders it | file:line |
|---|---|---|
| meshes | yes, real GPU draws | `world.rs:9448,9506` |
| instances (+ delta) | yes | `world.rs:9449,9507` |
| selection | yes | `world.rs:9771,9848-9849,8518-8525` |
| hover | yes, live pointer-move updates too | `world.rs:9772,9782,10547-10686` |
| environment | yes (clear color + light direction) | `world.rs:9804,9878,9880,6589-6608` |
| camera | yes | `world.rs:9415,9508,9064-9069` |
| **status** | **no progress UI** — only `cancellable`/`cancelAction` are read; `phase`/`phaseLabel`/`ratio`/`unitsDone`/`facesDone`/`inFlight` are never consumed, and `World3dState`/`render_world_3d` never reference `status_json` at all (own doc comment: "keeps geometry, never status") | `engine-canvas.rs:1770-1772,1780`; `wgpu-shell.rs:9282-9299` |

Six of seven transient lanes are at full parity — this was mostly closed before today (meshes/instances/
selection/hover/environment/camera show no 2026-09-13 edits). `status_json`'s cancel-only wiring is
today's chrome-parity lane's own addition (§4.1 of that report); the progress/phase half remains open —
ranked gap #4.

---

## 3. Node-graph canvas — paint, camera fit, captions, wires, minimap, dispatch

Both node-graph hosts (`FlowHost`, `GraphHost`) wrap the same shared, target-neutral `dag.rs::DagHost`
(6491 lines). Its `paint_scene` (`dag.rs:6113-6150`) is called directly from wgpu at
`engine-canvas.rs:2407,2412` — so **wires, node boxes, ports, captions and the minimap are all painted
by wgpu today**, not merely by React:

- **Wires**: real, two styles — `EdgeRouteStyle::Bezier` (`dag.rs:6139-6142`, real cubic bezier via
  `edge_curve`) and `SharpSz` (`:6144-6147`, S/Z straight-segment path); a live in-progress connection
  preview also paints when `pending_edge` is set.
- **Captions**: `paint_node_graph_labels` (`engine-canvas.rs:3120-3131`), called from
  `scenes.rs:1301-1309` phase 7, reading `dag.rs`'s own `label_overlay_paint_state_json` (`:5509-5532`).
- **Minimap**: `paint_minimap_widget` (`dag.rs:3607`), fed by `minimap_widget_layout`
  (`:3533-3607`) — genuinely painted, not the dead code an earlier pass of this audit mis-concluded from
  grepping only the thin `🖱️ui/🎯️targets/🧊️wgpu/🗺️minimap/🦀️.rs` math-library file (which is a real,
  used dependency of `dag.rs`, just not called *from* the `ui/targets/wgpu` tree directly).
- **Camera fit / pan / zoom**: `Fit graph` landed today (chrome-parity lane, not re-reported). Pan/zoom
  predates it and is independently wired (`engine-canvas.rs:2709-2746,2761,2764`), persisting through the
  same `nodeGraphViewport` action `Fit graph` also uses.
- **Dispatch — split result**:
  - **Node selection and node dragging work.** The live per-frame OS loop (`renderer.rs:13707-13811`)
    bounds-hits `node_graph_states` and calls `engine_canvas::node_graph_pointer_{down,move,up}_into`
    (`engine-canvas.rs:2747,2773,2786`), which call `host.plan_pointer`/`commit_pointer`
    (`:2877-2900`) → `DagHost::derive_pointer_plan`/`apply_pointer_plan` (`dag.rs:3103,3225`), gesture
    set `Idle | Pan | Drag | Select`. This is genuinely wired, separate from (and not blocked by) the
    shell's retained-hit registry, which per today's hit-registry report only registers
    `node-graph`/`board-2d` `ComponentScene` leaves as `HitKind::ScrollRegion` with no per-node action.
  - **Wire creation/deletion does not work.** `DagHost` separately exposes `pointer_down_screen`/
    `pointer_move_screen`/`pointer_up_screen` (`dag.rs:5054-5198`) which do port/handle hit-testing and
    enter `InteractionMode::DrawEdge` on a handle hit — fully built, reachable in principle (`GraphHost`
    even re-exposes it, `🗺️surface/🕸️node-graph/🦀️.rs:553-562`) — but **no call site in the wgpu target
    ever invokes it for the graph host**; the only three `pointer_down_screen` callers in
    `engine-canvas.rs` (`:3820,4105,4152`) are the Board2d host and the TextEditor surface, not
    `GraphHost`/`FlowHost`. Ranked gap #2.

---

## 4. Text input / form editing on wgpu

No dedicated `✏️TextEditor/🎯️targets/🧊️wgpu/` element exists — Form fields are serviced by the generic
retained-document widget pipeline, not a per-element TextEditor component (that component's wgpu target
is a separate `SurfaceKind::TextEditor` scene, used for code/richer editing surfaces, and *is* fully
wired: `text_editor_select_span_into`, `_pointer_button_into`, `_pointer_move_into`,
`_set_selection_into`, `_apply_completion_into`, `_pointer_click_into`, all in `engine-canvas.rs:4049-4160`,
committing real `on_change`-style actions).

**Two divergent systems exist for Form-field-style `Component::Input`, and this audit could not confirm
which (if either) a retained-document click actually reaches — this is the report's single most
consequential open question:**

1. `🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`'s `EventRouter` — real focus (`NodeFlags::FOCUSED`,
   `focus_next`/`focus_prev`, `:355,368`), real character-level editing (`route_text_insert`, IME
   composition, clipboard copy/cut/paste all wired, `:640-1316`) — but its own docstring
   (`:719-726`) says committing the edited value via `on_change` is *not implemented*. This is the
   system the retained-hit-registry lane routes pointer presses into
   (`route_retained_pointer_press`), i.e. it is very likely what a click on a Form window's retained
   `Component::Input` node actually reaches.
2. `renderer.rs`'s product-level `AppInteractionState.input: InputState<ActionDescriptor>` (real
   `focused_id`, `text_buffer: TextEditAuthority` with bounded streaming commit) — its
   `commit_focused_input` (~line 7402) **does** parse and dispatch `on_absolute` (numeric, via
   `stepper_metas`) / `on_change` (text) — but this system's own focus (`input.focused_id`) is a
   *shell-level* immediate-mode concept, and it is unconfirmed whether the retained-hit-registry's
   press-routing (into system 1 above) ever also sets it.

If system 2 is in fact unreachable from a retained click, then editing a generation's numeric/text
parameters is fully cosmetic on wgpu today (local caret movement, no guest dispatch) — this is ranked
gap #1, and is high-impact because editing generation inputs is core to the app's purpose. Keyboard
focus traversal itself (Tab order) exists structurally at the retained-document level
(`events.rs:287-368`), independent of this commit question.

---

## 5. i18n (English/German) and accessibility on wgpu chrome

**i18n**: `shell_chrome_string` (`wgpu-shell.rs:13931`) is a `match (key, is_de)` table, 32 keys × 2 =
64 arms, **1:1 EN/DE, no orphans**, with a safe `(other, _) => other` fallback (leaks the raw key rather
than crashing — acceptable, not silent-failure-dangerous). But only 21 call sites (16 distinct keys) in
the whole 14k-line shell file route through it. **The entire Command Palette bypasses it**:
`wgpu-shell.rs:9602-9644` hardcodes ~19 English-only strings ("Set Appearance", "Light", "Dark",
"Set Locale", "English", "Deutsch", "Set Theme", "Reset Dock Layout", …) via `Label::data("...")` —
none has a German counterpart, directly contradicting CLAUDE.md's no-default-language requirement for
chrome UI. Ranked gap #8. (Guest-authored Form/window content — "Add a generation to edit input
values.", "Generations", "Add Generation" — is a separate, guest-side i18n path, not audited here.)

**Accessibility**: DOM footprint is exactly what `📓️audit-wgpu-journey-readiness-2026-09-13.md` §4
found, plus one addition — `🚀️browser-boot/🟦️.ts:68` sets a single static, bilingual
`aria-label="Semio Arbeitsfläche"/"Semio workspace"` on the whole canvas. Beyond that, only
`[role="status"]` (`:95-96`) and `[role="alert"]` (`:142`) exist; no other `aria-*`/`role` attribute
exists anywhere under the wgpu renderer target.

**The data model already carries per-node accessibility intent, unused by wgpu — a clean, high-value
close.** `AccessibilitySpec { label, description, live: Liveness, shortcut, hidden }`
(`🧬️contract/♿️accessibility/🦀️.rs`, marked `⚠️ SCAFFOLD` but wired end-to-end: it is a field on every
`UiNodeRecord`, `📃️document/🦀️.rs:153`, with its own `SetAccessibility` patch op). React's Interpreter
consumes it directly — `accessibilityAriaProps(spec, idBase)` at `🗣️Interpreter/🟦️.tsx:871`, applied at
`:1061` as real `aria-label`/`aria-describedby`. **wgpu's `reconcile.rs`/`paint.rs` and the wgpu
`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` have zero references to `AccessibilitySpec` or
`record.accessibility`.** Ranked gap #3 — closing it means adding one projection in the wgpu target
analogous to `accessibilityAriaProps`, not inventing new schema.

---

## 6. Boot queries (`?example=`/`?mode=`/`?role=`) and the wgpu server's plumbing

Current `🚀️browser-boot/🟦️.ts:45-55` (re-read fresh, confirmed unchanged since this morning's audit and
deliberately left alone by today's chrome-parity lane): reads `plugin`, `role`, `mode` (+`hub`/`user`/
`dataDir`); **still no `example`**.

**Re-scoping the brief's framing**: this is *not* a wgpu-only gap. The shared cross-renderer contract
(`🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts`, explicitly documented as "the axes the two renderers must
agree on") defines only `plugin` and `role`. React's own dev entry
(`🔨️modules/🧑‍💻dev/🟦️.ts:48`) resolves its default example from a **build-time** env
(`VITE_SEMIO_DEFAULT_EXAMPLE`) with an explicit docstring that it has "no `?query=` override." So
wgpu adding `?example=` would be a probe/deep-link convenience, not a parity requirement — the actual
parity gap is the clickable example picker, already closed at the source level by today's chrome-parity
lane (runtime not yet proven, per that report's own §6).

`mode` is asymmetric the other way: wgpu can only set mode at boot via `?mode=` (no clickable control
exists yet per the chrome-parity report's mode-group section, though that report claims the mode group
*was* added today — cross-check at runtime once builds unblock); React's mode is pure clickable runtime
state with **no** query dependency at all.

**The wgpu server itself** (`🌐️server/🎚️config/🟦️.ts`, `📜️script.ts`) reads no example/mode/role —
only `SEMIO_PLUGIN` (playground variant) and `SEMIO_BUILD_MODE` (dev/release), plus two unrelated envs
(`GIS_MAP_TILE_SERVE_MODE`, `S_LOCAL_RELAY_URL`/`_SECRET`). `📜️script.ts`'s `serve` subcommand accepts
only `--port`/`--host`. Both axes are entirely client-side (`bootDescriptor()`), matching React's
architecture — no asymmetry here.

---

## 7. Resident document budget vs. largest example mesh sizes

Two **separate** budgets exist; mesh geometry is not gated by the one the brief named:

- `UiResidentPermit` (`🧬️contract/🎟️resident/🦀️.rs:11-14`): `UI_RESIDENT_SURFACE_BYTES` = 8 MiB,
  `UI_RESIDENT_AGGREGATE_BYTES` = 32 MiB, `UI_RESIDENT_SLOTS` = 64, `UI_RESIDENT_SURFACE_ITEMS` = 4097.
  This governs the retained `UiTree` — including the *string* fields `meshes_json`/`instances_json`
  while they sit on a resident `UiComponentSceneNode`, per `📨️scene_slots/🦀️.rs:1-9`'s own docstring
  (a read-side borrow off the same arena, not a bypass). Measured utilization: 3.9% across six
  simultaneous editor+viewer surfaces including chrome (`📓️wgpu-resident-budget-settle-2026-09-12.md`).
- `Mesh3dSchema` (`🖱️ui/🎬️scene/📐️math/🦀️.rs:286-290`): the *actual* geometry ceiling once bridged out
  of the JSON string — `MESH3D_OWNER_BYTE_CAPACITY` = 16 MiB per mesh, `MESH3D_AUTHORITY_PAGE_CAPACITY`
  × `MESH3D_PAGE_BYTES` = 64 MiB pool across up to `MESH3D_AUTHORITY_CAPACITY` = 256 concurrently-leased
  meshes.
- `RETAINED_DOCUMENT_OPPORTUNITIES` = 256 (`🐚️plugin-bridge/🟦️.ts:258`) is a retry-count for turn
  submission, not a byte/item budget — unrelated to mesh size despite the similar name.

All 8 bundled examples are simple analytic BREP shapes (box/sphere/torus/extrude primitives); the only
concrete per-example number found (`📓️preview-mesh-delivery-2026-09-12.md`) is
`face-sweep-extrude`: 12 triangles / 1 mesh / 1 368 B transferred. No report gives full vertex/byte
counts across all 8, but the transport itself is capped at 64 KiB raw per eval tick
(`GENERATION3D_FLOW_EVAL_RAW_BYTES`), which bounds how large any bundled example's mesh payload can even
be today. Neither the 32 MiB resident ceiling nor the 16 MiB/64 MiB mesh-pool ceiling is at plausible
risk from these 8 examples — flagged low-confidence-but-low per ranked gap #9, not a live concern, and
worth re-measuring only if a denser (imported/high-poly) example is ever added.

---

## Corrections to this audit's own earlier working notes

Mid-session, before delegating research, this audit briefly concluded the minimap math module
(`🖱️ui/🎯️targets/🧊️wgpu/🗺️minimap/🦀️.rs`) was dead code because a narrow grep found no caller inside
`ui/targets/wgpu` itself. That was wrong: its caller is `dag.rs` (a different crate,
`♾️infinite/🎲️board/…`), which wraps it and is itself called from `engine-canvas.rs`. The minimap does
paint on wgpu (§3). This is recorded so a future reader doesn't repeat the same narrow-grep mistake.
