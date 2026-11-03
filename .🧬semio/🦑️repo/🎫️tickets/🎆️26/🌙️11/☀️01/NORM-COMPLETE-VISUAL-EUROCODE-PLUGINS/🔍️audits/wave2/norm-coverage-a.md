# 🧭 Norm Coverage Audit A (wave 2): EN 1990 to EN 1994

Read-only audit. Nothing compiled or run (no cargo, no `repo`). Status is judged from the spec text, the spec-trace JSON and literal grep of check ids in the artifact sources. Semantics of individual checks were not audited.

Abbreviations: `ARTI` = `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts`. `SPEC` = `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/📐️specs`. Trace JSON = `ARTI/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json`. EN 1993 source root = `ARTI/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any`. Checks module path = `…/🧬️schema/💡️inferences/⚖️checks/`.

Method. Spec catalogue ids were taken from every bold id (`**E90.13`, `**V-424`, `**C-54` …) in the spec md files, inline or at line start. Group labels, visualisation views (`📐 §4–§5`) and worked examples (`E-01..E-18`, `E01..E16`, `E1-xx`) are excluded from the catalogue count and reported separately. Plain-text clause mentions are counted as "referenced in spec" when the trace has them. Trace kinds: `check` = modelled as a check, `design-case` = realised as a design case, `data` = realised as data or NDP, `not-modelled` = carries a reason in the trace.

## 1. Summary per code

| Code | Spec catalogue ids | Trace items | check | design-case | data | not-modelled | MISSING (spec, not trace) | Excluded from count |
|---|---|---|---|---|---|---|---|---|
| EN 1990 | 30 | 29 | 29 | 0 | 0 | 0 | 1 (E90.13) | none |
| EN 1991 | 100 bold + 9 plain-text only = 109 | 109 | 71 | 28 | 0 | 10 | 0 | 10 views (§4.2) |
| EN 1992 | 160 | 159 | 60 | 98 | 1 | 0 | 1 (M-11) | 18 worked examples E-01..E-18, 14 visualisation VZ-01..VZ-14 |
| EN 1993 | 166 | 142 | 133 | 0 | 5 | 4 | 24 | 16 worked examples E01..E16; table notes C1, Edge, NA-DE |
| EN 1994 | 54 | 54 | 50 | 0 | 3 | 1 | 0 | EN 1999 entries in the same spec file; E1-xx worked examples |

Coverage, "check or design-case or data or not-modelled" over catalogue: EN 1990 97 %, EN 1991 100 % (of which 6 bold not-modelled), EN 1992 99 % (of which 98 design-case, 61 %), EN 1993 86 %, EN 1994 100 %. Modelled as a check (trace kind `check`): EN 1990 97 %, EN 1991 65 %, EN 1992 38 %, EN 1993 80 %, EN 1994 93 %.

Literal check: every trace check id was grepped in its artifact's sources. Result: EN 1990 33 distinct ids (2 are prefixes, present), EN 1991 91 (3 prefixes, confirmed by grep), EN 1992 58 (all present), EN 1993 109 (all present), EN 1994 47 (all present). So trace checks are not orphaned in the code, but nothing has been compiled or run.

## 2. Per part breakdown

### EN 1990 (spec §1, file `en1990-en1991-target.md`)

| Part / clause | Spec ids | check | design-case | data | not-modelled | MISSING |
|---|---|---|---|---|---|---|
| 1.1 cl. 2-3 | 6 | 6 | | | | |
| 1.2 cl. 4 | 5 | 5 | | | | |
| 1.3 cl. 5 | 2 | 1 | | | | 1 (E90.13) |
| 1.4 cl. 6 | 8 | 8 | | | | |
| 1.5 Annex A1 | 3 | 3 | | | | |
| 1.6 Annex A2 | 3 | 3 | | | | |
| 1.7 Annex B, C, D | 3 | 3 | | | | |

### EN 1991 (spec §2, same file)

