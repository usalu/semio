# 🪶 EN 1999 Inventory

Read-only audit of the EN 1999 artifact. No source file was modified.

**Conventions.** `ART` = `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999`. `SUB` = `ART/🏅️standards/🔖️1/🪆️subsets/✳️any`. All file references are relative to `SUB` with a line number, unless marked otherwise. Shared surfaces live outside the artifact: `📇️registry/🧬️contract/🖥️app-surface` (renderers, `NormHost`, catalogue, field-meta lookup) and `⚖️compliance` (`CheckResult`, `Remedy`, `ClauseId`, `CheckReport`). Clause numbers and tables are reported as coded. The DIN text is not in the repo, so numeric values were not verified against the standard.

**Headline numbers.** 470 tracked files under `ART`. Schema, model, check and mutation code is about 34k lines including fixtures, of which roughly 4k are hand-written Rust for checks. The subset has 64 `#[test]` functions. There is no third-party runtime dependency. The Python "oracle" is an in-repo second implementation, not a third-party library. Editor and viewer are property-tree and table UIs only. No 2D or 3D rendering exists in this artifact.

---

## 1. Model

### 1.1 Artifact schema (`🧬️schema/🦀️.rs`)

`En1999Artifact` (line 24, `#[artifact_schema(id = "s.norm.en1999")]`) mirrors the snapshot. All fields are `#[state(artifact)]`:

| Field | Type |
|---|---|
| `annex` | `AnnexChoice` (`En` / `De`, from `semio_s_plugin_norm_compliance`) |
| `materials` | `Vec<AluminiumMaterial>` |
| `sections` | `Vec<AluminiumSection>` |
| `members` | `Vec<AluminiumMember>` |
| `connections` | `Vec<AluminiumConnection>` |
| `fire_scenarios` | `Vec<FireScenario>` |
| `fatigue_details` | `Vec<FatigueDetail>` |
| `cold_formed` | `Vec<ColdFormedSheet>` |
| `shells` | `Vec<AluminiumShell>` |

Conversions `to_snapshot`, `from_snapshot`, `Default` (lines 46-89). The descriptor (lines 91-101) wires the five facet leaves (`rust`, `typescript`, `graphql`, `json_schema`, `proto`) and the snapshot, diff and mutation leaves.

### 1.2 Snapshot model (`🧬️schema/📸️snapshot/🦀️.rs`)

Each entity derives `DslRecord`, `ToValue`, `FromValue`, the canonical-JSON traits, and serde under `cfg(test)`. The table below is what the Rust code declares. The JSON Schema, TS, GraphQL and proto leaves are separate hand-written files and were not cross-checked field by field in this audit.

| Entity (line) | Fields | Notes |
|---|---|---|
| `AluminiumMaterial` (22) | `id`, `designation` | Designation keys like `aw6082-t6` |
| `PlateElement` (34) | `id`, `width` [m], `thickness` [m], `outstand: bool`, `welded: bool`, `weld_position` [m] | `weld_position` defaults to 0.005 for welded elements (`default_plate`, line 326) |
| `AluminiumSection` (56) | `id`, `kind`, `height`, `width`, `flange_thickness`, `web_thickness`, `outer_diameter`, `elements: Vec<PlateElement>` (dsl table) | `kind` is a free string: `extrudedI`, `channel`, `angle`, `rhs`, `box`, `tube`/`chs`. Geometry is discriminated by convention, not by type |
| `MemberAction` (85) | `id`, `kind` (string), `category` (string), `source` (`udl`/`external`), `g_k_line`, `q_k_line` [N/m], `n_k`, `v_y_k`, `v_z_k`, `m_y_k`, `m_z_k` | Used by members, connections, cold-formed sheets and shells. All string enums are unchecked at type level |
| `SupportCondition` (115) | enum `SimplySupported`, `Continuous`, `Cantilever` | wire codes `simplySupported`, `continuous`, `cantilever` |
| `AluminiumMember` (138) | `id`, `section_id`, `material_id`, `length`, `support`, `buckling_length_y/z/t`, `ltb_length`, `c1`, `restrained_ltb: bool`, `actions` (dsl table) | |
| `BoltGroup` (161) | `material` (string grade), `diameter`, `rows: u32`, `bolts_per_row: u32`, `edge_distance`, `pitch`, `gauge`, `plate_thickness` | |
| `WeldGroup` (178) | `filler_alloy`, `throat`, `length`, `beta_w`, `haz_extent` | |
| `AluminiumConnection` (192) | `id`, `member_id`, `material_id`, `kind` (`bolted`/`welded`/`combined`), `actions` (table), `bolts`, `welds` | Missing `member_id` is caught by `evaluate_structure` (see 3.9) |
| `FireScenario` (211) | `id`, `member_id`, `theta_a` [°C], `duration_s` | |
| `FatigueDetail` (224) | `id`, `member_id`, `detail_category` (string), `delta_sigma_c` (derived, `skip_serializing` in test serde), `delta_sigma_ed`, `n_cycles`, `m1`, `m2` | Doc says Δσ_C is derived from `detailCategory`. The check (see 3.8) falls back to the stored value when the category is unknown |
| `ColdFormedSheet` (248) | `id`, `material_id`, `thickness`, `width`, `span`, `welded`, `actions` | |
| `AluminiumShell` (268) | `id`, `material_id`, `radius`, `thickness`, `length`, `actions` | |
| `En1999Snapshot` (291) | `annex`, `materials`, `sections`, `members`, `connections`, `fire_scenarios`, `fatigue_details`, `cold_formed`, `shells` (all dsl tables) | `#[dsl(id = "norm.en1999", layout = "lines")]` |

Notes on the model:

- `default()` returns `compliant_roof_purlin()` (line 346-350). `empty()` (line 354) is an empty shell.
- `compliant_roof_purlin()` (line 369) is an annex-DE `aw6082-t6` I-section purlin with a CHS post, a support weld connection, one fire scenario, one fatigue detail (Δσ_C 71 MPa), one cold-formed sheet and one shell.
- `noncompliant_multi_fail()` (line 533) is an annex-DE `aw6060-t6` weak I-section beam with a thin weld, a short bolt group, a hot fire scenario (320 °C, 3600 s), a 40-weld fatigue detail, a thin sheet and a thin shell.
- Literal constants in the model: `default_bolts()` (line 329: M8.8 Ø12, 2×2, e1 24, p1 36, g 16, t 8 mm) and `default_welds()` (line 342: filler 4043, throat 4 mm, length 120 mm, β_w 0.63, HAZ 25 mm). Both are used as default fixtures.
- `is_non_tube_od` (line 323) has the same body as `is_zero_f64`. The name is misleading.

### 1.3 Mutation vocabulary (`🧬️mutations/`)

`En1999Mutation` (`🧬️mutations/🦀️.rs` lines 44-63) has 18 variants: `ChangeAnnex`, `ChangeMaterials`, `ChangeSections`, `ChangeMembers`, `ChangeConnections`, `ChangeFireScenarios`, `ChangeFatigueDetails`, `ChangeColdFormed`, `ChangeShells`, `AddMember`, `RemoveMember`, `ChangeMemberNEd`, `ChangeMemberMYEd`, `ChangeMemberBucklingLength`, `ChangeMaterialDesignation`, `ChangePlateThickness`, `ChangeWeldThroat`, `ChangeBoltCount`. The kind list is `KINDS` (line 65). Diff entities are in `🔺️diff/🦀️.rs` (925 lines). Per-kind leaves live under `🧬️mutations/<kind>/🦀️.rs` (about 48 lines each).

