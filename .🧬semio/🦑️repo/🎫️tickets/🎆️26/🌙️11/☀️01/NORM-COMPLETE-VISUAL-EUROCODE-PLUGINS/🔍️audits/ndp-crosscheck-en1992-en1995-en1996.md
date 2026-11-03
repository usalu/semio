# 🔎 NDP Cross-Check EN 1992, EN 1995, EN 1996

🧭 Read-only audit of every `[V]` item in `📐️specs/en1992-target.md` and `📐️specs/en1995-en1996-target.md` against what the repository already encodes. No source or spec was modified.

## 📋 Method and legend

- Spec ids are `E92-nn`, `E95-nn`, `E96-nn`. The `Line` column is the line of the spec file where the `[V]` claim sits.
- Status: `confirmed` = repo value equals the spec value; `conflict` = repo value differs or the spec and repo disagree in formula or scope; `not found` = the repo has no encoding of the item (or the spec gives no value to compare).
- Repo roots (absolute): `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts`. Short aliases below are relative to it, using the common prefix `🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks` (EN 1992), `🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any` (EN 1995) and `🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any` (EN 1996).
- Aliases: `H92` = `…/⚖️checks/🧰️helpers-and-annex-parts/🦀️.rs`; `M92` = `…/⚖️checks/📋️member-evaluation/🏗️resistance-and-serviceability-checks/🦀️.rs`; `S92` = `…/⚖️checks/📋️member-evaluation/🧱️slenderness-prestress-and-ductility-checks/🦀️.rs`; `F92` = `…/⚖️checks/📋️member-evaluation/🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs`; `Snap92` = `🏛️en1992/…/🧬️schema/📸️snapshot/🦀️.rs`; `T95` = `🪵️en1995/…/🧬️schema/⚖️timber/🌲️material-and-annex-factors/🦀️.rs`; `S95` = `🪵️en1995/…/🧬️schema/⚖️timber/🏗️structure-and-members/🦀️.rs`; `F95` = `🪵️en1995/…/🧬️schema/⚖️timber/🔩️fasteners-and-johansen/🦀️.rs`; `O95` = `🪵️en1995/…/🔮️oracles/⚖️compliance/🐍️.py`; `F96` = `🪨️en1996/…/🧬️schema/⚖️masonry/📏️factors-and-geometry/🦀️.rs`; `Fire96` = `🪨️en1996/…/🧬️schema/⚖️masonry/🔥️fire-and-design-effects/🦀️.rs`; `Snap96` = `🪨️en1996/…/🧬️schema/📸️snapshot/🦀️.rs`; `EW96` = `🪨️en1996/…/🧬️schema/⚖️masonry/🧱️evaluate-wall/📏️wall-geometry-and-material/🦀️.rs`; `FD96` = `🪨️en1996/…/🧬️schema/⚖️masonry/🧱️evaluate-wall/🔥️fire-durability-and-reinforcement/🦀️.rs`; `O96` = `🪨️en1996/…/🔮️oracles/⚖️compliance/🐍️.py`.
- Kernel: `⚖️compliance/📜️annex-and-tables/🦀️.rs` only defines `AnnexChoice`, the `NationalAnnex` trait and interpolation helpers. It holds no NDP values.
- Research notes: `♻️mit-bestand` has no `recherche` folder and no EN 1992/1995/1996 NA document. The only hits are presentation PNGs. No NA PDF or markdown exists in the tracked files.

## 🚨 Conflict summary

- Total: 216 rows (86 EN 1992, 63 EN 1995, 67 EN 1996), of which 51 confirmed, 51 conflict, 114 not found. Per-spec counts are at the end of this file.
- The highest-impact conflicts are the German NA values that drive design results:
  - EN 1992 DE accidental `α_cc` and `γ_c`, DE `ν1`, DE `cot θ`, DE `v_min`, DE punching `C_Rd,c` (the repo itself contradicts itself, see E92-39 and E92-83), DE shear `k1`, `f_bd` without `α_ct/γ_c`, and the Table 5.5 fire beam values (E92-92).
  - EN 1995 DE `γ_M` (1.3 for all products instead of 1.25 / 1.2), `k_c,90` cap, `k_cr` for LVL, DE `f1` threshold (8 Hz in repo vs 6 Hz in spec), and the deflection limit `a` in both annexes.
  - EN 1995 strength fixtures: `ft,0,k`, `ft,90,k`, `f_c,0,k`, `f_c,90,k` and `G_mean` of C16, C24, C30, GL24h, GL28h and GL32h differ (E95-01 to E95-08). The oracle `O95` mirrors the repo values, so it cannot validate them.
  - EN 1996 `f_vk0`, `K` for clay / thin-layer mortar, `f_xk1`/`f_xk2`, `f_vlt` (0.045 vs 0.0117 `f_b`), `ρ2` for timber floors, DE `Φ1` (0.70 vs 0.85), the 4.5 m vs 4.2 m span threshold, and the REI fire thickness table.

## 🧱 EN 1992-1-1 (en1992-target.md)

