# Verify — EN 1997 (`🌍️en1997`, Wave D read-only, round 5)

VERDICT: FAIL (3 blocking)

**Runner (en1997):** `bun nx run @semio-tech/norm-en1997-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [3.862s] 92 tests run: 92 passed, 0 skipped** (log: `🗑️generated/verify-en1997/test-r5.txt`). Implementer claim **92/92** (`📓️impl-en1997.md:3–4`) **confirmed**.

**Runner (contract):** `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → **Summary [1.201s] 51 tests run: 51 passed, 0 skipped** (log: `🗑️generated/verify-en1997/contract-r5.txt`).

## Round history

| Round | Verdict | Notes |
|-------|---------|-------|
| 1 | FAIL (9) | Compile, DIN 1054 situations, DA2\*, passive sliding, settlement remedy, field-meta, remedy-law, oracle/jsonschema, false runner claim — all fixed in source. |
| 2 | FAIL (7) | Unread GWL/γ′, base inclination/GK, no Rankine/K₀, no DSL verdict tests, no leaf-meta iteration test, index paths / no resolve test, Bishop surrogate + embedment/UPL/CPT gaps. |
| 3 | FAIL (6) | Unread leaves (c_u, ν, governingLayerId, pileType), earth-pressure tautology, hardcoded γ_c — all addressed in code/tests. |
| 4 | FAIL (3) | CPT/SPT φ tautology; no governing design-situation report; TS snapshot/mutation facets still bare `unknown`/`Record<string, unknown>`. |
| 5 | **FAIL (3)** | Round-4 blockers **FIXED**. New/retained failures: empty `reference_tables()`; four `let _ =` dummy binds (gaming audit); no duplicate-entity-id integrity checks. |

## Round 4 blocker re-check (3/3 FIXED)

| # | Round-4 blocker | Round-5 | Evidence |
|---|-----------------|---------|----------|
| 1 | `en1997.2.phi.derived.*` tautology | **FIXED** | `.utilization(stated φ′, φ_char)` + explicit `Fail` when stated > investigation `:1331–1350`; test `phi_stated_above_investigation_fails_and_remedy_passes` `:753–771` |
| 2 | Governing BS-P/T/A summary (en+de) | **FIXED** | `push_governing_situation_summary` `:1115–1145`; per footing/pile/wall/uplift + project `en1997.governing.situation` `:1679–1690`, `:1803+`, `:1995+`, `:2165+`, `:2180–2204`; outline `governingSituation`/`governingApproach` `:27–89`; test `governing_design_situation_changes_with_load_case_situation` `:775–810` |
| 3 | Typed TS facets (no bare `unknown[]` / `Record<string, unknown>`) | **FIXED** | Typed `SoilLayer`/`SpreadFoundation`/… in `🧬️schema/🟦️.ts:5–113`; mutations union `🧬️mutations/🟦️.ts:4–24`; test `typed_facets_have_no_unknown_and_match_rust_fields` `:814–861` |

## Round 5 — per-item verification

### Round-4 blocking items (re-verify)

| Item | Evidence | Result |
|------|----------|--------|
| φ′ derived check not tautological; stated > investigation → Fail; remedy flips Pass | `🦀️.rs:1324–1350`; test `phi_stated_above_investigation_fails_and_remedy_passes` | **PASS** |
| Governing design-situation summary en+de in report metadata; piles/walls/UPL per-situation params | `push_governing_situation_summary`; pile loop `resolve_params` per `design_situations` `:1720–1721`; wall `:1834+`; UPL `resolve_upl_params` `:2150+`; outline `:54–89` | **PASS** |
| Typed TS snapshot/schema/mutation facets | `🟦️.ts`, `🧬️mutations/🟦️.ts`, `📸️snapshot/🟦️.ts`; test `typed_facets_have_no_unknown_and_match_rust_fields` | **PASS** |

### CORRECTION 14:54 — catalogue reference tables