| Part | Spec ids (bold) | check | design-case | not-modelled | Plain-text only (trace) | Notes |
|---|---|---|---|---|---|---|
| 1-1 (E11) | 9 | 6 | 3 | | | |
| 1-2 (E12) | 10 | 5 | 5 | | | |
| 1-3 (E13) | 13 | 8 | 5 | | | E13.13 now design-case |
| 1-4 (E14) | 16 | 9 | 7 | | | |
| 1-5 (E15) | 7 | 5 | 2 | | E15.06 is bold | |
| 1-6 (E16) | 3 | 2 | 1 | | 3 check (E16.04, .05, .06) | |
| 1-7 (E17) | 9 | 8 | 1 | | 1 NM (E17.10) | E17.10 is "Annex E not used" |
| 1-2 bridges, EN 1991-2 (E2) | 22 | 14 | 3 | 5 (E2.18 to E2.22) | | rail-bridge rows NM |
| 1-3 cranes (E3) | 6 | 5 | 1 | | 1 NM (E3.06) | |
| 1-4 silos (E4) | 5 | 4 | | 1 (E4.09) | 2 check, 2 NM | |

### EN 1992 (spec §2, file `en1992-target.md`)

| Part | Spec ids | check | design-case | data | MISSING |
|---|---|---|---|---|---|
| 2.1 Basis, materials (2, 3) | 14 | 6 | 7 | | 1 (M-11) |
| 2.2 Durability, cover (4) | 7 | 3 | 4 | | |
| 2.3 Analysis (5) | 16 | 5 | 11 | | |
| 2.4 ULS bending, axial (6.1) | 10 | 6 | 4 | | |
| 2.5 Shear (6.2) | 8 | 5 | 3 | | |
| 2.6 Torsion (6.3) | 4 | 3 | 1 | | |
| 2.7 Punching (6.4) | 7 | 4 | 3 | | |
| 2.8 Strut-and-tie, PLA (6.5, 6.7) | 5 | | 5 | | |
| 2.9 Anchorage, laps (8) | 9 | 5 | 4 | | |
| 2.10 Serviceability (7) | 9 | 6 | 3 | | |
| 2.11 Fatigue (6.8) | 7 | | 7 | | |
| 2.12 Prestress (5.10, 3.3) | 10 | 4 | 6 | | |
| 2.13 Slabs (9) | 14 | 1 | 13 | | |
| 2.14 Fire (1-2) | 8 | 2 | 6 | | |
| 2.15 Bridges (1-2, 2) | 9 | 1 | 8 | | |
| 2.16 Liquid retaining (1-3) | 8 | 1 | 7 | | |
| 2.17 Fastenings (1-4) | 14 | 8 | 6 | | |
| 2.18 Model integrity | 1 | | | 1 (DI-01) | |

### EN 1993 (spec §1, file `en1993-target.md`)

| Part | Spec ids | check | data | not-modelled | MISSING |
|---|---|---|---|---|---|
| 1.A Basis, partial factors (1-1 §2, NA) | 2 | 1 | 1 | | |
| 1.B Materials (1-1 §3, 1-4 §2, 1-10) | 6 | 4 | 2 | | |
| 1.C Global analysis (1-1 §5) | 10 | 9 | | | 1 (V-029) |
| 1.D Classification (Table 5.2) | 6 | 6 | | | |
| 1.E Section, cross-section (1-1 §6.2, 1-5) | 17 | 17 | | | |
| 1.F Member stability (1-1 §6.3) | 18 | 18 | | | |
| 1.G Built-up (6.4) | 4 | 4 | | | |
| 1.H Serviceability (§7, NA) | 2 | 2 | | | |
| 1.I Joints (1-8) | 28 | 24 | | | 4 (V-424..V-427) |
| 1.J Fatigue (1-9, NA-DE) | 7 | 4 | | 3 (V-501, V-505, V-506) | |
| 1.K Fire (1-2) | 12 | 10 | 1 | 1 (V-609) | |
| 1.L Cold-formed (1-3) | 11 | 5 | | | 6 (V-255..V-260) |
| 1.M Stainless (1-4) | 6 | 1 | | | 5 (V-301..V-305) |
| 1.N Plated (1-5) | 16 | 15 | 1 | | |
| 1.O Shells, silos (1-6, 4-1, 4-2) | 7 | 6 | | | 1 (V-806) |
| 1.P Towers, cables, piling, bridges, crane runways (3-1, 3-2, 1-11, 5, 2, 6) | 7 | 7 | | | |
| 1.Q Toughness (1-10), misc | 7 | | | | 7 (V-900..V-906) |

