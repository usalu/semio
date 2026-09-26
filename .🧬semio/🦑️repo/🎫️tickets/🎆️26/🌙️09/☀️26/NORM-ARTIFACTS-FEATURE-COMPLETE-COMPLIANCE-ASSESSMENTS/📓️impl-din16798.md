# Impl — DIN EN 16798 (`din16798` / 🌬️) — Wave D

**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Runner:** `bun nx run @semio-tech/norm-din16798-rs:test --skip-nx-cache -- --no-fail-fast`  
**Result:** `Summary [0.894s] 73 tests run: 73 passed, 0 skipped`

## Wave D blocking fixes (all done)

1. **Paths v1.2** — `zones[id=…]` / `ventSystems[id=…]`; `every_emitted_subject_path_parses_and_resolves_on_default_and_noncompliant`.
2. **ComfortModel typed enum** — wire `adaptive` / `fixed_hvac` only; adaptive runs Annex B.2 (θ_rm running mean + category band) and skips PMV/PPD; test `adaptive_model_runs_annex_b2_and_skips_pmv_ppd`.
3. **VentSystemType typed enum** — `decentral_mech` matched in `heat_recovery_eta_min` / `inspection_interval_years`; test η_min=0.73.
4. **Field-meta choices** — ODA1–3, ISO 16890 filters with %, duct A–D, SFP 0–7; human en+de for `usageType`/`pollutionClass`; descriptive `comfortCategory` labels.
5. **`default_snapshot_editable_leaves_have_en_de_field_meta`** — walks default snapshot leaves (incl. list id selectors) asserting en+de (+ unit where required).
6. **Filter Fail** — applicable `RemedyBound::OneOf` writing `filterSupClass`; `apply_remedy_flips_filter_fail_to_pass_via_oneof`.
7. **≥2 Pass-flip tests** — vent + SFP from `noncompliant_office()` → `CheckStatus::Pass`.
8. **Python oracle ±0.5 %** (q, SFP, CO₂) + third-party `jsonschema` vs `📸️snapshot/🔣️.json`.
9. **Entity labels** — distinct en/de (`Zone:` / `Raumzone:`, `Ventilation system` / `Lüftungsanlage`).
10. **Fan energy** — independent of SFP class: annual E vs SFP_ref=1000 W/(m³/s)·q·t (EN 16798-5-1 §6.1).
11. **Mutation suite** — wired `every_mutation_kind_changes_intended_leaf_and_inverse_restores` covering all 38 kinds (plus per-leaf apply tests on disk).

## Extras

- **A.** Taxonomy regenerate succeeded (`489 payloads`; en1993 identity-mismatched leaves skipped by generator, not din16798). Snapshot facets (JSON/TS/GraphQL/proto) aligned to hierarchical Rust zones/ventSystems.
- **B.** Winter + summer θ_op bands; PMV/PPD both seasons with season-specific clo (summer = subject clo, winter = 1.0); ISO 7730 clothing solver hardened (max natural/forced h_c + under-relaxation).
- **C.** Duplicate compliant-office tests removed; catalogue panel uses real `render_catalogue(examples)`; comfort category labels descriptive.

## CORRECTION 13:27 self-check

| # | Cause | Status |
|---|-------|--------|
| 1 | Human en+de choice labels | Pass |
| 2 | Editable leaf meta + coverage test | Pass |
| 3 | Structured editor | Pass |
| 4 | `[id=…]` paths + resolve test | Pass |
| 5 | ≥2 fail→pass + applicable remedies | Pass |
| 6 | DSL examples decode + evaluate | Pass |
| 7 | Python oracle + jsonschema | Pass |
| 8 | Facets + mutation behavioral tests | Pass |
| 9 | No tautologies / dead wires | Pass |
| 10 | No trivial tests | Pass |
| 11 | Semantic mutation verbs | Pass |
| 12 | Distinct en/de dynamic labels | Pass |

## Wave D round-2 (blocking + coordinator A–D)

### Blocking (verify FAIL)

1. **Wire `zone.ventSystemId`** — `evaluate_zone` resolves linked `VentSystemDocument`; integrity check `din16798.zone.ventSystem.<id>` (`🧬️schema/🦀️.rs` L717–760) with Fail/`OneOf` remedy listing system ids (action labels = system names). Zone vent/CO₂ remedies mention linked system (`L923+`). Capacity on `evaluate_vent` (`L1086+`, `din16798-3.capacity.<id>` L1098) uses `q_served = Σ outdoorAirSuppliedM3H` of zones with matching `ventSystemId`; unserved system → NotApplicable (`L1103`). Fan energy uses `designAirflowM3H` (`design_airflow_m3_h` on `VentSystemDocument`, `🦀️.rs` L108). Tests: `two_vent_relink_changes_capacity_on_both_systems` (`⚖️compliance/🦀️.rs` L405), `dangling_vent_system_id_fail_apply_remedy_to_pass` (L429).

2. **CORRECTION 13:43 perturb test** — `editable_leaves_perturb_at_least_one_check_across_examples` (L544) walks editable leaves on `compliant-two-vent`, `noncompliant`, `residential-method3`, `adaptive-office`; exempt only `id`/`name` (L546). Leaf→check wiring: θ_rm into adaptive N/A (`🧬️schema/🦀️.rs` ~L831), clothing+met into adaptive PMV N/A (L833), method-3 uses occupants+pollution (`required_outdoor_air_for_method` L565, `pollution_scale` L551).

