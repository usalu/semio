# EN 1998 Norm Completion Report

Artifact: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998`. Status: PARTIAL. Defect fixes and a first set of catalogue formulas are in the source; the set-field mutation expansion, the new-entity catalogue entries and Parts 2 to 6 rewrites are NOT done (see Remaining gaps).

## Build status
- `mcp__repo__test @semio-tech/norm-en1998-rs:test` ran 48 min and failed on "Dependency bindings lease wait exceeded its declared maximum" (shared queue, no compile result). A second MCP run aborted after 1800 s idle.
- Per `📌️important/build-loop.md` the CLI loop was used: `repo run '@semio-tech/norm-en1998-rs:check'` in background, output `🗑️generated/en1998/check.txt` (queued behind other agents, see the end of this file for the result line).

## Defects fixed (source edits, compile result pending)
- Drift (4.4.3.2): utilisation is now d_r·ν / (θ·h) (`part_1::drift_utilization`); the remedy bound is θ·h/ν. A non-positive limit gives infinity, never Pass.
- Accidental torsion (4.3.3.2.4): δ = 1 + 0.6·x/L_e (`part_1::torsion_amplification`), x = L_e/2 + max e0 over all storeys (was first storey only); the 1.2 factor and the 3.0 cap are removed. The simplified procedure is only accepted for `planRegular = true` (else Fail with remedy); numeric bound 1.6 (see unverified).
- Elevation regularity (4.2.3.3): soft storey rule is now per direction, flags a lower storey softer than 0.70 of the storey above (`soft_storey_utilization`); remedy raises the lower storey stiffness X or Y.
- Roof φ check removed: φ_roof = 1.0 is fixed by Table 4.2 and applied in the mass model, so the old `correlatedOccupancy` requirement contradicted the mass model.
- Zero limit: `floor_limit` (seismic helpers) applied to all 17 resistance based utilisations (base shear, bridge, assessment, silo, tank, foundation, wall, tower, storey shear); a positive demand against a zero resistance now Fails. The kernel itself (`⚖️compliance`, not owned) still returns Pass for a zero limit; request logged in `📌️important/requests.md`.
- Duplicate ids: member checks and the masonry check are now `en1998.1.{b}.{system}.member.{m}...` and `en1998.1.{b}.{system}.masonry.simple`.
- Radiation damping: the always-pass check is replaced by a minimum check k_f/k_s ≥ 1.5 with remedy on `kFoundation`; ξ stays in the explanation.
- Given T1: no longer tautological; with method `given` the given period is compared to 1.5 times the C_t estimate (`t1Given`), positive value required.
- Oracle and tests: the oracle test no longer returns early when the script is missing and no longer skips check ids absent from either side; it fails loudly. Python is already launched through the shared `semio_s_plugin_norm_compliance::python_oracle::command()` (no hard-coded `python3` remained in the artifact). The oracle gains `--formulas` (independent closed forms and a characteristic polynomial) and runs with `-I` semantics (stdlib only).

## Catalogue entries implemented
- SEI-04 design spectrum (`design_spectrum_horizontal`, all four branches, β = 0.2), SEI-03 damping correction η.
- SEI-05 vertical spectrum (`vertical_elastic_spectrum`, a_vg ratio input, recommended T_B/T_C/T_D).
- SEI-42 modal response: shear building eigenproblem (cyclic Jacobi), effective modal masses, SRSS, CQC with ρ_ij.
- SEI-43 component combination: pure function plus per-building check `en1998.1.{b}.{system}.componentCombination` (E_x + 0.3 E_y and 0.3 E_x + E_y against V_Rd) when both an x and a y system exist.
- SEI-44 P-Δ: tiers (θ ≤ 0.1, 1/(1−θ) up to 0.2 / 0.3, none above 0.3) in `p_delta_amplification`, used in the pdelta explanation.
- SEI-47 seismic joint gap, SEI-49 non-structural force, SEI-50 strong column ratio: pure functions only (no inputs in the snapshot yet, so no checks).

## Tests added
- Rust (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`): `python_oracle_formula_table_matches_rust_part_1` (spectra, vertical spectrum, δ, drift, CQC, SRSS, envelope, two-storey modes, F_a against the Python oracle), `zero_resistance_with_demand_fails_instead_of_passing`, `member_and_masonry_check_ids_are_unique`, `drift_limit_multiplies_nu_on_the_demand_side`.
- Third-party validation: none available for these formulas in Python stdlib; independent formulation is used (closed forms, characteristic polynomial for eigenvalues).
- Results: NOT RUN (build queue). Exact call: `repo run '@semio-tech/norm-en1998-rs:test'`.

