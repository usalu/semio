# 🪨 EN 1996 Inventory

Read-only audit of the EN 1996 artifact as committed. No source file was modified.

## Conventions

- `ART` = `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996` (repo-relative).
- `SUB` = `ART/🏅️standards/🔖️1/🪆️subsets/✳️any` (the only subset that exists; the artifact root holds `Cargo.toml`, `📦️packages/`, `📜️artifact-definition.json`).
- Line numbers refer to the files as they stand in the snapshot. "Bullet with `file:line`" entries are the primary evidence.
- File count: 935 tracked files under `ART`, of which about 921 are under `SUB` and 908 are non-empty (the rest are `📌️.empty.md` placeholders). The full list was saved to `ticket/🗑️generated/en1996-files.txt`.

## 0. Summary of findings

- The model is a single masonry-building subject: a building with `storeys`, `annex`, `masonryClass`, `designSituation` and a table of walls. Each wall has units, mortar, openings, reinforcement, and a table of load cases, each with concentrated loads.
- Compliance checks are implemented for EN 1996-1-1 §5.5.1 (slenderness), §3.1 (material), §6.1.2 (compression, middle-height compression, eccentricity limit), §6.1.3 (concentrated load), §6.2 (shear and sliding), §6.3 (lateral flexure), §6.6 (minimum reinforcement). They are also implemented for EN 1996-1-2 §4 (tabulated fire thickness plus alpha), EN 1996-2 Annex B and §8 (mortar class and bed joint), and EN 1996-3 §4.2 and §4.5 (simplified applicability, simplified compression, basement earth pressure).
- Several checks report hard-coded utilisation numbers (0.2, 0.5, 1.5, 2.0) instead of computed ratios. Several remedies use hard-coded targets. These are listed in section 8 as the most important gaps.
- Confined masonry, ties and anchors, prestressed masonry, and shear-wall or bracing checks are absent. Wall type `Shear` exists only as a classification.
- The "independent" Python oracle shares formulas with the Rust code, and its `fk` check compares a function with itself. See section 6.
- The editor's Inputs window is a structured property tree, not raw JSON. The brief's premise that `render_document_editor` renders JSON is incorrect; `render_document_json` exists but is only used by tests.

## 1. Model

### 1.1 Snapshot model (`SUB/🧬️schema/📸️snapshot/🦀️.rs`)

Entities, enums and their fields. Units are SI, with `#[dsl(unit=…)]` annotations where present.

| Type | Kind | Fields (camelCase on wire) | Notes |
|---|---|---|---|
| `En1996Snapshot` | struct, `#[dsl(id="norm.en1996", layout="lines")]`, line 23 | `annex: AnnexChoice`, `masonry_class: MasonryClass`, `design_situation: DesignSituation`, `storeys: u32`, `walls: Vec<MasonryWall>` (table) | `Default` = `compliant_clay_wall()` (line 38) |
| `MasonryWall` | struct, line 443 | `id`, `label_en`, `label_de`, `wall_type: WallType`, `thickness_m`, `height_m`, `length_m`, `support_sides: u8`, `openings: Vec<WallOpening>`, `slab_bearing_depth_m`, `eccentricity_top_m`, `eccentricity_bottom_m`, `unit_group: UnitGroup`, `unit_material: UnitMaterial`, `f_b_pa`, `unit_length_m`, `unit_width_m`, `unit_height_m`, `mortar_type: MortarType`, `mortar_class: MortarClass`, `mortar_strength_pa`, `bed_joint_thickness_m`, `reinforced: bool`, `as_vertical_m2`, `as_horizontal_m2`, `f_yd_pa`, `fire_rei_min: u32`, `exposure: ExposureClass`, `mu`, `density_kg_m3`, `phi_infinity`, `is_basement: bool`, `load_cases: Vec<WallLoadCase>` (table) | Field doc comments state the intended meaning |
| `WallLoadCase` | struct, line 404 | `id`, `design_situation: String` (free text: persistent / transient / accidental / seismic), `imposed_category: String`, `g_k_slab_n`, `q_k_imposed_pa`, `tributary_area_m2`, `slab_span_m`, `q_k_snow_pa`, `q_p_wind_pa`, `c_pe`, `h_k_earth_n`, `concentrated: Vec<ConcentratedLoad>` | `design_situation` and `imposed_category` are strings, validated only in integrity checks |
| `ConcentratedLoad` | struct, line 387 | `id`, `force_n`, `bearing_area_m2`, `bearing_length_m` | Characteristic force |
| `WallOpening` | struct, line 371 | `id`, `width_m`, `height_m`, `sill_height_m` | |
| `MasonryClass` | enum, line 243, `#[default] Class3` | `Class1..Class5` | `gamma_m_en` (line 254): 1.5, 1.7, 2.0, 2.2, 2.5. `gamma_m_de(accidental)` (line 266): accidental 1.3; otherwise 1.5, 1.7, 1.7, 2.0, 2.0 |
| `ExposureClass` | enum, line 283 | `Mx1..Mx5` | |
| `UnitMaterial` | enum, line 295 | `Clay`, `CalciumSilicate`, `Aerated`, `Concrete` | |
| `UnitGroup` | enum, line 306 | `Group1..Group4` | |
| `MortarType` | enum, line 317 | `GeneralPurpose`, `ThinLayer`, `Lightweight` | `Lightweight` is not special-cased in any formula (see section 8) |
| `MortarClass` | enum, line 327 | `M1`, `M2_5`, `M5`, `M10`, `M15`, `M20` | `f_m_mpa` (line 339); `compressive_strength_mpa` (line 350) is an explicitly labelled "alias used by older durability helpers" |
| `WallType` | enum, line 357 | `LoadBearing`, `Shear`, `NonLoadBearing` | Only `LoadBearing` vs non-load-bearing matters, in the simplified applicability check |

Whole-model constants: `EN1996_DIALECT` (line 508, artifact kind `s.norm.en1996`, standard `1`, subset `ANY`), `EN1996_DOCUMENT_SCHEMA = "semio.norm.en1996/v1"` (line 510). `artifact_kind()` (line 501) builds the registry entry.

Snapshot example constructors (lines 46–228). They are fixtures in code, not persisted files:

- `compliant_clay_wall()` (line 46): DE, Class1, persistent, 2 storeys; wall 5.0 m × 2.75 m × 0.365 m, 4-sided, clay group 1, f_b 20 MPa, M10 mortar, bed joint 12 mm, one load case (`ulsinz`, G_k 120 kN, q_k 2 kPa, tributary 12.5 m², span 4.5 m, wind 800 Pa, c_pe 0.8).
- `noncompliant_multi_fail()` (line 104): Class3, 4 storeys, aerated wall 3.2 m × 0.09 m, f_b 4 MPa, M2.5, Mx4, bed joint 4 mm, one load case with snow, wind, and a 80 kN concentrated load.
- `opening_wall_example()` (line 167): window and door opening.
- `basement_wall_example()` (line 190): `isBasement=true`, earth pressure H_k 45 kN.
- `concentrated_load_example()` (line 203): one beam load of 35 kN.
- `reinforced_wall_example()` (line 218): reinforced, As 6e-4 / 4e-4 m², f_yd 435 MPa.

