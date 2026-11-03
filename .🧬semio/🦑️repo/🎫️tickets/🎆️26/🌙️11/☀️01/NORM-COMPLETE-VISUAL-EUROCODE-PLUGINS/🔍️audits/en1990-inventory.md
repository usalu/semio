# ⚖️ EN 1990 Inventory

Read-only audit of the EN 1990 artifact as committed. No source file was modified.

## 0. Conventions

- Artifact root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990`. Subset root, used as the base for every relative path below: `R = 🏅️standards/🔖️1/🪆️subsets/✳️any/`. So `💡️inferences/⚖️checks/🦀️.rs` means `R/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`, and `🧬️schema/…` means `R/🧬️schema/…`.
- Line numbers were read from the files at audit time.
- Shared surfaces outside the artifact are named only where the artifact depends on them: `📇️registry/🧬️contract/🖥️app-surface` (the norm app surface) and `⚖️compliance` (the shared compliance crate `semio_s_plugin_norm_compliance`, which defines `CheckResult`, `Remedy`, `NationalAnnex`, `AnnexChoice`).
- Verdicts marked "inferred from code" were not confirmed by running the test suite. This audit did not build or run anything.
- Scope note: the artifact has no 3D and no 2D visual output. Everything below is tables, text and JSON.

---

## 1. Model

### 1.1 Document entities

**`En1990Snapshot`** (`R/🧬️schema/📸️snapshot/🦀️.rs:24-72`, `Default` at 74-126). Persisted basis-of-design subject. Units: forces N, lengths m, frequencies Hz.

| Field (Rust / camelCase) | Type | Notes |
|---|---|---|
| `annex` | `AnnexChoice` (`En`, `De`; defined in compliance crate) | Selects NaEn / NaDe. |
| `project_id` / `projectId` | `String` | Used as the SubjectRef id for DWL and K_FI checks. |
| `structure_kind` / `structureKind` | `String` | Convention: `building`, `road_bridge`, `footbridge`, `rail_bridge`. Not an enum. |
| `altitude_m` / `altitudeM` | `f64` | Only effect: snow ψ category switch above 1000 m (DE). |
| `consequence_class` / `consequenceClass` | `u8` | JSON schema allows 0..255. Checks only handle 1, 2, 3. |
| `reliability_class` / `reliabilityClass` | `u8` | JSON schema allows 0..255. Checks only handle 1, 2, 3. |
| `design_working_life_category` / `designWorkingLifeCategory` | `u8` | JSON schema allows 0..255. Table 2.1 categories are 1..5. |
| `design_working_life_years` / `designWorkingLifeYears` | `f64` | |
| `reference_period_years` / `referencePeriodYears` | `f64` | Drives target β (≥25 a means "50 a" column). |
| `supervision_level` / `supervisionLevel` | `String` | Free string, e.g. `DSL2`. Not an enum. |
| `inspection_level` / `inspectionLevel` | `String` | Free string, e.g. `IL2`. Not an enum. |
| `k_fi_declared` / `kFiDeclared` | `f64` | Declared K_FI, compared to the table value. |
| `beta_computed` / `betaComputed` | `f64` | Input, not computed by any code. Compared to the target β. |
| `permanents` | `Vec<PermanentAction>` | `#[dsl(table)]` |
| `variables` | `Vec<VariableAction>` | `#[dsl(table)]` |
| `accidentals` | `Vec<AccidentalAction>` | `#[dsl(table)]` |
| `seismics` | `Vec<SeismicAction>` | `#[dsl(table)]` |
| `members` | `Vec<Member>` | `#[dsl(table)]` |
| `bridge_sls` / `bridgeSls` | `Vec<BridgeSls>` | `#[dsl(table)]` |
| `effects` | `Vec<MemberEffect>` | `#[dsl(table)]` |

Default values (`R/🧬️schema/📸️snapshot/🦀️.rs:77-124`): annex `De`, projectId `office-cc2`, structureKind `building`, altitude 1200 m, CC2, RC2, DWL category 4, DWL 50 a, T 50 a, DSL2, IL2, K_FI 1.0, β 3.85. Actions: G-sup 80 000 N, G-inf 20 000 N, Q-office 40 000 N (office), Q-wind 20 000 N (wind), Q-snow 15 000 N (snow). One member `beam-B1` (labels "Office beam B1" / "Büroträger B1", rdStr 250 000, rdGeo 250 000, rdEquStab 200 000, rdEquDestab 180 000, rdFat 0, span 6 m, w 0.018 m, limit ratio 250, f 5.5 Hz, fmin 3.0 Hz). Five effects, all influence 1.0. No accidental, seismic or bridge entries. The default is hand-tuned to comply (β 3.85 vs target 3.8).

**Sub-entities** (`R/🧬️schema/📸️snapshot/🦀️.rs`):

- `PermanentAction` (142-146): `id: String`, `kind: String` (convention `g_sup` / `g_inf` / `prestress`), `gk: f64`.
- `VariableAction` (154-158): `id`, `category: String` (selects ψ), `qk: f64`.
- `AccidentalAction` (166-169): `id`, `ad: f64` (A_d already a design value).
- `ImportanceClass` enum (175-184): `I`, `II`, `III`, `IV`. `gamma_i()` (188-195): 0.8, 1.0, 1.2, 1.4 (EN 1998-1 Table 4.3).
- `SeismicAction` (204-208): `id`, `a_ek: f64`, `importance_class: ImportanceClass`. `a_ed()` (212-214) = γ_I · A_Ek.
- `Member` (223-237): `id`, `label_en`, `label_de`, `rd_str`, `rd_geo`, `rd_equ_stab`, `rd_equ_destab`, `rd_fat`, `span`, `deflection_w`, `deflection_limit_ratio`, `vibration_frequency`, `vibration_frequency_min`. 13 fields.
- `BridgeSls` (245-254): `id`, `member_id`, `deck_acceleration`, `deck_acceleration_limit`, `deck_twist`, `deck_twist_limit`, `bridge_deflection`, `bridge_deflection_limit`. 8 fields.
- `MemberEffect` (262-266): `member_id`, `action_id`, `influence: f64` (dimensionless).

### 1.2 Artifact schema

`R/🧬️schema/🦀️.rs`, struct `En1990Artifact` (21-62): the same 20 fields as the snapshot, each `#[state(artifact)]`, same element types. Conversions `to_snapshot` / `from_snapshot` (66-118). Descriptor (123-131) with id `s.norm.en1990`. The Rust file has a `#[cfg_attr(test, …serde…)]` gate, so serde is test-only.

