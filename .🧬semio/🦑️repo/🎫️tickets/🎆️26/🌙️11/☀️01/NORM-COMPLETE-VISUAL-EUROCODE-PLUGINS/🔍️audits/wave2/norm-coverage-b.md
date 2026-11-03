# 🧭 Norm Coverage B: EN 1995 to EN 1999 (wave 2, read-only)

Ticket: `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`. Auditor: NORM-COVERAGE-B. No sources edited, no git writes, no `repo` commands.

## Method and sources

- Spec text (ID-level): `📐️specs/en1995-en1996-target.md` (EN 1995, EN 1996), `📐️specs/en1997-en1998-target.md` (EN 1997, EN 1998), `📐️specs/en1994-en1999-target.md` (EN 1999 `A-xx`; `C-xx` is EN 1994 and excluded).
- Trace: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji>en199X/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json`, one item per `specIds` entry. Kinds: `check`, `design-case`, `data`, `not-modelled`.
- Spec IDs were collected from headings, line-start bullets, bold definitions and table first cells, then grouped by part. IDs mentioned only in prose were not counted, except `T-98` and `M-95`, which are defined in spec text (bullet / heading) and handled explicitly below.
- Wave-1 baseline: `🔍️audits/en199X-completion.md`. Wave-2 signals: `🔍️audits/wave2/` (`feature-rows-v3`, `en1995-scene-unify-v2`, `en1996-v11`, `en1997-v11`, `en1998-v15`, `en1999-v12`).
- Scratch scripts (kept in the ticket generated folder): `🗑️generated/norm-b-ids.py`, `norm-b-sections.py`, `norm-b-trace.py`, `norm-b-inspect.py`.
- Limit: "referenced in code" below means a key string grep hit in the artifact tree. It is not a compile or test result. Nothing was compiled, and the wave-2 reports say the same.

Artifact path prefix used below: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/` (abbreviated as `…/🗿️artifacts/`).

## 1. Summary table per code (ID level)

Classification legend: **check** = trace kind `check`; **design-case** = trace kind `design-case` (pointer to a design case); **data** = trace kind `data` (table/rule, no check); **not-modelled** = trace kind `not-modelled` with reason; **MISSING** = defined in spec text, no trace row.

| Code | Spec IDs | check | design-case | data | not-modelled | MISSING | Trace-only | Coverage |
|---|---|---|---|---|---|---|---|---|
| EN 1995 (all parts) | 68 | 34 | 34 | 0 | 0 | 0 | 0 | 100 % |
| EN 1996 (parts 1-1, 1-2, 2, 3) | 38 | 37 | 0 | 0 | 0 | 1 | 0 | 97 % |
| EN 1997 (parts 1 and 2) | 58 | 44 | 0 | 0 | 0 | 16 | 2 | 72 % (42 of 58) |
| EN 1998 (parts 1 to 6) | 68 | 29 | 0 | 4 | 0 | 35 | 0 | 49 % (33 of 68) |
| EN 1999 (parts 1-1 to 1-5) | 42 | 40 | 0 | 1 | 1 | 0 | 0 | 100 % |
| **Total** | **274** | **184** | **34** | **5** | **1** | **52** | **2** | **81 % (222 of 274)** |

Trace rows total 224 = 222 spec-covered + 2 trace-only (EN 1997 `HYD-02`, `HYD-03`).

Note on EN 1998 totals: EN 1998-1 `SEI-*` is 26 of 26 covered. The `ED-10..ED-18` IDs (section 6, "Realtime editor interactions") are UI interactions, not norm clauses. They are excluded from the 68 and from MISSING.

### 1a. Per part

