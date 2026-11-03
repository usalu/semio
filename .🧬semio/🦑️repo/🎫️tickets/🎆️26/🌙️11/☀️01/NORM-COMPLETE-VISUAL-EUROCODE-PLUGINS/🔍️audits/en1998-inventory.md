# 🫨 EN 1998 Inventory

Read-only audit of the EN 1998 artifact as it exists in the working tree. No source file was modified, no build or test was run, no repo tool was invoked. Statements marked **verify** compare code against EN 1998-1 / DIN NA values that are not present in this repository; they are flagged, not confirmed.

## 0. Scope and conventions

- Artifact root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998` (667 files: 431 outside fixtures and `Cargo.lock`, 235 fixture files, `Cargo.lock`). Rust source in the subset: 10,571 lines.
- Subset root (`ANY`): `🏅️standards/🔖️1/🪆️subsets/✳️any`. Paths below are relative to the artifact root unless they start with `ANY/`.
- Short names used in the location column:
  - `SNAP` = `ANY/🧬️schema/📸️snapshot/🦀️.rs` (snapshot model)
  - `SCH` = `ANY/🧬️schema/🦀️.rs` (artifact schema: `na_de`, `part_1`…`part_6`, `AnnexParams`)
  - `INF` = `ANY/🧬️schema/💡️inferences/`
  - `SH` = `INF/🧰️seismic-helpers/🦀️.rs`
  - `FSC` = `INF/🌍️full-seismic-check/🦀️.rs`
  - `BRS` = `INF/🌍️full-seismic-check/🏢️building-regularity-and-base-shear-checks/🦀️.rs`
  - `DBS` = `INF/🌍️full-seismic-check/🧱️structural-system-checks/🧭️direction-and-base-shear/🦀️.rs`
  - `SFD` = `INF/🌍️full-seismic-check/🧱️structural-system-checks/🏬️storey-forces-and-drift/🦀️.rs`
  - `DCM` = `INF/🌍️full-seismic-check/🧱️structural-system-checks/🔗️ductility-and-masonry/🦀️.rs`
  - `SYS` = `INF/🌍️full-seismic-check/🧱️structural-system-checks/🦀️.rs` (per-system loop)
  - `PE` = `INF/⚖️part-evaluators/🦀️.rs`
  - `EDR` = `ANY/✏️editor/🦀️.rs` (editor root)
- Paths beginning with `⚖️compliance/` or `📇️registry/` are relative to `✏️s/🔌️plugins/📕️norm`.
- Shared kernels outside the artifact that the artifact calls: `✏️s/🔌️plugins/📕️norm/⚖️compliance` (CheckBuilder, Remedy, Quantity, status logic), `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface` (generic editor, report, inspection and remedy rendering, set-field dispatch).

---

## 1. Model

### 1.1 Snapshot (`SNAP`, module `semio_s_artifact_norm_en1998_model`)

Root `En1998Snapshot` (L21–50), dialect `EN1998_DIALECT` (L385), artifact kind `s.norm.en1998`, document schema `semio.norm.en1998/v1`, DSL layout `lines`. `Default` = `compliant_de_office()`.

| Field | Type | Note |
|---|---|---|
| `annex` | String | `"de"` or `"en"`; any other value is treated as DE (see §8) |
| `site` | En1998Site | |
| `buildings` | Vec<En1998Building> | table |
| `bridges` | Vec<En1998Bridge> | table, EN 1998-2 |
| `assessments` | Vec<En1998Assessment> | table, EN 1998-3 |
| `silos` | Vec<En1998Silo> | table, EN 1998-4 |
| `tanks` | Vec<En1998Tank> | table, EN 1998-4 |
| `foundations` | Vec<En1998Foundation> | table, EN 1998-5 |
| `retaining_walls` | Vec<En1998RetainingWall> | table, EN 1998-5 (serialized `retainingWalls`) |
| `towers` | Vec<En1998Tower> | table, EN 1998-6 |

Entities (all `DslRecord`, `ToValue/FromValue`, camelCase on the wire):

| Struct (SNAP line) | Fields (type, unit where `#[dsl(unit)]`) |
|---|---|
| `En1998Site` (479–494) | `seismic_zone: DeSeismicZone`; `a_gr: f64` [m/s²] (EN reference PGA; ignored under DE); `de_ground_combo: DeGroundCombo`; `en_ground_type: String` (A–E, empty under DE); `en_spectrum_type: String` (`type1`/`type2`, empty under DE); `importance_class: String` |
| `En1998System` (502–513) | `id`, `direction` (String, x/y by convention), `system_type` (frame/wall/dual/masonry), `material` (rc/steel/composite/timber/masonry), `ductility_class` (dcl/dcm/dch), `q0: f64`, `alpha_u_over_alpha_1: f64`, `k_w: f64`, `base_shear_resistance_n: f64` [N] |
| `En1998VariableAction` (521–527) | `id`, `category` (A–H, snow, wind), `qk_n: f64` [N] |
| `En1998Storey` (535–563) | `id`, `height_m` [m], `permanent_gk_n` [N], `correlated_occupancy: bool`, `variables: Vec<En1998VariableAction>`, `stiffness_x`, `stiffness_y` (no unit attribute), `centre_of_mass_x_m`, `centre_of_mass_y_m`, `centre_of_stiffness_x_m`, `centre_of_stiffness_y_m` [m], `drift_x_m`, `drift_y_m` [m], `shear_resistance_n` [N] |
| `En1998Member` (571–587) | `id`, `material`, `role` (column/beam/wall), `detailing_compatible_with_q: bool` (input flag, not computed), `min_dimension_m` [m], `rho`, `rho_prime`, `omega_wd`, `steel_section_class: u8` |
| `En1998Building` (595–621) | `id`, `name`, `plan_width_m` (Lx), `plan_length_m` (Ly) [m], `systems: Vec<En1998System>`, `storeys: Vec<En1998Storey>`, `members: Vec<En1998Member>`, `plan_regular: bool`, `elevation_regular: bool`, `t1_method` (ct/rayleigh/given), `t1_given_s`, `ct`, `drift_limit_class` (brittle/ductile/isolated), `nu`, `multiple_resisting_systems: bool`, `claims_simple_masonry: bool`, `masonry_wall_area_ratio`, `accidental_eccentricity_ratio` |
| `En1998Bridge` (629–647) | `id`, `period_ratio` (T_isol/T_fixed), `fundamental_period_s`, `v_rd_n`, `bearing_d_rd_m`, `permanent_gk_n`, `correlated_occupancy`, `variables: Vec<En1998VariableAction>` |
| `En1998Assessment` (655–665) | `id`, `knowledge_level` (kl1/kl2/kl3), `limit_state` (nc/sd/dl), `supported_building_id`, `r_k_n` [N], `gamma_el` |
| `En1998Silo` (673–694) | `id`, `height_m`, `radius_m`, `permanent_gk_n`, `content_qk_n`, `content_category`, `filling_ratio`, `n_rd_n` [N], `v_rd_n` [N], `q_nominal` |
| `En1998Tank` (702–720) | `id`, `height_m`, `radius_m`, `permanent_gk_n`, `content_qk_n`, `content_category`, `filling_ratio`, `v_rd_n` [N] |
| `En1998Foundation` (728–740) | `id`, `supported_building_id`, `area_m2`, `p_rd_pa`, `h_rd_n`, `k_foundation`, `k_soil` |
| `En1998RetainingWall` (748–756) | `id`, `height_m`, `phi_deg`, `soil_gamma`, `r`, `h_rd_n_per_m` |
| `En1998Tower` (764–777) | `id`, `height_m`, `m_rd_nm`, `is_chimney: bool`, `q_nominal`, `permanent_gk_n`, `correlated_occupancy`, `variables` |

Enums:

- `DeSeismicZone` (395–404): `Zone0 | Zone1 | Zone2 | Zone3` (keys `zone0`…`zone3`). `a_gr()` (408–415) returns 0.0 / 0.4 / 0.6 / 0.8 m/s²; `as_u8()`.
- `DeGroundCombo` (433–458): `AR | BR | CR | BT | CT | CS` (keys `A-R`…`C-S`). `as_str()` (461–470).

