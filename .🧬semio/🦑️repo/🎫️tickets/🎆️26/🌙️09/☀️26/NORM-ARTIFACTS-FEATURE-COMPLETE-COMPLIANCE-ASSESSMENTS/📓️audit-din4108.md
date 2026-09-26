# DIN 4108 (`🧱️din4108`) — Per-Family Compliance Audit

**Executive summary.** The DIN 4108 artifact is the most mature thermal norm in the fleet: it has a 17-field snapshot, 22 semantic mutations with Python second-implementation oracles, pure `evaluate()` wiring parts 1–8 + 10 + Beiblatt 2, and numeric unit tests for individual formulae. It is **not feature-complete** against the coordination objective: the subject is a single opaque-wall scalar bag (not a building/envelope model), several checks use **surrogate physics and tabulated constants** (summer heat `0.04` factor, Glaser with zero exterior RH, two global μ values, wall-only `R_si`/`R_se`), **eight scope rows always pass**, `part_6::check_u_value` is dead code, `check_opaque_wall_with_bridges` hardcodes ten inputs, there is **no remediation or localized report text**, the editor exposes **raw JSON only**, and every end-to-end test uses the default snapshot that is **pre-tuned to pass all checks**. Wave C must restructure the subject, invert failures analytically, and add a committed non-compliant example before this family meets the ticket bar.

---

## 1. Inventory

### 1.1 Snapshot / document fields

Source: `…/🧬️schema/📸️snapshot/🦀️.rs` L15–53, `LayerDocument` at crate root `🦀️.rs` L28–33.

| Field | Type | Unit / enum | Default (`Default` L64–86) |
|-------|------|-------------|----------------------------|
| `category` | `String` | `residential` \| `office` \| `school` \| `industrial` (parsed L172–178) | `"residential"` |
| `layers` | `Vec<LayerDocument>` | each: `thickness_m` (m), `lambda_w_mk` (W/m·K) | brick 0.24/0.81 + EPS 0.14/0.035 |
| `climate` | `ClimateZoneDe` | `zone1`–`zone4` | `zone2` |
| `airtightness_n50` | `f64` | h⁻¹ | `2.5` |
| `psi_times_l_sum` | `f64` | W/K (building Σψ·l) | `0.02` |
| `rh_int` | `f64` | 0–1 | `0.5` |
| `catalog_id` | `String` | e.g. `AW-01` | `"AW-01"` |
| `material_id` | `String` | catalog key for part 4 | `"mineral_wool"` |
| `airtightness_class` | `String` | `class1`/`class2`/`class3` | `"class2"` |
| `t_int_c` | `f64` | °C | `20.0` |
| `solar_absorptance` | `f64` | 0–1 | `0.6` |
| `irradiance_w_m2` | `f64` | W/m² | `600.0` |
| `moisture_mu_exterior` | `f64` | μ‑value | `15.0` |
| `moisture_mu_interior` | `f64` | μ‑value | `1.3` |
| `envelope_area_m2` | `f64` | m² | `100.0` |
| `bb2_details_conform` | `bool` | Beiblatt 2 detail class | `true` |
| `application_type` | `String` | DIN 4108-10 code (`DEO`, …) | `"DEO"` |
| `declared_application_class` | `String` | `dm`/`dk`/`dg` | `"dk"` |

**Not modeled:** building element kind (wall/roof/floor/window), layer stack orientation (interior→exterior), per-layer material/μ, surface resistances per element type, room humidity class, ventilation, thermal bridges as geometry (ψ, l per detail), windows/doors, building-level aggregation.

### 1.2 Composed children

- `layers[]` → `LayerDocument { thickness_m, lambda_w_mk }` only.
- `ClimateZoneDe` → shared kernel `⚖️compliance/🦀️.rs` L405–442 (`design_external_temperature_c`, `summer_design_temperature_c`, `heating_degree_days`).
- Inference child `outline` (`💡️inferences/🧾outline/🦀️.rs`) — field list + `entry_count = layers.len()`; **not** used by compliance.

