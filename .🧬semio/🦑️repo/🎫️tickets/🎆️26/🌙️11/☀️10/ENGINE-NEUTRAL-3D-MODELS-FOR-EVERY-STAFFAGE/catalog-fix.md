# Catalog fix: pre-existing data defects of the staffage catalog

Ticket `ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE`. `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder. Complete audit list: `catalog-audit.md`. Scripts kept in `T`: `catalog-audit.ts`, `catalog-sweep-probe.ts`, `catalog-sweep-asset.ts`, `catalog-sweep-apply.py`, `catalog_edit.py` (surgical object edits, roundtrip-checked, never re-serialises a file), `catalog-fix-shapes.py`, `catalog-fix-walls.py`, `catalog-rename-ids.py`, `catalog-fix-solid-ids.py`, `catalog-agave-union.ts`, `catalog-dims.ts`, `catalog-model-check.ts`. All edits to catalog files are exact-text insertions, removals or replacements of the affected member or object; no catalog file was re-serialised.

## Result

| | before | after |
|---|---|---|
| Ajv errors over 152 assets + 4 asset fixtures (`#/$defs/Asset`) | about 6500 distinct rows (15 228 raw `pattern` rows alone in vegetation), 156 documents of which 45 failed | 0, all 156 documents conform (also with `volume` now required) |
| `validateAsset` findings | `solid-sweep` 365 plus 195 others | 25 in 5 assets (open issues below) |
| React `catalog-loading` ("one ready row per manifest entry") | 19 invalid assets | 5 invalid (the assets of the open issues) |
| `modelOf` + `validateModel` + `tessellateSolid` over all 152 assets | | 152 of 152 pass |

## B. Missing `sweep` (defect 1)

Decision per hint, by geometry. For every extruded hint without `sweep` (351 in the catalog: furniture 256, objects 95; plus 27 in four fixtures) both candidate outlines were evaluated (`catalog-sweep-probe.ts`): in the elevation against the shape's own flat outline (solid removed), in front and plan against the elevation twin of the same id or the elevation shapes of the same bone, and for the whole asset (`catalog-sweep-asset.ts`) against `dimensions` and the silhouettes of the authored `volume` (an independent 3D reference). Result: every hint is `w` (the profile was authored in the (u, v) plane of its bone, like an elevation shape, thickness along the lateral axis). No hint is `v`: nowhere did `v` agree better than `w` with the twin, `dimensions` or the volume. Asset evidence (rest, 1:50, extent and IoU against the volume, `w` / `v`): cafe-chair plan forward 0.440 / 0.610 m (dimension 0.44), plan IoU 0.97 / 0.13; folding-deck-chair front lateral 0.585 / 0.046 m (0.58), IoU front 1.00 / 0.07; office-swivel-chair elevation IoU 0.95 / 0.36; picnic-table front height 0.748 / 1.700 m (0.75); single-bed front height 0.950 / 1.095 m (0.95); shopping-trolley front lateral 0.570 / 0.769 m (0.57), IoU front 0.92 / 0.60; playground-swing front IoU 0.94 / 0.76 (the "misplaced" seat and chains are this defect, not a separate one); waste-bin front height 1.033 / 1.230 m (1.03). About 40 % of the hints are weak (identical under both candidates for one view: a hint decided by the asset-level evidence and the documented convention of the schema, "the profile lies in the bone's (u, v) plane, as every extruded solid has been authored").

| asset | hints set to `"sweep": "w"` |
|---|---|
| furniture: bar-stool 21, cafe-chair 8, folding-deck-chair 80, office-swivel-chair 73, operating-table 28, picnic-table 16, single-bed 30 | 256 |
| objects: charcoal-grill 11, market-parasol 18, playground-swing 26, shopping-trolley 8, waste-bin 32 | 95 |
| fixtures: `🧊️solid-placements` 2, `🧊️axial-twist` 7, `🧊️depth-order` 4, `🔄️continuous-rotation` 14 (their hand-computed vectors already assumed the lateral sweep) | 27 |

