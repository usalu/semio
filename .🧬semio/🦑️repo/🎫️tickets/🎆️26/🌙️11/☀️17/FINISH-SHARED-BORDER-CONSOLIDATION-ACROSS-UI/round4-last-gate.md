# Round 4 last live gate (chip snap, chip-gap abut, focus as seam recolour, credits popover)

Method: own tab in the in-app browser (pane shown, tab fronted, `pending:false` readings only), `border-gap-probe.js` base probe, native vite servers on 5800-5808 with fresh caches (all killed, caches and logs deleted). Three viewports 1440x900, 768x1024, 375x812. Servers: elements 5800, OS 5801, CAD 5802, staffage 5803, demonstrator 5804, play 5805, presentation 5806, hub admin 5807, course 5808. Three dev servers (OS, CAD, demonstrator) ran away at 100 % CPU once and were restarted; the elements harness config has `watch: null`, so it was restarted after every shared CSS edit.

## Gate table (violating pairs, 0 = pass)

| App / scope | 1440 | 768 | 375 |
|---|---|---|---|
| Elements, 126 stories | 0 | 0 | 0 |
| OS harness, 72 parts | 0 on 72 | 0 on 63, 9 parts class A/B (below) | 0 on 69, `layout`, `layout-bottom`, `layout-many` class D |
| CAD harness | 0 | 0 | 0 |
| Staffage: overview, all, animals, collections (+bottom), compare, playground | 0 | 0 | 0 |
| Staffage assets (walking-commuter, domestic-cat, cafe-chair, circulation-pump) Three views + 3D toolbar | 0 (3D stage `preparing`, no GPU here) | 0 (domestic-cat) | 0 (domestic-cat, stage `ready`) |
| Demonstrator intro / overview / opened / back | 0 | 0 | 0 |
| Demonstrator opened pane, credits popover open | n/a (full captions) | 0, footer 768/768 | 0, footer 375/375 |
| Play intro / landing / opened / back | 0 | 0 (landing was 1-2 before the chip snap) | 0 |
| Presentation, 34 slides, rest + every disposition kind (text, authors, affiliations, figure, pdf x2, json, iframe, markdown) + enlarge | 0 (112 readings) | 0 (112) | 0 (110) |
| Hub admin, 7 tabs + spaces expanded + create dialog | 0 | 0 | 0 |
| Course en: 13 home views, physics medium (submit dialog + results), physics hard (results) | 0 (32 readings) | 0 base (31), near probe 1 informational on `#bim` and `#clips` (0.05 and 3.06 px, content behind the sticky navbar, base 0) | 1, class G (4.39 px, hard step1) |
| Course en: excel-workbook hard (select.course-target rows) | 0 | not run | not run |
| Course de: 13 home views + physics medium with dialog + results | not run | not run | 0 (22 readings) |

## Real defects found and fixed in this gate

1. OS `window-rich-many` at 768: the Search chip and the Window Options chip of one window (38.5 / 39.7 px, a sliver U-gap the `chip-abut` rule did not close). Root cause: the chips are separate folded panes (`Pane`, anchored top-left / top-middle / top-right), i.e. separate silhouettes, so `settleChipRowAbuts` (one cap row) never saw them as a row. Fix: `settlePaneChipAbuts` / `chipAbutSize` in `🗂️WindowChrome/chip-abut.ts` order the folded anchored panes of one window body by position, a left-anchored chip reaches next, a right-anchored one prev, and the marked chip's stack takes the width that closes the gap (`--chip-abut-size`, CSS `[data-anchor] > [data-window-silhouette]:has(> [data-window-silhouette-chip][data-chip-abut])`). Wired in `🪟️Window/🟦️.tsx` via `observeChipRowAbuts(body, settlePaneChipAbuts)`. Tests: `chip-abut.cases.json` (2 new language-neutral cases), `chip-abut.test.ts` (size and painted-body tests). Live: 768 gap 0 (chips abut, count 0); 776 gap 40.5 stays open; 790 and 804 open (44 / 47.5), count 0, no flapping; 1440 open (206 px).
2. Focusable tabpanel with no or a UA ring: OS dock tabpanel `#mode-dock-panel-*` showed the browser's `outline: auto` ring, and `Tabs` content (`tabIndex=0`, `outline-none`, also passed by a story) had no visible focus at all. Fix: `[role="tabpanel"]:focus-visible { outline: 1px solid var(--active-base) !important; outline-offset: -1px }` (the one line on the edge the panel coincides with; forced colours `Highlight`) in `@layer shared-border-priority`, `outline-none` removed from `TabsContent`. Live: OS tabpanel `solid 1px rgb(255,52,79)`, Tabs content the same.
3. A selected / pressed / active control cannot show focus by recolouring its seam to the accent it already paints (selected `Tabs` trigger: focus invisible). Fix: second cue, the label is underlined (`:is([data-state="active"], [data-state="on"], [aria-selected="true"], [aria-pressed="true"], [data-active="true"]):not([role="tabpanel"]):focus-visible`, 2 px). Live: `Properties` trigger `underline`.
4. `OverviewCard` whole-card button: the focus stroke rule lost against the unlayered `[data-shared-border-segment][data-shared-border-state="normal"]` (layered normal beats nothing here, unlayered normal beats layered): card stroke stayed grey on Tab. Fix: `!important` on the card focus stroke (layered important beats unlayered normal). Live: stroke `rgb(255,52,79)` on Tab, card outlined red.
5. Course sort buttons (`.course-cell`, `course-sort-up/down`) kept the UA `outline: auto` ring beside their seam recolour. Fix: `.course-cell:focus-visible { outline: none }` (results-and-preferences css). Live: `out=none`, seam `border-inline-start` accent.
Tests: `focus-seam.cases.json` extended (tabpanel rule, underline rule, `!important` card stroke, course-cell outline, Tabs content without `outline-none`); WindowChrome, Window, Tabs, CardStack suites 109/109 (the Playwright browser test needs a chromium binary that is not installed here and was not part of this run).

