# Implement — EN 1997 (`🌍️en1997`, Wave C/D round 4)

**Runner:** `bun nx run @semio-tech/norm-en1997-rs:test --skip-nx-cache -- --no-fail-fast`  
**Result:** `Summary [   0.636s] 92 tests run: 92 passed, 0 skipped`

**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads.

## Round-4 blockers (3/3)

| # | Blocker | Fix (file:line) | Tests |
|---|---------|-----------------|-------|
| 1 | CPT/SPT φ′ tautology | Separate CPT / SPT / combined cautious checks `🧬️schema/🦀️.rs:1263–1355` (`en1997.2.phi.cpt.*`, `.spt.*`, `.derived.*`); Fail + `phiPrimeDeg` remedy when stated > investigation; bearing uses `characteristic_phi_deg` (`:1011–1039`) | `phi_investigation_correlation_numeric` (`⚖️compliance/🦀️.rs:726`), `phi_stated_above_investigation_fails_and_remedy_passes` (`:753`) |
| 2 | Governing BS-P/T/A summary | Per-element + project summaries via `push_governing_situation_summary` (`:1115`); footing/pile/wall/uplift + `en1997.governing.situation` (`:1682`, pile/wall/uplift sites, `:2185+`); outline `governingSituation`/`governingApproach` (`💡️inferences/🧾outline/🦀️.rs:27–89`) | `governing_design_situation_changes_with_load_case_situation` (`:775`) |
| 3 | Typed TS facets | Typed `SoilLayer` / `SpreadFoundation` / … in `🧬️schema/🟦️.ts`, mutations, snapshot, GraphQL, proto (no `unknown[]` / `Record<string, unknown>`) | `typed_facets_have_no_unknown_and_match_rust_fields` (`:814`) |

### Coordinator tightenings

| # | Item | Evidence |
|---|------|----------|
| a | Bishop in Python oracle ±0.5 % | Independent slice FoS in `🌍️compliance-en1997-1/🐍️.py` (`bishop_fos_slices`); parity `python_oracle_matches_check_project_within_half_percent` (`:287`) — no skip |
| b | GEO-3 for overall stability | Docstring `bishop_fos_slices` (`🦀️.rs:880–881`); evaluate comment + explanation (`:2016`, slope check en/de GEO-3 / EN 1997-1 §2.4.7.3.4 / DIN 1054 NA) |

### CORRECTION 14:37

- Removed epsilon / fingerprint gaming (`rg` clean on family).
- Bishop FoS driven by slope angle / height / length in slice geometry (`bishop_fos_slices_inner` `:892+`); perturb asserts status/computed/limit/utilization (`perturb_every_editable_leaf_changes_some_check` `:867`).

## Round-3 (retained)

87→92 tests; prior six round-3 blockers remain fixed (undrained c_u, Poisson elastic settlement, governingLayerId gate, pileType factors, earth-pressure regime, concreteGamma).

## Remaining gaps

None for the round-4 verify list.