Code (no implicit default any more):
- `S/🔨️modules/🧮️evaluation/🟦️.ts`: new `SolidSweepError` (asset, shape, viewpoint, bilingual `reason`), thrown by `placeShape` for an extruded solid whose `sweep` is not `v` or `w`; exported from `S/📦️packages/🟦️typescript/🟦️.ts`. `resolveViews` does not catch it (it only converts `RepresentationError` and `ViewMissingError`), so the multiview resolver refuses too.
- The semantic validator already had the stable rule id `solid-sweep` (`S/🔨️modules/🗂️catalog/🟦️.ts`, path `...solid.sweep`); `validateAsset` now also reports an asset whose evaluation throws as `evaluation-failed`.
- New vectors and test `S/🧫️fixtures/🧊️solid-sweep/🔣️.json`, `S/🧪️tests/🧊️solid-sweep/🟦️.ts`, `🥒️.feature`: hand-computed front and plan extents for `w` and `v` on the hanging thigh of the sample figure, recomputed with gl-matrix; missing and unknown sweep refused by `evaluate`, `resolveViews`, `validateAsset` (`solid-sweep`) and ajv. 14 tests, all pass.
- `S/🧫️fixtures/✅️asset-validation/🔣️.json`: case "extruded solid without sweep" expecting `solid-sweep`.
- README: extruded row of the solid table and the refusal documented.

Why the existing schema-conformance test missed it: `🧪️tests/🧬️schema-conformance` validated only the sample figure (which has no solids) and patched variants of it; no catalog asset and no asset fixture was ever run through ajv, and the schema's `required: ["kind","sweep","z0","z1"]` branch was never exercised by a vector. The semantic check (`solid-sweep`) was only reached by the React `catalog-loading` test, whose failures were treated as a known baseline. Fix: (1) the test now validates every catalog asset document and every asset fixture with ajv and fails with the first errors per asset (`validates every asset of the catalog and every asset fixture with ajv`), (2) three invalid vectors (no sweep, unknown axis, missing `z1`) and a positive test for both axes, (3) the `🧊️solid-sweep` test above. In this test file the pre-existing failure "rejects: unavailable without reason" was a stale vector (the sample's 1:500 is authored now); it replaces the whole 1:500 by an unavailable stub without `reason`. 43 tests pass.

## C. Other non-volume findings

Schema was wrong (schema and twin fixed, data untouched):
- `Clip.fps` `minimum: 1` → `exclusiveMinimum: 0` (`bicycle-stand` and `drinking-fountain` have 4 h and 2 h thermal clips at 0.01 fps, one frame per 100 s; the validator (`clip-duration`) already demanded only `fps > 0`).
- `ThermalRegion.operating.heatW` `minimum: 0` removed, signed (negative removes heat): the evaporators, cooling coils and chilled surfaces of `ceiling-diffuser` (-25 W), `expansion-valve` (-0.4, -0.8, -0.9, -2.4), `heat-pump-outdoor-unit` (-220), `scroll-compressor` (-0.8), `solar-thermal-collector` (-620), `split-ac-indoor-unit` (-20, -11) and the reversing operating levels of their poses are intended and solved as signed. Twin comment of `THERMAL_MODELS` updated.

Data fixed:
- `mvhr-unit` `extract-air.vapourKgPerKg` -0.0029 → 0.0047 and `heat-pump-outdoor-unit` `discharge-heating.vapourKgPerKg` -0.0008 → 0.0043: the transport source (`🔨️modules/💨️transport/🔥️sources`, `ratioOfSpecific`, and the fixtures `🌀️parcel-jet`) reads the member as the absolute specific humidity of the source air, which cannot be negative; the author meant "drier than the ambient air". Values are the saturation specific humidity at the discharge temperature (3.2 °C and 1.9 °C).
- `tower-server`: 7 shapes without `tone` (detail slits and ribs) got 0.3 like the neighbouring detail lines.
- `chalkboard`: stale rect members `x`, `y`, `w`, `h` removed from the ellipses `hanger-a` and `hanger-b`.
- `playground-swing` (`beam-cap-left|right`, front and plan, 4 shapes) and `excavator` (`hood-louvre-1..6`, plan, 6 shapes): rects of zero width or height are now open two-point `poly` lines of the same segment, unfilled.
- `noJoin` (a marker of an earlier authoring script, no code reads it) removed from 168 vegetation shapes: multi-stem-birch 6, date-palm 54, english-oak 8, scots-pine 4, miscanthus-grass 80, weeping-willow 10, lombardy-poplar 6.
- `scissor-lift`: pose `thermal.emitters` was nested one level too deep in 4 poses (`{"motor-vent": {"motor-vent": 0, "battery-plume": 0}, ...}`); flattened to the inner map.
- `etics-brick-wall-30`, `solid-brick-wall-1960` (`representation-copied`): the 1:500 is redrawn: etics build-up merged into one wall outline per view (0.38 m and 0.32 m layers are 0.76 and 0.64 mm on paper), solid brick wall as a closed polygon poché, wound the other way, tone 0.4. Both are nominal redraws; there is little to simplify at 1:500.
- Fixture `🔄️continuous-rotation`: `thermal.environment.diffuseFraction` 1 added (required member; no direct sun in the fixture), and a volume (stand box, annular rim revolve, spoke) because `volume` is required now.
- `concrete-mixer-truck` `drum-cone-cap`: revolve profile closed by repeating the first point (`[0, 1.12]`) as last; `volume-check` still passes (IoU 0.981 / 0.988 / 0.939, bbox within 0.2 %), `modelOf` + `validateModel` accept it.

