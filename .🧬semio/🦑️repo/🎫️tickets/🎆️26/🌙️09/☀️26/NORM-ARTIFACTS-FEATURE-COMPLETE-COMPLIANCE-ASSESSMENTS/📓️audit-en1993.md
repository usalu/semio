# Audit — EN 1993 (`🔩️en1993`)

## Executive summary

The EN 1993 artifact is a **flat 74-scalar input sheet** that runs **25 utilization checks** across 16 norm parts via `evaluate()` → `check_full_steel_member()`. Helper math is real and partially tested, but the **subject is not a structure** (no members, sections, load cases, joints, or FEM model), **χ is user-supplied** (no buckling-curve pipeline), **M+N+V interaction / LTB / SLS are absent**, **annex choice is ignored for the first three checks**, several fields are **decoupled from the checks that claim them** (e.g. `shell_sigma_x_ed_mpa` vs Janssen-computed silo stress), fire protection uses a **simplified surrogate**, and the report has **no remediation, no localized messages, and JSON-only editing**. Feature-complete compliance assessment is **not met**; Wave C must replace the flat snapshot with a hierarchical steel-work subject and wire analytic inversion into `CheckResult`.

---

## 1. Inventory

### 1.1 Snapshot / document fields (74 scalars + annex)

Source: `…/🧬️schema/📸️snapshot/🦀️.rs:14–208`, defaults `219–297`.

| Group | Fields (unit) | EN part |
|-------|---------------|---------|
| Identity | `annex` (en/de) | all |
| Base member | `n_ed_kn`, `m_ed_knm`, `v_ed_kn` (kN/kNm), `a_mm2`, `a_v_mm2`, `w_pl_mm3`, `f_y_mpa`, `f_u_mpa`, `chi`, `a_net_mm2`, `tension_n_ed_kn` | 1-1 |
| Fire | `fire_thickness_mm` (mm), `fire_rating` (str), `fire_massivity`, `fire_mu_0`, `fire_design_temperature_c` (°C) | 1-2 |
| Cold-formed | `cf_b_bar_mm`, `cf_t_mm` (mm), `cf_k_sigma`, `cf_psi`, `cf_n_ed_kn`, `cf_gross_resistance_kn` | 1-3 |
| Stainless | `stainless_m_ed_knm`, `stainless_w_pl_mm3`, `stainless_f_y_mpa` | 1-4 |
| Plated | `plated_lambda_p`, `plated_sigma_ed_mpa` (MPa) | 1-5 |
| Silo/shell | `silo_t_mm`, `silo_r_mm` (mm), `shell_sigma_x_ed_mpa` (MPa), `silo_k`, `silo_gamma_kn_m3`, `silo_depth_m` (m) | 1-6 + 4-1 |
| Bolts | `bolt_f_ed_kn`…`bolt_f_ub_mpa` (11 fields) | 1-8 |
| Welds | `weld_a_mm`, `weld_l_mm`, `weld_f_u_mpa`, `weld_steel_grade`, `weld_f_ed_kn` | 1-8 |
| Fatigue | `delta_sigma_mpa`, `fatigue_category` (u8), `fatigue_method` (str) | 1-9 (+ 2-9) |
| Toughness | `t10_steel_subgrade`, `t10_actual_thickness_mm`, `t10_t_ed_c` | 1-10 |
| Tension component | `tension_component_f_uk_kn`, `tension_component_f_k_kn`, `tension_component_n_ed_kn` | 1-11 |
| HSS | `hss_w_el_mm3`, `hss_f_y_mpa`, `hss_section_class`, `hss_m_ed_knm` | 1-12 |
| Bridge | `bridge_lambda`, `bridge_phi_2`, `bridge_delta_sigma_p_mpa` | 2 |
| Tower | `tower_wind_factor`, `tower_n_ed_kn` | 3-1 |
| Pile | `pile_sigma_mpa`, `pile_k_red`, `pile_n_ed_kn` | 5 |
| Crane | `crane_f_z_ed_kn`, `crane_wheel_contact_length_mm`, `crane_dispersion_mm`, `crane_t_w_mm` | 6 |

