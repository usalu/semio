# 🧩 EN 1994 Norm Completion Report

Artifact: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994` (subset `🏅️standards/🔖️1/🪆️subsets/✳️any`, abbreviated `ANY`). Brief: `norm-completion-brief.md`. Spec: `📐️specs/en1994-en1999-target.md` (C-01..C-54).

## 1. What changed (architecture)

- New pure formula module `ANY/🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🦀️.rs` (SI, no model coupling, named constants with provenance) with its worked-example tests in `🧮️catalogue/🧪️tests/🦀️.rs`.
- `part_1_1`, `part_column`, `part_slab`, `part_1_2` of `⚖️checks/🦀️.rs` now delegate to it (public signatures used by the scene crate are kept).
- The compliance report moved out of the 960-line `💡️inferences/🦀️.rs` into `💡️inferences/🧮️compliance/` (`🏗️beams`, `🧱️slabs`, `🏛️columns`, `🔥️fire-and-bridge`); `evaluate` is re-exported unchanged.
- Part labels follow the annex (`EN …` / `DIN EN …`); the constant "(DE-NA)" suffix and the hard-coded action ids (`Q-office`, `Q-col`, `Q-constr`) are gone (remedies find the governing action).

## 2. Defects fixed (inventory section 8.1)

| # | Defect | Fix |
|---|---|---|
| 1 | Stud and longitudinal shear demand = full-interaction N_cf | Demand is the concrete force N_c,req(M_Ed) from inverting the stress-block moment (`required_concrete_force`); stud check = N_c,req / (n_half·P_Rd); slab shear flow = N_c,req/(L_e/2) |
| 2 | Slab m-k took max(m-k, τ) with invented factors | Single m-k resistance V_l,Rd = (b d_p/γ_VS)(m A_p/(b L_s)+k); τ_u,Rd is derived from it for the partial-connection moment |
| 3 | Unexplained 0.5 factor in partial-interaction moment | Stress-block (C-12) and linear interpolation (C-13) both implemented; invariant stress-block ≥ linear tested |
| 4 | LTB used the Euler load | M_cr: bare beam with warping (construction stage), inverted U-frame k_s (hogging); χ_LT from λ_LT = √(M_pl,k/M_cr), α_LT by h/b |
| 5 | SLS stress steel-only with 0.9 f_y | Staged transformed-section stresses (M_a on steel, M_G with n_L, M_Q with n_0); f_y, 0.6 f_ck limits |
| 6/7 | Invented deflection stiffness, φ-form 0.55 | δ = construction + G(n_L, ψ_L=1.1) + Q(n_0) + shrinkage (ψ_L=0.55) with numerically verified I(n); two-span variant; separate variable-action check L/300 |
| 8 | b_eff check failed in the capped case | b_eff/spacing ≤ 1 always; the explanation reports the cap; b_0 is the connector-row distance, not the flange width |
| 9–11 | Column EI, M_pl, M_max, polygon and confinement invented | (EI)_eff = E_a I_a + E_s I_s + 0.6 E_cm I_c with E_cm,eff; fibre-integrated A-C-D-B polygon; second-order k; CHS confinement function tested |
| 12 | Crack control heuristics, 1.0 m cap, 0.9·500 MPa | A_s,min = k_s k_c k f_ct A_ct/σ_s with z_0 from the transformed section, cracked-section σ_s + Δσ_s, EN 1992-1-1 Table 7.3N spacing |
| 13/15 | Fire on the first beam, stored (unresolved) steel | Per-beam fire checks with catalogue-resolved steel; EN 1990 6.11b combination (ψ_1,1 leading, ψ_2,i); protected-steel temperature by time stepping (EN 1993-1-2 Eq. 4.27); M_fi,Rd with k_y,θ |
| 14 | Bridge fatigue used 0.85·W_pl on stored steel | W_el = I/z of the resolved section; reinforcement fatigue check added |
| 16 | Hidden catalogue overwrite | Kept deliberately (documented): geometry leaves of catalogue designations are ignored by design, covered by the leaf-impact test |
| 17/18 | Hard-coded action ids and heuristic remedy magnitudes | Generic `load_remedy`; crack and LTB remedies found by bisection on the check itself |
| 19 | Part label always "DIN EN", "(DE-NA)" suffix | Annex-dependent label, suffix removed |
| 23 | Stud spacing utilisation ignored the lower bound | max(s/s_max, s_min/s) |
| presets | Default sheeting b_0 = 35 mm and h_sc = 75 mm made every stud fail | `trapezoidal()` b_0 = 100 mm, `typical()` studs h_sc = 95 mm, 52 studs (matches the committed examples) |

## 3. Catalogue coverage

Implemented as checks (id prefix `en1994.`): C-01 `3.materials`; C-02 `5.5.class`; C-03 `5.4.1.2.beff`; C-04 `5.4.2.analysis`; C-05/C-06 inside stress, deflection, crack; C-07 `6.2.1.3.mrd`; C-08 `6.2.1.2.hogging`; C-09 inside `mrd` for class 3/4; C-12 `6.2.1.3.mrd`; C-13 `6.2.1.3.linear`; C-14 `6.6.1.2.etamin`; C-15 `6.6.1.1.layout`; C-16/C-17 `6.6.3.1.prd`; C-18 `6.6.3.1.prd`; C-19 `6.6.6.vlrd`; C-20 `6.6.5.5.spacing`, `6.6.5.detailing`; C-21/C-52 `2.6.8.*`; C-22 `6.2.2.vpl`, `6.2.2.4.vm`; C-23/C-51 `6.4.ltb`; C-25 `6.7.1.scope`; C-26/C-27/C-28 `6.7.3.npl`; C-29/C-30 `6.7.3.mn`; C-33 `9.3.sheeting`; C-34/C-36 `9.7.2.mrd`; C-35 `9.7.3.mk`; C-37 `9.7.5.vrd`; C-38 `9.7.6.punching`; C-39 `9.8.sls`; C-40 `7.3.1.deflection(-variable)`; C-41 `7.2.2.stress`; C-42/C-43 `7.4.crack`; C-44/C-45/C-47 `1-2.4.3.theta`, `1-2.4.3.mfi`; C-46 `1-2.4.2.insulation`, `1-2.slab.insulation`; C-49/C-50/C-53 via the beam checks on bridge girders (effective width at supports, cracked analysis, crack control, stud group); C-48 and C-54 and the remaining gaps are listed below.

## 4. Review findings applied (`review-en1993-en1994.md`, EN 1994 part)

| Finding | Change |
|---|---|
| 23 two-span statics | `part_en1990::action_internals`: permanent actions 9qL²/128 / qL²/8 / 5qL/8, variable sagging pattern 49qL²/512 with hogging from both spans, point loads 5PL/32 (13PL/64 one span), 3PL/16, V_B 11P/16; test `two_span_statics_follow_the_exact_coefficients`. The shared mechanics kernel was not used: it is not a dependency of this crate, so the closed-form two-span coefficients live in `part_en1990` |
| 24 crack Δσ_s | α_st = A·I/(A_a·I_a) of the cracked section (steel + reinforcement); test `two_span_cracked_alpha_st_is_the_cracked_ratio` |
| 25 C-04 | span ratio computed from the spans (minimum check vs 0.6), explanation states the one-span-length model limit |
| 26 χ_LT | α from Table 6.4: rolled 0.21/0.34, welded (non-catalogue) 0.49/0.76; W_pl for class 1/2, W_el for class 3 |
| 27 stud rule | transverse rule removed (no transverse spacing field), factor swap documented in the docstring |
| 28 E_s | 210 GPa |
| 29 slab vertical shear | EN 1992-1-1 6.2.2 form (C_Rd,c, k, v_min); E1-09 expected value 46.98 kN/m (the spec value 49.17 kN/m follows the ENV form); oracle and feature updated |
| 30 insulation table | unchanged, listed as unverified project data below |
| 5 negative capacity | in the shared compliance crate, not touched (not in the EN 1994 write scope) |

## 5. Tests

- Rust unit tests: `⚖️checks/🧮️catalogue/🧪️tests/🦀️.rs` (E1-01..E1-12, required-force monotonicity, U-frame stiffness, Table 7.3N, review regressions, oracle comparison with 30 quantities), `🧪️tests/⚖️compliance/🦀️.rs` (stress block vs spec E1-02, stud demand follows the moment, LTB via M_cr, m-k without max, two-span statics).
- Python: `🔮️oracles/🧮️catalogue/🐍️.py` (independent strip integration, double numerical integration of the curvature, RK4), `🔮️oracles/⚖️compliance/🐍️.py` (report comparison for beff, M_Ed, M_Rd, N_c,req, η, η_min, insulation), `🧪️tests/🧮️catalogue-en1994/🥒️.feature` plus runner `🐍️.py` (27/27 rows reproduced locally).
- Executed locally without the repo build: the catalogue module compiled standalone with the toolchain `rustc --test` against its own test file: 18/18 tests pass, including the oracle comparison (scratch copy only, no repo output).
- Repo builds (`🗑️generated/build-en1994/`): runs 1-4 failed in tooling (structural-scene manifest, compiler-ingress refusal, app-surface source budget, partition-cache owner). Run 5 reached rustc and found one real error in this artifact (`default_beam_actions` lived in the artifact root instead of the model crate; moved into `📸️snapshot/🦀️.rs`). Run 6 failed on compiler-image disk headroom, run 7 was cancelled when the central build runner took over (it requests `:build`, then `:test`, via `latest.md`; request is queued). The checks, inferences and downstream crates are therefore still uncompiled by the repo toolchain; they are reviewed by reading only.

## 6. Remaining gaps

- **Per-leaf set-field mutation kinds (coordinator request, not done).** `EDIT_RULES` still resolves 19 leaves and the `replace_row` fallback in `🧭️edit-rules` is still in place. A kind per editable leaf (about 110 leaves of beam, steel section, sheeting, studs, action, column, slab) needs per leaf: payload leaf, diff patch field and apply/inverse in `🔺️diff`, schema facets (json, graphql, proto, ts) of mutation and diff, binary tag, oracle catalog and python component entry, label census, 5 fixtures, feature rows. This is a generator-sized change that cannot be validated blind while no build of the artifact compiles; it must follow a green baseline. Design: one kind per entity leaf group generated from the field-meta table, `steel.designation` as a catalogue choice field (`SteelSection::from_catalogue` list), Placement fields from the scene crate through the same kinds.
- C-10 non-linear resistance (M-κ), C-24 patch loading of webs, C-31 load introduction, C-32/C-46 column fire (tables not transcribed, not guessed), C-48 bridge actions (EN 1991-2 load models), C-54 bridge detailing.
- Spec E1-12 and E1-13 stay placeholders (need transcribed tables / k_c, C_4).
- Mojibake in the editor files (editor-owned, request logged).
- Unequal spans, interior spans and cantilevers for L_e (Fig. 5.1), EI₂ over 15 % of the span in continuous deflection, sagging V-M interaction, encased I-section column geometry (review findings 31, 32, 34).

## 7. Unverified normative values


- Table 6.2 `k_t,max` cell values and the NA-DE sheeting restriction (spec marks [T]).
- `k_c·C_4 = π` (uniform moment, conservative) and α = 2 in k_1; I_2 from transverse reinforcement with lever 0.8 d (EN 1994-1-1 6.4.2 tables not transcribed).
- Creep φ = 2.5, ε_cs = 0.3 ‰, ψ_L = 1.1 / 0.55; NA-DE values for φ, ε_cs and the deflection limits (L/250 total EN, L/300 DE total, L/300 variable).
- Rib pitch 0.25 m, ribs developed length factor 1.2, sheeting S280GD f_yp = 280 MPa, sheeting formwork resistance as two idealised flanges at 0.9 h_p, ep = e = h_p/2, τ_u,Rd derived from m-k (no τ_u,Rk test input).
- Slab punching C_crit at 2 d_p around a 200 mm square footprint (EN 9.7.6 Fig. 9.8 not transcribed); h_eff of the slab fire criterion and its R30/60/90/120 limits 60/80/100/120 mm (spec [T]).
- Fire: board data λ_p = 0.12, c_p = 1000, ρ_p = 800 (no protection material in the model); the Table 4.2 insulation thicknesses are the values already encoded in the repo; uniform steel temperature for M_fi,Rd.
- Reinforcement fatigue Δσ_Rsk = 162.5 MPa at 10^6 cycles, k_2 = 9, γ_s,fat = 1.15; FLM3 surrogate forces (240 kN, 60 kN).
- Columns: encased and partially encased kinds use the equivalent-tube idealisation (no I-section geometry in the model); rebar at 50 mm from the edge; E_cm from f_cm = f_ck + 8 MPa.

