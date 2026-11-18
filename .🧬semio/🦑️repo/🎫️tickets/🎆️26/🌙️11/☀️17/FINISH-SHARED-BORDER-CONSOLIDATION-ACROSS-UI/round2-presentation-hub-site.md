# Round 2 — presentation, hub admin, semio-tech play

Probe: `border-gap-probe.js` (`count` = number of violating pairs). Servers started natively (ports 5394/5395/5397/5398/5399, cache dirs `.cache/b/p/m/border-*`), probed in an own browser tab at 1440x900 and 375x812, then killed. Dev-server mechanics (see "How the views were reached") are ticket-local; two vite configs got a real import fix.

## Result

| App | 1440x900 | 375x812 |
| --- | --- | --- |
| Presentation (33. Projektetage deck, 34 slides, every state) | 0 | 0 |
| Hub admin (gate x2, overview, spaces, spaces expanded, create dialog, users, connections, documents, events, observability, language chooser) | 0 | 0 |
| semio-tech play: language choice | 0 | 0 |
| semio-tech play: card grid | 132, all inside framework card stacks (below) | swipe mode: 0 |

## Presentation (`🧰️framework/🛍️products/🎤️presentation/**`)

Host: mit-bestand Präsentation `33.projektetage` (the presentation product has no vite app of its own; Reveal chrome with the product's selection/enlarge/pager/handles). Sweep script `round2-presentation-sweep.js` visits every slide with Reveal's own navigation and, per disposition kind on the slide, probes: rest, selected (resize handles + reset/enlarge group + pdf pager) and enlarged. 375px: Reveal switches to its scroll view; `round2-presentation-mobile.js` probes the whole scrolled page (every 600 px) and every disposition kind selected and enlarged.

| View / state | Before | After |
| --- | --- | --- |
| Slides at rest (all 34, incl. iframe slides 3/8, 3/9, 4/3, media slides) | 0 | 0 |
| Selected figure / text / json / iframe / markdown / pdf (handles + actions + pager) | pdf selected: 1 (pager 10 px above the south resize handle's own border) | 0 |
| Enlarged text on 0/2 (reset/enlarge group vs the slide-reset button, 31 px) | 1 | 0 |
| Enlarged markdown 3/9 (actions chip vs the table's header line, 39 px) | 1 | 0 |
| Mobile scroll view, rest (all 11 scroll offsets) | 1 (slide-reset button 0.45 px off the actions group, text selected) | 0 |
| Mobile selected / enlarged (34 dispositions) | 0 | 0 |

Fixes (`🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css`, `🎭️morph-views/🟦️.tsx`):
- Resize handles painted a hairline `border` around their filled dot, which sits on the selection edge next to the pager chip. The ring is now `box-shadow` (same look, no border run); the handle is a solid knob, not a bordered box.
- The slide-reset button and the selected element's own reset/enlarge group are two chips in the same corner. The slide-reset button is now hidden while any disposition of its slide is selected (`section:has(.presentation-interactive-disposition--selected) .presentation-interaction-slide-reset`); the selected element's own reset takes over.
- Markdown prose (a rendered table, i.e. content) is marked `data-border-probe-art=""`. Art list: `div.prose.presentation-markdown-prose` (slide 3/9 only). The figure screenshots, pdf canvases and iframes are images/canvases and paint no CSS border, so they needed no marking. The deck's slide art (PNG mockups of the web UI) paints no borders.
- Test: `🧪️tests/📄️pdf-and-figure-interaction` asserts both CSS rules. Presentation suite: 15 files, 140/140 pass (`🎚️config/🟦️.ts`).
- mit-bestand Präsentation vite config: only the `viteCacheDirectory` import (see below).

## Hub admin (`🌎️hub/🔨️modules/🛡️admin/**`)

Mock backend: ticket-local `vite-hub-admin.config.ts` wraps the real config and adds a middleware serving `/admin/api/*` (overview, observability from the repo fixture, spaces + members, users, connections, documents, events). `ADMIN_MOCK=unauthorized` and `ADMIN_MOCK=unreachable` give the two gate states. `round2-hub-walk.js` clicks every tab, expands a space and opens the create dialog; `round2-hub-chooser.tsx` mounts the language chooser (the page cannot show it with a supported browser language).

| View | Before (desktop) | After 1440 | After 375 |
| --- | --- | --- | --- |
| Gate: absent session (401) | not probed (new) | 0 | 0 |
| Gate: hub unreachable | not probed | 0 | 0 |
| Language chooser | not probed | 0 | 0 |
| Overview | 3 | 0 | 0 |
| Spaces | 8 | 0 | 0 |
| Spaces, row expanded (members panel, add-member row, invite row) | 14 | 0 | 0 |
| Spaces, create dialog open | 14 | 0 | 0 |
| Users | 8 | 0 | 0 |
| Connections | 2 | 0 | 0 |
| Documents | 4 | 0 | 0 |
| Events | 4 | 0 | 0 |
| Observability | 5 | 0 | 0 |

Root causes and fixes:
- Header: `border-b p-single` with an inset tab list and an inset locale select (tab 4 px, select 7 px above the header line). Replaced by the shared `Navbar` (`AdminApp`): tabs and the locale select are band items that stretch to the navbar line.
- Every page was `p-single gap-single` around title, table and buttons: floating bordered things 5-36 px under the header and from each other. New element `🧱️elements/📄️AdminPage/🟦️.tsx` (`AdminPage`, `AdminSection`, `AdminNote`, `ADMIN_ACTION_CELL`): a flush column of full-bleed bands; the head band is a `SharedBorderRow edges=["bottom"]` holding the title cell and the page actions (new space, refresh, load newer, space select, freshness) as cells, so everything starts on the navbar line and each band's bottom line is the next band's top line. `StatTiles` is a full-bleed band (bottom edge only). Overview rebuild/cancel/progress row, observability sections (heading band, note bands, tables), events/users/documents pages, connections list and the members panel (heading cell, members table, load-more cell, add row, invite row) all follow it.
- Table action cells held inset framed buttons 0.5-2 px from the row lines. Columns pass `ADMIN_ACTION_CELL` (`!px-0`, stretched inner flex, no gap) and the buttons sit in `SharedBorderRow framed={false}` groups: the table's own row lines are their top and bottom edges, dividers sit between the buttons.
- Tables are `h-auto` inside a page (`[&_[data-slot=scroll-area]]:!h-auto`), so the members panel follows the table instead of being pushed to the page bottom.
- Tests (`🧪️tests/🛡️admin/🟦️.tsx`): new "Flush pages" suite (navbar owns the bottom edge, no `header`, every page band is a frame with bottom edge only, no `p/gap-single` on the page, action cells hold groups not frames, no own frames in `tbody`). Hub admin suite via `vitest-hub-admin.config.ts`: 25/25 pass (3 files).

Known, not border: at 375 px the seven tabs overflow the navbar and clip (the locale select is pushed off); pre-existing, left as is.

## semio-tech play (`🏢️semio-tech/🎡️play/**`)

The play page needs wasm engine bundles that are not built here, so a ticket harness (`round2-play-harness.tsx`, `vite-play.config.ts`, stubs `play-wasm-stub.ts`, `play-runtime-stub.ts`) mounts the real `PlayLanding` with inert pane bodies. The 151 cards, labels, icons and taglines come from the real catalog JSON.

Refactor (needed to test the landing at all): the landing moved out of `🟦️.tsx` into `🏠️play-landing.tsx` (`PlayLanding({locale, renderPane})`, `playStorage`); `🟦️.tsx` keeps the composition root (host ports, `PlayShell` = the one `FrameworkOsShell` mount, locale boot, language choice). The `playpanecoverage` source law still sees exactly one `<FrameworkOsShell …/>` mount.

Product-level changes in the landing, radical rule:
- Card cell: `px-single` + `justify-center` removed; card `max-w-xs` -> `size-full`: the card fills its grid cell.
- Overlay: `items-center pb-double pt-[calc(var(--size-workbench)*1.5)]` -> `items-stretch` + inline `paddingTop: var(--navbar-height)`: the first card row starts on the navbar's bottom line, no page margin, no bottom padding.
- Swipe (touch) mode inset: `calc(var(--size-workbench) * 1.5)` -> `var(--navbar-height)`; the centred card no longer floats 37 px below the navbar. Probe: 375x812 swipe, 0.
- Test: `🧪️tests/🧪️playgrid` "landing grid" (no padding/centring/width cap, navbar-height padding and insets). Play suite: 8 files, 81/81 pass.

| View | Before | After |
| --- | --- | --- |
| Language choice (1440 / 375) | not probed | 0 / 0 |
| Landing, intro overlay open (1440) | 39 (cards vs the intro info box, which is a modal overlay by design, plus the pairs below) | not tracked: modal over cards, probe false positive |
| Landing, intro dismissed, 1440 | 13 (navbar vs card row 1.2 px, tall interlocked cards) | 132, all framework-level: see below |
| Landing, swipe mode 375 (intro dismissed) | 2 (navbar vs neighbour hint 9 px; navbar vs card 37 px) | 0 |

### Framework-level pairs for the card-grid owners (desktop landing, 132)

The cards now tile edge to edge (adjacent card boxes touch exactly: y 28.8/188.8/…, x 110.77 steps), but each `div[slot=play-pane-card-stack]` is a `data-window-silhouette` card whose silhouette SVG outlines its own box inset by ~1 px, so every adjacent pair paints two lines 2 px apart (horizontal and vertical neighbours alike, about 100 of the 132). The other pairs come from the stack interlock/lift (`transition-transform group-hover:-translate-y-0.5` wrapper and `useCardInterlock`-style offsets): single cards drawn 25.6 px higher (title row of the card above), pairs at 24-40 px. This needs the card owners' mechanism: neighbouring OverviewCard/CardStack silhouettes in a grid must share one line (silhouette outline on the cell box, or attach the shared sides), and the lift/interlock offset must not apply inside a grid of equal cells. Play only needs to keep `gap 0`, no cell padding and `size-full` cards, which it now does.

## How the views were reached (ticket-local)

- `repo run` is broken; vite natively with `SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=<repo>/.cache/b/p/m/border-<name>`.
- The real configs for the presentation host, hub admin and play imported `viteCacheDirectory` through the `🎮️playground` barrel, which pulls the workspace-payload `.cjs` (`Dynamic require of "node:path"`). Hub admin and mit-bestand Präsentation configs now import it from `⚡️caching/🌐️vite/📂️paths/🟦️.ts` (as the course and staffage configs already do); this makes their dev servers start. The play real config additionally needs `canonicalWasmBuildVitePlugin` and `viteCacheDirectory` from the `⚡️caching/🌐️vite` leaf modules (changed) and still cannot start without built wasm engines, hence the harness config.
- Other agents' edits to shared ui files trigger Vite page reloads that kill sweeps; the presentation wrapper `vite-presentation-frozen.config.ts` (watch and HMR off) avoids it. Vite then caches probe scripts by URL: use a fresh script file name per edit.
- Browser notes: `javascript_tool` treats a hash change as navigation and cuts the script; sweeps use Reveal key events instead. `resize_window` was reset to desktop and the tab closed.

## Files

Ticket: `round2-presentation-sweep.js`, `round2-presentation-mobile.js`, `round2-hub-walk.js`, `round2-hub-chooser.tsx`, `round2-play-harness.tsx`, `vite-presentation-frozen.config.ts`, `vite-hub-admin.config.ts`, `vite-play.config.ts`, `play-wasm-stub.ts`, `play-runtime-stub.ts`.

Repo: presentation `🎨️styles/🔲️interaction-handles-and-controls/🎨️.css`, `🔨️modules/🎭️morph-views/🟦️.tsx`, test `🧪️tests/📄️pdf-and-figure-interaction/🟦️.tsx`; mit-bestand `📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts`; hub admin `🏗️builder/🌐️vite/🟦️.ts`, `🧱️elements/{🛡️AdminApp,🏠️OverviewPage,🏛️SpacesPage,🙋️UsersPage,🔗️ConnectionsPage,📃️DocumentsPage,📰️EventsPage,📊️ObservabilityPage,📊️StatTiles}/🟦️.tsx`, new `🧱️elements/📄️AdminPage/🟦️.tsx`, test `🧪️tests/🛡️admin/🟦️.tsx`; play `🏗️builder/🌐️vite/🟦️.ts`, `🟦️.tsx`, new `🏠️play-landing.tsx`, `⚛️play-card.tsx`, test `🧪️tests/🧪️playgrid/🟦️.ts`.

Not run: full `repo test` for these projects (vitest run directly per package), mit-bestand demonstrator (not mine), browser pass of the play landing with real pane shells (needs built wasm).