Edit rules (`🧬️mutations/🧭️edit-rules/🦀️.rs`) map editor paths to kinds: 16 `set_field` rules, 1 `insert_item` rule (`members` → `add-member`), an empty `remove_item` list, and `ID_REMOVALS` for `members` → `remove-member` (lines 35-40 of the file, `resolve_edit` at line 45).

### 1.4 Inference (`💡️inferences/`)

- `En1999Inference` (`💡️inferences/🦀️.rs`) has one derived field, `outline: En1999Outline`. Its `fields()` declares `reads` of `annex`, `materials`, `sections`, `members`, `connections`, `fireScenarios`, `fatigueDetails` only. `coldFormed` and `shells` are missing from the declared reads.
- `En1999Outline` (`🧾outline/🦀️.rs`) returns `section_outline` (a fixed string list), `field_count` (7) and `entry_count` (sum of six collections). `entry_count` omits `cold_formed` and `shells`.
- `evaluate(document)` (`💡️inferences/🦀️.rs`) is a one-liner that calls `evaluate_structure`. This is the single pure compliance entry point used by the editor, the viewer and tests.

---

## 2. Compliance engine (`💡️inferences/⚖️checks/`)

### 2.1 Shared helpers (`⚖️checks/🦀️.rs`)

- Quantity constructors (lines 16-22) for force, moment, stress, length, temperature, time, dimensionless.
- `scale_char_for_limit` (line 26): `current * (limit/design) * 0.99`. This is the generic "reduce the leaf" remedy factor, with a hard-coded 0.99 margin.
- `scale_action_leaf_remedies` (line 34): emits `Remedy::at_most` for every non-fire action leaf that carries the demand, with the `SubjectRef` path `owner[id=…].actions[id=…].leaf`.
- Module mounts (lines 71-93): `parts`, `member_checks`, `connection_checks`, `fire_fatigue_checks`, `thin_wall_checks`, `structure_evaluation`.

### 2.2 EN 1990 combinations and NA-DE parameters (`📏️parts/🦀️.rs`)

- `na_de` (lines 5-26): `HAZ_ZONE_M = 0.025` (declared, **never used**), `E_PA = 70 000 MPa`, `NU = 0.33`, `AnnexParams { gamma_m1, gamma_m2 }`. `en()` and `de()` both give γ_M1 = 1.10 and γ_M2 = 1.25. The doc comment on `de()` states NA-DE keeps these values.
- `part_en1990::psi_row` (lines 35-48):
  - `residential`, `A`, `office`, `B`, `self`: ψ0/ψ1/ψ2 = 0.7 / 0.5 / 0.3
  - `congregation`, `C`, `retail`, `D`: 0.7 / 0.7 / 0.6
  - `storage`, `E`: 1.0 / 0.9 / 0.8
  - `snow`: 0.5 / 0.2 / 0.0. The `if de {…} else {…}` branch on line 41 has identical bodies, so the annex has no effect.
  - `snow_high`: 0.7 / 0.5 / 0.2
  - `wind`: 0.6 / 0.2 / 0.0
  - `temperature`: 0.6 / 0.5 / 0.0
  - `fire`: 0 / 0 / 0
  - default: 0.4 / 0.3 / 0.2
- `characteristic_effects` (lines 69-83) converts each action to `(N, V_y, V_z, M_y, M_z)`.
  - `udl` source: span factors from `span_factor` (lines 61-67). Simply supported uses `wL²/8` and `wL/2`. Continuous uses `wL²/12` and `wL/2`. Cantilever uses `wL/2` and `wL` (K_M, K_V). Line loads are `g_k + q_k`.
  - `external` source: an ad hoc factor (1, 0.5, 1/4 or 1/8) times `w` is added to V_z and M_y. This is not a standard expression for externally supplied forces.
- `governing_member_effects` (lines 98-156): ULS.
  - γ_G = 1.35, γ_Q = 1.50, ξ = 0.85, all hard-coded.
  - Two combinations per leading variable (lines 129-153): `uls-610a` (ψ0 on accompanying actions, γ_G on permanents) and `uls-610b` (ξ·γ_G on permanents, γ_Q on the leader, ψ0 on accompanying actions).
  - The governing case is the one with the largest sum of absolute component values (`util_key`, line 93). This is a heuristic, not the governing utilisation.
  - With no variable action it returns `uls-g` = γ_G · ΣG (line 117).
- `fire_member_effects` (lines 159-177): G + ψ2·Q for all variables. The reported `action_id` is the last variable, not the governing one.
- `sls_member_effects` (lines 180-241): characteristic (ψ0 on accompanying actions), frequent (ψ1 leader, ψ2 others), quasi-permanent (ψ2 for all). The maximum-utilisation-key selection is again heuristic.
- Proxies (lines 243-355): connections, cold-formed sheets and shells are wrapped as a synthetic `AluminiumMember` with `restrained_ltb = true`, `c1 = 1`, `support = SimplySupported`, and length = span or shell length. Their effects reuse the member path.

### 2.3 Clause tables (EN 1999-1-1, 1-2, 1-3) (`📏️parts/🦀️.rs`)

**EN 1999-1-1 (`part_1_1`, lines 359-630)**

- `CATALOGUE_ALLOY_ROWS` (lines 374-381): six rows, values as coded.
  - aw6060-t6: f_o 160, f_u 215 MPa, class A, ρ_o,haz 0.48, ρ_u,haz 0.56
  - aw6061-t6: 240 / 290, class A, 0.53 / 0.62
  - aw6063-t6: 170 / 215, class A, 0.49 / 0.56
  - aw6082-t6: 260 / 310, class A, 0.64 / 0.73
  - aw5083-o and aw5083-h111: 125 / 275, class B, ρ_haz = 1.0 / 1.0
- `resolve_alloy` (line 387): normalises case and hyphens and accepts `en-aw-6060-t6` as an alias.
- `epsilon` = √(250 MPa / f_o) (line 402).
- `eta_factor` (line 404): 0.70 if welded, else 1.0. This is applied in `beta` (line 406-409) as η·b/t.
- `classify_element` (lines 414-431): outstand limits 3 / 4.5 / 6 for classes 1 / 2 / 3, internal 11 / 16 / 22. Evaluated against β/ε.
- `local_buckling_rho` (lines 438-444): for class 4 only. β̄ = β/(ε·6) for outstand, β/(ε·22) for internal. ρ = 1/β̄ − 0.22/β̄², clamped to [0, 1]. This is the EN 1999 class-4 reduction form. The limits 6 and 22 are as coded.
- `section_geometry` (lines 446-490): A, I_y, I_z, W_el,y, W_el,z, I_t, I_w by kind.
  - tube: `A = π/4 (D² − d_i²)`, `I = π/64 (D⁴ − d_i⁴)`, `I_t = 2π((D−t)/2)³ t` (approximation)
  - box: standard rectangular-hollow formulas, with `I_t` as a thin-walled approximation
  - angle: `A = (b+h−t) t`, `I_t = (b+h−t) t³/3`
  - I/channel (default branch): `I_t = (2 b t_f³ + (h−2t_f) t_w³)/3`, `I_w = t_f b³/24 · ((h−t_f)/2)²`
  - The I_w expression is used for any non-tube, non-box, non-angle section, including channels, which is not correct for a channel.
