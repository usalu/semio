# 📝️ sonnet-viewer report

Scope: `L/🎯️targets/⚛️react`, `L/🌐️site` (L = `🎓️teaching/🏛️architecture/🧍️staffage`; R = `L/🎯️targets/⚛️react/🔨️modules`).

## 1. Inventory (before)

| Place | Viewpoint before |
|---|---|
| Viewer, stage, `ViewerState`, `vp` URL param | selectable (3 segments), but no sheet, no unavailability message |
| `StaffageFigure`, `<staffage-figure>` | optional, silently elevation |
| `pipeline.depictionOptions` | `?? "elevation"` |
| Catalog previews, browse, overview, compare card | always elevation, no prop |
| `PlacedFigure`, frieze, playground | always elevation, `try/catch` returned `null`, mirror always applied |
| Compare | no viewpoint in pane, token or UI |
| Playground | no viewpoint in type, store, URL, export |
| Detail to compare link, export-panel snippet and lede | viewpoint dropped, raw ids in the lede |
| Downloads | file names without viewpoint |
| Site (`L/🌐️site`) | thin Vite shell around the archive app: no thumbnails, no viewpoint logic; nothing to change |

## 2. What was built

- `R/🔭️viewpoints/🟦️.ts` (pure): `ViewpointChoice`, names, `resolveViewpoint(s)`, `threeViewLayout` (aligned: plan above side elevation shares paper x, front elevation to the RIGHT shares paper y; documented why right: plan up = +z folds to front right = +z), `sheetOf` (windows are unions over 8 clip samples), `SHEET_ORDER` (plan, side, front = tab order).
- `R/🔭️viewpoints/🎚️selector`: radio group (arrow keys, one tab stop), one `sr-only role=status` announcement, unavailable choices keep their place with the resolver reason in name and tooltip.
- `R/🔭️viewpoints/🪟️sheet`: `ThreeViewSheet` (one zoom for all cells, one shared clock, query, representation, environment) and the public reusable `StaffageViews` (measures its box or takes a zoom). Unavailable cells show title, resolver reason, details element for technical failures and "no other view is shown". No status region per cell.
- `R/🔭️viewpoints/🪝️preference`: persisted local preference `previewViewpoint` and selector (browse toolbar); previews, browse cards and the overview compare card follow it. The overview frieze is explicitly `elevation`.
- i18n bundle `🔭️viewpoints` (en, de) plus terminology items E1-E29, D1-D26, V1-V14 applied; unused `viewer.viewpoint` removed.
- Viewer: selector, `layout` (`vl=all`) with the focused pane (`vp`), sheet through a `Stage` `sheet` prop, unavailable notice in place of the stage, stale-scene fix (viewpoint compared too). Switching writes only `{viewpoint, layout}`.
- Routing: `vp`, `vl`, compare tokens `asset~repr~selection~viewpoint`, playground `vp`, `addressProblems()` reports unknown values in words (shown on detail and compare).
- Figure: `viewpoint` required, `data-viewpoint`, `data-fidelity`, conventional-symbol note (visible, localized, in `aria-describedby`), optional `representation`. Pipeline `viewpoint` required (no default). Element: `viewpoint` required, `elevation|front|plan|all` (aligned sheet), missing or unknown gives a `role=alert`; `no-patterns` replaces `patterns`.
- Placed figure: `viewpoint` required, explicit marker instead of `null`, mirror only in the side elevation, playground notice and disabled control otherwise.
- Playground: viewpoint in type, store, sanitize, URL, export (file name, SVG metadata), selector.
- Compare: viewpoint per pane (selector, URL, title), button "Same asset, three views".
- Detail: compare link keeps the viewpoint, export lede and snippet carry representation and viewpoint, structure tables use `mzNm` and add `fzN`, `mxNm`, `myNm`, `torsionNm` (en, de, units).
- Downloads: file names `<asset>_<what>_<viewpoint>_<repr>_1-<denominator>`, description text uses the approved terms.
- Patterns (hatching) default on in the viewer state (`pat=0` is the only written form) and the element; the library default was already on.

## 3. Evidence

