# 📝️ sonnet-thermal: thermal, transport and their views in three dimensions

Library L = `🎓️teaching/🏛️architecture/🧍️staffage`, modules M = `L/🔨️modules`, tests T = `L/🧪️tests`.

## Findings (before)

| Area | Finding |
|---|---|
| Thermal field | Already view-independent: `solveThermal` reads only `state.thermal`, `timeS`, velocity and the asset. Region values are keyed by region id; views only map them to shapes. |
| Thermal view `auto` range | Spanned the values of the shapes present in the view, so elevation, front and plan could get different ranges for one state. |
| Transport | 2D only. `Parcel.at`, pathlines, streamlines, condensation and emitters were `{x, y}` in the elevation plane; lateral z was dropped; emitter direction was a sagittal angle; wind blew in the x-y plane (`windDirectionDeg` as an elevation angle); the breath cone was one angle. Plan and front could not show any drift. |
| Transport view | Used `ctx.paper(p)`, the elevation mapping, for every viewpoint. |
| Two environment evaluators | thermal blended the wind along the shorter arc, transport as a vector. |
| Transient thermal in transport | Sampled one environment per time, ignoring the timeline history. |
| Catalog data | Emitter `directionDeg` is relative to the bone frame. Three assets have a bone rotated by 90 degrees plus `directionDeg: 90`, so the outflow points backward horizontally: `diesel-generator` (`stack` bone 90, exhaust 90), `panel-radiator` (`radiator` 90 / 90) and `charcoal-grill` `lid-vent-flue` (bone 90 / 90). Probably an authoring error: 0 would be up. Not my files; owners should confirm. |

## Physical model and conventions

- World frame: x ahead, y up, z left of the asset (left-handed, ruling D6). Cameras come from `M/🎨️depiction/🔭️views` (`project`, `nearness`); the transport view imports them and does not invent a basis.
- The wind has no vertical component. `windDirectionDeg` is the horizontal bearing the air moves towards, from +x towards +z (0 ahead, 90 to the left, 180 backward). All datasets used only 0 and 180, so nothing changed for them. Schema JSON and TS, ambient and thermal docs say so.
- Transport state per parcel is `(x, y, z, ux, uy, uz, m_d, h, w_t)`: wind adds `(Ux, 0, Uz)`, jet momentum decays isotropically (the drag term acts on all three components), buoyancy acts on y, the random walk has an independent Gaussian per axis (hashed from seed, parcel key + lateral key, step), ground `y >= 0`. Parcels stay in world space after emission.
- Emitters: position is `anchors[id].at` with `anchors[id].z`; outflow direction is `cos(directionDeg) u + sin(directionDeg) v` of the 3D bone frame (so spread and twist carry it), plume emitters follow buoyancy (0, +/-1, 0). `EmitterState.direction` is a unit Vec3 and `directionRad` is gone (D4).
- Breath fan: two independent Gaussian angles (sigma = atan(2 alpha_j), clipped at 2 sigma) in two perpendicular planes through the nozzle axis, so each view sees the old half angle.
- Streamlines: all in 3D. Seeds per active emitter: 3 stations by 5 transverse points (axis, plus/minus along the vertical plane through the outflow, plus/minus horizontal), per inhalation sink 6 points (plus/minus x, y, z) and a vertical upwind rake along the wind bearing.
- Thermal view: `auto` spans the values of all regions of the solved field (identical in all views); painted values of merged shapes use the module's `areaWeighted` (area x exposed fraction, T7); the legend prints the clamped centre and says so when a fixed range moved it (T6); a region without a shape in the viewpoint is named in the legend interpretation ("not visible from this viewpoint").
- Transport view: `projector(ctx)` = camera projection x scale; parcels painted far to near by `nearness`; an emitter whose outflow lies along the viewing axis is drawn end-on as a ring; `ctx.paper` is not used any more.
- One environment evaluator: `M/🌡️thermal/🌤️environment`; wind blends as a vector (D12); the transport `ambient` duplicate is deleted and `environmentAt` of the transport index re-exports it. The transient thermal of transport now solves against the whole timeline with one session per track (T5).

## Changed files

