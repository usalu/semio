# Verify — EN 1990 (`en1990` ⚖️) — Round 7

**Verifier:** adversarial read-only (Wave D)  
**Date:** 2026-09-26  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Implementer claim:** `📓️impl-en1990.md` (Round 6 fixes: catalogue γ_I + parity test)  
**Test log:** `🗑️generated/verify-en1990/test-r7.txt`  
**Contract log:** `🗑️generated/verify-en1990/contract-test-r7.txt`  
**Hand numerics:** `🗑️generated/verify-en1990/hand-numerics.txt` (carried from R4; still valid)

## Round 1 history

Round 1 (`2026-09-26`): **FAIL (12 blocking)** — effects-only action wiring, missing GEO/EQU-stab/design-life checks, tautological K_FI, wrong SLS clause IDs, ψ category mismatch, missing remedy-law & example-verdict tests, skipped jsonschema, no Annex A2, no favourable G_inf in STR/GEO, identical en/de copy. Tests: `104 passed, 0 skipped`.

## Round 2 history

Round 2 (`2026-09-26`): **FAIL (5 blocking)** — `projectId` orphan; no `every_editable_leaf` meta test; identical en/de STR/GEO/accidental/seismic copy; Annex A2.3 mis-scoped as γ_M (coordinator corrected to A2.4 action γ); Table 2.1 bridge category remedy gap. Tests: `114 passed, 0 skipped`.

## Round 3 history

Round 3 (`2026-09-26`): **FAIL (3 blocking)** — no perturb-each-editable-leaf test; bridge SLS member fields unread on default building snapshot; `variables[].altitudeM` per-variable instead of site `altitudeM`. Tests: `118 passed, 0 skipped`.

## Round 4 history

Round 4 (`2026-09-26`): **FAIL (1 blocking)** — no committed example with `accidentals[]` / `seismics[]`; perturb test scope 4 missing; `accidentals[].ad` / `seismics[].aEd` never walked. Tests: `125 passed, 0 skipped`.

## Round 5 history

Round 5 (`2026-09-26`): **FAIL (1 blocking)** — `is_exempt()` still skipped reference-id leaves (`id`, `memberId`, `actionId`); identity-cast `parseEn1990Artifact`; weak seismic N/A assert. Tests: `136 passed, 0 skipped` (impl later claimed 140).

## Round 6 history

Round 6 (`2026-09-26`): **FAIL (2 blocking)** — Round-5 fixes verified PASS; catalogue `importance_gamma_i()` hardcoded γ_I literals (0.8/1.0/1.2/1.4) instead of `ImportanceClass::gamma_i()`; no test reading `reference_tables()` cells against source functions. Tests: `140 passed, 0 skipped`.

## VERDICT: PASS

**Test Summary (family):** `141 tests run: 141 passed, 0 skipped` (`--skip-nx-cache -- --no-fail-fast`)  
**Test Summary (contract):** `51 tests run: 51 passed, 0 skipped` (`--skip-nx-cache`)

---

## Round 7 — Round-6 blocking re-verification table

| Item | Result | Evidence |
|------|--------|----------|
| `importance_gamma_i()` rows call `ImportanceClass::gamma_i()` (no hardcoded 0.8/1.0/1.2/1.4) | **PASS** | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:95–104` — iterates `[ImportanceClass::I..IV]` and `CatalogueCell::number(class.gamma_i(), 1)`. `rg` on catalogue file: no `0.8`/`1.0`/`1.2`/`1.4` literals. Single source of truth remains `ImportanceClass::gamma_i()` at `⚖️en1990/🦀️.rs:72–78`. |
| ψ rows still call `psi_for_category` (shared annex lookup) | **PASS** | `psi_row` (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:29–30`) calls `psi_for_category(annex, key)` for all EN/DE rows. |
| Test calls `reference_tables()`; EN office ψ₀ equals `psi_for_category(&NaEn, "office").psi_0`; class III γ_I equals `ImportanceClass::III.gamma_i()` | **PASS** | `reference_tables_cells_match_psi_and_gamma_i_sources` (`✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:38–54`). Included in runner (`141 passed`). |
| Perturbation signature `(id, status, computed, limit, utilization)` only | **PASS** | `sig()` (`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:534–547`) — no explanation text. |
| Gaming grep (`let _ =`, `fingerprint`, `1e-9 *`, `1e-12 *`) in evaluate/inference | **PASS** | `rg -n "let _ ="`, `rg -n "1e-9 \*|1e-12 \*|fingerprint"` under `🧬️schema/💡️inferences/` — no matches. Catalogue-only edit introduced none. |
| No ratio slack / whole-subtree skips | **PASS** | `assert!(unchanged.is_empty())` (`🔬️compliance-report/🦀️.rs:707`). Four scope-aware blocks unchanged. |

---

## Round 7 — Round-5 spot-check (no regression)

