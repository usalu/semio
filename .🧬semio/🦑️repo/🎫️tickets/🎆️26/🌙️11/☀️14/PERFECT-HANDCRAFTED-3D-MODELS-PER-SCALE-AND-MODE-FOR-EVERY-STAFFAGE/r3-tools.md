# Round 3 tools (`r3-*.ts`)

Run from the ticket folder `T`: `cd T; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`. `<asset>` = `category/slug`. Every tool reads the catalog asset with your source `T/volumes/<category>/<slug>.json` merged over it (as `volumes-render.ts` does, no splice needed) and writes only under `T/🗑️generated/r3-*/`. This file grows; re-read it. Defects and wishes: `T/tool-requests.md` (dated heading, "r3-" in the title).

## 1. `r3-sheet.ts` — six shaded views with the rig (available)

```
$BUN r3-sheet.ts <asset> --scale N [--list body|thermal|structure] [--pose ID] [--clip ID --t S]
                 [--only front|right|back|left|top|three-quarter] [--labels all] [--no-labels] [--no-rig] [--cut] [--turn DEG --tilt DEG]
```

- Output (path is printed): `T/🗑️generated/r3-sheet/<category>/<slug>.s<N>.<list>[.<pose>|.<clip>-t<S>][.<view>][.cut].png`, 2400 x 1600 px, about 1 s.
- Example: `$BUN r3-sheet.ts animals/horse --scale 50` writes `🗑️generated/r3-sheet/animals/horse.s50.body.png`; `$BUN r3-sheet.ts animals/horse --scale 50 --clip walk-cycle --t 0.3`; `$BUN r3-sheet.ts humans/standing-adult --scale 100 --pose pointing --list structure`.
- Panels, all at the same zoom: front (seen from +x = product view `front`), right side (from -z = product view `elevation`), back, left side, top (= product view `plan`, +x to the right), three-quarter from above (`--turn 50 --tilt 28` by default; turn 0 = right side, 90 = front, 180 = left side, 270 = back). The mirror sense is the product's (the 2D drawings), which is the opposite of the axonometric panels of `volumes-render.ts`.
- Shading: smooth where faces meet under 48 degrees, flat at creases; black line = outline or depth step, grey line = seam between two solids or crease. A seam you see is a real boundary between two solids of your list.
- Rig over the model: magenta line = bone from its joint (yellow dot) to its tip, label = bone id at mid bone in the front and right-side panels (`--labels all` for every panel; a label that finds no free place is dropped and counted in the panel title). A bone that is longer than the model (e.g. a vehicle `chassis` bone) is clipped by the panel.
- Grey line `Y = 0` = ground; scale bar 1 m (0.1 m when the asset is smaller than 0.8 m, 0.01 m under 0.08 m).
- `--list thermal`: colour per region and layer, legend below; `--cut` removes the half nearer to the viewer in every panel so hidden regions and nested shells show (body as pale ghost). `--list structure`: members blue, joints orange, supports green, masses magenta over the pale body ghost.
- `--pose` / `--clip --t` move solids and rig together (default: `presentation.defaultPose`). `--only <view>` gives that one view at the full sheet size with every label.
- A solid that fails to tessellate is listed on stderr as `NOT DRAWN` and counted in red on the sheet; the rest is still drawn.

## 2. `r3-validate.ts` — whole-asset validation (available)

```
$BUN r3-validate.ts <asset> [--volumes] [--catalog-only] [--json]
$BUN r3-validate.ts --file <asset document json> [--volumes] [--json]
```

