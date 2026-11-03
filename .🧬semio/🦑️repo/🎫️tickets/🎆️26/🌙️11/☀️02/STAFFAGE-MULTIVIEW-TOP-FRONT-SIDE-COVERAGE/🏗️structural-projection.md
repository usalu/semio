# 🏗️ Structural Projection: Forces, Moments, Supports and Diagrams in Side Elevation, Front Elevation and Plan

Owner: sonnet-structural. Code: `L/🔨️modules/🏗️structure` (solver, `📽️projection`) and `L/🔨️modules/🎨️depiction/🏗️structural-view`. Tests: `L/🧪️tests/🏗️structure-projection`, `🏗️rigid-trees-3d`, and the older structure cases.
Views use the approved names: Side elevation / Seitenansicht, Front elevation / Vorderansicht, Plan (top view) / Draufsicht.

## 1. One physical calculation, three projections

`solveStructure(state)` works in physical metres and newtons from the 3D bone frames of the evaluated state (`EvaluatedBone.frame`). Weights, d'Alembert forces, applied loads (`WorldForce` with `z`, `fzN`, `mxNm`, `myNm`, `mzNm`), supports (anchors with `z`), reactions, joint loads and member cuts are all three-dimensional. Nothing in the solver knows a viewpoint.
A view reads the one `StructuralResult` and only projects it. No lever arm is recomputed in screen space and no component is dropped: a force or moment keeps its three components in the result.

Audit result of the old code: the previous solver was purely sagittal. Masses, anchors and loads were placed from the projected elevation affine, so a slewed or spread member used foreshortened lever arms, and lateral loads, out-of-plane reactions and the moments about x and y did not exist. That is replaced, not wrapped.

## 2. Frame, signs and reference points

World axes: x ahead, y up, z the asset's left. This triple is left-handed (decision D6; the code keeps it).

| Quantity | Sign and reference |
|---|---|
| Force `(fxN, fyN, fzN)` | along +x, +y, +z |
| Moment `(mxNm, myNm, mzNm)` | the components of `M = r × F`; positive about axis k turns i towards j for the cyclic order (i, j, k): `mx` turns y towards z, `my` turns z towards x, `mz` turns x towards y |
| Reaction moment | the couple the support applies; independent of a reference point |
| Joint moment | moment of the child subtree's external loads about the joint origin |
| Member cut moment | moment of the distal external loads about the cut point (`momentVector`, world axes) |
| Residual | sums of all forces and of all moments about the world origin |
| Member components | `axialN` along u (tension +), `shearN = -F·v`, `shearZN = -F·w`, `torsionNm = M·u`, `momentNm = M·w` (sagging +, tension at local -y), `momentLateralNm = M·v` |

Because the frame is left-handed, a moment arrow follows the LEFT-hand rule (decision D23). Converted to a right-handed frame `(x, y, -z)` a moment is `(-mx, -my, mz)`; the numpy oracles do exactly this conversion.

### Support restraint fields (3D schema)

A support restrains through its `Anchor` fields `support`, `normalDeg` and `tiltDeg`:

| `support` | Restrains |
|---|---|
| `pin` | translation in x, y and z (no moment) |
| `roller` | translation along the normal `n = (cos normalDeg cos tiltDeg, sin normalDeg cos tiltDeg, sin tiltDeg)` only |
| `fixed` | all three translations and all three rotations |
| `contact` | compression along `n` plus friction in the tangent plane `(t1, t2)` (cone `μ·N`), footprint moments about `t2` (length) and `t1` (width) |

`tiltDeg` (new, default 0, -90 to 90) lifts the normal out of the sagittal plane towards +z. A lateral guide (a slip joint free in y that holds only z) is a `roller` with `normalDeg 0` and `tiltDeg 90`. The curtain-wall bay is therefore a head `pin` (x, y, z) and a base `roller` with `tiltDeg 90` (z only); the in-plane x of the seismic pose is taken at the head pin.

### Sign of mx, checked on the curtain-wall bay

Suction acts outward, -z, at height y: the moment about x of the cut is `mx = Σ (y_load - y_cut)·fz` over the distal loads. With the head reaction `+2467.3 N` at y 3.3 and the loads `-3628.8 N` at 2.075 and `-1360.8 N` at 0.45, the cut moment is `mx = +1135.0 N·m` at 0.45 and `+3022.5 N·m` at 2.075 (positive: `mx` turns y towards z, and a reaction pushing +z above the cut does so). For the vertical mullion (u = +y, v = -x, w = +z) the lateral bending moment is `M·v = -mx`, so `momentLateralNm = -3022.5` at the peak and `momentNm = M·w = mz = 0`; the shear `shearZN = -F·w` is `+2522.3`, `+1161.5`, `-2467.3 N` along the bay. The numpy oracle (`🧪️tests/🏗️structure-projection`, case `🏗️curtain-wall-bay`) agrees with these hand values within 0.05 N and N·m. Seen in the front elevation `mx` is a curl, clockwise when positive; in the side elevation and plan it is a double arrow along x.

## 3. Cameras and what a component along the line of sight is

The cameras are those of `L/🔨️modules/🎨️depiction/🔭️views` (`project`, `CAMERAS`); `📽️projection` imports them, there is no second table.

| View | Seen from | Paper x | Paper up | Toward the viewer |
|---|---|---|---|---|
| Side elevation | the asset's right, -z | x | y | -z |
| Front elevation | ahead, +x | z | y | +x |
| Plan | above, +y | x | z | +y |

