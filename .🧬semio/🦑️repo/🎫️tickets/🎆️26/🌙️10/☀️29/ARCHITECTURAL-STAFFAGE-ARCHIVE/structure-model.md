# Structural model of the staffage archive

Module: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts` (`solveStructure`, `structuralLegend`, `freeBody`).
Harness (hand-built `EvaluatedState` from a case document): `.../🏗️structure/🧰️harness/🟦️.ts`.
Cases: `🧪️tests/<case>/{🟦️.ts,🥒️.feature,🐍️.py}`, vectors and oracle values: `🧫️fixtures/<case>/🔣️.json`, shared oracle helper `🧪️tests/🏗️structure-oracle/🐍️.py`.

## 1. Model

The evaluated pose is treated as one instantaneously rigid body. External forces (all in world axes, N, N·m, CCW positive):

- weights `-m g ŷ`: point masses at the world position of `(comX, comY)`; uniform masses discretised into 16 sub-forces at the midpoints of 16 equal intervals of the bone (`comY` is the lateral offset, `comX` is ignored);
- inertia (only when `asset.structure.dynamics === "inertial"`): `-m a` with `a = kinematics.segmentAccelerations[bone]` at the same points; rotational inertia of the segments is neglected (the evaluated kinematics carry no angular acceleration);
- applied loads: `state.loads` as given (the evaluator is expected to have discretised distributed loads);
- reactions of the active supports (below).

### Supports
Active supports: anchors in `state.contacts` of kind `contact`/`support`, plus every anchor in `structure.supports` (always active). Type: `anchor.support`; if absent, kind `contact` is a unilateral `contact`, kind `support` is a `pin`.

Elastic-support model: the body displaces by `u = (ux, uy, θ)` about the mean support point; every support is a set of springs `k_i` along unit rows `d_i = (dx, dy, (p - c) × d)`. Normalised stiffness (documented constants `SUPPORT_STIFFNESS`): contact normal 1, contact friction 0.5, pin x/y 1, roller 1 along `normalDeg`, fixed x/y 1 and rotation 1 (per m²).

`K u = F`, `K = Σ k_i d_i d_iᵀ`, `F` = resultant of all non-reaction forces (moment about the reference point); the reaction of spring `i` on the body is `-k_i (d_i·u) d_i`, so `ΣR = -ΣF` exactly whenever `K` is regular. Hence:

- one fixed support, or pin + roller whose line of action misses the pin: exact statics, independent of the stiffness values;
- indeterminate support sets: the reactions follow the stated equal-stiffness assumption (documented in the result notes), not a measured support stiffness.

Unilateral contacts: if a contact normal force `N < 0` (it would pull), the most tensile contact is dropped (`active: false`, zero reaction) and the system is re-solved until no contact pulls. Dropped contacts are not re-added (no gap check). If every active support is a contact and every contact would pull, all but the contact closest to the centre-of-mass projection are dropped, that one is forced active and the notes say so (a pose with a contact listed never ends with zero active supports). The reference point of the 3x3 system is the mean of the currently active supports, so a single remaining contact keeps exact force balance. A pose whose centre of mass is outside the support polygon therefore ends with the far contact lifted and a non-zero moment residual, flagged in the notes.

Friction: `T = -k_t (t·u)`; where `|T| > μ N` the contact switches to sliding: the tangential spring is removed and a known tangential force `sign(T) μ N` (N from the previous iterate, fixed-point iteration, tolerance 1e-12 relative) is applied. `μ = anchor.frictionCoefficient`, default 0.6 (stated in the notes). A sliding contact that cannot be balanced leaves a non-zero residual which is reported unchanged (never hidden).

Contact pressure = `max(R·n, 0) / (footprintLengthM · footprintWidthM)` when both are authored, otherwise `null` (0 for lifted contacts with a footprint).

When no support is active (flight / ballistic), reactions are zero, the notes state "Unsupported ... flight phase / ballistic" and the residual is the unbalanced weight plus inertia, reported unchanged.
Singular systems: the 3x3 solve is exact when regular; when a degree of freedom is unrestrained (e.g. all contacts sliding) a 1e-11 relative regularisation is added, which leaves unmet force/moment in the residual instead of hiding it.

### Centre of pressure (footprint contacts)
A `contact` anchor with `footprintLengthM = l` may transmit a moment about the anchor with `|M| <= N l / 2` (elevation plane): rotational spring `k l² / 12` (rows `d = (0,0,1)`), engaged only when the support set cannot otherwise balance the moment (3x3 system singular without them, e.g. a single remaining foot), so the lever rule between two feet stays exact. Capped like friction (known moment `sign(M) N l/2`, fixed-point iteration, note "centre of pressure clamped ... the foot tips"). `Reaction.cop = anchor + t e` with `e = -M/N` (t = tangent of `normalDeg`), clamped to ±l/2; only for active footprint contacts with N > 0. Case `🦶️centre-of-pressure` (anastruct fixed-support oracle): COM 0.035 m beside the anchor → COP under the COM, residual 0; COM 0.177 m → COP clamped at 0.125 m toe, moment residual and tipping note. Reference-walker and sparse-smooth-walk pass the check CLI.

### Joint loads
`J_b` for every non-root bone: `S_b` = resultant of all external forces on the subtree of `b` (weights, inertia, applied, reactions).
`fxN, fyN = -S_b` (what the parent transmits to the child), `axialN = S_b·e_x`, `shearN = -S_b·e_y` (bone frame), `momentNm` = moment of the subtree's external loads about the joint (CCW +), i.e. the same value as the member-diagram moment at `s = 0` (inclusive side).
Remark for the coordinator: the force is the transmitted force `-S_b`, the moment follows the contracts text (moment of externals, not negated); the moment the parent transmits is `-momentNm`.

### Member diagrams
For bone `b` with `lengthM > 0` and cut at `s` (bone frame, `q` = cut point), the distal part is the bone beyond `s` plus every child subtree attached (at the projection of its origin onto the bone axis) at `s_c >= s`; `S(s)` = resultant of the externals on it:

`N = S·e_x` (tension +), `V = -S·e_y`, `M = Σ moment of those externals about q (CCW +)`.

With this definition a downward load on the free end of a cantilever gives `V = +P` and `M = -P (L - s)` (hogging), and a downward UDL on a simply supported beam gives sagging `M > 0`. Uniform weight sub-forces are integrated exactly across a cut (partial span), so the diagram is exact for piecewise-uniform loads.

Stations: 25 equidistant stations (24 intervals), the span edges of the uniform sub-forces, and a two-sided pair at every point load, reaction and child attachment (`s` identical, first = value just before the point, which includes the load at `s`, second = just after, which excludes it). Station `s = 0` of a pair therefore carries the full free-body resultant (`≈ 0` for a root member in equilibrium), and the second one the diagram value just inside the member.

Stress: `σ = N/A ∓ |M| c / I` (`stressMinPa`, `stressMaxPa`) where `section` exists, `null` otherwise.

Deflection (`deflection: Point[]`, `x = s` along the bone, `y = w` along local `+y`, 121 points): `w'' = M/EI` with trapezoid integration; boundary conditions: one fixed support alone, or a fixed parent joint with no other restraint, clamps `w = w' = 0`; two translation-restraining supports (pin, fixed excluded, roller/contact with transverse normal component) give `w = 0` at both; a revolute/prismatic parent joint counts as a pin at `s = 0`. All other cases return `null`. Shear and axial deformation neglected.

