# W3-LITERALS: Normative literals hard-coded in check, inference and kernel code

Scope: non-test Rust under `🗿️artifacts/{⚖️en1990,🏋️en1991,🏛️en1992,🔩️en1993,🧩️en1994,🪵️en1995,🪨️en1996,🌍️en1997,🫨️en1998,🪶️en1999}` (schema inferences, checks, kernel, masonry, catalogues; scene, editor, viewer and io excluded as UI) plus the shared `🧮️mechanics` and `⚖️compliance` crates. Root for relative paths below: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/`. Registry checked: `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (3754 rows).

Method. A heuristic scanner (`🗑️generated/w3-literals/scan.py`, output `candidates.tsv`) lexes every in-scope line, drops string contents and `//` comments, and scores each numeric literal: +3 normative identifier (gamma, psi, xi, alpha, k_, factor, limit, chase, annex, ndp, …), +3 clause citation within 6 lines, +2 non-benign decimal, +2 divisor from {100…800}; -4 for UI or geometry paths and names. Scanner output was then read against the source for every item below. Registry matching is by value and clause token, and it is loose: the "value-only" hits in the TSV are mostly false positives and were not counted as registry coverage. Counts are from score ≥ 8 (302 unique file:line sites); score ≥ 6 gives about 1,400 sites. Tables of standard constants inside formulas are listed as Low unless the value itself is an NDP.

Status of the chase-limit finding from the brief. `chase_limits` (0.115 m, 0.030 m) no longer hard-codes anything. It reads `ndp.en1996.chase.*` (for example `ndp.en1996.chase.band.b1.t-upper` = 0.115, `ndp.en1996.chase.v.b1.depth` = 0.03), via `🪨️en1996/…/🧬️schema/📚️masonry-tables/🕳️chase-tables/🦀️.rs`. Resolved. Only the cover and f_vlt items below remain for en1996.

## Ranked findings

Safety relevance: H = changes a utilisation or a pass/fail outcome, with no registry row; M = normative constant that should be a registry row or a documented table; L = fixed EN formula constant or a value that is already correct.

