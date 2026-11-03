# 🏛️ EN 1992 Inventory

Read-only audit of the EN 1992 artifact as it exists in the working tree. No source file was modified.

## 0. Conventions

- Artifact root: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992`
- `ANY/` = `🏅️standards/🔖️1/🪆️subsets/✳️any/` under the artifact root (the only subset that exists).
- `CHK/` = `ANY/🧬️schema/💡️inferences/⚖️checks/`
- Line numbers are from the working tree at audit time.
- 656 files are tracked under the artifact root; 166 are test fixtures (mostly `ANY/🧬️schema/🧬️mutations/…` fixtures).
- Clause references are given as the artifact cites them. The standard text itself was not available to this audit, so statements such as "EN default is X" are from working knowledge and are marked "verify" where it matters.

Headline: the artifact is a structured, schema-first RC compliance model with roughly 50 distinct check families (each producing one or more `CheckResult`s per member or anchor) (ULS flexure/shear/torsion/punching, SLS stress/crack/deflection, prestress, anchorage/laps, fire tables, bridge stress/fatigue, liquid-retaining crack width, EN 1992-4 anchor resistances). It has a DE/EN annex switch, an event-sourced editor with eight commands, a read-only table viewer, and 28 typed mutations. It has no 2D or 3D visualisation at all. Several formulas are simplified or ad hoc, and at least three appear to be wrong (Section 8).

---

## 1. Model

### 1.1 Artifact schema (`ANY/🧬️schema/🦀️.rs`)

`En1992Artifact` (lines 25-45, `#[artifact_schema(id = "s.norm.en1992")]`) has ten `#[state(artifact)]` fields, which are the same fields as the snapshot:

| Field | Type |
|---|---|
| annex | `AnnexChoice` (`En` / `De`, from `semio_s_plugin_norm_compliance`) |
| title | `String` |
| design_working_life_years | `f64` |
| delta_c_dev | `f64` |
| cement_type | `String` |
| concrete_grades | `Vec<ConcreteGrade>` |
| reinforcement_grades | `Vec<ReinforcementGrade>` |
| prestress_steels | `Vec<PrestressSteel>` |
| members | `Vec<RcMember>` |
| anchors | `Vec<Anchor>` |

`to_snapshot` / `from_snapshot` (lines 47-80) are field-by-field clones. The descriptor (line 84+) wires the rust/ts/graphql/json-schema/proto leaves, `SNAPSHOT_LEAVES`, `semio_s_artifact_norm_en1992_diff::DIFF_LEAVES` and `…_mutations::MUTATION_LEAVES`.

### 1.2 Snapshot model (`ANY/🧬️schema/📸️snapshot/🦀️.rs`, 786 lines)

Top level: `En1992Snapshot` (lines 23-48) with the same ten fields as the artifact. `Default` is `compliant_office_frame()` (line 51-55).

Enums (all `Copy`, `DslScalar`, canonical-JSON):

| Enum | Variants | Lines |
|---|---|---|
| `FireRating` | R30, R60, R90, R120 | 405-410 |
| `TightnessClass` | Tc0, Tc1, Tc2 | 416-420 |
| `MemberKind` | Beam, Slab, Column, Wall, FlatSlab, RibbedSlab, TensionMember, Bridge, LiquidRetaining | 426-437 |
| `SupportCondition` | SimplySupported, Continuous, Cantilever, Fixed | 442-449 |
| `ExposureClass` | X0, Xc1-Xc4, Xd1-Xd3, Xs1-Xs3, Xf1-Xf4, Xa1-Xa3 (18 variants) | 453-476 |
| `DuctilityClass` | A, B, C | 531-535 |
| `AnnexChoice` (external) | En, De | `semio_s_plugin_norm_compliance` |

Structs:

| Struct | Fields | Lines |
|---|---|---|
| `ConcreteGrade` | id, name, f_ck (Pa) | 479-484 |
| `ReinforcementGrade` | id, name, f_yk, e_s, ductility: DuctilityClass, k, eps_uk | 553-562 |
| `PrestressSteel` | id, name, f_pk, f_p0_1k | 580-585 |
| `BarLayer` | id, diameter, count: u32, position (string, "bottom"/"top"/"tension"), anchorage_length, lap_length, bond_condition (string, "good"/"poor"), aggregate_size | 593-604 |
| `Stirrups` | diameter, spacing, legs: u32 | 610-615 |
| `PunchingSpec` | column_width, column_depth, column_position (string: interior/edge/corner), asw | 622-630 |
| `PrestressSpec` | force, area, eccentricity, loss_ratio | 635-642 |
| `FireSpec` | rating: FireRating, axis_distance, column_method (string "A"/"B"), slab_system (string) | 648-655 |
| `LoadCaseActions` | id, kind (string: permanent/prestress/variable/accidental), category (string: office/shopping/snow/wind/…), source (string: udl/point/external), g_k_line, q_k_line, point_force, m_k, n_k, v_k, t_k, v_k_punch | 661-675 |
| `RcMember` | id, label_en, label_de, kind, concrete_grade_id, reinforcement_grade_id, prestress_steel_id, exposure, width, height, effective_depth, cover, span, support, buckling_length, longitudinal: Vec<BarLayer> (table), stirrups: Option<Stirrups>, punching: Option<PunchingSpec>, prestress: Option<PrestressSpec>, fire: Option<FireSpec>, actions: Vec<LoadCaseActions> (table), use_fem: bool, udl, deflection_sensitive: bool, tightness: Option<TightnessClass>, hd_over_h, liquid_sigma_s, liquid_rho_p_eff, liquid_f_ct_eff, liquid_s_r_max, bridge_sigma_c, bridge_delta_sigma_s | 682-716 |
| `Anchor` | id, h_ef, cracked, f_uk, f_yk, a_s, d, c1, f_ck, actions: Vec<LoadCaseActions> (table) | 762-775 |

Note: `RcMember` has `prestress_steel_id` but `PrestressSpec` has no steel reference; the steel is looked up by id on the member.

Derived methods:

- `ConcreteGrade` (lines 485-534): `from_f_ck`; `f_ck_cube` (lookup table 12..100 MPa, fallback `f_ck*1.25` at 490-498); `f_cm = f_ck+8`; `f_ctm` (EN 3.1.4 two-branch, 499-504); `f_ctk_005 = 0.7 f_ctm`; `e_cm = 22 GPa·(f_cm/10)^0.3` (506); `eps_c2`, `eps_cu2`, `eps_c3`, `eps_cu3 = eps_cu2`, `n_parabola` (507-530) for fck ≤ 50 and > 50 branches.
- `DuctilityClass::table_c1_minima` (537-550): A (k 1.05, eps 2.5%), B (1.08, 5.0%), C (1.15, 7.5%).
- `ReinforcementGrade::b500a/b500b/b500c` constructors (563-575). B500C uses `f_yk = 550 MPa` (line 571), which does not match the usual 500 MPa for DE B500 (verify).
- `RcMember::a_s_tension` (721-728: sum of bars in layers tagged `bottom` or `tension`), `rho_l` (730-738), `asw_per_s` (740-749), `subject_label` (751-754).
- `En1992Snapshot` constructors: `compliant_office_frame` (59-207), `failing_under_reinforced` (208-252), `prestressed_beam_member` (253-293), `failing_prestressed_beam` (294-311), `compliant_prestressed_beam` (312-320), `liquid_retaining_fem_anchor` (321-380); lookups `concrete(id)` / `reinforcement(id)` (381-392).
- Document schema id `semio.norm.en1992/v1` (line 786); dialect `s.norm.en1992`, standard `1`, subset `any` (line 784).

---

## 2. Checks and verifications

### 2.1 Evaluation pipeline

- `evaluate` (`ANY/🧬️schema/💡️inferences/🦀️.rs:76-93`): duplicate-id checks for members, anchors, concrete grades, reinforcement grades, prestress steels (lines 78-82, `push_duplicate_ids` 95-125, clause "EN 1992-1-1 §1"). Returns early with only those results when there are no members and no anchors. Then `evaluate_member` per member and `evaluate_anchor` per anchor.
- Member orchestration (`CHK/📋️member-evaluation/🦀️.rs:22-91`), in order: materials presence (concrete §3.1 line 25-41, reinforcement §3.2 line 42-58; missing grade returns Fail with remedy options, default fallback "c30"/"b500" at lines 36/53), cover, constitutive, combination (`combine_member_actions`), governing ULS/accidental/SLS-qp/SLS-char (lines 73-77), then ULS resistances, crack width, reinforcement limits, deflection, column slenderness, prestress, ductility, fire, SLS stress limits, anchorage and laps, bridge, liquid-retaining.
- Actions and combinations (`CHK/🧮️design-effects/🦀️.rs`): see 2.3.
- Anchor evaluation (`CHK/⚓️anchor-evaluation/🦀️.rs`): see 2.7.

### 2.2 EN 1992-1-1 materials, durability and cover

| Check id | Clause | Formula / rule | Where |
|---|---|---|---|
| `en1992.materials.concrete.*` | §3.1 | grade must exist | member-eval 25-41 |
| `en1992.materials.steel.*` | §3.2 | grade must exist | member-eval 42-58 |
| `en1992.3.1.constitutive.*` | §3.1.7 | computed = f_ck·ε_c2·n vs limit = f_ck,cube·ε_cu2·2 (utilisation). This is not a Table 3.1 or §3.1.7 check; it is a self-consistency ratio with no clear basis. | cover-and-constitutive 36-67, esp. 48-49 |
| `en1992.4.4.cover.*` | §4.4 | c ≥ c_nom, c_nom = max(c_min,b, c_min,dur, 10 mm) + Δc_dev. c_min,b uses the first layer's diameter, default 12 mm if none (line 7), first layer aggregate, default 16 mm (line 8), bundle count hard-coded 1 (line 9). | cover-and-constitutive 6-33; helpers 362-371 |
| c_min,dur | §4.4.1.2 Table 4.4N | exposure lookup (helpers 268-285), plus structural-class shift ±5 mm per class (helpers 342-359, `structural_class_delta`: design life ≥100 a → +2, ≤25 a → −1, cement containing "R" → −1, f_ck≥35 for slab/beam/flat slab → −1; clamped −1..+2) | helpers 268-285, 342-359 |
| w_max | §7.3.1 / NA-DE | 0.40 mm for X0/XC1, else 0.30 mm (quasi-permanent) | helpers 287-293 |

Observations: the c_min,dur table in code (for example XC2/XC3 = 20 mm, XC4 = 25 mm, XD1/XS1 = 40 mm) is a single mapping that was not traced to Table 4.4N or the NA-DE table in this audit. The structural-class steps are heuristic. The unit test `c_min_dur_xc3_de` (`ANY/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`) pins the same numbers, so it does not validate them.

### 2.3 Design effects (EN 1990 combinations, `CHK/🧮️design-effects/🦀️.rs`)

- Partial factors hard-coded: γG = 1.35, γQ = 1.5 (lines 80-81), ξ = 0.85 (line 109). No favourable γG = 1.0 case (6.10 with inferior permanent action) is generated.
- ψ factors (`psi_factors`, lines 17-27): office/residential/congregation (0.7, 0.5, 0.3), shopping/storage (0.7, 0.7, 0.6), snow (0.5, 0.2, 0.0), wind (0.6, 0.2, 0.0), temperature (0.6, 0.5, 0.0), accidental (1, 1, 1), default = office. DE NA snow and other national values are not separately modelled.
- Characteristic effects of a load line (lines 29-51): simply supported M = qL²/8, V = qL/2; cantilever M = qL²/2, V = qL; continuous/fixed M = qL²/12, V = qL/2. Point force adds pL/4 moment and p/2 shear regardless of support (lines 45-49), which is only correct for simply supported midspan.
- Defect: for permanent actions, `member.udl` is added to every permanent action's line load (line 34), so with several permanent actions the self-weight is counted more than once.
- Fallback (lines 70-79): when there are no permanent actions but `udl > 0`, the udl is used as the permanent load.
- `n_ed` from line loads is always 0 (line 50), so normal force only comes from FEM input or anchors.
- Combinations generated (lines 83-160): with no variable actions: ULS-6.10-G, SLS-qp, SLS-char. With variables: for each leading variable action, ULS-6.10a and 6.10b (with ξ), SLS-char-<id>, SLS-freq-<id> (ψ1), and one SLS-qp (ψ2) per member. Accidental combinations ACC-6.11-<id> (lines 162-171) use ψ1 on the variables.
- Governing (lines 175-179): the governing effect per situation is the one with the largest |M_Ed|. Shear, torsion and axial are taken from that same combination, not enveloped separately.

