<!-- 📋 SUMMARY-DIGEST v2 (snapshot 04:13). Updated again at ticket close. -->
# Ticket Close Summary: Norm Complete Visual Eurocode Plugins (Wave 2)

NOTE: The write to `🔍️audits/wave2/SUMMARY.md` was refused by subagent policy, so this is the full updated markdown (SUMMARY-DIGEST v2, snapshot 04:13). Method: all 73 wave-2 reports modified after 02:40 were checked (first lines of each; full read of the coordinator entries after 02:40, `status-matrix-6.md`, `typed-enums-v3.md`, `adjudicator.md`, `schema-codegen-v2.md`, and the per-code S1 sections). Where two reports disagree, both are given.

## 0. Headline
- Nothing is green on `check` or `test` for any EN 1990 to EN 1999 artifact. The only green build since the fixes is `norm-artifact-contract-rs:build` run 4 (01:42, exit 0, `🗑️generated/integrate/4.txt`). Mechanics was green at run 1 (14:40) and failed at run 2 (01:20). (`status-matrix-6.md` section 2; `🗑️generated/build-runner/runner.log`)
- Source is far ahead of the last compile. Every wave-2 edit from about 02:40 on is marked UNCOMPILED with a "ready for rebuild" line. The latest rustc round is EN 1990 and EN 1991 run 14 (23:46 and 23:47, one error each), both stale. EN 1990 run 15 (03:20) stopped in graph-plan with no rustc output. (`status-matrix-6.md` headline; `build-fixer.md`)
- Fixture checker: `verify_fixtures.py` reports 25 failures: EN 1991 (8), EN 1992 (2), EN 1993 (16), EN 1994 (2), EN 1995 (1). `fixture-kinds.md` (04:04) claimed zero there. Cause not yet confirmed (`status-matrix-6.md` 1a).
- Main blockers at close: the EN 1994 build freeze (`📌️important/freeze.txt` names `build-en1994`, graph-plan since 03:24, run 13 running at 04:08); host overload (load 37 to 68 on 12 CPUs; swap full at 02:32 and 04:03; `infra.md`, `coordinator.md`); Nx out-of-memory in six families; rust cache partition (`infra.md`: 46.4 of 48 GiB used).
- Correctness work was the main output of the late wave. Several S1 items were already correct (3b). Two verdict-changing fixes are in source: EN 1990 and EN 1994 span arrangements (items 27, 28) and the EN 1995 footbridge crowd factor (item 33).
- The repo MCP server reported `CONNECTION_CLOSED` in this session too, so `ticket_close` cannot run from here. Bookkeeping stays in files (`coordinator.md`).

## 0a. Claims in the 02:40 draft that are now outdated
| 02:40 draft said | Now | Source |
|---|---|---|
| "396 verified NDP" (coordinator NDP-FOLLOWUP) | 258 verified. The 396 was stale after the NDP-EVIDENCE downgrade of 117 k_mod/k_def and 21 GL rows | `status-matrix-5.md`; `status-matrix-6.md` |
| NDP "258 of 2938", "2680 unverified" | 258 of 3060 verified (8.4 %), 2802 unverified, 868 awaiting an annex value. `unverified-register.md` (03:08) counts 3053 rows and 2795 unverified (earlier snapshot) | `status-matrix-6.md`; `unverified-register.md` |
| Trace "656 rows" (catalogue-coverage-3) | 714 spec ids, 10 of 10 artifacts ok in the strict verifier. EN 1991 grew from 71 to 109 | `status-matrix-6.md`; `en1991-v10.md` |
| Runner start 02:32:47 logs `JOBS: unbound variable` and lock contention (`runner.out`) | No `JOBS` string in `runner.log` (grep: 0). The file is `runner.log`, not `runner.out`. The 02:32 entry is a normal "runner v2 start". Blockers are now the freeze, load and OOM | `runner.log` |
| EN 1990 "run 14, exit 1, k=1" as latest | Latest is run 15, 03:20, exit 1, k=0, SIGTERM in graph-plan. Run 14's E0063 `factor_overrides` is fixed in source, not confirmed by a compile | `runner.log`; `build-fixer.md` |
| EN 1994 "run 12, exit 143" as latest | Run 13 started 03:24 and was running at 04:08 under the freeze | `runner.log`; `freeze.txt` |
| EN 1990 0/260, EN 1991 0/543, EN 1992 0/194, EN 1994 0/70, EN 1995 255/735, EN 1996 0/289, EN 1997 0/163, EN 1998 0/217 | 0/270, 0/593, 0/212, 0/101, 255/743, 0/294, 0/162, 0/218 | `status-matrix-6.md` section 1 |
| Content leaves: 115, 184, 135, 511, 112, 171, 128, 151, 244, 182 | Re-measured: 103, 202, 119, 478, 109, 172, 112, 157 (1 without kind), 240 (2 without kind), 162 | `status-matrix-6.md` section 1 |
| EN 1991 "snow-drift floor flagged" | Now an NDP option (`drift-mu-w-reading`, conservative default). Adjudicated | `adjudicator.md`; `coordinator.md` 05:20 |
| "Schema sync: no twin generator; SCHEMA-CODEGEN launched" | SCHEMA-CODEGEN v2 landed: 191 mismatches to 0, 4899 twins generated, TS errors about 990 to 0 | `schema-codegen-v2.md` |
| Typed enums: "121 String fields classified; pilot conversions" | Converted: en1992 (8), en1999 (13), en1991 `FireOccupancy`, en1995 (6). v3 repaired fixture data (3c) | `typed-enums-v2.md`; `typed-enums-v3.md` |
| Fixture check "green for all ten" (kind-convention-v4) | 9 of 10 green at 04:04 (en1994 126/1). `status-matrix-6` reports failures in four families | `fixture-kinds.md`; `status-matrix-6.md` 1a |
| Owner tallies from `requests.md` (529 open, 233 done, 12 open S1) | Raw grep at 04:13: 564 `[open]`, 362 `[done]`, 133 `[announce]`, 93 `[info]` (not reconciled). Reconciled: `open-items.md` at 02:45, 510 distinct (section 5) | `requests.md`; `open-items.md` |
| S1 items all open | Several verified correct or fixed (3b) | `en1992-v10.md`, `en1996-v8.md`, `en1998-v10.md`, `en1999-v10.md`, `en1995-span-check.md`, `coordinator.md` 02:46 |
| "Viewport-config persistence open" | Already registered (SHARED v6) | `coordinator.md` |
| "Scene and diagram laws in tests" | 65 `assert_handle_laws` calls. `assert_scene_laws` and `assert_diagram_laws` have no artifact call sites (the i18n-3 claim of 64 was wrong) | `coordinator.md` 06:05 |
| EN 1995 two-span coefficients open | No change needed (support hogging governs) | `en1995-span-check.md` |
| EN 1998 cracked stiffness (4.3.1(7)) not applied | Applied, with an unverified NDP row (0.5). Typed `StiffnessBasis` replaces the 0 sentinel | `en1998-v12.md`; `en1998-v13.md` |