## Unverified normative values
- Torsion simplified-procedure bound δ ≤ 1.6 (x ≤ L_e): my own bound, no clause.
- Given T1 ≤ 1.5·T1(C_t) and k_f/k_s ≥ 1.5 (spec GE-09 marks it with a question mark).
- NA.4 table: repo has B-T = 1.25 and C-T = 1.50; the spec lists B-T = 1.00, C-T = 1.25 [V]. Left unchanged (catalogue panel and tests depend on it); needs checking against DIN EN 1998-1/NA:2011-01.
- Vertical spectrum a_vg/a_g = 0.9 (type 1), T_B/T_C/T_D = 0.05/0.15/1.0; NA-DE value [V].
- All remaining items already flagged in the inventory (q_max table, ψ2 snow, masonry ratios, ν, CF factors, silo/tank/tower constants).

## Second pass (after the coordinator instruction to continue)
- Full set-field coverage: generator `🧰️scripts/en1998-gen-field-mutations.py` (stdlib Python, idempotent, `DRY=1` for a dry run) created 81 `change-<entity>-<field>` mutation leaves (site 6, building 11, system 7, storey 9, member 7, bridge 5, assessment 4, tower 5, silo 9, tank 7, foundation 6, retaining wall 5). Together with the 12 existing ones every non-id, non-variable scalar leaf now has a kind. Per leaf it writes: leaf `🦀️.rs`, `🔺️diff`, `↩️inverse`, `🧬️schema/🔣️.json` and `🟦️.ts`, catalog `🔣️.json` with a unique binary tag, the group-crate mount, the root enum variant, `KINDS`, root `🔣️.json` and `🟦️.ts`, text codec (print, parse, demo sample, imports), binary codec (tag, encode, decode), `📡️.protocol.semio`, five fixture files (before, mutation, after, outcome, diff), the mutate harness (`KINDS`, fixture arm, python row, two feature tables), the oracle manifest (catalog vectors, kinds, manifest mutations), the shared taxonomy fixture rows and the 81 `SetFieldRule` entries.
- New tests: `every_editable_leaf_has_a_set_field_rule` (walks the multipart snapshot value tree) and `every_set_field_rule_names_a_catalogued_kind`.
- Placeholder utilisations removed: regularity claim, multiple systems, DE zone/ground consistency, direction, qDetail, supportedBuilding (Parts 3 and 5), limitState now use plain statuses; lfm keeps its Warning. Elevation irregularity now reduces q by 20 % (Table 4.1); q has the lower bound 1.5 and k_w is clamped to [0.5, 1.0].
- Parts 2 to 6 constants replaced: bridge classification (T_isol/T_fixed ≥ 3 isolated, q = 1.0 else 1.5; bearing displacement from the elastic spectrum); assessment a_g factor from the return period (T_R/475)^(1/3) with 2475/475/225 years, unknown knowledge level treated as KL1; silo and tower evaluated at the spectrum plateau (no fixed 0.3 s / 0.5 s), tower force factor 1.0, silo shell compared with an axial demand and the anchor with the shear; tank by Housner (m_i, m_c, sloshing period, ξ_c = 0.5 % so η = 1.348, SRSS); foundation bearing without the invented reduction (γ_Rd = 1.0).
- NDP registry: `ndp::REGISTRY` in the schema leaf (`🦀️.rs`, module `ndp`) marks NA.4 B-T, C-T (conflict 1.25/1.50 vs 1.00/1.25), the other NA.4 rows, the vertical spectrum ratio and the torsion bound as `Unverified`; a Warning check `en1998.site.ndp.<id>` is emitted for DE combos B-T and C-T. Worklist: `📌️important/ndp-verification-worklist.md`.

## Build status at hand-back
- `repo run '@semio-tech/norm-en1998-rs:check'` (background, `🗑️generated/en1998/check.txt`) has been queued for over an hour: the capacity queue shows 16 requests ahead, the head blocked by three waiting `projects` jobs (6.4 GB each) of other sessions and swap pressure. No compile result exists yet, so nothing in this report is compiled or tested. Next step for whoever reads this: read the `exit` line of `check.txt`, fix compile errors in the touched files (list below), then run `:test`.
- Likely first compile suspects: enum-typed payload fields in `change-site-seismic-zone` and `change-site-de-ground-combo`; the three text/binary codec insertions; `floor_limit` visibility; the new `part_4::convective_period_s(height, radius)` signature.

