# Audit — EN 1997 (`🌍️en1997`)

**Executive summary.** The EN 1997 artifact is a **22-scalar pilot** that runs five utilization checks (spread footing bearing/sliding/settlement, pile axial compression, investigation depth) via Meyerhof/Boussinesq surrogates—not EN 1997-1 Annex D or DIN 1054 workflows. It omits retaining structures, slopes, uplift/heave, eccentricity/overturning, pile tension, GEO-2/GEO-3 design situations, and almost all of EN 1997-2 beyond a depth rule. `evaluate()` never emits remediation; inverse mutations only undo edits. UI shows English-only report tables and raw JSON inputs. Mutation/fixture infrastructure is mature (22 kinds, Python second implementation); compliance math and subject model are not feature-complete.

**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997`  
**Goal association:** `🎯norm` (headless Eurocode geotechnical compliance with DE-NA)  
**Audit date:** 2026-09-26 · read-only

---

## 1. Inventory

### 1.1 Snapshot / document fields (22 flat scalars)

Defined identically in `En1997Snapshot` (`…/🧬️schema/📸️snapshot/🦀️.rs:14–58`) and `En1997Artifact` (`…/🧬️schema/🦀️.rs:14–58`).

| Field | Type | Unit / enum | Default (`snapshot/🦀️.rs:73–96`) |
|-------|------|-------------|----------------------------------|
| `v_ed_kn` | f64 | kN (ULS vertical) | 500 |
| `h_ed_kn` | f64 | kN (ULS horizontal) | 80 |
| `footing_area_m2` | f64 | m² | 2.0 |
| `phi_deg` | f64 | ° (φ′) | 30 |
| `c_kpa` | f64 | kPa | 0 |
| `gamma_kn_m3` | f64 | kN/m³ | 18 |
| `b_m` | f64 | m (footing width) | 2.0 |
| `d_f_m` | f64 | m (foundation depth) | 1.5 |
| `e_s_mpa` | f64 | MPa (stiffness) | 30 000 |
| `nu` | f64 | — | 0.3 |
| `design_approach` | String | `da1str`/`da1geo`/`da2`/`da3` | `"da1str"` |
| `annex` | `AnnexChoice` | `de` / `en` | `De` |
| `settlement_limit_mm` | f64 | mm (SLS limit, user-set) | 25 |
| `n_pile_ed_kn` | f64 | kN (pile ULS axial) | 800 |
| `alpha_s` | f64 | — (shaft factor) | 0.7 |
| `pile_d_m` | f64 | m | 0.6 |
| `q_s_kpa` | f64 | kPa (unit shaft) | 80 |
| `pile_l_m` | f64 | m (shaft in bearing stratum) | 12 |
| `q_b_kpa` | f64 | kPa (unit base) | 2500 |
| `pile_base_area_m2` | f64 | m² | 0.28 |
| `pile_n_profiles` | u32 | count of test profiles | 1 |
| `z_investigated_m` | f64 | m (investigation depth) | 8 |

**No composed children.** `En1997Outline::compute` sets `entry_count = 0` always (`…/💡️inferences/🧾outline/🦀️.rs:47–51`). No soil layers, load cases, pile test results, wall geometry, or slope meshes.

### 1.2 `evaluate()` call graph

```
evaluate(document)                          …/💡️inferences/🦀️.rs:123–147
  └─ parse_design_approach(string)          …/💡️inferences/🦀️.rs:112–118
  └─ check_full_geotechnical(...)             …/💡️inferences/🦀️.rs:77–109
       ├─ check_shallow_foundation(...)       …/🧬️schema/🦀️.rs:530–554
       │    ├─ part_1::design_bearing_capacity_kpa → ultimate_bearing_capacity_kpa (Meyerhof)
       │    ├─ part_1::bearing_resistance_kn
       │    ├─ part_1::sliding_resistance_kn (σ = V_ed/A)
       │    ├─ part_1::elastic_settlement_mm (Boussinesq, i_f=0.88)
       │    ├─ part_1::check_bearing / check_sliding / check_settlement
       ├─ part_1::shaft_resistance_kn, base_resistance_kn
       ├─ part_1::pile_characteristic_resistance_kn (mean=min=same value)
       ├─ part_1::pile_design_resistance_kn
       ├─ part_1::check_pile_axial
       └─ part_2::check_investigation_depth (z ≥ 3b)
