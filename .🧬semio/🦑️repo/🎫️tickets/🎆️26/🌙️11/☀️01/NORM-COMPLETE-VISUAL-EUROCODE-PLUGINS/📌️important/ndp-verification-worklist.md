# NDP Verification Worklist

Every registry entry that is still `unverified`, grouped by standard, part and clause, so a human can walk the printed standard from front to back. Registry: `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`. Until a human sets `verified`, every check citing the id yields `UnverifiedBasis`, never a compliant verdict.

Totals: 2842 registry entries, 2653 unverified (listed below), 189 marked verified (section 3, re-check the flagged ones).

How to verify: open the printed standard or national annex at the clause heading, compare `EN` (recommended value) and `DE` (NA-DE value) with the print, then edit the entry in `🔣️.json` (`status: verified`, `source.edition`, `source.locator`, values) and regenerate the Rust rows with `python3 -I TICKET/🧰️ndp/codegen.py 🔣️.json 🗃️table.rs` (the Rust, Ajv and Python adoption tests keep both in sync). `-` means the annex gives no number (table or formula text, see the registry `asWritten`). Rows whose note reads `as used by the artifact` carry the number the code uses today; the spec line (`file:line` in `📐️specs/`) is given where one exists.

## 1. Conflicts first (repo and spec disagree)

Source: `🔍️audits/ndp-crosscheck-en1992-en1995-en1996.md` (aliases such as `H92:78` are defined there). Candidate A is the spec value, candidate B the value currently encoded in the repo (file:line).

| Marker | Cross-check id | Spec line | Item | Candidate A (spec) | Candidate B (repo, file:line) | Registry ids |
|---|---|---|---|---|---|---|
| conflict | E92-02 | 28 | `α_cc` accidental | 1.0 | DE accidental 0.85 `H92:89`; EN 1.0 `H92:90` | `ndp.en1992.m-01.alpha-cc-accidental`, `ndp.en1992.m-01.alpha-cc-persistent`, `ndp.en1992.m-01.alpha-ct`, `ndp.en1992.m-01.gamma-c-accidental`, `ndp.en1992.m-01.gamma-c-persistent` |
| conflict | E92-05 | 28 | `γ_c` accidental | 1.2 | DE accidental 1.3 `H92:89`; EN 1.2 `H92:90` | `ndp.en1992.m-01.alpha-cc-accidental`, `ndp.en1992.m-01.alpha-cc-persistent`, `ndp.en1992.m-01.alpha-ct`, `ndp.en1992.m-01.gamma-c-accidental`, `ndp.en1992.m-01.gamma-c-persistent` |
| conflict | E92-13 | 81 | Structural class −1 if `f_ck ≥ C30/37` (XC1, Table 4.3N DE) | threshold C30/37 | `H92:348`: `f_ck ≥ 35 MPa` and only slab/flat slab/beam | `ndp.en1992.d-04.1` |
| conflict | E92-24 | 188 | Shear `k1` DE (`σ_cp` term) | 0.12 (EC 0.15) | `H92:185` uses 0.15 for both annexes | `ndp.en1992.v-02.1` |
| conflict | E92-25 | 188, 240 | `v_min` DE `(0.0525/γ_c)k^1.5√f_ck` (d ≤ 600), `(0.0375/γ_c)` (d > 800) | DE NA form | `H92:184` and `H92:217` use `0.035·k^1.5·√f_ck` for both annexes, no d-dependence | `ndp.en1992.v-02.1` |
| conflict | E92-26 | 196 (V-03) | DE `cot θ ≤ (1.2 + 1.4·σ_cp/f_cd)/(1 − V_Rd,cc/V_Ed)`, ≤ 3 | 1.2 + 1.4 coefficient, `V_Rd,cc` term | `H92:107-113`: `clamp(1.2 + 0.2·σ_cp/f_cd, 1, 3)`, no `V_Rd,cc` | `ndp.en1992.v-03.1` |
| conflict | E92-39 | 240 | Punching `C_Rd,c` DE | 0.18/γ_c | `H92:214` gives 0.15/γ_c for DE; `M92:169` uses 0.18/γ_c for both annexes | `ndp.en1992.p-03.2` |
| conflict | E92-41 | 240 | Punching `v_min` DE | DE NA form | `H92:217` 0.035 for both | `ndp.en1992.p-03.2` |
| conflict | E92-43 | 246 (P-04) | Punching with links `v_Rd,cs = 0.75·v_Rd,c + 1.5·(d/s_r)·A_sw·f_ywd,ef/(u_1·d)`, `f_ywd,ef = 250 + 0.25d` | formula | `M92:179`: `v_Rd,c + 0.75·A_sw·f_yd/(u_1·d)` | `ndp.en1992.p-04.1` |
| conflict | E92-47 | 270 (B-01) | DE `f_ctd = α_ct·f_ctk,0.05/γ_c` in `f_bd` | α_ct / γ_c applied | `F92:64` and `H92:373` use `f_ctk,0.05` directly (= 0.7 f_ctm, `Snap92:505`), no α_ct, no γ_c | `ndp.en1992.b-01.1` |
| conflict | E92-59 | 333 (F-03) | Fatigue `Δσ_Rsk = 162.5 MPa` (straight bars, N\* = 1e6) | 162.5 | `H92:565` `162.5e6 / 1.15` | `ndp.en1992.f-03.1` |
| conflict | E92-80 | 417 (FA-03) | Cone `k1`: post-installed cracked 7.7, uncracked 11.0; cast-in cracked 8.9, uncracked 12.7 | 7.7 / 8.9 cracked | `H92:601-603` `k = 7.5` cracked, 11.0 uncracked (single cone law) | `ndp.en1992.fa-03.1` |
| conflict | E92-82 | 562 | DE `ν1 = 0.75·ν2`, `ν2 = min(1.1 − f_ck/500, 1.0)` | 0.75·ν2 | `H92:200` `ν1 = 0.6(1 − f_ck/250)` for both annexes; no ν2 | `ndp.en1992.e-03.1` |
| conflict | E92-83 | 576 | Worked example `C_Rd,c = 0.18/1.5 = 0.12` | 0.12 | `M92:169` gives 0.12; `H92:214` gives 0.10 for DE | `ndp.en1992.e-05.1` |
| conflict | E92-86 | 616 | Table 5.5 simply supported beam, column 2: R30 80/15, R60 120/25, R90 150/35, R120 200/45 | 80/15 … 200/45 | `H92:445-448` `beam_ss` = 80/25, 120/40, 150/55, 200/65; `H92:451-454` `beam_cont` = 80/15 … 200/45 | `ndp.en1992.e-12.1` |
| conflict | E95-01 | 28 | C24 `f_t,0,k` | 14.5 | 14.0 `T95:42` | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-02 | 28 | C24 `f_t,90,k` | 0.4 | 0.5 `T95:42` (also `O95:34`) | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-04 | 28 | C16 `f_t,90,k` | 0.4 | 0.5 `T95:38` (also `O95:30`) | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-05 | 28 | C30 `f_t,90,k` | 0.4 | 0.6 `T95:44` (also `O95:36`) | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-06 | 28 | GL24h `f_t,0,k`, `f_t,90,k`, `f_c,90,k`, `G_mean` | 16.5, 0.4, 2.7, 650 | 19.2, 0.5, 2.5, 720 `T95:51` (also `O95:43`) | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-07 | 28 | GL28h `f_t,0,k`, `f_t,90,k`, `f_c,0,k`, `f_c,90,k`, `G_mean` | 19.5, 0.45, 26.5, 3.0, 650 | 22.3, 0.5, 28, 2.5, 780 `T95:55` (also `O95:47`) | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-08 | 28 | GL32h `f_t,0,k`, `f_c,0,k`, `f_c,90,k`, `G_mean` | 22.5, 29, 3.3, 650 | 25.6, 32, 2.5, 890 `T95:59` (also `O95:51`) | `ndp.en1995.strength.c16.fc0k`, `ndp.en1995.strength.c16.fc90k`, `ndp.en1995.strength.c16.ft0k`, `ndp.en1995.strength.c16.ft90k`, `ndp.en1995.strength.c16.gmean`, `ndp.en1995.strength.c24.fc0k`, `ndp.en1995.strength.c24.fc90k`, `ndp.en1995.strength.c24.ft0k`, `ndp.en1995.strength.c24.ft90k`, `ndp.en1995.strength.c24.gmean`, `ndp.en1995.strength.c30.fc0k`, `ndp.en1995.strength.c30.fc90k`, `ndp.en1995.strength.c30.ft0k`, `ndp.en1995.strength.c30.ft90k`, `ndp.en1995.strength.c30.gmean`, `ndp.en1995.strength.gl24h.fc0k`, `ndp.en1995.strength.gl24h.fc90k`, `ndp.en1995.strength.gl24h.ft0k`, `ndp.en1995.strength.gl24h.ft90k`, `ndp.en1995.strength.gl24h.gmean`, `ndp.en1995.strength.gl28h.fc0k`, `ndp.en1995.strength.gl28h.fc90k`, `ndp.en1995.strength.gl28h.ft0k`, `ndp.en1995.strength.gl28h.ft90k`, `ndp.en1995.strength.gl28h.gmean`, `ndp.en1995.strength.gl32h.fc0k`, `ndp.en1995.strength.gl32h.fc90k`, `ndp.en1995.strength.gl32h.ft0k`, `ndp.en1995.strength.gl32h.ft90k`, `ndp.en1995.strength.gl32h.gmean` |
| conflict | E95-14 | 43 | DE `γ_M` glulam | 1.25 | 1.3 `T95:122` (same for all products; also `O95:105`) | `ndp.en1995.t-03.1` |
| conflict | E95-15 | 43 | DE `γ_M` LVL | 1.2 | 1.3 `T95:122` | `ndp.en1995.t-03.1` |
| conflict | E95-16 | 43 | DE `γ_M` plywood / OSB | 1.2 | 1.3 `T95:122` | `ndp.en1995.t-03.1` |
| conflict | E95-21 | 43 | CLT `γ_M` DE | 1.25 | 1.3 `T95:122` | `ndp.en1995.t-03.1` |
| conflict | E95-26 | 92 | `k_c,90` values: 1.25 (conifer continuous), 1.5 (glulam continuous), 1.5 (solid discrete), 1.75 (glulam discrete) | table | `T95:359-365` geometric formula, capped at 1.5 | `ndp.en1995.t-13.1` |
| conflict | E95-29 | 105 | `k_cr = 1.0` for LVL | 1.0 | `T95:139` applies the formula to LVL (2.5/4.5 = 0.556) | `ndp.en1995.t-15.1` |
| conflict | E95-44 | 226 (7.3.3) | DE `f_1 ≥ 6 Hz` residential; 6–8 Hz stiffness check | 6 Hz | `S95:372`, `S95:394` threshold 8 Hz for DE and EN | `ndp.en1995.t-63.1` |
| conflict | E95-45 | 231 | EN recommended `a = 1.5 mm/kN` | 1.5 | 1.7 `T95:163` (also `O95:476`) | `ndp.en1995.t-64.1` |
| conflict | E95-46 | 231 | DE `a = 0.5 mm/kN` | 0.5 | 1.5 `T95:162` (also `O95:476`) | `ndp.en1995.t-64.1` |
| conflict | E96-01 | 496 | Shape factor δ, example `h = 238, w = 240` | δ = 1.15 | formula ≈ 0.81 (length 240 assumed) `F96:82-90` | `ndp.en1996.m-01.1` |
| conflict | E96-06 | 503 | K clay G3 GP | 0.35 | 0.40 `F96:53` (oracle `O96:15` gives 0.35 for G4, repo 0.40) | `ndp.en1996.m-02.2` |
| conflict | E96-07 | 503 | K clay G4 GP | 0.35 | 0.40 `F96:53` (oracle 0.35) | `ndp.en1996.m-02.2` |
| conflict | E96-08 | 503 | K clay thin layer G1 | 0.75 | 0.80 for all materials `F96:48` | `ndp.en1996.m-02.2` |
| conflict | E96-09 | 503 | K clay thin layer G2 | 0.70 | 0.80 `F96:48` | `ndp.en1996.m-02.2` |
| conflict | E96-15 | 512 | `f_vk0` GP mortar M10–M20 | 0.30 | 0.20 `F96:109` | `ndp.en1996.m-03.1` |
| conflict | E96-16 | 512 | `f_vk0` GP mortar M2.5–M9 | 0.20 | 0.15 (M5–M9) / 0.10 `F96:109, 112` | `ndp.en1996.m-03.1` |
| conflict | E96-18 | 512 | `f_vk0` thin layer | 0.30 | no thin-layer branch, mortar-strength law only `F96:105-116` | `ndp.en1996.m-03.1` |
| conflict | E96-21 | 514, 593 | DE cap `f_vlt = 0.45·f_bt,cal·√(1 + σ/f_bt,cal)`, `f_bt,cal = 0.026 f_b` | ≈ 0.0117 f_b at σ = 0 | `F96:120` `F_VLT_OVER_FB_DE = 0.045·f_b`, applied `F96:128-129` | `ndp.en1996.m-03.3` |
| conflict | E96-22 | 518 | `f_xk1` clay fm < 5 | 0.10 | `0.020·fm` (0.05 at fm = 2.5) `F96:137` | `ndp.en1996.m-04.1` |
| conflict | E96-24 | 518 | `f_xk2` clay G1 0.20 / 0.40 | 0.20 / 0.40 | `min(0.20, 0.04·fm)` `F96:138`, caps at 0.20 | `ndp.en1996.m-04.1` |
| conflict | E96-25 | 518 | `f_xk1/f_xk2` calcium silicate 0.05 / 0.10 | 0.05 / 0.10 | 0.10 / 0.20 `F96:138` | `ndp.en1996.m-04.1` |
| conflict | E96-36 | 535 | EN Table 2.3 by class: Cat I designed mortar 1.7, prescribed 2.0 (spec text) | 1.7 / 2.0 | `Snap96:254-262` 1.5 / 1.7 / 2.0 / 2.2 / 2.5 | `ndp.en1996.m-07.1` |
| conflict | E96-38 | 548 | `ρ_2 = 1.0` timber floors, `ρ_2 = 0.9` | 1.0 / 0.9 | `F96:169` always 0.75 | `ndp.en1996.m-10.1` |
| conflict | E96-54 | 678 | DE clear storey height `h ≤ 3.0 m` (2.75 m for t < 240) | 2.75 / 3.0 | `FD96:244-245` (3.0 m with t < 175 flagged) | `ndp.en1996.m-80.1` |
| conflict | E96-57 | 684 | DE `Φ_1 = 0.85 − 0.0011·(h_ef/t)²` | 0.85 | 0.70 `F96:260` (also `O96:33`) | `ndp.en1996.m-81.1` |
| conflict | E96-58 | 684 | `Φ_2 = 1.6 − l_f/6` used for spans > 4.2 m | 4.2 m | 4.5 m threshold `F96:231`, formula `F96:234` | `ndp.en1996.m-81.1` |
| conflict | E96-60 | 702 | REI clay G1 `t_min` 30 / 60 / 90 / 120 / 180 / 240 | 100 / 100 / 150 / 175 / 240 / 240 | `Fire96:9-14` 90 / 100 / 140 / 170 / 210 / 270 | `ndp.en1996.m-90.1` |
| conflict | E96-63 | 863 | E11 floor `f_1 = 7.08 Hz` passes DE 6 Hz stage | 6 Hz | repo threshold 8 Hz `S95:372` | `ndp.en1995.e3-bearing-perpendicular.1` |
| conflict | E96-67 | 891 | X12 clay G1 REI90 `t_min ≤ 175 mm` (row 150 mm) | 150 / 175 | `Fire96:12-13`: 140 mm at α = 0.7 | `ndp.en1996.x1-fk-for-clay-group-1-g.4` |

## 1b. Policy placeholders (no normative value exists in the repo spec)

These ids are modelling choices of an artifact owner, not values of any standard. Replace them with the printed wording or remove the feature.

- `ndp.en1990.a1.override.gamma.allowed` (EN 1, DE 0): POLICY PLACEHOLDER (unverified): EN 1990 Annex A1/A2 sets gamma_G, gamma_Q as NDPs of the National Annex; EN: allowed (placeholder), DE: not allowed (placeholder)
- `ndp.en1990.a1.override.k_fi.allowed` (EN 0, DE 0): POLICY PLACEHOLDER (unverified): EN 1990 Table B3 K_FI follows the reliability class (spec l.48: K_FI(RC1..3) = 0.9/1.0/1.1); override not allowed in EN and DE (placeholder)
- `ndp.en1990.a1.override.psi.allowed` (EN 1, DE 1): POLICY PLACEHOLDER (unverified): EN 1990 Table A1.1 psi values are NDPs; project override with justification: allowed in EN and DE (placeholder)
- `ndp.en1990.a1.override.xi.allowed` (EN 1, DE 0): POLICY PLACEHOLDER (unverified): EN 1990 Table A1.2(B) xi is an NDP; EN: allowed (placeholder), DE: not allowed (placeholder)

## 2. All unverified entries by standard, part and clause

### EN 1990 (260)

#### EN 1990, part not stated (260)

##### Clause 1.3.1 (4; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l167.1` | NA.A1.3.1 |  | - | - | 1.3.1 | en1990-en1991-target.md:167: `NA.A1.3.1` γ values (DE uses 1.35/1.0/1.5, EQU 1.1/0.9/1.5 ) |
| `l167.2` | NA.A2.x |  | - | - | 1.3.1 | en1990-en1991-target.md:167: bridges `NA.A2.x` temperature ψ0 0.8 |
| `l167.3` | NA.B |  | - | - | 1.3.1 | en1990-en1991-target.md:167: `NA.B` reliability tables unchanged |
| `l167.4` | NA.2.1 |  | - | - | 1.3.1 | en1990-en1991-target.md:167: Brandfall ψ1,1 . |

##### Clause 1.5 (5; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-24.1` | ? |  | - | - | 1.5 | en1990-en1991-target.md:123: Limit table is a data asset `SlsCriterion { id, element, family(char\|freq\|qp), quantity, limit_expr(L/250), source }` . |
| `e90-24.2` | ? |  | - | - | 1.5 | en1990-en1991-target.md:123: Vibration: floors: f1 ≥ 3 Hz for gym/dance (EN 1990 A1.4.4) |
| `e90-24.3` | ? |  | - | - | 1.5 | en1990-en1991-target.md:123: NA-DE: f1,min ≥ 3 Hz residential walking |
| `e90-24.4` | ? |  | - | - | 1.5 | en1990-en1991-target.md:123: Horizontal displacement: u ≤ H/500 per structure and u_i ≤ h_i/300 storey (guidance, EN A1.4.2(1)(2)) . out: utilisation |
| `na-de.1` | valid_for_category |  | - | - | 1.5 | en1990-en1991-target.md:118: NA-DE differences : ψ values by category match EN recommended for A-H |

##### Clause 1.05 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-26.1` | ? |  | - | - | 1.05 | en1990-en1991-target.md:145: f: EQU: γG,sup 1.05, γG,inf 0.95 (A2.4(A), EN rec) but NA-DE uses 1.05/0.95 |

##### Clause 1.6 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-25.1` | ? |  | - | - | 1.6 | en1990-en1991-target.md:140: 0.6 (DE bridges 0.8 ) |

##### Clause 1.7 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-28.1` | ? |  | - | - | 1.7 | en1990-en1991-target.md:155: NDP: NA-DE: β_target(50 yr, RC2)=3.8 , same table. out: reliability status and K_FI recommendation (E90.02). |

##### Clause 4.2 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e12-07.1` | ? |  | - | - | 4.2 | en1990-en1991-target.md:208: ψ_fi = ψ1,1 EN, DE: ψ1,1 ) with simplification E_fi,d = η_fi E_d. out: E_fi,d (with η_fi) |

##### Clause 5.1 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-12.alpha_cr_first_order` | α_cr first-order limit |  | 10 | 10 | 5.1 |  |

##### Clause 6.4.4 EN 1991-2 (8; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.f_lower_a` | n0 lower limit coefficient |  | 23.58 | 23.58 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_lower_b` | n0 lower limit exponent |  | -0.592 | -0.592 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_lower_break` | lower limit break span | m | 20 | 20 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_lower_short` | n0 lower limit, short spans | m/s | 80 | 80 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_range_max` | span range max | m | 100 | 100 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_range_min` | span range min | m | 4 | 4 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_upper_a` | n0 upper limit coefficient |  | 94.76 | 94.76 | 6.4.4 EN 1991-2 |  |
| `a2.rail.f_upper_b` | n0 upper limit exponent |  | -0.748 | -0.748 | 6.4.4 EN 1991-2 |  |

##### Clause 6.4.6.3 EN 1991-2 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.damping_default` | default damping ratio |  | 0.01 | 0.01 | 6.4.6.3 EN 1991-2 |  |

##### Clause 6.7 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-15.1` | ? |  | - | - | 6.7 | en1990-en1991-target.md:83: 0 (permanent partly favourable: when both favourable and unfavourable parts of the same action exist the same γ applies to |

##### Clause 6.10 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-16.1` | ExclusiveGroup |  | - | - | 6.10 | en1990-en1991-target.md:86: (6.10a)/(6.10b) are NOT offered in the NA-DE annex |
| `e90-16.2` | ExclusiveGroup |  | - | - | 6.10 | en1990-en1991-target.md:86: DE simplified rule NA to 6.4.3.2 : for buildings with more than one variable action Ed = 1.35 ΣG + 1.35 ΣQ is permitted as |

##### Clause A1.1 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a1.override.psi.allowed` | project ψ override allowed |  | 1 | 1 | A1.1 | POLICY PLACEHOLDER (unverified): EN 1990 Table A1.1 psi values are NDPs; project override with justification: allowed in EN and DE (placeholder): 1 = |

##### Clause A1.3 (3; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a1.override.gamma.allowed` | project γ override allowed |  | 1 | 0 | A1.3 | POLICY PLACEHOLDER (unverified): EN 1990 Annex A1/A2 sets gamma_G, gamma_Q as NDPs of the National Annex; EN: allowed (placeholder), DE: not allowed ( |
| `a1.override.k_fi.allowed` | project K_FI override allowed |  | 0 | 0 | A1.3 | POLICY PLACEHOLDER (unverified): EN 1990 Table B3 K_FI follows the reliability class (spec l.48: K_FI(RC1..3) = 0.9/1.0/1.1); override not allowed in |
| `a1.override.xi.allowed` | project ξ override allowed |  | 1 | 0 | A1.3 | POLICY PLACEHOLDER (unverified): EN 1990 Table A1.2(B) xi is an NDP; EN: allowed (placeholder), DE: not allowed (placeholder): 1 = a project may repla |

##### Clause A1.3.1 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gP_inf` | γP,favourable |  | 1 | 1 | A1.3.1 |  |
| `gP_sup` | γP,unfavourable |  | 1 | 1 | A1.3.1 |  |

##### Clause A1.4.2 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sls.w3.ratio` | sls.w3.ratio |  | 300 | 300 | A1.4.2 |  |
| `sls.w_net.ratio` | sls.w_net.ratio |  | 250 | 250 | A1.4.2 |  |

##### Clause A1.4.3 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sls.frame_drift.ratio` | sls.frame_drift.ratio |  | 500 | 500 | A1.4.3 |  |
| `sls.storey_drift.ratio` | sls.storey_drift.ratio |  | 300 | 300 | A1.4.3 |  |

##### Clause A1.4.4 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sls.floor_frequency_hz` | sls.floor_frequency_hz | Hz | 3 | 3 | A1.4.4 |  |

##### Clause A2.2 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-25.2` | ? |  | - | - | A2.2 | en1990-en1991-target.md:143: Footbridge (A2.2): Q_fwk crowd 0/0.4/0, Q_serv 0/0.4? . |

##### Clause A2.2.2 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.fw.speed_cap` | F*_W wind speed cap | m/s | 35 | 35 | A2.2.2 |  |
| `a2.wind_thermal.simultaneous` | wind and thermal simultaneous |  | 0 | 0 | A2.2.2 |  |

##### Clause A2.4.3.2 (4; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.footbridge.acc_h` | a2.footbridge.acc_h | m/s2 | 0.2 | 0.2 | A2.4.3.2 |  |
| `a2.footbridge.acc_h_crowd` | a2.footbridge.acc_h_crowd | m/s2 | 0.4 | 0.4 | A2.4.3.2 |  |
| `a2.footbridge.acc_v` | a2.footbridge.acc_v | m/s2 | 0.7 | 0.7 | A2.4.3.2 |  |
| `a2.footbridge.lock_in` | a2.footbridge.lock_in | m/s2 | 0.1 | 0.1 | A2.4.3.2 |  |

##### Clause A2.4.4.2.1 (4; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.acc_ballast` | a2.rail.acc_ballast | m/s2 | 3.5 | 3.5 | A2.4.4.2.1 |  |
| `a2.rail.acc_slab` | a2.rail.acc_slab | m/s2 | 5 | 5 | A2.4.4.2.1 |  |
| `a2.rail.dynamic_speed` | dynamic analysis speed | km/h | 200 | 200 | A2.4.4.2.1 |  |
| `a2.rail.speed_margin` | verification speed factor |  | 1.2 | 1.2 | A2.4.4.2.1 |  |

##### Clause A2.4.4.2.2 (6; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.twist_120` | a2.rail.twist_120 | mm/3m | 4.5 | 4.5 | A2.4.4.2.2 |  |
| `a2.rail.twist_200` | a2.rail.twist_200 | mm/3m | 3 | 3 | A2.4.4.2.2 |  |
| `a2.rail.twist_length` | twist base length | m | 3 | 3 | A2.4.4.2.2 |  |
| `a2.rail.twist_max` | a2.rail.twist_max | mm/3m | 1.5 | 1.5 | A2.4.4.2.2 |  |
| `a2.rail.twist_speed_1` | v1 twist band | km/h | 120 | 120 | A2.4.4.2.2 |  |
| `a2.rail.twist_speed_2` | v2 twist band | km/h | 200 | 200 | A2.4.4.2.2 |  |

##### Clause A2.4.4.2.3 (3; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.end_rotation_high` | end rotation, v > band | rad | 0.005 | 0.005 | A2.4.4.2.3 |  |
| `a2.rail.end_rotation_low` | end rotation, v <= band | rad | 0.0065 | 0.0065 | A2.4.4.2.3 |  |
| `a2.rail.end_rotation_speed` | end rotation speed band | km/h | 160 | 160 | A2.4.4.2.3 |  |

##### Clause A2.4.4.3 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.deflection.ratio` | a2.rail.deflection.ratio |  | 600 | 600 | A2.4.4.3 |  |

##### Clause Annex E EN 1991-2 (40; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a2.rail.hslm.a1.axle_load` | HSLM-A1 axle load P | N | 170000 | 170000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a1.axle_spacing` | HSLM-A1 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a1.coach_length` | HSLM-A1 coach length D | m | 18 | 18 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a1.coaches` | HSLM-A1 coaches |  | 18 | 18 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a10.axle_load` | HSLM-A10 axle load P | N | 210000 | 210000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a10.axle_spacing` | HSLM-A10 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a10.coach_length` | HSLM-A10 coach length D | m | 27 | 27 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a10.coaches` | HSLM-A10 coaches |  | 11 | 11 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a2.axle_load` | HSLM-A2 axle load P | N | 200000 | 200000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a2.axle_spacing` | HSLM-A2 axle spacing d | m | 3.5 | 3.5 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a2.coach_length` | HSLM-A2 coach length D | m | 19 | 19 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a2.coaches` | HSLM-A2 coaches |  | 17 | 17 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a3.axle_load` | HSLM-A3 axle load P | N | 180000 | 180000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a3.axle_spacing` | HSLM-A3 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a3.coach_length` | HSLM-A3 coach length D | m | 20 | 20 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a3.coaches` | HSLM-A3 coaches |  | 16 | 16 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a4.axle_load` | HSLM-A4 axle load P | N | 190000 | 190000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a4.axle_spacing` | HSLM-A4 axle spacing d | m | 3 | 3 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a4.coach_length` | HSLM-A4 coach length D | m | 21 | 21 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a4.coaches` | HSLM-A4 coaches |  | 15 | 15 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a5.axle_load` | HSLM-A5 axle load P | N | 170000 | 170000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a5.axle_spacing` | HSLM-A5 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a5.coach_length` | HSLM-A5 coach length D | m | 22 | 22 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a5.coaches` | HSLM-A5 coaches |  | 14 | 14 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a6.axle_load` | HSLM-A6 axle load P | N | 180000 | 180000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a6.axle_spacing` | HSLM-A6 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a6.coach_length` | HSLM-A6 coach length D | m | 23 | 23 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a6.coaches` | HSLM-A6 coaches |  | 13 | 13 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a7.axle_load` | HSLM-A7 axle load P | N | 190000 | 190000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a7.axle_spacing` | HSLM-A7 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a7.coach_length` | HSLM-A7 coach length D | m | 24 | 24 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a7.coaches` | HSLM-A7 coaches |  | 13 | 13 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a8.axle_load` | HSLM-A8 axle load P | N | 190000 | 190000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a8.axle_spacing` | HSLM-A8 axle spacing d | m | 2.5 | 2.5 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a8.coach_length` | HSLM-A8 coach length D | m | 25 | 25 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a8.coaches` | HSLM-A8 coaches |  | 12 | 12 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a9.axle_load` | HSLM-A9 axle load P | N | 210000 | 210000 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a9.axle_spacing` | HSLM-A9 axle spacing d | m | 2 | 2 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a9.coach_length` | HSLM-A9 coach length D | m | 26 | 26 | Annex E EN 1991-2 |  |
| `a2.rail.hslm.a9.coaches` | HSLM-A9 coaches |  | 11 | 11 | Annex E EN 1991-2 |  |

##### Clause C.7 (4; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `form.alpha_e` | form.alpha_e |  | -0.7 | -0.7 | C.7 |  |
| `form.alpha_r` | form.alpha_r |  | 0.8 | 0.8 | C.7 |  |
| `form.ratio_hi` | form.ratio_hi |  | 7.6 | 7.6 | C.7 |  |
| `form.ratio_lo` | form.ratio_lo |  | 0.16 | 0.16 | C.7 |  |

##### Clause E16.03 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e16-03.1` | ? |  | - | - | E16.03 | en1990-en1991-target.md:305: E16.04 accidental: falling load, impact of crane etc. |
| `e16-03.2` | ? |  | - | - | E16.03 | en1990-en1991-target.md:305: E16.05 4.9 / geometric imperfections during execution (tolerance Δ) and E16.06 Annex A.2 / Combination of construction lo |

##### Clause E90.27 (3; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-27.1` | BridgeSls |  | - | - | E90.27 | en1990-en1991-target.md:146: DE: deflection from LM1 frequent ≤ L/1000? |
| `e90-27.2` | BridgeSls |  | - | - | E90.27 | en1990-en1991-target.md:146: deck twist t ≤ 4.5 mm/3 m (v ≤ 120 km/h), 3.0 (120 < v ≤ 200), 1.5 (> 200) |
| `e90-27.3` | BridgeSls |  | - | - | E90.27 | en1990-en1991-target.md:146: rail bridge deflection limit L/600 (comfort level "very good") . out: utilisation = value / limit. ok ≤ 1. |

##### Clause EN 1990 6.3.5 (2; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gM.other` | gM.other |  | 1 | 1 | EN 1990 6.3.5 |  |
| `gR` | gR |  | 1 | 1 | EN 1990 6.3.5 |  |

##### Clause EN 1992-1-1 2.4.2.4 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gM.concrete` | gM.concrete |  | 1.5 | 1.5 | EN 1992-1-1 2.4.2.4 |  |

##### Clause EN 1993-1-1 6.1 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gM.steel` | gM.steel |  | 1 | 1 | EN 1993-1-1 6.1 |  |

##### Clause EN 1995-1-1 2.4.1 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gM.timber` | gM.timber |  | 1.3 | 1.3 | EN 1995-1-1 2.4.1 |  |

##### Clause NA.A.1.3.1 (1; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `simplified.gq` | simplified.gq |  | 1.35 | 1.35 | NA.A.1.3.1 |  |

##### Clause Table A1.1 (37; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi.congregation.0` | ψ0 congregation |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.congregation.1` | ψ1 congregation |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.congregation.2` | ψ2 congregation |  | 0.6 | 0.6 | Table A1.1 |  |
| `psi.office.0` | ψ0 office |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.office.1` | ψ1 office |  | 0.5 | 0.5 | Table A1.1 |  |
| `psi.office.2` | ψ2 office |  | 0.3 | 0.3 | Table A1.1 |  |
| `psi.residential.0` | ψ0 residential |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.residential.1` | ψ1 residential |  | 0.5 | 0.5 | Table A1.1 |  |
| `psi.residential.2` | ψ2 residential |  | 0.3 | 0.3 | Table A1.1 |  |
| `psi.retail.0` | ψ0 retail |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.retail.1` | ψ1 retail |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.retail.2` | ψ2 retail |  | 0.6 | 0.6 | Table A1.1 |  |
| `psi.roof.0` | ψ0 roof |  | 0 | 0 | Table A1.1 |  |
| `psi.roof.1` | ψ1 roof |  | 0 | 0 | Table A1.1 |  |
| `psi.roof.2` | ψ2 roof |  | 0 | 0 | Table A1.1 |  |
| `psi.snow.0` | ψ0 snow |  | 0.5 | 0.5 | Table A1.1 |  |
| `psi.snow.1` | ψ1 snow |  | 0.2 | 0.2 | Table A1.1 |  |
| `psi.snow.2` | ψ2 snow |  | 0 | 0 | Table A1.1 |  |
| `psi.snow_high.0` | ψ0 snow_high |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.snow_high.1` | ψ1 snow_high |  | 0.5 | 0.5 | Table A1.1 |  |
| `psi.snow_high.2` | ψ2 snow_high |  | 0.2 | 0.2 | Table A1.1 |  |
| `psi.storage.0` | ψ0 storage |  | 1 | 1 | Table A1.1 |  |
| `psi.storage.1` | ψ1 storage |  | 0.9 | 0.9 | Table A1.1 |  |
| `psi.storage.2` | ψ2 storage |  | 0.8 | 0.8 | Table A1.1 |  |
| `psi.temperature.0` | ψ0 temperature |  | 0.6 | 0.6 | Table A1.1 |  |
| `psi.temperature.1` | ψ1 temperature |  | 0.5 | 0.5 | Table A1.1 |  |
| `psi.temperature.2` | ψ2 temperature |  | 0 | 0 | Table A1.1 |  |
| `psi.traffic_heavy.0` | ψ0 traffic_heavy |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.traffic_heavy.1` | ψ1 traffic_heavy |  | 0.5 | 0.5 | Table A1.1 |  |
| `psi.traffic_heavy.2` | ψ2 traffic_heavy |  | 0.3 | 0.3 | Table A1.1 |  |
| `psi.traffic_light.0` | ψ0 traffic_light |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.traffic_light.1` | ψ1 traffic_light |  | 0.7 | 0.7 | Table A1.1 |  |
| `psi.traffic_light.2` | ψ2 traffic_light |  | 0.6 | 0.6 | Table A1.1 |  |
| `psi.wind.0` | ψ0 wind |  | 0.6 | 0.6 | Table A1.1 |  |
| `psi.wind.1` | ψ1 wind |  | 0.2 | 0.2 | Table A1.1 |  |
| `psi.wind.2` | ψ2 wind |  | 0 | 0 | Table A1.1 |  |
| `snow_altitude_m` | snow_altitude_m | m | 1000 | 1000 | Table A1.1 |  |

##### Clause Table A1.2(A) (3; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.equ.A1` | gG_inf.equ.A1 |  | 0.9 | 0.9 | Table A1.2(A) |  |
| `gG_sup.equ.A1` | gG_sup.equ.A1 |  | 1.1 | 1.1 | Table A1.2(A) |  |
| `gQ.equ.A1` | gQ.equ.A1 |  | 1.5 | 1.5 | Table A1.2(A) |  |

##### Clause Table A1.2(B) (4; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.str.A1` | gG_inf.str.A1 |  | 1 | 1 | Table A1.2(B) |  |
| `gG_sup.str.A1` | gG_sup.str.A1 |  | 1.35 | 1.35 | Table A1.2(B) |  |
| `gQ.str.A1` | gQ.str.A1 |  | 1.5 | 1.5 | Table A1.2(B) |  |
| `xi.str.A1` | xi.str.A1 |  | 0.85 | 0.85 | Table A1.2(B) |  |

##### Clause Table A1.2(B) via GEO-2 (DA2*) (3; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.geo2.A1` | gG_inf.geo2.A1 |  | 1 | 1 | Table A1.2(B) via GEO-2 (DA2*) |  |
| `gG_sup.geo2.A1` | gG_sup.geo2.A1 |  | 1.35 | 1.35 | Table A1.2(B) via GEO-2 (DA2*) |  |
| `gQ.geo2.A1` | gQ.geo2.A1 |  | 1.5 | 1.5 | Table A1.2(B) via GEO-2 (DA2*) |  |

##### Clause Table A1.2(C) (3; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.geo.A1` | gG_inf.geo.A1 |  | 1 | 1 | Table A1.2(C) |  |
| `gG_sup.geo.A1` | gG_sup.geo.A1 |  | 1 | 1 | Table A1.2(C) |  |
| `gQ.geo.A1` | gQ.geo.A1 |  | 1.3 | 1.3 | Table A1.2(C) |  |

##### Clause Table A2.1 (39; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi.construction_load.0` | ψ0 construction_load |  | 1 | 1 | Table A2.1 |  |
| `psi.construction_load.1` | ψ1 construction_load |  | 0 | 0 | Table A2.1 |  |
| `psi.construction_load.2` | ψ2 construction_load |  | 1 | 1 | Table A2.1 |  |
| `psi.road_gr3_pedestrian.0` | ψ0 road_gr3_pedestrian |  | 0 | 0 | Table A2.1 |  |
| `psi.road_gr3_pedestrian.1` | ψ1 road_gr3_pedestrian |  | 0.4 | 0.4 | Table A2.1 |  |
| `psi.road_gr3_pedestrian.2` | ψ2 road_gr3_pedestrian |  | 0 | 0 | Table A2.1 |  |
| `psi.road_gr4_crowd.0` | ψ0 road_gr4_crowd |  | 0 | 0 | Table A2.1 |  |
| `psi.road_gr4_crowd.1` | ψ1 road_gr4_crowd |  | 0.75 | 0.75 | Table A2.1 |  |
| `psi.road_gr4_crowd.2` | ψ2 road_gr4_crowd |  | 0 | 0 | Table A2.1 |  |
| `psi.road_horizontal.0` | ψ0 road_horizontal |  | 0 | 0 | Table A2.1 |  |
| `psi.road_horizontal.1` | ψ1 road_horizontal |  | 0 | 0 | Table A2.1 |  |
| `psi.road_horizontal.2` | ψ2 road_horizontal |  | 0 | 0 | Table A2.1 |  |
| `psi.road_pedestrian.0` | ψ0 road_pedestrian |  | 0.4 | 0.4 | Table A2.1 |  |
| `psi.road_pedestrian.1` | ψ1 road_pedestrian |  | 0.4 | 0.4 | Table A2.1 |  |
| `psi.road_pedestrian.2` | ψ2 road_pedestrian |  | 0 | 0 | Table A2.1 |  |
| `psi.road_single_axle.0` | ψ0 road_single_axle |  | 0 | 0 | Table A2.1 |  |
| `psi.road_single_axle.1` | ψ1 road_single_axle |  | 0.75 | 0.75 | Table A2.1 |  |
| `psi.road_single_axle.2` | ψ2 road_single_axle |  | 0 | 0 | Table A2.1 |  |
| `psi.road_special.0` | ψ0 road_special |  | 0 | 0 | Table A2.1 |  |
| `psi.road_special.1` | ψ1 road_special |  | 0 | 0 | Table A2.1 |  |
| `psi.road_special.2` | ψ2 road_special |  | 0 | 0 | Table A2.1 |  |
| `psi.road_traffic.0` | ψ0 road_traffic |  | 0.75 | 0.75 | Table A2.1 |  |
| `psi.road_traffic.1` | ψ1 road_traffic |  | 0.75 | 0.75 | Table A2.1 |  |
| `psi.road_traffic.2` | ψ2 road_traffic |  | 0 | 0 | Table A2.1 |  |
| `psi.road_udl.0` | ψ0 road_udl |  | 0.4 | 0.4 | Table A2.1 |  |
| `psi.road_udl.1` | ψ1 road_udl |  | 0.4 | 0.4 | Table A2.1 |  |
| `psi.road_udl.2` | ψ2 road_udl |  | 0 | 0 | Table A2.1 |  |
| `psi.snow_construction.0` | ψ0 snow_construction |  | 0.8 | 0.8 | Table A2.1 |  |
| `psi.snow_construction.1` | ψ1 snow_construction |  | 0 | 0 | Table A2.1 |  |
| `psi.snow_construction.2` | ψ2 snow_construction |  | 0 | 0 | Table A2.1 |  |
| `psi.temperature_bridge.0` | ψ0 temperature_bridge |  | 0.6 | 0.8 | Table A2.1 |  |
| `psi.temperature_bridge.1` | ψ1 temperature_bridge |  | 0.6 | 0.6 | Table A2.1 |  |
| `psi.temperature_bridge.2` | ψ2 temperature_bridge |  | 0.5 | 0.5 | Table A2.1 |  |
| `psi.wind_bridge.0` | ψ0 wind_bridge |  | 0.6 | 0.6 | Table A2.1 |  |
| `psi.wind_bridge.1` | ψ1 wind_bridge |  | 0.2 | 0.2 | Table A2.1 |  |
| `psi.wind_bridge.2` | ψ2 wind_bridge |  | 0 | 0 | Table A2.1 |  |
| `psi.wind_bridge_execution.0` | ψ0 wind_bridge_execution |  | 1 | 1 | Table A2.1 |  |
| `psi.wind_bridge_execution.1` | ψ1 wind_bridge_execution |  | 0 | 0 | Table A2.1 |  |
| `psi.wind_bridge_execution.2` | ψ2 wind_bridge_execution |  | 0 | 0 | Table A2.1 |  |

##### Clause Table A2.2 (6; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi.footbridge_crowd.0` | ψ0 footbridge_crowd |  | 0 | 0 | Table A2.2 |  |
| `psi.footbridge_crowd.1` | ψ1 footbridge_crowd |  | 0.4 | 0.4 | Table A2.2 |  |
| `psi.footbridge_crowd.2` | ψ2 footbridge_crowd |  | 0 | 0 | Table A2.2 |  |
| `psi.footbridge_service.0` | ψ0 footbridge_service |  | 0 | 0 | Table A2.2 |  |
| `psi.footbridge_service.1` | ψ1 footbridge_service |  | 0.4 | 0.4 | Table A2.2 |  |
| `psi.footbridge_service.2` | ψ2 footbridge_service |  | 0 | 0 | Table A2.2 |  |

##### Clause Table A2.3 (15; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi.rail_hslm.0` | ψ0 rail_hslm |  | 1 | 1 | Table A2.3 |  |
| `psi.rail_hslm.1` | ψ1 rail_hslm |  | 1 | 1 | Table A2.3 |  |
| `psi.rail_hslm.2` | ψ2 rail_hslm |  | 0 | 0 | Table A2.3 |  |
| `psi.rail_sw2.0` | ψ0 rail_sw2 |  | 0 | 0 | Table A2.3 |  |
| `psi.rail_sw2.1` | ψ1 rail_sw2 |  | 1 | 1 | Table A2.3 |  |
| `psi.rail_sw2.2` | ψ2 rail_sw2 |  | 0 | 0 | Table A2.3 |  |
| `psi.rail_traffic.0` | ψ0 rail_traffic |  | 0.8 | 0.8 | Table A2.3 |  |
| `psi.rail_traffic.1` | ψ1 rail_traffic |  | 0.8 | 0.8 | Table A2.3 |  |
| `psi.rail_traffic.2` | ψ2 rail_traffic |  | 0 | 0 | Table A2.3 |  |
| `psi.rail_unloaded.0` | ψ0 rail_unloaded |  | 1 | 1 | Table A2.3 |  |
| `psi.rail_unloaded.1` | ψ1 rail_unloaded |  | 0 | 0 | Table A2.3 |  |
| `psi.rail_unloaded.2` | ψ2 rail_unloaded |  | 0 | 0 | Table A2.3 |  |
| `psi.wind_rail.0` | ψ0 wind_rail |  | 0.75 | 0.75 | Table A2.3 |  |
| `psi.wind_rail.1` | ψ1 wind_rail |  | 0.5 | 0.5 | Table A2.3 |  |
| `psi.wind_rail.2` | ψ2 wind_rail |  | 0 | 0 | Table A2.3 |  |

##### Clause Table A2.4(A) (5; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.equ.A2` | gG_inf.equ.A2 |  | 0.95 | 0.95 | Table A2.4(A) |  |
| `gG_sup.equ.A2` | gG_sup.equ.A2 |  | 1.05 | 1.05 | Table A2.4(A) |  |
| `gQ.other.equ.A2` | gQ.other.equ.A2 |  | 1.5 | 1.5 | Table A2.4(A) |  |
| `gQ.rail.equ.A2` | gQ.rail.equ.A2 |  | 1.45 | 1.45 | Table A2.4(A) |  |
| `gQ.road.equ.A2` | gQ.road.equ.A2 |  | 1.35 | 1.35 | Table A2.4(A) |  |

##### Clause Table A2.4(B) (6; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.str.A2` | gG_inf.str.A2 |  | 1 | 1 | Table A2.4(B) |  |
| `gG_sup.str.A2` | gG_sup.str.A2 |  | 1.35 | 1.35 | Table A2.4(B) |  |
| `gQ.other.str.A2` | gQ.other.str.A2 |  | 1.5 | 1.5 | Table A2.4(B) |  |
| `gQ.rail.str.A2` | gQ.rail.str.A2 |  | 1.45 | 1.45 | Table A2.4(B) |  |
| `gQ.road.str.A2` | gQ.road.str.A2 |  | 1.35 | 1.35 | Table A2.4(B) |  |
| `xi.str.A2` | xi.str.A2 |  | 0.85 | 0.85 | Table A2.4(B) |  |

##### Clause Table A2.4(B) via GEO-2 (DA2*) (5; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.geo2.A2` | gG_inf.geo2.A2 |  | 1 | 1 | Table A2.4(B) via GEO-2 (DA2*) |  |
| `gG_sup.geo2.A2` | gG_sup.geo2.A2 |  | 1.35 | 1.35 | Table A2.4(B) via GEO-2 (DA2*) |  |
| `gQ.other.geo2.A2` | gQ.other.geo2.A2 |  | 1.5 | 1.5 | Table A2.4(B) via GEO-2 (DA2*) |  |
| `gQ.rail.geo2.A2` | gQ.rail.geo2.A2 |  | 1.45 | 1.45 | Table A2.4(B) via GEO-2 (DA2*) |  |
| `gQ.road.geo2.A2` | gQ.road.geo2.A2 |  | 1.35 | 1.35 | Table A2.4(B) via GEO-2 (DA2*) |  |

##### Clause Table A2.4(C) (5; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gG_inf.geo.A2` | gG_inf.geo.A2 |  | 1 | 1 | Table A2.4(C) |  |
| `gG_sup.geo.A2` | gG_sup.geo.A2 |  | 1 | 1 | Table A2.4(C) |  |
| `gQ.other.geo.A2` | gQ.other.geo.A2 |  | 1.3 | 1.3 | Table A2.4(C) |  |
| `gQ.rail.geo.A2` | gQ.rail.geo.A2 |  | 1.25 | 1.25 | Table A2.4(C) |  |
| `gQ.road.geo.A2` | gQ.road.geo.A2 |  | 1.15 | 1.15 | Table A2.4(C) |  |

##### Clause Table B2 (6; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `beta1.1` | beta1.1 |  | 4.2 | 4.2 | Table B2 |  |
| `beta1.2` | beta1.2 |  | 4.7 | 4.7 | Table B2 |  |
| `beta1.3` | beta1.3 |  | 5.2 | 5.2 | Table B2 |  |
| `beta50.1` | beta50.1 |  | 3.3 | 3.3 | Table B2 |  |
| `beta50.2` | beta50.2 |  | 3.8 | 3.8 | Table B2 |  |
| `beta50.3` | beta50.3 |  | 4.3 | 4.3 | Table B2 |  |

##### Clause Table B3 (4; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-02.1` | ? |  | - | - | Table B3 | en1990-en1991-target.md:48: NA-DE same (DE uses K_FI via the design supervision/inspection route, see E90.05). out: K_FI applied to unfavourable γF (T |
| `kfi.1` | kfi.1 |  | 0.9 | 0.9 | Table B3 |  |
| `kfi.2` | kfi.2 |  | 1 | 1 | Table B3 |  |
| `kfi.3` | kfi.3 |  | 1.1 | 1.1 | Table B3 |  |

##### Clause Table NA.A.1.1 (6; DIN EN 1990/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi.settlement.0` | ψ0 settlement |  | 1 | 1 | Table NA.A.1.1 |  |
| `psi.settlement.1` | ψ1 settlement |  | 1 | 1 | Table NA.A.1.1 |  |
| `psi.settlement.2` | ψ2 settlement |  | 1 | 1 | Table NA.A.1.1 |  |
| `psi.variable_other.0` | ψ0 variable_other |  | 0.8 | 0.8 | Table NA.A.1.1 |  |
| `psi.variable_other.1` | ψ1 variable_other |  | 0.7 | 0.7 | Table NA.A.1.1 |  |
| `psi.variable_other.2` | ψ2 variable_other |  | 0.5 | 0.5 | Table NA.A.1.1 |  |

### EN 1991 (534)

#### EN 1991, part 1 (285)

##### Clause 4.3.2 (3; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.k-r.coef` | k_r coefficient |  | 0.19 | 0.19 | 4.3.2 |  |
| `1-4.k-r.exp` | k_r exponent |  | 0.07 | 0.07 | 4.3.2 |  |
| `1-4.k-r.z0-ref` | k_r reference roughness | m | 0.05 | 0.05 | 4.3.2 |  |

##### Clause 5.3.4 (4; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.mu2.alpha-ref` | mu_2 reference angle | deg | 30 | 30 | 5.3.4 |  |
| `1-3.mu2.base` | mu_2 base |  | 0.8 | 0.8 | 5.3.4 |  |
| `1-3.mu2.multi-span-cap` | mu_2 multi-span cap |  | 1.6 | 1.6 | 5.3.4 |  |
| `1-3.mu2.slope` | mu_2 slope |  | 0.8 | 0.8 | 5.3.4 |  |

##### Clause 5.3.5 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.mu3.base` | mu_3 base |  | 0.2 | 0.2 | 5.3.5 |  |
| `1-3.mu3.slope` | mu_3 slope |  | 10 | 10 | 5.3.5 |  |

##### Clause 7.4 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.cf.signboard-default` | c_f signboard default |  | 1.8 | 1.8 | 7.4 |  |

##### Clause Annex F (7; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.delta-s.concrete` | structural damping delta_s concrete |  | 0.1 | 0.1 | Annex F |  |
| `1-4.delta-s.mixed` | structural damping delta_s mixed |  | 0.08 | 0.08 | Annex F |  |
| `1-4.delta-s.steel` | structural damping delta_s steel |  | 0.05 | 0.05 | Annex F |  |
| `1-4.mode-exponent.cantilever` | mode shape exponent cantilever |  | 1.5 | 1.5 | Annex F |  |
| `1-4.mode-exponent.core` | mode shape exponent core |  | 1 | 1 | Annex F |  |
| `1-4.mode-exponent.framed` | mode shape exponent framed |  | 0.6 | 0.6 | Annex F |  |
| `1-4.mode-exponent.lattice` | mode shape exponent lattice |  | 2 | 2 | Annex F |  |

##### Clause NA.4 (3; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.exceptional-factor` | s_Ad/s_k |  | 2.3 | 2.3 | NA.4 |  |
| `1-3.snow-altitude-offset` | altitude offset | m | 140 | 140 | NA.4 |  |
| `1-3.snow-altitude-scale` | altitude scale | m | 760 | 760 | NA.4 |  |

##### Clause NA.4 ground snow (20; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.snow-zone.z1.a` | s_k zone z1 a | kN/m2 | - | 0.19 | NA.4 ground snow |  |
| `1-3.snow-zone.z1.b` | s_k zone z1 b | kN/m2 | - | 0.91 | NA.4 ground snow |  |
| `1-3.snow-zone.z1.factor` | s_k zone z1 zone factor |  | - | 1 | NA.4 ground snow |  |
| `1-3.snow-zone.z1.min` | s_k zone z1 s_k minimum | kN/m2 | - | 0.65 | NA.4 ground snow |  |
| `1-3.snow-zone.z1a.a` | s_k zone z1a a | kN/m2 | - | 0.19 | NA.4 ground snow |  |
| `1-3.snow-zone.z1a.b` | s_k zone z1a b | kN/m2 | - | 0.91 | NA.4 ground snow |  |
| `1-3.snow-zone.z1a.factor` | s_k zone z1a zone factor |  | - | 1.25 | NA.4 ground snow |  |
| `1-3.snow-zone.z1a.min` | s_k zone z1a s_k minimum | kN/m2 | - | 0.65 | NA.4 ground snow |  |
| `1-3.snow-zone.z2.a` | s_k zone z2 a | kN/m2 | - | 0.25 | NA.4 ground snow |  |
| `1-3.snow-zone.z2.b` | s_k zone z2 b | kN/m2 | - | 1.91 | NA.4 ground snow |  |
| `1-3.snow-zone.z2.factor` | s_k zone z2 zone factor |  | - | 1 | NA.4 ground snow |  |
| `1-3.snow-zone.z2.min` | s_k zone z2 s_k minimum | kN/m2 | - | 0.85 | NA.4 ground snow |  |
| `1-3.snow-zone.z2a.a` | s_k zone z2a a | kN/m2 | - | 0.25 | NA.4 ground snow |  |
| `1-3.snow-zone.z2a.b` | s_k zone z2a b | kN/m2 | - | 1.91 | NA.4 ground snow |  |
| `1-3.snow-zone.z2a.factor` | s_k zone z2a zone factor |  | - | 1.25 | NA.4 ground snow |  |
| `1-3.snow-zone.z2a.min` | s_k zone z2a s_k minimum | kN/m2 | - | 0.85 | NA.4 ground snow |  |
| `1-3.snow-zone.z3.a` | s_k zone z3 a | kN/m2 | - | 0.31 | NA.4 ground snow |  |
| `1-3.snow-zone.z3.b` | s_k zone z3 b | kN/m2 | - | 2.91 | NA.4 ground snow |  |
| `1-3.snow-zone.z3.factor` | s_k zone z3 zone factor |  | - | 1 | NA.4 ground snow |  |
| `1-3.snow-zone.z3.min` | s_k zone z3 s_k minimum | kN/m2 | - | 1.1 | NA.4 ground snow |  |

##### Clause NA.B simplified profile (14; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.na-profile.coast.c1` | q_p profile coast c1 |  | 1.8 | 1.8 | NA.B simplified profile |  |
| `1-4.na-profile.coast.c2` | q_p profile coast c2 |  | 2.3 | 2.3 | NA.B simplified profile |  |
| `1-4.na-profile.coast.c3` | q_p profile coast c3 |  | 2.6 | 2.6 | NA.B simplified profile |  |
| `1-4.na-profile.coast.e2` | q_p profile coast e2 |  | 0.27 | 0.27 | NA.B simplified profile |  |
| `1-4.na-profile.coast.e3` | q_p profile coast e3 |  | 0.19 | 0.19 | NA.B simplified profile |  |
| `1-4.na-profile.coast.limit1` | q_p profile coast limit1 | m | 4 | 4 | NA.B simplified profile |  |
| `1-4.na-profile.coast.limit2` | q_p profile coast limit2 | m | 50 | 50 | NA.B simplified profile |  |
| `1-4.na-profile.inland.c1` | q_p profile inland c1 |  | 1.5 | 1.5 | NA.B simplified profile |  |
| `1-4.na-profile.inland.c2` | q_p profile inland c2 |  | 1.7 | 1.7 | NA.B simplified profile |  |
| `1-4.na-profile.inland.c3` | q_p profile inland c3 |  | 2.1 | 2.1 | NA.B simplified profile |  |
| `1-4.na-profile.inland.e2` | q_p profile inland e2 |  | 0.37 | 0.37 | NA.B simplified profile |  |
| `1-4.na-profile.inland.e3` | q_p profile inland e3 |  | 0.24 | 0.24 | NA.B simplified profile |  |
| `1-4.na-profile.inland.limit1` | q_p profile inland limit1 | m | 7 | 7 | NA.B simplified profile |  |
| `1-4.na-profile.inland.limit2` | q_p profile inland limit2 | m | 50 | 50 | NA.B simplified profile |  |

##### Clause Table 4.1 (10; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.terrain.i.z0` | z_0 category i | m | 0.01 | 0.01 | Table 4.1 |  |
| `1-4.terrain.i.zmin` | z_min category i | m | 1 | 1 | Table 4.1 |  |
| `1-4.terrain.ii.z0` | z_0 category ii | m | 0.05 | 0.05 | Table 4.1 |  |
| `1-4.terrain.ii.zmin` | z_min category ii | m | 2 | 2 | Table 4.1 |  |
| `1-4.terrain.iii.z0` | z_0 category iii | m | 0.3 | 0.3 | Table 4.1 |  |
| `1-4.terrain.iii.zmin` | z_min category iii | m | 5 | 5 | Table 4.1 |  |
| `1-4.terrain.iv.z0` | z_0 category iv | m | 1 | 1 | Table 4.1 |  |
| `1-4.terrain.iv.zmin` | z_min category iv | m | 10 | 10 | Table 4.1 |  |
| `1-4.terrain.zero.z0` | z_0 category zero | m | 0.003 | 0.003 | Table 4.1 |  |
| `1-4.terrain.zero.zmin` | z_min category zero | m | 1 | 1 | Table 4.1 |  |

##### Clause Table 5.2 (3; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.mu1.a0` | mu_1 at 0 deg |  | 0.8 | 0.8 | Table 5.2 |  |
| `1-3.mu1.a30` | mu_1 at 30 deg |  | 0.8 | 0.8 | Table 5.2 |  |
| `1-3.mu1.a60` | mu_1 at 60 deg |  | 0 | 0 | Table 5.2 |  |

##### Clause Table 7.1 (30; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.cpe.wall.a.h0-25.c1` | c_pe,1 zone A h/d h0-25 |  | -1.4 | -1.4 | Table 7.1 |  |
| `1-4.cpe.wall.a.h0-25.c10` | c_pe,10 zone A h/d h0-25 |  | -1.2 | -1.2 | Table 7.1 |  |
| `1-4.cpe.wall.a.h1.c1` | c_pe,1 zone A h/d h1 |  | -1.4 | -1.4 | Table 7.1 |  |
| `1-4.cpe.wall.a.h1.c10` | c_pe,10 zone A h/d h1 |  | -1.2 | -1.2 | Table 7.1 |  |
| `1-4.cpe.wall.a.h5.c1` | c_pe,1 zone A h/d h5 |  | -1.4 | -1.4 | Table 7.1 |  |
| `1-4.cpe.wall.a.h5.c10` | c_pe,10 zone A h/d h5 |  | -1.2 | -1.2 | Table 7.1 |  |
| `1-4.cpe.wall.b.h0-25.c1` | c_pe,1 zone B h/d h0-25 |  | -1.1 | -1.1 | Table 7.1 |  |
| `1-4.cpe.wall.b.h0-25.c10` | c_pe,10 zone B h/d h0-25 |  | -0.8 | -0.8 | Table 7.1 |  |
| `1-4.cpe.wall.b.h1.c1` | c_pe,1 zone B h/d h1 |  | -1.1 | -1.1 | Table 7.1 |  |
| `1-4.cpe.wall.b.h1.c10` | c_pe,10 zone B h/d h1 |  | -0.8 | -0.8 | Table 7.1 |  |
| `1-4.cpe.wall.b.h5.c1` | c_pe,1 zone B h/d h5 |  | -1.1 | -1.1 | Table 7.1 |  |
| `1-4.cpe.wall.b.h5.c10` | c_pe,10 zone B h/d h5 |  | -0.8 | -0.8 | Table 7.1 |  |
| `1-4.cpe.wall.c.h0-25.c1` | c_pe,1 zone C h/d h0-25 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.c.h0-25.c10` | c_pe,10 zone C h/d h0-25 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.c.h1.c1` | c_pe,1 zone C h/d h1 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.c.h1.c10` | c_pe,10 zone C h/d h1 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.c.h5.c1` | c_pe,1 zone C h/d h5 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.c.h5.c10` | c_pe,10 zone C h/d h5 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.d.h0-25.c1` | c_pe,1 zone D h/d h0-25 |  | 1 | 1 | Table 7.1 |  |
| `1-4.cpe.wall.d.h0-25.c10` | c_pe,10 zone D h/d h0-25 |  | 0.7 | 0.7 | Table 7.1 |  |
| `1-4.cpe.wall.d.h1.c1` | c_pe,1 zone D h/d h1 |  | 1 | 1 | Table 7.1 |  |
| `1-4.cpe.wall.d.h1.c10` | c_pe,10 zone D h/d h1 |  | 0.8 | 0.8 | Table 7.1 |  |
| `1-4.cpe.wall.d.h5.c1` | c_pe,1 zone D h/d h5 |  | 1 | 1 | Table 7.1 |  |
| `1-4.cpe.wall.d.h5.c10` | c_pe,10 zone D h/d h5 |  | 0.8 | 0.8 | Table 7.1 |  |
| `1-4.cpe.wall.e.h0-25.c1` | c_pe,1 zone E h/d h0-25 |  | -0.3 | -0.3 | Table 7.1 |  |
| `1-4.cpe.wall.e.h0-25.c10` | c_pe,10 zone E h/d h0-25 |  | -0.3 | -0.3 | Table 7.1 |  |
| `1-4.cpe.wall.e.h1.c1` | c_pe,1 zone E h/d h1 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.e.h1.c10` | c_pe,10 zone E h/d h1 |  | -0.5 | -0.5 | Table 7.1 |  |
| `1-4.cpe.wall.e.h5.c1` | c_pe,1 zone E h/d h5 |  | -0.7 | -0.7 | Table 7.1 |  |
| `1-4.cpe.wall.e.h5.c10` | c_pe,10 zone E h/d h5 |  | -0.7 | -0.7 | Table 7.1 |  |

##### Clause Table 7.2 (62; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.cpe.flat.curved.f.r0-05.c1` | c_pe,1 flat roof curved zone F r0-05 |  | -1.5 | -1.5 | Table 7.2 |  |
| `1-4.cpe.flat.curved.f.r0-05.c10` | c_pe,10 flat roof curved zone F r0-05 |  | -1 | -1 | Table 7.2 |  |
| `1-4.cpe.flat.curved.f.r0-10.c1` | c_pe,1 flat roof curved zone F r0-10 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.curved.f.r0-10.c10` | c_pe,10 flat roof curved zone F r0-10 |  | -0.7 | -0.7 | Table 7.2 |  |
| `1-4.cpe.flat.curved.f.r0-20.c1` | c_pe,1 flat roof curved zone F r0-20 |  | -0.8 | -0.8 | Table 7.2 |  |
| `1-4.cpe.flat.curved.f.r0-20.c10` | c_pe,10 flat roof curved zone F r0-20 |  | -0.5 | -0.5 | Table 7.2 |  |
| `1-4.cpe.flat.curved.g.r0-05.c1` | c_pe,1 flat roof curved zone G r0-05 |  | -1.8 | -1.8 | Table 7.2 |  |
| `1-4.cpe.flat.curved.g.r0-05.c10` | c_pe,10 flat roof curved zone G r0-05 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.curved.g.r0-10.c1` | c_pe,1 flat roof curved zone G r0-10 |  | -1.4 | -1.4 | Table 7.2 |  |
| `1-4.cpe.flat.curved.g.r0-10.c10` | c_pe,10 flat roof curved zone G r0-10 |  | -0.8 | -0.8 | Table 7.2 |  |
| `1-4.cpe.flat.curved.g.r0-20.c1` | c_pe,1 flat roof curved zone G r0-20 |  | -0.8 | -0.8 | Table 7.2 |  |
| `1-4.cpe.flat.curved.g.r0-20.c10` | c_pe,10 flat roof curved zone G r0-20 |  | -0.5 | -0.5 | Table 7.2 |  |
| `1-4.cpe.flat.curved.h.r0-05.c1` | c_pe,1 flat roof curved zone H r0-05 |  | -0.4 | -0.4 | Table 7.2 |  |
| `1-4.cpe.flat.curved.h.r0-05.c10` | c_pe,10 flat roof curved zone H r0-05 |  | -0.4 | -0.4 | Table 7.2 |  |
| `1-4.cpe.flat.curved.h.r0-10.c1` | c_pe,1 flat roof curved zone H r0-10 |  | -0.3 | -0.3 | Table 7.2 |  |
| `1-4.cpe.flat.curved.h.r0-10.c10` | c_pe,10 flat roof curved zone H r0-10 |  | -0.3 | -0.3 | Table 7.2 |  |
| `1-4.cpe.flat.curved.h.r0-20.c1` | c_pe,1 flat roof curved zone H r0-20 |  | -0.3 | -0.3 | Table 7.2 |  |
| `1-4.cpe.flat.curved.h.r0-20.c10` | c_pe,10 flat roof curved zone H r0-20 |  | -0.3 | -0.3 | Table 7.2 |  |
| `1-4.cpe.flat.i.minus` | c_pe flat roof zone I suction |  | -0.2 | -0.2 | Table 7.2 |  |
| `1-4.cpe.flat.i.plus` | c_pe flat roof zone I pressure |  | 0.2 | 0.2 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.f.a30.c1` | c_pe,1 flat roof mansard zone F a30 |  | -1.5 | -1.5 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.f.a30.c10` | c_pe,10 flat roof mansard zone F a30 |  | -1 | -1 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.f.a45.c1` | c_pe,1 flat roof mansard zone F a45 |  | -1.8 | -1.8 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.f.a45.c10` | c_pe,10 flat roof mansard zone F a45 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.f.a60.c1` | c_pe,1 flat roof mansard zone F a60 |  | -1.9 | -1.9 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.f.a60.c10` | c_pe,10 flat roof mansard zone F a60 |  | -1.3 | -1.3 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.g.a30.c1` | c_pe,1 flat roof mansard zone G a30 |  | -1.5 | -1.5 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.g.a30.c10` | c_pe,10 flat roof mansard zone G a30 |  | -1 | -1 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.g.a45.c1` | c_pe,1 flat roof mansard zone G a45 |  | -1.9 | -1.9 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.g.a45.c10` | c_pe,10 flat roof mansard zone G a45 |  | -1.3 | -1.3 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.g.a60.c1` | c_pe,1 flat roof mansard zone G a60 |  | -1.9 | -1.9 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.g.a60.c10` | c_pe,10 flat roof mansard zone G a60 |  | -1.3 | -1.3 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.h.a30.c1` | c_pe,1 flat roof mansard zone H a30 |  | -0.3 | -0.3 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.h.a30.c10` | c_pe,10 flat roof mansard zone H a30 |  | -0.3 | -0.3 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.h.a45.c1` | c_pe,1 flat roof mansard zone H a45 |  | -0.4 | -0.4 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.h.a45.c10` | c_pe,10 flat roof mansard zone H a45 |  | -0.4 | -0.4 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.h.a60.c1` | c_pe,1 flat roof mansard zone H a60 |  | -0.5 | -0.5 | Table 7.2 |  |
| `1-4.cpe.flat.mansard.h.a60.c10` | c_pe,10 flat roof mansard zone H a60 |  | -0.5 | -0.5 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0-025.c1` | c_pe,1 flat roof parapet zone F hp0-025 |  | -2.2 | -2.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0-025.c10` | c_pe,10 flat roof parapet zone F hp0-025 |  | -1.6 | -1.6 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0-05.c1` | c_pe,1 flat roof parapet zone F hp0-05 |  | -2 | -2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0-05.c10` | c_pe,10 flat roof parapet zone F hp0-05 |  | -1.4 | -1.4 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0-10.c1` | c_pe,1 flat roof parapet zone F hp0-10 |  | -1.8 | -1.8 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0-10.c10` | c_pe,10 flat roof parapet zone F hp0-10 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0.c1` | c_pe,1 flat roof parapet zone F hp0 |  | -2.5 | -2.5 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.f.hp0.c10` | c_pe,10 flat roof parapet zone F hp0 |  | -1.8 | -1.8 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0-025.c1` | c_pe,1 flat roof parapet zone G hp0-025 |  | -1.8 | -1.8 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0-025.c10` | c_pe,10 flat roof parapet zone G hp0-025 |  | -1.1 | -1.1 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0-05.c1` | c_pe,1 flat roof parapet zone G hp0-05 |  | -1.6 | -1.6 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0-05.c10` | c_pe,10 flat roof parapet zone G hp0-05 |  | -0.9 | -0.9 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0-10.c1` | c_pe,1 flat roof parapet zone G hp0-10 |  | -1.4 | -1.4 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0-10.c10` | c_pe,10 flat roof parapet zone G hp0-10 |  | -0.8 | -0.8 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0.c1` | c_pe,1 flat roof parapet zone G hp0 |  | -2 | -2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.g.hp0.c10` | c_pe,10 flat roof parapet zone G hp0 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0-025.c1` | c_pe,1 flat roof parapet zone H hp0-025 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0-025.c10` | c_pe,10 flat roof parapet zone H hp0-025 |  | -0.7 | -0.7 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0-05.c1` | c_pe,1 flat roof parapet zone H hp0-05 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0-05.c10` | c_pe,10 flat roof parapet zone H hp0-05 |  | -0.7 | -0.7 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0-10.c1` | c_pe,1 flat roof parapet zone H hp0-10 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0-10.c10` | c_pe,10 flat roof parapet zone H hp0-10 |  | -0.7 | -0.7 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0.c1` | c_pe,1 flat roof parapet zone H hp0 |  | -1.2 | -1.2 | Table 7.2 |  |
| `1-4.cpe.flat.parapet.h.hp0.c10` | c_pe,10 flat roof parapet zone H hp0 |  | -0.7 | -0.7 | Table 7.2 |  |

##### Clause Tables 7.3a/7.4a (126; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.cpe.pitched0.f.primary.m15.c1` | c_pe,1 pitched roof zone F primary -15 d |  | -2.8 | -2.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m15.c10` | c_pe,10 pitched roof zone F primary -15  |  | -2.5 | -2.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m30.c1` | c_pe,1 pitched roof zone F primary -30 d |  | -2 | -2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m30.c10` | c_pe,10 pitched roof zone F primary -30  |  | -1.1 | -1.1 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m45.c1` | c_pe,1 pitched roof zone F primary -45 d |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m45.c10` | c_pe,10 pitched roof zone F primary -45  |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m5.c1` | c_pe,1 pitched roof zone F primary -5 de |  | -2.5 | -2.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.m5.c10` | c_pe,10 pitched roof zone F primary -5 d |  | -2.3 | -2.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p15.c1` | c_pe,1 pitched roof zone F primary 15 de |  | -2 | -2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p15.c10` | c_pe,10 pitched roof zone F primary 15 d |  | -0.9 | -0.9 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p30.c1` | c_pe,1 pitched roof zone F primary 30 de |  | -1.5 | -1.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p30.c10` | c_pe,10 pitched roof zone F primary 30 d |  | -0.5 | -0.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p45.c1` | c_pe,1 pitched roof zone F primary 45 de |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p45.c10` | c_pe,10 pitched roof zone F primary 45 d |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p5.c1` | c_pe,1 pitched roof zone F primary 5 deg |  | -2.5 | -2.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p5.c10` | c_pe,10 pitched roof zone F primary 5 de |  | -1.7 | -1.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p60.c1` | c_pe,1 pitched roof zone F primary 60 de |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p60.c10` | c_pe,10 pitched roof zone F primary 60 d |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p75.c1` | c_pe,1 pitched roof zone F primary 75 de |  | 0.8 | 0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.primary.p75.c10` | c_pe,10 pitched roof zone F primary 75 d |  | 0.8 | 0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p15.c1` | c_pe,1 pitched roof zone F secondary 15  |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p15.c10` | c_pe,10 pitched roof zone F secondary 15 |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p30.c1` | c_pe,1 pitched roof zone F secondary 30  |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p30.c10` | c_pe,10 pitched roof zone F secondary 30 |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p45.c1` | c_pe,1 pitched roof zone F secondary 45  |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p45.c10` | c_pe,10 pitched roof zone F secondary 45 |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p5.c1` | c_pe,1 pitched roof zone F secondary 5 d |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.f.secondary.p5.c10` | c_pe,10 pitched roof zone F secondary 5  |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m15.c1` | c_pe,1 pitched roof zone G primary -15 d |  | -2 | -2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m15.c10` | c_pe,10 pitched roof zone G primary -15  |  | -1.3 | -1.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m30.c1` | c_pe,1 pitched roof zone G primary -30 d |  | -1.5 | -1.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m30.c10` | c_pe,10 pitched roof zone G primary -30  |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m45.c1` | c_pe,1 pitched roof zone G primary -45 d |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m45.c10` | c_pe,10 pitched roof zone G primary -45  |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m5.c1` | c_pe,1 pitched roof zone G primary -5 de |  | -2 | -2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.m5.c10` | c_pe,10 pitched roof zone G primary -5 d |  | -1.2 | -1.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p15.c1` | c_pe,1 pitched roof zone G primary 15 de |  | -1.5 | -1.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p15.c10` | c_pe,10 pitched roof zone G primary 15 d |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p30.c1` | c_pe,1 pitched roof zone G primary 30 de |  | -1.5 | -1.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p30.c10` | c_pe,10 pitched roof zone G primary 30 d |  | -0.5 | -0.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p45.c1` | c_pe,1 pitched roof zone G primary 45 de |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p45.c10` | c_pe,10 pitched roof zone G primary 45 d |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p5.c1` | c_pe,1 pitched roof zone G primary 5 deg |  | -2 | -2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p5.c10` | c_pe,10 pitched roof zone G primary 5 de |  | -1.2 | -1.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p60.c1` | c_pe,1 pitched roof zone G primary 60 de |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p60.c10` | c_pe,10 pitched roof zone G primary 60 d |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p75.c1` | c_pe,1 pitched roof zone G primary 75 de |  | 0.8 | 0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.primary.p75.c10` | c_pe,10 pitched roof zone G primary 75 d |  | 0.8 | 0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p15.c1` | c_pe,1 pitched roof zone G secondary 15  |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p15.c10` | c_pe,10 pitched roof zone G secondary 15 |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p30.c1` | c_pe,1 pitched roof zone G secondary 30  |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p30.c10` | c_pe,10 pitched roof zone G secondary 30 |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p45.c1` | c_pe,1 pitched roof zone G secondary 45  |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p45.c10` | c_pe,10 pitched roof zone G secondary 45 |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p5.c1` | c_pe,1 pitched roof zone G secondary 5 d |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.g.secondary.p5.c10` | c_pe,10 pitched roof zone G secondary 5  |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m15.c1` | c_pe,1 pitched roof zone H primary -15 d |  | -1.2 | -1.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m15.c10` | c_pe,10 pitched roof zone H primary -15  |  | -0.9 | -0.9 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m30.c1` | c_pe,1 pitched roof zone H primary -30 d |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m30.c10` | c_pe,10 pitched roof zone H primary -30  |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m45.c1` | c_pe,1 pitched roof zone H primary -45 d |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m45.c10` | c_pe,10 pitched roof zone H primary -45  |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m5.c1` | c_pe,1 pitched roof zone H primary -5 de |  | -1.2 | -1.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.m5.c10` | c_pe,10 pitched roof zone H primary -5 d |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p15.c1` | c_pe,1 pitched roof zone H primary 15 de |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p15.c10` | c_pe,10 pitched roof zone H primary 15 d |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p30.c1` | c_pe,1 pitched roof zone H primary 30 de |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p30.c10` | c_pe,10 pitched roof zone H primary 30 d |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p45.c1` | c_pe,1 pitched roof zone H primary 45 de |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p45.c10` | c_pe,10 pitched roof zone H primary 45 d |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p5.c1` | c_pe,1 pitched roof zone H primary 5 deg |  | -1.2 | -1.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p5.c10` | c_pe,10 pitched roof zone H primary 5 de |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p60.c1` | c_pe,1 pitched roof zone H primary 60 de |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p60.c10` | c_pe,10 pitched roof zone H primary 60 d |  | 0.7 | 0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p75.c1` | c_pe,1 pitched roof zone H primary 75 de |  | 0.8 | 0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.primary.p75.c10` | c_pe,10 pitched roof zone H primary 75 d |  | 0.8 | 0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p15.c1` | c_pe,1 pitched roof zone H secondary 15  |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p15.c10` | c_pe,10 pitched roof zone H secondary 15 |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p30.c1` | c_pe,1 pitched roof zone H secondary 30  |  | 0.4 | 0.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p30.c10` | c_pe,10 pitched roof zone H secondary 30 |  | 0.4 | 0.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p45.c1` | c_pe,1 pitched roof zone H secondary 45  |  | 0.6 | 0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p45.c10` | c_pe,10 pitched roof zone H secondary 45 |  | 0.6 | 0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p5.c1` | c_pe,1 pitched roof zone H secondary 5 d |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.h.secondary.p5.c10` | c_pe,10 pitched roof zone H secondary 5  |  | 0 | 0 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m15.c1` | c_pe,1 pitched roof zone I primary -15 d |  | -0.5 | -0.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m15.c10` | c_pe,10 pitched roof zone I primary -15  |  | -0.5 | -0.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m30.c1` | c_pe,1 pitched roof zone I primary -30 d |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m30.c10` | c_pe,10 pitched roof zone I primary -30  |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m45.c1` | c_pe,1 pitched roof zone I primary -45 d |  | -0.7 | -0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m45.c10` | c_pe,10 pitched roof zone I primary -45  |  | -0.7 | -0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m5.c1` | c_pe,1 pitched roof zone I primary -5 de |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.m5.c10` | c_pe,10 pitched roof zone I primary -5 d |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p15.c1` | c_pe,1 pitched roof zone I primary 15 de |  | -0.4 | -0.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p15.c10` | c_pe,10 pitched roof zone I primary 15 d |  | -0.4 | -0.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p30.c1` | c_pe,1 pitched roof zone I primary 30 de |  | -0.4 | -0.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p30.c10` | c_pe,10 pitched roof zone I primary 30 d |  | -0.4 | -0.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p45.c1` | c_pe,1 pitched roof zone I primary 45 de |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p45.c10` | c_pe,10 pitched roof zone I primary 45 d |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p5.c1` | c_pe,1 pitched roof zone I primary 5 deg |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p5.c10` | c_pe,10 pitched roof zone I primary 5 de |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p60.c1` | c_pe,1 pitched roof zone I primary 60 de |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p60.c10` | c_pe,10 pitched roof zone I primary 60 d |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p75.c1` | c_pe,1 pitched roof zone I primary 75 de |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.i.primary.p75.c10` | c_pe,10 pitched roof zone I primary 75 d |  | -0.2 | -0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m15.c1` | c_pe,1 pitched roof zone J primary -15 d |  | -1.2 | -1.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m15.c10` | c_pe,10 pitched roof zone J primary -15  |  | -0.7 | -0.7 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m30.c1` | c_pe,1 pitched roof zone J primary -30 d |  | -1.4 | -1.4 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m30.c10` | c_pe,10 pitched roof zone J primary -30  |  | -0.8 | -0.8 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m45.c1` | c_pe,1 pitched roof zone J primary -45 d |  | -1.5 | -1.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m45.c10` | c_pe,10 pitched roof zone J primary -45  |  | -1 | -1 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m5.c1` | c_pe,1 pitched roof zone J primary -5 de |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.m5.c10` | c_pe,10 pitched roof zone J primary -5 d |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p15.c1` | c_pe,1 pitched roof zone J primary 15 de |  | -1.5 | -1.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p15.c10` | c_pe,10 pitched roof zone J primary 15 d |  | -1 | -1 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p30.c1` | c_pe,1 pitched roof zone J primary 30 de |  | -0.5 | -0.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p30.c10` | c_pe,10 pitched roof zone J primary 30 d |  | -0.5 | -0.5 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p45.c1` | c_pe,1 pitched roof zone J primary 45 de |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p45.c10` | c_pe,10 pitched roof zone J primary 45 d |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p5.c1` | c_pe,1 pitched roof zone J primary 5 deg |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p5.c10` | c_pe,10 pitched roof zone J primary 5 de |  | -0.6 | -0.6 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p60.c1` | c_pe,1 pitched roof zone J primary 60 de |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p60.c10` | c_pe,10 pitched roof zone J primary 60 d |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p75.c1` | c_pe,1 pitched roof zone J primary 75 de |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.primary.p75.c10` | c_pe,10 pitched roof zone J primary 75 d |  | -0.3 | -0.3 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.secondary.p5.c1` | c_pe,1 pitched roof zone J secondary 5 d |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |
| `1-4.cpe.pitched0.j.secondary.p5.c10` | c_pe,10 pitched roof zone J secondary 5  |  | 0.2 | 0.2 | Tables 7.3a/7.4a |  |

#### EN 1991, part 1-1 (30)

##### Clause 1.0 (5; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-04.1` | allow_area_reduction |  | - | - | 1.0 | en1990-en1991-target.md:185: NA-DE: reduction by loaded area α_A is NOT permitted |
| `e11-04.2` | allow_area_reduction |  | - | - | 1.0 | en1990-en1991-target.md:185: reduction α_n for columns allowed with the ψ0 above |
| `e11-05.1` | ? |  | - | - | 1.0 | en1990-en1991-target.md:188: f: q_k line load on parapet/barrier (kN/m): A 0.5 (EN 0.2-1.0), B/C1 0.5, C2-C4/D 1.0? (rec 1.0 ), C5 3.0, E 1.0-2.0 (rec |
| `e11-05.2` | ? |  | - | - | 1.0 | en1990-en1991-target.md:188: Vehicle barriers (F): F_k = ½ m v²/(δ_c+δ_b) with m = vehicle mass (1500 kg), v = 4.5 m/s, deformations δ_c,δ_b (EN 6.4.3 |
| `e11-05.3` | ? |  | - | - | 1.0 | en1990-en1991-target.md:188: EN rec "F = 150 kN"? Use Annex-dependent: rec 150 kN at 375 mm above floor, DE 100 kN at 0.5 m . |

##### Clause 2.0 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-03.2` | ? |  | - | - | 2.0 | en1990-en1991-target.md:182: E1 7.5 / 7.0 (≥7.5, 3.0 per m storage height recommended: 6.0 kN/m^2 per metre in DE ) |

##### Clause 5.00 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l520.1` | ? |  | - | - | 5.00 | en1990-en1991-target.md:520: Note: α_A example is EN-only (NA-DE forbids area reduction ). |

##### Clause 6.3.1.2 NA (4; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `alpha-a.de-c0` | α_A c0 |  | - | 0.5 | 6.3.1.2 NA |  |
| `alpha-a.de-c1` | α_A c1 | m | - | 10 | 6.3.1.2 NA |  |
| `alpha-a.de-max` | α_A max |  | - | 1 | 6.3.1.2 NA |  |
| `alpha-a.de-min` | α_A min |  | - | 0.7 | 6.3.1.2 NA |  |

##### Clause 6.3.1.2(8) (7; DIN EN 1991-1-1/NA, DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-02.1` | LineLoad |  | - | - | 6.3.1.2(8) | en1990-en1991-target.md:179: heavier: single loads individually (no equivalent) . |
| `partition.band1.limit` | g1 | kN/m | 1 | 3 | 6.3.1.2(8) |  |
| `partition.band1.q` | q1 | kN/m2 | 0.5 | 0.8 | 6.3.1.2(8) |  |
| `partition.band2.limit` | g2 | kN/m | 2 | 5 | 6.3.1.2(8) |  |
| `partition.band2.q` | q2 | kN/m2 | 0.8 | 1.2 | 6.3.1.2(8) |  |
| `partition.band3.limit` | g3 | kN/m | 3 | 6 | 6.3.1.2(8) |  |
| `partition.band3.q` | q3 | kN/m2 | 1.2 | 1.6 | 6.3.1.2(8) |  |

##### Clause 6.3.1.2(10) (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `alpha-a.en-a0` | A0 | m2 | 10 | - | 6.3.1.2(10) |  |
| `alpha-a.en-min-cd` | α_A min C D |  | 0.6 | - | 6.3.1.2(10) |  |

##### Clause Annex A Table A.4 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `density.steel` | γ steel | kN/m3 | 77 | 78.5 | Annex A Table A.4 |  |

##### Clause Annex B (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `snow.drift-mu2-cap` | μ2 max |  | 4 | 2 | Annex B |  |

##### Clause E11.01 (2; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-01.1` | ? |  | - | - | E11.01 | en1990-en1991-target.md:175: recommended : concrete normal 24.0 (reinforced +1 ⇒ 25.0, unhardened +1), lightweight 9-20, steel 77.0-78.5 (structural 7 |
| `e11-01.2` | ? |  | - | - | E11.01 | en1990-en1991-target.md:175: DE NA references DIN 1055-1 for the full table . |

##### Clause E11.06 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-06.1` | ? |  | - | - | E11.06 | en1990-en1991-target.md:189: DE 0.0? NA-DE H roof: loads only point Q_k=1.0 kN on 0.05×0.05 and the snow load (no area imposed load when snow given) |

##### Clause E11.07 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-07.2` | ? |  | - | - | E11.07 | en1990-en1991-target.md:190: Dynamic magnification φ = 1.4 (pneumatic) / 2.0 (solid) . |

##### Clause E11.09 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-09.1` | ? |  | - | - | E11.09 | en1990-en1991-target.md:192: Annex B, 6.5 / Fahrzeuge auf Bruecken und Parkhaeusern**: car park F: Q_k 20 kN axle (cat F) / 90 kN (cat G), footprint 2 |

##### Clause Table 6.1DE (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-03.3` | ? |  | - | - | Table 6.1DE | en1990-en1991-target.md:182: NA-DE Table 6.1DE : A1 (Spitzboden) 1.0/1.0 |

##### Clause Table 6.2 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-03.1` | ? |  | - | - | Table 6.2 | en1990-en1991-target.md:181: **E11.03 6.3.1.2 Table 6.2 / Nutzlasten Gebaeude** category table (q_k kN/m^2, Q_k kN), EN recommended with range : |

##### Clause Table 6.5 (1; DIN EN 1991-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11-07.1` | ? |  | - | - | Table 6.5 | en1990-en1991-target.md:190: FL6: net weight 21 → 110 kN, hoist load 10 → 60 kN, wheel dimensions, axle: Q_k = 1.4 × (net+hoist) dynamic? EN Table 6.5 |

#### EN 1991, part 1-2 (14)

##### Clause 1.0 (1; DIN EN 1991-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e90-17.1` | ? |  | - | - | 1.0 | en1990-en1991-target.md:89: DE NA fire: ψ1,1 ). out: E_dA per accidental case. edge: A_d may be a set (impact x, impact y) |

##### Clause 1.8 (1; DIN EN 1991-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-44.1` | ? |  | - | - | 1.8 | en1994-en1999-target.md:258: **C-44 Fire design framework ** — F:2, F:3. |

##### Clause 2.2 (5; DIN EN 1991-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e12-03.1` | fire_load_density_qf |  | - | - | 2.2 | en1990-en1991-target.md:203: δ_n = Π δ_ni (sprinkler δn1=0.61, independent water supplies 0.87/0.7, automatic detection by heat 0.87, by smoke 0.73, a |
| `e12-03.2` | fire_load_density_qf |  | - | - | 2.2 | en1990-en1991-target.md:203: MJ/m^2 . δ_q1 by area: 25 m^2: 1.10, 250: 1.50, 2500: 1.90, 5000: 2.00, 10000: 2.13 (interpolate linear in log A [E]) |
| `e12-03.3` | fire_load_density_qf |  | - | - | 2.2 | en1990-en1991-target.md:203: paint 1.66, fireworks 1.66? . |
| `e12-03.4` | fire_load_density_qf |  | - | - | 2.2 | en1990-en1991-target.md:203: NA-DE: q_f,k per DIN EN 1991-1-2/NA Table NA.E.1 |
| `e12-10.1` | ? |  | - | - | 2.2 | en1990-en1991-target.md:212: NA-DE replacements for Annex E, F and the Table-based DE fire-load approach . |

##### Clause 3.1 Eq. 3.2 (3; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-2.configuration-factor` | Phi |  | 1 | 1 | 3.1 Eq. 3.2 |  |
| `1-2.flame-emissivity` | eps_f |  | 1 | 1 | 3.1 Eq. 3.2 |  |
| `1-2.member-emissivity` | eps_m |  | 0.8 | 0.8 | 3.1 Eq. 3.2 |  |

##### Clause Annex E (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-2.combustion-factor` | m |  | 0.8 | 0.8 | Annex E |  |

##### Clause E12.04 (1; DIN EN 1991-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e12-04.1` | ? |  | - | - | E12.04 | en1990-en1991-target.md:204: **E12.04 3.3.1.1 + Annex F / Aequivalente Branddauer** f [N]: t_e,d = q_f,d k_b w_f (F.1); k_b = 0.07 (0.09 for ..., depe |

##### Clause E12.05 (1; DIN EN 1991-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e12-05.1` | ? |  | - | - | E12.05 | en1990-en1991-target.md:205: 15000 y^−3.7 if y ≥ 1.0 (W/m^2) with y = (r+H+z′)/(L_h+H+z′) . out: gas temperature field / flux on beam surface. edge: D |

##### Clause E12.08 (1; DIN EN 1991-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e12-08.1` | ? |  | - | - | E12.08 | en1990-en1991-target.md:209: radiation flux φ_f ε_f σ T^4. data. |

#### EN 1991, part 1-3 (12)

##### Clause 1.6 (1; DIN EN 1991-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13-03.1` | ? |  | - | - | 1.6 | en1990-en1991-target.md:226: with snow guards or parapets μ1 = 0.8 . |

##### Clause 2.3 (3; DIN EN 1991-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13-06.1` | ? |  | - | - | 2.3 | en1990-en1991-target.md:230: NA-DE: μw limit 0.8 ≤ μ2 ≤ 2.0 ? (DE note: γ=2 kN/m^3 and cap μ2 ≤ 2.0 for h/b limits) drift length l_s = 2h with 5 m ≤ l |
| `na-de.2` | ? |  | - | - | 2.3 | en1990-en1991-target.md:224: Zone assignment from municipality (data asset: `snow_zone_by_postal_code/AGS`, ) |
| `na-de.3` | ? |  | - | - | 2.3 | en1990-en1991-target.md:224: EN recommended: s_k by CEN map zones (Annex C): Alpine Region s_k = (0.642 Z + 0.009)(1 + (A/728)²) ≥ 0.? , Central East |

##### Clause 5.3.5 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.cylindrical-mu3-cap` | mu3 cap cylindrical |  | 2 | 2 | 5.3.5 |  |

##### Clause 5.3.6 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.drift-mu-w-reading` | mu_w limit reading |  | 0 | 0 | 5.3.6 |  |
| `1-3.sliding-share` | sliding share |  | 0.5 | 0.5 | 5.3.6 |  |

##### Clause E13.01 (2; DIN EN 1991-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13-01.1` | ? |  | - | - | E13.01 | en1990-en1991-target.md:216: < 1.0 for U > 1 W/m^2K heated glazed ), s_k. ndp: C_e,C_t |
| `e13-01.2` | ? |  | - | - | E13.01 | en1990-en1991-target.md:216: NA-DE: C_e=1.0 (normal topography), no reduction for windswept , C_t=1.0 . |

##### Clause E13.04 (1; DIN EN 1991-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13-04.1` | ? |  | - | - | E13.04 | en1990-en1991-target.md:227: μ2 limited by γ h/s_k and 0.8 ≤ μ2 ≤ 2.0 . |

##### Clause E13.05 (1; DIN EN 1991-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13-05.1` | ? |  | - | - | E13.05 | en1990-en1991-target.md:228: **E13.05 5.3.5 / Zylindrische Daecher** μ3 = 0.2 + 10 h/b ≤ 2.0 (Fig 5.6), load case undrifted 0.8 uniform (μ1 = 0.8 acro |

##### Clause E13.11 (1; DIN EN 1991-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13-11.1` | ? |  | - | - | E13.11 | en1990-en1991-target.md:235: **E13.11 Annex E / Wichte des Schnees** γ: fresh 1.0 kN/m^3, settled several hours/days 2.0, settled weeks/months 2.5-3.5 |

#### EN 1991, part 1-4 (51)

##### Clause 1.10 (3; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-02.4` | ? |  | - | - | 1.10 | en1990-en1991-target.md:248: WZ4 0.95/1.15/1.30 kN/m^2 . |
| `e14-02.5` | ? |  | - | - | 1.10 | en1990-en1991-target.md:248: WZ4 1.25/1.40/1.50 . |
| `e14-02.6` | ? |  | - | - | 1.10 | en1990-en1991-target.md:248: Nordsee Inseln: WZ4 only 1.40/1.55/1.65? . |

##### Clause 1.25 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l569.1` | ? |  | - | - | 1.25 | en1990-en1991-target.md:569: E14-15: ½·1.25·625·2.6·120/1000 = 121.9 kN (C is a input). |

##### Clause 2.4 (8; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-02.1` | ? |  | - | - | 2.4 | en1990-en1991-target.md:243: NA-DE additionally uses *Mischprofile* (profile combining terrain categories II–III for inland, Table NA.B.2) and simplif |
| `e14-02.2` | ? |  | - | - | 2.4 | en1990-en1991-target.md:246: - Nordseeinseln / offshore: terrain 0/I with full EN profile |
| `e14-02.3` | mixed_terrain_upwind |  | - | - | 2.4 | en1990-en1991-target.md:247: existing `mixed_terrain_upwind`, `mixed_terrain_distance` map to the EN 'Annex A.2 transition between terrain roughness' |
| `e14-07.1` | ? |  | - | - | 2.4 | en1990-en1991-target.md:260: NA-DE: tables identical, plus 'DE: Windlast on Windseitige Wand D': NA gives q_p simplified . |
| `e14-08.1` | ? |  | - | - | 2.4 | en1990-en1991-target.md:275: Representative (θ=0°, α=15°): F −0.9/+0.2 (pressure/suction pair: both load cases must be evaluated), G −0.8/+0.2, H −0.3 |
| `e14-12.5` | ? |  | - | - | 2.4 | en1990-en1991-target.md:282: DE: ISO 10137 ). |
| `e14-14.1` | ? |  | - | - | 2.4 | en1990-en1991-target.md:285: see ), K = mode-shape/correlation factor (Table E.5: cantilever 0.13, K_w effective correlation length factor (Table E.6/ |
| `e14-14.2` | ? |  | - | - | 2.4 | en1990-en1991-target.md:285: Ovalling (E.3) for shells: v_crit,ov = ... . ok: vortex excited amplitudes y_max ≤ b/30? NA |

##### Clause 5.3 (3; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-11.1` | ? |  | - | - | 5.3 | en1990-en1991-target.md:279: Rectangular cross section: c_f = c_f,0 ψ_r ψ_λ (7.9) with c_f,0 from Fig 7.23 (function of d/b, digitised curve data asse |
| `e14-11.2` | ? |  | - | - | 5.3 | en1990-en1991-target.md:279: ψ_r corner rounding (1 − 1.9 r/b ≥ 0.5 ) |
| `e14-11.3` | ? |  | - | - | 5.3 | en1990-en1991-target.md:279: l ≤ 15 m: λ = min(2 l/b, 70) ). |

##### Clause 6.1 (4; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-12.1` | ? |  | - | - | 6.1 | en1990-en1991-target.md:281: δ = δ_s + δ_a + δ_d (logarithmic decrements: steel buildings 0.05 welded? Table F.2 : reinforced concrete 0.10, steel wel |
| `e14-12.2` | ? |  | - | - | 6.1 | en1990-en1991-target.md:281: chimneys with circular cross-section h < 60 m and h < 6.5 d (Fig 6.1 diagrams: c_s c_d values 0.85–1.0 for steel and conc |
| `e14-12.3` | ? |  | - | - | 6.1 | en1990-en1991-target.md:281: criterion for 'Schwingungsanfaelligkeit' (n_1 and δ-dependent): if v_m(z_s) / (n_1 b) ... a structure is sensitive if the |
| `e14-12.4` | ? |  | - | - | 6.1 | en1990-en1991-target.md:281: for non-sensitive: gust factor method G (the DE gust factor G = 1 + 2 g I_v) alternative . |

##### Clause 6.2 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.cscd-simple-height` | h limit c_s c_d = 1 | m | 15 | 15 | 6.2 |  |

##### Clause 7.2.9 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.c-pi.1` | c_pi overpressure |  | 0.2 | 0.2 | 7.2.9 |  |
| `1-4.c-pi.2` | c_pi suction |  | -0.3 | -0.3 | 7.2.9 |  |

##### Clause 8.3.1 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `8.traffic-rail-add` | d rail traffic | m | 4 | 4 | 8.3.1 |  |
| `8.traffic-road-add` | d road traffic | m | 2 | 2 | 8.3.1 |  |

##### Clause 8.3.3 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `8.cfz` | c_f,z |  | 0.9 | 0.9 | 8.3.3 |  |

##### Clause 8.3.4 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `8.longitudinal` | F_Wy/F_Wx |  | 0.25 | 0.25 | 8.3.4 |  |

##### Clause E14.01 (4; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-01.1` | ? |  | - | - | E14.01 | en1990-en1991-target.md:241: DE: c_dir = c_season = 1.0 . |
| `e14-01.2` | ? |  | - | - | E14.01 | en1990-en1991-target.md:241: Zone from municipality (data asset, PLZ/AGS) |
| `e14-01.3` | ? |  | - | - | E14.01 | en1990-en1991-target.md:241: Nordsee islands/offshore: WZ4-offshore by NA . |
| `e14-01.4` | ? |  | - | - | E14.01 | en1990-en1991-target.md:241: above, NA table . |

##### Clause E14.03 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-03.1` | orography_factor |  | - | - | E14.03 | en1990-en1991-target.md:249: NA-DE: orography effect only for exposed sites with Φ>0.05 and h-limits (c_o = 1 in the usual case |

##### Clause E14.04 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-04.1` | ? |  | - | - | E14.04 | en1990-en1991-target.md:250: DE mixed profile gives I_v implicit: I_v = σ_v/v_m ≈ 0.4 for z ≤ 7? . |

##### Clause E14.09 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-09.1` | ? |  | - | - | E14.09 | en1990-en1991-target.md:276: **E14.09 7.2.9 / Innendruck** w_i = q_p(z_i) c_pi. c_pi from Fig 7.13 versus μ = Σ(open area where c_pe ≤ 0)/Σ(all open a |

##### Clause E14.13 (2; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-13.1` | ? |  | - | - | E14.13 | en1990-en1991-target.md:283: plateau 1.3 for b/d_tot ≥ 4, rising towards 2.4 for b/d_tot ≈ 0.5 ) . |
| `e14-13.2` | ? |  | - | - | E14.13 | en1990-en1991-target.md:283: Vertical force F_Wz = ½ ρ v_b² C_z A_ref,z with C_z = ±0.9 (rec simplified) , Longitudinal force 25 %/50 % of F_Wx. out: |

##### Clause E14.15 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-15.1` | ? |  | - | - | E14.15 | en1990-en1991-target.md:286: log decrements Table F.2: reinforced concrete buildings 0.10, steel buildings 0.05, mixed 0.08, steel chimneys welded unl |

##### Clause E.1.3.1 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.kinematic-viscosity` | nu air | m2/s | 1.5e-05 | 1.5e-05 | E.1.3.1 |  |
| `1-4.vortex-onset` | v_crit/v_m onset factor |  | 1.25 | 1.25 | E.1.3.1 |  |

##### Clause Fig. 8.3 (4; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `8.cfx0-b-narrow` | b/d_tot upper plateau limit |  | 0.5 | 0.5 | Fig. 8.3 |  |
| `8.cfx0-b-wide` | b/d_tot lower plateau limit |  | 4 | 4 | Fig. 8.3 |  |
| `8.cfx0-narrow` | c_f,x0 at b/d_tot <= 0.5 |  | 2.4 | 2.4 | Fig. 8.3 |  |
| `8.cfx0-wide` | c_f,x0 at b/d_tot >= 4 |  | 1.3 | 1.3 | Fig. 8.3 |  |

##### Clause NA.A.1 Table NA.A.1 (4; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `vb0.zone1` | v_b,0 zone 1 | m/s | - | 22.5 | NA.A.1 Table NA.A.1 |  |
| `vb0.zone2` | v_b,0 zone 2 | m/s | - | 25 | NA.A.1 Table NA.A.1 |  |
| `vb0.zone3` | v_b,0 zone 3 | m/s | - | 27.5 | NA.A.1 Table NA.A.1 |  |
| `vb0.zone4` | v_b,0 zone 4 | m/s | - | 30 | NA.A.1 Table NA.A.1 |  |

##### Clause Table 7. (2; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-11.5` | ? |  | - | - | Table 7. | en1990-en1991-target.md:279: Lattice towers Table 7.? skip or . |
| `e14-16.1` | ? |  | - | - | Table 7. | en1990-en1991-target.md:287: vertical cylinders on roof, hip/domed roofs Table 7.? . |

##### Clause Table 7.9 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-10.1` | ? |  | - | - | Table 7.9 | en1990-en1991-target.md:277: free-standing walls Table 7.9 (zones A, B, C, D by l/h), c_p,net values e.g. solid wall φ=1 : A 2.3, B 1.4, C 1.2, D 1.2 |

##### Clause Table 7.13 (1; DIN EN 1991-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14-11.4` | ? |  | - | - | Table 7.13 | en1990-en1991-target.md:279: Circular cylinders Fig 7.28: c_f,0 vs R_e = b v(z_e)/ν, ν = 15 × 10⁻⁶ m²/s, v(z_e) = √(2 q_p/ρ) with equivalent roughness |

##### Clause Table 8.1 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `8.parapet-open-add` | d open parapet per side | m | 0.3 | 0.3 | Table 8.1 |  |

#### EN 1991, part 1-5 (36)

##### Clause 2.1 (2; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-900.1` | approx. with |  | - | - | 2.1 | en1993-target.md:477: `Delta T_epscf = -3 epsilon_cf[%]` approx. with `epsilon_cf = 100 t/(2 r)`) ****. |
| `v-900.2` | Tmd |  | - | - | 2.1 | en1993-target.md:477: NA-DE reference temperature may use `Tmd` from EN 1991-1-5 NA maps ****. |

##### Clause 2.5 (3; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15-01.1` | ? |  | - | - | 2.5 | en1990-en1991-target.md:293: Type 3 concrete: +8, +2 (Fig 6.1 approximations) . |
| `e15-01.2` | ? |  | - | - | 2.5 | en1990-en1991-target.md:293: NDP: T_min, T_max (isotherm maps): NA-DE (DIN EN 1991-1-5/NA, Tabelle): T_min = −24 °C, T_max = +37 °C for lower elevatio |
| `e15-01.3` | ? |  | - | - | 2.5 | en1990-en1991-target.md:293: regions of DE with locally lower values listed by NA map . |

##### Clause 6.1.5 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-5.omega-m` | omega_M |  | 0.75 | 0.75 | 6.1.5 |  |
| `1-5.omega-n` | omega_N |  | 0.35 | 0.35 | 6.1.5 |  |

##### Clause 7.6 (1; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l603.1` | ? |  | - | - | 7.6 | en1990-en1991-target.md:603: altitude correction Tmin 0.5K/100m @ 800m |

##### Clause E15.02 (3; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15-02.1` | ? |  | - | - | E15.02 | en1990-en1991-target.md:294: ballast 0.6/1.4? ). |
| `e15-02.2` | ? |  | - | - | E15.02 | en1990-en1991-target.md:294: Horizontal: ΔT_M,y = ±5 K (rec) . out: curvature κ = α ΔT_M / h |
| `e15-02.3` | ? |  | - | - | E15.02 | en1990-en1991-target.md:294: NA-DE: NA gives modified ΔT_M for composite/steel . |

##### Clause E15.03 (1; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15-03.1` | ? |  | - | - | E15.03 | en1990-en1991-target.md:295: **E15.03 6.2 / Differenzen zwischen Bauteilen** ΔT between main structure and tie/cable: 15 K (rec), between cables and d |

##### Clause E15.05 (1; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15-05.1` | ? |  | - | - | E15.05 | en1990-en1991-target.md:297: **E15.05 Annex C / Thermal expansion coefficient table** (α_T: concrete 10, steel 12, stainless 16, aluminium 24, masonry |

##### Clause E15.07 (1; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15-07.1` | ? |  | - | - | E15.07 | en1990-en1991-target.md:298: **E15.07 A.1 / Industrial: chimneys, silos** thermal gradient through shell (Annex D? 'cooling' ), stage . |

##### Clause Table 5.1 (1; DIN EN 1991-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15-04.1` | ? |  | - | - | Table 5.1 | en1990-en1991-target.md:296: T_in 20 °C winter 25 °C summer (Table 5.1) |

##### Clause Table 6.1 (10; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-5.dtm-cool.1` | dT_M,cool steel deck | K | 13 | 13 | Table 6.1 |  |
| `1-5.dtm-cool.2` | dT_M,cool composite deck | K | 18 | 18 | Table 6.1 |  |
| `1-5.dtm-cool.3` | dT_M,cool concrete box girder | K | 5 | 5 | Table 6.1 |  |
| `1-5.dtm-cool.4` | dT_M,cool concrete slab | K | 8 | 8 | Table 6.1 |  |
| `1-5.dtm-cool.5` | dT_M,cool concrete beam | K | 8 | 8 | Table 6.1 |  |
| `1-5.dtm-heat.1` | dT_M,heat steel deck | K | 18 | 18 | Table 6.1 |  |
| `1-5.dtm-heat.2` | dT_M,heat composite deck | K | 15 | 15 | Table 6.1 |  |
| `1-5.dtm-heat.3` | dT_M,heat concrete box girder | K | 10 | 10 | Table 6.1 |  |
| `1-5.dtm-heat.4` | dT_M,heat concrete slab | K | 15 | 15 | Table 6.1 |  |
| `1-5.dtm-heat.5` | dT_M,heat concrete beam | K | 15 | 15 | Table 6.1 |  |

##### Clause Table 6.2 (11; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-5.ksur-cool.0` | k_sur cooling surfacing 0 |  | 0.9 | 0.9 | Table 6.2 |  |
| `1-5.ksur-cool.1` | k_sur cooling surfacing 1 |  | 1.1 | 1.1 | Table 6.2 |  |
| `1-5.ksur-cool.2` | k_sur cooling surfacing 2 |  | 1 | 1 | Table 6.2 |  |
| `1-5.ksur-cool.3` | k_sur cooling surfacing 3 |  | 1.2 | 1.2 | Table 6.2 |  |
| `1-5.ksur-cool.4` | k_sur cooling surfacing 4 |  | 1.2 | 1.2 | Table 6.2 |  |
| `1-5.ksur-heat.0` | k_sur heating surfacing 0 |  | 0.7 | 0.7 | Table 6.2 |  |
| `1-5.ksur-heat.1` | k_sur heating surfacing 1 |  | 1.6 | 1.6 | Table 6.2 |  |
| `1-5.ksur-heat.2` | k_sur heating surfacing 2 |  | 1 | 1 | Table 6.2 |  |
| `1-5.ksur-heat.3` | k_sur heating surfacing 3 |  | 0.7 | 0.7 | Table 6.2 |  |
| `1-5.ksur-heat.4` | k_sur heating surfacing 4 |  | 0.7 | 0.7 | Table 6.2 |  |
| `1-5.ksur-other` | k_sur other decks |  | 1 | 1 | Table 6.2 |  |

#### EN 1991, part 1-6 (6)

##### Clause 1.0 (3; DIN EN 1991-1-6/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e16-02.1` | ? |  | - | - | 1.0 | en1990-en1991-target.md:304: used with 0.75? ) |
| `e16-02.2` | ? |  | - | - | 1.0 | en1990-en1991-target.md:304: Q_cf: loads from parts of the structure in temporary states (self-weight of precast elements, φ_dyn 1.1–1.5 for transport |
| `e16-02.3` | ? |  | - | - | 1.0 | en1990-en1991-target.md:304: fresh concrete as G (γ × h, 25 kN/m^3 reinforced) – pressure on formwork: p = γ h hydrostatic (for h ≤ ... ) with consist |

##### Clause E16.01 (1; DIN EN 1991-1-6/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e16-01.2` | ? |  | - | - | E16.01 | en1990-en1991-target.md:302: DE NA: Table NA.? with return periods modified . |

##### Clause Table 3.1 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e6-01.1` | T |  | - | - | Table 3.1 |  |

##### Clause Table 3.2 (1; DIN EN 1991-1-6/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e16-01.1` | ? |  | - | - | Table 3.2 | en1990-en1991-target.md:302: wind nearby: seasonal factor c_season Table 3.2 . |

#### EN 1991, part 1-7 (19)

##### Clause 4.3 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-7.impact-height-max` | impact height max | m | 1.5 | 1.5 | 4.3 |  |
| `1-7.impact-height-min` | impact height min | m | 0.5 | 0.5 | 4.3 |  |

##### Clause 4.5 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-7.forklift-factor` | F_d,x/W forklift |  | 5 | 5 | 4.5 |  |

##### Clause 6.0 (2; DIN EN 1991-1-7/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l621.1` | ? |  | - | - | 6.0 | en1990-en1991-target.md:621: both exceed 75 ⇒ min(...,75) = 75 under the 'lesser' reading . |
| `l621.2` | ? |  | - | - | 6.0 | en1990-en1991-target.md:621: D.2: 3 + 2.5 + 0.04/0.0016 = 30.5 kN/m^2 . |

##### Clause 7.8 (1; DIN EN 1991-1-7/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l631.1` | ? |  | - | - | 7.8 | en1990-en1991-target.md:631: pier 1000/500 |

##### Clause Annex C (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-7.vehicle-stiffness` | k | N/m | 300000 | 300000 | Annex C |  |

##### Clause Annex D (4; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-7.gas-base` | p_d base | kN/m2 | 3 | 3 | Annex D |  |
| `1-7.gas-vent-coefficient` | p_d vent term | kN/m2 | 0.04 | 0.04 | Annex D |  |
| `1-7.gas-volume-max` | V max | m3 | 1000 | 1000 | Annex D |  |
| `1-7.gas-volume-min` | V min | m3 | 5 | 5 | Annex D |  |

##### Clause E17.02 (4; DIN EN 1991-1-7/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e17-02.1` | ? |  | - | - | E17.02 | en1990-en1991-target.md:312: **E17.02 4.5 + Annex C.? / Anprall von Gabelstaplern** F_d,x = 5 W (W = net weight of loaded forklift, 5 W for rigid, 2.5 |
| `e17-02.2` | ? |  | - | - | E17.02 | en1990-en1991-target.md:312: **E17.03 4.? / Entgleisung (rail traffic)**, Tabellen 4.4–4.5 F_dx by v & distance |
| `e17-02.3` | ? |  | - | - | E17.02 | en1990-en1991-target.md:312: **E17.04 4.6 / Ship impact** Annex C.4 (inland waterways F_dx: Table C.2 ship class I–VII with forward 1000..? kN) |
| `e17-02.4` | ? |  | - | - | E17.02 | en1990-en1991-target.md:312: F_d = 3 m? rec: F_d = C √m with C = 3 kN/√kg, m in kg for rooftop helipads |

##### Clause Table 4.1 (4; DIN EN 1991-1-7/NA, DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-7.lateral-ratio` | F_dy/F_dx |  | 0.5 | 0.5 | Table 4.1 |  |
| `e17-01.1` | ? |  | - | - | Table 4.1 | en1990-en1991-target.md:311: Rec : Table 4.1 F_d,x / F_d,y (kN): motorways & country roads 1000 / 500 |
| `e17-01.2` | ? |  | - | - | Table 4.1 | en1990-en1991-target.md:311: For overhead decks: F_d,x = 500 kN, F_d,y = 250 kN? (4.3.1(2)) |
| `e17-01.3` | ? |  | - | - | Table 4.1 | en1990-en1991-target.md:311: NA-DE: Table NA.4.1 (DIN EN 1991-1-7/NA: F_d,x = 1000 kN (motorways, Bundesstrassen, speed ≥ 80), 500 kN?...). |

#### EN 1991, part 2 (21)

##### Clause 1.9 (1; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-48.1` | ? |  | - | - | 1.9 | en1994-en1999-target.md:268: **C-48 Bridge actions and combinations ** — 2:2, EN 1991-2 load models (LM1/LM2/LM3/ LM71): lane definition |

##### Clause 4.3.2 Fig. 4.2b (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `2.lm1-axle-spacing` | axle spacing | m | 1.2 | 1.2 | 4.3.2 Fig. 4.2b |  |

##### Clause 5.4 (1; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-01.1` | ? |  | - | - | 5.4 | en1990-en1991-target.md:320: NA-DE: same lane layout . |

##### Clause 6.3.2 (8; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `2.rail-alpha.1` | alpha 1 |  | 0.75 | 0.75 | 6.3.2 |  |
| `2.rail-alpha.2` | alpha 2 |  | 0.83 | 0.83 | 6.3.2 |  |
| `2.rail-alpha.3` | alpha 3 |  | 0.91 | 0.91 | 6.3.2 |  |
| `2.rail-alpha.4` | alpha 4 |  | 1 | 1 | 6.3.2 |  |
| `2.rail-alpha.5` | alpha 5 |  | 1.1 | 1.1 | 6.3.2 |  |
| `2.rail-alpha.6` | alpha 6 |  | 1.21 | 1.21 | 6.3.2 |  |
| `2.rail-alpha.7` | alpha 7 |  | 1.33 | 1.33 | 6.3.2 |  |
| `2.rail-alpha.8` | alpha 8 |  | 1.46 | 1.46 | 6.3.2 |  |

##### Clause ? (1; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-122.1` | Qd |  | - | - | T-122 Stress-laminated decks and bridge decks (EN  | en1995-en1996-target.md:433: check bolt/screw wheel loads on plank with `Qd`, `kdis`, wheel contact `a × b = 400 × 400 mm?` (EN 1991-2 load model 1 wh |

##### Clause Annex A (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `2.lm3-axle-spacing` | LM3 axle line spacing | m | 1.5 | 1.5 | Annex A |  |
| `2.lm3-vehicle-width` | LM3 vehicle width | m | 3 | 3 | Annex A |  |

##### Clause E2.02 (1; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-02.1` | ? |  | - | - | E2.02 | en1990-en1991-target.md:321: NA-DE (DIN EN 1991-2/NA, Tabelle NA.4.? ) : values the NA sets are the NDP |

##### Clause E2.04 (1; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-04.1` | ? |  | - | - | E2.04 | en1990-en1991-target.md:323: pressure area 0.15 m^2? NA-DE: LM3 SLW 60 / SLW 30 per DIN-Fachbericht (SLW 60 = 600 kN, axle 100 kN, 4 axles?) . |

##### Clause E2.07 (4; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-07.1` | fem |  | - | - | E2.07 | en1990-en1991-target.md:326: Annex B equivalent damage λ-method |
| `e2-07.2` | fem |  | - | - | E2.07 | en1990-en1991-target.md:326: **E2.11 4.7 / Accidental: vehicle collision on supports**, deck: Fdx = 1000/500 kN (4.7.3.1 ), on soffit: 500 kN? |
| `e2-07.3` | fem |  | - | - | E2.07 | en1990-en1991-target.md:326: horizontal Q_flk = 10 % of total vertical load, ≥ 10 kN? |
| `e2-07.5` | fem |  | - | - | E2.07 | en1990-en1991-target.md:326: A2 N=17 D=19 d=3.5 P=200 … A10 N=13 D=24 d=2.0 P=210 |

##### Clause Table 6.11 (1; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-07.4` | fem |  | - | - | Table 6.11 | en1990-en1991-target.md:326: 35 kN/m for SW/2 [N]. **E2.18 6.7 / Aerodynamic effects** from passing trains (kPa p_k = ±0.3–2.? ) . **E2.19 6.8 / Derai |

#### EN 1991, part 3 (15)

##### Clause 1.0 (3; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-01.1` | ? |  | - | - | 1.0 | en1990-en1991-target.md:336: - φ_5 = 1.0 ... 1.5 (linear or less: 1.0 ≤ φ_5 ≤ 2.0 depending on drive: 1.5 for smooth, 2.0 for abrupt) ; |
| `e3-01.2` | ? |  | - | - | 1.0 | en1990-en1991-target.md:337: - φ_6 test load factor 1.0? (dynamic test 1.1?) |
| `e3-01.3` | ? |  | - | - | 1.0 | en1990-en1991-target.md:337: 1.25 – 1.75 non-linear . |

##### Clause 1.05 (1; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l653.1` | ? |  | - | - | 1.05 | en1990-en1991-target.md:653: drive factors . |

##### Clause 2.7 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `3.phi-5` | phi_5 |  | 1.5 | 1.5 | 2.7 |  |

##### Clause 5.3.2 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `3.drive-friction-mu` | mu |  | 0.2 | 0.2 | 5.3.2 |  |

##### Clause E3.02 (1; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-02.1` | ? |  | - | - | E3.02 | en1990-en1991-target.md:338: Group 2: ..., see table , to be encoded as `CraneLoadGroup{ id, factors{φ1..φ7} per Q_C, Q_H, H_L, H_T, H_S, H_B }`. |

##### Clause E3.03 (2; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-03.1` | ? |  | - | - | E3.03 | en1990-en1991-target.md:339: **E3.03 2.7 + 2.8 / Horizontale Lasten**: longitudinal H_L,i = φ_5 K / n_r (K = μ ΣQ_r,max... the drive force K = μ ΣQ_r, |
| `e3-03.3` | ? |  | - | - | E3.03 | en1990-en1991-target.md:339: Tilting/uplift: H_TA from tilting of crane or lift . |

##### Clause E3.04 (1; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-04.1` | ? |  | - | - | E3.04 | en1990-en1991-target.md:340: out-of-service wind per E14 (crane wheel loads Q_r with wind) . |

##### Clause E3.05 (1; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-05.1` | ? |  | - | - | E3.05 | en1990-en1991-target.md:341: E3.06 Annex B: dynamic factors... . |

##### Clause E3.07 (1; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-07.1` | ? |  | - | - | E3.07 | en1990-en1991-target.md:342: **E3.07 5 / Maschinelle Einwirkungen: Dynamic loads from machinery** (6): static equivalent for rotating machines F_dyn = |

##### Clause Table 2. (2; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-03.2` | ? |  | - | - | Table 2. | en1990-en1991-target.md:339: f = 0.3(1 − e^{−250 α}) ≤ 0.3 with α = α_F + α_V + α_0 (skew angle, rad) [N], λ_s coefficients from Table 2.? (guide mean |
| `e3-07.2` | ? |  | - | - | Table 2. | en1990-en1991-target.md:343: two-crane combination ψ factors (Table 2.? 'two cranes: 100 % + 50 %?' ). |

##### Clause Table 2.x (1; DIN EN 1991-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-01.4` | ? |  | - | - | Table 2.x | ticket NORM-COMPLETE wave2 en1991 v2: Crane class to static wheel load Q_r (HC1 50, HC2 100, HC3 160, HC4 250 kN) is a modelling placeholder, not a st |

#### EN 1991, part 4 (8)

##### Clause 2.10 (2; DIN EN 1991-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e4-01.3` | ? |  | - | - | 2.10 | en1990-en1991-target.md:347: load multipliers: phf, pwf with K = 1.2 K_m? μ = 1.15? etc. . |
| `e4-01.4` | ? |  | - | - | 2.10 | en1990-en1991-target.md:347: Hopper: 5.4: p_nf for hopper wall inclination β (Eqs 6.? 6.5-6.7), p_vf at transition F factor: p_v = (γ h_o ... ) |

##### Clause 5.2.4 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `4.patch-factor` | p_pf / p_hf |  | 0.2 | 0.2 | 5.2.4 |  |

##### Clause E4.01 (2; DIN EN 1991-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e4-01.1` | ? |  | - | - | E4.01 | en1990-en1991-target.md:346: K_s?), a_K = 1.2?) and the extreme values: γ_u = 1.15 γ_m? . |
| `e4-01.2` | ? |  | - | - | E4.01 | en1990-en1991-target.md:346: Table E.1 for ~20 bulk solids . **E4.03 5.2 (5.1–5.5) / Janssen filling load, vertical walls** |

##### Clause E4.04 (2; DIN EN 1991-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e4-04.1` | ? |  | - | - | E4.04 | en1990-en1991-target.md:348: The simplified: C_h = 1.15, C_w = 1.10 for AAC2 slender . |
| `e4-04.2` | ? |  | - | - | E4.04 | en1990-en1991-target.md:348: E4.05 patch load p_p: p_pf = 0.2 p_hf (rec: 'patch load pp = 0.2 p_hf,max for ... (5.5.?)') with patch extent s = π d_c / |

##### Clause §5.2.4 / §6 (1; DIN EN 1991-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e4-02.1` | ? |  | - | - | §5.2.4 / §6 | ticket NORM-COMPLETE wave2 en1991 v2: Eccentric filling (pressure increase for e_f/d_c) and hopper wall pressures (steep/shallow hopper limit, hopper |

#### EN 1991, part not stated (37)

##### Clause ? (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l482.1` | eurocodepy |  | - | - | 6. Verification strategy (tests and third-party or | en1990-en1991-target.md:482: EN-recommended terrain II/III with `eurocodepy` if available ). |
| `l491.1` | [V] |  | - | - | 7. Worked examples (reference values for tests) | en1990-en1991-target.md:491: Annex is NA-DE where Germany specific, otherwise EN (recommended, data still to be keyed). |

##### Clause Table 6.1 (3; DIN EN 1991-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-14.1` | ? |  | - | - | Table 6.1 | ticket NORM-COMPLETE wave2 en1991 rail: LM71: Q_vk = 250 kN axles (4 x, 1.6 m spacing, a = 0.8 m lead and trail), q_vk = 80 kN/m, classification facto |
| `e2-14.2` | ? |  | - | - | Table 6.1 | ticket NORM-COMPLETE wave2 en1991 rail: SW/0: q_vk = 133 kN/m, a = 15.0 m, c = 5.3 m, times alpha, continuous girders; widely published, not checked a |
| `e2-14.3` | ? |  | - | - | Table 6.1 | ticket NORM-COMPLETE wave2 en1991 rail: SW/2: q_vk = 150 kN/m, a = 25.0 m, c = 7.0 m, applied without alpha; widely published, not checked against the |

##### Clause e2 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2.na-factors` | e2.na-factors |  | - | - | e2 |  |
| `e2.rail` | e2.rail |  | - | - | e2 |  |

##### Clause e3 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3.cranes` | e3.cranes |  | - | - | e3 |  |

##### Clause e4 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e4.silo` | e4.silo |  | - | - | e4 |  |

##### Clause e11 (6; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e11.alpha` | e11.alpha |  | - | - | e11 |  |
| `e11.barrier` | e11.barrier |  | - | - | e11 |  |
| `e11.density` | e11.density |  | - | - | e11 |  |
| `e11.forklift` | e11.forklift |  | - | - | e11 |  |
| `e11.na-table` | e11.na-table |  | - | - | e11 |  |
| `e11.partition` | e11.partition |  | - | - | e11 |  |

##### Clause e12 (5; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e12.annex-a` | e12.annex-a |  | - | - | e12 |  |
| `e12.annex-c` | e12.annex-c |  | - | - | e12 |  |
| `e12.annex-e` | e12.annex-e |  | - | - | e12 |  |
| `e12.annex-f` | e12.annex-f |  | - | - | e12 |  |
| `e12.firecurves` | e12.firecurves |  | - | - | e12 |  |

##### Clause e13 (5; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e13.drift` | e13.drift |  | - | - | e13 |  |
| `e13.en-annex-c` | e13.en-annex-c |  | - | - | e13 |  |
| `e13.na-zones` | e13.na-zones |  | - | - | e13 |  |
| `e13.overhang` | e13.overhang |  | - | - | e13 |  |
| `e13.weights` | e13.weights |  | - | - | e13 |  |

##### Clause e14 (6; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e14.annex-b` | e14.annex-b |  | - | - | e14 |  |
| `e14.annex-e` | e14.annex-e |  | - | - | e14 |  |
| `e14.bridge` | e14.bridge |  | - | - | e14 |  |
| `e14.cpe-flat` | e14.cpe-flat |  | - | - | e14 |  |
| `e14.cpe-pitched` | e14.cpe-pitched |  | - | - | e14 |  |
| `e14.na-profile` | e14.na-profile |  | - | - | e14 |  |

##### Clause e15 (2; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e15.altitude` | e15.altitude |  | - | - | e15 |  |
| `e15.shifts` | e15.shifts |  | - | - | e15 |  |

##### Clause e16 (1; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e16.returns` | e16.returns |  | - | - | e16 |  |

##### Clause e17 (3; DIN EN 1991/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e17.blast` | e17.blast |  | - | - | e17 |  |
| `e17.impact` | e17.impact |  | - | - | e17 |  |
| `e17.robust` | e17.robust |  | - | - | e17 |  |

### EN 1992 (194)

#### EN 1992, part 1-1 (123)

##### Clause 2.4.2.4 (6; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-c-fat` | gamma_c_fat |  | 1.5 | 1.5 | 2.4.2.4 |  |
| `gamma-s` | gamma_s |  | 1.15 | 1.15 | 2.4.2.4 |  |
| `gamma-s-acc` | gamma_s_acc |  | 1 | 1 | 2.4.2.4 |  |
| `gamma-s-fat` | gamma_s_fat |  | 1.15 | 1.15 | 2.4.2.4 |  |
| `m-01.gamma-c-accidental` | gamma_c_acc |  | 1.2 | 1.3 | 2.4.2.4 | en1992-target.md:28: Accidental. |
| `m-01.gamma-c-persistent` | gamma_c |  | 1.5 | 1.5 | 2.4.2.4 | en1992-target.md:28: Persistent and transient. |

##### Clause 3.1(4) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-05.2` | ? |  | - | - | 3.1(4) | en1994-en1999-target.md:74: for εcs simplified values (0.3 per mille typical). |

##### Clause 3.1.6 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `bridge-alpha-cc-lt` | bridge_alpha_cc_lt |  | 1 | 0.8 | 3.1.6 |  |

##### Clause 3.1.6(1) (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-01.alpha-cc-accidental` | alpha_cc_acc |  | 1 | 0.85 | 3.1.6(1) | en1992-target.md:28: Accidental situation; spec text says 1.0 for DE, repository keeps 0.85. **conflict** |
| `m-01.alpha-cc-persistent` | alpha_cc |  | 1 | 0.85 | 3.1.6(1) | en1992-target.md:28: Persistent and transient design situations. |

##### Clause 3.1.6(2) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-01.alpha-ct` | alpha_ct |  | 1 | 0.85 | 3.1.6(2) | en1992-target.md:28: Tensile strength factor. |

##### Clause 4.4.1 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-inst` | gamma_inst |  | 1 | 1 | 4.4.1 |  |
| `liquid-cover-add` | liquid_cover_add |  | 0 | 5 | 4.4.1 |  |

##### Clause 4.4.1.2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `bridge-cover-chloride` | bridge_cover_chloride |  | 0 | 10 | 4.4.1.2 |  |
| `cmin-dur-table` | cmin_dur_table |  | 0 | 0 | 4.4.1.2 |  |

##### Clause 4.4.1.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `delta-c-dev` | delta_c_dev |  | 10 | 15 | 4.4.1.3 |  |

##### Clause 4.5.1 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `spalling-add-mm` | spalling_add_mm |  | 10 | 10 | 4.5.1 |  |

##### Clause 5.5(4) (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `redistribution-k1` | redistribution_k1 |  | 0.44 | 0.64 | 5.5(4) |  |
| `redistribution-k2` | redistribution_k2 |  | 1.25 | 0.8 | 5.5(4) |  |

##### Clause 5.6.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `plastic-rotation-model` | plastic_rotation_model |  | 0 | 0 | 5.6.3 |  |

##### Clause 5.7 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-o-nl` | gamma_o_nl |  | 1.2 | 1.2 | 5.7 |  |

##### Clause 5.8.3.1 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lambda-lim-de-n` | lambda_lim_de_n |  | 0 | 0.41 | 5.8.3.1 |  |

##### Clause 5.8.3.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k1-global` | k1_global |  | 0.31 | 0.31 | 5.8.3.3 |  |

##### Clause 5.8.6 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-ce` | gamma_cE |  | 1.2 | 1.2 | 5.8.6 |  |

##### Clause 5.10.2.1 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-k1` | p_k1 |  | 0.8 | 0.8 | 5.10.2.1 |  |
| `p-k2` | p_k2 |  | 0.9 | 0.9 | 5.10.2.1 |  |

##### Clause 5.10.3 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-k7` | p_k7 |  | 0.75 | 0.75 | 5.10.3 |  |
| `p-k8` | p_k8 |  | 0.85 | 0.85 | 5.10.3 |  |

##### Clause 5.10.5.2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `friction-k` | friction_k |  | 0.005 | 0.005 | 5.10.5.2 |  |
| `friction-mu` | friction_mu |  | 0.19 | 0.19 | 5.10.5.2 |  |

##### Clause 5.10.8 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `unbonded-dsigma` | unbonded_dsigma |  | 100 | 100 | 5.10.8 |  |

##### Clause 5.10.9 (4; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-r-inf` | p_r_inf |  | 0.95 | 0.95 | 5.10.9 |  |
| `p-r-inf-unbonded` | p_r_inf_unbonded |  | 0.9 | 0.9 | 5.10.9 |  |
| `p-r-sup` | p_r_sup |  | 1.05 | 1.05 | 5.10.9 |  |
| `p-r-sup-unbonded` | p_r_sup_unbonded |  | 1.1 | 1.1 | 5.10.9 |  |

##### Clause 6.2.1.4(6) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-10.1` | ? |  | - | - | 6.2.1.4(6) | en1994-en1999-target.md:105: **C-10 Non-linear resistance ** — cl. 6.2.1.4(6): "Non-linear resistance to bending" / DE: "Nichtlineare Tragfähigkeit". |

##### Clause 6.2.2(1) (4; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `crd-c-shear` | crd_c_shear |  | 0.18 | 0.15 | 6.2.2(1) |  |
| `k1-shear` | k1_shear |  | 0.15 | 0.12 | 6.2.2(1) |  |
| `vmin-shear` | vmin_shear |  | 0.035 | 0.0525 | 6.2.2(1) |  |
| `vmin-shear-d800` | vmin_shear_d800 |  | 0.035 | 0.0375 | 6.2.2(1) |  |

##### Clause 6.2.3(2) (4; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `cot-theta-a` | cot_theta_a |  | 2.5 | 1.2 | 6.2.3(2) |  |
| `cot-theta-b` | cot_theta_b |  | 0 | 1.4 | 6.2.3(2) |  |
| `cot-theta-max` | cot_theta_max |  | 2.5 | 3 | 6.2.3(2) |  |
| `vrdcc-c` | vrdcc_c |  | 0 | 0.5 | 6.2.3(2) |  |

##### Clause 6.2.3(103) (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `alpha-cw` | alpha_cw |  | 1 | 1 | 6.2.3(103) |  |
| `nu1-factor` | nu1_factor |  | 1 | 0.75 | 6.2.3(103) |  |

##### Clause 6.4.3(6) (4; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `beta-corner` | beta_corner |  | 1.5 | 1.5 | 6.4.3(6) |  |
| `beta-edge` | beta_edge |  | 1.4 | 1.4 | 6.4.3(6) |  |
| `beta-interior` | beta_interior |  | 1.15 | 1.1 | 6.4.3(6) |  |
| `beta-wall-end` | beta_wall_end |  | 1.35 | 1.35 | 6.4.3(6) |  |

##### Clause 6.4.4(1) (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `crd-c-punch` | crd_c_punch |  | 0.18 | 0.18 | 6.4.4(1) |  |
| `k1-punch` | k1_punch |  | 0.1 | 0.1 | 6.4.4(1) |  |

##### Clause 6.4.5(3) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `punch-vrdmax-factor` | punch_vrdmax_factor |  | 0 | 1.4 | 6.4.5(3) |  |

##### Clause 6.4.5(4) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `punch-k-out` | punch_k_out |  | 1.5 | 1.5 | 6.4.5(4) |  |

##### Clause 6.5.2(2) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `strut-cracked` | strut_cracked |  | 0.6 | 0.75 | 6.5.2(2) |  |

##### Clause 6.5.4(4) (3; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `node-k1` | node_k1 |  | 1 | 1.1 | 6.5.4(4) |  |
| `node-k2` | node_k2 |  | 0.85 | 0.75 | 6.5.4(4) |  |
| `node-k3` | node_k3 |  | 0.75 | 0.75 | 6.5.4(4) |  |

##### Clause 6.6.6.1 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-19.1` | ? |  | - | - | 6.6.6.1 | en1994-en1999-target.md:148: **C-19 Longitudinal shear in slab (transverse reinforcement) ** — cl. 6.6.6.1–6.6.6.3, EN 1992-1-1 6.2.4. |

##### Clause 6.8.3 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-21.1` | ? |  | - | - | 6.8.3 | en1994-en1999-target.md:157: **C-21 Fatigue of studs and slab ** — cl. 6.8.3–6.8.7, 2:6.8. ΔτRd = ΔτC/γMf,s, ΔτC=90 MPa at 2e6 cycles, m=8 |
| `c-21.2` | ? |  | - | - | 6.8.3 | en1994-en1999-target.md:157: the combined check Eq. 6.? (reinforcement/steel flange + concrete compression) . |

##### Clause 6.8.4 (3; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fatigue-coupler` | fatigue_coupler |  | 35 | 35 | 6.8.4 |  |
| `fatigue-straight` | fatigue_straight |  | 162.5 | 162.5 | 6.8.4 |  |
| `fatigue-welded` | fatigue_welded |  | 58.5 | 58.5 | 6.8.4 |  |

##### Clause 7.2(2) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sigma-c-char` | sigma_c_char |  | 0.6 | 0.6 | 7.2(2) |  |

##### Clause 7.2(3) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sigma-c-qp` | sigma_c_qp |  | 0.45 | 0.45 | 7.2(3) |  |

##### Clause 7.2(5) (3; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sigma-p-qp` | sigma_p_qp |  | 0.75 | 0.65 | 7.2(5) |  |
| `sigma-s-char` | sigma_s_char |  | 0.8 | 0.8 | 7.2(5) |  |
| `sigma-s-imposed` | sigma_s_imposed |  | 1 | 1 | 7.2(5) |  |

##### Clause 7.2.1.4 (5; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `cone-k-cast-cracked` | cone_k_cast_cracked |  | 8.9 | 8.9 | 7.2.1.4 |  |
| `cone-k-cast-uncracked` | cone_k_cast_uncracked |  | 12.7 | 12.7 | 7.2.1.4 |  |
| `cone-k-post-cracked` | cone_k_post_cracked |  | 7.7 | 7.7 | 7.2.1.4 |  |
| `cone-k-post-uncracked` | cone_k_post_uncracked |  | 11 | 11 | 7.2.1.4 |  |
| `psi-m-n` | psi_m_n |  | 2 | 2 | 7.2.1.4 |  |

##### Clause 7.2.1.5 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pullout-k2-cracked` | pullout_k2_cracked |  | 7.5 | 7.5 | 7.2.1.5 |  |
| `pullout-k2-uncracked` | pullout_k2_uncracked |  | 10.5 | 10.5 | 7.2.1.5 |  |

##### Clause 7.2.2.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `anchor-shear-k` | anchor_shear_k |  | 0.5 | 0.5 | 7.2.2.3 |  |

##### Clause 7.2.2.5 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi-re-edge` | psi_re_edge |  | 1 | 1 | 7.2.2.5 |  |

##### Clause 7.3.1 (3; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `wk-lim` | wk_lim |  | 0.3 | 0.3 | 7.3.1 |  |
| `wk-lim-prestress` | wk_lim_prestress |  | 0.2 | 0.2 | 7.3.1 |  |
| `wk-lim-xc1` | wk_lim_xc1 |  | 0.4 | 0.4 | 7.3.1 |  |

##### Clause 7.3.1(110) (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `tightness-wk-high` | tightness_wk_high |  | 0.2 | 0.2 | 7.3.1(110) |  |
| `tightness-wk-low` | tightness_wk_low |  | 0.05 | 0.05 | 7.3.1(110) |  |

##### Clause 7.3.2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `bridge-fct-min` | bridge_fct_min |  | 3 | 3 | 7.3.2 |  |
| `fct-eff-factor-slab` | fct_eff_factor_slab |  | 1 | 0.5 | 7.3.2 |  |

##### Clause 7.3.4 (4; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k3-crack` | k3_crack |  | 3.4 | 3.4 | 7.3.4 |  |
| `k4-crack` | k4_crack |  | 0.425 | 0.425 | 7.3.4 |  |
| `kt-long` | kt_long |  | 0.4 | 0.4 | 7.3.4 |  |
| `kt-short` | kt_short |  | 0.6 | 0.6 | 7.3.4 |  |

##### Clause 7.4 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `tank-sigma-min` | tank_sigma_min |  | 1 | 1 | 7.4 |  |

##### Clause 7.4.2 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `ld-de-cap` | ld_de_cap |  | 0 | 35 | 7.4.2 |  |

##### Clause 7.4.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-43.1` | ? |  | - | - | 7.4.3 | en1994-en1999-target.md:254: Pass: σs <= limit for wk (NDP wk = 0.3 mm / 0.2 mm / 0.4 mm by exposure per EN 1992-1-1/NA). |

##### Clause 8.3 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k-mandrel-large` | k_mandrel_large |  | 7 | 7 | 8.3 |  |
| `k-mandrel-small` | k_mandrel_small |  | 4 | 4 | 8.3 |  |

##### Clause 8.4.2 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fbd-eta` | fbd_eta |  | 2.25 | 2.25 | 8.4.2 |  |

##### Clause 9.3.1.1(3) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `slab-smax-cracked` | slab_smax_cracked |  | 400 | 250 | 9.3.1.1(3) |  |

##### Clause 9.5.2(2) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `col-asmin-n` | col_asmin_n |  | 0.1 | 0.15 | 9.5.2(2) |  |

##### Clause 9.7.6 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-38.1` | ? |  | - | - | 9.7.6 | en1994-en1999-target.md:236: **C-38 Composite slab punching shear ** — cl. 9.7.6. |
| `c-38.2` | ? |  | - | - | 9.7.6 | en1994-en1999-target.md:236: the exact distance is the thing to be transcribed) |

##### Clause 9.10.2.2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `tie-q1` | tie_q1 |  | 10 | 10 | 9.10.2.2 |  |
| `tie-q2` | tie_q2 |  | 70 | 70 | 9.10.2.2 |  |

##### Clause 9.10.2.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `tie-internal` | tie_internal |  | 20 | 20 | 9.10.2.3 |  |

##### Clause 9.10.2.5 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `tie-column` | tie_column |  | 150 | 150 | 9.10.2.5 |  |

##### Clause 12.3.1 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `alpha-cc-pl` | alpha_cc_pl |  | 0.8 | 0.7 | 12.3.1 |  |

##### Clause ? (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l642.1` | [V] |  | - | - | 11. Verification flags to resolve before implement | en1992-target.md:642: Items marked are from memory and must be confirmed against DIN EN 1992-1-1/NA:2013-04 (+A1), -1-2/NA, -2/NA, -3/NA, DIN EN 1992- |
| `l642.2` | [V] |  | - | - | 11. Verification flags to resolve before implement | en1992-target.md:642: The worked examples that depend on items are marked and must be recomputed once the NDP table is frozen (the arithmetic in the e |

##### Clause Annex J.1 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `surface-reinf` | surface_reinf |  | 0.01 | 0.01 | Annex J.1 |  |
| `surface-smax` | surface_smax |  | 150 | 150 | Annex J.1 |  |

##### Clause Annex J.2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `corner-links-share` | corner_links_share |  | 0.5 | 0.5 | Annex J.2 |  |
| `corner-rho-open` | corner_rho_open |  | 0.02 | 0.02 | Annex J.2 |  |

##### Clause Annex J.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `corbel-k1` | corbel_k1 |  | 0.25 | 0.25 | Annex J.3 |  |

##### Clause DIN EN 1992-1-1/NA (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `na-de-edition` | edition year | year | 0 | 2013 | DIN EN 1992-1-1/NA | wave3 en1992 v3: constant formerly hard-coded in the checks: Edition year of the national annex the DE values are taken from (cited: DIN EN 1992-1-1/N |

##### Clause EN 1992-1-1 9.5.3(3) / NA-DE (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `col-link-spacing-de` | k_s,cl |  | 20 | 12 | EN 1992-1-1 9.5.3(3) / NA-DE | wave3 en1992 v3: constant formerly hard-coded in the checks: Factor on the smallest longitudinal bar diameter for the largest link spacing of columns |
| `col-link-spacing-max` | s_cl,t,max | mm | 400 | 400 | EN 1992-1-1 9.5.3(3) / NA-DE | wave3 en1992 v3: constant formerly hard-coded in the checks: Absolute cap of the link spacing of columns. |

##### Clause EN 1992-1-1 9.5.3(4) (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `col-link-near-factor` | k_near |  | 0.6 | 0.6 | EN 1992-1-1 9.5.3(4) | wave3 en1992 v3: constant formerly hard-coded in the checks: Reduction of the link spacing near beams and above or below slabs and at laps. |

##### Clause EN 1992-1-1 Table 7.4N note 2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `ld-flange-factor` | k_flange |  | 0.8 | 0.8 | EN 1992-1-1 Table 7.4N note 2 | wave3 en1992 v3: constant formerly hard-coded in the checks: Factor on the span-to-depth limit for flanged sections with b_f/b_w above the ratio row. |
| `ld-flange-ratio` | b_f/b_w |  | 3 | 3 | EN 1992-1-1 Table 7.4N note 2 | wave3 en1992 v3: constant formerly hard-coded in the checks: Flange to web width ratio above which the flange factor applies. |

##### Clause EN 1992-1-2 3.2-3.3 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire-ks-kc` | fire_ks_kc |  | 0 | 0 | EN 1992-1-2 3.2-3.3 |  |

##### Clause EN 1992-1-2 5 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire-tables` | fire_tables |  | 0 | 0 | EN 1992-1-2 5 |  |

##### Clause EN 1992-2 Annex KK (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `kk-ageing-chi` | kk_ageing_chi |  | 0.8 | 0.8 | EN 1992-2 Annex KK |  |

##### Clause EN 1992-4 5 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-ms-fat` | gamma_ms_fat |  | 1.35 | 1.35 | EN 1992-4 5 |  |

##### Clause EN 1992-4 7.2.1.8 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-ms-re` | gamma_ms_re |  | 1.15 | 1.15 | EN 1992-4 7.2.1.8 |  |

##### Clause Table 7.105 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lq-01.1` | hD/h <= 5: 0.20 |  | - | - | Table 7.105 | en1992-target.md:401: Table 7.105: `hD/h <= 5: 0.20`, `hD/h = 35: 0.05` linear |

#### EN 1992, part 1-2 (6)

##### Clause 5.1 (1; DIN EN 1992-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `r-01.1` | fck > C80/95 |  | - | - | 5.1 | en1992-target.md:377: spalling check: `fck > C80/95`? . |

##### Clause EN 1990 6.5.3 / EN 1992-1-2 2.4.2 (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi-fire-lead` | psi_fi selector |  | 1 | 1 | EN 1990 6.5.3 / EN 1992-1-2 2.4.2 | wave3 en1992 v3: constant formerly hard-coded in the checks: Combination of the leading variable action in fire: 1 = frequent value psi_1,1, 2 = quasi |

##### Clause EN 1992-1-2 B.2 (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire-zone-exponent` | e_az |  | 1 | 1 | EN 1992-1-2 B.2 | wave3 en1992 v3: constant formerly hard-coded in the checks: Exponent of a_z = w [1 - (k_c,m / k_c(theta_M))^e]; spec R-04 has no exponent, implemente |
| `fire-zones` | n |  | 5 | 5 | EN 1992-1-2 B.2 | wave3 en1992 v3: constant formerly hard-coded in the checks: Number of parallel zones of the zone method. |

##### Clause Table 3.1 (1; DIN EN 1992-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `r-07.1` | kc(theta) |  | - | - | Table 3.1 | en1992-target.md:385: **R-07 — 3.2, 3.3 — Material properties at elevated T.** `kc(theta)` Table 3.1 siliceous: 20: 1.00; 100: 1.00; 200: 0.95; 300: 0 |

##### Clause Table 3.2a (1; DIN EN 1992-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `r-03.1` | ks(theta) |  | - | - | Table 3.2a | en1992-target.md:380: `ks(theta)` reduction (EN 1992-1-2 Table 3.2a, hot-rolled bars `1.0` to 400, `0.78` at 500, `0.47` at 600, `0.23` at 700, `0.11` |

#### EN 1992, part 2 (4)

##### Clause 3.1.2 (1; DIN EN 1992-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `br-01.1` | alpha_cc = 0.85 |  | - | - | 3.1.2 | en1992-target.md:390: **BR-01 — 3.1.2, 3.1.5 DE — Materials for bridges.** `alpha_cc = 0.85` for bridges persistent, `fck <= C70/85`, additional DE NA |

##### Clause 6.2.2 (1; DIN EN 1992-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `br-03.1` | [V] |  | - | - | 6.2.2 | en1992-target.md:392: ), `alpha_cw = 1.0` |

##### Clause 7.3 (1; DIN EN 1992-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `br-05.1` | wk,lim |  | - | - | 7.3 | en1992-target.md:394: **BR-05 — 7.3 — Crack control bridges.** `wk,lim` DE bridges: prestressed `wk <= 0.2 mm` frequent? or decompression for bonded ` |

##### Clause Table 4.4N (1; DIN EN 1992-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `br-02.1` | cmin,dur |  | - | - | Table 4.4N | en1992-target.md:391: **BR-02 — 4.4.1.2 — Cover in bridges.** `cmin,dur` per structural class S4+ (Table 4.4N Bridge: design life 100 yrs -> `S6`; 4.4 |

#### EN 1992, part 3 (6)

##### Clause 4.4 (1; DIN EN 1992-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lq-05.1` | cmin |  | - | - | 4.4 | en1992-target.md:405: **LQ-05 — 4.4 — Cover for liquid tanks.** `cmin` increased by `5 mm` `Dc,dur,add`? ; Aggressive liquids: EN 206 `XA` classes, pr |

##### Clause 7.4 (1; DIN EN 1992-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lq-06.1` | p = n*P/(R*...) |  | - | - | 7.4 | en1992-target.md:406: **LQ-06 — 7.4 — Thermal and shrinkage restraint; prestressed tanks.** Hoop prestress `p = n*P/(R*...)`, losses via PR-04..PR-05; |

##### Clause EN 1992-3 7.3.1(105) (2; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lq-04-head-ratio` | h_D/h |  | 5 | 5 | EN 1992-3 7.3.1(105) | wave3 en1992 v3: constant formerly hard-coded in the checks: Hydrostatic head ratio up to which self-healing applies. |
| `lq-04-wk-selfhealing` | w_k,lim | mm | 0.2 | 0.2 | EN 1992-3 7.3.1(105) | wave3 en1992 v3: constant formerly hard-coded in the checks: Crack width limit with self-healing when h_D/h is not above the head ratio. |

##### Clause EN 1992-3 Table 7.105DE (1; DIN EN 1992-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lq-04-wk-above` | w_k,lim | mm | 0.1 | 0.1 | EN 1992-3 Table 7.105DE | wave3 en1992 v3: constant formerly hard-coded in the checks: Crack width limit for h_D/h above the head ratio (NA-DE tightness class 1). |

##### Clause Table 7.105DE (1; DIN EN 1992-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `lq-04.1` | wk,lim |  | - | - | Table 7.105DE | en1992-target.md:404: **LQ-04 — 7.3.1(105) — Self-healing crack width limits.** `wk,lim` allowed up to `0.2 mm` where `hD/h <= 5` and `wk <= 0.1 mm` — |

#### EN 1992, part 4 (2)

##### Clause 1.5 (1; DIN EN 1992-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fa-03.1` | [N, fck MPa, hef mm]; |  | - | - | 1.5 | en1992-target.md:417: `N0Rk,c = k1*sqrt(fck)*hef^1.5` [N, fck MPa, hef mm]; `k1 = 7.7` (post-installed, cracked) `11.0` (uncracked); cast-in headed: ` |

##### Clause 7.2.1.4 (1; DIN EN 1992-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fa-04.1` | NRk,p = psi_c*A_h*8*fck |  | - | - | 7.2.1.4 | en1992-target.md:418: **FA-04 — 7.2.1.4 — Pull-out failure.** `NRk,p = psi_c*A_h*8*fck`? precisely: `NRk,p = k2*Ah*fck` with `k2 = 7.5` (cracked), `10 |

#### EN 1992, part not stated (53)

##### Clause 1.0 (3; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `b-04.1` | <= 50% |  | - | - | 1.0 | en1992-target.md:285: DE: laps of main bars in a given section: all bars in one section if `<= 50%` - `<= 100%` for single-layer slabs with `phi <= 16 |
| `sl-04.1` | fctm,min = 2.9 |  | - | - | 1.0 | en1992-target.md:309: `fctm,min = 2.9`? in DE NA `fct,eff = 0.5*fctm` for at least slab thickness h <= 300 , bridges: `fctm` min). |
| `t-04.1` | . Box sections: |  | - | - | 1.0 | en1992-target.md:224: DE: `TEd/TRd,max + VEd/VRd,max <= 1` (6.3.2(4) eq. 6.29) and `VEd <= VRd,c*(1 - TEd/TRd,c)`... . |

##### Clause 1.1 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e-03.1` | nu1 = 0.75*nu2 = 0.75 |  | - | - | 1.1 | en1992-target.md:562: `nu2 = min(1.1 - 30/500, 1.0) = 1.0` (the raw value 1.04 is capped), `nu1 = 0.75*nu2 = 0.75` . |
| `p-02.1` | beta = 1.1 |  | - | - | 1.1 | en1992-target.md:235: DE NA simplified (non-sway braced, adjacent spans ratio 0.5..2): interior `beta = 1.1` (NA 6.4.3(6): `1.10` interior, `1.40` edg |

##### Clause 1.4 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-03.3` | [V] |  | - | - | 1.4 | en1992-target.md:242: (DE NA 6.4.5(3), applies to slabs with punching reinforcement |

##### Clause 1.5 (5; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e-05.1` | CRd,c = 0.18/1.5 = 0.12 |  | - | - | 1.5 | en1992-target.md:576: `CRd,c = 0.18/1.5 = 0.12` |
| `e-05.2` | [V] |  | - | - | 1.5 | en1992-target.md:577: -> `0.865 <= 1.031` ok (the perimeter to which the NA factor applies must be confirmed). |
| `p-04.1` | [V] |  | - | - | 1.5 | en1992-target.md:246: (`vRd,max = 1.96 vRd,c` for double-headed studs with approval) |
| `sl-07.1` | ( |  | - | - | 1.5 | en1992-target.md:322: `l/d <= K*[11 + 1.5*sqrt(fck)*rho0/rho + 3.2*sqrt(fck)*(rho0/rho - 1)^(3/2)]` (`rho <= rho0`); `l/d <= K*[11 + 1.5*sqrt(fck)*rho |
| `v-02.1` | 0.035*k^1.5*sqrt(fck) |  | - | - | 1.5 | en1992-target.md:188: EC `0.035*k^1.5*sqrt(fck)` / DE `(0.0525/gamma_c)*k^1.5*sqrt(fck)` for `d <= 600 mm`, `(0.0375/gamma_c)*k^1.5*sqrt(fck)` for `d |

##### Clause 2.0 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-01.1` | uout |  | - | - | 2.0 | en1992-target.md:230: DE `uout` at `1.5d`... via `kd` with `k=1.5`) |
| `p-03.1` | and |  | - | - | 2.0 | en1992-target.md:239: `rho_l = sqrt(rho_ly*rho_lz) <= 0.02` and `<= 0.5*fcd/fyd` (DE) |

##### Clause 2.2 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `d-02.1` | XC1 C16/20 |  | - | - | 2.2 | en1992-target.md:71: **D-02 — Table E.1DE (informative Annex E, normative in NA) — Minimum concrete strength class / Mindestbetonfestigkeitsklasse.** |
| `d-03.1` | Dc,dur,gamma = 0 |  | - | - | 2.2 | en1992-target.md:76: NDP: `Dc,dur,gamma = 0` EC / DE `0`; `Dc,dur,st = 0`, `Dc,dur,add = 0` EC / DE `0` . |

##### Clause 2.5 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-03.1` | 1.2 |  | - | - | 2.5 | en1992-target.md:196: `1.2` for shear cracked state, `VRd,cc = c*0.48*fck^(1/3)*(1 - 1.2*sigma_cp/fcd)*bw*z`, `c = 0.5`) |
| `v-05.1` | c |  | - | - | 2.5 | en1992-target.md:204: DE `c` rough `0.40`, indented `0.50` with limit `fctd`-based `c` |

##### Clause 2.7 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e17-02.5` | ? |  | - | - | 2.7 | en1990-en1991-target.md:313: Table A.1 (consequence classes CC1 – CC3 for buildings, with CC2a lower and CC2b upper risk ): CC1: houses ≤ 4 storeys, a |
| `e17-02.6` | ? |  | - | - | 2.7 | en1990-en1991-target.md:313: Brandschutz . |

##### Clause 2.12 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pr-04.1` | ; |  | - | - | 2.12 | en1992-target.md:346: Friction: `DPmu(x) = Pmax*(1 - exp(-mu*(theta + k*x)))`; `mu` per Tab (strand in steel duct `0.19`, DE per approval `mu = 0.17.. |

##### Clause 3.0 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pa-01.1` | alpha_cc = 0.85 |  | - | - | 3.0 | en1992-target.md:266: `alpha_cc = 0.85` -> `fcd` with `0.7`? . |

##### Clause 3.1.7(3) (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-07.1` | 0.9 |  | - | - | 3.1.7(3) | en1992-target.md:50: when the compression zone narrows towards the compression edge the NA factor `0.9` applies (3.1.7(3), `eta*fcd -> 0.9*eta*fcd`) . |

##### Clause 3.3 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-09.1` | sigma_p = fp0.1k/gamma_s |  | - | - | 3.3 | en1992-target.md:57: **M-09 — 3.3 — Prestressing steel diagrams / Spannstahl-Linie.** Inclined branch `sigma_p = fp0.1k/gamma_s` to `fpd = fp0.1k/gamm |

##### Clause 4.4.1.1 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `d-04.1` | [V] |  | - | - | 4.4.1.1 | en1992-target.md:81: EC 10 mm / DE 10 mm (reduced to 5 mm with quality control , 0 mm for precast with measured cover, 4.4.1.3(3-4)) |

##### Clause 5.5 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-06.1` | xu/d <= 0.45 |  | - | - | 5.5 | en1992-target.md:119: `xu/d <= 0.45` (B/C, `fck <= 50`), `0.35` (`fck > 50`) |
| `a-06.2` | delta >= 0.64 + 0.8*xu/d |  | - | - | 5.5 | en1992-target.md:119: DE: `delta >= 0.64 + 0.8*xu/d` if `fck <= 50` per NA with `Class A` steel limited to `delta >= 0.8` . |

##### Clause 5.7 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-08.1` | gamma_R |  | - | - | 5.7 | en1992-target.md:123: gating criteria: material design strengths with `gamma_R` global (gamma_O = 1.2 DE per 5.7 NA, `Rd = Rm(fcm-based)/gamma_R`). |

##### Clause 5.8.3.1 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-09.1` | lambda_lim = 25 |  | - | - | 5.8.3.1 | en1992-target.md:127: slender-limit per NA uses `lambda_lim = 25` for `\|n\| >= 0.41. |

##### Clause 5.8.6 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-14.1` | ; |  | - | - | 5.8.6 | en1992-target.md:141: **A-14 — 5.8.6 / 5.8.4 — Second-order effects of slender walls, lateral loads (global).** Global check: `Fv,Ed <= k1*sum Ecd*Ic/ |

##### Clause 5.10.2.1 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pr-01.1` | Pmax = Ap*sigma_p,max |  | - | - | 5.10.2.1 | en1992-target.md:341: **PR-01 — 5.10.2.1 — Maximum stressing force / Höchstkraft.** `Pmax = Ap*sigma_p,max`, `sigma_p,max = min(k1*fpk, k2*fp0.1k)`, ` |

##### Clause 5.10.8 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pr-06.1` | Ap |  | - | - | 5.10.8 | en1992-target.md:351: **PR-06 — 5.10.8 — Ultimate limit state bending of prestressed members.** S-10 (pre-strain), `Ap` strain increment, `DEpsilon_p |

##### Clause 5.10.9 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pr-07.1` | 1.10/0.90 |  | - | - | 5.10.9 | en1992-target.md:352: `1.10/0.90` (unbonded or external) (DE ) |

##### Clause 6.1 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `s-09.1` | hw >= 120 mm |  | - | - | 6.1 | en1992-target.md:176: DE limit `hw >= 120 mm` cast-in-place . |

##### Clause 6.2.1 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-01.1` | av/d >= 0.5 |  | - | - | 6.2.1 | en1992-target.md:182: DE NA: restricted to `av/d >= 0.5` . |

##### Clause 6.2.3(103) (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-04.1` | alpha_cw = 1.0 |  | - | - | 6.2.3(103) | en1992-target.md:200: DE uses `alpha_cw = 1.0` with formula-integrated `sigma_cp`) . |

##### Clause 6.4.4(1) (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-03.2` | CRd,c |  | 0.1 | 0.12 | 6.4.4(1) | en1992-target.md:240: NDP: `CRd,c` EC `0.18/gamma_c` / DE `0.18/gamma_c` (punching, 6.4.4(1) NA) ; `k1` EC `0.1` / DE `0.12`; `vmin` EC `0.035*k^1.5*s |

##### Clause 6.4.5(4) (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `p-05.1` | vRd,max |  | - | - | 6.4.5(4) | en1992-target.md:249: **P-05 — 6.4.5(4) — Shear studs layout per NA / ETA.** studs (DE additional approval `vRd,max` multipliers ). |

##### Clause 6.5.4 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `st-03.1` | k1 |  | - | - | 6.5.4 | en1992-target.md:261: DE NA values for `k1`, `k2`, `k3` to be taken from the NA text (my recollection: `k1 = 1.1`, `k2 = 0.75`, `k3 = 0.75`). |

##### Clause 6.8.4 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `f-03.1` | Delta sigma_Rsk = 162.5 |  | - | - | 6.8.4 | en1992-target.md:333: DE NA 6.8.4: `Delta sigma_Rsk = 162.5` (straight, `phi <= 28`? ) |

##### Clause 7.2(5) (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `pr-05.1` | Pm,t(x,t) |  | - | - | 7.2(5) | en1992-target.md:350: Outputs: `Pm,t(x,t)`, `Delta sigma_p,c+s+r`. Criterion: `sigma_p,m,t <= 0.65*fpk` (QP, 7.2(5) DE) ; `Pm,t >= Pm,t,min required` |
| `sl-02.1` | 0.65 fpk |  | - | - | 7.2(5) | en1992-target.md:303: after losses `0.65 fpk` quasi-permanent ). |

##### Clause 7.3.3 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sl-05.1` | wk = 0.2 |  | - | - | 7.3.3 | en1992-target.md:311: `wk = 0.2`: 160 -> 25, 200 -> 16, 240 -> 12, 280 -> 8, 320 -> 6, 360 -> 5, 400 -> 4 |
| `sl-05.2` | wk = 0.2 |  | - | - | 7.3.3 | en1992-target.md:311: `wk = 0.2`: 200, 150, 100, 50 |

##### Clause 8.4.2 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `b-01.1` | [V] |  | - | - | 8.4.2 | en1992-target.md:270: (`alpha_ct = 0.85`). |

##### Clause 9.2.2 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-03.2` | (DE: |  | - | - | 9.2.2 | en1992-target.md:197: Minimum shear reinforcement (9.2.2): `rho_w = Asw/(s*bw*sin alpha) >= rho_w,min = 0.16*fctm/fyk` (DE: `0.16*fctm/fyk` for beams, |

##### Clause 9.3.1 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `dt-06.1` | As,min |  | - | - | 9.3.1 | en1992-target.md:365: **DT-06 — 9.3.1 — Solid slabs: reinforcement.** `As,min` as DT-01; secondary `>= 0.2*As`; spacing `smax = min(3*h, 400 mm)` prin |

##### Clause 9.6 (2; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `dt-10.1` | As,v,min = 0.002*Ac |  | - | - | 9.6 | en1992-target.md:369: **DT-10 — 9.6 — Walls.** `As,v,min = 0.002*Ac` (DE `0.15*NEd/fyd >= 0.0015*Ac` unreinforced wall ), max `0.04*Ac` |
| `dt-10.2` | h >= 100 mm |  | - | - | 9.6 | en1992-target.md:369: DE `h >= 100 mm`? plain . |

##### Clause 9.10 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `dt-14.1` | or DE ( |  | - | - | 9.10 | en1992-target.md:373: ` or DE (`ft = 20 kN`, `Ftie, per` per DE NA 9.10 ), vertical ties. |

##### Clause 12.7 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e-08.1` | (DE |  | - | - | 12.7 | en1992-target.md:593: Strut capacity (EC recommended, node CCT): `sigma_Rd,max = k2*nu'*fcd = 0.85*(1 - 30/250)*17 = 0.85*0.88*17 = 12.7 MPa` (DE `k2 |

##### Clause Table 4.4N (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `d-03.2` | cmin,dur |  | - | - | Table 4.4N | en1992-target.md:77: `cmin,dur` table (reinforcement, Table 4.4N / DE equivalent): full matrix held in data, not hard coded: rows `X0, XC1, XC2/XC3, X |

##### Clause Table 5.5 (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e-12.1` | [V] |  | - | - | Table 5.5 | en1992-target.md:616: Table 5.5 column 2 (simply supported, ): R30 `80/15`, R60 `120/25`, R90 `150/35`, R120 `200/45`, R180 `240/60`, R240 `280/75` (` |

##### Clause Table 7.1DE (1; DIN EN 1992/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sl-06.1` | 0.4 |  | - | - | Table 7.1DE | en1992-target.md:318: DE Table 7.1DE: `0.4` for XC1 (X0, XC1: 0.4 / `0.2` for prestressed bonded post-tension, ...), `0.3` all other (XC2..XC4, XD, XS |

### EN 1993 (328)

#### EN 1993, part 1-1 (32)

##### Clause 1.00 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-001.1` | non-normative |  | - | - | 1.00 | en1993-target.md:34: Bridges (1993-2/NA): gM0 = 1.00, gM1 = 1.10 ****, gM2 = 1.25. |

##### Clause 1.0 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-902.1` | fu/fy >= 1.05 |  | - | - | 1.0 | en1993-target.md:481: ductility (V-011) relaxed: `fu/fy >= 1.05`, `eu >= 15 eps_y` with `elongation >= 10 %` for S>460 up to S700 ****. |

##### Clause 1.7 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-40.2` | ? |  | - | - | 1.7 | en1994-en1999-target.md:244: NDP: (per use: L/250 total, L/300 variable for floors |

##### Clause 3.2.1 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-010.1` | Table 3.1 nominal f_y and f_u (t <= 40 m |  | - | - | Table 3.1 |  |

##### Clause 3.2.2 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-011.1` | Ductility f_u/f_y |  | - | - | 3.2.2(1) |  |

##### Clause 3.2.6 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-013.1` | Design values of constants |  | - | - | 3.2.6(1) |  |

##### Clause 5.2.1 (2; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-1.alpha-cr-min` | alpha_cr min |  | 3 | 3 | 5.2.1 |  |
| `v-026.2` | Eq. 5.2 tolerance |  | 0.05 | 0.05 | 5.2.1(4)B |  |

##### Clause 5.3.2(6) (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-024.1` | Bow imperfection inclusion rule |  | - | - | 5.3.2(6) |  |

##### Clause 6.1(1) (4; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-m0` | γ_M0 |  | 1 | 1 | 6.1(1) |  |
| `gamma-m1` | γ_M1 |  | 1 | 1.1 | 6.1(1) |  |
| `gamma-m2` | γ_M2 |  | 1.25 | 1.25 | 6.1(1) |  |
| `gamma-m3` | γ_M3 |  | 1.25 | 1.25 | 6.1(1) |  |

##### Clause 6.2.6(3) (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-050.1` | Shear area A_v of rolled and welded I se |  | - | - | 6.2.6(3) |  |

##### Clause 6.3.1.4 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-101.1` | Torsional buckling length and curve |  | - | - | 6.3.1.4 |  |

##### Clause 6.3.2.2 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-113b.1` | W_y class dependent |  | - | - | 6.3.2.1(3) |  |

##### Clause 6.3.2.4(1)B (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-112.1` | Simplified LTB method k_fl and lambda_c0 |  | - | - | 6.3.2.4 |  |

##### Clause 6.3.4 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-123.1` | alpha_cr,op estimate |  | - | - | 6.3.4(2) |  |

##### Clause 7.2 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-090.1` | ? |  | - | - | 7.2 | en1993-target.md:192: limits NDP: L/250 beams, L/300 beams carrying brittle finishes? etc. (EN Table: roofs L/200 general, floors L/250, crane runways |

##### Clause 7.2.3 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-091.1` | Floor natural frequency limit |  | - | - | 7.2.3 |  |

##### Clause 10.1.5 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-113c.1` | ? |  | - | - | 10.1.5 | en1993-target.md:151: **V-113c NA-DE: Mcr for lateral restraint by trapezoidal sheeting / Kippen mit Drehbettung** (DIN EN 1993-1-1/NA NCI NA.? and Eu |

##### Clause EN 1993-1-9 Table 3.1 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-mf` | γ_Mf |  | 1.15 | 1.15 | EN 1993-1-9 Table 3.1 |  |

##### Clause Table 5.1 (10; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-1.t51.a.elastic` | L/e0 curve a elastic |  | 300 | 300 | Table 5.1 |  |
| `1-1.t51.a.plastic` | L/e0 curve a plastic |  | 250 | 250 | Table 5.1 |  |
| `1-1.t51.a0.elastic` | L/e0 curve a0 elastic |  | 350 | 350 | Table 5.1 |  |
| `1-1.t51.a0.plastic` | L/e0 curve a0 plastic |  | 300 | 300 | Table 5.1 |  |
| `1-1.t51.b.elastic` | L/e0 curve b elastic |  | 250 | 250 | Table 5.1 |  |
| `1-1.t51.b.plastic` | L/e0 curve b plastic |  | 200 | 200 | Table 5.1 |  |
| `1-1.t51.c.elastic` | L/e0 curve c elastic |  | 200 | 200 | Table 5.1 |  |
| `1-1.t51.c.plastic` | L/e0 curve c plastic |  | 150 | 150 | Table 5.1 |  |
| `1-1.t51.d.elastic` | L/e0 curve d elastic |  | 150 | 150 | Table 5.1 |  |
| `1-1.t51.d.plastic` | L/e0 curve d plastic |  | 100 | 100 | Table 5.1 |  |

#### EN 1993, part 1-10 (49)

##### Clause Table 2.1 (49; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-10.t21.s235-j0.t-10` | t_max S235-J0 -10 C | mm | 65 | 65 | Table 2.1 |  |
| `1-10.t21.s235-j0.t-20` | t_max S235-J0 -20 C | mm | 55 | 55 | Table 2.1 |  |
| `1-10.t21.s235-j0.t-30` | t_max S235-J0 -30 C | mm | 45 | 45 | Table 2.1 |  |
| `1-10.t21.s235-j0.t-40` | t_max S235-J0 -40 C | mm | 40 | 40 | Table 2.1 |  |
| `1-10.t21.s235-j0.t-50` | t_max S235-J0 -50 C | mm | 35 | 35 | Table 2.1 |  |
| `1-10.t21.s235-j0.t0` | t_max S235-J0 0 C | mm | 75 | 75 | Table 2.1 |  |
| `1-10.t21.s235-j0.t10` | t_max S235-J0 10 C | mm | 90 | 90 | Table 2.1 |  |
| `1-10.t21.s235-j2.t-10` | t_max S235-J2 -10 C | mm | 90 | 90 | Table 2.1 |  |
| `1-10.t21.s235-j2.t-20` | t_max S235-J2 -20 C | mm | 75 | 75 | Table 2.1 |  |
| `1-10.t21.s235-j2.t-30` | t_max S235-J2 -30 C | mm | 65 | 65 | Table 2.1 |  |
| `1-10.t21.s235-j2.t-40` | t_max S235-J2 -40 C | mm | 55 | 55 | Table 2.1 |  |
| `1-10.t21.s235-j2.t-50` | t_max S235-J2 -50 C | mm | 45 | 45 | Table 2.1 |  |
| `1-10.t21.s235-j2.t0` | t_max S235-J2 0 C | mm | 105 | 105 | Table 2.1 |  |
| `1-10.t21.s235-j2.t10` | t_max S235-J2 10 C | mm | 125 | 125 | Table 2.1 |  |
| `1-10.t21.s235-jr.t-10` | t_max S235-JR -10 C | mm | 40 | 40 | Table 2.1 |  |
| `1-10.t21.s235-jr.t-20` | t_max S235-JR -20 C | mm | 35 | 35 | Table 2.1 |  |
| `1-10.t21.s235-jr.t-30` | t_max S235-JR -30 C | mm | 30 | 30 | Table 2.1 |  |
| `1-10.t21.s235-jr.t-40` | t_max S235-JR -40 C | mm | 25 | 25 | Table 2.1 |  |
| `1-10.t21.s235-jr.t-50` | t_max S235-JR -50 C | mm | 20 | 20 | Table 2.1 |  |
| `1-10.t21.s235-jr.t0` | t_max S235-JR 0 C | mm | 50 | 50 | Table 2.1 |  |
| `1-10.t21.s235-jr.t10` | t_max S235-JR 10 C | mm | 60 | 60 | Table 2.1 |  |
| `1-10.t21.s355-j0.t-10` | t_max S355-J0 -10 C | mm | 45 | 45 | Table 2.1 |  |
| `1-10.t21.s355-j0.t-20` | t_max S355-J0 -20 C | mm | 40 | 40 | Table 2.1 |  |
| `1-10.t21.s355-j0.t-30` | t_max S355-J0 -30 C | mm | 35 | 35 | Table 2.1 |  |
| `1-10.t21.s355-j0.t-40` | t_max S355-J0 -40 C | mm | 30 | 30 | Table 2.1 |  |
| `1-10.t21.s355-j0.t-50` | t_max S355-J0 -50 C | mm | 25 | 25 | Table 2.1 |  |
| `1-10.t21.s355-j0.t0` | t_max S355-J0 0 C | mm | 55 | 55 | Table 2.1 |  |
| `1-10.t21.s355-j0.t10` | t_max S355-J0 10 C | mm | 65 | 65 | Table 2.1 |  |
| `1-10.t21.s355-j2.t-10` | t_max S355-J2 -10 C | mm | 65 | 65 | Table 2.1 |  |
| `1-10.t21.s355-j2.t-20` | t_max S355-J2 -20 C | mm | 55 | 55 | Table 2.1 |  |
| `1-10.t21.s355-j2.t-30` | t_max S355-J2 -30 C | mm | 45 | 45 | Table 2.1 |  |
| `1-10.t21.s355-j2.t-40` | t_max S355-J2 -40 C | mm | 40 | 40 | Table 2.1 |  |
| `1-10.t21.s355-j2.t-50` | t_max S355-J2 -50 C | mm | 35 | 35 | Table 2.1 |  |
| `1-10.t21.s355-j2.t0` | t_max S355-J2 0 C | mm | 75 | 75 | Table 2.1 |  |
| `1-10.t21.s355-j2.t10` | t_max S355-J2 10 C | mm | 90 | 90 | Table 2.1 |  |
| `1-10.t21.s355-jr.t-10` | t_max S355-JR -10 C | mm | 30 | 30 | Table 2.1 |  |
| `1-10.t21.s355-jr.t-20` | t_max S355-JR -20 C | mm | 25 | 25 | Table 2.1 |  |
| `1-10.t21.s355-jr.t-30` | t_max S355-JR -30 C | mm | 20 | 20 | Table 2.1 |  |
| `1-10.t21.s355-jr.t-40` | t_max S355-JR -40 C | mm | 15 | 15 | Table 2.1 |  |
| `1-10.t21.s355-jr.t-50` | t_max S355-JR -50 C | mm | 15 | 15 | Table 2.1 |  |
| `1-10.t21.s355-jr.t0` | t_max S355-JR 0 C | mm | 35 | 35 | Table 2.1 |  |
| `1-10.t21.s355-jr.t10` | t_max S355-JR 10 C | mm | 40 | 40 | Table 2.1 |  |
| `1-10.t21.s355-k2.t-10` | t_max S355-K2 -10 C | mm | 90 | 90 | Table 2.1 |  |
| `1-10.t21.s355-k2.t-20` | t_max S355-K2 -20 C | mm | 75 | 75 | Table 2.1 |  |
| `1-10.t21.s355-k2.t-30` | t_max S355-K2 -30 C | mm | 65 | 65 | Table 2.1 |  |
| `1-10.t21.s355-k2.t-40` | t_max S355-K2 -40 C | mm | 55 | 55 | Table 2.1 |  |
| `1-10.t21.s355-k2.t-50` | t_max S355-K2 -50 C | mm | 45 | 45 | Table 2.1 |  |
| `1-10.t21.s355-k2.t0` | t_max S355-K2 0 C | mm | 105 | 105 | Table 2.1 |  |
| `1-10.t21.s355-k2.t10` | t_max S355-K2 10 C | mm | 125 | 125 | Table 2.1 |  |

#### EN 1993, part 1-2 (43)

##### Clause 3.4 (1; DIN EN 1993-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-601.1` | ; |  | - | - | 3.4 | en1993-target.md:235: NDP-fire: **[NA]** DIN EN 1993-1-2/NA εm=0.7? and αc 25 ****. |

##### Clause 4.2.3.3 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-607.1` | Lateral-torsional buckling in fire |  | - | - | 4.2.3.3 |  |

##### Clause 4.2.3.5 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-608.1` | Bending and axial compression in fire |  | - | - | 4.2.3.5 |  |

##### Clause 4.3 (1; DIN EN 1993-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-47.1` | ? |  | - | - | 4.3 | en1994-en1999-target.md:264: **C-47 Simple calculation models ** — F:4.3 (composite beams: temperature of the steel bottom flange/web/upper flange θa |

##### Clause Table 3.1 (39; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-2.t31.ke.100` | k_E,theta 100 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ke.1000` | k_E,theta 1000 C |  | 0.045 | 0.045 | Table 3.1 |  |
| `1-2.t31.ke.1100` | k_E,theta 1100 C |  | 0.0225 | 0.0225 | Table 3.1 |  |
| `1-2.t31.ke.1200` | k_E,theta 1200 C |  | 0 | 0 | Table 3.1 |  |
| `1-2.t31.ke.20` | k_E,theta 20 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ke.200` | k_E,theta 200 C |  | 0.9 | 0.9 | Table 3.1 |  |
| `1-2.t31.ke.300` | k_E,theta 300 C |  | 0.8 | 0.8 | Table 3.1 |  |
| `1-2.t31.ke.400` | k_E,theta 400 C |  | 0.7 | 0.7 | Table 3.1 |  |
| `1-2.t31.ke.500` | k_E,theta 500 C |  | 0.6 | 0.6 | Table 3.1 |  |
| `1-2.t31.ke.600` | k_E,theta 600 C |  | 0.31 | 0.31 | Table 3.1 |  |
| `1-2.t31.ke.700` | k_E,theta 700 C |  | 0.13 | 0.13 | Table 3.1 |  |
| `1-2.t31.ke.800` | k_E,theta 800 C |  | 0.09 | 0.09 | Table 3.1 |  |
| `1-2.t31.ke.900` | k_E,theta 900 C |  | 0.0675 | 0.0675 | Table 3.1 |  |
| `1-2.t31.kp.100` | k_p,theta 100 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.kp.1000` | k_p,theta 1000 C |  | 0.025 | 0.025 | Table 3.1 |  |
| `1-2.t31.kp.1100` | k_p,theta 1100 C |  | 0.0125 | 0.0125 | Table 3.1 |  |
| `1-2.t31.kp.1200` | k_p,theta 1200 C |  | 0 | 0 | Table 3.1 |  |
| `1-2.t31.kp.20` | k_p,theta 20 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.kp.200` | k_p,theta 200 C |  | 0.807 | 0.807 | Table 3.1 |  |
| `1-2.t31.kp.300` | k_p,theta 300 C |  | 0.613 | 0.613 | Table 3.1 |  |
| `1-2.t31.kp.400` | k_p,theta 400 C |  | 0.42 | 0.42 | Table 3.1 |  |
| `1-2.t31.kp.500` | k_p,theta 500 C |  | 0.36 | 0.36 | Table 3.1 |  |
| `1-2.t31.kp.600` | k_p,theta 600 C |  | 0.18 | 0.18 | Table 3.1 |  |
| `1-2.t31.kp.700` | k_p,theta 700 C |  | 0.075 | 0.075 | Table 3.1 |  |
| `1-2.t31.kp.800` | k_p,theta 800 C |  | 0.05 | 0.05 | Table 3.1 |  |
| `1-2.t31.kp.900` | k_p,theta 900 C |  | 0.0375 | 0.0375 | Table 3.1 |  |
| `1-2.t31.ky.100` | k_y,theta 100 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ky.1000` | k_y,theta 1000 C |  | 0.04 | 0.04 | Table 3.1 |  |
| `1-2.t31.ky.1100` | k_y,theta 1100 C |  | 0.02 | 0.02 | Table 3.1 |  |
| `1-2.t31.ky.1200` | k_y,theta 1200 C |  | 0 | 0 | Table 3.1 |  |
| `1-2.t31.ky.20` | k_y,theta 20 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ky.200` | k_y,theta 200 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ky.300` | k_y,theta 300 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ky.400` | k_y,theta 400 C |  | 1 | 1 | Table 3.1 |  |
| `1-2.t31.ky.500` | k_y,theta 500 C |  | 0.78 | 0.78 | Table 3.1 |  |
| `1-2.t31.ky.600` | k_y,theta 600 C |  | 0.47 | 0.47 | Table 3.1 |  |
| `1-2.t31.ky.700` | k_y,theta 700 C |  | 0.23 | 0.23 | Table 3.1 |  |
| `1-2.t31.ky.800` | k_y,theta 800 C |  | 0.11 | 0.11 | Table 3.1 |  |
| `1-2.t31.ky.900` | k_y,theta 900 C |  | 0.06 | 0.06 | Table 3.1 |  |

#### EN 1993, part 1-3 (5)

##### Clause 6.1.4 (1; DIN EN 1993-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-254.1` | αLT = 0.34 |  | - | - | 6.1.4 | en1993-target.md:253: LTB 6.2.4 with curve b? `αLT = 0.34` (cold-formed) **** |

##### Clause 9.3.1 (1; DIN EN 1993-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-33.1` | ? |  | - | - | 9.3.1 | en1994-en1999-target.md:215: **C-33 Sheeting during construction ** — cl. 9.3.1: ULS and SLS of profiled sheeting as formwork per EN 1993-1-3: Mpl,she |

##### Clause ? (1; DIN EN 1993-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-252.1` | u = 2 σa⁴ bs⁴/(E² t² z²) |  | - | - | 1.L Cold-formed members (EN 1993-1-3) | en1993-target.md:251: **V-252 5.? — Flange curling (cold-formed beams):** `u = 2 σa⁴ bs⁴/(E² t² z²)` mean flange stress deflection ****. |

##### Clause Annex E (1; EN 1993-1-3:2006)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-258.1` | Purlin free flange on the sheeting sprin |  | - | - | Annex E | wave2 en1993 v4: value as used by the artifact: Free flange of flange, lip and one fifth of the web as a simply supported beam on a continuous lateral |

##### Clause Table 6. (1; DIN EN 1993-1-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-254.2` | , |  | - | - | Table 6. | en1993-target.md:253: **V-255 6.1.5 — Shear** `Vb,Rd = hw/sinφ · tt? fbv/gM0`, `fbv` from slenderness `λw = 0.346 (sw/t) sqrt(fyb/E)`: `fbv = 0.58 fyb |

#### EN 1993, part 1-4 (2)

##### Clause Annex C (1; EN 1993-1-4:2006)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-300.2` | Ramberg-Osgood exponent n of the built-i |  | - | - | Table 2.1 and Annex C |  |

##### Clause Table 5. (1; DIN EN 1993-1-4/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-300.1` | kyy |  | - | - | Table 5. | en1993-target.md:258: interaction 5.5.? `kyy` etc. with Table 5.? `ci` ****. **V-304 6.? Fire stainless** reduction factors table `k0.2p,θ`, `kE,θ`. * |

#### EN 1993, part 1-5 (4)

##### Clause 3.2 (1; DIN EN 1993-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-360.1` | β_ult = β^(κ?) |  | - | - | 3.2 | en1993-target.md:262: `β_ult = β^(κ?)`) **** |

##### Clause 5.3 (1; DIN EN 1993-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-367.1` | η = 1.2 |  | - | - | 5.3 | en1993-target.md:268: **[NA]** 1.2 (DE: `η = 1.2` unless > S460) ****. |

##### Clause 6.2 (1; DIN EN 1993-1-5/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-24.1` | ? |  | - | - | 6.2 | en1994-en1999-target.md:174: **C-24 Stud and bearing at concentrated loads/ stiffeners, web resistance ** — cl. 6.2.? EN 1993-1-5 chapter 6 for patch |

##### Clause Annex D (1; EN 1993-1-5:2006)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-376.1` | Corrugated web shear: tau_cr,l (k_l = 5. |  | - | - | Annex D.2 | wave2 en1993 v4: value as used by the artifact: Constants and the min combination of the local and global reductions as encoded; stiffnesses B_x, B_z |

#### EN 1993, part 1-6 (2)

##### Clause ? (1; DIN EN 1993-1-6/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-804.1` | σ = 0.9 ρ g H |  | - | - | 1.O Shells (EN 1993-1-6), silos/tanks (1-6, 4-1, 4 | en1993-target.md:285: top ring**. **** |

##### Clause Annex D (1; EN 1993-1-6:2007)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-806.1` | Required elastic factor of the ring gene | 1 | 3 | 3 | Annex D | wave2 en1993 v4: value as used by the artifact: p_cr = 3 E I/(R^3 l_r) (Bresse) with the effective shell width 1.56 sqrt(r t) must reach 3.0 times the |

#### EN 1993, part 1-7 (1)

##### Clause ? (1; DIN EN 1993-1-7/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-903.1` | w <= b/200 |  | - | - | 1.Q Toughness (EN 1993-1-10) and misc (appended ca | en1993-target.md:483: panel serviceability deflection `w <= b/200` rec ****. |

#### EN 1993, part 1-8 (11)

##### Clause 2.2 Table 2.1 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-8.gamma-m5` | gamma_M5 |  | 1 | 1 | 2.2 Table 2.1 |  |

##### Clause 3.6 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-402.1` | Punching shear of bolt heads and nuts |  | - | - | Table 3.4 |  |

##### Clause 3.10.2 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-405.1` | Block tearing geometry |  | - | - | 3.10.2 |  |

##### Clause 4.10 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-413.1` | Effective width of a plate welded to an  |  | - | - | 4.10 |  |

##### Clause 6.2.7.2 (1; EN 1993-1-8:2005)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-420.2` | group effect of bolt rows |  | end row cp = pi m + p, nc = 2m + 0.625e + 0.5p; inner row cp | end row cp = pi m + p, nc = 2m + 0.625e + 0.5p; inner row cp | Tables 6.4 and 6.6, 6.2.7.2(7) |  |

##### Clause 6.3.1 (1; DIN EN 1993-1-8/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-431.1` | per Table 6.11 |  | - | - | 6.3.1 | en1993-target.md:214: base plate 2.7 ****. |

##### Clause 6.3.1 Table 6.8 (1; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-8.psi-end-plate` | psi |  | 2.7 | 2.7 | 6.3.1 Table 6.8 |  |

##### Clause 7.4.1 (1; EN 1993-1-8:2005)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-440.3` | Table 7.8 validity range of the K/N gap  |  | - | - | Table 7.8 |  |

##### Clause 7.4.2 (3; EN 1993-1-8:2005)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-440.1` | Table 7.11 N_1,Rd chord face |  | - | - | Table 7.11 |  |
| `v-440.2` | Table 7.11 N_i,Rd brace effective width |  | - | - | Table 7.11 |  |
| `v-440.4` | Table 7.11 k_n chord stress function |  | - | - | Table 7.11 note |  |

#### EN 1993, part 1-9 (2)

##### Clause 6.3 (1; DIN EN 1993-1-9/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-502.1` | gM,f |  | - | - | 6.3 | en1993-target.md:226: **** NA-DE may restrict damage-tolerant to inspected details. |

##### Clause 6.8 (1; DIN EN 1993-1-9/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-52.1` | ? |  | - | - | 6.8 | en1994-en1999-target.md:272: **C-52 Fatigue of bridges ** — 2:6.8: 6.8.1 detail categories (EN 1993-1-9 plus studs 90 MPa), 6.8.2 partial factors γMf |

#### EN 1993, part 2 (6)

##### Clause 1.0 (1; DIN EN 1993-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-880.1` | gM0 |  | - | - | 1.0 | en1993-target.md:295: **V-880 2 — Steel bridges (EN 1993-2 + NA-DE, DIN EN 1993-2/NA):** `gM0` 1.0, `γM1` 1.1 ****, `γM2` 1.25 |

##### Clause 5.1 (1; DIN EN 1993-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-501.1` | for |  | - | - | 5.1 | en1993-target.md:225: for non-welded/stress-relieved `Delta sigma_E = sigma_max - 0.6 sigma_min` (compression part reduced by 0.6 when sigma_min < 0) |

##### Clause 9.4.2 (2; EN 1993-2:2006)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-881.1` | Minimum orthotropic deck plate thickness |  | 14 mm; 12 mm below a surfacing of at least 70 mm | 14 mm; 12 mm below a surfacing of at least 70 mm | 9.4.2 | wave2 en1993 v4: value as used by the artifact: t_d >= 14 mm, or 12 mm where the surfacing is at least 70 mm. |
| `v-881.2` | Minimum trough wall thickness of closed  | m | 0.006 | 0.006 | 9.4.2 | wave2 en1993 v4: value as used by the artifact: t_t >= 6 mm. |

##### Clause 9.5 (2; DIN EN 1993-2/NA, EN 1993-2:2006)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l405.1` | ? |  | - | - | 9.5 | en1993-target.md:405: NA for rail/road \| |
| `v-881.3` | Deck plate wheel strip model |  | - | - | 9.5.2 | wave2 en1993 v4: value as used by the artifact: Strip of unit width clamped over the clear span between the trough walls under the wheel force spread |

#### EN 1993, part 5 (2)

##### Clause 7.? driving (2; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `5.pile-driving-limit-ratio` | sigma_driving / f_y limit |  | 0.9 | 0.9 | 7.? driving |  |
| `5.pile-full-capacity-driving-ratio` | sigma_driving / f_y full |  | 0.6 | 0.6 | 7.? driving |  |

#### EN 1993, part not stated (169)

##### Clause 1.0 (3; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l396.1` | ? |  | - | - | 1.0 | en1993-target.md:396: 1.0/1.1/1.25 \| |
| `v-802.1` | σx,Rcr = 0.605 E Cx t/r |  | - | - | 1.0 | en1993-target.md:283: **V-802 1-6 D.1.? — Elastic critical buckling stress of cylinders:** axial: `σx,Rcr = 0.605 E Cx t/r` with `Cx` for medium lengt |
| `v-880.2` | gM0 |  | - | - | 1.0 | en1993-target.md:295: **V-880 2 — Steel bridges (EN 1993-2 + NA-DE, DIN EN 1993-2/NA):** `gM0` 1.0, `γM1` 1.1 **[verify]**, `γM2` 1.25; classification |

##### Clause 1.00 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l401.1` | ? |  | - | - | 1.00 | en1993-target.md:401: same, gFf 1.0 \| |

##### Clause 1.2 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l398.1` | ? |  | - | - | 1.2 | en1993-target.md:398: 1.2 , gM from 1-1 NA (gM1 1.1) \| |

##### Clause 1.5 (2; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l403.1` | ? |  | - | - | 1.5 | en1993-target.md:403: \| 1-11 \| gR \| 1.5 (rec) \| 1.5 \| |
| `v-860.1` | FEd <= FRd = Fuk/γR |  | - | - | 1.5 | en1993-target.md:292: **V-860 1-11 — Tension components with steel wires/cables:** `FEd <= FRd = Fuk/γR`, `γR = 1.5` for wire ropes? rec **[NA]** 1.5; |

##### Clause 3.2.2 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-250.1` | , |  | - | - | 3.2.2 | en1993-target.md:249: **V-250 3.2.2 — Enhanced yield from cold forming (average):** `fya = fyb + (fu - fyb) k n t²/Ag <= (fu + fyb)/2`, `k = 7` rolled |

##### Clause 3.2.6 (4; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-1.alpha-t` | alpha_T | 1/K | 1.2e-05 | 1.2e-05 | 3.2.6 |  |
| `1-1.e-modulus` | E | MPa | 210000 | 210000 | 3.2.6 |  |
| `1-1.g-modulus` | G | MPa | 81000 | 81000 | 3.2.6 |  |
| `1-1.poisson` | nu |  | 0.3 | 0.3 | 3.2.6 |  |

##### Clause 3.25 (2; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-890.1` | σoz,Ed = Fz,Ed/(leff tw) |  | - | - | 3.25 | en1993-target.md:296: **V-890 6 — Crane supporting structures (EN 1993-6, crane runway girders):** loads from EN 1991-3 (vertical wheel loads Qr,max, |
| `v-890.2` | σoz,Ed = Fz,Ed/(leff tw) |  | - | - | 3.25 | en1993-target.md:296: **V-890 6 — Crane supporting structures (EN 1993-6, crane runway girders):** loads from EN 1991-3 (vertical wheel loads Qr,max, |

##### Clause 4.4 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-362.1` | σcr,p = kσ,p σE |  | - | - | 4.4 | en1993-target.md:264: **V-362 4.4 — Effective widths of unstiffened plates (V-040)** and **4.5 stiffened plates:** orthotropic `σcr,p = kσ,p σE`, `σE |

##### Clause 5.3.2(3) (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l383.1` | ? |  | - | - | 5.3.2(3) | en1993-target.md:383: same \| |

##### Clause 5.3.4 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-026.1` | ? |  | - | - | 5.3.4 | en1993-target.md:69: NDP k = 0.5 / **[NA]** 0.5 ****. |

##### Clause 5.3.4(3) (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l384.1` | ? |  | - | - | 5.3.4(3) | en1993-target.md:384: \| 1-1 5.3.4(3) \| k (LTB imperfection) \| 0.5 \| 0.5 \| |

##### Clause 5.5 (2; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-028.1` | ? |  | - | - | 5.5 | en1993-target.md:73: **V-028 5.5 — Cross-section classification (summary)** -> V-030..V-034. **V-029 5.6 — Plastic global analysis conditions:** hinge |
| `v-369.1` | A_end >= ... |  | - | - | 5.5 | en1993-target.md:270: **V-369 5.5 — Stiffeners (end post, intermediate):** rigid end post: `A_end >= ...` requirement of bearing stiffener strength an |

##### Clause 5.5.2 (6; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.ltb.other.alpha` | alpha ltb other |  | 0.34 | 0.34 | 5.5.2 |  |
| `1-4.ltb.other.lambda0` | lambda_0 ltb other |  | 0.4 | 0.4 | 5.5.2 |  |
| `1-4.ltb.welded-open.alpha` | alpha ltb welded-open |  | 0.76 | 0.76 | 5.5.2 |  |
| `1-4.ltb.welded-open.lambda0` | lambda_0 ltb welded-open |  | 0.2 | 0.2 | 5.5.2 |  |
| `1-4.torsional.all.alpha` | alpha torsional all |  | 0.34 | 0.34 | 5.5.2 |  |
| `1-4.torsional.all.lambda0` | lambda_0 torsional all |  | 0.2 | 0.2 | 5.5.2 |  |

##### Clause 6.1.7.2 Table 6.? values (3; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-3.web-crippling.cat1.alpha` | alpha category 1 |  | 0.075 | 0.075 | 6.1.7.2 Table 6.? values |  |
| `1-3.web-crippling.cat1.la` | l_a category 1 |  | 10 | 10 | 6.1.7.2 Table 6.? values |  |
| `1-3.web-crippling.cat2.alpha` | alpha category 2 |  | 0.15 | 0.15 | 6.1.7.2 Table 6.? values |  |

##### Clause 6.2.4 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-420.1` | FT,1,Rd = 4 Mpl,1,Rd/m |  | - | - | 6.2.4 | en1993-target.md:209: **V-420 6.2.4 + Table 6.2 — T-stub in tension equivalent / T-Stummel.** Failure modes: Mode 1 (complete flange yielding): method |

##### Clause 6.2.9.1 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-068.1` | MEd <= MN,Rd |  | - | - | 6.2.9.1 | en1993-target.md:114: CHS (rec): `MN,Rd = Mpl,Rd (1 - n^1.7)` is the Dutch/ECCS form **[NA]/** (EN rec: approximate by RHS). |

##### Clause 6.3 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-113a.1` | ? |  | - | - | 6.3 | en1993-target.md:148: point load mid: C1 1.348, C2 0.630, C3 0.411. k=0.5: UDL C1 0.972, point 0.712 ****, fixed-fixed. |

##### Clause 6.3.1 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-431.2` | per Table 6.11 |  | - | - | 6.3.1 | en1993-target.md:214: **V-431 6.3.1 — Initial rotational stiffness / Anfangssteifigkeit:** `Sj,ini = E z^2/Σ_i (1/ki)` per Table 6.11 `μ` factor: `Sj |

##### Clause 6.3.1.2 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l385.1` | ? |  | - | - | 6.3.1.2 | en1993-target.md:385: same (S460 a0 ) \| |

##### Clause 6.3.1.4 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-850.1` | λ̄eff = 0.9 + ... |  | - | - | 6.3.1.4 | en1993-target.md:290: **V-850 3-1 — Lattice towers and guyed masts:** member classes (leg, diagonal, redundant); effective slenderness: `λ̄eff = 0.9 + |

##### Clause 6.3.2.3 (2; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-111.1` | lambda_bar_LT,0 = 0.4 |  | - | - | 6.3.2.3 | en1993-target.md:140: **V-111 6.3.2.3 — Rolled and equivalent welded sections / Walzprofile und gleichartige geschweisste Querschnitte.** NDP: `lambda |
| `v-111.2` | lambda_bar_LT,0 = 0.4 |  | - | - | 6.3.2.3 | en1993-target.md:140: welded sections with h/b > 3 / hybrid not allowed ****. |

##### Clause 6.3.2.3(1) (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l387.1` | ? |  | - | - | 6.3.2.3(1) | en1993-target.md:387: 0.4 / 0.75 \| |

##### Clause 6.3.2.4(1) (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l390.1` | ? |  | - | - | 6.3.2.4(1) | en1993-target.md:390: \| 1-1 6.3.2.4(1)B \| lambda_c0 \| lambda_LT,0 + 0.1 \| 0.5 \| |

##### Clause 6.3.3 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `na-de.1` | ? |  | - | - | 6.3.3 | en1993-target.md:168: **NA-DE addition:** Wirkt wegen Gleichung, "6.3.3 Nachweis ... gilt fuer Staebe mit doppelt symmetrischen", plus **NCI for unsym |

##### Clause 6.3.3(5) (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-121.1` | NDP 6.3.3(5) |  | - | - | 6.3.3(5) | en1993-target.md:167: NA-DE: Annex A and Annex B both usable **[NA] ** |

##### Clause 6.3.5 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-124.1` | Ls = (C_n 600 + 60 ...) |  | - | - | 6.3.5 | en1993-target.md:175: ` compute if the stable length between torsional restraints on adjacent hinges `Ls = (C_n 600 + 60 ...)` for linear moment varia |

##### Clause 6.4.2 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-131.1` | , |  | - | - | 6.4.2 | en1993-target.md:186: `MEd = (NEd e0 + M_Ed^I)/(1 - NEd/Ncr - NEd/Sv)`, `Ncr = pi^2 E Ieff/L^2`, `Ieff = 0.5 h0^2 Ach + 2 mu Ich` (with mu = 0 for lac |

##### Clause 7.1 / Figure 7.1 (5; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-9.sn.m-1` | m1 |  | 3 | 3 | 7.1 / Figure 7.1 |  |
| `1-9.sn.m-2` | m2 |  | 5 | 5 | 7.1 / Figure 7.1 |  |
| `1-9.sn.n-c` | N_C |  | 2e+06 | 2e+06 | 7.1 / Figure 7.1 |  |
| `1-9.sn.n-d` | N_D |  | 5e+06 | 5e+06 | 7.1 / Figure 7.1 |  |
| `1-9.sn.n-l` | N_L |  | 1e+08 | 1e+08 | 7.1 / Figure 7.1 |  |

##### Clause 7.2 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-371.1` | η2 + 0.8 η1 <= 1.4 |  | - | - | 7.2 | en1993-target.md:272: **V-371 7.2 — Interaction transverse force and bending and shear:** as above `η2 + 0.8 η1 <= 1.4`; `η1/ η3` etc. |

##### Clause 7.2.1(1) (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l393.1` | ? |  | - | - | 7.2.1(1) | en1993-target.md:393: per NA Table (project, L/250 floors etc.) \| |

##### Clause 8.5 (2; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-801.1` | λ̄ = sqrt(Rpl/Rcr) |  | - | - | 8.5 | en1993-target.md:282: **V-801 1-6 section 8.5 — LS3 buckling (reference resistance method):** `λ̄ = sqrt(Rpl/Rcr)`; `χ = 1` for `λ̄ <= λ̄0`; `1 − β (λ |
| `v-801.2` | λ̄ = sqrt(Rpl/Rcr) |  | - | - | 8.5 | en1993-target.md:282: **V-801 1-6 section 8.5 — LS3 buckling (reference resistance method):** `λ̄ = sqrt(Rpl/Rcr)`; `χ = 1` for `λ̄ <= λ̄0`; `1 − β (λ |

##### Clause 8.7 (2; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-803.1` | nx = μ·pn ... |  | - | - | 8.7 | en1993-target.md:284: **V-803 4-1 silos — Loads on silos (EN 1991-4 cross-reference) and shell design:** meridional compression from wall friction `nx |
| `v-803.2` | nx = μ·pn ... |  | - | - | 8.7 | en1993-target.md:284: **V-803 4-1 silos — Loads on silos (EN 1991-4 cross-reference) and shell design:** meridional compression from wall friction `nx |

##### Clause 10.5 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-372.1` | ρ_x |  | - | - | 10.5 | en1993-target.md:273: **V-372 10 — Reduced stress method / Methode der reduzierten Spannungen.** `ρ_x`, `ρ_z`, `χw` for plate slenderness `λ̄p = sqrt( |

##### Clause ? (12; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l394.1` | ? |  | - | - | 5. NA-DE consolidated NDP table (to be encoded as  | en1993-target.md:394: \| 1-1 NCI \| Mcr for lateral support by sheeting \| none \| NCI \| |
| `l5.1` | ? |  | - | - | 📐 EN 1993 Target Specification | en1993-target.md:5: entries marked **** MUST be re-checked against the published DIN text (including the A1:2014 amendment of 1-1 and the 2010+ NA edi |
| `v-255.1` | ? |  | - | - | ? | cited by artifact: cited by an artifact check; add the exact value from the spec item |
| `v-256.1` | ? |  | - | - | ? | cited by artifact: cited by an artifact check; add the exact value from the spec item |
| `v-257.1` | ? |  | - | - | ? | cited by artifact: cited by an artifact check; add the exact value from the spec item |
| `v-305.1` | ? |  | - | - | ? | cited by artifact: cited by an artifact check; add the exact value from the spec item |
| `v-800.1` | ? |  | - | - | ? | en1993-target.md:281: **V-800 1-6 section 8 — Limit states LS1 plastic, LS2 cyclic, LS3 buckling, LS4 fatigue.** Computational models by shell theory |
| `v-851.1` | Vcrit = b fn/St |  | - | - | ? | en1993-target.md:291: **V-851 3-2 — Chimneys:** vortex shedding (`Vcrit = b fn/St`; Strouhal 0.18-0.2), ovalling (`ni = 0.4...`), fatigue from vortex; |
| `v-851.2` | Vcrit = b fn/St |  | - | - | ? | en1993-target.md:291: **V-851 3-2 — Chimneys:** vortex shedding (`Vcrit = b fn/St`; Strouhal 0.18-0.2), ovalling (`ni = 0.4...`), fatigue from vortex; |
| `v-905.1` | t = p d/(2 fy/gM0) |  | - | - | ? | en1993-target.md:487: **V-905 EN 1993-4-1/4-2/4-3 pipelines (4-3):** pipe wall `t = p d/(2 fy/gM0)` hoop stress (Barlow), `sigma_x` with temperature ( |
| `v-905.2` | t = p d/(2 fy/gM0) |  | - | - | ? | en1993-target.md:487: **V-905 EN 1993-4-1/4-2/4-3 pipelines (4-3):** pipe wall `t = p d/(2 fy/gM0)` hoop stress (Barlow), `sigma_x` with temperature ( |
| `v-905.3` | t = p d/(2 fy/gM0) |  | - | - | ? | en1993-target.md:487: **V-905 EN 1993-4-1/4-2/4-3 pipelines (4-3):** pipe wall `t = p d/(2 fy/gM0)` hoop stress (Barlow), `sigma_x` with temperature ( |

##### Clause EN 1990 Table A1.1 (3; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi-self.0` | ψ0 permanent |  | 1 | 1 | EN 1990 Table A1.1 |  |
| `psi-self.1` | ψ1 permanent |  | 1 | 1 | EN 1990 Table A1.1 |  |
| `psi-self.2` | ψ2 permanent |  | 1 | 1 | EN 1990 Table A1.1 |  |

##### Clause Table 2.1 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l402.1` | ? |  | - | - | Table 2.1 | en1993-target.md:402: Table 2.1 (NA allows alternative Tmd) \| |

##### Clause Table 2.1 / Annex C (24; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.grade.1-4003.e` | e grade 1.4003 | MPa | 200000 | 200000 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4003.fu` | fu grade 1.4003 | MPa | 450 | 450 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4003.fy` | fy grade 1.4003 | MPa | 250 | 250 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4003.n` | n grade 1.4003 |  | 14 | 14 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4301.e` | e grade 1.4301 | MPa | 200000 | 200000 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4301.fu` | fu grade 1.4301 | MPa | 520 | 520 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4301.fy` | fy grade 1.4301 | MPa | 210 | 210 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4301.n` | n grade 1.4301 |  | 6 | 6 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4401.e` | e grade 1.4401 | MPa | 200000 | 200000 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4401.fu` | fu grade 1.4401 | MPa | 520 | 520 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4401.fy` | fy grade 1.4401 | MPa | 220 | 220 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4401.n` | n grade 1.4401 |  | 7 | 7 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4404.e` | e grade 1.4404 | MPa | 200000 | 200000 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4404.fu` | fu grade 1.4404 | MPa | 520 | 520 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4404.fy` | fy grade 1.4404 | MPa | 220 | 220 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4404.n` | n grade 1.4404 |  | 7 | 7 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4462.e` | e grade 1.4462 | MPa | 200000 | 200000 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4462.fu` | fu grade 1.4462 | MPa | 640 | 640 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4462.fy` | fy grade 1.4462 | MPa | 460 | 460 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4462.n` | n grade 1.4462 |  | 5 | 5 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4571.e` | e grade 1.4571 | MPa | 200000 | 200000 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4571.fu` | fu grade 1.4571 | MPa | 520 | 520 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4571.fy` | fy grade 1.4571 | MPa | 220 | 220 | Table 2.1 / Annex C |  |
| `1-4.grade.1-4571.n` | n grade 1.4571 |  | 7 | 7 | Table 2.1 / Annex C |  |

##### Clause Table 5.2 (3; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-870.1` | βD |  | - | - | Table 5.2 | en1993-target.md:294: **V-870 5 — Steel piling (sheet piles, tubular piles):** section classification (Table 5.2/5 general), reduction for interlocks |
| `v-870.2` | βD |  | - | - | Table 5.2 | en1993-target.md:294: **V-870 5 — Steel piling (sheet piles, tubular piles):** section classification (Table 5.2/5 general), reduction for interlocks |
| `v-870.3` | βD |  | - | - | Table 5.2 | en1993-target.md:294: **V-870 5 — Steel piling (sheet piles, tubular piles):** section classification (Table 5.2/5 general), reduction for interlocks |

##### Clause Table 5.3 (6; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.rho-internal.a` | rho internal element a |  | 0.772 | 0.772 | Table 5.3 |  |
| `1-4.rho-internal.b.cold` | rho internal element b cold-formed |  | 0.125 | 0.125 | Table 5.3 |  |
| `1-4.rho-internal.b.welded` | rho internal element b welded |  | 0.079 | 0.079 | Table 5.3 |  |
| `1-4.rho-outstand.a` | rho outstand element a |  | 1 | 1 | Table 5.3 |  |
| `1-4.rho-outstand.b.cold` | rho outstand element b cold-formed |  | 0.231 | 0.231 | Table 5.3 |  |
| `1-4.rho-outstand.b.welded` | rho outstand element b welded |  | 0.242 | 0.242 | Table 5.3 |  |

##### Clause Table 5.4 (4; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-4.flexural.other.alpha` | alpha flexural other |  | 0.49 | 0.49 | Table 5.4 |  |
| `1-4.flexural.other.lambda0` | lambda_0 flexural other |  | 0.4 | 0.4 | Table 5.4 |  |
| `1-4.flexural.welded-open.alpha` | alpha flexural welded-open |  | 0.49 | 0.49 | Table 5.4 |  |
| `1-4.flexural.welded-open.lambda0` | lambda_0 flexural welded-open |  | 0.2 | 0.2 | Table 5.4 |  |

##### Clause Table 6.2 (1; DIN EN 1993/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `v-100.1` | Ncr = pi^2 E I / Lcr^2 |  | - | - | Table 6.2 | en1993-target.md:132: Table 6.2 as EN, with additional clarification for S460 curve a0 and use of Table NA** ****. |

##### Clause Table 8.1 (14; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-9.category.c100` | detail category 100 | MPa | 100 | 100 | Table 8.1 |  |
| `1-9.category.c112` | detail category 112 | MPa | 112 | 112 | Table 8.1 |  |
| `1-9.category.c125` | detail category 125 | MPa | 125 | 125 | Table 8.1 |  |
| `1-9.category.c140` | detail category 140 | MPa | 140 | 140 | Table 8.1 |  |
| `1-9.category.c160` | detail category 160 | MPa | 160 | 160 | Table 8.1 |  |
| `1-9.category.c36` | detail category 36 | MPa | 36 | 36 | Table 8.1 |  |
| `1-9.category.c40` | detail category 40 | MPa | 40 | 40 | Table 8.1 |  |
| `1-9.category.c45` | detail category 45 | MPa | 45 | 45 | Table 8.1 |  |
| `1-9.category.c50` | detail category 50 | MPa | 50 | 50 | Table 8.1 |  |
| `1-9.category.c56` | detail category 56 | MPa | 56 | 56 | Table 8.1 |  |
| `1-9.category.c63` | detail category 63 | MPa | 63 | 63 | Table 8.1 |  |
| `1-9.category.c71` | detail category 71 | MPa | 71 | 71 | Table 8.1 |  |
| `1-9.category.c80` | detail category 80 | MPa | 80 | 80 | Table 8.1 |  |
| `1-9.category.c90` | detail category 90 | MPa | 90 | 90 | Table 8.1 |  |

##### Clause Table 8.1 notes (2; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-9.size-effect.exponent` | size effect exponent |  | 0.2 | 0.2 | Table 8.1 notes |  |
| `1-9.size-effect.t-ref` | reference thickness | mm | 25 | 25 | Table 8.1 notes |  |

##### Clause Table 8.8 (5; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-9.deck.rib-splice-butt-weld` | orthotropic deck detail rib-splice-butt- | MPa | 90 | 90 | Table 8.8 |  |
| `1-9.deck.rib-to-crossbeam-web-cutout` | orthotropic deck detail rib-to-crossbeam | MPa | 71 | 71 | Table 8.8 |  |
| `1-9.deck.rib-to-deck-full-penetration` | orthotropic deck detail rib-to-deck-full | MPa | 125 | 125 | Table 8.8 |  |
| `1-9.deck.rib-to-deck-partial-penetration-plate` | orthotropic deck detail rib-to-deck-part | MPa | 71 | 71 | Table 8.8 |  |
| `1-9.deck.rib-to-deck-partial-penetration-throat` | orthotropic deck detail rib-to-deck-part | MPa | 36 | 36 | Table 8.8 |  |

##### Clause Table C.1 (39; DIN EN 1993-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-2.stainless-fire.1-4301.k02p.t100` | k02p grade 1.4301 at 100 C |  | 0.82 | 0.82 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t1000` | k02p grade 1.4301 at 1000 C |  | 0.06 | 0.06 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t1100` | k02p grade 1.4301 at 1100 C |  | 0.03 | 0.03 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t1200` | k02p grade 1.4301 at 1200 C |  | 0 | 0 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t20` | k02p grade 1.4301 at 20 C |  | 1 | 1 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t200` | k02p grade 1.4301 at 200 C |  | 0.68 | 0.68 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t300` | k02p grade 1.4301 at 300 C |  | 0.64 | 0.64 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t400` | k02p grade 1.4301 at 400 C |  | 0.6 | 0.6 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t500` | k02p grade 1.4301 at 500 C |  | 0.54 | 0.54 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t600` | k02p grade 1.4301 at 600 C |  | 0.49 | 0.49 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t700` | k02p grade 1.4301 at 700 C |  | 0.4 | 0.4 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t800` | k02p grade 1.4301 at 800 C |  | 0.27 | 0.27 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.k02p.t900` | k02p grade 1.4301 at 900 C |  | 0.14 | 0.14 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t100` | ke grade 1.4301 at 100 C |  | 0.96 | 0.96 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t1000` | ke grade 1.4301 at 1000 C |  | 0.2 | 0.2 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t1100` | ke grade 1.4301 at 1100 C |  | 0.1 | 0.1 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t1200` | ke grade 1.4301 at 1200 C |  | 0 | 0 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t20` | ke grade 1.4301 at 20 C |  | 1 | 1 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t200` | ke grade 1.4301 at 200 C |  | 0.92 | 0.92 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t300` | ke grade 1.4301 at 300 C |  | 0.88 | 0.88 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t400` | ke grade 1.4301 at 400 C |  | 0.84 | 0.84 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t500` | ke grade 1.4301 at 500 C |  | 0.8 | 0.8 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t600` | ke grade 1.4301 at 600 C |  | 0.76 | 0.76 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t700` | ke grade 1.4301 at 700 C |  | 0.71 | 0.71 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t800` | ke grade 1.4301 at 800 C |  | 0.63 | 0.63 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ke.t900` | ke grade 1.4301 at 900 C |  | 0.45 | 0.45 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t100` | ku grade 1.4301 at 100 C |  | 0.87 | 0.87 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t1000` | ku grade 1.4301 at 1000 C |  | 0.07 | 0.07 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t1100` | ku grade 1.4301 at 1100 C |  | 0.03 | 0.03 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t1200` | ku grade 1.4301 at 1200 C |  | 0 | 0 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t20` | ku grade 1.4301 at 20 C |  | 1 | 1 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t200` | ku grade 1.4301 at 200 C |  | 0.77 | 0.77 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t300` | ku grade 1.4301 at 300 C |  | 0.73 | 0.73 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t400` | ku grade 1.4301 at 400 C |  | 0.72 | 0.72 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t500` | ku grade 1.4301 at 500 C |  | 0.67 | 0.67 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t600` | ku grade 1.4301 at 600 C |  | 0.58 | 0.58 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t700` | ku grade 1.4301 at 700 C |  | 0.43 | 0.43 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t800` | ku grade 1.4301 at 800 C |  | 0.27 | 0.27 | Table C.1 |  |
| `1-2.stainless-fire.1-4301.ku.t900` | ku grade 1.4301 at 900 C |  | 0.15 | 0.15 | Table C.1 |  |

### EN 1994 (70)

#### EN 1994, part 1-1 (4)

##### Clause EN 1990 A1.4 (1; DIN EN 1994-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `deflection-divisor` | deflection span divisor |  | 250 | 300 | EN 1990 A1.4 |  |

##### Clause EN 1994-1-1 2.4.1.2 (3; DIN EN 1994-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-c` | γ_C |  | 1.5 | 1.5 | EN 1994-1-1 2.4.1.2 |  |
| `gamma-s` | γ_S |  | 1.15 | 1.15 | EN 1994-1-1 2.4.1.2 |  |
| `gamma-v` | γ_V |  | 1.25 | 1.25 | EN 1994-1-1 2.4.1.2 |  |

#### EN 1994, part 1-2 (13)

##### Clause 1.0 (1; DIN EN 1994-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l29.1` | ? |  | - | - | 1.0 | en1994-en1999-target.md:29: entries MUST be confirmed. |

##### Clause 3.2 (1; DIN EN 1994-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-45.1` | ? |  | - | - | 3.2 | en1994-en1999-target.md:260: **C-45 Thermal and mechanical properties ** — F:3.2 (steel: ky,θ, kE,θ, kp,θ tables |

##### Clause EN 1994-1-2 (9; DIN EN 1994-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `1-2.buckling-curve` | 1-2.buckling-curve |  | - | - | EN 1994-1-2 |  |
| `1-2.concrete-conduction` | 1-2.concrete-conduction |  | - | - | EN 1994-1-2 |  |
| `1-2.eps-c1-theta` | 1-2.eps-c1-theta |  | - | - | EN 1994-1-2 |  |
| `1-2.interaction` | 1-2.interaction |  | - | - | EN 1994-1-2 |  |
| `1-2.kc-theta` | 1-2.kc-theta |  | - | - | EN 1994-1-2 |  |
| `1-2.ke-theta` | 1-2.ke-theta |  | - | - | EN 1994-1-2 |  |
| `1-2.stiffness-phi` | 1-2.stiffness-phi |  | - | - | EN 1994-1-2 |  |
| `1-2.surface-exchange` | 1-2.surface-exchange |  | - | - | EN 1994-1-2 |  |
| `1-2.tables` | 1-2.tables |  | - | - | EN 1994-1-2 |  |

##### Clause Table 4.1 (1; DIN EN 1994-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-46.1` | ηfi |  | - | - | Table 4.1 | en1994-en1999-target.md:262: **C-46 Tabulated data ** — F:4.2: Table 4.1 (composite beams with partially encased sections: min dimensions b, bc, etc. |

##### Clause Table 6.2 (1; DIN EN 1994-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l694.1` | [T] |  | - | - | Table 6.2 | en1994-en1999-target.md:694: No test may rely on a value until transcribed and double-checked by a second reviewer. |

#### EN 1994, part 2 (13)

##### Clause 5.4 (1; DIN EN 1994-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-49.1` | ? |  | - | - | 5.4 | en1994-en1999-target.md:269: **C-49 Global analysis ** — 2:5.4: effective width with beff,1 mid-span and at supports |

##### Clause 6.2 (1; DIN EN 1994-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-50.1` | ? |  | - | - | 6.2 | en1994-en1999-target.md:270: **C-50 Classification and resistance of beams, 2:6.2 ** — same as C-07..C-10 with additional: 2:6.2.1.5? effective flange |

##### Clause 6.4 (1; DIN EN 1994-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-51.1` | ? |  | - | - | 6.4 | en1994-en1999-target.md:271: **C-51 Lateral-torsional buckling of bridges (U-frame) ** — 2:6.4: simplified: λLT <= 0.2 or Mpl,Rd criterion |

##### Clause 6.6.2 (2; DIN EN 1994-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-53.1` | ? |  | - | - | 6.6.2 | en1994-en1999-target.md:273: **C-53 Longitudinal shear in decks, 2:6.6.2 ** — transverse reinforcement as C-19 |
| `c-53.2` | ? |  | - | - | 6.6.2 | en1994-en1999-target.md:273: SLS: stress limitation 2:7.2.2 (concrete 0.6 fck, reinforcement 0.8 fsk, steel fy), crack width for decks wk <= 0.3 mm ( |

##### Clause 6.6.5 (4; DIN EN 1994-1-1/NA, DIN EN 1994-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `2.end-plate-bearing` | F_Rdu |  | - | - | 6.6.5 |  |
| `2.stud-cross-spacing` | e_t/d |  | 2.5 | - | 6.6.5 |  |
| `2.stud-edge-distance` | e_D | m | 0.02 | - | 6.6.5 |  |
| `c-54.1` | ? |  | - | - | 6.6.5 | en1994-en1999-target.md:274: **C-54 Bridge-specific detailing ** — 2:6.6.5, 2:6.7 composite plates/ box girders, 2:6.9 (concrete-filled chords). |

##### Clause 6.6.6.3 (1; DIN EN 1994-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `2.min-transverse-reinforcement` | rho_t |  | 0.002 | - | 6.6.6.3 |  |

##### Clause EN 1994-2 (2; DIN EN 1994-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `2.rebar-fatigue-range` | 2.rebar-fatigue-range |  | - | - | EN 1994-2 |  |
| `2.shear-lag` | 2.shear-lag |  | - | - | EN 1994-2 |  |

##### Clause EN 1994-2 6.8.2 (1; DIN EN 1994-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-mf` | γ_Mf bridge steel fatigue |  | 1.15 | 1.35 | EN 1994-2 6.8.2 |  **conflict** |

#### EN 1994, part not stated (40)

##### Clause 1.0 (2; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-19.2` | ? |  | - | - | 1.0 | en1994-en1999-target.md:150: Minimum transverse reinforcement (6.6.6.3): >= 0.002 Ac (and ≥ 0.2 % of the concrete area, NDP) wait: value 0.002 hc: [N] |
| `c-30.1` | ? |  | - | - | 1.0 | en1994-en1999-target.md:205: Ncr,eff with (EI)eff,II = K0 (Ea Ia + Es Is + Ke,II Ecm,eff Ic), K0 = 0.9, Ke,II = 0.5 ( confirm). |

##### Clause 1.1 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-01.1` | ? |  | - | - | 1.1 | en1994-en1999-target.md:50: - NDP: αcc: (EN 1994 applies 0.85 explicitly |

##### Clause 1.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-11.1` | ? |  | - | - | 1.2 | en1994-en1999-target.md:107: **C-11 Generic stress-block integrator ** — implementation support for C-07/C-08/C-12. |

##### Clause 1.3 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-17.1` | ? |  | - | - | 1.3 | en1994-en1999-target.md:141: - NA-DE: restricts sheeting stud resistance additionally (kt <= value dependent on sheeting thickness, and the max number |

##### Clause 1.6 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-36.1` | ? |  | - | - | 1.6 | en1994-en1999-target.md:230: **C-36 Partial connection method (τu) ** — see C-34 |

##### Clause 1.25 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-16.1` | ? |  | - | - | 1.25 | en1994-en1999-target.md:133: NA-DE adds rules for lightweight concrete (η-factor on PRd,2: PRd,2 reduced by (0.9 + 0.1 ... ρ/2200)) . |

##### Clause 5.4.1.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-03.1` | ? |  | - | - | 5.4.1.2 | en1994-en1999-target.md:60: **C-03 Effective width of concrete flange ** — cl. 5.4.1.2, Fig. 5.1. |

##### Clause 5.4.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-04.1` | ? |  | - | - | 5.4.2 | en1994-en1999-target.md:69: **C-04 Global analysis method selection ** — cl. 5.4.2 (beams in buildings) / 2:5.4.2 (bridges). |

##### Clause 5.4.2.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-05.1` | ? |  | - | - | 5.4.2.2 | en1994-en1999-target.md:72: **C-05 Shrinkage, creep and modular ratios ** — cl. 5.4.2.2, 3.1(4), 7.2.1(?). |

##### Clause 6.2.1.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-07.1` | ? |  | - | - | 6.2.1.2 | en1994-en1999-target.md:84: **C-07 Plastic resistance moment, sagging, full shear connection ** — cl. 6.2.1.2. |

##### Clause 6.2.1.2(2) (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-08.1` | ? |  | - | - | 6.2.1.2(2) | en1994-en1999-target.md:95: **C-08 Plastic resistance moment, hogging ** — cl. 6.2.1.2(2), 6.2.1.3. |

##### Clause 6.2.1.3 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-12.1` | ? |  | - | - | 6.2.1.3 | en1994-en1999-target.md:109: **C-12 Partial shear connection in beams — stress-block method ** — cl. 6.2.1.3(3..), 6.6.1.2. |

##### Clause 6.2.1.3(3) (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-13.1` | ? |  | - | - | 6.2.1.3(3) | en1994-en1999-target.md:116: **C-13 Partial shear connection — linear interpolation ** — cl. 6.2.1.3(3), Fig. 6.? EN: "Linear interpolation (conservat |

##### Clause 6.2.1.4 (2; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-06.1` | ? |  | - | - | 6.2.1.4 | en1994-en1999-target.md:78: **C-06 Section properties (transformed) ** — 6.2.1.4, 7.2.1. |
| `c-09.1` | ? |  | - | - | 6.2.1.4 | en1994-en1999-target.md:98: **C-09 Elastic resistance, class 3/4, staged analysis ** — cl. 6.2.1.4, 7.2.1. |

##### Clause 6.2.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-22.1` | ? |  | - | - | 6.2.2 | en1994-en1999-target.md:161: **C-22 Vertical shear resistance ** — cl. 6.2.2. |

##### Clause 6.6 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l27.1` | ? |  | - | - | 6.6 | en1994-en1999-target.md:27: \| γMf \| fatigue / Ermüdung \| see 1999-1-3 (§6.6) \| \| |

##### Clause 6.6.1.1 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-15.1` | ? |  | - | - | 6.6.1.1 | en1994-en1999-target.md:125: **C-15 Number and distribution of connectors ** — cl. 6.6.1.1, 6.6.1.2, 6.6.1.3. nf = Ncf/PRd (or Npl,a/PRd) |

##### Clause 6.6.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-18.1` | ? |  | - | - | 6.6.2 | en1994-en1999-target.md:146: **C-18 Longitudinal shear at connectors, ULS ** — cl. 6.6.2, 6.6.3.1: demand per length vl,Ed = ΔFcd/Δx from the force di |

##### Clause 6.6.5.1 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-20.1` | ? |  | - | - | 6.6.5.1 | en1994-en1999-target.md:155: **C-20 Resistance to separation / transverse reinforcement at studs ** — cl. 6.6.5.1–6.6.5.5, 6.6.5.2: detailing: edge di |

##### Clause 6.7.1 (2; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-25.1` | in-scope\|out-of-scope |  | - | - | 6.7.1 | en1994-en1999-target.md:178: **C-25 Scope check ** — cl. 6.7.1: doubly symmetric uniform section |
| `c-25.2` | in-scope\|out-of-scope |  | - | - | 6.7.1 | en1994-en1999-target.md:178: for fully encased: cover limits cz <= 0.3 h, cy <= 0.4 b and cz >= 40 mm (or b/6) |

##### Clause 6.7.3.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-26.1` | ? |  | - | - | 6.7.3.2 | en1994-en1999-target.md:180: **C-26 Plastic resistance to compression Npl,Rd ** — cl. 6.7.3.2, 6.7.3.2(1),(6.30), (6.33). |

##### Clause 6.7.3.3 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-27.1` | ? |  | - | - | 6.7.3.3 | en1994-en1999-target.md:186: **C-27 Relative slenderness and buckling resistance ** — cl. 6.7.3.3, 6.7.3.5. |

##### Clause 6.7.3.6 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-29.1` | ? |  | - | - | 6.7.3.6 | en1994-en1999-target.md:194: **C-29 Interaction polygon (M–N) ** — cl. 6.7.3.6. |

##### Clause 7.2.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-41.1` | ? |  | - | - | 7.2.2 | en1994-en1999-target.md:248: concrete 0.6 fck for exposure XD/XF/XS) [T: values] |

##### Clause 7.3 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-40.1` | ? |  | - | - | 7.3 | en1994-en1999-target.md:242: **C-40 Deflection with creep/shrinkage ** — cl. 7.3, 7.2. |

##### Clause 7.4.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-42.1` | ? |  | - | - | 7.4.2 | en1994-en1999-target.md:250: **C-42 Crack control: minimum reinforcement ** — cl. 7.4.2. |

##### Clause 9.7.2 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-34.1` | ? |  | - | - | 9.7.2 | en1994-en1999-target.md:217: **C-34 Composite slab bending — full/partial connection ** — cl. 9.7.2–9.7.3. |

##### Clause 9.7.3(4) (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-35.1` | ? |  | - | - | 9.7.3(4) | en1994-en1999-target.md:225: **C-35 Composite slab longitudinal shear — m–k method ** — cl. 9.7.3(4). |

##### Clause 9.7.5 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-37.1` | ? |  | - | - | 9.7.5 | en1994-en1999-target.md:232: **C-37 Composite slab vertical shear ** — cl. 9.7.5. |

##### Clause 9.8 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-39.1` | ? |  | - | - | 9.8 | en1994-en1999-target.md:238: **C-39 Composite slab SLS and detailing ** — cl. 9.8: deflection of the slab (cracked I with reduced flexural stiffness |

##### Clause ? (3; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e1-12.1` | ? |  | - | - | 7. Worked Examples EN 1994 (hand-checked) | en1994-en1999-target.md:643: **E1-12 Fire tabulated and simple (C-46, C-47)** — slab R60: heff >= value from table (data load ) |
| `e1-13.1` | ? |  | - | - | 7. Worked Examples EN 1994 (hand-checked) | en1994-en1999-target.md:645: **E1-13 LTB U-frame (C-23)** — acceptance example is built from the transcribed kc/C4: fixture to be completed when the c |
| `e2-12.3` | ? |  | - | - | 8. Worked Examples EN 1999 (hand-checked) | en1994-en1999-target.md:677: E1-12, E1-13, E2-11 are partially and MUST be completed after table transcription.) |

##### Clause Table 4. (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-32.1` | ? |  | - | - | Table 4. | en1994-en1999-target.md:211: **C-32 Composite column fire (F:4.3–4.4, Annex G/H) ** — tabulated: Table 4.? minimum dimensions for filled CHS/RHS and e |

##### Clause Table 5.2. (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-02.1` | ? |  | - | - | Table 5.2. | en1994-en1999-target.md:54: **C-02 Classification of steel elements in composite members ** — cl. 5.5, Table 5.2. |

##### Clause Table 6. (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e1-13.2` | ? |  | - | - | Table 6. | en1994-en1999-target.md:645: the simplified exemption check is testable now: IPE 400, S355, adjacent span ratio >= 0.6, UDL, class 1/2 -> exempt if th |

##### Clause Table 6.6 (1; DIN EN 1994/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `c-31.1` | ? |  | - | - | Table 6.6 | en1994-en1999-target.md:209: **C-31 Introduction of loads / shear transfer ** — cl. 6.7.4. τRd (Table 6.6): completely concrete-encased 0.30 MPa |

### EN 1995 (537)

#### EN 1995, part 1-1 (35)

##### Clause 1.1 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-50.1` | n |  | - | - | 1.1 | en1995-en1996-target.md:203: for glued laminated/solid trusses with `n` members, EN 1995-1-1 6.6 (2) ****. |

##### Clause 1.3 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-03.1` | γM = 1.25 |  | - | - | 1.3 | en1995-en1996-target.md:43: CLT per Z-9.1-559/ETA and the 2nd-generation EN 1995-1-1:2025 `γM = 1.25` (DE value of CLT ****). |

##### Clause 2.00 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-04.1` | KdefTable |  | - | - | 2.00 | en1995-en1996-target.md:48: MDF.HLS 3.00/4.00 **** (data table `KdefTable`). |

##### Clause 2.3.2.2 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-04.2` | kser |  | - | - | 2.3.2.2 | en1995-en1996-target.md:51: - Connections: `kser` slip, `ufin = uinst (1 + 2 kdef)` for components of different creep behaviour (2.3.2.2 (3)) ****. |

##### Clause 2.4.1 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-06.1` | Ed = Emean |  | - | - | 2.4.1 | en1995-en1996-target.md:64: stiffness `Ed = Emean` (SLS), `E0,05` (second-order/stability, 2.4.1 (2)), `Ed(ULS) = Emean/γM` for second-order analysis |

##### Clause 5.0 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-97.1` | fr,k |  | - | - | 5.0 | en1995-en1996-target.md:374: NA-DE `fr,k` from Z-9.1-559: 1.1 N/mm² for adjacent layers with width-to-thickness ≥ 4) ****. |

##### Clause 5.2 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-18.2` | HoleRules |  | - | - | 5.2 | en1995-en1996-target.md:123: geometric limits as data in `HoleRules` ****. |

##### Clause 6.1.7 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-15.2` | Vd |  | - | - | 6.1.7 | en1995-en1996-target.md:107: `Vd` of loads near supports (`a < 2h`): EN 1995-1-1 6.1.7 (3) reduction `β = a/(2h)` of the point load within `2h` of the |

##### Clause 6.1.7(2) (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k-cr.en-constant` | k_cr EN |  | 0.67 | - | 6.1.7(2) |  |

##### Clause 7.2 (2; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `w-fin-divisor` | w_fin limit L/n |  | 250 | 200 | 7.2 |  |
| `w-inst-divisor` | w_inst limit L/n |  | 300 | 300 | 7.2 |  |

##### Clause 7.3.3 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-63.1` | f1 < 6 Hz |  | - | - | 7.3.3 | en1995-en1996-target.md:226: `f1 < 6 Hz` ⇒ special investigation (Wohnungstrenndecken, Hamm-Richter) ****. |

##### Clause ? (2; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-bearing-perpendicular.2` | kl |  | - | - | EN 1995 examples (E1–E10) | en1995-en1996-target.md:865: includes `kl`, `kp`, `kdis`, `kvol` hand-computed by the oracle and stored as fixture (****). |
| `t-33.1` | reinforcement |  | - | - | T-33 Reinforced apex zone (NA-DE NCI, DIN EN 1995- | en1995-en1996-target.md:184: Parameters in data ****. |

##### Clause EN 1995-1-1 2.3.2.2 and 3.2 (model assumption) (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `creep.time-constant-years` | time constant tau of the creep deflectio | a | 5 | 5 | EN 1995-1-1 2.3.2.2 and 3.2 (model assumption) | wave4: constant of the EN 1995 floor, creep and CLT entities: w(t) = w_inst (1 + k_def (1 - exp(-t/tau))): the standard fixes only the end value w_fin |

##### Clause EN 1995-1-1 7.3.3 (3) and NA-DE (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `floor.f1-limit` | lower limit of the floor fundamental fre | Hz | 8 | 6 | EN 1995-1-1 7.3.3 (3) and NA-DE | wave4: constant of the EN 1995 floor, creep and CLT entities: EN recommends 8 Hz; the German NA value 6 Hz is not confirmed by a public source (ndp-pu |

##### Clause EN 1995-1-1 7.3.3 (4) and NA-DE (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `floor.unit-load-a` | unit point load deflection limit a | mm/kN | 1.5 | 0.5 | EN 1995-1-1 7.3.3 (4) and NA-DE | wave4: constant of the EN 1995 floor, creep and CLT entities: a = 1.5 mm/kN recommended in the EN; NA-DE 0.5 mm/kN, unverified |

##### Clause EN 1995-1-1 Table C.1 (4; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `annex-c.eta.bolted` | eta bolted |  | 6 | 6 | EN 1995-1-1 Table C.1 | wave3: constant of the EN 1995 kernel formerly hard-coded: Annex C eta for bolted joints |
| `annex-c.eta.connectors` | eta connectors |  | 3 | 3 | EN 1995-1-1 Table C.1 | wave3: constant of the EN 1995 kernel formerly hard-coded: Annex C eta for connectors |
| `annex-c.eta.glued` | eta glued |  | 1 | 1 | EN 1995-1-1 Table C.1 | wave3: constant of the EN 1995 kernel formerly hard-coded: Annex C eta for glued joints |
| `annex-c.eta.nailed` | eta nailed |  | 4 | 4 | EN 1995-1-1 Table C.1 | wave3: constant of the EN 1995 kernel formerly hard-coded: Annex C eta for nailed joints (default of the catalogue) |

##### Clause EN 1995-1-1/NA 7.3.3 NCI (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `floor.na-de-additional-criterion` | NA-DE additional unit-load criterion app | Hz | 8 | 8 | EN 1995-1-1/NA 7.3.3 NCI | wave4: constant of the EN 1995 floor, creep and CLT entities: NA-DE: between 6 Hz and 8 Hz the unit-load criterion is mandatory (unverified) |

##### Clause NA 6.1.7(2) (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k-cr.de-numerator` | k_cr DE numerator | N/mm2 | - | 2.5 | NA 6.1.7(2) |  |

##### Clause Table 2.3 / NA.2.4.1 (10; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-m.clt` | γ_M clt |  | 1.25 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.connection` | γ_M connection |  | 1.3 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.fibreboard` | γ_M fibreboard |  | 1.3 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.glulam` | γ_M glulam |  | 1.25 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.lvl` | γ_M lvl |  | 1.2 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.osb` | γ_M osb |  | 1.2 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.particleboard` | γ_M particleboard |  | 1.3 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.plywood` | γ_M plywood |  | 1.2 | 1.3 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.punched-plate` | γ_M punched-plate |  | 1.25 | 1.25 | Table 2.3 / NA.2.4.1 |  |
| `gamma-m.solid` | γ_M solid |  | 1.3 | 1.3 | Table 2.3 / NA.2.4.1 |  |

##### Clause Table 3.1 (2; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-02.1` | KmodTable |  | - | - | Table 3.1 | en1995-en1996-target.md:35: particleboard P5, P7 and fibreboard values per Table 3.1 **** (data table `KmodTable`). |
| `t-02.2` | H > 1000 m |  | - | - | Table 3.1 | en1995-en1996-target.md:37: snow `H > 1000 m`: medium) ****. |

#### EN 1995, part 1-2 (19)

##### Clause 3.4.3 (1; DIN EN 1995-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-103.1` | tf |  | - | - | 3.4.3 | en1995-en1996-target.md:408: Data-driven formulas from EN 1995-1-2 3.4.3.3 ****. |

##### Clause ? (1; DIN EN 1995-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-105.1` | , with |  | - | - | T-105 Fire tabulated: timber floors/walls (cavity) | en1995-en1996-target.md:417: Data-based Annex C **** |

##### Clause EN 1990-1-2 / EN 1991-1-2 2.4.2 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `panel.fire-load-ratio` | reduction factor eta_fi of the load in f |  | 0.6 | 0.6 | EN 1990-1-2 / EN 1991-1-2 2.4.2 | wave4: constant of the EN 1995 floor, creep and CLT entities: E_fi,d = eta_fi E_d with the recommended eta_fi = 0.6 (unverified, depends on psi_fi and |

##### Clause EN 1995-1-2 4.2.3 (4; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire.reduced-bending-divisor` | k_mod,fi bending divisor |  | 200 | 200 | EN 1995-1-2 4.2.3 | wave3: constant of the EN 1995 kernel formerly hard-coded: k_mod,fi = 1 - p/(200 A_r) for bending |
| `fire.reduced-compression-divisor` | k_mod,fi compression divisor |  | 125 | 125 | EN 1995-1-2 4.2.3 | wave3: constant of the EN 1995 kernel formerly hard-coded: k_mod,fi = 1 - p/(125 A_r) for compression |
| `fire.reduced-tension-divisor` | k_mod,fi tension and stiffness divisor |  | 330 | 330 | EN 1995-1-2 4.2.3 | wave3: constant of the EN 1995 kernel formerly hard-coded: k_mod,fi = 1 - p/(330 A_r) for tension and modulus |
| `fire.reduced-time-limit` | t of linear k_mod,fi growth | min | 20 | 20 | EN 1995-1-2 4.2.3 | wave3: constant of the EN 1995 kernel formerly hard-coded: k_mod,fi interpolated linearly in time below 20 min |

##### Clause EN 1995-1-2 6.2 (1; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire.connection-reduced-load-k` | k of t_fi,d = -ln(eta)/k | 1/min | - | - | EN 1995-1-2 6.2 | wave3: constant of the EN 1995 kernel formerly hard-coded: reduced-load connection constant; no value is available, the case supplies k_fire |

##### Clause EN 1995-1-2 CLT research model (2; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire.clt-fall-off-factor` | CLT char rate factor after glue line fal |  | 2 | 2 | EN 1995-1-2 CLT research model | wave3: constant of the EN 1995 kernel formerly hard-coded: char rate factor 2 after a glue line (Schmid/Frangi research model, not in the printed stan |
| `fire.clt-fall-off-protect` | new char depth protecting the next CLT l | mm | 25 | 25 | EN 1995-1-2 CLT research model | wave3: constant of the EN 1995 kernel formerly hard-coded: factor applies until 25 mm of new char protect the layer (research model) |

##### Clause EN 1995-1-2 Table 2.1 (4; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k-fi.clt` | k_fi clt |  | 1.15 | 1.15 | EN 1995-1-2 Table 2.1 |  |
| `k-fi.glulam` | k_fi glulam |  | 1.15 | 1.15 | EN 1995-1-2 Table 2.1 |  |
| `k-fi.lvl` | k_fi lvl |  | 1.1 | 1.1 | EN 1995-1-2 Table 2.1 |  |
| `k-fi.solid` | k_fi solid |  | 1.25 | 1.25 | EN 1995-1-2 Table 2.1 |  |

##### Clause EN 1995-1-2 Table 3.1 (5; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `beta-0` | β_0 | mm/min | 0.65 | 0.65 | EN 1995-1-2 Table 3.1 |  |
| `beta-n.clt` | β_n clt | mm/min | 0.65 | 0.65 | EN 1995-1-2 Table 3.1 |  |
| `beta-n.glulam` | β_n glulam | mm/min | 0.7 | 0.7 | EN 1995-1-2 Table 3.1 |  |
| `beta-n.lvl` | β_n lvl | mm/min | 0.7 | 0.7 | EN 1995-1-2 Table 3.1 |  |
| `beta-n.solid` | β_n solid | mm/min | 0.8 | 0.8 | EN 1995-1-2 Table 3.1 |  |

#### EN 1995, part 2 (7)

##### Clause 6.2 (1; DIN EN 1995-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-121.1` | Δσ > 0.2 fd |  | - | - | 6.2 | en1995-en1996-target.md:429: traffic-induced fatigue for glulam bridge decks is only verified for members with `Δσ > 0.2 fd`... simplified `kfat = 1 − |

##### Clause EN 1995-2 7.3 / Setra (6; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `bridge.crowd-factor` | crowd synchronisation factor |  | 10.8 | 10.8 | EN 1995-2 7.3 / Setra | wave3: constant of the EN 1995 kernel formerly hard-coded: 10.8·√(ξ/n) (Setra footbridge guide) |
| `bridge.crowd-force` | dynamic force per pedestrian | N | 280 | 280 | EN 1995-2 7.3 / Setra | wave3: constant of the EN 1995 kernel formerly hard-coded: d·280 N·10.8·√(ξ/n)·ψ (Setra footbridge guide) |
| `bridge.crowd-psi-end` | psi vanishes above frequency | Hz | 2.1 | 2.1 | EN 1995-2 7.3 / Setra | wave3: constant of the EN 1995 kernel formerly hard-coded: ψ = 0 above 2.1 Hz |
| `bridge.crowd-psi-full-from` | psi equals 1 from frequency | Hz | 1 | 1 | EN 1995-2 7.3 / Setra | wave3: constant of the EN 1995 kernel formerly hard-coded: ψ = 1 between 1.0 and 1.7 Hz |
| `bridge.crowd-psi-full-to` | psi equals 1 up to frequency | Hz | 1.7 | 1.7 | EN 1995-2 7.3 / Setra | wave3: constant of the EN 1995 kernel formerly hard-coded: ψ = 1 between 1.0 and 1.7 Hz |
| `bridge.crowd-psi-start` | psi starts above frequency | Hz | 0.5 | 0.5 | EN 1995-2 7.3 / Setra | wave3: constant of the EN 1995 kernel formerly hard-coded: ψ = 0 below 0.5 Hz |

#### EN 1995, part not stated (476)

##### Clause 1.0 (2; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-15.1` | kcr = 1.0 |  | - | - | 1.0 | en1995-en1996-target.md:105: GL28h: 0.714), `kcr = 1.0` for LVL, `kcr = 1.0` for panels ****. |
| `t-20.1` | λ |  | - | - | 1.0 | en1995-en1996-target.md:136: - Edge: `λ` ≤ 150 recommended maximum (NA-DE: ≤ 150 for general, 170 for secondary) **** |

##### Clause 1.3 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-46.1` | x |  | - | - | 1.3 | en1995-en1996-target.md:197: check against T-68 withdrawal. ****. |

##### Clause 1.5 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-81.1` | kt = min(1.0, …) |  | - | - | 1.5 | en1995-en1996-target.md:319: as data tables ****. |

##### Clause 1.25 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-82.1` | … |  | - | - | 1.25 | en1995-en1996-target.md:323: Output anchorage, steel net-section, interaction `…` ****. |

##### Clause 2.6 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-70.1` | d = deff = 1.1 dc? |  | - | - | 2.6 | en1995-en1996-target.md:249: screws: `d = deff = 1.1 dc?` per 8.7 (`deff = 1.1 × thread root diameter`, `My,Rk = 0.15 fu,k deff^2.6`) **** for screws. |

##### Clause 3.4.2 (2; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-100.1` | 0.9/- |  | - | - | 3.4.2 | en1995-en1996-target.md:390: wood panelling/plywood `0.9/- `, wood-based panels `0.9` **** (table in data). |
| `t-100.2` | k_ρ |  | - | - | 3.4.2 | en1995-en1996-target.md:391: - Density correction `k_ρ` and thickness correction `kh` for panels (`β0,ρ,t = β0 kρ kh`) with `kρ = √(450/ρk)`, `kh = √( |

##### Clause 4.5 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e3-bearing-perpendicular.1` | f1 ≥ 6 Hz |  | - | - | 4.5 | en1995-en1996-target.md:863: passes the NA-DE `f1 ≥ 6 Hz` stage (then stiffness criterion required) ****. |

##### Clause 5.2 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-18.1` | Anet |  | - | - | 5.2 | en1995-en1996-target.md:122: tension net section: `Anet` of all fasteners within `0.5 d`... **** the staggering rule `a < 4d`. |

##### Clause 5.4.4 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-24.1` | ec = βc · lef/200 |  | - | - | 5.4.4 | en1995-en1996-target.md:158: - Bow imperfection `ec = βc · lef/200` (solid, `βc = 0.0025` ⇒ `lef/400` for glulam, `lef/500` LVL)`L/400` (solid), `L/50 |

##### Clause 6.1.5 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-13.1` | kc,90 |  | - | - | 6.1.5 | en1995-en1996-target.md:92: NA-DE: same shape and, additionally, `kc,90` capped by the NA limits `l ≤ 400 mm` for glulam and `kc,90 = 1.0` for hardwoo |

##### Clause 6.3.3 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-22.1` | 0.4 h |  | - | - | 6.3.3 | en1995-en1996-target.md:148: For members with one section dimension ≤ `0.4 h` etc. special cases ****. |

##### Clause 6.5.2 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-45.1` | kv = 1 |  | - | - | 6.5.2 | en1995-en1996-target.md:192: notches on the span side `kv = 1`, and `x ≤ 0.4 h` **** (`NCI zu 6.5.2`: Ausklinkungen mit `x/h`, Verstärkung per 6.5.3). |

##### Clause 7.3.3 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-64.1` | a = 0.5 mm/kN |  | - | - | 7.3.3 | en1995-en1996-target.md:231: NA-DE: `a = 0.5 mm/kN`? use data `NdpVibration.a`. |

##### Clause 8.1.4 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-78.1` | α |  | - | - | 8.1.4 | en1995-en1996-target.md:307: the splitting check for connections loaded at an angle `α`, with `ke` factors (NCI) ****. |

##### Clause 9.1 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-76.1` | ScrewApproval |  | - | - | 9.1 | en1995-en1996-target.md:296: Data in `ScrewApproval` entity ****. |

##### Clause 9.1.1 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-91.1` | hw/bw > 70 |  | - | - | 9.1.1 | en1995-en1996-target.md:340: web buckling for `hw/bw > 70` per 9.1.1 (1) sets `hw/bw ≤ 70`... **** |

##### Clause Table 1 (204; EN 338:2016 Table 1, EN 338:2016 Table 1 (unconfirmed transcription))

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `strength.c14.e005` | E_0,05 C14 | N/mm2 | 4700 | 4700 | Table 1 |  |
| `strength.c14.e0mean` | E_0,mean C14 | N/mm2 | 7000 | 7000 | Table 1 |  |
| `strength.c14.e90mean` | E_90,mean C14 | N/mm2 | 230 | 230 | Table 1 |  |
| `strength.c14.fc0k` | f_c,0,k C14 | N/mm2 | 16 | 16 | Table 1 |  |
| `strength.c14.fc90k` | f_c,90,k C14 | N/mm2 | 2 | 2 | Table 1 |  |
| `strength.c14.fmk` | f_m,k C14 | N/mm2 | 14 | 14 | Table 1 |  |
| `strength.c14.ft0k` | f_t,0,k C14 | N/mm2 | 7.2 | 7.2 | Table 1 |  |
| `strength.c14.ft90k` | f_t,90,k C14 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c14.fvk` | f_v,k C14 | N/mm2 | 3 | 3 | Table 1 |  |
| `strength.c14.gmean` | G_mean C14 | N/mm2 | 440 | 440 | Table 1 |  |
| `strength.c14.rhok` | ρ_k C14 | kg/m3 | 290 | 290 | Table 1 |  |
| `strength.c14.rhomean` | ρ_mean C14 | kg/m3 | 350 | 350 | Table 1 |  |
| `strength.c18.e005` | E_0,05 C18 | N/mm2 | 6000 | 6000 | Table 1 |  |
| `strength.c18.e0mean` | E_0,mean C18 | N/mm2 | 9000 | 9000 | Table 1 |  |
| `strength.c18.e90mean` | E_90,mean C18 | N/mm2 | 300 | 300 | Table 1 |  |
| `strength.c18.fc0k` | f_c,0,k C18 | N/mm2 | 18 | 18 | Table 1 |  |
| `strength.c18.fc90k` | f_c,90,k C18 | N/mm2 | 2.2 | 2.2 | Table 1 |  |
| `strength.c18.fmk` | f_m,k C18 | N/mm2 | 18 | 18 | Table 1 |  |
| `strength.c18.ft0k` | f_t,0,k C18 | N/mm2 | 10 | 10 | Table 1 |  |
| `strength.c18.ft90k` | f_t,90,k C18 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c18.fvk` | f_v,k C18 | N/mm2 | 3.4 | 3.4 | Table 1 |  |
| `strength.c18.gmean` | G_mean C18 | N/mm2 | 560 | 560 | Table 1 |  |
| `strength.c18.rhok` | ρ_k C18 | kg/m3 | 320 | 320 | Table 1 |  |
| `strength.c18.rhomean` | ρ_mean C18 | kg/m3 | 380 | 380 | Table 1 |  |
| `strength.c20.e005` | E_0,05 C20 | N/mm2 | 6400 | 6400 | Table 1 |  |
| `strength.c20.e0mean` | E_0,mean C20 | N/mm2 | 9500 | 9500 | Table 1 |  |
| `strength.c20.e90mean` | E_90,mean C20 | N/mm2 | 320 | 320 | Table 1 |  |
| `strength.c20.fc0k` | f_c,0,k C20 | N/mm2 | 19 | 19 | Table 1 |  |
| `strength.c20.fc90k` | f_c,90,k C20 | N/mm2 | 2.3 | 2.3 | Table 1 |  |
| `strength.c20.fmk` | f_m,k C20 | N/mm2 | 20 | 20 | Table 1 |  |
| `strength.c20.ft0k` | f_t,0,k C20 | N/mm2 | 11.5 | 11.5 | Table 1 |  |
| `strength.c20.ft90k` | f_t,90,k C20 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c20.fvk` | f_v,k C20 | N/mm2 | 3.6 | 3.6 | Table 1 |  |
| `strength.c20.gmean` | G_mean C20 | N/mm2 | 590 | 590 | Table 1 |  |
| `strength.c20.rhok` | ρ_k C20 | kg/m3 | 330 | 330 | Table 1 |  |
| `strength.c20.rhomean` | ρ_mean C20 | kg/m3 | 390 | 390 | Table 1 |  |
| `strength.c22.e005` | E_0,05 C22 | N/mm2 | 6700 | 6700 | Table 1 |  |
| `strength.c22.e0mean` | E_0,mean C22 | N/mm2 | 10000 | 10000 | Table 1 |  |
| `strength.c22.e90mean` | E_90,mean C22 | N/mm2 | 330 | 330 | Table 1 |  |
| `strength.c22.fc0k` | f_c,0,k C22 | N/mm2 | 20 | 20 | Table 1 |  |
| `strength.c22.fc90k` | f_c,90,k C22 | N/mm2 | 2.4 | 2.4 | Table 1 |  |
| `strength.c22.fmk` | f_m,k C22 | N/mm2 | 22 | 22 | Table 1 |  |
| `strength.c22.ft0k` | f_t,0,k C22 | N/mm2 | 13 | 13 | Table 1 |  |
| `strength.c22.ft90k` | f_t,90,k C22 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c22.fvk` | f_v,k C22 | N/mm2 | 3.8 | 3.8 | Table 1 |  |
| `strength.c22.gmean` | G_mean C22 | N/mm2 | 630 | 630 | Table 1 |  |
| `strength.c22.rhok` | ρ_k C22 | kg/m3 | 340 | 340 | Table 1 |  |
| `strength.c22.rhomean` | ρ_mean C22 | kg/m3 | 410 | 410 | Table 1 |  |
| `strength.c27.e005` | E_0,05 C27 | N/mm2 | 7700 | 7700 | Table 1 |  |
| `strength.c27.e0mean` | E_0,mean C27 | N/mm2 | 11500 | 11500 | Table 1 |  |
| `strength.c27.e90mean` | E_90,mean C27 | N/mm2 | 380 | 380 | Table 1 |  |
| `strength.c27.fc0k` | f_c,0,k C27 | N/mm2 | 22 | 22 | Table 1 |  |
| `strength.c27.fc90k` | f_c,90,k C27 | N/mm2 | 2.6 | 2.6 | Table 1 |  |
| `strength.c27.fmk` | f_m,k C27 | N/mm2 | 27 | 27 | Table 1 |  |
| `strength.c27.ft0k` | f_t,0,k C27 | N/mm2 | 16.5 | 16.5 | Table 1 |  |
| `strength.c27.ft90k` | f_t,90,k C27 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c27.fvk` | f_v,k C27 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.c27.gmean` | G_mean C27 | N/mm2 | 720 | 720 | Table 1 |  |
| `strength.c27.rhok` | ρ_k C27 | kg/m3 | 360 | 360 | Table 1 |  |
| `strength.c27.rhomean` | ρ_mean C27 | kg/m3 | 430 | 430 | Table 1 |  |
| `strength.c35.e005` | E_0,05 C35 | N/mm2 | 8700 | 8700 | Table 1 |  |
| `strength.c35.e0mean` | E_0,mean C35 | N/mm2 | 13000 | 13000 | Table 1 |  |
| `strength.c35.e90mean` | E_90,mean C35 | N/mm2 | 430 | 430 | Table 1 |  |
| `strength.c35.fc0k` | f_c,0,k C35 | N/mm2 | 25 | 25 | Table 1 |  |
| `strength.c35.fc90k` | f_c,90,k C35 | N/mm2 | 2.8 | 2.8 | Table 1 |  |
| `strength.c35.fmk` | f_m,k C35 | N/mm2 | 35 | 35 | Table 1 |  |
| `strength.c35.ft0k` | f_t,0,k C35 | N/mm2 | 22.5 | 22.5 | Table 1 |  |
| `strength.c35.ft90k` | f_t,90,k C35 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c35.fvk` | f_v,k C35 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.c35.gmean` | G_mean C35 | N/mm2 | 810 | 810 | Table 1 |  |
| `strength.c35.rhok` | ρ_k C35 | kg/m3 | 390 | 390 | Table 1 |  |
| `strength.c35.rhomean` | ρ_mean C35 | kg/m3 | 470 | 470 | Table 1 |  |
| `strength.c40.e005` | E_0,05 C40 | N/mm2 | 9400 | 9400 | Table 1 |  |
| `strength.c40.e0mean` | E_0,mean C40 | N/mm2 | 14000 | 14000 | Table 1 |  |
| `strength.c40.e90mean` | E_90,mean C40 | N/mm2 | 470 | 470 | Table 1 |  |
| `strength.c40.fc0k` | f_c,0,k C40 | N/mm2 | 27 | 27 | Table 1 |  |
| `strength.c40.fc90k` | f_c,90,k C40 | N/mm2 | 2.9 | 2.9 | Table 1 |  |
| `strength.c40.fmk` | f_m,k C40 | N/mm2 | 40 | 40 | Table 1 |  |
| `strength.c40.ft0k` | f_t,0,k C40 | N/mm2 | 26 | 26 | Table 1 |  |
| `strength.c40.ft90k` | f_t,90,k C40 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c40.fvk` | f_v,k C40 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.c40.gmean` | G_mean C40 | N/mm2 | 880 | 880 | Table 1 |  |
| `strength.c40.rhok` | ρ_k C40 | kg/m3 | 400 | 400 | Table 1 |  |
| `strength.c40.rhomean` | ρ_mean C40 | kg/m3 | 480 | 480 | Table 1 |  |
| `strength.c45.e005` | E_0,05 C45 | N/mm2 | 10000 | 10000 | Table 1 |  |
| `strength.c45.e0mean` | E_0,mean C45 | N/mm2 | 15000 | 15000 | Table 1 |  |
| `strength.c45.e90mean` | E_90,mean C45 | N/mm2 | 500 | 500 | Table 1 |  |
| `strength.c45.fc0k` | f_c,0,k C45 | N/mm2 | 29 | 29 | Table 1 |  |
| `strength.c45.fc90k` | f_c,90,k C45 | N/mm2 | 3.1 | 3.1 | Table 1 |  |
| `strength.c45.fmk` | f_m,k C45 | N/mm2 | 45 | 45 | Table 1 |  |
| `strength.c45.ft0k` | f_t,0,k C45 | N/mm2 | 30 | 30 | Table 1 |  |
| `strength.c45.ft90k` | f_t,90,k C45 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c45.fvk` | f_v,k C45 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.c45.gmean` | G_mean C45 | N/mm2 | 940 | 940 | Table 1 |  |
| `strength.c45.rhok` | ρ_k C45 | kg/m3 | 410 | 410 | Table 1 |  |
| `strength.c45.rhomean` | ρ_mean C45 | kg/m3 | 490 | 490 | Table 1 |  |
| `strength.c50.e005` | E_0,05 C50 | N/mm2 | 10700 | 10700 | Table 1 |  |
| `strength.c50.e0mean` | E_0,mean C50 | N/mm2 | 16000 | 16000 | Table 1 |  |
| `strength.c50.e90mean` | E_90,mean C50 | N/mm2 | 530 | 530 | Table 1 |  |
| `strength.c50.fc0k` | f_c,0,k C50 | N/mm2 | 30 | 30 | Table 1 |  |
| `strength.c50.fc90k` | f_c,90,k C50 | N/mm2 | 3.2 | 3.2 | Table 1 |  |
| `strength.c50.fmk` | f_m,k C50 | N/mm2 | 50 | 50 | Table 1 |  |
| `strength.c50.ft0k` | f_t,0,k C50 | N/mm2 | 33 | 33 | Table 1 |  |
| `strength.c50.ft90k` | f_t,90,k C50 | N/mm2 | 0.4 | 0.4 | Table 1 |  |
| `strength.c50.fvk` | f_v,k C50 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.c50.gmean` | G_mean C50 | N/mm2 | 1000 | 1000 | Table 1 |  |
| `strength.c50.rhok` | ρ_k C50 | kg/m3 | 430 | 430 | Table 1 |  |
| `strength.c50.rhomean` | ρ_mean C50 | kg/m3 | 520 | 520 | Table 1 |  |
| `strength.d18.e005` | E_0,05 D18 | N/mm2 | 8000 | 8000 | Table 1 |  |
| `strength.d18.e0mean` | E_0,mean D18 | N/mm2 | 9500 | 9500 | Table 1 |  |
| `strength.d18.e90mean` | E_90,mean D18 | N/mm2 | 630 | 630 | Table 1 |  |
| `strength.d18.fc0k` | f_c,0,k D18 | N/mm2 | 18 | 18 | Table 1 |  |
| `strength.d18.fc90k` | f_c,90,k D18 | N/mm2 | 7.5 | 7.5 | Table 1 |  |
| `strength.d18.fmk` | f_m,k D18 | N/mm2 | 18 | 18 | Table 1 |  |
| `strength.d18.ft0k` | f_t,0,k D18 | N/mm2 | 11 | 11 | Table 1 |  |
| `strength.d18.ft90k` | f_t,90,k D18 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d18.fvk` | f_v,k D18 | N/mm2 | 3.5 | 3.5 | Table 1 |  |
| `strength.d18.gmean` | G_mean D18 | N/mm2 | 590 | 590 | Table 1 |  |
| `strength.d18.rhok` | ρ_k D18 | kg/m3 | 475 | 475 | Table 1 |  |
| `strength.d18.rhomean` | ρ_mean D18 | kg/m3 | 570 | 570 | Table 1 |  |
| `strength.d24.e005` | E_0,05 D24 | N/mm2 | 8500 | 8500 | Table 1 |  |
| `strength.d24.e0mean` | E_0,mean D24 | N/mm2 | 10000 | 10000 | Table 1 |  |
| `strength.d24.e90mean` | E_90,mean D24 | N/mm2 | 640 | 640 | Table 1 |  |
| `strength.d24.fc0k` | f_c,0,k D24 | N/mm2 | 21 | 21 | Table 1 |  |
| `strength.d24.fc90k` | f_c,90,k D24 | N/mm2 | 7.8 | 7.8 | Table 1 |  |
| `strength.d24.fmk` | f_m,k D24 | N/mm2 | 24 | 24 | Table 1 |  |
| `strength.d24.ft0k` | f_t,0,k D24 | N/mm2 | 14 | 14 | Table 1 |  |
| `strength.d24.ft90k` | f_t,90,k D24 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d24.fvk` | f_v,k D24 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.d24.gmean` | G_mean D24 | N/mm2 | 600 | 600 | Table 1 |  |
| `strength.d24.rhok` | ρ_k D24 | kg/m3 | 485 | 485 | Table 1 |  |
| `strength.d24.rhomean` | ρ_mean D24 | kg/m3 | 580 | 580 | Table 1 |  |
| `strength.d30.e005` | E_0,05 D30 | N/mm2 | 9200 | 9200 | Table 1 |  |
| `strength.d30.e0mean` | E_0,mean D30 | N/mm2 | 11000 | 11000 | Table 1 |  |
| `strength.d30.e90mean` | E_90,mean D30 | N/mm2 | 730 | 730 | Table 1 |  |
| `strength.d30.fc0k` | f_c,0,k D30 | N/mm2 | 23 | 23 | Table 1 |  |
| `strength.d30.fc90k` | f_c,90,k D30 | N/mm2 | 8 | 8 | Table 1 |  |
| `strength.d30.fmk` | f_m,k D30 | N/mm2 | 30 | 30 | Table 1 |  |
| `strength.d30.ft0k` | f_t,0,k D30 | N/mm2 | 18 | 18 | Table 1 |  |
| `strength.d30.ft90k` | f_t,90,k D30 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d30.fvk` | f_v,k D30 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.d30.gmean` | G_mean D30 | N/mm2 | 690 | 690 | Table 1 |  |
| `strength.d30.rhok` | ρ_k D30 | kg/m3 | 530 | 530 | Table 1 |  |
| `strength.d30.rhomean` | ρ_mean D30 | kg/m3 | 640 | 640 | Table 1 |  |
| `strength.d35.e005` | E_0,05 D35 | N/mm2 | 10100 | 10100 | Table 1 |  |
| `strength.d35.e0mean` | E_0,mean D35 | N/mm2 | 12000 | 12000 | Table 1 |  |
| `strength.d35.e90mean` | E_90,mean D35 | N/mm2 | 800 | 800 | Table 1 |  |
| `strength.d35.fc0k` | f_c,0,k D35 | N/mm2 | 25 | 25 | Table 1 |  |
| `strength.d35.fc90k` | f_c,90,k D35 | N/mm2 | 8.1 | 8.1 | Table 1 |  |
| `strength.d35.fmk` | f_m,k D35 | N/mm2 | 35 | 35 | Table 1 |  |
| `strength.d35.ft0k` | f_t,0,k D35 | N/mm2 | 21 | 21 | Table 1 |  |
| `strength.d35.ft90k` | f_t,90,k D35 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d35.fvk` | f_v,k D35 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.d35.gmean` | G_mean D35 | N/mm2 | 750 | 750 | Table 1 |  |
| `strength.d35.rhok` | ρ_k D35 | kg/m3 | 540 | 540 | Table 1 |  |
| `strength.d35.rhomean` | ρ_mean D35 | kg/m3 | 650 | 650 | Table 1 |  |
| `strength.d40.e005` | E_0,05 D40 | N/mm2 | 10900 | 10900 | Table 1 |  |
| `strength.d40.e0mean` | E_0,mean D40 | N/mm2 | 13000 | 13000 | Table 1 |  |
| `strength.d40.e90mean` | E_90,mean D40 | N/mm2 | 860 | 860 | Table 1 |  |
| `strength.d40.fc0k` | f_c,0,k D40 | N/mm2 | 26 | 26 | Table 1 |  |
| `strength.d40.fc90k` | f_c,90,k D40 | N/mm2 | 8.3 | 8.3 | Table 1 |  |
| `strength.d40.fmk` | f_m,k D40 | N/mm2 | 40 | 40 | Table 1 |  |
| `strength.d40.ft0k` | f_t,0,k D40 | N/mm2 | 24 | 24 | Table 1 |  |
| `strength.d40.ft90k` | f_t,90,k D40 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d40.fvk` | f_v,k D40 | N/mm2 | 4 | 4 | Table 1 |  |
| `strength.d40.gmean` | G_mean D40 | N/mm2 | 810 | 810 | Table 1 |  |
| `strength.d40.rhok` | ρ_k D40 | kg/m3 | 550 | 550 | Table 1 |  |
| `strength.d40.rhomean` | ρ_mean D40 | kg/m3 | 660 | 660 | Table 1 |  |
| `strength.d50.e005` | E_0,05 D50 | N/mm2 | 11800 | 11800 | Table 1 |  |
| `strength.d50.e0mean` | E_0,mean D50 | N/mm2 | 14000 | 14000 | Table 1 |  |
| `strength.d50.e90mean` | E_90,mean D50 | N/mm2 | 930 | 930 | Table 1 |  |
| `strength.d50.fc0k` | f_c,0,k D50 | N/mm2 | 29 | 29 | Table 1 |  |
| `strength.d50.fc90k` | f_c,90,k D50 | N/mm2 | 9.3 | 9.3 | Table 1 |  |
| `strength.d50.fmk` | f_m,k D50 | N/mm2 | 50 | 50 | Table 1 |  |
| `strength.d50.ft0k` | f_t,0,k D50 | N/mm2 | 30 | 30 | Table 1 |  |
| `strength.d50.ft90k` | f_t,90,k D50 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d50.fvk` | f_v,k D50 | N/mm2 | 5 | 5 | Table 1 |  |
| `strength.d50.gmean` | G_mean D50 | N/mm2 | 880 | 880 | Table 1 |  |
| `strength.d50.rhok` | ρ_k D50 | kg/m3 | 620 | 620 | Table 1 |  |
| `strength.d50.rhomean` | ρ_mean D50 | kg/m3 | 740 | 740 | Table 1 |  |
| `strength.d60.e005` | E_0,05 D60 | N/mm2 | 14300 | 14300 | Table 1 |  |
| `strength.d60.e0mean` | E_0,mean D60 | N/mm2 | 17000 | 17000 | Table 1 |  |
| `strength.d60.e90mean` | E_90,mean D60 | N/mm2 | 1130 | 1130 | Table 1 |  |
| `strength.d60.fc0k` | f_c,0,k D60 | N/mm2 | 32 | 32 | Table 1 |  |
| `strength.d60.fc90k` | f_c,90,k D60 | N/mm2 | 10.5 | 10.5 | Table 1 |  |
| `strength.d60.fmk` | f_m,k D60 | N/mm2 | 60 | 60 | Table 1 |  |
| `strength.d60.ft0k` | f_t,0,k D60 | N/mm2 | 36 | 36 | Table 1 |  |
| `strength.d60.ft90k` | f_t,90,k D60 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d60.fvk` | f_v,k D60 | N/mm2 | 6 | 6 | Table 1 |  |
| `strength.d60.gmean` | G_mean D60 | N/mm2 | 1060 | 1060 | Table 1 |  |
| `strength.d60.rhok` | ρ_k D60 | kg/m3 | 700 | 700 | Table 1 |  |
| `strength.d60.rhomean` | ρ_mean D60 | kg/m3 | 840 | 840 | Table 1 |  |
| `strength.d70.e005` | E_0,05 D70 | N/mm2 | 16800 | 16800 | Table 1 |  |
| `strength.d70.e0mean` | E_0,mean D70 | N/mm2 | 20000 | 20000 | Table 1 |  |
| `strength.d70.e90mean` | E_90,mean D70 | N/mm2 | 1330 | 1330 | Table 1 |  |
| `strength.d70.fc0k` | f_c,0,k D70 | N/mm2 | 34 | 34 | Table 1 |  |
| `strength.d70.fc90k` | f_c,90,k D70 | N/mm2 | 13.5 | 13.5 | Table 1 |  |
| `strength.d70.fmk` | f_m,k D70 | N/mm2 | 70 | 70 | Table 1 |  |
| `strength.d70.ft0k` | f_t,0,k D70 | N/mm2 | 42 | 42 | Table 1 |  |
| `strength.d70.ft90k` | f_t,90,k D70 | N/mm2 | 0.6 | 0.6 | Table 1 |  |
| `strength.d70.fvk` | f_v,k D70 | N/mm2 | 6 | 6 | Table 1 |  |
| `strength.d70.gmean` | G_mean D70 | N/mm2 | 1250 | 1250 | Table 1 |  |
| `strength.d70.rhok` | ρ_k D70 | kg/m3 | 900 | 900 | Table 1 |  |
| `strength.d70.rhomean` | ρ_mean D70 | kg/m3 | 1080 | 1080 | Table 1 |  |

##### Clause Table 3.1 (75; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `kmod.clt.sc1.instant` | k_mod clt SC1 instant |  | 1.1 | 1.1 | Table 3.1 |  |
| `kmod.clt.sc1.long` | k_mod clt SC1 long |  | 0.7 | 0.7 | Table 3.1 |  |
| `kmod.clt.sc1.medium` | k_mod clt SC1 medium |  | 0.8 | 0.8 | Table 3.1 |  |
| `kmod.clt.sc1.permanent` | k_mod clt SC1 permanent |  | 0.6 | 0.6 | Table 3.1 |  |
| `kmod.clt.sc1.short` | k_mod clt SC1 short |  | 0.9 | 0.9 | Table 3.1 |  |
| `kmod.clt.sc2.instant` | k_mod clt SC2 instant |  | 1.1 | 1.1 | Table 3.1 |  |
| `kmod.clt.sc2.long` | k_mod clt SC2 long |  | 0.7 | 0.7 | Table 3.1 |  |
| `kmod.clt.sc2.medium` | k_mod clt SC2 medium |  | 0.8 | 0.8 | Table 3.1 |  |
| `kmod.clt.sc2.permanent` | k_mod clt SC2 permanent |  | 0.6 | 0.6 | Table 3.1 |  |
| `kmod.clt.sc2.short` | k_mod clt SC2 short |  | 0.9 | 0.9 | Table 3.1 |  |
| `kmod.clt.sc3.instant` | k_mod clt SC3 instant |  | 0.9 | 0.9 | Table 3.1 |  |
| `kmod.clt.sc3.long` | k_mod clt SC3 long |  | 0.55 | 0.55 | Table 3.1 |  |
| `kmod.clt.sc3.medium` | k_mod clt SC3 medium |  | 0.65 | 0.65 | Table 3.1 |  |
| `kmod.clt.sc3.permanent` | k_mod clt SC3 permanent |  | 0.5 | 0.5 | Table 3.1 |  |
| `kmod.clt.sc3.short` | k_mod clt SC3 short |  | 0.7 | 0.7 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc1.instant` | k_mod fibreboard-hb-hla2 SC1 instant |  | 1.1 | 1.1 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc1.long` | k_mod fibreboard-hb-hla2 SC1 long |  | 0.45 | 0.45 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc1.medium` | k_mod fibreboard-hb-hla2 SC1 medium |  | 0.65 | 0.65 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc1.permanent` | k_mod fibreboard-hb-hla2 SC1 permanent |  | 0.3 | 0.3 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc1.short` | k_mod fibreboard-hb-hla2 SC1 short |  | 0.85 | 0.85 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc2.instant` | k_mod fibreboard-hb-hla2 SC2 instant |  | 0.8 | 0.8 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc2.long` | k_mod fibreboard-hb-hla2 SC2 long |  | 0.3 | 0.3 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc2.medium` | k_mod fibreboard-hb-hla2 SC2 medium |  | 0.45 | 0.45 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc2.permanent` | k_mod fibreboard-hb-hla2 SC2 permanent |  | 0.2 | 0.2 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc2.short` | k_mod fibreboard-hb-hla2 SC2 short |  | 0.6 | 0.6 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc3.instant` | k_mod fibreboard-hb-hla2 SC3 instant |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc3.long` | k_mod fibreboard-hb-hla2 SC3 long |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc3.medium` | k_mod fibreboard-hb-hla2 SC3 medium |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc3.permanent` | k_mod fibreboard-hb-hla2 SC3 permanent |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-hb-hla2.sc3.short` | k_mod fibreboard-hb-hla2 SC3 short |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc1.instant` | k_mod fibreboard-mdf-hls SC1 instant |  | 1.1 | 1.1 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc1.long` | k_mod fibreboard-mdf-hls SC1 long |  | 0.4 | 0.4 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc1.medium` | k_mod fibreboard-mdf-hls SC1 medium |  | 0.6 | 0.6 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc1.permanent` | k_mod fibreboard-mdf-hls SC1 permanent |  | 0.2 | 0.2 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc1.short` | k_mod fibreboard-mdf-hls SC1 short |  | 0.8 | 0.8 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc2.instant` | k_mod fibreboard-mdf-hls SC2 instant |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc2.long` | k_mod fibreboard-mdf-hls SC2 long |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc2.medium` | k_mod fibreboard-mdf-hls SC2 medium |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc2.permanent` | k_mod fibreboard-mdf-hls SC2 permanent |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc2.short` | k_mod fibreboard-mdf-hls SC2 short |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc3.instant` | k_mod fibreboard-mdf-hls SC3 instant |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc3.long` | k_mod fibreboard-mdf-hls SC3 long |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc3.medium` | k_mod fibreboard-mdf-hls SC3 medium |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc3.permanent` | k_mod fibreboard-mdf-hls SC3 permanent |  | 0 | 0 | Table 3.1 |  |
| `kmod.fibreboard-mdf-hls.sc3.short` | k_mod fibreboard-mdf-hls SC3 short |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p5.sc1.instant` | k_mod particleboard-p5 SC1 instant |  | 1.1 | 1.1 | Table 3.1 |  |
| `kmod.particleboard-p5.sc1.long` | k_mod particleboard-p5 SC1 long |  | 0.45 | 0.45 | Table 3.1 |  |
| `kmod.particleboard-p5.sc1.medium` | k_mod particleboard-p5 SC1 medium |  | 0.65 | 0.65 | Table 3.1 |  |
| `kmod.particleboard-p5.sc1.permanent` | k_mod particleboard-p5 SC1 permanent |  | 0.3 | 0.3 | Table 3.1 |  |
| `kmod.particleboard-p5.sc1.short` | k_mod particleboard-p5 SC1 short |  | 0.85 | 0.85 | Table 3.1 |  |
| `kmod.particleboard-p5.sc2.instant` | k_mod particleboard-p5 SC2 instant |  | 0.8 | 0.8 | Table 3.1 |  |
| `kmod.particleboard-p5.sc2.long` | k_mod particleboard-p5 SC2 long |  | 0.3 | 0.3 | Table 3.1 |  |
| `kmod.particleboard-p5.sc2.medium` | k_mod particleboard-p5 SC2 medium |  | 0.45 | 0.45 | Table 3.1 |  |
| `kmod.particleboard-p5.sc2.permanent` | k_mod particleboard-p5 SC2 permanent |  | 0.2 | 0.2 | Table 3.1 |  |
| `kmod.particleboard-p5.sc2.short` | k_mod particleboard-p5 SC2 short |  | 0.6 | 0.6 | Table 3.1 |  |
| `kmod.particleboard-p5.sc3.instant` | k_mod particleboard-p5 SC3 instant |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p5.sc3.long` | k_mod particleboard-p5 SC3 long |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p5.sc3.medium` | k_mod particleboard-p5 SC3 medium |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p5.sc3.permanent` | k_mod particleboard-p5 SC3 permanent |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p5.sc3.short` | k_mod particleboard-p5 SC3 short |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p7.sc1.instant` | k_mod particleboard-p7 SC1 instant |  | 1.1 | 1.1 | Table 3.1 |  |
| `kmod.particleboard-p7.sc1.long` | k_mod particleboard-p7 SC1 long |  | 0.45 | 0.45 | Table 3.1 |  |
| `kmod.particleboard-p7.sc1.medium` | k_mod particleboard-p7 SC1 medium |  | 0.65 | 0.65 | Table 3.1 |  |
| `kmod.particleboard-p7.sc1.permanent` | k_mod particleboard-p7 SC1 permanent |  | 0.3 | 0.3 | Table 3.1 |  |
| `kmod.particleboard-p7.sc1.short` | k_mod particleboard-p7 SC1 short |  | 0.85 | 0.85 | Table 3.1 |  |
| `kmod.particleboard-p7.sc2.instant` | k_mod particleboard-p7 SC2 instant |  | 0.8 | 0.8 | Table 3.1 |  |
| `kmod.particleboard-p7.sc2.long` | k_mod particleboard-p7 SC2 long |  | 0.3 | 0.3 | Table 3.1 |  |
| `kmod.particleboard-p7.sc2.medium` | k_mod particleboard-p7 SC2 medium |  | 0.45 | 0.45 | Table 3.1 |  |
| `kmod.particleboard-p7.sc2.permanent` | k_mod particleboard-p7 SC2 permanent |  | 0.2 | 0.2 | Table 3.1 |  |
| `kmod.particleboard-p7.sc2.short` | k_mod particleboard-p7 SC2 short |  | 0.6 | 0.6 | Table 3.1 |  |
| `kmod.particleboard-p7.sc3.instant` | k_mod particleboard-p7 SC3 instant |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p7.sc3.long` | k_mod particleboard-p7 SC3 long |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p7.sc3.medium` | k_mod particleboard-p7 SC3 medium |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p7.sc3.permanent` | k_mod particleboard-p7 SC3 permanent |  | 0 | 0 | Table 3.1 |  |
| `kmod.particleboard-p7.sc3.short` | k_mod particleboard-p7 SC3 short |  | 0 | 0 | Table 3.1 |  |

##### Clause Table 3.2 (21; DIN EN 1995-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `kdef.clt.sc1` | k_def clt SC1 |  | 0.6 | 0.6 | Table 3.2 |  |
| `kdef.clt.sc2` | k_def clt SC2 |  | 0.8 | 0.8 | Table 3.2 |  |
| `kdef.clt.sc3` | k_def clt SC3 |  | 2 | 2 | Table 3.2 |  |
| `kdef.fibreboard-hb-hla2.sc1` | k_def fibreboard-hb-hla2 SC1 |  | 2.25 | 2.25 | Table 3.2 |  |
| `kdef.fibreboard-hb-hla2.sc2` | k_def fibreboard-hb-hla2 SC2 |  | 3 | 3 | Table 3.2 |  |
| `kdef.fibreboard-hb-hla2.sc3` | k_def fibreboard-hb-hla2 SC3 |  | 0 | 0 | Table 3.2 |  |
| `kdef.fibreboard-mdf-hls.sc1` | k_def fibreboard-mdf-hls SC1 |  | 3 | 3 | Table 3.2 |  |
| `kdef.fibreboard-mdf-hls.sc2` | k_def fibreboard-mdf-hls SC2 |  | 4 | 4 | Table 3.2 |  |
| `kdef.fibreboard-mdf-hls.sc3` | k_def fibreboard-mdf-hls SC3 |  | 0 | 0 | Table 3.2 |  |
| `kdef.particleboard-p4.sc1` | k_def particleboard-p4 SC1 |  | 2.25 | 2.25 | Table 3.2 |  |
| `kdef.particleboard-p4.sc2` | k_def particleboard-p4 SC2 |  | 3 | 3 | Table 3.2 |  |
| `kdef.particleboard-p4.sc3` | k_def particleboard-p4 SC3 |  | 0 | 0 | Table 3.2 |  |
| `kdef.particleboard-p5.sc1` | k_def particleboard-p5 SC1 |  | 2.25 | 2.25 | Table 3.2 |  |
| `kdef.particleboard-p5.sc2` | k_def particleboard-p5 SC2 |  | 3 | 3 | Table 3.2 |  |
| `kdef.particleboard-p5.sc3` | k_def particleboard-p5 SC3 |  | 0 | 0 | Table 3.2 |  |
| `kdef.particleboard-p6.sc1` | k_def particleboard-p6 SC1 |  | 1.5 | 1.5 | Table 3.2 |  |
| `kdef.particleboard-p6.sc2` | k_def particleboard-p6 SC2 |  | 2.25 | 2.25 | Table 3.2 |  |
| `kdef.particleboard-p6.sc3` | k_def particleboard-p6 SC3 |  | 0 | 0 | Table 3.2 |  |
| `kdef.particleboard-p7.sc1` | k_def particleboard-p7 SC1 |  | 1.5 | 1.5 | Table 3.2 |  |
| `kdef.particleboard-p7.sc2` | k_def particleboard-p7 SC2 |  | 2.25 | 2.25 | Table 3.2 |  |
| `kdef.particleboard-p7.sc3` | k_def particleboard-p7 SC3 |  | 0 | 0 | Table 3.2 |  |

##### Clause Table 5 (132; EN 14080:2013 Table 5)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `strength.gl20c.e005` | E_0,05 GL20c | N/mm2 | 8600 | 8600 | Table 5 |  |
| `strength.gl20c.e0mean` | E_0,mean GL20c | N/mm2 | 10400 | 10400 | Table 5 |  |
| `strength.gl20c.e90mean` | E_90,mean GL20c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl20c.fc0k` | f_c,0,k GL20c | N/mm2 | 15.5 | 15.5 | Table 5 |  |
| `strength.gl20c.fc90k` | f_c,90,k GL20c | N/mm2 | 2.4 | 2.4 | Table 5 |  |
| `strength.gl20c.fmk` | f_m,k GL20c | N/mm2 | 20 | 20 | Table 5 |  |
| `strength.gl20c.ft0k` | f_t,0,k GL20c | N/mm2 | 11.5 | 11.5 | Table 5 |  |
| `strength.gl20c.ft90k` | f_t,90,k GL20c | N/mm2 | 0.35 | 0.35 | Table 5 |  |
| `strength.gl20c.fvk` | f_v,k GL20c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl20c.gmean` | G_mean GL20c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl20c.rhok` | ρ_k GL20c | kg/m3 | 335 | 335 | Table 5 |  |
| `strength.gl20c.rhomean` | ρ_mean GL20c | kg/m3 | 370 | 370 | Table 5 |  |
| `strength.gl20h.e005` | E_0,05 GL20h | N/mm2 | 7000 | 7000 | Table 5 |  |
| `strength.gl20h.e0mean` | E_0,mean GL20h | N/mm2 | 8400 | 8400 | Table 5 |  |
| `strength.gl20h.e90mean` | E_90,mean GL20h | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl20h.fc0k` | f_c,0,k GL20h | N/mm2 | 20 | 20 | Table 5 |  |
| `strength.gl20h.fc90k` | f_c,90,k GL20h | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl20h.fmk` | f_m,k GL20h | N/mm2 | 20 | 20 | Table 5 |  |
| `strength.gl20h.ft0k` | f_t,0,k GL20h | N/mm2 | 16 | 16 | Table 5 |  |
| `strength.gl20h.ft90k` | f_t,90,k GL20h | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl20h.fvk` | f_v,k GL20h | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl20h.gmean` | G_mean GL20h | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl20h.rhok` | ρ_k GL20h | kg/m3 | 340 | 340 | Table 5 |  |
| `strength.gl20h.rhomean` | ρ_mean GL20h | kg/m3 | 370 | 370 | Table 5 |  |
| `strength.gl22c.e005` | E_0,05 GL22c | N/mm2 | 8600 | 8600 | Table 5 |  |
| `strength.gl22c.e0mean` | E_0,mean GL22c | N/mm2 | 10400 | 10400 | Table 5 |  |
| `strength.gl22c.e90mean` | E_90,mean GL22c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl22c.fc0k` | f_c,0,k GL22c | N/mm2 | 18.5 | 18.5 | Table 5 |  |
| `strength.gl22c.fc90k` | f_c,90,k GL22c | N/mm2 | 2.4 | 2.4 | Table 5 |  |
| `strength.gl22c.fmk` | f_m,k GL22c | N/mm2 | 22 | 22 | Table 5 |  |
| `strength.gl22c.ft0k` | f_t,0,k GL22c | N/mm2 | 13 | 13 | Table 5 |  |
| `strength.gl22c.ft90k` | f_t,90,k GL22c | N/mm2 | 0.35 | 0.35 | Table 5 |  |
| `strength.gl22c.fvk` | f_v,k GL22c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl22c.gmean` | G_mean GL22c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl22c.rhok` | ρ_k GL22c | kg/m3 | 355 | 355 | Table 5 |  |
| `strength.gl22c.rhomean` | ρ_mean GL22c | kg/m3 | 390 | 390 | Table 5 |  |
| `strength.gl22h.e005` | E_0,05 GL22h | N/mm2 | 8800 | 8800 | Table 5 |  |
| `strength.gl22h.e0mean` | E_0,mean GL22h | N/mm2 | 10500 | 10500 | Table 5 |  |
| `strength.gl22h.e90mean` | E_90,mean GL22h | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl22h.fc0k` | f_c,0,k GL22h | N/mm2 | 22 | 22 | Table 5 |  |
| `strength.gl22h.fc90k` | f_c,90,k GL22h | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl22h.fmk` | f_m,k GL22h | N/mm2 | 22 | 22 | Table 5 |  |
| `strength.gl22h.ft0k` | f_t,0,k GL22h | N/mm2 | 17.6 | 17.6 | Table 5 |  |
| `strength.gl22h.ft90k` | f_t,90,k GL22h | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl22h.fvk` | f_v,k GL22h | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl22h.gmean` | G_mean GL22h | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl22h.rhok` | ρ_k GL22h | kg/m3 | 370 | 370 | Table 5 |  |
| `strength.gl22h.rhomean` | ρ_mean GL22h | kg/m3 | 410 | 410 | Table 5 |  |
| `strength.gl24c.e005` | E_0,05 GL24c | N/mm2 | 9100 | 9100 | Table 5 |  |
| `strength.gl24c.e0mean` | E_0,mean GL24c | N/mm2 | 11000 | 11000 | Table 5 |  |
| `strength.gl24c.e90mean` | E_90,mean GL24c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl24c.fc0k` | f_c,0,k GL24c | N/mm2 | 21.5 | 21.5 | Table 5 |  |
| `strength.gl24c.fc90k` | f_c,90,k GL24c | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl24c.fmk` | f_m,k GL24c | N/mm2 | 24 | 24 | Table 5 |  |
| `strength.gl24c.ft0k` | f_t,0,k GL24c | N/mm2 | 17 | 17 | Table 5 |  |
| `strength.gl24c.ft90k` | f_t,90,k GL24c | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl24c.fvk` | f_v,k GL24c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl24c.gmean` | G_mean GL24c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl24c.rhok` | ρ_k GL24c | kg/m3 | 350 | 350 | Table 5 |  |
| `strength.gl24c.rhomean` | ρ_mean GL24c | kg/m3 | 385 | 385 | Table 5 |  |
| `strength.gl26c.e005` | E_0,05 GL26c | N/mm2 | 9600 | 9600 | Table 5 |  |
| `strength.gl26c.e0mean` | E_0,mean GL26c | N/mm2 | 11500 | 11500 | Table 5 |  |
| `strength.gl26c.e90mean` | E_90,mean GL26c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl26c.fc0k` | f_c,0,k GL26c | N/mm2 | 23 | 23 | Table 5 |  |
| `strength.gl26c.fc90k` | f_c,90,k GL26c | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl26c.fmk` | f_m,k GL26c | N/mm2 | 26 | 26 | Table 5 |  |
| `strength.gl26c.ft0k` | f_t,0,k GL26c | N/mm2 | 19 | 19 | Table 5 |  |
| `strength.gl26c.ft90k` | f_t,90,k GL26c | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl26c.fvk` | f_v,k GL26c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl26c.gmean` | G_mean GL26c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl26c.rhok` | ρ_k GL26c | kg/m3 | 385 | 385 | Table 5 |  |
| `strength.gl26c.rhomean` | ρ_mean GL26c | kg/m3 | 420 | 420 | Table 5 |  |
| `strength.gl26h.e005` | E_0,05 GL26h | N/mm2 | 10100 | 10100 | Table 5 |  |
| `strength.gl26h.e0mean` | E_0,mean GL26h | N/mm2 | 12100 | 12100 | Table 5 |  |
| `strength.gl26h.e90mean` | E_90,mean GL26h | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl26h.fc0k` | f_c,0,k GL26h | N/mm2 | 26 | 26 | Table 5 |  |
| `strength.gl26h.fc90k` | f_c,90,k GL26h | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl26h.fmk` | f_m,k GL26h | N/mm2 | 26 | 26 | Table 5 |  |
| `strength.gl26h.ft0k` | f_t,0,k GL26h | N/mm2 | 20.8 | 20.8 | Table 5 |  |
| `strength.gl26h.ft90k` | f_t,90,k GL26h | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl26h.fvk` | f_v,k GL26h | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl26h.gmean` | G_mean GL26h | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl26h.rhok` | ρ_k GL26h | kg/m3 | 405 | 405 | Table 5 |  |
| `strength.gl26h.rhomean` | ρ_mean GL26h | kg/m3 | 445 | 445 | Table 5 |  |
| `strength.gl28c.e005` | E_0,05 GL28c | N/mm2 | 10400 | 10400 | Table 5 |  |
| `strength.gl28c.e0mean` | E_0,mean GL28c | N/mm2 | 12500 | 12500 | Table 5 |  |
| `strength.gl28c.e90mean` | E_90,mean GL28c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl28c.fc0k` | f_c,0,k GL28c | N/mm2 | 24 | 24 | Table 5 |  |
| `strength.gl28c.fc90k` | f_c,90,k GL28c | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl28c.fmk` | f_m,k GL28c | N/mm2 | 28 | 28 | Table 5 |  |
| `strength.gl28c.ft0k` | f_t,0,k GL28c | N/mm2 | 19.5 | 19.5 | Table 5 |  |
| `strength.gl28c.ft90k` | f_t,90,k GL28c | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl28c.fvk` | f_v,k GL28c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl28c.gmean` | G_mean GL28c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl28c.rhok` | ρ_k GL28c | kg/m3 | 390 | 390 | Table 5 |  |
| `strength.gl28c.rhomean` | ρ_mean GL28c | kg/m3 | 425 | 425 | Table 5 |  |
| `strength.gl30c.e005` | E_0,05 GL30c | N/mm2 | 10800 | 10800 | Table 5 |  |
| `strength.gl30c.e0mean` | E_0,mean GL30c | N/mm2 | 13000 | 13000 | Table 5 |  |
| `strength.gl30c.e90mean` | E_90,mean GL30c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl30c.fc0k` | f_c,0,k GL30c | N/mm2 | 24.5 | 24.5 | Table 5 |  |
| `strength.gl30c.fc90k` | f_c,90,k GL30c | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl30c.fmk` | f_m,k GL30c | N/mm2 | 30 | 30 | Table 5 |  |
| `strength.gl30c.ft0k` | f_t,0,k GL30c | N/mm2 | 19.5 | 19.5 | Table 5 |  |
| `strength.gl30c.ft90k` | f_t,90,k GL30c | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl30c.fvk` | f_v,k GL30c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl30c.gmean` | G_mean GL30c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl30c.rhok` | ρ_k GL30c | kg/m3 | 390 | 390 | Table 5 |  |
| `strength.gl30c.rhomean` | ρ_mean GL30c | kg/m3 | 430 | 430 | Table 5 |  |
| `strength.gl30h.e005` | E_0,05 GL30h | N/mm2 | 11300 | 11300 | Table 5 |  |
| `strength.gl30h.e0mean` | E_0,mean GL30h | N/mm2 | 13600 | 13600 | Table 5 |  |
| `strength.gl30h.e90mean` | E_90,mean GL30h | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl30h.fc0k` | f_c,0,k GL30h | N/mm2 | 30 | 30 | Table 5 |  |
| `strength.gl30h.fc90k` | f_c,90,k GL30h | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl30h.fmk` | f_m,k GL30h | N/mm2 | 30 | 30 | Table 5 |  |
| `strength.gl30h.ft0k` | f_t,0,k GL30h | N/mm2 | 24 | 24 | Table 5 |  |
| `strength.gl30h.ft90k` | f_t,90,k GL30h | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl30h.fvk` | f_v,k GL30h | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl30h.gmean` | G_mean GL30h | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl30h.rhok` | ρ_k GL30h | kg/m3 | 430 | 430 | Table 5 |  |
| `strength.gl30h.rhomean` | ρ_mean GL30h | kg/m3 | 480 | 480 | Table 5 |  |
| `strength.gl32c.e005` | E_0,05 GL32c | N/mm2 | 11200 | 11200 | Table 5 |  |
| `strength.gl32c.e0mean` | E_0,mean GL32c | N/mm2 | 13500 | 13500 | Table 5 |  |
| `strength.gl32c.e90mean` | E_90,mean GL32c | N/mm2 | 300 | 300 | Table 5 |  |
| `strength.gl32c.fc0k` | f_c,0,k GL32c | N/mm2 | 24.5 | 24.5 | Table 5 |  |
| `strength.gl32c.fc90k` | f_c,90,k GL32c | N/mm2 | 2.5 | 2.5 | Table 5 |  |
| `strength.gl32c.fmk` | f_m,k GL32c | N/mm2 | 32 | 32 | Table 5 |  |
| `strength.gl32c.ft0k` | f_t,0,k GL32c | N/mm2 | 19.5 | 19.5 | Table 5 |  |
| `strength.gl32c.ft90k` | f_t,90,k GL32c | N/mm2 | 0.5 | 0.5 | Table 5 |  |
| `strength.gl32c.fvk` | f_v,k GL32c | N/mm2 | 3.5 | 3.5 | Table 5 |  |
| `strength.gl32c.gmean` | G_mean GL32c | N/mm2 | 650 | 650 | Table 5 |  |
| `strength.gl32c.rhok` | ρ_k GL32c | kg/m3 | 400 | 400 | Table 5 |  |
| `strength.gl32c.rhomean` | ρ_mean GL32c | kg/m3 | 440 | 440 | Table 5 |  |

##### Clause Table 7.2 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-61.1` | winst,Q ≤ L/300 |  | - | - | Table 7.2 | en1995-en1996-target.md:216: NA-DE: `winst,Q ≤ L/300`, `wnet,fin ≤ L/200` (general), `wfin ≤ L/200`..., for roof members with small usage `L/200` **** |

##### Clause Table 8.8 (1; DIN EN 1995/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t-80.1` | ⇒ |  | - | - | Table 8.8 | en1995-en1996-target.md:315: Spacing per Table 8.8 as data ****. |

##### Clause material (24; EN 14374 / ETA, EN 16351 lamellae C24)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `strength.clt100.e005` | E_0,05 CLT100 | N/mm2 | 7400 | 7400 | material |  |
| `strength.clt100.e0mean` | E_0,mean CLT100 | N/mm2 | 11000 | 11000 | material |  |
| `strength.clt100.e90mean` | E_90,mean CLT100 | N/mm2 | 370 | 370 | material |  |
| `strength.clt100.fc0k` | f_c,0,k CLT100 | N/mm2 | 21 | 21 | material |  |
| `strength.clt100.fc90k` | f_c,90,k CLT100 | N/mm2 | 2.5 | 2.5 | material |  |
| `strength.clt100.fmk` | f_m,k CLT100 | N/mm2 | 24 | 24 | material |  |
| `strength.clt100.ft0k` | f_t,0,k CLT100 | N/mm2 | 14.5 | 14.5 | material |  |
| `strength.clt100.ft90k` | f_t,90,k CLT100 | N/mm2 | 0.4 | 0.4 | material |  |
| `strength.clt100.fvk` | f_v,k CLT100 | N/mm2 | 4 | 4 | material |  |
| `strength.clt100.gmean` | G_mean CLT100 | N/mm2 | 690 | 690 | material |  |
| `strength.clt100.rhok` | ρ_k CLT100 | kg/m3 | 350 | 350 | material |  |
| `strength.clt100.rhomean` | ρ_mean CLT100 | kg/m3 | 420 | 420 | material |  |
| `strength.lvl32.e005` | E_0,05 LVL32 | N/mm2 | 11600 | 11600 | material |  |
| `strength.lvl32.e0mean` | E_0,mean LVL32 | N/mm2 | 14000 | 14000 | material |  |
| `strength.lvl32.e90mean` | E_90,mean LVL32 | N/mm2 | 500 | 500 | material |  |
| `strength.lvl32.fc0k` | f_c,0,k LVL32 | N/mm2 | 35 | 35 | material |  |
| `strength.lvl32.fc90k` | f_c,90,k LVL32 | N/mm2 | 6 | 6 | material |  |
| `strength.lvl32.fmk` | f_m,k LVL32 | N/mm2 | 36 | 36 | material |  |
| `strength.lvl32.ft0k` | f_t,0,k LVL32 | N/mm2 | 26 | 26 | material |  |
| `strength.lvl32.ft90k` | f_t,90,k LVL32 | N/mm2 | 0.9 | 0.9 | material |  |
| `strength.lvl32.fvk` | f_v,k LVL32 | N/mm2 | 4.5 | 4.5 | material |  |
| `strength.lvl32.gmean` | G_mean LVL32 | N/mm2 | 600 | 600 | material |  |
| `strength.lvl32.rhok` | ρ_k LVL32 | kg/m3 | 510 | 510 | material |  |
| `strength.lvl32.rhomean` | ρ_mean LVL32 | kg/m3 | 540 | 540 | material |  |

### EN 1996 (246)

#### EN 1996, part 1 (99)

##### Clause 3.6.1.2 (6; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `alpha.gp` | alpha general purpose |  | 0.7 | 0.65 | 3.6.1.2 |  |
| `alpha.lw` | alpha lightweight |  | 0.7 | 0.65 | 3.6.1.2 |  |
| `alpha.tl` | alpha thin layer |  | 0.85 | 0.85 | 3.6.1.2 |  |
| `beta.gp` | beta general purpose |  | 0.3 | 0.25 | 3.6.1.2 |  |
| `beta.lw` | beta lightweight |  | 0.3 | 0.25 | 3.6.1.2 |  |
| `beta.tl` | beta thin layer |  | 0 | 0 | 3.6.1.2 |  |

##### Clause 3.6.1.2 Table 3.3 / NA (48; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k.gp.aac.g1` | K general purpose aac g1 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.aac.g2` | K general purpose aac g2 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.aac.g3` | K general purpose aac g3 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.aac.g4` | K general purpose aac g4 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.casi.g1` | K general purpose casi g1 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.casi.g2` | K general purpose casi g2 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.casi.g3` | K general purpose casi g3 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.casi.g4` | K general purpose casi g4 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.clay.g1` | K general purpose clay g1 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.clay.g2` | K general purpose clay g2 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.clay.g3` | K general purpose clay g3 |  | 0.35 | 0.45 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.clay.g4` | K general purpose clay g4 |  | 0.35 | 0.4 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.concrete.g1` | K general purpose concrete g1 |  | 0.55 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.concrete.g2` | K general purpose concrete g2 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.concrete.g3` | K general purpose concrete g3 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.gp.concrete.g4` | K general purpose concrete g4 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.aac.g1` | K lightweight aac g1 |  | 0.45 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.aac.g2` | K lightweight aac g2 |  | 0.45 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.aac.g3` | K lightweight aac g3 |  | 0.45 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.aac.g4` | K lightweight aac g4 |  | 0.45 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.casi.g1` | K lightweight casi g1 |  | 0.45 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.casi.g2` | K lightweight casi g2 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.casi.g3` | K lightweight casi g3 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.casi.g4` | K lightweight casi g4 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.clay.g1` | K lightweight clay g1 |  | 0.3 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.clay.g2` | K lightweight clay g2 |  | 0.3 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.clay.g3` | K lightweight clay g3 |  | 0.25 | 0.45 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.clay.g4` | K lightweight clay g4 |  | 0.25 | 0.4 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.concrete.g1` | K lightweight concrete g1 |  | 0.45 | 0.6 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.concrete.g2` | K lightweight concrete g2 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.concrete.g3` | K lightweight concrete g3 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.lw.concrete.g4` | K lightweight concrete g4 |  | 0.45 | 0.5 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.aac.g1` | K thin layer aac g1 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.aac.g2` | K thin layer aac g2 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.aac.g3` | K thin layer aac g3 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.aac.g4` | K thin layer aac g4 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.casi.g1` | K thin layer casi g1 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.casi.g2` | K thin layer casi g2 |  | 0.65 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.casi.g3` | K thin layer casi g3 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.casi.g4` | K thin layer casi g4 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.clay.g1` | K thin layer clay g1 |  | 0.75 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.clay.g2` | K thin layer clay g2 |  | 0.7 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.clay.g3` | K thin layer clay g3 |  | 0.5 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.clay.g4` | K thin layer clay g4 |  | 0.35 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.concrete.g1` | K thin layer concrete g1 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.concrete.g2` | K thin layer concrete g2 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.concrete.g3` | K thin layer concrete g3 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |
| `k.tl.concrete.g4` | K thin layer concrete g4 |  | 0.8 | 0.7 | 3.6.1.2 Table 3.3 / NA |  |

##### Clause 3.6.2 Table 3.4 / NA (28; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fvk0.gp.aac.ge10` | f_vk0 general purpose aac ge10 | MPa | 0.2 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.aac.ge15` | f_vk0 general purpose aac ge15 | MPa | 0.2 | 0.26 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.aac.ge2-5` | f_vk0 general purpose aac ge2-5 | MPa | 0.15 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.aac.ge5` | f_vk0 general purpose aac ge5 | MPa | 0.15 | 0.18 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.aac.lt2-5` | f_vk0 general purpose aac lt2-5 | MPa | 0.1 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.casi.ge10` | f_vk0 general purpose casi ge10 | MPa | 0.2 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.casi.ge15` | f_vk0 general purpose casi ge15 | MPa | 0.2 | 0.26 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.casi.ge2-5` | f_vk0 general purpose casi ge2-5 | MPa | 0.15 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.casi.ge5` | f_vk0 general purpose casi ge5 | MPa | 0.15 | 0.18 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.casi.lt2-5` | f_vk0 general purpose casi lt2-5 | MPa | 0.1 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.clay.ge10` | f_vk0 general purpose clay ge10 | MPa | 0.3 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.clay.ge15` | f_vk0 general purpose clay ge15 | MPa | 0.3 | 0.26 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.clay.ge2-5` | f_vk0 general purpose clay ge2-5 | MPa | 0.2 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.clay.ge5` | f_vk0 general purpose clay ge5 | MPa | 0.2 | 0.18 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.clay.lt2-5` | f_vk0 general purpose clay lt2-5 | MPa | 0.1 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.concrete.ge10` | f_vk0 general purpose concrete ge10 | MPa | 0.2 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.concrete.ge15` | f_vk0 general purpose concrete ge15 | MPa | 0.2 | 0.26 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.concrete.ge2-5` | f_vk0 general purpose concrete ge2-5 | MPa | 0.15 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.concrete.ge5` | f_vk0 general purpose concrete ge5 | MPa | 0.15 | 0.18 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.gp.concrete.lt2-5` | f_vk0 general purpose concrete lt2-5 | MPa | 0.1 | 0.08 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.lw.aac.all` | f_vk0 lightweight aac all | MPa | 0.15 | 0.15 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.lw.casi.all` | f_vk0 lightweight casi all | MPa | 0.15 | 0.15 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.lw.clay.all` | f_vk0 lightweight clay all | MPa | 0.15 | 0.15 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.lw.concrete.all` | f_vk0 lightweight concrete all | MPa | 0.15 | 0.15 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.tl.aac.all` | f_vk0 thin layer aac all | MPa | 0.3 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.tl.casi.all` | f_vk0 thin layer casi all | MPa | 0.4 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.tl.clay.all` | f_vk0 thin layer clay all | MPa | 0.3 | 0.22 | 3.6.2 Table 3.4 / NA |  |
| `fvk0.tl.concrete.all` | f_vk0 thin layer concrete all | MPa | 0.3 | 0.22 | 3.6.2 Table 3.4 / NA |  |

##### Clause 3.6.3 Table 3.6 (16; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fxk.x1.aac.strong` | f_xk1 aac f_m >= 5 | MPa | 0.1 | 0.1 | 3.6.3 Table 3.6 |  |
| `fxk.x1.aac.weak` | f_xk1 aac f_m < 5 | MPa | 0.05 | 0.05 | 3.6.3 Table 3.6 |  |
| `fxk.x1.casi.strong` | f_xk1 casi f_m >= 5 | MPa | 0.1 | 0.1 | 3.6.3 Table 3.6 |  |
| `fxk.x1.casi.weak` | f_xk1 casi f_m < 5 | MPa | 0.05 | 0.05 | 3.6.3 Table 3.6 |  |
| `fxk.x1.clay.strong` | f_xk1 clay f_m >= 5 | MPa | 0.1 | 0.1 | 3.6.3 Table 3.6 |  |
| `fxk.x1.clay.weak` | f_xk1 clay f_m < 5 | MPa | 0.1 | 0.1 | 3.6.3 Table 3.6 |  |
| `fxk.x1.concrete.strong` | f_xk1 concrete f_m >= 5 | MPa | 0.1 | 0.1 | 3.6.3 Table 3.6 |  |
| `fxk.x1.concrete.weak` | f_xk1 concrete f_m < 5 | MPa | 0.05 | 0.05 | 3.6.3 Table 3.6 |  |
| `fxk.x2.aac.strong` | f_xk2 aac f_m >= 5 | MPa | 0.2 | 0.2 | 3.6.3 Table 3.6 |  |
| `fxk.x2.aac.weak` | f_xk2 aac f_m < 5 | MPa | 0.2 | 0.2 | 3.6.3 Table 3.6 |  |
| `fxk.x2.casi.strong` | f_xk2 casi f_m >= 5 | MPa | 0.4 | 0.4 | 3.6.3 Table 3.6 |  |
| `fxk.x2.casi.weak` | f_xk2 casi f_m < 5 | MPa | 0.2 | 0.2 | 3.6.3 Table 3.6 |  |
| `fxk.x2.clay.strong` | f_xk2 clay f_m >= 5 | MPa | 0.4 | 0.4 | 3.6.3 Table 3.6 |  |
| `fxk.x2.clay.weak` | f_xk2 clay f_m < 5 | MPa | 0.2 | 0.2 | 3.6.3 Table 3.6 |  |
| `fxk.x2.concrete.strong` | f_xk2 concrete f_m >= 5 | MPa | 0.4 | 0.4 | 3.6.3 Table 3.6 |  |
| `fxk.x2.concrete.weak` | f_xk2 concrete f_m < 5 | MPa | 0.2 | 0.2 | 3.6.3 Table 3.6 |  |

##### Clause 3.7.2 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `k-e` | K_E |  | 1000 | 950 | 3.7.2 |  |

#### EN 1996, part 1-1 (91)

##### Clause 2.5 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-50.1` | n ≥ 7 |  | - | - | 2.5 | en1995-en1996-target.md:648: NA-DE: DIN EN 1996-1-1/NA 8.5.2.3 Table NA.13: ties per m² = 7 (cavity 120 mm?)`n ≥ 7` in general, `n ≥ 5` for walls wher |

##### Clause 3.6.1.2 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-02.4` | HLz 12 NM IIa |  | - | - | 3.6.1.2 | en1995-en1996-target.md:506: Example row (to be pinned from the NA): `HLz 12 NM IIa` ⇒ ~ 4.4 N/mm² **** — do not hardcode |

##### Clause 6.2 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-20.1` | bef |  | - | - | 6.2 | en1995-en1996-target.md:592: `bef` per side `≤ min( ½ clear distance between webs, 6 tf + tw, 0.2 Htot)`) **** |

##### Clause 6.3.2 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-31.1` | α |  | - | - | 6.3.2 | en1995-en1996-target.md:608: `α` coefficients from the EN 1996-1-1 Annex E tables for boundary conditions A–J (four edges pinned, three edges with fre |

##### Clause 6.17 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-32.1` | qlat,d = fd t²/ la² |  | - | - | 6.17 | en1995-en1996-target.md:615: `qlat,d = fd t²/ la²``qlat,d = 2? fd (t/ℓa)²`** as **** — the explicit coefficient, the condition `ℓ_a/t ≤ 25?` and the t |

##### Clause 7.3.2 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-61.1` | l/d |  | - | - | 7.3.2 | en1995-en1996-target.md:664: Reinforced masonry deflection `l/d` ratios (7.3.2: `(l/d)max` = 20 simply supported, 7 cantilever, 26 continuous (for `fy |

##### Clause ? (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-52.1` | claims_simple_masonry |  | - | - | SEI-52 Timber and masonry (EN 1998-1 §8, §9) | en1997-en1998-target.md:499: - Masonry: simple masonry rule when `claims_simple_masonry` and wall area ratio per direction `Σ A_w / A_floor >= 0.02 .. |

##### Clause EN 1990 Table A1.1 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi` | psi |  | - | - | EN 1990 Table A1.1 |  |

##### Clause M-01 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m01.delta` | m01.delta |  | - | - | M-01 |  |

##### Clause M-02 (3; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m02.caps` | m02.caps |  | - | - | M-02 |  |
| `m02.k-de` | m02.k-de |  | - | - | M-02 |  |
| `m02.k-en` | m02.k-en |  | - | - | M-02 |  |

##### Clause M-03 (4; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m03.fvk-cap-en` | m03.fvk-cap-en |  | - | - | M-03 |  |
| `m03.fvk0` | m03.fvk0 |  | - | - | M-03 |  |
| `m03.fvlt-de` | m03.fvlt-de |  | - | - | M-03 |  |
| `m03.head-joint` | m03.head-joint |  | - | - | M-03 |  |

##### Clause M-04 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m04.fxk` | m04.fxk |  | - | - | M-04 |  |

##### Clause M-05 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m05.fbok` | m05.fbok |  | - | - | M-05 |  |

##### Clause M-06 (2; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m06.creep` | m06.creep |  | - | - | M-06 |  |
| `m06.e` | m06.e |  | - | - | M-06 |  |

##### Clause M-07 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m07.gamma` | m07.gamma |  | - | - | M-07 |  |

##### Clause M-10 (2; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m10.rho2` | m10.rho2 |  | - | - | M-10 |  |
| `m10.stiffener` | m10.stiffener |  | - | - | M-10 |  |

##### Clause M-11 (2; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m11.beta-floor` | m11.beta-floor |  | - | - | M-11 |  |
| `m11.limit` | m11.limit |  | - | - | M-11 |  |

##### Clause M-12 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m12.einit` | m12.einit |  | - | - | M-12 |  |

##### Clause M-13 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m13.phi1` | m13.phi1 |  | - | - | M-13 |  |

##### Clause M-14 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m14.beta` | m14.beta |  | - | - | M-14 |  |

##### Clause M-15 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m15.lintel` | m15.lintel |  | - | - | M-15 |  |

##### Clause M-20 (2; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m20.flange` | m20.flange |  | - | - | M-20 |  |
| `m20.stability` | m20.stability |  | - | - | M-20 |  |

##### Clause M-21 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m21.torsion` | m21.torsion |  | - | - | M-21 |  |

##### Clause M-31 (2; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m31.fixed-hogging` | m31.fixed-hogging |  | - | - | M-31 |  |
| `m31.yield-line` | m31.yield-line |  | - | - | M-31 |  |

##### Clause M-32 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m32.arching` | m32.arching |  | - | - | M-32 |  |

##### Clause M-34 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m34.basement` | m34.basement |  | - | - | M-34 |  |

##### Clause M-40 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m40.flexure` | m40.flexure |  | - | - | M-40 |  |

##### Clause M-41 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m41.shear` | m41.shear |  | - | - | M-41 |  |

##### Clause M-42 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m42.deep` | m42.deep |  | - | - | M-42 |  |

##### Clause M-43 (2; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m43.confined` | m43.confined |  | - | - | M-43 |  |
| `m43.prestress` | m43.prestress |  | - | - | M-43 |  |

##### Clause M-45 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m45.bond` | m45.bond |  | - | - | M-45 |  |

##### Clause M-50 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m50.ties` | m50.ties |  | - | - | M-50 |  |

##### Clause M-51 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m51.anchor` | m51.anchor |  | - | - | M-51 |  |

##### Clause M-52 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m52.joints` | m52.joints |  | - | - | M-52 |  |

##### Clause M-60 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m60.exposure` | m60.exposure |  | - | - | M-60 |  |

##### Clause M-61 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m61.slender` | m61.slender |  | - | - | M-61 |  |

##### Clause M-70 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m70.execution` | m70.execution |  | - | - | M-70 |  |

##### Clause M-80 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m80.limits` | m80.limits |  | - | - | M-80 |  |

##### Clause M-81 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m81.phis` | m81.phis |  | - | - | M-81 |  |

##### Clause M-82 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m82.shear` | m82.shear |  | - | - | M-82 |  |

##### Clause M-90 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m90.fire` | m90.fire |  | - | - | M-90 |  |

##### Clause M-91 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m91.fire` | m91.fire |  | - | - | M-91 |  |

##### Clause M-95 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m95.q` | m95.q |  | - | - | M-95 |  |

##### Clause NA.2.4.3 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-m.accidental` | γ_M accidental |  | - | 1.3 | NA.2.4.3 |  |

##### Clause Table 2.1 (1; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `durability.mortar` | durability.mortar |  | - | - | Table 2.1 |  |

##### Clause Table 2.3 / NA.2.4.3 (5; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-m.c1` | γ_M class 1 |  | 1.5 | 1.5 | Table 2.3 / NA.2.4.3 |  |
| `gamma-m.c2` | γ_M class 2 |  | 1.7 | 1.7 | Table 2.3 / NA.2.4.3 |  |
| `gamma-m.c3` | γ_M class 3 |  | 2 | 1.7 | Table 2.3 / NA.2.4.3 |  |
| `gamma-m.c4` | γ_M class 4 |  | 2.2 | 2 | Table 2.3 / NA.2.4.3 |  |
| `gamma-m.c5` | γ_M class 5 |  | 2.5 | 2 | Table 2.3 / NA.2.4.3 |  |

##### Clause Table A.1 (30; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `delta.h100.w100` | delta h=100 w=100 |  | 1 | 1 | EN 772-1 Table A.1 |  |
| `delta.h100.w150` | delta h=100 w=150 |  | 0.9 | 0.9 | EN 772-1 Table A.1 |  |
| `delta.h100.w200` | delta h=100 w=200 |  | 0.8 | 0.8 | EN 772-1 Table A.1 |  |
| `delta.h100.w250` | delta h=100 w=250 |  | 0.75 | 0.75 | EN 772-1 Table A.1 |  |
| `delta.h100.w50` | delta h=100 w=50 |  | 1.15 | 1.15 | EN 772-1 Table A.1 |  |
| `delta.h150.w100` | delta h=150 w=100 |  | 1.2 | 1.2 | EN 772-1 Table A.1 |  |
| `delta.h150.w150` | delta h=150 w=150 |  | 1.1 | 1.1 | EN 772-1 Table A.1 |  |
| `delta.h150.w200` | delta h=150 w=200 |  | 1 | 1 | EN 772-1 Table A.1 |  |
| `delta.h150.w250` | delta h=150 w=250 |  | 0.95 | 0.95 | EN 772-1 Table A.1 |  |
| `delta.h150.w50` | delta h=150 w=50 |  | 1.3 | 1.3 | EN 772-1 Table A.1 |  |
| `delta.h200.w100` | delta h=200 w=100 |  | 1.35 | 1.35 | EN 772-1 Table A.1 |  |
| `delta.h200.w150` | delta h=200 w=150 |  | 1.25 | 1.25 | EN 772-1 Table A.1 |  |
| `delta.h200.w200` | delta h=200 w=200 |  | 1.15 | 1.15 | EN 772-1 Table A.1 |  |
| `delta.h200.w250` | delta h=200 w=250 |  | 1.1 | 1.1 | EN 772-1 Table A.1 |  |
| `delta.h200.w50` | delta h=200 w=50 |  | 1.45 | 1.45 | EN 772-1 Table A.1 |  |
| `delta.h250.w100` | delta h=250 w=100 |  | 1.45 | 1.45 | EN 772-1 Table A.1 |  |
| `delta.h250.w150` | delta h=250 w=150 |  | 1.35 | 1.35 | EN 772-1 Table A.1 |  |
| `delta.h250.w200` | delta h=250 w=200 |  | 1.25 | 1.25 | EN 772-1 Table A.1 |  |
| `delta.h250.w250` | delta h=250 w=250 |  | 1.15 | 1.15 | EN 772-1 Table A.1 |  |
| `delta.h250.w50` | delta h=250 w=50 |  | 1.55 | 1.55 | EN 772-1 Table A.1 |  |
| `delta.h50.w100` | delta h=50 w=100 |  | 0.75 | 0.75 | EN 772-1 Table A.1 |  |
| `delta.h50.w150` | delta h=50 w=150 |  | 0.7 | 0.7 | EN 772-1 Table A.1 |  |
| `delta.h50.w200` | delta h=50 w=200 |  | 0.7 | 0.7 | EN 772-1 Table A.1 |  |
| `delta.h50.w250` | delta h=50 w=250 |  | 0.7 | 0.7 | EN 772-1 Table A.1 |  |
| `delta.h50.w50` | delta h=50 w=50 |  | 0.85 | 0.85 | EN 772-1 Table A.1 |  |
| `delta.h65.w100` | delta h=65 w=100 |  | 0.85 | 0.85 | EN 772-1 Table A.1 |  |
| `delta.h65.w150` | delta h=65 w=150 |  | 0.75 | 0.75 | EN 772-1 Table A.1 |  |
| `delta.h65.w200` | delta h=65 w=200 |  | 0.7 | 0.7 | EN 772-1 Table A.1 |  |
| `delta.h65.w250` | delta h=65 w=250 |  | 0.65 | 0.65 | EN 772-1 Table A.1 |  |
| `delta.h65.w50` | delta h=65 w=50 |  | 0.95 | 0.95 | EN 772-1 Table A.1 |  |

#### EN 1996, part 1-2 (26)

##### Clause 3.3 (1; DIN EN 1996-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-91.1` | 5 % rule |  | - | - | 3.3 | en1995-en1996-target.md:708: at minimum implement the `5 % rule` of Annex C (reduced strength factor `kθ`). ****. |

##### Clause ? (1; DIN EN 1996-1-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-90.1` | and |  | - | - | M-90 Tabulated minimum thickness (EN 1996-1-2 Anne | en1995-en1996-target.md:702: 1.0` and `1.4?` **** (data table `FireTable{material, group, rei, mu_fi, mortar -> t_min}`). |

##### Clause Annex B (24; DIN EN 1996-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire.aac.rei120` | t_min aac REI 120 | mm | 200 | 200 | Annex B tabulated data |  |
| `fire.aac.rei180` | t_min aac REI 180 | mm | 250 | 250 | Annex B tabulated data |  |
| `fire.aac.rei240` | t_min aac REI 240 | mm | 300 | 300 | Annex B tabulated data |  |
| `fire.aac.rei30` | t_min aac REI 30 | mm | 115 | 115 | Annex B tabulated data |  |
| `fire.aac.rei60` | t_min aac REI 60 | mm | 140 | 140 | Annex B tabulated data |  |
| `fire.aac.rei90` | t_min aac REI 90 | mm | 175 | 175 | Annex B tabulated data |  |
| `fire.casi.rei120` | t_min casi REI 120 | mm | 175 | 175 | Annex B tabulated data |  |
| `fire.casi.rei180` | t_min casi REI 180 | mm | 240 | 240 | Annex B tabulated data |  |
| `fire.casi.rei240` | t_min casi REI 240 | mm | 300 | 300 | Annex B tabulated data |  |
| `fire.casi.rei30` | t_min casi REI 30 | mm | 90 | 90 | Annex B tabulated data |  |
| `fire.casi.rei60` | t_min casi REI 60 | mm | 115 | 115 | Annex B tabulated data |  |
| `fire.casi.rei90` | t_min casi REI 90 | mm | 140 | 140 | Annex B tabulated data |  |
| `fire.clay.rei120` | t_min clay REI 120 | mm | 175 | 175 | Annex B tabulated data |  |
| `fire.clay.rei180` | t_min clay REI 180 | mm | 240 | 240 | Annex B tabulated data |  |
| `fire.clay.rei240` | t_min clay REI 240 | mm | 240 | 240 | Annex B tabulated data |  |
| `fire.clay.rei30` | t_min clay REI 30 | mm | 100 | 100 | Annex B tabulated data |  |
| `fire.clay.rei60` | t_min clay REI 60 | mm | 100 | 100 | Annex B tabulated data |  |
| `fire.clay.rei90` | t_min clay REI 90 | mm | 150 | 150 | Annex B tabulated data |  |
| `fire.concrete.rei120` | t_min concrete REI 120 | mm | 170 | 170 | Annex B tabulated data |  |
| `fire.concrete.rei180` | t_min concrete REI 180 | mm | 240 | 240 | Annex B tabulated data |  |
| `fire.concrete.rei240` | t_min concrete REI 240 | mm | 300 | 300 | Annex B tabulated data |  |
| `fire.concrete.rei30` | t_min concrete REI 30 | mm | 90 | 90 | Annex B tabulated data |  |
| `fire.concrete.rei60` | t_min concrete REI 60 | mm | 100 | 100 | Annex B tabulated data |  |
| `fire.concrete.rei90` | t_min concrete REI 90 | mm | 140 | 140 | Annex B tabulated data |  |

#### EN 1996, part 2 (3)

##### Clause 8.6 (1; DIN EN 1996-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-70.2` | chase_ok |  | - | - | 8.6 | en1995-en1996-target.md:672: Check `chase_ok` + reduced `tef` for resistance ****. |

##### Clause ? (2; DIN EN 1996-2/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-60.1` | Frost-Beständigkeit |  | - | - | M-60 Exposure and durability (EN 1996-2, EN 1996-1 | en1995-en1996-target.md:660: - Exposure class MX1…MX5, minimum unit freeze/thaw resistance F1/F2, reinforcement cover and protection class (MX), morta |
| `m-70.1` | ? |  | - | - | M-70 Execution classes and tolerances (EN 1996-2 3 | en1995-en1996-target.md:671: piers area ≥ 0.04 m² (NA-DE ≥ 0.10 m².1)... ****. |

#### EN 1996, part 3 (7)

##### Clause 2.5 (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `x1-fk-for-clay-group-1-g.3` | t = 0.365 |  | - | - | 2.5 | en1995-en1996-target.md:889: **X11 Basement wall minimum normal force (NA-DE/EN 1996-3).** `t = 0.365`, `h = 2.5 m`, `he = 2.0 m`, `ρe = 18 kN/m³`: `N |

##### Clause 2.6 (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-34.1` | N1,Rd |  | - | - | 2.6 | en1995-en1996-target.md:624: - Simplified NA/EN 1996-3 condition (Kellerwand): `N1,Rd` between `NEd,sup,min ≥ ρe h he²/(20 t)` and `NEd,max ≤ 0.45 t f |

##### Clause 2.75 (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `x1-fk-for-clay-group-1-g.2` | t = 240 mm |  | - | - | 2.75 | en1995-en1996-target.md:887: `Φs = 0.85 − 0.0011·8.594² = 0.7688` **** |

##### Clause 4.2.2.3 (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-81.1` | Φ1, Φ2, Φ3 |  | - | - | 4.2.2.3 | en1995-en1996-target.md:684: the exact set of `Φ1, Φ2, Φ3` and the roof-floor top storey rule `Φs = Φ3?` must be **** against the NA text, as must `ρ` |

##### Clause 5.5.1.4 (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-11.1` | hef/t ≤ 21 |  | - | - | 5.5.1.4 | en1995-en1996-target.md:556: ≤ 21 for simplified method EN 1996-3 (`hef/t ≤ 21`)..) **** |

##### Clause 6.0 (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-80.1` | H ≤ 20 m |  | - | - | 6.0 | en1995-en1996-target.md:678: **** for each NA value. |

##### Clause ? (1; DIN EN 1996-3/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l903.1` | kmod |  | - | - | 15. Open verification items ([V] summary) | en1995-en1996-target.md:903: No value marked may be hard-coded before it is verified against the standard text and cross-checked with the oracle. |

#### EN 1996, part not stated (20)

##### Clause 1.0 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-10.1` | ρ2 = 1.0 |  | - | - | 1.0 | en1995-en1996-target.md:548: `ρ2 = 1.0` timber floors ****. |

##### Clause 1.2 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-01.2` | SFK |  | - | - | 1.2 | en1995-en1996-target.md:498: NA-DE: unit strength classes `SFK` 6…60, mean `fb = SFK × 1.2? `: DE class `12` means `fb,decl ≥ 12 N/mm²` min and mean b |

##### Clause 3.1 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-01.1` | ) **[V]**, |  | - | - | 3.1 | en1995-en1996-target.md:496: `h = 238, w = 240 → δ = 1.15?`) ****, `fb = δ fb,decl`. |

##### Clause 3.2 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-02.2` | KTable |  | - | - | 3.2 | en1995-en1996-target.md:503: AAC G1 0.55 / 0.80 / 0.45) **** table `KTable`. |

##### Clause 3.6.1.2 (2; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-02.1` | NM I, II, IIa, III, IIIa |  | - | - | 3.6.1.2 | en1995-en1996-target.md:502: NA-DE: `NM I, II, IIa, III, IIIa`, `DM` (thin layer), `LM21`, `LM36` with `fm` = 2.5, 5, 10, 20 (IIIa: 15?) ****. |
| `m-02.3` | fk |  | - | - | 3.6.1.2 | en1995-en1996-target.md:505: - Unit with filled collar joints (>0.6 hole): `fk` factor 0.8 for dry / unfilled vertical joints `fk ≥…` ****. |

##### Clause 3.6.2 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-03.3` | fvk = fvk0 + 0.4 σDd |  | - | - | 3.6.2 | en1995-en1996-target.md:514: - NA-DE: `fvk = fvk0 + 0.4 σDd` with additional tensile limits `fvk ≤ 0.045 fb?` (Brickwork `fbt,cal = 0.026 fb` for hori |

##### Clause 3.6.3 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-04.2` | fxk1 |  | - | - | 3.6.3 | en1995-en1996-target.md:519: - NA-DE: `fxk1` in DE denoted `fxk,1 = 0.0 …` tension-free for lateral design when no tension considered ("Biegezugfestig |

##### Clause 3.7 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-06.1` | G = 0.4 E |  | - | - | 3.7 | en1995-en1996-target.md:528: Poisson, shear `G = 0.4 E`) ****. |

##### Clause 5.4 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-10.2` | ρn |  | - | - | 5.4 | en1995-en1996-target.md:551: NA-DE: `ρn` for walls with openings: reduce via `h/L` for unsupported length at openings (`h_ef` calculation for walls wi |

##### Clause 6.2 (2; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-03.2` | fvlt |  | - | - | 6.2 | en1995-en1996-target.md:513: - Filled/unfilled perpends: for unfilled head joints (`fvlt`): `fvk = 0.5 fvk0 + 0.4 σd` (EN 6.2 (2)) ****. |
| `m-20.2` | ULS: ε |  | - | - | 6.2 | en1995-en1996-target.md:593: overturning stability `ULS: ε` with `0.9 G` ****. |

##### Clause 8.5.1 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-51.1` | Fd = 1.5%? |  | - | - | 8.5.1 | en1995-en1996-target.md:652: - Anchors: horizontal restraint force `Fd = 1.5%?`: lateral anchor force of floor tie `Td = max(15 kN?, …)`; tie force fo |

##### Clause ? (3; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l901.1` | ? |  | - | - | 15. Open verification items ([V] summary) | en1995-en1996-target.md:901: Open verification items ( summary) |
| `x1-fk-for-clay-group-1-g.1` | α |  | - | - | EN 1996 examples (X1–X10) | en1995-en1996-target.md:883: the kernel's table-based `α` must be ≥ this elastic/plastic bound consistency check and match the oracle for the table ca |
| `x1-fk-for-clay-group-1-g.4` | t = 365 mm |  | - | - | EN 1996 examples (X1–X10) | en1995-en1996-target.md:891: the table row is pinned from the NA by the oracle (****). |

##### Clause Table 2.3 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-07.1` | . NA-DE: |  | - | - | Table 2.3 | en1995-en1996-target.md:535: accidental fire 1.0 ****. |

##### Clause Table 3.2 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-06.2` | φ∞ = φ(∞) |  | - | - | Table 3.2 | en1995-en1996-target.md:529: manufactured stone 1.0–2.0 — NA-DE gives single values (e.g. clay 1.0 ****. |

##### Clause Table 3.4 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-03.1` | 0.15 |  | - | - | Table 3.4 | en1995-en1996-target.md:512: Group 2–4 `0.15` ****. |

##### Clause Table 3.6 (1; DIN EN 1996/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m-04.1` | 0.05/0.10 … |  | - | - | Table 3.6 | en1995-en1996-target.md:518: calcium silicate `0.05/0.10 …`) **** |

### EN 1997 (119)

#### EN 1997, part 1 (100)

##### Clause 1.60 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l220.1` | R_c;m |  | - | - | 1.60 | en1997-en1998-target.md:220: - `R_c;m` from signal matching (CAPWAP); `ξ5/ξ6`: n>=2 1.60/1.50 ... (EN Table A.11, ≥5: 1.50/1.35; ≥20: 1.45/1.30?) ; DE |

##### Clause 2.3 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l125.1` | m |  | - | - | 2.3 | en1997-en1998-target.md:125: rect with `m` per Annex D formula. |

##### Clause 6.10 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l128.1` | μ <= 1 |  | - | - | 6.10 | en1997-en1998-target.md:128: - Pass/fail: `μ <= 1`, `e` inside the 1/3 rule (DE: gaping joint `e_B <= B/3` for permanent combos in BS-P, `<= B/2` for |

##### Clause 7.7 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `7.7.lateral-deflection-limit` | y_lim | m | 0.01 | - | 7.7 |  |

##### Clause 9.7 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `9.blum-embedment-increase` | t/t0 |  | 1.2 | - | 9.7 |  |

##### Clause DIN 1054 GZ 1C (3; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `de.geo3.a.q` | γ_Q GEO-3 accidental |  | - | 1.1 | DIN 1054 GZ 1C |  |
| `de.geo3.p.q` | γ_Q GEO-3 persistent |  | - | 1.3 | DIN 1054 GZ 1C |  |
| `de.geo3.t.q` | γ_Q GEO-3 transient |  | - | 1.2 | DIN 1054 GZ 1C |  |

##### Clause DIN 1054 Table A 2.1 (6; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `de.act.a.g` | γ_G accidental |  | - | 1.1 | DIN 1054 Table A 2.1 |  |
| `de.act.a.q` | γ_Q accidental |  | - | 1.15 | DIN 1054 Table A 2.1 |  |
| `de.act.p.g` | γ_G persistent |  | - | 1.35 | DIN 1054 Table A 2.1 |  |
| `de.act.p.q` | γ_Q persistent |  | - | 1.5 | DIN 1054 Table A 2.1 |  |
| `de.act.t.g` | γ_G transient |  | - | 1.2 | DIN 1054 Table A 2.1 |  |
| `de.act.t.q` | γ_Q transient |  | - | 1.3 | DIN 1054 Table A 2.1 |  |

##### Clause DIN 1054 Table A 2.2 (3; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `de.soil.a.f` | γ_φ=γ_c=γ_cu accidental |  | - | 1.1 | DIN 1054 Table A 2.2 |  |
| `de.soil.p.f` | γ_φ=γ_c=γ_cu persistent |  | - | 1.25 | DIN 1054 Table A 2.2 |  |
| `de.soil.t.f` | γ_φ=γ_c=γ_cu transient |  | - | 1.15 | DIN 1054 Table A 2.2 |  |

##### Clause DIN 1054 Table A 2.3 (12; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `de.res.a` | γ_a DE |  | - | 1.1 | DIN 1054 Table A 2.3 |  |
| `de.res.a.re` | γ_R,e accidental |  | - | 1.2 | DIN 1054 Table A 2.3 |  |
| `de.res.a.rv` | γ_R,v accidental |  | - | 1.2 | DIN 1054 Table A 2.3 |  |
| `de.res.b` | γ_b DE |  | - | 1.1 | DIN 1054 Table A 2.3 |  |
| `de.res.p.re` | γ_R,e persistent |  | - | 1.4 | DIN 1054 Table A 2.3 |  |
| `de.res.p.rv` | γ_R,v persistent |  | - | 1.4 | DIN 1054 Table A 2.3 |  |
| `de.res.rh` | γ_rh DE |  | - | 1.1 | DIN 1054 Table A 2.3 |  |
| `de.res.s` | γ_s DE |  | - | 1.1 | DIN 1054 Table A 2.3 |  |
| `de.res.st` | γ_st DE |  | - | 1.15 | DIN 1054 Table A 2.3 |  |
| `de.res.t` | γ_t DE |  | - | 1.1 | DIN 1054 Table A 2.3 |  |
| `de.res.t.re` | γ_R,e transient |  | - | 1.3 | DIN 1054 Table A 2.3 |  |
| `de.res.t.rv` | γ_R,v transient |  | - | 1.3 | DIN 1054 Table A 2.3 |  |

##### Clause EN 1997-1 2.4.7 (2; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-acc.g` | γ_G accidental |  | 1 | 1 | EN 1997-1 2.4.7 |  |
| `a-acc.q` | γ_Q accidental |  | 1 | 1 | EN 1997-1 2.4.7 |  |

##### Clause EN 1998-5 7.3.2 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `9.seismic.gamma-sliding` | gamma_Rh,E |  | 1 | - | EN 1998-5 7.3.2 |  |

##### Clause Table A.3/A.4 (4; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a1.g` | γ_G A1 |  | 1.35 | 1.35 | Table A.3/A.4 |  |
| `a1.q` | γ_Q A1 |  | 1.5 | 1.5 | Table A.3/A.4 |  |
| `a2.g` | γ_G A2 |  | 1 | 1 | Table A.3/A.4 |  |
| `a2.q` | γ_Q A2 |  | 1.3 | 1.3 | Table A.3/A.4 |  |

##### Clause Table A.4 (8; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `m1.c` | γ_c M1 |  | 1 | 1 | Table A.4 |  |
| `m1.cu` | γ_cu M1 |  | 1 | 1 | Table A.4 |  |
| `m1.gamma` | γ_gamma M1 |  | 1 | 1 | Table A.4 |  |
| `m1.phi` | γ_phi M1 |  | 1 | 1 | Table A.4 |  |
| `m2.c` | γ_c M2 |  | 1.25 | 1.25 | Table A.4 |  |
| `m2.cu` | γ_cu M2 |  | 1.4 | 1.4 | Table A.4 |  |
| `m2.gamma` | γ_gamma M2 |  | 1 | 1 | Table A.4 |  |
| `m2.phi` | γ_phi M2 |  | 1.25 | 1.25 | Table A.4 |  |

##### Clause Tables A.1, A.2, A.17 / A 2.1 (24; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `upl.a.equdst` | equdst accidental |  | 1 | 1 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.equq` | equq accidental |  | 1 | 1.15 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.equstb` | equstb accidental |  | 1 | 0.95 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.gdst` | gdst accidental |  | 1 | 1 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.gstb` | gstb accidental |  | 1 | 0.95 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.hyd` | hyd accidental |  | 1 | 1.3 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.hydstb` | hydstb accidental |  | 1 | 0.95 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.a.qdst` | qdst accidental |  | 1 | 1.15 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.equdst` | equdst persistent |  | 1.1 | 1.05 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.equq` | equq persistent |  | 1.5 | 1.5 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.equstb` | equstb persistent |  | 0.9 | 0.95 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.gdst` | gdst persistent |  | 1 | 1 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.gstb` | gstb persistent |  | 0.9 | 0.9 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.hyd` | hyd persistent |  | 1.35 | 1.8 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.hydstb` | hydstb persistent |  | 0.9 | 0.9 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.p.qdst` | qdst persistent |  | 1.5 | 1.5 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.equdst` | equdst transient |  | 1.1 | 1.05 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.equq` | equq transient |  | 1.5 | 1.3 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.equstb` | equstb transient |  | 0.9 | 0.95 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.gdst` | gdst transient |  | 1 | 1 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.gstb` | gstb transient |  | 0.9 | 0.9 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.hyd` | hyd transient |  | 1.35 | 1.45 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.hydstb` | hydstb transient |  | 0.9 | 0.9 | Tables A.1, A.2, A.17 / A 2.1 |  |
| `upl.t.qdst` | qdst transient |  | 1.5 | 1.3 | Tables A.1, A.2, A.17 / A 2.1 |  |

##### Clause Tables A.5-A.14 (32; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `r1.a` | γ_a R1 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r1.b` | γ_b R1 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r1.re` | γ_re R1 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r1.rh` | γ_rh R1 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r1.rv` | γ_rv R1 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r1.s` | γ_s R1 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r1.st` | γ_st R1 |  | 1.25 | 1.25 | Tables A.5-A.14 |  |
| `r1.t` | γ_t R1 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r2.a` | γ_a R2 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r2.b` | γ_b R2 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r2.re` | γ_re R2 |  | 1.4 | 1.4 | Tables A.5-A.14 |  |
| `r2.rh` | γ_rh R2 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r2.rv` | γ_rv R2 |  | 1.4 | 1.4 | Tables A.5-A.14 |  |
| `r2.s` | γ_s R2 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r2.st` | γ_st R2 |  | 1.15 | 1.15 | Tables A.5-A.14 |  |
| `r2.t` | γ_t R2 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r3.a` | γ_a R3 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r3.b` | γ_b R3 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r3.re` | γ_re R3 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r3.rh` | γ_rh R3 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r3.rv` | γ_rv R3 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r3.s` | γ_s R3 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r3.st` | γ_st R3 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r3.t` | γ_t R3 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r4.a` | γ_a R4 |  | 1.1 | 1.1 | Tables A.5-A.14 |  |
| `r4.b` | γ_b R4 |  | 1.3 | 1.3 | Tables A.5-A.14 |  |
| `r4.re` | γ_re R4 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r4.rh` | γ_rh R4 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r4.rv` | γ_rv R4 |  | 1 | 1 | Tables A.5-A.14 |  |
| `r4.s` | γ_s R4 |  | 1.3 | 1.3 | Tables A.5-A.14 |  |
| `r4.st` | γ_st R4 |  | 1.6 | 1.6 | Tables A.5-A.14 |  |
| `r4.t` | γ_t R4 |  | 1.3 | 1.3 | Tables A.5-A.14 |  |

#### EN 1997, part 2 (1)

##### Clause 2.2 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l292.1` | z_a >= max(6 m, 2 B) |  | - | - | 2.2 | en1997-en1998-target.md:292: \| GI-01 \| 2.2/2.3 + Annex B.3 \| Investigation extent / Erkundungsumfang \| GC, structure footprint \| min depth `z_a >= max |

#### EN 1997, part not stated (18)

##### Clause 1.0 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l136.1` | [V] |  | - | - | 1.0 | en1997-en1998-target.md:136: passive mobilisation: only 50 % of E_p unless wall movement admissible (DE: use mobilised passive pressure `E_p,mob` at s |

##### Clause 1.5 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l318.1` | η >= 1.5 |  | - | - | 1.5 | en1997-en1998-target.md:318: Kranz `η >= 1.5` (EN 1.0 with design values ). |

##### Clause 1.25 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l316.1` | F_s >= 1/1.4 |  | - | - | 1.25 | en1997-en1998-target.md:316: equal to `F_s >= 1/1.4` flows (DE: γ=1.0 EQU-like ). |

##### Clause 2.5 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l145.1` | μ = E_dst,d/E_stb,d <= 1 |  | - | - | 2.5 | en1997-en1998-target.md:145: - Criterion: `μ = E_dst,d/E_stb,d <= 1`. DE additionally eccentricity: BS-P `e <= B/3` (gaping joint at most up to centre |

##### Clause 2.8 (2; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l166.1` | σ0 = q - γ d |  | - | - | 2.8 | en1997-en1998-target.md:166: - Inputs: net base pressure `σ0 = q - γ d`, geometry, layered `Eoed,i` (or `Es`), `ν`, limit depth `z_lim` where `σz = 0. |
| `l170.1` | [V] |  | - | - | 2.8 | en1997-en1998-target.md:170: NDP: isolated footing 50 mm typical (EN informative), DE: project-defined . |

##### Clause 2.14 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l323.1` | R = 575 s sqrt(H k) |  | - | - | 2.14 | en1997-en1998-target.md:323: Kusakin `R = 575 s sqrt(H k)` with k in m/s? |

##### Clause 7.1 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l183.1` | [V] |  | - | - | 7.1 | en1997-en1998-target.md:183: - ξ (NA-DE, DIN 1054 Table A 7.1) : n=1: 1.35/1.35; 2: 1.25/1.15; 3: 1.15/1.05; 4: 1.05/1.00; >=5: 1.00/1.00. Stiff struc |

##### Clause 9.81 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l97.1` | [V] |  | - | - | 9.81 | en1997-en1998-target.md:97: - NDP: γw 9.81 (EN) / 10.0 (DE practice) . |

##### Clause ? (6; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l212.1` | s >= 3 D |  | - | - | PILE-06 Group effects | en1997-en1998-target.md:212: - NDP: DE group effects for spacing `s >= 3 D` none, closer: reduction . |
| `rw-01.1` | δa = 2/3 φ' |  | - | - | RW-01 Active earth pressure, Coulomb | en1997-en1998-target.md:232: `δa = 2/3 φ'` rough, `δa = 0` smooth, DE: `δa = ±2/3 φ'` for normal rough, `0` for sheet pile cantilever? ). |
| `rw-02.1` | ; cohesion |  | - | - | RW-02 Active, Rankine | en1997-en1998-target.md:236: `Ka = (1 - sinφ')/(1 + sinφ')` level ground vertical smooth; sloped `Ka = cosβ (cosβ - sqrt(cos²β - cos²φ'))/(cosβ + sqrt |
| `rw-03.1` | [V] |  | - | - | RW-03 Passive earth pressure | en1997-en1998-target.md:240: ): `Kp(φ=30°, δ=-φ'/2, vertical wall, level) ≈ 4.0 ... 4.78`, Rankine `Kp = (1+sinφ')/(1-sinφ')` (=3.0 at 30°) |
| `rw-05.1` | (strip), line load |  | - | - | RW-05 Surcharges and water | en1997-en1998-target.md:248: strip load per Boussinesq `Δσh = (2 q/π)(β - sinβ cos(β + 2δ))` (strip), line load `Δσh = (4 Q/π) ...` (Terzaghi/Gerber), |
| `rw-07.1` | [V] |  | - | - | RW-07 Embedded (sheet pile, bored pile, diaphragm) | en1997-en1998-target.md:258: (EAB: increase 20 % for assumed rotation). |

##### Clause Table A2.1 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l45.1` | [V] |  | - | - | Table A2.1 | en1997-en1998-target.md:45: Table A2.1 (actions, DIN 1054:2010 Table A 2.1) : |

##### Clause Table A2.2 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l62.1` | [V] |  | - | - | Table A2.2 | en1997-en1998-target.md:62: Table A2.2 (soil parameters, only GZ 1C and DA3) : |

##### Clause Table A2.3 (1; DIN 1054:2021-04)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l72.1` | [V] |  | - | - | Table A2.3 | en1997-en1998-target.md:72: Table A2.3 (resistances, GZ 1B) : |

### EN 1998 (236)

#### EN 1998, part 1 (132)

##### Clause 1.00 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-01.1` | [V] |  | - | - | 1.00 | en1997-en1998-target.md:353: : A-R S 1.00, TB 0.05, TC 0.20, TD 2.0 |

##### Clause 1.3 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-50.1` | ΣM_Rc |  | - | - | 1.3 | en1997-en1998-target.md:482: exempt: top floor, and joints with `ΣM_Rc` check relaxed when columns of ground floor with `N_Ed` low... (NA-DE permits r |

##### Clause 3.2.2.3 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l343.1` | [V] |  | - | - | 3.2.2.3 | en1997-en1998-target.md:343: NA-DE: vertical spectrum follows DIN EN 1998-1/NA NA.? (`avg = 0.5 ag S`?, to be verified, NDP `avg_ratio` table asset). |

##### Clause 3.2.3.1.2(4) (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.time-history.zero-period-ratio` | mean zero-period acceleration over a_g·S |  | 1 | 1 | 3.2.3.1.2(4) |  |

##### Clause 4.2.5 / NA (4; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.gamma-i.i` | gamma_I class I |  | 0.8 | 0.8 | 4.2.5 / NA |  |
| `en1.gamma-i.ii` | gamma_I class II |  | 1 | 1 | 4.2.5 / NA |  |
| `en1.gamma-i.iii` | gamma_I class III |  | 1.2 | 1.2 | 4.2.5 / NA |  |
| `en1.gamma-i.iv` | gamma_I class IV |  | 1.4 | 1.4 | 4.2.5 / NA |  |

##### Clause 4.4.2 (overturning stability) (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.overturning.stabilising-factor` | γ stabilising on G_k in the seismic comb |  | 1 | 1 | 4.4.2 (overturning stability) |  |

##### Clause 4.4.2.2 plan envelope (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.plan-gap-ratio` | plan gap ratio limit |  | 0.05 | 0.05 | 4.4.2.2 plan envelope |  |

##### Clause 4.4.3.2 (3; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.drift-theta.brittle` | drift limit ratio brittle |  | 0.005 | 0.005 | 4.4.3.2 |  |
| `en1.drift-theta.ductile` | drift limit ratio ductile |  | 0.0075 | 0.0075 | 4.4.3.2 |  |
| `en1.drift-theta.isolated` | drift limit ratio isolated |  | 0.01 | 0.01 | 4.4.3.2 |  |

##### Clause 5.4.2.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-50.2` | [V] |  | - | - | 5.4.2.2 | en1997-en1998-target.md:485: (dual walls DCH), design envelope of bending moments (tension shift `a_l`). |

##### Clause 5.4.3 / 5.5.3 (18; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.rc.rho-max.beam.dch` | rho_max beam DCH |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.beam.dcm` | rho_max beam DCM |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.beam.low` | rho_max beam LOW |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.column.dch` | rho_max column DCH |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.column.dcm` | rho_max column DCM |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.column.low` | rho_max column LOW |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.other.dch` | rho_max other DCH |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.other.dcm` | rho_max other DCM |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-max.other.low` | rho_max other LOW |  | 0.04 | 0.04 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.beam.dch` | rho_min beam DCH |  | 0.0035 | 0.0035 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.beam.dcm` | rho_min beam DCM |  | 0.0025 | 0.0025 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.beam.low` | rho_min beam LOW |  | 0.0025 | 0.0025 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.column.dch` | rho_min column DCH |  | 0.01 | 0.01 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.column.dcm` | rho_min column DCM |  | 0.01 | 0.01 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.column.low` | rho_min column LOW |  | 0.01 | 0.01 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.other.dch` | rho_min other DCH |  | 0.002 | 0.002 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.other.dcm` | rho_min other DCM |  | 0.002 | 0.002 | 5.4.3 / 5.5.3 |  |
| `en1.rc.rho-min.other.low` | rho_min other LOW |  | 0.002 | 0.002 | 5.4.3 / 5.5.3 |  |

##### Clause 5.4.3.2.1 / 5.5.3.2.1 (3; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.rc.column-min-b.dch` | RC column minimum width DCH | m | 0.25 | 0.25 | 5.4.3.2.1 / 5.5.3.2.1 |  |
| `en1.rc.column-min-b.dcm` | RC column minimum width DCM | m | 0.25 | 0.25 | 5.4.3.2.1 / 5.5.3.2.1 |  |
| `en1.rc.column-min-b.low` | RC column minimum width LOW | m | 0.2 | 0.2 | 5.4.3.2.1 / 5.5.3.2.1 |  |

##### Clause 5.4.3.2.2 / 5.5.3.2.2 (3; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.rc.omega-wd-min.dch` | omega_wd min DCH |  | 0.12 | 0.12 | 5.4.3.2.2 / 5.5.3.2.2 |  |
| `en1.rc.omega-wd-min.dcm` | omega_wd min DCM |  | 0.08 | 0.08 | 5.4.3.2.2 / 5.5.3.2.2 |  |
| `en1.rc.omega-wd-min.low` | omega_wd min LOW |  | 0 | 0 | 5.4.3.2.2 / 5.5.3.2.2 |  |

##### Clause NA.12 (unverified) (5; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.masonry-wall-ratio.ge0-7` | masonry wall area ratio a_gR band ge0-7 |  | 0.04 | 0.04 | NA.12 (unverified) |  |
| `en1.masonry-wall-ratio.importance-iii-iv` | masonry wall ratio factor importance III |  | 1.25 | 1.25 | NA.12 (unverified) |  |
| `en1.masonry-wall-ratio.lt0-5` | masonry wall area ratio a_gR band lt0-5 |  | 0.02 | 0.02 | NA.12 (unverified) |  |
| `en1.masonry-wall-ratio.lt0-7` | masonry wall area ratio a_gR band lt0-7 |  | 0.03 | 0.03 | NA.12 (unverified) |  |
| `en1.masonry-wall-ratio.none` | masonry wall area ratio a_gR band none |  | 0 | 0 | NA.12 (unverified) |  |

##### Clause Table 3.2 (20; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t32.a.s` | S ground A |  | 1 | - | Table 3.2 |  |
| `t32.a.tb` | T_B ground A | s | 0.15 | - | Table 3.2 |  |
| `t32.a.tc` | T_C ground A | s | 0.4 | - | Table 3.2 |  |
| `t32.a.td` | T_D ground A | s | 2 | - | Table 3.2 |  |
| `t32.b.s` | S ground B |  | 1.2 | - | Table 3.2 |  |
| `t32.b.tb` | T_B ground B | s | 0.15 | - | Table 3.2 |  |
| `t32.b.tc` | T_C ground B | s | 0.5 | - | Table 3.2 |  |
| `t32.b.td` | T_D ground B | s | 2 | - | Table 3.2 |  |
| `t32.c.s` | S ground C |  | 1.15 | - | Table 3.2 |  |
| `t32.c.tb` | T_B ground C | s | 0.2 | - | Table 3.2 |  |
| `t32.c.tc` | T_C ground C | s | 0.6 | - | Table 3.2 |  |
| `t32.c.td` | T_D ground C | s | 2 | - | Table 3.2 |  |
| `t32.d.s` | S ground D |  | 1.35 | - | Table 3.2 |  |
| `t32.d.tb` | T_B ground D | s | 0.2 | - | Table 3.2 |  |
| `t32.d.tc` | T_C ground D | s | 0.8 | - | Table 3.2 |  |
| `t32.d.td` | T_D ground D | s | 2 | - | Table 3.2 |  |
| `t32.e.s` | S ground E |  | 1.4 | - | Table 3.2 |  |
| `t32.e.tb` | T_B ground E | s | 0.15 | - | Table 3.2 |  |
| `t32.e.tc` | T_C ground E | s | 0.5 | - | Table 3.2 |  |
| `t32.e.td` | T_D ground E | s | 2 | - | Table 3.2 |  |

##### Clause Table 3.3 (20; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `t33.a.s` | S ground A |  | 1 | - | Table 3.3 |  |
| `t33.a.tb` | T_B ground A | s | 0.05 | - | Table 3.3 |  |
| `t33.a.tc` | T_C ground A | s | 0.25 | - | Table 3.3 |  |
| `t33.a.td` | T_D ground A | s | 1.2 | - | Table 3.3 |  |
| `t33.b.s` | S ground B |  | 1.35 | - | Table 3.3 |  |
| `t33.b.tb` | T_B ground B | s | 0.05 | - | Table 3.3 |  |
| `t33.b.tc` | T_C ground B | s | 0.25 | - | Table 3.3 |  |
| `t33.b.td` | T_D ground B | s | 1.2 | - | Table 3.3 |  |
| `t33.c.s` | S ground C |  | 1.5 | - | Table 3.3 |  |
| `t33.c.tb` | T_B ground C | s | 0.1 | - | Table 3.3 |  |
| `t33.c.tc` | T_C ground C | s | 0.25 | - | Table 3.3 |  |
| `t33.c.td` | T_D ground C | s | 1.2 | - | Table 3.3 |  |
| `t33.d.s` | S ground D |  | 1.8 | - | Table 3.3 |  |
| `t33.d.tb` | T_B ground D | s | 0.1 | - | Table 3.3 |  |
| `t33.d.tc` | T_C ground D | s | 0.3 | - | Table 3.3 |  |
| `t33.d.td` | T_D ground D | s | 1.2 | - | Table 3.3 |  |
| `t33.e.s` | S ground E |  | 1.6 | - | Table 3.3 |  |
| `t33.e.tb` | T_B ground E | s | 0.05 | - | Table 3.3 |  |
| `t33.e.tc` | T_C ground E | s | 0.25 | - | Table 3.3 |  |
| `t33.e.td` | T_D ground E | s | 1.2 | - | Table 3.3 |  |

##### Clause Table NA.4 (24; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `na4.ar.s` | S AR |  | - | 1 | Table NA.4 |  |
| `na4.ar.tb` | T_B AR | s | - | 0.05 | Table NA.4 |  |
| `na4.ar.tc` | T_C AR | s | - | 0.2 | Table NA.4 |  |
| `na4.ar.td` | T_D AR | s | - | 2 | Table NA.4 |  |
| `na4.br.s` | S BR |  | - | 1.25 | Table NA.4 |  |
| `na4.br.tb` | T_B BR | s | - | 0.05 | Table NA.4 |  |
| `na4.br.tc` | T_C BR | s | - | 0.25 | Table NA.4 |  |
| `na4.br.td` | T_D BR | s | - | 2 | Table NA.4 |  |
| `na4.bt.s` | S BT |  | - | 1.25 | Table NA.4 |  **conflict** |
| `na4.bt.tb` | T_B BT | s | - | 0.1 | Table NA.4 |  |
| `na4.bt.tc` | T_C BT | s | - | 0.3 | Table NA.4 |  |
| `na4.bt.td` | T_D BT | s | - | 2 | Table NA.4 |  |
| `na4.cr.s` | S CR |  | - | 1.5 | Table NA.4 |  |
| `na4.cr.tb` | T_B CR | s | - | 0.05 | Table NA.4 |  |
| `na4.cr.tc` | T_C CR | s | - | 0.3 | Table NA.4 |  |
| `na4.cr.td` | T_D CR | s | - | 2 | Table NA.4 |  |
| `na4.cs.s` | S CS |  | - | 0.75 | Table NA.4 |  |
| `na4.cs.tb` | T_B CS | s | - | 0.1 | Table NA.4 |  |
| `na4.cs.tc` | T_C CS | s | - | 0.5 | Table NA.4 |  |
| `na4.cs.td` | T_D CS | s | - | 2 | Table NA.4 |  |
| `na4.ct.s` | S CT |  | - | 1.5 | Table NA.4 |  **conflict** |
| `na4.ct.tb` | T_B CT | s | - | 0.1 | Table NA.4 |  |
| `na4.ct.tc` | T_C CT | s | - | 0.4 | Table NA.4 |  |
| `na4.ct.td` | T_D CT | s | - | 2 | Table NA.4 |  |

##### Clause Tables 5.1/6.2/7.2/8.1/9.1 (25; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.q-max.composite.dch` | q_max composite DCH |  | 6 | 6 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.composite.dcm` | q_max composite DCM |  | 4 | 4 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.composite.low` | q_max composite LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.default` | q_max default |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.masonry.dch` | q_max masonry DCH |  | 2.5 | 2.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.masonry.dcm` | q_max masonry DCM |  | 2.5 | 2.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.masonry.low` | q_max masonry LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.dual.dch` | q_max rc.dual DCH |  | 5.2 | 5.2 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.dual.dcm` | q_max rc.dual DCM |  | 3.6 | 3.6 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.dual.low` | q_max rc.dual LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.frame.dch` | q_max rc.frame DCH |  | 5.85 | 5.85 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.frame.dcm` | q_max rc.frame DCM |  | 3.9 | 3.9 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.frame.low` | q_max rc.frame LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.wall.dch` | q_max rc.wall DCH |  | 4.4 | 4.4 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.wall.dcm` | q_max rc.wall DCM |  | 3 | 3 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.rc.wall.low` | q_max rc.wall LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.steel.frame.dch` | q_max steel.frame DCH |  | 6.5 | 6.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.steel.frame.dcm` | q_max steel.frame DCM |  | 4 | 4 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.steel.frame.low` | q_max steel.frame LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.steel.wall.dch` | q_max steel.wall DCH |  | 5 | 5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.steel.wall.dcm` | q_max steel.wall DCM |  | 4 | 4 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.steel.wall.low` | q_max steel.wall LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.timber.dch` | q_max timber DCH |  | 5 | 5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.timber.dcm` | q_max timber DCM |  | 2.5 | 2.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |
| `en1.q-max.timber.low` | q_max timber LOW |  | 1.5 | 1.5 | Tables 5.1/6.2/7.2/8.1/9.1 |  |

#### EN 1998, part 2 (1)

##### Clause 3.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l514.1` | 0.9 |  | - | - | 3.2 | en1997-en1998-target.md:514: Not `0.9` → bridges use `0.7`/`0.5`/ |

#### EN 1998, part 4 (2)

##### Clause 1.7 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l546.1` | [V] |  | - | - | 1.7 | en1997-en1998-target.md:546: ), `h_c = H [1 - (cosh(1.84 H/R) - 1)/((1.84 H/R) sinh(1.84 H/R))]` |

##### Clause 3.4 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l545.1` | q_nominal |  | - | - | 3.4 | en1997-en1998-target.md:545: `q_nominal` (silo q = 1.5…2.0 ). |

#### EN 1998, part 5 (4)

##### Clause 3.1 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-01.2` | ag S <= 0.4 g |  | - | - | 3.1 | en1997-en1998-target.md:355: - Edge: vs30 on boundary (use lower class), site requiring special study (S1: cu 10-20 kPa soft clay, S2: liquefiable) => |

##### Clause 3.5 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l559.1` | (informative, |  | - | - | 3.5 | en1997-en1998-target.md:559: for sand: `a = 0.92, b = 1.25, c = 0.92, d = 1.25, e = 0.92, f = 0.41, m = 0.96, k = 0.39, k' = 0.39, c_T = 2.14, c'_T = |

##### Clause 4.1.4 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l555.1` | ag S <= 0.15 g |  | - | - | 4.1.4 | en1997-en1998-target.md:555: NA-DE `ag S <= 0.15 g`? |

##### Clause 5.4.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l561.1` | [V] |  | - | - | 5.4.2 | en1997-en1998-target.md:561: for `ag S < 0.1g` ... |

#### EN 1998, part not stated (97)

##### Clause 1.3 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-46.1` | V_diaph,d = γd 1.3 ... |  | - | - | 1.3 | en1997-en1998-target.md:462: - Foundation checks see SEI-70. Diaphragm check: `V_diaph,d = γd 1.3 ...` amplified by 1.3 (transfer forces) . |

##### Clause 1.5 (3; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-30.1` | q >= 1.5 |  | - | - | 1.5 | en1997-en1998-target.md:396: DE NA lower bound `q >= 1.5` in zone 1? . |
| `sei-30.3` | q <= 1.5 |  | - | - | 1.5 | en1997-en1998-target.md:410: - NDP DE: reduction for hysteresis? only `q <= 1.5` in zone 1/2 allowed to omit DCM detailing (DE NA permits DCL for zone |
| `sei-49.1` | F_a = 1.5 γ_a α S W_a |  | - | - | 1.5 | en1997-en1998-target.md:476: DE NA: simplified `F_a = 1.5 γ_a α S W_a`? . |

##### Clause 3.2.2.3 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `vertical-spectrum.avg-ratio` | a_vg/a_g |  | 0.9 | 0.9 | 3.2.2.3 |  |

##### Clause 3.2.4 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-11.1` | ψ2 = 0 |  | - | - | 3.2.4 | en1997-en1998-target.md:384: - Edge: more than one variable per storey; roof with snow `ψ2 = 0` (below 1000 m) ; mass eccentricities. |

##### Clause 4.2.3.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-20.1` | r_x >= l_s |  | - | - | 4.2.3.2 | en1997-en1998-target.md:392: NA-DE adds `r_x >= l_s` criterion with tolerance |

##### Clause 4.3.3.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-40.1` | [V] |  | - | - | 4.3.3.2 | en1997-en1998-target.md:426: NA-DE keeps 0.85 |

##### Clause 4.3.3.2.4 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en.torsion.simplified-bound` | δ max |  | 1.6 | 1.6 | 4.3.3.2.4 |  |

##### Clause 4.4.2.7 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-47.1` | d_old = 0.01 H_old |  | - | - | 4.4.2.7 | en1997-en1998-target.md:469: - Edge: existing neighbour of unknown stiffness (assumed displacement `d_old = 0.01 H_old` as an editor default ), multip |

##### Clause 4.4.3.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-45.1` | ν = 0.5 |  | - | - | 4.4.3.2 | en1997-en1998-target.md:456: NA-DE `ν = 0.5` for classes I, II and `0.4` for III, IV (existing field `nu`) |

##### Clause 8.3 (2; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.timber.max-diameter-mm` | d max | mm | 12 | 12 | 8.3 |  |
| `en1.timber.min-thickness-factor` | t/d min |  | 8 | 8 | 8.3 |  |

##### Clause ? (2; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l779.1` | [V] |  | - | - | 8. Test Strategy | en1997-en1998-target.md:779: - Every NDP has a negative control test that fails if the value is changed silently (golden NDP table hash). |
| `l791.1` | [V] |  | - | - | 9. Delivery Order and Definition of Done | en1997-en1998-target.md:791: Done when every catalogue id has implementation, localisation EN/DE, fixture + oracle, viewport, live-recompute target, d |

##### Clause EN 1992-1-1 3.2.7(4) (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en3.annex-a.steel-modulus-mpa` | E_s | MPa | 200000 | 200000 | EN 1992-1-1 3.2.7(4) | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: E_s = 200000.0 as used by the artifact |

##### Clause EN 1997-1 Annex H.2 (2; DIN EN 1997-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sls.angular-distortion-limit` | limit of the angular distortion / relati |  | 0.002 | 0.002 | EN 1997-1 Annex H.2 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: limit of the angular distortion / relative rotation |
| `sls.tilt-limit` | limit of the structure tilt |  | 0.004 | 0.004 | EN 1997-1 Annex H.2 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: limit of the structure tilt = 0.004 as used by the a |

##### Clause EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 (10; DIN EN 1997-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `xi.static.n1.1` | xi1, n = 1 (static load tests) |  | 1.4 | 1.35 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi1, n = 1 (static load tests) = 1.4 as used by the |
| `xi.static.n1.2` | xi2, n = 1 (static load tests) |  | 1.4 | 1.35 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi2, n = 1 (static load tests) = 1.4 as used by the |
| `xi.static.n2.1` | xi1, n = 2 (static load tests) |  | 1.3 | 1.25 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi1, n = 2 (static load tests) = 1.3 as used by the |
| `xi.static.n2.2` | xi2, n = 2 (static load tests) |  | 1.2 | 1.15 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi2, n = 2 (static load tests) = 1.2 as used by the |
| `xi.static.n3.1` | xi1, n = 3 (static load tests) |  | 1.2 | 1.15 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi1, n = 3 (static load tests) = 1.2 as used by the |
| `xi.static.n3.2` | xi2, n = 3 (static load tests) |  | 1.05 | 1.05 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi2, n = 3 (static load tests) = 1.05 as used by the |
| `xi.static.n4.1` | xi1, n = 4 (static load tests) |  | 1.1 | 1.05 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi1, n = 4 (static load tests) = 1.1 as used by the |
| `xi.static.n4.2` | xi2, n = 4 (static load tests) |  | 1 | 1 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi2, n = 4 (static load tests) = 1.0 as used by the |
| `xi.static.n5.1` | xi1, n = >= 5 (static load tests) |  | 1 | 1 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi1, n = >= 5 (static load tests) = 1.0 as used by t |
| `xi.static.n5.2` | xi2, n = >= 5 (static load tests) |  | 1 | 1 | EN 1997-1 Table A.9 / DIN 1054 Table A 7.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi2, n = >= 5 (static load tests) = 1.0 as used by t |

##### Clause EN 1997-1 Table A.10 (14; DIN EN 1997-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `xi.ground.n1.3` | xi3, n = 1 (ground-test profiles) |  | 1.4 | 1.4 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 1 (ground-test profiles) = 1.4 as used by t |
| `xi.ground.n1.4` | xi4, n = 1 (ground-test profiles) |  | 1.4 | 1.4 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 1 (ground-test profiles) = 1.4 as used by t |
| `xi.ground.n10.3` | xi3, n = 10 (ground-test profiles) |  | 1.25 | 1.25 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 10 (ground-test profiles) = 1.25 as used by |
| `xi.ground.n10.4` | xi4, n = 10 (ground-test profiles) |  | 1.08 | 1.08 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 10 (ground-test profiles) = 1.08 as used by |
| `xi.ground.n2.3` | xi3, n = 2 (ground-test profiles) |  | 1.35 | 1.35 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 2 (ground-test profiles) = 1.35 as used by |
| `xi.ground.n2.4` | xi4, n = 2 (ground-test profiles) |  | 1.27 | 1.27 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 2 (ground-test profiles) = 1.27 as used by |
| `xi.ground.n3.3` | xi3, n = 3 (ground-test profiles) |  | 1.33 | 1.33 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 3 (ground-test profiles) = 1.33 as used by |
| `xi.ground.n3.4` | xi4, n = 3 (ground-test profiles) |  | 1.23 | 1.23 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 3 (ground-test profiles) = 1.23 as used by |
| `xi.ground.n4.3` | xi3, n = 4 (ground-test profiles) |  | 1.31 | 1.31 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 4 (ground-test profiles) = 1.31 as used by |
| `xi.ground.n4.4` | xi4, n = 4 (ground-test profiles) |  | 1.2 | 1.2 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 4 (ground-test profiles) = 1.2 as used by t |
| `xi.ground.n5.3` | xi3, n = 5 (ground-test profiles) |  | 1.29 | 1.29 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 5 (ground-test profiles) = 1.29 as used by |
| `xi.ground.n5.4` | xi4, n = 5 (ground-test profiles) |  | 1.15 | 1.15 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 5 (ground-test profiles) = 1.15 as used by |
| `xi.ground.n7.3` | xi3, n = 7 (ground-test profiles) |  | 1.27 | 1.27 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi3, n = 7 (ground-test profiles) = 1.27 as used by |
| `xi.ground.n7.4` | xi4, n = 7 (ground-test profiles) |  | 1.12 | 1.12 | EN 1997-1 Table A.10 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi4, n = 7 (ground-test profiles) = 1.12 as used by |

##### Clause EN 1997-1 Table A.11 (10; DIN EN 1997-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `xi.dynamic.n10.5` | xi5, n = 10 (dynamic tests) |  | 1.45 | 1.45 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi5, n = 10 (dynamic tests) = 1.45 as used by the ar |
| `xi.dynamic.n10.6` | xi6, n = 10 (dynamic tests) |  | 1.3 | 1.3 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi6, n = 10 (dynamic tests) = 1.3 as used by the art |
| `xi.dynamic.n15.5` | xi5, n = 15 (dynamic tests) |  | 1.42 | 1.42 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi5, n = 15 (dynamic tests) = 1.42 as used by the ar |
| `xi.dynamic.n15.6` | xi6, n = 15 (dynamic tests) |  | 1.25 | 1.25 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi6, n = 15 (dynamic tests) = 1.25 as used by the ar |
| `xi.dynamic.n2.5` | xi5, n = 2 (dynamic tests) |  | 1.6 | 1.6 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi5, n = 2 (dynamic tests) = 1.6 as used by the arti |
| `xi.dynamic.n2.6` | xi6, n = 2 (dynamic tests) |  | 1.5 | 1.5 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi6, n = 2 (dynamic tests) = 1.5 as used by the arti |
| `xi.dynamic.n20.5` | xi5, n = 20 (dynamic tests) |  | 1.4 | 1.4 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi5, n = 20 (dynamic tests) = 1.4 as used by the art |
| `xi.dynamic.n20.6` | xi6, n = 20 (dynamic tests) |  | 1.25 | 1.25 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi6, n = 20 (dynamic tests) = 1.25 as used by the ar |
| `xi.dynamic.n5.5` | xi5, n = 5 (dynamic tests) |  | 1.5 | 1.5 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi5, n = 5 (dynamic tests) = 1.5 as used by the arti |
| `xi.dynamic.n5.6` | xi6, n = 5 (dynamic tests) |  | 1.35 | 1.35 | EN 1997-1 Table A.11 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: xi6, n = 5 (dynamic tests) = 1.35 as used by the art |

##### Clause EN 1998-1 3.2.3.1.2 (2; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.time-history.mean-spectrum-ratio` | mean spectrum over elastic spectrum |  | 0.9 | 0.9 | EN 1998-1 3.2.3.1.2 | 📐️specs/en1997-en1998-target.md SEI items: mean spectrum over elastic spectrum = 0.9 as used by the artifact |
| `en1.time-history.min-records` | minimum number of accelerograms |  | 3 | 3 | EN 1998-1 3.2.3.1.2 | 📐️specs/en1997-en1998-target.md SEI items: minimum number of accelerograms = 3.0 as used by the artifact |

##### Clause EN 1998-1 4.4.2.5 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.diaphragm.transfer-factor` | γ_d transfer amplification |  | 1.3 | 1.3 | EN 1998-1 4.4.2.5 | 📐️specs/en1997-en1998-target.md SEI items: γ_d transfer amplification = 1.3 as used by the artifact |

##### Clause EN 1998-1 4.4.3.2 (2; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.importance.nu-i-ii` | ν importance I, II |  | 0.5 | 0.5 | EN 1998-1 4.4.3.2 | 📐️specs/en1997-en1998-target.md SEI items: ν importance I, II = 0.5 as used by the artifact |
| `en1.importance.nu-iii-iv` | ν importance III, IV |  | 0.4 | 0.4 | EN 1998-1 4.4.3.2 | 📐️specs/en1997-en1998-target.md SEI items: ν importance III, IV = 0.4 as used by the artifact |

##### Clause EN 1998-2 5.6.3.3 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en2.pier.gamma-0` | γ_0 |  | 1.35 | 1.35 | EN 1998-2 5.6.3.3 |  |

##### Clause EN 1998-3 A.1 (7; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en3.annex-a.theta-um.axial` | θ_um axial load base |  | 0.3 | 0.3 | EN 1998-3 A.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_um axial load base = 0.3 as used by the artifact |
| `en3.annex-a.theta-um.base` | θ_um base value |  | 0.016 | 0.016 | EN 1998-3 A.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_um base value = 0.016 as used by the artifact |
| `en3.annex-a.theta-um.confinement` | θ_um confinement base |  | 25 | 25 | EN 1998-3 A.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_um confinement base = 25.0 as used by the artifact |
| `en3.annex-a.theta-um.diagonal` | θ_um diagonal reinforcement base |  | 1.25 | 1.25 | EN 1998-3 A.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_um diagonal reinforcement base = 1.25 as used by t |
| `en3.annex-a.theta-um.reinforcement` | θ_um reinforcement exponent |  | 0.225 | 0.225 | EN 1998-3 A.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_um reinforcement exponent = 0.225 as used by the a |
| `en3.annex-a.theta-um.shear-span` | θ_um shear span exponent |  | 0.35 | 0.35 | EN 1998-3 A.1 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_um shear span exponent = 0.35 as used by the artif |
| `en3.chord-rotation.sd-factor` | SD factor |  | 0.75 | 0.75 | EN 1998-3 A.1 |  |

##### Clause EN 1998-3 A.10b (3; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en3.annex-a.theta-y.bar-slip` | θ_y bar slip factor |  | 0.13 | 0.13 | EN 1998-3 A.10b | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_y bar slip factor = 0.13 as used by the artifact |
| `en3.annex-a.theta-y.base` | θ_y constant term |  | 0.0013 | 0.0013 | EN 1998-3 A.10b | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_y constant term = 0.0013 as used by the artifact |
| `en3.annex-a.theta-y.slenderness` | θ_y h/L_V factor |  | 1.5 | 1.5 | EN 1998-3 A.10b | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: θ_y h/L_V factor = 1.5 as used by the artifact |

##### Clause EN 1998-5 4.1.4 (3; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en5.liquefaction.min-safety-factor` | F_L min |  | 1.25 | 1.25 | EN 1998-5 4.1.4 |  |
| `en5.liquefaction.screen-ag-s-g` | a_g S screening limit in g |  | 0.1 | 0.15 | EN 1998-5 4.1.4 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: a_g S screening limit in g = 0.1 as used by the arti |
| `en5.liquefaction.screen-depth-m` | screening depth | m | 15 | 15 | EN 1998-5 4.1.4 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: screening depth = 15.0 as used by the artifact |

##### Clause EN 1998-5 5.4.2 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en5.pile.stiffness-contrast` | G_2/G_1 for kinematic interaction |  | 6 | 6 | EN 1998-5 5.4.2 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: G_2/G_1 for kinematic interaction = 6.0 as used by t |

##### Clause EN 1998-5 Annex B (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en5.liquefaction.annex-b.csr-factor` | τ_e factor |  | 0.65 | 0.65 | EN 1998-5 Annex B | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: τ_e factor = 0.65 as used by the artifact |

##### Clause EN 1998-5 Annex B (chart), Youd et al. 2001 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `ref.liquefaction.youd-2001` | reference model |  | r_d, CRR 7.5, MSF, fines correction | r_d, CRR 7.5, MSF, fines correction | EN 1998-5 Annex B (chart), Youd et al. 2001 |  |

##### Clause EN 1998-5 Annex F (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en5.annex-f.gamma-rd` | γ_Rd |  | 1 | 1 | EN 1998-5 Annex F | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: γ_Rd = 1.0 as used by the artifact |

##### Clause EN 1998-5 Table F.1 (15; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en5.annex-f.sand.a` | a |  | 0.92 | 0.92 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.b` | b |  | 1.25 | 1.25 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.beta` | β |  | 2.57 | 2.57 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.c` | c |  | 0.92 | 0.92 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.c-m` | c_M |  | 2.57 | 2.57 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.c-m-prime` | c'_M |  | 1.85 | 1.85 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.c-t` | c_T |  | 2.14 | 2.14 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.c-t-prime` | c'_T |  | 1.81 | 1.81 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.d` | d |  | 1.25 | 1.25 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.e` | e |  | 0.92 | 0.92 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.f` | f |  | 0.41 | 0.41 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.gamma` | γ |  | 4 | 4 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.k` | k |  | 0.39 | 0.39 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.k-prime` | k' |  | 0.39 | 0.39 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |
| `en5.annex-f.sand.m` | m |  | 0.96 | 0.96 | EN 1998-5 Table F.1 | 📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2 |

##### Clause Nikolaou et al. 2001 (4; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `ref.pile.nikolaou-2001.coefficient` | M = c τ_c d³ ... |  | 0.042 | 0.042 | Nikolaou et al. 2001 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: M = c τ_c d³ ... = 0.042 as used by the artifact |
| `ref.pile.nikolaou-2001.length-exponent` | (L/d)^p |  | 0.3 | 0.3 | Nikolaou et al. 2001 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: (L/d)^p = 0.3 as used by the artifact |
| `ref.pile.nikolaou-2001.modulus-exponent` | (E_p/E_1)^q |  | 0.65 | 0.65 | Nikolaou et al. 2001 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: (E_p/E_1)^q = 0.65 as used by the artifact |
| `ref.pile.nikolaou-2001.velocity-exponent` | (v_s2/v_s1)^r |  | 0.5 | 0.5 | Nikolaou et al. 2001 | wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition: (v_s2/v_s1)^r = 0.5 as used by the artifact |

##### Clause Table 8.1 (1; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `sei-30.2` | 2.0 |  | - | - | Table 8.1 | en1997-en1998-target.md:407: - Timber (§8.3 Table 8.1): DCL 1.5, DCM 2.0 (nailed walls `2.0`), DCH 3.0 (moment-resisting frames with ductile connector |

##### Clause Table 9.2 (2; DIN EN 1998-1/NA:2023-11)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `en1.masonry.min-thickness-m` | t_ef min | m | 0.24 | 0.24 | Table 9.2 |  |
| `en1.masonry.slenderness-max` | h_ef/t_ef max |  | 12 | 12 | Table 9.2 |  |

### EN 1999 (129)

#### EN 1999, part 1-1 (36)

##### Clause 6.1.4.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-06.1` | ? |  | - | - | 6.1.4.3 | en1994-en1999-target.md:314: **A-06 Distortional/stiffened elements ** — cl. 6.1.4.3–6.1.5.3 (edge/intermediate stiffeners, effective thickness of sti |

##### Clause 6.7.5 (transverse load on the web) (5; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `transverse-load.chi-const` | χ_F constant |  | 0.5 | 0.5 | 6.7.5 (transverse load on the web) |  |
| `transverse-load.fcr-factor` | F_cr factor |  | 0.9 | 0.9 | 6.7.5 (transverse load on the web) |  |
| `transverse-load.kf-base` | k_F,0 |  | 6 | 6 | 6.7.5 (transverse load on the web) |  |
| `transverse-load.kf-slope` | k_F,1 |  | 2 | 2 | 6.7.5 (transverse load on the web) |  |
| `transverse-load.m2-factor` | m_2 factor |  | 0.02 | 0.02 | 6.7.5 (transverse load on the web) |  |

##### Clause EN 1990 Table A1.2(B) (3; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-g` | gamma_g |  | 1.35 | 1.35 | EN 1990 Table A1.2(B) |  |
| `gamma-q` | gamma_q |  | 1.5 | 1.5 | EN 1990 Table A1.2(B) |  |
| `xi` | xi |  | 0.85 | 0.85 | EN 1990 Table A1.2(B) |  |

##### Clause EN 1991-1-2 §3.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire-alpha-c` | fire_alpha_c |  | 25 | 25 | EN 1991-1-2 §3.2 |  |

##### Clause EN 1999-1-1 Table 6.4 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `alpha-max` | alpha_max |  | 1.25 | 1.25 | EN 1999-1-1 Table 6.4 |  |

##### Clause EN 1999-1-1 §6.1.3 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-m1` | gamma_m1 |  | 1.1 | 1.1 | EN 1999-1-1 §6.1.3 |  |
| `gamma-m2` | gamma_m2 |  | 1.25 | 1.25 | EN 1999-1-1 §6.1.3 |  |

##### Clause EN 1999-1-1 §6.1.4.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `edge-stiffener-min-depth-ratio` | edge_stiffener_min_depth_ratio |  | 5 | 5 | EN 1999-1-1 §6.1.4.3 |  |

##### Clause EN 1999-1-1 §6.1.6.3 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `haz-t-junction-factor` | haz_t_junction_factor |  | 1.33 | 1.33 | EN 1999-1-1 §6.1.6.3 |  |
| `haz-tig-factor` | haz_tig_factor |  | 1.5 | 1.5 | EN 1999-1-1 §6.1.6.3 |  |

##### Clause EN 1999-1-1 §6.2.9.1 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi-section-i` | psi_section_i |  | 2 | 2 | EN 1999-1-1 §6.2.9.1 |  |
| `psi-section-tube` | psi_section_tube |  | 1.5 | 1.5 | EN 1999-1-1 §6.2.9.1 |  |

##### Clause EN 1999-1-1 §6.3.1.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `kappa-end-lambda` | kappa_end_lambda |  | 1.2 | 1.2 | EN 1999-1-1 §6.3.1.1 |  |

##### Clause EN 1999-1-1 §6.3.3.1 (3; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `eta-0` | eta_0 |  | 0.8 | 0.8 | EN 1999-1-1 §6.3.3.1 |  |
| `gamma-0` | gamma_0 |  | 0.8 | 0.8 | EN 1999-1-1 §6.3.3.1 |  |
| `xi-0` | xi_0 |  | 0.8 | 0.8 | EN 1999-1-1 §6.3.3.1 |  |

##### Clause EN 1999-1-1 §6.3.3.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `omega-x-exponent` | omega_x_exponent |  | 1 | 1 | EN 1999-1-1 §6.3.3.3 |  |

##### Clause EN 1999-1-1 §6.7.4 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `shear-buckling-k-tau` | shear_buckling_k_tau |  | 5.34 | 5.34 | EN 1999-1-1 §6.7.4 |  |

##### Clause EN 1999-1-1 §6.7.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `patch-load-k-f` | patch_load_k_f |  | 6 | 6 | EN 1999-1-1 §6.7.5 |  |

##### Clause EN 1999-1-1 §7.2.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `deflection-ratio` | deflection_ratio |  | 200 | 200 | EN 1999-1-1 §7.2.1 |  |

##### Clause EN 1999-1-1 §8.1.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-mw` | gamma_mw |  | 1.25 | 1.25 | EN 1999-1-1 §8.1.1 |  |

##### Clause EN 1999-1-2 §2.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-m-fi` | gamma_m_fi |  | 1 | 1 | EN 1999-1-2 §2.3 |  |

##### Clause EN 1999-1-2 §4.2.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `fire-emissivity` | fire_emissivity |  | 0.3 | 0.3 | EN 1999-1-2 §4.2.5 |  |

##### Clause EN 1999-1-3 Table 2.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-mf` | gamma_mf |  | 1 | 1.35 | EN 1999-1-3 Table 2.1 |  |

##### Clause EN 1999-1-3 §2.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `gamma-ff` | gamma_ff |  | 1 | 1 | EN 1999-1-3 §2.2 |  |

##### Clause EN 1999-1-5 §5.3 (4; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `shell-alpha-hoop-shear` | shell_alpha_hoop_shear |  | 0.65 | 0.65 | EN 1999-1-5 §5.3 |  |
| `shell-quality-q` | shell_quality_q |  | 25 | 25 | EN 1999-1-5 §5.3 |  |
| `shell-quality-q-class-a` | shell_quality_q_class_a |  | 40 | 40 | EN 1999-1-5 §5.3 |  |
| `shell-quality-q-class-c` | shell_quality_q_class_c |  | 16 | 16 | EN 1999-1-5 §5.3 |  |

##### Clause project limit (not in EN 1999-1-1 §7.2) (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `stress-ratio` | stress_ratio |  | 0.8 | 0.8 | project limit (not in EN 1999-1-1 §7.2) |  |

#### EN 1999, part 1-2 (6)

##### Clause 2.7 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-50.1` | ? |  | - | - | 2.7 | en1994-en1999-target.md:406: per alloy group) |
| `a-50.2` | ? |  | - | - | 2.7 | en1994-en1999-target.md:406: λa = 0.07 θa + 190 (3xxx/5xxx) or 0.07 θa + (6xxx) |

##### Clause 4.2.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-52.1` | ? |  | - | - | 4.2.1 | en1994-en1999-target.md:409: member buckling at fire: χfi with λ̄θ = λ̄ sqrt(k0.2proof,θ/kE,θ) |

##### Clause 4.2.5 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-51.1` | ? |  | - | - | 4.2.5 | en1994-en1999-target.md:407: ksh = 1.0 for aluminium (conservative) |
| `a-51.2` | ? |  | - | - | 4.2.5 | en1994-en1999-target.md:407: protected members (insulation) formula with φ = (cp ρp/ca ρa) dp Ap/V (EN 1999-1-2 4.3.4) . |

##### Clause Table 3.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-50.3` | ? |  | - | - | Table 3.1 | en1994-en1999-target.md:406: strength reduction k0.2proof,θ and ku,θ per alloy and θ (Table 3.1, 3.2) . |

#### EN 1999, part 1-3 (2)

##### Clause 3.4 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-40.1` | ? |  | - | - | 3.4 | en1994-en1999-target.md:397: N Δσ^{m2} = ND ΔσD^{m2} for ΔσL <= Δσ < ΔσD, m2 = m1 + 2 (stated ) |

##### Clause 6.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-41.1` | ? |  | - | - | 6.2 | en1994-en1999-target.md:401: equivalent constant amplitude range Δσ,E,2 = λ × Δσ(Qk) (bridges, cranes) |

#### EN 1999, part 1-4 (14)

##### Clause 2.8 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-62.1` | ? |  | - | - | 2.8 | en1994-en1999-target.md:415: **A-62 Purlins/sheeting: SLS ** — deflection with effective I_eff(σ) and local ponding |

##### Clause 5.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-60.1` | ? |  | - | - | 5.1 | en1994-en1999-target.md:413: **A-60 Effective cross-section and stiffened elements ** — 1-4:5.1–5.5: sheeting with thickness 0.5..6 mm |

##### Clause 6.1 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-61.1` | ? |  | - | - | 6.1 | en1994-en1999-target.md:414: **A-61 Resistance: bending, web crippling, shear ** — 1-4:6.1 etc. |
| `a-61.2` | ? |  | - | - | 6.1 | en1994-en1999-target.md:414: shear VRd = hw/sinφ tw fv/γM1 with fv = 0.9? 0.58 fo(1 − 0.1...) per λw . |

##### Clause 6.1.7 (web crippling) (10; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `web-crippling.end.alpha` | α |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.end.c-a` | c_a |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.end.c-l` | c_l |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.end.c-phi` | c_φ |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.end.c-r` | c_r |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.interior.alpha` | α |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.interior.c-a` | c_a |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.interior.c-l` | c_l |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.interior.c-phi` | c_φ |  | - | - | 6.1.7 (web crippling) |  |
| `web-crippling.interior.c-r` | c_r |  | - | - | 6.1.7 (web crippling) |  |

#### EN 1999, part 1-5 (5)

##### Clause 2.9 (3; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-70.1` | ? |  | - | - | 2.9 | en1994-en1999-target.md:419: χx = α/λ̄x² for λ̄ >= λ̄p, with imperfection parameter α (fabrication quality class A/B/C) |
| `a-71.1` | ? |  | - | - | 2.9 | en1994-en1999-target.md:421: **A-71 Shell geometry and stiffeners ** — 1-5:5 (stiffened shell: ring stiffeners, orthotropic smearing), 1-5:9 (shell wi |
| `a-72.1` | ? |  | - | - | 2.9 | en1994-en1999-target.md:422: **A-72 Welded shells with HAZ ** — 1-5 with ρo,haz of A-03 on axial compression resistance (reduction of fo by HAZ for fa |

##### Clause 5 (ring stiffener stiffness) (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `ring-stiffener.required-inertia-factor` | I_r,min factor |  | - | - | 5 (ring stiffener stiffness) |  |

##### Clause 5 (ring stiffener, effective shell width) (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `ring-stiffener.effective-width-factor` | b_e/√(r·t) |  | 0.78 | 0.78 | 5 (ring stiffener, effective shell width) |  |

#### EN 1999, part not stated (66)

##### Clause 1.0 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-30.2` | ? |  | - | - | 1.0 | en1994-en1999-target.md:378: with countersunk bolts reduced (cl. 8.5.5 ). |

##### Clause 1.1 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l454.1` | ? |  | - | - | 1.1 | en1994-en1999-target.md:454: \| 1999-1-4 \| γM1, γM2 \| 1.1/1.25 \| \| |
| `l455.1` | ? |  | - | - | 1.1 | en1994-en1999-target.md:455: \| 1999-1-5 \| γM0, γM1 \| 1.1 \| \| |

##### Clause 1.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l448.1` | ? |  | - | - | 1.5 | en1994-en1999-target.md:448: \| 1994-1-1 9.? \| sheeting construction load 1.5 kN/m² \| 1.5 \| \| |

##### Clause 1.25 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-32.2` | ? |  | - | - | 1.25 | en1994-en1999-target.md:388: list of fillers per DIN EN 1011-4 [N: ]. |

##### Clause 2.1 (7; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-01.2` | ? |  | - | - | 2.1 | en1994-en1999-target.md:289: t 3–25: 160/195? |
| `a-01.3` | ? |  | - | - | 2.1 | en1994-en1999-target.md:290: - EN AW-6061 T6: fo 240, fu 260, BC A, ρo,haz 0.48, ρu,haz 0.67 . |
| `a-01.4` | ? |  | - | - | 2.1 | en1994-en1999-target.md:291: - EN AW-5083 H12/O (sheet/plate), 6005A T6, 5754, 3005, etc. . |
| `a-02.1` | ? |  | - | - | 2.1 | en1994-en1999-target.md:296: TIG: bhaz = factor larger |
| `a-02.2` | ? |  | - | - | 2.1 | en1994-en1999-target.md:296: interpass temperature > 60 °C (6xxx) or > 120 °C (3xxx/5xxx): bhaz × increase (bhaz,eff = bhaz [1 + (Tint − 60)/... ]) |
| `a-04.1` | ? |  | - | - | 2.1 | en1994-en1999-target.md:303: outstands: β = b/t when tip in compression ... or η-based with ψ per Table |
| `a-05.2` | ? |  | - | - | 2.1 | en1994-en1999-target.md:309: If element contains HAZ the combined reduction: teff = min(ρc, ρo,haz) t in HAZ region (cl. 6.1.5.? ) . |

##### Clause 2.3.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-81.1` | ? |  | - | - | 2.3.5 | en1994-en1999-target.md:427: **A-81 Temperature and creep ** — cl. 2.3.5.? thermal expansion α = 23e-6 |

##### Clause 6.1.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-05.1` | ? |  | - | - | 6.1.5 | en1994-en1999-target.md:308: **A-05 Local buckling: effective thickness, ρc ** — cl. 6.1.5. |

##### Clause 6.1.6.2 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-03.1` | ? |  | - | - | 6.1.6.2 | en1994-en1999-target.md:300: **A-03 HAZ strength reduction ** — cl. 6.1.6.2, 6.1.6.3: fo,haz = ρo,haz fo |
| `a-03.2` | ? |  | - | - | 6.1.6.2 | en1994-en1999-target.md:300: for 7xxx (after natural ageing) with . |

##### Clause 6.2.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-10.1` | ? |  | - | - | 6.2.1 | en1994-en1999-target.md:318: **A-10 General yielding vs. ultimate ** — cl. 6.2.1: Fed check design: resistances with fo/γM1 for general yielding and f |

##### Clause 6.2.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-11.1` | ? |  | - | - | 6.2.3 | en1994-en1999-target.md:320: **A-11 Tension, gross section ** — cl. 6.2.3. |

##### Clause 6.2.7 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-16.1` | ? |  | - | - | 6.2.7 | en1994-en1999-target.md:339: with combined: 6.2.7.2 interaction of shear+torsion: VEd <= VRd reduced by (1 − τt,Ed√3γM1/fo) ... . |

##### Clause 6.2.8 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-18.1` | ? |  | - | - | 6.2.8 | en1994-en1999-target.md:343: **A-18 Combined shear/bending/axial ** — cl. 6.2.8: if VEd <= 0.5VRd ignore |

##### Clause 6.2.9 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-17.1` | ? |  | - | - | 6.2.9 | en1994-en1999-target.md:341: **A-17 Combined bending and axial ** — cl. 6.2.9. |
| `a-17.2` | ? |  | - | - | 6.2.9 | en1994-en1999-target.md:341: for I sections ψ=1, η=... ) |

##### Clause 6.2.11 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-19.1` | ? |  | - | - | 6.2.11 | en1994-en1999-target.md:345: **A-19 Holes/openings and local loads ** — cl. 6.2.11–6.2.12: web bearing (cl. 6.7.5 'patch loading' for I-sections, FRd |
| `a-19.2` | ? |  | - | - | 6.2.11 | en1994-en1999-target.md:345: Cross-check with 1-5? Provide formulas as . |

##### Clause 6.3.1.1 (3; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-21.1` | ? |  | - | - | 6.3.1.1 | en1994-en1999-target.md:355: **A-21 Welded members: HAZ reduction factors κ, ωx (ηc not in this clause) ** — cl. 6.3.1.1(?) and 6.3.3.3. |
| `a-21.2` | ? |  | - | - | 6.3.1.1 | en1994-en1999-target.md:355: Table gives κ for non-welded 1.0 and for longitudinally welded: κ = 1 − (1 − κ0)... where κ0 = ρo,haz? |
| `a-21.3` | ? |  | - | - | 6.3.1.1 | en1994-en1999-target.md:355: localized transverse weld: ω_x = 1/(χ + (1 − χ) sin(π xs/lc)^c) <= 1 with xs distance of weld to midspan, c depends |

##### Clause 6.3.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-23.1` | ? |  | - | - | 6.3.3 | en1994-en1999-target.md:362: **A-23 Interaction: axial compression plus bending ** — cl. 6.3.3 (Method with η0, γ0, ξ0 exponents |

##### Clause 6.3.3.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-24.1` | ? |  | - | - | 6.3.3.3 | en1994-en1999-target.md:365: **A-24 Members with local transverse HAZ (ωx) and torsional buckling ** — cl. 6.3.3.3, Annex? cross-ref A-21. |

##### Clause 6.6.4.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l441.1` | ? |  | - | - | 6.6.4.2 | en1994-en1999-target.md:441: (additional limit) \| |

##### Clause 6.6.6.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l442.1` | ? |  | - | - | 6.6.6.3 | en1994-en1999-target.md:442: \| 1994-1-1 6.6.6.3 \| min transverse reinforcement \| 0.002 \| \| |

##### Clause 6.7.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-26.1` | ? |  | - | - | 6.7.2 | en1994-en1999-target.md:370: **A-26 Plate girders: bending with web slenderness ** — cl. 6.7.2–6.7.3: reduced section modulus Weff via ρc on web & fla |

##### Clause 6.7.3.3 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l444.1` | ? |  | - | - | 6.7.3.3 | en1994-en1999-target.md:444: \| 1994-1-1 6.7.3.3 \| Ke \| 0.6 \| \| |

##### Clause 6.7.3.4 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l443.1` | ? |  | - | - | 6.7.3.4 | en1994-en1999-target.md:443: \| 1994-1-1 6.7.3.4 \| K0, Ke,II \| 0.9, 0.5 \| \| |

##### Clause 6.7.3.6 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l445.1` | ? |  | - | - | 6.7.3.6 | en1994-en1999-target.md:445: \| 1994-1-1 6.7.3.6 \| αM \| 0.9/0.8 \| \| |

##### Clause 6.7.4 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-25.1` | ? |  | - | - | 6.7.4 | en1994-en1999-target.md:369: **A-25 Plate girders: shear buckling ** — cl. 6.7.4 (web of plate girders). |
| `a-25.2` | ? |  | - | - | 6.7.4 | en1994-en1999-target.md:369: Table values . |

##### Clause 6.7.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-27.1` | ? |  | - | - | 6.7.5 | en1994-en1999-target.md:371: **A-27 Patch loading/web bearing ** — cl. 6.7.5. |

##### Clause 6.7.6 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-28.1` | ? |  | - | - | 6.7.6 | en1994-en1999-target.md:372: **A-28 Stiffeners ** — cl. 6.7.6 (transverse stiffener checks, flexural), 6.7.7 (longitudinal stiffeners). |

##### Clause 6.8.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l449.1` | ? |  | - | - | 6.8.2 | en1994-en1999-target.md:449: \| 1994-2 6.8.2 \| γMf, γFf \| 1.0, 1.0 \| \| |

##### Clause 7.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-80.1` | ? |  | - | - | 7.2 | en1994-en1999-target.md:426: **A-80 Deflection [V/T]** — cl. 7.2: elastic EI (using Ieff when class 4, bf by effective width), limits L/250..L/360 by |

##### Clause 7.2.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l446.1` | ? |  | - | - | 7.2.2 | en1994-en1999-target.md:446: \| 1994-1-1 7.2.2 \| stress limits \| 0.6 fck / 0.8 fsk \| \| |

##### Clause 7.3.1 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l447.1` | ? |  | - | - | 7.3.1 | en1994-en1999-target.md:447: \| 1994-1-1 7.3.1 \| deflection limits \| L/250 \| \| |

##### Clause 8.2 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `l452.1` | ? |  | - | - | 8.2 | en1994-en1999-target.md:452: \| 1999-1-1 8.2 \| bolt/rivet material \| per Table \| \| |

##### Clause 8.5 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-31.1` | ? |  | - | - | 8.5 | en1994-en1999-target.md:384: **A-31 Rivets ** — cl. 8.5.? solid and blind rivets: Fv,Rd = 0.6 fur A0/γM2 for 6082 rivets (fur 160 ) |

##### Clause 8.5.2.3 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-12.1` | ? |  | - | - | 8.5.2.3 | en1994-en1999-target.md:325: angles connected through one leg (cl. 8.5.2.3 ) |
| `a-30.3` | ? |  | - | - | 8.5.2.3 | en1994-en1999-target.md:381: - Block tearing (8.5.2.3) , group effects, slip-resistant bolting (prestressed) . |

##### Clause 8.5.10 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-33.1` | ? |  | - | - | 8.5.10 | en1994-en1999-target.md:392: **A-33 Hybrid/other connectors ** — cl. 8.5.10 adhesives, 8.9 pin joints, 8.8 lug |

##### Clause 8.8 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-30.1` | ? |  | - | - | 8.8 | en1994-en1999-target.md:377: 0.6 fub A/γM2 (shank), classes 4.6/5.6/8.8/10.9, stainless A2-70/A4-70, aluminium bolts (e.g. 6082-T6: fub = 310 ). |

##### Clause ? (6; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `e2-10.1` | ? |  | - | - | 8. Worked Examples EN 1999 (hand-checked) | en1994-en1999-target.md:671: Δθa = 1.0·200/(911.2·2700)·2 105.3·5 = **0.856 K** (ksh = 1.0 ). |
| `e2-11.1` | ? |  | - | - | 8. Worked Examples EN 1999 (hand-checked) | en1994-en1999-target.md:673: the χx evaluation requires the transcribed α, λ̄x0, β, η for fabrication class . |
| `e2-12.1` | ? |  | - | - | 8. Worked Examples EN 1999 (hand-checked) | en1994-en1999-target.md:675: **E2-12 HAZ extent and effective area (A-02, A-13)** — plate 120×8 (6082-T6, 5<t<=15, fo = 260) with one centred longitud |
| `e2-12.2` | ? |  | - | - | 8. Worked Examples EN 1999 (hand-checked) | en1994-en1999-target.md:675: T-junction variant: bhaz × 1.33 = 39.9 mm (Ahaz = 638.4 mm²), Aeff = 960 − 638.4·0.52 = 628.0 mm², NRd = 148.4 kN . |
| `l453.1` | ? |  | - | - | 3. NDP Registry (selection) | en1994-en1999-target.md:453: \| 1999-1-3 \| γMf \| table \| \| |
| `l693.1` | [T] |  | - | - | 11. Open Items Requiring Norm-Text Transcription ( | en1994-en1999-target.md:693: Open Items Requiring Norm-Text Transcription ( summary) |

##### Clause EN 1990 Table A1.1 (6; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `psi-default.0` | ψ0 default |  | 0.4 | 0.4 | EN 1990 Table A1.1 |  |
| `psi-default.1` | ψ1 default |  | 0.3 | 0.3 | EN 1990 Table A1.1 |  |
| `psi-default.2` | ψ2 default |  | 0.2 | 0.2 | EN 1990 Table A1.1 |  |
| `psi-fire.0` | ψ0 fire |  | 0 | 0 | EN 1990 Table A1.1 |  |
| `psi-fire.1` | ψ1 fire |  | 0 | 0 | EN 1990 Table A1.1 |  |
| `psi-fire.2` | ψ2 fire |  | 0 | 0 | EN 1990 Table A1.1 |  |

##### Clause Table 3.2a (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-01.1` | ? |  | - | - | Table 3.2a | en1994-en1999-target.md:284: **A-01 Alloy/temper data ** — cl. 3.2, Table 3.2a (wrought: sheet/strip/plate), 3.2b (extruded), 3.2c (drawn tube), 3.2d |

##### Clause Table 6. (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-23.2` | ? |  | - | - | Table 6. | en1994-en1999-target.md:362: ηc = η0 χz^?, γc = γ0 , ξc = ξ0 χz ... with η0, γ0, ξ0 in Table 6.? for Class 1/2 (and 1.0 for class 3/4) |

##### Clause Table 6.2 (2; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-04.2` | ? |  | - | - | Table 6.2 | en1994-en1999-target.md:304: intermediate stiffeners 6.1.5) . |
| `a-04.3` | ? |  | - | - | Table 6.2 | en1994-en1999-target.md:304: Round tubes: D/t limits with parameter (3 ε², ... ) . |

##### Clause Table 8.8 (1; DIN EN 1999-1-1/NA)

| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |
|---|---|---|---|---|---|---|
| `a-32.1` | ? |  | - | - | Table 8.8 | en1994-en1999-target.md:387: - Fillet weld resistance: directional method (cl. 8.6.3.3): √(σ⊥² + 3(τ⊥² + τ∥²)) <= fw/(γMw?) hmm: aluminium: Fw,Rd = fw |

## 3. Marked verified in the registry (re-check)

These rows were flagged verified in the code before the registry existed (EN 1995 strength, k_mod and k_def rows) or by a later reviewer; they gate nothing wrongly only while the numbers match the print. `review-en1995-en1996.md` doubts the C16 and C30 numbers, check those first.

- `ndp.en1995.kdef.glulam` (3 cells)
- `ndp.en1995.kdef.lvl` (3 cells)
- `ndp.en1995.kdef.osb2` (3 cells)
- `ndp.en1995.kdef.osb3` (3 cells)
- `ndp.en1995.kdef.osb4` (3 cells)
- `ndp.en1995.kdef.plywood-636-1` (3 cells)
- `ndp.en1995.kdef.plywood-636-2` (3 cells)
- `ndp.en1995.kdef.plywood-636-3` (3 cells)
- `ndp.en1995.kdef.solid` (3 cells)
- `ndp.en1995.kmod.glulam.sc1` (5 cells)
- `ndp.en1995.kmod.glulam.sc2` (5 cells)
- `ndp.en1995.kmod.glulam.sc3` (5 cells)
- `ndp.en1995.kmod.lvl.sc1` (5 cells)
- `ndp.en1995.kmod.lvl.sc2` (5 cells)
- `ndp.en1995.kmod.lvl.sc3` (5 cells)
- `ndp.en1995.kmod.osb3.sc1` (5 cells)
- `ndp.en1995.kmod.osb3.sc2` (5 cells)
- `ndp.en1995.kmod.osb3.sc3` (5 cells)
- `ndp.en1995.kmod.osb4.sc1` (5 cells)
- `ndp.en1995.kmod.osb4.sc2` (5 cells)
- `ndp.en1995.kmod.osb4.sc3` (5 cells)
- `ndp.en1995.kmod.plywood.sc1` (5 cells)
- `ndp.en1995.kmod.plywood.sc2` (5 cells)
- `ndp.en1995.kmod.plywood.sc3` (5 cells)
- `ndp.en1995.kmod.solid.sc1` (5 cells)
- `ndp.en1995.kmod.solid.sc2` (5 cells)
- `ndp.en1995.kmod.solid.sc3` (5 cells)
- `ndp.en1995.strength.c16` (12 cells)
- `ndp.en1995.strength.c24` (12 cells)
- `ndp.en1995.strength.c30` (12 cells)
- `ndp.en1995.strength.gl24h` (12 cells)
- `ndp.en1995.strength.gl28h` (12 cells)
- `ndp.en1995.strength.gl32h` (12 cells)