## 1. Per Eurocode
Spec ids, leaves and NDP counts from `status-matrix-6.md` section 1. Build from `runner.log` and `build-runner.md`. Fixtures are scenarios/failures from `verify_fixtures.py` as run for matrix 6.

| Code | Spec ids (trace kinds) | Content leaves / without kind | NDP verified / rows | Fixtures (scen./fail) | Last build |
|---|---|---|---|---|---|
| EN 1990 | 29 (check 29) | 103 / 0 | 0 / 270 | 148 / 0 | run 15, 03:20, exit 1, k=0 (graph-plan SIGTERM) |
| EN 1991 | 109 (check 63, design-case 36, not-modelled 10) | 202 / 0 | 0 / 593 | 246 / 8 | run 14, 23:47, exit 1, k=1 (parse error, stale) |
| EN 1992 | 159 (check 60, design-case 98, data 1) | 119 / 0 | 0 / 212 | 154 / 2 | run 16, 00:26, exit 1, k=0 (timeout 2390 s) |
| EN 1993 | 142 (check 133, data 5, not-modelled 4) | 478 / 0 | 3 / 332 | 578 / 16 | run 12, 00:55, exit 1, k=0 (Nx OOM) |
| EN 1994 | 54 (check 51, data 3) | 109 / 0 | 0 / 101 | 126 / 2 | run 13 running from 03:24 (frozen) |
| EN 1995 | 68 (check 34, design-case 34) | 172 / 0 | 255 / 743 | 208 / 1 | run 17, 22:10, exit 1, k=0 (stale) |
| EN 1996 | 37 (check 37) | 112 / 0 | 0 / 294 | 147 / 0 | run 12, 23:25, exit 1, k=0 (Nx OOM) |
| EN 1997 | 42 (check 42) | 157 / 1 (`piles[].concreteClass`) | 0 / 162 | 204 / 0 | run 14, 22:30, exit 1, k=0 |
| EN 1998 | 32 (check 28, data 4) | 240 / 2 | 0 / 218 | 287 / 0, plus 11 orphan fixture dirs | run 12, 00:08, exit 1, k=0 (OOM) |
| EN 1999 | 42 (check 40, data 1, not-modelled 1) | 162 / 0 | 0 / 135 | 223 / 0 | run 11, 00:19, exit 1, k=0 (duplicate NDP build project) |

Totals: 714 spec ids; 3060 NDP rows (258 verified, 8.4 %); 8 codes with zero verified rows (`status-matrix-6.md`). 13 Python oracle self-checks pass (`py-sweep.md`, 03:39).

### EN 1990 (`en1990-v9.md`, `en1990-v8.md`, `en1990-v7.md`)
- Modelled: per-span permanent-action arrangements (A1.3.1). A permanent action varying by span gets gamma_G,sup or gamma_G,inf per span (2^n); the envelope runs on the shared mechanics solver. Two-span 6+6 m case: M_sag 151 423.65 N m against 145 000.36 N m with one factor (+4.4 %). Test `one_permanent_action_is_unfavourable_in_one_span_and_favourable_in_the_other`; PyNite and anastruct oracles (`en1990-v9.md` section 1).
- Traffic groups typed (`LoadGroup`, kinds 134 to 137, `coordinator.md` EN 1990 v8). Reliability class enum with K_FI from registry (`en1990-v7.md`). `reliabilityClass` threaded through all eight codes (`rc-threading-v2.md`).
- Unverified: K_FI, E90.10 to E90.12, rail values, 54 rows (`en1990-v5.md`). The gamma sign is addressed by the span change, not run.

