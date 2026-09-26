# Audit — EN 1990 (`⚖️en1990`)

**Executive summary.** The EN 1990 artifact is a **scalar toy model** (6 snapshot fields + a composed variable-action table) that runs **~20 generic Ed ≤ R checks** against a single `resistance_kn` capacity. Combination arithmetic for 6.10a/6.10b, SLS ψ combinations, accidental γ=1, and 6.12b is **partially correct and numerically tested**, but Eq. **6.10 is a documented surrogate**, **β is hardcoded 3.9** (CC3 always fails reliability; CC never affects β), **`evaluate()` omits transient/seismic combination sets and Eq. 6.11**, and **EQU/STR/GEO/FAT, K_FI, γ sets A1.2(A/B/C), Annex A1.4 SLS deformation/vibration, and γM material checks are absent**. The editor exposes **raw JSON**, reports are **English-only tables with no remediation**, and the shipped CC3 example **cannot pass** the reliability check. Feature-complete compliance assessment requires a full basis-of-design subject, faithful NA tables, governing combination logic, and localized remediation output — none of which exist today.

Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990`  
Shared core: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs` (`CheckResult` has no remediation/localization/subject ref)  
App surface: `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`

---

## 1. Inventory

### 1.1 Snapshot / document fields

| Field | Type | Unit / domain | Notes |
|-------|------|---------------|-------|
| `g_k` | `f64` | kN (permanent action resultant) | Single lumped G; no favourable/unfavourable split |
| `q_k` | composed `s.stdio.semio/table` child | rows: `category: String`, `value: f64` kN | `En1990QkEntry`; read via `en1990_qk()` (`🦀️.rs:109–111`) |
| `resistance_kn` | `f64` | kN | Stand-in for entire structural resistance R_d |
| `consequence_class` | `u8` | 1–3 (CC1–CC3) | Drives β **target only**; not K_FI |
| `annex` | `AnnexChoice` | `en` / `de` | Selects `NaEn` / `NaDe` |
| `seismic_a_ed_kn` | `f64` | kN | A_Ed for Eq. 6.12b; doc says 0 disables — **check still runs** |

Source: `…/🧬️schema/📸️snapshot/🦀️.rs:20–36`, `…/🧬️schema/🦀️.rs:16–31`.

**Default snapshot** (`📸️snapshot/🦀️.rs:87–91`): G=100 kN, Q=[office 50, wind 30], R=300 kN, CC2, DE, A_ed=40 kN.

### 1.2 Composed children

- `q_k` → `En1990QkChild` (`store::ArtifactChild<SemioTableSnapshot>`)
- Converters: `en1990_qk_table_from_entries` / `en1990_qk_entries_from_table` (`🦀️.rs:47–76`)
- Working scene: `En1990QkWorkingTable` (`🦀️.rs:83–111`)

### 1.3 Mutations (10 kinds)

`change-annex`, `change-permanent-action`, `change-resistance`, `change-consequence-class`, `change-seismic-action`, `insert/remove/reorder-variable-action`, `change-variable-action-category/value` — `…/🧬️mutations/🦀️.rs:61–72`.

Each has diff + inverse (undo only, no compliance inversion).

### 1.4 `evaluate()` call graph

```
evaluate(document)                                    💡️inferences/🦀️.rs:85–96
├── action_set_from_document                          :80–82
├── NationalAnnexes ← annex choice                    :89
├── append_combination_set(Persistent)                :91
│   └── check_combination_set                         schema/🦀️.rs:550–577
│       ├── check_combination × N                     :544–547
│       │   ├── combination_uls (ULS rules)           :451–464
│       │   └── combination_value (SLS rules)         :495–504
│       └── rules_for_situation                       :507–518
├── append_combination_set(Accidental)                :92
├── check_seismic_situation → combination_6_12b     :93, :628–641
└── check_reliability_index(3.9, consequence_class)   :94, :597–609
```

**Helpers defined but NOT reached by `evaluate()`:**

| Function | Location | Why it matters |
|----------|----------|----------------|
| `check_design_basis` | `schema/🦀️.rs:619–626` | Adds **Seismic** `check_combination_set` + same hardcoded β |
| `check_uls_action` | `:580–583` | Alternate 6.10 entry |
| `combination_6_10` (true max) | `:424–427` | Surrogate only |
| `DesignSituation::Transient` | — | Never appended |
| `LimitState::Als` / `Fls` | `rules_for_situation` `:516–517` | Never invoked (no ALS/FLS checks) |
| `gamma_m` / `gamma_r` on `NationalAnnex` | `:288–299`, `:340–350` | Never used in combinations |

