# Implement — EN 1997 (`🌍️en1997`, Wave C/D round 5)

**Runner:** `bun nx run @semio-tech/norm-en1997-rs:test --skip-nx-cache -- --no-fail-fast`  
**Result:** `Summary [   1.654s] 97 tests run: 97 passed, 0 skipped`

**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads.

## Round-5 blockers (3/3)

| # | Blocker | Fix (file:line) | Tests |
|---|---------|-----------------|-------|
| 1 | Empty `reference_tables()` | DIN 1054 partial factors + UPL/HYD + Annex D N-factors via `resolve_params` / `resolve_upl_params` / `part_1::bearing_factor_*` in `✏️editor/📌️panels/📚️catalogue/🦀️.rs:19–129`; `render` takes `windows` (`:132–139`); editor passes `TreeWindows::for_body` | `renders_reference_tables_with_examples` (catalogue unit `:21`), `reference_tables_partial_factors_match_resolve_params` (`:41`) |
| 2 | Four `let _ =` dummy binds | `gamma_w` in settlement buoyancy `🧬️schema/🦀️.rs:664–665`; drained layer subject/remedy → `cohesionEffective` (`:1348–1375`); drop unused `path_type`/`path_h` dummies; wall/slope remedies target `height` (`:2025–2040`, `:2182–2192`) | `rg` clean on `let _ =` / `fingerprint` / `1e-9 *` in family; oracle settlement synced |
| 3 | No duplicate entity-id integrity | `push_duplicate_ids` / `push_all_duplicate_ids` (`:1116–1260`) covering layers, footings, loadCases, piles, testProfiles, retainingWalls, slopes, upliftCases; called from `check_project` (`:1290`) | `duplicate_layer_id_fails_integrity` (`⚖️compliance:1027`), `duplicate_footing_id_fails_integrity` (`:1045`), `duplicate_ids_fail_for_every_id_bearing_collection` (`:1061`), `governing_layer_id_missing_still_fails_with_duplicates_present` |

### Round-4 blockers (retained, not regressed)

| # | Item | Evidence |
|---|------|----------|
| 1 | φ′ derived not tautological | `en1997.2.phi.derived.*` Fail when stated > investigation |
| 2 | Governing BS-P/T/A summary | `push_governing_situation_summary` + outline |
| 3 | Typed TS facets | no bare `unknown` / `Record<string, unknown>` |

## Remaining gaps

None for the round-5 verify list.
