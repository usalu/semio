# Verify — DIN EN 16798 (`din16798` / 🌬️)

**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Ticket:** `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/`

---

## Round 1 (2026-09-26)

**VERDICT: FAIL (11 blocking)** — wire bugs, index paths, smoke mutation tests, weak remedies.  
Full table: git history of this file at Round 1 close.

---

## Round 2 (2026-09-26)

**Verifier:** Wave D adversarial read-only audit  
**Impl claim:** `📓️impl-din16798.md` — 73/73 tests passed  
**Test run:** `bun nx run @semio-tech/norm-din16798-rs:test --skip-nx-cache -- --no-fail-fast`  
→ **`Summary [0.833s] 73 tests run: 73 passed, 0 skipped`** (saved: `🗑️generated/verify-din16798/test.txt`)

**VERDICT: FAIL (2 blocking)**

### Round-2 blocking items

| # | Item | R2 | Evidence |
|---|------|-----|----------|
| 1 | Wire `zone.ventSystemId` into normative checks | **FAIL** | Editable but never read in `evaluate()` |
| 2 | CORRECTION 13:43 perturbation test | **FAIL** | No leaf-walk perturb test in family |

### Round-2 check table (abbrev.)

| # | Check | R2 |
|---|-------|-----|
| 1 | Subject completeness | FAIL (`ventSystemId` dead wire) |
| 2 | Clause coverage | PASS |
| 3 | Numerics | PASS |
| 4 | Applicability | PASS |
| 5 | National annex | PASS |
| 6 | Report quality | PASS |
| 7 | Examples | PASS |
| 7b | Inputs UX | PASS |
| 8 | Mutations & schema | PASS |
| 9 | Tests | FAIL (missing perturb test) |
| 10 | Stubs | PASS |

Round-2 blocking fix list and non-blocking observations preserved in git history at Round 2 close.

---

## Round 3 (2026-09-26)

**Verifier:** Wave D adversarial read-only audit (Round 3)  
**Impl claim:** `📓️impl-din16798.md` — round-2 fixes landed; 79/79 tests  
**Test runs (this audit):**

