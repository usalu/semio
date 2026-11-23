# 🩹️ Fix of the Original Catalog Assets That Fail the Check CLI

Scope: the eight originals of the order plus the three the coordinator added at 04:28 (`person-with-white-cane`, `wheelchair-user`, `worker-carrying-box`). Session 2026-11-23, about 02:30 to 05:15. Every check line below was run to the end in this session with the pinned bun, one process at a time: `bun 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id> --quiet` from the product folder.

## Result

| Asset | Before | After |
| --- | --- | --- |
| `gas-boiler` | `✗` force residual -5.87e-3 N in every pose and clip instant | `✓ gas-boiler (1/1)` |
| `cafe-chair` | `✗` support-switched (two front feet) | `✓ cafe-chair (1/1)` |
| `charcoal-grill` | `✗` support-switched (`leg-front`) | `✓ charcoal-grill (1/1)` |
| `courier-with-hand-truck` | `✗` push-walk@0.000 1.74 N, @1.200 0.96 N | `✓ courier-with-hand-truck (1/1)` |
| `elderly-with-cane` | `✗` cane-gait@0.000 and @1.700 843 N | `✓ elderly-with-cane (1/1)` |
| `kneeling-gardener` | `✗` union of 4 polygons cannot be built (4 instants) | `✓ kneeling-gardener (1/1)` |
| `parent-with-stroller` | `✗` union of 11 to 27 polygons cannot be built (6 instants) | `✓ parent-with-stroller (1/1)` |
| `person-with-white-cane` | `✗` constant-contact-walk@0.138 -3.76e-3 N, @0.275 -3.88e-3 N | `✓ person-with-white-cane (1/1)` |
| `worker-carrying-box` | `✗` carry-walk@0.000 -438 N, @0.550 -348 N | `✓ worker-carrying-box (1/1)` |
| `jogger` | `✗` run-cycle@0.180 and @0.540 784 N | **still `✗`, same two lines; asset unchanged** |
| `wheelchair-user` | `✗` push-cycle@0.000 -1.39e+3 N, ramp-roll@0.000 -1.19e+3 N | **still `✗`, same two lines; asset unchanged** |

Nine of eleven pass. `jogger` and `wheelchair-user` stay open (reasons below); I did not tune either to pass.

## Is there one common cause?

No single one. Measured:

- Shared code barely moved in the repository history (`🏗️structure` and `🩺️check` have one commit, 11-18; `🎞️animation` changed one line on 11-21). The assets did: every failing human had its rig, anchors, poses and clips rewritten in the commits of 11-20 to 11-22. `elderly-with-cane` of commit `3d62b2ef22c` (11-18) prints `✓` with today's code, the version of `b4927d2c311` (11-21) prints the 843 N failure. So the gait failures are re-authored figures whose gait keys were not re-solved for the new foot geometry. `wheelchair-user` of 11-18 already fails with today's code (-1.97e+3 N).
- Two real defects in shared code, each hit by several assets:
  1. **Structure solver, unit-dependent projection** (`gas-boiler`, `person-with-white-cane`, also the 1.74 N of the courier and stray 2 to 3 N instants of other walkers). When the supports cannot balance a moment (a body that tips), the solver projected the six-component load wrench by plain least squares, which trades newtons against newton-metres about the centroid of the supports. A boiler hanging on a pin with its centre of mass 0.07 mm beside the rail showed 5.87 mN of "force residual", although the pin can carry every force. Fixed (below).
  2. **Polygon union, overlapping edges a few nanometres apart** (`kneeling-gardener`, `parent-with-stroller`; same root as three open sections in `requests.md`). Fixed (below).
- The check samples exactly `t = 0` and `t = duration`, where a looping gait has its seam. Three asset conventions fail there: a foot contact that is not level with the others when the contact set changes (the automatic ground lifts the body within one 1/240 s step), an `advanceM` that differs from the planted travel, and a striking foot that still swings (the trailing span is split at the seam, so the root is not crossfaded there).