### 1.3 Inference

`R/🧬️schema/💡️inferences/🦀️.rs`:

- `En1990Inference { outline: En1990Outline }` (21-24), `infer` (26-33), `InferenceSpec::fields()` lists 20 reads (35-69).
- `En1990Outline` (`🧾outline/🦀️.rs:31-40`): `section_outline: Vec<String>`, `field_count`, `entry_count`, `check_count`, `pass_count`, `all_pass`, `governing_clause`, `governing_utilization`.
- `compute` (`🧾outline/🦀️.rs:43-67`) runs `evaluate`, counts Pass, takes the non-NotApplicable check with the largest utilization as "governing". `section_outline` is a constant list (`SECTION_FIELDS`, 7-24) that omits `structureKind`, `kFiDeclared` and `bridgeSls`. `entry_count` counts only permanents + variables + members.

### 1.4 Diff

`R/🧬️schema/🔺️diff/🦀️.rs`: `En1990Diff` (111+). Sparse `Option<…>` for every scalar field (`annex`, `project_id`, `structure_kind`, `altitude_m`, `consequence_class`, `reliability_class`, `design_working_life_category`, `design_working_life_years`, `reference_period_years`, `supervision_level`, `inspection_level`, `k_fi_declared`, `beta_computed`, …), plus keyed row deltas per list (e.g. `En1990PermanentDelta`, `En1990EffectEdit` at 300). Row-level helpers: `En1990RowOp` (159), `En1990RowView` with `Insert`, `Remove`, `Replace`, `Malformed` (169-173).

### 1.5 Mutations (the closed vocabulary)

`R/🧬️schema/🧬️mutations/🦀️.rs`, enum `En1990Mutation` (58-89), 30 variants, `KINDS` list (91-122):

| Group | Kinds |
|---|---|
| Scalar setters (`change-*`) | annex, project-id, altitude-m, consequence-class, reliability-class, design-working-life-category, design-working-life-years, reference-period-years, supervision-level, inspection-level, beta-computed |
| Whole-list setters | change-permanents, change-variables, change-accidentals, change-seismics, change-members, change-bridge-sls, change-effects |
| Insert | insert-permanent, insert-variable, insert-accidental, insert-seismic, insert-member, insert-effect |
| Remove | remove-permanent, remove-variable, remove-accidental, remove-seismic, remove-member, remove-effect |