### EN 1994 (spec §1, file `en1994-en1999-target.md`, EN 1994 part only)

| Part | Spec ids | check | data | not-modelled | MISSING |
|---|---|---|---|---|---|
| 1.1 Materials, classification, analysis | 6 | 6 | | | |
| 1.2 Composite beam bending | 9 | 8 | 1 | | |
| 1.3 Shear connection | 6 | 5 | 1 | | |
| 1.4 Shear, LTB, bearing | 3 | 3 | | | |
| 1.5 Composite columns | 8 | 8 | | | |
| 1.6 Composite slabs | 7 | 7 | | | |
| 1.7 Serviceability | 4 | 4 | | | |
| 1.8 Fire (1-2) | 4 | 2 | 1 | 1 (C-46) | |
| 1.9 Bridges (2) | 7 | 7 | | | |

## 3. MISSING list, ranked by engineering importance

Each entry is in the spec catalogue but has no row in the spec-trace. Owner = where the trace row and, where it exists, the check code belong.

1. **EN 1993 V-900** (1-10 cl. 2.1), brittle fracture avoidance, `sigma_Ed <= t_max` by grade and subgrade. Safety-relevant for every member. Owner: `EN93/…/⚖️checks/🧮️clauses/🌡️toughness/🦀️.rs` (literal `en1993.1-10.` present) plus trace JSON. Table 2.1 is not encoded (unverified).
2. **EN 1993 V-029** (1-1 cl. 5.6), plastic global analysis conditions (hinge rotation capacity, restraint spacing Lm). Basis of all plastic frame design. Owner: new check in `EN93/…/⚖️checks/🧱️member-checks/` plus trace JSON. No literal check id exists in source.
3. **EN 1993 V-424, V-425, V-426, V-427** (1-8 cl. 6.2.6.4, 6.2.6.5, 6.2.6.6, 6.2.6.8), column flange in bending, end-plate in bending, beam flange and web in compression, beam web in tension. Joint resistance. Code exists: `cbend`, `tbend`, `mjrd` in `EN93/…/⚖️checks/🔗️special-checks/🧩️plate-checks/🦀️.rs` (wave-2 en1993-v14). Trace rows missing. Owner: trace JSON.
4. **EN 1993 V-255, V-256, V-257, V-258, V-259, V-260** (1-3 cl. 6.1.5 shear, 6.1.7 web crippling, 6.1.8 combined with web crippling, 6.1.9 axial plus bending, §8 connections, §10 liner trays and purlin restraint). Cold-formed members. Owner: `EN93/…/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs` (`en1993.1-3.6.` literal present) plus trace JSON.
5. **EN 1993 V-301, V-302, V-303, V-304, V-305** (1-4 effective width, cross-section resistance, flexural buckling, fire, Annex Ramberg-Osgood). Stainless. Owner: `EN93/…/⚖️checks/🧮️clauses/🔬️stainless/🦀️.rs` (`en1993.1-4.` literal present) plus trace JSON.
6. **EN 1992 M-11** (cl. 3.1.4(6), Annex B.2), shrinkage strain. Feeds PR-05 losses and S-12. Currently covered only through A-02, which is design-case. Owner: EN 1992 trace JSON, `ARTI/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json`.
7. **EN 1993 V-902** (1-12), supplementary rules for S460 to S700. Grade-dependent. Owner: `EN93/…/⚖️checks/🔗️special-checks/🏛️shell-and-structure-checks/🦀️.rs` (`en1993.1-12` literal present) plus trace JSON.
8. **EN 1993 V-906** (1-6 thin-shell gate, t/r < 1/20, gates V-800..V-805). Owner: `EN93/…/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs` (`en1993.1-6.8` literal present) plus trace JSON.
9. **EN 1993 V-903** (1-7 out-of-plane plated loading). Partly covered by V-805 (`en1993.1-7.lateral`). Owner: `EN93/…/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs` plus trace JSON.
10. **EN 1993 V-904** (EN 1993-5 piling cross-checks). Owner: `EN93/…/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs` (`en1993.5.5` literal present) plus trace JSON.
11. **EN 1993 V-901** (fire, accidental and seismic cross-references). Cross-reference row only. Owner: EN 1993 trace JSON.
12. **EN 1993 V-806** (1-5 Annex D). Partly covered by V-376 (Annex D corrugated webs). Owner: `EN93/…/⚖️checks/🔗️special-checks/🏛️annex-block-checks/🦀️.rs` (`en1993.1-5.D` literal present) plus trace JSON.
13. **EN 1993 V-905** (EN 1993-4-3 pipelines). Owner: `EN93/…/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs` (`en1993.4-3` literal present) plus trace JSON.
14. **EN 1990 E90.13** (cl. 5.2, design assisted by testing, "see Annex D (E90.30)"). Cross-reference only; E90.30 (`en1990.annex-d.`) covers it. Owner: `ARTI/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json`.