### Residual
`ΣF` and `ΣM` about the world origin of weights + inertia + applied + reactions (reactions include reaction moments). For static states with sufficient support it is zero to rounding; for the inertial model the d'Alembert forces are part of the sum, so it is also zero for a balanced instant. A non-zero value signals tipping, sliding, or missing supports (and adds an "Unbalanced" note).

### Depiction helpers
- `structuralLegend(quantity, result, range)` with `quantity ∈ axial | shear | stress | contact-pressure | moment`; stops: compression warm `#d7301f → #fdae61`, grey `#d9d9d9` at zero, tension cool `#abd9e9 → #2c7bb6`; only the stops of the present sign are emitted; `auto` range includes zero (pressure: `[0, max]`).
- `freeBody(result)` lists one resultant weight and one inertia arrow per bone (placed on the line of action of the group), each applied load, and each active reaction (`bone: null`).

## 2. Sign conventions
World x right, y up, CCW positive. N > 0 tension. V = -Σ distal transverse force. M > 0 sagging. Reactions in world axes (force on the structure). Deflection w along bone local +y. Mapping from anastruct: reaction = -node result, M = -M_anastruct, V = -Q_anastruct, w = -uy_anastruct.

## 3. Oracle results, tolerances, non-claims
| Case | Oracle | Quantities checked | Tolerance | Result |
|---|---|---|---|---|
| 🏗️cantilever | anastruct 1.7.0 + closed form | R_y = +1000 N, M_R = +2000 N·m CCW, M(x) = -P(L-x), V = +P, stress ±10.29 MPa, tip deflection -6.535e-4 m | 1e-9 (statics), 0.5 % vs anastruct and 1e-4 vs PL³/3EI (deflection) | pass |
| 🌉️simply-supported | anastruct + closed form | reactions 4500 N each, max M 7000 N·m, station moments, shear jump 5000 N, midspan w -2.4508e-3 m | 1e-9 statics, 0.5 % anastruct moment/deflection, 1e-4 closed-form deflection | pass |
| 🪢️tied-canopy | anastruct truss + closed form | rod tension 2347.87 N, pin (2100, 1050) N, canopy N = -2100 N, M = 0 at supports | 1e-4 vs anastruct, 1e-9 vs closed form | pass |
| 🧍️two-foot-stance | anastruct beam on hinge+roller, direct lever arms | foot reactions, pressures, heavier-foot flip (right → left), shoulder fx, fy, N, V, M, lift-off of the far foot | 1e-7 reactions, 1e-9 joint loads | pass |
| ⚖️equilibrium | numpy (determinate equilibrium; independent numpy assembly of the equal-stiffness springs for three pins) | 18 seeded trees: residual < 1e-9 relative, reactions, joint balance, root cut | 1e-9 | pass |
| 🏃️inertial-consistency | numpy | Σ R = Σ m (g + a), foot normals/tangents, friction cap μ N (μ = 0.2), flight, forced single contact, quasi-static twin | 1e-9 | pass |

