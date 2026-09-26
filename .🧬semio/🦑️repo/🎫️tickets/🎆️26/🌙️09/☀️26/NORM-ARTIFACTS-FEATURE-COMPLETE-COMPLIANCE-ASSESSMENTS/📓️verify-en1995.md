# Wave D Verification — EN 1995 (`🪵️en1995`) — Round 4

**VERDICT: PASS (0 blocking)**

Reviewer: adversarial Wave D (read-only). Fixer claim (`142eadd6`): `Summary [ 2.831s] 177 tests run: 177 passed, 0 skipped` and `Summary [ 0.154s] 51 tests run: 51 passed, 0 skipped`. Evidence date: 2026-09-26. Untrusted until re-run — confirmed below.

---

## Test runs (mandated)

```text
Summary [   2.159s] 177 tests run: 177 passed, 0 skipped
```

Command: `bun nx run @semio-tech/norm-en1995-rs:test --skip-nx-cache -- --no-fail-fast`

```text
Summary [   0.221s] 51 tests run: 51 passed, 0 skipped
```

Command: `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache`

---

## Round 4 — Round 3 blocking list re-check (FAIL 4)

| R3 # | Issue | Round 4 | Evidence |
|------|--------|---------|----------|
| 1 | `SlsFrequent` + ψ₁ combos + frequent §7.2 check | **FIXED** | `ComboKind::SlsFrequent` (`⚖️timber/🦀️.rs` L612–617); `enumerate_combos()` applies ψ₁ from `psi_factors()` (`L263–281`, `L767–786`); frequent deflection check `en1995.7.2.wfreq.*` (`L1423–1440`); DE ψ₁ divergence test `sls_frequent_uses_psi1_and_de_snow_high_diverges` (`🧪️tests/⚖️compliance/🦀️.rs` L714–779). Default beam derives effects from `q_line_n_per_m` actions (`📸️snapshot/🦀️.rs` L73–99), not hand-typed `mEdNm`. |
| 2 | Stale `📝️text/🟦️.ts` (9 kinds, `unknown`) | **FIXED** | 66-kind externally tagged union (`🧬️mutations/📝️text/🟦️.ts` L1, L347–413); no `Record<string, unknown>`; matches Rust `En1995Mutation` + `KINDS` (`🧬️mutations/🦀️.rs` L77–214, 66 entries). `InsertMember`/`RemoveMember` retain list `index` matching Rust list-splice semantics — not legacy flat-scalar CRUD. |
| 3 | Stale binary protocol flat scalars | **FIXED** | 66 hierarchical wire records tag=0..65 (`🧬️mutations/💾️binary/📡️.protocol.semio` L14–145); no `change-m-ed-knm`, `change-w-mm3`, or other flat-scalar tags. |
| 4 | Identical en/de combined-interaction prose | **FIXED** | Distinct German lead-in: EN `"Combined compression and bending: …"` vs DE `"Kombination Druck und Biegung: …"` (`⚖️timber/🦀️.rs` L1363–1368). Formula symbols may match; prose differs. |

---

## ADDENDA spot-check (unchanged from Round 3 — still PASS)

| Addendum | Result | Evidence |
|----------|--------|----------|
| §7.3 f₁ | **PASS** | `assess_floor` computes `f1` (`⚖️timber/🦀️.rs` L1468–1498). |
| Johansen plate/rows | **PASS** | `steel_plate_changes_capacity_and_can_make_the_check_fail` (`🧪️tests/⚖️compliance/🦀️.rs` L251–281); `connection_rows_change_johansen_capacity` (L703–710). |
| `bucklingLengthZM` in buckling | **PASS** | `lam_z = m.buckling_length_z_m / iz` (`⚖️timber/🦀️.rs` L1341–1346). |
| Catalogue k_mod parity | **PASS** | `reference_table_k_mod_cell_equals_evaluated_modification_factor` (`🧪️tests/⚖️compliance/🦀️.rs` L674–688). |
| Duplicate/dangling ids → `one_of` | **PASS** | `push_duplicate_ids` + `Remedy::one_of` (`⚖️timber/🦀️.rs` L1129–1202); tests L638–670. |
| Perturbation signature | **PASS** | `(id, status, computed, limit, utilization)` (`🧪️tests/⚖️compliance/🦀️.rs` L371–386); no `field_fingerprint`/`id_score` in family evaluate path. |

