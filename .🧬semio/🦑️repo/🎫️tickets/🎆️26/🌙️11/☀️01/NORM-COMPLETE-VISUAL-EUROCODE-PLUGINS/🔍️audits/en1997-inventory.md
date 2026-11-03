# 🌍 EN 1997 Inventory

Read-only audit of the EN 1997 artifact at `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997` (subset root `🏅️standards/🔖️1/🪆️subsets/✳️any`, abbreviated `SUB/` below). No source file was modified. All paths are relative to `SUB/` unless absolute. Line numbers refer to the current working tree. Numeric statements marked "computed" were checked with a stdlib Python one-liner run during the audit.

Scope note: 509 tracked files, of which 182 are under `🧬️schema`, 133 under `🧪️tests` (mostly committed mutation fixtures), 79 under `🚪️io`, 60 under `✏️editor`, 27 under `👁️viewer`.

---

## 0. Architecture at a glance

- Snapshot (persisted domain document): `SUB/🧬️schema/📸️snapshot/🦀️.rs`. Artifact mirror: `SUB/🧬️schema/🦀️.rs`.
- Checks (the only compliance logic in the artifact): `SUB/🧬️schema/💡️inferences/⚖️checks/**`, entry `check_project`. The inference `evaluate` (`SUB/🧬️schema/💡️inferences/🦀️.rs:82-84`) is a one-line wrapper around it.
- Derived inference `outline` (`SUB/🧬️schema/💡️inferences/🧾outline/🦀️.rs`) summarises the report (field count, check count, pass count, governing check).
- Mutations: 20 typed kinds, `SUB/🧬️schema/🧬️mutations/🦀️.rs`, edit routing in `SUB/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`.
- Shared compliance kernel (not in this artifact): `✏️s/🔌️plugins/📕️norm/⚖️compliance` (`CheckResult`, `CheckReport`, `Remedy`, `ClauseId`, `AnnexChoice`, `NormHost`). Shared UI surface (not in this artifact): `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`.
- Editor: `SUB/✏️editor/🦀️.rs`. Viewer: `SUB/👁️viewer/🦀️.rs`. Oracles: `SUB/🔮️oracles/`, `SUB/🧪️tests/`.
- Workspace: root `Cargo.toml` lists 15 members (14 subset crates plus the packages root); `**/🔮️oracles/**` is excluded from the Cargo workspace.

---

## 1. Model

### 1.1 Snapshot (`SUB/🧬️schema/📸️snapshot/🦀️.rs`)

Event-sourced document root, `#[dsl(id = "norm.en1997", layout = "lines")]`, `#[artifact_schema(id = "s.norm.en1997")]`, `DOCUMENT_SCHEMA = "semio.norm.en1997/v1"` (`SUB/✏️editor/🦀️.rs:55`).

`En1997Snapshot` (lines 23-58), 13 fields, all `#[state(artifact)]`:

| Field | Type | Unit / note |
|---|---|---|
| `structure_id` | String | |
| `geotechnical_category` | u8 | GK 1-3 (checked 1..=3) |
| `design_situation` | String | `bsP`/`bsT`/`bsA` (parsed leniently) |
| `design_approach` | String | `da1str`, `da1geo`, `da2`/`geo2`, `da3`, `geo3` |
| `annex` | `AnnexChoice` (external, `en`/`de`) | |
| `groundwater_level` | f64 | m |
| `investigation_depth` | f64 | m |
| `layers` | Vec<SoilLayer> | table |
| `footings` | Vec<SpreadFoundation> | table |
| `piles` | Vec<Pile> | table |
| `retaining_walls` | Vec<RetainingWall> | table |
| `slopes` | Vec<Slope> | table |
| `uplift_cases` | Vec<UpliftCase> | table |

Nested records (all `DslRecord`, `ToValue`, `FromValue`, `CanonicalJsonTree`; camelCase on the wire):

- `SoilLayer` (255-275), 13 fields: `id`, `soil_type` (free String, not enum), `depth_top`, `depth_bottom` (m), `gamma`, `gamma_prime` (N/m³), `phi_prime_deg`, `cohesion_effective`, `cohesion_undrained`, `oedometric_modulus` (Pa), `poisson_ratio`, `cpt_qc` (Pa), `spt_n`.
- `FoundationLoadCase` (283-298), 8 fields: `id`, `design_situation` (empty = inherit snapshot), `vertical_permanent`, `vertical_variable` (N), `horizontal_permanent`, `horizontal_variable` (N), `moment_permanent`, `moment_variable` (J, i.e. N·m as typed).
- `SpreadFoundation` (306-319), 7 fields: `id`, `width`, `length`, `embedment` (m), `base_inclination_deg`, `settlement_limit` (m), `load_cases` (table).
- `PileTestProfile` (327-333), 3 fields: `id`, `shaft_resistance`, `base_resistance` (N).
- `Pile` (341-364), 13 fields: `id`, `pile_type` (free String: bored/driven/cfa and aliases), `diameter`, `length`, `count: u32`, `alpha_s`, `unit_shaft_resistance`, `unit_base_resistance` (Pa), `compression_permanent`, `compression_variable`, `tension_permanent`, `tension_variable` (N), `test_profiles` (table).
- `RetainingWall` (371-398), 15 fields: `id`, `height`, `embedment`, `base_width`, `stem_thickness`, `backfill_phi_deg`, `backfill_gamma`, `wall_friction_deg`, `earth_pressure_mode` (String: active/atRest/increasedActive), `wall_movement` (String: free/rigid/propped), `ocr`, `concrete_gamma`, `surcharge` (Pa), `vertical_permanent`, `horizontal_permanent` (N).
- `Slope` (405-413), 5 fields: `id`, `angle_deg`, `height`, `length`, `governing_layer_id`.
- `UpliftCase` (420-432), 6 fields: `id`, `permanent_stabilizing`, `permanent_destabilizing`, `variable_destabilizing` (N), `pore_pressure`, `total_stress` (Pa).

Not modelled at all: anchors/ground anchors, pile groups, negative skin friction, per-layer groundwater, lateral pile resistance, seismic/accidental load parameters beyond the BS-A label, load-combination tables (combinations are implicit), laboratory test records, retaining-wall props/anchors, pile settlement, slope water/seismic, footing overturning/tilt.

Example constructors in the snapshot file:
- `compliant_demo()` (68-180): structure `geo-demo-compliant`, GK2, bsP, DA2, DE, GWL 4 m, investigation 12 m; two layers (sand 0-8 m, gravel 8-20 m), footing F1 2.5x2.5 m, bored pile P1 D0.6 L14 with three test profiles, wall W1 H4 m, slope S1 25° H6 m, uplift U1.
- `noncompliant_demo()` (183-229): derived from the compliant one with deliberate failures (footing 1.2 m, low E_oed, short pile, small wall base, slope 42°, uplift overloaded).
- `impl Default` returns `compliant_demo()`.

### 1.2 Artifact mirror (`SUB/🧬️schema/🦀️.rs`)

`En1997Artifact` (22-49) has the same 13 fields as the snapshot. Conversions: `to_snapshot`, `from_snapshot`, `From<En1997Snapshot>`, `Default` via `from_snapshot(En1997Snapshot::default())`. Descriptor `en1997_artifact_schema_descriptor` (106-114) wires facet leaves (`🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`).

### 1.3 Schema facet leaves

For each of `snapshot`, `artifact`, `diff`, `mutations`, `inferences`, `presence`, `config` there are five hand-written leaves: Rust, TypeScript, GraphQL, JSON Schema, Proto. Examples: `SUB/🧬️schema/📸️snapshot/🔣️.json`, `SUB/🧬️schema/🔣️.json`, `SUB/🧬️schema/🔺️diff/`, `SUB/🧬️schema/🧬️mutations/🔣️.json`. `SUB/🏅️standards/🔖️1/🪆️subsets/🔣️.json` is the subset catalogue.

### 1.4 Diff and inference families

