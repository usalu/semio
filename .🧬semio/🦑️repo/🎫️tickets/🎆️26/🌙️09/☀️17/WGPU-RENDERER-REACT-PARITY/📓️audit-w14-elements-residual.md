# W14 — Residual element/component coverage gap list (React ↔ wgpu)

Read-only audit, 2026-09-18. Sources: `📓️audit-interpreter-elements.md` (Wave 0, 2026-09-17) as the
baseline coverage matrix, re-verified by direct grep/read against the tree **as it stands now** —
after W1l (flex/grid/scroll/overlay/absolute layout), W1m (UiIntent dispatch), W1n (Image/Popover/
Dialog/Tooltip/Skeleton), W1o (widget metrics + Select keyboard), W2f (CAD engine port) and W2k (RTL,
chrome polish, mono, icon re-raster, puzzle5d, Tabs twin) closed a large fraction of the Wave-0
findings. This report does **not** re-list what those packets closed (confirmed closed by direct
read, cited inline where relevant) — it lists what a fresh grep still finds open, plus a handful of
items the Wave-0 pass didn't reach (WidgetNode kit's residual variant gap, chrome-commit min/max/step
wiring, CAD post-W2f hand-offs).

React = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
(Interpreter target) + `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (ui module's own react
target) + `ui_contract::Component` schema (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/`).
wgpu = `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/*` + os renderer's
`…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`.

Legend: **P0** = missing behaviour (a real thing the user can do/see on React that does nothing or
renders nothing on wgpu). **P1** = drift (both sides do something, results differ materially). **P2**
= polish (cosmetic/edge-case, low user impact).

---

## 1. Closed since Wave 0 (verified, not re-audited below)

Stated once here so the tables below aren't cluttered with "OK" rows already covered in the baseline
audit. Confirmed by direct read of current source, not by trusting the packet reports:

- **Layout**: `align`/`justify`/`wrap`/`grow`/per-side padding/full 7-value `SpaceToken` ramp/Grid
  (2 columns render as 2 columns)/Scroll (clips)/Overlay (out-of-flow)/Absolute — all live in
  `📐️flex/🦀️.rs`, no `#[cfg(test)]` left except the trailing `mod tests` (W1l).
- **Dispatch**: `UiIntent` (revision-staleness, per-surface `seq`, separate `args`/`input`) now the
  live path for every document-sourced control; `record_action`/`ActionDescriptor` is the deliberate
  fallback for scene-embedded/no-intent chrome only, not the whole surface (W1m).
- **Image**: `data:image/png` decodes and paints a real raster quad (`admit_ui_image`/
  `push_raster_quad`, `🖌️paint/🦀️.rs` region `🖼️UiImageSources`); `http(s)://`/relative `src` defers to
  the existing host asset pipeline (W1n).
- **Popover/Dialog**: `resolve_anchored_placement` is a field-for-field port of React's
  `resolvePopoverPlacement` (side/align/offset/flip/clamp), now the ONE positioner in the repo — the
  second, diverged copy in `🖌️render/🖱️dispatch/🦀️.rs` was retired to a 1-line shim over the same
  contract function (W1n, W2k §1).
- **Tooltips**: 400 ms dwell, debounced hover-out, `formatControlTooltipText`-equivalent label
  resolution, same positioner (W1n).
- **Loading skeletons**: 9-shape `skeleton_kind` replaces content (not just a border tint) under
  `UiStatus::Loading`, `Progress` exempted like React (W1n).
- **Disabled cursor**: `NotAllowed` before any drag/text affordance (W1n).
- **Icon sizes / line-height / progress geometry / select row metrics / focus-ring colour**: all
  token-derived, matching React's px values (W1o).
- **Select keyboard**: closed-state open-on-key, open-state arrow/Home/End/PageUp/PageDown/typeahead
  (with NFKD-equivalent accent folding), `Tab` closes-and-moves-focus — full port (W1o).
- **Slider/Toggle/NumberStepper keys**: arrow/Home/End stepping, Enter/Space toggle — ported (W1o).
- **`focus-visible`**: `NodeFlags::FOCUS_VISIBLE` stamped only on keyboard-driven focus, read by
  `focus_ring_visible()` at all 11+ paint sites (`🖌️paint/🦀️.rs:2041-2042`, confirmed live) — a
  pointer-focused control no longer shows the accent ring (W2k §3c).
