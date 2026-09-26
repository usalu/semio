# Verify — EN 1993 (`🔩️en1993`)

**Verifier:** Wave D adversarial (read-only) — Round 4  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1993.md` Round-3 section — 159/159, all 9 R3 blockers closed  
**Logs:** `🗑️generated/verify-en1993/`

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | **FAIL** | 11 |
| R2 | **FAIL** | 8 |
| R3 | **FAIL** | 9 |
| R4 | **PASS** | 0 |

---

## VERDICT: PASS

---

## Round 4 — R3 blocker re-check (9 items)

| # | R3 blocker | Result | Evidence |
|---|------------|--------|----------|
| 1 | Catalogue `reference_tables()` non-empty; shared consts; en/de titles; limit-equals-cell test | **PASS** | `reference_tables()` returns γ_M + HEB section tables (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:23–73`) importing `GAMMA_M*` and `rolled_heb_catalogue()` from schema. Tests: `renders_reference_tables_with_examples` asserts non-empty + distinct en/de titles (`🧪️tests/🔬️unit/🦀️.rs:25–33`); `reference_table_gamma_and_area_match_evaluated_axial_limit` matches `GAMMA_M1_DE` to `AnnexParams::de().gamma_m1` and finds evaluated limit ≈ `A·fy/γ_M0` (`:37–61`). |
| 2 | Perturbation signature `(id, status, computed, limit, utilization)`; no slack; exemptions descriptive only | **PASS** | `perturb_every_editable_leaf_in_committed_examples_changes_a_check` maps all five tuple fields (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:374–384`, `:407–417`); `assert!(unchanged.is_empty())` — no `allowed + 3` (`:424–427`). Exemptions: `id`/`name`/`title`/`label`/`labelEn`/`labelDe` only (`:394`); geometry `r`/`it`/`iw`/`gModulus` no longer exempt. `designation` is perturbed (normative → integrity Fail). |
| 3 | Dangling refs → `Fail` en+de + `one_of`; duplicate ids → `Fail` | **PASS** | `push_referential_integrity` at evaluate start (`🦀️.rs:1571`); `push_dangling_ref` / `push_duplicate_ids` emit `CheckStatus::Fail`, bilingual `loc()` explanations, `Remedy::one_of` (`:1362–1425`, `:1428–1564`). Covers members/sections/materials/loadCases/actions/joints/fatigue/fire/bridge/tower/pile/crane/cold-formed/plated/tension + nested action load-case refs + unknown `designation`. Tests: `dangling_member_section_material_and_load_case_refs_fail` (`🧪️tests/⚖️compliance/🦀️.rs:432–457`); `duplicate_entity_ids_fail_integrity` (`:460–466`). |
| 4 | §5.2 class 4 → `Fail` with utilization + section `one_of` (not informational) | **PASS** | Class > 3 sets `utilization(A, A_eff)` → auto-`Fail` when `A > A_eff` (`🦀️.rs:1620–1633`; `⚖️compliance/🦀️.rs:294` util gate). `class_four_section_fails_classification_with_section_remedy` asserts `CheckStatus::Fail` + non-empty `one_of` remedy (`🧪️tests/⚖️compliance/🦀️.rs:491–504`). Tautology test retained (`class4_uses_effective_section_not_class_over_three_tautology`). |
| 5 | `designTemperature` restored and wired in fire path | **PASS** | Field on `FireExposure` (`🦀️.rs:222`; `📸️snapshot/🔣️.json:573–606`; field meta `:175`). Fire evaluate uses `theta_a = fire.design_temperature.max(theta_heating)` (`🧬️schema/🦀️.rs:2411`). Test `design_temperature_changes_fire_check_limit_or_utilization` (`🧪️tests/⚖️compliance/🦀️.rs:469–488`). |
| 6 | `validate_snapshot.py` — no jsonschema skip hatch | **PASS** | Missing `jsonschema` → `SystemExit(1)` with stderr message (`🧪️tests/⚖️compliance-oracle/validate_snapshot.py:6–8`). Oracle nx gate still runs via `python_oracle_and_jsonschema_agree_within_half_percent`. |
| 7 | Proto facets regenerated from Rust snapshot | **PASS** | `📸️snapshot/🛰️.proto` + root `🛰️.proto`: `SteelJoint` has `actions`/`gauge`/`pitch` (no stale `shear_force`/`tension_force` fields); `FatigueDetail.spectrum` via `FatigueBand`; `FireExposure.design_temperature` (`🛰️.proto:13–16`). Diff/mutation text guards typed `Readonly<En1993Snapshot>` (`🔺️diff/📝️text/🟦️.ts:22`, `🧬️mutations/📝️text/🟦️.ts:22`). |
| 8 | `🔩️high-strength-connection` part-scoped lists populated | **PASS** | `🖼️assets/🔩️high-strength-connection/snapshot.json` now carries non-empty `coldFormedMembers`, `platedPanels`, `siloShells`, `tensionComponents`, `bridgeFatigue`, `towerLegs`, `piles`, `craneRunways` (`:203–353`). `noncompliant_overloaded_frame()` inherits compliant lists and mutates them (`📸️snapshot/🦀️.rs:274–339`). `multi_part_examples_populate_all_eight_entity_lists` (`🧪️tests/⚖️compliance/🦀️.rs:702–715`). |
| 9 | `gamma_factors` — dead `_annex` removed; STR γ documented identical EN/DE | **PASS** | `fn gamma_factors() -> (f64, f64, f64)` with doc comment “STR values identical for EN recommended and DE NA” (`🦀️.rs:1185–1188`); no `_annex` bind. Annex divergence proven via `psi_factors` + `de_na_snow_psi0_differs_from_en_recommended` (`🧪️tests/⚖️compliance/🦀️.rs:662–668`) and `de_gamma_m1_raises_buckling_utilization_vs_en` (`:28–40`). |

---

## Round 4 — CORRECTION 13 carry-over

| # | Cause | R4 |
|---|--------|-----|
| 1 | Human en+de `NormFieldChoice` | **PASS** |
| 2 | Every editable leaf has meta + test | **PASS** — `field_meta_covers_every_editable_leaf_en_de_unit` |
| 3 | Structured inputs editor | **PASS** |
| 4 | `[id=…]` paths + resolve test | **PASS** |
| 5 | ≥2 fail→pass remedy tests | **PASS** |
| 6 | Example DSL decode + verdicts | **PASS** |
| 7 | Rust runs Python oracle + jsonschema | **PASS** — skip hatch removed |
| 8 | Facets regenerated | **PASS** — proto aligned |
| 9 | No tautologies / ignored fields | **PASS** — `designTemperature` wired; §5.2 class 4 fails |
| 10 | No trivially-true tests | **PASS** |
| 11 | Semantic mutation verbs | **PASS** |
| 12 | Dynamic text localized | **PASS** — `committed_examples_have_distinct_en_and_de_text` |

---

## Test runners (Round 4)

| Command | Result |
|---------|--------|
| `bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache -- --no-fail-fast` | `Summary [   0.663s] 159 tests run: 159 passed, 0 skipped` (`test-r4.txt`) |
| `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | `Summary [   0.192s] 51 tests run: 51 passed, 0 skipped` (`contract-r4.txt`) |

---

## Blocking fix list

*None — all Round-3 blocking items substantively closed.*

---

## Non-blocking observations

- Test count 154 → 159 (+5): integrity, designation, class-4, designTemperature, catalogue-eval tests.
- Perturbation adds `label` to descriptive exemptions (R3 text listed `labelEn`/`labelDe` only); `member.label` is display-only in `subject_member` — acceptable.
- Integrity tests assert `Fail` status but not `one_of` remedy presence; implementation emits remedies and `committed_examples_have_distinct_en_and_de_text` covers remedy localization on emitted checks.
- No dedicated test for unknown `designation` integrity path (implementation present at `🦀️.rs:1453–1467`).
- `💡️inferences/📝️text/🟦️.ts` guards still use `Record<string, unknown>` (inference runtime only; diff/mutation guards fixed).
- Perturbation loop still walks two Rust builders only; high-strength asset covered via populated snapshot + inherited `noncompliant_overloaded_frame()`.
- First nx graph wait ~10 s under concurrent build; test binary green on first attempt with Summary.

---

## Manual verification log

| Check | Result |
|-------|--------|
| `rg 'allowed + 3\|SystemExit(0)\|reference_tables\(\) -> Vec::new'` in en1993 | Clean |
| `rg 'shear_force\|tension_force'` in proto/json schema | Clean (evaluate uses `JointForceAction.shear`/`tension`) |
| `rg 'gamma_factors\(_'` in en1993 | Clean |
| Read high-strength `snapshot.json` entity lists | All eight part-scoped lists non-empty |
| Read perturbation harness | Full 5-tuple signature, zero slack |
