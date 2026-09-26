# Verify — EN 1992 (🏛️) adversarial Wave D

**Round history:** R1 **FAIL (14 blocking)** · R2 **FAIL (10 blocking)** · R3 **PASS**  
**Auditor:** read-only adversarial verification, 2026-09-26 (round 3)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1992.md` — 98/98, 0 gaps  
**Prior verify:** R2 `📓️verify-en1992.md` — FAIL (10 blocking)

**VERDICT: PASS**

---

## Test run

| Target | Command | Result |
|--------|---------|--------|
| Family | `bun nx run @semio-tech/norm-en1992-rs:test --skip-nx-cache -- --no-fail-fast` | **Summary [0.765s] 98 tests run: 98 passed, 0 skipped** |
| Contract | `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **Summary [0.451s] 51 tests run: 51 passed, 0 skipped** |

Logs: `🗑️generated/verify-en1992/test-r3.txt`, `🗑️generated/verify-en1992/contract-test-r3.txt`

Impl claim 98/98 is **confirmed**.

---

## Round-2 blocker re-check (10 items)

| # | R2 blocker | R3 | Evidence |
|---|------------|-----|----------|
| 1 | Scope-aware perturbation `(id, status, computed, limit, utilization)` | **PASS** | `every_editable_leaf_influences_a_check_scope_aware` (`🧪️tests/⚖️compliance/🦀️.rs` L470–523): five committed examples; signature L487/L505; exemptions only `id`/`name`/`labelEn`/`labelDe`/`title`; asserts `pointForce`, `tK`, prestress paths |
| 2 | Committed prestressed examples + verdict | **PASS** | `compliant_prestressed_beam` / `failing_prestressed_beam` (`📸️snapshot/🦀️.rs` L291–315); DSL assets `🧵compliant-prestressed-beam`, `💥failing-prestressed-beam`; `prestressed_examples_evaluate` L656–663 |
| 3 | Mutation facets typed — no `_placeholder` / `Record<string, unknown>` | **PASS** | `mutation_facets_have_no_placeholder_or_unknown` L396–414; sample leaf `🏋️change-action-n-ed/🧬️schema/🔗️.graphql` → typed `ChangeActionNEd { memberId, actionId, newValue }` |
| 4 | ACC-6.11 + `AnnexParams::for_situation(..., "accidental")` γ_c=1.3 DE | **PASS** | `flexure.acc` check (`🧬️schema/🦀️.rs` L1207–1227) uses `for_situation(annex, "accidental")`; `accidental_de_gamma_reduces_vs_uls` L312–328 |
| 5 | Anchor characteristic actions → EN 1990 combine | **PASS** | `Anchor` has `actions: Vec<LoadCaseActions>` with `n_k`/`v_k` (`🦀️.rs` L384–396); `combine_anchor_actions` L1907–1991; `evaluate_anchor` L1994+; field-meta `anchors[].actions[].nK` L240–241 (no design-effect-only inputs) |
| 6 | Remove `let _ =`; read k/ε_uk, ε_cu2, n, f_p0,1k | **PASS** | `rg 'let _ ='` over `🧬️schema/🦀️.rs` + `💡️inferences` → 0 hits; ductility §3.2.7 L1644–1663; constitutive §3.1.7 L1142–1156; prestress §5.10 L1571–1573 |
| 7 | Fire `required_for` routes 5.2a/b, 5.3, 5.9, 5.11 | **PASS** | `part_1_2_fire::required_for` L745+; `fire_tables_route_per_kind_and_rating` L331–354 |
| 8 | Distinct en/de explanations | **PASS** | `no_identical_en_de_explanations_in_committed_examples` L357–393 across five examples |
| 9 | `title` as report subject label | **PASS** | `member_ref` prefixes `doc.title` (`🧬️schema/🦀️.rs` L265–271); `title_appears_in_subject_labels` L666–671; title exempt from perturbation only |
| 10 | Non-zero `pointForce` + `tK` on beam-B1 | **PASS** | `compliant_office_frame` Q-office: `point_force: 25.0e3`, `t_k: 8.0e3` (`📸️snapshot/🦀️.rs` L95); perturbation asserts both L515–516 |

**R2 score:** 10/10 FIXED