```

**Never reached by `evaluate()`:** `part_2::phi_from_cpt_deg`, `part_2::phi_from_spt_deg` (`…/🧬️schema/🦀️.rs:506–514`) — tested only in unit tests.

### 1.3 Mutations (22)

`En1997Mutation` enum `…/🧬️mutations/🦀️.rs:52–74`, catalog `KINDS` lines 82–104. One `change-<field>` per snapshot scalar. Each triad has `↩️inverse/` that **restores the pre-mutation value from base state** (e.g. `change-v-ed-kn/↩️inverse/🦀️.rs:9–10`) — not analytic compliance inversion.

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs window | `…/✏️editor/…/📥️inputs/🦀️.rs:19–20` | `render_document_json` — pretty-printed JSON of all 22 fields |
| Results window | `…/📊️results/🦀️.rs:22–23` | `render_report` — virtualized list: clause, status, u, English message |
| Inspection panel | `…/📌️panels/🔍️inspection/🦀️.rs:19–20` | Single selected check detail (English labels via shared `app_surface`) |
| Viewer report | `…/👁️viewer/…/📊️report/🦀️.rs:30–32` | `TableWindowKit` with `report_table_columns()` — **English headers only** (`🖥️app-surface/🦀️.rs:159–160`) |
| Catalogue panel | `…/📌️panels/📚️catalogue/🦀️.rs:3–5` | Placeholder headline only |

Window labels localized (`Inputs`/`Eingaben`, `Results`/`Ergebnisse`); check content is not.

### 1.5 Examples / assets / tests / oracles

- **Example:** `demo` — DSL `…/🖼️assets/🎬️demo/🗣️.dsl.semio` (matches `En1997Snapshot::default()`).
- **Compliance tests:** `…/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — some numeric (pile shaft 1266.69 kN, ξ factors, DA2* vs DA2); `shallow_foundation_e2e` only `!is_empty()` (line 13).
- **Inference tests:** `…/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — count=5, one utilization bound, no numeric limits on checks 0–2.
- **Mutation cross-lang:** `…/🧪️tests/🌍️mutate-en1997-1/` (Rust + Python), 22 fixture vectors; oracle `…/🔮️oracles/🔣️.json` covers **mutations only**, zero compliance-calculation oracle.
- **27×** `📌️.empty.md` placeholder dirs under editor/viewer taxonomy (config, presence, commands, transient, etc.).

---

## 2. Stub / fake detection

| Issue | Evidence | Impact |
|-------|----------|--------|
| **Meyerhof bearing, not Annex D** | `ultimate_bearing_capacity_kpa` doc + impl `…/🧬️schema/🦀️.rs:406–412` | Spread footing resistance does not follow EN 1997-1 Annex D / DIN 1054 bearing model |
| **Boussinesq settlement surrogate** | `elastic_settlement_mm` `…/🧬️schema/🦀️.rs:438–442`; hardcoded `i_f = 0.88` line 440 | SLS settlement not per §6.6 methods; ULS `V_ed` used as contact stress (line 547) |
| **Pile R_k with mean = min** | `check_full_geotechnical` `…/💡️inferences/🦀️.rs:104–105` passes `r_s_cal` twice | ξ₃/ξ₄ correlation never discriminates profiles; `pile_n_profiles` effect is trivial |
| **DA1 STR material factors ignore EN annex** | `annex_params` `(Da1Str, _)` uses `na_de::gamma_c()` etc. `…/🧬️schema/🦀️.rs:370` | `AnnexChoice::En` + DA1-C1 still applies DE γ_c=1.4, not EN γ_c=1.25 |
| **DA3 same** | line 372 | EN recommended γ_c/γ_φ not applied when `annex=en` |
| **No GEO-2 / GEO-3** | Only `DesignApproach` enum `…/🧬️schema/🦀️.rs:331–337` | German DIN practice (separate GEO combinations) not modeled |
| **Redundant geometry** | `footing_area_m2` and `b_m` independent | User can set A=6.25 m², b=2 m with no consistency check |
| **σ for sliding = V_ed/A** | `check_shallow_foundation` line 547 | No eccentricity; overturning not checked |
| **Dead helpers** | `phi_from_cpt_deg`, `phi_from_spt_deg` never in call graph | EN 1997-2 Annex D correlations are orphan code |
| **Weak e2e test** | `shallow_foundation_e2e` `…/⚖️compliance/🦀️.rs:11–13` | `assert!(!report.is_empty())` only |
| **Placeholder UI** | Catalogue `…/📚️catalogue/🦀️.rs:3–5`; 27× `📌️.empty.md` | No clause browser, empty extension slots |
| **Pilot language borrow** | `🦀️.rs:87–131` imports `semio_s_artifact_norm_en1996` grammars | EN 1997 document codec piggybacks EN 1996 pilot languages (tech debt, not functional stub) |
| **No `todo!()`** | — | Compliance gaps are silent omissions, not marked TODOs |

**Checks that rarely fail with defaults:** settlement u≈0.001 (E_s=30 000 MPa, σ=250 kPa → s≈0.01 mm vs limit 25 mm); investigation depth passes at z=8 m ≥ 3×2 m.

---

## 3. Complete subject definition (norm engineering target)

A feature-complete EN 1997 (+ DIN EN 1997-1/NA, DIN 1054) artifact should model a **geotechnical design package** for a structure or geotechnical works, not isolated scalars.

### 3.1 Top-level

- **Project metadata:** structure ID, design life, consequence class (link EN 1990), national annex (`de`/`en`), selected **design approaches** per design situation (STR: DA1-C1/C2 or DA2*; GEO: DA1-C2, DA2, DA3; DE GEO-2/GEO-3 per NA).
- **Ground model:** stratigraphy `Layer[]` (z_top, z_bot, γ, γ_sat, φ′, c′, cu, E_s, ν, k, OCR); groundwater table(s); partial saturation.
- **Load cases / combinations:** permanent, variable, accidental, seismic (from EN 1991); factored actions per DA and situation (STR vs GEO vs STR/GEO for DA2*).

### 3.2 Spread / shallow foundations (EN 1997-1 §6, Annex D)

Per footing: plan dimensions (B, L), thickness, embedment d_f, structural tie-down, **eccentricity** (e_B, e_L) or column position; design actions V_ed, H_ed, M_ed; bearing resistance per **Annex D** (or approved alternative) with shape/depth/inclination/load factors; **sliding** (§6.5.3) with drained/undrained base; **overturning / eccentricity limits** (§6.5.4); **SLS settlement** (§6.6) with separate service loads, allowable s, tilt; optional **structural strength** of footing (link EN 1992).

### 3.3 Piles (§7)

Per pile/group: layout, D, L, material; **axial compression and tension**; shaft/base resistance from static formula, CPT/SPT correlations, or **load tests** with `n` profiles storing individual R_cal,i (mean and min required); group effects; negative skin friction; cyclic/fatigue where relevant; ξ₃, ξ₄ from Table A.10; γ_b, γ_s per DA.

### 3.4 Retaining structures (§9)

Wall type, geometry, drainage, surcharge; active/passive/at-rest earth pressure (drained/undrained); **sliding, overturning, bearing** at base; structural member checks; groundwater and earthquake (link EN 1998).

### 3.5 Ground anchors, nailed slopes, reinforced fill (§8–9)

Anchor proof loads, bond length; slope circles or block mechanisms; facing stability.

### 3.6 Uplift / hydraulic failure (§10)

Uplift equilibrium; **heave** (σ_eff vs overburden); piping/boiling where groundwater high.

### 3.7 Slope stability (§11)

2D/3D slip surfaces, pore pressures, partial factors on resistance per DA GEO.

### 3.8 Ground investigation (EN 1997-2)

Campaign: boreholes/CPT/SPT/lab schedule; **minimum depth** rules (§2.4.2 and project-specific); spacing; derived parameters (φ′, c′, cu, E_m) via Annex D correlations feeding the ground model — not orphan functions.

**Relationships:** `Project → {Footings[], Piles[], Walls[], Slopes[], InvestigationCampaign} → GroundModel.layers`; each element references load cases and design situations.

---

## 4. Check catalogue

| # | Part | Clause / ref | Verified today | Required inputs | Limit source (DE-NA vs EN) | Failure meaning |
|---|------|--------------|----------------|-----------------|---------------------------|-----------------|
| 1 | 1 | §6.5 bearing ULS | Meyerhof q_ult / γ_R,v | V_ed, A, φ, c, γ, b, d_f, DA, annex | γ_R,v: 1.0 (DA1-C1); DA2*: 1.4 (DE) vs 1.0 (EN); material γ_φ 1.25, γ_c **1.4 DE / 1.25 EN** (DE wrongly applied for EN) | Base pressure exceeds factored resistance |
| 2 | 1 | §6.5.3 sliding ULS | c_d + σ tan φ_d over A | H_ed, V_ed, A, φ, c, DA | γ_R,h: 1.0 DA1-C1; 1.1 DA1-C2/DA2; DA2*: 1.1 | Footing slides on base |
| 3 | 1 | §6.6 settlement SLS | Boussinesq elastic | E_s, ν, b, σ=V_ed/A, user limit | User `settlement_limit_mm` (not code-default) | Settlement exceeds serviceability limit |
| 4 | 1 | §7.6.2 pile compression | R_c,d vs N_ed | N_ed, α_s, D, q_s, L, q_b, A_b, n_profiles, DA | γ_b, γ_s: 1.1 (DA2) or 1.0 (DA1-C1) etc. | Pile overload |
| 5 | 2 | §2.4.2 investigation depth | z ≥ 3b | z_investigated, b | Same rule both annexes (simplified) | Investigation too shallow |

**Not implemented (family brief scope):** Annex D bearing; eccentricity/overturning; retaining sliding/overturning/bearing; pile **tension**; slope stability; uplift/heave; EN 1997-2 test scheduling; DA2* STR+GEO coupled verification; partial factors on water pressures; structural checks.

---

## 5. Remediation strategy (per implemented check)

Core `CheckResult` has no remediation field (`⚖️compliance/🦀️.rs:138–146`). Target: extend report (Wave B) then family-specific analytic inverses.

| Check | On fail, report should say | Target field(s) | Computation sketch |
|-------|---------------------------|-----------------|-------------------|
| Bearing | “Reduce V_ed to ≤ {R_d} kN **or** increase footing area to ≥ {A_req} m² **or** widen b to ≥ {b_req} m **or** improve ground (φ′ ≥ {φ_req}°)” | `v_ed_kn`, `footing_area_m2`, `b_m`, `phi_deg` | Solve `V_ed = A × q_d(φ,c,γ,b,d_f,DA)` for unknown; for φ: invert Meyerhof/Annex D numerically |
| Sliding | “Reduce H_ed to ≤ {R_sl} kN **or** increase vertical load/contact σ” (if beneficial) **or** increase φ′/c′” | `h_ed_kn`, `phi_deg`, `c_kpa` | `H_ed ≤ A × (c_d + σ tan φ_d)/γ_R,h` |
| Settlement | “Increase E_s to ≥ {E_req} MPa **or** widen footing to ≤ {s_lim} mm **or** reduce service pressure to ≤ {q_serv} kPa” | `e_s_mpa`, `b_m`, service load field (missing) | Invert `s = i_f q B (1-ν²)/E_s` — **requires separate SLS load input** |
| Pile axial | “Reduce N_ed to ≤ {R_c,d} kN **or** increase L to ≥ {L_req} m **or** q_s/q_b **or** add test profiles” | `n_pile_ed_kn`, `pile_l_m`, `q_s_kpa`, `q_b_kpa`, `pile_n_profiles` + per-profile R_cal[] | Solve `N_ed = R_b,k/γ_b + R_s,k/γ_s`; vary L in R_s = α π D q_s L |
| Investigation | “Deepen investigation to ≥ {3b} m (currently {z} m)” | `z_investigated_m` | `z_req = 3 × b_m` |

Inverse mutation dirs today only undo (`↩️inverse`); they must grow **forward solvers** or call shared `Remediation` builder.

---

## 6. Report & UX gaps

| Requirement | Status |
|-------------|--------|
| Per-clause pass/fail | ✅ Five checks with `CheckStatus` |
| What complies / not | ⚠️ Status + utilization only; no grouping by element or ULS/SLS |
| How to comply | ❌ No remediation text or subject field pointers |
| Localized en + de | ⚠️ App chrome partially de; messages English (`"bearing resistance ULS"` etc.); table headers English (`app-surface/🦀️.rs:159–160`) |
| Edit all subject fields | ⚠️ All 22 in JSON; no typed forms, units, ranges, or cross-field validation |
| Readable report | ⚠️ List/table of checks; no computed/limit quantity columns in UI (only in `CheckResult` struct, not rendered) |
| Catalogue / clauses | ❌ Placeholder |
| GEO vs STR visibility | ❌ Single DA string; no design-situation dimension |

---

## 7. Target design

### 7.1 Snapshot sketch (Rust-ish)

```rust
pub struct En1997Snapshot {
    pub annex: AnnexChoice,
    pub design_situations: Vec<DesignSituation>, // STR, GEO, STR_GEO (DA2*)
    pub ground: GroundModel,                     // layers + water
    pub footings: Vec<SpreadFooting>,
    pub piles: Vec<Pile>,
    pub retaining_walls: Vec<RetainingWall>,
    pub slopes: Vec<Slope>,
    pub investigation: InvestigationCampaign,
}
// Each SpreadFooting: id, B, L, d_f, actions: ActionsPerSituation, service_actions, s_limit_mm
// Each Pile: …, test_profiles: Vec<PileTestResult { r_cal_kn }>, tension: bool
```

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1997Snapshot) -> CheckReport {
    let mut r = CheckReport::default();
    for situation in applicable_situations(doc) {
        let params = resolve_annex_params(doc.annex, situation);
        for footing in &doc.footings {
            r.extend(check_spread_footing_annex_d(footing, &doc.ground, params, situation));
        }
        for pile in &doc.piles {
            r.extend(check_pile_compression_and_tension(pile, params, situation));
        }
        // walls, slopes, uplift, heave …
    }
    r.extend(check_investigation_campaign(&doc.investigation, &doc));
    r
}
```