### 1.2 Artifact schema (`SUB/🧬️schema/🦀️.rs`)

- `En1996Artifact` (line 22): `#[derive(ArtifactSchema, DslRecord, ToValue, FromValue)]`, `#[artifact_schema(id="s.norm.en1996")]`. Fields are the same as the snapshot: `annex`, `masonry_class`, `design_situation`, `storeys`, `walls`, all `#[state(artifact)]`.
- Conversions (lines 37–71): `From<En1996Snapshot>`, `to_snapshot`, `from_snapshot`. Field-by-field copy, no logic.
- `en1996_artifact_schema_descriptor()` (line 74) wires the five leaf facets (rust/ts/graphql/json-schema/proto) and the snapshot, diff and mutation leaves.
- `pub mod checks` (line 86) mounts `⚖️masonry`.

Other schema facets: `SUB/🧬️schema/🔗️.graphql`, `🔣️.json`, `🛰️.proto`, `🟦️.ts`. Snapshot facets: `SUB/🧬️schema/📸️snapshot/` (same five leaves plus `📦️packages`, `🧪️tests`). Inference facets: `SUB/🧬️schema/💡️inferences/`.

### 1.3 Inferences (`SUB/🧬️schema/💡️inferences/🦀️.rs`)

- `En1996Inference { #[derived] outline: En1996Outline }` (line 21). `infer()` (line 26) computes only the outline: field count (5), section list, and wall count (`🧾outline/🦀️.rs` lines 12–25). This is not a compliance inference.
- `evaluate(document)` (line 72) is the real compliance entry point. It calls `checks::masonry::evaluate_building` (`⚖️masonry/🦀️.rs` via `SUB/🧬️schema/🦀️.rs`) and returns `CheckReport`.

### 1.4 Diff and mutation schemas

- Diff: `SUB/🧬️schema/🔺️diff/🦀️.rs`. Typed `En1996Diff` built from patch structs. Example at line 63: `En1996WallPatch of MasonryWall { set { label_en, label_de, wall_type, … } }`.
- Mutation enum `En1996Mutation` (`SUB/🧬️schema/🧬️mutations/🦀️.rs`, lines 81–200). 58 variants, each a leaf module with `diff`, `inverse` and `SemanticDescriptor` (verb/entity/kind). The kinds are `change-*` (annex, storeys, masonry class, design situation, imposed category, load case situation, wall fields such as length/height/thickness/eccentricity/φ∞/label/type/density/μ/unit fields/mortar/fire REI/reinforcement fields/support sides/exposure/tributary area/slab span/slab bearing depth/g_k/q_k/q_p/c_pe/h_k earth/bearing length/bearing area/force/opening width/height/sill/is-basement, etc.), `insert-*` and `remove-*` for walls, load cases, openings and concentrated loads, plus `change-c-pe`, `change-mu`, and similar.
- Mutation groupings: `⚖️loads`, `🧰️loads-2`, `🧱️walls`, `🪜️walls-2`, `🪟️openings`, `🪣️mortar`, `🔩️reinforcement`, `🟫️units`, `🏢️project`, `📍️concentrated`, `📐️eccentricity`. The `-2` groups are split-off parts; see section 8.
- Edit-rules (`SUB/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`, 71 lines): 50 `set_field` rules, 4 `insert_item` rules and 4 `remove_item` rules. Each maps an editor path (e.g. `walls[].loadCases[].slabSpanM`) to a kind and to selector keys (wall index, load-case index, item index).

## 2. Checks and verifications

Entry: `evaluate_building` (`SUB/🧬️schema/⚖️masonry/🦀️.rs` via `🛡️conformance-and-integrity/🦀️.rs:233-263`). It runs referential integrity first, then `evaluate_wall` per wall. `evaluate_wall` (`🧱️evaluate-wall/🦀️.rs:18-50`) runs, in order: slenderness, material conformance, the load-case checks (only if load cases exist; otherwise a NotApplicable check `en1996.loads.empty.*`), fire, durability, reinforced masonry, and simplified-plus-basement.

Notation: `f_k` characteristic masonry compressive strength (`f_k_pa`, `factors-and-geometry:92`). `f_d = f_k/γ_M` (`load-case-checks:28-29`). `A = ℓ_ef · t` (`factors-and-geometry:188`). `h_ef = ρ_n · h` (`:179`).

Pass or fail rule (shared kernel `✏️s/🔌️plugins/📕️norm/⚖️compliance/✅️check-results/🦀️.rs`): `utilization(computed, limit)` sets `Pass` when `computed/limit ≤ 1.0` (line 182). `minimum(computed, min)` sets Pass when `computed ≥ min` (line 186–195). `build()` (line 232) overrides to `NotApplicable` if flagged and to `Warning` only when `warn_above` is set. No check in the masonry code sets `warn_above` (grep count 0).

### 2.1 EN 1996-1-1 §5.5.1 Slenderness

- Check id `en1996.5.5.1.slenderness.{wall}`, clause `§5.5.1`. File: `🧱️evaluate-wall/📏️wall-geometry-and-material/🦀️.rs:48-87`.
- Formula: `λ = h_ef / t`, `h_ef = ρ_n · h`. The utilisation is `λ / 27`. Limit 27 is a literal (line 65).
- `ρ_n` (`factors-and-geometry:168-177`): 2 sides → 0.75; 3 sides → `0.75 / (1 + (0.75·h/(3·ℓ))²)`; 4 sides → `0.75 / (1 + (0.75·h/ℓ)²)`. The 4-sided formula is not EN 1996 Table 5.x as far as this auditor can tell; the standard text is not in the repo, so this is unverified.
- Opening-aware `ℓ_ef` (`effective_length_m`, `factors-and-geometry:152-166`): piers (sill ≤ 5 % of h and opening height ≥ 85 % of h) subtract full width; other openings subtract `width · open_frac · (0.35 + 0.65·(1 − sill_frac))`. The factors 0.35, 0.65, 0.05, 0.85 are literal and unsourced.
- Remedies: thickness to `h_ef/27` (line 71); `supportSides` to 4 (line 82).

### 2.2 EN 1996-1-1 §3.1 Material conformance (DE NA)

- Check id `en1996.3.1.material.{wall}`, clause `§3.1`. File: `wall-geometry-and-material/🦀️.rs:90-162`, logic in `🛡️conformance-and-integrity/🦀️.rs:6-39`.
- Pass condition (`material_conformance_ok`): `|f_m,declared − f_m,class| ≤ 0.15·f_m,class`; unit group dims ≥ 90 mm; group 2 requires aspect `h/w < 2.5`; bed joint 6–15 mm (general purpose) or 0.5–3 mm (thin layer); f_b in `[1.5, 10]` MPa (aerated), `[4, 100]` (clay), `[2, 100]` (others).
- Utilisation is hard-coded: `0.5` when OK (line 110) and `1.5` when failing (line 125). The failing case does not reflect any ratio.
- Remedies: align mortar strength (exact), raise f_b to `max(6, …)` MPa (line 135), unit height 0.113 m (line 141), unit width `max(h/2.4, 0.100)` m (line 147), bed joint 10 mm / 2 mm (lines 114–118, 153).

