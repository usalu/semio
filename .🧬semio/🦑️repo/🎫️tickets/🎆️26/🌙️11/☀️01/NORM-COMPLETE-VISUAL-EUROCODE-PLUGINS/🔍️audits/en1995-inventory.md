# 🪵 EN 1995 Inventory

Read-only audit of the EN 1995 artifact as it exists in the working tree. No source file was modified.

Root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995`
Subset: `<root>/🏅️standards/🔖️1/🪆️subsets/✳️any` (abbreviated `S/` below).
Tracked files: 1024 total, 345 under `fixtures/`, 997 non-placeholder. Line numbers refer to the files as read.

## Key findings

- Compliance coverage is a timber-only subset of EN 1995-1-1 (ch. 6, 7, 8 partly), a partial EN 1995-1-2 fire check (bending only) and a heuristic EN 1995-2 footbridge check. Not implemented: tapered/curved/pitched beams (6.4), glued-in rods, CLT-specific checks, block shear, tension-plus-bending (6.2.3), biaxial bending.
- The EN 1990 ULS is only Eq. 6.10. Eq. 6.10a/b and the DIN ξ = 0.85 factor are absent for `De`.
- Several formulas are approximations or differ in form from the standard. The most consequential ones are listed in section 8 (for example `w_fin = w_inst·(1+k_def·ψ2)` applies ψ2 to the permanent part too, `k_v` has a different shape from EN 6.60, the Johansen mode (c) ratio is `t1/t2`, and the floor acceleration/velocity expressions are not EN forms).
- The governing internal forces (V, N, F_c90) are taken from the same combination that maximises |M| (`assessment-vocabulary` `governing_uls`), not from the maximum of each.
- The Python "independent oracle" is standard-library only (`json`, `math`, `sys`). It reuses the same tables and formulas as the Rust code, so it checks implementation parity, not clause interpretation. It does not satisfy the AGENTS.md rule to validate against a third-party library. The only third-party package touched is `jsonschema`, used in one Rust test for schema validation.
- The editor and viewer are table/tree UIs only: no 3D, no 2D diagrams, no utilisation heat maps, no direct manipulation in a viewport.

---

## 1. Model

### 1.1 Snapshot (`S/🧬️schema/📸️snapshot/`)

Rust: `🦀️.rs` (564 lines). TypeScript twin: `🟦️.ts` (103). JSON Schema: `🔣️.json` (1100). Also `🔗️.graphql`, `🛰️.proto`.

**`En1995Snapshot`** (`🦀️.rs:23`, `#[dsl(id="norm.en1995", layout="lines")]`, `#[artifact_schema(id="s.norm.en1995")]`)
- `annex: AnnexChoice` (`En` | `De`), state=artifact
- `members: Vec<TimberMember>` (table), state=artifact
- `connections: Vec<TimberConnection>` (table), state=artifact
- `Default` = `compliant_building_beam()` (`🦀️.rs:35`).

Constructors (scenario fixtures in Rust, `🦀️.rs:41-357`): `compliant_building_beam` (DE, beam-B1 GL28h Floor role, 5 m, 0.20×0.40, conn-C1 bolt GL28h, 8 fasteners), `noncompliant_building` (DE/EN per field, beam-B2 C24 12×24 cm span 5 m fire 3600 s notch 40 mm, col-C1 C24 10×10 SC2, conn-C2 nail x4), `compliant_bridge` (bridge-G1 GL28h 0.24×0.72 span 12 m SC2), `noncompliant_bridge` (bridge-G2 GL24h 0.16×0.40 span 14 m).

**`TimberMember`** (`🦀️.rs:465`; units SI)
| Field | Type | Unit / note |
|---|---|---|
| id, label_en, label_de | String | labels are bilingual, required |
| role | `MemberRole` | beam / column / floor / bridge |
| strength_class | String | tabulated class name, not validated by schema |
| service_class | u8 | 1–3 |
| support | `SupportType` | simplySupported / cantilever / continuousTwoSpan |
| b_m, h_m | f64 | m |
| span_m, support_length_m, bearing_length_m | f64 | m |
| buckling_length_y_m, buckling_length_z_m | f64 | m |
| lateral_restraint_spacing_m | f64 | m |
| notch_depth_m, notch_distance_m | f64 | m |
| m_crit_nm | f64 | N·m, user input (not computed) |
| mass_kg_per_m, mass_kg_per_m2 | f64 | vibration mass |
| damping_xi | f64 | modal ξ |
| fire_duration_s | f64 | s |
| bridge_n_obs, bridge_t_l_years, bridge_beta, bridge_a, bridge_b | f64 | EN 1995-2 Annex A inputs |
| bridge_crowd_per_m2 | f64 | 1/m² pedestrian density |
| actions | `Vec<CharacteristicAction>` | table |

**`CharacteristicAction`** (`🦀️.rs:420`): `id`, `kind` (String: permanent | imposed | snow | wind | accidental; unvalidated), `category` (A–H when imposed), `load_duration` (String), `q_line_n_per_m`, `f_point_n`, `m_k_nm`, `v_k_n`, `n_k_n`, `n_t_k_n`, `f_c90_k_n` (all f64, N or N·m). Internals are derived from q/F and support if q or F is non-zero, otherwise the analysed M/V/N/Nt/Fc90 are used (`load-combinations/🦀️.rs:18-59`).

**`TimberConnection`** (`🦀️.rs:524`): `id`, `label_en`, `label_de`, `fastener_type` (String: nail/screw/bolt/dowel by substring), `strength_class`, `service_class: u8`, `diameter_m`, `number: u32`, `rows: u32`, `spacing_m`, `edge_distance_m`, `end_distance_m`, `t1_m`, `t2_m`, `steel_plate: bool`, `steel_plate_thickness_m`, `shear_planes: u32`, `f_u_k` (Pa), `actions: Vec<ConnectionAction>`.

**`ConnectionAction`** (`🦀️.rs:451`): `id`, `kind`, `load_duration`, `f_k_n`.

