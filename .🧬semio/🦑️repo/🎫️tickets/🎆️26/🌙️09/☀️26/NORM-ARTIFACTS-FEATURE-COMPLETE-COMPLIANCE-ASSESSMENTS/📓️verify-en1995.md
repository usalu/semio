# Wave D Verification — EN 1995 (`🪵️en1995`) — Round 2

**VERDICT: FAIL (12 blocking)**

Reviewer: adversarial Wave D (read-only). Implementer claim: 84/84 passed, no gaps (`📓️impl-en1995.md`). Evidence date: 2026-09-26.

---

## Round 1 history

Round 1 (**FAIL, 11 blocking**): field-meta would not compile (`NormFieldChoice`); positional `members[{idx}]` paths; wrong compression/combined remedies; spacing tautology; missing EN 1995-2; stale flat mutations; no jsonschema/oracle parity tests; fresh nx run failed to compile.

Round 2: crate compiles; **84/84** tests green; paths, spacing, compression/combined remedies, AnnexParams fire routing, EN 1995-2 stubs, oracle/jsonschema tests added. **Subject model, clause depth, example tests, field-meta quality, ignored editable fields, and test rigour remain insufficient.**

---

## Test run (mandated)

```text
Summary [   0.749s] 84 tests run: 84 passed, 0 skipped
```

Command: `bun nx run @semio-tech/norm-en1995-rs:test --skip-nx-cache -- --no-fail-fast`  
Log: `🗑️generated/verify-en1995/test-output.txt`

---

## Check summary (brief §1–10 + 7b)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **FAIL** | Hierarchical `members`/`connections` in SI (`📸️snapshot/🦀️.rs` L15–24), but design effects remain pre-merged per member (`mEdNm`, `vEdN`, `nEdN`, … L59–69); no load cases/combinations. `floorAVert` is a direct scalar, not derived from stiffness/mass (§7.3). `supportLengthM` and `bucklingLengthZM` are editable + mutated but never read by `evaluate()` (only `buckling_length_y_m` at `⚖️timber/🦀️.rs` L332, L363). |
| 2 | Clause coverage | **FAIL** | Checks in `⚖️timber/🦀️.rs` L246–587 cover 1-1 ULS/SLS/§8, 1-2 fire, simplified 1995-2. **Missing / surrogate:** §7.3 `f₁ > 8 Hz` (DE NA) — only `a_vert` limit (L421–431). `connections[].steelPlate` / `steelPlateThicknessM` ignored in Johansen (`evaluate_connection` L544 — always single-shear timber-timber). EN 1995-2 Annex A fatigue uses ad-hoc `k_fat` log formula (L473–476), not Annex A damage accumulation. Bridge ULS (L503–511) duplicates 1-1 bending on the same `mEdNm`. Johansen modes a–f only (L201–213); no double-shear / steel-plate path despite subject fields. |
| 3 | Numerics | **PASS** | Hand chain for default GL28h DE beam: bending u≈0.389, shear u≈0.211 (`⚖️timber/🦀️.rs` L276–308; test `shear_utilization_diverges_between_annexes_for_c24` L35–51). `d_ef(3600 s)=49 mm` (`fire_def_r60` L69–73). |
| 4 | Applicability | **PASS** | Empty members → `NotApplicable` (L249–251). Tension, compression, c90, SLS, vibration, fire, bridge gated on positive inputs. |
| 5 | National annex | **PASS** | DE vs EN: `k_cr` (L71–75, test L26–51), `w_fin` L/200 vs L/250 (L78, L409), floor `a_vert` 0.05 vs 0.10 (L79, L422), bridge `a_vert` 0.5 vs 0.7 (L80, L490). `γ_M` glulam 1.25 / solid 1.3 per Table 2.3 (L69) — acceptable. |
| 6 | Report quality | **PASS** | `members[id=<id>].…` / `connections[id=<id>].…` via `mpath`/`cpath` (`⚖️timber/🦀️.rs` L7–8). Localized `title`/`explanation` en+de. Compression remedy `b_req = a_req/h` (L342–345). Combined remedy bounded search (L357–373). Spacing utilization from geometry (L238–243, L562–583). One remedy-apply test for bending (`🧪️tests/⚖️compliance/🦀️.rs` L96–105). **Gap:** no automated test that every emitted path resolves (see check 9). |
| 7 | Examples | **FAIL** | `compliant_glulam_beam()` / `noncompliant_multi_fail()` exist and are asserted in schema tests (`🧪️tests/⚖️compliance/🦀️.rs` L77–93). **Example crate tests** only `text.len() > 8`: `📚️examples/🌉️glulam-footbridge/🧪️tests/🧩️example/🦀️.rs` L2–4; `📚️examples/❌️multi-fail-timber/🧪️tests/🧩️example/🦀️.rs` L2–3 — do not decode DSL/pack or assert `complies()` / `fail_count ≥ 2`. |
| 7b | Inputs UX | **FAIL** | `🏷️field-meta/🦀️.rs` compiles with `NormFieldChoice`. **Strength-class choices** use raw wire codes as labels: `choice("C14", "C14", "C14")` etc. (L10–16) — violates CORRECTION 13:27 #1. Structured editor wired (`📥️inputs/🦀️.rs` L20). **No test** iterating default-snapshot editable leaves for en+de meta (CORRECTION #2). |
| 8 | Mutations & schema | **FAIL** | `En1995Mutation` has **46** semantic kinds (`🧬️mutations/🦀️.rs` L55–152); oracle manifest lists 46 (`🔮️oracles/🔣️.json`). **Stale artifacts:** 20 legacy flat-scalar dirs under `🧫️fixtures/🧬️mutations/` (`change-m-ed-knm`, `change-w-mm3`, …). `🪵️mutate-en1995-1/🦀️.rs` still declares 20 old `KINDS` (L59–80). Orphaned dead test `🔺️diff/📝️text/🧪️tests/🔬️unit/🦀️.rs` references removed `ChangeMEdKnm` / `m_ed_knm` (L8–11) — not in module tree but misleading. TS diff facets still use `Record<string, unknown>[]` for member lists (`🔺️diff/🟦️.ts` L3–4). |
| 9 | Tests | **FAIL** | 84 executed, 0 skipped (see above). Python↔Rust parity + jsonschema present (`🧪️tests/⚖️compliance/🦀️.rs` L124–194). **Only one** fail→pass remedy test (bending height, L96–105); CORRECTION 13:27 #5 requires ≥2. **No** path-resolution test for emitted paths (CORRECTION #4). Editor tests still `!checks.is_empty()` only (`✏️editor/🧪️tests/🔬️unit/🦀️.rs` L175, L232). |
| 10 | Stubs | **PASS** (obs.) | No `todo!`/`unimplemented!` on evaluate path. Residual: catalogue placeholder (`📚️catalogue/🦀️.rs` L3); legacy fixture trees; simplified EN 1995-2 fatigue. |

