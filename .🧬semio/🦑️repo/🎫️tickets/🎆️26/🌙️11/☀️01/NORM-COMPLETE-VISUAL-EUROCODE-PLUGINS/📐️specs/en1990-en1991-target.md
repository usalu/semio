# 📐 EN 1990 and EN 1991 Target Specification

Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` · goal `norm` · artifacts `⚖️en1990`, `🏋️en1991` · annexes: `EN-recommended`, `NA-DE` (DIN EN 1990/NA, DIN EN 1991-x/NA).
Companion files: `en1990-en1991-examples.py` (generator of every number in section 7; plain Python, standard library only, rerunnable).

## 0. Conventions, scope and honesty rules

### 0.1 Confidence tags
Numeric data in this document were reproduced without the printed standards at hand. Every NDP value and table cell carries a tag:
- `[N]` normative formula or value that is stable across editions and cross-checked by an independent closed form.
- `[V]` value or table cell that MUST be double-keyed from the printed standard (EN and NA-DE) and cross-checked by two people or one OCR run plus one manual run before the data asset is enabled. Until verified the plugin shows a visible "unverified NDP" badge in the editor and the report, and compliance output is stamped "provisional".
- `[E]` editorial or engineering choice of this software (not in the standard).
Tables are data assets (`📚️data/<table-id>.json` in schema form), never code constants.

### 0.2 Annex mechanism (existing vocabulary)
`AnnexChoice` (from `semio_s_plugin_norm_compliance`) selects the NDP set. Both artifacts already carry `annex: AnnexChoice`. Target: `NdpSet { id, standard, clause, symbol, unit, recommended, values_by_annex[], source, confidence }`, resolved by `resolve_ndp(annex, symbol)`; an edit of an NDP is an event `NdpOverridden` (explicit, user-visible, never silent). Allowed annex values: `EN_RECOMMENDED`, `NA_DE`, plus `CUSTOM(nationalAnnexId)` for further countries (no default; the user must choose, per AGENTS.md "no default language", and analogously no default annex).

### 0.3 Units
SI base in the model: N, m, Pa, K/degC for temperature differences (kelvin deltas stored as `delta_K`), s, kg. Display units are a UI concern (kN, kN/m, kN/m^2, MPa, mm, degC). Existing snapshots store forces in N (`rd_str: 250_000.0`); the target keeps N and N/m^2 in the model and formats in kN in the editor. Every field in section 3/4 is listed with its model unit.

### 0.4 Catalogue entry format
```
ID  clause  EN name / DE name
in   inputs (model unit)
f    formula(s) exactly
ndp  symbol: recommended | NA-DE
out  outputs
ok   pass/fail criterion
edge edge cases
```
IDs: `E90.n` (EN 1990), `E11.n` ... `E17.n`, `E12.n`, `E2.n`, `E3.n`, `E4.n` (EN 1991 parts, `E11` = 1-1, `E12` = 1-2, `E13` = 1-3, `E14` = 1-4, `E15` = 1-5, `E16` = 1-6, `E17` = 1-7, `E2` = part 2, `E3` = part 3, `E4` = part 4).

### 0.5 Existing model (aligned vocabulary)
`En1990Snapshot`: `annex, project_id, structure_kind, altitude_m, consequence_class, reliability_class, design_working_life_category/_years, reference_period_years, supervision_level, inspection_level, k_fi_declared, beta_computed, permanents(PermanentAction{id,kind,gk}), variables(VariableAction{id,category,qk}), accidentals(AccidentalAction{id,ad}), seismics(SeismicAction{id,a_ek,importance_class}), members(Member{rd_str,rd_geo,rd_equ_stab,rd_equ_destab,rd_fat,span,deflection_w,deflection_limit_ratio,vibration_frequency(_min)}), bridge_sls(BridgeSls{deck_acceleration,deck_twist,bridge_deflection + limits}), effects(MemberEffect{member_id,action_id,influence})`.
`En1991Snapshot`: `snow_zone, altitude, en_sk, north_german_lowland_snow, wind_zone, en_vb, terrain_category, mixed_terrain_upwind, mixed_terrain_distance, orography_factor, coast_or_island, air_density, height,width,depth, assumed_delta_t, t_max,t_min, thermal_element_type, thermal_bridge_type, delta_t_m, storey_count, fire_mode/fire_curve/fire_*, construction_activity, structure_kind, bridge_lane/bridge_span/bridge_lane_width/assumed_bridge_*, bridge_load_group, crane_*, silo_*, floors[FloorArea], self_weight_elements, roofs[RoofArea], wind_faces[WindFace], accidental_cases[AccidentalCase{impact[],explosion[]}]`.
Gaps against the target (drives section 3/4): no geometry (only scalar height/width/depth), no wind zones A-J per face, no snow shapes (drifts), no lanes/vehicles, no crane wheel groups, no silo geometry, many `assumed_*` inputs (user-typed results instead of computed ones), actions in EN 1990 are scalar lists without combination groups, no Annex B/C/D/ A2 data, no fatigue-load/rail model. `assumed_*` fields are removed (greenfield, no compatibility): all become computed outputs of inferences.

### 0.6 Cross-artifact contract (event-sourced)
`en1991` computes characteristic actions and publishes them as events `ActionCharacteristicDerived { source: "E14.12", actionId, class, nature, kind, value, unit, field(geometry ref), psi_hint }`. `en1990` subscribes, owns the combination matrix and ψ values (Annex A1/A2 tables), and publishes `CombinationEvaluated`. `en1991` never applies partial factors; `en1990` never computes a characteristic load. Member resistances come from `en1992..en1999` through `MemberEffect` (influence) and `Member.rd_*`.

## 1. EN 1990 catalogue (Basis of structural design, incl. A1, A2, B, C, D)

### 1.1 Clauses 2-3: requirements, reliability, design situations
**E90.01 2.1 / Basic requirements / Grundlegende Anforderungen**
in: structure_kind, use (`building|bridge|footbridge|railway|crane_runway|silo|tower`). f: none (classification). out: enabled limit-state families, applicable annex (A1 buildings, A2 bridges). ok: n/a. edge: mixed structures (building with footbridge) are modelled as several `StructurePart`s each with own annex group.

**E90.02 2.2 + B Table B1 / Zuverlaessigkeitsmanagement: Schadensfolgeklassen CC1-CC3, RC1-RC3**
in: consequence_class CC in {1,2,3}, reliability_class RC (default = CC, override by EN 1990 B.3). f: K_FI(RC1)=0.9, K_FI(RC2)=1.0, K_FI(RC3)=1.1 [N]. ndp: K_FI: 0.9/1.0/1.1 | NA-DE same [V] (DE uses K_FI via the design supervision/inspection route, see E90.05). out: K_FI applied to unfavourable γF (Table B3): γF,eff = K_FI·γF for unfavourable permanent and variable actions. ok: RC3 requires DSL3-compatible supervision flag; mismatch raises a warning not an error. edge: K_FI is not applied to favourable actions or to γM.

**E90.03 2.3 Table 2.1 / Entwurfsnutzungsdauer**
in: category 1-5. f (table, NDP-free in EN, [N]): cat 1: 10 yr; 2: 10-30 yr (typ. replaceable parts, 25); 3: 15-30 yr (agricultural/similar, 25); 4: 50 yr (buildings, common structures); 5: 100 yr (monumental, bridges). out: `design_working_life_years` default, reference period for variables (T_ref = 50 yr buildings, 100 yr bridges for characteristic values of traffic/snow/wind in 1991). Editor allows an override with a justification note. ok: n/a. edge: reference period ≠ working life: variable-action characteristic values are rescaled with E13.12 / E14.13 (return period factors) and event `ActionRescaled`.

**E90.04 2.4 / Dauerhaftigkeit**
in: environment class from en1992/1993/1995 (link), design_working_life. out: durability flag consumed by material parts. ok: n/a.

**E90.05 2.5 + B Tables B4, B5 / Qualitaetsmanagement: DSL, IL**
in: design_supervision_level DSL1-3, inspection_level IL1-3. f: Table B4 DSL1 (relaxed, self-check) / DSL2 normal (checking by different persons) / DSL3 extended (third party); Table B5 IL1 self inspection, IL2 normal, IL3 extended. Combination rule (Annex B.5(3)): γM reduction/increase is expressed through K_FI; software does NOT scale γM by DSL/IL silently, it only records and checks plausibility (RC3 ⇒ DSL3 or IL3 recommended). ok: warning only.

**E90.06 3.2 / Bemessungssituationen**
in: set of situations {persistent, transient, accidental, seismic, fatigue?}. f: mapping situation → combination formula (E90.15-E90.18). out: list of active combination families. edge: transient situations (execution, from 1991-1-6) get their own `ReferencePeriod` and reduced variable values.

### 1.2 Clause 4: actions and environmental influences
**E90.07 4.1.1 / Einwirkungsklassifikation**
in per action: variation in time {permanent G, variable Q, accidental A}, origin {direct, indirect}, spatial {fixed, free}, nature {static, dynamic, quasi-static}, effect {favourable, unfavourable per effect}. out: `ActionClass` bitfield used by combination generator. ok: every action must be classified; unclassified actions block evaluation with a remedy list.

**E90.08 4.1.2 / Charakteristische Werte**
in: statistics of G (mean G_m, coefficient of variation V_G) or nominal. f: if V_G ≤ 0.05 single G_k (mean); else G_k,sup = G_m + 1.64 σ_G ... equivalent: G_k,sup=G_m(1+1.64V_G), G_k,inf=G_m(1-1.64V_G) [N] (EN 1990 4.1.2(2),(3), (4.1)); for variable: Q_k = 98% fractile of annual maximum (50-yr: 2%). Prestress P_k,sup/P_k,inf from en1992. out: G_k,sup, G_k,inf. edge: V_G>0.1 ⇒ separate upper/lower values mandatory; for self-weight of concrete with varying thickness the editor offers "thickness tolerance Δt" which feeds V_G.

**E90.09 4.1.3 / Weitere repraesentative Werte**
f: ψ0 Q_k (combination), ψ1 Q_k (frequent), ψ2 Q_k (quasi-permanent) [N]. ndp: ψ values Table A1.1 (E90.22) and A2.1 (E90.25). out: Q_rep per combination. ok: ψ0 ≥ ψ1 ≥ ψ2 ≥ 0 [N plausibility, enforce on custom edits].

**E90.10 4.2 / Materialeigenschaften** X_d = η X_k/γ_m (6.3) [N], X_k = 5% fractile (strength) or mean (stiffness). out: consumed by material parts (`Member.rd_*`). edge: η conversion factor of 1.0 default.

**E90.11 4.3 / Geometrische Groessen** a_d = a_nom ± Δa (6.4) [N]. out: imperfection tolerance values passed to material parts.

### 1.3 Clause 5: analysis and design assisted by testing
**E90.12 5.1.1-5.1.4 / Berechnungsmodelle** in: analysis type {first-order linear, second-order, plastic, non-linear}, imperfections, time effects. out: flags that govern which `Member.rd_*` apply; checks α_cr thresholds are owned by material parts. **E90.13 5.2** see Annex D (E90.30).

### 1.4 Clause 6: verification by the partial factor method
**E90.14 6.4.1 (6.1)-(6.4)** definition of limit states EQU, STR, GEO, FAT (UPL, HYD live in EN 1997: link only).

**E90.15 6.4.2 (6.5)-(6.8) / Nachweis EQU (Lagesicherheit)**
in: E_d,stb (stabilising, γG,inf), E_d,dst (destabilising, γG,sup/γQ), optional T_d (shear/anchor resistance). f: E_d,dst ≤ E_d,stb (6.7); with resistance: E_d,dst ≤ E_d,stb + T_d [N]. ndp: Table A1.2(A): γG,sup=1.10 | 1.10; γG,inf=0.90 | 0.90; γQ,1 unfavourable 1.50 | 1.50; γQ,i 1.50 | 1.50; favourable variable 0 | 0 (permanent partly favourable: when both favourable and unfavourable parts of the same action exist the same γ applies to the whole in EN, DE NA allows split (see edge)). out: utilisation η_EQU = E_d,dst / (E_d,stb+T_d). ok: η ≤ 1.0. edge: DE NA.A.1.3.1 permits 1.35/1.0 for single permanent load cases with a spatially uniform variation (set B alternative) [V]; stabilising variable actions are never counted.

**E90.16 6.4.3.1-2 (6.9b)-(6.10)-(6.10b) / Grundkombination GZT STR/GEO**
f: (6.10) Σ_j γ_G,j G_k,j + γ_P P + γ_Q,1 Q_k,1 + Σ_i>1 γ_Q,i ψ0,i Q_k,i. Alternative (6.10a): Σ γ_G,j G_k,j + γ_P P + γ_Q,1 ψ0,1 Q_k,1 + Σ γ_Q,i ψ0,i Q_k,i; (6.10b): Σ ξ_j γ_G,j G_k,j + γ_P P + γ_Q,1 Q_k,1 + Σ γ_Q,i ψ0,i Q_k,i, with E_d = max(6.10a, 6.10b) [N]. ndp: set B (STR/GEO): γG,sup=1.35 | 1.35; γG,inf=1.00 | 1.00; γQ=1.50 | 1.50; ξ=0.85 | n/a. NA-DE: (6.10) is the base equation; (6.10a)/(6.10b) are NOT offered in the NA-DE annex [V]; they are enabled for EN_RECOMMENDED and other annexes. DE simplified rule NA to 6.4.3.2 [V]: for buildings with more than one variable action Ed = 1.35 ΣG + 1.35 ΣQ is permitted as a simplified upper bound (conservative). Set C (GEO, 6.10, approach DA3 in en1997): γG=1.0, γQ=1.3 | DE: see en1997 (A1+M2+R3). out: all combinations with lead action enumeration: |permutation| = (number of mutually-exclusive-aware variable lead choices) × (γG sup/inf choices if the pattern load requires); E_d = max over combinations per effect. ok: E_d ≤ R_d (6.8) → η_STR = E_d/R_d ≤ 1. edge: variables in one `ExclusiveGroup` (e.g. traffic load groups, snow and construction) enter at most once; favourable variables dropped (γ=0); G inf/sup per spatial member (alternating span loading) is a pattern generator, not a single factor; K_FI scaling (E90.02) applies to all unfavourable γ.

**E90.17 6.4.3.3 (6.11b) / Aussergewoehnliche Kombination**
f: Σ G_k,j,sup + G_k,j,inf + P + A_d + (ψ1,1 or ψ2,1) Q_k,1 + Σ ψ2,i Q_k,i, all γ=1.0 [N]. ndp: choice ψ1,1 vs ψ2,1 for the leading variable: depends on the accidental action (impact, fire: ψ1,1 in EN 1991-1-2 4.3; DE NA fire: ψ1,1 [V]). out: E_dA per accidental case. edge: A_d may be a set (impact x, impact y); the generator creates one combination per A_d.

**E90.18 6.4.3.4 (6.12b) / Erdbeben** f: Σ G_k,j + P + A_Ed + Σ ψ2,i Q_k,i [N]; A_Ed = γ_I A_Ek (EN 1998; owned there). out: E_dEd.

**E90.19 6.5.3 (6.14b)-(6.16b) / Gebrauchstauglichkeit**
f: characteristic Σ G_k,j + P + Q_k,1 + Σ ψ0,i Q_k,i; frequent Σ G + P + ψ1,1 Q_k,1 + Σ ψ2,i Q_k,i; quasi-permanent Σ G + P + Σ ψ2,i Q_k,i [N]. ndp: γ=1.0 | 1.0. out: three effect sets. ok: E_d ≤ C_d (6.13). edge: which family applies to which criterion is NDP-free guidance: characteristic = irreversible (damage), frequent = reversible (comfort, cracked non-structural elements), quasi-permanent = long-term (creep, appearance); the SLS criterion table (E90.24) stores the pairing.

**E90.20 6.3.1-6.3.2 / Teilsicherheitsbeiwerte** γ_F = γ_f γ_Sd (6.2); E_d = γ_Sd E{γ_f F_rep; a_d}; R_d = R{η X_k/γ_m; a_d}/γ_Rd (6.6, 6.6a-c) [N]. γ_Sd default 1.0, owned by material parts.

**E90.21 6.4.5 FAT / Ermuedung** link: combination for fatigue per EN 1991-2/3 and material parts; ΔσE,2 equivalent; EN 1990 only states the verification structure E_d,fat ≤ R_d,fat; modelled: `FatigueSpectrum{ actionId, cycles N, range Δ, factor λ_i }` (Palmgren-Miner Σ n_i/N_i ≤ 1, owned by material parts, with EN 1990 A2.4.x load models for bridges).

### 1.5 Annex A1 (buildings)
**E90.22 Table A1.1 / ψ-Beiwerte Hochbau**
| Action | ψ0 | ψ1 | ψ2 |
|---|---|---|---|
| Imposed A residential | 0.7 | 0.5 | 0.3 |
| B offices | 0.7 | 0.5 | 0.3 |
| C congregation | 0.7 | 0.7 | 0.6 |
| D shopping | 0.7 | 0.7 | 0.6 |
| E storage | 1.0 | 0.9 | 0.8 |
| F traffic ≤30 kN | 0.7 | 0.7 | 0.6 |
| G traffic 30-160 kN | 0.7 | 0.5 | 0.3 |
| H roofs | 0 | 0 | 0 |
| Snow, EN: CEN states H>1000 m a.s.l. | 0.70 | 0.50 | 0.20 |
| Snow, EN: H ≤1000 m | 0.50 | 0.20 | 0 |
| Snow, NA-DE ≤ NN+1000 m | 0.5 | 0.2 | 0 |
| Snow, NA-DE > NN+1000 m | 0.7 | 0.5 | 0.2 |
| Wind | 0.6 | 0.2 | 0 |
| Temperature (not fire) | 0.6 | 0.5 | 0 |
NA-DE differences [V]: ψ values by category match EN recommended for A-H; the discriminating NDP is snow altitude; NA-DE Table NA.A.1.1 additionally lists "Nutzlast in Spitzboeden/Treppen" (follow category of the adjoining floor) and "Windlast and Schnee on gleichzeitig: zeitgleich with ψ0" (no change). The data asset keeps `valid_for_category` plus `altitude_threshold_m` as fields. Custom ψ edit emits `PsiOverridden` with mandatory justification. ok: validated monotonic ψ0≥ψ1≥ψ2.

**E90.23 Tables A1.2(A) EQU, A1.2(B) STR/GEO, A1.2(C) GEO + A1.3 / Teilsicherheitsbeiwerte Hochbau** values as in E90.15/E90.16. A1.3 (accidental/seismic) uses all γ=1.0 and ψ of E90.17/18 [N].

**E90.24 A1.4 / Gebrauchstauglichkeit Hochbau**
in: member span L (m), deflection components w_c (pre-camber, mm), w_1 (permanent), w_2 (long-term of permanent), w_3 (variable), w_tot = w_1+w_2+w_3, w_net,fin = w_tot − w_c (Fig A1.1) [N]. ndp: limits: EN gives only guidance; NA-DE leaves limits to material parts (typical L/250 w_net,fin ; L/300 w_3 for brittle; L/500 plaster) with NA-DE for concrete/steel/timber referencing en1992/1993/1995. Limit table is a data asset `SlsCriterion { id, element, family(char|freq|qp), quantity, limit_expr(L/250), source }` [V]. Vibration: floors: f1 ≥ 3 Hz for gym/dance [V] (EN 1990 A1.4.4); NA-DE: f1,min ≥ 3 Hz residential walking [V]; footbridge comfort via E90.27. Horizontal displacement: u ≤ H/500 per structure and u_i ≤ h_i/300 storey (guidance, EN A1.4.2(1)(2)) [V]. out: utilisation per criterion. ok: w ≤ limit. edge: cantilever: L = 2·projection; creep ratio φ(t) from en1992; w2 = φ w1.

### 1.6 Annex A2 (bridges)
**E90.25 Tables A2.1, A2.2, A2.3 / ψ-Beiwerte Bruecken**
Road (A2.1) EN recommended [V on NA-DE, the NA-DE bridge table mirrors EN with temperature ψ0=0.8 option, see below]:
| Action | ψ0 | ψ1 | ψ2 |
|---|---|---|---|
| gr1a TS (LM1 tandem) | 0.75 | 0.75 | 0 |
| gr1a UDL | 0.40 | 0.40 | 0 |
| gr1a pedestrian/cycle | 0.40 | 0.40 | 0 |
| gr1b single axle | 0 | 0.75 | 0 |
| gr2 horizontal forces | 0 | 0 | 0 |
| gr3 pedestrian | 0 | 0.40 | 0 |
| gr4 crowd | 0 | 0.75 | 0 |
| gr5 special vehicles | 0 | 0 | 0 |
| Wind F_Wk (persistent) | 0.6 | 0.2 | 0 |
| Wind F*_W | 1.0 | – | – |
| Thermal T_k | 0.6 (DE bridges 0.8 [V]) | 0.6 | 0.5 |
| Snow Q_Sn,k construction | 0.8 | – | 0 |
| Construction loads Q_c | 1.0 | – | 1.0 |
Footbridge (A2.2): Q_fwk crowd 0/0.4/0, Q_serv 0/0.4? [V]. Railway (A2.3): LM71/SW0 groups gr11-gr31: ψ0=0.80, ψ1=0.80, ψ2=0; SW/2: 0/1.00/0; unloaded train 1.00/–/–; HSLM ψ1=1.0 [V]. Traffic actions combine per group; groups are mutually exclusive (one ExclusiveGroup).
**E90.26 Table A2.4(A,B,C) + A2.4.4 / Teilsicherheitsbeiwerte Bruecken**
f: EQU: γG,sup 1.05, γG,inf 0.95 (A2.4(A), EN rec) but NA-DE uses 1.05/0.95 [V]; road traffic γQ=1.35 unfavourable (0 favourable); rail γQ=1.45; pedestrian 1.35; other variable 1.50; STR/GEO set B: γG 1.35/1.00. Prestress γP per en1992. out: bridge combinations with ExclusiveGroups (gr1a..gr5, wind with traffic rule: F_W** combines with the traffic group only up to F*_W; thermal only if effects unfavourable).
**E90.27 A2.4.3 / SLS bridges** criteria data (BridgeSls existing): road bridge: vertical comfort per EN 1990 A2.4.3.2: no general limit; DE: deflection from LM1 frequent ≤ L/1000? [V]; footbridge: vertical acceleration a_v ≤ 0.7 m/s² (a_h ≤ 0.2 normal use, 0.4 crowded, lock-in limit ≈ 0.1 m/s² as trigger) [N rec]; railway: deck acceleration ≤ 3.5 m/s² ballasted, 5.0 m/s² direct fixation; deck twist t ≤ 4.5 mm/3 m (v ≤ 120 km/h), 3.0 (120 < v ≤ 200), 1.5 (> 200) [V]; rail bridge deflection limit L/600 (comfort level "very good") [V]. out: utilisation = value / limit. ok ≤ 1. Existing `BridgeSls` fields map one-to-one.

### 1.7 Annexes B, C, D
**E90.28 B / Zuverlaessigkeitsdifferenzierung** Table B2 recommended minimum β:
| RC | β (1 yr) | β (50 yr) | pf(1yr) | pf(50 yr) |
|---|---|---|---|---|
| RC3 | 5.2 | 4.3 | 9.96e-8 | 8.54e-6 |
| RC2 | 4.7 | 3.8 | 1.30e-6 | 7.23e-5 |
| RC1 | 4.2 | 3.3 | 1.33e-5 | 4.83e-4 |
f: pf = Φ(−β) [N]. Relationship: β_50 from β_1 by p_50 = 1−(1−p_1)^50 → β_50 = −Φ⁻¹(p_50) (independent-year approximation; values above are the standard rounded ones, do not recompute). Verify: β_computed (existing field) must be ≥ β_target(RC, reference period). NDP: NA-DE: β_target(50 yr, RC2)=3.8 [V], same table. out: reliability status and K_FI recommendation (E90.02).
**E90.29 C / Basis fuer Teilsicherheitsbeiwerte und Zuverlaessigkeitsanalyse**
in: distribution types {normal, lognormal, Gumbel, Weibull}, mean, V, β_t. f (FORM, [N]): design point X_d with sensitivity factors α_E = −0.7, α_R = 0.8 valid for 0.16 < σ_E/σ_R < 7.6 (otherwise α = ±1.0 for the dominant and ±0.4 for the non-dominant variable) (C.5-C.7). Normal: X_d = μ_X − α_X β σ_X (C.2: resistance μ_R(1 − α_R β V_R), action μ_E(1 − α_E β V_E)); lognormal X_d = μ_X exp(−α β σ_ln)/√(1+V²) with σ_ln=√ln(1+V²). Partial factor γ_R = R_k / R_d, γ_E = E_d / E_k. Limit-state function g = R − E; iterative HL-RF for general g with convergence tolerance 1e-6, max 100 iterations; cancellable. out: β, α_i, X_d,i, γ_i, P_f. ok: β ≥ β_t. edge: non-convergence (report last iterate and flag), correlated variables (ρ, Nataf transform), truncated distributions.
**E90.30 D / Versuchsgestuetzte Bemessung**
in: n test results x_i, V_X known/unknown, η_d. f (D7): X_d = η_d m_X (1 − k_n V_X) / γ_m; characteristic X_k = m_X (1 − k_n V_X) for normal, lognormal X_k = exp(m_y − k_n s_y), with k_n from Table D1:
| n | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 20 | 30 | ∞ |
|---|---|---|---|---|---|---|---|---|---|---|---|
| k_n V_X known | 2.31 | 2.01 | 1.89 | 1.83 | 1.80 | 1.77 | 1.74 | 1.72 | 1.68 | 1.67 | 1.64 |
| k_n V_X unknown | – | – | 3.37 | 2.63 | 2.33 | 2.18 | 2.00 | 1.92 | 1.76 | 1.73 | 1.64 |
out: X_k, X_d, γ_m*. ok: n ≥ 3 if V unknown; warns when V estimated from < 6 samples. edge: k_n interpolation for n between table nodes is linear in 1/n [E]; for V_X known and n not tabulated use the fractile t-formula k_n = 1.645 √(1 + 1/n) (≈) as cross-check only.

### 1.8 NA-DE items beyond ψ/γ (summary for the NDP data asset)
`NA.2.1` consequence classes; `NA.A1.3.1` γ values (DE uses 1.35/1.0/1.5, EQU 1.1/0.9/1.5 [V]); `NA.A1.1` ψ (snow altitude split); `NA.A1.4` SLS (references to material NA); bridges `NA.A2.x` temperature ψ0 0.8 [V]; `NA.B` reliability tables unchanged [V]; Brandfall ψ1,1 [V]. A complete list is generated at implementation time from the NA text; the schema field `NdpSet.confidence` tracks verification status.

## 2. EN 1991 catalogue

Common: every part yields `CharacteristicAction` events (section 0.6). The `assumed_*` fields of the current snapshot disappear.

### 2.1 EN 1991-1-1 Densities, self-weight, imposed loads (`E11`)

**E11.01 Annex A / Wichten von Baustoffen und Lagerstoffen** data asset `Material{id,name_en,name_de,gamma_kN_m3 (min,max,recommended)}`; recommended [V]: concrete normal 24.0 (reinforced +1 ⇒ 25.0, unhardened +1), lightweight 9-20, steel 77.0-78.5 (structural 78.5 DE [V]), aluminium 27, timber C14-C50 3.5-5.0 / glulam GL24h-GL36 3.7-4.4 / plywood 4.5-7, masonry clay brick 7-22, calcium silicate 12-22, aerated concrete 4-9, mortar 19-23, screed cement 22, asphalt 24, glass 25, soil: gravel 16-20, sand 17-20, clay 18-20, water 10, snow see 1-3. DE NA references DIN 1055-1 for the full table [V].
f: g_k = Σ ρ·t [kN/m^2], line load = γ·A, volume load = γ. edge: moisture adder; reinforcement adder +1 kN/m^3 (up to 2 % steel, else computed).

**E11.02 5.2 / Eigenlasten: bewegliche Trennwaende, Fassaden**
Self-weight of movable partitions: per-length wall load g_wall (kN/m) → equivalent uniform load Δq_k: EN (6.3.1.2(8)) wall ≤1.0 kN/m: 0.5; ≤2.0: 0.8; ≤3.0: 1.2 kN/m^2 [V]; NA-DE: g ≤ 3.0 kN/m: 0.8; 3.0 < g ≤ 5.0: 1.2; 5.0 < g ≤ 6.0: 1.6 kN/m^2; heavier: single loads individually (no equivalent) [V]. Applicability: only floors with sufficient lateral load distribution (DE: slabs capable of transverse distribution); heavy walls (> 6 kN/m) are `LineLoad` actions placed by the user on 3D plan. out: Δq_k added to Q_k of the floor category. edge: Δq_k not added in categories C-E where q_k ≥ 5 (EN: ≥ 5 kN/m^2 not necessary).

**E11.03 6.3.1.2 Table 6.2 / Nutzlasten Gebaeude** category table (q_k kN/m^2, Q_k kN), EN recommended with range [V]:
A floors 2.0 / 2.0; A stairs 2.0 / 2.0; A balconies 4.0 / 2.0; B 3.0 / 4.5; C1 3.0 / 4.0; C2 4.0 / 4.0; C3 3.0-5.0 / 4.0; C4 5.0 / 4.0; C5 5.0 / 4.5; D1 4.0 / 4.0; D2 5.0 / 7.0; E1 7.5 / 7.0 (≥7.5, 3.0 per m storage height recommended: 6.0 kN/m^2 per metre in DE [V]); F ≤30 kN vehicle 2.5 / 20; G 30-160 kN 5.0 / 90; H roof 0.4 / 1.0 (access for repair only); I,K roofs by use (K: helicopter 1-2 kN/m^2). NA-DE Table 6.1DE [V]: A1 (Spitzboden) 1.0/1.0; A2 1.5/–; A3 2.0/1.0; B1 2.0/1.0; B2 3.0/1.5; B3 5.0/4.0; C1 3.0/4.0; C2 4.0/4.0; C3 5.0/4.0; C4 5.0/4.0; C5 5.0/4.0; C6 7.5/10.0; D1 5.0/4.0; D2 5.0/4.0; D3 5.0/4.0; E1 6.0/7.0; T1 2.0/2.0; T2 3.0/3.0; Z (balkon) 4.0/2.0; H roof 0.0 / 1.0. Stored as `UseCategory{code,label_en,label_de,q_k,Q_k,A_ref(0.05×0.05 m^2 for Q), annexed psi}`. NDP: choose q_k within the range (EN) – editor slider restricted to range; NA-DE fixed values. Q_k applies to a square 50 × 50 mm unless stated; Q_k and q_k are not simultaneous (EN: applied separately, verify both).

**E11.04 6.3.1.2(10), 6.3.1.2(11) / Abminderung αA, αn**
f: α_A = 5/7 ψ0 + A0/A ≤ 1.0 with A0 = 10 m^2 (EN 6.1) for categories A-D (and C-D: α_A ≥ 0.6) [N]; α_n = (2 + (n − 2) ψ0)/n for n > 2 storeys (cat A-D) (6.2) [N]. out: reduced q_k for beams/slabs (α_A) and columns/walls (α_n). NA-DE: reduction by loaded area α_A is NOT permitted [V]; reduction α_n for columns allowed with the ψ0 above [V]; flagged by a boolean NDP `allow_area_reduction`. edge: α_A not applied to Q_k, and neither for cat E.

**E11.05 6.4 Table 6.12-6.13 / Horizontallasten auf Absturzsicherungen (Gelaender, Brüstungen)**
f: q_k line load on parapet/barrier (kN/m): A 0.5 (EN 0.2-1.0), B/C1 0.5, C2-C4/D 1.0? (rec 1.0 [V]), C5 3.0, E 1.0-2.0 (rec 2.0), DE: A/B 0.5 (T1,T2,Z, and 1.0 for crowd C2/C3 [V]), height of application 1.2 m (EN 6.4(1)). Vehicle barriers (F): F_k = ½ m v²/(δ_c+δ_b) with m = vehicle mass (1500 kg), v = 4.5 m/s, deformations δ_c,δ_b (EN 6.4.3 Eq. 6.3/6.4 [V]): F = ½·1500·4.5²/(0.1+0.1)=76 kN; EN rec "F = 150 kN"? Use Annex-dependent: rec 150 kN at 375 mm above floor, DE 100 kN at 0.5 m [V].
**E11.06 6.3.4 Table 6.10-6.11 / Daecher** category H: q_k 0.4 (rec EN) | DE 0.0? NA-DE H roof: loads only point Q_k=1.0 kN on 0.05×0.05 and the snow load (no area imposed load when snow given) [V]; I, K by use.
**E11.07 6.3.3 / Parkdecks, Flurfoerderzeuge (forklifts)** Table 6.5-6.6: forklift class FL1 .. FL6: net weight 21 → 110 kN, hoist load 10 → 60 kN, wheel dimensions, axle: Q_k = 1.4 × (net+hoist) dynamic? EN Table 6.5: FL1: 21 kN net, 10 hoist, 0.85 axle width? [V]; wheel contact area: Table 6.6. Dynamic magnification φ = 1.4 (pneumatic) / 2.0 (solid) [V].
**E11.08 4.4 Figure 6.2 / Verteilung** Q_k load spread in slabs: spread 45° through the layers to mid-plane [E]; used by `en1992` punching. out: wheel/point load footprint (viz).
**E11.09 5.1(2) Annex B, 6.5 / Fahrzeuge auf Bruecken und Parkhaeusern**: car park F: Q_k 20 kN axle (cat F) / 90 kN (cat G), footprint 200×200 mm (F) / 300 × 300 (G) [V]. pressure p = Q_k/(a·b).

ok: none (actions are inputs for combinations). edge for all: unit mismatches; categories with ranges require explicit chosen value event `NdpChosen`.

### 2.2 EN 1991-1-2 Actions on structures exposed to fire (`E12`)
**E12.01 3.2.1 / Einheits-Temperaturzeitkurven**
f (t in min, θ in °C) [N]: standard ISO 834: θ_g = 20 + 345 log10(8t + 1) (3.4); external: θ_g = 660(1 − 0.687 e^−0.32t − 0.313 e^−3.8t) + 20 (3.5); hydrocarbon: θ_g = 1080(1 − 0.325 e^−0.167t − 0.675 e^−2.5t) + 20 (3.6). α_c = 25 (std, ext) | 50 (hydrocarbon) W/m^2K. out: θ_g(t) series for t ∈ [0, t_end], monotone. edge: t < 0 invalid; t=0 gives 20.
**E12.02 3.3 / Naturbrandkurven: Parametrische Temperaturzeitkurve (Annex A)**
in: floor area A_f (m^2), enclosure area incl. openings A_t (m^2), vertical opening area A_v (m^2), weighted height h_eq (m), thermal absorptivity b = √(ρ c λ) (J/m^2s^½K) with 100 ≤ b ≤ 2200, design fire load q_f,d (MJ/m^2 floor), growth class (slow t_lim=25 min, medium 20, fast 15; offices/dwelling medium).
f [N]: O = A_v √h_eq / A_t (m^½); Γ = ((O/b)/(0.04/1160))^2; t* = t·Γ (t in h); heating: θ_g = 20 + 1325 (1 − 0.324 e^−0.2t* − 0.204 e^−1.7t* − 0.472 e^−19t*); q_t,d = q_f,d A_f/A_t (50 ≤ q_t,d ≤ 1000 MJ/m^2); t_max = max(0.2·10⁻³ q_t,d / O ; t_lim) (h); fuel-controlled if t_max = t_lim: then t*_max uses Γ_lim = ((O_lim/b)/(0.04/1160))², O_lim = 0.1·10⁻³ q_t,d / t_lim, and when O > 0.04 and q_t,d < 75 and b < 1160 Γ increased by factor k = 1 + ((O−0.04)/0.04)((q_t,d−75)/75)((1160−b)/1160); cooling: θ_g = θ_max − 625 (t* − t*_max x) for t*_max ≤ 0.5; −250 (3 − t*_max)(t* − t*_max x) for 0.5 < t*_max < 2; −250 (t* − t*_max x) for t*_max ≥ 2 (x = 1 if t_max>t_lim else t_lim Γ/t*_max). Validity: A_f ≤ 500 m^2, h ≤ 4 m, no openings in roof; 0.02 ≤ O ≤ 0.20. out: θ_g(t), θ_max, t_max. edge: warnings outside validity; minimum 0.02.
**E12.03 3.3.2 + Annex E / Brandlastdichte**
f [N]: q_f,d = q_f,k · m · δ_q1 · δ_q2 · δ_n (E.1); m = 0.8; δ_n = Π δ_ni (sprinkler δn1=0.61, independent water supplies 0.87/0.7, automatic detection by heat 0.87, by smoke 0.73, automatic alarm 0.87, fire brigade on site 0.61, off-site 0.78, safe access routes 0.9/1.0/1.5, fire-fighting devices 1.0/1.5? [V]). Table E.1 q_f,k (80 % fractile): dwelling 948 (mean 780), hospital 280 (230), hotel 377 (310), library 1824 (1500), office 511 (420), classroom 347 (285), shopping centre 730 (600), theatre 365 (300), transport 122 (100) MJ/m^2 [V]. δ_q1 by area: 25 m^2: 1.10, 250: 1.50, 2500: 1.90, 5000: 2.00, 10000: 2.13 [V] (interpolate linear in log A [E]); δ_q2 by occupancy risk: art gallery, museum, swimming pool 0.78; offices, residence, hotel, paper industry 1.0; manufacturing of machinery 1.22; chemical 1.44; paint 1.66, fireworks 1.66? [V]. NA-DE: q_f,k per DIN EN 1991-1-2/NA Table NA.E.1 [V]; DE method (Aequivalente Branddauer via DIN 18230-1 / NA Annex F). out: q_f,d (MJ/m^2). Pre-existing `fire_load_density_qf` is the input q_f,k.
**E12.04 3.3.1.1 + Annex F / Aequivalente Branddauer** f [N]: t_e,d = q_f,d k_b w_f (F.1); k_b = 0.07 (0.09 for ..., depends on thermal inertia b: b>2500: 0.04; 720<b≤2500: 0.055; b≤720: 0.07 [V]); w_f = (6/H)^0.3 [0.62 + 90 (0.4 − α_v)^4/(1 + b_v α_h)] ≥ 0.5; α_v = A_v/A_f ∈ [0.025, 0.25]; α_h = A_h/A_f (roof openings); b_v = 12.5 (1 + 10 α_v − α_v^2) ≥ 10; H = compartment height (m). out: t_e,d (min) to be compared with standard-fire resistance class R30/60/90/120 of the member (ok: t_e,d ≤ R). 
**E12.05 Annex C / Lokale Braende** f [N]: flame length L_f = −1.02 D + 0.0148 Q^{2/5} (C.1) (Q in W, D in m); z_0 = −1.02 D + 0.00524 Q^{2/5} (virtual origin) (C.9); axis temperature θ(z) = 20 + 0.25 Q_c^{2/3} (z − z_0)^{−5/3} ≤ 900 °C (C.9) with Q_c = 0.8 Q; if L_f ≥ H (flame hits ceiling) Hasemi flux h-dot = 100000 if y ≤ 0.30; 136300 − 121000 y if 0.30<y<1.0; 15000 y^−3.7 if y ≥ 1.0 (W/m^2) with y = (r+H+z′)/(L_h+H+z′) [V]. out: gas temperature field / flux on beam surface. edge: D from fire area (D = √(4A/π)).
**E12.06 3.4 + Annex D / Waermestrom und Waermeuebergang** f [N]: h_net,d = h_net,c + h_net,r; h_net,c = α_c (θ_g − θ_m); h_net,r = Φ ε_m ε_f σ [(θ_r + 273)^4 − (θ_m + 273)^4], σ = 5.67×10⁻⁸ W/m²K⁴, ε_f = 1.0, ε_m = 0.8 (Annex C of material parts; EN 1991-1-2 default 0.8), Φ configuration factor 1.0 default, θ_r ≈ θ_g. α_c unexposed side 4 W/m²K (incl. radiation). out: net flux (W/m^2) per member face; heating via lumped-capacity for steel (ec3: Δθ_a,t = k_sh (A_m/V)/(c_a ρ_a) h_net Δt, Δt ≤ 5 s, k_sh for I-sections, c_a(θ) EN 1993-1-2) – owned by en1993.
**E12.07 4.3.1 + 4.2 (E_fi,d) / Einwirkungen im Brandfall**
f [N]: E_fi,d,t = G_k + (ψ1,1 or ψ2,1) Q_k,1 + Σ ψ2,i Q_k,i + A_d(t) (4.2 → EN 1990 6.11b); η_fi = (G_k + ψ_fi Q_k,1)/(γ_G G_k + γ_Q,1 Q_k,1) (EN 1992-1-2 Eq 2.5a; ψ_fi = ψ1,1 EN, DE: ψ1,1 [V]) with simplification E_fi,d = η_fi E_d. out: E_fi,d (with η_fi); heat actions. edge: wind lead ψ_fi = ψ1,1 (wind), snow (ψ1,1 depends on altitude).
**E12.08 Annex B / Thermal actions for external members** (Lf external flames, Annex B: radiation from openings I_f = ε_f σ T_f^4, ε_f=1; T_f flame temperature; flame geometry). Specification: flame width/height from Fig B.2-B.4 as function of window geometry and fire-load/ventilation controlled; radiation flux φ_f ε_f σ T^4. [V] data.
**E12.09 Annex G / Konfigurationsfaktor** Φ = Σ... for rectangular emitter to receiver: Φ(a,b,c) = (1/2π)[a/√(1+a²) atan(b/√(1+a²)) + b/√(1+b²) atan(a/√(1+b²))] with a = h/c, b = w/c [N standard result]. out: Φ per face.
**E12.10 3.3.3 + zone models / Advanced fire models** two-zone and CFD are out of scope: the plugin accepts an imported temperature–time series (`FireCurve::Imported`) and treats it like E12.01.
NDP summary: α_c; ε_m; Φ; t_lim; k_b; ψ_fi; q_f,k; δ tables; NA-DE replacements for Annex E, F and the Table-based DE fire-load approach [V].
ok: output curves are inputs to en1992/1993/1994/1995 fire parts; EN 1991-1-2 itself has no pass/fail except `t_e,d ≤ t_required`.

### 2.3 EN 1991-1-3 Snow loads (`E13`)
**E13.01 5.2 (5.1) Persistent/transient design: s = μ_i C_e C_t s_k** [N]. in: μ_i shape coefficient, C_e exposure (windswept 0.8, normal 1.0, sheltered 1.2), C_t thermal (1.0; < 1.0 for U > 1 W/m^2K heated glazed [V]), s_k. ndp: C_e,C_t | NA-DE: C_e=1.0 (normal topography), no reduction for windswept [V], C_t=1.0 [V].
**E13.02 4.1 + NA-DE / Schneelast auf dem Boden s_k**
NA-DE (DIN EN 1991-1-3/NA, snow load zones with altitude A in m a.s.l. [V formulas strongly recalled]):
- Zone 1: s_k = max(0.65 ; 0.19 + 0.91 ((A + 140)/760)²)
- Zone 1a: s_k = 1.25 × zone 1 value
- Zone 2: s_k = max(0.85 ; 0.25 + 1.91 ((A + 140)/760)²)
- Zone 2a: s_k = 1.25 × zone 2 value
- Zone 3: s_k = max(1.10 ; 0.31 + 2.91 ((A + 140)/760)²)  (kN/m^2)
Zone assignment from municipality (data asset: `snow_zone_by_postal_code/AGS`, [V]); the editor offers a map picker plus PLZ lookup (offline table). Norddeutsches Tiefland: accidental design value s_Ad = 2.3 s_k (zones listed in NA, 'north_german_lowland_snow: bool' existing). EN recommended: s_k by CEN map zones (Annex C): Alpine Region s_k = (0.642 Z + 0.009)(1 + (A/728)²) ≥ 0.? [V], Central East s_k = (0.264 Z − 0.002)(1 + (A/256)²) [V], Mediterranean s_k = (0.498 Z − 0.209)(1 + (A/452)²) [V], with Z zone number; plugin carries these as 'EN-Annex-C' mode.
**E13.03 5.3.2 Table 5.2 / Formbeiwerte Pultdach und Satteldach**
f [N]: μ1(α) = 0.8 for 0° ≤ α ≤ 30°; 0.8 (60 − α)/30 for 30° < α < 60°; 0 for α ≥ 60°; μ2(α) = 0.8 + 0.8 α/30 (used for multi-span valleys; μ2 = 1.6 for α ≥ 30°). Duopitch load arrangements: (i) undrifted: μ1(α) both slopes; (ii) drifted: μ1(α1) on one side & 0.5 μ1(α2) other; (iii) both sides 0.5μ1... per Fig 5.3: case (i) μ1(α1), μ1(α2); (ii) 0.5 μ1(α1), μ1(α2); (iii) μ1(α1), 0.5 μ1(α2). Monopitch: μ1(α). NA-DE: same load cases, but 'ohne Schneefang; sliding' note: μ_1 reduces to 0 for α ≥ 60 even with snow guards; with snow guards or parapets μ1 = 0.8 [V].
**E13.04 5.3.4 Table 5.2, Fig 5.5 / Mehrfeldrige Daecher** f: μ1(α_mean?) outer; μ2 = 0.8 + 0.8 α/30 where α = (α1+α2)/2, ≥ 1.6 at α ≥30°; μ2 limited by γ h/s_k and 0.8 ≤ μ2 ≤ 2.0 [V]. Load arrangement: undrifted μ1 both; drifted μ2 at valley.
**E13.05 5.3.5 / Zylindrische Daecher** μ3 = 0.2 + 10 h/b ≤ 2.0 (Fig 5.6), load case undrifted 0.8 uniform (μ1 = 0.8 across), drifted 0.5 μ3 and μ3 asym distribution with angle limit β=60° (μ1 = 0.8 for slope ≤ 60°) [V].
**E13.06 5.3.6 / Angrenzende und hoehere Bauwerke: Schneeverwehung am Hindernis (Aufbau, Attika)**
f [N]: μ1 = 0.8 (roof outside), μ2 = μs + μw, with sliding μs = from upper roof slide: for α>15° 50 % of the max load of the adjacent upper roof (per EN: μs = 0.5 × upper-roof load / s_k, rec); wind-induced μw = (b1 + b2)/(2h) ≤ γh/s_k, 0.8 ≤ μw ≤ 4 (rec) | NA-DE: μw limit 0.8 ≤ μ2 ≤ 2.0 [V]? (DE note: γ=2 kN/m^3 and cap μ2 ≤ 2.0 for h/b limits) drift length l_s = 2h with 5 m ≤ l_s ≤ 15 m (EN 5.3.6(1)), γ = 2 kN/m^3. Reduced μw if b1 < ... plus 'weight of snow trapped' = q = γ h... Use: if parapet h/sk * γ > cap: capped.
**E13.07 5.3.7 / Schneeueberhang an Traufe** f [N]: s_e = k s²/γ (kN/m) (5.4), k = 3/d, k ≤ d γ, γ = 3 kN/m^3, d = snow depth = s/γ, s = s_k (1.0?) per roof case, d in m. out: line load on eaves.
**E13.08 5.3.8 / Schneefanggitter (Fs)** f [N]: F_s = s b sin α (5.5) with b = distance (m) from the guard to the next guard or ridge (sloped length), s per sliding snow μ1.
**E13.09 5.4 Annex B / Ausserordentliche Schneeverwehungen** B1: μ2 exceptional drift; B2 in snow drift: μ2 = ... Annex B: exceptional (Norwegian) drift with s_Ad = C_esl s_k ... DE: s_Ad = 2.3 s_k only for North German lowland.
**E13.10 Annex D / Anpassung an Wiederkehrperioden** f [N]: s_n/s_k = [1 − V {0.45 + 0.78 ln(−ln(1 − P))}] / (1 + 2.59 V) (D.1) with P = annual exceedance probability = 1/n (50 yr: P=0.02 ⇒ =1.0). V coefficient of variation of annual max. out: s_n.
**E13.11 Annex E / Wichte des Schnees** γ: fresh 1.0 kN/m^3, settled several hours/days 2.0, settled weeks/months 2.5-3.5, wet 4.0 [V].
**E13.12 4.2 / Ausserordentliche Situationen** exceptional snow: s_Ad = C_esl s_k with C_esl in the NA-DE '2.3' (NGL) else none.
**E13.13 6 / Local effects** Annex: snow on canopies, 'Schneelasten an Dachvorspruengen' (6.2 line load), sliding etc.; dealt in E13.07/08.
out for all: roof zone polygon list with μ per zone (viz: coloured shapes), s in kN/m^2 on the *horizontal projection*. ok: n/a. edge: α measured from horizontal; flat roofs with α<5° still get μ=0.8; roofs > 60° zero but with snow guards F_s.

### 2.4 EN 1991-1-4 Wind actions (`E14`)
**E14.01 4.2 (4.1)-(4.3) / Basiswindgeschwindigkeit** v_b = c_dir c_season v_b,0 [N]; q_b = ½ ρ v_b² with ρ = 1.25 kg/m^3 [N]; DE: c_dir = c_season = 1.0 [V]. NA-DE wind zones (v_b,0 [m/s] / q_b,0 [kN/m^2] at ρ = 1.25): WZ1 22.5 / 0.32; WZ2 25.0 / 0.39; WZ3 27.5 / 0.47; WZ4 30.0 / 0.56 [N for q at ρ=1.25: 0.3164, 0.3906, 0.4727, 0.5625; the NA text rounds to 0.32/0.39/0.47/0.56]. Zone from municipality (data asset, PLZ/AGS) [V]; Nordsee islands/offshore: WZ4-offshore by NA [V]. Altitude: NA-DE gives v_b,0 without altitude factor up to 800 m; above, NA table [V].
**E14.02 4.3.2 (4.3)-(4.5) / Rauigkeit und Windprofil**
f [N]: c_r(z) = k_r ln(z/z_0) for z_min ≤ z ≤ z_max (=200 m); c_r(z) = c_r(z_min) for z < z_min; k_r = 0.19 (z_0/z_0,II)^0.07, z_0,II = 0.05 m; v_m(z) = c_r(z) c_o(z) v_b. Terrain categories (z_0 / z_min): 0 sea 0.003/1; I 0.01/1; II 0.05/2; III 0.30/5; IV 1.0/10 [N]. NA-DE additionally uses *Mischprofile* (profile combining terrain categories II–III for inland, Table NA.B.2) and simplified profiles (Tabelle NA.B.3) [V]: 
- Binnenland (WZ1-4): q_p(z) = 1.5 q_b,0 for z ≤ 7 m; 1.7 q_b,0 (z/10)^0.37 for 7 < z ≤ 50 m; 2.1 q_b,0 (z/10)^0.24 for 50 < z ≤ 300 m
- Kueste und Inseln der Ostsee: q_p(z) = 1.8 q_b,0 (z ≤ 4 m); 2.3 q_b,0 (z/10)^0.27 (4–50 m); 2.6? q_b,0 (z/10)^0.19 (50–300 m) [V all three coefficients]
- Nordseeinseln / offshore: terrain 0/I with full EN profile [V]
- Mischprofil Kuestennaehe (WZ3,4 < 5 km from coast / sea; between coastline and 'Binnenland' by distance): linear interpolation of q_p between coast and inland profile over 'mixed_terrain_distance' [E]; existing `mixed_terrain_upwind`, `mixed_terrain_distance` map to the EN 'Annex A.2 transition between terrain roughness' (step change from smooth to rough: height of internal boundary layer h_dis = 0.28 x^0.8 z_0^0.2 [V]).
- Simplified (h ≤ 10 / 10 < h ≤ 18 / 18 < h ≤ 25): Binnenland q_p: WZ1 0.50/0.65/0.75; WZ2 0.65/0.80/0.90; WZ3 0.80/0.95/1.10; WZ4 0.95/1.15/1.30 kN/m^2 [V]. Coast/Ostsee: WZ2 0.85/1.00/1.10; WZ3 1.05/1.20/1.30; WZ4 1.25/1.40/1.50 [V]. Nordsee Inseln: WZ4 only 1.40/1.55/1.65? [V]. The plugin shows both full and simplified; if |simplified − profile| > 5 % the editor marks which is governing; simplified only for buildings h ≤ 25 m, structures not dynamically sensitive (E14.12 criterion).
**E14.03 4.3.3 + Annex A.3 / Orographie** c_o(z) = 1 for Φ < 0.05; 1 + 2 s Φ for 0.05 ≤ Φ ≤ 0.3 (A.1: with s from Fig A.2-A.3: function of X/Lu, z/Le, Le=Lu for Φ>0.3 ...); Φ = H/L_u upwind slope; for Φ>0.3: 1 + 0.6 s. NA-DE: orography effect only for exposed sites with Φ>0.05 and h-limits [V] (c_o = 1 in the usual case; only explicit via `orography_factor` in existing snapshot; target: compute from hill geometry).
**E14.04 4.4 (4.7)-(4.8) / Turbulenzintensitaet** I_v(z) = k_I / (c_o ln(z/z_0)) for z_min ≤ z ≤ z_max; = I_v(z_min) below; k_I = 1.0 [N]; DE mixed profile gives I_v implicit: I_v = σ_v/v_m ≈ 0.4 for z ≤ 7? [V].
**E14.05 4.5 (4.8) / Spitzengeschwindigkeitsdruck** q_p(z) = [1 + 7 I_v(z)] ½ ρ v_m²(z) = c_e(z) q_b [N]; DE: from profiles above. out: q_p(z) curve (viz).
**E14.06 4.2(2) NOTE, 1991-1-6 / Wiederkehrperioden-Faktor** c_prob = ((1 − K ln(−ln(1−p)))/(1 − K ln(−ln 0.98)))^n with K = 0.2, n = 0.5 [N], p annual exceedance probability (1/Tr). out: scaled v_b. Used by 1-6.
**E14.07 7.2.2 Table 7.1 / Aussendruck vertikale Waende** (Fig 7.5)
in: h, b (crosswind), d (along wind), e = min(b, 2h). zones: e<d: A = e/5, B = 4e/5, C = d − e; e ≥ d: A = e/5, B = d − e/5, no C; e ≥ 5d: only A (width d). Coefficients c_pe,10 / c_pe,1 [N]: 
| h/d | A | B | C | D | E |
|---|---|---|---|---|---|
| 5 | −1.2 / −1.4 | −0.8 / −1.1 | −0.5 | +0.8 / +1.0 | −0.7 |
| 1 | −1.2 / −1.4 | −0.8 / −1.1 | −0.5 | +0.8 / +1.0 | −0.5 |
| ≤ 0.25 | −1.2 / −1.4 | −0.8 / −1.1 | −0.5 | +0.7 / +1.0 | −0.3 |
linear interpolation in h/d. Reference height z_e: for h ≤ b: one strip z_e = h; b < h ≤ 2b: lower strip z_e = b (height b) and upper strip z_e = h; h > 2b: strips of height b from the bottom up to h − b, plus upper strip. D zone reference z_e = strip's z. NA-DE: tables identical, plus 'DE: Windlast on Windseitige Wand D': NA gives q_p simplified [V].
**E14.08 7.2.3 Tables 7.2-7.5, Fig 7.6-7.9 / Flachdach, Pultdach, Satteldach, Walmdach**
Flat roof (Table 7.2) c_pe,10 / c_pe,1: F, G, H, I
| edge | F | G | H | I |
|---|---|---|---|---|
| sharp | −1.8 / −2.5 | −1.2 / −2.0 | −0.7 / −1.2 | +0.2 / −0.2 (±0.2) |
| parapet h_p/h=0.025 | −1.6 / −2.2 | −1.1 / −1.8 | −0.7 / −1.2 | ±0.2 |
| 0.05 | −1.4 / −2.0 | −0.9 / −1.6 | −0.7 / −1.2 | ±0.2 |
| 0.10 | −1.2 / −1.8 | −0.8 / −1.4 | −0.7 / −1.2 | ±0.2 |
| curved r/h=0.05 | −1.0 / −1.5 | −1.2 / −1.8 | −0.4 | ±0.2 |
| 0.10 | −0.7 / −1.2 | −0.8 / −1.4 | −0.3 | ±0.2 |
| 0.20 | −0.5 / −0.8 | −0.5 / −0.8 | −0.3 | ±0.2 |
| mansard 30° | −1.0 / −1.5 | −1.0 / −1.5 | −0.3 | ±0.2 |
| 45° | −1.2 / −1.8 | −1.3 / −1.9 | −0.4 | ±0.2 |
| 60° | −1.3 / −1.9 | −1.3 / −1.9 | −0.5 | ±0.2 |
Zones (e = min(b,2h)): F: e/4 × e/10 corner patches; G: e/10 strip between F; H: e/2 − e/10 wide behind; I: rest. Flat-roof interpolation in h_p/h and r/h linear. Duopitch (Table 7.4a/7.4b, zones F G H I J ; wind θ = 0° and 90° with F, G, H, I, J, K, L, M, N, O) c_pe values per pitch α ∈ {−45, −30, −15, −5, 5, 15, 30, 45, 60, 75} [V: tabulate 7.3a, 7.3b, 7.4a, 7.4b, 7.5 as double-keyed JSON]. Representative (θ=0°, α=15°): F −0.9/+0.2 (pressure/suction pair: both load cases must be evaluated), G −0.8/+0.2, H −0.3/+0.2, I −0.4/–, J −1.0/−1.5 (c_pe,10) [V]. Zones F: e/4 × e/10, G: e/10 strip, H: e/2 − e/10, I: e/2 to ridge, J: e/10 around ridge (θ=0°); for θ=90°: F, G, H, I per Fig 7.8. Size rule: c_pe = c_pe,1 for A ≤ 1 m^2; c_pe = c_pe,1 − (c_pe,1 − c_pe,10) log10 A for 1 m^2 < A < 10 m^2; c_pe,10 for A ≥ 10 m^2 [N].
**E14.09 7.2.9 / Innendruck** w_i = q_p(z_i) c_pi. c_pi from Fig 7.13 versus μ = Σ(open area where c_pe ≤ 0)/Σ(all open area) and h/d: for μ=0 c_pi = +0.35? / for μ=1 −0.5? [V]. Simplification (7.2.9(6)): use the worse of +0.2 and −0.3 [N]. Dominant face: c_pi = 0.75 c_pe if the dominant-face opening area is 2× the rest; 0.90 c_pe if ≥ 3×, linear interpolation between 2× and 3× (7.2.9 (5), Expressions 7.1, 7.2) [N]. out: net pressure on face = w_e − w_i with sign per face orientation.
**E14.10 7.4-7.5 / Freistehende Dachfelder, Wandscheiben, Schilder** canopy monopitch (Table 7.6) overall c_f and local c_p,net with blockage φ = 0 (empty) and φ = 1; duopitch canopies Table 7.7; free-standing walls Table 7.9 (zones A, B, C, D by l/h), c_p,net values e.g. solid wall φ=1 : A 2.3, B 1.4, C 1.2, D 1.2 (l/h=1..) [V]; signboards (7.4.3) c_f = 1.8; z_e = z_g + h/2.
**E14.11 7.8-7.10 / Kraftbeiwerte Kreisprofile, rechteckige Querschnitte, Mehrfeldrig** 
Force F_w = c_s c_d c_f q_p(z_e) A_ref (5.3) [N]. Rectangular cross section: c_f = c_f,0 ψ_r ψ_λ (7.9) with c_f,0 from Fig 7.23 (function of d/b, digitised curve data asset [V]); ψ_r corner rounding (1 − 1.9 r/b ≥ 0.5 [V]); ψ_λ end-effect factor from Fig 7.36 vs φ=1 (λ): λ = effective slenderness (Table 7.16: l ≥ 50 m: λ = min(1.4 l/b, 70); l ≤ 15 m: λ = min(2 l/b, 70) [V]). Circular cylinders Fig 7.28: c_f,0 vs R_e = b v(z_e)/ν, ν = 15 × 10⁻⁶ m²/s, v(z_e) = √(2 q_p/ρ) with equivalent roughness k/b (Table 7.13: glass 0.0015 mm, galvanised steel 0.2, paint 0.1, rough concrete 1.0–3.0, sharp rust 2 mm [V]); formula c_f = c_f,0 ψ_λ. Lattice towers Table 7.? skip or [V]. 
**E14.12 Annex B + Annex C / Konstruktionsfaktor c_s c_d (und DE-Vereinfachungen)** 
Detailed procedure [N]: c_s c_d = (1 + 2 k_p I_v(z_s) √(B² + R²))/(1 + 7 I_v(z_s)) (6.1); z_s = 0.6 h ≥ z_min; B² = 1/(1 + 0.9 ((b + h)/L(z_s))^0.63) (B.3); L(z) = L_t (z/z_t)^α, L_t = 300 m, z_t = 200 m, α = 0.67 + 0.05 ln z_0 (B.1); R² = (π²/2δ) S_L(z_s, n_1,x) R_h(η_h) R_b(η_b) (B.7); S_L = 6.8 f_L/(1 + 10.2 f_L)^{5/3}, f_L = n_1,x L(z_s)/v_m(z_s); R_h(η) = 1/η − (1/(2η²))(1 − e^{−2η}) (η>0; =1 for η=0); η_h = 4.6 (h/L) f_L S_L... (η_h = 4.6 h/L(z_s) · f_L; η_b = 4.6 b/L(z_s) · f_L) (B.8); ν = n_1,x √(R²/(B² + R²)) ≥ 0.08 Hz; k_p = max(√(2 ln(ν T)) + 0.6/√(2 ln(ν T)), 3), T = 600 s (B.4); δ = δ_s + δ_a + δ_d (logarithmic decrements: steel buildings 0.05 welded? Table F.2 [V]: reinforced concrete 0.10, steel welded 0.05, bolted 0.05? composite 0.08, timber 0.1?; aerodynamic δ_a = c_f ρ b v_m(z_s)/(2 n_1 m_e)). n_1,x ≈ 46/h Hz for buildings (F.2 simple; 1.0 ≤ ... h in m) or from the FEM modal analysis. Simplified (no dynamic check): c_s c_d = 1 for buildings with height < 15 m; facade/roof elements; framed buildings with structural walls and h < 100 m and h < 4 b (EN 6.2(1)) ; chimneys with circular cross-section h < 60 m and h < 6.5 d (Fig 6.1 diagrams: c_s c_d values 0.85–1.0 for steel and concrete buildings [V])). NA-DE: DIN EN 1991-1-4/NA 'Anhang C' replaced by NA.C? : *Vereinfachtes Verfahren fuer Hochbauten*, valid for h ≤ 200 m, tall structure heuristics; criterion for 'Schwingungsanfaelligkeit' (n_1 and δ-dependent): if v_m(z_s) / (n_1 b) ... a structure is sensitive if the first natural frequency n_1 < 5 Hz (with limit by h/d) [V]; for non-sensitive: gust factor method G (the DE gust factor G = 1 + 2 g I_v) alternative [V].
Outputs: B², R², ν, k_p, c_s c_d, equivalent static load profile; peak acceleration σ_a,x(z) = c_f ρ I_v(z_s) v_m²(z_s) R K_x Φ_1,x(z)/m_1,x (B.9) with K_x = 1.5? (for linear mode shape ξ=1: K_x = (2ξ+1)(ξ+1)... K = (ξ+1)·.../… ) a_max = k_p σ_a,x. ok: a_max ≤ a_lim from comfort NDP (EN 1991-1-4 Annex?; DE: ISO 10137 [V]).
**E14.13 8 / Windwirkungen auf Bruecken** Bridge force F_W = ½ ρ v_b² C A_ref,x (8.1) with C = c_e c_f,x; c_f,x = c_f,x0 for the static case; 'simplified method' d_tot = deck depth + parapets (Table 8.1: open 0.3 m? solid 0.6?) A_ref,x = d_tot L; c_f,x0 as in Fig 8.3 (digitised curve; plateau 1.3 for b/d_tot ≥ 4, rising towards 2.4 for b/d_tot ≈ 0.5 [V]) [V]. NA-DE: simplified F_W = c_f q_p A_ref with z_e = deck height above ground, tabulated for WZ (DIN EN 1991-1-4/NA) and typical values 'F_W ≈ 1.8 – 3.5 kN/m'. Vertical force F_Wz = ½ ρ v_b² C_z A_ref,z with C_z = ±0.9 (rec simplified) [V], Longitudinal force 25 %/50 % of F_Wx. out: kN/m on deck, bearing reactions.
**E14.14 E / Wirbelerregung, Galloping, Interferenz, Ovalling (Annex E)**
f [N]: critical wind speed v_crit,i = b n_i,y / St (E.2); St = 0.18 circular (Fig E.2: 0.18; rectangle d/b dependent 0.11–0.14); Scruton number Sc = 2 δ_s m_i,e/(ρ b²) (E.4); Re at vcrit = b v_crit/ν; vortex-shedding is relevant if v_crit,i ≤ 1.25 v_m(z_s) (E.1.3.1); amplitude y_F,max/b = (1/St²)(1/Sc) K K_w c_lat (approach 1 E.1.5.2.1) with c_lat from Table E.2 (Re ≤ 1e5: 0.2? 0.7 ... c_lat 0.2 for Re 1e5..5e5; see [V]), K = mode-shape/correlation factor (Table E.5: cantilever 0.13, K_w effective correlation length factor (Table E.6/E.7)). Galloping v_CG = 2 Sc b n_1,y /a_G (E.18) etc. Ovalling (E.3) for shells: v_crit,ov = ... [V]. ok: vortex excited amplitudes y_max ≤ b/30? NA [V]; Criteria: v_crit/v_m ≤ 1.25 triggers detailed analysis.
**E14.15 Annex F / Dynamische Kenngroessen** n_1 = 46/h (F.2) (h in m); cantilever: n_1 = (ε_1 b/h_eff²) √(W_s/W_t) (F.3); modal mass m_e = ∫ m Φ² dz/∫Φ² dz; Φ_1(z) = (z/h)^ζ, ζ = 0.6 (framed w/o core), 1.0 (central core/concrete), 1.5 (slender cantilever), 2.0 (lattice towers) [N]; log decrements Table F.2: reinforced concrete buildings 0.10, steel buildings 0.05, mixed 0.08, steel chimneys welded unlined 0.012 [V].
**E14.16 7.6-7.13 / Weitere Kraftbeiwerte**: frictional drag F_fr = c_fr q_p A_fr, c_fr = 0.01 (smooth steel, smooth concrete), 0.02 (rough concrete), 0.04 (corrugated, ribs) (7.5) applies if (d/b ≤ 1 and d/h ≤ 4 not exceeded: A_fr = those areas parallel to wind beyond 4×min(b,2h) of upwind edge) [N]; spheres (Fig 7.30); flags (7.11 c_f,flag = ?); lattice structures Table 7.?; vertical cylinders on roof, hip/domed roofs Table 7.? [V].
NDP summary: v_b,0, c_dir, c_season, ρ, k_I, c_o, terrain table, Annex A choices, c_s c_d method, k_p limits, δ, St, K, c_lat, F_W coefficients, comfort limits, flat-roof parapet adjustments.
ok: no pass/fail inside 1991; derived actions feed 1990 combos; criterion of dynamic sensitivity (n_1, a_lim) is implemented as `ok`.

### 2.5 EN 1991-1-5 Thermal actions (`E15`)
**E15.01 5-6 / Temperaturaenderungen in Bruecken: Gleichmaessiger Temperaturanteil**
f [N]: T_e,min = T_min + ΔT_min; T_e,max = T_max + ΔT_max with deck types: Type 1 steel deck: ΔT_min = −3, ΔT_max = +16; Type 2 composite: +4, +4; Type 3 concrete: +8, +2 (Fig 6.1 approximations) [V]. Contraction range ΔT_N,con = T_0 − T_e,min, expansion ΔT_N,exp = T_e,max − T_0; T_0 = 10 °C default (initial temperature at restraint) [N]; expansion length ΔL = α_T ΔT L, α_T steel 12×10⁻⁶ /K, concrete 10×10⁻⁶ /K, aluminium 24×10⁻⁶ [N]. NDP: T_min, T_max (isotherm maps): NA-DE (DIN EN 1991-1-5/NA, Tabelle): T_min = −24 °C, T_max = +37 °C for lower elevation, with correction −0.5 K per 100 m above sea level (T_min) and −0.5 K / 100 m for T_max [V]; regions of DE with locally lower values listed by NA map [V]. Joint design range: ΔT_N,exp + ΔT_N,con (+20 K bearing margin for joints: ΔT_N +20 K) [N 6.1.3.3(3) rec].
**E15.02 6.1.4 Table 6.1 / Linearer Temperaturunterschied (vertikal)** ΔT_M,heat/ΔT_M,cool = k_sur × [Type 1: 18/13; Type 2: 15/18; Type 3a slab: 15/8; 3b box: 10/5; 3c beam: 15/8 K] with k_sur(Table 6.2 steel Type 1: unsurfaced 0.7/0.9; waterproofed 1.6/1.1; 50 mm surfacing 1.0/1.0; 100 mm 0.7/1.2; 150 mm 0.7/1.2; ballast 0.6/1.4? [V]). Nonlinear alternative: Figure 6.2 profile with ΔT_1..ΔT_4 and h_1,h_2,h_3. Horizontal: ΔT_M,y = ±5 K (rec) [V]. out: curvature κ = α ΔT_M / h; restrained moment M = E I α ΔT_M / h; end rotation θ = α ΔT_M L / (2h) (simply supported). Combination: ΔT_M,heat + ω_N ΔT_N,exp or ΔT_N,exp + ω_M ΔT_M,heat (ω_N = 0.35, ω_M = 0.75 rec) [N]. NA-DE: NA gives modified ΔT_M for composite/steel [V].
**E15.03 6.2 / Differenzen zwischen Bauteilen** ΔT between main structure and tie/cable: 15 K (rec), between cables and deck: 10–20 K; fixed [V].
**E15.04 5 / Temperaturaenderungen in Gebaeuden** T_out summer: T_out = T_max + T_3 (solar radiation absorption coefficient, orientation; Table 5.? T_1..T_4 for surface absorptance 0.5 (light) → T_out increments) and winter T_out = T_min; T_in 20 °C winter 25 °C summer (Table 5.1) [V]; thermal movement ΔT = T_out − T_0; restrained stress σ = E α ΔT (full restraint) [N].
**E15.05 Annex C / Thermal expansion coefficient table** (α_T: concrete 10, steel 12, stainless 16, aluminium 24, masonry clay 5–8, glass 9, timber 5 (parallel) [V]); **E15.06 Annex A / T_min,T_max isotherm interpolation** with altitude correction.
**E15.07 A.1 / Industrial: chimneys, silos** thermal gradient through shell (Annex D? 'cooling' ), stage [V].
ok: none (actions); outputs: ΔT, ΔL, M, N_restraint; edge: negative sign conventions: heat expands, cool contracts; Type selection by deck cross-section.

### 2.6 EN 1991-1-6 Actions during execution (`E16`)
**E16.01 3 / Bemessungssituationen und Wiederkehrperiode** table 3.1: duration ≤ 3 days → recommended return period for climatic actions 2 yr; ≤ 3 months → 5 yr; ≤ 1 yr → 10 yr; > 1 yr → 50 yr [N]. f: c_prob (E14.06) for wind (K=0.2, n=0.5), E13.10 for snow (V=0.4 rec, Annex D) [N]; wind nearby: seasonal factor c_season Table 3.2 [V]. DE NA: Table NA.? with return periods modified [V].
**E16.02 4.11.1 Table 4.1 / Bauzustaende Nutzlasten (Qca-Qcf)**
Q_ca: personnel and hand tools in working area: 1.0 kN/m^2 over A ≥ 3×3 m (rec; used with 0.75? [V]); Q_cb: storage of movable items (building materials, prefab): in Table 4.2 values by use, min 0.2 kN/m^2 over spans; Q_cc: non-permanent equipment (formwork travellers, cranes) as per manufacturer; Q_cd: movable heavy machinery; Q_ce: accumulation of waste; Q_cf: loads from parts of the structure in temporary states (self-weight of precast elements, φ_dyn 1.1–1.5 for transport) – variable, treat as G or Q depending [V]. Table 4.? : fresh concrete: working personnel: Q_ca 0.75 kN/m^2 in 3×3 m (over concrete placement area); fresh concrete as G (γ × h, 25 kN/m^3 reinforced) – pressure on formwork: p = γ h hydrostatic (for h ≤ ... ) with consistency classes from DIN 18218 (NA) [V].
**E16.03 4.13 Table 4.? / Wind und Schnee im Bauzustand** use E14/E13 with transient factors; E16.04 accidental: falling load, impact of crane etc. [V]; E16.05 4.9 / geometric imperfections during execution (tolerance Δ) and E16.06 Annex A.2 / Combination of construction loads 'Q_c = ΣQ_ci with ψ0 = 1.0, ψ1 = –, ψ2 = 1.0' (EN 1990 A2) and loads on formwork, scaffolds DIN EN 12811 [V].
out: Q_c(t) timeline with phases; viz: timeline Gantt of load phases (see section 5).
ok: none. edge: partially built structure geometry → new `ConstructionStage` entity (event-sourced per stage).

### 2.7 EN 1991-1-7 Accidental actions (`E17`)
**E17.01 4.3 + Table 4.1-4.2 / Anprall von Strassenfahrzeugen auf Stuetzen und Waende (equivalent static)**
Rec [V]: Table 4.1 F_d,x / F_d,y (kN): motorways & country roads 1000 / 500; urban 750 / 375; courtyards/parking cars ≤ 2.5 t 50 / 25; lorries > 2.5 t 150 / 75. Application: on columns/walls 0.5 m to 1.5 m above the carriageway (rec) (resultant at h = 0.5 m? or 1.25 m); area 0.5 m height × 1.5 m width (or the member width if less). For overhead decks: F_d,x = 500 kN, F_d,y = 250 kN? [V] (4.3.1(2)); clearance of deck h < 5.0 m (rec) applies. Safety class by distance to carriageway: for structures with d ≥ 5 m (parking ≥ 3 m) no impact. NA-DE: Table NA.4.1 [V] (DIN EN 1991-1-7/NA: F_d,x = 1000 kN (motorways, Bundesstrassen, speed ≥ 80), 500 kN?...). Dynamic design: F_d = v_r √(k m) (C.1, C.2) with k ≈ 300 kN/m (rec equivalent stiffness), m = 1500 kg (cars), 30000 kg (lorry), speed v_r = 15 m/s? (rec). Energy: E = ½ m v² . out: resultant forces + patches (viz: red impact patch on column).
**E17.02 4.5 + Annex C.? / Anprall von Gabelstaplern** F_d,x = 5 W (W = net weight of loaded forklift, 5 W for rigid, 2.5 W?) at height 0.75 m [V]; **E17.03 4.? / Entgleisung (rail traffic)**, Tabellen 4.4–4.5 F_dx by v & distance [V]; **E17.04 4.6 / Ship impact** Annex C.4 (inland waterways F_dx: Table C.2 ship class I–VII with forward 1000..? kN) [V]; **E17.05 4.7 / Helicopter hard landing** F_d = C √(m): C = 3 × 10⁵? (3 √m kN?) F_d = 3 m? rec: F_d = C √m with C = 3 kN/√kg, m in kg for rooftop helipads [V]; **E17.06 Annex A / Robustheit, Gebaeudeklassen**
Table A.1 (consequence classes CC1 – CC3 for buildings, with CC2a lower and CC2b upper risk [V]): CC1: houses ≤ 4 storeys, agricultural; CC2a: ≤ 4 storeys hotels/offices ≤ 5 storeys and area per storey ≤ 2000 m^2; CC2b: hotels/apartments 5–15 storeys, retail < 5 storeys & 5000 m^2; CC3: > 15 storeys, grandstands > 5000 spectators etc. Strategies: CC1: no specific measures; CC2a: effective horizontal ties; CC2b: horizontal ties + vertical ties or notional removal of load-bearing element (column/beam/wall) + key element 34 kN/m^2; CC3: systematic risk analysis. Horizontal ties: peripheral T_p = 0.4 (g_k + ψ q_k) s L or 75 kN whichever is the lesser (A.5.1 (3) Eq. A.1) | internal T_i = 0.8 (g_k + ψ q_k) s L or 75 kN whichever is the lesser (A.2), ψ = ψ1 or ψ2; with s = spacing of ties, L = span [V; the "lesser" reading must be confirmed in the printed text]; vertical ties: tie force equals the greatest design vertical load from a floor (A.5.2). Notional element removal: remove one column/beam/wall (length ≤ 2.25 H?) with damage limit: min(15 % of floor area; 100 m²) per storey (A.7) [N rec]. Key elements: A_d = 34 kN/m^2 on any member and supporting structure, any direction (A.8) [N rec]. NA-DE: Robustheit per NDP: ties per DIN EN 1992/NA for concrete, tie requirement list; Brandschutz [V].
**E17.07 Annex B / Risikoanalyse** risk R = Σ P(H) P(D|H) C(D) with acceptance criterion; Event tree (B.2); out: R and ALARP check (R < R_acc). **E17.08 Annex C / Dynamic design for impact**: elastic F = v_r √(k m); plastic F_0 = ... y_max = ... ; deformation criterion E = ½ m v² = F_0 y + ... The plugin implements the rigid-body energy method: energy balance ½ m v² = ½ F_el y_el + F_pl (y_max − y_el); out: y_max ≤ y_allowed. **E17.09 Annex D / Innere Explosion (Gasexplosion)** p_d (kN/m^2): D.1 p_d = 3 + p_stat (p_stat = static design pressure of venting panels, ≥ 2 kN/m^2? ) when p_stat ... ; D.2 p_d = 3 + p_stat/2 + 0.04/(A_v/V)² (kN/m^2) with A_v/V ≤ ... ; adopt maximum? Both formulas are defined with applicability: p_stat = 2 kN/m^2? ; validity 5 ≤ V ≤ 1000 m^3, A_v/V < 1/.. . [V critical]. E17.10 Annex E is not used.
ok: ties: T_provided ≥ T_required; key element: R_d ≥ E_d(A_d = 34 kN/m^2); removal: structure remains stable with member removed (evaluated by user model in en1992-1999: link).
edge: accidental design situation uses ψ1,1/ψ2 per E90.17.

### 2.8 EN 1991-2 Traffic loads on bridges (`E2`)
**E2.01 4.2.2 Table 4.1 / Fahrstreifenaufteilung**
in: carriageway width w (m) between kerbs. f [N]: w < 5.4 m: n_1 = 1, w_l = 3.0 m, remaining w − 3 m; 5.4 ≤ w < 6.0: n_1 = 2, w_l = w/2, remaining 0; w ≥ 6.0: n_1 = Int(w/3), w_l = 3.0 m, remaining w − 3 n_1. Lane numbering: lane 1 (most adverse), 2, 3. The user can reposition lanes (E2.01a: worst position by influence-line search; each lane position variable → enumerated by influence surface). NA-DE: same lane layout [V].
**E2.02 4.3.2 Table 4.2 / Lastmodell 1 (LM1)** Tandem system TS: two axles of weights α_Qi Q_ik, axle spacing 1.20 m, wheel spacing 2.00 m, wheel footprint 0.40 × 0.40 m, wheel load α_Qi Q_ik / 2. UDL: α_qi q_ik. EN recommended [N]: lane 1: Q_1k = 300 kN, q_1k = 9.0 kN/m^2; lane 2: 200, 2.5; lane 3: 100, 2.5; other lanes: 0, 2.5; remaining area: 0, 2.5 kN/m^2. Adjustment factors α_Qi, α_qi, α_qr: recommended 1.0 [N]. NA-DE (DIN EN 1991-2/NA, Tabelle NA.4.? ) [V]: values the NA sets are the NDP; spec: α_Q1 = α_Q2 = α_Q3 =… *the printed table must be keyed*; interim default for tests: DE equivalent to DIN-Fachbericht 101: 300/200/100 kN and 9.0/2.5/2.5/2.5 kN/m^2 with α=1.0. TS placement: lane 1 TS at the worst location; at most one TS per lane; for local effects the two axles may be placed in one lane; the UDL only on unfavourable portions of the influence surface.
**E2.03 4.3.3 Table 4.2 / LM2** single axle β_Q Q_ak = 400 kN β_Q (rec β_Q = α_Q1), contact 0.35 × 0.60 m (rec), wheel spacing 2.0 m, also reduced when two wheels combined; LM2 applied only for short-element verification (orthotropic decks) at any lane position [N].
**E2.04 4.3.4 / LM3 special vehicles** Annex A / standard vehicles 600 (=600 kN) → 3600 kN: axle lines 150 kN × n_axles, spacing 1.50 m (axle lines, from 1200 to 3600 as combos of 150 kN and 200 kN axle); pressure area 0.15 m^2? NA-DE: LM3 SLW 60 / SLW 30 per DIN-Fachbericht (SLW 60 = 600 kN, axle 100 kN, 4 axles?) [V].
**E2.05 4.3.5 / LM4 crowd** q_fk = 5 kN/m^2 on relevant parts of deck, general verification of the bridge only; combination with LM1 not required [N].
**E2.06 4.3.6 / Dispersal** load spreading through pavement and slab at 45° to mid-plane [N] (wheel footprint increases t_pavement + ½ t_slab on each side). Out: footprint, used in en1992 punching. 
**E2.07 4.4.1 / Bremsen und Anfahren Q_lk** Q_lk = 0.6 α_Q1 (2 Q_1k) + 0.10 α_q1 q_1k w_1 L, bounded 180 α_Q1 ≤ Q_lk ≤ 900 kN (4.6) [N]; applied at pavement level along lane axis (L=loaded length, w_1 = 3 m). **E2.08 4.4.2 / Fliehkraft** Q_tk = 0.2 Q_v (r < 200 m); 40 Q_v/r (200 ≤ r ≤ 1500 m); 0 (r > 1500 m) with Q_v = Σ α_Qi (2 Q_ik), applied radially at carriageway level [N]. **E2.09 4.5 / Load groups** gr1a (TS + UDL + footway 3 kN/m^2), gr1b (single axle), gr2 (braking/centrifugal + vertical frequent), gr3 (pedestrian & cycle 5 kN/m^2), gr4 (crowd 5 kN/m^2), gr5 (special vehicles) – characteristic/frequent/QP values per Table 4.4a-b; **E2.10 4.6 / Fatigue load models** FLM1 (LM1 with 0.7 Q_ik, 0.3 q_ik), FLM2 (frequent lorries Table 4.7 five vehicles), FLM3 (single vehicle 4 axles of 120 kN, wheel contact 0.40 × 0.40, track 2.0 m, axle spacing 1.2 m, 6.0 m, 1.2 m), FLM4 (set of 5 equivalent lorries), FLM5 (traffic records); Annex B equivalent damage λ-method [V]; **E2.11 4.7 / Accidental: vehicle collision on supports**, deck: Fdx = 1000/500 kN (4.7.3.1 [V]), on soffit: 500 kN?; **E2.12 5 / Footbridge**: uniform load q_fk = 2.0 + 120/(L + 30) kN/m^2, 2.5 ≤ q_fk ≤ 5.0 (5.1) [N]; horizontal Q_flk = 10 % of total vertical load, ≥ 10 kN? [V]; service vehicle Q_sv: 2-axle 80 kN? (40+40) 'axle spacing 3.0 m, wheel 0.20 × 0.20'; **E2.13 5.7 / Dynamic models for footbridges** pedestrian walking f: 1.7–2.1 Hz vertical (1st harmonic 0.4 kN F_v), 0.9–1.1 horizontal; jogging 2.0–3.5 Hz; crowd density d = 0.5–1.5 p/m^2; comfort limits E90.27. **E2.14 6.3.2 / LM71** four 250 kN axles spacing 1.60 m + UDL 80 kN/m on both sides starting 0.8 m beyond the end axles, no limit; classified vertical loads α × values with α ∈ {0.75, 0.83, 0.91, 1.00, 1.10, 1.21, 1.33, 1.46} [N]. **E2.15 6.3.3 / SW/0 and SW/2** SW/0: two spans each 15.0 m load length, UDL 133 kN/m, spacing between loaded lengths 5.3 m (continuous bridges) α=1.21 ('classified'); SW/2: 150 kN/m, loaded length 25 m, spacing 7.0 m [N]. **E2.16 6.4 / Dynamic factor Φ** Φ_2 = 1.44/(√L_Φ − 0.2) + 0.82, 1.00 ≤ Φ_2 ≤ 1.67 (well maintained track); Φ_3 = 2.16/(√L_Φ − 0.2) + 0.73, 1.00 ≤ Φ_3 ≤ 2.00 (normal maintenance) (6.2–6.3) [N]; L_Φ determinant length (Table 6.2: simply supported girder L_Φ = L; continuous 1.2 L_mean...). Applied to LM71/SW/0 not SW/2? (SW/2 yes; HSLM no (dynamic analysis)). **E2.17 6.5 / Horizontal: centrifugal** Q_tk = (v²/(127 r)) f Q_vk, q_tk = (v²/(127 r)) f q_vk with f = 1 for v ≤ 120 km/h or L_f ≤ 2.88 m; f = [1 − (v − 120)/1000 · (814/v + 1.75) (1 − √(2.88/L_f))] for 120 < v ≤ 300 and L_f > 2.88 m (6.17) [N]; nosing Q_sk = 100 kN × α not multiplied by Φ; traction Q_lak = 33 kN/m L_a,b (≤ 1000 kN); braking Q_lbk = 20 kN/m L_a,b (≤ 6000 kN) for LM71, SW/0; 35 kN/m for SW/2 [N]. **E2.18 6.7 / Aerodynamic effects** from passing trains (kPa p_k = ±0.3–2.? ) [V]. **E2.19 6.8 / Derailment** design situations I and II: lateral line load 45 kN/m over 20 m for situation I etc. [V]. **E2.20 6.9 / Load groups rail** (Table 6.11): gr11–gr31 with ψ and characteristic/frequent; **E2.21 6.10 / Fatigue rail** standard train mix + LM71 with Φ; **E2.22 Annex F / dynamic analysis HSLM-A (A1..A10), HSLM-B** for v > 200 km/h: HSLM-A trains with N articulated coaches (A1: N=18, D=18 m, d=2.0 m, P=170 kN) A2 N=17 D=19 d=3.5 P=200 … A10 N=13 D=24 d=2.0 P=210 [V]; the plugin stores axle sequences; time integration is out of scope: request Dynamic result link from `fem`.
ok: LM loads produce effects via influence surfaces; combos with E90.25-26; verification pass/fail only in en1990.
edge: skew bridges, curved decks (lane centre-line along 3D curve), lane overlaps, w_l < 3 m.

### 2.9 EN 1991-3 Actions induced by cranes and machinery (`E3`)
**E3.01 2.5 Table 2.? / Dynamische Beiwerte φ_i** [N rec]:
- φ_1 = 1 ± a (0.1) for crane self-weight excitation: 1.1 (unfavourable) or 0.9; 
- φ_2 = φ_2,min + β_2 v_h (Table 2.5): HC1 β_2 = 0.17, φ_2,min = 1.05; HC2 0.34, 1.10; HC3 0.51, 1.15; HC4 0.68, 1.20; v_h = steady hoisting speed (m/s).
- φ_3 = 1 − (Δm/m)(1 + β_3): sudden release of payload fraction Δm/m; β_3 = 0.5 (grab/similar), 1.0 (magnets, spreader, with active release);
- φ_4 = 1.0 for rails well-maintained (otherwise 1 + ... for tolerance δ and wheel base);
- φ_5 = 1.0 ... 1.5 (linear or less: 1.0 ≤ φ_5 ≤ 2.0 depending on drive: 1.5 for smooth, 2.0 for abrupt) [V];
- φ_6 test load factor 1.0? (dynamic test 1.1?) [V]; φ_7 for buffer forces: 1.25 for linear characteristic; 1.25 – 1.75 non-linear [V].
**E3.02 2.7 Tab 2.2 / Lastgruppen (Load groups)** eight 'groups' (1–10) of simultaneous effects on runway girders: e.g. Group 1: φ_1 Q_C + φ_2 Q_H (max wheel + hoist); Group 2: ..., see table [V], to be encoded as `CraneLoadGroup{ id, factors{φ1..φ7} per Q_C, Q_H, H_L, H_T, H_S, H_B }`.
**E3.03 2.7 + 2.8 / Horizontale Lasten**: longitudinal H_L,i = φ_5 K / n_r (K = μ ΣQ_r,max... the drive force K = μ ΣQ_r, μ = 0.2 steel on steel (rec for steel on steel) [V]); transverse H_T,1,2 from acceleration of crane: H_T,i = ξ_2 M/a (ξ_2 = distribution factor), by M = drive moment; skewing H_S,i,j,k = f λ_s,j,k ΣQ_r; f = 0.3(1 − e^{−250 α}) ≤ 0.3 with α = α_F + α_V + α_0 (skew angle, rad) [N], λ_s coefficients from Table 2.? (guide means: rollers vs flanges) [V]. Buffer forces H_B = φ_7 v_1 √(m_c S_B) (2.? ) with S_B spring constant [N]. Tilting/uplift: H_TA from tilting of crane or lift [V].
**E3.04 2.9 / Pruefung und Last bei Ausser Betrieb**: test loads Q_T = 1.1 × (max hoist) static + 1.25 dynamic; out-of-service wind per E14 (crane wheel loads Q_r with wind) [V].
**E3.05 2.12 / Ermuedung** Annex A: damage-equivalent factors λ_i = λ_{i,1}...; stress cycle classes S_0–S_9 (Table 2.? ), Q_e = Q_{max} λ_i; load spectrum factor k_Q = (Σ Δ... ) ; E3.06 Annex B: dynamic factors... [V].
**E3.07 5 / Maschinelle Einwirkungen: Dynamic loads from machinery** (6): static equivalent for rotating machines F_dyn = m e ω² ... [V] (e eccentricity).
Design wheel loads per crane: arrangement of up to 4 wheels per rail end; load combinations of Q_r,max and Q_r,(max) with trolley position; viz: wheel loads at runway girders. out: Q_r,max(i), Q_r,(max), Q_r,min, H_L,i, H_T,i, H_S,i,j,k per wheel and group; **ok:** none; edge: wheel spacing and corner buffer; two-crane combination ψ factors (Table 2.? 'two cranes: 100 % + 50 %?' [V]).

### 2.10 EN 1991-4 Silos and tanks (`E4`)
**E4.01 Table 1.? / Bemessungssituationen Klassifizierung** Action Assessment Class AAC: AAC3: capacity > 10,000 t (or eccentric discharge e_o/d_c > 0.25 etc.), AAC1: capacity < 100 t, AAC2: all other silos, so exactly 100 t and 10,000 t are AAC2 (Table 2.1 as quoted by national annexes; registry rows `ndp.en1991.4.aac-class1-below|above`, unverified) [N]. Slenderness classification by h_c/d_c: slender h_c/d_c ≥ 2; intermediate 1.0 < h_c/d_c < 2.0; squat h_c/d_c ≤ 1.0; retaining h_c/d_c ≤ 0.4 with plan shape factors; d_c = inscribed circle diameter. **E4.02 Annex E / Schuettgutkennwerte** γ (unit weight), φ_i (internal friction), φ_w (wall friction by wall category D1–D4), K (lateral pressure ratio) with K_m = 1.1 (1 − sin φ_i) [N] (K_m: mean; K_s?), a_K = 1.2?) and the extreme values: γ_u = 1.15 γ_m? [V]. Table E.1 for ~20 bulk solids [V]. **E4.03 5.2 (5.1–5.5) / Janssen filling load, vertical walls** 
p_hf(z) = p_ho Y_J(z), p_ho = γ K z_o, z_o = A/(K μ U), Y_J = 1 − e^{−z/z_o}, p_wf = μ p_hf, p_vf = p_hf/K; N_wf (wall friction force per perimeter length) = ∫ p_wf dz = μ p_ho [z − z_o (1 − e^{−z/z_o})] [N]. z = depth below equivalent surface. Check 2 values (K high with μ low and K low with μ high) as design extremes; load multipliers: phf, pwf with K = 1.2 K_m? μ = 1.15? etc. [V]. Hopper: [V] 5.4: p_nf for hopper wall inclination β (Eqs 6.? 6.5-6.7), p_vf at transition F factor: p_v = (γ h_o ... ) Janssen vertical load at transition [N]: p_vft = γ z_o [1 − e^{−h/z_o}]... 
**E4.04 5.3 / Entleerungslasten** p_he = C_h p_hf; p_we = C_w p_wf with C_h = 1 + 0.15 C_op? : EN: Table 5.? C_h = 1.15 (slender, flow-controlled) etc. Mass flow: C_h = 1 + 0.15 (e_f/…). The simplified: C_h = 1.15, C_w = 1.10 for AAC2 slender [V]. Patch load: p_pf = 0.21 C_pf p_hf,max? i.e. E4.05 patch load p_p: p_pf = 0.2 p_hf (rec: 'patch load pp = 0.2 p_hf,max for ... (5.5.?)') with patch extent s = π d_c / 16 (width) [V]. E4.06 Thermal actions on walls ΔT; E4.07 Dust explosion pressure p_max; E4.08 Fluid tanks: hydrostatic p = γ_liquid z; slosh; **E4.09 Annex A,B,C Wind/pressure** and **fatigue**. 
out: horizontal wall pressure profile p_hf(z), p_wf(z), p_vf(z), hopper loads, patches; ok: none; edge: eccentric discharge, active/passive pressures via Rankine K_a=(1−sin φ)/(1+sin φ).

## 3. Schema-first entity model

Notation: `Entity { field: type [unit] = constraint }`; enums in `{a|b}`; `Id<T>` = stable string id; `Vec3 [m]`; `Label = LocalizedLabel::native(en, de)` (no default language). Stored in the `snapshot` facets (`artifact` lane) unless marked `ephemeral` (local or shared presence lane). All mutations are commands that emit events (no CRUD): the verbs listed under *Events*.

### 3.1 Shared kernel (new, in `⚖️compliance` or a new `🧱️actions` leaf shared by en1990/en1991)
```
NdpSet      { id, standard, clause, symbol, unit, recommended: Value, by_annex: Map<AnnexChoice,Value>, range: [Value]?, confidence: {verified|unverified} }
NdpChoice   { ndp_id, annex, value, justification: Label? }
Geometry    { vertices: Vec<Vec3 [m]>, faces: Vec<Face>, up: Vec3, north_deg [deg], origin_wgs84: (lat [deg], lon [deg], alt [m])? }
Face        { id, polygon: Vec<Id<Vertex>>, kind: {wall|roof|slab|deck|opening|parapet|canopy}, normal_hint }
Opening     { id, face: Id<Face>, rect: (u [m], v [m], w [m], h [m]), state: {closed|open|dominant|permeable(p [%])}, kind: {door|window|vent|damaged} }
Site        { location: GeoPoint, altitude_m [m], zones: ZoneLookup{snow_zone, wind_zone, thermal_zone, seismic_zone}, terrain: TerrainDef, orography: Orography?, municipality_ags: String? }
TerrainDef  { mode: {category(0|I|II|III|IV)|mixed_upwind(cat_a, cat_b, distance [m])|na_de_profile{binnenland|kueste|nordsee_inseln}|na_de_simplified}, per_sector: Vec<{from_deg, to_deg, TerrainDef}> }
Orography   { kind: {hill|escarpment|valley}, H [m], L_u [m], L_d [m], x [m], s_factor? }
```
Events: `SiteLocated`, `ZoneAssigned`, `TerrainSet`, `OpeningToggled`, `GeometryEdited`, `NdpChosen`.

### 3.2 EN 1990 entities
```
Project      { id, annex, structure_part[]: StructurePart, situations: Vec<SituationKind>, reliability: ReliabilityDecl }
StructurePart{ id, kind: {building|bridge_road|bridge_rail|footbridge|crane_runway|silo|tower}, geometry_ref, label }
ReliabilityDecl { cc: {CC1|CC2|CC3}, rc: {RC1|RC2|RC3}, dsl: {DSL1|DSL2|DSL3}, il: {IL1|IL2|IL3}, k_fi [-] = 0.9|1.0|1.1, dwl_category: 1..5, dwl_years [yr], reference_period [yr], beta_target: derived, beta_computed [-]? }
Action       { id, label, class: ActionClass, source: {manual|en1991(part,clause)|en1998|material}, values: Vec<ValueSet>, group: Id<ExclusiveGroup>?, psi_key: PsiKey, footprint: Footprint?, effect_per_member: Vec<MemberEffect> }
ActionClass  { time: {permanent|variable|accidental|seismic}, spatial: {fixed|free}, origin: {direct|indirect}, nature: {static|dynamic|quasi_static}, favourable_split: bool }
ValueSet     { kind: {char|char_sup|char_inf|freq|qp|comb|accidental}, value: f64, unit: {N|N/m|N/m2|N/m3|Pa|K|m|...} }
PsiKey       { table: {A1.1|A2.1|A2.2|A2.3|custom}, row: String, psi0, psi1, psi2: f64 = 0..1 }
ExclusiveGroup { id, members: Vec<Id<Action>>, rule: {one_of|sum_with_cap|traffic_group} }
PartialFactorSet { annex, state: {EQU|STR_B|STR_C|GEO_C|ACC|SEI|SLS}, gamma_G_sup, gamma_G_inf, gamma_Q_by_class: Map<Class,f64>, gamma_P, xi?, source_ndp: Id<NdpSet> }
Combination  { id, family: {STR_6.10|STR_6.10a|STR_6.10b|EQU|ACC|SEI|SLS_char|SLS_freq|SLS_qp|FAT}, lead: Id<Action>?, terms: Vec<{action, factor, value_kind}>, source_equation: String }
Member       { id, label, rd_str,rd_geo,rd_equ_stab,rd_equ_destab,rd_fat [N or Nm], span [m], deflection: DeflectionSet, vibration: {f1 [Hz], f_min [Hz]}, material_link: ArtifactRef? }
DeflectionSet{ w_c, w_1, w_2, w_3 [m], limit_expr: String, limit_value [m], family }
MemberEffect { member_id, action_id, influence [-], effect_kind: {N|V|M|T|u|w|a|R}, curve: InfluenceLine? }
SlsCriterion { id, element, quantity, family: {char|freq|qp}, limit_expr, unit, source: ClauseRef }
BridgeSls    { (existing) + comfort_class, footbridge_a_v/a_h limits }
Result       { combination, member, effect_kind, E_d [N...], R_d, utilisation [-], governing_action }
ReliabilityAnalysis { variables: Vec<RandomVar{ name, dist: {normal|lognormal|gumbel|weibull}, mean, cov [-], role: {R|E} }>, g_expr, method: {FORM|SORM|MC}, tol, result: {beta, alpha[], x_star[], pf} }
TestSeries   { property, samples: Vec<f64>, vx: {known(v)|unknown}, eta_d, dist: {normal|lognormal}, result: {m, s, kn, x_k, x_d} }
```
Events: `ActionDefined`, `ActionValueChanged`, `ActionGrouped`, `PsiOverridden`, `PartialFactorChosen`, `CombinationSetGenerated` (derived), `MemberResistanceSet`, `InfluenceAssigned`, `ReliabilityDeclared`, `SamplesAdded`, `FormRequested/FormProgress/FormCompleted/FormCancelled`.

### 3.3 EN 1991 entities
```
Building     { id, geometry_ref, eaves_h [m], ridge_h [m], storeys[], roof_planes[], walls[], openings[] }   // replaces scalar height/width/depth
Storey       { id, level_z [m], floor_use: UseCategory code, area_polygon, partitions: Vec<Wall{ g_k [N/m], line }> }
SelfWeightLayer { id, material: Id<Material>, thickness [m], area_ref, gamma_override [N/m3]? }
UseCategoryAssign { storey, code: {A1,A2,A3,B1,..,K}, q_k [N/m2], Q_k [N], reduction: {alpha_a? , alpha_n?} }
Snow         { site, s_k [N/m2] (derived), roof_cases: Vec<RoofSnow{ roof_plane, mu_shapes: Vec<SnowZone{ polygon, mu, s [N/m2], tag }>, ce, ct, obstructions: Vec<Obstruction{ h [m], position }>, parapet h_p [m], snow_guard: bool, overhang }> } , ngl_accidental: bool }
Wind         { site, zone: u8, terrain: TerrainDef, v_b,0 [m/s] (derived), profile: Profile(q_p(z) samples), faces: Vec<WindFace{ face_id, wind_dir_deg [deg], zone_label: {A|B|C|D|E|F|G|H|I|J|K|L|M|N|O}, polygon, z_e [m], c_pe10, c_pe1, c_pi_case, w_e, w_i, w_net [Pa], area [m2] }>, dynamics: {n1 [Hz], delta [-], m_e [kg/m], mode_exponent zeta [-], cscd [-], a_max [m/s2]}, directions: Vec<angle>, structures: Vec<FreeStanding{ kind: {canopy|wall|sign|chimney|mast|bridge_deck}, ...}> }
Thermal      { site, t_min,t_max [degC], deck_type: {1|2|3a|3b|3c}, surfacing_mm [mm], t0 [degC], members: Vec<{L [m], alpha [1/K], restraint}> , results: {te_min, te_max, dT_con, dT_exp, dT_m_heat, dT_m_cool, dL} }
Execution    { stages: Vec<ConstructionStage{ id, duration [d], loads: Vec<{kind: Qca..Qcf, value, area}>, climatic_return_period [yr] }> }
Accidental   { cases: Vec<AccidentalCase{ id, kind: {vehicle_impact|forklift|ship|derail|explosion|column_removal|ties|key_element|helicopter}, params... }>, consequence_class: {CC1|CC2a|CC2b|CC3} , ties: Vec<TieRun{ line, s [m], L [m], g_k [N/m2], q_k [N/m2], psi, T_req [N], T_prov [N] }> }
Fire         { compartment{ A_f,A_t,A_v [m2], h_eq [m], b [J/m2s0.5K], q_fk [J/m2], deltas, growth }, curve: {iso834|external|hydrocarbon|parametric|imported(series)|localised(D, Q [W], H [m])}, results: series θ_g(t), θ_max, t_eq, hnet }
Traffic      { deck: DeckAxis{ polyline3d [m], width profile, kerb_to_kerb w(s) [m], curvature r(s) [m] }, lanes: Vec<Lane{ id, offset(s) [m], width [m], rank 1..n }>, vehicles: Vec<VehicleInstance{ model: {TS|LM2|LM3|FLM3|LM71|SW0|SW2|HSLM|crowd|service}, s [m], lane, axle_seq }>, groups, braking, centrifugal, footway, adjustments α..., rail: {track_axis, v [km/h], alpha_class, L_phi [m], phi_choice: {2|3}} }
Crane        { runway: {span [m], rails: [Rail{ axis, wheel_positions[] }]}, class: HC1..4, S_class: S0..S9, Q_C [N], Q_H [N], wheel_groups: Vec<WheelGroup{ n_w, spacing [m], Q_r_max [N], Q_r_min [N] }>, v_h [m/s], drive: {mu, n_drives}, skew: {alpha}, results per load group }
Silo         { geometry: {shape: {circular|rect|polygon}, d_c [m], h_c [m], hopper: {angle β [deg], h_h [m]}, outlet: {e_o [m]}}, bulk: {γ [N/m3], φ_i [deg], φ_w [deg or mu], K, wall_category: D1..D4}, filling_eccentricity e_f [m], result profiles }
```
All `assumed_*` fields are removed. Computed fields are `derived` (never stored as input; stored only in `snapshots`' result lane as cache with source-hash).

Events: `RoofPlaneAdded`, `RoofPlaneEdited`, `SnowZoneRecomputed`, `WindDirectionStepped`, `TerrainSectorSet`, `LanePositioned`, `VehicleDragged`, `WheelGroupEdited`, `FireCurveSelected`, `TieRunDrawn`, `ColumnRemoved`.

### 3.4 Projection of the existing fields (migration of vocabulary, no compatibility layer)
`snow_zone/altitude/en_sk/north_german_lowland_snow` → `Site.zones.snow_zone`, `Site.altitude_m`, derived `Snow.s_k`, `Snow.ngl_accidental`. `wind_zone/en_vb/terrain_category/mixed_terrain_*/orography_factor/coast_or_island/air_density` → `Site`+`Wind.terrain`/`orography`. `height/width/depth` → `Building` geometry. `floors/self_weight_elements/roofs/wind_faces` → `Storey`/`SelfWeightLayer`/`RoofSnow`/`WindFace`. `bridge_*` → `Traffic`. `crane_*` → `Crane`. `silo_*` → `Silo`. `fire_*` → `Fire`. `accidental_cases` → `Accidental`. `construction_*` → `Execution`. The corresponding old snapshot fields are removed in one change set; fixtures/examples are rewritten by hand.

## 4. Visualisation specification

General: one `Viewport3D` (orbit/pan/zoom, section planes, X-ray, measure) per artifact plus linked `Diagram2D` panes; all colour scales are accessible (colour-blind-safe diverging scale + hatching + numeric labels on hover), light/dark theme tokens, units switchable, German/English labels. Colour legends are always shown with units. Everything is derived from the same state as the numbers (single source of truth). Utilisation colour: green ≤0.7, amber 0.7–1.0, red > 1.0 (also shape: ▲ marker for > 1.0).

### 4.1 EN 1990 views
- **V90.1 Combination matrix** (2D): rows = actions with class icons, columns = combination families (STR 6.10, EQU, ACC, SEI, SLS ×3); cells show γ·ψ factor and active/lead highlighting; click a column → the 3D view shows the resulting load pattern; sortable by governing member.
- **V90.2 ψ and γ explainer** (2D): bar triple ψ0/ψ1/ψ2 per action with edit handles; table of γ per state with annex badge `EN`/`NA-DE`, unverified-NDP hatch.
- **V90.3 Reliability** (2D): β–pf chart (log pf vs β) with RC1-3 target bands; FORM design-point circle (α_E, α_R) with markers; K_FI gauge.
- **V90.4 Member utilisation** (3D on the structure from material artifacts): beams coloured by η_STR; EQU arrows (stabilising green, destabilising red) at supports with tipping axis; governing combination label per member (callout).
- **V90.5 Deflection** (3D): exaggerated deflected shape (scale slider ×1–×200) with w_c, w_1, w_2, w_3 stacked dimension lines per Fig A1.1, limit L/250 line; frequency mode shape animation when f1 < f_min.
- **V90.6 Bridge SLS** (3D): deck with acceleration contour, twist arrows per 3 m, deflection cap.
- **V90.7 Load case stacks** (3D): toggle any combination to see arrows of every action scaled with γψ (stack view) over the structure; arrow colour by class.
- **V90.8 Annex D** (2D): sample dot-plot with m, k_n V band, X_k line.

### 4.2 EN 1991 views
**E11** floors: 3D storey plates coloured by category (categories legend), q_k as extruded pressure slab (height ∝ q), partition walls as red line elements with Δq_k badge, concentrated Q_k as a pin at selected point with 50×50 footprint; vehicle (car park) 3D wheel patches; reduction factors as annotations (α_A, α_n); 2D: category lookup table with range sliders. Self-weight: exploded layer stack of floor build-up with kN/m^2 per layer and sum.
**E12** fire: 2D θ–t chart for all curves overlaid (ISO, external, HC, parametric, imported) with t_eq marker and R-class bands; 3D compartment box with openings (vent factor), plume cone for localised fires (L_f, z_0, flame impingement), colour-coded beam flux/temperature along span (h_net), exposure arrows. Cooling-phase highlighted.
**E13** snow: 3D roof planes coloured by s (kN/m²) with legend; drift wedges (triangular prisms with μ2 and l_s labels) behind obstructions/parapets; sliding snow arrow bands over eaves; snow overhang bar; cylindrical-roof distribution; per-load-case switcher (undrifted / drifted left / drifted right) with animated morph; map picker with Zones 1,1a,2,2a,3 and altitude slider showing s_k live; 2D s_k vs altitude chart per zone with current point.
**E14** wind: 3D building envelope with c_pe zones A–E on walls and F–J (K–O) on roofs, each face coloured by w (positive/negative, diverging scale), labels with zone letters and c_pe; wind direction compass dial with 15° stepping or free angle (shows the corresponding table θ = 0/90 and cross-case); internal pressure arrows; q_p(z) vertical profile (2D) with strip heights (z_e) overlaid on the building elevation; terrain sector wheel (per sector category with fetch); Mischprofil blending band; dynamics: mode shape, vortex-shedding map (v_crit vs v_m), acceleration at top; free-standing canopies/walls with c_p,net zones; bridge deck cross-section with F_W resultants.
**E15** thermal: 3D bridge deck with temperature gradient shading (linear and non-linear profile plotted next to cross section), expansion arrows at joints with ΔL labels, bowing exaggeration; 2D plot of T_e,min/T_e,max versus T_min/T_max (Fig 6.1) and the combination domain (ΔT_N vs ΔT_M, ω factors) as a polygon.
**E16** execution: timeline (Gantt) with stage loads, 3D partial structure per stage (ghost of full structure), formwork pressure diagram on formwork wall (hydrostatic triangle with cap), storage area hatch with Q_cb.
**E17** accidental: 3D impact patch (0.5 m × 1.5 m rectangle) in red at the selected column/wall height with vehicle silhouette and approach arrow; tie network overlay (peripheral thick, internal thin lines labelled T_req/T_prov, red when T_prov < T_req); notional-removal: click a column to ghost it out and display the remaining area bounds (15 %/100 m²) as a hatched region; key element 34 kN/m² pressure arrows; explosion pressure on enclosing faces with venting panels; energy-balance chart (F–y curve, area = ½ m v²).
**E2** traffic: 3D bridge deck (polyline with width), notional lanes numbered 1..n coloured, remaining area greyed; LM1 tandems drawn as 4 wheel footprints per lane (axle 1.2 m × wheel 2.0 m, 0.4×0.4) with live drag along the deck; UDL as translucent slab strip; LM2 axle footprint; LM3 special vehicle strip; crowd load hatch; braking/centrifugal arrows at carriageway level; influence-surface heat map under the vehicle; rail: LM71 four-axle train with UDL, SW/0 and SW/2 as bar loads with gap spacing, nosing/traction/braking arrows, centrifugal at 1.8 m above rail head; footbridge pedestrian crowd; 2D diagrams: Φ–L_Φ curve, LM71 load diagram with axle positions, α classification table, q_fk–L curve.
**E3** cranes: 3D runway beams with crane bridge and trolley; wheel loads Q_r,max/Q_r,min as arrows at wheel positions with numeric labels, horizontal forces H_L,H_T,H_S at rail head, buffers; skew arrows; group selector; 2D: φ_2 versus v_h by HC class, influence line of the runway for the moving wheels (envelope).
**E4** silos: 3D cylinder/rect/polygon silo with transparent wall coloured by p_hf(z), arrows for p_wf and p_vf, hopper section with p_n/p_t, patch load as an extruded rectangle at an adjustable height/angle; 2D: pressure profile p(z) for filling/emptying, Janssen curve, bulk-material table.

### 4.3 Common interaction affordances in 3D
Hover: tooltip with clause id + formula + inserted numbers; click: opens the catalogue entry in the side panel; pin result labels; section/clipping planes; deterministic camera presets (iso, plan, elevations); screenshot/export to SVG/PNG/glTF for the report; accessibility: keyboard navigation of faces/zones, screen reader text alternatives that state e.g. "Zone F, roof, c_pe,10 = −1.8, w = −1.2 kN/m²".

## 5. Realtime editor interactions

Principle: every edit is a command → event → incremental recompute → views update in the same frame budget; expensive jobs (FORM, dynamic wind, influence search) run async with progress and cancel; no JSON text editing; every numeric field has unit, range, presets, and a "why" popover linking the clause. The undo/redo stack is event-sourced.

### 5.1 EN 1990 editor
- Wizard: structure kind → annex (EN/NA-DE, no preselected) → CC/RC/DSL/IL with consequence descriptions → working life.
- Action table with inline class chips; drag an action into an `ExclusiveGroup`; ψ chips with sliders (disabled for NA-DE verified values, enable via explicit "override" toggle).
- Combination matrix cells are clickable: shows contributions as a stacked bar; live utilisation per member in the member list; slider for each G_k and Q_k (live η update, < 16 ms for ≤ 200 combinations).
- Preset library: office B, residential A, parking F, warehouse E, bridge gr1a, footbridge.
- Reliability panel: β slider; FORM run button with progress/cancel; immediate K_FI suggestion.
- Remedies: "increase section" opens the member artifact; "reduce lead action" suggests the load reduction factor needed (η → 1).

### 5.2 EN 1991 editor (per part)
- **Site**: map + PLZ search → snow zone, wind zone, T_min/T_max, altitude; altitude slider updates s_k live; override flags.
- **Building envelope**: direct manipulation of footprint polygon (vertex handles), eaves/ridge height handles, pitch angle handle on the roof plane, parapet handle; openings dragged on facade (dominant opening toggle) and live c_pi.
- **Wind**: compass handle to rotate wind direction (snap 0/90/180/270 and free); live zone recolouring; terrain sector wheel with drag arcs; roughness category dropdown with example photos; q_p(z) profile displayed with the current building as overlay; dynamic sensitivity banner (n_1 field editable from FEM or from 46/h).
- **Snow**: roof plane handles; obstruction height drag; toggles for load case (undrifted/drifted); snow guard checkbox; the map zone switch.
- **Floors**: draw polygons per storey, choose category from icons; drag partition wall lines (g_k computed from wall type); reduction factors toggle shows α_A/α_n live.
- **Fire**: curve selector with cards; compartment sliders for A_v, h_eq, q_f,k with live θ–t curve; "required R class" slider; t_eq marker moves.
- **Thermal**: deck type picker with cross-section sketches, surfacing thickness slider, joint length handle; live ΔL and bearings.
- **Execution**: stage timeline drag; load chips per stage.
- **Accidental**: drag impact patch up/down the column and choose road class; draw tie runs by clicking along beams; click column to remove; key element radius.
- **Traffic**: drag lanes across the carriageway (snap to 3 m multiples with remaining area auto), drag tandem along the deck (live influence-line effect), toggle LM1/LM2/LM3/LM4, adjustment α sliders (locked for NA-DE unless override), rail class selector with α chips and speed slider (live f factor and Φ).
- **Crane**: drag wheel group positions, change span and hoist class; wheel-loads recalculated; load-group tabs.
- **Silo**: shape + diameter/height handles, bulk-material picker (table) with live Janssen profiles, patch load drag.
Common: "Explain" toggle prints the formulas with numbers substituted; "Compare annex" splits the screen EN vs NA-DE.

### 5.3 Performance budgets and async
Pressure/snow/lane recomputes ≤ 16 ms (incremental per face/zone); influence surface maps async progressive; FORM, c_s c_d iteration, parametric fire series ≤ 100 ms but cancellable; all long operations emit `…Progress{percent}` and honour `…Cancelled`.

### 5.4 Offline/multi-user
Local-first: all lookups (zone maps, PLZ→zone, tables) shipped as data assets; presence lane: cursor and selection of other users per roof/face/lane; shared ephemeral: transient drag preview; persisted: committed events on drop.

## 6. Verification strategy (tests and third-party oracles)

Per catalogue entry at least one language-agnostic test (JSON in `🧪️tests/<id>.case.json` with `inputs`, `annex`, `expected`, `tolerance`, `source`) executed by every implementation (Rust now, TS/Python later), plus an oracle comparison via independent Python in `🔮️oracles` using only well-known libraries:
- `scipy.stats.norm` / `statistics.NormalDist` for β, pf, FORM, k_n fractiles; `numpy` for integrals (Janssen wall friction, R_h integrals); `scipy.integrate` for Annex B filtering function checks.
- For fire: independent implementation of the curves plus (if installed) `ofire`-style libraries or the SFPE tables; closed forms already in section 7.
- For combinations: brute-force enumeration (itertools) of leads/ψ.
- Wind/snow: independent table re-key from a second source (DIN/ÖNORM/BS NA cross-values where the same EN formula applies; e.g. EN-recommended terrain II/III with `eurocodepy` if available [V]).
- Property tests (hypothesis-like): monotonicity of ψ, c_prob(p) increasing in p, q_p(z) non-decreasing for z ≥ z_min, μ1(α) continuity at 30° and 60°, α_A ≤ 1, LM lane count sums to w, total Janssen vertical force balance (γ A z − A p_v = ∫ μ U p_h dz) with tolerance 1e-9.
- Table fidelity: every data asset has a `checksum` and two key-entry files with a diff test (double-keyed). Unverified rows block compliance stamping.
- Golden files for 3D: pressure zone polygons (area sums equal face area ± 1e-9), lane geometry (sum widths = w), snow zone polygons (cover the roof).
- Language/label tests: all labels exist in en and de, no default language; accessibility: names for each 3D element.
- Cancellation tests: FORM and cscd iterations stop within one iteration; progress monotone.

## 7. Worked examples (reference values for tests)

All values were produced by `en1990-en1991-examples.py` (stdlib only) and hand-checked as noted. Units: forces kN, kN/m, kN/m^2, m, temperatures °C unless stated; pf dimensionless. Tolerance for tests: relative 1e-6 unless "rounded" is stated. Annex is NA-DE where Germany specific, otherwise EN (recommended, `[V]` data still to be keyed).

### 7.1 EN 1990 (15 examples, 20 values)
Hand-check notes: E90-02 and E90-03 coincide at 109.5 by arithmetic (office lead 112.5 governs). E90-01: 67.5 + 30 + 9 + 6 = 112.5; E90-06 EQU: 0.9·400 = 360 vs 1.1·120 + 1.5·60 = 132 + 90 = 222 ⇒ η_EQU = 222/360 = 0.617; E90-10: Φ(−3.8) = 7.23×10⁻⁵ standard table value; E90-12: σ_ln = √ln(1.01) = 0.09975, R_d/μ = exp(−0.8·3.8·0.09975)/√1.01 = 0.7348, R_k/μ = exp(−1.645·0.09975)/√1.01 = 0.8445, γ_R = 1.1493; E90-13: 100(1 − 2.18·0.08) = 82.56; E90-14: 5·10·6⁴/(384·15000) = 0.01125 m.
| ID | Case | Result |
|---|---|---|
| E90-01 | STR (6.10) office lead, DE psi: 1.35*50+1.5*20+1.5*0.6*10+1.5*0.5*8 | 112.5 |
| E90-02 | STR (6.10) wind lead | 109.5 |
| E90-03 | STR (6.10) snow lead | 109.5 |
| E90-04 | (6.10a) 1.35G+1.5*0.7*Q1+1.5*0.6W+1.5*0.5S [EN only] | 103.5 |
| E90-05 | (6.10b) 0.85*1.35G+1.5Q1+1.5*0.6W+1.5*0.5S [EN only] | 102.375 |
| E90-06 | EQU stabilising 0.9*400, destabilising 1.1*120+1.5*60 | 360; 222 |
| E90-07 | Char (office lead) 50+20+.6*10+.5*8 ; Freq (office lead) 50+.5*20+0 ; QP 50+.3*20 | 80; 60; 56 |
| E90-08 | Accidental Ad=100: 50+100+psi11*20 (office .5)+psi2 wind 0 | 160 |
| E90-09 | Seismic AEd=40: 50+40+.3*20 | 96 |
| E90-10 | pf=Phi(-3.3) | 0.000483424 |
| E90-10 | pf=Phi(-3.8) | 7.2348e-05 |
| E90-10 | pf=Phi(-4.3) | 8.53991e-06 |
| E90-10 | pf=Phi(-4.2) | 1.33457e-05 |
| E90-10 | pf=Phi(-4.7) | 1.30081e-06 |
| E90-10 | pf=Phi(-5.2) | 9.96443e-08 |
| E90-11 | KFI RC3: 1.1*1.5, 1.1*1.35 | 1.65; 1.485 |
| E90-12 | lognormal V=.1 beta 3.8 aR .8: Rd/mean, Rk/mean, gammaR | 0.734754; 0.844453; 1.1493 |
| E90-13 | Annex D n=6, Vx unknown kn=2.18, mean 100 sd 8: Xk=m(1-kn V) | 82.56 |
| E90-14 | Deflection 5qL^4/384EI q=10kN/m L=6 EI=1.5e4 kNm2 ; L/250 | 11.25; 24 |
| E90-15 | Bridge gr1a TS+UDL: 1.35*1000+1.35*(300+150)+1.5*.6*80 | 2029.5 |


### 7.2 EN 1991-1-1 (10 examples)
Hand-check notes: E11-01: 0.20·25 = 5.00, 0.06·22 = 1.32. E11-03: 5/7·0.7 + 10/40 = 0.5 + 0.25 = 0.75. E11-05: (2 + 3·0.7)/5 = 0.82. E11-06: 14·0.115·2.6 = 4.186 + 0.4 = 4.586 kN/m ⇒ NA-DE band 3-5 kN/m ⇒ Δq = 1.2 kN/m^2 (note E11-09 lists the three DE bands added to q_k = 2.0). E11-07: 10/0.04 = 250. Note: α_A example is EN-only (NA-DE forbids area reduction [V]).

| ID | Case | Result |
|---|---|---|
| E11-01 | slab 0.20 m RC 25 kN/m3 + screed 0.06 m 22 kN/m3 | 5; 1.32; 6.32 |
| E11-02 | steel 77.0 vs 78.5 kN/m3 plate 10 mm kN/m2 | 0.77; 0.785 |
| E11-03 | alphaA=5/7*psi0+A0/A, A=40,A0=10 | 0.75 |
| E11-04 | alphaA A=10 -> capped 1.0; A=200 -> C-D floor 0.6 | 1; 0.6 |
| E11-05 | alphan n=5 psi0 .7 | 0.82 |
| E11-06 | wall line load: 14*0.115*2.6 + plaster 0.4 kN/m ; DE addition 1.2(<=5) | 4.586 |
| E11-07 | wheel 10 kN on 0.2x0.2 -> kN/m2 | 250 |
| E11-08 | Balcony DE Z: 4.0 kN/m2*3.0 m depth*1 m | 12 |
| E11-09 | Office B1 DE 2.0+partition 0.8 / 1.2 / 1.6 (g<=3/5/6) | 2.8; 3.2; 3.6 |
| E11-10 | fill soil 0.5 m x 20 | 10 |

### 7.3 EN 1991-1-2 (9 examples)
Hand-check notes: E12-01: 20 + 345·log10(241) = 20 + 345·2.38202 = 841.8; t=60: log10(481) = 2.68215 ⇒ 945.3. E12-04: A_t = 2·50 + 4·7.07·3 = 184.84 m^2, O = 5·√2/184.84 = 0.03826, Γ = (0.03826/0.04)² = 0.9147, q_f,d = 420·0.8 = 336 (δ = 1), q_t,d = 336·50/184.84 = 90.89, t_max = 0.2e-3·90.89/0.03826 = 0.4752 h > t_lim = 0.333 h ⇒ ventilation controlled, θ_max = 822.2. E12-05: 420·0.8·1.5·1·1 = 504. E12-06 (units W/m^2): 25·341.8 + 0.8·5.67e-8·(1115.0⁴ − 773.15⁴) = 8545 + 53,862 ≈ 62.4 kW/m^2. E12-07: (100 + 25)/(135 + 75) = 0.5952. E12-08: w_f = (6/3)^0.3·(0.62 + 90·0.3⁴) = 1.2311·1.349 = 1.6608, t_e,d = 504·0.07·1.6608 = 58.6 min. E12-09: Q in W (2e6), L_f = −1.02 + 0.0148·2e6^0.4 = 3.885 m.

| ID | Case | Result |
|---|---|---|
| E12-01 | ISO 834 t=15,30,60,90,120 | 738.6; 841.8; 945.3; 1006; 1049 |
| E12-02 | hydrocarbon t=5,30,60 | 947.7; 1097.7; 1100 |
| E12-03 | external t=5,30,60 | 588.5; 680; 680 |
| E12-04 | parametric Af=50 Av=5 heq=2 At=2*50+4*7.07*3 b=1160: O,Gamma,qtd,tmax(h),theta_max | 0.0382551; 0.914657; 90.8894; 0.475176; 822.168 |
| E12-05 | qfd=qfk*m*dq1*dq2*dn office 250 m2 | 504 |
| E12-06 | hnet ISO 30min tg=841.8 tm=500 W/m2 | 62407 |
| E12-07 | eta_fi Gk=100 Qk=50 psi11=.5 | 0.595238 |
| E12-08 | te,d=qfd*kb*wf  kb=.07 H=3 av=.1 ah=0 | 1.66081; 58.5935 |
| E12-09 | Heskestad D=1 m Q=2e6 W: Lf,z0,theta(z=3 m) | 3.88539; 0.716774; 883.85 |

### 7.4 EN 1991-1-3 NA-DE (12 examples)
Hand-check notes: E13-01: (440/760)² = 0.33518, 0.25 + 1.91·0.33518 = 0.8902 ≥ 0.85 ⇒ s_k = 0.890; s = 0.8·0.890 = 0.712. E13-02: (940/760)² = 1.5298, 0.31 + 2.91·1.5298 = 4.762. E13-03: zone 1 A = 50: max(0.65, 0.19 + 0.91·0.0625 = 0.2469) = 0.65 ⇒ 1a: 0.8125, s_Ad = 2.3·0.65 = 1.495. E13-05: 0.8·20/30 = 0.5333. E13-07: 2·1/0.85 = 2.353 ⇒ capped 2.0; l_s = max(5, 2h = 2) = 5 m. E13-08: d = 0.9/3 = 0.3 m, k = min(10, 0.9) = 0.9, s_e = 0.9·0.81/3 = 0.243 kN/m. E13-09: 0.2 + 10·2/12 = 1.8667. E13-10: ln(−ln 0.99) = −4.6001, 1 + 0.4·(0.45 + 0.78·4.6001) ... = (1 + 0.4·3.1381)/2.036 = 1.1077.

| ID | Case | Result |
|---|---|---|
| E13-01 | Zone 2 A=300: sk, s=0.8*sk | 0.890194; 0.712155 |
| E13-02 | Zone 3 A=800 | 4.76166 |
| E13-03 | Zone 1a A=50 ; sAd=2.3*sk(zone 1, NGL) | 0.8125; 1.495 |
| E13-04 | Zone 1 A=900 ; zone 2a A=100 | 1.89404; 1.0625 |
| E13-05 | mu1(40deg) | 0.533333 |
| E13-06 | multi-span alpha=30: mu2 | 1.6 |
| E13-07 | obstruction h=1 sk=.85: mu2 raw, capped, ls | 2.35294; 2; 5 |
| E13-08 | overhang s=.9: d=s/3, k=min(3/d,d*gamma), se=k s^2/gamma | 0.3; 0.9; 0.243 |
| E13-09 | cylindrical b=12 h=2 mu3 | 1.86667 |
| E13-10 | Annex D V=.4 n=100 | 1.10768 |
| E13-11 | snow guard Fs=s*b*sin(alpha) s=1.0 b=4 alpha=30 | 2 |
| E13-12 | Ce windswept .8 Ct 1 mu1 .8 sk .85 | 0.544 |

### 7.5 EN 1991-1-4 (16 examples)
Hand-check notes: E14-01: ½·1.25·22.5² = 316.4 N/m^2 etc. E14-02: WZ2: 1.5·0.3906 = 0.5859 (z=5), 1.7·0.3906·1 = 0.6641 (z=10), 1.7·0.3906·3^0.37 = 0.9971 (z=30), 2.1·0.3906·10^0.24 = 1.4255 (z=100). E14-03: k_r = 0.19·6^0.07 = 0.2154, c_r = 0.2154·ln(33.33) = 0.7553, v_m = 18.88, I_v = 1/3.5066 = 0.2852, q_p = (1 + 1.9963)·0.5·1.25·18.882² = 0.6677. E14-04: terrain II: c_r = 0.19·ln(200) = 1.0067, I_v = 0.1887, q_p = 0.9189. E14-07: e = min(30, 24) = 24, A = 4.8, B = 20 − 4.8 = 15.2. E14-08: h/d = 0.6 ⇒ D = 0.7 + 0.1·(0.35/0.75) = 0.7467, E = −0.3 − 0.2·(0.35/0.75) = −0.3933, w_D = 0.9646·0.7467 = 0.7202. E14-09 areas: F 14.4 (each of two), G 43.2, H 288, I 240 (b·d = 600 = 2·14.4 + 43.2 + 288 + 240). E14-10: −2.5 + 0.7·log10(3) = −2.1660. E14-12 (zs = 24 m, α = 0.67 + 0.05 ln 0.3 = 0.6098, L = 300·(0.12)^0.6098 = 82.34): B² = 0.5517, n_1 = 1.15 Hz, S_L = 0.05392, R² = 1.5737, ν = 0.9895 Hz, k_p = 3.742, c_s c_d = 1.3435 (⇒ dynamic procedure governs, > 1; the building is sensitive). E14-13: c_prob(5 yr) = √((1 − 0.2·ln(0.22314))/(1 − 0.2·ln(0.020203))) = 0.8545. E14-14: 0.5·1.5/0.18 = 4.167 m/s, Sc = 2·0.05·300/(1.25·0.25) = 96. E14-15: ½·1.25·625·2.6·120/1000 = 121.9 kN (C is a [V] input).

| ID | Case | Result |
|---|---|---|
| E14-01 | DE vb0 WZ1..4 ; qb0 kN/m2 | 0.3164; 0.3906; 0.4727; 0.5625 |
| E14-02 | Binnenland qp WZ2 z=5,10,30,100 kN/m2 | 0.5859; 0.6641; 0.9971; 1.4255 |
| E14-03 | EN cat III z0=.3 vb=25 z=10: kr,cr,vm,Iv,qp | 0.215389; 0.755275; 18.8819; 0.28518; 0.667652 |
| E14-04 | EN cat II z0=.05 vb=25 z=10 | 0.19; 1.00668; 25.167; 0.188739; 0.918863 |
| E14-05 | EN cat I,0,IV z=10 vb=25 qp | 1.08145; 1.16583; 0.459442 |
| E14-06 | orography co=1+2*s*Phi, s=1 Phi=.15 ; qp cat II z=10 | 1.3; 1.34891 |
| E14-07 | walls e=min(b,2h), A=e/5, B=d-e/5 (e>=d) | 24; 4.8; 15.2 |
| E14-08 | h/d=.6 cpeD,cpeE ; qp(12) cat II ; we_D, we_E | 0.746667; -0.393333; 0.964573; 0.720215; -0.379399 |
| E14-09 | flat roof e=24: F=e/4*e/10, G=(b-2*e/4)*e/10, H=(e/2-e/10)*b, I rest | 14.4; 43.2; 288; 240 |
| E14-10 | size effect F A=3 m2: cpe | -2.16602; -2.5; -1.8 |
| E14-11 | cpi dominant: .75*cpe, .90*cpe cpe=-1.2 | -0.9; -1.08 |
| E14-12 | Annex B h=40 b=30 cat III vb 25 delta .1 : zs,Li,B2,n1,SL,R2,nu,kp,cscd | 24; 82.3388; 0.551724; 1.15; 0.0539174; 1.5737; 0.989546; 3.74179; 1.34354 |
| E14-13 | cprob p=0.2 (5 yr),p=.1 (10 yr), p=.5 (2 yr) | 0.854501; 0.90248; 0.776433 |
| E14-14 | vortex vcrit=b n/St, St=.18 ; Sc=2*ds*me/(rho b^2) ds=.05 me=300 kg/m | 4.16667; 96 |
| E14-15 | Bridge FW=.5 rho vb^2 C Aref C=ce*cfx=2.0*1.3? vb=25 Aref=3*40 : kN | 121.875 |
| E14-16 | Re=b v/nu b=.5 v(ze) from qp(12) | 1.3095e+06 |

### 7.6 EN 1991-1-5 (8 examples)
Hand-check notes: E15-01 (T_min = −24, T_max = 37, T_0 = 10): steel Type 1 T_e,min = −27, T_e,max = 53 ⇒ ΔT_con = 37, ΔT_exp = 43, ΔL_exp = 12e-6·43·100 m = 51.6 mm; concrete Type 3: −16/39 ⇒ 26/29 K ⇒ 26/29 mm. E15-03: 210000·12e-6·40 = 100.8 MPa; 33000·10e-6·30 = 9.9 MPa. E15-04: 18 + 0.35·43 = 33.05; 43 + 0.75·18 = 56.5. E15-05: 210000·1e9·12e-6·18/1000 = 4.536e7 Nmm = 45.36 kNm.

| ID | Case | Result |
|---|---|---|
| E15-01 | Type1 steel: Te,min Te,max dTcon dTexp dL100(mm) con,exp | -27; 53; 37; 43; 44.4; 51.6 |
| E15-01 | Type2 comp: Te,min Te,max dTcon dTexp dL100(mm) con,exp | -20; 41; 30; 31; 36; 37.2 |
| E15-01 | Type3 conc: Te,min Te,max dTcon dTexp dL100(mm) con,exp | -16; 39; 26; 29; 26; 29 |
| E15-02 | linear heating 18 K ksur .7 ; cooling 13 ksur 1.1(waterproof) | 12.6; 14.3 |
| E15-03 | full restraint sigma=E a dT steel 210000*12e-6*40 ; conc 33000*10e-6*30 | 100.8; 9.9 |
| E15-04 | combination dTM + .35 dTN ; dTN + .75 dTM  (dTM=18,dTN=43) | 33.05; 56.5 |
| E15-05 | restrained bowing M=E I a dTM/h  E=210000 MPa I=1e9 mm4 h=1000 mm dT=18 kNm | 45.36 |
| E15-06 | joint movement 100 m steel dTN exp 43 ,psi:  | 51.6 |
| E15-07 | altitude correction Tmin 0.5K/100m @ 800m [V] | -28 |
| E15-08 | K to C, T0 defaults | 283.15 |

### 7.7 EN 1991-1-6 (8 examples)
Hand-check notes: E16-01: 1.0·9 = 9 kN. E16-02: 25·0.6 = 15 kN/m^2. E16-03: c_prob(5 yr) = 0.8545 ⇒ v_b = 21.36 m/s ⇒ q_b = 0.2852 kN/m^2. E16-08: 2-yr snow factor (V = 0.4, P = 0.5): ln(−ln 0.5) = −0.3665, (1 − 0.4·(0.45 − 0.2859))/2.036 = 0.9344/2.036 = 0.4589.

| ID | Case | Result |
|---|---|---|
| E16-01 | Qca 1.0 kN/m2 on 3x3 | 9 |
| E16-02 | fresh concrete hydrostatic 25*0.6 ; + Qca .75 | 15; 15.75 |
| E16-03 | cprob 3 months ~ 5 yr; wind vb scaling | 21.3625; 0.285223 |
| E16-04 | Qcb storage 0.2 kN/m2 *area 50 | 10 |
| E16-05 | stack of 4 pallets 1.2*... bricks 2.0 kN/m2*10 m2 | 20 |
| E16-06 | combination Qc,sum = 1.0*(Qca)+1.0*(Qcb) | 1.2 |
| E16-07 | Qcf temporary partial structure self weight 150 kN * psi 1 | 150 |
| E16-08 | 2yr snow factor via Annex D V=.4 P=.5 | 0.458916 |

### 7.8 EN 1991-1-7 (8 examples)
Hand-check notes: E17-02: s = 6.0, L = 7.2, g = 5, q = 3, ψ2 = 0.3: 0.4·5.9·43.2 = 101.95; 0.8·5.9·43.2 = 203.9; both exceed 75 ⇒ min(...,75) = 75 under the 'lesser' reading [V]. E17-03: min(0.15·400, 100) = 60 m^2. E17-04: √(3e5·1500) = 21213 N, ·15 = 318.2 kN. E17-05: D.1: 3 + 5 = 8; D.2: 3 + 2.5 + 0.04/0.0016 = 30.5 kN/m^2 [V]. E17-06: 34·18 = 612 kN.

| ID | Case | Result |
|---|---|---|
| E17-01 | Impact urban 750/375 cars 50/25 | 750; 375; 50; 25 |
| E17-02 | ties peripheral .4(g+.3q)sL ; internal .8(...)sL ; 75 cap least | 101.952; 203.904; 75; 75 |
| E17-03 | notional removal area min(15% of 400, 100) | 60 |
| E17-04 | dynamic impact F=v sqrt(k m) v=15 m/s k=3e5 N/m m=1500 kg kN | 318.198 |
| E17-05 | explosion pd D1 3+pv ; D2 3+pv/2+.04/(Av/V)^2 | 8; 30.5 |
| E17-06 | key element Ad=34 kN/m2 * 3 m *6 m | 612 |
| E17-07 | impact on deck 500/250; pier 1000/500 [V] | 500; 250 |
| E17-08 | impact energy .5 m v^2 : 1500 kg 15 m/s kJ | 168.75 |

### 7.9 EN 1991-2 (12 examples)
Hand-check notes: E2-01: w = 5.0 ⇒ (1, 3.0, 2.0); 5.5 ⇒ (2, 2.75, 0); 6.0 ⇒ (2, 3.0, 0); 10.5 ⇒ (3, 3.0, 1.5). E2-02: tandems 2·600 = 1200 kN; UDL 9·3 = 27, 2.5·3 = 7.5 kN/m, remaining 1.5 m ⇒ 3.75 kN/m. E2-03: 360 + 0.1·9·3·40 = 468 kN ∈ [180, 900]. E2-04: Q_v = 1200: 40·1200/300 = 160 kN; r = 150 m: 0.2·1200 = 240 kN. E2-05: 2 + 120/80 = 3.5 kN/m^2; L = 5 ⇒ 5.0 (cap); L = 300 ⇒ 2.5 (floor). E2-06: √20 = 4.4721, 1.44/4.2721 = 0.3371 ⇒ Φ_2 = 1.1571; 2.16/4.2721 + 0.73 = 1.2356. E2-07: f = 1 − 0.04·(5.0875 + 1.75)·(1 − 0.3098) = 0.8112; Q_tk = 160²/(127·1000)·0.8112·1000 = 163.5 kN. E2-08: 33·30 = 990 kN (≤ 1000), 20·30 = 600.

| ID | Case | Result |
|---|---|---|
| E2-01 | lanes w=5.0,5.5,6.0,10.5 | (1, 3, 2); (2, 2.75, 0); (2, 3, 0); (3, 3, 1.5) |
| E2-02 | LM1 3 lanes: tandems 2*(300+200+100); UDL kN/m for 3 m lanes | 1200; 27; 7.5; 7.5; rest 1.5 m: 2.5*1.5=; 3.75 |
| E2-03 | braking Qlk=.6*2*300+.1*9*3*L  [180,900] | 468 |
| E2-04 | centrifugal 40Qv/r r=300 ; 0.2Qv r<200 r=150 | 160; 240 |
| E2-05 | footbridge qfk=2+120/(L+30) L=50 ; clamps L=5,300 | 3.5; 5; 2.5 |
| E2-06 | LM71 phi2,phi3 L_phi=20 | 1.15707; 1.2356 |
| E2-07 | rail centrifugal v=160 r=1000 Lf=30: f,Qtk=v^2/(127r)*f*Qvk | 0.811241; 163.526 |
| E2-08 | traction 33*30 ; braking 20*30 (LM71) | 990; 600 |
| E2-09 | SW/0 total 2 spans 15 m: 133*15 ; LM71 UDL 80 kN/m | 1995; 80 |
| E2-10 | road gr1a 1.35*(TS 2*(300)... ) crowd 5 kN/m2 *3 m wide | 15 |
| E2-11 | LM2 400 kN axle wheel 0.35x0.6: pressure kN/m2 per wheel 200 kN | 952.381 |
| E2-12 | LM1 wheel 150 kN on .4x.4 pressure | 937.5 |

### 7.10 EN 1991-3 (9 examples)
Hand-check notes: E3-01: 1.05 + 0.17·0.25 = 1.0925; 1.10 + 0.34·0.25 = 1.185; 1.15 + 0.51·0.25 = 1.2775; 1.20 + 0.68·0.25 = 1.37. E3-02: 1 − 0.5·1.5 = 0.25; 1 − 0.5·2 = 0. E3-05: K = 0.2·400 = 80; H_L = 1.5·80/2 = 60 kN. E3-06: 0.3·(1 − e^−3.75) = 0.29294. E3-07: 1.25·0.7·√(2e4·2e6) = 1.25·0.7·2e5 N = 175 kN. E3-08, E3-09 are schematic and shown only to fix arithmetic; drive factors [V].

| ID | Case | Result |
|---|---|---|
| E3-01 | phi2 HC1..4 vh=.25: min+b2*vh | 1.0925; 1.185; 1.2775; 1.37 |
| E3-02 | phi3 = 1-dm/m(1+b3) dm/m=.5 b3=.5(grab)=?, b3=1.0  | 0.25; 0 |
| E3-03 | hoist load Q_H=100 kN * phi2(HC2) | 118.5 |
| E3-04 | self weight phi1=1.1 *300 kN ; 0.9*300 | 330; 270 |
| E3-05 | drive K=mu*SumQr 0.2*400 ; HL=phi5*K/nr=1.5*80/2 | 80; 60 |
| E3-06 | skew f=.3(1-exp(-250a)) a=.015 | 0.292945 |
| E3-07 | buffer HB=phi7*v1*sqrt(mc*SB) phi7=1.25 v1=.7 mc=20000kg SB=2e6 N/m kN | 175 |
| E3-08 | transverse HT=xi*M/a | 25 |
| E3-09 | wheel Qr,max group1 300*1.1(phi1)+ (Q_H share)  | 224.25 |

### 7.11 EN 1991-4 (12 values, 8 examples)
Hand-check notes: E4-01: K = 1.1·0.5 = 0.55, A/U = 6/4 = 1.5 m, z_o = 1.5/(0.55·0.4) = 6.818 m, p_ho = 9·0.55·6.818 = 33.75 kPa; z = 10: Y_J = 1 − e^−1.4667 = 0.7693, p_hf = 25.96, p_wf = 10.39, p_vf = 47.21 kPa. E4-08 equilibrium check: μ·p_ho·(z − z_o(1 − e^−z/z_o)) = 13.5·4.755 = 64.19 kN/m = γ(A/U) z − (A/U) p_vf = 135 − 70.81 = 64.19 (exact equality, used as property test). E4-04: π/4·36·12·9/9.81 = 311.3 t ⇒ AAC2.

| ID | Case | Result |
|---|---|---|
| E4-01 | Janssen z=2: K,z0,pho,Yj,phf,pwf,pvf | 0.55; 6.81818; 33.75; 0.254226; 8.58014; 3.43206; 15.6003 |
| E4-01 | Janssen z=10: K,z0,pho,Yj,phf,pwf,pvf | 0.55; 6.81818; 33.75; 0.769307; 25.9641; 10.3856; 47.2075 |
| E4-01 | Janssen z=20: K,z0,pho,Yj,phf,pwf,pvf | 0.55; 6.81818; 33.75; 0.946781; 31.9538; 12.7815; 58.0979 |
| E4-02 | slenderness hc/dc 12/6=2 -> slender (>=2); 8/6 intermediate; 4/6 squat; <.4 retaining | 2; 1.33333; 0.666667 |
| E4-03 | Rankine active K=(1-sin)/(1+sin) | 0.333333 |
| E4-04 | mass held pi/4 d^2 h gamma/g ->t : d=6 h=12 gamma=9 | 311.277 |
| E4-05 | AAC by capacity t : 100 t, 10000 t thresholds | 100; 10000 |
| E4-06 | wall friction angle phiw=.75*phi ; mu=tan | 0.414214 |
| E4-07 | hydrostatic tank gamma 10 z 5 | 50 |
| E4-08 | wall friction per m perimeter at z=10: mu*pho*(z-z0(1-e^(-z/z0))) == gamma*A/U*z - A/U*pvf | 64.1888; 64.1888 |

## 8. NA-DE NDP register (consolidated, with confidence)

| Symbol | Clause | EN recommended | NA-DE | Tag |
|---|---|---|---|---|
| γ_G,sup / γ_G,inf (STR) | 1990 A1.2(B) | 1.35 / 1.00 | 1.35 / 1.00 | N |
| γ_Q | 1990 A1.2(B) | 1.50 | 1.50 | N |
| γ_G,sup / γ_G,inf (EQU) | 1990 A1.2(A) | 1.10 / 0.90 | 1.10 / 0.90 | N |
| (6.10a)/(6.10b) ξ | 1990 6.4.3.2 | ξ = 0.85 | not offered | V |
| ψ snow | 1990 A1.1 | H≤1000: 0.5/0.2/0 | same + >1000 m: 0.7/0.5/0.2 | V |
| ψ0 temperature (bridges) | 1990 A2.1 | 0.6 | 0.8 | V |
| K_FI | 1990 B | 0.9/1.0/1.1 | same | V |
| β_target | 1990 B | 4.2/4.7/5.2 (1 yr) | same | V |
| α_A area reduction | 1991-1-1 6.3.1.2 | allowed | not allowed | V |
| q_k, Q_k categories | 1991-1-1 6.3.1 | ranges | Table 6.1DE fixed | V |
| Δq_k partitions | 1991-1-1 6.3.1.2(8) | 0.5/0.8/1.2 | 0.8/1.2/1.6 | V |
| ψ_fi | 1991-1-2 4.3.1 | ψ1,1 or ψ2,1 | ψ1,1 | V |
| s_k | 1991-1-3 4.1 | CEN zones, Annex C | Zones 1, 1a, 2, 2a, 3 formulas | V (formulas), data asset zone map |
| C_e, C_t | 1991-1-3 5.2 | 0.8/1.0/1.2, 1.0 | 1.0, 1.0 | V |
| s_Ad | 1991-1-3 4.3 | C_esl | 2.3 s_k (North German lowland) | V |
| μ2 cap, l_s | 1991-1-3 5.3.6 | μw 0.8..4, l_s 5..15 | μ2 0.8..2.0 | V |
| v_b,0 | 1991-1-4 4.2 | national map | WZ1..4: 22.5/25/27.5/30 | N |
| terrain / profiles | 1991-1-4 4.3 | cat 0–IV | Binnenland / Kueste / Inseln profiles, Mischprofil, simplified Table | V |
| c_dir, c_season | 1991-1-4 4.2 | 1.0 | 1.0 | V |
| c_s c_d method | 1991-1-4 6 | Annex B/C | NA.C, sensitivity limits | V |
| T_min / T_max | 1991-1-5 A | isotherm maps | −24 / +37 °C base, altitude corr. | V |
| F_d,x impact | 1991-1-7 4.3 | 1000/750/50/150 | Table NA.4.1 | V |
| LM1 α_Q, α_q, Q_ik, q_ik | 1991-2 4.3.2 | 300/200/100, 9/2.5 | Table NA.4.? | V |
| φ_i, μ crane | 1991-3 2.5 | HC1–HC4 | same + NA tables | V |
| C_h, C_w, K, patch | 1991-4 5 | EN | NA-DE | V |

The register is implemented as `NdpSet` data (section 3.1). The editor shows the three-state NDP badge: verified, unverified, overridden. No value tagged `V` may be used to produce a "compliant" stamp until a human sign-off event `NdpVerified{ndp_id, by, source_page}` exists.

## 9. Open verification items (to be done before enabling compliance stamps)
1. Key all tables of EN 1991-1-4 7.2 (7.3a-7.5, 7.6-7.13) and Annex E/F; compare the double-key.
2. Confirm NA-DE wind profile coefficients (coast/island 1.8, 2.3, 2.6 and exponents), simplified q_p table, and the wind zone assignment source.
3. Confirm NA-DE snow zone assignment data (municipality or PLZ) and the zone-formula constants; confirm s_Ad = 2.3 s_k applicability list.
4. Confirm NA-DE traffic: LM1 α factors, LM3 special vehicles, tandem/UDL distribution and combination factors for temperature.
5. Confirm tie-force reading ("lesser of") and CC table in EN 1991-1-7 A.5 and NA-DE.
6. Confirm Table 6.1DE imposed loads and partition addition Δq_k.
7. Confirm crane dynamic factors φ_4..φ_7, λ_s, load groups, and silo load multipliers C_h, C_w, patch load.
8. Confirm thermal-bridge table values (T_e,min/T_e,max offsets, k_sur, ΔT_M) and NA-DE temperature maps.

## 10. Delivery and acceptance (for the fleet)
- Deliver in vertical slices, each with: schema leaf + data assets + inference + test case JSON + oracle + 3D view + editor interaction. Suggested order: (1) EN 1990 combination engine with ψ/γ tables and matrix view; (2) EN 1991-1-3 snow with roof 3D; (3) EN 1991-1-4 wind zones on 3D envelope; (4) EN 1991-1-1 floors; (5) EN 1991-2 lanes/LM1 and rail; (6) EN 1991-1-2 fire; (7) 1-5, 1-6, 1-7; (8) 1991-3 and 1991-4; (9) Annex B/C/D tools.
- A slice is accepted when: every catalogue entry it covers has a test case; the example IDs in section 7 pass; values tagged V are either verified or displayed as unverified; German and English labels exist; keyboard and screen-reader paths exist for the 3D elements; edit-to-view latency meets the budgets of section 5.3; cancellation works.
- Out of scope here: FEM analysis of the structure (`fem`), material verifications (EN 1992-1999), seismic action derivation (EN 1998 provides A_Ek), CFD/zone fire models.