- Typecheck: `repo tools exec bun -- x tsc -p tsconfig.json --noEmit` in `R-package`: 0 errors in staffage code; 12 errors only in `🧰️framework/🖱️ui` (`@semio-tech/framework-transient` is not linked in `node_modules`).
- Tests: the stock `repo run @teaching/staffage-react:test` cannot resolve `@semio-tech/framework-transient` here, so tests ran with vitest and a ticket-local config adding that one alias (`🛠️scripts/sonnet-viewer/vitest.config.ts`). Full react suite at the last run: 20 files, 155 tests: 151 passed, 1 skipped, 3 failed (all three outside this work, below).
- New file `🧪️tests/🔭️viewpoints` (34 tests, all pass, with `🥒️.feature`) and `🧪️tests/🧮️structure-columns` (2 tests, written, not yet run in isolation; they ran in the full suite without failing). Covered: selector (names en/de, keys, live region, unavailable reason), state preservation of the viewer switch, sheet alignment, one mm/m, one instant while scrubbing, focus order, unavailable cells (missing plan, scale 1:500), `vp`/`vl` round trip and unknown values, compare and playground tokens, mirror rule, element (all viewpoints, invalid, `all`), cache keys, file name uniqueness, fidelity note, hatching default.
- Failures NOT caused by this work: `catalog-loading` (22 of 149 catalog assets invalid under the contracts validator at the last run); `playground-drag` x2 (the reference walker fixture is now `invalid`: `extent-front-depth`, `extent-plan-depth`, `unavailable-but-drawable`, so the picker has no "Place" button). 
- Real browser check of the site: not done; the site build and dev server depend on the same unlinked workspace package.

Also done in the last round: `Frieze` follows the preview preference (or an explicit `viewpoint`), stored and URL unknown viewpoints are reported (`ArchiveStore.problems` shown in the footer, `addressProblems` on detail, compare and playground), mirroring is refused in the reducer and in links, the compare link keeps pose or clip and time (`comparisonOf`).

## 4. Open

- Playground in front elevation arranges figures side by side along the line (no depth superposition); flagged for the coordinator.
- `viewpointCaption` (library `✂️section`) still says "Side view / Front view / Top view" and feeds `describeScene` aria-labels (contracts' area).
- Not done: an interaction test of the compare page and of the playground selector, an export-panel snippet test, a Playwright check.
- Generated tool output is in `🗑️generated/sonnet-viewer` and should be deleted at close.

## 5. Round 3 (coordinator rulings D38 and items 2-6)

- **D38 placement position:** `Placement` now has `zM` (sideways, the asset's left is +) and `yawDeg`. `linePosition(viewpoint, placement, lengthM)` projects the one position: elevation keeps x and orders by -z, front puts the figure at its z (centre of the line + z) and orders by x (nearer = larger x, painted later, overlapping where they overlap), plan keeps x and lifts by z. Used by the playground screen, the SVG export (`liftM`, sorted by nearness) and the address (`p=asset~x~pick~offset~m~z~yaw`). Keyboard: Up and Down move z, Page Up and Page Down turn (Shift 45, Alt 1), plus localized fields (z, yaw); drag in the front elevation changes z. Persisted in the store and the URL.
- **Yaw is not drawn:** the evaluator has no yaw (no `yawDeg` in `EvaluationQuery`, `root.angleDeg` is not a heading). Yaw is stored, edited and linked, and a visible localized note says it is drawn unturned. Needs a rigs decision: a yaw parameter on the evaluation query.
- **Friezes:** keep their side-by-side presentation layout and show a localized note (`data-frieze-note`) in front and plan.
- **Fixture:** `🧫️fixtures/🚶️reference-walker` made valid: the 1:500 decision is now truthful (still unavailable, with a new handcrafted `figure-block` true-extent symbol with all three views at the real 0.37 x depthM x 1.73 m, offered as an alternative); arm geometry is the original and matches the `depthM` 0.55 that another worker set meanwhile. `validateAsset` returns no issue.
- **One vocabulary:** `VIEWPOINT_NAMES` is the library's `VIEWPOINT_CAPTIONS`; the bundle entries `viewer.viewpoint.*` are tested equal to them.
- **New tests** (`🧪️tests/🧭️viewpoint-pages`, 8 tests): compare interaction (pane viewpoint into the address, three views button on one clock, wrong link value reported), playground selector, z and yaw by field and key with the address round trip, front depth order, camera projection, key map, export snippet and lede, one vocabulary.
- **Environment:** no committed workaround; the vitest alias lives only in `🛠️scripts/sonnet-viewer/vitest.config.ts`.
- **Counts at the last run:** typecheck 0 errors outside `🧰️framework/🖱️ui`; vitest 21 files, 165 tests: 163 passed, 1 skipped, 1 failed (`catalog-loading`, 4 of 149 catalog assets still invalid, left as ruled).