- Diff: `SUB/🧬️schema/🔺️diff/🦀️.rs` (`En1997Diff`, referenced by `#[mutations(diff = En1997Diff)]`).
- Inference: `SUB/🧬️schema/💡️inferences/🦀️.rs` (`En1997Inference`, lines 27-30, one `#[derived]` field `outline`).
- Outline: `SUB/🧬️schema/💡️inferences/🧾outline/🦀️.rs` (`En1997Outline`, 27-38): `section_outline` (13 field names, lines 6-20), `field_count`, `entry_count` (sum of collections), `check_count`, `pass_count`, `all_pass` (`report.complies()`), `governing_clause`, `governing_utilization`, `governing_situation` (string-matched "BS-T"/"BS-A"/else "BS-P", lines 66-73), `governing_approach`. Governing check = the `en1997.governing.situation` row if present, else the max-utilisation non-N/A row (lines 54-64).

### 1.5 Mutation vocabulary (`SUB/🧬️schema/🧬️mutations/🦀️.rs`)

`En1997Mutation` enum, 20 variants (tag `mutation`, camelCase), `KINDS` list at lines ~57-78:

- `change-annex`, `change-geotechnical-category`, `change-design-situation`, `change-design-approach`, `change-groundwater-level`, `change-investigation-depth`, `change-footing-width`, `change-footing-embedment`, `change-pile-length`, `change-pile-count`, `change-wall-base-width`, `change-slope-angle`, `change-layer-phi-prime`, `change-layer-oedometric-modulus` (14 change kinds)
- `insert-layer`, `insert-footing`, `insert-pile`; `remove-layer`, `remove-footing`, `remove-pile` (6 insert/remove kinds)

Each kind has its own leaf directory with `🦀️.rs`, `↩️inverse/`, `🔺️diff/`, and a `🔣️.json` schema.

Edit routing (`SUB/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`, `EDIT_RULES`): 14 `SetFieldRule`s (lines 10-23), 3 `InsertItemRule`s (26-28), 3 `RemoveItemRule`s (31-33). Only these paths can be set through `set-field`, `insert-item`, `remove-item`, and `apply-remedy`:

`footings[].width`, `slopes[].angleDeg`, `footings[].embedment`, `layers[].oedometricModulus`, `annex`, `groundwaterLevel`, `designSituation`, `piles[].length`, `layers[].phiPrimeDeg`, `investigationDepth`, `piles[].count`, `geotechnicalCategory`, `designApproach`, `retainingWalls[].baseWidth`; inserts/removes for `footings`, `layers`, `piles`.

Everything else in the snapshot (for example `layers[].gamma`, `cohesionEffective`, `cohesionUndrained`, `piles[].diameter`, `retainingWalls[].height`, `upliftCases[]`, `footings[].loadCases[]`, `slopes[].height`, `retainingWalls[].surcharge`) has no typed mutation. Remedies that target such paths render as "Not auto-applicable" (`✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🌳️report-tree/🦀️.rs`, `build_check_tree_item` else-branch).

---

## 2. Checks and verifications

Entry: `check_project(doc)` in `SUB/🧬️schema/💡️inferences/⚖️checks/🏗️project-checks/🦀️.rs:5-100`. It runs duplicate-id integrity, GK check, soil-layer checks, spread-foundation checks, then `check_piles_walls_slopes_and_uplift` (`🧱️pile-wall-slope-and-uplift-checks/🦀️.rs:6-11`), then a governing-situation summary.

Clause ids below are the `ClauseId::new(...)` line (the `CheckResult::assess` call is 2-3 lines above). "Part" is the `part` label passed to `assess`.

### 2.1 Design approach and national annex (DIN 1054 NA)

File: `🧭️design-approach-and-parameters/🦀️.rs`.

- `na_de` constants (5-9): γ_c 1.4, γ_φ 1.25, γ_γ 1.0.
- `DesignApproach` enum (12-20): `Da1Str`, `Da1Geo`, `Da2`, `Da3`, `Geo2`, `Geo3`. `Geo2` is an alias of `Da2` in `parse_design_approach` (line 210) so the variant is only reachable programmatically.
- `AnnexParams` (22-35): `gamma_g`, `gamma_q`, `gamma_r_v`, `gamma_r_h`, `gamma_c`, `gamma_phi`, `gamma_gamma`, `gamma_b`, `gamma_s`, `gamma_st`, `gamma_r_e`.
- `annex_params` (49-90), base table per approach and annex; values hard-coded:
  - DA2/GEO-2 DE: γ_G 1.35, γ_Q 1.5, γ_R,v 1.4, γ_R,h 1.1, γ_c 1.0, γ_φ 1.0, γ_γ 1.0, γ_b 1.1, γ_s 1.1, γ_st 1.1, γ_R,e 1.1.
  - DA2/GEO-2 EN: as DE except γ_R,v 1.0, γ_R,h 1.0.
  - GEO-3 (both annexes): γ_G 1.0, γ_Q 1.3, γ_c 1.25, γ_φ 1.25, γ_b 1.1, γ_s 1.1, γ_R,e 1.1.
  - DA1-C1 (STR) EN: γ_G 1.35, γ_Q 1.5, γ_c 1.25, γ_φ 1.25, γ_b 1.0, γ_s 1.0.
  - DA1-C1 (STR) DE: γ_c 1.4 (na_de), γ_φ 1.25 (na_de).
  - DA1-C2 (GEO): γ_G 1.0, γ_Q 1.3, γ_R,v 1.4, γ_R,h 1.1, γ_c 1.0, γ_φ 1.0 (both annexes).
  - DA3 EN: γ_G 1.35, γ_Q 1.5, γ_c 1.25, γ_φ 1.25, γ_b/γ_s/γ_st 1.15, γ_R,e 1.1. DA3 DE: same with na_de material factors.
- `DesignSituation` (94-100): `Persistent`, `Transient`, `Accidental`; `parse_design_situation` (102-108) maps unknown strings silently to Persistent.
- `resolve_params(approach, annex, situation)` (114-172): DA2* rule (DE only) sets γ_c = γ_φ = γ_γ = 1.0 (lines 117-122). Persistent DE DA2 sets γ_G 1.35, γ_Q 1.5, γ_R,v 1.4, γ_R,h 1.1 (127-131); GEO-3/DA1-C2 DE sets γ_G 1.0, γ_Q 1.3 (133-136). Transient: γ_G 1.20, γ_Q 1.30 (143-144); DA2 DE transient γ_R,v 1.30, γ_R,h 1.10 (146-147); GEO-3/DA3 DE transient multiply γ_φ and γ_c by 0.96 floored at 1.0 (151-154). Accidental: γ_G 1.00, γ_Q 1.00 (161-162); DA2 DE accidental γ_R,v 1.20, γ_R,h 1.10 (164-165).
- UPL/HYD factors `resolve_upl_params` (183-205) per annex and situation: DE persistent γ_G,stb 0.90, γ_G,dst 1.10, γ_Q,dst 1.50, γ_H 1.50; DE transient 0.95 / 1.05 / 1.30 / 1.30; DE accidental all 1.00; EN uses the same transient/accidental values and the DE persistent values. Doc comment cites "DIN 1054 Table A.2.1 / EN 1997-1 §10".
- `parse_design_approach` (207-215): unknown strings silently become DA1-C1 (`_ =>` arm).

Unit-level checks of these laws exist in `SUB/🧬️schema/🧪️tests/⚖️compliance/🧮️design-approach-and-resistance-laws/🦀️.rs` (18 tests, see §6).

### 2.2 Layer strength, duplicate ids, governing summary, GK

File: `🏗️project-checks/🦀️.rs` and `🪨️layer-and-duplicate-id-helpers/🦀️.rs`.

