# Audit — DIN EN 16798 (`🌬️din16798`)

**Executive summary.** The `🌬️din16798` artifact is a **wide flat bag of 62 scalars** with 25 always-on compliance checks wired through `evaluate()` → `check_full_environment()`. Individual helper functions are mostly real arithmetic (PMV ISO 7730, IDA ventilation, SFP classes, duct leakage), and unit tests assert numeric worked examples — but the **subject is not a building/zone/system model**, six **reference/limit fields live on the subject** (`fan_energy_reference_kwh`, `cooling_reference_kwh`, …) making checks self-referential, **default snapshot is tuned to pass all 25 checks** (`Din16798Snapshot::default()` + `full_environment_evaluate_covers_all_nine_parts`), there is **no applicability gating** (office ventilation + dwelling ventilation + data-centre supply air all run for every occupancy), **no remediation/localization**, DE-NA acoustic divergence is declared but unused, and the editor is a JSON dump. Feature-complete compliance assessment requires a hierarchical subject schema, norm-derived limits, conditional check dispatch, inverse remediation, localized report UX, and adversarial examples/tests.

---

## 1. Inventory

### 1.1 Snapshot / document fields (62 scalars, flat struct)

Source: `…/🧬️schema/📸️snapshot/🦀️.rs` L14–160, `…/🧬️schema/🦀️.rs` L12–137.

| Group | Field | Type | Unit (DSL) | Role |
|-------|-------|------|------------|------|
| Meta | `annex` | `AnnexChoice` | — | EN vs DE-NA |
| Meta | `occupancy` | `String` | — | Parsed to `OccupancyType` |
| IEQ P1 | `comfort_category` | `String` | — | I / II / III |
| IEQ P1 | `t_op_c` | `f64` | °C | Operative temperature |
| IEQ P1 | `rh_percent` | `f64` | % | Relative humidity |
| IEQ P1 | `air_speed_m_s` | `f64` | m/s | Draught |
| IEQ P1 | `theta_rm_c` | `f64` | °C | Running mean outdoor temp (adaptive) |
| IEQ P1 | `co2_ppm` | `f64` | ppm | Indoor CO₂ |
| IEQ P1 | `df_percent` | `f64` | % | Daylight factor |
| IEQ P1 | `l_aeq_db` | `f64` | dB | Ventilation noise |
| Vent P3 | `persons` | `u32` | — | Occupancy count (non-res) |
| Vent P3 | `ida_class` | `String` | — | IDA 1–4 |
| Vent P3 | `ventilation_m3_h` | `f64` | m³/h | Supplied outdoor air |
| Vent P3 | `floor_area_m2` | `f64` | m² | Floor area |
| Vent P3 | `bedrooms` | `u32` | — | Dwelling bedrooms |
| Vent P3 | `dwelling_ventilation_m3_h` | `f64` | m³/h | Dwelling supply |
| Vent P3 | `occupants` | `u32` | — | Residential occupants |
| Vent P3 | `residential_ventilation_m3_h` | `f64` | m³/h | Residential supply |
| Vent P3 | `sfp_w_m3_s` | `f64` | W/(m³/s) | Specific fan power |
| Vent P3 | `sfp_required_class` | `u8` | — | SFP1–6 target class |
| Vent P3 | `heat_recovery_eta` | `f64` | — | Delivered η |
| Vent P3 | `heat_recovery_eta_min` | `f64` | — | **Subject-stored minimum** |
| Vent P3 | `system_type` | `String` | — | `central_mech` / `decentral` |
| Vent P3 | `years_since_inspection` | `u32` | a | Since last inspection |
| Vent P3 | `humidification_required_kg_h` | `f64` | kg/h | Required |
| Vent P3 | `humidification_provided_kg_h` | `f64` | kg/h | Provided |
| Energy P5-1 | `fan_q_v_m3_s` | `f64` | m³/s | Fan volume flow |
| Energy P5-1 | `fan_t_run_h` | `f64` | h | Annual run hours |
| Energy P5-1 | `fan_energy_reference_kwh` | `f64` | kWh | **Subject-stored limit** |
| Energy P5-1 | `night_setback_k` | `f64` | K | Night setback depth |
| Energy P5-2 | `hr_m_dot_kg_s` … `hr_t_h` | `f64` | various | HR savings inputs |
| Energy P5-2 | `hr_savings_reference_kwh` | `f64` | kWh | **Subject-stored limit** |
| Infil P7 | `n50_h_inv`, `volume_m3` | `f64` | 1/h, m³ | Blower-door |
| Infil P7 | `infiltration_allowance_m3_h` | `f64` | m³/h | **Subject-stored limit** |
| Infil P7 | `cellar_area_m2`, `cellar_ventilation_m3_h` | `f64` | m², m³/h | Cellar |
| Cool P9 | `h_tr_w_k`, `h_ve_w_k`, `theta_e_c`, `theta_set_c`, `cooling_delta_t_h`, `cooling_gains_kwh`, `cooling_utilization_factor` | `f64` | various | Cooling degree-hour model |
| Cool P9 | `cooling_reference_kwh` | `f64` | kWh | **Subject-stored limit** |
| Gen P13 | `chiller_type`, `eer_actual`, `q_c_kwh` | various | — | Chiller |
| Gen P13 | `generation_reference_kwh` | `f64` | kWh | **Subject-stored limit** |
| Gen P13 | `data_center_supply_c` | `f64` | °C | DC supply air |
| DHW P15 | `h_st_w_k`, `theta_st_c`, `theta_amb_c`, `storage_t_h` | `f64` | various | Storage losses |
| DHW P15 | `storage_allowance_kwh` | `f64` | kWh | **Subject-stored limit** |
| DHW P15 | `dhw_delivery_c` | `f64` | °C | DHW delivery |
| Duct P17 | `duct_class`, `duct_test_pressure_pa`, `duct_leakage_m3_s_m2` | various | — | Leakage class |