### 1.3 Mutation list (22)

`…/🧬️mutations/🦀️.rs` L61–84, `KINDS` L91–114: `change-category`, `change-climate`, `change-airtightness-n50`, `change-psi-times-l-sum`, `change-rh-int`, `change-catalog-id`, `change-material-id`, `change-airtightness-class`, `change-t-int-c`, `change-solar-absorptance`, `change-irradiance-wm2`, `change-moisture-mu-exterior`, `change-moisture-mu-interior`, `change-envelope-area-m2`, `change-bb2-details-conform`, `change-application-type`, `change-declared-application-class`, `insert-layer`, `remove-layer`, `reorder-layers`, `change-layer-thickness`, `change-layer-lambda`.

Oracle: `🔮️oracles/🔣️.json` — Python second implementation, 22 fixture vectors; **no third-party DIN 4108 compliance oracle**.

### 1.4 `evaluate()` call graph

Entry: `…/💡️inferences/🦀️.rs` `evaluate` L182–215 → `check_full_envelope` L125–169.

```
evaluate(snapshot)
  └─ check_full_envelope(...)
       ├─ part_1::scope_check × 8 (parts 1–8, OpaqueWall)     L146–148
       ├─ part_1::check_input_plausibility                      L149–150
       ├─ part_2::climate_adjusted_u_limit → limit              L151
       ├─ part_2::check_minimum_thermal_protection              L152
       ├─ moisture_layers_from_wall (μ assignment)              L153
       ├─ part_3::check_surface_temperature                   L155
       ├─ part_3::check_glaser_moisture                         L156
       ├─ part_4::design_lambda_for_material + check_design_lambda (last layer) L157–162
       ├─ part_5::check_summer_heat_protection                  L163
       ├─ part_6::check_u_value_with_bridges(limit from part 2) L164
       ├─ part_7::check_airtightness                            L165
       ├─ part_8::check_against_catalog (plain U, not bridged)  L166
       ├─ part_10::check_application_class                      L167
       └─ bb_2::check_beiblatt_2_equivalence                    L168
```

**Never reached from `evaluate`:** `part_6::check_u_value` (`🧬️schema/🦀️.rs` L668), `check_opaque_wall` / `check_opaque_wall_with_bridges` (latter **hardcodes** 10 envelope fields at L120), `part_1::applies_to_element` (only via scope), `Din4108Outline::compute`.

Helpers/constants: `R_SI_WALL_M2K_W=0.13`, `R_SE_WALL_M2K_W=0.04`, `F_RSI_MINIMUM=0.25` (`🧬️schema/🦀️.rs` L261–263).

### 1.5 Editor / viewer

| Surface | Path | Behaviour |
|---------|------|-----------|
| Inputs | `✏️editor/…/📥️inputs/🦀️.rs` L19–21 | Pretty-printed **JSON** of full snapshot via `render_document_json` — no per-field widgets |
| Results | `…/📊️results/🦀️.rs` L22–24 | `render_report(host.report())` — virtualized check list |
| Artifact panel | `📌️panels/🗿️artifact/🦀️.rs` L18–19 | One-line English summary: check count, worst u, all_pass |
| Inspection | `📌️panels/🔍️inspection/🦀️.rs` L19–20 | Single selected check; labels English (`Clause`, `Status`, …) |
| Catalogue | `📌️panels/📚️catalogue/🦀️.rs` L3–4, L21 | **Placeholder** headline only |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs` L30–32 | `TableWindowKit`: columns from `app_surface::report_table_columns()` — English headers L159–160 |

Commands: `setSnapshot`, `evaluate`, `setSelectedCheckIndex`, `setActiveExample` (`✏️editor/🦀️.rs` L41–46).

### 1.6 Examples / assets

- One catalogue example `demo`: `🖼️assets/🎬️demo/🗣️.dsl.semio` — identical to `Din4108Snapshot::default()`.
- `demo-session` under editor examples — workflow shell only.
- **No** second example demonstrating failures or remediation.

### 1.7 Tests

| Suite | Path | Asserts numbers? |
|-------|------|------------------|
| Compliance formulae | `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | **Yes** — U≈0.224, R≈4.466, f_Rsi, Magnus, μ-resistance, zone limits, bb2 ΔU |
| Compliance report | `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | **Weak** — `all_pass()` on default only; `checks.len() >= 15` |
| Editor host | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` L179–185 | Empty layers → `!all_pass()` (input error path) |
| Mutations | 22× fixture + Python oracle | Snapshot JSON equality, **not** check numerics |
| UI | `app_surface` shared | Table column alignment; placeholder when empty report |

