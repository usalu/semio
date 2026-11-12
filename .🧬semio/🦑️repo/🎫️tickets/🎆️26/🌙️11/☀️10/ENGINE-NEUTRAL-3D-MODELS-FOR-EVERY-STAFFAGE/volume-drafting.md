# Volume drafting aids: scripts, draft quality and advice for authors

Ticket-local authoring aids (not repo code). All scripts sit next to this file and are run through the repo's own runtime. Outputs go to `🗑️generated/` (deleted when the ticket closes), so every command below is re-runnable. Staffage modules and the neutral kernel tessellator are only imported, never edited.

## How to run

Run from this ticket folder. The first start of `repo tools exec` may wait for machine capacity (a few minutes on a busy host); the run itself takes about 15 s for all assets.

```
repo tools exec bun -- volume-draft.ts --asset city-bicycle            # one asset (slug, category/slug or a JSON file)
repo tools exec bun -- volume-draft.ts --all                           # all 152 assets, progress "[i/n] category/slug" on stderr
repo tools exec bun -- volume-summary.ts                               # tables -> 🗑️generated/tables.md and tables.json
repo tools exec bun -- volume-check.ts city-bicycle --volume 🗑️generated/drafts/vehicles/city-bicycle.json
repo tools exec bun -- volume-check.ts <slug>                          # uses asset.volume of the catalog JSON once authored
```

- `volume-draft.ts` options: `--out <dir>` (default `🗑️generated/drafts`), `--scale 50`, `--no-check`. Writes `<out>/<category>/<slug>.json` = `{ volume: { status: "authored", solids, decisions: [] }, report }` and with `--all` also `<out>/index.json`. Ctrl-C (SIGINT/SIGTERM, SIGHUP or SIGBREAK) stops after the current asset and exits 130; a second Ctrl-C forces exit.
- `volume-check.ts` options: `--volume <file>` (draft or bare volume, otherwise `asset.volume`), `--scale 50`, `--max-bbox-error 0.08`, `--min-iou 0.80`, `--json`. Exit 0 pass, 1 below a threshold, 2 usage or no volume, 130 cancelled. It prints the bounding box per axis against `dimensions` with relative error, and per view the IoU, recall (share of the drawn area the volume covers) and precision (share of the volume inside the drawing) plus the solids that stick out most and the drawn shapes the volume misses most.
- Files: `volume-lib.ts` (catalog access, rest frames, rasteriser, analysis), `volume-draft.ts`, `volume-check.ts`, `volume-summary.ts`.

## What the draft does

Source: the 1:50 representation. Every `body`, `detail` and `glazing` elevation shape becomes solids in the bone-local u, v, w frame (`texture` and `ground` shapes are skipped and listed in `report.skipped`).

| elevation shape | draft solid |
|---|---|
| `capsule` | `capsule`; lateral radius from the matched front/plan span becomes `lateralScale` (built on the u axis and placed by `transform` when tilted, because the kernel applies `lateralScale` to a fixed direction) |
| `ellipse` | `ellipsoid` with the lateral semi-axis from the span, `rotationDeg` as a `transform` about w |
| `rect` | `box` (with `cornerRadius`) rather than an extrude: exact and one of the six kinds |
| closed `poly` / `path` | `extrude` (plane `xy`, along w) of the outline, paths flattened by the staffage `flattenGeometry` (0.5 mm chord) and simplified to 0.3 mm; rings are grouped into outers and holes; further outers become `-2`, `-3` solids |
| open `poly` (tubes, spokes, hairline details) | one round-bar `capsule` per segment; radius from the exact-id lateral twin, else from the role pen at this scale (flagged `needs-author`), capped at 20 % of the longest segment |
| `solid.revolved` | `revolve` about the hinted axis (axis point and direction converted from `axisBone` to the shape bone through the rest frames), profile = outline as [axial, radial], `lateralScale` kept; outlines straddling the axis use their radial envelope |
| `solid.extruded` | `extrude` over `z0..z1` (`sweep` `w`: plane `xy`; `sweep` `v`: plane `zx`); a `zTip` taper becomes a `loft` of rectangle-like superellipse sections along u, because `taperTo` scales the whole profile about its centroid and cannot express a span that narrows along the bone |
| `member` | `box` (rectangular section) or two-section `loft` (circular) between the converted end points |

Lateral span: front and plan shapes of the same `id`; else the same id family (`-l/-r`, `-1`, prefix) on the same bone; else the best 1D overlap on the same bone (flagged `low-confidence-lateral`). Spans from both views that overlap are merged (the better-scored one wins, disagreement above 35 % is flagged). Two symmetric clusters become one solid with `repeat { count: 2, translation: [0, 0, step] }` (flag `pair-as-repeat`), otherwise one solid per cluster (`-l`/`-r`). No match at all: span estimated from the shape's own size and `depthM`, flag `needs-author`. Parts that only the front or plan view draws (no elevation shape claims them) are drafted from their own outline, extruded over the extent of their twin in the other view, or over an estimate (`needs-author`); see `view-only-front` / `view-only-plan` in the provenance.

`id`, `bone`, `material`, `role`, `tone`, `region`/`regions` are carried over. A missing shape material is taken from the front/plan twin, then from the thermal region, then from the commonest material on the bone (flag `material-guessed`, info). `report.provenance` lists per solid the route and the source shapes and views; `report.flags` has severities `needs-author`, `low-confidence`, `info`.

