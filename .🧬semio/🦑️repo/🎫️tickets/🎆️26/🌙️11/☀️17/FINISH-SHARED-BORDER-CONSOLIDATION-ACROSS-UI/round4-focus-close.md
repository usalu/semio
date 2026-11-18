# Round 4 focus close: focus on accent-painted cells, course skip link, live keyboard gate

## 1. One generic cue for focus on a cell that already paints the accent

Seam recolour cannot show focus where the seam is already `--active-base`. The cue is **weight, not a ring**, applied in the shared layer (`🎨️styling/🖌️ui/🎨️.css`, `@layer shared-border-priority`):

- Silhouette chips (title chip of the active window, any chip of an active silhouette): the focus path is painted by `WindowChromeSilhouetteBorder` with `data-focus-on-accent` when the silhouette kind is `active`; CSS doubles its weight (`stroke-width: calc(var(--stroke-hairline) * 2) !important`, forced colours `2`). It lies on the chip's own lines, 0 px from them. `!important` is required: the layered rule otherwise loses to the unlayered `.window-silhouette-border` (found live: first attempt stayed 1 px).
- CSS-bordered cells and controls (selected tabs, toggles, `aria-selected/pressed`, `data-state=active|on`, `data-active`): the existing label underline (2 px) now also applies to a direct focusable child of such a cell; it excludes `[role=tabpanel]`, `[data-window-silhouette]` and `[data-slot=window]` (the first draft matched every control inside an active window, caught live).
- Defect found while verifying: the focus path of a dock title chip sat 2 px above the window's own cap line (`M 1 1 L 72 1 L 72 28` against silhouette `H73 V30`), a double line and one probe pair at 1440. Fix in `🗂️WindowChrome/chip-focus.ts`: chip edges within 2 px of a line of the silhouette's own border snap onto it (`M 1 1 L 73 1 L 73 30 L 1 30 Z` live).
- Tests: `focus-seam.cases.json` (weight rule, underline rule with `> :is(...)` and exclusions, `data-focus-on-accent` in the tsx, skip link css and class), `🪟️Window/.../🟦️.tsx` (weight and underline rules), `chip-focus.test.ts` (snap case).

## 2. Course skip link

`.course-skip-link` (class added in `📚️app/🟦️.tsx`; rule in `⚖️footer-and-forced-colors/🎨️.css`): on focus the UA outline is removed, the box gets a `--stroke-hairline` `--active-base` border (`Highlight` forced) and an underlined label. Live: outline `none`, border `rgb(255,52,79)` 1 px, underline 2 px.

## 3. Live (own tab, pane shown, tab fronted before each probe, only `pending:false`; servers 5900 course, 5901/5902 OS harness, all killed, caches and logs removed). Real Tab keys, `:focus-visible` true each time

| Item | Result |
|---|---|
| Course skip link (1440) | border accent 1 px, underline, outline none |
| Course `.course-card-link` on `#clips` (1440) | outline none, shadow none, underlined, 4 focus segments 1 px `rgb(255,52,79)`, base probe 0 |
| Course `.course-input` guess field (physics hard step2, 1440) | outline none, shadow none, 1 focus segment accent, probe 0 |
| excel-workbook hard `select.course-target` at 768 and 375 | outline none, own and next cell seam accent (2 focus segments), probe 0 at both |
| OS `window-rich-folded` active title chip, Close control at 1440 / 768 / 375 | outline none, focus path 2 px accent (`onaccent`), snapped onto the cap line, probe 0 at all three (tooltip marked art) |
| OS `window-rich-folded` Window Options pane toggle (1440) | 3 focus segments 1 px, probe 0 |

Border-gap base probe 0 on every reading:

| Scope | 1440 | 768 | 375 |
|---|---|---|---|
| Course de: 13 home views + physics medium (quiz page, dialog, steps, submit dialog, results) | 0 (23 readings) | 0 (22), `ovl=4` informational overlap count on 768 pages (not violating pairs, unchanged class), 0 in the dialog and results | 0 (earlier gate) |
| Course en: clips, excel hard step1, physics hard step2 | 0 | 0 | 0 |
| OS `window-rich-folded` with chip focus | 0 | 0 | 0 |

Forced-colours rules checked in the stylesheet only (cannot be emulated).

## Tests (native)
`vitest-ui-all` WindowChrome, Window, Tabs, OverviewCard, CardStack: 127/127; `vitest-course`: 1135/1135; `vitest-os-final`: 158/158.

## Files
`🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`, `🧱️elements/🗂️WindowChrome/🟦️.tsx`, `chip-focus.ts`, `chip-focus.test.ts`, `focus-seam.cases.json`, `🧱️elements/🪟️Window/🧪️tests/🧩️component/🟦️.tsx`, course `📚️app/🟦️.tsx` and `⚖️footer-and-forced-colors/🎨️.css`. Ticket tooling: `r5-find.js`, `r5-live.js`.

## Open
Icon-only controls inside an accent-painted CSS cell (not a silhouette chip) get the underline cue, which is invisible on an icon; none seen in the live views. The contrast of the doubled accent line equals the earlier measured `--active-base` values.

## Follow-up: no `!important` for the doubled focus line (cascade fixed at the root)

- Root cause: the unlayered `.window-silhouette-border { stroke-width: var(--stroke-hairline) }` beat the layered focus rule. It now sits in `@layer shared-border-ownership` (the lowest declared layer), so `@layer shared-border-priority` wins by layer order. The `!important` is gone from both the normal and the forced-colours doubled-weight rule.
- Audit of every CSS use of `window-silhouette-border` (`🎨️.css`, `🌐️globals/🎨️.css`): the kind rules (`-introduced`, `-loading`, `-waiting`, `-active`, `-normal`, celebrated) set their own stroke-width/stroke unlayered and are unchanged; the others set `stroke` (hover, forced colours, ghost), `pointer-events` or opacity, never the base stroke-width, so nothing relied on the base rule being unlayered. Live check: kind paths stay 1 px, the focus path is 2 px.
- Guard: `focus-seam.cases.json` requires the layered base rule and forbids the `!important` doubled-weight rules and the unlayered base rule. The remaining `!important` in the shared-border layer are the seam/outline/stroke recolours that must beat utility classes and unlayered product CSS; they are untouched.
- Tests: WindowChrome, Window, Tabs, OverviewCard, CardStack 127/127; `vitest-course` 1135/1135; `vitest-os-final` 158/158.
- Live OS `window-rich-folded`, active title chip Close focused by real Tab: 1440 probe 0, focus path 2 px `rgb(255,52,79)`, path `M 1 1 L 73 1 L 73 30 L 1 30 Z`; 375 probe 0, 2 px. Server on 5903 killed, cache and log removed.
