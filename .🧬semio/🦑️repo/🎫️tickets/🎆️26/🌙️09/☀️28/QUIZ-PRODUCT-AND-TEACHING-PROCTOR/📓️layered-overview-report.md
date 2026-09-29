# 📓️ Layered overview — shared element, geometry, play and demonstrator migration

## 00. API change 2026-09-29 (night): grid rest (`📓️design.md` §17) — read first

Additive; nothing existing changed, play and the demonstrator are untouched (`rest` defaults to `"panorama"`).

```ts
export type LayeredRest = "panorama" | "grid";
export interface LayeredTracks { readonly columns?: readonly number[]; readonly rows?: readonly number[] }   // fr weights, missing ones weigh 1
// LayeredOverviewProps gains:
readonly rest?: LayeredRest;          // default "panorama"; only for mode "strip" (list mode ignores it)
readonly gridTracks?: LayeredTracks;  // track sizes of the grid rest (and of the overlay variables below)
```

Behaviour of `rest="grid"`:
- **At rest every page is mounted and live** (booted at mount, no warm queue), `inert` + `aria-hidden` as before. Each page is laid out at the
  full container size and placed with its own imperative `transform: translate3d(…) scale(k)` (origin `0 0`) and `clip-path: inset(…)`: it
  **covers its track cell without distortion** (uniform scale = max of the cell's width/height fraction, anchored top-centre, the overflow cut
  off). With `C = R` equal tracks the pages fit their cells exactly (3 × 3 → `scale(0.333333)`, no cut).
- The cells are the host's `cells` over the tracks of `gridTracks` (default equal). For a page to lie exactly behind its card, the overlay grid
  must use the **same tracks with no outer padding and no gaps between tracks** (spacing inside the cells). The overlay element carries the track
  lists as CSS variables: `grid-template-columns: var(--layered-columns); grid-template-rows: var(--layered-rows)` (e.g. `minmax(0, 1fr)
  minmax(0, 1.5fr) minmax(0, 1fr)` for `gridTracks.columns = [1, 1.5, 1]`).
- **Reveal** (mouse enter or keyboard focus) zooms the camera from the whole grid to that page's cell — same 500 ms `easeInOutCubic`, epoch
  guard, reduced motion snaps — so the page grows to full size while its neighbours slide out; the veil hole is that page's current on-screen
  rectangle every frame (glass hidden once it fills the view). **Leave/blur** restores the glass at once and zooms back out. **Open** = the page
  at full size as before (hash, focus, Escape/Overview); closing zooms back out (a keyboard close returns focus to the card, which keeps its page
  revealed until focus leaves). A deep link starts zoomed in on its page. **No pointer pan.**
- New root attribute `data-rest="grid" | "panorama"`; the strip has no transform in grid rest; each `[data-layered-pane]` carries the transform.
- New pure helpers (chrome + barrel): `LAYERED_VIEW`, `restRect(cell, grid, tracks)`, `trackSpans`, `trackTemplate`, `lerpRect`, `glideRect`,
  `viewRect(rest, camera)`, `coverPlacement(view)` → `{ transform, clipPath }`, `veilForRect(rect)`, `spanAxisBounds`; types `LayeredRect`,
  `LayeredTracks`, `LayeredRest`. Fixture/schema/oracle cover them (`rests`, `trackTemplates`, `views`, `zooms`, `placements`, `rectVeils`).
- Budget/time release still apply as configured: for the grid rest pass `budget ≥ panes.length` and no finite `suspend*Ms`, or released pages
  show their placeholder in the grid.

## 0. API as built (read first — for the quiz home adoption)

The §10.2 API of `📓️ui-reference-layered-landing.md` is implemented **exactly**; nothing was renamed or removed. Import everything from
`@semio-tech/ui-react/chrome` (also re-exported by the barrel).

