# 📝️ sonnet-structural: Report

Convention: `T/🏗️structural-projection.md`. Library `L` = `🎓️teaching/🏛️architecture/🧍️staffage`.

## Audit finding (item 1)

The old solver was purely sagittal (x, y, moment about z). Masses, anchors and loads were placed from the projected elevation affine; lateral loads, out-of-plane reactions and moments about x and y did not exist, and a slewed or spread member was solved with a foreshortened lever arm. The old views drew everything with elevation coordinates (`ctx.paper(point)`) in every viewpoint. Both are replaced.

## Changed files

- `L/🔨️modules/🏗️structure/🟦️.ts`: full 3D solver (6x6 elastic-support equilibrium by a Jacobi pseudo-inverse, 3D friction cone, footprint length and width moments, 3D member cuts with N, Vy, Vz, T, My, Mz, 3D joints, 3D residual, 3D free body); `structuralLegend(..., include)`; `shearOf`, `bendingOf`, `dominantOf`, `worldForce`.
- `L/🔨️modules/🏗️structure/📽️projection/🟦️.ts` (new): `planeOf`, `towardViewer`, `projectForce`, `projectMoment`, `tensionSide`, `magnitude`; imports the cameras of `🎨️depiction/🔭️views`.
- `L/🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts`: rewritten to project (force dot/cross symbols with value labels, moment curl and double arrow, view-specific supports, diagram ordinates and out-of-plane peak symbols, view-aware free-body inset with six residual components, one legend text for all views, dash encoding).
- `L/🔨️modules/🔗️contracts/🟦️.ts`: only the `🏗️Structure` region (Reaction, JointLoad, DiagramStation, StructuralResult residual and centre of mass) plus the sign docstrings (D6).
- `L/🔨️modules/🧮️evaluation/🟦️.ts`: `worldForces()` places loads by the bone frames and keeps `z`, `fzN`, `momentXNm`, `momentYNm`, `wzNPerM`; its call site passes `kin.now`. (Whole-file CRLF I introduced was reverted to LF.)
- `L/🔨️modules/🎨️depiction/🟦️.ts`: only `modelScene` analysis block (DXF arrows and diagrams project by `options.viewpoint`) and one import.
- `L/🔨️modules/🏗️structure/🧰️harness/🟦️.ts`: 3D case documents (bone `z`, `abductionDeg`, `twistDeg`, anchor `z`, 3D loads), frames from `solveChain`, `clipTimeS`.
- Tests: new `L/🧪️tests/🏗️structure-projection/` (🟦️.ts, 🐍️.py, 🥒️.feature) and `L/🧪️tests/🏗️rigid-trees-3d/` (same three files); fixtures `L/🧫️fixtures/🏗️balcony-lateral`, `🏗️crane-slewed`, `🏗️lean-lateral`, `🏗️projection-vectors`, `🏗️rigid-trees-3d`.
- Updated for the renamed moment fields: `🏗️cantilever`, `⚖️equilibrium`, `🌉️simply-supported` (D4), `🦶️centre-of-pressure`, `🧍️two-foot-stance`, `🪢️tied-canopy`, `🏃️inertial-consistency`, `🎨️structure-legend` tests; `🧫️fixtures/🌉️simply-supported/🔣️.json` and its 🐍️.py now store the staffage (sagging +) sign.
- T: `🏗️structural-projection.md`, this file.

## Coverage completed

Items S1 (view wired to the projection module and cameras), S3 (one `structuralLegend` over one member set), S4 (fixed range for contact pressure, no 1 Pa floor), S5 (3D residual asserted in all six components in every 3D case), D3 (unique `:cross` and `:hatch` ids), D4, D5, D6 (docstrings for Reaction, JointLoad, DiagramStation), D7, D8, D9, D23 (legends state components of M = r × F, left-handed frame, left-hand rule, per-view curl senses), O1 and O2 (German and English terminology in the solver notes).

## Verification evidence (vitest, config `L/🧪️tests/🎚️config/🟦️.ts`, run from `L/📦️packages/🟦️typescript`)

