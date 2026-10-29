# Depiction, SVG writer and preview CLI

Root `S` = `🎓️teaching/🏛️architecture/🧍️staffage`.

## Files

- `S/🔨️modules/🎨️depiction/🟦️.ts`: `depict`, `depictAll`, `defaultDepictionOptions`, `modelScene`.
  Helpers: `🧩️shapes` (LOD, z order, hidden lines), `📐️geometry`, `🖍️palette`, `🧱️canvas` (extent tracking, label column), `🏷️annotations`, `🌡️thermal-view`, `🏗️structural-view` (forces + moments), `🖨️preview`.
- `S/🔨️modules/📤️export/🖼️svg/🟦️.ts`: `writeSvg`, `svgMarkup`.
- Tests: `S/🧪️tests/{📐️paper-dimensions,🖊️line-hierarchy,🔭️lod,🎨️view-synchrony,🧾️legend-units}` (+ `🥒️.feature`), fixtures `S/🧫️fixtures/{🧱️synthetic-figure,📐️paper-dimensions,🖊️line-hierarchy}`.

## Design decisions

- Paper mm, y down. `metadata.origin` is where model (0,0) landed: paper = `(origin.xMm + x·k, origin.yMm − y·k)`, `k = mmPerMetre`. Scene size = content extent (asset bounds, ground line ±0.3 m, plumes, arrows, diagrams, annotations when shown) + `marginMm`. Views differ in size, so UIs must align scenes by `origin`, not by top-left.
- Every filled closed shape becomes one `fill`-layer item `shape:<id>` with identical commands in all seven views (verified in model space relative to origin). Line views: knockout fill in background colour; solid views: tone ramp; analytic views: quantity colour.
- Outlines (`outline:<id>:<n>` in layer body/detail/texture/glazing/ground) use exact hidden-line removal: parts inside a filled shape of higher z are cut. Filled shapes of the same `region` on the same or adjacent bones hide each other's inner outlines, so knee/elbow overlaps read as one silhouette.
- Pen widths come from `lineWeightMm` (contract table). Texture is omitted wherever the weight is `null`. Shapes with `lod` above `lodFor(scale)` are omitted; at LOD <= 1 details smaller than 0.4 mm are dropped; rings are simplified at 0.015 mm.
- Themes: dark bg `#001117`, ink `#f7f3e3`; light bg `#f7f3e3`, ink `#001117`. Solid tone ramp `#2e3c3d -> #f1edde` (dark), `#243235 -> #d3d2c5` (light). `transparent` only removes the background rectangle; ink and knockouts stay. Analytic views use the light theme (no theme option exists).
- Ramps: `STRUCTURAL_COLORS` of the structure module (warm `#d7301f -> #fdae61`, cool `#2c7bb6 -> #abd9e9`, zero `#d9d9d9`). Temperatures centred on air temperature, fluxes on 0; tension/axial: positive cool. Legend `stops[].at` is normalised 0..1 between `min` and `max`.
- Value labels (force, moment, diagram peaks) are collected and placed in a column right of the drawing with leader lines, sorted by anchor height and at least 1.4 line heights apart; the free-body inset has its own column. Numbers go through `formatQuantity`; residuals below 1e-6 x weight print as 0.
- Thermal field (streamlines, parcels) is clipped to the asset bounds plus 0.6 x height, and streamlines stop where speed < 0.05 m/s and excess < 0.05 K.
- Arrows: head at the point of application, length proportional to |F| (min 1.5 mm, longest 7..14 mm).
- Diagrams: N and V offset to the local +y side; M on the tension side (M > 0 toward local -y); 6 mm ordinate for the global peak.
- `modelScene`: metres, LOD 3, pen table 1:100 (x100 as 1/100 mm). Layers STAFFAGE-BODY (ACI 7, 35), -DETAIL (8, 18), -TEXTURE (9, 13), -GLAZING (4, 18), GROUND (3, 50), ANALYSIS-ARROWS (1, 25), ANALYSIS-DIAGRAM (5, 18).
- SVG: `width/height` in mm, viewBox in mm, `<g id="staffage-<layer>" inkscape:groupmode="layer">`, hatch `<pattern>` defs, `<title>/<desc>/<metadata>`, `data-quantity/value/unit` on analytic items, generic font stack, fill-rule evenodd, no raster.

## Usage

```
repo tools exec bun -- S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --file <category.json | asset.json> --asset <id> \
  [--pose id | --clip id --time s] [--view dark-line|dark-solid|light-line|light-solid|thermal|forces|moments] \
  [--scale 50] [--language en|de] [--dpi 300] [--no-png] [--sheet] --out <dir>
```

Without `--view` all seven views are written (`<asset>--<pose>--<view>-1-<scale>.svg/.png`). `--sheet` writes `<asset>--sheet-1-<scale>` with all poses as rows and the seven views as columns. PNGs come from the system `rsvg-convert` (not a registered repo tool: `repo tools exec rsvg-convert` fails with "Unknown tool"), behind the `Rasterizer` interface. Open PNGs with the Read tool.

## Contract decisions (applied)

1. `DepictionOptions.theme?: "light" | "dark"` sets paper and ink of thermal/forces/moments (default light); `defaultDepictionOptions(view, language, denominator, viewpoint, theme?)`. Dark analytic views use paper `#001117`, ink `#f7f3e3`, lifted strong ramp ends and lighter overlay colours.
2. `Legend.stops[].at` is normalised 0..1 everywhere; the depiction emits it that way. Legend ticks closer than 6 mm are dropped.
3. Scenes of different views differ in extent; views are aligned by `metadata.origin`.
4. `rsvg-convert` is not a registered tool; the CLI uses the system binary behind `Rasterizer`.
5. Plume legends come from `plumeLegend` of the thermal module: airflow parcels and streamlines are coloured by temperature excess (K, |excess| ramp, colder-than-ambient uses the cool ramp), vapour parcels by vapour excess over ambient (g/kg), never absolute humidity. The field layer is clipped to the asset bounds plus 1.5 x height (at most 3 m); longer calm-air exhaust plumes are cut there.
