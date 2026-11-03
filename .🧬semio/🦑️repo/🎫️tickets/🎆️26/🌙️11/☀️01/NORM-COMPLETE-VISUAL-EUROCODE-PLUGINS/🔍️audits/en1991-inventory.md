# 🏋️ EN 1991 Inventory

Read-only audit of the EN 1991 artifact. No source file was modified.

**Path convention.** `S` is the subset root
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any`. Paths below are relative to `S` unless they start with `/`. Shared kernel paths are relative to `✏️s/🔌️plugins/📕️norm`. Line numbers were read from the working tree on 2026-11-01.

**Scope totals.** 1119 tracked files (excluding `.empty.md` placeholders). 29 Rust test files, 1 Gherkin feature (246 lines), 111 test functions, 80 mutation kinds, 1 Python oracle module, 1 Python mutation-test engine import, 4 example assets. The full file list is in `🗑️generated/en1991-files.txt` in the ticket folder.

---

## 1. Model

### 1.1 Snapshot model (`🧬️schema/📸️snapshot/🦀️.rs`, 489 lines)

`En1991Snapshot` (line ~18, `#[dsl(id = "norm.en1991", layout = "lines")]`, `#[artifact_schema(id = "s.norm.en1991")]`) has 70 public fields. Defaults are in `impl Default` follows the struct in the same file.

| Group | Fields (type, SI unit where declared) | Default |
|---|---|---|
| Annex / site | `annex: AnnexChoice` (`En`/`De`), `snow_zone: String`, `altitude: f64` (m), `en_sk: f64` (Pa), `north_german_lowland_snow: bool`, `wind_zone: u8`, `en_vb: f64` (m/s), `terrain_category: u8`, `mixed_terrain_upwind: u8`, `mixed_terrain_distance: f64` (m), `orography_factor: f64`, `coast_or_island: bool`, `air_density: f64` (kg/m³) | De, "2", 150, 850, false, 2, 25, 2, 2, 0, 1.0, false, 1.25 |
| Building | `height`, `width`, `depth` (m), `storey_count: u8` | 20, 15, 25, 3 |
| Thermal | `assumed_delta_t` (K), `t_max`, `t_min`, `t_0` (no unit, °C by field meta), `thermal_element_type: String`, `thermal_bridge_type: u8`, `delta_t_m` (K) | 40, 37, -24, 10, "building", 1, 0 |
| Fire | `fire_mode: FireMode`, `fire_curve: part_1_2::FireCurve`, `fire_duration` (s), `assumed_gas_temperature` (K), `assumed_h_net` (W/m²), `fire_compartment_area` (m²), `fire_compartment_height` (m), `fire_opening_factor`, `fire_thermal_inertia`, `fire_occupancy: String`, `fire_load_density_qf` (J/m²), `assumed_qf_d` (J/m²) | None, Standard, 3600, 1200, 25000, 100, 3, 0.04, 1160, "office", 420e6, 420e6 |
| Execution | `construction_activity: String`, `assumed_construction_qk` (Pa) | "scaffolding", 1500 |
| Bridge | `structure_kind: StructureKind`, `bridge_lane: u8`, `bridge_span` (m), `bridge_lane_width` (m), `assumed_bridge_tandem` (N), `assumed_bridge_udl` (Pa), `assumed_bridge_lm2` (N), `assumed_bridge_footway` (Pa), `assumed_bridge_lm3` (N), `assumed_bridge_lm4` (Pa), `bridge_load_group: String` | Building, 1, 20, 3, 0, 0, 0, 0, 0, 0, "gr1a" |
| Crane | `crane_claimed: bool`, `crane_class: String`, `hoist_class: String`, `hoisting_speed` (m/s), `assumed_crane_wheel` (N), `assumed_crane_horizontal` (N) | false, "HC2", "HC2", 0.5, 150000, 25000 |
| Silo | `silo_claimed: bool`, `silo_kind: String`, `silo_bulk_density` (N/m³), `silo_height` (m), `silo_hydraulic_radius` (m), `silo_mu`, `silo_k`, `assumed_silo_pressure` (Pa), `assumed_silo_patch` (Pa), `assumed_silo_wall_friction` (Pa) | false, "silo", 8000, 12, 1.5, 0.4, 0.4, 50000, 20000, 15000 |
| Collections (`#[dsl(table)]`) | `floors: Vec<FloorArea>`, `self_weight_elements: Vec<SelfWeightElement>`, `roofs: Vec<RoofArea>`, `wind_faces: Vec<WindFace>`, `accidental_cases: Vec<AccidentalCase>` | one each: office-l1, slab-rc, roof-main, facade-d; none for accidental |

Sub-entities (all `DslRecord`, `CanonicalJsonTree`, camelCase):

- `FloorArea { id, category, area (m²), assumed_qk (Pa), assumed_qk_concentrated (N), assumed_partitions (Pa) }`.
- `SelfWeightElement { id, material, thickness (m), assumed_gk (Pa) }`.
- `RoofArea { id, roof_type, pitch_deg, c_e, c_t, has_parapet, parapet_height (m), drift_obstruction_height (m), multi_span, assumed_sk (Pa) }`.
- `WindFace { id, zone, z (m), c_pe10, c_pe1, c_pi, c_s, c_d, loaded_area (m²), assumed_wp (Pa) }`.
- `AccidentalCase { id, impact: Vec<AccidentalImpact>, explosion: Vec<AccidentalExplosion> }`. The "discriminated" variant is two vectors that hold 0 or 1 entry each, not a Rust enum.
- `AccidentalImpact { vehicle_mass (kg), vehicle_speed (m/s), assumed_force (N) }`.
- `AccidentalExplosion { explosion_mass (kg), standoff (m), assumed_pressure (Pa) }`.

Enums:

- `FireMode { None, Nominal, Parametric }`.
- `StructureKind { Building, Bridge }`.
- `part_1_2::FireCurve { Standard, External, Hydrocarbon, Parametric }`.
- `AnnexChoice { En, De }`, defined in the compliance kernel (`⚖️compliance/📜️annex-and-tables/🦀️.rs:10`).

Constants: `EN1991_DIALECT` (artifact kind `s.norm.en1991`, standard `1`, subset `*`), `EN1991_DOCUMENT_SCHEMA = "semio.norm.en1991/v1"`. Facet leaves for the snapshot exist in five languages: `🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`.

### 1.2 Artifact schema (`🧬️schema/🦀️.rs`, 332 lines)

- `En1991Artifact` (line 21) has 70 `#[state(artifact)]` fields. They mirror the snapshot one-to-one, with no presence or draft lane. The doc comment on line 1 says "artifact and presence lanes", which is stale.
- `to_snapshot` and `from_snapshot` (lines 166-317) are field-by-field copies. They are fully hand-written, not generated.
- `en1991_artifact_schema_descriptor()` (line 322) registers the artifact, snapshot, diff, and mutation facets under id `s.norm.en1991`.

### 1.3 Inference (`🧬️schema/💡️inferences/🦀️.rs`, 547 lines)