---

## Check summary (brief §1–10 + 7b)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | EN 1990 ULS + SLS char/frequent(ψ₁)/QP from `enumerate_combos()`; actions drive design effects. |
| 2 | Clause coverage | **PASS** (obs.) | Unchanged from Round 3 spot-check. |
| 3 | Numerics | **PASS** | Oracle ±0.5% (`🧪️tests/⚖️compliance/🦀️.rs` L329–348). |
| 4 | Applicability | **PASS** | Role/magnitude gating throughout `assess_member`. |
| 5 | National annex | **PASS** | DE vs EN divergence tests L55–90, L205–211, L714–779. |
| 6 | Report quality | **PASS** | Localized paths/remedies; combined check en/de prose distinct (`⚖️timber/🦀️.rs` L1367–1368). |
| 7 | Examples | **PASS** | Four examples decode + assert verdicts. |
| 7b | Inputs UX | **PASS** | Structured editor + field-meta with human labels. |
| 8 | Mutations & schema | **PASS** | Rust/TS/binary/protocol aligned at 66 kinds; diff TS typed (`🔺️diff/🟦️.ts`). |
| 9 | Tests | **PASS** | 177 executed, 0 skipped (+1 `sls_frequent_uses_psi1_and_de_snow_high_diverges`). |
| 10 | Stubs | **PASS** | No `todo!` on evaluate path. |

---

## Blocking fix list

None

---

## Non-blocking observations

- Plugin-wide `NormMutationLeafTaxonomy` `rows.maxItems: 392` vs 547 payloads — not blocking for this family.
- `🏅️standards/🔖️1/🪆️subsets/🔣️.json` L10 `subsetPolicyRationale` still names `change-m-ed-knm` (stale prose).
- `w_fin` uses ψ₂ from governing **characteristic** combo (`⚖️timber/🦀️.rs` L1397), not the quasi-permanent combo — verify against EN 1995-1-1 §7.2 intent.
- Round 3 → Round 4: 176 → 177 tests; all four Round 3 blockers **confirmed fixed** in source and runner.

---

# Wave D Verification — EN 1995 (`🪵️en1995`) — Round 3

**VERDICT: FAIL (4 blocking)**

Reviewer: adversarial Wave D (read-only). Fixer claim (`de37a795`): `Summary [ 1.961s] 176 tests run: 176 passed, 0 skipped`. Evidence date: 2026-09-26. Untrusted until re-run — confirmed below.

---

## Test runs (mandated)

```text
Summary [   2.555s] 176 tests run: 176 passed, 0 skipped
```

Command: `bun nx run @semio-tech/norm-en1995-rs:test --skip-nx-cache -- --no-fail-fast`  
Log: `🗑️generated/verify-en1995/test-output.txt`

```text
Summary [   0.561s] 51 tests run: 51 passed, 0 skipped
```

Command: `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache`

---

## Round 3 — original 12 blockers re-check