Failure-case unit tests exist only for part 1 plausibility (U=12), part 10 class, bb2 flat-rate (L111, L121, L143 in compliance tests). **No** worked example asserting a specific failing U-limit or required insulation thickness.

---

## 2. Stub / fake detection

| Issue | Location | Evidence |
|-------|----------|----------|
| Hardcoded envelope in `check_opaque_wall_with_bridges` | `💡️inferences/🦀️.rs` L119–120 | Ignores caller except first 5 args; passes `0.5, 20.0, 0.6, 600.0, 15.0, 1.3, "mineral_wool", "AW-01", "class2", 100.0, true, "DEO", "dk"` |
| Summer heat **simplified** model | `🧬️schema/🦀️.rs` L636 | `solar_absorptance * irradiance_w_m2 * 0.04` — undocumented factor |
| Summer limit table | L639–642 | Four-point interpolation labelled `(DIN 4108-5 simplified)` |
| f_Rsi humidity correction | L526–527 | `f_thermal - 0.10 * (rh - 0.5).max(0.0)` — not DIN 4108-3 table 4 |
| Glaser exterior boundary | L478–493 | Vapor pressure linear to **0 Pa** at exterior; no `rh_ext`, no driving rain |
| μ per layer | `💡️inferences/🦀️.rs` L80–88 | Index 0 → `mu_exterior`, all others → `mu_interior` regardless of material |
| Wall-only surface resistances | `🧬️schema/🦀️.rs` L261–262 | Fixed `R_si=0.13`, `R_se=0.04` for all elements |
| Part 2 U base limits | L408–414 | Flat 0.28 / 0.50 — not tied to GEG requirement tables or element type |
| HDD climate factor | L421 | Four-point table, not official ZEB/ZWS data |
| Part 4 materials | L576–597 | 17-string lookup; `material_id` on document **not** bound to layer λ |
| Part 8 catalog | L737–746 | Eight fabricated `AW-*` entries; comment says "DIN 4108-8" but behaves as typology U reference |
| Part 8 check uses **unbridged** U | `💡️inferences/🦀️.rs` L149 vs L166 | `u` without ψ·l while part 6 uses bridged U |
| Scope checks never fail | L146–148 | `OpaqueWall` → all eight `scope_check` → `Pass` (L324–326) |
| `part_6::check_u_value` dead | `🧬️schema/🦀️.rs` L668–674 | Defined, never called from `evaluate` |
| Default parse fallbacks | `💡️inferences/🦀️.rs` L91–96, L99–115 | Unknown airtightness → Class2; unknown application → `Dad`/`Dm` |
| Catalogue panel placeholder | `📌️panels/📚️catalogue/🦀️.rs` L3–4 | Explicit placeholder |
| Report columns English-only | `🖥️app-surface/🦀️.rs` L159–160 | No DE strings |
| `CheckResult` no remediation | `⚖️compliance/🦀️.rs` L138–146 | `message: String` only |
| Empty layers → generic input clause | `💡️inferences/🦀️.rs` L204–213 | `ClauseId::new("DIN 4108", "input", "1")` — not per-field |
| Mutation `change-catalog-id` → `AW-07` | oracle L118 | **Not** in `CATALOG` L737 — evaluate would error if that snapshot evaluated |
| `📌️.empty.md` stubs | 27 paths under editor/viewer | Taxonomy placeholders (presence, config, transient, …) |