No composed children, no id-keyed collections, no geometry, no section catalogue, no load-case tree.

### 1.2 Mutations (17)

`…/🧬️mutations/🦀️.rs:56–99`: `change-annex`, `update-member-properties`, `update-fire-inputs`, `update-cold-formed-inputs`, `update-stainless-inputs`, `update-plated-inputs`, `update-silo-shell-inputs`, `update-bolt-inputs`, `update-weld-inputs`, `update-fatigue-inputs`, `update-through-thickness-inputs`, `update-tension-component-inputs`, `update-hss-inputs`, `update-bridge-inputs`, `update-tower-inputs`, `update-pile-inputs`, `update-crane-inputs`. Each has diff, inverse, fixture, and unit test.

### 1.3 `evaluate()` call graph

`…/💡️inferences/🦀️.rs:202–204` → `check_full_steel_member` (`94–198`):

```
check_steel_member (schema/🦀️.rs:1159) → part_1_1::{axial,buckling,bending}
part_1_1::check_shear, check_net_tension
part_1_2::check_fire_protection, check_critical_temperature
part_1_3::check_cold_formed_effective_section (+ lambda_p, reduction_factor)
part_1_4::check_stainless_steel
part_1_5::check_plated_buckling
part_1_6::check_shell_buckling (+ sigma_x_rcr, lambda_bar, alpha, chi)
part_1_8::check_bolt_shear, check_bolt_bearing, check_fillet_weld
part_1_9::check_fatigue_range
part_1_10::check_through_thickness
part_1_11::check_tension_component
part_1_12::check_high_strength_bending
part_2::check_steel_bridge, check_bridge_fatigue
part_3::check_tower_buckling
part_4::check_silo_wall (+ janssen_pressure, membrane_hoop_stress)
part_5::check_pile_driving_stress, check_pile_foundation_steel
part_6::check_crane_runway_web
```

**Not reached by `evaluate()`** (helpers exist, tests only): `part_1_1::{flange_class, web_class, section_class, chs_class, lambda_bar, chi}` (`schema/🦀️.rs:525–623`), `check_steel_member_from_fem` (`1196`, `#[cfg(feature = "cross-fem")]`).

`En1993Inference` (`💡️inferences/🦀️.rs:20–28`) only computes `outline` — not compliance.

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `✏️editor/…/📥️inputs/🦀️.rs:19–21` | Pretty-printed JSON of full snapshot via `render_document_json` |
| Results | `…/📊️results/🦀️.rs:22–24` | One row per check: clause, `Debug` status, utilization, English `message` |
| Inspection | `…/📌️panels/🔍️inspection/🦀️.rs:19–21` | Clause, status, utilization, message for selected index |
| Document panel | `…/📌️panels/🗿️artifact/🦀️.rs` | Summary: check count, worst u, all-pass |
| Catalogue | `…/📌️panels/📚️catalogue/🦀️.rs:3–5` | Placeholder headline only |
| Viewer | `👁️viewer/🦀️.rs:76–80` | Read-only report window |

Commands (`✏️editor/🦀️.rs:41–46`): `setSnapshot`, `evaluate`, `selected-check`, `setActiveExample`. Manifest labels localized en/de; check messages are not.

### 1.5 Examples / assets / tests / oracles