---

## CORRECTION 13:27 — recurring causes (explicit)

| # | Cause | Result | Evidence |
|---|--------|--------|----------|
| 1 | `NormFieldChoice` human en+de labels, not wire codes | **FAIL** | `STRENGTH` choices `("C24","C24","C24")` etc. `🏷️field-meta/🦀️.rs` L10–16 |
| 2 | Every editable leaf has meta + default-snapshot leaf test | **FAIL** | Table covers members/connections wildcards; **no** iteration test in family tests |
| 3 | Structured inputs editor, not JSON dump | **PASS** | `render_document_editor` + `en1995_field_meta` `📥️inputs/🦀️.rs` L20 |
| 4 | Entity paths `[id=…]` + path-resolve test | **PARTIAL** | Paths emitted correctly (`⚖️timber/🦀️.rs` L7–8); **no** resolve test |
| 5 | ≥2 distinct fail→pass remedy-apply tests | **FAIL** | One test: `remedy_increasing_height_improves_bending` `🧪️tests/⚖️compliance/🦀️.rs` L96–105 |
| 6 | Example tests decode asset + `complies()` / `fail_count ≥ 2` | **FAIL** | Example tests `text.len() > 8` only (see check 7) |
| 7 | Python oracle ±0.5% + jsonschema on snapshot | **PASS** | `🧪️tests/⚖️compliance/🦀️.rs` L124–194 |
| 8 | Facets regenerated; no bare `object` stubs | **FAIL** | `En1995MemberList.values: Record<string, unknown>[]` `🔺️diff/🟦️.ts` L3–4 |
| 9 | No tautologies / hardcoded geometry / ignored editables | **FAIL** | `supportLengthM`, `bucklingLengthZM`, `steelPlate*` editable, never used in `evaluate()` |
| 10 | No trivial-only tests (`!is_empty()`, `text.len`) | **FAIL** | Example + editor tests cited above |
| 11 | Semantic mutation verbs, not CRUD | **PASS** | `change-member-m-ed`, `insert-member`, … `🧬️mutations/🦀️.rs` L105–151 |
| 12 | Dynamic copy localized (no `copy(x,x)`) | **PASS** | `loc(en, de)` with distinct German strings throughout `⚖️timber/🦀️.rs` |