Snapshot constructors (fixtures in code; the `.dsl.semio` examples must decode to these, see §7):
`compliant_de_office` (61–131, zone 2, B-R, RC frame DCH, 4 storeys, literal masses such as `2334780.000000`), `noncompliant_de_office` (134–165), `compliant_de_multipart` (168–238, adds bridge `br-1`, assessment `as-1`, silo `si-1`, tank `tk-1`, foundation `fd-1`, retaining wall `rw-1`, tower `tw-1`), `noncompliant_de_multipart` (241–311), `compliant_en_office` (314–321, EN ground B, type1), `noncompliant_en_office` (324–331, ground D), `compliant_de_torsion_regular` (334–347), `noncompliant_de_torsion_irregular` (350–367).

### 1.2 Artifact schema (`SCH`)

- `En1998Artifact` (L20–41): the same ten fields as the snapshot, each `#[state(artifact)]`. `to_snapshot` (45–58) and `from_snapshot` (60–73) copy field by field. `Default` (77–81) = `from_snapshot(Snapshot::default())`.
- `AnnexParams` (L546–551): `De { zone: na_de::SeismicZone, combo: na_de::GroundCombo }` | `En { a_gr: f64, ground: char, spectrum_type1: bool }`. `ground_params()` (558–569) returns (a_g, S, T_B, T_C, T_D); `elastic_response_spectrum(t)` (571–574).
- `na_de::SeismicZone` (90–111) mirrors the snapshot enum with `a_gr`, `as_u8`; `From<DeSeismicZone>` (113–122).
- `na_de::GroundCombo` (126–166): mirror enum with `From<DeGroundCombo>`, `parse(class, subsoil)` (142–153, not used by the evaluator), `spectrum_params()` (156–165).
- Descriptor `en1998_artifact_schema_descriptor` (597–607) wires the schema, snapshot, diff and mutation leaves.

### 1.3 Inference model (`INF/🦀️.rs`)

- `En1998Inference` (22–25): single `#[derived] outline: En1998Outline`. `infer(snapshot)` (33–40) = `En1998Outline::compute`. Field reads declared (42–51): annex, site, buildings, bridges, assessments, silos, tanks, foundations, retainingWalls, towers.
- `En1998Outline` (`INF/🧾outline/🦀️.rs` 18–26): `section_outline: Vec<String>` (10 section names), `field_count: u32`, `entry_count: u32` (sum of entity counts). It is a count summary, not a check.

### 1.4 Diff model (`ANY/🧬️schema/🔺️diff/🦀️.rs`, L179–199)

`En1998Diff` with `annex: Option<String>`, `site: Option<En1998Site>` and one keyed delta per collection (`buildings`, `bridges`, `assessments`, `silos`, `tanks`, `foundations`, `retaining_walls`, `towers`). Each delta has removal / insertion / relocation / modification variants. The per-entity patches (lines 21–168) cover every stored field, so the diff layer can express any field change; the editor does not expose them all (see §4.4).

### 1.5 Mutation vocabulary (`ANY/🧬️schema/🧬️mutations/🦀️.rs`, enum L54–84; KINDS L86–116)

29 kinds, enum `En1998Mutation`. Payloads (from the leaf files under `🧬️mutations/`):

| Kind | Payload |
|---|---|
| `update-site` | `site: En1998Site` |
| `change-annex` | `new_annex: String` |
| `insert-building` | `index: Option{usize}`, `building` |
| `remove-building` | `index: usize` |
| `change-system-v-rd-n` | `building_index`, `system_index`, `new_base_shear_resistance_n` |
| `change-storey-permanent-gk-n` | `building_index`, `storey_index`, `new_permanent_gk_n` |
| `change-storey-stiffness-x` | `building_index`, `storey_index`, `new_stiffness_x` (X only) |
| `change-storey-drift-xm` | `building_index`, `storey_index`, `new_drift_x_m` (X only) |
| `change-building-plan-regular` | `building_index`, `new_plan_regular` |
| `change-elevation-regular` | `building_index`, `new_elevation_regular` |
| `change-member-detailing` | `building_index`, `member_index`, `new_detailing_compatible_with_q` |
| `change-masonry-wall-ratio` | `building_index`, `new_masonry_wall_area_ratio` |
| `insert-bridge` / `remove-bridge` | `index`, `bridge` / `index` |
| `change-bridge-v-rd-n` | `index`, `new_v_rd_n` |
| `insert-assessment` / `remove-assessment` | `index`, `assessment` / `index` |
| `change-assessment-rkn` | `index`, `new_r_k_n` |
| `insert-silo` / `remove-silo` | `index`, `silo` / `index` |
| `insert-tank` / `remove-tank` | `index`, `tank` / `index` |
| `insert-foundation` / `remove-foundation` | `index`, `foundation` / `index` |
| `insert-retaining-wall` / `remove-retaining-wall` | `index`, `wall` / `index` |
| `insert-tower` / `remove-tower` | `index`, `tower` / `index` |
| `change-tower-m-rd-nm` | `index`, `new_m_rd_nm` |

Count: 12 change, 8 insert, 8 remove, 1 update. Each kind has `🔺️diff`, `↩️inverse` and `🧬️schema` leaves; `🧭️edit-rules/🦀️.rs` L6–41 maps editor paths to kinds (§4.4).

### 1.6 Editor command enum (`EDR` L58–73)

`En1998Command`: `evaluate`, `selected-check` (`setSelectedCheckIndex`), `set-field`, `insert-item`, `remove-item`, `apply-remedy`.

### 1.7 Config, presence, transient

- `✏️editor/🎚️config/🧬️schema/` and `👥️presence/🧬️schema/` are placeholders: `NoConfig`, `NoPresence`, `NoDraft`, `NoTransient` (`📌️.empty.md` markers). No persisted view config beyond the results window's selected-check index.

---

## 2. Checks and verifications

Every check is built through `CheckResult::assess(id, part, ClauseId, SubjectRef, title)` and `.utilization(computed, limit)`. In the compliance kernel, `utilization()` sets status Pass when util ≤ 1 and Fail otherwise, and a zero limit yields util 0 and Pass (`⚖️compliance/✅️check-results/🦀️.rs` L174–183). `minimum()` (186–196) sets Pass/Fail from the minimum. Remedies are `at_least`, `at_most`, `exactly`, `one_of` (`✅️check-results/🦀️.rs` L84–…).

Clause strings are those written in the code (the `ClauseId` argument). They are not checked against the standard text.

### 2.1 Integrity (all parts)

| Check id | Clause | Criterion | Location |
|---|---|---|---|
| `en1998.integrity.duplicate.{table}.{id}` | "EN 1998 §2 id" | Duplicate `id` in buildings, bridges, assessments, silos, tanks, foundations, retainingWalls, towers, and in systems, storeys, members, storey variables, bridge/tower variables → Fail with `one_of` remedy | `PE` L5–73 |
| `en1998.site.deNaConsistencyUnderEn` (EN annex only) | EN 1998-1 3.2.1 | Zone a_gR equals site a_gR (or site a_gR ≤ 0); DE combo equals the mapping A→A-R, B→B-R, C→C-R, D/E→C-S (default B-R); util 0.5/1.5 | `FSC` L27–72 (mapping L34–40, check L42–71) |
| `en1998.site.aGr.derived` (DE only) | DIN EN 1998-1/NA, 3.2.1 | site.aGr ≠ zone a_gR → **Warning** (not Fail), site value ignored | `FSC` L74–100 |