### 7.3 Example subjects

**Compliant (spread + pile, DE, DA1-C1):** 2.0×2.0 m pad, d_f=1.5 m, sand φ′=32°, γ=19 kN/m³; V_ed=400 kN, H_ed=50 kN; service V=250 kN, s_lim=25 mm; single bored pile D=0.6 m, L=14 m, q_s=90 kPa, q_b=3000 kPa, N_ed=600 kN, 3 load tests with R_cal = [2100, 1950, 2200] kN; z_inv=10 m.

**Non-compliant (multi-failure):** Same footing but V_ed=1200 kN (bearing u>1), H_ed=200 kN (sliding), E_s=5000 MPa (settlement), z_inv=4 m; pile N_ed=1500 kN with n_profiles=1; report lists ≥4 failures with distinct remediations.

### 7.4 Numeric tests to add

| Test | Expected (illustrative) |
|------|-------------------------|
| Default demo DSL | bearing u≈0.53, sliding u≈0.35, settlement u≈0.001, pile u≈0.57, depth Pass (derive from `check_full_geotechnical` with default snapshot) |
| DA2* vs DA2 EN bearing | q_d_en / q_d_de ≈ 1.4 (existing test `da2_star_de_diverges…` `⚖️compliance/🦀️.rs:75–81`) |
| Pile shaft | R_s = 0.7×π×0.6×80×12 = 1266.69 kN (existing) |
| Investigation fail | z=4, b=2 → Fail (existing) |
| Annex D bearing (new) | Hand calc vs implementation once Annex D replaces Meyerhof |

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / compliance | `…/🧬️schema/🦀️.rs`, `📸️snapshot/🦀️.rs`, `💡️inferences/🦀️.rs`, all facets (graphql/json/proto/ts) |
| Mutations | Regenerate from new shape; add element-scoped mutations |
| Editor | `…/📥️inputs/` structured forms; `📊️results/` remediation column |
| Viewer | `…/📊️report/🦀️.rs` localized columns |
| Examples | New compliant + failing DSL fixtures under `📚️examples/`, `🖼️assets/` |
| Tests | `🧪️tests/⚖️compliance/`, `🔬️compliance-report/`, new compliance oracle in `🔮️oracles/` |
| Core (Wave B) | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` — `Remediation`, `LocalizedText` on checks |

---

## 8. Risks / open questions

1. **Scope boundary:** Is the artifact one geotechnical “project” or one foundation element? Coordinator should decide before schema migration.
2. **Annex D vs DIN 1054:** Germany often uses DIN 1054 procedures alongside EC7 — full DE fidelity may need DIN 1054-specific tables, not only DIN EN 1997-1/NA partial factors.
3. **DA2* coupling:** DA2* requires both STR and GEO verifications with different factor sets — single `DesignApproach::Da2` branch may be insufficient.
4. **EN 1996 grammar dependency:** Replacing pilot languages with native EN 1997 grammars is a separate ticket; blocks clean package boundary (`artifact-definition.json` already depends on `en1996`).
5. **Compliance oracle gap:** No third-party or second-implementation reference for calculation results — only mutations are oracle-backed.
6. **Meyerhof + Boussinesq in production path:** Replacing with code-prescribed methods will change all existing numeric tests and demo “pass” story.
7. **Wave B dependency:** Remediation UX cannot ship family-complete until `CheckResult` carries remediation + subject references.

---

*Auditor: read-only subagent · ticket `26/09/26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS`*
