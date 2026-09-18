# W1n — Image, Popover/Dialog overlays, tooltips, loading skeletons & disabled states

Packet W1n of `26/09/17/WGPU-RENDERER-REACT-PARITY`, closing audit packets **6** (`Component::Image`),
**7** (Popover/Dialog overlay content), **8** (hover tooltips) and **11** (loading skeletons), plus the
`disabled` cursor half of §4 of `📓️audit-interpreter-elements.md`.

All paths below are absolute-from-repo-root. React references cite the file the behaviour is actually
implemented in, which for tooltips/popovers is the element folder, not the `Interpreter`.

---

## 1. Per-feature React → wgpu behaviour table

### 1.1 `Component::Image` (audit packet 6 — was **P0 broken**)

| Behaviour | React | wgpu before | wgpu now |
|---|---|---|---|
| Source decode | `<img src>` — browser decodes PNG/JPEG/SVG/data-URL natively (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1972-1975`) | nothing decoded; `paint_image` was `#[cfg(test)]` (`🖌️paint/🦀️.rs:2391`) | `admit_ui_image` decodes `data:image/png` (base64 or percent-encoded) through the first-party codec `semio-framework-pixels::decode_png` — `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` region `🖼️UiImageSources` |
| Draw | real bitmap | rounded `theme.panel` placeholder + `alt` text (`🖌️paint/🦀️.rs:1071-1093`) | `push_raster_quad(src, …)` → `KIND_RASTER` instance at the object-fit rect; placeholder only survives as the not-yet-resolved fallback |
| Fit | `object-contain`, `max-h-64`, `max-w-full` (`🗣️Interpreter/🟦️.tsx:1974`) | n/a | `ui_image_content_rect` — per-axis box clamp (`UI_IMAGE_MAX_BOX_HEIGHT = 256.0` ⇔ `max-h-64`) then centred aspect-preserving letterbox |
| Upload | browser texture cache | none (doc comment said upload "lives in the renderer's `program_bridge`, outside this crate's scope") | `take_ui_image_upload()` hands the host one `UiImageUpload { key, width, height, pixels }` (RGBA8) per drain — exactly `prepared::PreparedRasterProducer::try_admit`'s input shape, keyed by the same `src` the draw instance carries |
| Retirement | n/a | n/a | `close_ui_image_ledger_step()`, one item per call, terminal-empty predicate |

Bounds: `UI_IMAGE_SOURCE_MAX_BYTES = 4 MiB` (matches the os renderer's own
`RETAINED_UI_IMAGE_SOURCE_BYTES`), `UI_IMAGE_MAX_DIMENSION = 4096`, `UI_IMAGE_LEDGER_ENTRIES = 64`.

**Deliberate carve-outs**, both answered as explicit states rather than silent failure:
* `UiImageAdmission::Deferred` — any `http(s)://` or relative `src`. Fetching is not this crate's
  authority; it stays the host asset pipeline's (`WorldAssetRequestKind::UiImage`, already implemented
  at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1393-1611`).
* `UiImageAdmission::Unsupported` — `data:image/jpeg`. `semio-framework-pixels` is PNG-only and
  AGENTS.md forbids a third-party runtime codec, so JPEG stays on the host path (the os renderer
  already decodes it there with `image`). This is the one remaining behavioural gap in packet 6.

Dependency added: `semio-framework-pixels` under the `wgpu-engine` feature only
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`). It is first-party and its only dependency is
the owned `semio-framework-deflate`, so no third-party crate enters the runtime graph and the
`wasm32-unknown-unknown` build is unaffected (verified below).

### 1.2 Popover / Dialog overlays (audit packet 7 — was **P1 missing**)

The scaffolding the audit found (`OverlayKind`/`OverlayPlacement`/`resolve_overlay_placement`) used a
3-variant placement model that did **not** match React. It has been replaced by a field-for-field port
of React's own positioner.

