# Verify — EN 1990 (`en1990` ⚖️) — Round 5

**Verifier:** adversarial read-only (Wave D)  
**Date:** 2026-09-26  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Implementer claim:** `📓️impl-en1990.md` (136/136 tests, no gaps)  
**Test log:** `🗑️generated/verify-en1990/test-r5.txt`  
**Contract log:** `🗑️generated/verify-en1990/contract-test-r5.txt`  
**Hand numerics:** `🗑️generated/verify-en1990/hand-numerics.txt` (carried from R4; still valid)

## Round 1 history

Round 1 (`2026-09-26`): **FAIL (12 blocking)** — effects-only action wiring, missing GEO/EQU-stab/design-life checks, tautological K_FI, wrong SLS clause IDs, ψ category mismatch, missing remedy-law & example-verdict tests, skipped jsonschema, no Annex A2, no favourable G_inf in STR/GEO, identical en/de copy. Tests: `104 passed, 0 skipped`.

## Round 2 history

Round 2 (`2026-09-26`): **FAIL (5 blocking)** — `projectId` orphan; no `every_editable_leaf` meta test; identical en/de STR/GEO/accidental/seismic copy; Annex A2.3 mis-scoped as γ_M (coordinator corrected to A2.4 action γ); Table 2.1 bridge category remedy gap. Tests: `114 passed, 0 skipped`.

## Round 3 history

Round 3 (`2026-09-26`): **FAIL (3 blocking)** — no perturb-each-editable-leaf test; bridge SLS member fields unread on default building snapshot; `variables[].altitudeM` per-variable instead of site `altitudeM`. Tests: `118 passed, 0 skipped`.

## Round 4 history

Round 4 (`2026-09-26`): **FAIL (1 blocking)** — no committed example with `accidentals[]` / `seismics[]`; perturb test scope 4 missing; `accidentals[].ad` / `seismics[].aEd` never walked. Tests: `125 passed, 0 skipped`.

## VERDICT: FAIL (1 blocking)

**Test Summary (family):** `136 tests run: 136 passed, 0 skipped` (`--skip-nx-cache -- --no-fail-fast`)  
**Test Summary (contract):** `48 tests run: 48 passed, 0 skipped` (`--skip-nx-cache`)

---