| Item | Evidence | Result |
|------|----------|--------|
| `reference_tables()` not `Vec::new()` | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:17–18` returns `Vec::new()` | **FAIL** |
| Same numbers as `evaluate()` (partial factors / bearing / φ limits) | `resolve_params` `:357–418`, `resolve_upl_params` `:430–451` — no shared const or table export | **FAIL** |
| Clause, distinct en/de titles, units | No tables published | **FAIL** |
| Test: one evaluated limit equals matching table cell | Catalogue test only asserts empty tables don't invent UI sections `:23–24`; no numeric parity test | **FAIL** |

### CORRECTION 14:42 — gaming audit + perturbation harness

| Item | Evidence | Result |
|------|----------|--------|
| No `let _ =` dummy binds in evaluate/inference | `rg -n "let _ ="` → `🦀️.rs:665` (`gamma_w`), `:1221` (`path_c`), `:1251` (`path_type`), `:1548` (`path_h`) | **FAIL** |
| Perturbation signature `(id, status, computed, limit, utilization)` — no explanation | `perturb_every_editable_leaf_changes_some_check` `:870–883` — 5-tuple, no explanation field | **PASS** |
| No ratio slack / no over-broad exemptions | `assert!(unchanged.is_empty())` `:1020–1022`; exemptions only `id`/`structureId`/`name`/`title`/`labelEn`/`labelDe` `:897` | **PASS** |
| Nested leaf walk (all array items/depths) | `walk` recurses objects + arrays `:888–912` | **PASS** |
| Dangling reference → explicit Fail + `one_of` remedy (en+de) | `governingLayerId` missing → `en1997.11.bishop.*` Fail `:2048–2075`; test `governing_layer_id_missing_fails_bishop` `:664–670`; perturb includes `"missing-layer-x"` `:995` | **PASS** |
| Duplicate entity ids → Fail | No `push_duplicate_ids` or equivalent in `check_project`; en1990 pattern at `⚖️en1990/…/💡️inferences/🦀️.rs:912+` absent | **FAIL** |

### CORRECTION 14:37 — perturbation gaming

| Item | Evidence | Result |
|------|----------|--------|
| No `fingerprint`, `1e-9 *`, `1e-12 *` in evaluate/inference | `rg` over family → clean (test-only float guards like `phi > phi_cpt + 1e-9` at `:1278` acceptable) | **PASS** |
| Perturbation asserts status/computed/limit/utilization — not explanation-only | Signature includes status + computed + limit + utilization `:878–880` | **PASS** |

### CORRECTION 13:43 — structural actions

| Item | Evidence | Result |
|------|----------|--------|
| Characteristic actions combined per EN 1990 + DE NA inside `evaluate()` | `design_actions(&p, g, q)` `:1073`; footing loop `:1393–1395`; piles `:1733`, `:1767` | **PASS** |
| No hand-typed design effects as sole action input | Subject uses `FoundationLoadCase` permanent/variable fields | **PASS** |

### CORRECTION 13:27 (12 causes)

| # | Cause | Result | Evidence |
|---|--------|--------|----------|
| 1 | Human en+de `NormFieldChoice` labels | **PASS** | `lookup_field_meta` `:2220+`; test `field_meta_enum_choices_are_localized` |
| 2 | Every editable leaf has meta + iteration test | **PASS** | `default_snapshot_editable_leaves_have_meta` `:548–585` |
| 3 | Structured inputs editor | **PASS** | `📥️inputs/🦀️.rs` (not JSON dump) |
| 4 | Entity paths `[id=…]` + resolve test | **PASS** | `every_emitted_path_resolves_via_get_value_at_path` `:589–609` |
| 5 | ≥2 remedy-apply fail→Pass tests | **PASS** | `remedy_law_footing_width_clears_bearing_or_improves`, `remedy_law_investigation_depth_clears`, `phi_stated_above_investigation_fails_and_remedy_passes` |
| 6 | DSL decode + verdict tests | **PASS** | `compliant_dsl_asset_complies`, `noncompliant_dsl_asset_has_multiple_fails` `:533–544` |
| 7 | Python oracle ±0.5 % + jsonschema | **PASS** | `python_oracle_matches_check_project_within_half_percent` `:287`; Bishop in oracle `🐍️.py:276+`; `example_snapshot_validates_against_json_schema` `:332+` — no skip |
| 8 | Facets regenerated | **PASS** | `typed_facets_have_no_unknown_and_match_rust_fields` |
| 9 | No ignored editable fields / tautologies | **PASS** | Perturbation `:867–1023`; φ derived no longer tautological |
| 10 | No trivially-true tests | **PASS** | Numeric assertions throughout; oracle compares utilization values |
| 11 | Semantic mutation verbs | **PASS** | `change-footing-width`, `insert-layer`, … |
| 12 | Localized dynamic text (no copy en→de) | **PASS** | Distinct en/de in `loc(...)` throughout checks |

### Brief checks §1–10 (round 5)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical SI subject; governing situation now reported |
| 2 | Clause coverage | **PASS** | Real checks; φ derived no longer tautological |
| 3 | Numerics | **PASS** | `annex_d_n_factors_phi30`, `pile_shaft_worked_example`, DA2\* / BS-T tests `:16–86` |
| 4 | Applicability | **PASS** | `empty_collections_are_not_applicable` |
| 5 | National annex | **PASS** | DE vs EN γ_R,v, DA1-C1 γ_c, UPL params tests |
| 6 | Report quality | **PASS** | en+de; paths resolve; fail→pass remedies verified |
| 7 | Examples | **PASS** | Compliant/noncompliant DSL + demos |
| 7b | Inputs UX | **PASS** | `lookup_field_meta` table `:2213+` |
| 8 | Mutations & schema | **PASS** | Typed facets; semantic mutations |
| 9 | Tests | **PASS** | 92 executed, 0 skipped |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in family tree |

## Blocking fix list

1. **`✏️editor/📌️panels/📚️catalogue/🦀️.rs`** — Implement `reference_tables()` with at least: (a) DIN 1054 / `resolve_params` partial-factor table (γ_G, γ_Q, γ_R,v, γ_R,h by BS-P/BS-T/BS-A + approach + annex) sharing the same numbers as `resolve_params` (`🧬️schema/🦀️.rs:357–418`); (b) UPL/HYD factors from `resolve_upl_params` (`:430–451`); optionally bearing N_q/N_c/N_γ sample row from `part_1::bearing_factor_*`. Each table needs `CatalogueTable { id, title_en, title_de, clause, columns, rows }` with distinct en/de column labels and units. Add a unit test (catalogue or compliance) asserting `reference_tables()` is non-empty and that one cell value equals the corresponding output of `resolve_params` / `resolve_upl_params` for a fixed (approach, annex, situation) tuple.

2. **`🧬️schema/🦀️.rs:665, :1221, :1251, :1548`** — Remove all four `let _ = …` dummy bindings. Either wire the value normatively or delete the unused path/constant: `:665` use `gamma_w` in the GWL buoyancy term (σ′ increment) instead of only `gamma_prime/gamma`; `:1221`/`path_c` wire `cohesionEffective` into the layer-strength check subject path when drained; `:1251`/`path_type` stop binding `soil_type` only in explanation — `is_cohesive_layer` already reads it (`:811–816`); `:1548`/`path_h` remove or use `path_h` in a remedy target. Re-run `rg -n "let _ =" ` over the family — must be empty.

3. **`🧬️schema/🦀️.rs` (`check_project`) + compliance tests** — Add referential-integrity checks for **duplicate entity ids** in every id-bearing collection (`layers`, `footings`, `footings[].loadCases`, `piles`, `piles[].testProfiles`, `retainingWalls`, `slopes`, `upliftCases`), mirroring en1990 `push_duplicate_ids` (`⚖️en1990/…/💡️inferences/🦀️.rs:912–991`): Fail status, localized en+de explanation, remedy `one_of` with free id strings. Add tests `duplicate_layer_id_fails_integrity` and `duplicate_footing_id_fails_integrity` (or one parameterized test per collection).

## Non-blocking observations

- Round-4 engineering fixes (φ investigation, governing situation, typed facets) are genuine; 92/92 runner is trustworthy for regression.
- Perturbation signature tuple order is `(id, status, utilization, computed, limit)` rather than the documented `(id, status, computed, limit, utilization)` — all five fields are compared; functionally adequate.
- `structureId` remains descriptive-only (exempt); perturbation skips it `:897`.
- `gamma_w` is defined at `:651` but the settlement GWL branch only scales by `gamma_prime/gamma` — implementing item 2 should complete the buoyancy model.
- Python oracle includes Bishop slice FoS (`🐍️.py:276+`); parity test covers both examples.
- Slopes use GEO-3 params regardless of project DA (`🦀️.rs:2016–2018`) — documented in check explanation en/de.
- OCR field label en/de both `"OCR"` (`:2336` area) — acceptable technical term.
- Guard helpers under `📝️text/🟦️.ts` use `Record<string, unknown>` for runtime parsing only; excluded from `typed_facets_have_no_unknown_and_match_rust_fields` facet list.

---

*Verifier: read-only subagent · ticket `26/09/26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` · logs `🗑️generated/verify-en1997/`*