### 2.2 Part 1 (DIN EN 1998-1 and NA-DE): building level

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.1.{b}.zone0` | 1 3.2.1 (DIN NA) | `NotApplicable` when DE zone 0 or a_g ≤ 0 (building skipped) | `FSC` L104–127 |
| `en1998.1.buildings.na`, `en1998.1.{b}.system.na` | 1 4, 1 5 | NotApplicable when empty | `FSC` L106–113, L129–137 |
| `en1998.1.{b}.t1Given` | 1 4.3.3.2.2 | util = `t1GivenS` / max(T1, 0.05) when method = given; else limit 0.05 and Fail if given > 0.05. **Tautological when method=given** (T1 is then t1GivenS, util = 1) | `BRS` L38–62 |
| `en1998.1.{b}.regularity.claim` | 1 4.2.3 | util 0.5 if planRegular ∧ elevationRegular else 1.5 | `BRS` L64–96 |
| `en1998.1.{b}.storey.{s}.planRegularity` | 1 4.2.3.2 | e0 = max(\|CM−CS\|x, y); r = min(rx, ry), rx = √(Kθ/Ky), ry = √(Kθ/Kx), Kθ = Kx(Ly/2)² + Ky(Lx/2)²; ℓs = √((Lx²+Ly²)/12); criterion e0 ≤ 0.30·r and ℓs ≤ r; util = e0/(0.30 r) | `BRS` L98–136; metrics `SCH` L233–252 |
| `en1998.1.{b}.elevationRegularity.{lo}-{hi}` | 1 4.2.3.3 | k = Kx+Ky; k_ratio = k_hi/k_lo; m_ratio = m_hi/m_lo; util = max(0.70/k_ratio, m_ratio/1.50). Criterion direction is **verify** (§8) | `BRS` L138–173 (util L149) |
| `en1998.1.{b}.planDims` | 1 4.3.2 | Pass iff Lx > 0 and Ly > 0; util shown as Lx/Ly | `BRS` L176–209 |
| `en1998.1.{b}.multipleSystems` | 1 5.2.2.1 | If `multipleResistingSystems`: needs a frame/dual and a wall system, and plan regularity; util is 0.5/1.5 artificial | `BRS` L211–249 |
| `en1998.1.{b}.lfm` | 1 4.3.3.2 | Lateral force method applicable iff elevation regular ∧ T1 ≤ 4·Tc ∧ T1 ≤ 2.0 s; otherwise **Warning** (util 0.5/0.95 artificial) | `BRS` L251–268; `SCH` L437–439 |

### 2.3 Part 1: per structural system (direction, q, base shear, storeys, drift, P-Δ)

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.1.{b}.{sys}.direction` | 1 4.3.3.2.1 | direction ∈ {x, y} and unique within building; else util 1.5 and remedy | `DBS` L5–48 |
| `en1998.1.{b}.{sys}.qLimit` | 1 5.2.2.2 | q = max(1, q0·αu/α1·kw) (`SCH` L265–267); q_max from table (`SCH` L270–311); ok iff q ≤ q_max; util = min(q, q_max)/q_max if ok else q/q_max | `DBS` L51–103 |
| `en1998.1.{b}.{sys}.baseShear` | 1 4.3.3.2.2 | S_e(T1) with η = 1; S_d = max(S_e·γI/min(q, q_max), 0.2·a_g); M = Σ storey masses (ΣG_k + Σψ_E·Q_k)/9.81; λ = 0.85 if T1 ≤ 2·Tc and n > 2; F_b = S_d·M·λ; util = F_b / V_Rd,sys | `DBS` L105–138 (S_d L70, F_b L72) |
| `en1998.1.{b}.{sys}.torsion` | label "1 4.3.3.2.4" | e_ai = ratio·Lx(or Ly); e0 = \|CM−CS\| of **first storey only**; e_tot = e0 + e_ai; δ = min(1 + 1.2·e_tot/L_e, 3.0); util δ/1.5. Formula differs from EN 1998-1 Eq. (4.12), see §8 | `SFD` L18–59 (δ at L30) |
| `en1998.1.{b}.storey.{s}.roofPhi` | 1 4.2.4 | Roof requires `correlatedOccupancy = true`; util 0.5 / 1.5. Inconsistent with φ_roof = 1.0 used in the mass (`SCH` L187–196), see §8 | `SFD` L60–88 |
| `en1998.1.{b}.{sys}.storeyForces.{s}` | 1 4.3.3.2.3 | F_i = F_b·δ·m_i·z_i / Σ m_j·z_j (`SCH` L407–419); V_i = F_b·δ − Σ_{j<i} F_j; V_Rd,i = storey.shearResistanceN, or system V_Rd if 0; util V_i/V_Rd,i | `SFD` L90–116 |
| `en1998.1.{b}.{sys}.drift.{s}` | 1 4.4.3.2 | Input d_r = driftXM/driftYM (no q·d_e amplification); limit = ν·θ·h with θ = 0.0075 (ductile default), 0.005 (brittle), 0.010 (isolated) (`SCH` L348–354, L427–429); util d_r/limit | `SFD` L121–160 (limit L139) |
| `en1998.1.{b}.{sys}.pdelta.{s}` | 1 4.4.2.2 | θ = P_tot·d_r / (V_tot·h), P_tot = Σ_{j≥i} seismic actions, V_tot = V_i; util θ/0.30; 0.30 limit | `SFD` L162–195 (θ formula `SCH` L432–435 is not called here; inline at L164) |

### 2.4 Part 1: members (RC and steel), ductility and masonry

Member checks run once per structural system (see §8 for duplicate ids). `gov_dc` is the system's ductility class.

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `...member.{m}.rc.minB` (columns) | 1 5.4.3.2.1 | b ≥ 0.25 m for DCM/DCH, 0.20 otherwise (`SCH` L314–320) | `DCM` L19–42 |
| `...member.{m}.rc.beamMinB` (beams) | 1 5.4.1.2.1 | b_w ≥ 0.20 m (hard-coded L69) | `DCM` L68–93 |
| `...member.{m}.rc.rhoPrimeAbs` | 1 5.4.3 | ρ′ ≤ ρ (util ρ′/ρ) — label says "Abs" but it is a ratio check | `DCM` L44–67 |
| `...member.{m}.rc.rho` | 1 5.4.3 | ρ within [ρ_min, ρ_max]: column [0.01, 0.04]; beam DCH [0.0035, 0.04], beam other [0.0025, 0.04], other [0.002, 0.04] (`SCH` L323–333) | `DCM` L96–125 |
| `...member.{m}.rc.noSteelClass` | 1 5 | steelSectionClass must be 0 for RC | `DCM` L127–149 |
| `...member.{m}.rc.omegaWd` (columns) | 1 5.4.3.2.2 | ω_wd ≥ 0.12 (DCH) / 0.08 (DCM) (`SCH` L335–341) | `DCM` L151–173 |
| `...member.{m}.rc.rhoPrime` (beams) | 1 5.4.3.1.2 | ρ′/ρ ≥ 0.5, applied for all ductility classes | `DCM` L174–197 |
| `...member.{m}.rc.omegaWdBeam` (beams) | 1 5.4.3.2.2 | ω_wd ≤ 0.30 (hard-coded), status recorded | `DCM` L198–221 |
| `...member.{m}.steel.class` | 1 6.5 | section class ∈ [1, max]; max = 1 if q > 4, 2 if q > 2, else 3 (`SCH` L344–346) | `DCM` L223–245 |
| `...member.{m}.qDetail` | 1 5.2 | Input flag `detailingCompatibleWithQ` only (not computed) | `DCM` L247–269 |
| `en1998.1.{b}.masonry.simple` | DIN EN 1998-1/NA 9.7.2 (label "Table NA.12") | Required wall ratio by zone 0/1/2/3 = 0/2/3/4 % × 1.25 for IC III/IV (`SCH` L442–448); fail if claimed (flag or system type masonry) and ratio < limit. Evaluated once per system (duplicates). Table NA.12 value is a **simplification** per the code comment | `DCM` L273–313 |

Not present: capacity design of columns, beams, walls and joints (EN 1998-1 §5.2.3–5.2.4, §5.5, §6.6, §7, §8, §9 as applicable); composite, timber and masonry member checks (only the q table covers these materials).

