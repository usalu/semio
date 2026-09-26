# Audit — EN 1999 (`🪶️en1999`)

**Executive summary.** The EN 1999 artifact is a mature **mutation/codec scaffold** (26 scalar fields, 26 mutations, DSL/pack/json facets, Python second-implementation oracle for mutations) wrapped around a **thin surrogate calculator**: `evaluate()` emits exactly **8 utilization checks** from pre-aggregated scalars, not from a structural model. Buckling reduction `chi`, section class, HAZ, bolted joints, shear, LTB, flexural buckling curves (α, λ̄₀), and most of EN 1999-1-1 Table 3.1 are missing or never reached. User-supplied `chi` and a simplified torsional-buckling formula make the **default snapshot fail buckling catastrophically** (u ≈ 10⁴) while tests mostly assert `checks.len()` or helper numerics, not end-to-end compliance truth. UI shows English-only messages in a JSON editor + utilization table; **no remediation, no subject-element binding, no localized report prose**. DIN EN NA γ factors match EN (documented, correct); other NA topics (allowed alloys, detail categories) are unmodelled. Rebuild requires a hierarchical subject schema, computed resistances, ~40–80 clause checks, remediation in core, and typed en/de editors — not scalar tweaks.

Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999`  
Subset: `🏅️standards/🔖️1/🪆️subsets/✳️any`  
Shared core: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs`

---

## 1. Inventory

### 1.1 Snapshot / document fields (26 scalars + annex)

Source: `🧬️schema/📸️snapshot/🦀️.rs` L14–67 (mirrored in `🧬️schema/🦀️.rs` L14–67 `En1999Artifact`).

| Field | Type | Unit (implicit) | Role in evaluate |
|-------|------|-----------------|------------------|
| `n_ed_kn` | f64 | kN | 1-1 cross-section & buckling actions |
| `m_ed_knm` | f64 | kNm | 1-1 bending action |
| `a_mm2` | f64 | mm² | 1-1 area → N_Rd, buckling |
| `w_el_mm3` | f64 | mm³ | 1-1 bending resistance |
| `alloy` | String | — | lookup → f₀.₂, f_u (2 alloys only) |
| `chi` | f64 | — | **user-supplied** flexural buckling χ (not computed) |
| `i_t_mm4` | f64 | mm⁴ | torsional buckling helper input |
| `l_cr_mm` | f64 | mm | buckling length |
| `theta_c` | f64 | °C | 1-2 fire scalar check |
| `delta_sigma_ed` | f64 | MPa | 1-3 fatigue action |
| `delta_sigma_c` | f64 | MPa | 1-3 detail category strength @ 2×10⁶ |
| `fatigue_m` | f64 | — | S–N slope m |
| `n_cycles` | f64 | — | fatigue cycles N |
| `v_weld_ed_kn` | f64 | kN | weld action |
| `weld_throat_mm` | f64 | mm | weld geometry |
| `weld_length_mm` | f64 | mm | weld geometry |
| `beta_w` | f64 | — | weld correlation factor |
| `sheet_b_mm`, `sheet_t_mm`, `sheet_k_sigma`, `sheet_w_el_mm3`, `sheet_m_ed_knm` | f64 | mm / — / mm³ / kNm | 1-4 cold-formed strip (disconnected from member above) |
| `shell_t_mm`, `shell_r_mm`, `sigma_ed_shell_mpa` | f64 | mm / MPa | 1-5 cylindrical shell (disconnected) |
| `annex` | `AnnexChoice` | en \| de | γ_M1, γ_M2 |

**Default snapshot** (`📸️snapshot/🦀️.rs` L79–109): e.g. `n_ed_kn=80`, `chi=0.85`, `alloy="aw6060t6"`, `annex=De`.

No composed child entities (no members[], connections[], materials catalogue, load cases, zones).

### 1.2 Mutations (26)

`🧬️schema/🧬️mutations/🦀️.rs` L56–117 `KINDS`: one `change-<field>` per scalar; `from_snapshot` L126–155 bundles bulk replace.

Each mutation triad: `🦀️.rs`, `🔺️diff/`, `↩️inverse/`, fixture under `🧫️fixtures/`, unit test under `🧪️tests/`. Mutations validate finiteness / no-op only — **never re-run compliance**.

### 1.3 `evaluate()` call graph

