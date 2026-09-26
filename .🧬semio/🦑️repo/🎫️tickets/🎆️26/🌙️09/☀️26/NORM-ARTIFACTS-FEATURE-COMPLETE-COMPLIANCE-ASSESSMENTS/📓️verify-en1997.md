# Verify — EN 1997 (`🌍️en1997`, Wave D read-only, round 6)

VERDICT: PASS

**Runner (en1997):** `bun nx run @semio-tech/norm-en1997-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [1.256s] 97 tests run: 97 passed, 0 skipped** (log: `🗑️generated/verify-en1997/test-r6.txt`). Implementer claim **97/97** (`📓️impl-en1997.md:3–4`) **confirmed**.

**Runner (contract):** `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → **Summary [0.629s] 51 tests run: 51 passed, 0 skipped** (log: `🗑️generated/verify-en1997/contract-r6.txt`).

## Round history

| Round | Verdict | Notes |
|-------|---------|-------|
| 1 | FAIL (9) | Compile, DIN 1054 situations, DA2\*, passive sliding, settlement remedy, field-meta, remedy-law, oracle/jsonschema, false runner claim — all fixed in source. |
| 2 | FAIL (7) | Unread GWL/γ′, base inclination/GK, no Rankine/K₀, no DSL verdict tests, no leaf-meta iteration test, index paths / no resolve test, Bishop surrogate + embedment/UPL/CPT gaps. |
| 3 | FAIL (6) | Unread leaves (c_u, ν, governingLayerId, pileType), earth-pressure tautology, hardcoded γ_c — all addressed in code/tests. |
| 4 | FAIL (3) | CPT/SPT φ tautology; no governing design-situation report; TS snapshot/mutation facets still bare `unknown`/`Record<string, unknown>`. |
| 5 | FAIL (3) | Round-4 blockers **FIXED**. Retained failures: empty `reference_tables()`; four `let _ =` dummy binds; no duplicate-entity-id integrity checks. |
| 6 | **PASS** | All three round-5 blockers **FIXED** and verified. Round-4 items not regressed. 97/97 + 51/51 runners clean. |

## Round 4 blocker re-check (spot-check, not regressed)

| # | Round-4 blocker | Round-6 | Evidence |
|---|-----------------|---------|----------|
| 1 | `en1997.2.phi.derived.*` tautology | **PASS** (unchanged) | `🦀️.rs:1470–1490` `.utilization(stated φ′, φ_char)` + explicit `Fail` when stated > investigation; test `phi_stated_above_investigation_fails_and_remedy_passes` `:753–771` |
| 2 | Governing BS-P/T/A summary (en+de) | **PASS** (unchanged) | `push_governing_situation_summary` `:1251–1281`; calls at `:1816+`, `:1937+`, `:2135+`, `:2314+`; test `governing_design_situation_changes_with_load_case_situation` `:775–810` |
| 3 | Typed TS facets (no bare `unknown[]` / `Record<string, unknown>`) | **PASS** (unchanged) | `🟦️.ts`, `🧬️mutations/🟦️.ts`, `📸️snapshot/🟦️.ts`; test `typed_facets_have_no_unknown_and_match_rust_fields` `:814–861` |

## Round 6 — per-item verification

### Round-5 blocking claim 1 — catalogue `reference_tables()`

