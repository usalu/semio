# Audit — DIN V 18599 (`⚡️din18599`)

**Executive summary.** The `din18599` artifact is a **13-scalar annual energy balance sketch** with a composed monthly climate table child — not a building subject. `evaluate()` runs 12 checks whose **clause IDs mis-map DIN V 18599 parts 2–10** (e.g. part 4 labelled “internal gains” is actually lighting; part 8 labelled “cooling” is actually DHW). Most checks compare unrelated energy terms against a single user-editable `annual_limit_kwh` bucket; **H′T (GEG)**, **reference-building comparison (GEG §48)**, zoning, envelope layers, plant systems, and lighting are absent. Hardcoded surrogates (`envelope_area_m2 = 3×floor`, `dhw = 900 kWh/person`, sinusoidal climate, tabular `q_P,tab` constants) stand in for norm calculations. Tests assert some intermediate numbers for the 100 m² reference helper but **never assert pass/fail compliance** or remediation. UI shows raw JSON inputs and English-only report rows with no “how to comply”. **Not feature-complete** for Wave C without a full subject schema rebuild and norm-aligned check catalogue.

Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599`.

---

## 1. Inventory

### 1.1 Snapshot / document fields

| Field | Type | Unit / domain | Notes |
|-------|------|---------------|-------|
| `use_class` | `UseClass` enum | — | `Residential`, `Office`, `School` only (`🦀️.rs:29-33`) |
| `heated_area_m2` | `f64` | m² | Treated as conditioned floor area (`📸️snapshot/🦀️.rs:24`) |
| `occupants` | `u32` | persons | DHW proxy only (`🧬️schema/🦀️.rs:367-368`) |
| `h_t` | `f64` | W/K | **Direct input** — not derived from envelope (`📸️snapshot/🦀️.rs:28`) |
| `h_v` | `f64` | W/K | **Direct input** — not derived from ventilation (`📸️snapshot/🦀️.rs:30`) |
| `climate` | composed `s.stdio.semio/table` child | θ_e °C, G_h W/m² per month | 12 rows × 2 columns (`🦀️.rs:80-87`) |
| `internal_gains_w_m2` | `f64` | W/m² | Specific internal gains (`📸️snapshot/🦀️.rs:36`) |
| `solar_gains_kwh` | `f64` | kWh/a | **Annual scalar** — not fenestration model (`📸️snapshot/🦀️.rs:37`) |
| `system_losses_kwh` | `f64` | kWh/a | Generation/distribution losses (`📸️snapshot/🦀️.rs:40`) |
| `renewable_kwh` | `f64` | kWh/a | On-site renewable credit (`📸️snapshot/🦀️.rs:41`) |
| `annual_limit_kwh` | `f64` | kWh/a | **Generic limit reused by 9 checks** (`📸️snapshot/🦀️.rs:42`) |
| `energy_carrier` | `String` | — | Lookup key for `f_p` (`🧬️schema/🦀️.rs:388-401`) |
| `reference_q_p_kwh` | `f64` | kWh/a | Reference-building primary energy target (`📸️snapshot/🦀️.rs:48`) |

`MonthlyClimate` wire type (`theta_e_c[12]`, `g_h_w_m2[12]`) at `🦀️.rs:42-45`. Default snapshot at `📸️snapshot/🦀️.rs:93-114` (100 m² residential, zone-2 climate child).

### 1.2 Composed children

- **`climate`** → `Din18599ClimateChild` (`store::ArtifactChild<SemioTableSnapshot>`), converters `din18599_climate_table_from_data` / `din18599_climate_data_from_table` (`🦀️.rs:80-110`), read via `din18599_climate(snapshot)` (`🦀️.rs:143-145`).

### 1.3 Mutations (13)

From `🧬️mutations/🦀️.rs:70-84`:

| Mutation kind | Target field |
|---------------|--------------|
| `change-use-class` | `use_class` |
| `change-heated-area-m2` | `heated_area_m2` |
| `change-occupants` | `occupants` |
| `change-ht` | `h_t` |
| `change-hv` | `h_v` |
| `change-internal-gains-wm2` | `internal_gains_w_m2` |
| `change-solar-gains-kwh` | `solar_gains_kwh` |
| `change-system-losses-kwh` | `system_losses_kwh` |
| `change-renewable-kwh` | `renewable_kwh` |
| `change-annual-limit-kwh` | `annual_limit_kwh` |
| `change-energy-carrier` | `energy_carrier` |
| `change-reference-qp-kwh` | `reference_q_p_kwh` |
| `update-climate` | composed `climate` child (payload `MonthlyClimate`) |

Each mutation has diff/inverse/fixture tests under `🧬️mutations/<slug>/🧪️tests/` — **mutation plumbing is mature**; subject modelling is not.

### 1.4 `evaluate()` call graph

```
evaluate(document)                          💡️inferences/🦀️.rs:95-106
  └─ balance_annual(inputs)                 💡️inferences/🦀️.rs:77-91
       ├─ part_1::check                     → aggregate_primary_energy_kwh, primary_energy_factor
       ├─ part_2::check                     → transmission_losses_kwh → heating_degree_hours, h_t
       ├─ part_3::check                     → ventilation_losses_kwh → h_v
       ├─ part_4::check                     → internal_gains_kwh
       ├─ part_5::check                     → solar_gains_kwh (passthrough field)
       ├─ part_6::check                     → system_losses_kwh (passthrough field)
       ├─ part_7::check                     → net_heating_demand_kwh
       ├─ part_8::check                     → cooling_demand_kwh → cooling_degree_hours
       ├─ part_9::check                     → dhw_demand_kwh → occupants × 900
       ├─ part_10::check                    → primary_energy_kwh → f_p, losses, renewable
       ├─ part_11::check                    → automation_factor(use_class)
       └─ part_12::check                    → tabular_primary_energy_kwh → reference_area_factor, tabular_specific_primary_energy_kwh_m2