| Check id pattern | Part | Clause | Location | Formula / rule |
|---|---|---|---|---|
| `en1997.gk.category` | DIN 1054 | A.2 (line 34) | project-checks 31-53 | GK in 1..=3; otherwise fail with remedy one-of {1,2,3}. |
| `en1997.layer.strength.<id>` | EN 1997-1 | 3.3 (121) | project-checks 103-154 | Cohesive (see 4.1): `c_u ≥ max(1, 0.15·γ·z̄)` where z̄ is mid-depth. Non-cohesive: index `φ′ + c′/1000 ≥ 20`. Heuristic, not an EN clause. |
| `en1997.2.phi.cpt.<id>` | EN 1997-2 | 2.4.3 (166) | project-checks 161-190 | φ′_stated ≤ φ′_CPT from `phi_from_cpt_deg` (part_2 559-562). |
| `en1997.2.phi.spt.<id>` | EN 1997-2 | 2.4.3 (197) | project-checks 192-221 | φ′_stated ≤ φ′_SPT from `phi_from_spt_deg` (part_2 564-566). |
| `en1997.2.phi.derived.<id>` | EN 1997-2 | 2.4.3 (228) | project-checks 223-252 | Same test as above with the min of CPT/SPT (`investigation_phi_deg`). |
| `en1997.integrity.duplicate.<table>.<id>` | EN 1997-1 | 1.5 (85) | layer-and-duplicate-id-helpers 55-109, 111-189 | Duplicate ids in layers, footings, loadCases, piles, testProfiles, retainingWalls, slopes, upliftCases. Remedy: rename to `<id>-2` or `<id>-unique`. |
| `en1997.governing.situation[...]` | EN 1997-1 | 2.4.7 (207) | layer-and-duplicate-id-helpers 191-222 | Summary row with utilisation `u` and limit `max(u, 1)`. See 6.1: this row cannot fail. |

Characteristic φ′ (part_2 595-605): `min(stated, investigation)` where investigation = CPT/SPT correlation (`investigation_phi_deg`, 577-592); the min is taken, the derived value is used only if lower than stated. Used for bearing and sliding (`phi_use`, project-checks 288).

### 2.3 Spread foundations (`en1997.6.5.*`, `en1997.6.6.*`, `en1997.2.4.2.*`)

Entry: `check_footings` (project-checks 258-296), per footing: `check_footing_load_cases` (300-505), `check_footing_uplift_and_bounds` (509-594), `check_footing_governing_situation` (598-648). Footing soil layer = `layer_at(layers, embedment)` (layer-and-duplicate-id-helpers 5-7; fallback is the first layer, or an invented 'default' sand layer: γ 18 kN/m³, γ′ 10 kN/m³, φ′ 30°, E_oed 30 MPa; project-checks 270-274).

Actions per load case (project-checks 319-325): `V_d = γ_G·V_G + γ_Q·V_Q`, `H_d = γ_G·H_G + γ_Q·H_Q`, `M_d = γ_G·M_G + γ_Q·M_Q`, eccentricity `e = M_d/max(V_d,1)`, all factored by `resolve_params` for the load case's situation. Foundation area `A = B·L`.

Bearing (`en1997.6.5.bearing.<fid>.<lcid>`, ClauseId line 349):
- GK1: presumptive bearing table `presumptive_bearing_pa` (part_1 544-553), `R = q_all·A / γ_R,v` (project-checks 332-334). Clause `DIN 1054 A.6.10`.
- Short-term (BS-T/BS-A) cohesive layer with c_u > 0: undrained Annex D.3 (`undrained_bearing_resistance_n`, part_1 276-281; `undrained_bearing_pa`, 265-274: `q_u = (π+2)·c_u·s_c·i_c + γ·D`, `s_c = 1+0.2·B/L`).
- Otherwise drained Annex D (`design_bearing_resistance_with_params`, part_1 130-142; `ultimate_bearing_pa`, 103-121): `q_u = c·N_c·s_c·d_c·i_c·b_c + q·N_q·s_q·d_q·i_q·b_q + ½·γ′·B′·N_γ·s_γ·d_γ·i_γ·b_γ`, `R = q_u·B′·L′/γ_R,v`.
- Factor laws (part_1): `N_q = e^(π·tanφ)·tan²(45°+φ/2)` (15-18), `N_c = (N_q−1)·cotφ` with 5.14 at φ=0 (21-25), `N_γ = 2·(N_q−1)·tanφ` (28-30). `s_q = 1 + (B/L)·sinφ`, `s_γ = max(0.7, 1−0.3·B/L)` (37-45). `d_q = 1 + 2·tanφ·(1−sinφ)²·arctan(D/B)`, `d_c = d_q − (1−d_q)/(N_c·tanφ)` (47-54). `i_q = (1 − H/(V + A·c·cotφ))^m`, `m = (2+B/L)/(1+B/L)`, `i_c = i_q − (1−i_q)/(N_c·tanφ)`, `i_γ = (1−…)^(m+1)` (56-68). Base tilt `b_q = (1−α·tanφ)²`, `b_c = b_q − (1−b_q)/(N_c·tanφ)`, `b_γ = b_q` (71-86).
- Effective width `B′ = B − 2|e|` (33-35). Effective γ below base with GWL linear interpolation over zone width B (89-100).
- Remedy: `at_least` footing width to `b_req` from `find_required_width` (layer-and-duplicate-id-helpers 11-37; width ×1.08 steps from start, first width with `V ≤ R` times 1.02 rounded up to mm, stop at 80 m).

Sliding (`en1997.6.5.3.sliding.<fid>.<lcid>`, ClauseId 388), project-checks 385-417: `R_h,d = (c′·A + V_d·tanφ_d)/γ_R,h + passive`, `φ_d = atan(tanφ/γ_φ)`, passive `½·E_p,k` only if embedment > 0 and divided by γ_R,h (part_1 157-170). Passive E_p = `½·γ·D²·K_p·L` with `K_p = kp_coulomb(φ, δ=0)` (part_1 145-151). Remedy: reduce horizontal variable action or widen to `max(b_req, 1.2·B)`.

Undrained sliding (`en1997.6.5.3.sliding.undrained.*`, ClauseId 454), project-checks 451-472: `R = A·c_u/γ_R,h` (part_1 284-286). Only if `c_u > 0` or cohesive.

Undrained bearing (`en1997.6.5.bearing.undrained.*`, ClauseId 426 "EN 1997-1 6 D.3"), project-checks 420-449: `R_d = q_u·B′·L′/γ_R,v` with `c_u/γ_c` and `γ/γ_γ`, runs for every footing with `c_u > 0` or cohesive layer (even when situation is persistent). Remedies: raise c_u or widen by `√(V/R)`.

Eccentricity (`en1997.6.5.4.eccentricity.*`, ClauseId 484 "DIN 1054 7.5"), project-checks 475-503: `e_perm = M_G/V_G ≤ B/6` and `e_tot = (M_G+M_Q)/(V_G+V_Q) ≤ B/3`, computed on **unfactored** characteristic actions. Remedy: widen to `max(6·e_perm, 3·e_tot)`.

Settlement (`en1997.6.6.settlement.<fid>`, ClauseId 534 "EN 1997-1 6.6"), project-checks 509-560: `q_SLS = max over load cases of (V_G+V_Q)/A` (unfactored). Oedometric layer-wise `s = Σ Δσ/E_oed·Δz`, with `Δσ = q·B·L/((B+z)(L+z))` (2:1 method, part_1 173-178), layer mid-depth `z` measured below embedment (part_1 187-225). Buoyancy factor `clamp(γ′/γ, 0.4, 1.0)` applied where GWL is above the mid-depth (part_1 205-206). Fallback when no layer contributes: `s = q/E·B·0.5` (part_1 216-223). Remedies: raise E_oed of the governing layer; raise the limit.

Immediate elastic settlement (`en1997.6.6.elastic.*`, ClauseId 569 "EN 1997-1 F.2"), project-checks 561-592: `s = q·B·(1−ν²)·I_f/E·√(B/L)` with `E = E_oed·(1+ν)(1−2ν)/(1−ν)`, layered with `I_f = 1/(1+z/B)` clamped 0.2..1.0, stress spread `(B/(B+z))²`, layer weight `dz/(2B)` clamped 0.02..1.0 (part_1 289-329). Note: EN Annex F.2 is not what this formula implements (see 6.2).