---

## Round 1 blocking items — re-check

| R1 # | Issue | Round 2 |
|------|--------|---------|
| 1 | field-meta compile | **FIXED** — `NormFieldChoice` throughout `🏷️field-meta/🦀️.rs` |
| 2 | positional paths | **FIXED** — `members[id=…]` (`⚖️timber/🦀️.rs` L7–8) |
| 3 | compression remedy √A | **FIXED** — `b_req = a_req / h` L342–345 |
| 4 | combined remedy heuristic | **FIXED** — bounded search L357–373 |
| 5 | fire hardcoded literals | **FIXED** — `AnnexParams::k_fi/k_mod_fi/gamma_m_fi/beta_n` L434–441 |
| 6 | spacing tautology | **FIXED** — `spacing_utilization` L238–243 |
| 7 | EN 1995-2 missing | **PARTIAL** — checks exist L462–525 but surrogate fatigue / duplicate ULS |
| 8 | mutations legacy | **PARTIAL** — enum 46 kinds; legacy `🧫️fixtures` + stale mutate adapter remain |
| 9 | python snapshot parity | **FIXED** — `🧪️tests/⚖️compliance/🦀️.rs` L124–168 |
| 10 | jsonschema test | **FIXED** — L170–194 |
| 11 | compile / stale cache | **FIXED** — 84/84 fresh run |

---

## Implemented check catalogue (as shipped)

| Part | Clause | Check id pattern | Notes |
|------|--------|------------------|-------|
| EN 1995-1-1 | §6.1.6 | `en1995.6.1.6.bending.<id>` | LTB `k_crit` |
| EN 1995-1-1 | §6.1.7 | `en1995.6.1.7.shear.<id>` | `k_cr` EN/DE, notch `k_v` |
| EN 1995-1-1 | §6.1.2 | `en1995.6.1.2.tension.<id>` | gated |
| EN 1995-1-1 | §6.3.2 | `en1995.6.3.2.compression.<id>` | `k_c`; **Y axis only** |
| EN 1995-1-1 | §6.2.4 | `en1995.6.2.4.combined.<id>` | interaction |
| EN 1995-1-1 | §6.1.5 | `en1995.6.1.5.c90.<id>` | `k_c,90` |
| EN 1995-1-1 | §7.2 | `en1995.7.2.winst/wfin.<id>` | `k_def`, DE/EN fin limit |
| EN 1995-1-1 | §7.3 | `en1995.7.3.vibration.<id>` | `a_vert` only; **no f₁** |
| EN 1995-1-2 | §4.2 | `en1995.1-2.4.fire.<id>` | reduced section via `AnnexParams` |
| EN 1995-1-1 | §8.2.2 | `en1995.8.2.2.johansen.<id>` | modes a–f + rope; no steel plate |
| EN 1995-1-1 | §8.3 | `en1995.8.spacing.<id>` | geometry ratios |
| EN 1995-2 | Annex A | `en1995.2.a.fatigue.<id>` | simplified log `k_fat` |
| EN 1995-2 | Annex B | `en1995.2.b.vibration.<id>` | `a_vert` vs bridge comfort |
| EN 1995-2 | §5 | `en1995.2.uls.bending.<id>` | duplicates 1-1 bending |
| EN 1995-2 | §7 | `en1995.2.sls.deflection.<id>` | L/400 |

**Not implemented:** load-combination-driven actions; §7.3 `f₁`; torsion; §6.4 tapered/curved; block shear; axially loaded screws as distinct check; double-shear / steel-plate Johansen; Z-axis column buckling.

---

## Numeric derivations (≥3, default compliant member B1)

**Inputs:** GL28h, DE, SC1, medium (`k_mod=0.8`), `b=0.20 m`, `h=0.40 m`, `M_Ed=28 kN·m`, `V_Ed=18 kN`, `M_crit=120 kN·m`, `γ_M=1.25`.

