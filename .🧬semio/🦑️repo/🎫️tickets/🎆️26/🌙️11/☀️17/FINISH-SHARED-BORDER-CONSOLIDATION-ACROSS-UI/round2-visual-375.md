# Round 2 — course 375x812 visual defects

Own tab, fronted before every probe, `pending:false` checked. Note: the browser pane renders screenshots at only ~129 px wide, so the evidence below is DOM/`elementsFromPoint` based, not pixel based.

## 1. Navbar brand overlapping Back/Forward

Before: at 375 px the nav ways kept their captions ("Course overview", "Back", "Forward", "Up"), so the flow row was 420 px wide and overflowed the bar (the DE button sat at x=375-420). `navbarFreeBandV1` found no free band, answered the whole width, and the centred brand was placed at 154-210, on top of Back (123-182) and Forward (182-257).

After: the captions of the ways are `max-md:sr-only` (icon only below 768 px, still the accessible name; `title`/`aria-label` unchanged) in `🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🚏️navigation/🟦️.tsx`. The row is 129 px of ways + 2 language buttons; the brand fits in the free band (154-220, buttons at 1-129 and 282-374). The framework Navbar and the `.course-brand` give-way CSS were already correct; they were only defeated by the overflowing row.

## 2. Bordered blurred boxes in the title-chip band and footer band (swipe view)

Found with `elementsFromPoint` at (320,70) and (60,737): through the card's U-gap (`[data-window-silhouette-gap]`, a real cut-out by design and test) nothing foreground lies; below it comes `[data-layered-strip]` of the nested LayeredOverview, i.e. the BACKDROP PAGES (real pages: the full-size cards 0,29,375,576 / 0,576,375,715 ... with their silhouettes and shared-border cells) seen through the `ui-veil` glass (`blur(8px)`, alpha 0.4). Blur 8 px does not erase borders, hence "bordered blurred boxes". They are background-layer cards, not foreground elements.

Fix (`🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`, next to the tinted-card rule): `[data-layered-veil]` is a flat opaque `--surface-bg` backdrop (no backdrop-filter). Backdrop pages are only visible through the hole of a revealed (pressed) card, crisp. The glide/pan of the strip still runs, it just is not visible under the closed glass.

## 3. Stray vertical fragments at x≈0 (y 545-575) and x≈374 (y 575-605)

Same owner: edges of the backdrop pages' cards peeking through the translucent edge hints (`button[data-layered-neighbour].ui-glass`, 0,57,28,753 and 347,57,375,753) and the veil: card 1 (0,29,375,576) has its footer U-gap in 545-576, card 2 (0,576,375,715) its title band 576-602, whose side borders sit at x=0 / x=374. Removed by the same opaque veil; no separate scrollbar or border owner exists there.

## Probe: `border-gap-probe.js`

- Existing API/fields kept. New `overlaps` (array of `{area, a, b}`): pairs of visible interactive controls (button, a, [role=button], input, select) or text-bearing leaf elements whose rects (clipped to scrollports and the viewport) intersect by more than 2 px of area, neither containing the other; art, `.sr-only`, inert/behind-glass, hidden are excluded. Sweep-and-prune, 5-80 ms. Verified with a synthetic overlapping button pair (reported, area 900).
- Coordinator request: an open modal layer (`[role=dialog][aria-modal=true]`, `[role=menu]`, `[role=listbox]`, `[popover]:popover-open`, `[data-slot^="introduction-"]`) is its own layer for gap pairs and overlaps: pairs across it are skipped, pairs inside count. New export `clear()` removes `[data-border-probe-mark]`; `mark:true` also marks overlap pairs.
- `spanned()` tolerance widened 1.5 -> 2 px: the 1 px stroke inset of the footer line plus the fractional navbar height (28.797 px, so the scroller ends at 781.203) made the task card's footer U-cavity (chips risers at x=86/290, footer closing line) miss by 0.06 px and report a 33 px "gap" at the bottom of the run view (scroll end, 375). That cavity is enclosed by the chip risers and the footer line (a probe false positive).
- Related framework change: `createSharedBorderSnap` default tolerance 0.5 -> 0.75 px (a scroller's rounded scroll range left the last card 0.56 px above the footer). `snap.cases.json` gained a 0.56 px case; the "two lines" cases now use 1 px / 0.9 px separations. Residual: at the U-gap ends the footer closing line (782.2) and the chip bottoms (780.9) can still differ by ~1 px at fractional bar heights; not touched.

## Sweep (probe count / overlaps)

| view | 375x812 | 1440x900 |
|---|---|---|
| course home | 0 / 0 | 0 / 0 |
| #energy | 0 / 0 | 0 / 0 |
| #physics | 0 / 0 | 0 / 0 |
| run (Physical Understanding, Medium), scroll top / mid / bottom | 0/0, 0/0, 0/0 | 0/0, 0/0, 0/0 |

Before (375, energy): navbar overlap, defects 2/3 invisible to the probe (behind glass), run view bottom 1 (the false positive above).

## Tests

- Course (`vitest-course.config.ts`): 1124/1125 in the full run, the single failure (`deputy-and-contract`, heading "Haushaltsphysik" not found in time) is a load flake: the file passes alone, 25/25. The 4 "no test suite" files are the kit helpers matched by the config's glob (same as before).
- UI regression (`vitest-ui.config.ts`, Navbar, WindowChrome, WindowSilhouette incl. `snap.test`, LayeredOverview, `owned-locale-detector-retirement`): 689/689 tests pass; the "failed files" are helper modules without suites and a `bun:test` file picked up by the positional filter.

## Files

- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🚏️navigation/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.ts`, `snap.cases.json`
- `TICKET/border-gap-probe.js`
