# Round 2 — staffage site/app

Dev server: native Vite on 127.0.0.1:5392 (`restart-dev.sh`, `TEACHING_ARCHITECTURE_STAFFAGE_WATCH=off`, cache `.cache/b/p/m/border-staffage`), killed. Probe: `staffage-probe-run.js` + `staffage-helpers.js` (wrap the shared `border-gap-probe.js`; split results into `own` / `card` (card-vs-card, framework) / `scrollEdge`). Probe version with overflow clipping used.

## What changed (staffage)
- `🪟️chrome`: new vocabulary on `SharedBorderRow`: `Bar` (row, stacks on narrow windows), `Column`, `Cell`, `Group`, `Section`, `SectionTitle`, `Note`, `asCells`; `Segmented`, `ButtonGroup`, `Select/Text/Number/Slider/CheckField`, `Notice` (`plain` inside cards), `ProgressNotice`, `PageFrame`, `SectionHeading` are cells/rows; `Tag` is tinted, never framed. `SharedBorderRow*` exported from `ui-react/chrome` (one line in the slim barrel).
- App shell: navbar search/segmented controls are cells of the navbar; footer is a compositor frame (`edges=top`); settings panel and language gate are columns of cells.
- Pages: browse filters/sort/viewpoint = one bar of cells, filter panel = groups; browse/collections grids use framework `CardGrid`/`CardGridCell` (strip cells `data-card-grid-cell`); collection remove button moved into the card footer; detail page = column of sections (facts as a cell list, tables flush, poses/clips as radio column, export formats as nested columns); viewer = two columns of cells (stage is a cell + `data-border-probe-art`), transport/inspector/legends columns; compare panes = columns in one bar; playground config/selection/share/palette = bars and columns (palette tiles as 2-cell rows).
- Level: panes behind the glass (not opened) are `data-border-probe-art` (inert blurred pages); swipe mode no longer gets the strip grid class; `.staffage-entry` fills like the course.
- Art (`data-border-probe-art`): frieze, viewer stage / 3D stage, compare figure boxes, playground surface, clip timeline, legend gradient bar, inert panes.
- Tests: `🧷️shared-borders` updated to the new frame markup; fields carry `aria-label` (range-span tests). Teaching suite (`vitest-teaching.config.ts`): 107/109 tests pass; 15 files fail to load on the pre-existing missing fixture, 2 tests fail on the pre-existing catalog count (unchanged).

## Probe table (count = shared probe total; own = staffage-owned remainder)
| view / state | 1440x900 | 375x812 |
|---|---|---|
| overview (root grid) | 0 | 2 (framework swipe band, see below) |
| all assets, top / mid / bottom | 0 / 0 / 1 card-vs-footer (framework) | 0 / 0 |
| all assets, filters panel open | 0 | 0 |
| category humans / furniture | 0 | 2 (framework swipe band) |
| detail humans (walking-commuter), scroll 0..end | 0 | 0 (5 scroll positions) |
| detail furniture (cafe-chair) | 0 | - |
| detail appliances (circulation-pump) | 0 | - |
| detail animals (domestic-cat), scroll 0..end, thermal+range, three-view sheet, 3D model | 0 | - |
| collections, top / mid | 0 | 0 |
| collections bottom | 4 card-vs-footer d=1.9 (framework) | - |
| compare | 0 | 0 |
| playground (empty, with figures and selection) | 0 | 0 |
| settings panel (probe root = panel) | 0 | 0 (floating over page: cross-layer pairs excluded) |

Not probed: language gate (only shown when no language resolves), asset/export error states, DE locale.

## For the framework agent
1. Swipe mode (375): first/last card sits a 28 px hint band below the navbar / above the footer (overview hero, category cards): pairs navbar||card d=30 and card||footer d=30; the band has no flush hint frame on the side without a neighbour.
2. `CardGrid` as `ul` (all assets) leaves one card-vs-card vertical pair (d=24.8) at 1440; and the last card row ends 1.9 px above the footer line (silhouette inset).
3. Probe note: elements of a floating dialog over a page are cross-layer pairs.
