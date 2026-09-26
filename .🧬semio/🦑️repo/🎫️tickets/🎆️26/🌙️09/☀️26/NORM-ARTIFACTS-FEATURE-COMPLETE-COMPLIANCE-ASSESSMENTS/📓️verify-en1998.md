# Verify — EN 1998 (`en1998` / 🫨️) — Round 5

**Verifier:** adversarial read-only (Wave D)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Implementer claim:** `📓️fix-en1998-r4.md` — NA.1/NA.4 catalogue, dangling `one_of`, duplicate-id integrity; 74/74  
**Test runs:**  
- `bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache -- --no-fail-fast` → `Summary [0.667s] 74 tests run: 74 passed, 0 skipped` (`🗑️generated/verify-en1998/test-r5.txt`)  
- `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → `Summary [0.136s] 51 tests run: 51 passed, 0 skipped` (`🗑️generated/verify-en1998/contract-r5.txt`)

---

**VERDICT: PASS**

---

## Round history

| Round | Verdict | Blocking count |
|-------|---------|----------------|
| R1 | FAIL | 12 |
| R2 | FAIL | 7 |
| R3 | FAIL | 2 |
| R4 | FAIL | 2 |
| R5 | **PASS** | **0** |

---

## Round 5 — 10-check table (brief-verify-family)

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Subject completeness | **PASS** | Nested `En1998Snapshot`: `buildings[]` with `systems`/`storeys`/`members`, characteristic `permanentGkN` + `variables[]`; parts 2–6 entities with `permanentGkN`/`contentQkN` (no `massKg`/`seismicWeightN`). Default `compliant_de_office` realistic 4-storey RC frame (`📸️snapshot/🦀️.rs:58–120`). |
| 2 | Clause coverage | **PASS** | Part 1: spectrum, base shear, drift, plan/elevation regularity, torsion, roof φ, q-detail, masonry, dual-system. Parts 2–6: bridge shear/bearing, assessment capacity/limit-state, silo/tank, foundation bearing/sliding, retaining wall, tower overturning. Empty collections → explicit `NotApplicable` (`💡️inferences/🦀️.rs:189–196`, `:1140–1142`, etc.). No `limit==computed` tautologies on Fail paths. |
| 3 | Numerics | **PASS** | Hand derivation for compliant default: T1/S_d/F_b and plan ℓ_s (`🗑️generated/verify-en1998/hand-numerics-r5.txt`). Matches unit tests `de_na_br_spectrum_at_t1` (`🧪️tests/⚖️compliance/🦀️.rs:10–16`), `t1_ct_and_base_shear_lambda` (`:30–40`), `compliant_default_has_no_fail` (`:55–58`). Oracle ±0.5 % on all DE combos (`:590–648`). |
| 4 | Applicability | **PASS** | `en1998.1.buildings.na`, `en1998.2.na`, `en1998.3.na`, `en1998.4.silo.na` / `tank.na`, `en1998.5.foundation.na` emit `NotApplicable` with localized reason when list empty. Zone 0 low-seismicity test (`:43–52`). |
| 5 | National annex | **PASS** | DE: `AnnexParams::De` + `na_de::GroundCombo`/`SeismicZone`. EN: typed `enGroundType` + `enSpectrumType` (`💡️inferences/🦀️.rs:77–93`). `en_type1_vs_de_divergence` (`🧪️tests/⚖️compliance/🦀️.rs:19–27`) proves S_e differs >5 %. EN annex consistency check `en1998.site.deNaConsistencyUnderEn` (`💡️inferences/🦀️.rs:110–154`). |
| 6 | Report quality | **PASS** | All checks carry localized en≠de explanations (`committed_examples_have_localized_explanations_and_remedies` on 6 fixtures). Remedy paths use `[id=…]` (`emitted_remedy_paths_use_id_selectors_and_resolve`). Fail remedies flip checks: `remedy_raising_vrd_improves_base_shear`, `remedy_setting_detailing_flips_q_detail`, `remedy_regularity_flips_when_both_flags_set`. Dangling refs: explicit `CheckStatus::Fail` + `Remedy::one_of` (`💡️inferences/🦀️.rs:1216–1225`, `:1352–1361`; test `dangling_supported_building_fails_with_one_of_remedy`). |
| 7 | Examples | **PASS** | 8 committed snapshots: office ±, multipart ±, torsion ±, EN office ± (`📸️snapshot/🦀️.rs`). Verdict tests: `compliant_example_evaluates_clean`, `fail_example_has_expected_fails`, `multipart_examples_evaluate_expected_verdicts`, `torsion_irregular_examples_evaluate_expected_verdicts`, `en_annex_examples_evaluate_expected_verdicts`. Editor lists 4 DSL examples (`✏️editor/🦀️.rs:63–68`). |
| 7b | Inputs UX | **PASS** | `field_meta_covers_every_editable_leaf_of_default_snapshot` — every default leaf has en+de label; `site.seismicZone` and `site.deGroundCombo` have localized enum choices (`🧪️tests/⚖️compliance/🦀️.rs:147–181`). |
| 8 | Mutations & schema | **PASS** | Granular `En1998Mutation` per editable scalar; facets regenerated (typed nested interfaces in snapshot). Mutation oracle tests under `🧪️tests/🫨️mutate-en1998-1/`. |
| 9 | Tests | **PASS** | 74 executed, 74 passed, 0 skipped. Perturbation, oracle, catalogue-eval tie, integrity, remedy, jsonschema, DE/EN divergence tests present and value-asserting. Contract 51/51. |
| 10 | Stubs | **PASS** | `rg` over evaluate/inference: no `todo!`, `unimplemented!`, `TBD`, `fingerprint`, `let _ =` (mutation inverse stubs only). `reference_tables()` populated (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:19–21`). |

