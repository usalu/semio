# Audit — EN 1994 (`🧩️en1994`)

**Executive summary.** The EN 1994 artifact is a **22-scalar compliance worksheet**, not a composite structure model. `evaluate()` runs seven utilization checks stitched from partial helpers, but **~80% of the claimed family scope is absent** (effective width, LTB, composite columns, slab shear, transverse reinforcement, most of EN 1994-1-2, most of EN 1994-2). Several checks compare **pre-supplied resistances** (`m_pl_rd`, `v_l_rd`, `eta`) instead of deriving them from sections, connectors, and geometry. Two checks are **physically wrong surrogates** (bridge fatigue: stress vs detail-category integer; fire: hardcoded mm table with swapped computed/limit roles). DE national annex differs from EN only in name — γ factors are identical and fire is hard-coded `AnnexChoice::De`. The report has **no remediation**, **no subject-field binding**, and **English-only** UI strings. Default snapshot and the sole registered example **pass all seven checks**; no committed failing example exists. Mutation infrastructure (22 kinds, Python second implementation) is mature; compliance oracles are **mutation-only**.

Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994`  
Subset path: `🏅️standards/🔖️1/🪆️subsets/✳️any/`

---

## 1. Inventory

### 1.1 Snapshot / document fields (`En1994Snapshot`)

Defined at `🧬️schema/📸️snapshot/🦀️.rs:14–71` (mirrored in `🧬️schema/🦀️.rs:12–57`).

| Field | Type | Unit (DSL) | Role in checks |
|-------|------|------------|----------------|
| `annex` | `AnnexChoice` | — | γ_V for stud P_Rd; other checks ignore annex choice |
| `m_ed_knm` | f64 | kNm (implicit) | Bending demand |
| `v_ed_kn` | f64 | kN | Longitudinal shear demand |
| `m_pla` | f64 | kNm | Steel plastic moment (input, not computed) |
| `m_pl_rd` | f64 | kNm | Full composite plastic moment (input, not computed) |
| `eta` | f64 | — | Shear-connection degree (input, not computed from stud count) |
| `v_l_rd` | f64 | kN | Longitudinal shear resistance (input, not computed) |
| `insulation_thickness_mm` | f64 | mm | Fire insulation provided |
| `fire_rating` | String | — | R30/R60/R90/R120 (parsed loosely) |
| `deck_type` | String | — | `trapezoidal` / `re-entrant` |
| `delta_sigma_mpa` | f64 | MPa | Bridge steel stress range (surrogate fatigue) |
| `fatigue_detail` | String | — | Maps to integer “category” |
| `d_mm` | f64 | mm | Stud diameter |
| `h_sc_mm` | f64 | mm | Stud height |
| `f_ck_mpa` | f64 | MPa | Concrete cylinder strength |
| `f_u_mpa` | f64 | MPa | Stud ultimate strength |
| `e_cm_mpa` | f64 | MPa | Concrete secant modulus |
| `v_ed_per_stud_kn` | f64 | kN | Per-stud shear demand |
| `span_m` | f64 | m | Span (η_min only) |
| `f_y_mpa` | f64 | MPa | Steel yield (η_min only) |
| `n_cycles_stud` | f64 | — | Stud fatigue cycles |
| `delta_tau_stud_mpa` | f64 | MPa | Stud shear stress range |

**Default values** (`📸️snapshot/🦀️.rs:81–107`): all seven checks pass (computed offline: worst u ≈ 0.96 bending).

**No composed children** — flat artifact; `En1994Outline.entry_count` is always 0 (`💡️inferences/🧾outline/🦀️.rs:50`).

### 1.2 Mutations (22)

Catalog: `🧬️schema/🧬️mutations/🦀️.rs:85–108`, oracle `🔮️oracles/🔣️.json:311–334`.

One `change-<field>` per scalar. `↩️inverse/` modules are **undo-only** (restore base value), e.g. `🌀️change-m-ed-knm/↩️inverse/🦀️.rs:7–8` — not compliance inversion.

### 1.3 `evaluate()` call graph

Entry: `💡️inferences/🦀️.rs:121–145` → `check_full_composite` (`💡️inferences/🦀️.rs:86–117`).

```
evaluate(snapshot)
└─ check_full_composite(...)
   ├─ check_composite_beam (🧬️schema/🦀️.rs:463–468)
   │  ├─ plastic_moment_partial_knm (🦀️.rs:321–323) → m_rd
   │  ├─ check_composite_bending (🦀️.rs:362–364)
   │  └─ check_longitudinal_shear (🦀️.rs:366–368)
   ├─ check_stud_resistance (🦀️.rs:370–373)
   │  └─ connector_resistance_kn (🦀️.rs:348–354)
   │     └─ stud_alpha (🦀️.rs:338–345), AnnexParams::for_annex (🦀️.rs:297–302)
   ├─ check_shear_connection_degree (🦀️.rs:375–378)
   │  └─ min_shear_connection_degree (🦀️.rs:357–360)
   ├─ check_fire_composite (🦀️.rs:410–413)
   │  └─ insulation_thickness_mm (🦀️.rs:396–408), parse_fire_rating (💡️inferences/🦀️.rs:75–82)
   ├─ inline bridge fatigue (💡️inferences/🦀️.rs:114–115)
   │  └─ bridge_fatigue_category (🦀️.rs:422–428)
   └─ check_stud_fatigue (🦀️.rs:445–448)
      └─ stud_fatigue_resistance_mpa (🦀️.rs:440–442)
