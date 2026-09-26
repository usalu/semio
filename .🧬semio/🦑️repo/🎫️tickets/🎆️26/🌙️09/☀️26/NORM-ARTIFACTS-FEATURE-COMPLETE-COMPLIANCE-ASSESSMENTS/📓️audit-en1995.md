# EN 1995 (🪵️en1995) — Per-Family Compliance Audit

**Executive summary.** The `🪵️en1995` artifact is a **flat bag of 20 pre-aggregated scalars** (design actions, section properties, characteristic strengths, classification strings, connection/fire/bridge inputs) with **8 utilization checks** wired through `evaluate()` → `check_full_timber()`. Pure helpers for `k_mod`, LTB `k_crit`, and DE-vs-EN `k_cr` are **real and numerically tested**, but the artifact is **not** the complete subject of EN 1995-1-1 / DIN EN 1995-1-1/NA, EN 1995-1-2, or EN 1995-2: no strength classes, no tension/perpendicular compression/combined stress, no column buckling `k_c`, no SLS deflection/`k_def`, no Johansen connections, no proper fire residual-capacity check, and no remediation in reports. UI is JSON-in / English table-out; mutation oracles cover schema edits only, not compliance math. **Wave C must replace the scalar snapshot with a structured timber structure, fix the fire and bearing models, implement the missing check families, and attach localized remediation to every failing check.**

---

## 1. Inventory

### 1.1 Snapshot / document fields (`En1995Snapshot`)

Source: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` L14–54, mirrored in `🧬️schema/🦀️.rs` L14–55.

| Field | Type | Unit / domain | Role in checks |
|-------|------|---------------|----------------|
| `annex` | `AnnexChoice` (EN/DE) | — | γ_M (same EN/DE), `k_cr` shear width |
| `m_ed_knm` | `f64` | kN·m | Bending ULS (1-1 + 2) |
| `n_ed_kn` | `f64` | kN | Compression parallel ULS |
| `v_ed_kn` | `f64` | kN | Shear ULS |
| `w_mm3` | `f64` | mm³ | Bending resistance, LTB λ |
| `a_mm2` | `f64` | mm² | Compression resistance |
| `b_mm`, `h_mm` | `f64` | mm | Shear τ (Eq. 6.13a) |
| `f_m_k`, `f_c_0_k`, `f_v_k` | `f64` | MPa | Characteristic strengths (free scalars, not classes) |
| `service_class` | `String` | sc1/sc2/sc3 | `k_mod` lookup |
| `load_duration` | `String` | permanent/long/medium/short/instantaneous | `k_mod` lookup |
| `m_crit_knm` | `f64` | kN·m | LTB λ_rel,m |
| `f_ed_kn` | `f64` | kN | Connection “bearing” check |
| `a_ef_mm2` | `f64` | mm² | Connection resistance area |
| `fire_duration_min` | `f64` | min | Charring depth |
| `section_depth_mm` | `f64` | mm | Fire residual depth |
| `a_vert_m_s2` | `f64` | m/s² | Bridge pedestrian vibration |
| `n_cycles_bridge` | `f64` | — | Simplified fatigue factor |

**Defaults** (`📸️snapshot/🦀️.rs` L67–91): DE annex, SC1/medium, glulam-like strengths (f_m,k=24, f_c,0,k=21, f_v,k=4 MPa), fire R30, bridge inputs populated. Several default checks **fail** (bending u≈1.63, fire u≈13.4) — see §2.

**No composed children.** Flat scalar document; `En1995Outline::compute` sets `entry_count = 0` (`💡️inferences/🧾outline/🦀️.rs` L45–49).

### 1.2 Mutations (20 kinds)

`🧬️schema/🧬️mutations/🦀️.rs` L51–103, `KINDS` L82–103: one `change-<field>` per scalar + `change-annex`. Each triad has `🦀️.rs`, `🔺️diff/`, `↩️inverse/` (inverse = undo to BASE value, not compliance inversion). Fixture vectors under `🧫️fixtures/🧬️mutations/<kind>/`. Language-agnostic mutation oracle: `🧪️tests/🪵️mutate-en1995-1/` + `🐍️.py` (mutation semantics only, no `evaluate` parity).

### 1.3 `evaluate()` call graph

```
evaluate(document)                          💡️inferences/🦀️.rs:135
└─ check_full_timber(...)                   💡️inferences/🦀️.rs:77
   ├─ k_mod, lambda_rel_m, k_crit          🧬️schema/🦀️.rs:362–417
   ├─ check_glulam_beam(...)               🧬️schema/🦀️.rs:549
   │  ├─ part_1_1::bending_resistance_knm  🧬️schema/🦀️.rs:424
   │  ├─ part_1_1::compression_resistance  🧬️schema/🦀️.rs:429
   │  ├─ part_1_1::check_bending            🧬️schema/🦀️.rs:451
   │  ├─ part_1_1::check_compression       🧬️schema/🦀️.rs:461
   │  └─ part_1_1::check_shear             🧬️schema/🦀️.rs:470
   │     └─ AnnexParams::k_cr (EN 0.67 / DE min(1,2.5/f_v,k))  🧬️schema/🦀️.rs:343
   ├─ part_1_1::connection_bearing_*       🧬️schema/🦀️.rs:434–467
   ├─ part_1_2::charred_depth_mm            🧬️schema/🦀️.rs:486
   ├─ part_1_2::residual_section_mm         🧬️schema/🦀️.rs:490
   ├─ part_1_2::check_fire                  🧬️schema/🦀️.rs:494
   ├─ part_2::bridge_bending_resistance     🧬️schema/🦀️.rs:510
   ├─ part_2::check_bridge_timber           🧬️schema/🦀️.rs:514
   ├─ part_2::check_pedestrian_vibration    🧬️schema/🦀️.rs:519
   └─ part_2::check_bridge_fatigue          🧬️schema/🦀️.rs:533
      └─ part_2::fatigue_reduction_factor   🧬️schema/🦀️.rs:524
