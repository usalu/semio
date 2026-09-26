# Impl — DIN V 18599 (Wave D Round 2)

Family: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/`

## Round 2 blockers closed

1. **Path casing** — H′T `SubjectRef` + ΔU_WB remedy use `deltaUWbWM2k` (serde/JSON camelCase). Regression: `remedy_law_apply_delta_u_wb_makes_ht_check_pass` (raise ΔU_WB → Fail → apply remedy via `set_value_at_path` + snapshot write → Pass).
2. **Facets** — root + snapshot `🔣️.json` / `🟦️.ts` / `🔗️.graphql` / `🛰️.proto` and inferences reads aligned on `deltaUWbWM2k`; TS/GQL/proto regenerated to the hierarchical subject (no flat legacy fields).
3. **Path resolution** — `every_emitted_path_resolves_via_get_value_at_path` parses + resolves every non-empty `subject.path` and `remedy.target.path` after `evaluate_document`, and exercises `set_value_at_path` for numeric remedies.
4. **Norm-sourced limits** — η_WRG min = `GEG_ANLAGE1_TABELLE1_ETA_WRG_REF` (0.80); tabular q_p = `din_v_18599_12_tabelle5_qp_specific` (66/100/85). All evaluate limit/default floats named in `#region 📜️NormTables` with clause/table ids; `norm_table_rows_match_cited_sources` asserts rows.
5. **Editor report_out** — asserts `summary.complies`, check ids `din18599.geg.ht-prime` / `din18599.geg.qp`, and numeric computed/limit for H′T.
6. **Localization** — cooling explanation en≠de; η_sys explanation en≠de; HRV remedy text uses the 0.80 const (not stale 75%).

## Evaluate limit/default float inventory (named)