- `En1991Inference { outline: En1991Outline }` (line 27) is the only inference. It is `#[derived]` and implements `protocol::Inference<En1991Snapshot>`. Its id is `s.norm.en1991.inference`, schema version 1.
- `🧾outline/🦀️.rs` (102 lines): `En1991Outline { section_outline: Vec<String>, field_count: u32, entry_count: u32 }`.
  - `section_outline` is a hard-coded list of 54 field names (`SECTION_FIELDS`, lines 10-65). It is not derived from the schema.
  - `entry_count` is the sum of the lengths of floors, roofs, wind faces, self-weight elements, and accidental cases.

### 1.4 Diff (`🧬️schema/🔺️diff/🦀️.rs`, 679 lines)

- `En1991Diff` (line 96) has 71 public `Option`-style patch fields, one per scalar plus collection-level operations. The mutation diffs fill only the field they touch.
- `DIFF_LEAVES` (line 678) registers the five facet leaves.
- Two unit tests in `🧪️tests/🔬️unit/🦀️.rs`: `change_floor_diff_patches_only_its_field` and `absorb_cancels_a_create_followed_by_its_delete`.

### 1.5 Mutations (`🧬️schema/🧬️mutations/🦀️.rs`, 300 lines)

- `En1991Mutation` (line 104) is a `dsl::Mutations` enum.
- `KINDS` (lines 187-276) lists 80 kinds. They are 70 `change-*` kinds (scalar setters with payload `new_<field>`, plus five index-addressed setters: `change-floor-assumed-qk`, `change-self-weight-assumed-gk`, `change-roof-assumed-sk`, `change-wind-face-assumed-wp`, `change-accidental-assumed-force`) and 10 insert/remove kinds for the five collections.
- Each kind is a folder `🧬️mutations/<emoji><slug>/` with `🦀️.rs`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🔣️.json`, and a `🧬️schema/` (2 files). The inverse is computed from the base.
- `apply_en1991_mutation` (line 286) runs `ColdMutation::diff` and then `apply_diff_cold`. It returns the new snapshot and diagnostics.
- Input validation is minimal. The diff rejects only out-of-range indices (`mutation.target-missing`) and emits a warning for no-op values (`mutation.no-op`). It never validates a value range. Examples: negative areas, a wind zone of 0, or an unknown snow zone string are accepted.

### 1.6 Compliance kernel types used by the artifact (`⚖️compliance`, shared)

- `CheckResult`, `CheckReport`, `CheckStatus { Pass, Warning, Fail, NotApplicable }` (`✅️check-results/🦀️.rs:9`).
- `CheckBuilder::minimum` (line 186) and `CheckBuilder::utilization` (line 174). `warn_above` (line 204) exists but no en1991 check calls it, so the Warning status is never produced by this artifact.
- `Remedy::at_least` and `Remedy::one_of`, `ClauseId::new(standard, clause, sub)`, `LocalizedCopy`, `Quantity`/`QuantityKind`.

---

## 2. Checks and verifications

All checks are emitted by `check_full_actions` (`💡️inferences/🦀️.rs:158`) and its helpers. The helpers themselves are in `💡️inferences/⚖️checks/🦀️.rs`. Check ids in the table below come from `CheckResult::assess` or `assess_covers`. Clause triples are the `ClauseId::new(standard, clause, section)` arguments.

### 2.0 Integrity (not a clause check)

- `push_duplicate_ids` (inferences line 81), called from `push_referential_integrity` (line 149) for floors, selfWeightElements, roofs, windFaces, and accidentalCases.
- Emits `en1991.integrity.duplicate.<table>.<id>`, status Fail, clause `EN 1991 §2 id`, with a `one_of` remedy.
- `push_dangling_ref` (line 119) is dead code (`#[allow(dead_code)]`). The file's own comment says "No cross-ref leaves on the subject today".

### 2.1 EN 1991-1-1 (densities, imposed loads, partitions)

Code: `⚖️checks/🦀️.rs` `pub mod part_1_1` (lines 125-222).

| Check id | Inferences line | Clause (standard, clause, section) | Assumed vs required | Formula / data | Helper location |
|---|---|---|---|---|---|
| `en1991.1-1.imposed.<floor>` | 181 | EN 1991-1-1, Table 6.1 / Eq. 6.1DE–6.2DE, 6.3.1 | `floor.assumed_qk` vs `q_k·α_A·α_n` | DE table A–K (DE NA Table 6.1DE, lines 156-164); EN table A–K (166-170); α_A = clamp(0.5 + 10/√A, 0.7, 1.0) for cat. C, D, E, I, J, K else 1.0 (190-199); α_n = clamp((2 + (n−2)·0.3)/n, 0.7, 1.0) for cat. A–D and n ≥ 2 else 1.0 (202-210) | 153-214 |
| `en1991.1-1.floor-force.<floor>` | 190 | EN 1991-1-1, §6.3.1 | `assumed_qk·A` vs `required·A` (total force) | Same as above, multiplied by floor area | 181-183 |
| `en1991.1-1.concentrated.<floor>` | 194 | EN 1991-1-1, Table 6.2, 6.3.1 | `assumed_qk_concentrated` vs Q_k (N) | DE: 4 kN (C, D), 7 kN (E, I, J, K, F), 4.5 kN (G), else 2 kN; EN: H = 1 kN, else 2 kN | 176-187 |
| `en1991.1-1.partitions.<floor>` | 198 | EN 1991-1-1, §6.3.1.2 | `assumed_partitions` vs allowance | DE 0.8 kN/m²; EN 1.0 kN/m² | 216-221 |
| `en1991.1-1.self-weight.<element>` | 208 | EN 1991-1-1, Annex A, 5.1 | `assumed_gk` vs γ·t | γ table (kN/m³) in lines 129-149; default 20 for unknown material | 150 |
| `en1991.1-1.imposed.none` | 173 | EN 1991-1-1, §6.3 | not applicable | when no floors | |
| `en1991.1-1.self-weight.none` | 202 | EN 1991-1-1, Annex A, 5.1 | not applicable | when no elements | |

Notes. `alpha_a` has identical match arms for DE and EN (lines 195-198). The concentrated-load table has a fallback of 2 kN for every category not listed. The Python oracle implements the same α_A and α_n formulas, so it confirms transcription, not normative correctness.

### 2.2 EN 1991-1-2 (fire)

Code: `pub mod part_1_2` (`⚖️checks/🦀️.rs` lines 414-549).

Branch by `fire_mode` (inferences lines 211-256):

- **None** (line 213): `en1991.1-2.fire.none`, not applicable.
- **Nominal** (lines 215-230):
  - `en1991.1-2.gas-temperature` (line 220), clause `EN 1991-1-2 §3.2 / 3.2`. θ_g from `nominal_gas_temp_k` (checks lines 419-440). Compared against `assumed_gas_temperature`.
  - `en1991.1-2.h-net` (line 225), clause `Eq. (3.1)/(3.2)`. `h_net_w_m2` (checks 484-490): h = α_c(θ_g−θ_m) + Φ·ε_m·ε_f·σ(θ_g⁴−θ_m⁴). Hard-coded Φ = 1.0, ε_m = 0.8, ε_f = 1.0, θ_m = 293.15 K, σ = 5.67e-8. α_c = 25 (50 for hydrocarbon, checks 543-548).
  - `en1991.1-2.fire-load` (line 229), clause `Annex E / NA, E.1`. q_f,d = q_f,k·m·δq1·δq2·δn (checks 494-498) with m clamped to [0.5, 1.0] but passed as the constant 0.8. DE δ table (checks 503-512): dwelling δq2 = 0.8, industrial δq1 = δq2 = 1.2, parking δq2 = 0.8, and others. The check compares `assumed_qf_d` with the computed q_f,d.
