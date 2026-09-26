# Audit — EN 1991 (`🏋️en1991`)

**Executive summary:** The EN 1991 artifact is a **flat bag of 32 independent scalars** (floor bay + fire + snow + wind + thermal + construction + accidental + bridge + crane + silo parameters) with **`evaluate()` emitting 11 checks** that mix real formulae with **hardcoded limits, fixed coefficients, and two checks that can never fail**. Default snapshot is tuned so `full_actions_de_na_numeric` asserts `all_pass()` (`💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:30`). There is **no remediation**, report strings are **English-only**, and the editor exposes subject fields only as **pretty-printed JSON**. DIN EN NA divergences (snow zones 1a/2a/3a, altitude formula, roof μ, drifts, wind orography, LM1 detail, tank loads, explosion models) are largely absent. **Wave C must replace the subject model and checks; Wave B must add remediation + localized reporting before this family is feature-complete.**

---

## 1. Inventory

### 1.1 Snapshot / document fields (`En1991Snapshot`, 32 scalars)

Source: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:17–94`, defaults `:105–141`.

| Field | Type | Unit | Default | Part domain |
|-------|------|------|---------|-------------|
| `area_m2` | f64 | m² | 50 | 1-1 imposed |
| `category` | `ImposedCategory` | — | B (office) | 1-1 |
| `annex` | `AnnexChoice` | — | De | all NA |
| `self_weight_material` | String | — | `reinforced_concrete` | 1-1 |
| `self_weight_thickness_m` | f64 | m | 0.2 | 1-1 |
| `assumed_g_k_kn_m2` | f64 | kN/m² | 6.0 | 1-1 |
| `fire_curve` | `FireCurve` | — | Standard | 1-2 |
| `fire_resistance_min` | f64 | min | 30 | 1-2 |
| `fire_member_capacity_c` | f64 | °C | 900 | 1-2 |
| `snow_zone` | u8 | — | 2 | 1-3 |
| `snow_altitude_m` | f64 | m | 150 | 1-3 |
| `en_s_k_kn_m2` | f64 | kN/m² | 0.85 | 1-3 (EN path) |
| `wind_zone` | u8 | — | 2 | 1-4 |
| `en_v_b_m_s` | f64 | m/s | 25 | 1-4 (EN path) |
| `delta_t_k` | f64 | K | 30 | 1-5 |
| `construction_activity` | String | — | `scaffolding` | 1-6 |
| `accidental_mass_t` | f64 | t | 30 | 1-7 |
| `accidental_speed_km_h` | f64 | km/h | 80 | 1-7 |
| `bridge_lane` | u8 | — | 1 | 2 |
| `bridge_span_m` | f64 | m | 20 | 2 |
| `bridge_lane_width_m` | f64 | m | 3 | 2 |
| `bridge_moment_resistance_knm` | f64 | kNm | 3000 | 2 (capacity, not action) |
| `crane_class` | String | — | HC2 | 3 |
| `hoist_class` | String | — | HC2 | 3 |
| `hoisting_speed_m_s` | f64 | m/s | 0.5 | 3 |
| `silo_bulk_density_kn_m3` | f64 | kN/m³ | 8 | 4 |
| `silo_height_m` | f64 | m | 12 | 4 |
| `silo_hydraulic_radius_m` | f64 | m | 1.5 | 4 |
| `silo_mu` | f64 | — | 0.4 | 4 |
| `silo_k` | f64 | — | 0.4 | 4 |
| `c_s` | f64 | — | 1.0 | 1-4 |
| `c_d` | f64 | — | 1.0 | 1-4 |

No collections, geometry, load cases, zones, envelope elements, or member graph. Comment at `🧬️mutations/🦀️.rs:7–9` explicitly calls this a **flat bag**.

### 1.2 Composed children / facets

- **Schema:** `🧬️schema/🦀️.rs` — artifact + compliance helpers `part_1_1`…`part_4`, `na_de`.
- **Snapshot facets:** Rust, text DSL, binary pack, JSON, GraphQL, proto under `📸️snapshot/`.
- **Diff / mutations:** 32 `change-*` mutations with diff + inverse (`🧬️mutations/🦀️.rs:73–106`, `KINDS` `:113–144`).
- **Inferences:** `En1991Inference { outline }` only (`💡️inferences/🦀️.rs:20–23`); compliance lives in same file `:68–124`.
- **IO:** reuses EN 1990 document grammars (`🦀️.rs:96–148`).
- **Editor:** play app with Inputs (JSON), Results, panels (`✏️editor/`).
- **Viewer:** read-only Report table (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`).
- **Oracles:** mutation-only Python second implementation (`🔮️oracles/🔣️.json`); **no evaluate/compliance oracle**.