- **Example**: `📚️examples/🔩️high-strength-connection/` — one DSL fixture; German label not translated (`🦀️.rs:7`: `"High Strength Connection"` for both locales).
- **Assets**: `🖼️assets/🔩️high-strength-connection/…/🗣️.dsl.semio` (non-compliant heavy-load scenario, annex=en).
- **Compliance unit tests**: `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — 20+ tests on helpers with numeric assertions (axial S355, M20 bolt bearing 123.6 kN, shell σ_x,Rcr=338.8 MPa, etc.).
- **Evaluate tests**: `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:4–17` — only `checks.len()==25` and family-name presence; **no pass/fail or utilization assertions**.
- **Mutation E2E**: `🧪️tests/🔩️mutate-en1993-1/` + Python second implementation (`🔮️oracles/🔣️.json`).
- **Empty placeholders**: 27× `📌️.empty.md` under editor/viewer window subtrees (config, presence, transient, actions).

---

## 2. Stub / fake detection

| Issue | Location | Evidence |
|-------|----------|----------|
| **χ supplied, not computed** | `snapshot/🦀️.rs:40`, `evaluate` uses `document.chi` | No `L`, `N_cr`, curve selection, or `lambda_bar` → `chi` pipeline in evaluate |
| **Annex ignored (first 3 checks)** | `schema/🦀️.rs:1159–1168` | `check_steel_member` hardcodes `AnnexChoice::De` / `AnnexParams::de()`; `check_full_steel_member` passes `document.annex` to later checks only |
| **Fire board thickness surrogate** | `schema/🦀️.rs:676–684` | Comment `(simplified)`; linear base×massivity factor, not EN 1993-1-2 Table 4.3 section-factor method |
| **Fillet weld simplified method** | `schema/🦀️.rs:881` | Comment `simplified method`; directional/stress components omitted |
| **Tower wind = scalar divisor** | `schema/🦀️.rs:1082–1083` | `tower_buckling_kn = N_b,Rd / wind_factor` — not EN 1993-3-1 gust/loading model |
| **Pile `k_red` user scalar** | `snapshot/🦀️.rs:192`, `part_5/1117` | No soil/geometry; reduction factor is input not derived |
| **Fatigue category fallback** | `schema/🦀️.rs:923` | Unknown category → `71.0` MPa silently |
| **Weld grade default** | `schema/🦀️.rs:877` | Unknown `steel_grade` → `β_w=0.9` |
| **Subgrade fallback** | `schema/🦀️.rs:989` | Unknown subgrade → index `2.0` (J0) |
| **`shell_sigma_x_ed_mpa` decoupled from silo check** | `inferences/🦀️.rs:130–135, 181–183` | Part 1-6 uses user `shell_sigma_x_ed_mpa`; Part 4 computes `silo_sigma_ed` from Janssen — two independent stress paths, same silo geometry |
| **HSS `hss_section_class` user-supplied** | `snapshot/🦀️.rs:174` | Class not computed from geometry; class 4 → `m_rd=0` can never pass (`schema/🦀️.rs:1032–1034`) |
| **`plated_lambda_p` user-supplied** | `snapshot/🦀️.rs:166` | No plate geometry → slenderness not derived |
| **Bridge interaction oversimplified** | `schema/🦀️.rs:1055–1057` | `η = |N/N_Rd| + |M/M_Rd|` — not EN 1993-2 interaction expressions |
| **Checks that can never fail (defaults)** | `snapshot/🦀️.rs:219–296` | Default snapshot yields all passes (editor test `✏️editor/🧪️tests/🔬️unit/🦀️.rs:173` only asserts `!is_empty()`) |
| **Placeholder catalogue** | `catalogue/🦀️.rs:3–5` | Explicit placeholder comment |
| **FEM not in evaluate** | `schema/🦀️.rs:1194–1221` | `check_steel_member_from_fem` behind `cross-fem` feature; not called from `evaluate` |
| **Classification helpers unused** | `schema/🦀️.rs:525–572` | `section_class`, `chs_class` only in unit tests |
| **No `todo!()`** | — | None found |
| **DE-NA partial** | `schema/🦀️.rs:497–504` | Only γ_M1=1.1 vs EN 1.0 implemented; DIN EN NA has further deviations (buckling curves, S235/S355 tables, etc.) |

---

## 3. Complete subject definition (engineering target)

A feature-complete EN 1993 artifact must model a **steel structure or sub-structure** under assessment:

```
Project
├── NationalAnnex (EN | DE-NA + NA tables)
├── Materials[] (grade, f_y(t), f_u, E, G, subgrade, stainless/HSS flags)
├── Sections[] (catalogue ref or parametric: I/CHS/RHS/plate; t, b, r; A, I, W_pl, W_el, A_v)
├── Members[] (section, material, L, L_cr,y, L_cr,z, L_LT, end conditions, ψ factors)
├── Connections[] (bolted: layout, bolt grade/size; welded: a, l, type, orientation)
├── LoadCases[] (ULS/SLS/FAT/FIRE; N, M, V per member/connection)
├── SpecialElements (cold-formed profile, silo shell, bridge joint, tower leg, pile, crane beam)
└── AnalysisModel? (FEM nodes/elements → envelope N,M,V)
```

**Per-part minimum fields:**

- **1-1**: Section class (Table 5.2), N/M/V resistances, M+N+V interaction (§6.2.8/6.2.10), flexural buckling (§6.3.1, curves a0–d, λ̄ from N_cr), LTB (§6.3.3), tension net/gross, SLS deflection/vibration limits.
- **1-2**: Fire load case, protection system (board/spray), section factor A_m/V, μ₀ from cold design, θ_a,cr, load level R.
- **1-3**: Profile geometry (b̄, t, k_σ, ψ), effective width ρ, built-up constraints.
- **1-4**: Stainless grade, ε with E, γ_M=1.1, section class, M/V/N as applicable.
- **1-5**: Plate panel (a, b, t, σ_x,Ed, boundary conditions), λ_p from geometry.
- **1-6**: Shell (r, t, L, quality class, meridional/circumferential stresses).
- **1-8**: Full joint: bolt group (shear planes, tension, block tearing), weld (directional, throat), bearing edge distances.
- **1-9**: Detail category, spectrum, γ_Ff, γ_Mf, Δσ_E2, N_Ed cycles, damage Σ(n_i/N_i).
- **1-10**: T_Ed, σ_Ed/f_y(t), subgrade, thickness limit from Table 2.1.
- **1-11**: Cable/rod F_uk, F_k, end fittings.
- **1-12**: S460–S700 elastic checks, class restrictions.
- **2**: Bridge-specific interaction, damage-equivalent fatigue (λ, Φ₂).
- **3-1**: Tower loading (wind/ice), leg buckling, connection details.
- **4-1**: Silo geometry, fill properties (γ, k, φ), pressures, shell buckling.
- **5**: Pile section, driving stresses, soil reduction (not a single k_red scalar).
- **6**: Crane runway beam, wheel loads, local web crippling/stiffener checks.

**DIN EN NA specifics**: γ_M1=1.1 (implemented), national buckling curve choices (Table 6.1 NA), S235/S275/S355/S460 selection rules, fire NA parameters, bridge fatigue γ_Mf — most **not** in code.

---

## 4. Check catalogue (implemented)

| Part | Clause (as coded) | Verified | Required inputs | Limit source (DE vs EN) | Failure meaning |
|------|-------------------|----------|-------------------|-------------------------|-----------------|
| 1-1 | §6.2.4 | N_Ed ≤ N_Rd | `n_ed_kn`, `a_mm2`, `f_y_mpa`, annex | N_Rd=A·f_y/γ_M0; γ_M0=1.0 both | Axial ULS fail |
| 1-1 | §6.3.1 | N_Ed ≤ N_b,Rd | `n_ed_kn`, `a_mm2`, `f_y_mpa`, **`chi` (input)** | N_b,Rd=χ·A·f_y/γ_M1; **DE γ_M1=1.1**, EN=1.0 | Buckling fail |
| 1-1 | §6.2.5 | M_Ed ≤ M_c,Rd | `m_ed_knm`, `w_pl_mm3`, `f_y_mpa` | W_pl·f_y/γ_M0 | Bending fail |
| 1-1 | §6.2.6 | V_Ed ≤ V_pl,Rd | `v_ed_kn`, `a_v_mm2`, `f_y_mpa` | A_v·f_y/(√3·γ_M0) | Shear fail |
| 1-1 | §6.2.3 | N_t,Ed ≤ N_t,Rd | `tension_n_ed_kn`, `a_mm2`, `a_net_mm2`, `f_y_mpa`, `f_u_mpa` | min(gross, 0.9·A_net·f_u/γ_M2); γ_M2=1.25 | Tension fail |
| 1-2 | §4.2 | t_protection ≥ t_req | `fire_thickness_mm`, `fire_rating`, `fire_massivity` | **Simplified** board formula | Fire protection fail |
| 1-2 | §4.2.3/4.22 | θ_a,cr ≥ θ_design | `fire_mu_0`, `fire_design_temperature_c` | Eq. 4.22 | Critical temp fail |
| 1-3 | §5.5.2 | N_Ed ≤ N_eff,Rd | `cf_*` group | ρ from λ_p(geometry) | Cold-formed fail |
| 1-4 | §6 | M_Ed ≤ M_Rd | `stainless_*` | γ_M=1.1 (EN base) | Stainless fail |
| 1-5 | §4.1 | σ_Ed ≤ σ_c,Rd | `plated_lambda_p`, `plated_sigma_ed_mpa`, `f_y_mpa` | ρ(λ_p)·f_y/γ_M0 | Plate buckling fail |
| 1-6 | §8.5.2 | σ_x,Ed ≤ σ_x,Rd | `shell_sigma_x_ed_mpa`, `silo_t_mm`, `silo_r_mm`, `f_y_mpa` | χ from λ̄; E=210000 fixed | Shell buckling fail |
| 1-8 | §3.6.1 | F_Ed ≤ F_v,Rd | `bolt_*` | α_v=0.6, γ_M2=1.25 | Bolt shear fail |
| 1-8 | §3.6.1b | F_Ed ≤ F_b,Rd | `bolt_*` | Table 3.4 α_b, k₁ | Bearing fail |
| 1-8 | §4.5.3.3 | F_Ed ≤ F_w,Rd | `weld_*` | Simplified fillet, β_w table | Weld fail |
| 1-9 | §8 | Δσ ≤ Δσ_C/γ_Mf | `delta_sigma_mpa`, `fatigue_category`, `fatigue_method` | γ_Mf: 1.0/1.15/1.35 | Fatigue fail |
| 1-10 | §2.1 | t ≤ t_max(T_Ed, subgrade) | `t10_*` | Table 2.1 bilinear | Toughness fail |
| 1-11 | §6.2 | N_Ed ≤ F_Rd | `tension_component_*` | min(F_uk/1.5, F_k) | Cable fail |
| 1-12 | §4.1 | M_Ed ≤ M_el,Rd | `hss_*` | Elastic only; class>3 → Rd=0 | HSS fail |
| 2 | §6 | η ≤ 1 | `n_ed`, `m_ed`, resistances | \|N/N_Rd\|+\|M/M_Rd\| | Bridge interaction fail |
| 2 | §9.3 | Δσ_E2 ≤ limit | `bridge_lambda`, `bridge_phi_2`, `bridge_delta_sigma_p_mpa` | λ·Φ₂·Δσ_p vs Δσ_C/γ_Mf | Bridge fatigue fail |
| 3-1 | §5 | N_Ed ≤ N_b,Rd/gust | `tower_*`, shared `chi` | Wind factor divisor | Tower fail |
| 4-1 | §5.3 | σ_θ,Ed ≤ σ_Rd | `silo_k`, `silo_gamma`, `silo_depth`, `silo_r`, `silo_t` | Janssen p_h vs shell χ·f_y/γ_M1 | Silo wall fail |
| 5 | §12.1 | σ ≤ 0.9·f_y | `pile_sigma_mpa`, `f_y_mpa` | Driving limit | Driving stress fail |
| 5 | §6.1 | N_Ed ≤ N_c,Rd | `pile_n_ed_kn`, `pile_k_red`, member props | k_red·A·f_y/γ_M0 | Pile compression fail |
| 6 | §5.7.1 | σ_oz ≤ f_y/γ_M0 | `crane_*`, `f_y_mpa` | Wheel load web stress | Crane runway fail |

**Not implemented** (claimed by family scope): §6.2.8/6.2.10 M+N+V interaction, §6.3.2 torsional, §6.3.3 LTB, §6.3.4 combined buckling, §7 SLS, §6.61/6.62 member interaction formulae, block tearing, preloaded bolts, partial-strength joints, 1-2 advanced thermal analysis, 1-8 full joint resistance tables, 1-9 cumulative damage, FEM-driven envelopes.

---

## 5. Remediation strategy (per check type)

| Check | Remediation target field(s) | How to compute target |
|-------|----------------------------|------------------------|
| Axial §6.2.4 | `a_mm2` or `f_y_mpa` or reduce `n_ed_kn` | A_req = N_Ed·γ_M0·1000/f_y |
| Buckling §6.3.1 | `chi` (today) → **L_cr, section, curve** | Solve χ≥N_Ed·γ_M1·1000/(A·f_y); invert λ̄ from curve; pick section with N_cr≥… |
| Bending §6.2.5 | `w_pl_mm3` | W_pl,req = M_Ed·γ_M0·10⁶/f_y |
| Shear §6.2.6 | `a_v_mm2` | A_v,req = V_Ed·√3·γ_M0·1000/f_y |
| Net tension §6.2.3 | `a_net_mm2` or bolt layout | A_net,req = N_Ed·γ_M2/(0.9·f_u·1000) if rupture-governed |
| Fire protection | `fire_thickness_mm` | t ≥ board_thickness_mm(rating, massivity) — replace with NA table method |
| Critical temp | `fire_mu_0` (redesign cold util.) or protection | Lower μ₀ or increase protection so θ_a,cr≥θ_fire |
| Cold-formed | `cf_t_mm` or `cf_b_bar_mm` | Reduce λ_p until ρ≥N_Ed/N_gross |
| Bolts shear | `bolt_n_bolts`, `bolt_a_s_mm2`, grade | n·α_v·A_s·f_ub/γ_M2 ≥ F_Ed |
| Bolts bearing | `bolt_e1_mm`, `bolt_e2_mm`, `bolt_t_mm` | Increase edge distance or plate thickness |
| Weld | `weld_a_mm`, `weld_l_mm` | a·l ≥ F_Ed·√3·β_w·γ_M2·1000/f_u |
| Fatigue | `delta_sigma_mpa` or category | Δσ_allow = Δσ_C/γ_Mf; improve detail category |
| Toughness | `t10_steel_subgrade` or reduce `t10_actual_thickness_mm` | Lookup Table 2.1: pick subgrade or t≤t_max |
| Silo | `silo_t_mm` | t ≥ p_h·r/σ_Rd from Janssen |
| Crane | `crane_t_w_mm` or wheel load | t_w ≥ F_z·1000/(l_eff·f_y/γ_M0) |

All remediation requires new `CheckResult.remediation: LocalizedText` + `subject_ref` (Wave B core).

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| No remediation | `CheckResult` (`⚖️compliance/🦀️.rs:138–146`) has `message: String` only |
| Messages English-only | e.g. `"cross-section axial ULS"` — not localized |
| No subject-element link | Cannot jump from failing check to input field/group |
| Inputs = raw JSON | No typed forms per mutation group (`📥️inputs/🦀️.rs:19–21`) |
| Results row format | `u={:.2}` + Debug status (`app-surface/🦀️.rs:213`) — no computed/limit quantities shown in list |
| Inspection panel | Shows clause/status/u/message — not computed vs limit values |
| Catalogue | Placeholder (`catalogue/🦀️.rs:3–5`) |
| Example i18n | German label missing (`high-strength-connection/🦀️.rs:7`) |
| Annex switch UX | Mutation exists; first 3 checks still use DE factors regardless |
| Viewer | Report only; no edit path |
| Export | `report:out` via `export_media` — format not remediation-aware |

---

## 7. Target design

### 7.1 Snapshot sketch

```rust
pub struct En1993Snapshot {
    pub annex: AnnexChoice,
    pub materials: Vec<MaterialId>,
    pub sections: Vec<SectionDef>,      // geometry + derived A, I, W_pl, class
    pub members: Vec<MemberId>,         // section, material, lengths, buckling lengths
    pub connections: Vec<ConnectionDef>,
    pub load_cases: Vec<LoadCase>,      // ULS/SLS/FAT/FIRE tags
    pub member_envelopes: Vec<Envelope>,// N,M,V per case (or FEM ref)
    pub silos: Vec<SiloDef>,
    pub bridge_fatigue: Option<BridgeFatigueDef>,
    // ...
}
```

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1993Snapshot) -> CheckReport {
    let params = AnnexParams::for_choice(doc.annex);
    for member in &doc.members {
        let env = envelope(member, &doc.load_cases);
        let sec = section(member.section, &doc.sections);
        let class = classify(sec, material);
        report.extend(member_uls(member, env, sec, class, params));
        report.extend(member_buckling(member, sec, params)); // compute χ
        report.extend(member_ltb(member, sec, params));
        report.extend(member_interaction(env, sec, params)); // §6.61/6.62
    }
    for conn in &doc.connections { report.extend(joint_checks(conn, params)); }
    // part-specific blocks...
}
```