- `effective_area` (lines 492-509): a_eff = Σ b·t·ρ over elements. When any element is welded, an HAZ factor `min(1 + weld_pos/width, 1.25)` divides `min(ρ_o,haz, ρ_u,haz)`, with a floor of 0.05. Fallback when a_eff ≤ 0 is `A · min(ρ_haz)`.
- `effective_wel_y` (lines 511-523): W_el,y times the **global minimum** of ρ across elements (including the same HAZ term). This replaces the EN 1999 effective-section procedure.
- `buckling_curve` (line 525): class A gives (α, λ̄0) = (0.20, 0.10), class B gives (0.32, 0.00). Used for both axes and for torsion and LTB (see 3.5).
- `chi_from_lambda` (lines 530-535): standard Φ/χ form with `χ ≤ 1` and `λ̄ ≤ λ̄0 → χ = 1`.
- `lambda_bar` (lines 537-543): λ̄ = (L/i)/(π√(E/f_o)).
- `torsional_lambda_bar` (lines 551-559): `N_cr,T = G·I_t / i_0` with `i_0² = (I_y + I_z)/A`. This is a simplified expression, with no warping or open-section term.
- `m_cr_ltb` (lines 562-567): `M_cr = C1 · π²EI_z/L² · √(I_w/I_z + L²GI_t/(π²EI_z))`. Standard doubly-symmetric form, with C1 only and no C2 or C3.
- `shear_area` (lines 569-580): tube/CHS → 2A/π; otherwise `(h − t_f)·t_w`.
- `bolt_fu_pa` (lines 582-590): 4.6 → 400, 5.6 → 500, 8.8 → 800, 10.9 → 1000 MPa. Other grades return 0 and fail with a remedy (see 3.6).
- `bolt_shear_resistance` (lines 592-598): `n · 0.6 · f_ub · A / γ_M2`. α_v is 0.6 for every grade.
- `bolt_bearing_resistance` (lines 600-609): `n · k1 · α_d · f_u · d · t / γ_M2`, with `α_d = min(e1/3d, p1/3d − 0.25, 1)` and `k1 = min(2.5, 2.8·e2/d − 1.7, 2.5)`. e2 is taken as gauge.
- `filler_fu_pa` (lines 611-618): 4043 → 190, 5356 → 240, 5183 → 270 MPa. Fallback is `min(parent_fu, 190)`.
- `haz_extent_m` (lines 620-623): `max(3·a, t) + haz_extent`. `weld_resistance` (lines 625-629): `a_w · f_w / (β_w · γ_M2)` with `a_w = throat · length`.

**EN 1999-1-2 (`part_1_2`, lines 632-643)**

- `k_theta` (lines 635-639): linear from 1.0 at 100 °C to 0.0 at 550 °C. This is a simplified curve, not the EN 1999-1-2 tabulated values.
- `critical_temperature_c` (lines 640-642): `170 + 0.4 · f_o[MPa]`. Used only for reporting.

**EN 1999-1-3 (`part_1_3`, lines 645-712)**

- Constants (lines 648-652): N_C = 2·10⁶, N_D = 5·10⁶, N_L = 10⁸, γ_Mf(DE) = 1.35, γ_Mf(EN) = 1.00.
- `detail_category_delta_sigma_c` (lines 655-674): 14 to 90 MPa (14, 18, 20, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90). "40-weld" is an alias for 40. This is a subset of Annex J rows, selected by the author.
- `fatigue_strength_pa` (lines 681-692): bi-linear S–N, m1 below N_D, m2 between N_D and N_L, constant below N_L cut-off.
- `damage_ratio` (lines 695-711): Miner sum `D = n / N_allow`, multiplied by `(1 + 0.02·m2/m1)`. The bracketed factor has no clause basis.

### 2.4 Member checks (`🧱️member-checks/`)

`check_member` (`🧱️member-checks/🦀️.rs` lines 91-163) runs, in order:

1. Alloy resolve (line 98). Failure returns a Fail with a one-of remedy over the six catalogue alloys (lines 99-117).
2. Geometry and class: `section_geometry`, `section_class` (max class over elements), `effective_area`, `effective_wel_y` with HAZ on when any element is welded.
3. Governing ULS effects from `governing_member_effects`.
4. Effective lengths (lines 139-147): k factors are Continuous 0.7, Cantilever 2.0, SimplySupported 1.0. `L_cr = k · buckling_length`, falling back to `member.length` when the buckling length is ≤ 0. `L_LT` falls back to `member.length`.

Then the individual checks, in order. Each emits a `CheckResult` through `utilization_check` (`🧱️member-checks/🦀️.rs` lines 56-88). Utilisation is `|computed| / |limit|`. If u > 1 and no remedy was given, a generic remedy is added (scale the length or the demand by 1/u, times 0.99).

| Check id prefix | Clause | Location | Formula / behaviour |
|---|---|---|---|
| `en1999.6.1.4.class.*` | §6.1.4 (Table 6.2) | `🧭️classification-and-axial-checks` 6-58 | One check per plate element. Limit β/ε ≤ 6 (outstand) or 22 (internal). Only the class 3 boundary is checked, not classes 1 and 2. Remedy: increase thickness to t_req = η·b/(limit·ε), plus alloy suggestions |
| `en1999.6.2.3.n.*` | §6.2.3 | `🧭…` 61-121 | N_Rd = A_eff · f_o / γ_M1 (line 78). Remedies list `sec-i160`, `sec-i180`, `sec-box120` (not in any catalogue, see 8) |
| `en1999.6.2.8.nm.*` | §6.2.8 | `🧭…` 124-163 | Linear sum N/N_Rd + M_y/M_Rd + M_z/M_Rd,z ≤ 1 (line 139). Not the EN 1999 interaction expression |
| `en1999.6.2.5.m.*` | §6.2.5 | `🔧️bending-and-shear-checks` 6-64 | M_Rd,y = W_eff,y · f_o / γ_M1 (line 21). W_eff uses the global ρ_min |
| `en1999.6.2.6.v.*` | §6.2.6 | `🔧…` 67-113 | V_Rd = A_v · f_o / (γ_M1·√3) with A_v from `shear_area`. Web-buckling factor `39ε/(h_w/t)` applied if h_w/t > 39ε (lines 84-87). Remedy lists `sec-i160`, `sec-i180` |
| `en1999.6.2.5.mz.*` | §6.2.5 | `🔧…` 116-171 | W_el,z from geometry (no ρ or HAZ reduction, line 130-138), M_Rd,z = W_el,z · f_o / γ_M1 |
| `en1999.6.2.6.vy.*` | §6.2.6 | `🔧…` 174-209 | Reuses the **z-axis** shear resistance `v_rd_b` (line 152) for V_y |
| `en1999.6.3.1.fb.*` | §6.3.1 | `🏛️buckling-and-interaction-checks` 6-116 | N_b,Rd = χ · A_eff · f_o / γ_M1 per axis. λ̄ uses `max(A_eff, A_gross)` (line 25), so λ̄ effectively uses gross area. Curve for z is the same as for y. Remedies bisect L_cr for χ_req (40 iterations) |
| `en1999.6.3.1.fb.z.*` | §6.3.1 | `🏛️…` 73-114 | Same for z, with bisection on L_cr,z and scaling of N_k |
| `en1999.6.3.1.lambda.y.*` | §6.3.1 | `🏛️…` 119-165 | λ̄_y ≤ 3 is reported as the limit (lines 139, 151-165). The limit 3 is a project heuristic, not a clause value |
| `en1999.6.3.1.lambda.t.*` | §6.3.1.4 | `🏛️…` 166-193 | λ̄_T ≤ 3 limit, same comment |
| `en1999.6.3.1.lcr.*` | §6.3.1 | `🏛️…` 194-208 | Reports ΣL_cr against 2·ΣL_cr, so utilisation is always 0.5 when L_cr > 0. **Always passes** |
| `en1999.6.3.1.tb.*` | §6.3.1.4 | `🏛️…` 212-264 | λ̄_T from `torsional_lambda_bar`, χ_T from the flexural curve, N_t,Rd = χ_T · A_eff · f_o / γ_M1. Remedy halves L_cr,T |
| `en1999.6.3.2.ltb.*` | §6.3.2 | `🏛️…` 267-328 | M_cr from `m_cr_ltb` (or infinite if `restrained_ltb`), λ̄_LT = √(W_eff·f_o/M_cr), χ_LT from the **flexural** (α, λ̄0) pair (line 293). No separate LTB curve, no k_c, no C2/C3 |
| `en1999.6.3.3.int.*` | §6.3.3 | `🏛️…` 331-376 | Linear sum N_Ed/N_b,Rd + M_Ed/M_b,Rd ≤ 1 (line 345). Not the EN 1999 k_yy/k_zy form |
| `en1999.7.2.defl.*` | §7.2 | `📉️serviceability-checks` 6-71 | w_qp from `q = 8·M_qp/L²` back-calculated from the quasi-permanent moment, then `w = 5qL⁴/(384EI_y)` (lines 20-22). Limit L/200 hard-coded (line 24). This is not a deflection from the load model |
| `en1999.7.2.stress.*` | §7.2 | `📉️…` 74-126 | Characteristic σ = M_y,char / W_eff ≤ 0.8·f_o (lines 86-87) |
| `en1999.7.2.sls-freq.*` | §7.2 | `📉️…` 129-178 | Frequent σ ≤ 0.8·f_o (line 140). Same hard-coded limit |

