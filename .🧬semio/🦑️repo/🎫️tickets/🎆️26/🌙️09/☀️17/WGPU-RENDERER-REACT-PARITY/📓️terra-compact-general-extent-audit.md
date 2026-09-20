# Compact General Panel Extent Audit

## Current Qualification — Production Producer Trace Required

**The prior verdict is withdrawn.** Sealed checkpoint 13 observed full-height WGPU glass after settle. The earlier review followed the Shell consumer of `retained_content_height`, which was insufficient to establish that a normal upward General document publishes a compact value. The current source trace makes the intended producer path explicit, but it is not a runtime parity result and does not outweigh the contrary sealed observation.

The production path is currently:

1. `ShellState::build_settings_general_ui` produces `UiNode::Tree` ([Shell](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7614)). `PanelProjection` preserves it as `Component::Tree` ([Shell](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4747)), and production reconcile recreates a root `UiNode::Tree` from that record ([reconcile](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:835)).
2. `MountedLayoutJob` classifies that root as `LayoutNodeKind::Tree` with `retained_tree_height`: the sum of the visible, current-disclosure section heights, rather than the viewport ([mounted layout](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:119), [classification](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:514)). Each Tree gets `Dim::Length(height)` ([flex](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:286)), which replaces the bottom-up aggregate with that explicit height ([flex](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:558)). The height source itself is row/header arithmetic ([tree layout](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:168)).
3. On accepted publication, the engine copies the job root intrinsic height into `intrinsic_content_height` ([engine](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1128)); `surface_content_height` prefers that finite value and falls back to `root_layout.height` only without one ([engine](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1925)). Shell then adds its tab and inset allowance and clamps the resulting panel to its anchor band ([Shell](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22655), [panel rectangle](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15839)).

This rules out one static explanation: an accepted, correctly reconciled General `Tree` is not deliberately measured from its allocated viewport. It can still occupy its entire band when its own visible row sum reaches the clamp, which is valid React behavior. The source audit cannot establish which condition produced the sealed frame: the runtime may have rendered before the accepted production document, may have used an unexpected root, or may have produced a band-sized content sum. There is no existing normal-mount receipt that records the General root kind, accepted intrinsic height, anchor band, and final glass rectangle together.

## Verdict

No stable General compactness or React parity claim is justified yet. The current implementation has a content-derived Tree producer in source, and Shell scopes glass, scissor, and retained hits to the resulting panel/content rectangle; the sealed full-height screenshot remains a production failure until the normal mount path is run and explained.

`upward_tree_frame_completes_after_empty_nested_and_terminal_cursors` is not that proof: it calls the test-only `Ui::apply_tree`, supplies a fixed 320×240 viewport, and asserts reachability/frame completion, not the root intrinsic value or Shell panel footprint ([law](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/📂️retained-section-collapse/🦀️.rs:181)). The existing Settings law similarly either hand-calculates a supplied panel rect or stops while the fresh production document remains pending ([Settings law](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:45), [pending render](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:107)). Neither can disprove a permanent full-band footprint.

## React Authority

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` derives `isBottom` from the anchor flow, positions the panel with one vertical edge and `maxHeight`, and uses a reverse flex stack for an upward-flowing panel. It does not assign a fixed panel height.

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` makes the same contract explicit in `anchorPositionStyle`: it sets a vertical edge and a bounded `maxHeight`, not `height`. Its bottom panel viewport uses `justify-end`. The React body surface is `pointer-events-none` and is bounded by `geometry.bodyRegion`; its own extent comes from the compact stack rather than the Shell layout viewport.

## WGPU Paint and Hit Ownership

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` computes the final panel through `anchor_panel_rect` and passes `anchor_content_height` into it. That height is the tab strip plus the retained document's `retained_content_height`. `render_panel_step` paints panel glass with this panel rectangle, derives a smaller flowed content rectangle, scissors retained rendering to that content rectangle, and registers retained body hits with that same content rectangle.

The Shell `body_rect` is the canvas middle region, but it is not used as the General panel surface or generic retained hit owner. `register_retained_body_hits` maps actual retained targets to the panel content rectangle. In `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`, the retained Tree paints rows within its passed bounds and does not add a generic Tree body fill. The full-canvas background in Shell is the application floor, not General panel glass.

## Bounded Recommendation and Fixture Oracle

Extend the existing `⚙️settings-general-layout` oracle with a real document ingress/reconcile/layout/second-render path, and retain a matching browser law:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/⚙️settings-general-layout/🔣️json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/⚙️settings-general-layout/🔣️json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧪️tests/⚙️settings-general-layout/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs`

Add one compact upward General scenario whose band exceeds the fixture's visible-tree row sum. Drive the actual General records through document ingress, reconcile, accepted layout, and a new Shell panel cursor. The native law must assert the root is `Tree`, record the accepted intrinsic height, assert it is below the band in this vector, and then assert the final glass rectangle, flowed content rectangle, and retained hit-owner rectangle all use that compact footprint. A point in the unused portion of the former band must resolve to no General target, while a real General control still resolves to its document target. The browser law should mount the same General content, assert React's `maxHeight`/bonded bottom edge/`justify-end` contract, and compare its compact bounding rectangle against the fixture oracle.

Keep the first-frame fallback separate from this stable-state oracle. If the normal-mount law still reports a band-sized root for a below-band General fixture, repair the producer or publication handoff identified by that receipt; do not broaden final panel paint or hits to `body_rect`.