| R2 # | Issue | Round 3 | Evidence |
|------|--------|---------|----------|
| 1 | Pre-merged design scalars / no EN 1990 combinations | **FIXED** (partial) | `CharacteristicAction` + `enumerate_combos()` (`⚖️timber/🦀️.rs` L680–811); ULS/SLS char/QP from actions (`L1258–1262`, `L1357–1387`). Default beam uses `qLineNPerM` loads (`📸️snapshot/🦀️.rs` L73–99), not member-level `mEdNm`. **Gap:** no SLS **frequent (ψ₁)** combination (see blocking #1). |
| 2 | §7.3 `f₁ > 8 Hz` | **FIXED** | `assess_floor` computes `f1` from mass/stiffness/span (`⚖️timber/🦀️.rs` L1399–1462); check `en1995.7.3.f1.*`; example test asserts it (`📚️examples/🏠️glulam-floor-beam/🧪️tests/🧩️example/🦀️.rs` L19). |
| 3 | `floorAVert` opaque scalar | **FIXED** | `a_vert` derived from `massKgPerM`/`massKgPerM2`, `dampingXi`, `f1` (`⚖️timber/🦀️.rs` L1417–1423); bridge uses separate crowd model (`L1564–1577`). |
| 4 | `bucklingLengthZM` ignored | **FIXED** | `lam_z = m.buckling_length_z_m / iz` in compression buckling (`⚖️timber/🦀️.rs` L1311–1316). `supportLengthM` enters `k_c_90` (`L366–368`, `L1341–1342`). |
| 5 | Johansen steel plate / rows | **FIXED** | `evaluate_connection` branches on `steel_plate` + `shear_planes` (`L1658–1663`); tests `steel_plate_changes_capacity_and_can_make_the_check_fail`, `connection_rows_change_johansen_capacity` (`🧪️tests/⚖️compliance/🦀️.rs` L251–282, L703–710). |
| 6 | Bridge fatigue surrogate / duplicate ULS | **FIXED** | Annex A Wöhler `N_R = 10^(a − b·log₁₀ Δσ)`, `k_fat = (N_R/N)^(1/β)` (`⚖️timber/🦀️.rs` L1543–1562). Bridge ULS uses crowd line load `m_ed = 1.5·m_crowd`, not 1-1 `mEdNm` (`L1532–1541`). |
| 7 | Example tests trivial | **FIXED** | All four examples decode DSL and assert verdicts (`📚️examples/*/🧪️tests/🧩️example/🦀️.rs`; compliance `compliant_examples_pass` L120–125). |
| 8 | Strength-class wire labels | **FIXED** | `choice("C24", "C24 — softwood strength class", "C24 — Nadelholz-Festigkeitsklasse")` etc. (`✏️editor/🏷️field-meta/🦀️.rs` L13–39). |
| 9 | No default-snapshot field-meta test | **FIXED** | `default_snapshot_every_editable_leaf_has_en_de_meta` (`🧪️tests/⚖️compliance/🦀️.rs` L232–247). |
| 10 | Legacy flat mutation fixtures | **FIXED** | 66 hierarchical kinds (`🧬️mutations/🦀️.rs` L147+); 66 fixture dirs; `on_disk == KINDS.len()` (`🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` L184). |
| 11 | Only one remedy-apply test | **FIXED** | `remedy_increasing_height_flips_bending` + `remedy_spacing_or_compression_flips_fail` (`🧪️tests/⚖️compliance/🦀️.rs` L139–165). |
| 12 | No path-resolution test | **FIXED** | `every_emitted_path_resolves_on_examples` (`🧪️tests/⚖️compliance/🦀️.rs` L168–184). |

---

## ADDENDA re-check

| Addendum | Result | Evidence |
|----------|--------|----------|
| 14:54 catalogue | **PASS** | `reference_tables()` returns 4 tables via shared `k_mod()` (`✏️editor/📌️panels/📚️catalogue/🦀️.rs` L202–204, L87–107). Test `reference_table_k_mod_cell_equals_evaluated_modification_factor` (`🧪️tests/⚖️compliance/🦀️.rs` L674–688). |
| 14:42 integrity | **PASS** (partial) | Duplicate member/connection/action ids → `Fail` + `Remedy::one_of` (`⚖️timber/🦀️.rs` L1099–1179); tests L638–656. Unknown strength → `one_of` tabulated classes L1219–1223, test L659–670. No cross-entity reference fields in snapshot (no `memberId` on connections) — dangling-ref scenario N/A; `strengthClass` invalid ref covered. |
| 14:37 perturbation | **PASS** | Signature `(id, status, computed, limit, utilization)` (`🧪️tests/⚖️compliance/🦀️.rs` L371–386). No `fingerprint`/`field_fingerprint`/`id_score` in family `*.rs`. `rg "let _ ="` evaluate path: only test cleanup (`🧪️tests/⚖️compliance/🦀️.rs` L365). `perturb_every_editable_leaf_in_committed_examples_changes_a_check` L610–634. Scope skips (`scope_skips` L512–531) are role/plate-gated per CORRECTION 13:43, not whole-subtree. |
| 14:37 identical en/de | **FAIL** | Combined interaction explanation identical en/de (`⚖️timber/🦀️.rs` L1336). |
| 14:37 `Record<string, unknown>` | **FAIL** | `🧬️schema/🧬️mutations/📝️text/🟦️.ts` L9, L38 — stale 9-kind positional union with `unknown` nests. |
| Coordinator item 1 (ψ₁) | **FAIL** | `ComboKind` has ULS, SLS char, SLS QP, accidental only (`⚖️timber/🦀️.rs` L611–616); `psi_factors` returns ψ₁ (`L263–278`) but no `SlsFrequent` combo or check uses ψ₁. |

---

## Check summary (brief §1–10 + 7b)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **FAIL** | Hierarchical members/connections + EN 1990 eq 6.10 ULS and SLS char/QP (`⚖️timber/🦀️.rs` L680–811). **Missing SLS frequent (ψ₁).** |
| 2 | Clause coverage | **PASS** (obs.) | §6.1–8, §7.2–7.3 (f₁, a_vert, velocity), EN 1995-2 Annex A/B, fire, Johansen 8.2.2/8.2.3, spacing. |
| 3 | Numerics | **PASS** | Hand chain still valid; oracle ±0.5% (`🧪️tests/⚖️compliance/🦀️.rs` L329–348). |
| 4 | Applicability | **PASS** | Role/magnitude gating throughout `assess_member`. |
| 5 | National annex | **PASS** | DE vs EN divergence tests L55–90, L205–211. |
| 6 | Report quality | **FAIL** | Identical en/de on combined check (`⚖️timber/🦀️.rs` L1336). Otherwise localized paths/remedies OK. |
| 7 | Examples | **PASS** | Four examples decode + assert verdicts. |
| 7b | Inputs UX | **PASS** | Structured editor + field-meta with human labels. |
| 8 | Mutations & schema | **FAIL** | Rust 66 kinds + diff TS typed (`🔺️diff/🟦️.ts` L3–11). **Stale:** `🧬️mutations/📝️text/🟦️.ts` (9 kinds, `Record<string, unknown>`); `🧬️mutations/💾️binary/📡️.protocol.semio` L16+ still lists `change-m-ed-knm` flat scalars. |
| 9 | Tests | **PASS** | 176 executed, 0 skipped; perturbation, oracle, jsonschema, remedies, paths present. |
| 10 | Stubs | **PASS** | No `todo!` on evaluate path. |

---

## Blocking fix list

1. **`⚖️timber/🦀️.rs` `ComboKind` + `enumerate_combos()`** — Add `SlsFrequent` using ψ₁ from `psi_factors()` (EN 1990 Table A1.1); emit at least one SLS frequent check (e.g. deflection or stress) beside existing char/QP §7.2 checks; test DE ψ₁ divergence where applicable.

2. **`🧬️schema/🧬️mutations/📝️text/🟦️.ts`** — Regenerate from Rust `En1995Mutation` (66 id-addressed kinds); remove `Record<string, unknown>` member/connection nests and positional `index` CRUD shapes.

3. **`🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio`** — Regenerate wire tags for all 66 hierarchical kinds; remove legacy `change-m-ed-knm`, `change-w-mm3`, … flat-scalar records.

4. **`⚖️timber/🦀️.rs` L1336** — Localize combined-interaction explanation: German prose must differ from English (e.g. „Kombination Druck und Biegung“ / formula labels), not copy the identical formula string to both `loc()` arms.

---

## Non-blocking observations

- Plugin-wide `NormMutationLeafTaxonomy` `rows.maxItems: 392` while generate emits 547 payloads (`✏️s/🔌️plugins/📕️norm/🧬️schema/🔣️.json` L964). EN 1995 nx target passes 176/176 — **not blocking for this family**.
- `🏅️standards/🔖️1/🪆️subsets/🔣️.json` L10 `subsetPolicyRationale` still names `change-m-ed-knm` (stale prose).
- `w_fin` uses ψ₂ from the governing **characteristic** combo (`L1364`), not the quasi-permanent combo already enumerated — verify against EN 1995-1-1 §7.2 intent.
- Round 2 → Round 3: 84 → 176 tests; fixer claim on EN 1990 combinations, f₁, Johansen, integrity, catalogue k_mod parity, and perturbation signature **confirmed** where listed above.

---

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