| Const / table row | Value | Source |
|---|---|---|
| `NATURAL_GAS` | 1.1 | see const name |
| `HEATING_OIL` | 1.1 | see const name |
| `ELECTRICITY_GRID` | 1.8 | see const name |
| `DISTRICT_HEATING` | 0.7 | see const name |
| `BIOMASS` | 0.2 | see const name |
| `WALL` | 0.28 | see const name |
| `ROOF` | 0.20 | see const name |
| `FLOOR` | 0.35 | see const name |
| `WINDOW` | 1.3 | see const name |
| `DOOR` | 1.8 | see const name |
| `WALL` | 0.28 | see const name |
| `ROOF` | 0.20 | see const name |
| `FLOOR` | 0.35 | see const name |
| `WINDOW` | 1.5 | see const name |
| `DOOR` | 1.8 | see const name |
| `GEG_ANLAGE1_TABELLE1_ETA_WRG_REF` | 0.80 | GEG Anlage 1 Tabelle 1 |
| `GEG_ANLAGE1_DELTA_U_WB_REF` | 0.05 | GEG Anlage 1 Tabelle 1 |
| `GENERATION` | 0.95 | see const name |
| `DISTRIBUTION` | 0.95 | see const name |
| `STORAGE` | 0.98 | see const name |
| `TRANSFER` | 0.95 | see const name |
| `GEG_ANLAGE1_WINDOW_G_REF` | 0.60 | GEG Anlage 1 Tabelle 1 |
| `RESIDENTIAL_WFH` | 5000.0 | see const name |
| `OFFICE` | 2500.0 | see const name |
| `SCHOOL` | 2000.0 | see const name |
| `RESIDENTIAL` | 10.0 | see const name |
| `OFFICE` | 12.0 | see const name |
| `SCHOOL` | 15.0 | see const name |
| `RESIDENTIAL` | 500.0 | see const name |
| `OFFICE` | 100.0 | see const name |
| `SCHOOL` | 150.0 | see const name |
| `STORAGE_OF_USEFUL` | 0.25 | see const name |
| `DISTRIBUTION_OF_USEFUL` | 0.20 | see const name |
| `STORAGE_FLOOR_KWH_A` | 50.0 | see const name |
| `DISTRIBUTION_FLOOR_KWH_A` | 30.0 | see const name |
| `RESIDENTIAL` | 66.0 | see const name |
| `OFFICE` | 100.0 | see const name |
| `SCHOOL` | 85.0 | see const name |
| `DIN_V_18599_5_ETA_SYS_MIN` | 0.85 | DIN V 18599-5 |
| `DIN_V_18599_7_COOLING_TO_HEATING_RATIO` | 0.5 | DIN V 18599-7 |
| `REDUCE_FACTOR` | 0.7 | see const name |
| `FLOOR` | 0.3 | see const name |
| `DETACHED_AN_LE_350` | 0.40 | see const name |
| `DETACHED_AN_GT_350` | 0.50 | see const name |
| `SEMI_OR_END_AN_LE_350` | 0.45 | see const name |
| `SEMI_OR_END_AN_GT_350` | 0.50 | see const name |
| `MID_TERRACE` | 0.50 | see const name |
| `AN_THRESHOLD_M2` | 350.0 | see const name |
| `CLASS_A` | 0.88 | see const name |
| `CLASS_B` | 0.93 | see const name |
| `CLASS_C` | 0.97 | see const name |
| `CLASS_D` | 1.00 | see const name |
| `OUTDOOR` | 1.0 | see const name |
| `GROUND` | 0.6 | see const name |
| `UNHEATED` | 0.5 | see const name |
| `HEATED` | 0.0 | see const name |
| `THETA_I_HEAT_C` | 20.0 | see const name |
| `THETA_I_COOL_C` | 26.0 | see const name |
| `INTERNAL_GAINS_W_M2` | 3.5 | see const name |
| `SOUTH_HORIZ` | 1.0 | see const name |
| `EAST_WEST_HORIZ` | 0.7 | see const name |
| `NORTH_HORIZ` | 0.4 | see const name |
| `LOW_TILT` | 0.9 | see const name |
| `STEEP_TILT` | 1.0 | see const name |
| `MID_TILT` | 0.95 | see const name |
| `DIN_V_18599_2_UTILIZATION_A` | 0.95 | DIN V 18599-2 |
| `DIN_V_18599_1_RHO_CA` | 0.34 | DIN V 18599-1 |
| `DIN_V_18599_1_HOURS_PER_MONTH` | 730.0 | DIN V 18599-1 |
| `DIN_V_18599_5_ETA_SYS_FLOOR` | 0.05 | DIN V 18599-5 |
| `GEG_SECTION10_QP_FACTOR_DEFAULT` | 0.55 | GEG §10 |
| `geg_anlage4_primary_energy_factors::NATURAL_GAS` | 1.1 | GEG Anlage 4 |
| `geg_anlage4_primary_energy_factors::HEATING_OIL` | 1.1 | GEG Anlage 4 |
| `geg_anlage4_primary_energy_factors::ELECTRICITY_GRID` | 1.8 | GEG Anlage 4 |
| `geg_anlage4_primary_energy_factors::DISTRICT_HEATING` | 0.7 | GEG Anlage 4 |
| `geg_anlage4_primary_energy_factors::BIOMASS` | 0.2 | GEG Anlage 4 |
| `geg_anlage2_reference_u::WALL` | 0.28 | GEG Anlage 2 U_ref |
| `geg_anlage2_reference_u::ROOF` | 0.20 | GEG Anlage 2 U_ref |
| `geg_anlage2_reference_u::FLOOR` | 0.35 | GEG Anlage 2 U_ref |
| `geg_anlage2_reference_u::WINDOW` | 1.3 | GEG Anlage 2 U_ref |
| `geg_anlage2_reference_u::DOOR` | 1.8 | GEG Anlage 2 U_ref |
| `geg_anlage3_mean_u::WALL` | 0.28 | GEG Anlage 3 mean U |
| `geg_anlage3_mean_u::ROOF` | 0.20 | GEG Anlage 3 mean U |
| `geg_anlage3_mean_u::FLOOR` | 0.35 | GEG Anlage 3 mean U |
| `geg_anlage3_mean_u::WINDOW` | 1.5 | GEG Anlage 3 mean U |
| `geg_anlage3_mean_u::DOOR` | 1.8 | GEG Anlage 3 mean U |
| `geg_anlage1_tabelle1_heating_eta::GENERATION` | 0.95 | GEG Anlage 1 Tabelle 1 |
| `geg_anlage1_tabelle1_heating_eta::DISTRIBUTION` | 0.95 | GEG Anlage 1 Tabelle 1 |
| `geg_anlage1_tabelle1_heating_eta::STORAGE` | 0.98 | GEG Anlage 1 Tabelle 1 |
| `geg_anlage1_tabelle1_heating_eta::TRANSFER` | 0.95 | GEG Anlage 1 Tabelle 1 |
| `din_v_18599_10_fan_hours::RESIDENTIAL_WFH` | 5000.0 | DIN V 18599-10 |
| `din_v_18599_10_fan_hours::OFFICE` | 2500.0 | DIN V 18599-10 |
| `din_v_18599_10_fan_hours::SCHOOL` | 2000.0 | DIN V 18599-10 |
| `din_v_18599_4_lighting_power_density::RESIDENTIAL` | 10.0 | DIN V 18599-4 |
| `din_v_18599_4_lighting_power_density::OFFICE` | 12.0 | DIN V 18599-4 |
| `din_v_18599_4_lighting_power_density::SCHOOL` | 15.0 | DIN V 18599-4 |
| `din_v_18599_10_dhw_specific::RESIDENTIAL` | 500.0 | DIN V 18599-10 |
| `din_v_18599_10_dhw_specific::OFFICE` | 100.0 | DIN V 18599-10 |
| `din_v_18599_10_dhw_specific::SCHOOL` | 150.0 | DIN V 18599-10 |
| `din_v_18599_8_dhw_loss_fractions::STORAGE_OF_USEFUL` | 0.25 | DIN V 18599-8 |
| `din_v_18599_8_dhw_loss_fractions::DISTRIBUTION_OF_USEFUL` | 0.20 | DIN V 18599-8 |
| `din_v_18599_8_dhw_loss_fractions::STORAGE_FLOOR_KWH_A` | 50.0 | DIN V 18599-8 |
| `din_v_18599_8_dhw_loss_fractions::DISTRIBUTION_FLOOR_KWH_A` | 30.0 | DIN V 18599-8 |
| `din_v_18599_12_tabelle5_qp_specific::RESIDENTIAL` | 66.0 | DIN V 18599-12:2018 Tabelle 5 |
| `din_v_18599_12_tabelle5_qp_specific::OFFICE` | 100.0 | DIN V 18599-12:2018 Tabelle 5 |
| `din_v_18599_12_tabelle5_qp_specific::SCHOOL` | 85.0 | DIN V 18599-12:2018 Tabelle 5 |
| `din_v_18599_7_g_remedy::REDUCE_FACTOR` | 0.7 | DIN V 18599-7 |
| `din_v_18599_7_g_remedy::FLOOR` | 0.3 | DIN V 18599-7 |
| `geg_anlage2_ht_prime_limits::DETACHED_AN_LE_350` | 0.40 | GEG Anlage 2 H′T table |
| `geg_anlage2_ht_prime_limits::DETACHED_AN_GT_350` | 0.50 | GEG Anlage 2 H′T table |
| `geg_anlage2_ht_prime_limits::SEMI_OR_END_AN_LE_350` | 0.45 | GEG Anlage 2 H′T table |
| `geg_anlage2_ht_prime_limits::SEMI_OR_END_AN_GT_350` | 0.50 | GEG Anlage 2 H′T table |
| `geg_anlage2_ht_prime_limits::MID_TERRACE` | 0.50 | GEG Anlage 2 H′T table |
| `geg_anlage2_ht_prime_limits::AN_THRESHOLD_M2` | 350.0 | GEG Anlage 2 H′T table |
| `din_v_18599_11_automation_factor::CLASS_A` | 0.88 | DIN V 18599-11 |
| `din_v_18599_11_automation_factor::CLASS_B` | 0.93 | DIN V 18599-11 |
| `din_v_18599_11_automation_factor::CLASS_C` | 0.97 | DIN V 18599-11 |
| `din_v_18599_11_automation_factor::CLASS_D` | 1.00 | DIN V 18599-11 |
| `din_v_18599_2_adjacency_fx::OUTDOOR` | 1.0 | DIN V 18599-2 |
| `din_v_18599_2_adjacency_fx::GROUND` | 0.6 | DIN V 18599-2 |
| `din_v_18599_2_adjacency_fx::UNHEATED` | 0.5 | DIN V 18599-2 |
| `din_v_18599_2_adjacency_fx::HEATED` | 0.0 | DIN V 18599-2 |
| `din_v_18599_10_zone_defaults::THETA_I_HEAT_C` | 20.0 | DIN V 18599-10 |
| `din_v_18599_10_zone_defaults::THETA_I_COOL_C` | 26.0 | DIN V 18599-10 |
| `din_v_18599_10_zone_defaults::INTERNAL_GAINS_W_M2` | 3.5 | DIN V 18599-10 |
| `din_v_18599_2_solar_geometry_factors::SOUTH_HORIZ` | 1.0 | DIN V 18599-2 |
| `din_v_18599_2_solar_geometry_factors::EAST_WEST_HORIZ` | 0.7 | DIN V 18599-2 |
| `din_v_18599_2_solar_geometry_factors::NORTH_HORIZ` | 0.4 | DIN V 18599-2 |
| `din_v_18599_2_solar_geometry_factors::LOW_TILT` | 0.9 | DIN V 18599-2 |
| `din_v_18599_2_solar_geometry_factors::STEEP_TILT` | 1.0 | DIN V 18599-2 |
| `din_v_18599_2_solar_geometry_factors::MID_TILT` | 0.95 | DIN V 18599-2 |