1. **Bending §6.1.6:** `W=5.333×10⁻³ m³`; `k_h=1.041`; `λ_rel,m=1.115`; `k_crit=0.725`; `f_m,d=13.49 MPa`; `σ_m=5.25 MPa` → **u=0.389**.

2. **Shear §6.1.7 (DE):** `k_cr=0.714`; `τ=0.473 MPa`; `f_v,d=2.24 MPa` → **u=0.211**.

3. **Fire `d_ef` (3600 s):** `β_n=0.70 mm/min`; `d_char=42 mm`; `d_ef=49 mm` — matches `fire_def_r60`.

---

## Blocking fix list

1. **`🧬️schema/📸️snapshot/` + `⚖️timber/🦀️.rs`** — Replace pre-merged design scalars (`mEdNm`, `vEdN`, `nEdN`, …) with load cases / combinations that derive design effects, **or** narrow declared norm scope and remove unused action fields. Subject must not pretend to be a complete structural model while actions are hand-typed constants.

2. **`⚖️timber/🦀️.rs` §7.3** — Add mandatory `f₁ > 8 Hz` floor-frequency check (DE NA) with mass/stiffness inputs; gate with localized `NotApplicable` when not a floor member.

3. **`⚖️timber/🦀️.rs` + snapshot** — Derive `floorAVert` from editable stiffness/mass/span fields (§7.3 model), or split floor vs bridge acceleration so §7.3 and EN 1995-2 Annex B do not share one opaque scalar.

4. **`⚖️timber/🦀️.rs` L330–347** — Use `bucklingLengthZM` (and `supportLengthM` if structurally relevant) in column buckling / bearing logic, or delete fields + mutations + field-meta rows.

5. **`⚖️timber/🦀️.rs` `evaluate_connection`** — Wire `steelPlate`, `steelPlateThicknessM`, `shearPlanes` into Johansen (double shear / steel-plate modes per §8.2.2–8.2.3); checks must fail when plate flag changes capacity.

6. **`⚖️timber/🦀️.rs` `evaluate_bridge_member`** — Replace log-`k_fat` surrogate with EN 1995-2 Annex A fatigue assessment (damage accumulation, stress ranges); bridge ULS must not duplicate 1-1 bending on the same `mEdNm` without bridge-specific load model.

7. **`📚️examples/*/🧪️tests/🧩️example/🦀️.rs`** — Decode committed DSL/pack, call `evaluate()`, assert compliant example `complies()` and noncompliant `fail_count ≥ 2` with named failing ids.

8. **`✏️editor/🏷️field-meta/🦀️.rs` L10–16** — Strength-class `NormFieldChoice` labels: human en+de (e.g. `"C24 — strength class"` / `"C24 — Festigkeitsklasse"`), not `"C24"/"C24"`.

9. **New test** (family `🧪️tests/`) — Walk default `En1995Snapshot` value tree; every editable scalar/list leaf must return `en1995_field_meta` with en+de label (and SI unit where quantity).

10. **`🧫️fixtures/🧬️mutations/`** — Remove 20 legacy flat-scalar triad directories; update `🪵️mutate-en1995-1/🦀️.rs` `KINDS` + `🐍️.py` to 46 hierarchical kinds; delete or rewrite orphaned `🔺️diff/📝️text/🧪️tests/🔬️unit/🦀️.rs`.

11. **`🧪️tests/⚖️compliance/🦀️.rs`** — Add a **second** fail→pass remedy-apply test on a different check (e.g. `en1995.8.spacing.*` on `conn-C2` or compression width on `col-C1`).

12. **New test** — For `compliant_glulam_beam()` and `noncompliant_multi_fail()` reports, parse every `subject.path` and `remedy.target.path` with app-surface `parse_path` / `get_value_at_path`; all must resolve.

---

## Non-blocking observations

- `γ_M` glulam 1.25 matches EN 1995-1-1 Table 2.3 (not 1.3) — acceptable.
- Compliant default runs EN 1995-2 checks because `bridgeNCycles=2e6` (`📸️snapshot/🦀️.rs` L69); floor §7.3 suppressed when `bridgeNCycles > 0` (`⚖️timber/🦀️.rs` L421).
- Catalogue panel remains placeholder (`📚️catalogue/🦀️.rs` L3).
- Implementer “no gaps” claim contradicted by subject-model and test-gate gaps above despite green runner.

---

*Verification artifacts: `🗑️generated/verify-en1995/test-output.txt`.*
