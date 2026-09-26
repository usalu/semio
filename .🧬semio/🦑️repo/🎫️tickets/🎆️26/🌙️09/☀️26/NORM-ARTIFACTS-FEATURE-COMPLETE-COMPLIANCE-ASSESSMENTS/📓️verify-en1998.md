# Verify — EN 1998 (`en1998` / 🫨️) — Round 3

**Verifier:** adversarial read-only (Wave D)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Round 1:** FAIL (12 blocking)  
**Round 2:** FAIL (7 blocking) — `📓️verify-en1998.md` (prior)  
**Implementer claim:** `📓️impl-en1998.md` — 63/63, round-2 blockers closed  
**Test run:** `bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary line:** `Summary [   0.432s] 63 tests run: 63 passed, 0 skipped`  
**Logs:** `🗑️generated/verify-en1998/test-r3.txt`

---

**VERDICT: FAIL (2 blocking)**

---

## Round history

| Round | Verdict | Blocking count |
|-------|---------|----------------|
| R1 | FAIL | 12 |
| R2 | FAIL | 7 |
| R3 | **FAIL** | **2** |

---

## Round 2 → Round 3 delta (prior blockers)

| Round-2 blocker | R3 status | Evidence |
|-----------------|-----------|----------|
| 1 Seismic mass from G_k + ψ_E·Q_k (not hand `massKg` on storeys) | **FIXED** | `storeys[].permanentGkN` + `variables[]` (`🦀️.rs:En1998Storey`); `part_1::storey_seismic_mass_kg` (`🦀️.rs:201–213`); used in `evaluate` (`💡️inferences/🦀️.rs:166–168,464–466`) |
| 2 Parts 2–6 demands from spectrum + model (not hand `eDN`/`hEdN`/…) | **FIXED** | Bridge V from `S_d(T)·massKg` (`:727–728`); silo/tank impulsive+convective (`:784–815`); tower `V`+`M_ed` (`:909–913`); assessment `E_d` from `building_base_shear_n` (`:760`); foundation `H_ed=V_b` (`:864`) |
| 3 `multipleResistingSystems` read in real check | **FIXED** | `en1998.1.{bid}.multipleSystems` (`💡️inferences/🦀️.rs:256–294`) |
| 4 `assessments[].limitState` selects NC/SD/DL | **FIXED** | `part_3::limit_state_ag_factor` + explicit `limitState` check (`:750–773`) |
| 5 TS/GQL/proto facets regenerated | **FIXED** | Typed nested interfaces in `📸️snapshot/🟦️.ts`, `🔗️.graphql`, `🛰️.proto`; no `Record<string, unknown>` on snapshot types |
| 6 No `plan_w = 24.0` fallback | **FIXED** | `planDims` Fail+remedy when ≤0 (`:221–254`); torsion only when dims >0 (`:432–460`); `24.0` is example input only (`📸️snapshot/🦀️.rs:73`) |
| 7 Foundation sliding from supported-building `V_b` | **FIXED** | `h_ed = v_b` (`💡️inferences/🦀️.rs:864`) |

---

## Check table

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical snapshot; buildings use characteristic actions (`permanentGkN`, `variables[]`); parts 2–6 on discriminated entity types (`En1998Bridge`, `En1998Silo`, … in `🦀️.rs:368–485`). Parts 2–6 retain scalar `massKg` as seismic mass model (resistances only as capacity inputs) — acceptable for ancillary structures; building path uses EN 1990 combination. |
| 2 | Clause coverage | **PASS** | EN 1998-1 core (spectrum, LFM, q-limit, storey shear, drift, P-Δ, detailing, DE masonry); parts 2–6 scoped evaluate fns with N/A when lists empty (`:708–712`). |
| 3 | Numerics | **PASS** | Hand recomputation in `🗑️generated/verify-en1998/hand-numerics.txt`: T₁=0.543 s, S_e=0.864 m/s², S_d=0.148 m/s², m=1.05×10⁶ kg, F_b≈1.55×10⁵ N (<0.5 %). Automated: `de_na_br_spectrum_at_t1`, `t1_ct_and_base_shear_lambda` (`🧪️tests/⚖️compliance/🦀️.rs:10–40`). |
| 4 | Applicability | **PASS** | Empty scoped lists → `NotApplicable`; zone 0 → building N/A (`💡️inferences/🦀️.rs:140–161`). |
| 5 | National annex | **PASS** | DE zones 0–3 enum (`na_de::SeismicZone`, `🦀️.rs:86–108`); a_gR 0/0.4/0.6/0.8 Table NA.1; DE-NA ground A-R…C-S with sourced `(S,T_B,T_C,T_D)` (`🦀️.rs:153–162`); no zone 4; DE≠EN test (`🧪️tests/⚖️compliance/🦀️.rs:19–27`). |
| 6 | Report quality | **FAIL** | Paths use `[id=…]` + resolution test (`🧪️tests/⚖️compliance/🦀️.rs:129–144`). **Blocking:** ≥15 `explanation` blocks emit identical en/de strings (English wire tokens / numeric templates copied to both locales) — see 13:27 table cause 12. |
| 7 | Examples | **PASS** | `compliant_de_office` / `noncompliant_de_office` + DSL assets; `compliant_example_evaluates_clean`, `fail_example_has_expected_fails` (`🧪️tests/⚖️compliance/🦀️.rs:114–126`). |
| 7b | Inputs UX | **PASS** | `en1998_field_meta` with localized choices; `field_meta_covers_every_editable_leaf_of_default_snapshot` (`🏷️field-meta/🦀️.rs`; `🧪️tests/⚖️compliance/🦀️.rs:147–181`). Structured editor via B2 hook (`✏️editor/…/📥️inputs/🦀️.rs:14`). |
| 8 | Mutations & schema | **PASS** | Semantic domain mutations (`change-storey-permanent-gk-n`, `insert-bridge`, …); facets agree with Rust snapshot (`📸️snapshot/🔣️.json` anchor). |
| 9 | Tests | **FAIL** | 63 executed, 63 passed, 0 skipped (`test-r3.txt`). JSON-schema + oracle + remedy tests present. **Blocking:** no scope-aware perturbation test per CORRECTION 13:43 (only meta-coverage test at `:147`). |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in evaluate path; radiation damping computed (`💡️inferences/🦀️.rs:870–883`). |

---

## Mandatory family checks (DE / EN 1998-1)

| Requirement | Result | Notes |
|-------------|--------|-------|
| DE seismic zone enum 0–3, localized | **PASS** | `DeSeismicZone`; field-meta `ZONES` |
| No zone-4 Fail check | **PASS** | Enum + tests cap at zone3 |
| a_gR per zone 0/0.4/0.6/0.8 | **PASS** | `na_de::SeismicZone::a_gr` (`🦀️.rs:92–98`) |
| DE-NA subsoil A-R…C-S | **PASS** | `DeGroundCombo`; Table NA.4 params sourced in code |
| Importance γ_I | **PASS** | `part_1::gamma_i` |
| Zone 0 → N/A | **PASS** | `de_seismic_zone_is_schema_enum_0_to_3` |
| S_d(T), T₁, LFM, F_b+λ, storey distribution | **PASS** | Mass from `ΣG_k+ψ_E·Q_k` |
| Regularity §4.2.3 | **PASS** | Both remedies flip (`:100–111`) |
| q by type/ductility | **PASS** | `qLimit` check |
| Drift §4.4.3.2, P-Δ §4.4.2.2 | **PASS** | Hand-verified in `hand-numerics.txt` §3–4 |
| Ductility detailing for material | **PASS** | RC + steel rules |
| DE masonry simplified rules | **PASS** | When `claimsSimpleMasonry` |
| Remedies flip ≥2 distinct fails (test) | **PASS** | 3 remedy-law tests |
| Seismic mass from characteristic actions (buildings) | **PASS** | Round-2 blocker closed |
| Perturbation test for every editable leaf | **FAIL** | Absent — see 13:43 |

---

## CORRECTION 13:27 — twelve causes

| # | Cause | Result | Evidence |
|---|-------|--------|----------|
| 1 | `NormFieldChoice` human en+de labels | **PASS** | `🏷️field-meta/🦀️.rs:10–117` |
| 2 | Every editable leaf has meta + coverage test | **PASS** | `field_meta_covers_every_editable_leaf_of_default_snapshot` |
| 3 | Structured editor, not JSON dump | **PASS** | `render_document_editor` (`📥️inputs/🦀️.rs:14`) |
| 4 | `[id=…]` paths + resolve test | **PASS** | `emitted_remedy_paths_use_id_selectors_and_resolve` |
| 5 | ≥2 fail→pass remedy tests; applicable writable targets | **PASS** | 3 tests (`:69–111`); noncompliant example asserts remedies non-empty (`:65`) |
| 6 | Example decode + `complies()` / `fail_count ≥ 2` | **PASS** | `:114–126` |
| 7 | Python oracle ±0.5 % + third-party jsonschema | **PASS** | `:184–235`; oracle uses action-based mass (`🔮️oracles/🐍️.py:58–63`) |
| 8 | Facets regenerated; no snapshot `Record<string, unknown>` | **PASS** | `📸️snapshot/🟦️.ts` typed interfaces |
| 9 | No tautologies / hardcoded geometry / ignored fields | **PASS** | Round-2 items closed; static audit (`ignored-fields-audit.txt`) finds no unread non-label leaves |
| 10 | No trivially-true tests | **PASS** | No `\|\| true` in compliance tests |
| 11 | Semantic domain mutation verbs | **PASS** | `change-storey-permanent-gk-n`, `insert-bridge`, … (not CRUD) |
| 12 | Dynamic issue text localized (no `copy(x,x)`) | **FAIL** | Identical en/de `explanation` templates, e.g. `planRegular=…` (`💡️inferences/🦀️.rs:198–201`), `δ={delta:.3}…` (`:450`), `T=…Ved=…` (`:730`), `limitState=` (`:769`), `V={v:.0} N` (`:817`) — 30+ `explanation(lc(&format!(…), &format!(…)))` pairs with same format string |

---

## CORRECTION 13:43 findings

| Item | Result | Evidence |
|------|--------|----------|
| Perturbation test exists and covers every editable leaf in committed example scope | **FAIL** | No `perturb` / `editable_leaf` test in family tree; only meta walk at `🧪️tests/⚖️compliance/🦀️.rs:147` |
| Static audit snapshot fields vs `evaluate()` reads | **PASS** | `🗑️generated/verify-en1998/ignored-fields-audit.txt` — all field-meta leaves match read patterns in `💡️inferences/🦀️.rs` (incl. `correlatedOccupancy`, `variables[].category/qkN` via `part_1::storey_seismic_action_n`) |
| Kind-specific parts 2–6 on discriminated schema | **PASS** | Separate structs per part (`🦀️.rs:368–485`); not flat always-present scalars |
| Default example scope | **NOTE** | `compliant_de_office()` has empty `bridges`/`assessments`/… — perturbation test must still cover building leaves; parts 2–6 leaves require a second scoped example when those lists are populated |

---

## Prior-item re-check (R1 + R2)

| Item | Status | File:line |
|------|--------|-----------|
| R1-1 Field-meta localized choices | FIXED | `🏷️field-meta/🦀️.rs:10–85` |
| R1-2 Field-meta coverage test | FIXED | `🧪️tests/⚖️compliance/🦀️.rs:147–181` |
| R1-3 DE ground UX | FIXED | `site.deGroundCombo` sole DE control |
| R1-4 Positional `[index]` paths | FIXED | `id_sel()` + resolve test `:129–144` |
| R1-5 Regularity remedy both flags | FIXED | `:100–111` |
| R1-6 Tautological storeyForces | FIXED | `V_i` vs `shearResistanceN` `:397–428` |
| R1-7 Hardcoded plan_w 24 m | FIXED | No fallback; `planDims` check `:221–254` |
| R1-8 q-limit check | FIXED | `:337–363` |
| R1-9 Stale facets | FIXED | Typed `🟦️.ts` / `🔗️.graphql` / `🛰️.proto` |
| R1-10 ≥2 remedy-flip tests | FIXED | 3 tests `:69–111` |
| R1-11 Example evaluate tests | FIXED | `:114–126` |
| R1-12 JSON-schema test | FIXED | `:184–198` |
| R2-1 Actions-based seismic mass | FIXED | `🦀️.rs:201–213`, `💡️inferences/🦀️.rs:166–168` |
| R2-2 Spectrum-derived parts 2–6 demands | FIXED | `evaluate_bridges` … `evaluate_towers` |
| R2-3 `multipleResistingSystems` | FIXED | `:256–294` |
| R2-4 `limitState` NC/SD/DL | FIXED | `:750–773` |
| R2-5 Facet parity | FIXED | Snapshot facets |
| R2-6 Plan dimensions required | FIXED | `:221–254` |
| R2-7 Foundation sliding from V_b | FIXED | `:864` |

---

## Numerics (hand derivation — check 3)

See `🗑️generated/verify-en1998/hand-numerics.txt`. Summary for compliant DE office:

1. **Spectrum:** Zone 2, B-R → S_e(T₁)=0.864 m/s², S_d=0.148 m/s² (q=5.85).  
2. **Base shear:** m=1.05×10⁶ kg from G_k+ψ_E·Q_k, λ=1.0 → F_b≈1.55×10⁵ N (<0.5 % vs Rust).  
3. **Drift:** ductile θ=0.0075, ν=0.5, h=3.5 m → limit 0.0525 m; top drift 0.011 m → u≈0.21.  
4. **P-Δ:** θ≈0.21 (<0.3).

---

## Blocking fix list

1. **`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`** — Add `every_editable_leaf_perturbation_changes_report` (CORRECTION 13:43): for each editable leaf in the default building snapshot (and a second fixture with populated parts 2–6 entities), clone `En1998Snapshot`, perturb the leaf (typed delta: bool flip, ±10 % numeric, enum swap), call `inferences::evaluate`, assert ≥1 check’s `status` or `utilization` changes vs baseline. Exempt only descriptive `name`/`title`/`id` entity labels. Scope-aware: only perturb leaves present in the committed example under test.

2. **`🧬️schema/💡️inferences/🦀️.rs`** — Localize every `explanation` en/de pair (CORRECTION 13:27 cause 12): German strings must use engineering terminology, not English wire keys. Fix all identical `&format!(…), &format!(…)` copies (regularity `:198–201`, torsion `:450`, drift `:482`, P-Δ `:507`, parts 2–6 `:730,735,769,795,817,894,915`, etc.). Titles already localized; explanations must diverge.

---

## Non-blocking observations

- `python_oracle_matches_within_half_percent` soft-`continue` on parse errors (`🧪️tests/⚖️compliance/🦀️.rs:217–221`) — should hard-fail if oracle cannot parse schema.  
- Python oracle `C-S` spectrum params `(1.50,…)` disagree with Rust `(0.75,…)` (`🔮️oracles/🐍️.py:21` vs `🦀️.rs:161`) — not exercised by default B-R example but should align with Table NA.4.  
- No committed example with populated bridges/assessments/silos/tanks/foundations/towers — parts 2–6 checks only N/A in default path; add a multi-part example for E2E UX once perturbation test lands.  
- Parts 2–6 `massKg` is a direct seismic mass scalar (not decomposed into G_k/Q_k) — acceptable for ancillary structures but consider characteristic actions if full EN 1990 parity is desired later.  
- Impl claim “no gaps” remains overstated until perturbation test and de explanations land.

---

## Test runner transcript (excerpt)

```
Summary [   0.432s] 63 tests run: 63 passed, 0 skipped
NX   Successfully ran target test for project @semio-tech/norm-en1998-rs
```
