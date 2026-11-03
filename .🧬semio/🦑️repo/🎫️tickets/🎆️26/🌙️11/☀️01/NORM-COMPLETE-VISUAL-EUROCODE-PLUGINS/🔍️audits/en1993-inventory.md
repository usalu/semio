# 🔩 EN 1993 Inventory

Read-only audit of the EN 1993 artifact as it exists in the working tree. No source file was modified.

Root: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993`
Subset: `🏅️standards/🔖️1/🪆️subsets/✳️any` (all paths below are relative to this subset unless marked otherwise).

Size: 963 tracked files, of which 936 are non-placeholder. Rust source (including tests, excluding fixtures) is about 12.7k lines. Generated JSON schemas and oracle manifests add about 11k lines. Test fixtures are 405 files.

Short verdict: the artifact has a real, well-structured clause engine for a subset of EN 1993 (members in compression, bending, LTB, M+N, SLS deflection, bolts, fillet welds, Miner fatigue, simplified fire, cold-formed, plates, shells, tower, pile, crane, bridge). It is not norm-complete. Many verifications are simplified or ad hoc, and several look non-conservative or wrong. The editor and viewer are tree/table UIs only, with no 2D or 3D graphics. The only external-library validation is a Python `jsonschema` check. No third-party library validates any clause formula.

---

## 1. Model

### 1.1 Artifact and snapshot

- Artifact state: `🧬️schema/🦀️.rs:23-60`, `En1993Artifact`. It has 18 `#[state(artifact)]` fields that mirror the snapshot. `to_snapshot` and `from_snapshot` are at lines 64-110. The descriptor is at line 113 and carries rust, ts, graphql, json-schema and proto leaves, plus snapshot, diff and mutation leaves.
- Snapshot: `🧬️schema/📸️snapshot/🦀️.rs:23-74`, `En1993Snapshot`, dsl id `norm.en1993`, document schema `semio.norm.en1993/v1`. `annex: AnnexChoice` (En|De) plus 17 table fields. `Default` is `compliant_heb240_frame()` (line 76).
- Example constructors: `compliant_heb240_frame` (line 84, DE annex, HEB 240 S355 frame exercising parts 1-6), `noncompliant_overloaded_frame` (line 276, same frame with overloaded actions, Lcr 12 m, plastic analysis, etc.).
- Catalogue: `catalogue_heb240` (348), `catalogue_heb260` (374), `catalogue_heb280` (400), `catalogue_heb300` (426), `rolled_heb_catalogue` (452). These are the only rolled sections; all remedies choose from them.

Every struct derives `DslRecord`, `ToValue`, `FromValue`, `CanonicalJsonTree` and `RetireOwned`. All SI units: Pa, m, m², m³, m⁴, N, N·m. All `kind`, `type`, `method` and `rating` fields are plain `String`s. The only enums are `AnnexChoice` (from the compliance crate) and the internal `BucklingCurve`, `FireRating`, `AssessmentMethod`.

### 1.2 Entities (snapshot, `🧬️schema/📸️snapshot/🦀️.rs`)

| Entity | Lines | Fields (type) | Notes |
|---|---|---|---|
| `SteelMaterial` | 468-482 | id, grade, fy, fu, e_modulus, g_modulus (f64), subgrade (String, e.g. J2/K2), kind (String: carbon, stainless) | Table `materials` |
| `SteelSection` | 485-511 | id, designation, kind, h, b, tw, tf, r, area, shear_area_y, shear_area_z, iy, iz, it, iw, w_el_y, w_el_z, w_pl_y, w_pl_z, area_net (f64) | kind: rolledI, weldedI, hss (field-meta) |
| `SteelMember` | 514-535 | id, label, member_type (beam, column, beamColumn, brace, tie), section_id, material_id, length, buckling_length_y, buckling_length_z, ltb_length, ltb_restraint_spacing, load_application (shearCenter, topFlange, bottomFlange), end_moment_ratio_psi, moment_diagram (uniform, parabolic, linear), deflection_limit_ratio, analysis (elastic, plastic) | |
| `DesignAction` | 538-550 | n, vy, vz, my, mz, t (f64) | value type, not a table |
| `LoadCase` | 553-562 | id, name, kind (permanent, prestress, imposed, wind, snow, accidental, self), category (office, residential, congregation, shopping, storage, snow, snow_high, wind, temperature, self, permanent, accidental) | Psi factors are looked up by category |
| `MemberAction` | 566-577 | id, member_id, load_case_id, action: DesignAction | |
| `ForceAction` | 579-589 | id, load_case_id, force (f64) | Documented as N. Plated panels reuse it as stress (see gaps) |
| `JointForceAction` | 591-602 | id, load_case_id, shear, tension (f64) | |
| `FatigueBand` | 604-614 | id, delta_sigma, cycles (f64) | Spectrum row |
| `SteelJoint` | 616-648 | id, kind (bolted, welded), member_id, bolt_class (String), bolt_diameter (f64), bolt_rows, bolts_per_row, shear_planes, friction_surfaces (u32), pitch, gauge, end_distance, edge_distance, plate_thickness, plate_fu, weld_throat, weld_length, weld_fu, weld_grade (String), actions: Vec<JointForceAction>, category (String, A/B/C), friction_mu, preload_force, slip_factor_ks | |
| `FatigueDetail` | 650-663 | id, member_id, category (u8), method (damage_tolerant, safe_life, low_consequence), spectrum: Vec<FatigueBand> | |
| `FireExposure` | 665-682 | id, member_id, rating (r30, r60, r90, r120), protection_thickness, section_factor (Am/V), mu0, design_temperature, protection_conductivity, protection_density, protection_specific_heat | |
| `ColdFormedMember` | 684-700 | id, b_bar, thickness, k_sigma, psi, fy, gross_resistance, actions: Vec<ForceAction> | gross_resistance is an input, not computed |
| `PlatedPanel` | 702-717 | id, a, b, thickness, fy, k_sigma, actions: Vec<ForceAction> | |
| `SiloShell` | 719-733 | id, thickness, radius, depth, k, gamma, fy | Not a table, plain entity |
| `TensionComponent` | 735-747 | id, f_uk, f_k, actions: Vec<ForceAction> | |
| `BridgeFatigue` | 749-763 | id, member_id, lambda, phi2, delta_sigma_p, category (u8), method | |
| `TowerLeg` | 765-778 | id, member_id, force_coefficient, dynamic_factor, actions | |
| `SteelPile` | 780-795 | id, section_id, material_id, driving_stress, embedded_length, shaft_perimeter, actions | |
| `CraneRunway` | 797-808 | id, member_id, wheel_contact_length, dispersion, web_thickness, fy, phi, actions | |