### 1.3 `evaluate()` call graph

`evaluate` → `check_full_actions` (`💡️inferences/🦀️.rs:121–122`, `:92–117`):

```
check_full_actions
├── part_1_1::check_imposed
├── part_1_1::check_self_weight
├── part_1_2::check_fire_action
│   └── gas_temperature_c → standard/external/hydrocarbon curves
├── part_1_3::design_ground_snow_load → ground_snow_load_zone / altitude_correction
│   └── roof_snow_load(s_k, μ=0.8 hardcoded)
│   └── check_snow(s, limit=1.2)
├── part_1_4::design_basic_wind_velocity
│   └── exposure_factor(z=10, TerrainCategory::II hardcoded)
│   └── peak_velocity_pressure(ρ=1.25)
│   └── wind_pressure(q_p, c_pe=0.8, c_pi=0.2 hardcoded) × structural_factor(c_s, c_d)
│   └── check_wind(w_p, limit=1.5)
├── part_1_5::check_temperature_action(delta_t_k, limit=50.0)
├── part_1_6::construction_load_kn_m2 → check_construction_load(q, limit=5.0)
├── inline impact CheckResult (impact_force_kn, limit=500 kN)
├── part_2::check_lm1_moment → lm1_design_tandem_kn, mid_span_moment_knm
├── part_3::design_vertical_wheel_load → check_crane_load(wheel, wheel×1.2)
└── part_4::janssen_horizontal_pressure_kpa → check_silo_pressure(p, limit=100 kPa)
```

**Not reached by `evaluate`:** `check_floor_actions` (`:77–88`, dead alternate entry); `part_1_5::check_fire_boundary_temperature`, `temperature_difference_action`; `part_1_7::check_accidental_pressure`, `explosion_pressure_kpa`; `part_2::check_imposed_bridge`; `part_3::crane_horizontal_force_kn`; `part_4::silo_wall_pressure_kpa` (surrogate), `tank_hydrostatic_pressure_kpa`.

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs window | `✏️editor/…/📥️inputs/🦀️.rs:19–21` | `render_document_json` — full snapshot as JSON; labels en/de (`:14`) |
| Results window | `✏️editor/…/📊️results/🦀️.rs:22–23` | `render_report` via shared app-surface |
| Inspection panel | `✏️editor/📌️panels/🔍️inspection/🦀️.rs:20` | Single check: clause, status, utilization, message (English keys) |
| Catalogue panel | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:3–4` | **Placeholder headline** |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs:30–32` | Table: Clause, Status, Utilization, Message (`🖥️app-surface/🦀️.rs:159–166`) |

No typed field editors, no per-part grouping, no remediation links, no de report columns.

### 1.5 Examples / assets

- **`retail-hydrocarbon-fire`:** DSL at `🖼️assets/…/🗣️.dsl.semio` (hydrocarbon fire, annex=en, mixed domains); label **not localized de** (`📚️examples/…/🦀️.rs:7`).
- **`demo-session`:** editor-only demo (`✏️editor/📚️examples/🎬️demo-session/`).
- **No** dedicated compliant vs non-compliant evaluate fixtures.

### 1.6 Tests

| Test file | Asserts numbers? | Notes |
|-----------|------------------|-------|
| `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | Yes | Snow 0.85, wind q_b≈0.39, categories, DE vs EN snow/bridge α_Q |
| `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | Yes | 11 checks, θ_g≈841.8°C @30 min, impact≈7.41 kN, **asserts `all_pass()` on default** `:30` |
| `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:34–45` | No | Clause family string presence only |
| 32× mutation scenario tests | No | Diff/apply/invariant; **no evaluate outcome** |
| `🧪️tests/🏋️mutate-en1991-1/` | Partial | Rust/Python mutation parity, not compliance |
| Example tests | No | `text.len() > 8`, inference determinism |