No `todo!()` in compliance path. Tests avoid `!is_empty()` alone for report quality but **`all_pass()` on tuned default** masks weak coverage.

---

## 3. Complete subject definition (engineering target)

The artifact should represent **one assessable building component or an aggregatable envelope slice** under DIN 4108 (plus GEG/U-value proof practice), not seventeen independent scalars.

### 3.1 Domain entities

```
Building (optional parent)
  └─ ThermalZone { category, t_int_design_c, rh_int | humidity_class, occupancy }
  └─ ClimateReference { zone_de | location, t_ext_winter_c, t_ext_summer_c, hdd }
  └─ EnvelopeElement {
        id, kind: Wall|Roof|Floor|BasementCeiling|Window|Door,
        orientation?, area_m2,
        layer_stack: Layer[],           // ordered interior → exterior
        r_si, r_se,                     // from DIN 4108-6 / tables by element & heat flow
        linear_bridges: ThermalBridge[], // { psi_w_mk, length_m, detail_ref }
        point_bridges?: …
     }
  └─ AirtightnessTest { n50_h-1, volume_m3, method, class_target }
  └─ InsulationProduct? { application_type, declared_class, lambda_d, mu, density, … }  // DIN 4108-10
  └─ CatalogReference? { din4108_detail_id, beiblatt2_conform }
```

### 3.2 Layer fields (each)

| Field | Unit | Valid range |
|-------|------|-------------|
| `material_id` | — | DIN 4108-4 table key |
| `thickness_m` | m | > 0 |
| `lambda_design_w_mk` | W/m·K | > 0, ≤ table max |
| `mu` | — | ≥ 1 |
| `rho_kg_m3` | kg/m³ | optional |
| `position` | enum | interior / exterior / structural |

### 3.3 Relationships & assessments

- **Part 2:** U ≤ U_limit(category, climate, element) — limits from current GEG annex / DIN 4108-2 tables per element.
- **Part 3:** f_Rsi ≥ f_Rsi,min(humidity class); Glaser or equivalent with exterior boundary conditions; optional mould criterion.
- **Part 4:** λ from declared moisture content / application conditions per material row.
- **Part 5:** Dynamic or normative summer model (not single `0.04·I` term); orientations, shading, mass.
- **Part 6:** U = (R_total)⁻¹; corrections ΔU for bridges, fasteners, air layers; proof documentation.
- **Part 7:** n50 vs class; leakage area; connection to ventilation concept.
- **Part 10:** Product application type vs declared class vs location in stack.
- **Beiblatt 2:** ΔU_WB vs detail conformity or measured ψ; not only Σψ·l / A.

### 3.4 Out of scope for single component artifact (but linked)

Whole-building primary energy (DIN V 18599), ventilation (DIN 1946), fire — separate artifacts.

---

## 4. Check catalogue

