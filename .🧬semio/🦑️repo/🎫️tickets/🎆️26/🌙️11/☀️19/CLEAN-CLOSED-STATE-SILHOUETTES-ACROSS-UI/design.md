# 🫧 Closed State Silhouettes: Design

## Problem (observed live, elements story site, 2026-11-19)

- **ToggleGroup**: with a focused item the whole group outline turns active red; hovering an item recolours the whole group outline grey; the hovered/selected item itself never gets an outline.
- **Window chips / cell rows**: a hovered or selected cell shows only its two vertical dividers in state colour (the previous cell's right border via `:has(+ …)` and its own right border); its top and bottom belong to the owner and stay normal: an open, bracket-shaped silhouette.
- Root causes:
  1. `BorderCompositor.state()` lets every framed ancestor take a descendant's hover (`element.contains(hovered)`) and focus (`element.contains(activeElement)`).
  2. Cells of a `data-shared-border-cells` owner are not compositor participants. Their dividers are CSS borders, and their state is faked with sibling `:has(+ …)` border-colour rules (`🎨️.css` ~6864–6915), so a cell can never close its outline.
  3. Per-element CSS state lines (`border-color` on hover, `focus-visible` rings and outlines) compete with the compositor, which leads to doubled or partial lines.

## Mechanism

One model, the same on the web, native wgpu and the TUI:

1. **Boxes.** Every window silhouette, shared-border frame, and cell of a shared-border owner is a *box*. A box has an id, a parent box, a rect, the edges it draws, and whether it is *interactive*, meaning it is itself a button, link, tab, option, menu item, treeitem, switch, checkbox, radio, form field or `[tabindex]`, or carries `data-shared-border-interactive`. It also has its own state markers, `disabled`, and its *owner* (the overlay that paints it). A cell's owner is its cells-owner frame; every other box owns itself.
2. **State ownership** (`resolveBoxStates`, cases in `states.cases.json`). A box's state comes only from the box itself:
   - **hover** goes to the innermost interactive, enabled box on the path from the pointer's box to the root. No other box is hovered.
   - **focus** goes to the innermost interactive box on the path from the `:focus-visible` element's box to the root. Pointer focus never paints.
   - **active, waiting, loading, introduced, celebrated and explicit `data-shared-border-state`** apply to the marked box only. A cell whose single interactive child carries the marker counts as marked.
   - The final state is the highest priority among these: hover > focus > celebrated > introduced > loading > waiting > invalid > active > normal. Transient interaction states outrank persistent ones, so a neighbour's selection can never open a focused or hovered silhouette.
3. **Silhouette edges** (`boxSilhouetteSegments`, cases in `silhouettes.cases.json`):
   - A normal box contributes the edges it declares. A side that lies flush on an edge its parent does not draw is still suppressed.
   - A stated box always contributes all four edges, so its outline is closed even on a navbar's undrawn top.
   - Cells contribute their four edges in their owner's overlay. Coincident cell edges resolve to one divider, and cell edges on the owner's outline resolve into that outline. There are no CSS divider borders on cells.
4. **Resolution** (unchanged `resolveBorderSegments`). On every collinear overlap the highest-priority state wins. Every stated box's perimeter is therefore fully painted, in its own state or in a stronger state of a neighbour. Corners are square with a miter join, at the same snapped device pixels as the outline.
5. **One painter.** Lines that show state are painted only by the compositor:
   - Participants carry no CSS `border-color`, `outline`, ring or `box-shadow` that changes with state.
   - Backgrounds and fills stay free.
   - Forced colours map the state strokes (`Highlight`, `CanvasText`) inside the overlay.
6. **Clipping.** A stated silhouette is clipped only by a real scroller or clipping ancestor, never by its own owner. The overlay is inset by one device pixel and has `overflow: visible`.

7. **Borderless controls.** Tabs, tree rows, menu items, icon buttons and similar controls become boxes with `data-shared-border-edges=""`. In the normal state they draw no line. When stated, they draw their closed silhouette, which replaces the browser focus outline and any CSS rings or underlines. The compositor creates their overlay only while they are stated, so long lists cost nothing at rest. Fill-based hover and selection stay on top of the silhouette as decoration.

## Closure invariant (gate)

For every box with state `s ≠ normal` that is fully inside its clip, every point of its perimeter is covered by a resolved segment. Each covering segment's state is `s` or a higher-priority neighbour state, and no second parallel line of a state colour lies within 2 px of it. The live probe `silhouette-probe.js` checks this on the painted overlay paths, and the TS and Rust case tests check it on the logical segments.
