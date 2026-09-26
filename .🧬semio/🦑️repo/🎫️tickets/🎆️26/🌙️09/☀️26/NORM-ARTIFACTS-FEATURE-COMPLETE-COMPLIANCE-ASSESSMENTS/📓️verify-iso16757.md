# Verify — ISO 16757 (`📇️iso16757`) — Round 3

**VERDICT: FAIL (4 blocking)**

**Test runs (executed 2026-09-26, `--skip-nx-cache -- --no-fail-fast`):**

```
bun nx run @semio-tech/norm-iso16757-rs:test
→ Summary [1.990s] 307 tests run: 307 passed, 0 skipped
```

Full log: `🗑️generated/verify-iso16757/test-r3.txt`

```
bun nx run @semio-tech/norm-artifact-contract-rs:test
→ Summary [0.354s] 48 tests run: 48 passed, 0 skipped
```

Full log: `🗑️generated/verify-iso16757/contract-r3.txt`

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | FAIL | 8 |
| R2 | FAIL | 4 |
| R3 | **FAIL** | **4** |

R1 (**FAIL, 8 blocking**): JSON inputs dump; stub mutation facets; missing oracle/jsonschema tests; Part 5 §8 not evaluating document script; identical en/de dynamic issues; non-applicable Fail remedies; weak example tests; uncovered surfaces/substitute_parameters/EditionProfile.

R2 (**FAIL, 4 blocking**): compliance engine, remedies, oracle, jsonschema, localized issues, introduce/retire mutation facets in Rust catalog, and coordinator extras A–E substantively fixed; **inputs still cleared catalogue/dictionary/geometry; leaf-meta coverage + perturbation tests absent; EditionProfile enum unlocalized; eight CRUD mutation ids in catalog.**

R3: R2 items **1–3 fixed** (B2 lazy TreeWindows + family field-meta/perturbation tests). R2 item **4 partially fixed** (Rust `KINDS` + folders use `introduce-*`/`retire-*`, but **`update-script-limits` CRUD verb remains** and **Python/cucumber mutate oracle still lists eight `create-*`/`delete-*` kinds**). New R3 findings: **snapshot/diff TS facets stub top-level collections as `string`**; **mutations TS `GeometryObject` still uses `unknown[]`**.

---