Helpers NOT reached by evaluate():
  - check_primary_energy()                  🧬️schema/🦀️.rs:706-708 (duplicate of part_10)
  - from_building(), reference_residential() — test/demo builders only
  - transmission_loss_coefficient, ventilation_loss_coefficient — used only inside from_building
```

`Din18599Inference::infer` only computes `outline` field list (`💡️inferences/🦀️.rs:25-28`) — **compliance is not cached in inference schema**.

### 1.5 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs window | `✏️editor/.../📥️inputs/🦀️.rs:19-21` | **Pretty-printed JSON** of full snapshot — no field-level forms |
| Results window | `✏️editor/.../📊️results/🦀️.rs:22-23` | `render_report` → virtualized check list |
| Inspection panel | `✏️editor/.../🔍️inspection/🦀️.rs:19-20` | Single check: clause, status, u, message (English keys) |
| Document panel | `✏️editor/.../🗿️artifact/🦀️.rs` | One-line summary via `render_summary` |
| Catalogue panel | `✏️editor/.../📚️catalogue/🦀️.rs:3-5` | **Placeholder headline** only |
| Viewer report | `👁️viewer/.../📊️report/🦀️.rs:30-32` | Table: Clause, Status, Utilization, Message (`🖥️app-surface/🦀️.rs:159-166`) |

Window labels localized (`Results`/`Ergebnisse`, `Inputs`/`Eingaben`); **report content is not**.

Commands: `set-snapshot`, `evaluate`, `selected-check`, `set-active-example` (`✏️editor/🦀️.rs:41-46`).

### 1.6 Examples / assets / tests / oracles

- **Example:** `demo` — DSL at `🖼️assets/🎬️demo/🗣️.dsl.semio` (default 100 m² residential scalars + climate child ref).
- **Assets:** demo DSL only under `🖼️assets/`.
- **Compliance tests:** `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — 16 tests with **numeric assertions** on Q_T, Q_V, Q_P, Q_C, f_p table, part_1 aggregation; no pass/fail.
- **Report tests:** `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — only `checks.len() == 12` and part_1 clause family.
- **Mutation integration:** `🧪️tests/⚡️mutate-din18599-1/` (language-agnostic).
- **Oracle:** `🔮️oracles/🔣️.json` — mutation kind catalog only.
- **27 empty scaffold dirs** with `📌️.empty.md` under editor/viewer (config, presence, transient, actions, etc.).

---

## 2. Stub / fake detection

| # | Finding | Location | Impact |
|---|---------|----------|--------|
| S1 | **H_T, H_V stored as inputs** — not computed from envelope, bridges, or ventilation system | `📸️snapshot/🦀️.rs:28-30`; mutations `change-ht`, `change-hv` | User can set arbitrary W/K; no link to U-values or air flow |
| S2 | **`envelope_area_m2 = 3.0 × floor_area`** — cubic surrogate, not perimeter×height | `🧬️schema/🦀️.rs:267-268` | `from_building` H_T wrong for real geometry |
| S3 | **Hardcoded `psi_l = 0.10 × side × 4`**, ground floor `0.15 × area` | `🧬️schema/🦀️.rs:304-306` | Thermal bridges not modelled |
| S4 | **`MonthlyClimate::german_reference`** — cosine θ_e + fixed G_h array | `🦀️.rs:48-59`, `g_h` line 54 | Not TRY/WMO test reference years per -10 |
| S5 | **`internal_gains_w_m2: 3.5`**, **`system_losses: 8.0 × area`**, **`renewable: 15.0 × area`**, **`annual_limit: 75.0 × area`** in `from_building` | `🧬️schema/🦀️.rs:317-321` | Reference building is formula-filled |
| S6 | **`dhw_demand_kwh = occupants × 900`** | `🧬️schema/🦀️.rs:367-368` | Ignores DIN 18599-8 usage profiles, distribution losses |
| S7 | **`internal_gains` utilisation factor `0.35` hardcoded** | `🧬️schema/🦀️.rs:342` | Not from -2 Table / use class |
| S8 | **`cooling` gain factor `0.35` hardcoded** | `🧬️schema/🦀️.rs:363-364` | Not -3/-7 procedure |
| S9 | **`primary_energy_factor` string match table** — default `_ => 1.1` | `🧬️schema/🦀️.rs:388-401` | Incomplete vs -10 Table 12; no time/version |
| S10 | **`tabular_specific_primary_energy_kwh_m2`**: 100/95/85 constants | `🧬️schema/🦀️.rs:414-419` | Not GEG Anlage 2 / -12 tables by building type & year |
| S11 | **`automation_factor`**: 0.95/0.90/0.92 by use class only | `🧬️schema/🦀️.rs:379-384` | No TGA class, BACS factors from -11 |
| S12 | **`annual_limit_kwh` reused as limit** for Q_T, Q_V, Q_H, Q_C, Q_W, Q_sys, Q_P, Q_tab | `part_2`–`part_10`, `part_12` checks | Normatively meaningless; user can “pass” by raising limit |
| S13 | **Part 4 & 5 limits = `annual_limit × 2`** — gains checks inverted (higher is “better” but compared as utilization) | `🧬️schema/🦀️.rs:516-517`, `538-539` | Misleading pass semantics for gains |
| S14 | **Part 11 check can never fail** for current `UseClass` variants (all factors ≥ 0.85) | `🧬️schema/🦀️.rs:379-384`, `671-677` | Dead check unless enum extended below 0.85 |
| S15 | **Clause IDs mis-assign norm parts** (see §4) | `part_2`–`part_10` modules | False traceability |
| S16 | **`solar_gains_kwh` / `system_losses_kwh` passthrough** — `check` reads input, not climate/building | `🧬️schema/🦀️.rs:345-350`, `529-531`, `551-553` | Inputs ignored in “computation” |
| S17 | **No H′T check** (GEG §48 / DIN 4108-2) | entire family | Claimed in brief, absent in code |
| S18 | **`reference_q_p_kwh` free scalar** — not output of reference-building simulation | `📸️snapshot/🦀️.rs:48` | GEG reference comparison is manual |
| S19 | **`check_primary_energy` never called** | `🧬️schema/🦀️.rs:706-708` | Dead export |
| S20 | **Catalogue panel placeholder** | `✏️editor/.../📚️catalogue/🦀️.rs:3-5` | No clause browser |
| S21 | **Report columns English-only** | `🖥️app-surface/🦀️.rs:159-160` | Violates en+de report requirement |
| S22 | **`Din18599Outline::compute` ignores snapshot** — `entry_count = 0` always | `💡️inferences/🧾outline/🦀️.rs:24-28` | Inference stub |
| S23 | **θ_int heat `19.0` °C, cool `26.0` °C hardcoded** | `🧬️schema/🦀️.rs:332`, `362` | Not zone/use-class specific |

---

## 3. Complete subject definition (norm engineering target)

A feature-complete DIN V 18599 assessment artifact must model the **building (or building unit)** and **technical systems** such that all applicable procedures in parts 1–12 and GEG can run without free-floating result scalars.

### 3.1 Top-level

- **Project metadata:** calculation method (monthly/annual, detailed/tabular), norm edition, GEG application date, climate data set (-10).
- **Building:** `use_class` (full DIN taxonomy), `heated_volume_m3`, `heated_area_m2`, `reference_area_m2` (f_ref from -12), storeys, construction year, attachment (mid-terrace, etc.).
- **Climate zone** (`ClimateZoneDe` or location → TRY) — not only 12 monthly scalars but data provenance.
- **Zones** (-1): list of thermal zones with area, volume, design temperatures (heating/cooling), occupancy, internal gains (persons, equipment, lighting), setpoints, schedules.

### 3.2 Envelope (-2, GEG H′T)

Per opaque building element (wall, roof, floor, door, window):
- Area A [m²], orientation, inclination; layers (λ, d, ρ, c); U [W/(m²·K)] or computed; b-factor; shading.
- Linear thermal bridges: Ψ [W/(m·K)], length l [m].
- Ground contact: equivalent U×A.
- **Derived:** H_T [W/K], **H′T = H_T / A_heated** [W/(m²·K)] vs GEG limit curve by area.

### 3.3 Ventilation (-6 residential, -7 RLT non-residential)

- Ventilation concept (natural, hybrid, mechanical), min air flow per person/area (DIN 1946-6 / -16798), heat recovery η_hr, fan power, duct losses.
- **Derived:** H_V [W/K], auxiliary electricity.

### 3.4 Gains and losses (-2 balance)

- Solar: glazing g-value, frame factor, shading factor F_sh, orientation — **monthly Q_sol,m**.
- Internal: persons × W/person, equipment, **lighting (-4)** with time profile.
- Thermal mass / utilisation factors f_f,m per -2.

### 3.5 Systems

- **Heating (-5):** generators (η, standby), distribution (insulated pipes), emitters, control; final energy carriers per month.
- **Cooling / AC (-3, -7):** chiller COP, distribution, dehumidification.
- **DHW (-8):** storage volume, circulation losses, draw-off profile per occupants.
- **Multi-functional (-9):** CHP, heat pumps (COP vs source temp), solar thermal.
- **Renewables:** PV/thermal yields (not a single kWh credit).

### 3.6 Boundary conditions & weighting (-10)

- Primary energy factors f_p,i (carrier, year), CO₂ factors, electricity mix.
- User profiles, hot water, lighting operation.

### 3.7 Automation (-11)

- Building automation class, control factors f_BAC,n per subsystem.

### 3.8 Compliance outputs (GEG)

- **Reference building** per GEG Anlage 2 / -12: same geometry, standard U-values, standard systems → Q_P,ref.
- **Requirement:** Q_P ≤ Q_P,ref (and H′T ≤ limit, renewable quotas per GEG 2024).
- **Tabular method (-12):** q_P,tab vs limit for small residential.

Valid ranges: areas > 0; U > 0; η_hr ∈ [0,1]; occupants ≥ 1 for residential DHW; monthly climate θ_e ∈ [-30, 45] °C, G_h ≥ 0.

---

## 4. Check catalogue (as implemented today)

| Impl. module | Clause ID (as coded) | What is verified | Required inputs | Limit source | Failure meaning |
|--------------|---------------------|------------------|-----------------|--------------|-----------------|
| `part_1` | DIN V 18599-1 §6.1 | Σ Q_f,i·f_p,i vs `reference_q_p_kwh` | carriers, demands, f_p | User `reference_q_p_kwh` | Aggregated primary energy exceeds reference scalar |
| `part_2` | DIN V 18599-2 §8.1 | Q_T annual from H_T×HDH | `h_t`, climate | `annual_limit_kwh` | Transmission losses exceed arbitrary limit |
| `part_3` | DIN V 18599-3 §7.1 | Q_V from H_V×HDH | `h_v`, climate | `annual_limit_kwh` | Ventilation losses exceed arbitrary limit |
| `part_4` | DIN V 18599-4 §6.1 | Q_I from `internal_gains_w_m2` | area, gains | `2 × annual_limit_kwh` | Internal gains exceed 2×limit (nonsense) |
| `part_5` | DIN V 18599-5 §6.1 | `solar_gains_kwh` field | scalar input | `2 × annual_limit_kwh` | Solar gains exceed 2×limit |
| `part_6` | DIN V 18599-6 §8.1 | `system_losses_kwh` field | scalar input | `annual_limit_kwh` | System losses exceed limit |
| `part_7` | DIN V 18599-7 §9.1 | Q_H = Q_T+Q_V−Q_I−Q_S | all above | `annual_limit_kwh` | Net heat demand exceeds limit |
| `part_8` | DIN V 18599-8 §10.1 | Q_C from (H_T+H_V)×CDH×0.35 | `h_t`, `h_v`, climate | `annual_limit_kwh` | Cooling demand exceeds limit |
| `part_9` | DIN V 18599-9 §11.1 | 900×occupants | `occupants` | `annual_limit_kwh` | DHW exceeds limit |
| `part_10` | DIN V 18599-10 §12.1 | Q_P with f_p, losses, renewable | demands, carrier, losses | `annual_limit_kwh` | Primary energy exceeds limit |
| `part_11` | DIN V 18599-11 §7.1 | automation_factor ≥ 0.85 | `use_class` | min 0.85 | Factor below 0.85 (unreachable today) |
| `part_12` | DIN V 18599-12 §4.1 | q_tab×A_ref×f_auto | area, class | `annual_limit_kwh` | Tabular reference exceeds limit |

**Normative mapping error:** Real DIN V 18599-2 = monthly/annual **net energy balance**; -3 = **air conditioning**; -4 = **lighting**; -5 = **heating**; -6 = **residential ventilation**; -7 = **RLT**; -8 = **DHW**; -9 = **multi-functional generation**; -10 = **boundary conditions** (climate, f_p). Coded part numbers do not match standard titles.

**Missing GEG checks:** H′T ≤ H′T,limit(A); Q_P,actual ≤ Q_P,reference building; renewable share.

**Default demo (`📸️snapshot/🦀️.rs:93-114`) expected failures** (from compliance test numbers): Q_T≈11055, Q_H≈14793, Q_P≈19609, Q_tab≈9500 vs `annual_limit_kwh`=7500; part_1 Q_p≈20308 vs `reference_q_p_kwh`=10000.

---

## 5. Remediation strategy (per check — target behaviour)

| Check | Remediation the report should state | Subject field(s) | Target computation |
|-------|-------------------------------------|------------------|-------------------|
| H′T (missing) | “Reduce H_T from {computed} to ≤ {limit} W/K by improving element {id}: increase insulation of layer {n} from {d} mm to ≥ {d_req} mm (U→{U_req})” | envelope.layers[], bridges | Invert U-value sum; GEG Table Anlage 2 H′T limit vs A |
| Q_P vs reference (GEG) | “Lower Q_P from {Q_P} to ≤ {Q_P,ref} kWh/a: reduce H_T by {ΔH_T} W/K or switch carrier from {c} (f_p={f}) to {c′}” | systems, envelope | Q_P,ref from reference building run (-12 Anlage 2) |
| Q_T / Q_H | “Reduce transmission: add Δd_insulation to {element}” or “Reduce H_T to {H_T,max} W/K” | envelope | Q_T = H_T·Σ(θ_int−θ_e)+·hours; solve H_T |
| Q_V | “Increase η_hr to ≥ {η} or reduce air flow to {V_dot} m³/h” | ventilation | H_V = 0.34·V̇·(1−η_hr) |
| Q_I (lighting -4) | “Reduce lighting power density to ≤ {LPD} W/m² in zone {z}” | zones.lighting | Per -4 installed power × hours |
| Q_S | “Add external shading” / “Reduce g-value of glazing {w} to ≤ {g_req}” | windows | Monthly solar model -2 |
| Q_C | “Reduce cooling load: lower solar gains or improve envelope” | envelope, shading | -3/-7 monthly balance |
| Q_W | “Reduce DHW: lower storage losses or occupants profile” | dhw_system | -8 standard demand |
| System losses | “Insulate distribution of {system} to η≥{η_req}” | heating.distribution | -5/-9 loss model |
| f_p / carrier | “Replace {carrier} (f_p={f}) with {alt} (f_p={f_alt})” | energy_carriers[] | -10 Table 12 |
| Automation | “Upgrade to BACS class {class}” | automation.class | -11 factor table |
| Tabular (-12) | “For A={A} m², q_P must be ≤ {q_limit} kWh/(m²·a)” | use_class, area | GEG/DIN table lookup |

All remediation requires **`CheckResult` extensions** (Wave B): `remediation: LocalizedText[]`, `subject_ref: Path`, `computed`/`limit` with units.

---

## 6. Report & UX gaps

| Requirement | Status |
|-------------|--------|
| Per-clause pass/fail | Partial — 12 rows, but limits often meaningless |
| How to comply | **Absent** — `CheckResult.message` is English descriptor only (`⚖️compliance/🦀️.rs:138-145`) |
| Localized report (en+de) | **Absent** — columns hardcoded English (`🖥️app-surface/🦀️.rs:159-160`); inspection labels English (`🦀️.rs:246-249`) |
| Edit all subject fields | **No** — JSON dump only (`📥️inputs/🦀️.rs:19-21`); climate requires `update-climate` mutation, not table UI |
| Readable report | Partial — list/table of u values; no grouping by part, no units column, no computed vs limit columns separate |
| Reference vs actual | `reference_q_p_kwh` editable scalar, not computed reference building |
| H′T / envelope | Not exposed |

---

## 7. Target design

### 7.1 Snapshot schema (sketch)

```rust
pub struct Din18599Snapshot {
    pub meta: CalculationMeta,           // method, edition, climate_set_id
    pub building: BuildingShell,         // area, volume, use_class, zones[]
    pub envelope: EnvelopeModel,         // elements[], bridges[], h_t_derived
    pub ventilation: VentilationSystem,  // flows, eta_hr, h_v_derived
    pub gains: MonthlyGains,             // solar per glazing, internal per zone
    pub systems: TechnicalSystems,       // heat, cool, dhw, renewables[]
    pub carriers: Vec<FinalEnergy>,      // monthly Q_f per carrier
    pub reference: ReferenceBuilding,    // linked or embedded ref run
    pub climate: Din18599ClimateChild,
}
```

Remove free `h_t`, `h_v`, `solar_gains_kwh`, `annual_limit_kwh` as primary inputs; derive or store monthly balances.

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &Din18599Snapshot) -> CheckReport {
    let derived = derive_monthly_balance(doc);   // DIN 18599-2 core
    let mut r = CheckReport::default();
    r.push(check_ht_prime(doc, &derived));       // GEG
    r.push(check_primary_vs_reference(doc, &derived)); // GEG §48
    r.push(check_geg_renewable_share(doc, &derived));
    // Optional branches by method:
    if doc.meta.method == Tabular { r.push(check_tabular_12(doc)); }
    else { r.extend(check_systems_5_9(doc, &derived)); }
    r
}
```