Not-modelled with reason (not MISSING, ranked): EN 1991-2 E2.19 (derailment, 6.8), E2.20 (rail load groups gr11 to gr31, Table 6.11), E2.21 (rail fatigue, 6.10) and E2.22 (HSLM, Annex F); EN 1993 V-501, V-505 (Tables 8.1 to 8.10 not in repo) and V-506 (Paris law); EN 1994 C-46 (fire tables 4.1 to 4.3 not transcribed); EN 1991-3 E3.06 (Annex B dynamic factors); EN 1991-4 E4.06, E4.07, E4.09; EN 1991-2 E2.18 (aerodynamic); EN 1993 V-609 (advanced FE fire, out of scope by spec).

Excluded from the catalogue count, listed for completeness: 1992 worked examples E-01..E-18 (no trace rows, expected as golden tests), 1992 VZ-01..VZ-14 (visualisation), 1993 E01..E16 (worked examples), 1991 §4.2 views.

## 4. Wave-1 completion gaps: closed or open

Status values: CLOSED = evidence in trace or code. PARTIAL = some part closed. OPEN = no evidence. "Claimed" = closure asserted in a wave-2 or follow-up report, not verified in code.

### EN 1990 (wave-1 `en1990-completion.md`)
- Build and test not run: OPEN (en1990-v10 also uncompiled).
- Mutation kinds for other editable leaves: OPEN.
- Snapshot tables FatigueSpectrum, ReliabilityAnalysis, TestSeries: CLOSED (claimed, en1990-v9 item 1).
- Exclusive groups in snapshot (`group` field): CLOSED (claimed, en1990-v9: `variables[].group`).
- Review items S2-8, S2-9, S2-10: OPEN (no wave-2 evidence).

### EN 1991 (wave-1 `en1991-completion.md`)
- Crane class table and crane static wheel load (snapshot field): OPEN.
- E13.13: design-case now (was blocked). PARTIAL.
- E16.04, E16.05: now `check` (trace). CLOSED.
- E17.03, E17.04: now `check` (trace), tables flagged unverified. CLOSED (unverified).
- E2.06, E2.12, E2.13, E2.16 (dispersal, footbridge, rail dynamic): `check` via en1991-v12 collections. CLOSED.
- E2.19, E2.20, E2.21, E2.22: still not-modelled. OPEN.
- E3.06, E4.06, E4.07, E4.09: still not-modelled. OPEN.
- LM1 tandem positions, rail LM71 and SW model fields, silo geometry: PARTIAL (en1991-v12 added footbridges, railDynamics, loadDispersals; the rest OPEN).
- Build and test, mutation oracle manifest, TS twins: OPEN.