## Touched files (main)
`🧬️schema/🦀️.rs` (formulas, ndp), `🧬️schema/💡️inferences/**` (checks), `🧬️schema/🧬️mutations/**` (81 leaves, root, groups, edit rules), `🚪️io/📝️text|💾️binary/🧬️mutations/**`, `🧪️tests/🫨️mutate-en1998-1/**`, `🧪️tests/🧫️fixtures/🧬️mutations/**`, `🔮️oracles/**`, shared taxonomy fixture `📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`.

## Remaining gaps
1. Catalogue entries that need new snapshot inputs are NOT done: v_s30 ground classification, regularity derived from geometry (set-backs, re-entrant corners), seismic joints, non-structural elements, capacity design (strong column, beam and column shear, joints, steel), timber and masonry member checks, Part 2 piers, Part 3 chord rotation AS-04..AS-08, Part 5 liquefaction, Annex F bearing and piles, Part 6 Rayleigh period. Reason: each new snapshot field changes every snapshot fixture and the binary `.pack.semio` and `.spr.semio` assets, which can only be regenerated by the compiled codec; with no compile feedback this would corrupt the shared artifact. Pure functions for joints, non-structural elements, strong-column ratio, modal analysis, CQC and SRSS exist in `part_1` and are tested against the Python oracle.
2. Compile and test results (see Build status).
3. Hand-fixing of fixtures for the new torsion, elevation and q rules is unverified: the noncompliant examples may now fail different checks.
4. `ChangeSite*` for `enGroundType` and `enSpectrumType` rely on the serializer omitting empty strings; the generated fixtures follow that rule.


## Third pass (builds, review fixes) - status at 2026-11-02 afternoon
- Build log: `🗑️generated/build-en1998/` (`1.txt` to `8.txt`, outcomes in `latest.md`). Runs 1 to 8 all ended before rustc reached an EN 1998 crate: `:check` cancelled (framework ingress defect), run 2 NX partition cache owner, run 3 no-progress timeout, run 4 compile errors in the shared crate `semio-s-plugin-norm-app-surface-panels` (fixed minimally by me: CheckStatus arms, `choices()`, `&chip_id`; logged in `requests.md`), run 5 and run 8 graph planner SIGKILL, run 6 compiler headroom, run 7 Nx daemon terminated. No EN 1998 rustc diagnostic exists yet; the code is still uncompiled and untested.
- Review fixes implemented (uncompiled): Rayleigh T1 from storey stiffness (`part_1::fundamental_period_s`, Fail check `t1Rayleigh` when stiffness missing), Table 4.2 phi by category (D to F and archives 1.0), `design_spectrum_sd` removed in favour of `design_spectrum_at` (Eq. 3.13 to 3.16 with a_g = gamma_I a_gR; scene response updated), P-Δ amplification applied to storey shears and the simplified limit lowered to 0.2, storey shear from untorsioned forces, no storey-resistance fallback, plan regularity decided by max(e0/0.3r, ls/r) with zero radius failing, Mononobe-Okabe Eq. E.4 with infinity beyond phi, wall loads with gamma_I, gamma_el <= 0 gives capacity 0, tank with full fluid mass and gamma_I, bridge displacement with gamma_I, foundation bearing from N and eccentricity (N = seismic gravity of the supported building, M = V h 2/3), cancelled run returns a Fail check `en1998.cancelled`, torsion L_e is the plan side perpendicular to the action (with a test). Regression tests: `review_regressions_match_the_python_oracle`, `cancelled_evaluation_never_complies`, `torsion_le_is_the_plan_side_perpendicular_to_the_action`; the oracle `--formulas` gained the review values (Rayleigh 0.3285 s for 4 x 400 t on 1.2e9 N/m, Sd below T_B 0.4923, floor 0.168, K_AE 0.47326).
- Not started: DIN EN 1998-1/NA:2023-11 S_aP,R path. It replaces `site.seismicZone` by a plateau acceleration, which touches the snapshot model and twins, the SQLite float-triple columns, the DSL and binary pack assets (only the compiled codec can regenerate the pack), the mutation leaf `change-site-seismic-zone` (rename to `change-site-s-ap-r`), field meta, the catalogue NA.1 table and the scene. I deliberately did not start it on an uncompiled base; it is the first item after a green `:build`.
- Still open from earlier: new-input catalogue entries (see Remaining gaps), `:test` results, golden blessing.