Dimension axes: the schema defines `widthM` as the extent along x (ahead), `heightM` along y, `depthM` along z (lateral). All bounding boxes in drafts and checks are `[x, y, z]` against `[widthM, heightM, depthM]` (the brief's "depth(x), width(z)" wording is the reverse of the schema).

Check method: the neutral kernel tessellates every solid (chord 1 mm to 20 mm by asset size, 12 to 64 segments), the triangles are carried to world at rest and projected with the staffage `CAMERAS`, and rasterised with an owned scanline filler on a grid of the long side / 640. The authored side is the staffage `evaluate` of a synthetic rest pose, so authored solids, members and flat shapes are placed by the real code; only closed `body`/`detail`/`glazing` shapes count. Thin parts below one grid cell are under-sampled on both sides, so IoU of fine wire work is optimistic. Python shapely is not installed on this machine, so the owned rasteriser is used.

## Draft quality, all 152 assets (1:50, at rest)

Gates: bounding box error at most 8 % per axis and IoU at least 0.80 per view. "Needs-author" and "low-confidence" columns count flags (and in brackets the assets that have at least one). "Unmatched shapes" are front/plan shapes no elevation shape claims; they were drafted as view-only parts, count them as work to review.

| category | assets | median IoU e/f/p | median abs bbox err % x/y/z | pass both gates | needs-author (assets) | low-confidence (assets) | unmatched shapes |
|---|---|---|---|---|---|---|---|
| animals | 14 | 1.00 / 0.88 / 0.90 | 1 / 4 / 1 | 10 | 58 (11) | 26 (8) | 47 |
| appliances | 24 | 0.99 / 0.96 / 0.98 | 0 / 0 / 0 | 16 | 351 (24) | 96 (19) | 54 |
| components | 18 | 1.00 / 1.00 / 1.00 | 0 / 0 / 0 | 14 | 156 (17) | 124 (13) | 97 |
| furniture | 12 | 0.92 / 0.63 / 0.81 | 1 / 0 / 0 | 3 | 71 (10) | 80 (10) | 30 |
| humans | 28 | 0.98 / 0.88 / 0.84 | 3 / 2 / 1 | 10 | 682 (28) | 373 (27) | 511 |
| machinery | 16 | 1.00 / 0.97 / 1.00 | 0 / 0 / 0 | 10 | 120 (12) | 247 (11) | 61 |
| objects | 15 | 0.90 / 0.84 / 0.88 | 1 / 0 / 0 | 3 | 95 (12) | 67 (13) | 77 |
| vegetation | 16 | 0.98 / 0.77 / 0.78 | 0 / 1 / 0 | 1 | 410 (14) | 1062 (15) | 346 |
| vehicles | 9 | 1.00 / 0.91 / 0.96 | 0 / 0 / 0 | 6 | 143 (9) | 150 (9) | 39 |
| **all** | 152 | 0.99 / 0.90 / 0.90 | 0 / 0 / 0 | 73 | 2086 (137) | 2225 (125) | 1262 |

### animals

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| dairy-cow | 48 | +1 / +2 / +0 | 0.98 / 0.94 / 0.88 | 1.00 / 0.99 / 0.99 | 0.98 / 0.94 / 0.88 | 11 | 2 | 6 | 3 |
| domestic-cat | 50 | +2 / +7 / +1 | 1.00 / 0.82 / 0.88 | 1.00 / 1.00 / 1.00 | 1.00 / 0.82 / 0.88 | 6 | 7 | 1 | 4 |
| goat | 34 | -0 / +0 / +0 | 0.99 / 0.77 / 0.84 | 1.00 / 0.98 / 0.99 | 0.99 / 0.78 / 0.84 | 2 | 1 | 8 | 0 |
| hedgehog | 16 | +3 / +31 / +0 | 0.78 / 0.88 / 0.91 | 1.00 / 1.00 / 0.98 | 0.78 / 0.88 / 0.92 | 0 | 0 | 20 | 0 |
| herring-gull | 20 | +4 / -6 / -3 | 1.00 / 0.88 / 0.93 | 1.00 / 0.99 / 0.99 | 1.00 / 0.89 / 0.94 | 0 | 0 | 0 | 0 |
| horse | 55 | -5 / -5 / +1 | 1.00 / 0.90 / 0.87 | 1.00 / 1.00 / 1.00 | 1.00 / 0.90 / 0.88 | 10 | 4 | 0 | 6 |
| labrador-dog | 49 | -0 / +2 / +0 | 0.98 / 0.88 / 0.86 | 1.00 / 1.00 / 0.99 | 0.98 / 0.88 / 0.87 | 8 | 7 | 1 | 4 |
| mallard-duck | 23 | -2 / +7 / +1 | 1.00 / 0.90 / 0.94 | 1.00 / 0.98 / 0.99 | 1.00 / 0.92 / 0.94 | 2 | 2 | 0 | 0 |
| pig | 28 | -0 / +0 / -1 | 1.00 / 0.84 / 0.90 | 1.00 / 0.99 / 0.99 | 1.00 / 0.85 / 0.91 | 3 | 2 | 2 | 0 |
| pigeon | 27 | +0 / +4 / +0 | 0.99 / 0.91 / 0.86 | 1.00 / 1.00 / 0.99 | 0.99 / 0.91 / 0.87 | 2 | 0 | 4 | 6 |
| red-fox | 26 | +0 / +0 / +1 | 1.00 / 0.97 / 0.99 | 1.00 / 0.97 / 1.00 | 1.00 / 1.00 / 0.99 | 1 | 0 | 0 | 0 |
| red-squirrel | 21 | +98 / -5 / +3 | 0.99 / 0.79 / 0.97 | 1.00 / 0.99 / 1.00 | 1.00 / 0.80 / 0.97 | 0 | 0 | 2 | 0 |
| roe-deer | 36 | -1 / +10 / +0 | 1.00 / 0.82 / 0.90 | 1.00 / 0.99 / 0.99 | 1.00 / 0.83 / 0.91 | 10 | 1 | 1 | 3 |
| sheep | 27 | +0 / +0 / -0 | 1.00 / 0.83 / 0.91 | 1.00 / 1.00 / 1.00 | 1.00 / 0.83 / 0.91 | 3 | 0 | 2 | 0 |

### appliances

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| ceiling-diffuser | 11 | +0 / +0 / +0 | 0.97 / 0.96 / 1.00 | 1.00 / 1.00 / 1.00 | 0.97 / 0.96 / 1.00 | 4 | 0 | 5 | 1 |
| circulation-pump | 19 | +0 / +0 / +0 | 1.00 / 0.96 / 0.99 | 1.00 / 0.99 / 1.00 | 1.00 / 0.97 / 0.99 | 7 | 4 | 1 | 0 |
| decentralised-hrv-fan | 46 | +0 / +0 / +0 | 1.00 / 0.81 / 0.83 | 1.00 / 0.82 / 0.83 | 1.00 / 0.99 / 0.99 | 41 | 6 | 0 | 0 |
| desktop-monitor | 33 | -0 / -0 / +0 | 0.99 / 0.98 / 0.96 | 1.00 / 0.99 / 0.98 | 0.99 / 0.99 / 0.99 | 3 | 5 | 2 | 0 |
| duct-fan | 15 | +0 / +0 / +0 | 1.00 / 0.81 / 0.81 | 1.00 / 0.81 / 0.81 | 1.00 / 1.00 / 1.00 | 11 | 4 | 0 | 0 |
| electric-fan-heater | 30 | +4 / +0 / +0 | 0.99 / 0.99 / 0.99 | 1.00 / 1.00 / 1.00 | 0.99 / 1.00 / 0.99 | 19 | 4 | 2 | 1 |
| electric-kettle | 8 | -2 / -1 / +0 | 1.00 / 0.97 / 0.78 | 1.00 / 1.00 / 1.00 | 1.00 / 0.97 / 0.78 | 1 | 1 | 3 | 2 |
| electricity-meter | 24 | +0 / +0 / +0 | 1.00 / 0.98 / 1.00 | 1.00 / 0.99 / 1.00 | 1.00 / 0.99 / 1.00 | 6 | 10 | 2 | 1 |
| gas-boiler | 41 | +0 / +0 / +0 | 1.00 / 0.98 / 0.99 | 1.00 / 1.00 / 1.00 | 1.00 / 0.98 / 0.99 | 18 | 6 | 1 | 0 |
| gas-hob | 38 | +0 / +0 / +0 | 0.94 / 0.95 / 1.00 | 1.00 / 0.99 / 1.00 | 0.94 / 0.96 / 1.00 | 9 | 2 | 5 | 1 |
| gas-meter | 33 | +0 / +3 / +0 | 0.99 / 0.98 / 1.00 | 1.00 / 1.00 / 1.00 | 0.99 / 0.98 / 1.00 | 11 | 22 | 2 | 0 |
| hot-water-cylinder | 39 | +0 / +0 / +0 | 1.00 / 0.99 / 0.78 | 1.00 / 1.00 / 1.00 | 1.00 / 0.99 / 0.78 | 8 | 1 | 0 | 0 |
| kitchen-extractor-hood | 9 | +4 / +0 / +0 | 0.99 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 0.99 / 1.00 / 1.00 | 1 | 0 | 1 | 1 |
| laptop | 48 | -19 / -89 / +0 | 0.82 / 0.87 / 1.00 | 1.00 / 0.96 / 1.00 | 0.82 / 0.90 / 1.00 | 11 | 5 | 6 | 0 |
| mvhr-unit | 27 | +28 / +8 / +0 | 0.71 / 0.96 / 0.87 | 1.00 / 1.00 / 1.00 | 0.71 / 0.96 / 0.87 | 13 | 1 | 0 | 0 |
| pendant-lamp | 20 | +0 / +1 / +0 | 0.98 / 0.79 / 0.78 | 1.00 / 1.00 / 1.00 | 0.98 / 0.79 / 0.78 | 1 | 0 | 1 | 0 |
| room-thermostat | 5 | +0 / +0 / +0 | 1.00 / 0.83 / 0.93 | 1.00 / 0.92 / 0.93 | 1.00 / 0.90 / 1.00 | 3 | 0 | 0 | 0 |
| server-rack | 156 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 143 | 4 | 4 | 3 |
| shower-head | 10 | +1 / -1 / -1 | 0.45 / 0.26 / 0.51 | 1.00 / 0.97 / 0.98 | 0.45 / 0.27 / 0.52 | 1 | 0 | 0 | 0 |
| split-ac-indoor-unit | 19 | +0 / +1 / +0 | 1.00 / 0.99 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 0.99 / 1.00 | 14 | 2 | 12 | 3 |
| thermostatic-radiator-valve | 6 | +0 / +0 / +6 | 0.88 / 0.89 / 0.95 | 1.00 / 0.97 / 0.95 | 0.88 / 0.92 / 1.00 | 2 | 1 | 2 | 0 |
| toaster | 21 | +19 / +12 / +0 | 0.80 / 0.93 / 0.92 | 1.00 / 1.00 / 1.00 | 0.80 / 0.93 / 0.92 | 9 | 1 | 2 | 0 |
| tower-server | 10 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 2 | 9 | 3 | 4 |
| vacuum-cleaner | 39 | +74 / -60 / -0 | 1.00 / 0.91 / 0.97 | 1.00 / 0.97 / 0.99 | 1.00 / 0.93 / 0.98 | 13 | 8 | 0 | 20 |

### components

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| cantilever-balcony | 35 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 18 | 10 | 10 | 1 |
| curtain-wall-bay | 24 | +0 / +0 / +0 | 1.00 / 0.79 / 0.98 | 1.00 / 0.79 / 0.98 | 1.00 / 1.00 / 1.00 | 6 | 20 | 4 | 1 |
| etics-brick-wall-12 | 16 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 12 | 0 | 0 | 0 |
| etics-brick-wall-30 | 16 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 12 | 0 | 0 | 0 |
| external-venetian-blind | 29 | +0 / +1 / +0 | 0.93 / 0.64 / 1.00 | 1.00 / 0.64 / 1.00 | 0.93 / 1.00 / 1.00 | 6 | 2 | 2 | 4 |
| glass-balustrade | 15 | +0 / +0 / +0 | 1.00 / 0.99 / 1.00 | 1.00 / 0.99 / 1.00 | 1.00 / 1.00 / 1.00 | 0 | 14 | 0 | 0 |
| glulam-beam | 27 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 6 | 2 | 2 | 16 |
| masonry-wall-with-lintel | 30 | +0 / +0 / +0 | 0.95 / 1.00 / 0.94 | 1.00 / 1.00 / 1.00 | 0.95 / 1.00 / 0.94 | 22 | 0 | 21 | 54 |
| panel-radiator | 16 | +22 / +1 / +0 | 0.89 / 0.99 / 0.82 | 1.00 / 0.99 / 1.00 | 0.89 / 1.00 / 0.82 | 6 | 12 | 4 | 8 |
| sectional-garage-door | 112 | +1 / +0 / +0 | 0.42 / 0.94 / 0.44 | 0.99 / 0.95 / 0.54 | 0.42 / 0.99 / 0.71 | 11 | 9 | 7 | 2 |
| sliding-door | 25 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 17 | 7 | 5 | 3 |
| solid-brick-wall-1960 | 15 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 12 | 0 | 0 | 0 |
| spiral-stair | 146 | +0 / +0 / +0 | 0.89 / 0.94 / 0.87 | 1.00 / 1.00 / 1.00 | 0.89 / 0.94 / 0.87 | 2 | 30 | 4 | 33 |
| stair-flight | 74 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1 | 0 | 15 | 14 |
| steel-column | 20 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 8 | 7 | 4 | 0 |
| tied-canopy | 22 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 4 | 5 | 1 | 2 |
| tilt-window | 19 | +0 / +0 / +0 | 1.00 / 0.91 / 1.00 | 1.00 / 0.91 / 1.00 | 1.00 / 1.00 / 1.00 | 6 | 5 | 9 | 0 |
| wheelchair-ramp | 24 | +0 / +0 / +0 | 0.87 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 0.87 / 1.00 / 1.00 | 7 | 1 | 9 | 7 |

### furniture

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| bar-stool | 17 | +1 / +0 / +1 | 0.96 / 0.52 / 0.73 | 0.96 / 0.99 / 1.00 | 1.00 / 0.52 / 0.73 | 0 | 8 | 0 | 0 |
| bookshelf | 35 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 15 | 30 | 5 | 0 |
| cafe-chair | 19 | +33 / +0 / +1 | 0.46 / 0.71 / 0.17 | 1.00 / 0.95 / 0.91 | 0.46 / 0.73 / 0.17 | 3 | 6 | 0 | 2 |
| dining-table | 10 | +0 / +0 / +0 | 1.00 / 0.27 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 0.27 / 1.00 | 4 | 1 | 0 | 0 |
| folding-deck-chair | 14 | +16 / +11 / -92 | 0.17 / 0.88 / 0.79 | 1.00 / 0.88 / 0.79 | 0.17 / 1.00 / 1.00 | 2 | 1 | 8 | 0 |
| height-adjustable-desk | 41 | +39 / +0 / +0 | 0.29 / 0.32 / 0.81 | 1.00 / 1.00 / 1.00 | 0.29 / 0.32 / 0.81 | 13 | 5 | 0 | 0 |
| office-swivel-chair | 43 | -1 / +0 / -0 | 0.35 / 0.35 / 0.78 | 0.36 / 0.99 / 0.88 | 0.93 / 0.35 / 0.87 | 9 | 1 | 12 | 5 |
| operating-table | 32 | +0 / +0 / +0 | 0.88 / 0.95 / 0.71 | 1.00 / 1.00 / 1.00 | 0.88 / 0.95 / 0.71 | 7 | 0 | 4 | 0 |
| park-bench | 63 | +25 / +0 / -0 | 0.45 / 0.41 / 0.84 | 1.00 / 0.94 / 1.00 | 0.45 / 0.42 / 0.84 | 10 | 23 | 0 | 0 |
| picnic-table | 11 | +0 / +0 / +0 | 1.00 / 0.54 / 0.98 | 1.00 / 0.55 / 1.00 | 1.00 / 0.97 / 0.98 | 0 | 0 | 0 | 0 |
| single-bed | 14 | +0 / +0 / +0 | 1.00 / 0.97 / 0.82 | 1.00 / 0.97 / 0.89 | 1.00 / 1.00 / 0.92 | 4 | 2 | 1 | 0 |
| three-seat-sofa | 42 | +1 / +1 / +0 | 1.00 / 0.95 / 0.98 | 1.00 / 1.00 / 1.00 | 1.00 / 0.95 / 0.98 | 4 | 3 | 0 | 0 |

### humans

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| child-on-kick-scooter | 77 | -0 / +4 / -1 | 0.97 / 0.87 / 0.87 | 1.00 / 0.99 / 0.98 | 0.97 / 0.88 / 0.89 | 25 | 25 | 14 | 5 |
| courier-with-hand-truck | 73 | -1 / +1 / +2 | 0.95 / 0.91 / 0.87 | 1.00 / 0.98 / 0.99 | 0.95 / 0.93 / 0.88 | 12 | 19 | 17 | 22 |
| cyclist | 89 | +0 / +0 / +0 | 0.95 / 0.85 / 0.70 | 1.00 / 0.99 / 0.99 | 0.95 / 0.85 / 0.71 | 28 | 42 | 21 | 48 |
| elderly-with-cane | 56 | -14 / +9 / -1 | 0.93 / 0.88 / 0.81 | 1.00 / 0.99 / 1.00 | 0.93 / 0.89 / 0.81 | 23 | 9 | 23 | 13 |
| jogger | 89 | -65 / +3 / +1 | 0.99 / 0.90 / 0.80 | 1.00 / 1.00 / 1.00 | 0.99 / 0.90 / 0.80 | 49 | 8 | 36 | 23 |
| kneeling-gardener | 35 | +1 / +50 / +2 | 0.93 / 0.92 / 0.85 | 1.00 / 0.98 / 1.00 | 0.93 / 0.93 / 0.86 | 7 | 4 | 34 | 11 |
| leaning-person | 61 | -36 / +1 / +1 | 0.98 / 0.89 / 0.83 | 1.00 / 0.97 / 0.99 | 0.99 / 0.91 / 0.84 | 27 | 9 | 15 | 10 |
| parent-carrying-toddler | 39 | +0 / +2 / +0 | 0.96 / 0.88 / 0.85 | 1.00 / 1.00 / 1.00 | 0.96 / 0.88 / 0.85 | 6 | 10 | 31 | 8 |
| parent-with-stroller | 106 | -32 / +5 / +0 | 0.99 / 0.93 / 0.69 | 1.00 / 0.99 / 0.96 | 1.00 / 0.95 / 0.71 | 40 | 21 | 8 | 11 |
| person-cooking | 51 | +1 / +0 / +0 | 0.99 / 0.97 / 0.96 | 1.00 / 0.99 / 0.99 | 0.99 / 0.98 / 0.97 | 9 | 16 | 8 | 27 |
| person-on-crutches | 71 | -23 / +6 / +0 | 0.98 / 0.73 / 0.83 | 1.00 / 1.00 / 0.97 | 0.98 / 0.73 / 0.85 | 27 | 19 | 11 | 9 |
| person-with-rollator | 93 | -1 / +2 / +2 | 0.98 / 0.83 / 0.86 | 1.00 / 0.99 / 0.98 | 0.98 / 0.84 / 0.87 | 40 | 36 | 11 | 4 |
| person-with-umbrella | 100 | -0 / -13 / -0 | 0.98 / 0.89 / 0.81 | 1.00 / 1.00 / 0.96 | 0.98 / 0.89 / 0.83 | 49 | 14 | 9 | 8 |
| person-with-white-cane | 54 | -62 / +33 / +0 | 1.00 / 0.85 / 0.81 | 1.00 / 1.00 / 0.98 | 1.00 / 0.85 / 0.82 | 18 | 15 | 8 | 11 |
| pregnant-person | 47 | -50 / +1 / -20 | 0.98 / 0.86 / 0.85 | 1.00 / 0.98 / 0.99 | 0.98 / 0.87 / 0.86 | 18 | 17 | 30 | 29 |
| runner-with-blade-prosthesis | 43 | -10 / +2 / +3 | 0.98 / 0.87 / 0.80 | 1.00 / 0.97 / 0.97 | 0.98 / 0.89 / 0.82 | 9 | 10 | 16 | 24 |
| running-child | 59 | +0 / -0 / -1 | 0.99 / 0.91 / 0.83 | 1.00 / 0.99 / 0.96 | 0.99 / 0.92 / 0.86 | 25 | 15 | 24 | 11 |
| seated-guitarist | 50 | -4 / +19 / +3 | 0.97 / 0.86 / 0.85 | 1.00 / 0.98 / 0.98 | 0.97 / 0.87 / 0.86 | 23 | 5 | 32 | 23 |
| seated-office-worker | 95 | +0 / +4 / +1 | 0.91 / 0.87 / 0.97 | 1.00 / 1.00 / 0.99 | 0.91 / 0.88 / 0.97 | 46 | 14 | 26 | 22 |
| seated-reader | 53 | -0 / -2 / +0 | 0.99 / 0.92 / 0.97 | 1.00 / 0.97 / 1.00 | 1.00 / 0.94 / 0.97 | 20 | 15 | 22 | 24 |
| skateboarder | 41 | -25 / +2 / -71 | 1.00 / 0.96 / 0.95 | 1.00 / 0.99 / 0.99 | 1.00 / 0.97 / 0.96 | 20 | 4 | 41 | 17 |
| sleeping-person | 31 | -1 / -0 / +0 | 1.00 / 0.97 / 0.97 | 1.00 / 0.99 / 1.00 | 1.00 / 0.98 / 0.97 | 6 | 0 | 6 | 5 |
| standing-adult | 42 | +8 / +0 / +0 | 0.96 / 0.87 / 0.81 | 1.00 / 1.00 / 1.00 | 0.96 / 0.87 / 0.81 | 6 | 3 | 11 | 27 |
| standing-phone-user | 51 | -37 / +2 / +0 | 0.97 / 0.87 / 0.79 | 1.00 / 1.00 / 1.00 | 0.97 / 0.87 / 0.80 | 11 | 4 | 13 | 27 |
| tour-guide | 87 | -54 / -9 / -1 | 0.99 / 0.89 / 0.77 | 1.00 / 0.99 / 0.90 | 0.99 / 0.90 / 0.84 | 37 | 6 | 12 | 12 |
| walking-commuter | 60 | -2 / -0 / +0 | 0.88 / 0.83 / 0.78 | 1.00 / 1.00 / 1.00 | 0.88 / 0.83 / 0.78 | 21 | 10 | 12 | 15 |
| wheelchair-user | 76 | -0 / -0 / +1 | 1.00 / 0.90 / 0.96 | 1.00 / 0.99 / 1.00 | 1.00 / 0.91 / 0.96 | 27 | 20 | 11 | 14 |
| worker-carrying-box | 107 | -43 / +1 / +0 | 0.99 / 0.93 / 0.87 | 1.00 / 1.00 / 0.93 | 0.99 / 0.93 / 0.93 | 53 | 3 | 9 | 5 |

### machinery

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| concrete-mixer-truck | 66 | +0 / +11 / +0 | 0.97 / 0.87 / 0.88 | 1.00 / 0.96 / 0.98 | 0.97 / 0.91 / 0.89 | 29 | 16 | 0 | 0 |
| diesel-generator | 22 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 8 | 3 | 10 | 1 |
| evaporative-humidifier | 14 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 6 | 0 | 6 | 0 |
| excavator | 55 | +18 / +63 / +0 | 0.99 / 0.95 / 0.97 | 1.00 / 0.97 / 1.00 | 0.99 / 0.98 / 0.97 | 16 | 6 | 2 | 10 |
| expansion-valve | 9 | +3 / +7 / +0 | 0.79 / 0.91 / 0.86 | 1.00 / 1.00 / 1.00 | 0.79 / 0.91 / 0.86 | 0 | 0 | 0 | 0 |
| finned-heat-exchanger | 30 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 3 | 0 | 2 | 0 |
| forklift | 46 | +24 / +0 / -0 | 1.00 / 0.96 / 0.99 | 1.00 / 0.98 / 0.99 | 1.00 / 0.98 / 1.00 | 8 | 34 | 4 | 11 |
| heat-pump-outdoor-unit | 17 | -1 / +0 / +2 | 1.00 / 0.99 / 1.00 | 1.00 / 0.99 / 1.00 | 1.00 / 1.00 / 1.00 | 9 | 3 | 8 | 0 |
| oil-pumpjack | 31 | +0 / -3 / +0 | 1.00 / 0.69 / 1.00 | 1.00 / 0.99 / 1.00 | 1.00 / 0.70 / 1.00 | 0 | 2 | 0 | 0 |
| plate-heat-exchanger | 8 | +0 / -2 / +0 | 1.00 / 0.99 / 1.00 | 1.00 / 0.99 / 1.00 | 1.00 / 1.00 / 1.00 | 1 | 0 | 0 | 0 |
| scissor-lift | 25 | +0 / +0 / +0 | 1.00 / 0.98 / 1.00 | 1.00 / 0.98 / 1.00 | 1.00 / 1.00 / 1.00 | 0 | 12 | 0 | 0 |
| scroll-compressor | 37 | -3 / +0 / +0 | 1.00 / 0.97 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 0.97 / 1.00 | 2 | 0 | 0 | 0 |
| solar-thermal-collector | 20 | +0 / +0 / +0 | 0.96 / 0.90 / 1.00 | 1.00 / 1.00 / 1.00 | 0.96 / 0.90 / 1.00 | 7 | 17 | 4 | 0 |
| sorption-wheel | 148 | +0 / +0 / +0 | 1.00 / 0.84 / 0.83 | 1.00 / 0.84 / 0.83 | 1.00 / 1.00 / 1.00 | 9 | 135 | 0 | 0 |
| tower-crane | 71 | +0 / +0 / +0 | 1.00 / 0.20 / 0.99 | 1.00 / 1.00 / 1.00 | 1.00 / 0.20 / 0.99 | 0 | 5 | 19 | 25 |
| wheel-loader | 52 | -0 / +0 / +0 | 0.94 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 0.94 / 1.00 / 1.00 | 22 | 14 | 6 | 0 |

### objects

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| bicycle-stand | 8 | +1 / +0 / +0 | 1.00 / 0.99 / 0.98 | 1.00 / 1.00 / 1.00 | 1.00 / 0.99 / 0.98 | 6 | 0 | 6 | 8 |
| chalkboard | 21 | +0 / +0 / +0 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 9 | 16 | 0 | 0 |
| charcoal-grill | 23 | +2 / +0 / +1 | 0.90 / 0.72 / 0.79 | 1.00 / 1.00 / 1.00 | 0.90 / 0.72 / 0.79 | 7 | 6 | 11 | 7 |
| cooking-pot | 18 | +0 / +0 / +0 | 0.93 / 1.00 / 0.79 | 1.00 / 1.00 / 1.00 | 0.93 / 1.00 / 0.79 | 9 | 2 | 9 | 1 |
| drinking-fountain | 18 | +33 / +0 / +3 | 0.81 / 0.67 / 0.69 | 1.00 / 1.00 / 1.00 | 0.81 / 0.67 / 0.69 | 3 | 1 | 6 | 1 |
| market-parasol | 55 | +9 / +3 / +3 | 0.37 / 0.46 / 0.88 | 1.00 / 1.00 / 1.00 | 0.37 / 0.46 / 0.88 | 7 | 11 | 4 | 4 |
| patio-heater | 21 | +0 / +3 / +0 | 0.90 / 0.75 / 0.78 | 1.00 / 1.00 / 1.00 | 0.90 / 0.75 / 0.78 | 4 | 5 | 10 | 12 |
| playground-swing | 44 | +0 / +0 / +0 | 0.65 / 0.94 / 0.94 | 1.00 / 0.95 / 0.95 | 0.65 / 0.98 / 0.98 | 24 | 13 | 7 | 3 |
| retractable-bollard | 24 | +1 / -33 / +0 | 0.84 / 0.90 / 1.00 | 1.00 / 1.00 / 1.00 | 0.84 / 0.90 / 1.00 | 7 | 1 | 5 | 4 |
| shopping-trolley | 20 | +23 / +5 / +35 | 0.47 / 0.57 / 0.82 | 1.00 / 0.80 / 1.00 | 0.47 / 0.67 / 0.82 | 5 | 2 | 3 | 15 |
| stepladder | 15 | +0 / +0 / +2 | 1.00 / 0.60 / 0.82 | 1.00 / 1.00 / 0.98 | 1.00 / 0.60 / 0.83 | 0 | 3 | 0 | 16 |
| storage-crate-stack | 30 | +0 / +0 / +0 | 1.00 / 0.95 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 0.95 / 1.00 | 0 | 5 | 10 | 10 |
| street-lamp | 28 | +0 / +0 / +0 | 1.00 / 0.75 / 0.90 | 1.00 / 1.00 / 1.00 | 1.00 / 0.75 / 0.90 | 3 | 1 | 2 | 4 |
| tealight | 6 | +12 / +0 / +0 | 0.81 / 0.84 / 0.77 | 1.00 / 1.00 / 1.00 | 0.81 / 0.84 / 0.77 | 0 | 1 | 0 | 0 |
| waste-bin | 25 | +24 / +2 / +0 | 0.93 / 0.88 / 0.93 | 1.00 / 0.97 / 1.00 | 0.93 / 0.90 / 0.93 | 11 | 0 | 4 | 1 |

### vegetation

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| agave | 41 | +1 / +1 / -54 | 1.00 / 0.19 / 0.16 | 1.00 / 0.54 / 1.00 | 1.00 / 0.23 / 0.16 | 0 | 34 | 1 | 1 |
| apple-tree | 57 | +0 / +2 / +0 | 0.98 / 0.73 / 0.90 | 1.00 / 1.00 / 1.00 | 0.98 / 0.73 / 0.90 | 35 | 12 | 17 | 19 |
| bamboo-clump | 62 | -0 / -0 / +0 | 0.55 / 0.79 / 0.55 | 1.00 / 1.00 / 0.99 | 0.55 / 0.80 / 0.56 | 24 | 11 | 74 | 48 |
| date-palm | 103 | +13 / -0 / -71 | 0.98 / 0.93 / 0.63 | 1.00 / 0.99 / 1.00 | 0.98 / 0.93 / 0.63 | 12 | 22 | 7 | 43 |
| english-oak | 107 | +0 / +0 / +0 | 0.99 / 0.75 / 0.90 | 1.00 / 0.97 / 1.00 | 0.99 / 0.77 / 0.90 | 17 | 73 | 52 | 59 |
| hornbeam-hedge | 60 | +21 / +18 / +0 | 0.68 / 0.76 / 0.86 | 1.00 / 1.00 / 1.00 | 0.68 / 0.76 / 0.86 | 1 | 1 | 1 | 18 |
| hosta-clump | 27 | +3 / -6 / -69 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 1.00 / 1.00 / 1.00 | 0 | 0 | 0 | 0 |
| ivy-on-trellis | 64 | +2 / -0 / +0 | 0.93 / 0.91 / 0.78 | 1.00 / 1.00 / 0.94 | 0.93 / 0.91 / 0.82 | 11 | 8 | 40 | 16 |
| japanese-maple | 47 | -0 / -0 / +0 | 0.99 / 0.72 / 0.74 | 1.00 / 0.95 / 0.99 | 0.99 / 0.75 / 0.75 | 30 | 19 | 14 | 27 |
| lombardy-poplar | 86 | +0 / -2 / +0 | 0.99 / 0.86 / 0.84 | 1.00 / 0.92 / 1.00 | 0.99 / 0.93 / 0.84 | 28 | 53 | 22 | 21 |
| london-plane-pollarded | 61 | +0 / +2 / +0 | 1.00 / 0.81 / 0.76 | 1.00 / 0.99 / 0.99 | 1.00 / 0.81 / 0.77 | 33 | 25 | 8 | 35 |
| miscanthus-grass | 202 | -0 / -4 / -1 | 0.38 / 0.31 / 0.75 | 1.00 / 0.99 / 0.97 | 0.38 / 0.31 / 0.76 | 82 | 119 | 39 | 19 |
| multi-stem-birch | 101 | -1 / +0 / +1 | 0.89 / 0.69 / 0.77 | 1.00 / 0.99 / 0.99 | 0.89 / 0.69 / 0.78 | 23 | 101 | 11 | 24 |
| scots-pine | 77 | +0 / -0 / +0 | 0.85 / 0.62 / 0.82 | 1.00 / 0.98 / 0.95 | 0.85 / 0.62 / 0.87 | 25 | 64 | 18 | 20 |
| small-leaved-lime | 78 | +0 / +1 / +0 | 1.00 / 0.78 / 0.85 | 1.00 / 0.88 / 0.97 | 1.00 / 0.88 / 0.87 | 53 | 23 | 16 | 41 |
| weeping-willow | 357 | +0 / -0 / -0 | 1.00 / 0.93 / 0.75 | 1.00 / 0.99 / 0.92 | 1.00 / 0.94 / 0.81 | 36 | 497 | 26 | 23 |

### vehicles

| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |
|---|---|---|---|---|---|---|---|---|---|
| cargo-bike | 40 | -0 / +3 / +0 | 0.98 / 0.95 / 0.85 | 1.00 / 0.98 / 0.92 | 0.98 / 0.97 / 0.93 | 5 | 5 | 5 | 11 |
| city-bicycle | 59 | +0 / +1 / +0 | 0.67 / 0.77 / 0.71 | 1.00 / 0.99 / 0.86 | 0.67 / 0.77 / 0.80 | 19 | 1 | 6 | 0 |
| city-bus | 81 | +0 / +0 / +0 | 1.00 / 0.99 / 1.00 | 1.00 / 0.99 / 1.00 | 1.00 / 1.00 / 1.00 | 27 | 50 | 2 | 17 |
| city-car | 54 | +1 / -0 / +0 | 1.00 / 0.91 / 0.96 | 1.00 / 0.97 / 0.98 | 1.00 / 0.93 / 0.98 | 24 | 25 | 1 | 11 |
| delivery-van | 26 | +17 / +0 / +0 | 0.97 / 0.96 / 0.99 | 1.00 / 0.99 / 1.00 | 0.97 / 0.98 / 0.99 | 11 | 10 | 6 | 1 |
| electric-kick-scooter | 16 | +0 / +1 / +0 | 1.00 / 0.89 / 0.97 | 1.00 / 1.00 / 1.00 | 1.00 / 0.89 / 0.97 | 1 | 2 | 3 | 0 |
| motor-scooter | 22 | +2 / +0 / +0 | 1.00 / 0.56 / 0.81 | 1.00 / 1.00 / 1.00 | 1.00 / 0.56 / 0.81 | 2 | 1 | 6 | 0 |
| tram | 44 | +0 / +0 / +0 | 0.99 / 0.98 / 0.99 | 1.00 / 1.00 / 1.00 | 0.99 / 0.98 / 0.99 | 19 | 12 | 6 | 9 |
| vintage-car-1929 | 93 | +0 / +0 / +0 | 1.00 / 0.82 / 0.95 | 1.00 / 0.98 / 0.99 | 1.00 / 0.83 / 0.96 | 35 | 44 | 4 | 5 |


## Known systematic weaknesses by asset family

- **Foliage and vegetation (all 16, only 1 passes both gates).** Hundreds of leaf, frond, needle and crown shapes are drawn per view and are not twins of each other: the elevation leaf is matched to a front/plan outline only through the bone, so the lateral span is a guess (over 1000 low-confidence flags). Front and plan IoU stay near 0.77 (agave 0.19/0.16: leaves on spread bones have `sweep: v` or tiny spans while the front/plan views draw them spread out; its staffage evaluation of the authored solids even fails, the draft then falls back to the flat outlines, flag `silhouette-degraded`). Palms and pines overshoot in plan (trunk solids matched to crown outlines). Expect to rebuild crowns as a few lofts or ellipsoid clusters instead of keeping every leaf.
- **Wheels, drums, rings and tyres.** Authored mostly as elevation discs with `extruded` hints (flat slab) while front and plan draw them as revolved discs or torus sections. The draft follows the elevation hint, so a bicycle or scooter wheel is a thin extruded disc: front IoU about 0.55 to 0.6 (motor-scooter 0.56). Re-author as `revolve` about the axle (`axis` along w) with a tyre profile; spokes are drafted as capsules and are fine.
- **Slats, blinds, tilted panels, garage-door sections.** Spans from the front/plan view are correct only when the plan twin has the same id; blind slats and door sections matched by bone overlap give wrong thicknesses (sectional-garage-door 0.42/0.94/0.44, external-venetian-blind 0.93/0.64). Check `low-confidence-lateral` first.
- **Tapered canopies, umbrellas, parasols, hoods, drum-like cones.** Revolved hints are kept, but the elevation half outline is revolved about its hinted axis; shapes drawn as polygons without hint become flat extrusions with a rectangular plan (market-parasol 0.37/0.46/0.88, kitchen-extractor-hood). Use `revolve` or `loft` with a `lateralScale` or an exponent below 8.
- **Tapered limbs.** `zTip` spans become 14-station lofts with exponent 8 (rounded rectangles); holes of the outline are dropped.
- **Thin tubes and chairs.** Open polylines become round capsules; chair and bench legs are drawn as tubes in elevation and as wide shapes in front, so the radius comes from the pen when no same-id twin exists (cafe-chair 0.46/0.71/0.17, folding-deck-chair, bar-stool front 0.52). Pairs of front legs drawn as one elevation line become one solid with `repeat`; verify the step.
- **Furniture tables and desks.** A table top drawn as left/right halves in plan gives clusters that merge into the leg shapes (dining-table front IoU 0.27: `leg-r` covers a whole table side). Height-adjustable-desk elevation 0.29.
- **Walls and wall-like components.** Layers are drawn as a stack of rectangles in elevation (section view); the draft makes one box per layer with the span of the matching plan layer, which works (components median IoU 1.00 on all three views), but layers drawn only once (plaster) can need a hand-set depth.
- **People.** Every human has many view-only parts (about 18 per asset: hands, shoes, face parts, held objects), drafted with an estimated forward extent (`needs-author`). Bounding-box error up to 3 % median but several above 8 % (arms or objects in the lateral direction).
- **Cranes and long structures.** Trusses are drawn with many thin diagonals; the elevation matches (1.00) but front IoU 0.20: diagonal members seen end-on and plan-only ballast/foot parts need their own solids.

## Advice for authors

1. Work per asset from `report.flags`: fix every `needs-author` first (no measured lateral data), then `low-confidence-lateral`, then the view-only parts (`unmatched`, provenance route `view-only-*`), then material guesses.
2. Replace aggregates by authored primitives: a hundred leaf extrusions should be a handful of lofts or ellipsoids; tyres, hubs, drums and canopies should be `revolve`; tubes and bars `capsule` or `member`-style boxes. Keep draft ids so the checker and the report still map.
3. Re-run `volume-check.ts` after each editing round: bounding box first (it is cheap and catches a wrong lateral span immediately), then the view with the lowest IoU; `worstSolids` are solids sticking out of the drawing, `worstShapes` are drawn parts your volume misses.
4. IoU against drawn shapes is not the target by itself: where the 2D drawing is schematic (hatching, stylised leaves, section stacks) trust `dimensions` and the three views' envelope and note the simplification in `decisions` (en and de).
5. The draft puts no `maxDenominator` and no `weights`; add `maxDenominator` to fine details and `weights` to blended skin-like parts as needed. Solid ids may carry `-l`/`-r`/`-n` suffixes from pairs; with `repeat` the second copy is the first shifted along w, so mirror-asymmetric parts must be split.
6. Bone-local conversions are already done: solids are in the rest bone frame (u along the bone, v normal, w lateral, left +). Hinted extrusions measure `z0..z1` from the world z of the bone origin; the draft subtracts that, so copy values from the draft, not from the 2D hint.
