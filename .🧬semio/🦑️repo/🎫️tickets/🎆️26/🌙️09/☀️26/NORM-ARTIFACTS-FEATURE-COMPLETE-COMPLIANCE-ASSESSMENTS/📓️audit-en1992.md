# EN 1992 — Per-Family Compliance Audit

**Executive summary.** The `🏛️en1992` artifact is a **flat bag of 35 scalars** (one RC beam + fire + two bridge SLS scalars + liquid-tightness scalars + one anchor) wired to **9–10 utilization checks** in `evaluate()`. Helper libraries for punching, torsion, crack width, deflection, durability cover, `V_Rd,s`, and exposure-class SLS exist but are **never called**. Shear uses only `V_Rd,c` with no DIN NA `cot θ` limits and no stirrup path; flexure ignores axial force despite `n_ed_kn`; fire uses a single Table 5.5 beam check while `check_fire_cover` is dead code with **swapped computed/limit**. DE-NA differs only on `α_cc`/`α_ct`; bridge/fatigue/anchor/liquid checks hardcode `AnnexChoice::En`. Reports show clause/status/utilization/message in English only — **no remediation, no subject-field binding, no localized check text**. Default snapshot **fails shear** (`V_Ed=80 kN > V_Rd,c≈59 kN`). Mutation/oracle coverage is strong (35 kinds, Python second implementation); compliance UX and norm completeness are not.

---

## 1. Inventory

### 1.1 Snapshot / document fields (`En1992Snapshot`, 35 fields)

Source: `…/🧬️schema/📸️snapshot/🦀️.rs` L20–113, mirrored in `…/🧬️schema/🦀️.rs` L14–85.

| Field | Type | Unit (DSL) | Role |
|-------|------|------------|------|
| `annex` | `AnnexChoice` | — | EN vs DE-NA material factors |
| `m_ed_knm` | f64 | kNm | Flexure demand (analytical path) |
| `v_ed_kn` | f64 | kN | Shear demand |
| `f_ck` | f64 | MPa | Concrete strength |
| `b_mm`, `d_mm` | f64 | mm | Section width, effective depth |
| `a_s_mm2` | f64 | mm² | Tension steel area |
| `f_yk` | f64 | MPa | Reinforcement yield |
| `rho_l` | f64 | — | Longitudinal reinforcement ratio (shear) |
| `n_ed_kn` | f64 | kN | Axial force (shear `σ_cp` only) |
| `p_kn`, `a_c_mm2` | f64 | kN, mm² | Prestress transfer check (if `p_kn>0`) |
| `use_fem` | bool | — | FEM vs analytical actions |
| `span_m`, `udl_kn_m` | f64 | m, kN/m | FEM simply-supported UDL beam |
| `fire_rating` | `FireRating` | — | R30/R60/R90/R120 |
| `provided_axis_distance_mm` | f64 | mm | Fire axis distance *a* |
| `bridge_sigma_c_mpa` | f64 | MPa | Bridge frequent concrete stress |
| `bridge_delta_sigma_s_mpa` | f64 | MPa | Bridge fatigue stress range |
| `tightness_class` | `TightnessClass` | — | TC0/TC1/TC2 |
| `hd_over_h` | f64 | — | Liquid head ratio (TC2 crack limit) |
| `liquid_sigma_s_mpa` | f64 | MPa | Steel stress for crack calc |
| `liquid_rho_p_eff` | f64 | — | Effective reinforcement ratio |
| `liquid_f_ct_eff_mpa` | f64 | MPa | Effective tensile strength |
| `liquid_e_s_mpa` | f64 | MPa | Steel modulus |
| `liquid_s_r_max_mm` | f64 | mm | Max crack spacing |
| `anchor_*` (10 fields) | mixed | mm, MPa, kN, bool | EN 1992-4 single anchor |

**Missing from schema:** exposure class (`XC*`, `XD*`, `XS*`), `c_min,dur`, cover, stirrup geometry, `cot θ`, punching perimeter, torsion, SLS limits, `l/d`, member list, load cases, materials catalogue, detailing rules.

**Defaults** (`📸️snapshot/🦀️.rs` L125–162): `annex=De`, `m_ed=120`, `v_ed=80`, `f_ck=30`, `b=300`, `d=450`, `a_s=1200`, `f_yk=500`, `rho_l=0.01`, `use_fem=false`, etc.