### 2.3 EN 1996-1-1 §6.1.2 Compression (ULS, annex-dependent γ_M, Φ-method)

Driven by `check_load_cases` (`🏋️load-case-checks/🦀️.rs:6-35`) for each load case.

- Design effects (`🔥️fire-and-design-effects/🦀️.rs:140-206`), see section 2.9 for combinations. The compression check uses the governing combination (`governing_compression`, line 208) as the largest `n_bot`.
- Eccentricities (`check_wall_compression`, lines 39–206):
  - `e_slab = |t/2 − a/2|` (`factors-and-geometry:204`), with `a` the slab bearing depth.
  - `e_θ = min(e_full · share, 0.05·t)` where `e_full = ℓ_f/25` with ℓ_f capped at 6 m, and `share = clamp(N_floor/|N_Ed|, 0, 1)`. `N_floor = G_k,slab + (q_imp + q_snow)·A_trib` (unfactored, slab-only; self-weight and concentrated loads excluded). (`factors-and-geometry:214-226`).
  - `e_init = h_ef/450` (line 63).
  - `e_top = |e_top,decl + e_slab + e_θ|`; `e_bot = |e_bot,decl|` (lines 60–61).
  - `e_m = 0.5·(e_top + e_bot) + e_init`; `e_k = 0.002·φ∞·(h_ef/t)·√(t·e_m)` (`creep_eccentricity_m`, `factors-and-geometry:266-272`; line 67).
  - `e_mk = max(|e_m + e_k|, 0.05·t)`.
- Reduction factors:
  - `Φ_i = clamp(1 − 2e/t, 0, 1)` (`phi_i`, line 240).
  - `Φ_m = a1 · exp(−0.5·u²)`, `a1 = 1 − 2e_mk/t`, `u = max(0, (λ−2)(t−2e_mk)/(23t))` (`phi_m`, lines 246–255). This is a Gaussian-type approximation written by the authors. It is not verified against EN 1996-1-1 Annex G. The test named `annex_g_phi_m_midheight_na_method` only asserts `0.5 < Φ_m < 1.0` for one input.
  - `Φ₁` (NA slab span): 1.0 for `ℓ_f ≤ 4.5 m`, else `clamp(1.6 − ℓ_f/6, 0, 1)` for `ℓ_f ≤ 6 m` (`phi_1_slab_span`, lines 229–236).
  - Stations: top (`Φ_i(e_top)·Φ₁`), mid (`Φ_m·Φ₁`), bottom (`Φ_i(e_bot)·Φ₁`), governing = max of `N/Φ` (lines 72–73).
- Resistance: `N_Rd = Φ · f_d · A` (line 74). Utilisation `N_Ed / N_Rd`, check id `en1996.6.1.2.compression.{wall}.{lc}`. Check `en1996.6.1.2.compression.mid.*` for the mid-height station (lines 136–170).
- Remedies: thickness to `t·max(1, N/N_Rd)` (line 95), `f_b` scaled (line 96), slab bearing to `max(t/3, a)` (line 113), slab span `max(0.85·ℓ, 4.0)` (line 119), `φ∞·0.7` min 0.5 (line 125), density `max(0.9·ρ, 600)` (line 131).
- Eccentricity limit `en1996.6.1.2.eccentricity.*` (`check_wall_compression` lines 172–204): declared eccentricity ≤ 0.2·t (line 173). Note that this uses only the declared top/bottom values, not the slab-bearing contribution.

### 2.4 EN 1996-1-1 §6.1.3 Concentrated load

- Check id `en1996.6.1.3.concentrated.{wall}.{lc}.{load}`. File: `load-case-checks/🦀️.rs:350-401`.
- `β = clamp((1 + 0.15·x/h)·(1.5 − 1.5·a1/ℓ), 1.0, 1.5)`, `x = max(0, ℓ/2 − a1/2)` (`concentrated_beta`, `fire-and-design-effects:79-83`). The formula and coefficients should be checked against the standard before use.
- `N_Rdc = β · f_d · max(A_b, l_b · t)` (line 363).
- `F_Ed = γ_Q · F_k`, with `γ_Q = 1.5` (line 364). The whole concentrated force is treated as variable.
- Utilisation `F_Ed / N_Rdc`. Remedies on bearing area (×1.01 fudge, line 379), bearing length, thickness.

### 2.5 EN 1996-1-1 §6.2 Shear and sliding

- Check id `en1996.6.2.shear.{wall}.{lc}`. File: `load-case-checks/🦀️.rs:208-265`.
- `σ_d = N_mid / A`. `f_vk = f_vk0 + 0.4·σ_d` with `f_vk0` from mortar class and material table (`factors-and-geometry:105-116`). Under the DE annex, `f_vk` is capped at `0.045·f_b` (`F_VLT_OVER_FB_DE`, line 120, `f_vk_pa` line 122-132). The cap is written as `raw.min(f_vlt).min(f_vk0 + 0.4σ)`, where the second term is never smaller than `raw`, so the second `min` is redundant.
- `V_Rd,masonry = f_vk/γ_M · A`.
- `V_Rd,slide = μ · N_mid / max(γ_M, 1)` (line 229). Dividing friction resistance by γ_M is not an EN 1996 formula as far as this auditor knows. The `max(γ_M, 1)` guard is unnecessary given γ_M ≥ 1.3.
- `V_Rd = min(masonry, slide)`; utilisation `V_Ed / V_Rd` where `V_Ed = γ_Q · (w_k·h·ℓ + H_k)` (`design_effects`, lines 150–151, 177, 199).
- Remedies: thickness, μ to `min(1.25·μ, 0.8)` (line 260).

### 2.6 EN 1996-1-1 §6.3 Lateral flexure (wind / earth on panel)

- Check id `en1996.6.3.flexure.{wall}.{lc}`. File: `load-case-checks/🦀️.rs:269-346`.
- Lateral action: `w_Ed = γ_Q · q_p · |c_pe|` (`design_effects` line 148, 200). Not applicable when there is no lateral action (the NotApplicable branch, lines 283–295).
- `M_Ed = w_Ed · h² / 8 · ℓ_ef` (line 297). Uniform load, simply supported, full height, length ℓ_ef.
- `Z = ℓ_ef · t² / 6` (`section_modulus_m3`, `factors-and-geometry:192`).
- `f_xd = f_xk1/γ_M` with `f_xk1 = min(0.10, 0.020·f_m)` MPa for clay, calcium silicate, concrete (`f_xk_pa`, `factors-and-geometry:134-143`), `0.05` MPa for aerated.
- Reinforcement contribution: `M_Rd,s = A_s,h · f_yd · z_s · ℓ_ef`, `z_s = 0.9·t` (lines 300–307).
- `M_Rd = f_xd · Z + M_Rd,s` (line 308). Utilisation `M_Ed / M_Rd`.
- Hard-coded: if `A_s,h > 0` and `f_yd ≤ 0`, utilisation is forced to 1.5 (line 341). The remedy `f_yd` target is `435e6` Pa (line 337). The thickness remedy has a `×1.01` fudge (line 325).

