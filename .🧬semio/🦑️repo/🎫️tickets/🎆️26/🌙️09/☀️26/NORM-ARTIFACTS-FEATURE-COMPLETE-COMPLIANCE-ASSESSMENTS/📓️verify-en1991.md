# Verify — EN 1991 (🏋️) adversarial family verification

**Verifier:** Wave D Round 4 (read-only; tests run)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1991.md` — round-3 closeout, 79/79 tests  
**Test runs (verifier):**

| Command | Result | Log |
|---------|--------|-----|
| `bun nx run @semio-tech/norm-en1991-rs:test --skip-nx-cache -- --no-fail-fast` | **Summary [1.439s] 79 passed, 0 skipped** | `🗑️generated/verify-en1991/test-r4.txt` |
| `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **Summary [0.346s] 51 passed, 0 skipped** | `🗑️generated/verify-en1991/contract-r4.txt` |

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | **FAIL** | 10 |
| R2 | **FAIL** | 6 |
| R3 | **FAIL** | 4 |
| R4 | **PASS** | **0** |

**VERDICT: PASS**

---

## Round 4 — Round-3 blocking re-check

| R3 # | Requirement | Result | Evidence |
|------|-------------|--------|----------|
| 1 | Scope-aware perturbation: every nested editable leaf (all array indices/depths); numeric+bool+enum; signature `(id, status, computed, limit, utilization)`; exemptions only id/name/title/labelEn/labelDe | **PASS** | `scope_aware_perturbation_of_editable_leaves` (`💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:405–627`). `collect_leaves` recurses all depths and array indices (`:449–475`), including `accidentalCases[id=explosion-1].explosion[0].*` second-row paths. `assign_at` perturbs numbers, bools, and strings via `next_string` (`:528–557`, `:482–509`). Delta asserts `id`, `status`, `computed.value`, `limit.value`, `utilization` (`:616–623`). Per-suite top-level N/A lists scope out-of-domain keys only (e.g. bridge fields on office); descriptive leaf exemptions = `EXEMPT_LEAVES` (`:448`). Test passes on all four committed examples (office/bridge/fire/accidental). |
| 2 | `reference_tables()` from shared evaluate consts; non-empty; one catalogue cell = evaluated limit (B1 DE 2000 Pa) | **PASS** | `reference_tables()` (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:18–116`) calls `part_1_1::imposed_qk_pa`, `part_1_3::ground_snow_pa`, `part_1_4::na_b3_qp_pa` — same modules as `evaluate()`. Four tables, distinct en/de titles (`🧪️tests/🔬️unit/🦀️.rs:27–32`). `reference_tables_cells_match_imposed_and_evaluate_limit` (`:36–61`): B1 DE row = `part_1_1::imposed_qk_pa("B1", De)` = 2000 Pa; `en1991.1-1.imposed.b1` `limit.value` matches cell within 1e-9. |
| 3 | Duplicate entity ids → Fail (en+de, remedy `one_of`); compliance tests | **PASS** | `push_duplicate_ids` + `push_referential_integrity` (`💡️inferences/🦀️.rs:80–154`): localized en+de explanation, `Remedy::one_of` with sibling ids. Tests: `duplicate_floor_id_fails_integrity`, `duplicate_wind_face_id_fails_integrity`, `duplicate_roof_id_fails_integrity`, `duplicate_accidental_id_fails_integrity` (`compliance-report/🦀️.rs:629–671`). `push_dangling_ref` retained (`:117–145`, `#[allow(dead_code)]`) — no cross-ref leaves on subject today. |
| 4 | Diff + root artifact facets: `AccidentalCase={id,impact[],explosion[]}`; drop flat kind / `requiredDeltaT`; `structureKind` string enum; drop orphan `kind` field-meta | **PASS** | Snapshot + root + diff JSON/TS aligned: `AccidentalCase` required `id, impact, explosion` (`🧬️schema/🔣️.json:559–584`, `🔺️diff/🔣️.json:491–515`, `🟦️.ts:152`, `🔺️diff/🟦️.ts:154`). `structureKind: "building" \| "bridge"` (`🟦️.ts:81`). `rg requiredDeltaT\|"kind"` over `🧬️schema/*.{json,ts}` → **0** hits. Field-meta: `impact`/`explosion` group labels, no orphan `kind` (`🏷️field-meta/🦀️.rs:99–106`). Mutation diff uses `impact.first()` (`🧬️mutations/️change-accidental-assumed-force/🔺️diff/🦀️.rs:9–13`). |

---

## Round 4 — ADDENDA 14:54 / 14:42 / CORRECTION 13:27

| Requirement | Result | Evidence |
|-------------|--------|----------|
| `reference_tables()` not empty; shared const with `evaluate()`; test: one limit = table cell; distinct en/de | **PASS** | See R3 #2 above. |
| Dangling reference ids Fail (en+de, remedy `one_of`); duplicate entity ids Fail | **PASS** | See R3 #3 above. |
| No `let _ =` / fingerprints / field epsilons in evaluate/inference | **PASS** | `rg` over `💡️inferences/`: **0** `let _ =`, **0** `fingerprint`, **0** `1e-9 *` field folds. `1e-9` only in pass/fail tolerances (`:616–622`). |
| No identical en/de prose; no stale diff TS on accidental model | **PASS** | Dynamic explanations differ throughout `💡️inferences/🦀️.rs`. JSON/TS facets current (R3 #4). |
| Fire fields on fire example perturbed when in scope | **PASS** | Fire suite `de_fire_parametric_compliant()` with `fireMode=Parametric`; compartment/thermal leaves not in `fire_na`; `fireCurve` excluded on parametric path because evaluate hardcodes `FireCurve::Parametric` for h_net (`💡️inferences/🦀️.rs:238`) — correct scope N/A. |

---

## Round 4 — Brief checks 1–10 (summary)

| # | Check | Result | Notes |
|---|--------|--------|-------|
| 1 | Subject completeness | **PASS** | Hierarchical SI snapshot; bridge/fire/accidental discriminators wired. |
| 2 | Clause coverage | **PASS** | Parts 1-1…1-7, 2–4 with derived limits; selective bridge load group. |
| 3 | Numerics | **PASS** | `snow_zone2_150m_roof_mu08`, `imposed_category_b1_de`, oracle ±0.5%. |
| 4 | Applicability | **PASS** | Explicit N/A for empty lists, `fireMode=None`, `structureKind=Building` bridge block. |
| 5 | National annex | **PASS** | DE vs EN in compliance tests + partition factors. |
| 6 | Report quality | **PASS** | `[id=…]` paths, dual remedy test, localized en/de. |
| 7 | Examples | **PASS** | Compliant / non-compliant DSL + evaluate; bridge/fire/accidental committed examples. |
| 7b | Inputs UX | **PASS** | Structured editor + `field_meta_coverage_all_committed_examples`. |
| 8 | Mutations & schema | **PASS** | JSON/TS diff + snapshot facets aligned on discriminated accidental model; jsonschema test passes. |
| 9 | Tests | **PASS** | 79/79 green; perturbation, catalogue parity, duplicate-id integrity all exercised. |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in evaluate path. |

---

## Round 4 — CORRECTION 13:27 (twelve causes)

| # | Cause | R4 |
|---|-------|-----|
| 1 | Human en+de choice labels | **PASS** |
| 2 | Every editable leaf has meta (+ test) | **PASS** |
| 3 | Structured editor | **PASS** |
| 4 | `[id=…]` paths + resolve test | **PASS** |
| 5 | ≥2 fail→pass remedy tests | **PASS** |
| 6 | Example decode + verdict tests | **PASS** |
| 7 | Python oracle + third-party jsonschema | **PASS** |
| 8 | Facets regenerated, no loose typing | **PASS** (JSON/TS) |
| 9 | No tautologies / ignored fields | **PASS** |
| 10 | No trivial tests | **PASS** |
| 11 | Semantic mutation verbs | **PASS** |
| 12 | Dynamic localized issue text | **PASS** |

---

## Round 4 — CORRECTION 14:37 / 14:42 (perturbation gaming)

| Finding | R4 |
|---------|-----|
| Gaming instances in evaluate | **PASS** — CLEAN |
| Perturbation signature excludes explanation | **PASS** |
| Perturbation walks every nested editable leaf | **PASS** — recursive `collect_leaves` + all indices |
| Ratio slack / over-broad exemptions | **PASS** |
| Referential integrity on dangling / duplicate ids | **PASS** |

---

## Runner summaries

| Target | Summary |
|--------|---------|
| `@semio-tech/norm-en1991-rs:test` | `79 tests run: 79 passed, 0 skipped` |
| `@semio-tech/norm-artifact-contract-rs:test` | `51 tests run: 51 passed, 0 skipped` |

Logs: `🗑️generated/verify-en1991/` (Round 4).

---

## Blocking fix list

_None — all four Round-3 blockers closed._

---

## Non-blocking observations

- GraphQL/proto facets (`🔗️.graphql`, `🛰️.proto`) still carry legacy `requiredDeltaT` and `[JSON!]` entity blobs; JSON/TS facets (R3 scope) are current. Regenerate when next facet sweep runs.
- Perturbation test silently `continue`s on `assign_at` failure / serde decode failure (contrast en1990 `unchanged.is_empty()` inventory); no in-scope leaf found unassignable on the four committed examples.
- Duplicate-id tests assert `Fail` status only; do not assert en+de explanation text or `one_of` remedy payload (implementation provides both).
- `bridgeLoadGroup` check still emits all LM constituent checks for bridge examples — verbose but not incorrect.
- Catalogue column labels differ en/de (`Category` / `Kategorie`) but only table titles are asserted distinct in unit test.
