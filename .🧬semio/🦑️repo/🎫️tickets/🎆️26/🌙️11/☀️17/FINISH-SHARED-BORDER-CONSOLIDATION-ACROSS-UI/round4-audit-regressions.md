# 🔍 Round 4 Audit: Shared Border Regressions (static, read-only)

Scope: TSX and CSS files under `/Users/ueli/Documents/semio` modified in the last 30 hours (`find -mmin -1800`, excluding node_modules, .cache, .🧬semio, dist, target). 307 files. No source file was edited and no modifying git command was run.

Paths below are relative to `/Users/ueli/Documents/semio/`. Verdicts: **HIGH** = clear violation of the radical rule, **MED** = likely violation or duplicated mechanism, **LOW** = hack or smell, probably fine after review.

## Clean checks (no hits in production code)

- Negative-margin hacks (`-ml-px`, `-mt-px`, `-mx-*` on bordered things): none. The only `-m*` hits are a sr-only `margin: -1` (`🎓️teaching/.../🧍️figure/🟦️.tsx:62`) and a menu separator (`🔽️Select/🟦️.tsx:828`), both false positives.
- `border-l-0`, `border-t-0`, `border-[xytblr]-0`, `first:`/`last:border-*` pseudo overrides: none outside tests, which only assert their absence.
- `[DEBUG]` leftovers in scanned files: none.
- `CardGrid` / `CardStack` (`🃏️CardStack/🟦️.tsx:99-150`): no gap, no padding, no border classes. Compliant.
- `OverviewCard` (`🃏️OverviewCard/🟦️.tsx`): gaps are inside the title and chip rows only. Compliant.
- Gap and space hits that touch border or card classes, when they sit inside a `SharedBorderRowCell` (for example `🪟️chrome/🟦️.tsx:33,202,284,296,313`, `🏠️home/🟦️.tsx:389`, `🎮️Engagement/🟦️.tsx:263,303`, `🏘️SpaceBrowser/🟦️.tsx:314`): internal content spacing, which the brief allows.
- `StagedArgs` chip row (`📝️staged-args/🟦️.tsx:58-62`): uses `SharedBorderRow` with `data-shared-border-cell` chips. Compliant.
- `TimeTravelWindowIndicator` (`⏪️time-travel/🟦️.tsx:401`): a shared cell with no raw border. Compliant.
- Input, Textarea and Select raw `border` classes (`✏️Input/🟦️.tsx:297,512`, `🔤️Textarea/🟦️.tsx:121`, `🔽️Select/🟦️.tsx:430`): overridden by the frame and field CSS (`🎨️.css:6733-6783`). Accepted.

## Findings, ranked by confidence