| Id | Line | Item | Spec value | Repo value (file:line) | Status |
|---|---|---|---|---|---|
| E92-01 | 28 | `α_cc` DE persistent | 0.85 | `H92:78` | confirmed |
| E92-02 | 28 | `α_cc` accidental | 1.0 | DE accidental 0.85 `H92:89`; EN 1.0 `H92:90` | conflict |
| E92-03 | 28 | `α_ct` DE | 0.85 | `H92:78` | confirmed |
| E92-04 | 28 | `γ_c` DE persistent | 1.5 | `H92:78` | confirmed |
| E92-05 | 28 | `γ_c` accidental | 1.2 | DE accidental 1.3 `H92:89`; EN 1.2 `H92:90` | conflict |
| E92-06 | 50 (M-07) | DE factor 0.9 on `η·f_cd` when compression zone narrows | 0.9 | `H92:147-148` (η, λ only; no 0.9) | not found |
| E92-07 | 57 (M-09) | Prestress steel DE `ε_ud = 0.9·ε_uk` | 0.9·ε_uk | `S92:185` | confirmed |
| E92-08 | 57 (M-09) | Strand modulus `E_p` | 195 GPa | no strand modulus; only B500A `E_s` 200 GPa `Snap92:565` | not found |
| E92-09 | 71 (D-02) | Table E.1DE minimum strength classes per XC/XD/XS/XF/XA/XM | table rows | none | not found |
| E92-10 | 76 | DE `Δc,dur,γ`, `Δc,dur,st`, `Δc,dur,add` = 0 | 0 mm | additive `Δc,dur` absent; `H92:355-358` uses structural class only | not found |
| E92-11 | 77 | `c_min,dur` matrix S1..S6 | data (not stated) | `H92:268-281` S4 base only: X0 10, XC1 15, XC2/3 20, XC4 25, XD1/XS1 40, XD2/XS2 40, XD3/XS3 45, XF1 20, XF2/3 25, XF4 30, XA1 25 mm | not found |
| E92-12 | 81 (D-04) | `Δc,dev` DE 10 mm (5 mm with QC) | 10 / 5 mm | `Δc,dev` is an input (`H92:368-371`), no default value | not found |
| E92-13 | 81 | Structural class −1 if `f_ck ≥ C30/37` (XC1, Table 4.3N DE) | threshold C30/37 | `H92:348`: `f_ck ≥ 35 MPa` and only slab/flat slab/beam | conflict |
| E92-14 | 81 | Structural class +2 for design life 100 y | +2 | `H92:344` | confirmed |
| E92-15 | 119 (A-06) | Redistribution limits `k3`, `k4`, `k5`, `x_u/d` ≤ 0.45 / 0.35; DE `δ ≥ 0.64 + 0.8·x_u/d` | limits | no redistribution routine | not found |
| E92-16 | 123 (A-08) | DE `γ_O = 1.2` | 1.2 | none | not found |
| E92-17 | 127 (A-09) | DE `λ_lim = 25` for `n ≥ 0.41` | 25 | `H92:397-398` (`lambda_lim_de`) | confirmed |
| E92-18 | 141 (A-14) | DE `k1 = 0.31` (global wall stiffness) | 0.31 | none | not found |
| E92-19 | 176 (S-09) | DE `α_cc,pl = 0.70` (plain concrete) | 0.70 | none (no plain-concrete routine) | not found |
| E92-20 | 176 | DE minimum `h_w ≥ 120 mm` cast in place | 120 mm | none | not found |
| E92-21 | 182 (V-01) | DE restriction `a_v/d ≥ 0.5` for `β` | 0.5 | none | not found |
| E92-22 | 188 (V-02) | `C_Rd,c` shear DE | 0.15/γ_c | `H92:181` | confirmed |
| E92-23 | 188 | `C_Rd,c` shear EC | 0.18/γ_c | `H92:182` | confirmed |
| E92-24 | 188 | Shear `k1` DE (`σ_cp` term) | 0.12 (EC 0.15) | `H92:185` uses 0.15 for both annexes | conflict |
| E92-25 | 188, 240 | `v_min` DE `(0.0525/γ_c)k^1.5√f_ck` (d ≤ 600), `(0.0375/γ_c)` (d > 800) | DE NA form | `H92:184` and `H92:217` use `0.035·k^1.5·√f_ck` for both annexes, no d-dependence | conflict |
| E92-26 | 196 (V-03) | DE `cot θ ≤ (1.2 + 1.4·σ_cp/f_cd)/(1 − V_Rd,cc/V_Ed)`, ≤ 3 | 1.2 + 1.4 coefficient, `V_Rd,cc` term | `H92:107-113`: `clamp(1.2 + 0.2·σ_cp/f_cd, 1, 3)`, no `V_Rd,cc` | conflict |
| E92-27 | 196 | `V_Rd,cc = c·0.48·f_ck^(1/3)(1 − 1.2σ_cp/f_cd)·b_w·z`, `c = 0.5` | formula | none | not found |
| E92-28 | 197 (V-03) | DE `ρ_w,min = 0.16 f_ctm/f_yk` (beams), slab variant | formula | none | not found |
| E92-29 | 200 (V-04) | `α_cw` DE = 1.0 (σ_cp integrated) | 1.0 | `H92:200-203` alpha_cw = 1.0 for both annexes (EC σ_cp branch absent) | confirmed |
| E92-30 | 204 (V-05) | DE shear friction `c` rough 0.40, indented 0.50, `c = 0` fatigue | 0.40 / 0.50 | none | not found |
| E92-31 | 224 (T) | DE torsion interaction `T_Ed/T_Rd,max + V_Ed/V_Rd,max ≤ 1` | interaction | `H92:230` torsion only, no interaction check | not found |
| E92-32 | 230 (P-02) | Control perimeter `u_1` at `2d` | 2d | `H92:409-412` (`u_0 + 2π·2d`) | confirmed |
| E92-33 | 230 | `u_out` at `1.5d` (DE) | 1.5d | none | not found |
| E92-34 | 235 | DE punching `β` interior | 1.10 | `H92:407` (`_ => 1.10`) | confirmed |
| E92-35 | 235 | DE punching `β` edge | 1.40 | `H92:407` | confirmed |
| E92-36 | 235 | DE punching `β` corner | 1.50 | `H92:407` | confirmed |
| E92-37 | 235 | `β` for wall ends and corners (DE) | 1.35 | none (wall position not distinguished) | not found |
| E92-38 | 239 | DE `ρ_l ≤ 0.5 f_cd/f_yd` cap | cap | only `ρ_l ≤ 0.02` (`H92:211`) | not found |
| E92-39 | 240 | Punching `C_Rd,c` DE | 0.18/γ_c | `H92:214` gives 0.15/γ_c for DE; `M92:169` uses 0.18/γ_c for both annexes | conflict |
| E92-40 | 240 | Punching `k1` (EC 0.1 / DE 0.12, σ_cp term) | 0.1 / 0.12 | no σ_cp term in punching (`H92:217-218`) | not found |
| E92-41 | 240 | Punching `v_min` DE | DE NA form | `H92:217` 0.035 for both | conflict |
| E92-42 | 242 | Punching `v_Rd,max = 1.4·v_Rd,c` DE | 1.4 | `H92:224` | confirmed |
| E92-43 | 246 (P-04) | Punching with links `v_Rd,cs = 0.75·v_Rd,c + 1.5·(d/s_r)·A_sw·f_ywd,ef/(u_1·d)`, `f_ywd,ef = 250 + 0.25d` | formula | `M92:179`: `v_Rd,c + 0.75·A_sw·f_yd/(u_1·d)` | conflict |
| E92-44 | 249 (P-05) | DE stud approval `v_Rd,max` multipliers (1.96 double-headed) | multiplier | none (EN `2.0` only, `H92:225`) | not found |
| E92-45 | 261 (ST-03) | Node factors DE `k1`, `k2`, `k3` | from NA text | none | not found |
| E92-46 | 266 | Bearing `F_Rdu` with DE `α_cc = 0.85` and `3 f_cd A_c0` cap | formula | none | not found |
| E92-47 | 270 (B-01) | DE `f_ctd = α_ct·f_ctk,0.05/γ_c` in `f_bd` | α_ct / γ_c applied | `F92:64` and `H92:373` use `f_ctk,0.05` directly (= 0.7 f_ctm, `Snap92:505`), no α_ct, no γ_c | conflict |
| E92-48 | 270 | `η1` 0.7 poor / 1.0 good | 0.7 / 1.0 | `F92:61` | confirmed |
| E92-49 | 270 | `η2 = (132 − φ)/100` for φ > 32 | formula | `F92:62` | confirmed |
| E92-50 | 285 | DE lap percentages (50 % / 100 %) | % | none | not found |
| E92-51 | 303 (SL-02) | `σ_p` jacking 0.8 f_pk / 0.9 f_p0.1k | 0.8 / 0.9 | `S92:106` | confirmed |
| E92-52 | 303 | `σ_p` after transfer 0.75 f_pk / 0.85 f_p0.1k | 0.75 / 0.85 | `S92:107` | confirmed |
| E92-53 | 303 | `σ_p` after losses QP 0.65 f_pk | 0.65 | none (`S92:113` uses 0.85 f_p0.1k) | not found |
| E92-54 | 309 | DE `f_ct,eff = 0.5 f_ctm` (h ≤ 300) | 0.5 | none | not found |
| E92-55 | 311 (SL-05) | Table 7.2N / 7.3N (φ\*, spacing) | table | none | not found |
| E92-56 | 318 (SL-06) | `w_k,lim` DE 0.4 (XC1), 0.3 (others) | 0.4 / 0.3 | `H92:290-291` | confirmed |
| E92-57 | 318 | `w_k,lim` 0.2 prestressed / decompression | 0.2 | none | not found |
| E92-58 | 322 | DE slab `l/d` cap 35 / 150/l | cap | none (EN formula only, `H92:262-264`) | not found |
| E92-59 | 333 (F-03) | Fatigue `Δσ_Rsk = 162.5 MPa` (straight bars, N\* = 1e6) | 162.5 | `H92:565` `162.5e6 / 1.15` | conflict |
| E92-60 | 341 (PR-01) | `σ_p,max = min(k1 f_pk, k2 f_p0.1k)`, k1 0.8, k2 0.9 | 0.8 / 0.9 | `S92:106` | confirmed |
| E92-61 | 346 (PR-02) | DE friction `μ = 0.17..0.2`, `k = 0.005 rad/m` | μ, k | none | not found |
| E92-62 | 350 (PR-04) | DE `σ_p,m,t ≤ 0.65 f_pk` (QP) | 0.65 | none | not found |
| E92-63 | 351 (PR-06) | DE unbonded `Δσ_p,ULS` 50..100 MPa | 50..100 | none | not found |
| E92-64 | 352 (PR-07) | `r_sup/r_inf` 1.05/0.95 and 1.10/0.90 (DE) | factors | none | not found |
| E92-65 | 365 (DT-06) | DE `s_max = 250 mm` in cracked zones | 250 | none | not found |
| E92-66 | 369 (DT-10) | DE `A_s,v,min = 0.15 N_Ed/f_yd ≥ 0.0015 A_c` (unreinforced wall) | formula | none | not found |
| E92-67 | 373 (DT-14) | DE tie forces (`f_t = 20 kN`, peripheral ties) | forces | none | not found |
| E92-68 | 377 (R-01) | Spalling 10 mm for `f_ck ≥ 70` | 10 mm | none | not found |
| E92-69 | 380 (R-02) | `k_s(θ)` table (EN 1992-1-2 Table 3.2a) | table | none | not found |
| E92-70 | 385 (R-07) | `k_c(θ)` siliceous table (1.00 @ 20 °C … 0.00 @ 1200 °C) | table | none | not found |
| E92-71 | 390 (BR-01) | Bridge `α_cc = 0.85` (persistent) | 0.85 | `H92:78` (generic DE) | confirmed |
| E92-72 | 390 | DE NA 3.1.6 factor 0.8 long-term `α_cc` bridges with prestress | 0.8 | none | not found |
| E92-73 | 391 (BR-02) | Bridge `c_min,dur` S6 (100 y) and `Δc,dur,add` +10 mm XD3/XS3 | +10 mm | none | not found |
| E92-74 | 392 (BR-03) | DE bridges `cot θ ≤ 3.0` | 3.0 | `H92:111` | confirmed |
| E92-75 | 394 (BR-05) | Bridge `w_k` prestressed 0.2 mm / decompression; `f_ct,eff = max(f_ctm(3d), 3.0)` | values | none | not found |
| E92-76 | 401 (LQ-01) | Tightness class `w_k,lim` 0.2 (h_D/h ≤ 5) to 0.05 (h_D/h ≥ 35) | table | `Snap92` `TightnessClass` Tc0–Tc2 only, no limits | not found |
| E92-77 | 404 (LQ-04) | Self-healing `w_k ≤ 0.1 mm` (Table 7.105DE) | 0.1 | none | not found |
| E92-78 | 405 (LQ-05) | Liquid tank cover `+5 mm` | +5 mm | none | not found |
| E92-79 | 406 (LQ-06) | Tank compression zone `σ_c ≥ 1.0 MPa` | 1.0 | none (`M92`/`S92:137` decompression at 0.45 f_ck) | not found |
| E92-80 | 417 (FA-03) | Cone `k1`: post-installed cracked 7.7, uncracked 11.0; cast-in cracked 8.9, uncracked 12.7 | 7.7 / 8.9 cracked | `H92:601-603` `k = 7.5` cracked, 11.0 uncracked (single cone law) | conflict |
| E92-81 | 418 (FA-04) | Pull-out `k2 = 7.5` cracked, 10.5 uncracked | 7.5 / 10.5 | none (7.5 in H92 is the cone term) | not found |
| E92-82 | 562 | DE `ν1 = 0.75·ν2`, `ν2 = min(1.1 − f_ck/500, 1.0)` | 0.75·ν2 | `H92:200` `ν1 = 0.6(1 − f_ck/250)` for both annexes; no ν2 | conflict |
| E92-83 | 576 | Worked example `C_Rd,c = 0.18/1.5 = 0.12` | 0.12 | `M92:169` gives 0.12; `H92:214` gives 0.10 for DE | conflict |
| E92-84 | 577 | `v_Rd,max = 1.4·v_Rd,c = 1.031 MPa` | 1.4 | `H92:224` | confirmed |
| E92-85 | 593 | Node `k2` DE (0.75 vs EC 0.85) | 0.75 / 0.85 | none | not found |
| E92-86 | 616 | Table 5.5 simply supported beam, column 2: R30 80/15, R60 120/25, R90 150/35, R120 200/45 | 80/15 … 200/45 | `H92:445-448` `beam_ss` = 80/25, 120/40, 150/55, 200/65; `H92:451-454` `beam_cont` = 80/15 … 200/45 | conflict |