- **Parametric** (lines 231-255):
  - `en1991.1-2.opening-factor` (line 238), clause `Annex A, A.2`. Geometric O = 0.2·(4√A_f·H)·√H / (2A_f + 4√A_f·H) (checks 453-460). Compared with the input `fire_opening_factor` (if the input is 0, geometry is used). Remedy when the difference is greater than 0.005 (line 235).
  - `en1991.1-2.gas-temperature` (line 246), clause `Annex A, A.2`. θ_g = 1325(1−0.324e^(−0.2t*)−0.204e^(−1.7t*)−0.472e^(−19t*)) with t* = t·Γ and Γ = (O/b)²/(0.04/1160)² (checks 443-450). Required = max(θ_g, θ_max) where θ_max uses t_max = 0.2e-3·q_t,d/O (hours) (checks 471-482). Only the heating phase is implemented; there is no cooling branch and no t_lim.
  - `en1991.1-2.h-net` (line 251), same formula, with `required_theta` and α_c = 25.
  - `en1991.1-2.fire-load` (line 254), `Annex E / NA, E.1`. Compares `assumed_qf_d` with q_f,d (the q_t,d value is computed but only used for θ_max).

Other fire functions: `external_gas_temp_c` (checks 423-425), `hydrocarbon_gas_temp_c` (checks 427-429), `standard_gas_temp_c` (checks 419-421), `design_fire_load_qt_d_j_m2` (checks 463-469, q_t,d = q_f,d·A_f/A_t). `characteristic_fire_load_j_m2` (checks 526-541, DE Annex E q_f,k table) is **never called** by a check. The fire load is the user's input.

### 2.3 EN 1991-1-3 (snow)

Code: `pub mod part_1_3` (checks 224-261). Inferences lines 259-268.

- `en1991.1-3.snow.<roof>` (line 267), clause `§5.2 / 5.2`. Required s = μ·C_e·C_t·s_k (`roof_snow_pa`, checks 260). The check compares `assumed_sk`.
- Ground load s_k. DE: `ground_snow_pa` (checks 226-240). Zone 1: 0.65 kN/m² up to 400 m, then 0.19 + 0.91((a+140)/760)². Zone 1a: ×1.25. Zone 2: 0.85 up to 285 m, then 0.25 + 1.91((a+140)/760)². Zone 2a: ×1.25. Zone 3: 1.10 up to 255 m, then 0.31 + 2.91((a+140)/760)². EN: the input `en_sk` is used directly (checks 242-244).
- Shape coefficient μ, `shape_mu` (checks 245-259): 0.8 for α ≤ 30°; 0.8(60−α)/30 for 30° < α < 60°; 0 otherwise. Duo-pitch with 15° ≤ α < 30° gives at least 0.8. Multi-span gives at least 1.6. With a parapet, μ ≥ min(2·max(h, 0.5), 2). Without a parapet but with drift height h > 0, μ ≥ max(min(2h, 2), 0.8). `exceptional_north` doubles μ and caps it at 2.0 (line 257). This is an ad hoc treatment of the NA "außergewöhnliche Schneelast" and is not a separate s_Ad load.
- `en1991.1-3.snow.none` (line 259), not applicable when no roofs.

### 2.4 EN 1991-1-4 (wind)

Code: `pub mod part_1_4` (checks 263-412). Inferences lines 276-348.

Per wind face (inferences 280-348):

- `en1991.1-4.cpe10.<face>` (line 300), clause `Tables 7.1–7.4, 7.2`. Compares the user's c_pe,10 with the tabulated value from `tabulated_cpe10` (checks 381-402). Tolerance ±0.05 (inferences line 287). Walls: A −1.2, B −0.8, C −0.5, D +0.8 for h/d ≥ 1 and +0.7 otherwise, E −0.7 (h/d ≥ 5), −0.5 (h/d ≥ 1), −0.3. Roofs F, G, H step at pitch 5° and 15°; I and J +0.2. These are step tables, not the interpolated Table 7.1 and Figure 7.5/7.6 values.
- `en1991.1-4.cpe1.<face>` (line 326), clause `Tables 7.1–7.4 / Fig. 7.2, 7.2`. c_pe,1 = clip(1.25·c_pe,10) (checks 404-407), with tolerance ±0.05.
- Both cpe checks emit a pseudo-utilization. Its numerator is `1 − |Δ|` and its denominator is 1.0 (inferences 306, 332). This is not a physical utilization.
- `en1991.1-4.wind.<face>` (line 347), clause `§5.2 / Fig. 7.2, 5.2`. Required w = q_p·|c_pe − c_pi|·c_s·c_d (`wind_pressure_pa`, checks 409-411), where c_pe is the area-reduced value (`area_cpe`, log interpolation between 1 m² and 10 m², checks 354-363).
  - **Sign defect:** the call passes `cpe_a.abs()` (inferences line 344). For suction faces (negative c_pe) the expression becomes |−|c_pe| − c_pi|. This underestimates the suction pressure. The formula should use the signed c_pe.
- Peak velocity pressure q_p (inferences 342) from `peak_velocity_pressure_pa` (checks 316-330):
  - DE NA B.3: `na_b3_qp_pa` table by zone (1–4), terrain (0–4), and height (10, 20, 50, 100 m) with linear interpolation (checks 283-315). Scaled by orography and by ρ/1.25.
  - EN: q_b·c_e(z)·c_o where c_e = (k_r·ln(z/z_0))² and k_r = 0.19(z_0/0.05)^0.07 (checks 276-282). The (1 + 7I_v) turbulence factor is absent. This should be verified against EN 1991-1-4 Eq. (4.8).
  - Mixed terrain: linear blend over 2000 m (checks 319-325).
  - Coast (DE): terrain forced to at most category I (checks 318, 321).
- Basic velocity: DE `v_b0` zone table 22.5 / 25.0 / 27.5 / 30.0 m/s (checks 118-121); EN from `en_vb` (checks 272-274).
- Reference height z_e and reference size e = min(b, 2h) (inferences 341; checks 332-351).
- `en1991.1-4.wind.none` (line 271), not applicable when no faces.
- `wall_zone_letter` (checks 365-378) is **dead code**.

### 2.5 EN 1991-1-5 (thermal)

Code: `pub mod part_1_5` (checks 551-603). Inferences 350-353.

