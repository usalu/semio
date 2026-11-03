# 📐 EN 1995 and EN 1996 Target Specification

Scope: norm-complete target for EN 1995-1-1, -1-2, -2 (timber) and EN 1996-1-1, -1-2, -2, -3 (masonry) with the German national annexes (NA-DE). Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`, goal `norm`. Artifacts: `🪵️en1995`, `🪨️en1996` under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/`. Specification only; no source was changed.

> Reliability note: formulas and numbers were written from memory of the standards, not from the normative texts. Every `[V]` item, and any entry containing `…` or a trailing `?`, is unverified and must be pinned against the standard and the Python oracle before implementation. Worked-example arithmetic (section 13) was checked numerically; the input constants (tables) are the `[V]`-dependent part.

## 0. Conventions

- Units are SI in schema and kernel (N, m, Pa, kg, s); the UI shows N/mm², kN, mm per user unit preference. Formulas below are written in the standard's customary units (N, mm, N/mm²) and say so when they differ.
- IDs: `T-nn` = timber verification/derivation (EN 1995), `M-nn` = masonry (EN 1996). The id is the stable key of the `Verification` registry (section 4 / 8) and of the test fixtures.
- NDP column: `EN` = recommended value, `DE` = value of the German annex (DIN EN 1995-1-1/NA, DIN EN 1996-1-1/NA, DIN EN 1996-3/NA). Where equal, one value is given.
- Verification flags: items marked **[V]** must be transcribed/pinned from the normative text (tables, NA values) when implemented and cross-checked by the Python oracle; this document states the structure and the value I am confident of, and marks where a table or NA number is not reproduced from memory. A `[V]` is a task, not a hole: the schema holds the table as data (`StrengthTable`, `KTable`), never as code constants.
- Design situations: persistent/transient (ULS), accidental, seismic (no timber/masonry seismic specifics beyond EN 1998-1 references, see section 2.9/3.9), fire (EN 1990 6.4.3.3, `ψ1,1`), SLS characteristic/frequent/quasi-permanent.
- DE partial factors on actions (EN 1990/NA): `γG = 1.35`, `γQ = 1.50`, `γG,fav = 1.0`; combination `Ed = Σ γG Gk + γQ1 Qk1 + Σ γQi ψ0,i Qki`.
- Every verification returns a uniform `CheckResult`: `{id, clause, name{en,de}, inputs[], steps[] (formula string + substituted values), demand, capacity, unit, utilisation, status ∈ {pass, fail, not_applicable, input_missing, out_of_scope}, remedies[], governing_combination}`. Utilisation is `η = Ed/Rd` (or `σ/f`); pass iff `η ≤ 1.0` (computed in f64; tolerance for pass is `η ≤ 1 + 1e-9`).
- Each derivation is pure (no I/O), deterministic, cancellable at step granularity, and reports progress for sweeps (section 7.7).

---

# PART A. EN 1995 (Timber)

## 1. Materials, actions, factors (EN 1995-1-1 chapter 2 and 3)

### T-01 Strength class lookup and characteristic properties (EN 338:2016, EN 14080:2013/2023, EN 14374 LVL, EN 12369/EN 13986 panels)

- Inputs: product (`solid`, `glulam_homogeneous`, `glulam_combined`, `lvl`, `plywood`, `osb`, `particleboard`, `fibreboard`, `clt`), class string (`C14…C50`, `D18…D70`, `GL20h/c … GL32h/c`, `LVL…`), moisture state.
- Outputs (table lookup, in schema as `TimberStrengthClass`): `fm,k, ft,0,k, ft,90,k, fc,0,k, fc,90,k, fv,k, fr,k (rolling shear), E0,mean, E0,05, E90,mean, Gmean, Gr,mean, ρk, ρmean`, bending `fm,y,k, fm,z,k` for LVL (edge/flat).
- Reference values to be used as fixtures (EN 338:2016): C24 `fm 24, ft0 14.5, ft90 0.4, fc0 21, fc90 2.5, fv 4.0, E0,mean 11 000, E0,05 7 400, E90,mean 370, Gmean 690, ρk 350, ρmean 420` (N/mm², kg/m³). C16 `16, 10, 0.4, 17, 2.2, 3.2, 8 000, 5 400, 270, 500, 310, 370`. C30 `30, 18, 0.4, 23, 2.7, 4.0, 12 000, 8 000, 400, 750, 380, 460`. GL24h (EN 14080:2013) `24, 16.5, 0.4, 24, 2.7, 3.5, 11 500, 9 600, 300, 650, 385`. GL28h `28, 19.5, 0.45, 26.5, 3.0, 3.5, 12 600, 10 500, 300, 650, 425`. GL32h `32, 22.5, 0.5, 29, 3.3, 3.5, 14 200, 11 800, 300, 650, 440` **[V]** (EN 14080:2013 vs 2023 differ; the schema carries an `edition` field and both tables).
- Rule: the strength class is a pure key; every derived characteristic stored with its source (table + edition) for traceability.
- Edge cases: unknown class ⇒ `input_missing`; hardwood `D` classes use different `kh` and `βn`; combined glulam `c` classes have different outer/inner lamella strengths (`fm,k` for outer, `ft,0,k` etc. from the tabulated combined values); LVL `fm,k` has size exponent `s` (see T-05).

### T-02 Modification factor `kmod` (EN 1995-1-1 Table 3.1, 3.1.3)

- Inputs: product, service class SC1/SC2/SC3, load-duration class (`permanent >10 a, long-term 6 m–10 a, medium 1 w–6 m, short <1 w, instantaneous`).
- Solid, glulam, LVL (EN 1995-1-1 Table 3.1) SC1/SC2: `0.60 / 0.70 / 0.80 / 0.90 / 1.10`; SC3: `0.50 / 0.55 / 0.65 / 0.70 / 0.90`. Plywood EN 636-1 SC1 as above; OSB/3, OSB/4: SC1 `0.40 / 0.50 / 0.70 / 0.90 / 1.10`, SC2 `0.30 / 0.40 / 0.55 / 0.70 / 0.90`; particleboard P5, P7 and fibreboard values per Table 3.1 **[V]** (data table `KmodTable`).
- Combination: for a combination of actions in different duration classes, `kmod` of the shortest class is used (3.1.3 (2)); for a connection with different member `kmod`: `kmod = √(kmod,1 · kmod,2)` (2.3.2.2 (2)).
- NA-DE: values as EN; load-duration assignment of actions per NA Table NA.1 (snow `H ≤ 1000 m a.s.l.`: short; wind: short; imposed category A/B: medium; C–E: medium (storage: long); snow `H > 1000 m`: medium) **[V]**.
- Outputs: `kmod`, governing duration class per combination. Edge: SC3 + instantaneous with non-conforming product ⇒ `not_applicable`.

### T-03 Partial factor `γM` (EN 1995-1-1 Table 2.3)

- EN: solid 1.3; glulam 1.25; LVL, plywood, OSB 1.2; particleboard, hard fibreboard 1.3; medium fibreboard (MDF) 1.3; soft fibreboard 1.3; connections 1.3; punched metal plate fasteners 1.25; accidental 1.0.
- NA-DE: solid 1.3, glulam 1.25, LVL 1.2, plywood/OSB 1.2, particleboard 1.3, fibreboard 1.3, connections 1.3, nail plates 1.25, accidental 1.0; CLT per Z-9.1-559/ETA and the 2nd-generation EN 1995-1-1:2025 `γM = 1.25` (DE value of CLT **[V]**).
- Output: `γM` by product and situation. Fire: `γM,fi = 1.0` (EN 1995-1-2 2.3).

### T-04 Deformation factor `kdef` (EN 1995-1-1 Table 3.2, 2.3.2.2)

- Solid, glulam, LVL: SC1 0.60, SC2 0.80, SC3 2.00. Plywood EN 636-1 0.80/-, EN 636-2 0.80/1.00/-, EN 636-3 0.80/1.00/2.50. OSB/3,4: SC1 1.50, SC2 2.25; OSB/2 2.25 (SC1). Particleboard P4/P5 2.25/3.00; P6/P7 1.50/2.25; hardboard HB.HLA2 2.25/3.00; MDF.HLS 3.00/4.00 **[V]** (data table `KdefTable`).
- Final deformation: `ufin = uinst (1 + kdef)` for each action; combination with quasi-permanent: `ufin = uinst,G (1 + kdef) + uinst,Q1 (1 + ψ2,1 kdef) + Σ uinst,Qi (ψ0,i + ψ2,i kdef)`.
- For members with different `kdef` and creep (composite): `ufin = uinst (1 + kdef,eq)` using the modular-ratio method (2.3.2.2 (3)): `Efin = Emean/(1+ψ2 kdef)`.
- Connections: `kser` slip, `ufin = uinst (1 + 2 kdef)` for components of different creep behaviour (2.3.2.2 (3)) **[V]**.

### T-05 Size effect `kh` and volume effects (EN 1995-1-1 3.2, 3.3, 3.4, 6.1.6)

- Solid timber: `kh = min((150/h)^0.2, 1.3)` for bending depth `h < 150 mm`; for tension parallel `h` = larger cross-section dimension (width); `kh` applies to `fm,k` and `ft,0,k`.
- Glulam: `kh = min((600/h)^0.1, 1.1)` for `h < 600 mm` (applies to `fm,k` and `ft,0,k`, `h` = depth for bending, width for tension).
- LVL: `kh = min((300/h)^s, 1.2)`, `s` = size effect exponent from EN 14374 (default 0.12); length effect for tension `kl = min((3000/l)^(s/2), 1.1)`.
- Beam volume factor `kvol` for curved/pitched: `(V0/V)^0.2`, `V0 = 0.01 m³`, with `V ≤ 2/3 Vb` (T-33).
- Effective rolling shear/panels: `kh` not applicable to panels.
- Output: `kh` (and for Annex of CLT `kh,CLT`). Edge: `h ≥ limit` ⇒ `kh = 1.0`.

### T-06 Design strength `Xd` (EN 1995-1-1 2.4.1)

- `Xd = kmod · Xk / γM` (strengths); `Ed = Σ …` for actions; stiffness `Ed = Emean` (SLS), `E0,05` (second-order/stability, 2.4.1 (2)), `Ed(ULS) = Emean/γM` for second-order analysis of redistribution **[V]**.
- Connections: `Rd = kmod · Rk / γM`.

## 2. EN 1995-1-1 verification catalogue

Geometry convention: member axis x, strong axis y (depth h, `Wy = b h²/6`), weak axis z (width b). Compression/tension parallel to grain are `σc,0,d`/`σt,0,d`; the combination of axial and bending uses relative stresses.

### 2.1 Chapter 6.1 Cross-section checks

#### T-10 Tension parallel to the grain (6.1.2)

- Inputs: `Nt,d [N]`, area `A [mm²]` (net, with holes/notches), `ft,0,k`, `kmod`, `γM`, `kh`.
- Formula: `σt,0,d = Nt,d / A ≤ ft,0,d`, `ft,0,d = kh kmod ft,0,k / γM`; net area from T-18 when holes/notches present.
- Output: `η = σt,0,d / ft,0,d`.
- Edge: eccentricity at bolt group handled by T-11 combined; `A` is net at governing section (reduction by fastener holes ⇒ T-71).

#### T-11 Tension perpendicular to the grain (6.1.3 and 6.5, 6.4.3)

- Formula: `σt,90,d ≤ ft,90,d`, `ft,90,d = kmod ft,90,k / γM`. For curved/pitched see T-36, for notches T-45; for connections with loads at an angle T-66 (splitting). Solid timber cases: only where the stress is local; for glulam the volume factor `kvol` ⇒ T-36.

#### T-12 Compression parallel to the grain (6.1.4)

- `σc,0,d = Nc,d / A ≤ fc,0,d`, `fc,0,d = kmod fc,0,k / γM`; stability (6.3) required if `λrel > 0.3` (T-20/T-21).

#### T-13 Compression perpendicular to the grain and bearing (6.1.5)

- Inputs: `Fc,90,d`, contact length `l` (support), effective contact length `lef = l + 2·min(30 mm, a, l, l1/2)` where `a` = overhang distance, `l1` = distance to next load; width `b`; `fc,90,k`; shape `kc,90`.
- Formula: `σc,90,d = Fc,90,d / Aef ≤ kc,90 fc,90,d`; `Aef = b · lef`. `fc,90,d = kmod fc,90,k/γM`.
- `kc,90` EN (6.1.5 (4)): 1.0 default; for continuous support (6.4): solid conifer 1.25 (`h ≤ 2.5 l`... `l1 ≥ 2h`), glulam conifer 1.5 (`l ≤ 400 mm`), for discrete support 1.5 (solid) / 1.75 (glulam, `l ≤ 400`); LVL 1.0. NA-DE: same shape and, additionally, `kc,90` capped by the NA limits `l ≤ 400 mm` for glulam and `kc,90 = 1.0` for hardwood **[V]**.
- Edge: loads at the member end (`a < …`) use `lef = l + min(30 mm, a)` (single-sided); `lef` never greater than the physical support length plus spreads.

#### T-14 Bending (6.1.6)

- Inputs: `My,d, Mz,d [Nmm]`, `Wy, Wz [mm³]`, `fm,k`, `kmod`, `γM`, `kh`, `km`.
- Formulas: `σm,y,d = My,d/Wy`, `σm,z,d = Mz,d/Wz`; `σm,y,d/fm,y,d + km σm,z,d/fm,z,d ≤ 1` and `km σm,y,d/fm,y,d + σm,z,d/fm,z,d ≤ 1` (6.11, 6.12). `km = 0.7` rectangular solid, glulam, LVL; `km = 1.0` other cross-sections and other wood-based products.
- Output: `η = max` of the two. `fm,d = kh kmod fm,k/γM`. NA-DE: same `km`.
- Edge: with LTB (`kcrit`), use T-22.

#### T-15 Shear (6.1.7) with cracking factor `kcr`

- Formulas: `τd = 1.5 Vd /(kcr b h)` (rectangular); general `τd = Vd S/(I bef)`; `τd ≤ fv,d`, `fv,d = kmod fv,k/γM`; `bef = kcr b`.
- `kcr`: EN rec. 0.67 (solid and glulam), 1.0 for other wood-based. NA-DE (NCI to 6.1.7): `kcr = 2.5/fv,k` with `fv,k` in N/mm² for solid and glulam (C24: 0.625; GL28h: 0.714), `kcr = 1.0` for LVL, `kcr = 1.0` for panels **[V]**.
- Support shear reduction: forces within `h` of the support edge may be neglected for the shear check where the load acts on top and the support is on the bottom of the member (6.1.7 (3)); `V` taken at distance `h` from the face of the support; `fv` notches ⇒ T-45.
- Edge: beams with `kcr` < 1 and `h/b` large; hardwood; `Vd` of loads near supports (`a < 2h`): EN 1995-1-1 6.1.7 (3) reduction `β = a/(2h)` of the point load within `2h` of the support **[V]**.

#### T-16 Torsion (6.1.8)

- `τtor,d ≤ ksh fv,d`, `ksh = min(1 + 0.15 h/b, 2.0)` for rectangular, `ksh = 1.2` solid circular, `ksh = 1.0` other sections.
- Torsion shear `τtor,d = Mt,d / Wt`, with `Wt = α b² h` for rectangle (`α` from `h/b`: 1: 0.208; 2: 0.246; 3: 0.267; 5: 0.291; 10: 0.312; ∞: 0.333) or `Wt = π d³/16` circle.

#### T-17 Axial and bending combined (6.2.3, 6.2.4)