- **RTL vocabulary**: `UiFlow`/`FlowInline`/`FlowBlock` exist and drive the ONE positioner + Select
  inline edge + mirrored arrow-key handling — dock-anchor-derived, matching React's own model (W2k §2).
  *(Not wired to any production window yet — see §5 below, this is a real residual gap, not fully
  closed.)*
- **`mono` theme**: derives from the same generated `ChromePalette`/token source as `light`/`dark`,
  21 hand-literal lines deleted (W2k §4).
- **Icon re-raster on density change**: bounded `IconAtlasRebuild` job, no longer boot-only (W2k §5).
- **Tabs**: a real wgpu twin now exists (`🧱️elements/📑️Tabs/🎯️targets/🧊️wgpu/🦀️.rs`, confirmed present
  by directory listing) for the one production non-window-dock consumer found (W2k §8).
- **puzzle 5D pose solver**: ported to Rust (`🖐️5d/…/📐️geometry/🦀️.rs`), proven against React's own
  oracle fixture (W2k §7).

---

## 2. Interactive controls

| Kind | Prop/behaviour | React (file:line) | wgpu (file:line) | Status |
|---|---|---|---|---|
| **Input** | `min`/`max`/`step` clamp/snap at commit | `🗣️Interpreter/🟦️.tsx:1263-1266` | `InputMeta::commit_value` implements clamp+snap and is unit-tested (`⚡️events/🦀️.rs`, W1m) | **P1 — primitive exists but the chrome shell's own commit path bypasses it.** `commit_focused_input`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11038`, still dispatches raw `"value": input.text_view()` — a numeric shell-chrome input can commit an out-of-range/off-step value the equivalent React `<input type=number>` would clamp. (W1m gap G6, confirmed still open by grep — zero `commit_value`/`InputMeta` hits in that file.) |
| **Input** | `date`/`color`/`file` native pickers | real OS pickers, free | none (immediate-mode canvas) | P2 — bounded/accepted gap, stated in W1m/W1o, unchanged |
| **Input** | `accept` reaches a file dialog | native `<input accept>` | `InputMeta.accept`/`UiInputNode.accept` carried on the wire, zero consumer (no file picker exists) | P2 — accepted gap (W1m G5) |
| **NumberStepper** | delta fires only when a `Trigger::Delta` binding exists | `🗣️Interpreter/🟦️.tsx:1353` | Gated on the **intent** path (`pointer_commit_action`/`FiredAction`, tested: `a_stepper_takes_the_relative_path_only_when_it_declares_a_delta_binding`). BUT `number_stepper_node`'s legacy `ActionDescriptor` fields still call `record_action_or_inert(record, Trigger::Delta, …)` unconditionally, `🔀️reconcile/🦀️.rs:734` | P2 — live document path is fixed; the legacy per-node `ActionDescriptor` fallback (scene-embedded/no-intent consumers) still builds an unconditional delta descriptor. Verify `record_action_or_inert`'s no-binding behaviour before closing outright. |
| **Select** | production document path builds option rows from the retained arena (not just the inline spec) when opened | Radix renders `<SelectContent>` children on open | `children_of`/`apply_tree` (the gate that gives a `Select` its child rows for anything reading the generic arena — accessibility projection, tree/hit consumers) is still `#[cfg(any(test, feature = "testkit"))]`-only, `🔀️reconcile/🦀️.rs:1118-1119` (confirmed unchanged since Wave 0: exact same gate). Paint itself (`paint_select`) reads inline `UiSelectNode.items`, so the VISIBLE menu is fine; anything else walking the arena's children sees none. | **P1 — W1m gap G4, still open**, narrower than Wave 0 stated (visible paint is not affected) |
| **Select** | scroll chevrons actually scroll a long popup | `SelectScrollUpButton`/`DownButton`, React `🔽️Select/🟦️.tsx:769-795` | `select_scroll_step`/`select_clamped_scroll` exist and are unit-tested (`🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:140,149`) but **have zero callers** — confirmed by grep, no `scroll_offsets` field anywhere in the crate. Chevrons paint but do nothing on press. | **P1 — W2k gap 6, still open.** Long Selects (>viewport rows) are clamped/culled correctly but stuck once scrolled to the first page. |
| **Toggle/Checkbox** | tri-state checkbox | `Checkbox`, real production use only inside the CAD plugin's own React-only editor (`✏️s/🔌️plugins/📐️cad/…/✏️editor/⚙️engine/📺️renderer/🟦️.tsx`) | No `Component::Checkbox`/`UiNode::Checkbox` on either side — contract ceiling, not a wgpu gap | N/A (shared contract limit, unchanged) |
| **ToggleGroup** | generic `Component`-level control-group | No `ui_contract::Component` variant; React's own `ToggleGroup` element (`🧱️elements/🎛️ToggleGroup/🟦️.tsx`) has zero production call sites through the Interpreter | wgpu's `WindowEngagementControl::ToggleGroup` is unrelated window-chrome machinery | N/A (orphaned on React too — confirmed, still zero non-story call sites) |