### 2.4 ULS resistances (`CHK/📋️member-evaluation/🏗️resistance-and-serviceability-checks/🦀️.rs`)

| Check id | Clause | Formula used | Lines |
|---|---|---|---|
| `en1992.6.1.flexure.*` | §6.1 | M_Rd from `flexural_resistance_nm` (helpers 134-167): rectangular stress block, λ, η per §3.1.7 (for fck > 50), x limited to min(x_bal, 0.45d), compression from axial force reduces M_Rd by (1 − N/N_Rd). The axial reduction is a heuristic, not the §6.1 interaction. | 24-61 (M_Rd call at 26) |
| `en1992.6.1.flexure.acc.*` | §6.1 ACC-6.11 | M_Rd,acc = M_Rd × (f_cd,acc / f_cd,uls). A crude scaling, not a separate accidental resistance. | 62-84, esp. 66 |
| `en1992.6.2.shear.*` | §6.2 | V_Rd,c (Eq. 6.2, helpers 170-188), V_Rd,s (Eq. 6.8, helpers 191-193) with z = 0.9d (line 87), V_Rd,max (Eq. 6.9, helpers 196-204, ν1 = 0.6(1−fck/250), α_cw = 1). cotθ from `cot_theta`. Without stirrups the check is against V_Rd,c only. Line 95 `_v_rd` is unused. | 86-131 |
| `en1992.6.3.torsion.*` | §6.3 | T_Rd,max = a thin-walled-style approximation on the solid rectangle: t_ef = min(b,h)/6, A_k = (b−t)(h−t), T = 2·A_k·t·ν·f_cd·0.5 (helpers 230-238). No torsion–shear interaction and no longitudinal torsion steel. | 133-155 |
| `en1992.6.4.punching.*` | §6.4 | v_Ed = β·V_Ed/(u1·d), β from `punching_beta` (helpers 406-408: edge 1.40, corner 1.50, interior 1.10). u0 = 2(c1+c2), u1 = u0 + 4πd (helpers 409-412). v_Rd,c from Eq. 6.47 (helpers 207-220), DE branch with 0.15/γc and an extra u0/d factor (resistance 163-175). v_Rd,cs = v_Rd,c + 0.75·asw·f_yd/(u1·d) (line 179). | 157-211, esp. 183 |
| `en1992.6.4.punching-max.*` | §6.4.5 | v_Ed ≤ v_Rd,max; DE: 1.4·v_Rd,c (helpers 222-227); EN: 2.0·v_Rd,c (line 225, not a standard EN expression). | 212-224 |
| `en1992.uls.na.*` | §6 | Not-applicable marker when no ULS combination exists. | 227-234 |

Defaults and conventions: `cot_theta` for DE is clamp(1.2 + 0.2·σ_cp/f_cd, 1.0, 3.0) (helpers 107-115). For EN it is the constant 2.5, from `1.0f64.max(2.5_f64.min(3.0))` (helpers line 113), so EN does not use a free cotθ and the constant is not explained.

### 2.5 SLS (`CHK/📋️member-evaluation/🏗️resistance-and-serviceability-checks/🦀️.rs`)

| Check id | Clause | Formula used | Lines |
|---|---|---|---|
| `en1992.7.3.crack.*` | §7.3.4 (Eq. 7.8, 7.9) | σ_s = M_qp/(A_s·0.9d); ρ_p,eff = A_s/(b·min(2.5(c+φ/2), h/2)) with φ hard-coded 16 mm (line 257-258); s_r,max = 3.4c + 0.425·0.8·0.5·16 mm/ρ_p,eff (line 262, all constants hard-coded); w_k from `crack_width_m` (helpers 241-248: ε = (σ_s − 0.4·f_ct,eff/ρ·(1+α_e·ρ))/E_s with the 0.6σ_s floor; α_e = E_s/22 GPa hard-coded, line 245). | 238-288 |
| `en1992.9.minas.*` | §9.2.1.1 | A_s,min = max(0.26·f_ctm/f_yk·b·d, 0.0013·b·d) (helpers 296-299) | 291-325 |
| `en1992.9.maxas.*` | §9.2.1.1 | A_s,max = 0.04·b·h (helpers 301-303) | 327-345 |
| `en1992.7.4.ld.*` | §7.4.2 | l/d ≤ min(basic limit, DE cap). Basic limit `basic_ld_limit` (helpers 251-266) with K by support. Compression steel ρ' is always 0 (line 351). DE cap `de_ld_caps` = min(K·35, K²·150/l if deflection-sensitive) (helpers 386-389). Applied to Beam, Slab, FlatSlab only. | 349-377 |
| `en1992.7.2.sigma-s.*` | §7.2(5) | σ_s,char ≤ 0.8·f_yk (line 398) from cracked-section elastic stress (`cracked_sigma_s_pa`, helpers 415-421) | 380-420 |
| `en1992.7.2.sigma-c.*` | §7.2(2) | σ_c,char ≤ 0.6·f_ck (line 400; the `harsh` branch at line 400 and 425 has identical values, so the branch is dead) | 421-441 |
| `en1992.7.2.creep.*` | §7.2(3) | σ_c,qp ≤ 0.45·f_ck (line 445) | 443-462 |

### 2.6 Column slenderness and second order (`CHK/📋️member-evaluation/🧱️slenderness-prestress-and-ductility-checks/🦀️.rs`)

- `en1992.5.8.slender.*` (lines 23-66) for Column and Wall with buckling length > 0. Slenderness λ = l0/i with i = min(b,h)/√12 (helpers 306-313). Limit `lambda_lim_de` (helpers 397-399): 25 if n ≥ 0.41, else 16/√n. This is not the EN 5.8.3.1 expression 20·A·B·C/√n, and is labelled DE without a source. n = N_Ed/(b·h·f_cd) (line 26).
- Second-order moment `second_order_moment_nm` (helpers 400-405): curvature = (0.45·f_yd/E_s)/d, e2 = l0²/(10·r). Defect (line 401): nominal curvature per EN 5.8.8.2 is ε_yd/(0.45·d) = f_yd/(E_s·0.45·d), so the code's expression is about 0.2 times the standard value and the second-order moment is strongly underestimated (verify).
- Check: |M_Ed + M_2| ≤ M_Rd (utilisation). Remedies: widen column (b_req = l0/λ_lim·√12), reduce buckling length. `_ok_cap` (line 36) is unused.

