# Verify Gate — ISO 16757 Remedy Clear (`iso16757` / 📇️)

**VERDICT: PASS**

**Verifier:** adversarial read-only gate audit (remedy-clear fix)  
**Family root (via `os.walk` under norm plugin artifacts):** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/`  
**Implementer claim:** `📓️impl-gate-iso16757.md` — 313/313 family tests; six gate Fail sticks cleared  
**`📓️impl-gate-iso16757.md` present:** yes (this ticket)  
**Compliance gate:** not rerun (per audit brief)  
**Family test:** `bun nx run @semio-tech/norm-iso16757-rs:test --skip-nx-cache` — **not rerun** (Nx `@nxlv/python` plugin worker failed this session); `cargo test --lib` in family crate started but did not finish within audit window. Judgment is from source + gate-law trace on `examples/broken` DSL (`🖼️assets/🚫️broken/🗣️.dsl.semio`).

---

## Blocking fix list

None.

---

## Gate law (reference)

`🧪️tests/🚦️compliance-gate/🦀️.rs` `assert_remedies_flip_fails`: for each **Fail** with applicable remedies, try `apply_remedy_edit` option 0 per remedy; if still **Fail**, apply all applicable remedies sequentially on one tree; cleared when check id absent or status ≠ **Fail**. u=2.0000 on uncleared rows is the `fail()` helper dummy (`🧰common.rs:146`); u=0.05 / u=1.0 stuck **Fail** indicates bound/status mismatch.

---

## Audited pre-fix symptoms (broken example)

| Check id | Pre-fix gate symptom | Remedy shape now | Clears per gate law? |
|----------|----------------------|------------------|----------------------|
| `iso16757.1.6.4.searchTags.index-cv50-dup` | remedy left u=2.0000 | `fail()` + `Remedy::one_of` on `catalogue.productIndexes[id=index-cv50-dup].searchTags[0]` with `expected_needles` (product id, preferred name, dn tokens — not dummy `"1"`) (`📈️part1.rs:1811–1827`) | **Yes** — option 0 e.g. `product-cv` makes tag match product token → aggregate check **Pass** (matched≥1) |
| `iso16757.1.4.2.selection.empty` | u=2.0000 | `Remedy::one_of` on `selection.constraints[id=constraint-dn].value.value` with real property/parameter values first (`📈️part1.rs:2130–2147`); broken DSL keeps `property-values` at 50 and constraint at 999 | **Yes** — option 0 `50` satisfies `prop-dn` equal → `select_products` non-empty → id `…selection.empty` replaced by `…selection` **Pass** |
| `iso16757.1.10.bim.index-cv50` | u=2.0000 | Err-path `Remedy::one_of` on `…parameterValues.dn.value` (allowed domain `50`) before geometry/product fallbacks; `params.clone()` on resolve (`📈️part1.rs:2256–2330`) | **Yes** — dn→50 removes domain error; embedding **Ok** with non-none `resolved_geometry_id` (variant `geom.missing` string still Some) → no per-index **Fail** |
| `iso16757.1.10.bim.index-cv50-dup` | u=2.0000 | same Err-path remedies for shared variant | **Yes** — same as `index-cv50` |
| `iso16757.2.5.3.5.clearance.geom-valve-50` | three remedies u≈0.05; sequential u=1.0 still **Fail** | three `Remedy::at_least` on `geometry.objects.geom-valve-50.spaces[id=installation].bounds.max[{axis}]`; status predicate `ok = axis_ok` only (`have[i]+ε ≥ needed[i]`) — `product_inside` no longer blocks **Pass** (`📐️part2.rs:133–144,182–214`) | **Yes** — sequential writes max to `[0.25,0.30,0.20]` m; `axis_ok` true → **Pass** (equality at ≥ limit) |
| `iso16757.5.6.10.partNumber` | remedy[1] left u=40.0000 (input sum) | three `Remedy::at_least` on `scriptLimits.{maxSteps,maxRecursion,timeoutMs}` with required 10000 / 64 / 50 (`🔄part5.rs:395–422`) | **Yes** — sequential raises all zero caps; script `dn*10+50` with dn=40 → **Pass** at id `…partNumber` |
| `iso16757.5.8.scriptLimits` | u=2.0000 | same three-limit `Remedy::at_least` set when any cap is zero (`🔄part5.rs:567–594`) | **Yes** — sequential clears; fail row not emitted when caps > 0 (check id absent → gate cleared) |

---

## Collateral checks (fixer claim)

| Item | Result | Evidence |
|------|--------|----------|
| Dangling refs → `Remedy::one_of` existing ids | **PASS** | unchanged `📚️part4.rs` dangling `targetId` remedy lists live subject ids |
| Duplicate entity ids → rename `OneOf` | **PASS** | `check_unique_ids` emits `{id}-renamed` option (`📈️part1.rs:386–408`) |
| No fingerprint / dummy utilization shortcuts in new remedy paths | **PASS** | `check_sources_contain_no_fingerprint_gaming_patterns` guard unchanged; clearance/partNumber/scriptLimits use physical lengths or script caps, not `.len()` echo |
| Broken example: every fail has applicable remedy | **PASS** (by structure) | `📚️examples/🚫️broken/🧪️tests/📚️example/🦀️.rs` asserts applicable remedies; paths include new `.value.value` / `.parameterValues.{id}.value` leaves |
| Path parse + resolve on broken | **PASS** (by structure) | `emitted_subject_and_remedy_paths_parse_and_resolve` walks `broken_fixture()` remedy targets (`🔬️compliance-report/🦀️.rs:86–105`) |

---

## Non-blocking observations

- `fail()` still hard-codes u=2.0/1.0 for descriptive fails (searchTags, BIM Err, scriptLimits); gate cares about status flip, not those dummy quantities.
- BIM **Ok** path does not verify `resolved_geometry_id` exists in `geometry.objects` — pre-existing; remedy audit scope was gate clear, not geometry catalog completeness.
- Implementer 313/313 Summary not independently executed this session; source + gate-law trace align with `📓️impl-gate-iso16757.md`.
