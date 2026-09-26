# Verify — EN 1994 (`🧩️en1994`) — Round 3

**Auditor:** read-only adversarial verification, 2026-09-26 (round 3)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1994.md` (72/72, no gaps, round-2 CORRECTION 13:43 closeout)  
**Prior verify:** R1 **FAIL (8)**, R2 **FAIL (5)**

**VERDICT: FAIL (6 blocking)**

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | FAIL | 8 — index paths, raw enums, tautological b_eff, missing SLS crack, surrogate LTB, incomplete DSL, no example verdict tests, no jsonschema in nx |
| R2 | FAIL | 5 — hand-typed M_Ed/V_Ed/N_Ed (no EN 1990 load cases); ignored studs.spacingM, columns.kind, tw/tf, sheeting.thicknessM |
| R3 | **FAIL** | 6 — crate does not compile (0 tests); TS facets `unknown[]`; leaf perturbation not scope-aware; missing SLS frequent combination; sls_char orphan; duplicated imposed load |

R2 round-1 items and extras A–D remain **PASS** (unchanged). R2 blocking #2–#5 are **FIXED** in source (evidence in re-check table). R2 blocking #1 is **partially fixed** (CharacteristicAction + `part_en1990` exist) but still **FAIL** on missing frequent SLS and duplicated imposed intensities.

---

## Test run

```
bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache -- --no-fail-fast
error[E0061]: this function takes 5 arguments but 3 arguments were supplied
  --> …/✏️editor/📌️panels/📚️catalogue/🦀️.rs:21
  crate::app_surface::render_catalogue(&examples, locale, controller_id)
note: function defined at …/🖥️app-surface/🦀️.rs:1218
  pub fn render_catalogue(examples, tables, locale, controller_id, windows)
error: could not compile `semio-s-artifact-norm-en1994` (lib) due to 1 previous error
```

**0 tests executed.** Impl claim 72/72 is unverified.

Log: `🗑️generated/verify-en1994/test-r3.txt`

---

## Check table (brief §1–10)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **FAIL** | Hierarchical beams/columns/slabs + `CharacteristicAction[]` with EN 1990 ULS/construction/SLS char+qp (`🧬️schema/🦀️.rs:264-441`, `💡️inferences/🦀️.rs:104-107`). Hand-typed beam M_Ed fields removed. **Missing:** SLS **frequent** (ψ₁) combination — no `sls_frequent` / freq check (`rg sls_freq` → 0). **Duplicated imposed load:** `default_beam_actions` sets both `qAreaPa` and `qLineNPerM` on Q-office (`🦀️.rs:327-328`); `action_internals` sums both (`schema/🦀️.rs:323`). |
| 2 | Clause coverage | **PASS** (caveats) | 1-1: beff, class, mrd, construction, spacing, prd, etamin, vpl, vlrd, ltb, deflection, crack; columns npl+mn; slabs §9; 1-2 fire; 2 bridge fatigue. Caveats: crack is min-A_s proxy; no freq SLS; stud fatigue γ_Mf hardcoded 1.0 (`schema/🦀️.rs:776-798`). |
| 3 | Numerics (≥3 hand checks) | **PASS** (unverified run) | Hand derivations in `🗑️generated/verify-en1994/hand-numerics.txt`: b_eff=2.30 m, P_Rd=81.656 kN, η_min=0.49, n_L=3.030, CFST N_pl≈3939 kN. Unit tests exist (`🧪️tests/⚖️compliance/🦀️.rs:9-55`) but did not execute. |
| 4 | Applicability | **PASS** (unverified) | LTB N/A propped (`inferences/🦀️.rs:328-334`); building fatigue N/A (`624-628`); fire rating gate (`582-587`). |
| 5 | National annex | **PASS** (unverified) | DE γ_Mf 1.35 vs EN 1.15 with comments citing EN 1993-1-9/NA (`schema/🦀️.rs:227-248`); tests `de_vs_en_bridge_fatigue_gamma_mf`, `de_bridge_gamma_mf_stricter_than_en`. Stud Δτ fatigue ignores annex (GAMMA_MF_S=1.0). |
| 6 | Report quality | **PASS** (unverified) | `[id=…]` paths, en+de copy, governing combo labels on ULS/deflection (`inferences/🦀️.rs:165-166, 350-351`); remedy-flip tests present in source. |
| 7 | Examples | **PASS** (unverified) | Three DSL assets; verdict tests `passing_example_dsl_complies`, `failing_example_dsl_does_not_comply_with_named_ids`, `bridge_example_runs_fatigue_checks`. |
| 7b | Inputs UX | **PASS** (unverified) | `🏷️field-meta/🦀️.rs` full table; `every_default_leaf_has_en_de_field_meta`. |
| 8 | Mutations & schema | **FAIL** | Semantic mutations present. TS snapshot facets still `beams: unknown[]` etc. (`🧬️schema/🟦️.ts:5-7`, `📸️snapshot/🟦️.ts:5-7`) — CORRECTION 13:27 #8. |
| 9 | Tests | **FAIL** | Compile error; 0 executed. Cannot confirm oracle, jsonschema, remedy law, perturbation. |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!`; `default_placeholder()` only for insert mutations. |