There are no enums for these string fields. Unknown strings fall back silently (see gaps).

### 1.3 Other model-side artifacts

- Inference outline: `🧬️schema/💡️inferences/🧾outline/🦀️.rs:37`, `En1993Outline` (section_outline, field_count, entry_count). It is a structural outline, not a clause result.
- Inference entry: `🧬️schema/💡️inferences/🦀️.rs:77-78`, `evaluate(document) -> CheckReport`, which calls `check_full_steel_structure`. This is the single pure entry point used by the editor and viewer.
- Diff: `🧬️schema/🔺️diff/🦀️.rs` (392 lines), "sparse scalar fields plus keyed row deltas for every list".
- Mutations: `🧬️schema/🧬️mutations/🦀️.rs:76-124`, enum `En1993Mutation` with 49 variants:
  - `ChangeAnnex` (1)
  - `Update*Inputs` (16): UpdateMemberProperties, UpdateFireInputs, UpdateColdFormedInputs, UpdateStainlessInputs, UpdatePlatedInputs, UpdateSiloShellInputs, UpdateBoltInputs, UpdateWeldInputs, UpdateFatigueInputs, UpdateThroughThicknessInputs, UpdateTensionComponentInputs, UpdateHssInputs, UpdateBridgeInputs, UpdateTowerInputs, UpdatePileInputs, UpdateCraneInputs
  - `Insert*` / `Remove*` pairs (16 each): material, section, member, load case, member action, joint, fatigue detail, fire exposure, cold-formed member, plated panel, silo shell, tension component, bridge fatigue, tower leg, pile, crane runway
  - Mutation edit rules: `EDIT_RULES.resolve` (used by all commands).
- Presence: `👁️viewer/👥️presence/🧬️schema/`, `✏️editor/👥️presence/🧬️schema/`. Generated schema leaves only; no presence behaviour.

### 1.4 Persistence and IO (`🚪️io`)

- `🚪️io/🦀️.rs:1-19`: no stdio format bridges. `import_stdio_kinds` and `export_stdio_kinds` return empty lists. Earlier csv, json, txt, xlsx and zip leaves were deleted.
- Text DSL: `🚪️io/📝️text/📸️snapshot/🦀️.rs`, `parse_dsl` (line 20), `print_dsl` (25), `encode_en1993_snapshot_json` (49), `decode_en1993_snapshot_json` (60), `decode_en1993_dsl` (68), `encode_en1993_dsl` (76). Carrier `.dsl.semio`.
- Binary pack: `🚪️io/💾️binary/📸️snapshot/🦀️.rs`, `encode` / `decode` through `pack::snapshot::ArtifactPack` (framework), plus an independent codec (`native_snapshot_codec`). Specs in `📡️.protocol.semio`, `🥋️.ksy`, `🌶️.spicy`, `🔠️.abnf`.
- SQLite: `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql` (73 lines). A single `en1993_document` row with CHECK annex IN ('En','De'), then one table per entity and child tables for actions, bands and spectra. Codec at `🪶️sqlite/📸️snapshot/🦀️.rs:42`.
- The text carrier has no formal grammar. The feature file states that the committed grammar is the repository-wide placeholder `payload = OCTET+`.

---

## 2. Checks and verifications

### 2.0 Orchestration

- Entry: `⚖️checks/🦀️.rs` (127 lines). `AnnexParams` (lines 22-63) with `en()` and `de()`. Constants at lines 32-45.
- Driver: `🧱️member-checks/🦀️.rs:14-110`, `check_full_steel_structure`.
  - Runs referential integrity first (line 18), then an empty-subject check (lines 20-46).
  - For each member (lines 48-105): section and material lookup, member kind, section class, effective section properties for class 4, radii of gyration, buckling curves, λ̄ and χ, N_b,Rd, M_cr, λ̄_LT, χ_LT, M_b,Rd, combinations, then ULS checks, then SLS.
  - Calls `check_special_elements` (line 107) for joints, fatigue, fire, cold-formed, stainless, plated, shells, bridge, tower, pile and crane.
- Member kind gating (`🧰️structure-helpers/🦀️.rs:67-90`):
  - flexural buckling: not beam or tie
  - LTB: not column, brace or tie
  - bending: not brace or tie
  - compression: not tie
  - member interaction: beamColumn or column (or empty)
- Load combination and selection: `🧰️structure-helpers/🦀️.rs`.
  - Scalars: `combine_scalar_forces` (92-132)
  - Member actions: `combine_member_actions` (146-219)
  - Governing selection: `governing_scalar` (134-136) uses max |value|. `governing_effects` (221-227) uses max of |N| + |My| + |Vz| over the ULS rows.
  - Governing rule: `check_uls_actions` (`🧮️action-resistance-checks/🦀️.rs:36`) takes `governing_effects(...).into_iter()`, so **only one ULS row per member is checked**.

### 2.1 Actions and combinations (EN 1990 §6.4.3.2, Table A1.1)

- Partial factors and ξ: `🧰️structure-helpers/🦀️.rs:55-57` returns (γG, γQ, ξ) = (1.35, 1.50, 0.85). Constants at `🦀️.rs:43-45`.
- Eq. 6.10a (ULS): γG·G + γQ·Q1 + Σ γQ·ψ0·Qi. Eq. 6.10b: ξ·γG·G + γQ·Q1 + Σ γQ·ψ0·Qi. Implemented in `combine_scalar_forces` (111-122) and `combine_member_actions` (173-200). Combination ids `uls-6.10a-<lead>` and `uls-6.10b-<lead>`.
- SLS characteristic: G + Q1 + Σ ψ0·Qi, ids `sls-char-<lead>` (`structure-helpers:130`, `:216`).
- Permanent-only case: `uls-g` with γG·G only (structure-helpers:105-107 and 161-171).
- Psi factors: `psi_factors` at `🧰️structure-helpers/🦀️.rs:40-52`.
  - office, residential, congregation: 0.7 / 0.5 / 0.3
  - shopping, storage: 0.7 / 0.7 / 0.6
  - snow (DE): 0.7 / 0.2 / 0.0; snow (EN): 0.5 / 0.2 / 0.0
  - wind: 0.6 / 0.2 / 0.0; temperature: 0.6 / 0.5 / 0.0
  - self, permanent, accidental: 1.0 / 1.0 / 1.0
- Permanent loads are never split into favourable and unfavourable. Only `permanent` and `prestress` are treated as G. Accidental and seismic (6.11) combinations are not implemented. `accidental` is treated as a variable load.