```

**Helpers never reached by `evaluate()`:**

| Function | Location | Note |
|----------|----------|------|
| `effective_width_mm` | `🦀️.rs:326–330` | Tested (`⚖️compliance/🦀️.rs:12–15`), not in report |
| `longitudinal_shear_kn` | `🦀️.rs:333–335` | Tested (`⚖️compliance/🦀️.rs:26–29`), V_L,Rd is input |
| `full_plastic_moment_knm` | `🦀️.rs:311–313` | Dead code |
| `shear_connection_degree` | `🦀️.rs:316–318` | η is input scalar |
| `check_bridge_composite` | `🦀️.rs:451–458` | Superseded by inline duplicate in `check_full_composite` |

Editor binding: `✏️editor/🦀️.rs:199–201` delegates to same `inferences::evaluate`.

### 1.4 Editor / viewer

| Surface | File | Capability |
|---------|------|------------|
| Inputs window | `✏️editor/…/📥️inputs/🦀️.rs:19–20` | Pretty-printed **JSON blob** via `render_document_json` — no typed field editors |
| Results window | `✏️editor/…/📊️results/🦀️.rs:22–23` | `render_report` — clause, status, u, English message |
| Inspection panel | `✏️editor/📌️panels/🔍️inspection/🦀️.rs:19–20` | Single check detail, English labels |
| Document panel | `✏️editor/📌️panels/🗿️artifact/🦀️.rs:18–19` | Count + worst u + pass flag (English) |
| Catalogue panel | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:3–5,20–22` | **Placeholder** headline |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs:30–32` | Table: Clause/Status/Utilization/Message (English) |

Shared render: `🖥️app-surface/🦀️.rs:204–254` — `norm_ui_label` is **not localized** (`🦀️.rs:176–178`).

### 1.5 Examples / assets

| Example | Path | Notes |
|---------|------|-------|
| `composite-bridge-girder` | `📚️examples/🌉️composite-bridge-girder/` | Registered in editor (`✏️editor/🦀️.rs:57`); DSL at `🖼️assets/…/🗣️.dsl.semio` (330 kNm, 20 m span). Label DE = EN (`🦀️.rs:7`). **Passes all 7 checks** (u_max ≈ 0.96). |
| `demo-session` | `✏️editor/📚️examples/🎬️demo-session/` | Command script, not a compliance subject |

### 1.6 Tests

| Suite | Path | Asserts numbers? |
|-------|------|------------------|
| Compliance helpers | `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | **Yes** — stud P_Rd ≈ 81.656 kN (`:43–46`), b_eff ≈ 2160 mm (`:13–14`), η_min ≈ 0.49 (`:51–52`) |
| Compliance report | `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | Partial — `full_composite_worked_example` checks u < 1 (`:9–13`); `evaluate_runs_all_parts` only `len == 7` (`:17–19`) |
| `composite_beam_e2e` | `⚖️compliance/🦀️.rs:4–8` | `!is_empty()` + m_rd arithmetic only |
| Mutation fixtures | `🧫️fixtures/🧬️mutations/**` | Snapshot/diff/oracle round-trips, not compliance |
| Example | `📚️examples/…/🧪️tests/🧩️example/🦀️.rs` | `text.len() > 8` only (`:2–4`) |

**No third-party compliance oracle** — `🔮️oracles/🔣️.json:17–18` documents mutation-only Python second implementation.

### 1.7 Pilot language debt

`🦀️.rs:92–135` — document/op/diff/pack/spr grammars and protocols are **borrowed from `en1993`**, not EN 1994–specific.

---

## 2. Stub / fake detection

### 2.1 Pre-supplied resistances (inputs standing in for computed values)

| Field | Should be derived from | Currently |
|-------|------------------------|-----------|
| `m_pl_rd`, `m_pla` | Section geometry, materials, b_eff, partial connection | User scalars (`📸️snapshot/🦀️.rs:23–24`) |
| `eta` | n_f / n_f,req from stud layout | User scalar (`:25`) — `shear_connection_degree()` exists (`🦀️.rs:316–318`) but unused |
| `v_l_rd` | Longitudinal shear resistance from slab thickness, reinforcement, b_eff | User scalar (`:27`) — `longitudinal_shear_kn()` computes demand only (`🦀️.rs:333–335`) |
| `v_ed_kn` | Could chain from M_ed, stud spacing | Independent input |

### 2.2 Surrogate / simplified models

| Item | Location | Issue |
|------|----------|-------|
| Fire insulation table | `🦀️.rs:396–408` | Hardcoded 10/18/28/40 mm + 10% re-entrant factor — not EN 1994-1-2 Table 4.2 lookup by deck profile, fire exposure, load level |
| Bridge fatigue (EN 1994-2 §8) | `💡️inferences/🦀️.rs:114–115` | Compares `delta_sigma_mpa` to `bridge_fatigue_category(detail)` as MPa limit — category 71/80/90 is **not** a stress in MPa |
| `check_fire_composite` annex | `🦀️.rs:412` | Hardcoded `AnnexChoice::De` regardless of `document.annex` |
| Fire check quantity roles | `🦀️.rs:412` | `from_utilization(required, thickness)` — **computed=required, limit=provided** (semantically inverted labels) |
| `parse_fire_rating` default | `💡️inferences/🦀️.rs:75–81` | Unknown strings → R60 silently |
| `bridge_fatigue_category` default | `🦀️.rs:427` | Unknown detail → 71 |
| DE ≡ EN partial factors | `🦀️.rs:288–295` | `AnnexParams::de()` copies `en()` γ_V/γ_C/γ_S — comment claims intentional; no other DE NDPs implemented |

### 2.3 Checks that can never fail with defaults

Default snapshot (`📸️snapshot/🦀️.rs:81–107`) and bridge example DSL both yield **7/7 pass**. No example documents multiple simultaneous failures.

### 2.4 Placeholder / empty / orphan

| Item | Location |
|------|----------|
| Catalogue panel placeholder | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:3–5` |
| 27× `📌️.empty.md` under editor/viewer window slots | e.g. `✏️editor/🎚️config/📌️.empty.md` |
| `set-snapshot` orphan stub | `🧬️mutations/🦀️.rs:15–16` |
| Bridge example DE label | `📚️examples/🌉️composite-bridge-girder/🦀️.rs:7` — `"Composite Bridge Girder"` for both locales |
| `evaluate` command emits zero mutations | `✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:7` (doc comment) |

### 2.5 Clause ID vagueness

- `§6.2` / `6.2` for bending — entire clause, not Eq. 6.2 or 6.10 partial-connection formula
- `§6.6` / `6.6` for longitudinal shear — not Eq. 6.6–6.11 family
- `§8` / `8.1` for bridge fatigue — not a verifiable equation reference

### 2.6 Tests asserting only non-emptiness

- `⚖️compliance/🦀️.rs:6` — `!report.checks.is_empty()`
- `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:19` — `len == 7` only
- `📚️examples/…/🧩️example/🦀️.rs:4` — `text.len() > 8`

---

## 3. Complete subject definition (engineering target)

A feature-complete EN 1994 artifact must model **structures → members → cross-sections → materials → connectors → load cases → design effects**, not pre-aggregated resistances.

### 3.1 EN 1994-1-1 — General rules (buildings)

**Structure**

- `Project`: annex (`EN`/`DE`), consequence class, building category
- `Storey` / `Bay` grid: beam spacing, span L, support conditions (propped / unpropped / continuous)
- `CompositeBeam`: steel section ref (HEB/IPE catalogue or parametric), orientation, restraint (LTB length L_cr)

**Concrete flange / slab**

- Slab type: solid in-situ, composite with **profiled sheeting** (re-entrant/trapezoidal, geometry h_p, b_rib, t), precast
- `b_eff` per §5.4.1.2: b_0, beam spacing, span — **compute**, not omit
- Concrete grade f_ck, f_ctm, E_cm; reinforcement (longitudinal/transverse) layout
- Longitudinal shear interface: V_L,Ed from §6.6.2; resistance V_L,Rd from §6.6.4–6.6.8 (including transverse reinforcement)

**Steel section**

- Section class, W_pl,y, W_el,y, steel grade f_y (≤460), partial factors γ_M0
- **LTB** per §6.4: M_cr, λ_LT, χ_LT, M_b,Rd — **absent today**

**Shear connection**

- Headed studs: d, h_sc, f_u, layout (n_f, spacing e, rows), degree η
- P_Rd per Eq. 6.18/6.19 (implemented) with DE-NA limits on stud dimensions/spacing
- η_min per §6.6.1.2 (implemented) vs η from layout
- Partial connection: M_Rd from Eq. 6.10 (implemented via `plastic_moment_partial_knm`)

**ULS internal forces**

- M_Ed, V_Ed from compatible load combination (link to EN 1990 artifact or embedded combinations)
- **SLS**: deflection, crack width — not started

**Composite columns** (§6.7) — **entirely absent**

- Encased or concrete-filled tubular: N_Ed, M_Ed, N_pl,Rd, M_pl,Rd, buckling curves, M–N interaction (Eq. 6.43–6.47)

### 3.2 EN 1994-1-2 — Fire

- Fire exposure (ISO curve), load level η_fi, member type, deck/slab protection
- Insulation thickness from **tabular data** (profile-specific), not 4-point ladder
- Reduction factors k_y,θ, k_c,θ for moment resistance — absent
- DE-NA: national tabulated data where EN is non-informative

### 3.3 EN 1994-2 — Bridges

- Global analysis: composite girder, shear lag, effective width for box/twin-girder
- Fatigue load model (LM3 etc.), Δσ from **Gassner/Miner** or damage equivalent — not Δσ as bare input vs category integer
- Stud fatigue per §6.8.3 (partially implemented) plus **steel base material fatigue** per §6.8.2 and detail categories from EN 1993-1-9 Table 8.1
- γ_Mf,s, γ_Mf from annex — only γ_Mf,s = 1.0 constant (`🦀️.rs:432`)

### 3.4 Valid ranges (indicative)

| Quantity | Range | Unit |
|----------|-------|------|
| f_ck | 20–90 | MPa |
| f_y | 235–460 | MPa |
| d (stud) | 16–25 | mm |
| h_sc/d | ≥ 3 (typical ≥ 4 for full α) | — |
| η | 0–1 | — |
| span | 2–40+ | m |

---

## 4. Check catalogue

| # | Part | Clause / eq. | Verified today | Required subject inputs | Limit source | Failure meaning |
|---|------|--------------|----------------|-------------------------|--------------|-----------------|
| 1 | 1-1 | §6.2 / partial Eq. 6.10 | M_Ed ≤ M_Rd(η) | `m_ed_knm`, `m_pla`, `m_pl_rd`, `eta` | M_Rd computed from inputs | Bending ULS exceeded |
| 2 | 1-1 | §6.6 | V_L,Ed ≤ V_L,Rd | `v_ed_kn`, `v_l_rd` | **User-supplied** V_L,Rd | Longitudinal shear exceeded |
| 3 | 1-1 | §6.6.3.1 Eq. 6.18/6.19 | v_ed,stud ≤ P_Rd | stud geom + concrete + `v_ed_per_stud_kn` | P_Rd(γ_V=1.25 EN & DE) | Stud shear failure |
| 4 | 1-1 | §6.6.1.2 | η ≥ η_min(L, f_y) | `eta`, `span_m`, `f_y_mpa` | η_min formula (`🦀️.rs:357–360`) | Partial connection insufficient |
| 5 | 1-2 | §4.2 (claimed) | t_ins ≥ t_req | `insulation_thickness_mm`, `fire_rating`, `deck_type` | **Surrogate table** `🦀️.rs:396–408`; annex forced DE | Fire insulation inadequate |
| 6 | 2 | §8.1 (claimed) | Δσ ≤ limit | `delta_sigma_mpa`, `fatigue_detail` | **Category integer as MPa** — wrong | Meaningless pass/fail |
| 7 | 2 | §6.8.3 Eq. 6.24 | Δτ ≤ Δτ_c(N)/γ | `delta_tau_stud_mpa`, `n_cycles_stud` | Δτ_c = 90 MPa @ 2×10⁶, m=8 (`🦀️.rs:435–441`) | Stud fatigue exceeded |

**Not implemented (family scope per task brief):**

| Part | Topic |
|------|-------|
| 1-1 §5.4.1.2 | Effective width b_eff (helper only) |
| 1-1 §6.4 | Lateral torsional buckling |
| 1-1 §6.6.4–6.8 | Longitudinal shear resistance detail, transverse reinforcement |
| 1-1 §6.7 | Composite columns N/M interaction |
| 1-1 | Composite slab with profiled sheeting (shear bond, punching, end anchorage) |
| 1-2 | Full fire M/N reduction, temperature fields |
| 2 | Bridge global analysis, steel flange fatigue with proper detail classes |

---

## 5. Remediation strategy (target behaviour)

Core `CheckResult` (`⚖️compliance/🦀️.rs:138–146`) has no remediation field — Wave B must add `remediation: LocalizedText`, `subject_path`, `target_quantity`.

Per-check analytic inversions (to implement in Wave C):

| Check | On failure, report | Target field(s) | Formula |
|-------|-------------------|-----------------|---------|
| 1 Bending | “Reduce M_Ed to ≤ {M_Rd} kNm OR increase M_pl,Rd / η” | `m_ed_knm` or `m_pl_rd`/`eta` | M_Rd = m_pla + η(m_pl_rd − m_pla); set m_ed_knm ≤ M_Rd |
| 2 Long. shear | “Increase V_L,Rd to ≥ {V_Ed} kN OR reduce V_Ed” | `v_l_rd` or `v_ed_kn` | v_l_rd ≥ v_ed_kn |
| 3 Stud | “Add studs / larger studs: need P_Rd ≥ {v_ed} kN (currently {P_Rd})” | `d_mm`, `h_sc_mm`, `f_u_mpa`, count | P_Rd from Eq. 6.18/6.19; or reduce `v_ed_per_stud_kn` |
| 4 η_min | “Increase η to ≥ {η_min} (add {n_add} studs)” | `eta` or stud layout | η_min(span, f_y); η = n_f/n_req |
| 5 Fire | “Increase insulation to ≥ {t_req} mm for {rating} {deck}” | `insulation_thickness_mm` | t_req from national table |
| 6 Bridge Δσ | Replace with proper damage/Δσ_limit from detail class | `delta_sigma_mpa` or detail | Per EN 1993-1-9 + EN 1994-2 |
| 7 Stud fatigue | “Reduce Δτ to ≤ {limit} MPa OR increase N_ref domain” | `delta_tau_stud_mpa` | limit = 90·(2×10⁶/N)^(1/8)/γ |

Inverse mutations (`↩️inverse/`) should gain **compliance siblings** (`remedy/`) that compute `new_*` from failed check + snapshot.

---

## 6. Report & UX gaps

| Gap | Evidence |
|-----|----------|
| No pass/fail remediation text | `CheckResult.message` is static English tag (`🦀️.rs:363` “composite bending ULS”) |
| No subject-field reference | No path to `m_ed_knm` etc. in result |
| Report not localized | `render_report` English only (`🖥️app-surface/🦀️.rs:210–214`); manifest actions localized (`✏️editor/🦀️.rs:230`) |
| Inspection panel English | `Status`, `Utilization`, `Message` (`🖥️app-surface/🦀️.rs:246–249`) |
| Viewer table English columns | `report_table_columns` (`🖥️app-surface/🦀️.rs:159–161`) |
| No typed editors | Inputs = JSON (`📥️inputs/🦀️.rs:19–20`) — 22 fields not individually editable in UI |
| Catalogue useless | Placeholder (`📚️catalogue/🦀️.rs:3–5`) |
| Fire computed/limit swap confuses UI | User sees “computed=0.018 m, limit=0.020 m” for passing case — inverted semantics |
| Default never shows failure UX | All examples pass |

---

## 7. Target design

### 7.1 Proposed snapshot (sketch)

```rust
pub struct En1994Snapshot {
    pub annex: AnnexChoice,
    pub structure: StructureRef,           // grid, bays
    pub beams: Vec<CompositeBeam>,         // id, span, spacing, support
    pub sections: Vec<CompositeSection>,   // steel + slab + deck profile
    pub materials: Materials,              // steel grades, concrete, stud catalogue
    pub connectors: Vec<StudRow>,          // position, d, h_sc, f_u, n
    pub load_cases: Vec<LoadCase>,         // or EN 1990 link
    pub design_effects: Vec<BeamEffects>, // M_Ed, V_Ed, V_L,Ed per beam/check zone
    pub fire_scenario: Option<FireScenario>,
    pub fatigue_scenario: Option<FatigueScenario>,
}
// Deprecate direct m_pl_rd, v_l_rd, eta as inputs — compute in evaluate()
```

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1994Snapshot) -> CheckReport {
    let mut r = CheckReport::default();
    for beam in &doc.beams {
        let sec = doc.section(beam.section_id);
        let b_eff = part_1_1::effective_width_mm(...);
        let m_rd = part_1_1::plastic_moment_partial_knm(...); // from computed M_pl,a, M_pl,Rd
        r.push(check_with_remedy(part_1_1::check_composite_bending(...), &beam, ...));
        r.push(... longitudinal shear from computed V_L,Rd ...);
        r.push(... stud ...);
        r.push(... eta from connector layout ...);
        r.push(... LTB ...);
    }
    for col in &doc.columns { r.push(... N-M interaction ...); }
    if let Some(f) = &doc.fire_scenario { r.push(... tabulated insulation ...); }
    if doc.is_bridge() { r.push(... proper fatigue damage ...); }
    r
}
```

### 7.3 Example subjects

**Compliant (office floor beam)**  
HEB 300, span 8 m, spacing 3 m, C30/37, 19×95 studs @ 200 mm, η ≈ 0.75, M_Ed = 180 kNm → all ULS checks u < 0.85.

**Non-compliant (multi-failure)**  
Same beam, M_Ed = 320 kNm (bending fail u ≈ 1.54), insulation 10 mm @ R60 (fire fail u = 1.8), v_ed_per_stud = 90 kN (stud fail u ≈ 1.10). Commit as `📚️examples/🏢composite-floor-beam-failing/`.

### 7.4 Worked-example tests (add)

```rust
// Stud P_Rd — already at ⚖️compliance/🦀️.rs:32–46
// NEW: evaluate failing fixture — assert status Fail + utilization > 1.0 for checks [0,2,4]
// NEW: b_eff drives M_pl,Rd chain — numeric M_Rd from section catalogue
// NEW: DE fire table row ≠ EN where DIN NA differs
```

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / snapshot | `🧬️schema/🦀️.rs`, `📸️snapshot/**`, `🔺️diff/**` |
| Compliance | `🧬️schema/🦀️.rs` (part modules), `💡️inferences/🦀️.rs` |
| Mutations | New collection mutations; retire direct `change-m-pl-rd` as primary path |
| Editor inputs | Replace JSON blob with field panels per section |
| Results / inspection | Localized labels + remediation lines |
| Examples | Add failing example; fix bridge DE label |
| Tests | `⚖️compliance/`, `🔬️compliance-report/`, language-agnostic JSON oracle for `evaluate` |
| Oracles | Add compliance oracle (not just `en1994-1-mutate`) |
| Core (Wave B) | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` |

---

## 8. Risks / open questions

1. **Scope boundary** — One artifact per “member check” vs whole building? Columns + beams + slabs in one document or separate artifact kinds?
2. **EN 1990 coupling** — M_Ed/V_Ed supplied vs computed from linked EN 1990 load-combination artifact?
3. **Section catalogue** — Integrate EN 1993 rolled sections or parametric plate/assembled sections?
4. **DE-NA fire tables** — DIN EN 1994-1-2/NA tabulated insulation: digitize which deck profiles first?
5. **Bridge fatigue** — Drop bogus check #6 until proper EN 1993-1-9 damage equivalent is implemented?
6. **en1993 grammar borrow** — When does EN 1994 get its own DSL/protocol facets?
7. **η from layout** — Breaking change: `eta` becomes derived; migration for existing `.dsl.semio` files?
8. **Third-party compliance oracle** — `structuralcodes` (Python) cited in `🔮️oracles/🔣️.json:35` — adopt for formula cross-check?
9. **Wave B remediation schema** — Confirm `subject_path` format for flat vs nested future schema.
10. **450-line ceiling** — Composite columns + slabs alone may need multiple artifact subsets (`1-1/building`, `1-2/fire`, `2/bridge`).

---

*Auditor: read-only pass, 2026-09-26. No code modified.*