Member-check remedies also suggest section IDs `sec-i160`, `sec-i180`, `sec-i200`, `sec-box120`, `sec-chs168`, `sec-tube114` and alloys `aw6082-t6`, `aw6061-t6`, `aw6063-t6`. Most of these IDs do not exist in any model catalogue or example.

### 2.5 Connection checks (`🔗️connection-checks/🦀️.rs`)

`check_connection` (lines 6-85) first resolves the alloy (fail with one-of remedy if unknown). It then builds N_Ed, V_Ed and M_Ed from the governing ULS effects using a hard-coded lever arm of 0.50 m (line 41):

- `N_Ed = |N| + 0.25·(|M_y|+|M_z|)/0.50` (line 42)
- `V_Ed = |V_y| + |V_z| + (|M_y|+|M_z|)/0.50` (line 43)

These couple moments into the axial and shear demand in an ad hoc way. A missing member reference returns a Fail (lines 46-66). A connection of another kind returns not-applicable (lines 70-83).

- **Bolted** (`check_bolted_connection`, lines 89-216), applied when kind is `bolted` or `combined`:
  - Bolt grade must be in `bolt_fu_pa` (line 105), else Fail with one-of 4.6 / 5.6 / 8.8 / 10.9.
  - F_v,Rd and F_b,Rd from the helpers above. Combined resistance `F_Rd = F_v·F_b/(F_v+F_b) · pitch_factor` (line 130), with `pitch_factor = clamp(0.85 + 0.05·p/d, 0.80, 1.25)` (line 129). This is not the EN 1999 minimum-of-modes rule.
  - Pitch check `en1999.8.5.pitch.*` (§8.5.1): p1 ≥ 2.2·d (line 177).
  - Bolt check `en1999.8.5.bolt.*` (§8.5): V_Ed + |N_Ed| ≤ F_Rd (line 206-207). Remedies: raise bolts per row and rows, reduce V_z,k and N_k.
  - Edge distance e1 ≥ 1.2·d is only a remedy (line 169), not a separate check.
- **Welded** (`check_welded_connection`, lines 220-305), for `welded` or `combined`:
  - F_w,Rd = weld_resistance · ρ_haz/(1 + HAZ·10) (line 236). The `·10` is an unexplained constant. The ULS version has no clamp; the SLS version clamps to [0.2, 1.0] (line 337). The ULS remedy loop clamps to [0.2, 1.0] (line 246).
  - Remedy loop (lines 242-252) iterates up to 48 times to find a throat.
  - One remedy is `welds.hazExtent` → 0. Setting the stored HAZ extent to zero is a data edit, not a physical fix.
  - Check id `en1999.8.6.weld.*` (§8.6).
- **Serviceability** (`check_connection_serviceability`, lines 309-405):
  - Frequent-combination connection force F_Ed,freq ≤ min(bolt, weld) resistance (lines 320-372).
  - Check id `en1999.8.sls-freq.*` with clause label "EN 1990 §6.5.3" (line 376). The ψ1 combination comes from the proxy member path.

Not implemented for connections: net-section tension, block shear, bearing on the connected part beyond the bolt formula, weld throat-length rules beyond the one formula, edge distance e2 and gauge checks, bolt pretension, slip resistance, and rigidity.

### 2.6 Fire (`🔥️fire-fatigue-checks/🦀️.rs`, `check_fire` lines 6-120)

- θ from duration: `θ_t = 20 + 345·log10(1 + t/60)` (line 38). This is an approximation of the ISO 834 curve, not the EN 1999-1-2 procedure.
- Effective temperature: `θ_eff = θ_a + 0.25·max(0, θ_t − 20)` (line 39). The 0.25 weight is ad hoc.
- `k = k_theta(θ_eff)` (line 40), the linear curve from 2.1 above.
- Resistances use the **ambient** partial factor: `N_fi = k · A_gross · f_o / γ_M1` (line 43-45) and `M_fi = k · W_el,y · f_o / γ_M1`. No fire partial factor γ_M,fi is modelled. The member section is gross, with no local-buckling or HAZ reduction, and `W_el,y` is unreduced.
- Utilisation `u = max(N_Ed/N_fi, M_Ed/M_fi)` (line 47-49). No flexural buckling, no lateral-torsional buckling, no temperature gradient, no restraint, no thermal elongation.
- Demand: `fire_member_effects` with ψ2 on variables (see 2.2).
- Check id `en1999.1-2.fire.*`, clause §4.2 (line 102).
- Remedies: reduce duration to 0, lower θ_a to `550 − 450·k_req` clamped to [20, 550] (line 64), reduce N_k, M_y,k, g_k and q_k.
- Missing member, section or material returns Fail (line 176-203).

### 2.7 Fatigue (`🔥️fire-fatigue-checks/🦀️.rs`, `check_fatigue` lines 122-197)

- Δσ_C from `detail_category_delta_sigma_c` (line 124). Unknown category silently falls back to the stored `delta_sigma_c`.
- Member scaling (lines 125-133): `Δσ_Ed × (1 + |M_y,Ed| / 10⁶)` where M is in N·m. This amplifies the stress range by the bending moment in MN·m, which has no clause basis.
- Resistance `rd = (0.9·Δσ_R(N) + 0.1·Δσ_R(max(N, 1.01·N_D))) / γ_Mf` (line 136). The 0.9 and 0.1 blend is hard-coded.
- Damage `D = damage_ratio(...)` (line 137) is **reported** but the pass criterion is `Δσ_Ed > rd` (line 160). The Miner sum does not drive the verdict.
- Check id `en1999.1-3.fat.*`, clause §7.1 (line 143 and 179).
- Remedies: reduce Δσ_Ed to rd and suggest detail categories 71 / 80 / 90.
- Missing member reference returns Fail (lines 139-159).

### 2.8 Cold-formed sheeting (`🥫️thin-wall-checks/🦀️.rs`, `check_cold_formed` lines 6-272)

Checks, each as a `CheckResult`:

- Local buckling (§5.4): β = b/t, limit β/ε ≤ 22. ρ = β_limit/β (lines 35-39). This is a simplified reduction, not an effective-width procedure.
- Bending (§5.4): `b_eff = ρ·b`, `W = b_eff·t²/4` (in the bending block, around lines 36-41). **The rectangular plate modulus is bt²/6, so this expression is non-standard.** `M_Rd = W·f_o/γ_M1`.
- Axial (§6.1): `A_eff = b_eff·t·ρ_haz` (if welded), `N_Rd = A_eff·f_o/γ_M1`.
- Web crippling-like check (§6.1.5): `R_Ed = 2M/span` vs `R_Rd = 0.5·t²·f_o/γ_M1`. The formula has no clause expression.
- Shear (§6.1.7): `A_v = t·b` (full width, not web-only) and `V_Rd = A_v·f_o/(γ_M1·√3)`.
- N–M interaction (§6.1.4): linear sum ≤ 1.

Thickness remedies add a `1.05` safety factor. `wel` has no HAZ term in the bending check.

### 2.9 Shells (`🥫️thin-wall-checks/🦀️.rs`, `check_shell` lines 274-446)

- Critical stresses (classical-style): `σ_x,rcr = 0.605·E·t/r`, `σ_θ,rcr = 0.92·E·(t/r)·√(r/L)`, `τ_rcr = 0.75·E·(t/r)·(r/L)^0.75` (lines 310-312).
- Reduction parameters (lines 306-309): α = 0.62, β = 0.60, λ̄0 = 0.20, η = 1.0. Used identically for meridional and hoop.
- χ from the same Φ/χ expression as flexural buckling (lines 313-318).
- Resistances: `σ_x,Rd = χ_x · σ_x,rcr / γ_M1`, `σ_θ,Rd = χ_θ · min(σ_θ,rcr, f_o) / γ_M1`, `τ_Rd = τ_rcr / γ_M1` (no χ, no f_o cap).
- Demand: `governing_shell_effects` with n_k → σ_x, m_y_k → σ_θ, v_z_k → τ. Demand mapping is a convention, not a stress-resultant derivation.
- Check ids `en1999.1-5.buckle.*`, `en1999.1-5.ring.*`, `en1999.1-5.shear.*` (clause §5.3).

### 2.10 Structure evaluation (`🧾️structure-evaluation/🦀️.rs`)

`evaluate_structure` (lines 57-303) is the report entry point. Before the per-clause checks it runs:

- Duplicate-id checks for all eight collections (lines 61-68). Each duplicate yields a Fail with one-of suggestions (`-2`, `-b`, `-unique`, lines 18-27).
- Action-field validation (lines 71-74 and 306-386): kind must be in `permanent, imposed, snow, wind, temperature, fire`. Category must be in a fixed list including `self`, `office`, `snow`, `wind`, `residential`, `A`, `B`, `congregation`, `C`, `retail`, `D`, `storage`, `E`, `snow_high`, `temperature`, `fire` (line 308). Source must be `udl` or `external`. A permanent action must use category `self`, and a variable action must not (lines 351-384). Clauses cited: EN 1990 §6 (6.10) and §A1.2.2.
- Reference checks: member section and material (lines 91-115), connection member and material (lines 131-161), fire member, section and material (lines 176-203), fatigue member (lines 220-223), cold-formed material (lines 239-261), shell material (lines 278-300).
- Not-applicable placeholders (`en1999.1-1.empty`, `en1999.8.empty`, `en1999.1-2.empty`, `en1999.1-3.empty`, `en1999.1-4.empty`, `en1999.1-5.empty`) when a collection is empty (lines 76-89, 117-130, 162-175, 206-219, 225-238, 264-277).

**There is no check for**: annex-specific parameters beyond γ_Mf, whether the connection `kind` is one of the three allowed strings (only the lowercased comparison in `check_connection`), duplicate action ids across owners, material designation validity beyond the alloy lookup, or lengths and thicknesses being positive except in the cold-formed and deflection code.

---

## 3. NA-DE parameters actually used

| Parameter | Value | Where | DE vs EN |
|---|---|---|---|
| γ_M1 | 1.10 | `na_de::AnnexParams` (`📏️parts` lines 19-21) | identical |
| γ_M2 | 1.25 | same | identical |
| γ_Mf (fatigue) | 1.35 (DE), 1.00 (EN) | `part_1_3` lines 651-652, 676-678 | **only parameter that differs** |
| γ_G | 1.35 | `part_en1990::governing_member_effects` line 99 | identical, hard-coded |
| γ_Q | 1.50 | line 100 | identical, hard-coded |
| ξ | 0.85 | line 101 | identical, hard-coded |
| ψ0/ψ1/ψ2 snow | 0.5 / 0.2 / 0.0 | `psi_row` line 41 (dead `if de`) | identical |
| E | 70 000 MPa | `na_de::E_PA` line 8 | not annex-specific |
| ν | 0.33 | `na_de::NU` line 9 | not annex-specific |
| HAZ_ZONE_M | 0.025 | declared line 7 | **unused** |
| Deflection limit | L/200 | `📉️serviceability-checks` line 24 | not annex-specific, hard-coded |
| Stress limit | 0.8 f_o | serviceability lines 87, 140 | hard-coded |

The `de_and_en_gamma_identical` test (`🧬️schema/🧪️tests/⚖️compliance/✅️compliance-gates/🦀️.rs` line 27) confirms that all checks except fatigue give the same utilisation under DE and EN. So the NA-DE implementation is limited to γ_Mf. No DE-specific NDP values for χ, α, λ̄0, k_θ, the fire partial factor, or the classification limits are implemented. No NA-DE tables are present in the repo.

---

## 4. Editor (`✏️editor/`)

### 4.1 Commands (`✏️editor/🦀️.rs` lines 56-68, `app_commands!`)

| Command | Wire id | Behaviour |
|---|---|---|
| `evaluate` | `evaluate` | No-op: emits zero mutations, since the report is derived on every read |
| `setSelectedCheckIndex` | `selected-check` | View-only: sets the inspection tab's selected check index |
| `setField` | `set-field` | Resolves a path and JSON value through `EDIT_RULES` (16 rules, see 1.3), then dispatches a concrete kind mutation |
| `insertItem` | `insert-item` | Only `members` → `add-member` |
| `removeItem` | `remove-item` | Only `members` → `remove-member`, by row id |
| `applyRemedy` | `apply-remedy` | Delegated to the shared norm app surface (`semio_s_plugin_norm_app_surface`). Not inspected in depth here |

Set-field paths covered by rules: `materials[].designation`, `coldFormed` (whole list), `members[].actions[].mYK`, `annex`, `members[].actions[].nK`, `members` (whole list), `members[].bucklingLengthZ`, `sections` (whole list), `fatigueDetails` (whole list), `connections` (whole list), `fireScenarios` (whole list), `connections[].welds.throat`, `connections[].bolts.rows`, `materials` (whole list), `sections[].elements[].thickness`, `shells` (whole list).

Several paths replace an entire collection, so most edits are not granular. Editable paths with no rule (for example `fireScenarios[].thetaA`, `connections[].bolts.pitch`, `coldFormed[].thickness`) are only editable by replacing the whole collection.

### 4.2 Field metadata (`✏️editor/🏷️field-meta/🦀️.rs` lines 67-187)

A `TABLE` of about 110 `NormFieldMeta` rows gives English and German labels, SI units and optional choices (for `annex`, `kind`, `support`, `source`, `category`). Lookup is longest-prefix with `[]` wildcards (`lookup_norm_field_meta` in the app-surface `🔖️field-meta`). Covered paths include the snapshot fields listed in 1.2, plus `coldFormed[].actions[]` and `shells[].actions[]`. The `cfg(test)` test `field_meta_covers_every_editable_leaf_en_de` checks coverage.