## ⚡ EN 1995-1-1 (en1995-en1996-target.md, timber)

| Id | Line | Item | Spec value | Repo value (file:line) | Status |
|---|---|---|---|---|---|
| E95-01 | 28 | C24 `f_t,0,k` | 14.5 | 14.0 `T95:42` | conflict |
| E95-02 | 28 | C24 `f_t,90,k` | 0.4 | 0.5 `T95:42` (also `O95:34`) | conflict |
| E95-03 | 28 | C24 `f_m, f_c,0, f_c,90, f_v, E_0,mean, E_0,05, E_90, G, ρ_k` | 24, 21, 2.5, 4.0, 11000, 7400, 370, 690, 350 | identical `T95:42` | confirmed |
| E95-04 | 28 | C16 `f_t,90,k` | 0.4 | 0.5 `T95:38` (also `O95:30`) | conflict |
| E95-05 | 28 | C30 `f_t,90,k` | 0.4 | 0.6 `T95:44` (also `O95:36`) | conflict |
| E95-06 | 28 | GL24h `f_t,0,k`, `f_t,90,k`, `f_c,90,k`, `G_mean` | 16.5, 0.4, 2.7, 650 | 19.2, 0.5, 2.5, 720 `T95:51` (also `O95:43`) | conflict |
| E95-07 | 28 | GL28h `f_t,0,k`, `f_t,90,k`, `f_c,0,k`, `f_c,90,k`, `G_mean` | 19.5, 0.45, 26.5, 3.0, 650 | 22.3, 0.5, 28, 2.5, 780 `T95:55` (also `O95:47`) | conflict |
| E95-08 | 28 | GL32h `f_t,0,k`, `f_c,0,k`, `f_c,90,k`, `G_mean` | 22.5, 29, 3.3, 650 | 25.6, 32, 2.5, 890 `T95:59` (also `O95:51`) | conflict |
| E95-09 | 35 | Particleboard P5/P7, fibreboard `k_mod` | Table 3.1 | `T95:269-282` solid / glulam / LVL / CLT only | not found |
| E95-10 | 35 | SC1/SC2 `k_mod` solid, glulam, LVL: 0.60 / 0.70 / 0.80 / 0.90 / 1.10; SC3 0.50 / 0.55 / 0.65 / 0.70 / 0.90 | values | `T95:271-279` | confirmed |
| E95-11 | 35 | OSB/3, OSB/4 `k_mod` | 0.40/0.50/0.70/0.90/1.10 | none | not found |
| E95-12 | 37 | NA Table NA.1 load-duration assignment (snow, wind, imposed A–E) | per-action durations | duration is input; default Medium `T95:240-244`; no NA table | not found |
| E95-13 | 43 | DE `γ_M` solid | 1.3 | 1.3 `T95:122` | confirmed |
| E95-14 | 43 | DE `γ_M` glulam | 1.25 | 1.3 `T95:122` (same for all products; also `O95:105`) | conflict |
| E95-15 | 43 | DE `γ_M` LVL | 1.2 | 1.3 `T95:122` | conflict |
| E95-16 | 43 | DE `γ_M` plywood / OSB | 1.2 | 1.3 `T95:122` | conflict |
| E95-17 | 43 | DE `γ_M` particleboard / fibreboard | 1.3 | 1.3 `T95:122` | confirmed |
| E95-18 | 43 | DE `γ_M` connections | 1.3 | 1.3 `T95:128-130` | confirmed |
| E95-19 | 43 | DE `γ_M` nail plates | 1.25 | none | not found |
| E95-20 | 43 | DE `γ_M` accidental 1.0 | 1.0 | `γ_M` has no accidental branch (`T95:120-127`) | not found |
| E95-21 | 43 | CLT `γ_M` DE | 1.25 | 1.3 `T95:122` | conflict |
| E95-22 | 48 | `k_def` solid / glulam / LVL SC1 0.60, SC2 0.80, SC3 2.00 | values | `T95:284-290` | confirmed |
| E95-23 | 48 | `k_def` panels (OSB, particleboard, MDF/HB) | values | none | not found |
| E95-24 | 51 | Connection `u_fin = u_inst(1 + 2·k_def)` for different creep | formula | `S95:274` uses `u_fin = u_inst(1 + k_def·ψ2)` only | not found |
| E95-25 | 64 | `E_d(ULS) = E_mean/γ_M` for second-order analysis | formula | none | not found |
| E95-26 | 92 | `k_c,90` values: 1.25 (conifer continuous), 1.5 (glulam continuous), 1.5 (solid discrete), 1.75 (glulam discrete) | table | `T95:359-365` geometric formula, capped at 1.5 | conflict |
| E95-27 | 92 | DE `k_c,90 = 1.0` for hardwood | 1.0 | none | not found |
| E95-28 | 105 | DE `k_cr = 2.5/f_v,k` (C24 0.625; GL28h 0.714) | formula | `T95:139`; 2.5/4.0 = 0.625 and 2.5/3.5 = 0.714 | confirmed |
| E95-29 | 105 | `k_cr = 1.0` for LVL | 1.0 | `T95:139` applies the formula to LVL (2.5/4.5 = 0.556) | conflict |
| E95-30 | 105 | `k_cr = 1.0` for panels | 1.0 | none (formula applied) | not found |
| E95-31 | 107 | `β = a/(2h)` reduction of loads within `2h` of support | formula | none (`T95:367-382` is the notch factor `k_v` only) | not found |
| E95-32 | 122 | Net section and staggering `a < 4d` | rules | none | not found |
| E95-33 | 123 | DE holes in glulam `d ≤ 0.15 h` at low shear | 0.15 h | none | not found |
| E95-34 | 136 | Slenderness `λ ≤ 150` general, 170 secondary (DE) | 150 / 170 | none | not found |
| E95-35 | 148 | `k_crit = 1` for continuous restraint (6.3.3(5)) | 1.0 | `T95:330-339` scales `M_crit` by `(span/l_ef)²`; `k_crit` not set to 1 | not found |
| E95-36 | 158 | Imperfection `e_0 = l_ef/300`, `L/400` solid, `L/500` glulam/LVL | values | none | not found |
| E95-37 | 184 | Ft,90 reinforcement `R_ax,d` (T-68) | rules | none | not found |
| E95-38 | 192 | DE notch α ≥ 0.5, `x ≤ 0.4 h`, `k_v` | limits | `T95:367-382` EN notch model with `k_n = 5` (`T95:377`); no DE limits | not found |
| E95-39 | 197 | `F_t,90,d = 1.3·V_d·[3(1−α)² − 2(1−α)³]` (DIN 1052 basis) | formula | none | not found |
| E95-40 | 203 | `k_sys = 1.1` load-sharing | 1.1 | none | not found |
| E95-41 | 216 | `w_inst ≤ L/300` | 300 | `T95:144-146` | confirmed |
| E95-42 | 216 | `w_net,fin ≤ L/200` (DE) | 200 | `T95:147-151` (DE 200) | confirmed |
| E95-43 | 216 | Roof members with small use `L/200` | 200 | none | not found |
| E95-44 | 226 (7.3.3) | DE `f_1 ≥ 6 Hz` residential; 6–8 Hz stiffness check | 6 Hz | `S95:372`, `S95:394` threshold 8 Hz for DE and EN | conflict |
| E95-45 | 231 | EN recommended `a = 1.5 mm/kN` | 1.5 | 1.7 `T95:163` (also `O95:476`) | conflict |
| E95-46 | 231 | DE `a = 0.5 mm/kN` | 0.5 | 1.5 `T95:162` (also `O95:476`) | conflict |
| E95-47 | 249 | Yield moment bolts / dowels / nails `0.3 f_u,k d^2.6` | 0.3 | `F95:36` | confirmed |
| E95-48 | 249 | Screws `M_y,Rk = 0.15 f_u,k d_ef^2.6`; staples `240 d^2.6`; square nails `0.45` | values | none | not found |
| E95-49 | 296 | Self-tapping screw approvals (`f_ax,k`, `ρ_k ≤ 440`) | ETA data | none | not found |
| E95-50 | 307 | DE splitting check, `k_e` factors | rules | none | not found |
| E95-51 | 315 | `k3 = min(1.75, ρ_k/350)`, `k4` 1.0 timber / 1.1 steel | values | none | not found |
| E95-52 | 319 | Spacing `a_1 = (1.2 + 0.8|cos α|)d` | formula | none | not found |
| E95-53 | 323 | Anchorage / plate net section, ETA | data | none | not found |
| E95-54 | 340 | Web buckling `h_w/b_w ≤ 70` | 70 | none | not found |
| E95-55 | 374 | Rolling shear `f_r,k = 1.0` (EN), 1.1 (DE, Z-9.1-559) | 1.0 / 1.1 | none | not found |
| E95-56 | 390 | Charring `β_n` solid softwood 0.80, glulam 0.70, LVL 0.70 mm/min | values | `T95:194-196` (converted to m/s) | confirmed |
| E95-57 | 390 | Charring `β_0` 0.65 for softwood, LVL, glulam; hardwood and panel values | values | `β_0` absent (only `β_n`) | not found |
| E95-58 | 391 | Panel density and thickness correction `k_ρ = √(450/ρ_k)`, `k_h = √(20/h_p)` | formulas | none | not found |
| E95-59 | 408 | Charring phases `k_2 = 2.0` | 2.0 | none | not found |
| E95-60 | 417 | Separating function `t_ins` / Annex C | data | none | not found |
| E95-61 | 429 | Bridge fatigue `k_fat` | formula | none | not found |
| E95-62 | 433 | Wheel print 400 × 400 mm, `k_dis` / prestress `σ_p,min ≥ 0.35` | values | none | not found |
| E95-63 | 865 | E12 curved apex factors `k_l`, `k_p`, `k_dis`, `k_vol` (oracle fixture) | values | none | not found |