### EN 1991 (`en1991-v11.md`, `en1991-v10.md`, `en1991-v9.md`, `en1991-v8.md`)
- Modelled: braking check 4.4.1 (14 unverified rows); centrifugal load with radius field (790 fixtures); live tandem envelope agreeing with PyNite and closed form (`en1991-v9.md`). Six collections from v6 (helicopter, ties, construction, falling loads, execution imperfections, derailment) with checks and 3D (`en1991-v6.md`). Fire load: typed `FireOccupancy`, Table E.4 fractile 548.13 MJ/m2 (`correctness-2.md`). Crane runway envelope and silo checks (`en1991-v11.md`).
- Trace rewritten to 109 ids: 16 check, 15 design-case, 9 not-modelled for EN 1991-2/-3/-4 (`en1991-v10.md`). E2.18 merged into E2.17 (no separate spec id); aerodynamics not modelled (`en1991-v11.md`).
- Hipped roof: zones J, M, N labelled "unverified layout"; no spec text for Figure 7.9 (`en1991-v8.md`).
- Unverified: 46 NDP rows; braking, centrifugal, hipped and fire rows.
- Open: six not-modelled ids; 8 fixture failures (insert/remove of `derailment`, `falling`, `helicopter`, `imperfection`, "addresses None") (`status-matrix-6.md` 1a).

### EN 1992 (`en1992-v12.md`, `en1992-v11.md`, `en1992-v10.md`, `en1992-flatslab.md`)
- Modelled: flat-slab example with interior, edge and corner panels; corner fails at 1.255 (`en1992-flatslab.md`). Punching: NA-DE small-u0 reduction, unverified rows, both oracles (`en1992-v11.md`). `s_r` now largest radial gap between stud rings (Eq. 6.52), six rows recomputed (`en1992-v12.md`). Per-stud rows, beta from unbalanced moments and openings deferred to a schema-additions batch.
- Shear: S1 `asw * 1e3` and `min(V_Rd,s, V_Rd,max)` already correct, pinned by `review_s1_regressions` (`en1992-v10.md`).
- Unverified: E_cm moved to registry rows (unverified); punching NA-DE values.
- Open: 9 handle write paths (`status-matrix-4.md` 2b, not re-measured); studs per-stud.

### EN 1993 (`en1993-v10.md`, `en1993-v9.md`)
- Modelled: tolerances justified (thin-wall vs FE) and tightened to 0.5 % with FE on solid outlines; `StiffenerShape` and `WarpingSupport` enums; six feature rows (`en1993-v9.md`). Materials and loadCases as owning collections; load-case cards (`en1993-v10.md`). Ownership per CHECK-HIGHLIGHT (`en1993-v10.md` section 1).
- Not modelled: 1-3 intermediate stiffeners (spec has no equations, `en1993-v10.md`).
- Trace text: V-251 text fixed; TRACE-TEXTS corrected 359 descriptions across 9 codes (`trace-texts.md`).
- Unverified: hollow-joint formulas; 1-5 Annex values (formula unverified, `en1993-v4.md`); 6 NDP ids.
- Open: S1 prying marked done (`requests.md`; `coordinator.md` 02:46). HEB 240 not re-verified. 16 fixture failures.

### EN 1994 (`en1994-v10.md`, `en1994-v9.md`, `en1994-v8.md`)
- Modelled: continuous spans on the shared span envelope; M_sag +4.6 % after the fix (`coordinator.md` SPAN-ENVELOPE; `span-envelope.md`). Beam fire rebar product as typed leaf `rebarProduct` (329 fixtures). Rebar k_s,theta via RebarProduct, 55 unverified fire rows (`en1994-v9.md`). Beam fire `NDPS` constant removed (no reader, `en1994-v10.md`).
- Freeze: `checks` regressed 832 to 481 at the 04:06 run after edits during the freeze (`coordinator.md`, 04:06).
- Open: S1 two-span coefficients and delta sigma s (span change does not address delta sigma s); stud collection; C-32 and C-46 tables; `en1994-checks` near budget (split by CRATE-BUDGET F, `crate-budget-f.md`).

### EN 1995 (`en1995-v10.md`, `en1995-v11.md`, `en1995-decks.md`, `en1995-t96-draw.md`, `en1995-span-check.md`)
- Adjudicated against oracle (3d): T-95 f_d, q_d bracing divisors, k_h density limit, T-96 deflection, DE NA k_f (unverified, `adjudicator.md`).
- Verdict change: footbridge crowd gamma_Q 1.5 to 1.35 (A2), and categories F/G/T/Z psi, via shared ActionFactors (`coordinator.md` 02:50; `gamma-unify.md`).
- Decks T-122 as typed entity: 18 fields, 20 kinds, 565 fixtures, PyNite deck-bar oracle, 3D (`en1995-decks.md`). `b_ef = a + 2h` not confirmed (spec line 433 elides the formula, `en1995-v10.md`).
- T-96 drawing keyed on `n_braced` (`en1995-t96-draw.md`).
- Span check: no change needed; `ContinuousTwoSpan` computes support hogging (governs) and shear with gamma_G,sup (`en1995-span-check.md`). Oracle M_sag change 10098.7 to 10376.7 remains an inconsistency noted in the coordinator (EN1995-SPAN-CHECK).
- Strength class fixtures: 273 recovered from committed base; 154 identical to HEAD (`typed-enums-v3.md` a).
- Forms: 22 of 22 EN 1995 entries form-drawn (FORMS-V2); lost case grips restored (FORMS-V3).
- Unverified: 255 verified EN 338:2016 rows; 117 k_mod and k_def rows downgraded to unverified (`ndp-evidence.md`).
- Open: EN1995-SCENE-UNIFY (48 of 70 cases on shared forms, 22 left, `coordinator.md`).

