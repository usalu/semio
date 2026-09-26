# Impl — EN 1993 Wave D

**Family:** `🔩️en1993`  
**Runner:** `bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   1.081s] 154 tests run: 154 passed, 0 skipped`  
**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads; en1993 leaves included.

## Subject (characteristic actions + EN 1990 combinations)

- Snapshot: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — hierarchical SI entities.
- `LoadCase`: `kind` + `category` (no hand-typed limit state as design gate).
- `MemberAction.action`: characteristic N_k / V_k / M_k (`DesignAction`).
- `combine_member_actions` / `governing_effects`: `🧬️schema/🦀️.rs` (EN 1990 §6.4.3 Eq. 6.10a/b + ψ₀/γ_G/γ_Q/ξ; DE NA factors).
- Special-entity forces are characteristic (`nK`, `wheelForceK`, `sigmaK`) and scaled by γ_Q in evaluate.
- Joint `shearForce` / `tensionForce`: characteristic F_v,k / F_t,k → ×γ_Q in bolted checks.

## Blocking items → file:line / tests

| # | Fix | Location | Test |
|---|-----|----------|------|
| 1 | `next_section_options` emits section **ids** | `🧬️schema/🦀️.rs` `next_section_options` | `remedy_law_oneof_section_id_flips_axial_or_buckling_to_pass` |
| 2 | Second fail→pass remedy | bolt shear rows | `remedy_law_bolt_shear_rows_flips_to_pass` (+ buckling length law) |
| 3 | Oracle + jsonschema in nx | `⚖️compliance-oracle/🐍️.py`, `validate_snapshot.py` via Rust | `python_oracle_and_jsonschema_agree_within_half_percent` |
| 4 | Human annex + `NormFieldChoice` | `✏️editor/🏷️field-meta/🦀️.rs` | `field_meta_covers_every_editable_leaf_en_de_unit` |
| 5 | `lookup_norm_field_meta` + `[]` | same | same leaf walk |
| 6 | Example DSL decode + verdicts | `📚️examples/*/🧪️tests/📨️example/` | compliant `complies()`; HSS `fail_count ≥ 2` |
| 7 | Nested snapshot JSON `$ref`/`definitions` | `📸️snapshot/🔣️.json` (+ TS/GQL/proto) | jsonschema in oracle test |
| 8 | Path resolve | `parse_path` / `get_value_at_path` | `every_emitted_path_parses_and_resolves` |
| 9 | Slip-resistant B/C | `part_1_8::slip_resistance_n`, joint `category` | `slip_resistant_category_c_produces_slip_check` |
| 10 | LTB Table 6.4 | `ltb_curve_table_6_4` | `ltb_curve_follows_table_6_4_for_rolled_i` |
| 11 | Distinct de explanations / labels | `subject_member`, fire/silo/axial DE copy | covered by report localization + leaf meta |

## CORRECTION 13:43 — structural actions

- Test: `en1993_combinations_form_uls_from_characteristic_actions` (~200 kN design N from G+Q).
- Perturb: `perturb_every_editable_leaf_in_committed_examples_changes_a_check` (scope-aware).

## Extras A–D

- A Fire: `steel_temperature_c` incremental §4.2.5; Table 3.1 k_y,θ/k_E,θ; χ_fi §4.2.3.2.
- B M+N: `mn_interaction_eta` (6.31–6.41).
- C Tower: `forceCoefficient`×`dynamicFactor`; piles: soil reduction from driving/geometry inputs in evaluate.
- D Catalogue panel comment updated; oracle covers combinations + slip.

## Remaining gaps

See Round-2 section (catalogue tables / gaming audit not claimed).


## Round-2 (Wave D verify FAIL → fix)

**Runner:** `bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   1.081s] 154 tests run: 154 passed, 0 skipped`  
**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → `547 payloads` (`🗑️generated/en1993-wave-d/taxonomy-generate.txt`).

