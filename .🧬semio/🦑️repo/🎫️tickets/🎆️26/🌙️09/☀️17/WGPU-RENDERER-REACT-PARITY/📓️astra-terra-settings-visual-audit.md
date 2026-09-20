# Terra Settings General Visual Audit

## Scope and evidence

Read-only audit of the recorded General-settings checkpoint and the retained WGPU path. No source, fixture, or test was changed, and no build, Cargo test, or browser run was started.

The compared evidence is exactly:

- `🗑️generated/astra-runtime/paired-checkpoint-10/react/06-settings-general.png`
- `🗑️generated/astra-runtime/paired-checkpoint-10/wgpu/06-settings-general.png`
- the recorded `settings-general` step in the checkpoint's `steps.json`.

The step is an actual General-settings interaction. The later manual evidence also establishes that General tab selection lands after the worker frame, and that pointer activation of Appearance eventually opens the select (`expanded: true`). This audit therefore does **not** classify the mismatch as an inactive General tab or a missing select action.

## What the checkpoint shows

React renders a compact end/bottom panel. Its Tree grows upward: the Driver editor is initially collapsed, property controls are inline at the right, and the dock path rows sit at the panel's bottom in the reverse order required by that anchor.

WGPU renders a tall, top-first field document. The Driver editor's seven selects, name field, and actions are visible even though its authored `default_open` is false. Every setting label is above a full-width select. Its root and leaf tab path rows are painted at the top. The added document height makes the bottom-anchored panel hug the far larger content and move upward, which accounts for the apparent root/child tab-bar duplication. Do not remove a tab-path row before the bottom-flow correction is measured: the available evidence identifies wrong placement/order, not a second data record.

The later Appearance evidence is a separate visual blocker: after a successful open, popup glass appears around `(0..293, 175..258)` while the trigger is around `(979..1273, 225)`. The inline `System` text also shifts over its field. This is an overlay coordinate/compositing defect, not evidence that the event router failed to open the menu.

## Parity trace

| Layer | React oracle | WGPU path | Finding |
| --- | --- | --- | --- |
| Settings schema/data | `📌️ChromePanels/🟦️.tsx` `buildSettingsGeneralTree` builds app, General, and Driver editor sections with the same stable control ids, values, options, and actions. | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `build_settings_general_ui` builds the same control data. | The General values/actions are largely data-parallel. The visual failure is not an Appearance-value data mismatch. |
| Projection | React projects the data as `TreePanelConfig` / `TreeDataSection` / `TreeDataItem`, whose `control` is a right-hand property cell. | `build_settings_general_ui` makes `Stack → Section → Field → Select`; `settings_select_row_with` authors `UiNode::Field`. `PanelProjection` faithfully contracts that generic hierarchy. | The producer shape is non-isomorphic before retained layout begins. A Field necessarily yields label-above-control presentation. |
| Collapsed Driver | `🌳️Tree/🟦️.tsx` owns `defaultOpen` state and only mounts `CollapsibleContent` while open. Its `up` direction reverses sections/items and renders children before their trigger. | `🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` admits and unwinds every `UiNode::Section` child without consulting `default_open`. `🖌️paint/🦀️.rs` paints the Section chevron but `paint_stack` still walks all children. | Confirmed retained layout/paint defect: `default_open: Some(false)` changes the chevron only, so the Driver editor contributes its full height and paints. |
| Panel shell | React's end/bottom Tree flow puts content above the tab path. | `render_panel_step` calls `set_ui_document_flow(window, anchor.flow())`, but `anchor_content_rect` always starts below `anchor_tab_bar_height` and `paint_anchor_tab_bar` always increments downward from `panel.y`. `anchor_content_height` then hugs the retained document height. | The document receives flow, while the dock chrome does not. Bottom-anchor tab ordering and content origin remain top-first. |
| Overlay | A React Select portal is positioned in the panel's screen coordinate system. | `resolve_overlay_placement_side` deliberately returns a **window-local** location. `Ui::frame_into_step` is meant to add the passed document bounds offset to overlay chrome, body paint, and hit registration. | Runtime evidence proves that an offset is still lost on the live path. The immediate seam is overlay paint/composition, not activation. The expected offset exists in the retained-engine API, so trace it with a non-zero document origin before changing placement policy. |

### Exact producer and retained seams

1. `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
   - `build_settings_general_ui` and `build_settings_driver_editor` are the product producer.
   - `settings_select_row_with` converts React-style property rows into `UiNode::Field`.
   - `PanelProjection::node` maps the selected `UiNode` hierarchy into the retained document. It already has Tree vocabulary, but this producer does not use it.
2. `🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` and `🖌️paint/🦀️.rs` are the generic Section defect. Fixing only the chevron cannot correct measurement, clipping, hit registration, or child paint.
3. `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `anchor_panel_rect`, `anchor_content_height`, `anchor_content_rect`, `paint_anchor_tab_bar`, and `render_panel_step` are the panel flow seam.
4. `🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` resolves local overlay placement. `⚙️engine/🦀️.rs` `frame_into_step` is responsible for applying the non-zero document offset to both overlay chrome and overlay body. The retained paint walk reads `UiTree::overlay_origins` for both paint and hit geometry.