## Check table

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical subject: project → action catalogues (`permanents`/`variables`/`accidentals`/`seismics`) → `members` + `effects[]` wiring → optional `bridgeSls[]`. `SeismicAction.a_ed()` = γ_I·A_Ek (`🦀️.rs:82–97`). Site `altitudeM` on snapshot (`📸️snapshot/🦀️.rs:24`). |
| 2 | Clause coverage | **PASS** | ULS STR/GEO/EQU (+stab), SLS 6.14b–6.16b, 6.11/6.12b, FAT, Annex A1.4, Annex B, Annex C β, Table 2.1, bridge A2.4 SLS. NDP A1.3.1(4) max(6.10a,6.10b) in evaluate loop (`💡️inferences/🦀️.rs:195`). |
| 3 | Numerics | **PASS** | Hand derivations in `🗑️generated/verify-en1990/hand-numerics.txt` (6 cases ±0.5 %). Accidental 6.11 oracle 165 N (`🧪️tests/⚖️compliance/🦀️.rs:76`). Seismic compliant A_Ed = 1.0×60 kN = 60 kN; failing A_Ed = 1.2×150 kN = 180 kN (`📚️examples/🏢️accidental-seismic-*/🦀️.rs`). |
| 4 | Applicability | **PASS** | Explicit `NotApplicable` + en/de for empty members, no A_d/A_Ed, FAT rdFat≤0, vibration min≤0, non-bridge A2, missing bridgeSls row (`💡️inferences/🦀️.rs:262–320, 420–477, 691–725`). Scope 4 asserts 6.11/6.12b return N/A when tables cleared (`🧪️tests/🔬️compliance-report/🦀️.rs:629–636`). |
| 5 | National annex DE vs EN | **PASS** | `en_combination_6_10a_differs_on_other_psi` 246 vs 241.5 (`🧪️tests/⚖️compliance/🦀️.rs:34–42`). `de_snow_high_altitude_psi` (`:46–51`). `seismic_combination_de_vs_en_diverge_on_other_psi_2` 165 vs 155 (`:105–110`). Bridge A2.4(B) γ_Q rail DE 1.40 vs EN 1.45 (`🧪️tests/🔬️compliance-report/🦀️.rs:228–245`). |
| 6 | Report quality | **PASS** | `[id=…]` paths + resolve test (`🧪️tests/🔬️compliance-report/🦀️.rs:60–76`). Remedy-law β + deflection (`:91–116`); Table 2.1 category (`:205–224`). Distinct en/de: `str_title`/`geo_title` (`💡️inferences/🦀️.rs:130–141`); accidental/seismic (`:269, 286, 316`). |
| 7 | Examples | **PASS** | Default + high-consequence comply/fail (`:14–28`). Bridge `road_bridge_compliant_subject_complies` / `road_bridge_failing_fails_with_multiple_checks` (`:262–274`). Accidental/seismic `accidental_seismic_compliant_activates_6_11_and_6_12b` / `accidental_seismic_failing_fails_6_11_or_6_12b` (`:277–294`). Fatigue compliant/failing (`:297–311`). |
| 7b | Inputs UX | **PASS** | `every_editable_leaf_has_en_de_field_meta` incl. `accidentals[].ad`, `seismics[].aEk`, `seismics[].importanceClass` (`✏️editor/🏷️field-meta/🧪️tests/🔬️unit/🦀️.rs:28–34`). Structured editor (`✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs:20`). |
| 8 | Mutations & schema | **PASS** | Semantic mutations incl. insert/remove accidental/seismic; facets 🦀️/🟦️/🔗️/🔣️/🛰️ regenerated. Snapshot `🟦️.ts` typed interfaces (`🧬️schema/🟦️.ts:5–84`); `Record<string, unknown>` only in text-guard helpers. |
| 9 | Tests | **FAIL** | `136 executed / 136 passed / 0 skipped`. Python oracle + jsonschema (`🧪️tests/🔬️compliance-report/🦀️.rs:135–195`; `🚪️io/🧪️tests/🔬️unit/🦀️.rs:64–80`). Perturb scope 4 present (`:587–637`). **Gap:** `is_exempt` still skips reference-id leaves `id`, `memberId`, `actionId` — violates ADDENDUM/CORRECTION 14:42 (`:322–329`). |
| 10 | Stubs | **PASS** | `rg todo!\|unimplemented!\|stub\|dummy\|0.0 /\*` in family tree: no runtime stubs. |

---

## Round-4 blocking item (re-check)

| R4 item | R5 status | Evidence |
|---------|-----------|----------|
| Commit accidental/seismic example + perturb scope 4 | **FIXED** | Examples `📚️examples/🏢️accidental-seismic-compliant/🦀️.rs` (A-impact ad=50 kN, E-1 aEk=60 kN + effects wiring) and `🏢️accidental-seismic-failing/🦀️.rs`. Scope 4 perturbs `accidentals[id=…].ad`, `seismics[id=…].aEk`, `seismics[id=…].importanceClass`; asserts 6.11/6.12b active then N/A when cleared (`🧪️tests/🔬️compliance-report/🦀️.rs:587–637`). Note: design field is `aEk` (derived `a_ed()`), not stored `aEd` — acceptable. |
| Bridge example verdict tests | **FIXED** | `road_bridge_compliant_subject_complies`, `road_bridge_failing_fails_with_multiple_checks` (`:262–274`). |

---

## CORRECTION 13:27 self-check

| # | Item | Result | Evidence |
|---|------|--------|----------|
| 1 | NormFieldChoice human en+de | **PASS** | Choice label≠value assert (`🏷️field-meta/🧪️tests/🔬️unit/🦀️.rs:71–75`). |
| 2 | Every editable leaf meta + test | **PASS** | Wildcard table incl. accidental/seismic (`🏷️field-meta/🧪️tests/🔬️unit/🦀️.rs:28–34`). |
| 3 | Structured editor (not JSON dump) | **PASS** | `render_document_editor` (`📥️inputs/🦀️.rs:20`). |
| 4 | `[id=…]` + resolve test | **PASS** | `every_emitted_subject_path_parses_and_resolves` (`🔬️compliance-report/🦀️.rs:60–76`). |
| 5 | ≥2 remedy-law tests | **PASS** | β + deflection (`:91–116`); Table 2.1 (`:205–224`). |
| 6 | Example DSL + complies / fail≥2 | **PASS** | Six example pairs with verdict tests (`:14–44, 262–311`). |
| 7 | Python oracle + jsonschema | **PASS** | Oracle ±0.5 % (`:135–195`); jsonschema via python3 (`🚪️io/🧪️tests/🔬️unit/🦀️.rs:64–80`). |
| 8 | Facets regenerated | **PASS** | Typed `En1990Artifact` (`🧬️schema/🟦️.ts`); no `_placeholder`. |
| 9 | No tautology; wired inputs | **PASS** | Accidental/seismic leaves normatively read via `action_set_for_member` (`💡️inferences/🦀️.rs:183–184`). |
| 10 | No trivial asserts | **PASS** | Numeric asserts use concrete values (e.g. 237.0 N, 165 N); no `|| true`. |
| 11 | Semantic mutations | **PASS** | `insert-accidental`, `insert-seismic`, `change-altitude-m`, etc. |
| 12 | Localized dynamic text | **PASS** | `str_title`/`geo_title`/accidental/seismic distinct en/de (`💡️inferences/🦀️.rs:130–141, 269, 316`). |

