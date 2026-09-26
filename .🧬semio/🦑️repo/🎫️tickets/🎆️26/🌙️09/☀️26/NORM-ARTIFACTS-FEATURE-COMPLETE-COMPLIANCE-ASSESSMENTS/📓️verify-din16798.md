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

---

## Round 4 (2026-09-26)

**Verifier:** Wave D adversarial read-only audit (Round 4)  
**Impl claim:** `📓️impl-din16798.md` — round-3 fixes landed; 82/82 tests  
**Test runs (this audit):**

| Runner | Command | Result |
|--------|---------|--------|
| Family | `bun nx run @semio-tech/norm-din16798-rs:test --skip-nx-cache -- --no-fail-fast` | **`Summary [3.485s] 82 tests run: 82 passed, 0 skipped`** |
| Contract | `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **`Summary [0.411s] 51 tests run: 51 passed, 0 skipped`** |

Saved: `🗑️generated/verify-din16798-r4/runner-summary.txt`

**VERDICT: FAIL (1 blocking)**

---

### Round-4 per-item table (Round-3 re-verify)

| Item | Evidence (file:line or test) | R4 |
|------|------------------------------|-----|
| **1** Fixed-HVAC adaptive Annex B.2 → plain `NotApplicable`; no `thetaRmC` in computed/limit/utilization; no fingerprint comment; adaptive path uses `thetaRmC` | `🧬️schema/🦀️.rs` L852–860: `.not_applicable(...)` on `comfortModel` only; L834 `adaptive_comfort_temperature_c(doc.theta_rm_c)` on adaptive branch; no "fingerprint" comment in evaluate; `adaptive_model_runs_annex_b2_and_skips_pmv_ppd` | **PASS** |
| **2** N/A PMV/PPD slots do not fold `clothingClo`/`metabolicRateMet` into utilization; real PMV uses them under fixed-HVAC | `🧬️schema/🦀️.rs` L825–832: plain `.not_applicable(...)` for four PMV/PPD slots; L865–891 `pmv_iso7730(..., zone.metabolic_rate_met, clo_summer)` under fixed-HVAC | **PASS** |
| **3** `cellarAreaM2≤0` → cellar ventilation `NotApplicable` without echoing `cellarVentilationM3H`; positive area → EN 16798-7 §6.2 check | `🧬️schema/🦀️.rs` L1330–1351; `noncompliant_office()` `cellar_area_m2: 40.0` (`📸️snapshot/🦀️.rs` L132–133); perturb scope `leaf_applies_in_example` L584 | **PASS** |
| **4** `🧬️schema/🟦️.ts` typed `Din16798Zone[]` / `Din16798VentSystem[]`; no exported `Record<string, unknown>` / `_placeholder` | `🧬️schema/🟦️.ts` L1–10; test `schema_ts_facets_forbid_unknown_record_and_placeholder` L688–716 (scans all schema `*.ts`) | **PASS** |
| **5** ODA4 field-meta choice + stricter `required_filter_for_oda` than ODA3 + test | `🏷️field-meta/🦀️.rs` L150; `part_3::required_filter_for_oda` L648 → `ePM1_80_G` rank 5; test `oda4_requires_stricter_filter_than_oda3` L648–656 | **PASS** |
| **6** SFP catalogue class-3 cell = `part_3::sfp_bound(3)` = SFP check limit | `📚️catalogue/🦀️.rs` L42 `sfp_bound(class)`; test `sfp_catalogue_class3_matches_sfp_bound_and_check_limit` L660–684 | **PASS** |
| **7** Perturbation signature `(id, status, computed, limit, utilization)`; dangling `ventSystemId` Fail + en/de + `one_of`; no gaming greps; **duplicate entity ids Fail** | `report_fingerprint` L469–474; `dangling_vent_system_id_fail_apply_remedy_to_pass` L429–444; `editable_leaves_perturb_at_least_one_check_across_examples` L590–645; no `fingerprint`/`1e-9 *`/`let _ =` in `🧬️schema/🦀️.rs` evaluate or `💡️inferences/🦀️.rs`; **no duplicate-id integrity check** in `check_full_environment` L687–704 (contrast `en1997` `push_duplicate_ids` / `din4108` `push_duplicate_ids`) | **FAIL** |
| **8** `ventSystemId` normative wiring; no identical en/de prose; no skip hatches; CORRECTION 13:27 #1–#12 | `evaluate_zone` L715–760 + `evaluate_vent` L1084–1114; tests `two_vent_relink_changes_capacity_on_both_systems`, `dangling_vent_system_id_fail_apply_remedy_to_pass`; spot-check distinct `loc` en/de (e.g. L733–735, L1108–1110); no `@pytest.mark.skip` / `#[ignore]` in family tests; CORRECTION 13:27 table in Round 3 all **PASS** — still holds | **PASS** |