### 1.5 Editor / viewer

| Surface | File | Capability |
|---------|------|------------|
| Inputs window | `✏️editor/…/📥️inputs/🦀️.rs:19–21` | **Pretty-printed JSON** of full snapshot — not field editors |
| Results window | `✏️editor/…/📊️results/🦀️.rs:22–24` | Virtualized list via `app_surface::render_report` |
| Inspection panel | `✏️editor/📌️panels/🔍️inspection/🦀️.rs:19–20` | Single check: clause, status, u, message (EN) |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs:30–33` | Table: Clause, Status, Utilization, Message |
| Catalogue | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:3–5` | **Placeholder headline** |
| Example picker | `📚️examples/🏢️high-consequence-office/🦀️.rs` | One example; label **not DE-localized** (`:7`) |

Report columns (`🖥️app-surface/🦀️.rs:159–166`): no computed/limit quantities, no annex column in table, no remediation.

### 1.6 Examples / assets / tests / oracles

- **Examples:** 1 — `high-consequence-office` (CC3, EN annex, seismic off). DSL: `🖼️assets/…/🗣️.dsl.semio` (G=250, R=420, CC3, 3 Q entries via child ref).
- **Oracles:** `🔮️oracles/🔣️.json` (mutation catalog only).
- **Tests:** 38 `🦀️.rs` test modules. Strong **unit** coverage for combination formulas (`🧪️tests/⚖️compliance/🦀️.rs`). Weak **evaluate** coverage (2 tests, count + one numeric). Example test: `text.len() > 8` only (`📚️examples/…/🦀️.rs:4`).

---

## 2. Stub / fake detection

| # | Issue | Evidence |
|---|-------|----------|
| S1 | **β hardcoded 3.9** — not computed from subject | `💡️inferences/🦀️.rs:94`, `schema/🦀️.rs:624` |
| S2 | **Eq. 6.10 surrogate = 6.10a** (not max(6.10a, 6.10b)) | `schema/🦀️.rs:424–427`; `combination_6_10` calls `combination_6_10a` |
| S3 | **`check_reliability_index` ignores inputs except CC target** — computed β always 3.9; CC3 **always fails** (3.9 < 4.3); CC1/CC2 **always pass** regardless of structure | `:597–609`; CC3 example + hardcoded 3.9 |
| S4 | **`resistance_kn` is fake capacity** — one scalar vs all combinations; no members, materials, partial factors on R | All `check_combination` calls |
| S5 | **`evaluate()` ≠ `check_design_basis()`** — skips seismic **combination set** and transient | Compare `:85–96` vs `:619–626`; compliance test `:122` covers `check_design_basis` only |
| S6 | **Seismic “disabled” at 0.0 not honoured** — `check_seismic_situation` always runs; default A_ed=40 | `📸️snapshot/🦀️.rs:33–35`, default `:90` |
| S7 | **DE ξ for permanent equals EN (0.85)** — coordinator finding confirmed; DE **does** differ for accidental/seismic (1.0) but that only affects 6.10b/uls path | `NaDe::xi` `:301–306` vs `NaEn::xi` `:353–355` |
| S8 | **γM material factors unused** | `NaDe::gamma_m` `:288–295` never in combination pipeline |
| S9 | **`check_reliability_index` annex hardcoded `En`** even for DE documents | `schema/🦀️.rs:607` |
| S10 | **Unknown Q categories → default ψ** (e.g. example `"partition-walls"`) | `psi_row_de` default `:235`; example `reference_snapshot` `:25` |
| S11 | **No Eq. 6.11** accidental combination | No symbol in codebase |
| S12 | **No EQU/STR/GEO/FAT limit states** in checks | `LimitState` exists in core; EN1990 never branches on it |
| S13 | **No K_FI** — mutation comment mentions it (`⚠️change-consequence-class/🧪️…/🦀️.rs:3`) but no implementation | — |
| S14 | **No Annex A1.4** deformation/vibration SLS | — |
| S15 | **Report/UX stubs** — JSON inputs, EN-only messages, no remediation | `app-surface/🦀️.rs:159–254` |
| S16 | **Catalogue placeholder** | `📚️catalogue/🦀️.rs:3–5` |
| S17 | **27 `📌️.empty.md` folders** under editor/viewer | e.g. `✏️editor/🎚️config/📌️.empty.md` |
| S18 | **Weak tests** — `outline.check_count > 0` (`🧾outline/🧪️…/🦀️.rs:18`); example `len > 8` | No end-to-end fail/pass + remediation test |
| S19 | **`check_uls_action` dead** for evaluate | Never called from `evaluate` |
| S20 | **6.10 check uses `combination_uls` with rule Uls610 → still 6.10a surrogate** | `:545`, `:424–427` |