### 2.7 Prestress (same file, lines 70-176)

- Prestress set-up: P_m0 = force, P_m∞ = P·(1 − clamp(loss, 0, 0.5)) (line 100), σ_pm0 = P/A_p.
- Limits: σ_p,max = min(0.8 f_pk, 0.9 f_p0.1k) (line 106, EN 5.10.2.1 form); σ_pm0 ≤ min(0.75 f_pk, 0.85 f_p0.1k) (line 107); σ_c = P/A_c + P·|e|·(h/2)/I (line 110, eccentricity sign is discarded); σ_c ≤ 0.6 f_ck (line 111).
- Losses limit: loss_ratio / 0.30 (line 116), a fixed 30 % limit without basis. Losses are input ratios, not computed (no §5.10.5 loss calculation).
- All six utilisations are max-combined into one `en1992.5.10.prestress.*` result (lines 118, 119-127). This hides which limit governs.
- Decompression `en1992.5.10.decompression.*` (lines 136-152): utilisation = max(−σ_c, 0)/(0.45 f_ck) (line 137), remedy +10 % force (line 147). The check is non-standard.
- Dangling/orphan prestress steel id checks (lines 72-96, 154-175). Remedy default option list "yp1860" if empty (line 90).

### 2.8 Reinforcement ductility (Annex C, same file lines 179-203)

- `en1992.annexC.ductility.*`: k ≥ k_min, ε_uk ≥ ε_min from `table_c1_minima` (model 537-550), ε_ud = 0.9·ε_uk displayed only. Utilisation = max(k_min/k, ε_min/ε_uk).

### 2.9 Fire, EN 1992-1-2 (`CHK/📋️member-evaluation/🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs` lines 6-48; tables `CHK/🧰️helpers-and-annex-parts/🦀️.rs` 436-554)

- Check `en1992.1-2.a.*`: axis distance ≥ a_req (clause §5.6, tabulated), `en1992.1-2.bmin.*`: b ≥ b_min. Both use `member.width` as the minimum dimension (line 29), including for slabs, where the minimum dimension should be the thickness h. Defect.
- Tables transcribed in code: beam simply supported (Table 5.5, helpers 445-449), beam continuous (5.6, 451-455), column method A (5.2a, 457-461) and B (5.2b, 463-467), wall (5.4, 469-473), tension member (5.3, 475-479), one-way slab (5.8, 481-485), two-way slab (5.9, 487-491), flat slab (5.9/5.10, 493-497), ribbed slab (5.11, 499-503), beam axis distance per width (536-551). The column method A rows for R90 and R120 are identical (350/45), which needs checking. The tables were not re-verified against the printed standard. The `DE NA restrictions` in the module doc are not implemented as separate code.
- Only tabulated fire data is used. No thermal or mechanical calculation (§4, §5 calculation methods).

### 2.10 Anchorage and laps, EN 1992-1-1 §8.4 and §8.7 (fire/anchorage file lines 51-115)

- Per bar layer: η1 = 0.7 if bond "poor" else 1.0 (line 61); η2 = 1 for φ ≤ 32 mm, else clamp((132−φ)/100, 0.7, 1) (line 62); η_agg = 0.95 if aggregate > 16 mm (line 63, not a standard factor); f_bd = 2.25·η1·η2·f_ctk,0.05 · η_agg (line 64). Note: §8.4.2 uses f_ctd (design), not the 5 % characteristic value. Hard-coded α1 = 1.0 (line 67). `l_b,rqd = φ/4·σ_sd/f_bd` (helpers 374-377), `l_bd = max(α1·l_b,rqd, 0.3·l_b,rqd, 10φ, 100 mm)` (helpers 378-381). σ_sd = f_yd.
- `en1992.8.4.anchorage.*`: provided ≥ l_bd. Remedies: longer anchorage, or smaller bar diameter.
- `en1992.8.7.lap.*` (lines 93-113): α6 hard-coded 1.5 (line 93), l_0 = max(α6·l_bd, 0.3·α6·l_bd, 15φ, 200 mm) (helpers 382-385). Lap percentage is not an input.

### 2.11 Bridges, EN 1992-2 (fire/anchorage/bridge file lines 118-153)

- `en1992-2.7.2.*`: frequent concrete stress σ_c ≤ 0.6 f_ck (helpers 560-562).
- `en1992-2.6.8.*`: Δσ_s ≤ fixed value 162.5 MPa/1.15 (helpers 564-566). This is a placeholder, not the Δσ_Rsk/γ_s,fat with the slope of the S-N curve. Only the single-value limit is implemented (verify against 6.8.4).
- Activated when `kind == Bridge` or either bridge input is > 0.

### 2.12 Liquid-retaining, EN 1992-3 (fire/anchorage/bridge file lines 156-186)

- Tightness classes (helpers 573-582): TC0 → not applicable; TC1 → w_lim = 0.3 mm; TC2 → linear interpolation between 0.2 mm and 0.05 mm over h_D/h from 5 to 35 (helpers 577-580). The interpolation is ad hoc.
- `en1992-3.crack.*`: w_k from `crack_width_m` with user-supplied inputs liquid_sigma_s, liquid_rho_p_eff, liquid_f_ct_eff, liquid_s_r_max (lines 167-169). These are not derived from geometry. Remedy: reduce σ_s to σ_s·w_lim/w_k.

### 2.13 Anchors, EN 1992-4 (`CHK/⚓️anchor-evaluation/🦀️.rs`)