Identifier renames (ids that violated the kebab-case `Slug`), consistently in rig, anchors, every representation, poses, clips (keys and tracks), thermal and structure bindings, loads, supports, symbols and in `volume` solids' `bone` (and `region`, `material`) references, by `catalog-rename-ids.py`; rule: lower case, `.` → `-`, the sides `-1`/`--1` of the office chair → `-left`/`-right` (left is +z); solid ids of the volume were renamed only for agave and hosta (`catalog-fix-solid-ids.py`) and no other `volume` member changed. The same replacement was applied to the ticket sources `T/volumes/<category>/<slug>.json`.

| asset | ids renamed | examples |
|---|---|---|
| agave | 49 (19 bones, loads `wind-*`, anchors) + 19 solid ids | `leaf-L1-a` → `leaf-l1-a`, `leaf-R2-b` → `leaf-r2-b`, `wind-L1` → `wind-l1` |
| hosta-clump | 45 (18 bones) + 18 solid ids | `blade-C1` → `blade-c1`, `petiole-R1` → `petiole-r1`, `water-L1` → `water-l1` |
| ivy-on-trellis | 12 (4 bones) | `post-L` → `post-l`, `crown-R` → `crown-r`, `base-support-L` → `base-support-l`, `tp-upper-crown-L` → `tp-upper-crown-l`, `shoe-L` → `shoe-l` |
| miscanthus-grass | 234 (12 bones) | `bl-L1a` → `bl-l1a`, `L1-1-a` → `l1-1-a`, `snow-bl-L1b` → `snow-bl-l1b` |
| bamboo-clump | 60 shape ids | `crown-A` → `crown-a`, `foliage-A-shade-0` → `foliage-a-shade-0` |
| date-palm | 3 | `ring-2.4` → `ring-2-4`, `ring-4.4` → `ring-4-4`, `ring-6.4` → `ring-6-4` |
| scissor-lift | 10 bones | `A1`..`A5` → `a1`..`a5`, `B1`..`B5` → `b1`..`b5` |
| three-seat-sofa | 4 | `back-seam-0.73` → `back-seam-0-73`, `-0.7333` → `-0-7333`, `-1.2667` → `-1-2667`, `-1.27` → `-1-27` |
| office-swivel-chair | 8 | `armrest-post-1` → `armrest-post-left`, `armrest-post--1` → `armrest-post-right` (same for `armrest`, `armrest-line`, `armrest-pad`) |

No fixture, test, Python or React consumer names any of these ids (searched `🎓️teaching`); the cast manifest names assets and poses only. The generator `T/volumes/machinery/scissor-lift.py` was updated (`a{i}`, `b{i}`, `a5`); the vegetation `*.author.ts` generators derive ids from the asset and were not run again.

## D. Agave polygon union (defect 3)

Root cause in the code, not the data. `unionPolygons` (`S/🔨️modules/🔺️polygon/🟦️.ts`) threw for agave pose `frost-sag` at 1:100 (19 polygons: a thin 17-point leaf outline and its 17 sweep quads): the fast edge-splitting union closed no boundary (it returned no ring, as before), the exact slab union built the right boundary (area 0.0087003125 m², identical to polygon-clipping's 0.0087003125), but `dropCollinear` then removed 86 vertices and shrank the area by 1.3e-4 (> the 1e-4 acceptance), because it judged each vertex against its original neighbours with an absolute cross-product threshold of 1e-12: a run of closely spaced vertices on a gently curved edge was collapsed into its chord. Fix: vertices are dropped only when they lie within 1 nm of the line from the last kept vertex to the next one (and do not turn back); `PolygonUnionError` now carries its `input`. Also fixed the strict-mode type errors of `chainPieces` (library tsc errors 4 → 1). Regression: a 19-polygon case in `S/🧫️fixtures/🔺️polygon-union/🔣️.json` (checked against the shapely oracle when `uv` exists) and a new test block `the union of the vectors against polygon-clipping` (a third-party JS library) in `S/🧪️tests/🔺️polygon-union/🟦️.ts`, 6 tests, 5 pass here (the shapely one needs `uv`; the file's top-level `requireTool("uv")` moved into that test so the others run, likewise in `🧊️solid-placements`, `🧊️depth-order`, `🧊️axial-twist`). All poses of agave evaluate at all four scales and 7 clip instants.