---

### Round-4 standard check table

| # | Check | R4 | Evidence |
|---|-------|-----|----------|
| 1 | Subject completeness | **PASS** | Hierarchical zones + ventSystems; linked capacity + integrity |
| 2 | Clause coverage | **PASS** | Parts -1/-3/-5-1/-7/-17; ODA4 mapped |
| 3 | Numerics | **PASS** | Worked examples + oracle unchanged |
| 4 | Applicability | **PASS** | Plain N/A gates; scope-aware perturb examples |
| 5 | National annex | **PASS** | `de_annex_diverges_from_en_on_same_subject` |
| 6 | Report quality | **FAIL** | Missing duplicate-id Fail checks + remedies (CORRECTION 14:42) |
| 7 | Examples | **PASS** | compliant + noncompliant + method3 + adaptive perturb paths |
| 7b | Inputs UX | **PASS** | ODA4 choice; structured editor + field-meta |
| 8 | Mutations & schema | **PASS** | Typed aggregate TS; facet parity test |
| 9 | Tests | **PASS** | 82/82 family; 51/51 contract; 0 skipped |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in evaluate path |
| Gaming audit | **PASS** | Round-3 dummy binds removed; evaluate/inference clean |
| Catalogue tables | **PASS** | `reference_tables()` non-empty; SFP cell = `sfp_bound` |

---

## Blocking fix list (Round 4)

1. **`🧬️schema/🦀️.rs` `check_full_environment` (before zone/vent loops, ~L687)** — Add referential-integrity checks that **Fail** when `zones[].id` or `ventSystems[].id` appears more than once (pattern: `din4108` `push_duplicate_ids` in `💡️inferences/🦀️.rs` L90–119 or `en1997` `push_duplicate_ids` L1115–1171). Each duplicate must emit localized en+de explanation, `SubjectRef` on `zones[id=<id>].id` / `ventSystems[id=<id>].id`, and an applicable remedy (`Rename … to a free id` or `one_of` unused ids). Add tests `duplicate_zone_id_fails_integrity` and `duplicate_vent_system_id_fails_integrity` in `🧪️tests/⚖️compliance/🦀️.rs` asserting `CheckStatus::Fail` and remedy applicability.

---

## Non-blocking observations (Round 4)

- `📓️audit-perturbation-gaming.md` din16798 section still lists the Round-2 gaming instances (L39–40); code is fixed — update audit doc on next coordinator pass.
- `report_fingerprint` test helper name (L469) is acceptable; signature excludes explanation text.
- Fan-flow tolerance `q_design * (1.0 + 1e-6)` (L1125) is a divide/numerics guard, not field epsilon gaming.
- Perturbation harness correctly scope-skips `thetaRmC` on fixed-HVAC-only examples, `clothingClo` on adaptive, `cellarVentilationM3H` when `cellarAreaM2≤0`, and asserts each is covered in a committed applicable example (L642–644).
- Catalogue panel unit test still weak (body contains `"catalogue"` only) but tables are populated and SFP parity is asserted.

---

## Round 5 (2026-09-26)

**Verifier:** Wave D adversarial read-only audit (Round 5)  
**Impl claim:** `📓️impl-din16798.md` — round-4 duplicate-id fix landed; 84/84 tests  
**Test runs (this audit):**