- `en1991.1-5.thermal` (line 353), clause `§5 / §6, 5`. Required ΔT = max(|T_max − T_0|, |T_0 − T_min|) for building elements, plus ΔT_M. For bridges (element type starting with "bridge") it uses ΔT_N from the bridge type table (checks 574-590; DE 15/18/10/13 K by type and direction).
- DE defaults: T_max 37 °C, T_min −24 °C (checks 557-561). A provided value of 0 °C is treated as "not provided" (`abs() > 0.01`), so 0 °C can never be entered as an explicit value.
- Bridge type parsed from the element-type string (`contains('2')` / `contains('3')`, line 596). This is fragile.

### 2.6 EN 1991-1-6 (execution)

Code: `pub mod part_1_6` (checks 605-611). Inferences 355-358.

- `en1991.1-6.construction` (line 358), clause `§4.11, 4.11`. Required q_k by activity: storage 2, machinery/concreting 3, scaffolding 1, any other string 0.5 kN/m². Silent fallback.

### 2.7 EN 1991-1-7 (accidental)

Code: `pub mod part_1_7` (checks 613-623). Inferences 360-377.

- `en1991.1-7.accidental.none` (line 361), not applicable when no cases.
- `en1991.1-7.impact.<case>` (line 368), clause `Annex C, C`. Required F = 0.5·m·v²/δ with δ = 0.1 m hard-coded (checks 615-618). The δ value is not sourced from the standard.
- `en1991.1-7.explosion.<case>` (line 374), clause `Annex D, D`. Required p = 2m/d²·1000 Pa (checks 619-622). This is an ad hoc surrogate, not the scaled-distance procedure of Annex D.
- Only the first impact and the first explosion entry of each case are evaluated (`.first()`, inferences lines 364 and 370).

### 2.8 EN 1991-2 (bridge traffic)

Code: `pub mod part_2` (checks 625-710). Inferences 380-496. Applies only when `structure_kind == Bridge`; otherwise `en1991.2.bridge.none` (line 382).

- `en1991.2.notional-lanes` (line 391), clause `Table 4.1, 4.2.3`. `notional_lanes` (checks 691-703): w < 5.4 m → 1 lane of width w (the standard uses a 3 m notional lane and remaining area w−3; verify). 5.4 ≤ w < 6 → 2 lanes of w/2. w ≥ 6 → floor(w/3) lanes of 3 m with remaining width.
- `en1991.2.lm1-tandem` (line 398), clause `§4.3.2, 4.3.2`. Required Q_ak = α_Q·base with base 300/200/100 kN per lane (checks 629-632, α_Q = 0.9 for DE). Compared as moment: assumed tandem·L/4 versus α_Q·Q·L/4 (checks 670-672).
- `en1991.2.lm1-udl` (line 402), clause `Table 4.2, 4.3.2`. q_ak = α_q·9 kN/m² (lane 1) or α_q·2.5 kN/m² (lanes 2–3). α_q = 0.4 for DE (checks 645-656).
- `en1991.2.lm1-remaining` (line 405), clause `Table 4.2, 4.3.2`. Evaluated only when w_rem > 0.05 m. Required 2.5 kN/m²·α_q3. The check reuses the assumed UDL value (`assumed_bridge_udl`).
- `en1991.2.lm2` (line 410), clause `§4.3.3, 4.3.3`. Required 400 kN·0.9 (DE) = 360 kN (checks 659-662).
- `en1991.2.footway` (line 414), clause `§5.2, 5.2`. Required 5.0 kN/m² (checks 664-667).
- `en1991.2.lm3` (line 418), clause `§4.3.4 / NA, 4.3.4`. Required 600 kN per axle, one constant for both annexes (checks 674-678). This is not the LM3 vehicle definition.
- `en1991.2.lm4` (line 422), clause `§4.3.5, 4.3.5`. Required 5.0 kN/m² (checks 680-681).
- `en1991.2.group.selected` (line 482), clause `Table 4.4a, 4.5`. For each group in `["gr1a", "gr1b", "gr3", "gr4", "gr5"]` (gr2 is absent), computes demand = max of the ratios of required to assumed for the included constituents (checks 682-688 and inferences 424-458). Membership: gr1a = LM1 + footway; gr1b = LM2; gr3 = LM4 + footway; gr4 = LM4; gr5 = LM3. `load_group_includes_lm1("")` also returns true for an empty string.
  - The reported utilization is pseudo: `computed_g = demand + 1.0` when the selected group fails, `demand + 0.5` when it fails to govern, otherwise `demand` (inferences 480).

### 2.9 EN 1991-3 (cranes)

Code: `pub mod part_3` (checks 712-743). Inferences 498-509. Applies only when `crane_claimed`; otherwise `en1991.3.crane.none` (line 499).

- `en1991.3.crane-vertical` (line 504), clause `§2.3, 2.3`. Required = Q_c(crane_class)·max(φ_1, φ_2(hoist_class, v_h)) (checks 715-735). Q_c table HC1 50, HC2 100, HC3 160, HC4 250 kN, default 80 kN. φ_1 = 1.1 (line 730). φ_2 = φ_min + β·v_h with (HC1: 1.05, 0.17), (HC2: 1.10, 0.34), (HC3: 1.15, 0.51), default (1.20, 0.68). The code uses max(φ_1, φ_2), not a product; verify against EN 1991-3 §2.3.
- `en1991.3.crane-horizontal` (line 508), clause `§2.5, 2.5`. H = ξ·φ_5·Q_c with φ_5 = 1.5 and ξ = 0.15 (DE) or 0.10 (EN) (checks 737-742). The code uses the Q_c value; the comment says Q_r (static wheel load).

### 2.10 EN 1991-4 (silos and tanks)

Code: `pub mod part_4` (checks 745-770). Inferences 511-531. Applies only when `silo_claimed`; otherwise `en1991.4.silo.none` (line 512).

- `silo_kind == "tank"`: `en1991.4.tank` (line 517), clause `§7, 7`. Required p = γ·h (`tank_hydrostatic_pa`, line 767).
- Otherwise:
  - `en1991.4.silo-pressure` (line 522), clause `Eq. (5.1), 5.2`. Janssen p_h = (γ·a/(μK))·(1 − exp(−μK·z/a)) with a = hydraulic radius (`janssen_horizontal_pa`, checks 749-753).
  - `en1991.4.silo-patch` (line 526), clause `§5.2.1.2, 5.2.1.2`. Required p_patch = C_p·p_h with C_p = 0.4 (DE) or 0.25 (EN). The factor is keyed by annex, not by slenderness (checks 755-759).
  - `en1991.4.silo-wall-friction` (line 530), clause `Eq. (5.3), 5.2`. Required p_w = μ·p_h (checks 762-764).
- Only "tank" versus anything else is distinguished. There is no hopper, eccentric, discharge, or flow pressure.

### 2.11 Summary of check ids

Typical fully populated subject emits one `imposed`, one `floor-force`, one `concentrated`, one `partitions` per floor; one `self-weight` per element; one `snow` per roof; two `cpe` and one `wind` per face; `thermal`; `construction`; and one `impact` or `explosion` per case. Non-applicable rows are emitted as explicit `NotApplicable` entries. Each claimed part (2, 3, 4) adds its own set of checks listed in Sections 2.8-2.10.

---

