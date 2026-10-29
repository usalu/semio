# Website of the Architectural Staffage Archive

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `R` = `S/🎯️targets/⚛️react`. Entry: `S/🌐️site/🟦️.ts` calls `mountArchive(root)` from `R/📦️packages/🟦️typescript/📚️app/🟦️.tsx`.

## Structure (`R/🔨️modules`)

| Module | Role |
|---|---|
| `🌐️i18n` | `Key`, `translator`, `resolveLanguage`; bundles in parts (`🧱️core`, `🎛️viewer`, `📖️detail`, `⚖️compare`, `🧪️playground`, `⭐️collections`) per language; the German bundle is typed `Record<keyof English, string>` |
| `💾️persistence` | event-sourced local store (`ArchiveStore`, `reduce`, `sanitize`), one versioned key `semio.staffage.v1`, cross-tab `storage` events, volatile fallback |
| `🗺️routing` | `Route`, hash parser and formatter, `ViewerState`, compare and playground parameters, `useAddress`, remembered filter queries |
| `🔎️search` | pure search (accent-folded, both languages, tags, summaries), filters, sort, `namedPoses` (hides `pose.key`) |
| `📚️catalog` | `CatalogStore`: manifest inline, one lazy chunk per category (`import.meta.glob`), `validateAsset` per asset with progress and cancel, `validateManifest` when all are loaded, `useAssetLoading` |
| `🧭️context` | language, translator, stores, motion decision, ambient clock |
| `🪟️chrome` | buttons, segmented radio groups, fields, sliders, notices (empty, loading, error, unsupported, unavailable, info), progress with cancel, archive card, page frame |
| `🥞️levels` | `Level`: the design system's `LayeredOverview`, shared pointer bus, breakpoints, measured size |
| `🏠️overview` | root grid: hero frieze, six category cards, collections, compare and playground cards |
| `🗃️browse` | asset card, toolbar, `BrowsePage` (all), `CategoryPage` (nested grid) |
| `🔬️detail` | `AssetPage` plus sections (dimensions, poses and clips with timeline, model inspector, provenance, export panel) |
| `🎛️viewer` | `Viewer`, `🗺️stage` (pan, zoom), `🧾️reading` (inspector, legends), `🛡️boundary` |
| `⚖️compare`, `🧪️playground`, `⭐️collections` | pages (forks A and B), see their files |
| `📍️placed`, `🖼️previews`, `⏯️transport` | figure on a true ground line, live previews and unavailable states, play/step/scrub |

Reference design reused as is: `LayeredOverview` and `OverviewCard` from `@semio-tech/ui-react/chrome` (glass veil, pointer parallax, hover reveal, swipe on phones, Escape to parent), `Navbar`, `useElementsSurfaceChrome` themes. The strip layout is the archive's own CSS (`R/🎨️.css`). Nesting: root level (hero, categories, tools) opens a category level (asset cards), which opens the asset page; the previous grid stays live and inert behind the glass.

## Routes and URL schema

`#/` overview · `#/all` · `#/<category>` · `#/<category>/<asset>` · `#/collections[/<id>]` · `#/compare` · `#/playground`. Only non-default state is written, in stable order.

- filters: `q cat an h root done sort dir`
- viewer: `view scale pose|clip t zoom pan bg th tq fq range pat dg air mrt rh wind sun` (time is written only when paused or scrubbed)
- compare: `p=asset~view~(pose|clip).id` up to four times, `scale`, `t`
- playground: `p=asset~x~(pose|clip).id~offset~m`, `scale view bg len`
- language and theme are the reader's and never in a link.

State classes: persisted local-only (preferences, favorites, saved collections, playground), ephemeral shared (address), ephemeral local-only (clock, hover, drag, inspector).

## Design decisions

- No default language: stored choice, else the first matching browser language (preselect, not stored), else a bilingual prompt and no `lang` on the document.
- Views align by `metadata.origin`: the viewer places every view in one window around the model origin (frame = union of the dark-line scene and the chosen view); zoom `fit` refits per view, a numeric zoom keeps the origin fixed. Compare panes share one frame; frieze and playground use `PlacedFigure` (origin at the ground point).
- Zoom is a screen transform (1 = print size, 96 dpi) separate from the drawing scale; pan is stored in paper mm.
- Analytic theme follows the site theme unless overridden (`th`); ground follows the view unless chosen.
- Progress and cancel: category loading (per card and all-asset loading), exports, playground SVG export.
- Honest states: not authored, invalid (with reasons), loading, category failed, no results, export unsupported or unavailable (DWG writer), persistence volatile.

## Verification