Run: `repo tools exec bun -- x vitest run --config 🧪️tests/🎚️config/🟦️.ts <case folders>` from the staffage root: 129 tests passed. Oracles: `repo tools exec uv -- run --with anastruct python <ticket>/structure-oracles.py --write|--check [case]` (anastruct 1.7.0 and numpy resolved offline from the lock; anastruct's own mesh limits its sampled max moment/deflection to about 0.04 % / 0.2 % below the closed form, hence the 0.5 % tolerance). The `tsc -p` check of the module and tests reports no errors in structure files.

Non-claims: no validated biomechanics (joint values are rigid-segment resultants); support stiffness is a stated equal-stiffness assumption for indeterminate cases; no re-addition of lifted contacts (no gap check); rotational inertia of segments and shear/axial deformation are neglected; deflection only for clamped or two-point-supported members; distributed `AppliedLoad`s must already be discretised into `state.loads`; friction default μ = 0.6; a single remaining contact or any unbalanced instant reports a non-zero residual instead of a fabricated balance.

Reference-walker check (`🩺️check` CLI): force residuals up to 1e4 N remain because the evaluated segment accelerations of the walker clip contain spikes (inertial forces up to 1.2 kN vertical in one instant, the core worker's fix is pending), so one contact is lifted and a single foot cannot balance the moment or the horizontal inertia; the vertical force balance is exact whenever a contact is active.