---

## 3. Complete subject definition (norm engineering target)

EN 1990 governs the **basis of structural and constructional design**. A feature-complete artifact must model:

### 3.1 Project / basis-of-design envelope

- **Design project:** title, codes, national annex, design working life, reference period.
- **Consequence class CC1–CC3** with **K_FI** partial-factor adjustments (DIN EN 1990/NA Table NA.A.1.2(B) note / Annex B).
- **Reliability differentiation:** target β (Annex C) **as output of reliability analysis** or explicit design parameters — not a constant.
- **Design situations:** persistent, transient, accidental, seismic (Table A1.1) — each with applicable combinations.
- **Limit states:** ULS (**EQU, STR, GEO, FAT**), SLS (deformation, vibration per **Annex A1.4**), ALS, FLS where claimed.

### 3.2 Actions (EN 1990 §4, EN 1991 linkage)

- **Permanent actions G:** separate favourable / unfavourable components; prestress P; soil permanent.
- **Variable actions Q:** typed by EN 1991 category (A–H + special: wind, snow, temperature, …) with **ψ_0, ψ_1, ψ_2** from Table A1.1 / NA.A.1.1.
- **Accidental actions A** (Eq. 6.11): explicit accidental action + accompanying ψ₁/ψ₂ rules.
- **Seismic actions A_ed** (Eq. 6.12 / 6.12a–c): magnitude, direction, ψ for accompanying.
- **Action groups / simultaneous load patterns** — which Q can lead together.

### 3.3 Structural system (verification target)

- **Structural model:** members, joints, supports (or explicit “resultant check” zone with traceability).
- **Resistances R:** material (concrete, steel, timber, …) with **γM** from Table A1.2(A/B/C) and NA overrides.
- **SLS response quantities:** deflections δ, frequencies f, accelerations a — with **Annex A1.4** limits (span/300, etc.).
- **Geotechnical / equilibrium** checks where GEO/EQU govern (separate γ sets).

### 3.4 Relationships

```
Project → DesignSituations[] → CombinationRules[] → ActionEffects(Ed)
        → LimitStates[] → Verifications(Ed vs Rd or Ed vs limit)
        → ReliabilityClass → K_FI, β_target
NationalAnnex → {γG, γQ, γM, ξ, ψ tables, recommended combinations}
```

Current artifact collapses this to **`ActionSet { g_k, q_k[] }` + `resistance_kn`**.

---

## 4. Check catalogue

| Part | Clause / eq. | Verified today? | Required inputs | Limit source (DE-NA vs EN) | Failure meaning |
|------|--------------|-----------------|-----------------|----------------------------|-----------------|
| ULS persistent | §6.4 **6.10** | ⚠️ Surrogate 6.10a | G, Q[], leading, R | γG=1.35, γQ=1.5 (both NA) | Ed > R |
| ULS persistent | **6.10a** | ✅ | same | same | Ed > R |
| ULS persistent | **6.10b** | ✅ | same + **ξ** permanent: DE/EN **0.85** | same | Ed > R |
| ULS transient | 6.10* | ❌ not in evaluate | situation flag | NA γ, ξ | — |
| ULS accidental | **6.11** | ❌ | A_acc, ψ_accompanying | γ=1.0 | — |
| ULS accidental | 6.10a γ=1 | ✅ in accidental set | G, Q[] | unit γ (`:411–412`) | Ed > R |
| ULS seismic | **6.12** (general) | ❌ | seismic model | NA | — |
| ULS seismic | **6.12b** | ✅ single check | G, A_ed, ψ₂Q, R | ψ₂ from NA table | Ed > R |
| ULS seismic | combination set | ❌ in evaluate | same as persistent rules γ=1 | — | — |
| SLS | **6.14** char | ✅ | G, Q, ψ₀ | ψ₀ NA.A.1.1 | Ed > R (mislabeled — should be service limit) |
| SLS | **6.16** frequent | ✅ | ψ₁, ψ₂ | NA table | same |
| SLS | **6.17** quasi-perm | ✅ | ψ₂ | NA table | same |
| SLS | **Annex A1.4** deflection | ❌ | δ, span, use | EN recommended / NA | — |
| SLS | **Annex A1.4** vibration | ❌ | a, f, use | EN / NA | — |
| Reliability | **Annex C** β | ⚠️ fake | **computed β** | CC1 3.1, CC2 3.8, CC3 4.3 | β < β_target |
| Partial factors | **Table A1.2(A/B/C)** | ❌ (only one γ) | limit state, material | DE may differ on γM sets | — |
| Consequence | **K_FI** | ❌ | CC | NA B.3 | — |
| ψ tables | **A1.1 / NA.A.1.1** | ✅ partial | category string | DE: extra `snow_high`; `other` ψ differs DE 0.8 vs EN 0.7 (`:231–254`) | wrong Ed |
| EQU/STR/GEO/FAT | §2 | ❌ | action effect type | separate γ in A1.2 | — |

