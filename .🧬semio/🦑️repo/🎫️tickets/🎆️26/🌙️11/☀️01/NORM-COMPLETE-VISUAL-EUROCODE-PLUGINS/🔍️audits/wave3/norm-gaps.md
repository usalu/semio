# 🔍️ Norm Gap Audit, EN 1990 to EN 1999 (Wave 3, W3-NORM-GAPS)

Read-only audit. No code was edited and no build or test was run. Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`.

## Method

- Targets: every id of each code's spec-trace `specIds` (the set the trace declares as the target spec). All `specIds` appear in the target markdown (`📐️specs/*-target.md`), and no trace item is missing from `specIds`.
- Implementation evidence: the trace `items[].realisation` of `🗿️artifacts/<code>/…/🧭️spec-trace/🔣️.json`, checked against the artifact's non-test Rust code (`🧬️schema/**`, `⚖️compliance`, excluding `spec-trace`, `tests`, `showcase`, `scene`, `viewer`, `editor`, `examples`, fixtures).
  - `check` rows: a fragment is implemented when it is a literal in non-test code. A row is implemented when its quoted catalogue id (for example `"GI-02"`) is a literal in non-test code, even if the trace fragment is absent (trace stale).
  - `design-case` rows: implemented when the quoted entry id is in the catalogue.
  - `data` rows: implemented when the leading token of the data path is in non-test code.
  - `not-modelled` rows: missing, or blocked when the reason cites an unverified or printed value.
- Blocked: implemented rows whose text or target row says `[V]`, `unverified`, `provisional` or `print`, and not-modelled rows whose reason cites print or unverified data.
- Partial: a check row with some fragments unmatched, or an implemented row whose text states a scope limit ("not modelled", "not carried", "special study", "out of scope", "not covered", "not used").
- Spec line: first line of the target markdown where the id appears (table row preferred). Clause as written in the trace.
- Caveat: the trace fragments are often stale (see Findings). Status is a static check, not a behavioural test. The per-code rows below are the result of the script run; the counts were not cross-checked against the reports.

## Per-code counts

| Code | Items | Implemented | Partial | Missing | Blocked on print/unverified | Trace-only share |
|---|---|---|---|---|---|---|
| en1990 | 30 | 23 | 7 | 0 | 0 | 5 stale-fragment rows |
| en1991 | 109 | 76 | 1 | 7 | 25 | 6 stale-fragment rows |
| en1992 | 160 | 157 | 2 | 0 | 1 | 0 stale-fragment rows |
| en1993 | 166 | 137 | 6 | 3 | 20 | 0 stale-fragment rows |
| en1994 | 54 | 52 | 1 | 0 | 1 | 0 stale-fragment rows |
| en1995 | 68 | 67 | 0 | 0 | 1 | 24 stale-fragment rows |
| en1996 | 37 | 30 | 0 | 0 | 7 | 9 stale-fragment rows |
| en1997 | 61 | 58 | 2 | 0 | 1 | 37 stale-fragment rows |
| en1998 | 68 | 33 | 5 | 7 | 23 | 0 stale-fragment rows |
| en1999 | 42 | 31 | 4 | 5 | 2 | 0 stale-fragment rows |
| **Total** | 795 | 664 | 28 | 22 | 81 | |

## Top 25 missing or partial items, ranked by engineering importance

Ranking order: safety-relevant ultimate-limit-state (ULS) and accidental items first (stability, fracture, plastic resistance, connections, seismic collapse), then fatigue and traffic, then SLS and design inputs. Only 22 items are "missing" in total; the list fills to 25 with partial items that carry ULS weight. EN 1991 E2.19 (derailment) is blocked in the status rules because its reason cites an unverified value, but it is a missing model and ranks here.

| Rank | Code | Id | Status | Spec line | Why it ranks here |
|---|---|---|---|---|---|
| 1 | en1999 | A-70 | missing | en1994-en1999-target.md:419 | Shell buckling: classical elastic critical stress, cylindrical shells (EN 1999-1-5 §6-7). Stability ULS for tanks, silos, tubes. |
| 2 | en1999 | A-61 | missing | en1994-en1999-target.md:414 | Sheeting bending, web crippling and shear (EN 1999-1-4 §6.1). Core ULS resistance for purlins and cladding. |
| 3 | en1999 | A-72 | missing | en1994-en1999-target.md:422 | Welded shells with HAZ reduction on axial compression (EN 1999-1-5). ULS of welded shells. |
| 4 | en1999 | A-12 | partial | en1994-en1999-target.md:300 | Net-section tension N_u,Rd = 0.9 A_net f_u/γM2 with holes not modelled (6.2.3). ULS fracture of bolted members. |
| 5 | en1993 | V-501 | missing | en1993-target.md:225 | Stress-range derivation (5.1/5.2) from bands or rainflow history; fatigue ULS input for every detail. |
| 6 | en1993 | V-259 | missing | en1993-target.md:253 | Cold-formed screw and blind-rivet connections (section 8). ULS of fastened joints not modelled. |
| 7 | en1993 | V-126 | partial | en1993-target.md:179 | Buckling length Lcr = kL with nomogram k from G_A, G_B not modelled; buckling lengths are direct inputs (6.3.1.3). ULS stability. |
| 8 | en1993 | V-407 | partial | en1993-target.md:204 | Eccentric bolt groups (3.12): bolt coordinates and eccentricity missing, so group resistance is not derived. ULS joints. |
| 9 | en1993 | V-250 | partial | en1993-target.md:41 | Enhanced yield strength of cold-formed sections (3.2.2): f_u and corner data not carried. ULS resistance gain. |
| 10 | en1993 | V-029 | partial | en1993-target.md:73 | Plastic global analysis gate (5.6): class and restraint at hinges. ULS when plastic design is used. |
| 11 | en1992 | ST-03 | partial | en1992-target.md:261 | Strut-and-tie node capacity CCC/CCT/CTT (6.5.4); trace fragment stale, id present, verify CCC node. ULS. |
| 12 | en1992 | T-02 | partial | en1992-target.md:216 | Torsion links and longitudinal steel (6.3.2); trace fragment stale, id present, verify torsion-links. ULS. |
| 13 | en1991 | E2.19 | blocked-unverified | en1990-en1991-target.md:326 | Derailment design situations I and II (EN 1991-2 6.8). Accidental ULS for rail bridges; not modelled. |
| 14 | en1991 | E4.07 | missing | en1990-en1991-target.md:348 | Dust explosion pressure p_max on silos (EN 1991-4). Accidental ULS; not modelled. |
| 15 | en1991 | E2.20 | missing | en1990-en1991-target.md:326 | Rail load groups gr11 to gr31 (Table 6.11) for traffic ULS and SLS; not modelled. |
| 16 | en1991 | E4.06 | missing | en1990-en1991-target.md:348 | Thermal actions on silo walls (ΔT); not modelled. |
| 17 | en1998 | GE-04 | missing | en1997-en1998-target.md:561 | Pore pressure increase and lateral-spreading flag (EN 1998-5 4.1.4, Annex B). Seismic ULS for liquefiable ground. |
| 18 | en1998 | BR-10 | missing | en1997-en1998-target.md:526 | Seismic isolation of bridges (EN 1998-2 §6). Seismic ULS; not modelled beyond spring stiffness. |
| 19 | en1998 | BR-11 | missing | en1997-en1998-target.md:527 | Foundation and abutment soil-structure interaction for bridges; not modelled. |
| 20 | en1998 | GE-12 | partial | en1997-en1998-target.md:569 | Pile in seismic ground: inertial and kinematic moments and bending resistance (EN 1998-5 5.4); partial. |
| 21 | en1998 | SEI-53 | partial | en1997-en1998-target.md:505 | Building isolation (T_eff, ξ_eff, d_cd) not modelled; only isolated bridges covered; seismic ULS. |
| 22 | en1994 | C-32 | partial | en1994-en1999-target.md:211 | Composite column fire, simple calculation model; tabulated data C-46 not modelled (EC4 F:4.3). Accidental ULS. |
| 23 | en1993 | V-011 | partial | en1993-target.md:43 | Ductility f_u/f_y ≥ 1.10 and ε_u ≥ 15 ε_y (3.2.2); elongation check not carried. ULS for plastic and global design. |
| 24 | en1991 | E2.18 | missing | en1990-en1991-target.md:326 | Aerodynamic effects from passing trains (6.7) not modelled; spec gives unverified p_k. |
| 25 | en1997 | GI-03 | partial | en1997-en1998-target.md:296 | CPT undrained shear strength c_u with cone factor N_kt (Annex D); partial. Geotechnical ULS input. |

Blocked items with high ULS weight that are not in the top 25 because they are blocked on print: EN 1994 C-46 (tabulated fire data, F:4.2), EN 1993 V-505 (fatigue detail Tables 8.1 to 8.10), EN 1993 V-609 (advanced fire methods), EN 1996 M-05 (anchorage bond length), EN 1996 M-31 (yield-line two-way bending), EN 1998 GE-06 (seismic bearing capacity, Annex F), EN 1998 BR-06 (seismic component combination), EN 1998 AS-06 (cyclic shear capacity).

## Missing and partial items (full list, clause and spec line)

### Missing and partial (clause, spec line, evidence)

| Code | Id | Clause | Status | Spec line | Requirement (abridged) | Evidence |
|---|---|---|---|---|---|---|
| en1990 | E90.06 | 3.2 | partial | en1990-en1991-target.md:59 | Design situations: persistent, transient, accidental and seismic (3.2), realised within the STR combination check | some check fragments absent: en1990.str., en1990.6.11b., en1990.6.12b.; id literal present in ⚖️en1990 |
| en1990 | E90.09 | 4.1.3 | partial | en1990-en1991-target.md:69 | Combination, frequent and quasi-permanent values ψ0 Q_k, ψ1 Q_k, ψ2 Q_k with ψ0 ≥ ψ1 ≥ ψ2 ≥ 0 (4.1.3), realised within the serviceability check family | some check fragments absent: en1990.str.; id literal present in ⚖️en1990 |
| en1990 | E90.14 | - | partial | en1990-en1991-target.md:80 | Definition of the limit states EQU, STR, GEO and FAT (6.4.1), realised within the EQU stability check | some check fragments absent: en1990.str.; id literal present in ⚖️en1990 |
| en1990 | E90.16 | 6.4.3.1-2 (6.9b)-(6.10)-(6.10b) | partial | en1990-en1991-target.md:85 | Fundamental combination for STR/GEO in the ultimate limit state (6.4.3.1-2, (6.10), (6.10a/b)), realised as the STR combination check | some check fragments absent: en1990.str.; id literal present in ⚖️en1990 |
| en1990 | E90.22 | Table A1.1 | partial | en1990-en1991-target.md:70 | ψ-factors for buildings (Table A1.1), realised within the STR combination check | some check fragments absent: en1990.str.; id literal present in ⚖️en1990 |
| en1990 | E90.23 | Tables A1.2(A) EQU, A1.2(B) STR/GEO, A1. | partial | en1990-en1991-target.md:120 | Partial factors for buildings in Tables A1.2(A), A1.2(B), A1.2(C) and A1.3, realised within the STR combination check | some check fragments absent: en1990.str.; id literal present in ⚖️en1990 |
| en1990 | E90.25 | Tables A2.1, A2.2, A2.3 | partial | en1990-en1991-target.md:70 | ψ-factors for bridges (Tables A2.1, A2.2, A2.3), realised as the traffic-group check | some check fragments absent: en1990.str.; id literal present in ⚖️en1990 |
| en1991 | E17.10 | EN 1991-1-7 Annex E | missing | en1990-en1991-target.md:314 | Annex E external explosions are not used by the spec and not modelled | not modelled |
| en1991 | E2.12 | 5 | partial | en1990-en1991-target.md:326 | Footbridge uniform load q_fk = 2.0 + 120/(L + 30) kN/m2 within 2.5 to 5.0 and horizontal force Q_flk = max(10 % of the total vertical load, 10 kN) per footbridge row (5.3 | some check fragments absent: en1991.2.footbridge-horizontal.; id literal present in 🏋️en1991 |
| en1991 | E2.18 | 6.7 | missing | en1990-en1991-target.md:326 | Aerodynamic effects from passing trains are not modelled | not modelled |
| en1991 | E2.20 | 6.9 | missing | en1990-en1991-target.md:326 | Rail load groups gr11 to gr31 are not modelled | not modelled |
| en1991 | E2.21 | 6.10 | missing | en1990-en1991-target.md:326 | Rail fatigue with the standard train mix is not modelled | not modelled |
| en1991 | E4.06 | EN 1991-4 (clause not stated by the spec | missing | en1990-en1991-target.md:348 | Thermal actions on silo walls are not modelled | not modelled |
| en1991 | E4.07 | EN 1991-4 (clause not stated by the spec | missing | en1990-en1991-target.md:348 | Dust explosion pressure is not modelled | not modelled |
| en1991 | E4.09 | Annex A, B, C | missing | en1990-en1991-target.md:348 | Silo wind, pressure annexes and fatigue are not modelled | not modelled |
| en1992 | T-02 | 6.3.2 | partial | en1992-target.md:216 | Torsion resistance with TRd,max = 2 ν α_cw fcd Ak tef sin θ cos θ and 1.0 ≤ cot θ ≤ 2.5, and the required ASw/s = TEd/(2 Ak fywd cot θ) and ΣAsl = TEd uk cot θ/(2 Ak fyd) | some check fragments absent: en1992.6.3.torsion-links.; id literal present in 🏛️en1992 |
| en1992 | ST-03 | 6.5.4 | partial | en1992-target.md:261 | Node capacity σ_Rd,max = k ν' fcd for compression nodes CCC (k1), compression-tension nodes CCT (k2) and tension nodes CTT (k3); realised as a check family on deep beams  | some check fragments absent: en1992.6.5.node-ccc.; id literal present in 🏛️en1992 |
| en1993 | V-011 | 3.2.2 | partial | en1993-target.md:43 | Ductility requirement f_u/f_y ≥ 1.10 (3.2.2); the elongation of 15 % and ε_u ≥ 15 ε_y are not carried | description limits scope |
| en1993 | V-029 | 5.6 | partial | en1993-target.md:73 | Plastic global analysis conditions (5.6) as a gate for members with plastic analysis: class 1 at the hinge, the restraint at the hinge location (restraint row with hingeT | description limits scope |
| en1993 | V-126 | 6.3.1.3 / 6.3.1.2(3) | partial | en1993-target.md:179 | Slenderness and buckling length Lcr = k L (6.3.1.3); buckling lengths are direct member inputs, the nomogram factor k from the end stiffness ratios G_A and G_B is not mod | description limits scope |
| en1993 | V-407 | 3.12 | partial | en1993-target.md:204 | Distribution of load over a group of fasteners (3.12); the eccentric bolt group needs bolt coordinates and an eccentricity that the joint row does not carry, so the bolt  | description limits scope |
| en1993 | V-501 | 5.1/5.2 | missing | en1993-target.md:225 | Stress range derivation per 5.1/5.2 from entered bands or the rainflow-counted stress history, not modelled for the 0.6 compression reduction | not modelled |
| en1993 | V-506 | Annex C / D | missing | en1993-target.md:230 | Fracture mechanics assessment of damage tolerant design with Paris crack growth da/dN = C (ΔK)^m (Annexes C and D) is not implemented | not modelled |
| en1993 | V-250 | 3.2.2 | partial | en1993-target.md:41 | Enhanced yield strength of cold-formed material f_ya = f_yb + (f_u − f_yb) k n t²/A_g ≤ (f_u + f_yb)/2 (3.2.2); partly not modelled as the member carries no f_u, bend cou | description limits scope |
| en1993 | V-259 | 8 | missing | en1993-target.md:253 | Screw and blind-rivet connections of cold-formed members per section 8 are not modelled yet | not modelled |
| en1993 | V-905 | 4-3 | partial | en1993-target.md:487 | Pipelines per EN 1993-4-3: wall thickness from the hoop stress, von Mises combined stress and buckling under external pressure p_cr, realised by the checks en1993.4-3.5.t | description limits scope |
| en1994 | C-32 | F:4.3 | partial | en1994-en1999-target.md:211 | composite column fire: the simple calculation model (the tabulated data are C-46, not modelled) | description limits scope |
| en1997 | GI-01 | 2.4 | partial | en1997-en1998-target.md:294 | Investigation extent: required depth z_a ≥ max(6 m, 2 B) below a footing or max(5 m, 3 D_b) below a pile base, point spacing, coverage against the available depth (Annex  | some check fragments absent: en1997.gi-01.; id literal present in 🌍️en1997 |
| en1997 | GI-03 | D | partial | en1997-en1998-target.md:296 | Undrained shear strength from CPT c_u = (q_t − σ_v0)/N_kt with the cone factor N_kt of the layer or the registry row (Annex D); per layer the characteristic c_u (and φ′ f | some check fragments absent: en1997.gi-03.; id literal present in 🌍️en1997 |
| en1998 | SEI-53 | 10 | partial | en1997-en1998-target.md:505 | isolated bridges only: period ratio and bearing displacement; building isolation (T_eff, ξ_eff, d_cd) is not modelled | description limits scope |
| en1998 | GE-01 | EN 1998-1 §3.1 | partial | en1997-en1998-target.md:558 | v_s,30 of the layered soil profile (thickness-weighted travel time over 30 m), the derived ground type of Table 3.1 and the site period; the small-strain modulus G_max =  | description limits scope |
| en1998 | GE-04 | EN 1998-5 §4.1.4, Annex B | missing | en1997-en1998-target.md:561 | pore pressure increase and lateral-spreading flag | not modelled |
| en1998 | GE-09 | EN 1998-5 §5.2 | partial | en1997-en1998-target.md:566 | foundation stiffness ratio k_foundation / k_soil with the rigidity limit 1.5 and the radiation damping credited from it; the Gazetas springs K_x, K_z, K_rx of a circular  | description limits scope |
| en1998 | GE-12 | EN 1998-5 §5.4 | partial | en1997-en1998-target.md:569 | pile in seismic ground: inertial head shear on a Winkler bed (fixed or free head), kinematic moment at the stiffness contrast of two layers and the bending resistance; or | description limits scope |
| en1998 | BR-10 | 6 | missing | en1997-en1998-target.md:526 | seismic isolation of bridges | not modelled |
| en1998 | BR-11 | 5.? | missing | en1997-en1998-target.md:527 | foundation and abutment soil-structure interaction | not modelled |
| en1998 | BR-12 | 2.3.6 | missing | en1997-en1998-target.md:528 | spatial variability of the seismic action for long bridges | not modelled |
| en1998 | AS-09 | 5, 6 | missing | en1997-en1998-target.md:544 | Retrofit strengthening schemes (jackets, FRP), design values of new materials and interface checks (not modelled: the spec row carries no equations) | not modelled |
| en1998 | ST-01 | EN 1998-4 §3 | partial | en1997-en1998-target.md:549 | silo: filled mass with the storage psi_E, shell axial demand N_Ed = m (g + 0.9 a_g) against N_Rd and the base shear F_h = m S_d(T) against the anchorage V_Rd (silo q); no | description limits scope |
| en1998 | ST-03 | EN 1998-4 pipelines | missing | en1997-en1998-target.md:551 | buried pipelines | not modelled |
| en1998 | TW-01 | EN 1998-6 damping | missing | en1997-en1998-target.md:573 | structural damping and the damping correction of towers | not modelled |
| en1999 | A-06 | 6.1.4.3–6.1.5.3 | partial | en1994-en1999-target.md:314 | Distortional and stiffened elements: edge-stiffened outstands are classified with their stiffener, the stiffener spring iteration and intermediate stiffeners are not mode | description limits scope |
| en1999 | A-12 | 6.2.3 | partial | en1994-en1999-target.md:300 | Tension in the net section N_u,Rd = 0.9 A_net f_u/γ_M2 with HAZ: the HAZ enters A_eff of the axial check, the net section at holes is not modelled (6.2.3) | description limits scope |
| en1999 | A-24 | - | partial | en1994-en1999-target.md:365 | ω_x of a localized transverse weld enters the 6.3.3 interaction (member transverseWeld) and the torsional buckling of free outlines (6.3.1.4); a HAZ torsional buckling ch | description limits scope |
| en1999 | A-51 | 2:4.2.5 | partial | en1994-en1999-target.md:407 | the unprotected temperature θ_a(t) (section factor, step integration of part_1_2) is reported against θ_cr; protected members (insulation) are not modelled | description limits scope |
| en1999 | A-61 | EN 1999-1-4 | missing | en1994-en1999-target.md:414 | Resistance of sheeting in bending, web crippling and shear (EN 1999-1-4, 6.1), realised as the bending check | check fragments absent from implementation |
| en1999 | A-62 | EN 1999-1-4 | missing | en1994-en1999-target.md:415 | Serviceability of purlins and sheeting: deflection with effective second moment of area (EN 1999-1-4), realised as the deflection check | check fragments absent from implementation |
| en1999 | A-70 | EN 1999-1-5 | missing | en1994-en1999-target.md:419 | Buckling of cylindrical shells with classical elastic critical stress (EN 1999-1-5, Sections 6-7), realised as the shell buckling check | check fragments absent from implementation |
| en1999 | A-72 | EN 1999-1-5 | missing | en1994-en1999-target.md:422 | Welded shells with HAZ reduction ρ_o,haz on the axial compression resistance (EN 1999-1-5), realised within the shell buckling check | check fragments absent from implementation |
| en1999 | A-81 | 2.3.5.? thermal expansion α = 23e-6; cre | missing | en1994-en1999-target.md:427 | Thermal expansion and creep of aluminium (2.3.5) are excluded by the spec and not modelled | not modelled |


Partial rows marked "id literal present" have their quoted catalogue id in non-test code. They are probably implemented and the trace fragment is stale. Verify before counting them as gaps. The en1990 E90.xx partials are an example, since the STR combination is implemented under `en1990.6.4.combinations`.

## Blocked on print or unverified (ids per code)

Counted as blocked, not missing. Each needs printed-standard or NA-DE verification before the value is trusted.

- en1990 (0): 
- en1991 (25): E11.09, E12.05, E15.06, E16.06, E17.03, E17.05, E17.06, E17.09, E2.02, E2.06, E2.07, E2.08, E2.10, E2.11, E2.13, E2.14, E2.15, E2.19, E2.22, E3.01, E3.04, E3.06, E3.07, E4.02, E4.04
- en1992 (1): FA-11
- en1993 (20): V-010, V-012, V-015, V-066, V-440, V-450, V-505, V-609, V-251, V-256, V-257, V-258, V-301, V-303, V-304, V-305, V-805, V-900, V-902, V-903
- en1994 (1): C-46
- en1995 (1): T-122
- en1996 (7): M-05, M-15, M-31, M-52, M-60, M-70, M-90
- en1997 (1): PILE-09
- en1998 (23): SEI-01, SEI-46, SEI-50, SEI-52, GE-02, GE-06, GE-07, GE-08, GE-11, BR-01, BR-02, BR-03, BR-04, BR-06, BR-07, BR-08, BR-09, AS-04, AS-06, AS-10, ST-02, TW-04, TW-05
- en1999 (2): A-19, A-28

## Findings

- 81 implemented check rows (en1997 37, en1995 24, en1996 9, en1991 6, en1990 5) match only by catalogue id; their trace fragment is absent. Plus 28 partial rows, most of which also have an id literal present. Ids are implemented under catalogue names; the trace still points at old fragment names. The trace is not a reliable gap source on its own. Fix the trace fragments rather than the code.
- 22 rows are truly missing (no fragment, no catalogue id). Most are "not modelled" in the spec itself: EN 1991 rail and silo items, EN 1998 bridge and pipe items, EN 1999 sheeting and shell buckling (A-61, A-62, A-70, A-72).
- EN 1999 sheeting and shells (A-61, A-70, A-72) are the largest ULS gap in the code set. No catalogue entry exists for them.
- EN 1993 partials (V-126, V-407, V-250, V-029) are scope limits stated in the spec text, not stale fragments.
- EN 1992 has no missing rows. Its partials (T-02, ST-03) are stale-fragment rows with id literals present.