**No composed children** — single `Din16798Snapshot` root; inference `outline` mirrors the same 62 field names (`…/💡️inferences/🧾outline/🦀️.rs` L10–73).

### 1.2 Mutations (62 kinds)

`…/🧬️schema/🧬️mutations/🦀️.rs` L99+ — one `change-<field>` per scalar (`ChangeAnnex` … `ChangeDuctLeakageM3SM2`). Each triad has `🦀️.rs`, `🔺️diff/`, `↩️inverse/` (undo only — restores prior scalar, no compliance inversion). Oracle catalog: `…/🔮️oracles/🔣️.json` (62 fixture vectors, Python second implementation).

### 1.3 `evaluate()` call graph

`…/💡️inferences/🦀️.rs` L190–191 → `check_full_environment()` L132–186:

```
evaluate(snapshot)
  └─ check_full_environment
       ├─ parse_occupancy / parse_comfort_category / parse_ida_class / parse_duct_class / parse_chiller_type
       ├─ annex_params::AnnexParams::for_choice(annex)
       ├─ part_1::check_operative_temperature
       ├─ part_1::check_pmv_comfort          ← ignores comfort_category
       ├─ part_1::check_adaptive_comfort
       ├─ part_1::check_co2_level
       ├─ part_1::check_daylight_factor
       ├─ part_1::check_acoustic_category
       ├─ part_3::check_ventilation_rate
       ├─ part_3::check_dwelling_ventilation
       ├─ part_3::check_residential_ventilation
       ├─ part_3::check_design_sfp
       ├─ part_3::check_heat_recovery_efficiency
       ├─ part_3::check_inspection_due
       ├─ part_3::check_humidification_capacity
       ├─ part_5_1::check_building_fan_energy
       ├─ part_5_1::check_night_setback
       ├─ part_5_2::check_heat_recovery_savings
       ├─ part_7::check_infiltration
       ├─ part_7::check_cellar_ventilation
       ├─ part_9::check_cooling_energy_need
       ├─ part_13::check_chiller_eer
       ├─ part_13::check_generation_energy
       ├─ part_13::check_supply_air_temperature
       ├─ part_15::check_storage_losses
       ├─ part_15::check_dhw_temperature
       └─ part_17::check_duct_leakage
```

