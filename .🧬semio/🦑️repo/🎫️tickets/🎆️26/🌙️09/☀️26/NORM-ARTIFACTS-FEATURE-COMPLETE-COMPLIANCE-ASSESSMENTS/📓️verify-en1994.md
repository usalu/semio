# Verify — EN 1994 (`🧩️en1994`) — Round 5

**Auditor:** read-only adversarial verification, 2026-09-26 (round 5)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1994.md` (75/75, Round-4 closeout)  
**Prior verify:** R1 **FAIL (8)**, R2 **FAIL (5)**, R3 **FAIL (6)**, R4 **FAIL (3)**

**VERDICT: PASS**

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | FAIL | 8 — index paths, raw enums, tautological b_eff, missing SLS crack, surrogate LTB, incomplete DSL, no example verdict tests, no jsonschema in nx |
| R2 | FAIL | 5 — hand-typed M_Ed/V_Ed/N_Ed (no EN 1990 load cases); ignored studs.spacingM, columns.kind, tw/tf, sheeting.thicknessM |
| R3 | FAIL | 6 — crate does not compile (0 tests); TS facets `unknown[]`; leaf perturbation not scope-aware; missing SLS frequent combination; sls_char orphan; duplicated imposed load |
| R4 | FAIL | 3 — empty `reference_tables()`; `let _ = annex` in γ_G/γ_Q; perturbation pred still exempts `annex` + beam/slab force leaves, no failing-beam scope |
| R5 | **PASS** | 0 — Round-4 blockers substantively fixed; 75/75 + 51/51 contract executed |

---

## Test run

| Target | Command | Result |
|--------|---------|--------|
| Family | `bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache -- --no-fail-fast` | **75 run / 75 passed / 0 skipped** — `Summary [2.196s]` |
| Contract | `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **51 run / 51 passed / 0 skipped** — `Summary [0.166s]` |

Logs: `🗑️generated/verify-en1994/test-r5.txt`, `contract-test-r5.txt`

Impl claim 75/75 is **confirmed**.

---

## Round-4 blocker re-check (per-item)

| # | Round-4 blocker | R5 | Evidence |
|---|-----------------|-----|----------|
| 1 | Catalogue `reference_tables()` sharing evaluate consts; test asserts limit = cell | **PASS** | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:20-130` — three tables (γ_G/γ_Q, ψ₀/ψ₁/ψ₂, stud s_min/s_max) call `part_en1990::gamma_g`/`gamma_q`, `psi_factors`, `part_1_1::stud_spacing_limits_m` / `STUD_SPACING_*`. Test `reference_tables_cells_match_psi_and_gamma_i_sources` (`🧪️tests/🔬️unit/🦀️.rs:38-63`) asserts catalogue cells equal evaluate sources; empty tables rejected (`:27`). |
| 2 | Remove `let _ = annex` in `gamma_g`/`gamma_q`; annex branches normatively | **PASS** | `🧬️schema/🦀️.rs:313-323` — `match annex { En => GAMMA_*_EN, De => GAMMA_*_DE }`. `rg 'let _ = annex'` over family → 0 hits. `accumulate` reads `gamma_g(annex)`/`gamma_q(annex)` (`:414-415`). EN/DE constants numerically equal (1.35/1.50) but distinct symbols; catalogue publishes both rows. |
| 3 | Scope-aware perturbation; no blanket skips for `annex`/force leaves; `failing-beam` scope | **PASS** | `compliance-report/🦀️.rs:322-588` — six scopes (`default-building`, `failing-beam`, `unpropped-ltb`, `custom-plate`, `bridge-fatigue`, `fire-demanding`). `failing-beam` decodes `composite-floor-beam-failing` DSL (`:445-477`). `annex` walked in `bridge-fatigue` pred (`:494`) + inline γ_Mf limit assert (`:542-554`). `beam_force_override` / `column_area_companion` removed. Column `mKNm`/`nKN` walked in dedicated block (`:560-585`). Beam/slab `mKNm`/`vKN`/`nKN` **removed** from schema (`🦀️.rs:92-110` — `CharacteristicAction` has `qAreaPa` + `fKN` only; columns use `ColumnAction`). `fKN` perturbation on area-loaded actions triggers `en1994.action.single-source.*` dual-source check (`inferences/🦀️.rs:565-604`). Signature `(id, status, computed, limit, utilization)` (`:324-335`). |

---

## Fixer claims (Round 4 closeout)

| Claim | R5 | Evidence |
|-------|-----|----------|
| Catalogue γ/ψ/stud from shared consts | **PASS** | See blocker #1 |
| Annex changes γ (branch) + bridge fatigue check | **PASS** | `gamma_g`/`gamma_q` match; `de_vs_en_bridge_fatigue_gamma_mf` + leaf-test annex block |
| Beam forces from area load, not second editable copy | **PASS** | `action_internals` derives `q_area_pa × spacing_m` (`🦀️.rs:332-339`); no `qLineNPerM`; `mKNm`/`vKN`/`nKN` gone from beam actions |
| jsonschema cannot skip | **PASS** | nx test inline `python3 -c "…jsonschema.validate…"` hard-fails (`compliance-report/🦀️.rs:272-284`); `validate_snapshot.py` exits 1 if `jsonschema` missing (`🧪️tests/⚖️compliance-oracle/validate_snapshot.py:6-8`) |
| Oracle compares η | **PASS** | `🐍️.py:199-220` — compares η, η_min, utilization for `en1994.6.6.1.2.etamin.*`; no shape-skip hatch (`rg skip` → 0) |

---

## Brief checks §1–10 (Round 5)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical beams/columns/slabs + `CharacteristicAction[]` / `ColumnAction[]`; area loads + optional `fKN`; EN 1990 ULS/construction/SLS char/freq/qp + fire |
| 2 | Clause coverage | **PASS** (caveats) | Parts 1-1, 1-2, 2 fatigue; SLS char/freq/qp wired. Crack remains min-A_s proxy |
| 3 | Numerics (≥3 hand checks) | **PASS** | Prior derivations in `hand-numerics.txt` still valid; `full_composite_worked_example_passes_default`, oracle `maxRel ≤ 0.005` executed |
| 4 | Applicability | **PASS** | LTB N/A propped; building fatigue N/A; fire rating gate — tests run |
| 5 | National annex | **PASS** | DE γ_Mf 1.35 vs EN 1.15 on bridge (`de_vs_en_bridge_fatigue_gamma_mf`, `de_bridge_gamma_mf_stricter_than_en`); γ_G/γ_Q annex-branched; stud Δτ uses `AnnexParams::gamma_mf` |
| 6 | Report quality | **PASS** | `[id=…]` paths, en+de copy, remedy-flip tests executed |
| 7 | Examples | **PASS** | `passing_example_dsl_complies`, `failing_example_dsl_does_not_comply_with_named_ids`, `bridge_example_runs_fatigue_checks` |
| 7b | Inputs UX | **PASS** | `every_default_leaf_has_en_de_field_meta` executed |
| 8 | Mutations & schema | **PASS** | TS typed (`CompositeBeam`, `CharacteristicAction`, `ColumnAction`); semantic mutations present |
| 9 | Tests | **PASS** | 75 executed, 0 skipped; oracle + jsonschema + perturbation all ran |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!`; `default_placeholder()` only for insert mutations |