- Validates the catalog asset with your `T/volumes` source merged over it: (a) the JSON schema `S/🧬️schema/🔣️.json` (`AssetDocument`, ajv strict) and (b) the product's `validateAsset` (references, rig, poses, clips, anchors, representations and print suitability, thermal, structure, the analytic volume rules, exports), with the whole catalog and the environment presets as context. About 1 s. No output file: every issue is one stdout line (`SCHEMA <json pointer>: ...`, `ASSET <code> <path>: <message>`), the last line is `VALID ...` or `INVALID ... : schema n, validateAsset n`; exit 0 / 1 (2 = usage).
- Example: `$BUN r3-validate.ts animals/horse` prints `VALID animals/horse (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes not run (--volumes)`.
- Root fixes (rig, dimensions, 2D shapes, poses, clips, anchors, thermal, structure): write your candidate asset document to `T/🗑️generated/r3-<slug>/🔣️.json` first and run `--file` on it (the file as it is, with its own `volumes`); write the catalog file only when that is `VALID`, then run the normal form again.
- `--catalog-only`: the catalog file without your source (what is spliced right now). `--volumes` adds the measured volume rules (`checkVolumes`, the same findings as `volumes-check.ts --product`, slow on large assets); the measurements with their numbers stay in `volumes-check.ts`.
- If `validateAsset` itself throws, the line `ASSET validator-threw` says so: nothing after that point was checked, fix that first.

## 3a. `r3-2d.ts` — the three 2D drawings alone, as the product draws them (available)

```
$BUN r3-2d.ts <asset> --scale N [--pose ID] [--clip ID --t S] [--file <asset document json>] [--views light-line,light-solid] [--svg]
```

- Output: `T/🗑️generated/r3-2d/<category>/<slug>.s<N>[.<pose>|.<clip>-t<S>].png` (2400 x 1434 px, about 5 s; `--file`: `🗑️generated/r3-2d/file/<folder name of the file>.s<N>.png`). Example: `$BUN r3-2d.ts animals/horse --scale 50` writes `🗑️generated/r3-2d/animals/horse.s50.png`.
- Columns: side elevation, front elevation, plan. Rows: `light-line` (every stroke with its pen and hatch) and `light-solid` (tones). These are the product's own drawings (`evaluate` + `depict` + SVG export, rasterised by librsvg), with its height dimension and scale bar; each panel is fitted to its frame, so the three are not at one common zoom (compare sizes through the dimension and the bar).
- It draws the CATALOG asset's 2D (your volumes source does not matter for 2D). To judge 2D root fixes before writing the catalog file, write the candidate document to `T/🗑️generated/r3-<slug>/🔣️.json` and pass `--file`.
- `--views` takes any of `dark-line,dark-solid,light-line,light-solid,thermal,forces,moments` (thermal and forces are drawn without a solved analysis, so only their geometry is meaningful here). `--svg` keeps the SVG of every panel next to the PNG. A panel that fails to draw shows the error in red and the exit code is 1.

## 3b. `r3-project2d.ts` — ruling E1: the 3D body projected into the frame of the 2D shapes (available)

```
$BUN r3-project2d.ts <asset> --scale N --view elevation|front|plan [--bone ID] [--merge] [--check] [--candidate] [--quiet]
```

