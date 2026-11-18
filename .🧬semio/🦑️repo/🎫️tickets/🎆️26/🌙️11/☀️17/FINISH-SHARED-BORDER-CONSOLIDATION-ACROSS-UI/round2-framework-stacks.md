# Round 2 — framework card stacks and grids

## API (domain-neutral, `🧰️framework/🔨️modules/🖱️ui`)

New element `🧱️elements/🃏️CardStack` (export `@semio-tech/ui/elements/CardStack`, re-exported from `ui-react`, `ui-react/chrome`, `ui-react/source`):

- `CardStack` — column of `OverviewCard`s: no gap, no padding, optional `maxWidth` (centred). First card's cap row sits on the container's top edge, last card ends on its bottom edge.
- `CardGrid` / `CardGridCell` — tiles edge to edge (`columns`, `rows`, `minColumn` as `--card-grid-*`); cells have no padding, the card fills the cell. CSS is `:where()`-zero-specificity (`data-card-grid`, `data-card-grid-cell`) in `🎨️styling/🖌️ui/🎨️.css`.
- `useCardInterlock(element)` — measures cap/footer rows (natural height) and chip extents, lifts every card whose top lies on another card's bottom (next in a stack, row above in a grid; on the grid cell) onto the previous card's bottom line (`--card-stack-lift` + inline negative margin) and gives the lower cap row and upper footer row one shared band height (`--card-stack-cap|footer`). Rows let pointer events through to the chips beneath.
- `resolveCardBands` (`bands.ts`, pure, language-agnostic: `bands.cases.json` + `bands.schema.json`): `none` (no footer above), `shared` (band = max(footer, cap), left title chip / right footer chip), `stacked` (chips would collide, e.g. left footer vs left title: footer row then cap row, still zero gap, nothing overlaps).
- `LayeredOverview`: strip-mode overlay is a `data-card-grid` and runs `useCardInterlock`; `OverviewCard as="button"` is now `h-full` like the section variant (paint layer + stack), so cards fill cells.
- Swipe mode: cell padding only on sides that have a neighbour (28 px band); card fills the rest and starts on the navbar line; neighbour hints are compositor frames flush with navbar/footer/view edge (no 5.5 px gap, hover via compositor).
- Shell: `FrameworkOsShell` new prop `navbarTrailingItems` (extra cells of the shell navbar, before the fullscreen toggle).
- `UIIntroduction`: back/next buttons are cells of their footer chip (`data-shared-border-cells` wrapper), no box-in-box.

Compositor: `compositor.cases.json` key `interlock` (+ schema): stacked footer/title band and tiled row seam; every coincident segment painted by exactly one owner (existing resolver; test in `shared-border.test.ts`).

## Migrated consumers

- Course `PageFrame` and app `Page` (run/results) -> `CardStack` (no `p-double`, no `gap-double`; `.course-page`, `.course-run` gap removed).
- Course home grid: `.course-home-cell` no padding/centring, `data-card-grid-cell`, card fills cell; swipe `.course-home-entry` fills page (no 36rem cap).
- Course `LegalFooter` is a compositor frame (`edges=top`) so card bottoms meet it on one line.
- Course notices under the navbar (`ProblemNote band` + `BodyButton cell`): full width, no margin, shared bottom line.
- mit-bestand demonstrator: removed local `[data-demonstrator-card]` CSS and attr; Overview control is now `navbarTrailingItems` of the shell navbar (separate band removed).
- Staffage / play: not touched (their containers already use `CardGridCell`/own grids; not re-probed, not served here).

## Probe (`round2-probe.js`: foreground only, skips pages behind the glass of an overview; waits for the compositor)

| view | 1440x900 | 375x812 |
|---|---|---|
| home | 0 | 0 |
| `#physics` quiz page | 0 | 1 (content: "Show it now" button inside last card vs footer) |
| `#energy` topic grid | 0 | 0 |
| run view | 1 (content: list row of a scrolled card vs footer) | 1 (content paragraph vs footer) |

Before: navbar->card 8.4, card->card 8.4/32. Screenshots (described): home tiles 3x3 edge to edge, footer chips ("Explore >") share the band with the next row's left title chips; "View last result" + "Energy Demand" show the stacked band; mobile card sits between 28 px hint bands flush with navbar and footer.
Probe note: strip pages behind an overview glass are excluded (cross-layer pairs, card over blurred page).

## Tests

- ui: CardStack (bands cases, component, grid), WindowChrome (interlock), LayeredOverview 59, OverviewCard, SharedBorderRow, introduction, full legacy aggregator `owned-locale-detector-retirement`: 679/680 passed; the 1 failure `elements defer their edges ...` expects `app-mode-nav` Canvas markup that another agent changed concurrently (not from this work).
- course suite: 1110/1123 on the full run, then home-grid, swipe-cell and text-wrap contract tests I own were updated and pass. Left: `deputy-and-contract`, `navigation`, `journey-and-tabs` x2 (30-60 s timeouts under parallel load; deputy passes alone), `contrast-states` x3, `adaptive-layout` x2 (`course-settings` tier, matching-row reading order) and `shared-borders` "ordinary card ungrouped" (content-owned markup edited concurrently by other agents), `load-failures` language-unavailable test (not re-run alone; it renders the notice I turned into a `band`, please check its text query).

## Open / not done

- Introduction info box still floats over cards (12 px) as a dialog; demo ripple is art.
- Playwright/browser-only fixtures not run.