- Contracts (Transport region only, D4): `M/🔗️contracts/🟦️.ts` (Parcel, Pathline, Streamline, EmitterState, condensation become Vec3; `direction` replaces `directionRad`).
- Schema (wind text only): `L/🧬️schema/🔣️.json`, `L/🧬️schema/🟦️.ts`.
- Transport: `M/💨️transport/🟦️.ts`, `🌬️ambient`, `🔥️sources`, `🧮️parcels`, `〰️streamlines`.
- Thermal: `M/🌡️thermal/🟦️.ts` (relative air speed uses the 3D COM velocity and the horizontal wind), `🌤️environment`.
- Views: `M/🎨️depiction/🌡️thermal-view`, `M/🎨️depiction/💨️transport-view`.
- Fixtures: `L/🧫️fixtures/🌡️thermal/🟦️.ts` (bone frame, anchor z, root, 3D COM velocity), `L/🧫️fixtures/💨️transport/🟦️.ts` (frame, anchor z, optional `rootZ` and `yawKeysDeg`).
- Tests: `T/💨️transport-field/🟦️.ts` (3D direction, six sink seeds), new `T/🌐️thermal-multiview/{🟦️.ts,🥒️.feature,🐍️.py}`.

## Evidence

- Typecheck: `tsc --noEmit -p 📦️packages/🟦️typescript/tsconfig.json` shows no errors in thermal, transport, ambient, streamlines, parcels, sources or the new test.
- `vitest run --root 📦️packages/🟦️typescript --config ../../🧪️tests/🎚️config/🟦️.ts transport-field transport-view parcel-jet`: 34 of 34 pass after the 3D change (run with vitest directly, because the `repo` runner was not used).
- Final run, `vitest run` over thermal-multiview, thermal-legend, transport-view, transport-assets, transport-field, parcel-jet, thermal-transient: 85 of 85 pass (numpy drift, projection and auto-range oracle included). Wider thermal and transport selection: 299 of 305 pass; the 6 failures are `thermal-span` and `reference-environment`, which fail because the cast manifest and the export CLI now require `viewpoint` (exports and course owners), not thermal or transport.
- Existing tests adapted to the 3D contract: `transport-field` (direction vector, six sink seeds), `transport-view` (stub context names a viewpoint), `transport-assets` (z of emitter positions, thermal against the whole timeline), `thermal-legend` (area x exposed fraction wording and weights).

## Audit T1 to T13

| # | State |
|---|---|
| T1 | Loop-boundary continuity test added (`one environment, one clock, one weighting`); passes with D11 (`clipTimeS` landed). |
| T2, T3 | One evaluator (`🌡️thermal/🌤️environment`), vector wind blend, headers and schema text updated; crosswind drift visible in plan. |
| T4 | Transport overlay projects through `🎨️depiction/🔭️views` (`project`, `nearness`). |
| T5 | Transient thermal of transport solves against the whole environment timeline. |
| T6, T7 | Legend states the clamped centre; merged value uses `areaWeighted` of the thermal module (area x exposed fraction); tests added or updated. |
| T8 | Not changed: totals are sums of powers in W, each on its physical basis (convection and radiation on exposed area, evaporation on skin area); judged a model choice, not a defect. |
| T9, T10 | Fixed (see below). |
| T11 | Unchanged by design: plume and transpiration strength use the documented 0.5 s grid `THERMAL_SAMPLE_S`. |
| T12 | Disclosed in the parcel legend (paper radius 0.15 to 12 mm). |
| T13 | Legend range of `span` is reported as `shared`, which is what the values use. |

## Open

