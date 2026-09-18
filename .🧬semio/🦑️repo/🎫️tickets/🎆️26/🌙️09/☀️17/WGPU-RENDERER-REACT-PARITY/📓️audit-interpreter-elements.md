# wgpu ↔ React Interpreter / Element / Layout Parity Audit

Lane: **UiNode interpreter, element/widget coverage, layout rules**. Read-only audit, 2026-09-17.
Cross-refs: ticket `26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING` (tree windowing, not duplicated here),
ticket `26/09/02/PUZZLE-3D-END-TO-END` wave B12 (input-commit / stepper-binding history cited below),
ticket `26/09/09/PROCEDURAL-3D-END-TO-END` (accessibility projection, wgpu-document-reconcile notes).

## 0. Scope & method

Grepped/read primary source only (`grep`/`sed`/Python-assisted reads where BSD `grep` choked on this
file's emoji byte sequences — see note at end of §2). No cargo builds run. Four parallel read-only
research passes fed this report (widget coverage matrix, layout rules, wire format/reconcile bridge,
tree+accessibility+i18n+presence); their findings are folded in below with citations, cross-checked
against my own independent reads of the same files where the claims mattered most (the reconcile
bridge, the flex engine's dead-code status, and the CAD-plugin gap were independently re-verified).

## 1. Architecture overview — there are two schemas, not one

This is the single most important fact for reading everything below. The ticket's framing ("UiNode
element coverage") suggests one shared node type; there are actually **two**, and only the first is
truly shared:

1. **`ui_contract::Component`** — the canonical, 19-variant, flat, id-keyed wire record kind, defined
   once at `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:606-626`: `Container, Text,
   Button, Separator, Input, Select, Toggle, KeyValueList, Slider, NumberStepper, Ring, IconSelect,
   Progress, Tree, TreeSection, TreeItem, Image, Surface, Extension`. Every node also carries a
   renderer-neutral `LayoutSpec` (`🧬️contract/📐️layout/🦀️.rs`), `StyleSpec`
   (`🧬️contract/🎨️style/🦀️.rs`), `AccessibilitySpec` (`🧬️contract/♿️accessibility/🦀️.rs`), retained
   in a flat id-keyed table as `UiNodeRecord` (`🧬️contract/📃️document/🦀️.rs:1-292`, `UiPatch`/
   `UiPatchOp` diff ops at lines 210-292). **React reads this record directly** — no intermediate
   conversion.
2. **wgpu's own `UiNode`** — a *second*, tree-shaped, 20-variant enum private to the wgpu target,
   defined at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:3265-3286`: `Stack,
   Text, Button, Separator, Input, Select, Toggle, KeyValue, Slider, NumberStepper, Ring, IconSelect,
   Progress, Field, Section, Group, Tree, Image, ComponentScene, ExternalSlot`. wgpu **converts** every
   `Component` record into this shape via `ui_node_from_record()`
   (`🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:727-830`) before painting it. The conversion is
   **variant-exhaustive** (every `Component` arm is handled, confirmed by direct read — no `_ =>`
   catch-all at that match) but, as §5 shows, it is **not field-exhaustive**: several fields on
   `LayoutSpec` in particular are read out of the source record and then never placed anywhere in the
   destination `UiNode`, because the destination struct has no field to hold them.

React's dispatch (`renderComponent`, `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/
🗣️Interpreter/🟦️.tsx:2157-2203`) switches directly on `record.component.type` and is exhaustive over
all 19 `Component` kinds with a `default: UnknownComponentView` (line 2200-2201) fallback for anything
unrecognized. wgpu's bridge is architecturally equivalent in spirit but produces a *different* tree
shape, which is itself a second source of drift independent of any individual field being dropped:
`Container` role `Section`/`Group`/`Field` each become distinct wgpu `UiNode` variants
(`🔀️reconcile/🦀️.rs:729-755`), while on the React side `role=section` and `role=group` render through
the **same** `<Section>` widget (`🗣️Interpreter/🟦️.tsx:1129-1135`) — i.e. React itself treats
Section/Group as visually identical, which happens to make wgpu's split harmless in practice.

wgpu additionally paints through **two separate systems that are not variant-equivalent**:
- The real, incremental, cursor-stepped production painter, `paint_node`/`paint_node_self` in
  `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`, driven from the Interpreter target's
  `render_ui_document_step` (`🛍️products/💻️os/.../🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1219`). This
  is the actual per-frame path apps go through.
- A second, smaller, non-incremental `WidgetNode<E>`/`render_widget` kit,
  `🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:182-388`, used by scene-embedded panels
  (`🛍️products/💻️os/.../🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`) and the standalone Tree element target.
  **This kit's `WidgetNode` enum is strictly smaller** than `UiNode` — it has no `Progress`, `Image`,
  `Group`, `ComponentScene`, or `ExternalSlot` arm — so any scene-embedded panel restricted to this kit
  cannot show those five kinds *at all*, independent of the main paint path's own coverage.

A number of **11 individual per-widget wgpu implementation files** also exist, colocated as siblings of
their React counterparts under each element's own folder (`🧱️elements/<Name>/🎯️targets/🧊️wgpu/🦀️.rs`
for Input, Ring, Tree, Slider, IconSelector, PresenceBar, Toggle, KeyValue, Button, Select, Stepper —
confirmed by direct listing). These are thin (25-294 lines each), pulled back into
`crate::wgpu::widgets` as sibling modules (`🎯️targets/🧊️wgpu/🦀️.rs:180-195`, e.g. `#[path =
"../../🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs"] mod select;`) per ticket
`26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE` — a code-organization move, not a second implementation
layer; the substantive logic lives in exactly the files this audit already cites. Verified directly:
`🔽️Select`'s 88-line file (`render_select`/`render_select_menu`) is a hand-drawn, hit-tested dropdown
with **no search/filter/keyboard-typeahead**, confirming the P2 finding below independently.

## 2. Coverage matrix — core `Component`/`UiNode` kinds

*(Method note: BSD `grep` on this machine silently returns zero output — not an error, not a "0" count
— on the React Interpreter `.tsx` file, apparently choking on a multibyte sequence somewhere in its
~130 KB of emoji-laden source; `python3 -c "open(...).read()"` reads it fine. Anyone re-running greps
against `🗣️Interpreter/🟦️.tsx` should verify a known-good string round-trips, or use Python, before
trusting a "no matches" result.)*

| Node kind | React (file:line) | wgpu (file:line) | Status |
|---|---|---|---|
| **Container role=Plain/Form/Toolbar** | `ContainerView`, `🗣️Interpreter/🟦️.tsx:1112-1161`: plain `<div>`; `role="form"`/`"toolbar"` HTML attribute from `component.role` (line 1121); activate-binding → `role="button"` + `onClick` (1150-1156); `data-activity`, ARIA via `accessibilityAriaProps`. | `ui_node_from_record`, `🔀️reconcile/🦀️.rs:762-773`: **all three roles collapse into one `UiNode::Stack`** — direction/gap/padding via `stack_metrics`, drop/activate actions wired. Form/Toolbar semantic role has no wgpu representation. | P2 divergent (role semantics dropped; visually near-identical since wgpu has no ARIA landmark rendering anyway) |
| **Container role=Section** | `🗣️Interpreter/🟦️.tsx:1129-1135`: `<Section>` widget, title + id. **Same component as role=Group.** | `🔀️reconcile/🦀️.rs:729-735` → `UiNode::Section`; painted `🖌️paint/🦀️.rs:1013-1036` (chevron + label, `default_open`/collapse state). | OK |
| **Container role=Group** | `🗣️Interpreter/🟦️.tsx:1129-1135`, identical to Section. | `🔀️reconcile/🦀️.rs:736-743` → `UiNode::Group`; painted `🖌️paint/🦀️.rs:1039-1064`, same chevron+label pattern. | OK |
| **Container role=Field** | `🗣️Interpreter/🟦️.tsx:1137-1144`: `<Field>` — label/description/required/error. | `🔀️reconcile/🦀️.rs:744-755` → `UiNode::Field`; painted `🖌️paint/🦀️.rs:992-1011`. The struct's `child` slot is a `UiSeparator` placeholder never painted (reconcile.rs:749-751 comment) — the real child is a normal arena child, not this field. | OK |
| **Text** | `TextView`, `🗣️Interpreter/🟦️.tsx:1164-1173`: `<p>`, `emphasize`→bold, `data-attributes` passthrough. | `🔀️reconcile/🦀️.rs:774` → `UiNode::Text`; painted `🖌️paint/🦀️.rs:598-613`. `data_attributes` carried through the wire (`data_attributes()` helper) with no DOM to attach to on wgpu — likely inert there. | OK |
| **Button** | `ButtonView`, dispatch `2166`, icon via `resolveControlIconNode` (1182). | `button_node()`, `🔀️reconcile/🦀️.rs:636-647`; `paint_button`, `🖌️paint/🦀️.rs:2371`. | OK |
| **Separator** | Inline `<hr>`, dispatch `2168`. | `🔀️reconcile/🦀️.rs:775` → `UiNode::Separator`; themed line, `🪀️widgets/🦀️.rs:317-319`. | OK |
| **Input** (`text`/`longText`/`number`/`date`/`color`/`file`) | `InputView`, `🗣️Interpreter/🟦️.tsx:1215-1272`. Real `<input type=number\|date\|color\|file\|text>`/`<textarea>` — native OS date/color pickers and file dialog for free. Draft/commit-on-blur state machine (1192-1213) fixes a real documented bug (comment 1194-1201: a controlled input with no `onChange` silently reverted every keystroke — ticket `26/09/09/PROCEDURAL-3D-END-TO-END`). Continuous coalescing lane for un-committed number input (1219-1230, `useContinuousTriggerLane`). `min`/`max`/`step`/`accept` all passed to the DOM element (1263-1266). | `input_node()`, `🔀️reconcile/🦀️.rs:590-606`; `UiInputNode` (`🧩️component/🦀️.rs:2015-2040`) **carries** `min`/`max`/`step`/`accept` on the wire, but `render_widget`'s `Input` arm destructures only `{ id, value, placeholder, commit, on_change, .. }` (`🪀️widgets/🦀️.rs:325-328`) — **the `..` explicitly discards min/max/step/accept**, confirmed by direct read of both the struct and the destructure. `input_kind` only selects a text tag (`🔀️reconcile/🦀️.rs:317-326`); no native date-picker/color-picker/file-dialog exists in an immediate-mode canvas, and none of the three was found wired to any host-level dialog anywhere in the crate. | **P1 missing** for `min`/`max`/`step`/`accept` (data present on the wire, silently dropped at render); **P2 divergent** for `date`/`color`/`file` kinds (no native-picker equivalent); OK for plain `text`/`longText`/`number` |
| **Select** | `SelectView`, `🗣️Interpreter/🟦️.tsx:1274-1290`: Radix `<Select>` — keyboard nav, typeahead, portal-rendered dropdown, collision-aware positioning, all free from the library. | `select_node()`, `🔀️reconcile/🦀️.rs:608-619`; `render_select`/`render_select_menu`,
`🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:41-88` (verified directly): hand-drawn hit-tested rows, hover highlight, no search/filter, no keyboard arrow-key navigation of the open menu found, no collision-aware repositioning (menu always opens directly below, `bounds.y + bounds.h + 2.0`, `select.rs:66`). | P2 divergent (functional parity for the click-to-choose case; no keyboard/typeahead/collision handling) |
| **Toggle** | `ToggleView`, `1292-1295`: `<Toggle pressed text icon>`. | `toggle_node()`, `🔀️reconcile/🦀️.rs:621-633`; `paint_toggle`, `🖌️paint/🦀️.rs:2374`. **Neither side's wire contract has a true Checkbox or radio-group `Component` kind** — both are limited to this one boolean `Toggle` plus `Select`/`IconSelect` for choice-from-a-set. This is a contract-level limitation shared identically by both targets, not a React/wgpu divergence (see also the Checkbox row in §3 for the one place a real tri-state checkbox exists in the codebase). | OK (shared contract ceiling, not a renderer gap) |
| **KeyValueList** | `KeyValueListView`, `1297-1309`. | `key_value_node()`, `🔀️reconcile/🦀️.rs:648-656`; `paint_key_value`, `🖌️paint/🦀️.rs:2375`. | OK |
| **Slider** | `SliderView`, `1311-1336`: Radix slider (drag+click+arrow-key), optional `unit` suffix text (1327-1334). | `slider_node()`, `🔀️reconcile/🦀️.rs:657-670`; `render_slider(id, value, min, max, step, ready, disabled, ...)` — full min/max/step honored (`🪀️widgets/🦀️.rs:338-340`), unlike Input. `unit` field exists on the struct; its paint-side rendering wasn't confirmed present in the same pass. | OK core parity; unit-suffix rendering unverified |
| **NumberStepper** | `NumberStepperView`, `1338-1356`. Comment (1347-1353) documents a **real historical bug**: `onDelta` used to be wired unconditionally, silently swallowing every +/- click for any node that declared only `Trigger::Change` (puzzle3d's four Settings steppers, all four inert — ticket `26/09/02/PUZZLE-3D-END-TO-END` wave B12, browser-measured). Now fixed: `onDelta` gated on `(record.bindings ?? []).some(binding => binding.trigger === "delta")` (line 1353). | `number_stepper_node()`, `🔀️reconcile/🦀️.rs:672-685`: **both `on_absolute` and `on_delta` are wired unconditionally** via `record_action_or_inert` for each trigger (lines 680-681) — no analogous binding-presence gate. Whether `record_action_or_inert` degrades to a safe no-op when no `delta` binding exists was not fully confirmed in this pass. | **P2 divergent — same historical bug class not confirmably fixed on wgpu.** Verify `record_action_or_inert`'s no-binding behavior before treating this as closed. |
| **Ring** | `RingView`, `1358-1361`. | `ring_node()`, `🔀️reconcile/🦀️.rs:686-697`; `paint_ring`, `🖌️paint/🦀️.rs:2378`. | OK |
| **IconSelect** | `IconSelectView`, `1363-1374`: `classifierKind === "puzzle2d"` activates an app-specific `classifyIconSelectorMode` callback (1367). | `icon_select_node()`, `🔀️reconcile/🦀️.rs:698-709`: `classifier_kind` carried as a plain string; no equivalent puzzle2d-specific mode found in `paint_icon_select`/`render_icon_select`. | P2 divergent (app-specific classify mode is React-only) |
| **Progress** | `ProgressView`, `2098-2101`, ARIA value text via shared `accessibility_value()` projection. | `progress_node()`, `🔀️reconcile/🦀️.rs:711-715, 785`; `paint_progress`, `🖌️paint/🦀️.rs:957`. **Absent from the smaller `WidgetNode` kit** — scene-embedded panels using that kit cannot show a progress bar at all. | OK on the real paint path; **P1 gap** on the scene-panel `WidgetNode` path |
| **Image** | `ImageView`, `1971-1973`: real `<img src>`, browser decodes/loads/caches the bitmap. | `🔀️reconcile/🦀️.rs:786` → `UiNode::Image`. Painted `🖌️paint/🦀️.rs:1066-1089`: when `has_scene_host` is false (**the default, common case**), draws **only a placeholder rounded rect + alt/id text label** — it never decodes or uploads the image bytes named by `src`. The real raster-quad drawer, `fn paint_image` (`🖌️paint/🦀️.rs:2385-2398`), is **`#[cfg(test)]`-gated** — compiled out of every production build — and its own doc comment (2383-2388) says texture upload "lives in the renderer's `program_bridge`/`engine_canvas`, outside this crate's scope," with no confirmed wiring found anywhere. | **P0 broken** — a plugin emitting `Component::Image` for a real picture (not routed through a Surface/SceneHost) shows a grey placeholder box on wgpu, never the image |
| **Surface** (`Canvas2d`/`World3d`/`NodeGraph`/`TextEditor`/`Table`/`Paint2d`/`VirtualFileSystem`/`TiledMap`/`Board2d`/`IconRender`/`InkCanvas`/`GraphTimeline`/`BlockList`/`DiffView`/`EventFeed` — 15 `SurfaceKind`s) | `SurfaceView`/`PagedSurfaceView`, `1971-2096`, dispatches to `ComponentSceneHost` per kind. | `surface_scene_node()` → `UiNode::ComponentScene`; all 15 kinds mapped 1:1 (`🔀️reconcile/🦀️.rs:318-336, 394-436`). | Enum-level parity confirmed; per-scene-kind painting behavior is its own large audit surface, out of this lane's depth budget — flag for a dedicated Scenes/EngineCanvas audit (the ticket's own status.md already lists one) |
| **Extension** (guest plugin slot) | `ExtensionView`, `2122-2130`, wrapped in `ShellFaultBoundary`. | `🔀️reconcile/🦀️.rs:826-830` → `UiNode::ExternalSlot`; `paint_external_slot`, `🖌️paint/🦀️.rs:1799`. Note: `props.props` is serialized to the extension slot via **literal `serde_json::to_string`** (reconcile.rs, Extension arm) — a concrete point where the wire is textual JSON regardless of whatever binary envelope wraps the rest of the patch (see §6). | OK structurally; JSON sub-encoding noted for §6 |
| **Tree (root)** | `TreeView`, `1878-1945`: real scroll-driven virtualization — `useTreeWindowObserver` measures the live scroll viewport, computes visible rows + overscan against `TREE_WINDOW_BODY_NODE_BUDGET`/`TREE_WINDOW_OVERSCAN_ROWS`, and **requests** a window from the guest each time the viewport changes (`reportWindows`, ~1480). | `🔀️reconcile/🦀️.rs:787-812` builds one inline `UiTreeNode` from **every** `TreeSection`/`TreeItem` child eagerly (depth-capped only by `UI_DOCUMENT_RECONCILE_DEPTH`, not by viewport). Painted via `retained_tree_node_step` up to a fixed `RETAINED_NODE_COLLECTION_ITEMS: usize = 256` (`🖌️paint/🦀️.rs:47, 293`) inside a GPU scissor rect — **clipping at paint time, not virtualizing at data time**. The gap is explicitly self-documented: `🧩️component/🦀️.rs:2362-2367` states wgpu "RENDERS a window but never REQUESTS one... a wgpu tree only ever shows the first-paint window its guest chose. There is deliberately no `+N` continuation-row fallback." `paint.rs:14-15` separately notes "no scrollable-viewport paint exists yet" for `Tree`'s own live scroll offset. | **P1 missing** — real, already-ticketed gap (`26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING` packet P5); cited here for completeness, not re-litigated |
| **TreeSection/TreeItem outside a Tree parent** (malformed doc) | `2189-2192`: falls back to `<ContainerView>`, explicit "malformed document" comment. | `🔀️reconcile/🦀️.rs:813-824`: falls back to `UiNode::Stack`, same fallback pattern. | OK — both sides degrade identically |
| **Tree row actions / drag-drop** | `TreeDataItem`/`RowAction` fields, `TreeDragAndDropController`. | `row_action()` (`🔀️reconcile/🦀️.rs:460-474`), `drag_data()`, `drop_overlay()` (`:772`). | OK, not deep-audited beyond dispatch presence |