| # | Rank | File (under root) | Line | Literal | Likely meaning | Clause | Registry row |
|---|---|---|---|---|---|---|---|
| 1 | H | `🗿️artifacts/🏛️en1992/…/📋️members/🏗️resistance-and-serviceability-checks/🦀️.rs` | 371 | `/ 250.0` (and `2.0 *` for cantilever) | Deflection limit span/250 | EN 1992-1-1 7.4.1 / 7.4.3 | none (no deflection-limit row in JSON) |
| 2 | H | `🗿️artifacts/🏛️en1992/…/🪟️serviceability/🦀️.rs` | 321 | `l / 250.0`, `l / 500.0` | Deflection limit l/250 and l/500 after construction | EN 1992-1-1 7.4.1 | none |
| 3 | H | `🗿️artifacts/🧩️en1994/…/🧬️schema/💡️inferences/🧮️compliance/🧱️slabs/🦀️.rs` | 102 | `span_m / 250.0`, `0.0054`, `5.0 / 384.0` | Composite slab deflection limit and elastic formula | EN 1994-1-1 7.4 | none |
| 4 | H | `🧮️mechanics/🔩️fastening/🦀️.rs` | 166–168 | `SteelPartialFactors::EN_1992_4` = `{ slope_n 1.2, min_n 1.4, slope_v 1.0, min_v 1.25, other 1.5, ratio_max 0.8, f_uk_max 800 }` | Partial factors γ_Ms for anchor tension and shear, hard-coded in a shared crate; its own doc comment says "unverified" | EN 1992-4 Table 4.1 | partial only: `ndp.en1992.gamma-ms-re` = 1.15, `ndp.en1992.gamma-ms-fat` = 1.35; no row for 1.4, 1.25 or 1.5 |
| 5 | H | `🧮️mechanics/🔩️fastening/🦀️.rs` | 104 | `(0.5 + h_ef / 200.0).min(1.0)` | Dense-reinforcement or edge-distance factor in anchor resistance | EN 1992-4 7.2.1.4 (to verify) | none |
| 6 | H | `🗿️artifacts/🪨️en1996/…/📚️masonry-tables/📚️normative-tables/🦀️.rs` | 125–134 | `0.065`, `0.45`, `0.032 / 0.026 / 0.020` (per unit group) | Upper limit f_vlt of the shear strength, EN branch and NA-DE branch | EN 1996-1-1 3.6.2 / 6.1.2.3 (NA-DE value per the comment) | `ndp.en1996.m-03.2` and `ndp.en1996.m03.fvlt-de` exist with value None; the constants live in code |
| 7 | H | `🗿️artifacts/🪨️en1996/…/🧬️schema/📇️masonry-catalogue/🌦️serviceability-simplified-and-fire/🦀️.rs` | 44–48 | `0.020, 0.025, 0.030, 0.040` (minimum cover per exposure Mx1 … Mx5) | Minimum reinforcement cover by exposure class | EN 1996-2 Table NA / EN 1996-1-1 exposure table (to verify) | `ndp.en1996.m60.exposure` = None; no cover rows |
| 8 | H | `🗿️artifacts/🏛️en1992/…/✂️shear-torsion/🦀️.rs` | 79 (`cot_theta_max` DE branch) | `ndp(vrdcc-c) * 0.48 * fck^(1/3)` | V_Rd,cc in the DE branch. A registry row `ndp.en1992.vrdcc-c` exists with naDe = 0.5, and the code also multiplies by a literal 0.48. This may double-count the factor or mix two conventions. Not verified | EN 1992-1-1 6.2.3(2) | row exists (`ndp.en1992.vrdcc-c`, naDe 0.5); the literal 0.48 is not in the registry |
| 9 | H | `🗿️artifacts/🏛️en1992/…/✂️shear-torsion/🦀️.rs` | 32–33 | DE branch `(1.1 - fck / 500.0).min(1.0)` times `ndp.en1992.nu1-factor` | Strength reduction ν1 with the DE formula; the numbers 1.1 and 500 are hard-coded | EN 1992-1-1 6.2.2(6) / NA | partial: `ndp.en1992.nu1-factor` exists; 1.1 and 500 have no row |
| 10 | H | `🗿️artifacts/🔩️en1993/…/🔗️special-checks/🌐️storey-checks/🦀️.rs` | 94 | `bracing_length / 500.0` | Bracing deflection limit δq ≤ L/500 | EN 1993-1-1 5.3.3 | none |
| 11 | H | `🗿️artifacts/🔩️en1993/…/🧮️clauses/🌐️global/🦀️.rs` | 117 (and `🧮️clauses/🧰️misc/🦀️.rs` 17) | `e0 = alpha_m * l / 500.0` | Equivalent bracing imperfection e0 = α_m L/500 | EN 1993-1-1 5.3.3(4) | none |
| 12 | M | `🗿️artifacts/🪵️en1995/…/⚖️kernel/🧱️cross-sections/🦀️.rs` | 226 | `0.208, 0.246, 0.267, 0.291, 0.312` | Torsion factor α for rectangular timber sections (table) | EN 1995-1-1 6.1.7, Table 6.? | none |
| 13 | M | `🗿️artifacts/🧩️en1994/…/🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🦀️.rs` | 25, 26 | `PSI_L_PERMANENT = 1.1`, `PSI_L_SHRINKAGE = 0.55` | Long-term effect factors ψL on the modular ratio (bridge) | EN 1994-2 6.4.2 (to verify) | none (`ndp.en1994.psi-l*` absent); ψL is the kind of value that normally needs an NDP row |
| 14 | M | `🗿️artifacts/🧩️en1994/…/🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🦀️.rs` | 24 | `CREEP_PHI = 2.5` | Creep coefficient φ | EN 1994-1-1 3.1.4 (to verify) | none |
| 15 | M | `🗿️artifacts/🧩️en1994/…/🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🦀️.rs` | 22 | `CONCRETE_BEAM_FACTOR = 0.85` | 0.85 f_cd in the stress block | EN 1994-1-1 6.2.1.2 | none, but EN-fixed |
| 16 | M | `🗿️artifacts/🏛️en1992/…/🧵️prestress-and-fatigue/🦀️.rs` | 183 | `0.85 * b * fcd * (1.0 - fck / 250.0)` | Concrete fatigue strength k1 = 0.85 and the (1 − fck/250) term | EN 1992-1-1 6.8.7 | `ndp.en1992.m-01.alpha-cc-persistent` (0.85 in DE) is a different factor. No row for k1 |
| 17 | M | `🗿️artifacts/🏛️en1992/…/🧬️schema/💡️inferences/⚖️checks/🏛️analysis-models/🦀️.rs` | 8 | `th0 = 1.0 / 200.0` | Initial imperfection θ0 | EN 1992-1-1 5.2(5) | none |
| 18 | M | `🗿️artifacts/🏛️en1992/…/🧬️schema/💡️inferences/⚖️checks/✂️shear-torsion/🦀️.rs` | 71 | `alpha_cw`: `1.25`, `2.5 * (1.0 - r)` (EN branch) | EN α_cw formula pieces | EN 1992-1-1 6.2.3(3) | EN-fixed. The DE branch is correctly read from `ndp.en1992.alpha-cw`. Low |
| 19 | M | `🗿️artifacts/🏛️en1992/…/✂️shear-torsion/🦀️.rs` | 44, 146, 213, 231, 244 | `0.6 * (1.0 - fck / 250.0)` | ν = 0.6(1 − fck/250), EN formula | EN 1992-1-1 6.2.2(6), eq. 6.6N | none, EN-fixed. Low–M |
| 20 | M | `🗿️artifacts/🏛️en1992/…/🕳️punching-and-struts/🦀️.rs` | 74 | `[0.45, 0.60, 0.70, 0.80]` | Table k values | EN 1992-1-1 Table 6.1 (to verify) | none |
| 21 | M | `🗿️artifacts/🔩️en1993/…/🧮️clauses/🧱️stability/🦀️.rs` | 211 (`c1_end_moments`) | `2.752, 2.927, 2.704, 2.281, 1.879, 1.563, 1.323, 1.141, 1.0` | C1 for end moments, a table with linear interpolation. Labelled "ENV 1993-1-1 Annex F" | EN 1993-1-1 Annex BB, Table BB.3 (the comment cites an ENV annex) | none. Table constants in code |
| 22 | M | `🗿️artifacts/🔩️en1993/…/📏️parts/🦀️.rs` | 246–249 | `alpha` = `0.13, 0.21, 0.34, 0.49` | Imperfection factors for buckling curves a0, a, b, c, d | EN 1993-1-1 Table 6.1 | none; `ndp.en1993.1-4.ltb.other.alpha` = 0.34 is a different clause (value-only) |
| 23 | M | `🗿️artifacts/🔩️en1993/…/🧮️clauses/🥫️cold-formed/🦀️.rs` | 10 | `0.35, 0.83, 0.6, 0.35` | k_σ for edge folds | EN 1993-1-3 Table 5.? (to verify) | none |
| 24 | M | `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs` | 1180–1182 | `1.15 * mu`, `k / 1.2`, `1.15 * mu`, `1.2 * k` | Silo wall friction: μ_u = 1.15 μ and K_l = K/1.2 | EN 1991-4 (clause to verify) | none |
| 25 | M | `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs` | 383 | `return_period_factor: 0.45 + 0.78 * ln(…)`, `2.59 * v` | Return-period wind factor formula constants | EN 1991-1-4 Eq. (to verify) | none |
| 26 | M | `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs` | 955, 1052, 1075 | `2.5` kN/m² | Footbridge and rail UDL | EN 1991-2 (to verify) | value-only match with `ndp.en1990.a2.rail.hslm.a8.axle_spacing`, a different clause. None |
| 27 | M | `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs` | 1089 | `[0.2, 1.44, 0.82, 1.67, 2.16, 0.73]` | Rail dynamic factors Φ table | EN 1991-2 6.4.6 (to verify) | none |
| 28 | M | `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs` | 822–824 | `1.02`, `0.0148`, `0.4`, `0.8` | Local fire temperature formula (Annex E) | EN 1991-1-2 Annex E | none |
| 29 | M | `🗿️artifacts/🏛️en1992/…/🔥️fire-design/🦀️.rs` | 132, 137 | kc(θ) and ks(θ) tables | Concrete and steel reduction tables at temperature | EN 1992-1-2 Table 3.1 / EN 1993-1-2 Table 3.1 | steel rows exist (`ndp.en1993.1-2.t31.ky.*` …); the code keeps its own copy |
| 30 | M | `🗿️artifacts/🧩️en1994/…/🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🦀️.rs` | 642–645 | ky(θ) table `0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02` | Steel reduction table in the EN 1994 catalogue | EN 1993-1-2 Table 3.1 | rows exist for steel (`ndp.en1993.1-2.t31.ky.*`); the copy is in code |
| 32 | M | `🗿️artifacts/🏛️en1992/…/🧬️schema/💡️inferences/⚖️checks/🧱️materials-and-time/🦀️.rs` | 504, 510, 512 | `0.85 * (220 + 110 ads1)`, `[(100,1.0),(200,0.85),(300,0.75),(500,0.70)]`, `2.5` shrinkage | Creep and shrinkage coefficients (B.4 / Table 3.x) | EN 1992-1-1 Annex B, 3.1.4(6) | none |
| 33 | L | `🗿️artifacts/🔩️en1993/…/🧮️clauses/🧱️stability/🦀️.rs` | 205–206 | `ltb_f`: `0.5`, `0.8` | Modification factor f, Eq. 6.58 | EN 1993-1-1 6.3.2.3 | EN-fixed |
| 34 | L | `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs` | 282–283 | `5.0 / 7.0 * psi_0` | α_A formula | EN 1991-1-4 6.2 | the registry rows (`en-a0`, `en-min-cd`) are already read through `ndp_number` |
| 35 | L | `🗿️artifacts/🏛️en1992/…/🏛️analysis-models/🦀️.rs` | 125 | `β = 0.35 + fck/200 − λ/150` | Creep-shape factor | EN 1992-1-1 5.8.8.3 Eq. 5.31 | EN-fixed |
| 36 | L | `🧮️mechanics/🔩️fastening/🦀️.rs` | 168 (splitting) | `0.8, 1.2, 1.4, 1.25` | Same table as row 4, used for splitting N_Rk | EN 1992-4 | see row 4 |
| 37 | L | `🗿️artifacts/🧩️en1994/…/🧬️schema/💡️inferences/⚖️checks/🧮️catalogue/🦀️.rs` | 146, 383, 569, 843 | `0.85`, `(true, 1, true) => 0.85`, `0.85 * f_cd`, `SpanPosition::End => 0.85` | 0.85 f_cd and span-position factor | EN 1994-1-1 6.2.1.2 | `ndp.en1994.gamma-vs/gamma-c/gamma-s/gamma-v` are read correctly |