- Action combination (`combine_anchor_actions`, lines 6-90): N = N_k + |g_line|+|q_line| (udl) or + |point| (point) or + both (other sources); plus |M_k|/h_ef (lines 25-27) and V = V_k + |T_k|/c1 (lines 28-30) + |V_k,punch|. These are ad hoc conversions of moment and torsion to normal and shear forces. Combinations: ULS-6.10-G, ULS-6.10b-<id>, SLS-qp-<id>, ACC-6.11-<id> with γG 1.35, γQ 1.5, ψ factors as Section 2.3.
- Resistances (`CHK/🧰️helpers-and-annex-parts/🦀️.rs` 589-640; anchor evaluation 93-290):
  - `en1992-4.steel.*` (§7.2.1.4): N_Rd,s = min(A_s·f_uk/γ_Ms, A_s·f_yk/1.2) (helpers 595-599). γ_Ms = 1.4 if f_uk/f_yk ≤ 1.25 else 1.5 (helpers 591-593). The 1.2 is hard-coded.
  - `en1992-4.steel.sls.*`: N_Rd,s,SLS = A_s·f_yk/1.15 (anchor line 131).
  - `en1992-4.cone.*` (§7.2.1.5): N_Rk,c = k·√f_ck·h_ef^1.5, k = 11 uncracked, 7.5 cracked, divided by γ_Mc = 1.5 (helpers 601-606). Remedy h_ef·(N/N_Rd)^(2/3)·1.05 (anchor 158).
  - `en1992-4.pullout.*` (§7.2.1.6): N_Rd,p = A_s·f_uk/1.5 (helpers 609-611). This is a stand-in, not the bond/τ formulation of EN 1992-4 §7.2.1.6.
  - `en1992-4.splitting.*` (§7.2.1.7): N_Rd,sp = 8·√f_ck·h^1.5·min(c1/(1.5h), 1)/1.5 (helpers 614-619). Ad hoc scaling.
  - `en1992-4.pryout.*` (§7.2.2.4): N_Rd,cp = 2·N_Rd,c (helpers 622-624). The fixed factor 2 is an approximation. The check compares V_Ed, not N_Ed (anchor line 215).
  - `en1992-4.edge.*` (§7.2.2.5): V_Rd,c = 1.7·√d·h^0.2·√f_ck·c1^1.5/γ_Mc (helpers 626-633). Simplified, not the EN expression.
  - `en1992-4.interaction.*` (§7.2.3): (N/N_Rd)^1.5 + (V/V_Rd)^1.5 ≤ 1, with V_Rd = min(V_Rd,c, N_Rd,p) (helpers 636-640, anchor line 249). The exponent 1.5 is the EN form; the choice of N_Rd,t as min of steel, cone, pullout and splitting is a simplification.
- Missing: steel shear resistance V_Rd,s (§7.2.2.3), group effects, multi-anchor distribution, edge-with-group, concrete edge with reinforcement, and anchor spacing rules.
- Remedies use fixed heuristics (pry-out × 1.2, interaction × 1.15, etc.).

### 2.14 Not implemented at all (EN part coverage gaps)

- EN 1992-1-1: strut-and-tie (§6.5), partially loaded areas (§6.7), shear with axial and stress (§6.2.3), openings, D-regions (§10), walls (§9.6), deep beams (§9.7), foundations (§9.8), bar spacing and detailing rules (§8.2, §9.5), time-dependent effects and loss calculation (§5.10.5, §3.1.4), second-order via method based on nominal stiffness (§5.8.7) and general method (§5.8.6), SLS calculated deflection (§7.4.3), crack control with spacing detailing beyond the single formula.
- EN 1992-1-2: thermal and mechanical calculations (only tables).
- EN 1992-2: detailed fatigue verification (§6.8), shear and torsion bridge specifics.
- EN 1992-3: only the crack-width tightness check with user inputs.
- EN 1992-4: see 2.13.

---

## 3. NA-DE parameters actually used

Source: `CHK/🧰️helpers-and-annex-parts/🦀️.rs:61-117` (module `na_de`) and direct constants in the check modules.

| Parameter | DE value | EN value | Where used |
|---|---|---|---|
| α_cc | 0.85 | 1.00 | `AnnexParams::de/en`, `f_cd_pa` (all flexure, shear, punching max, torsion, anchors via `f_cd`) |
| α_ct | 0.85 | 1.00 | Stored in struct only; never read (dead). |
| γ_c (persistent) | 1.50 | 1.50 | `AnnexParams::de/en`; used for f_cd and C_Rd,c |
| γ_s | 1.15 | 1.15 | f_yd (`f_yd_pa`) for flexure, shear reinforcement, prestress checks |
| γ_c (accidental) | 1.30 | 1.20 | `AnnexParams::accidental`, lines 86-92 |
| γ_s (accidental) | 1.00 | 1.00 | same |
| C_Rd,c (shear and punching) | 0.15/γ_c | 0.18/γ_c | helpers 180-183, 213-216 |
| cotθ | clamp(1.2 + 0.2·σ_cp/f_cd, 1.0, 3.0) | constant 2.5 (line 113) | helpers 107-115 |
| Punching v_Rd,max | 1.4·v_Rd,c | 2.0·v_Rd,c (not an EN expression) | helpers 222-227 |
| Punching c_Rd,c u0/d factor | (0.5 + 0.125 u0/d), capped at 1, for u0/d < 4 | none | resistance 170-172 |
| Basic l/d DE cap | min(K·35, K²·150/l) | none | helpers 386-389 |
| λ_lim (columns) | 25 if n ≥ 0.41 else 16/√n | not separate (uses the same function) | helpers 397-399 |
| w_max (quasi-permanent) | 0.40 mm X0/XC1; 0.30 mm others | same | helpers 287-293 |
| c_min,dur structural class shift | -1..+2, implemented in code | same mechanism | helpers 342-359 |
| Fire restrictions | described in module doc only, no separate code | | fire tables |

The DE/EN switch changes α_cc and γ, C_Rd,c, cotθ, punching max, and accidental factors. Several DE specific items above are labelled "DE" in the code but not traced to NA-DE clauses in this audit.

---

## 4. Editor

Source root: `ANY/✏️editor/`.

### 4.1 Commands (`ANY/✏️editor/🦀️.rs` lines 62-72; `ANY/✏️editor/🎮️commands/`)

| Command | Keyword | Payload | Behaviour |
|---|---|---|---|
| evaluate | `evaluate` | none | Returns `Emit::default()`. Results are recomputed by rendering, not by a mutation. Action kind View, interactive job. |
| setSelectedCheckIndex | `selected-check` | index: Option<u32> | Changes only the results-window config (inspection panel). |
| setField | `set-field` | path: String, value_json: String | Resolves an edit via `semio_s_artifact_norm_en1992_mutations::resolve_edit`. |
| insertItem | `insert-item` | path, index, value_json (optional) | Inserts into a collection via the mutations crate. |
| removeItem | `remove-item` | path, index | Removes from a collection. |
| applyRemedy | `apply-remedy` | check_id, remedy_index | Applies a remedy from a check via `dispatch_apply_remedy` and the mutations crate. |

Keybindings: `mod+z` undo, `mod+shift+z` redo. The `editor` declares tools `evaluate`, `setSelectedCheckIndex`, `setField`, `insertItem`, `removeItem`, `applyRemedy` (line 123).