| # | Blocking item | Proof (file:line / test) |
|---|---------------|--------------------------|
| 1 | Part-scoped committed examples (bridge/tower/pile/crane/cold-formed/plated/silo/tension) + perturbation | Assets `🖼️assets/✅️heb240-compliant/…/🗣️.dsl.semio` + `🖼️assets/🔩️high-strength-connection/…/🗣️.dsl.semio` populate the eight entity lists; `multi_part_examples_populate_all_eight_entity_lists` (`⚖️compliance/🦀️.rs:610`); `perturb_every_editable_leaf_in_committed_examples_changes_a_check` (`:367`); compliant/failing verdicts `compliant_example_has_no_fails` / `noncompliant_example_fails_and_remedies_applicable` (`:70`, `:85`) |
| 2 | Shared EN 1990 engine for joints/tower/pile/crane/…; governing combination named | `combine_scalar_forces` / `combine_member_actions` (`🧬️schema/🦀️.rs:1208+`); `joint_and_tower_explanations_name_governing_combination` (`:601`); `en1993_combinations_form_uls_from_characteristic_actions` (`:499`) |
| 3 | Unread fields: pitch/gauge, momentDiagram, memberType, Miner spectrum, fire θ computed, pile k_red derived-only | `pitch_and_gauge_enter_bearing_factors` (`:580`); `moment_diagram_changes_m_cr` (`:591`); `member_type_gates_buckling_and_ltb_checks` (`:628`) + `member_kind`/`kind_allows_*` (`🧬️schema/🦀️.rs:1184–1206`); `FireExposure` has no `designTemperature`; `SteelPile` has no editable `soilReduction`; spectrum on `FatigueDetail` |
| 4 | Palmgren-Miner / S-N with cycles; oracle at N=2×10⁶ ±0.5 % | `miner_damage` (`🧬️schema/🦀️.rs:995`); `palmgren_miner_damage_at_two_million_cycles_matches_oracle` (`:510`) |
| 5 | Class 4 → A_eff/W_eff (no u=class/3); plastic-analysis class-1 gate | `effective_area_class4` (`:389`); classification informational + plastic gate in `check_full_steel_structure`; `class4_uses_effective_section_not_class_over_three_tautology` (`:520`) |
| 6 | Distinct en/de text | `committed_examples_have_distinct_en_and_de_text` (`:544`) |
| 7 | Replace trivial `!checks.is_empty()` | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:176` asserts `en1993.*` check id |
| 8 | Remove `design_gamma`; DE NA combination factors via shared engine | `design_gamma` absent; `de_na_snow_psi0_differs_from_en_recommended` (`:570`); `de_gamma_m1_raises_buckling_utilization_vs_en` (`:28`) |

### Gaps still unproven by a dedicated test

- Catalogue `reference_tables()` emptiness (CORRECTION 14:54) — not claimed closed in this round.
- Gaming-audit items in `📓️audit-perturbation-gaming.md` — not re-audited this round beyond the existing perturbation signature test.


## Round 3 — Wave C fixer (9 blocking)

Closed the Round-3 blocking list for EN 1993:

1. **Catalogue `reference_tables()`** — publishes shared `GAMMA_M*` consts and rolled HEB section properties (same numbers `evaluate()` / `AnnexParams` read). Catalogue test asserts non-empty tables, distinct en/de titles, and one evaluated axial limit equals `A·fy/γ_M0` from those consts.
2. **Perturbation harness** — signature `(id, status, computed, limit, utilization)`; no `allowed + 3` slack; exemptions only descriptive `id`/`name`/`title`/`label`/`labelEn`/`labelDe`. Designation is normative (unknown designation → integrity Fail with catalogue `one_of`).
3. **Referential integrity** — dangling `memberId` / `sectionId` / `materialId` / `loadCaseId` (and nested action load-case refs) Fail with en+de + `one_of`; duplicate entity ids Fail. Tests cover dangling refs and duplicates.
4. **§5.2 class 4** — Fails with utilization `A` vs `A_eff` (not class/3 tautology) and section `one_of` remedy.
5. **`designTemperature` restored** on `FireExposure`; fire path uses `max(designTemperature, θ_heating)` for resistance/critical checks; perturbation test proves limit/utilization change.
6. **`validate_snapshot.py`** — missing `jsonschema` now exits non-zero (no skip hatch).
7. **Protos regenerated** from Rust snapshot (`SteelJoint.actions`/`gauge`, `FatigueDetail.spectrum`, aligned `FireExposure` + `design_temperature`); diff/mutation text guards return `Readonly<En1993Snapshot>` instead of `Record<string, unknown>`.
8. **`🔩️high-strength-connection`** — part-scoped lists populated (DSL + snapshot) so parts 2–6 subjects are non-empty.
9. **`gamma_factors()`** — dead `_annex` parameter removed; STR γ_G/γ_Q/ξ stay identical EN/DE (annex still wired for ψ).

Runner: `bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache -- --no-fail-fast` → **159 passed, 0 skipped**.