---

## Round-1 / R2 item re-check

| Item | R2 | R3 | Evidence |
|------|-----|-----|----------|
| R1 #1 stable `[id=…]` paths | PASS | **PASS** | `subject_paths_use_stable_ids_and_survive_reorder` (source) |
| R1 #2 en+de field meta | PASS | **PASS** | `🏷️field-meta/🦀️.rs`, `every_default_leaf_has_en_de_field_meta` |
| R1 #3 real b_eff | PASS | **PASS** | `uncapped_effective_width_m` vs spacing (`inferences/🦀️.rs:102-125`) |
| R1 #4 SLS cracking | PASS | **PASS** | `en1994.7.4.crack.*` (`inferences/🦀️.rs:359-383`) |
| R1 #5 LTB from construction/hogging | PASS | **PASS** | `uls_c` / `uls.m_hog_nm` (`inferences/🦀️.rs:302-327`) |
| R1 #6 failing + bridge DSL RECs | PASS | **PASS** | Failing + bridge DSL assets include nested RECs |
| R1 #7 example verdict tests | PASS | **PASS** | `compliance-report/🦀️.rs:177-204` |
| R1 #8 jsonschema in nx | PASS | **UNVERIFIED** | Test exists; crate won't compile |
| Extras A–D | PASS | **PASS** | one_of designation, column PNA polygon, fire table, oracle test (source) |
| **R2 #1 EN 1990 structural actions** | FAIL | **FAIL** | Actions + combinations added; missing **freq** SLS; `sls_char` dead (`inferences/🦀️.rs:363`); duplicate Q-office intensities (`🦀️.rs:327-328`) |
| **R2 #2 studs.spacingM** | FAIL | **FIXED** | `studs_in_shear_span`, spacing check (`inferences/🦀️.rs:206-213`; `schema/🦀️.rs:597-599`) |
| **R2 #3 columns.kind** | FAIL | **FIXED** | `n_pl_rd_n`, `confinement_factors`, `local_buckling_util` (`schema/🦀️.rs:817-855`) |
| **R2 #4 steel.twM/tfM** | FAIL | **FIXED** | Section class, shear area, buckling (`inferences/🦀️.rs:129-144, 271-282`; `schema/🦀️.rs:613-631`) |
| **R2 #5 sheeting.thicknessM** | FAIL | **FIXED** | `sheeting_kt`, `a_p_m2_per_m`, slab checks (`schema/🦀️.rs:503-509, 649`; `inferences/🦀️.rs:460-472`) |

---

## CORRECTION 13:27 (12 causes)

| # | Cause | R3 |
|---|--------|-----|
| 1 | Human enum labels | **PASS** (source) |
| 2 | Every editable leaf has meta | **PASS** (source) |
| 3 | Structured editor | **PASS** (field-meta wired) |
| 4 | `[id=…]` paths + resolve test | **PASS** (source) |
| 5 | ≥2 remedy-flip tests | **PASS** (source; unexecuted) |
| 6 | Example comply / fail ≥2 | **PASS** (source; unexecuted) |
| 7 | Oracle ±0.5 % + jsonschema | **FAIL** — tests won't compile |
| 8 | Facets regenerated | **FAIL** — `unknown[]` on beams/columns/slabs |
| 9 | No tautologies / ignored fields | **FAIL** — sls_char orphan; duplicate Q load; perturbation gaps |
| 10 | No trivial tests | **PASS** (source review) |
| 11 | Semantic mutation verbs | **PASS** |
| 12 | Localized dynamic copy | **PASS** |

---

## CORRECTION 13:43 findings