## Bounded implementation packets

### P1 — General Tree projection and bottom panel flow

1. Replace only the General producer's `Stack/Section/Field` hierarchy with one `UiNode::Tree` containing app, General, and Driver editor `UiTreeSectionNode`s.
2. Represent every setting as a `UiTreeItemNode` with its existing Select, Input, or Button as `control`. Preserve all existing ids, values, options, controller actions, lock gates, and `default_open` values. The Driver section remains initially closed.
3. Make the panel shell honor `anchor.flow().block` when choosing the document rect and tab-bar origin/order. For an upward/bottom anchor, reserve the tab path at the panel bottom and place document content above it. Keep root/leaf rows as one path; reverse their visual progression only when the anchor flow requires it.
4. Retain content-height calculation, but measure the Tree after Driver's closed branch is excluded. The expected result is the compact panel, inline-right controls, closed Driver, and bottom dock tabs seen in React.

This is the smallest product-parity packet because it changes the WGPU representation to React's actual Tree contract. It should not be replaced with a Field styling patch.

### P2 — Generic retained Section collapse

Implement persisted Section open state in the retained UI module: use `default_open` only to initialize state; do not admit, measure, paint, or register hidden descendants; and invalidate layout when the section header toggles. This remains a real framework defect even after P1, but is broader than General because P1 should use Tree semantics immediately.

Keep P2 independently testable. It must prove collapsed children have no rect, no paint output, no hit target, and no accessibility projection, then prove opening restores all four.

### P3 — Select overlay origin

Do not alter `Select` activation or its worker timing. Add a retained-engine test which opens a Select in a `frame_into_step` viewport with a non-zero origin comparable to the observed panel position. Assert that the overlay chrome bounds, popup body glyph/quads, and registered option hit rect all equal local placement plus that viewport origin.

If that test passes, the defect is downstream in the shell's draw-layer/compositor route: trace the document's overlay route from `render_ui_document_step` through `DrawList::begin_overlay_route` to final composition, preserving the same absolute rect for glass, text, and hits. If it fails, repair the one missing translation in `frame_into_step`/retained overlay publication. In either case, use one coordinate authority; do not pre-offset placement and then offset again during paint.

### P4 — Accessibility visibility publication

The four old app Settings steppers in the global mirror are consistent with retained-document lifetime, not proof that the hidden panel is being painted. `refresh_ui` deliberately retains shell and application panel documents, while `render_panel_step` paints and registers only an anchor's active leaf. In contrast, `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` `build_accessibility_dump(None)` enumerates every `engine.window_id`; its existing introspection test explicitly asserts this behavior.

Publish a shell-owned visible-document set (active dock leaf documents plus visible overlay roots and `shell.chrome`) to the browser accessibility mirror. Filter an unnamed production mirror dump through that set, and gate accessibility event routing through the same authority. Keep an explicitly requested diagnostic window dump available for introspection. Do **not** retire inactive documents just to remove them from the mirror; retention is necessary for panel state and is separate from visual/AT exposure.

The remaining AX Select issue is separate: successful expanded state with no mirrored listbox/options needs an accessibility overlay-tree packet. It must not be used to re-open P3's solved activation path.

## Existing oracle and fixture seams

- Best neutral geometry seam: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json` with its Rust test. It already owns Tree row rect and nested/collapsed-height assertions, so extend it with a property control and upward-flow case rather than inventing Settings-specific layout math.
- Best mounted React geometry oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️tree-row-rects/🟦️.ts`. Use it to produce the matching Tree row/control ordering expectation for the product fixture.
- Best mounted React accessibility oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx`, paired with `🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json`. Extend it for visible-window pruning and a Select listbox after an accessibility activation.
- `🧫️fixtures/🚗️driver-editor/🔣️.json`, its React test, and the Shell Rust test are the semantic driver-axis seam. They validate values/actions but not the required Tree geometry or collapsed branch visibility; keep that distinction explicit.

## Acceptance evidence for the next executor

At the same checkpoint viewport and dark state:

1. General selection exposes only the General Tree in the visible panel and its accessibility mirror; inactive app Settings steppers are absent.
2. App and General are open; Driver editor is closed and contributes no visible or measured child rows.
3. Appearance, Layout, Driver, Language, and Terminology controls sit inline at the tree row's end edge; the panel hugs the compact Tree height at bottom right.
4. Root/leaf dock path rows are at the bottom and follow the React upward ordering.
5. Opening Appearance puts popup glass, text, option hit rects, and the AX listbox beside the trigger, never at the canvas origin.
6. Pointer and accessibility activation each settle after their expected worker frame without duplicate actions or focus escaping to the General tab.