**Helpers defined but NOT reached by `evaluate()`:** `part_1::pmv_simplified` (L471), `part_1::ppd_from_pmv` (L528), `part_3::OdaClass` (L682–697), `part_3::classify_sfp` (L772 — only used in unit test), `annex_params.acoustic_limit_residential_db` (L1177 — never read). Alternate entry: `check_residential_environment()` (L77–82) — 3 checks only, unused by `evaluate()`.

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `…/✏️editor/…/📥️inputs/🦀️.rs` L19–20 | `render_document_json` — **pretty JSON only**, no typed form |
| Results | `…/✏️editor/…/📊️results/🦀️.rs` L22–23 | `render_report` via `NormHost` |
| Viewer report | `…/👁️viewer/…/📊️report/🦀️.rs` L30–32 | `TableWindowKit` 4 columns |
| Catalogue | `…/✏️editor/📌️panels/📚️catalogue/🦀️.rs` L3–4 | **Placeholder headline** |
| Inspection | shared `🖥️app-surface/🦀️.rs` L238–253 | Single selected check, English labels |

Report columns (`🖥️app-surface/🦀️.rs` L159–166): `Clause`, `Status`, `Utilization`, `Message` — **no computed/limit values, no DE, no remediation**.

### 1.5 Examples / assets / tests

- **Example:** `…/📚️examples/🎬️demo/` — one DSL file identical to `Din16798Snapshot::default()` (`…/🖼️assets/🎬️demo/🗣️.dsl.semio`).
- **Compliance unit tests:** `…/🧪️tests/⚖️compliance/🦀️.rs` — 24 tests with **numeric assertions** (PMV, ventilation rates, fan energy 12 kWh, HR savings 56.53 kWh, cooling net 14 kWh, duct limit 0.147 m³/(s·m²), etc.).
- **E2E compliance test:** `…/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` L11–15 — asserts **default passes all 25 checks** (anti-pattern).
- **Mutation fixtures:** 62 scenarios under `…/🧫️fixtures/🧬️mutations/` — assert field deltas, **not compliance outcomes**.
- **Oracles:** Python independent implementation for mutations only; **no third-party DIN 16798 oracle** (`🔮️oracles/🔣️.json` L18).

---

## 2. Stub / fake detection