**Numeric spot-check (DE, sample_actions G=100, Q_office=50, Q_wind=30, leading=0):** 6.10a Ed=237 kN (`🧪️compliance/🦀️.rs:22`), 6.10b Ed=216.75 (`:30`), accidental 6.10a Ed=168 (`:111`), 6.12b with A_ed=40 → Ed=155 (`compliance-report/🦀️.rs:22`).

---

## 5. Remediation strategy (per check — target design)

Shared pattern: extend `CheckResult` (Wave B) with `remediation: Vec<RemediationStep { field_path, current, required, locale }>`.

| Check | Remediation computation | Target field(s) |
|-------|-------------------------|-----------------|
| ULS 6.10a/6.10b (Ed > R) | Solve R_required = Ed_governing; or reduce actions: e.g. lower leading Q_k until Ed ≤ R | `resistance_kn` or `q_k[i].value` / `g_k` |
| Governing 6.10b vs 6.10a | Report which rule governs; if 6.10b: suggest **increase R by (Ed_6.10b − R)** or reduce G by `(Ed−R)/(ξ·γG)` | same |
| SLS 6.14–6.17 | Invert combination for Q or δ limit (once SLS limits are real quantities, not R) | service limits struct |
| 6.12b seismic | `R ≥ G + A_ed + Σψ₂Q`; disable if `seismic_a_ed_kn=0` → N/A | `resistance_kn`, `seismic_a_ed_kn` |
| Reliability β | **Do not hardcode** — either omit until probabilistic model exists, or `β_computed` from subject; remediate CC or design parameters | `consequence_class`, design life |
| Wrong ψ category | “Set category to `{valid}` or override ψ in NA table row” | `q_k[i].category` |
| CC3 example fail | “Increase reliability: target β=4.3, computed β=3.9 — adjust consequence class or perform reliability analysis” | `consequence_class` / reliability fields |

Example message (DE): *„Erhöhen Sie `resistance_kn` von 300 kN auf ≥ 237 kN (EN 1990 §6.4 6.10a, führende Variable Büro)“*.

---

## 6. Report & UX gaps

| Requirement | Status |
|-------------|--------|
| Shows what complies / fails | Partial — Pass/Fail + utilization in list |
| **How to comply** | **Missing** — no remediation |
| Localized en + de | Window **titles** only; check `message` English (`"ULS 6.10a leading=0"`, `"reliability index β"`) |
| All subject fields editable | **No** — JSON dump only |
| Readable report | Table OK; inspection panel minimal; **no grouping by situation/limit state** |
| CC3 example UX | Opens example that **fails reliability by construction** — confusing demo |

---

## 7. Target design

### 7.1 Proposed snapshot (sketch)

