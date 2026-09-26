# EN 1996 (`🪨️en1996`) — Feature-Complete Compliance Audit

## Executive summary

The EN 1996 artifact is a **22-scalar pilot wall slice**, not a masonry building/element subject. `evaluate()` (`💡️inferences/🦀️.rs:117–118`) runs eight hand-rolled ULS/durability checks with **no Φ/slenderness/eccentricity**, no `f_k = K·f_b^α·f_m^β` derivation, no concentrated loads, shear walls, lateral stability, or reinforced masonry. Several helpers are **surrogate tables** (fire thickness, exposure mortar, EN 1996-3 `Φ_s`) with wrong/vague clause IDs and hardcoded annex choices. Tests assert **check count and helper fragments**, not end-to-end pass/fail numbers for realistic subjects. UI shows English-only status strings in a four-column table; inputs are raw JSON; **no remediation**. Mutations/oracles are mature (22 kinds, Python second implementation); compliance is not. Rebuild needs a structured wall/building schema, full EN 1996-1-1/NA (+ optional 1-2, 2, 3/NA) check engine, localized remediation, structured editor, and numeric oracle-backed tests.

---

## 1. Inventory

### 1.1 Snapshot / document fields (`📸️snapshot/🦀️.rs:15–59`)

| Field | Type | Unit | Role in checks |
|-------|------|------|----------------|
| `m_ed_knm` | `f64` | kN·m | Flexure ULS (`part_1_1::check_flexure`) |
| `n_ed_kn` | `f64` | kN | Compression σ=N/A; sliding μN term; simplified N_Rd |
| `v_ed_kn` | `f64` | kN | Shear ULS |
| `h_ed_kn` | `f64` | kN | Sliding ULS |
| `z_mm3` | `f64` | mm³ | Flexural resistance lever arm section modulus |
| `area_mm2` | `f64` | mm² | Compression area; simplified N_Rd |
| `shear_area_mm2` | `f64` | mm² | Shear + sliding mortar contribution area |
| `f_k_mpa` | `f64` | MPa | **Direct** characteristic compressive strength (not derived) |
| `f_vk_mpa` | `f64` | MPa | Characteristic shear strength (direct) |
| `annex` | `AnnexChoice` | — | γ_M resolution (`annex_params`) |
| `masonry_class` | `MasonryClass` | — | γ_M for EN annex only; **ignored for DE** (flat 1.5) |
| `design_situation` | `DesignSituation` | — | Only sets `accidental` → γ_M=1.3 (DE) |
| `mu` | `f64` | — | Bed-joint friction (sliding) |
| `wall_thickness_mm` | `f64` | mm | Fire thickness check only |
| `fire_resistance_min` | `u32` | min | Fire R30–R240 class input |
| `unit` | `String` | — | Parsed to `MasonryUnit` (clay / calcium silicate / aac) |
| `exposure` | `ExposureClass` | — | MX1–MX5 durability (`part_2`) |
| `mortar` | `MortarClass` | — | EN 998-2 strength class |
| `bed_joint_thickness_mm` | `f64` | mm | Execution range 6–15 mm |
| `storeys` | `u32` | — | EN 1996-3 applicability (≤3) |
| `h_ef_mm` | `f64` | mm | Effective height (Φ_s, slenderness) |
| `t_ef_mm` | `f64` | mm | Effective thickness |

**Default values** (`📸️snapshot/🦀️.rs:74–97`): e.g. `n_ed_kn=200`, `f_k_mpa=5`, `annex=De`, `storeys=2`, `h_ef_mm=2500`, `t_ef_mm=240`.

**No composed children** — flat record; `En1996Outline::entry_count` is always `0` (`💡️inferences/🧾outline/🦀️.rs:50`).

### 1.2 Mutations (`🧬️mutations/🦀️.rs:55–108`)