### 2.2 Clause table

Each row gives the check id, clause, what is computed, and file:line. Lines are in the file named by the group heading. "Member" checks live in `🧱️member-checks/🧮️action-resistance-checks/🦀️.rs` unless noted.

**EN 1993-1-1 §5 (classification)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.empty` | 1-1 §1 | Not-applicable when the snapshot has no subjects | `🧱️member-checks/🦀️.rs:33-45` |
| `en1993.5.2.class.<member>` | 5.2, Table 5.2 | Flange c/t vs 9ε, 10ε, 14ε; web c/t vs 72ε, 83ε, 124ε (bending limits used for all actions). Class = max(flange, web). c_f=(b−tw−2r)/2, c_w=h−2tf−2r. Class 4 gives A_eff utilization. | `🏷️section-class-checks/🦀️.rs:14-41`; formulas `📏️parts/🦀️.rs:9-48` |
| `en1993.5.2.plastic.<member>` | 5.2.2 | Only when analysis=plastic and class>1; requires class 1 | `🏷️section-class-checks/🦀️.rs:43-67` |
| ε | 5.2 | ε = √(235/f_y[MPa]) | `📏️parts/🦀️.rs:9-11` |

**EN 1993-1-1 §6.2 (cross-section)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.6.2.4.n.<combo>` | 6.2.4 | N_Ed ≤ N_Rd = A·f_y/γM0 (A_eff for class 4). Compression only (N ≥ 0). Remedy: upsize, else area. | action-res `:69-109`; formula `parts:78-80` |
| `en1993.6.2.3.nt.<combo>` | 6.2.3 | N_t,Ed ≤ N_t,Rd = min(A·f_y/γM0, 0.9·A_net·f_u/γM2). Tension only. | action-res `:110-139`; formula `parts:94-98` |
| `en1993.6.2.5.my.<combo>` | 6.2.5 | M_y,Ed ≤ M_c,Rd = W·f_y/γM0, with W_pl (class ≤2), W_el (class 3) or effective W_el (class 4), reduced by 6.2.8 for V_z. Only My is checked. | action-res `:141-182`; formulas `parts:83-86`, `parts:266-277` |
| `en1993.6.2.6.v.<combo>` | 6.2.6 | V_Ed = max(|V_y|, |V_z|) ≤ V_pl,Rd = A_v·(f_y/√3)/γM0. Remedy: shear area. | action-res `:185-233`; formula `parts:88-91` |
| `en1993.6.2.9.mn.<combo>` | 6.2.9 | M+N η. Class ≥3: linear sum. Class ≤2: M_N,y = M_pl,y(1−n)/(1−0.5a), M_N,z, α=2, β=max(5n,1), a=(A−2bt_f)/A ≤ 0.5. Limit η ≤ 1. | action-res `:249-285`; formula `parts:280-318` |

**EN 1993-1-1 §6.2.8 (shear reduction)**

- ρ = (2V/V_pl − 1)², applied when V > 0.5·V_pl, only with V_z against My. Formula at `parts:266-277`. Used at `action-res:44`. No standalone check on M_z (see gaps).

**EN 1993-1-1 §6.3 (buckling)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.6.3.1.nb.<combo>` | 6.3.1 | N_Ed ≤ N_b,Rd = χ·A·f_y/γM1, with N_b,Rd = min over y and z. Compression only. Remedy: bisection on L_cr, then section. | action-res `:316-386`; member `:73-79`; formula `parts:175-177` |
| λ̄ | 6.3.1 | λ̄ = (L_cr/i)/(π√(E/f_y)) | `parts:164-167` |
| χ | 6.49 | φ = ½[1+α(λ̄−0.2)+λ̄²], χ = min(1, 1/(φ+√(φ²−λ̄²))); χ=1 for λ̄≤0.2 | `parts:154-161` |
| α | Table 6.1 | a0 0.13, a 0.21, b 0.34, c 0.49, d 0.76 | `parts:110-119` |
| Buckling curve | Table 6.2 | h/b and t_f thresholds (0.040, 0.100 m) for y and z axes. fy>460 MPa forces a0 on both axes. | `parts:123-151`, `parts:125-127` |
| `en1993.6.3.2.mb.<combo>` | 6.3.2 | M_y,Ed ≤ M_b,Rd = χ_LT·W_y·f_y/γM1. Remedy: restraint spacing by bisection. | action-res `:388-457`; formula `parts:234-237` |
| λ̄_LT | 6.3.2 | λ̄_LT = √(W_y·f_y/M_cr) | `parts:226-232` |
| M_cr | 6.3.2.2 (simplified) | C1·√(π²EI_z/L²)·√(GI_t + π²EI_w/L²) × k_g, with k_g = 1/(1+0.5|z_g/h|) (ad hoc) and C1 from ψ approximation or 1.13 / 1.35 by moment diagram | `parts:180-205`; C1 `:183-195`; k_g `:203` |
| LTB curve | Table 6.4 | rolled: h/b≤2 → a, else b; welded (kind contains "weld"): h/b≤2 → c, else d | `parts:208-218` |
| `en1993.6.3.3.int.<combo>` | 6.3.3 (Eq. 6.61/6.62) | η61 = N/N_b,Rd,y + k_yy·My/M_b,Rd + k_yz·Mz/M_z,Rd; η62 analogous. Check η = max ≤ 1. Only beamColumn or column. | action-res `:462-524`; `kij` `parts:240-257`; `eta` `parts:260-264` |
| k_ij | Annex B (simplified) | Cm = 0.6+0.4ψ (≥0.4). k_yy = Cm(1+(λ̄−0.2)n) capped at Cm(1+0.8n), n=N/N_Rk. k_zz analogous. k_yz = 0.6 k_zz, k_zy = 0.6 k_yy. No χ factor. Class ≥4 uses 0.6λ̄n form. | `parts:240-257` |

**EN 1993-1-1 §7 (SLS)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.7.sls.<combo>` | 7.2 | δ = 5·q·L⁴/(384·E·I_y) with q = 8M/L² (uniform-load equivalent from the characteristic My). Check δ ≤ L/deflection_limit_ratio. Applies to all member kinds. | action-res `:527-584`; formula `parts:321-327` |

