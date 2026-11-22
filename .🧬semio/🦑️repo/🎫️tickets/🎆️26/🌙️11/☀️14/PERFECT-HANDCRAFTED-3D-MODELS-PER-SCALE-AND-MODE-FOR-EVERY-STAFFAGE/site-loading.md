# Site loading: one asset per page, checks off the serving thread, real loading states

S = `🎓️teaching/🏛️architecture/🧍️staffage`, R = `S/🎯️targets/⚛️react`. Written 22 Nov 2026, about 03:30 CET.

The previous agent on this task was stopped by a rate limit and left its code in the shared tree without a note. I read all of it, kept its design where it was sound, changed it in three places (findings no longer hide the viewer, the overview no longer loads categories, a failed manifest no longer throws) and verified the whole in a browser.

## What was wrong

1. **A page loaded far more than its asset.** The asset page is drawn on top of the category grid and the overview, and both kept loading while hidden: the grid loaded its whole category, and every overview card loaded its whole category after a staggered timer. Opening `#/animals/domestic-cat` therefore pulled asset modules from every category.
2. **The check blocked serving.** `R/🌐️vite/🟦️.ts` ran `validateAsset` (about 20 s per asset) inside `load`, on the one thread that serves modules. A dozen modules meant minutes of "Loading…".
3. **A loading asset was called planned.** The asset page rendered "Planned, not yet authored" for every row without an asset, which includes rows that are still loading.
4. **Findings hid the model (found during verification).** `humans/standing-adult` currently has 466 findings from the tightened `volumes-feature` rule. The page showed "Unavailable: failed validation" in place of the viewer, so the model the findings are about could not be looked at.
5. **A manifest that could not be read threw (found by the test run).** `loadAsset` fell through to `loadCategory`, which read `manifest.categories` of `null`: two unhandled rejections in `📖️detail-sections`.

## What changed

### Serving and checking (`R/🌐️vite`)

- `🩺️findings/🟦️.ts` (`StaffageFindings`) runs checks on worker threads (`🩺️findings/🧵️worker/🟦️.ts`). One check per revision however many readers ask, progress to every reader, results kept per revision, and a check nobody waits for is taken off the queue or its thread is ended. Default: a quarter of the processors, at most four threads; idle threads end after 30 s.
- The development server answers an asset module at once. The module carries the address of its findings (`<base>@staffage-findings/<sha256>`); that address streams newline-delimited JSON: progress lines (the six parts of `validateAsset`), then the findings, or `stale` when the file changed meanwhile. Closing the stream withdraws the check.
- A build and a test still wait for the findings and carry them in the module; a release build rejects a module with findings, naming the file and each finding in English and German. Validation stays in the build and test pipeline and no longer blocks the development server.
- A changed `.ts` source ends the checking threads and forgets kept findings, so a rule change is picked up without restarting the server.
- `validateAsset` reports progress after each of its six parts (`S/🔨️modules/🗂️catalog/🟦️.ts`, `AssetContext.progress`).

### Loading (`R/🔨️modules`)

- `📚️catalog/🟦️.ts`: `CatalogStore.loadAsset` loads one asset; `streamedFindings` reads the stream, at most two at once so checks never take the connections assets load over. A row that is offered while its check runs carries `check` (done, total, label). `loadAsset` and `loadCategory` return when the manifest could not be read.
- `🔬️detail/🟦️.tsx`: the opened asset page asks for its one asset and withdraws the request when the reader leaves or cancels.
- `🗃️browse/🟦️.tsx`: a category grid loads only while it is the page the reader is on, not behind an opened asset.
- `🏠️overview/🟦️.tsx`: cards load only while the overview is the visible page. A category card takes the up to four assets its frieze shows from the manifest and asks for those alone; the hero card asks for the featured assets alone. No card asks for a category any more.

### States on the asset page

- Loading: a live region "Loading {title}…" / "{title} wird geladen …" with Cancel.
- Checking: the viewer is already there; a notice says "Checking this asset… Part 5 of 6: Poses and clips checked."
- Findings that arrive from the development server's check are listed above the viewer ("Findings of the check: 466", the first five, all of them on request) and the asset stays shown. Rows carry them as `Row.findings`. An asset whose module already came with findings (build, test) is still unavailable, and a release cannot contain one.
- "Planned, not yet authored" only for a row whose document does not exist.
- All new text is in `🌐️i18n/🔠️english/🧱️core.ts` and `🌐️i18n/🔡️german/🧱️core.ts` (`state.assetLoading.*`, `state.checking.*`, `state.findings.*`).

## How it was verified

Browser: the desktop app's browser pane on the native dev server (`staffage-site-native`, port 6072), 02:00 to 02:45 CET, while other agents were editing catalog assets and react sources. The server is stopped again.

