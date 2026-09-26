# Impl — EN 1994 (`🧩️en1994`) — Round 3

## Subject (SI, hierarchical)
- `En1994Snapshot`: annex, structureKind, steelFYPa, beams[], columns[], slabs[], fireRating, insulationThicknessM, fatigueDetail
- **CharacteristicAction** (area-only source of truth): `qAreaPa` × member tributary width → line load in `part_en1990::action_internals` (`🧬️schema/🦀️.rs` action_internals). Removed `qLineNPerM`.
- Columns may carry externally analysed characteristic `nKN` / `mKNm` when `qAreaPa == 0`.
- Scoped helpers: `En1994Snapshot::unpropped_building`, `custom_plate_building`, `bridge_girder`, `fire_demanding` (`📸️snapshot/🦀️.rs`).

## Combinations (`part_en1990`)
| Mode | EN 1990 | Governs |
|------|---------|---------|
| ULS 6.10 | γ_G/γ_Q ψ₀ | ULS M/V/N, construction, columns |
| SLS characteristic | ψ₀ | §7.2.2 steel stress (`en1994.7.2.2.stress.*`) |
| SLS frequent | ψ₁ lead + ψ₂ acc | §7.3.1 deflection (`en1994.7.3.1.deflection.*`) |
| SLS quasi-permanent | ψ₂ | §7.4 crack with n_L (`en1994.7.4.crack.*`) |
| Fire 6.11 | ψ_fi | fire combination demand |

## Round-3 blocking fixes
1. Catalogue `render_catalogue(examples, tables, locale, controller_id, windows)` — already on B2 API (`✏️editor/📌️panels/📚️catalogue/🦀️.rs`).
2. TS facets typed: `CompositeBeam` / `Column` / `Slab` / `CharacteristicAction` in `🧬️schema/🟦️.ts` + `📸️snapshot/🟦️.ts`; graphql/proto/json-schema nested; test `ts_snapshot_facets_have_no_unknown_and_match_rust_leaves`.
3. Leaf test `every_editable_leaf_affects_at_least_one_check` — scope-aware (default / unpropped / custom-plate / bridge / fire / column forces); asserts status+computed+limit+utilization; no fingerprint gaming; label exemptions only.
4. SLS frequent combination + checks wired (table above).
5. Removed `let _ = sls_char`; stress check uses `sls_char`.
6. Single-source area loads; fixtures/DSL `qAreaPa` restored for permanents.

## Non-blocking
- Stud Δτ uses annex `γ_Mf` (`part_2::stud_fatigue_resistance_pa(n, annex)`).
- Column N–M: `α_M` 0.9/0.8 + `moment_resistance_at_n` polygon (`part_column`, §6.7.3.6).
- Custom plate: `SteelSection::custom_plate` + `En1994Snapshot::custom_plate_building` for leaf geometry.

## Tests (proof)
| Item | Test name |
|------|-----------|
| Leaf coverage | `every_editable_leaf_affects_at_least_one_check` |
| TS parity | `ts_snapshot_facets_have_no_unknown_and_match_rust_leaves` |
| Annex γ_Mf | `de_vs_en_bridge_fatigue_gamma_mf` (+ annex assert inside leaf test) |
| Examples | `passing_example_dsl_complies`, `failing_example_dsl_does_not_comply_with_named_ids`, `bridge_example_runs_fatigue_checks` |
| Oracle / schema | existing oracle ±0.5% + jsonschema |

## Runner
```
Summary [   5.463s] 74 tests run: 74 passed, 0 skipped
```
`bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache -- --no-fail-fast`


## Round-4 blocking fixes
1. Catalogue `reference_tables()` publishes γ_G/γ_Q (shared `part_en1990::GAMMA_*` / `gamma_g`/`gamma_q`), ψ₀/ψ₁/ψ₂ (shared `psi_factors`), and stud spacing min/max (shared `part_1_1::STUD_SPACING_*` / `stud_spacing_limits_m`). Test `reference_tables_cells_match_psi_and_gamma_i_sources` asserts cells equal evaluate sources; empty tables no longer allowed.
2. Removed `let _ = annex` in `gamma_g`/`gamma_q` — both branch on `AnnexChoice`. Bridge fatigue annex walk + γ_Mf DE/EN divergence remain the normative annex proof (`bridge-fatigue` scope walks `annex`).
3. Beam/slab force duplicates removed: `CharacteristicAction` keeps `qAreaPa` or sole `fKN` (never both — `en1994.action.single-source.*`); columns use `ColumnAction` with `nKN`/`mKNm` only. Perturbation adds `failing-beam` scope from `composite-floor-beam-failing`; pred exemptions are labels only (plus catalogue steel geom overwrite / scope routing).
4. `validate_snapshot.py` fails hard if `jsonschema` is missing. Python oracle compares η / η_min / utilization for `etamin` (no shape-skip hatch).

## Runner (Round 4)
```
Summary [   1.430s] 75 tests run: 75 passed, 0 skipped
```
`bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache -- --no-fail-fast`

## Remaining gaps
None for Round-4 blockers.

## Requests to coordinator
None