Structure set (cantilever, structure-projection, rigid-trees-3d, equilibrium, simply-supported, centre-of-pressure, two-foot-stance, tied-canopy, inertial-consistency, structure-legend, smooth-continuity): 204 passed, 1 failed. The one failure is `🌊️smooth-continuity > sparse four-key walk` (catalog validator codes `unavailable-cause-missing`, `symbol-front-missing`, extent checks of the sample asset), unrelated to statics.
`🏗️structure-projection` alone: 28 of 28; `🏗️rigid-trees-3d`: 33 tests (8 trees x 4 plus coverage), all passing; `🎨️structure-legend`: 5 of 5.
Older anastruct cases reproduce unchanged (147 of 147 before the new files). numpy oracles: `🐍️.py --check` prints OK for all five fixtures (numpy 2.5.4). anastruct is not installed in this environment, so the anastruct oracles were not regenerated; I deduced the anastruct midspan sign from the stored values (`float(-max(moments)) = -7000` means anastruct is sagging-positive there) and stored the simply-supported values unnegated.
`tsc -p tsconfig.json --noEmit`: no errors in structure, projection, structural view, harness, evaluation or contracts; remaining errors are in `🌐️thermal-multiview`, `📏️print-size`, `🔭️multiview-coverage`, `🧫️fixtures/🌡️thermal` (not mine).
Other suites failing in the same run, not mine: `♾️physical-invariance` (ViewMissingError for `sample-figure` front view) and `🔄️continuous-rotation` (valid-range-gap validator, a cycle-difference sample).

## Unresolved and cross-owner

1. React detail model (`🎯️targets/⚛️react/🔨️modules/🔬️detail/🧬️model/🟦️.tsx` lines ~144-160) still reads `load.momentNm`, `reaction.momentNm`, `structure.residual.momentNm`, `joint.momentNm`: rename to `mzNm` and show `fzN`, `mxNm`, `myNm`, `torsionNm`. Owner sonnet-viewer.
2. `L/🔨️modules/🎞️animation/🟦️.ts` (lines ~295-305) scales and blends `fxN`, `fyN`, `momentNm` only: a `load:<id>` track or a transition drops `fzN`, `momentXNm`, `momentYNm`, `z` and `distributed.wzNPerM`. Owner sonnet-rigs. This silently loses lateral loads in clips.
3. `WorldForce` in contracts (the contracts line of D6): state that `mxNm`, `myNm`, `mzNm` are the couple of the force about its own point and follow the sign rule of section 2 of `🏗️structural-projection.md`.
4. The solver deflection line is still only in the rig plane; lateral deflection of members is not computed. Torsion is not part of the extreme-fibre stress.
5. A statically indeterminate lateral support layout has no independent oracle beyond equilibrium (e.g. the tied canopy with a lateral load is reported unbalanced by design: one pin plus a tie rod cannot hold a lateral moment).
6. The core's label-budget suffix `n further values in the inspector` is the only text that differs between views of a legend.
7. The README structure paragraphs (haiku-docs) should mention the 3D solver and link `🏗️structural-projection.md`.

## Addendum 1: curtain-wall bay and the 3D support schema

- Schema (shared with contracts, lockstep JSON and TS twin): `Anchor.tiltDeg` (default 0, -90..90) lifts the support normal out of the sagittal plane towards +z; a lateral guide is a `roller` with `normalDeg 0`, `tiltDeg 90`. Solver: the contact and roller normal and the tangent plane `(t1, t2)` are fully 3D; the structural view draws a tilted support with the plan and front symbols.
- Answer for sonnet-buildings: head `pin` (x, y, z), base `roller` with `tiltDeg 90` (z only, free in y); the in-plane x of the seismic pose is taken at the head pin.
- New oracle case `🏗️curtain-wall-bay` (numpy plus the hand values): head +2467.3 N, base +2522.3 N, mx +1135.0 and +3022.5 N·m, shears +2522.3, +1161.5, -2467.3 N, all within 0.05; the sign of mx is documented in `🏗️structural-projection.md`.
- Structure set after this change: 202 of 202 (cantilever, structure-projection, rigid-trees-3d, equilibrium, simply-supported, centre-of-pressure, two-foot-stance, tied-canopy, inertial-consistency, structure-legend). `🧬️schema-conformance` failed in the same run (symbols lacking front and plan in the sample figure), which is contracts' work in progress.

## Addendum 2: sneakers axial, hatching, free-body texts