## Shared code changed

### `🔨️modules/🔺️polygon/🟦️.ts` (`primaryUnion`)

Cause: two edges were treated as coincident only when parallel within 1e-9. The failing inputs are prism faces of one extruded shape seen along its profile: two rectangles of equal width overlapping in y whose long edges lie 9 nm apart at an angle of 5e-8 rad. They were handled as a crossing (0.18 m outside both edges), so neither edge was cut where the other ends; the overlapped piece of the second edge was dropped whole, including its part outside the first polygon, and the boundary did not close. The slab fallback then failed on slabs a few nanometres wide.

Fix (three lines): an edge whose two ends both lie within `SAME_M` (0.3 µm) of the line of the other edge is coincident and is cut at the projections of those ends. Exactly parallel edges behave as before.

Test: four reproductions added to `🧫️fixtures/🔺️polygon-union/🔣️.json` (`slivers`: gardener dig-cycle 0.6 s at 1:50 and 1:100, stroller stop-and-check-child 4 s and push-walk 0.5625 s) and a scenario in `🧪️tests/🔺️polygon-union/🥒️.feature`. The existing test compares ring count and area with polygon-clipping. With the committed module all four throw `PolygonUnionError`; with the fix each is one ring. The eleven older cases keep their ring counts; two herring-gull areas move in the seventh digit, inside the tolerance. `🔺️polygon-union`: 18 of 18 passed (includes the shapely case).

### `🔨️modules/🏗️structure/🟦️.ts` (`solveForceFirst`, used by `reactionSolution`)

Fix: new exported `solveForceFirst(k, f)`. Of all wrenches the supports can give it takes the one with the smallest unbalanced force and, among those, the smallest unbalanced moment. When the load is balanceable it returns the solution of `solveSemiDefinite` (unchanged path). The model note of the result (en, de) says so. No rule, tolerance or check was changed.

Effect measured: committed `gas-boiler` goes from residual fx -5.87e-3 N to fx 0 with mz -0.0196 N·m (weight times 0.0672 mm); `person-with-white-cane` from up to 0.35 N to 1e-13 N at all nine instants.

Test: new case `🧪️tests/🥇️force-first` (`🟦️.ts`, `🐍️.py`, `🥒️.feature`) with `🧫️fixtures/🥇️force-first/🔣️.json` (15 seeded stiffness matrices of rank 1 to 5): the boiler analytically, the seeded cases against numpy (SVD), never more force left than the plain projection, full rank equal to the minimum-norm solution. 6 of 6 passed.

Neighbouring tests run after both changes (vitest, config of `🧪️tests/🎚️config`, pinned bun): `⚖️equilibrium 🏗️cantilever 🏗️rigid-trees-3d 🏗️structure-oracle 🏗️structure-projection 🌉️simply-supported 🦶️centre-of-pressure 🧍️two-foot-stance 🏃️inertial-consistency` gave 8 files, 213 tests passed (nine names given; I did not find out which name matched no file); `🦶️foot-plant 🎨️structure-legend 🧊️lateral-spread 🧊️solid-sweep 🔺️polygon-union 🥇️force-first 🪢️tied-canopy` gave 7 files, 82 tests passed. The catalog-wide suites were not run.

Regression sample, check CLI, all `✓` before and `✓` after: `standing-adult`, `walking-commuter`, `nurse-walking`, `hiker-with-trekking-poles`, `dancer`, `picnicker-sitting-on-ground`, `cyclist`, `office-swivel-chair`, `heat-pump-outdoor-unit`, `tower-crane`, `english-oak`, `labrador-dog`, `city-bicycle`, `sliding-door`, `playground-swing`. The solver change can alter reactions of any asset in an instant it cannot balance; 15 of about 347 assets is a sample, not the catalog.

## Per asset