## 3. NA-DE parameters actually used

All values are in code, not in a data file. Source line references are to `💡️inferences/⚖️checks/🦀️.rs` unless noted.

| Area | Parameter | Value(s) in code | Lines |
|---|---|---|---|
| Snow zones | Ground load s_k (kN/m²) | Zone 1 0.65 (≤400 m), Zone 2 0.85 (≤285 m), Zone 3 1.10 (≤255 m); Zones 1a and 2a ×1.25; altitude formula above | 226-240 |
| Snow | Exceptional north-German lowland | μ doubled, capped at 2.0 | 257 |
| Wind | Basic velocity v_b0 (m/s) | Zone 1 22.5, 2 25.0, 3 27.5, 4 30.0 | 118-121 |
| Wind | q_p table (DE NA B.3, kN/m²) | Zones 1–4 by terrain 0–4 and heights 10/20/50/100 m | 283-315 |
| Wind | Coast terrain | Forced to category I at most | 318, 321 |
| Wind | c_pe,10 tables | "DE NA unchanged for |c_pe|" (comment) | 380 |
| Thermal | T_max / T_min (°C) | 37 / −24 (EN 40 / −20) | 555-562 |
| Thermal | Bridge ΔT_N (K) | DE type 1: 15 exp / 18 con; type 2: 15 / 18; type 3: 10 / 13 | 574-590 |
| Imposed | Table 6.1DE q_k (kN/m²) | A 1.5, A2 3.0, A3 4.0, B 2.0, B2 3.0, B3 5.0, C 3.0, C2 4.0, C3–C5 5.0, D 4.0, D2 5.0, E 5.0, E2 7.5, F 2.0, G 5.0, H 0.75, I 10.0, J 5.0, K 5.0 | 156-164 |
| Imposed | Concentrated Q_k (kN) | C–D 4, E/I/J/K 7, F 7, G 4.5, else 2 | 179-184 |
| Imposed | α_A and α_n | Eqs. 6.1DE and 6.2DE (see Section 2.1) | 190-210 |
| Imposed | Partitions (kN/m²) | 0.8 | 218 |
| Self-weight | Densities (kN/m³) | Listed in lines 130-147 | 130-147 |
| Fire | Characteristic fire load q_f,k (MJ/m²) | Table exists (lines 526-541) but is **not used** by any check | 526-541 |
| Fire | Occupancy δ factors | DE: dwelling δq2 0.8; industrial δq1 1.2, δq2 1.2; hospital/school/hotel δq1 1.05; shopping δq1 1.1; parking δq2 0.8; other 1.0 | 503-512 |
| Fire | Combustion factor m | Constant 0.8 at both call sites (inferences 226, 240) | |
| Traffic | α_Q (LM1 tandem) | 0.9, all lanes | 635-640 |
| Traffic | α_q (LM1 UDL) | 0.4, all lanes (EN 1.0) | 645-650 |
| Traffic | LM2 | 0.9 × 400 kN = 360 kN | 659-662 |
| Traffic | LM3 | 600 kN for both annexes | 675-678 |
| Traffic | Footway, LM4 | 5.0 kN/m² for both annexes | 665, 681 |
| Cranes | ξ (horizontal) | DE 0.15, EN 0.10 | 739-740 |
| Silos | C_p (patch) | DE 0.4, EN 0.25 | 757 |

The Python oracle reads none of these parameters from a data file. Its tables are also hard-coded (`🔮️oracles/⚖️compliance/🐍️.py` lines 38-45).

---

## 4. Editor

Code: `✏️editor/🦀️.rs` (349 lines) plus the modules listed below.

### 4.1 Commands (`✏️editor/🎮️commands/`)

Declared in `app_commands!` at `✏️editor/🦀️.rs:65-72`:

| Command (camelCase action) | Wire keyword | Handler | Behaviour |
|---|---|---|---|
| `evaluate` | `evaluate` | `🧮️evaluate/🦀️.rs` | Returns `Emit::default()`. It changes nothing. The report is recomputed on each read. |
| `setSelectedCheckIndex` | `selected-check` | `☑️selected-check/🦀️.rs` | View-only. Maps to a window config mutation on the results window. |
| `setField` | `set-field` | `✏️set-field/🦀️.rs` | Arguments `path` and `value_json` (a JSON string). Path is flattened through `field_meta::flatten_editor_path`, then dispatched through `EDIT_RULES`. |
| `insertItem` | `insert-item` | `➕insert-item/🦀️.rs` | Arguments `path`, `index`, optional `value_json`. |
| `removeItem` | `remove-item` | `➖remove-item/🦀️.rs` | Arguments `path`, `index`. |
| `applyRemedy` | `apply-remedy` | `🩹apply-remedy/🦀️.rs` | Arguments `checkId`, `remedyIndex`. Uses `dispatch_apply_remedy` and the edit rules. |

Keybindings: `mod+z` for undo, `mod+shift+z` for redo. Actions are classified `InteractiveJobClassification::Migrated`.

### 4.2 Field meta (`✏️editor/🏷️field-meta/🦀️.rs`, 144 lines)

- `en1991_field_meta` maps a path to `NormFieldMeta { label_en, label_de, unit, choices }`. There are 113 `exact` entries.
- Units are shown as text (m, Pa, kN, N/m³, J/m², °C, K, W/m², m½, etc.).
- Choices (enums) are given for `annex`, `snowZone`, `windZone`, `terrainCategory`, `constructionActivity`, `structureKind`, `bridgeLane`, `craneClass`, `hoistClass`, `siloKind`, `fireMode`, `fireCurve`, `fireOccupancy`, `bridgeLoadGroup`, `thermalElementType`, `roofType`, `category`, `material`, and others.
- `flatten_editor_path` strips the group prefixes `site.`, `building.`, `thermal.`, `execution.`, `bridge.`, `crane.`, `silo.` back to flat snapshot keys (lines 13-18).

### 4.3 Panels (`✏️editor/📌️panels/`)

Three panel tabs are declared in `create_en1991_app` (`✏️editor/🦀️.rs:244-246`):

1. **Artefakt / Artifact** (`🗿️artifact/🦀️.rs`, workbench group). Renders one line via `render_summary`: verdict "complies / does not comply", counts of checks, failing, warnings, passing, worst utilization, and a per-part ✓/✗ list.
2. **Katalog / Catalogue** (`📚️catalogue/🦀️.rs`, 132 lines, workbench group). Four read-only tables: Table 6.1 DE imposed q_k (A1–K), Table 6.1 EN imposed q_k, ground snow s_k by DE zone, and DE NA B.3 q_p for terrain II. It also serves the examples picker.
3. **Inspektion / Inspection** (`🔍️inspection/🦀️.rs`, details group). Shows one check in full: id, part, clause, then the computed values, explanation, and remedies. The check is chosen by the results window's `selected_check_index`, defaulting to index 0.

### 4.4 Windows (`✏️editor/🎭️modes/✏️edit/🪟️windows/`)

Single mode `edit` (default), layout `row` at 42 / 58 (`🎭️modes/✏️edit/🦀️.rs:15-17`).