### 2.5 Part 1: NA-DE and EN annex common items

- `en1998.1.{b}.zone0`: see 2.2.
- Spectrum and ground parameters are computed in `SCH` (see §3); the EN annex uses `en_ground_params` (`SCH` L577–595) for ground A–E, type 1 and type 2.

### 2.6 Part 2: bridges (`PE` L75–128)

Only evaluated when `bridges` is non-empty; otherwise `en1998.2.na` (NotApplicable, L78).

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.2.{bridge}.periodRatio` | 2 7.1 | T_isol/T_fixed; Fail if < 1.0; util = ratio / **3.0** (arbitrary limit) | `PE` L84–107 (util L92) |
| `en1998.2.{bridge}.shear` | 2 5.3 | F_b = S_d(T)·M·1.0, S_d with q = q_isol = max(r², 1) (`SCH` L453–456, no EN basis); M from `entity_seismic_mass_kg` (non-roof φ) | `PE` L108–121 |
| `en1998.2.{bridge}.bearing` | 2 7.5 | d_Ed = S_d·(T/2π)²; util d_Ed / d_Rd (`SCH` L457–461) | `PE` L122–126 |

### 2.7 Part 3: retrofit assessment (`PE` L130–195)

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.3.na` | 3 2.3 | NotApplicable when no assessments | `PE` L133 |
| `en1998.3.{a}.supportedBuilding` | 3 2 | Reference resolves; util is **fake** 0.5 (resolved) / 1.5 (dangling); Fail with `one_of` remedy if dangling | `PE` L136–165 |
| `en1998.3.{a}.capacity` | 3 2.1 | R_d = R_k / (CF·γ_el); CF from KL (kl1 1.35, kl2 1.20 default, kl3 1.00; `SCH` L465–471), CF = 1.0 for DL; E_d = base shear of the supported building at a_g × factor (NC 1.72, DL 0.5, else 1.0; `SCH` L472–479, ad hoc); util E_d/R_d; remedy R_k ≥ E_d·CF·γ_el | `PE` L175–193 (E_d L177) |
| `en1998.3.{a}.limitState` | 3 2.1 | Valid iff nc/sd/dl; util fake 0.5 / 2.0 | `PE` L184–192 |

### 2.8 Part 4: silos and tanks (`PE` L197–247)

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.4.silo.na`, `en1998.4.tank.na` | 4 3, 4 4 | NotApplicable when empty | `PE` L199–201, L229–231 |
| `en1998.4.{silo}.wall` | 4 3.4 | H/R = h/r; impulsive mass ratio 1 − 0.436·tanh(H/R) (`SCH` L493–494, **verify**); convective ratio = 1 − impulsive, clamped 0.1–0.9; T_i fixed at **0.3 s** (`PE` L205); T_c = 2π√(R/(1.84·g)) (`SCH` L502–504); V = √(V_i² + V_c²) (SRSS, `SCH` L506); util V / **N_Rd** (base shear compared with an axial resistance) | `PE` L202–222 |
| `en1998.4.{silo}.anchor` | 4 3.5 | util V / V_Rd (same V) | `PE` L223–227 |
| `en1998.4.{tank}.shear` | 4 4.3 | T_i = 2π√(0.574·R / √(tanh(9.81·H/R))) (`SCH` L499–501, **verify**, dimensionally suspicious); V = m·(M_i·S_e(T_i) + M_c·S_e(T_c)); no behaviour factor; util V/V_Rd | `PE` L232–246 |

Silo mass = ΣG_k + filling_ratio·ψ_E(E, roof=true, correlated=true)·Q_k (`SCH` L227–230).

### 2.9 Part 5: foundations and retaining walls (`PE` L266–356)

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.5.foundation.na`, `en1998.5.wall.na` | 5 7, 5 7.3.2 | NotApplicable when empty | `PE` L268–270, L339–341 |
| `en1998.5.{f}.supportedBuilding` | 5 5.3 | As 2.7 (fake 0.5/1.5 utilization) | `PE` L273–300 |
| `en1998.5.{f}.bearing` | 5 7.3 | P_ed = V_b/A; P_rd = p_Rd·(1 − 1.5·a_g/g, min 0.5) (`SCH` L509–512, ad hoc); util P_ed/P_rd; remedy A ≥ V_b/P_rd | `PE` L307–316 |
| `en1998.5.{f}.sliding` | 5 5.3 | H_ed = V_b vs H_Rd; util H_ed/H_Rd | `PE` L317–322 |
| `en1998.5.{f}.radiationDamping` | 5 5.3 | ξ = (k_f/k_s)/(1 + k_f/k_s) (`SCH` L529–533); util ξ/1.0, so **always Pass** (no limit) | `PE` L323–338 |
| `en1998.5.{w}.thrust` | 5 E.2 | k_h = (a_g/g)·S/r; Mononobe–Okabe K_AE (`SCH` L520–525, vertical wall, zero friction); H_ed = ½·γ·H²·K_AE per metre; util H_ed / H_Rd,per m | `PE` L342–355 (k_h L343, K_AE L344, H_ed L345) |

Not present: liquefaction (EN 1998-5 4.1.4 / Annex B), kinematic interaction, pile checks, foundation capacity design.

### 2.10 Part 6: towers (`PE` L358–381)

| Check id | Clause (as coded) | Computed / criterion | Location |
|---|---|---|---|
| `en1998.6.na` | 6 4.3 | NotApplicable when no towers | `PE` L361 |
| `en1998.6.{t}.overturning` | 6 4.3.2 | q = clamp(q, 1.0, 2.0) for chimney, clamp(q, 1.0, 3.5) otherwise (`SCH` L537–539); S_e at fixed **T = 0.5 s** (`PE` L366); S_d; V = Γ·S_d·M with Γ = **1.5** constant (`SCH` L540); M_Ed = V·(2/3)·H (`SCH` L543); util M_Ed/M_Rd | `PE` L364–379 |

Only overturning is checked. No second-order, no shear, no ovalisation or wind checks.

### 2.11 Part-by-part coverage summary

| EN 1998 part | Computed | Stub or missing |
|---|---|---|
| 1 ground types | EN types A–E (type 1 and type 2) in `SCH` L577–595; DE combos via NA.4 | No Eurocode ground-type classification from V_s30 |
| 1 elastic / design spectrum | S_e (3.2.2.2) and S_d with β = 0.2 (`SCH` L357–377) | Damping ratio fixed (η = 1), no 3.2.2.5 lower-bound variants |
| 1 behaviour factor q | q0·αu/α1·kw, q_max table (`SCH` L265–311) | Table lookup is hard-coded; lower bound clamp at 1.0 (**verify**) |
| 1 regularity | Plan (4.2.3.2) and elevation (4.2.3.3) checks | Elevation direction and plan metric convention (§8) |
| 1 lateral force method | Applicability check (4.3.3.2) and base shear (4.3.3.2.2) | |
| 1 modal response spectrum | — | Absent |
| 1 combination of components (4.3.3.5) | — | Absent (SRSS only in silos, `SCH` L506) |
| 1 P-Δ (4.4.2.2) | θ and limit 0.30 | Second-order amplification not applied |
| 1 interstorey drift (4.4.3.2) | ν·θ·h limit | Formula direction and q·d_e (§8) |
| 1 capacity design concrete | Member detailing and ρ, ω_wd, ρ′/ρ, b_min | No M_Rd/V_Rd capacity design |
| 1 capacity design steel | Section class vs q | No capacity design |
| 1 capacity design composite | — | Only q table |
| 1 capacity design timber | — | Only q table |
| 1 capacity design masonry | Simple-building wall ratio | No Eurocode 8 masonry capacity checks |
| 2 bridges | Period ratio, base shear, bearing displacement | Simplified; no pier, seat or deck checks |
| 3 assessment | KL, LS, CF, γ_el, E_d/R_d | Ad hoc LS factors; no displacement-based method |
| 4 silos | Impulsive/convective base shear, anchor | Heuristic periods and mass ratios |
| 4 tanks | Impulsive/convective shear | Heuristic |
| 5 foundations | Bearing, sliding, radiation damping (trivial), retaining wall thrust | No liquefaction, no kinematic |
| 6 towers | Overturning moment | Fixed period and participation |