| Part | Clause / ref (as coded) | Verified | Required inputs | Limit source | Failure meaning |
|------|---------------------------|----------|-----------------|--------------|-----------------|
| 1 | §3 3.1 scope ×8 | Part applies to opaque wall | — | N/A (always Pass) | **Cannot fail** for wall |
| 1 | §3 3.1 plausibility | λ>0, R>0, U∈[0.1,5] | `layers`, computed U | Ad hoc band | Absurd U or empty layers |
| 2 | §4 4.1 | U ≤ climate-adjusted limit | `category`, `layers`, `climate` | 0.28×HDD factor (res/ind 0.50) | Under-insulated vs **simplified** limit |
| 3 | §6 6.1 | f_Rsi ≥ 0.25 | layers+μ, `t_int_c`, `climate`, `rh_int` | 0.25 + humidity fudge | Mould/condensation risk on inner surface |
| 3 | §7 7.1 | Glaser no condensation | same | margin ≥ 1 | Interstitial condensation |
| 4 | Table 1 λ | λ_insulation ≤ λ_design(material) | `material_id`, last layer λ | Tabulated × moisture factor | λ too high vs declared material |
| 5 | §4 4.1 | peak flux ≤ limit | `layers`, `climate`, `t_int_c`, `solar_absorptance`, `irradiance_w_m2` | **Simplified** W/m² table | Summer overheating (surrogate) |
| 6 | §5 5.2 | U' = U + ψ·l ≤ limit | `layers`, `psi_times_l_sum`, part 2 limit | Same as part 2 | Bridges push U over limit |
| 7 | §4 4.2 | n50 ≤ class limit | `airtightness_n50`, `airtightness_class` | 1.0 / 3.0 / 6.0 h⁻¹ | Too leaky |
| 8 | Table 1 `{id}` | U ≤ catalog typical | `catalog_id`, plain U | Hardcoded 8 entries | Worse than catalog reference |
| 10 | Table 1 1.1 | declared class ≥ min(type) | `application_type`, `declared_application_class` | dm/dk/dg matrix | Wrong product class for use |
| Bbl.2 | §5 5.1 | ΔU_WB,actual ≤ 0.05 or 0.10 | `psi_times_l_sum`, `envelope_area_m2`, `bb2_details_conform` | 0.05 conform / 0.10 flat | Thermal bridge surcharge too high |

**DE vs EN:** All checks use `AnnexChoice::De` only; DIN 4108 is national — no EN parallel in code.

---

## 5. Remediation strategy (per check)

Shared need: extend `CheckResult` (Wave B) with `remediation: LocalizedText`, `subject_ref: FieldPath`, `target_value: Quantity`.

| Check | Remediation computation | Target field(s) |
|-------|-------------------------|-----------------|
| Plausibility | If U>5: increase total R; message: `R_T ≥ {1/5.0}` | `layers[*].thickness_m` or λ |
| Part 2 min U | Invert U(R): find Δd such that U(R+Δd/λ_ins) ≤ limit → **"increase insulation layer {i} thickness from {d} mm to ≥ {d_req} mm"** | `layers[last_insulation].thickness_m` |
| f_Rsi | Increase R_ins or reorder; solve f_Rsi(R)=0.25 | thickness / layer order |
| Glaser | Add vapour barrier (lower μ), move insulation, or increase T at interface | per-layer `mu`, thickness |
| Part 4 λ | **"reduce declared λ of layer {i} to ≤ {λ_design} W/m·K or change material_id to {suggestion}"** | `layers[i].lambda_w_mk`, `material_id` |
| Part 5 summer | Lower α, increase insulation, or reduce `irradiance_w_m2` design value | `solar_absorptance`, thickness |
| Part 6 bridges | **"reduce ψ·l sum from {ψl} to ≤ {ψl_max} W/(m²·K)"** or thicken wall | `psi_times_l_sum` |
| Part 7 | **"achieve n50 ≤ {limit} h⁻¹ (currently {n50}); tighten envelope or target class {class}"** | construction QA (flag only) or `airtightness_n50` |
| Part 8 catalog | Thicken to match catalog U_typical | layers |
| Part 10 | **"declare class ≥ {min} (e.g. dg for DUK); currently {declared}"** | `declared_application_class` |
| Beiblatt 2 | **"reduce Σψ·l to ≤ {ψl_max} W/K"** or set `bb2_details_conform=true` with detail redesign | `psi_times_l_sum`, `bb2_details_conform` |

Analytic inversion for part 2 (current model): with U=1/(R_si+Σd/λ+R_se), solve for d_ins:  
`d_req = λ_ins · (1/U_limit - R_si - R_other - R_se)`.

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| Compliance vs non-compliance | Table shows `Pass`/`Fail`/`NotApplicable` — **no** "how to comply" |
| Localization | Panel tabs DE (`Ergebnisse`, `Inspektion`); report headers/messages **English**; `CheckResult.message` monolingual |
| Subject editing | JSON blob only — layers not editable as table; enums free-text strings |
| Catalogue | Placeholder — no browse of DIN 4108-4 materials or Beiblatt details |
| Inspection | Shows one check; no link from row → field editor |
| Default narrative | Summary line English: `"DIN 4108 — N checks, worst u=…"` (`app_surface` L228) |
| Fail visibility | Tuned default passes everything; user sees green path unless they hand-edit JSON |