| Item | As built | Change vs §10.2 |
|---|---|---|
| `LayeredOverviewProps` | `panes, cells, renderCard, overlayClassName?, overlayStyle?, renderChrome?, mode?, rest?, gridTracks?, pan?, routing?, openedId?, onOpenedIdChange?, onRevealedIdChange?, lifecycle?, windowing?, reducedMotion?, labels` | `rest?`, `gridTracks?` added (§00) |
| `LayeredPane` | `id, label, icon?, render(state), poster?, capturePoster?` | none |
| `LayeredPaneState` / `LayeredCardState` | `{opened, revealed, dirty}` / `{mode, revealed, opened, open}` | none (`opened` of a card is always `false`: cards only exist while nothing is opened) |
| `LayeredLifecycle` | `budget, warmStartMs, warmIntervalMs, suspendIdleMs, suspendOffscreenMs, suspendHiddenMs, sweepMs` | none |
| Named helper types | `LayeredMode = "strip" \| "list"`, `LayeredChromeState = {revealed, opened}`, `LayeredLabels = {grid, overview, waiting(pane), failed(pane)}` | **additive** names for the inline types of §10.2 (structurally identical) |
| `onOpenedIdChange` | uncontrolled: notified on every change **including the initial deep link** (mount); controlled: a request (also sent on mount when the hash names another pane) | clarification |
| Defaults | `mode "strip"`, `pan "pointer"`, `routing "hash"`, `reducedMotion "auto"`, `windowing.radius 1`, lifecycle `{budget 4, warmStartMs 1500, warmIntervalMs 35000, suspendIdleMs ∞, sweepMs 5000}`; `suspendOffscreenMs`/`suspendHiddenMs` default to the resolved `suspendIdleMs` | as §10.2 comments |
| Geometry helpers | `nearSquareGrid(count)`, `centeredRowSpan(row, count)`, `centeredLastRowCells(count)` (**array** in pane order — map it to ids), `stripGrid(cells)`, `cellOffset(cell)`, `occupiedColumns(cells, y)`, `clampOffset(offset, cells)`, `pointerOffset(fx, fy, grid)`, `easeInOutCubic`, `lerpOffset`, `glideOffset`, `followStep`, `stripTransform`, `paneAxisBounds` (fractions), `veilClip(cell, offset)` → `LayeredVeil` (`whole`/`clear`/`hole`), `veilPolygon`, `veilClipPath`, `windowAround`, `inWindow`, `resolveLifecycle`, `nextWarmBoot`, `warmDelay`, `panesOverBudget(liveByRecency, keep, budget)`, `panesToRelease`, `scheduleIdle`; constants `LAYERED_GLIDE_MS`, `LAYERED_FOLLOW_LERP`, `LAYERED_FOLLOW_EPSILON`, `LAYERED_DEFAULT_LIFECYCLE` | §10.2 names kept; extra helpers additive |
| `capturePosterFromCanvases(container)` | exported next to the element (it touches the DOM, so it is not in the pure module) | placement only |

Things the host must know (they are the behaviour of §10.3, stated for adoption):
- Each card is wrapped in a `div.contents[data-layered-card=<id>]` (use descendant selectors, e.g. `.quiz-home-grid [data-card="board"]`). The overlay is `pointer-events-none`, so a card needs `pointer-events-auto` (the `as="button"` `OverviewCard` has it; a `section` card must add it).
- Reveal: `pointerenter` with `pointerType === "mouse"`, or a **keyboard** focus (`:focus-visible`) inside the card wrapper; conceal: `pointerleave` (mouse) or focus leaving the wrapper. A mouse click that focuses a card does not reveal by focus (the hover already did).
- On close, focus goes to the first focusable element of the card (`a[href]`, enabled `button`, …) — the heading link for the quiz cards.
- Hash routing uses `replaceState`; a hash that names no pane (e.g. `#main` of a skip link) is ignored and never cleared.
- `render` is called only while a pane is mounted; `labels.waiting/failed` are called on every render (keep them cheap).
- Equal-but-fresh `panes`/`cells` arrays are harmless (ids and cells are kept stable by value, so a re-render never restarts a glide or the warm
  timer). For the cheapest re-renders keep `panes` and `renderCard` referentially stable (`useMemo`/`useCallback`): pane views and card hosts are
  memoized, so a reveal then re-renders only the two affected cards. The quiz home (`🏠️home/🟦️.tsx`) currently rebuilds both per render — it works,
  it just re-renders all nine cards and pages on every home render.

## 1. Files