Investigation depth (`en1997.2.4.2.investigation.<fid>`, ClauseId 616 "EN 1997-2 2.4.2"), project-checks 598-634: `z_min = k·max(B,L)` with k = 2 (GK1), 3 (GK2, default), 3.5 (GK3) (part_2 567-574). Compared with `doc.investigation_depth`.

Footing NA rows: `en1997.footing.na`, `en1997.footing.<fid>.loads.na` when no load cases (project-checks 56-63, 279-286).

Governing footing summary: `en1997.governing.situation.footing.<fid>` (project-checks 635-647), uses the max utilisation among bearing/sliding of the footing (`foot_gov_u`).

### 2.4 Piles (`en1997.7.6.*`)

File: `🧱️pile-wall-slope-and-uplift-checks/🦀️.rs` 14-173; capacity laws in `📐️geotechnical-parts/🦀️.rs` part_1.

- Averages and minima over `testProfiles` (or single geometric estimate if empty) at lines 25-35: `R_s,mean, R_s,min, R_b,mean, R_b,min`, n = number of profiles. Geometric estimate: `R_s = α_s·π·D·q_s·L` (part_1 231-233), `R_b = q_b·π·(D/2)²` (235-237).
- Correlation factors ξ3, ξ4 (part_1 239-249): n=1 → (1.40, 1.40); n=2 → (1.35, 1.27); n=3,4 linear interpolation to (1.30, 1.15); n≥5 → (1.30, 1.15).
- Type factors `pile_type_factors` (part_1 332-347): driven → γ_b ≥ 1.10, γ_s ≥ 1.15 and hard-coded ξ (1.35/1.35 for n≤1, 1.30/1.25 for n=2, 1.25/1.15 otherwise); CFA → γ_b ≥ 1.15, γ_s ≥ 1.15, ξ ≥ 1.40; other (bored) → γ_b, γ_s ≥ 1.10 with the default ξ.
- Characteristic `R_k = min(mean/ξ3, min/ξ4)` (line 42-43 for base and shaft), then geometric cap `min(R_k, R_geom/ξ3)` (47-48).

Compression (`en1997.7.6.2.compression.<pid>`, ClauseId 58 "EN 1997-1 7.6.2"), lines 55-81: `R_c,d = (R_b,k/γ_b + R_s,k/γ_s)·count`, `N_ed = γ_G·N_G + γ_Q·N_Q` per situation (max utilisation situation selected, lines 147-160). Remedies: pile count ≥ `ceil(N_ed/(R_c,d/count))`, or pile length scaled by `N_ed/R_c,d`.

Tension (`en1997.7.6.3.tension.<pid>`, ClauseId 93 "EN 1997-1 7.6.3"), lines 86-145: runs when `T_ed > 1` or a tension action is given. `R_t,d = R_s,k/γ_s·count` (shaft only). Remedies: more piles, reduce variable tension (with a `t_var_cap` estimate), reduce permanent tension.

Not checked: lateral pile capacity, group efficiency, pile settlement, negative skin friction, base resistance in tension, pile structural capacity.

Wall-related null row: `en1997.wall.na` (line 167) when no walls.

### 2.5 Retaining walls (`en1997.9.*`, DIN 4085)

File: `🧱️pile-wall-slope-and-uplift-checks/🦀️.rs` 176-391.

- Earth-pressure coefficient `earth_pressure_coefficient` (part_1 381-398): `atRest` → K₀ Jáky; `increasedActive` → `½(K_a + K₀)` (labelled "Ka*"); `active` → Coulomb `K_a` if δ ≠ 0 else Rankine (labelled Ka-Coulomb / Ka-Rankine).
  - `ka_rankine = tan²(45°−φ/2)` (362-365, min 0.05).
  - `k0_jaky = max(1−sinφ, 0.15)·OCR^sinφ` (374-378).
  - **`ka_coulomb` (400-406) is not the Coulomb formula** (see 6.1; computed: φ=30°, δ=20° → 0.633 vs 0.297 textbook; at δ=0 → 0.464 vs Rankine 0.333).
  - `kp_coulomb = 1/K_a` (408-411), not a Coulomb passive coefficient.
- Active thrust `E_a = ½·K·γ·H² + K·q·H` (line 189), with `K` from the selected mode.
- Sliding (`en1997.9.sliding.<wid>`, ClauseId 236 "EN 1997-1 9.7.3"), lines 233-280: the governing situation maximises `H_d/R_slide` across load-case situations (lines 192-209). `H_d = γ_G·(H_G + E_a)`; `V_d` includes concrete self-weight `b·t·γ_c` (with GWL-dependent buoyancy blend, lines 213-225). `R_slide = (V_d·tanφ)/γ_R,h + ½·E_p,k/γ_R,e`, with `E_p,k` over a 1 m strip (lines 228-231). Remedies: widen base, increase embedment (`E_p,need = 2(H_d−R_base)γ_R,e` scaling), reduce height.
- Earth-pressure regime (`en1997.9.earthPressure.<wid>`, ClauseId 304 "DIN 4085 6.1"), lines 282-342: checks that `earthPressureMode` ∈ {active, atRest, increasedActive}, `wallMovement` ∈ {free, rigid, propped} and that the pair is consistent (free↔active; rigid/propped↔atRest/increasedActive). The "utilisation" is a sentinel: `(K₀ clamped, 1.0)` on success, `(2.0, 1.0)` on regime failure. This row does not compute a DIN 4085 check (see 6.3).
- Overturning (`en1997.9.overturning.<wid>`, ClauseId 352 "EN 1997-1 9.7.2"), lines 344-367: `M_dst = H_d·H/3`, `M_stb = V_d·B/2`. Simplified geometry, no earth-pressure moment, no passive contribution. Remedy: widen base to `B·M_dst/M_stb`.
- Governing wall summary `en1997.governing.situation.wall.<wid>` (368-380).

### 2.6 Slopes (`en1997.11.bishop.*`)

File: `🧱️pile-wall-slope-and-uplift-checks/🦀️.rs` 393-474; solver in part_1 421-541.

- Partial factors: GEO-3 DE/EN from `DesignApproach::Geo3.annex_params` (line 396), independent of the document's design approach (this is intentional per the comment at part_1 421-422).
- FoS `bishop_fos_slices` (part_1 424-466): circle-search grid `xc ∈ {0.25,0.4,0.55,0.7,0.85}·L`, `yc ∈ {0.8,1.1,1.4,1.8,2.2}·H`, radii `{r_toe, r_crest, mean, 1.15·r_toe, 1.15·r_crest}`, 16 slices (line 442), simplified Bishop iteration with 40 steps and damping 0.5 (468-530). FoS accepted in (0.05, 50). Falls back to `bishop_fos` (legacy closed form, 533-541) when no circle converges.
- Layer properties are taken at slice mid-depth from the governing layer lookup `layer_props_at` (413-419); submerged slices use γ′ and pore pressure `γ_w·(GWL−z)`.
- Clause ids: `EN 1997-1 11.5` (lines 407, 437). Check passes when FoS ≥ 1.0 (`minimum(fos, 1.0)`).
- Remedies when FoS < 1: angle × 0.85 (min 10°), height × 0.9 (min 1 m).
- Missing governing layer: check fails with `one_of` remedy over layer ids (lines 433-463).
- Null row `en1997.slope.na` (lines 383-390).

### 2.7 Uplift and heave (`en1997.10.*`)

File: `🧱️pile-wall-slope-and-uplift-checks/🦀️.rs` 476-565.

- Factor selection: `resolve_upl_params` per situation, governing = max `γ_dst·G_dst + γ_Q·Q_dst` over `γ_stb·G_stb`, lines 483-494.
- UPL (`en1997.10.upl.<uid>`, ClauseId 501 "EN 1997-1 10.2"): `G_stb,d = γ_stb·G_stb ≥ V_dst,d = γ_dst·G_dst + γ_Q·Q_dst` (lines 495-511). Remedy: raise stabilising force to `dest/max(γ_stb, 0.5)`.
- HYD (`en1997.10.hyd.<uid>`, ClauseId 528 "EN 1997-1 10.3"): `σ ≥ u_d = u·γ_H` (lines 522-549), with `u` = `pore_pressure` if > 1 Pa else `9810·max(GWL,0)`. Remedies: raise total stress, or lower GWL to `0.7·GWL`.
- Null row `en1997.upl.na` (lines 466-473).