```
evaluate(document)                          💡️inferences/🦀️.rs L135–164
  └─ parse_alloy(&document.alloy)           L126–131 (unknown → Aw6060T6)
  └─ check_full_aluminium(...)              L76–124
       ├─ check_aluminium_member(...)       🧬️schema/🦀️.rs L621–633
       │    ├─ na_de::AnnexParams::for_choice
       │    ├─ part_1_1::check_cross_section
       │    ├─ part_1_1::check_buckling(N_Ed, min(N_b,Rd, N_t,Rd))
       │    │    ├─ buckling_resistance_kn(a, f₀.₂, chi_user, γ_M1)  L615–617
       │    │    └─ part_1_1::torsional_buckling_resistance_kn(...)  L459–465, E=70000 hardcoded L626
       │    └─ part_1_1::check_bending → m_c_rd_knm                  L426–428
       ├─ part_1_2::check_fire_protection(θ_c, θ_cr(f₀.₂))         L107–108, θ_cr L488–490
       ├─ part_1_3::check_fatigue(Δσ_Ed, fatigue_strength_mpa(...))  L109–110
       ├─ part_1_1::check_welded_joint                               L111–113
       ├─ part_1_4::check_cold_formed_sheeting (λ_p, ρ, W_eff)       L114–117
       └─ part_1_5::check_shell_buckling (σ_cr, λ̄, χ_shell, σ_Rd)   L118–122
```

**Helpers defined but NOT reached by `evaluate()`:**

| Helper | Location | Only used in |
|--------|----------|--------------|
| `classify_flange_outstand` | `🧬️schema/🦀️.rs` L411–423 | compliance test L14 |
| `haz_strength_mpa`, `haz_reduction_factor` | L449–456 | tests L19–32 |
| `strength_reduction_factor` (k_θ) | `part_1_2` L493–498 | test L63 |
| `bending_resistance_knm` | L610–612 | — (dead) |
| `HAZ_ZONE_MM` | `na_de` L334 | test L23 |

### 1.4 Editor / viewer

| Surface | Path | Behaviour |
|---------|------|-----------|
| Inputs window | `✏️editor/.../📥️inputs/🦀️.rs` L19–21 | `render_document_json` — raw camelCase JSON, no per-field widgets |
| Results window | `.../📊️results/🦀️.rs` L22–24 | virtualized check list via `render_report` |
| Inspection panel | `📌️panels/🔍️inspection/🦀️.rs` L19–20 | single check: clause, status, u, message (EN labels) |
| Document panel | `📌️panels/🗿️artifact/🦀️.rs` L18–19 | one-line summary EN only (`app-surface` L226–228) |
| Catalogue | `📌️panels/📚️catalogue/🦀️.rs` L3–5, L21–22 | **placeholder** headline |
| Viewer report | `👁️viewer/.../📊️report/🦀️.rs` L30–32 | `TableWindowKit`: columns `Clause, Status, Utilization, Message` (`app-surface` L159–166) |

Commands (`✏️editor/🦀️.rs` L41–46): `setSnapshot`, `evaluate`, `selectedCheckIndex`, `setActiveExample`. Media ports: `model:in`, `report:out` (L154–160).

### 1.5 Examples / assets

| Asset | Path | Notes |
|-------|------|-------|
| `aluminium-roof-purlin` | `📚️examples/🏠️aluminium-roof-purlin/🦀️.rs` | DSL in `🖼️assets/.../🗣️.dsl.semio`; label **not localized** (L7: DE = EN) |
| `demo-session` | `✏️editor/📚️examples/🎬️demo-session/` | editor workflow facet |
| Default DSL export | `encode_en1999_dsl` | round-trip tests only |

### 1.6 Tests (54 `🦀️.rs` under family)

| Bucket | Asserts numbers? | Gap |
|--------|------------------|-----|
| `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | **Yes** — M_c,Rd, ρ(λ_p), σ_cr, weld, HAZ ρ | Unit helpers only |
| `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | Weak — `len==8`, one `u<1`, EN==DE annex | No per-check expected u |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `!report.checks.is_empty()` L174, L228 | No numeric compliance |
| 26 mutation fixtures | snapshot field equality | No evaluate outcome |
| `🪶️mutate-en1999-1` + Python oracle | codec/mutation parity | **Zero evaluate coverage** |
| Example tests | inference determinism, `text.len()>8` | No compliance |