| Runner | Command | Result |
|--------|---------|--------|
| Family | `bun nx run @semio-tech/norm-din16798-rs:test --skip-nx-cache -- --no-fail-fast` | **`Summary [0.996s] 79 tests run: 79 passed, 0 skipped`** |
| Contract | `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **`Summary [0.188s] 51 tests run: 51 passed, 0 skipped`** |

Saved: `🗑️generated/verify-din16798-r3/runner-summary.txt`

**VERDICT: FAIL (5 blocking)**

---

### Round-3 per-item table

| Item | Evidence (file:line or test) | R3 |
|------|------------------------------|-----|
| **R2-1** Wire `zone.ventSystemId` → normative check change | `evaluate_zone` resolves link L713–760; `evaluate_vent` capacity `q_served` L1090–1120; tests `two_vent_relink_changes_capacity_on_both_systems`, `dangling_vent_system_id_fail_apply_remedy_to_pass` | **PASS** |
| **R2-2** Perturbation over editable leaves, signature `(id, status, computed, limit, utilization)` | `editable_leaves_perturb_at_least_one_check_across_examples` L544–583; `report_fingerprint` L469–474; `collect_leaf_paths` walks all array items L154–191; examples: compliant-two-vent, noncompliant, residential-method3, adaptive-office; exempt `id`/`name` only L546 | **PASS** |
| **R2-2b** Dangling reference → Fail + en/de + `one_of` remedy | `dangling_vent_system_id_fail_apply_remedy_to_pass` L429–444; integrity check L717–760 | **PASS** |
| **Gaming** θ_rm fingerprint comment + N/A utilization on fixed-HVAC | `🧬️schema/🦀️.rs` L855–866: comment "changes the report fingerprint"; `.utilization(theta_rm_c, centre_na)` on `NotApplicable` adaptive slot while `comfortModel=fixed_hvac` | **FAIL** |
| **Gaming** adaptive PMV/PPD N/A dummy utilization (clothing/met) | `🧬️schema/🦀️.rs` L823–835: `.utilization(clo+met, …)` on N/A PMV/PPD checks | **FAIL** |
| **Gaming** cellar ventilation dummy utilization when area=0 | `🧬️schema/🦀️.rs` L1336–1349: `.status(Pass).utilization(q_cellar, …)` when `cellarAreaM2≤0` | **FAIL** |
| **Gaming grep** `let _ =` in evaluate/inference | Only test/fs helpers in compliance tests (`🧪️tests/⚖️compliance/🦀️.rs` L204, L337, L368); none in `evaluate_*` | **PASS** |
| **Gaming grep** `fingerprint` / epsilon folds in evaluate | `fingerprint` only in test helper name L469; no `1e-9 *` / `1e-12 *` / `* 1e-` in evaluate (divide guards `max(1e-9)` only) | **PASS** (evaluate); test helper name `report_fingerprint` is OK |
| **Identical en/de prose** | Spot-check: no identical literal `loc("…","…")` pairs; numeric templates differ in German engineering terms (e.g. L774–775, L1109–1110) | **PASS** |
| **Facet parity** — no `unknown` / loose objects in schema facets | `🧬️schema/🟦️.ts` L5–6: `zones: unknown[]; ventSystems: unknown[]` while `📸️snapshot/🟦️.ts` L49–50 is fully typed | **FAIL** |
| **Facet parity** — no `Record<string, unknown>` in generated TS | Text guard facets (`🧬️schema/📸️snapshot/📝️text/🟦️.ts` L20–21, mutations/diff/inference guards) retain `Record<string, unknown>` | **FAIL** (non-regenerated guards; see fix list) |
| **Skip hatches** in python/jsonschema tests | `🐍️.py` and `validate_snapshot.py`: no `@pytest.mark.skip`, no conditional bypass; `jsonschema_validates_compliant_and_noncompliant_snapshots` L364–379 asserts exit success | **PASS** |
| **ODA4** in choices + filter table or documented exclusion | `🏷️field-meta/🦀️.rs` L142–150: ODA1–3 only; `required_filter_for_oda` L643–648: unknown → ODA2 default; no impl clause cite for ODA4 exclusion | **FAIL** |
| **CORRECTION 13:27 #1** Human en+de choice labels | `🏷️field-meta/🦀️.rs` L39–178 | **PASS** |
| **CORRECTION 13:27 #2** Editable leaf meta + coverage test | `default_snapshot_editable_leaves_have_en_de_field_meta` L195–255 | **PASS** |
| **CORRECTION 13:27 #3** Structured inputs editor | `📥️inputs/🧪️tests/🔬️unit/🦀️.rs` `renders_structured_editor_not_json_dump` L11–18 | **PASS** |
| **CORRECTION 13:27 #4** `[id=…]` paths + resolve test | `every_emitted_subject_path_parses_and_resolves_on_default_and_noncompliant` L129–152 | **PASS** |
| **CORRECTION 13:27 #5** ≥2 fail→pass remedy tests | `apply_remedy_flips_vent_fail_to_pass` L295–304; `apply_remedy_flips_sfp_fail_to_pass` L307–316; `apply_remedy_flips_filter_fail_to_pass_via_oneof` L319–331 | **PASS** |
| **CORRECTION 13:27 #6** DSL examples decode + evaluate | `bundled_dsl_examples_decode_and_evaluate` L384–391 | **PASS** |
| **CORRECTION 13:27 #7** Python oracle ±0.5 % + jsonschema | `python_oracle_matches_rust_q_sfp_co2_within_half_percent` L335–360; `jsonschema_validates_compliant_and_noncompliant_snapshots` L364–379 | **PASS** |
| **CORRECTION 13:27 #8** Facets regenerated / parity | Snapshot JSON/proto/GraphQL typed; aggregate `🧬️schema/🟦️.ts` stale (`unknown[]`) | **FAIL** |
| **CORRECTION 13:27 #9** No tautologies / dead wires | Gaming dummy binds (θ_rm, clo/met, cellar) keep leaves “read” without normative pass/fail effect | **FAIL** |
| **CORRECTION 13:27 #10** No trivial tests | Perturbation, numeric, remedy tests assert concrete values/status | **PASS** |
| **CORRECTION 13:27 #11** Semantic mutation verbs | `change-zone-co2`, `change-vent-design-airflow`, `insert-vent-system`, … (40 kinds) | **PASS** |
| **CORRECTION 13:27 #12** Distinct en/de dynamic labels | Entity labels `Zone:`/`Raumzone:`; vent integrity en/de explanations L732–733 | **PASS** |
| **Brief #1** Subject completeness | Hierarchical zones + ventSystems; typed enums; `designAirflowM3H` L108; linked capacity | **PASS** |
| **Brief #2** Clause coverage | Parts -1/-3/-5-1/-7/-17 checks present; draught L1051+; vent methods L565–587; method-3 example | **PASS** |
| **Brief #3** Numerics | Hand checks unchanged from R2 (q=1008, SFP class 3=1250, adaptive θ_c=23.75 @ θ_rm=15) | **PASS** |
| **Brief #4** Applicability | Adaptive/fixed gates; HR N/A natural; capacity N/A unlinked vent | **PASS** |
| **Brief #5** National annex | DE CO₂ + acoustic tighter; `de_annex_diverges_from_en_on_same_subject` | **PASS** |
| **Brief #6** Report quality | Paths v1.2; localized titles; applicable remedies | **PASS** |
| **Brief #7** Examples | compliant + noncompliant DSL; multi-fail ≥5 | **PASS** |
| **Brief #7b** Inputs UX | Full field-meta incl. `designAirflowM3H`, `ventMethod`, `turbulenceIntensityPercent` | **PASS** |
| **Brief #8** Mutations & schema | 40 mutation kinds + behavioral suite; aggregate TS facet stale | **FAIL** (facet) |
| **Brief #9** Tests | 79 executed / 0 skipped | **PASS** |
| **Brief #10** Stubs | No `todo!`/`unimplemented!` in family evaluate path | **PASS** |
| **E2E verbs** setField / insertItem / removeItem / applyRemedy | `✏️editor/🦀️.rs` L48–51, L255–285 | **PASS** |

---

### Round-3 standard check table

| # | Check | R3 | Evidence |
|---|-------|-----|----------|
| 1 | Subject completeness | **PASS** | Linked vent capacity + integrity; SI snapshot |
| 2 | Clause coverage | **PASS** | Full catalogue; ODA4 gap tracked separately |
| 3 | Numerics | **PASS** | Worked examples + oracle |
| 4 | Applicability | **PASS** | N/A gates with localized reasons |
| 5 | National annex | **PASS** | DE vs EN tests |
| 6 | Report quality | **PASS** | Paths, remedies, en/de titles |
| 7 | Examples | **PASS** | compliant + noncompliant + method3 + adaptive perturb paths |
| 7b | Inputs UX | **PASS** | Structured editor + field-meta |
| 8 | Mutations & schema | **FAIL** | `🧬️schema/🟦️.ts` `unknown[]`; guard `Record<string, unknown>` |
| 9 | Tests | **PASS** | 79/79 family; 51/51 contract |
| 10 | Stubs | **PASS** | Clean evaluate tree |

---

## Blocking fix list (Round 3)

1. **`🧬️schema/🦀️.rs` L854–866 (`evaluate_zone`, fixed-HVAC branch)** — Remove the dummy `NotApplicable` adaptive check that binds `doc.theta_rm_c` into `.utilization(computed=θ_rm, limit=θ_c)`. On `comfortModel=fixed_hvac`, Annex B.2 adaptive comfort is irrelevant: emit a plain `NotApplicable` without computed/limit/utilization derived from `thetaRmC`, or make `thetaRmC` non-editable on fixed-HVAC subjects (derive from climate service) and cite EN 16798-1 Annex B.2 applicability in `📓️impl-din16798.md`. Delete the "report fingerprint" comment.

2. **`🧬️schema/🦀️.rs` L817–835 (`evaluate_zone`, adaptive branch)** — Remove `.utilization(clothing_clo + metabolic_rate_met, …)` on N/A PMV/PPD slots. N/A checks must not fold editable leaves into utilization when the clause does not apply; clothing/met already affect adaptive deviation only through the real adaptive check when applicable, or through PMV when `fixed_hvac`.

3. **`🧬️schema/🦀️.rs` L1336–1349 (`evaluate_envelope`)** — When `cellarAreaM2≤0`, stop emitting a `Pass` check whose utilization echoes `cellarVentilationM3H`. Use `NotApplicable` without utilization tied to the ventilation rate, or gate `cellarVentilationM3H` editability to subjects with cellar area > 0 per EN 16798-7 §6.2.

4. **`🧬️schema/🟦️.ts` (regenerate from Rust snapshot types)** — Replace `zones: unknown[]` / `ventSystems: unknown[]` with the typed `Din16798Zone[]` / `Din16798VentSystem[]` already present in `📸️snapshot/🟦️.ts`; regenerate text-guard facets to drop exported `Record<string, unknown>` surface where the pipeline allows typed guards.

5. **`🏷️field-meta/🦀️.rs` L142–150 + `part_3::required_filter_for_oda` L643–648** — Add ISO 16890-1 **ODA4** to `odaClass` choices and the ODA→SUP filter mapping used by `din16798-3.filter.*`, **or** document in `📓️impl-din16798.md` that ODA4 process/outdoor air is outside the assessed DIN EN 16798-3 scope with the exact clause reference and remove `odaClass` from editable leaves for this subset.

---

## Non-blocking observations

- Perturbation test currently passes partly because gaming binds on `thetaRmC` (default fixed-HVAC office), `clothingClo`/`metabolicRateMet` (adaptive-office example), and `cellarVentilationM3H` (default cellarArea=0) inflate the fingerprint without normative effect; fixing items 1–3 will tighten the test’s meaning.
- `report_fingerprint` helper name in tests (L469) is fine; it hashes `(id, status, computed, limit, utilization)` only — no explanation text.
- Fan-flow check uses `q_design * (1.0 + 1e-6)` tolerance (L1131) — divide guard, not field gaming.
- Catalogue panel test still only asserts body contains `"catalogue"` — weak but wired.
- SFP class choice labels share numeric bounds in en/de (symbols/numbers exception applies).

---

## Numerics appendix

Round 2 hand derivations unchanged; see `🗑️generated/verify-din16798/hand-numerics.txt`.