```

**Defined but never reached by `evaluate()`:** `k_def` (`🧬️schema/🦀️.rs:383`), `na_de::NaDe` re-export (`🧬️schema/🦀️.rs:298–300`, zero call sites).

**Duplicate binding:** `En1995Family::evaluate` (`✏️editor/🦀️.rs:199`) delegates to the same `inferences::evaluate`.

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `✏️editor/.../📥️inputs/🦀️.rs:19` | Pretty-printed JSON of full snapshot; no field-level form |
| Results | `✏️editor/.../📊️results/🦀️.rs:22` | Virtualized list via `app_surface::render_report` |
| Inspection | `✏️editor/.../🔍️inspection/🦀️.rs:19` | Single check: clause, status, u, English `message` |
| Catalogue | `✏️editor/.../📚️catalogue/🦀️.rs:20` | Placeholder headline (`render_catalogue`) |
| Viewer report | `👁️viewer/.../📊️report/🦀️.rs:30` | Table: Clause, Status, Utilization, Message (English) |
| Commands | `✏️editor/🦀️.rs:41–46` | `setSnapshot`, `evaluate`, `selectedCheck`, `setActiveExample` |

Window/presence/config slots: 27× `📌️.empty.md` placeholders under `✏️editor/` and `👁️viewer/`.

### 1.5 Examples, assets, tests, oracles

- **Example:** `📚️examples/🌉️glulam-footbridge/` — one DSL asset (`🖼️assets/.../🗣️.dsl.semio`), DE label missing (`🦀️.rs:7` both locales `"Glulam Footbridge"`).
- **Tests asserting numbers:** `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `k_mod`, LTB, `k_cr` EN/DE shear utilizations (0.2274 vs 0.2438), fire char 19.5 mm, bearing ~18.46 kN, vibration pass/fail threshold.
- **Tests asserting only shape:** `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:15–17` (`checks.len() == 8`); `✏️editor/🧪️tests/🔬️unit/🦀️.rs:174,228` (`!is_empty()`).
- **Oracles:** `🔮️oracles/🔣️.json` — mutation-only `en1995-1-python-independent`; **no compliance/evaluate oracle**.

---

## 2. Stub / fake detection

