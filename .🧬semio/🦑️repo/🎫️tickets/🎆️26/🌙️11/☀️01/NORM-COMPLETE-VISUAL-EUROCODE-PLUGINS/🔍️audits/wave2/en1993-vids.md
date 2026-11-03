# EN 1993 V-id traceability and catalogue-gap checks (wave 2, vids agent)

Nothing was compiled or run (rule). Python was run only for the registry script, the entry generator and the oracle. All Rust is unverified by a compiler: expect first-compile fixes (see "Risks").

## 1. Before and after (V-ids of `catalogue-coverage-2.md`, 142 ids)

| State | implemented (check family traced) | data table | clause-cited only | not modelled (with reason) | missing |
|---|---:|---:|---:|---:|---:|
| Before (audit v2) | 0 | 0 | 55 | 0 | 87 |
| After | 124 | 6 | 0 | 12 | 0 |

- 124 `Check` rows: 104 map to families that already existed under clause keys (the audit could not see them), 20 V-ids are newly realised by 16 new check families (below).
- 6 `Data` rows: V-001, V-014, V-015, V-366, V-600, V-601 (partial-factor registry, bolt classes, stainless grade table, k_sigma tables, reduction factors, specific heat).
- 12 `NotModelled` rows, each with a reason: V-034 (hybrid f_y,red), V-066 (warping torsion, no bimoment), V-126 (nomogram k, no G_A/G_B), V-250 (no f_u, n, A_g on the cold-formed row), V-376 (v4 package), V-407 (eccentric bolt group), V-501 (stress-range derivation), V-504 (rainflow, no stress history), V-505 (Tables 8.1-8.10 not in repo), V-506 (informational annex), V-609 (advanced FE methods), V-805 (lateral pressure on plates).
- The audit's `Check(prefix)` literal recogniser targets EN 1999's tuple form; EN 1993 uses the typed struct `Entry { id, clause, entity, realisation, note }` with `Realisation::{Check(&[prefixes]), Data(path), NotModelled(reason)}`, so the audit script needs one extra regex to count them.

## 2. Traceability table and test

- `catalogue::ENTRIES` (+ `catalogue::entry(id)`): `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📏️parts/🦀️.rs`, generated from the spec headings by `🧰️scripts/en1993-vids-entries.py` (needs the scratch spec extract; the generated Rust is the source of truth now).
- Test `every_catalogue_id_is_traced_to_a_produced_check_family` plus `the_catalogue_lists_the_142_ids_of_the_spec_once` in `⚖️checks/🧪️tests/🗂️catalogue-trace/🦀️.rs`: runs `check_full_steel_structure` on 9 example documents (compliant, overloaded, portal and special-parts showcases, and variants with tension, torsion plus Mz, parabolic moments, bolted tension plus category B, welded flange joint) and asserts that every `Check` row has a produced prefix; `Data` needs a path, `NotModelled` a reason (> 30 chars).
- Surfaced in the editor catalogue panel as the reference table `table-en1993-traceability` (142 rows, EN + DE headings).

## 3. New check families (module `⚖️checks/🧱️member-checks/🧾️vid-checks`, checks crate; all with clause, utilisation or informative value, quantities, EN + DE text, remedies on failure, unverified NDP rows cited)