```rust
pub struct En1990Snapshot {
    pub annex: AnnexChoice,
    pub consequence_class: ConsequenceClass, // typed enum
    pub design_working_life_years: u32,
    pub situations: Vec<DesignSituationSpec>, // persistent | transient | accidental | seismic
    pub permanent: PermanentActions,         // g_unfav_kn, g_fav_kn, prestress_kn
    pub variable: Vec<VariableAction>,       // category: ImposedCategory, q_kn, psi_override?
    pub accidental: Option<AccidentalAction>,
    pub seismic: Option<SeismicAction>,      // a_ed_kn, disabled flag
    pub verifications: Vec<VerificationCase>, // { limit_state, situation, rule, ed_kn, rd_kn OR sls_limit }
    pub reliability: ReliabilitySubject,     // beta_target, beta_computed OR explicit "not evaluated"
}
```

Keep `q_k` composed table or migrate rows to typed `VariableAction` children.

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &En1990Snapshot) -> CheckReport {
    let annex = annex_for(doc.annex);
    for situation in active_situations(doc) {
        for rule in governing_rules(situation, limit_states(doc)) {
            for leading in leading_indices(&doc.variable) {
                push_combination_check(..., with_remediation);
            }
        }
        if situation == Accidental { push_eq_6_11(...); }
        if situation == Seismic { push_eq_6_12_family(...); }
    }
    push_sls_a14_deformation_vibration(...);
    if doc.reliability.is_evaluated() {
        push_beta_check(doc.reliability.computed, target_beta(doc.consequence_class));
    }
    apply_k_fi_adjustments_report(...);
}
```

Implement **true 6.10 = max(6.10a, 6.10b)** as governing check.

### 7.3 Example subjects

**Compliant (DE, CC2 office):** G=80, Q=[office 40, wind 20], R=250, A_ed=0, β_computed=3.85 ≥ 3.8, all Ed ≤ R.

**Non-compliant (multi-failure):** CC3 + R=200 kN + default Q → fails **6.10a** (Ed≈237), **reliability** (3.9<4.3), possibly **SLS frequent** — report lists three remediations.

### 7.4 Worked tests to add

1. `evaluate_governing_6_10_is_max_of_a_and_b` — assert governing Ed = max(237, 216.75) for sample_actions.
2. `evaluate_cc3_reliability_fails_with_remediation` — CC3 + hardcoded β documents expected fail until fixed.
3. `seismic_zero_is_not_applicable` — A_ed=0 → 6.12b check N/A, not Ed=G+Σψ₂Q.
4. `de_other_psi_differs_from_en` — already in compliance tests; wire through full `evaluate`.
5. `high_consequence_office_example_intentional_failures` — parse example DSL, assert specific failing clauses + numeric Ed.

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Schema / compliance | `…/🧬️schema/🦀️.rs`, `…/💡️inferences/🦀️.rs` |
| Snapshot facets | `…/📸️snapshot/{🦀️,🟦️,🔣️,🛰️/🔗️}.*` |
| Mutations | all 10 under `…/🧬️mutations/` + new fields |
| Editor inputs | `…/📥️inputs/` — structured editors per field |
| Viewer/editor report | `…/📊️results/`, `…/📊️report/`, `🖥️app-surface/🦀️.rs` |
| Examples | `📚️examples/` — add compliant + fix CC3 demo narrative |
| Tests | `🧪️tests/⚖️compliance/`, `💡️inferences/🧪️tests/🔬️compliance-report/` |
| Oracles | `🔮️oracles/🔣️.json` |
| Shared core | `⚖️compliance/🦀️.rs` (remediation model — Wave B) |

---

## 8. Risks / open questions

1. **Scope boundary:** Is EN 1990 artifact a **full BOD document** or a **combination calculator service** for downstream EN 199x artifacts? Current code is the latter masquerading as the former.
2. **β / K_FI:** Will reliability be **declared input** or **computed** from member-level data in EN 1992/1993 artifacts?
3. **Resistance scalar:** Retain as ** pedagogical resultant** with explicit disclaimer, or require linked structural artifact?
4. **DE NA fidelity:** Verify DIN EN 1990/NA:2012 γ, ξ, ψ, combination choice (6.10 vs 6.10a/b) against official PDF — code uses simplified single γ set.
5. **SLS vs ULS limit typing:** Using `resistance_kn` for SLS checks is **physically wrong** — needs separate service limits.
6. **Wave B dependency:** Remediation/localization blocked on core `CheckResult` extension.
7. **CC3 example:** Should it demonstrate **failure** with explanation, or be retuned to pass?
8. **`evaluate` vs `check_design_basis` divergence** — unintentional? Tests cover helper not production path.

---

*Audit date: 2026-09-26. Read-only; no codebase modifications.*