| Issue | Location | Evidence |
|-------|----------|----------|
| **Scalar subject, not structure** | `📸️snapshot/🦀️.rs:14–54` | No members, load cases, materials catalogue, connections — pre-merged M_Ed, W, A |
| **`k_def` dead code** | `🧬️schema/🦀️.rs:383–401` | Table 3.2 implemented; never called from `evaluate` or `check_full_timber` |
| **`na_de` unused** | `🧬️schema/🦀️.rs:298–300` | Re-export only; DIN-specific NDPs beyond `k_cr` not applied |
| **Connection bearing uses f_v,k** | `🧬️schema/🦀️.rs:434–437` | `connection_bearing_resistance_kn(a_ef, f_v_k, …)` — shear strength substituted for bearing (should be f_c,90,k / Johansen) |
| **Fire check nonsensical** | `🧬️schema/🦀️.rs:494–496` | Compares `remaining_mm` vs `charred_depth_mm` as computed/limit; always fails for realistic R30 (261/19.5≈13.4). Not §4.2 reduced-section capacity |
| **Constant β₀ only** | `🧬️schema/🦀️.rs:484` | `CHARRING_RATE_MM_MIN = 0.65`; no β_n, no zero-layer, no d_ef structural re-check |
| **Simplified fatigue** | `🧬️schema/🦀️.rs:523–529` | Comment: “Annex A **style** S-N degradation”; 10%/decade floor 0.5 — not EN 1995-2 Annex A |
| **Hardcoded annex on bridge/SLS** | `🧬️schema/🦀️.rs:515,520,541` | `check_bridge_timber`, `check_pedestrian_vibration`, `check_bridge_fatigue` pass `AnnexChoice::En` regardless of `document.annex` |
| **Fire check hardcoded DE** | `🧬️schema/🦀️.rs:495` | `AnnexChoice::De` fixed in `check_fire` |
| **Compression without k_c** | `🧬️schema/🦀️.rs:429–432,461` | Pure N_Ed ≤ N_Rd; no slenderness, no Eq. 6.26 column buckling |
| **Free-string enums** | `💡️inferences/🦀️.rs:115–130` | Unknown `service_class`/`load_duration` silently default to SC1/Medium |
| **Strength classes absent** | `f_m_k` etc. | User supplies MPa; no C24/GL28h catalogue or γ_M routing by product type beyond hardcoded Glulam/Solid in helpers |
| **Catalogue placeholder** | `✏️editor/.../📚️catalogue/🦀️.rs:3–4` | “headline placeholder today” |
| **Report columns English-only** | `🖥️app-surface/🦀️.rs:159–166` | `["Clause","Status","Utilization","Message"]` — no DE |
| **Check messages English-only** | `🧬️schema/🦀️.rs:456,474,…` | e.g. `"timber bending ULS"` |
| **No remediation in `CheckResult`** | `⚖️compliance/🦀️.rs:138–146` | `message: String` only; no `remediation`, no subject field ref |
| **Inverse = undo, not comply** | e.g. `↩️inverse/🦀️.rs:9–10` | Restores BASE scalar; cannot answer “increase W to …” |
| **Example test non-numeric** | `📚️examples/.../🦀️.rs:4` | `text.len() > 8` only |
| **Mutate adapter blocked** | `🧪️tests/🪵️mutate-en1995-1/🦀️.rs:41–44` | Subject parity cannot run until crate compiles |

**Checks that can never fail (given positive inputs):** pedestrian vibration when `a_vert_m_s2 ≤ 0.7` (`part_2/🦀️.rs:505`); fatigue when `n_cycles ≤ 10⁶` (`fatigue_reduction_factor` returns 1.0, L525–527).

---

## 3. Complete subject definition (engineering target)

The artifact must model a **timber structure (or assessable sub-structure)** under EN 1990 actions, not pre-merged scalars.

### 3.1 EN 1995-1-1 (+ DIN EN 1995-1-1/NA)

**Project / annex:** `AnnexChoice`, building class, consequence class (from EN 1990 link).

**Environment:** `ServiceClass` (1–3), moisture content where relevant; `LoadDuration` per action component.

**Materials (per member/layer):**
- Product: solid sawn (C classes), glulam (GL), LVL, CLT, plywood.
- Strengths: f_m,k, f_t,0,k, f_c,0,k, f_c,90,k, f_v,k, E_0,mean, G_mean, ρ_k — from **strength class tables** (EN 338 / EN 14080) or declared values.
- Modifiers: k_mod (Table 3.1), k_sys, k_h (size), k_cr (shear width: EN 0.67; DE NA min(1, 2.5/f_v,k)), k_v (notched beams), γ_M (Table 2.3: solid 1.3, glulam 1.25, connections 1.3 — unchanged in DE NA).