| Code | Part | Spec IDs | check | design-case | data | not-modelled | MISSING | Notes |
|---|---|---|---|---|---|---|---|---|
| EN 1995 | 1-1 (ch. 2, 3, 5-9) | 57 | 30 | 27 | 0 | 0 | 0 | includes `T-98` (spec 2.9 bullet, design-case) |
| EN 1995 | 1-2 (fire) `T-100..105` | 6 | 1 | 5 | 0 | 0 | 0 | |
| EN 1995 | 2 (bridges) `T-120..124` | 5 | 3 | 2 | 0 | 0 | 0 | |
| EN 1996 | 1-1 (`M-01..61`) | 30 | 30 | 0 | 0 | 0 | 0 | `M-43` spec says out_of_scope, trace says check (conflict) |
| EN 1996 | 1-2 (fire) `M-90, M-91` | 2 | 2 | 0 | 0 | 0 | 0 | |
| EN 1996 | 2 `M-70` | 1 | 1 | 0 | 0 | 0 | 0 | |
| EN 1996 | 3 (simplified) `M-80..83` | 4 | 4 | 0 | 0 | 0 | 0 | |
| EN 1996 | cross-ref to EN 1998-1 ch. 9 `M-95` | 1 | 0 | 0 | 0 | 0 | 1 | see MISSING #27 |
| EN 1997 | 1 (`G-`, `GEO-F`, `EQU`, `UPL`, `HYD-01`, `SLS`, `PILE`, `RW`, `SLOPE`, `ANC`, `DEW`) | 42 | 42 | 0 | 0 | 0 | 0 | `DEW-02` not defined in spec |
| EN 1997 | 1 trace-only | 0 | 2 | 0 | 0 | 0 | 0 | `HYD-02`, `HYD-03` (clause 10.4): no spec-text definition; wave-2 `en1997-v11` added them to the trace |
| EN 1997 | 2 (`GI-01..16`) | 16 | 0 | 0 | 0 | 0 | 16 | no trace rows; `G-02` check key `en1997.gi-17.` points at a GI-17 that the spec does not define |
| EN 1998 | 1 (`SEI-*`) | 26 | 22 | 0 | 4 | 0 | 0 | `data`: SEI-03, SEI-04, SEI-05, SEI-42 |
| EN 1998 | 2 bridges (`BR-*`) | 12 | 3 | 0 | 0 | 0 | 9 | covered: BR-04, BR-07, BR-09 |
| EN 1998 | 3 assessment (`AS-*`) | 10 | 0 | 0 | 0 | 0 | 10 | none |
| EN 1998 | 4 silos/tanks (`ST-*`) | 3 | 2 | 0 | 0 | 0 | 1 | covered: ST-01, ST-02 |
| EN 1998 | 5 foundations/geotech (`GE-*`) | 12 | 0 | 0 | 0 | 0 | 12 | none |
| EN 1998 | 6 towers/masts (`TW-*`) | 5 | 2 | 0 | 0 | 0 | 3 | covered: TW-03, TW-04 |
| EN 1999 | 1-1 (`A-01..33, A-80, A-81`) | 31 | 29 | 0 | 1 | 0 | 0 | `data` A-01 (alloy table), `not-modelled` A-81 |
| EN 1999 | 1-2 fire `A-50..52` | 3 | 3 | 0 | 0 | 0 | 0 | |
| EN 1999 | 1-3 fatigue `A-40, A-41` | 2 | 2 | 0 | 0 | 0 | 0 | |
| EN 1999 | 1-4 cold-formed `A-60..62` | 3 | 3 | 0 | 0 | 0 | 0 | |
| EN 1999 | 1-5 shells `A-70..72` | 3 | 3 | 0 | 0 | 0 | 0 | |

## 2. Design-case and data items (classification detail)

- EN 1995 design-case (34): T-10, T-11, T-13, T-16, T-17, T-21, T-23, T-24, T-45, T-50, T-62, T-66, T-70, T-75, T-76, T-77, T-78, T-79, T-80, T-81, T-82, T-83, T-90, T-91, T-92, T-93, T-98, T-100, T-102, T-103, T-104, T-105, T-123, T-124. Design-case rows are pointers only (`entry: T-nn`). Each one needs a design-case owner to be verified; this audit did not check that.
- EN 1998 data (4): SEI-03 (elastic spectrum), SEI-04 (design spectrum), SEI-05 (vertical spectrum), SEI-42 (modal response spectrum analysis, SRSS/CQC). SEI-42 is a method, not a table. It should probably be a check.
- EN 1999 data (1): A-01 (alloy/temper table, `part_1::ALLOY_BANDS`).
- EN 1999 not-modelled (1): A-81 thermal expansion and creep, reason recorded in trace (creep negligible below 100 °C). Acceptable.

## 3. MISSING list, ranked by engineering importance

Each entry: clause ID, one-line description (from the spec row), suggested owner artifact. Ranking reflects life-safety and input-basis impact first, then scope breadth. Rank 1 to 27 are the substantive items. The rest are optional or informational.

Tier A, life-safety verifications with no trace row (seismic geotechnics and bridges):