| # | Finding | Evidence |
|---|---------|----------|
| S1 | **Default snapshot tuned to pass** — checks can never fail on open/evaluate without editing | `📸️snapshot/🦀️.rs` L171–244 defaults; `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` L14–15 `assert!(report.all_pass())` |
| S2 | **Six subject-stored limits** — user sets both computed and limit on same document | `fan_energy_reference_kwh` L82, `hr_savings_reference_kwh` L97, `infiltration_allowance_m3_h` L105, `cooling_reference_kwh` L126, `generation_reference_kwh` L136, `storage_allowance_kwh` L150; checks at `🦀️.rs` L881, L929, L952, L1001, L1052, L1096 compare computed vs these subject fields |
| S3 | **`heat_recovery_eta_min` on subject** — minimum should come from norm tables (occupancy, climate, system class) | `🦀️.rs` L789–796 `check_heat_recovery_efficiency(eta_delivered, eta_min)` with `eta_min` from snapshot |
| S4 | **`pmv_simplified` surrogate** — linear toy, not used in checks but tested | `🦀️.rs` L470–474 `0.28 * (t_op_c - 25.0) + …`; test L4–6 |
| S5 | **PMV check ignores `comfort_category`** — always ±0.5 (Cat II) | `check_pmv_comfort` L534–543 hardcodes `limit = 0.5`; `evaluate` L144 does not pass category |
| S6 | **PPD never checked** despite ISO 7730 implementation | `ppd_from_pmv` L528–530 defined, only in test L20 |
| S7 | **`acoustic_limit_residential_db` DE-NA divergence unused** | `annex_params` L1177–1188 defines EN 30 / DE 25 dB; `check_acoustic_category` L642 uses category table L633–638, never annex |
| S8 | **`OdaClass` dead** — outdoor air quality not modelled | `🦀️.rs` L682–697; no field on snapshot, no check |
| S9 | **Infiltration simplified** `q_inf = n50·V/20` | `part_7` L946–948 comment "simplified shielding-corrected" |
| S10 | **Dwelling/residential rates folded from TRs** with ad-hoc formulas | `dwelling_ventilation_rate` L800–803 `max(0.5·A, 21·bedrooms)`; `residential_ventilation_rate` L819–822 `max(0.4·A, 30·persons)` |
| S11 | **All 25 checks always run** — no `NotApplicable`; office + dwelling + DC checks together | `check_full_environment` L143–184 unconditional `report.push` |
| S12 | **Operative temp check one-sided limit display** | `check_operative_temperature` L547–559 stores only `t_max` as limit, `utilization` 0 or 1.1 |
| S13 | **Inverse mutations are undo, not remediation** | e.g. `…/💨️change-ventilation-m3-h/↩️inverse/🦀️.rs` L9–10 restores base value |
| S14 | **Catalogue placeholder** | `…/📚️catalogue/🦀️.rs` L3–4 |
| S15 | **27 empty UI folders** | `📌️.empty.md` under editor/viewer config, presence, transient, options, … |
| S16 | **E2E test name wrong** — "nine parts", actually 10 part modules / 25 checks | `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` L11 |
| S17 | **No failing end-to-end example** — only passing default + isolated unit fail cases | compliance-report tests only `all_pass()` |
| S18 | **SFP units mis-tagged** | `check_design_sfp` L781 `QuantityKind::Power` for W/(m³/s) |
| S19 | **CO₂ tagged Dimensionless** not concentration | `check_co2_level` L604–605 |
| S20 | **Inspection intervals hardcoded** | `inspection_interval_years` L838–843 `central_mech→3`, `decentral→5` |

---

## 3. Complete subject definition (norm engineering target)

DIN EN 16798 governs **indoor environmental input parameters** (Part 1), **ventilation of non-residential buildings** (Part 3), **energy performance calculation methods** (Parts 5-1, 5-2, 7, 9, 13, 15, 17), with **DIN EN 16798-1/3 NA-DE** national deviations. A complete assessment subject:

### 3.1 Top-level entities

```
BuildingAssessment
├── metadata: { project_id, climate_zone, annex: En|De, assessment_profile }
├── outdoor_environment: { oda_class, design_theta_e_winter_c, design_theta_e_summer_c, theta_rm_series[] }
├── zones: Zone[]                    // EN 16798-1 IEQ per space
├── dwellings: Dwelling[]            // EN 16798-3 §7.1 / TR-4
├── ventilation_systems: VentSystem[] // EN 16798-3, -5-1, -5-2, -17
├── thermal_envelope: Envelope       // EN 16798-7 infiltration
├── cooling_plant: CoolingPlant?     // EN 16798-9, -13
├── dhw_storage: DhwStorage?         // EN 16798-15
└── cellars: CellarSpace[]
```

### 3.2 Zone (EN 16798-1) — per assessed space