### 4.3 Panels (`✏️editor/📌️panels/`)

- **Artifact** (`🗿️artifact/🦀️.rs`): `render_summary`. A headline of the form "complies / does not comply", plus a row per part with ✓ or ✗ (`📄️ artifact` panel, document tab).
- **Catalogue** (`📚️catalogue/🦀️.rs`): example picker (two examples, `aluminium-roof-purlin` and `noncompliant-multi-fail`) and one reference table, **EN 1999-1-1 Table 3.2** with six rows (designation, f_o, f_u, ρ_o,haz, ρ_u,haz). Each table section is windowed.
- **Inspection** (`🔍️inspection/🦀️.rs`): `render_inspection` shows one check in full: id, part, subject, status, utilisation, explanation and remedies, for the index chosen by `setSelectedCheckIndex`.

### 4.4 Windows (`✏️editor/🎭️modes/✏️edit/🪟️windows/`)

- **Inputs** (`📥️inputs/🦀️.rs`, body `norm.en1999.play.inputs`, icon `download`): calls `render_document_editor` with the field-meta function. `render_document_editor` → `render_value_editor` walks the document's value tree and renders one tree row per leaf, with an editable control bound on commit to `setField`. Labels and units come from field-meta. **This is a structured property tree, not raw JSON.** The legacy `render_document_json` exists in the app surface and is not used by this editor.
- **Results** (`📊️results/🦀️.rs`, body `norm.en1999.play.results`, icon `bar-chart-3`): `render_report` shows a grouped tree. Each part is a row with its verdict, and each check is a child row with status, utilisation, title and remedies.

No 2D or 3D rendering, no curves, no tables for the editor, no drag-and-drop, and no live viewport exist. `🎚️config`, `👥️presence`, `🫧️transient`, `🎬️actions`, `🪛️utilities` and `☑️options` under the editor modes are placeholder `📌️.empty.md` files.

### 4.5 Editor example

`📚️examples/demo-session` contains `🖼️assets/🎮️.cmd.semio` with two lines (`semio norm.en1999.cmd v1`, `action=demo`). It is a stub and does not exercise the editor.

---

## 5. Viewer (`👁️viewer/`)

- One window kind, `📊️report` (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`). Its `render(document, locale)` calls `evaluate(document)` on every render and passes `report_table_columns` and `report_table_rows` to `TableWindowKit::render`.
- Columns (app-surface `🏷️ids-chrome-and-labels` lines 49-59): Part, Clause, Subject, Status, Utilization, Title, Remedy.
- Read-only: the viewer never declares an editable command.
- The viewer imports `semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit}` with an explicit `SDK GAP` comment (line 9-10). `Dialect` is also re-exported from `app` (`👁️viewer/🦀️.rs` lines 20-21).

Nothing else is shown: no per-check detail, no diagrams, no 3D.

---

## 6. Tests and oracles

### 6.1 Rust tests (64 `#[test]` functions in `SUB`)

Grouped by location:

- **Compliance** (`🧬️schema/🧪️tests/⚖️compliance/`): 37 tests. Examples: `compliant_example_passes`, `noncompliant_example_has_multiple_fails_with_remedies`, `de_and_en_gamma_identical`, `remedy_paths_use_id_selectors_and_resolve_in_snapshot`, `chi_never_read_from_snapshot`, `bundled_dsl_parses_and_evaluates_compliant`, `python_oracle_matches_rust_utilizations_within_half_percent`, `chi_from_lambda_class_a_worked`, `alloy_table_3_2_6082_t6`, `m_c_rd_6082_hand`, `fatigue_strength_at_5e5`, `fire_k_theta_curve`, `shell_chi_from_geometry_hand_value`, `dangling_connection_member_id_fails_with_oneof_existing_members`, `gate_parity_every_fail_remedy_clears_status`, `facet_diff_matches_rust`, `explanations_en_de_not_identical_except_numbers`, etc.
- **Inferences** (`💡️inferences/🧪️tests/🔬️compliance`): 2 tests. Outline (`🧾outline/🧪️tests/🔬️unit`): 1.
- **Mutations** (`🧬️mutations/🧪️tests/`): 2 unit tests plus a kinds-catalog test.
- **SQLite snapshot** (`🚪️io/🪶️sqlite/📸️snapshot/🧪️tests`): about 15 tests, including round-trips, out-of-range admission, and exact-width column limits.
- **Text and binary IO**: a few tests (`a_mutation_crosses_the_binary_op_codec_unchanged`, `default_snapshot_has_members`).
- **Editor, panels and viewer**: `editor_module_links` (repeated), `renders_reference_tables_with_examples`.
- **Examples**: `example_snapshot_is_realistic`.

Most tests are structural or pinned to hand-calculated values. Coverage gaps: no tests for classification class-boundary values for classes 1 and 2, LTB curve, interaction factors, connection SLS, or fire k_θ table points beyond the linear curve.

### 6.2 Python oracles

**A. Compliance oracle (`🔮️oracles/⚖️compliance/🐍️.py`, 413 lines).**

- Imports: `json`, `math`, `sys` only (stdlib). No third-party library.
- Hard-coded constants (lines 6-21): E, γ_M1, γ_G, γ_Q, ξ, N_C/N_D/N_L, γ_Mf (DE/EN), and an `ALLOYS` dict with the same six alloys as Rust. These are copies, not independent sources.
- `evaluate(doc)` (line 192) emits checks for:
  - `en1999.3.2.alloy.*` (utilisation hard-coded to 1.0 for unknown alloy)
  - `en1999.6.2.3.n.*` and `en1999.6.2.5.m.*`
  - `en1999.7.2.defl.*` and `en1999.7.2.stress.*`
  - `en1999.8.5.bolt.*` and `en1999.8.6.weld.*`
  - `en1999.1-2.fire.*`
  - `en1999.1-3.fat.*`
  - `en1999.1-4.{local,bend,axial,nm,shear}.*`
  - `en1999.1-5.{buckle,ring,shear}.*`
- **Not covered by the oracle**: classification (§6.1.4), shear (§6.2.6), N–M (§6.2.8), flexural, torsional and LTB buckling (§6.3.x), interaction (§6.3.3), connection SLS, and the shear-y check.
- Invoked by `🧬️schema/🧪️tests/⚖️compliance/✅️compliance-gates/🦀️.rs` lines 131-175, which spawns `python3` via `std::process::Command`. It compares checks by id: relative error ≤ 0.5 % **or** absolute error ≤ 0.005. Checks with no match are silently skipped (`continue`), and the test requires at least two overlaps (`compliance-gates/🦀️.rs` lines 131-175).
- Manifest `🔮️oracles/🔣️.json`: `"package": ""`, `"kind": "verified-native-second-implementation"`, `"productionReachable": false`, `"license": "AGPL-3.0-only"`. The rationale states that PyPI has no `en1999`, `eurocode`, `vdi3805` or `iso16757` distribution.

**B. Mutation oracle (`🧪️tests/🪶️mutate-en1999-1/`)**.

- `🐍️.py` (92 lines) imports `semio_norm_vocabulary` from the shared engine at `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`, which lives outside the artifact. The artifact file only supplies data.
- `🥒️.feature` (106 lines) has one Scenario Outline over 18 `✅apply` vectors, plus the `⛔dupe` refusals, `inverse-` and `identity-round-trip` rows. The identity row reads a `.dsl.semio` carrier that has no published grammar. The comparison therefore stops at envelope, lines and digest.
- `🦀️.rs` (399 lines) is the Rust driver. Its tests run the shared vectors through both implementations.

### 6.3 Third-party libraries

