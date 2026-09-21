# Settings Six-to-Three Presented-Input Trace

## Conclusion

The current WGPU source has a direct, reachable truncation defect. It constructs all Settings leaves, then silently stops painting the first child row whose next chip would exceed the floating panel width. The same early exit prevents tail leaves from receiving pointer targets, and the ACK path derives chrome accessibility nodes only from those registered targets. Therefore the tail is absent from the browser accessibility mirror's presented DOM as well as from pixels and pointer input.

This is not a missing Default Apps or Conflicts provider.

No build, native test, or browser session was run for this audit.

## Exact source path

### 1. Both products declare the full branch

The full React ShellHost supplies both optional Settings providers at [ShellHost:9212-9216](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:9212). Its Settings factory emits General, Theme when unlocked, Keybindings, Default Apps, and Conflicts at [ChromePanels:1080-1160](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx:1080). App Settings leaves are prepended at [ShellHelpers:1526-1534](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1526).

WGPU puts app BottomRight leaves before the five framework leaves at [Shell WGPU:8762-8779](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8762), and declares the same framework sequence at [Shell WGPU:8800-8813](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8800). Every framework leaf has a retained body builder at [Shell WGPU:20337-20358](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20337).

A persisted dock skeleton is also not the deletion source. Its branch resolver appends each default child not mentioned by the skeleton at [Shell WGPU:4094-4110](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4094).

### 2. The opened Settings panel selects its complete child row

For a bottom anchor, WGPU deliberately omits the root Settings chip from the floating-panel bar and renders the branch children. [Shell WGPU:24017-24026](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24017) makes this explicit. The rows originate from the active dock path and retain every branch child at [Shell WGPU:23996-24010](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23996).

### 3. The WGPU row drops the tail before any candidate exists

The leaf loop at [Shell WGPU:24043-24083](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24043) measures a chip, then executes:

```rust
if x + chip_w > panel.x + panel.w {
    break;
}
```

The `break` occurs before the draw calls, `input.register_hit`, and introduction element registration. It silently excludes the remaining siblings rather than representing horizontal overflow. With a default 300px panel and app-plus-framework Settings sequence, a three-chip prefix is plausible; the exact surviving IDs depend on measured labels, theme, and whether the app supplies a Settings child. The defect is independent of that exact prefix: any tail beyond width becomes unavailable.

The existing WGPU drop-preview loop repeats the same early exit at [Shell WGPU:24091-24109](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24091), so drag insertion cannot address the omitted tail either.

React does not reduce the child list. `PanelTabRow` maps every sorted tab at [PanelTabBar:510-585](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:510), and the shared panel bar is explicitly `overflow-x-auto` at [React target:6871-6875](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6871). React resets the scroll offset and scrolls the active button into view at [PanelTabBar:519-528](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:519).

### 4. Candidate, ACK, pointer, and accessibility all preserve that loss

At frame setup WGPU clears the staging hit registry and accessibility-visible staging list at [Shell WGPU:22778-22789](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22778). An open anchor enters `render_panel_step`, whose phase 7 calls `paint_anchor_tab_bar` before the panel document at [Shell WGPU:24145-24224](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24145).

Sealing records the current candidate, while a successful ACK swaps the current hits, publishes them, and then creates the chrome accessibility nodes at [Shell WGPU:12998-13050](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12998). The chrome accessibility projection iterates only `input.hits()` at [Shell WGPU:29633-29689](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:29633), so an unregistered tail node cannot appear in the presented accessibility tree.

The Interpreter publishes the same chrome projection into the normal dump at [Interpreter:3648-3677](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:3648); the browser mirror only builds an ARIA element for each projected node at [accessibility mirror:64-90](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:64). Thus one paint-time `break` produces the user-visible three-item strip, three-item hit candidate, and three-item DOM/AX list after ACK.

## Bounded repair direction

Port React's per-row horizontal overflow behavior. The row needs a bounded ephemeral horizontal scroll position keyed by the panel anchor and active branch path. Keep the complete dock tree as the declaration authority; do not duplicate it into a shortened display list.

For each frame, iterate every child, translate it by the row offset, intersect its visual content with the panel-row clip, and register a pointer target only for a nonempty visible intersection. Do not break the loop. A horizontal wheel/trackpad route and focus/selection route must update that offset; selecting a tail leaf must move it into view, matching React's `scrollIntoView` behavior. Tail controls must be represented by the presented chrome accessibility model so keyboard/assistive activation can select them and reveal their clipped box. The replacement must preserve a correctly clipped pointer rect: it must never create a hit outside the panel.

This is a panel-tab-row repair. It should not widen the floating Settings panel, increase a capacity, add providers, or register off-panel pointer rectangles.

## Fail-first regression

Extend the existing Shell panel/presented-input harness, which already uses the real candidate seal and ACK machinery, with a neutral Settings branch of six fixed-width leaves under BottomRight and a panel width that admits only three at scroll offset zero.

After the initial accepted frame, require:

1. The dock declaration retains all six ordered IDs.
2. The presented chrome accessibility projection contains all six semantic tab IDs, rather than only the first visible prefix.
3. The initial pointer registry exposes only the physically visible prefix and none of its rects exceed the panel row clip.
4. Advance the actual horizontal panel-tab scroll route. After the next accepted frame, the three tail IDs become visible/hit-testable and the first prefix no longer receives a tail coordinate.
5. Activate Default Apps and Conflicts from the scrolled tail through both pointer and accessibility. Each must set the complete path `[framework.settings, leaf]`, publish its actual retained body, and preserve the active child after a fresh ACK.
6. Re-selecting an offscreen tail leaf must reveal it rather than fold Settings or route to the app Settings leaf.

The paired React oracle should use the same six IDs and constrained panel width, verify all six DOM tab buttons persist under the horizontal scroller, and select Default Apps and Conflicts after scrolling the actual `PanelTabRow`.

## Confidence

High. The exact source condition excludes tail nodes before draw, hit registration, candidate sealing, and accessibility projection. The only unmeasured point is the particular live panel's measured widths, which affects which three IDs appear but does not affect the demonstrated drop mechanism.