**Joints (EN 1993-1-8), `🔗️special-element-checks/🔩️joint-checks/🦀️.rs`**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.1-8.3.6.shear.<joint>` | 3.6.1 | F_v,Ed ≤ F_v,Rd = n·m·α_v·A_s·f_ub/γM2 with α_v = 0.6 for all classes. Remedy: rows, diameters. | `🔩️joint-checks:31-64`; formula `parts:584-587` |
| `en1993.1-8.3.6.bearing.<joint>` | 3.6.1 | F_b,Ed ≤ F_b,Rd = Σ k1·α_b·f_u·d·t/γM2. α_b = min(e1/3d0, p1/3d0−¼, f_ub/f_u, 1), k1 = min(2.8e2/d0−1.7, 1.4p2/d0−1.7, 2.5). d0 = d+2 mm. | `🔩️joint-checks:66-102`; formulas `parts:589-603` |
| `en1993.1-8.3.6.tension.<joint>` | 3.6.1 | F_t,Ed ≤ F_t,Rd = 0.9·A_s·f_ub/γM2. Checked only when tension_ed > 0. No shear–tension interaction. | `🔩️joint-checks:104-131`; formula `parts:605-607` |
| `en1993.1-8.3.9.slip.<joint>` | 3.9 | F_s,Rd = k_s·n·μ·F_p,C/γM3, F_p,C = 0.7·f_ub·A_s if not declared. Category B or C only. | `🔩️joint-checks:133-180`; formula `parts:619-626` |
| `en1993.1-8.4.5.simplified.<joint>` | 4.5.3.3 | F_w,Rd = a·L·f_u/(β_w·√3·γM2), β_w by weld grade (S235 0.8, S275 0.85, S355/S420/S460 0.9). Welded joints only. | `🔩️joint-checks:182-213`; formulas `parts:609-616`, `parts:629-631` |
| `en1993.1-8.4.5.directional.<joint>` | 4.5.3.2 | Equal-share σ⊥ = τ⊥ = F/(√2·a·L), τ∥ = 0. Check √(σ⊥²+3τ⊥²) ≤ f_u/(β_w·γM2). Welded joints only. | `🔩️joint-checks:215-247`; formula `parts:634-645` |

Bolt resistance inputs: `bolt_as` (`parts:561-574`, hard-coded M12-M30 table, approximation otherwise), `bolt_fub` (`parts:576-582`, 10.9 → 1000, 12.9 → 1200, else 800 MPa), preload `parts:619-621`.

**Fire (EN 1993-1-2), `⏳️fatigue-fire-and-thin-walled-checks/🦀️.rs`**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.1-2.prot.<fire>` | 4.2.5 | Required protection thickness t_req by bisection on d_p (0 to 80 mm), where the steel temperature after the rating time is ≤ θ_a,cr. Check t_p ≥ t_req. Steel temperature by explicit time stepping (dt = 0.5 s, ISO 834 gas curve, net heat flux with α_c=25, ε=0.7). | `⏳️...:58-114`; formulas `parts:360-481` |
| `en1993.1-2.theta.<fire>` | 4.2.4 | θ_a,cr = 39.19·ln(1/(0.9674·μ0^3.833)−1)+482 (Eq. 4.22) ≥ θ_a = max(design temperature, computed). Remedy: reduce μ0 by 0.95 steps. | `⏳️...:116-158`; formula `parts:361-364` |
| k_y,θ, k_E,θ | Table 3.1 | Step functions, no interpolation. | `parts:366-398` |
| χ_fi | 4.2.3.2 | Computed and shown in the explanation only. Not used in any utilization. λ̄θ = √(k_y/k_E), with no member λ̄. The φ term omits the (λ̄θ−0.2) offset. | `⏳️...:80-88`; `parts:401-408` |

**Cold-formed (EN 1993-1-3), same file**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.1-3.cf.<member>` | 5.5.2 | N_Ed ≤ ρ·N_gross. λ_p = (b̄/t)/(28.4·ε·√k_σ). ρ = (λ_p−0.055(3+ψ))/λ_p² for λ_p>0.673. N_gross is an input. Remedy: thickness ∝ √(N/N_eff), or gross resistance. | `⏳️...:163-207`; formulas `parts:486-498` |

**Stainless (EN 1993-1-4)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.1-4.ss.<member>` | 6 | M_Ed ≤ W_pl·f_y/1.1 (γM fixed at 1.1 regardless of annex). Only for materials with kind "stainless", only My. Remedy suggests S355/S460 (carbon grades). | `⏳️...:210-243`; formula `parts:504-507` |

**Plated (EN 1993-1-5)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.1-5.plate.<panel>` | 4.1 | σ_Ed ≤ ρ·f_y/γM0 with λ_p = (b/t)/(28.4·ε·√k_σ) and ρ = (λ_p−0.055)/λ_p² for λ_p>0.673. ForceAction.force is read as a stress in Pa. Remedies: thickness, width. | `⏳️...:245-286`; formulas `parts:513-528` |

**Shells, silos, through-thickness, tension, HSS (`🏛️shell-and-structure-checks/🦀️.rs`)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.1-6.shell.<silo>` | 8.5.2 | σ_Ed = p_h·r/t, p_h = K·γ·d (Janssen). σ_x,Rcr = 0.605·E·t/r with E hard-coded 210 GPa. λ̄ = √(f_y/σ_Rcr), α = 0.62/(1+1.91(r/(400t))^1.44)^0.6, χ, σ_Rd = χ·f_y/γM1. Check σ_Ed ≤ σ_Rd. | `🏛️...:6-49` (E at `:10`); formulas `parts:751-758`, `parts:533-556` |
| `en1993.4-1.silo.<silo>` | 5.3 | Same computation as the EN 1993-1-6 check, reported under 4-1. | `🏛️...:51-77` |
| `en1993.1-10.t.<member>` | 2.1 | t = max(t_f, t_w) ≤ t_max (base 20/30/50/70 mm for JR/J0/J2/K2-M-N, minus 5 or 10 mm for negative temperature). Temperature hard-coded to 0 °C. | `🏛️...:80-110`; formulas `parts:699-711` |
| `en1993.1-11.tc.<component>` | 6.2 | N_Ed ≤ min(f_uk/1.5, f_k) (1.5 hard-coded). | `🏛️...:112-151`; formula `parts:714-717` |
| `en1993.1-12.hss.<member>` | 4.1 | M_Ed ≤ W_el·f_y/γM0. Applies to materials with kind "hss" or f_y ≥ 460 MPa. Class >3 gives a dummy utilization. | `🏛️...:154-210`; formula `parts:720-728` |