### 2.8 Remedy inventory

Remedy constructors (`Remedy::at_least`, `at_most`, `one_of`) appear at: pile count/length/tension (`pile-wall...` 70-80, 116-139), wall sliding/overturning/regime (263-278, 315-340, 360-365), slope (419-429, 448-460), uplift (513-518, 539-549), footing width/eccentricity/sliding/undrained (project-checks 360-365, 399-409, 437-447, 496-501), layers (133-151, 441-470 region), settlement (548-558, 580-590), investigation (627-632), duplicate ids (98-105). Remedy targets whose path is not in `EDIT_RULES` (section 1.5) are not auto-applicable.

---

## 3. NA-DE (German national annex, DIN 1054) parameters in use

All values are hard-coded in code, not loaded from data.

| Use | Value | Location |
|---|---|---|
| NA material factors (DA1-C1 and DA3, DE) | γ_c 1.4, γ_φ 1.25, γ_γ 1.0 | `🧭️design-approach-and-parameters` lines 5-9 |
| DA2*/GEO-2 DE persistent | γ_G 1.35, γ_Q 1.50, γ_R,v 1.40, γ_R,h 1.10, γ_b 1.1, γ_s 1.1, γ_R,e 1.1; γ_c=γ_φ=γ_γ=1 | 51-54, 127-131 |
| DA2* DE transient | γ_G 1.20, γ_Q 1.30, γ_R,v 1.30, γ_R,h 1.10 | 143-147 |
| DA2* DE accidental | γ_G 1.00, γ_Q 1.00, γ_R,v 1.20, γ_R,h 1.10 | 161-165 |
| GEO-3 DE | γ_G 1.0, γ_Q 1.3, γ_c 1.25, γ_φ 1.25 (persistent); transient ×0.96 floored at 1.0 | 59-62, 133-136, 151-154 |
| UPL/HYD DE persistent | γ_G,stb 0.90, γ_G,dst 1.10, γ_Q,dst 1.50, γ_H 1.50 | 192-194 |
| UPL/HYD DE transient | 0.95, 1.05, 1.30, 1.30 | 186-188 |
| UPL/HYD DE accidental | 1.00, 1.00, 1.00, 1.00 | 189-191 |
| Pile factors | γ_b, γ_s floor 1.10 / 1.15 (driven and CFA variants) | part_1 332-347 |
| Presumptive bearing (GK1, DIN 1054 A.6.10) | gravel 400, sand 200, silt 150, clay 120, default 180 kPa; ×(1+0.1·min(D,3)) | part_1 544-553 |
| Eccentricity limits (DIN 1054 7.5) | e_perm ≤ B/6, e_tot ≤ B/3 | project-checks 477-478 |
| Investigation depth (EN 1997-2 2.4.2 as coded) | 2B (GK1), 3B (GK2), 3.5B (GK3) | part_2 567-574 |
| Earth-pressure modes (DIN 4085) | active, atRest (K₀ Jáky), increasedActive (½(Ka+K0)) | part_1 381-398 |
| Wall movement classes | free, rigid, propped | pile-wall 286-294 |
| Design situations | BS-P, BS-T, BS-A labels | layer-helpers 39-45 |

---

## 4. Editor (`SUB/✏️editor`)

### 4.1 Application and commands (`✏️editor/🦀️.rs`)

- Identity: `LABEL = "EN 1997"`, `VARIANT = "en1997"`, `CONTROLLER_ID = "s.norm.en1997@1/*#editor"` (lines 50-54).
- Commands (`semio_framework_plugin::app_commands!`, lines 59-73), 6 entries:
  - `evaluate` → `Emit::default()`, no mutation (`🎮️commands/🧮️evaluate/🦀️.rs`). Recomputation happens on every read.
  - `setSelectedCheckIndex` (kebab `selected-check`) → view-only, changes the Results window config (`☑️selected-check`).
  - `setField` (`set-field`) with `path`, `value_json` → `dispatch_set_field` → `resolve_edit` (EDIT_RULES). Only the 14 mapped paths can be set.
  - `insertItem` (`insert-item`) with `path`, `index`, optional `value_json`; `removeItem` (`remove-item`) with `path`, `index`. Only layers/footings/piles.
  - `applyRemedy` (`apply-remedy`) with `checkId`, `remedyIndex` → `dispatch_apply_remedy` (`🩹apply-remedy`).
- Manifest (`create_en1997_app`, lines 235-291): mode `edit` (default), layout inputs | results 42/58 (`🎭️modes/✏️edit/🦀️.rs:15-17`), panels document / catalogue / inspection, keybindings `mod+z` undo and `mod+shift+z` redo, action descriptions (localized, EN/DE).
- Media: `export_media` `report:out` (JSON `CheckReport`, schema = artifact kind id) and `artifact:out` (pack, base64). `import_media` `model:in` accepted as inert; `artifact:in` refused (app-surface `🪟️manifest-and-media/🦀️.rs`, lines 108-141).
- Config and presence: `NoConfig`, `NoConfigMutation`, `NoPresence`. Results window has its own `ResultsWindowConfigOwner` holding `selected_check_index`.
- Examples (`examples()`, line 82-84): demo, compliant, noncompliant (see section 7).

### 4.2 Windows

Two window kinds, both in `🎭️modes/✏️edit/🪟️windows/`:

1. **Inputs** (`norm-en1997-inputs`, body `norm.en1997.play.inputs`, icon `download`): calls `render_document_editor(document, locale, controller, lookup_field_meta, windows)` (`📥️inputs/🦀️.rs:16-18`). This is a generic tree of the snapshot's value tree, one row per leaf. Enum-like fields become `select` controls with up to 32 options (localized EN/DE labels from `lookup_field_meta`, app-surface `📝️document-editors/🦀️.rs:150-166`); other leaves become text inputs committed on blur. Numbers display as `value unit` using the field-meta unit (document-editors 138-139). Nested collections are tree groups. It is a structured form, not raw JSON. The JSON renderer `render_document_json` (document-editors 181-184) is marked "Legacy" and is not used by this app. There are no 2D or 3D views.
2. **Results** (`norm-en1997-results`, body `norm.en1997.play.results`, icon `bar-chart-3`): `render_report(host.report(), …)` (`📊️results/🦀️.rs:23-25`). A virtualised flat list grouped by `part`: part rows with a verdict, then one row per check. Failing and warning checks expand into an Explanation child, and remedies. OneOf remedies show one child per option, each bound to `applyRemedy`. Non-auto-applicable remedies show "Not auto-applicable". Checks show the label `"<id> … u"` and the clause. No charts or heat maps.

Panels (all `PanelGroup`):
- **Artifact** (`norm.en1997.play.artifact`, Workbench): `render_summary` (app-surface document-editors 187+). Verdict "complies"/"does not comply", a list of parts with ✓/✗ marks, plus counts from the report summary.
- **Catalogue** (`norm.en1997.play.catalogue`, Workbench): `render_catalogue(examples, reference_tables)`. Lists the 3 examples, plus three reference tables from `✏️editor/📌️panels/📚️catalogue/🦀️.rs`:
  1. `partial_factors_table` (line 28): rows per approach/annex/situation with γ_G, γ_Q, γ_R,v, γ_R,h, …
  2. `upl_hyd_factors_table` (68): γ_G,stb, γ_G,dst, γ_Q,dst, γ_H by annex and situation.
  3. `bearing_n_factors_table` (108): φ′ (°), N_q, N_c, N_γ.
- **Inspection** (`norm.en1997.play.inspection`, Details): `render_inspection(report, selected_index)`. Shows one check in full: Id, Part, Clause, then Explanation and remedies. The index comes from the Results window config, default 0.

### 4.3 Field-meta (`✏️editor/📌️panels` lookups, and `lookup_field_meta` in `⚖️checks/🦀️.rs:39-171`)