- Tension + bending: `σt,0,d/ft,0,d + σm,y,d/fm,y,d + km σm,z,d/fm,z,d ≤ 1` and symmetric (6.17, 6.18).
- Compression + bending (no buckling): `(σc,0,d/fc,0,d)² + σm,y,d/fm,y,d + km σm,z,d/fm,z,d ≤ 1` and symmetric (6.19, 6.20).
- Edge: `σc` squared term only for compression; hardwood and LVL with the same equations.

#### T-18 Net sections, notches and holes (5.2, 6.5)

- Net area `Anet = A − Σ(d·b)` of holes in a section at the governing cross-section (bolts: no projection reduction when the hole is filled by bolt and compression, 5.2 (2)); tension net section: `Anet` of all fasteners within `0.5 d`... **[V]** the staggering rule `a < 4d`.
- Holes in beams: DIN EN 1995-1-1/NA NCI: circular holes `d ≤ 0.15 h` in glulam only at low shear; reinforcement required otherwise; geometric limits as data in `HoleRules` **[V]**.

### 2.2 Chapter 6.3 Stability

#### T-20 Column buckling `kc` (6.3.2)

- Inputs: `lef,y, lef,z [mm]`, `iy = √(Iy/A)`, `fc,0,k`, `E0,05`, `βc`.
- Formulas:
  - `λy = lef,y/iy`, `λrel,y = (λy/π) √(fc,0,k/E0,05)`, same for z.
  - If `λrel ≤ 0.3`: `kc = 1`. Else `ky = 0.5 (1 + βc (λrel,y − 0.3) + λrel,y²)`, `kc,y = 1/(ky + √(ky² − λrel,y²))`.
  - `βc = 0.2` solid, `0.1` glulam and LVL (EN 1995-1-1 6.29).
- Checks (6.23, 6.24): `σc,0,d/(kc,y fc,0,d) + σm,y,d/fm,y,d + km σm,z,d/fm,z,d ≤ 1` and `σc,0,d/(kc,z fc,0,d) + km σm,y,d/fm,y,d + σm,z,d/fm,z,d ≤ 1`.
- NA-DE: same; `βc` and the limit are as EN; effective lengths from system analysis (second-order alternative via T-24).
- Edge: `λ` ≤ 150 recommended maximum (NA-DE: ≤ 150 for general, 170 for secondary) **[V]**; `lef` from end conditions: pinned-pinned 1.0 L, fixed-pinned 0.7, fixed-fixed 0.5, cantilever 2.0; sway frames: system length.

#### T-21 Combined compression with slender bending (6.3.2, general)

- Same as T-20 with `kc` for each axis and `kmodified`; the output provides both utilisations and the governing axis.

#### T-22 Lateral-torsional buckling `kcrit` (6.3.3)

- Formulas: `σm,crit = My,crit/Wy = π √(E0,05 Iz Gtor Itor)/(lef Wy)` ⇒ rectangular softwood: `σm,crit = 0.78 b² E0,05 /(h lef)` (6.32). `λrel,m = √(fm,k/σm,crit)`.
  - `kcrit = 1` if `λrel,m ≤ 0.75`; `1.56 − 0.75 λrel,m` if `0.75 < λrel,m ≤ 1.4`; `1/λrel,m²` if `λrel,m > 1.4` (6.34).
- Check: `σm,d ≤ kcrit fm,d` (6.33); with axial compression (6.35): `(σm,d/(kcrit fm,y,d))² + σc,d/(kc,z fc,0,d) ≤ 1`.
- `lef` (Table 6.1): simply supported, constant moment `lef = 1.0 L` (rel. to load at compression edge: `lef/ℓ = 0.9`; at tension edge `1.0 + 2h`); uniformly distributed `0.9 L`; point load at midspan `0.8 L`; cantilever uniform `0.5 L`; point at end `0.8 L`. Load position effect: `lef` increased by `2h` if load at tension edge and reduced by `0.5 h` if at compression edge.
- Continuous lateral restraints (decking) ⇒ `kcrit = 1` if lateral displacement and torsional rotation at the compression edge are prevented (6.3.3 (5)). For members with one section dimension ≤ `0.4 h` etc. special cases **[V]**.
- NA-DE: `σm,crit` and `kcrit` equal; the NA permits a second-order analysis alternative (T-24). Hardwood, LVL use the same.
- Edge: `λrel,m` undefined for `σm,crit = 0` ⇒ `not_applicable`.

#### T-23 Biaxial stability interaction (6.3.2 (3), 6.3.3 (6))

- Aggregates T-20 and T-22 with `Mz` and `N`; additional check for the compression flange of bending members.

#### T-24 Second-order (6.3.2, 5.4.4, Annex) imperfections analysis

- Bow imperfection `ec = βc · lef/200` (solid, `βc = 0.0025` ⇒ `lef/400` for glulam, `lef/500` LVL)`L/400` (solid), `L/500` (glulam/LVL) per 5.4.4 (1); `e0 = lef/300 ≤ …` **[V]**.
- Inputs: nodal loads, imperfections; Output: `Ed = M_II`, rechecking T-14/T-17 without `kc`. Used when frame analysis (`system`) replaces effective-length formulas.

### 2.3 Chapter 6.4 Members with varying cross-section or curved shape

#### T-30 Single tapered beam (6.4.2)

- Taper on one face, angle `αtap`; the stress at the tapered edge `σm,α,d = 6 Md / (b h²)` and at the intersection: `σm,α,d ≤ km,α fm,d` (tension side) with `km,α = 1/√(1 + (fm,d/(0.75 fv,d) tan α)² + (fm,d/ft,90,d tan² α)²)` (taper on tension side, 6.38) and `km,α = 1/√(1 + (fm,d/(1.5 fv,d) tan α)² + (fm,d/fc,90,d tan² α)²)` (compression side, 6.39).
- Edge: `αtap ≤ 10°`; the section where the maximum stress lies is determined by the stress diagram; both cut surfaces checked.

#### T-31 Double tapered, curved and pitched cambered beams (6.4.3, Figure 6.9)

- Geometry: apex height `hap`, radius `rin`, inner radius, lamella thickness `t`, `r = rin + 0.5 hap`, pitch angle `αap`.
- Bending in apex zone: `σm,d = kl · 6 Map,d /(b hap²)` (6.42), `kl = k1 + k2 (hap/r) + k3 (hap/r)² + k4 (hap/r)³`, `k1 = 1 + 1.4 tan αap + 5.4 tan² αap`, `k2 = 0.35 − 8 tan αap`, `k3 = 0.6 + 8.3 tan αap − 7.8 tan² αap`, `k4 = 6 tan² αap`; pitched cambered with `kl` from the same; for curved beams (no pitch) `kl = 1 + 0.35 hap/r + 0.6 (hap/r)²`.
- Check: `σm,d ≤ kr fm,d` (6.41): `kr = 1` for `rin/t ≥ 240`; `kr = 0.76 + 0.001 rin/t` for `rin/t < 240` (6.49).
- Edge: `hap/r` up to 0.5; curved beam `hap` meaning the apex depth; NA-DE keeps EN with extra requirement for min lamella thickness.

#### T-32 Tension perpendicular at apex (6.4.3)

- `σt,90,d = kp · 6 Map,d /(b hap²)` with `kp = k5 + k6 (hap/r) + k7 (hap/r)²`, `k5 = 0.2 tan αap`, `k6 = 0.25 − 1.5 tan αap + 2.6 tan² αap`, `k7 = 2.1 tan αap − 4 tan² αap`. Pitched cambered: with distributed load add `pd` term: `σt,90,d = kp 6 Map,d/(b hap²) − 0.6 pd/b` (6.54).
- Check (6.50): `σt,90,d ≤ kdis (V0/V)^0.2 ft,90,d`, `kdis = 1.4` for double tapered/curved, `1.7` for pitched cambered; `V` = stressed volume of apex zone `V = Vb (1 − hap²... )` with `V ≤ 2/3 Vb`; `V0 = 0.01 m³`.
- Combined shear/tension: `τd/fv,d + σt,90,d/(kdis (V0/V)^0.2 ft,90,d) ≤ 1` (6.53) for the combined check in the presence of shear.
- Output: both utilisations, apex zone volume.

#### T-33 Reinforced apex zone (NA-DE NCI, DIN EN 1995-1-1/NA and DIN EN 14080)

- Provide `reinforcement` option: fully-threaded screws or glued-in rods `Ft,90,d` carried by the reinforcement per apex; resistance of reinforcement in withdrawal of the strong part (T-68). Check `Ft,90,d,total ≤ Σ Rax,d`. Parameters in data **[V]**.

### 2.4 Chapter 6.5 Notched members

#### T-45 Notch at support (6.5.2)

- Inputs: `h`, `hef` (net), `x` (distance of the support reaction centre to the notch corner), `α = hef/h`, taper `i` (gradient tan of slope), `b`, `Vd`, `kn`.
- Formulas: `τd = 1.5 Vd /(b hef) ≤ kv fv,d` (6.60); `kv = 1` if notch on the tension... (opposite side of support); on the support side: `kv = min(1, kn (1 + 1.1 i^1.5/√h) / (√h (√(α(1−α)) + 0.8 (x/h) √(1/α − α²))))` (6.62), `kn = 4.5` LVL, `5` solid, `6.5` glulam; `h` in mm.
- NA-DE: restricts notch depth on the support side to `α ≥ 0.5` (hef ≥ 0.5 h), requires reinforcement for deeper notches; notches on the span side `kv = 1`, and `x ≤ 0.4 h` **[V]** (`NCI zu 6.5.2`: Ausklinkungen mit `x/h`, Verstärkung per 6.5.3).
- Edge: shear force `Vd` taken at the notch face; inclined cut `i`; tension-side notches require rounding radius `r` and are covered by T-46.

#### T-46 Reinforced notches (NCI)

- Tensile force `Ft,90,d = 1.3 Vd [3(1−α)² − 2(1−α)³]` (Informative Annex NA, based on DIN 1052:2008) resisted by screws/rods placed within `x` (rod length `lad ≥ …`); check against T-68 withdrawal. **[V]**.

### 2.5 Chapter 6.6 System strength

#### T-50 System strength factor `ksys` (6.6)

- For ≥ 4 equally spaced similar parallel members loaded laterally and connected by continuous load-distributing system: `fd` multiplied by `ksys = 1.1`; for trusses at spacing ≤ 1.2 m: `ksys = 1.1`; for glued laminated/solid trusses with `n` members, EN 1995-1-1 6.6 (2) **[V]**.
- Applies to bending and shear checks only for the stated geometry; the schema marks a "system" group (`MemberGroup`) with `spacing ≤ 1.2 m` and `distributing_element`.

### 2.6 Chapter 7 Serviceability

#### T-60 Instantaneous deflection `winst` (7.2)

- Components: bending `wm = ∫ M m/(E I) dx` and shear `wv = ∫ V v κ/(G A) dx` (shear factor `κ = 1.2` rectangular); use `E0,mean`, `Gmean`. Simply supported UDL: `w = 5 q L⁴/(384 E I)` (+ `1.2 q L²/(8 G A)` for shear).
- Output: `winst,G`, `winst,Q1`, `winst,Qi`.

#### T-61 Final deflection `wfin` and net final `wnet,fin` (7.2)

- `wfin = winst (1 + kdef)`; `wnet,fin = wfin − wc` (precamber `wc`). Combination rules T-04.
- Limits: EN recommended `winst ≤ L/300…L/500` (beams), `wnet,fin ≤ L/250…L/350`, `wfin ≤ L/150…L/300` (cantilevers `L`→ `2L`). NA-DE: `winst,Q ≤ L/300`, `wnet,fin ≤ L/200` (general), `wfin ≤ L/200`..., for roof members with small usage `L/200` **[V]** (NDP Table 7.2 NA-DE: `winst: L/300 (L_k/150)`, `wnet,fin: L/250 for floors/L/200 for roof`).
- Edge: cantilever uses `2·L`; variable stiffness (tapered) uses numeric integration.

#### T-62 Slip contribution (7.2, 7.3, Annex B)

- Mechanically jointed beams: deflection from `EIef` (T-80); connections: `ukser`; `kser` per T-67 table; effective stiffness `Kser`, `Ku = 2/3 Kser`.

#### T-63 Floor vibration frequency `f1` (7.3.3, DIN EN 1995-1-1/NA NCI)

- Formula (7.5): `f1 = π/(2 ℓ²) √((EI)ℓ/m)`, `ℓ` span [m], `(EI)ℓ` equivalent plate bending stiffness about an axis perpendicular to beam direction [Nm²/m], `m` mass per unit area [kg/m²] (permanent + 10 % of quasi-permanent imposed in the NA-DE; EN: only `G + ψ2 Q` mass).
- Criterion EN 7.3.3: `f1 > 8 Hz` (rec.). NA-DE (NCI to 7.3.3): residential floors `f1 ≥ 6 Hz`; `6 Hz ≤ f1 < 8 Hz` ⇒ additional stiffness criterion; `f1 < 6 Hz` ⇒ special investigation (Wohnungstrenndecken, Hamm-Richter) **[V]**.
- Output: `f1`, `m`, `(EI)ℓ`, pass/fail, mode shape (for T-65 viz).

#### T-64 Unit point load deflection `w` (7.3.3 (2), NA-DE)

- Criterion: `w ≤ a [mm/kN]`, where `w` is the instantaneous vertical deflection under 1 kN point load at any point (parallel plate action allowed: `w` computed via load distribution `kdist = max(0.38?, …)` in NA; effective width `bef`). EN rec. `a = 1.5` (range 0.5–4). NA-DE: `a = 0.5 mm/kN`? [V] use data `NdpVibration.a`.
- Edge: with timber-concrete composite floors; point load distribution using `ℓ/ (EI_b/EI_ℓ)^0.25` effective width.

#### T-65 Unit impulse velocity response `v` (7.3.3 (4), (5))

- `v = 4 (0.4 + 0.6 n40)/(m b l + 200) [m/(Ns²)]`, `n40 = ((40/f1)² − 1) (b/l)(EI)ℓ/(EI)b)^{0.25}` (7.6 / 7.7 with `l` span and `b` width); criterion `v ≤ b^(f1 ζ − 1)` with EN rec. `b = 100…150`, modal damping `ζ = 0.01` (EN rec.). NA-DE gives `b` as function of `a` by Figure 7.2; DE uses `f1`+`a` criteria and may skip `v`.
- Output: `v`, `n40`, limit, `η`.
- Edge: `f1 > 40 Hz` ⇒ `n40 ≤ 0` ⇒ unconditional pass (v ignored).

#### T-66 Walking/rhythmic excitation for footbridges (EN 1995-2 §7.4, see T-120).

### 2.7 Chapter 8 Connections

General inputs: fastener type; diameter `d` [mm]; `fu,k` [N/mm²]; strength class/density `ρk` of each member (and for panels `ρk` of panel); thicknesses `t1, t2`; angle between force and grain `α`; shear planes; rope effect flag.

#### T-70 Embedment strength `fh,k` and yield moment `My,Rk` (8.5.1, 8.3)

