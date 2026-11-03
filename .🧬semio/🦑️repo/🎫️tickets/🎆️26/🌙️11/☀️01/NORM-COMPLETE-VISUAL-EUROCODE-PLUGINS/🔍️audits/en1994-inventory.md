# 🧩 EN 1994 Inventory

📌 Read-only audit of the EN 1994 artifact as it exists in the working tree. No source file was modified, and no build or test was executed.

**Root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994`
**Subset:** `🏅️standards/🔖️1/🪆️subsets/✳️any` (abbreviated `ANY/` below)
**Shared kernel:** `✏️s/🔌️plugins/📕️norm/⚖️compliance` (AnnexChoice, CheckResult, Remedy, ClauseId)
**Tracked files:** 524 non-placeholder files (`📌️.empty.md` excluded). 140 of them are mutation fixtures.

Path conventions: `S` = `ANY/🧬️schema/`, `C` = `S/💡️inferences/⚖️checks/🦀️.rs`, `I` = `S/💡️inferences/🦀️.rs`. Line numbers refer to the file as it is now.

---

## 0. Layout at a glance

| Family | Location | Role |
|---|---|---|
| Model | `S/📸️snapshot/🦀️.rs` (537 l.) | Snapshot entities, presets, catalogue steel |
| Artifact | `S/🦀️.rs` (95 l.) | `En1994Artifact` (same 9 fields as the snapshot), conversions, descriptor |
| Checks | `C` (969 l.) | Resistance helpers per EN part, including NA-DE constants |
| Inferences | `I` (963 l.) | `evaluate(snapshot) -> CheckReport`, every check with its clause, remedies and fire/bridge branches |
| Outline inference | `S/💡️inferences/🧾outline/🦀️.rs` | Metadata only (field count, entry count). It computes no compliance. |
| Diff | `S/🔺️diff/` | `En1994Diff`, beam/column/slab row diffs |
| Mutations | `S/🧬️mutations/` | `En1994Mutation`, 25 kinds, `KINDS`, edit rules |
| IO | `🚪️io/` | Text DSL (`.dsl.semio`), binary (`.pack.semio`), SQLite snapshot codecs |
| Editor | `✏️editor/` | `En1994PlayApp`, 6 commands, field-meta, inputs/results windows, 3 panels |
| Viewer | `👁️viewer/` | `En1994Viewer`, one read-only Report table |
| Examples | `📚️examples/`, `✏️editor/📚️examples/`, `ANY/🖼️assets/` | 3 model examples, 1 stub demo |
| Oracles | `ANY/🔮️oracles/` | Python `⚖️compliance/🐍️.py`, `🧬️snapshot-schema/🐍️.py`, `🔣️.json` registration |
| Tests | `ANY/🧪️tests/`, `S/🧪️tests/`, and `🧪️tests/🔬️unit` dirs | Mutation feature and Python differential, compliance report, checks unit tests |

Packages: 1 root workspace (`Cargo.toml`) with 16 member manifests. The only external crates are `serde` and `serde_json`. Both are used only under `cfg(test)` or in dev-dependencies. The runtime uses only path dependencies on `🧰️framework` and `⚖️compliance`. There is no runtime third-party dependency.

---

## 1. Model

### 1.1 Snapshot (`S/📸️snapshot/🦀️.rs`), the persisted subject

| Entity | Fields (unit) | Lines |
|---|---|---|
| `En1994Snapshot` | `annex: AnnexChoice {En,De}`, `structure_kind: String` (`building`/`bridge`), `steel_f_y_pa: f64` [Pa], `beams: Vec<CompositeBeam>`, `columns: Vec<CompositeColumn>`, `slabs: Vec<CompositeSlab>`, `fire_rating: String` (`r30..r120`), `insulation_thickness_m: f64` [m], `fatigue_detail: String` | 23-47 |
| `SteelSection` | `designation`, `height_m`, `width_m`, `tw_m`, `tf_m`, `a_m2`, `w_pl_y_m3`, `i_y_m4`, `a_v_m2` | 128-146 |
| `ProfiledSheeting` | `profile: String` (`trapezoidal`/`re-entrant`), `height_m`, `rib_width_m`, `thickness_m`, `ribs_parallel_to_beam: bool` | 154-163 |
| `HeadedStuds` | `diameter_m`, `height_m`, `f_u_pa`, `count_per_rib: u32`, `spacing_m`, `total_count: u32` | 171-182 |
| `CharacteristicAction` | `id`, `kind` (permanent/imposed/snow/wind/construction/fatigue), `category` (self/A–H/flm3), `stage` (construction/composite), `q_area_pa`, `f_k_n`, `delta_sigma_k_pa`, `delta_tau_k_pa` | 195-213 |
| `ColumnAction` | `id`, `kind`, `category`, `stage`, `n_k_n`, `m_k_nm` | 221-232 |
| `CompositeBeam` | `id`, `span_m`, `spacing_m`, `support` (simply_supported/continuous_2_span), `construction` (propped/unpropped), `steel: SteelSection`, `slab_thickness_m`, `concrete_f_ck_pa`, `concrete_e_cm_pa`, `sheeting: ProfiledSheeting`, `studs: HeadedStuds`, `transverse_as_m2_per_m`, `ltb_length_m`, `as_hogging_m2_per_m`, `bar_spacing_m`, `wk_limit_m`, `n_cycles`, `actions: Vec<CharacteristicAction>` | 240-269 |
| `CompositeColumn` | `id`, `kind` (encased/concrete_filled/partially_encased), `length_m`, `outer_size_m`, `wall_thickness_m`, `steel_a_m2`, `steel_f_y_pa`, `concrete_a_m2`, `concrete_f_ck_pa`, `reinforcement_as_m2`, `reinforcement_f_yk_pa`, `i_m4`, `buckling_curve` (a/b/c/d as string), `actions: Vec<ColumnAction>` | 277-305 |
| `CompositeSlab` | `id`, `span_m`, `support`, `sheeting`, `concrete_thickness_m`, `f_ck_pa`, `m_factor`, `k_factor`, `as_m2_per_m`, `actions: Vec<CharacteristicAction>` | 313-329 |

Other items in the snapshot file:
- `Default` (49-70): annex De, building, 355 MPa, `beam-B1` (HEB300, 8 m span, 3 m spacing, propped construction), `col-C1`, `slab-S1`, r60, 25 mm insulation, `stud_welded`.
- Presets (73-110): `unpropped_building`, `custom_plate_building`, `bridge_girder` (FLM3, 25 m, 2e6 cycles, Δσ 65 MPa, Δτ 40 MPa), `fire_demanding` (r120, 10 mm).
- `SteelSection::from_catalogue` (343-388): 11 HEB entries, HEB300 to HEB700. `resolve()` (391-393) replaces stored geometry with catalogue geometry whenever the designation matches. `custom_plate()` (396-402). `heavier_heb_options` (405-410). `h_w_m` (413-415) = h − 2·tf. `shear_area_from_plates_m2` (418-420) = max(h·tw, a_v).
- `ProfiledSheeting::a_p_m2_per_m` (429-431) = `thickness × 1.2`. The factor 1.2 is a hard-coded developed-profile approximation.
- `HeadedStuds::typical` (435-437): 19 mm, 75 mm, 450 MPa, 72 studs, 150 mm spacing.
- Presets for actions (440-477) and placeholders for beam/column/slab (480-525): span 8 m, spacing 3 m, slab 140 mm, fck 30 MPa, Ecm 33 GPa, LTB 4 m, A_s,hog 5e-4 m²/m, bar 150 mm, wk 0.3 mm, 2e6 cycles.
- Artifact-level constants: `artifact_kind()` (528-530), `EN1994_DIALECT` (535), `EN1994_DOCUMENT_SCHEMA` = `semio.norm.en1994/v1`.

### 1.2 Artifact (`S/🦀️.rs`)

- `En1994Artifact` (25-44) mirrors the snapshot's nine fields, all `#[state(artifact)]`. No presence lane.
- `to_snapshot` / `from_snapshot` (49-79).
- `en1994_artifact_schema_descriptor` (85-93) registers the snapshot, diff and mutation leaves. The comment says "twenty handcrafted schema leaves": 5 facets for each of artifact, snapshot, diff and mutations.

