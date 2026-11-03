# 📐 EN 1997 and EN 1998 Target Specification

Ticket: `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` (goal `norm`). Scope: `🌍️en1997` (parts 1, 2) and `🫨️en1998` (parts 1-6) including DIN EN 1997-1/NA + DIN 1054 and DIN EN 1998-1/NA. Status: target specification only, no source changed.

## 0. Conventions and Honesty Notes

- Units: SI, kN, kPa, m, degrees for angles in UI, radians internal; stress in kPa, forces in kN (schema fields keep `#[dsl(unit)]`; existing snapshot uses N and Pa, the target keeps N/Pa at the schema boundary and converts to kN/kPa in the UI only).
- Subscripts: `k` characteristic, `d` design, `m` mean, `dst`/`stb` destabilising/stabilising.
- Every catalogue entry has the fixed shape: id, clause, name EN / DE, inputs, formulas, NDPs (EN recommended / NA-DE), outputs, criterion, edge cases.
- NDP values marked `[V]` are transcribed from the author's knowledge of the NA/DIN text and MUST be re-verified against the licensed edition before the fixture is frozen; every fixture cites the edition (`DIN 1054:2010-12`, `DIN EN 1997-1/NA:2010-12`, `DIN EN 1998-1/NA:2011-01`). A `[V]` value never silently defaults: it lives in the NDP table asset and is overridable in the editor.
- Numbers in section 7 were computed with a standalone Python script (stdlib `math` only) and are the hand-check references; the Python oracle (`🔮️oracles`) recomputes them with an independent implementation (scipy/numpy where available, plain math otherwise) and compares at relative tolerance 1e-6 (closed form) or 1e-3 (iterative).
- Existing vocabulary kept: `En1997Snapshot { structure_id, geotechnical_category, design_situation, design_approach, annex: AnnexChoice, groundwater_level, investigation_depth, layers: SoilLayer[], footings: SpreadFoundation[], piles: Pile[], retaining_walls: RetainingWall[], slopes: Slope[], uplift_cases: UpliftCase[] }` and `En1998Snapshot { annex, site: En1998Site{seismic_zone: DeSeismicZone, a_gr, de_ground_combo: DeGroundCombo, en_ground_type, en_spectrum_type, importance_class}, buildings, bridges, assessments, silos, tanks, foundations, retaining_walls, towers }`. Target model extends these; no compatibility layer, fixtures are rewritten by hand.

## 1. EN 1997 Partial Factor System (shared by all EN 1997 checks)

### 1.1 Limit states and design situations

| Key | EN | DE |
|---|---|---|
| EQU | loss of static equilibrium | Lagesicherheit (GZ 1A) |
| UPL | uplift by water pressure | Auftrieb (GZ 1A) |
| HYD | hydraulic heave / piping | Hydraulischer Grundbruch (GZ 1A) |
| STR | structural failure | Versagen von Bauteilen (GZ 1B) |
| GEO | ground failure | Versagen des Baugrunds (GZ 1B bearing/sliding/pile; GZ 1C slope/overall) |
| SLS | serviceability | Gebrauchstauglichkeit (GZ 2) |
| BS-P / BS-T / BS-A | persistent / transient / accidental | ständig / vorübergehend / außergewöhnlich |

Geotechnical category GC 1/2/3 (EN 1997-1 §2.1, DE: DIN 4020 / DIN EN 1997-2 Table NA): GC drives minimum investigation (section 2.12) and mandatory checks; GC 3 adds mandatory independent check flag.

### 1.2 Design approaches

| Approach | Combination | Actions | Materials | Resistances | Applies |
|---|---|---|---|---|---|
| DA1-C1 | A1 + M1 + R1 | A1 | M1 | R1 | STR/GEO pile, anchor, footing |
| DA1-C2 | A2 + M2 + R1 | A2 | M2 | R1 | slopes, footing GEO |
| DA2 | A1 + M1 + R2 | A1 | M1 | R2 | footing, pile, anchor, wall |
| DA2* | A1 + M1 + R2 with factoring of action effects (E) not actions | as DA2 but factor the effect of actions | M1 | R2 | embedded walls/slopes where factoring ground actions is unnatural |
| DA3 | (A1 structural / A2 geotechnical) + M2 + R3 | A1* or A2 | M2 | R3 | slopes, overall stability |
| DE | DA2 for footings/sliding/piles/anchors (GZ 1B), DA3-like with GZ 1C for slopes/overall stability and slope-failure-like limit states | | | | DIN 1054 3.? |

DE mapping: GZ 1A = EQU/UPL/HYD; GZ 1B = STR + GEO bearing/sliding/pile/anchor, equals DA2 (A1+M1+R2) with the resistance partial factors of Table A 2.3; GZ 1C = overall stability and slope (soil-parameter reduction, partial factor 1.0 on permanent actions, equals DA3 on soil parameters but with action factors per Table A 2.1). DA2* is selectable by the editor (`design_approach`), default DE = DA2 on GZ 1B.

### 1.3 Partial factor tables (selected by `annex` + `design_situation`)

Table A2.1 (actions, DIN 1054:2010 Table A 2.1) `[V]`:

| Set | Action | BS-P | BS-T | BS-A |
|---|---|---|---|---|
| GZ 1B | γG unfavourable | 1.35 | 1.20 | 1.10 |
| GZ 1B | γQ unfavourable | 1.50 | 1.30 | 1.15 |
| GZ 1B | γG,fav / γQ,fav | 1.00 / 0 | 1.00 / 0 | 1.00 / 0 |
| GZ 1A EQU | γG,dst | 1.05 | 1.05 | 1.00 |
| GZ 1A EQU | γG,stb | 0.95 | 0.95 | 0.95 |
| GZ 1A EQU | γQ,dst | 1.50 | 1.30 | 1.15 |
| GZ 1A UPL | γG,dst / γG,stb | 1.00 / 0.90 | 1.00 / 0.90 | 1.00 / 0.95 |
| GZ 1A UPL | γQ,dst | 1.50 | 1.30 | 1.15 |
| GZ 1A HYD | γH (seepage force) | 1.80 | 1.45 | 1.30 |
| GZ 1A HYD | γG,stb | 0.90 | 0.90 | 0.95 |
| GZ 1C | γG | 1.00 | 1.00 | 1.00 |
| GZ 1C | γQ unfavourable | 1.30 | 1.20 | 1.10 |

Table A2.2 (soil parameters, only GZ 1C and DA3) `[V]`:

| Parameter | BS-P | BS-T | BS-A |
|---|---|---|---|
| γφ' (on tanφ') | 1.25 | 1.15 | 1.10 |
| γc' | 1.25 | 1.15 | 1.10 |
| γcu | 1.25 | 1.15 | 1.10 |
| γqu | 1.25 | 1.15 | 1.10 |
| γγ | 1.00 | 1.00 | 1.00 |

Table A2.3 (resistances, GZ 1B) `[V]`:

| Resistance | BS-P | BS-T | BS-A |
|---|---|---|---|
| bearing γR,v | 1.40 | 1.30 | 1.20 |
| sliding γR,h | 1.10 | 1.10 | 1.10 |
| passive earth γR,e | 1.40 | 1.30 | 1.20 |
| pile compression γb = γs = γt (tests, tables) | 1.10 | 1.10 | 1.10 |
| pile tension γs,t | 1.15 | 1.15 | 1.15 |
| anchor (grouted) γa | 1.10 | 1.10 | 1.10 |
| anchor/pile structural pull-out via test γa | 1.10 | 1.10 | 1.10 |

EN 1997-1 recommended: A1 (γG 1.35, γQ 1.5), A2 (γG 1.0, γQ 1.3), M1 (all 1.0), M2 (γφ' 1.25, γc' 1.25, γcu 1.4, γqu 1.4, γγ 1.0), R1 (all 1.0), R2 bearing 1.4, sliding 1.1, R3 bearing 1.0, sliding 1.0; pile driven R1 γb 1.0 γs 1.0, R2 γb 1.1 γs 1.1 γt 1.1, R3 γb 1.0 γs 1.0 (with M2 on soil), R4 γb 1.3 γs 1.3 γt 1.3; EQU recommended γG,dst 1.1, γG,stb 0.9, γQ,dst 1.5; UPL γG,stb 0.9, γG,dst 1.0, γQ,dst 1.5; HYD γG,stb 0.9, γG,dst 1.35, γQ,dst 1.5.

Implementation: asset `partial-factors.json` (schema-first, `PartialFactorTable { annex, set_id, situation: BS, values }`), looked up by `(annex, limit_state, design_approach, situation)`. Editor shows the applied factors read-only next to every check and lets the user override (marks result `non-standard`).

## 2. EN 1997 Verification Catalogue

Notation: all catalogue items return `CheckResult { id, clause, value_ed, value_rd, utilisation μ = Ed/Rd (or Rd-ratio form), status pass|fail|n/a, remedies[] }`.

### 2.1 Soil layering and strata (G-01)

- Clause: EN 1997-1 §2.4.3, §3, DIN 4020; DIN 4023 (Darstellung). EN: ground model, strata; DE: Baugrundmodell, Schichtenfolge.
- Inputs: layers `i=1..n` with `z_top,i`, `z_bot,i` [m], soil class (DIN 18196 symbol GW, SE, TL, UM, …), γ, γ' [kN/m³], φ'k [°], c'k, cu,k [kPa], Eoed [MPa], ν [-], qc [MPa], N_SPT, groundwater level `z_w` [m], optional per-layer `k` [m/s], OCR, e0, Cc, Cs, cv [m²/s].
- Formulas: contiguity `z_bot,i = z_top,i+1`; thickness `h_i = z_bot,i - z_top,i`; total stress `σv(z) = Σ γ_i,eff h_i`, with γ_i,eff = γ above z_w and γ_sat below; `u(z) = γw (z - z_w)` for z>z_w (γw = 10 kN/m³ DE, 9.81 recommended allowed via NDP `gamma_w`); `σ'v = σv - u`.
- NDP: γw 9.81 (EN) / 10.0 (DE practice) `[V]`.
- Outputs: stress profile arrays, equivalent layer for a depth range (thickness-weighted tanφ, c).
- Criterion: gap/overlap-free, h_i>0, γ' < γ, φ' in [0,50], cu>=0, Eoed>0, investigation_depth >= max required depth (section 2.12).
- Edge: groundwater inside a layer splits it virtually; perched water table (second z_w by layer range); artesian head (separate `head_at_layer` > z_w); layers extending past investigation depth; zero thickness.

### 2.2 Characteristic values (G-02)

- Clause: EN 1997-1 §2.4.5.2; EN 1997-2 §2.4.5.2 / Annex C (statistics); DE: charakteristische Bodenkennwerte (DIN 1054 A 2.4.?, vorsichtiger Schätzwert).
- Inputs: sample set `x_1..x_n`, `n`, optional `V_x` (known coefficient of variation), mode `mean|local`, approach `cautious estimate | statistical`.
- Formula (statistical, 5 % fractile of the mean, large failure volume): `X_k = X_m (1 - k_n V_x)` with `k_n = t_{0.95,n-1} sqrt(1/n)` (V_x unknown, estimated from the sample) or `k_n = 1.645 sqrt(1/n)` (V_x known); local/small-volume failure (governed by lowest values): `X_k = X_m (1 - k_n V_x)` with `k_n = t_{0.95,n-1} sqrt(1 + 1/n)` (or `1.645 sqrt(1 + 1/n)`). Sample statistics: `X_m = Σx/n`, `s = sqrt(Σ(x-X_m)²/(n-1))`, `V_x = s/X_m`.
- Table of t_{0.95,ν}: ν=1 6.314, 2 2.920, 3 2.353, 4 2.132, 5 2.015, 6 1.943, 7 1.895, 8 1.860, 9 1.833, 10 1.812, 20 1.725, ∞ 1.645.
- Outputs: X_k, k_n, flags `cautious_override` when user enters value directly (needs note).
- Criterion: informational; warn when n<3, V_x>0.3.
- Edge: n=1 (no statistics, forces cautious input), bimodal data, negative X_k clipped to 0, anisotropy.

### 2.3 Bearing resistance of spread foundations, EN 1997-1 Annex D (GEO-F01 drained, GEO-F02 undrained, GEO-F03 DIN 4017 variant)