**Enums / value types**
- `AnnexChoice` (`En`, `De`) — from `semio_s_plugin_norm_compliance`.
- `MemberRole` (`🦀️.rs:395`): Beam, Column, Floor, Bridge.
- `SupportType` (`🦀️.rs:408`): SimplySupported, Cantilever, ContinuousTwoSpan.
- `TimberProduct` (`🦀️.rs:382`): Solid, Glulam, Lvl, Clt.
- `TimberProperties` (`🦀️.rs:365`): f_m_k, f_t_0_k, f_t_90_k, f_c_0_k, f_c_90_k, f_v_k, e_0_mean, e_0_05, e_90_mean, g_mean, rho_k (all SI Pa / kg·m⁻³), product.
- Editor/checks-side (`S/🧬️schema/⚖️timber/…`): `ServiceClass` (Sc1–Sc3, `material-and-annex-factors:~345`), `LoadDuration` (Permanent, Long, Medium, Short, Instantaneous; ordering used for shortest-duration), `ComboKind` (Uls, SlsCharacteristic, SlsFrequent, SlsQuasiPermanent, Accidental), `LoadCombo` (id, kind, duration, m_ed_nm, v_ed_n, n_ed_n, n_t_ed_n, f_c90_ed_n, q_ed_line, f_ed_point, psi2), `ActionInternals`, `AnnexParams` (wraps `AnnexChoice`), `Grain` (Millimetre, Unit, Count), `Lever<T>` (leaf, label_en/de, unit, grain, kind, get, set), `Head`, `Assessed<T>`.

### 1.2 Mutation vocabulary (`S/🧬️schema/🧬️mutations/`)