- `M/🎨️depiction/🧩️shapes` still exports an `areaWeighted` that nothing uses any more (contracts' file).
- Catalog emitters `diesel-generator`, `panel-radiator`, `charcoal-grill` point backward horizontally (bone angle 90 plus `directionDeg` 90): owners should confirm the authoring.

## T9 and T10 (transient grid and start)

- `historyPlan` (`M/🌡️thermal/⏳️transient`) now plans on a grid that depends only on the asset and the session: `τ_max` comes from the reference instant t = 0 (not the query time), `W = 7 τ_max` (cap 3 h), step `0.5 s · 2^k` with `2 W / step <= 180`. History starts at an epoch boundary: epochs are `ceil(W / step) · step` long and a query in epoch `n` starts at `(n - 1) E` (0 in the first epoch), so history is between `W` and `2 W` long, and all queries of one epoch share one start and one chain.
- The chain start is the steady state of the inputs at the start time for every region, placed (body) regions included (`steadyBody(...).diag.regions[slot].surfaceC` at the start instant); it no longer reads the query time's steady temperatures (T10).
- Tests (`T/🌐️thermal-multiview`, `transient thermal grid and start`): temperatures at 0.4, 7.3, 31.9, 250.6, 900.1 and 2400.7 s of a jogger run cycle under a ramping environment are bit-identical cold, played in order with one session and scrubbed in reverse with one session; the plan grid and start are equal for all queries of one epoch.

## Sweep (all assets, extended)

`T/🛠️scripts/thermal/sweep.ts` (bun), shard-safe: `bun sweep.ts --out <existing dir> [--shard i/n] [--category c] [--asset slug] [--environments asset,office-neutral,summer-hot-sun | all]`. It never creates directories (the caller does, because bun crashes on recursive mkdir with emoji paths); each shard truncates and appends its own `sweep-<i>of<n>.jsonl` (one line per asset, then a `total` line), and prints `[FINDING]`, `[AIRBORNE]` and `[DEBUG]` lines. Assets are dealt round robin by their index in the sorted catalog.

Coverage of the last run (6 shards, 1061 s wall): 149 assets, 2808 instants, 632835 renders.
- Every authored scale of every asset; all seven views in elevation, front and plan; environments: the asset's own, `office-neutral` (indoor reference) and `summer-hot-sun` (outdoor reference).
- Instants: default pose, every clip at 0, 25, 50, 75 and 99.9 % of its duration, and transitions (first pose to second and last to first, durationS 1, at 0, 0.5 and 1).
- Hard checks: every view renders; a region drawn in some view has the solved field value in every view where it is drawn and so the same value in all of them (no mismatch found); the three thermal legends are equal; a region not drawn in a viewpoint is named in its legend; emitter markers lie inside the paper; parcel items equal the field's parcels; D43: a region with no shape anywhere is accepted only if it is air-borne (its `hidden` note says air-borne and luftgetragen and an emitter with `region` traces it) and is then reported as `air-borne (traced by <emitter>)`.
- Air-borne (accepted): `🔌️appliances/shower-head` region `spray` (traced by `steam-plume`).
- Authored scales that the print profile calls unsuitable are one finding per asset and scale (the depiction refuses them by name); the sweep skips them.

Findings by asset (22 of 149; the transient `wheelchair-user`, `shower-head` spray and `charcoal-grill` region findings of the first sweep no longer occur):

| Asset | Finding |
|---|---|
| `🌳️vegetation/hosta-clump` | region `petioles` has no shape in any view |
| `🌳️vegetation/ivy-on-trellis` | region `stems` has no shape in any view |
| `🚜️vehicles/city-car` | region `interior` has no shape in any view |
| `🚜️vehicles/vintage-car-1929` | region `interior` has no shape in any view |
| `🚜️vehicles/electric-kick-scooter` | region `bms-controller` has no shape in any view |
| `🪑️objects/market-parasol` | region `canopy-underside` has no shape in any view (at some scales) |
| `🏗️machinery/concrete-mixer-truck`, `🏗️machinery/forklift`, `🪑️objects/charcoal-grill` | authored 1:500 is unsuitable under the print profile |
| `🐕️animals/red-fox` | authored 1:200 unsuitable |
| `👤️humans/child-on-kick-scooter`, `cyclist`, `person-with-rollator`, `person-with-white-cane` | authored 1:200 unsuitable |
| `👤️humans/courier-with-hand-truck`, `elderly-with-cane`, `parent-with-stroller`, `runner-with-blade-prosthesis`, `running-child`, `seated-reader`, `worker-carrying-box` | authored 1:500 unsuitable |
| `🔌️appliances/shower-head` | authored 1:200 unsuitable |

The other 127 assets pass every check in every combination. The sweep output (`🗑️generated/thermal`) is left in place because deleting it was blocked in this session; delete it when the findings are routed.

## Hatching legend

The thermal legend states in en and de what the absence of a hatch means (`unhatched = within ±35 %` / `unschraffiert = innerhalb von ±35 %`, hot/cold hatch beyond it, or "Hatching is off" when patterns are off); the transport temperature legend states the same. Asserted in all three viewpoints in `T/🌐️thermal-multiview` (near-ambient pose).

## Transport view additions (modes audit)

- Emitter labels use `formatNumber(..., language)` (German decimal comma), also the time in the parcel legend.
- Temperature parcels carry a second, colour-independent cue: hatch `hot` or `cold` beyond 35 % of the half range (vapour keeps `vapour`); the legend says so.
- `T/💨️transport-view` now covers elevation, front and plan (items, markers, legends, relative paper positions) plus the German comma and the hatch cue.
