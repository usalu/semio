# 🔎 Round 3 Audit — Framework

# Round 3 static audit: framework (remaining shared-border violations)

Base path for every file below: `/Users/ueli/Documents/semio/`. Shorthand used in tables: `UI/` = `🧰️framework/🔨️modules/🖱️ui/`, `COURSE/` = `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/`, `OS/` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`.

Method: grep of every border token, ring/outline, divide, `-ml-px`/`-mt-px`/`border-*-0`/`first:`/`last:` hacks, spacing classes on lines carrying borders, and CSS border/margin/gap rules across `🧰️framework`. Hits were read in context. I did not run the live probe (read-only). Coverage is exhaustive for `UI/` elements and styling. Products (course, os renderer) were checked by grep and read in context. Other products and modules were only sampled, so treat them as partial.

## Root-cause finding (fix this first)

The compositor does not turn off the native `border` of a frame whose `data-shared-border-edges` is a subset. The overlay draws only the listed sides, so the native border still draws the rest. That gap is why call sites add `border-t-0` by hand (F1). The generic fix is a CSS rule such as `[data-shared-border-frame][data-shared-border-edges]:not([data-shared-border-edges~="top"]) { border-top-width: 0 }`, repeated for each side. The `data-shared-border-edges` value then drives native sides too, and the call-site hacks go away. The navbar/footer `border-width: 0 !important` at `UI/🎨️styling/🖌️ui/🎨️.css` ~6580 is a per-element version of the same idea.

## Ranked findings

### High visibility (core primitives)

**F1. Ad-hoc `border-t-0` on partial-edge frames (category 4)**
- `UI/🧱️elements/🎮️Engagement/🟦️.tsx:313`: `<ButtonGroup id={UI_ENGAGEMENT.actions} {...(optionsFirst ? { className: "border-t-0", "data-shared-border-edges": "right bottom left" } : {})}>`
- `UI/🧱️elements/🪟️Window/🟦️.tsx:382`: `<ActionGroup ... className="border-t-0" data-shared-border-edges="right bottom left">`
- Why: the top edge is removed by hand so the native border does not double the edge above. The mechanism already knows the edges, so the hack duplicates it. Window quick actions are visible in every window.
- Fix: add the generic edges-to-native-width rule (see root cause) in `🎨️.css` near the `[data-shared-border-cells] > [data-shared-border-frame]` block (~6713). Then delete `border-t-0`. Also confirm the `data-shared-border-edges` prop wins over the `ActionGroup` root default at `UI/🧱️elements/⚡️ActionGroup/🟦️.tsx:64`, which hardcodes `top right bottom left`. If the prop is spread before that attribute, the override never takes effect.

**F2. Session ring plus gap-separated children in Engagement and WindowSearch (categories 1, 2)**
- `UI/🧱️elements/🎮️Engagement/🟦️.tsx:285`: `flex w-full min-w-0 max-w-full flex-col gap-half` with `sessionActive && "bg-active-base ring-accent/35 rounded-sm ring-1 shadow-sm"`. The children are `EngagementControlView` rows (`:293-296`, likely framed inputs or selects) and the `engagement-options` group (`:306-313`), separated by `gap-half`.
- `UI/🧱️elements/🔍️WindowSearch/🟦️.tsx:416`: the same class string with the same ring.
- Why: if the control rows are framed, `gap-half` separates bordered siblings. The `ring-1` is a second outline drawn outside the element, which is a parallel line, not a shared one.
- Fix: make the column a `data-shared-border-group data-orientation="vertical"` with framed rows as cells. Remove `gap-half` and put the padding inside the text cells. Replace the ring with the `shared-border-priority` accent segment (the same way `:focus-within` and active seams are drawn). Check with the probe first, since the framed-ness of `EngagementControlView` was not traced.

**F3. Border plus ring double line on avatars (category 1)**
- `UI/🧱️elements/📻️TableAvatar/🟦️.tsx:62`: `!isFullSize && "border"`
- `:65-66`: `isSelected && "ring-1 ring-[color:var(--active-base)]"`, `isHovered && "ring-1 ring-[color:var(--hover-base)]"`
- Why: on hover or selection, the native border and a 1px ring sit one pixel apart around the same circle, so the rule gets two concentric lines.
- Fix: when selected or hovered, recolour the border instead of adding a ring (`border-[color:var(--active-base)]`). Alternatively drop the native border and use an inset `box-shadow`, the same pattern as the context-menu swatch at `UI/🧱️elements/🖱️ContextMenu/🎨️item-render/🟦️.tsx:49`, which is already fixed.

**F4. Concentric inset boxes in the scene skeleton (category 2)**
- `UI/🧱️elements/🎬️Scene/🟦️.tsx:504-506`: `absolute inset-0 border-4 ... rounded-lg`, `absolute inset-2 border-2 ...`, `absolute inset-4 border ...`
- Why: three bordered boxes nested inside each other, each inset from the last. This is a box-in-box, unless it counts as art.
- Fix: this is a loading glyph in a 3D viewport, so it is content art. Either mark the root `data-border-probe-art` and list it in the report, or redraw it as one framed shape with no nesting. The Scene element (`UI/🧱️elements/🎬️Scene/🟦️.tsx`) is not marked as art today.

**F5. Tree file card: native border with padded content and a sibling suppression rule**
- `UI/🧱️elements/🌳️Tree/🟦️.tsx:638`: `<div data-slot="tree-files" className={\`not-prose border bg-card ${className}\`}>`. No `data-shared-border-frame`.
- `:641`: `<div className="flex flex-col gap-single p-medium">`. The `p-medium` insets every row.
- CSS `UI/🎨️styling/🖌️ui/🎨️.css` ~7393: `[data-slot="tree-files"] + [data-slot="tree-files"] { border-top-width: 0; }`. This is a hand-written seam, one side only.
- Why: the card is a bordered box that is not a shared frame, padded over its contents, and stacked by a CSS hack instead of the compositor.
- Fix: `data-shared-border-frame` with `data-shared-border-edges="top right bottom left"`. Make the rows a `data-shared-border-group` (vertical) for consistency. Move `p-medium` onto the title and list wrappers. Delete the `+` sibling rule, because adjacent frames then share one line.

**F6. Floating bordered overview button inset in the layered overlay (category 2)**
- `UI/🧱️elements/🥞️LayeredOverview/🟢️overview-button/🟦️.tsx:16`: `ui-glass absolute right-double top-double z-40 ... border border-border-normal ...`
- Why: a bordered box placed `double` units inside the overlay's top-right corner, not on an edge. This is an inset bordered element inside the overview layer.
- Fix: anchor it to the overlay edge (`top-0 right-0`) and make it a cell of the overlay's chrome band (`data-shared-border-cells` on the band), so its edges become the band's edges. Alternatively drop its border and let the band draw the seam.

### Medium

**F7. Chip stack with negative-margin overlap (category 4, CSS)**
- `UI/🎨️styling/🖌️ui/🎨️.css:1767`: `.introduction-demo-callout-chip { margin-block-start: calc(-1 * var(--stroke-hairline)); border: var(--stroke-hairline) solid var(--accent-foreground); ... }`, used with `.introduction-demo-callout-chips` at ~1736 (`gap: 0`).
- Why: each chip has its own full border and the stack relies on a negative margin to overlap the borders. That is the `-mt-px` hack the brief names. Only the intro demo uses it, but it is shown live.
- Fix: put the chips in `data-shared-border-group data-orientation="vertical"` and make each chip a `data-shared-border-cell`. Remove the margin and the chip's own border. The rule `:is([data-shared-border-frame], [data-shared-border-group])[data-orientation="vertical"] > ...` at `🎨️.css` ~6740-6760 already draws the dividers.

**F8. Dashed drop zone with rounded corners inside the tab strip (category 2 and the dashed rule)**
- `UI/🧱️elements/🖼️Panel/🟦️.tsx:310`: `flex h-medium min-w-[10rem] items-center justify-center rounded-sm border border-dashed`, `borderNormalClass`, `dropZoneReadyFillClass`.
- Rendered by `PanelEmptyDockZone` (`:296`) and, via `🎯️targets/⚛️react/📑️panel-chrome-tab-bar/🟦️.tsx:71`, inside the tab row.
- Why: a dashed, rounded, own-bordered box in a shared tab row, which the brief lists as a violation. It appears only during a dock drag.
- Fix: make it `data-shared-border-cell`, with the drop indication drawn as the accent segment of the row (`shared-border-priority`). Remove `rounded-sm` and `border-dashed`.

**F9. Floating button and drop-indicator rings on bordered rows (transient, categories 1 and 4)**
- `UI/🧱️elements/📊️Table/🟦️.tsx:258`: `borderNormalBottomClass` plus `isOver && ... ring-2 ring-active` on the `tr`.
- `UI/🧱️elements/🧭️PanelTabBar/🟦️.tsx:470`: `isChildDropTarget && "ring-2 ring-accent"` on a tab that is also `panelTabButtonDividerClass` (a shared cell).
- `UI/🧱️elements/🎨️Canvas/🟦️.tsx:132`: `absolute rounded-sm border-2 border-accent bg-accent/20` (mode-dock split drop indicator).
- Why: a ring or a 2px border drawn over a seam adds a second line next to the shared one during drags.
- Fix: draw the drop target with the shared-border accent segment, not with rings or new borders. Keep the rounded corners off.

**F10. Input and textarea: frame overlay plus native child border at the same coordinates (cleanup)**
- `UI/🧱️elements/✏️Input/🟦️.tsx:491`: `data-slot="input-root" data-shared-border-frame ...` with no native border of its own.
- `:512-514`: the `<input>` has `border ... ${borderElementClass}`.
- `:297` (collapsed display): `border` on a child of the same frame.
- `UI/🧱️elements/🔤️Textarea/🟦️.tsx:113` and `:121` do the same.
- Why: the overlay and the native child border sit on the same edges and can diverge in colour during state changes. Probe false positive is likely, since the distance is zero. The double paint is still a structural duplicate.
- Fix: extend the cell rule to standalone frames: `[data-shared-border-frame] > :is(input, textarea) { border-width: 0; border-radius: 0; }` in `🎨️.css` near 6790, so the overlay alone draws the edge.

### Product-level (course, OS renderer)

**F11. Gap-separated bordered fallback buttons (category 1, low traffic)**
- `COURSE` `🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx:39` and `:42`: two `rounded-md border border-border` buttons in `span.flex flex-wrap gap-single` (`:38`).
- Fix: `data-shared-border-group` with both buttons as `data-shared-border-cells`/cells, `rounded-none`, no gap.

**F12. Hand-made divider on card actions (category 4)**
- `COURSE` `🔨️modules/📖️quiz-page/🟦️.tsx:50` and `:58`: `CardAction className="border-e border-normal"` on the "last result" and "resume" actions, with the primary action last.
- Fix: put the actions in a shared-border row (`ROW`/`CELL` helpers in `COURSE` `🔨️modules/🪟️chrome/🟦️.tsx:133-135`) so the divider comes from the mechanism.

**F13. Hand-rolled dividers in course layouts (category 5, migration candidates)**
- `COURSE` `🎨️styles/🧩️task-layouts/🎨️.css`: `.course-cell` (`:89-91`), `.course-chip > select` (`:112-124`), `.course-slot > .course-guess` (`:302-323`), `.course-match` cells (`:351-353`), and `.course-classify` (`:140-146`). Each sets `border-inline-start`/`border-block-start` by hand in place of the shared rules.
- `COURSE` `🎨️styles/🏁️results-and-preferences/🎨️.css:36-38` does the same for `[data-crowd]`.
- Why: these are per-product copies of the compositor's divider rules. I did not trace the DOM, so I cannot prove a both-sides seam here. They are at risk and should be migrated.
- Fix: rely on `data-shared-border-group`/`data-shared-border-cell` (already present on most of these rows) and drop the hand-written rules, or keep only the rules that set the container edges.

**F14. Native `border-t` stacked bands and a 1px divider (low)**
- `OS` `🧱️elements/✏️TextEditor/🟦️.tsx:662`, `:668`, `:679`: three bottom-stacked bands, each with `border-t border-border`.
- `OS` `🧱️elements/🔺️DiffViewHost/🟦️.tsx:127`: `<div className="border-border border-l" />` in a grid with `gap-x-single`, so the divider is separated from its neighbours by a gap.
- Fix: use `SharedBorderRow` with `edges={["top"]}` for the bands, as `DialogFooter` does (`UI/🧱️elements/💬️Dialog/🟦️.tsx:596`). For the diff, remove the gap or use a cell divider. The diff is content, so it may be exempt.

**F15. Dashed alert box (low, error state)**
- `OS` `🧱️elements/🗣️Interpreter/🔤️display-views/🟦️.tsx:152`: `rounded-md border border-dashed` alert for unknown components.
- Fix: this is a standalone error notice, so a dashed bordered box may be acceptable. If it sits in a row, make it a cell.

## Low or cosmetic

- `UI/🎨️styling/🖌️ui/🎨️.css` (`:` for `[data-shared-border-frame] table > tbody > tr:last-child`, the rule near 7400): suppresses the last row's bottom border by hand. It is consistent with the last-cell rule, but it is a special case. Route tables through the cell mechanism instead.
- `UI/🎨️styling/🖌️ui/🎨️.css` ~6581 and ~7560 (`border-width: 0 !important` on navbar, footer, dialog-content, intro box): the owner paints the edge, so the native border is zeroed with `!important`. Works, but it is the same missing generic rule as F1.
- `UI/🧱️elements/🗂️WindowChrome/🟦️.tsx:539` and `:551`: `border-0 ... gap-single` on the enlarge and close buttons. These are one control with an icon and text, so the gap is internal and acceptable.
- `OS` `ShellHost` floating notices (`🏛️ShellHost/🏛️...:245`, `:296`, `:417`, `🚪close-guard:36`, `🍞transient-notice:17`, `🔖session-notice:15`): standalone overlays with their own `border`. Not inset in a framed parent, so probably fine.
- `OS` `NodeGraph/🟦️.tsx:182` legend chip (`right-2 top-2 border`) and `UI/🧱️elements/🕸️Diagram/🧩️inner/🟦️.tsx:231` `MiniMap className="border"`: overlays on diagram content. Treat as art-adjacent. Mark the diagram root `data-border-probe-art` if it is inset.
- `OS` `InkCanvasHost` `rounded border` and table cells, and `NodeGraph` selection boxes `border-2 border-accent`: content art, exempt.

## Verified already fixed (do not repeat)

- Round 2 F1 (Canvas app-mode nav padding): `UI/🧱️elements/🎨️Canvas/🟦️.tsx:200` now uses `data-shared-border-cells` and no padding.
- Round 2 F3 (presence roster gap): `UI/🧱️elements/👥️PresenceBar/🟦️.tsx:92` is a `SharedBorderRow`.
- Round 2 F4 (UI dialog arguments gap): `UI/🧱️elements/📨️UIDialog/🟦️.tsx:120-135` is `SharedBorderRow` with edges.
- Round 2 F5 (dialog padding and gap): `UI/🧱️elements/💬️Dialog/🟦️.tsx:553` has `gap-0 ... p-0`, and the footer is a `SharedBorderRow` with `edges={["top"]}`.
- Round 2 F6 (popover padding over framed children): `UI/🧱️elements/🗨️Popover/🟦️.tsx:423-427` uses `data-shared-border-cells`, `p-0`.
- Round 2 F7 (tree card margin): `my-medium` is gone from `UI/🧱️elements/🌳️Tree/🟦️.tsx:638`.
- Round 2 F8 (tree control gap): `UI/🧱️elements/🌳️Tree/🍃️item/🟦️.tsx:287` uses group and cells.
- Round 2 F9 (tabs list padding): `UI/🧱️elements/📑️Tabs/🟦️.tsx:157` is `p-0` with a group.
- Round 2 F13 (drag-preview margins): `UI/🧱️elements/🎨️Canvas/🧲️drag-preview/🟦️.tsx:9` has no margin and no border.
- Round 2 F14 (swatch border): `UI/🧱️elements/🖱️ContextMenu/🎨️item-render/🟦️.tsx:49` uses an inset box-shadow.
- Round 2 F15 (avatar overlap): `UI/🧱️elements/🕰️HistoryTable/🟦️.tsx:135` masks with `ring-2 ring-background` and `border-0`. The overlap is still visible as a ring, so this is partly fixed.
- Round 2 F17 (scroll-area inset): `UI/🎨️styling/🖌️ui/🎨️.css:7386-7389` sets `padding: 0`.
- Round 2 F19 (skeleton gaps): `UI/🧱️elements/🦴️Skeletons/🟦️.tsx:28` uses `SharedBorderRowCell`.
- No `divide-*` utilities are used in any TSX under `🧰️framework`. The only `divide` references are in the class-name tables and dead CSS at `🎨️.css` ~6702-6707.
- No `-ml-px`, `-mt-px`, `first:border-*` or `last:border-*` hacks exist in `🧰️framework`, apart from F1.

## Not verified or out of reach

- I did not run `border-gap-probe.js` or the browser. Probe-level violations are inferred from class and CSS structure.
- `EngagementControlView` and the Tree/Engagement children were not traced to confirm which of them are framed (F2).
- Products other than course and the OS renderer (presentation, pets, print, 3d, infinite, repo library) were only grep-sampled. The grep returned no border hits outside `tests`.
- About 11,700 TS/TSX/CSS files sit under `🧰️framework`, so a fully exhaustive read of every file was not feasible. This list covers every border hit in `UI/` and the styling CSS, and the product hits that grep surfaced.