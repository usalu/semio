# Cell Overflow: Cells Fill Their Owner

## Cause (defect 1)
- `chromeControlItemBaseClass` carried `min-h-medium h-full max-h-full`. In a bordered `h-medium` frame the content box is 2 px shorter than `min-h-medium`, so the min height beat `h-full`/`max-h-full` and the cell ended 1 px below the owner's border box (25.8-48.19 vs 24.8-47.19), open at the bottom and clipped by `overflow: hidden` rows.

## Fix
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎛️chrome-control-presentation/🟦️.ts:25`: base class is `self-stretch`, no height utilities. Standalone controls keep their own height from their owner/props.
- `:32` `chromeControlTabItemClass` states `min-h-medium` itself (tabs live in tab strips, not in a bordered frame).
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️.tsx`: dropped the `min-h-0` cancel on items; vertical and wrapped groups (`h-auto`) give their direct cells `[&>*]:min-h-medium` (the owner states the height); the item action cell uses `self-stretch` instead of `h-full`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️.tsx:40`: item variants lose `min-h-0 h-full`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx`: item loses `min-h-0`.
- Toggle (`🔀️Toggle`) and ToggleGroup shells already used `items-stretch`; they inherit the base fix.
- Audited, no change needed: Select/Input/Textarea (own bordered, border-box `h-medium` fields, not cells of an owner), SharedBorderRow cells (no height), Ribbon (borderless zone, items `h-full` of a definite `h-medium`), Stepper (fixed earlier). Window controls (`🗂️WindowChrome/🟦️.tsx:571,583`) keep `h-medium`: their chip has no CSS border and takes its height from them, so there is no border box to overshoot.

## Tests
- New `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎛️chrome-control-presentation/cell-contract.test.ts` (3): cell has `self-stretch` and no `h-/min-h-/max-h-` token; owner shell has `items-stretch` and no height; mocked geometry (border 1, medium 24) cell bottom equals owner border-box bottom minus border.
- Updated: ToggleGroup component test, shell-components-6 (ToggleGroup and Engagement class assertions), ActionGroup spacing vector json (itemRequired/itemForbidden; the `bun:test` file itself cannot run under vitest).
- Full `vitest-ui.config.ts` run: 1643 passed, 6 failed, all needing Chromium (Tree text-flow x3, Layout component x1, prepared-readback, text-advances). No other regressions. No product (os/course) consumer imports these classes.

## Defect 2: Ribbon/WithUndoRedo
- Undo/redo are `ButtonGroupItem`s in a `ButtonGroup` (bordered `h-medium` frame), items had `min-h-medium` from the base class overridden by `min-h-0` but `h-full` remained; the remaining structural suspect is the same class (cell not tied to the owner's content box). Now stretched. Not verifiable without the browser; if still open, probe the button rect vs the group's rect (top/bottom must equal the group's border-box top/bottom).

## Defect 3: Table rows
- Row boxes are `tr` with an absolutely positioned SVG child inside `Scrollable` (clipping) with a `w-full` table: row left/right equal the clip edges. Geometry in the compositor puts the stroke at inset 1 (covers 0.5-1.5 px), so a pure logical clip should not cut it. Not reproducible without live data.
- Compositor need (for the `shared-border.ts` owner): check live that the `tr` overlay's `getBoundingClientRect` equals the `tr` rect (an SVG in a `tr` of a `border-collapse` table may be placed by the table, not the row, so the vertical sides land on the scroller clip and the snapped line is cut by half a pixel), and that `snap` does not move the vertical lines onto device pixel 0 / width. If so, vertical lines of a box flush with a real scroller must be inset by the full line width (paint centre at 1 px, snapped to centre +0.5), as is done for the other sides.

## Defects 4 and 5: Layout/Default and Layout/Mobile
- Fullscreen toggle: `Toggle` frame is `h-medium` inside a `h-full items-stretch` trailing slot; the button cannot be taller than the frame by CSS (previously bounded by `max-h-full`, now by `self-stretch`). The 53-512 span is therefore the right edge of the box resolved onto the outer owner's edge (flush-ancestor walk: right side flush with navbar, shell and root ancestors moves onto their right edge; the shell root and the panel frames are `overflow-hidden`), not the button's own height. `button#properties` is a panel tab in a scrolling group (overflow-x auto) at the right panel edge: same shape.
- Mobile `button#explorer`: tabs are cells of a group that scrolls (`panelTabBarScrollRowClass`), so they are painted by the next owner out, the tab frame (edge bottom only, `h-large`), which lies in the `overflow-hidden` mobile panel frame. All four sides open with four ancestor segments stated means the cell's own paths were resolved away/clipped and four ancestor sources carry the state instead: the cell lies flush on the group, the frame and the panel on several sides, and the flush-ancestor walk plus the clip widening make every side coincide with an ancestor line whose owner overlay is not painted over the scroller (the scroller's `z-40` row vs overlay `z-40` at rest; stated overlay is 41).
- Compositor need: a cell flush with a scrolling group must be painted by the frame's overlay with its own four sides in the stated state (not merged into ancestor segments that are not stated), and flush-walk merging must not take a side onto an ancestor edge whose owner does not draw it. Send `silhouette.sweep()` output (owner, box ids, painted paths) for `#explorer`, `#properties` and `#ui.fullscreen.toggle` to confirm; I could not reproduce these three without the browser.