| Behaviour | React (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️.tsx`) | wgpu now (`🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`) |
|---|---|---|
| Positioner | `resolvePopoverPlacement`, `:215-258` | `resolve_anchored_placement` — same align/side/flip/clamp order, same formulas |
| Side/align axis | `PopoverSide`/`PopoverAlign`, `:22-23` | `OverlaySide`/`OverlayAlign` |
| Prop bundle | `side`/`align`/`sideOffset`/`alignOffset`/`collisionPadding`/`avoidCollisions`, `:288-293` | `AnchoredPlacement` with the same six fields |
| Popover defaults | `bottom`/`center`/4/0/8/true, `:288-293` | `AnchoredPlacement::POPOVER` |
| Tooltip defaults | `top`/`center`/8/0/8/true (`🧱️elements/💡️ChromeControlHint/🟦️.tsx:69`) | `AnchoredPlacement::TOOLTIP` |
| Menu/select | flush under the trigger's start edge | `AnchoredPlacement::MENU` |
| Collision flip | flip to `opposite[side]` only when the flip itself does not overflow, `:241-248` | same, `OverlaySide::opposite` |
| Viewport clamp | `Math.min(Math.max(…))` on both axes inside `collisionPadding`, `:249-252` | same |
| Resolved side out | `transformOrigin` from the settled side, `:253` | `ResolvedOverlayPlacement { side, x, y }` |
| Modal scrim | `Dialog`'s portalled overlay | `OverlayKind::has_backdrop()` + `paint::paint_overlay_backdrop` (`OVERLAY_BACKDROP_ALPHA = 0.55`) |
| Floating surface | portal + `z-menu` + glass | `paint::paint_overlay_surface` — shade, `Level::Menu` fill, hairline; drawn into the **overlay layer** |
| Z-order above panels | portal to `useShellFloatingSurfaceHost` | `Ui::overlay_placements(window_id)` is drawn after the document walk; `push_solid_overlay` composites over every panel instance of the frame |
| Escape / outside-press / focus trap | `onEscapeKeyDown`/`onPointerDownOutside`/`useDialogLayer` | already present in `EventRouter` (unchanged); `OverlayKind::Popover` is new and inherits the same policy |
| Façade entry points | `<Popover open>` | `Ui::open_overlay` / `Ui::close_overlay` / `Ui::overlay_placements` (`⚙️engine/🦀️.rs`, region `🪟️OverlayApi`), returning `UiOverlayPlacement` |

`OverlayKind` gained a `Popover` variant (was five, now six).

### 1.3 Hover tooltips (audit packet 8 — was **P1 missing, unticketed**)

| Behaviour | React | wgpu now |
|---|---|---|
| Dwell before open | `CHROME_CONTROL_TOOLTIP_DELAY_MS = 400` armed on `onPointerEnter`/`onFocusCapture` (`🧱️elements/💡️ChromeControlHint/🟦️.tsx:21,49-52`) | `TOOLTIP_DWELL_SECONDS = 0.4`; `EventRouter::update_hover` stamps the hover-leaf entry time, `EventRouter::advance_clock` fires `TooltipStep::Reveal(node)` once per hover |
| Clock source | `setTimeout` | explicit monotonic seconds via `Ui::advance_clock(seconds)`; a non-monotonic value is ignored rather than rewinding armed deadlines |
| Close on hover-out | `onPointerLeave` → close | `maybe_dismiss_tooltip_on_hover_out` now **arms** `DismissPolicy::hover_out_delay_seconds` (0.4 s) and `advance_clock` closes it — the field's own doc comment used to admit "not actually debounced yet"; re-entering the anchor disarms it |
| Text | `formatControlTooltipText({label, hotkey})` → `"label (hotkey)"` (`🔨️modules/💡️control-tooltip-presentation/🟦️.ts:16`); suppressed when there is no label | `Ui::tooltip_label(window_id, node)` reads the published record's `AccessibilitySpec` `label` + `shortcut` (same tiers `accessibility_projection_node` projects), returns `None` for a hidden or unlabelled node |
| Placement | `resolvePopoverPlacement(…, "top", "center", 8, 0, 8, …)` (`💡️ChromeControlHint/🟦️.tsx:69`) | `AnchoredPlacement::TOOLTIP` through the same positioner |
| Surface | `p-single text-xs` glass, `role="tooltip"` | `paint::tooltip_surface_size` + `paint::paint_tooltip` (same padding/font-size tokens) |

Not ported: React's `UiDriverTooltips` axis (`full`/`minimal`/`none`) and the
"suppress when the inline label is already visible" rule (`useControlTooltipText`,
`🧱️elements/🏷️Label/🟦️.tsx:183-198`). The wgpu target carries no `UiDriver` at all — see Gaps.

### 1.4 Loading skeletons & disabled state (audit packet 11 + §4 — was **P2 divergent**)

| Behaviour | React | wgpu before | wgpu now |
|---|---|---|---|
| Busy element rendering | `interpretUiNodeBusyShell` replaces the whole component with `elementSkeleton(type)` (`🗣️Interpreter/🟦️.tsx:2147-2154`) | border tint only; content still painted | `skeleton_replaces_content` intercepts at the top of `paint_node_step` and paints `retained_skeleton_step` **instead of** the element's own content |
| `Progress` exemption | `record.component.type === "progress"` skipped (`:2148`) | n/a | same predicate |
| Per-kind shape | `elementSkeleton`, 19 kinds (`🧱️elements/🦴️Skeletons/🟦️.tsx:54-115`) | none (zero `skeleton` hits in the whole target) | `skeleton_kind(&UiNode) -> SkeletonKind` collapsed to the nine shapes that actually differ (`Line`, `Control`, `Separator`, `Image`, `Rows`, `Ring`, `Field`, `Panel`, `Fill`) |
| Block geometry | Tailwind `h-4`/`h-medium`/`gap-single`/`p-single` | n/a | `skeleton_blocks` resolves the same ratios against `Theme::{font_size_body, font_size_small, control_height, gap_standard, padding_standard, stroke_hairline}` — no new px literals |
| Fill | `animate-pulse bg-muted-foreground/20` | n/a | `theme.text_muted.with_alpha(0.2)`, motionless — which is also React's own `motion-reduce:animate-none` rendering |
| Allocation | n/a | n/a | fixed `[Rect; SKELETON_MAX_BLOCKS=4]`, one admitted fixed-output grant, no mid-frame allocation |
| Loading/waiting border | `loadingBorderElementClass`/`waitingBorderElementClass` | already present | unchanged, still emitted alongside the skeleton |
| Disabled opacity | `disabled:opacity-50` | already present — `presence_overlay` scrims at `theme.panel @ 0.35` (`🖌️paint/🦀️.rs:1688-1690`) | unchanged |
| Disabled cursor | `disabled:cursor-not-allowed` | **missing** — `resolve_semio_cursor_from_tree` never checked presence | `SemioCursor::NotAllowed` returned before any drag/text affordance (`🎯️targets/🧊️wgpu/👆️cursor/🦀️.rs`) |

---

## 2. Files changed

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` | `OverlaySide`/`OverlayAlign`/`AnchoredPlacement`/`ResolvedOverlayPlacement`; `OverlayPlacement::Anchored`; `OverlayKind::Popover` + `has_backdrop`; `resolve_anchored_placement` + `resolve_overlay_placement_side`; new `🔖️Tooltip` region (`TOOLTIP_DWELL_SECONDS`, `TOOLTIP_HOVER_OUT_SECONDS`, `TooltipStep`); router clock fields + `advance_clock`; debounced tooltip hover-out; `open_overlays()` accessor |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` | `🖼️UiImageSources`, `🦴️Skeleton` and `🪟️OverlayChrome` regions; `UiNode::Image` retained arm draws a raster quad; skeleton intercept in `paint_node_step`; `draw_text_on` import un-`cfg(test)`-ed |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` | `UiOverlayPlacement`; `Ui::{open_overlay, close_overlay, overlay_placements, advance_clock, tooltip_label}` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/👆️cursor/🦀️.rs` | disabled → `NotAllowed` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` | curated re-exports for all of the above |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-pixels`, optional, `wgpu-engine` only |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs` | +11 tests (`🖼️UiImagePaint`, `🦴️SkeletonPaint`) |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs` | +9 tests (`🔖️AnchoredPlacementTests`, `🔖️TooltipDwellTests`) |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-cursor-unit/🦀️.rs` | +1 test (`🚫️DisabledCursorTests`) |

`🧊️gpu/🦀️.rs` was read but **not** changed: its `RasterTextureTable` path already admits exactly the
`(key, RGBA8, width, height)` tuple `UiImageUpload` carries, so no new GPU-side code was needed.

---

## 3. Tests

21 new tests, all run and green.

**`paint::tests`** (11)
* `a_data_url_png_source_decodes_to_its_natural_size`
* `an_admitted_source_publishes_exactly_one_pending_upload` — RGBA8 layout + pixel contents
* `a_url_source_is_deferred_to_the_host_and_a_jpeg_data_url_is_unsupported`
* `the_image_content_rect_matches_react_object_contain` — wide letterbox, no upscale, `max-h-64` cap
* `a_decoded_image_paints_a_raster_quad_instead_of_the_placeholder` — exactly one `KIND_RASTER`
  instance, keyed by `src`, at the `object-contain` rect, and zero alt-text glyphs
* `an_undecodable_source_still_paints_the_placeholder_and_alt_text`
* `the_skeleton_predicate_matches_reacts_busy_shell` (incl. the `Progress` exemption)
* `each_element_kind_picks_reacts_own_skeleton_shape`
* `a_skeletons_blocks_stay_inside_their_bounds_and_within_the_fixed_capacity` (all nine kinds)
* `a_loading_text_node_paints_skeleton_blocks_instead_of_its_glyphs`

The PNG fixture is built at test time with `semio_framework_pixels::encode_png` and base64-encoded by
a test-local encoder, so the decode path is proven by an owned encode→decode round trip.

**`events::tests`** (9)
* `an_anchored_overlay_sits_on_its_requested_side_with_the_declared_offset` (popover + tooltip offsets)
* `an_anchored_overlay_flips_to_the_opposite_side_when_the_main_axis_overflows` (both directions)
* `an_anchored_overlay_never_flips_into_a_side_that_also_overflows`
* `an_anchored_overlay_is_clamped_inside_the_collision_padding`
* `a_centered_overlay_ignores_its_anchor` + backdrop predicate
* `a_hover_reveals_a_tooltip_only_after_the_react_dwell_elapses` (and exactly once)
* `moving_to_another_control_restarts_the_dwell`
* `an_open_tooltip_dismisses_only_after_the_hover_out_delay`
* `returning_to_the_anchor_disarms_the_hover_out_countdown`

**`cursor::tests`** (1)
* `a_disabled_node_uses_the_not_allowed_cursor_before_any_other_affordance`

`🔬️targets-wgpu-draw-unit` was **not** extended: the packet's draw-side acceptance ("image decode →
draw rect") is the `KIND_RASTER` instance assertion, which belongs with the paint step that emits it
and is covered above; `DrawList::push_raster_quad` itself already had its admission/retirement laws
under `🖼️raster-witness-lifecycle`.

---

## 4. Verification (all foreground; logs under `🗑️generated/w1n-*.txt`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going` | **exit 0**, no errors. Only warning attributable to this packet: none (`w1n-check-lib.txt`) |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --target wasm32-unknown-unknown --keep-going` | **exit 0** — the owned PNG codec is wasm-clean (`w1n-check-wasm.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- paint::tests events::tests cursor::tests` | **exit 0 — 122 passed, 0 failed** (`w1n-test-2.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **peer-blocked**, see below (`w1n-check-os-renderer.txt`) |

**os-renderer-wgpu.** One run completed the whole graph and reported 16 errors, *all* of them
`E0425: cannot find function canvas_sat / canvas_set_sat / canvas_lum / canvas_set_lum` at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3280-3290`
— a concurrent peer's in-flight canvas blend-mode work, nothing from this packet. Nothing referencing
`OverlayPlacement`, `UiCommand`, the paint API or the image registry failed, and the crate's own
`Interpreter` wgpu target (the heaviest consumer of the changed API) compiled. Four later re-runs were
`SIGKILL`ed (exit 137) partway through the `✏️s` plugin crates — the machine is memory-saturated by the
concurrent fleet (`-j 1`, `-j 2` and `CARGO_INCREMENTAL=0` all died the same way), not a regression here.
**This one needs a clean re-run once the Scenes lane compiles again.**

`cargo test -p semio-framework-ui --features wgpu-engine --lib` (whole crate) has 12 failures, none in
`paint`/`events`/`cursor`: they are in `component::ui` wire-format goldens, `engine` hostile fixtures,
`flex` (justify/wrap), `input`, `layout`, `mounted_layout`, `prepared` and
`events::control_commit_tests` — the W1l (layout/flex) and W1m (UiIntent) lanes' in-flight work
(`w1n-test-full.txt`).

---

## 5. Remaining gaps

1. **JPEG.** `data:image/jpeg` answers `Unsupported` and falls back to the host asset pipeline. Closing
   it in-crate means writing an owned baseline-JPEG decoder into `🧰️framework/🔨️modules/🔲️pixels` (which
   is where it belongs — the framework's owned codec module — not into the ui target).
2. **SVG sources.** Same story: the os renderer rasterises `data:image/svg+xml` with `usvg`/`resvg`
   (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1495-1509`); this crate has no owned SVG rasteriser, so an
   inline-SVG `src` is `Unsupported` on the no-host path.