22 `change-<field>` mutations, one per scalar. Each triad has `🦀️.rs`, `🔺️diff/`, `↩️inverse/` (inverse = restore pre-change value from base, not compliance solve). Catalog: `🔮️oracles/🔣️.json` (`en1996-1-any`, 22 fixture vectors). Python second implementation: `🧪️tests/🪨️mutate-en1996-1/🐍️.py`.

### 1.3 `evaluate()` call graph

```
evaluate(document)                          [💡️inferences/🦀️.rs:117]
  └─ check_full_masonry(document)           [💡️inferences/🦀️.rs:92]
       ├─ annex_params(document).gamma_m()  [💡️inferences/🦀️.rs:87–88 → 🧬️schema/🦀️.rs:346]
       ├─ part_1_1::design_strength_mpa, shear_design_strength_mpa
       ├─ sigma = n_ed_kn * 1000 / area_mm2
       ├─ part_1_1::flexural_resistance_knm, shear_resistance_kn, sliding_resistance_kn
       ├─ parse_masonry_unit(&unit)         [💡️inferences/🦀️.rs:75–80]
       ├─ part_1_1::check_flexure/compression/shear/sliding
       ├─ part_1_2::required_wall_thickness_mm + check_fire_wall
       ├─ part_2::check_exposure_mortar + check_bed_joint_thickness
       └─ part_3::phi_s + check_simplified_compression
```

**Not reached by `evaluate()`:** `check_masonry_wall` (`🧬️schema/🦀️.rs:529`) — dead helper used only in unit tests.

