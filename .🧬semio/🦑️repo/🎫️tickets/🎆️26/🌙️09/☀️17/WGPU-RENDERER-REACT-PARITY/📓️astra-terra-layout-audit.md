# Current WGPU Layout and Compositing Residuals

Read-only audit, 2026-09-19. This is a current-source follow-up to
`📓️audit-w14-elements-residual.md` and `📓️w15a-elements-residual.md`. It omits their completed
items and restricts itself to three visible, production-path defects.

## 1. Retained Document Select Bypasses the Current Popup Renderer

**Impact: P0/P1.** A long Select inside an interpreted retained document has neither the React-style
bounded viewport nor working scroll chevrons. Its menu glass is composited after its rows, leaving
row fills and labels beneath the glass instead of in its foreground.

The W15a popup path is `render_select_menu` in
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:420-477`. It uses
`select_menu_painted_height`, scroll-offset slots, `select_scrolled_row_window`, clipping, and
shared chevron controls. The production retained-document path is a different painter:
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:827-918`.

That latter path computes `select_menu_height`, walks every item up to
`RETAINED_NODE_COLLECTION_ITEMS`, and reads no scroll slot or shared row window. At lines 869-873
it records the glass region, then records highlights and glyphs as ordinary scene content at
lines 874-918. It never calls `DrawList::begin_glass_content`. The prepared GPU pipeline renders
ordinary commands to the scene at
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:613-630`, composites glass at
lines 650-668, then draws foreground commands at lines 670-680. The popup has no foreground lane.

**Bounded repair.** Make the retained Select cursor delegate to shared popup geometry and scroll
state, or extract one renderer-neutral retained-popup helper. Create a bounded popup rect, paint only
the visible row window, arm and consume existing chevron state, and bracket row/highlight/text output
with the glass foreground route. Close the route on every terminal, fault, and cancellation path.

**Acceptance law.** Open a retained `UiNode::Select` with 20 items in a viewport shorter than its
natural menu. Assert that its bottom remains in the viewport, only visible rows are painted, a
down-chevron press advances the first visible item by React's scroll step, and prepared commands put
the menu glass before that popup's row foreground.

## 2. Overlay Routing Does Not Include Raster Images

**Impact: P1.** A decodable `data:image/*` image inside an open Dialog or Popover is rasterized below
the modal/popover chrome. React portals the image with its overlay content, where it remains visible
above the surface.

The retained image arm calls `push_raster_quad` at
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1223`. Overlay routing changes only
`active_ui_instances` and `active_vector_vertices` in
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:629-646`;
`push_raster_quad` always appends to `DrawLayer::raster_instances` at lines 834-842. The prepared
presenter emits ordinary raster in the scene phase before glass and foreground phases, while
overlay-routed UI appears later. This is the W15a residual and remains true in current code.

**Bounded repair.** Add an overlay-raster collection to `DrawLayer`, select it from
`push_raster_quad` when `overlay_routed()`, and teach prepared-command measurement and
`encode_prepared_draw_scalar` to emit it in the composite foreground phase with overlay UI/vector
content. Keep raster texture admission and cleanup shared; change only the ordering bucket.

**Acceptance law.** Build a Dialog containing one decodable data-URL Image plus an identical
non-overlay image. Prepared commands must draw the latter in the scene phase and the dialog image
after its surface in the composite phase. Its raster upload key and decoded dimensions must be equal
before and after the routing change.

## 3. `LayoutSpec::Overlay` Has Incompatible React and WGPU Meanings

**Impact: P1, requiring a contract decision before implementation.** Current React and WGPU layout
give authored Overlay records opposite flow behavior.

React's `layoutSpecStyle` returns `position: "absolute"` and `inset` for an Overlay at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:913-914`,
but `ContainerView` immediately overwrites that record's position with `"relative"` at line 1117.
The resulting container stays in normal flow and acts as a positioning context. WGPU maps the same
spec to `absolute: true` at
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:242-245`; its solver removes it from
flow. Its law asserts that a following vertical-stack sibling starts at y=0 in
`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs:216-229`.

For a vertical Stack containing `[Overlay(inset: md), Fixed leaf]`, React and WGPU therefore assign
different y coordinates to the leaf. The existing WGPU test validates WGPU's contract interpretation,
not the React reference implementation.

**Decision and repair.** Investigate the shared contract semantics before copying React's override:
the authored layout may intentionally need an in-flow positioning context, in which case a paired
repair is appropriate. If Overlay is in flow, repair WGPU to retain it in flow and update its
out-of-flow law. If Overlay is out of flow, repair React by removing the relative override and add a
React regression. Then make contract documentation, React style mapping, WGPU `flow_from_spec`, and
the shared conformance fixture state one behavior.

**Acceptance law.** Feed both targets the same vertical-stack fixture and compare measured rectangles.
Assert the selected semantics in a renderer-neutral contract fixture and both target suites, including
all-four-side inset geometry.
