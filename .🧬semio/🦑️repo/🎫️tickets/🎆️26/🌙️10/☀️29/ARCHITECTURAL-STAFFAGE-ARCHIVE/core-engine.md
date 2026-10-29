# Core engine (worker report)

Root S = `🎓️teaching/🏛️architecture/🧍️staffage`.

## Modules
- `📐️units`: `SCALE_PRESETS`, `mmPerMetre`, `metresToMm`, `mmToMetres`, `lodFor`, `presetIndex`, `lineWeightMm`, `textSizeMm`, `formatQuantity`/`quantityParts` (N, Pa, N·m, °C, W/m², W; prefixes chosen on the rounded magnitude, 3 significant digits, `,` in German, NBSP before the unit).
- `🦴️rig`: `IDENTITY`, `placement`, `compose`, `apply`, `invert` (throws if singular), `effectiveOffset`, `parentFirstOrder` (throws on duplicate id, missing root/parent, cycle, second root), `solveRig`.
- `🎞️animation`: `ease`, `wrapTime`, `cycleOf`, `frameAt`, `poseOf`, `clipOf`, `locate`, `sampleTrack`, `contactsAt`, `phaseAt`, `blendThermal`, `blendLoads`, `sampleClip`, `samplePose`, `transition`, `PLANT_HZ`.
- `🧮️evaluation`: `evaluate`, `flattenPath`, `flattenGeometry`, re-exports `poseOf`/`clipOf`.
- `🗂️catalog`: `validateAsset`, `validateManifest`; `📥️files`: async `loadCatalogFromFiles(root?, signal?)` (skips category files that do not exist yet).
- `🧬️schema/🔣️.json`: generated once from the TS enum constants by `🧰️scripts/generate-schema.ts` (ticket folder); 52 `$defs`, draft-07, `$id` semio.staffage/v1, root `oneOf [CategoryDocument, Manifest]`.

## Decisions
- `Sample.timeS` / `EvaluatedState.timeS` is the wrapped (loop) or clamped time; a looping clip adds `cycle × advanceM` to the root of `plant` clips only.
- Root motion `plant`: 240 Hz table (plus span boundaries as instants), earliest planted active span drives; a later span that outlasts it takes over through a smoothstep crossfade over the overlap (root x and velocity continuous). Spans starting together: declaration order. Free spans continue with the last velocity. Spans that wrap the loop seam are written as two spans (`[a, duration]`, `[0, b]`); `advanceM` must equal the planted travel per cycle (validator code `clip-advance`, 2 mm).
- Ground `auto` when both key poses of the segment are `auto` and at least one contact is active; otherwise authored y.
- Thermal defaults: unbound emitters 0, unbound operating levels 0, `exposedFraction` 1, activity `baseMet ?? 1`; `ThermalInputs` lists every declared region, emitter and operating key.
- Loads present in only one of the two key poses fade with the eased weight; `load:<id>` tracks multiply on top.
- Bounds ignore `ground`-role shapes. Plan shapes are in plan coordinates and follow their bone only by its x travel from rest.
- Kinematics: central differences with h = 1/240 on the unwrapped time (seamless at the loop seam); non-looping clips use one-sided velocity and zero acceleration at the ends.
- Interpolation is C0 across keys (linear) so velocity kinks at keys produce acceleration spikes in the central differences; author dense keys or smooth foot paths (the walker has 96 keys from two-link IK of smooth foot paths).

## Reference walker fixture
`🧫️fixtures/🚶️reference-walker/🔣️.json` is produced by `🧰️scripts/generate-reference-walker.ts`: 6 poses, 3 clips (walk with 96 IK keys, idle-breathing, bag-swing), overlapping contact spans (left `[0.95,1]+[0,0.55]`, right `[0.45,1]+[0,0.05]`), ball-of-foot contact anchors, Winter (2009) masses. `repo`-less full-pipeline check `🩺️check` is clean (✓) at 9, 25 samples.

## Tests (all green, `vitest run` with `🧪️tests/🎚️config/🟦️.ts`, 8 files, 137 tests)
📏️print-size (d3-scale, d3-format), 🦴️forward-kinematics (gl-matrix), 🎞️clip-sampling (three, d3-ease), 🦶️foot-plant, 〰️path-flattening (three curves), ✅️asset-validation (54 variants), 🧬️schema-conformance (ajv, enum twin), 🧮️evaluation-state. Each has fixtures under `🧫️fixtures/<case>/🔣️.json` and a `🥒️.feature`.

## C1 smooth interpolation (coordinator decision)
- `ease: "smooth"` on a key = cubic Hermite (Catmull-Rom with uneven spacing) from that key to the next: tangent at a key = (v[next] − v[prev]) / (t[next] − t[prev]); looping clips wrap neighbours (times shifted by one duration), non-looping clips use zero end tangents. Applies to bone offsets (angleDeg, dx, dy), authored root x/y/angle and scalar tracks. `linear`/`in`/`out`/`step` unchanged. Thermal bindings and loads still blend by the smoothstep weight.
- A looping clip (or track) whose last key is at or beyond the duration and whose first key is at t = 0 drops that last key (it repeats the first).
- Schema `Ease` description updated (generator `🧰️scripts/generate-schema.ts`). Keys with equal neighbours reduce to smoothstep, so two-key and symmetric clips keep their old values.
- Test `🧪️tests/🌊️smooth-continuity` (fixtures `🌊️smooth-continuity`, `🦶️sparse-smooth-walk`): velocity equal on both sides of keys, linear jumps, loop seam, three.js CatmullRomCurve3 oracle, root continuity, track slope, transition at rest, check CLI clean.
- Sparse walk: 4 smooth keys sampled from the dense gait (`WALK_OFF=0.02`), cycle stretched to 4 s (0.23 m/s). Faster cycles with only 4 keys leave residuals of 100–500 N in the check (friction cap; the Catmull-Rom path through 4 samples is not the planted-foot path).