**Inference schema** (`💡️inferences/🦀️.rs:20–28`): only `outline` field; compliance is side function, not cached inference output.

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `✏️editor/…/📥️inputs/🦀️.rs:19` | `render_document_json` — pretty-printed JSON of all 22 fields; **not** field-level forms |
| Results | `✏️editor/…/📊️results/🦀️.rs:22` | `render_report` — virtualized list: clause, status, u, message |
| Inspection | `✏️editor/📌️panels/🔍️inspection/🦀️.rs:19` | Single selected check detail (English labels via `app_surface`) |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs:30` | `TableWindowKit` — same four columns as shared `report_table_columns` |
| Document panel | `app_surface::render_summary` | One-line: check count, worst u, all_pass |

Report columns (`🖥️app-surface/🦀️.rs:159–166`): `Clause`, `Status`, `Utilization`, `Message` — **English only**; `Status` is `Debug` of `CheckStatus` (`Pass`/`Fail`/`NotApplicable`).

Commands (`✏️editor/🦀️.rs:41–45`): `setSnapshot`, `evaluate`, `setSelectedCheckIndex`, `setActiveExample`. Action descriptions localized en+de; **check messages are not**.

### 1.5 Examples / assets

- One example: `loadbearing-wall` (`📚️examples/🧱️loadbearing-wall/🦀️.rs`).
- DSL: `🖼️assets/🧱️loadbearing-wall/…/🗣️.dsl.semio` — 4 storeys, `storeys=4` (simplified method N/A), MX3/CaSi/M10, R90.
- Example label: `LocalizedLabel::native("Loadbearing Wall", "Loadbearing Wall")` — **German not translated** (`📚️examples/🧱️loadbearing-wall/🦀️.rs:7`).

### 1.6 Tests (51 `🦀️.rs` under family)

| Area | Asserts numbers? | Notes |
|------|------------------|-------|
| `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | Partial | `phi_s`, `n_rd`, `gamma_m` EN vs DE; exposure/bed-joint status |
| `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | **No** | `report.checks.len() == 8` only (`:6`, `:12`) |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | **No** | `!report.checks.is_empty()` (`:174`, `:228`) |
| Mutation fixtures (22×) | Diff/json canonicality | No compliance values |
| `🧪️tests/🪨️mutate-en1996-1` | Mutation round-trip | No `evaluate()` oracle |

**No compliance oracle** in `🔮️oracles/🔣️.json` — only `en1996-1-mutate`.

### 1.7 Empty placeholders

27 `📌️.empty.md` under editor/viewer config, presence, commands, transient (viewer-heavy).

### 1.8 Language / codec wiring bug

`pilot_languages()` (`🦀️.rs:152–195`) includes **EN 1995** grammars/protocols (`semio_s_artifact_norm_en1995::…`) for all five language roles — likely copy-paste; EN 1996 has its own `snapshot/text`, `mutations/text`, etc.

---

## 2. Stub / fake detection

| # | Issue | Location | Evidence |
|---|-------|----------|----------|
| S1 | **f_k is a free input**, not `K·f_b^α·f_m^β` | `f_k_mpa` field; no `f_b`, `f_m`, `K`, `α`, `β` | Subject scalars bypass EN 1996-1-1 Table 3.1 / NA |
| S2 | **Compression without Φ, eccentricity, slenderness** | `check_full_masonry` `:96–103` | `sigma = N/A` only; no `Φ`, `e`, `λ`, `h_ef` for 1-1 |
| S3 | **Flexure uses compressive f_d** for `M_Rd = Z·f_xd` | `part_1_1::flexural_resistance_knm` `🧬️schema/🦀️.rs:364–366` | Should use flexural strength `f_xd1` (tabulated), not `f_k/γ_M` |
| S4 | **Surrogate fire table** | `part_1_2::required_wall_thickness_mm` `🧬️schema/🦀️.rs:413–427` | Comment says "simplified"; `_ => 90.0` fallback for unknown R; unit factors 1.0/1.1/1.25 |
| S5 | **Surrogate exposure/mortar matrix** | `part_2::required_mortar_strength_mpa` `🧬️schema/🦀️.rs:443–454` | Not traced to EN 1996-2 Annex B table; `f64::INFINITY` = inadmissible |
| S6 | **Surrogate EN 1996-3 Φ_s** | `part_3::phi_s` `🧬️schema/🦀️.rs:494–496` | `Φ_s = 0.85 − 0.0011·(h_ef/t_ef)²`; DIN EN 1996-3/NA may differ |
| S7 | **DE γ_M flat 1.5** — `masonry_class` ignored | `AnnexParams::gamma_m` `🧬️schema/🦀️.rs:346–352` | EN Class2→1.7 vs DE Class2→1.5 (tested `:68–78`); class field **misleading** for DE users |
| S8 | **Hardcoded annex in checks** | `check_fire_wall` `:431` → `AnnexChoice::De`; `part_2` `:468,480,482` → `En`; `check_masonry_wall` `:533` → `De` | Ignores `document.annex` |
| S9 | **Vague/wrong clause IDs** | `part_1_1::check_flexure` `:386` §6.2 "6.2" (flexure is 6.2.x); compression `:395` "6.1"; shear/sliding share "6.2" | Poor traceability |
| S10 | **`parse_masonry_unit` silent default** | `💡️inferences/🦀️.rs:75–80` | Any unknown string → `Clay` |
| S11 | **`En1996Outline::compute` ignores snapshot** | `🧾outline/🦀️.rs:47–51` | `_snapshot` unused; static field list |
| S12 | **Bed-joint check asymmetric** | `check_bed_joint_thickness` `🧬️schema/🦀️.rs:473–483` | Fail path uses `limit = 15 mm` even when **too thin** (3 mm); utilization `t/15` |
| S13 | **Inverse mutations ≠ compliance inversion** | e.g. `↩️inverse/🦀️.rs:9–10` | Restores old scalar; cannot answer "required N to pass" |
| S14 | **Tests that never fail meaningfully** | `compliance-report/🦀️.rs:6–12`; `masonry_wall_e2e` `⚖️compliance/🦀️.rs:15–17` | Count / `!is_empty()` only |
| S15 | **`check_masonry_wall` orphan** | `🧬️schema/🦀️.rs:529–535` | Never called from `evaluate()` |
| S16 | **Example sets `storeys=4`** | `🗣️.dsl.semio` line 2 | Guarantees check #8 `NotApplicable` — masks simplified-method failure path in demo |

**Default snapshot numeric outcome** (derived): γ_M=1.5, f_d=3.33 MPa → flexure/compression/sliding/fire/exposure/bed-joint/simplified **pass**; **shear fails** (V_Rd≈30 kN < V_Ed=35 kN, u≈1.17). No test asserts this.

---

## 3. Complete subject definition (engineering target)

The artifact must model the **masonry structural element(s) and execution context** the norm family governs, not pre-aggregated design resistances.

### 3.1 Domain graph

```
Project
 └─ Building / storeys[] (elevation, usage, fire compartment)
     └─ MasonryWall | MasonryPier | ShearWall | Lintel | Column
         ├─ Geometry: length, thickness t, height h, openings[], effective h_ef, t_ef
         ├─ Support/restraint: edge conditions, stiffening piers, floor diaphragms
         ├─ MasonryUnit: group (Group 1/2/4), material (clay CS AAC), format, f_b [MPa]
         ├─ Mortar: class, f_m [MPa], thin-layer / general-purpose, joint t_b [mm]
         ├─ Derived masonry: f_k per Eq. (3.1) + NA tables; f_vk; μ; E, α_t
         ├─ Reinforcement? (bars, bed-joint reinforcement, confinement) → EN 1996-1-1 §9
         ├─ Load cases[] → DesignSituation (persistent, transient, accidental, seismic)
         │    └─ Actions: N, M, V, H per combination (from EN 1990)
         ├─ Eccentricity e, first-order/second-order, Φ factor, slenderness λ
         └─ Fire: R requirement, insulation/load-bearing criteria