### 1.3 Compliance and computation types (no persistence)

- `AnnexChoice` (shared kernel `⚖️compliance/📜️annex-and-tables/🦀️.rs:10-15`): `En`, `De`.
- `AnnexParams` (`C` 22-51): `choice`, `gamma_v`, `gamma_c`, `gamma_s`, `gamma_m0`, `gamma_m1`, `gamma_mf`.
- `DesignEffects` (`C` 62-70, `part_en1990`): `m_nm`, `m_hog_nm`, `v_n`, `n_n`, `label_en`, `label_de`.
- `NmPoint` (`C` 711-714): `n_n`, `m_nm`.
- `FireRating` (`C` 599-601): `R30`, `R60`, `R90`, `R120`.

### 1.4 Inference (`I`, `🧾outline`)

- `En1994Inference { outline: En1994Outline }` (I 27-30), with `infer` and `InferenceSpec` (I 32-51).
- `En1994Outline { section_outline: Vec<String>, field_count: u32, entry_count: u32 }` (`🧾outline/🦀️.rs` 23-27). It holds only metadata.
- Naming note: the directory `💡️inferences/` contains the real compliance `evaluate()` (I 839-951). The `outline` inference, the only named inference registered in the schema, computes no compliance.

### 1.5 Mutations (`S/🧬️mutations/🦀️.rs`)

`En1994Mutation` has 25 variants (lines 51-77) and a `KINDS` list. The variants are:
- Scalars: `ChangeAnnex`, `ChangeStructureKind`, `ChangeSteelFYPa`, `ChangeFireRating`, `ChangeInsulationThicknessM`, `ChangeFatigueDetail`.
- Beams: `InsertBeam`, `RemoveBeam`, `ChangeBeamActionQAreaPa`, `ChangeBeamStudSpacingM`, `ChangeBeamSpanM`, `ChangeBeamSlabThicknessM`, `ChangeBeamStudDiameterM`, `ChangeBeamStudCount`, `ChangeBeamStudFUPa`, `ChangeBeamTransverseAs`, `ChangeBeamConstruction`.
- Columns: `InsertColumn`, `RemoveColumn`, `ChangeColumnActionForceN`, `ChangeColumnKind`.
- Slabs: `InsertSlab`, `RemoveSlab`, `ChangeSlabActionQAreaPa`, `ChangeSlabThicknessM`.

Mutation families such as `🌉️beams`, `🏛️columns-slabs`, `📌️studs` and `🏗️project` each have their own crate. The `🧰️testkit` provides the default placeholders. `🧭️edit-rules` maps editor paths to kinds (see §4.3).

### 1.6 Diff (`S/🔺️diff/`)

`En1994Diff` (`🦀️.rs` 42) with a `structure_kind` option and per-family row diffs (`🏗️beam-rows`, `🏛️column-rows`, `🧱️slab-rows`), plus `🔁️apply-and-inverse`.

### 1.7 Schema facets

Each entity family has five hand-written facets: `🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json` (JSON Schema), `🛰️.proto`. The snapshot's `🔣️.json` is 1334 lines. The TypeScript facet is checked by a test. Several domain enumerations are kept as strings in the model, not as typed enums: `structure_kind`, `fire_rating`, `fatigue_detail`, `support`, `construction`, column `kind`, `buckling_curve`, action `kind`/`category`/`stage` and sheeting `profile`.

---

## 2. Checks and verifications

### 2.0 Mechanics

- `evaluate(&En1994Snapshot) -> CheckReport` (I 839-951) is the single entry point. It is called by `NormFamily::evaluate` in the editor (`✏️editor/🦀️.rs` 226-228), by the viewer's report window (`👁️viewer/.../📊️report/🦀️.rs` 31), and by the compliance-report tests.
- Per beam: push dual-source action checks (I 849-851), then `evaluate_beam` (I 97-117).
- Per column: `evaluate_column` (I 644-697).
- Per slab: dual-source action checks (I 856-858), then `evaluate_slab` (I 699-795).
- Fire and fatigue are run once for the document (I 862-949).
- If the document has no members, the output is a single `en1994.subject.empty` N/A (I 842-847).
- Remedies are generated per failing check. Each remedy has a target path with stable ids (`beams[id=…].…`) and an SI value. Kinds are `at_least`, `at_most` and `one_of` (`Remedy::*`).
- The check `status` is Pass, Fail, Warning or N/A, and `utilization` is a dimensionless ratio. The `part` label is always `"DIN EN 1994-1-1"` for members, regardless of annex (I 122, 140, 194 and so on). Fire checks use `"DIN EN 1994-1-2"` and bridge checks use `"DIN EN 1994-2"`.

### 2.1 EN 1994-1-1, composite beams (per beam)

Helpers are in `C`, `part_1_1` (348-594). The check bodies are in I, `evaluate_beam` and its helpers (I 97-642).