### Geometric / numeric method parameters (not compliance limits)

Angle breakpoints for solar orientation (45°/135°/225°/315°, tilt 30°/80°) and ε thresholds (`1e-9`, `1e-12` in tests) are method geometry, not table limits.

### Subject-held defaults

`gegQpFactor` on the snapshot defaults to `GEG_SECTION10_QP_FACTOR_DEFAULT` (0.55) via example subjects — not a hidden evaluate literal.

## Check catalogue (post Round 2)

| Check id | Limit source const |
|---|---|
| `din18599.geg.ht-prime` | `geg_anlage2_ht_prime_limits::*` |
| `din18599.geg.mean-u.*` | `geg_anlage3_mean_u::*` |
| `din18599.2.heating-demand` | reference-building `q_h_ref` (GEG Anl. 1) |
| `din18599.4.lighting-power` | `din_v_18599_4_lighting_power_density::*` |
| `din18599.5.heating-efficiency` | `DIN_V_18599_5_ETA_SYS_MIN` |
| `din18599.6.heat-recovery` | `GEG_ANLAGE1_TABELLE1_ETA_WRG_REF` |
| `din18599.7.cooling` | `DIN_V_18599_7_COOLING_TO_HEATING_RATIO` |
| `din18599.8.dhw` | `din_v_18599_10_dhw_specific` + `din_v_18599_8_dhw_loss_fractions` |
| `din18599.geg.qp` | `gegQpFactor` · Q_P,Ref |
| `din18599.11.automation` | class D fail / `din_v_18599_11_automation_factor` |
| `din18599.12.tabular` | `din_v_18599_12_tabelle5_qp_specific::*` |