**Structure graph:**
- `Structure` → `Members[]` (beam, column, plate, truss) → `Section` (rectangle, built-up, effective after openings) → `MaterialRef`.
- `Supports`, `EffectiveLength` (L_ef,y, L_ef,z, L_ef,tors), `LateralRestraint` (for LTB M_cr).
- `LoadCases[]` / `LoadCombinations[]` (ULS, SLS char, SLS quasi-permanent, accidental) with tagged durations.

**Per-member design effects (derived or stored):** M_Ed, V_Ed, N_Ed (tension + compression), σ components, deflections w_inst, w_net, w_fin.

**Check families required:**
- Bending (§6.1.6), tension parallel (§6.1.5), compression parallel with **k_c** buckling (§6.3.2, Eq. 6.26), compression ⊥ with **k_c,90** (§6.1.5), shear (§6.1.7), combined stress (§6.2.1–6.2.4), torsion where relevant.
- **SLS:** deflection limits (span/xxx), `k_def` creep (Table 3.2), floor vibration (§7.3.3 freq + acceleration) for buildings.
- **Connections:** Johansen EYM — dowel-type (nails, screws, bolts): failure modes (a–j), f_h,k, f_ax,k, t₁, d, n_rows; not a single F_Ed vs f_v,k·A_ef surrogate.

### 3.2 EN 1995-1-2 (+ DIN NA)

- Fire exposure (R/E/I/M), charring model (β_0, β_n, t_ch, notional charring), **reduced cross-section** d_ef, b_ef.
- Verify **fire ULS** capacity of charred section (bending, shear, compression) at elevated k_mod,fi / γ_M,fi — not “remaining depth > char depth”.

### 3.3 EN 1995-2

- Bridge traffic load model, lanes, dynamic factor.
- ULS for bridge deck/girders, **fatigue** per Annex A (Δσ–N, load cycles, stress range), SLS vibration (§7) with DE-specific comfort criteria where NA differs.
- Separate bridge members from building members in the same project.

**Relationships:** `load combination` → `member` → `section` + `material` + `service class`; `connection` links members with fastener groups; `fire zone` overrides section for 1-2 checks.

---

## 4. Check catalogue

| # | Part | Clause / eq. | Verified today | Required inputs (current) | Limit source | Failure meaning |
|---|------|--------------|----------------|---------------------------|--------------|-----------------|
| 1 | 1-1 | §6.1.6 | M_Ed/M_Rd ≤ 1 | m_ed, w, f_m,k, SC, duration, m_crit, annex | k_mod·k_crit·W·f_m,k/γ_M | Bending ULS exceeded |
| 2 | 1-1 | §6.1.4 | N_Ed/N_Rd ≤ 1 | n_ed, a, f_c,0,k, SC, duration | k_mod·A·f_c,0,k/γ_M | **Compression only — no k_c buckling** |
| 3 | 1-1 | §6.1.7 | τ/f_v,d ≤ 1 | v_ed, b, h, f_v,k, annex | f_v,d; k_cr EN 0.67 / DE min(1,2.5/f_v,k) | Shear ULS exceeded |
| 4 | 1-1 | §8.1.2 | F_Ed/F_Rd ≤ 1 | f_ed, a_ef, f_v,k† | k_mod·A_ef·f_v,k/γ_M | **Wrong strength — bearing not shear** |
| 5 | 1-2 | §4.2 | remaining/charred ≤ 1‡ | fire_duration, section_depth | β₀=0.65 mm/min | **Semantically wrong — not fire capacity** |
| 6 | 2 | §6.1.6 (reuse) | Bridge M_Ed/M_Rd | same as #1 | same; annex forced EN | Bridge bending |
| 7 | 2 | §7 | a_vert/0.7 ≤ 1 | a_vert_m_s2 | A_VERT_LIMIT=0.7 m/s² (EN) | Comfort exceeded |
| 8 | 2 | Ann. A†† | M_Ed/(M_Rd·k_fat) ≤ 1 | m_ed, m_rd, n_cycles | k_fat surrogate | Fatigue surrogate exceeded |
| — | 1-1 | §6.1.5 | **Missing** | f_t,0,k, N_t,Ed | k_mod·A·f_t,0,k/γ_M | — |
| — | 1-1 | §6.1.5 | **Missing** | f_c,90,k, k_c,90, F_c,90,Ed | k_mod·k_c,90·A·f_c,90,k/γ_M | — |
| — | 1-1 | §6.2.x | **Missing** | σ_m, σ_t, σ_c combined | interaction formulas | — |
| — | 1-1 | §6.3.2 | **Missing** | λ, k_c | Eq. 6.26 | — |
| — | 1-1 | §7.2 | **Missing** | w_inst, w_net, w_fin, k_def | span/deflection limits | — |
| — | 1-1 | §7.3.3 | **Missing** (buildings) | floor freq, mass | DE NA comfort | — |
| — | 1-1 | §8 | **Missing** | Johansen modes | EYM | — |
| — | 1-2 | §4+Ch.6 | **Missing** | d_ef, fire k_mod,fi | charred section ULS | — |