| Check id | Clause | Formula and source | Where | Notes |
|---|---|---|---|---|
| `en1994.5.4.1.2.beff.<id>` | §5.4.1.2 | `b_eff = min(2·be1, 2·be2, spacing)` with `be1 = L/8 + b0/2`, `be2 = spacing/2` (C 352-356). Equivalent for simply supported uniform cases to `min(b0 + L/4, spacing)`. Util = `(b0 + L/4) / spacing` (I 120-136). | I 120-136; C 352-356 | The check FAILS when `b0 + L/4 > spacing` and suggests increasing spacing (I 132-135). EN caps b_eff at the spacing in that case, so a normal capped beam is reported as non-compliant. No continuous-span Le reduction. |
| `en1994.5.5.class.<id>` | §5.5, EN 1993-1-1 T5.2 | Flange `c/tf` vs 9/10/14·ε. Web `hw/tw` vs 72/83/124·ε with ε = √(235/fy) (C 507-520). Util = class/3 (I 138-169). | I 138-169; C 507-520 | Stored geometry is used after `resolve()`. Remedy for class > 3: catalogue HEB options, or `tw × 1.2`. |
| `en1994.6.2.1.3.mrd.<id>` | §6.2.1.3 | `M_Rd(η) = M_pla + η·(M_pl,Rd − M_pla)` (C 376-378). `M_pla = max(plate-based M_pl, W_pl·fy/γM0)` (C 531-540, 359-361). `M_pl,Rd` from `full_plastic_moment_nm` (C 364-373). `η = n_f / n_f,req` (C 429-431). Util = `M_Ed/M_Rd` (I 193-234). | I 174-234; C 364-373, 418-426, 531-540 | `full_plastic_moment_nm` uses a lever arm `z = h/2 + t_slab/2` and multiplies `N_cf·z` by an unexplained factor 0.5 (C 371-372). Verify against EN §6.2.1.2 Eq. set. `n_f = max(declared total_count, layout count)` (I 188-189). |
| `en1994.6.2.1.construction.<id>` | §6.2.1 / §9.3 | `M_Ed,c ≤ M_pl,a,Rd` (I 236-252). `M_Ed,c` is the construction-stage ULS moment (steel alone). | I 236-252 | Remedy hard-codes `actions[id=Q-constr].qAreaPa` (I 249). |
| `en1994.6.6.5.5.spacing.<id>` | §6.6.5.5 | `5d ≤ s ≤ min(6·h_c, 800 mm)` (C 491-504). Util = `s / s_max` (I 254-270). | I 254-270; C 491-504 | Fails when `s > s_max` or `s < s_min` (I 266). Only `s_max` is in the utilisation. |
| `en1994.6.6.3.1.prd.<id>` | §6.6.3.1, Eq. 6.18/6.19 | `P_Rd = min(0.8·fu·π·d²/4, 0.29·α·d²·√(fck·Ecm)) / γV · k_t` (C 403-415). `α = 0.2(h/d+1)` for h/d ≤ 4, else 1 (C 381-384). `k_t` (C 387-400). Demand: `V_L,Ed / n_f`, where `V_L,Ed = N_cf` (I 272-281). | I 272-300; C 381-415 | Demand is the full-interaction `N_cf`, not a force derived from `M_Ed`, so this is a full-connection check independent of the applied moment. `k_t` includes a non-EN thickness factor `t/0.9 mm` clamped to 0.55–1.35 (C 393, 399). |
| `en1994.6.6.1.2.etamin.<id>` | §6.6.1.2 | `η_min = 1 − (355/fy)(0.75 − 0.03·L)` for L ≤ 25 m, else `1 − (355/fy)·0.3`, floor 0.4 (C 434-442). Util is `η_min/η` or `η/η_min` (I 302-316). | I 302-316; C 434-442 | Uses `span_m`, not an equivalent span Le. |
| `en1994.6.2.2.vpl.<id>` | §6.2.2 | `V_pl,Rd = A_v·(fy/√3)/γM0` (C 445-449). `A_v = max(h·tw, a_v)` (snapshot 418-420). Shear-buckling util `(hw/tw)/(72·ε/η)` with η = 1 (C 523-528). Util = V_Ed/V_pl,Rd (I 335-398). | I 335-398; C 445-449, 523-528 | No moment-shear interaction (§6.2.2.4). |
| `en1994.6.6.6.vlrd.<id>` | §6.6.6 | `V_L,Rd = max(2.5·b·h_f·f_ctd, A_sf·f_yd·L/2)` with `f_ctd = 0.21·fck^(2/3)/γC`, `f_yd = 500 MPa/γS` (C 452-460). Util = `V_L,Ed/V_L,Rd`, with `V_L,Ed = N_cf` (I 400-417). | I 400-417; C 452-460 | `f_yk = 500 MPa` is hard-coded for transverse reinforcement (C 457). The demand is load-independent (see §6.6). |
| `en1994.6.4.ltb.<id>` | §6.4 | `M_b,Rd = χ_LT·W_pl·fy/γM1` (C 474-477). `χ_LT` from `λ = √(A·fy/N_cr)` with `N_cr = π²EI/L²` and curve factor 0.34 fixed (C 462-472). Demand: `uls_c.m_nm` if unpropped, else `uls.m_hog_nm` (I 419-427). Otherwise N/A (I 486-492). | I 419-493; C 462-477 | Uses an Euler load `N_cr` rather than the elastic critical moment M_cr. The curve is fixed. Search for L_req by bisection, 32 steps (I 444-456). |
| `en1994.7.2.2.stress.<id>` | §7.2.2 | `σ_a = |M_char| / W_el`, `W_el = I/(h/2)` of the steel section only (I 498-499). Limit `0.9·fy` (I 500). Util = σ_a/limit. | I 497-540 | Steel-only section, ignores composite stiffness and stage. The limit is 0.9·fy in code. EN 1994-1-1 §7.2.2 is commonly quoted as 0.8·fyk for this check; verify before relying on it. Uses `sls_characteristic` (composite stage only). |
| `en1994.7.3.1.deflection.<id>` | §7.3.1 | `δ = 5·M_freq·L²/(48·E·I_eff)` with `I_eff = I_a·(1 + 0.35·n_L/10)` (I 552-554). `n_L = 210 GPa/(Ecm·(1 + 0.55·φ))` with φ = 2.0 fixed at the call site (C 480-483, I 552). Limit `L/250` (EN) or `L/300` (DE) (C 552-565). Util = δ/limit. | I 543-607; C 480-483, 552-565 | The stiffness factor `0.35·n_L/10` is not an EN expression. The simply-supported formula is applied to all supports. Only the steel section is used. The check named `cracking_and_vibration` contains no vibration check. |
| `en1994.7.4.crack.<id>` | §7.4 / EN 1992-1-1 §7.3 | `A_s,min = max(kc·k·fct,eff·act/σs, 0.001·h_c)` with `kc = 0.4`, `k = 0.8`, `σs = 0.9·500 MPa`, `act = h_c·min(b_eff, spacing, 1.0 m)` (C 573-581). Max bar spacing table 150/200/250 mm for w_k ≤ 0.3/0.4/else (C 584-593). Heuristic scale `n_L_fac·load_fac` (I 610-616). Util = max(A_s,min·scale/A_s,prov, s/s_lim) (I 617). | I 609-641; C 573-593 | `min(1.0 m)` cap on the effective-area width is undocumented. Scale factors are heuristic, not EN 7.4. `σs` is fixed. |

### 2.2 EN 1994-1-1, composite columns (per column)

Helpers in `C`, `part_column` (706-907). Check bodies in I 644-697.

| Check id | Clause | Formula and source | Where | Notes |
|---|---|---|---|---|
| `en1994.6.7.3.npl.<id>` | §6.7.3 | `N_Ed ≤ N_Rd = χ·N_pl,Rd` (I 652). `N_pl` (C 716-729): concrete-filled `η_a·A_a·fy/γM0 + η_c·0.85·A_c·fck/γC + A_s·fsk/γS`; partially encased `n_a + 0.85·n_c + n_s`; others `n_a + n_c + n_s`. Confinement `η_a, η_c` (C 732-748), an approximation. `χ` from `λ̄ = √(N_pl/N_cr)` with `N_cr = π²·210 GPa·I_steel/L²` (C 888-906). `d/t` local buckling util against 90ε² / 44ε / 40ε (C 751-760). | I 644-673; C 716-760, 888-906 | `EI_eff` of §6.7.3.4 is not used, only the steel `I`. Confinement formulas are simplified and not the EN Eq. 6.31-6.33. No second-order (§6.7.3.5) check. |
| `en1994.6.7.3.mn.<id>` | §6.7.3.6 | Util `= N_Ed/N_pl + α_M·μ_d/μ_dd` on a polygon A(N_pl,0)-B(N_pm,M_max)-C(0.5 N_pm,M_pl)-D(0,M_pl) (C 859-886). `α_M = 0.9` (S ≤ 355) else `0.8` (C 820-822). `M_pl` and `M_max` from plastic stress blocks with fixed arm factors (`h/4`, `0.4h`, `0.45h`) (C 804-837). | I 675-696; C 804-886 | This is a linearised polygon interaction, not the EN Fig. 6.19 integral. Arm factors are invented. Remedy hard-codes `actions[id=Q-col].nKN` (I 691). |

