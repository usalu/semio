# Verify — EN 1996 (`🪨️en1996`) Wave D Adversarial Review

**Reviewer:** read-only verifier (Wave D, Round 2)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1996.md` (138/138, no gaps)  
**Test run:** `bun nx run @semio-tech/norm-en1996-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary line:** `Summary [   0.701s] 139 tests run: 139 passed, 0 skipped`  
**Logs:** `🗑️generated/verify-en1996/test-r2.txt`, `hand-numerics.txt`, `ignored-fields-audit.txt`

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | **FAIL** | 9 |
| R2 | **FAIL** | 4 |

R1 blockers addressed in source: characteristic actions + EN 1990 combinations, sliding μ, fire α, EN 1996-3/NA Table NA.A.1 + basement §4.5, f_vk DE caps, material conformance (replacing f_k tautology), creep e_k in e_mk, designSituation choices, mortarStrengthPa SI Pa. Tests now execute (139/139). Remaining gaps: scope-aware perturbation harness, jsonschema skip hatch, mutation localization, slab-end rotation derivation.

---

## VERDICT: FAIL (4 blocking)

---

## Check table

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PARTIAL / FAIL** | Hierarchical `walls[]` with characteristic actions (`🦀️.rs:173-209`) and `design_effects()` EN 1990 ULS combinations (`⚖️masonry/🦀️.rs:405-477`). **Blocking:** `slabSpanM` is stored but only gates EN 1996-3/NA Table NA.A.1 (`1134-1137`); slab-end rotation eccentricity per DIN EN 1996-1-1/NA simplified method is not derived from span — compression uses user-declared `eccentricityTopM`/`eccentricityBottomM` + bearing depth offset only (`692-701`, `236-239`). |
| 2 | Clause coverage | **PASS** | Check ids include slenderness, material conformance, compression (Φ_i/Φ_m + creep e_k), shear/sliding (μ), lateral §6.3, concentrated §6.1.3, fire α+t_min, durability, bed joint, reinforced §6.6, simplified §4.2, basement §4.5. No remaining always-pass f_k≥0.5 MPa tautology (`630-664`). |
| 3 | Numerics | **PASS** | Hand recomputation in `🗑️generated/verify-en1996/hand-numerics.txt`: f_k=7.599324 MPa (DE NA Eq. 3.1 + δ); λ=4.829; Φ_s=0.674349; Φ_m≈0.532; shear V_Rd,slide=μ·N/γ_M. Python oracle test passes ±0.5 % on simplified + compression utilizations (`🔬️oracle/🦀️.rs:21-48`, `🐍️.py:221-239`). |
| 4 | Applicability | **PASS** | Empty walls/load cases, non-load-bearing, out-of-scope simplified (storeys, λ, q_k, span, t, height) gated `NotApplicable` with localized reasons (`666-678`, `1116-1167`). Basement earth with `isBasement=false` gated (`1251-1266`). |
| 5 | National annex | **PASS** | DE vs EN: f_k factors, γ_M, Φ_s, f_vk cap (`⚖️compliance/🦀️.rs:11-48`, `⚖️masonry/🦀️.rs:153-165`, `261-267`). |
| 6 | Report quality | **PASS** | `[id=]` paths + resolve test (`every_emitted_path_parses_and_resolves_with_id_selectors`, `73-90`). ≥2 fail→pass remedies (`remedy_law_flips_at_least_two_distinct_fails_to_pass`, `94-113`). Dynamic check explanations distinct en/de (`loc()` throughout `⚖️masonry/🦀️.rs`). |
| 7 | Examples | **PASS** | `loadbearing-wall` complies; `multi-fail-masonry` ≥3 fails (`compliant_evaluate_passes`, `noncompliant_has_multiple_fails`, `regenerate_dsl_pack_assets_parse_and_assert_verdicts`). |
| 7b | Inputs UX | **PASS** | Structured editor + `en1996_field_meta` (`📥️inputs/🦀️.rs:14`). `designSituation` has `NormFieldChoice` en+de (`🏷️field-meta/🦀️.rs:119`). `mortarStrengthPa` unit Pa (`105`). `every_editable_leaf_has_en_de_field_meta` (`⚖️compliance/🦀️.rs:117-135`). |
| 8 | Mutations & schema | **PARTIAL / FAIL** | 51+ semantic mutations with diff/inverse. Snapshot TS typed (`📸️snapshot/🟦️.ts`). **Blocking:** diff facet still has bare `unknown[]` — `En1996Diff.walls?: { values: unknown[] }` (`🔺️diff/🟦️.ts:8`). |
| 9 | Tests | **PARTIAL / FAIL** | 139 executed, 0 skipped (`test-r2.txt`). Oracle runs and asserts values. **Blocking:** `json_schema_validates_example_snapshots` accepts `skip-no-jsonschema` when jsonschema missing (`🔬️oracle/🦀️.rs:64-74`) — no-skip-hatch violation per brief ADDENDUM / CORRECTION 13:27(7). |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!`/`stub` in family `*.rs` (rg). Catalogue headline comment only (`📚️catalogue/🦀️.rs:3`). |

---

## Round-1 blocking re-check

| # | R1 blocker | R2 | Evidence |
|---|------------|-----|----------|
| 1 | Hand-typed N_Ed/V_Ed/W_Ed only | **FIXED** | `WallLoadCase` characteristic actions (`🦀️.rs:184-206`); `design_effects()` (`⚖️masonry/🦀️.rs:405-477`) |
| 2 | Sliding §6.2.4 / μ unused | **FIXED** | `V_Rd,slide = μ·N_Ed/γ_M` governing min with masonry (`785-830`) |
| 3 | Fire α utilization | **FIXED** | α = N_Ed,fi/N_Rd; tabulated t_min(α) (`938-976`, `281-335`) |
| 4 | EN 1996-3/NA Table NA.A.1 + basement §4.5 | **FIXED** | Applicability reasons (`1116-1167`); basement check (`1208-1267`) |
| 5 | f_vk DE NA caps / 1 MPa hardcode | **FIXED** | `f_vlt = 0.045·f_b` cap (`153-165`) |
| 6 | f_k tautology ≥0.5 MPa | **FIXED** | `material_conformance_ok` check `en1996.3.1.material.*` (`497-664`) |
| 7 | Editable-but-ignored fields | **PARTIAL** | Fields read when present in evaluate(); **perturbation not scope-aware** for opening/concentrated leaves (see 13:43) |
| 8 | Creep in e_mk | **FIXED** | `creep_eccentricity_m` (`269-276`); used in compression (`700-701`) |
| 9 | designSituation choices + fMPa unit | **FIXED** | `🏷️field-meta/🦀️.rs:105,119` |

---

## CORRECTION 13:27 explicit self-check

| Cause | Result | Evidence |
|-------|--------|----------|
| (1) Human en+de `NormFieldChoice` labels | **PASS** | `🏷️field-meta/🦀️.rs:9-73` |
| (2) Every editable leaf has meta | **PASS** | `every_editable_leaf_has_en_de_field_meta` |
| (3) Structured inputs, not JSON | **PASS** | `render_document_editor` + `en1996_field_meta` |
| (4) `[id=]` paths + resolve test | **PASS** | `⚖️compliance/🦀️.rs:73-90` |
| (5) ≥2 remedy fail→pass | **PASS** | `⚖️compliance/🦀️.rs:94-113` |
| (6) Example DSL decode + verdict | **PASS** | `regenerate_dsl_pack_assets_parse_and_assert_verdicts` |
| (7) Python oracle ±0.5 % + jsonschema | **FAIL** | Oracle OK; jsonschema test has skip hatch (`🔬️oracle/🦀️.rs:64-74`) |
| (8) Facets regenerated | **FAIL** | Snapshot TS OK; `En1996Diff` uses `unknown[]` (`🔺️diff/🟦️.ts:8`) |
| (9) No tautologies / ignored fields | **PASS** | Material/bed-joint pass paths use fixed low utilization when conforming (weak but can fail); no hand-typed design effects |
| (10) No trivial-only tests | **PASS** | `simplified_method_storey_limit_de_na` asserts `NotApplicable` (`58-64`) |
| (11) Semantic mutation verbs | **PASS** | `change-mu`, `change-gk-slab`, `insert-opening`, … |
| (12) Localized dynamic copy | **FAIL** | Opening/basement mutation labels identical en/de or broken DE (`🪟change-opening-height/🦀️.rs:27`, `➕️insert-opening/🦀️.rs:27`, `🏗️change-is-basement/🦀️.rs:26`) |

---

## CORRECTION 13:43 findings

### Structural actions (characteristic → design effects)

**PASS.** `WallLoadCase` carries G_k, q_k (+ category), snow, wind q_p·c_pe, earth H_k, tributary/span; `design_effects()` forms ULS combinations with γ_G/γ_Q/ψ₀ and reports governing combo in explanations (`405-477`, `717-725`). Self-weight from density×geometry enters N (`228-232`, `409-431`).

**Gap (blocking under check 1):** slab-end rotation eccentricity is not computed from `slabSpanM` (or slab stiffness); only user eccentricities + `(t−a_bearing)/2` offset. Span is an editable leaf with limited live effect (NA.A.1 gate only).

### Scope-aware perturbation

**FAIL.** `perturb_every_editable_leaf_changes_some_check` (`⚖️compliance/🦀️.rs:139-246`) walks **only** `compliant_clay_wall()` JSON. That snapshot has **zero** openings and **zero** concentrated loads (`📸️snapshot/🦀️.rs:55,91`). Meta exists for `walls[].openings[].*` and `walls[].loadCases[].concentrated[].*` but those leaves are never perturbed. No additional committed examples (opening wall, basement+earth, reinforced panel, lateral wind panel) are wired into the test despite 13:43 requiring perturbation where each leaf applies. Independent audit: `🗑️generated/verify-en1996/ignored-fields-audit.txt`.

Exempt labels: `id`, `labelEn`, `labelDe` (explicitly skipped in test `169-171`).

---

## Blocking fix list

1. **`⚖️compliance/🦀️.rs` (`perturb_every_editable_leaf_changes_some_check`) + `📸️snapshot/🦀️.rs`** — Add committed scope examples (opening wall with partial + pier opening; basement wall with `isBasement=true` and `hKEarthN>0`; load case with concentrated actions; reinforced wall with As>0) and extend the perturbation harness to walk each example’s editable leaves where applicable. Assert ≥1 check computed value/status change per leaf. Do not exempt opening/concentrated meta paths because the default wall has empty lists.

2. **`🔬️oracle/🦀️.rs` (`json_schema_validates_example_snapshots`, lines 64-74)** — Remove `skip-no-jsonschema` exit path; fail the test if third-party `jsonschema` is unavailable or validation fails. Assert stdout is exactly `ok`.

3. **`🧬️schema/🧬️mutations/**` (opening + basement mutation labels)** — Replace identical en/de mutation labels with proper German engineering text, e.g. `🪟change-opening-height/🦀️.rs:27` → `"Change opening height"` / `"Öffnungshöhe ändern"`; same for width/sill, insert/remove opening; fix `🏗️change-is-basement/🦀️.rs:26` (`"Kellerwand kennzeichnen"` or similar, not `"Ändern: change is basement"`).

4. **`⚖️masonry/🦀️.rs` (compression eccentricity chain ~692-701) + schema if needed** — Derive slab-end rotation eccentricity from `slabSpanM` (and wall height/stiffness per DIN EN 1996-1-1/NA simplified method) into the governing e_0 / e_mk chain; ensure `slabSpanM` perturbation changes compression check numerics in the default compliant example, not only simplified-method applicability.

*(Non-blocking but recommended: regenerate `🔺️diff/🟦️.ts` so `walls` diff is typed, not `{ values: unknown[] }`; use opening height in `area_m2` or document as intentional simplification; remove dead `opening_path` touch loop `1270-1273`.)*

---

## Non-blocking observations

- First R2 nx invocation hit a transient compile error (`error[E0061]: this function takes 5 arguments but 3`); immediate retry compiled and ran 139/139 — likely concurrent edit race, not reproduced on cargo re-run.
- Impl claims 138 tests; runner reports **139** passed.
- `change-mu` mutation label fixed vs R1 (`🧲️change-mu/🦀️.rs:26` — proper DE).
- Python oracle now cross-checks compression `6.1.2` utilization (`🐍️.py:224-226`); R1 gap closed.
- `jsonschema` 4.x is installed in this environment; skip hatch is latent.
- `regenerate_dsl_pack_assets_parse_and_assert_verdicts` writes assets during test (side effect).
- Catalogue panel placeholder comment remains shared across families.

---

## Numeric spot-check summary (check 3)

| Quantity | Hand (R2) | Oracle/Rust | Δ |
|----------|-----------|-------------|---|
| f_k [MPa] | 7.599324 | 7.599324 (oracle self-check) | <0.01 % |
| λ | 4.829 | 4.829 | <0.01 % |
| Φ_s (DE) | 0.674349 | 0.674349 | <0.01 % |
| u_simplified | 0.051186 | oracle vs report | <0.5 % (test pass) |
| u_compression (Annex G) | 0.064870 | oracle vs report | <0.5 % (test pass) |

Full derivation: `🗑️generated/verify-en1996/hand-numerics.txt`.