**Bridges (EN 1993-2)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.2.fat.<bridge>` | 9.3 | Δσ_E,2 = λ·φ2·Δσ_p ≤ Δσ_c/γMf (Δσ_c from detail category). | `🏛️...:218-255`; formulas `parts:732-738` |
| `en1993.2.int.<bridge>` | 6 | η = N/N_Rd + M/M_Rd ≤ 1, N_Rd = A·f_y/γM0, M_Rd = W_pl·f_y/γM0 with class 2 forced. Linear, no buckling or LTB. Uses the governing member row. | `🏛️...:257-289`; formula `parts:731-734` |

**Towers (EN 1993-3-1)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.3-1.tower.<leg>` | 5 | N_Ed ≤ χ·A·f_y/γM1/max(c_f·c_d, 1). χ from the y-axis curve only. Annex B approximation, ad hoc. | `🏛️...:291-335`; formula `parts:744-747` |

**Piles (EN 1993-5)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.5.drive.<pile>` | 12.1 | σ_drive ≤ 0.9·f_y. Remedy suggests 0.6·f_y (inconsistent). | `🏛️...:337-365`; formula `parts:767-769` |
| `en1993.5.comp.<pile>` | 6.1 | N_Ed ≤ k_red·A·f_y/γM0. k_red from driving stress and shaft geometry (ad hoc factors). | `🏛️...:367-404`; formulas `parts:772-787` |

**Crane runways (EN 1993-6)**

| Check id | Clause | Computed | Location |
|---|---|---|---|
| `en1993.6.crane.<runway>` | 5.7.1 | σ = φ·F/(l_eff·t_w) with l_eff = contact + 2·dispersion ≤ f_y/γM0. Ad hoc, no web crippling or buckling. | `🏛️...:406-448`; formulas `parts:791-797` |

**Referential integrity (`🧰️structure-helpers/🦀️.rs:229-300`)**

- Duplicate ids in all 17 tables: `push_duplicate_ids` (line 229), called at 295-324. Check ids `en1993.integrity.<table>.<id>`.
- Dangling references (member sectionId/materialId, memberActions memberId/loadCaseId, joint, fatigue, fire, bridge, tower, pile, crane, cold-formed, plated and tension actions): `push_dangling_ref` (266) and `push_referential_integrity` (295-433). Titles "EN 1993 integrity" (243-244, 272-273).

### 2.3 Not implemented (clauses or verifications absent)

- EN 1993-1-1 §6.2.1 (stress checks), §6.2.7 (torsion; `t` is combined but never checked), §6.2.10 (bi-axial bending by shear), §6.2.8 for V_y.
- Standalone M_z bending check (only inside 6.2.9 and 6.3.3).
- §6.3.2.3 (general LTB, kc and kw factors), §6.3.4 (Annex BB), §6.3.5 (general buckling), Annex A and B method 1.
- §6.4 shear buckling and §5.5 web shear buckling (EN 1993-1-5).
- EN 1993-1-7 (out-of-plane plates). EN 1993-1-8: punching, block tearing, plate net section, shear-tension interaction (Table 3.4), hollow-section joints (§7).
- EN 1993-1-9: stress spectrum from rainflow, cut-off, multiple detail types per member.
- EN 1993-1-10: the real method (σ_Ed and thickness correction tables).
- EN 1993-1-11, 1-12: only the simplified checks above.
- Vibration and other serviceability limits.
- Accidental and seismic combinations (6.11).

---

## 3. NA-DE parameters actually used

Sources: `🧰️structure-helpers` and `🧬️schema/💡️inferences/⚖️checks/🦀️.rs:20-63`.

| Parameter | EN recommended (code) | DE NA (code) | Where | Used by |
|---|---|---|---|---|
| γ_M0 | 1.00 | 1.00 | root `:32`, `:37` | 6.2.x, plates, shells, towers, piles, crane, HSS |
| γ_M1 | 1.00 | **1.10** | root `:33`, `:38` | flexural, LTB, member interaction, shells (σ_Rd), 1-12 n/a |
| γ_M2 | 1.25 | 1.25 | root `:34`, `:39` | net tension, bolts, welds, directional weld |
| γ_M3 | 1.25 | 1.25 | root `:35`, `:40` | slip (single value, no SLS variant) |
| γ_Mf | 1.15 | 1.15 | root `:36`, `:41` | Overwritten per fatigue method (1.00 / 1.15 / 1.35) |
| γ_G | 1.35 | 1.35 | root `:43` | Combinations, both annexes |
| γ_Q | 1.50 | 1.50 | root `:44` | Combinations, both annexes |
| ξ | 0.85 | 0.85 | root `:45` | Eq. 6.10b, both annexes |
| ψ0 snow | 0.5 | **0.7** | `🧰️structure-helpers:44-45` | Snow combinations only. Not verified against DIN EN 1990/NA. The test `de_na_snow_psi0_differs_from_en_recommended` asserts the difference. |
| ψ1, ψ2 snow | 0.2 / 0.0 | 0.2 / 0.0 | same | |
| Stainless γ_M0, γ_M2 | 1.1 / 1.25 | 1.1 / 1.25 | `parts:505` (hard-coded, annex-independent) | 1-4 bending |

The annex only changes γ_M1 and ψ0 for snow. Everything else is annex-independent. `AnnexChoice` comes from `⚖️compliance` (variants En and De). Oracle `⚖️compliance/🐍️.py` uses DE γ_M1 = 1.1 and EN = 1.0.

---

## 4. Editor (`✏️editor`)

Application manifest: `✏️editor/🦀️.rs:234-349`, `create_en1993_app`. Mode `edit` (default). Layout `edit_mode::layout()`.

Windows (window kinds):

- **Inputs** (`norm-en1993-inputs`, icon "download", `🪟️windows/📥️inputs/🦀️.rs`). Renders the whole document as a generic value tree (`render_document_editor` → `render_value_editor` in `📇️registry/🧬️contract/🖥️app-surface/📝️document-editors/🦀️.rs:109-117`), with 171 field-meta entries (`🏷️field-meta/🦀️.rs`): English and German labels, SI units and choice lists. It is an editable tree, not raw JSON text. The pretty-JSON path `render_document_json` is kept only for tests (`document-editors/🦀️.rs:181-184`).
- **Results** (`norm-en1993-results`, icon "bar-chart-3", `🪟️windows/📊️results/🦀️.rs`). Renders `render_report` (`📇️.../🌳️report-tree/🦀️.rs:214-232`): one tree node per part (✓/✗), with one item per check. Each item shows status, utilization and remedy rows. Remedy text is localised; the choice of remedy is a command.

Panels:

- **Artifact** (`📌️panels/🗿️artifact/🦀️.rs`): a single text line from `render_summary` (`document-editors/🦀️.rs:187+`), e.g. "Eurocode 3 — N checks: complies / does not comply: X pass, Y fail, … · max=… · ✓/✗ per part".
- **Catalogue** (`📌️panels/📚️catalogue/🦀️.rs`): two static tables. (a) Partial factors γ_M, EN vs DE. (b) Rolled HEB property table (designation, A cm², W_pl,y cm³, I_y cm⁴) for HEB 240, 260, 280 and 300. The examples list is also shown here.
- **Inspection** (`📌️panels/🔍️inspection/🦀️.rs`): one selected check in full (`render_inspection`, `📚️catalogue-and-inspection/🦀️.rs:150-175`). Fields: id, part, clause, subject, subject path, status, title, explanation, computed, limit, utilization, remedies. The selection is view state in the results window config.

Commands (actions), `✏️editor/🦀️.rs:65-72` and `🎮️commands/`:

| Action | Payload | Effect |
|---|---|---|
| `evaluate` | none | `Emit::default()` (`🎮️commands/🧮️evaluate/🦀️.rs:24-27`). A no-op: recomputation happens on render, not in the command. |
| `setSelectedCheckIndex` | `index: Option<u32>` | View state only (results window config). |
| `setField` | `path`, `value_json` | Mutation via `EDIT_RULES.resolve`. |
| `insertItem` | `path`, `index`, `value_json?` | Mutation. |
| `removeItem` | `path`, `index` | Mutation. |
| `applyRemedy` | `checkId`, `remedyIndex` | Mutation chosen from the check's remedy list. |

Field-meta choice vocabularies (`🏷️field-meta/🦀️.rs:9-76`): annex (En, De); action kind; action category (permanent, imposed, wind, snow, accidental, self, office, shopping, …); member type (beam, column, beamColumn, brace, tie); load application (shearCenter, topFlange, bottomFlange); analysis (elastic, plastic); moment diagram (linear, uniform, parabolic); material kind (carbon, stainless); joint kind (bolted, welded); joint category (A, B, C); fatigue method (damage_tolerant, safe_life, …); fire rating (r30, r60, r90, r120); section kind (rolledI, weldedI, hss).

Examples (`📚️examples`, `✏️editor/📚️examples/`): `demo-session` (asset `.cmd.semio`, content `action=demo`). Catalogue examples come from `📚️examples/` (see section 7).

What the editor does **not** have: no viewport, no 2D diagrams, no 3D rendering, no drag manipulation, no interaction curves, no heat maps, no live graphics. The only visual output is tree and text.

---

## 5. Viewer (`👁️viewer`)

- App: `👁️viewer/🦀️.rs:94-106`, `create_en1993_viewer`. Document `["semio","norm","en1993"]`, icon "check-circle", mode `view` (default), one window kind `report`, layout `view::layout()`.
- Commands: none beyond the view mode.
- Report window (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`): read-only table via `TableWindowKit`. Each render recomputes the full check report with `inferences::evaluate` (line 27-29) and tables it.
- Columns (`🏷️ids-chrome-and-labels/🦀️.rs:49`): Part, Clause, Subject, Status, Utilization, Title, Remedy (EN and DE: Teil, Klausel, Gegenstand, Status, Ausnutzung, Titel, Abhilfe). The explanation text and the computed and limit quantities are not shown in the table.
- No graphics, no interaction beyond reading the table.

