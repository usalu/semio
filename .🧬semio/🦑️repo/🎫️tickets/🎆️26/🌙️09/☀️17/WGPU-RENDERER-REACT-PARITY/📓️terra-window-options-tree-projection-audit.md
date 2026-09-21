# Window Options Tree Projection Audit

## Scope and evidence

This is a source-only audit of the paired checkpoint-20 Window Options images:

- [WGPU image](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/checkpoint20-iab/08-window-options-wgpu.jpg)
- [React image](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/checkpoint20-iab/08-window-options-react.jpg)

The WGPU rail is already physically right-anchored: [`window_measures_rect`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4752) computes its `x` from the body’s right edge. The visible mismatch is the retained document’s flat Field/Section projection, its missing top-right logical flow, and its full-height rect. It is not evidence for a second rail-width or physical-anchor implementation.

No build, browser run, or source mutation was performed for this audit.

## Established React contract

The React producer keeps `WindowMeasure` as the shared semantic input, then builds a specialised tree:

- [`renderWindowMeasure`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:3824) maps groups to `WindowMeasureTreeGroup`, supplies a group header slider when authored, and maps every leaf to `WindowMeasureTreeLeaf`.
- [`WindowMeasuresTree`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4486) provides tree guides, compact rows, and a direction-aware block flow. Its row is a two-column grid using `windowMeasureValueColumnUiSpacing`, not the generic Tree value column ([lines 4455–4479](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4455)).
- [`WindowMeasureTreeGroup`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4518) places an optional header control in the same row as the group and preserves the authored `defaultOpen` state.
- [`WindowMeasureTreeLeaf`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4590) gives a normal leaf a label column and a compact control column.
- [`WindowMeasureToggle`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx:75) renders `TreeCheckbox`, whose native input is `type="checkbox"` and has a single change path ([lines 643–660](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:643)). This is a checkbox control, not a generic pressed icon toggle.

The React Window mounts this content in a `Pane anchor="top-right"` ([Window](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:322)). `Pane` derives `flowFromAnchor(anchor)`, publishes `dir="rtl"` for that anchor, and wraps its body in the matching `FlowProvider` ([React target](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10109)). `flowFromAnchor` explicitly maps a right anchor to RTL ([lines 6998–7001](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6998)). Thus the compact tree’s physical left/right arrangement must be driven by the pane flow, rather than by changing its semantic measure order.

## Current WGPU cause

[`window_measures_overlay_records`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4392) still publishes a `Container::Plain` root. The `WindowMeasuresProjection` then:

- wraps labelled leaves in `ContainerRole::Field` ([lines 4463–4472](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4463));
- emits a generic `ToggleProps { on, icon, text }` for a measure toggle ([lines 4506–4510](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4506)); and
- emits each group as `ContainerRole::Section`, making its optional slider a separate child instead of a row-inline header control ([lines 4512–4529](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4512)).

That exactly selects WGPU’s generic field/section layout and generic toggle painter. The generic retained toggle has no appearance field in either the contract ([`ToggleProps`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:322)) or `UiToggleNode` ([component target](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2104)); `paint_toggle` deliberately paints the bordered icon-toggle treatment ([paint](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:2571)).

There is an existing `window_measure_tree_rows` helper, but it is the Tool panel’s legacy conversion and cannot be reused. It discards group header-slider fields and uses the same identifier for the row and its embedded control ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5312)). Reusing it would create a non-distinct presentation row/control identity and lose a React-visible control.

The retained Tree contract itself already supports the right shape: `Tree`, `TreeSection`, and `TreeItem` records use ordinary child records ([contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:420)), and reconciliation takes the first control child as the row control while retaining remaining children as nested tree items ([reconcile](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:596)). An unlabeled `TreeSection` is a useful carrier: it occupies zero header height ([layout](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:208)).

Two shared renderer gaps remain after changing the projected record shape:

1. Generic retained tree metrics use `CONTROL_VALUE_COLUMN_UI_SPACING` (50) ([layout](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:144)), whereas the React measures tree uses the dedicated `WINDOW_MEASURE_VALUE_COLUMN_UI_SPACING` (32.5) ([styling token](/Users/ueli/Documents/semio/🧰️framework/🎨️styling/🔤️tokens/🦀️.rs:401)). The generic control geometry also pins controls at physical right ([flex](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:288)), which disagrees with the top-right pane’s RTL flow.
2. The Measures rendering path never sets the retained surface flow. The existing exact seam is [`set_ui_document_flow`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2594), and the shared contract already provides `UiFlow::for_anchor(Anchor::TopEnd)` ([layout contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📐️layout/🦀️.rs:362)). Shell calls `render_ui_document_step` directly at [Shell:4727](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4727), without that flow assignment. This is the bounded, existing top-right flow seam; a new bespoke anchor type is unnecessary.

## Recommended bounded repair

1. Replace only `WindowMeasuresProjection`’s general-rail records with a direct retained `Tree` root, one unlabeled `TreeSection`, and recursive `TreeItem` records. A group becomes one `TreeItem` with its authored `default_open`; its first child is the optional header-slider control and its remaining children are nested measure rows. A leaf becomes one `TreeItem` plus exactly one control child.

2. Keep the control child key as the existing `windowId/measureId` binding/address. Give the enclosing `TreeItem` a distinct stable presentation key such as `windowId/measureId.row`. The header slider remains the existing `windowId/groupId.header-slider` identity. This keeps native pointer/keyboard/AX actions on their present keys and prevents the row/control collision in the Tool-only helper.

3. Add a closed tree presentation/metric selection for this semantic tree, selecting the dedicated Window Measures value column and compact row/control dimensions. Apply its flow through the existing per-surface `set_ui_document_flow(surface, UiFlow::for_anchor(Anchor::TopEnd))` before the first ingress/layout opportunity. Flex, tree painter, mounted layout, hit testing, and keyboard routing must consume the same resolved flow; reversing only source-record order would leave the physical control side and input geometry inconsistent.

4. Add a closed `Checkbox` toggle appearance carried from `ToggleProps` through `UiToggleNode` and the inline-tree control painter/accessibility projection. Only the Window Measures projector selects it; ordinary Toggle records retain their current icon-button rendering. The WGPU checkbox must expose checkbox semantics and dispatch the existing `Change` binding exactly once. It must not redefine generic `Toggle` visual behavior.

5. Compute the unfolded Measures rect from accepted `retained_content_height(surface)` plus the top pane-chip clearance, bounded by the body inset and available body height. Until the first accepted layout returns a height, retain the existing safe band rather than guessing a zero-height rect. This follows the existing retained-content-height contract ([engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2719)); the current path always consumes the body height ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4752)). Pane chips already paint after the Measures rail and therefore retain their foreground/input precedence ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23818)).

## Fail-first laws

The current native Window Measures suite already contains an unrun intended regression shape: [`window_options_publish_compact_tree_rows_and_checkbox_controls`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs:18) expects a `Tree` root, `TreeItem` labels, the original control key, and wire `appearance: "checkbox"`. It is backed by the neutral [tree-parity fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🌳️window-measures-tree-parity/🔣️.json), which also pins a 104-pixel control column, 20-pixel maximum row height, a closed group, and a 0.85 source intensity.

Keep that law and add these behavioral cases rather than substituting source-string assertions:

- React mounted Window oracle: open the top-right rail; verify its checkbox has native checkbox semantics, the group header slider remains in the group row, the closed group hides its child, and the right-anchor flow places the compact control column on the physical left.
- Native accepted-paint law: publish the same fixture, set the surface’s `TopEnd` flow before layout, seal/ack, then assert the exact control rect is on the physical left, its row label is on the right, and the chip-clearance/intrinsic-height bounds hold after accepted layout.
- Native interaction law: pointer press/release and keyboard Space on `grid-visible` produce one `setVisible` action with the existing control key and `pressed` argument; group-row disclosure produces no control action; the header slider produces its own existing change binding.
- Compact-metric law: the Tree’s Window Measures presentation uses the dedicated 32.5-token column rather than the generic 50-token column, and rows remain at or below the fixture’s 20-pixel bound.

The observed `Sun intensity` and `Spacing` values are included in the parity fixture as source-state values, but this audit did not establish a separate WGPU author-state mismatch. The tree projection must preserve those values without inventing a renderer-side correction; a fresh matched action/state probe is needed before assigning any remaining numeric difference to the producer.