```

### 3.2 EN 1996-1-1 (general — primary for DE)

| Topic | Required inputs | Notes |
|-------|-----------------|-------|
| Characteristic strength | `f_b`, `f_m`, unit group, K, α, β → `f_k` | NA tables; not a single `f_k_mpa` knob |
| Partial factors | γ_M per class/situation; DE NA flat 1.5 (accidental 1.3) | Tie to `masonry_class` meaning per annex |
| Vertical walls compression | N_Ed, A, Φ(e,λ,h_ef,t_ef), f_d | §6.1.2–6.1.4; **Φ reduction mandatory** |
| Flexure / tension face | M_Ed, W or Z, f_xd1, γ_M | Distinct from compressive f_d |
| Shear | V_Ed, f_vk, γ_M, σ_n, geometry | §6.2.3; bed-joint / diagonal mechanisms |
| Sliding | H_Ed, μ, N_Ed, f_vd, A | §6.2.4 |
| Concentrated loads | Local stress under bearing plate | §6.1.5 |
| Lateral loading / flexural walls | M, V, slenderness, out-of-plane | §6.3 |
| Shear walls | In-plane V, aspect ratio, piers | §6.5 |
| Reinforced masonry | A_s, f_yd, bond, ductility | §9 |
| Serviceability | Crack width, deflection | §7 |

### 3.3 EN 1996-1-2 (fire)

Fire resistance period R; tabulated min thickness / cover from **material-specific** tables; insulation vs load-bearing criteria; not a single scalar `wall_thickness_mm` vs stepped lookup.

### 3.4 EN 1996-2 (design & execution)

Durability (MX classes), mortar/unit compatibility, joints, tolerances, curing, testing — broader than mortar≥lookup & 6≤t≤15.

### 3.5 EN 1996-3 / DIN EN 1996-3/NA (simplified — DE widely used)

Scope limits (≤3 storeys, λ≤27, etc.); **Φ_s** and N_Rd per NA; wall catalog geometries; differs from plugin's single quadratic.

### 3.6 Relationships to other norms

Actions from EN 1990/1991; seismic EN 1998; geotechnical EN 1997 for basement/retaining (explicitly removed from plugin `part_3` comment `🧬️schema/🦀️.rs:489`).

---

## 4. Check catalogue

| Part | Clause / equation | What is verified today | Required subject inputs | Limit source (DE-NA vs EN) | Failure meaning |
|------|-------------------|------------------------|-------------------------|----------------------------|-----------------|
| 1-1 | §6.1.2 σ≤f_d | σ=N_Ed/A vs f_k/γ_M | `n_ed_kn`, `area_mm2`, `f_k_mpa`, annex, class, situation | γ_M: DE **1.5** flat (`🦀️.rs:350`); EN class table (`🦀️.rs:37–44`) | Compression overload ( **no Φ** ) |
| 1-1 | §6.2 flexure | M_Ed vs Z·f_d | `m_ed_knm`, `z_mm3`, `f_k_mpa`, γ_M | Uses **compressive** f_d — wrong strength | Flexure overload (wrong f) |
| 1-1 | §6.2.3 shear | V_Ed vs A·f_vd | `v_ed_kn`, `shear_area_mm2`, `f_vk_mpa`, γ_M | f_vd=f_vk/γ_M | Shear overload |
| 1-1 | §6.2.4 sliding | H_Ed vs μN+A·f_vd | `h_ed_kn`, `mu`, `n_ed_kn`, `shear_area_mm2`, `f_vk_mpa` | μ user input | Sliding overload |
| 1-1 | §6.1.3 Φ, λ, e | **Missing** | h_ef, t_ef, e, stiffness | NA buckling curves | Would govern slender walls |
| 1-1 | Eq. (3.1) f_k | **Missing** | f_b, f_m, K, α, β | Table 3.1 + NA | Cannot verify material basis |
| 1-1 | §6.1.5 concentrated | **Missing** | bearing length, t, local A | — | — |
| 1-1 | §6.5 shear walls | **Missing** | in-plane geometry, piers | — | — |
| 1-1 | §9 reinforced | **Missing** | A_s, f_yd, details | — | — |
| 1-2 | §4 / Table 5.1 | t≥t_req(R, unit) | `wall_thickness_mm`, `fire_resistance_min`, `unit` | **Surrogate** table `🦀️.rs:414–421`; annex forced **DE** `:431` | Fire insufficient thickness |
| 2 | Annex B durability | mortar strength ≥ lookup | `exposure`, `unit`, `mortar` | **Custom** matrix `🦀️.rs:443–454`; annex **EN** `:468` | Inadmissible combo |
| 2 | §8 bed joint | 6≤t≤15 mm | `bed_joint_thickness_mm` | EN 6–15 mm GP mortar | Execution noncompliance |
| 3 | §4.2 Φ_s, N_Rd | N_Ed ≤ Φ_s·f_d·A if applicable | `n_ed_kn`, `storeys`, `h_ef_mm`, `t_ef_mm`, `area_mm2`, `f_k_mpa` | **Surrogate** Φ_s `🦀️.rs:494–496`; scope ≤3 storeys, λ≤27 | Simplified compression fail / N/A |
| 3 | DIN NA simplified | **Not differentiated** | wall type, NA coefficients | DE practice uses 3/NA | German path incomplete |

---

## 5. Remediation strategy (per current check)

Core `CheckResult` has **no remediation field** (`⚖️compliance/🦀️.rs:138–145`). Below: intended behavior after Wave B core work.

| Check | On failure, report should say | Target field(s) | Inversion sketch |
|-------|------------------------------|-----------------|------------------|
| Compression σ≤f_d | "Reduce N_Ed to ≤ **N_max = Φ·f_d·A**" or "Increase `f_k` to ≥ **f_k,req = γ_M·σ/Φ**" or "Increase `area_mm2` to ≥ **N_Ed·1000/(Φ·f_d)**" | `n_ed_kn`, `f_k_mpa`, `area_mm2` (+ future Φ) | Φ must be computed first |
| Flexure | "Reduce M_Ed to ≤ **M_Rd = Z·f_xd1/γ_M**" or increase Z / flexural strength | `m_ed_knm`, `z_mm3`, unit/group | Need `f_xd1` not `f_d` |
| Shear | "Reduce V_Ed to ≤ **V_Rd = A_v·f_vk/γ_M**" or raise `f_vk_mpa` | `v_ed_kn`, `f_vk_mpa`, `shear_area_mm2` | `v_ed_kn ≤ shear_area_mm2 * f_vk_mpa / (1000·γ_M)` |
| Sliding | "Reduce H_Ed to ≤ **μ·N_Ed + A·f_vd**" or increase `mu` / `n_ed_kn` | `h_ed_kn`, `mu`, `n_ed_kn` | Linear in H_Ed |
| Fire | "Increase `wall_thickness_mm` to ≥ **t_req(R, unit)** mm" | `wall_thickness_mm` | Table lookup inverse |
| Exposure/mortar | "Upgrade mortar to ≥ **M_class**" or change unit/exposure | `mortar`, `exposure`, `unit` | Discrete class picker |
| Bed joint | "Set `bed_joint_thickness_mm` to **[6, 15]** mm (current **t** mm)" | `bed_joint_thickness_mm` | Clamp to range |
| Simplified N | "Reduce storeys to ≤3" / "Reduce slenderness to ≤27" / "Reduce N_Ed to ≤ **Φ_s·f_d·A**" | `storeys`, `h_ef_mm`, `t_ef_mm`, `n_ed_kn` | Piecewise |

Wire remediation via `CheckResult.remediation: Vec<RemediationHint { field_path, locale, action, target_value }>` and connect to `change-*` mutations / inverse solvers.

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| Pass/fail only | Status + utilization; no "how to comply" |
| Localization | Window titles en/de; **messages English**; `Status` debug string; example label not German |
| Inputs | Raw JSON (`📥️inputs/🦀️.rs:19`); no validation ranges, units, or engineering labels |
| Results | No grouping by part (1-1 / 1-2 / 2 / 3); no computed/limit quantity columns in table (only in `CheckResult`, not shown) |
| Inspection | Shows clause/status/u/message; no link from failure → input field |
| `masonry_class` | Editable but **ineffective** for DE — UX bug |
| `f_k`, `f_vk` | User can type strengths without material traceability |
| Export | `report:out` JSON has full `CheckResult` but no remediation |

---

## 7. Target design

### 7.1 Snapshot sketch (Rust-ish)

```rust
pub struct En1996Snapshot {
    pub meta: ProjectMeta,                    // name, annex, design_code
    pub storeys: Vec<Storey>,                 // id, elevation_m, count
    pub walls: Vec<MasonryWall>,              // primary assessables
    pub load_cases: Vec<LoadCase>,            // EN 1990 refs, N,M,V,H per wall
    pub combinations: Vec<DesignCombination>, // situation → governing actions
}