---

## 7. Target design

### 7.1 Snapshot sketch

```rust
pub struct Din4108Snapshot {
    pub element: EnvelopeElement,  // kind, area_m2, r_si, r_se, layers[]
    pub zone: ThermalZone,         // category, t_int_c, humidity_class
    pub climate: ClimateZoneDe,
    pub airtightness: AirtightnessProof,
    pub thermal_bridges: ThermalBridgeSet, // list + bb2_conform flag
    pub product: Option<InsulationProductDeclaration>,
    pub catalog_ref: Option<String>,
}
pub struct LayerRow {
    pub material_id: String,
    pub thickness_m: f64,
    pub lambda_w_mk: f64,
    pub mu: f64,
}
```

### 7.2 `evaluate()` structure

1. Validate completeness → structured errors per field.
2. Compute R_T, U, U′, f_Rsi, Glaser profile, summer metric, n50, ΔU_WB.
3. Push **one row per normative requirement** (drop no-op scope spam or mark `NotApplicable` with reason).
4. Attach remediation via shared `RemediationBuilder` (Wave B).

### 7.3 Examples

| Example | Intent |
|---------|--------|
| `compliant-etics-wall` | ETICS wall zone 2, passes all checks with documented numbers |
| `failing-thin-insulation` | Same wall with 80 mm EPS → fails part 2 + 6; remediation cites ≥124 mm |

### 7.4 Worked test (non-compliant)

Wall: brick 0.24/0.81 + EPS 0.08/0.035, residential zone 2.  
R_T = 0.13 + 0.24/0.81 + 0.08/0.035 + 0.04 ≈ 2.85 m²K/W → U≈0.35 W/m²K.  
Limit ≈ 0.28 → **Fail** part 2. Required d_ins ≥ 0.035×(1/0.28−2.71) ≈ **0.124 m**.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / helpers | `🧬️schema/🦀️.rs`, `📸️snapshot/🦀️.rs`, `🔺️diff/`, all `🧬️mutations/**` |
| evaluate | `💡️inferences/🦀️.rs` |
| Core remediation | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` |
| Editor UI | `📥️inputs/`, new structured panels |
| Viewer | `📊️report/🦀️.rs` |
| Examples | `📚️examples/`, `🖼️assets/` |
| Tests | `🧪️tests/⚖️compliance/`, `🔬️compliance-report/`, new failing fixture |
| Oracles | `🔮️oracles/🔣️.json` + Python if snapshot shape changes |

---

## 8. Risks / open questions

1. **Normative scope:** Is the artifact one **component** or a **whole building envelope**? `psi_times_l_sum` + `envelope_area_m2` imply building-level aggregation while `layers` imply component — reconcile.
2. **GEG vs DIN 4108-2:** U limits in code (0.28) may not match current GEG requirement sets per element — legal source table needed.
3. **DIN 4108-8 naming:** Code catalog does not match published part 8 (insulation products / climate data) — rename or re-source.
4. **Layer ordering convention:** Must be fixed in schema (interior-first) and enforced in UI; μ assignment bug follows.
5. **Summer heat:** Accept surrogate for v1 or block on DIN 4108-5 / DIN 4108-6 Beiblatt 44 implementation?
6. **Wave B dependency:** Remediation/localization blocked on `CheckResult` extension — coordinate serially.
7. **Oracle gap:** No external DIN/Wärme tool — adversarial verify (Wave D) will rely on hand-derived worksheets.
8. **`AW-07` fixture:** Mutation test references unknown catalog id — evaluate path untested for that scenario.

---

*Audit date: 2026-09-26. Family path: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108`. Auditor: read-only Wave A.*