### 1. HIGH: raw seam borders on a shared cell (navbar overview toggle)

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟢️overview-button/🟦️.tsx:13,17`
  - Snippet: `data-shared-border-cell=""` together with `className="ui-glass absolute right-0 top-0 ... border-b border-l border-border-normal ..."`
  - Why: a cell's only divider should be the mechanism's `border-inline-end`. Raw `border-b` and `border-l` paint their own seams on top of the overlay's and neighbours' lines. The button sits in the top-right corner, so its bottom line sits against the glass or the first card. This is exactly the "toggle drawing its own bottom border" case the brief names.
  - Root fix: drop `border-b border-l border-border-normal` and let the cell and compositor own the seams. If the top-right corner must be a frame edge, express it with `SharedBorderRow edges={["bottom","left"]}` or `data-shared-border-edges` on the overlay bar, not with raw borders on the child.

### 2. HIGH: floating trigger draws top and left borders onto the footer band

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:412`
  - Snippet: `className="fixed bottom-[var(--footer-band-height)] right-0 z-50 border-l border-t bg-background/80"`
  - Why: the element is pinned to the top edge of the footer band, so its `border-t` lands on the footer's own top line. Its `border-l` is a seam against the footer's right edge area. That gives two lines where there should be one, and a bordered element sits inset in the footer band.
  - Root fix: mount this trigger as a cell of the footer `SharedBorderRow` (or the footer's frame) so the compositor owns the top line, and remove the raw `border-l border-t`. If it must float, mark its attached side with `data-shared-border-attached="top left"` so the other owner paints the line.

### 3. HIGH: dashed bordered guess input

- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🎚️challenge-keys/🎨️.css:100-103`
  - Snippet: `.course-guess > input:read-only { border-style: dashed; background: transparent; }`
  - Why: the brief forbids dashed drop zones inside bordered sections. This is a dashed frame inside a bordered guess row. Its border style also breaks the solid shared line that the compositor paints for the row.
  - Root fix: keep the solid shared border and show the "no more answers" state through a `data-state` fill or text colour. Make the input a `data-shared-border-field` of the row so the mechanism owns its line.

### 4. MED: hand-rolled vertical divider overrides duplicate the mechanism

- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🧩️task-layouts/🎨️.css:52-59`
  - `.course-steps[data-shared-border-group][data-orientation="vertical"] > [data-shared-border-cell] { border-block-end-width: 0; border-inline-end: var(--stroke-hairline) solid var(--border-normal-color); }`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🧩️task-layouts/🎨️.css:140-146`
  - `.course-pool[data-shared-border-cell] { border-block-end-width: 0 }` and `.course-categories[data-shared-border-cell] { border-inline-start: var(--stroke-hairline) solid ... }`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🏁️results-and-preferences/🎨️.css:36-39`
  - `.course-result[data-crowd][data-shared-border-group][data-orientation="vertical"] > [data-shared-border-cell]:first-child { border-block-end-width: 0; border-inline-end: ... }`
- Why: these re-declare the cell divider by hand and remove the block-end line that the vertical group rules (`🎨️.css:6806-6814`) already manage. Explicit `border-inline-start` on a cell adds a second seam next to the neighbour's `border-inline-end`. Any change to the mechanism now has to be mirrored here.
- Root fix: delete the overrides. Let `data-shared-border-group data-orientation="vertical"` plus cell attributes produce the dividers. If the pool and categories need different edges, pass `edges` through `SharedBorderRow` instead of CSS per product.

### 5. MED: gap between two columns of a shared-frame row

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🕹️control-tree/🟦️.tsx:175`
  - Snippet: `<div ... data-shared-border-frame="" data-shared-border-edges="bottom" className={cn("grid min-w-0 w-full items-center gap-x-tiny min-h-small", className)} ...>`
  - Why: `gap-x-tiny` separates the `left` and `right` slots (`:176-181`). If the `right` slot holds bordered controls, such as toggles or buttons, the gap splits bordered siblings, which the rule forbids. The mechanism only handles this if each slot is a cell.
  - Root fix: make `control-tree-row-left` and `control-tree-row-right` `data-shared-border-cell` elements with padding inside, and set the grid to no gap. Confirm first what `right` renders, because a text-only `right` would make this a false positive.

### 6. MED: raw `border-t` where the edge mechanism is available

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📝️staging/🟦️.tsx:13,18`
  - Snippet: `<SharedBorderRow framed={false} className="w-full border-t">`
  - Why: the row's top seam is a raw class and not `edges={["top"]}`. It also sits inside a `<dialog>` that has its own `border`, so the seam is hand-placed against the dialog's edges. It bypasses the compositor's coincident-edge rules.
  - Root fix: `<SharedBorderRow edges={["top"]} className="w-full">` and remove `border-t`. Let the dialog frame own its outer edges with `data-shared-border-frame` and `data-shared-border-edges`, not a raw `border`.

### 7. MED: dropped border on a frame's last row by selector

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:7454-7455`
  - Snippet: `:is([data-shared-border-frame], [data-slot="popover-content"], [data-slot="dialog-content"]) table > tbody > tr:last-child { border-bottom-width: 0; }`
  - Why: this removes a table row's bottom line by position. It is an ad hoc cure for a seam that the frame should already own. It is also not limited to the frame's own bottom edge, so it could drop a line that is a real seam when the table sits in a frame with edges beyond the bottom.
  - Root fix: remove the rule and let the frame's `data-shared-border-edges` decide whether the table's last row owns the bottom line, or mark the table `data-shared-border-cell` and let the compositor resolve it.

### 8. LOW: raw `border` on a frame that already has all four edges

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:641`
  - Snippet: `data-shared-border-frame="" data-shared-border-edges="top right bottom left" ... className={`not-prose relative flex flex-col border bg-card ${className}`}`
  - Why: the frame paints all four edges through the compositor, and the raw `border` is a second outer line. Probably harmless, because the frame's own edges come out the same. It is still a double source for the same line.
  - Root fix: remove `border` from the class list and rely on `data-shared-border-edges`. Keep `bg-card`.

### 9. LOW: negative overlap and border removal on avatars

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🟦️.tsx:135`
  - Snippet: `<div className="flex shrink-0 -space-x-2">` with `className="border-0 ring-2 ring-background"` on each `TableAvatar`
  - Why: intentional avatar stacking, not a bordered-element seam. It is still a negative margin in disguise plus a border removal. Low risk because avatars are not bordered cells.
  - Root fix: if the avatar stack counts as a shared group, make it a `SharedBorderRow` of avatar cells. Otherwise leave it and note it as content art.
- Related: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧩️BlockListHost/🟦️.tsx:179` puts `border-0` on a `data-shared-border-cell`. That is a border-removal override on a cell. Low. Remove `border-0` and let the cell mechanism own the seam.
- Related: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/💡️hints/🟦️.tsx:69` uses `border-0` on an absolute glass chip. Low. Pick the intended border in the chip's own style or drop the class.

### 10. LOW, documented: negative margins in the interlock and silhouette mechanisms

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:7565-7567`: `margin-block-start: calc(var(--card-stack-lift, 0px) * -1)` on `[data-card-stack] > [data-overview-card]` and `[data-card-grid] > [data-card-grid-cell]`. This is the documented card-interlock lift from `useCardInterlock`. Accepted as the designed mechanism, not a regression.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:7485-7487`: `.window-silhouette-content-plane` negative block margins that cancel its own padding. Documented, accepted.
- `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:9`: negative margin to centre a corner handle. Not a bordered element (`border: 0`). Accepted.
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🗃️record-tables/🎨️.css:89`: `margin-block` negative on a zero-height `::before` spacer. No border involved. Accepted.

### 11. LOW: dashed frames in art and overlays (exempt candidates)

- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧪️playground/🖼️stage/🟦️.tsx:131`: `absolute border-t border-dashed` guide line in the planar stage. Content art (diagram). Exempt if the stage root carries `data-border-probe-art`. Verify that it does.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:1808-1815`: `.introduction-demo-zoom-ring` with `border: ... dashed var(--accent)`. Demo overlay, pointer-events none. Exempt as art.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx:290-311`: `PanelEmptyDockZone` docstring says "Dashed drop-zone placeholder", but the element is a `data-shared-border-cell` with no dashed class in the TSX. Low. The doc is stale or the dash comes from `dropZoneReadyFillClass`. Check that helper before any change. If the placeholder really is dashed, it violates rule 3 and must become solid.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/📖️stories/🧪️.story.tsx:62-75`: `space-y-4` and `gap-1 rounded-sm border p-2` buttons. Content gallery, and the root is marked `data-border-probe-art`. Exempt.

### 12. LOW: test fixtures that still use the old pattern

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧶️control-chrome-3-and-more/🟦️.tsx:312-315`: a `border h-medium` wrapper around three `Toggle`s, with `border-0 border-l` on the second and third. Gap-free, but it draws its own seams. Test fixture only. Migrate it to `SharedBorderRow` when the fixture is next touched.
- `🧰️framework/🛍️products/📚️course/🧪️tests/🧱️shared-borders/🟦️.tsx:43` and related test assertions that forbid `gap-double`: these guard the rule and are correct.

## Not findings (reviewed and skipped)

- Text-only gaps inside frames: `⚖️legal/🟦️.tsx:105-108` (footer links), `🛠️AgentChatPanel/🟦️.tsx:162` (title and presence), `🟦️ demonstrator-card.tsx:40` (text body). Per the brief, text-only gaps are skipped.
- `🧩️task/🟦️.tsx`, `🧩️BlockListHost` drag and grip code: the "drop" hits are event handlers, not drop zones.

## Coverage and limits

- Static scan only. The browser probe (`border-gap-probe.js`) was not run, so no DOM-level distances were measured.
- Gap and space hits were filtered to lines that mention border, card, button, chip or tile classes, plus the card components. Remaining `p-*` and `m-*` container cases were not exhaustively reviewed.
- Components referenced from other files (Input, Select, Dialog, Popover) were judged by their own class lists and the shared CSS overrides, not by each call site.