### 1.7 Oracles

`🔮️oracles/🔣️.json`: `en1999-1-python-independent` — **mutations only** (26 vectors). Rationale L18: no third-party EN 1999 interchange library; compliance evaluate has **no oracle**.

### 1.8 Infrastructure debt

- Pilot DSL/protocol grammars **borrowed from EN 1998** (`🦀️.rs` L87–131 `semio_s_artifact_norm_en1998`).
- **27** `📌️.empty.md` placeholder facets (editor/viewer config, presence, transient, commands).

---

## 2. Stub / fake detection

| ID | Evidence (file:line) | Severity |
|----|----------------------|----------|
| **S1 User-supplied χ** | `chi` is snapshot input L26; `buckling_resistance_kn(..., chi, ...)` L615–617 — buckling check uses typed value, not λ̄ from geometry | Critical — hides non-compliance |
| **S2 Torsional buckling surrogate** | `torsional_buckling_resistance_kn` L459–465: ν=0.33, single-term C_T, generic χ(λ) with 0.21 — not EN 1999-1-1 §6.3.3 tables | Critical — default doc N_t,Rd≈0.008 kN → u≈10⁴ |
| **S3 Alloy catalogue stub** | `Alloy` enum: only `Aw6060T6`, `Aw6082T6` L369–372; `parse_alloy` unknown→6060 L126–131 | High — mutation stores `aw7020t6` but evaluate silently uses 6060 strengths |
| **S4 HAZ unimplemented** | `haz_reduction_factor` L454–456, `HAZ_ZONE_MM` L334 — **never in evaluate** despite family brief | High |
| **S5 Section class unimplemented** | `classify_flange_outstand` L411–423 — never in evaluate | High |
| **S6 Fire simplified** | `critical_temperature_c`: `170+0.4·f₀.₂` L488–490; `k_θ` L493–498 **unused**; check is θ_c vs θ_cr only L501–508 — not 1-2 strength reduction at fire | High |
| **S7 Fatigue detail stub** | User types `delta_sigma_c`, `fatigue_m` — no detail category tables, no Δσ_c,0 from EN 1999-1-3 Tables | Medium |
| **S8 Cold-formed / shell disconnected** | Separate scalar groups; no link to member `alloy`/section; always evaluated | Medium — misleading multi-part “full” report |
| **S9 Weld only directional throat** | `weld_resistance_kn` L468–470 — no fillet/partial/intermittent, no bolted §8 (family brief) | High |
| **S10 DE annex identity** | `AnnexParams::de()` γ_M1=γ_M2 same as EN L348–351 — **correct for γ** per comment L348; test L18–27 locks equality | OK for γ; **false comfort** if user expects other NA deltas |
| **S11 Checks that can “never fail” fire** | With θ_c=200, θ_cr=246 (6060), u=0.81 always pass until θ_c>246 — no link to structural capacity reduction | Medium |
| **S12 English-only messages** | `check_*` messages: `"aluminium cross-section ULS"` etc. L431–604; table headers L159–160 | UX gap |
| **S13 Catalogue placeholder** | `catalogue/🦀️.rs` L3–5 | Low |
| **S14 Example i18n stub** | `aluminium-roof-purlin` L7 | Low |
| **S15 No remediation** | `CheckResult` L138–146: `message: String` only | Critical per coordination objective |
| **S16 Tests `!is_empty()`** | `editor/🧪️tests` L174, L228; `compliance-report` L14 `len==8` | High — masks S2 failure on default |

**Default snapshot derived utilizations** (recomputed from code formulae, 6060-T6, γ_M1=1.1):

| # | Clause (as emitted) | u | Pass? |
|---|---------------------|---|-------|
| 1 | EN 1999-1-1 §6.2 | 0.39 | ✓ |
| 2 | EN 1999-1-1 §6.3 | **≈10471** | ✗ (torsional min) |
| 3 | EN 1999-1-1 §6.2.5 | 0.96 | ✓ |
| 4 | EN 1999-1-2 §4 | 0.81 | ✓ |
| 5 | EN 1999-1-3 §7 | 0.53 | ✓ |
| 6 | EN 1999-1-1 §8.5 | 0.19 | ✓ |
| 7 | EN 1999-1-4 §5.4 | 0.65 | ✓ |
| 8 | EN 1999-1-5 §5.3 | 0.87 | ✓ |