| Runner | Command | Result |
|--------|---------|--------|
| Family | `bun nx run @semio-tech/norm-din16798-rs:test --skip-nx-cache -- --no-fail-fast` | **`Summary [2.587s] 84 tests run: 84 passed, 0 skipped`** |
| Contract | `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **`Summary [0.676s] 51 tests run: 51 passed, 0 skipped`** |

Saved: `🗑️generated/verify-din16798-r5/runner-summary.txt`

**VERDICT: PASS**

---

### Round-5 per-item table

| Item | Evidence (file:line or test) | R5 |
|------|------------------------------|-----|
| **R4 blocker** Duplicate `zones[].id` / `ventSystems[].id` Fail in `check_full_environment` | `push_duplicate_ids` L689–758; called L763–780 before zone/vent loops; `CheckStatus::Fail` L746; en+de explanations L742–745 (distinct); `SubjectRef` paths `zones[id={id}].id` / `ventSystems[id={id}].id` L770/L779; `Remedy::one_of` unused ids L747–754 | **PASS** |
| **R4 blocker tests** `duplicate_zone_id_fails_integrity` / `duplicate_vent_system_id_fails_integrity` | L447–485: `CheckStatus::Fail`, subject path on `.id`, applicable `OneOf`, options exclude duplicate id, `explanation.en ≠ explanation.de` | **PASS** |
| **1** Fixed-HVAC adaptive → plain `NotApplicable`; no `thetaRmC` bind | `🧬️schema/🦀️.rs` L943–951 plain `.not_applicable(...)` on `comfortModel`; adaptive branch L925 uses `theta_rm_c` only when `ComfortModel::Adaptive` | **PASS** (no regression) |
| **2** N/A PMV/PPD do not fold clothing/met into utilization | L916–923 plain `.not_applicable(...)`; fixed-HVAC PMV uses `clo_summer`/`metabolic_rate_met` L956+ | **PASS** (no regression) |
| **3** `cellarAreaM2≤0` → cellar ventilation `NotApplicable` without `cellarVentilationM3H` utilization | L1421–1430 `.not_applicable(...)` on `cellarAreaM2`; positive area path L1432+ | **PASS** (no regression) |
| **4** Typed `Din16798Zone[]` / `Din16798VentSystem[]` aggregate TS facets | `🧬️schema/🟦️.ts` L1–10; `schema_ts_facets_forbid_unknown_record_and_placeholder` L730+ | **PASS** (no regression) |
| **5** ODA4 field-meta + stricter `required_filter_for_oda` than ODA3 | `🏷️field-meta/🦀️.rs` L150; `required_filter_for_oda` L649 → `ePM1_80_G` rank 5; `oda4_requires_stricter_filter_than_oda` L690–698 | **PASS** (no regression) |
| **6** SFP catalogue class-3 cell = `sfp_bound(3)` = check limit | `📚️catalogue/🦀️.rs` L42; `sfp_catalogue_class3_matches_sfp_bound_and_check_limit` L702–722 | **PASS** (no regression) |
| **7** Perturbation signature `(id, status, computed, limit, utilization)`; dangling `ventSystemId` Fail + en/de + `one_of` | `report_fingerprint` L511–516; `dangling_vent_system_id_fail_apply_remedy_to_pass` L429–444; `editable_leaves_perturb_at_least_one_check_across_examples` L648–686 | **PASS** (no regression) |
| **Gaming grep** `let _ =` in evaluate/inference | None in `🧬️schema/🦀️.rs` evaluate or `💡️inferences/🦀️.rs`; test helpers only (`🧪️tests/⚖️compliance/🦀️.rs` L204/L337/L368) | **PASS** |
| **Gaming grep** field epsilon / fingerprint folds from duplicate-id edit | `push_duplicate_ids` has no `let _ =` or field-epsilon utilization; evaluate retains only divide guards (`max(1e-9)`, fan-flow `1e-6` tolerance L1216, ISO7730 `1e-6` L460/L490) | **PASS** |
| **ventSystemId** normative wiring; CORRECTION 13:27 #1–#12; no skip hatches | `evaluate_zone` integrity L815–852; capacity L1178+; no `@pytest.mark.skip` / `#[ignore]` in family tests | **PASS** |

---

### Round-5 standard check table

| # | Check | R5 | Evidence |
|---|-------|-----|----------|
| 1 | Subject completeness | **PASS** | Hierarchical zones + ventSystems; duplicate-id + dangling-link integrity |
| 2 | Clause coverage | **PASS** | Parts -1/-3/-5-1/-7/-17; ODA4 mapped |
| 3 | Numerics | **PASS** | Worked examples + oracle unchanged |
| 4 | Applicability | **PASS** | Plain N/A gates; scope-aware perturb examples |
| 5 | National annex | **PASS** | `de_annex_diverges_from_en_on_same_subject` |
| 6 | Report quality | **PASS** | Duplicate-id Fail + remedies; localized paths |
| 7 | Examples | **PASS** | compliant + noncompliant + method3 + adaptive perturb paths |
| 7b | Inputs UX | **PASS** | ODA4 choice; structured editor + field-meta |
| 8 | Mutations & schema | **PASS** | Typed aggregate TS; facet parity test |
| 9 | Tests | **PASS** | 84/84 family; 51/51 contract; 0 skipped |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in evaluate path |
| Gaming audit | **PASS** | Round-3 dummy binds absent; duplicate-id edit clean |
| Catalogue tables | **PASS** | `reference_tables()` non-empty; SFP cell = `sfp_bound` |

---

## Blocking fix list (Round 5)

None.

---

## Non-blocking observations (Round 5)

- `📓️audit-perturbation-gaming.md` din16798 section still lists Round-2 gaming instances; code remains fixed — coordinator doc sync still pending.
- `report_fingerprint` test helper name (L511) is acceptable; signature excludes explanation text.
- Fan-flow tolerance `q_design * (1.0 + 1e-6)` (L1216) is a divide/numerics guard, not field epsilon gaming.
- Catalogue panel unit test still weak (body contains `"catalogue"` only) but SFP parity is asserted.
- nx wall-clock ~16m on this host (cargo package-cache lock); nextest summaries confirm 84+51 passed with 0 skipped.
