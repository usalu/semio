# Impl — EN 1993 Wave D

**Family:** `🔩️en1993`  
**Runner:** `bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   0.428s] 144 tests run: 144 passed, 0 skipped`  
**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 537 payloads; en1993 leaves included (not skipped).

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

none