### Coordinator decisions

| Dec | Delivery | File:line / tests |
|----|----------|-------------------|
| **A** | `designAirflowM3H` + mutation `change-vent-design-airflow` (tag 40); capacity check; integrity OneOf; zone applicability from linked `ventSystemType` (HR/humid N/A for natural, L1230); fan formulas use design airflow | schema L1086–1230; field-meta `designAirflowM3H` / `ventSystemId`; tests L405, L429 |
| **B** | Scope-aware perturb across examples; exempt `id`/`name` only | `editable_leaves_perturb_at_least_one_check_across_examples` L544–546 |
| **C** | Draught DR ISO 7730 (`draught_rate_percent` L591) + `turbulenceIntensityPercent` + mutation `change-zone-turbulence`; vent method enum §6.3 (`VentMethod` L275, mutation `change-zone-vent-method`); method 1 = B.6/B.7; method 2 = limit concentration via metabolic CO₂ generation (annex provides Δppm + G_CO₂≈0.005 L/s·person@1.2 met — values sufficient, no gap); method 3 = predefined area×pollution + per-person; example `🏠residential-method3`; en/de explanations distinct | draught L1051 + test L457; method3 test L447 |
| **D** | Python oracle ±0.5 % covers capacity sum + DR + method q; jsonschema snapshot validation | `python_oracle_matches_rust_q_sfp_co2_within_half_percent` L335; `🐍️.py` capacity/DR/method helpers |

### Catalogue (shared API sync)

`render_catalogue` now takes tables + `TreeWindows`; din16798 catalogue publishes SFP + DR reference tables (`reference_tables`) and passes `TreeWindows::for_body`.

### Taxonomy

`bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → **547 payloads**.

### Runner

```
Summary [   0.703s] 79 tests run: 79 passed, 0 skipped
```

## Remaining gaps

None for Wave D round-2 DoD.


## Wave D round-3 (blocking)

### Gaming removals (CORRECTION 14:37 / 14:42)

| Site | File:line | Change | Clause |
|------|-----------|--------|--------|
| Fixed-HVAC adaptive N/A | `🧬️schema/🦀️.rs` L858 | Plain `NotApplicable` on `comfortModel`; no θ_rm utilization / fingerprint comment | EN 16798-1 Annex B.2 applies only when `comfortModel=adaptive`; θ_rm drives `adaptive_comfort_temperature_c` (L834) on that path only |
| Adaptive PMV/PPD N/A | `🧬️schema/🦀️.rs` L830 | Plain `NotApplicable` on `comfortModel`; clothing/met not folded into utilization | ISO 7730 PMV/PPD under fixed HVAC; adaptive uses Annex B.2 |
| Cellar area ≤ 0 | `🧬️schema/🦀️.rs` L1336 | `NotApplicable` on `cellarAreaM2` without ventilation utilization | EN 16798-7 §6.2; positive area uses `cellar_ventilation_required_m3_h` vs `cellarVentilationM3H` |

### Facets / ODA4 / catalogue

| Item | Delivery | Tests |
|------|----------|-------|
| Aggregate `🧬️schema/🟦️.ts` | Typed `Din16798Zone[]` / `Din16798VentSystem[]` via snapshot re-export | `schema_ts_facets_forbid_unknown_record_and_placeholder` (L688) |
| Text-guard facets | Dropped exported `Record<string, unknown>` (return `object`) | same |
| ODA4 | field-meta L150 + `required_filter_for_oda` L648 → `ePM1_80_G` rank 5 (ISO 16890-1 ODA4 / EN 16798-3 §7.2) | `oda4_requires_stricter_filter_than_oda3` (L648) |
| SFP one source of truth | Catalogue class-3 cell = `sfp_bound(3)` = SFP check limit | `sfp_catalogue_class3_matches_sfp_bound_and_check_limit` (L660) |

### Perturbation (coordinator B)

`editable_leaves_perturb_at_least_one_check_across_examples` (L590) — signature `(id, status, computed, limit, utilization)`; scope skips θ_rm on fixed-HVAC-only, clothing on adaptive, cellar ventilation when area≤0, metabolic on adaptive unless method 2; asserts θ_rm / clothing / cellar vent each covered in a committed applicable example. `ventSystemId` dangling Fail + en/de + `one_of` unchanged (round-2).

### Taxonomy

`bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → **547 payloads**.

### Runner

```
Summary [   2.601s] 82 tests run: 82 passed, 0 skipped
```


## Wave D round-4 (blocking)

| Item | Delivery | Tests |
|------|----------|-------|
| Duplicate `zones[].id` / `ventSystems[].id` Fail | `push_duplicate_ids` (L689) called from `check_full_environment` (L760) before zone/vent loops; en+de explanation, `SubjectRef` on `zones[id=…].id` / `ventSystems[id=…].id`, applicable `one_of` unused ids | `duplicate_zone_id_fails_integrity` (L447), `duplicate_vent_system_id_fails_integrity` (L468) |

### Runner

```
Summary [   1.913s] 84 tests run: 84 passed, 0 skipped
```