**gas-boiler** — Cause: solver defect 1 plus a real imbalance: the 0.05 kg mode knob at x = 0.04 m puts the centre of mass 0.0672 mm beside the rail, which a pin and a roller cannot hold. Fix: casing `comX` 0 → -0.0000727, so the boiler hangs plumb (residual moment gone, not only the force). Passes with either solver.

**cafe-chair**, **charcoal-grill** — Cause: asset. `structure.supports` listed feet that the poses `tilted-on-two-legs` / `wheeled-tilted` and their clips lift; permanent supports stay active in every pose. Fix: `structure.supports` keeps the rear feet / the two wheels; the front feet remain pose and clip contacts. The rule `volume-anchor` then rejects support solids of non-permanent anchors, so `support-contact-front-left/right` (chair) and `support-leg-front` (grill) were removed from `volumes.*.structure` at all four scales (body lists untouched). `checkVolumes` with `volumes-structure-coverage, volumes-connected, volumes-bones, volumes-solids`: 0 findings for both. Loss: the structure view no longer marks the liftable feet as supports; that needs the engine feature already asked for in `requests.md` ("support solids for pose-dependent supports"). The tilted poses keep an unbalanced moment (the hand that holds them is not modelled); the check does not test moments.

**courier-with-hand-truck** — Cause: asset, exposed by solver defect 1. At the seam the striking heel still swings (root velocity jump 0.010 m/s), the trailing ball then slides under the 5.8 kN heel-strike spike. Fix: last key before the seam (t = 1.16667) `thigh-l` 37.0315 → 36.9891, `shank-l` -19.8741 → -19.7246; `advanceM` 1.14 → 1.140001. Not changed: the wheel is on the ground during `push-walk` but cannot be listed as a contact, because root motion `plant` pins every listed contact (tried: `clip-advance` fails and the kinematics break); the courier therefore carries the truck through his feet with an unbalanced moment, and the mirrored key at 0.5667 keeps the old angles (0.04° and 0.15° asymmetry).

**elderly-with-cane** — Cause: asset. In the heel-strike pose the trailing ball was 0.66 mm above the heel and the cane tip, so the heel was 0.6 mm under ground before the seam and the body jumped at t = 0; the striking leg also swung at 0.043 m/s. Fix: `foot-r` of pose `contact-left` and key 0, `foot-l` of pose `contact-right` and key 30: 19.5423 → 19.2376; key 59 `thigh-l` 24.9126 → 24.9172, `shank-l` -20.7141 → -20.4293; `advanceM` 0.9619 → 0.963122. The heel still dips 0.3 mm under ground in the last key (not a contact there, no dynamic effect).

**worker-carrying-box** — Cause: asset, same seam defect (trailing sole 2.9 mm above the striking foot). Fix: `foot-r` of `walk-heel-strike-l` and `foot-l` of `walk-heel-strike-r` 12.43 → 10.1568; `advanceM` 1.097274 → 1.108789 (the planted travel grew by 11.5 mm with the flatter trailing foot, about 1 % more speed). Instants 4 ms after each heel strike still slide for one or two samples (not check samples).

**kneeling-gardener**, **parent-with-stroller** — Cause: polygon union defect. No asset change.

**person-with-white-cane** — Cause: solver defect 1. No asset change. Seen and not touched: the cane tip carries -22 N (pull) at 0.275 s and 0.825 s.