| # | Round-5 claim | Result | Evidence |
|---|---------------|--------|----------|
| R5-1 | Reference-id leaves perturbed; narrow `is_exempt` | **PASS** | `is_exempt` (`🔬️compliance-report/🦀️.rs:348–351`); `perturb_leaf` dangling `{id}-x` (`:370–375`). |
| R5-2 | Dangling `*Id` → Fail referential-integrity + `one_of` remedy | **PASS** | Test `dangling_effect_action_id_fails_referential_integrity` (`:57`). |
| R5-3 | Duplicate entity ids Fail | **PASS** | Test `duplicate_member_id_fails_integrity` (`:75`). |
| R5-4 | Seismic N/A requires `NotApplicable` when no A_Ek | **PASS** | Test `evaluate_marks_seismic_not_applicable_when_no_a_ek` (`:48–53`). |
| R5-5 | Real `parseEn1990Artifact` (no identity cast) | **PASS** | Test `parse_en1990_artifact_ts_rejects_malformed_snapshot` (`🚪️io/🧪️tests/🔬️unit/🦀️.rs:99`). |

---

## Check table (Wave D 1–10)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical subject with action catalogues, `effects[]` wiring, `bridgeSls[]`, site `altitudeM`. |
| 2 | Clause coverage | **PASS** | ULS STR/GEO/EQU (+stab), SLS, FAT, Annex A/B/C, Table 2.1, bridge A2.4, accidental 6.11, seismic 6.12b. |
| 3 | Numerics | **PASS** | Hand derivations in `🗑️generated/verify-en1990/hand-numerics.txt` (carried from R4). |
| 4 | Applicability | **PASS** | Explicit `NotApplicable` + en/de; scope 4 N/A flip when tables cleared. |
| 5 | National annex DE vs EN | **PASS** | `en_combination_6_10a_differs_on_other_psi`, `de_snow_high_altitude_psi`, `seismic_combination_de_vs_en_diverge_on_other_psi_2`, `bridge_a2_partial_factors_de_vs_en_diverge_on_rail_gamma_q`. |
| 6 | Report quality | **PASS** | `[id=…]` paths + resolve test. Remedy-law β + deflection. Distinct en/de copy. |
| 7 | Examples | **PASS** | Six example pairs with verdict tests (default, high-consequence, bridge, accidental/seismic, fatigue). |
| 7b | Inputs UX | **PASS** | `every_editable_leaf_has_en_de_field_meta`; structured editor `render_document_editor`. |
| 8 | Mutations & schema | **PASS** | Semantic mutations; facets regenerated; typed snapshot TS. |
| 9 | Tests | **PASS** | `141 executed / 141 passed / 0 skipped`. Catalogue parity test present and green. |
| 10 | Stubs | **PASS** | No runtime `todo!`/`unimplemented!`/`stub` in family evaluate path. |

---

## CORRECTION 13:27 self-check

| # | Item | Result | Evidence |
|---|------|--------|----------|
| 1 | NormFieldChoice human en+de | **PASS** | Choice label≠value in field-meta tests. |
| 2 | Every editable leaf meta + test | **PASS** | Wildcard table incl. accidental/seismic. |
| 3 | Structured editor (not JSON dump) | **PASS** | `render_document_editor`. |
| 4 | `[id=…]` + resolve test | **PASS** | `every_emitted_subject_path_parses_and_resolves`. |
| 5 | ≥2 remedy-law tests | **PASS** | β + deflection; Table 2.1. |
| 6 | Example DSL + complies / fail≥2 | **PASS** | Six example pairs. |
| 7 | Python oracle + jsonschema | **PASS** | Oracle ±0.5 %; jsonschema via python3. |
| 8 | Facets regenerated | **PASS** | Typed `En1990Artifact`; no `_placeholder`. |
| 9 | No tautology; wired inputs | **PASS** | Perturb harness green; referential integrity on dangling refs. |
| 10 | No trivial asserts | **PASS** | Concrete numeric values throughout. |
| 11 | Semantic mutations | **PASS** | `insert-accidental`, `insert-seismic`, `change-altitude-m`, etc. |
| 12 | Localized dynamic text | **PASS** | Distinct en/de in STR/GEO/accidental/seismic/integrity checks. |

---

## Blocking fix list

*(none — Round 6 blocking items resolved)*

---

## Non-blocking observations

- Round-6 catalogue fixes are minimal and correct: γ_I table now shares `ImportanceClass::gamma_i()`; parity test asserts both ψ₀ and γ_I against the same functions `evaluate()` uses.
- `📓️audit-perturbation-gaming.md` ⚖️ en1990 section remains stale (still lists old exemptions); code re-grep is clean.
- nx runner wall time ~1m 14s (cargo lock contention on shared cache); nextest execution itself 1.044 s for 141 tests.
- Contract crate stable at 51 tests; all green.
- `🧬️mutations/🟦️.ts` still uses `unknown[]` for bulk list mutations — acceptable for mutation payloads.
