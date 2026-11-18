# Round 2 Static Audit: Radical Border Rule (UI elements and shared stylesheet)

Scope: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/*/🟦️.tsx` (test files excluded) and `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`. Read-only: no source file was edited. Method: class-token greps for gap/space/margin utilities, padding utilities within 4 lines of a border token, dashed/dotted grep, CSS selector scan of every `gap`/`margin`/`padding` declaration near shared-border and slot selectors, and line-level reads of each hit. Excluded per brief (OverviewCard, WindowChrome, LayeredOverview, SharedBorderRow); briefly listed at the end.

Notes on the probe: `border-gap-probe.js` ignores transparent borders. Ghost buttons (`border-transparent`) therefore do not count as bordered visually. Several findings below depend on whether a child is visibly framed, and that is stated per finding.

Path prefix used below: `🧰️framework/🔨️modules/🖱️ui/` is shortened to `UI/`.

## Counts

| Category | Findings |
|---|---|
| (1) gap / margin between bordered siblings | 7 (F2, F3, F4, F7, F8, F9, F12) |
| (2) bordered container with padding over a bordered child (box-in-box) | 4 (F1, F5, F6, F13) plus 1 double-arc (F15) |
| (3) dashed or dotted drop zone inset in a parent frame | 0 (1 dashed zone checked, not inset) |
| (4) popover, menu, dialog or tooltip content with inner bordered block | 3 (F5, F6, F14) |
| (5) CSS margin or gap between framed siblings | 1 (F12) |
| Low-confidence or cosmetic | 5 (F8, F14, F17, F19, F21) |

Total findings: 16 (F1-F9, F12-F15, F17, F19, F21). Confidence: HIGH 5 (F1, F3, F4, F5, F12), MEDIUM 6 (F2, F6, F7, F9, F13, F15), LOW 5 (F8, F14, F17, F19, F21).

## Findings

### F1. Canvas app-mode nav frame pads its framed Select trigger (box-in-box plus double bottom edge)
- Element: `UI/🧱️elements/🎨️Canvas/🟦️.tsx:200`
- Classes: `data-shared-border-frame data-shared-border-edges="bottom"`, `gap-[var(--padding-standard)] p-[var(--padding-standard)]`, `borderNormalBottomClass`
- Child: `SelectTrigger` at `UI/🧱️elements/🔽️Select/🟦️.tsx:430` carries `border ... borderElementClass`, so it is a visible framed control.
- Why: the frame pads its only child by `--padding-standard` on all sides, so the trigger sits inset from the frame's left, right and bottom edges. Its bottom border sits about one padding unit above the frame's own bottom hairline: two bottom lines, plus a gap. This is the navbar-toggle failure mode from the brief, in a non-navbar band.
- Fix: the band should not pad. Use `p-0` and `data-shared-border-cells` on the frame so the trigger becomes a cell: `align-self: stretch`, full band height, no own frame, the frame's bottom edge is the trigger's bottom edge. Move the horizontal text padding into the trigger (`px-single` on the trigger's content, already inside the cell). Drop the `gap` (single child, no effect but misleading).
- Confidence: HIGH.

### F2. Window controls: gap between a custom controls node and a bordered ActionGroup; controls float inset from the window edge
- Element: `UI/🧱️elements/🪟️Window/🟦️.tsx:265` and `:306`
- Classes: `:265 flex items-stretch gap-[var(--gap-standard)]` wraps `{controls}` and `<ActionGroup>` (`:268`, its items are bordered control groups). `:306 absolute top-1 right-1 z-panel flex items-stretch gap-[var(--gap-standard)]`.
- Why: (a) gap between two framed things (`controls` and the ActionGroup) when both are visible; (b) `top-1 right-1` (4px) inset places the group inside the window's own area instead of on its edge, which is a box-in-box over the window's body. The window has no permanent frame (`loadingBorderStateClass` / `waitingBorderStateClass` only at `:302`), so the inset is visible only where the window is rendered outside WindowChrome.
- Fix: replace `gap-[var(--gap-standard)]` with `data-shared-border-group` on the wrapper (groups without gap, one divider per cell). Anchor the controls on the frame edge (`top-0 right-0`, `data-shared-border-cells`) so they share the frame edge rather than floating 4px in. If Window is always inside WindowChrome, this overlay should be removed: WindowChrome already owns window controls.
- Confidence: MEDIUM (gap case HIGH; inset depends on whether Window renders standalone).

### F3. Presence roster: gap between bordered avatars
- Element: `UI/🧱️elements/👥️PresenceBar/🟦️.tsx:85`, `:92` (`flex items-center gap-single`)
- Child: each peer is a `TableAvatar` (`UI/🧱️elements/📻️TableAvatar/🟦️.tsx`, class list around lines 55-68: `!isFullSize && "border"`) with a 2px colored ring (`:109`, `borderWidth: 2`). Status badges at `:114` and `:124` are also `border` circles.
- Why: adjacent avatars are separated by `gap-single`, so every seam is two bordered circles with a gap. This is a direct violation of the rule (gap between bordered siblings).
- Fix: the roster becomes one shared frame: `data-shared-border-group` + `data-shared-border-cells` on the container, peers as cells (`rounded-none`, no own border, divider from the cell rule), status badge as a cell overlay on the cell edge. If circular avatars are a product requirement, the roster cannot satisfy the radical rule as written: raise it to the owner rather than keep the gap. The probe will flag the current layout.
- Confidence: HIGH.

### F4. UI dialog argument stack: gap and margin between bordered fields; margin between fields and footer buttons
- Element: `UI/🧱️elements/📨️UIDialog/🟦️.tsx:120`, `:121`, `:124`, `:126`, `:133`, `:135`
- Classes: `:120 mb-single` (title), `:121 mb-double` (description), `:124 mb-double flex flex-col gap-single` (argument block), `:126 flex flex-col gap-tiny` (per argument, wraps a field), `:133 flex flex-wrap items-start justify-between gap-single` (footer row), `:135 data-shared-border-group`.
- Why: argument fields (bordered inputs or selects) are stacked with `gap-single`, and the block is separated from the footer's bordered choice buttons by `mb-double`. Both are gaps between bordered elements. The footer row also uses `gap-single` between the ghost Cancel (no visible frame, so acceptable) and the choice group (bordered).
- Fix: argument rows become a vertical shared-border group (`data-shared-border-group data-orientation="vertical"` with fields as `data-shared-border-cells` / `data-shared-border-field` cells). Remove `gap-single` and `mb-double` from `:124`. Spacing between title, description and fields goes inside the text cells (padding of text). The footer attaches to the argument group so its top edge is the group's bottom edge.
- Confidence: HIGH.

### F5. Dialog content: padding and gap around framed children (header, body, footer)
- Element: `UI/🧱️elements/💬️Dialog/🟦️.tsx:546-552` (frame), `:596` (footer)
- Classes: frame `data-shared-border-frame data-shared-border-edges="top right bottom left"` plus `border ... gap-medium p-medium`; footer `data-shared-border-group` with `flex flex-col-reverse sm:flex-row`.
- Why: (a) `p-medium` insets every direct child of the framed dialog, including user body content that commonly contains bordered inputs or selects (box-in-box, rule 2); (b) `gap-medium` separates the body from the footer's bordered buttons (rule 1). The footer is a group, not a frame, so the padding inset affects its buttons directly.
- Fix: the frame gets `p-0` and `gap-0`. Padding moves into non-framed text wrappers (header, body text) inside the dialog. Framed body children become shared cells (`data-shared-border-cells` on the body wrapper). The footer is attached to the frame's bottom edge with no gap.
- Confidence: HIGH for the classes; MEDIUM that a live dialog hits it (depends on user body content).

### F6. Popover content: padding over framed children
- Element: `UI/🧱️elements/🗨️Popover/🟦️.tsx:423` (`data-shared-border-frame`), `:425` (`p-1 border`)
- Why: `p-1` (4px) insets every direct child of the framed popover. Any framed child (a select, input, button group, card) sits inset, so the popover's border and the child's border run parallel with a gap. Rule (2).
- Fix: frame `p-0`; wrap text and non-framed content in a padded inner div; framed children become `data-shared-border-cells` on the popover content (the rule already exists for `[data-slot="popover-content"]` in CSS at `🎨️.css` 7341 for table rows; extend the same selector idea to direct children).
- Confidence: MEDIUM (generic container; the violation is real whenever a framed child is placed in it).

### F7. Tree file card: margin between stacked framed cards; padding over the card's content
- Element: `UI/🧱️elements/🌳️Tree/🟦️.tsx:638` (`not-prose my-medium p-medium rounded-lg border bg-card`), `:639` (`mb-4` title), `:641` (`flex flex-col gap-single`)
- Why: `my-medium` separates consecutive `TreeFiles` cards (framed siblings, rule 1). `p-medium` insets the list. The `gap-single` at `:641` is between tree rows; rows themselves are not framed, so that gap is acceptable.
- Fix: remove `my-medium` (cards stack with a shared edge; the next card's top edge is the previous card's bottom edge). Replace `p-medium` with padding on the inner title and list wrappers. Keep `gap-single` for rows (not framed).
- Confidence: MEDIUM (margin rule clear; whether TreeFiles cards are stacked in practice is not verified).

### F8. Tree control rows and window-measure rows: gaps between value-slot controls
- Element: `UI/🧱️elements/🌳️Tree/🍃️item/🟦️.tsx:287` (`tree-item-control` with `gap-double` and `value` slot), `UI/🧱️elements/🌳️Tree/🕹️control-tree/🟦️.tsx:223` and `:252` (`gap-double`), `UI/🧱️elements/🌳️Tree/🔢️window-measures/🟦️.tsx:32`, `:115`, `:187` (`gap-double`)
- Why: the value slot may hold two framed controls (for example a Select next to a Slider or a number stepper) separated by `gap-double`. Label-to-control gaps are fine (label not framed). Only the case where two framed controls are siblings is a violation, and the slot is generic, so it cannot be verified statically.
- Fix: where the value slot contains more than one framed control, render it as `data-shared-border-group` with cells instead of gap-separated items. Otherwise leave as is.
- Confidence: LOW.

### F9. Tabs: gap between list and content, and an inset active tab inside a padded list
- Element: `UI/🧱️elements/📑️Tabs/🟦️.tsx:123` (`flex flex-col gap-single`), `:157` (`TabsList`: `p-single`, `data-shared-border-group`), `:212` (`TabsTrigger`: `border border-transparent ... h-[calc(100%-var(--stroke-hairline))]`)
- Why: the list pads by `p-single`, so the active trigger's visible border (`border-active-base` when active) is inset from the list's edges. The `h-[calc(100%-stroke)]` offset is a seam workaround that proves the inset. The root `gap-single` separates the list from the content panel (`TabsContent` `:247` is `flex-1`, unframed by default, but framed content is common).
- Fix: root `gap-0`. `TabsList` `p-0`, triggers as `data-shared-border-cells` (stretch to full list height, no `h-calc` workaround, dividers from the cell rule). The list's bottom edge is the content frame's top edge.
- Confidence: MEDIUM.

### F10. (Checked, no finding) Field: gap between label and control
- Element: `UI/🧱️elements/📝️Field/🟦️.tsx:35`, `:36`
- The label row is not framed; the gap separates text from a bordered control. Not a rule violation. Note only.

### F11. (Checked, no finding) Dashed drop zone
- Element: `UI/🧱️elements/🖼️Panel/🟦️.tsx:310` (`border border-dashed`, `PanelEmptyDockZone`)
- The only dashed or dotted border in the elements. It is rendered by `PanelGhostRoot` (`:467`, `absolute ... w-fit`, no border, no padding). Not inset in a framed parent, so not a violation of (3). Still a dashed bordered element: if the brief's "dashed drop zone" rule covers it, the fix is to draw the drop indication as the accent segment of the panel's own frame.

### F12. CSS: intro demo callout chips stacked with a gap
- File: `UI/🎨️styling/🖌️ui/🎨️.css:1736-1737` (`.introduction-demo-callout-chips`: `gap: calc(var(--ui-spacing) * 0.5)`, `padding-top`), `:1746-1747` (`.introduction-demo-callout-chip`: `border: var(--stroke-hairline) solid ...`, `display:none` by default)
- Consumer: `UI/🎯️targets/⚛️react/🎬️introduction-demo/🟦️.tsx` (outside `elements/`, but it is the only user of these classes and the stylesheet is in scope).
- Why: each chip is bordered and the list is separated by a gap: rule (1) and rule (5), CSS form. Shown when the chip is active, so the violation is live in the intro demo.
- Fix: `gap: 0`; the chips use the cell mechanism (`data-shared-border-group data-orientation="vertical"` on the list, chips as cells), or a negative margin of one hairline, but the first choice is the shared mechanism. Keep `padding-top` on the list container (padding outside the chips, not between them).
- Confidence: HIGH (for the gap); the intro demo is not in the probe's element list, so verify on that page.

### F13. Canvas mode-dock insert preview: margins and its own border inset inside the dock tab row
- Element: `UI/🧱️elements/🎨️Canvas/🧲️drag-preview/🟦️.tsx:9` (`modeDockTabInsertPreviewClass`: `mx-half my-half ... rounded-sm border-2 border-accent bg-accent/20`)
- Why: a bordered preview box with `mx-half my-half` sits inset inside the dock's tab row, which is itself a framed band. Transient (drag only), so a live probe may miss it, but it produces a box-in-box and a double line during a drag.
- Fix: no margins and no own border. Draw the insert indicator as an accent overlay on the target cell edge using the `shared-border-priority` accent segment (the mechanism already drives `:focus-within` and active seams).
- Confidence: MEDIUM.

### F14. Menu item colour swatch: bordered block inside a menu row
- Element: `UI/🧱️elements/🖱️ContextMenu/🎨️item-render/🟦️.tsx:49` (`size-small shrink-0 rounded-sm border border-border`)
- Why: a bordered swatch inside a menu row (rule 4). It is colour content, not chrome, so it could be argued as art. It is not marked as art anywhere.
- Fix: either remove its border (the fill itself identifies the colour, with an inset hairline drawn as `box-shadow: inset 0 0 0 hairline`, which is not a separate border box), or mark it `data-border-probe-art` deliberately. Confidence LOW.

### F15. Overlapping avatar stack: two bordered circles overlap (double arcs)
- Element: `UI/🧱️elements/🕰️HistoryTable/🟦️.tsx:135` (`flex shrink-0 -space-x-2` over `TableAvatar`, whose `!isFullSize && "border"` applies)
- Why: overlap by 8px means each avatar's border crosses its neighbour's border, producing two arcs where the rule demands one line. Negative spacing is not a gap, but it is also not one shared line.
- Fix: mask the overlapped edge with a surface-coloured ring (`ring-2 ring-[surface]`, plus `border-0` on all but the front avatar), or render the stack as one framed cell group with avatars as cells. Confidence MEDIUM.

### F17. (Low) Scroll areas inside panels and panes padded by one hairline
- File: `UI/🎨️styling/🖌️ui/🎨️.css:7335-7339` (`[data-slot="panel"] [data-slot="scroll-area"], [data-slot="pane"] [data-slot="scroll-area"] { padding: var(--stroke-hairline) }`)
- Why: inset of one hairline inside a framed panel or pane. The comment states the purpose (keep scrollbars inside the frame). A bordered child at the scroll edge would produce a second hairline beside the frame. Currently no bordered child is known to sit at the edge.
- Fix: keep the inset for scrollbars only (`scrollbar-gutter` or a scrollbar-only inset), not on the content box. Confidence LOW.

### F19. (Low) Skeleton placeholders stacked with gaps
- Element: `UI/🧱️elements/🦴️Skeletons/🟦️.tsx:72`, `:87`, `:95` (`gap-single`, `gap-double`)
- Placeholders are unframed `SkeletonBlock`s standing in for bordered controls (`h-medium` blocks at `:89`, `:97`, `:98`). No visible violation, but the skeleton shape disagrees with the final layout once controls become framed and cells. Align the skeleton shape with the cell layout of the real control. Confidence LOW.

### F21. (Low) Toggle group action slot margin
- Element: `UI/🧱️elements/🎛️ToggleGroup/🟦️.tsx:246` (`ms-single` on `toggle-group-item-action`, `aspect-square h-full`)
- The action slot is not framed (`surfaceClass`, background only), so `ms-single` is text spacing inside a framed item. Not a violation. Confidence LOW.

## Checked, no finding

- `UI/🧱️elements/🪜️Stepper/🟦️.tsx` (frame at `:251-257`, cells at `:262` and `:275`): cells are the shared mechanism. OK.
- `UI/🧱️elements/🔽️Select/🟦️.tsx:430`: `gap-single px-tiny py-single border` on the trigger is internal content spacing of a single framed control. OK.
- `UI/🧱️elements/🔳️ButtonGroup/🟦️.tsx:137`, `🔀️Toggle/🟦️.tsx:193,336`, `⚡️ActionGroup/🟦️.tsx:107,176,234`, `🎛️ToggleGroup/🟦️.tsx:220`: gaps occur only when `inlineText` is set and separate content inside one item (icon and label). OK.
- `UI/🧱️elements/⌨️Command/🟦️.tsx:286` (`gap-single px-tiny borderNormalBottomClass`): the wrapper's bottom line is its own edge. Child input not checked deeper. OK at class level.
- `UI/🧱️elements/📊️Table/🟦️.tsx:222`, `:284`, `:442`, `:551`, `:561`: `gap-1` and `p-single` are within cells. OK.
- `UI/🧱️elements/🎬️Scene/🟦️.tsx`, `🕸️Diagram/🟦️.tsx`: no gap or margin hits.
- `UI/🧱️elements/🔚️Footer/🟦️.tsx`, `🔝️Navbar/🟦️.tsx`: no gap or margin hits on framed children at class level.
- `UI/🧱️elements/🎀️Ribbon/🟦️.tsx:42,83,91,93`: `data-shared-border-group` rows, `my-auto` divider, no gaps. OK.
- `UI/🧱️elements/🔽️Select/🟦️.tsx:828` (`-mx-single my-single h-px`) and `🖱️ContextMenu/🔗️fusion/🟦️.tsx:195` (`my-single h-px`): decorative separator lines, unframed. OK.
- `UI/🧱️elements/🎨️Canvas/🟦️.tsx:56`, `:63` (`gap-0`): OK.
- CSS `.window-silhouette-content-plane` (`🎨️.css:7373-7376`, negative margins plus compensating padding): intentional compensation for silhouette clearance, not framed siblings. OK.
- CSS `[data-celebrated="true"]::after` (`🎨️.css:1362`, `padding: var(--celebrate-border-padding)`): overlay ring on one element, not a sibling. OK.
- CSS `[data-slot="footer"]` and `[data-slot="navbar"]` edge rules (`🎨️.css:6555-6580`): hairlines drawn once on the band edge (pseudo-elements), no inset. OK.
- CSS `[data-shared-border-cells] > [data-shared-border-frame]` family (`🎨️.css:6671-6700`): cells mechanism, no margins. OK.

## Art exemptions

No element currently carries `data-border-probe-art` (grep of `UI/` returns 0 hits). Candidates for the marker, to be added deliberately and listed here when applied:
- `UI/🧱️elements/🎬️Scene/` (3D viewport root)
- `UI/🧱️elements/🕸️Diagram/` (graph canvas root)
- The puzzle canvas referenced by `data-testid="puzzle2d-canvas"` (`🎨️.css:7744`) is content art.

## Excluded elements (listed briefly, not audited in depth)

- `UI/🧱️elements/🃏️OverviewCard/🟦️.tsx:78`, `:130`, `:137`: title and chip rows with `gap-single px-single` inside chip content. Chips are cells inside the card; the gap is inside a chip. Striking only if chips are stacked with gaps.
- `UI/🧱️elements/🗂️WindowChrome/🟦️.tsx:539`, `:551`: enlarge and close buttons with `gap-single` (icon and text inside one control). `:586`, `:598`: footer gap cells, no gap class.
- `UI/🧱️elements/🥞️LayeredOverview/🟢️overview-button/🟦️.tsx:16`: floating button at `absolute right-double top-double` with its own `border`: sits inset from the overlay edge (box-in-box candidate).
- `UI/🧱️elements/🥞️LayeredOverview/💡️hints/🟦️.tsx:67`: hint chip `gap-[4px] border`, absolute overlay.
- `UI/🧱️elements/🪢️SharedBorderRow/🟦️.tsx:70`, `:82`, `:100`: `px-single py-1` inside cells and `gap-single` inside a field cell. Padding of cell content is allowed by the brief.

## Summary of fix mechanism to reuse

1. Gap between framed siblings: replace with `data-shared-border-group` (and `data-orientation="vertical"` for stacks) or `data-shared-border-cells`; cells draw their own dividers via `[data-shared-border-cell]`.
2. Padding over framed children: move padding into non-framed text wrappers; framed children become cells stretched to the frame.
3. Margins between framed siblings: remove; the next card's top edge is the previous card's bottom edge.
4. Overlays (drag previews, overview buttons, window controls): anchor on the frame edge, not inset, and reuse the segment priority layer for the accent state.