### 2.3 EN 1994-1-1, composite slabs with profiled sheeting (per slab)

Helpers in `C`, `part_slab` (909-962), plus sheeting helpers in `part_1_1`. Check bodies in I 699-795.

| Check id | Clause | Formula and source | Where | Notes |
|---|---|---|---|---|
| `en1994.9.3.sheeting.<id>` | §9.3 / §9.7 | Construction-stage moment `M_Ed,c = q_constr·L²/8`, or `25 kN/m³ × h_c × L²/8` if no construction load (I 706-711). Sheeting resistance `M_Rd = a_p·fyp·h/4` with `fyp = 280 MPa` hard-coded (C 543-548; I 706). | I 706-731 | `25 kN/m³` fallback and `280 MPa` are hard-coded. `a_p = 1.2·t` (snapshot 429-431). |
| `en1994.9.7.2.mrd.<id>` | §9.7.2 | `m_Rd = A_s·f_sd·0.9·d + 0.5·A_p·f_yp·h/2` with `d = h_c − 0.02`, `f_sd = 500 MPa/γS`, `f_yp = 280 MPa/γM0` (C 927-936). Util = `m_Ed/m_Rd` (I 733-759). | I 733-759; C 927-936 | `0.9·d`, `−0.02`, the 0.5 sheeting factor and `500 MPa`/`280 MPa` are hard-coded. Verify against EN 9.7.2 and Annex. |
| `en1994.9.7.3.mk.<id>` | §9.7.3 | `v_l,Rd = max(m-k, τ_u)` (I 761). `m-k` (C 913-925): `b·d_p·(m·A_p/(b·L_s) + k)·1e6/γVs × profile_fac × rib_fac × parallel_fac`, with `d_p = h_c − h_p/2`, `L_s = L/4`, `γVs = 1.25`. `τ_u` method (C 939-953) uses `τ_u = 0.45 MPa` and `280 MPa·A_p/γ`. Util = `v_Ed/v_l,Rd` (I 762-775). | I 761-775; C 913-953 | Taking `max` of two methods is non-conservative. The `k` term does not contain the EN `√fck` factor. Profile, rib and parallel factors are not EN factors. |
| `en1994.9.7.5.vrd.<id>` | §9.7.5 | `v_Rd,c = 0.12·fck^(1/3)·d/γC` with `d = h_c − 0.02` (C 955-961). Util = `v_Ed/v_Rd,c` (I 778-794). | I 778-794; C 955-961 | No minimum term and no `ρ` term of EN 1992-1-1 §6.2.2. |

Slab action note: the slab's `uls_composite` is called with a 1.0 m strip width (I 701).

### 2.4 Action single-source checks (EN 1990 §6.1, all members)

`en1994.action.single-source.<member>.<action>` (I 797-837). For every characteristic action on a beam or slab, the check fails when both `qAreaPa` and `fKN` are set. A fatigue action must not carry `qAreaPa` or `fKN` (`Δσ_k/Δτ_k` only). The check is a structural consistency rule, not an EN 1990 verification.

### 2.5 EN 1990 load effects (shared by all members)

Helpers in `C`, `part_en1990` (57-346), used by `I`.

- Partial factors: `γ_G = 1.35`, `γ_Q = 1.50` (C 98-104). Both are the same for EN and DE.
- ψ factors (C 79-95): permanent (1,1,1); construction (1,1,0); imposed by category A/B (0.7, 0.5, 0.3), C/D (0.7, 0.7, 0.6), E (1.0, 0.9, 0.8), H (0,0,0); snow (0.5, 0.2, 0); wind (0.6, 0.2, 0); fatigue (0,0,0).
- Action internals (C 125-143): for a distributed load `q = q_area × spacing`, `M = qL²/8` (simply supported) or `qL²/16` (span) and `qL²/8` (hogging) for a 2-span continuous beam. `V` is `qL/2`, or `1.25·qL/2` for continuous. Point loads use `FL/4`, `FL/8` and `F/2` variants. Point loads apply only if `|q| ≈ 0` and `|F| > 1 N`.
- ULS composite (C 206-314, mode `uls`): leading-variable search over all variables, with `γ_G·G + γ_Q·(Q_lead + Σ ψ0·Q_other)`. The result is the largest combination by `|M|+|M_hog|+|V|+|N|`.
- ULS construction (`uls_construction`): stage = construction, same approach.
- SLS characteristic, frequent (ψ1 leading, ψ2 others), quasi-permanent (ψ2 for all): C 261-303.
- Fire (`fire`): C 304-311, factor `ψ_fi = max(ψ1, ψ2)` for non-permanents (C 119-122). This is an approximation and not the EN 1990 §6.4.3.4 rule.
- Column ULS (`accumulate_column`, C 150-200): leading-variable search over N and M with `γ_G`/`γ_Q`. The label includes "Eq. 6.10".
- FLM3 surrogates (C 336-345): `Δσ = 240 kN·(L/4)/W`, `Δτ = 60 kN / (π d²/4)`. These are simplified stand-ins for EN 1991-2 FLM3, not the traffic model.

Note: `uls` considers only Eq. 6.10 (no 6.10a/b split). No accidental or seismic combinations are computed. The SLS characteristic, frequent and quasi-permanent combinations are used for the checks in §2.1.

### 2.6 EN 1994-1-2, fire (document-wide)

Source: I 864-906. Only the first beam's steel and section are used.

| Check id | Clause | Formula and source | Notes |
|---|---|---|---|
| `en1994.1-2.4.2.insulation` | §4.2 / Table 4.2 | Required board thickness from a hard-coded table (C 615-638) by rating (R30-R120), profile (trapezoidal or re-entrant) and steel height (≥400, ≥300, else mm). Provided `insulation_thickness_m` ≥ required. | Table values are hard-coded. DIN NA is said not to amend them (C 614). The table is not checked against the standard in this audit. The insulation thickness is one global value, not per beam. |
| `en1994.1-2.4.3.theta` | §4.3 | `θ_a = 20 + 345·ln(0.085·t + 1) / (protect·√(1 + 0.04·φ))`, capped at 1200 °C (C 657-665), with `φ = clamp(A_m/V / d_ins, 0.5, 80)`, `A_m/V = 2(h+b)/A` (C 641-644). `θ_cr = 39.19·ln(1/(0.9674·η^3.833) − 1) + 482` (C 668-671). `η_fi = clamp(M_fire/M_ULS, 0.2, 0.7)` with fallback 0.65 (I 881-885). Util = `θ_a/θ_cr`. | This is a simplified form. Verify against EN 1993-1-2 §4.2.5 and EN 1994-1-2 §4.3 before use. `η_fi` uses a moment ratio, not the EN load ratio in §4.3. |
| `en1994.1-2.4.2.insulation` (N/A) | §4.2 | Emitted when the rating string is not `r30..r120` (I 900-903). | — |

### 2.7 EN 1994-2, bridges (only when `structure_kind == "bridge"`)

Source: I 907-943. For every beam:

| Check id | Clause | Formula and source | Notes |
|---|---|---|---|
| `en1994.2.6.8.2.delta-sigma.<id>` | §6.8.2 | `Δσ` = max fatigue action `Δσ_k`, or the FLM3 surrogate `240 kN·(L/4)/(0.85·W_pl)` (I 910-914). Resistance `Δσ_R = Δσ_c·(2e6/N)^(1/3) / γMf`, where `Δσ_c` depends on the detail (C 681-697). Detail table: stud_welded 80 MPa, shear_connector 71, reinforcement 90, flange_butt_weld 71, other 71 (C 683-687). | `W = 0.85·W_pl` is a stand-in for the elastic modulus (I 910). Remedy scales `W_pl`. |
| `en1994.2.6.8.3.stud.<id>` | §6.8.3 | `Δτ` = max fatigue `Δτ_k`, or FLM3 surrogate `60 kN / A_stud` (I 927-928). Resistance `Δτ_R = 90 MPa·(2e6/N)^(1/8) / γMf` (C 700-703). | Uses `m = 8`. Remedy increases stud diameter by `√(Δτ/Δτ_R)`. |
| `en1994.2.fatigue` (N/A) | §6.8 | Emitted for non-bridge structures (I 944-949). | — |

Note: bridge fatigue on reinforcement is not checked, although `reinforcement` is an option (`fatigue_detail`). No FLM4, no traffic model, no shear lag, no bridge deflection or vibration checks.

### 2.8 Coverage summary

- Computed per beam: 13 check ids (beff, class, mrd, construction, spacing, prd, etamin, vpl, vlrd, ltb, stress, deflection, crack). Two bridge checks are added when applicable.
- Computed per column: 2 check ids (npl, mn).
- Computed per slab: 4 check ids (sheeting, mrd, mk, vrd).
- Computed per action: one single-source check each.
- Document-wide: fire (2 or 3 checks) and bridge fatigue (N/A or 2 per beam).

---

## 3. NA-DE parameters actually used

Source: `C` 18-51 (`AnnexParams`) and the annex-specific branches listed below.

| Parameter | EN value | DE value | Where used |
|---|---|---|---|
| γ_M0 | 1.00 | 1.00 | plastic moments, `N_pl` steel, `V_pl` (C 36, 42; used by C 359-361, 445-449, 531-540, 718, 780, 808) |
| γ_M1 | 1.00 | **1.10** | LTB resistance only (C 476). Comment cites DIN EN 1993-1-1/NA. |
| γ_Mf | 1.15 | **1.35** | bridge steel and stud fatigue (C 696, 702). Comment cites DIN EN 1993-1-9/NA safe-life. |
| γ_V | 1.25 | 1.25 | shear-connector P_Rd (C 413) |
| γ_C | 1.50 | 1.50 | concrete (C 368, 422, 719, 765, 781, 959) |
| γ_S | 1.15 | 1.15 | reinforcement: longitudinal shear (C 457), `N_s`, `M_s`, slab reinforcement (C 749, 930) |
| γ_G | 1.35 | 1.35 | ULS permanent (C 98-100) |
| γ_Q | 1.50 | 1.50 | ULS variable (C 102-104) |
| ψ factors | — | — | identical for EN and DE (C 79-95) |
| Deflection span divisor | 250 | **300** | SLS frequent limit `L/250` vs `L/300` (C 552-565) |
| Fire insulation table | Table 4.2 | Table 4.2 | `_annex` is ignored (C 615). DE is not amended. |

Observations:
- The DE/EN difference is applied only via `AnnexParams::for_annex` and the deflection divisor. The EN-only fallback on γ_M1 and γ_Mf is consistent with `AnnexParams::en()`.
- Every check's explanation string appends `(DE-NA)` to the German text, regardless of annex. This appears in I 310, 408, 510, 770, 788 and 936. The `part` label is `"DIN EN 1994-1-1"` for both annexes (I 122 and onward).
- The `De` default in `En1994Snapshot::default()` (snapshot 59) means that a default document is DE. The `annex` change is a mutation (`ChangeAnnex`).

---

## 4. Editor

Source: `✏️editor/`. Manifest in `✏️editor/🦀️.rs` 235-291. Mode `edit` with a row layout 42/58.

### 4.1 Commands (`✏️editor/🦀️.rs` 59-73)

| Command | Wire id | Payload | Behaviour |
|---|---|---|---|
| `evaluate` | `evaluate` | none | Returns `Emit::default()`, zero mutations. The report is recomputed on every read (`🧮️evaluate/🦀️.rs` 3-7 and 24-26). |
| `setSelectedCheckIndex` | `selected-check` | `index: Option<u32>` | View state only. Changes the Results window config (`☑️selected-check/🦀️.rs`). |
| `setField` | `set-field` | `path: String`, `value_json: String` | Resolves the path through `EDIT_RULES` into kinds (§4.3). The value is parsed from a JSON string (`dispatch_set_field`, app-surface `⌨️command-args-and-dispatch` 50-56). |
| `insertItem` | `insert-item` | `path`, `index: u32`, `value_json?` | Inserts into beams, columns or slabs (§4.3). |
| `removeItem` | `remove-item` | `path`, `index` | Removes beams, columns or slabs by row id (`ID_REMOVALS`, `🧭️edit-rules/🦀️.rs` 56-66). |
| `applyRemedy` | `apply-remedy` | `checkId`, `remedyIndex` | Writes the remedy's `required.value` (SI) into `target.path`. Faults if not applicable or if the path is empty (app-surface `✏️edit-rules/🦀️.rs` 194-236). |

Keybindings: `mod+z` undo, `mod+shift+z` redo (`✏️editor/🦀️.rs` 286-287). All mutations are declared as "Migrated" interactive jobs.

### 4.2 Field metadata (`✏️editor/🏷️field-meta/🦀️.rs`, 187 l.)

- Each leaf has `label_en` and `label_de` and a unit (SI).
- Choice lists (`NormFieldChoice`) exist for: `annex` (En, De), `structure_kind` (building, bridge), `fire_rating` (r30 to r120), `fatigue_detail` (4 values), `construction` (propped, unpropped), `support` (2 values), action `kind` (6 values), action `stage` (construction, composite), sheeting `profile` (trapezoidal, re-entrant), booleans (Yes/No), column `kind` (3 values), `buckling_curve`.
- Wildcard paths (`[]`) cover list leaves, for example `beams[].steel.tfM` and `columns[].actions[].nKN`.
- Test `every_default_leaf_has_en_de_field_meta` (`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`) ensures every default leaf has both labels.

### 4.3 Edit rules (`🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`)

`EDIT_RULES` contains exactly 19 `set_field` rules, 3 `insert_item` rules (`beams`, `columns`, `slabs`) and 3 `remove_item` rules.

set-field paths that resolve (19):
`annex`, `steelFYPa`, `structureKind`, `fireRating`, `insulationThicknessM`, `fatigueDetail`, `beams[].spanM`, `beams[].slabThicknessM`, `beams[].construction`, `beams[].transverseAsM2PerM`, `beams[].studs.totalCount`, `beams[].studs.spacingM`, `beams[].studs.diameterM`, `beams[].studs.fUPa`, `beams[].actions[].qAreaPa`, `columns[].kind`, `columns[].actions[].nKN`, `slabs[].concreteThicknessM`, `slabs[].actions[].qAreaPa`.

Any other path returns `no kind sets '<path>'` (`🧭️edit-rules` 115-120), which is surfaced as the fault `norm.edit-unresolved`. Several leaves are rendered in the Inputs window but cannot be edited: steel geometry (`designation`, heights, `tw`, `tf`, areas, `W_pl`, `I`), `concreteFCkPa`, `concreteECmPa`, sheeting, `ltbLengthM`, `asHoggingM2PerM`, `barSpacingM`, `wkLimitM`, `nCycles`, `construction` of columns, action `kind`/`category`/`stage`/`fKN`/`deltaSigmaKPa`/`deltaTauKPa`, all column geometry, all slab geometry except thickness, `f_ck`, `m`/`k` factors and the action quantities, and the column and slab support fields.