A viewpoint changes only this basis. Pose, time, load case, scale and legend do not depend on it.

## 4. Forces

An arrow is the in-plane part of the force acting on the body, its head at the point of application, its length proportional to the in-plane magnitude on the one scale `longest ≙ max |F|` (the largest total magnitude of all forces, the same in every view).
A component along the line of sight is explicit: a ringed dot (towards the viewer) or a ringed cross (away) at the head, with a radius that grows with the component, and the label states it: `Hook 5.0 kN (⊙ 3.0 kN)`. The label always carries the full invariant magnitude `|F|`; `|F|² = in-plane² + out-of-plane²` holds in every view (tested).
Examples: a lateral wind load (z) is a dot or cross in the side elevation and a full arrow in plan and front; a weight is a dot or cross in plan and a full arrow in the other two.

## 5. Moments

A moment is drawn from its three components, scaled by the one total `|M|`:

| Part | Symbol |
|---|---|
| Component along the line of sight | a curl (arc with a head), counter-clockwise as seen when positive: positive `mz` is counter-clockwise in the side elevation, positive `mx` clockwise in the front elevation, positive `my` clockwise in the plan |
| Components in the view plane | one double-headed arrow along the in-plane components of `M` (paper axes), left-hand rule |

The label gives the total and the two parts: `Wall 2.1 kN·m (↺ 2.0 kN·m, ⟷ 0.6 kN·m)`. A moment below 2 % of the largest is not drawn (the legend says so). Reaction, joint and applied moments use the same symbols.
Example: a balcony with a lateral tip load has `my ≠ 0` at the wall: in the side elevation it is a double arrow along y, in the plan it is a clockwise-or-counter-clockwise curl, in the front elevation a double arrow along y again.

## 6. Supports

Side elevation: the existing symbols oriented by the anchor normal (pin triangle, roller triangle with wheels, fixed hatched wall, contact footprint). Plan and front: the footprint of a contact as the projected rectangle (length along the tangent, width along z, at least 0.7 mm thick), pin as a ringed cross, roller as a ringed slot, fixed as a hatched square. Shape, not colour, tells the types apart.
Contact reactions include the centre of pressure `cop` as a 3D point inside the length and width of the footprint (`|M| ≤ N·l/2` about the lateral axis, `|M| ≤ N·b/2` about the tangent axis); a contact transmits no torque about its normal.

## 7. Diagrams

Ordinates are offset perpendicular to the projected member, on the one scale `6 mm ≙ peak`, where `peak` is the invariant maximum over members of the 3D vector (|N|, |V| of both shears, |M| of both bendings), so the legend text is identical in every view.
The ordinate is the in-plane part of the 3D vector: the shear vector `-Σ distal transverse force`, or the tension-side direction `-(M × u)` of the bending moment. Where the vector has a component along the line of sight, a ringed dot or cross at the member's peak station refers to that same vector (the shear force vector, or the tension-side direction of the bending moment), and the label states the total and that component. A member that lies along the line of sight has no projected direction and shows only these symbols.

## 8. Legends and colour independence

The legends are identical in all views (tested; the only view-dependent text is the core's `n further values in the inspector` suffix when the label budget omits labels). Colour is never the only code: arrow kinds differ by outline dash (applied solid, reaction long dash, weight short dash, inertia dotted), moment kinds likewise, supports by shape, compression and tension by hatch where the pattern option is on.
Member colours use one function over one member set: `structuralLegend(quantity, result, range, include)`; the view passes the members it colours, so the drawn legend and the function agree. A `fixed` range is honoured for every quantity including the contact pressure.
The sign statement of every moment legend (en and de): the components are those of `M = r × F` in (x, y, z); the frame is left-handed, so the arrow follows the left-hand rule; positive `mz` is counter-clockwise in the side elevation, positive `mx` clockwise in the front elevation and positive `my` clockwise in the plan.

## 9. Load cases

Applied loads come from the state (`loads`, scaled by `load:<id>` tracks) and carry `z`, `fzN` and the three couples. A view never adds, removes or reorders loads; all three views of one state read the same `StructuralResult` (tested: the result is bit-identical before and after depicting all views).

## 10. Verification

| Check | Oracle |
|---|---|
| Balcony with lateral load, reaction in all six components, lateral shear, lateral bending, no torsion | numpy statics in a right-handed frame, explicit conversion |
| Crane slewed 90 degrees: base moment about the forward axis, jib bending by the full lever arm, jib direction | numpy, hand geometry |
| Person leaning sideways on two feet: lateral lever rule, shoulder moment about the forward axis | numpy |
| Eight seeded 3D rigid trees with lateral offsets, abduction, twist: root reaction, every joint (force, moment vector, torsion, both bendings), member cuts | numpy matrix-product frames and wrench |
| Projection of 8 vectors in three views, force plane, line-of-sight component, moment arrow and curl | numpy cameras derived from the viewer position and `right = up × toward_viewer` |
| Old 2D cases (anastruct): cantilever, simply supported, tied canopy, centre of pressure, two-foot stance, equilibrium | anastruct and closed forms, unchanged results |

Not covered: lateral deflection of members (the deflection line stays in the rig plane), torsion in the stress, and a structure with several indeterminate lateral supports has no independent oracle beyond equilibrium of the elastic-support model.