### EN 1992 (wave-1 `en1992-completion.md`)
- Compile and test: OPEN.
- Check stations not emitted: CLOSED (claimed; follow-up says emitted in `📋️member-evaluation` for flexure, shear, crack, deflection, punching).
- Snapshot geometry (explicit bars, polygon sections, tendon control points, stud rings, anchor groups, Placement): PARTIAL. Openings added (en1992-v14 task 1). The rest OPEN.
- Review S2-9 (PR-05 sign convention): CLOSED (claimed, follow-up 2).
- Review S2-11 (Table 4.4DE), 15d/e (FA-07), A-07: OPEN, flagged unverified.
- Review 14c/d (fire table facade): CLOSED (claimed, follow-up 2).
- Review 16 (slender column minimum eccentricity), 17 (corner beta), 18 (A-07 Fig. 5.6N, A-15 limits): OPEN (no evidence).
- Unverified NDP values: OPEN by design (needs DIN NA text).
- M-11 shrinkage row: OPEN (surfaced by this audit, section 3 item 6).

### EN 1993 (wave-1 `en1993-completion.md`)
- Engine wiring V-020..V-027 (storeys): PARTIAL. Trace `check` rows exist (`en1993.1-1.5.2.alpha` and others). Engine wiring not verified.
- V-113b, V-123, V-124 (restraints) and V-130..V-133 (built-up): trace `check`. CLOSED at trace level; engine not verified.
- 1-8 component method and joints: PARTIAL. V-420..V-423, V-430, V-433 are `check`. Base plate, T-stub, M_j,Rd and cone breakout are in code (en1993-v14) but V-424..V-427 have no trace rows (section 3 item 3).
- Hollow section joints V-440 and weld groups: trace `check`. CLOSED at trace level.
- 1-3 cold-formed V-250..V-260: PARTIAL. V-250..V-254 `check`; V-255..V-260 MISSING (section 3 item 4).
- 1-4 stainless V-300..V-305: PARTIAL. V-300 `check`; V-301..V-305 MISSING (section 3 item 5).
- 1-5 plated V-360..V-376: PARTIAL. V-376 description says Annex A, B, C stay open (Annex D corrugated done).
- 1-6 shells V-800..V-806: PARTIAL. V-800..V-805 `check`; V-806 MISSING.
- 3-1, 3-2, 1-11, 5, 2, 6 (V-850..V-890): trace `check`. CLOSED at trace level.
- Fire and fatigue on scalar entities: OPEN (no evidence).
- 1-10 temperature fixed at 0 degC, Table 2.1 not encoded: OPEN (V-015 data unverified; V-900 MISSING).
- Annex A/B class 4 e_N shift: OPEN.
- Third-party validation, `.feature` adapter: OPEN.
- Diagrams and grips (editor-ux gaps 2, 3, 18): CLOSED (claimed, en1993-diagrams).

### EN 1994 (wave-1 `en1994-completion.md`)
- C-10 (non-linear M-kappa), C-24 (patch loading), C-31 (load introduction), C-48 (bridge actions), C-54 (bridge detailing): `check` in trace with literal check ids in source. CLOSED at trace level.
- C-32 (column fire simple model): `check`. CLOSED at trace level.
- C-46 (fire tables 4.1 to 4.3): not-modelled. OPEN.
- C-43 Δσs tension stiffening: CLOSED (en1994-v11, claimed).
- E1-12, E1-13 placeholders: OPEN (worked examples only).
- Unequal spans, interior spans, EI2 over 15 % span, sagging V-M interaction, encased I geometry (review 31, 32, 34): OPEN (no evidence).
- Editor mojibake: OPEN (editor-owned).

## 5. Caveats

- Nothing compiled. Every "check" is a trace declaration with a literal id present in the source, not a verified implementation.
- Spec catalogue ids were taken from bold tokens. Prose-only clauses in spec sections (e.g. 1990 §1.8 NA-DE list) are not enumerated.
- Counts are reproducible with the scripts `inline.py` and `sections.py` in `/tmp/claude-1000/-home-deck-semio/84faf2e3-4721-4a15-8b57-5d44cdd1d7f5/scratchpad/norm/`.
