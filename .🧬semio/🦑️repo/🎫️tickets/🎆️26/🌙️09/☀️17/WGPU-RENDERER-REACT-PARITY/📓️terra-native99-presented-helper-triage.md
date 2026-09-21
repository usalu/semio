# Native99 Presented-Frame Test Helper Triage

## Evidence

The runtime makes the input publication boundary a whole-frame operation. `FrameBuildBoundary` calls `ShellState::seal_presented_input_candidate` only after `render_chrome_step` reports complete in [renderer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15081). The presenter subsequently checks the witness and calls `ShellState::acknowledge_presented_input` only after its packet has been accepted [there](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:14621).

`acknowledge_presented_input` is the atomic promotion: it swaps retained owner, scene, and widget maps and geometry, then calls `InputState::publish_hits` [in Shell](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13237). Calling `InputState::publish_hits` after a Shell paint walk without that acknowledgement publishes candidate hits against old presentation maps, which cannot model a real accepted frame.

The existing test-only seam is `ShellState::publish_retained_hit_registry` [in Shell](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13267). It seals and immediately acknowledges. Existing correct actual-path uses include the World owner fixture in [wgpu-shell-input](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:120) and settled General-panel helpers in [settings-general-layout](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:388).

## Repair Classification

Only a test that both stages data through actual `ShellState` paint/registration and later asks a presentation-facing question needs the accepted-frame seam. Such questions include `retained_hit_window`, `pointer_owner_at`, `wheel_reaches_scene_surface`, a Shell pointer/keyboard route, published geometry, or accessibility.

For those tests, finish the entire candidate walk and call `publish_retained_hit_registry` before the first presentation-facing observation. If a test uses a non-default theme, use the underlying `seal_presented_input_candidate(&theme)` followed by `acknowledge_presented_input` rather than this default-theme convenience helper. The seal computes geometry from its theme.

Do not replace every `input.publish_hits()`:

* [wgpu-panel-anchor-model](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:205) measures raw `PanelResize` hit ordering and rectangles. It has no Shell owner, route, or geometry assertion, so direct `InputState` publication remains the narrow unit-test seam.
* The fabricated Chrome drag in [wgpu-shell-input](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1062) and fabricated negative wheel cases in [wgpu-wheel-and-escape-routing](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs:45) deliberately test `InputState` without a registered retained owner. They must retain direct publication.

The two concrete stale General-panel patterns are:

* [settings-general-layout](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:237) publishes raw footer and panel hits, then drives a physical Shell gesture. It must acknowledge a completed frame before the gesture.
* The same test at [line 324](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:324) turns staged General hits into live hits after `render_panel_step`; its geometry/owner result needs the accepted-frame seam.

At [line 224](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:224), the panel is explicitly still pending. A completed `render_chrome_step` has not occurred, so no current renderer path can acknowledge that mixed candidate. Keep this as a staging assertion using `staged_hits`; remove the claim that the footer is already interactive or add a separate prior accepted frame if that is the behavior under test. Promoting it through `InputState::publish_hits` is a stale fixture behavior, not a production regression.

## Minimal Regression Boundary

The test helper must establish one accepted frame before observing or dispatching presentation-facing input. A companion law should keep a newly staged, unacknowledged control absent from `input.hits()` and refuse its route; this preserves the Native99 Button laws' no-fallback purpose. No production early-publication path is justified by these fixture failures.