- 83 entries (`NormFieldMeta { label_en … }`) covering every editable leaf in the snapshot with EN and DE labels, SI unit (`N/m³`, `Pa`, `N`, `N·m`, `m`, `°`), and choices where relevant.
- Choice lists: ANNEX (de, en), SITUATION (bsP, bsT, bsA), APPROACH (da1str, da1geo, da2, geo2, da3, geo3), PILE_TYPE (bored, driven, cfa), EP_MODE (active, atRest, increasedActive), WALL_MOVE (free, rigid, propped), SOIL (sand, gravel, clay, silt, fill).
- Units are displayed but not converted. Values are SI base units (N, Pa, m), which is why the snapshot uses `"Pa"` for `cohesionEffective` and `"N"` for forces. No unit conversion layer exists.
- The `unit` on `groundwaterLevel`, `investigationDepth` etc. is metres.
- `soilType` is a free-text field with a choice list; the check code also string-matches "clay", "ton", "cohesive" (see 2.2).

### 4.4 Editor tests

See section 6.

---

## 5. Viewer (`SUB/👁️viewer`)

- `En1997Viewer` implements `ArtifactViewer` only (no mutations, one inert `En1997ViewCommand::Noop`), `handle` returns `ViewEmit::default()` (`👁️viewer/🦀️.rs:70-82`).
- Manifest: `Viewer::builder(EN1997_DIALECT)`, icon `check-circle`, mode `view` (single full-pane window, `🎭️modes/👁️view/🦀️.rs:15-18`).
- Single window kind `report` (`TableWindowKit::KIND_ID`), rendered by `report::render` (`🪟️windows/📊️report/🦀️.rs:30-33`): recomputes `evaluate(document)` and renders a table with columns from `report_table_columns(locale)` and rows from `report_table_rows(report, locale)` (app-surface `🏷️ids-chrome-and-labels/🦀️.rs:49-70`).
- So the viewer shows exactly the same checks as the editor's Results list, as a flat table (one row per check). No diagrams, no 3D, no remedies.
- Presence and config: `NoPresence`, `NoConfig`.

---

## 6. Tests and oracles

### 6.1 Rust tests inside the artifact

Counts are `#[test]` or `async_test` attributes per file.