## 🧱 EN 1996-1-1 (en1995-en1996-target.md, masonry)

| Id | Line | Item | Spec value | Repo value (file:line) | Status |
|---|---|---|---|---|---|
| E96-01 | 496 | Shape factor δ, example `h = 238, w = 240` | δ = 1.15 | formula ≈ 0.81 (length 240 assumed) `F96:82-90` | conflict |
| E96-02 | 498 | DE strength classes SFK 6..60, mean factor 1.2 / 1.25 | SFK | none | not found |
| E96-03 | 502 | DE mortar groups NM I, II, IIa, III, IIIa, DM, LM21, LM36 | fm 2.5 / 5 / 10 / 20 | `Snap96:327-335` EN M-classes only | not found |
| E96-04 | 503 | K clay G1 GP | 0.55 | 0.55 `F96:51` | confirmed |
| E96-05 | 503 | K clay G2 GP | 0.45 | 0.45 `F96:52` | confirmed |
| E96-06 | 503 | K clay G3 GP | 0.35 | 0.40 `F96:53` (oracle `O96:15` gives 0.35 for G4, repo 0.40) | conflict |
| E96-07 | 503 | K clay G4 GP | 0.35 | 0.40 `F96:53` (oracle 0.35) | conflict |
| E96-08 | 503 | K clay thin layer G1 | 0.75 | 0.80 for all materials `F96:48` | conflict |
| E96-09 | 503 | K clay thin layer G2 | 0.70 | 0.80 `F96:48` | conflict |
| E96-10 | 503 | K calcium silicate / concrete / AAC G1 GP | 0.55 | 0.55 `F96:54, 56, 57` | confirmed |
| E96-11 | 503 | K thin layer (calcium silicate G1) | 0.80 | 0.80 `F96:48` | confirmed |
| E96-12 | 503 | K lightweight mortar clay | 0.30 | none | not found |
| E96-13 | 503 | α / β GP 0.70 / 0.30; TL 0.85 / 0 | values | `F96:60`, `F96:48` | confirmed |
| E96-14 | 506 | Example `HLz 12 NM IIa ≈ 4.4 N/mm²` | 4.4 | not tabulated; formula only | not found |
| E96-15 | 512 | `f_vk0` GP mortar M10–M20 | 0.30 | 0.20 `F96:109` | conflict |
| E96-16 | 512 | `f_vk0` GP mortar M2.5–M9 | 0.20 | 0.15 (M5–M9) / 0.10 `F96:109, 112` | conflict |
| E96-17 | 512 | `f_vk0` GP mortar M1–M2 | 0.10 | 0.10 `F96:109` | confirmed |
| E96-18 | 512 | `f_vk0` thin layer | 0.30 | no thin-layer branch, mortar-strength law only `F96:105-116` | conflict |
| E96-19 | 512 | `f_vk0` lightweight / groups 2–4 | 0.15 | none | not found |
| E96-20 | 513 | Head joints `f_vk = 0.5 f_vk0 + 0.4 σ_d` | formula | none (bed joint only `F96:122-132`) | not found |
| E96-21 | 514, 593 | DE cap `f_vlt = 0.45·f_bt,cal·√(1 + σ/f_bt,cal)`, `f_bt,cal = 0.026 f_b` | ≈ 0.0117 f_b at σ = 0 | `F96:120` `F_VLT_OVER_FB_DE = 0.045·f_b`, applied `F96:128-129` | conflict |
| E96-22 | 518 | `f_xk1` clay fm < 5 | 0.10 | `0.020·fm` (0.05 at fm = 2.5) `F96:137` | conflict |
| E96-23 | 518 | `f_xk1` clay fm ≥ 5 | 0.10 | 0.10 `F96:137` | confirmed |
| E96-24 | 518 | `f_xk2` clay G1 0.20 / 0.40 | 0.20 / 0.40 | `min(0.20, 0.04·fm)` `F96:138`, caps at 0.20 | conflict |
| E96-25 | 518 | `f_xk1/f_xk2` calcium silicate 0.05 / 0.10 | 0.05 / 0.10 | 0.10 / 0.20 `F96:138` | conflict |
| E96-26 | 519 | DE tension-free `f_xk,1` (NCI) | rule | none | not found |
| E96-27 | 528 | `E = K_E·f_k`, `K_E = 1000` (EN) | 1000 | none | not found |
| E96-28 | 528 | DE `E = 950 f_k`, `G = 0.4 E` | 950 | none | not found |
| E96-29 | 529 | DE creep `φ_∞` single values (clay 1.0) | 1.0 | `φ_∞` is an input (`F96:265-272`), no table | not found |
| E96-30 | 535 | DE `γ_M` general (Class 1) | 1.5 | 1.5 `Snap96:266-276` (class 1) | confirmed |
| E96-31 | 535 | DE `γ_M` normal mortar, Cat II | 1.7 | 1.7 `Snap96:272-273` | confirmed |
| E96-32 | 535 | DE `γ_M` erection state | 1.3 | none | not found |
| E96-33 | 535 | DE `γ_M` accidental | 1.3 | 1.3 `Snap96:268` | confirmed |
| E96-34 | 535 | `γ_S = 1.15` reinforcement | 1.15 | none | not found |
| E96-35 | 535 | Accidental fire `γ_M` | 1.0 | none | not found |
| E96-36 | 535 | EN Table 2.3 by class: Cat I designed mortar 1.7, prescribed 2.0 (spec text) | 1.7 / 2.0 | `Snap96:254-262` 1.5 / 1.7 / 2.0 / 2.2 / 2.5 | conflict |
| E96-37 | 548 | `ρ_2 = 0.75` for monolithic concrete floors | 0.75 | `F96:169` | confirmed |
| E96-38 | 548 | `ρ_2 = 1.0` timber floors, `ρ_2 = 0.9` | 1.0 / 0.9 | `F96:169` always 0.75 | conflict |
| E96-39 | 551 | DE `ρ_n` for walls with openings (window ≥ 1/4) | rule | `F96:152-166` different model | not found |
| E96-40 | 556 | `h_ef/t_ef ≤ 27` load-bearing | 27 | `EW96:65`, `EW96:71` (27 for both annexes) | confirmed |
| E96-41 | 556 | `h_ef/t_ef ≤ 21` simplified (EN 1996-3) | 21 | none | not found |
| E96-42 | 592 | Flange effective width `≤ min(½ web spacing, 6t_f + t_w, 0.2 H)` | rule | none | not found |
| E96-43 | 593 | Overturning / sliding `0.9 G` (DE NA) | 0.9 | none | not found |
| E96-44 | 608 | Annex E coefficients `α_1`, `α_2`, `μ` for boundary cases A–J | tables | none | not found |
| E96-45 | 615 | Arching lateral capacity `q_lat,d = f_d (t/ℓ_a)²` (EN 6.17/6.18) | formula | none | not found |
| E96-46 | 624 | Basement wall `N_min = ρ_e h h_e²/(20 t)`, `h ≤ 2.6 m`, `t ≥ 240 mm` | formula / limits | none | not found |
| E96-47 | 648 | Ties per m² DE Table NA.13 (`n ≥ 7`) vs EN 2.5/m² | 7 / 2.5 | none | not found |
| E96-48 | 652 | Ring beam / floor tie force 30 kN | 30 kN | none | not found |
| E96-49 | 660 | DE frost / MX Table NA | table | none | not found |
| E96-50 | 664 | Reinforced masonry `(l/d)_max` 20 / 7 / 26 | 20 / 7 / 26 | none | not found |
| E96-51 | 671 | DE minimum wall thickness 115 mm | 115 mm | `FD96:240-241` | confirmed |
| E96-52 | 671 | DE pier area ≥ 0.10 m² (and 0.04 m² EN) | 0.10 / 0.04 | none | not found |
| E96-53 | 672 | Chase depth table (Table 8.1 NA) | table | none | not found |
| E96-54 | 678 | DE clear storey height `h ≤ 3.0 m` (2.75 m for t < 240) | 2.75 / 3.0 | `FD96:244-245` (3.0 m with t < 175 flagged) | conflict |
| E96-55 | 678 | Floor span `l_f ≤ 6.0 m` | 6.0 | `FD96:236` | confirmed |
| E96-56 | 684 | `Φ_s = 0.85 − 0.0011·(h_ef/t)²` EN | 0.85 | `F96:259, 262` | confirmed |
| E96-57 | 684 | DE `Φ_1 = 0.85 − 0.0011·(h_ef/t)²` | 0.85 | 0.70 `F96:260` (also `O96:33`) | conflict |
| E96-58 | 684 | `Φ_2 = 1.6 − l_f/6` used for spans > 4.2 m | 4.2 m | 4.5 m threshold `F96:231`, formula `F96:234` | conflict |
| E96-59 | 684 | `Φ_3 = 0.333` top storey (l_f > 6) | 0.333 | none | not found |
| E96-60 | 702 | REI clay G1 `t_min` 30 / 60 / 90 / 120 / 180 / 240 | 100 / 100 / 150 / 175 / 240 / 240 | `Fire96:9-14` 90 / 100 / 140 / 170 / 210 / 270 | conflict |
| E96-61 | 702 | REI 60 clay G1 | 100 | 100 `Fire96:10` | confirmed |
| E96-62 | 708 | 5 % rule for reduced strength `k_θ` (Annex C) | rule | none | not found |
| E96-63 | 863 | E11 floor `f_1 = 7.08 Hz` passes DE 6 Hz stage | 6 Hz | repo threshold 8 Hz `S95:372` | conflict |
| E96-64 | 883 | X8 yield-line `α = 0.04167` (isotropic square) | 0.04167 | none | not found |
| E96-65 | 887 | X10 `Φ_s = 0.7688` for `h_ef/t = 8.594` (EN 0.85) | 0.7688 | `F96:259, 262` gives 0.7688; `ρ_2 = 0.75` `F96:169` | confirmed |
| E96-66 | 889 | X11 `N_min = 24.66 kN/m` (basement) | formula | none | not found |
| E96-67 | 891 | X12 clay G1 REI90 `t_min ≤ 175 mm` (row 150 mm) | 150 / 175 | `Fire96:12-13`: 140 mm at α = 0.7 | conflict |