- Clause: EN 1997-1 §6.5.2, Annex D.3/D.4; DE: DIN EN 1997-1 + NA §6.5.2, DIN 4017:2006 (Grundbruch). EN: Bearing resistance; DE: Grundbruchnachweis.
- Inputs: B, L [m], embedment d [m], base inclination α [°] (positive tilted up), design loads `V_d [kN]` (incl. footing weight and backfill), `H_B,d`, `H_L,d` [kN], `M_B,d`, `M_L,d` [kNm], soil `φ'd, c'd, cu,d, γ1 (below base), γ2 (above base, q' = γ2 d), γw`, groundwater (use γ' below water).
- Eccentricity: `e_B = M_B/V`, `e_L = M_L/V`. Effective area (D.1): `B' = B - 2e_B`, `L' = L - 2e_L` (A' = B'L' for rectangular; Meyerhof; circular footing: ellipse area, use A' by intersection of circle R with 2-eccentric rectangle) with `B' <= L'` swap if needed. Rect: `A' = B' L'`.
- Drained (D.4):
  `R/A' = c' Nc bc sc ic + q' Nq bq sq iq + 0.5 γ' B' Nγ bγ sγ iγ`
  `Nq = e^{π tanφ'} tan²(45° + φ'/2)`; `Nc = (Nq - 1) cotφ'`; `Nγ = 2 (Nq - 1) tanφ'` (rough base, δ >= φ'/2).
  Base inclination: `bq = bγ = (1 - α tanφ')²` (α in rad), `bc = bq - (1 - bq)/(Nc tanφ')`.
  Shape: rectangular `sq = 1 + (B'/L') sinφ'`, `sγ = 1 - 0.3 B'/L'`, `sc = (sq Nq - 1)/(Nq - 1)`; square/circle `sq = 1 + sinφ'`, `sγ = 0.7`.
  Inclination (H in direction of B): `m = mB = (2 + B'/L')/(1 + B'/L')`; direction of L: `mL = (2 + L'/B')/(1 + L'/B')`; combined `m = mB cos²θ + mL sin²θ`, θ = angle of H to L; `iq = [1 - H/(V + A' c' cotφ')]^m`, `iγ = [1 - H/(V + A' c' cotφ')]^{m+1}`, `ic = iq - (1 - iq)/(Nc tanφ')`.
  Depth factors (not part of Annex D; switchable `include_depth=false` default): Hansen `dq = 1 + 2 tanφ'(1-sinφ')² arctan(d/B)`, `dc = dq - (1-dq)/(Nc tanφ')`, `dγ = 1`.
- Undrained (D.3): `R/A' = (π + 2) cu bc sc ic + q`; `bc = 1 - 2α/(π+2)`; `sc = 1 + 0.2 B'/L'` (rect), `1.2` square/circle; `ic = 0.5 (1 + sqrt(1 - H/(A' cu)))`, `H <= A' cu`; `q = γ2 d` (total stress).
- DIN 4017 variant (GEO-F03, when `bearing_method = din4017`): `R_n,k = A' (c Nc sc ic + γ2 d Nd sd id + γ1 B' Nb sb ib)` with `Nd = Nq`, `Nb = (Nq-1) tanφ` (term `γ B Nb`), `sd = 1 + (B'/L') sinφ`, `sb = 1 - 0.3 B'/L'`, `sc = (sd Nd - 1)/(Nd - 1)`; inclination (tanδ = H/V): `id = (1 - 0.7 tanδ)³` (strip), `ib = (1 - tanδ)³`, `ic = (id Nd - 1)/(Nd - 1)` for φ>0; rect with `m` per Annex D formula.  [V] NA constants.
- DA2/GZ 1B check: `V_d <= R_d`, `R_d = R_k/γR,v`, `R_k = R/A' · A'`; DA1: C1 `M1,R1`, C2 `M2,R1` on tanφ', c', cu.
- Outputs: Nq, Nc, Nγ, all factors, R_k, R_d, μ = V_d/R_d, A', eccentricities, "inside core" flags `e_B <= B/6`, `e_B/B + e_L/L <= 1/6` double-sided: `(e_B/B)² + (e_L/L)² <= 1/9`.
- Pass/fail: `μ <= 1`, `e` inside the 1/3 rule (DE: gaping joint `e_B <= B/3` for permanent combos in BS-P, `<= B/2` for total in BS-T? `[V]`: DIN 1054 6.10: BS-P resultant within 1st core (e <= B/6), full load within 2nd core `e<=B/3`).
- Edge: φ'=0 limit switch to undrained form; `H > V tanδ` fail; `e_B >= B/2` => A'<=0 invalid; groundwater above base (γ' in Nγ term, q' reduced); layered ground below base (use equivalent layer to depth `B` or wedge: failure-depth `z_max ≈ B' tan(45+φ/2) e^{...}`; if weak layer below within z_max show `punching` warning); sloped ground/near slope (ß factor, `bq` uses slope β - DIN 4017 slope factors `λ = (1 - 0.5 tanβ)^6`) ; tension (V<=0) skip.

### 2.4 Sliding resistance (GEO-F04)

- Clause: EN 1997-1 §6.5.3; DE: DIN 1054 Gleitwiderstand. EN: sliding; DE: Gleiten.
- Inputs: `H_d`, `V_k` (characteristic permanent vertical force only stabilising, plus variable stabilising 0), base friction angle `δ_k` (cast-in-situ: δ=φ'k; precast: 2/3 φ'k; DE DIN 1054 `δ=φ'k` rough, `tanδ = 2/3 tanφ'` precast), c_u,k for undrained, passive earth `E_p,k`, adhesion.
- Formulas: drained `R_t,k = V_k tanδ_k`; undrained `R_t,k = A' cu,k` (limit `R <= 0.4 V` where specified); `R_d = R_t,k/γR,h + E_p,d`, `E_p,d = E_p,k/γR,e`; `H_d = γG H_G,k + γQ H_Q,k`.
- NDP: γR,h 1.0 R1/1.1 R2 (EN) / 1.10 (NA-DE); passive mobilisation: only 50 % of E_p unless wall movement admissible `[V]` (DE: use mobilised passive pressure `E_p,mob` at settlement limits).
- Outputs: `μ = H_d/R_d`.
- Criterion: `H_d <= R_d`.
- Edge: uplift of foundation (V<=0, μ=∞ fail); water pressure reduces V (use buoyant V'); inclined base (resolve along base); passive resistance only with embedded nib and no later excavation (flag `passive_secured`); soil interface with geotextile (δ set manually).

### 2.5 Overturning EQU (EQU-01)

- Clause: EN 1997-1 §2.4.7.2, §9.? ; DE: Kippen / Lagesicherheit (GZ 1A). EN: Overturning / loss of static equilibrium; DE: Kippsicherheit.
- Formula: `E_dst,d <= E_stb,d + T_d` with `E_dst,d = γG,dst M_G,dst + γQ,dst M_Q,dst`, `E_stb,d = γG,stb M_G,stb`; moment about toe.
- Criterion: `μ = E_dst,d/E_stb,d <= 1`. DE additionally eccentricity: BS-P `e <= B/3` (gaping joint at most up to centre line for BS-A) `[V]`.
- Edge: stabilising passive earth must not be counted unless secured; toe bearing check in parallel (GEO-F01); soil weight on heel counted with stb factors; EN uses same soil weight in both lists => decompose actions in editor.

### 2.6 Uplift UPL (UPL-01)

- Clause: EN 1997-1 §10.2; DE: Auftrieb. EN: uplift; DE: Auftriebssicherheit.
- Formula: `V_dst,d = G_dst,d + Q_dst,d`, `G_dst,d = γG,dst G_dst,k`, `Q_dst,d = γQ,dst Q_dst,k`, `V_dst,d <= G_stb,d + R_d`, `G_stb,d = γG,stb G_stb,k`; for pore pressure form `u_dst,d = γ u <= σ_stb,d`.
- Outputs: `μ`, required extra ballast weight `ΔG`, required anchor force.
- Edge: anchors via section 2.13, friction at shear surfaces `R_d` separately; sequence of water levels (flooding BS-A); water level above ground surface; permeable slab (relief wells).

### 2.7 Hydraulic heave HYD (HYD-01, HYD-02, HYD-03)

- Clause: EN 1997-1 §10.3; DE: Hydraulischer Grundbruch. EN: hydraulic heave; DE: hydraulischer Grundbruch.
- Formulas: `u_dst,d <= σ_stb,d` or `S_dst,d <= G'_stb,d` with `S_dst,k = i γw V_soil` (seepage force on prism of width d/2 behind wall, depth d), `G'_stb,k = γ' V_soil`; `i = Δh/L_flow`, `i_crit = γ'/γw`; DE: `γH S_dst,k <= γG,stb G'_stb,k`, `γH = 1.80`.
- Flow net: simplified (Terzaghi prism `d/2` width), optionally Harza `i_exit = Δh /(L total)` with refinement by flow-net factor `ϑ` from tables (DIN 1054 / Davidenkoff).
- Outputs: i, i_crit, F = i_crit/i, μ.
- HYD-02 Piping at the foot of an embedded wall (EN 1997-1 §10.4): `i_exit <= i_crit = γ'/γw`, Lane/Bligh creep ratios `L_w/Δh`.
- HYD-03 Internal erosion and heave (EN 1997-1 §10.4): Terzaghi toe prism `γH γw h_m d/2 <= γH,stb γ' d²/2` and largest gradient of the finite-difference flow net against `γ'/γw`.
- Edge: anisotropic permeability, layered soils with low-k lid (piping), sand boil, filter requirements (Terzaghi `d15,f < 4 d85,b`, `d15,f > 4 d15,b`), floor pressures.

### 2.8 Settlement (SLS-01 elastic/stress-strain, SLS-02 consolidation, SLS-03 secondary, SLS-04 tilt) Annex F

- Clause: EN 1997-1 §6.6 and Annex F.1 (stress-strain), F.2 (adjusted elasticity); DE: DIN 4019 (Setzungsberechnung), DIN EN 1997-1/NA §6.6. EN: settlement; DE: Setzung.
- Inputs: net base pressure `σ0 = q - γ d`, geometry, layered `Eoed,i` (or `Es`), `ν`, limit depth `z_lim` where `σz = 0.2 σ'v0` (DE: 0.2 σ'v, or 0.1 σ'v for yielding soils `[V]`).
- Formulas: `s = Σ σ_z,i h_i / E_oed,i` (stress-strain, DIN 4019 form `s = Σ (σz,i / Es,i) h_i`), stresses by Boussinesq/Steinbrenner `σz = σ0 · i(z)` rect corner method; circular centre `σz = σ0 [1 - (1/(1 + (a/z)²))^{3/2}]`; 2:1 simplified spread `σz = σ0 BL/((B+z)(L+z))`. Elastic (Annex F.2) `s = p b f / E_m`, `f` influence factor.
- Consolidation (primary): `s_c = Cc H/(1+e0) log10((σ'0 + Δσ)/σ'0)` NC; OC: `s = Cs H/(1+e0) log10(σ'p/σ'0) + Cc H/(1+e0) log10((σ'0+Δσ)/σ'p)` if `σ'0+Δσ > σ'p`; time: `U(Tv)`: `Tv = π/4 U²` for U<0.6, `Tv = -0.933 log10(1-U) - 0.085` otherwise; `t = Tv H_dr²/cv`; T90 = 0.848.
- Secondary: `s_sec = Cα H/(1+e0) log10(t2/t1)`.
- Differential/tilt: `tan ω = (s_A - s_B)/L`; angular distortion limits (Table H.1 informative): `β <= 1/500` frames (limit for serviceability), `1/150` ultimate; `s_max` NDP: isolated footing 50 mm typical (EN informative), DE: project-defined `[V]`.
- Outputs: s(x) over footing, s_max, time-settlement curve, angular distortion, contour.
- Criterion: `s <= s_lim` (SLS characteristic combination, partial factors 1).
- Edge: groundwater changes (stress path), rigid vs flexible footing correction (0.8 centre average factor), interaction of neighbouring footings (superposition), heave on excavation (swelling `Δσ<0`), preconsolidation transition, layer thinner than `z_lim`.

### 2.9 Piles (PILE-01…PILE-09)

#### PILE-01 Compression resistance from static load tests

- Clause: EN 1997-1 §7.6.2.2 (Table A.9), DE: DIN 1054 §7.6.2.2, Table A 7.1; EN: static load test; DE: statische Probebelastung.
- Inputs: measured `R_c,m,i` for `n` tests (at settlement criterion `s = 0.1 D` or failure), design factors ξ.
- Formulas: `R_c,k = min{ (R_c,m)_mean/ξ1 ; (R_c,m)_min/ξ2 }`; `R_c,d = R_c,k/γt` (total) or `R_b,k/γb + R_s,k/γs`.
- ξ (EN recommended, Table A.9): n=1: ξ1 1.40, ξ2 1.40; 2: 1.30/1.20; 3: 1.20/1.05; 4: 1.10/1.00; >=5: 1.00/1.00.
- ξ (NA-DE, DIN 1054 Table A 7.1) `[V]`: n=1: 1.35/1.35; 2: 1.25/1.15; 3: 1.15/1.05; 4: 1.05/1.00; >=5: 1.00/1.00. Stiff structure bonus (EN: ξ divided by 1.1 when structure can redistribute, ξ1 >= 1.0).
- Criterion: `F_c,d <= R_c,d`.

#### PILE-02 Compression resistance from ground tests (CPT/profiles)

- Clause: EN 1997-1 §7.6.2.3 (Table A.10), DE: DIN 1054 / EA-Pfähle empirical tables, Erfahrungswerte.
- Formula: `R_c;cal = R_b;cal + R_s;cal`; `R_b = A_b q_b;k`, `R_s = Σ A_s,i q_s,i;k`; `R_c;k = min{ (R_c;cal)_mean/ξ3 ; (R_c;cal)_min/ξ4 }`.
- ξ3/ξ4 (EN Table A.10): n=1: 1.40/1.40; 2: 1.35/1.27; 3: 1.33/1.23; 4: 1.31/1.20; 5: 1.29/1.15; 7: 1.27/1.12; 10: 1.25/1.08. NA-DE: replaced by empirical-table resistances `R_c,k` from EA-Pfähle with `γ = 1.10` and settlement-dependent resistance-settlement lines (mobilisation of `q_s(s)`, `q_b(s)`); resistance-settlement table gives `s_g = 0.02 D`...`0.10 D`.
- Outputs: shaft/base split, utilisation, R(s) curve.

#### PILE-03 Tension resistance

- `R_t,k` from tests (Table A.9 ξ) or ground tests (ξ3, ξ4); `R_t,d = R_t,k/γs,t`; self weight `G_pile` favourable (1.0). DE `γs,t = 1.15`. Criterion `F_t,d <= R_t,d + G_stb,d`.

#### PILE-04 Transverse (lateral) loading

- Clause: EN 1997-1 §7.7; DE: DIN 1054 + EA-Pfähle (Blum, Winkler/p-y).
- Formulas: Winkler subgrade modulus `k_s,h = E_s,h/D` (or p-y), characteristic length `λ = (k D/(4 E_p I_p))^{1/4}`, closed form free-head long pile: `y0 = 2 H λ/(k D)` ... `M_max = 0.322 H/λ` (free head, constant k); Broms short-pile limit: `H_u = 1.5 Kp γ D L³/(2 (e + L))`... (cohesionless free head `H_u = 0.5 γ D L³ Kp/(e+L)`), cohesive `H_u = 9 cu D (L - 1.5 D - f)`. Verification: `H_d <= H_R,d = H_R,k/γR,e`, deflection `y <= y_lim` (SLS), structural `M_Ed <= M_Rd` through EN 1992.
- Edge: pile group shadowing `p-multiplier` row 1 0.8 / 0.4 / 0.3 (3+ rows), liquefiable layers (EC8-5), free vs fixed head.

#### PILE-05 Negative skin friction (downdrag)

- Clause: EN 1997-1 §7.3.2.2, §7.3.2.? ; DE: negative Mantelreibung. Settling layer thickness `h_s`, `q_s,neg = β σ'v` (β = K tanδ ≈ 0.2…0.3), `F_neg = Σ π D q_s,neg h`; neutral plane at `F_c = R_s,pos`. Check: `F_c,d + F_neg,d <= R_c,d` where `F_neg` is treated as permanent action (γG), no reduction by negative friction in tension.
- Outputs: neutral depth `z_n`, drag-load, additional pile settlement.

#### PILE-06 Group effects

- Clause: EN 1997-1 §7.6.2.? and §7.5; DE: Pfahlgruppe. Efficiency `η_g = R_g/(n R_single)` (Converse-Labarre `η = 1 - θ [(m-1)n + (n-1)m]/(90 m n)`, `θ = arctan(D/s)` [°]); block failure `R_block = 2 (B + L) Σ cu,i h_i + B L N_c cu,b` (undrained); pile-raft interaction coefficient `α_pr`; load distribution by rigid cap `N_i = V/n ± M_x y_i/Σy² ± M_y x_i/Σx²`.
- Criterion: `max N_i,d <= R_c,d`, `min N_i,d >= -R_t,d`, `R_block,d >= V_d`.
- NDP: DE group effects for spacing `s >= 3 D` none, closer: reduction `[V]`.

#### PILE-07 Structural pile design interface (EN 1992/1993)

- Output actions `N_Ed, V_Ed, M_Ed` along shaft; hand over to `en1992` check; buckling in soft clay `N_cr = π² EI/L_eff²` with `L_eff` (cu<10 kPa).

#### PILE-08 Dynamic tests and pile driving formulas (EN 1997-1 §7.6.2.4)

- `R_c;m` from signal matching (CAPWAP); `ξ5/ξ6`: n>=2 1.60/1.50 ... (EN Table A.11, ≥5: 1.50/1.35; ≥20: 1.45/1.30?) `[V]`; DE: only with calibration; dynamic formula `R = η_h W h/(s + 0.5 s_e)`. Optional.

#### PILE-09 Settlement of pile (resistance-settlement)

- `s = s_s + s_b` elastic compression `s_el = Σ N_i L_i/(E A)`, base via `s_b = q_b D/(…)` or non-linear t-z curves `q_s(s) = q_s,max · s/s_ref` linear up to `s_ref = 0.5…2%D`; verify `s_d <= s_lim` (SLS characteristic).

### 2.10 Earth pressure and retaining walls (RW-01…RW-10)

Clause family EN 1997-1 §9, Annex C (informative); DE: DIN 4085 (Erddruck), DIN 1054 §9, EAB.

#### RW-01 Active earth pressure, Coulomb

`Ka = cos²(φ' - α) / ( cos²α cos(α + δ) [1 + sqrt( sin(φ' + δ) sin(φ' - β) / (cos(α + δ) cos(α - β)) )]² )`, α wall batter from vertical (positive leaning backward), β ground slope, δ wall friction (a ≤ φ'; `δa = 2/3 φ'` rough, `δa = 0` smooth, DE: `δa = ±2/3 φ'` for normal rough, `0` for sheet pile cantilever? `[V]`). Resultant `E_a = 0.5 γ H² Ka`, components `Eah = E_a cos(α+δ)... ` (for vertical wall `Eah = Ea cosδ`, `Eav = Ea sinδ`).

#### RW-02 Active, Rankine

`Ka = (1 - sinφ')/(1 + sinφ')` level ground vertical smooth; sloped `Ka = cosβ (cosβ - sqrt(cos²β - cos²φ'))/(cosβ + sqrt(cos²β - cos²φ'))`; cohesion `e_a(z) = Ka γ z - 2 c' sqrt(Ka)`, tension zone `z0 = 2c'/(γ sqrt(Ka))` ignored (DE: negative pressure not taken; minimum `e_a >= 0.2 · γ z Ka` (EAB minimal pressure `0.2 p_v` for cohesive) `[V]`).

#### RW-03 Passive earth pressure

`Kp(Coulomb)= cos²(φ' + α) / ( cos²α cos(δ - α) [1 - sqrt( sin(φ' + δ) sin(φ' + β) / (cos(δ - α) cos(α - β)) )]² )` is unsafe for δ>φ'/3, therefore use Caquot-Kerisel/Sokolovskii curved failure surface `Kp` from table (log-spiral) with `δp = -2/3 φ'` (DE `δp ≤ -φ'/2`... `[V]`): `Kp(φ=30°, δ=-φ'/2, vertical wall, level) ≈ 4.0 ... 4.78`, Rankine `Kp = (1+sinφ')/(1-sinφ')` (=3.0 at 30°); `Ep,k = 0.5 γ t² Kp + 2 c' t sqrt(Kp)`; `Ep,d = Ep,k/γR,e`; mobilisation `E_p,mob = E_p,k · s/s_p`, `s_p ≈ 0.04…0.10 t` dense/loose. Surcharge adds `Kp q`.

#### RW-04 At-rest earth pressure

`K0 = (1 - sinφ') √OCR` (Jaky/Mayne-Kulhawy); slope `K0β = K0 (1 + sinβ)`; used when wall deformation prevented (basement walls) and for ΔEah-increments `E0g` (DIN 4085: mobilised for SLS of rigid bottom-braced walls).

#### RW-05 Surcharges and water

- Uniform surcharge `Δe = K q`; strip load per Boussinesq `Δσh = (2 q/π)(β - sinβ cos(β + 2δ))` (strip), line load `Δσh = (4 Q/π) ...` (Terzaghi/Gerber), compaction pressure `[V]`. Water: hydrostatic `u = γw (z - z_w)`, `e_a = Ka σ'v + u` below z_w; steady seepage: `e_a` with reduced `γ'` ± `i γw` (flow-net); partial submergence; unbalanced water head `Δh γw` across wall. Output pressure diagram per height: arrays `{z, e_a,earth, e_a,surcharge, u}` and resultants (force + lever arm).

#### RW-06 Gravity / cantilever wall verifications

- Overturning EQU (2.5); sliding (2.4); bearing (2.3, eccentric base, inclined load `tanδ = Eah/(V+Eav)`); overall stability (2.11); structural (EN 1992). Base pressure distribution trapezoidal/triangular: `σ = V/(B)(1 ± 6e/B)` for `e <= B/6`, `σmax = 2V/(3 (B/2 - e))` for `e > B/6` (no tension).
- DE extra: mobilised wall friction check `δ <= φ'`, wall tilt/rotation about toe.

#### RW-07 Embedded (sheet pile, bored pile, diaphragm) walls

- Clause: EN 1997-1 §9.7, DE: EAB/DIN 4085, DIN 1054 §9.7. Types: free earth support (hinged, base at fixed depth), full fixity (Blum), cantilever, single/multi prop, anchor.
- Free-earth-support equilibrium: find embedment `t` such that `Σ M(prop) = 0`: `E_ah,d (z_a) - E_ph,d (z_p) = 0` with `E_ph,d = E_ph,k/γR,e` (DA2 DE: passive resistance reduced by γR,e = 1.40; DA2*: factor effects). Cantilever: `M_wall = 0` at `y` below dredge; DE extra embedment `+20 %` for fixed-end assumption (Blum `t_final = 1.2 t_0`) `[V]` (EAB: increase 20 % for assumed rotation).
- Verifications: toe stability (vertical force: `V_d = Σ(E_av + P_anchor,v + G) <= R_toe,d`), wall bending (`M_max,d`), prop/anchor force `A_d`, vertical equilibrium, hydraulic (2.7), basal heave in clay `γ H / cu <= N_c` (Terzaghi/Bjerrum-Eide `F = cu Nc / (γ H + q)`, `Nc ≈ 5.14…7.5`).
- Outputs: embedment length, support loads, bending moment diagram, deflection (optional beam on Winkler), mobilised passive percentage.
- Edge: stepped excavation staged analysis (construction sequence as event list), water on one side, sloping toe, ground anchors inclination `θ`, tied wall with berm, rigid vs flexible reduction of passive (Rowe moment reduction).

#### RW-08 Seismic active/passive (Mononobe-Okabe) — cross reference EN 1998-5 §7.3.2.

#### RW-09 Reinforced soil / gabion (informational)

- Internal stability (tie-back wedge, rupture and pullout), `T_max = K σv Sv`, `L_e = T/(2 b f* σ'v)`; mark as optional extension (not required for norm-completeness).

#### RW-10 Surcharge at crest (traffic) and compaction-induced pressure for rigid walls

- Lateral pressure from traffic `q = 10…33 kPa` (NDP bridge approaches, EN 1991-2 LM1 distribution 45° spread) .

### 2.11 Overall stability, slope (SLOPE-01 Bishop, SLOPE-02 Janbu, SLOPE-03 infinite slope, SLOPE-04 wedge)

- Clause: EN 1997-1 §11, DE: DIN 4084 (Böschungs- und Geländebruch), DIN 1054 §11.
- Method Bishop simplified (circular), slice i: `W_i = γ_i b_i h_i + q_i b_i`, `α_i` base inclination, `u_i`:
  `F_s = Σ [ (c'_i b_i + (W_i - u_i b_i) tanφ'_i)/mα_i ] / Σ W_i sinα_i`, `mα_i = cosα_i (1 + tanα_i tanφ'_i/F_s)`; iterate on `F_s` (fixed-point, tolerance 1e-6, max 50).
  Undrained circle (φu=0): `F_s = Σ cu,i l_i R / (Σ W_i x_i)`.
  Fellenius: `F_s = Σ [c' l + (W cosα - u l) tanφ'] / Σ W sinα`.
- Janbu simplified (non-circular): `F_s = f0 Σ [(c'b + (W - u b) tanφ')/ (cosα m_α)] / Σ W tanα`, `f0` correction `1 + b1 (d/L - 1.4 (d/L)²)`, b1 = 0.69 (c-φ), 0.5 (c only), 0.31 (φ only).
- DE design: GZ 1C: design shear parameters `tanφ'd = tanφ'k/γφ'`, `c'd = c'k/γc'`, `cu,d = cu,k/γcu`, action factors γG = 1.00, γQ = 1.30; degree of utilisation `μ = E_d / R_d = Σ(W sinα)_d / Σ(resisting)_d <= 1.0` where driving moments are `E_d` and resisting `R_d` (Bishop/Fellenius in design values; no extra model factor; DIN 4084 `η_M = 1.0`). EN DA3: `A2 + M2 + R3`, `μ = Md/Mr,d`.
- Search: grid of centres `(xc, yc)` × radii `R` (or tangents to a depth), report global minimum F_s, per-slice table, critical circle polygon. Plus non-circular polyline for Janbu (user drawn).
- Infinite slope: dry `F = tanφ'/tanβ`; seepage parallel `F = (γ'/γsat) tanφ'/tanβ`; c-φ `F = c'/(γ z sinβ cosβ) + tanφ'/tanβ`.
- Edge: tension crack (depth `z_c = 2c'/(γ sqrt(Ka))`, optional water), submerged slopes, earthquake coefficient (EN 1998-5 pseudo-static `k_h = 0.5 ag S/g`), tie-back reinforcement, multiple layers with a wall; Bishop non-convergence when `mα <= 0.2` (toe slices, large negative α) => error flag.

### 2.12 Ground investigation and parameter derivation (EN 1997-2)

Clause family EN 1997-2 (Ground investigation and testing), DE: DIN EN 1997-2 + NA, DIN 4020, DIN 4094 (CPT/DPT), DIN EN ISO 22476.

| Id | Clause | Name EN / DE | Inputs | Formula | Output |
|---|---|---|---|---|---|
| GI-01 | 2.2/2.3 + Annex B.3 | Investigation extent / Erkundungsumfang | GC, structure footprint | min depth `z_a >= max(6 m, 2 B)` for footings (B width); piles `z_a >= max(5 m, 3 D_b)` below base; spacing 20-40 m (GC1-GC2), 10-25 m (GC3, DIN 4020) `[V]` | required depth, spacing, missing-investigation warning |
| GI-02 | Annex E (CPT) | CPT classification / Drucksondierung | `qc, fs, u2` | `qt = qc + u2 (1 - a)`; friction ratio `Rf = fs/qc` ·100; `Ic = sqrt((3.47 - log10 Q)² + (log10 F + 1.22)²)`, `Q = (qt - σv)/σ'v ... normalized, F = fs/(qt - σv)·100` | soil behaviour type |
| GI-03 | Annex D.? | CPT cu | `qt, σv0` | `cu = (qt - σv0)/Nkt`, `Nkt = 15…20` (NA/ project) | cu profile |
| GI-04 | D.? | CPT φ' (Kulhawy-Mayne) | `qc, σ'v0` | `φ' = 17.6° + 11° log10( (qc/pa)/sqrt(σ'v0/pa) )`, pa=100 kPa | φ' |
| GI-05 | D.? | CPT Eoed | `qc` | `Eoed = α_E qc`, `α_E = 3…5` sand/ clay by Ic | Eoed |
| GI-06 | Annex F (SPT) | SPT correction | `N, rod energy, borehole, sampler, rod length` | `N60 = N Er/60 Cb Cs Cr`; `(N1)60 = N60 sqrt(100/σ'v0 [kPa]) (CN <= 1.7)` | corrected N |
| GI-07 | F.? | SPT φ' (Hatanaka-Uchida) | `(N1)60` | `φ' = sqrt(20 (N1)60) + 20` | φ' |
| GI-08 | F.? | SPT Dr | `(N1)60` | `Dr = sqrt( (N1)60 / 60 )` (Skempton) | Dr |
| GI-09 | Annex G/H | DPH/DPL, DPSH | `N10, N20` | `N10 = …`; DIN 4094 correlation to `φ'` via `ID`; table | φ' range |
| GI-10 | Annex ? | Lab: oedometer | e-logσ' series | `Cc = Δe/Δlog σ'`, `Cs`, `σ'p` (Casagrande), `cv` (log-time Taylor: `cv = 0.848 H²/t90`) | consolidation parameters |
| GI-11 | Lab: triaxial | CU/CD | `q-p'` data | `φ' = arcsin(tanα')` ..., `c' = a/ cosφ'`, `Mohr-Coulomb envelope` | φ', c' |
| GI-12 | Lab: direct shear | `τ_f(σ)` | linear regression `τ = c' + σ tanφ'` (n>=3) | φ', c' |
| GI-13 | 2.4 | Characteristic value derivation | see G-02 | `Xk` | per-parameter |
| GI-14 | Pressuremeter | Menard | `p_l*, E_M` | `qb,k = k_p p_le*`, `E_oed = E_M/α_M` | |
| GI-15 | Grain-size | classification | sieve/hydrometer | `Cu = d60/d10`, `Cc = d30²/(d10 d60)`, DIN 18196 mapping, `k = 0.0116 d10²` (Hazen, k in m/s with d10 in mm) | soil group, k |
| GI-16 | Water | permeability, pump test | `k` | Dupuit/Thiem (2.14) | k |
| GI-17 | 2.4.5.2 | Lab strength regression, characteristic lower bound | `σ-τ` or `p-q` series (n>=3) | least-squares line, one-sided Student-t lower bound of intercept and slope (n-2 degrees of freedom), applied to the GI-11/GI-12 regressions and compared with the assumed layer values `φ_assumed`, `c_assumed` | φ'k, c'k |

Criterion for GI: qualitative (coverage, number of data points n per layer >= 3 recommended; V_x warnings). Edge: CPT refusal, SPT gravel correction, scale effect, correlations valid only in stated soil ranges (editor attaches validity range chips and refuses silent extrapolation).

### 2.13 Anchors (ANC-01…ANC-04)

- Clause: EN 1997-1 §8, DE: DIN 1054 §8, DIN EN 1537 (Verpressanker).
- ANC-01 Pull-out resistance from suitability tests: `R_a,k = min{ (R_a,m)_mean/ξ_a1 ; (R_a,m)_min/ξ_a2 }`, ξ_a1 = ξ_a2 = 1.10 (n>=? NDP, EN recommended 1.10), `R_a,d = R_a,k/γa`, `γa = 1.1` (EN permanent/transient 1.1 recommended R1: 1.1; DE: 1.10 all situations).
- ANC-02 Fixed-length empirical: `R_a,k = π D L_f τ_f,k` with grout-soil skin friction `τ_f`, `L_f` fixed length 4-10 m; `τ_f` table (sand 0.1…0.3 MPa, clay by cu 0.05…0.1).
- ANC-03 Anchor force and inclination: `A_d = E_h,d / cosθ`, tested at `1.25 A_k` (proof), lock-off 0.8-1.0 `A_k`; external stability (Kranz): wedge between wall toe and mid of fixed length with `A_k,max` iterative on `E_ah'`: `A_Kranz = (E_ah - E_a,s ...)` ; equal to `F_s >= 1/1.4` flows (DE: γ=1.0 EQU-like `[V]`).
- ANC-04 Anchor steel structural: `A_d <= P_t,d` (EN 1537 `P_t0.1k/γ_s`), free length `L_free >= 4.5 m` (3 m for bars), anchor head; deformation.
- Criterion: `A_d <= R_a,d`; Kranz `η >= 1.5` (EN 1.0 with design values `[V]`). Edge: anchors in temporary vs permanent, group effects spacing `< 1.5 m`, corrosion protection class (permanent: DE).

### 2.14 Dewatering (DEW-01…DEW-03)

- DEW-01 Single well discharge and required number of wells. DEW-02 Well group as equivalent big well `r_eq = sqrt(A/π)` with superposed drawdown. DEW-03 Settlement from the drawdown `Δσ' = γw Δh`.

- Clause: EN 1997-1 §10; DE: DIN 4126 / EAB EB 9, DIN 4044 (Wasserhaltung). EN: groundwater control; DE: Wasserhaltung.
- Single well, unconfined Dupuit: `Q = π k (H² - h²)/ln(R/r)`; confined: `Q = 2 π k D (H - h)/ln(R/r)`; radius of influence Sichardt `R = 3000 s sqrt(k)` [m, s in m, k in m/s] ; Kusakin `R = 575 s sqrt(H k)` with k in m/s? `[V]`; well group (equivalent big well): `Q = π k (H² - h²)/(ln R/r_eq)`, `r_eq = sqrt(A/π)`, superposition for drawdown `s(x,y) = Σ Q_j/(2 π k D) ln(R/r_j)`.
- Pore pressure / piping, HYD feeder (2.7).
- Output: Q [m³/h], drawdown contour, required number of wells (`Q_tot/Q_well`), settlement from drawdown through `Δσ' = γw Δh` (feeds SLS-01).
- Edge: wells not fully penetrating, anisotropy `k_h/k_v`, recharge boundary, drawdown below sheet-pile toe (HYD).

### 2.15 Seismic cross reference

All geotechnical seismic checks (liquefaction, foundations, walls) are cataloged in section 3 (EN 1998-5) and reuse sections 2.3-2.11 with `Ed` augmented by seismic combination (`A_Ed` in `E_d = Σ G + P + A_Ed + Σ ψ2 Q`, partial factors 1.0, soil factors `γφ' = 1.25`, `γcu = 1.4`, bearing `γRd = 1.0`, EC8-5 Annex F).

## 3. EN 1998 Verification Catalogue

Common formulas used across items (EN 1998-1 §3.2, DE NA):

- Importance factor `γI` and `ag = γI · agR`: EN recommended I 0.8, II 1.0, III 1.2, IV 1.4; NA-DE same values; DE `agR` per zone (Table NA.1): zone 0: n/a (no verification), zone 1: 0.4 m/s², zone 2: 0.6 m/s², zone 3: 0.8 m/s² (existing `DeSeismicZone::a_gr`). EN annex: `agR` entered by the user from the national map.
- Elastic horizontal spectrum Se(T) (η = sqrt(10/(5+ξ)) >= 0.55, ξ in %):
  `0 <= T <= TB: Se = ag S [1 + (T/TB)(2.5η - 1)]`
  `TB <= T <= TC: Se = 2.5 ag S η`
  `TC <= T <= TD: Se = 2.5 ag S η (TC/T)`
  `TD <= T <= 4 s: Se = 2.5 ag S η (TC TD/T²)`
- Design spectrum Sd(T): `0<=T<=TB: ag S [2/3 + (T/TB)(2.5/q - 2/3)]`, `TB..TC: ag S 2.5/q`, `TC..TD: max( ag S (2.5/q)(TC/T) ; β ag )`, `TD..: max( ag S (2.5/q)(TC TD/T²) ; β ag )`, `β = 0.2`.
- Vertical spectra Sve(T) (EN 1998-1 §3.2.2.3, η = 1 for 5 %): `0<=T<=TB: avg [1 + (T/TB)(3η - 1)]`, `TB..TC: 3 avg η`, `TC..TD: 3 avg η (TC/T)`, `TD..: 3 avg η (TC TD/T²)`; `avg/ag` and period parameters `TB = 0.05, TC = 0.15, TD = 1.0` for both types (recommended); Type 1 `avg/ag = 0.90`, Type 2 `0.45`. NA-DE: vertical spectrum `[V]` follows DIN EN 1998-1/NA NA.? (`avg = 0.5 ag S`?, to be verified, NDP `avg_ratio` table asset).

### 3.1 EN 1998-1 General rules, seismic actions, buildings

#### SEI-01 Ground types and soil factor S (EN 1998-1 §3.1, Table 3.1; DE NA 3.1.2)

- Name EN / DE: Ground types / Baugrundklassen und geologische Untergrundklassen.
- Inputs: `v_s,30 = 30 / Σ_{i=1}^{N} (h_i/v_i)` [m/s] (h_i summing to 30 m), `N_SPT` or `cu` alternative; DE: geologic subsoil class R (Fels/ rock-like), T (Übergangsbereich), S (Sedimentbecken) from the geological map (DIN 4149:2005 map, DIN EN 1998-1/NA map) combined with ground class A (hard), B (medium), C (soft) -> combos `A-R, B-R, C-R, B-T, C-T, C-S` (existing `DeGroundCombo`); other combos not defined => invalid.
- EN ground types: A `vs30 > 800`; B `360-800` (N_SPT > 50, cu > 250 kPa); C `180-360` (N_SPT 15-50, cu 70-250); D `< 180` (N_SPT < 15, cu < 70); E surface alluvium of C/D type 5-20 m over A-like substratum.
- Type 1 parameters (S, TB, TC, TD): A 1.0, 0.15, 0.4, 2.0; B 1.2, 0.15, 0.5, 2.0; C 1.15, 0.20, 0.6, 2.0; D 1.35, 0.20, 0.8, 2.0; E 1.4, 0.15, 0.5, 2.0. Type 2: A 1.0, 0.05, 0.25, 1.2; B 1.35, 0.05, 0.25, 1.2; C 1.5, 0.10, 0.25, 1.2; D 1.8, 0.10, 0.30, 1.2; E 1.6, 0.05, 0.25, 1.2. Type chosen by `Ms` (Ms>5.5 => Type 1).
- NA-DE (Table NA.7) `[V]`: A-R S 1.00, TB 0.05, TC 0.20, TD 2.0; B-R 1.25, 0.05, 0.25, 2.0; C-R 1.50, 0.05, 0.30, 2.0; B-T 1.00, 0.10, 0.30, 2.0; C-T 1.25, 0.10, 0.40, 2.0; C-S 0.75, 0.10, 0.50, 2.0. DE `η = 1` (5 %) default; `β = 0.2`.
- Outputs: ground class, S, TB, TC, TD, flags.
- Edge: vs30 on boundary (use lower class), site requiring special study (S1: cu 10-20 kPa soft clay, S2: liquefiable) => redirect to EN 1998-5 SEI-60; DE: zone 0 => seismic check not required but spectrum and checks are hidden/greyed; DE Zone 1 with importance I/II simplified rules permitted (`ag S <= 0.4 g`?) `[V]`.

#### SEI-02 Seismic zone and ag (EN 1998-1 §3.2.1; DE NA zones 0-3)

- Inputs: zone, importance class. Output `ag = γI agR`, very-low-seismicity flag (`ag S <= 0.05 g`; low: `ag S <= 0.1 g`).
- NA-DE zones: zone 0 `agR` undefined (no design), 1: 0.4, 2: 0.6, 3: 0.8 m/s². Map visual (section 5) keyed by gemeinde-level data asset `zones-de.json`.

#### SEI-03 Elastic response spectrum horizontal (EN 1998-1 §3.2.2.2)

- Formulas see header. Outputs: `Se(T)` arrays, `Se(T1)`, `Sde(T) = Se(T) (T/(2π))²` displacement spectrum (valid up to 4 s; for `T > TE=4.5 s` Eq. 3.7).
- Damping correction: `η = sqrt(10/(5 + ξ)) >= 0.55`.
- Edge: T beyond 4 s (hold `Se(4)·(4/T)²`); `ξ <= 0` invalid; Type 2 only when specified; anomaly in the NDP `TE`.

#### SEI-04 Design spectrum for linear analysis (EN 1998-1 §3.2.2.5)

- Formulas see header; lower bound `β = 0.2` [EN + DE]. Outputs `Sd(T)`; checks `q >= 1.0`.

#### SEI-05 Vertical spectra (§3.2.2.3, §3.2.2.5(5)); vertical design spectrum with q = 1.5 (EN) used for vertical members; combination with horizontal per SEI-30. Criterion/uses: required when `avg > 0.25 g` for specific elements (horizontal cantilevers, prestressed, beams carrying columns, base-isolated), see §4.3.3.5.2.

#### SEI-06 Seismic action definition for time-history / spatially varying (§3.2.3.1, §3.2.3.4): at least 3 accelerograms, mean of response >= 1.0 of elastic; scaled to `ag S`, `0.2 T1`…`2 T1` range ` mean Se >= 0.9`; not a computing priority but an input mode with plotting.

#### SEI-10 Importance classes (§4.2.5, Table 4.3)

- Class I (agricultural), II (ordinary), III (schools, assembly), IV (hospitals, civil protection); `γI` 0.8, 1.0, 1.2, 1.4. Output also reduction factor `ν` for damage limitation (SEI-45).

#### SEI-11 Mass for seismic analysis (§3.2.4, 4.2.4)

- Formula: `Gk,j "+" ψE,i Qk,i`, `ψE,i = φ ψ2,i`; φ (Table 4.2): roof 1.0; storeys with correlated occupancies (categories A-C) 0.8; independently occupied 0.5; storage (E) 1.0; categories D-F 1.0. `ψ2`: A 0.3, B 0.3, C 0.6, D 0.6, E 0.8, F 0.6, G 0.3, H 0.0 (EN 1990 A1.1; NA-DE identical except snow/wind).
- Mass `m_i = (G_k,i + Σ ψE,i Q_k,i)/g`.
- Edge: more than one variable per storey; roof with snow `ψ2 = 0` (below 1000 m) `[V]`; mass eccentricities.

#### SEI-20 Structural regularity in plan (§4.2.3.2) and in elevation (§4.2.3.3)

- Plan criteria: (a) approx. symmetric in two directions with respect to stiffness and mass; (b) compact shape: re-entrant corners <= 25 % of plan dimension; (c) floors sufficiently stiff in-plane (rigid diaphragm); (d) slenderness `λ = Lmax/Lmin <= 4`; (e) `e_0x <= 0.30 r_x`, `r_x >= l_s` (e_0 = distance between centre of stiffness and centre of mass; r = torsional radius `sqrt(K_θ/K_x)`; `l_s = sqrt((L² + B²)/12)` radius of gyration of floor mass). All five => plan-regular.
- Elevation criteria: (a) lateral stiffness/mass elements run continuous; (b) stiffness and mass constant or reduce gradually; (c) ratio `R_i = V_Ed,i/V_Rd,i` between storeys: `(R_i/R_avg) <= 0.2`..., set-backs rules (<= 20 %, <= 50 % overall, <= 30 % at base, <=10 % set back); (d) storey drift stiffness ratios.
- Table 4.1 consequences: regular-plan + regular elevation: model planar, lateral force method permitted, q = reference; irregular plan: model spatial, LFM permitted if cond. 4.3.3.2.1 satisfied; irregular elevation: Sd with q reduced by 20 % (`q = 0.8 q0`), modal analysis; irregular both: spatial + modal + 0.8 q.
- Output: flags `plan_regular`, `elevation_regular` derived (not input) from geometry/stiffness model with explanations (which criterion failed), `q_reduction_factor`.
- Edge: single-storey buildings treat elevation as regular; NA-DE adds `r_x >= l_s` criterion with `[V]` tolerance; reduced rotation of frames in torsionally flexible systems (`rx < ls`) => torsionally flexible: q <= 2.0 for DCM (`q_o = 2.0`).

#### SEI-30 Behaviour factor q (§5.2.2.2, §6.3.2, §7.3.1, §8.3, §9.3, §4.3.4; DE NA)

- Formula: `q = q0 kw >= 1.5`; DE NA lower bound `q >= 1.5` in zone 1? `[V]`.
- RC (EN 1998-1 Table 5.1; `αu/α1` 1.1/1.2/1.3 per type): 
  Frame/dual/coupled-wall: DCM `q0 = 3.0 αu/α1`, DCH `4.5 αu/α1`.
  Uncoupled wall: DCM `3.0`, DCH `4.0 αu/α1`.
  Torsionally flexible: DCM `2.0`, DCH `3.0`.
  Inverted pendulum: DCM `1.5`, DCH `2.0`.
  Default `αu/α1`: one-storey frame 1.1; multi-storey single-bay frame 1.2; multi-storey multi-bay 1.3; wall-equivalent dual: one wall 1.0, wall-systems 1.1.
  `kw = 1` frames and frame-equivalent dual systems; `kw = (1 + α0)/3 ≤ 1` and `>= 0.5` walls, `α0 = Σhwi/Σlwi`.
  If irregular in elevation: `q = 0.8 q_calc`.
- Steel (§6.3.2 Table 6.2): moment frames DCM `4`, DCH `5 αu/α1` (αu/α1: 1.1/1.2/1.3 as above); concentric diagonal DCM `4`, DCH `4`; V-bracing `2.5`, `2.5`; eccentric `4`, `5 αu/α1`; inverted pendulum `2`, `2αu/α1`; moment frame + concentric: `4`, `4αu/α1`; dissipative class DCL = `1.5…2` (no ductility).
- Composite (§7.3.2 Table 7.? ) steel-concrete: analogous to steel (DCM = 4, DCH = 5αu/α1 for moment frame); composite structural walls DCM 3.0, DCH 4.0 αu/α1.
- Timber (§8.3 Table 8.1): DCL 1.5, DCM 2.0 (nailed walls `2.0`), DCH 3.0 (moment-resisting frames with ductile connectors up to 4.0 `[V]`), dowel-type joints.
- Masonry (§9.3 Table 9.1): unreinforced (EN 1996) `1.5-2.5`, confined `2.0-3.0`, reinforced `2.5-3.0`.
- Output: `q`, reasons, ductility class DCL/DCM/DCH requirement set (see SEI-60 detailing).
- NDP DE: reduction for hysteresis? only `q <= 1.5` in zone 1/2 allowed to omit DCM detailing (DE NA permits DCL for zone 1 and 2 with `q<=1.5`); DCM/DCH are mandatory otherwise. `[V]`
- Edge: system differs in two directions (q per direction `q_x`, `q_y`, per existing `En1998System`), `q > 1.5` with detailing incompatible (`detailing_compatible_with_q=false` => fail), q in vertical direction always `<= 1.5`.

#### SEI-35 Fundamental period T1 (§4.3.3.2.2)

- `T1 = Ct H^{3/4}`, `Ct = 0.085` steel moment frames, `0.075` RC moment frames and eccentrically braced steel, `0.050` other; alternatively `Ct = 0.075/sqrt(Ac)`, `Ac = Σ [Ai (0.2 + (lwi/H))²]`, `lwi/H <= 0.9`; Rayleigh: `T1 = 2π sqrt( Σ m_i d_i² / Σ F_i d_i )`; `T1 <= 4 TC` and `T1 <= 2.0 s` for LFM applicability.
- Outputs: T1, method used.

#### SEI-40 Lateral force method (§4.3.3.2)

- Applicability: `T1 <= min{4 TC, 2.0 s}` and elevation regular (§4.3.3.2.1).
- Base shear: `Fb = Sd(T1) m λ`; `λ = 0.85` if `T1 <= 2 TC` and building has more than two storeys; else `1.0`; `m` total mass above foundation.
- Distribution: `Fi = Fb s_i m_i/Σ s_j m_j` (linear mode shape `s_i = z_i`); with modal-shape `s_i = φ_i` equivalent; storey shear `V_i = Σ_{j>=i} F_j`; storey overturning moments.
- Torsion add-on: accidental eccentricity SEI-41.
- Outputs: `Fb`, `Fi`, `V_i`, `M_i`, element forces (via stiffness distribution `k_xy/Σ k`).
- Criterion: element utilisation by material code (en1992/1993/1995/1996/1994) after combination SEI-31.
- Edge: `T1 < TB` use plateau; mass change by storey; `λ` NA-DE keeps 0.85 `[V]`; base-isolated buildings excluded; stories < 3: `λ=1`.

#### SEI-41 Accidental torsion (§4.3.2)

- `e_ai = ± 0.05 L_i` (floor dimension perpendicular to the seismic action); LFM: each element force multiplied by `δ = 1 + 0.6 x/Le` (x distance of element from centre of mass perpendicular to action, Le = distance between outermost lateral load resisting elements); spatial models: torsional moment `M_ai = e_ai F_i`; when `e_0 > e_ai` real eccentricity plus accidental.
- Outputs: `e_ai`, amplification `δ`, torsion moments, directional results.
- Edge: δ for non-symmetric plan applies to stiffer side (use max δ), simplified `(1 + 1.2 x/L)` (§4.3.3.2.4(1)).

#### SEI-42 Modal response spectrum analysis (§4.3.3.3, SRSS/CQC)

- Model: lumped masses per floor, rigid diaphragms 3 DOF (ux, uy, θz) per storey; stiffness matrix from storey stiffness or user-linked FEM; eigen problem `(K - ω² M) φ = 0` (generalised symmetric Jacobi or subspace iteration).
- Modal participation: `Γ_n = φ_nᵀ M r / (φ_nᵀ M φ_n)` (r influence vector), effective mass `M_eff,n = (φ_nᵀ M r)²/(φ_nᵀ M φ_n)`; criterion: `Σ M_eff,n >= 0.90 M_tot` and all modes with `M_eff,n >= 0.05 M_tot` included, or `k >= 3 sqrt(n)` and `T_k <= 0.20 s`.
- Modal forces `F_i,n = Γ_n Sd(T_n) m_i φ_i,n`, base shear modal `V_b,n = Γ_n Sd(T_n) M_eff,n`... (consistent `V_b,n = Sd(T_n) M_eff,n`).
- Combination: SRSS `E_E = sqrt(Σ E_E,n²)` valid if `T_j <= 0.9 T_i` for all i<j; else CQC `E_E = sqrt(ΣΣ ρ_ij E_i E_j)`, `ρ_ij = 8 ξ² (1 + r) r^{3/2} / [(1 - r²)² + 4 ξ² r (1 + r)²]`, `r = ω_j/ω_i`; damping ξ = 0.05 (equal).
- Outputs: periods, mode shapes, effective masses, combined shears/displacements, `T_n` markers on spectrum.
- Edge: closely spaced modes (CQC), rigid-body/zero modes, missing mass correction, higher mode truncation, modal sign conventions.

#### SEI-43 Combination of the horizontal components (§4.3.3.5.1)

- `E_Edx "+" 0.30 E_Edy` and `0.30 E_Edx "+" E_Edy` (envelope). Vertical: `E_Edx "+" 0.3 E_Edy "+" 0.3 E_Edz`, etc. (three-component combos, 3 combos for 2 horizontal + vertical). Alternative SRSS `sqrt(Ex² + Ey² + Ez²)` allowed. Output envelope incl. sign handling.

#### SEI-44 Second-order effect P-Δ, θ (§4.4.2.2(2))

- `θ = P_tot d_r/(V_tot h)`; `P_tot` total gravity load at and above storey (seismic combination), `d_r = q_d d_e` design interstorey drift (`q_d = q`), `V_tot` seismic storey shear, `h` storey height.
- Criterion: `θ <= 0.10` neglect; `0.10 < θ <= 0.20` multiply seismic effects by `1/(1 - θ)`; `0.20 < θ <= 0.30` second-order analysis; `θ > 0.30` forbidden.
- Edge: `θ` for the storey with lowest stiffness; V_tot = 0.

#### SEI-45 Damage limitation drift (§4.4.3.2)

- `d_r ν <= 0.005 h` brittle non-structural elements attached to the structure; `<= 0.0075 h` ductile non-structural; `<= 0.010 h` no interference (non-structural elements fixed in a way not interfering).
- `ν`: EN recommended 0.4 (importance III, IV) and 0.5 (I, II) - note: recommended `ν = 0.5` for importance I and II, `0.4` for III and IV; NA-DE `ν = 0.5` for classes I, II and `0.4` for III, IV `[V]` (existing field `nu`); reduction applied to take the lower return period.
- Output: `d_r ν/h` per storey and direction as bar (section 5).

#### SEI-46 Ultimate limit state: resistance and ductility conditions (§4.4.2)

- `E_d <= R_d` for all members under seismic combination `ΣG + P + A_Ed + Σψ2 Q`; ductility local/global condition: `μ_δ` adequate (`μ_δ = 2 q - 1` for `T1 >= TC`, `μ_δ = 1 + 2(q - 1) TC/T1` for `T1 < TC`); `Rd` per material code with seismic reduced partial factors (`γ = 1` ordinary for NA-DE except concrete/ steel as per material).
- Foundation checks see SEI-70. Diaphragm check: `V_diaph,d = γd 1.3 ...` amplified by 1.3 (transfer forces) `[V]`.
- Equilibrium: overturning moment stability `M_ov,Ed <= M_stab,Rd` (for seismic combination with stab factor 1.0).

#### SEI-47 Seismic joints (§4.4.2.7) 

- Required separation: to the site boundary `d_gap >= d_max = q_d d_e` at the level considered; between separate units on the same property `d_gap >= sqrt(d1² + d2²)` (d_i = q_d d_e,i at the collision level); if the floor elevations of both units coincide the distance may be multiplied by 0.7 (§4.4.2.7(3)).
- Output: required gap, available gap, μ.
- Edge: existing neighbour of unknown stiffness (assumed displacement `d_old = 0.01 H_old` as an editor default `[V]`), multiple levels, columns pounding on slab at different elevation (no 0.7 reduction).

#### SEI-48 Diaphragm and horizontal elements (§4.4.2.5) check of floor shear and chords; amplified forces for transfer.

#### SEI-49 Non-structural elements (§4.3.5)

- `F_a = S_a W_a γ_a / q_a`; `S_a = α S [ 3 (1 + z/H) / (1 + (1 - Ta/T1)²) - 0.5 ]`, `α = ag/g`, `S` soil factor, `Ta` period of element, `T1` fundamental period of building, `z` height of the element above the foundation, `H` building height, `γ_a` importance (1.5 for anchors of machinery/ safety-critical, ≥1.0 otherwise), `q_a` behaviour factor 1.0 – 2.0 (cantilever parapets 1.0, chimneys 1.0, anchors 1.0, partitions 2.0, etc.); limit `S_a >= α S`.
- Criterion: `F_a,d <= R_a,d` (anchor/connection). DE NA: simplified `F_a = 1.5 γ_a α S W_a`? `[V]`.
- Edge: elements below ground; large mass-ratio equipment (> 0.2 m_i) must be included in model.

#### SEI-50 Capacity design, RC frames (EN 1998-1 §5.4.2.2, 5.5.2.x)

- Strong column-weak beam (§4.4.2.3(4), §5.2.3.3(2)): at every joint of primary seismic frames
  `Σ M_Rc >= γ_Rd Σ M_Rb`, `γ_Rd = 1.3` DCM and DCH (EN recommended), NA-DE `1.3` `[V]`; condition for both seismic directions and both bending directions; exempt: top floor, and joints with `ΣM_Rc` check relaxed when columns of ground floor with `N_Ed` low... (NA-DE permits relaxation at `ν_d < 0.1`?? `[V]`).
- Beam shear capacity design (DCM, §5.4.2.2): `V_Ed = V_g+0.3q + γ_Rd (M_Rb,i + M_Rb,j)/l_cl` with `γ_Rd = 1.0` DCM, `1.2` DCH; hogging/sagging sign combos taking worst.
- Column shear: `V_Ed = γ_Rd (M_Rc,i + M_Rc,j)/l_cl`, `γ_Rd = 1.1` DCM, `1.3` DCH; and `M_Ed` for column design `M_Ed = γ_Rd M_Rb ...` amplified bending moment at the ends `1.3 ΣM_Rb ...` (DCM `M_Rc ≥ 1.3 ΣM_Rb` principle).
- Walls: shear magnification `ε = q sqrt( (γ_Rd/q)² (M_Rd/M_Ed)² + 0.1 (Se(Tc)/Se(T1))² ) >= q` `[V]` (dual walls DCH), design envelope of bending moments (tension shift `a_l`).
- Detailing: critical region length `l_cr = max(h_cr, l_w/6, ...)`, confinement `ω_wd`, `α ω_wd >= 30 μ_φ ν_d ε_sy,d b_c/b_o - 0.035`, `μ_φ = 2 q0 - 1` (T1 >= TC) etc., min `ρ_min = 0.5 f_ctm/f_yk`, `ρ_max = ρ' + 0.0018 f_cd/(μ_φ ε_sy,d f_yd)` (DCM/DCH). `ν_d = N_Ed/(A_c f_cd) <= 0.65 (DCM), 0.55 (DCH)`. Minimum dimension of columns `h_c >= 0.25` m.
- Output: per-joint ratio `ΣMRc/(1.3 ΣMRb)`, shear values, detailing flags.
- Criterion: ratio >= 1.0, `ν_d` limits, steel/concrete ductility class B/C reinforcement.
- Edge: joints with beams in one direction only, column at roof, joints with unequal depth, T-joints, hogging vs sagging depends on direction, beam-column joint shear (§5.5.3.3) `V_jhd = γ_Rd A_s1 f_yd - V_C`, `γ_Rd = 1.2`, interior check `V_jhd <= η f_cd sqrt(1 - ν_d/η) b_j h_jc`, `η = 0.6 (1 - f_ck/250)`.

#### SEI-51 Capacity design, steel frames (§6.6)

- Beams-columns: `Σ M_Rc >= 1.3 γ_ov Σ M_Rb` (with overstrength `γ_ov = 1.25` recommended; sum of design moments `M_pl,Rd,c (reduced by N_Ed)`); concentric braces `Ω_i = N_pl,Rd,i/N_Ed,i`, `Ω_max/Ω_min <= 1.25` dissipative diagonals; non-dissipative members `N_pl,Rd(M_Ed) >= N_Ed,G + 1.1 γ_ov Ω N_Ed,E`; connection overstrength `R_d >= 1.1 γ_ov R_fy`; slenderness limits for X, V braces `λ̄ <= 2.0`; eccentric links `e_s = 1.6 M_p,link/V_p,link` (short), `e_l = 3.0 M_p,link/V_p,link` (long), link rotation `θ_p <= 0.08` (short), 0.02 (long).
- Out: Ω per diagonal; criterion see formulas.

#### SEI-52 Timber and masonry (EN 1998-1 §8, §9)

- Timber: dissipative zones only in connections; `ρ_k ≤ 350`; nail diameter `d <= 12 mm`; DCM/DCH min plywood thickness; `q` per SEI-30.
- Masonry: simple masonry rule when `claims_simple_masonry` and wall area ratio per direction `Σ A_w / A_floor >= 0.02 ...` Table (zone, storeys count) `[V]`; confined masonry min area; shear check per EN 1996-1-1 `V_Rd = f_vd t l`.

#### SEI-53 Base isolation (EN 1998-1 §10) 

- Effective period `T_eff = 2π sqrt(M/K_eff)`, effective damping `ξ_eff = Σ E_D,j /(2π K_eff d_cd²)`, design displacement `d_cd = M Se(T_eff,ξ_eff)/K_eff,min`, `K_eff = Σ K_eff,j`; checks: `d_cd <= d_max` (bearing), centring `F_{ec}`, `T_eff >= 3 T_fixed`, `T_eff <= 3 s`, `T_eff >= 2 s`? Provide as part of the bridges item SEI-70 (shared with EN 1998-2).

#### SEI-54 Buildings overview verifier

- Aggregates all SEI items per building into `ComplianceBundle`.

### 3.2 EN 1998-2 Bridges

| Id | Clause | Name EN / DE | Inputs | Formula | NDP (EN / NA-DE) | Output/criterion |
|---|---|---|---|---|---|---|
| BR-01 | 2.1, 2.3 | Compliance criteria, design seismic action / Anforderungen | `ag`, importance (γI bridge: Class I 0.85, II 1.0, III 1.3) | `ag = γI agR`; return period `T_NCR = 475 yr` | γI per NDP | ag |
| BR-02 | 3.2 | Elastic spectrum for bridges | same as SEI-03 with `TB, TC, TD` | `Se(T)` incl. vertical `avg/ag = 0.7` (Type 1) Not `0.9` → bridges use `0.7`/`0.5`/ `[V]` | | |
| BR-03 | 4.1.? | Ductile behaviour q for piers | `Lv = M/V` shear span, `h` section depth, `αs = Lv/h` | `q = 3.5 λ(αs)` RC vertical piers in bending, `λ(αs) = 1` for `αs >= 3`, `λ(αs) = sqrt(αs/3)` for `1 <= αs < 3`; steel `q = 3.5`; inclined struts `q=1.2`; abutments rigid `q=1`; bearing-critical limited ductile `q = 1.5`. | `q_max` | q |
| BR-04 | 4.2 | Analysis methods: fundamental mode (single mode), multi-mode, time history, non-linear | `M_eff`, `K` | `T = 2π sqrt(M_eff/K)`; `F = M_eff Sd(T)`; single-mode `d_E = Se(T)(T/2π)²` (elastic) displacement | | |
| BR-05 | 4.1.6 | Design seismic displacement and effect of cracked stiffness | | cracked stiffness `EI_eff = ν EI_g`... `I_eff = 0.08 … I_g` | | |
| BR-06 | 4.1.8 | Design seismic action combination | `E_Ed = E_x + 0.3 E_y + 0.3 E_z` etc. | | | |
| BR-07 | 5.3, 5.? | Plastic hinge capacity and capacity design | `M_Rd,hinge`, over-strength `γ_0 = 1.35` (RC), `1.25` steel; `γ_0 M_Rd` to shear and foundations | `V_Ed = γ_0 M_Rd,end/L_v`; capacity-protected members `E_Ed = E_G + γ_0 E_Mrd` | `γ_0` | |
| BR-08 | 5.? | Shear design in hinge region | | `V_Rd`, `V_Ed,CD` | | |
| BR-09 | 5.? | Bearings and seismic links | max displacement | `d_Ed = d_E + d_G + ψ2 d_T` , bearing capacity `d_Rd`; seismic link length `L = d_eg + d_Eg + ...` | | `d_Ed <= d_Rd` |
| BR-10 | 6 | Seismic isolation of bridges | `K_eff`, `ξ_eff` | `T_eff = 2π sqrt(M/K_eff)` ; `d_cd = M Se/K_eff`; `ξ_eff = Σ W_j/(2π K_eff d²)`; `η = sqrt(0.10/(0.05 + ξ_eff))` | | |
| BR-11 | 5.? | Foundation and abutments, soil-structure interaction (spring stiffness) | | | | |
| BR-12 | 2.3.6 | Spatial variability of seismic action for long bridges | | `d_ground = ...` | | |

Existing model fields: `En1998Bridge { period_ratio, fundamental_period_s, v_rd_n, bearing_d_rd_m, permanent_gk_n, variables }`. Target adds pier and bearing arrays.

### 3.3 EN 1998-3 Assessment and retrofitting of buildings

| Id | Clause | Name EN / DE | Inputs | Formula | Output / criterion |
|---|---|---|---|---|---|
| AS-01 | 2.2, 3.3 Table 3.1 | Knowledge levels KL1/KL2/KL3 and confidence factors CF / Kenntnisstufen | geometry, details, materials (limited, extended, comprehensive); analysis permitted: KL1 LF or MRS; KL2/3 all | CF: KL1 1.35, KL2 1.20, KL3 1.00 | CF; analysis method gating |
| AS-02 | 2.1 Table 2.1 + DE | Limit states NC / SD / DL (Near Collapse / Significant Damage / Damage Limitation), return periods 2475 / 475 / 225 yrs (EN recommended) | limit state, `agR` with return-period factor `ag(T_R) = ag(475) (T_R/475)^{1/k}` , `k = 3` | | `ag` per state |
| AS-03 | 4.3 / A.? | Material strengths from tests | mean material values `f_m`; ductile mechanism: mean/CF; brittle (shear): `f_m/(CF γ_m)` | | |
| AS-04 | Annex A.3.2 | Chord rotation capacity, yielding `θ_y` | member geometry `h, d, L_v`, `φ_y`, `d_b`, `f_y` [MPa], `f_c` [MPa], `a_V` (1 if shear cracking before flexural yielding else 0), `z` | `θ_y = φ_y (L_v + a_V z)/3 + 0.0013 (1 + 1.5 h/L_v) + 0.13 φ_y d_b f_y/sqrt(f_c)` (A.10b) | θ_y |
| AS-05 | A.3.2 | Ultimate chord rotation `θ_um` | detailing, `ν`, `ω, ω'`, `α` (confinement effectiveness), `ρ_s`, `f_yw`, `f_c` | `θ_um = (1/γ_el) 0.016 (0.3^ν) [ max(0.01,ω')/max(0.01,ω) f_c ]^{0.225} (L_v/h)^{0.35} 25^{α ρ_sx f_yw/f_c} 1.25^{100 ρ_d}` (A.1, for primary seismic elements, `γ_el = 1.5` primary/1.0 secondary; for NC ultimate `θ_um`, for SD `0.75 θ_um`) ; plastic part `θ_pl = θ_um - θ_y` | `D/C` = `θ_Ed/θ_Rd` |
| AS-06 | A.? | Shear capacity degrading with ductility (cyclic) `V_R` | | `V_R = (1/γ_el)[ (h - x)/(2 L_v) min(N; 0.55 A_c f_c) + (1 - 0.05 min(5; μ_Δ^pl)) [0.16 max(0.5; 100 ρ_tot)(1 - 0.16 min(5; L_v/h)) sqrt(f_c) A_c + V_w ] ]` | |
| AS-07 | 4.4 | Verification: ductile: `D = E_d`/ capacity `C`; brittle: `E_d <= R_d`, `E_d` from equilibrium with ductile members at capacity | `E_d`, `C` | check `D/C <= 1.0` | |
| AS-08 | 4.3.3 | Analysis: LF, MRS, non-linear static (pushover), NL time history | | pushover target displacement `d_t = S_e(T*) (T*/2π)² · C...`, N2: `d_t* = Se(T*)/(2π/T*)²` ... (equal displacement for `T* >= TC`: `d_et* = Se(T*)(T*/2π)²`, `d_t* = d_et*`; for `T* < TC`: `d_t* = (d_et*/q_u)[1 + (q_u - 1) TC/T*]`) | | |
| AS-09 | 5/6 | Retrofit: strengthening schemes (jackets, FRP), design values of new materials, interface | | | | |
| AS-10 | DE NA | NA-DE: use of DIN EN 1998-3/NA: limit states restricted, CF per NA, return period fixed to `T_R = 475` with `agR` Table NA.1 | | | | |

### 3.4 EN 1998-4 Silos, tanks, pipelines

- ST-01 Silos: `F_h = m_total Sd(T)` with content mass participation; filling ratio and `ψ_E`: `ψ2 = 0.8` storage (E); mass of content `m_c = filling_ratio Q_k/g`; effective mass fraction for flexible silo; granular content friction `F_wall = μ N`; checks `N_Ed <= N_Rd` (axial buckling), `V_Ed <= V_Rd`; `q_nominal` (silo q = 1.5…2.0 `[V]`). Existing: `En1998Silo { height_m, radius_m, permanent_gk_n, content_qk_n, content_category, filling_ratio, n_rd_n, v_rd_n, q_nominal }`.
- ST-02 Tanks (Housner), cylindrical rigid tanks (Annex A.3): fluid height `H`, radius `R`, `γ = H/R`; impulsive mass `m_i = m tanh(1.7 R/H)/(1.7 R/H)`, convective mass `m_c = m 0.230 (R/H) tanh(1.84 H/R)`; convective frequency `ω_c = sqrt( (1.84 g/R) tanh(1.84 H/R) )`, `T_c = 2π/ω_c`; convective damping `ξ_c = 0.5 %` (`η = sqrt(10/(5 + 0.5)) = 1.348`); base shear `Q = (m_i + m_w) Se(T_i) + m_c Se(T_c)` combined `Q = sqrt(Q_i² + Q_c²)` (SRSS, EN 1998-4 A.3.2.2); overturning moment `M = sqrt(M_i² + M_c²)` with heights `h_i = 0.375 H (γ >= ... )` (`h_i/H = 0.5 - 0.09375 (γ - 1.333)` approx. `[V]`), `h_c = H [1 - (cosh(1.84 H/R) - 1)/((1.84 H/R) sinh(1.84 H/R))]`; sloshing wave height `d_max = 0.84 (R/g) ... = R Se(T_c)/g · 0.84` → freeboard check `d_max <= freeboard`. Hydrodynamic pressure on wall (Veletsos) `p(z,θ) = ...cosθ`. Check: elephant-foot buckling `σ_axial ≤ σ_cr`: `σ_cr = 0.6 E t/R` (clad) with pressure boost.
- ST-03 Pipelines (buried): `ε = a_g / V_s` strain (wave passage `ε = a_g/(2 c)`), `Δ = ε L`; relative displacement at joints; verification `ε <= ε_rd` — optional.
- NDP: `q_tank = 1.5…2.0`, `ξ_c = 0.5 %`, DE NA `EN 1998-4/NA` with `Se` per zone. Existing: `En1998Tank { height_m, radius_m, permanent_gk_n, content_qk_n, content_category, filling_ratio, v_rd_n }`.

### 3.5 EN 1998-5 Foundations, retaining structures, geotechnical aspects

| Id | Clause | Name EN / DE | Inputs | Formula | NDP | Criterion |
|---|---|---|---|---|---|---|
| GE-01 | 3.1 | Ground investigation / Untersuchung, shear wave velocity, `vs30` | boreholes, vs profile | `vs,30` (SEI-01), `Gmax = ρ vs²`, strain-dependent reduction `G/Gmax(γ)`: `1/(1+γ/γ_r)` | | informative |
| GE-02 | 4.1.4, Annex B | Liquefaction susceptibility screen | `ag S >= 0.1 g`, sand fines, GW depth `<= 15 m` | reject if `ag S < 0.1 g` (and soils below 15 m); NA-DE `ag S <= 0.15 g`? `[V]` | | screen |
| GE-03 | 4.1.4(11), Annex B | Liquefaction resistance (Seed-Idriss simplified) | `(N1)60`, `σv0`, `σ'v0`, `ag`, `S`, magnitude `Ms`, fines content | `CSR = τ_e/σ'v0 = 0.65 (ag S/g)(σv0/σ'v0)` (Eq. B.1, EN has no depth reduction `r_d` in Annex B), `CRR_7.5 = 1/(34 - N) + N/135 + 50/(10 N + 45)² - 1/200` (Rauch, N=(N1)60,cs < 30), magnitude scaling `CRR = CRR_7.5 · MSF`, `MSF = (7.5/Ms)^{2.56}`... ; `FS = CRR/CSR` | | `FS >= 1.25` (EC8-5 4.1.4(11)); if `FS < 1.25` ground improvement; NA-DE: note |
| GE-04 | 4.1.4, B.? | Pore pressure increase, lateral spreading flag | | | | |
| GE-05 | 5.? | Slope stability pseudo-static | `k_h = 0.5 α S`... `F_h = 0.5 ag S/g W`, vertical `0.33 or 0.5 F_h` (§4.1.3.3: `F_H = 0.5 α S W`, `F_V = ±0.33 F_H`, if `avg/ag > 0.6`) with topographic amplification `S_T = 1.2…1.4` (Annex A.) ; use SLOPE-01 | | `F_s >= 1.0` with M2 factors / GZ 1C |
| GE-06 | 5.? | Foundation bearing: seismic bearing capacity Annex F | `N_Ed`, `V_Ed`, `M_Ed` | `N̄ = N_Ed γ_Rd / (N_max)`, `V̄ = V_Ed γ_Rd/N_max`, `M̄ = M_Ed γ_Rd/(B N_max)`; interaction `[ (1 - e F_h)^{c_T} (β V̄)^{c'_T} ] ... ` generalised failure surface: `(1 - e M̄ c_M k'_) ^ ...`; criterion `F(N̄, V̄, M̄) <= 0` with `γ_Rd = 1.0` (sand) /1.15 (loose), `1.5` for sensitive clays; for sand: `a = 0.92, b = 1.25, c = 0.92, d = 1.25, e = 0.92, f = 0.41, m = 0.96, k = 0.39, k' = 0.39, c_T = 2.14, c'_T = 1.81, β = 2.57, γ = 4.0` (informative, `[V]`) | | `F <= 0` |
| GE-07 | 5.3.? | Sliding of foundation, seismic | | `F_h <= (V - u) tanδ/γ_Rd` with `γ_Rd = 1.0` | | |
| GE-08 | 5.4.2 | Foundation beams/tie beams: relative displacement limit `d = 0.6 ag S/g ... L` ; ties designed for `N = ± 0.3 α S N_Ed` (soil B) / `0.4` (C) / `0.6` (D) `[V]` for `ag S < 0.1g` ... | ties | | `N_tie ≥ ...` | |
| GE-09 | 5.2 | Foundation stiffness (soil-structure interaction): `K_x = 8 G R/(2 - ν)`, `K_z = 4 G R/(1 - ν)`, `K_rx = 8 G R³/(3 (1 - ν))` (circular, Gazetas); embedment correction; existing field `k_foundation`, `k_soil` | | ratio `k_foundation/k_soil` | rigid if `k_f >= 1.5 k_s`? | |
| GE-10 | 7.3.2, Annex E | Retaining walls: Mononobe-Okabe | `kh = α S/r` (`r` from Table 7.1: free gravity wall 2.0 `(ag S ≤ ...)`, `1.5` flexible/ anchored, `1.0` restrained/ basement), `kv = ± 0.5 kh` or `±0.33 kh`, `θ = arctan(kh/(1 ± kv))` | `K_AE = cos²(φ - θ - ψ)/( cosθ cos²ψ cos(δ + ψ + θ) [1 + sqrt( sin(φ + δ) sin(φ - θ - β)/( cos(δ + ψ + θ) cos(β - ψ) ) )]² )`, `E_d = 0.5 γ (1 ± kv) K_AE H²`; seismic increment `ΔE_d = E_d - E_static` at `0.6 H` from base; passive `K_PE` analog with `-`. Wall inertia `F_w = kh W`. Existing: `En1998RetainingWall { height_m, phi_deg, soil_gamma, r, h_rd_n_per_m }` | `r` Table 7.1 | `H_Ed <= H_Rd` |
| GE-11 | 7.? | Water pressure on walls: hydrodynamic `Δ q = 7/12 kh γw H²` for water pressures (Westergaard) | | | | |
| GE-12 | 5.4 | Piles in seismic: kinematic interaction, bending moments `M_k` from soil deformation, liquefaction-induced loss of lateral support (zero `k_h` in liquefied layer; negative skin friction after liquefaction); `M_Ed`, inertia from superstructure; pile head fixity | | | | |

### 3.6 EN 1998-6 Towers, masts and chimneys

- TW-01 Applicability & damping: structural damping `ξ` per Table (steel chimney 1-2 %, RC 2-5 %, guyed mast 0.5 %…); `η` correction with `ξ`.
- TW-02 Fundamental period: `T1` from Rayleigh for cantilever of variable stiffness / `T = 2π sqrt(m/k)`; chimneys: approximate `T = 1.5 H²/D sqrt(w/EI)`? replaced by Rayleigh.
- TW-03 Base shear (single-mode for uniform-mass cantilever): `Fb = Sd(T1) λ W/g`, linear mode `φ = z/H`: equivalent forces `F_i = Fb m_i z_i/Σ m_j z_j`; overturning `M_b = Σ F_i z_i`; uniform mass resultant at `2H/3`: `M_b = 2/3 H Fb`; higher-mode contribution for slender structures (SRSS) when `T_2 > 0.2 s`: SRSS of first 3 modes; chimney `q ≤ 1.5` (RC) and `q = 1.5…2` steel; guyed masts non-linear cable.
- TW-04 Verification: `M_Ed <= M_Rd` (existing `m_rd_nm`), shear, buckling of steel shell; combination with wind (not in seismic combination) and `ψ2` per EN 1990 (existing variable actions list). Edge: top mass appendages, soil-structure, vertical component for `avg > 0.25 g` on cantilever brackets.
- TW-05 Guyed masts: cable prestress, `Δ = a_g/ω²`; optional.

## 4. Schema-First Entity Models

Schema lives in `🧬️schema/📸️snapshot` (state), `🧬️mutations` (events/commands, CQRS), `💡️inferences` (derived read models), `🔺️diff`. Notation below is language-agnostic (field: type [unit] = default). Existing fields keep their names; new ones are additions. Every entity has `id: string`, `label: LocalizedLabel{en,de}`, `note?: LocalizedLabel`. Events are the only write path; snapshots are folds.

### 4.1 EN 1997 entities

```
En1997Snapshot
  structure_id, geotechnical_category: 1|2|3, design_situation: BS-P|BS-T|BS-A
  design_approach: DA1-C1|DA1-C2|DA2|DA2star|DA3|DE-GZ1B
  annex: AnnexChoice (en|de)  partial_factor_table: ref<PartialFactorTable>
  gamma_w: kN/m3 = 10.0 (de) | 9.81 (en)
  groundwater: GroundwaterModel[]   // z_w, perched, artesian head, stage (event-driven water level series)
  investigation_depth: m
  boreholes: Borehole[]  cpts: CptProfile[]  spts: SptProfile[]  lab_tests: LabTest[]
  layers: SoilLayer[]  derived_parameters: ParameterDerivation[]
  footings: SpreadFoundation[]  piles: Pile[]  pile_groups: PileGroup[]
  retaining_walls: RetainingWall[]  embedded_walls: EmbeddedWall[]
  anchors: Anchor[]  slopes: Slope[]  slip_searches: SlipSearch[]
  uplift_cases: UpliftCase[]  hydraulic_cases: HydraulicHeaveCase[]
  dewatering: DewateringSystem[]  construction_stages: Stage[]
SoilLayer (existing) + z_top, z_bot, soil_class: Din18196, gamma, gamma_prime, gamma_sat,
  phi_prime_deg, cohesion_effective, cohesion_undrained, oedometric_modulus, poisson_ratio,
  cpt_qc, spt_n + new: cc, cs, c_alpha, e0, ocr, sigma_p, cv, k, delta_interface, colour
CharacteristicValueSpec  parameter: enum, mode: cautious|statistical, samples: f64[], v_x?: f64, result: f64
SpreadFoundation (existing) + shape: strip|rect|circle, position: (x,y,z), rotation, base_inclination_deg,
  load_cases: FoundationLoadCase[]  bearing_method: annex-d|din4017  include_depth_factors: bool  slope_beta_deg
FoundationLoadCase (existing) + ecc_b/l derived (not stored), design_situation, combination_ref
Pile (existing) + axis: line3, cross_section: circle|square|h|tube, install: driven|bored|cfa|micro,
  test_profiles: PileTestProfile[]  test_kind: static|ground|dynamic, n_tests, xi_set, neg_skin_layers: id[]
PileGroup  piles: ref<Pile>[], cap: Rect{lx,ly,t,z}, loads: LoadCase[], layout: grid(nx,ny,sx,sy)|free
RetainingWall (existing) + kind: gravity|cantilever|l-shape|gabion, geometry: polygon2d, backfill_slope_deg,
  wall_batter_deg, surcharges: Surcharge[], water_left/right, drainage: bool, earth_pressure_mode: active|at-rest|mobilised
EmbeddedWall  profile: sheet|bored|diaphragm, EI, t_wall, stages: Stage[], props: Prop[], anchors: ref<Anchor>[], toe_level
Surcharge  kind: uniform|strip|line|point|compaction, magnitude, extent
Anchor  kind: grouted|screw|deadman, inclination_deg, free_len, fixed_len, d_fixed, prestress, tests: AnchorTest[]
Slope (existing) + polyline2d: points[], water_table_polyline[], tension_crack_depth, seismic_kh, surcharges
SlipSearch  method: bishop|janbu|fellenius|wedge, grid{x0,y0,nx,ny,dx,dy}, radii[], slices: int=40, result: critical_circle
UpliftCase (existing) + hydraulic: head_h, uplift_area, anchors: ref<Anchor>[]
HydraulicHeaveCase  delta_h, flow_path, layer_ids, filter: FilterSpec
DewateringSystem  wells: Well[], q_total, drawdown_target, radius_method: sichardt|kusakin
PartialFactorTable  annex, approach, rows: (limit_state, situation, factor_name, value)
CheckResult  id, clause, label, ed, rd, mu, status, remedies[], explanation: Localized
```

### 4.2 EN 1998 entities

```
En1998Snapshot (existing) { annex: en|de, site: En1998Site, buildings, bridges, assessments, silos, tanks, foundations, retaining_walls, towers }
En1998Site (existing) + coords?: (lat,lon), vs_profile: VsLayer[], vs30 (derived), magnitude_ms, ground_type_override, topography: S_T, damping_xi
En1998Building (existing) + position/rotation, storeys[] (existing) with polygon_plan: polygon2d, elevation z,
  elements: StructuralElement[]  (columns, beams, walls, braces: id, nodes, section_ref, material, stiffness EI/k,
    mrd_top, mrd_bot (capacity), dissipative: bool)
  joints: Joint[] (ids, connected elements, mrb_sum, mrc_sum derived)
  diaphragms: Diaphragm[]  nonstructural: NonStructuralElement[] (mass, Ta, qa, gamma_a, z)
  adjacent_buildings: Neighbour[]  analysis_method: lateral|modal|pushover|time-history
  seismic_mass_override?: per-storey
En1998Storey (existing: height_m, permanent_gk_n, correlated_occupancy, variables, stiffness_x/y, centre of mass/stiffness, drift_x/y, shear_resistance_n)
  + masses derived; plan_polygon; torsion_stiffness K_theta
En1998System (existing) + alpha_u_alpha_1, wall_alpha0, torsionally_flexible, regularity_override_reason
En1998Member (existing)
En1998Bridge (existing) + piers: Pier[], bearings: Bearing[], deck: Deck{mass,length}, abutments, isolators: Isolator[], spans[]
Pier  height, section, Lv, hinge_mrd, shear_rd, q_derived, location
Bearing  kind, d_rd, k_h, link_length, gap
En1998Assessment (existing: knowledge_level, limit_state, supported_building_id, r_k_n, gamma_el) + members to assess, ConfidenceFactor derived
En1998Silo, En1998Tank (existing) + geometry profile, content density, wall thickness, anchorage, foundation_type
En1998Foundation (existing) + soil_profile_ref, ties[], piles_ref, liquefaction_cases: LiqCase[]
LiqCase  layer_ref, depth_z, n1_60, fc_percent, gw_depth, ms
En1998RetainingWall (existing) + wall_movement_class, slope_beta, delta, kh_override
En1998Tower (existing) + mass_profile: (z,m)[], ei_profile: (z,EI)[], damping, guys: Guy[]
SeismicResult (derived read-model) { spectrum_samples, periods, mode_shapes, storey_forces, drifts, theta, capacity_ratios }
```

### 4.3 Events (mutations; selection)

`LayerSplit`, `LayerSetProperty{layer, field, value}`, `GroundwaterSet{z_w, stage}`, `BoreholeAdd`, `FootingMoved{id, x,y}`, `FootingResized{id, B, L, d}`, `LoadChanged{case, field, value}`, `PileAdded/Moved/Removed`, `PileGroupLayoutSet`, `WallPolygonVertexMoved`, `SurchargeAdded`, `SlipCentreMoved{x,y,R}`, `StageAdded`, `StoreyAdded/Removed`, `StoreyMassSet`, `StiffnessSet`, `CentreOfMassMoved`, `SystemQSet`, `ZoneSet`, `GroundComboSet`, `ImportanceSet`, `PeriodMarkerAdded`, `ElementCapacitySet`. Each event is local-persisted (persisted local-only) while preview drag state is ephemeral local-only; shared edits are persisted shared. Undo/redo by compensating events. No CRUD endpoints.

### 4.4 Language-agnostic test assets

- `🧪️tests/fixtures/*.json`: input snapshot + expected `CheckResult[]` (as in section 7); consumed by Rust, Python oracle and any future language.
- `🔮️oracles/oracle_en1997.py`, `oracle_en1998.py`: independent re-implementation + third-party libs: scipy.optimize (Bishop fixed-point), numpy.linalg.eigh (modal), scipy.stats.t (k_n), `pysheeter`-style or `geotecha` (consolidation), `PyNite`/`openseespy` (cross-check modal and pushover where installed), `numpy` Boussinesq; Python `-I` mode, results written to `🗑️generated/`.

## 5. Visualisation Targets

All viewports share the `🧰️framework` 3D scene graph (orbit/pan/zoom, snapping, selection, transform gizmos). Colour scales are accessible (colour-blind-safe viridis / diverging blue-orange), plus numeric labels and patterns (never colour alone); DE/EN labels; keyboard operable (arrow keys nudge values, Tab cycles entities, Esc cancels a drag). Utilisation colours: μ <= 0.7 green, 0.7-0.9 yellow-green, 0.9-1.0 amber, > 1.0 red with hatching.

### 5.1 EN 1997 viewports

| Id | View | Contents | Interactions |
|---|---|---|---|
| V97-1 | 3D soil strata with foundation | extruded soil block, layered coloured bands per DIN 4023 hatching/colour, water table plane with transparency, foundation box, load arrows (V,H,M) scaled, eccentric load point and effective area `A'` (hatched rectangle) on the base, `B'`/`L'` dimension lines, influence depth `z_lim` surface | drag layer boundaries (z_top/z_bot), drag water plane, drag/resize foundation handles (B, L, d), drag load vector tip (H), eccentricity handle |
| V97-2 | Bearing failure mechanism | 2D section wedge: active wedge under base (angle 45+φ/2), Prandtl radial fan (log-spiral), passive wedge, with the slip lines coloured by mobilisation; failure mode label (general/punching/local) | toggles for factors s,i,b,d; show q', γ B' contributions as stacked bar |
| V97-3 | Earth pressure diagrams | wall section with pressure polygon `e_a(z)`, surcharge rectangles, water pressure, resultants with arrows and lever arms, passive diagram on toe side, net pressure, shear/moment curves for embedded wall | drag surcharge extent/magnitude, drag backfill slope, change δ via slider, mobilisation slider for passive |
| V97-4 | Slip circles | slope cross-section with slices (vertical bars, colour by Fs contribution), critical circle thick, grid of centres as heat map of F_s (contour), sensitivity to water table | drag circle centre, radius handle, drag water table polyline, drag slope toe/crest vertices |
| V97-5 | Pile group in 3D with layered soil | cylinder piles, cap, transparent layered soil, skin-friction arrows per layer, negative skin friction zone highlighted, neutral plane marker, load vector on the cap, block failure prism wireframe, per-pile utilisation heat-map (axial N_i), lateral deflection curve | drag pile to move, add via grid tool, drag cap load, slider for length per pile |
| V97-6 | Settlement contour | 3D dish surface under footings (exaggeration slider), contour lines (mm) and colour map on plan, influence depth bulb of stress isobars (σz/σ0 = 0.5, 0.2, 0.1), time-settlement 2D curve for consolidation with `t90` marker | drag footing position to see interaction, drag layer E modulus |
| V97-7 | Hydraulic / uplift | flow lines + equipotentials of the simple flow net, exit gradient arrow, uplift pressure diagram under base, seepage prism | drag h differences, wall toe |
| V97-8 | Anchored wall | wall, anchors with free/fixed lengths, Kranz wedge, forces | drag anchor angle/length |
| V97-9 | Dewatering | plan view of wells, drawdown cone surface, contour | drag wells |
| V97-10 | Ground investigation | CPT/SPT profile plots (qc, fs, Rf, N, Ic), correlated φ'/cu series, soil-behaviour chart | drag cut-off depth, select soil layers |

### 5.2 EN 1998 viewports

| Id | View | Contents | Interactions |
|---|---|---|---|
| V98-1 | 3D building with storey masses | wireframe/solid building, floor slabs, storey masses as spheres/discs scaled to m_i, centres of mass (blue) and stiffness (red) with eccentricity vector, torsional radius ellipse, accidental eccentric points `±e_ai` | drag centre of mass/stiffness, drag storey mass, add storeys, edit heights |
| V98-2 | Mode shapes (animated) | selectable mode n; vertex displacement `u(t) = A φ_n sin(ω_n t)` with scale slider; colour by displacement; period/effective-mass chip; respects reduced-motion (static envelope + slider) | scrub phase, switch modes, amplitude |
| V98-3 | Storey drift bars | horizontal bars per storey showing `d_r ν/h` in ‰ with limit lines 5, 7.5, 10 ‰, θ markers (0.1, 0.2, 0.3), colour by utilisation; per direction toggle x/y | click storey to highlight in 3D |
| V98-4 | Response spectrum 2D | log or linear axes: Se, Sd (and vertical), TB/TC/TD vertical lines, period markers for T1, T2, T3, 2TC, 4TC, current Sd(T1) and Fb on y, ground-type family curves ghosted, q slider shows Sd shape live; displacement spectrum alternate tab | drag markers (T1 override), drag q, switch ground types |
| V98-5 | Seismic zone map | map of Germany with zones 0-3 polygons (map asset), selected site pin, tooltips with agR and subsoil class R/T/S overlay; EN: free agR input | click map to pick zone/site, hover shows numbers |
| V98-6 | Capacity design joint view | joint cut-out with M_Rb sum (arrows) vs M_Rc sum, ratio dial, hinge plan (beam hinges vs column hinges) colours | edit beam/column M_Rd, section |
| V98-7 | Plastic mechanism | storey-collapse vs beam-sway mechanism overlay | |
| V98-8 | Pushover capacity curve (EN 1998-3) | V-d curve, target displacement, bilinear idealisation, performance points NC/SD/DL | drag target |
| V98-9 | Tank/silo sloshing | tank section with impulsive/convective masses, wave height arrow, pressure profile | fill-level slider |
| V98-10 | Liquefaction profile | CSR vs CRR depth plot, FS bar with 1.25 limit line, layer shading | drag GW |
| V98-11 | Mononobe-Okabe wall | wall with static + dynamic pressure diagrams, kh slider | |
| V98-12 | Tower/mast | cantilever with mass nodes, mode 1 shape, shear and moment diagrams | |
| V98-13 | Bridge | deck + piers + bearings, displacement arrows, gap bars | |
| V98-14 | Seismic joint gap | two buildings with displaced profiles, gap dimension | |

## 6. Realtime Editor Interactions and Live-Recompute Targets

General: each edit emits an event, the compute graph (`💡️inferences`) recomputes only invalidated nodes (incremental), progress reporting and cancellation for expensive jobs (slip circle grid search, modal, pushover). Budget: p95 interaction -> updated utilisation within 16 ms for closed-form checks (single frame), 100 ms for multi-check dashboards, <= 250 ms for a 40-slice Bishop single circle, <= 2 s (progress bar, cancel button) for 50×50×10 grid search (run in worker, preview a coarse 10×10 grid meanwhile), <= 100 ms for modal analysis up to 30 storeys (3 DOF each), <= 500 ms for pushover. Heavy results display "stale" watermark until update arrives, never freeze UI. Short connection loss: edits queue as local events, replay on reconnect (persisted shared).

| Id | Interaction | Recompute (live) |
|---|---|---|
| ED-1 | Drag foundation width/length/depth handle | A', all s,i,b,d factors, R_k, R_d, μ, settlement dish, sliding, eccentricity core |
| ED-2 | Drag load tip or type M/V | eccentricities, effective area patch, bearing μ, base pressure triangle, gap warning |
| ED-3 | Drag layer interface; edit φ', c', γ from inline chips (units on chip, EN/DE label) | stress profile, bearing, settlement, earth pressure, slope Fs; characteristic value stats badge |
| ED-4 | Drag water level | `γ'` swap, uplift μ, HYD μ, effective pressures, slope Fs |
| ED-5 | Switch design approach / annex | factor table re-applied, all μ re-evaluated with diff highlight vs before |
| ED-6 | Paste CPT/SPT table | auto-derived φ', cu, E series with validity chips; "accept as characteristic" action |
| ED-7 | Drag retaining wall vertices / backfill slope / δ slider | Ka/Kp, pressure diagram, overturning, sliding, bearing, base pressure |
| ED-8 | Embedded wall: drag toe level, prop level | required embedment, moment diagram, prop force, mobilised passive % (live) |
| ED-9 | Move slip circle centre/radius | Fs for the circle, grid heat map updates incrementally |
| ED-10 | Pile: change length/diameter/count; add/move piles | R_c,k via ξ, group efficiency, N_i distribution, block failure, downdrag neutral plane |
| ED-11 | Edit storey mass / stiffness / height | T1, Sd(T1), Fb, Fi, V_i, drift, θ, regularity flags, modal re-solve (debounced 50 ms), spectrum markers |
| ED-12 | Drag q slider or pick system/material/ductility class | q formula steps shown, spectrum Sd, Fb, capacity-design requirements set toggled |
| ED-13 | Pick zone / ground combo / importance | agR, ag, S, TB/TC/TD, spectra, all downstream |
| ED-14 | Drag centres of mass/stiffness | e0, r, regularity (e0 <= 0.3 r, r >= ls), torsional amplification δ per element |
| ED-15 | Edit member M_Rd values at joints | ΣMRc/ΣMRb ratio dial, fail highlights, remedies ("increase column M_Rd by x kNm") |
| ED-16 | Mode scrubbing | animated shapes, M_eff table, SRSS vs CQC choice |
| ED-17 | Tank fill-level slider | masses, T_c, Q, M, wave height |
| ED-18 | Liquefaction: drag GW, N slider | CSR/CRR/FS live |

Inline feedback: every field shows its utilisation chip when it influences a check; hovering shows the governing formula with substituted values (rendered, localised); remedies (auto-suggestions: "increase B to 2.3 m", "increase embedment by 0.4 m", "choose DCM", "stiffen storey 1") are one-click events. No raw JSON exposed: JSON only in an advanced inspector. Fields are validated per-keystroke, units convertible, undo/redo, multi-select edits, keyboard accessible, ARIA-live announcements for status changes (pass/fail), high-contrast and reduced-motion variants, language switch EN/DE without default language (first-run chooser, English listed first, then German).

## 7. Worked Examples (hand-checked reference numbers)

Reference tolerance: closed forms 1e-6 relative; hand-rounded values below are 4 significant figures. Each example is a fixture `{input snapshot, expected}`; every EN 1997 and EN 1998 example listed with `kind: pass|fail`.

### 7.1 EN 1997 (15 examples)

| Id | Case | Given | Expected |
|---|---|---|---|
| EX97-01 | Strip footing drained, DE GZ 1B centric | B = 2.0 m, L = ∞, d = 1.0 m, φ'k = 30°, c' = 0, γ1 = γ2 = 18 kN/m³, G_k = 300 kN/m, Q_k = 100 kN/m | Nq = 18.401, Nc = 30.140, Nγ = 20.093; sq = sγ = 1; `R/A' = 18·18.401 + 0.5·18·2·20.093 = 331.22 + 361.68 = 692.90 kPa`; `R_k = 1385.8 kN/m`; `V_d = 1.35·300 + 1.5·100 = 555 kN/m`; `R_d = 1385.8/1.40 = 989.9`; μ = 0.5607, pass |
| EX97-02 | Square footing, undrained | B = L = 3.0 m, d = 1.0 m, cu = 60 kPa, γ = 19 kN/m³, H = 0, α = 0 | sc = 1.2; `R/A' = (π+2)·60·1.2 + 19 = 370.19 + 19 = 389.19 kPa`; R_k = 3502.8 kN; R_d (γR,v = 1.40) = 2501.9 kN |
| EX97-03 | Eccentric rectangle, c'-φ' | B = 2.5, L = 4.0, e_B = 0.3, e_L = 0.4, d = 1.2, φ' = 30°, c' = 5 kPa, γ = 19, V = 1500 kN centric part | B' = 1.9, L' = 3.2, A' = 6.08 m²; sq = 1.29688, sγ = 0.82188, sc = 1.31394; `R/A' = 1040.18 kPa`; R = 6324.3 kN |
| EX97-04 | Same with inclined load | H_B = 200 kN, V = 1500 kN | `B'/L' = 0.59375`, m = mB = 1.6275; `t = 1 - H/(V + A' c' cotφ') = 0.87119`; iq = 0.79898, iγ = 0.69606, ic = 0.78743; `R/A' = 798.12 kPa`; R = 4852.6 kN |
| EX97-05 | Sliding, DE | V_k = 800 kN (permanent), δ = φ' = 30°, H_G = 100, H_Q = 80, no passive | H_d = 255.0; R_t,k = 461.88; R_d = 461.88/1.10 = 419.89; μ = 0.6073 pass |
| EX97-06 | Overturning EQU gravity wall | W = 300 kN/m (lever 1.2 m), E_ah,G = 90 kN/m (1.5 m), Q-surcharge effect 20 kN/m (2.2 m) | DE (0.95/1.05/1.5): M_stb = 342.0, M_dst = 141.75 + 66.0 = 207.75, μ = 0.6075 pass; EN rec. (0.9/1.1/1.5): 324.0 vs 214.5, μ = 0.6620 pass |
| EX97-07 | Uplift UPL | slab 15 kPa + soil 18 kPa per m², water head 3 m above base (30 kPa) | G_stb,d = 0.9·33 = 29.7 kPa < 30.0 kPa, μ = 1.0101 fail; remedy: add at least 0.33 kPa characteristic ballast (required G_stb,k = 30/0.9 = 33.33 kPa) |
| EX97-08 | Hydraulic heave | Δh = 2.5 m, d = 5 m (i = 0.5), γ' = 9.5, γw = 10 | S_dst,k = i γw = 5.0 kN/m³; DE: 1.8·5.0 = 9.0 > 0.9·9.5 = 8.55, μ = 1.0526 fail; EN rec.: 1.35·5.0 = 6.75 ≤ 8.55, μ = 0.789 pass; i_crit = 0.95 |
| EX97-09 | Settlement stress-strain | circular flexible footing a = 1.5 m, σ0 = 200 kPa, layers 0-2 m E = 8 MPa, 2-4 m E = 12 MPa, mid-depths 1 m and 3 m, `σz = σ0[1 - (1/(1+(a/z)²))^{3/2}]` | σz(1) = 165.86, σz(3) = 56.89 kPa; s1 = 165.86·2/8000 = 41.47 mm, s2 = 56.89·2/12000 = 9.48 mm; s = 50.95 mm |
| EX97-10 | Primary consolidation | NC clay H = 3 m, Cc = 0.3, e0 = 0.9, σ'0 = 60 kPa, Δσ = 80 kPa, cv = 1e-7 m²/s, double drainage (H_dr = 1.5 m) | `s = 0.3·3/1.9·log10(140/60) = 0.1743 m`; `t90 = 0.848·1.5²/1e-7 = 1.908e7 s = 220.8 d` |
| EX97-11 | Active earth pressure Coulomb | H = 5 m, vertical wall, level ground, γ = 19, φ' = 30°, δ = 20°, q = 10 kPa | Ka = 0.29731; e_top = 2.973, e_bot = 31.218 kPa; E_a = 85.48 kN/m, E_ah = 80.32, E_av = 29.24; Rankine δ = 0: Ka = 1/3; cohesive Rankine c = 10, φ = 30: z0 = 1.823 m, e_a(5 m) = 20.12 kPa (γ = 19) |
| EX97-12 | Pile static tests (n = 3) | R_c,m = 1450, 1500, 1650 kN | mean = 1533.33, min = 1450; DE ξ1 = 1.15, ξ2 = 1.05: 1333.33 vs 1380.95 -> R_c,k = 1333.33; R_c,d = /1.10 = 1212.12 kN. EN rec. ξ1 = 1.20, ξ2 = 1.05: 1277.78 -> R_c,d (γt = 1.1) = 1161.62 kN |
| EX97-13 | Slope undrained circle & infinite slope | circle R = 6 m, 80° arc, cu,k = 40 kPa, W = 720 kN/m at x = 1.5 m; infinite slope β = 25°, φ' = 30° dry | M_R = 40·6·(80π/180)·6 = 2010.6 kNm/m, M_D = 1080; DE GZ 1C: M_R,d = 2010.6/1.25 = 1608.5, μ = 0.6714 pass; infinite slope: Fs = tan30/tan25 = 1.2381; design tanφ'd = 0.46188 -> μ = 0.46631/0.46188 = 1.0096 fail |
| EX97-14 | Parameter derivation | CPT: qc = 12 MPa, σ'v0 = 100 kPa; qt = 2400 kPa, σv0 = 60, Nkt = 15; SPT (N1)60 = 20; samples φ' = 30,32,34,31,33 | Kulhawy-Mayne φ' = 17.6 + 11·log10(120/1) = 40.47°; cu = (2400 - 60)/15 = 156 kPa; Hatanaka-Uchida φ' = sqrt(20·20) + 20 = 40°; statistical: mean 32, s = 1.5811, Vx = 0.04941, k_n = 2.132·sqrt(1/5) = 0.9535 -> X_k = 32(1 - 0.9535·0.04941) = 30.49° |
| EX97-15 | Dewatering single well | k = 1e-4 m/s, H = 10 m, h = 6 m (s = 4), r = 0.3 m, Sichardt R = 3000·4·sqrt(1e-4) = 120 m | `Q = π·1e-4·(100 - 36)/ln(120/0.3) = 0.020106/5.9915 = 3.356e-3 m³/s = 12.08 m³/h` |

(EX97-01…EX97-04 exercise DA1/DA2/DA3 toggles by parametrising: DA1-C2 uses tanφ'd = tan30/1.25 = 0.46188, φ'd = 24.79°; cu,d = 60/1.4 = 42.86.) Each example additionally records fail-twin fixtures (modified input flipping the verdict) as required by the existing `compliant_*`/`noncompliant_*` convention.

### 7.2 EN 1998 (18 examples)

Common site for EX98-01…09: DE zone 2 (agR = 0.6 m/s²), importance II (γI = 1.0), ground C-R (S = 1.5, TB = 0.05, TC = 0.30, TD = 2.0).

| Id | Case | Given | Expected |
|---|---|---|---|
| EX98-01 | DE elastic spectrum | above, ξ = 5 % (η = 1) | Se(0) = 0.9; Se(0.05) = 2.25; plateau 2.25 m/s² up to 0.30 s; Se(0.5) = 1.35; Se(1.0) = 0.675; Se(2.0) = 0.3375; Se(3.0) = 0.15 |
| EX98-02 | EN spectrum Type 1, ground C, importance III | agR = 0.25 g = 2.4525, γI = 1.2: ag = 2.943; S = 1.15, TB = 0.20, TC = 0.60, TD = 2.0 | plateau = 2.943·1.15·2.5 = 8.461 m/s²; vs30 for layers 5 m@150, 10@250, 15@400 = 30/(0.0333+0.04+0.0375) = 270.68 m/s -> ground C |
| EX98-03 | Behaviour factor | RC multi-storey multi-bay frame, DCM, regular elevation | αu/α1 = 1.3, q0 = 3.0·1.3 = 3.9, kw = 1 -> q = 3.9; irregular elevation -> 0.8·3.9 = 3.12; wall system α0 = Σh/Σl = 2.0 -> kw = (1 + 2)/3 = 1.0; α0 = 1.0 -> kw = 0.667 |
| EX98-04 | Design spectrum DE, q = 3.9 | | Sd plateau = 0.6·1.5·2.5/3.9 = 0.5769; Sd(0.5) = 0.3462; Sd(3 s) = max(0.6·1.5·2.5/3.9·0.3·2/9 = 0.03846; 0.2·0.6 = 0.12) = 0.12 (lower bound governs) |
| EX98-05 | Mass for analysis | office (B, ψ2 = 0.3), correlated occupancies φ = 0.8, G_k = 3000 kN, Q_k = 1000 kN | ψE = 0.24; m = (3000 + 0.24·1000)/9.81 = 330.28 t; roof (φ = 1.0): ψE = 0.30 |
| EX98-06 | Lateral force method, 4 storeys of 3 m, m = 400 t each, q = 3.9 | H = 12 m, Ct = 0.075 | T1 = 0.075·12^{0.75} = 0.4836 s ≤ 2TC = 0.6 -> λ = 0.85; Sd(T1) = 0.5769·0.3/0.4836 = 0.3579 m/s²; Fb = 0.3579·1600 t·0.85 = 486.78 kN; Fi (z·m) = 48.68, 97.36, 146.03, 194.71 kN; V_i = 486.78, 438.10, 340.74, 194.71 kN |
| EX98-07 | P-Δ and drift | storey 1 of EX98-06, d_e = 1.2 mm, h = 3 m, P_tot = 1600·9.81 = 15696 kN | d_r = 3.9·1.2 mm = 4.68 mm; θ = 15696·0.00468/(486.78·3) = 0.0503 ≤ 0.10 ok; drift: ν = 0.5, d_r ν = 2.34 mm ≤ 0.005·3000 = 15 mm, ratio 0.156 pass. Fail twin: d_e = 4 mm -> d_r = 15.6 mm, θ = 0.1677 (amplify 1/(1 - θ) = 1.2015) |
| EX98-08 | Accidental torsion | plan 24 m × 12 m, action direction y so the perpendicular floor dimension is L_i = 24 m; outermost resisting element at x = 12 m from the centre of mass, Le = 24 m | e_ai = 0.05·24 = 1.2 m; δ = 1 + 0.6·12/24 = 1.30 |
| EX98-09 | Modal 2 DOF shear building | m1 = m2 = 100 t, k1 = k2 = 40 MN/m, q = 2.0, same site | ω = 12.3607, 32.3607 rad/s; T = 0.50832, 0.19416 s; φ1 = (0.618, 1), φ2 = (-1.618, 1); Γ1 = 1.17082, M_eff1 = 94.72 % ; M_eff2 = 5.28 %; Sd(T1) = 0.6640 m/s², Sd(T2) = 1.125; V_b1 = 125.78 kN, V_b2 = 11.88 kN; SRSS V_b = 126.34 kN; F modal 1 = (48.04, 77.74) kN; CQC r = 2.618, ρ12 = 0.0088 (SRSS ≈ CQC) |
| EX98-10 | Capacity design joint | interior joint, one seismic direction: beams on both sides with M_Rb = 300 kNm each (ΣM_Rb = 600 kNm); columns above and below M_Rc = 400 kNm each; beam shear case: M_Rb,hog = 300, M_Rb,sag = 200 kNm, l_cl = 5 m, gravity shear 60 kN; column l_cl = 2.6 m | 800 ≥ 1.3·600 = 780, ratio 800/780 = 1.0256 pass. Fail twin: M_Rc = 380 -> 760 < 780 (0.974). Shear: beam DCM `V = (300 + 200)/5 + 60 = 160 kN`; column DCM `V = 1.1(400 + 400)/2.6 = 338.5 kN` |
| EX98-11 | Chord rotation (EN 1998-3) | h = 0.5 m, d = 0.46 m, z = 0.414 m, L_v = 1.5 m, φ_y = 0.0060 1/m, d_b = 20 mm, f_y = 500 MPa, f_c = 25 MPa, a_V = 1; KL2 CF = 1.20 | `θ_y = 0.0060·(1.5 + 0.414)/3 + 0.0013·(1 + 1.5·0.5/1.5) + 0.13·0.0060·0.020·500/sqrt(25) = 0.003828 + 0.00195 + 0.00156 = 0.007338` rad (φ_y in 1/m, d_b in m, f_y and f_c in MPa) |
| EX98-12 | Tank Housner | R = 10 m, H = 8 m, zone 2 C-R | tanh(1.84·0.8) = 0.89996; ω_c = 1.2745 rad/s, T_c = 4.930 s; m_c/m = 0.230·1.25·0.89996 = 0.2587; η_c = sqrt(10/5.5) = 1.3484; Se(T_c) = 0.6·1.5·2.5·1.3484·0.3·2/4.93² = 0.0749 m/s² |
| EX98-13 | Mononobe-Okabe | H = 5 m, γ = 19, φ = 30°, δ = 0, ψ = β = 0, kh = 0.1, kv = 0 | θ = 5.7106°; K_AE = 0.39655; E_d = 0.5·19·25·0.39655 = 94.18 kN/m; static 79.17; ΔE = 15.02 kN/m acting 0.6H above base |
| EX98-14 | Liquefaction | z = 6 m, GW at 1 m, γ = 18 above, γ_sat = 19, (N1)60 = 14, zone 2 C-R (ag S = 0.9 m/s²) | σv0 = 113 kPa, u = 49.05, σ'v0 = 63.95; CSR = 0.65·0.9/9.81·113/63.95 = 0.1054; CRR = 0.05 + 0.1037 + 0.0015 - 0.005 = 0.1502; FS = 1.425 ≥ 1.25 pass. Fail twin: N = 10 -> CRR = 1/24 + 0.0741 + 0.0024 - 0.005 = 0.1132, FS = 1.074 fail |
| EX98-15 | Tower (cantilever, uniform mass) | H = 40 m, W = 2000 kN, T1 = 1.2 s, q = 1.5, zone 2 C-R, λ = 1 | Sd(1.2) = 0.6·1.5·2.5/1.5·0.3/1.2 = 0.375 m/s²; Fb = 0.375·2000/9.81 = 76.45 kN; M_b = Fb·2H/3 = 2038.7 kNm |
| EX98-16 | Bridge pier & displacement | pier L_v = 8 m, h = 1.6 m; deck T = 1.0 s; same site | αs = 5 ≥ 3 -> λ = 1, q = 3.5; d_E = Se(1.0)(T/2π)² = 0.675·0.025330 = 0.01710 m; short pier αs = 2 -> q = 3.5 sqrt(2/3) = 2.858 |
| EX98-17 | Seismic joint | d1 = 40 mm, d2 = 30 mm | same level: 0.7·sqrt(40² + 30²) = 35 mm (floor levels same); different level 50 mm |
| EX98-18 | Non-structural | α S = 0.6·1.5/9.81 = 0.0917, z/H = 1, Ta = T1 (ratio 1), γ_a = 1.5, q_a = 2 | S_a = 0.0917[3·2/(1 + 0) - 0.5] = 0.0917·5.5 = 0.5046; F_a = 0.5046·W_a·1.5/2 = 0.378 W_a |

Notes on hand checks: EX98-10 uses γRd = 1.3 (joint), 1.0 (beam DCM), 1.1 (column DCM). EX98-11 term by term: `φ_y(L_v + a_V z)/3 = 0.0060·1.914/3 = 0.003828`; `0.0013(1 + 1.5 h/L_v) = 0.0013·1.5 = 0.00195`; `0.13 φ_y d_b f_y/sqrt(f_c) = 0.13·0.0060·0.020·500/5 = 0.00156`. EX98-18 satisfies the lower bound `S_a >= α S`.

## 8. Test Strategy

- For each catalogue id at least one language-agnostic fixture (JSON) plus a Python oracle value computed with an independent code path (third-party: scipy, numpy, pandas where useful; hand reference values in section 7 are cross-asserted).
- Property tests: monotonicity (μ increases with H, decreases with B), symmetry (B,L swap invariance), limit cases (φ'→0 consistency between drained and undrained forms, β→0 for Bishop = Fellenius when slices→1, zone 0 -> no check), factor tables round trip, spectrum continuity at TB, TC, TD.
- Every `[V]` NDP has a negative control test that fails if the value is changed silently (golden NDP table hash).
- Visual tests: scene-graph snapshot tests (geometry counts, bounding boxes, colour class per utilisation) and screenshot goldens for 2D diagrams; accessibility tests (labels in EN and DE exist, keyboard path covers every handle).
- Performance budget tests per section 6.
- Progress and cancellation tests for grid search, modal, pushover.

## 9. Delivery Order and Definition of Done

1. Partial-factor and NDP assets (section 1), characteristic values, strata and stresses.
2. Bearing/sliding/EQU/UPL/HYD/settlement, then earth pressure and walls, slope, pile, anchor, dewatering, parameter derivation.
3. Spectrum, ground types, zone map, q, regularity, LFM, torsion, P-Δ, drift, modal, capacity design.
4. Parts 2-6, assessment, tanks/silos, liquefaction, towers.
5. Viewports and editor interactions per sections 5-6 in parallel per entity.
Done when every catalogue id has implementation, localisation EN/DE, fixture + oracle, viewport, live-recompute target, documented `[V]` closing against the edition, and all examples of section 7 pass (including fail twins).