---

## 3. Complete subject definition (engineering target)

The artifact must model an **aluminium structure (or assessable sub-assembly)** governed by DIN EN 1999 (+ NA). Minimum domain graph:

```
Project
 └─ NationalAnnex (DE | EN + NA tables)
 └─ MaterialCatalogue[] ← EN 1999-1-1 Table 3.1 (+ NA permitted alloys/tempers)
      └─ AlloyTemper { id, f_o, f_u, E, G, ε, fracture, exposure class }
      └─ HazData { f_o_haz, ρ_o_haz, ρ_u_haz, haz_width_mm, process }
 └─ CrossSectionLibrary[]
      └─ Section { kind, geometry, A, I_y, I_z, W_el, W_pl, I_t, I_w, class, ρ_local }
 └─ Member[]
      └─ { id, sectionRef, materialRef, L, L_cr_y, L_cr_z, L_cr_T, end_restraints, imperfection }
      └─ LoadCase[] → MemberForces { N_Ed, V_y,Ed, V_z,Ed, M_y,Ed, M_z,Ed, Δσ ranges }
 └─ Connection[]
      └─ Welded { throat, length, type, β_w, direction, HAZ length }
      └─ Bolted { bolt grade, diameter, pattern, bearing, slip }
 └─ ColdFormedElement[] (1-4) — sheet geometry, supports, k_σ, k_τ, stiffeners
 └─ ShellSegment[] (1-5) — cylinder/cone, t, r, boundary, α_buckling
 └─ FireScenario[] (1-2) — θ_a, insulation, duration, member heating model
 └─ FatigueDetail[] (1-3) — detail category, spectrum, N_Ed, m, N_D
```

**Field requirements by part:**

- **1-1 General structural rules:** tension §6.2.3, compression §6.2.4, bending §6.2.5, shear §6.2.6, torsion, combined §6.2.7–6.2.8; classification Table 6.2 (β/ε); local buckling class 4; **member buckling** §6.3 with curve selection (α, λ̄₀ per Table 6.4), **LTB** §6.3.2; **HAZ** §6.2.9 / Table 6.2 ρ_o,haz, ρ_u,haz; γ_M from NA (DE: 1.10 / 1.25 — current code OK).
- **1-2 Fire:** θ_a, member temperature field or conservative θ_c; k_θ(θ) on f_o, E; fire combinations — not scalar θ compare alone.
- **1-3 Fatigue:** detail category → Δσ_c,0; Palmgren-Miner, cut-off N_L=2×10⁶, slope m=8 (variable per detail).
- **1-4 Cold-formed:** internal/external elements, effective widths, distortional buckling, section resistances tied to **sheet profile**.
- **1-5 Shells:** buckling shapes (E, S, T), geometric imperfections, α_buckling factors — not only σ_cr=0.605Et/r.

**Valid ranges:** all actions ≥0 where physical; ε>0; slenderness >0; N_cycles ≥1; temperatures within aluminium service; annex enum with NA table binding.

---

## 4. Check catalogue (current vs required)

| Part | Clause / eq. | What is verified today | Required inputs today | Limit source (DE / EN) | Failure meaning |
|------|--------------|------------------------|----------------------|-------------------------|-----------------|
| 1-1 | §6.2 | N_Ed ≤ N_Rd = A·f₀.₂/γ_M1 | `n_ed_kn`, `a_mm2`, alloy | f₀.₂ Table 3.1; γ_M1=1.1 both | Compression/tension ULS |
| 1-1 | §6.3 | N_Ed ≤ min(χ·N_Rd, N_t,Rd) | `n_ed_kn`, `chi` **input**, `i_t_mm4`, `l_cr_mm` | χ user; N_t surrogate | Member buckling ULS |
| 1-1 | §6.2.5 | M_Ed ≤ W_el·f₀.₂/γ_M1 | `m_ed_knm`, `w_el_mm3` | same | Bending ULS |
| 1-1 | §6.2 Table 6.2 | — **not checked** | b, t, alloy | β/ε limits | Class 1–4 |
| 1-1 | §6.2.9 HAZ | — **not checked** | weld zones | ρ_o,haz Table 6.2 | HAZ strength |
| 1-1 | §6.2.6 shear | — | V_Ed, A_v | — | Shear ULS |
| 1-1 | §6.3.2 LTB | — | M_Ed, LTB length, C_1 | — | Lateral buckling |
| 1-1 | §8 bolted | — | bolt layout | — | Connection ULS |
| 1-1 | §8.5 weld | V_Ed ≤ F_w,Rd | throat, length, `beta_w`, `v_weld_ed_kn` | f_u, γ_M2=1.25 | Weld ULS |
| 1-2 | §4 | θ_c ≤ θ_cr | `theta_c`, f₀.₂ | θ_cr formula L488 | Fire scalar (simplified) |
| 1-2 | k_θ strength | — | θ, duration | NA tables | Fire resistance |
| 1-3 | §7 | Δσ_Ed ≤ Δσ_RD(N) | `delta_sigma_ed/c`, `fatigue_m`, `n_cycles` | user-typed Δσ_c | Fatigue |
| 1-4 | §5.4 | M_Ed ≤ W_eff·f₀.₂/γ_M1 | sheet scalars | k_σ user | Sheeting bending |
| 1-5 | §5.3 | σ_Ed ≤ χ·f₀.₂/γ_M1 | shell t,r, σ_Ed | σ_cr L580 | Shell buckling |

