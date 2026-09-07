# W7 — Kernel Benchmarks Become Real Correctness Gates

Scope: `✏️s/🔨️modules/🏗️fem/⚙️engine/**` `#[cfg(test)]` modules only. Addresses gaps 2 and 3 of
`📓️explore-engine-kernel.md` and items 1–9 of its benchmark-suite proposal.

**Not compiled or run** (host swap exhausted, per the standing instruction — the coordinator
compiles later). Every edited file was re-read and parse-checked with `rustfmt --emit stdout`
(exit 0 on all three), and every kernel API called was grepped against its definition.

## 0. Reference provenance

Every literal in the new assertions comes from
`🔨️w7-kernel-references.py` (this folder), run as `uv run --no-sync python <file> [section…]`.
It computes each number **twice** wherever possible:

| Quantity | Third party | Second, independent path |
|---|---|---|
| Cook Quad4/Quad8 | scikit-fem 12.0.2 `ElementQuad1` / `ElementQuadS2` | hand-assembled numpy isoparametric quad |
| Hex8 cantilever | scikit-fem `ElementHex1` | hand-assembled numpy trilinear hex |
| MacNeal-Harder Quad4 | scikit-fem `ElementQuad1` | hand-assembled numpy quad |
| SS plate `w_max` | scikit-fem `ElementTriMorley` (Kirchhoff plate) | Navier double sine series (closed form) + numpy Batoz-DKT |
| Cantilever modal | — | scipy `linalg.eigh` on the consistent-mass Hermite beam system + `cos x cosh x + 1 = 0` roots |
| Euler buckling | — | scipy geometric-stiffness eigenproblem + `π²EI/(KL)²` / `tan x = x` root |
| Beam & portal frame | — | numpy/scipy direct-stiffness solver (`Frame2d`) + textbook closed forms |

Quadrature was matched deliberately so the third-party value is comparable to the last digit:
`intorder=2` for `ElementQuad1`/`ElementHex1` (2×2 / 2×2×2, matching `Quad4::rule` / `hex8_gauss_points`)
and `intorder=4` for `ElementQuadS2` (3×3, matching `Quad8::rule`). Plane stress is obtained in
scikit-fem via `lam* = Eν/(1-ν²)`. With those settings numpy and scikit-fem agree to **1e-11 or
better on every mesh** — that agreement is what licenses hard-coding the numbers.

## 1. Before / after, per test

| # | Test | Before | After |
|---|---|---|---|
| 1 | `analyses::modal_cantilever_matches_analytical_frequencies` | 10 % vs closed form, 3 modes | **2 %**, same 9-element mesh, docstring records the 0.0001/0.005/0.039 % scipy-measured discretisation error |
| 2 | `analyses::buckling_euler_column_matches_analytical_load` | 10 %, pinned-pinned only | **2 %**, four end conditions K = 1.0 / 0.5 / 0.7 / 2.0, each with its own zero-transverse-displacement pre-check |
| 3a | `elements2d::quad4_cooks_membrane_tip_deflection_is_positive_and_finite` | "positive and finite" | **replaced** by `quad4_and_quad8_cooks_membrane_match_reference_tip_deflection`: three same-mesh scikit-fem values at 1e-6, plus monotone convergence to 23.96 and Quad8 4×4 within 1.5 % of it |
| 3b | `elements2d::plate_dkt_simply_supported_square_center_deflection_right_order_of_magnitude` | ratio in (0.5, 2.0) | **replaced** by `…_matches_closed_form`: same-mesh DKT references at 1e-6 on 4×4 and 8×8, monotone convergence, 8×8 within **2 %** of `0.00406 q a⁴/D` |
| 3c | `elements3d::hex8_meshed_cantilever_deflection_is_right_order_of_magnitude` | ratio in (0.02, 3.0) | **replaced** by `hex8_meshed_cantilever_matches_reference_and_beam_theory`: same-mesh scikit-fem references at 1e-6 on 8×2×2 and 12×2×5, monotone convergence, refined mesh within **5 %** of `PL³/3EI + PL/(κGA)` |
| 4a | `elements2d::beam_eb2_cantilever_matches_hand_calc` | kept (already 1e-6 vs `PL³/3EI`) | unchanged |
| 4b | *(absent)* | — | **new** `beam_eb2_simply_supported_udl_matches_closed_form`: midspan `5wL⁴/384EI`, end rotations `wL³/24EI`, reactions `wL/2`, all at 1e-9 |
| 4c | *(absent)* | — | **new** `beam_eb2_propped_cantilever_reactions_match_closed_form`: `5wL/8`, `3wL/8`, `wL²/8` at 1e-9 |
| 4d | *(absent)* | — | **new** `beam_eb2_portal_frame_sway_matches_stiffness_method`: 10 displacement/reaction values vs the scipy stiffness-method solution at 1e-9, plus global force and moment equilibrium |
| 5 | patch tests (`tri3/tri6/quad4/quad8/plate_dkt/tet4/hex8/shell_facet3`) | kept, already exact | unchanged; **new** `quad4_macneal_harder_distorted_cantilever_matches_reference_sensitivity` adds the distorted-mesh case |