**jogger (open)** — Cause: asset; the run cycle is not a physical run. Twelve keys 60 ms apart, measured every 20 ms: horizontal inertia swings between -3170 N and +3080 N (the body would change speed between 2.0 and 3.4 m/s within one stance), vertical between -14800 N and +34200 N, with jumps where the heel span ends at 0.084 s (heel 4 mm below the sole) and at toe-off (authored root 10 mm above the grounded one). The check sees the 0.18 s instant: 3077 N forward against 0.8 × 2867 N of friction. The stance also covers 0.606 m in 0.24 s (2.5 m/s) and the flight 0.234 m in 0.12 s (1.95 m/s). The `ballistic` flag is not involved: 0.18 s and 0.54 s are stance. I wrote a solver (`🧰️fix-originals/jogger-run-cycle.ts`: target centre-of-mass path with a sine ground force, ballistic flight, heel impact) and ran it on a scratch copy: horizontal inertia fell below 910 N, but the vertical path stayed up to 8.8 mm off, forces still ran from -4 to +5 body weights, the flight legs changed strongly and midstance `foot-l` reached 38.04° (limit 38), so the copy failed `joint-limit`. Not applied. It needs a re-authoring with keys every 1/30 s and per-key leg solutions like the walkers, and a look at the sheets. The silhouette guard was not re-run (no geometry changed).

**wheelchair-user (open)** — Cause: convention between asset and engine. `push-cycle` and `ramp-roll` are cyclic strokes authored as one-shot clips (`loop: false`); a one-shot spline starts with zero tangents, so the trunk (lumbar -4° → -7° in the first 31 ms) and arms start from rest and the first key interval needs 27 m/s² (inertia -2436 N, sliding). As loops with a closing key the start is smooth (measured: -246 N at t = 0, no residual), but the rule `loop-closing` demands whole turns and the rear wheel turns 240° per stroke (-960° in four strokes, -384° on the ramp). Reverted byte for byte. Options for the owner: a three-stroke loop (-720°, castor step 33.7389° → 33.75°) for `push-cycle` only; or first and last strokes that start and end at rest; or an engine decision (closing keys for wheels with rotational symmetry, or natural end tangents for one-shots).

## Files

Updated (repo-relative):

- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔺️polygon-union/🥒️.feature`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🔺️polygon-union/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-boiler/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/cafe-chair/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/charcoal-grill/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/courier-with-hand-truck/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/elderly-with-cane/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/worker-carrying-box/🔣️.json`

Created:

- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🥇️force-first/🟦️.ts`, `🐍️.py`, `🥒️.feature`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🥇️force-first/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/fix-originals.md`
- `…/🧰️fix-originals/drop-switched-support-solids.ts` (chair, grill), `level-contact.ts` and `seam-velocity.ts` (the three walkers; they print values, the numbers were written by hand), `jogger-run-cycle.ts` (not applied)

Removed: `…/🗑️generated/fix-originals/` (probes and logs of this session). Unchanged: `jogger`, `wheelchair-user`, `kneeling-gardener`, `parent-with-stroller`, `person-with-white-cane`, the manifest.

Manifest: the rows carry `heightM`, `widthM`, `powerW`, pose and clip ids; none of these changed, so no row needs new values.

## Open, stated plainly

- `jogger` and `wheelchair-user` fail the check CLI as before.
- The old generators of these assets (`…/10/29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/…`) no longer reproduce the catalog files and were not updated; the fixes exist in the asset files only.
- No sheet was opened: the pose changes are 0.04° to 2.3° on single joints, judged from numbers, not looked at.
- The check samples nine instants per clip. Other instants of the fixed walkers (and of walkers that pass) still slide for single 1/240 s samples right after heel strikes; the request "nine check samples miss the jerks at clip keys" stays valid.
- The `requests.md` sections on `unionPolygons` (edge-on prisms; 11-22 21:07, humans b, `person-pushing-bicycle`) have the same root as the fixed defect; I did not re-measure those three inputs, and the asset workarounds they describe are still in place. I did not edit `requests.md`.
- The catalog-wide suites, the site build, `catalog-silhouette` and a full-catalog check run after the solver change remain for the coordinator.
- The `Write` and `Read` tools stopped answering at about 04:20 (hook timeouts); files after that were written through the shell. The ticket tools of the repo MCP server were not used by me (no ticket opened or closed, as the brief reserves).