Missing: there is no kind for `structureKind` and none for `kFiDeclared`. Each kind has its own leaf folder with `🦀️.rs`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🔣️.json` and `🟦️.ts`. Leaf files are grouped by module: `🧬️mutations/{⚓️actions, 🌋️accidental, 🎯️reliability, 🏗️members, 🏛️design-basis}`.

Validation: the diffs apply no range checks. `change-consequence-class` (`⚠️change-consequence-class/🔺️diff/🦀️.rs:8-16`) accepts any `u8` and only warns on no-op. Inserts past the end clamp to last with a `mutation.clamped` warning. Duplicate ids raise `mutation.duplicate-id`.

### 1.6 Editor command payloads

`R/✏️editor/🎮️commands/…` (6 variants, enum at `✏️editor/🦀️.rs:65-72`):

- `evaluate`: `Evaluate {}`, no fields.
- `selected-check`: `SetSelectedCheckIndex { index: Option<u32> }`.
- `set-field`: `SetField { path: String, value_json: String }`.
- `insert-item`: `InsertItem { path: String, index: u32, value_json: Option<String> }`.
- `remove-item`: `RemoveItem { path: String, index: u32 }`.
- `apply-remedy`: `ApplyRemedy { check_id: String, remedy_index: u32 }`.

### 1.7 Compliance output types (consumed, from the shared crate)

The checks produce `CheckReport` / `CheckResult` (`id`, `part`, `clause: ClauseId`, `subject: SubjectRef {id, path, label}`, `title`, `status: CheckStatus`, `utilization`, `computed`, `limit`, `annex`, `explanation`, `remedies`). Remedy bounds used: `at_least`, `at_most`, `exactly`, `one_of`. Remedy paths use the form `members[id=…].rdStr` and `bridgeSls[id=…].deckTwist` (see §8).

---

## 2. Checks and verifications

All active checks are emitted by `evaluate` (`R/🧬️schema/💡️inferences/📋️compliance-evaluation/🦀️.rs:6-55`). Document-level checks run once. Member-level checks run for every member, and the ULS/SLS loop runs for every leading variable index (`leading` ∈ 0..n_variables, with `n_leading = max(1, #variables)`, lines 35-47).

Shared helpers used by all member checks:

- `action_set_for_member` (`R/🧬️schema/💡️inferences/🧰️report-helpers/🦀️.rs:63-93`): each action's effect is `Σ influence × magnitude`. Permanents are split by kind into `g_k` (`g_sup`), `g_k_inf` (`g_inf`) and `p_k` (`prestress`). Variables go into `q_k` with `resolve_psi_category` applied. Accidentals give `a_d`. Seismics give `a_ed = Σ influence × γ_I × A_Ek`. Note: `q_k` includes variables with zero influence.
- `k_fi(CC)` (`⚖️checks/🦀️.rs:243-249`): CC1 → 0.9, CC3 → 1.1, else 1.0.
- `resolve_psi_category` (`⚖️checks/🦀️.rs:76-84`): DE and `snow` with altitude > 1000 m → `snow_high`.

### 2.1 Document-level checks (active)

| Check id | Clause | Formula / logic | Location |
|---|---|---|---|
| `en1990.integrity.duplicate.<table>.<id>` (permanents, variables, accidentals, seismics, members, bridgeSls) | EN 1990 §2 "id" | Fails if an id occurs twice. Remedy `one_of` lists other ids. | `R/🧬️schema/💡️inferences/⛓️referential-integrity/🦀️.rs:21-56` |
| `en1990.integrity.effects.<i>.memberId` / `.actionId` | §2 "reference" | Fails on dangling reference. Marks the member as broken. | same, 58-112 |
| `en1990.integrity.bridgeSls.<id>.memberId` | §2 "reference" | Fails on dangling reference. | same, 114-129 |
| `en1990.integrity.member.<id>.unassessable` | §2 "reference" | Emitted for members in the broken set. Their ULS/SLS checks are skipped (`compliance-evaluation/🦀️.rs:29-33`). A member with no effect wiring is also marked broken when effects exist (ref-integrity 131-138). | same, 142-168 |
| `en1990.annex-b.kfi` | EN 1990 Annex B, "K_FI" | Pass iff declared K_FI = table K_FI (within 1e-9). utilization = declared / expected. | `R/🧬️schema/💡️inferences/🧮️partial-factor-sets/🦀️.rs:6-39` |
| `en1990.table-2.1.design-working-life` | EN 1990 §2.3, Table 2.1 | Non-bridge: years in [lo, hi] for the category. Bridge: years = 100 when category is 5, else fail. Fail remedy sets years to hi (or 100 for bridges). utilization = years / limit. | `R/🧬️schema/💡️inferences/🌍️working-life-and-bridge-annexes/🦀️.rs:6-51` |
| `en1990.table-2.1.category` | §2.3, Table 2.1 | Pass iff category in 1..5 and, for bridges, equal to 5. utilization = cat / 5. | same, 53-83 |
| `en1990.annex-b.dsl` | Annex B, "DSL" | Exact string match of `supervisionLevel` against `annex_b_dsl_il_for_rc(RC)` (RC1→DSL1, RC3→DSL3, else DSL2). utilization is 1.0 or 2.0 (sentinel). | same, 86-119 |
| `en1990.annex-b.il` | Annex B, "IL" | Exact string match of `inspectionLevel` (RC1→IL1, RC3→IL3, else IL2). Same sentinel. | same |
| `en1990.annex-c.beta` | EN 1990 Annex C, C.2 | β_computed ≥ β_target(RC, T). Target table in `target_reliability_index_for` (`⚖️checks/🦀️.rs:252-262`): RC1 3.3/4.2, RC2 3.8/4.7, RC3 4.3/5.2 (50 a / <25 a). Fail remedy `at_least`. | `compliance-evaluation/🦀️.rs:22-28`, `⚖️checks/🦀️.rs:715-742` |
| `en1990.members` | §2 | N/A when no members. | `compliance-evaluation/🦀️.rs:9-17` |

### 2.2 Member-level checks (active, per member and per leading index)

| Check id pattern | Clause label | Resistance compared | Formula used | Location |
|---|---|---|---|---|
| `en1990.str.<m>.6.10a.<l>`, `.6.10b.<l>`, `.6.10.<l>` | §6.4, STR A1.2(B) (buildings) / A2.4(B) (bridges) | `rd_str` | 6.10a: `K·γG,sup·(Gk,sup+Pk) + K·γG,inf·Gk,inf + Σ K·γQ·(ψ0 for non-leading)·Qi`. 6.10b: same with ξ on γG,sup. 6.10 = max(6.10a, 6.10b). Factors: γG,sup 1.35, γG,inf 1.0, γQ from set (A1.2(B): 1.5; bridge: traffic table). K=K_FI. | `R/🏗️ultimate-limit-state/🦀️.rs:6-45`; `⚖️checks/🦀️.rs:423-455` |
| `en1990.geo.<m>.6.10a.<l>`, `.6.10b.<l>`, `.6.10.<l>` | §6.4, "GEO A1.2(B)" or "GEO A2.4(B)" label | `rd_geo` | Same STR formulas and factor sets (no separate GEO set for buildings). | `ultimate-limit-state/🦀️.rs:173-208` |
| `en1990.geo.set-c.<m>.<l>` | Annex A2.4(C), "GEO" | `rd_geo` | Bridges only. γG = K·1.00 (both parts); γQ = K·1.30, or bridge-traffic factor for traffic categories (`gamma_q_for_action`). | `partial-factor-sets/🦀️.rs:41-74`; `⚖️checks/🦀️.rs:458-469` |
| `en1990.equ.destab.<m>.<l>` | Table A1.2(A) "EQU" (A1.2(A) buildings / A2.4(A) bridges) | `rd_equ_destab` | `E = K·γG,dst·(Gk,sup+Pk) + Σ K·γQ·ψ·Qi − K·γG,stb·Gk,inf`; γG,dst 1.10 (bridge 1.05), γG,stb 0.90 (bridge 0.95), γQ 1.50 (bridge 1.35). | `ultimate-limit-state/🦀️.rs:47-77`; `⚖️checks/🦀️.rs:312-335, 477-489` |
| `en1990.equ.stab.<m>.<l>` | Table A1.2(A) "EQU-stab" | `rd_equ_stab + K·0.90 (or 0.95)·Gk,inf` | demand = ed_net + stabilising term. Constants 0.90/0.95 hard-coded (`ultimate-limit-state/🦀️.rs:210-244`, lines 213-214). Remedy required = demand − stabilising term. | `ultimate-limit-state/🦀️.rs:210-244` |
| `en1990.sls.<m>.6.14b.<l>` | §6.5 SLS characteristic | **`rd_str`** (see §8) | `G + Q1 + Σψ0·Qi` (plus Gk,inf) | `R/🧬️schema/💡️inferences/📏️serviceability-and-fatigue/🦀️.rs:6-33`; `⚖️checks/🦀️.rs:535-542` |
| `en1990.sls.<m>.6.15b.<l>` | §6.5 SLS frequent | **`rd_str`** | `G + ψ1·Q1 + Σψ2·Qi` | same |
| `en1990.sls.<m>.6.16b` | §6.5 SLS quasi-permanent | **`rd_str`** | `G + Σψ2·Qi`; only one row (no leading loop) | same |
| `en1990.6.11.<m>.<l>` | §6.4, Eq. 6.11 "Accidental" | `rd_str` | `ΣG + A_d + ψ1,1·Q1 + Σψ2·Qi`. N/A when no accidental and A_d = 0. | `ultimate-limit-state/🦀️.rs:79-124`; `⚖️checks/🦀️.rs:515-523` |
| `en1990.6.12b.<m>` | §6.4.3.4, Eq. 6.12b "Seismic" | `rd_str` | `ΣG + A_Ed + Σψ2·Qi`, A_Ed = Σ influence·γ_I·A_Ek. N/A when A_Ed = 0. | `ultimate-limit-state/🦀️.rs:126-171`; `⚖️checks/🦀️.rs:525-532` |
| `en1990.a14.deflection.<m>` | "EN 1990 Annex A1.4", clause "deflection" | `span / deflection_limit_ratio` (fallback `span/250`) | Compares `deflection_w` with the limit. Labelled as a code criterion; it is a project criterion (see §8). | `R/🧬️schema/💡️inferences/📏️serviceability-and-fatigue/🦀️.rs:35-77` |
| `en1990.a14.vibration.<m>` | "EN 1990 Annex A1.4", clause "vibration" | `vibration_frequency_min` | f ≥ f_min. N/A when f_min ≤ 0. | same, 79-122 |
| `en1990.fat.<m>` | "EN 1990 FAT", clause "§2 FAT" | `rd_fat` | Compares the frequent SLS combination with `rd_fat`. N/A when rd_fat ≤ 0. No fatigue damage model. | same, 124-156 |
| `en1990.a2.acceleration.<m>.<sls>` | Annex A2.4 | `deck_acceleration_limit` | Footbridge and rail bridge only. | `R/🧬️schema/💡️inferences/🌍️working-life-and-bridge-annexes/🦀️.rs:121-246` |
| `en1990.a2.twist.<m>.<sls>` | Annex A2.4 | `deck_twist_limit` | Rail bridge only. | same |
| `en1990.a2.deflection.<m>.<sls>` | Annex A2.4 | `bridge_deflection_limit` | Only when limit > 0. | same |
| `en1990.a2.bridge.<m>` (N/A) | Annex A2 | — | Emitted when a bridge member has no `bridgeSls` entry, or the structure is not a bridge. | same, 121-157 |

Also in the ULS path: `push_seismic` and `push_accidental` return N/A checks when the action is absent. `push_uls_str_geo` also emits the three `6.10a / 6.10b / 6.10` rows, each with a remedy when `E_d > R_d`.

### 2.3 Combination and factor tables (active)

Partial-factor sets (`⚖️checks/🦀️.rs:302-335`):

| Set | γG,sup unfav | γG,inf fav | γQ |
|---|---|---|---|
| EQU A1.2(A) | 1.10 | 0.90 | 1.50 |
| STR/GEO A1.2(B) | 1.35 | 1.00 | 1.50 |
| EQU A2.4(A) | 1.05 | 0.95 | 1.35 |
| STR/GEO A2.4(B) | 1.35 | 1.00 | 1.50 (traffic: see below) |
| GEO A2.4(C) | 1.00 | 1.00 | 1.30 |

Bridge traffic γQ (`gamma_q_bridge_traffic`, 338-349): road_traffic 1.35 (DE and EN), footbridge_crowd 1.50, rail_traffic DE 1.40 / EN 1.45, other categories 1.50.

Combination factors ψ0 / ψ1 / ψ2 (`psi_row_de` 31-52, `psi_row_en` 54-73):

| Category | DE ψ0 / ψ1 / ψ2 | EN ψ0 / ψ1 / ψ2 |
|---|---|---|
| residential, office (A, B) | 0.7 / 0.5 / 0.3 | same |
| congregation, retail (C, D) | 0.7 / 0.7 / 0.6 | same |
| storage (E) | 1.0 / 0.9 / 0.8 | same |
| traffic_light (F) | 0.7 / 0.7 / 0.6 | same |
| traffic_heavy (G) | 0.7 / 0.5 / 0.3 | same |
| roof (H) | 0 / 0 / 0 | same |
| snow (≤1000 m) | 0.5 / 0.2 / 0.0 | same |
| snow_high (>1000 m, DE only) | 0.7 / 0.5 / 0.2 | not present, falls to default 0.7 / 0.5 / 0.3 |
| wind | 0.6 / 0.2 / 0.0 | same |
| road_traffic | 0.75 / 0.40 / 0.20 | same |
| footbridge_crowd | 0.40 / 0.40 / 0.0 | same |
| rail_traffic | 0.80 / 0.50 / 0.20 | same |
| temperature | 0.6 / 0.5 / 0.0 | same |
| settlement | 1.0 / 1.0 / 1.0 | same |
| other | 0.8 / 0.7 / 0.5 | not present, falls to default 0.7 / 0.5 / 0.3 |
| unknown category | default 0.7 / 0.5 / 0.3 | default 0.7 / 0.5 / 0.3 |

Note: `psi_row_en` has no `other` arm, so EN silently uses the default row (0.7 / 0.5 / 0.3). The catalogue shows only a subset of rows (`📌️panels/📚️catalogue/🦀️.rs:21-111`).

The ψ rows are `pub`, but `psi_row_de` / `psi_row_en` repeat the same 20-odd arms.

### 2.4 Dead or test-only combination code

The following are defined in `⚖️checks/🦀️.rs` but are not reached from `evaluate` (verified by grep for call sites):

- `check_design_basis` (752-759). It hard-codes `check_reliability_index_for(3.9, …)` (line 757), not the snapshot β.
- `check_reliability_index` (711-713, legacy wrapper). Hard-codes RC = CC and `AnnexChoice::En`.
- `target_reliability_index` (265-267, "legacy helper").
- `check_uls_action` (697-707), `check_combination`, `check_combination_set` (667-694), `append_combination_set` (746-749), `rules_for_situation` (577-588), `combination_value` (563-574), `combination_uls` (492-513), `gamma_for_situation` (371-376), `xi_for_situation` (378-384), `check_seismic_situation` (762-790).
- Generic `combination_6_10` / `combination_6_10a` / `combination_6_10b` (391-414). These omit the stabilising Gk,inf. They are used only by dead code and by tests.
- `combination_str_geo_610a` / `combination_str_geo_610b` non-`_for` wrappers (417-455), `combination_equ_a12a` (472-474).
- `member_index` in `🧰️report-helpers/🦀️.rs:51-53` (unused private helper).

The live check code is the `_for` variants and the `push_*` functions in `ultimate-limit-state`, `serviceability-and-fatigue`, `working-life-and-bridge-annexes`, `partial-factor-sets`, and `compliance-evaluation`.

### 2.5 Summary and governing clause

`En1990Outline::compute` (`🧾outline/🦀️.rs:43-67`): the governing clause is the non-N/A check with the largest `utilization`. The utilizations are heterogeneous dimensionless ratios (force ratios, K_FI ratio, years ratio, category/5, DSL/IL sentinel 2.0, β ratio), so "largest" is not a meaningful comparison.

---

## 3. NA-DE (German national annex) parameters

Defined in `⚖️checks/🦀️.rs:99-149` (`NaDe`). Compared with `NaEn` (151-198).

| Parameter | NA-DE value | EN (NaEn) | Used by live check? | Location |
|---|---|---|---|---|
| γG (`gamma_g`) | 1.35 | 1.35 | **No** on the live path. Live γG comes from the hard-coded `gamma_g_set` table (316, 320). `annex.gamma_g()` is used only by dead code. | 109-111 |
| γQ (`gamma_q`) | 1.5 | 1.5 | **No** on the live path. Live γQ comes from the hard-coded `gamma_q_set` table (327-335). `annex.gamma_q()` is used only by dead code. | 113-115 |
| γM concrete | 1.5 | 1.5 | **No** (never called) | 117-124 |
| γM steel | 1.0 | 1.0 | **No** | same |
| γM timber | 1.3 | 1.3 | **No** | same |
| γM other | 1.0 | 1.0 | **No** | same |
| γR | 1.0 | 1.0 | **No** | 126-128 |
| ξ (permanent) | 0.85 | 0.85 | Yes, 6.10b (`str_geo_610b_for`, 444) | 130-135 |
| ξ (accidental, seismic) | 1.0 | 0.85 for all | Only in dead `xi_for_situation` | 130-135, 182-184 |
| ψ0 / ψ1 / ψ2 | table | table | Yes | 137-147; `psi_row_de` 31-52 |
| Snow altitude rule | `snow_high` above 1000 m | not applied | Yes | `resolve_psi_category` 76-84 |
| γQ bridge traffic | road 1.35, footbridge 1.50, rail 1.40 | road 1.35, footbridge 1.50, rail 1.45 | Yes | `gamma_q_bridge_traffic` 338-349 |
| K_FI (CC1, CC2, CC3) | 0.9, 1.0, 1.1 | same | Yes | `k_fi` 243-249 |
| Target β (RC, T) | Table C.2 | same | Yes | `target_reliability_index_for` 252-262 |
| Annex B DSL/IL per RC | RC1 DSL1/IL1, RC2 DSL2/IL2, RC3 DSL3/IL3 | same | Yes | `annex_b_dsl_il_for_rc` 235-241 |
| Table 2.1 design working life | 1: 10 a; 2: 10–25 a; 3: 15–30 a; 4: 50 a; 5: 100 a | same | Yes | `design_working_life_indicative` 223-232 |
| γ_I (seismic importance) | 0.8, 1.0, 1.2, 1.4 (DE NA Table NA.5 claimed identical) | same | Yes | `ImportanceClass::gamma_i` in snapshot 186-196 |

Citations inside the code (not verified here): "DIN EN 1990/NA NDP A1.3.1(4)" for the max(6.10a, 6.10b) rule (`ultimate-limit-state/🦀️.rs:22`), and "DIN EN 1990/NA Table NA.A.1.1" for ψ (catalogue, `📌️panels/📚️catalogue/🦀️.rs:74`). The NA text itself is not in the repo, so these citations could not be checked.

---

## 4. Editor

Root: `✏️editor/🦀️.rs`. Mode `edit`, layout `row` with 42 % inputs and 58 % results (`✏️editor/🎭️modes/✏️edit/🦀️.rs:13-17`). Default mode `edit`.

### 4.1 Commands

Dispatched through `dispatch_norm_command` in the shared app surface. Tools list in `🦀️.rs:123`.

| Command | Payload | Mutation emitted | Notes | Location |
|---|---|---|---|---|
| `evaluate` | none | None (`Emit::default()`) | Report is recomputed on every read. | `🎮️commands/🧮️evaluate/🦀️.rs:24-26` |
| `setSelectedCheckIndex` | `index: Option<u32>` | None; window-config mutation only | View state (inspection tab). | `🎮️commands/☑️selected-check/🦀️.rs:24-30` |
| `setField` | `path`, `value_json` | Via `EDIT_RULES.resolve` | Only 11 paths are routable (see below). | `🎮️commands/✏️set-field/🦀️.rs:18-20` |
| `insertItem` | `path`, `index`, `value_json` | Via `EDIT_RULES` | 6 collections. | `🎮️commands/➕insert-item/🦀️.rs:19-21` |
| `removeItem` | `path`, `index` | Via `EDIT_RULES` | 6 collections. | `🎮️commands/➖remove-item/🦀️.rs:18-20` |
| `applyRemedy` | `check_id`, `remedy_index` | Via `EDIT_RULES` on the remedy's target path | See §8. | `🎮️commands/🩹apply-remedy/🦀️.rs:18-20` |

`EDIT_RULES` (`🧬️mutations/🧭️edit-rules/🦀️.rs:6-36`):

- `set_field` (7-19): `referencePeriodYears`, `consequenceClass`, `altitudeM`, `annex`, `reliabilityClass`, `projectId`, `supervisionLevel`, `designWorkingLifeCategory`, `designWorkingLifeYears`, `betaComputed`, `inspectionLevel`.
- `insert_item` (20-27): `permanents`, `seismics`, `accidentals`, `effects`, `variables`, `members`.
- `remove_item` (28-35): `effects`, `permanents`, `variables`, `seismics`, `accidentals`, `members`.

Not routable from the editor (no rule in `EDIT_RULES`): `structureKind`, `kFiDeclared`, every field of `members[]`, `effects[]` fields, `permanents[].gk/kind/id`, `variables[].category/qk/id`, `accidentals[].ad`, `seismics[].aEk/importanceClass`, `bridgeSls[]` fields. The matching `change-*` mutation kinds for members, effects and bridge SLS exist (`🧬️mutations/🦀️.rs:70-76`) but no command reaches them.

### 4.2 Field metadata

`🏷️field-meta/🦀️.rs`. Table `TABLE` (66-122) with 67 entries. Each entry gives `label_en`, `label_de`, `unit` (N, m, a, Hz, m/s², rad) and `choices` for enums. Lookup is longest-prefix with `[]` wildcards (`lookup_norm_field_meta`, 125-127). Choice lists: `ANNEX`, `CC`, `RC`, `STRUCTURE_KIND`, `DWL_CATEGORY` (5 entries with year ranges), `DSL`, `IL`, `PERMANENT_KINDS`, `IMPORTANCE` (with γ_I values in labels), `VARIABLE_CATEGORIES` (16 entries).

### 4.3 Panels

Registered in `🦀️.rs:107-133` (render dispatch at 161-172):

| Panel (body key) | Group | Renders |
|---|---|---|
| `norm.en1990.play.artifact` (document panel, `📌️panels/🗿️artifact/🦀️.rs`) | Workbench | One text line from `render_summary`: family label, check count, fail and warning and pass counts, worst utilization, and a ✓/✗ per part. Text only. |
| `norm.en1990.play.catalogue` (`📌️panels/📚️catalogue/🦀️.rs`) | Workbench | Section "Examples" (7 examples) plus three reference tables (windowed): ψ EN (9 rows), ψ DE NA (9 rows), γ_I (4 rows). Table text only. |
| `norm.en1990.play.inspection` (`📌️panels/🔍️inspection/🦀️.rs`) | Details | One check in full as key/value rows: id, part, clause, subject, subject path, status, title, plus explanation and remedies. Chosen by `selected_check_index`, default first. |

### 4.4 Windows

| Window (kind id) | Mode | Renders | Location |
|---|---|---|---|
| Inputs (`norm-en1990-inputs`, body `norm.en1990.play.inputs`) | edit, left 42 % | `render_document_editor`: a schema-driven property tree built from the camelCase value tree of the snapshot. Each leaf gets a field-meta label, unit and (for enums) a choice picker. Not raw JSON. (`📝️document-editors/🦀️.rs:169-178`.) The raw JSON renderer `render_document_json` (181-184) is defined but not called by this artifact. | `🪟️windows/📥️inputs/🦀️.rs:21-23` |
| Results (`norm-en1990-results`, body `norm.en1990.play.results`) | edit, right 58 % | `render_report`: a flat windowed list of every check, grouped by part (verdict row per part), each check row with its remedies. Remedies are buttons that dispatch `applyRemedy` when applicable. Text only. | `🪟️windows/📊️results/🦀️.rs:23-25`; `🌳️report-tree/🦀️.rs:214-234` |

No 3D viewport, no 2D chart, no M-N or interaction curve, no heat map, no stress block, no diagram exists in the editor.

Persistence and presence: `Config = NoConfig`, `Presence = NoPresence`, `Draft = NoDraft`, `Transient = NoTransient`. `✏️editor/🫧️transient/` is empty. `✏️editor/👥️presence/🧬️schema/` has schema leaves but no live presence.

Media: `export_media` / `import_media` delegate to the shared app surface (`✏️editor/🦀️.rs:178-185`). Ports `report:out` and `artifact:out`.

Examples offered in the editor (`✏️editor/🦀️.rs:82-84`): high-consequence-office, road-bridge-compliant, road-bridge-failing, accidental-seismic-compliant, accidental-seismic-failing, fatigue-compliant, fatigue-failing. `default-office` is not in the list.

Mojibake: the header of `✏️editor/🦀️.rs` (lines 1-7) and the constant doc at line 49 are corrupted text ("â", "ðï¸"). Doc-only, but visible.

---

## 5. Viewer

`👁️viewer/🦀️.rs`: `En1990Viewer` implements `ArtifactViewer` only. Mode `view`, default mode `view`, icon `check-circle`, one window kind `report`.

- Window `report` (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:28-33`): recomputes `evaluate(document)` and renders a `TableWindowKit` table (read-only).
- Columns (`🏷️ids-chrome-and-labels/🦀️.rs:49-60`): Part, Clause, Subject, Status, Utilization, Title, Remedy.
- Rows: one per check; remedy column shows text.
- No command, no mutation, no 2D or 3D output. The viewer manifest (`👁️viewer/🦀️.rs:94-103`) declares no actions.

The viewer's `🎚️config`, `👥️presence` and `🦀️.rs` folders hold schema leaves only.

---

## 6. Tests and oracles

### 6.1 Rust tests

119 `#[test]` or `#[async_test]` attributes across 33 files under `R`. Largest:

| File | Count | Notes |
|---|---|---|
| `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | 26 | Default, high-consequence and road-bridge examples, accidental and seismic, fatigue, referential integrity, remedy laws, and the Python oracle check (see 6.2). |
| `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` | 14 | SQLite codec. |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 14 | Command ids, ordering, dispatch. |
| `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | 12 | Combination numbers, K_FI, β target, ψ tables, seismic N/A. |
| `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | 5 | Mutation unit tests. |
| Others (1–4 each, 28 files) | 48 | Panels, windows, viewer, IO, examples, field-meta, inferences outline. |

Fixtures: `🧪️tests/🧫️fixtures/🧬️mutations/…` has 30 kind folders. Each has `✅apply` and `⛔dupe` / `📏clamp` / `🔬middle-row` branches, with `🎯️outcome`, `📸️snapshot/⬅️before`, `➡️after`, `🔺️diff` and `🦠️mutation` JSON files. About 235 JSON fixtures plus 8 `🚫️.absent` markers. Also `🧪️tests/🧫️fixtures/🧫️child-owner-isolation/🔣️.json`.

Example-level tests (`📚️examples/…/🧪️tests/🧩️example/🦀️.rs`): only "primary asset nonempty" and inference determinism. Verdicts for the examples are asserted in the compliance-report tests, not in the example tests.

Important: several tests write temp files under `std::env::temp_dir()` and spawn `python3`. See §6.2 and §8.

### 6.2 Python and other oracles

**`🔮️oracles/🔣️.json`** (registration manifest): one oracle `en1990-1-python-independent`, `ecosystem: python`, `package: ""`, `testOnly: true`, kind `verified-native-second-implementation`. The rationale text says this is not a third-party library, and that a qualifying third-party reference is still owed for the 30-kind vocabulary. One mutation catalog `en1990-1-any` with 30 kinds and 30 vector groups.

**`🧪️tests/⚖️mutate-en1990-1/🐍️.py`** (+ `🥒️.feature`, 3 scenarios, and `🦀️.rs` runner): the oracle imports `import_module("🐍️")` from the norm plugin's engine at `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py` (outside this artifact). It reads the kind list and vectors from `🔮️oracles/🔣️.json`. Stdlib only (`json`, `importlib`, `pathlib`). The Rust side (`⚖️mutate-en1990-1/🦀️.rs:45-192`) drives production dispatch against the committed vectors.

**`🧬️schema/🧪️tests/⚖️compliance/🐍️.py`**: standalone combination oracle. Stdlib only. Defines `combination_6_10a`, `combination_6_10b`, `combination_6_10` (lines 4-19), `psi0_de` (21-22) and `psi0_en` (24-25), `k_fi` (27-28), `beta_target` (30-32). Self-check asserts at 34-41 (237.0, 216.75, K_FI 1.1, β 4.3). It only has 5 ψ entries for DE and 4 for EN, and defaults to 0.7 for the rest. It hard-codes ξ 0.85, γG 1.35, γQ 1.5.

Its Rust consumer is `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:157-215` (`python_oracle_matches_evaluate_json_within_half_percent`). It shells out to `python3` with `SourceFileLoader`, writes a temp JSON, and compares only the governing STR 6.10 value at leading 0 for the default and high-consequence snapshots, with tolerance 0.5 %. It does not cover SLS, EQU, bridges, Annex B, K_FI, working life or deflection.

**Third-party library used: `jsonschema`** (PyPI). Used only in `🚪️io/🧪️tests/🔬️unit/🦀️.rs` `validate_payload`, which runs `python3 -c "import jsonschema …"` against `🧬️schema/📸️snapshot/🔣️.json`. This is the only third-party library in any artifact test. It is not declared in any manifest; it is a runtime-free test-time dependency on a system Python.

Rust dev-dependencies `serde` and `serde_json` (only under `cfg(test)`) are used for JSON round-trips. No runtime third-party crate was found in any `Cargo.toml` under the artifact (path and workspace deps only).

### 6.3 Mutation oracle scope

`🧬️schema/🧪️tests/⚖️compliance/…` is not the mutation oracle. The mutation oracle is `🧪️tests/⚖️mutate-en1990-1/` only.

---

## 7. Examples and fixtures

Example modules (`📚️examples/🦀️.rs:12-28`, 7 modules):

| Module | Structure | Notes |
|---|---|---|
| `high_consequence_office` | building, CC3, RC3, DWL 50 a, DSL/IL per RC | Has `.spr.semio` and `.op.semio` assets under `📚️examples/🏢️high-consequence-office/🖼️assets/`. Expected to fail ≥ 2 checks (test at compliance-report:33-39). |
| `road_bridge_compliant` | `road_bridge` | Expected compliant. |
| `road_bridge_failing` | `rail_bridge` with label "Rail Bridge Failing" (id says road) | Expected to fail multiple checks. Label and id disagree. |
| `accidental_seismic_compliant` | accidental A_d and seismic A_Ek | Expected to activate 6.11 and 6.12b. |
| `accidental_seismic_failing` | same | Expected to fail 6.11 or 6.12b. |
| `fatigue_compliant` | rd_fat > 0 | Expected compliant. |
| `fatigue_failing` | rd_fat > 0 | Expected to fail the FAT check. |

Asset folders under `🖼️assets/` (`R` is not used for these; path is `R/🖼️assets/`): `accidental-seismic-compliant`, `accidental-seismic-failing`, `default-office` (pack + DSL), `fatigue-compliant`, `fatigue-failing`, `high-consequence-office` (pack + DSL), `road-bridge-compliant`, `road-bridge-failing`. Each is a `.dsl.semio` text document with header `semio norm.en1990.dsl v1`. `default-office` is not in the editor catalogue.

Editor demo: `✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio` (command log).

Snapshot fixture in `🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/`: `control.json`, `semantic.json`, `binary64.json`, `invalid-u8.json` and a schema-level `🔣️.json`.

Mutation vectors: see §6.1.

---

## 8. Gaps, stubs, TODOs, hard-coded values, ad hoc shortcuts

### 8.1 Editing reachability

1. `structureKind` and `kFiDeclared` have no mutation kind and no `EDIT_RULES` entry. The editor cannot change building vs bridge type, or the declared K_FI (`🧬️mutations/🦀️.rs:91-122`, `🧭️edit-rules/🦀️.rs:7-19`). Yet the diff, field-meta and checks all depend on them.
2. No `EDIT_RULES` route for member resistances (`rdStr`, `rdGeo`, `rdEquStab`, `rdEquDestab`, `rdFat`), spans, deflection, vibration, nor for `effects[].influence`, `permanents[].gk`, `variables[].qk`, `accidentals[].ad`, `seismics[].aEk`, or any `bridgeSls` field. Only the whole-list `change-*` kinds exist for those, and no command reaches them.
3. Remedies: `applyRemedy` writes to `remedy.target.path`, then routes through `EDIT_RULES.resolve`. Member remedies target `members[id=…].rdStr` (or similar). `EDIT_RULES` has no such path, so resolve fails with "no kind sets '…'" (app-surface `✏️edit-rules/🦀️.rs`, resolve at lines 115-124). Inferred from code, not run. Remedies that target routable paths work: `betaComputed`, `designWorkingLifeYears`, `designWorkingLifeCategory`, `supervisionLevel`, `inspectionLevel`, `consequenceClass`. Remedies on `kFiDeclared` are not routable (see 8.1-1).
4. Generic remedy paths use invented ids: `resistance_remedy` (`⚖️checks/🦀️.rs:616-628`) uses `members[id=governing].rdStr` with empty member id, in dead code. The active `push_*` remedies use real member ids.
5. `evaluate` and `selected-check` mutations are intentionally empty (`evaluate/🦀️.rs:4-7`).

### 8.2 Hard-coded and ad hoc values

6. `check_design_basis` hard-codes β = 3.9 (`⚖️checks/🦀️.rs:757`). Dead code, but a trap.
7. `check_reliability_index` (legacy) hard-codes RC = CC and `AnnexChoice::En` (`⚖️checks/🦀️.rs:711-713`). Dead.
8. EQU stabilising constants 0.90 and 0.95 are hard-coded in `ultimate-limit-state/🦀️.rs:213-214`, not taken from the factor table.
9. Serviceability deflection falls back to `span/250` when `deflection_limit_ratio` ≤ 0 (`serviceability-and-fatigue/🦀️.rs:36`).
10. Working life forces bridges to category 5 / 100 a (`working-life-and-bridge-annexes/🦀️.rs:14-20, 36, 53, 69`). Not what Table 2.1 says for all bridges.
11. Default snapshot values are hand-tuned to comply (β 3.85, specific loads, `office-cc2`). The default test asserts it complies (`compliance-report` line 15-19).
12. Action-set categories are compared as strings: `"g_sup"`, `"g_inf"`, `"prestress"` (`report-helpers/🦀️.rs:67-79`), `structureKind` values, `supervisionLevel`, `inspectionLevel`, `category` strings. Unknown kinds are silently dropped from the action set (permanent kinds not in the three are ignored).
13. Unknown `consequence_class`, `reliability_class` or `design_working_life_category` silently fall back to defaults: K_FI 1.0 (`k_fi`), target β for RC2 (`target_reliability_index_for`), 50 a (`design_working_life_indicative`). No error, no warning. Mutations accept any u8.
14. Annex B DSL/IL check compares strings exactly. `"dsl2"` fails, and the failure message does not say why.

### 8.3 Clause and semantics issues

15. SLS action effects (`en1990.sls.*`, §6.5) are compared against `rd_str`, the ultimate structural resistance, not against a serviceability criterion (`serviceability-and-fatigue/🦀️.rs:7, 11-13`). Likely wrong.
16. The fatigue check (`en1990.fat.*`, clause "§2 FAT") compares the frequent SLS combination with `rd_fat`. No damage-equivalent model, no S-N curve, no reference to EN 1992/1993 fatigue rules. Clause label is invented.
17. Deflection and vibration (`en1990.a14.*`) are labelled as "EN 1990 Annex A1.4". They are project criteria (L/250, f_min) and do not cite a clause. Clause label is invented.
18. Clause mismatch in dead code: `clause_for_rule` (`⚖️checks/🦀️.rs:592-600`) gives SLS 6.14 / 6.16 / 6.17, while `rule_slug` (660-662) and the live SLS check use 6.14b / 6.15b / 6.16b. Dead code and live code disagree.
19. Generic 6.10a/b (`⚖️checks/🦀️.rs:396-414`) omit the stabilising permanent Gk,inf, unlike the live `str_geo_610a_for`. The live version is the only one used by `evaluate`.
20. GEO: buildings use the STR A1.2(B) factors for GEO (`ultimate-limit-state/🦀️.rs:173-208`), and set C is applied to bridges only. Design approach (DA1/DA2/DA3) is not modelled. "GEO A1.2(B)" is a label only.
21. EQU is checked twice with different inequalities: `equ.destab` (E_dst − stabilising ≤ R_destab) and `equ.stab` (E_dst ≤ R_stab + stabilising). Their conventions are not reconciled (`ultimate-limit-state/🦀️.rs:47-77, 210-244`).
22. Seismic 6.12b is compared against `rd_str`, the STR resistance. Whether that is the right capacity for a seismic combination needs a check against EN 1998 and is not verified here. Accidental 6.11 uses ψ1 for the leading variable and ψ2 for the rest (`⚖️checks/🦀️.rs:515-523`), which matches EN 1990 as written.
23. The live 6.10 max applies K_FI to both γG and γQ (`str_geo_610a_for` / `_610b_for`). The clause source for this is not cited in the code, so it is unverified against Annex B.

### 8.4 Dead code and duplication

24. Dead: see §2.4 list. About 20 functions plus `member_index`.
25. Duplicated: `psi_row_de` / `psi_row_en` (near-identical arms); `copy`, `q_force`, `q_dim` defined in both `⚖️checks/🦀️.rs` and `🧰️report-helpers/🦀️.rs`; `is_bridge` vs `is_bridge_kind`; `push_uls_str_geo` and `push_uls_geo` (near-identical bodies); the `if rule == "6.10"` explanation branch.
26. `psi_row_en` lacks `other` and `snow_high`; DE has them. Catalogue and tests reflect this asymmetry.

### 8.5 Schema and typing

27. JSON schema allows `consequenceClass`, `reliabilityClass`, `designWorkingLifeCategory` 0..255 (`📸️snapshot/🔣️.json`). Domain is 1..3 / 1..5 (see 8.2-13).
28. String-typed enums: `structureKind`, `supervisionLevel`, `inspectionLevel`, permanent `kind`, variable `category`. Only field-meta lists them, nothing enforces them in Rust or schema.

### 8.6 Tests and oracles

29. Only one oracle compares a numeric result (governing 6.10, leading 0, two snapshots, 0.5 % tolerance). No independent oracle for SLS, EQU, bridges, A2.4, Annex B, K_FI, working life, deflection, vibration, FAT, seismic or accidental.
30. The combination oracle `🧬️schema/🧪️tests/⚖️compliance/🐍️.py` is not registered in `🔮️oracles/🔣️.json` (only the mutation oracle is). It is a self-written second implementation, not a third-party library.
31. Python tests shell out to `python3` and write to `std::env::temp_dir()`. Hermetic? No. Requires system Python, and `jsonschema` for the IO test. This violates the repo rule for zero-touch cross-platform setup (inferred).
32. Example tests do not assert verdicts. Verdicts are in the compliance-report test file. Example labels: `road-bridge-failing` shows "Rail Bridge Failing".
33. Mutation oracle rationale self-states the third-party requirement is still owed.
34. No EN 1990 test applies a remedy end to end. The only remedy/apply tests are in the shared app surface (`📇️registry/…/🧪️tests/🔬️unit/🦀️.rs`, e.g. `apply_remedy_with_id_path_target_flips_failing_check`).

### 8.7 UI and output

35. No 3D, no 2D diagrams, no interaction curves, no utilisation heat maps, no failure-mode views, no load-path views. Every output is text rows or table cells.
36. Inputs window is a generic property tree, not a direct-manipulation editor. No live re-evaluation feedback in the inputs panel beyond the results panel.
37. Results and inspection windows show remedies as text with an apply button; no per-check visualisation.
38. No progress or cancellation for any operation. `evaluate` is recomputed synchronously on every read. Cheap today, but the rule requires progress and cancellation for expensive ops.
39. Mojibake in doc comments in `✏️editor/🦀️.rs` (lines 1-7, 49).
40. Multi-language: all check titles and explanations are en + de, field-meta has en + de. Catalogue tables are en + de. Good. But the EN ψ table shows "snow" without the ≤1000 m qualifier, while DE does.

### 8.8 Stubs and TODO markers

41. No `TODO`, `FIXME`, `todo!` or `unimplemented!` markers found in the artifact's Rust files. Stubs appear as N/A checks and ad hoc fallbacks (see §8.2 and §8.3).
42. SDK gap notes: `Dialect` (`✏️editor/🦀️.rs:42-45`), `WindowKit` / `TableWindowKit` / `TableView` (`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9-11`) are not in the framework's crate-root re-export list. Workaround: reach them through `app`.
43. `🫧️transient/` is empty. `🎚️config` and `👥️presence` hold schema leaves only (NoConfig / NoPresence).