## Remaining gaps (closed in Round 3)

Round-2 gaps none; Round-3 verify blockers closed below.

## Tests

```
bun nx run @semio-tech/norm-din18599-rs:test --skip-nx-cache -- --no-fail-fast
Summary [   0.654s] 99 tests run: 99 passed, 0 skipped
```

Compliant subjects/DSL/fixtures bumped `heatRecoveryEta` 0.75 → 0.80 to meet `GEG_ANLAGE1_TABELLE1_ETA_WRG_REF`.


## Round 3 — multi-zone DIN V 18599 (Wave D verify blockers + coordinator A–F)

**Summary:** `Summary [   0.914s] 102 tests run: 102 passed, 0 skipped`

### Blocking items → implementation

| Item | Decision | File:line | Tests |
|---|---|---|---|
| 1a heatedVolumeM3 → V_e / A/V_e / n50 share + ΣV≤V_e + net/gross 0.80 | B | schema/rs L709 DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO; L420–428 zone_ventilation_hv; L907–951 check din18599.1.heated-volume | norm_table_constants_match_cited_sources |
| 1b zones[].usageProfile → hours / outdoor-air / fan / lighting | A | root rs L38–44 UsageProfile; schema L370–392 usage_profile_row; L466 Q_l; L760–762 fan hours | usage_profile_change_changes_zone_hours_and_gains |
| 1c zones[].volumeM3 → H_V + volume share | A | schema L420–428, L436–440 | two_zone_moving_element_changes_zone_balances |
| 1d zones[].lightingPowerWM2 → Q_l / part-4 (building LPD removed) | C | root L217–220 LightingSystem{control_factor}; schema L466, L1217–1245 per-zone lighting checks | every_editable_leaf_influences_a_check |
| 1e elements[].zoneId → zone H_T / solar; dangling → Fail + one_of | A | schema L398–399 elements_for_zone; L431–478 zone_balances; L984+ zoneId check | two_zone_moving_element_changes_zone_balances |
| 2 every_editable_leaf_influences_a_check | D | compliance tests L438–501; subject cooled_office_building L484–500; exempts labelEn/labelDe/id (+ climate.*) | every_editable_leaf_influences_a_check |