---

## CORRECTION 13:27 (12 causes)

| # | Cause | R5 |
|---|--------|-----|
| 1 | Human enum labels | **PASS** |
| 2 | Every editable leaf has meta | **PASS** |
| 3 | Structured editor | **PASS** |
| 4 | `[id=…]` paths + resolve test | **PASS** |
| 5 | ≥2 remedy-flip tests | **PASS** |
| 6 | Example comply / fail ≥2 | **PASS** |
| 7 | Oracle ±0.5 % + jsonschema | **PASS** |
| 8 | Facets regenerated | **PASS** |
| 9 | No tautologies / ignored fields | **PASS** |
| 10 | No trivial tests | **PASS** |
| 11 | Semantic mutation verbs | **PASS** |
| 12 | Localized dynamic copy | **PASS** |

---

## CORRECTION 13:43 / gaming audit

| Finding | R5 | Evidence |
|---------|-----|----------|
| EN 1990 combinations in `evaluate()` | **PASS** | ULS 6.10, construction ULS, SLS char/freq/qp, fire 6.11 |
| Hand-typed M_Ed/V_Ed/N_Ed sole beam input | **PASS** | Removed; `part_en1990` derives from actions |
| Every editable leaf read by ≥1 check | **PASS** | Scope-aware perturbation + column-forces block; dual-source guard on `fKN` |
| Scope-aware perturbation | **PASS** | Six scopes; `failing-beam` from committed DSL |
| One source of truth (loads) | **PASS** | `qAreaPa` or `fKN` (mutually exclusive); `single-source` check on dual set |
| Gaming `let _ = annex` | **PASS** | Removed |
| Gaming `let _ = sls_char` | **PASS** | `sls_char` governs §7.2.2 stress (`inferences/🦀️.rs:338-364`) |
| Perturbation signature | **PASS** | Full normative tuple |
| Ratio / fingerprint slack | **PASS** | None (`rg fingerprint|1e-9 \*|1e-12 \*` → 0) |

---

## Perturbation scope map (R5)

| Leaf group | Scope | Notes |
|------------|-------|-------|
| `annex` | `bridge-fatigue` + inline γ_Mf assert | Inert on default building (γ_G/γ_Q identical EN/DE); correctly routed |
| `ltbLengthM` | `unpropped-ltb` | N/A on propped default |
| `nCycles`, `fatigueDetail`, `deltaSigma*`, `deltaTau*` | `bridge-fatigue` | N/A on building default |
| `mKNm`, `nKN` (columns) | `column-forces` block | Dedicated walk on default snapshot |
| `fKN` (beams/slabs) | `default-building` + dual-source check | Area-loaded actions: perturbing `fKN` triggers `single-source` fail |
| `qAreaPa`, studs, span (failing) | `failing-beam` | From `composite-floor-beam-failing` DSL |
| Steel geom (catalogue) | `custom-plate` | Designation overwrite bypass |

`default-building` pred still lists `annex` among scope-local skips (`:455`) — acceptable because annex is normative only in bridge scope and is walk-perturbed there; perturbing annex on default building would not change any check signature.

---

## Blocking fix list

*None — Round-4 blockers cleared.*

---

## Non-blocking observations (Round 5)

- `GAMMA_G_EN == GAMMA_G_DE` and `GAMMA_Q_EN == GAMMA_Q_DE` (both 1.35 / 1.50) — annex branch is wired but building ULS has no EN/DE divergence; bridge γ_Mf provides the annex differential proof.
- Catalogue partial-factor row label typo: "EN empfohleniert" (`catalogue/🦀️.rs:39`).
- Some mutation fixture JSON under `🧫️fixtures/` still carries legacy `mKNm`/`vKN`/`nKN` on beam actions; schema no longer defines these fields — serde ignores extras; tests pass.
- No committed DSL example using sole `point_force` (`qAreaPa=0`, `fKN>0`) on a beam; path covered by dual-source perturbation and `CharacteristicAction::point_force` helper.
- Column `m_max_rd_nm` polygon shortcut (R2 note) unchanged.

---

*R5 logs: `🗑️generated/verify-en1994/test-r5.txt`, `contract-test-r5.txt`*