| Item | Evidence | Result |
|------|----------|--------|
| `reference_tables()` non-empty (3 tables) | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:19–20` returns `vec![partial_factors_table(), upl_hyd_factors_table(), bearing_n_factors_table()]` | **PASS** |
| Partial-factor rows call `resolve_params` (not duplicated constants) | `:47` `let p = resolve_params(*approach, *annex, sit)` inside row mapper | **PASS** |
| UPL/HYD rows call `resolve_upl_params` | `:87` `let p = resolve_upl_params(*annex, sit)` | **PASS** |
| Bearing N-factors from shared `part_1::bearing_factor_*` | `:122–124` | **PASS** |
| Clause id on each table | `:36`, `:76`, `:109` `ClauseId::new(...)` | **PASS** |
| Distinct en/de table titles | `:34–35`, `:74–75`, `:107–108`; test asserts `title_en != title_de` `:35` | **PASS** |
| Column labels + units where applicable | Partial-factor / UPL columns dimensionless (`unit: None`); φ column `unit: Some("°")` `:111` | **PASS** |
| Test: one cell equals `resolve_params` for fixed tuple | `reference_tables_partial_factors_match_resolve_params` `:41–65` — row `de-da2-bsp` γ_G/γ_Q/γ_R,v/γ_R,h vs `resolve_params(Da2, De, "bsP")` | **PASS** |
| Test: tables non-empty in render path | `renders_reference_tables_with_examples` `:21–37` | **PASS** |

### Round-5 blocking claim 2 — gaming audit (`let _ =`, normative field reads)

| Item | Evidence | Result |
|------|----------|--------|
| No `let _ =` in evaluate/inference | `rg -n "let _ ="` over family → **0 hits** | **PASS** |
| `gamma_w` in groundwater buoyancy / effective-stress term | `settlement_oedometric_with_gwl` `:651–665` — `(layer.gamma - gamma_w).max(0.0)` scales Δσ when GWL above mid-layer | **PASS** |
| Drained `cohesionEffective` changes computed value | Layer strength `:1353` `idx = phi_prime_deg + cohesion_effective/1000.0`; remedy targets `path_c` `:1378–1383` | **PASS** |
| Soil type selects cohesive vs drained | `is_cohesive_layer` `:811–816` reads `soil_type` + `cohesion_undrained`; branch at `:1348–1355` | **PASS** |
| Height in formula | Wall active pressure `:1964` `wall.height.powi(2)`; Bishop `:2165` `slope.height`; field-meta `:2465`, `:2482` | **PASS** |
| Height in remedy target | Wall sliding `:2037–2040` `retainingWalls[].height`; slope Bishop `:2189–2192` `slopes[].height` | **PASS** |
| No `fingerprint` / `1e-9 *` / `1e-12 *` gaming folds in evaluate | `rg` over `🧬️schema/🦀️.rs` — only divide guards (`max(1e-9)`) and comparison tolerances; no `fingerprint` | **PASS** |

### Round-5 blocking claim 3 — duplicate entity-id integrity

| Item | Evidence | Result |
|------|----------|--------|
| `push_duplicate_ids` / `push_all_duplicate_ids` | `🦀️.rs:1116–1248` | **PASS** |
| Collections covered: layers, footings, loadCases, piles, testProfiles, retainingWalls, slopes, upliftCases | `push_all_duplicate_ids` `:1171–1248` | **PASS** |
| Fail + en+de explanation + `one_of` remedy | `:1153–1165` | **PASS** |
| Called from `check_project` before other checks | `:1290` | **PASS** |
| Test: duplicate layer id | `duplicate_layer_id_fails_integrity` `:1027–1041` | **PASS** |
| Test: duplicate footing id | `duplicate_footing_id_fails_integrity` `:1045–1057` | **PASS** |
| Test: all 8 id-bearing collections | `duplicate_ids_fail_for_every_id_bearing_collection` `:1061–1126` | **PASS** |
| Dangling `governingLayerId` → explicit Fail (not N/A) | Bishop else-branch `:2196–2224`; tests `governing_layer_id_missing_fails_bishop` `:664–670`, `governing_layer_id_missing_still_fails_with_duplicates_present` `:1130–1138` | **PASS** |

### Perturbation harness (CORRECTION 14:42 / 14:37)

| Item | Evidence | Result |
|------|----------|--------|
| Signature fields: id, status, computed, limit, utilization (no explanation) | `perturb_every_editable_leaf_changes_some_check` `:870–883` — 5-tuple `(id, status, util×1e9, computed×1e6, limit×1e6)`; no explanation field | **PASS** |
| No ratio slack | `assert!(unchanged.is_empty())` `:1020–1022` | **PASS** |
| Exemptions only descriptive id/name/title/labelEn/labelDe (+ structureId) | `:897` | **PASS** |
| Nested leaf walk (all array items) | `walk` recurses objects + arrays `:888–912` | **PASS** |
| No explanation-only acceptance | Compares status + computed + limit + utilization scalars | **PASS** |

### CORRECTION 13:43 / 13:27 (retained)

| Item | Evidence | Result |
|------|----------|--------|
| Characteristic actions combined in `evaluate()` | `design_actions` + footing/pile loops (unchanged) | **PASS** |
| Field-meta, paths, remedies, oracle, examples | Prior round-5 evidence; tests still present and passing | **PASS** |
| Stubs | `rg` todo/unimplemented/placeholder/stub/dummy → 0 in family | **PASS** |

### Brief checks §1–10 (round 6)

| # | Check | Result |
|---|--------|--------|
| 1–10 | Subject, clauses, numerics, applicability, annex, report, examples, UX, mutations, tests, stubs | **PASS** (no new regressions; 97 executed, 0 skipped) |

## Blocking fix list

*None — all round-5 blockers resolved.*

## Non-blocking observations

- Perturbation signature tuple **field order** is `(id, status, utilization, computed, limit)` rather than the documented `(id, status, computed, limit, utilization)` — all five fields are compared; functionally adequate.
- Catalogue partial-factor / UPL column labels reuse symbol text (e.g. `γ_G`) for en and de; table **titles** are properly localized.
- UPL/HYD table cells are sourced from `resolve_upl_params` in code but only the partial-factor row is covered by the numeric parity unit test; code path is direct call, not a second constant table.
- `structureId` remains descriptive-only (perturbation exempt at compliance test `:897`).
- Slopes use GEO-3 params regardless of project DA (`🦀️.rs:2158–2160`) — documented in check explanation en/de.
- Python oracle and compliance tests use `1e-9`/`1e-12` only as comparison tolerances and divide guards — not perturbation gaming.
- Test count rose 92 → 97 (+5 catalogue/integrity tests per `📓️impl-en1997.md`).

---

*Verifier: read-only subagent · ticket `26/09/26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` · logs `🗑️generated/verify-en1997/`*