| File | What |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts` | **new** pure module (no DOM): contracts `LayeredCell/Offset/Grid/Span/Lifecycle`, grid (`nearSquareGrid`, `centeredRowSpan`, `centeredLastRowCells`, `stripGrid`, `occupiedColumns`, `clampOffset`, `pointerOffset`), motion (`easeInOutCubic`, `lerpOffset`, `glideOffset`, `followStep`, `stripTransform`), veil (`paneAxisBounds`, `veilClip`, `veilPolygon`, `veilClipPath`), windowing, lifecycle policy (`resolveLifecycle`, `nextWarmBoot`, `warmDelay`, `panesOverBudget`, `panesToRelease`, `scheduleIdle`), grid rest (`LayeredRect`, `LayeredTracks`, `LAYERED_VIEW`, `trackSpans`, `trackTemplate`, `restRect`, `lerpRect`, `glideRect`, `viewRect`, `coverPlacement`, `spanAxisBounds`, `veilForRect`; `veilClip` now delegates to `veilForRect`) |
| `…/🔨️modules/🥞️layered-overview-geometry/🧪️tests/🔬️unit/🟦️.ts` | **new** fixture-driven unit tests + oracles `polygon-clipping` (veil areas, panorama and grid rest), `d3-ease`, schema check (Ajv), plus laws (rest cells tile the view for any tracks; every placement is undistorted and exactly inside its view rectangle; the zoom hole equals the page's current view) |
| `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🥞️layered-overview/🔣️.json` | **new** language-agnostic vectors (§10.8): grids incl. 148 → 13 × 12, centred last row, occupied columns, clamps, pointer offsets, easing, glides at 0/125/250/375/500/600 ms, follow steps, strip transforms, 9 veil cases (whole / clear / partial / corner / mid-glide / epsilon) with evenodd polygon and area, windows, windowed-cell counts, lifecycles (null = never), warm boots, warm delays, budgets, time-based releases; grid rest: `rests` (uniform, the quiz's 1 : 1.5 : 1 / 1 : 1.4 : 1 tracks, tablet 2 × 5, demonstrator 4 × 2, missing weights), `invalidTracks`, `trackTemplates`, `views` (camera → view), `zooms` (0/125/250/500/700 ms both ways), `placements` (fit, full, outside, mid-zoom, wide/tall cover-crop, quiz centre), `rectVeils` (whole / clear / outside / at rest / mid-zoom / edge cut) |
| `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🥞️layered-overview/🔣️.json` | **new** draft-07 schema of the fixture (schema-first; validated in the unit test) |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx` | **new** element + `capturePosterFromCanvases`; imports only `cn`, `loadingBorderClass`, `Icon` and the geometry module (no barrel, no `CanvasSkeleton` — local placeholder) |
| `…/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx` | **new** 25 DOM cases (Testing Library, fake clock), 5 of them for the grid rest |
| `…/🥞️LayeredOverview/📖️stories/🧪️.story.tsx` | **new** stories Strip / List / Opened / ReducedMotion / GridRest (the rig renders GridRest in a real browser) |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts`, `…/⚛️react/🟦️.tsx` (barrel) | exports (chrome + barrel region `🥞️LayeredOverview`) |
| `…/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the two test files |
| `…/📦️packages/🟦️typescript/package.json`, `bun.lock` | devDependencies `polygon-clipping@0.15.7`, `d3-ease@3.0.1`, `@types/d3-ease@3.0.2` (test-only) |
| `…/📦️packages/🟦️typescript/📋️project.json` | nx `namedInputs` gain `🖱️ui/🔨️modules/**`, `🧫️fixtures/**`, `🧬️schema/**` (the new tests read them; the cache ignored them before) |
| `🧰️framework/🔨️modules/🖱️ui/🔮️oracles/🔣️.json` | oracles `polygon-clipping` (capability `layered-veil-area`) and `d3-ease` (`layered-glide-easing`) |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️Label/🟦️.tsx` (+ barrel) | **new** `useLabelFormatter(key)` — `useLabel` for a label interpolated per item (needed for `labels.waiting/failed` without casts) |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx` | `onMouseEnter/onMouseLeave/onFocus` props removed (§10.4: the element's card host owns hover/focus); no other consumer used them |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | member names `🥞️LayeredOverview` (elements), `🥞️layered-overview-geometry` (modules), `🥞️layered-overview` (fixtures, schema) |
| `🏢️semio-tech/🎡️play/🟦️.tsx` | 849 → 206 lines: labels, `PlayShell`, pages, cells, card placement, chrome, lifecycle `{budget 4, warmStartMs 4000, warmIntervalMs 35000, suspendIdleMs 120000}` |
| `🏢️semio-tech/🎡️play/⚛️play-card.tsx`, `🪧️brand.ts`, `🧪️tests/🧪️playgrid/🟦️.ts`, `🧪️tests/🎭️acceptance/🟦️.ts` | hover props and `data-play-pane-card`/`data-pane-id` gone; grid/idle/budget helpers moved to the geometry module (their vectors moved into the fixture); acceptance hooks → `data-layered-*` |
| `♻️mit-bestand/🧺️demonstrator/🟦️.tsx` | 892 → 207 lines: labels (now en + de bundles instead of hard-coded German), `DemonstratorShell`, pages, cells, chrome with credits, lifecycle `{budget 8, warmStartMs 1500, warmIntervalMs 35000, suspendIdleMs 300000, suspendOffscreenMs 30000, suspendHiddenMs 60000}` |
| `♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx`, `🪧️brand.ts`, `🎨️globals.css`, `🧪️tests/🎭️acceptance/🟦️.ts` | card label from an en/de bundle, icon title typed (`uiDataLabel`), hover props gone; `scheduleDemonstratorIdle` → `scheduleIdle`; CSS keyed on `[data-overview-card]`; acceptance hooks → `data-layered-card` |
| `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorpanebootvariants/🟦️.ts` | replaces `🧪️scheduledemonstratoridle` (its scheduler case moved to the fixture test) |
| ticket: `layered_overview_rig.ts`, `layered_overview_mutations.py` | runtime rig (real landings, faked shell, Vite + Playwright) and mutation checks |

## 2. Behaviour and decisions (beyond the letter of §10.3)

- **Geometry in percent of the container, written imperatively.** Strip `width: C·100%`, `height: R·100%`, `translate3d(−x·100/C %, −y·100/R %, 0)`;
  one rAF chain (epoch-guarded, stops when settled) writes transform, veil `clip-path`/`visibility`/`data-veil`; React renders only when the
  window of cells changes. Root `overflow: clip` (a focus inside a pane cannot scroll the strip), panes `contain: layout paint` (fixed-position
  overlays of an opened app now resolve against the pane, not the C × R strip).
- **Veil**: one `ui-veil` with `data-level="dialog"` (the level `🖌️ui/🎨️.css` prescribes; measured tint `color(srgb .746 .753 .712 / .4)`
  instead of the base tint), hole computed from the current offset at reveal start and every frame.
- **Reveal on focus only for keyboard focus** (`:focus-visible`), so a mouse click that focuses a card or the focus returned after a mouse
  click on Overview does not pin a reveal the pointer never asked for; Escape (keyboard) returns focus and reveals.
- **Lifecycle**: the warm queue pre-boots at most `budget` panes over the page's life (play: the first four, as before once its budget was
  full; demonstrator and quiz: all), never in list mode (phones; the demonstrator already skipped it); time-based release applies to warm-booted
  panes too (baseline = last touch or mount) — an idle play tab now also drops its warm shells after two minutes; the most recently opened pane
  (initially the first) is exempt from time-based release while the overview shows (the demonstrator's rule, now for everyone); closing a pane
  counts as a touch. Budget order: never-touched panes first, then least recently touched.
- **List mode**: the veil of a section exists only within the window radius (148 blurred layers → 2–3 on a phone); a mouse or keyboard reveal
  hides that section's veil; touch never reveals.
- **Chrome placement fix**: both landings wrap their `Navbar` in `div.pointer-events-none.absolute.inset-x-0.top-0.z-40` (the navbar keeps
  `position: relative !important`); the demonstrator's credits wrap (`flex-wrap`) instead of colliding at 375 px.
- **Labels**: play keeps its bundles; the demonstrator's landing and card strings are now en + de bundles (German via the German lock).
- **Grid rest** (§17): a single strip transform cannot put C × R pages exactly into uneven cells without distorting them, so the grid rest
  places every page itself: a camera rectangle (the part of the rest layout shown full size) glides between the whole view and a page's rest
  cell, and each page gets `transform` + `clip-path: inset` covering `viewRect(rest, camera)` (uniform scale, top-centre anchor). Two style
  writes per page per frame, no layout, percent only; the panorama path (one strip transform) is untouched, so play's 148 pages pay nothing.
  All pages boot at mount (they are the visible backdrop); the overlay exposes the track lists as `--layered-columns`/`--layered-rows`.

## 3. Defects of §10.9 — status

| # | Defect | Status (evidence) |
|---|---|---|
| 1 | Brand navbar off-screen | fixed — rig: navbar `top 0`, visible, `aria-label` "semio Play"/"Entwerfen mit Bestand" (`*-1440-rest.jpg`) |
| 2 | Play focus reveal never concealed | fixed — rig `blurred`: veil `whole` after focus leaves; component test "reveals on keyboard focus and conceals…" |
| 3 | Demonstrator: no keyboard reveal | fixed — rig: Tab → generator revealed, Tab → koordinator revealed (`demonstrator-1440-keyboard-focus-reveal.jpg`) |
| 4 | Focus lost on open / not restored | fixed — rig: opened region focused; after Escape focus on the card button (`*-closed-focus-on-card.jpg`) |
| 5 | Hole from the target offset | fixed — rig mid-glide veils are `hole` polygons of the current offset; component test "glides … cuts the hole from it every frame" |
| 6 | Landing re-rendered every frame | fixed — imperative paint; memoized pane views and card hosts (test "re-renders only the cards whose reveal changed") |
| 7 | Four veil rectangles, 745 blur layers, 148 placeholders | fixed — 1 veil; play at rest 445 backdrop-filter elements (148 cards × 3 glass cells + 1), 8 placeholders, 4 394 DOM nodes (was 9 828) |
| 8 | No reduced motion | fixed — `reducedMotion` (auto/always/never): snap, no pointer pan (test "snaps instead of gliding…") |
| 9 | Posters only for canvas apps | by design now: posters only via an explicit `capturePoster`; DOM-only pages never get one (test). Play/demonstrator still pass `capturePosterFromCanvases` (canvas regions only) — a DOM rasteriser would be the next step |
| 10 | Veil host without `data-level` | fixed — `data-level="dialog"` on the veil (strip and every list section) |

## 4. Commands and results

| Command | Result |
|---|---|
| `bun ./📜️script.ts test quick layered` (ui-react package) | **165 passed** (geometry unit 140 incl. schema check and the two oracles; component 25) |
| `python layered_overview_mutations.py` (ticket) | **19/19 mutants killed**, baseline green (panorama round). Grid-rest round: the script's restore write failed once on a Windows file lock (the mutant stayed in `🥞️layered-overview-geometry/🟦️.ts` for about a minute and was reverted by hand); the 7 grid mutants (letterbox instead of cover, tracks ignored, pointer pans in grid, boot on demand, zoom always snaps, hole only on the next frame, no zoom-out on leave) were then applied and reverted one at a time with the editor: **7/7 killed**; files verified clean, 165 green afterwards |
| `bun ./📜️script.ts test quick` (ui-react package) | 1 000 passed, 22 failed — the same 22 pre-existing failures the react report lists (Tree, Panel, Mode, Window, ShellScope, Layout, tokens); 0 failures in the layered tests |
| `bun ./📜️script.ts test` (`🏢️semio-tech/🎡️play`) | 44 passed, 3 failed — pre-existing (`play pane example defaults`, `playActivationLanes`, `playDevStaticDirMounts`: stale registry/graph); the 28 former grid/idle cases now live in the ui fixture |
| `bun ./📜️script.ts test` (`♻️mit-bestand/🧺️demonstrator`) | 19 passed; 1 file fails pre-existing (`🧪️demonstratorpanebranding`: `@semio-tech/flow-core` wasm not built) |
| `tsc --noEmit -p tsconfig.json` (ui-react) | 51 errors, **0 in touched files** (all pre-existing: `🧬️contract/🧵️retained/…`, `🛂️manifest`, `🎭️actor`, Panel test, Tree) |
| `tsc -p 🗑️generated/layered-final/tsconfig.landings.json` (play + demonstrator files) | 0 errors in touched files; 1 pre-existing in `🧪️playpanecoverage` (untouched), the rest inside the OS renderer graph |
| `bun ./📜️script.ts verify taxonomy report --scope <each new dir>` | `clean=true` for all four new directories (the ui scope as a whole has 419 pre-existing errors) |
| `bun layered_overview_rig.ts` (ticket) | real landings, faked shell; 0 significant console errors (only Vite's first-load `504 Outdated Optimize Dep`), numbers above; `rig-report.json` |
| `bun ./📜️script.ts typecheck` / `lint` (ui-react) | cannot run on native Windows (bun executes `node_modules/.bin/tsc`/`eslint` shell shims) — pre-existing tooling issue; tsc run directly instead |

Not run: the Playwright acceptance suites of play/demonstrator (they need the full wasm dev build; hooks updated to `data-layered-*`), axe
(not installed — skipped per instructions; the DOM tests assert roles, names, `inert`, `aria-hidden`, focus instead).

## 5. Runtime evidence (rig, `🗑️generated/layered-final/`)

| Screenshot | Shows |
|---|---|
| `demonstrator-1440-first-visit-intro.jpg`, `play-1440-first-visit-intro.jpg` | introduction over the overview |
| `demonstrator-1440-rest.jpg`, `play-1440-rest.jpg` | one veil, cards, **visible navbar** at the top, credits (demonstrator) |
| `*-1440-pointer-pan.jpg` | pointer pan panorama |
| `demonstrator-1440-hover-aggregator-clear.jpg`, `…-statik-clear.jpg`, `play-1440-hover-flow-clear.jpg`, `…-puzzle3d-clear.jpg` | hover: page clear, card lifted, chrome hidden; strip `-50% 0%` / `-75% -50%` / `-23.0769% 0%` / `-84.6154% 0%` |
| `*-1440-keyboard-focus-reveal.jpg` | Tab reveals the focused card's page |
| `*-1440-opened.jpg` | opened page, hash `#koordinator` / `#generation3d`, Overview/Übersicht button, focus on the region |
| `*-1440-closed-focus-on-card.jpg` | after Escape: focus ring on the card, page revealed |
| `demonstrator-1440-deep-link-statik.jpg`, `play-1440-deep-link-draw.jpg` | deep link: only that page mounted, no cards/veil, no intro |
| `*-375-list.jpg`, `*-375-list-second.jpg` | touch phone list mode (2–3 veils near the view, current + next live), demonstrator credits wrapped |
| `grid-1440-rest.jpg`, `grid-1440-zooming.jpg`, `grid-1440-zoomed-board.jpg`, `grid-1440-back.jpg` | grid rest (`GridRest` story, tracks 1 : 1.5 : 1 × 1 : 1.4 : 1, 1440 × 576 container): nine live pages under the glass, each card centred on its page's cell; hovering Leaderboard zooms its page (hole = its current rectangle at 160 ms), full size and glass hidden after 500 ms; leaving zooms back to the identical rest |

Measured grid rest (real browser): every pane `live`; the board page's visible rectangle is its cell (411, 169, 617 × 237) and its card centre (720, 288) sits on the cell centre (719.5, 287.6) — likewise for all nine; zoomed: board 1440 × 576 at (0, 0), neighbours outside; back: transforms identical to rest; 286 DOM nodes, 28 backdrop-filter elements, 0 console errors.

Measured (1440 × 900): demonstrator 362 DOM nodes, 25 backdrop-filter elements, live ≤ 4 of 8 during the walk; play 4 394 nodes, 8
placeholders, 445 backdrop-filter elements, live ≤ 4 in every settled state (one transient 5th mount between a touch and the budget release).

## 6. Open issues

- Play still renders 148 cards (each three glass cells) — the dominant cost now; a card grid windowed like the panes would need a scroll model
  for the card overlay first.
- `capturePosterFromCanvases` ignores DOM regions of mixed panes (as before); a DOM rasteriser would need an external library behind an interface.
- The play/demonstrator Playwright acceptance suites were not run (wasm build); the rig covers the landing mechanics with the real landing code.
- After dismissing `UIIntroduction`, the next Tab starts after the removed dialog (browser focus-navigation starting point), so on the
  demonstrator it lands on the footer credits first; the introduction could return focus to the first card.
- The quiz home passes fresh `panes`/`renderCard` every render (fine functionally; see §0 for the cheap path).