### EN 1996 (`en1996-v9.md`, `en1996-v8.md`, `en1996-v7.md`)
- f_vk0 material independence already correct, +2 oracle rows. All 37 spec ids are checks (M-05 new). Annex E governing case in 3D. M-95 split into 6 unverified rows (`en1996-v8.md`).
- v9: wall stress colour field (sigma / f_d with eccentricity), yield-line deflection field, bearing-length grip (`en1996-v9.md`).
- Masonry on shared mechanics Analysis (`en1996-v7.md`). Diaphragm ratio 1.10 unverified (`en1996-v6.md`). RC threading end to end (`rc-threading.md`).
- Open: diaphragm ratio 1.10 and Annex E cells (printed standards).

### EN 1997 (`en1997-v9.md`, `en1997-v8.md`, `en1997-v7.md`)
- 23 leaves given set kinds (139 leaves; `en1997-v9.md`). PILE-08 oracle (groundhog xi); downdrag vs R_c,d; E_cm as registry rows (14 unverified).
- Capacity-zero utilisation: `requests.md:1257` marked done; catalogue returns a large value for demand over zero and 0 only for demand zero. No separate report.
- Unverified: 33 GPa modulus "from memory" (`coordinator.md`); RC threading applies K_FI to DA sets as an unverified assumption (`rc-threading-v2.md`).
- Open: gamma_phi and gamma_c of M2 in earth pressure (`requests.md:328`, marked done, no report evidence); internal erosion; 7 open en1997 items (`open-items-sweep-v2.md`).

### EN 1998 (`en1998-v13.md`, `en1998-v12.md`, `en1998-v11.md`, `en1998-v10.md`)
- Cracked stiffness applied (EN 1998-1 4.3.1(6)/(7)): NDP `ndp.en1998.en1.cracked-stiffness-factor` = 0.5, unverified, from memory. Applied to period, Rayleigh and modes, N2 inputs, regularity radii. Oracle: T ratio 1.414; base shear 0.286 gross vs 0.207 cracked (`en1998-v12.md`).
- Typed `StiffnessBasis` (simplified/declared) replaces the 0 sentinel (`en1998-v13.md`).
- Wall G/E from `wallPoissonRatio` (default 0.25); 0.4 constant removed; `joints[]` capacity design (`en1998-v11.md`).
- Rayleigh and phi already correct: scipy eigh gives Rayleigh 0.3285 s vs exact 0.3303 s (`en1998-v10.md`).
- Housner tank coefficient 0.460 per R/H (`coordinator.md`; not in a wave-2 report).
- Unverified: 12 NDP rows incl. Table 3.1 thresholds (`en1998-v8.md`).
- Open: Table 9.3 masonry; 11 orphan fixture dirs; 2 leaves without kind.

### EN 1999 (`en1999-v11.md`, `en1999-v10.md`, `en1999-v9.md`, `en1999-stiffeners.md`, `en1999-v7.md`)
- Section constants for tee, elliptical, channel, angle from FE outlines with oracle adapters (`en1999-v11.md`). Fire f_0.2,theta and resistance-vs-time diagrams.
- Transverse stiffeners `transverseStiffeners[]`, 7 kinds, 6.7.6 checks; `stiffenerSpacing` deleted, panels derived from stiffener rows (`en1999-v9.md`, `en1999-stiffeners.md`).
- E2-07 reverted to spec values after a kernel-matching change was rejected; the spec's thin-walled idealisation applies; FE J within 4 % (`en1999-v7.md`). Rule: worked examples are the reference.
- S1 eta on internal flanges and HAZ plate check: both already correct, tests added (`en1999-v10.md`).
- Unverified: web crippling coefficients null; M_cr sign; five web-stiffener constants (`en1999-v9.md`).
- Open: web crippling UX; 7 open en1999 items (`open-items-sweep.md`).