1. **GI-01**: EN 1997-2 investigation extent (min depth z_a >= max(6 m, 2B) for footings; Annex B.3). Owner `…/🗿️artifacts/🌍️en1997`.
2. **GI-03**: CPT undrained strength cu = (qt − σv0)/Nkt, Nkt 15 to 20. Input basis for GEO-F02 and SLOPE. Owner `…/🌍️en1997`.
3. **GE-03**: Liquefaction resistance, simplified Seed-Idriss CSR/CRR with (N1)60, magnitude and fines (EN 1998-5 4.1.4(11), Annex B). Owner `…/🫨️en1998`.
4. **GE-02**: Liquefaction susceptibility screen, reject if ag·S < 0.1 g (and groundwater ≤ 15 m) (4.1.4, Annex B). Owner `…/🫨️en1998`.
5. **GE-06**: Seismic bearing capacity of shallow foundations with N, V, M eccentricity reduction (Annex F). Owner `…/🫨️en1998`.
6. **GE-07**: Seismic sliding of foundations, F_h <= (V − u)·tanδ / γRd (5.3). Owner `…/🫨️en1998`.
7. **GE-10**: Retaining walls, Mononobe-Okabe seismic earth pressure, kh = α·S/r (7.3.2, Annex E). Overlaps the 1997 check `RW-08`, so the owner must decide one source. Owner `…/🫨️en1998`.
8. **BR-08**: Shear design in plastic-hinge region, V_Ed,CD from over-strength moment (5.x). Owner `…/🫨️en1998`.
9. **BR-06**: Combination of seismic components E_x + 0.3E_y + 0.3E_z (4.1.8). Owner `…/🫨️en1998`.
10. **BR-03**: Ductile behaviour factor q for RC piers, q = 3.5·λ(αs) with shear span αs (4.1). Owner `…/🫨️en1998`.
11. **BR-01**: Design seismic action for bridges, ag = γI·agR with importance factors (Class I 0.85, II 1.0, III 1.3). Owner `…/🫨️en1998`.
12. **BR-02**: Bridge elastic spectrum incl. vertical ratio avg/ag = 0.7 (3.2). Owner `…/🫨️en1998`.
13. **AS-07**: Assessment verification, ductile D/C <= 1, brittle E_d <= R_d (4.4). Owner `…/🫨️en1998`.
14. **AS-05**: Ultimate chord rotation θum with confinement and detailing (A.3.2). Owner `…/🫨️en1998`.
15. **AS-06**: Shear capacity degrading with ductility, V_R under cyclic loading (A.x). Owner `…/🫨️en1998`.
16. **AS-04**: Chord rotation at yield θy from member geometry and material (Annex A.3.2). Owner `…/🫨️en1998`.
17. **AS-03**: Material strengths from tests, mean/CF for ductile and f_m/(CF·γm) for brittle (4.3). Owner `…/🫨️en1998`.
18. **AS-01**: Knowledge levels KL1/KL2/KL3 and confidence factors CF (2.2, 3.3). Owner `…/🫨️en1998`.
19. **AS-02**: Limit states NC/SD/DL with return periods 2475/475/225 years (2.1). Owner `…/🫨️en1998`.
20. **AS-08**: Analysis methods for assessment (LF, MRS, pushover/N2, NL time history) (4.3.3). Owner `…/🫨️en1998`.
21. **GE-05**: Pseudo-static seismic slope stability, F_H = 0.5·α·S·W, F_V = ±0.33·F_H (5.x). Owner `…/🫨️en1998`.
22. **GE-08**: Foundation tie beams: relative displacement limit and tie force N = ±0.3 α S N_Ed for soil B (5.4.2). Owner `…/🫨️en1998`.
23. **GE-01**: Ground investigation, vs30 from boreholes and shear-wave profile, Gmax = ρ·vs², strain-dependent G reduction (3.1). Feeds SEI-01. Owner `…/🫨️en1998`.
24. **GE-12**: Piles in seismic ground, kinematic moments and loss of lateral support in liquefied layers (5.4). Owner `…/🫨️en1998`.
25. **GE-09**: Foundation stiffness, soil-structure interaction springs (Gazetas circular foundation) (5.2). Owner `…/🫨️en1998`.
26. **GE-04**: Pore pressure increase and lateral-spreading flag (4.1.4, Annex B). Owner `…/🫨️en1998`.

Tier B, gaps that matter for specific structure classes:

27. **M-95**: Seismic masonry (EN 1998-1 ch. 9), q factors for unreinforced, confined and reinforced masonry, Φ-based in-plane checks. Defined in EN 1996 spec 7.10 as a stub but absent from the EN 1996 trace. EN 1998 `SEI-52` covers only "masonry simple". Owner `…/🪨️en1996` (stub) with `…/🫨️en1998` (SEI-52 extension).
28. **BR-10**: Seismic isolation of bridges, T_eff, ξ_eff, η and d_cd (6). Owner `…/🫨️en1998`.
29. **BR-11**: Foundation and abutment soil-structure interaction springs (5.x). Owner `…/🫨️en1998`.
30. **BR-05**: Cracked stiffness for design displacement, I_eff = 0.08 to I_g (4.1.6). Owner `…/🫨️en1998`.
31. **BR-12**: Spatial variability of seismic action for long bridges (2.3.6). Owner `…/🫨️en1998`.
32. **AS-09**: Retrofit strengthening schemes (jackets, FRP) and interface checks (5/6). Owner `…/🫨️en1998`.
33. **AS-10**: NA-DE restrictions for DIN EN 1998-3/NA (fixed T_R = 475, CF per NA). Owner `…/🫨️en1998`.
34. **GI-02**: CPT soil classification (qt, Rf, Ic, Annex E). Owner `…/🌍️en1997`.
35. **GI-04**: CPT friction angle φ' (Kulhawy-Mayne, Annex D). Owner `…/🌍️en1997`.
36. **GI-05**: CPT oedometer modulus E_oed (Annex D). Owner `…/🌍️en1997`.
37. **GI-06**: SPT energy/correction (Annex F). Owner `…/🌍️en1997`.
38. **GI-07**: SPT φ' (Hatanaka-Uchida, Annex F). Owner `…/🌍️en1997`.
39. **GI-08**: SPT relative density D_r (Annex F). Owner `…/🌍️en1997`.
40. **GI-09**: DPH/DPL/DPSH dynamic probing (Annex G/H). Owner `…/🌍️en1997`.
41. **GI-10**: Laboratory oedometer test series (Annex). Owner `…/🌍️en1997`.
42. **GI-11**: Laboratory triaxial CU/CD test series. Owner `…/🌍️en1997`.
43. **GI-12**: Laboratory direct shear τ_f(σ). Owner `…/🌍️en1997`.
44. **GI-13**: Characteristic value derivation from test series (2.4). Owner `…/🌍️en1997`.
45. **GI-14**: Pressuremeter, Ménard modulus. Owner `…/🌍️en1997`.
46. **GI-15**: Grain-size classification. Owner `…/🌍️en1997`.
47. **GI-16**: Permeability from pump test, Dupuit/Thiem (2.14). Owner `…/🌍️en1997`.
48. **GE-11**: Hydrodynamic water pressure on retaining walls, Westergaard Δq = 7/12 · kh · γw · H² (5.x, 7.x). Owner `…/🫨️en1998`.
49. **TW-01**: Damping ξ per structure type and η correction (4.x). Owner `…/🫨️en1998`.
50. **TW-02**: Fundamental period T1 from Rayleigh or chimney approximation. Owner `…/🫨️en1998`.
51. **ST-03**: Buried pipelines, strain ε = a_g/(2c) and joint displacement, optional. Owner `…/🫨️en1998`.
52. **TW-05**: Guyed masts, cable prestress and displacement, optional. Owner `…/🫨️en1998`.

Note: TW-03 and TW-04 are covered by the trace and are not MISSING. The MISSING count is 52 in total: EN 1997-2 GI 16, EN 1998 35 (BR 9, AS 10, ST 1, GE 12, TW 3), EN 1996 M-95 1.

## 4. Trace and spec drift (not MISSING, but must be resolved)

- `HYD-02`, `HYD-03` (EN 1997, clause 10.4): in the trace, not in spec text. Either add the rows to the target spec or drop them from the trace.
- `M-43` (EN 1996, confined and prestressed masonry): spec says "not in scope for phase 1; registry stubs out_of_scope". Trace says `check`. Should be `not-modelled` with that reason.
- EN 1999 `A-33` (adhesives and pin joints): wave-1 says "not modelled". Trace says `check` (`en1999.8.5.pin`), and that key is referenced in 6 code files. Needs a decision on which is true.
- EN 1999 `A-16` (torsion), `A-19` / `A-27` (local loads), `A-28` (stiffeners), `A-06` (stiffened elements): wave-1 listed these as schema-blocked. Trace marks them `check`, and their keys are referenced in code (3 to 5 files each). Wave-2 is the likely cause. Still needs a verification pass.
- EN 1997 `G-02` key `en1997.gi-17.` names GI-17, which the spec does not define.
- Trailing-dot check keys (e.g. `en1995.t-30.`, `en1996.m-01.`) occur in many rows. They look like a naming convention and are not treated as stubs here. Not verified.