- **One module.** After a reload of `#/animals/domestic-cat` the network list held exactly one `staffage-asset` module (the cat) and one `@staffage-findings` stream. On `#/humans/standing-adult`, reached from the cat page, the list held the cat and the adult and nothing else.
- **Time to content.** Title after about 1.5 s, 3D stage ready 5 to 7 s after a full reload (cat 5.2 s on phone, adult 6.9 s on desktop, car 5.1 s). That is the unbundled dev graph (about 700 module requests); it was minutes before. Switching mode or scale afterwards settled within the 0.5 s my probe can resolve.
- **States seen on screen.** "Loading City Car…" with Cancel; "Checking this asset… Part 5 of 6" on the cat right after an author changed it; "Findings of the check: 466" above a rendering adult. "Planned, not yet authored" never appeared for the three assets.
- **Every scale and mode, 3D viewer.** For `animals/domestic-cat`, `humans/standing-adult` and `vehicles/city-car`: 4 scales × 7 modes × 2 widths (1440×900 and 375×812) = 168 states. Each state was reached by its address (`?vp=model&view=…&scale=…`); the stage had to report `data-state="ready"` with the right scale and mode pressed and no alert; the WebGL canvas was then copied into a contact sheet. I looked at all six contact sheets, and at seven direct screenshots of the cat at 1:50 plus single screenshots of the adult and the cat page on desktop and phone. Every cell shows the model on the right ground colour, with visibly different geometry per scale. No cell was blank, no stage alert, and no uncaught error or `console.error` was recorded on a freshly loaded page.
- **Overview after the per-asset change** (12:10 CET, fresh server): all nine category friezes and the hero frieze were drawn 18 s after a cold server start; before the change four category friezes were still empty after 40 s. `vehicles/city-car` in thermal 1:200 then rendered with its (new) findings listed above the viewer.
- One artefact to know: errors "useArchive needs an ArchiveProvider" appeared in the console while I was editing sources under the running server (hot replacement of the context module). They did not recur after a reload.

## Tests

Run natively in each package folder: `bun <repo>/node_modules/vitest/vitest.mjs run --configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts --maxWorkers=1`.

- React target, first full run (03:15): 32 of 33 files and 321 of 322 tests passed, with two unhandled rejections (defect 5 above, fixed and pinned afterwards). The run after that fix is under "Final run".
- The one failing test is `📚️catalog-loading` › "loads the authored archive as one ready row per manifest entry with an empty manifest check": 62 of 152 catalog assets currently have findings (tightened `volumes-feature`; authors are working through them). It is a statement about the catalog, not about loading, and I left it as it is.

Tests that pin the behaviour:

- `R/🧪️tests/🗂️grid-scroll`: "requests only the opened asset on a desktop, and its category once the reader goes up" (an asset page loads one asset module); "asks on the overview for the assets its cards show and for no category document"; loading and planned wording in English and German; the viewer stays beside arriving findings.
- `R/🧪️tests/📚️catalog-loading`: rows while and after a later check, withdrawn checks, the stream reader, at most two streams, a manifest that cannot be read. The decode test no longer depends on whether `city-bicycle` currently has findings.
- `R/🧪️tests/🩺️asset-findings`: checks off the serving thread, one per revision, cancellation, the server answering before its findings, the build waiting for them. Its findings are compared with `validateAsset` on the same document.

## What remains

1. **The dev server stalls when the repository is being written heavily.** At 02:15 the server did not answer for over 30 s (index and asset module both timed out, 70 % CPU). A process sample showed its main thread entirely inside the file watcher's event callback (a loop over watcher listeners with one path comparison each, per file event). With many machines replicating into the worktree the event rate is high and the watcher is the bottleneck, not the asset plugin. `TEACHING_ARCHITECTURE_STAFFAGE_WATCH=off` avoids it but then edits are not picked up. A clean fix is a watcher restricted to the paths the site serves; not done.
2. **A changed asset file reloads the page.** The asset module accepts its own update by reloading, and it does so on any open page that loaded that asset earlier in the session, even when it now shows another one. With authors editing the cat and the car every minute or two, the page reloaded under me repeatedly. Replacing the document in the catalog store and re-rendering in place would keep camera and scroll position.
3. **The overview still needs the documents of the figures it draws** (up to four per category card plus the featured ones, about 40 instead of 152). Needing none would take previews drawn by the build.
4. **Two readers of one asset start two findings streams.** The overview's hero and a category card can ask for the same asset; the server runs one check, the browser reads it twice. Harmless, but `loadAsset` could share the request.
5. **Off-screen stages do not draw.** On the phone the stage sits below the findings notice; until it scrolls into view the canvas is empty by design. Worth knowing when capturing screenshots.
6. Observations for the model reviewers, not loading defects: the two line-drawing modes show only a few strokes in the 3D view for all three assets; the adult's thermal view has a cone above the head.

## Final run

- React target (03:25 CET, after the manifest fix): 32 of 33 files, 322 of 323 tests passed, no unhandled errors. The one failure is the archive-validity test named above.
- Site (12:09 CET): 4 of 4 files, 15 tests passed, 1 skipped.
- A mistake of mine to know about: my first site run started from the wrong folder and ran the whole `S/🧪️tests` library suite instead, for about eight hours on the shared host (323 failures there, almost all catalog rules on assets under rework: `🧊️model-catalog` 144, `🧊️catalog-silhouette` 112). I did not analyse those; they are not site tests.
- The dev server is stopped; `T/🗑️generated/site/` is deleted.
