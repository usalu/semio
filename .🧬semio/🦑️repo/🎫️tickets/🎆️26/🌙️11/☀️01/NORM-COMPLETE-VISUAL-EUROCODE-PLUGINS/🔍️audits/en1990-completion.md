# EN 1990 Norm Completion (second attempt)

## Status
Source implemented, still NOT compiled or tested. Build history (`🗑️generated/build-en1990/`): run 1 SIGKILL in graph-plan (host OOM); run 2 `:check` refused by the compiler ingress (framework defect, `ingress-blocker.md`); run 4 `:build` stopped before rustc because `semio-s-plugin-norm-app-surface` exceeds its source budget (`📈️diagrams2d/🖼️canvas-lowering/🦀️.rs` 32979 B > 29401 B), a file owned by the visual/integration agents. Next: when that crate is split, `repo run '@semio-tech/norm-en1990-rs:build'`, then `:test`, fix errors, regenerate the packs (below), delete the temporary test.

## Round 2 changes (coordinator and review items)
- NDP registry adopted: private `verified` flags and unverified lists removed; checks cite `ndp.en1990.*` ids via `.ndp()` (K_FI e90-02.1, DE 6.10-only e90-16.1, A1.4 e90-24.1/2, thermal/footbridge psi e90-25.1/2, bridge EQU e90-26.1, beta e90-28.1; catalogue entries E90.02/16/24/25/26/27/28). Tests now expect `UnverifiedBasis` or `fail == 0` instead of `complies()`.
- Review S1-1: gamma_G,sup/inf chosen by the sign of each permanent action's effect (`Action.favourable` removed); G-inf effects are -1 in the default, examples, fixtures and DSL assets. S2-1: one signed convention (`total()`, `net()`, `stabilising()`). S2-2: `combinations_dir(..., -1.0)` and `governing_min`; STR/GEO emit `.min` checks when the effect reverses. S2-3: FORM non-convergence returns an error (indeterminate), start at design-value guesses. S2-4/5: Limit gate `!(demand <= capacity)`, positive-input guards in E90.10/20/24. S2-6: `annex_gamma_m` returns NaN for unknown materials. S2-7: NA-DE categories A1..E1, T, Z map to table rows.
- Regression tests from `review/en1990-recompute.py`: demo A (125.0), demo B (-90.0), categories, NaN gamma_M, FORM/zero-span errors. Oracle updated to the sign convention.

## Implemented (paths relative to `R = ⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any`)
Kernel `R/🧬️schema/💡️inferences/⚖️checks/` (rewritten, no dead code, no hard-coded factors):
- `📜️ndp-tables`: NDP registry `NDPS` (symbol, clause, EN and NA-DE value, verified flag), `PSI_TABLE` (Tables A1.1, A2.1-A2.3, snow split at 1000 m for both annexes, NA-DE thermal bridge psi0 0.8), gamma/xi/K_FI/Table 2.1/Annex B tables, `unverified_ndps()`.
- `🧮️combination-generator`: Eq. 6.10, 6.10a, 6.10b (EN only; NA-DE offers 6.10), 6.11b (psi1,1 or psi2,1), 6.12b, 6.14b-6.16b; leading and accompanying actions, mutually exclusive groups (one active group per combination), favourable variables dropped, K_FI on unfavourable only, EQU/STR/GEO sets A/B/C (A1 and A2).
- `📈️reliability-math`: erfc/Phi/Phi^-1, Gumbel/lognormal/normal/Weibull, FORM (Rackwitz-Fiessler HL-RF), Annex C alphas, S-N endurance and Palmgren-Miner, Annex D k_n (linear in 1/n).
- `📇️catalogue`: registry E90.01 to E90.30 (`registry`, `evaluate_case`, `evaluate_outcomes`), every entry with inputs, outcomes, EN/DE titles, NA for missing inputs.
Snapshot report (`R/🧬️schema/💡️inferences/*`): generator-driven STR, GEO (6.10 / 6.10a,b and set C), EQU, 6.11b, 6.12b, SLS against C_d, A1.4 deflection (w1+w2+w3-w_c, w3), vibration, fatigue range, K_FI by RC, RC-versus-CC, input vocabulary validation, Table 2.1, Annex B levels, Annex C beta with P_f, A2 SLS.