### 4.4 Windows (`✏️editor/🎭️modes/✏️edit/🪟️windows/`)

- **Inputs** (`norm-en1994-inputs`, body `norm.en1994.play.inputs`, icon `download`, left pane 42%). Renders the whole snapshot through `render_document_editor` (app-surface `📝️document-editors/🦀️.rs` 169-180). This is a generic tree: each leaf is a control (text, number with unit, choice selector, boolean or read-only byte/null), list items are tree groups, and commits go to `setField` with the leaf path. No diagrams, no 2D or 3D views, no inline utilisation. Raw JSON is not rendered here (`render_document_json`, marked "legacy", is available but unused).
- **Results** (`norm-en1994-results`, body `norm.en1994.play.results`, icon `bar-chart-3`, right pane 58%). Renders the full `CheckReport` through `render_report` (app-surface `🌳️report-tree/🦀️.rs` 214-225). Rows are grouped by part, each with a verdict. Fail and Warning rows expand into explanation and remedy children. A OneOf remedy with no options shows "No options", and a non-applicable remedy shows "Not auto-applicable". The window is virtualised with `window_section_or_placeholder`. Text only, no colour-coded diagram.

### 4.5 Panels (`✏️editor/📌️panels/`)

- **Artifact** (`🗿️artifact`, body `norm.en1994.play.artifact`, Workbench group). One text line from `render_summary`: verdict, counts of checks, failing, warnings and passing, worst utilisation, and a per-part ✓/✗ list.
- **Catalogue** (`📚️catalogue`, body `norm.en1994.play.catalogue`, Workbench group). Lists the four example sources, plus three reference tables: partial factors γ (EN 1990 Table A1.2(B)), ψ factors and stud spacing limits (`📌️panels/📚️catalogue/🦀️.rs` 24-136).
- **Inspection** (`🔍️inspection`, body `norm.en1994.play.inspection`, Details group). For the selected check: id, part, clause, subject, subject path, status, title, explanation, computed, limit, utilisation (2 decimals), annex, and a row per remedy with an "Apply N" action (app-surface `📚️catalogue-and-inspection/🦀️.rs` 150-215).

### 4.6 Other editor surfaces

- Ports: `export_media` for `report:out` and `artifact:out`; `import_media` for `model:in` and `artifact:in` (`✏️editor/🦀️.rs` 178-186). `import_media` has an unused `_base` binding (line 184).
- Presence, transient, draft and config are empty (`NoPresence`, `NoTransient`, `NoDraft`, `NoConfig`). The Results window config stores only `selected_check_index`.
- `reference_birth` returns `En1994Snapshot::default()`.
- Examples source list: `composite-floor-beam`, `composite-floor-beam-failing`, `composite-bridge-girder`, `demo-session` (see §7).

---

## 5. Viewer

Source: `👁️viewer/`. `En1994Viewer` implements `ArtifactViewer` only (`👁️viewer/🦀️.rs` 48-101). Mode `view` (`🎭️modes/👁️view/🦀️.rs`) is a single full-pane window.

- Window: `report` (`📊️report/🦀️.rs`), kind `TableWindowKit`, read-only (no `set-cell`).
- Renders `report_table_columns` and `report_table_rows` (app-surface `🏷️ids-chrome-and-labels/🦀️.rs` 49-85). Seven columns: Part, Clause, Subject, Status, Utilisation (2 decimals), Title, and Remedy, which shows only the first remedy's text.
- Every render recomputes `evaluate(document)` from scratch (`📊️report/🦀️.rs` 30-32). No caching.
- The viewer imports nothing from the editor, which is enforced by a purity rule (`📊️report` 2-4).
- There is no 2D or 3D visualisation, no chart, no diagram, and no drill-down from table to member.

---

## 6. Tests and oracles

### 6.1 Rust tests (approx. 99 by `#[test]`/`async fn` count across the artifact)