## E. Dimension mismatches (defect 4)

Finding: almost every reported mismatch compared the rig rest frame with `dimensions`. `dimensions` describe the presentation (default) pose; measured in that pose at 1:50 (`catalog-dims.ts`, width/height/depth drawn vs declared) all but one agree. Nothing was changed. "dimensions describe pose X":

| asset | pose | drawn at 1:50 vs `dimensions` |
|---|---|---|
| red-squirrel | `sitting-eating` | 0.197 × 0.279 × 0.124 vs 0.21 × 0.275 × 0.12 (0.416 m is the rest/stretched frame) |
| roe-deer | `alert` | 1.460 × 1.082 × 0.20 vs 1.46 × 1.08 × 0.2 (1.19 m is another pose) |
| domestic-cat | `standing-alert` | 0.768 × 0.359 vs 0.77 × 0.36 (the +7.6 % was the rest frame) |
| laptop | `open` | 0.274 × 0.218 × 0.313 vs 0.275 × 0.218 × 0.313 |
| vacuum-cleaner | `running` | 1.502 × 0.762 × 0.30 vs 1.5 × 0.76 × 0.3 |
| delivery-van | `parked` | 5.365 × 1.99 × 1.99 vs 5.34 × 1.99 × 1.99 (rest frames draw the rear doors open) |
| standing-adult | `relaxed` | 0.342 (1:50), 0.337 (1:100) vs 0.328 m wide: within the validator tolerance max(2 %, 15 mm) |
| date-palm | `calm` | 6.319 × 12.486 × 7.026 vs 6.3 × 12.5 × 7.0; the fronds in one plane are the elevation, front shows 7.0 m |
| bamboo-clump | `summer-calm` | 3.003 × 4.533 × 1.700 vs 3.03 × 4.57 × 1.7 |
| city-bicycle | `riding` | 1.875 × 1.100 × 0.620 vs 1.87 × 1.09 × 0.62 |

Left unchanged because the fix is artwork, not a number (open issues): `hosta-clump` is drawn 0.585 m high (1:50, 1:100, front) against `heightM` 0.62 m, which the summary, the symbol and the unavailable reasons repeat: the declared side is right, the drawing is 35 mm short (validator `extent-height`, `extent-front-height`; the authored volume follows the drawing, -5.2 %); `kneeling-gardener` 1:200 and 1:500 are declared unavailable (`unavailable-but-drawable`: 9.2 and 3.7 mm on paper, validator wants a drawing or a symbol: needs a redrawn simplified figure, a nominal copy is refused as `representation-copied`); `lateral-ambiguous` on `pregnant-person` (6), `sleeping-person` (2), `tour-guide` (11): flat front/plan shapes on spread bones without a `solid` or twin, up to 2.4 to 9.4 mm wrong on paper at 1:500; the per-asset 2D inconsistencies of the reports (`person-cooking` head revolves 0.15 m off the body, stroller bones inheriting z -0.1925, child legs 0.14 m in elevation vs 0.1 m (front/plan), `person-with-umbrella` side view, bamboo foliage positions, city-bicycle hub sizes, chalkboard frame depth, charcoal-grill handle, stepladder rear brace, cooking-pot section, retractable-bollard cuts) are single-asset artwork decisions listed here, not data errors with one right number.

## Code changes (paths under `S`)