## Fixed defects (inventory numbers)
1 (6.10a had no psi0 on the leading action), 6/7/24 dead code removed, 8 EQU constants from tables, 9 deflection fallback removed, 10 bridge working life now a warning, 12/13/14/28 closed vocabularies validated with failing checks and remedies (no silent fallbacks), 15 SLS compared with C_d (new member fields `slsLimit*`), 16 fatigue uses the variable range plus Miner in E90.21, 17/18 clauses A1.4.2, A1.4.4, 6.4.5, A2.4.x, 19 generic 6.10 gone, 20 set C for all structures, 21 EQU unified on the generator, 23 K_FI by RC and not on favourable actions, 25/26 duplicated psi tables merged, EN snow_high/other rows, 27 sentinel DSL/IL utilisation replaced, 29/30 oracle covers all checks, 39 not touched (editor file owned elsewhere), 31 test oracle uses the repo interpreter.
Model: `Member` gained `precamber`, `deflectionW1..3`, `deflectionLimitRatioW3`, `slsLimitChar/Freq/Qp` and lost `deflectionW`. Hand-updated: snapshot Rust, schema twins (json/proto/graphql/ts, snapshot, root, diff), SQLite codec (.rs, .sql, .ts, admission, tests, semantic contract 5838 B / 57 columns / 2458 B), field-meta (EN+DE), 7 examples, 8 DSL assets, 100 JSON fixtures (script, then script deleted). `categories` choice list now equals the psi table.

## Tests and oracle
- `R/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`: 20 kernel tests (spec examples E90-01..09, 15; groups; K_FI; Table B2; Phi; FORM; Annex D; Miner; registry coverage E90.01-30).
- `R/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`: updated report tests plus `catalogue_matches_the_independent_python_oracle` (56 cases, over 200 outcomes) and `snapshot_checks_match_the_brute_force_python_oracle` (14 documents, over 300 checks).
- Oracle `R/🔮️oracles/⚖️compliance/🐍️.py` (scipy.stats, scipy.optimize SLSQP FORM, quadrature, itertools brute force); registered as existing `en1990-1-compliance-independent`. Run with `.venv/bin/python -I`.
- Feature `R/🧪️tests/📇️catalogue-en1990-1/🥒️.feature` (56 rows + 4 scenarios). Fixture `R/🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json`.
- Exact MCP/CLI calls: none succeeded; the only call is the queued `repo run '@semio-tech/norm-en1990-rs:check'`.
- A temporary test `debug_regenerate_pack_assets` in `R/🚪️io/🧪️tests/🔬️unit/🦀️.rs` writes new `.pack.semio` bytes to `🗑️generated/en1990/packs/`; after the first test run copy both files over `R/🖼️assets/🏢️default-office/🎒️.pack.semio` and `R/🖼️assets/🏢️high-consequence-office/🎒️.pack.semio` (the committed packs still hold the old member layout) and delete the test.

## Unverified normative values
Listed by code: `unverified_ndps()` and `psi_unverified_categories()`. Includes: A2.4(A) gamma_G 1.05/0.95 and gamma_Q; rail gamma_Q 1.45 (old DE value 1.40 removed); A2.4(C) gamma_Q 1.15/1.25/1.30; all A2 psi rows (road gr1a-gr5, footbridge, rail, thermal 0.8 in DE); NA-DE "6.10 only" and simplified bound 1.35; A1.4 ratios (L/250, L/300, H/500, h/300, 3 Hz); footbridge accelerations 0.7/0.2/0.4/0.1; rail acceleration, twist table, L/600; gamma_M timber 1.3 and gamma_M other; snow split altitude basis (NN+1000 m).

## Remaining gaps
1. Build and test not run (blocked by the app-surface source budget, see Status), so the artifact is not shown to compile or be green.
2. Mutation kinds for the other editable leaves (structureKind, kFiDeclared, row fields of actions, members, bridge SLS, effects) not added: each kind needs registration in the shared taxonomy fixture `✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1`, the framework schema index, io text/binary codecs and the shared Python mutation vocabulary, none of which can be verified without a build.
3. Snapshot tables for FatigueSpectrum, ReliabilityAnalysis and TestSeries not added; Annex C, D and Miner are reachable through the catalogue only.
4. Exclusive groups exist in the generator and catalogue but the snapshot has no `group` field.
5. Review items not done: S2-8 (split NA-DE GEO-2/GEO-3 rows), S2-9 (Table B2 interpolation), S2-10 (psi scale for 6.12b), S3 list; pattern loading per span.