Edit-mode command shim (`ANY/✏️editor/🎮️commands/🧮️evaluate/`) and the `selected-check` command have unit tests. The remaining command files are 21-37 lines each.

### 4.2 Field metadata (`ANY/✏️editor/🏷️field-meta/🦀️.rs`, 272 lines)

- English and German labels for every editable leaf path, using template paths such as `members[].actions[].gKLine` and `anchors[].fUk`.
- Choice lists: annex (En/De), member kind, support, exposure, fire rating, slab system, column method, column position, ductility class, action source, action category (with ψ-relevant categories office/residential/congregation/shopping/storage/snow/wind/accidental), bond condition, prestress, tightness, and so on.
- Unit display via `NormFieldMeta` (SI, shown beside numeric values).
- Lookup with longest prefix at a `.` boundary (`lookup_norm_field_meta`, from the shared `🔖️field-meta` module).

### 4.3 Panels and windows (`ANY/✏️editor/📌️panels/`, `ANY/✏️editor/🎭️modes/✏️edit/🪟️windows/`)

Default layout (`ANY/✏️editor/🎭️modes/✏️edit/🦀️.rs:15-17`): a row split 42 % / 58 %, Inputs left, Results right.

- Window "Inputs" (`norm-en1992-inputs`, body `norm.en1992.play.inputs`, `ANY/✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs`). Renders the whole `En1992Snapshot` with `render_document_editor` (shared `📝️document-editors` in the norm app surface, see 4.4). This is a structured property tree: objects and arrays as collapsible groups with counts, and per leaf a control chosen by type. Bool is a select (true/false), number is a numeric input with SI unit text, string is either a select (when field-meta has choices) or a text input committed on blur, bytes and null are read-only labels. It is not a raw JSON editor. The raw JSON renderer `render_document_json` exists in the shared surface but is not used by this artifact.
- Window "Results" (`norm-en1992-results`, body `norm.en1992.play.results`, `…/📊️results/🦀️.rs`). Calls `render_report` on the report computed by `NormHost`. Layout: grouped by part; each check row shows label, status and utilisation, and its remedies as children. Each applicable remedy is a button bound to `applyRemedy` with check id and index; non-applicable remedies show "Not auto-applicable". A check row is selectable and drives the inspection panel.
- Panel "Artifact" (`📌️panels/🗿️artifact`, body `norm.en1992.play.artifact`, Workbench group): summary headline with verdict (complies / does not comply) and per-part marks (✓ / ✗) from `render_summary`.
- Panel "Catalogue" (`📌️panels/📚️catalogue`, body `…catalogue`): example picker over five examples (see Section 7) plus optional normative reference tables (`CatalogueTable`, rendered windowed; no tables are supplied by this artifact).
- Panel "Inspection" (`📌️panels/🔍️inspection`, body `…inspection`): details of the check at `selected_check_index` (default first).
- No 2D or 3D views of any kind (no M-N curves, stress blocks, member sketches, load paths, utilisation heat maps). The string "3d" in two test files refers to framework test names only.

### 4.4 Shared norm surface (`📇️registry/🧬️contract/🖥️app-surface/`)

Outside the artifact but used by it: `📝️document-editors` (property tree), `🌳️report-tree` (results rows), `🏷️ids-chrome-and-labels` (report table columns), `✏️edit-rules`, `🧵️retained-commands`, `🪟️manifest-and-media`.

---

## 5. Viewer

Source root: `ANY/👁️viewer/`.