### 7.3 Example subjects

**Compliant**: Single HEB 240 S355, L=4 m, N_Ed=200 kN, M_Ed=80 kNm, V_Ed=50 kN, pinned-pinned curve b, DE-NA — all η≤1.

**Non-compliant**: Same member, N_Ed=900 kN (buckling η>1), bolt group M24 insufficent (bearing η>1), fire μ₀=0.85 with R60 protection 15 mm (thickness fail), fatigue Δσ=90 MPa on category 71 damage-tolerant (η>1).

### 7.4 Numeric tests to add

1. `evaluate_heb240_compliant` — assert each check Pass with expected η (N_Rd=1910.6 kN for A=5382, f_y=355, γ_M0=1.0).
2. `evaluate_annex_changes_buckling_only` — DE vs EN γ_M1 changes η for §6.3.1 only.
3. `evaluate_bolt_bearing_fail_remediation` — F_Ed=150 kN > F_b,Rd=123.6 kN; inverse gives e1≥48 mm.
4. `evaluate_silo_janssen_consistent` — Part 4 stress matches geometry; Part 1-6 uses same σ_x,Ed.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema | `🧬️schema/🦀️.rs`, `📸️snapshot/*`, `🔺️diff/*` |
| Inferences | `💡️inferences/🦀️.rs`, new `🧾outline` fields |
| Mutations | All 17 `update-*` groups → structural mutations |
| Editor | `📥️inputs` typed panels per entity; `📊️results` remediation rows |
| Viewer | `👁️viewer/…/report` parity with editor results |
| Examples | Multi-scenario compliant/non-compliant DSL |
| Tests | `⚖️compliance`, `🔬️compliance-report`, oracles |
| Core (Wave B) | `⚖️compliance/🦀️.rs` remediation model; `🖥️app-surface/🦀️.rs` render |

---

## 8. Risks / open questions

1. **Scope boundary**: Is one artifact one building, one member, or a catalogue of checks? Current flat sheet implies "all parts at once" which is unrealistic for production.
2. **FEM integration**: `cross-fem` exists but is orthogonal to `evaluate` — should envelopes replace scalar `n_ed_kn`/`m_ed_knm`?
3. **DE-NA depth**: Which DIN EN NA tables are MVP (γ factors only vs full curve/grade selection)?
4. **Wave B dependency**: Remediation UX blocked on core `CheckResult` extension — coordinate with `📓️impl-core.md`.
5. **Oracle gap**: No third-party Eurocode steel reference tool (`🔮️oracles/🔣️.json:17` acknowledges); compliance tests are self-referential.
6. **`check_steel_member` annex bug**: Is hardcoded DE intentional legacy? Must fix before annex mutation is meaningful.
7. **Silo dual-stress**: Should Part 4 and Part 1-6 share one computed meridional stress, making `shell_sigma_x_ed_mpa` derived read-only?
8. **Performance**: Hierarchical subject with FEM may need caching in `En1993Inference` beyond `outline`.

---

*Auditor: read-only pass, 2026-09-26. Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993`.*