- **Inputs** (`norm-en1991-inputs`, label "Inputs / Eingaben", icon `download`, 42% width). Renders `render_document_editor` over `GroupedEditor` (`📥️inputs/🦀️.rs:31-48`), a grouped projection of the snapshot into sections: site, building, thermal, fire, execution, bridge, crane, silo, plus the five collections as lists. The renderer (`📝️document-editors/🦀️.rs:169`) builds a tree from the value tree:
  - Booleans become a select with values "true" and "false".
  - Numbers become a number input.
  - Strings with field-meta choices become a select. Other strings become a text input that commits on blur.
  - Each change dispatches `setField` with the path and the JSON-encoded value.
  - Labels, units, and choices come from field meta with English and German text.
  - It does not render raw JSON on this path. `render_document_json` exists in the shared surface but is not called by en1991.
- **Results** (`norm-en1991-results`, label "Results / Ergebnisse", icon `bar-chart-3`, 58% width). Renders `render_report` (`📊️results/🦀️.rs:23`) as a grouped list:
  - One row per part, with a verdict.
  - Under each part, one row per check. Rows are labelled from the check. Fail and Warning rows expand to show the explanation and each remedy.
  - An `atLeast` remedy shows "current → required" and an apply button (`applyRemedy`).
  - A `oneOf` remedy shows one "Choose" button per option (`setField`).
  - Rows for a pass show status only.
- No 3D viewport, no 2D chart, no curve, no heat map, no diagram. The grep for canvas, SVG, viewport, mesh, plot, and diagram found no matches in the Rust or TypeScript of this artifact.

### 4.5 Media ports

`export_media` (`📇️registry/.../🪟️manifest-and-media/🦀️.rs:108`):

- `report:out` produces the `CheckReport` as structured JSON, under the artifact kind schema.
- `artifact:out` produces the whole-document pack.
- Any other port returns `NotImplemented`.

`import_media`: `model:in` is accepted and ignored. `artifact:in` is refused.

### 4.6 Examples in the editor

`examples()` (`✏️editor/🦀️.rs:82-84`) lists only `de_office_compliant` and `multi_fail_noncompliant`. The demo session (`📚️examples/🎬️demo-session/`, action `demo`) is mounted but not in this list.

### 4.7 Editor-side gaps

- The Inputs TypeScript twin (`✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️.ts`) describes the window as "pretty-printed JSON" with a `documentJson` field. This is stale. The Rust renders a structured tree.
- Doc comments in `✏️editor/🦀️.rs` (lines 1-7, 48-56, 174-186, 233, 291-297) show mojibake (for example `ð®ï¸`, `â`). They should be re-encoded as UTF-8.

---

## 5. Viewer

Code: `👁️viewer/🦀️.rs` (128 lines), `👁️viewer/🎭️modes/👁️view/`.

- `En1991Viewer` implements `ArtifactViewer` only. It has no commands except `En1991ViewCommand::Noop`. `handle` returns an empty `ViewEmit`. The viewer has no mutations.
- Mode `view`: one full-pane window, layout `single_window_layout(report, "Report")` (`🎭️modes/👁️view/🦀️.rs`).
- Window `report` (`🪟️windows/📊️report/🦀️.rs`): a read-only `TableWindowKit` table. On each render it recomputes `evaluate(document)` and tables it.
  - Columns from `report_table_columns` (shared surface, `🏷️ids-chrome-and-labels/🦀️.rs:49`): Part, Clause, Subject, Status, Utilization, Title, Remedy (seven). Rows from `report_table_rows` (line 63) put the check part, clause, subject label, status, utilization with two decimals, title, and the first remedy's action text.
  - Rows: one per computed check.
- Stale TypeScript twin (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🟦️.ts`): declares row fields clause, status, utilization, message. The Rust table has seven columns and no `message` column.
- The viewer's `🧪️tests/🔬️unit/🦀️.rs` has three tests: viewer definition, dialect coordinate, unknown body fallback.

---

## 6. Tests and oracles

### 6.1 Rust tests (111 test functions in 29 files)

- **Compliance kernel tests** (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`, 11 tests): snow zone 2 at 150 m is 850 Pa; zone 1a is 1.25 × zone 1; DE versus EN snow; B1 imposed 2000 Pa; basic velocity pressure 390.625 Pa at 25 m/s and 1.25 kg/m³; NA B.3 positive; DE zone 2 v_b0 = 25; LM1 tandem DE 270 kN vs EN 300 kN; LM1 UDL DE 3.6 vs EN 9.0 kN/m²; crane horizontal DE > EN; silo patch DE > EN.
- **Compliance report tests** (`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`, 685 lines, 21 test functions). The main ones:
  - `default_subject_complies_de_na`, `de_office_compliant_example_passes`: asserts zero Fail rows.
  - `noncompliant_raises_fail_with_remedy`: asserts at least 3 fails and every fail has a remedy.
  - `remedy_law_*`: apply a remedy and re-evaluate. The assertion is `status != Fail || utilization <= 1.0`, which passes trivially for the utilization defect in Section 7.
  - `emitted_entity_paths_use_id_selectors_and_resolve`: paths resolve against the document.
  - `scope_aware_perturbation_of_editable_leaves`: perturbs each editable leaf.
  - `snapshot_json_validates_against_schema`: validates against the snapshot JSON schema using Python `jsonschema` (third-party, test-only).
  - `field_meta_coverage_all_committed_examples`: every path in the examples has field meta.
  - `duplicate_*_id_fails_integrity`: four tests.
  - `python_oracle_within_half_percent`: see 6.2.
  - `regenerate_example_assets_once`: rewrites the committed assets when `EN1991_REGEN_ASSETS=1`.
  - `oracle_manifest_kind_count_matches_mutation_enum`.
- **Inference tests**: determinism and default law (2 tests). Outline field count and determinism (2 tests).
- **Diff tests** (2 tests, see 1.4).
- **Mutations tests** (`🧬️schema/🧬️mutations/🧪️tests/`): `🔬️unit`, `🔬️kinds-catalog`, `🔬️vectors`.
- **Editor tests** (about 19 functions, including helpers, in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, plus per-node unit tests): retained-command dispositions against the language-neutral oracle, command round trips in text and binary, manifest stitch, ports declared, unknown body fallback, every declared body renders, `evaluate` commits a report, `selectedCheckIndex` is config-only.
- **Viewer tests** (3, see Section 5).
- **Examples** (`📚️examples/🧬️subjects/🧪️tests/🔬️unit/🦀️.rs`): `dump_example_dsl_and_pack_assets` writes the committed example assets.
- **Mutation differential test** (`🧪️tests/🏋️mutate-en1991-1/🦀️.rs`): the Rust subject. It applies each of the 80 kinds to the committed before-snapshot, asserts the after-snapshot, and asserts that the inverse restores the before-snapshot. Gated by the `sut` feature.

### 6.2 Python oracles

**Compliance oracle** `🔮️oracles/⚖️compliance/🐍️.py` (84 lines). Standard library only (`json`, `math`, `sys`, `pathlib`). No third-party library.