## 2. Shared and framework
- Live scene (`live-scene.md`): per-entity projection, `SceneMemo`, minimal `SceneDelta`; rolled out in all ten artifacts with rebuild-count and drag tests. Golden status not re-verified (post-build not run).
- Checks and highlight (`check-highlight.md`, 03:19): almost every check already carries `subject.entity_id`; real gaps are empty or collection-level subjects. Shared `Owners` rule, `highlight_owners` in all ten scenes (`open-items-sweep.md`).
- Editor UX (`shared-v8.md` to `shared-v11.md`): shared entity editing (copy, paste, duplicate, bulk edit, snapping, unit-aware entry), key routing, bulk inspector with Mixed/Gemischt (v8); select-all framework verb (v9; v12 partial, stopped by freeze); token-valued choice handles (v10); check-to-entity link wired in all ten editors (v11).
- Typed `ViewOptions` replaces the `#group=` codec (`shared-v8.md`).
- Choice tokens (`choice-tokens.md`, 04:10): generated from schema enums; `choices=267 mismatches=1` (frozen en1994 ANNEX labels). About 100 plain-string fields remain for TYPED-ENUMS.
- Schema codegen (`schema-codegen-v2.md`, 03:37): generator canonical for 4899 twins; checker 191 mismatches to 0; TS errors ~990 to 0. Remaining: en1998 mid-edit, 13 drifted twins after the freeze (`coordinator.md`, 04:15).
- Spec trace (`trace-strict.md` 02:41, `trace-texts.md` 03:02): strict verifier counts check/evaluator crates only; 714 ids all ok. `note` renamed `description`; bilingual `Text {en, de}` (`i18n-fix.md`, `i18n-fix-v2.md`).
- i18n: I18N-FIX (02:43) and v2 (02:50): bilingual SpecText, `designator()`, no default `Lang`, 68 EN 1995 descriptions rewritten. Pre-fix EN-only sink counts in `i18n-3.md` (231 spec-trace descriptions, 141 annex labels, 41 en1997 reasons, 24 check texts).
- NDP registry (`ndp*.md`, `unverified-register.md`, `ndp-diff-fixture.md`, `ndp-orphans.md`): 3060 rows; 0 real orphan lookups (the earlier 134 was scan noise); 162 annex-sensitive ids (`ndp-diff-fixture.md`).
- No defaults (`no-defaults.md` 03:26, `no-defaults-v2.md` 03:31): `impl Default` removed on EN snapshots; `example(annex)` constructors (178 call sites in 52 files).
- Span envelope (`span-envelope.md`): shared `SpanBeam`, `SpanFactors`, `continuous_beam_envelope` in mechanics; used by EN 1990, 1994, 1995.
- Action factors (`gamma-unify.md`, 02:40): shared `ActionFactors` / `ReliabilityClass` with a Python twin; local copies replaced in EN 1990, 1991, 1992, 1994 (partial), 1995, 1996.
- Handle laws (`handle-laws.md`, `laws-wiring.md`, `handle-rules.md`): `assert_handle_laws` in 65 scene tests; the 12 handle gaps were checker false positives.
- Visual coverage (`visual-coverage.md`, 02:57): 140 collections, 129 with 3D, 94 with grips, 46 with diagrams (static). `visual-grips.md` (03:19) and `visual-grips-v2.md` (03:29): many gaps were false negatives.
- Forms (`forms-v2.md` 03:13, `forms-v3.md` 03:32, `forms-v4.md` 04:00): prism, laminate, char, mode, assembly forms; `GripKind`; annotation roles. All uncompiled.
- Live units and latency (`live-units.md`, `realtime-qa.md`): four stale-verdict bugs fixed; latency bench in ten editors; targets drag p95 12 ms, commit-to-checks 400 ms; unmeasured.
- Path budget (`path-budget.md`, 04:08): en1994 path at 256 bytes fixed; 240-byte guard test passes (max 238, `coordinator.md`). Other plugins still over 256 (out of scope).
- Rule guard (`rule-guard.md` 04:04; coordinator RULE-GUARD 21): 0 paths at or above 256 bytes; 0 duplicate scenario ids; 1 schema mismatch (en1998 joints).
- Post-build (`post-build.md`, `post-build-pipeline.md`): `post-build.sh` and `latency-report.py`; hook in runner; not run.

## 3. Correctness fixes and adjudications