## Check table

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Subject completeness | **PASS** | `Iso16757Snapshot` holds catalogue + dictionary + geometry + selection + Part 5 exchange (`🧬️schema/📸️snapshot/🦀️.rs`). Checks read real fields across parts (`📈️part1.rs`, `📐️part2.rs`, `📚️part4.rs`, `🔄part5.rs`). No evaluate-path fixture constants. |
| 2 | Clause coverage | **PASS** | Parts 1/2/4/5 covered. §8 evaluates `partNumberRule.source` under `scriptLimits` (`🔄part5.rs:448–500`). `substitute_parameters` (`📐️part2.rs:419–423`). Surfaces, descriptive media, `EditionProfile` (`📐️part2.rs:367+`, `📚️part4.rs:203+`, `🔄part5.rs:20–58`). Perturbation test passes over default+broken (`🔬️compliance-report/🦀️.rs:372–414`). |
| 3 | Numerics | **PASS** | Default volume \(0.15×0.20×0.10=0.003\) m³ (`🔬️compliance-report/🦀️.rs:22–25`); part number `dn=50 → 550`, `dn=40 → 450` (`🔬️compliance-report/🦀️.rs:19–20,77–83`). |
| 4 | Applicability | **PASS** | `na()` with localized reasons (`📐️part2.rs:15–22`, `🔄part5.rs:525+`). |
| 5 | National annex | **PASS (N/A)** | No DE numeric annex; `AnnexChoice::En` throughout (`🧰common.rs:79`). |
| 6 | Report quality | **PASS** | Dynamic text uses distinct en/de (`copy(en, de)` throughout checks). Path parse+resolve test (`🔬️compliance-report/🦀️.rs:87–105`). All Fail remedies `applicable: true` (`rg applicable: false` → 0). ≥2 fail→pass remedy tests (`🔬️compliance-report/🦀️.rs:43–73`, `182–238`). Geometry map keys via field-meta `*` + resolver. |
| 7 | Examples | **PASS** | Demo/broken DSL decode + `evaluate` (`📚️examples/*/🧪️tests/📚️example/🦀️.rs`). Default complies; broken fails ≥2 with applicable remedies (`🚫️broken/…/🦀️.rs:17–20`). |
| 7b | Inputs UX | **PASS** | Full snapshot via `render_document_editor` — no clearing (`📥️inputs/🦀️.rs:19–20`). `iso16757_field_meta` with `[]`/`*` wildcards + localized choices including `editionProfile` (`🏷️field-meta/🦀️.rs:13–18,81,367–373`). Slot test passes collapsed+expanded (`📥️inputs/🧪️tests/🔬️unit/🦀️.rs:22–84`). Contract lazy-tree tests pass (`contract-r3.txt`). |
| 8 | Mutations & schema | **FAIL** | Rust catalog uses `introduce-*`/`retire-*` (`🧬️mutations/🦀️.rs:110–127`). **But `update-script-limits` CRUD verb** remains (folder `🚦️update-script-limits`, kind line 100, semantic verb `"update"` at `🚦️update-script-limits/🦠️mutation/🦀️.rs:17`). **Eight `create-*`/`delete-*` ids** still in Python mutate oracle (`🧪️tests/📇️mutate-iso16757-1/🐍️.py:44–53,77–86`) and cucumber feature (`🥒️.feature:97–106`). Snapshot TS facet stubs collections as `string` (`📸️snapshot/🟦️.ts:5–9`). Mutations TS `GeometryObject` has `unknown[]` (`🧬️mutations/🟦️.ts:110–114). |
| 9 | Tests | **PASS** | 307 executed, 0 skipped. Oracle + jsonschema wired (`🔬️compliance-report/🦀️.rs:122–178`). `field_meta_covers_every_editable_leaf_on_default_snapshot` + `every_editable_leaf_perturbation_changes_a_check` present and passing. |
| 10 | Stubs | **PASS (evaluate/mutations)** | No `todo!`/`unimplemented!` in checks. `pass()` no fake utilization (`🧰common.rs`). Catalogue panel is real `render_catalogue`. Doc-comment `stub` reference only in deleted-set-snapshot note (`🧬️mutations/🦀️.rs:19`). **Facet TS snapshot lane still stub-typed** (see check 8). |

---

## Round-2 blocking items (re-check)

| # | R2 item | R3 | Evidence |
|---|---------|-----|----------|
| 1 | Stop clearing catalogue/dictionary/geometry in Inputs | **FIXED** | `📥️inputs/🦀️.rs:19–20` passes full `document` to `render_document_editor`; no clone/clear. B2 lazy `TreeWindows` materialises subtrees on demand. Test `full_default_snapshot_inputs_expose_catalogue_sections_within_slots` asserts catalogue/dictionary/geometry sections + `setField` within retire slots (`📥️inputs/🧪️tests/🔬️unit/🦀️.rs:22–84`). **Owner: shared app-surface (fixed).** |
| 2 | EditionProfile choices + leaf-meta coverage test | **FIXED** | `EDITION_PROFILE` choices with distinct en/de (`🏷️field-meta/🦀️.rs:13–18,81`). Tests `edition_profile_choices_are_localized`, `field_meta_covers_every_editable_leaf_on_default_snapshot` (`🏷️field-meta/🦀️.rs:368–373`; `🔬️compliance-report/🦀️.rs:292–322`). **Owner: family.** |
| 3 | Scope-aware perturbation test (CORRECTION 13:43) | **FIXED** | `every_editable_leaf_perturbation_changes_a_check` walks default+broken DSL leaves; exempts descriptive name/title labels only (`🔬️compliance-report/🦀️.rs:276–414`). Binding checks in `📈️part1.rs:1177+`, `📚️part4.rs:259+`, `📐️part2.rs`, `🔄part5.rs:558+`. **Owner: family.** |
| 4 | Rename CRUD mutation ids to introduce/retire | **NOT FIXED** | Rust `KINDS` + triad folders renamed (`🧬️mutations/🦀️.rs:110–127`; folders `📦️introduce-product`, `🚫️retire-product`, …). **Remaining:** `update-script-limits` (`🧬️mutations/🦀️.rs:100`; folder `🚦️update-script-limits/`). **Stale:** Python oracle + cucumber still use `create-product`, `delete-product`, `create-subject`, `delete-subject`, `create-product-group`, `delete-product-group`, `create-property-definition`, `delete-property-definition` (`🐍️.py:44–53`; `🥒️.feature:97–106`) — drift from Rust/oracle `🔣️.json` (which correctly lists `introduce-*`/`retire-*`). **Owner: family.** |

---

## Round-1 items + extras A–E (still relevant)

| Item | Result | Evidence |
|------|--------|----------|
| R1-1 Structured inputs editor | **FIXED** | Full document structured editor (`📥️inputs/🦀️.rs:19–20`). |
| R1-2 Field-meta wildcards + localized choices | **FIXED** | Wildcards + all enum choices including `editionProfile` (`🏷️field-meta/🦀️.rs`). |
| R1-3 Mutation facets (introduce/retire) | **PASS** | Real TS diff/inverse on introduce/retire leaves; no `/** diff facet stub */`. |
| R1-4 Oracle + jsonschema tests | **PASS** | `🔬️compliance-report/🦀️.rs:122–178`. |
| R1-5 Part 5 §8 document script | **PASS** | `runtime.execute(&source, …, limits)` (`🔄part5.rs:485`). |
| R1-6 Localized dynamic issues | **PASS** | No identical en/de copies. |
| R1-7 Applicable Fail remedies | **PASS** | Remedy-law + flip tests (`🔬️compliance-report/🦀️.rs:43–73,182–238`). |
| R1-8 Example verdict tests | **PASS** | `complies()` / `≥2` fails (`📚️examples/*/🧪️tests/📚️example/🦀️.rs`). |
| A IFC/STEP structure | **PASS** | `validate_exchange` globalId/entity_type; STEP `#id` ref scan. |
| B `substitute_parameters` | **PASS** | Check + unit tests (`📐️part2.rs:419–423`). |
| C surfaces / media / EditionProfile | **FIXED** | Checks + UI choices (`🔄part5.rs:20–58`; `🏷️field-meta/🦀️.rs:13–18`). |
| D geometry map paths | **PASS** | Map keys + `[id=…]`; path test + field-meta `geometry.objects.*`. |
| E pass() + catalogue panel | **PASS** | `pass()` no fake utilization; `render_catalogue`. |
| CRUD mutation renames | **NOT FIXED** | See R2-4 above. |

---

## CORRECTION 13:27 (twelve causes)

| # | Cause | Result | Evidence |
|---|-------|--------|----------|
| 1 | Human en+de choice labels, not raw codes | **PASS** | `NormFieldChoice` rows throughout `🏷️field-meta/🦀️.rs`; test `choices_are_localized_not_raw_codes` (`🏷️field-meta/🦀️.rs:358–365`). |
| 2 | Every editable leaf has meta + coverage test | **PASS** | `field_meta_covers_every_editable_leaf_on_default_snapshot` (`🔬️compliance-report/🦀️.rs:292–322`). |
| 3 | Structured editor, not JSON dump | **PASS** | `render_document_editor` (`📥️inputs/🦀️.rs:19–20`). |
| 4 | Entity paths `[id=…]` + resolve test | **PASS** | `emitted_subject_and_remedy_paths_parse_and_resolve` (`🔬️compliance-report/🦀️.rs:87–105`). |
| 5 | ≥2 fail→pass remedy tests; applicable remedies | **PASS** | `remedy_law_writing_required_improves_or_passes` + `applying_applicable_fail_remedies_reduces_failures` (`🔬️compliance-report/🦀️.rs:43–73,182–238`). |
| 6 | Example tests decode DSL; broken ≥2 fails | **PASS** | `🚫️broken/…/🦀️.rs:17–20`. |
| 7 | Python oracle ±0.5% + third-party jsonschema | **PASS** | `python_compliance_oracle_matches_default_and_broken_within_half_percent` + `snapshot_validates_against_committed_json_schema` (`🔬️compliance-report/🦀️.rs:122–178`). No skip hatch. |
| 8 | Facets regenerated — no `Record<string, unknown>`, `unknown[]`, bare stubs | **FAIL** | Snapshot TS types top-level collections as `string` (`📸️snapshot/🟦️.ts:5–9`, `🧬️schema/🟦️.ts:5–9`). Mutations TS `GeometryObject`: `spaces/surfaces/ports: unknown[]`, `shape/symbolic?: unknown` (`🧬️mutations/🟦️.ts:110–114). JSON schema permissive stub (`📸️snapshot/🔣️.json:17–37` bare `"type":"object"`, empty `{}` for enums, `additionalProperties: true`). Guard helpers using `Record<string, unknown>` are acceptable; exported schema interfaces are not. |
| 9 | No tautologies / hardcodes / ignored editable fields | **PASS** | Perturbation test passes. Binding checks encode leaves into `computed`/`status`. |
| 10 | No trivially-true tests | **PASS** | Numeric/remedy/oracle tests assert concrete values. |
| 11 | Semantic mutation verbs, not CRUD | **FAIL** | `update-script-limits` (`🧬️mutations/🦀️.rs:100`; `🚦️update-script-limits/` folder; semantic verb `"update"`). Stale `create-*`/`delete-*` in `🐍️.py` + `🥒️.feature`. |
| 12 | Dynamic issue text localized (no `copy(x,x)`) | **PASS** | Distinct en/de templates throughout checks. |

---

## CORRECTION 13:43 findings

| Requirement | Result | Evidence |
|-------------|--------|----------|
| Every editable field read by ≥1 check | **PASS** | `every_editable_leaf_perturbation_changes_a_check` over default+broken; 0 inert leaves (`🔬️compliance-report/🦀️.rs:372–414`; test suite 307/307). |
| Scope-aware perturbation (N/A-in-default not exempt) | **PASS** | Test runs on both `Iso16757Snapshot::default()` and `::broken_fixture()`. |
| Only descriptive `name`/`title` labels exempt | **PASS** | Exempt set explicit in `is_descriptive_name_or_title_leaf` (`🔬️compliance-report/🦀️.rs:276–290`): `text`, `title`, `label*`, `shortName`, `locale` on name records. |
| Duplicated quantities → one source of truth | **PASS (N/A observed)** | Geometry bounds drive clearance/volume checks; catalogue property values referenced via definitions — no parallel building/zone duplicate scalars found. |
| Independent static audit | **PASS (consistent with automated test)** | `🗑️generated/verify-iso16757/ignored-fields-audit.txt` — checks reference catalogue/dictionary/geometry/part5 top-level fields; automated perturbation is binding proof. |

---

## Blocking fix list

1. **`🧬️schema/🧬️mutations/` — rename `update-script-limits` → semantic non-CRUD kind** (e.g. `change-script-limits`, matching peer `change-exchange-process`). Update folder `🚦️update-script-limits/`, Rust enum/`KINDS`, TS/GraphQL/proto/DSL/text/binary facets, oracle `🔣️.json`, fixtures, and `SemanticDescriptor.verb` (currently `"update"` at `🚦️update-script-limits/🦠️mutation/🦀️.rs:17`). Regenerate taxonomy. **Family-owned.**

2. **`🧪️tests/📇️mutate-iso16757-1/🐍️.py` + `🥒️.feature`** — Replace eight stale `create-*`/`delete-*` kind strings and vector keys with `introduce-*`/`retire-*` matching Rust `KINDS` and committed triad folder names (`📦️introduce-product`, `🚫️retire-product`, …). Python `KINDS` list must mirror `🧬️mutations/🦀️.rs:98–128` exactly. **Family-owned.**

3. **`🧬️schema/📸️snapshot/🟦️.ts`, `🧬️schema/🟦️.ts`, `🧬️schema/🔺️diff/🟦️.ts`, `📸️snapshot/🔣️.json`** — Regenerate snapshot/diff TS + JSON Schema from Rust `Iso16757Snapshot` with fully typed nested catalogue/dictionary/geometry/selection/part5 structures (not `catalogue: string` placeholders; not bare `"type":"object"` with `additionalProperties: true`). **Family-owned.**

4. **`🧬️schema/🧬️mutations/🟦️.ts`** — Replace `GeometryObject` `unknown[]`/`unknown` fields with typed `Space`, `Surface`, `Port` (and shape/symbolic union types matching Rust). **Family-owned.**

---

## Non-blocking observations

- `update-script-limits` is the only remaining CRUD-prefix kind in the Rust catalog; all other lifecycle kinds correctly use `introduce-*`/`retire-*`.
- JSON schema test passes against the permissive committed schema — test is green but schema does not yet enforce hierarchical structure (tighten with item 3).
- `partNumberRule.source` prefix fallback from `partNumberRule` row still acceptable; explicit row now present in field-meta (`🏷️field-meta/🦀️.rs:73`).
- Part 5 §8 runtime `1/(0)` probe after document script eval remains a runtime guard (`🔄part5.rs:489–501`), not sole gate.
- Mutate Rust adapter (`🧪️tests/📇️mutate-iso16757-1/🦀️.rs:64–94`) already lists introduce/retire — only Python/cucumber side drifted.

---

## Check id inventory (unchanged scope)

| Part | Sections | Pattern |
|------|----------|---------|
| 1 | 3.1–7.2, 10 | `iso16757.1.*` (`📈️part1.rs`) |
| 2 | 5.3.5–7.1 | `iso16757.2.*` (`📐️part2.rs`) |
| 4 | 4.3, 5.1, 6.3.2 | `iso16757.4.*` (`📚️part4.rs`) |
| 5 | 4.1, 6.1, 6.10, 8 | `iso16757.5.*` (`🔄part5.rs`) |

Part 3 out of scope. Part 2 full CSG→IFC geometry export not claimed.
