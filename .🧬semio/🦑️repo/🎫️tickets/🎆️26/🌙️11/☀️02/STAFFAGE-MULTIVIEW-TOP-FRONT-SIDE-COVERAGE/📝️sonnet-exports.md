# 📝️ sonnet-exports: exports and the multiview sheet

L = `🎓️teaching/🏛️architecture/🧍️staffage`. Status 2026-11-02.

## Changed files

- New `L/🔨️modules/📤️export/📑️sheet/🟦️.ts`: `sheetScenes`, `composeSheet`, `depictSheet` (SVG, PDF) and `modelSheet` (DXF, DWG). Barrel exports are added to `L/📦️packages/🟦️typescript/🟦️.ts`.
- `🎬️cli`:
  - `--viewpoint` is required, with no default.
  - Sheet formats `sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg` take `--projection first|third` and refuse `--viewpoint`.
  - `--time` writes one clip instant.
  - The output stem is `asset-when[-t<time>]-mode-<viewpoint|sheet-projection>-1to<denominator>`.
  - DXF and DWG receive the analysis (thermal, structure) like the browser, and DXF writes `$PROJECTNAME` with asset, instant, mode, viewpoint and scale.
- `🖼️svg`:
  - Prose and title are localized: mode via `VIEW_TITLES`, viewpoint via `viewpointCaption`, both en / de. The machine ids stay in attributes.
  - New attributes `view-title-*`, `viewpoint-title-*`; sheet attributes `projection`, `projection-title-*`, `<staffage:view>` origins and `<staffage:extents-m>`.
- `📕️pdf`: Subject and Keywords are localized, and a sheet names its projection.
- `📐️dxf`: `writeDxf(scene, note)`.
- `🎞️frames`: the manifest records `viewpoint`.
- `🎭️cast`:
  - Schema, twin and exporter: `viewpoint` is required, `mirror` requires `elevation`, and `viewpointOf` is removed.
  - `heightM` is now ALWAYS the physical figure height (D13). The paper extents are in `boundsM` and `figureBoundsM`.
- `🎨️depiction/🟦️.ts`: `VIEW_TITLES` is now exported (one word, contracts file).
- `🎨️depiction/🖨️preview`: `--viewpoint` is required unless `--views`. The file names carry the viewpoint, and `thermal.referenceEnvironment` is honoured like the export CLI does (D14).
- Tests:
  - New `🧪️tests/📑️multiview-sheet/{🟦️.ts,🐍️.py,🥒️.feature}`.
  - Edited `🎭️cast-export` (viewpoint everywhere, plan-depth assertion moved to `boundsM`), `🗜️zip-reopen`, `🌡️thermal-span`, `🌤️reference-environment`.
  - Fixtures: `🎭️cast-export` (mirrored front and plan became `invalid` cases), `🎭️cast-cancellation`, `🧍️front-projection`, `📕️pdf-reopen` (viewpoint `side` was not a viewpoint).

## Arrangement (D6 / D7)

- The side elevation is the main view. The plan shares its x with it, and the front shares its y (the ground line).
- `third`: plan above, front right of the elevation (third-angle, method 3).
- `first`: plan below, front left (first-angle, method 1).
- The projection is an explicit argument with no default. The projection symbol is drawn on the sheet.
- Overall dimensions are measured on the drawn figure without ground, forces or diagrams: width and depth under the views, height and plan depth outside.
- A scale bar and a title block are drawn. Captions use `viewpointCaption`, so they follow whatever wording contracts sets (currently "Side view / Top view", to be changed to the D6 terms by contracts).
- Languages en and de via `language`, which has no default.
- Per-view `widthM`, `depthM` and `heightM` are in `sheet.extentsM` and in the SVG metadata.
- A sheet is composed only from scenes of one query, time, mode and scale; otherwise it throws.
- Preview CLI: the sheet reuses `viewpointCaption`.

## Evidence (real files, third-party parsers)

`repo tools exec bun -- x vitest run --config 🧪️tests/🎚️config/🟦️.ts multiview-sheet`: 16 of 16 passed.
- Eight sheets: four cases times first and third angle:
  - standing-adult pointing, light-line, 1:50
  - walking-commuter at clip t = 0.3 s, dark-solid, 1:100
  - tilt-window, thermal
  - forklift, moments
- Parsed independently by lxml (SVG), PyMuPDF (PDF, optional-content layers so overlays are excluded) and ezdxf (DXF, with `audit()` reporting zero errors).
- Paper mm of the figure in each view equals `metres * 1000 / denominator`, with metres taken from the evaluated state's shapes:
  - SVG within 0.02 mm, PDF within 0.03 mm, DXF within 0.02 m.
  - The residual is the depiction's contour simplification; a 1:100 figure differs by 0.009 mm.
  - Page size equals the sheet size.
  - `mmPerMetre = 1000 / denominator`.
- The alignment holds: shared x, shared ground y, and the third or first arrangement checked on the parsed boxes.
- No raw mode id (`light-line`, etc.) in the title, desc, PDF Subject or Keywords.
- Determinism: a rerun gives the same string or bytes for SVG, PDF and DXF, tested.
- CLI: three viewpoints and two sheet projections times SVG and DXF give ten different files (tested). A missing viewpoint is refused. `--time` is tested.
- Other suites run: cast-export (66 passed), thermal-span, reference-environment, zip-reopen, dxf-reopen, pdf-reopen, paper-dimensions, frame timing. All pass in the last runs, with 8 or 9 files per run.

## Findings and open points

1. **Frame handedness:** my derivation matches D6. The brief's "seen from +z" is wrong; the code and the cameras use -z.
2. **Preview DWG:** the DWG sheet goes through the same `writeDwg(ModelScene)`. Here `staffage_dwg.wasm` is not built, so it fails with the explicit `DwgUnavailableError`. DWG sheets are therefore unverified on a real file, as in the existing DWG path. The DWG carries no viewpoint metadata, because the encoder JSON has no field.
3. **Plan vs front depth:** the plan depth and the front width of the same state differ by about 1 cm for the pointing adult (1.10 m vs 1.09 m), and the walking-commuter elevation and plan widths differ by 9 mm. These are depiction differences, within the 2 cm guard tolerance.
4. **Cache keys:**
   - The Python `cast_key` hashes the whole entry, the catalog file, the presets and every source under `🔨️modules` and `🧬️schema`. The viewpoint is therefore part of the key now that it is required, and the exporter and schema revision is part of it too.
   - Not changed by me: site and React download caches (viewer owner); check there that the viewpoint is in their keys.
5. **Resolver:** `sheetScenes` calls `depict` three times from one `evaluate`d state and one shared analysis. Switching it to `resolveViews` from `🔭️multiview` is a small follow-up, once its `DrawingSource` suits the CLI.
6. **Docs not touched by me:**
   - `README.md` line 120 (the cast `viewpoint` default) and the CLI usage text.
   - `T/📐️conventions.md` should state: plan above plus front right is third-angle, plan below plus front left is first-angle.
   - The sheet prose of `🧑‍🏫` are in the SVG text.
   - Video (`🎥️video`) has no view input; frames carry the viewpoint in the manifest.
7. **Python:** `staffage_height` of sonnet-course must read the new `heightM` meaning.