† Uses f_v,k for bearing resistance. ‡ Inverted criterion. †† Not real Annex A.

---

## 5. Remediation strategy (per implemented check)

Core must gain `Remediation { field_path, current, required, formula, localized_text }`. Below: analytic target for Wave C.

| Check | Remediation target field(s) | Target value (solve u=1) |
|-------|----------------------------|--------------------------|
| Bending §6.1.6 | `m_ed_knm` ↓ or `w_mm3` ↑ or `f_m_k` ↑ or `m_crit_knm` ↑ (LTB) | `m_ed ≤ k_mod·k_crit·W·f_m,k/(γ_M·10⁶)` kN·m; or `w_mm3 ≥ m_ed·γ_M·10⁶/(k_mod·k_crit·f_m,k)` mm³ |
| Compression §6.1.4 | `n_ed_kn` ↓ or `a_mm2` ↑ or `f_c_0_k` ↑ | `n_ed ≤ k_mod·A·f_c,0,k/(γ_M·1000)` kN |
| Shear §6.1.7 | `v_ed_kn` ↓ or `b_mm`/`h_mm` ↑ or `f_v_k` ↑ | Solve τ=1.5·V·10³/(k_cr·b·h) ≤ k_mod·f_v,k/γ_M |
| Connection §8 | `f_ed_kn` ↓ or `a_ef_mm2` ↑ or **correct** `f_c,90,k` | Replace model first; then `f_ed ≤ k_mod·A_ef·f_c,90,d` |
| Fire §4.2 | `section_depth_mm` ↑ or `fire_duration_min` ↓ | After proper model: `d_ef ≥ d_min` for M_Rd,fi ≥ M_Ed,fi |
| Bridge bending | same as bending | Same formulas on bridge member |
| Vibration §7 | `a_vert_m_s2` ↓ or stiffness/mass | `a_vert ≤ 0.7` m/s² (verify DE NA limit) |
| Fatigue | `n_cycles_bridge` ↓ or section ↑ | Replace with Annex A Δσ–N; remediate stress range or detail category |

**Report text pattern (en/de):** “§6.1.6 bending: utilization 1.63 — reduce M_Ed from 25.0 to ≤ 15.4 kN·m, or increase W from 1.0×10⁶ to ≥ 1.63×10⁶ mm³ (GL28, SC1, medium, DE).”

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| Pass/fail visible | Yes — status + utilization in results/viewer/inspection |
| What complies | Only implicit (Pass); no grouped summary by part |
| How to comply | **Absent** — no remediation field or inversion |
| Localization | Actions/mode labels en+de (`✏️editor/🦀️.rs:248`); **report columns, messages, inspection labels English-only** (`🖥️app-surface/🦀️.rs:159`, `246–249`) |
| Subject editing | Raw JSON (`📥️inputs/🦀️.rs:20`); no typed forms, units, or strength-class pickers |
| Field coverage | All 20 scalars editable via JSON/mutations; **no UI for missing domain** (combinations, connections, deflection) |
| Catalogue | Placeholder (`📚️catalogue/🦀️.rs:3`) |
| Example i18n | Footbridge label not German (`📚️examples/🌉️glulam-footbridge/🦀️.rs:7`) |

---

## 7. Target design

### 7.1 Snapshot sketch (Rust-ish)

```rust
pub struct En1995Project {
    pub annex: AnnexChoice,
    pub service_class: ServiceClass,      // enum, not String
    pub members: Vec<TimberMember>,
    pub connections: Vec<TimberConnection>,
    pub load_combinations: Vec<LoadCombination>,
    pub fire_zones: Vec<FireZone>,        // 1-2
    pub bridge: Option<BridgeModel>,      // 2
}

pub struct TimberMember {
    pub id: String,
    pub material: TimberMaterialSpec,     // class GL28h or declared
    pub section: Section,                   // b, h, A, W, I, J
    pub effective_lengths: EffectiveLengths,
    pub design: MemberDesignEffects,        // M_Ed, V_Ed, N_Ed, deflections
}

pub struct TimberConnection {
    pub id: String,
    pub fastener: FastenerSpec,           // dowel, nail, screw
    pub geometry: JohansenGeometry,
    pub design: ConnectionDesignEffects,
}
```