pub struct MasonryWall {
    pub id: WallId,
    pub geometry: WallGeometry,               // L, t, h, h_ef, t_ef, openings
    pub unit: MasonryUnitSpec,                // group, material, f_b_mpa
    pub mortar: MortarSpec,                   // class, f_m_mpa, t_b_mm
    pub derived: DerivedMasonry,              // f_k, f_vk, mu, E (computed, cached)
    pub restraint: RestraintClass,
    pub reinforcement: Option<ReinfLayout>,
    pub fire: FireRequirement,                // R_min, criteria
    pub exposure: ExposureClass,
}
```

Keep `evaluate(wall_id, combination)` or whole-building with per-element `CheckReport` sections.

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1996Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for wall in &doc.walls {
        let actions = resolve_actions(wall.id, doc);
        let params = annex_params(&doc.meta);
        derive_material(&wall.unit, &wall.mortar, &doc.meta.annex); // f_k Eq. 3.1
        report.extend(part_1_1::vertical_compression_with_phi(wall, actions, params));
        report.extend(part_1_1::flexure(wall, actions, params));
        report.extend(part_1_1::shear_and_sliding(wall, actions, params));
        // optional: concentrated, shear wall, reinforced
        report.extend(part_1_2::fire(wall, &doc.meta.annex));
        report.extend(part_2::durability_and_execution(wall));
        if part_3::is_applicable(&doc.storeys, &wall.geometry) {
            report.extend(part_3::simplified_compression(wall, actions, params, &doc.meta.annex));
        }
    }
    report
}
```