### 3a. Fixes found tonight (in source; none verified by a run unless noted)
| # | Code and clause | Problem | Fix | Report |
|---|---|---|---|---|
| 1 | EN 1992 bars (fixtures) | Bars above centroid in 92 of 147 fixtures. | 218 fixtures corrected; regression rows `fail-08..10`. | `correctness.md` 1 |
| 2 | EN 1992 V_Rd,c | `sigma_cp` used b d / 0.9 instead of A_c. | `sigma_cp = -N_Ed / A_c`; V_Rd,c 101877 N (was 102728 N). | `correctness.md` 2 |
| 3 | EN 1992 polygon members | Not centred on outline centroid. | Shoelace centroid. | `correctness.md` 3 |
| 4 | EN 1992 decompression | Oracle sign issue. | Fixed against concreteproperties. | `en1992-v5.md` |
| 5 | EN 1993 governing row | Sum of kN and kNm. | Unit-free demand per effect. | `correctness.md` 5 |
| 6 | EN 1993 alpha_cr (Eq. 5.2) | Overshoot 12 to 17 %, unconservative. | Exact sway eigen solve. | `coordinator.md` |
| 7 | EN 1993 flange curling | Dimensionally wrong. | Fixed. | `coordinator.md` |
| 8 | EN 1994 section class | Root radius omitted; HEB300/S355 class 2 (7.61). | `root_radius_m()`; 6.18, class 1. | `correctness.md` 4 |
| 9 | EN 1994 governing | Unit-mixed sum. | Unit-free demand. | `correctness.md` 5 |
| 10 | EN 1991 fire load (Table E.4) | 450.3 MJ/m2 instead of table fractile. | Typed `FireOccupancy`; 548.13 MJ/m2 (unverified rows). | `correctness-2.md` 1; `en1991-v5.md` |
| 11 | EN 1991 LM1 tandem | One axle; about 43 % unsafe. | Two axles at 1.2 m; worst position; live tandem envelope. | `coordinator.md`; `en1991-v9.md` |
| 12 | EN 1991 wind and fire oracles | Rows missing. | Added. | `correctness-2.md` 3 |
| 13 | EN 1997 elastic settlement | Aspect ratio swapped. | Fixed. | `coordinator.md` |
| 14 | EN 1997 earth pressure | Utilisation basis wrong. | K_required / K_used; GK1 oracle. | `correctness-2.md` |
| 15 | EN 1995 f_r_k | 0 gave infinite utilisation. | Removed. | `en1995-v9.md` |
| 16 | EN 1995 T-95 and k_h | Oracle f_d error (666.7 vs 400 N); Rust error at density 700. | Both fixed (3d). | `adjudicator.md` |
| 17 | EN 1998 accidental torsion | Dropped. | Restored. | `coordinator.md` |
| 18 | EN 1998 planDims, elevation | Status forced to pass. | Fixed. | `coordinator.md` |
| 19 | EN 1998 tanks | Housner 0.230 instead of 0.460 per R/H. | Corrected. | `coordinator.md` |
| 20 | EN 1996 shear | Ignored N and M. | Shear with N and M. | `coordinator.md` |
| 21 | EN 1999 HAZ | Warning instead of Fail at u = 1.94. | Shared status rule. | `adjudicator.md` |
| 22 | EN 1999 E2-07 | Expectations moved to kernel (rejected). | Reverted to spec. | `en1999-v7.md` |
| 23 | Shared list_delta | E0117 in 43 sites. | Local key markers. | `list-delta-fix.md` |
| 24 | Shared io crates | Orphan rule E0117/E0116. | Mounted into owning crate, re-exported. | `io-orphan.md`; `coordinator.md` 03:30 |
| 25 | Shared placements | `origin.x` vs flat `x`. | Flat placements. | `coordinator.md` |
| 26 | Tooling Cargo members | ndp-model member duplicated. | Deduped; check-cargo-authority 0 errors (30 crates). | `crate-budget-g.md`; `coordinator.md` |
| 27 | EN 1990 gamma_G per span (A1.3.1) | One factor across two spans. | Span arrangements: M_sag +4.4 % (151 423.65 vs 145 000.36 N m). Not run. | `en1990-v9.md`; `span-envelope.md` |
| 28 | EN 1994 span arrangements | Same flaw. | Shared envelope, M_sag +4.6 %. Not run. | `coordinator.md`; `span-envelope.md` |
| 29 | EN 1998 cracked stiffness | Not applied anywhere. | Applied via NDP factor 0.5 (unverified). | `en1998-v12.md` |
| 30 | EN 1992 punching `s_r` (Eq. 6.52) | A_sw from innermost ring only. | `s_r` from ring radii; six rows recomputed. Not run. | `en1992-v12.md` |
| 31 | EN 1992 punching NA-DE small-u0 | Reduction missing. | NA-DE reduction, unverified rows. | `en1992-v11.md` |
| 32 | EN 1991 braking 4.4.1 | Not modelled. | Check with 14 unverified rows. | `en1991-v9.md` |
| 33 | EN 1995 footbridge crowd gamma_Q | 1.5 used. | 1.35 (A2) via ActionFactors; verdicts change. Not run. | `coordinator.md` 02:50; `gamma-unify.md` |
| 34 | EN 1995 category psi; EN 1994 construction psi | Local values. | Registry via ActionFactors. | `coordinator.md` 02:50 |
| 35 | EN 1998 wall G/E | Constant 0.4. | `wallPoissonRatio` (0.25 default). | `en1998-v11.md` |
| 36 | EN 1998 Table 3.1 thresholds | Hard-coded from memory. | 12 unverified registry rows. | `en1998-v8.md` |
| 37 | EN 1996 M-95 | One value. | Six unverified rows. | `en1996-v8.md` |
| 38 | EN 1997 pile E_c | Not a field. | Field; 6 design cases to checks. | `coordinator.md` |
| 39 | Shared check ownership | Owners undefined. | Shared `Owners` rule and law tests. | `check-highlight.md` |

### 3b. S1 items: verified correct or fixed
| Item | Status | Source |
|---|---|---|
| EN 1992 `asw * 1e3`; `min(V_Rd,s, V_Rd,max)` | Already correct, pinned | `en1992-v10.md` |
| EN 1993 prying `lb <= l_b_star` | Marked done; verified in source | `coordinator.md` 02:46; `requests.md` |
| EN 1995 two-span coefficients | No change needed | `en1995-span-check.md` |
| EN 1996 `f_vk0` | Already correct, +2 oracle rows | `en1996-v8.md` |
| EN 1997 `utilization()` capacity 0 | Fixed in source | `requests.md:1257` (done) |
| EN 1997 gamma_phi, gamma_c of M2 | Marked done; no report evidence | `requests.md:328` |
| EN 1998 Rayleigh T1; Table 4.2 phi | Already correct | `en1998-v10.md` |
| EN 1999 eta; HAZ plate | Already correct, tests added | `en1999-v10.md` |
| EN 1990 gamma_G sup/inf | Fixed by span arrangements; not run | `en1990-v9.md` |
| EN 1994 two-span; delta sigma s | M_sag changed; delta sigma s not addressed | `span-envelope.md`; open |
| EN 1993 HEB 240 | Not re-verified | open |

### 3c. Typed-enum data repair (`typed-enums-v3.md`, 04:12)
- EN 1995 `strengthClass`: v2 restored 288 fixtures "by most common pattern" (a guess). v3 recovered 273 exactly from the committed base snapshot; 154 identical to HEAD. Residual uncertainty is generator semantics.
- EN 1990 `snow_high` (A1.1, H over 1000 m) restored with psi rows and tests.
- `convert.py --check` guards against lossy conversion. Priority 2 (remaining enums) not done.