### 1.2 Composed children

None. Flat scalar artifact; no `store::ArtifactChild`, zones, members, or layers.

### 1.3 Mutations (35)

`…/🧬️mutations/🦀️.rs` L67–103, `KINDS` L110–146: one `change-<field>` per snapshot field. Each triad: `🦀️.rs`, `🔺️diff/`, `↩️inverse/` (undo-only, e.g. `change-n-ed-kn/↩️inverse/🦀️.rs` L9–10). Fixture vectors: 35 scenarios in `🔮️oracles/🔣️.json`; Python mirror `🧪️tests/🏛️mutate-en1992-1/🐍️.py`.

### 1.4 `evaluate()` call graph

`…/💡️inferences/🦀️.rs` L79–106:

```
evaluate(snapshot)
├─ [use_fem] check_rc_beam_from_fem → check_rc_beam (flexure + shear)     [cross-fem feature]
│            OR CheckReport::default() on FEM error / no feature          L83–87
└─ [else]     check_full_rc_beam → check_rc_beam + optional prestress     L90–91
├─ part_1_2::check_fire_beam_axis_distance(b, provided_a, fire_rating)    L93
├─ part_2::check_bridge_concrete_stress(bridge_sigma_c, f_ck)             L95
├─ part_2::check_bridge_fatigue(bridge_delta_sigma_s)                     L96
├─ part_3::crack_width_tightness_mm → check_tightness_crack_width         L98–99
└─ part_4::check_anchor_steel / cone / edge_shear                         L101–104
```

**Reachable helpers:** `part_1_1::{flexural_resistance,shear_resistance,prestress_*}`; `part_1_2::{required_axis_distance_beam,check_fire_beam_*}`; `part_2::{concrete_stress_limit,check_bridge_*}`; `part_3::{crack_width_tightness,tightness_crack_width_limit,check_tightness_*}`; `part_4::{steel/cone/edge resistances}`; `na_de::AnnexParams` (flexure/punching/torsion helpers only).

**Defined but NOT reached by `evaluate()`** (`🧬️schema/🦀️.rs`):

| Lines | Dead API |
|-------|----------|
| 397–406 | `punching_v_rd_max_mpa`, `punching_resistance_kn`, `check_punching` |
| 408–414, 462–469 | `torsion_resistance_knm`, `check_torsion` |
| 426–437, 472–474 | `crack_width_wk_mm`, `steel_strain_eps_sm`, `check_crack_width` |
| 437–442, 476–478 | `deflection_ss_udl_mm`, `check_deflection` |
| 416–424 | `slenderness_lambda`, `radius_of_gyration_mm` |
| 509–530 | `min_axis_distance_mm`, `check_fire_cover` |
| 568–570 | `check_bridge_flexure` |
| 611–626 | `crack_width_liquid_mm`, `check_liquid_crack_width`, `check_steel_stress` |