- One mode, `view` (`👁️viewer/🎭️modes/👁️view/`), no commands except an inert `Noop` variant. The viewer is read-only by construction.
- One window, "Report" (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`). Recomputes `evaluate(document)` on every render and shows a `TableWindowKit` table (the framework table window, not a canvas or diagram).
- Table columns (from `report_table_columns`, `ANY/✏️editor/…` shared `🏷️ids-chrome-and-labels/🦀️.rs:49-60`): Part, Clause, Subject, Status, Utilization, Title, Remedy. One row per check. The Remedy cell shows only the first remedy's action text.
- Typed TypeScript mirror (`🟦️.ts`): `En1992ViewReportRow { clause, status, utilization, message }`. The Rust table has no `message` column, so the TS mirror and the Rust table differ.
- Viewer manifest: icon `check-circle`, mode `view`, window kind `framework.window.table`, default layout `view::layout()`.

---

## 6. Tests and oracles

### 6.1 Rust tests

Counted `#[test]` and `async_test` per file (total about 130 across the artifact). Main groups:

- `ANY/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (26 tests, 801 lines): numeric worked examples (`shear_vrdc_worked_example_de`, lines 13-19: V_Rd,c ≈ 69.9 kN DE and 83.9 kN EN ±2 kN), flexure DE vs EN (`flexure_de_vs_en_divergence`, lines 22-30: M_Rd,DE ≈ 208 kNm ±40 kNm, a 19 % tolerance), `cot_theta_de_limits` (lines 33-39, which contains a tautological assertion `cot_hi <= 3.0`), fire R60 axis distance (lines 43-48), compliant frame passes and failing frame has ≥ 3 fails with applicable remedies, remedy-law flip-to-pass tests, Python oracle parity (lines 305-349), jsonschema validation of examples (lines 352-372, Python `jsonschema`), field-meta coverage, every editable leaf influences a check (lines 543-595), and example evaluations.
- `ANY/🧬️schema/💡️inferences/🧪️tests/` and `🧾outline/🧪️tests/`: compliance-report and outline unit tests.
- `ANY/🧬️schema/🧬️mutations/🧪️tests/` and `🔺️diff/🧪️tests/`: mutation and diff laws.
- `ANY/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/` (13 tests), `ANY/🚪️io/📝️text/📸️snapshot/🧪️tests/` (grammar conformance and unit), `ANY/🚪️io/💾️binary/📸️snapshot/🧪️tests/`.
- Editor: `ANY/✏️editor/🧪️tests/`, panel tests, window tests, command tests (about 35 files total with the sub-windows).
- Viewer: `ANY/👁️viewer/🧪️tests/` and report window tests.
- Examples: `ANY/📚️examples/*/🧪️tests/🧩️example/` (one test file per example; the liquid-retaining example has 3).

Weak or self-referential tests (to be treated as weak, not as validation):

- `de_na_material_factors_and_cover_tables` (compliance 232-247) asserts the implemented constants against themselves.
- `c_min_dur_xc3_de` asserts 20 mm and 25 mm, the code's own values.
- `cot_theta_de_limits` contains `|| cot_hi <= 3.0`.
- Flexure and shear numeric tests have ±19 % and ±3 % tolerances.

### 6.2 Python oracles

1. `ANY/🧬️schema/🧪️tests/⚖️compliance/🐍️.py` (583 lines, stdlib only: `json`, `math`, `sys`, `typing`). This is a second implementation of `evaluate` (design effects, shear, flexure, crack, deflection, column, prestress, anchorage, lap, fire, punching). It emits check ids and utilisations. The Rust test `python_oracle_matches_utilizations_within_half_percent` runs it on the compliant and failing examples and requires agreement within 0.5 % relative or 0.005 absolute, with at least min(len, 10) overlapping checks. It is a mirror of the same author's formulas, so it does not validate the formulas against the standard. It also implements some of the same defects.
2. `ANY/🔮️oracles/🔣️.json` (989 lines, schema version 2): one oracle entry `en1992-1-python-independent`, ecosystem python, `package: ""`, `testOnly: true`, `productionReachable: false`, `networkDuringExecution: false`, engine family `none`, implementation "in-repository second implementation", specification sources `📓️taxonomy.md; 📓️derivation-rules.md; .dsl.semio`, 30 vectors, capability `en1992-1-mutate`, comparison profile `ordered-json-v1`, license AGPL-3.0-only. The rationale explicitly states that no third-party Python library (structuralcodes, concreteproperties, anastruct) reads or writes the artifact format, and that the Python side is a second implementation, not a third-party library. It also lists the mutation catalog (28 kinds) and 28 mutation manifests.
3. `ANY/🧪️tests/🏛️mutate-en1992-1/🐍️.py` (115 lines) and `🥒️.feature` (129 lines, 3 scenario outlines: apply, undo/inverse, identity round-trip). The Python file imports `semio_norm_vocabulary` through `import_module("🐍️")`. That module is the repository's own shared norm reference engine (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), not a third-party package. It checks mutation vocabulary only (apply, inverse, refusal rows), not design formulas. The 28 `✅apply` vectors and 2 refusal rows cover every mutation kind.
4. `ANY/🧪️tests/🏛️mutate-en1992-1/🦀️.rs` (490 lines): the Rust side of the same differential feature.

### 6.3 Third-party libraries

- Python `jsonschema` (test only, compliance test lines 352-372). Validates the example snapshot against `ANY/🧬️schema/📸️snapshot/🔣️.json`.
- Python: no numerical or structural library (no numpy, scipy, structuralcodes, concreteproperties, anastruct) is imported anywhere in the artifact.
- Rust: `serde_json` in tests only (for oracles and the JSON projection). Runtime uses the in-repo framework crates.
- No third-party EN 1992 oracle exists for any formula. The "independent" oracle is an in-repo re-implementation.

---

## 7. Examples and fixtures

### 7.1 Examples (`ANY/📚️examples/`, mounted in `ANY/📚️examples/🦀️.rs`)

| Example | Files | Snapshot constructor | Intent |
|---|---|---|---|
| `🏢️compliant-office-frame` | `🦀️.rs`, test | `compliant_office_frame()` | DE NA, multi-member RC office frame, expected to pass |
| `❌️failing-under-reinforced` | `🦀️.rs`, test | `failing_under_reinforced()` | multiple failures with remedies |
| `🧵️compliant-prestressed-beam` | `🦀️.rs`, test | `compliant_prestressed_beam()` | prestressed beam expected to pass |
| `💥️failing-prestressed-beam` | `🦀️.rs`, test | `failing_prestressed_beam()` | prestressed beam expected to fail |
| `🛢️liquid-retaining-fem-anchor` | `🦀️.rs`, `🟦️.ts`, tests, `🖼️assets` (`📡️…spr.semio`, `🔧️…op.semio` binary/text carriers), `🟦️.ts` | `liquid_retaining_fem_anchor()` | liquid-retaining wall with FEM actions and anchor |

Pack and DSL carriers for each example are in `ANY/🖼️assets/` (`🎒️.pack.semio` and `🗣️.dsl.semio` per example).

Editor demo: `ANY/✏️editor/📚️examples/🎬️demo-session/` with `🖼️assets/🎮️.cmd.semio` (a command script), `🦀️.rs`, `🟦️.ts`, and a test.

### 7.2 Fixtures

- `ANY/🧬️schema/🧬️mutations/…` and `ANY/🧪️tests/…` mutation fixtures: 166 tracked fixture files including the 28 kinds, before/after snapshots, mutation payloads, diffs, and inverse documents, each with a `✅apply` vector and two refusal vectors where applicable.
- `ANY/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json`: sqlite round-trip fixture.
- Shared specification examples: `ANY/🖼️assets/*` (5 example packs).

### 7.3 IO

- Text carrier (`ANY/🚪️io/📝️text/`): snapshot, diff, mutations, inferences encoded through a grammar (`📖️.grammar.semio`, `🅰️.g4`, `🔤️.ebnf`) and text codec. The grammar is documented as a `payload = OCTET+` placeholder (see the feature file line 31).
- Binary carrier (`ANY/🚪️io/💾️binary/`): kaitai (`🥋️.ksy`), abnf, protocol semio definitions.
- sqlite (`ANY/🚪️io/🪶️sqlite/`): SQL and TypeScript declarations for the snapshot cells.
- Five degenerate media formats (csv, json, txt, xlsx, zip) were removed from the IO surface.

---

## 8. Obvious gaps, stubs, TODOs and ad hoc shortcuts

### 8.1 Probable formula errors (fix first)

1. Second-order curvature inverted by factor 0.45² (`CHK/🧰️helpers-and-annex-parts/🦀️.rs:401`). Should be f_yd/(E_s·0.45·d). Also feeds `cot`-free M_2 for all columns.
2. `basic_ld_limit` uses √500 (f_yk reference) where the expression needs √f_ck (helpers 257-258, 262, 264). Deflection limits are wrong for every fck. Compression steel ρ' is always 0 (resistance line 351).
3. Interior punching β = 1.10 (helpers line 407) against the EN default 1.15 (verify against NA-DE).
4. EN punching v_Rd,max = 2.0·v_Rd,c (helpers 225), not an EN expression.
5. EN cotθ fixed at 2.5 (helpers 113), not free in [1, 3] as the doc comment says.
6. Punching shear reinforcement term 0.75·asw·f_yd/(u1·d) (resistance line 179) does not match §6.4.5 Eq. 6.52 (no 0.75 factor on v_Rd,c; no 1.5·d/s_r term). `asw` units need checking.
7. Anchorage f_bd uses f_ctk,0.05 with no γ_c (anchorage file line 64, helpers 373). §8.4.2 uses design f_ctd.
8. Fire min dimension uses width for slabs (fire line 29).
9. Lap α6 and anchorage α1 hard-coded (fire lines 67, 93).
10. Design effects: udl counted once per permanent action (design-effects line 34).
11. Design effects: point-load moment formula assumes simply supported midspan for every support (design-effects 45-49).
12. Prestress σ_c sign: eccentricity absolute (slenderness line 110).
13. Decompression utilisation uses 0.45 f_ck as denominator (slenderness line 137) and a fixed 1.1 remedy factor.
14. B500C f_yk = 550 MPa (model line 571). DE B500 is normally 500 MPa (verify).
15. Constitutive check (cover-and-constitutive 48-49) is not a standard check.
16. Shear with no stirrups checks V_Ed against V_Rd,c only, and the V_Rd,max cap is ignored in that branch (resistance 96).
17. Crack width: φ = 16 mm and k1, k2, 0.425 hard-coded (resistance 257-262); α_e = E_s/22 GPa (helpers 245).

### 8.2 Ad hoc constants and heuristics

- Axial reduction of M_Rd by (1 − N/N_Rd) (helpers 160-166).
- Accidental M_Rd by f_cd ratio (resistance 66).
- Torsion via thin-walled formula on a solid rectangle (helpers 230-238).
- Column λ_lim = 16/√n for n < 0.41 (helpers 397-399).
- Structural class heuristics (helpers 342-352), cement "R" test by substring.
- Loss limit 30 % (slenderness line 116), loss clamp 0.5 (line 100).
- Fatigue limit 162.5/1.15 MPa (helpers 565).
- Tightness TC2 linear interpolation (helpers 577-580).
- Pullout, splitting, edge and pryout formulas in EN 1992-4 are approximations with fixed constants (helpers 609-633).
- Remedy factors (1.05, 1.15, 1.2, 1.25) and fixed increments (e.g. +5e-4 m² asw, bar diameter −2 mm).
- Default material fallbacks "c30", "b500", "yp1860" when ids are missing (member-eval 36, 53; slenderness 90).
- The `harsh` branch in SLS concrete check is dead (resistance 400, 425).
- Unused values and code: `_v_rd` (resistance line 95), `_ok_cap` (slenderness line 36), `alpha_ct` (never read).
- `bond_condition`, `position`, `kind`, `category`, `source`, `column_method`, `slab_system`, `column_position` are plain strings matched by literal value (no enum), which weakens validation. Unknown values fall through to defaults silently (for example `psi_factors` default = office, `column_method` any non-"B" = A).

### 8.3 Stubs and placeholders

- `evaluate` command is a no-op returning `Emit::default()` (editor commands/evaluate).
- `render_document_json` (shared surface) is unused by this artifact.
- Viewer `Noop` command (intentional but inert).
- `field_metadata()` in `CHK/🦀️.rs:40-42` returns an empty slice (superseded by `en1992_field_meta`).
- Fire docstring says "DE NA restrictions" but no separate DE code path exists.
- Feature file and Python oracle depend on a placeholder carrier grammar.

### 8.4 Missing or weak items relative to the objective

- No visualisation: no 3D (members, sections, joints, foundations, load paths, failure modes, utilisation heat maps) and no 2D (M-N interaction, stress blocks, spectra, S-N curves). Viewer is a table only.
- Editor is a generic property tree, not a direct-manipulation editor. No live viewport, no inline utilisation beside inputs (utilisation only in the results list).
- `evaluate` is registered as an interactive job in the manifest (editor `🦀️.rs` line 249), but the artifact code itself reports no progress and offers no cancellation. Evaluation is a single synchronous pass.
- Inputs are many free-form strings for classifications (position, bond, kind, category, source, column method, slab system). Enums would be cleaner.
- Concrete parameters are derived from fck only (no fck-dependent E_cm override, no f_ctm override).
- `DE NA` items other than α_cc, γ and cotθ are not separately traced (snow ψ, punching DE branch, λ_lim, l/d caps).
- Structural class and c_min,dur tables not traced to the printed tables.
- Load combinations: no favourable γG, no 6.10 with multiple leading actions envelope beyond the simple case, no shear/torsion/axial envelope.
- Member kinds `TensionMember`, `Bridge`, `LiquidRetaining`, `RibbedSlab` have limited or no checks specific to them (for example, deflection excludes RibbedSlab, Bridge and LiquidRetaining; tension members get only fire and tie checks through common paths).

---

## 9. Quick reference: files of interest

- Schema: `ANY/🧬️schema/📸️snapshot/🦀️.rs` (model), `ANY/🧬️schema/🦀️.rs` (artifact), `ANY/🧬️schema/🔣️.json`, `ANY/🧬️schema/🔺️diff/🦀️.rs`, `ANY/🧬️schema/🧬️mutations/🦀️.rs`
- Checks: `ANY/🧬️schema/💡️inferences/🦀️.rs`, `CHK/🦀️.rs`, `CHK/🧮️design-effects/🦀️.rs`, `CHK/⚓️anchor-evaluation/🦀️.rs`, `CHK/📋️member-evaluation/🦀️.rs` (+ 4 submodules), `CHK/🧰️helpers-and-annex-parts/🦀️.rs`
- Editor: `ANY/✏️editor/🦀️.rs`, `ANY/✏️editor/🏷️field-meta/🦀️.rs`, `ANY/✏️editor/🎮️commands/*`, `ANY/✏️editor/📌️panels/*`, `ANY/✏️editor/🎭️modes/✏️edit/🪟️windows/*`
- Viewer: `ANY/👁️viewer/🦀️.rs`, `ANY/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`
- Oracles: `ANY/🧬️schema/🧪️tests/⚖️compliance/🐍️.py`, `ANY/🔮️oracles/🔣️.json`, `ANY/🧪️tests/🏛️mutate-en1992-1/`
- Examples: `ANY/📚️examples/`, `ANY/🖼️assets/`
- Shared surface (outside artifact): `/home/deck/semio/✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/`, `/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/`