**No test asserts a failing check or remediation.**

---

## 2. Stub / fake detection

| Issue | Location | Evidence |
|-------|----------|----------|
| **Check never fails: imposed** | `🧬️schema/🦀️.rs:410–414` | Compares `q·ψ₀` (computed) vs `q` (limit); ψ₀≤1 ⇒ utilization≤1 always |
| **Check never fails: crane** | `💡️inferences/🦀️.rs:113–114` | `check_crane_load(wheel, wheel×1.2)` — limit always 20% above computed |
| Hardcoded snow roof μ=0.8 | `💡️inferences/🦀️.rs:99` | Not in subject; ignores pitch, multi-pitch, Annex B |
| Hardcoded wind z=10 m, terrain II | `💡️inferences/🦀️.rs:102` | `exposure_factor(10.0, TerrainCategory::II)` |
| Hardcoded c_pe=0.8, c_pi=0.2 | `💡️inferences/🦀️.rs:105` | Not editable; real values zone/building dependent |
| Hardcoded ρ=1.25 kg/m³ | `💡️inferences/🦀️.rs:103` | Fixed air density |
| Hardcoded check limits | `💡️inferences/🦀️.rs:100,106–107,109,111,116` | Snow 1.2, wind 1.5, thermal 50 K, construction 5, impact 500 kN, silo 100 kPa — **not norm citations** |
| Simplified impact | `🧬️schema/🦀️.rs:666–668` | `0.5·m·v²/1000`; not EN 1991-1-7 Annex B vehicle model |
| Surrogate silo pressure | `🧬️schema/🦀️.rs:800–803` | `silo_wall_pressure_kpa` linear `k·γ·h`; marked "Legacy…surrogate"; unused in evaluate |
| Simplified DE altitude snow | `🧬️schema/🦀️.rs:488–499` | Linear 0.1%/m above Δh; DIN NA uses zone-specific maps/formulae |
| DE snow zones 1–3 only | `🧬️schema/🦀️.rs:321–344`, `:475–481` | Missing DIN zones **1a, 2a, 3a** and associated s_k |
| Default material fallback | `🧬️schema/🦀️.rs:397` | Unknown material → 20 kN/m³ |
| Fire "check" | `🧬️schema/🦀️.rs:457–466` | Gas θ_g vs scalar `fire_member_capacity_c`; not REB/insulation/fire resistance class |
| DE bridge α_Q only divergence | `🧬️schema/🦀️.rs:709–715` | Single NA tweak; rest of LM1/2/3, fatigue, footways absent |
| `check_imposed` wrong clause | `🧬️schema/🦀️.rs:413` | `"Table 6.1", "q"` — vague; not a verification |
| Unused helpers | see §1.3 | Dead code paths |
| `check_floor_actions` orphan | `💡️inferences/🦀️.rs:77–88` | Never called from `evaluate` |
| Placeholder catalogue | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:3–4` | Explicit placeholder |
| Example label not de | `📚️examples/…/🦀️.rs:7` | Both locales English |
| Empty UI slots | 27× `📌️.empty.md` under editor/viewer | Config, presence, commands, transient |
| Default tuned to pass | `📸️snapshot/🦀️.rs:105–141` + test `:30` | No failing scenario in repo |
| Shared core: no remediation | `⚖️compliance/🦀️.rs:138–145` | `CheckResult { message: String }` only |

---

## 3. Complete subject definition (norm engineering target)

The artifact must become a **multi-part action model** tied to a real assessable subject (building, bridge, crane runway, silo/tank, or coupled project), not one scalar tuple.

### 3.1 EN 1991-1-1 (densities, imposed)

- **Site / use:** categories A–H with area per category; optional reduction α_A for large areas (Table 6.2).
- **Permanent:** layered build-ups `{material, γ_k [kN/m³], thickness [m]}` per structural element; water, fill, equipment.
- **Imposed:** q_k from Table 6.1; ψ₀, ψ₁, ψ₂ per NA; concentrated loads where governing.
- **Relationships:** floor/roof zones → areas; categories → ψ from `AnnexChoice`.

### 3.2 EN 1991-1-2

- Fire scenarios (standard/external/hydrocarbon), **t** [min], compartment, ventilation; link to member **fire resistance class** (REI/R) and protection — not a free-form °C capacity scalar.

### 3.3 EN 1991-1-3 + DIN EN NA

- **DE:** snow load zone map (1, **1a**, 2, **2a**, 3, **3a**), altitude **A** [m], s_k,0 from NA tables/formulae; **μ** shape coefficients (mono-/duo-pitch, etc.); **C_e**, **C_t**; drift at projections/parapets (§5.5); exceptional snow where required.
- **EN path:** user s_k,0; same shape/drift machinery.

### 3.4 EN 1991-1-4 + DIN EN NA

- **DE:** wind zone 1–4 → v_b,0; **terrain category** 0–IV; reference height **z** [m]; orography **c_o(z)**; **c_dir**, **c_season**; **c_pe**, **c_pi** per zone/façade/roof; **c_s·c_d** (or c_f·c_d); resulting **w** [kN/m²] on each envelope zone.

### 3.5 EN 1991-1-5

- Thermal actions: ΔT [K], gradient, fire boundary; α_T, E-modulus linkage to **downstream** EN 199x models — not ΔT vs fixed 50 K.

### 3.6 EN 1991-1-6

- Execution phases, equipment, stored materials; transient ψ; activity-specific q_k with cited NA tables.

### 3.7 EN 1991-1-7

- Accidental scenarios: vehicle impact (mass, speed, angle, barrier class), explosion (mass TNT equivalent, standoff), key element identification — compare to **design resistance** from structural model, not 500 kN constant.

### 3.8 EN 1991-2

- Bridge: span, carriageway, notional lanes, traffic type (LM1/LM2/…); **α_Q**, **α_q**, **ψ** per NA; UDL + tandem positions; **M_Ed**, **V_Ed** vs member resistances; fatigue where applicable.

### 3.9 EN 1991-3

- Crane class HC, hoist class, v_h; φ₁, φ₂; vertical/horizontal wheel loads, buffer forces, runway geometry.

### 3.10 EN 1991-4

- Silo: bulk properties, wall friction μ, k, hydraulic radius a, fill height; Janssen + patch loads; **tank:** fluid ρ, fill height, hydrostatic + sloshing; wall design pressure vs capacity.

**Cross-cutting:** `AnnexChoice`, design situations, action combinations (with EN 1990 artifact), localization en+de for every enumerated value.

---

## 4. Check catalogue (current `evaluate`)

| # | Part | Clause ID (as coded) | Verified | Required inputs (coded) | Limit source | Failure meaning |
|---|------|----------------------|----------|-------------------------|--------------|-----------------|
| 0 | 1-1 | Table 6.1 / q | q·ψ₀ vs q | area, category, annex | Self-referential | **Never fails** |
| 1 | 1-1 | Annex A / A.1 | g_k,mat vs assumed g_k | material, thickness, assumed | User assumed | assumed too low |
| 2 | 1-2 | §3.2 / 3.4 | θ_g(t) vs capacity °C | curve, t_min, capacity | Member scalar | θ_g exceeds capacity |
| 3 | 1-3 | §5 / 5.1 | s_roof vs 1.2 kN/m² | zone, altitude, en_s_k, annex, μ=0.8 | **Hardcoded 1.2** | snow > 1.2 |
| 4 | 1-4 | §5 / 5.1 | w_p vs 1.5 kN/m² | zones, c_s, c_d + fixed z, terrain, c_pe, c_pi | **Hardcoded 1.5** | wind > 1.5 |
| 5 | 1-5 | §6 / 6.1 | ΔT vs 50 K | delta_t_k | **Hardcoded 50** | ΔT > 50 |
| 6 | 1-6 | §4 / 4.1 | q_const vs 5 kN/m² | activity string | **Hardcoded 5** | construction > 5 |
| 7 | 1-7 | Annex B / B.2 | impact vs 500 kN | mass, speed | **Hardcoded 500** | impact > 500 |
| 8 | 2 | §4.3.2 / 4.4 | M_LM1 vs M_Rd | span, lane, width, resistance, annex | Resistance input | moment exceedance |
| 9 | 3 | §2 / 2.3 | wheel vs wheel×1.2 | crane/hoist class, speed | **Synthetic +20%** | **Never fails** |
| 10 | 4 | §5 / 5.1 | p_Janssen vs 100 kPa | silo geometry, bulk | **Hardcoded 100** | pressure > 100 |

**DE vs EN divergences implemented:** snow s_k zones + altitude (`🧬️schema/🦀️.rs:502–507`), wind v_b zones (`:584–594`), bridge α_Q lane 1 = 0.9 DE (`part_2:709–715`). **Missing:** snow 1a/2a/3a, full DIN altitude, wind orography, most LM1/NA tables, tanks, explosions.

---

## 5. Remediation strategy (per check)

| Check | Remediation target field(s) | How to compute target |
|-------|----------------------------|------------------------|
| Self-weight | `assumed_g_k_kn_m2` or layer thickness/material | `assumed_g_k ≥ Σ γ_k·t`; invert: `t ≥ (g_req−Σγt)/γ` |
| Fire gas temp | `fire_member_capacity_c` or protection | Increase capacity to `≥ θ_g(curve, fire_resistance_min)` or change curve/compartment |
| Snow | `snow_zone`, `snow_altitude_m`, roof μ in subject | Increase s_k,0 or reduce μ via geometry; cite DIN NA §4.1 + shape table |
| Wind | `wind_zone`, `en_v_b_m_s`, `c_s`, `c_d`, add c_pe/c_pi/z/terrain | Reduce w: lower zone, improve category, enlarge c_pi (internal) |
| Thermal | `delta_t_k` or insulation | `ΔT ≤ limit` from EN 1991-1-5 + project spec |
| Construction | `construction_activity` or duration | Select lower-activity phase or redistribute load |
| Impact | `accidental_mass_t`, `accidental_speed_km_h`, barriers | Reduce energy or add certified barrier per Annex B |
| Bridge moment | `bridge_span_m`, `bridge_moment_resistance_knm`, lane config | `M_Rd ≥ M_Ed(LM1)` → min section or max span inversion |
| Crane | runway/wheel capacity (new field) | `N_Rd ≥ φ·Q`; fix check to compare against capacity not wheel×1.2 |
| Silo | geometry, μ, k, bulk density | `p_w ≤ f_t` → min wall thickness / max fill per EN 1991-4 |
| Imposed (when fixed) | `category`, `area_m2` | Compare q_ed to slab/beam capacity from EN 199x — drop tautology |

All remediation rows need **`Remediation { field_path, target_value, unit, clause_ref, en, de }`** in Wave B core.

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| Pass/fail only | Status + utilization; no "how to comply" |
| English-only | Table headers, messages, inspection keys (`🖥️app-surface/🦀️.rs:159–166`, `:246–249`) |
| No subject linkage | Checks not tied to snapshot field paths |
| Inputs UX | JSON blob only (`📥️inputs/🦀️.rs:20`); 32 fields not individually editable in UI |
| No part grouping | All 11 checks in one flat list for unrelated domains |
| Catalogue | Placeholder (`📚️catalogue/🦀️.rs:3–4`) |
| Default always green | Masks failures (`compliance-report/🦀️.rs:30`) |
| Viewer columns | Debug-oriented (`Utilization`, raw `message`) |

---

## 7. Target design

### 7.1 Snapshot sketch (multi-root)

```rust
pub struct En1991Snapshot {
    pub annex: AnnexChoice,
    pub site: SiteActions { snow_zone, altitude_m, wind_zone, terrain, orography_factor, en_s_k, en_v_b },
    pub building: Option<BuildingActions {
        zones: Vec<LoadedZone { id, category, area_m2, layers: Vec<Layer>, imposed_q_k }>,
        roofs: Vec<RoofZone { pitch_deg, mu, c_e, c_t, drift_neighbors }>,
        envelope: Vec<WindFace { c_pe, c_pi, z_m, c_s, c_d }>,
    }>,
    pub fire: Option<FireScenario { curve, duration_min, members: Vec<MemberFire { id, rei_class }> }>,
    pub thermal: ThermalScenario { delta_t_k, … },
    pub construction: ConstructionPhase { activity, duration, loads },
    pub accidental: Vec<AccidentalCase { kind: Impact|Explosion, …, resistance_kn }>,
    pub bridge: Option<BridgeTraffic { span_m, lanes, width_m, resistances }>,
    pub crane: Option<CraneRunway { crane_class, hoist_class, v_h, capacity_kn }>,
    pub silo_tank: Option<SiloOrTank { kind, geometry, bulk, wall_capacity_kpa }>,
}
```

### 7.2 `evaluate()` structure

- Partition by **enabled** subject roots (building-only project skips bridge/crane checks).
- Each check: `{ clause, computed, limit, status, remediation: Option<Remediation> }`.
- Drop tautological checks; replace hardcoded limits with **norm-derived limits** or linked EN 199x resistances.
- Wire unused helpers or delete: explosion, tank hydrostatic, horizontal crane, fire boundary.

### 7.3 Example subjects

1. **Compliant:** DE office 240 m², zone 2 snow @150 m, wind zone 2, flat roof μ=0.8, REI 60 — all checks pass with cited clauses.
2. **Non-compliant:** Same but `assumed_g_k_kn_m2=4.0` (fail self-weight), `bridge_moment_resistance_knm=2000` (fail LM1), `fire_member_capacity_c=700` @ hydrocarbon 60 min (fail fire) — **≥3 failures** with remediation text.

### 7.4 Worked tests (derive expected values)

| Case | Derivation | Expected |
|------|------------|----------|
| Self-weight RC 0.2 m | 25×0.2=5.0 kN/m² vs assumed 6.0 | Pass, u=5/6 |
| θ_g standard 30 min | Eq. 3.4 → 841.796°C (`compliance-report/🦀️.rs:12–13`) | Pass vs 900 |
| DE snow zone 2 @150 m | s_k=0.85, no alt corr | s_roof=0.68 kN/m² vs 1.2 | Pass |
| LM1 DE lane 1, L=20 m | α_Q=0.9, tandem 270 kN → M≈2700 kNm (`compliance-report/🦀️.rs:26`) | Pass vs 3000 |
| Impact 30 t @ 80 km/h | `part_1_7:666–668` → 7.407 kN | Pass vs 500 |

Add **failure** cases with inverted inequalities.

### 7.5 Files to change (Wave B + C)

| Area | Paths |
|------|-------|
| Core remediation | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` |
| Subject schema | `🏋️en1991/…/🧬️schema/🦀️.rs`, `📸️snapshot/*`, all facet mirrors |
| evaluate | `…/💡️inferences/🦀️.rs` |
| Helpers | `…/🧬️schema/🦀️.rs` `part_*`, `na_de` |
| Mutations | new/update `🧬️mutations/*`, `🔮️oracles/🔣️.json` |
| Editor | `…/📥️inputs/*` structured forms; `📊️results/*` remediation UI |
| Viewer | `…/📊️report/*` localized columns |
| Examples | `📚️examples/*`, assets, de labels |
| Tests | `🧪️tests/⚖️compliance`, `💡️inferences/🧪️tests/🔬️compliance-report`, failure fixtures |
| Oracles | third-party or reference-tool entry for evaluate (currently **mutation-only**) |

---

## 8. Risks / open questions

1. **Scope:** One artifact for all EN 1991 parts vs split profiles (building / bridge / industrial)?
2. **Coupling EN 1990:** Action combinations and ψ live in EN 1990 — should EN 1991 checks emit **characteristic actions only** or full ULS pairs?
3. **Resistance fields:** Bridge moment and silo limits are stub capacities — integrate EN 1992/EN 1993/EN 1994 or keep norm-local simplified resistances?
4. **DIN NA fidelity:** Full snow zone map + altitude curves is non-trivial data — tabular asset vs formula?
5. **Wave B dependency:** Remediation/localization blocked on core `CheckResult` extension (coordination baseline).
6. **Oracle gap:** No external compliance reference; Wave D verify audit will need numeric baselines (hand calc sheets?).
7. **UI JSON editor:** Acceptable interim if structured editor is large — coordinator to confirm priority.
8. **Delete vs keep** `check_floor_actions` and surrogate helpers — avoid dual evaluate paths.

---

*Auditor: read-only pass, 2026-09-26. Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991`.*