### 3d. Oracle-vs-Rust adjudications (`adjudicator.md`, 02:10; decided by clause)
| Item | Side changed | Basis |
|---|---|---|
| EN 1995 T-95 f_d (9.2.5.2(3)) | Oracle: N_d/k_f,1 = 400 N solid; N_d/k_f,2 = 250 N glulam | Printed text |
| EN 1995 q_d bracing divisors (9.2.5.3(1), Table 9.2) | Oracle: 111.1 N/m | Table 9.2 |
| EN 1995 DE NA k_f | Registry rows unverified | No source |
| EN 1995 T-96 deflection (9.2.5.3(2)) | Model comparison: F_d/k_br | Clause names no spring layout |
| EN 1995 k_h hardwood (3.2(3)) | Rust: applies to rho_k <= 700 for all solid classes | Density-limited clause |
| EN 1991 E14.09 c_pi_dominant (7.2.9(5)) | Both interpolate linearly | Clause text |
| EN 1991 E4.01 aac (Table 2.1) | Both: boundaries class 2; thresholds as unverified rows | Table |
| EN 1991 E2.13 resonance (A2.4.3.2) | Both read footbridge band registry rows | Registry |
| EN 1991 E17.10 p_so | Unchanged; no printed formula | Not a disagreement |
| EN 1996 M-60, M-52, M-05, M-16 | Unchanged; pass-through only | Rows prove pass-through |
| EN 1999 6.1.6.haz | Already fixed | Test |
| EN 1998 check_full_seismic floor | Already aligned | Test `sd_floor_gamma_1_4` |
| EN 1991 snow drift floor 0.8 vs 0.4 | Both readings in registry as NDP option | `drift-mu-w-reading` |

## 4. Build status (`🗑️generated/build-runner/runner.log` last `done` per project; `🔍️audits/build-runner.md`)

| Project | Last build | Result | Cause or note |
|---|---|---|---|
| norm-artifact-contract-rs | run 4, 01:42 | GREEN, exit 0 | run 3 (22:48) superseded |
| mechanics | run 2, 01:20 | exit 1, k=0 | `Compiler-captured admitted Source image identity required` (INFRA v3); run 1 green 14:40 |
| EN 1990 | run 15, 03:20 | exit 1, k=0 | SIGTERM in graph-plan (`build-fixer.md`); run 14 k=1 E0063 `factor_overrides`, fixed in source, not confirmed |
| EN 1991 | run 14, 23:47 | exit 1, k=1 | parse error at `bridge_lanes`, stale |
| EN 1992 | run 16, 00:26 | exit 1, k=0 | runner timeout 2390 s |
| EN 1993 | run 12, 00:55 | exit 1, k=0 | Nx OOM |
| EN 1994 | run 13, from 03:24 | running (frozen) | `freeze.txt`; run 12 02:32 exit 143 |
| EN 1995 | run 17, 22:10 | exit 1, k=0 | NDP en1990 dependency failure, stale |
| EN 1996 | run 12, 23:25 | exit 1, k=0 | Nx OOM; earlier run 11 k=8 (E0063 `placements`) |
| EN 1997 | run 14, 22:30 | exit 1, k=0 | Nx failed |
| EN 1998 | run 12, 00:08 | exit 1, k=0 | OOM in graph-plan |
| EN 1999 | run 11, 00:19 | exit 1, k=0 | duplicate Build project `semio-s-plugin-norm-ndp-en1990` |
| fixture-sweep | run 1, 23:23 | exit 1, k=0 | graph planning |

Infra and runner state:
- Freeze: `freeze.txt` names `build-en1994` since 03:26. Queue priority integrate, mechanics, en1990, en1991, en1994 (`coordinator.md`, INFRA v7 04:15).
- Cache: rust partition 48 GiB, 46.4 GiB used; unit-cache lanes (`infra.md`, 04:10).
- Pre-build gate, load gate, `nice-others.sh`, `queue-sequencer.sh` in place (04:06 to 04:07). `build-owners sync` passed (`crate-budget-f.md`, `rc-threading-v2.md`).
- Compile reviews (static only): I (11 certain fixes, `compile-review-i.md`), J (9, `compile-review-j.md`), K (4, `compile-review-k.md`), L (`compile-review-l.md`), M (`compile-review-m.md`); H gate `compile-gate-h/gate.sh` (about 4 min, no cargo).
- Crate budget: F and G split over-budget crates (`crate-budget-f.md`, `crate-budget-g.md`); H pending (en1999-checks at 99.6 %).
- Not run: compile of current source; `:test` anywhere; golden blessing; latency measurement; `post-build.sh`.

## 5. Open items by owner type
Source: `📌️important/open-items.md` (02:45): 511 open lines, 510 distinct.

| Owner | Open items |
|---|---|
| shared | 222 |
| infra | 92 |
| needs printed standards | 61 |
| en1990 | 20 |
| en1991 | 10 |
| en1992 | 19 |
| en1993 | 7 |
| en1994 | 15 |
| en1995 | 6 |
| en1996 | 13 |
| en1997 | 7 |
| en1998 | 14 |
| en1999 | 17 |
| other standards (din, iso, vdi) | 7 |