- **Compliance laws** (`SUB/🧬️schema/🧪️tests/⚖️compliance/`):
  - `🧮️design-approach-and-resistance-laws` (18): `annex_d_n_factors_phi30`, `da2_star_de_gamma_r_v_diverges_from_en`, `da2_star_uses_characteristic_soil_and_resistance_factors`, `da1_str_en_uses_en_material_factors`, `bs_p_vs_bs_t_changes_bearing_utilization`, `passive_earth_pressure_can_change_sliding_pass_fail`, `settlement_oedometric_has_no_hardcoded_influence`, `pile_shaft_worked_example`, `pile_correlation_n2`, `investigation_depth_rule`, `compliant_demo_passes`, `noncompliant_demo_has_failures_with_remedies`, `remedy_law_footing_width_clears_bearing_or_improves`, `remedy_law_investigation_depth_clears`, `remedy_law_pile_length_clears_compression`, `settlement_remedy_targets_governing_layer_id_path`, `field_meta_enum_choices_are_localized`, `empty_collections_are_not_applicable`.
  - `🏷️field-meta-and-perturbation-laws` (18): `default_snapshot_editable_leaves_have_meta`, `every_emitted_path_resolves_via_get_value_at_path`, `bishop_slices_uses_slope_length_and_gwl`, `undrained_cu_bearing_and_sliding_can_fail_with_remedies`, `poisson_ratio_drives_elastic_settlement_companion`, `governing_layer_id_missing_fails_bishop`, `pile_type_changes_design_compression_resistance`, `earth_pressure_invalid_mode_fails_and_movement_inconsistency_fails`, `wall_concrete_gamma_changes_sliding_vertical_action`, `phi_investigation_correlation_numeric`, `phi_stated_above_investigation_fails_and_remedy_passes`, `governing_design_situation_changes_with_load_case_situation`, `typed_facets_have_no_unknown_and_match_rust_fields`, `perturb_every_editable_leaf_changes_some_check`, `duplicate_layer_id_fails_integrity`, `duplicate_footing_id_fails_integrity` (plus helpers).
  - `🔮️oracle-and-remedy-laws` (5): `python_oracle_matches_check_project_within_half_percent` (spawns `python3 SUB/🔮️oracles/⚖️compliance/🐍️.py --json`, compares each check id's utilisation within 0.5 %), `example_snapshot_validates_against_json_schema` (python3 + `jsonschema`, see below), `groundwater_level_changes_bearing_utilization`, `base_inclination_changes_bearing_utilization`, `rankine_vs_at_rest_changes_wall_earth_pressure`.
  - Root `🦀️.rs` (2): `compliant_dsl_asset_complies`, `noncompliant_dsl_asset_has_multiple_fails`.
- **Inferences**: `🔬️compliance-report` (2): `evaluate_default_has_checks`, `evaluate_noncompliant_fails`. `🔬️unit` (2) and `🧾outline/🔬️unit` (2) exist.
- **Mutations**: `🔬️unit` (5), `🔬️kinds-catalog` (1: kinds match the committed oracle manifest), `🔬️vectors` (1). `🔺️diff/🔬️unit` (4).
- **Snapshot**: `🔬️regen-assets` (1, regenerates the committed `.dsl.semio` assets).
- **IO**: `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (15 tests): exact-word oracle for all 55 fields via `bun -e` script (JavaScript, third-party runtime `bun`), SQLite round trip, IEEE-word survival, limits and cancellation. Other IO test files: binary (3), text (6).
- **Editor** (31 tests across unit suites: root 14, edit mode 2, inputs 2, results 2, artifact panel 2, catalogue 4, inspection 3, evaluate command 1, selected-check command 1; plus the demo-session example 1). **Viewer** (6 tests: view mode 1, report window 2, viewer root 3).
- **Examples**: `demo` (3 tests), `demo-session` (1).

### 6.2 Python oracles

1. **Evaluation oracle** `SUB/🔮️oracles/⚖️compliance/🐍️.py` (492 lines). Stdlib only (`json`, `math`, `sys`, `typing`). No third-party package, and the manifest records `package: ""`.
   - `evaluate(doc)` (lines 367-470) recomputes, independently of Rust: per footing and load case, `en1997.6.5.bearing.*` and `en1997.6.5.3.sliding.*` (drained bearing with the Annex D factors, sliding without passive); per footing `en1997.6.6.settlement.*` (oedometric only); per pile `en1997.7.6.2.compression.*` (compression only); per slope `en1997.11.bishop.*` (simplified Bishop with 16 slices, reimplementation of `bishop_fos_slices`).
   - `resolve_params` (64-100) reimplements the DA2*/GEO-3/DA1 partial factors. Its DA1-C1 DE uses γ_c 1.4 and the EN branch 1.25; its DA1-C2 uses 1.0 material factors (same as the Rust side).
   - Self-check (`self_check`, 473-477): `N_q(30)≈18.401`, `N_c(30)≈30.14`, `N_γ(30)≈20.093`, `shaft_kn(0.7, 0.6, 80, 12)≈1266.69`.
   - Also the Rust test compares utilisations, not the full report.
   - **Not covered by the oracle**: walls (sliding, overturning, DIN 4085), uplift and HYD, undrained bearing and sliding, eccentricity, investigation depth, layer strength, CPT/SPT correlations, GK1 presumptive bearing, elastic settlement, tension piles, duplicate-id integrity, and all remedies. The oracle also takes `designSituation` only from the document (not per load case) for piles, and assumes GWL = 1e9 when absent, while Rust assumes 0.
2. **Mutation oracle**: `SUB/🧪️tests/🌍️mutate-en1997-1/🐍️.py` (100 lines, stdlib `importlib` only), a thin adapter that loads the shared norm engine `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py` (outside this artifact) via `importlib.import_module`. Its manifest is `SUB/🔮️oracles/🔣️.json` (oracle id `en1997-1-python-independent`, kind `verified-native-second-implementation`, `testOnly: true`, `networkDuringExecution: false`, `engine.family: "none"`). The manifest records that PyPI was searched for `structuralcodes`, `concreteproperties`, `anastruct` and rejected as non-interchange code-formula libraries.
   - The Gherkin feature `SUB/🧪️tests/🌍️mutate-en1997-1/🥒️.feature` specifies: 20 `apply` outline rows against committed vectors, 20 `inverse` rows, 3 `dupe` refusals (`mutation.duplicate-id`, `mutation.target-missing`, `mutation.no-op`), and one `identity-round-trip` (carrier re-emission). Rust side: `SUB/🧪️tests/🌍️mutate-en1997-1/🦀️.rs` (441 lines, `semio_repo_test_host::Adapter`).
   - Fixtures: `SUB/🧪️tests/🧫️fixtures/🧬️mutations/` has 20 mutation directories each with `✅apply/` (before, after, diff, mutation payload, outcome), plus 3 `⛔dupe/` directories (insert-footing, insert-layer, insert-pile, and remove-footing/remove-layer/remove-pile have `🧭️middle/` variants).

### 6.3 Third-party libraries used in tests

- Python `jsonschema` (test only, `oracle-and-remedy-laws` lines 62-68): validates the example snapshot against the JSON Schema leaf. Note that the test writes the instance into `std::env::temp_dir()`.
- Python 3 standard library (oracles).
- `bun` (JavaScript runtime) in the SQLite exact-word oracle, `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:84`.
- `serde_json` (test-only, `#[cfg_attr(test, …)]`) in Rust tests.
- No runtime third-party library in the artifact's own code.

---

## 7. Examples and fixtures

- Examples (`SUB/📚️examples/`):
  - `compliant` (`🏗️compliant/`): Compliant project, text from `🖼️assets/🏗️compliant/🗣️.dsl.semio`.
  - `demo` (`🎬️demo/`): "Compliant geotechnical package" (default subject). Its `🗣️.dsl.semio` and `📦️.pack.semio` are byte-identical to `compliant`.
  - `noncompliant` (`🚨️noncompliant/`): Non-compliant project (`🚨noncompliant/🗣️.dsl.semio`, 1970 bytes).
  - `demo-session` (under `SUB/✏️editor/📚️examples/🎬️demo-session/`): a 37-byte command script `🎮️.cmd.semio`.
- Assets (`SUB/🖼️assets/`): `.dsl.semio` (text carrier) and `.pack.semio` (binary) per example.
- Fixtures: `SUB/🧪️tests/🧫️fixtures/🧬️mutations/**` (committed before/after/diff/mutation/outcome for each kind), `SUB/🧬️schema/🧬️mutations/**/🧪️tests/` (`🔣️.cases.json` for remove-footing, remove-layer, remove-pile), and `SUB/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json` (SQLite oracle fixture).
- Snapshot fixtures for the Python oracle come from `compliant_demo()` and `noncompliant_demo()` via `.dsl.semio` assets (the json schema test reads `🔣️.json`).

---

## 8. Gaps, stubs, TODOs and ad-hoc shortcuts

No `TODO`, `FIXME`, `XXX`, `todo!` or `unimplemented!` markers exist in the artifact's source. The following are the substantive gaps, ordered by impact.

### 8.1 Likely calculation errors (verify against the standard text)

1. **Coulomb K_a is not the Coulomb formula.** `ka_coulomb` (`📐️geotechnical-parts/🦀️.rs:400-406`) computes `cosφ/(cosφ + √(sinφ·sin(φ−δ)/cosδ))²`. Computed: φ=30°, δ=20° gives 0.633, textbook Coulomb 0.297; φ=30°, δ=0 gives 0.464, Rankine 0.333. The result feeds active thrust `E_a` for any wall with δ ≠ 0 (the demo wall uses δ=20°), so E_a and overturning/sliding utilisation are overstated.
2. **K_p is taken as 1/K_a** (`kp_coulomb`, line 410). This is not the Coulomb passive coefficient. It feeds passive resistance for walls and footings.
3. **Bearing shape factor s_c uses N_c in the denominator** (`shape_factors`, line 43: `(s_q·N_q − 1)/N_c`). The standard form (EN 1997-1 Annex D, Vesic/Hansen) uses `(N_q − 1)`. Computed for φ=30°, B/L=1: code 0.883, standard 1.529. The code therefore understates the bearing capacity of square and rectangular footings.
4. **Design-approach material factors for DA1-C1 and DA1-C2** (`annex_params`, lines 67-79). Per my reading of EN 1997-1 Tables A.4/A.5, Combination 1 uses M1 (material factors 1.0) and Combination 2 uses M2 (γ_φ and γ_c 1.25). The code uses γ_φ 1.25 (and γ_c 1.25 EN, 1.4 DE) in DA1-C1 and 1.0 in DA1-C2, which looks inverted. Treat as a probable error to verify against the standard text, not a confirmed one.
5. **Overturning (`en1997.9.overturning`)** uses arms `H/3` and `B/2` (lines 344-345). It ignores the earth-pressure moment, base shear eccentricity, and passive/resistance moments. Not an EN 1997-1 §9 verification.
6. **DIN 4085 "check"** (`en1997.9.earthPressure`, lines 296-298) returns `(K₀, 1.0)` on success and `(2.0, 1.0)` on regime failure. It does not compute a DIN 4085 quantity, so the utilisation has no physical meaning.
7. **Governing-situation rows cannot fail** (`push_governing_situation_summary`, `layer-and-duplicate-id-helpers` lines 211-214). The limit is `util.max(1.0)`, so `util ≤ limit` always holds. These rows still feed the governing-check selection (`outline`).
8. **Elastic settlement** (`elastic_settlement_m`, part_1 289-300) is a 2:1 layered approximation, not EN 1997-1 Annex F.2 (which uses ν and E with a shape/depth influence factor). The cited clause `F.2` is therefore misleading.
9. **Eccentricity** uses **unfactored** characteristic actions (project-checks 475-478) and DIN 1054 7.5 limits. The EN 1997-1 6.5.4 form uses design actions in the relevant combination.
10. **Pile correlation factors** (part_1 239-249): n ≥ 5 uses a flat (1.30, 1.15) instead of the EN table values, and n = 3, 4 are linear interpolations. Verify against the EN 1997-1 Table A.10 values.
11. **Overall (Bishop) stability** is labelled "Bishop slices / GEO-3" and "EN 1997-1 11.5", but it uses a 5×5×5 grid search, a 16-slice approximation, and falls back to a "Legacy closed-form Bishop" (part_1 532-541) that does not use slices.
12. **Undrained bearing** is cited as `EN 1997-1 6 D.3` for the Annex D.3 formula and as `6.5.2` in other rows; the clause labels are inconsistent. The formula `(π+2)·c_u·s_c·i_c + q` with `s_c = 1+0.2·B/L` is an approximation of Annex D.3 and should be checked against the standard.
13. **Settlement** uses unfactored characteristic loads over the SLS (V_G+V_Q) without quasi-permanent combination factors. This is acceptable for a simplified check but is not the EN 6.6 procedure.

### 8.2 Silent defaults and fallbacks

- Unknown `designSituation` → Persistent (`parse_design_situation`, line 106). Unknown `designApproach` → DA1-C1 (`parse_design_approach`, line 213). No diagnostic is raised.
- Missing layers → an invented "default" layer (γ 18 kN/m³, γ′ 10 kN/m³, φ′ 30°, E_oed 30 MPa, `project-checks` 270-274). `layer_props_at` (part_1 413-419) falls back to φ′ 30°, γ 18 kN/m³, γ′ 10 kN/m³.
- Missing or unknown `governingLayerId` → fail with remedy (slope checks); no silent fallback.
- Pore pressure ≤ 1 Pa → `9810·GWL` (uplift, line 480).
- Settlement with no contributing layer → `q/E·B·0.5` (part_1 216-223).
- `soilType` is free text; cohesive detection uses substring matches "clay", "ton", "cohesive", and `c_u > 1 Pa` (part_1 353-359).
- Concrete γ clamped to ≥1000 N/m³ (pile-wall 196, 214); water unit weight 9810 N/m³ is repeated as a literal in at least 8 places (part_1 193, 472; pile-wall 195, 212, 227, 480).

### 8.3 Hard-coded tuning constants

- Remedy suggestion factors: angle × 0.85 (min 10°), height × 0.9 (min 1 m), GWL × 0.7, ν × 0.85, height × √(R/H)·0.95 (min 0.5 m), width × 1.05/1.02, width ≥ 1.2·B, eccentricity stress scaling (project-checks 407, 445, 468, 547, 583).
- Bishop search: grid `{0.25…0.85}` and `{0.8…2.2}`, radii combinations, 16 slices, 40 iterations, damping 0.5, convergence 1e-4, FoS clamp (0.05, 50), β clamp (0.05, 1.45) rad (part_1 436-530).
- Width search: multiplicative 1.08 step, 80 iterations, cap 80 m (layer-helpers 17-35).
- CPT/SPT correlations are clamped (CPT φ′ ∈ [28°, 42°], SPT φ′ ∈ [25°, 42°]; part_2 561, 565) with constants 27/10/0.5 (CPT) and 27.1/0.3/0.00054 (SPT). The "cautious" wording is not backed by a reference.
- Settlement factor clamps: buoyancy `0.4..1.0`, elastic influence `0.5..1.5` and `0.2..1.0`, stress spread `0.05..1.0`, layer weight `0.02..1.0`, Poisson ratio clamped `0..0.49` (part_1 206, 293, 316-318; project-checks 563).
- Investigation depth multipliers 2/3/3.5 × max(B, L) (part_2 567-574).
- Earth pressure floors: K_a ≥ 0.05, K_a (Coulomb) ≥ 0.1, K_p ≤ 20, K₀ floor 0.15 (part_1 362-411).
- Layer strength limits: `c_u ≥ 0.15·σ′v0` and `φ′ + c′/1000 ≥ 20` (project-checks 112-116). These are not EN clauses.
- Eccentricity limits B/6 and B/3 are DIN 1054 values hard-coded on the Rust side.
- Passive resistance uses a 1.0 m strip for walls (pile-wall 202, 228) and a 0.5 factor on E_p (project-checks 402-410).
- Wall base width scaled by 1.05 in remedies.

### 8.4 Unimplemented or stub behaviour

- `evaluate` command is a deliberate no-op (`🧮️evaluate/🦀️.rs:29-33`).
- `setSelectedCheckIndex` is view-only and emits no document mutation.
- `import_media` `model:in` is accepted and ignored (app-surface media, lines 130-135). `let _base = doc.snapshot.clone();` (editor root line 184) is dead code.
- `check_project` computes `_p_project` and `_upl_p` and never uses them (project-checks 10-11).
- `settlement_m` ignores `_poisson` (part_1 227-229).
- Unused helpers (0 call sites in the artifact): `design_bearing_resistance_n` (part_1 123-128), `pile_design_resistance_n` (256-258), `kp_rankine` (368-371). `sliding_resistance_n` (153-155) and `bishop_fos` (533-541) are only referenced by tests.
- The "Legacy" comment on `render_document_json` (app-surface) and the bishop fallback are legacy markers in a greenfield repo that must not carry legacy support.

### 8.5 Coverage gaps (vs the brief's norm-completeness goal)

Not implemented anywhere in the artifact:
- Anchors (ground anchors, tie-backs), pile groups, negative skin friction, lateral pile resistance, pile settlement, pile-group efficiency.
- Retaining walls with props or anchors, toe/heel sliding by layers, GEO-2/GEO-3 split for walls (only one `earthPressureMode` per wall).
- Spread footing overturning or tilt, structural (STR) checks of foundations.
- EN 1997-1 §9 earth-pressure envelopes, §10 HYD beyond simple critical gradient, §11 slope with seismic or water-level changes.
- Combination tables (DA1-C1, DA1-C2, DA2, DA3 combinations are implied by `designSituation` only).
- Ground investigation (EN 1997-2): no lab test model (triaxial, oedometer curves), no sample quality, no Part 2 reporting. Only derived φ′ from CPT and SPT and a minimum-depth rule.
- Seismic/accidental actions beyond the BS-A factor set.
- NA-DE for UPL and HYD outside the Table values hard-coded in code.
- Visual outputs: no 3D members, sections, foundations, load paths, utilisation heat maps, failure surfaces, M-N or stress diagrams. The Bishop circle is computed but not exposed.

### 8.6 Editor and UI gaps

- Only 14 paths are settable through typed mutations, 6 insert/remove kinds (section 1.5). Remedies outside these paths are "Not auto-applicable".
- Remedy targets such as `retainingWalls[].height`, `retainingWalls[].embedment`, `layers[].cohesionEffective`, `upliftCases[].*`, `piles[].tensionVariable`, `footings[].loadCases[].horizontalVariable` fall outside the edit rules.
- Results list is flat; no grouping by footing or governing check beyond the part header.
- Inputs window: no inline validation of unit ranges (negative widths, zero lengths), no link from an input to the check that depends on it, no live utilisation next to inputs.
- Viewer: one flat table; no remedies, no drill-down.
- Units: SI base units only, no unit switch.
- Language: EN/DE labels are present throughout, but some labels are composed in code (for example `format!("Layer {}")`), not from the translation tables.

### 8.7 Hygiene

- `SUB/✏️editor/🦀️.rs:1-7` header comment is mojibake (`â`, `ð`, `ï¸`), a file-encoding corruption.
- `SUB/✏️editor/🎚️config/…` and `presence` leaves are placeholders with `.empty.md` markers, which are expected for empty facets.
- `jsonschema` test writes to `std::env::temp_dir()` without cleanup.
- `#[allow(clippy::too_many_arguments)]` is used on three functions in project-checks (299, 508, 597) because the functions were split from one monolith. This is structural debt, not a bug.
- The Python oracle has no wall path at all, so `ka_coulomb`, `kp_coulomb` and `earth_pressure_coefficient` have no independent check. The only Rust test touching them is `rankine_vs_at_rest_changes_wall_earth_pressure`, which does not validate the Coulomb form.

---

## 9. Priority list (for the next sprint, derived from this inventory)

1. Replace `ka_coulomb` and `kp_coulomb` with the standard forms, add a unit test that checks Rankine at δ=0 and Coulomb values at δ≠0 (use the textbook pair: φ=30°, δ=20° → 0.297).
2. Fix `shape_factors` s_c to `(s_q·N_q − 1)/(N_q − 1)`; add a Python-oracle vector (φ=30°, B/L=1 → s_c=1.529).
3. Verify DA1-C1 and DA1-C2 material factors against EN 1997-1 Tables A.4/A.5 and DIN 1054 NA; align Rust, Python oracle and tests.
4. Make `push_governing_situation_summary` compare against a real limit (for example the governing check's limit) so the row can fail or be removed.
5. Replace the DIN 4085 sentinel with an actual regime-consistency check (mode/movement table), returning Pass/Fail without a fake utilisation.
6. Extend the Python oracle to walls, uplift/HYD, undrained checks, eccentricity and layers, so the 0.5 % parity test covers the whole `check_project` surface, not only bearing/sliding/settlement/compression/Bishop.
7. Add typed mutations for the remaining editable fields (layer γ and cohesions, pile diameter, wall height/embedment, uplift forces, load-case actions) so that every remedy target is auto-applicable, or drop the remedies.
8. Remove dead code (`_p_project`, `_upl_p`, `_base`, `design_bearing_resistance_n`, `pile_design_resistance_n`, `kp_rankine`, `settlement_m` poisson argument), and the legacy closed-form Bishop fallback.
9. Replace unknown-string fallbacks (`parse_design_situation`, `parse_design_approach`) with explicit diagnostics.
10. Fix the mojibake header in `SUB/✏️editor/🦀️.rs`.
</content>