Paths relative to `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/` (`🏅️standards/🔖️1/🪆️subsets/✳️any/` for schema/tests).

### Coordinator decisions A–F

| Dec | Mapping |
|---|---|
| A Zone monthly balance; aggregate building | zone_balances L431–478; balance_for L765–813; continuous orientation_solar_factor L484–492 |
| B V_e + Σ net ≤ V_e + 0.80 net/gross (DIN V 18599-1) | check din18599.1.heated-volume L907–951; const L709 |
| C Zone lighting SoT; remove building lighting.powerDensityWM2 | LightingSystem control-only; schema/DSL/fixtures/oracle cleaned; no aliases |
| D Cooling plant: Option<CoolingPlant>; scope via cooled_office_building; label exempts explicit | root L181–196; cooling Q_C uses θ_i,c L462–465; test L438+ |
| E Rename mutations → specify-heating-system / specify-dhw-system (verb set) | mutations/🔥specify-heating-system L20; 🚿specify-dhw-system L20; taxonomy regenerated |
| F Two-zone / profile / oracle ±0.5% / jsonschema | two_zone_moving_element_changes_zone_balances; usage_profile_change_changes_zone_hours_and_gains; python_oracle_matches_evaluate_within_half_percent (+ two_zone, cooled); json_schema_validates_… (+ cooled, two_zone); oracle ⚡️balance-din18599-1/🐍️.py |

### Check catalogue additions (Round 3)

| Check id | Notes |
|---|---|
| din18599.1.heated-volume | V_e vs Σ zone net; net/gross ≥ 0.80 |
| din18599.1.net-floor-area | A_N vs Σ zone areas |
| din18599.2.zone-id.* | dangling elements[].zoneId → Fail + one_of zone ids |
| din18599.4.lighting-power.{zoneId} | per-zone LPD vs profile limit (replaces building-level) |

### Runner

```
bun nx run @semio-tech/norm-din18599-rs:test --skip-nx-cache -- --no-fail-fast
Summary [   0.914s] 102 tests run: 102 passed, 0 skipped
```

`bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads.