## Keyboard focus-visible (real Tab key, `:focus-visible` true each time)

| Item | Result |
|---|---|
| OS window chips (title chip close, pane toggles Actions / Search / Window Options, action group Draw / Move / Focus / Close) | outline none, box-shadow none, compositor focus segments painted (4-6) in `--active-base` around the focused chip, e.g. the Search chip outlined in the accent; seams of the group recolour (frame border accent). |
| OS window cap chips: sliver at 768 | closed, see fix 1 |
| OS footer chips (`window-chrome-footer`) | 0 pairs at 768 / 375 in the sweep, no clipping |
| Elements `LayeredOverview/Strip`: whole-card Open button | card silhouette (13-15 strokes) `rgb(255,52,79)`, visible in the screenshot; `OverviewCard/Button` fixed (fix 4) |
| Elements `Tabs/Default` | trigger underline + tabpanel line (fixes 2, 3) |
| Elements `ActionGroup/Default`, `Panel/PanelNestedTabs` (panel tab buttons), window pane chrome toggles | focus segments (4 / 1 / 2) recolour, no ring |
| Course: submit dialog primary (`Submit quiz`) chip | outline none, shadow none, chip rectangle in the accent (4 focus segments) |
| Course `select.course-target` (excel-workbook hard) | outline none, own and next cell `border-inline-start` accent, 2 focus segments |
| Course sort buttons | fixed (fix 5) |
| Course nav bar buttons (Course overview, Back, Forward, Up) | outline none, focus segments (5-11) |
| Course `.course-input` guess field, `.course-card-link` on the overview | not reached in the runs (no guess input in the executed hard/medium runs, overview card link not tabbed): covered by `focus-seam.cases.json` only, not live |
| Skip link `Skip to main content` (course) | still `outline: auto 1px` UA ring; it is a standalone skip link outside the border grid, left as is (informational) |

## Focus colour contrast (WCAG 1.4.11 / 2.4.7)

`--active-base` = `rgb(255,52,79)` (`#ff344f`), computed with sRGB relative luminance:

| Surface | Contrast |
|---|---|
| Dark theme, window surface `rgb(6,26,31)` | 4.98 : 1 |
| Dark theme, card / base `rgb(0,17,23)` | 5.36 : 1 |
| Dark theme, page `rgb(0,0,0)` | 5.86 : 1 |
| Dark theme, glass chip fill `rgb(38,56,58)` | 3.43 : 1 |
| Dark theme, OS panel parent `rgb(0,17,23)` | 5.36 : 1 |
| Light theme, base `rgb(247,243,227)` | 3.22 : 1 |
| Light (white story host) | 3.59 : 1 |
| Unfocused seam `rgb(123,130,125)` vs dark base | 4.88 : 1 (focus vs unfocused seam 1.10 : 1 by luminance, the change is hue, plus thickness 1 px accent on the seam) |
| Focus on an already-accent surface (selected tab, active window chip) | 1.00 : 1 vs that fill; the outside half of the stroke still meets 4.98 : 1 against the dark surface; selected controls get the underline (fix 3); the active window's own title chip focus relies on the tooltip (`Close`) and the focused control name, no extra cue |

All numbers meet 3 : 1 where the indicator touches a different surface.

## Forced colours

The browser tools cannot emulate `forced-colors`. Verified only through the CSSOM: the live stylesheets contain `@media (forced-colors: active)` rules with `Highlight` for cell seams, frame borders, `[role="tabpanel"]:focus-visible`, the overview card stroke, compositor segments (focus and hover) and the course `:focus-visible` / `.course-cell` rules. Not verified visually.

## False positives and classified leftovers (unchanged classes)

- A. OS 768 `settings-general|theme|keys|defaults|conflicts`, `display`, `marketplace`, `panel-bottom`: 38.6 px between the a/b split and the left edge of the floating settings panel (two layers).
- B. OS 768 `layout-bottom`: 12 px, bottom-right panel over the canvas (two layers).
- D. OS 375 `layout`, `layout-bottom`, `layout-many`: desktop `Layout` forced into a phone viewport by the harness; real `layout-mobile*` are 0.
- E. Elements `LayeredOverview/ReducedMotion`, `Strip`: informational overlaps only, count 0.
- G. Course 375 `physics/hard step1`: 4.39 px between the sticky navbar and `select.course-target css:top` of content scrolled under it at one offset (same class as the earlier 1.92-35 px cases); course 768 `#bim`, `#clips` near-probe 0.05 / 3.06 px, same sticky-navbar class, base 0.
- Informational: demonstrator opened pane in the harness shows `Plugin Recovery: This program crashed` (no plugin runtime in the harness), unrelated.
- The earlier Open 3 (OS 768 `window-rich-many` U-gap) is closed by fix 1; Open 4 (course focus rings) by fixes 2-5 plus the earlier ticket changes.

## Files touched in this gate

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/chip-abut.ts`, `chip-abut.cases.json`, `chip-abut.test.ts`, `focus-seam.cases.json`; `🧱️elements/🪟️Window/🟦️.tsx`; `🧱️elements/📑️Tabs/🟦️.tsx`; `🎨️styling/🖌️ui/🎨️.css`; `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🏁️results-and-preferences/🎨️.css`; ticket tooling: `round3-os-run.js` (module cached per page, was reimported per part).