### 7.3 Example subjects

**A — Compliant DE single-storey wall (simplified path)**  
Clay Group 1, f_b=20 MPa, M10, t=365 mm, h_ef=2.8 m, N_Ed=180 kN, V_Ed=12 kN, storeys=1, annex=DE.  
Expect: f_k≈10.2 MPa (typical K,α,β), γ_M=1.5, Φ_s≈0.77, all Pass.

**B — Non-compliant multi-failure**  
Same wall, N_Ed=600 kN (compression fail), V_Ed=80 kN (shear fail), t=90 mm with R90 (fire fail), MX4 + AAC (durability fail), t_b=4 mm (joint fail), storeys=4 (simplified N/A).  
Expect: ≥5 Fail + explicit remediation per failure.

### 7.4 Worked test (default snapshot shear)

```
γ_M = 1.5 (DE persistent)
f_vd = 0.15 / 1.5 = 0.10 MPa
V_Rd = 300_000 × 0.10 / 1000 = 30.0 kN
V_Ed = 35.0 kN → u = 35/30 ≈ 1.17 → Fail
Remediation: V_Ed ≤ 30 kN OR f_vk ≥ 0.175 MPa OR shear_area ≥ 350_000 mm²
```

Assert in `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / snapshot | `🧬️schema/📸️snapshot/🦀️.rs`, `🔺️diff/`, `🧬️mutations/**` (regenerate) |
| Compliance helpers | `🧬️schema/🦀️.rs` (`part_1_1`, `part_1_2`, `part_2`, `part_3`, `na_de`) |
| evaluate | `🧬️schema/💡️inferences/🦀️.rs` |
| Core remediation | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` |
| Editor inputs | `✏️editor/…/📥️inputs/🦀️.rs` + new structured windows |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs` |
| Examples | `📚️examples/**`, `🖼️assets/**` |
| Tests | `🧪️tests/⚖️compliance/`, `💡️inferences/🧪️tests/`, new `🔮️oracles` compliance vectors |
| Grammars | `🦀️.rs` `pilot_languages` — replace en1995 refs with en1996 facets |
| Crate root entities | `🦀️.rs` (`MasonryUnit`, `MortarClass`, …) |

---

## 8. Risks / open questions

1. **Scope boundary:** Is the artifact one wall, one storey, or whole building? Coordinator should fix granularity before schema migration.
2. **DE path priority:** DIN EN 1996-3/NA simplified vs full 1-1 Φ method — which is default for German users?
3. **Reinforced masonry:** In scope for v1 or explicit exclusion with `NotApplicable`?
4. **f_k derivation:** Store `f_b`/`f_m` and compute, or accept `f_k` with provenance metadata?
5. **EN 1995 grammar reuse:** Intentional shared codec or bug? Breaking change if fixed.
6. **Compliance oracle:** No third-party EN 1996 library — need handbook worked examples + NA PDFs as fixtures.
7. **`masonry_class` on DE:** Hide, repurpose, or implement NA-specific meaning?
8. **Wave B dependency:** Remediation/localization blocked on `CheckResult` extension.
9. **Performance:** Whole-building evaluate with many walls — caching in inference schema?

---

*Audit: read-only, 2026-09-26. Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996`.*