## 5. Wave-1 completion gaps: closed vs still open

Status is as of wave 2 (grep and report evidence only, nothing compiled).

### EN 1995
- Closed: 22 hand-rolled cases moved onto shared forms (`en1995-scene-unify-v2`: premise stale, the cases were already on forms; incremental equals full on 40 random edits). Feature rows 38 of 38 with pass and fail (`feature-rows-v3`).
- Still open (from wave-1, not addressed in wave 2): snapshot entities and mutation leaves for catalogue cases; `mCritNm` removal (216 fixture and twin files); selectable ULS 6.10a/b with ξ = 0.85; Gherkin step adapters; NDP registration in `⚖️compliance/📜️annex-and-tables`; visual fields (tapered/curved geometry, CLT layers, split rings, trusses, diaphragms, footbridge decks).

### EN 1996
- Closed: EN 1996-1-1 seismic γM mismatch. `AnnexParams::for_document` now exhaustive over design situations (`en1996-v11`). Area check 6.1.3.area reworked (`en1996-v10`, `v11`).
- Still open: "not compiled or tested" is the blocking gap (`en1996-v11`: nothing compiled). Snapshot entities for confined masonry, prestressing, ties, anchors, flanged shear walls, bracing layout, piers and lintels, shear reinforcement, deep beams. Annex E tables for fixed-edge panels other than S-S-S-S, S-S-S-F and two free edges. Concentrated-load a1 = 0 assumption (no distance-to-end field). Open disagreement: Rust reduces γM for Transient in DE, the oracle does not.
- New: `M-95` absent from the trace (MISSING #27). `M-43` trace/spec conflict.

### EN 1997
- Closed: trace rows for `HYD-02`, `HYD-03` added (`en1997-v11`, but see drift above). Feature families 30 of 30 with pass and fail rows (`feature-rows-v3`). Earth-pressure M2 checks (`RW-03`, `RW-06`, `RW-07`, `RW-12`) reviewed with no defect (`en1997-v11`).
- Still open: editor edit coverage (55 snapshot leaves without a typed kind, about 22 files per kind). Snapshot lacks anchors, embedded walls, pile groups, lab tests and per-layer water entities. `.feature` step adapter. Editor does not list the catalogue. NDP values not in the shared registry.
- New: EN 1997-2 (`GI-01..16`) has no trace rows at all. Wave-1 said "lab series beyond GI-10..12 not done". The `G-02` key implies GI wiring exists in code, but the spec-trace does not show it.

### EN 1998
- Closed: stale fixtures regenerated losslessly (`en1998-v15`: 624 files with problems before, 0 after, schema validation). All 26 EN 1998-1 `SEI-*` IDs now have trace rows (4 are `data`).
- Still open: catalogue entries needing new snapshot inputs (v_s30 classification, regularity from geometry, seismic joints, non-structural elements, capacity design, timber and masonry member checks, EN 1998-2 piers, and the bridge, assessment, silo, tank and tower check families, which the trace covers only partly). Compile and test not run. DIN EN 1998-1/NA S_aP,R path not started. Parts 2 to 6 almost entirely absent from the trace (see MISSING #8 to #52).

### EN 1999
- Closed: trace covers all 42 `A-*` IDs. Ring-stiffener oracle twin (`en1999-v12`) with a third-party check via sectionproperties (deviation 3.5e-12). Check keys for the wave-1 "schema-blocked" items exist in code (see drift).
- Still open: table-blocked items (A-23 Annex H method B, A-61 web crippling and its combination, exact η0/γ0/ξ0 and ψ/η/ξ exponents, α_max and κ tables, full Table 3.2 beyond 6082-T6; `en1999-v12` reports the other alloy rows as repo values, not verified). Not modelled: HAZ TIG and T-junction factors, shear buckling ρv rule, stress-gradient treatment. `.feature` adapter not written.

## 6. Totals

- Spec IDs (norm scope): 274. Covered by a trace row: 222 (81 %). Missing: 52. Trace-only: 2.
- Engineering-critical missing (MISSING ranks 1 to 26): EN 1998-5 geotechnics and EN 1998-2/-3 bridges and assessment, plus EN 1997-2 ground investigation inputs.
- Most of EN 1998 Parts 2 to 6 has no trace coverage. EN 1997-2 has none.