---

## 6. Tests and oracles

### 6.1 Rust tests (72 `#[test]` functions in this subset)

- `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (32). Examples: axial resistance HEB240 S355 vs hand calc; flexural χ never user-supplied; DE γ_M1 raises buckling utilisation; bolt bearing M20 8.8; critical temperature Eq. 4.22; fatigue category 71; compliant example has no fails; noncompliant fails and remedies apply; section class HEB240 S355 is class 1 or 2; M_cr positive; remedy flips for section id and bolt rows; every emitted path parses; LTB curve follows Table 6.4; slip for category C; **`python_oracle_and_jsonschema_agree_within_half_percent`** (runs `python3` oracle and `jsonschema` validator); perturb every editable leaf changes a check; dangling references and duplicate ids fail; design temperature changes fire check; class-4 section fails with remedy; Palmgren-Miner at 2 M cycles; class-4 uses effective section; EN and DE text distinct; DE snow ψ0 differs; pitch and gauge enter bearing; moment diagram changes M_cr; joint and tower explanations name the combination; multi-part examples fill all lists; member type gates buckling and LTB.
- `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (18), `🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🦀️.rs` (1), `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (13), `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` (2), `✏️editor/🏷️field-meta/🧪️tests/🔬️unit/🦀️.rs` (3), `✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs` (2), `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (1).
- The viewer, the editor windows and panels, the inspection and the inference unit folders contain no `#[test]` attributes in this subset (their `mod tests` files exist but are empty of test functions).

### 6.2 Mutation test suite (language-agnostic, Gherkin)

- Feature: `🧪️tests/🔩️mutate-en1993-1/🥒️.feature` (198 lines). Scenario outlines: apply each mutation to committed `before` and compare to committed `after` and outcome; undo each mutation with its inverse; re-emit the committed `.dsl.semio` byte for byte. Tags name the external oracle `oracle-en1993-1-python-independent`.
- Rust adapter: `🧪️tests/🔩️mutate-en1993-1/🦀️.rs` (739 lines, `adapter()` at line 724). Fixture projections, mutate, inverse, round-trip.
- Python component: `🧪️tests/🔩️mutate-en1993-1/🐍️.py` (184 lines). It imports `semio_norm_vocabulary` (module `🐍️`) from the shared norm reference engine outside this subset, at `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`. That engine is shared by all 15 norm artifacts (its own docstring says so), so it is not an independent check of EN 1993 numbers.

### 6.3 Fixtures