---

## ADDENDUM 14:42 / CORRECTION 14:37 perturbation audit

| Item | Result | Evidence |
|------|--------|----------|
| Gaming grep (`fingerprint`, `1e-9 *`, `let _ =`) in evaluate/inference | **PASS** | No matches in `💡️inferences/🦀️.rs` or `🧬️schema/🦀️.rs`. `f64::EPSILON` only for divide-by-zero / range guards (`💡️inferences/🦀️.rs:262, 598, 838`). |
| Signature excludes explanation text | **PASS** | `(id, status, round(util×1e6), round(computed×1e3), annex)` (`🔬️compliance-report/🦀️.rs:478–491`). |
| No ratio slack | **PASS** | `assert!(unchanged.is_empty())` — no `unchanged.len() * 2 < n` (`:640`). |
| Full nested leaf walk | **PASS** | Recursive `walk()` all array items/depths (`:331–355`). |
| Scope-aware (not whole-subtree skip) | **PASS** | Four scopes: building, DE-snow altitude, bridge `bridgeSls[]`, accidental/seismic (`:503–637`). Building skip of `bridgeSls`/`altitudeM` covered in dedicated scopes — not an exemption. |
| Reference-id perturbation (14:42) | **FAIL** | `is_exempt` exempts `id`, `memberId`, `actionId` (`:322–329`). ADDENDUM 14:42 allows only descriptive `id`/`name`/`title`/`labelEn`/`labelDe`; `effects[].actionId`, `effects[].memberId`, `bridgeSls[].memberId`, catalogue entity `id` are reference keys wired in `influence_for` (`💡️inferences/🦀️.rs:152–154`). Dangling perturbation (`format!("{other}-x")` at `:469`) already exists but is never applied to these leaves. |
| Audit `📓️audit-perturbation-gaming.md` ⚖️ en1990 | **FAIL** | Audit lists same exemptions (`:99–101`); unresolved per ADDENDUM 14:42. |

---

## Blocking fix list

1. **`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — reference-id perturbation (ADDENDUM/CORRECTION 14:42)**  
   Narrow `is_exempt()` (`:322–329`) to descriptive labels only: keep `projectId`, `labelEn`, `labelDe` (and entity display `name`/`title` if present). **Remove** blanket exemptions for leaf `id`, `memberId`, and `actionId`. For each reference-id leaf encountered by `walk()`, perturb to a dangling value (reuse the existing `format!("{other}-x")` branch in `perturb()` at `:469`) and assert `sig()` changes — e.g. `effects[id=…].actionId` "G-sup" → "G-sup-x" zeros permanent influence and flips STR utilization; `bridgeSls[id=…].memberId` dangling drops A2.4 SLS checks to `NotApplicable`. Do **not** add new exemption lists or ratio slack.

---

## Non-blocking observations

- **136/136 green**; contract crate 48/48 green.
- Round-4 accidental/seismic gap fully closed (examples + scope 4 + N/A flip assertion).
- Fatigue examples (`fatigue-compliant` / `fatigue-failing`) and verdict tests added beyond minimum DoD.
- `evaluate_marks_seismic_not_applicable_when_no_a_ek` (`:48–56`) uses weak tri-state assert — could tighten to require `NotApplicable` on default empty seismics.
- `parseEn1990Artifact` in TS is identity cast (`🧬️schema/🟦️.ts:88–89`) — runtime validation delegated to jsonschema test; acceptable for greenfield.
- GEO set C now reads `structure_kind` (`impl-en1990.md` Round 4 C) — not re-derived here; tests green.
