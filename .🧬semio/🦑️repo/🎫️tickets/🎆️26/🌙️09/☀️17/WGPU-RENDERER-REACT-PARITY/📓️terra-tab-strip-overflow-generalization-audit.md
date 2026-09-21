# Shared Tab-Strip Overflow and Accessibility Audit

## Finding

The Settings six-to-three fault is one instance of a shared WGPU tab-strip cutoff. The desktop floating-panel painter stops iterating at the first chip that exceeds the row width. The same condition appears in the drag-insertion calculation and in the mobile flattened strip. It removes a tab before it can be painted, hit-tested, exposed to the chrome accessibility model, or activated.

No build, native test, or browser session was run for this audit.

## Reachable shared paths

### Desktop panel child rows

[`anchor_tab_rows`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23996) creates a root row and then one active-path branch row per level. [`anchor_panel_tab_rows`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24020) gives top and bottom anchors only their descendant rows because their root rows live in navbar/footer chrome; side anchors retain their root row in the floating panel.

Every one of those panel-owned rows reaches [`paint_anchor_tab_bar`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24043), which executes `break` at [line 24060](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24060) before painting, hit registration, and introduction geometry. Thus constrained nested rows at all nine anchors can lose their tail. BottomRight Settings only made the defect conspicuous.

The top/bottom root strips use a different chrome-band clipping path, [`footer_tab_row_rect`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23501) and related navbar walkers. It clips each independently through [`shell_chrome_clip_horizontal`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16665), rather than using the panel loop's early exit. That is a distinct geometry policy and is not evidence that root strips already provide a scroll/reveal interaction.

### Drag and mobile

[`dock_tab_insert_preview`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24091) repeats the same early exit at [line 24100](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24100). [`dock_tab_drop_index`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23942) independently recomputes row geometry without any scroll offset. A repair must share the same row layout/offset with paint, preview, and drop targeting. Otherwise a tail tab can render under a translated coordinate while a drop is interpreted at the unscrolled coordinate.

The mobile flattened row has its own early exit at [`paint_mobile_tab_bar`:24378-24397](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24378). It needs the same underlying row protocol, with its mobile control-id convention preserved.

## React behavior

React's [`PanelTabRow`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:512) maps every sorted tab. The row is horizontally scrollable through [`panelTabBarScrollClass`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6871), and on active-ID or membership change it resets then calls `scrollIntoView` for the active button at [lines 519-528](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:519). Every child remains a native focusable button with its pressed state at [`PanelTabButton`:388-490](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:388).

## Accessibility and keyboard boundary

WGPU currently makes painted pointer targets the sole chrome accessibility authority:

- A completed ACK calls `chrome_accessibility_nodes(input.hits())` at [Shell line 13047](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13047).
- [`chrome_accessibility_nodes`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:29638) enumerates only those hits.
- [`handle_accessibility_event`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13474) re-creates that list and refuses a target that is absent. It then finds the same visible hit before dispatching activation.
- Normal `Tab` handling cycles dock windows at [Shell lines 15397-15405](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15397); there is no panel-tab focus/navigation or reveal route.

A no-`break` visual patch would therefore still fail semantic discovery and activation if it omits clipped tabs from pointer hits, while registering full off-panel hit rectangles would violate the physical ownership boundary.

## One shared repair scope

Add one bounded, ephemeral **panel-tab row presentation state** keyed by the rendered row identity: anchor, row parent path, and mobile/desktop variant. It owns only horizontal offset and focused/active reveal state. It is not a duplicate dock tree.

Have one bounded row-layout routine produce every child's untranslated interval, translated visible interval, and row clip. Paint, pointer registration, drag preview, and drop index consume that routine. The loop must visit every declared child; it must not stop at the first overflow.

Pointer policy:

1. Draw only each interval's intersection with the row clip.
2. Register a pointer target only when that intersection has positive area.
3. Use that same translated geometry for preview/drop, preserving drag exclusions and the existing half-chip insertion rule.

Semantic policy:

1. Build the presented chrome accessibility row from all current-row declarations, not only from `InputState::hits`.
2. Give offscreen members their normal tab identity, role, pressed state, and no pointer rectangle.
3. Resolve an AX Focus or Activate through a presented row-member resolver validated against the currently ACKed anchor/path declaration. It may call the existing `panel_tab_anchor`/selection path; it must not synthesize an off-panel pointer hit.
4. Focus or activation moves the row offset just enough to expose the tab. Its pointer rectangle becomes live only after the following accepted frame.
5. Retire row state when that exact row declaration/path ceases to be presented, so a replacement branch cannot inherit its offset or focused ID.

This joins all affected anchors and mobile into one component-level behavior without changing root chrome bands, panel dimensions, tab declaration, or input capacity.

## Fail-first law

Extend the actual candidate/ACK panel harness in [`settings-general-layout`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:351). Its helper already drives `render_panel_step` and `publish_retained_hit_registry`, avoiding a direct-paint fixture.

Use six stable child IDs in a constrained row under BottomRight Settings, then repeat for one side-anchor root row and the mobile flattened row.

After each accepted frame require:

1. The declared row retains all six IDs in order.
2. The presented accessibility projection retains all six tab identities and correct pressed state, including clipped members.
3. Pointer hits are exactly the visible intersections and stay inside the row clip.
4. AX focus/activation of a clipped tail preserves the declared anchor/path, changes only the bounded row offset, and exposes the target after the next ACK.
5. A pointer release on the newly exposed tail selects its actual retained body. A pointer at the old coordinate cannot select it.
6. A drag preview and final drop use the translated row geometry and do not insert according to the old unscrolled coordinate.
7. Closing or changing the active branch retires the row state; reopening a same-anchor successor starts at React's reset-and-reveal state.

Use React's actual constrained `PanelTabRow` as the paired oracle: all six buttons remain in the DOM, focus/selection reveals the tail, and no pointer activation occurs outside the scroller's visible bounds.

## Confidence

High for the shared truncation, AX coupling, and absent WGPU panel-tab keyboard route. The recommended state key and exact horizontal input gesture are design recommendations; this audit did not execute a browser or native behavioral probe.