---

## 5. Remediation strategy (per current check)

Until Wave B `CheckResult` supports remediation, target messages should name **subject path**, **current**, **required**, **clause**.

| Check | Remediation computation | Target field(s) |
|-------|-------------------------|-----------------|
| §6.2 N | `N_Ed,req = N_Rd` or `A_req = N_Ed·γ_M1/f₀.₂` | `member[].N_Ed` or section A |
| §6.3 buckling | **Remove χ input**; invert λ̄(α, λ̄₀) → `L_cr,max` or pick larger section (`A`, `I_t`) | `member.L_cr`, section |
| §6.2.5 M | `M_Ed,req = W_el·f₀.₂/γ_M1` → increase `w_el_mm3` or alloy | section / material |
| §1-2 fire | `θ_c,limit` from fire calc; or increase protection → lower θ_c | `fire.insulation` |
| §1-3 fatigue | `Δσ_Ed,allow = Δσ_c·(2e6/N)^(1/m)`; reduce spectrum or upgrade detail | `fatigue.detailCategory` |
| §8.5 weld | `t_req` or `l_req` from `V_Ed ≤ a·l·f_u/(β_w·γ_M2)` | `connection.weld` |
| §1-4 sheet | thicken `t`, reduce `b`, or increase `k_σ` (stiffener) | `coldFormed.elements[]` |
| §1-5 shell | increase `t`, reduce `σ_Ed` (stiffeners), or `r` | `shell.segments[]` |

Example message (DE): *„Nachweis EN 1999-1-1 §6.3: N_Ed=80 kN > N_b,Rd=0.008 kN — kritische Drehflächensteifigkeit I_t erhöhen auf ≥ 4,2×10⁷ mm⁴ oder Biegedrucklänge L_cr verkürzen auf ≤ 850 mm.“*

---

## 6. Report & UX gaps

| Requirement (coordination) | Status |
|----------------------------|--------|
| Per-clause comply / not comply | Partial — `Pass`/`Fail` from u≤1 |
| How to comply | **Missing** — no remediation field |
| Localized en + de | **Partial** — window/panel labels localized; **check messages, table headers, summary EN-only** (`app-surface` L159–166, L226–228) |
| Edit all subject fields | **No** — JSON blob only (`inputs` L19–21) |
| Readable report | Table + inspection OK structurally; no quantities with units in columns |
| Grouping by member/part | **No** |
| Link check → subject element | **No** |

---

## 7. Target design

### 7.1 Snapshot sketch (Rust-ish)