- **Rust runtime**: none. `Cargo.toml` depends only on workspace `semio-*` crates.
- **Rust test-only**: `serde`, `serde_json` under `cfg(test)`. The app-surface oracle `🧪️tests/🔬️retained-disposition-oracle` also uses `serde_json`, but that is outside this artifact.
- **Python**: no third-party packages. The oracles use only the standard library.
- **External references**: none in `SUB`. The brief's `structuralcodes`, `concreteproperties`, `anastruct` are mentioned only in the manifest rationale as examples that were rejected.

---

## 7. Examples and fixtures

**Examples (`📚️examples/`)**

- `🏠️aluminium-roof-purlin`: Rust source (`🦀️.rs`) calls `compliant_roof_purlin()`. Assets: `🖼️assets/📡️aluminium-roof-purlin.spr.semio` (158 bytes, `norm.en1999.spr v1`, empty body) and `🖼️assets/🔧️aluminium-roof-purlin.op.semio` (61 bytes, `edit reuse started="0"`). These are stubs.
- `🏚️noncompliant-multi-fail`: Rust source calls `noncompliant_multi_fail()`.
- Editor example `demo-session` (`✏️editor/📚️examples/🎬️demo-session`): two-line stub (`action=demo`).

**Assets (`🖼️assets/`)**

- `🏠️aluminium-roof-purlin/`: `🎒️.pack.semio`, `🏠️aluminium-roof-purlin/🔣️.json`, `🏠️aluminium-roof-purlin/🗣️.dsl.semio`. The DSL is parsed and encoded in the `bundled_dsl_parses_and_evaluates_compliant` test.
- `🏚️noncompliant-multi-fail/`: the same three file types.

**Mutation fixtures (`🧪️tests/🧫️fixtures/🧬️mutations/`)**

- 18 `✅apply` vectors, each with `🎯️outcome`, `📸️snapshot/⬅️before`, `📸️snapshot/➡️after`, `🔺️diff` and `🦠️mutation` JSON. Kinds: change-material-designation, change-cold-formed, add-member, remove-member, change-member-my-ed, change-annex, change-member-n-ed, change-members, change-member-buckling-length, change-sections, change-fatigue-details, change-connections, change-fire-scenarios, change-weld-throat, change-bolt-count, change-materials, change-plate-thickness, change-shells.
- 2 refusal rows: `➕add-member/⛔dupe` and `➖remove-member/🧭️middle`.

**Other fixtures**