- Covers: imposed loads (DE table, partial: A, A1, A2, A3, B, B1, B2, C, C1, H; other categories default to 2 kN/m²), α_A, α_n, self-weight (three materials, others 20 kN/m³), ground snow, and μ (no roof-type, parapet, or drift handling).
- Invoked only by `python_oracle_within_half_percent` on `En1991Snapshot::default()`. Relative tolerance 0.5%.
- Limitations: it covers one default document only. It never covers the wind, fire, thermal, traffic, crane, or silo checks. Its formulas are a second transcription of the same reading as the Rust code, so agreement shows transcription, not normative correctness.
- The Rust test skips any id not found in the report (`if let Some(check)`), so a missing check passes silently.
- The test writes its temporary file to `std::env::temp_dir()`. Project rules direct temporary output into the ticket folder or the scratchpad.

**Oracle registry** `🔮️oracles/🔣️.json`: one oracle, `en1991-1-python-independent`, `ecosystem: python`, `package: ""`, `testOnly: true`, licence AGPL-3.0-only. It registers the 80-kind `en1991-1-any` catalog and a mutation manifest. The manifest's rationale states that no PyPI package serves `s.norm.en1991`.

**Mutation oracle** `🧪️tests/🏋️mutate-en1991-1/🐍️.py` (only lines 1-60 were read; the rest was not inspected). Imports `import_module("🐍️")`, which resolves to the shared norm vocabulary engine `semio_norm_vocabulary`. That engine lives outside this artifact and was not read here. The file reads the catalog, the vectors, and the schemas from this subset. It asserts that the Python and Rust implementations reach the same committed after-snapshot.

**Gherkin feature** `🧪️tests/🏋️mutate-en1991-1/🥒️.feature` (246 lines). A `Scenario Outline` with 80 example rows (one per kind), each applying the committed `before → mutation → after` triple. Tagged `@capability-en1991-1-mutate @oracle-en1991-1-python-independent @comparison-ordered-json-v1`.

### 6.3 Third-party libraries

- Python `jsonschema` (test-only, invoked as a subprocess from `python_oracle` and schema tests).
- Python 3 itself (test-only oracle runner).
- Rust `serde` and `serde_json` (test-only, under `cfg(test)`).
- No runtime external library in this artifact. No `[dev-dependencies]` section in `Cargo.toml`.

---

## 7. Gaps, stubs, TODOs, and hard-coded values

### 7.1 Correctness defects

1. **Standard fire curve formula is wrong.** `standard_gas_temp_c` (`⚖️checks/🦀️.rs:419-421`) uses `log10(1 + t/8)`. EN 1991-1-2 Eq. (3.4) is `20 + 345·log10(8t + 1)`. At t = 60 min the code gives 340.6 °C; the correct value is 945.3 °C. The Nominal branch therefore under-requires the gas temperature (`inferences 220`). No test covers this.
2. **External fire curve exponent.** `external_gas_temp_c` (line 424) uses `e^(−3.9t)`. EN 1991-1-2 Eq. (3.5) appears to use `e^(−3.8t)`. Verify against the standard.
3. **Utilization of failing minimum checks is inverted.** `CheckBuilder::minimum` (`⚖️compliance/✅️check-results/🦀️.rs:186-198`) sets utilization to `computed/minimum` when the check fails. Computed is less than minimum in that case, so the value is below 1 and reads as passing. `worst_utilization` (line 360) takes the maximum over all checks, so failing rows are excluded from it. The `remedy_law_*` tests accept `utilization <= 1.0` for Fail rows, so the defect is not caught.
4. **Wind suction sign.** `inferences/🦀️.rs:344` calls `wind_pressure_pa(qp, cpe_a.abs(), ...)`. For suction faces with negative c_pe, |c_pe − c_pi| is computed from the absolute value and underestimates the pressure.
5. **Notional lane for w < 5.4 m.** `notional_lanes` (`checks:693-694`) returns lane width = w and remaining = 0. EN 1991-2 Table 4.1 uses a 3 m notional lane with remaining width w − 3. Verify.
6. **EN exposure factor omits (1 + 7I_v).** `exposure_factor` (checks 276-282) and the peak pressure built on it. Verify against EN 1991-1-4 Eq. (4.8).
7. **Warning status is unreachable.** `warn_above` is never called by en1991. The Warning category is dead for this artifact.
8. **Only the first accidental impact and explosion are evaluated** (`inferences:364, 370`).
9. **Load group gr2 is never evaluated** (inferences line 424). `load_group_includes_lm1("")` accepts an empty group name.
10. **Zero is treated as "not provided"** for T_max and T_min (`checks:556, 560`). A genuine 0 °C reading is replaced by the DE default.
11. **Bridge type parsed from the element-type string** (`checks:596`): `contains('2')` and `contains('3')`. Any other string falls back to `bridge_type`.
12. **Crane vertical load** combines φ_1 and φ_2 by `max` (checks 734). Verify whether EN 1991-3 requires a product or a maximum.
13. **Crane horizontal force** is computed from Q_c (line 741) but its comment says Q_r.

### 7.2 Silent fallbacks (no diagnostic)

- `SnowZone::parse` (checks 111-115): any unknown zone becomes Zone 2.
- `WindZone::from_u8` (line 120): any value other than 1–3 becomes Zone 4. So wind zone 0 becomes Zone 4.
- `TerrainCategory::from_u8` (line 268): any value above 3 becomes category IV.
- `imposed_qk_pa` (lines 164, 170): unknown category becomes 2.0 kN/m².
- `self_weight_n_m3` (line 146): unknown material becomes 20 kN/m³.
- `construction_qk_pa` (line 608): unknown activity becomes 0.5 kN/m².
- `fire_delta_factors` (lines 512, 520): unknown occupancy gives δ = 1.
- `characteristic_fire_load_j_m2` (line 538): unknown occupancy gives 500 MJ/m².
- The mutation diff does not validate values (Section 1.5).

### 7.3 Hard-coded constants

- h_net: Φ = 1.0, ε_m = 0.8, ε_f = 1.0, θ_m = 293.15 K (inferences 222, 248).
- Combustion factor m = 0.8 (inferences 226, 240).
- Impact crushing distance δ = 0.1 m (checks 616).
- Explosion surrogate p = 2m/d²·1000 (checks 621).
- Silo patch C_p 0.4 (DE) / 0.25 (EN) keyed by annex (checks 757).
- Crane φ_5 = 1.5, φ_1 = 1.1 (checks 730, 739).
- Crane ξ 0.15 / 0.10 keyed by annex (checks 740).
- LM2 400 kN × 0.9; LM3 600 kN; LM4 5 kN/m²; footway 5 kN/m² (checks 659-681).
- Tolerances: c_pe ±0.05 (inferences 287-288); O ±0.005 (inferences 235); comparisons use `+ 1e-9`.
- Wall and roof c_pe tables are step functions with breakpoints at h/d = 1 and 5, and pitch = 5° and 15° (checks 385-402). They are not the interpolated tables.
- Wind: c_pe,1 = 1.25·c_pe,10 (checks 405).
- Thermal: all bridge ΔT_N values (checks 577-588).
- Snow: exceptional doubling (checks 257).
- Snapshot defaults include concrete values (for example the default office and roof) that are used both as the default document and as test subjects.

