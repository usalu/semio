# Round 3 tools (`r3-*.ts`)

Run from the ticket folder `T`: `cd T; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`. `<asset>` = `category/slug`. Every tool reads the catalog asset with your source `T/volumes/<category>/<slug>.json` merged over it (as `volumes-render.ts` does, no splice needed) and writes only under `T/🗑️generated/r3-*/`. This file grows; re-read it. Defects and wishes: `T/tool-requests.md` (dated heading, "r3-" in the title).

## 1. `r3-sheet.ts` — six shaded views with the rig (available)

```
$BUN r3-sheet.ts <asset> --scale N [--list body|thermal|structure] [--pose ID] [--clip ID --t S] [--file <asset document json>]
                 [--only front|right|back|left|top|three-quarter] [--labels all] [--no-labels] [--no-rig] [--cut] [--turn DEG --tilt DEG]
```

- Output (path is printed): `T/🗑️generated/r3-sheet/<category>/<slug>.s<N>.<list>[.<pose>|.<clip>-t<S>][.<view>][.cut][.turn<DEG>-tilt<DEG>][.no-rig][.no-labels][.labels-all].png`, 2400 x 1600 px, about 1 s. Every option that changes the picture is in the name, in the catalog form and in the `--file` form alike (checked 2026-11-22: `--pose`, `--clip --t`, `--list`, `--only`, `--cut` were already carried by `--file`; `--turn/--tilt`, `--no-rig`, `--no-labels`, `--labels all` are new in the name), so two renders of one candidate never overwrite each other. An unknown `--pose` or `--clip` is refused with the list of ids the document has (it is not drawn in another pose).
- zsh does not split a variable into words: `ARGS="animals/horse --scale 50"; $BUN r3-sheet.ts $ARGS` passes ONE argument and the tool answers `scale undefined …`. Write the arguments out, use an array (`ARGS=(animals/horse --scale 50); $BUN r3-sheet.ts $ARGS`) or `${=ARGS}`.
- `--file <asset document json>` renders a candidate document as it is (its own rig, poses, clips, anchors and `volumes`; nothing is merged from `T/volumes`), with the semantics of `r3-2d.ts --file` and `r3-validate.ts --file`: write the candidate to `T/🗑️generated/r3-<slug>/🔣️.json`, look at rig and pose changes there, and write the catalog file only when it is right. Output: `🗑️generated/r3-sheet/file/<folder name of the file>.s<N>.<list>….png` with the same suffixes as the catalog form (two candidates need two folder names: the name is taken from the folder of the file).
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
$BUN r3-validate.ts <asset> [--volumes [--scale N] [--rules a,b]] [--no-print] [--catalog-only] [--json]
$BUN r3-validate.ts --file <asset document json> [--volumes [--scale N] [--rules a,b]] [--no-print] [--json]
```

- **Fast inner loop (2026-11-22).** `--scale N` runs `checkVolumes` for the volumes of 1:N only (the cross-scale rules then compare 1:N with the next finer scale only), `--rules a,b` only the named rules (with or without the prefix `volumes-`: `silhouette`, `bounds`, `monotonic`, `cross-scale`, `thermal-coverage`, `structure-coverage`, `connected`, `feature`, `slender`, `bones`, `solids`, `copied`, `views-disagree`, `availability`); both imply `--volumes`. `--no-print` skips the `PRINT` lines, the print measurement being most of the time of a run without `--volumes` (16 of 17 CPU seconds on `animals/horse`). Example: `$BUN r3-validate.ts vehicles/city-car --scale 500 --rules connected,silhouette --no-print` takes 2 s instead of 2 minutes; `--scale 50 --rules thermal-coverage --no-print` proves one thermal list. The summary line then says `PARTIAL: checkVolumes limited to …` (and `PRINT checks skipped`), the JSON has `"partial": true`, a scale without authored volumes is the issue `VOLUMES no-such-scale`. A partial run is a working aid: the proof of an asset stays the full `$BUN r3-validate.ts <asset> --volumes`.
- Measured cost of the full `--volumes` run on an idle core after the speed work of 2026-11-22 (CPU seconds of `checkVolumes`, before → after): `vehicles/city-car` 113 → 68 (thermal 1:50 45 → 22), `humans/standing-adult` 23 → 20, `animals/horse` 14 → 12, `appliances/gas-boiler` 7 → 5; under the load of many concurrent validations the wall time is several times that, so prove one scale and one rule while you work and run the full form once at the end.

- Validates the catalog asset with your `T/volumes` source merged over it: (a) the JSON schema `S/🧬️schema/🔣️.json` (`AssetDocument`, ajv strict) and (b) the product's `validateAsset` (references, rig, poses, clips, anchors, representations and print suitability, thermal, structure, the analytic volume rules, exports), with the whole catalog and the environment presets as context. About 1 s. No output file: every issue is one stdout line (`SCHEMA <json pointer>: ...`, `ASSET <code> <path>: <message>`), the last line is `VALID ...` or `INVALID ... : schema n, validateAsset n`; exit 0 / 1 (2 = usage).
- Example: `$BUN r3-validate.ts animals/horse` prints `VALID animals/horse (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes not run (--volumes)`.
- Root fixes (rig, dimensions, 2D shapes, poses, clips, anchors, thermal, structure): write your candidate asset document to `T/🗑️generated/r3-<slug>/🔣️.json` first and run `--file` on it (the file as it is, with its own `volumes`); write the catalog file only when that is `VALID`, then run the normal form again.
- `--catalog-only`: the catalog file without your source (what is spliced right now). `--volumes` adds the measured volume rules (`checkVolumes`, the same findings as `volumes-check.ts --product`, slow on large assets); the measurements with their numbers stay in `volumes-check.ts`.
- If `validateAsset` itself throws, the line `ASSET validator-threw` says so: nothing after that point was checked, fix that first.

## 3a. `r3-2d.ts` — the three 2D drawings alone, as the product draws them (available)

```
$BUN r3-2d.ts <asset> --scale N [--pose ID] [--clip ID --t S] [--file <asset document json>] [--views light-line,light-solid] [--svg]
```

- Output: `T/🗑️generated/r3-2d/<category>/<slug>.s<N>[.<pose>|.<clip>-t<S>][.<view>+<view>…].png` (2400 x 1434 px, about 5 s; the last part only with `--views`; `--file`: `🗑️generated/r3-2d/file/<folder name of the file>.s<N>…png` with the same suffixes, so two poses of one candidate are two files; an unknown `--pose` or `--clip` is refused with the ids the document has). Example: `$BUN r3-2d.ts animals/horse --scale 50` writes `🗑️generated/r3-2d/animals/horse.s50.png`.
- Columns: side elevation, front elevation, plan. Rows: `light-line` (every stroke with its pen and hatch) and `light-solid` (tones). These are the product's own drawings (`evaluate` + `depict` + SVG export, rasterised by librsvg), with its height dimension and scale bar; each panel is fitted to its frame, so the three are not at one common zoom (compare sizes through the dimension and the bar).
- Front and plan shapes are drawn in rest coordinates with the root at the origin (see `r3-project2d.ts` below for the frame of each view): moving the root joint against the body shifts every front and plan shape by that amount against the elevation, which `volumes-views-disagree` reports since 2026-11-22 (`validator.md`, item 15).
- It draws the CATALOG asset's 2D (your volumes source does not matter for 2D). To judge 2D root fixes before writing the catalog file, write the candidate document to `T/🗑️generated/r3-<slug>/🔣️.json` and pass `--file`.
- `--views` takes any of `dark-line,dark-solid,light-line,light-solid,thermal,forces,moments` (thermal and forces are drawn without a solved analysis, so only their geometry is meaningful here). `--svg` keeps the SVG of every panel next to the PNG. A panel that fails to draw shows the error in red and the exit code is 1.

## 3b. `r3-project2d.ts` — ruling E1: the 3D body projected into the frame of the 2D shapes (available)

```
$BUN r3-project2d.ts <asset> --scale N --view elevation|front|plan [--bone ID] [--merge] [--check] [--candidate] [--quiet]
```

- Per bone: the orthographic outline of that bone's `body` solids of your source in `presentation.defaultPose` (solids with `texture: true` and `slender: true` left out), cleaned for the scale (closing and opening by a quarter of the smallest feature, Douglas-Peucker at a quarter of the smallest feature or finer on small parts, nothing under one feature squared unless it is part of a larger body; see "2026-11-22 — r3-project2d at 1:500" below), printed as one paste-ready shape JSON per line: the members of the existing shape it goes over (`id`, `bone`, `role`, `z`, `filled`, `tone`, `region`, `material`, `hatch`, `penMm`) plus the new geometry (`kind: "poly"`, `points`, `closed: true`; an outline with holes is one `path` with a subpath per ring).
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

## 2026-11-21 — `valid-range-gap`: never narrow a band into a hole (R01, design E2)

`validateAsset` (so `r3-validate.ts`) now reports `ASSET valid-range-gap representations.<scale>.validRange` when the valid ranges of the authored representations leave a hole in the promised bands of the authored presets (1:50 → 1:20…75, 1:100 → 75…150, 1:200 → 150…350, 1:500 → 350…1000; `SCALE_BANDS` in `S/🧬️schema`). Inner boundaries between two authored neighbours may move; the finest start and the coarsest end may not shrink. A `PRINT range-end-*` finding is fixed by redrawing the named shapes (and their 3D solids at that scale) so they are at least the profile minimum at the band end (0.2 mm on paper: 70 mm real at 1:350, 200 mm at 1:1000), never by narrowing. Sweep: `$BUN r5-range-gaps-sweep.ts`; what a widened band would violate: `$BUN r5-range-gaps-probe.ts <asset>`.

## 2026-11-22 — r3-project2d at 1:500

Outlines of `r3-project2d.ts` at 1:500 could fail `volumes-silhouette` on a valid body (reported by rework batch 07). Root cause: the cleaning for the scale judged every bone alone, while the product compares the whole body. Three effects, all in `outline`:

- A bone under one feature squared (cannon, hoof, ear of a goat at 1:500: feature 75 mm) was dropped, so the lower legs were missing from the drawing; the product still counts their solids. Now an outer ring under one feature squared is dropped only when the island of the whole body it lies in (all non-texture, non-slender body solids of the view, closed by a quarter feature) is under one feature squared too. Such a kept outline carries the note `N outline(s) under one feature squared kept: part of the body silhouette the product compares`.
- The opening by a quarter feature removed limbs thinner than half a feature from a bone that also had a thicker part. Now a piece the opening took away is put back when it covers a quarter of a feature squared or more; smaller pieces (spurs, raster slivers) still go.
- Douglas-Peucker at a quarter feature (18.75 mm at 1:500) cut parts of one or two features down to 4 points (round trip IoU 0.41 to 0.75), and a part thinner than the tolerance collapsed to a line, in which case the unsimplified raster staircase was printed (118 points). Now a ring that keeps less than 0.95 of its traced area (IoU, `PART_IOU`) or collapses is simplified within half the tolerance, and so on down to half a raster cell. Parts that pass at a quarter feature are printed exactly as before (1:50, 1:100 outlines of ordinary parts do not change); small parts at 1:200 and 1:500 get more points.

Measured with `--scale 500 --check` (`iou` / `p95M`, elevation, front, plan; limits `iou` 0.9 / 0.88 / 0.88, `p95M` 0.150 / 0.300 / 0.300 m):

| asset | before | after |
|---|---|---|
| `animals/goat` | 0.9152 / **0.2223 m fails**; **0.8046 fails** / 0.1233; 0.9049 / 0.1213 | 0.9628 / 0.0051; 0.9425 / 0.0170; 0.9591 / 0.0051 |
| `animals/horse` | 0.9762 / 0.0114; 0.9492 / 0.0091; 0.9773 / 0.0000 | 0.9829 / 0.0114; 0.9598 / 0.0091; 0.9849 / 0.0000 |
| `animals/dairy-cow` | 0.9724 / 0.0698; 0.9609 / 0.0283; 0.9408 / 0.2193 | 0.9843 / 0.0698; 0.9752 / 0.0283; 0.9500 / 0.2193 |
| `humans/standing-adult` | 0.9466 / 0.0067; 0.9278 / 0.0336; 0.9118 / 0.0157 | 0.9693 / 0.0067; 0.9477 / 0.0202; 0.9427 / 0.0138 |

After only: `vehicles/city-car` 0.9965 / 0.9904 / 0.9980, `animals/sheep` 0.9709 / 0.9608 / 0.9670, horse 1:50 elevation 0.9974. Goat front round trip per bone: shank 0.79 to 0.85, cannon 0.73 to 1.00, thigh 0.88 to 0.94, head 0.84 to 0.95; a foot of pastern and two claws stays near 0.5 because the closing fills the cleft, which is intended at this scale.

Still true: the check replaces the whole view. Pasting only some bones next to hand-drawn neighbours is measured by `r3-validate.ts --file <candidate> --volumes`, not by `--check`; a kept sub-feature outline is a correct silhouette, not yet an economical drawing (merge it into its neighbour by hand where the pose allows).

## 2026-11-22 afternoon — rig tools (`rig-rest.ts`, `rig-lateral-compare.ts`)

- `$BUN rig-rest.ts <asset> --solve BONE --parent ID --at "x,y,z" --u "x,y,z" [--w "x,y,z"]` solves the rest placement of a bone that is NOT in the rig yet (checked: `--solve rib-new-l --parent torso --at "0.02,1.3,0.03" --u "0,0,1" --w "-1,0,0"` on `humans/standing-adult` prints the paste-ready `parent, x, y, z, angleDeg, abductionDeg, twistDeg`). `--parent`, `--at` and `--u` are required for a new bone; `--w` defaults to the joint axis of the parent and must be given when the bone runs along that axis (a lateral bone under a sagittal parent: `--w "-1,0,0"`). Alternatively put the bone into a candidate document and pass `--file`. Details: `rig-lateral-rest.md` 1.2.
- `$BUN rig-lateral-compare.ts <before json> <after json>` now pairs the placed 2D shapes by id and bone, never by index: a changed depth order is no move (checked on `animals/goat` against itself with every view list reversed: `largest move 0.0000 mm`). Shapes or anchors that only one document has are listed (`only before: …`, `only after: …`). A scale that is not `authored` in both documents is skipped and named (`1:200 skipped: not authored in both documents`) instead of the error `has no available representation at 1:200` (checked on `animals/red-squirrel`).
- `rework-07-2d.py` no longer runs `main()` at import (`if __name__ == "__main__":`), so `splice` and the other helpers can be imported.
