# Verify — EN 1996 (`🪨️en1996`) Wave D Adversarial Review

**Reviewer:** read-only verifier (Wave D, Round 5)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** fixer `daafe239` — 144/144 + 51/51, catalogue evaluate-limit gate closed  
**Test run:** `bun nx run @semio-tech/norm-en1996-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary line:** `Summary [   0.567s] 144 tests run: 144 passed, 0 skipped`  
**Contract:** `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → `Summary [   0.279s] 51 tests run: 51 passed, 0 skipped`  
**Logs:** `🗑️generated/verify-en1996/test-r5.txt`, `contract-r5.txt`

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | **FAIL** | 9 |
| R2 | **FAIL** | 4 |
| R3 | **FAIL** | 2 |
| R4 | **FAIL** | 1 |
| R5 | **PASS** | 0 |

---

## VERDICT: PASS (0 blocking)

---

## Check table

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical `walls[]` + characteristic actions; `design_effects()` EN 1990 ULS combinations. Slab-end rotation `eccentricity_from_slab_rotation_m` + `phi_1_slab_span` in compression Φ chain (`⚖️masonry/🦀️.rs:247-270`, `922-935`). Scope examples intact (`📸️snapshot/🦀️.rs:161-222`). |
| 2 | Clause coverage | **PASS** | Slenderness, material conformance, compression (Φ·Φ₁ + creep e_k), shear/sliding, lateral §6.3, concentrated §6.1.3, fire α+t_min, durability, bed joint, reinforced §6.6, simplified §4.2, basement §4.5. |
| 3 | Numerics | **PASS** | Hand/unit tests: λ, Φ_s(DE), f_k DE/EN divergence, fire tabulated (`⚖️compliance/🦀️.rs:11-54`). Oracle ±0.5 % (`🔬️oracle/🦀️.rs`). |
| 4 | Applicability | **PASS** | Empty walls/load cases, non-load-bearing, out-of-scope simplified, basement without earth gated `NotApplicable` with localized reasons. |
| 5 | National annex | **PASS** | DE vs EN f_k factors, γ_M, Φ_s, f_vk cap (`de_na_fk_clay_group1_m10`, `en_vs_de_fk_divergence`, `phi_s_de_vs_en_na_simplified`, `de_na_gamma_m_by_category_and_execution_class`). |
| 6 | Report quality | **PASS** | `[id=]` paths + resolve (`every_emitted_path_parses_and_resolves_with_id_selectors`). ≥2 fail→pass remedies (`remedy_law_flips_at_least_two_distinct_fails_to_pass`). Distinct en/de (`mutation_and_check_labels_differ_en_de`). |
| 7 | Examples | **PASS** | `loadbearing-wall` / `multi-fail-masonry` DSL assets parse; `committed_dsl_pack_assets_parse_and_assert_verdicts` reads committed assets only (`305-323`). |
| 7b | Inputs UX | **PASS** | Field meta for openings/concentrated incl. `widthM` (`🏷️field-meta/🦀️.rs:89-133`). `every_editable_leaf_has_en_de_field_meta` covers core wildcards. |
| 8 | Mutations & schema | **PASS** | Distinct en/de mutation labels. Typed `En1996Diff.walls?: { values: MasonryWall[] }` (`🔺️diff/🟦️.ts:9`). `typed_diff_and_snapshot_facets_have_no_unknown` (`380-428`). |
| 9 | Tests | **PASS** | 144/144 executed, 0 skipped. `reference_tables_cells_match_evaluate_sources` now calls `evaluate_building` on `En1996Snapshot::compliant_clay_wall()`, finds `en1996.1-2.fire.{wall_id}`, and asserts `check.limit.value == *t_min` where `t_min` is the `fire-min-thickness-rei90` / `rei90-clay` catalogue cell (`📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:90-111`). Fire evaluate emits limit from `fire_min_thickness_na` via `.minimum(t, t_min)` (`⚖️masonry/🦀️.rs:1232-1245`). Catalogue row sources `fire_min_thickness_m` → same `fire_min_thickness_na` chain (`📚️catalogue/🦀️.rs:126`, `⚖️masonry/🦀️.rs:368-370`). |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!`/`stub` in evaluate path. `rg "let _ ="` clean in `⚖️masonry/` and `💡️inferences/`. No fingerprint/epsilon gaming in schema evaluate. |

---

## Round-4 blocker re-check (R5)

### Blocker 1 — Catalogue `reference_tables()` (ADDENDUM 14:54)

| Requirement | Result | Evidence |
|-------------|--------|----------|
| `reference_tables()` non-empty | **PASS** | Returns 5 tables via shared helpers (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:24-31`). |
| Same consts `evaluate()` reads | **PASS** | Catalogue imports `fire_min_thickness_m` (`🦀️.rs:4`, `126`); evaluate fire block calls `fire_min_thickness_na` (`⚖️masonry/🦀️.rs:1232`); `fire_min_thickness_m` delegates to `fire_min_thickness_na` at α=1.0 (`368-370`). |
| DE γ_M, f_k factors, fire REI 90, ψ₀, f_vlt/f_b | **PASS** | Tables unchanged (`🦀️.rs:34-187`). |
| Distinct en/de titles | **PASS** | Asserted per table (`📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:29`). |
| Test: evaluated check `limit` = catalogue cell | **PASS** | `reference_tables_cells_match_evaluate_sources` (`📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:90-111`): `evaluate_building` on committed `compliant_clay_wall`; load multiplier sweep until α ∈ (0.6, 1.0]; `check.id == "en1996.1-2.fire.{wall_id}"`; `assert_eq!(check.limit.value, *t_min)` with `t_min` from `rei90-clay` cell (not a standalone hardcoded limit). Prior `cell == const` assertions retained (`53-88`). |

