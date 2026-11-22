# Package `physics`: the air flows around the body and reads right in depth

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `R` = `S/🎯️targets/⚛️react/🔨️modules`.

## What changed

| File | Change |
|---|---|
| `S/🔨️modules/💨️transport/🌊️flow/🪨️obstacles/🟦️.ts` (new) | The body as obstacles. `bodyClusters` (once per authored volume: kernel tessellation of the `body` solids in their bone frame, support points in 74 directions, neighbour merging by least wasted box volume down to at most 16 groups, groups under 1/500 of the largest left out), `obstaclesOf(state)` (groups carried by the evaluated bone frames, oriented box, superellipsoid of exponent 2, 4 or 8 that holds every support point, kind `solid` / `lattice` / `foliage` with blockage), `blendObstacles`, `packWalls`, `openingsOf`, `deflect`, `insideWall`, `porousShare`, `pushOut`, `flowVisibility`, `obstacleLevel`. |
| `S/🔨️modules/💨️transport/🌊️flow/🟦️.ts` | `FlowOptions.obstacles`; `flowSampler(…, walls)` splits the undisturbed velocity into external and jet part and deflects it, zero field inside a wall; `traceStream` puts a point back on the surface and ends at a stall; `flowSeeds` drops seeds inside a wall (nozzle seeds are moved out along their axis) and spreads the wind rake across the body (`WIND_RAKE_ACROSS`); `FlowField.obstacles`; `FlowDetail.eye` and `visible` / `puffVisible` in `flowMarks` (cached per field and eye). |
| `S/🔨️modules/💨️transport/🧮️parcels/🟦️.ts` | Parcels slide along walls once they have left their nozzle (`confine`, `wallsAt`: obstacles interpolated between evaluations every 8 steps, push limited to `WALL_PUSH_MS` = 1.2 m/s), wind slowed inside porous obstacles. Runs only for an asset with an authored volume. `Resolved.bodyScale?`. |
| `S/🔨️modules/💨️transport/🟦️.ts` | Passes `obstaclesOf(state)` to `flowAt`; `TransportOptions.bodyScale`; exports of the obstacle API. |
| `S/🔨️modules/🔗️contracts/🟦️.ts` | `FlowObstacle`, `FlowEye`, `FLOW_HIDDEN`, `FlowField.obstacles`, `FlowRibbon.visible`, `FlowStreak.visible`, `FlowMarks.puffVisible`. |
| `S/🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts` | Marks asked for with the eye of the viewpoint; streams cut per segment where hidden and drawn fainter in `FLOW_VEIL_BANDS` steps behind foliage; arrows, vapour lines, puffs, fog and tracers follow. |
| `R/🌊️flow/🖌️overlay/🟦️.ts` | `FlowProjection.eye`; every segment of streams, vapour lines and tracers, arrows, puffs and fog hidden or veiled by its visibility. |
| `R/🧊️spatial/🖥️stage/🟦️.ts` (`#projection` only) | The eye of the camera from the view matrix (direction for an orthographic camera, place for a perspective one). |
| `R/🌊️flow/🖥️stage/🟦️.ts` (`paperProjection`, smallest edit, not my file) | The eye of the viewpoint, so the 2D screen figure and the custom element hide what paper hides. |
| `S/README.md` | "Air flow": new bullets **Body** and **Depth**, the limits paragraph rewritten, the test list. |
| Tests | `S/🧪️tests/🌊️flow-obstacles/{🟦️.ts, 🥒️.feature, 🐍️.py, 🧱️mesh.ts}`, `S/🧫️fixtures/🌊️flow-obstacles/🔣️.json` (196 K). |
| Ticket scripts `physics/` | `parcel-digest.ts`, `obstacles-probe.ts`, `occlusion-probe.ts`, `unobstructed-digest.ts`, `regenerate-oracle.sh`. |

Pathlines: package `legacy-layers` already carries `FlowTrail.visible` through `flowMarks`, the paper view and the overlay (checked on disk: `flowMarks` fills it from the same `flowVisibility`, both renderers cut per segment at `FLOW_HIDDEN`), so the occlusion covers them.

## Method (documented in the module text)