| Field | Unit | Range / notes |
|-------|------|---------------|
| `id`, `name`, `occupancy_type` | enum | office, meeting, classroom, retail, kitchen, residential, corridor, datacentre |
| `floor_area_m2` | m² | > 0 |
| `volume_m3` | m³ | > 0 |
| `persons` | — | ≥ 0 |
| `comfort_category` | I/II/III | drives PMV/PPD, DF, noise limits |
| `comfort_model` | enum | `fixed_hvac` \| `adaptive_free_running` |
| `t_op_c`, `rh_percent`, `air_speed_m_s` | °C, %, m/s | measured/design |
| `metabolic_rate_met`, `clothing_clo` | met, clo | ISO 7730 inputs (default 1.2, 0.5 office) |
| `co2_ppm` | ppm | vs DE-NA: 1200 res / 800 classroom / 900 other |
| `df_percent` | % | Cat I ≥3, II ≥2, III ≥1.5 (informative TR-10) |
| `l_aeq_db` | dB(A) | Cat I ≤30, II ≤35, III ≤40; DE residential NA ≤25 |

### 3.3 VentSystem (EN 16798-3, -5-1, -5-2, -17)

| Field | Unit | Notes |
|-------|------|-------|
| `system_id`, `system_type` | enum | central_mech, decentral, natural, hybrid |
| `served_zone_ids` | ref[] | applicability |
| `ida_class` | 1–4 | per zone or system |
| `q_supplied_m3_h` | m³/h | per zone or aggregated |
| `q_required_m3_h` | m³/h | **computed** from Table 1 (per person + per m²) × IDA multiplier × ODA correction |
| `sfp_w_m3_s` | W/(m³/s) | vs Table 10 class bound |
| `sfp_required_class` | 1–6 | from building type / size |
| `heat_recovery` | `{ eta_t, eta_min_from_norm }` | η_min from §7.3 tables, not user input |
| `humidification` | `{ required_kg_h, provided_kg_h }` | |
| `ductwork` | `{ class A–D, test_pressure_pa, leakage_m3_s_m2 }` | EN 16798-17 |
| `fans` | `{ q_v_m3_s, t_run_h, sfp }` | |
| `inspection` | `{ last_date, interval_years }` | from system type |

### 3.4 Relationships & missing norm scope

- **Per-area outdoor air** (EN 16798-3 Table 1 col. 2): e.g. office 1.0–1.3 dm³/(s·m²) — **not implemented** (only per-person L700–707).
- **PMV/PPD categories I/II/III** (±0.2/±0.5/±0.7 PMV; PPD 6/10/15 %) — PMV Cat II only; **no PPD check**.
- **RH comfort bands** (30–70 % typical) — **no check**.
- **Air speed comfort** (e.g. ≤0.15 m/s seated) — input exists, **no check**.
- **Parts not modelled:** 16798-2 (design cond.), 16798-4 (dwellings TR — partially folded), 16798-6 (cellar TR — ad-hoc), 16798-8 (duct TR → 17), 16798-10–12, 14, 16 (folded into other checks with simplified numbers).
- **DIN NA-DE:** CO₂ limits implemented; **acoustic residential NA unused**; no explicit NA tables for SFP class selection by building category.

---

## 4. Check catalogue