`En1995Mutation` (`🦀️.rs:91`) has 66 variants: 58 `Change*` (includes `ChangeAnnex`), 4 `Insert*` (InsertMember, InsertMemberAction, InsertConnection, InsertConnectionAction), 4 `Remove*`. Apply and inverse: `apply_en1995_mutation` (`🦀️.rs:246`), `inverse_en1995_mutation` (`🦀️.rs:253`). Each kind lives in its own taxonomy directory (`↔️change-member-span/`, `➕️insert-member/`, etc.) with `🦀️.rs`, `🔣️.json`, `🧬️schema/🟦️.ts`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`.

Edit rules (`S/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`): 58 `SetFieldRule` (every editable snapshot leaf maps to exactly one change kind), 4 `InsertItemRule` (connections, connections[].actions, members, members[].actions), 4 `RemoveItemRule` (same four paths).

### 1.3 Diff (`S/🧬️schema/🔺️diff/🦀️.rs`, 135 lines)

`En1995Diff` (`🦀️.rs:82`): `annex: Option<AnnexChoice>`, `members: En1995MemberDelta`, `connections: En1995ConnectionDelta`. Delta types are in the same file.

### 1.4 Inference (`S/🧬️schema/💡️inferences/`)

- `En1995Inference` (`🦀️.rs:30-45`): one derived field `outline: En1995Outline`. Reads `members`, `connections`, `annex`.
- `En1995Outline` (`🧾outline/🦀️.rs:12`): `section_outline: Vec<String>` (fixed `["annex","members","connections"]`), `field_count: u32`, `entry_count: u32` (members + connections). It is a document outline, not a compliance inference.
- `evaluate(document) -> CheckReport` (`💡️inferences/🦀️.rs:~60`): calls `checks::timber::evaluate_structure(annex, members, connections)`. This is the single pure entry point used by the editor, the results window and the viewer.

### 1.5 Artifact schema root and descriptors

- `S/🧬️schema/🦀️.rs` (38 lines): mounts `timber`, `mutations`, `diff`, `snapshot`.
- Facet leaves for schema, inference, mutations, diff: `🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`.
- `📜️artifact-definition.json` (repo root): id `s.norm.en1995`, `rust_package` `semio-s-artifact-norm-en1995`.
- `🦀️.rs` at artifact root (145 lines): declaration with capabilities (schema, inference, composer, grammar document/op/diff/pack/spr, codec `semio.norm.en1995/v1`, EN and DE localisation).

---

## 2. Checks and verifications

Entry: `⚖️timber/🏗️structure-and-members/🦀️.rs:89` `evaluate_structure`. For each member `evaluate_member` (`:115`) → `assess_member` (`:138`); for each connection `evaluate_connection` (bridge file `:92`) → `assess_connection` (`:118`). Each check is a `CheckResult` with `id`, `part`, `clause`, `subject`, `computed`, `limit`, `utilization = computed/limit`, status `Pass` iff `utilization ≤ 1` (kernel `⚖️compliance/✅️check-results/🦀️.rs:182`), bilingual copy.

Paths below are relative to `S/🧬️schema/⚖️timber/`. "Member" ids are `en1995.<clause>.<kind>.<memberId>`.

### 2.0 Integrity, applicability and material lookup

| Check id | Clause | Behaviour | Location |
|---|---|---|---|
| `en1995.integrity.duplicate.{members,connections,memberActions,connectionActions}.<id>` | EN 1995-1-1 §1 | Fail when an id repeats; remedy `one_of` free ids | `🏗️…/🦀️.rs:6-86` |
| `en1995.subject.empty` | §1 | NotApplicable when no members | `🏗️…:93-105` |
| `en1995.material.unknown.<id>` | §3.2 | Fail, remedy `one_of` strength-class list | `🏗️…:116-133` |
| `en1995.conn.material.<id>` | §8 | Fail, remedy `one_of` strength-class list | `🌉️…:93-113` |

Strength-class table `properties_for_class` (`🌲️…/🦀️.rs:6-92`): C14, C16, C18, C20, C22, C24, C27, C30, C35, C40, C45, C50; GL20H, GL22H, GL24H, GL24C, GL26H, GL26C, GL28H, GL28C, GL30H, GL30C, GL32H, GL32C; LVL32 (alias LVL-32); CLT100 (alias CLT). Values in MPa / GPa / kg·m⁻³. The CLT100 row has no EN 1995 source (not an EN 338/14080 class); it is a hard-coded set.

### 2.1 EN 1995-1-1 cross-section and member checks (ch. 6)

| Check id | Clause | Formula as coded | Location |
|---|---|---|---|
| `en1995.6.1.6.bending.<id>` | 6.1.6 | σ_m,d = M_Ed/W ≤ f_m,d = k_mod·k_h·k_crit·f_m,k/γ_M; W = b h²/6 | `🏗️…:171-183` (`k_crit` at `🌲️…:312`) |
| `en1995.6.1.7.shear.<id>` | 6.1.7 | τ = 1.5·V_Ed/(b_ef·h·k_v) ≤ f_v,d = k_mod·f_v,k/γ_M; b_ef = k_cr·b | `🏗️…:185-199` |
| `en1995.6.1.2.tension.<id>` | 6.1.2 | σ_t,0,d = N_t/A ≤ k_mod·f_t,0,k/γ_M (only if N_t > 0); no k_h | `🏗️…:201-214` |
| `en1995.6.3.2.compression.<id>` | 6.3.2 | σ_c,0,d = N/A ≤ k_c·k_mod·f_c,0,k/γ_M; k_c = min(k_c,y, k_c,z); k_c from λ_rel with β_c = 0.2 for all products | `🏗️…:216-235`; `k_c_buckling` `🌲️…:341-348` (β_c at `:342`) |
| `en1995.6.2.4.combined.<id>` | 6.2.4 | (σ_c/(k_c f_c,d))² + σ_m/f_m,d ≤ 1 | `🏗️…:238` (labelled 6.2.4; eq. 6.19 without the k_m·σ_m,z term) |
| `en1995.6.1.5.c90.<id>` | 6.1.5 | σ_c,90,d = F/(b·l) ≤ k_c,90·k_mod·f_c,90,k/γ_M; k_c,90 = clamp((l+a1+a2)/l, 1, 1.5), a1=a2=min(30 mm,…) | `🏗️…:251-265`; `k_c_90` `🌲️…:359-365` |
| `en1995.6.3.3` (inside 6.1.6) | 6.3.3 | λ_rel,m = √(W f_m,k/M_crit) with M_crit scaled by (L/ℓ_ef)²; k_crit = 1 (λ≤0.75), 1.56−0.75λ (≤1.4), 1/λ² | `🌲️…:312-339`; used at `🏗️…:171` |

Service class and duration factors (`🌲️…`): `k_mod` table (`:269-283`, SC1/SC2: 0.60/0.70/0.80/0.90/1.10; SC3: 0.50/0.55/0.65/0.70/0.90 for perm/long/med/short/inst; matches EN 1995-1-1 Table 3.1). `k_def` (`:284-290`: 0.60/0.80/2.00). `k_h` (`:292-310`: solid (150/h)^0.2 ≤ 1.3; other products (600/h)^0.1 ≤ 1.1, so LVL and CLT use the glulam branch). `psi_factors` (`:249-267`) EN 1990 Table A1.1, with imposed categories A/B, C/D, E, H and snow/snow_high/wind. Note `snow_high` is a user-selected kind (ψ0 0.7, ψ1 0.5, ψ2 0.2), not an altitude-derived value.

Governing combination (`🧰️…/🦀️.rs:5-15`): the ULS combination with the largest |M_Ed| is used for bending, shear, tension, compression, F_c90 and the duration/k_mod. V, N and F_c90 are taken from that same combination.

### 2.2 EN 1995-1-1 serviceability (ch. 7, SLS deflection)

| Check id | Clause | Formula as coded | Location |
|---|---|---|---|
| `en1995.7.2.winst.<id>` | 7.2 | w_inst = deflection(q_char, F_char) using E_0,mean·I ≤ L/300 (both annexes) | `🏗️…:267-286`; deflection `🧮️…:371-384` |
| `en1995.7.2.wfin.<id>` | 7.2 / 2.2 | w_fin = w_inst·(1 + k_def·ψ2) ≤ L/200 (De) or L/250 (En) | `🏗️…:287-296`, divisors `🌲️…:144-151` |
| `en1995.7.2.wfreq.<id>` | 7.2 | w_freq (ψ1 combo) ≤ L/300 | `🏗️…:300-319` |

The characteristic and frequent combinations are selected by `max |q_ed_line|`, not by maximum deflection. `w_fin` multiplies the whole combined instantaneous deflection by (1 + k_def·ψ2). See section 8.

### 2.3 EN 1995-1-1 floor vibration (§7.3, only `role = floor`)

`assess_floor` (`🏗️…:330-419`). EI = E_0,mean·I; μ = mass per length, else mass per area × b, else ρ_k·A. Support factor (`:340-344`): simply 1.0, cantilever 0.56, two-span 1.5 (ad hoc, not from EN). f₁ = support·π/(2L²)·√(EI/μ) (`:345`).

| Check id | Limit | Notes |
|---|---|---|
| `en1995.7.3.f1.<id>` | f₁ ≥ 8 Hz (`:372`, `f1_lim = 8.0`) | ratio 8/f₁ |
| `en1995.7.3.stiffness.<id>` | w(1 kN) ≤ 1.5 mm (De) / 1.7 mm (En) | `🌲️…:160-165`; w from F=1000 N point load |
| `en1995.7.3.velocity.<id>` (f₁ ≥ 8) | v ≤ b^(f₁ξ−1), b = 100 | v = √(π/(0.8·m·ξ)) (`:355-357`), non-EN form |
| `en1995.7.3.acceleration.<id>` (f₁ < 8) | a ≤ 0.05 m/s² (De) / 0.10 (En) | a = (1/(m*√ξ))·max(f₁/8, 0.1)·0.25 (`:349-353`), non-EN form |

Default ξ = 0.01 when ξ ≤ 0 (`:347`).

### 2.4 EN 1995-1-2 fire (§4.2 residual section, bending only)

`assess_fire` (`🏗️…:421-446`), `en1995.1-2.4.fire.<id>`, clause label "§4.2", applies when `fire_duration_s > 0`.
- d_char = β_n·t, β_n from `🌲️…:192-199` (solid 0.80, glulam 0.70, LVL 0.70, CLT 0.65 mm/min).
- d_ef = d_char + k_0·7 mm, k_0 = min(1, t/1200 s) (`🔩️…/🦀️.rs:11-19`).
- b_fi = b − 2d_ef, h_fi = h − 2d_ef (`🔩️…:21-23`). Four-side charring is assumed.
- W_fi = b_fi·h_fi²/6; M_Rd,fi = k_mod,fi·k_fi·f_m,k/γ_M,fi·W_fi (`🏗️…:429`) with k_fi = 1.25 solid, 1.15 glulam, 1.10 LVL, 1.15 CLT (`🌲️…:177-183`), k_mod,fi = γ_M,fi = 1.0.
- Demand: M_Ed = max |M_ULS| over combinations (`:422`), not an accidental fire combination.

### 2.5 EN 1995-2 footbridge (`MemberRole::Bridge`, `🌉️…/🦀️.rs:9-90`)

| Check id | Clause | Formula as coded |
|---|---|---|
| `en1995.2.a.fatigue.<id>` | Annex A | Δσ = 0.4·M_crowd/W; N_R = 10^(a − b·log10 Δσ[MPa]); k_fat = min(1, (N_R/N)^(1/β)); f_fat,d = k_mod·k_fat·f_m,k/γ_M; N = N_obs·t_L; defaults a=15, b=4, β=5 when inputs are 0 (`:26-35`) |
| `en1995.2.b.avert.<id>` | Annex B | a_vert = (p/1.5)·(5/f₁)·√(0.01/ξ)·0.35 ≤ 0.7 m/s² (`:51`, limit `🌲️…:170`) |
| `en1995.2.b.ahor.<id>` | Annex B | a_hor = 0.3·a_vert ≤ 0.2 m/s² (`:52`, limit `🌲️…:173`) |
| `en1995.2.uls.bending.<id>` | §5 | M = 1.5·M_crowd, M_crowd = q·L²·c, c = 1/8, 1/2 (cantilever), 1/14 (two-span); q = p·800·b (`:5-7`, `:21-22`); σ ≤ k_mod(medium)·k_h·k_crit·f_m,k/γ_M |
| `en1995.2.sls.deflection.<id>` | §7 | w(q) ≤ L/400 (`:80`, hard-coded) |

f₁ = π/(2L²)·√(EI/μ) (default 1.0 Hz if not computable, `:49`). Bridge checks also run the fire check when fire duration > 0.

### 2.6 EN 1995-1-1 dowel-type connections (ch. 8)

`assess_connection` (`🌉️…:118-181`). Governing ULS combination = max |F_ed| (`:120-127`), duration from that combination.

| Check id | Clause | Formula as coded |
|---|---|---|
| `en1995.8.2.2.johansen.<id>` (or `8.2.3` if `steelPlate`) | 8.2.2 / 8.2.3 | F_Ed ≤ F_v,Rd = k_mod·n_ef·n_planes·F_v,Rk/γ_M, γ_M = 1.3 (`:151`); n_per_row = round(number/rows) (`:148`); n_ef = n_row^0.9·(a₁/(13d))^0.25 capped at n_row (`🔩️…:129-137`), times rows |
| `en1995.8.spacing.a1.<id>` | 8.3 | required 5d vs provided spacing_m |
| `en1995.8.spacing.a3t.<id>` | 8.3 | required 10d (nail/screw) or 7d (other) vs end distance |
| `en1995.8.spacing.a4t.<id>` | 8.3 | required 5d (nail/screw) or 3d (other) vs edge distance (`🔩️…:140-150`) |

Connection formulas (`🔩️…`):
- f_h,k (`:25-32`): nail d<8 mm: 0.082·ρ_k·d^−0.3; otherwise 0.082·(1−0.01d)·ρ_k (both in Pa).
- M_y,k (`:35-37`): 0.3·f_u,k·d^2.6 (N·mm, d in mm, converted to N·m).
- Screw axial `F_ax,Rk = 0.2·f_h·d·t1` (`🌉️…:140`), zero for other fasteners; rope term min(F_ax/4, 0.25·base) in each mode.
- Johansen single shear (`:40-74`): modes a, b, c, d, e, f per EN 8.6; mode c uses `r = t1/t2` (`:52`), see section 8.
- Johansen double shear (`:77-100`), steel–timber single shear (`:104-117`, thin 0.4·f_h·t·d branch `:113`, thick `:121-127`, linear interpolation between 0.5d and d).
- Plate selection (`🌉️…:141-146`): steel+2 planes → central plate; steel+1 → steel–timber; no steel+2 planes → double shear; else single shear.

### 2.7 Remedies (`🧰️…/🦀️.rs:183-227` `finish`, `:140-180` `solve`)

For each failing check (utilisation > 1) and each lever in that check's lever set, `solve` bisects the value (grain: mm rounded up for dimensions, 1 unit for counts) by re-evaluating the whole member/connection (`probe`). It emits an `at_least` `Remedy` per lever that alone flips the check. If no lever solves it, a non-applicable remedy says to combine measures. Lever sets (`🧰️…:100-118`):
- bending: H, MCRIT (`BENDING_LEVERS`)
- shear and section: H, B
- tension: B
- compression/bearing: BEARING for c90
- floor: MASS, MASS2, DAMPING (vibration); DEPTH for others
- Johansen: NUMBER, DIAMETER; spacing: SPACING, END, EDGE

Remedy count is capped by the lever list only; `Count` grain writes are marked not applicable as floats (`:203`).

---

## 3. NA-DE (German national annex) parameters

The `AnnexChoice::De` branch (`🌲️…`) changes these values. Everything else is shared between `En` and `De`.

| Parameter | De | En | Location |
|---|---|---|---|
| γ_M solid / glulam / LVL / CLT | 1.3 / 1.3 / 1.3 / 1.3 | 1.3 / 1.25 / 1.2 / 1.3 | `gamma_m` `:120-126` |
| γ_M connection | 1.3 | 1.3 | `gamma_m_connection` `:128` |
| k_cr (cracking, shear) | min(1, 2.5 / f_v,k[MPa]) | 0.67 | `k_cr` `:131-142` |
| w_fin divisor | 200 | 250 | `w_fin_limit_divisor` `:147-151` |
| w_inst divisor | 300 | 300 | `:144-146` |
| floor w(1 kN) limit | 1.5 mm | 1.7 mm | `floor_w_1kn_limit_m` `:160-165` |
| floor acceleration limit | 0.05 m/s² | 0.10 m/s² | `floor_a_limit` `:153-158` |
| floor velocity base b | 100 | 100 | `floor_velocity_b` `:167-169` |
| ψ1 snow > 1000 m | 0.5 (`snow_high`) | snow ψ1 0.2 | `psi_factors` `:249-267` |
| Bridge limits a_vert / a_hor | 0.7 / 0.2 | 0.7 / 0.2 (annex-independent) | `:170-175` |
| k_fi, k_mod,fi, γ_M,fi | same for both | same | `:177-188` |

Annex-related but not actually used as a branch: `k_def` is identical in both annexes. The NA Table NA.1 wording only appears in the `load_duration` doc comment in the snapshot.

Not modelled for `De`: the DIN ξ = 0.85 reduction in Eq. 6.10a/b (see 2.1, combinations), any DE k_c,90 or spacing-table differences, DE snow altitude mapping (the user chooses `snow_high`), and DE-specific fire/bridge values.

Notes:
- `AnnexParams::de()` / `::en()` exist (`:112-117`) but the annex is also a required snapshot field and the parse/editor default is `De`.
- The Python oracle carries the same annex branches (`🔮️…/⚖️compliance/🐍️.py:101-113`, `:402-404`, `:441`, `:476`, `:484`).

---

## 4. Editor (`S/✏️editor/`)

Root: `✏️editor/🦀️.rs` (349 lines). App name "EN 1995", locale-keyed labels in English and German (`LocalizedLabel::native`). Undo/redo keybindings `mod+z`, `mod+shift+z`.

### 4.1 Commands (`🎮️commands/`)

| Action | Kind | Effect | Source |
|---|---|---|---|
| `evaluate` | view (hash) | Fires an empty `Emit` (no mutation). Report is recomputed on every read. | `🧮️evaluate/🦀️.rs:1-33` |
| `setSelectedCheckIndex` | view | Sets the results window config `selected_check_index`, no document change. | `☑️selected-check/🦀️.rs` |
| `setField` (args `path`, `value`) | mutation | Resolves `path` through `EDIT_RULES` (58 set-field rules) to a change kind. | `✏️set-field/🦀️.rs:1-21` |
| `insertItem` (args `path`, `index`, `value`) | mutation | 4 insert rules. | `➕insert-item/🦀️.rs` |
| `removeItem` (args `path`, `index`) | mutation | 4 remove rules. | `➖remove-item/🦀️.rs` |
| `applyRemedy` (args `checkId`, `remedyIndex`) | mutation | Applies a check's remedy `target` and `required` value through `EDIT_RULES`. | `🩹apply-remedy/🦀️.rs:1-21` |

Action labels, arguments and "Migrated" interactive-job classification: `✏️editor/🦀️.rs:~200-260`.

### 4.2 Modes, windows and panels

Mode `edit` (`🎭️modes/✏️edit/🦀️.rs`): default mode, layout `row` 42/58 with panes "Inputs" and "Results".

**Windows**
- **Inputs** (`🪟️windows/📥️inputs/🦀️.rs`, id `norm-en1995-inputs`, body `norm.en1995.play.inputs`): renders the whole snapshot through `render_document_editor` → `render_value_editor`. This is a generic recursive tree editor over the `DslValue` tree, not raw JSON. Each leaf has a row with a label from field-meta:
  - bool → select true/false;
  - number → numeric number input with unit label (`🏷️field-meta`);
  - string with choices in field-meta (annex, role, strength class, service class, support, action kind, category, load duration, fastener) → select;
  - other string → text input, committed on blur;
  - null → "empty" row;
  - arrays → add/remove item rows (`insertItem`/`removeItem`).
  Value changes route to `setField` (`render_value_editor`, `📝️document-editors/🦀️.rs:109-164`).
- **Results** (`🪟️windows/📊️results/🦀️.rs`, id `norm-en1995-results`, body `norm.en1995.play.results`): renders the `CheckReport` through `render_report` (`🌳️report-tree/🦀️.rs:214`) as a windowed tree grouped by part (one verdict row per part), one tree item per check (`build_check_tree_item`), with remedy items that are buttons bound to `applyRemedy` when applicable and a text line otherwise ("Not auto-applicable"). Selecting a check sets the results config `selected_check_index`.

**Panels** (`📌️panels/`)
- **Artifact** (`🗿️artifact/🦀️.rs`, body `norm.en1995.play.artifact`, Workbench): one text line from `render_summary`: family label, total checks, pass/fail counts, worst utilisation, per-part ✓/✗, verdict "complies" / "does not comply".
- **Inspection** (`🔍️inspection/🦀️.rs`, body `norm.en1995.play.inspection`, Details): one selected check via `render_inspection` (`📚️catalogue-and-inspection/🦀️.rs:150`), chosen by the results config index (default first check). Content per the shared renderer, not re-read in full here.
- **Catalogue** (`📚️catalogue/🦀️.rs`, 223 lines): reference tables `strength-classes`, `k-mod`, `fastener-types`, `member-roles`, plus the example list. These are static tables with bilingual headers.

Presence and config are empty (`👥️presence/`, `🎚️config/`, both generated). There are no transient or draft states.

### 4.3 Field metadata (`🏷️field-meta/🦀️.rs`, 169 lines)

Choices: annex (En/De), strength classes (26), service classes (1–3), roles (4), support (3), action kinds (5), categories (A–H plus none), load durations (5), fasteners (4), bool. Units per leaf (m, N·m, kg/m, kg/m², s, 1/a, 1/m²). Covers every snapshot field.

### 4.4 Example catalogue and demo

`examples()` (`📚️examples/🦀️.rs:~45`) returns the four bundled documents (section 7). The editor also contains `📚️examples/🎬️demo-session/` with a 37-byte `.cmd.semio` script.

### 4.5 Visual surface

No 3D viewport, no 2D diagram, no utilisation heat map, no section drawing, no interaction curve, no stress block, no M-N diagram, no direct manipulation. The editor is entirely form/tree/table based.

---

## 5. Viewer (`S/👁️viewer/`)

- `En1995Viewer` (`👁️viewer/🦀️.rs`) implements `ArtifactViewer` only, read-only (`Noop` command, `ViewEmit::default()`).
- Single mode `view` (`🎭️modes/👁️view/🦀️.rs`), full-pane window `Report`.
- Window `report` (`🪟️windows/📊️report/🦀️.rs`, 40 lines): calls `semio_s_artifact_norm_en1995_inferences::evaluate` and renders `TableWindowKit` with columns **Part, Clause, Subject, Status, Utilization, Title, Remedy** (`report_table_columns`, `🏷️ids-chrome-and-labels/🦀️.rs:49-60`). One row per check. The Remedy column shows only the first remedy.
- Icon `check-circle`. No 3D, no chart, no diagram.

---

## 6. Tests and oracles

### 6.1 Rust tests (artifact-owned, outside fixtures): 118 test attributes

| Area | Path | Notes |
|---|---|---|
| Compliance (headline) | `S/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (783 lines, 9 tests) | `python_oracle_matches_rust_utilizations_within_half_percent` (`:333`): bundled examples + 5 connection variants × 2 annexes; identical check-id sets; relative error ≤ 0.5 %. `example_snapshots_validate_against_json_schema` (`:355`): Python `jsonschema`. `perturb_every_editable_leaf_in_committed_examples_changes_a_check` (`:612`): every editable leaf must move a check. `duplicate_member_and_connection_ids_fail_with_one_of` (`:640`), `unknown_strength_class_fails_with_one_of_options` (`:661`), `reference_table_k_mod_cell_equals_evaluated_modification_factor` (`:676`), `lateral_restraint_spacing_changes_bending_utilization` (`:694`), `connection_rows_change_johansen_capacity` (`:705`), `sls_frequent_uses_psi1_and_de_snow_high_diverges` (`:716`). |
| Inference | `S/🧬️schema/💡️inferences/🧪️tests/` (`compliance-report`, `unit`); `🧾outline/🧪️tests/🔬️unit` | Determinism, default, outline checks |
| Mutations | `S/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (204 lines) | Per-kind apply/inverse. |
| Mutations (io) | `S/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/` and `🔬️unit/` | Text codec round-trip |
| Mutations (binary) | `S/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/` | Binary codec |
| Snapshot io | `S/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/` | Text codec |
| Snapshot sqlite | `S/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (129 lines, about 8 test functions) + `🟦️.ts` | Relational round-trip, numeric binary64 preservation, refusal of malformed rows |
| Editor | `S/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (252 lines) and per-node units | Commands, undo/redo, manifest stitch, render of every body, report out |
| Viewer | `S/👁️viewer/🧪️tests/`, `🎭️modes/👁️view/🪟️windows/📊️report/🧪️tests/` | Read-only, table |
| Examples | `S/📚️examples/*/🧪️tests/🧩️example/🦀️.rs` | Assert decode equals the expected snapshot and the expected pass/fail set |
| Mutate-EN1995-1 | `S/🧪️tests/🪵️mutate-en1995-1/🦀️.rs` (744 lines), `🥒️.feature` (202), `🐍️.py` (188) | Differential over 66 `✅apply` vectors + `⛔dupe` refusals + carrier round-trip. The Python side imports the shared norm engine `semio_norm_vocabulary` from `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`. |

Tests that exercise a formula directly (`sls_frequent_uses_psi1…`, `lateral_restraint…`, `connection_rows…`) are narrow. There are no tests that compare a single formula against a hand-computed or textbook value, aside from the oracle `main()` smoke (below).

### 6.2 Oracles

**`S/🔮️oracles/⚖️compliance/🐍️.py`** (606 lines, Python 3, stdlib only: `json`, `math`, `sys`).
- Declares `annex`, members, connections; emits `{"checks":[{"id","utilization"}]}` with the same ids as Rust (`evaluate` at `:580`).
- Same tables (`STRENGTH` `:28-55`), same constants (`K_MOD`, `K_DEF`, `K_FI`, `BETA_N_MM_PER_MIN` `:58-63`), same formulas (`k_v` `:170`, `k_c90` `:162`, `johansen_*` `:489-531`, `psi` `:183`, `floor_checks` `:455`, `bridge_checks` `:347`).
- It emits no `en1995.integrity.duplicate.*` or `en1995.material.*` ids beyond unknown class. It cannot be compared on duplicate-id inputs, and the test corpus contains none.
- `main()` without `--json` runs an assert smoke on k_cr at 1.5·15000 N / (k_cr·0.2·0.3) vs 0.8·4 MPa/1.3, with expected values 0.2438 (De) and 0.2274 (En) at tolerance ±0.01 (`:597-601`). These constants are hard-coded.

**`S/🔮️oracles/🔣️.json`** (2271 lines): oracle registration. Entry `en1995-1-python-independent`, `ecosystem: python`, `package: ""`, `testOnly: true`, `comparisonProfiles: ["ordered-json-v1"]`, license `AGPL-3.0-only`. Its rationale says no third-party library reads or writes `s.norm.en1995`, checked against PyPI, and that nearby packages (`structuralcodes`, `concreteproperties`, `anastruct`) do not speak the format. It also lists `noOracleDecisions: []` and one mutation catalogue.

**Shared norm engine** (`S/🧪️tests/🪵️mutate-en1995-1/🐍️.py` imports it): `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`. Not an EN 1995 oracle; it implements the mutation verbs and addressing.

**Third-party libraries used**
- Python `jsonschema` in `example_snapshots_validate_against_json_schema` (test only, run via `python3 -c`).
- Rust `serde` / `serde_json` under `cfg(test)` only.
- No third-party library implements the EN 1995 checks.

### 6.3 Known oracle gaps
- Not independent of the Rust formulas and tables (identical constants and approximations, see section 8).
- No third-party reference implementation of any EN 1995 formula (no `structuralcodes`-style cross-check, no hand-derived textbook examples).
- The mutation oracle is explicitly "not a third party".

---

## 7. Examples and fixtures

Bundled documents (`S/📚️examples/`, assets in `S/🖼️assets/`):

| Example | Assets | Content | Expected result (from example tests) |
|---|---|---|---|
| `glulam-floor-beam` (`🏠️`) | `🗣️.dsl.semio`, `🎒️.pack.semio` | Compliant DE glulam floor beam GL28h, bolted support connection | complies; includes connection and floor vibration checks |
| `glulam-footbridge` (`🌉️`) | `📡️glulam-footbridge.spr.semio`, `🔧️glulam-footbridge.op.semio`, `🗣️.dsl.semio`, `🎒️.pack.semio` | Compliant EN 1995-2 simply supported bridge girder G1 (GL28h, 12 m, SC2) | complies; every bridge id prefix present |
| `multi-fail-timber` (`❌️`) | `🗣️`, `🎒` | Non-compliant building: beam C24 with notch and 3600 s fire, column C24 SC2, nailed connection | ≥ 2 failing checks, each with remedy, specific ids asserted |
| `overloaded-footbridge` (`⚠️`) | `🗣️`, `🎒` | Non-compliant bridge GL24h, dense crowd, low damping | fails bridge checks, `avert` failing, each with remedy |
| `demo-session` (editor) | `🎮️.cmd.semio` | Editor command script (37 bytes) | non-empty only |

Fixtures:
- `S/🧪️tests/🧫️fixtures/🧬️mutations/`: 66 vector directories, each with `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`, `🔺️diff/🔣️.json`, `🦠️mutation/🔣️.json`, `🎯️outcome/🔣️.json` (≈330 `✅apply` files; the `⛔dupe` rows reuse the insert vectors).
- `S/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/`: `🎛️control.json`, `🎛️semantic.json`, `🔢️binary64.json`, `🔣️.json`, `🛂️choices.json`.

---

## 8. Obvious gaps, stubs, TODOs, hard-coded values and shortcuts

### 8.1 Stubs and no-ops
- `evaluate` and `selected-check` emit no document mutation by design. `evaluate` exists only to refresh the view.
- The `Warning` status is never produced for EN 1995: `warn_above` is not set anywhere in `🏗️`, `🌉️`, `🧰️`, `🩹…` (only the compliance kernel's own tests use it).
- The viewer `Noop` command is a placeholder.
- The `🧾outline` inference is a document outline (counts), not a verification.
- No TODO / FIXME / `unimplemented!` markers exist in the artifact. The only "SDK GAP" notes are at `✏️editor/🦀️.rs:42`, `👁️viewer/🦀️.rs:20` and `👁️viewer/🪟️windows/📊️report/🦀️.rs:9`.
- Stale comments referring to removed modules exist: `🚪️io/🦀️.rs:1-7` mentions `⚙️engine::register`; `🦀️.rs` root and `✏️editor/🦀️.rs` mention "pre-migration" enums.

### 8.2 Missing clauses and scope
- Tapered, curved and pitched beams (EN 1995-1-1 §6.4): absent.
- Notches (§6.5): only the shear factor k_v (`🌲️…:367-382`), no separate notch check. The k_v shape differs from EN 6.60 (uses `(1−α) − (x/h)²` and `√(α(1−α))` rather than the √(1/α − α²) term with the 0.8 factor).
- Tension + bending (§6.2.3): absent. Biaxial bending (k_m) and weak-axis bending: absent.
- Glued-in rods, block shear, group effects, steel-to-timber with multiple rows, robustness (§10), built-up members (§9): absent.
- CLT: only the CLT100 property row and the glulam depth factor. No rolling-shear or CLT-specific check.
- Fire (EN 1995-1-2): residual bending only. No compression, shear or connection checks, no 3-side exposure, and the fire demand is the ULS max |M| rather than an accidental fire combination.
- Bridges (EN 1995-2): fatigue and comfort are heuristic (below). No shear, compression or support checks.
- Floor vibration: formulas for velocity and acceleration are not the EN forms (see 2.3).
- ULS: only Eq. 6.10 (`enumerate_combos`, `🧮️…:181-204`). No Eq. 6.10a/b, no ξ factor for `De`.
- Connections: no withdrawal (`F_ax,Rk`) check for nails or bolts (screw rope term is ad hoc), no spacing for α ≠ 0.

### 8.3 Formula approximations and likely deviations (to verify against the source)
- `w_fin = w_inst·(1+k_def·ψ2)` (`🏗️…:274`) applies the variable-load ψ2 to the whole deflection, including permanent G. EN 2.3.2.2 separates G (1+k_def) and Q (1+ψ2·k_def).
- Governing forces: V, N, F_c90 come from the moment-governing combination (`🧰️…:5-15`), not from their own maxima.
- Johansen mode (c), `🔩️…:52-56`: ratio `r = t1/t2`; the EN 8.6(c) expression uses `t2/t1`. Verify. Same ratio in the Python oracle (`🐍️.py:492`).
- Thin steel plate branch `🔩️…:113`: `0.4·f_h·t·d` coefficient; verify against EN 8.2.3.
- Spacing minima `🔩️…:140-150`: a single row per fastener type, α = 0 only. For nails, EN Table 8.2 gives a₃,t = (10+5 cos α)·d = 15d at α = 0; code uses 10d. No predrilling, density or 80 mm bolt minimum. Verify all four tables.
- `k_c,90` (`🌲️…:359-365`): effective-length ratio with 30 mm a₁/a₂, clamped to [1, 1.5]. EN 6.1.5 uses a product- and geometry-dependent factor (1.25/1.5/1.75 forms). Not implemented as coded.
- `β_c = 0.2` fixed for all products (`🌲️…:342`). EN 6.3.2 uses 0.1 for glulam.
- Continuous two-span internal forces (`🧮️…:29-31`, `:22-23` of oracle): M = qL²/14 (sagging only), V = 0.6·qL; EN coefficients differ (0.625qL, hogging 0.125qL²). Deflection constants `/185`, `/100` are used as-is.
- Charring rates β_n (`🌲️…:192-209`): solid 0.80, glulam 0.70, LVL 0.70, CLT 0.65 (mm/min). CLT has no table row; LVL value should be checked against EN 1995-1-2 Table 3.1. β_0 (`:201`) is defined but has no caller in the checks.
- `k_h` for LVL and CLT uses the glulam branch (`🌲️…:292-310`).
- Effective M_crit (`🌲️…:331-339`): user-entered M_crit scaled by (L/ℓ_ef)². It is not computed from section and restraint geometry as in EN 6.3.3.
- Fire residual depth: four-side charring assumed (`🔩️…:21-23`) and k_0·7 mm (`:13-18`) fixed.
- Fire k_fi/k_mod,fi/γ_M,fi: k_mod,fi = γ_M,fi = 1 hard-coded (`🌲️…:185-189`).
- Two floor support factors (0.56 cantilever, 1.5 two-span, `🏗️…:340-344`) are not from EN 1995-1-1 §7.3.
- Floor velocity `√(π/(0.8·m·ξ))` and acceleration `(1/(m√ξ))·max(f₁/8,0.1)·0.25` (`🏗️…:349-357`) are non-standard expressions.
- Bridge crowd: q = p·800·b (magic 800, `🌉️…:5-7`); a_vert constants 1.5, 5, 0.35, 0.01 (`:51`); a_hor = 0.3·a_vert (`:52`); bridge defaults a=15, b=4, β=5 (`:30-33`); N_R exponent convention; L/400 (`:80`). Heuristic, not Annex A/B formulas.
- Bridge ULS load factor 1.5 (`:22`) rather than EN 1990 γ_Q or the EN 1995-2 combination.
- Connection γ_M = 1.3 for all fasteners (`🌉️…:137`).
- Mass default when absent: μ = ρ_k·A (`🏗️…:338`, `🌉️…:48`). Uses characteristic density as mean density.
- Strength table CLT100 and the oracle's CLT row: no EN source.
- Strength-class and k_mod table rows in the catalogue are duplicated by hand in `📌️panels/📚️catalogue/🦀️.rs` and field-meta, so they can drift.

### 8.4 Hard-coded values and constants
- Load factors γ_G = 1.35, γ_Q = 1.5 (`🧮️…:160-161`, `:326-327`).
- ψ defaults when kind is unknown: (0.7, 0.5, 0.3) (`🌲️…:267`).
- Default ξ = 0.01 (`🏗️…:347`, `🌉️…:50`), default f₁ = 1.0 Hz (`🌉️…:49`).
- Default M_crit floor `max(1.0)` and span floor `1e-6` throughout.
- Floor f₁ threshold 8.0 Hz (`🏗️…:372`), b = 100 (`🌲️…:167`).
- Limits: L/300 (`🌲️…:144`, `:307`), L/200 and L/250 (`:147`), L/400 bridge (`🌉️…:80`), bridge a_vert 0.7 and a_hor 0.2 (`🌲️…:170-175`).
- Screw axial 0.2 (`🌉️…:140`). Rope cap 0.25 (`🔩️…:72`, `:98`, `:112`, `:126`).
- Johansen constants 1.05, 1.15, 2.3 and M_y coefficient 0.3, exponent 2.6 (`🔩️…`, EN form).
- k_v: k_n = 5, 1.1 (`🌲️…:378-383`), α clamp 0.05–1.
- Spacing minima fixed per type (see 8.3).
- n_ef exponent 0.9 and 13d (`🔩️…:135`).
- Johansen connection `number / rows` integer rounding (`🌉️…:148`).
- Bridge fatigue defaults and k_fat cap 1.0.
- `grain` rounding for remedies: 1 mm, 1e-9 tolerance (`🧰️…:41-44`).
- Combo ψ defaults for connections: `psi_factors(kind, "")` (category ignored) (`🧮️…:343`).

### 8.5 Ad hoc shortcuts and structural issues
- Snapshot strings for `kind`, `category`, `loadDuration`, `fastenerType`, `strengthClass` are not enumerated in the schema; invalid values are only caught by parse fallbacks (`LoadDuration` → Medium, `ServiceClass` → Sc1 for unknown values, `fastener` unknown → non-nail formulas).
- Snapshot scalars such as `serviceClass` are `u8`, with range enforced only in the TS/JSON bounds.
- Unknown strength class yields a Fail with a remedy, not an error; this is consistent but means every typo is a compliance failure.
- Member role `Floor` gates vibration; `Bridge` gates the entire EN 1995-2 path; `Column` gates N only. A beam with `n_k > 0` gets a compression check, but a column has no slenderness check beyond λ_rel.
- `CharacteristicAction` analysed M/V/N values are ignored whenever q or F is non-zero (`🧮️…:22-23`).
- `TimberProperties` fields f_t_90_k, g_mean, e_90_mean are stored but unused in checks.
- Shared `NormHost` re-evaluates the report on every read, so cost is linear in member and connection count with no caching.
- Mutation rules are per kind, one directory each, with 58 set-field rules that are kept in sync by a test (`kinds_match_the_enum_and_the_catalog`) but duplicated across `🧭️edit-rules`, `🔣️.json` and `🟦️.ts` facets.

### 8.6 Test gaps
- No test checks a formula against a textbook or hand-computed number.
- No test runs the Python oracle with a duplicate-id or unknown-class input.
- The oracle's own smoke test checks only k_cr (`main`).
- No test of the rope effect, the 25 % cap, or the screw axial term.
- No third-party oracle (see 6.3).