```rust
pub struct En1999Snapshot {
    pub annex: AnnexChoice,
    pub materials: Vec<AluminiumMaterial>,      // Table 3.1 + HAZ
    pub sections: Vec<AluminiumSection>,
    pub members: Vec<AluminiumMember>,
    pub connections: Vec<AluminiumConnection>,
    pub cold_formed: Vec<ColdFormedMember>,      // optional part 1-4
    pub shells: Vec<ShellSegment>,               // optional part 1-5
    pub fire: Option<FireAssessment>,
    pub fatigue: Vec<FatigueAssessment>,
}
// Computed at evaluate — NOT stored: chi, class, W_eff, σ_cr, ...
```

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1999Snapshot) -> CheckReport {
    let na = AnnexParams::for_choice(doc.annex);
    for member in &doc.members {
        let mat = resolve_material(&doc.materials, member.material_id);
        let sec = resolve_section(&doc.sections, member.section_id);
        report.extend(check_1_1_member(member, sec, mat, na)); // incl. class, HAZ, LTB, shear
    }
    for conn in &doc.connections { report.extend(check_1_1_connection(conn, na)); }
    if let Some(f) = &doc.fire { report.extend(check_1_2(f, na)); }
    for fat in &doc.fatigue { report.extend(check_1_3(fat, na)); }
    for cf in &doc.cold_formed { report.extend(check_1_4(cf, na)); }
    for sh in &doc.shells { report.extend(check_1_5(sh, na)); }
    report
}
```

### 7.3 Example subjects

**Compliant — roof purlin (DE):** single member AW-6082-T6, I-section 80×60×4, L=6 m, L_cr,y=1.2 m, C_1=1.0, g+q snow; N_Ed≈12 kN, M_Ed≈3.2 kNm; class 2 flange; χ from §6.3 curve b (α=0.2, λ̄₀=0.21); all u≤0.85.

**Non-compliant — multi-failure:** (1) M_Ed > M_c,Rd (undersized W_el); (2) LTB λ̄>1.0; (3) weld throat too small; (4) fatigue Δσ_Ed > Δσ_RD at N=5×10⁵; (5) HAZ zone with reduced f_o in welded flange.

### 7.4 Numeric worked tests (to add)

1. **6082-T6 M_c,Rd** — already `compliance/🦀️.rs` L4–8: W_el=24000 → 4.145 kNm ✓  
2. **Remove χ-input regression** — given A, I_y, L_cr, α, λ̄₀ → χ=0.823 (hand calc) → N_b,Rd  
3. **HAZ welded check** — ρ_o,haz=0.71 (6082 GMAW) → M_Rd,haz = ρ·W·f_o/γ_M1  
4. **Full evaluate default** — assert check[1] **Fail** until I_t fixed (documents S2)  
5. **EN vs DE** — only where NA tables diverge (future alloy list restrictions)

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / snapshot | `🧬️schema/🦀️.rs`, `📸️snapshot/**`, facets `🟦️.ts`, `🔣️.json`, `🔗️.graphql` |
| Diff / mutations | Regenerate from new shape; retire 26 scalar mutations |
| Compliance | `part_1_1`…`part_1_5` modules; move χ computation; add bolted/HAZ/LTB |
| Inferences | `💡️inferences/🦀️.rs` `evaluate`, `outline` field tree |
| Editor | Typed `inputs` windows per entity; DE labels on fields |
| Viewer | Localized report columns + remediation rows |
| Examples | ≥2 full DSL documents + compliance oracle JSON |
| Tests | `⚖️compliance`, `compliance-report`, per-member integration; Python evaluate oracle |
| Oracles | `🔮️oracles/🔣️.json` — add `en1999-evaluate-*` capability |
| Core (Wave B) | `⚖️compliance/🦀️.rs` remediation + `LocalizedText` messages |

---

## 8. Risks / open questions

1. **Scope of “one artifact”:** Is one document one member, one connection detail, or whole building? Drives schema root.
2. **χ surrogate removal** breaks all snapshots that store `chi` — migration: recompute or drop field.
3. **Torsional formula bug?** — confirm against EN 1999-1-1 worked examples; current impl may be dimensionally wrong (N_t,Rd≈0).
4. **Multi-part always-on checks** — should cold-formed/shell/fire/fatigue be optional scopes flags?
5. **DIN EN NA beyond γ:** alloy restrictions, execution classes, national fatigue details — source tables?
6. **EN 1998 grammar reuse** — when to fork `en1999.document` protocol from en1998?
7. **Wave B dependency** — remediation blocked on `CheckResult` extension; coordinate message i18n keys.
8. **Third-party oracle** — structuralcodes or similar for formula cross-check vs in-repo second impl?
9. **Performance** — full building with hundreds of members vs single-member MVP for Wave C.
10. **Default example claims “roof purlin”** but schema cannot represent purlin geometry — rename or enrich?

---

*Audit date: 2026-09-26. Read-only; no code changes.*