- Per bone: the orthographic outline of that bone's `body` solids of your source in `presentation.defaultPose` (solids with `texture: true` and `slender: true` left out), cleaned for the scale (closing and opening by a quarter of the smallest feature, Douglas-Peucker at a quarter of the smallest feature, nothing under one feature squared), printed as one paste-ready shape JSON per line: the members of the existing shape it goes over (`id`, `bone`, `role`, `z`, `filled`, `tone`, `region`, `material`, `hatch`, `penMm`) plus the new geometry (`kind: "poly"`, `points`, `closed: true`; an outline with holes is one `path` with a subpath per ring).
- Output: stdout and `T/🗑️generated/r3-project2d/<category>/<slug>.s<N>.<view>[.<bone>].json` (`bones[].shapes`, `replaces`, `index`, `roundTripIou`, `frameResidualM`, `notes`). Example: `$BUN r3-project2d.ts animals/horse --scale 50 --view elevation --bone thorax` prints `## bone thorax: solids chest -> over shape "chest" (poly, index 27 of geometry.elevation); 1 outline(s), 26 points; round trip IoU 0.9988, frame residual 0.000 mm` and the shape line. 1 to 10 s per view.
- Which shape an outline goes over: a solid whose id equals the id of a filled closed body/detail/glazing shape of the same bone in that view gets its own outline over that shape; all other solids of the bone are merged into the outline over the bone's largest `body` shape. `--merge` gives one outline per bone. Other filled shapes of the bone are listed in a note: redraw, keep or remove them yourself. A bone without such a shape gets a `NEW shape` (id `r3-new-<bone>`; choose id, z, tone, region). Detail, texture, glazing strokes and open strokes are never touched: redraw them by hand where the form moved.
- The frame (stated in the second output line) is the product's own: the tool places probe shapes through `evaluate` and solves the outline through that placement, so nothing is re-derived. Elevation: bone-local (x along the bone from its joint, y across). Front: x = lateral (world z), y = height of the REST pose, carried to the pose by the bone. Plan: x = ahead, y = lateral of the REST pose. Front and plan shapes are carried at the depth of their elevation twin (same id and bone), so do the elevation first, write it, then front and plan.
- Proof per bone in the header: `round trip IoU` (the printed shape placed again by `evaluate` against the bone's projected solids; 0.98 to 1.0 on ordinary parts, lower only on parts of a few features because of the simplification) and `frame residual` (largest distance of a placed point from its target, 0.000 mm when solved). `--check` replaces every filled closed body/detail/glazing shape of the view in memory and prints the product's silhouette numbers. Measured 2026-11-19: horse 1:50 `iou` 0.998 / 0.998 / 0.998 (elevation / front / plan), standing-adult 1:50 0.994 / 0.994 / 0.995 and 1:200 0.978 / 0.984 / 0.977, city-car 1:50 0.999 / 0.999 / 1.000 and 1:500 0.991 / 0.994 / 0.991. In front and plan `--check` keeps the `solid`-generated shapes as they are, so an `iou` clearly under these values there points at a `solid` profile or span that no longer fits the new body (horse front fell to 0.970 while its author was changing the ears and tail).
- Shapes with a `solid` member (extruded or revolved profile): in the elevation an extruded profile swept along w on a bone that stays in the paper plane is replaced point by point and keeps its `solid` (note: set `z0`, `z1`, `zTip` to the new lateral span yourself); any other `solid` shape is printed for REFERENCE ONLY (note says so). In front and plan the product generates `solid` shapes from the elevation profile and the span, so they are never pasted over: correct the elevation profile and the span. Do not remove `solid` to make a paste fit: `validateAsset` then reports `lateral-ambiguous`.
- `followTip` shapes: the pasted poly drops `followTip` and is exact for the default pose only (note says so); check another pose with `r3-2d.ts --pose`.
- `--candidate` writes the catalog document with these replacements to `🗑️generated/r3-project2d/<category>/<slug>.s<N>.<view>/🔣️.json`: look at it with `$BUN r3-2d.ts --file <that file> --scale N` and check it with `$BUN r3-validate.ts --file <that file>`. It is a starting point for looking, not a result: a dumped outline still has to be drawn (economical points, the line hierarchy, the overlaps between neighbouring bones that keep poses closed, regions that must stay drawn; `validateAsset` reports e.g. `region-undrawn` or the extent rules when the dump removed something).
- Sections: where the representation declares `sections[view]` the body is cut at that plane first (C12).

## Notes for all r3 tools

- Shared code: `r3-lib.ts` (loading, rig of a pose or clip frame, shaded panels). Import it if useful, never edit it; requests go to `T/tool-requests.md`.
- The sheets and the 2D panels are 2400 px wide; the image viewer shows them slightly reduced. For small labels or details use `r3-sheet.ts --only <view>`.

## 2026-11-20 16:55 — `r3-validate.ts` also checks print suitability

Besides schema and `validateAsset`, it now runs the product's `validateRanges` and `measureSuitability` against the default print profile for every authored representation. New lines start with `PRINT` (`PRINT unsuitable $.representations.<scale>` = the product refuses to draw that scale; `PRINT range-end-feature ...` = the declared valid range ends below a printable feature). They count as `validateAsset` issues, so `VALID` now means the product also draws every scale.