- `🧪️tests/🧫️fixtures/🔬️check-report` (compliance report fixture).
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json` (209 lines) and `🎛️semantic.json`.
- `🧬️schema/🎲️field-metadata.json` (110 lines) mirrors field-meta.
- `🔮️oracles/🔣️.json` (665 lines) is the oracle manifest.

---

## 8. Gaps, stubs, TODOs, hard-coded values, ad hoc shortcuts

Ranked by impact on norm completeness.

**Clause-level gaps and simplifications**

1. **Interaction formulas are linear sums, not the EN 1999 expressions.** Affected: N–M (§6.2.8, `classification-and-axial-checks` 124-163), member interaction (§6.3.3, `buckling-and-interaction-checks` 331-376), cold-formed N–M (`thin-wall-checks` 216-272), connection F_Rd combination (`connection-checks` line 130).
2. **Effective section is a global minimum ρ.** `effective_wel_y` (`part_1_1` lines 511-523) and `effective_area` apply one ρ_min or summed ρ per element, not the EN 1999 effective-section procedure.
3. **Classification checks only the class 3 boundary.** `check_classification` compares only against 22 (internal) and 6 (outstand). Class 1 and 2 limits (Table 6.2) are not checked, and the class output is never used as a verdict.
4. **Flexural buckling uses gross area for λ̄.** `lambda_bar` gets `max(a_eff, a_gross)` (`buckling-and-interaction-checks` line 25, 26).
5. **Same buckling curve for both axes, torsion and LTB.** The α/λ̄0 pair from `buckling_curve` is used for y, z, T and LTB. There is no separate LTB curve, no α_LT, and no k_c.
6. **Shear for V_y reuses the V_z resistance.** `check_shear_y` (`bending-and-shear-checks` line 152).
7. **Weak-axis bending has no local-buckling reduction.** `check_bending_z` uses unreduced W_el,z (lines 130-138).
8. **Shear web-buckling factor `39ε/(h_w/t)` is ad hoc.** `check_shear_z` lines 84-87.
9. **Bolt shear α_v = 0.6 for all grades.** `bolt_shear_resistance` (`part_1_1` line 596). A grade-dependent α_v is not implemented.
10. **Bolted combined resistance is a harmonic combination with pitch factor.** `F_v·F_b/(F_v+F_b)·clamp(0.85+0.05p/d)` (`connection-checks` 129-130).
11. **Connection forces include moment-to-force conversion with lever 0.50 m.** `connection-checks` lines 41-43.
12. **Weld resistance HAZ term uses `·10` and two different clamps.** `connection-checks` 236, 246, 337.
13. **Deflection from back-calculated UDL.** `q = 8M/L²` (`serviceability-checks` line 21) is not a load-based deflection.
14. **SLS stress and deflection limits are hard-coded.** 0.8·f_o (lines 87, 140) and L/200 (line 24).
15. **Fire model is simplified in four ways.** Linear k_θ (`part_1_2` 635-639), ISO-like θ(t) with a 0.25 blending (`fire-fatigue-checks` 38-39), gross section with no HAZ, and γ_M1 used in place of a fire partial factor. Also no buckling in fire.
16. **Fatigue verdict ignores Miner damage.** Verdict is `Δσ_Ed > rd` (`fire-fatigue-checks` 160), while D is reported only (137).
17. **Fatigue member scaling is ad hoc.** `1 + |M_y|/10⁶` (`fire-fatigue-checks` 129), 0.9/0.1 blend (136), and `(1 + 0.02·m2/m1)` in `damage_ratio` (`part_1_3` 710).
18. **Cold-formed bending section modulus is non-standard.** `W = b_eff·t²/4` (`thin-wall-checks` ~40). Rectangular plate modulus is bt²/6.
19. **Cold-formed shear uses full width.** `A_v = t·b` (`thin-wall-checks` ~183-186).
20. **Cold-formed local buckling ρ is ad hoc.** `ρ = β_limit/β` (`thin-wall-checks` 35-39), not an effective-width formula.
21. **Shell critical stresses and reduction parameters are hard-coded.** `0.605`, `0.92`, `0.75`, `α=0.62`, `β=0.60`, `λ̄0=0.20`, `η=1.0` (`thin-wall-checks` 306-312). Source not cited in code.
22. **Torsional λ̄ uses an open-section approximation.** `torsional_lambda_bar` (`part_1_1` 551-559). No warping term and no checks for closed-section torsion.
23. **LTB M_cr uses C1 only.** No C2, C3, shear-centre or load-height terms.
24. **Shear-buckling and web slenderness use element width, not clear-web height.** `check_shear_z` line 84.
25. **Connections omit net-section, block shear, edge distance e2, gauge and slip checks.**

**Always-passing or non-binding checks**

26. `en1999.6.3.1.lcr.*` always reports utilisation 0.5 (`buckling-and-interaction-checks` 194-208). It cannot fail.
27. `en1999.6.3.1.lambda.*` and `lambda.t` use λ̄ ≤ 3 as a limit. This is a project heuristic, not a clause limit.
28. Fatigue `D` is computed and reported but does not drive the verdict.
29. Connection edge distance e1 ≥ 1.2·d is only a remedy, not a utilisation check.
30. `en1999.8.na.*` (not-applicable) fires when a connection's kind is neither `bolted`, `welded` nor `combined`. Because the earlier checks are gated on `kind` strings, an unknown kind passes silently as n/a instead of failing (`connection-checks` 70-83).
31. The Python oracle reports 1.0 utilisation for unknown alloys, and the Rust test silently skips unmatched check ids (`compliance-gates/🦀️.rs` lines 131-175).

**Hard-coded values and dead code**

32. γ_G 1.35, γ_Q 1.50, ξ 0.85 (`parts` 99-101). Not annex-specific.
33. `HAZ_ZONE_M` declared and unused (`parts` line 7).
34. `psi_row` snow branch is a dead `if de` (`parts` line 41).
35. `is_non_tube_od` duplicates `is_zero_f64` (snapshot line 323).
36. Remedy section IDs `sec-i160`, `sec-i180`, `sec-i200`, `sec-box120`, `sec-tube114`, `sec-chs168` are hard-coded and not in the model catalogue. Only `sec-i120`, `sec-chs168` and `sec-weak` exist in the snapshot fixtures.
37. Remedy `welds.hazExtent → 0` (`connection-checks` 276-281) is a data-edit suggestion with no physical basis.
38. `default_plate` sets `weld_position = 0.005` when welded (snapshot line 326). `default_bolts` and `default_welds` (snapshot 329-344) are literal defaults.
39. Bolt grade fallback `_ => 0.0` in `bolt_fu_pa`. Unknown grades become Fail (`connection-checks` 105), which is correct, but the list of allowed grades (remedy line 121) is duplicated in code.
40. Alloy catalogue has six rows only. Table 3.2 coverage is not complete. Duplicate HAZ data sits in the Python oracle `ALLOYS` (lines 14-21).
41. Fatigue detail catalogue is a subset of Annex J (15 rows). Unknown categories fall back silently to the stored Δσ_C (`fire-fatigue-checks` 124).

**Stubs and placeholders**

42. 27 `📌️.empty.md` placeholders in the subset for editor config, presence, transient, actions, utilities and options.
43. `✏️editor/📚️examples/🎬️demo-session` is a two-line stub.
44. `📚️examples/*/🖼️assets/*.spr.semio` and `.op.semio` are stubs (158 and 61 bytes).
45. The `inferences` outline is trivial: it reports counts, not compliance-relevant inferences. Its `reads` list omits `coldFormed` and `shells`, and its `entry_count` omits them too (`🧾outline/🦀️.rs` lines 26-37).
46. `🧬️schema/🧪️tests/⚖️compliance/🧭️path-helpers` and the oracle test require `python3` on PATH. This is not cross-platform (native Windows commonly has `py` or `python`), which conflicts with the repo's zero-touch rule.
47. Editor and viewer use `SDK GAP` comments (`✏️editor/🦀️.rs` 42; `👁️viewer/🦀️.rs` 20-21; `👁️viewer/.../📊️report/🦀️.rs` 9-10). `Dialect`, `WindowKit`, `TableWindowKit`, `TableView` are reached through the `app` module.
48. `💡️inferences/🦀️.rs` declares a `FacetLeaves` for the inference schema that includes `🔗️.graphql`, `🔣️.json`, `🛰️.proto` and `🟦️.ts`. Those leaves were not cross-checked in this audit.

**UI and interaction gaps**

49. No diagrams: no M-N interaction curve, no stress block, no S-N curve, no spectra, no 3D members, sections, joints, load paths or utilisation heat maps.
50. No direct manipulation. The inputs window is a property tree with commit-on-enter rows.
51. Editing is mostly whole-collection replacement for sections, members, connections, fire, fatigue, cold-formed, shells, materials and annex (see 4.1). Remedies are presented as text and can be applied through `applyRemedy`, but the viewport feedback is only tree rows.
52. Multi-language: all labels have EN and DE pairs, and tests check EN and DE explanations differ except for numbers. Good.
53. Only the report window is available in the viewer. No per-check card, no comments.

**Fleet brief reconciliation**

54. The brief says editors "render the document as JSON through `render_document_editor`". In the current code `render_document_editor` renders a structured value tree (`render_value_editor`), not JSON. The legacy `render_document_json` exists and is not used by EN 1999.
55. The brief lists "member buckling with ηc". The code has no ηc or η_c term. η = 0.70 appears only as the welded-element factor in β (`part_1_1::eta_factor`).
56. The brief lists "HAZ softening ρo,haz/ρu,haz". These are implemented: `CATALOGUE_ALLOY_ROWS` plus the HAZ factor in `effective_area`, weld resistance and connection checks.
57. The brief lists "1-4 cold-formed sheeting" and "1-5 shells". Both are implemented as described in 2.8 and 2.9, with the simplifications noted above.
58. The brief lists "fire" (1-2) and "fatigue" (1-3). Both are implemented in simplified form (2.6, 2.7).
59. `🚪️io` is SQLite, text (`.dsl.semio`, `.grammar.semio`, `.ebnf`, `.g4`), binary (`.ksy`, `.spicy`, `.abnf`) and protocol definitions. Not inspected in depth.

---

## 9. Summary of checks by EN 1999 part

| Part | Check ids | Status |
|---|---|---|
| EN 1999-1-1 §3.2 | `en1999.3.2.alloy.*` | Implemented (6 alloys) |
| EN 1999-1-1 §6.1.4 | `en1999.6.1.4.class.*` | Class 3 boundary only |
| EN 1999-1-1 §6.1.5 | ρ_c in `local_buckling_rho` and `effective_area` | Implemented, class 4 only |
| EN 1999-1-1 §6.2.3 | `en1999.6.2.3.n.*` | Implemented, gross/effective area |
| EN 1999-1-1 §6.2.5 | `en1999.6.2.5.m.*`, `.mz.*` | Implemented, global ρ |
| EN 1999-1-1 §6.2.6 | `en1999.6.2.6.v.*`, `.vy.*` | Implemented, simplified web factor |
| EN 1999-1-1 §6.2.8 | `en1999.6.2.8.nm.*` | Linear sum only |
| EN 1999-1-1 §6.3.1 | `fb`, `fb.z`, `lambda.y`, `lambda.t`, `lcr`, `tb` | Implemented, `lcr` always passes |
| EN 1999-1-1 §6.3.2 | `en1999.6.3.2.ltb.*` | Flexural curve reused, C1 only |
| EN 1999-1-1 §6.3.3 | `en1999.6.3.3.int.*` | Linear sum only |
| EN 1999-1-1 §7.2 | `defl`, `stress`, `sls-freq` | Hard-coded limits, back-calculated deflection |
| EN 1999-1-1 §8.5 | `pitch`, `bolt`, `boltgrade` | Implemented, α_v fixed, combined F_Rd heuristic |
| EN 1999-1-1 §8.6 | `weld` | Implemented, HAZ heuristic |
| EN 1999-1-2 §4.2 | `en1999.1-2.fire.*` | Simplified k_θ and θ(t), no γ_M,fi |
| EN 1999-1-3 §7.1 | `en1999.1-3.fat.*` | Bi-linear S–N, verdict not Miner |
| EN 1999-1-4 §5.4, §6.1.x | `local`, `bend`, `axial`, `shear`, `nm` | Simplified, W = bt²/4 |
| EN 1999-1-5 §5.3 | `buckle`, `ring`, `shear` | Hard-coded parameters |
| EN 1990 §6.10 / §6.5.3 / §A1.2.2 | `ref.action.*`, sls-freq | Action validation and ULS/SLS combinations |
| Structure | `ref.*`, `*.empty`, duplicate ids | Implemented |

---

*Audit complete. Nothing in `SUB` was modified; this file is the only write.*