## 3. Coverage matrix — composed / chrome-level widgets

These are the ticket's other named kinds (table, list, tabs, tooltip, badge/chip, menus, forms, actions
pane, engagement controls). None of them is a `Component`/`UiNode` variant — they are either DOM
compositions of `Container`/`Group` on the React side, or OS-shell-chrome constructs outside the
per-plugin retained document entirely. Framing every one of these as "missing in wgpu" would
overstate the finding; the table below distinguishes real gaps from widgets that are architecturally
outside the wire contract on **both** targets.

| Widget | React reality | wgpu reality | Status |
|---|---|---|---|
| **Table** | `Table`/`TableAvatar`/`HistoryTable`, `🎯️targets/⚛️react/🟦️.tsx:8456,11227`. Real production usage found only in shell chrome (`🛍️products/💻️os/.../🧵️TaskManager/🟦️.tsx`) plus its own stories/tests — **not reachable through `ui_contract::Component`** at all. | No `UiNode::Table`. `SurfaceKind::Table` (a *Scene* embedded via `Component::Surface`) is a structurally different, real mechanism on both sides (see Surface row above) — not the same thing as this generic DOM table. | N/A for the generic widget (dead/chrome-only on React too); TaskManager's own wgpu chrome counterpart is unaudited (shell-lane scope, not this audit's) |
| **Tabs** | `Tabs`/`TabsContent`/`TabsList`/`TabsTrigger`, `⚛️react/🟦️.tsx:9490`. Grep across `🧰️framework` and `✏️s` (plugins) found **zero production call sites** — only the element's own stories/tests. Superseded in practice by `PanelTabBar`/window-dock tabs, a separate hand-authored shell-chrome mechanism. | No `UiNode::Tabs`; no wire concept. | N/A — effectively dead generic widget on React too, not a wgpu regression |
| **List / fixed list** | **No React `.tsx` implementation exists** at `🧱️elements/📃️List/` at all — the folder's only real content is a leftover TUI target (`🎯️targets/⌨️tui/🦀️.rs`, 49 lines, `list_on_key`/`paint_list`). | Same absence. | N/A — absent on both sides identically, worth flagging to whoever owns the element inventory as a stale/orphaned folder, not a parity gap |
| **Scroll areas** | `Scrollable`, `⚛️react/🟦️.tsx:9018`; `ScrollLayout` contract type. | `NodeFlags::SCROLLABLE` (`🌳️tree/🦀️.rs:193`) + `WidgetState::scroll_offset`, routed in `⚡️events/🦀️.rs:1145-1160`; `render_scroll_region()`, `🪀️widgets/🦀️.rs:492-503`. | **P2 divergent** — flags/offset exist and a render helper exists, but `🖌️paint/🦀️.rs:14-15`'s own doc comment says "no scrollable-viewport paint exists yet" for the one case it calls out (`Tree`'s live scroll) — real clipped-scroll-region painting is not confirmed end-to-end |
| **Tooltip** | No dedicated `Tooltip` element folder. Hover text comes from a shared tiered reveal system, `UiDriverTooltips` (`none`/`label`/`tooltip`), `useControlTooltipText`/`formatControlTooltipText` (`⚛️react/🟦️.tsx:1525-1613`) — layered onto *any* control generically via `AccessibilitySpec`/label tiers. | Zero hits for any tooltip/hover-reveal concept anywhere in `🧩️component/🦀️.rs`, `🖌️paint/🦀️.rs`, or the Interpreter target. wgpu is a custom immediate-mode canvas — it gets no free OS/DOM hover affordance, so this needs bespoke hit-testing + delayed popup, and none exists. | **P1 missing** — real, unticketed gap |
| **Badge / Chip** | `Chip` element folder exists but grep for `<Chip` usage anywhere under `🧰️framework`/`✏️s` (excluding tests/stories/node_modules) returns **zero** results. | No wire concept, no `UiNode` variant. | N/A — orphaned on the React side too |
| **Menus (context menu)** | `ContextMenuController` (`🗣️Interpreter/🟦️.tsx:19`), driven by `record.menu`/`requestContextMenu`. | `menu_ref()` called on essentially every node kind in the reconcile bridge (`🔀️reconcile/🦀️.rs:247-248` + ~20 call sites). The full `ContextMenuOrganizer`/`ContextMenuItemSpec`/grouping-taxonomy machinery is **defined once**, in the wgpu crate, and explicitly documented as shared: `🧩️component/🦀️.rs:214-543`'s own comment states the taxonomy flows "through `ContextMenuController` (React) / `render_context_menu` (wgpu) unchanged." Implementations confirmed in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` and `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, with a dedicated test `🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs`. | **OK — one of the strongest-engineered parity areas found.** (Tree row actions separately route through a narrower `UiTreeActionPlacement::Menu` mechanism, `paint.rs:461, 2314`, which is *not* the same system as this general context menu — a naming collision, not a functional gap, but worth knowing if the two are ever compared.) |
| **Popover** | `Popover`, `🧱️elements/🗨️Popover/🟦️.tsx` (444 lines): its own from-scratch DOM-rect collision-aware positioner (side flip, `alignOffset`/`sideOffset`, boundary detection) — **not** the same `Anchor` enum `OverlayLayout` defines (see §5); a separate `anchorRef`-based system. | `OverlayKind`/`OverlayPlacement`/`resolve_overlay_placement` scaffolding exists in `⚡️events/🦀️.rs` (line 546+), re-exported at the crate root (`🎯️targets/🧊️wgpu/🦀️.rs:280`), but is **never consumed** by `paint`/`engine`/`mounted_layout` — zero further hits anywhere else in the module. | **P1 missing** — positioning/focus-trap scaffolding exists but nothing actually renders popover *content* |
| **Dialog / modal** | `Dialog`, `🧱️elements/💬️Dialog/🟦️.tsx` (609 lines), real focus-trap + portal. | Same `OverlayKind::Dialog` scaffolding (`⚡️events/🦀️.rs:562`, focus-trap logic at 631/966) with the identical "unwired to paint" caveat as Popover. | **P1 missing**, same root cause as Popover |
| **Forms** | `Form`, `🧱️elements/🧾️Form/🟦️.tsx` (23 lines) — a thin native `<form>` wrapper, essentially free. | N/A. | Trivial on both sides, not worth tracking as a gap |
| **Actions pane (ActionGroup)** | `ActionGroup`, `⚛️react/🟦️.tsx:8795`. Real usage is OS window title-bar chrome (`🧱️elements/🪟️Window/🟦️.tsx`), not per-plugin document content. | No `UiNode::ActionGroup`. Whether wgpu's window-chrome implementation (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, confirmed to exist and to implement context menus) has an equivalent titlebar action group was **not** audited here — it's shell-chrome, outside this lane's Interpreter/element scope. | N/A/unaudited — flag as a shell-lane follow-up, not a finding of this audit |
| **Checkbox** | `Checkbox`, `⚛️react/🟦️.tsx:8815` — real tri-state control (`indeterminate`/mixed). Its only non-test/story production usage found anywhere is the **CAD plugin's own hand-authored renderer**: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` — which bypasses `UiDocumentStore`/`Interpreter`/`ui_contract` entirely. | No `UiNode::Checkbox`, no `Component::Checkbox`. **The CAD plugin's `⚙️engine/` has no `🎯️targets`/`🧊️wgpu` sibling directory at all** — independently re-verified by direct listing (`find .../⚙️engine` shows only `🗿️artifact, 🧬️typology, 🕹️interaction, 🎬️actions, 🎰️stately, 📺️renderer, 📔️registry, 🏃️runtime` — no `🎯️targets`). | **P0 broken — the whole CAD spatial-tree-editor screen has no wgpu implementation of any kind**, not just this one widget. Known/expected given `brepjs`'s React-only status per project memory, but worth stating explicitly since it's a full screen, not an isolated control. |
| **Engagement controls** | No direct "engagement" naming found in the shared `ui_contract`; `UiPresence.peers: Vec<UiPeerMark>` (multiplayer hover/selection marks, §4) is the closest shared concept. | wgpu additionally has its **own**, unrelated `WindowEngagement*` system (`🧩️component/🦀️.rs:1212-1326`: `WindowEngagementControl::{Ring, ToggleGroup, Select}` etc.) — this is **window-chrome session-control** machinery (the "engagement" panel on a window's titlebar), a different feature from per-node presence, and out of this lane's Interpreter/element scope (shell-lane territory). Note for the reader: a naive name-match against React's `ToggleGroup` element would be a **false positive** — `WindowEngagementToggleGroupOption` is not that element. | N/A/unaudited — shell-lane scope |

## 4. Presence, disabled and loading states

`UiPresence` (`🧩️component/🦀️.rs:127-150`) is a real, fairly rich shared model: `state: UiState`
(`Introducing, Celebrating, Previewed, Normal, Disabled, Hidden` — lines 60-68), `status: UiStatus`
(`Waiting, Loading, Idle, Finished` — lines 74-80), plus `hover`, `selected`, `color` (a session's
hub-assigned palette index), and multiplayer `peers: Vec<UiPeerMark>` (every *other* peer currently
marking the element). `visible()` gates purely on `state != Hidden` (line 158-160).

React reads the same wire fields directly: `record.disabled` (`🗣️Interpreter/🟦️.tsx:1183, 2063-2064`),
`presence?.state === "disabled"` / `presence?.status === "loading"|"waiting"` (1050, 1054-1055), and
renders both a border treatment (`loadingBorderElementClass`/`waitingBorderElementClass`, 1107-1108,
2149) **and** a full-shape `elementSkeleton(record.component.type)` placeholder keyed by element kind
(2150) for a not-yet-resolved element.

wgpu's paint side implements the matching two-state **border** treatment —
`paint_loading_border`/`paint_waiting_border`, dispatched on `UiStatus::Loading`/`Waiting`
(`🖌️paint/🦀️.rs:389-390, 1692-1693, 2279-2280`) — but **there is no skeleton/placeholder-shape concept
anywhere in the wgpu target** (zero `skeleton` hits in the whole `🎯️targets/🧊️wgpu` tree). So loading/
waiting/disabled *state* is structurally shared and correctly threaded through the wire on both sides;
the *visual treatment* is strictly thinner on wgpu — a border tint only, never a shimmering
placeholder shape sized to the pending element.

**Status: P2 divergent** (state parity is real; visual treatment is not).

## 5. Layout rules

### 5.1 The central finding: wgpu's real flex engine is dead code in production

`🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` implements a genuine Taffy-backed flex layout engine
(`LayoutEngine`), with a module doc comment (lines 2-15) that explicitly calls out "Pixel-parity
requirements: every child of a `Stack` gets `flex_grow: 1.0`..." and describes real
`Style::flex_direction`/`gap`/`padding` mapping (`style_for`, `apply_stack_metrics`,
`apply_field_metrics`, `apply_section_metrics`). **Every item that actually performs the flex
computation is gated `#[cfg(test)]`** — confirmed directly: lines 17-385 are essentially all behind
`#[cfg(test)]`, including `style_for` (80), `apply_stack_metrics` (108), `impl LayoutEngine` and its
`compute`/`sync`/`write_back` methods (174-386). The **only** two items in the whole file *not*
test-gated are `LayoutJobStage`/`LayoutJobStep` (lines 187, 204) — plain progress-credit bookkeeping
enums, not layout math — and these are exactly (and only) what production code imports: `⚙️engine/
🦀️.rs:19` and `📌️mounted_layout/🦀️.rs:7` both pull in `LayoutJobStage`/`LayoutJobStep`, never
`LayoutEngine` itself. Independently re-confirmed: `⚙️engine/🦀️.rs` mentions `flex::LayoutEngine::
compute` only inside a **doc comment** (line 1192) describing an aspirational pipeline — no live code
calls it; the function underneath, `frame_step`, actually calls `progressive_layout_preview` →
`MountedLayoutResult`, i.e. the crude path below.

Production layout is therefore entirely `📌️mounted_layout/🦀️.rs` + `🧮️layout/🦀️.rs`: a hand-rolled
"sum intrinsic sizes, distribute 100% of leftover space equally across every child" pass
(`LayoutNodeKind::Stack { horizontal, gap, padding }`, `mounted_layout.rs:44` — **no align/justify/
wrap/grow field exists in this enum at all**, structurally, not just unpopulated).

### 5.2 Divergence table

| Rule | React (file:line) | wgpu (file:line) | Verdict |
|---|---|---|---|
| **direction** | `layoutSpecStyle` stack case: `flexDirection: l.axis === "horizontal" ? "row" : "column"` — real CSS flex (`🗣️Interpreter/🟦️.tsx:881-895`). | `stack_metrics()` maps `Axis`→`"horizontal"/"vertical"` string (`🔀️reconcile/🦀️.rs:220-223`); `mounted_layout` sums widths (horizontal) or heights (vertical) (`📌️mounted_layout/🦀️.rs:589-590, 638-651`). | **MATCH** |
| **gap (value scale)** | 7-point ramp × `uiSpacingRem` → **0, 3.2, 6.4, 12.8, 19.2, 25.6, 38.4 px** @16px root (`🟦️.tsx:837, 843-845`). | `space_token()` collapses 7 values into 3 buckets (`None→"none"`, `Xs|Sm→"tight"`, `Md→`default, `Lg|Xl|Xxl→"loose"`, `🔀️reconcile/🦀️.rs:199-206`); `gap_for_token()` hardcodes `tight=4.0px`, `loose=12.0px`, default=`theme.gap_standard`=3.2px (`🧮️layout/🦀️.rs:14-21`). | **DIVERGES.** `Xs`/`Sm` render identically (both 4.0px vs React's 3.2/6.4px); `Lg`/`Xl`/`Xxl` render identically (12.0px vs React's 19.2/25.6/38.4px); `Md` (React 12.8px) becomes wgpu's *default* 3.2px, a 4× shrink. |
| **padding (value scale)** | Same ramp, `edgeSpaceToPadding`/`spaceTokenRem` (`🟦️.tsx:847-851`). | `padding_for_token()`: `tight=6.0px`, `loose=16.0px`, default=3.2px (`🧮️layout/🦀️.rs:23-30`) — a **second, independently hardcoded** literal set, not derived from the same constants as gap. | **DIVERGES**, same collapse, and gap/padding drift from *each other* on wgpu despite tracing to the same `SpaceToken` wire enum. |
| **padding (per-side)** | Fully honors `EdgeSpace::Each{top,right,bottom,left}`/`Symmetric{vertical,horizontal}` as 4 independent CSS values (`🟦️.tsx:847-851`). | `edge_token()` reads **only one representative field** (`Symmetric` ignores `horizontal`; `Each` ignores `right`/`bottom`/`left`, `🔀️reconcile/🦀️.rs:208-213`) and applies that single value to all 4 sides (`UiStackNode.padding` is one `Option<String>`, `🧩️component/🦀️.rs:1909`). | **DIVERGES.** Any genuinely asymmetric padding renders correctly in React and becomes uniform-from-one-sampled-side on wgpu. |
| **align-items** | Full 5-value map (`start/center/end/stretch/baseline`) → real `alignItems` (`🟦️.tsx:860, 889`). | `StackLayout.align` is **never read** — `stack_metrics` extracts only axis/gap/padding (`🔀️reconcile/🦀️.rs:218-224`); `UiStackNode` has no align field (`🧩️component/🦀️.rs:1900-1928`, confirmed by full struct read). Cross-axis size is unconditionally hardcoded to full-stretch (`mounted_layout.rs:634-641`). | **DIVERGES except by coincidence at the default** (`Align::Stretch` is the contract default, `🧬️contract/📐️layout/🦀️.rs:62-63`). Any explicit `Start`/`Center`/`End`/`Baseline` is silently ignored. |
| **justify-content** | Full 6-value map, real `justifyContent`; children keep intrinsic size unless `grow`=true (`🟦️.tsx:861, 890-891`). | `StackLayout.justify` **never read**. `mounted_layout`'s arrange step **always** redistributes 100% of leftover main-axis space equally across every child (`mounted_layout.rs:633-642`) — every child of every Stack behaves as `flex-grow:1` unconditionally. | **DIVERGES for every value including the default `Start`.** `SpaceBetween`/`Around`/`Evenly` have no equivalent — space always goes *into* children on wgpu, never *between* them. |
| **grow** (Stack's own flex participation) | `flex: l.grow ? "1 1 auto" : undefined` (`🟦️.tsx:891`). | `stack.grow` never read (dropped in the same destructure as align/justify/wrap); no equivalent concept in `mounted_layout`. | **DIVERGES.** A `grow:true` stack gets no special treatment relative to a `grow:false` sibling. |
| **wrap** | `flexWrap: l.wrap ? "wrap" : "nowrap"` (`🟦️.tsx:892`). | `stack.wrap` never read; no wrap field anywhere in `UiStackNode`/`LayoutNodeKind::Stack`; no multi-line arrangement logic anywhere in `mounted_layout.rs`. | **DIVERGES totally.** Overflowing children never wrap on wgpu — a responsive toolbar overflows/clips instead of reflowing. |
| **Sizing (Hug/Fill/Fixed) on Leaf/Absolute** | `sizingToCss`: `Hug→auto`, `Fill→100%`, `Fixed(token)→rem` — applied on Leaf/Scroll/Absolute (`🟦️.tsx:853-858, 879, 925-926, 930`). | `LayoutSpec::Leaf(_)`/`Absolute(_)` matched with `_` in `stack_metrics` — **the inner `Sizing` is discarded entirely** (`🔀️reconcile/🦀️.rs:227`). No other wgpu code path reads `LeafLayout`/`AbsoluteLayout` fields outside tests. | **DIVERGES completely.** `Fill`/`Fixed` have no effect — every leaf sizes at raw intrinsic/measured size; a `Fill`-width spacer collapses toward zero instead of spanning its container. |
| **Grid layout kind** | Real CSS grid: `gridTemplateColumns`/`Rows` from `GridTrack`, independent `columnGap`/`rowGap`, `align`/`justify` (`🟦️.tsx:863-873, 896-910`). | `LayoutSpec::Grid` → `UiNode::Stack` with direction **hardcoded `"vertical"`**, only `row_gap` kept; columns/rows/`column_gap`/align/justify all discarded (`🔀️reconcile/🦀️.rs:224`). **No `UiNode::Grid` variant exists at all** (`🧩️component/🦀️.rs:3265-3286` — `Stack` is the only container). | **DIVERGES categorically.** An N-column grid renders as N side-by-side columns in React and as **one vertical stack of N rows** in wgpu — a structural rearrangement, not a pixel offset. |
| **Scroll layout kind** | `overflowX`/`Y` per `ScrollAxes`, real clipping + scrollbars (`🟦️.tsx:915-926`). | `LayoutSpec::Scroll` → `UiNode::Stack`, direction hardcoded `"vertical"`, only `padding` kept; `axes`/`sizing` discarded (`🔀️reconcile/🦀️.rs:225`). No `UiNode::Scroll` variant; no clip-region concept anywhere in `mounted_layout`. | **DIVERGES.** A scroll container becomes an ordinary unclipped in-flow stack; content overflows into siblings instead of being clipped/scrolled. |
| **Overlay layout kind** | `position:absolute`, `inset` from `EdgeSpace` (`🟦️.tsx:911-914`). React itself also does **not** read `anchor` here — only `inset` — a note, not a wgpu-specific gap. | `LayoutSpec::Overlay` → `UiNode::Stack`, hardcoded `"vertical"`, `inset` collapsed via the same one-side `edge_token`; **no `position:absolute` equivalent anywhere in `mounted_layout`** — the node is arranged as an ordinary flow child and pushes later siblings (`mounted_layout.rs:665-671`). | **DIVERGES fundamentally.** In React an overlay is taken out of flow; on wgpu it is an ordinary flow child that displaces siblings — a popover/tooltip/modal declared via `LayoutSpec::Overlay` shoves the rest of the layout down instead of floating over it. `anchor` (9-point placement) is unread on **both** sides for this generic path — Popover instead does its own DOM-rect positioning (§3) — so `Anchor` may be dead contract surface everywhere, worth a follow-up question to whoever owns `contract-layout`. |
| **Absolute layout kind** | `position:absolute`, explicit size from `sizingWidth`/`Height` (`🟦️.tsx:927-931`). | Same arm as Leaf — no positioning/sizing info survives at all (`🔀️reconcile/🦀️.rs:227`). | **DIVERGES fundamentally**, same in-flow-vs-out-of-flow problem as Overlay, plus total loss of explicit size. |
| **Intrinsic text sizing** | Native browser shaping (kerning, subpixel) — free. | `measure_text()` sums **per-codepoint** glyph advance via real parley/swash shaping (`📝️text/🦀️.rs:479-486`) — real font metrics, not a monospace-average hack, but **not kerning-aware** (acknowledged in the module's own header, lines 8-11). | Diverges slightly, in a well-understood, small way — proportional fonts (Anta/Kelly Slab) drift a little from browser-kerned widths; the genuinely monospace `Share Tech Mono` is unaffected. |
| **Text wrapping** | Native Unicode line-breaking (UAX#14). | `measure_text_wrapped()`: naive greedy word-wrap, re-measuring `current + " " + word` per word (`📝️text/🦀️.rs:490-503`). | Diverges marginally — same first-fit strategy browsers effectively use for simple Latin, but no true UAX#14 (CJK, hyphenation). |
| **Line height** | Tailwind-style per-size-token ratios, e.g. `--text-sm--line-height ≈ 1.5`, `--text-base ≈ 1.556` (`🎨️styling/🖌️ui/🎨️.css:806-810`). | **Hardcoded `size * 1.35`** in the wrapped path only (`📝️text/🦀️.rs:506`); the single-line path uses a **third, different** formula: `max_height.max(size)` — raw glyph bbox, no multiplier at all. | **DIVERGES, and is internally inconsistent within wgpu itself** — neither 1.35 nor ~1.0 matches React's ~1.4–1.56, and wrapped vs. unwrapped text on wgpu disagree with *each other* at the same font size. |
| **Font family fallback chains** | Full chains: `--font-sans: Anta, "Anta Extended", "Anta Math", "Anta Symbols", "Noto Emoji"...` (`🎨️styling/🎨️palette/🎨️.css:217-219`). | Registers only **one file per family** — `FAMILY_SANS="Anta"` etc., `🏛️latin/📖️regular/` only (`📝️text/🦀️.rs:41-48`) — though the Extended/Math/Symbols/Cyrillic `.ttf` files exist on disk unused. | Diverges for glyph coverage outside base Latin (math symbols, extended Latin, Cyrillic): React falls through correctly; wgpu falls all the way to the 8×16 ASCII bitmap fallback with different (fixed 8px-advance) metrics, changing measured width/wrap for that text. |
| **Font weight** | CSS `font-weight`, browser synthesizes bold if needed. | Never requests a bold `StyleProperty::Weight` (`📝️text/🦀️.rs:365-378`); also moot in practice — none of the three custom fonts ship a bold `.ttf` on disk at all (shared asset gap, not solely a code gap). | Minor, shared-root-cause divergence. |
| **Font size values (px)** | Compact-density CSS: `--text-xs:11.2px, --text-sm:12.8px, --text-base:14.4px` (`🎨️styling/🖌️ui/🎨️.css:804-810`). | `🔤️tokens/🦀️.rs:280-282` defines the **same** generated constants, wired at `🎨️theme/🦀️.rs:234-236`. | **MATCH — the one place both sides genuinely share a token source** rather than hand-duplicating. |
| **Density (Compact/Standard/Touch)** | Full second ramp exists under a `.touch` CSS class (`🎨️styling/🖌️ui/🎨️.css:630-643`), but React's own `layoutSpecStyle`/`spaceTokenRem` path is *itself* pinned to a fixed `COMPACT_UI_SPACING_REM` constant (`🎨️styling/🌓️theme/🟦️.ts:370-375`) rather than reading the live CSS custom property — so `LayoutSpec`-driven gap/padding is not actually touch-reactive on React either. | `Theme` has **no density parameter at all** — `Theme::light()`/`dark()` never take a `Density` (`🎨️theme/🦀️.rs:274-291`); every value is permanently the compact constant. | **DIVERGES in principle** (`StyleSpec.density` is a first-class contract field wgpu never reads) but **smaller in practice than it looks**, since React's layout-driven spacing is also compact-pinned; density-aware *class-based* CSS sizing (buttons/controls via `var(--ui-spacing)` directly) does respond to `.touch` on React with no wgpu equivalent at all. |
| **Icon size** | 4 tokens resolve to **12/20/24/32px** at 16px root (`🧱️elements/🔣️Icons/🟦️.tsx:34-45`, multipliers in `🎨️styling/🔣️.json:363-366`). | `TREE_ICON_SIZE: f32 = 14.0` — one hardcoded literal, duplicated verbatim in two files (`🖌️paint/🦀️.rs:45`, `🪀️widgets/🦀️.rs:207`), not derived from any of the four token multipliers. | **DIVERGES** — 14px matches none of React's four tokens (closest is `tiny`=12px, off by 2px); a tree row's icon would typically use `small`=20px in React. A clear hand-duplicated, drifted magic number. |
| **min/max width/height reset** | Every layout kind sets `minWidth:0, minHeight:0` unconditionally — a standard CSS flex-overflow guard, not contract-driven. | No equivalent clamp anywhere in the measure/arrange passes. | Not contract-driven, but a real failure-mode difference: deeply nested wgpu content that exceeds its parent's bounds simply overflows uncontained, where React's children compress toward 0 first. |

### 5.3 Two ticket-listed files were mis-scoped for this lane (noted for the record)

- `🏷️label/🦀️.rs` (191 lines) is **not** about label sizing/layout — it's the compile-time-checked
  i18n string type (`Label`/`LabelText`/`LocalizedLabel`/`AppLabels`). No layout code lives there.
- `🔣️icon-name/🧾️value/🦀️.rs` (40 lines) is **not** about icon sizing — it's a hand-written
  `ToValue`/`FromValue` wire-codec bridge for the generated `IconName` enum (naming only). The actual
  icon-size constants are in `🖌️paint/🦀️.rs:45` and `🪀️widgets/🦀️.rs:207` (§5.2 table).
- `IconName` itself, incidentally, is a genuine shared-source win: a single generated 249-variant,
  2344-line enum (`🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`) consumed
  identically by both targets — **OK/parity**, worth stating since so much else in this section isn't.

## 6. Accessibility, keyboard focus, localisation

**Accessibility: OK — genuine, well-engineered shared-source parity.**
`🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs` (89 lines) is a complete pre-order projection
(`accessibility_projection`, lines 43-67) built from the retained arena, stamping live
`focused`/`rect` state and reusing the **shared** `ui_contract::accessibility_role`/
`accessibility_is_focusable`/`liveness_name` (`🧬️contract/♿️accessibility/🦀️.rs:67-129`) — role and
focusability are computed **once**, in the contract crate, and consumed identically by both targets.
React's `accessibilityAriaProps` (`🗣️Interpreter/🟦️.tsx:956-963`) maps the same `AccessibilitySpec` to
real `aria-*` DOM attributes (`role="button"` at 1150, `role="application"` for Surface/canvas parity
at 2062, `role="progressbar"` with `aria-valuemin/max/now/valuetext` at 2105, `aria-busy` for loading
at 2149). Both sides are proven against the **same shared fixture**
(`🧬️contract/🧫️fixtures/♿️accessibility-projection.json`, which names both the Rust and the TypeScript
projection functions explicitly as the answer key). This is one of the strongest-engineered parity
surfaces found in the whole audit.

**Keyboard focus: substantial real infrastructure on both sides, but with one confirmed no-op seam.**
wgpu `⚡️events/🦀️.rs` implements a full Tab-order cycle — `is_focusable`/`collect_focusable`
(288-295), `set_focus`/`clear_focus`/`focus_next`/`focus_prev` (315-381), dialog focus-trap bounding
(`topmost_focus_trap_root`, 671; `"Tab"` handling, 1510) — built from the **same**
`accessibility_is_focusable` set the shared contract defines, so focus and ARIA "focusable" cannot
disagree by construction. React relies on native DOM tab order plus one explicit case, `Component::
Surface` (`tabIndex={record.disabled ? -1 : 0}`, `🗣️Interpreter/🟦️.tsx:2063`, with a comment at
2040-2047 explaining why a canvas needs an explicit `tabIndex`). This is a deliberate, matched design.
However: `NodeFlags::FOCUSED` exists on the wgpu Interpreter target
(`🎯️targets/🧊️wgpu/🦀️.rs:2075`, used only for a debug `focus_path` dump), and **`UiCommand::
FocusChanged` is an explicit intentional no-op** at `🗗️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:497`
(`UiCommand::FocusChanged { .. } => {}`), with a doc comment (466-467) deferring the real work to a
separate, off-limits `shell::ShellInput` region / sibling workstream `w3-shell-input-cutover`'s `note_
content_focus_commands`. **Status: P1 missing/deliberately deferred** — the seam is real and worth a
follow-up audit of that specific shell-input subsystem, which this lane did not cover.

**Localisation: mostly OK; one named file is a red herring, and both sides may have a small unaudited
gap.**
`🌐️locale-terminology/🧾️value/🦀️.rs` (49 lines) really is trivial — it's *only* a `ToValue`/
`FromValue` wire bridge for the generated `Locale`/`Terminology` enums, confirmed by direct read (it's
just the Locale/Terminology enum definitions plus round-trip impls, nothing sizing- or layout-related).
The real terminology/locale resolution matrix lives one directory over, in `🏷️label/🦀️.rs` (191
lines): a full `Locale × Terminology` resolution function (`resolve`, lines 91-140) every wgpu label
goes through. Both `Locale{En,De}` and `Terminology{Native,Reuse}` are generated from the **same**
`🖱️ui/🎚️axes/🔣️.json` source that also generates React's `SHELL_LOCALES`/`SHELL_TERMINOLOGIES`
(`🎚️axes/📽️projection/🟦️.ts:68-77`) — genuine single-source-of-truth parity. One open question flagged
in passing, not confirmed either way: React's `UiLabelPair` (`📚️I18n/🟦️.tsx:15-19`, `{normal,
beginner}`) looks like a *separate*, additional experience-level axis with no wgpu counterpart found in
this pass — worth a dedicated look, not asserted as a gap here.

## 7. Wire format — guest ↔ renderer transport

*(This section was independently traced end-to-end in a separate research pass; citations below are
from that trace, cross-checked against the `UiPatch`/`apply_patch` reads this lane did directly.)*

### 7.1 The document/patch transport: binary "pack" (OpBinary), never JSON on the wire

The retained document's transactional unit is `UiPatch` (`🧬️contract/📃️document/🦀️.rs:287-292`:
`surface`, `base_revision`, `revision`, `ops: UiPatchOps`), whose ops (`UiPatchOp`, lines 210-292)
cover `Upsert(UiNodeRecord)`, `SetComponent`, `SetLayout`, `SetActivity`, `SetChildren`, and more — a
real diff protocol, not full-tree replacement, applied transactionally via `apply_patch()`
(`🧬️contract/🛡️limits/🦀️.rs:1182`: base-revision check → shadow map → validate → commit-or-reject-
whole).

The actual bytes-on-the-wire format is defined in the plugin WIT schema, not in the `ui` module tree:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:10` declares `type pack =
list<u8>`, and the `interface ui` block (~lines 812-903) types every patch-op payload field
(`patch-upsert.node`, `patch-set-component.component`, `patch-set-layout.layout`, etc.) as `pack` —
raw bytes, **not** a JSON string type. Only `patch-set-children` carries structured `list<node-id>`
(u64s) rather than a pack blob. This confirms the project's own prior finding (memory note "Retained
Tool Wire Is Binary Op Encoding") applies directly to UI patches, not just to tool-run pages: the
binary encoder/decoder is `pack_rt::encode_wire_value`/`decode_wire_value`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:5420-5432`, calling
`os_pack::encode_record_body`/`decode_value_record_body_exact`), with `OP_BINARY_FORMAT: u8 = 1`
(line 5382) as the format-byte convention every encoded operation starts with. Note: grepping the
`🖱️ui` module tree alone for the literal string `OpBinary` returns **zero hits** — the convention
lives in `store`/`plugin/reactor`/`replication`, and the `ui` side only shows up via
`encode_record_body`/`decode_record_body` calls (e.g. `🖱️ui/🎬️scene/📦️pack/🦀️.rs:2`). A future
grep limited to `🖱️ui` for `OpBinary` will wrongly conclude the module doesn't use it.

Host-side, the pack bytes are decoded and bridged into the typed Rust contract structs at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs`: `decode_pack` (lines
39-44) calls `pack_rt::decode_wire_value` → `DslValue`, **asserts the bytes are the canonical
re-encoding of that value** (rejects as "non-canonical pack" otherwise), then bridges through
`serde_json::Value`/`serde_json::from_value` into `UiNodeRecord`/`Component`/`LayoutSpec` etc. — i.e.
JSON appears only as an **in-process intermediate representation** for this one conversion step, never
on the actual wire. `wit_ui_patches_to_kernel` (lines 92-115) is the single point that moves decoded
patches into the retained kernel turn owner; its own doc comment (line 91) records a **since-fixed
historical bug**: until 2026-09-15 it admitted exactly one patch per turn and silently dropped the
rest, worth knowing if any stale-looking UI state resurfaces in old bug reports.

Full hop-by-hop shape, confirmed by direct read at every arrow:

```
guest wasm SDK (SurfaceReconciler, 🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs)
  diffs its own ComponentTree against a shadow UiSnapshotState mirror, keyed by (parent, key)
  → ui_contract::UiPatch, each op field individually pack/OpBinary-encoded to raw bytes
  → crosses the WIT boundary as wit_ui::UiPatch (fields typed `pack = list<u8>`)
  → host: 🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs decodes pack→DslValue→serde_json::Value→typed struct,
    verifies canonical round-trip, pushes into UiTurnPatches
  → kernel applies the patch transactionally against the canonical UiSnapshotState
    (flat, id-keyed UiNodeRecord table) — the ONE source of truth both renderers read
  → React: UiDocumentStore (TS) receives the snapshot/patch as plain JSON objects at this point
    and applies them with the same (parent,key)-identity algorithm as the Rust UiSnapshotState
  → wgpu: 🔀️reconcile/🦀️.rs reads the SAME in-process Rust UiNodeRecord/Component structs
    directly — no further (de)serialization — and projects them into its own UiNode tree
```

One concrete, **directly confirmed** exception to "never JSON on the wire": `Component::Extension`'s
`props` field is serialized with **literal `serde_json::to_string(&props.props)`** at the point wgpu
builds a `UiExternalSlotNode` (`🔀️reconcile/🦀️.rs`, Extension arm, ~line 828) — regardless of what
binary envelope wraps the rest of a `UiPatch`, a guest plugin's own extension-slot parameters cross
this specific seam as plain JSON text.

A **second, unrelated** golden-JSON format exists purely for wgpu's own internal `UiNode` tree shape —
locked by `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/
🦀️.rs:149-152` (`serde_json::to_string(&node) == GOLDEN_UI_NODE_TREE_JSON`, plus a round-trip
assertion). This is a **test/regression-guard artifact**, never sent by a guest — do not confuse it
with the transport above; it exists to catch accidental struct/field renames in the wgpu-native type.

Two files the task's brief named as likely wire-format carriers are, on direct inspection, about
something else entirely — corrected here so a future auditor doesn't repeat the detour:
- `🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs` (90 lines) is the generated `Locale`/`Terminology` enum pair
  (`// @generated from 🖱️ui/🎚️axes/🔣️.json`), unrelated to `Component`/`UiNode` serialization.
- `🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` (3242 lines) is a GPU draw-command scheduling/credit system
  (`PreparedRenderLimits`: max draw items/bytes, max upload items/bytes; `PreparedRenderCommand`/
  `DrawList`/`Mesh3dLease`) — downstream of layout/paint, not upstream of it, and has no `UiNode`
  reference at all.

The retirement layer (`🧬️contract/♻️retirement`, §-referenced elsewhere in this report) is **not** a
renderer-facing diff stream either — confirmed by grep, it has zero references anywhere under
`🎯️targets/🧊️wgpu` or in the TS/React side. It is exclusively about fixed-arena byte/value ownership
for pack-encoded payloads inside the **guest SDK's own** retained tree (`🧠️runtime/♻️retirement/
🌲️tree/🦀️.rs`) and host-side plugin glue — a memory-lifecycle detail orthogonal to renderer parity,
not a second document-diff mechanism.

**Status: OK / well-understood.** The document transport is real, shared, binary, and correctly
described by prior project memory; the one confirmed textual-JSON exception (`Extension.props`) is
narrow and citable. This part of the wire is **not** where this audit's biggest risk lives — §7.2 is.

### 7.2 Action/event dispatch back to the guest — the biggest asymmetry found in this whole audit

This is a **separate wire**, upstream-to-downstream reversed (renderer → guest), and it is where a
major, previously-unflagged P0 lives.

wgpu's outbound side is `🎯️targets/🧊️wgpu/🎬️action/🦀️.rs`: a `BoundedActionQueue`/
`BoundedActionReservation` (fixed-capacity ring buffer, 256 items / 1 MiB, lines 6-13, 613-650)
carrying `ActionDescriptor { controller_id, action, args: Option<DslValue> }`
(`🧩️component/🦀️.rs:28-31`). Every single interactive `Component` kind wgpu reconciles — Button,
Input, Select, Toggle, KeyValueList, Slider, NumberStepper, Ring, IconSelect, Tree, drag/drop — is
wired through this queue via `record_action`/`record_action_or_inert`
(`🔀️reconcile/🦀️.rs:274-287`), whose **own doc comment self-identifies it as legacy**: "The record's
binding for `trigger`, as the **legacy** `ActionDescriptor` the wgpu paint/event path dispatches."

React's `Interpreter/🟦️.tsx` has **two** channels, and only one of them is what wgpu uses:
- `UiInterpreterContext.onAction: (action: ActionDescriptor) => void` (line 665-668) — explicitly
  documented as **legacy**: "the seam this Interpreter still speaks to the **14 unowned scene-host
  elements** through... semantic components (button/input/select/…) **never use this**; they go
  through `emitIntent`/`UiIntent` instead."
- `onIntent: (intent: UiIntent) => void | Promise<void>` (line 671-674) — the **modern** path every
  semantic control (Button/Input/Select/Toggle/Slider/NumberStepper/Ring/IconSelect/Tree/…) actually
  dispatches through today.

`UiIntent`'s real shape (`🧬️contract/🧬️schema/🦀️.rs:937-963`): `{ surface, revision, node, nodeKey,
trigger, action, args, input, seq }`. The fields `ActionDescriptor` simply does not have are exactly
the ones that make an event pipeline robust under latency/reorder: `revision` lets the runtime **drop
stale intents** fired against geometry the user never actually saw (comment, lines 939-941); `seq` is
"Renderer-monotonic per surface — lets the runtime order and de-duplicate intents independently of
transport delivery order" (lines 959-962); and `input` is a payload field kept separate from `args` so
a trigger's live value (Change's new value, Delta's step count, Drop's payload) doesn't get conflated
with a binding's static arguments (lines 954-957). `ActionDescriptor` conflates all of this into one
optional `args: DslValue` and carries none of the staleness/ordering protection.

Grepping the whole `🎯️targets/🧊️wgpu` tree for `UiIntent` or `coalesce_key`/`coalesceKey` returns
**zero hits**. wgpu's entire interactive surface therefore runs on the channel React's own codebase
now reserves for 14 legacy, unowned scene-host elements — no stale-intent rejection, no seq-based
ordering or de-duplication, no `nodeKey` identity check. This is not a missing-variant bug like the
rest of this report; it's a whole dispatch-generation gap between the two renderers, and it plausibly
explains any hard-to-reproduce "wrong control fired" or "double-fired action" reports on wgpu that
don't reproduce on React. **Treat this as the single highest-priority non-layout finding in this
audit** — see Work Packet 2 below.

### 7.3 Other reconcile-bridge findings worth carrying forward

- **`Component::Extension` → `UiExternalSlotNode.plugin_id` is hardcoded to `""`** — confirmed as a
  real, unused field, not a props artifact: `ExtensionProps` (`🧬️contract/🧩️component/🦀️.rs:580-587`)
  has no `plugin_id` at all (only `extension`/`props`), yet `UiExternalSlotNode.plugin_id`
  (`🧩️component/🦀️.rs:3175`) is a real field consumed elsewhere for action-id composition
  (`menu.open-with.{plugin_id}.{app_id}...`, lines 1780-1794) and correctly populated by the
  hand-written constructor `ui_external_slot(plugin_id, ...)` (line 3516-3517). Both current paint call
  sites for `ExternalSlot` (`🖌️paint/🦀️.rs:1113` production, `:2414` `#[cfg(test)]`-only) never read
  `.plugin_id` — so this is latent, not yet symptomatic, but will silently misbehave (empty plugin id)
  the moment the deferred `program_bridge` wiring starts reading it. `🔀️reconcile/🦀️.rs:827` vs.
  `🧩️component/🦀️.rs:3516-3517,1780-1794`.
- **`Select`'s option rows always materialize regardless of open/closed state** — self-documented as a
  known gap in the reconcile file's own header (`🔀️reconcile/🦀️.rs:8-13`): `tree::WidgetState` is
  currently a zero-field marker with nowhere to record "is this Select open," so the synthesized rows
  build unconditionally rather than only while a dropdown is actually visible.
- **Quarantined-plugin recovery UI has no React equivalent.** `ui_recovery_panel(plugin_id,
  quarantined, is_de)` (`🧩️component/🦀️.rs:3729-3740`) hand-builds a `UiNode` tree **in native Rust**,
  entirely bypassing `Component`/`UiPatch`/reconcile, called in production at
  `🛍️products/💻️os/🖥️host/🦀️.rs:227-231` whenever a plugin is quarantined. Grepping the whole
  `📺️renderer` module for "quarantin" (case-insensitive) finds only unrelated "surface quarantine"
  hits in presenter-admission fixtures — no React-side plugin-quarantine recovery screen was found in
  this pass. Since this `UiNode` is synthesized natively and injected straight into the arena, it never
  becomes a `Component`/`UiPatch` and is structurally invisible to `UiDocumentStore`. Flagged as an
  open question (a wider grep across the whole React `os` product, not just the renderer module, would
  be needed to rule out a differently-named equivalent before treating this as a confirmed gap).

## 8. Findings ranked

1. **P0 — `LayoutSpec`'s expressive surface is mostly unread in production wgpu**, because the real
   Taffy flex engine (`📐️flex/🦀️.rs`) is `#[cfg(test)]`-only dead code; production layout has no
   align/justify/wrap/grow, collapses Grid/Scroll/Overlay/Absolute into a plain vertical Stack, and
   collapses the 7-value spacing ramp into 3 mismatched, independently-hardcoded pixel buckets. This is
   very likely the single largest source of "elements in the wrong place" between the two renderers.
   §5.1–5.2.
2. **P0 — wgpu's entire interactive surface dispatches through React's own self-described "legacy"
   `ActionDescriptor` channel, never the modern `UiIntent` protocol.** Every control wgpu reconciles
   (Button/Input/Select/Toggle/KeyValueList/Slider/NumberStepper/Ring/IconSelect/Tree/drag-drop) goes
   through `record_action`/`record_action_or_inert` (`🔀️reconcile/🦀️.rs:274-287`, itself doc-commented
   as legacy) into a plain `ActionDescriptor{controller_id, action, args}`
   (`🧩️component/🦀️.rs:28-31`), while React reserves that exact channel for 14 unowned scene-host
   elements only and routes every semantic control through `UiIntent` instead
   (`🗣️Interpreter/🟦️.tsx:665-674`), which carries `revision`/`seq`/a separate `input` field for
   stale-intent rejection and per-surface ordering/de-duplication (`🧬️contract/🧬️schema/🦀️.rs:937-963`)
   that `ActionDescriptor` has no equivalent of at all. Zero `UiIntent`/`coalesce_key` hits anywhere in
   `🎯️targets/🧊️wgpu`. This is a whole dispatch-generation gap, not a missing-field bug, and is the
   most plausible root cause of any hard-to-reproduce "wrong control fired"/"double-fired action"
   behavior that's wgpu-only. §7.2.
3. **P0 — `Component::Image` never shows the actual picture on wgpu's default (non-Surface) path** —
   only a placeholder box; the real texture drawer is test-only dead code with no confirmed host wiring.
   §2.
4. **P0 — the CAD plugin's spatial-tree editor screen has no wgpu implementation at all**, not just
   missing widgets (no `🎯️targets/🧊️wgpu` sibling under its `⚙️engine/`, independently re-verified).
   §3.
5. **P1 — Input's `min`/`max`/`step`/`accept` are carried on the wire and silently dropped at wgpu's
   render call site** (`..` in the destructure). §2.
6. **P1 — Tree virtualized/streaming windowing** — wgpu paints a window but never requests one; already
   tracked at `26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING` packet P5, cited not duplicated. §2.
7. **P1 — Popover/Dialog overlay positioning scaffolding exists (`OverlayKind`/`resolve_overlay_
   placement`) but is never consumed by paint** — no overlay content actually renders. §3.
8. **P1 — no hover-tooltip mechanism anywhere in the wgpu target**, while React has a full tiered
   reveal system layered onto every control generically. §3.
9. **P1 — keyboard `FocusChanged` is an explicit no-op** in the audited wgpu Interpreter target,
   deferred to an unaudited sibling shell-input workstream. §6.
10. **P2 — NumberStepper's historical binding-gating bug (ticket 26/09/02 wave B12) is not confirmably
    fixed the same way on wgpu** — `on_absolute`/`on_delta` both wire unconditionally, a direct symptom
    of finding #2's legacy channel having no binding-presence discipline. §2, §7.2.
11. **P2 — `Component::Extension` → `UiExternalSlotNode.plugin_id` is hardcoded to `""`**, a latent
    dead-field bug that will misbehave silently once deferred `program_bridge` wiring starts reading
    it; and **`Select`'s option rows always materialize regardless of open/closed state**, a gap the
    reconcile file's own header already documents as pending on a `WidgetState.open` field. §7.3.
12. **P2 — icon size and text line-height are hardcoded, drifted magic numbers** not derived from the
    shared token source (`TREE_ICON_SIZE=14.0` matches none of React's 4 tokens; wgpu's own two line-
    height formulas disagree with each other, let alone with React). §5.2.
13. **P2 — Select has no keyboard/search/collision-aware positioning**; Input's `date`/`color`/`file`
    kinds have no native-picker equivalent (expected, given an immediate-mode canvas has no OS pickers
    for free, but worth stating as a bounded, accepted gap rather than an oversight). §2.
14. **Open question, not confirmed either way** — quarantined-plugin recovery UI (`ui_recovery_panel`,
    `🧩️component/🦀️.rs:3729-3740`) is hand-built native `UiNode`, structurally invisible to
    `UiDocumentStore`; no React-side equivalent found in this pass, but a wider grep across the whole
    React `os` product (not just the renderer module) is needed before calling this a confirmed gap.
    §7.3.
15. Everything in §3 marked **N/A** (Table-as-generic-widget, Tabs, List, Chip) is dead/unused on the
    React side too — do not spend implementation effort chasing these; if anything, flag them to the
    element-inventory owner as candidates for deletion.

## 9. Recommended work packets

Ordered roughly by leverage; file seams chosen to minimize overlap between packets so they can run in
parallel.

1. **Wire the real flex engine into production.** Seam: `🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` (remove
   `#[cfg(test)]` gating from `LayoutEngine`/`style_for`/`apply_*_metrics`), `⚙️engine/🦀️.rs`
   `frame_step`/`progressive_layout_preview` (swap the call from the `mounted_layout` crude path to
   `LayoutEngine::compute`), `📌️mounted_layout/🦀️.rs` (retire or reconcile `LayoutNodeKind::Stack`'s
   hand-rolled arrange step once the taffy path is live). React reference: `layoutSpecStyle`,
   `🗣️Interpreter/🟦️.tsx:875-934` (the field-for-field target behavior). Acceptance: a fixture Stack
   with `align: End, justify: SpaceBetween, wrap: true` lays out children in visibly different
   positions than today's stretch/space-fill-everything behavior, matching React's computed
   pixel rects within the flex engine's own existing unit-test tolerances (`🧪️tests/🔬️targets-wgpu-
   flex-unit/🦀️.rs` already has fixtures to extend).
2. **Migrate wgpu's action dispatch from legacy `ActionDescriptor` to `UiIntent`.** Seam:
   `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:274-287` (`record_action`/`record_action_or_inert` — build a
   `UiIntent{surface, revision, node, nodeKey, trigger, action, args, input, seq}` instead of an
   `ActionDescriptor`), `🎯️targets/🧊️wgpu/🎬️action/🦀️.rs` (`BoundedActionQueue` — carry `UiIntent`,
   add the revision-staleness check and per-surface `seq` de-duplication React's runtime already does).
   Folding in the NumberStepper fix from finding #10 as part of the same seam: gate `on_delta` on
   binding presence exactly like `🗣️Interpreter/🟦️.tsx:1353` does, which this migration should make
   natural once `UiIntent`'s `input` field replaces the current unconditional dual-wire. React
   reference: `UiInterpreterContext.onIntent`, `🗣️Interpreter/🟦️.tsx:665-674`, and `UiIntent`'s shape,
   `🧬️contract/🧬️schema/🦀️.rs:937-963`. Acceptance: an intent fired against a stale revision (the
   user scrolled/re-rendered before the event was processed) is dropped on wgpu the same way React
   drops it; two rapid-fire clicks on the same control are ordered/de-duplicated by `seq` instead of
   racing; a stepper with only a `Trigger::Change` binding no longer emits a delta action.
3. **Restore the SpaceToken → pixel scale.** Seam: `🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs`
   `gap_for_token`/`padding_for_token` (replace the 3-bucket tight/loose collapse with the full 7-value
   ramp, ideally by consuming `🖌️render/📏️layout/🦀️.rs`'s existing `4/8/12/16/24/32`px resolution
   function directly instead of a third hand-duplicated scale), `🔀️reconcile/🦀️.rs:199-213`
   (`space_token`/`edge_token` — stop collapsing to `"tight"/"loose"` strings; either pass the token
   through structurally or resolve to a number at this boundary). React reference: `spaceTokenRem`/
   `SPACE_TOKEN_MULTIPLIER`, `🗣️Interpreter/🟦️.tsx:837-845`. Acceptance: all 7 `SpaceToken` values
   produce 7 visibly distinct gap widths in a wgpu screenshot test, matching React's ramp in ratio.
4. **Add real Grid/Scroll/Overlay/Absolute container kinds to `UiNode`.** Seam:
   `🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` (new `UiNode` variants alongside `Stack`), `🔀️reconcile/
   🦀️.rs:224-227` (stop discarding `LayoutSpec::Grid/Scroll/Overlay/Absolute` into a fake vertical
   Stack), `📌️mounted_layout/🦀️.rs` (measure/arrange passes for the new kinds — at minimum, Overlay
   must stop contributing to sibling offset). React reference: `layoutSpecStyle`'s `grid`/`scroll`/
   `overlay`/`absolute` cases, `🗣️Interpreter/🟦️.tsx:896-933`. Acceptance: a 2-column Grid renders as
   2 columns (not 2 rows) in a wgpu screenshot; a Dialog/Popover declared via `Overlay` no longer
   pushes its siblings down.
5. **Fix Input's dropped constraint fields.** Seam: `🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:325-328`
   (destructure and actually use `min`/`max`/`step`/`accept` instead of `..`), plus whatever paint call
   this feeds visually clamps/labels the value. React reference: `InputView`,
   `🗣️Interpreter/🟦️.tsx:1263-1266`. Acceptance: a numeric wgpu input clamps to its declared min/max
   the same way the browser's native `<input type=number>` does.
6. **Fix `Component::Image`'s default paint path.** Seam: `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1066-1089`
   (the live `has_scene_host==false` arm) and `2385-2398` (`fn paint_image`, remove `#[cfg(test)]` and
   wire real texture upload, coordinating with whatever `program_bridge`/`engine_canvas` hook it
   depends on per its own doc comment). React reference: `ImageView`, `🗣️Interpreter/🟦️.tsx:1971-1973`.
   Acceptance: a plugin emitting `Component::Image{src}` for a real bitmap shows that bitmap in a wgpu
   screenshot, not a placeholder box.
7. **Wire Popover/Dialog overlay content to paint.** Seam: `🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
   (`OverlayKind`/`OverlayPlacement`/`resolve_overlay_placement`, already present ~line 546+) plumbed
   into `🖌️paint/🦀️.rs` and `⚙️engine/🦀️.rs`'s per-frame pipeline so an overlay actually draws.
   React reference: `🧱️elements/🗨️Popover/🟦️.tsx`, `🧱️elements/💬️Dialog/🟦️.tsx`. Acceptance: a Group/
   Section marked as an overlay-anchored popover actually appears floating over content in wgpu,
   dismissible the same way React's is.
8. **Add a hover-tooltip mechanism.** Seam: new code in `🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` (hit-test +
   dwell timer) and `🖌️paint/🦀️.rs` (delayed popup draw), reading the same `AccessibilitySpec`/label
   tiers the contract already carries. React reference: `UiDriverTooltips`,
   `🎯️targets/⚛️react/🟦️.tsx:1525-1613`. Acceptance: hovering a control with a tooltip-tier label for
   the dwell duration shows a popup in wgpu matching React's reveal timing.
9. **Un-hardcode icon size and line-height from the shared token source.** Seam:
   `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:45` and `🪀️widgets/🦀️.rs:207` (`TREE_ICON_SIZE` → read from
   `ui_styling` icon tokens the way `🎨️theme/🦀️.rs:234-236` already does for font sizes),
   `📝️text/🦀️.rs:506` (unify the wrapped and single-line line-height formulas and derive the multiplier
   from the same CSS ratio React uses). React reference: `🧱️elements/🔣️Icons/🟦️.tsx:34-45`,
   `🎨️styling/🖌️ui/🎨️.css:804-810`. Acceptance: wgpu icon sizes and wrapped-paragraph heights match
   React's within rounding, for all 4 icon-size tokens and the 3 font-size tokens.
10. **Add Select keyboard navigation and collision-aware menu placement.** Seam:
    `🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs` (`render_select`/`render_select_menu`). React
    reference: `SelectView`, `🗣️Interpreter/🟦️.tsx:1274-1290` (Radix `<Select>` behavior). Acceptance:
    arrow keys move the highlighted row in an open wgpu select menu, and the menu flips above the
    control when there isn't enough room below.
11. **Add a loading-skeleton placeholder shape.** Seam: `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` (new
    per-`Component`-kind skeleton draw, keyed the same way `elementSkeleton` is). React reference:
    `elementSkeleton`, called at `🗣️Interpreter/🟦️.tsx:2150`. Acceptance: a node with `UiStatus::
    Loading` and no committed content shows a shaped placeholder in wgpu, not just a border tint.
12. **Fix the two latent reconcile-bridge field bugs.** Seam:
    `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:827` (thread a real `plugin_id` into `UiExternalSlotNode`
    instead of `String::new()` — the caller/context that knows the extension's owning plugin id needs
    to be threaded down to `ui_node_from_record`), and the same file's `select_node()` path once
    `🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs`'s `WidgetState` grows an `open: bool` field (tracked in the
    reconcile file's own header comment, lines 8-13) — gate synthesized option-row construction on it.
    React reference: n/a (these are wgpu-internal latent bugs, not parity gaps against React).
    Acceptance: an `ExternalSlot`'s `plugin_id` round-trips correctly through `menu.open-with.*` action
    ids once `program_bridge` reads it; a closed Select's option rows are not built/measured every
    frame.