### 1.5 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `✏️editor/…/📥️inputs/🦀️.rs` L19–21 | **JSON dump** of full snapshot — no typed form |
| Results | `✏️editor/…/📊️results/🦀️.rs` L22–24 | Virtualized check list via `app_surface::render_report` |
| Inspection | `✏️editor/📌️panels/🔍️inspection/🦀️.rs` L19–20 | Single check: clause, status, u, message |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs` L30–32 | Table: Clause, Status, Utilization, Message |
| Catalogue | `✏️editor/📌️panels/📚️catalogue/🦀️.rs` L3–5 | **Placeholder** headline only |
| Example | `📚️examples/🛢️liquid-retaining-fem-anchor/` | One DSL; label **not** German-localized (L7) |

Window/presence/config slots: 27 `📌️.empty.md` placeholders under editor/viewer trees.

### 1.6 Examples, assets, tests, oracles

- **Example:** `🖼️assets/…/🗣️.dsl.semio` — FEM+liquid+anchor scenario (`use-fem=true`, `annex=en`, TC2, cracked anchor).
- **Compliance unit tests:** `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — 16 tests; several assert numeric values (punching 4.488 MPa L12–13, FEM M≈90 kNm L59–60, cone N≈43111 N L129–130). `rc_beam_e2e` only asserts `!is_empty()` and `u>0` (L6–7).
- **Evaluate integration:** `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — default → 9 checks, all families present (L23–31); prestress → 10 checks (L15–19).
- **Oracle:** `🔮️oracles/🔣️.json` — mutation-only; **no third-party Eurocode oracle** for formulae (L17–18).

---

## 2. Stub / fake detection

### 2.1 Simplified / surrogate models (quoted)

| Location | Issue |
|----------|-------|
| `🧬️schema/🦀️.rs` L387–394 | Shear: **only `V_Rd,c`**, no `V_Rd,s`, no `θ`, no `V_Ed,max`, no size effect beyond `k` |
| `🧬️schema/🦀️.rs` L379–385 | Flexure: rectangular block, **no N–M interaction**; `n_ed_kn` unused in flexure |
| `🧬️schema/🦀️.rs` L509 | Fire `min_axis_distance_mm`: comment admits **"simplified tabulated values"**; not DIN NA Table |
| `🧬️schema/🦀️.rs` L698–719 | Anchor edge breakout: **"simplified form"** of Eq. 7.2.2.5; clause tagged `7.2.2.5 (simplified)` |
| `🧬️schema/🦀️.rs` L777 | FEM: `e: 30e9` **hardcoded** MPa, `density: 2500`, `n_ed` forced **0** in FEM path (L784) |
| `💡️inferences/🦀️.rs` L83 | FEM failure → **empty `CheckReport`** then still appends other checks — silent partial failure |
| `💡️inferences/🦀️.rs` L85–87 | `use_fem=true` without `cross-fem` feature → **empty RC checks**, no diagnostic |
| `🧬️schema/🦀️.rs` L527–530 | `check_fire_cover`: uses `from_utilization(required, cover)` — **computed/limit inverted** vs `check_fire_beam_axis_distance` (L548–550, correct `from_minimum`) |
| `🧬️schema/🦀️.rs` L529 | `check_fire_cover` hardcodes `AnnexChoice::De` regardless of document annex |
| `🧬️schema/🦀️.rs` L550 | Fire beam check hardcodes `AnnexChoice::En` |
| `part_2` L569, L579, L591 | Bridge checks hardcode `AnnexChoice::En` — **no DE-NA path** |
| `part_3` L607, L625 | `steel_stress_limit_mpa`: unknown exposure → **200 MPa default** (silent) |
| `part_4` L709–719 | All anchor checks `AnnexChoice::En` |

### 2.2 Hardcoded constants standing in for inputs

| Location | Constant |
|----------|----------|
| `part_2` L560, L563 | `GAMMA_F_FAT=1.0`, `GAMMA_S_FAT=1.15`, `DELTA_SIGMA_RSK_MPA=162.5` — not tied to bar type/cycle count |
| `part_4` L664 | `GAMMA_MC=1.5` |
| `part_1_2` L412 | Torsion `alpha_cw=1.0` |
| `part_1_1` L381 | `γ_s=1.15` inline in `f_yd` (not from `AnnexParams.gamma_s`) |

### 2.3 DE-NA vs EN divergence (implemented vs missing)

**Implemented:** `na_de::AnnexParams` (`🧬️schema/🦀️.rs` L331–371): DE `α_cc=α_ct=0.85` vs EN `1.0`; `γ_c=γ_s` same. Tests L75–89 confirm `M_Rd,EN > M_Rd,DE`.

**Missing DIN EN 1992-1-1/NA specifics:**

- Shear: **no `cot θ` limits** (NA typically 2.5–2.0); no `V_Rd,s` with `α_cw`, `ν`, `f_ywd`
- Durability: **no exposure classes**, **no `c_min,dur` table** (DE Table 4.4N differs from EN)
- Crack width SLS: no `w_max` from exposure / quasi-permanent combination
- Deflection: no `l/d` basic ratios (NA Table 7.4N)
- Min/max reinforcement §9.3 — **absent**
- Fire: simplified tables, not full NA tabulated `(b_min, a)` workflow for all element types
- Bridge EN 1992-2: only 2 of many SLS/ULS provisions

### 2.4 Checks that can never fail / misleading pass

- `check_tightness_crack_width` for **TC0** → always `NotApplicable` (L649–652) — OK by norm, but default `annex=De` + `tc1` never exercises TC0 N/A in default evaluate.
- `check_fire_cover` (dead): inverted arguments would mark **non-compliance as pass** if ever wired.
- Default snapshot: **shear fails** (~u=1.35) but no test asserts default evaluate status.

### 2.5 Placeholders / empty

- Catalogue panel (§1.5).
- 27 `📌️.empty.md` editor/viewer slots.
- Example `label()` L7: German equals English string.

### 2.6 Tests weak on compliance outcomes

- `rc_beam_e2e` L6–7: `!is_empty()`, `u>0` only.
- No test asserts **full default `evaluate()` pass/fail per check**.
- No oracle compares `CheckReport` numbers to independent Eurocode tool.

---

## 3. Complete subject definition (engineering target)

A feature-complete EN 1992 artifact should model a **structure assessment project**, not a single beam scalar card.

### 3.1 Top-level

```text
En1992Project
├── meta: { title, annex: En|De, design_code_refs[], assessor, date }
├── materials: ConcreteGrade[], ReinforcementGrade[], PrestressSteel[]
├── exposure_environment: { exposure_classes[], cement_type, Δc_dev, design_working_life }
└── elements: Element[]   // typed, with cross-refs
```

### 3.2 EN 1992-1-1 (general buildings)

**Per `Beam` / `Slab` / `Column` / `Wall` / `Foundation`:**

- **Geometry:** `b`, `h`, `d`, `cover`, `axis_distance_a`, `l_eff`, `l_span`, section type
- **Materials:** `f_ck`, `f_yk`, `f_ywd`, `E_cm`, `E_s`; DE-NA `α_cc`, `α_ct`, `γ_c`, `γ_s`
- **Reinforcement:** `A_s`, `A_s_min`, `A_s_max`, `ρ_l`, `ρ_p_eff`, bar diameters, spacing, `s_r,max` (Eq. 7.11)
- **Durability:** `exposure_class` → `c_min,dur` (+ `Δc_dev`) → required cover
- **ULS bending:** `M_Ed`, `N_Ed` → `M_Rd` with N–M diagram (§6.1, §6.8)
- **ULS shear:** `V_Ed`, `V_Ed,max`; `V_Rd,c`; if `V_Ed > V_Rd,c` → stirrups: `A_sw/s`, `θ` within NA limits, `V_Rd,s`, `V_Rd,max`
- **ULS torsion + punching** (slabs): `T_Ed`, `T_Rd`; `V_Ed,punch`, `u_1`, `β`, `V_Rd,c`, `V_Rd,s`, `v_Rd,max`
- **SLS crack:** quasi-permanent `σ_s`, `w_k` (Eq. 7.8–7.9) vs `w_max` (Table 7.1N / NA)
- **SLS stress:** σ_c, σ_s under characteristic / quasi-permanent
- **SLS deflection:** `δ` vs limit; simplified `l/d` vs Table 7.4N
- **Detailing:** §9.3 min/max reinforcement, bar spacing, anchorage lengths
- **Prestress:** transfer, service, ultimate; losses; `σ_c ≤ 0.6 f_ck(t)` etc.

### 3.3 EN 1992-1-2 (fire)

Per member: `fire_rating` (R/E/I/M), `element_type`, `b_min`, axis distance `a`, tabulated data (§5.6, Table 5.5–5.8), optional simplified vs advanced zone method.

### 3.4 EN 1992-2 (bridges)

Per bridge member: traffic/load model refs (with EN 1991), fatigue spectrum, `Δσ` at `N*`, concrete stress under **frequent** combination, shear/flexure with bridge-specific γ, tendon deviator checks, etc.

### 3.5 EN 1992-3 (liquid retaining)

Per retaining element: `tightness_class` TC0–TC2, `h_D/h`, exposure, `w_k` limits (Table 7.1N), steel stress limits (Table 7.1N), joint detailing, thermal/seasonal effects.

### 3.6 EN 1992-4 (fastenings)

Per anchor / group: geometry (`h_ef`, `c_1`, `c_2`, `s`, `d`), cracked state, loads (`N_Ed`, `V_Ed`, combined), failure modes (steel, cone, pull-out, edge, splitting, pry-out), group factors — not single-anchor simplified cone only.

### 3.7 FEM path

`Model` with members, sections, materials, load cases/combinations → extracted `M_Ed`, `V_Ed`, `N_Ed`, deflections, then clause checks. Must not hardcode `E`, must propagate `n_ed`, support multiple members.

---

## 4. Check catalogue

### 4.1 Implemented in `evaluate()` (default analytical, `p_kn=0`)

| Part | Clause / eq. | Verified | Required inputs | Limit source | Failure meaning |
|------|--------------|----------|-----------------|--------------|-----------------|
| 1-1 | §6.1 | `M_Ed ≤ M_Rd` | `m_ed_knm`, `f_ck`, `b_mm`, `d_mm`, `a_s_mm2`, `f_yk`, `annex` | `M_Rd` from rectangular block; DE: `f_cd=0.85·f_ck/1.5` | Flexural ULS insufficient |
| 1-1 | §6.2 | `V_Ed ≤ V_Rd,c` | `v_ed_kn`, `b_mm`, `d_mm`, `f_ck`, `rho_l`, `n_ed_kn` | Eq. 6.2 a–c; **no `V_Rd,s`** | Concrete shear capacity exceeded |
| 1-1 | §5.10 | `σ_c ≤ 0.6 f_ck` | `p_kn`, `a_c_mm2`, `f_ck` (if `p_kn>0`) | §5.10.9 | Prestress transfer stress too high |
| 1-2 | Table 5.5 / §5.6.3 | `a_provided ≥ a_req(b, R)` | `b_mm`, `provided_axis_distance_mm`, `fire_rating` | Interpolated table (EN values) | Fire axis distance insufficient |
| 2 | §7.2 | `σ_c,freq ≤ 0.6 f_ck` | `bridge_sigma_c_mpa`, `f_ck` | 0.6·f_ck | Bridge concrete overstress (frequent) |
| 2 | §6.8.4 | `γ_F,fat·Δσ_s ≤ Δσ_Rsk/γ_S,fat` | `bridge_delta_sigma_s_mpa` | 162.5/1.15 ≈ 141.3 MPa | Fatigue range exceeded |
| 3 | Table 7.1N / §7.3.2 | `w_k ≤ w_lim(TC, h_D/h)` | liquid_* fields, `tightness_class`, `hd_over_h` | TC1: 0.3 mm; TC2: 0.2→0.05 mm | Leakage risk — crack too wide |
| 4 | §7.2.1.4 | `N_Ed ≤ N_Rd,s` | anchor steel fields | `A_s·f_uk/γ_Ms` | Anchor steel fracture |
| 4 | §7.2.1.5 | `N_Ed ≤ N_Rd,c` | `f_ck`, `h_ef`, `cracked` | Eq. 7.2 cone / γ_Mc | Concrete cone failure |
| 4 | §7.2.2.5 (simp.) | `V_Ed ≤ V_Rd,c,edge` | `d`, `h_ef`, `c_1`, `f_ck`, `v_ed` | Simplified polynomial | Edge breakout |

**Default snapshot computed (DE, analytical):** `M_Rd≈208 kNm` (pass); `V_Rd,c≈59 kN` (**fail** at 80 kN); fire pass (a_req≈25 mm @ b=300 R60); bridge/fatigue/liquid/anchor pass.

### 4.2 Helpers present but NOT in `evaluate()`

| Part | Clause | Gap |
|------|--------|-----|
| 1-1 | §6.4 punching | `check_punching` — no `u_1`, column geometry in schema |
| 1-1 | §6.3 torsion | `check_torsion` — no `T_Ed`, hollow section dims |
| 1-1 | §7.3 crack | `check_crack_width` — no `w_max`, exposure |
| 1-1 | §7.4 deflection | `check_deflection`, `deflection_ss_udl_mm` — no `I`, limit |
| 1-1 | Table 4.4N cover | no exposure / `c_min,dur` |
| 1-1 | §6.2.3 `V_Rd,s` | not implemented |
| 1-2 | §4.2 `check_fire_cover` | dead + wrong comparator |
| 3 | §7.1 steel stress | `check_steel_stress` — no exposure field |
| 2 | flexure | `check_bridge_flexure` dead |

---

## 5. Remediation strategy (target behaviour)

Core `CheckResult` lacks remediation (`⚖️compliance/🦀️.rs` L138–146). Per-check **target inversions** (subject field → required value):

| Check | Remediation template | Fields | Inversion |
|-------|---------------------|--------|-----------|
| Flexure §6.1 | "Increase `a_s_mm2` from {cur} to ≥ {A_s,req} mm²" or "Reduce `m_ed_knm` to ≤ {M_Rd}" | `a_s_mm2`, `m_ed_knm` | Solve rectangular `M_Rd(A_s)≥M_Ed` bisection on `a_s` |
| Shear §6.2 | "Increase `rho_l` to ≥ {ρ_req}" / "Increase `d_mm` to ≥ {d_req}" / "Add shear reinforcement: `A_sw/s ≥ …`" | `rho_l`, `d_mm`, stirrup fields (new) | `V_Rd,c(ρ_l)≥V_Ed`; if impossible, compute `V_Rd,s` with NA `θ_min` |
| Prestress §5.10 | "Reduce `p_kn` to ≤ {P_max=0.6 f_ck A_c}" or "Increase `a_c_mm2`" | `p_kn`, `a_c_mm2` | `P ≤ 0.6 f_ck A_c` |
| Fire Table 5.5 | "Increase `provided_axis_distance_mm` from {a} to ≥ {a_req(b,R)} mm" | `provided_axis_distance_mm` | `required_axis_distance_beam_mm(b, rating)` (exists L543–545) |
| Bridge σ_c | "Reduce `bridge_sigma_c_mpa` to ≤ {0.6 f_ck}" | `bridge_sigma_c_mpa` | `σ_c ≤ 0.6 f_ck` |
| Fatigue | "Reduce `bridge_delta_sigma_s_mpa` to ≤ {141.3} MPa" | `bridge_delta_sigma_s_mpa` | `Δσ_s ≤ Δσ_Rsk/γ_S,fat` |
| Tightness crack | "Reduce `liquid_sigma_s_mpa` to ≤ …" / "Reduce `w_k` via `s_r_max` or `ρ_p_eff`" | `liquid_sigma_s_mpa`, `liquid_s_r_max_mm`, `liquid_rho_p_eff` | invert Eq. 7.8–7.9 for `σ_s` or spacing |
| Anchor steel | "Use larger bar: `anchor_a_s_mm2` ≥ {A_s,req}" | `anchor_a_s_mm2` | `N_Ed ≤ N_Rd,s` |
| Anchor cone | "Increase `anchor_h_ef_mm` to ≥ {h_ef,req}" | `anchor_h_ef_mm` | invert Eq. 7.2 for `h_ef` |
| Anchor edge | "Increase `anchor_c1_mm` to ≥ {c_1,req}" | `anchor_c1_mm` | invert simplified edge formula |

Wire remediation into Wave B `CheckResult` + link `subject_path` + `LocalizedText` (en/de). Reuse existing `↩️inverse/` only for undo, not compliance.

---

## 6. Report & UX gaps

| Gap | Evidence |
|-----|----------|
| No remediation text | `CheckResult.message` is static English tag e.g. `"shear ULS"` (`🦀️.rs` L455) |
| No localization of checks | Column headers English (`app-surface/🦀️.rs` L159–160); `Status` is `Debug` `Pass`/`Fail` |
| No subject-field reference | Cannot jump from failed shear to `v_ed_kn` field |
| Inputs = raw JSON | `inputs/🦀️.rs` L19–21 — unusable for non-expert |
| No typed editors per domain | Fire, bridge, liquid, anchor mixed in one flat JSON |
| Inspection panel minimal | 4 fields (`render_inspection` L245–249) |
| FEM path opaque | User cannot see derived `M_Ed`/`V_Ed` when `use_fem=true` |
| Example i18n | `LocalizedLabel::native("Liquid Retaining Fem Anchor", same)` |
| Catalogue empty | `catalogue/🦀️.rs` L3–5 |

**Partial positives:** Results/viewer windows localized titles ("Results"/"Ergebnisse"); family label DE in `definition()` (`🦀️.rs` L74); virtualized report list handles many checks.

---

## 7. Target design

### 7.1 Snapshot sketch (illustrative)

```rust
pub struct En1992Snapshot {
    pub annex: AnnexChoice,
    pub members: Vec<MemberId>,
    pub member_index: u32, // active member for UI
    // keyed storage via ArtifactChild or id-map
}
pub struct RcBeamMember {
    pub tag: String,
    pub geometry: SectionGeometry,      // b, h, d, cover, a_fire
    pub materials: MaterialRef,
    pub reinforcement: ReinforcementLayout,
    pub exposure: ExposureClass,
    pub actions: ActionSet,             // ULS/SLS combinations from EN1990 link
    pub shear_reinf: Option<ShearReinforcement>,
    pub fire: Option<FireSpec>,
}
```

Keep **scalar card** as `LegacyScalarCard` example only.

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1992Snapshot) -> CheckReport {
    let mut r = CheckReport::default();
    for m in doc.members.active_rc_beams() {
        let actions = resolve_actions(&m, &doc.load_cases);
        r.extend(check_durability_cover(&m));
        r.extend(check_flexure_nm(&m, actions, doc.annex));
        r.extend(check_shear_full(&m, actions, doc.annex)); // Vrdc + Vrds + theta_NA
        r.extend(check_sls_crack_deflection(&m, actions, doc.annex));
        r.extend(check_detailing(&m));
    }
    for a in doc.anchors() { r.extend(check_anchor_group(a, doc.annex)); }
    // bridge / liquid typed subsets
    r
}
```

