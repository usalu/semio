FINAL

# 🍎️ Mac Lane site-mac: Takeover of s6 (suites) and s7 (overview) from g1-m04

State written 2026-11-25 22:25. Everything below is measured in a log that ran to its end. `R` = `/Users/ueli/Documents/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket. Scripts: `U/🧰️mac/`. Logs, JSON and screenshots: `U/🗑️generated/mac/` (left for the coordinator to read and delete with the ticket). Browser: the installed Google Chrome 157 through Playwright's `chrome` channel (this Mac has no Playwright Chromium and none was downloaded). Only runs of this session are counted. No sub-agents.

## 0. Where it stands

- **Build, type-check, suites and smoke run end to end on this Mac.** Release build twice (exit 0; 24 min 24 s, then 14 min 57 s, 375 asset modules, 97 MB), smoke on both builds 62 of 62 checks in English and German on desktop (1440 × 900) and phone (390 × 844 and 320 × 640) with no console, page or network issue, `label-fit` 8 of 8, the new `idle-overview` browser case 1 of 1.
- **The idle desktop overview is bounded** (job 3): renderer 1.0 to 1.2 GB median and 1.25 GB maximum from 60 s on, 13 to 14 live previews weighing at most 7 985 of 8 000 points, DOM nodes at most 270 000 (Linux before this lane: 2 to 3 GB, still growing; section 3).
- **Tag titles are bilingual** (job 5): schema, 375 catalog documents and 4 fixtures, React target, 8 tests (section 5). The German page shows German tags.
- **Suites green:** React target 44 case files, 385 of 385 tests (368 in the final run of 43 files, 17 in `catalog-loading`), site 16 of 16 plus `model-split` 2 of 2, `label-fit` 8 of 8 and `idle-overview` 1 of 1, engine `tag-titles` 4 of 4. The four reds of the first run are explained and repaired (section 2); the `catalog-loading` case that failed on Linux passes (17 of 17, 20 minutes).
- **Open:** (1) the site type-check has 4 errors that are not staffage's: `R/🧰️framework/🔨️modules/🎠️kernel/🛂️descriptor-admission/🟦️.ts`, another lane's uncommitted edit of 16:55, reached through the site project (section 1; request filed); (2) `model-split`'s build case needs a quiet Mac or a raised runner limit (section 2); (3) section 6.

## 0.1 Capacity: how the work was admitted

- The first 70 minutes nothing of this chat's agent session was admitted: the session `a22ahslj165` (all fixers of the chat) held 6 500 MB of reservations against its session budget of 33.3 % (5 461 MB), so every request of it waited with "this agent session already holds its share" (the long matrix chains `MA/worker.sh` and others hold 800 to 2 500 MB for hours). Filed in `U/requests.md` ("site-mac — the capacity queue admits nothing…").
- Then I opened an own session with the documented command `repo session open --agent=site-mac --export` (README "Agent sessions": one identity per agent, its share of the queue derives from it) and ran every request of this lane under `SEMIO_AGENT_SESSION=a3fba9d52db0.apzha18p8c`. No limit was lifted: memory floor, pressure and first-come order held; at most two requests were in the queue at any time.
- Measured peaks (`/usr/bin/time -l`): release build RSS 2.07 GB and 2.68 GB, peak footprint 4.7 GB and 5.0 GB (reserved 3 500 MB: the footprint is above it, the resident set below); `catalog-loading` 1 889 MB; the React suite chain about 1.3 GB; a Chrome smoke or idle probe about 1.6 GB of all Chrome processes.
- Two of my waiting requests were ended with SIGTERM at 21:04 by something outside this lane (cause unknown) and one gave up after 1 801 s because the host itself held the memory ("command needs at least 2500 MB … only 1590 MB are free"); each was queued again, smaller where it helped.

## 1. Type-check

| Project | Command (via `U/🧰️mac/typecheck.sh`, tsc 5.9, `--noEmit`, under a grant) | Result |
| --- | --- | --- |
| React target (`S/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`) | `typecheck.sh react`, then again after all my edits (`tc2`) | exit 0, 0 errors, both times |
| Site (`S/🌐️site/📦️packages/🟦️typescript/tsconfig.json`) | `typecheck.sh site`, then again after all my edits (`tc2`) | exit 2, 4 errors both times, all the same four in `R/🧰️framework/🔨️modules/🎠️kernel/🛂️descriptor-admission/🟦️.ts` lines 76, 134, 136 (`PluginManifest` against `ManifestWithActionSets`: the file was edited at 16:55 together with `🛂️manifest`; not staffage) |

- The two errors of the engine file `🗂️catalog/🧊️volume/🟦️.ts` line 254 that the brief expected are gone: the engine owner repaired them before I ran, so nothing was fixed by me there and nothing is noted in `U/requests.md` for it.
- The 4 framework errors are another lane's work in progress; I did not edit the file. Noted in `U/requests.md`.

## 2. Suites

Runner: `U/🧰️mac/chain-suites.sh`, `chain-final.sh`, `vitest-one.sh`: Vitest of the package's own configuration, one worker, inside one capacity grant, the JSON report read for the counts. The level is `quick` for the React target and `long` for the site.

### React target (`S/🎯️targets/⚛️react/🧪️tests`, 44 case files with the new `tag-titles`)

| Run | Files | Passed | Failed | Seconds |
| --- | --- | --- | --- | --- |
| first, all case files but `catalog-loading` | 43 | 364 | 4 | 548 |
| `fix1`: the four failing files again, test limit raised to 900 s | 4 | 38 | 1 | 328 |
| `fix2`: `preview-liveness`, `tag-titles` after the repairs | 2 | 9 | 0 | 6 |
| `catalog-loading` alone, budget `SEMIO_TEST_BUDGET_MS=3300000`, `--testTimeout=3300000` | 1 | 17 | 0 | 1 199 |
| final, all case files but `catalog-loading`, test limit 900 s (`final1`) | 43 | 368 | 0 | 503 |

The four reds of the first run and their causes:

| Case | Cause | Done |
| --- | --- | --- |
| `model-downloads` "model JSON and the ZIP bundle holds every pose…" | timed out at its limit of 180 s (166 s on the idle Linux worker); host limit, not a defect | passes with `--testTimeout=900000` (in `fix1`); the case file is unchanged |
| `preview-liveness` "ends the drawing worker once every still figure has arrived…" | two defects of s7's code, found by this very case: (a) the live set kept its `IntersectionObserver` after its last seat left, so a later document reused a stale observer and never saw a card scroll near; (b) a still preview that woke up did not draw again, so it never took its lease back and the drawing worker was not started | (a) `PreviewLiveSet` disconnects its observers when the last seat leaves; (b) a waking figure forgets its last request; `🖼️previews/🚦️live/🟦️.ts`; the case passes 6 of 6; the `[DEBUG]` line s7 left in the case is removed |
| `tag-titles` "shows the same titles beside the title of the page" | my test: the preview page (`opened: false`) shows no tags | the case opens the page as the other cases do |
| `tag-titles` "finds an asset by a tag title of either language or by its slug" | my search text had dropped the hyphenated slug | `🔎️search/🟦️.ts` keeps the slug and both titles |

### Site (`S/🌐️site/🧪️tests`, level `long`)

| Case file | Result |
| --- | --- |
| `host-document` | 5 of 5 (first and final run) |
| `stale-chunk` | 7 of 7 (first and final run) |
| `flow-worker` | 2 of 2 (first and final run; the case `flow-worker` is s1's, red before its repair of the release build) |
| `catalog-release` | 2 of 2 (first and final run) |
| `model-split` | classification 1 of 1 in every run. The release build case: in the first run it timed out at its own 900 s while a second release build and the React suite ran beside it; run alone with `--testTimeout=3000000` it passes: 2 of 2 in 879 s, peak 2 934 MB (it builds the whole site a second time; the case file is unchanged). Also the markers of the case on the finished builds: `🧰️mac/split-markers.mjs dist/release` finds all 7 (WebGL2 shader, GLB variants extension, FBX header, stage, volume schema, hidden-surface text, model schema) in lazy chunks and none in the entry |
| `label-fit` (Chrome, release build) | 8 of 8 (320, 768, 1024, 1440 px, English and German); screenshots in `U/🗑️generated/mac/bc1/label-fit/` |
| `idle-overview` (new, Chrome, release build, 60 s idle) | 1 of 1 |

## 3. s7: the renderer memory of the idle desktop overview

What s7 left (written 13:07 to 13:12 on g1-m04, never run there): `🖼️previews/🚦️live` (`PreviewLiveSet`, `runFigure`, `PREVIEW_LIVE_BUDGET_POINTS` = 8 000 evaluated points), the previews of `🖼️previews`, the frieze of `🏠️overview`, the placed figure `📍️placed` (`preview` flag), `Drawing.points`, `statePoints` and `drawingBench` in `🧮️pipeline`, and the case `🚦️preview-liveness` (6 scenarios). Run for the first time here: it found the two defects of section 2 and is green now.

How it behaves, measured in Chrome on the release build with `U/🧰️mac/idle-probe.mjs` (sample every 4 s for 150 s on the English desktop overview, renderer resident memory from `ps`; the probe is the Linux one with `/proc` replaced by `ps`):

| Build | Renderer RSS median / max from 60 s | All Chrome RSS median / max | Live / still previews (150 s) | DOM nodes median / max | Script heap median |
| --- | --- | --- | --- | --- | --- |
| first release build | 987 / 1 275 MB | 1 305 / 1 606 MB | 14 / 54 | 157 000 / 269 000 | 57 MB |
| final release build (inside `bc1`) | 1 171 / 1 255 MB | 1 583 / 1 663 MB | 13 / 55 | 150 000 / 221 000 | 57 MB |
| g1-m04 before (s1's probe on the dev server, 150 to 200 s) | 2 000 to 3 000 / 3 200 MB | 3 000 to 3 700 MB | 15 pages mounted, no bound | 100 000 to 900 000 | 80 to 190 MB |

- The live previews hold their budget: 13 live previews weigh 7 976 to 7 985 of 8 000 points at every sample; none lies further than one row from the window.
- The pages that the layered overview warms behind the glass are bounded as before (5 pages at 150 s).
- Caveats: macOS resident memory leaves out compressed pages, so the numbers can be a little low when the machine is under pressure (swap was 0 for these runs); the Linux numbers are another host and another server (dev server); no run of the code before s7's change was made on this Mac. The proof of the bound is the weight (7 985 of 8 000) and the DOM count, which do not depend on the host.
- New browser case `S/🌐️site/🧪️tests/🪟️idle-overview/` (`🥒️.feature`, `🟦️.ts`; level `long`): on the built site or a running one, 40 s idle by default, it reads every 5 s the weight of the live previews against `PREVIEW_LIVE_BUDGET_POINTS`, that every live card lies within one row of the window, the DOM nodes (at most 400 000, Chrome's counter) and that previews are drawn and some rest. Written after the measurement, never seen red; its limits are tolerant on purpose (they catch the earlier state of 835 000 nodes, not a 20 % drift).
- Screenshots: `U/🗑️generated/mac/idle1/idle-48s.png` and `idle-120s.png` (the overview, drawn), `bc1/idle-overview/idle-overview.png`. The smoke's own overview and detail screenshots (`smoke1/*-1-overview.png`, `*-3-detail.png`) are taken before the first drawing arrives and show empty frames; the checks wait for the drawing, the screenshots do not.

## 4. Release smoke (desktop and phone, English and German)

`U/🧰️mac/smoke.mjs` (s1's, with the browser channel and two paths adapted), run on the release build with `U/🧰️mac/smoke.sh release <name>`; JSON `s1-smoke.json` in `U/🗑️generated/mac/smoke1/` (first build) and `bc1/smoke/` (final build).

| Context | Checks | Console, page, network issues |
| --- | --- | --- |
| English desktop 1440 × 900 | 17 of 17 | none |
| German desktop 1440 × 900 | 17 of 17 | none |
| English phone 390 × 844 | 14 of 14 | none |
| German phone 320 × 640 | 14 of 14 | none |

Same 62 checks on both builds: language gate, overview with 9 categories, humans, all 375 assets, detail with elevation, three viewpoints and sheet, 3D model on a canvas, PDF and the browser's print, downloads (SVG, DXF, DWG, GLB, FBX, WebM) with signature checks, on desktop one asset of each category, compare and playground. Looked at (opened): the overview in German, on the phone in English and after idling in English, the English and German detail with the elevation, the German phone detail, the first 15 cards of all assets, `label-fit` at 320 and 1440 px in German. Not opened: the three-view sheet, the thermal and force views, the compare and playground pages (their checks passed, their pictures were not looked at). Seen: German tags in German ("Mensch, erwachsen, Fußgänger, Pendler, Mantel, Tasche, Gangart"), no overlapping segment labels, phone page without sideways scroll.

## 5. Tag titles

- **Schema first.** `S/🧬️schema/🔣️.json`: new `$defs/Tag` (`id`: slug, `title`: `Text`; both required, nothing else allowed), `Asset.tags` is an array of `Tag`. `S/🧬️schema/🟦️.ts`: `Tag` type and `tags: readonly Tag[]`.
- **Catalog data.** 1 343 distinct tags of the 375 assets (and the 4 fixtures) got a title in English and German. English is the slug as words (a few with their own wording, e.g. "DHW"), German is handcrafted for each (`U/🧰️mac/tags/de-1.txt`, `de-2.txt`, `de-3.txt`, corrections in `fixes.txt`, merged into `vocabulary.json` by `build.mjs`). I read the contexts of the ambiguous ones in the catalog and corrected them (for example "walker" is a pedestrian, "strings" are string instruments, "percussion" the family, "marker" a felt pen) and removed every pair of tags that read alike on one asset in German ("gebraucht" twice on 56 assets: "used" stays "gebraucht", "second-hand" is "Second-Hand"). The rewrite touched only the `tags` field: `apply.mjs` (slug to `{ id, title }`) and `retitle.mjs` (later title corrections) replace the text of the first `tags` array in place, keep each file's own formatting (the 29 one-line documents stay one line), re-read the file immediately before the write and skip it when it changed meanwhile (0 retries, 0 conflicts), and check by parsing that nothing but the tags changed. 379 files written, in two passes.
- **React target.** `🔬️detail/📐️dimensions` and `🔬️detail/🟦️.tsx` show `tag.title[language]`; `🔎️search/🟦️.ts` searches the slug and both titles of every tag.
- **Tests.** `S/🧪️tests/🏷️tag-titles/` (engine; ajv judges the schema definition and every tag of every document; ids once per asset; an id reads the same in every asset; no two tags of one asset read alike; more than 60 % of the titles differ between the languages): 4 of 4 in the final run (3 of 3 before the last case was added). `S/🎯️targets/⚛️react/🧪️tests/🏷️tag-titles/` (facts in both languages, header, search by either title or slug): 3 of 3. `🔎️search-filter-sort` adjusted to the new shape: 14 of 14. Both new cases were written before the schema and code and failed for the missing definition; the first runs of the React case found two faults of its own (section 2).

## 6. Not done, not checked

- `model-split`'s build case needs more than its own 900 s when anything runs beside it on this Mac; it passed alone with a raised runner limit (section 2). The case file keeps its limit.
- The overview was measured at 1440 × 900 in Chrome 157 on macOS only; no Firefox, Safari or Windows. The phone is an emulated viewport, not a device.
- `repo run` for staffage was not used (it reads the whole project graph); the scripts of `U/🧰️mac/` call Vitest, tsc and Vite directly under `repo capacity run`.
- The release build is still not a single-file-system-neutral artefact check: `☀️17/…/check-release-artifact.mjs` (predates the compressed payload) was not run.
- Not deleted: `U/🗑️generated/mac` (logs, screenshots and downloaded files) for the coordinator.

## 7. Files

- Written or changed in `S`: `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`; 375 `🗂️catalog/*/*/🔣️.json` (field `tags` only) and 4 fixtures (`🧫️fixtures/{🦶️sparse-smooth-walk,👤️sample-figure,🔄️continuous-rotation,🚶️reference-walker}/🔣️.json`); `🎯️targets/⚛️react/🔨️modules/{🖼️previews/🚦️live/🟦️.ts,🔬️detail/🟦️.tsx,🔬️detail/📐️dimensions/🟦️.tsx,🔎️search/🟦️.ts}`; tests `🧪️tests/🏷️tag-titles/`, `🎯️targets/⚛️react/🧪️tests/{🏷️tag-titles,🚦️preview-liveness,🔎️search-filter-sort}/`, `🌐️site/🧪️tests/{🪟️idle-overview,🏷️label-fit}/` (label-fit: browser channel by variable).
- Tools and notes: `U/🧰️mac/*`, `U/site-mac.md`, `U/requests.md` (two entries).
