# Fix — EN 1998 Round 4 (Wave C)

**Family:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998`
**Verifier:** Round 4 FAIL (2 blocking) — `📓️verify-en1998.md`  
**Anti-gaming (R4 items 1–5, 8–9):** left intact; greps clean (no fingerprint / string-length ε / inventory / `let _ =`).

## Blocking fixes

### 1. `reference_tables()` — DIN EN 1998-1/NA Tables NA.1 + NA.4

**File:** `✏️editor/📌️panels/📚️catalogue/🦀️.rs`

- `table-na-1-seismic-zones` — `SeismicZone::a_gr()` (same source as evaluate)
- `table-na-4-ground-combos` — `GroundCombo::spectrum_params()` → S, T_B, T_C, T_D for A-R…C-S
- Distinct `title_en` / `title_de`; SI units on period / a_gR columns

**Tests:**

- Catalogue: `reference_tables_cells_match_na4_spectrum_params`
- Compliance: `catalogue_na4_cell_matches_evaluated_spectrum_params` (evaluated annex params ≡ catalogue cell ±0.5 %)
- Existing `renders_reference_tables_with_examples` now asserts rendered table ids when non-empty

### 2. Dangling `supportedBuildingId` → `Remedy::one_of`

**File:** `🧬️schema/💡️inferences/🦀️.rs` (assessments + foundations)

- Explicit `CheckStatus::Fail`
- `Remedy::one_of` listing existing `document.buildings` ids (en1990 pattern)
- Localized en≠de explanation / remedy

**Test:** `dangling_supported_building_fails_with_one_of_remedy`

### 3. Duplicate entity ids → Fail + `one_of`

**File:** `🧬️schema/💡️inferences/🦀️.rs`

- `push_duplicate_ids` / `push_referential_integrity` at start of `check_full_seismic`
- Collections: buildings, bridges, assessments, silos, tanks, foundations, retainingWalls, towers; per-building systems/storeys/members; nested variables

**Test:** `duplicate_building_id_fails_integrity`

## Runner

```text
bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache -- --no-fail-fast
Summary [   0.991s] 74 tests run: 74 passed, 0 skipped
```

```text
bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache
Summary [   0.106s] 51 tests run: 51 passed, 0 skipped
```

Logs: `🗑️generated/fix-en1998-r4/test-r4-fixer.txt`, `contract-r4-fixer.txt`