---

## Fixer claims (Round-2 closeout)

| Claim | R3 | Evidence |
|-------|-----|----------|
| Scope-aware perturbation signature | **PASS** | L487/L505 — five-tuple includes status, computed, limit, utilization |
| Prestressed examples | **PASS** | See blocker #2 |
| Typed mutation facets | **PASS** | See blocker #3 |
| Accidental ACC-6.11 | **PASS** | See blocker #4 (flexure companion; shear has no separate `.acc` id — non-blocking) |
| Anchor characteristic actions | **PASS** | See blocker #5 |
| Ductility / constitutive / prestress fields read | **PASS** | See blocker #6 |
| Fire table routing | **PASS** | See blocker #7 |
| Distinct German | **PASS** | See blocker #8 |
| Title as label | **PASS** | See blocker #9 |
| pointForce + torsion tK | **PASS** | See blocker #10 |
| Shared Table 3.1 f_ck catalogue | **PASS** | `TABLE_3_1_FCK_MPA` + `reference_tables()` (`📌️panels/📚️catalogue/🦀️.rs`); `evaluated_concrete_fck_matches_catalogue_cell` L705–724 |
| Dangling prestressSteelId Fail + `one_of` remedy | **PASS** | `dangling_prestress_steel_id_fails` L727–732; integrity checks `🧬️schema/🦀️.rs` L1542–1563 |
| Duplicate ids Fail + `one_of` remedy | **PASS** | `push_duplicate_ids` in `evaluate()` (`💡️inferences/🦀️.rs` L75–127) — no dedicated unit test (non-blocking) |
| Liquid example verdict | **PASS** | `liquid_retaining_example_has_verdict` L686–702 |

---

## CORRECTION 14:42 — gaming / perturbation re-audit

| Requirement | R3 | Evidence |
|-------------|-----|----------|
| R2 `audit-perturbation-gaming.md` instances fixed | **PASS** | Prior five `let _ =` binds at L1063/L1092/L1487/L1490/L1656 **removed**; `rg 'let _ ='` in evaluate/inference schema → 0 |
| Perturbation signature `(id, status, computed, limit, utilization)` | **PASS** | L487/L505 |
| No explanation-only signature | **PASS** | Signature excludes explanation/title text |
| Exemptions only descriptive labels | **PASS** | L478–481: `id`, `name`, `labelEn`, `labelDe`, `title` |
| Scope-aware across committed examples | **PASS** | office / failing / prestressed ×2 / liquid+anchor |
| Dangling reference integrity | **PASS** | `dangling_prestress_steel_id_fails`; perturb maps `""` → `"dangling-ref"` L615 |
| No `let _ =` in evaluate | **PASS** | 0 hits in `🧬️schema/🦀️.rs` evaluate paths |

---

## CORRECTION 13:27 (12 causes)

| # | Cause | R3 |
|---|-------|-----|
| 1 | `NormFieldChoice` human en+de | **PASS** | `field_meta_covers_every_editable_leaf_en_de` L172–201 |
| 2 | Every editable leaf meta + en/de | **PASS** | + `field_meta_no_prefix_only_fallback_for_editable_leaves` L444–466 |
| 3 | Structured editor, not JSON dump | **PASS** | `📥️inputs/🦀️.rs` (unchanged) |
| 4 | `[id=…]` paths + resolve | **PASS** | `subject_paths_use_id_selectors_and_resolve` L123–152 |
| 5 | ≥2 fail→pass remedy tests | **PASS** | `remedy_law_cover_as_and_stirrups_flip_to_pass` L70–113 |
| 6 | Example DSL + verdict asserts | **PASS** | compliant/failing L50–67; prestressed L656–663; liquid L686–702; five DSL assets under `🖼️assets/` |
| 7 | Python oracle ±0.5 % + jsonschema | **PASS** | `python_oracle_matches_utilizations_within_half_percent` L228–272; `example_snapshot_validates_against_json_schema` L275–296 |
| 8 | Facets match Rust snapshot | **PARTIAL** | `🔣️.json` + `🔗️.graphql` + mutation `🟦️.ts` aligned on characteristic actions; **`🛰️.proto` still stale** (Anchor `n_ed`/`v_ed`, LoadCaseActions `m_ed`/`n_ed`) — non-blocking: JSON is parity anchor per `📓️brief-wave-c-family.md` L34 |
| 9 | No tautologies / ignored fields | **PASS** | Perturbation + field-read law; no gaming binds |
| 10 | No trivially-true tests | **PASS** | Numeric tolerances, fail counts, oracle parity |
| 11 | Semantic mutation verbs | **PASS** | `change-member-cover`, `insert-member`, etc.; legacy `change-action-n-ed`/`change-action-v-ed` **labels** still say `n_ed`/`v_ed` but diffs write `n_k`/`v_k` — non-blocking naming drift |
| 12 | Dynamic issue text localized | **PASS** | `no_identical_en_de_explanations_in_committed_examples` |