| Finding | Result | Evidence |
|---------|--------|----------|
| EN 1990 + DE NA combinations in `evaluate()` | **PARTIAL** | ULS 6.10, construction ULS, SLS char, SLS qp, fire 6.11 (`part_en1990`). **No frequent (ψ₁).** Governing label on ULS/deflection. |
| Hand-typed M_Ed/V_Ed/N_Ed as sole beam input | **FIXED** | Beam scalars removed; `part_en1990` derives effects from actions. Columns use external `m_k_nm`/`n_k_n` at characteristic level (acceptable override path). |
| Every editable leaf read by ≥1 check | **FAIL** | `sls_char` computed but unused in check math (`inferences/🦀️.rs:363`). Stud fatigue limit ignores `annex`. |
| Scope-aware perturbation test | **FAIL** | `every_editable_leaf_affects_at_least_one_check` uses **only** `En1994Snapshot::default()` (`compliance-report/🦀️.rs:416-419`). Hard skips: `annex`, `fatigueDetail`, `nCycles`, `deltaSigma*`, `deltaTau*`, `ltbLengthM`, steel plate dims, `mKNm`/`vKN`/`nKN`, column load companions (`:352-366`). No bridge/column/unpropped example perturbation. |
| N/A-in-default leaves not exempt | **FAIL** | `ltbLengthM`, bridge/fatigue leaves skipped in test without alternate example. |
| One source of truth (no duplicated quantities) | **FAIL** | Q-office: `qAreaPa` + `qLineNPerM` both non-zero (`🦀️.rs:327-328`). |

Static audit: `🗑️generated/verify-en1994/ignored-fields-audit.txt`

---

## Hand derivations (check 3)

See `🗑️generated/verify-en1994/hand-numerics.txt`. Summary: b_eff **2.30 m**, P_Rd **81.656 kN**, η_min **0.49**, n_L **3.030**, I_eff factor **1.106**.

---

## Blocking fix list

1. **`✏️editor/📌️panels/📚️catalogue/🦀️.rs:21`** — Crate does not compile: `render_catalogue` now requires `(examples, tables, locale, controller_id, &TreeWindows)` per B2 `🖥️app-surface/🦀️.rs:1218`. Update call signature (or family catalogue stub) so `bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache -- --no-fail-fast` executes all tests with 0 skipped.

2. **`🧬️schema/🟦️.ts:5-7` + `📸️snapshot/🟦️.ts:5-7`** — Regenerate TS snapshot facets: replace `beams: unknown[]` / `columns: unknown[]` / `slabs: unknown[]` with typed nested interfaces matching Rust DSL records (CORRECTION 13:27 #8).

3. **`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:322-444`** — Rewrite `every_editable_leaf_affects_at_least_one_check` per ADDENDUM 13:43: perturb each editable leaf in the committed example where it applies (default building beam, failing beam, bridge girder, column, slab, unpropped/hogging for `ltbLengthM`, fire example for insulation/fireRating). Remove blanket skips for `annex`, `fatigueDetail`, `nCycles`, `deltaSigma*`, `deltaTau*`, `ltbLengthM`, and force-override leaves when a scoped example uses them. Only exempt descriptive `name`/`title`/`id` entity labels.

4. **`🧬️schema/🦀️.rs` (`part_en1990`) + `💡️inferences/🦀️.rs`** — Add EN 1990 SLS **frequent** combination (ψ₁ factors already in `psi_factors`); emit at least one SLS frequent check or govern an existing SLS limit with it. Structural-family rule requires ULS + SLS char/**freq**/quasi-perm + construction stage.

5. **`💡️inferences/🦀️.rs:107,363`** — `sls_char` must govern a real check (e.g. crack width / stress) or be removed from the subject surface; `let _ = sls_char` is a dead binding proving characteristic SLS is not evaluated.

6. **`🦀️.rs:327-328` + `part_en1990::action_internals`** — Resolve duplicated imposed load on Q-office: store either `qAreaPa` **or** `qLineNPerM`, not both; ensure `action_internals` uses a single source of truth per action (CORRECTION 13:43).

---

## Non-blocking notes

- Round-2 ignored-field blockers (#2–#5) are substantively wired in `evaluate()`; static audit confirms reads.
- `part_en1990` ULS 6.10 leading-variable loop and construction/composite stage filter are a real (simplified) combination engine — major progress on R2 #1.
- DE bridge γ_Mf 1.35 vs EN 1.15 is documented with normative references (`schema/🦀️.rs:227-248`); steel fatigue test logic looks correct; stud Δτ should also respect annex γ_Mf.
- `SteelSection::resolve` overwrites catalogue geometry from designation — acceptable for catalogue workflow; custom sections need perturbation in a non-catalogue example.
- Column `m_max_rd_nm` shortcut (R2 note) remains; not re-raised as blocking.
- Impl md "Remaining gaps: None" and "72/72 passed" are false until #1 clears.

---

*Logs: `🗑️generated/verify-en1994/test-r3.txt`, `hand-numerics.txt`, `ignored-fields-audit.txt`*