Files touched: `🧮️analyses/🦀️.rs`, `📏️elements2d/🦀️.rs`, `🧱️elements3d/🦀️.rs`.

## 2. Per-benchmark detail

### 1 · Modal cantilever — 10 % → 2 %, mesh unchanged at 9 elements

E=200 GPa, `Iy`=1e-5, A=0.01, ρ=7850, L=3, 9 `BeamEb2` elements.

`sparse::SUBSPACE_MAXIMUM_ORDER = 40` caps the free-DOF count, so the mesh can never exceed 13
elements (3 DOF/node) — but it does not need to. scipy `eigh` on the identical consistent-mass
system puts 9 elements at **0.0001 % / 0.005 % / 0.039 %** above the closed form for modes 1–3
(12 elements would give 0.00004 / 0.0016 / 0.0124 %). 2 % therefore gates `subspace_iteration`
convergence and the mass assembly, not the mesh — the mesh contributes ~2 % of the budget.
`β_nL` upgraded from 6 to 16 digits.

### 2 · Euler buckling — 10 % → 2 %, one case → four

Same 7-element column (E=200 GPa, `Iy`=8e-6, A=0.005, L=3), axial reference load −1 N at the tip.
Support sets: pinned-pinned `(Tx,Ty | Ty)`, fixed-fixed `(Tx,Ty,Rz | Ty,Rz)`, fixed-pinned
`(Tx,Ty,Rz | Ty)`, fixed-free `(Tx,Ty,Rz | —)`. The scipy geometric-stiffness eigenproblem on the
same discretisation lands 0.006 % / 0.087 % / 0.265 % / 0.0004 % from `π²EI/(KL)²`. The
fixed-pinned case's 0.265 % is almost entirely the K = 0.7 rounding — against the exact
`4.4934²EI/L²` root of `tan x = x` the same FE answer is 0.023 % off. 2 % is a genuine gate for
all four.

### 3 · Cook's membrane — reference values on the same mesh

E=1, ν=1/3, t=1, total tip shear 1, tip point (48, 52), consistent edge loads
(`h/2,h/2` for Quad4; `h/6, 2h/3, h/6` for Quad8 — note the old test's `1/(n+1)` per node was
**not** the consistent distribution and therefore not comparable to any published value).
The Quad4 sequence reproduces the classical Cook table exactly:

| mesh | Quad4 | Quad8 |
|---|---|---|
| 2×2 | 11.845179503502 | 22.717747347876 |
| 4×4 | **18.299165832569** | **23.708288809430** |
| 8×8 | **22.079183389482** | 23.883744169981 |
| 16×16 | 23.430411260065 | — |
| 32×32 | 23.817633955743 | — |

Bold values are asserted (1e-6 relative). Meshes were kept at ≤ 162 DOFs so the dense
`solve_linear_static` path stays cheap.

### 4 · Simply supported square plate — reference values plus closed form

a=2, t=0.01, E=2e11, ν=0.3, q=1000 ⇒ D = 18315.018315.

* Navier double sine series: `w_max D/(q a⁴) = 0.00406235` — i.e. the tabulated **0.00406** is
  correct to the digit given.
* scikit-fem `ElementTriMorley` converges to that series value: 7.53 % → 1.89 % → 0.47 % →
  **0.12 %** at 289 / 1089 / 4225 / 16641 DOFs. Independent third-party confirmation of the constant.
* numpy Batoz-DKT on the *identical* mesh the Rust test builds (cells split on the
  `(i,j)-(i+1,j+1)` diagonal, `Tz=0` on the boundary, `q·A/3` lumping):