| Part | Clause / table | Verified | Required inputs | Limit source | Failure meaning |
|------|----------------|----------|-----------------|--------------|-----------------|
| 1 | §7.2.2 | Operative temp in band | `occupancy`, `t_op_c` | Band by occupancy (L461–467) | Temp outside 20–24 (res) etc. |
| 1 | §7.2.2 / ISO 7730 | \|PMV\| ≤ limit | `t_op_c`, `rh_percent`, `air_speed_m_s` | **Hardcoded 0.5** (Cat II only) | Thermal discomfort |
| 1 | Annex A | Adaptive comfort | `theta_rm_c`, `t_op_c`, `comfort_category` | ±2/3/4 K (L568–573) | Free-running non-compliance |
| 1 | §6.2 + NA | CO₂ | `occupancy`, `co2_ppm`, `annex` | DE: 1200/800/900; EN: 1500/1000/1000 (L1183–1188) | IAQ failure |
| 1 | Annex B / TR-10 | Daylight factor | `comfort_category`, `df_percent` | 3/2/1.5 % (L612–617) | Insufficient daylight |
| 1 | Annex B / TR-11 | Acoustic | `comfort_category`, `l_aeq_db` | 30/35/40 dB (L633–638); **not DE res 25** | Too noisy |
| 3 | Table 1 | Outdoor air (persons only) | `occupancy`, `persons`, `ida_class`, `ventilation_m3_h` | q×persons×IDA mult (L711–713) | Underventilated |
| 3 | §7.1 / TR-4 | Dwelling ventilation | `floor_area_m2`, `bedrooms`, `dwelling_ventilation_m3_h` | `max(0.5·A, 21·n_bed)` (L800–803) | Dwelling underventilated |
| 3 | §7.2 | Residential ventilation | `floor_area_m2`, `occupants`, `residential_ventilation_m3_h` | `max(0.4·A, 30·n)` (L819–822) | Residential underventilated |
| 3 | Table 10 | SFP class | `sfp_w_m3_s`, `sfp_required_class` | Class bound 500–4500 (L728) | Fan too powerful |
| 3 | §7.3 | Heat recovery η | `heat_recovery_eta`, **`heat_recovery_eta_min`** | **User-set minimum** | HR below requirement |
| 3 | §8.1 | Inspection | `system_type`, `years_since_inspection` | 3 or 5 a (L838–843) | Overdue inspection |
| 3 | §7.4 | Humidification | `humidification_required_kg_h`, `humidification_provided_kg_h` | required ≤ provided | Insufficient humidification |
| 5-1 | §6.1 | Fan energy | `sfp`, `fan_q_v_m3_s`, `fan_t_run_h`, **`fan_energy_reference_kwh`** | **User reference** | Fan energy exceeds allowance |
| 5-1 | §6.2 | Night setback | `occupancy`, `night_setback_k` | 2–4 K (L893–898) | Insufficient setback |
| 5-2 | §6.3 | HR savings | HR inputs, **`hr_savings_reference_kwh`** | **User reference** | Savings below reference |
| 7 | §6.1 | Infiltration | `n50_h_inv`, `volume_m3`, **`infiltration_allowance_m3_h`** | **User allowance** | Infiltration exceeds allowance |
| 7 | §6.2 | Cellar | `cellar_area_m2`, `cellar_ventilation_m3_h` | 0.3·m² (L964–965) | Cellar underventilated |
| 9 | §6.1 | Cooling need | H, θ, gains, **`cooling_reference_kwh`** | **User reference** | Cooling demand too high |
| 13 | Table 5 | Chiller EER | `chiller_type`, `eer_actual` | 2.5/3.0/0.7 (L1027–1032) | Inefficient chiller |
| 13 | §6.2 | Generation energy | `q_c_kwh`, `eer_actual`, **`generation_reference_kwh`** | **User reference** | Generation energy too high |
| 13 | §7.2 / TR-16 | DC supply air | `data_center_supply_c` | 18–27 °C (L1064–1065) | DC air temp out of band |
| 15 | §6.1 | Storage losses | storage params, **`storage_allowance_kwh`** | **User allowance** | Storage losses too high |
| 15 | §7.1 / TR-14 | DHW delivery | `dhw_delivery_c` | 55–60 °C (L1102–1103) | DHW temp out of band |
| 17 | §8.2 | Duct leakage | `duct_class`, `duct_test_pressure_pa`, `duct_leakage_m3_s_m2` | f=c·p^0.65 (L1147–1149) | Leakage exceeds class |

---

## 5. Remediation strategy (per check family)

Core pattern: `CheckResult` must gain `remediation: LocalizedText[]` + `subject_ref: FieldPath` (Wave B). Per-check analytic inversions:

| Check | Remedy message (en) | Target field(s) | Formula |
|-------|---------------------|-----------------|---------|
| Operative temp | "Set operative temperature to {t_target} °C (within {t_min}–{t_max} °C for {occupancy})" | `zones[].t_op_c` | clamp to band |
| PMV | "Adjust t_op to ≈{t_op_inv} °C for PMV ≤ {limit} (Cat {cat})" | `t_op_c` | bisect `pmv_iso7730` |
| CO₂ | "Increase ventilation to ≥{q_req} m³/h or reduce occupancy to ≤{n_max}" | `ventilation_m3_h` or `persons` | `q = (co2-400)·V/(outdoor_rate)` simplified, or raise air change |
| Outdoor air P3 | "Raise supply airflow to ≥{required} m³/h ({ida_class}, {persons} persons)" | `ventilation_m3_h` | `outdoor_air_per_person × persons × mult` |
| Dwelling | "Raise dwelling ventilation to ≥{max(0.5·A,21·bed)} m³/h" | `dwelling_ventilation_m3_h` | L800–803 |
| SFP | "Reduce SFP to ≤{class_bound} W/(m³/s) or upgrade to class {n}" | `sfp_w_m3_s` or `sfp_required_class` | Table 10 bound |
| HR η | "Increase heat recovery efficiency to ≥{eta_min_norm}" | `heat_recovery_eta` | lookup η_min from norm table |
| Fan energy | "Reduce fan energy to ≤{E_allow} kWh (lower SFP, q_v, or t_run)" | `sfp_w_m3_s`, `fan_q_v_m3_s`, `fan_t_run_h` | `E = SFP·q_v·t_run/1000` |
| Infiltration | "Improve airtightness to n50 ≤ {n50_max} 1/h" | `n50_h_inv` | invert `n50·V/20 ≤ allowance` |
| Duct leakage | "Reduce leakage to ≤{limit} m³/(s·m²) at {p} Pa or upgrade to class {X}" | `duct_leakage_m3_s_m2` | `c·p^0.65` |
| Cooling | "Reduce net cooling need to ≤{Q_ref} kWh (lower H_tr, θ_set, or gains)" | envelope / setpoints | invert degree-hour formula |

**Remove from subject:** all `*_reference_kwh`, `infiltration_allowance_m3_h`, `heat_recovery_eta_min` — compute limits from norm + building context.

---

## 6. Report & UX gaps

| Gap | Current | Required |
|-----|---------|----------|
| Pass/fail clarity | `Status` Debug string | Localized Pass/Fail/NA badges (en+de) |
| Computed vs limit | Not shown in table | Two numeric columns with units |
| Remediation | None | Actionable text per failed check |
| Applicability | All checks always shown | Hide/mark NA by occupancy & system scope |
| Editor | JSON blob (`inputs/🦀️.rs` L19–20) | Structured forms per zone/system with units |
| Localization | Window titles en/de (`LocalizedLabel`); report **English only** | Clause titles, messages, remediation in en+de |
| Catalogue | Placeholder | Browse clauses/parts with links |
| Inspection panel | Shows one check, no remedy | Full detail + suggested edits |
| Default experience | Opens compliant | Open with realistic partial failures |

---

## 7. Target design

### 7.1 Snapshot sketch (replace flat 62-scalar)

