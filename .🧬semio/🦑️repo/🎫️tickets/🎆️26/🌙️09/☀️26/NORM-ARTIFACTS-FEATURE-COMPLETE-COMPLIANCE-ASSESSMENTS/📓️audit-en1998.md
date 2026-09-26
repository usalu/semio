# Audit — EN 1998 (`🫨️en1998`)

**Executive summary.** The EN 1998 artifact is a **wide but shallow** compliance demo: one flat `En1998Snapshot` with **49 unrelated scalars** (lines 13–112 of `📸️snapshot/🦀️.rs`) is always evaluated by a single `check_full_seismic` that emits **exactly 12 utilization checks** across parts 1–6 (`💡️inferences/🦀️.rs:76–147`). Helper math for spectra, base shear, silo/tank hydrodynamics, Mononobe-Okabe, and retrofit confidence factors is **real and numerically tested** (`🧪️tests/⚖️compliance/🦀️.rs`), but the **subject is not a building/bridge/silo**—it is a kitchen-sink parameter bag. **No remediation**, **no localized report text**, **no structured editor**, and **inverse mutations are undo-only** (not compliance inversions). DE-NA is **partial** (zones 0–3 only; ground A–E not DIN subsoil R/S/T combos; `seismic_zone=4` silently maps to zone 2). Many helpers (`part_6::along_wind_*`, `bearing_displacement_limit_mm`, `silo_behaviour_factor` in evaluate) are **dead code**. Default inputs are tuned so all 12 checks **pass**; fixture names like `raises-v-rd-kn-to-925-0` are **hand-picked mutation demos**, not analytic remedies. Feature-complete compliance assessment is **not met**.

---

## 1. Inventory

### 1.1 Snapshot / document fields (49 scalars, flat, no children)