---

## 3. NA-DE (DIN EN 1998-1/NA) parameters in use

| Parameter | Values used | Location | Source claim in code |
|---|---|---|---|
| Seismic zones and a_gR | zone 0 = 0.0, zone 1 = 0.4, zone 2 = 0.6, zone 3 = 0.8 m/s² | `SCH` L93–101 (`a_gr`); `SNAP` L407–415; catalogue `ANY/✏️editor/📌️panels/📚️catalogue/🦀️.rs` L27–48; oracle `🔮️oracles/⚖️compliance/🐍️.py` L10–11 | Table NA.1 (2011) |
| Ground combo spectrum (S, T_B, T_C, T_D) | A-R (1.00, 0.05, 0.20, 2.0); B-R (1.25, 0.05, 0.25, 2.0); C-R (1.50, 0.05, 0.30, 2.0); B-T (1.25, 0.10, 0.30, 2.0); C-T (1.50, 0.10, 0.40, 2.0); C-S (0.75, 0.10, 0.50, 2.0) | `SCH` L156–165; catalogue L64–86; oracle L14–23 (identical in three places, hand-copied) | Table NA.4 (**values not verified against the NA text**) |
| Zone 0 | Exemption, building skipped | `FSC` L104–127 | DIN NA |
| Importance class → γI | I 0.8, II 1.0, III 1.2, IV 1.4 (also CC1, CC3, CC4 aliases) | `SCH` L256–263 | EN 1998-1 Table 4.3 (same numbers; DE aliases) |
| ψ2 (imposed loads) | A, B 0.3; C, D 0.6; E 0.8; F 0.6; G 0.3; H 0.0; snow 0.2; wind 0.0; unknown 0.3 | `SCH` L171–185 | EN 1990/NA Table NA.A.1.1 (snow value **verify**) |
| φ (correlated occupancy) | roof 1.0; storeys correlated 0.8; independent 0.5 | `SCH` L188–196 | EN 1998-1 Table 4.2 |
| Simple masonry wall ratio | zone 0 = 0, zone 1 = 2 %, zone 2 = 3 %, zone 3 = 4 %; × 1.25 for IC III and IV | `SCH` L442–448 | DIN NA Table NA.12 "simplified" (code comment) |
| Annex switch | `annex = de` → a_gR from zone; site.aGr ignored (Warning if inconsistent) | `FSC` L74–100; `SH` L23–40 | |
| Ground combo under EN annex | Consistency only: A→A-R, B→B-R, C→C-R, D/E→C-S | `FSC` L34–40 | ad hoc mapping |

The DE zone table feeds the catalogue panel (NA.1 and NA.4 rows), and `compliant_de_office` uses zone 2 with B-R.

---

## 4. Editor

Source: `EDR` (manifest, commands, mode). Fields and labels: `ANY/🏷️field-meta/🦀️.rs`.

### 4.1 Manifest and mode

- `Editor::builder` with dialect `s.norm.en1998@1/*` (`EDR` L239–295). Label "EN 1998" (L50), variant `en1998` (L52).
- Mode `edit` (`ANY/✏️editor/🎭️modes/✏️edit/🦀️.rs`), default. Layout: `row`, Inputs 42 % | Results 58 % (`🪟️windows/…` via `edit/🦀️.rs` L15–17).
- Keybindings: `mod+z` undo, `mod+shift+z` redo (`EDR` L290–291).
- Examples in the navbar: the four `📚️examples` (§7).
- Media ports: `report:out`, `artifact:out` (export) and `model:in`, `artifact:in` (import) through the shared surface (`EDR` L183–190).

### 4.2 Windows

| Window | Body key | What it renders | Location |
|---|---|---|---|
| Inputs `norm-en1998-inputs` | `norm.en1998.play.inputs` | `render_document_editor`: a generic recursive tree/property editor built from the snapshot value tree. Each leaf gets its label, unit and choice list from field-meta. Edits emit `setField` with a JSON value; collections get insert and remove. Not raw JSON (the TS twin `🪟️windows/📥️inputs/🟦️.ts` still says "pretty-printed JSON"; stale) | `ANY/✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs` L11–17; shared `📝️document-editors/🦀️.rs` L169–178 |
| Results `norm-en1998-results` | `norm.en1998.play.results` | `render_report`: one tree grouped by part with a verdict mark per part (✓/✗); each check is a row; Fail and Warning rows expand to the explanation and their remedies, each remedy an "Apply" action (`applyRemedy`), `one_of` remedies as "Choose" actions (`setField`). The results window config stores `selected_check_index` | `…/📊️results/🦀️.rs` L17–25; shared `🌳️report-tree/🦀️.rs` L214–232 |

No 3D view, no 2D diagram, no spectrum plot, no utilisation heat map, no M–N curve, no stress block, no S–N curve. The results window's icon is `bar-chart-3`, but it renders a list.

### 4.3 Panels

| Panel | Body key | What it renders | Location |
|---|---|---|---|
| Artifact (Workbench) | `norm.en1998.play.artifact` | Summary line: family label, check count, failing count, worst utilisation, verdict, per-part ✓/✗ | `…/🗿️artifact/🦀️.rs` L16–24; shared `render_summary` `📝️document-editors/🦀️.rs` L187– |
| Catalogue (Workbench) | `norm.en1998.play.catalogue` | Examples list plus two reference tables: NA.1 (zone → a_gR) and NA.4 (ground combo → S, T_B, T_C, T_D) | `…/📚️catalogue/🦀️.rs` L23–86, render L89–91 |
| Inspection (Details) | `norm.en1998.play.inspection` | One check in full: id, part, clause, subject, subject path, status, title, explanation, computed, limit, utilization (2 dp), annex, and each remedy with its current → required value and path | `…/🔍️inspection/🦀️.rs` L16–25; shared `📚️catalogue-and-inspection/🦀️.rs` L150–215 |

### 4.4 Commands

| Command (action id) | Payload | Effect | Location |
|---|---|---|---|
| `evaluate` | none | Emits zero mutations; report is recomputed on every render (`NormHost::from_document`) | `…/🧮️evaluate/🦀️.rs` L24–26 |
| `setSelectedCheckIndex` | `index: Option{u32}` | View-only: selects the check shown in Inspection | `…/☑️selected-check/🦀️.rs` L24–30 |
| `setField` | `path: String`, `value_json: String` | Shared `dispatch_set_field` → `EDIT_RULES.resolve` (`ANY/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`) | `…/✏️set-field/🦀️.rs` L18–20 |
| `insertItem` | `path`, `index: u32`, `value_json?` | Shared insert, resolves via `EDIT_RULES` | `…/➕insert-item/🦀️.rs` L19–21 |
| `removeItem` | `path`, `index` | Shared remove, resolves via `EDIT_RULES` | `…/➖remove-item/🦀️.rs` L18–20 |
| `applyRemedy` | `checkId`, `remedyIndex` | Looks up the **first** check with that id, takes its remedy, applies it as a `setField`/mutation | `…/🩹apply-remedy/🦀️.rs` L19–21; `📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs` L192–196, L255–275 |

`EDIT_RULES` (`ANY/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs` L6–41) is the only route from editor paths to mutation kinds. It wires:

- 12 set-field paths: `towers[].mRdNm`, `buildings[].storeys[].permanentGkN`, `buildings[].members[].detailingCompatibleWithQ`, `assessments[].rKN`, `buildings[].systems[].baseShearResistanceN`, `annex`, `buildings[].elevationRegular`, `buildings[].storeys[].driftXM`, `buildings[].storeys[].stiffnessX`, `bridges[].vRdN`, `buildings[].planRegular`, `buildings[].masonryWallAreaRatio`.
- 8 insert collections: buildings, bridges, assessments, towers, tanks, retainingWalls, foundations, silos.
- 8 remove collections: assessments, bridges, buildings, foundations, retainingWalls, silos, tanks, towers.