---

## Brief checks §1–10

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical members/anchors; `LoadCaseActions` with characteristic `mK`/`nK`/`vK`/`tK`/line/point/external; EN 1990 `combine_member_actions` + `combine_anchor_actions`; prestressed + liquid examples |
| 2 | Clause coverage | **PASS** (caveats) | 30+ checks per member + anchor suite; ACC flexure companion only (no shear.acc); bridge/liquid gated |
| 3 | Numerics | **PASS** | `shear_vrdc_worked_example_de` V_Rd,c,DE≈69.9 kN; `flexure_de_vs_en_divergence` M_Rd,DE≈208 kNm; `fire_r60_axis_distance` a=35 mm; oracle ±0.5 % |
| 4 | Applicability | **PASS** | `NotApplicable` for missing ULS, TC0 liquid, zero torsion; no skipped tests |
| 5 | National annex DE vs EN | **PASS** | α_cc, C_Rd,c, cotθ, cover tables; `for_situation` accidental γ_c=1.3; ACC-6.11 flexure check |
| 6 | Report quality | **PASS** | `[id=…]` paths; distinct en/de; remedy-flip test |
| 7 | Examples | **PASS** | ≥2 compliant + ≥2 failing paths; five DSL assets |
| 7b | Inputs UX | **PASS** | Field-meta tests on three+ examples |
| 8 | Mutations & schema | **PASS** | Typed mutation leaves; proto drift noted (non-blocking) |
| 9 | Tests | **PASS** | 98 executed, 0 skipped; contract 51/51 |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in evaluate; no `_placeholder` mutation stubs |

---

## ADDENDUM 13:43 — structural actions & field-read law

| Requirement | R3 | Evidence |
|-------------|-----|----------|
| Members + characteristic actions combined per EN 1990 in `evaluate()` | **PASS** | `combine_member_actions` L933+; governing L1050+ |
| No hand-typed design effects as only action input | **PASS** | Members and anchors use `LoadCaseActions` characteristic fields; design `n_ed`/`v_ed` only inside `DesignEffects` after combination |
| Scope-aware perturbation | **PASS** | See blocker #1 |
| Static audit: editable leaves vs evaluate reads | **PASS** | Perturbation covers all non-exempt templates across five examples |

---

## Blocking fix list

_(none)_

---

## Non-blocking observations

- `📸️snapshot/🛰️.proto` still describes design-effect `LoadCaseActions` and scalar `Anchor.n_ed`/`v_ed` — regenerate when proto consumers are wired; `🔣️.json` and `🔗️.graphql` are current.
- `change-action-n-ed` / `change-action-v-ed` mutation **labels** say design-effect names but diffs mutate `n_k` / `v_k`; consider rename to `change-action-nk` / `change-action-vk` ( `change-action-mk` already correct for `m_k`).
- ACC-6.11 accidental resistance is emitted for flexure only, not shear/torsion companions.
- `push_duplicate_ids` has no dedicated unit test (behavior implemented in `evaluate()`).
- Oracle + jsonschema tests run on office-frame examples only, not prestressed/liquid (those have separate evaluate verdict tests).
- `📓️audit-perturbation-gaming.md` §en1992 verdict **GAMING (5)** is obsolete after Round-2 fixes; re-audit above shows **CLEAN**.

---

## Test log

Full output: `🗑️generated/verify-en1992/test-r3.txt`  
Contract: `🗑️generated/verify-en1992/contract-test-r3.txt`