### 7.2 `evaluate()` structure

```
evaluate(project) -> CheckReport
  for member in applicable_members:
    km = k_mod(member.sc, combo.duration)
    push bending, tension, compression(k_c), shear(k_cr), combined, deflection(k_def)
  for conn in connections:
    push johansen_modes (a..j)
  for member in fire_exposed:
    push fire_reduced_section_uls (β₀/β_n, d_ef)
  if project.bridge:
    push bridge_uls, fatigue_annex_a, pedestrian_vibration
  attach_remediation_for_each_fail()
```

### 7.3 Example subjects

**Compliant (DE glulam beam SC1):** GL28h 200×400 mm, L=6 m, M_Ed=28 kN·m, V_Ed=18 kN, SC1/permanent+medium combo, LTB M_cr=120 kN·m → all 1-1 ULS u < 1.

**Non-compliant (multi-fail):** Same beam with M_Ed=45, V_Ed=35, N_Ed=80 kN (slender column L_ef=3 m), a_vert=1.2 m/s², R60 fire on 300 mm depth → fails bending, shear, compression buckling, vibration, fire capacity.

### 7.4 Numeric worked-example tests (required)

1. **k_mod SC1 permanent = 0.6** — already `🧪️tests/⚖️compliance/🦀️.rs:4–6`.
2. **DE shear u=0.2438 @ V=15 kN** — already L66–77.
3. **Bending remediation:** W=1.8×10⁶ mm³, M_Ed=25 → u<1 (`💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:5–7`); add exact u=0.97 assert.
4. **New:** GL28 f_m,k=28, k_c buckling λ=1.2 → k_c from Eq. 6.26, N_Rd vs N_Ed.
5. **New:** Johansen Mode F double shear — compare one mode to hand calc.
6. **New:** Fire R60 d_ef=261 mm — M_Rd,fi vs M_Ed,fi (replace bogus remaining/charred).
7. **New:** Deflection w_fin = w_inst·(1+k_def) vs limit span/300.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / snapshot | `🧬️schema/🦀️.rs`, `📸️snapshot/*`, `🔺️diff/*`, all facets (ts/graphql/json/proto) |
| Compliance | `🧬️schema/🦀️.rs` (part modules), `💡️inferences/🦀️.rs` |
| Mutations | Re-derive from new shape; retire flat scalar kinds |
| Editor | `📥️inputs/` (typed forms), `📊️results/`, `🔍️inspection/` |
| Viewer | `📊️report/` |
| Examples | `📚️examples/`, `🖼️assets/` |
| Tests | `🧪️tests/⚖️compliance/`, `💡️inferences/🧪️tests/`, new evaluate oracle |
| Core (Wave B) | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` (remediation + i18n columns) |
| Oracles | `🔮️oracles/🔣️.json` — add compliance reference |

---

## 8. Risks / open questions

1. **Scope of one artifact:** Single flat document cannot serve buildings + bridges + connections — split profiles (`building`, `bridge`) or one graph with optional facets?
2. **EN 1990 coupling:** ULS combinations, ψ factors, k_mod duration per action — import from `⚖️en1990` or duplicate?
3. **Strength class catalogue:** Ship EN 338/EN 14080 tables in-plugin vs user-declared only?
4. **DE NA completeness:** Only `k_cr` differentiated; confirm no other DIN EN 1995-1-1/NA amendments (e.g. γ_M, deflection limits, vibration).
5. **Fire 1-2 depth:** Full thermal+mechanical vs reduced-section tabular approach for MVP?
6. **Johansen scope:** All dowel types in v1 or nails/screws subset first?
7. **Compile blocker:** `🧪️tests/🪵️mutate-en1995-1/🦀️.rs:41` — mutation parity unverified until `semio-s-plugin-norm` builds.
8. **Third-party compliance oracle:** Mutation oracle discharged; **no external EN 1995 calculation reference** registered — Wave D needs hand-worked benchmarks or certified tool comparison.

---

*Audit: read-only, 2026-09-26. Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995`.*
