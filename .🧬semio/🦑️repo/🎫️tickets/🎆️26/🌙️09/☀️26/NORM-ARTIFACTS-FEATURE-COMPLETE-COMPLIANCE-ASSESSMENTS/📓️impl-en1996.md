# Impl — EN 1996 (`🪨en1996`) Wave D closeout

**Family:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨en1996/`  
**Runner:** `bun nx run @semio-tech/norm-en1996-rs:test --skip-nx-cache -- --no-fail-fast`

## Round 1 (prior)

**Summary:** `Summary […] 139 tests run: 139 passed` (R1 blockers cleared; see verify R1).

## Round 2

**Summary:** `Summary [   0.861s] 139 tests run: 139 passed, 0 skipped`  
**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads.

### Blocking fixes (verify R2)

| # | Fix | Where / tests |
|---|-----|----------------|
| 1 | Scope examples (openings / basement+earth / concentrated / reinforced) + multi-scope perturb harness; asserts status/computed/limit/utilization only (14:37 — no explanation gaming) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` L161: `opening_wall_example`, `basement_wall_example`, `concentrated_load_example`, `reinforced_wall_example`; `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L139: `perturb_every_editable_leaf_changes_some_check` |
| 2 | Remove `skip-no-jsonschema`; fail if `jsonschema` missing/invalid; stdout exactly `ok` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs` L52: `json_schema_validates_example_snapshots`; oracle `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs` L21: `python_oracle_matches_compliant_and_noncompliant` + `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️evaluate-en1996-1/🐍️.py` |
| 3 | Distinct en/de mutation + check labels (opening height/width/sill, insert/remove, basement, …) + identity test | mutation `LocalizedLabel::native(…)` pairs; `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L326: `mutation_and_check_labels_differ_en_de` |
| 4 | Slab-end rotation e_θ + Φ₁ from `slabSpanM` into e₀/e_mk / Φ chain (DIN EN 1996-1-1/NA **NDP 6.1.2.2 / NA Annex C**: e_θ=(N_floor/N)·ℓ_f/25 capped 0,05·t; Φ₁=1,6−ℓ_f/6 on 4,5…6,0 m) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🦀️.rs` L244: `eccentricity_from_slab_rotation_m`; L261: `phi_1_slab_span`; mid-height check L809; oracle `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️evaluate-en1996-1/🐍️.py` |

### Coordinator A–D

| ID | Fix | Where / tests |
|----|-----|----------------|
| A | Opening height+width (+ sill) reduce ℓ_ef / A (§5.5.1.4); pier vs partial | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🦀️.rs` L181: `effective_length_m`, `area_m2` |
| B | Removed dead `opening_path` touch / epsilon gaming; family grep clean of fingerprint/1e-9\* | evaluate path |
| C | DSL/pack regen via `📜️script.ts regenerate-example-assets` (+ `launch.json` `📦️generate📕️norm🪨en1996🖼️example-assets`); test parse-only | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/📜️script.ts`; `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L305: `committed_dsl_pack_assets_parse_and_assert_verdicts` |
| D | Typed `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (no `unknown[]`); facet parity | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L380: `typed_diff_and_snapshot_facets_have_no_unknown` |

### Norm notes

- Mid-height compression always reported so φ∞ → e_k → Φ_m enters status/limit (not only explanation).
- Reinforced As check uses `.minimum(As, As,min)` (not inverted utilization).
- Fail paths always carry ≥1 remedy (perturbation must not trip `debug_assert` on empty remedies).

## Round 3 (Wave C fixer)

**Summary:** `Summary [   1.511s] 144 tests run: 144 passed, 0 skipped`  
**Contract:** `Summary [   0.607s] 51 tests run: 51 passed, 0 skipped`

### Blocking fixes (verify R3)

| # | Fix | Where / tests |
|---|-----|----------------|
| 1 | Non-empty `reference_tables()` from evaluate consts/helpers (DE γ_M, DE/EN f_k factors, fire REI 90 t_min, ψ₀ categories, DE f_vlt/f_b = `F_VLT_OVER_FB_DE`) + distinct en/de titles + SI units | `✏️editor/📌️panels/📚️catalogue/🦀️.rs`; test `reference_tables_cells_match_evaluate_sources` |
| 2 | Referential integrity before normative checks: duplicate `walls` / `loadCases` / `openings` / `concentrated` ids Fail with en+de rename remedy; dangling `imposedCategory` / `designSituation` Fail with `one_of` | `🧬️schema/⚖️masonry/🦀️.rs`; tests `duplicate_wall_id_fails_integrity`, `duplicate_load_case_opening_and_concentrated_ids_fail_integrity`, `dangling_imposed_category_fails_integrity`, `perturb_wall_id_to_duplicate_changes_integrity_signature` |

### Round-2 regressions

Scope perturb, jsonschema non-zero exit, distinct en/de labels, slab-span eccentricity — still green in the 144-pass suite.

## Round 4 (Wave C fixer)

**Summary:** `Summary [   1.424s] 144 tests run: 144 passed, 0 skipped`  
**Contract:** `Summary [   0.185s] 51 tests run: 51 passed, 0 skipped`

### Blocking fix (verify R4)

| # | Fix | Where / tests |
|---|-----|----------------|
| 1 | Fire check `en1996.1-2.fire.*` emits `check.limit` = `t_min` from `fire_min_thickness_na` (via `.minimum(t, t_min)`); catalogue test extends `reference_tables_cells_match_evaluate_sources` to `evaluate_building` a Clay/Group1/GP/REI90 snapshot with α ∈ (0.6, 1.0] so limit equals `fire-min-thickness-rei90` / `rei90-clay` cell (same const path as `fire_min_thickness_m`) | `🧬️schema/⚖️masonry/🦀️.rs` fire block; `✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs` |

### Round-3 regressions

Referential integrity (duplicate wall/load-case/opening/concentrated Fail; dangling FK Fail + `one_of`; perturbation signature `(id, status, computed, limit, utilization)`), distinct en/de, no fingerprints / `.len()` echoes — still green in the 144-pass suite.

## Remaining gaps

None for the Wave C round-4 blocking list.
