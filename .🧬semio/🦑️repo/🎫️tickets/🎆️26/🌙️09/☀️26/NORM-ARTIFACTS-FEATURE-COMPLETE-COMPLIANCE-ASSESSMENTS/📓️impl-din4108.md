# Impl — DIN 4108 (`din4108`)

**Wave D status:** PASS candidate — all Wave D blockers cleared.  
**Runner:** `bun nx run @semio-tech/norm-din4108-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   1.273s] 82 tests run: 82 passed, 0 skipped`

## Subject

Hierarchical envelope: `climateZone`, `usage`, `tIntC`, `rhInt`, ventilation/`n50`, `bb2DetailsConform`, `zones[]` (windows with orientation + `inclinationDeg`), `elements[]` (kind/adjacent/area/`orientationDeg`/`inclinationDeg`/`deltaUG|F|R` + layers with density/segments), `thermalBridges[]`.

## Check catalogue

| Part | Clause | Check id | Remedy |
|------|--------|----------|--------|
| DIN 4108-2 | Table 3 | `din4108-2.table3.{elementId}` | Increase insulation thickness; usage → temp class 12–19 °C; light ≤100 kg/m²; frameOpaque / rollerShutterBox rows |
| DIN 4108-2 | §6.2 | `din4108-2.frsi.{elementId}` | Thicken insulation (f_Rsi ≥ 0.70) |
| DIN 4108-2 | §8.3 | `din4108-2.summer.{zoneId}` | S_vorh = Σ(A·g·Fc·Fw·Fi)/A_G; S_zul = Table 8 + solar/passive S_x |
| DIN 4108-3 | §4.3 / Annex A | `din4108-3.glaser.{elementId}` | Documented Annex A BC (−5 °C/80 %, 20 °C/50 %; summer 12 °C/70 %, roof 20 °C) |
| DIN 4108-4 | Table 1 | `din4108-4.lambda.{elementId}.{layerId}` | Cap λ to design table |
| EN ISO 6946 | §6.1/§6.7 | `din4108-6.u.{elementId}` | U+ΔU_g/f/r when inhomogeneous or corrections; else N/A (Table 3 covers) |
| DIN 4108 Bbl.2 | §5.1 | `din4108-6.u-prime.{elementId}` | Per-element U′ with Σψ·l share; `din4108-bb2.delta-u` building ΔU_WB |
| DIN 4108-7 | §4.2 | `din4108-7.n50` | n50 ≤ 1.5 (mech) / 3.0 (natural) |

## Wave D fixes landed

1. Ignored asset-regen → non-writing drift test (`committed_demo_and_failing_assets_match_regenerated_dsl`).
2. All SubjectRef/Remedy paths use `[id=…]`; path-resolve test on default + failing.
3. Human NormFieldChoice en+de labels.
4. `field_meta_covers_every_editable_leaf_on_default_snapshot`.
5. Remedy laws: thickness (table3), n50, summer (Fc/area).
6. Python oracle `🔮️oracles/🐍️.py --report` ±0.5 % for U/S/R/f_Rsi/n50.
7. Third-party jsonschema validation of default + failing snapshots.
8. Table 3 light ≤100 kg/m² + frameOpaque/rollerShutterBox tests.
9. ΔU_WB + ISO 6946 ΔU_g/f/r; duplicate homogeneous U → N/A.
10. Glaser Annex A named constants (incl. roof summer).
11. `usage` → Table 3 low-temp class; orientation/inclination → Fw/Fi in S_vorh.
12. Example DSL decode → evaluate (`complies` / `fail_count ≥ 2` + f_Rsi Fail).
13. Oracle manifest: 33 kinds + compliance oracle registration.
A. Failing example thin-eps 0.004 m → f_Rsi Fail.
B. `command_ids_cover_every_row` covers 8 tools; comments updated.
C. S_zul = Σ S_x (Table 8 + solar + passive); n50 limits tested.

## Remaining gaps

None.

## Requests to coordinator

None.


## Round 2 — Wave D blockers (coordinator decisions)

**Runner:** `Summary [   1.073s] 88 tests run: 88 passed, 0 skipped`