Source: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:13–112`.

| Group | Field | Type | Unit / enum | Default (`:126–176`) |
|-------|-------|------|-------------|----------------------|
| **EN 1998-1 building** | `seismic_zone` | `u8` | DE zone 0–3 (parser) | 2 |
| | `ground_type` | `String` | `a`–`e` | `"b"` |
| | `importance_class` | `String` | `cc1`–`cc4` | `"cc2"` |
| | `structural_system` | `String` | see §3 | `"moment_frame_dch"` |
| | `t1_s` | `f64` | s | 0.3 |
| | `mass_t` | `f64` | t | 500 |
| | `v_rd_kn` | `f64` | kN (capacity) | 800 |
| | `drift_mm` | `f64` | mm (demand) | 20 |
| | `height_m` | `f64` | m | 12 |
| | `multiple_resisting_systems` | `bool` | — | true |
| | `annex` | `String` | `de` / `en` | `"de"` |
| | `en_a_gr` | `f64` | g | 0.15 |
| | `en_ground_type` | `String` | `a`–`e` | `"b"` |
| | `en_spectrum_type` | `String` | `type1` / `type2` | `"type1"` |
| **EN 1998-2 bridge** | `period_ratio` | `f64` | — | 2.0 |
| | `bridge_v_rd_kn` | `f64` | kN | 600 |
| | `bearing_d_ed_mm` | `f64` | mm | 120 |
| | `bearing_d_rd_mm` | `f64` | mm | 250 |
| **EN 1998-3 retrofit** | `retrofit_knowledge_level` | `String` | `kl1`–`kl3` | `"kl2"` |
| | `retrofit_limit_state` | `String` | DL / SD / NC | `"significant_damage"` |
| | `retrofit_e_d_kn` | `f64` | kN | 250 |
| | `retrofit_r_k_kn` | `f64` | kN | 400 |
| | `retrofit_gamma_el` | `f64` | — | 1.0 |
| **EN 1998-4 silo/tank** | `silo_height_m`, `silo_radius_m` | `f64` | m | 10, 5 |
| | `silo_n_rd_kn`, `silo_v_ed_kn`, `silo_v_rd_kn` | `f64` | kN | 500, 180, 300 |
| | `silo_q_nominal` | `f64` | — | 2.0 |
| | `tank_height_m`, `tank_radius_m`, `tank_mass_t` | `f64` | m, m, t | 8, 4, 300 |
| | `tank_v_rd_kn` | `f64` | kN | 400 |
| **EN 1998-5 foundation/wall** | `foundation_area_m2` | `f64` | m² | 100 |
| | `foundation_p_rd_kpa` | `f64` | kPa | 500 |
| | `foundation_h_ed_kn`, `foundation_h_rd_kn` | `f64` | kN | 150, 400 |
| | `k_foundation`, `k_soil` | `f64` | kN/m | 500000, 200000 |
| | `wall_height_m`, `wall_phi_deg` | `f64` | m, ° | 4, 30 |
| | `wall_soil_gamma_kn_m3`, `wall_r` | `f64` | kN/m³, — | 18, 1.5 |
| | `wall_h_rd_kn` | `f64` | kN/m | 150 |
| **EN 1998-6 tower** | `tower_m_ed_knm`, `tower_m_rd_knm` | `f64` | kNm | 1200, 2500 |
| | `tower_is_chimney` | `bool` | — | true |
| | `tower_q_nominal`, `tower_mass_t` | `f64` | —, t | 2.5, 80 |

**Composed children:** none. `En1998Outline::compute` ignores snapshot content and sets `entry_count = 0` (`💡️inferences/🧾outline/🦀️.rs:74–78`).

### 1.2 Mutations (49 kinds)

`En1998Mutation` enum — one scalar field each (`🧬️mutations/🦀️.rs`). All `↩️inverse/🦀️.rs` files restore **pre-change base value** only (e.g. `🛡️change-v-rd-kn/↩️inverse/🦀️.rs:9–10`), not a compliance target.

Oracle: `🔮️oracles/🔣️.json` — Python second implementation for **mutation codec only** (`en1998-1-mutate`); **no evaluate oracle**.

### 1.3 `evaluate()` call graph

```
evaluate (💡️inferences/🦀️.rs:232)
 └─ check_full_seismic (💡️inferences/🦀️.rs:76)
      ├─ parse_* (💡️inferences/🦀️.rs:150–228)
      ├─ AnnexParams::ground_params (🦀️.rs:674–684)
      ├─ part_1::elastic_response_spectrum_type1 (🦀️.rs:593)
      ├─ part_1::design_spectrum_sd (🦀️.rs:607)
      ├─ check_building_seismic_with_annex (🦀️.rs:948) → check_base_shear, check_drift
      ├─ part_2::isolation_reduction_factor, isolated_spectrum_sd, check_bridge_seismic, check_isolation_bearing
      ├─ part_3::check_element_capacity
      ├─ part_4::impulsive/convective periods & mass ratios, silo_base_shear_kn, tank_base_shear_kn, check_silo_*, check_tank_*
      ├─ part_5::bearing_reduction_factor, seismic_bearing_pressure_kpa, check_foundation_*, mononobe_okabe, retaining_wall_thrust, check_retaining_wall_sliding
      └─ part_6::tower_behaviour_factor, design_spectrum_sd, cantilever_modal_participation_factor, check_tower_overturning
