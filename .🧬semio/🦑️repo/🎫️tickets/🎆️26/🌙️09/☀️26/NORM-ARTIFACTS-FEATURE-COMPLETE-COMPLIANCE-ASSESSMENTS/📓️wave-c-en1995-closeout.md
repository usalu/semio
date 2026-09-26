# EN 1995 Wave C fixer — closeout

Date: 2026-09-26.

## Verdict

Fresh Wave C pass after prior fixer infrastructure death. Source already held load combinations, §7.3 f₁, steel-plate Johansen, hierarchical mutations, and most Round-2 gates. This pass closed CORRECTION 14:xx gaps and unused editable leaves.

## Changes

1. `effective_m_crit_nm` uses `lateralRestraintSpacingM` (M_crit ∝ 1/ℓ_ef²).
2. Connection `rows` in `n_ef` (Rust + Python oracle).
3. Referential integrity: duplicate member/connection/action ids Fail + `one_of`; unknown connection strength class Fail + `one_of`.
4. Perturbation harness with `(id, status, computed, limit, utilization)` signature; scope skips for bridge leaves / steel thickness.
5. Catalogue k_mod cell ↔ evaluate limit equality test.
6. Taxonomy regenerate via norm-plugin target.

## Runner

`Summary [   1.961s] 176 tests run: 176 passed, 0 skipped`