Map modules to **real** part numbers; delete misnamed `part_4`–`part_10` gain/loss splits as compliance gates (keep as intermediate derived metrics only).

### 7.3 Example subjects

**Compliant (100 m² EFH, zone 2):** Envelope meets GEG Anlage 2 U-values, H′T=0.42 W/(m²·K) < limit 0.50; gas condensing η=0.95; Q_P,actual=8200 kWh/a < Q_P,ref=9500 kWh/a. Tests: assert H′T, Q_P utilizations ≤ 1.0 with worked HDH from TRY.

**Non-compliant (same building, thin insulation):** Wall U=0.45, H′T=0.62; Q_P=11200 kWh/a. Expect fail H′T with remediation “insulation thickness ≥ 140 mm”; fail Q_P with “reduce H_T to ≤ 78 W/K or improve η to ≥ 1.02”.

### 7.4 Worked numeric tests (add)

| Test | Derivation | Expected |
|------|------------|----------|
| H_V at 120 m³/h, η=0 | 0.34×120 | 40.8 W/K (existing `🧪️tests/⚖️compliance/🦀️.rs:27`) |
| Q_P multi-carrier | 10000×1.1+2000×1.8 | 14600 (line 116) |
| GEG H′T limit A=100 m² | interpolate table | assert limit W/(m²·K) |
| Reference building Q_P,ref | full -12 run | assert within 1% of published example |

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / compliance | `🧬️schema/🦀️.rs`, `📸️snapshot/🦀️.rs`, `🔺️diff/`, all `🧬️mutations/` |
| Inferences | `💡️inferences/🦀️.rs`, `🧾outline/🦀️.rs` |
| Crate root | `🦀️.rs` (types, climate, remove scalar shortcuts) |
| Editor | `📥️inputs/` (structured forms), new envelope/zone panels |
| Viewer | `📊️report/🦀️.rs` (localized columns, remediation rows) |
| Examples | `📚️examples/`, `🖼️assets/` — compliant + failing building |
| Tests | `🧪️tests/⚖️compliance/`, `🔬️compliance-report/`, new GEG oracle fixtures |
| Oracles | `🔮️oracles/` — compliance scenario catalog |
| Shared | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` (Wave B remediation model) |
| Cross-family | Reuse `din4108` envelope layers, `din16798` ventilation — already imported in `from_building` (`🧬️schema/🦀️.rs:249-250`) but not in snapshot |

---

## 8. Risks / open questions

1. **Scope:** Full DIN V 18599-1…-12 is a building physics suite — confirm Wave C target: GEG minimum (H′T + Q_P reference) vs full monthly balance for all building types.
2. **Part renumbering:** Breaking change to clause IDs in stored reports — migration or version field?
3. **`annual_limit_kwh`:** Deprecate entirely or repurpose as GEG absolute cap only for Q_P?
4. **Reference building:** Compute in-artifact vs link to second snapshot artifact?
5. **Climate child:** Keep composed table or inline monthly arrays for simpler editing?
6. **Tabular vs detailed:** Single artifact with `method` enum or separate subsets?
7. **GEG 2024 renewables / EH 55 / EH 70:** Which amending clauses are in scope?
8. **Coordination with `din4108`:** H′T limits live in GEG/DIN 4108 — single source of truth?
9. **Performance:** Monthly zone balance may need inference caching (`Din18599Inference` currently outline-only).
10. **Tests:** Compliance tests exist but green builds do not prove regulatory correctness — need external benchmark case (e.g. BMWSB example building).

---

*Auditor: read-only Wave A · 2026-09-26 · Family `⚡️din18599`*