```

**Computed but discarded in evaluate:** `part_4::silo_behaviour_factor` (`💡️inferences/🦀️.rs:117`), `part_5::radiation_damping(stiffness_ratio(...))` (`:134`), `part_6::tower_base_shear_kn` (`:144`).

**Never called from evaluate:** `part_2::bearing_displacement_limit_mm` (`🦀️.rs:710`), `part_6::along_wind_overturning_knm`, `critical_wind_speed_m_s`, `tower_frequency_hz` (`🦀️.rs:899–914`).

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `✏️editor/.../📥️inputs/🦀️.rs:19–21` | Pretty-printed **JSON blob** only (`render_document_json`) |
| Results | `✏️editor/.../📊️results/🦀️.rs:22–24` | `render_report` — clause, status, u, English message |
| Viewer report | `👁️viewer/.../📊️report/🦀️.rs:30–32` | Same table via `report_table_columns/rows` |
| Window labels | `📥️inputs/🦀️.rs:14`, `📊️results/🦀️.rs:17` | **Localized** en/de titles only |

Report columns (`🖥️app-surface/🦀️.rs:159–166`): `Clause`, `Status`, `Utilization`, `Message` — **English headers, English messages**, `format!("{:?}", check.status)`.

### 1.5 Examples / assets / tests

| Asset | Path | Notes |
|-------|------|-------|
| Example | `📚️examples/🏢️seismic-rc-frame/` | Single DSL line; label **not** German (`🦀️.rs:7`: `"Seismic Rc Frame"` both locales) |
| DSL asset | `🖼️assets/.../🗣️.dsl.semio` | Zone 3, `dual_system`, EN annex type2 — still flat scalars |
| Compliance unit tests | `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | **Numeric** spectrum, base shear, SRSS, Mononobe-Okabe, tank 412.86 kN |
| E2E evaluate | `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | Asserts `checks.len() == 12` only |
| Mutation tests | 49 × `🧬️mutations/*/🧪️tests/` | Codec/diff round-trip; **no link to evaluate** |
| Editor UI | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:174`, results `:14` | `!checks.is_empty()` / `!contains("No checks computed")` |

**Miswired pilot languages:** root `🦀️.rs:87–131` registers `en1998.*` grammars from **`semio_s_artifact_norm_en1997`** paths.

---

## 2. Stub / fake detection

| Issue | Evidence |
|-------|----------|
| Flat scalar bag, not a structure | `En1998Snapshot` 49 fields, no storeys/members (`📸️snapshot/🦀️.rs:13–112`) |
| All parts run on every document | `check_full_seismic` always pushes 12 checks (`💡️inferences/🦀️.rs:94–145`) |
| Hardcoded ductility **DCM** for drift | `check_building_seismic_with_annex` uses `DuctilityClass::Dcm` regardless of `structural_system` (`🦀️.rs:966`) |
| Hardcoded **ν = 1.0** drift factor | `drift_limit_mm(..., 1.0)` (`🦀️.rs:966`) |
| Hardcoded **η = 1.0** in spectrum | `elastic_response_spectrum_type1` (`🦀️.rs:594`) |
| `structural_system` `"wall_dcm"` stored but **not parsed** | Fixture `🏗️switches-structural-system-to-wall-dcm` stores string; `parse_structural_system` has no `wall_dcm` → defaults `MomentFrameDch` (`💡️inferences/🦀️.rs:178–187`) |
| `seismic_zone = 4` invalid for DE | Fixture `🫨️raises-seismic-zone-to-4` sets zone 4; parser `_ => Zone2` (`💡️inferences/🦀️.rs:150–156`) — **same a_g as zone 2** |
| DE ground **A–E only**, not DIN **R/S/T** combos | `na_de::GroundType` enum (`🦀️.rs:477–483`) |
| Tank **linear** combine, silo **SRSS** | `tank_base_shear_kn` additive (`🦀️.rs:809`); `silo_base_shear_kn` SRSS (`:804`) — inconsistent §4 models |
| `silo_q_nominal` / `tower_q_nominal` unused in checks | Computed then `_ = silo_behaviour_factor(...)` (`💡️inferences/🦀️.rs:117`); tower q only affects unused `s_d_tower` path for shear |
| Foundation bearing uses **building** `v_b` / area | `seismic_bearing_pressure_kpa(v_b, area)` (`💡️inferences/🦀️.rs:131`) — not vertical load combination |
| Retaining wall: **H_ed supplied**, thrust computed but check uses `h_ed_wall` vs `wall_h_rd_kn` | Demand from formula (`💡️inferences/🦀️.rs:138–139`); capacity is free scalar |
| Tower overturning: **M_ed user scalar** | No computation from `tower_mass_t`, spectrum (`💡️inferences/🦀️.rs:145`) |
| Inverse mutations ≠ remediation | `inverse` restores base (`🛡️change-v-rd-kn/↩️inverse/🦀️.rs:9–10`) |
| Fixture "raises X to Y" not derived from failure | e.g. `925.0` for `v_rd_kn` — hand-picked (`🛡️raises-v-rd-kn-to-925-0`) |
| Default snapshot **all-pass tuned** | v_rd 800 > V_b ≈ 460 kN; drift 20 < limit 84 mm; etc. |
| E2E tests count-only | `assert_eq!(report.checks.len(), 12)` (`💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:6`) |
| No `todo!` / placeholder text in compliance | — |
| Empty UI slots | 27× `📌️.empty.md` under editor/viewer |
| `CheckResult` no remediation field | `⚖️compliance/🦀️.rs:138–146` |

---

## 3. Complete subject definition (engineering target)

A feature-complete EN 1998 artifact must model **scoped subjects** with typed relationships, not one global scalar sheet.

### 3.1 EN 1998-1 — General rules (buildings)

- **Site:** reference peak ground acceleration `a_gR` or DE **seismic zone 0–3** (DIN EN 1998-1/NA Table NA.1); **importance class** I–IV → γ_I (Table 4.3); optional **topography**.
- **Ground:** EN types A–E **or** DE-NA **subsoil class** (Einzelwerte R, S, T and Kombinationen A-R … E-T per NA Tabelle zu 3.1); spectrum type 1/2; `(S, T_B, T_C, T_D)`.
- **Structural system:** material (RC/steel/masonry/timber), **ductility class** (DCL/DCM/DCH), system typology (moment frame, wall, dual, inverted pendulum, …), **q** from Table 6.1 (material-specific EN 1992/1993/1996/1995 rules).
- **Geometry & mass:** storeys (height, mass, tributary area), **T₁** (measured or C_T·h^(3/4)), higher modes if needed.
- **Analysis path:** regularity (plan/elevation), **lateral force method** (V_b, force distribution, torsion) **or** **modal** (≥90% mass), accidental eccentricity.
- **Checks:** base shear ULS; drift SLS (ν, ρ, θ limits §4.3.3.4); **P-Δ** (θ_Δ limit); member capacity (delegated to material Eurocodes with seismic combinations from EN 1990).

### 3.2 EN 1998-2 — Bridges

- Deck/isolation system, **q_isol**, bearing devices (stiffness, displacement capacity), bridge static scheme; **not** shared building mass.

### 3.3 EN 1998-3 — Assessment/retrofitting

- Building model, **KL/KT/KC**, CF, limit state (DL/SD/NC), element catalog with `E_d`, `R_k`, γ_el; global vs local intervention paths.

### 3.4 EN 1998-4 — Silos/tanks

- Geometry, contents, shell material, anchorage, impulsive/convective split, **q** capped; interaction with EN 1998-1 spectrum at structure site.

### 3.5 EN 1998-5 — Foundations / retaining

- Foundation type, soil profile, **K**-ratio, radiation damping, sliding resistance, bearing under seismic; retaining wall geometry, groundwater, **K_AE**, wall friction.

### 3.6 EN 1998-6 — Towers/masts/chimneys

- Dynamic properties (f₁, ξ), wind vs seismic load cases, **q** by typology; along-wind if governing.

**Valid ranges (examples):** `a_gR` > 0; DE zone ∈ {0,1,2,3}; γ_I ∈ [0.8, 1.4]; q ≥ 1.0; T₁ > 0; drift θ ≤ limit per ductility class.

---

## 4. Check catalogue (implemented)

| # | Part | Clause (as coded) | Verified | Required inputs | Limit source | Failure meaning |
|---|------|-------------------|----------|-----------------|--------------|-----------------|
| 1 | 1 | EN 1998-1 §4.3.4 | V_b ≤ V_rd | zone/annex, ground, γ_I, q, T₁, m, v_rd_kn | V_rd user | Insufficient lateral resistance |
| 2 | 1 | EN 1998-1 §4.3.3 | drift ≤ limit | drift_mm, height_m, ρ, **DCM fixed**, ν=1 | θ=0.007·ρ·ν·h (DCM) | Excessive drift |
| 3 | 2 | EN 1998-2 §5.3 | V_bridge ≤ V_rd | period_ratio, bridge_v_rd_kn, shared spectrum | bridge_v_rd_kn | Bridge shear overload |
| 4 | 2 | EN 1998-2 §7.5 | d_ed ≤ d_rd | bearing_d_ed_mm, bearing_d_rd_mm | bearing_d_rd_mm | Bearing displacement exceeded |
| 5 | 3 | EN 1998-3 §2.3.* | E_d ≤ R_k/(CF·γ_el) | retrofit_* scalars | computed R_d | Retrofit capacity insufficient |
| 6 | 4 | EN 1998-4 §3.4 | N_ed ≤ N_rd | silo geometry, shared S_d, silo_n_rd_kn | silo_n_rd_kn | Silo wall overload |
| 7 | 4 | EN 1998-4 §3.5 | V_ed ≤ V_rd | silo_v_ed_kn, silo_v_rd_kn | silo_v_rd_kn | Anchorage failure |
| 8 | 4 | EN 1998-4 §4.3 | V_tank ≤ V_rd | tank geometry/mass, spectra | tank_v_rd_kn | Tank shear overload |
| 9 | 5 | EN 1998-5 §7.3 | p_ed ≤ p_rd | v_b, area, p_rd, a_g | p_rd·(1−1.5a_g) min 0.5 | Bearing pressure |
| 10 | 5 | EN 1998-5 §7.4 | H_ed ≤ H_rd | foundation_h_ed_kn, foundation_h_rd_kn | h_rd | Sliding |
| 11 | 5 | EN 1998-5 §6 E.2 | H_ed ≤ H_rd | wall soil, φ, r, height, wall_h_rd_kn | wall_h_rd_kn | Wall sliding |
| 12 | 6 | EN 1998-6 §4.3.2 | M_ed ≤ M_rd | tower_m_ed_knm, tower_m_rd_knm | m_rd | Overturning |

**DE vs EN divergence (implemented):** same nominal a_g=0.15, DE zone2/B gives S_e(0.3s)=0.375 g; EN type1/B gives 0.45 g (`🧪️tests/⚖️compliance/🦀️.rs:64–72`). DE zones: a_g 0 / 0.08 / 0.15 / 0.24 (`🦀️.rs:465–471`).

**Not implemented (claimed family scope):** lateral force distribution, T₁ estimate, regularity, P-Δ θ, multi-mode, accidental torsion, material-specific ductility q tables, steel/masonry rules, bridge deck isolation detail, silo q in demand, radiation damping effect, wind towers, bearing limit helper.

---

## 5. Remediation strategy (per check — target behaviour)

Shared pattern: extend `CheckResult` (Wave B core) with `remediation: Vec<RemediationStep { field_path, current, required, unit, clause, locale }>`.

| Check | Remedy target field(s) | Analytic inversion (from current code) |
|-------|------------------------|------------------------------------------|
| 1 Base shear | `v_rd_kn` **or** reduce demand via `mass_t`, `t1_s`, `structural_system` (q), `importance_class`, site | `v_rd_kn ≥ V_b = S_d(T₁)·m·9.81` with `S_d = S_e·γ_I/q` (`🦀️.rs:607–618, 964`) |
| 2 Drift | `drift_mm` **or** `height_m`, `multiple_resisting_systems` | `drift_mm ≤ ν·ρ·θ·h·1000` (`🦀️.rs:631–637`); if ρ=1.3 needed, set `multiple_resisting_systems=true` |
| 3 Bridge shear | `bridge_v_rd_kn` | `≥ V_b·(γ_I/q_isol)` with `q_isol=max(1,T_b/T_d)²` (`🦀️.rs:700–707`) |
| 4 Bearing disp. | `bearing_d_rd_mm` | `≥ bearing_d_ed_mm` |
| 5 Retrofit | `retrofit_r_k_kn` **or** lower `retrofit_e_d_kn`, `retrofit_gamma_el`, improve KL | `r_k_kn ≥ e_d_kn·CF·γ_el` (`🦀️.rs:767–774`) |
| 6 Silo wall | `silo_n_rd_kn` | `≥ V_silo` from SRSS impulsive/convective (`💡️inferences/🦀️.rs:106–115`) |
| 7 Silo anchor | `silo_v_rd_kn` | `≥ silo_v_ed_kn` |
| 8 Tank | `tank_v_rd_kn` | `≥ (m_i·S_e(T_i)+m_c·S_e(T_c))·9.81` (`🦀️.rs:809`) |
| 9 Foundation bearing | `foundation_p_rd_kpa` **or** `foundation_area_m2` | `p_rd·red ≥ v_b/area`; increase area or capacity |
| 10 Foundation sliding | `foundation_h_rd_kn` | `≥ foundation_h_ed_kn` |
| 11 Retaining wall | `wall_h_rd_kn` **or** reduce `wall_height_m`, `wall_phi_deg`, site a_g | `h_rd ≥ 0.5·γ·H²·K_AE(φ,k_h)` (`🦀️.rs:876–886`) |
| 12 Tower | `tower_m_rd_knm` | `≥ tower_m_ed_knm` (future: compute M_ed from Γ·S_d·m·g·h) |

Mutation `↩️inverse` should be replaced by **forward compliance inverse** emitting the target scalar from formulas above.

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| No remediation | `CheckResult.message` is static English (`🦀️.rs:647–652`) |
| Report not localized | Columns/messages English (`🖥️app-surface/🦀️.rs:159–166`) |
| No subject element reference | Cannot point to storey/member/bearing |
| Inputs = raw JSON | No field validation, units, enums (`📥️inputs/🦀️.rs:19–21`) |
| No pass/fail grouping | Flat list of 12 |
| Viewer = editor table | No drill-down to formula terms |
| Example i18n | `seismic-rc-frame` label not German (`📚️examples/🏢️seismic-rc-frame/🦀️.rs:7`) |
| `selected_check` command | View-only; no remediation action (`☑️selected-check/`) |

---

## 7. Target design

### 7.1 Snapshot sketch (scoped)

```rust
pub struct En1998Project {
    pub annex: AnnexChoice,
    pub site: SiteSeismic { zone_de: Option<DeZone>, a_gr: f64, ground: GroundModel, importance: ImportanceClass },
    pub buildings: Vec<Building>,      // EN 1998-1
    pub bridges: Vec<Bridge>,          // EN 1998-2
    pub assessments: Vec<RetrofitCase>,// EN 1998-3
    pub silos: Vec<Silo>, pub tanks: Vec<Tank>, // EN 1998-4
    pub foundations: Vec<Foundation>, pub retaining_walls: Vec<RetainingWall>, // EN 1998-5
    pub towers: Vec<Tower>,            // EN 1998-6
}
```

`evaluate(project)` → only runs checks for **non-empty** scoped children; each check carries `subject_id`.

### 7.2 `evaluate()` structure

1. Resolve site spectrum once (`AnnexParams`).
2. For each building: compute T₁, S_d, V_b, distribution (future), drift, P-Δ.
3. For each other scope: part-specific demand → `CheckResult` + `remediation`.
4. Aggregate `CheckReport` grouped by part and subject.

### 7.3 Worked examples

**Compliant (DE office, zone 2, RC DCH frame):** 4-storey, T₁=0.35 s, m=450 t, V_rd=520 kN, drift=18 mm @ h=14 m → V_b≈414 kN, drift limit≈98 mm → all pass (derive in test like `🧪️tests/⚖️compliance/🦀️.rs:49–61`).

**Non-compliant (multi-failure):** Same but `v_rd_kn=350`, `drift_mm=95`, `multiple_resisting_systems=false` → checks 1 & 2 fail; remediation: `v_rd_kn≥414`, `drift_mm≤68` (ρ=1.3).

**Numeric test to add:** `full_seismic_default_passes` asserting each of 12 statuses `Pass` with expected utilizations ±1e-3; `seismic_rc_frame_example_fails_base_shear` on asset DSL.

### 7.4 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema | `🧬️schema/📸️snapshot/🦀️.rs`, `🔺️diff/`, `🧬️mutations/` (regenerate 49→field-scoped) |
| Compliance | `🧬️schema/🦀️.rs` (`part_1`–`part_6`, `check_building_*`), `💡️inferences/🦀️.rs` |
| Core | `⚖️compliance/🦀️.rs` (remediation model) |
| Editor | `✏️editor/.../📥️inputs/` structured forms; `📊️results/` remediation UI |
| Viewer | `👁️viewer/.../📊️report/` localized columns |
| Examples | `📚️examples/` compliant + non-compliant per scope |
| Tests | `🧪️tests/⚖️compliance/`, `💡️inferences/🧪️tests/`, per-mutation compliance inverses |
| Oracles | `🔮️oracles/` evaluate cross-implementation |
| Root | `🦀️.rs` fix en1997 grammar wiring (`:87–131`) |

---

## 8. Risks / open questions

1. **Scope:** Is one artifact one **project** (multi-building) or one **structure**? Current flat schema implies both simultaneously.
2. **DE-NA depth:** Implement full DIN subsoil R/S/T matrix or map to equivalent A–E?
3. **Material Eurocodes:** Seismic member checks stay in EN 1998-1 plugin or delegate to en1992/en1993 artifacts?
4. **Demand vs capacity inputs:** Many `*_ed_*` are user-supplied — should plugin compute E_d from spectrum + FEM?
5. **Mutation explosion:** 49 field mutations don't scale to nested schema — move to generic `set_field` with schema validation?
6. **Wave B dependency:** Remediation UX blocked on `CheckResult` extension (`📓️coordination.md`).
7. **Oracle gap:** No third-party evaluate reference; acceptable for greenfield?
8. **`seismic_zone` u8 vs enum:** Zone 4 fixture — reject invalid or extend model?

---

*Auditor: read-only pass, 2026-09-26. Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998`.*