### 2.7 EN 1996-1-2 §4 Fire (tabulated thickness with α)

- Check id `en1996.1-2.fire.{wall}`, clause `§4`. File: `fire-durability-and-reinforcement/🦀️.rs:6-62`.
- `N_Ed,fi = max over load cases of (G + 0.7·Q)` where `G = G_k,slab + self weight + ΣF_conc` and `Q = (q_imp + q_snow)·A_trib`. The factor 0.7 is `ψ_fi`, literal, not category-dependent.
- `N_Rd,cold = max(Φ_s, Φ_m(0.05·t)) · f_d · A` (line 21).
- `α = N_Ed,fi / N_Rd,cold` (line 28).
- Minimum thickness `t_min = fire_min_thickness_na(REI, material, group, mortar, α)` (`fire-and-design-effects:7-58`). Tables by REI (30/60/90/120/180 ranges) for clay, calcium silicate, aerated and concrete, then multipliers: α ≤ 0.6 → ×0.9, α > 1 → ×1.15, non-group-1 → +10 mm, thin-layer → +5 mm. The table values are literals with no citation in the code.
- Check uses `minimum(t, t_min)`. If α > 1.0 the utilisation is forced to `α`, otherwise no utilisation is set and the minimum drives status.
- Remedy: `t ≥ max(t_min, t·α)` (line 49).

### 2.8 EN 1996-2 Annex B / §8 Durability and bed joint

- Durability check `en1996.2.durability.{wall}`, clause `Annex B`. File: `fire-durability-and-reinforcement/🦀️.rs:65-129`. Required mortar by exposure (`required_mortar_mpa`, `fire-and-design-effects:66-77`): MX1 1.0, MX2 2.5, MX3 5.0, MX4 10.0 (not for aerated, infinite), MX5 infinite. Pass when `f_m ≥ required`. Utilisation hard-coded `2.0` when the combination is inadmissible.
- Bed-joint check `en1996.2.bedjoint.{wall}`, clause `§8`: general-purpose 6–15 mm; thin layer ≤ 3 mm. Utilisation hard-coded `0.5` (pass) or `1.5` (fail).
- The durability check does not include the Annex B "exposure-to-unit-group" rules beyond the simple table above.

### 2.9 EN 1990 combinations (used by §6.1.2, §6.1.3, §6.2, §6.3, EN 1996-3)

`design_effects` (`fire-and-design-effects:140-206`) returns combinations, each with station values `N_top`, `N_mid`, `N_bot`, `V`, `w_Ed` and labels. Three combinations per load case, plus snow variant.

- ULS max-N (imposed-leading): `γ_G·(G_slab + F_conc + self-weight·(0, 0.5, 1)) + γ_Q·(Q_imp + Q_acc)` where `Q_acc = ψ_s·Q_snow` if snow present, otherwise `ψ_0,imp·Q_imp·0.25`. The `0.25` factor is literal and unsourced (line 175).
- ULS max-N (snow-leading): `γ_Q·Q_snow + ψ_0,imp·Q_imp`. Shear and wind: `γ_Q·ψ_w·v_k` (lines 185–191).
- ULS min-N / max-H: `γ_G,inf = 1.0` with the same action set (lines 193–204). The combination label says `W+E`, but seismic action `E` is not modelled; only wind and `H_k`.
- `γ_G,sup = 1.35` for both annexes; `γ_G,sup = 1.20` for transient situations (line 153, hard-coded); `γ_Q = 1.5` for both annexes; `γ_Q = 1.0` for accidental (`gamma_q`, lines 129–137).
- Combination factors: `ψ_0` per imposed category: A, B, C, D = 0.7; E = 1.0; H = 0.0; anything else 0.7 (`psi0_imposed`, lines 96–104). `ψ_s = 0.5` (line 106) and `ψ_w = 0.6` (line 108), both fixed. DE NA values for snow are zone-dependent; they are not implemented.
- The governing shear combination is the one with largest `V/N_mid` (line 212). Lateral governing is the largest `w_Ed` (line 220).

### 2.10 Simplified method (EN 1996-3 §4.2, DE NA Table NA.A.1)

- Applicability check `en1996.3.simplified.{wall}`, clause `§4.2` (`fire-durability-and-reinforcement/🦀️.rs:200-308`). It is NotApplicable when any of these hold: storeys > 3; λ > 27; not load-bearing; q_k > 5 kN/m² (any load case); slab span > 6 m; t < 115 mm; height > 3 m with t < 175 mm; building height `storeys · h` > 12 m. The list comment cites NA.A.1 for the 175 mm and the rest are implied.
- If applicable: `N_Rd = Φ_s · f_d · A`, `N_Ed = max_lc max(n_bot)` (lines 275–286). Utilisation `N_Ed / N_Rd`. Remedy `t · max(1, N_Ed/N_Rd)`.
- Φ_s: `0.85 − 0.0011·λ²` (EN), `0.70 − 0.0011·λ²` (DE) (`phi_s`, `factors-and-geometry:257-263`).

### 2.11 EN 1996-3 §4.5 Basement wall (earth pressure)

- Check `en1996.3.basement.{wall}`, clause `§4.5` (lines 311–349). Only for `isBasement = true`. Otherwise, if any load case has `H_k > 0`, a NotApplicable warning check is issued (lines 350–365).
- `N_Ed,min` and `N_Ed,max` taken from design-effect stations. `h_Ed = max V` over combinations. Requirement `N_min,req = 0.2 · h_Ed · H / t` (line 324), an unsourced formula.
- `u = max(N_min,req / N_min, N_max / N_Rd,s)`. Remedy: thickness `×1.2` (line 346) and text saying "increase permanent load" with no quantity.

### 2.12 Reinforced masonry (EN 1996-1-1 §6.6, minimal)

- Check `en1996.6.6.reinforced.{wall}`. File: `fire-durability-and-reinforcement/🦀️.rs:131-197`.
- `A_s,min = 0.0005 · A` (line 134). Only the minimum is checked; there is no design resistance of reinforced masonry.
- If not `reinforced` and any reinforcement or `f_yd` is given, utilisation 1.5 and a remedy setting `reinforced = true` (lines 156–167). Otherwise utilisation 0.2 (line 168).
- `stray = as_have + f_yd_pa` (line 157) adds m² and Pa. This is dimensionally inconsistent; it only tests `> 0`.
- If reinforced and `A_s < A_s,min` or `f_yd ≤ 0`, fail with remedies (lines 170–184). The `f_yd` remedy target is 435e6 Pa (line 182).

### 2.13 Integrity and referential checks

