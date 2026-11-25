# Visual defects handoff: frame and chip design

Defects that markup and jsdom tests did not see, found by looking at the built Storybook in Chromium. Paths relative to `🧰️framework/🔨️modules/🖱️ui` unless stated.

## Root causes and fixes

| # | Defect | Root cause | Fix |
| --- | --- | --- | --- |
| 1 | Tabs: chip covers the first row, no inner edges, no dividers | `--tabs-list-clearance` was never published: the list ref ran before the root ref existed (children attach first), so `measureList` returned early. The list was a plain group, so the compositor drew no chip edges. | `📑️Tabs`: the clearance finds its root with `closest('[data-slot="tabs"]')`. The list is a cells chip (`data-shared-border-frame`, `data-shared-border-cells`, edges `right bottom`, `left bottom` in RTL); every tab is a cell (`sharedBorderlessBoxProps`), the selected one carries the active state, so it closes on all four sides and the dividers are the cells' lines. |
| 2 | Layout collapsed onto its bands | The canvas is an absolute bleed layer, so an auto-height host gave the layout no height; the story's top panels also had no active tab, so their body was empty. | Layout guarantees a floor (`minBlockSize: 24rem`, `LAYOUT_MIN_HEIGHT`); the story fills the viewport and names `activeTabPath` for the console and properties panels. |
| 3 | Select: duplicated scroll arrows, no row lines | The story put `SelectScrollUpButton` and `SelectScrollDownButton` inside `SelectContent`, which already renders both. Rows inside `SelectGroup` were not children of the cells viewport, so no cell lines. | The two buttons are internal now (exports, story and test removed). Options and labels carry `data-shared-border-cell`, so the popup frame owns one line per seam; the separator has no height; rows keep `min-h-(--chip-row-height)!` (the cells rule resets `min-height`). |
| 4 | Context menu: browser focus ring, rows narrower than the frame, no lines | `role="menu"` with `tabIndex=-1` got the default `:focus-visible` ring; the body had `w-fit p-single`; the menu was not a cells owner; a native `button` ignores `width: auto`. | CSS: `[role=menu], [role=listbox]:focus-visible { outline: none }`; `button[data-shared-border-frame]` in a vertical cells owner is `width: 100%`. Menu and wing are `data-shared-border-cells` vertical, body `px-0` (the chip clearance stays), menu `w-full min-w-max`, rows `min-h-(--chip-row-height)!`, labelled separators are rows, unlabelled ones `h-0`, submenu rows are cells. |
| 5 | Active border incomplete (right and bottom pale) | A line shared between abutting frames is centred on the seam, so the owner's own clip (`overflow-clip`, `overflow-hidden`) cut half of it: right and bottom were 0.5 px and pale beside the full-width chip lines. | `BorderCompositor.insideOwner`: when the owner clips its own overflow on that axis, the line is held one frame inset inside the clip (`WINDOW_FRAME_PATH_INSET`), endpoints on the same normal. Non-clipping owners keep the seam centred, so the existing seam vectors stay. Left, top, right and bottom now cover about one pixel each. |
| 6 | Window content starts below the chip row, base strip above it | Two causes: `WindowChrome` merged `bodySurfaceClassName` (`windowBodyFrameClass` carries `relative`) after `absolute`, so the body surface was a zero-height in-flow element; and the dock plane painted the base level while the hosted `Window` paints the window level. | `WindowChrome`: `cn(surface, "pointer-events-none absolute inset-0 z-0 border-0")` (positioning last). `ModeDockStack`: `bodySurfaceLevel="window"`, so the plane under the chip rows has the colour of the content. Footer rows follow (same plane). |
| 7 | Canvas windows: no outline, no divider, bottom cut | A `Window` outside a chrome was not a frame; the story was `h-screen` inside the host padding. | `Window` registers `data-shared-border-frame` and all four edges unless it sits in a `WindowChrome` body (`WindowFrameHostContext`, which its frame already outlines). Seams between windows are held inside by fix 5. Stories use `calc(100dvh - 2rem)`. |
| 8 | Found while inspecting | `StatusChip` dots (`bg-success-border` and kin) painted nothing: only CSS variables existed, not theme colours. `window--nested-layout` showed one window: `Window` is `w-full`, so siblings in `HorizontalWindows` shrank to zero. | Theme colours `--color-{info,success,warning,destructive}-{bg,foreground,border}` in `🎨️styling/🖌️ui/🎨️.css`; `HorizontalWindows` and `VerticalWindows` give their children `flex-1`. |

## Tests

- Chromium (new): `🧪️tests/🖼️storybook-frame-visuals/{🟦️.ts,cases.json,cases.schema.json}`, registered in `.storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts`. Nine tests: tab chip clearance, bottom edge and dividers; layout between bands with all panels; one scroll band per end and a line on every row seam; menu rows span the frame, a line on every seam, no focus ring; active outline coverage on all four sides (screenshot pixels, red-minus-green share of the stroke); content surface under the cap row equals the content; window outline, seam and canvas inside the viewport; status dots filled; nested windows share the canvas. Against the build before the fixes: 9 failed; after: 9 passed.
- jsdom (new or changed): `🧱️elements/🗂️WindowChrome/frame-structure.test.tsx` (surface stays absolute, Window frame markers and host context, layout floor, menu body padding, select cells and one band each), `shared-border.test.ts` (seam inside a clipping owner), `state-boxes.test.tsx` and `📑️Tabs/🧪️tests/🧩️component/boundary.tsx` (tab cells, RTL edges, published clearance).
- Runner: `node frame-visuals-run.mjs <static dir>` in this ticket serves a static build in-process and runs the suite with the full Chromium binary (`STORYBOOK_CHROMIUM_PATH`): the legacy headless shell never finishes `domcontentloaded` on the Storybook iframe on this Mac. Screenshots: `story-shots.mjs` (variants through `SHOT_VARIANT=dark,rtl,mobile,forced,reduced-transparency,de`, `SHOT_CLIP=x,y,w,h` for a 4x crop), `static-probe.mjs`, `pixels.mjs`.

## Evidence

`evidence/final/*.png` (after), `evidence/base/*.png` and `evidence/stories/*.png` (before), `evidence/variants/*.png` (dark, RTL, 375 px, forced colours, reduced transparency). Looked at: tabs, mode (quad, tab stack), layout (default, mobile), select, contextmenu, canvas, window (default, controls, utility bar, nested), skeletons, dialog, popover, tree, presence, command, status chip, introduction, overview card, panel, table, providers. Unchanged and right: uiintroduction, overviewcard, panel, table, providers, layout--mobile.

## Open

- Tabs RTL cannot be flipped by the document direction in a story (the component takes `dir`); covered by the jsdom test only.
- Command palette rows have no seam lines (not a chip defect; not changed).
- `Window` outlines now exist for windows outside a chrome; hosts that relied on a bare window without a line have to opt out by sitting in a frame.
- The wgpu targets of Select and ContextMenu were not touched.

## Final run

`compositor-vitest.sh` (stock UI config, through the vitest lock): 83 files, 1712 tests passed. Two stale expectations of the dock stack body level (`base`) now read `window`, the level of the content the plane lies under (`📐️shell-components-4`, `🗂️shell-components-5`).