- `🔨️modules/🧮️evaluation/🟦️.ts` (`SolidSweepError`, refusal in `placeShape`), `📦️packages/🟦️typescript/🟦️.ts` (export).
- `🔨️modules/🔺️polygon/🟦️.ts` (`dropCollinear`, `PolygonUnionError.input`, typing).
- `🧬️schema/🔣️.json` (`Clip.fps`, `operating.heatW`, `Asset.required` + `volume`), `🧬️schema/🟦️.ts` (`volume` non-optional, `THERMAL_MODELS` comment), `🔨️modules/🗂️catalog/🟦️.ts` (`volume` reported as `required-missing`), `README.md`.
- Tests and vectors: `🧪️tests/🧊️solid-sweep` (new), `🧫️fixtures/🧊️solid-sweep` (new), `🧪️tests/🧬️schema-conformance` (+ catalog-wide ajv, sweep vectors, stale vector fixed), `🧫️fixtures/🧬️schema-conformance`, `🧪️tests/🔺️polygon-union`, `🧫️fixtures/🔺️polygon-union`, `🧫️fixtures/✅️asset-validation`, `🧪️tests/🧊️model-exports` (casts of the deliberately volume-less asset), `🧪️tests/{🧊️solid-placements,🧊️depth-order,🧊️axial-twist}` (uv guard moved), fixtures `🧊️solid-placements`, `🧊️axial-twist`, `🧊️depth-order`, `🔄️continuous-rotation` (sweeps), `🦶️sparse-smooth-walk`, `🚶️reference-walker` (the sample figure's volume, same rig, materials, regions and dimensions), `🔄️continuous-rotation` (own volume).

## `volume` is required

`Asset.required` has `volume`, the twin `readonly volume: Volume`, `validateAsset` reports an absent one (`required-missing`, path `volume`). All 152 assets and all 4 asset fixtures have one and conform (ajv 0 errors); the three fixtures that lacked one got it as above.

## Tests: before and after

Direct runs (`bun node_modules/vitest/vitest.mjs run --config 🧪️tests/🎚️config/🟦️.ts`, `uv`, `ffmpeg`, `rsvg-convert`, `qpdf` and the DWG wasm are not installed on this host, as in the baseline; `repo run @teaching/staffage:test` queued for the 150 s I allowed and was cancelled, so it was not run to the end).

| | baseline (HEAD, `staffage-volume-schema.md`) | state before this fix (same report, final) | now |
|---|---|---|---|
| library vitest, passed / failed / skipped (files) | 1402 / 30 / 0 (84) | 1492 / 31 / 25 (85) | 2248 / 33 / 25 (2306 tests, 416 suites; other waves added tests) |
| `🧬️schema-conformance` | 38 tests, 1 failed ("unavailable without reason") | same | 43 tests, 0 failed |
| `🧊️solid-sweep` | | | 14 new, 0 failed |
| `🔺️polygon-union` | 3 (file did not load without `uv`) | | 6, 5 pass, 1 needs `uv` (shapely) |
| `✅️asset-validation` | 72 | | 73, 0 failed |
| library tsc errors | 4 | 4 | 1 (`🏷️cast-provenance`, pre-existing) |
| React `catalog-loading`: invalid of 152 | | 19 | 5 |

The 33 failures now: tests that need a missing tool (`uv` shapely and numpy oracles, `ffmpeg`, `rsvg-convert`, `qpdf`, `wasm` DWG: file load errors named in the report by `requireTool`) and 15 assertions that are independent of this fix and of the sweep: `🌡️thermal-span` (cast schema requires `scale`), `🎓️course-coverage` (`cooling-heating-comparison-cooler`), `🎞️clip-sampling` (transition fixture), `🔄️continuous-rotation` (cycle offset), `🧊️volume-schema` (axonometric refusal test, the depiction exists now), `🧭️representation-resolution` (2) and `🪞️representation-copied` (the sample's 1:500 is authored, the tests expect it unavailable), `🧍️plan-projection` (2), `🧊️model-catalog` (date-palm: the 3D volume is 7.013 m wide in x against `widthM` 6.3 m while the drawn elevation is 6.319 m), `🎭️cast-export` (4). `🧊️solid-placements`, `🧊️axial-twist`, `🧊️depth-order` fail only in their shapely test.

## Open issues and left as is

- `hosta-clump`: 0.585 m drawn against `heightM` 0.62 m (see E); 4 validator findings.
- `kneeling-gardener` 1:200 and 1:500 (2 findings); `pregnant-person` 6, `sleeping-person` 2, `tour-guide` 11 `lateral-ambiguous` findings: artwork, not touched.
- `vapourKgPerKg: 0` on about 35 vent, fan and flue emitters (`radiator-air`, `supply-air`, `jet-left`, ...): `emissionAt` reads 0 as the absolute specific humidity of the source air (bone-dry air at level 1), while the authors mean "no change against the ambient air". The negative values were fixed because they are impossible under either reading; the zero case needs a decision on the member's semantics (make the member optional-only, or define it as an excess and change the three source fixtures), not a data edit.
- The ticket generators `T/volumes/vegetation/*.author.ts` and `T/volumes/**/*.py` were not rerun after the renames except by text edits of their JSON outputs and `scissor-lift.py`.
- Volume-related Ajv or validator findings: none remain (0 of 0 after the solid-id renames of agave and hosta); the ids of other vegetation volume solids were slugs already.
- Ticket not closed: the `repo` MCP server was not connected in this session (`ticket_close` unavailable).