- Sneakers axial (-39.87 in the front, 0 in elevation and plan): not a per-view scalar. I rendered standing-adult in all three viewpoints and every bone has one identical axial value in all of them (`Left foot and shoe` -39.87, `Left forefoot` 0). The difference comes from which shapes a view draws: the front geometry has only the foot shapes (`shoe-l` on `foot-l`) and no `toe` shape, while elevation and plan also draw `shoe-toe-l` on `toe-l` (axial 0, a massless forefoot). A reader that takes one value for the shared region or label `sneakers` therefore sees -39.87 in the front and the last or first of two bones (0) elsewhere. Cure on the reading side: key by bone, not by region. New test `member scalars are the same in every view` asserts, for 7 asset and pose samples, that every bone has exactly one value and the same value for axial, shear and stress (forces view) and for bending (moments view) in all three viewpoints, and that the drawn axial equals the extreme station of the solver.
- Hatching: members now carry the hatch cue for shear, stress, axial and bending moment (diagonal = negative and warm, dots = positive and cool, above 15 % of their side of the range, when patterns are on); the moments view passes `options.patterns` (it passed `false` before); contact-pressure footprints are hatched and the pressure legend says so. The member legend interpretation names the hatch in en and de.
- Free-body diagram and Residual: now bilingual `Text` constants (`TEXT.freeBody`, `TEXT.residual`) read by `ctx.language`; no inline ternary. The React bundles (`🌐️i18n`) belong to the viewer target and the depiction library cannot import them, so the library keeps its own bilingual constants; the viewer key `viewer.diagram.freeBody` stays its own.
- `🏗️structure-projection`: 39 of 39.

## Addendum 3: D45 split supports, convergence criterion, swivel chair

- Comparison old (HEAD, 2D) against new (3D) solver over 922 catalog poses, scripts in `🛠️scripts/sonnet-structural/` (`compare.ts` imports the old solver from `old/`, regenerate with `git show HEAD:<L>/🔨️modules/🏗️structure/🟦️.ts`, imports rewritten to absolute paths and `momentNm` renamed `mzNm`; the copy is deleted again).
- Convergence defect: the active-set loop declared "did not converge" when the discrete set had been seen before while only continuous quantities (friction cap, centre-of-pressure cap) were still creeping, so a solution converged to 1e-11 reported failure (kneeling-gardener kneeling-weeding, leaning-person, seated-office-worker, the old solver as well). Fix: a repeat of the active-set key only counts as cycling when the key changed between iterations (`lastKey`), and the continuous update tolerance is 1e-9 relative instead of 1e-12. No catalog pose reports non-convergence any more (checked over all 922).
- Swivel chair `swivel-quarter`: not a solver bug and not truthful sliding alone. All five castors have `z = 0` (and the back two sit 0.05 above the ground), so they are collinear in plan while the swivelled seat moves the centre of mass 0.045 m sideways: the chair tips about the castor line and the model cannot carry the moment `mx`. The friction caps ("slides", true as stated) are the symptom of the least-squares fit. New note in en and de: `Collinear supports: ...` / `Kollineare Auflager: ...` whenever two or more active supports lie on one line in plan and the residual moment about that line is significant. Needs lateral castor positions from the objects worker (D45).
- D45 confirmation (anchors by `z`): tram 8 (±0.715), concrete-mixer-truck 8 (±1.115), excavator 8 (±1.2), forklift 4 (±0.5, ±0.24), wheel-loader, city-car, city-bus, delivery-van, diesel-generator, oil-pumpjack, dining-table, picnic-table, three-seat-sofa, operating-table, bar-stool, cafe-chair, stepladder and others are split with real z. Still all `z = 0`: office-swivel-chair (5 castors), cargo-bike, city-bicycle, motor-scooter, kick-scooter, vacuum-cleaner, courier-with-hand-truck, wheelchair-ramp, masonry-wall-with-lintel, sectional-garage-door; people and animals carry their lateral position through the bone frames.
- Expectation changes: a pose with several contacts at real lateral positions now has a statically indeterminate 3D distribution, so contacts that would pull lift off in more poses than in 2D (parent-with-stroller, walking-commuter, kneeling-gardener show fewer active contacts and `lifted` notes); this is the honest result, not a regression. New test `supports split per physical contact`: for 12 split assets the active reactions sum to weight minus applied load, all six residual components vanish, at least three contacts are active at two or more different `z`; plus a convergence test and a tipping-axis note test. Structure suite: 223 of 223.