## 📊 Counts

- EN 1992: 86 rows, 23 confirmed, 15 conflict, 48 not found. Conflicts: E92-02, 05, 13, 24, 25, 26, 39, 41, 43, 47, 59, 80, 82, 83, 86.
- EN 1995: 63 rows, 11 confirmed, 16 conflict, 36 not found. Conflicts: E95-01, 02, 04, 05, 06, 07, 08, 14, 15, 16, 21, 26, 29, 44, 45, 46.
- EN 1996: 67 rows, 17 confirmed, 20 conflict, 30 not found. Conflicts: E96-01, 06, 07, 08, 09, 15, 16, 18, 21, 22, 24, 25, 36, 38, 54, 57, 58, 60, 63, 67.

## 🔎 Observations for the implementer

- Repo-internal contradiction: the punching `C_Rd,c` is 0.15/γ_c for DE in `H92:214` but 0.18/γ_c in `M92:169`. Pick one after reading the NA text (E92-39, E92-83).
- `H92:113` returns a constant 2.5 for EN `cot θ` instead of a free choice in [1, 2.5].
- `O95` and `O96` reproduce the Rust constants (C24 `f_t,90 = 0.5`, `γ_M = 1.3` for DE, `Φ_1 DE = 0.70`). They are not independent of the repo, so they cannot confirm the `[V]` values. The EN clay Group 4 `K` also differs between `O96:15` (0.35) and `F96:53` (0.40).
- `S95` and `T95` are internally consistent with each other. The `T95:139` `k_cr` applies the DE formula to LVL, which is inconsistent with the spec's `k_cr = 1.0` for LVL.
- The spec's own EN 14080 fixtures (E95-06 to E95-08) say they follow EN 14080:2013 and that the 2023 edition differs. Confirm the edition before changing values.
- Items marked `not found` are gaps in the implementation, not necessarily errors in the spec. Most are NA-only clauses (ties, chases, frost, anchorage, lap rules, fatigue, tank rules).