- Duplicate ids for walls, load cases, openings and concentrated loads (`🛡️conformance-and-integrity/🦀️.rs:43-98`, `186-231`). Status Fail with remedy `{id}-2` or `{id}-unique`.
- Unknown imposed category (`:100-139`). Accepts only A, B, C, D, E, H. F and G (and any other category) fail. Clause cited as "EN 1990 A1.1 Table A1.1".
- Unknown design situation (`:141-184`). Accepts `persistent, transient, accidental, seismic, bsp, bst, bsa`.
- Empty subject: NotApplicable `en1996.subject.empty` (`:242-256`).

### 2.14 Not implemented

- Confined masonry and ties/anchors: not implemented. A case-insensitive grep of the artifact's Rust, Python and TypeScript for "confin", "tie", "anchor", "prestress" and "shear wall" finds no check logic.
- Shear walls and bracing (EN 1996-1-1 §5 and §6.x for `WallType::Shear`): no check. `WallType::Shear` is only stored.
- Prestressed masonry: no check.
- Lateral load distribution to shear walls, diaphragm effects, floor-slab load paths: no check.
- Full reinforced-masonry design (§6.6 as a resistance): only the minimum reinforcement test.
- Fire: only tabulated minimum thickness (EN 1996-1-2 §4 style). No Annex-based or structural fire design.

## 3. NA-DE parameters actually used

Source of each value is the code location. None is cited to a specific NA table number in code, except the comments noted.

