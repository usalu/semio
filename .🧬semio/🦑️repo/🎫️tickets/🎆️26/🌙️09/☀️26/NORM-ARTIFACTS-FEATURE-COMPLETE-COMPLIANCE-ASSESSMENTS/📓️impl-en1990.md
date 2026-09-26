# Impl — EN 1990 (Wave C Round 6)

**Family:** `⚖️en1990`  
**Runner (family):** `bun nx run @semio-tech/norm-en1990-rs:test --skip-nx-cache -- --no-fail-fast`  
**Result:** `Summary [3.108s] 141 tests run: 141 passed, 0 skipped`  

## Round 6 fixes (CORRECTION 14:54)

| Item | Location |
|------|----------|
| `importance_gamma_i` rows from `ImportanceClass::{I..IV}.gamma_i()` (no duplicated literals) | `✏️editor/📌️panels/📚️catalogue/🦀️.rs` |
| Catalogue ↔ source parity: EN office ψ₀ + class III γ_I | `reference_tables_cells_match_psi_and_gamma_i_sources` in `✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs` |

Round-5 fixes (reference-id perturbation, referential integrity, duplicate ids, seismic N/A, `parseEn1990Artifact`) unchanged.

## Remaining gaps

None.

---

# Impl — EN 1990 (Wave D Round 5)

**Family:** `⚖️en1990`  
**Runner (family):** `bun nx run @semio-tech/norm-en1990-rs:test --skip-nx-cache -- --no-fail-fast`  
**Result:** `Summary [0.337s] 140 tests run: 140 passed, 0 skipped`  
**Runner (contract):** `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache`  
**Result:** `Summary [0.120s] 51 tests run: 51 passed, 0 skipped`  
**Also:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads

## Round 5 fixes (CORRECTION 14:42)

### Blocking — reference-id perturbation + integrity

| Item | Location |
|------|----------|
| `is_exempt` narrow (only `projectId`/`labelEn`/`labelDe`/`name`/`title`) | `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:348` |
| `perturb_leaf` dangling `memberId`/`actionId` (`{id}-x`); entity `id` → sibling duplicate else `{id}-x` | `:370` |
| `sig` = `(id, status, computed, limit, utilization)` | `:538` |
| `push_referential_integrity` + duplicate-id Fail + dangling Fail + `one_of` remedies | `🧬️schema/💡️inferences/🦀️.rs:979` |
| Broken wiring → `push_member_unassessable` (no silent Pass) | evaluate member loop |
| Bridge SLS check ids include `sls.id` | `en1990.a2.acceleration.{member}.{sls}` |
| Tests | `dangling_effect_action_id_fails_referential_integrity` (`:57`), `dangling_bridge_sls_member_id_fails_referential_integrity`, `duplicate_member_id_fails_integrity`, `every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope` |

### C. Seismic N/A tighten

`evaluate_marks_seismic_not_applicable_when_no_a_ek` (`:48`) requires `NotApplicable` on default empty seismics.

### D. `parseEn1990Artifact` structural validation

| Item | Location |
|------|----------|
| Typed `En1990ParseError` + field checks | `🧬️schema/🟦️.ts:89` / `parseEn1990Artifact` `:222` |
| Bun script | `🧬️schema/🧪parse-en1990-artifact.bun.ts` |
| Cross-lang test | `parse_en1990_artifact_ts_rejects_malformed_snapshot` (`🚪️io/🧪️tests/🔬️unit/🦀️.rs:99`) |

## Round 4 carry-forward (still green)

Seismic `aEk`+γ_I; accidental/seismic + fatigue examples; perturb scopes 1–4; GEO set C from `structure_kind`; A1.3.1(4).

## Remaining gaps

None.

---

# Impl — EN 1990 (Wave D Round 4)

**Family:** `⚖️en1990`  
**Runner:** `bun nx run @semio-tech/norm-en1990-rs:test --skip-nx-cache -- --no-fail-fast`  
**Result:** `Summary [0.303s] 136 tests run: 136 passed, 0 skipped`  
**Also:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads

## Round 4 fixes

### A. Seismic A_Ed = γ_I · A_Ek (EN 1990 §6.4.3.4)

| Item | Location |
|------|----------|
| Accidental A_d design docstring (§1.5.3.5 / §6.4.3.3) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🦀️.rs:46` |
| `ImportanceClass` + `gamma_i` (Table 4.3 / NA.5) | `…/⚖️en1990/🦀️.rs:59` / `:78` |
| `SeismicAction { a_ek, importance_class }` + `a_ed()` | `…/⚖️en1990/🦀️.rs:82` / `:95` |
| Evaluate uses `s.a_ed()` | `…/✳️any/🧬️schema/💡️inferences/🦀️.rs:184` |
| Field-meta `aEk` / `importanceClass` | `…/✳️any/✏️editor/🏷️field-meta/🦀️.rs` |

### B. Accidental + seismic examples + perturb scope 4

| Item | Location |
|------|----------|
| Examples | `…/📚️examples/🏢️accidental-seismic-compliant/`, `…/accidental-seismic-failing/` |
| Verdict tests | `accidental_seismic_compliant_activates_6_11_and_6_12b` (`🔬️compliance-report/🦀️.rs:277`), `accidental_seismic_failing_fails_6_11_or_6_12b` |
| Perturb scope 4 (`ad` / `aEk` / `importanceClass`; 6.11/6.12b N/A when cleared) | `every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope` (`🔬️compliance-report/🦀️.rs:314`, scope @ `:588`) |

### C. Non-blocking

| Item | Location |
|------|----------|
| Bridge `complies` / `fail ≥ 2` | `road_bridge_compliant_subject_complies` (`:262`), `road_bridge_failing_fails_with_multiple_checks` |
| GEO set C from `structure_kind` | `combination_geo_a24c` (`🧬️schema/🦀️.rs:710`), `push_geo_set_c` (`💡️inferences/🦀️.rs:862`) |
| NDP A1.3.1(4) max(6.10a,6.10b) | `💡️inferences/🦀️.rs:195` (+ GEO loop) |
| Fatigue examples + verdicts | `fatigue-compliant` / `fatigue-failing`; `fatigue_compliant_subject_complies` (`:297`), `fatigue_failing_fails_fat_check` |

## Round 3 carry-forward (still green)

Perturb scopes 1–3; `BridgeSls`; site `altitudeM`; A2.4 action γ; Table 2.1 remedy; localized STR/GEO/accidental/seismic.

## Remaining gaps

None.