### 7.4 Stubs, dead code, and unused paths

- `characteristic_fire_load_j_m2` (checks 526-541): DE Annex E table, never called.
- `wall_zone_letter` (checks 365-378): never called.
- `push_dangling_ref` (inferences 118-146): `#[allow(dead_code)]`, never called.
- `render_document_json` in the shared surface: called by no en1991 code path.
- `PHI_1` is a constant used only inside `design_vertical_wheel_n`.
- `En1991Command::evaluate` is a no-op (`🧮️evaluate/🦀️.rs`).
- `En1991ViewCommand::Noop`: the viewer has no real command.
- `alpha_a`: DE and EN branches are identical (checks 195-198).
- Fire parametric: no cooling phase, no t_lim (checks 442-482).
- Crane runway fatigue, buffers, skew, and the LM3 vehicle definition are absent.
- Silo: no hopper, eccentric, discharge, or flow pressure. `silo_kind` only distinguishes "tank".
- Wind: no directional or seasonal factors, no internal pressure from openings, no force coefficients, no dynamic response (only c_s·c_d scalars). Roof wind uses the first roof's pitch for every face (`inferences:282`).
- Accidental: no Annex D scaled-distance procedure, no collision load with hard-coded 0.1 m.
- Snow: no drift, no sliding, no s_Ad.

### 7.5 Stale or inconsistent documentation

- `🧬️schema/🦀️.rs` line 1 says "artifact and presence lanes". There is no presence lane.
- `🧬️schema/📸️snapshot/🦀️.rs` line 1 says "complete design-load assumption subject". The fields do not cover all of EN 1991-1-3 through 1-7 (for example, no drift, no hopper, no crane-runway fatigue).
- Editor TS twin for Inputs says "pretty-printed JSON". The Rust renders a structured tree.
- Viewer TS twin for Report declares clause, status, utilization, message. The Rust table has seven columns (Part, Clause, Subject, Status, Utilization, Title, Remedy).
- Editor catalogue lists two examples. Five named subjects in `🧬️subjects` (bridge compliant and non-compliant, fire parametric compliant and non-compliant, accidental variants) are not in the navbar.
- Clause triples are inconsistent in their third element. For example, Table 6.1 uses section `6.3.1` (line 181), while thermal uses `5` (line 353).
- Mojibake in the doc comments of `✏️editor/🦀️.rs` and several `📇️registry` files.

### 7.6 Test-coverage gaps

- No numeric oracle for: fire curves, h_net, parametric fire, wind c_pe tables and q_p, bridge notional lanes, crane, silo (Janssen, patch, wall friction), accidental, thermal bridge ΔT_N.
- No test for the Warning status.
- The `remedy_law_*` tests cannot catch the utilization defect (7.1-3).
- The Python oracle covers only the default document and skips missing ids.
- Two tests write to shared locations (`std::env::temp_dir()` and committed assets through an environment gate).

### 7.7 Shortcuts in the editor

- Editing takes a JSON string (`value_json`) on the wire, not a typed value.
- `setField` paths use the editor's grouped form (`site.snowZone`) and are flattened back in `flatten_editor_path`. Any field not in that list bypasses grouping.
- The results window's selected check is a view-only config index with a fallback to 0.

---

## 8. Fixtures and examples

### 8.1 Example documents

- `📚️examples/🏢️de-office-compliant/` (DSL, pack, Rust, test). Office building, no bridge, no fire, crane HC2 claimed, silo claimed. Passes all checks.
- `📚️examples/⚠️multi-fail-noncompliant/` (DSL, pack, Rust, test). Carries one member in every collection, including the vehicle impact case. Used by the mutation test. Designed to fail several checks with remedies.
- `📚️examples/🧬️subjects/🦀️.rs` (lines 1-200+). Seven named subjects: `de_office_compliant`, `de_bridge_compliant`, `de_bridge_noncompliant`, `de_fire_parametric_compliant`, `de_fire_parametric_noncompliant`, `de_accidental_variants`, `multi_fail_noncompliant`.
- `✏️editor/📚️examples/🎬️demo-session/` contains `🖼️assets/🎮️.cmd.semio` (`semio norm.en1991.cmd v1`, `action=demo`). It is mounted but not in the catalogue list.

### 8.2 Assets

`🖼️assets/` holds four files: `🏢de-office-compliant/🗣️.dsl.semio` (2064 bytes), `📦️.pack.semio` (761 bytes); and `⚠️multi-fail-noncompliant/🗣️.dsl.semio` (2139 bytes), `📦️.pack.semio` (785 bytes). The `.pack` is the binary envelope and the `.dsl` is the text form. Both are regenerated by `dump_example_dsl_and_pack_assets`. Note that the DSL text has no published grammar: `📖️.grammar.semio` is a placeholder `payload = OCTET+` (per the mutation feature text).

### 8.3 Mutation fixtures

- `🧪️tests/🧫️fixtures/🧬️mutations/`: 80 directories, one per kind, containing about 400 files. Each vector holds `📸️snapshot/⬅️before`, `🦠️mutation`, `📸️snapshot/➡️after`, `🎯️outcome`, and `✅️apply` fixtures.
- `🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`: one leaf schema per kind (96 schema directories in total, including the insert/remove and inverse folders).

### 8.4 Storage codecs

- `🚪️io/📝️text/` (DSL text, JSON), `🚪️io/💾️binary/` (pack encode/decode), `🚪️io/🪶️sqlite/` (SQLite snapshot codec with `🗄️.sql` and `🗄️.d.ts`). The SQLite codec is only referenced by tests.

---

## Appendix: Files read

Key reads, all under `S` unless noted: `🧬️schema/🦀️.rs`, `🧬️schema/📸️snapshot/🦀️.rs`, `🧬️schema/💡️inferences/🦀️.rs`, `🧬️schema/💡️inferences/⚖️checks/🦀️.rs`, `🧬️schema/💡️inferences/🧾outline/🦀️.rs`, `🧬️schema/🔺️diff/🦀️.rs` (struct and tests), `🧬️schema/🧬️mutations/🦀️.rs`, `🧬️schema/🧬️mutations/🏢change-air-density/`, `🧬️schema/🧬️mutations/➕️insert-floors/`, `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`, `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`, `✏️editor/🦀️.rs`, `✏️editor/🎮️commands/*`, `✏️editor/🏷️field-meta/🦀️.rs`, `✏️editor/📌️panels/*`, `✏️editor/🎭️modes/✏️edit/*`, `👁️viewer/🦀️.rs`, `👁️viewer/🎭️modes/👁️view/*`, `🔮️oracles/⚖️compliance/🐍️.py`, `🔮️oracles/🔣️.json`, `🧪️tests/🏋️mutate-en1991-1/*`, `📚️examples/*`. Shared: `📇️registry/🧬️contract/🖥️app-surface/` (document-editors, report-tree, retained-commands, ids-chrome-and-labels, manifest-and-media) and `⚖️compliance/` (check-results, annex-and-tables).
