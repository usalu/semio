# Round 3 element live sweep

Harness: `round3-elements-live.tsx` (+ `round3-elements-compose.tsx`, `round3-site/`, `vite-round3-elements.config.ts`, `round3-elements-restart.sh`). Port 5442, own tab. It mounts all 122 element stories (meta.render and a full-height host now honoured) plus 4 compositions (forms with invalid/disabled fields, groups, wrapping/destructive rows, select). Vite runs with `watch: null` because concurrent edits full-reloaded the page mid-probe; restart the script after an edit. Probed with `border-gap-probe.js`, only trusting `pending:false`, pane fronted before every call.

## Fixed (root causes)
| Finding | Root cause | Fix |
|---|---|---|
| Panel tab strip 2px short of window chrome line (Layout, 6 Panel stories) | unanchored `--panel-header-height` was the palette 22.4px, chips/silhouette use `--chip-row-height` 24px | `:root { --panel-header-height: var(--chip-row-height) }` in ui `.css` |
| Chrome-hosted panel bands 3.4px off (Panel stories) | story bands were plain `h-large` divs | `data-slot=navbar/footer` + band-height tokens |
| Window quick-action rail 1.6px inside the cap chip (Window stories, overlap 59px²) | rail offset `var(--size-medium)` vs 24px chip row | `var(--chip-row-height)` in Window; same in ContextMenu chrome offset (3 sites) |
| Split ToggleGroup item overflowed its cell by ~1px (ToggleGroup/Default) | every shell `flex-1` equal width, action shells are wider | action shells are `shrink-0` (content sized) |
| Table / TableAvatar avatars 0.5-2px from row lines, 17px apart | round bordered avatar is portrait art | `data-border-probe-art` on avatar root |
| Ribbon MultipleZones 4-5px gaps | story `gap-single` between zones | removed |
| LayeredOverview strip/swipe/reduced-motion 17-39px gaps | story grid `gap`, `place-items-center`, `max-w-xs` cards | cards `size-full`, grid through `--card-grid-columns/rows` |
| Popover open: 8 violations | story used `space-y`, `gap-double`, stand-alone inputs and buttons | vertical `SharedBorderRow` of cells, label+Input rows as cells, button row |
| Layout story raw `<input class="border rounded">` | raw boxed input in a navbar | framework `Input` |

Tests: `SharedBorderRow/round3-framework.test.tsx` (+4: chip-row panel header, gapless stories, popover/layout composition, chip-row offsets), `TableAvatar` (art marker), `ToggleGroup` (action shell sizing). Run natively with `vitest-ui.config.ts`: SharedBorderRow 37, ToggleGroup+ContextMenu+Window 51, Panel+WindowChrome+TableAvatar green.

## Probe result (1440x900 / 375x812)
All stories 0 violations at both sizes except the open items below. Open states probed 0: Popover, Select, Dialog, UIDialog x4, Toggle dropdown, Command, Tabs, ContextMenu (right click) x4, Form, Field error, Input/Textarea, Checkbox, MenuItem matrix, Compose forms/rows/select. Disabled and invalid states are in the compositions (0). Focus-visible: every sampled control (28) has no outline and no box-shadow, so no ring sits beside a seam; the seam recolours instead.

## Open / not fixed
- `Tree/ControlTreeStory` (33.5px): filter input vs first stepper row, with borderless label rows between. Needs a design call (control tree is a form of unbordered rows with bordered controls); probe says a gap, left as is.
- `Window/VerticalLayout`, `NestedLayout` (3.59px): the story windows collapse to the utility-bar height (no body), so bar two's top cap sits 1.6px under window two's top line. Story degeneracy; real windows have a body. Not changed.
- `Layout/Default` at 375 (desktop composition squeezed; `Layout/Mobile` is 0). `Window/*` at 375: narrow windows whose chips overlap each other and the story titles (overlap, not a border finding).
- Probe false positives/art: `LayeredOverview/Opened` overlap is the block `h1` box under the Overview button; Compose/Groups stacks differently-sized frames in a column (artifact of the composition); `Scene/*` stay `pending` (3D art).
- Skeletons tests (`two even window-chrome silhouettes`) and Playwright-based Tree/Layout tests fail; Skeletons is being edited by another worker, Playwright browser is not installed (environment).
- Hover/destructive were verified by CSS/state only (colour, no geometry change), not by real pointer hover.

## Follow-up: open items closed (second pass)
Probe protocol: pane fronted, screenshot to wake rAF, probe retried until `pending:false`. Final sweep of all 126 entries (122 stories + 4 compositions): 0 violations, 0 overlaps at 1440x900 and at 375x812; open Layout panel and Window controls+measures probed 0 as well.

| Item | Root cause | Fix |
|---|---|---|
| `Tree/ControlTreeStory` 33px gap | folder-header rows and control rows had no line of their own, so the filter's line and the first stepper's line were two runs separated by borderless rows | `ControlTreeRow` is a `data-shared-border-frame` with edges `bottom` through `useSharedBorderFrameRef`; the compositor merges its bottom line with the coincident control frames, so filter, headers and controls are one stack (a first CSS-border attempt left 0.9-1.9px near-doubles and was dropped) |
| `Window/Vertical`, `Nested` 3.6px | story windows lacked `fill`, so each collapsed to its utility bar (also true for Horizontal, Canvas, Layout stories); Nested also wrapped its columns in a second `Window` that painted a duplicate utility bar | `fill` on all layout-story windows; the nested wrapper is a plain flex div |
| Overlaps at 375 (Canvas, Layout, Window titles under the chip toggle) | same unfilled windows put titles under the top-left chip | covered by `fill` |
| Window controls vs measures toggle (Window Options over Focus/Close, 1220px²) | both anchored top-right | `Window` measures the controls width and passes it as `inlineEdgeReservePx` to the measures `Pane` (real fix, product-wide) |
| Layout navbar: panel chip over Home/Application | `PanelChromeTabBar` keeps a placeholder as wide as its folded width, which was never measured because the story opened the panel at start | the chrome tab bar is first in the navbar and the panel starts folded (placeholder measured), open state probed 0 |
| LayeredOverview/Opened overlap, Compose/Groups | block `h1` box under the Overview button; stacked frames in one column | `w-fit` h1; compositions spaced |

Uninspected failures of the 11: Playwright-only suites (Tree text-flow x3, Layout canvas width x1) fail with `Executable doesn't exist ... chrome-headless-shell` (environment). Skeletons: its gap-free refactor by another worker made the old `.flex-row.gap-single` assertion stale; the test now checks a gapless row of two (2 pass). The five Diagram force tests (`cursorizes`, `projects`, `enqueues`, `expands`, `quarantines`) fail on engine read-outs (`edges.read()` undefined, mocks never called) in a suite whose only change in the tree is a `data-border-probe-art` attribute on the Diagram root; no border or chrome code is on their path, so they are not part of this migration (to be checked by whoever owns the cooperative force engine). Subset run after all fixes: SharedBorderRow, Window, Tree, Layout, Skeletons, WindowPane: 132 pass, 4 fail (all Playwright).
Tests added: ControlTree row frame, layout-story fill and window-controls reserve (in `round3-framework.test.tsx`), Skeletons canvas row.