### 7.3 Example subjects

**Compliant:** C30/37, DE-NA, `b×h=300×500`, `d=450`, `A_s=1800 mm²`, `M_Ed=150`, `V_Ed=55`, `ρ_l=0.015`, XC3 `cover=35 mm`, `a_fire=40` @ R60, TC1 liquid, anchor M12 h_ef=100 uncracked — all u≤1.

**Non-compliant (multi-failure):** Default snapshot + `provided_axis_distance_mm=15` (fire fail) + `bridge_delta_sigma_s_mpa=160` (fatigue fail) + `anchor_n_ed_kn=30` (cone fail) — expect ≥3 distinct failures with remediations.

### 7.4 Numeric worked-example tests

1. **Shear V_Rd,c:** `b=300, d=450, f_ck=30, ρ_l=0.01, N_Ed=0` → `V_Rd,c=58.9 kN` (assert fail at `V_Ed=80`).
2. **DE vs EN flexure:** same section → `M_Rd,DE=208.1`, `M_Rd,EN=245.0 kNm` (±1%).
3. **Fire R60 b=160:** `a_req=35 mm` (existing test L92–98).
4. **Fatigue:** `Δσ_Rd=141.30 MPa`; fail at 150 (L111–112).
5. **Evaluate default:** assert `checks[1].status==Fail` and clause §6.2.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema | `🧬️schema/🦀️.rs`, `📸️snapshot/*`, `🔺️diff/*`, `🧬️mutations/*` |
| Compliance | `💡️inferences/🦀️.rs`, new `members/`, `shear.rs`, `durability.rs` |
| Core | `⚖️compliance/🦀️.rs` (remediation, `LocalizedText` messages) |
| Editor | `✏️editor/…/inputs/` → typed forms; new panels per domain |
| Viewer | `👁️viewer/…/report/` → remediation column |
| Examples | ≥2: compliant + failing multi-check |
| Tests | `🧪️tests/⚖️compliance/`, `🔬️compliance-report/` |
| Oracles | Third-party or hand-derived numeric oracle for `CheckReport` (not just mutations) |

---

## 8. Risks / open questions

1. **Scope:** Is the scalar card intentional forever (demo) or stepping stone? Full multi-member schema is large — prioritize parts (1-1 ULS shear/flexure + durability first)?
2. **FEM coupling:** `cross-fem` feature availability in production builds — empty report risk (`💡️inferences/🦀️.rs` L85–87).
3. **EN 1992-4 simplified edge formula** — acceptable for compliance product or must be full group model?
4. **Bridge/liquid scalars** on every document — should be optional subtrees to avoid N/A noise?
5. **DE-NA tables:** Source of truth for `c_min,dur`, `cot θ`, `l/d` — digitize from DIN PDF or partner data?
6. **EN 1990 linkage:** `M_Ed`, `V_Ed` should come from load combinations — currently user-typed scalars.
7. **Wave B dependency:** Remediation/localization blocked on core `CheckResult` extension.
8. **Third-party oracle gap** (`🔮️oracles/🔣️.json` L17): no external formula verification — regression risk on algebra edits.

---

*Audit: read-only, 2026-09-26. Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992`.*
