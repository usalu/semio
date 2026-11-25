FINAL

# 🏷️ Linux Lane s5-labels: Labels, Caption and Tags of the Detail Page

State written 2026-11-25. Host g1-m04. `R` = `/home/deck/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket. Scripts: `U/🧰️linux/s5/`. Logs and screenshots: `U/🗑️generated/linux-s5/` (logs carry the prefix `s5-`). Only runs of this session are counted.

## Where it stands

- **Labels, segments and the caption fit on the development server of the live tree:** the new browser case `label-fit` passes 8 of 8 (English and German at 320, 768, 1024 and 1440 px). Before the repair it failed 6 of 6 on the release build of 11:18 (320, 768 and 1440 px; 1024 px was added afterwards).
- **The same on a release build of 11:59 (exit 0, 12 min 16 s):** `label-fit` 8 of 8, and the s1 smoke for German desktop and both phones 45 of 45 (section 3).
- **Tags stay English on the German page and that is not repaired:** the catalog carries tags in one language only (section 4). No translation was invented.
- The root cause of the cut caption lies in the shared engine and is not repaired there; the React target now takes the wider text into the window of the drawing (section 2, D).

## 1. The test

`S/🌐️site/🧪️tests/🏷️label-fit/` (`🥒️.feature`, `🟦️.ts`), level `long`. It opens the walking commuter's page with the clip held at 0.5 s in Chromium (Playwright), on the built site (`dist/<SEMIO_BUILD_MODE>`) or on a running one (`TEACHING_ARCHITECTURE_STAFFAGE_URL`), once per language and width, and reads bounding boxes in the page:

| Finding | What is measured |
| --- | --- |
| `label-outside-control`, `label-clipped` | every text box of a button, segment, link or summary of the asset page and the navigation bar lies inside its control (0.5 px tolerance); the control does not scroll inside |
| `segments-overlap`, `segment-outside-choice` | no two segments of one choice overlap; every segment lies inside its choice |
| `control-outside-window`, `page-scrolls-sideways` | no control reaches past the window; the document is not wider than the window |
| `touch-target-low` | with a touch screen (320, 768, 1024 px) every button, segment, summary and select is at least 44 px high |
| `drawing-text-cut` | every text of the drawing lies inside the drawing; the caption states the held time |

Not measured by it: the text inside a native select, check box labels, other pages than the asset page, other assets than the walking commuter, other moments than 0.5 s, the 3D model's own tool bar, the three-view sheet, the light theme, the large text size.

Runner scripts: `🧰️linux/s5/label-fit.sh <name> [release|prototype|url]` (built site) and `🧰️linux/s5/label-fit-dev.sh <name>` (development server of the live tree, one capacity grant for both).

### Before (release build of 11:18, without the changes)

`label-fit.sh before release`: exit 1, 6 of 6 failed. Log `s5-before.log`, findings `before/<language>-<width>.json`.

| Case | Findings |
| --- | --- |
| German 1440 | 8 labels outside their segment by 5 to 23 px (dunkle Strichzeichnung, helle Strichzeichnung, Thermikansicht, Kraftansicht, Momentenansicht, Seitenansicht, Vorderansicht, Axonometrie); caption cut by 7.4 px; first event marker of the timeline 5.6 px outside the window |
| German 768 | caption cut by 7.3 px; navigation bar past the window: Hell 38 px, Dunkel 82 px, Anzeigeeinstellungen 126 px; event marker 13.2 px |
| German 320 | 11 labels outside their segment by 2 to 28 px (the eight above, Draufsicht, Drei Ansichten, Transparent); caption cut by 2.7 px; event marker 13.2 px |
| English 1440 | Axonometric view and By representation outside by 5 px; event marker 5.6 px |
| English 768 | Display settings 13.2 px past the window; event marker 13.2 px |
| English 320 | Axonometric view, By representation, Transparent outside by 2 to 10 px; event marker 13.2 px |

The English caption was not cut at 0.5 s in this run; it is cut at other moments (`🗑️generated/linux-s1/final-desktop-en/desktop-en-3-detail.png`, 1.12 s), because the digits of the site's face differ in width.

## 2. Changes

All in `S/🎯️targets/⚛️react` and `S/🌐️site/🧪️tests`. No catalog asset, manifest, allowlist, engine module or multiview file was edited.

A. Segments (`🔨️modules/🪟️chrome/🟦️.tsx`, `Segmented`): the row wraps (`SharedBorderRow wrap` of the framework, which keeps one hairline between lines); a segment no longer shrinks below its label and grows to fill its line; a label breaks inside its segment only when it is wider than the whole row. The rule `[data-staffage-phone] [role="radiogroup"] { flex-wrap: wrap }` in `🎨️.css` is gone, the component wraps at every width.

B. Navigation bar (`📦️packages/🟦️typescript/📚️app/🟦️.tsx`): the labels of "up" and of the four links show from 1280 px on (before: from 768 px); below that the icons stand alone, with name and tooltip.

C. Timeline of a clip (`🔨️modules/🔬️detail/🎞️clips/🟦️.tsx`, `🎨️.css`): the track is inset by half an event marker (`--staffage-marker`: 1.5 rem, the finger target on phone and tablet), so the marker of an event at 0 s lies inside the page; a click on the track is read against the track, not against the padded cell; the row of markers is as high as a marker.

D. Caption of the drawing: new `🔨️modules/🧮️pipeline/🔤️text/🟦️.ts` (`textOverhang`) and `🔤️text/⚛️hook/🟦️.tsx` (`useTextEpoch`). Cause: the engine reserves the Helvetica width of every text (`textWidthMm` × `TEXT_SLACK` 1.04 in `S/🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts`) but its SVG writer names `Anta` first (`FONT_STACK` in `S/🔨️modules/📤️export/🖼️svg/🟦️.ts`), and the site has that face, which is wider. The React target now measures every text of a scene in the face the page uses (2D canvas) and widens the window of the drawing by what the widest text takes beyond its room: `sceneExtent` and the new `sceneWindow` in `🧮️pipeline/🟦️.ts`, used by `🧍️figure/🟦️.tsx`, `🧩️element/🟦️.ts`, `🎛️viewer/🟦️.tsx` and `⚖️compare/🟦️.tsx`. A digit counts as the widest digit, so the window does not change while the clip runs. A face that finishes loading triggers a new measure.

E. Touch at desktop width (`📚️app/🟦️.tsx`): a coarse pointer takes the tablet device of the design system at every width (layout stays desktop). Found by the 1024 px case: 91 controls were 24 or 29 px high there.

F. Zoom readout of the viewer (`🎛️viewer/🟦️.tsx`): padding and no wrap; "Eingepasst" touched both edges of its cell.

G. Test double (`🧪️tests/🗂️grid-scroll/🟦️.tsx`): its `ResizeObserver` called back inside `observe()` and re-entered the framework's wrap hook without end (15 of 16 cases failed with a stack overflow after A). The double no longer delivers inside its own callback, as a browser's observer never does. The framework hook (`R/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow/🟦️.tsx`, `useSharedBorderWrap`) was not edited.

New tests: `S/🌐️site/🧪️tests/🏷️label-fit/` (red before, section 1) and `S/🎯️targets/⚛️react/🧪️tests/🔤️text-reach/` (`🥒️.feature`, `🟦️.tsx`; written after the module, so it was never seen red).

## 3. Runs after the changes

| Command | Exit | Result |
| --- | --- | --- |
| `label-fit-dev.sh dev-2` (development server, live tree) | 0 | 8 of 8; log `s5-dev-2.log` |
| `🧰️linux/s1/react-test.sh` with 22 case names (text-reach, figure-render, shared-borders, detail-sections, settings-focus, custom-element, range-span, viewpoints, viewpoint-pages, model-viewer-page, model-controls, playground-tabs, mount-archive, language-selection, compare-state, active-chrome, address-roundtrip, level-warming, grid-scroll, svg-templates, drawing-transport, downloads) | 1 | 24 files, 203 of 219 passed; 15 failures in `grid-scroll` (stack overflow, see G) and 1 in `model-viewer-page` ("switches to the 3D model with one patch"); log `s5-react-2.log` |
| `react-test.sh grid-scroll model-viewer-page` after G | 0 | 23 of 23; log `s5-react-3.log`. `model-viewer-page` was not changed between the two runs: it failed beside 22 other files and passed with one, as s1 saw too. Not explained |
| `🧰️linux/s1/build.sh release` | 0 | built in 12 min 16 s into `S/🌐️site/📦️packages/🟦️typescript/dist/release` (replaces the build of 11:18); log `s5-build-release.log` |
| `label-fit.sh after release` | 0 | 8 of 8, no finding in any of the 8 cases; log `s5-after.log` |
| `node 🧰️linux/s1/smoke.mjs <dist/release> 🗑️generated/linux-s5/smoke --only=desktop-de,phone-de,phone-en` | 0 | 45 of 45 checks, no console or network issue; log `s5-smoke.log` |

The 22 React cases were not run again as one set after G and F; only `grid-scroll` and `model-viewer-page` were.

Screenshots (all under `U/🗑️generated/linux-s5/`, one page shot and one shot of the viewer per case, `<language>-<width>.png` and `<language>-<width>-viewer.png`, with the findings in `<language>-<width>.json`):

- Before: `before/` (320, 768, 1440 px), and s1's `U/🗑️generated/linux-s1/final-desktop-de/desktop-de-6-model.png`, `final-desktop-en/desktop-en-3-detail.png`.
- After, release build: `after/` (320, 768, 1024, 1440 px), and `smoke/desktop-de-6-model.png`, `smoke/desktop-de-3-detail.png` for the same views as s1's.
- After, development server: `dev-2/`.

## 4. Tags

The tags under the title are `asset.tags`, an array of slugs (`readonly tags: readonly Slug[]` in `S/🧬️schema/🟦️.ts`, line 831). Neither the asset documents nor the manifest carry a title per tag in any language, so the German page can only show the slugs. Not changed: no translation table was added to the site, because that would be invented catalog content. To show tags in the reader's language the schema needs a bilingual title per tag (or a tag vocabulary in the catalog); that is catalog and schema work of the Mac coordinator.

## 5. Not checked

- No typecheck of the site or the React target; the full React suite and the site suite were not run (only the cases in section 3).
- English desktop smoke (`desktop-en`, with the 375-card check) was not run on the new build.
- Only the walking commuter's page, in the dark default view, held at 0.5 s. The caption at other moments rests on the widest-digit rule and its unit test, not on a browser measurement. Other assets, the thermal and force views with their legends, the three-view sheet, the comparison page and the custom element take the same code path and were not measured.
- Only Chromium 156 headless on Linux; a touch screen is Playwright's emulation. No Firefox, no Safari: a browser without `OffscreenCanvas` 2D (Safari before 16.4) measures nothing and still cuts the caption.
- Widths between the four measured ones; the navigation bar between 1024 and 1280 px was estimated, not measured.
- Exports (SVG, PDF, WebM, sheet) were not changed and not examined for the same cut: an exported SVG still names `Anta` first over a Helvetica-wide room.
- Inside a drawing a text that is wider than its room can still run into a neighbouring mark; only the window was widened. The repair that removes the cause is in the engine: measure the face the writer names, or name the face that is measured. Left to its owner.
- `useSharedBorderWrap` of the framework re-enters without end when a `ResizeObserver` calls back inside `observe()`; only test doubles do that. Not edited (outside this lane).
- `🗑️generated/linux-s5` (screenshots and logs) is left for the coordinator to read and delete with the ticket.
