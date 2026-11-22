# Work package `legacy-layers`: one airflow vocabulary

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `V` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`.

## Result

The older transport overlay is gone from the product. The flow view is the only drawing of air, with five layers: `streams`, `tracers`, `moisture`, `fog`, `pathlines`. Casts, the course manifest, the Python bridge, the preview CLI, the docs and the tests speak only that vocabulary.

| Old | Now |
|---|---|
| dashed streamlines with one arrow head (`〰️streamlines`, `TransportField.streamlines`) | `streams` |
| parcel discs with hatch (temperature or vapour) | `moisture` (vapour clouds, dotted lines) and the temperature colour of `streams`, `tracers`, `pathlines` |
| hollow cyan condensation discs (`TransportField.condensation`) | `fog` (a parcel with `condensateKgPerKg > 0` is a fog puff) |
| pathlines of the overlay | flow layer `pathlines` (new) |
| emitter arrow markers and flow labels | removed, nothing replaces them |
| depiction options `transport`, `transportStrokeMm`, `transportTrailS`, `transportDensity` | `flow`, `flowDevelopS`, `flowDetail.{spacingMm, trailS, strokeMm, …}` |
| cast options `transport`, `transportAgeS`, `transportStrokeMm`, `transportTrailS` | `flow`, `flowDevelopS`, `flowTrailS`, `flowStrokeMm`, `flowSpacingMm` |
| level of detail `transportDensity` (every 4th parcel) | `flowSpacingFactor` (1, 2, 2: the spacing of the flow marks) |
| provenance `scenario.transport` | `scenario.flow` |

`solveTransport` keeps parcels and pathlines (the model).

## The `pathlines` layer

- Contract (`S/🔨️modules/🔗️contracts/🟦️.ts`): `FlowPath` (`FlowField.paths`: points, parcel age per point, what the parcel carries now), `FlowTrail` (`FlowMarks.trails`: kept points, `agoS`, `fade`, `visible`, `excessK`), `FLOW_LAYERS` with `pathlines`, `FlowMarkDetail.trailS` and `.strokeMm`. `TRANSPORT_LAYERS`, `TransportLayer`, `Streamline`, `TransportField.streamlines` and `.condensation` and the four `transport*` options are deleted.
- Field (`S/🔨️modules/💨️transport/🌊️flow/🟦️.ts`): `flowAt(…, pathlines)` fills `paths` with the pathlines of the parcels inside the domain; `trailOf` (whole path, or the last `trailS` seconds with a linearly falling fade and an interpolated tail point), `flowTrails` (thinned by the rule of the streams at the spacing of the scale), `flowMarks(…).trails` with the occlusion of the eye like the other marks; `FLOW_SPACING_MM`; the `Sink` type moved here.
- Paper (`S/🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts`): thin solid line in the colour of the parcel's temperature excess, four opacity steps along a trail, hidden behind the body; legend "Parcel paths" / "Teilchenbahnen" (s) and the temperature legend also for paths; `projector` lives here now; `flowPaint(…, strokeMm)` with `strokeScale` (streams, tracers, vapour lines and paths scale with it).
- Depiction (`S/🔨️modules/🎨️depiction/🟦️.ts`): transport block of `depictSteps`, `depictAll`, default options without `transport`; the transport request solves pathlines only when the layer is asked for.
- Screen (`S/🎯️targets/⚛️react/🔨️modules/🌊️flow/🖌️overlay/🟦️.ts`): draws the trails (the session, the address code `p` and the texts were already wired by a sibling package).

Deleted: `S/🔨️modules/🎨️depiction/💨️transport-view`, `S/🔨️modules/💨️transport/〰️streamlines`, `S/🧪️tests/💨️transport-view`.

## Casts

- Schema first: `S/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json` (`FlowLayer`, the five cast options, rules: flow needs `flowDevelopS` and the thermal view; options need flow; `flowTrailS` needs `pathlines`), twin `🟦️.ts` with the same messages, `🧊️lod/🔣️.json` + schema + `🟦️.ts`, `S/🔨️modules/🏷️revision/🟦️.ts`.
- Exporter (`…/🎭️cast/🟦️.ts`): passes `flow`, `flowDevelopS`, `flowDetail` to the depiction and develops the air once before the first frame (`developFlow`, cancellable). One semantic with the viewer and the CLI: the flow of a frame is solved `flowDevelopS` after the frame time; a looping clip moves on by whole periods. A pose can now carry flow (the old overlay refused poses).
- Index: `maxPaths`, `logical`, `figureBoundsM`, `partsM`, `heightM` computed as before; `clock.offsetS` stays in the contract and is always 0 now (frames start at clip time 0).
- Course manifest (`V/🧍️staffage/🔣️.json`), four rows by hand: `shared-person-pathlines`, `shared-office-worker-pathlines`, `ventilation-volume-cook` → `["streams", "moisture", "pathlines"]`, `flowDevelopS` 5, `flowTrailS` 1, `flowStrokeMm` 0.4; `physics-fundamentals-kettle` → `["streams", "moisture", "fog"]`, `flowDevelopS` 4, `flowStrokeMm` 0.4. Cast ids are unchanged, so no course scene changes.
- Python: `V/🧍️staffage/🐍️.py` (`_FLOW_LAYERS`, `_FLOW_OPTIONS`, validator messages, provenance `flow`), `V/🧪️tests/🐍️course_staffage.py`, `🐍️media_staffage.py`, `🐍️scenes.py`, `🔣️scenes.json` (`flow_figure`).
- Other uses: `🖨️preview` (`--flow`, `--flow-develop`), `🔭️multiview`, `🧊️volume/🟦️.ts` (removed option), `📦️packages/🟦️typescript/🟦️.ts`, `S/README.md` (Air flow, cast row table, levels of detail).

## Verified

Commands as in the brief (vitest with the staffage config; `tsc --noEmit`).

| What | Result |
|---|---|
| `tsc` library package | 0 errors |
| `tsc` React package | only the known errors in `🧰️framework/🔨️modules/🖱️ui` |
| `flow-view` | 19 of 19 (9 new cases: paths as projected parcel histories in three viewpoints, hand-derived trail with d3-scale as oracle, occlusion by a ball, stroke, legends, second reading, ranges) |
| `thermal-multiview` | 23 of 23, including the numpy reference |
| `transport-field` | 21 of 21 (streamline and condensation cases now on flow streams, paths and fog puffs) |
| `flow-field`, `flow-obstacles`, `parcel-jet`, `condensation`, `vapour-pressure`, `phase-change`, `seven-modes`, `cast-provenance`, `schema-twin`, `schema-conformance` | green |
| `cast-export` | 75 of 76; the one failure is the course manifest against the catalog (`heat-pump-outdoor-unit` has no clip `heating-sweep`) |
| `cast-lod` | 21 of 23; the 2 failures are the catalog (`heat-pump-outdoor-unit` has no clip `heating-sweep`), not this package. The four level revisions were recomputed independently with Python `hashlib` and match |
| `course-coverage` | 61 of 62; the failure is the course against the catalog under re-authoring (`row-mismatch` ×30, `clip-unknown` ×1), no flow code |
| `transport-assets` | 6 of 7; `keeps parcels continuous over the loop boundary` fails on parcel positions (0.0388 > 0.0345), which this package does not touch (only the removed `streamlines` option was dropped from the calls) |
| React `flow` | green; `flow-downloads` (a sibling's new suite) has 2 failures about the field time in the legend text |
| Python `🐍️media_staffage.py` (with `jsonschema`) | 2 of 2 |
| Python `🐍️course_staffage.py` | 4 of 5; same catalog failure as above |
| Python bridge without Manim (`legacy-layers/bridge-probe.py`) | validator refuses all 38 invalid casts of the shared fixture with the twin's messages, accepts the 13 valid and the 98 course casts; the 4 level revisions match |

Cast frames looked at (exported into `🗑️generated`, since deleted): `shared-person-pathlines` poster (grey plume streams with arrow heads over the head, teal breath vapour, short path trails at the nose and above the head), `physics-fundamentals-kettle` preview frame 85 of 90 (white fog cloud with cyan rim leaning with the room air, dotted vapour lines, entrained streams), `ventilation-volume-cook` poster (19 streams, 53 vapour clouds, 42 path runs).

## Final run (15:40–15:46)

The dependency image of the repo (`.cache/b/d/fixed.dmg`, the target of the root `node_modules` link) was detached by something outside this package between about 15:26 and 15:40; the closing run was done after it came back.

- `tsc --noEmit`: library package 0 errors; React package only the known errors in `🧰️framework/🔨️modules/🖱️ui`.
- One vitest run of `flow-view`, `thermal-multiview`, `transport-field`, `cast-export`, `cast-lod`, `cast-provenance`, `course-coverage`, `seven-modes`, `multiview-coverage`, `schema-twin`, `schema-conformance`: 344 of 350 tests, 7 of 11 files green. The six failures are none of this package:
  - catalog under re-authoring: `cast-export` "accepts the course cast manifest … in the catalog" and `cast-lod` ×2 (`heat-pump-outdoor-unit` has no clip `heating-sweep`), `course-coverage` "resolves every course cast" (`row-mismatch` ×30, `clip-unknown` ×1);
  - rig and structure: `multiview-coverage` "keeps planted contacts still" (0.080 > 0.072 m) and "reads one magnitude per region and member" (pelvis bending moment differs between elevation and front). Its flow case is green.
- All air cases of `cast-export` (flow layers, develop time, stroke, trail, spacing, pose with flow, refused retired options) are green after the trail occlusion.
- Sweep for the retired names (`streamlines`, `condensation`, `transportStrokeMm`, `transportTrailS`, `transportDensity`, `transportAgeS`, `TRANSPORT_LAYERS`, `TransportLayer`, `transport-view`) over `S` and `V`: no option, layer or module is left. The remaining hits are the physics word condensation, "streamlines" as prose for what a stream is, and the negative vectors that pin the retired names as refused.

## Not done, and findings

- Python tests that import Manim could not run on this host (no Manim environment, pycairo): `V/🧍️staffage/🧪️tests/🐍️.py`, `V/🧍️staffage/🧪️tests/🐍️lod.py`, `V/🧪️tests/🐍️scenes.py`, `V/🧪️tests/🐍️staffage_only.py`. The course was not rendered.
- With `thermalSpanK` the flow colours use that span too (`flowPaint`, mode `span`): at ±25 K a body plume of 1–2 K reads grey in the course casts. A separate span for the air would be a decision for the flow view.
- At a whole-body domain the arc step of the streams (up to 10 cm) is coarser than the seeds around an inhaling nose (4 diameters), so no inhaled streams are traced there; the old streamlines used 2 cm. The test pins them at `stepM` 0.015.
- A cast clip that does not loop shows its air `flowDevelopS` ahead of the figure (the depiction's rule); the kettle uses 4 s against a boil of 200 s.
- The looping casts now sample from clip time 0 instead of `transportAgeS`; where that was no whole number of frames (3.5 s at 15 fps) the sampled poses shift by half a frame, so `figureBoundsM` of a breathing figure can differ in the sub-millimetre range.
- `S/🔨️modules/📤️export/🎬️cli/🟦️.ts` (package `exports`) accepts `pathlines` through `FLOW_LAYERS`, but its doc string still lists four layers.

## Scripts kept in `legacy-layers/`

`pathline-fixture.py` (derives the trail vector with exact fractions and compares it with the fixture), `bridge-probe.py`, `arrow-probe.ts`, `inhale-probe.ts`. `🗑️generated` is deleted.

## Follow-up: `export-performance`, pinned digests of `walker-line`

Finding: the bytes moved because the asset `walking-commuter` was re-authored by the other session, not because of any code of this ticket. The test is left alone; no pin was touched.

The cast `walker-line` of `S/🧫️fixtures/🎭️cast-export` draws the catalog asset `S/🗂️catalog/👤️humans/walking-commuter/🔣️.json` (not `🧫️fixtures/🚶️reference-walker`). The pins (`castDigests` of `S/🧫️fixtures/🚀️export-performance/🔣️.json`) were last committed on 2026-11-18 (`3d62b2ef22c`); the asset changed in the commits of 11-20 (`5db034ffe53`), 11-21, 11-22 13:42 and again in the working tree at 16:12.

Proof (`legacy-layers/walker-digest.ts`: exports `walker-line` against a catalog root and compares every frame with the pins by the test's own normalisation; the catalog root is the live one with only this asset file replaced by `git show <commit>:…`):

| Exporter, depiction, SVG writer | Asset `walking-commuter` | 18 frames against the pins |
|---|---|---|
| current working tree (all edits of this ticket) | as of `3d62b2ef22c` (when the pins were made) | all equal |
| current working tree | as of `5db034ffe53` (11-20) | all differ |
| current working tree | as of `b4927d2c311` (11-21), `HEAD` (11-22 13:42), working tree (16:12) | all differ |
| `HEAD` code extracted with `git archive` (13:42, before this package's first edit of the exporter, depiction and contracts) | `HEAD` and working tree | all differ, and byte-identical to the current code for the same asset (frames 0, 5, 17 compared with `cmp`) |

So the current code reproduces the pinned bytes exactly when it is given the asset the pins were made from, and the committed code of before this package gives the same bytes as the current code. What changed in `0000.svg` is the figure itself: from the pinned asset to 11-20 the shape set changed (126 → 104 groups: `shape-coat-lapel`, `shape-coat-button-1…3`, `shape-thumb-l`, `shape-ear`, `shape-sleeve-cuff-l` gone; `shape-collar`, `shape-wrist-l`, `shape-shirt-cuff-l`, `shape-shoe-sole-r` and others new; 62 further groups with new coordinates) and the shared viewBox went from 42.085 × 37.1998 mm to 42.3092 × 37.921 mm; since then further outlines changed (100 groups, viewBox 42.3092 × 37.9281 mm). `index.frames` and the file list still match because the clip and its clock are unchanged.

The pins of `walker-line` (and likely of `walker-thermal`, the same asset) need regenerating by whoever finishes the re-authoring of `walking-commuter`, once the asset is final; doing it now would pin a moving target.