| V-id | Check id prefix | Entity | Formula |
|---|---|---|---|
| V-010 | `en1993.3.2.1.grade` | materials | declared f_y, f_u against Table 3.1 (t <= 40 mm, S235, S275, S355, S450) |
| V-011 | `en1993.3.2.2.ductility` | materials | f_u/f_y >= 1.10 (elongation and eps_u not carried) |
| V-013 | `en1993.3.2.6.constants` | materials | E = 210 GPa, G = E/(2(1+nu)), 1 % tolerance |
| V-024/025/026 | `en1993.5.3.2.bow` | members | e_0/L of Table 5.1 (registry rows), inclusion rule 5.3.2(6), k e_0 for LTB; informative |
| V-050 | `en1993.6.2.6.av` | members | A_v of 6.2.6(3) (rolled/welded I) against the stored shear area (HEB 240 stores h t_w = 24.0 cm2, formula 33.2 cm2: conservative) |
| V-101/102 | `en1993.6.3.1.tf` | members | N_cr,T, lambda_T, chi_T on the z-z curve, N_b,Rd,T |
| V-111 | `en1993.6.3.2.3.rolled` | members | chi_LT (Eq. 6.57, Table 6.5), f of Eq. 6.58 with k_c of Table 6.6 |
| V-112 | `en1993.6.3.2.4.cf` | members | compression flange as column, lambda_f <= lambda_c0 M_c,Rd/M_y,Ed |
| V-091 | `en1993.7.2.3.freq` | members | f_1 = pi/(2L2) sqrt(EI/m) >= 3 Hz (uniform load, permanent mass) |
| V-405 | `en1993.1-8.3.10.block` | joints | V_eff,1,Rd concentric block tearing |
| V-402 | `en1993.1-8.3.6.punching` | joints | B_p,Rd = 0.6 pi d_m t_p f_u/gamma_M2 per bolt |
| V-413 | `en1993.1-8.4.11.beff` | joints (welded) | b_eff = t_w + 2s + 7 k t_f against f_y,p b_p/f_u,p |
| V-500/502 | `en1993.1-9.6.cafl` | fatigueDetails | Delta sigma_D, Delta sigma_L; constant amplitude: gamma_Ff Delta sigma <= Delta sigma_D/gamma_Mf (Fail with remedies), variable amplitude: informative ratio (the Miner check decides) |
| V-607 | `en1993.1-2.4.2.3.ltb` | fireExposures | M_b,fi,t,Rd = chi_LT,fi W_y k_y,theta f_y/gamma_M,fi against 0.65 M_Ed |
| V-608 | `en1993.1-2.4.2.3.int` | fireExposures | Eq. 4.21a and 4.21b (k_y, k_LT, mu bounds) |
| V-252 | `en1993.1-3.5.4.curl` | coldFormedMembers | u = 2 sigma_a2 b_s4/(E2 t2 z) <= 5 % of the depth |

Units: new incremental unit tags `vidMaterials vidMembers vidJoints vidFatigue vidFire vidColdFormed` appended at the end of `🧮️units` in the order of the calls (the exact-order fold test of the existing documents stays valid; frame documents already order their frame unit differently, my set-based test covers them).

## 4. Fixes found on the way

- `clauses::cold_formed::flange_curling` had the wrong, dimensionally inconsistent formula `2 sigma^4 b^4/(E^2 t^2 z^2)`; EN 1993-1-3 Eq. 5.1 is `2 sigma_a^2 b_s^4/(E^2 t^2 z)`. Fixed and unit-tested (0.044082 mm for 150 MPa, 60 mm, 2 mm, 75 mm).
- `CombinedScalar` and `CombinedEffects` (+ fields) were `pub(super)` of the core crate although the checks and special crates read them: made `pub` (latent E0603/E0616).
- `unused clause functions` now wired: `block_tearing`, `bp_rd`, `delta_sigma_d/l`, `flange_curling`, `n_cr_torsional`, `chi_lt_rolled`, `ltb_f`, `kc_psi/kc_udl_fork`, `ltb_curve_table_6_5`, `simplified_compression_flange_utilisation`, `bow_ratio`, `bow_must_be_included`, `av_rolled_i`, `av_welded_web`, `floor_frequency`, `m_b_fi_t_rd`-family. Still unwired (reasons in the table): `rainflow`, `bolt_group_force`, `f_ya`, `k_nomogram`, `warping_torsion_fork`, `plate_lateral_pressure_stress`, `angle_net_resistance`.
- New clause functions with worked-example unit tests: `block_tearing_areas`, `punching_mean_diameter`, `weld_flange_beff`, `weld_flange_thickness_for`, `m_b_fi_t_rd`, `mu_y_fire`, `mu_lt_fire`, `k_fire`, `bending_axial_fire`.