## 3. Tree / TreeSection / Table (scene kinds)

| Behaviour | React | wgpu | Status |
|---|---|---|---|
| **Tree windowing/streaming** | `useTreeWindowObserver` requests a scroll-driven window from the guest | still builds the whole `TreeSection`/`TreeItem` subtree eagerly and clips at PAINT time (`RETAINED_NODE_COLLECTION_ITEMS = 256`); wgpu's own doc comment at `🧩️component/🦀️.rs:2362-2367` still says it "never REQUESTS" a window | **P1 — cross-ticket, tracked at `26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING` packet P5, not re-owned here.** Cited for completeness; do not duplicate work. |
| **Table/List-family scene kinds** (Table, VFS, GraphTimeline, BlockList, DiffView, EventFeed) | real production paint via `SurfaceView`/`ComponentSceneHost` | painting + hit-testing restored to production by W1a (list-like scenes were quarantined in `#[cfg(test)]` by the 09-08 sweep, since fixed) | **OK, closed** (not re-audited in depth here; W1a/W11a/W13d already drove the whole Scenes lane's test suites to 0 red) |
| **Generic `Table`/`Tabs`/`List`/`Chip`/`Badge` DOM widgets** | zero-or-near-zero production call sites through `ui_contract::Component` (chrome-only or orphaned) | Tabs now has a wgpu twin (W2k); Table/List/Chip/Badge remain absent on both sides | N/A — confirmed still dead on React too by the same grep pattern Wave 0 used |

## 4. Overlays, dialogs, menus

| Behaviour | React | wgpu | Status |
|---|---|---|---|
| **Context menu** | `ContextMenuController` | `ContextMenuOrganizer`/`render_context_menu`, shared taxonomy, dedicated keyboard test | OK — still one of the strongest parity areas, unchanged since Wave 0 |
| **Popover/Dialog content paint** | portal + real content | positioner + backdrop/surface paint exist (W1n); **not yet folded into `Ui::frame_step`'s `RetainedPaintPhase`** — façade-level only, per W1n gap 3 | **P1 — verify whether this landed in W1l's or a later lane's retained-phase rewrite; grep found `overlay_placements`/`paint_overlay_surface` still called only from the façade API (`⚙️engine/🦀️.rs` region `🪟️OverlayApi`), no call site inside the per-frame `frame_step` ladder.** If still true, an overlay a host doesn't manually pump every frame will not repaint itself. |
| **RTL mirroring** | `useFlow()`-derived, applied by `Popover`/`Select`/`Tabs`/`ToggleGroup`/`Slider` | vocabulary + positioner mirroring exist (W2k §2), but `Ui::set_window_flow` **has zero production callers** — every window stays `UiFlow::DEFAULT` | P2 — latent, not wrong (React's own default is also LTR-equivalent today; both shipped locales are LTR). Real work is gated on W2c/W2d's panel/window rewrite per W2k gap 4. |
| **`UiDriverTooltips` axis** (`full`/`minimal`/`none`, suppress-when-label-visible) | `🗨️ChromeControlHint`, driver-gated | zero `UiDriver` hits anywhere in `🎯️targets/🧊️wgpu` (confirmed by grep — same finding as W1n gap 4, unchanged) | P2 — tooltips always fire when a label exists; no compact-mode suppression |

## 5. Text, layout, theme tokens, density

| Item | React | wgpu | Status |
|---|---|---|---|
| **Bold/`emphasize` weight** | `font-semibold` | `TextWeight`/`faux_bold_offset`/`draw_text_weighted` exist (`🪀️widgets/🦀️.rs:571`) and are unit-tested, but **have zero callers outside their own definition/doc-comment** — confirmed by grep across the whole target. `retained_text_node_step` (the production text painter, `🖌️paint/🦀️.rs:313`) still only swaps font SIZE. | **P1 — primitive built (W2k §3c), never wired into production paint.** Emphasized text is still a size bump, not a weight change, on every screenshot. |
| **`UiNode::Grid`/`Scroll`/`Overlay`/`Absolute` variants** | n/a (React has no equivalent enum) | Still absent from the `UiNode` enum itself (`🧩️component/🦀️.rs:3265-3286`, 20 variants, `Stack` still the only container) — W1l instead threads `LayoutSpec`/`FlowStyle` onto `tree::Node` directly, bypassing `UiNode` for the geometry. Behaviourally closed (grid renders as columns, overlay is out-of-flow — proven by W1l's fixture tests) but **structurally** the audit's work-packet-4 "add real variants" was not done as specified. | P2 — cosmetic/architectural only; the user-visible gap is closed |
| **Container role=Plain/Form/Toolbar** | distinct `role` HTML attribute, semantically inert visually | still collapse into one `UiNode::Stack`, `🔀️reconcile/🦀️.rs:762-773` region (unchanged) | P2 — unchanged since Wave 0, harmless (no wgpu ARIA-landmark rendering exists regardless) |
| **Density (`StyleSpec.density`)** | React's own layout path is ALSO pinned to the compact ramp (shared limitation) | `Theme` has no density parameter at all | P2 — shared gap, not a wgpu regression, unchanged |
| **`WidgetNode` kit's variant ceiling** (scene-embedded panels + standalone Tree target, `🪀️widgets/🦀️.rs:211-227`) | n/a — this is wgpu's own second, smaller paint kit | Still **20 vs 15 arms**: no `Progress`, `Image`, `Group`, `ComponentScene`, `ExternalSlot` on `WidgetNode<E>` — confirmed unchanged line-for-line since Wave 0. Any panel painted through this kit (not the main `paint_node` ladder) cannot show a progress bar, an image, a nested group, a scene, or an extension slot **at all**. | **P1 — unchanged since Wave 0.** No Wave-1/2 packet touched this file's enum shape. |
| **Line-height / icon-size token drift** | ramp-derived | token-derived, matches (W1o) | OK, closed |
| **Text wrap (word-break)** | native UAX#14 | CSS-equivalent greedy word wrap (W9a fixed the mid-word-break bug found in tour-card live testing) | OK for the common Latin case; no true UAX#14 (CJK/hyphenation) — unchanged, low-impact P2 |

## 6. Keyboard / focus / accessibility

| Item | React | wgpu | Status |
|---|---|---|---|
| **`UiCommand::FocusChanged`** | native DOM `focusin` | intentional no-op BY DESIGN now (re-documented, not a deferred TODO): `content_focus_node` retains the real node id (W2k §3f) | OK, closed — re-scoped from "deferred" to "by design" |
| **Tab-order / dialog focus-trap** | native | full parity, unchanged, still OK | OK |
| **`focus-visible`** | CSS pseudo-class | closed, see §1 | OK, closed |
| **Accessibility projection** | `accessibilityAriaProps` | shared `ui_contract` projection, proven against a shared fixture | OK, unchanged strongest area |

## 7. CAD spatial-tree editor (plugin-scoped, post-W2f)

Wave 0 flagged "no wgpu implementation at all." W2f corrected the census (the shell-rendered CAD
screen is the Rust play app, not the React-only `@semio-tech/cad-js` editor reachable only from
storybook) and ported the typology + picking engines. Residual, confirmed by W2f's own gaps section
and not touched since (status.md has no later CAD packet):

| Gap | Detail | Priority |
|---|---|---|
| **No `World3dScene` pick-target lane** | Sub-object selections are shown (preview paints) but not hit-tested — a click still resolves at whole-object granularity, not the sub-feature the preview highlights | **P0** — the interaction the preview visually promises (click a highlighted sub-object) does not work |
| **Curved edges degrade to endpoints** | `CadEdgeCurve` carries only `kind`, no centre/poles — arc/circle/ellipse/nurbs edges sample as 2 points vs React's 32-64-point tessellation (`edgeSamplePoints`) | P1 — visual fidelity, not interaction-breaking |
| **No per-instance opacity for locked objects** | `target_style`'s locked arm resolves correctly but the instance lane has no opacity field to carry it — `WORLD_LOCKED_OPACITY_SCALE` cannot reach a committed mesh | P1 |

## 8. Retained-arena / paging constraints affecting content visibility

- **512-byte `UiText` cap** (`🧬️contract/🧵️retained/📦️wire/🟦️.ts:11` `TEXT_BYTES = 512`, mirrored
  Rust-side fixtures) is a **shared contract-level** fixed-arena limit, not wgpu-specific — it bounds
  any single retained-document text value on both renderers' resident path. Project memory ("One UI
  Admission Fault Kills Every Later Refresh") already documents a live incident where overflow here
  faulted `refreshUi` wholesale. Not re-litigated as a wgpu parity gap; flagged because it compounds
  with the Tree-windowing gap (§3) — a wgpu tree that eagerly materializes a whole subtree (instead of
  requesting a bounded window) is more exposed to hitting this cap on a text-heavy tree than React's
  windowed request ever is.
- **`RETAINED_NODE_COLLECTION_ITEMS = 256`** (`🖌️paint/🦀️.rs:47,293`) and the scene-kind list-row
  reservations (`reserve_list_rows`, W1a) are wgpu-side paint-time caps layered on top of the shared
  arena cap — both already bounded/tested, not a new finding.

---

## 9. Top 10 packets to dispatch

1. **P0 — Wire a `World3dScene` pick-target lane for the CAD play app.** Sub-object clicks currently
   resolve to whole-object selection even though the preview already highlights the sub-feature.
   Seam: new lane in `🌐️World3dHost`/`♾️infinite/🌍️world`, `merge_selection_targets`/
   `resolve_spatial_pick_targets_to_render` from `⚙️engine/🧲️picking/🦀️.rs` (already built, unconsumed
   for this half). Cites `📓️w2f-cad-spatial-editor-wgpu.md` §5.1.
2. **P1 — Add `Progress`/`Image`/`Group`/`ComponentScene`/`ExternalSlot` to `WidgetNode<E>`
   (`🪀️widgets/🦀️.rs:211-227`).** Any scene-embedded panel or the standalone Tree target using this
   second, smaller paint kit silently cannot show five whole component kinds. Unchanged since Wave 0 —
   no packet has touched this enum.
3. **P1 — Wire `draw_text_weighted`/`TextWeight::Semibold` into `retained_text_node_step`
   (`🖌️paint/🦀️.rs:313`).** The faux-bold primitive is built and unit-tested (W2k) but has zero
   callers; every emphasized `Text` node still renders as a size bump, not a weight change, in every
   live screenshot.
4. **P1 — Make Select's scroll chevrons actually scroll.** `select_scroll_step`/`select_clamped_scroll`
   are tested arithmetic with no caller; add a per-popup offset slot to `WidgetContext` and route a
   chevron press into it (W2k gap 6, restated).
5. **P1 — Fold Popover/Dialog content paint into `Ui::frame_step`'s retained phase ladder.** Currently
   façade-only (`Ui::open_overlay`/`overlay_placements`); confirm whether a host must manually re-pump
   it every frame, and if so add a terminal `Overlay` phase (W1n gap 3, re-verify against current
   `frame_step`).
6. **P1 — Close the chrome shell's Input commit path to `InputMeta::commit_value`.**
   `commit_focused_input` (`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11038`) still dispatches raw
   `input.text_view()`, bypassing the min/max/step clamp the intent path already enforces elsewhere
   (W1m gap G6, confirmed still open).
7. **P1 — Give production Select a real open/child-materialization path.** `children_of`/`apply_tree`
   are still `#[cfg(any(test, feature = "testkit"))]`-gated (`🔀️reconcile/🦀️.rs:1118-1119`); anything
   reading the generic arena children of an open Select (not the direct paint call) sees none in
   production (W1m gap G4).
8. **P1 — Curved-edge tessellation for the CAD play app.** `CadEdgeCurve` needs centre/pole data (or a
   real `Brep` handle) instead of degrading arc/circle/ellipse/nurbs to their two endpoints
   (`📓️w2f-cad-spatial-editor-wgpu.md` §5.2).
9. **P1 — Per-instance opacity for locked CAD objects.** Add an opacity field to the CAD instance lane
   so `target_style`'s already-correct locked arm can reach a committed mesh
   (`📓️w2f-cad-spatial-editor-wgpu.md` §5.5).
10. **P2 (bundle) — `UiDriverTooltips` axis + `Ui::set_window_flow` production wiring.** Neither is
    user-visible today (no compact-driver product surface confirmed; both shipped locales are LTR), but
    both are one small seam each once their owning lanes (chrome driver plumbing; W2c/W2d's panel/window
    rewrite) land — worth bundling into whichever packet already touches those files rather than a
    standalone dispatch.

Cross-ticket, not re-dispatched here: Tree virtualized/streaming windowing (owned by
`26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING` packet P5).