---

## Round 4 → Round 5 delta (fixer claims)

| R4 blocker | R5 status | Evidence |
|------------|-----------|----------|
| `reference_tables()` empty | **FIXED** | `table_na1_zones()` + `table_na4_ground_combos()` from `SeismicZone::a_gr()` / `GroundCombo::spectrum_params()` (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:19–81`). Tests: `reference_tables_cells_match_na4_spectrum_params` (catalogue unit), `catalogue_na4_cell_matches_evaluated_spectrum_params` (compliance), `renders_reference_tables_with_examples` asserts rendered ids. |
| Dangling `supportedBuildingId` wrong remedy | **FIXED** | `Remedy::one_of` with building ids + `CheckStatus::Fail` on assessments (`💡️inferences/🦀️.rs:1216–1225`) and foundations (`:1352–1361`). Test `dangling_supported_building_fails_with_one_of_remedy`. |
| Duplicate entity ids not Fail | **FIXED** | `push_referential_integrity` at evaluate start (`:185`); `push_duplicate_ids` for buildings, bridges, assessments, silos, tanks, foundations, retainingWalls, towers, nested systems/storeys/members/variables (`:1108–1135`). Test `duplicate_building_id_fails_integrity`. |

---

## Perturbation-gaming re-check (ADDENDA 14:37 / 14:42 / 14:54)

| Audit item | R5 status |
|------------|-----------|
| `a_gr + len()` epsilon (`:159`) | **REMOVED** — `rg` clean under `💡️inferences/` |
| `we * 1e-9` inventory (`:439`) | **REMOVED** |
| `let _ = ok_rho` (`:664`) | **REMOVED** — `ok_rho` gates status |
| Test signature includes `explanation` | **FIXED** — `(id, status, computed, limit, utilization)` at `🧪️tests/⚖️compliance/🦀️.rs:518–524` |
| Ratio slack / broad exemptions | **CLEAN** — `assert!(unchanged.is_empty())`; exemptions `id`/`name`/`title` only (`:474`); reference ids perturbed to dangling via `is_reference_id_leaf` (`:444–450`, `:487–488`) |
| Empty catalogue | **FIXED** — see R4 delta |

**Verdict:** implementation gaming **CLEAN**; perturbation harness **CLEAN**.

---

## Test runner summaries

| Runner | Executed | Passed | Skipped | Failed | Log |
|--------|----------|--------|---------|--------|-----|
| `@semio-tech/norm-en1998-rs:test` | 74 | 74 | 0 | 0 | `🗑️generated/verify-en1998/test-r5.txt` |
| `@semio-tech/norm-artifact-contract-rs:test` | 51 | 51 | 0 | 0 | `🗑️generated/verify-en1998/contract-r5.txt` |

Key round-5 tests observed passing: `catalogue_na4_cell_matches_evaluated_spectrum_params`, `duplicate_building_id_fails_integrity`, `dangling_supported_building_fails_with_one_of_remedy`, `every_editable_leaf_perturbation_changes_report`, `de_ground_combo_table_matches_oracle_for_all_combos`.

---

## Blocking fix list

*(empty — family closed)*

---

## Non-blocking observations

- Torsion-regular, torsion-irregular, and EN annex examples exist in `📸️snapshot/🦀️.rs` with evaluate tests but are **not** registered in `✏️editor/🦀️.rs::examples()` (catalogue lists rc-frame + multipart only). UX parity gap only.
- `python_oracle_matches_within_half_percent` still `continue`s when an oracle check id is absent from Rust (`🧪️tests/⚖️compliance/🦀️.rs:221–222`) — not a parse soft-continue; oracle coverage could assert bidirectional id parity.
- `example_snapshot_validates_against_json_schema` runs on office compliant/noncompliant only, not all eight fixtures.
- `📓️audit-perturbation-gaming.md` §en1998 is stale (pre–round-3b); code re-grep was clean.
- `📓️impl-en1998.md` runner counts outdated (claims 63/68/70; actual 74).

---

## Prior rounds (unchanged)

See R1–R4 sections in prior `📓️verify-en1998.md` revisions for full Wave-D history, CORRECTION 13:27/13:43, and R2→R3 gaming removal narrative.