- `tsc -p tsconfig.json --noEmit` in `R/📦️packages/🟦️typescript`: 0 errors. Fast path used: `repo tools exec bun -- x tsc ...`; vitest: `repo tools exec bun -- /home/deck/semio/node_modules/vitest/vitest.mjs run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native`: 15 files, 99 tests green.
- My cases in `R/🧪️tests`: `🗺️address-roundtrip` (cross-checked with the WHATWG `URL` parser), `🔎️search-filter-sort`, `💾️favorites-persistence`, `🌐️language-selection` (mounted app, no default language, bundle parity), `📚️catalog-loading`, `🚀️mount-archive`; each with `🥒️.feature` except the smoke case.
- Browser: Vite from `repo tools exec bun -- x vite --mode prototype --config S/🌐️site/🏗️builder/🌐️vite/🟦️.ts --port 6071` with `SEMIO_VITE_CACHE_DIRECTORY=/home/deck/semio/.cache/b/p/m/vite/state/staffage` (the queued `repo run ...:dev` never started); Playwright with `PLAYWRIGHT_BROWSERS_PATH=<repo>/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, scripts `T/🧰️scripts/website/shoot.mjs`, `probe.mjs`, `probe2.mjs`, specs `spec-*.json`.
- Screenshots in `T/🗑️generated/website/shots` (looked at): `overview`, `overview-de-dark`, `overview-mobile`, `category-humans`, `detail-top`, `detail-de`, `detail-mobile`, `detail-scroll-{900,2900,6000}`, `final-view-{dark-line,dark-solid,light-line,light-solid,thermal,forces,moments}` (1:100), `final-detail-dark-forces`, `compare`, `compare-aligned`, `playground`, `playground-filled`, `collections`, `all`, `search-oak`, `settings2`.

## Audit fixes (T/audit-website.md)

High: settings panel moves focus in, closes on Escape/outside/focus-out, `aria-controls` only while open; manifest and category failures are shown with a retry (`CatalogProblem`, row status `failed`, `CatalogStore.retryCategory`); counts read "N authored · M planned" from the completion of each row.
Medium: stage is focusable with a described keyboard help, wheel zooms only with Ctrl/Meta and touch keeps vertical scroll; transport announces the time when paused or scrubbed and no longer sets `aria-pressed` on a text-changing button; the clip timeline has a native slider (keyboard) beside the event buttons; headings run h1, h2 (sections), h3; number fields keep what is typed and flag invalid entries; copy-markup says whether it worked; the playground's fallback link field is read-only.
Low: language buttons are named "English" and "Deutsch"; external links say they open in a new tab; no "Rust" or "WebAssembly" in user text (DWG says it needs the DWG encoder); no `String(error)` or raw ids in text (bones, regions, tracks, tags show titles); hero frieze and compare default come from the manifest collection named in `⚙️config` (`street-life`); compare card shows loading; the URL carries playback rate and loop (`rate`, `loop`), PDF sheet, frame rate and duration (`xs`, `xf`, `xd`) and the playground zoom (`pz`); figures without valued items are no longer focusable; contrast of the app's helper text was measured at 4.5 or more in the dark theme (the light-theme probe parses `oklch` colours wrongly, so it was judged by eye).

Content checks with all 48 assets (screenshots `re3-overview`, `re-animals`, `re2-vehicles`, `re-crane`, `re-oak-thermal`): frieze and category previews are laid out from the drawn extent of the default pose (not the nominal width), figures stand in place on a treadmill instead of drifting with root motion, tall items (the 49 m tower crane) are left out of category previews (height above 2.5 times the median and 4 m) but stay at true scale on its own page and in the frieze.

## Known gaps

- Viewer, asset page and overview have no jsdom test (they need the design system's icon catalog, which the assets build produces); they were checked in the browser only. Drag in the playground is not unit-tested.
- React logs "Maximum update depth exceeded" a few times (development build only) when a page opens with a non-default `view` in the address; it is raised by the clock's per-frame store updates, nothing loops, the page works. Not yet traced to its source.
- `repo run @teaching/staffage-site:dev` and `:build` were not exercised (capacity queue); no production build, so bundle size and chunking are unmeasured. Vite's file watcher stopped delivering changes once; restart the server if edits do not show.
- `fit` zoom refits when switching to an analytic view whose extent differs (legends, free-body inset); with a numeric zoom the model stays fixed. Force and moment views are small at fit because of their label columns.
- Hero walkers loop along a lane and fade at the ends, so only some of them are visible at any instant.
- Settings panel is a portal outside the layered stage; the mobile navigation links live in it (below 768 px the cards are the main route).
- Footer chips of cards inherit the chrome's footer font; text is larger than the chip text elsewhere.
- `🧍️figure` (component worker) was changed in one line (`tabIndex` only when the figure has valued items) and `📤️downloads` in the DWG reason text, both for the audit.