- Deflection: per obstacle a doublet on the external flow, `Δu = φ/(2s³)·(U − 3(U·n)n)` (exact potential flow for a sphere), for jets only the removal of what runs into the wall, mirror images below the ground, a shell that removes the residual the other obstacles leave on a wall (nearest wall last), a limit of 2 U on the summed speed-up, open walls around active nozzles and inhaling openings.
- Foliage rule: a body solid whose material is the material of a thermal region of kind `foliage` is porous, `φ = 1 − exp(−0.2 · 1 m²/m³ · L)`, `L` the mean chord of the obstacle; sparse body solids (cover of the silhouette below one half, by Cauchy's formula) are a `lattice` with `φ` = cover. English oak: trunk one wall, 8 limb lattices (φ 0.06–0.32), 7 foliage obstacles (φ 0.48–0.61).
- Decision that departs from the prompt's wording: the obstacles come from the finest authored volume, not from the drawn scale, so parcels and field are one physics whatever scale is drawn and the solver sessions are not split per scale; `TransportOptions.bodyScale` names another volume.

## Verified

- New suite `🌊️flow-obstacles`: 15 tests green. Sphere velocity against the closed form at 12 points (1e-9 m/s), no normal velocity and the analytic slip on the surface, stagnation points, 1.5 U at the equator, 6 streams of 300 points against scipy `solve_ivp` (4 mm, travel time 1 %), impermeable walls and ground with three meeting obstacles, porous share, occlusion against trimesh ray casts for ellipsoid, rounded cylinder and rounded box from 5 eyes × 441 probes (exact outside the silhouette margin), sample figure in wind: mesh digest of the kernel's posed triangles (4020 triangles, 17 watertight solids), every vertex within `s < 1.05` of an obstacle, no stream point and no parcel older than 1 s inside a wall or in one of 686 voxels at least 5 cm inside the triangles (trimesh), occlusion agreement with ray casts on the triangles above 0.9, parcels without the volume pinned by digest. `🐍️.py` without `--write`: difference 0.
- Bit-identity without a volume: `physics/parcel-digest.ts` (parcels, streams and puffs of 6 solves per asset, `volumes` removed) before the first edit and after the last code edit: identical for sample-figure, electric-fan-heater, electric-kettle and english-oak (`8b70d42b…`, `95a80eb2…`, `9c62ff7e…`, `2ce30f21…`). standing-adult differs only because another session rewrote its catalog file at 15:13.
- Last run of the suites I touched, after the last code edit: `flow-obstacles`, `flow-field`, `flow-environments`, `flow-view`, `parcel-jet`, `condensation`, `transport-assets`: 7 files, 84 tests green. `transport-field`: green in the run before (74 tests in 4 files, 69 green; the 5 red ones below). React target: `🌊️flow` and `figure-render`: 14 green.
- `tsc --noEmit`: library package 0 errors, React package only the known errors in `🧰️framework/🔨️modules/🖱️ui`.
- Cost (standing adult, host load 30): field refresh 55 ms before, 60 ms now (10 frames 596 ms; limit 80 ms); develop 3.4 s → 4.3 s of CPU; grouping once per volume 0.2 s (adult), 0.5 s (oak); posing 1–3 ms per state.
- Looked at (SVG rasterised with `qlmanage`): standing adult in `spring-mild-sun` in elevation, plan, front and axonometric (wind streams part around the body in plan, the streams behind the body are cut, those in front run over it), english oak 1:200 (streams fainter through the crown, the low stream bends around the trunk), electric fan heater (jet leaves the grille, entrained air comes over and around the casing, none through it).

## Not done, open

- The final rerun after the last README edit did not run: the dependency store behind `node_modules` (`.cache/b/d/m/j/…`) disappeared from the host at about 15:35 (not by me), so vitest and tsc are missing. No code changed after the green runs above.
- Not seen in the browser: the 3D stage with the new eye. The site on port 6072 stopped answering before I could take the picture; the stage path is covered by the type check only (jsdom has no canvas).
- Red in sibling suites at 15:16, for their owners: `flow-exports` (3: a mark 0.02 mm outside the projected domain, "changes nothing but the flow marks between two frames", the tracer sets per sheet cell, which occlusion legitimately makes differ per view — the owner was already rewriting that test) and `flow-model` (1: "structure triangles grow from 1:200 to 1:500" for the sample figure; the streams around the body changed, the budget rule needs a look by `model-flow`). `transport-assets` loop continuity was red once (parcels pushed 3.9 cm in one step by the moving body) and is green since the walls are interpolated and the push is limited.
- Finding, not from this package: the fan heater's streams reach 143 K excess while its parcels carry 41 K (two coincident jets each add their Gaussian centreline); same with and without obstacles.
- Limits kept (in the README): fitted superellipsoids, not triangles (an obstacle is up to a few centimetres larger than its solids, gaps such as arm to trunk are closed); superposition of doublets, no wake; a skinned solid moves with its own bone.