- Nails/staples/bolts/dowels (no pre-drill, nails `d ≤ 8 mm`): nails without pre-drilling `fh,k = 0.082 ρk d^−0.3`; with pre-drilling `0.082 (1 − 0.01 d) ρk` [N/mm²]; bolts and dowels `fh,0,k = 0.082 (1 − 0.01 d) ρk`, `fh,α,k = fh,0,k/(k90 sin²α + cos²α)`, `k90 = 1.35 + 0.015 d` (softwood), `1.30 + 0.015 d` (LVL), `0.90 + 0.015 d` (hardwood). Panels: plywood `fh,k = 0.11 ρk d^−0.3`, OSB/particleboard `fh,k = 50 d^−0.6 t^0.2`.
- Yield moments: round nails `My,Rk = 0.3 fu,k d^2.6` [N mm]; square/grooved nails `0.45 fu,k d^2.6`; bolts `0.3 fu,k d^2.6`; dowels `0.3 fu,k d^2.6`; staples `240 d^2.6`; screws: `d = deff = 1.1 dc?` per 8.7 (`deff = 1.1 × thread root diameter`, `My,Rk = 0.15 fu,k deff^2.6`) **[V]** for screws.
- NA-DE: for nails `d ≤ 6 mm` (NA) additional min thickness; nail `fh,k` formulas same.

#### T-71 Johansen European Yield Model: timber-to-timber, single shear (8.2.2)