Any other path is refused with `no kind sets '{path}'` (`📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs` L124). Consequences: seismic zone, a_gR, ground combo, spectrum type, importance class, system q0, direction, material, ductility, storey height, Y stiffness, centres of mass and stiffness, Y drift, member ρ, ρ′, ω_wd, section class, bridge period ratio and fundamental period, assessment knowledge level, limit state, silo, tank, foundation and wall fields (only insert and remove are routed), tower q and chimney flag are **not editable** through the editor, even though the Inputs window shows them. `update-site` exists as a mutation kind but no edit rule routes to it.

### 4.5 Field metadata (`ANY/🏷️field-meta/🦀️.rs`)

- 127 path entries in `TABLE` (L118 onward), each with EN and DE label, unit where set, and optional choice list.
- 16 choice lists: zones, DE ground combos, importance classes, annex, T1 method, drift class, direction, system type, material, ductility class, EN ground type, EN spectrum type, booleans, member role, imposed-load category, limit state.
- Known typo: the annex choice label `EN empfohlenungen` (German, should be "EN-Empfehlungen").
- Coverage is checked only for field-meta presence on the default snapshot (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L148–183), not for edit-rule coverage.

---

## 5. Viewer

Source: `ANY/👁️viewer/🦀️.rs` and `ANY/👁️viewer/🎭️modes/👁️view/`.

- `En1998Viewer` implements `ArtifactViewer` only (L50–90). Command channel: one inert `Noop` variant (L30–43). `handle` returns an empty `ViewEmit` (L73–82).
- Mode `view`, single full-pane window `report` (`🎭️modes/👁️view/🦀️.rs` L15–18).
- Window `report` (kind `TableWindowKit`, `🪟️windows/📊️report/🦀️.rs`): recomputes `inferences::evaluate(document)` on each render (L30–33), then tables it with columns **Part | Clause | Subject | Status | Utilization (2 dp) | Title | Remedy** (`🏷️ids-chrome-and-labels/🦀️.rs` L49–60). Each row shows only the first remedy of the check.
- Unknown body key: plain text "Unknown body: …" (`👁️viewer/🦀️.rs` L84–89).
- No inspection, no drill-down, no diagrams, no 3D, no edits.
- Viewer icon `check-circle`.

---

## 6. Tests and oracles

### 6.1 Rust tests

About 97 `async_test` functions across 28 files (`#[semio_framework_async_macros::async_test]`), plus the Gherkin-driven mutation harness. Main files:

| File | Tests | What they check |
|---|---|---|
| `ANY/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | 24 | DE BR spectrum at T=0.25 s = 1.875 (L11–18); EN vs DE divergence (L20–29); T1 and λ (L31–42); zone enum range (L44–54); compliant default has no Fail (L56–60); noncompliant has Fails with remedies (L62–68); remedies flip checks to Pass (V_Rd, detailing, regularity: L70–113); field-meta covers every editable leaf (L148–183); example JSON validates against snapshot JSON Schema via Python `jsonschema` (L185–201); Python oracle within 0.5 % (L203–230); NA.4 combo table vs oracle (L587–647); NA.4 catalogue cell equals evaluated spectrum (L676–709); duplicate id fails with `one_of` (L711–728); dangling supportedBuilding (L730–751); every leaf perturbation changes the report (L449–531, L532); committed examples have distinct EN and DE text (L539–558) |
| `ANY/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` | 1 | default evaluate runs |
| `ANY/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | 2 | default complies; empty bridges → `en1998.2.na` NotApplicable |
| `ANY/🧬️schema/💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs` | 2 | outline field count and determinism |
| Editor tests across `ANY/✏️editor/**` | 35 | editor rendering, locale switch, remedy application, command behaviour |
| `ANY/👁️viewer/…/🧪️tests/` | 3 + 1 + 2 | viewer dialect and unknown body |
| `ANY/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` | 11 | SQLite round trip |
| `ANY/🧬️schema/🧬️mutations/🧪️tests/` | 4 | kinds catalogue, vectors, unit |
| `ANY/📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🦀️.rs` | 4 | asset non-empty, inference determinism, every example asset decodes to its code-built snapshot (L28–38) |

### 6.2 Mutation feature (`ANY/🧪️tests/🫨️mutate-en1998-1/`)

- `🥒️.feature` (143 lines): 29 `✅apply` vectors plus refusal rows (`⛔dupe`, `🟰noop`, `🔬middle-row`) for each kind, run for `mutate-` (apply, refuse) and `inverse-` (apply then inverse restores before-snapshot), plus an identity round trip of the committed `.dsl.semio` carrier against the binary pack twin.
- `🦀️.rs` (533 lines): Rust subject adapter over the 29 kinds, reads the same committed fixtures with `include_str!`.
- `🐍️.py` (129 lines): Python second implementation, imported from the shared norm engine (`import_module("🐍️")`), registered in `ANY/🔮️oracles/🔣️.json` as `en1998-1-python-independent` (`verified-native-second-implementation`, package empty, testOnly). The registration says no third-party library reads or writes `s.norm.en1998`.

### 6.3 Numeric oracle (`ANY/🔮️oracles/⚖️compliance/🐍️.py`, 124 lines)

- Standard library only: `json`, `sys`, `typing`. No third-party package.
- Computes only one quantity: base-shear utilisation per structural system (`evaluate`, L71–101), for the DE annex (zone a_gR, NA.4 spectrum) and for the EN annex with **hard-coded** S = 1.2, T_B = 0.15, T_C = 0.40, T_D = 2.0 (ground ignored).
- Modes: `--json` (stdin → checks), `--combo-table` (NA.4 spectrum and S_e at 0.25 s), no argument (self-check, L103–121).
- Differences from Rust: q is not capped by q_max (Rust caps it, `DBS` L70); `spectrum_params` silently defaults unknown combos to B-R (L23); `a_gr_de` defaults unknown zones to 0.6 (L11); T1 is always CT-based (no Rayleigh or given); no S_d lower bound is applied to EN annex consistency.
- Tolerance: 0.5 % relative (`python_oracle_matches_within_half_percent` and `de_ground_combo_table_matches_oracle_for_all_combos`).
- Oracle registration: `ANY/🔮️oracles/🔣️.json`, capability `en1998-1-mutate`, comparison profile `ordered-json-v1`, license AGPL-3.0-only, testOnly.

### 6.4 Third-party libraries