| mesh | triangles | DOFs | `w_centre` | vs 0.00406 |
|---|---|---|---|---|
| 2×2 | 8 | 27 | 2.920511165096e-3 | 17.66 % |
| 4×4 | 32 | 75 | **3.406677456811e-3** | 3.95 % |
| 6×6 | 72 | 147 | 3.487709484707e-3 | 1.67 % |
| 8×8 | 128 | 243 | **3.514848272890e-3** | 0.90 % |
| 10×10 | 200 | 363 | 3.527201297181e-3 | 0.55 % |
| 12×12 | 288 | 507 | 3.533860610523e-3 | 0.37 % |

Test asserts the two bold values at 1e-6, monotone convergence from below, and 8×8 within 2 %
of the closed form (it lands at 0.90 %).

### 5 · Hex8 meshed cantilever — reference values plus Timoshenko closed form

The original geometry (L=4, b=0.2, h=0.3) could not reach 5 %: with L/h ≈ 13 the trilinear hex
shear-locks so hard that even 3675 DOFs only reach 96 % of beam theory, and the dense solver
cannot afford that. The test was re-cast on a **4.0 × 1.0 × 2.0 box** (L/h = 2, so the shear term
is a substantial 19.5 % of the bending term — which is exactly the point of the "+ shear
correction" in the reference formula) and moved onto the sparse `analyses::solve_multi_case`
pipeline, which has no `SUBSPACE_MAXIMUM_ORDER`-style cap and no O(n³) dense factorisation.

E=200 GPa, ν=0.3, P=10 kN spread over the `x=L` face, clamped face at `x=0`.
`PL³/3EI = 1.600e-6`, `PL/(κGA) = 3.120e-7` (κ=5/6) ⇒ closed form **1.912e-6 m**.

| mesh | DOFs | tip `Tz` (numpy = scikit-fem) | `|tip|`/closed |
|---|---|---|---|
| 4×1×1 | 60 | -1.509225301129e-6 | 0.78934 |
| 8×2×2 | 243 | **-1.722214505218e-6** | 0.90074 |
| 8×2×3 | 324 | -1.763045061481e-6 | 0.92209 |
| 12×2×5 | 702 | **-1.822205573460e-6** | 0.95304 |
| 16×2×6 | 1071 | -1.840505985162e-6 | 0.96261 |

numpy and scikit-fem agree to twelve digits on all of these (the 2×2×2 rule is exact on a
rectangular hex). Test asserts the two bold values at 1e-6, monotone convergence, and 12×2×5
within 5 % of the closed form (it lands at 4.70 %).

### 6 · Closed-form beam / frame checks

Simply supported beam, 4 `BeamEb2` elements, L=6, w=2000, E=200 GPa, `Iy`=1e-5. Consistent nodal
loads make cubic-Hermite **nodal** values exact for a UDL, so 1e-9 is the right bar:

* midspan `-1.687499999999994e-2` vs `-5wL⁴/384EI = -1.6875e-2` (3.3e-15 relative)
* end rotations ∓`9.0e-3` vs `wL³/24EI = 9.0e-3`
* reactions 6000 N each vs `wL/2`

Propped cantilever, same beam: fixed-end `Ty` 7500 N (`5wL/8`), prop `Ty` 4500 N (`3wL/8`),
fixed-end `Rz` 9000 N·m (`wL²/8`). Signs follow the kernel's own `r = Ku − f` reaction convention,
which the Python solver reproduces exactly.

Single-bay portal frame, both bases fully fixed, 6 m span, 4 m columns, columns
E=210 GPa/A=0.008/I=8e-5, rafter E=210 GPa/A=0.012/I=2e-4, 15 kN lateral at the windward eaves:

| quantity | value |
|---|---|
| `ux` eaves left / right | 3.045764339376e-3 / 3.027946678835e-3 m |
| `rz` eaves left / right | -3.297738248139e-4 / -3.261293033395e-4 rad |
| base `Fx` left / right | -7516.582572708 / -7483.417427292 N |
| base `Fy` left / right | -4540.867810293 / +4540.867810293 N |
| base `Mz` left / right | 16418.215209634 / 16336.577928609 N·m |

All ten asserted at 1e-9 relative; the test additionally re-derives global `ΣFx` and `ΣMz`
equilibrium (the Python solver's own moment residual about the origin is 9.7e-10 N·m).

### 7 · MacNeal-Harder distorted-mesh sensitivity (Quad4)

Standard strip L=6, h=0.2, t=0.1, E=1e7, ν=0.3, six elements, unit tip shear, beam-theory tip
deflection 0.1081. Three meshes: rectangular, 45° parallelogram, 45° alternating trapezoid
(interior mesh lines slanted ±h/2 top/bottom; end lines vertical).

| mesh | tip `v` (numpy = scikit-fem to <1e-13) | normalized |
|---|---|---|
| rectangular | 0.010088000000 | 0.09332 |
| parallelogram | 0.002613057737 | 0.02417 |
| trapezoidal | 0.002908744060 | 0.02691 |

**Important framing recorded in the test's docstring:** MacNeal & Harder's published
0.904 / 0.080 / 0.071 row is for a QUAD4 *with incompatible modes*. This kernel's `Quad4` is a
plain fully-integrated bilinear quad, which locks to ~0.093 rectangular — so the test asserts the
exact same-mesh values (1e-6) plus the *sensitivity ordering*: rectangular must stay under 0.12 of
theory, and each distorted mesh must cost at least a further factor of three. Adding incompatible
modes to `Quad4` would move these numbers toward the published row; that is a formulation change,
out of scope here, and is the follow-up this benchmark now makes visible.

## 3. Script output (verbatim)

```
##############################################################################
## 1 · modal cantilever (BeamEb2, consistent mass)
##############################################################################
beta_n*L roots of cos(x)cosh(x)+1=0 : [1.875104069 4.694091133 7.854757438]
analytic f_n [Hz]                  : [  9.924497896  62.195766651 174.149947569]
n= 9 elements (27 free dofs) fe=[  9.924511  62.198891 174.216924] rel-err=[0.0001 0.005  0.0385] %
n=12 elements (36 free dofs) fe=[  9.924502  62.196765 174.17158 ] rel-err=[4.1288e-05 1.6053e-03 1.2422e-02] %
n=13 elements (39 free dofs) fe=[  9.924501  62.196493 174.165714] rel-err=[3.0000e-05 1.1677e-03 9.0534e-03] %

##############################################################################
## 2 · Euler column buckling (BeamEb2 + geometric stiffness)
##############################################################################
pinned-pinned (K=1.0)    fe_Pcr=1754694.177328 N   pi^2EI/(KL)^2=1754596.337971 N   rel= 0.0056 %
fixed-fixed (K=0.5)      fe_Pcr=7024460.631334 N   pi^2EI/(KL)^2=7018385.351886 N   rel= 0.0866 %
fixed-pinned (K=0.7)     fe_Pcr=3590288.884977 N   pi^2EI/(KL)^2=3580808.853003 N   rel= 0.2647 %   exact(4.4934^2 EI/L^2)=3589462.854476 rel=0.0230 %
fixed-free (K=2.0)       fe_Pcr= 438650.625233 N   pi^2EI/(KL)^2= 438649.084493 N   rel= 0.0004 %

##############################################################################
## 3 · Cook's membrane (E=1, nu=1/3, t=1, total shear F=1, tip point (48,52))
##############################################################################
published converged reference tip deflection: 23.96
Quad4  2x2  numpy=11.845179503502  skfem=11.845179503502  diff=1.652e-13  load_sum=1.000000  ratio_to_23.96=0.4944
Quad4  4x4  numpy=18.299165832569  skfem=18.299165832569  diff=6.501e-13  load_sum=1.000000  ratio_to_23.96=0.7637
Quad4  8x8  numpy=22.079183389482  skfem=22.079183389484  diff=2.416e-12  load_sum=1.000000  ratio_to_23.96=0.9215
Quad4 16x16 numpy=23.430411260065  skfem=23.430411260066  diff=1.485e-12  load_sum=1.000000  ratio_to_23.96=0.9779
Quad4 32x32 numpy=23.817633955743  skfem=23.817633955750  diff=6.938e-12  load_sum=1.000000  ratio_to_23.96=0.9941
Quad8  2x2  numpy=22.717747347876  skfem=22.717747347875  diff=1.076e-12  load_sum=1.000000  ratio_to_23.96=0.9482
Quad8  4x4  numpy=23.708288809430  skfem=23.708288809433  diff=3.322e-12  load_sum=1.000000  ratio_to_23.96=0.9895
Quad8  8x8  numpy=23.883744169981  skfem=23.883744169973  diff=8.377e-12  load_sum=1.000000  ratio_to_23.96=0.9968

##############################################################################
## 4 · simply supported square plate under UDL
##############################################################################
D = 18315.018315
Navier series w_max            = 3.548871284e-03 m
alpha = w_max D /(q a^4)       = 0.00406235  (tabulated 0.00406)
closed form 0.00406 q a^4 / D  = 3.546816000e-03 m
DKT  2x2  (  8 triangles,   27 dofs) w_centre=2.920511165096e-03  rel-to-0.00406=17.658 %  rel-to-Navier=17.706 %
DKT  4x4  ( 32 triangles,   75 dofs) w_centre=3.406677456811e-03  rel-to-0.00406= 3.951 %  rel-to-Navier= 4.007 %
DKT  6x6  ( 72 triangles,  147 dofs) w_centre=3.487709484707e-03  rel-to-0.00406= 1.666 %  rel-to-Navier= 1.723 %
DKT  8x8  (128 triangles,  243 dofs) w_centre=3.514848272890e-03  rel-to-0.00406= 0.901 %  rel-to-Navier= 0.959 %
DKT 10x10 (200 triangles,  363 dofs) w_centre=3.527201297181e-03  rel-to-0.00406= 0.553 %  rel-to-Navier= 0.611 %
DKT 12x12 (288 triangles,  507 dofs) w_centre=3.533860610523e-03  rel-to-0.00406= 0.365 %  rel-to-Navier= 0.423 %
Morley refine=3 (  289 dofs) w_centre=3.816086129e-03  rel-to-Navier= 7.530 %
Morley refine=4 ( 1089 dofs) w_centre=3.616049161e-03  rel-to-Navier= 1.893 %
Morley refine=5 ( 4225 dofs) w_centre=3.565690122e-03  rel-to-Navier= 0.474 %
Morley refine=6 (16641 dofs) w_centre=3.553077535e-03  rel-to-Navier= 0.119 %

##############################################################################
## 5 · Hex8 meshed cantilever vs Euler-Bernoulli + shear correction
##############################################################################
geometry L=4.0 b=1.0 h=2.0 E=200000000000.0 nu=0.3 P=10000.0
PL^3/3EI          = 1.600000000e-06
PL/(kappa G A)    = 3.120000000e-07  (19.50 % of bending)
total closed form = 1.912000000e-06
hex  4x1x1 (  20 nodes,    60 dofs) tip_dz=-1.509225301129e-06  |tip|/closed=0.78934
hex  8x2x2 (  81 nodes,   243 dofs) tip_dz=-1.722214505218e-06  |tip|/closed=0.90074
hex  8x2x3 ( 108 nodes,   324 dofs) tip_dz=-1.763045061481e-06  |tip|/closed=0.92209
hex 12x2x5 ( 234 nodes,   702 dofs) tip_dz=-1.822205573460e-06  |tip|/closed=0.95304
hex 16x2x6 ( 357 nodes,  1071 dofs) tip_dz=-1.840505985162e-06  |tip|/closed=0.96261
skfem ElementHex1 8x2x2 tip_dz=-1.722214505218e-06  |tip|/closed=0.90074
skfem ElementHex1 12x2x5 tip_dz=-1.822205573460e-06  |tip|/closed=0.95304
skfem ElementHex1 16x2x6 tip_dz=-1.840505985162e-06  |tip|/closed=0.96261

##############################################################################
## 6 · closed-form beam / frame checks
##############################################################################
--- simply supported beam under UDL (4 BeamEb2 elements) ---
midspan v = -1.687499999999994e-02   5wL^4/384EI = -1.687500000000000e-02  rel=3.290e-15
end rotations = -9.000000000000e-03 / 9.000000000000e-03   wL^3/24EI = 9.000000000000e-03
support reactions Ty = 6000.000000000 / 6000.000000000   wL/2 = 6000.000000000
--- propped cantilever under UDL (4 BeamEb2 elements) ---
fixed-end reaction  Ty = 7500.000000000   5wL/8 = 7500.000000000
fixed-end moment    Rz = 9000.000000000   -wL^2/8 = -9000.000000000
prop reaction       Ty = 4500.000000000   3wL/8 = 4500.000000000
max sag at x=0.4375L, w=wL^4/185EI = 7.005405405e-03
--- 2D portal frame, lateral sway (stiffness method, scipy) ---
sway ux(node1) = 3.045764339376e-03   ux(node2) = 3.027946678835e-03
base reactions Fx = -7516.582572708 / -7483.417427292   sum = -15000.000000000  (must equal -15000.0)
base reactions Fy = -4540.867810293 / 4540.867810293   sum = 0.000e+00
base moments   Mz = 16418.215209634 / 16336.577928609
global Mz equilibrium residual about (0,0) = -9.677024e-10
rotations rz(node1) = -3.297738248139e-04   rz(node2) = -3.261293033395e-04

##############################################################################
## 7 · MacNeal-Harder straight cantilever, Quad4 distortion sensitivity
##############################################################################
E=1e7 nu=0.3 t=0.1 L=6 h=0.2, 6 elements, unit tip shear; beam theory tip v = 0.1081
(normalized values for a PLAIN fully-integrated bilinear quad; MacNeal-Harder's own 0.904/
 0.071/0.080 table row is for QUAD4 WITH incompatible modes, which this kernel does not have)
rectangular     numpy tip=0.010088000000  skfem tip=0.010088000000  diff=9.803e-14  normalized=0.09332
parallelogram   numpy tip=0.002613057737  skfem tip=0.002613057737  diff=2.095e-15  normalized=0.02417
trapezoidal     numpy tip=0.002908744060  skfem tip=0.002908744060  diff=8.854e-15  normalized=0.02691
```

## 4. Constraints found in the kernel that shaped these tests

1. **`sparse::SUBSPACE_MAXIMUM_ORDER = 40`** (`🔢️sparse/🦀️.rs:2922`) plus the
   `NUMERICAL_OWNER_PAGE_BYTES = 4096` capacity gates in `SubspaceIterationJob::new`
   (`:2938-2943`) hard-cap any modal/buckling test at ~13 beam elements. `SubspaceIterationJob`
   faults (`m = 0`) rather than degrading, and the free `subspace_iteration` wrapper turns that
   fault into `panic!("subspace batch adapter faulted")`. That is why test 1 keeps 9 elements
   instead of refining — it does not need to, and refining much further would trip the gate.
2. **`model::solve_linear_static` is a dense O(n³) LU.** Anything past a few hundred DOFs must use
   `analyses::solve_multi_case` (sparse, RCM-ordered, single LDLT). The free `sparse::ldlt_factor`
   has **no** job-admission page gate, so the sparse path has no equivalent size cap. Test 3c
   therefore routes through `solve_multi_case`; tests 3a and 3b stay dense (≤ 243 DOFs).
3. **`buckling` regularises `−Kg`** with `eps = 1e-6 · max|diag Kg|` (`🧮️analyses/🦀️.rs:2915-2919`)
   because bending elements leave their own axial DOF unstressed. The Python reference avoids the
   regularisation entirely by solving `K⁻¹(−Kg)` and inverting the eigenvalues; the resulting
   difference is ~1e-6 relative, four orders below the 2 % bound.
4. **`PlateDkt` has no `geometric_stiffness`** — plate/shell buckling is still structurally
   impossible (gap 4 of the exploration report, unchanged by this work).

## 5. Follow-ups this work makes visible

* `Quad4` has no incompatible/enhanced-assumed-strain modes; the MacNeal-Harder test now records
  exactly how much that costs (0.093 rectangular, 0.024 parallelogram, 0.027 trapezoidal against
  a theory value of 1.0). Adding EAS modes would be the single largest accuracy win in the 2D
  continuum family.
* `Hex8` likewise locks; the 12×2×5 mesh needs 702 DOFs to reach 95 % of beam theory.
* `Quad8`'s serendipity mapping was confirmed identical to the bilinear one when mid-side nodes sit
  at edge midpoints (that is why scikit-fem's `MeshQuad + ElementQuadS2`, a subparametric pairing,
  matches our isoparametric `Quad8` to 1e-11) — worth knowing before anyone adds curved-edge Quad8
  meshes, where the two would diverge.
* The old Cook's-membrane test applied `1/(n+1)` per tip node, which is not the consistent edge
  load; any earlier comparison of that test against published Cook values would have been invalid.