Key items:
- Shared: select-all and viewer stepping (`shared-v12.md`, frozen); host UI wiring; SCHEMA-ADDITIONS (en1992 punching beta, openings, studs; en1997 concreteClass; en1991 fields); TYPED-ENUMS for about 100 string fields; FIXTURE-KINDS and the 25 fixture failures (`status-matrix-6.md` 1a).
- Per artifact (S1 first): EN 1990 gamma by action kind (fixed, not run); EN 1991 hipped roof (no source), rail twist and acceleration; EN 1992 studs; EN 1993 HEB 240, 1-3 stiffeners (not modelled); EN 1994 delta sigma s, C-32, C-46, fire rows; EN 1995 scene unify, b_ef formula, T-96 drawing; EN 1996 diaphragm ratio, Annex E cells; EN 1997 33 GPa modulus, internal erosion, gamma_phi and gamma_c evidence; EN 1998 Table 9.3, wall plan positions, cracked factor, orphan fixture dirs; EN 1999 web crippling UX, M_cr sign, web-stiffener constants.
- Infra: EN 1994 build (run 13) under freeze; graph-plan OOM and Nx timeouts; rust cache GC; closure-verification memo.
- Needs printed standards: 2802 NDP rows unverified (868 awaiting annex value); EN 1990 and EN 1991 annex values (printed EN and DE NA); EN 1995 k_mod, k_def (117), GL (21); EN 1998 Table 3.1, F.1, 9.3; EN 1991 Table 7.5 and rail values; EN 1993 1-5 Annex charts; EN 1994 fire rows, C-32, C-46; EN 1996 diaphragm ratio; EN 1997 33 GPa and erosion values.

## 6. Scripts and generated folders

### 6a. Tool scripts under `🧰️scripts/` (inputs, keep)
The 205-entry list in the 02:40 draft still holds. Add scripts created after 02:40 (idempotent, niced): `en1991-v10/trace_v10.py`, `en1991-v11/ndp_v11.py`, `🧰️ndp/wave2_en1992_v11.py`, `en1994-v10/`, `en1995-collections/`, `en1995_strength_recover.py`, `typed-enums/convert.py` and `en1995_strength_audit_flat.py`, `en1998-v13/` (4 scripts), `adjudicator/registry.py`, `adjudicator-dryrun.sh`, `no-defaults-v2.py`, `leaf-kinds/en1997.py`, `rc-threading/` (`rc_lib.py`, `rc_leaf.py`, `rc_run.py`), `crate-budget/f_*.py`, `g_*.py`, `schema-codegen/` (`batch.sh`, `roundtrip.sh`, `classify.py`, `importcheck.py`, `typecheck.cjs`), `compile-gate-h/`, `ndp/ndp_annex_differences.py`, `ndp/codegen.py`, `io-orphan-owner-mount.py`, `kind-convention-v4/taxonomy_generate.ts`, `build-runner/nice-others.sh`, `build-runner/queue-sequencer.sh`, `pre-build-gate.sh`. Test generators: `⚖️compliance/🧪️tests/🏷️field-meta-choices/` (`generate.py`, `check.py`), `i18n-sinks/🐍️.py`.
- `leaf-kinds/en1992_run.py --apply` remains destructive (`coordinator.md`, EN 1992 v8).

### 6b. Scripts inside `🗑️generated/` to move before deletion
Unchanged from the 02:40 draft, plus: `status-matrix-5/matrix.py`, `status-matrix-6/matrix.py`; `compile-review-i`, `-j`, `-k`, `-l`, `-m` scan scripts (reusable ones already in `compile-gate-h`); `ndp-orphans/scan-lookups.py`, `scan-exact.py`, `scan-dyn.py`; `en1993-v9/mesh_study.py`, `chord_study.py`; `schema-codegen-v2/` scripts not in `🧰️scripts/`.

### 6c. Generated folders to delete at close
The `🗑️generated/` folder (93-entry list of the 02:40 draft; about 1.4 GiB at 02:40, not re-measured). Add entries created after 02:40: `compile-review-i` to `-m`, `status-matrix-5`, `status-matrix-6`, `schema-codegen-v2`, `ndp-orphans`, `py-sweep`, `fixture-kinds`, `path-budget`, `en1994-v10`, `en1998-v13`, `visual-coverage`, `rc-threading`, `forms-*`.

Keep until the runner stops (runtime state): `build-runner/runner.lock`, `cache-free.mb`, `freeze.txt`, `priority`, `pids/`, live `build-*/latest.md` and `integrate/latest.md`.

Caveats:
- Reports cite `🗑️generated/` paths; after deletion those citations are dead.
- `io-orphan/backup/` and `leaf-kinds-en1993/stage/` are source copies: delete only after mounts and stages are confirmed.
- `compile-review-j.md` used `/tmp` scratch; the ticket rule keeps temporary files in the ticket folder. Move or remove before close.
- About 717 MB of tree copies were deleted by the coordinator at 04:03 and `/home/deck/ivm` by INFRA v5 (`coordinator.md`); not re-verified here.

## 7. Files kept
This `SUMMARY.md` (once writable). The `🔍️audits/wave2/` reports, `coordinator.md`, `📌️important/requests.md`, `📌️important/open-items.md`, `📌️important/build-loop.md`, `📌️important/post-build.md` stay.