## Items confirmed as compliant (no action)

- `🗿️artifacts/🧩️en1994/…/🧮️catalogue/🦀️.rs:` `gamma_vs()` reads `ndp.en1994.gamma-vs` (1.25). `gamma_ap()` reads `ndp.en1994.gamma-ap`.
- `🗿️artifacts/🪨️en1996/…/📚️masonry-tables/🕳️chase-tables/🦀️.rs`: all chase limits read `ndp.en1996.chase.*`. The 0.115 and 0.030 numbers no longer appear in code.
- `🗿️artifacts/🏛️en1992/…/✂️shear-torsion/🦀️.rs:33`: ν1 DE reads `ndp.en1992.nu1-factor`.
- `🗿️artifacts/🏛️en1992/…/✂️shear-torsion/🦀️.rs:79` `cot_theta_max` EN branch reads `ndp.en1992.cot-theta-max`, and the DE cot θ terms read `ndp.en1992.cot-theta-a/b`.
- `🗿️artifacts/🏋️en1991/…/🧱️parts/🦀️.rs:282–283` `alpha_a`: the EN limits and a0 come from `ndp.en1991.alpha-a.*`.

## Hard-coded normative values: what to do

1. Deflection limits (rows 1–3). Add limit-ratio registry rows for EN 1992-1-1 7.4.1 (span/250), the l/500 post-construction limit, and the EN 1994 slab limit. Read them from the registry in all three places.
2. Fastening partial factors (rows 4–5, 36). Add `ndp.en1992.*` rows for the EN 1992-4 Table 4.1 values (γ_Ms N and V, the ratio and the slopes) with the source note, or mark them unverified in the registry. Then replace `SteelPartialFactors::EN_1992_4` with registry reads.
3. Masonry (rows 6–7). Add registry rows for f_vlt (EN and NA-DE) and for the cover by exposure class.
4. Shear DE branch (row 8). Confirm whether `vrdcc-c` = 0.5 already contains the 0.48 factor. If so, remove the duplicate literal. This needs a check against clause 6.2.3(2) and the NA.
5. Bracing and imperfections (rows 10–11, 17). Add registry rows for L/500 and θ0 = 1/200, with the EN 1993 and EN 1992 clause.
6. Tables in code (rows 20–30). Each table in the code duplicates values that the registry also has in part (steel fire ky/kc/ks rows). Move the table into the registry, or read the existing rows.

Limits of this audit: the scanner is heuristic. It may miss a literal without a clause nearby or hidden in a helper, and it may over-report formula constants. Items marked "to verify" are unverified clause numbers and must be confirmed against the standard or the NA before a registry row is written. No values were invented, and no code was edited (read-only audit).