| # | Decision / item | Mapping |
|---|-----------------|---------|
| 1 | Leaf perturbation DoD (CORRECTION 13:43) | `🧬️schema/🎪️tests/⚖️compliance/🦀️.rs` — `every_editable_leaf_perturbation_changes_some_check_on_default_snapshot` (default + timber; id-leaf exemptions listed in test) |
| 2 | `bb2Type` Beiblatt 2 Kategorie A/B/detailed → ΔU_WB + per-bridge | `🧬️schema/🦀️.rs` `bb_2::{bb2_category,delta_u_wb_limit,check_equivalence,check_bridge_category}`; enum labels in `✏️editor/🏷️field-meta/🦀️.rs`; test `flipping_bb2_type_on_default_bridge_changes_report` |
| 3 | Segment λ/μ/ρ/materialId → ISO 6946 / Glaser / Table 3 / part_4 | `layer_mu_eq`, `layer_surface_mass_kg_m2`, `check_segment_design_lambda`; test `timber_segment_lambda_and_density_affect_checks` |
| 4 | Real mutate suite + feature 43 kinds | `🎪️tests/🎱️mutate-din4108-1/🦀️.rs` `mutate_suite_every_kind_fixture_changes_leaf_and_inverse_restores`; `🥒️.feature` “43 kinds”; fixtures regenerated via `🧬️mutations/🎪️tests/🔬️fixture` |
| 5 | Oracle vectors = KINDS len + lock test | `🔮️oracles/🔣️.json` `fixtureCoverage.vectors=43`; test `oracle_manifest_mutation_vectors_match_kinds_len` |
| 6 | DIN 4108-10 application types + property classes | `part_10::{APPLICATION_TYPES,required_application_types,check_application}`; LayerDocument fields; examples compliant/failing; test `din4108_10_wrong_application_fails_and_remedy_passes` |

### Non-blocking

| Item | Mapping |
|------|---------|
| Glaser `ClimateZoneDe` | Re-wired into `check_glaser` explanation (Annex A BC remain fixed constants) |
| Semantic mutations | `change-element-orientation-deg`, `…-inclination-deg`, `…-delta-ug/uf/ur`, `change-thermal-bridge-bb2-type`, `change-zone-window-orientation`, `…-inclination-deg`, `change-layer-application-type`, `…-compressive-class` (full pipeline: leaf/diff/inverse/OpText/OpBinary/protocol tags) |
| Inputs render en+de | `📥️inputs/…/🔬️unit` asserts field-meta labels/choices for climate, bb2Type, applicationType |


## Round 3 — Wave D blockers (CORRECTION 14:42 / 14:54)

**Runner:** `bun nx run @semio-tech/norm-din4108-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   2.520s] 96 tests run: 96 passed, 0 skipped`  
**Taxonomy:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads

| # | Blocker | Mapping |
|---|---------|---------|
| 1 | Perturbation signature drops `explanation.en` | `🧬️schema/🎪️tests/⚖️compliance/🦀️.rs` `check_signature` → `(id, status, computed, limit, utilization)` only; `every_editable_leaf_perturbation_changes_some_check_on_default_snapshot` |
| 2 | DIN 4108-10 water/tensile/acoustic in pass/fail | `part_10::{min_water_for,min_tensile_for,min_acoustic_for,check_application}` — Table 1 class ranks in score; test `din4108_10_property_classes_affect_status_or_utilization` |
| 3 | Glaser `climate` selects BC | `part_3::{glaser_winter_t_ext_c,glaser_summer_t_ext_c}` + facade azimuth; test `glaser_climate_changes_condensation_or_limit` |
| 4 | Referential integrity Fail + `one_of` | `💡️inferences/🦀️.rs` `push_referential_integrity` before clause checks; tests `dangling_material_id_*`, `dangling_zone_id_*`, `duplicate_element_id_*` |
| 5 | Opaque `zoneId` → zone H_T | `part_2::{zone_transmission_ht_wk,check_zone_transmission_loss}`; second zone `zone-utility`; test `moving_opaque_zone_id_changes_zone_transmission_loss` |
| 6 | Catalogue reference tables | `✏️editor/📌️panels/📚️catalogue/🦀️.rs` `reference_tables()` — design-λ (`part_4::DESIGN_LAMBDA_ROWS`), Table 3 (`part_2::TABLE3_R_MIN_ROWS`), 4108-10 (`application_property_row`); `render(..., windows)`; tests `design_lambda_table_matches_part4_const`, `catalogue_design_lambda_matches_evaluated_limit` |

### Leaves re-wired after signature tighten

Inclination → ISO 6946 R_si; orientation → Glaser BC + dry sd_req; usage → summer N/A unless residential; density → `check_surface_mass`; segment layers average declared μ/ρ with segments; softwood/osb in design-λ catalogue.