| Area | Location | Count (approx.) | What they check |
|---|---|---|---|
| Compliance report | `S/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | 17 tests | Worked example passes by default (≥10 checks). Building and fire parts run. Failing subject has remedies. DE vs EN bridge fatigue (`γ_Mf` 1.15 pass, 1.35 fail). Remedy law for stud count. Stable subject paths survive reordering. OneOf remedy writes designation. Insulation remedy flips fire check. Passing and failing DSL examples. Bridge example runs fatigue. Bridge LTB and crack remedies clear. Every default leaf has EN/DE metadata. Example snapshot validates against JSON Schema (`python3 -c` with `jsonschema`). Python oracle matches evaluate within 0.5% (§6.2). Every editable leaf affects at least one check. TS facets have no `unknown` and match Rust leaves. |
| Checks unit tests | `S/🧪️tests/⚖️compliance/🦀️.rs` | 8 | effective width for 8 m span, stud P_Rd worked example, `η_min` for 8 m, plastic moment equation, DE γ_Mf stricter, DE γ_M1 stricter, DE deflection tighter, bridge fatigue detail strength. |
| Inference unit | `I/🧪️tests/🔬️unit`, `🧾outline/🧪️tests/` | 4 | Determinism and default law for the outline inference. |
| Diff | `S/🔺️diff/🧪️tests/🔬️unit` and `apply-and-inverse` | 5 + | Diff apply and inverse. |
| Mutation vectors | `S/🧬️mutations/🧪️tests/` (unit, kinds-catalog, vectors) | 3 | Kind list matches enum and catalogue, vectors. |
| Mutation case | `ANY/🧪️tests/🧩️mutate-en1994-1/🦀️.rs` | 455-line case | Applies each of the 25 kinds against committed vectors, checks after-snapshot, inverse restoration and the identity round trip. Requires the `sut` feature. |
| Editor | `✏️editor/🧪️tests/🔬️unit` and per-command/per-window test dirs | ~40 | Command dispatch, panels, windows, catalogue tables. |
| Viewer | `👁️viewer/🧪️tests`, `report/🧪️tests` | 6 | Report table lines up with check report. |
| IO | `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests` (14), `📝️text` (3+1), `💾️binary` (1), `🧬️mutations` (1) | ~20 | Codec round trips. |
| Examples | `📚️examples/🌉️composite-bridge-girder/🧪️tests` (3), `🎬️demo-session/🧪️tests/🧩️example` (1) | 4 | Bridge example, demo session. |

### 6.2 Python oracles

**`ANY/🔮️oracles/⚖️compliance/🐍️.py` (264 l.)** Stdlib only (`json`, `math`, `sys`, `pathlib`). It recomputes:
- effective width (`effective_width_m`, `uncapped_effective_width_m`) and the self-check assertion;
- stud P_Rd with `α` and `k_t` (`connector_resistance_n`, `stud_alpha`);
- `η_min` (`min_eta`);
- insulation table (`insulation_m`, which matches C 615-638 for the rows it covers);
- ULS composite moment (`uls_composite`, EN and DE γ_G/γ_Q only, `psi_factors` table).

It compares, within 0.5% (`maxRel ≤ 0.005`, `compared ≥ 3`), the utilisation of `beff`, `prd`, the `mrd` computed value, the `etamin` quantities and the `insulation` utilisation. It does not cover M_Rd itself, LTB, shear, column, slab or fire θ checks. Its `kt` omits the thickness factor that the Rust side applies (`C` 393, 399), so the two can diverge for sheeting ≠ 0.9 mm. This is an independent second implementation of a subset, not an independent reading of the standard: it shares the same modelling choices, including the full-interaction `N_cf` demand.

**`ANY/🔮️oracles/🧬️snapshot-schema/🐍️.py` (12 l.)** Uses `jsonschema` (third-party Python). The check is invoked from the compliance-report test via `python3 -c` (`🧬️schema/💡️inferences/🧪️tests/.../🦀️.rs` 248-270).

**`ANY/🧪️tests/🧩️mutate-en1994-1/🐍️.py` (107 l.)** Imports `semio_norm_vocabulary` through `import_module("🐍️")` from `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`, which is outside this artifact and was not inspected. The Gherkin feature (`🥒️.feature`, 115 l.) has three scenarios and states that the 25 `✅apply` vectors cover 19 change, 3 insert and 3 remove kinds, all on the bridge girder.

**`ANY/🔮️oracles/🔣️.json`** registers the mutation oracle for this subset (`en1994-1-any`). It is generated from the leaf `SemanticDescriptor.kind` order.

### 6.3 Third-party libraries used by tests and oracles

- Python: `jsonschema` (schema test only), `python3` invoked by name (not version pinned, and not a zero-touch binary on Windows). Stdlib for the compliance oracle.
- Rust (test only): `serde`, `serde_json` (`cfg_attr(test)` derives and dev-dependencies). No runtime third-party crate.
- No test library is used for the Rust side beyond `semio_framework_async_macros::async_test`.

---

## 7. Examples and fixtures

### 7.1 Examples (`ANY/🖼️assets/`, `📚️examples/`, `✏️editor/📚️examples/`)

| Example | Files | Content |
|---|---|---|
| `composite-floor-beam` | `🗣️.dsl.semio`, `🎒️.pack.semio` | Compliant floor beam, DE annex (default presets), 8 m span, HEB300, 140 mm slab, r60 fire, 3 slab/column actions |
| `composite-floor-beam-failing` | `🗣️.dsl.semio` only | Same beam, failing inputs used by the failing-subject tests |
| `composite-bridge-girder` | `🗣️.dsl.semio`, `🎒️.pack.semio` | Bridge (`structure_kind = bridge`), FLM3 fatigue action, 25 m span, `stud_welded` |
| `demo-session` | `🎮️.cmd.semio` | Stub: only the line `action=demo`. Not an EN 1994 example. |

Each `.dsl.semio` is a text file in the `semio norm.en1994.dsl v1` format, with a header line and lines for each member table. Binary twins (`.pack.semio`) exist for the first and third examples only.

### 7.2 Fixtures

- 140 mutation fixtures under `🧪️tests/🧫️fixtures/🧬️mutations/` (25 kinds, roughly 5 or 6 scenarios each).
- Snapshot schema facets and sample JSON inside the `📸️snapshot` directory.

---

## 8. Gaps, stubs, TODOs, hard-coded values and suspected defects

Items are ordered by impact. "Verify" means the item depends on a standard text that this audit did not check.

### 8.1 Suspected correctness defects

1. **Stud and longitudinal shear demands are load-independent.** `V_L,Ed` is set to the full-interaction flange force `N_cf` (I 280, 317 returns it, used at I 281 and I 406). The stud check (§6.6.3.1) and the longitudinal shear check (§6.6.6) therefore never see `M_Ed` or the actual shear-connector force. The stud check is effectively a full-connection check.
2. **Slab longitudinal shear takes the larger of two methods** (`max(m-k, τ)`, I 761). Taking the larger resistance is non-conservative. The m-k term omits the EN `√fck` factor (verify against EN 9.7.3). Profile, rib and parallel factors are invented (C 918-924, 945-952).
3. **Partial-interaction moment resistance has an unexplained factor.** `full_plastic_moment_nm` (C 371-372) multiplies `N_cf·z` by 0.5 with `z = h/2 + t_slab/2`. The lever arm is not the EN stress-block arm (verify against EN §6.2.1.2).
4. **LTB uses an Euler load, not M_cr.** `χ_LT` (C 463-472) uses `λ = √(A·fy/N_cr)` with `N_cr = π²EI/L²` and a fixed curve factor of 0.34. Verify against EN 1993-1-1 §6.3.2.
5. **SLS stress uses steel-only section modulus and 0.9·fy.** I 498-500. The composite section, the construction stage and propping are ignored. Verify the limit: EN 1994-1-1 §7.2.2 is commonly quoted as 0.8·fyk.
6. **Deflection uses an invented stiffness factor.** `I_eff = I_a·(1 + 0.35·n_L/10)` (I 553). The simply-supported formula is applied regardless of support. The creep factor `φ = 2.0` is fixed at the call site (I 552).
7. **Modular ratio uses `0.55·φ`.** `modular_ratio_long_term` (C 480-483) does not match the EN form `(1 + ψ_L·φ_t)` (verify ψ_L for permanent actions).
8. **Effective-width check fails in the normal capped case.** I 132-135. EN caps `b_eff` at the spacing. A beam with `b0 + L/4 > spacing` is reported as Fail and gets a remedy to increase spacing, which is not a non-compliance.
9. **Column stiffness uses steel `I` only.** `relative_slenderness` (C 888-894) and `chi` (C 896-906) ignore the concrete and reinforcement contribution `EI_eff` of §6.7.3.4. The curve is chosen by a string label, and λ̄ has no cap.
10. **Column `M_pl` and `M_max` are approximations with invented arm factors** (`h/4`, `0.4h`, `0.45h`, C 804-837). The interaction is a polygon with fixed points (C 859-870), not the Fig. 6.19 integral.
11. **Confinement factors** (C 732-748) are invented formulas labelled as Eq. 6.31-6.33 simplified.
12. **Crack control `A_s,min` caps the effective width** with `min(1.0 m)` (C 578). The scale factors and the bar-spacing table (C 584-593) are heuristic and use `σs ≈ 280 MPa`.
13. **Fire is evaluated on the first beam only**, with one global insulation value (I 862-882). `η_fi` is a moment ratio clamped to [0.2, 0.7] with a fallback of 0.65. `θ_a` is a simplified formula (C 657-665); verify against EN 1993-1-2 §4.2.5 and EN 1994-1-2 §4.3.
14. **Bridge fatigue uses a stand-in elastic modulus** (`W = 0.85·W_pl`, I 910), not `I/y`. The FLM3 surrogates (C 336-345) are not the EN 1991-2 traffic model.
15. **Section data is inconsistent.** Beam checks use catalogue-resolved steel (I 101), but the fire check (I 878) and bridge fatigue (I 909) use the stored, unresolved steel. Beams use the document-level `steel_f_y_pa`, while columns use their own `steel_f_y_pa`.
16. **Hidden catalogue overwrite.** `SteelSection::resolve()` replaces stored `tw`, `tf` and the other geometry whenever the designation matches HEB300 to HEB700. The stored leaves are then silently ignored for those members.
17. **Remedies hard-code action ids.** `Q-office`, `Q-col`, `Q-constr` (I 207, 249, 366, 513, 594, 691). A member without those ids gets fewer remedies or a generic one.
18. **Remedy magnitudes use heuristics**, for example `q_cur·m_rd/m_ed·0.95` (I 213), `q_cur·(v_pl/v_ed)·0.98` (I 375), `span·(scale)^0.5` (I 530). Only some are tested by the remedy tests.
19. **Every check is labelled "DIN EN 1994-1-1"** regardless of annex (I 122 onward). The German explanation text always contains "(DE-NA)" (I 310, 408, 510, 770, 788, 936).
20. **Construction moment fallback** uses 25 kN/m³ concrete density (I 710).
21. **Vertical shear of slab** (C 955-961) has no minimum term and no ρ term.
22. **Check named "cracking and vibration" has no vibration check** (I 543).
23. **Stud spacing check utilisation** shows only the upper bound (I 260); the lower bound fails the check but is not in the utilisation.

### 8.2 Hard-coded values (inventory)

- `210e9` Pa for steel E: C 464, 481, 890, 554.
- `500e6` Pa for reinforcement `f_yk` when a slab or the longitudinal shear uses it: C 411, 457, 749, 930; I 411, 579 (`sigma_s = 0.9·500 MPa`, C 579).
- `280e6` Pa for sheeting `f_yp`: I 706, 751; C 933, 941, 952.
- `1.2` developed-profile factor for `a_p`: snapshot 430.
- `1.25`, `0.45 MPa`, `τ_u`, `b = 1.0`: C 917, 941-942, 944.
- `0.02` m (cover) for `d` in slab checks: C 929, 957; I 748.
- `0.04` m minimum concrete height `h_c`: C 421, 500, 574.
- `0.0009` m reference sheeting thickness: C 392.
- `0.35` and `0.55` stiffness and creep factors: I 553, C 482.
- `240 kN`, `60 kN` FLM3 surrogates: C 337, 342.
- `345`, `20`, `0.085`, `0.04`, `0.9674`, `3.833`, `39.19`, `482`: fire temperature constants, C 657-671.
- `0.85` concrete stress-block factor: C 368, 422, 719, 765.
- `0.3` and `0.75`, `0.03`, `355` in `η_min`: C 436-439.
- `0.9`, `0.85` in the `W` and steel-stress factors: I 500, 910.
- `Q-office`, `Q-col`, `Q-constr`: I 207, 249, 366, 513, 594, 691.
- `2.0e3` default area load in a remedy: I 212.
- `wk_limit_m.max(0.0003)`: I 610.
- Default fire η 0.65: I 885.
- Snapshot `Default` (`annex De`, `beam-B1`, `col-C1`, `slab-S1`): snapshot 49-70.

### 8.3 Stubs, placeholders and gaps

- `demo-session` example is a one-line stub (`action=demo`) (`✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio`).
- `evaluate` command is a no-op that exists only for the command log (`🧮️evaluate/🦀️.rs` 3-7 and 24-26).
- `EngineHandles` is passed to `handle` and never used (`✏️editor/🦀️.rs` 156).
- `render_document_json` is labelled legacy and kept for tests (app-surface `📝️document-editors`).
- `import_media` binds an unused `_base` (`✏️editor/🦀️.rs` 184).
- Editor and viewer use SDK-gap comments for `Dialect`, `WindowKit` and `TableWindowKit` (`✏️editor/🦀️.rs` 42-44; `👁️viewer/🦀️.rs` 20; `📊️report` 9-10).
- Presence, transient, draft and config lanes are empty (`NoPresence` and others).
- The `🫧️transient` folders contain only placeholder files.
- Mojibake in comments and section markers: `✏️editor/🦀️.rs` (lines 1-6, 48-74 and others) and `✏️editor/🧪️tests/🔬️unit/🦀️.rs` show double-encoded emoji (`ð`, `â`). The file is not valid UTF-8 text for those comments.

### 8.4 Repo-rule and portability issues

- `python3` is invoked by name in three test and oracle call sites. The compliance-report test (`python_oracle_matches_evaluate_json_within_half_percent`) and the schema test both spawn `python3`. This is not zero-touch on native Windows.
- The mutation oracle imports a module whose name is an emoji (`import_module("🐍️")`). This is fragile on some platforms.
- Snapshot enumerations are strings, not typed enums (§1.7).
- Duplicate constants: `AnnexParams` values are repeated in the catalogue panel tables and in the checks; the `210e9` constant is repeated in several functions.
- The compliance evaluate is recomputed on every read and every render. There are no progress or cancellation hooks, although the computation is synchronous and quick for small documents.
- Set-field carries values as JSON strings (`value_json`), which is raw JSON transport at the command boundary.

### 8.5 Coverage gaps against the EN 1994 objectives

Not computed anywhere in the artifact:
- §6.2.1.2 full elastic moment resistance, §6.2.2.4 moment-shear interaction, §6.3 (web crippling, transverse forces), §6.4 for sagging LTB with `M_cr`.
- Continuous-span effective width reduction (`Le`) and partial-shear LTB.
- §6.6.3 stud fatigue for building structures, §6.6.4 stud group reduction, §6.6.2 detailing except spacing.
- §6.7.3.4 `EI_eff`, §6.7.3.5 second order and sway, shear in columns, transverse reinforcement of encased columns, and fire for columns.
- §7.3 vibration; §7.2 for slabs and sheeting in SLS; §7.4 for sagging cracking.
- Slab: ULS in-plane shear around the stud, sheeting bearing and web crippling, SLS deflection of the sheeting, EN 9.7.5 minimum term.
- Fire: beams at hogging, tabular method of §4.3.4 beyond the first beam, composite slab fire, and per-member fire protection.
- EN 1994-2: reinforcement fatigue, shear lag, traffic-model fatigue (FLM3 and FLM4 as in EN 1991-2), bridge SLS deflection and vibration, and bridge-specific rules for stud groups.
- EN 1990: accidental and seismic combinations, the Eq. 6.10a/b split, and altitude-dependent snow.
- 2D or 3D views of members, sections, joints, load paths, utilisation heat maps, failure modes, M–N diagrams, stress blocks, spectra or S–N curves. The brief asks for these. None exists in this artifact. `EngineHandles` is passed but unused.
- Direct manipulation in a viewport. Editing is by a generic value tree, and about 19 leaves are editable (§4.3).
- Inline utilisation feedback next to each field. The Inputs window does not show utilisation.
- Remedies in the editor are applied one at a time through `applyRemedy`, and no preview of the post-remedy state is shown.

---

## 9. Files of record (quick index)

- Model: `S/📸️snapshot/🦀️.rs`, `S/🦀️.rs`
- Checks: `S/💡️inferences/⚖️checks/🦀️.rs`
- Evaluate: `S/💡️inferences/🦀️.rs`, `S/💡️inferences/🧾outline/🦀️.rs`
- Mutations: `S/🧬️mutations/🦀️.rs`, `S/🧬️mutations/🧭️edit-rules/🦀️.rs`
- Editor: `✏️editor/🦀️.rs`, `✏️editor/🏷️field-meta/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`
- Shared UI surface used by the editor: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/📝️document-editors/🦀️.rs`, `…/🌳️report-tree/🦀️.rs`, `…/📚️catalogue-and-inspection/🦀️.rs`, `…/✏️edit-rules/🦀️.rs`
- Viewer: `👁️viewer/🦀️.rs`, `👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`
- Oracles: `ANY/🔮️oracles/⚖️compliance/🐍️.py`, `ANY/🔮️oracles/🧬️snapshot-schema/🐍️.py`
- Tests: `S/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`, `S/🧪️tests/⚖️compliance/🦀️.rs`, `ANY/🧪️tests/🧩️mutate-en1994-1/`