| Parameter | Value(s) in code | Location |
|---|---|---|
| γ_M by masonry class, persistent | Class1 1.5; Class2 1.7; Class3 1.7; Class4 2.0; Class5 2.0 | `📸️snapshot/🦀️.rs:266-276` |
| γ_M accidental (DE) | 1.3 | `📸️snapshot/🦀️.rs:267-269` |
| f_k factors, clay | Group1 k=0.60; Group2 0.50; Group3 0.45; Group4 0.40; α=0.65; β=0.25 | `factors-and-geometry:63-79` |
| f_k factors, calcium silicate / concrete | Group1 k=0.60, others 0.50 | same |
| f_k factors, aerated | k=0.60 | same |
| f_k factors, thin layer | k=0.70, α=0.85, β=0 | same |
| Shape factor δ | Literal polynomial of unit height, width and length with clamp [0.70, 1.40] | `factors-and-geometry:82-90` (not EN 772-1 Table; shape factor is a formula written by the authors) |
| φ_s | base 0.70 (DE) vs 0.85 (EN), minus 0.0011·λ² | `factors-and-geometry:257-263` |
| Φ₁ (slab span, NDP 6.1.2.2) | 1.0 for ℓ_f ≤ 4.5 m; 1.6 − ℓ_f/6 for 4.5 < ℓ_f ≤ 6.0 m | `factors-and-geometry:229-236` |
| e_θ (slab rotation, Annex C) | ℓ_f/25, capped at 6 m, times floor share, capped at 0.05·t | `factors-and-geometry:210-226` |
| γ_G sup / γ_Q | 1.35 and 1.5 (both annexes); transient γ_G = 1.20 | `fire-and-design-effects:121-137, 153` |
| ψ_0 imposed | A, B, C, D 0.7; E 1.0; H 0.0; default 0.7 | `fire-and-design-effects:96-104` |
| ψ_s snow / ψ_w wind | 0.5 / 0.6 (fixed, not zone-dependent) | `fire-and-design-effects:106-108` |
| f_vlt upper limit for shear | 0.045·f_b (DE only) | `factors-and-geometry:120, 127-131` |
| Fire min thickness table (DE NA) | clay / calcium silicate / aerated / concrete tables by REI | `fire-and-design-effects:7-58` |
| Mortar requirement by exposure | MX1 1.0; MX2 2.5; MX3 5.0; MX4 10 (not aerated); MX5 n/a | `fire-and-design-effects:66-77` |
| Simplified applicability (NA.A.1) | storeys ≤ 3; λ ≤ 27; q_k ≤ 5 kPa; span ≤ 6 m; t ≥ 115 mm; t ≥ 175 mm if h > 3 m; building height ≤ 12 m | `fire-durability-and-reinforcement/🦀️.rs:218-252` |
| Bed-joint thickness | GP 6–15 mm; thin layer ≤ 3 mm | `fire-durability-and-reinforcement/🦀️.rs:101-102`, `conformance-and-integrity/🦀️.rs:16-20` |
| Catalogue citation | "DIN EN 1996-1-1/NA NA.2.4.3" for γ_M table | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:50` |

EN (non-DE) values implemented for comparison: γ_M 1.5, 1.7, 2.0, 2.2, 2.5 (`gamma_m_en`); f_k k = 0.55 (clay G1), 0.45 (clay G2), 0.40 (other clay), 0.55 (calcium silicate G1), 0.45 (other calcium silicate), 0.55 (aerated), 0.55 (concrete G1), 0.45 (other concrete); α 0.70, β 0.30; thin layer k 0.80, α 0.85; φ_s base 0.85 (`factors-and-geometry:46-61`, `257-263`).

## 4. Editor

Root: `SUB/✏️editor/🦀️.rs`. App `En1996PlayApp` implements `ArtifactEditor`. Single mode `edit` (default). Layout is a row of Inputs (42 %) beside Results (58 %) (`✏️editor/🎭️modes/✏️edit/🦀️.rs:24-30`).

### 4.1 Commands (`✏️editor/🎮️commands/`)

Declared in `🦀️.rs:58-74` as an `app_commands!` enum. Each handler is in its folder:

- `set-field` (`✏️set-field/🦀️.rs`): `path`, `value_json`. Dispatches through `EDIT_RULES` (50 rules) into a typed `En1996Mutation`. Editable paths are listed in section 1.4.
- `insert-item` (`➕insert-item/🦀️.rs`): `path`, `index`, optional `value_json`. Rules for concentrated loads, load cases, openings and walls.
- `remove-item` (`➖remove-item/🦀️.rs`): `path`, `index`. Rules for concentrated loads, load cases, openings and walls.
- `apply-remedy` (`🩹apply-remedy/🦀️.rs`): `checkId`, `remedyIndex`. Applies a remedy via `dispatch_apply_remedy` in the shared app surface. Only remedies with a scalar target and no options are applied (test `gate_blocking_fails_clear_via_remedy_bounds`, `SUB/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:118`).
- `evaluate` (`🧮️evaluate/🦀️.rs:24-27`): returns `Emit::default()`, so it emits no mutation. It only triggers a re-render.
- `selected-check` (`☑️selected-check/🦀️.rs`): view-only; sets `selected_check_index` in the Results window config.

All six tools are declared as interactive jobs with classification `InteractiveJobClassification::Migrated` (`🦀️.rs:~192-212`). Keybindings `mod+z` undo and `mod+shift+z` redo.

### 4.2 Field metadata (`✏️editor/🏷️field-meta/🦀️.rs`)

58 entries in `TABLE` (lines 75–134) with English and German labels, SI unit (`m`, `m2`, `Pa`, `N`, `kg/m3`, `min`), and choice lists for enums (annex, masonry class, design situation, wall type, unit material, unit group, mortar type and class, exposure, support sides, booleans, imposed category). Imposed-category choices are A, B, C, D, E, H only. Lookup is by exact or longest-prefix path (`lookup_norm_field_meta`).

### 4.3 Panels (`✏️editor/📌️panels/`)

- Artifact panel (`🗿️artifact/🦀️.rs`, `PanelGroup::Workbench`): shared `render_summary` (`📇️registry/🧬️contract/🖥️app-surface/📝️document-editors/🦀️.rs:187+`). Text headline: verdict (complies / does not comply), per-part ✓/✗ list, counts.
- Catalogue panel (`📚️catalogue/🦀️.rs`, `PanelGroup::Workbench`): lists the two examples (`loadbearing-wall`, `multi-fail-masonry`) and five reference tables built from the same helpers as the compliance code (`reference_tables`, lines 25–33): γ_M by class (DE), f_k factors, fire min thickness at REI 90, ψ_0 table, f_vlt cap. Shared renderer `render_catalogue` (`📚️catalogue-and-inspection/🦀️.rs:110`).
- Inspection panel (`🔍️inspection/🦀️.rs`, `PanelGroup::Details`): one check, chosen by `selected_check_index` (default 0). Rows: id, part, clause, subject, subject path (when set), status, title, explanation, computed value, limit, utilisation (2 decimals), annex, and one row per remedy (`action · current → required · target path`) (`📚️catalogue-and-inspection/🦀️.rs:150-200`). Text rows, not graphics.

### 4.4 Windows (`✏️editor/🎭️modes/✏️edit/🪟️windows/`)

- Inputs (`📥️inputs/🦀️.rs`, id `norm-en1996-inputs`, icon `download`): `render_document_editor` (`document-editors/🦀️.rs:169-178`). This is a structured, schema-driven property editor over the value tree. Each leaf is rendered as a labelled control: choice selectors for enums and booleans, numeric fields with the unit from field meta, text for ids and labels, and list rows with insert and remove for tables. Commit triggers `setField`. The brief is wrong that this window "renders the document as JSON". The raw-JSON renderer `render_document_json` (line 181) is labelled "Legacy pretty JSON — kept for tests" and has no production caller.
- Results (`📊️results/🦀️.rs`, id `norm-en1996-results`, icon `bar-chart-3`): `render_report` (`🌳️report-tree/🦀️.rs:214-236`). A windowed tree grouped by part. Each part row shows a verdict. Each check row is a single line (`check_row_label`); only Fail and Warning checks expand, showing an explanation and their remedies. Scalar remedies show `current → required` with an "apply" action; option remedies show "Not auto-applicable" (`🌳️report-tree/🦀️.rs:149-163` and following). Also has a selected-check config owner (`ResultsWindowConfigOwner`).

### 4.5 Scope of the editor

- No 3D, 2D or diagram rendering. No utilisation heat map. No M-N or stress-block plots.
- Presence and transient facets are empty (`NoPresence`, `NoTransient`), so there is no multi-user presence.
- Drafts are empty (`NoDraft`).
- Media: `export_media` and `import_media` delegate to shared `norm` implementations (`🦀️.rs:181-190`).

## 5. Viewer

Root: `SUB/👁️viewer/🦀️.rs`. `En1996Viewer` implements `ArtifactViewer` only (no mutations, no commands other than an inert `Noop`).

- Report window (`SUB/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`, 40 lines): recomputes the report with `evaluate(document)` and renders a read-only `TableWindowKit` table. Columns: Part, Clause, Subject, Status, Utilization, Title, Remedy. The Remedy column shows only the first remedy's action text (`ids-chrome-and-labels/🦀️.rs:49-75`).
- No graphics, no diagrams, no 3D.

### 5.1 Extra files in `ART/📦️packages`

- `📦️packages/🦀️rust/Cargo.toml`: package `semio-s-artifact-norm-en1996`, `role = "artifact"`, lib path `../../🦀️.rs` (artifact root), and a second binary `regenerate-en1996-example-assets` from `bins/regenerate_example_assets.rs`. The binary calls `write_committed_example_assets` to regenerate `SUB/🖼️assets/*.dsl.semio` and `*.pack.semio`.
- `📦️packages/🦀️rust/📋️project.json` (repo project: build and test targets through `repo:script`) and `📜️script.ts` (runs the Rust package via the framework's build driver).
- `📦️packages/🟦️typescript/package.json`: `@semio-tech/norm-en1996`, private, ESM, LGPL-3.0-or-later, `types: dist/🟦️.d.ts`, one workspace dependency `@semio-tech/norm-js`. Scripts call `repo run`.
- `📦️packages/🟦️typescript/📋️project.json` and `📜️script.ts`: build, check and test through the framework's TypeScript artifact builder, with the SQLite snapshot suite (`SUB/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`).
- Artifact-root files: `Cargo.toml` (workspace of 21 members and about 50 path dependencies on framework crates), `Cargo.lock`, `📜️artifact-definition.json` (`id = s.norm.en1996`, `nx_project`, no dependencies).

## 6. Tests and oracles

### 6.1 Rust tests (51 `#[test]` attributes in the artifact)

- Compliance tests, `SUB/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (603 lines, about 20 test fns): `de_na_fk_clay_group1_m10`, `en_vs_de_fk_divergence`, `de_na_gamma_m_by_category_and_execution_class`, `phi_s_de_vs_en_na_simplified`, `annex_g_phi_m_midheight_na_method` (range check only), `fire_min_thickness_rei90_clay_and_aac_tabulated`, `simplified_method_storey_limit_de_na`, `compliant_evaluate_passes`, `every_emitted_path_parses_and_resolves_with_id_selectors`, `remedy_law_flips_at_least_two_distinct_fails_to_pass`, `gate_blocking_fails_clear_via_remedy_bounds`, `every_editable_leaf_has_en_de_field_meta`, `perturb_every_editable_leaf_changes_some_check`, `committed_dsl_pack_assets_parse_and_assert_verdicts`, `mutation_and_check_labels_differ_en_de`, `typed_diff_and_snapshot_facets_have_no_unknown`, `rho_n_four_sided_less_than_two`, `duplicate_wall_id_fails_integrity`, `duplicate_load_case_opening_and_concentrated_ids_fail_integrity`, `dangling_imposed_category_fails_integrity`, `perturb_wall_id_to_duplicate_changes_integrity_signature`.
- Oracle driver, `SUB/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs`: `python_oracle_matches_compliant_and_noncompliant` and `json_schema_validates_example_snapshots`. See 6.2.
- Other Rust tests: `SUB/✏️editor/…/🧪️tests/`, `SUB/👁️viewer/…/🧪️tests/`, `SUB/🧬️schema/🔺️diff/🧪️tests/`, `SUB/🧬️schema/🧬️mutations/…/🧪️tests/`, `SUB/🚪️io/…/🧪️tests/` (binary, text, sqlite round-trips), `SUB/🧬️schema/💡️inferences/🧪️tests/`.
- Mutation case, `SUB/🧪️tests/🪨️mutate-en1996-1/🦀️.rs` (183 lines): a Rust adapter for the shared mutation-vector law. It drives production dispatch with the committed vectors (`before`, `mutation`, `after`, `outcome`).

### 6.2 Python oracles

1. `SUB/🔮️oracles/⚖️compliance/🐍️.py` (299 lines, stdlib only: `json, math, sys, pathlib`). Named `en1996-1-python-independent` in `SUB/🔮️oracles/🔣️.json`. Its manifest declares `package: ""`, `kind: verified-native-second-implementation`, `productionReachable: false`, and that no third-party library exists for `s.norm.en1996`, based on a PyPI survey of structuralcodes, concreteproperties and anastruct.
   - What it computes: f_k (`f_k_mpa`), φ_s, Φ_i, Φ_m, ρ_n, γ_M, ψ_0, ℓ_ef, self-weight, e_θ, Φ₁, ULS stations (`design_stations`), and a governing compression utilisation (`assess_wall`).
   - What it checks in `main()` (lines 226–296): (a) `fk` against its own `f_k_mpa` (see 6.3); (b) utilisation for the Rust report checks whose ids contain `simplified` (not basement) and `6.1.2.compression` (not mid). Tolerance 0.5 %. Degenerate utilisation ≥ 90 is skipped.
   - Its formulas are transliterated from the Rust code (same Φ_m, same δ polynomial, same 0.25 factor, same e_θ). It is a transliteration, not an independent derivation.
   - Its fk tables ignore unit material. For EN, group 3 and 4 give 0.40 and 0.35, while the Rust code gives 0.45 for non-group-1 calcium silicate and concrete. The two implementations disagree there; the oracle would only agree with Rust for clay.
   - If no snapshot file is passed it falls back to an embedded clay wall (lines 232–244).
2. `SUB/🧪️tests/🪨️mutate-en1996-1/🐍️.py` (173 lines): the second producer for the mutation vocabulary. It imports `semio_norm_vocabulary` from a sibling module (`import_module("🐍️")`, outside this artifact) and defines the 58 kinds in `KINDS` (line ~20). It has no `def test` functions; checking is done by the `.feature` file and the Rust adapter.
3. Test-only Python snippet inside `SUB/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs:53-75`: uses the third-party `jsonschema` package to validate `SUB/🧬️schema/📸️snapshot/🔣️.json` against the serialised compliant example. This is the only third-party library in the artifact's tests.

### 6.3 Oracle findings

- The `fk` check is tautological. `assess_wall` and `main()` both call `f_k_mpa` with the same inputs, so they compare the Python function with itself. Rust `f_k_pa` is never compared to Python (the report does not carry f_k).
- The `ok` flag only reflects comparisons that actually happened. If no check matches, `ok` remains true and `note` says "no comparable utilization check found".
- Compared checks: `simplified` and `6.1.2.compression` (non-mid). Not compared: `reinforced`, `flexure`, `shear`, `concentrated`, `durability`, `fire`, `slenderness`, `material`, `eccentricity`, `basement`, and the mid-height compression.
- Only two example walls are fed to the oracle.
- The oracle test writes intermediate files into `std::env::temp_dir()` (system temp; `SUB/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs:29,60`), not into the ticket. This conflicts with the repository rule that temporary files go in the ticket folder.

### 6.4 Mutation vector suite

- `SUB/🧪️tests/🧫️fixtures/🧬️mutations/<kind>/✅apply/{⬅️before, 📸️snapshot/➡️after, 🔺️diff, 🦠️mutation, 🎯️outcome}/🔣️.json`: 58 kind directories, 310 JSON vectors.
- Feature file `SUB/🧪️tests/🪨️mutate-en1996-1/🥒️.feature` (195 lines): three scenarios. "Apply <id>" and "Undoing <id> restores its committed before-snapshot" are outlines over the kinds, and the third re-emits the committed carrier document.
- Oracle registration in `SUB/🔮️oracles/🔣️.json` claims 62 committed fixture vectors (manifest count) for the one capability `en1996-1-mutate`. The on-disk count is 310 JSON files across 58 kinds (directory counts vary; treat the manifest number as stale).

## 7. Examples and fixtures

- Editor catalogue examples (`SUB/📚️examples/`, registered in `SUB/✏️editor/🦀️.rs:82-85`):
  - `loadbearing-wall` (`SUB/📚️examples/🧱️loadbearing-wall/`): compliant clay wall; primary text is `SUB/🖼️assets/🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio`, plus a `🎒️.pack.semio`. The example's `🖼️assets/` also holds `📡️loadbearing-wall.spr.semio` and `🔧️loadbearing-wall.op.semio`, which the catalogue does not reference.
  - `multi-fail-masonry` (`SUB/📚️examples/❌️multi-fail-masonry/`): the non-compliant wall; asset `SUB/🖼️assets/❌️multi-fail-masonry/❌️multi-fail-masonry/🗣️.dsl.semio`.
- Demo session (`SUB/✏️editor/📚️examples/🎬️demo-session/`): command file `🖼️assets/🎮️.cmd.semio`. Not in the editor's example list.
- Snapshot fixture constructors, four unused in the catalogue: `opening_wall_example`, `basement_wall_example`, `concentrated_load_example`, `reinforced_wall_example`. They are only used by the compliance test table (`⚖️compliance/🦀️.rs:309-312`) and one test at line 515.
- Serialised DSL specs: `SUB/🚪️io/📝️text/`, with the grammar `📖️.grammar.semio` as a placeholder (`payload = OCTET+`, per the feature file's own note). `SUB/🚪️io/💾️binary/` binary-pack spec (`.ksy`, `.abnf`, `.spicy`), `SUB/🚪️io/🪶️sqlite/` table schema (`🗄️.sql`).
- There is no committed golden fixture for compliance reports. Compliance verdicts are asserted in Rust test code; the only fixtures on disk are the mutation vectors (section 6.4).

## 8. Gaps, stubs, TODOs, hard-coded values

### 8.1 Hard-coded or placeholder utilisations (highest impact)

Each of these shows a number to the user that is not a computed ratio.

- Material conformance: 0.5 when OK (`wall-geometry-and-material/🦀️.rs:110`), 1.5 when failing (`:125`).
- Bed joint: 0.5 / 1.5 (`fire-durability-and-reinforcement/🦀️.rs:116,119`).
- Durability inadmissible: 2.0 (`:79`).
- Reinforced: 1.5 when reinforcement is given without `reinforced=true` (`:160`); 0.2 when nothing is given (`:168`).
- Flexure: 1.5 when `A_s,h > 0` and `f_yd ≤ 0` (`load-case-checks/🦀️.rs:341`).
- Fire: α used as utilisation only when α > 1.0; otherwise no utilisation is set (`:45`).

### 8.2 Hard-coded remedy targets and fudge factors

- Unit height target 0.113 m (`wall-geometry-and-material:141`); unit width `max(h/2.4, 0.1)` (`:147`).
- Bed joint targets 10 mm / 2 mm (`:114-118`).
- Mortar f_b target `max(6, …)` MPa (`:119-135`).
- f_yd target 435 MPa (`load-case-checks:337`; `fire-durability…:182`).
- Remedy multipliers: `×1.01` (`load-case-checks:325, 379`), `×1.2` (basement, `fire-durability…:346`), slab span `×0.85` with floor 4.0 m (`load-case-checks:119`), φ∞ `×0.7` floor 0.5 (`:125, 160`), μ `×1.25` cap 0.8 (`:260`), density `×0.9` floor 600 kg/m³ (`:131`), support sides target 4 (`wall-geometry…:82`), reinforcement increment `+50 mm²` (`load-case-checks:331`).
- Thickness minimum `0.175 m` for wall height > 3 m (`fire-durability…:244`), `0.115 m` (`:240`); NA.A.1 citation only partial.

### 8.3 Formula concerns (for specialist review; standard text is not in the repo)

- Sliding resistance `V_Rd,slide = μ·N/γ_M` (`load-case-checks:229`).
- Shear cap `f_vlt` applied with a redundant second `min` (`factors-and-geometry:129`).
- `Φ_m` Gaussian approximation labelled as Annex G (`factors-and-geometry:246-255`).
- Shape factor δ is an authored polynomial, not the EN 772-1 table (`factors-and-geometry:82-90`).
- Concentrated-load β formula and coefficients (`fire-and-design-effects:79-83`).
- Opening-aware ℓ_ef coefficients (0.35, 0.65, 0.05, 0.85) (`factors-and-geometry:152-166`).
- `q_acc = ψ_0·Q_imp·0.25` when no snow (`fire-and-design-effects:175`) and the `×0.25` in the oracle (`oracles/⚖️compliance/🐍️.py:145`).
- Basement requirement `0.2·h_Ed·H/t` (`fire-durability…:324`).
- Reinforced `stray = as_h + f_yd` (`:157`) mixes units.
- Wind shear `v = w·h·ℓ` on the whole panel without distribution (`fire-and-design-effects:150`).
- `ψ_s = 0.5`, `ψ_w = 0.6` fixed; DE NA snow is zone-dependent.
- "ULS min-N/max-H" label mentions `E` but no seismic action exists.
- Concentrated load: full `γ_Q` on characteristic force (`load-case-checks:364`).
- Fire combination factor `ψ_fi = 0.7` fixed (`fire-durability…:26`).

### 8.4 Stubs, shortcuts and legacy traces (violations of repository rules)

- `fire_min_thickness_m` labelled "Compatibility" (`fire-and-design-effects:60-63`). The repo rules forbid compatibility layers.
- `MortarClass::compressive_strength_mpa` labelled "Alias used by older durability helpers" (`📸️snapshot/🦀️.rs:349-350`). Legacy alias.
- `render_document_json` labelled "Legacy pretty JSON — kept for tests" (`app-surface/📝️document-editors/🦀️.rs:180-183`).
- Mutation groups with `-2` suffix: `🧰️loads-2`, `🪜️walls-2`, and parallel `⚖️loads`, `🧱️walls` (`SUB/🧬️schema/🧬️mutations/`, and in `ART/Cargo.toml` members). Suggests a split that was not cleaned up.
- Editor `evaluate` command is a no-op with a comment about a pre-migration version (`🧮️evaluate/🦀️.rs:4-7`).
- `InteractiveJobClassification::Migrated` used for every editor action (`✏️editor/🦀️.rs` manifest).
- "SDK GAP" comments in editor and viewer (`✏️editor/🦀️.rs:42`, viewer `:20-22`, report window `:9-11`).
- `DOCUMENT_SCHEMA` string literal duplicated in the editor (`✏️editor/🦀️.rs` `const DOCUMENT_SCHEMA`), while the model defines `EN1996_DOCUMENT_SCHEMA`.
- `imposedCategory` strings: F and G rejected by integrity, while psi0 returns 0.7 for any unknown category. The two rules disagree.
- `MortarType::Lightweight` exists in the model and field meta but no formula treats it differently from general purpose.
- `warn_above` is never used, so `Warning` status is unreachable in masonry checks.
- `WallType::Shear` and `NonLoadBearing` only affect the simplified applicability list.
- Oracle test writes to system temp, against the ticket-folder rule (section 6.3).

### 8.5 Not implemented (overall)

- Confined masonry, ties and anchors, prestressed masonry, shear walls and bracing checks, full reinforced design, seismic (E) action, snow zones, EN 1996-1-2 structural fire, EN 1996-2 Annex B beyond the simple exposure table.
- Visuals: no 3D, no 2D diagrams (interaction curves, M-N, stress block, utilisation heat map). The editor is a property tree and text; the viewer is a table.
- Live feedback: each edit goes through a mutation and a re-evaluation on read. No visual progress or cancellation; the six commands are interactive jobs but the evaluation itself is a synchronous pure function.
- Multi-user: presence and drafts empty.
- Tests: no committed compliance golden reports; no third-party cross-check of compliance numbers beyond the Python second implementation (see 6.3).

## 9. Paths referenced

- Model: `SUB/🧬️schema/📸️snapshot/🦀️.rs`
- Artifact schema: `SUB/🧬️schema/🦀️.rs`
- Inference and outline: `SUB/🧬️schema/💡️inferences/🦀️.rs`, `SUB/🧬️schema/💡️inferences/🧾outline/🦀️.rs`
- Checks: `SUB/🧬️schema/⚖️masonry/🦀️.rs`, `…/🧱️evaluate-wall/🦀️.rs`, `…/📏️factors-and-geometry/🦀️.rs`, `…/🔥️fire-and-design-effects/🦀️.rs`, `…/🛡️conformance-and-integrity/🦀️.rs`, `…/🧱️evaluate-wall/📏️wall-geometry-and-material/🦀️.rs`, `…/🏋️load-case-checks/🦀️.rs`, `…/🔥️fire-durability-and-reinforcement/🦀️.rs`
- Shared kernel: `ART/../../⚖️compliance/✅️check-results/🦀️.rs` (repo path `✏️s/🔌️plugins/📕️norm/⚖️compliance/✅️check-results/🦀️.rs`)
- Mutations: `SUB/🧬️schema/🧬️mutations/🦀️.rs`, `…/🧭️edit-rules/🦀️.rs`
- Editor: `SUB/✏️editor/🦀️.rs`, `…/🎮️commands/*`, `…/🏷️field-meta/🦀️.rs`, `…/📌️panels/*`, `…/🎭️modes/✏️edit/🪟️windows/*`
- Viewer: `SUB/👁️viewer/🦀️.rs`, `…/🪟️windows/📊️report/🦀️.rs`
- Oracle: `SUB/🔮️oracles/⚖️compliance/🐍️.py`, `SUB/🔮️oracles/🔣️.json`, `SUB/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs`
- Mutation test: `SUB/🧪️tests/🪨️mutate-en1996-1/{🐍️.py,🦀️.rs,🥒️.feature}`
- Packages: `ART/📦️packages/🦀️rust/…`, `ART/📦️packages/🟦️typescript/…`, `ART/Cargo.toml`
