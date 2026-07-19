# Norm Feature Complete — Per-Part Gate Checklist (Round 2)

Gate per part: real formulas, clause tables, DE-NA divergence (where the standard has NDPs), numeric worked-example test, evaluate() coverage.

Verified: `CARGO_TARGET_DIR=<ticket>/target-norm cargo test -p norm_core -p norm_din_4108 -p norm_din_en_16798 -p norm_din_v_18599 -p norm_en_1990 -p norm_en_1991 -p norm_en_1992 -p norm_en_1993 -p norm_en_1994 -p norm_en_1995 -p norm_en_1996 -p norm_en_1997 -p norm_en_1998 -p norm_en_1999` — **214/214 passed** across 14 crates (up from 130 at the prior, premature closure). `norm-plugin` verified separately once the unrelated `semio-framework-plugin` compile error (another session's in-flight Media refactor) resolved.

## Decisions documented in crate docstrings
- **DIN 4108 / DIN V 18599**: national standards — no EN/DE annex split, `AnnexChoice::De` fixed, no `annex` Document field.
- **EN 1997**: models the classic Eurocode 7 generation only (part 1 general design incl. piles, part 2 ground investigation) — no part 3.
- **DIN EN 16798**: models the 9 normative parts only (1, 3, 5-1, 5-2, 7, 9, 13, 15, 17); even-numbered Technical Reports absorbed into their sibling normative part.
- **EN 1998**: dam-stability content removed entirely (out of EN 1998 scope).

## norm_core
- [x] Quantity/ClauseId/CheckResult/CheckReport/AnnexChoice/NationalAnnex/table lookups/NormFamily/NormHost
- [x] QuantityKind::Acceleration (EN 1998 spectra)

## norm_en_1990 — basis of structural design
- [x] Persistent/Transient (6.10/6.10a/6.10b), Accidental, **Seismic (Eq. 6.12b, newly wired into evaluate)** — all in evaluate()
- [x] Genuine DE-vs-EN ψ-table divergence ("other" category) + numeric + divergence tests
- [x] `annex: AnnexChoice` (renamed from `use_de_na: bool`)

## norm_en_1991 — actions on structures
- [x] part_1_1 imposed load + self-weight (absorbed from old part_1_2) — evaluate + numeric test
- [x] part_1_2 fire actions (ISO 834 standard/external/hydrocarbon curves) — evaluate + numeric test
- [x] part_1_3 snow: DE zone/altitude formula vs EN user s_k — evaluate + divergence test
- [x] part_1_4 wind: DE zone table vs EN user v_b — evaluate + divergence test
- [x] part_1_5 thermal actions, part_1_6 construction loads, part_1_7 accidental impact/explosion — evaluate + numeric tests
- [x] part_2 bridge LM1 with DE α_Q divergence + mid-span moment — evaluate + divergence test
- [x] part_3 crane φ1/φ2 dynamic factors — evaluate
- [x] part_4 silo Janssen horizontal pressure (real asymptotic formula) — evaluate + numeric test

## norm_en_1992 — concrete structures
- [x] part_1_1 flexure/shear/punching/torsion/crack/prestress/FEM — evaluate + numeric tests
- [x] part_1_2 fire: Table 5.5/5.6 (b_min, a) tabulated data — evaluate + numeric test
- [x] part_2 bridges: 0.6·f_ck stress limit + reinforcement fatigue — evaluate + numeric test
- [x] part_3 liquid-retaining: tightness-class crack-width limits — evaluate + numeric test
- [x] part_4 **fastenings** (renamed from mislabeled "precast"): steel/concrete-cone/edge-shear anchor resistance — evaluate + numeric test
- [x] AnnexParams α_cc EN 1.0 / DE 0.85 — genuine divergence + test

## norm_en_1993 — steel structures (largest restructure)
- [x] part_1_1 general rules + net-section tension (γ_M2 fix) + CHS classification — evaluate + numeric test
- [x] part_1_2 fire: critical temperature θ_a,cr — evaluate + numeric test
- [x] part_1_3 cold-formed members: effective width ρ — evaluate + numeric test
- [x] part_1_4 stainless steel — evaluate + numeric test
- [x] part_1_5 plated structures (renamed from pile-driving content, which moved to part_5) — evaluate + numeric test
- [x] part_1_6 shells: critical buckling stress — evaluate + numeric test
- [x] part_1_8 bolts (bearing formula fixed) + fillet welds — evaluate + numeric test
- [x] part_1_9 fatigue (moved from old part_1_3) — evaluate + numeric test
- [x] part_1_10 through-thickness: Table 2.1 bilinear lookup — evaluate + numeric test
- [x] part_1_11 tension components (moved from old CHS content) — evaluate + numeric test
- [x] part_1_12 high-strength steel S460–S700 elastic-only — evaluate + numeric test
- [x] part_2 bridges: damage-equivalence fatigue — evaluate
- [x] part_3 towers/masts — evaluate (was dead, now wired)
- [x] part_4 silos/tanks/pipelines (Janssen membrane stress vs part_1_6 shell resistance) — evaluate
- [x] part_5 piling (absorbed driving-stress limit) — evaluate
- [x] part_6 crane runways (moved from old part_1_6) — evaluate + numeric test
- [x] AnnexParams γ_M1 EN 1.0 / DE 1.1 — genuine divergence + test
- [x] All 16 parts reached from check_full_steel_member/evaluate (was 9/16)

## norm_en_1994 — composite structures
- [x] part_1_1 bending/shear + stud resistance pair (§6.6.3.1, both branches, α from h_sc/d) + shear-connection degree — evaluate + numeric test
- [x] part_1_2 fire — evaluate
- [x] part_2 bridges + stud fatigue (new) — evaluate
- [x] AnnexParams documented EN=DE equality + test

## norm_en_1995 — timber structures
- [x] part_1_1 + real k_cr divergence (EN 0.67 vs DE min(1.0, 2.5/f_v,k), fixes prior no-op alias) — evaluate + divergence test
- [x] part_1_2 fire (char depth) — evaluate
- [x] part_2 bridges: pedestrian vibration + fatigue reduction (real content, was thin wrapper) — evaluate + numeric test

## norm_en_1996 — masonry structures
- [x] part_1_1, part_1_2 — evaluate
- [x] part_2 **materials/execution** (rewritten from dead "lintel" content): exposure class admissibility + joint thickness — evaluate + numeric test
- [x] part_3 **simplified calculation method** (rewritten from retaining-wall content): Φ_s reduction factor — evaluate + numeric test
- [x] AnnexParams γ_M EN class table vs DE flat 1.5 — genuine divergence + test

## norm_en_1997 — geotechnical design
- [x] part_1 bearing/sliding + pile design (§7, ξ correlation factors, absorbed from old part_2) — evaluate + numeric test
- [x] part_2 **ground investigation** (rewritten from misplaced pile-design content): CPT/SPT → φ′ derivation + min. investigation depth — evaluate + numeric test
- [x] DesignApproach/AnnexParams DA2* (DE) vs DA2 (EN) — genuine divergence + test

## norm_en_1998 — seismic design
- [x] part_1 base shear + **EN Type 1/2 spectrum vs DE zone model** (genuine NDP divergence, not just a factor) — evaluate + divergence test
- [x] part_2 bridges — evaluate
- [x] part_3 **assessment & retrofitting** (rewritten from misplaced silo content): knowledge levels → confidence factors — evaluate + numeric test
- [x] part_4 **silos/tanks/pipelines** (rewritten from misplaced tower content, absorbed silo content from old part_3): impulsive/convective base shear — evaluate + numeric test
- [x] part_5 **foundations/retaining** (rewritten, absorbed from old part_6, dam content deleted): Mononobe–Okabe — evaluate + numeric test
- [x] part_6 **towers/masts/chimneys** (rewritten, absorbed from old part_4) — evaluate + numeric test

## norm_en_1999 — aluminium structures
- [x] part_1_1 general rules + welds/HAZ softening (absorbed from deleted part_1_6) — evaluate + numeric test
- [x] part_1_2 fire, part_1_3 fatigue — evaluate
- [x] part_1_4 **cold-formed sheeting** (rewritten from fabricated "food-contact coating") — evaluate + numeric test
- [x] part_1_5 **shell structures** (rewritten from mislabeled "hollow section") — evaluate + numeric test
- [x] part_1_6 deleted (not a real EN 1999 part)
- [x] AnnexParams documented EN=DE equality + test

## norm_din_en_16798 — indoor environmental input parameters
- [x] part_1 PMV/PPD + adaptive comfort + daylight/acoustic categories + CO₂ (DE-vs-EN divergence) — evaluate + numeric + divergence test
- [x] part_3 ventilation: IDA/ODA classes, SFP classes, heat-recovery minimum (absorbed old parts 4/9/10/11) — evaluate + numeric test
- [x] part_5_1/part_5_2 fan energy / heat-recovery savings (rewritten from thin constants, absorbed old part_12) — evaluate + numeric test
- [x] part_7 infiltration (moved from old part_7's dwelling content, now real n50/V formula) — evaluate + numeric test
- [x] part_9 cooling degree-hour energy need (rewritten from thin constant) — evaluate + numeric test
- [x] part_13 chiller EER + generation energy (rewritten from fixed 30 dB constant, absorbed old part_16) — evaluate + numeric test
- [x] part_15 storage losses (absorbed old part_14 DHW content) — evaluate + numeric test
- [x] part_17 duct leakage class A–D (rewritten from fixed capture-velocity constant, absorbed old part_8) — evaluate + numeric test
- [x] Restructured 17 modules → 9 real normative parts

## norm_din_4108 — thermal performance of buildings
- [x] part_1 input plausibility validation (honest rewrite — DIN 4108-1 is withdrawn) — evaluate + numeric test
- [x] part_2 U-limit, part_3 Glaser + f_Rsi, part_4 material λ, part_5 summer heat, part_6 U + bridges, part_7 airtightness, part_8 catalog — evaluate + numeric tests
- [x] part_10 (new) factory-made insulation application types — evaluate + numeric test
- [x] Beiblatt 2 (new) thermal-bridge equivalence check — evaluate + numeric test

## norm_din_v_18599 — energy balance of buildings
- [x] part_1 **primary energy aggregation** (rewritten from scope-only): Σ Q_f,i·f_p,i — evaluate + numeric test
- [x] parts 2–12 balance_annual — evaluate + numeric tests (part_8 cooling numeric test added)

## norm-plugin
- [x] All 13 families registered as DocumentApp via define_norm_family_app!
- [x] "evaluate" view action fixed: now emits a real SetDocument commit (was a discarded no-op `NormHost::from_document`)