- `🧪️tests/🧫️fixtures/🧬️mutations/`: 49 mutation kinds, each with `✅apply` (before snapshot, mutation, after snapshot, diff, outcome, 5 files each) and `⛔dupe` (refusal) directories. 405 fixture files in total under `🧪️tests/🧫️fixtures`.
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/` (sqlite fixtures) and `🚪️io/💾️binary/📸️snapshot/🧪️tests/` (binary).

### 6.4 Oracles

**`🔮️oracles/⚖️compliance/🐍️.py`** (144 lines, stdlib only: json, math, pathlib). It is the only independent numeric oracle for clauses:
- Reads `🖼️assets/✅️heb240-compliant/snapshot.json` and `🖼️assets/🔩️high-strength-connection/snapshot.json`.
- Computes EN 1990 combination (first member only), axial utilisation, flexural buckling DE and EN utilisation with **hard-coded** Table 6.2 curves for h/b = 1 (y: b = 0.34, z: c = 0.49), overloaded non-compliant case with hard-coded 1.35 × 2.1 MN and L_cr = 12 m, critical temperature at μ0 = 0.5 (584.7 °C), and a slip value by hand (μ=0.5, 4 bolts, A_s = 245 mm², f_ub = 800 MPa).
- Asserts: axial < 1, buckling < 1, DE > EN, non-compliant fails, θ_cr ≈ 584.7 ±2, design N ≈ 200 kN ±50 N.
- The Rust test compares only checks 6.2.4 and 6.3.1 (both annexes, and the overloaded case) within 0.5%.

**`🔮️oracles/🧬️snapshot-schema/🐍️.py`** (14 lines): validates the snapshot JSON against `🧬️schema/📸️snapshot/🔣️.json` with **`jsonschema` (third-party, Python)**. It is invoked from the compliance test.

**`🔮️oracles/🔣️.json`** (1829 lines): oracle manifest. Entry `en1993-1-python-independent`, `ecosystem: python`, `package: ""`, `kind: verified-native-second-implementation`, `testOnly: true`, capability `en1993-1-mutate`, 65 fixture vectors. Its rationale says PyPI was surveyed (structuralcodes, concreteproperties, anastruct rejected) and that under protocol v2 a qualifying third-party reference is **still owed** for the 49-kind mutation vocabulary.

**Third-party libraries**

- Runtime (artifact crates): none. Crates depend on framework path or workspace crates only.
- Test-only: `serde`, `serde_json` (Rust, `cfg(test)` and dev-dependencies). `jsonschema` (Python, one test).
- Clause formulas: no third-party library is used to produce an expected value. This falls short of the AGENTS rule that test output must match at least one third-party library.

**Language-agnostic tests**: only the mutation Gherkin suite. Compliance checks have no feature file.

---

## 7. Examples and fixtures

- `📚️examples/✅️heb240-compliant/` (`🦀️.rs` reads `🖼️assets/✅️heb240-compliant/✅️heb240-compliant/🗣️.dsl.semio`, 53 lines; `snapshot.json`, 332 lines). Annex DE. HEB 240 S355, member `member-b1` (beam-column, 4 m), joint `joint-j1` (bolted M20 8.8, 2×2), fatigue detail category 71, fire R60, cold-formed member, plated panel, silo shell, tension component, bridge fatigue, tower leg, pile, crane runway. Intended to pass.
- `📚️examples/🔩️high-strength-connection/` (`🦀️.rs` label "Overloaded Steel Frame"). Asset `🖼️assets/🔩️high-strength-connection/` (`snapshot.json`, 354 lines; `🔩️.../🗣️.dsl.semio` 53 lines; `🎒️.pack.semio` 2 lines). Annex EN, S460 HEB 260 member `member-b1` (beamColumn, 12 m buckling length, plastic analysis in the overloaded variant), overloaded actions, multi-part.
- `📚️examples/🔩️high-strength-connection/🖼️assets/` also has `📡️high-strength-connection.spr.semio` and `🔧️high-strength-connection.op.semio` (2 lines each).
- `✏️editor/📚️examples/🎬️demo-session/` (asset `🎮️.cmd.semio`, content `semio norm.en1993.cmd v1` / `action=demo`).
- Mutation example fixtures: `🧪️tests/🧫️fixtures/🧬️mutations/` (see 6.3).
- `🖼️assets/✅️heb240-compliant/` and `🖼️assets/🔩️high-strength-connection/` hold the canonical JSON and DSL assets used by the oracle and the tests.

---

## 8. Gaps, stubs, TODOs, hard-coded values and ad hoc shortcuts

No TODO, FIXME or `todo!`/`unimplemented!` in production code. The grep found only "SDK GAP" notes (`✏️editor/🦀️.rs:42`, `👁️viewer/🦀️.rs:20-21`, `👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9`) and some "approx"/"simplified" doc comments. The problems below are from reading the code.

### 8.1 Likely defects or non-conservative results (verify before relying on them)

1. Fatigue: `params.gamma_mf = gmf;` at `⏳️...:10`, but `params` is an immutable parameter in `check_fatigue_and_fire` (line 6). This looks like a compile error. Verify the build.
2. LTB remedy bisection: the first loop (`action-res:403-417`) computes `lo` and then discards it. Line 412-413 repeats `lo = mid;`. Dead code.
3. §6.3.3 interaction: `m_b_rd.max(m_y_rd_red)` at `action-res:484` uses the larger of M_b,Rd and M_c,Rd. It should use M_b,Rd. Non-conservative.
4. Plate reduction (EN 1993-1-5 §4.4): `part_1_5::plate_reduction_factor` (`parts:518-523`) subtracts the constant 0.055 instead of 0.055(3+ψ). Non-conservative for ψ ≠ −2. The 1-5 check (`⏳️...:253-255`) uses this path.
5. Fire buckling χ_fi: `parts:406` omits the (λ̄θ−0.2) offset, and `⏳️...:80` uses √(k_y/k_E) without the member slenderness. χ_fi is only displayed, never checked.
6. Bolt shear: α_v = 0.6 for all bolt classes (`parts:585`). EN 1993-1-8 Table 3.4 gives 0.5 for class 10.9 (and some others).
7. Plastic analysis: `section-class:43` requires class 1. EN 1993-1-1 §5.2.2 allows class 2 for plastic resistance (class 1 for rotation). Over-strict.
8. Combination psi: storage is mapped to shopping values (`structure-helpers:43`: 0.7 / 0.7 / 0.6). EN 1990 Table A1.1 gives storage ψ0 = 1.0, ψ1 = 0.9, ψ2 = 0.8. Congregation (`:42`) uses office values; Table A1.1 category C gives 0.7 / 0.7 / 0.6. Medium-high confidence; verify against the standard.
9. DE snow ψ0 = 0.7 (`:44`). EN recommended is 0.5. Verify against DIN EN 1990/NA.
10. Stainless remedy suggests S355 and S460 (`⏳️...:230`), which are carbon grades. Wrong grade family.
11. Plated panel: `ForceAction.force` is documented as N elsewhere but read as a stress in Pa (`⏳️...:246-250`; example value 300e6). Units are ambiguous.
12. Pile: the driving-stress check uses 0.9·f_y (`🏛️...:340`), but the remedy suggests 0.6·f_y (`:395-396`). Inconsistent.
13. Silo: E is hard-coded at 210 GPa (`🏛️...:10`), not taken from the material. Hoop stress is compared to an axial-buckling resistance. The EN 1993-4-1 check (`:51-77`) repeats the same calculation under a different clause.
14. Joint tension is checked only when tension > 0 (`joint-checks:104`). No combined shear and tension check.
15. Directional weld (`parts:634-645`): σ⊥ = τ⊥ = F/(√2·a·L), τ∥ = 0, and the 0.9·f_u/γM2 limit on σ⊥ is missing.
16. §6.2.6 (`action-res:204-206`): V_Ed is the larger of V_y and V_z, and only that direction's V_pl is used. No check on the other direction.
17. §6.2.8 shear reduction is applied to My with V_z only (`action-res:44`).

### 8.2 Architecture and coverage gaps

18. **Single governing ULS row per member** (`structure-helpers:221-227`, `action-res:36`). Chosen by |N|+|My|+|Vz|. Other combinations that govern a different check (e.g. max My with a different N) are never evaluated. This is the largest coverage gap.
19. No standalone M_z check. Torsion T is combined but never checked.
20. Accidental and seismic (6.11) combinations are missing. Favourable permanent loads (ξ on the favourable side) are not modelled.
21. SLS deflection `parts:321-327` assumes a uniform load equivalent from the moment, regardless of `moment_diagram` or load pattern.
22. M_cr uses an ad hoc load-height factor k_g = 1/(1+0.5|z_g/h|) (`parts:203`) and approximate C1 values (`parts:183-195`). L_LT is `max(ltb_length, restraint spacing)` (`member-checks:80`) while the LTB remedy uses restraint spacing.
23. Kij (Annex B) is simplified: no χ, k_yz = 0.6 k_zz, k_zy = 0.6 k_yy (`parts:254-255`).
24. `effective_area_class4` (`parts:61-69`) is an ad hoc flange-only reduction. `_a_web` at line 67 is unused. W_el,z for class 4 is scaled by the same factor (`member-checks:62-63`).
25. Class is taken as max of flange and web, with web limits for bending for all actions (`parts:44-48`).
26. Buckling curves for f_y > 460 MPa collapse to a0 on both axes (`parts:125-127`), and the Table 6.2 thresholds are applied to the rolled-I case only.
27. Cold-formed: N_gross is an input (`gross_resistance`), not computed from geometry. No bending, no local-global interaction.
28. Stainless: only bending; γ fixed at 1.1; no buckling, no shear.
29. Plate (1-5): only σ_Ed vs ρ f_y/γM0. No shear, no interaction, no effective width.
30. Shell (1-6): only axial-buckling-based resistance for hoop stress. No hoop buckling, no meridional or interaction check.
31. Fire: steel temperature by explicit scheme with dt = 0.5 s and an ad hoc protected-steel term and a −50 K per step clamp (`parts:423-455`). Required thickness bisection returns the upper bound 0.080 m if the target is never met (`parts:458-481`). k_y,θ and k_E,θ are step functions (`parts:367-398`), not interpolated.
32. Toughness (1-10): base thickness table with temperature fixed at 0 °C (`🏛️...:84`).
33. Tension component (1-11): 1.5 hard-coded (`parts:716`).
34. HSS (1-12): dummy utilization (3.0 or class) when class >3 (`🏛️...:169-182`).
35. Bridge (EN 1993-2): interaction forces class 2 for M_Rd (`🏛️...:262`), linear sum, no buckling or LTB.
36. Tower (EN 1993-3-1): amplification `max(c_f·c_d, 1)` (`parts:745`), y-curve only, ad hoc.
37. Pile (EN 1993-5): reduction factors and geometry factor `0.85+0.02√(L·P)` are hard-coded (`parts:772-787`).
38. Crane (EN 1993-6): σ = φF/(l_eff·t_w), l_eff = contact + 2·dispersion, limit f_y/γM0. No web crippling or buckling (`parts:791-797`).
39. Bolt tensile area is a hard-coded table M12-M30 with an approximation fallback (`parts:561-574`). Hole d0 = d+2 mm is hard-coded (`joint-checks:21`). Unknown bolt class defaults to 800 MPa (`parts:576-582`).
40. Preload defaults to 0.7·f_ub·A_s when not declared (`parts:619-621`).
41. Unknown strings fall back silently: rating → R60 (`parts:342-349`), fatigue method → damage tolerant (`parts:682-688`), detail category → 71 MPa (`parts:649-654`), subgrade → idx 2 (`parts:700-707`), weld grade → 0.9 (`parts:609-616`), member kind → gating defaults. No validation at model level.
42. Fatigue D > 1 remedy searches only categories 80-160 (`⏳️...:28`).
43. `evaluate` command is a no-op (`🎮️commands/🧮️evaluate/🦀️.rs:24-27`), though its action description says it recomputes. Live feedback relies on re-render, not on the command.
44. Viewer and editor recompute the full report on each render (`👁️viewer/.../report/🦀️.rs:27-29`). No caching.
45. Editor and viewer UI: tree and table only. No graphics. Viewer table omits explanation text.
46. Catalogue: rolled HEB section list is HEB 240-300 only (`rolled_heb_catalogue`, `model:452`). `next_section_options` (`structure-helpers:6-27`) chooses only from this list, so remedies cannot propose any other section.
47. Clause labels are generic in several checks: `ClauseId::new("EN 1993-1-9", "8", "8")`, `EN 1993-2 "6"`, `EN 1993-3-1 "5"`, `EN 1993-1-4 "6"`, `EN 1993-1-12 "4.1"`. Traceability is weak.
48. `policyViewerPurityBreaches` and similar notes are design-time guards, not gaps.
49. Duplicate code: remedy blocks for section upsizing are repeated across member, shear, buckling, interaction and deflection checks. `parts:83-85` has two identical if-branches for class ≤2 and else.
50. Oracle hard-codes section and load values (`🔮️oracles/⚖️compliance/🐍️.py:115-116`, `:79-81`, `:126-129`) and checks only the first member.
51. The mutation oracle (`🐍️`, shared norm engine) is not independent of the norm family. The manifest says a third-party reference is still owed.
52. Text carrier grammar is a placeholder (`payload = OCTET+`, per the mutation feature file). The `.dsl.semio` format has no formal definition in this subset.

### 8.3 What is solid

- The EN 1990 combination machinery (6.10a/b, ψ table), the Table 5.2 classification, Eq. 6.49 χ, Table 6.1 and 6.2 curve selection, the 6.2.6 V_pl and 6.2.9 M+N form for class ≤2, Eq. 4.22 critical temperature, the Palmgren-Miner tri-linear form, the slip formula, the fillet-weld simplified method, and the bolt tension and bearing formulas are coded close to the standard.
- Referential integrity and duplicate-id checks are complete for all 17 tables.
- Remedy generation is systematic (section upsizing, at-least and at-most bisections) and localised.