## 5. Unverified values (never invented; all rows `unverified`, written from memory, flagged in `evidence`)

13 new registry rows (`🧰️ndp/wave2_en1993_vids.py`): `ndp.en1993.v-010.1` (Table 3.1 cells S235/S275/S355/S450 for t <= 40 mm: 235/360, 275/430, 355/510, 440/550 MPa), `v-011.1`, `v-013.1` (1 % tolerance is an engineering value), `v-024.1`, `v-050.1` (3 % tolerance), `v-091.1` (3 Hz), `v-101.1` (l_T = minor-axis buckling length), `v-112.1` (lambda_c0, k_fl), `v-402.1` (across-flats 1.5 d), `v-405.1` (block geometry), `v-413.1`, `v-607.1`, `v-608.1` (beta_M = 1.3). Reused existing rows: `v-026.1`, `v-111.1/2`, `v-252.1`, `v-502.1`, `t51.*`. Every new check therefore reports `UnverifiedBasis` instead of Pass, as the registry rules demand (note that `gamma-m*` are unverified as well, so `complies()` is already false for the compliant document until a human verifies them; this is not changed by my rows).

## 6. Diagrams, 3D, oracle and features

- Diagrams (scene, entity diagrams): `tf-buckling:<member>` (chi curve of the z-z axis with torsional operating point and flexural slenderness), `fire-ltb:<fire>` (M_b,fi,theta,Rd over theta with M_fi,Ed and the operating point); detail sketch `block-tearing:<joint>` (plate, bolt grid, torn-out block, shear and tension faces). Existing diagrams already cover the S-N curve (`sn:`), M-N surface (`mn:`), classification (`class:`), fire curves (`fire:`, `fire-k:`), chi-LT (`ltb:`). No new 3D entities (no new geometry fields); the block sketch is 2D with table twin.
- Oracle `🔮️oracles/🧾️vids/🐍️.py` (numpy; third-party `sectionproperties` recomputes J = 1.03727e-6 m4 and Gamma = 4.76234e-7 m6 of the HEB 240 outline by FE, N_cr,T = 10.1728 MN against the catalogue-value 10.1709 MN: 2e-4) plus feature `🧪️tests/🧾️vid-checks/🥒️.feature` (8 rows) with Rust subject adapter and Python oracle adapter (same step layer as `🕸️frame-eigen`). Values printed by the oracle: ncrT 10170865.9 N, block tearing 429685.5 N, curling 1.9592e-5 m, Delta sigma_D(71) 52.3132 MPa, M_b,fi(560 C) 170.968 kNm.
- Rust tests (checks crate, `🗂️catalogue-trace`): torsional closed form, block tearing hand formula and failing remedy, punching/b_eff variants, material fails with remedies, shear area, bow/rolled/simplified/frequency, CAFL and fire summaries, flange curling, unit-fold audit on all example documents.

## 7. Risks and open items

- First compile: the three `vid-checks` files, scene `entity-diagrams` (two functions) and `detail-sketches` (`block_tearing_sketch`), editor catalogue panel table, test module. Debug asserts: every Fail must carry a remedy (all branches add one) and every cited NDP id must exist (all registered).
- Another agent (NDP rewrite) already replaced `E_STEEL` by `e_steel()` in my curling code; I left that as is.
- Re-bless the scene golden and the DSL assets (`SEMIO_UPDATE_GOLDEN=1`), register the oracle case ids (requests.md).
- Not done: V-250, V-407, V-501, V-504 (data model gaps listed above; no schema change this pass because 1544 fixtures and five twins would be touched while v4 edits the same snapshot), the angle net-section check (V-405 second half), stainless grades beyond 1.4301 in fire.
- Coordination: v4 owns V-376, orthotropic deck, 1-5 Annex A/D, 1-6 shells, purlins and the en1993 mutations; none touched. Announced in `📌️important/requests.md` ("en1993 vids (wave 2)").