### Blocker 2 — Referential integrity (CORRECTION 14:42)

| Requirement | Result | Evidence |
|-------------|--------|----------|
| Duplicate `walls[]` / `loadCases[]` / `openings[]` / `concentrated[]` → Fail en+de | **PASS** | `push_duplicate_ids` + `push_referential_integrity` before normative loop (`⚖️masonry/🦀️.rs:576-631`, `719-774`). |
| Dangling foreign keys → Fail en+de + `one_of` remedy | **PASS** | `push_unknown_imposed_category`, `push_unknown_design_situation` with `Remedy::one_of` (`633-717`, `620`, `662`, `707`). |
| Tests: duplicate + dangling + signature change | **PASS** | `duplicate_wall_id_fails_integrity` (`⚖️compliance/🦀️.rs:438-456`); `duplicate_load_case_opening_and_concentrated_ids_fail_integrity` (`459-488`); `dangling_imposed_category_fails_integrity` (`491-503`); `perturb_wall_id_to_duplicate_changes_integrity_signature` (`506-548`). |

---

## Round-2 spot-check (no regression)

| Item | R5 | Evidence |
|------|-----|----------|
| Scope perturb harness (5 scopes) | **PASS** | `perturb_every_editable_leaf_changes_some_check` scopes (`⚖️compliance/🦀️.rs:249-262`). |
| jsonschema non-zero exit + stdout `ok` | **PASS** | `json_schema_validates_example_snapshots` (`🔬️oracle/🦀️.rs:70-73`). |
| Distinct en/de mutation labels | **PASS** | `mutation_and_check_labels_differ_en_de` (`326-377`). |
| Slab-span → e_θ / Φ₁ in compression | **PASS** | `eccentricity_from_slab_rotation_m`, `phi_1_slab_span` at compression (`⚖️masonry/🦀️.rs:922-935`); perturb bumps `slabSpan` (`199-200`). |

---

## ADDENDA 14:37 / 14:42 perturbation gaming

| Item | R5 | Evidence |
|------|-----|----------|
| Signature excludes explanation | **PASS** | `sig()` tuples omit explanation (`⚖️compliance/🦀️.rs:140-154`, `507-520`). |
| No fingerprint / id_score / epsilon fold in evaluate | **PASS** | `rg` clean in `⚖️masonry/` evaluate path; `.len()` only in mutations/tests, not evaluate scoring. |
| `id` exempt in leaf walk only for descriptive ids | **PASS** | Walk skips `id`/`labelEn`/`labelDe` (`163-165`); dedicated integrity tests cover id duplication. |

---

## Evaluate-limit assertion evidence

```90:111:✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs
    let mut doc = En1996Snapshot::compliant_clay_wall();
    let wall_id = doc.walls[0].id.clone();
    let base_g = doc.walls[0].load_cases[0].g_k_slab_n;
    let mut matched = false;
    for mult in 1..=80 {
        doc.walls[0].load_cases[0].g_k_slab_n = base_g * (mult as f64);
        let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
        let check = report
            .checks
            .iter()
            .find(|c| c.id == format!("en1996.1-2.fire.{wall_id}"))
            .expect("fire check from evaluate_building");
        if (check.limit.value - *t_min).abs() < 1e-12 {
            assert_eq!(check.limit.value, *t_min);
            matched = true;
            break;
        }
    }
    assert!(
        matched,
        "evaluate_building fire limit never matched catalogue cell {t_min} (need α in (0.6, 1.0])"
    );
```

Fire source: `t_min = fire_min_thickness_na(...)` → `.minimum(Quantity::length_m(t), Quantity::length_m(t_min))` (`⚖️masonry/🦀️.rs:1232-1245`).

---

## Blocking fix list

None

---

## Non-blocking observations

- R3 blocker 2 (referential integrity) is substantively closed: integrity runs at `evaluate_building` entry (`774`), uses `Remedy::one_of`, and has four dedicated tests.
- `push_unknown_design_situation` is implemented but lacks a dedicated test (imposed-category dangling test covers the `one_of` pattern).
- `perturb_every_editable_leaf_changes_some_check` signature tuple orders utilization before computed/limit (`148-150`) while `perturb_wall_id_to_duplicate_changes_integrity_signature` uses computed/limit/utilization (`515-517`); both exclude explanation — harmonize for readability only.
- Material conformance pass path still uses fixed 0.5 utilization when conforming (`⚖️masonry/🦀️.rs:869`) — weak but remedy path can fail.
- `every_editable_leaf_has_en_de_field_meta` wildcard list still omits some concentrated/opening paths though meta exists in `🏷️field-meta/🦀️.rs`.