- Inputs: `fh,1,k, fh,2,k, t1, t2, d, My,Rk, β = fh,2,k/fh,1,k, Fax,Rk` (rope effect).
- `Fv,Rk = min(`
  - (a) `fh,1,k t1 d`
  - (b) `fh,2,k t2 d`
  - (c) `fh,1,k t1 d/(1+β) [√(β + 2β²(1 + t2/t1 + (t2/t1)²) + β³ (t2/t1)²) − β (1 + t2/t1)] + Fax,Rk/4`
  - (d) `1.05 fh,1,k t1 d/(2+β) [√(2β(1+β) + 4β(2+β) My,Rk/(fh,1,k d t1²)) − β] + Fax,Rk/4`
  - (e) `1.05 fh,1,k t2 d/(1+2β) [√(2β²(1+β) + 4β(1+2β) My,Rk/(fh,1,k d t2²)) − β] + Fax,Rk/4`
  - (f) `1.15 √(2β/(1+β)) √(2 My,Rk fh,1,k d) + Fax,Rk/4 `)`.
- Rope effect limits `Fax,Rk/4`: round smooth nails 15 %, square/grooved nails 25 %, other nails 50 %, screws 100 %, bolts 25 %, dowels 0 % of the Johansen part.

#### T-72 Timber-to-timber, double shear (8.2.2, 8.2.3)

- Modes (g), (h), (j), (k) for the middle member `t2` and side members `t1`: (g) `fh,1,k t1 d`; (h) `0.5 fh,2,k t2 d`; (j) `1.05 fh,1,k t1 d/(2+β) [√(2β(1+β) + 4β(2+β) My,Rk/(fh,1,k d t1²)) − β] + Fax,Rk/4`; (k) `1.15 √(2β/(1+β)) √(2 My,Rk fh,1,k d) + Fax,Rk/4`. Resistance per shear plane = `min` for the double shear connection divided by 2 where the table is per shear plane; the tool returns `Rk` per fastener (both planes).

#### T-73 Steel-to-timber, single shear (8.2.3)

- Thin steel plate (`t ≤ 0.5 d`): (a) `0.4 fh,k t1 d`, (b) `1.15 √(2 My,Rk fh,k d) + Fax,Rk/4`.
- Thick steel plate (`t ≥ d`; tolerance for outer plate in double shear): (c) `fh,k t1 d`, (d) `fh,k t1 d [√(2 + 4 My,Rk/(fh,k d t1²)) − 1] + Fax,Rk/4`, (e) `2.3 √(My,Rk fh,k d) + Fax,Rk/4`.
- Between thin and thick (`0.5 d < t < d`): linear interpolation between thin and thick results.
- Double shear steel central plate: (f) `fh,1,k t1 d`, (g) `fh,1,k t1 d [√(2 + 4 My,Rk/(fh,1,k d t1²)) − 1] + Fax,Rk/4`, (h) `2.3 √(My,Rk fh,1,k d) + Fax,Rk/4` (thick), (j) `0.5 fh,2,k t2 d` thin, (k) `1.15 √(2 My,Rk fh,k d)`.
- Steel plate edge distances and net-section capacity per EN 1993-1-8 (cross-reference to the `en1993` artifact; clause T-79).

#### T-74 Spacing, edge and end distances (8.3, 8.5, 8.7)

- Nails (8.3.1.2, Table 8.2): `a1 = (5 + 5|cos α|) d` (non-predrilled `ρk ≤ 420`, `d ≤ 5 mm`; `a1 = (5 + 7|cos α|) d` for `d ≥ 5 mm`); `a2 = 5d`; `a3,t = (10 + 5 cos α) d`; `a3,c = 10 d`; `a4,t = (5 + 5 sin α) d (d<5 mm: (5+2 sin α))`; `a4,c = 5d`. Pre-drilled: `a1 = (3 + 2|cos α|) d`, `a2 = 3d`, `a3,t = (7 + 5 cos α) d`, `a3,c = 7d`, `a4,t = (3 + 2 sin α)d`, `a4,c = 3d`.
- Bolts (Table 8.4): `a1 = (4 + |cos α|) d`; `a2 = 4d`; `a3,t = max(7d, 80 mm)`; `a3,c = max(a3,t /?…)` (`a3,c = 4d` for α > 150°…`(1 + 6 sin α) d ≥ 4d`); `a4,t = max((2 + 2 sin α) d, 3d)`; `a4,c = 3d`.
- Dowels: `a1 = (3 + 2|cos α|) d`, `a2 = 3d`, `a3,t = max(7d, 80 mm)`, `a4,t = max((2 + 2 sin α) d, 3d)`.
- Screws (8.7.1, Table 8.6/8.7): lateral loaded as nails (for `d ≤ 6 mm`), axially loaded `a1 = 7d`, `a2 = 5d`, `a2,CG = 4d`, `a1,CG = 10d`, `a1,CG` = 12d, edge `3d` (minimum timber thickness `t ≥ 12 d`); NA-DE: Z-9.1-xxx for self-tapping screws with approvals (ETA) `a1`, `a2` per approval.
- Output: per fastener the spacing pass/fail; `A_geo` to drive the viewer (spacing zones).

#### T-75 Effective number of fasteners `nef` (8.1.2, 8.3.1.1, 8.5.1.1)

- Nails in a row (8.3.1.1): `nef = n^kef`, `kef` by spacing `a1` ratio: `a1 ≥ 14 d: 1.0; 10 d: 0.85; 7 d: 0.7; 4 d: 0.5` (pre-drilled `kef` 1.0, 0.85/0.7/0.5 for `a1` 14 d,10 d,7 d,4 d); linear interpolation.
- Bolts/dowels in a row parallel to grain (8.5.1.1): `nef = min(n, n^0.9 (a1/(13 d))^0.25)`; loaded perpendicular to grain: `nef = n`; at angle `α`: interpolation between 8.5.1.1 (parallel) and `n`.
- Screws loaded axially (8.7.2): `nef = n^0.9`.
- Output: `nef,total`; `Fv,ef,Rk = nef Fv,Rk`.

#### T-76 Axial resistance of nails, screws, bolts (8.3.2, 8.5.2, 8.7.2)

- Nails (8.3.2): smooth nails in withdrawal `Fax,Rk = min(fax,k d lef, fhead,k d²h + fax,k d tpen`... `fax,k = 20·10^−6 ρk²` N/mm² (nails with timber), `fhead,k = 70·10^−6 ρk²`; nails loaded in axial withdrawal only for threaded nails or SC1/SC2 `lef ≥ 8 d` (8.3.2 (5)); penetration `tpen ≥ 12 d` for smooth.
- Screws (8.7.2, EN 1995-1-1/A1:2008 and prEN 1995-1-1:2025): `Fax,α,Rk = nef · fax,k d lef kd / (1.2 cos²α + sin²α) [N]`, with `α` angle of screw axis to grain, `fax,k = 0.52 d^−0.5 lef^−0.1 ρk^0.8` (N/mm²), `kd = min(d/8, 1)`; `lef` = threaded part length in the member containing the point; head pull-through `Fax,head,Rk = nef fhead,k dh² (ρk/ρa)^0.8`, `fhead,k = 8.0 N/mm²` (with approval); steel tensile and torsional failure `Ft,Rk = ftens,k` (approval); Spacing from T-74.
- Bolts (8.5.2): `Fax,Rk` = washer plate size; `Fax,Rk = min(3 fc,90,k Ahead, ftens bolt)`; washer diameter ≥ 3 d, thickness ≥ 0.25 d.
- NA-DE: for self-tapping screws verification by Z-9.1-xxx approvals; `fax,k` per ETA, `ρk` ≤ 440 etc. Data in `ScrewApproval` entity **[V]**.

#### T-77 Combined lateral and axial loading (8.7.3, 8.3.3, 8.5.3)

- Nails/screws: `(Fax,Ed/Fax,Rd)² + (Fv,Ed/Fv,Rd)² ≤ 1` (screws 8.28: "d ≥ 6 mm" quadratic); for nails with head withdrawal: `Fax/Fax,Rd + Fv/Fv,Rd ≤ 1` (linear, 8.3.3 (1)).
- Bolts: linear unless the washers carry axial force.

#### T-78 Splitting and block shear (8.1.4, 8.2.2 (4), Annex A)

- Splitting: for force component perpendicular to grain `F90,Ed ≤ F90,Rd`, `F90,Rk = 14 b w √(he/(1 − he/h))` with `w = max((ws/100)^0.35, 1)` for punched metal plates `w` and `1` for others; `he` loaded edge distance; `b` width; `h` height; `h` and `he` in mm; (8.4) `Fv,Ed ≥ …`.
- Block shear / plug shear (Annex A, 8.2.2 and 8.2.3 (A.1): for steel-timber dowelled connections): `Fbs,Rk = max(1.5 Anet,t ft,0,k, 0.7 Anet,v fv,k)` with net tension area and net shear area (A.1); provide 2D geometry from the viewer.
- NA-DE: Annex A is normative informative; the splitting check for connections loaded at an angle `α`, with `ke` factors (NCI) **[V]**.

#### T-79 Steel details inside timber connections

- Steel plate verification (net section, bearing) delegated to `en1993` kernel via `steel_plate_check(plate, bolts)`; the timber artifact consumes the verdict.

#### T-80 Split rings and shear plates (8.9)

- Characteristic resistance (EN 912): `Fv,0,Rk = min(k1 k2 k3 ke (35 dc^1.5), k1 k3 dc he 31.5?)` ⇒ `Fv,0,Rk = min(k1 k2 k3 k4 (35 dc^1.5), k1 k3 he (31.5 dc))`; angle `Fv,α,Rk = Fv,0,Rk/(k90 sin²α + cos²α)`; `k1 = min(1, ta1/(3 he), ta2/(5 he))`, `k2 = min(a3,t/(1.5 dc), 1)`, `k3 = min(1.75, ρk/350)`, `k4 = 1.0` timber-timber, `1.1` steel-timber; `ke` effective factor. Spacing per Table 8.8 as data **[V]**.

#### T-81 Toothed-plate connectors (8.10)

- `Fv,0,Rk = (18 kt ke dc^1.5)` type C; for type A/B plate; with `kt = min(1.0, …)`; spacing `a1 = (1.2 + 0.8|cos α|) dc`; `ke`, `k1, k2, k3` per EN 912 + 8.10; as data tables **[V]**.

#### T-82 Punched metal plate fasteners (8.8)

- Anchorage strength `fa,α,β,k` from ETA, plate net section, `Rd` with `γM,MP = 1.25`; check anchorage area `Aef`, plate tension `ft,0,k`, shear, and rotation `k` (Annex NA). Data-driven; for DE requires national technical approvals (`ETA`). Output anchorage, steel net-section, interaction `…` **[V]**.

#### T-83 Connection stiffness `kser`, `Ku` (7.1, Table 7.1)

- `Kser = ρm^1.5 d/23` (bolts, dowels, screws with predrill, nails predrilled), `ρm^1.5 d^0.8/30` (nails without predrill), `ρm^1.5 dc/2` (split rings/shear plates per shear plane), `ρm^1.5 dc/4` toothed plates, `ρm^1.5 d/23`; steel-timber `kser` doubled (2.0×); `ρm` timber mean density (kg/m³), `d` mm ⇒ N/mm. `Ku = 2/3 Kser`. In second-order analyses use `Ku`.

### 2.8 Chapter 9 Structural components and assemblies

#### T-90 Mechanically jointed beams, γ-method (Annex B)

- Parts `i = 1…3`: `Ai`, `Ii`, `Ei`, spacing `si`, slip `Ki` (= `Kser` or `Ku`), span `ℓ`. `γi = 1/(1 + π² Ei Ai si/(Ki ℓ²))` for `i=1,3`, `γ2 = 1`.
- `a2 = (γ1 E1 A1 (h1 + h2) − γ3 E3 A3 (h2 + h3)) / (2 Σ γi Ei Ai)`, `a1 = (h1 + h2)/2 − a2`, `a3 = (h2 + h3)/2 + a2`.
- `(EI)ef = Σ (Ei Ii + γi Ei Ai ai²)`; stresses `σi = γi Ei ai M/(EI)ef` (axial), `σm,i = 0.5 Ei hi M/(EI)ef`, fastener load `Fi = γi Ei Ai ai si V/(EI)ef`, shear in web `τ2,max = 0.5 E2 (h2/2 + a2)² V/(EI)ef`.
- Checks: fibre stress `σc/t,i + σm,i ≤ f` (B.3), shear `τ ≤ fv`, fastener `Fi ≤ Rd`. Deflection from `EIef`. For ULS `K = Ku`, SLS `K = Kser`. Edge: 3-part symmetric, T-beams, I-beams, box-beams (Annex B.3 for webs).

#### T-91 Glued thin-webbed beams (9.1.1)

- Section with flange-web: plane section; flange `σf,c,max,d ≤ kc? fm,d`; web `σm,d` and compressive stresses; web shear `τ ≤ fv,d` (panel shear or `Av`); for webs from panel products `τ = Vd Sf/(Iy · bw)`; web buckling for `hw/bw > 70` per 9.1.1 (1) sets `hw/bw ≤ 70`... **[V]**; local compression flange buckling `bc,ef ≤ 25 hf`; effective flange widths for plate: `bef = bc,ef + bw` with `bc,ef ≤ 25 hf` (compression, plywood), `bt,ef`... shear lag `0.5 ℓ…`.
- Output: flange stresses with effective widths, web shear flows, glue-line shear `τ_gl = Vd Sf/(Iy · bf)`.

#### T-92 Glued thin-flanged beams (9.1.2)

- Effective flange width `bef = bc,ef/bt,ef + bw`, with `bef ≤ 0.1 ℓ` per side, `bc,ef` per Table 9.1: flange material (plywood: `0.1 ℓ`, panel buckling: `20 hf` surface grain parallel, `25 hf`), and shear lag: `bef` limited per `hf`. Check combined stress with `fm,d` flange panel and `fc,0,d`.

#### T-93 Mechanically jointed columns (9.1.3, Annex C)

- Annex C: buckling of spaced columns: `λef = √(λ² + η (n/2) λ1²)`, `η` by spacer fixation (glued 1, nailed 4, bolted 6, connectors 3 ...), `λ1 = ℓ1/i1`; `Fi,d` shear force from imperfection and fasteners.

#### T-94 Roof/floor diaphragms and wall diaphragms (9.2.3, 9.2.4)

- Wall diaphragm method A and B (9.2.4): 
  - Method A (9.2.4.2) racking strength `Ff,v,Rd = Σ Fi,v,Rd` (panels only: `Fi,v,Rd = Ff,Rd bi c_i/ b0 (…)`): with `bi = b/…`, `ci = 1` for `bi ≥ b0 = h/2`, `ci = bi/b0` otherwise; fastener spacing `s0 = 9700 / d^1.7` mm (nails), nail spacing ratio `ks = 0.5 + …``Ff,v,Rd = Ff,Rd bi ci /s`. Using the `Ff,Rd = kmod Ff,Rk/γM` per fastener, `Fi,v,Rd = Ff,Rd bi ci /s`, with `ci` above, `ki,q = …` uniformly distributed vertical load `k_i,q = 1 + …`. The shear wall stiffness `Fv,Rd = Σ Fi,v,Rd` limited by `ci`.
  - Method B (9.2.4.3): `Fi,v,Rd` for effective wall with `kd`, `ki,q`, `kn`, `ks`, `kopening` (openings) ⇒ unchanged. Deflection `wf = …`
  - Output: racking capacity per wall segment; with openings: 1/2 of the segment lengths.
- Overturning/hold-down: `Fi,t,Ed = Fi,v,Ed h/b − Vd(b/2)` per wall; anchors verified by T-76.
- Roof/floor diaphragms (9.2.3): `Fv,Rd,diaphragm = …` with diaphragm shear flow `qv,Ed = Fd/ℓ`, panel shear `qv,Rd`, chord force `Nchord = M/(b)`.
- Edge: mixed-sheeting walls `Σ` of the weaker; anchored top plate; openings treated as unframed sub-panels.

#### T-95 Trusses (9.2.2, 9.2.1)

- Axial analysis with nodal rotation stiffness for nailed/punched plate trusses (9.2.2): effective buckling length `lef`; stability bracing `Fd,brace = kf,3 ΣNd/60? (n / (kl))` with `ks = …` lateral bracing force `qd = kl Nd/(30 l)` `kl = min(1, √(15/l))` for `l` in m (9.2.5.2); `Fd = Nd/50` for solid, `Nd/80` glulam (9.2.5.2 (2)). Check connection and member chords T-20, T-71. Deflection per T-60.
- Truss geometry node/axis eccentricity bending included; truss as system for `ksys` (T-50). Punched plates T-82.

#### T-96 Bracing systems and restraints (9.2.5)

- Required lateral stiffness of bracing for beams and trusses: `Fbr,d = kf,3 Nd/…` and `kbr` deflection limit `ℓ/500` of bracing; compression chord imperfection; verify bracing system `ΣFd ≤ Rd`. Data cross-check with `T-22` restraint.

#### T-97 Cross-laminated timber CLT (prEN 1995-1-1:2025 Annex, EN 16351, DIBt Z-9.1-559 / ETA-xx, NA-DE NCI)

- Layers: thickness `ti`, orientation `0°/90°`, grade (`fm,k, E0,mean, fv,k, fr,k (rolling shear), Gr,mean`), layup `n ≥ 3`.
- Gamma method (Annex B generalised, 5 layers or `n` layers): effective bending stiffness `(EI)ef = Σ Ei Ii + Σ γi Ei Ai ai²` with the shear flexibility including rolling shear: `γi = 1/(1 + π² Ei Ai ti? /(Gr,i ... ℓ²))`: `γ = [1 + π² E A tr /(Gr b ℓ²)]^−1` (layers 0° only; cross layers `γ = 0`). Shear analogy (Kreuzinger) alternative as oracle.
- Bending: `σ ≤ fm,d` of governing outer layer 0° (`fm,CLT,k = fm,k,layer × ksys`); shear: gross `τ ≤ fv,CLT,d` (`fv,CLT,k = 5.0 N/mm²? EN 16351`), net and rolling shear in the crosslayers `τR ≤ fr,d`, `fr,k = 1.0 N/mm²` (rolling shear, EN 16351; NA-DE `fr,k` from Z-9.1-559: 1.1 N/mm² for adjacent layers with width-to-thickness ≥ 4) **[V]**.
- Compression perpendicular `fc,90,k = 2.5` with `kc,90 = 1.0`; tension perpendicular; deflection `w = wm + wv`, `wv = q ℓ²/(8 κ… GA)`; vibration as `T-63` with CLT `(EI)ℓ`, `(EI)b = …`.
- In-plane (wall) action: `Nd` buckling with `EA`, `kc` with reduction for rolling shear; `Fv,Ed` shear with gross net `σ`; `fire` T-110.
- NA-DE NCI: use CLT approvals; strength classes `C24/C16 lamellae`; CLT with edge glue `bonded` factor. Data in `CltProduct` tables.

### 2.9 Seismic and robustness (reference only)

- Timber under seismic loading: `EN 1998-1 ch. 8` ductility classes `DCL/DCM/DCH` and `q` ⇒ delegated to `en1998` artifact: `T-98 timber_q_factor(class, system)` returns `q` and capacity design rules; `kmod` for seismic = `instantaneous`.
- Robustness (EN 1995-1-1 2.4.3, A.1 accidental): `γM = 1.0`.

## 3. EN 1995-1-2 Fire (reduced cross-section, charring)

All fire checks use the accidental combination (`ψ1,1` frequent), `kmod,fi = 1.0`, `γM,fi = 1.0`, `fd,fi = kfi · fk/γM,fi`, strength modification `kfi`: solid 1.25, glulam 1.15, wood panels 1.15, LVL 1.1; connections with timber side members 1.15, steel side members 1.05 (EN 1995-1-2 2.3 (2) and Table 2.1).

#### T-100 Charring depth (3.4.2, 3.4.3, Table 3.1)

- Notional charring `dchar,n = βn t` (with corners rounded), 1D `dchar,0 = β0 t`. `t` [min]. Values (softwood glulam `ρk ≥ 290`) `β0 = 0.65`, `βn = 0.70`; solid softwood `0.65 / 0.80`; LVL `0.65 / 0.70`; hardwood solid `ρk = 290` `0.65/0.70`, `ρk ≥ 450` `0.50/0.55`; wood panelling/plywood `0.9/- `, wood-based panels `0.9` **[V]** (table in data).
- Density correction `k_ρ` and thickness correction `kh` for panels (`β0,ρ,t = β0 kρ kh`) with `kρ = √(450/ρk)`, `kh = √(20/hp)` (panels) **[V]**.
- Output: `dchar` over time; for sweep (0..120 min) emits time series for the viewer (T-110/V-T6).

#### T-101 Reduced cross-section method (4.2.2, Annex E)

- `def = dchar,n + k0 d0`, `d0 = 7 mm`, `k0 = t/20` for `t < 20 min` (unprotected: `k0 = 1.0` for `t ≥ 20 min`).
- Effective cross-section: exposed faces reduced by `def` (3/4-sided). Cross-section resistance with `kmod,fi = 1` and `kh` unchanged; `Rd,fi = kfi Rk/γM,fi`.
- Checks: T-10 to T-23 with the reduced section, bending with `Wy,fi`, `kcrit,fi` with `E0,05`, `G0,05` reduced... (the fire form of T-22/T-20 uses `kmod,fi = 1`).
- Output: residual section dimensions and utilisation at the required R-time; R-time achievable as `t_R = max t with η ≤ 1` (bisection).

#### T-102 Zero-strength layer method (EN 1995-1-2 Annex E / DIN EN 1995-1-2/NA)

- `def = dchar,0 + k0 d0` using `d0 = 7 mm`; effective section `b − 2 def`, etc.; NA-DE accepts the reduced cross-section with `d0 = 7 mm` (A) and `dchar,n`; for panels in cavity fill; ZSL with `k0` ramp.
- Explicit comparison output (`dchar,0` vs `dchar,n`) for the viewer.

#### T-103 Fire protection claddings and `tch`, `tf` (3.4.3)

- Cladding failure time `tf` (Gypsum A, F boards), `tch` start of charring behind protection; charring phases (`k2`, `kpos`); `dchar` after `tf`: `β = k2 β0` phases with `k2 = 2.0` etc. Data-driven formulas from EN 1995-1-2 3.4.3.3 **[V]**.

#### T-104 Connections in fire (6.2)

- Simplified rules (6.2.1): unprotected connections with side members of wood ≥ `t_fi/…`: `tfi,rq = …` and increase factor in minutes `ttot`; added edge distance `afi`; fire resistance by `Rfi,d,η = η Rd` with `η = e^(−k t_req)` for exposure `k = … d/(ρ)`; for tabulated values: `ds` e.g. R30 side timber ≥ `a_fi`, member thickness; `Fv,Rd,fi = kfi Fv,Rk/γM,fi`.
- Steel plates exposed: `η` reduction of capacity with steel temperature (EN 1995-1-2 6.2.1.2).

#### T-105 Fire tabulated: timber floors/walls (cavity) (EN 1995-1-2 Annex D)

- Floor/wall assemblies: separating function EI using `tins = Σ tprot,i + Σ tins,i`, with `tprot` of boards (`tprot = (hp / (…)`). Data-based Annex C **[V]**; flagged out of immediate scope in phase 1 but present in registry.

## 4. EN 1995-2 Bridges (timber bridges)

#### T-120 Pedestrian dynamic (EN 1995-2 7.2, 7.3, DIN EN 1995-2/NA)

- Natural frequency `f1` of the first vertical mode: `f1 = (π/(2ℓ²)) √(EI/m)` (simply supported), and lateral. If `f1 > 5 Hz` vertical / `f1 > 2.5 Hz` horizontal ⇒ no dynamic check; otherwise accelerations: `a = n0.5? …` via `Nequ`, with synchronous crowd `Fv = Fp ψ cos(2π f t)`; comfort limit `avert,max = 0.7 m/s²` (vertical), `0.2 m/s²` (lateral, normal), `0.4` (crowded); `ψ` from EN 1991-2 or Annex B of EN 1995-2; DIN EN 1995-2/NA sets `a_max`.
- Inputs: `m` [kg/m], `ℓ`, damping `ζ` (glulam 0.015; with deck 0.02–0.03?), crowd density `n [persons/m²]`.
- Repo fields already present: `bridge_n_obs, bridge_t_l_years, bridge_beta, bridge_a, bridge_b, bridge_crowd_per_m2` map to **fatigue parameters (n_obs, t_L)** and the crowd density; the new schema renames them (section 5).

#### T-121 Fatigue (EN 1995-2 6.2)

- Stress range `Δσ ≤ …`; traffic-induced fatigue for glulam bridge decks is only verified for members with `Δσ > 0.2 fd`... simplified `kfat = 1 − …`: `(fm)fat = kfat fm,d`, `kfat = 1 − log(N)/…` **[V]**. Provide `n_obs` yearly passages, `tL` design life, Miner sum.

#### T-122 Stress-laminated decks and bridge decks (EN 1995-2 Annex A, 5)

- Orthotropic plate model: `Dx, Dy, Dxy, kgrid`; `bef` for point load spread (Annex A.2): `bef = …`; prestress loss `σp,min ≥ 0.35 N/mm²` (stress-laminated: `fcont` friction, long-term loss 50–60 %), `slip check` `μ σp,min ≥ τ`: `0.35`..; check bolt/screw wheel loads on plank with `Qd`, `kdis`, wheel contact `a × b = 400 × 400 mm?` (EN 1991-2 load model 1 wheel print 0.4 × 0.4 m). **[V]**.

#### T-123 Connections and details in bridges (EN 1995-2 6.1, 6.3)

- Reduced `kmod` for bridge: `kmod` EC5-2 table 3.? (service class 2/3 only, moisture protective cover 1.0 `γM,connection 1.3`), `ke`; bearing of crossing members; protection `type`; verification of dowelled connection wrap-up; cyclic `fatigue of connections`.

#### T-124 Bridge vibrations walkers, hand rails and robustness — see T-120..T-123.

## 5. EN 1995 schema-first entity model (target)

Layer 1: domain neutral kernel entities (reused across all Eurocode artifacts); Layer 2: timber-specific extension. All entities carry `LocalizedLabel::native(en, de)`. Events are the only mutation interface (CQRS + ES); snapshot is a projection.

```
En1995Snapshot {
  annex: AnnexChoice (De | En | None -> must be explicit)
  edition: StandardEdition { part: "1-1"|"1-2"|"2", edition: "2004+A2:2014"|"2025" , na: "DIN EN 1995-1-1/NA:2013-08"}
  design_situation: DesignSituation
  materials: Vec<TimberMaterial>            // product, class, edition, derived props (T-01)
  sections: Vec<TimberSection>              // rect, circle, I, box, T, trapezoid, CLT panel, user polygon
  members: Vec<TimberMember>
  restraints: Vec<LateralRestraint>
  supports: Vec<Support>
  load_groups: Vec<LoadGroup> + actions: Vec<CharacteristicAction>
  combinations: Vec<CombinationDef>
  connections: Vec<TimberConnection>
  assemblies: Vec<Assembly>                 // floor, wall diaphragm, truss, mechanically jointed beam, CLT panel
  fire: Vec<FireScenario>
  bridge: Option<BridgeContext>
  vibration: Vec<VibrationSetup>
}
TimberMaterial   { id, label, product: TimberProduct, strength_class, edition, props: StrengthProps, density, hardwood: bool, treatment }
StrengthProps    { fm_k, ft0_k, ft90_k, fc0_k, fc90_k, fv_k, fr_k, e0_mean, e0_05, e90_mean, g_mean, gr_mean, rho_k, rho_mean }
TimberSection    { id, shape: SectionShape, b, h, tf, tw, lamellae: Vec<Lamella>, layers: Vec<ClTLayer> , derived: SectionProps{A, Iy, Iz, Wy, Wz, It, Wt, S, i_y, i_z} }
TimberMember     { id, material, section, role, geometry: MemberGeometry{ axis: Polyline3d, taper: Option<Taper>, curve: Option<Curve>, apex: Option<Apex>, pitch, camber }, service_class, support_ids, restraint_ids, notches: Vec<Notch>, holes: Vec<Hole>, reinforcements: Vec<Reinforcement>, ksys_group: Option<GroupId>, effective_lengths: EffLengths{ ly, lz, lef_ltb, source: Manual|Derived } }
Notch            { id, side: Top|Bottom|Support, x, depth, i_taper, radius, reinforced }
Hole             { id, x, y, d, shape }
Reinforcement    { id, kind: Screws|Rods|Panels, positions, d, l, n }
LateralRestraint { id, x, kind: Point|Continuous, prevents: Displacement|Rotation|Both }
CharacteristicAction { id, kind, category, load_duration, distribution: Uniform|Point|Moment|Trapezoid|..., value, position, psi0, psi1, psi2 }
TimberConnection { id, label, members: Vec<MemberRef>, kind, fasteners: Vec<FastenerGroup>, plates: Vec<SteelPlate>, geometry: ConnectionGeometry3d }
FastenerGroup    { id, type: Nail|Staple|Bolt|Dowel|Screw|SelfTappingScrew|SplitRing|ShearPlate|ToothedPlate|NailPlate|GluedInRod, d, l, fu_k, positions: Vec<FastenerPos{x,y,z,angle}>, predrilled, head, thread_length, approval: Option<ApprovalRef> }
SteelPlate       { id, thickness, grade, polygon, holes }
Assembly         { id, kind: Floor|WallDiaphragm|Truss|MechJointedBeam|ThinWebBeam|ClT|Roof, parts: Vec<PartRef>, joints: Vec<JointSpec> }
FireScenario     { id, member_ids, t_req_min, curve: Standard|Parametric, exposed_faces: bitmask, protection: Vec<Cladding>, method: ReducedCrossSection|ZeroStrength }
VibrationSetup   { id, floor_assembly, span_x, span_y, mass_kg_m2, damping_zeta, ndp_ref }
BridgeContext    { deck, crowd_density, n_obs, t_L, fatigue params, load models }
NdpSet           { annex, key->Value with source clause }
Verification      { id, clause, inputs_schema, evaluator_ref }   // registry
CheckResult       // see section 0
```

Events (extending `mutations`): `MaterialAssigned`, `SectionResized`, `MemberAdded|Moved|Removed`, `NotchPlaced`, `HolePlaced`, `RestraintPlaced`, `LoadAdded|Changed|Removed`, `FastenerPlaced|Moved|Removed`, `FastenerPatternApplied`, `ConnectionAssembled`, `FireScenarioSet`, `AnnexSwitched`, `NdpOverridden` (explicitly marked non-compliant), `CombinationDefined`. Every event is a command-validated, replayable fact; the projection recomputes only affected verifications (section 8.4).

Vocabulary migration from the current snapshot: `TimberMember.{b_m,h_m,span_m,m_crit_nm,...}` become section/geometry/restraint entities; flat `bridge_*` fields become `BridgeContext`; `m_crit_nm` input is removed (derived by T-22); `CharacteristicAction.{q_line_n_per_m, f_point_n, m_k_nm, v_k_n, n_k_n, n_t_k_n, f_c90_k_n}` becomes typed `distribution` with positions. No compatibility layer is kept (greenfield).

---

# PART B. EN 1996 (Masonry)

## 6. Materials and parameters (EN 1996-1-1 chapter 3)

#### M-01 Unit classification and normalised strength `fb` (3.1, EN 772-1)

- Inputs: declared mean compressive strength `fb,decl`, unit dimensions `l × w × h` (mm), conditioning (air-dry), shape factor `δ` (Table NA.1 EN 772-1: height 50…250 mm, least horizontal dimension 50…250 mm; e.g. `h = 238, w = 240 → δ = 1.15?`) **[V]**, `fb = δ fb,decl`.
- Group classification (Group 1: net hole volume ≤ 25 %; Group 2: 25–55 %; Group 3: 25–70 %; Group 4: 25–70 % with web/shell limits (Table 3.1 limits per clause 3.1.1)). Material: clay, calcium silicate, aggregate concrete, autoclaved aerated concrete (AAC), manufactured stone, natural stone.
- Output: `fb`, group, category I/II (control category). NA-DE: unit strength classes `SFK` 6…60, mean `fb = SFK × 1.2? `: DE class `12` means `fb,decl ≥ 12 N/mm²` min and mean by factor 1.25 (DIN V 105-100) **[V]**.

#### M-02 Mortar and characteristic compressive strength `fk` (3.6.1.2)

- Mortar: GP (general purpose) `M1…M20` (N/mm², `fm`), TL (thin layer, `fm ≥ 5`), LW (lightweight, ρ < 1500). NA-DE: `NM I, II, IIa, III, IIIa`, `DM` (thin layer), `LM21`, `LM36` with `fm` = 2.5, 5, 10, 20 (IIIa: 15?) **[V]**.
- Formula (3.2): `fk = K fb^α fm^β` [N/mm²] with `α = 0.7, β = 0.3` for GP, `α = 0.85, β = 0` for thin-layer (clay Group 1/2? and AAC/calcium silicate), `α = 0.7, β = 0.3` LW; `K` from Table 3.3 by unit material/group and mortar type (clay G1 GP 0.55 / TL 0.75 / LW 0.30; G2 0.45 / 0.70 / 0.30; G3 0.35 / 0.50 / 0.25; G4 0.35 / 0.35? / 0.25; calcium silicate G1 0.55 / 0.80 / 0.45?; concrete G1 0.55 / 0.80 / 0.45; AAC G1 0.55 / 0.80 / 0.45) **[V]** table `KTable`. Limits: `fk ≤ 0.8 fb` (for GP; stricter `fk ≤ 0.8 fb` also for groups 2–4 conditions on `fm ≤ min(20, 2 fb)`).
- `fm` limits (3.6.1.2 (2)): GP `fm ≤ 20 N/mm²` and `fm ≤ 2 fb` for Groups 2–4; `fm ≤ min(20, 2fb)`. With thin layer: `fm` = 10.
- Unit with filled collar joints (>0.6 hole): `fk` factor 0.8 for dry / unfilled vertical joints `fk ≥…` **[V]**.
- NA-DE (DIN EN 1996-1-1/NA, NCI table NA.5…NA.12): `fk` is tabulated per unit type (Vollziegel Mz, Hochlochziegel HLz A/B, Kalksand KS, KS-L, Betonsteine Hbn, Hbl, Vbl, Porenbeton PP/PPE, Leichtbeton), per strength class, per mortar group (NM IIa, III, IIIa, DM, LM21, LM36); the kernel stores these tables as data (`FkTableNaDe{unit_type, sfk, mortar → fk}`) and can still compute `K fb^α fm^β` for non-NA cases; tabulated values override. Example row (to be pinned from the NA): `HLz 12 NM IIa` ⇒ ~ 4.4 N/mm² **[V]** — do not hardcode; fixtures use only entries verified against the oracle.
- Output: `fk`, branch used (`formula|table`), `γM`, `fd = fk/γM`.

#### M-03 Characteristic shear strength `fvk` (3.6.2)

- Formula (3.5): `fvk = fvk0 + 0.4 σd`, limited by `fvk ≤ 0.065 fb` (Group 1) or `fvlt` the stated value, and `fvk ≤ fvlt` where unit tension failure governs; `σd` design compressive stress perpendicular to the shear plane at the member's compressed portion [N/mm²].
- `fvk0` (Table 3.4): GP mortar M10…M20 `0.30` (clay Group 1, calcium silicate, concrete, AAC), M2.5…M9 `0.20`, M1…M2 `0.10`; thin-layer `0.30`; lightweight `0.15`; Group 2–4 `0.15` **[V]**.
- Filled/unfilled perpends: for unfilled head joints (`fvlt`): `fvk = 0.5 fvk0 + 0.4 σd` (EN 6.2 (2)) **[V]**.
- NA-DE: `fvk = fvk0 + 0.4 σDd` with additional tensile limits `fvk ≤ 0.045 fb?` (Brickwork `fbt,cal = 0.026 fb` for horizontal `tensile`), `fvlt` in terms of `hu/lu` (unit aspect ratio): `fvlt = 0.45 fbt,cal √(1 + σ/fbt,cal)` for DE (disc shear) **[V]**.

#### M-04 Flexural strength `fxk1`, `fxk2` (3.6.3)

- `fxk1` — plane of failure parallel to bed joints; `fxk2` — perpendicular to bed joints. EN Table 3.6 per unit type/mortar strength (e.g. clay `fm < 5: fxk1 = 0.10, fm ≥ 5: 0.10`; `fxk2 = 0.20/0.40 (Group 1)`; calcium silicate `0.05/0.10 …`) **[V]**; `fxd = fxk/γM`. Orthogonal ratio `μ = fxk1/fxk2`.
- NA-DE: `fxk1` in DE denoted `fxk,1 = 0.0 …` tension-free for lateral design when no tension considered ("Biegezugfestigkeit nicht angesetzt") for walls with permanent compression; table NCI **[V]**.
- Also: `fxk1` can be increased with the minimum vertical stress `σd` per 6.3.1: `fxd1,app = fxd1 + σd` (apparent flexural strength) (6.16).

#### M-05 Initial shear strength and anchorage (3.6.2 / 3.6.4)

- Anchor bond `fbok = 4.0 N/mm²` plain bars, `fbok` = 2.0 etc. (EN 1996-1-1 Table 3.5 and 8.6) ⇒ `lb` anchorage length for reinforced masonry: `lb = ϕ fyd/(4 fbd)`, `fbd = fbok/γM`.

#### M-06 Elastic modulus and creep (3.7)

- Short-term modulus of elasticity `E = KE fk` with `KE = 1000` (EN rec.; range 700-1100); NA-DE: `E = 950 fk` (via NA Table NA.1; actual NA gives per-unit `E` values e.g. `E = 950 fk`; Poisson, shear `G = 0.4 E`) **[V]**.
- Creep `φ∞ = φ(∞)` (EN Table 3.2): clay 0.5–1.5; calcium silicate 1.0–2.0; concrete (dense & lightweight) 1.0–2.0; AAC 0.5–1.5; manufactured stone 1.0–2.0 — NA-DE gives single values (e.g. clay 1.0 **[V]**. Long-term modulus `Elt = E/(1+φ∞)`.
- Shrinkage/ thermal: `εcs`, `αt` (Table 3.2: clay 0 to −0.2 mm/m, 6×10⁻⁶; calcium silicate −0.2 to −0.4, 10×10⁻⁶; AAC −0.2 to −0.4 mm/m, 8×10⁻⁶; concrete −0.2 to −0.6, 10×10⁻⁶).
- Output: `E, G, Elt, εcs, αt`, plus design `Ed = E/γM` for second-order (EN 1996: `E/ γM` not applied; documentation `E` only).

#### M-07 Partial factor `γM` for masonry (Table 2.3)

- Table 2.3 (EN) by class: materials category I units: mortar designed 1.7 / prescribed 2.0 (execution class 1), class 2: 2.0/2.2, class 3 (2.2/2.5?)… The repo maps `MasonryClass 1…5 → 1.5, 1.7, 2.0, 2.2, 2.5`. NA-DE: `γM = 1.5` (Kat I units with design mortar), `1.7` ... ; "Allgemein 1.5; bei Verwendung von Normalmörtel+Kat.II 1.7; Montagezustand 1.3; außergewöhnlich 1.3; reinforcement `γS = 1.15`; accidental fire 1.0 **[V]**.
- Reinforced masonry reinforcement `γS = 1.15`; anchorage/bond `γM`; ties and connectors `γM = 1.5…`.
- Execution classes & control: category I/II units, Class 1/2 execution.

## 7. EN 1996-1-1 verification catalogue

Definitions: wall thickness `t`, clear height `h`, wall length `l`, effective height `hef = ρn h`, effective thickness `tef` (t single leaf; `tef = ρt t` stiffened by piers: `ρt` from Table 5.2 with pier spacing `c_s` and `tp/t`; cavity walls `tef = (t1³ + t2³)^(1/3)` under equal loading), `fd = fk/γM`.

### 7.1 Vertical load

#### M-10 Effective height `hef` and restraint factor `ρn` (5.5.1.2)

- `hef = ρn h`, `ρn` by edge support: 
  - 2 sides (top and bottom held): `ρ2 = 1.0` (no moment restraint), `ρ2 = 0.75` if floor slab (concrete) bearing ≥ `2t/3` on both sides with eccentric load `e ≤ 0.25 t` (or slab bearing ≥ `0.85?`); `ρ2 = 0.75` only for top and bottom restraints per 5.5.1.2 (3) (timber floors: `ρ2 = 0.85–1.0`); NA-DE: `ρ2 = 0.75` for monolithic concrete floors bearing the full wall thickness… and `ρ2 = 0.9` ; `ρ2 = 1.0` timber floors **[V]**.
  - 3 sides (one vertical edge free, `L` = free distance from the free edge to the stiffening wall): `ρ3 = ρ2/(1 + (ρ2 h/(3 L))²)` for `h ≤ 3.5 L`; `ρ3 = 1.5 L/h` for `h > 3.5 L` (and `ρ3 ≥ 0.3`: `ρ3` ≥ 0.3).
  - 4 sides (stiffening walls both sides, spacing `L`): `ρ4 = ρ2/(1 + (ρ2 h/L)²)` for `h ≤ 1.15 L`; `ρ4 = 0.5 L/h` for `h > 1.15 L`.
- Stiffening walls requirements (5.4): length ≥ `1/5 h`, thickness ≥ 0.3 t (≥ 85 mm), openings limits. NA-DE: `ρn` for walls with openings: reduce via `h/L` for unsupported length at openings (`h_ef` calculation for walls with window ≥ 1/4 of area) **[V]**.
- Output: `hef` per axis, and the stiffener check status.

#### M-11 Slenderness `λ = hef/tef` (5.5.1.4)

- Limit: `hef/tef ≤ 27` (EN rec., NA-DE: ≤ 27 for load-bearing; ≤ 21 for simplified method EN 1996-3 (`hef/t ≤ 21`)..) **[V]**; `h/t ≥ …`.
- Output: `λ`, status; value `λc` for `ρ` adjust.

#### M-12 Eccentricity at top/bottom `ei` and `Φi` (6.1.2.2)

- `ei = Mid/Nid + ehe + einit ≥ 0.05 t` where `Mid` design bending moment at top/bottom of wall from the floor connection (continuous frame analysis, or simplified: `Mid` = from Annex C coefficients, or `e = t/2 − a/3` with `a` bearing depth), `ehe` eccentricity due to horizontal loads (wind) at the node, `einit = hef/450`.
- `Φi = 1 − 2 ei/t` (6.4). Resistance per m: `NRd,i = Φi t fd` (6.2). Check `NEd,i ≤ NRd,i`.
- Simplified bearing eccentricity from beams/floors: `e = t/2 − a/3` with `a` the bearing length (EN 1996-1-1 Annex C / NA). Edge: `Φi ≤ 0` ⇒ fail (eccentricity > t/2).

#### M-13 Mid-height `Φm` (6.1.2.2, Annex G)

- `emk = em + ek ≥ 0.05 t`, `em = Mmd/Nmd + ehm ± einit`, `ek = 0.002 φ∞ (hef/tef) √(t em)`.
- `Φm = A1 exp(−u²/2)` (Annex G), `A1 = 1 − 2 emk/t`, `u = (λ − 0.063)/(0.73 − 1.17 emk/t)`, `λ = (hef/tef) √(fk/E)`.
- Check `NEd,m ≤ Φm t fd`. Output: `NRd` at top, mid, bottom; governing section along height; profile `Φ(z)` for the viewer.
- NA-DE: simplified `Φm` via the `NA` table (Φ as function of `λ` and `e/t` for `Φm`) allowed; same underlying `Φm` expression for the formula route; `k_m = 1.0`.

#### M-14 Concentrated loads (6.1.3)

- `NEdc ≤ NRdc = β Ab fd`, `β = (1 + 0.3 a1/hc)(1.5 − 1.1 Ab/Aef) ≤ min(1.25 + a1/(2 hc), 1.5)` with `β ≥ 1.0`; `Ab` loaded area, `Aef = lefm t` (effective bearing length at mid-height `lefm`), `a1` distance from wall end to load edge, `hc` height from load to the base. For `Ab > 0.45 Aef`: `β = 1`. Bearing stress under the load plus other vertical loads: `(NEdc + NEd,other) ≤ …` and local `Ab ≤ 0.45 Aef`; check bearing at plates with spreading.
- NA-DE: `β` formula same; limit on `σ1 = Fc/Ab ≤ 1.5 fd` for `Ab ≤ 2·t²`. Output: `β`, capacity, spreading length.

#### M-15 Walls with openings: piers and lintels (5.5, 6.1, Annex)

- Piers treated as independent walls (effective height = opening height or storey height depending on restraint); the load path above openings via lintel arching: triangle load (45° or 60° spread) `Wlintel = G(h ≤ l/3?)`; masonry arch action over lintel check M-18.
- Output: stresses per pier; piers `lpier ≥ 0.5 m`; eccentricity with partial support.

#### M-16 Walls loaded with vertical loads and wind simultaneously (6.1.2 (2), Annex A)

- The vertical resistance is evaluated for the eccentricity with `ehe`; bending effect included via `Mid`.

### 7.2 Shear

#### M-20 Shear wall resistance (6.2)

- `VEd ≤ VRd = fvd t lc`, `fvd = fvk/γM`, `lc` compressed length of wall: `lc = 3 (l/2 − e)` with `e = M/N`; for triangular compressed zone `lc = 3(l/2 − e)` ≤ l, with `e` eccentricity of the resultant; `fvk` per M-03 using `σd = NEd/(lc t)`.
- Extra checks: `fvd` limited by `0.065 fb/γM` (EN) and tension failure of units; rocking (overturning) `e ≤ l/6` for no tension; cracked section.
- Shear walls with flanges (6.2, 5.5.3): effective flange width per side `bf,ef = min(0.15 H?, l_flange, 6 t_flange, 0.5 × distance…)` — EN 1996-1-1 5.5.3 (? effective flange width `≤ 0.1 H` ... `bef` per side `≤ min( ½ clear distance between webs, 6 tf + tw, 0.2 Htot)`) **[V]**; the interaction "T/U/L" cross-section `I`, centroid; shear stress distribution in the web.
- NA-DE: simplified shear verification (no wall panels): `VRd = fvd (lcal)…` with `fvk = fvk0 + 0.4 σDd`, `lcal = …`; also the tension failure `fvk ≤ 0.45 fbt,cal √(1 + σ/ fbt,cal)`; overturning stability `ULS: ε` with `0.9 G` **[V]**.

#### M-21 Bracing wall layout and torsion (5.4, 8.5)

- Building-level: stiffness-weighted distribution of horizontal load to walls (`k = 1/(h³/(3EI) + h/(GA))`), torsion by eccentricity `e = e0 + 0.05 B`; check each wall M-20. Provides coherent 3D distribution for viewer.

### 7.3 Lateral loading

#### M-30 One-way vertical span (6.3, 6.3.1)

- `MEd = wEd h²/8` (pinned, simply supported) or `wEd h²/(2 …)`, cantilever `wEd h²/2`; `MRd = fxd1or2 Z`, `Z = t²/6` per metre width; apparent flexural strength `fxd1,app = fxd1 + σd` (compression helps, `σd ≤ 0.2 fd`? 6.16) ⇒ `MRd = (fxd1 + σd) Z`; resistance with zero tension: `MRd = σd t²/6?` etc.
- Output: `MEd/MRd`.

#### M-31 Two-way bending (6.3.2, Annex E)

- `MEd1 = α1 wEd l²` (about axis parallel to bed joints → failure plane parallel to bed joints, `fxd1`), `MEd2 = α2 wEd l²` (`α2 = μ α1`), `μ = fxk1/fxk2` orthogonal strength ratio. `α` coefficients from the EN 1996-1-1 Annex E tables for boundary conditions A–J (four edges pinned, three edges with free vertical edge, two edges with free top, etc.) as function of `h/l` and `μ` (interpolation) **[V]** (tables are data, `AlphaTable{support_case, mu, h_over_l -> alpha}`).
- Resistance `MRd1 = fxd1 Z`, `MRd2 = fxd2 Z`; `Z = t²/6`. Check `MEd1 ≤ MRd1` and `MEd2 ≤ MRd2` (6.12, 6.13).
- Yield line oracle: yield-line pattern analysis computed via upper-bound for given boundary conditions & orthotropic moments (shown in the viewer).
- NA-DE: tension strengths `fxk` as tabulated (zero if no tension recognised); `fxd1` apparent.

#### M-32 Arching (6.3.3, Annex E.? and NA-DE)

- Lateral load capacity with arching for walls laterally restrained at top and bottom: `qlat,d = fd (t/ la)² ·  …`: EN 1996-1-1 (6.17)/(6.18) `qlat,d = fd t²/ la²``qlat,d = 2? fd (t/ℓa)²`** as **[V]** — the explicit coefficient, the condition `ℓ_a/t ≤ 25?` and the thrust condition `t ≥ 100 mm` and `σd`-dependent range must be pinned from the normative text before implementation; the kernel test uses the oracle (statics of three-hinge arch with thrust block `0.1 t` at the supports).
- Edge: arch rise with elongation; stiffness of supports `kA` check. In NA-DE the arching reduction applies only if the wall abutments can take the thrust: `ΔL/L ≤ …`.

#### M-33 Wind and earth pressures

- Wind `we = qp cpe` (EN 1991-1-4) from `en1991` kernel; internal pressure; earth pressure `e0,k = K0 γ h` for basement. Combined loads `ψ0`.

#### M-34 Retaining/basement walls (6.3 + EN 1996-3 4.? and NA-DE)

- Simplified NA/EN 1996-3 condition (Kellerwand): `N1,Rd` between `NEd,sup,min ≥ ρe h he²/(20 t)` and `NEd,max ≤ 0.45 t fd`... **[V]** — wall height `h ≤ 2.6 m`, soil height `he`, thickness `t ≥ 240 mm`, `ρe` backfill density (kN/m³), `qk ≤ 5 kN/m²` surface load, no hydrostatic pressure; verification of crushing `NEd ≤ NRd`.

### 7.4 Reinforced, confined and special masonry

#### M-40 Reinforced masonry in bending (6.6.1, 6.6.2)

- Section: `d` effective depth, `b`, `As`, `fyd = fyk/γS`; `z = d (1 − 0.5 As fyd/(b d fd)) ≤ 0.95 d`; `MRd = As fyd z` with the limit on `MRd ≤ 0.4 fd b d²` (limitation of the compression zone depth: `x ≤ 0.4 d`).
- Check anchorage `lb` (M-05) and minimum reinforcement `As,min ≥ 0.05 % b d` and max `4 %`.
- Edge: `As fyd/(b d fd)` too high ⇒ failure of masonry before steel.

#### M-41 Reinforced masonry in shear (6.7.2)

- `VRd1 = fvd b d` with `fvk = fvk0 + 0.4 σd` and enhancement `2d/av`; with shear reinforcement `VRd2 = 0.9 Asw d fyd/s? ` (EN 1996-1-1 6.7.2 (3)): `VRd2 = Asw fyd 0.9 d / s`; `VRd ≤ 0.25 fd b d`.

#### M-42 Reinforced masonry deep beams and column-like members (6.7)

- Deep beams: lever arm `z = 0.7 l` (`h ≥ 0.5 l`), `Fs = M/z`, bearing; walls loaded in-plane `MRd` with vertical bars: interaction `M-N` diagrams for reinforced shear walls (plane section, strain limits `εmu = 0.0035`, `εsy = fyd/Es`). Interaction curve for viewer.

#### M-43 Confined masonry (EN 1996-1-1 8.7, Annex)

- Confining columns and tie beams: minimum column area, reinforcement, spacing and shear, realised as the check `m-43.`. Prestressed masonry stays out of scope for phase 1.

### 7.5 Ties, cavity walls and accessories

#### M-50 Wall ties in cavity walls (8.5.2.3, EN 1996-2 3.?)

- Number of ties per m²: `n ≥ 2.5/m²` (EN 1996-1-1 8.5.2.3 for cavity 75…150 mm, Table: 2.5 ties/m² for the standard; 3/m² for ≥ 100 mm), at openings, near free edges, within 150 mm, 300 mm vertical spacing ties; tie capacity `Ftie,Rd` from EN 845-1 and the NA. Check: wind pressure distributed `wEd` between leaves in proportion to stiffness `E t³`: `w1 = wEd t1³/(t1³ + t2³)`; each tie `Ftie,Ed = wEd,leaf A_trib`, resist `Rd`. NA-DE: DIN EN 1996-1-1/NA 8.5.2.3 Table NA.13: ties per m² = 7 (cavity 120 mm?)`n ≥ 7` in general, `n ≥ 5` for walls where `h ≤ 12 m`… **[V]** (Z-17.1, Table NA.1 of 1996-1-1/NA).

#### M-51 Connections between walls and floors, ring beams (8.5.1, Annex D)

- Anchors: horizontal restraint force `Fd = 1.5%?`: lateral anchor force of floor tie `Td = max(15 kN?, …)`; tie force for stability: ring beam tie `Td ≥ 30 kN`; NA-DE: ring beam/tie `Ft,Rd = 30 kN?`. Lap length; **[V]**.

#### M-52 Movement joints and non-structural spacing (EN 1996-2 2.?) — recommended spacing `l ≤ …` per material: clay 12 m, calcium silicate 7–8 m, AAC 6 m: input check reports `warning` not failure.

### 7.6 Serviceability and durability

#### M-60 Exposure and durability (EN 1996-2, EN 1996-1-1 section 4)

- Exposure class MX1…MX5, minimum unit freeze/thaw resistance F1/F2, reinforcement cover and protection class (MX), mortar class suitability (M5 for MX3 etc.). NA-DE: `Frost-Beständigkeit` Table NA.? **[V]**. Pass/fail `not_applicable` outputs.

#### M-61 Deformation limit and crack control (EN 1996-1-1 7, 8.5.1)

- Rotation at supports/spalling; vertical deflection of lintels `δ ≤ l/500`; crack width not computed. Reinforced masonry deflection `l/d` ratios (7.3.2: `(l/d)max` = 20 simply supported, 7 cantilever, 26 continuous (for `fyk = 500` modify `×(310/ σs)`)) **[V]**.

### 7.7 EN 1996-2 execution and design aspects (masonry bridges excluded; part 2 = design considerations, selection of materials and execution)

#### M-70 Execution classes and tolerances (EN 1996-2 3, 8)

- Execution class checklist: unit moisture, mortar workability, head joint fill, bed joint thickness limits (`6…15 mm` GP; `0.5…3 mm` thin layer), perpend overlap `lol ≥ 0.4 hu` and `≥ 45 mm`: `lol ≥ 0.4 hu` (EN 1996-1-1 8.1.5 (2)); bonds (stretcher, header, English, Flemish) pattern validity `overlap ≥ 0.2 hu``bond_valid` check with `lol ≥ 0.4 hu` (NA: ≥ 0.4 hu and ≥ 45 mm; for `hu > 250` ≥ 0.4 hu) used also by the viewer (M-70 colouring invalid joints red).
- Minimum wall thickness: load-bearing ≥ 100 mm?, NA-DE ≥ 115 mm; piers area ≥ 0.04 m² (NA-DE ≥ 0.10 m².1)... **[V]**.
- Chases and recesses (8.6, Table 8.1): limits depth/width by wall thickness: NA table (vertical chases `t ≥ 115?: depth ≤ 30 mm`…). Check `chase_ok` + reduced `tef` for resistance **[V]**.

### 7.8 EN 1996-3 Simplified calculation method (NA-DE: DIN EN 1996-3/NA)

#### M-80 Applicability limits (EN 1996-3 4.1, NA-DE 4.1)

- Building height `H ≤ 20 m` above ground (≤ 5 storeys? NA-DE: `hges ≤ 20 m`), floor span `lf ≤ 6.0 m` (7.0 m for Betondecke? DE limit 6.0 m for ρ2 = 0.75), clear storey height `h ≤ 3.0 m` (DE: 2.75 m for `t < 240`; NA: `h ≤ 12 t`), imposed load `qk ≤ 5 kN/m²`, wall thickness `t ≥ 115 mm` (load-bearing: ≥ 115 mm, ≥ 150 mm for `H > 2 storeys`); stiffening/wind: structural-box action; wall length `≥ 0.5 m`, wall area `≥ 0.1 m²`. All limits explicit in data `SimplifiedLimits{annex}`; **[V]** for each NA value.
- Output: list of failed applicability conditions (blocking: `out_of_scope`).

#### M-81 Simplified vertical resistance (EN 1996-3 4.2, NA-DE 4.2)

- `NEd ≤ NRd = Φs A fd`, `A = t · l` (net, deducting chases), `fd = fk/γM`.
- `Φs = 0.85 − 0.0011 (hef/t)²` (EN 1996-3, 4.2.2.3 (1), with `hef/t ≤ 21`) with `Φs ≥ …`; for slab-supported walls `Φs = min(Φs, Φ2)` with `Φ2 = 1.6 − lf/6 ≤ 0.9 t..` (floor slab span `lf`; `Φs = Φ2` when floor spans exceed 4.2 m). NA-DE: `Φ1 = 0.85 − 0.0011 (hef/t)²` for the wall `Φ2 = 1.6 − lf/6` `≤ 0.9 t/…` (`Φ2 ≤ 0.9`, `1.6 − lf/6` with `lf` in m (lf ≤ 6 m)), and at the lowest storey top: `Φ3 = 0.333` for `lf > 6`?; the exact set of `Φ1, Φ2, Φ3` and the roof-floor top storey rule `Φs = Φ3?` must be **[V]** against the NA text, as must `ρ` for walls fixed at top/bottom `ρ2`.
- Effective height `hef = ρn h`, `ρn` per M-10 with DE simplified `ρ2 = 0.75/0.90/1.0` for the supports (NA 4.2.2.3 Table NA.? ).
- Concentrated loads in simplified method: `NEdc ≤ 1.5? fd Ab` where `Ab ≤ 0.2 A?`; NA: `σ ≤ 1.5 fd`.
- Walls subjected to wind: no separate check if `qlat` small `…` (EN 1996-3 5: `wEd ≤ …` via `H ≤ …`).
- Output: `Φs`, `NRd`, `η`, chosen `Φ` branch trace.

#### M-82 Simplified shear (EN 1996-3 4.4 / NA 4.?)

- Wall panels: `VEd ≤ VRd = fvd t lc` with `lc` minimal 0.3 m / `fvk0` per NA and `fvk` compression `σD`; NA: 4.4 requires minimal cross-wall lengths (`l ≥ 0.5 m`) and stiffening wall numbers ("Windscheiben") per height `H`.

#### M-83 Simplified lateral loading & basement walls

- Walls not subject to lateral loads other than wind in 4.? (small panels): the simplified method excludes calculation when thicknesses & lengths satisfy table; basement: M-34.

### 7.9 EN 1996-1-2 Fire (masonry)

#### M-90 Tabulated minimum thickness (EN 1996-1-2 Annex B.? / NA-DE)

- Tabulated data (Annex B): minimal wall thickness `t_min` for REI (load-bearing separating), EI (non-load-bearing), `REI 30/60/90/120/180/240` by unit material and group (e.g. clay Group 1: `REI 30: 100 mm`, `60: 100`, `90: 150`, `120: 175`, `180: 240`, `240: 240`) with utilisation level `αfi` (load ratio `μfi = NEd,fi/NRd`), full-fire exposure (one side) and mortar type, NA-DE (DIN EN 1996-1-2/NA Tabellen 4.1…): values by unit type, strength class, `μfi ≤ 0.6 | 0.7 | 1.0` and `1.4?` **[V]** (data table `FireTable{material, group, rei, mu_fi, mortar -> t_min}`).
- Check: `t ≥ t_min(rei_required, μfi, material)`; interpolation allowed between utilisation columns; plaster additions `tplaster × factor` equivalent thickness (EN 1996-1-2 Annex, NA).
- Output: pass/fail + `t_min`, and achievable REI as max class satisfied.

#### M-91 Advanced calculation: temperature profile method (EN 1996-1-2 3.3 / Annex C)

- Reduced section by `ad`/stresses with temperature-dependent `fk(θ)`, `E(θ)` using the standard curve — phase 2; registry stub `T-lane`; at minimum implement the `5 % rule` of Annex C (reduced strength factor `kθ`). **[V]**.

### 7.10 Seismic masonry (EN 1998-1 ch. 9) — referenced via `en1998`; `q` factors for unreinforced, confined, reinforced masonry with `Φ`-based in-plane verifications; `M-95 masonry_seismic_q` stub.

## 8. EN 1996 schema-first entity model (target)

```
En1996Snapshot {
  annex: AnnexChoice, edition, design_situation, execution_class (1|2|3), control_category (I|II)
  storeys: Vec<Storey{id, level_z, height, floor: FloorSystem}>
  wall_assemblies: Vec<WallAssembly>
  units: Vec<MasonryUnitType>, mortars: Vec<MortarType>, masonry_types: Vec<MasonryType>
  walls: Vec<MasonryWall>
  loads: Vec<MasonryLoad>, combinations
  reinforcement: Vec<ReinforcementDef>, ties: Vec<TieSet>
  fire: Vec<FireRequirement>
}
MasonryUnitType { id, material, group, standard (EN 771-x), l,w,h, hole_fraction, density, fb_decl, delta, fb, category }
MortarType      { id, kind: GP|TL|LW|NA-DE-class, fm, class label, bed_joint_t, head_joint: Filled|Unfilled|Interlocked }
MasonryType     { id, unit, mortar, fk_branch, fk, fvk0, fvlt, fxk1, fxk2, E, G, phi_inf, eps_cs, alpha_t, derived_trace }
MasonryWall     { id, assembly, masonry_type, wall_type: LoadBearing|Shear|NonLoad|Cavity|Basement|Pier|Lintel,
                  geometry: WallGeometry{ base polyline, height, thickness, leaves: Vec<Leaf>, piers, openings: Vec<Opening{kind, x,y,w,h,lintel}> , chases: Vec<Chase> },
                  bond: BondPattern{ kind, course_h, lol, courses (derived) },
                  supports: EdgeSupports{top:Restraint, bottom:Restraint, left, right} with bearing depths,
                  stiffeners: Vec<StiffeningWall>, flanges: Vec<Flange>, reinforcement_ids, tie_set }
Opening         { id, x, width, height, sill, lintel: LintelSpec }
EdgeSupport     { kind: Free|Pinned|FixedMonolithic|Floor{slab_depth, material, span, bearing}, restraint_factor_source }
MasonryLoad     { id, kind: Vertical|Eccentric|Concentrated|Wind|Earth|Imposed|Thermal, distribution, position, value, ecc }
LoadCase        // as combos with psi
Verification, CheckResult // shared with part A
```

Events: `UnitTypeDefined`, `MortarDefined`, `WallPlaced`, `WallMoved`, `WallResized`, `OpeningPlaced|Moved|Resized|Removed`, `BondPatternChanged`, `EdgeSupportSet`, `FloorBearingSet`, `LoadPlaced|Moved|Changed`, `ConcentratedLoadPlaced`, `ReinforcementPlaced`, `TieSetPlaced`, `AnnexSwitched`, `NdpOverridden`, `FkOverrideApplied` (explicit tested value from test certificates).

Migration of the current `MasonryWall` (flat `f_b_pa`, `mortar_strength_pa`, `phi_infinity`, `eccentricity_top_m`, etc.) into the structure above: no compatibility layer; fixtures are rewritten by hand.

---

# PART C. Visualisation

Stack: the shared `app-surface` (to be extended) with 3D viewport(s) built on the existing 3D plugins (`🏗️fem`, `🏙️bim`, `📐cad` shared mesh/scene contracts); every 3D object carries `entity_id` and `check_ids` so utilisation colouring and picking are generic. 2D diagrams render via the shared chart kit. Each visualisation has a headless scene description (JSON scene graph) that is test-asserted (vertex counts, bounding boxes, colour ramps), because the pixels are not.

## 9. Timber visualisations

- **V-T1 Member 3D**: extruded section along axis with true grain direction texture; tapered, double tapered, curved (arc/polyline lamellae), pitched cambered beams generated from `MemberGeometry` (lamella layers visible as stacked sheets for glulam, layers for CLT with alternating orientation colours); notches/holes cut in the mesh (boolean on parametric profile), reinforcements (screws/rods) as thin cylinders. Utilisation heat map by station along the axis (colour per check selectable: bending, shear, bearing, buckling, LTB, deflection).
- **V-T2 Stress diagram overlays**: bending `σ(y)` triangular blocks at section, shear parabola, tension perpendicular zone at apex (volume `V` shaded), notch region with `kv` annotation, bearing `lef` zone on the support.
- **V-T3 Connection 3D**: members as translucent solids; fasteners as cylinders (nails with heads, bolts with washers and nuts, dowels, self-tapping screws with thread length visualised, split rings/shear plates/toothed plates as discs, nail plates as planar patterns, glued-in rods); steel plates with holes; distances `a1, a2, a3, a4` drawn as dimension chains; violation zones (min spacing/edge/end distances) tinted red and measured live; load arrows with `α`; failure-mode inset (Johansen modes a-k) showing the deformed fastener shape and embedment stress blocks for the governing mode; block-shear/plug-shear rupture surface; splitting plane.
- **V-T4 Fire char layer animation**: time slider and play (0..t_req, default standard fire curve); charred layer drawn as black/dark-brown extrusion shrinking the residual section; `dchar,0`, `dchar,n`, `def` shown as nested outlines; corner rounding for `βn`; utilisation `η(t)` curve under the viewport; cladding (gypsum) with `tf` marker; connection fire reduction visual.
- **V-T5 Floor vibration**: 3D floor plate with joists; first-mode shape `φ(x,y) = sin(πx/ℓ) sin(πy/b)` (plate-beam with orthotropic ratio) animated at `f1` (time-scaled) with amplitude colour; modal mass; unit-point-load deflection bowl (`w(1 kN)` contour) and `v` response; frequency criterion band chart (`f1` vs 6 Hz / 8 Hz thresholds).
- **V-T6 Diagrams (2D)**: `kc` vs `λrel` buckling curve with the member's operating point; `kcrit` vs `λrel,m` curve; `kmod` and `kdef` look-up tables as legend; `fh` vs angle `α`; Johansen mode chart (`Fv,Rk` per mode vs `t1/d`) with the current design as marker; `nef` vs `a1/d`; γ-method stress and shear diagrams; creep `u(t)`; `dchar(t)`.
- **V-T7 Assembly views**: wall diaphragm (racking, hold-down forces), truss with bracing, CLT panel with layup exploded view and rolling-shear stress distribution, mechanically jointed beam with slip and stress per part.
- **V-T8 Bridge**: deck with stress-laminated planks and prestress, pedestrian crowd mode animation, fatigue damage heat map.

## 10. Masonry visualisations

- **V-M1 Wall 3D**: parametric wall with openings (window/door cutouts with lintels), pier highlighting, floors (slab bearing depth shown), ring beams; bond pattern by explicit unit generation (stretcher, header, English, Flemish, running; courses of height `hu + tbed`) with perpends overlap `lol` and invalid overlap (< 0.4 hu or < 45 mm) coloured; mortar joints as inset geometry; instancing for performance (≥ 50 k units LOD, switching to textured planes beyond distance).
- **V-M2 Eccentricity / `Φ` diagrams along height**: elevation of the wall with side plots: `e(z)` (top `ei`, mid `emk`, bottom `ei`), `Φ(z)`, `NEd(z)` vs `NRd(z)` envelope, governing sections marked; slenderness `hef` graphic (buckling shape, restraint types, `ρn`).
- **V-M3 Lateral load patterns**: yield-line pattern drawn on the wall (hinge lines from edge conditions and `μ`), moment contour `MEd1, MEd2`, arching thrust line; wind pressure arrows; utilisation shading on the wall face; supports hatched by type.
- **V-M4 Shear walls**: compressed zone `lc` stress block, flange effective widths shaded, principal-stress / failure mode (rocking, sliding, diagonal tension) icons, overturn line.
- **V-M5 Concentrated load**: spreading 60° zone / `lefm`, bearing area `Ab`, `β` annotation; local crushing region.
- **V-M6 Reinforced masonry**: section with bars, strain diagram, `M-N` interaction curve with operating point; anchorage length.
- **V-M7 Cavity wall**: both leaves, tie positions with count/m² overlay, tie force heat map.
- **V-M8 Fire**: REI tabulated chart `t_min` vs R-class with the wall marked; plaster equivalent thickness.
- **V-M9 Parameter diagrams**: `fk` surface vs `fb` and `fm` (formula vs table points), `Φ` vs `e/t` for different `λ`, `fvk` Mohr-Coulomb line with `0.065 fb` cap.

---

# PART D. Realtime editor

## 11. Interaction model

All editors are event-sourced: a gesture produces a command, the command a (small) set of events, the projection re-evaluates affected checks. Optimistic local apply; no blocking on network (local-first). Every expensive operation (sweeps, full re-verify, 3D regeneration for large walls) reports progress and is cancelable (new edit cancels in-flight stale evaluation: coalescing by `generation` counter).

### 11.1 Timber editor

- Direct manipulation in the 3D viewport: drag member end-handles (span), section handles (b, h with snap to commercial sizes `20 mm`/lamella `40 mm`), taper angle handle, apex radius/pitch handles, camber handle, notch/hole placement with drag and numeric field overlay, restraint markers snap to member nodes, load arrows draggable along members (positions in mm), fastener pattern: place first fastener, drag the pattern grid handle (rows × columns) with live spacing readout and traffic-light snap (green ≥ min distance, amber ≥ 0.9 min, red < min); edge/end distance handles; steel plate polygon vertex dragging; connection type switch (nail → bolt → screw) keeps pattern; "nearest valid size" suggestions.
- Inline utilisation: each member/connection gets a utilisation badge (max `η`), colour ramp (≤ 0.5 blue-green, 0.5–0.8 green, 0.8–1.0 amber, > 1 red); hover shows the check table with formula/substitution; click opens the step-by-step derivation (formula and values, EN+DE, clause link).
- Remedies: for `η > 1` the editor proposes ranked fixes computed by sweeping (cancellable): next larger `b,h` that passes, upgrade strength class (C24 → C30, GL24h → GL28h), add lateral restraint, shorten buckling length, increase bearing length, add reinforcement at notch/apex, increase fastener count/diameter, shift to screw reinforced; each remedy previews utilisation deltas and applies as one event.
- Panels: `Inputs` (grouped forms with typed fields and units, field-meta from schema), `Results` (table + diagrams), `Annex/NDP` (every NDP shown with EN/DE value and switch, override flagged), `Loads/Combinations`, `Fire` (time slider), `Vibration`, `Report` (printable). No raw JSON editors; JSON import/export stays as I/O.
- Keyboard accessibility: all handles operable via keyboard (arrow keys = 1 unit, shift = 10); screen-reader labels from `LocalizedLabel`; high-contrast ramp; language switch (EN first, DE second; no default language selected silently: onboarding picks).

### 11.2 Masonry editor

- Direct: draw wall footprint polyline, drag wall height/thickness handles (snap to unit module multiples), drag openings (window/door) with snap to brick module (`125/250 mm` grid, `m_o = n·lu + joint`), edge support selector on wall edges (click an edge → cycle free/pinned/restrained), floor bearing depth handle, concentrated-load footprints drag, reinforcement bar placement, cavity ties auto-layout with override.
- Material pickers: unit type (material, group, class) + mortar class: instant `fk` readout with derivation (`K fb^α fm^β` or NA table row highlighted), `γM` by execution class.
- Live outputs: `Φ(top/mid/bottom)` gauges, `NRd` vs `NEd` bar, slenderness, `ρn` explanation; changing bond/unit length updates overlap check live.
- Remedies: thicker wall/higher unit strength/stronger mortar/add stiffening wall/shorten opening/add lintel, shown as ranked, previewed.

### 11.3 Live recompute targets (performance budget, desktop mid-range, in-process kernel)

| Interaction | Budget p95 | Notes |
|---|---|---|
| single member all checks (T-10…T-65) | < 2 ms | pure closed-form |
| connection (≤ 50 fasteners) full check | < 5 ms | nef, modes, splitting, block shear |
| connection geometry drag (spacing violation tinting) | 1 frame (16 ms) | incremental |
| 3D mesh regeneration on drag (member, tapered/curved) | < 8 ms | cached lamella profile |
| glulam apex / notch recompute | < 2 ms | |
| γ-method / CLT shear analogy (≤ 9 layers) | < 3 ms | |
| fire time sweep 0–120 min (120 steps) | < 15 ms | |
| vibration f1 + w + v | < 3 ms; 3D mode shape update < 16 ms | |
| masonry wall full checks (M-10…M-34) | < 3 ms | |
| bond pattern regeneration (≤ 50 000 units) | < 30 ms with instancing; progress above | |
| yield-line / α table lookup + moments | < 1 ms | |
| full model (≤ 200 members/walls) re-verify | < 150 ms, progress + cancel | dependency-driven partial |
| remedy sweep (≤ 200 candidates) | < 250 ms, progress + cancel | |

Recompute is dependency-driven: each verification declares its input paths; edits invalidate only checks whose declared inputs changed (static input-path index built from the schema). Stale results are never shown unlabelled: UI shows "computing" shimmer for checks older than the current generation.

### 11.4 Collaboration (multi-user)

- Presence: cursor/selection per user on the 3D scene (already `presence` schema in artifact); ephemeral-shared state for selection and drag previews, persisted-shared for events, local-only for view settings. Concurrent edits to different entities merge; same-entity conflict resolved by event ordering (last event wins with visible attribution); no CRDTs.

### 11.5 Accessibility/internationalisation

- Every label and message with `LocalizedLabel::native(en, de)`; formula steps carry both languages for names; number formatting by locale; alternative text for 3D (textual member/connection tree with utilisations, keyboard navigable); reduced-motion option disables fire/vibration animation (static frames).

---

# PART E. Testing and oracles

## 12. Test strategy

- Language-agnostic test format: `📚️examples/<id>/{input.json, expected.json}` where `expected` has `{check_id → {demand, capacity, utilisation, status}}` with tolerance `1e-6` relative (formula checks) or `5e-3` (table-interpolated). One fixture runner per implementation language (Rust kernel first; TS viewer parity).
- Third-party validation (Python oracles in `🔮️oracles`): `numpy/scipy` for closed forms (Johansen modes by independent implementation, γ-method, eigen analysis for vibration via `scipy.linalg.eigh` of a plate-beam FE model, yield-line upper bound via `scipy.optimize`, arch statics), and where available published reference calculators/tables: `eurocodepy` (EC5 `kmod`, `kh`, `kc` helpers) and `handcalcs`-style reproduction of the textbook examples; each oracle output is stored with the source and compared in CI. Where no library exists, the oracle is an independent re-derivation reading the same table data file.
- Property tests: monotonicity (η decreases with larger section/class), unit-scaling invariance, `kc ≤ 1`, `kcrit ≤ 1`, `Φ ∈ [0,1]`, mode min of Johansen is envelope, `nef ≤ n`.
- Annex switch test: same input with `annex=En` vs `De` yields NDP-dependent differences exactly where listed (kcr, γM, vibration limits, fk table vs formula).
- Visual tests: scene-graph snapshot assertions (counts, bounds, colour mapping), no pixel diffs.
- Regression: the four current example sets (`glulam-floor-beam`, `glulam-footbridge`, `multi-fail-timber`, `overloaded-footbridge`, `loadbearing-wall`) are rewritten into the new schema by hand and must keep their pass/fail verdicts per check.

## 13. Worked examples (hand-checked numbers)

Numbers computed independently (closed form, double-checked in Python at spec time; the kernel and the oracle must reproduce them to the stated precision). Units N, mm unless stated; DE annex, γG = 1.35, γQ = 1.5.

### EN 1995 examples (E1–E10)

**E1 Glulam floor beam GL28h 200 × 400, span 5.0 m, SC1 (bending, shear, deflection).** `gk = 2.5 kN/m, qk = 3.0 kN/m` (cat. A, medium). `qd = 1.35·2.5 + 1.5·3.0 = 7.875 kN/m`; `Md = qd L²/8 = 24.609 kNm`; `Vd = 19.6875 kN`. `W = 200·400²/6 = 5.3333·10⁶ mm³`; `σm,d = 4.614 N/mm²`. `kh = min((600/400)^0.1, 1.1) = 1.0414`; `kmod = 0.80` (medium); `γM = 1.25`; `fm,d = 0.8·1.0414·28/1.25 = 18.66 N/mm²`; `η_M = 0.2473`. Shear: `kcr = 2.5/3.5 = 0.7143` (NA-DE); `τd = 1.5·19 687.5/(0.7143·200·400) = 0.5168 N/mm²`; `fv,d = 0.8·3.5/1.25 = 2.24`; `η_V = 0.2307`. SLS: `I = 1.0667·10⁹ mm⁴`, `E0,mean = 12 600`: `winst,G = 1.514 mm`, `winst,Q = 1.817 mm`; `kdef = 0.6`: `wfin = 1.514·1.6 + 1.817 = 4.239 mm` (creep applied to G only, i.e. `ψ2 = 0`; with `ψ2 = 0.3` the Q term becomes `1.817·(1 + 0.3·0.6)`); limits `L/300 = 16.7 mm` for `winst`, `L/200 = 25 mm`.

**E2 Solid timber column C24 100 × 100, L = 3.0 m pinned, SC2 (buckling `kc`).** `i = 100/√12 = 28.87 mm`; `λ = 103.92`; `λrel = (103.92/π)√(21/7400) = 1.7622`; `βc = 0.2`; `k = 0.5(1 + 0.2(1.7622 − 0.3) + 1.7622²) = 2.1989`; `kc = 1/(2.1989 + √(2.1989² − 1.7622²)) = 0.28457`; `fc,0,d = 0.8·21/1.3 = 12.923 N/mm²` (medium-term); `σc,0,d,Rd = kc fc,0,d = 3.677 N/mm²` ⇒ `NRd = 36.77 kN` (A = 10 000 mm²).

**E3 Bearing perpendicular `kc,90`.** C24 solid, continuous support, `fc,90,k = 2.5`, `kmod = 0.8`, `γM = 1.3`, `kc,90 = 1.25`: `kc,90 fc,90,d = 1.25·0.8·2.5/1.3 = 1.923 N/mm²`. GL28h discrete (as per NA conditions `l ≤ 400`), `kc,90 = 1.5`, `fc,90,k = 3.0`: `1.5·0.8·3.0/1.25 = 2.880 N/mm²`.

**E4 Lateral-torsional buckling GL28h 120 × 360, `lef = 8.0 m`.** `σm,crit = 0.78 b² E0,05/(h lef) = 0.78·120²·10 500/(360·8000) = 40.95 N/mm²`; `λrel,m = √(28/40.95) = 0.8269`; `kcrit = 1.56 − 0.75·0.8269 = 0.9398`. Same section with `lef = 3.0 m`: `σm,crit = 109.2`, `λrel,m = 0.5064 ≤ 0.75` ⇒ `kcrit = 1.0`.

**E5 Notch at support, C24 120 × 240, `hef = 200`, `x = 100`, `i = 0`, `Vd = 10 kN`, SC1, medium.** `α = 0.8333`; `kn = 5`; `kv = 5/(√240·(√(0.8333·0.1667) + 0.8·(100/240)·√(1/0.8333 − 0.8333²))) = 0.5294`; `τd = 1.5·10 000/(120·200) = 0.625 N/mm²`; resistance `kv fv,d = 0.5294·0.8·4.0/1.3 = 1.3032 N/mm²` (`fv,d = 2.4615`); `η = 0.625/1.3032 = 0.4796` (equivalently `τd/kv = 1.1807`, `η = 0.4796`).

**E6 Dowel/bolt Johansen, single shear timber–timber C24/C24, 60/60, M12 (fu,k = 400), SC1, kmod = 0.8.** `ρk = 350`: `fh,k = 0.082·(1 − 0.12)·350 = 25.256 N/mm²`; `My,Rk = 0.3·400·12^2.6 = 76 745 Nmm`; `β = 1`. Mode (a)=(b) `18 184 N`; (c) `7 532 N`; (d)=(e) `7 643 N` (without rope effect); (f) `7 844 N` ⇒ `Fv,Rk = 7.532 kN` (mode c); `Fv,Rd = 0.8·7.532/1.3 = 4.635 kN` per shear plane. Steel-timber thick plate single shear, same timber: (c) `18 184`, (d) `9 282`, (e) `11 092` ⇒ `Fv,Rk = 9.282 kN`, `Fv,Rd = 5.712 kN`.

**E7 nef bolt row.** M12, `a1 = 90 mm`, four bolts in a row parallel to grain: `nef = min(4, 4^0.9·(90/(13·12))^0.25) = 3.482·0.8715 = 3.035`; two such rows (the repo's 8-bolt group): `nef,total = 6.07`; with `a1 = 60 mm` (5d): `nef = 3.482·(60/156)^0.25 = 2.742`; 8 bolts in one row `a1 = 120`: `6.085`.

**E8 Axially loaded screw, d = 8 mm, `lef = 80 mm`, `ρk = 350`, perpendicular to grain.** `fax,k = 0.52·8^−0.5·80^−0.1·350^0.8 = 12.865 N/mm²`; `kd = 1`; `Fax,Rk = 12.865·8·80 = 8.233 kN`; `Fax,Rd = 0.8·8.233/1.3 = 5.067 kN`; group of 4: `nef = 4^0.9 = 3.482`.

**E9 Fire, glulam GL28h 160 × 320, R60, 3-sided (bottom + two sides), reduced cross-section.** `βn = 0.7`, `dchar,n = 42 mm`, `k0 = 1`, `d0 = 7`, `def = 49 mm`; residual `b = 160 − 98 = 62 mm`, `h = 320 − 49 = 271 mm`; `W = 62·271²/6 = 758 890 mm³`; `kfi = 1.15`, `kmod,fi = 1`, `γM,fi = 1`; `fm,fi,d = 1.15·28 = 32.2`; `kh = (600/271)^0.1 = 1.0827`; `Mfi,Rd = 758 890·32.2·1.0827 = 26.46 kNm`. R30 version: `dchar,n = 21`, `def = 28`, `b = 104`, `h = 292`.

**E10 Mechanically jointed beam γ-method (three boards 200 × 50, nailed Ø3.1, s = 150 mm, ℓ = 3.0 m, SLS).** `E = 11 000`; `Kser = ρm^1.5 d^0.8/30 = 350^1.5·3.1^0.8/30 = 539.6 N/mm`; `γ1 = γ3 = 1/(1 + π²·11 000·10 000·150/(539.6·3000²)) = 0.02896`; `a1 = a3 = 50 mm`, `a2 = 0`; `(EI)ef = 3·E·I1 + 2γ E A a² = 68.75 + 15.93 = 84.68 kNm²` (rigid `618.75 kNm²`, ratio 0.137).

**E11 Floor vibration (timber joist floor, 80 × 220 C24 at 625 mm, span 4.5 m, `m = 150 kg/m²`).** `(EI)ℓ = 11 000·(80·220³/12)/625 mm → 1.249·10⁶ Nm²/m`; `f1 = π/(2·4.5²)·√(1.249·10⁶/150) = 7.08 Hz`: fails EN recommended 8 Hz; passes the NA-DE `f1 ≥ 6 Hz` stage (then stiffness criterion required) **[V]**.

**E12 Curved/pitched apex (check structure):** for the oracle only, 20 m glulam pitched cambered beam, values from the DIN EN 1995-1-1/NA example collection; includes `kl`, `kp`, `kdis`, `kvol` hand-computed by the oracle and stored as fixture (**[V]**).

### EN 1996 examples (X1–X10)

**X1 `fk` for clay Group 1 / GP mortar M10.** `fb = 20 N/mm²`, `fm = 10`, `K = 0.55`: `fk = 0.55·20^0.7·10^0.3 = 8.935 N/mm²` (cap `0.8 fb = 16` not governing); `γM = 1.5` ⇒ `fd = 5.957 N/mm²`; `E = 1000 fk = 8935 N/mm²`; `Elt = E/(1 + 1.5) = 3574 N/mm²` (φ∞ = 1.5).

**X2 Effective height and slenderness.** Wall `t = 0.365 m`, `h = 2.75 m`, both ends restrained by concrete slabs, `ρ2 = 0.75`: `hef = 2.0625 m`, `hef/t = 5.65`; `einit = hef/450 = 4.583 mm`. 3-sided (L = 3.0 m): `ρ3 = 0.75/(1 + (0.75·2.75/9)²) = 0.7126`; 4-sided (L = 3.0): `ρ4 = 0.75/(1 + (0.75·2.75/3)²) = 0.5093`.

**X3 `Φ` at top (continuation of X1/X2).** `ei = 0.020 + 0.004583 = 0.024583 m ≥ 0.05t = 0.01825`; `Φ_top = 1 − 2·0.024583/0.365 = 0.8653`; `NRd,top = 0.8653·0.365·5.957 = 1881 kN/m`.

**X4 `Φm` mid-height (X1 wall).** `φ∞ = 1.5`; `em = 0.0125 + 0.004583 = 0.017083 m`, `ek = 0.002·1.5·5.65·√(0.365·0.017083) = 1.34·10⁻³`; `emk = max(em + ek, 0.05 t) = 0.01825 m`; `A1 = 1 − 2·0.01825/0.365 = 0.9`; `λ = 5.65·√(8.935/8935) = 0.17869`; `u = (0.17869 − 0.063)/(0.73 − 1.17·0.05) = 0.17229`; `Φm = 0.9·exp(−0.17229²/2) = 0.8867`; `NRd,mid = 1928 kN/m` (top governs at 1881 kN/m). Check `NEd = 200 kN/m` ⇒ `η = 0.106`.

**X5 Concentrated load.** Wall `t = 0.365`, beam bearing `0.2 × 0.365 m` (`Ab = 0.073 m²`), `a1 = 0.5 m`, `hc = 2.75`, `Aef = lefm t = 1.0·0.365`: `β = (1 + 0.3·0.5/2.75)(1.5 − 1.1·0.073/0.365) = 1.3498`, cap `min(1.25 + 0.5/(2·2.75), 1.5) = 1.3409` ⇒ `β = 1.3409`; `NRdc = 1.3409·0.073·5.957·10³ = 583.1 kN`.

**X6 Shear wall.** Same masonry, `l = 2.0 m`, `σd = 0.8 N/mm²`, `fvk0 = 0.30`: `fvk = 0.30 + 0.4·0.8 = 0.62 N/mm²` (cap `0.065 fb = 1.30` not governing); `fvd = 0.4133`; `VRd = 0.4133·365·2000 = 301.7 kN` (assuming full length compressed, `lc = l`; with e = M/N reduce `lc = 3(l/2 − e)`: e = 0.2 m ⇒ `lc = 2.4 > l` ⇒ full).

**X7 Laterally loaded one-way wall.** `t = 0.175`, `h = 2.75`, pinned–pinned, wind `0.64 kN/m²` ⇒ `wEd = 1.5·0.64 = 0.96 kN/m²`; `MEd = 0.96·2.75²/8 = 0.9075 kNm/m`; `fxk1 = 0.2 N/mm²`, `γM = 1.5` ⇒ `fxd1 = 0.1333`; `Z = 0.175²/6 = 5.104·10⁻³ m³/m`; `MRd = 0.6806 kNm/m` ⇒ `η = 1.333` (fail; with `σd = 0.2 N/mm²` apparent strength `fxd1 + σd = 0.333` ⇒ `MRd = 1.70 kNm/m`, `η = 0.534`).

**X8 Two-way slab check (yield-line oracle).** Isotropic (μ = 1) square panel simply supported on four edges: yield-line `m = w l²/24` ⇒ `α = 0.04167`; the kernel's table-based `α` must be ≥ this elastic/plastic bound consistency check and match the oracle for the table cases (**[V]** table values).

**X9 Reinforced masonry flexure.** `b = 1000`, `d = 300`, `As = 600 mm²/m`, `fyd = 435`, `fd = 5.957`: `z = 300·(1 − 0.5·600·435/(1000·300·5.957)) = 278.1 mm ≤ 0.95 d = 285`; `MRd = 600·435·278.1 = 72.58 kNm/m`.

**X10 EN 1996-3 simplified method.** `t = 240 mm`, `h = 2.75 m`, `ρ2 = 0.75` ⇒ `hef = 2.0625`, `hef/t = 8.594`; `Φs = 0.85 − 0.0011·8.594² = 0.7688` **[V]**; `fb = 14`, `fm = 5`, `K = 0.55` ⇒ `fk = 5.654 N/mm²`; `fd = 3.769`; `NRd = 0.7688·0.240·3.769·10³ = 695.4 kN/m`.

**X11 Basement wall minimum normal force (NA-DE/EN 1996-3).** `t = 0.365`, `h = 2.5 m`, `he = 2.0 m`, `ρe = 18 kN/m³`: `Nmin = ρe h he²/(20 t) = 18·2.5·4/(20·0.365) = 24.66 kN/m` **[V]**.

**X12 Fire tabulated:** clay Group 1 wall `t = 365 mm`, R90 required, `μfi = 0.7`: `t_min ≤ 175 mm` by Annex data ⇒ pass; fixture asserts only that `t ≥ t_min(table)`; the table row is pinned from the NA by the oracle (**[V]**).

## 14. Delivery order

1. Shared kernel: `CheckResult`, `NdpSet`, data tables (`KmodTable`, `KdefTable`, strength classes, `KTable`, `AlphaTable`, `FireTable`), dependency index.
2. EN 1995 phase 1: T-01…T-24, T-45, T-60…T-65, T-70…T-77, T-90, T-100…T-102; viewers V-T1, V-T3, V-T4, V-T6.
3. EN 1996 phase 1: M-01…M-14, M-20, M-30, M-31, M-81, M-90; viewers V-M1, V-M2, V-M3.
4. Phase 2: curved/pitched (T-30…T-33), CLT (T-97), diaphragms (T-94), bridges (T-120…T-123), reinforced masonry, cavity/ties, advanced fire.
5. Editors: the interaction layer in section 11 per phase; JSON-as-UI removed per artifact when its editor lands.

## 15. Open verification items ([V] summary)

Table transcription and NA-DE number pinning needed before implementation (each has an oracle test): strength class tables (EN 338 vs EN 14080 editions), `kmod` for panels, `kdef` panels, load-duration assignment (NA Table NA.1), `kc,90` NA conditions, NA-DE `kcr` (formula `2.5/fv,k` to confirm), notch limits and reinforcement formula, vibration NA-DE limits (`f1`, `a`, `b`), serviceability deflection limits NDP, split ring/shear plate/toothed plate tables, nail plate approvals, CLT NA-DE (`γM`, `fr,k`, shear analogy), charring table values and cladding formulas, fire tabulated masonry values, masonry `K` table, NA-DE `fk`/`fxk`/`fvk0` tables and `E` rule, `ρn` NA values, arching formula coefficient, `Φ1/Φ2/Φ3` in EN 1996-3/NA, simplified-method applicability limits, tie densities (NA Table), basement-wall limits, chase tables, `δ` shape factors. No value marked [V] may be hard-coded before it is verified against the standard text and cross-checked with the oracle.