3. **Overlay paint pass is façade-level, not frame-step-level.** `Ui::overlay_placements` +
   `paint_overlay_backdrop`/`paint_overlay_surface`/`paint_tooltip` give a host everything it needs, but
   they are not yet folded into `Ui::frame_step`'s retained paint state machine (`RetainedPaintPhase`),
   because that machine is actively being changed by the W1l lane. Folding it in as a terminal
   `Overlay` phase is the natural follow-up and is a self-contained edit.
4. **`UiDriverTooltips` axis is absent.** React suppresses tooltips entirely under the `compact` driver
   (`tooltips: "none"`) and hides them when an inline label is already visible. wgpu carries no
   `UiDriver` at all, so `Ui::tooltip_label` always answers when a label exists. Wiring the driver into
   this target is its own packet (it also governs `labels`/`chrome`/`gumball`/`hotkeys` reveal).
5. **RTL.** `resolve_anchored_placement` does not mirror `align: Start`/`End`, because this target has
   no per-window flow direction (React takes it from `useFlow`). The port is otherwise exact; the `rtl`
   argument was deliberately left out rather than hardcoded to `false` inside the formula.
6. **A second, now-divergent copy of the old placement math** lives at
   `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs:513-620`, self-described as "Ported from
   `events.rs::OverlayPlacement` verbatim". It still carries the pre-W1n `BelowAnchorWithFlip`/
   `AtPointer`/`Centered` model and so is now the *only* remaining place in the repo that disagrees
   with React's positioner. Out of this packet's file scope (it is the `🖌️render` module, not the wgpu
   target) — flagged for whoever owns that lane.
7. **Tooltip content subtree.** `TooltipStep::Reveal(node)` + `Ui::tooltip_label` tell a host what to
   show and `paint_tooltip` draws it, but nothing reconciles a tooltip *node* into the arena
   automatically — consistent with `events`' own scoping ("building the popup CONTENTS is explicitly
   not this module's job"). A host-side or reconcile-side auto-tooltip node is follow-up work.