```rust
pub struct Din16798Snapshot {
    pub annex: AnnexChoice,
    pub climate: ClimateContext { theta_rm_c, oda_class, … },
    pub zones: Vec<Zone>,           // IEQ inputs per space
    pub dwellings: Vec<Dwelling>,   // optional
    pub vent_systems: Vec<VentSystem>,
    pub envelope: Envelope { n50_h_inv, volume_m3, cellar: … },
    pub cooling: Option<CoolingPlant>,
    pub dhw: Option<DhwStorage>,
    // NO reference_kwh fields on subject
}
```

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &Din16798Snapshot) -> CheckReport {
    let annex = AnnexParams::for_choice(doc.annex);
    let mut report = CheckReport::default();
    for zone in &doc.zones {
        report.extend(evaluate_zone_ieq(zone, &annex));
        if zone.needs_mechanical_ventilation() {
            report.extend(evaluate_zone_ventilation(zone, &doc.vent_systems, &annex));
        }
    }
    for dwelling in &doc.dwellings { report.extend(evaluate_dwelling(dwelling)); }
    for sys in &doc.vent_systems { report.extend(evaluate_vent_system(sys, &annex)); }
    if let Some(c) = &doc.cooling { report.extend(evaluate_cooling(c, &doc.envelope)); }
    // …
    report
}
```

### 7.3 Example subjects

**Compliant office (Cat II, IDA2):** 200 m² open office, 20 persons, `t_op_c=23`, PMV≈0, `co2_ppm=900`, `ventilation_m3_h=720` (36×20), `sfp=1200` class 3, duct class C @ 400 Pa, 0.08 leakage.

**Non-compliant multi-failure:** Same office but `co2_ppm=1100` (fail DE-NA 900), `ventilation_m3_h=500` (fail 720), `sfp_w_m3_s=2000` with `sfp_required_class=3` (fail 1250), `l_aeq_db=38` (fail Cat II 35), `t_op_c=28` (fail PMV & operative band).

### 7.4 Worked-example tests to add

1. Office IDA2 ventilation: 10 persons × 36 m³/(h·p) = 360 m³/h required — **already tested** L66–68.
2. **Failing evaluate():** mutate default `co2_ppm=950` with `annex=de` → expect Fail on CO₂.
3. PMV Cat I: `comfort_category=I`, `t_op_c=24` → expect Fail at ±0.2 (not implemented yet).
4. SFP remediation: given fail at 2000 vs class 3 (1250), assert remedy suggests ≤1250.
5. Remove `all_pass()` on default; replace with scenario-specific pass/fail fixtures.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema | `🧬️schema/🦀️.rs`, `📸️snapshot/*`, `🔺️diff/*`, `🛰️.proto`, `🔣️.json` |
| Compliance helpers | `part_1`…`part_17`, `annex_params` in `🧬️schema/🦀️.rs` |
| Evaluate | `💡️inferences/🦀️.rs` |
| Mutations | Regenerate from new schema; drop reference-field mutations |
| Inverse | `↩️inverse/` → compliance remediation solvers |
| Editor | `📥️inputs/` structured forms; `📊️results/` enhanced columns |
| Viewer | `📊️report/🦀️.rs` |
| Examples | `📚️examples/` compliant + non-compliant DSL |
| Tests | `⚖️compliance/`, `🔬️compliance-report/`, oracles |
| Core (Wave B) | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` |

---

## 8. Risks / open questions

1. **Scope boundary with `⚡️din18599`:** `residential_ventilation_rate` comment L818 says "consumed by DIN V 18599" — clarify which family owns residential ventilation compliance to avoid duplicate/conflicting checks.
2. **Folded TR guidance:** Daylight, acoustic, dwelling, cellar, DC, DHW TRs use simplified constants — need product decision: cite informative TR values or require full TR implementation?
3. **PMV defaults:** ISO 7730 needs metabolic rate & clothing per occupancy — currently hardcoded 1.2 met / 0.5 clo (L499–500); must become zone fields.
4. **Adaptive vs fixed conflict:** `evaluate` runs both fixed-band and adaptive checks simultaneously (L143–145) — norm requires choosing model per building/zone.
5. **Wave B dependency:** Remediation/localization blocked on `CheckResult` extension — coordinate with `📓️impl-core.md`.
6. **62 mutation oracle debt:** Schema reshape invalidates all mutation fixtures and Python oracle — budget full regeneration.
7. **Third-party oracle gap:** No external DIN 16798 validation tool registered — accept native second-implementation only for mutations; compliance numbers need manual norm cross-check or spreadsheet oracle.
8. **Applicability rules:** Coordinator must define rule matrix (which checks apply to which `occupancy` × `system_type`) before Wave C to prevent false failures on dwelling/DC checks for offices.

---

*Auditor: read-only Wave A. Family dir: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798`. Lines refer to `🏅️standards/🔖️1/🪆️subsets/✳️any/` subtree unless prefixed.*