- Python `jsonschema` (test only, `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L185–201, `python3 -c "import jsonschema…"`).
- Rust `serde` and `serde_json` (dev/test only, `cfg_attr(test)` and `[dev-dependencies]`).
- No third-party library validates any numeric check. The only numeric oracle is in-house (§6.3).
- `Cargo.lock` is 180 KB; the subset's `Cargo.toml` depends only on `semio-*` workspace crates. No runtime external dependency was found.

### 6.5 Test hygiene issues

- The Python oracle test returns early (passes) when the script is missing (L203–209), and skips any Rust check id absent from the oracle output (`continue`, L220–223).
- Tests spawn `python3` by name (`Command::new("python3")` in `example_snapshot_validates_against_json_schema`, `python_oracle_matches_within_half_percent` and `de_ground_combo_table_matches_oracle_for_all_combos`). Not portable to Windows (`py`), so it fails the cross-platform rule.
- The schema test (`example_snapshot_validates_against_json_schema`) writes instance files to `std::env::temp_dir()`.

---

## 7. Examples and fixtures

### 7.1 Examples (`ANY/📚️examples/`, mounted from `EDR` L82–89)

| Id | Constructor | Text asset | Label (EN) |
|---|---|---|---|
| `seismic-rc-frame` | `compliant_de_office` | `ANY/🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio` (zone 2, B-R, RC frame, 4 storeys) | Seismic RC frame (DE zone 2, compliant) |
| `seismic-rc-frame-fail` | `noncompliant_de_office` | `ANY/🖼️assets/⚠️seismic-rc-frame-fail/…/🗣️.dsl.semio` | Seismic RC frame (DE zone 2, failing) |
| `seismic-multipart` | `compliant_de_multipart` | `ANY/🖼️assets/🏢️seismic-multipart/…/🗣️.dsl.semio` | Seismic multi-part (DE, compliant) |
| `seismic-multipart-fail` | `noncompliant_de_multipart` | `ANY/🖼️assets/⚠️seismic-multipart-fail/…/🗣️.dsl.semio` | Seismic multi-part (DE, failing) |

- Extra assets for `seismic-rc-frame`: `🎒️.pack.semio` (binary envelope, parsed by a separate codec in the mutation test), `🔧️seismic-rc-frame.op.semio` (`edit reuse` op log, plain text), `📡️seismic-rc-frame.spr.semio` (binary grammar `norm.en1998.spr v1`; not decoded by any test I found).
- `ANY/✏️editor/📚️examples/🎬️demo-session/` (`🎮️.cmd.semio` containing `action=demo`) is a stub: no code consumes the action.
- Stale comment: `ANY/🚪️io/📝️text/📸️snapshot/🦀️.rs` L11–17 describes the rc-frame example as a zone 3 dual system on ground D with EN annex Type 2. The asset is zone 2, moment frame, DE annex. The round-trip test (`🧪️tests/🧩️example`, L28–38) confirms the asset equals `compliant_de_office`, so the comment is wrong.

### 7.2 Fixtures (`ANY/🧪️tests/🧫️fixtures/`, 235 files)

- `🧬️mutations/{kind-dir}/` vectors for each mutation kind: `✅apply/` (`⬅️before`, `🦠️mutation`, `➡️after`, `🎯️outcome`, `🔺️diff`), `⛔dupe/` (refusal rows, with `🔺️diff/🚫️.absent`), `🟰noop/` (update-site), `🔬middle-row/` (remove rows in the middle).
- Root fixtures: `🔢️binary64.json`, `🛂️choices.json`, `🎛️control.json`, `🔣️.json`.
- Fixtures are committed JSON; they are the cross-implementation reference for the mutation harness.

---

## 8. Gaps, stubs, TODOs, hard-coded values, ad hoc shortcuts

### 8.1 Missing scope (per the ticket objective: norm complete, visual, realtime)

1. **No visualisation.** No 3D (members, sections, joints, foundations, load paths, failure modes), no 2D (interaction curves, M–N, stress blocks, spectra, S–N). The only output is text tables and trees (§4.2, §5). The `bar-chart-3` icon has no chart behind it.
2. **Editor covers 12 of ~150 leaves.** Set-field is refused for seismic zone, a_gR, ground, spectrum type, importance, system q0/direction/material/ductility, storey heights, Y stiffness and drift, centres, member ρ/ρ′/ω_wd/section class, bridge period ratio and T, assessment LS and KL, silo/tank/foundation/wall/tower fields (§4.4). Inputs window shows all of them, so the user sees fields that refuse edits.
3. **No modal response spectrum** (EN 1998-1 4.3.3.3) and **no combination of components** (4.3.3.5). Only SRSS in silos.
4. **No capacity design** for any material beyond the q table (§2.11). No member resistance checks (M_Rd, V_Rd of columns, beams, walls, joints).
5. **No liquefaction, kinematic interaction, pile checks** (EN 1998-5).
6. **Bridges, assessments, silos, tanks, towers** are heuristic (see 8.3).
7. **No second-order amplification** (θ ≤ 0.1 / 0.2 / 0.3 rules are reduced to a single 0.30 limit).
8. **No live editor for 3D**; no realtime viewport; the "immediate" part is only "recompute on render".
9. **No localised numeric formatting** beyond `{:.2}`.

### 8.2 Likely defects in the check logic (**verify** against EN text before fixing)

1. **Zero limit passes.** `CheckBuilder::utilization()` returns util 0 and Pass when the limit is 0 (`⚖️compliance/✅️check-results/🦀️.rs` L174–183). Any element with a zero resistance (V_Rd, N_Rd, V_Rd of a tank, ν = 0 limit) passes regardless of demand. No test covers it.
2. **Duplicate check ids across systems.** Member checks and the masonry check are emitted inside the per-system loop, but their ids do not include the system id (`DCM` L24, 46, 72, 97, 128, 155, 178, 200, 227, 249, 289; loop at `SYS` L38–45). Result list shows the same id twice per member for each system, and `applyRemedy` resolves the **first** match (`📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs` L193).
3. **Drift limit direction.** Code checks d_r ≤ ν·θ·h (`SFD` L139, `SCH` L427–429). EN 1998-1 4.4.3.2(2) is written as d_r·ν ≤ θ·h. Also d_r should be q·d_e; the input is used as d_r directly. The isolated-class label (0.010) does not match the EN third class ("no brittle non-structural elements").
4. **Accidental torsion.** δ = 1 + 1.2·e_tot/L_e capped at 3.0 with a 1.5 threshold (`SFD` L30, L40, L46). EN 1998-1 4.3.2 Eq. (4.12) uses δ = 1 + 0.6·x/L_e and no cap. e_tot uses only storey 1 for e0.
5. **Roof φ.** Check requires `correlatedOccupancy = true` on the roof (`SFD` L64–88), but φ_roof = 1.0 is applied regardless of the flag in the mass (`SCH` L187–196, `phi_table_4_2`). The two disagree.
6. **Elevation stiffness rule.** Flags k_hi < 0.70·k_lo (`BRS` L149). A soft-storey rule is usually stated as k_lo < 0.70·k_hi (lower storey soft). Direction **verify**. The check also uses Kx+Ky, not per direction.
7. **Plan regularity.** rx uses k_y in the denominator and ry uses k_x (`SCH` L249–250); min() makes the verdict insensitive, but the reported rx/ry values are swapped relative to EN 4.2.3.2 convention. Kθ is approximated from plan half-dimensions (`SCH` L248).
8. **Given T1 check is tautological** when method = given (`BRS` L47: util = t1Given/t1Given).
9. **Radiation damping check cannot fail** (`PE` L323–338): ξ/1.0 with ξ < 1 always.
10. **Silo wall check compares base shear with an axial resistance** (`PE` L215–221: V vs N_Rd).
11. **Silo impulsive period fixed at 0.3 s** (`PE` L205) regardless of geometry; tower period fixed at 0.5 s (`PE` L366).
12. **Impulsive mass ratio** 1 − 0.436·tanh(H/R) (`SCH` L493–494) is not the Housner closed form; the tank impulsive period (`SCH` L499–501) puts 9.81·H/R inside tanh and is dimensionally suspicious. **verify**.
13. **Foundation bearing reduction** 1 − 1.5·a_g/g with a floor of 0.5 (`SCH` L509–512) has no EN basis in code.
14. **Silo and tank forces:** the silo and tank shears have no behaviour factor for tanks, while silos use q clamped to [1.0, 1.5] (`SCH` L505). No basis given.
15. **Bridge period ratio:** util uses an arbitrary limit 3.0 (`PE` L92); q_isol = max(r², 1) (`SCH` L453–456) has no EN basis.
16. **Assessment factors** NC 1.72, DL 0.5 (`SCH` L475–476) and default CF 1.20 for unknown KL (`SCH` L469) are ad hoc. Supported-building and limit-state checks use fake utilisations (0.5/1.5, 0.5/2.0; `PE` L147, L185).
17. **Member ρ′/ρ ≥ 0.5** applied for all beams regardless of ductility class (`DCM` L174–197); EN requires it for DCH only (**verify**).
18. **Beam ω_wd** is limited to 0.30 with a hard-coded number (`DCM` L206, L212), status is recorded but treated as Fail.
19. **Steel class vs q** uses q thresholds 4.0 and 2.0 (`SCH` L344–346) rather than the EN 6.5 tables keyed by ductility class (**verify**).
20. **Behaviour factor clamp** q = max(1, …) (`SCH` L266). EN lower bound and αu/α1 folding into q0 **verify** (possible double count).
21. **Tower overturning** M_Ed = V·(2/3)·H with fixed Γ = 1.5 (`SCH` L540–543). Not the EN 1998-6 model.
22. **Zero-storey / zero-system fallback.** `unwrap_or(1.5)` for q when no systems (`PE` L259) and `storeys.first()` defaults to "s1" for remedy paths (`DBS` L132).
23. **Permanent G_k remedy** uses 0.85 magic factor and first storey only (`DBS` L132–135).
24. **Annex fallbacks.** Unknown annex string → DE (`SH` L24–27). EN spectrum type is type 1 unless the string contains "2" (`SH` L36). Unknown EN ground → B (`SCH` L584, `SH` L35). Unknown DE combo keys via `parse` return None but the evaluator never calls it. Unknown ψ2 category → 0.3 (`SCH` L183). These are silent defaults that hide input errors.
25. **Composite, timber, masonry** appear only in the q_max table and the masonry simple ratio. Members of these materials get no checks.
26. **Storey shear fallback** uses system V_Rd when the storey value is 0 (`SFD` L93), which treats "unset" as "same as system" silently.

### 8.3 Hard-coded numbers and constants (non-exhaustive)

| Value | Where | Meaning / issue |
|---|---|---|
| g = 9.81 | `SCH` L214, 223, 229, 388, 500–503, 511, 518 | Hard-coded gravity |
| η = 1.0 | `SCH` L358 | Damping 5 % only |
| β = 0.2 | `SCH` L375 | Lower bound on S_d |
| λ = 0.85; n > 2 | `SCH` L399 | EN 4.3.3.2.2 |
| 0.30 r, 0.70, 1.50 | `BRS` L112, L149 | Regularity limits |
| 0.30 P-Δ | `SFD` L173, L179 | θ limit |
| 1.2, 3.0, 1.5 | `SFD` L30, L40, L46 | Torsion (§8.2 item 4) |
| 0.05 minimum T1 | `BRS` L47, `PE` L82 | Floor |
| 0.5, 1.5 utilisation placeholders | many `BRS`, `DBS`, `PE`, `DCM` | Fake utilisations (§8.2) |
| 0.20 beam b_min | `DCM` L69 | Hard-coded |
| 0.25 column b_min | `SCH` L316–317 | DCM/DCH same value |
| 0.30 ω_wd beam | `DCM` L206, L212 | Hard-coded |
| 0.5 ρ′/ρ | `DCM` L176 | Hard-coded |
| 4.0, 2.0 steel thresholds | `SCH` L344–346 | Not EN tables |
| 1.5 default q | `PE` L259 | Silent default |
| 0.3 s silo period | `PE` L205 | Fixed |
| 0.5 s tower period | `PE` L366 | Fixed |
| 1.5 tower Γ | `SCH` L540 | Fixed |
| 2/3 overturning | `SCH` L543 | Fixed |
| 1.72, 0.5 LS factors | `SCH` L475–476 | Ad hoc |
| 1.35, 1.20, 1.00 CF | `SCH` L466–469 | EN 1998-3 table; default 1.20 for unknown |
| 1.25 masonry IC factor | `SCH` L445 | DIN NA simplification |
| 0.436 impulsive ratio | `SCH` L494 | Not Housner |
| 1.84 convective | `SCH` L503 | Housner convective constant |
| 0.574 impulsive | `SCH` L500 | Housner-type constant, dimensionally suspicious |
| 0.85 G_k remedy | `DBS` L134 | Magic |
| 0.02 ~ 0.04 masonry ratio | `SCH` L443 | Simplified |
| 1e-9, 1e-12, 1e-6 tolerances | throughout | Acceptable |

### 8.4 Snapshot defaults and examples

- `compliant_de_office` uses literal masses (`2334780.000000`, etc.) and stiffness series that decrease linearly. These are demonstration values, not derived.
- `t1_method = "ct"` and `ct = 0.075` for all examples, regardless of frame, wall or dual system.
- Every example has one `system_type` per building; the multi-part example's bridge, silo, tank and tower are single records.

### 8.5 Naming, comments, text and encoding

- `EDR` contains mojibake in region tags and doc comments (L1, L4–5, L48–63, L74–76, L179–193, L238, L296–302), and so does `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (L91–245, sparse). Repair the source encoding.
- `ANY/🪟️windows/📥️inputs/🟦️.ts` L1–2 documents the inputs window as raw JSON. Stale.
- Field-meta annex label typo: `EN empfohlenungen` (German).
- Check `title` is set to the same en/de text in some rows (e.g. `lc(&bridge.id, &bridge.id)` for subjects); the "distinct en/de" test does not catch this for subject labels (it excludes numeric-only strings and only tests explanations and remedies).
- `CheckBuilder` clauses: several use `ClauseId::new("EN 1998", "§2", "id")` for integrity checks, which is not a standard clause.
- Clause label "DIN EN 1998-1/NA 9.7.2" (masonry) and "DIN EN 1998-1/NA NA.3.2.1 Table NA.1" are plausible but unverified.

### 8.6 Test and tooling gaps

- Oracle tests can silently pass (§6.5).
- `python3` hard-coded (not Windows-safe).
- No test asserts the numeric value of any EN formula against a published example.
- No test exercises `apply-remedy` on a duplicate check id.
- Zero-limit behaviour is untested.
- Editor coverage test checks field-meta presence, not edit-rule coverage (§4.5).
- `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L148–183 checks field-meta for every leaf of `compliant_de_office` but not for the multi-part snapshot.
- Oracle registration claims "no third-party library" (`🔣️.json`), which is why no third-party numeric validation exists (§6.4).

### 8.7 Other observations

- Artifact definition lists dependencies on `semio-s-artifact-norm-en1990` and `semio-s-artifact-norm-en1997` (`📜️artifact-definition.json` L8–11). Not used by EN 1998 code paths I read; **verify** whether these are needed.
- The shared compliance kernel has `debug_assert!` requiring at least one remedy on every Fail (`✅️check-results/🦀️.rs` L241). Fails produced by `.status(Fail)` without a remedy would panic in debug builds.
- `En1998Snapshot::default()` is a compliant DE office. `reference_birth` returns the same.

---

## 9. Files reviewed

Core: `SNAP`, `SCH`, `INF/🦀️.rs`, `SH`, `FSC`, `BRS`, `DBS`, `SFD`, `DCM`, `PE`, `INF/🧾outline`, `ANY/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`, `ANY/🧬️schema/💡️inferences/🧪️tests/`, `ANY/🧬️schema/🧬️mutations/🦀️.rs`, `ANY/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`, `ANY/🧬️schema/🔺️diff/🦀️.rs`, `EDR`, editor windows, panels and commands, `ANY/🏷️field-meta/🦀️.rs`, `ANY/👁️viewer/` (root, mode, window, tests), `ANY/📚️examples/`, `ANY/🔮️oracles/`, `ANY/🧪️tests/🫨️mutate-en1998-1/`, `ANY/🚪️io/` (root, text snapshot, sqlite), `📜️artifact-definition.json`, `Cargo.toml`, `📦️packages/`.

Shared (outside artifact, read only where the artifact relies on it): `⚖️compliance/✅️check-results/🦀️.rs`, `📇️registry/…/🖥️app-surface` (document-editors, report-tree, catalogue-and-inspection, edit-rules, command-args-and-dispatch, ids-chrome-and-labels).

Not read in full: `ANY/🚪️io/💾️binary/` (binary codec), `ANY/🚪️io/📝️text/🧬️mutations/` (text mutation codec), `ANY/🧬️schema/🛰️.proto`, `🔗️.graphql`, `🔣️.json` and `🟦️.ts` leaves (generated or handcrafted schema leaves, not inspected line by line), fixture contents beyond the file listing.

Not executed: no build, no test run, no oracle run.
