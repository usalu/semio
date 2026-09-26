# Verify — EN 1991 (🏋️) adversarial family verification

**Verifier:** Wave D Round 2 (read-only)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1991.md` — 70/70 tests, no gaps  
**Test run:** `bun nx run @semio-tech/norm-en1991-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary line:** `Summary [   0.559s] 70 tests run: 70 passed, 0 skipped` (cache bypassed; log `🗑️generated/verify-en1991/test-r2.txt`)

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | **FAIL** | 10 |
| R2 | **FAIL** | 6 |

**VERDICT: FAIL (6 blocking)**

---

## Check table

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **FAIL** | Characteristic actions are derived for imposed/snow/wind/thermal/construction/accidental/bridge/crane/silo (`💡️inferences/🦀️.rs:89–340`). **Five editable top-level leaves are never read in `evaluate()`:** `width` (`📸️snapshot/🦀️.rs:213`), `bridgeLaneWidth` (`📸️snapshot/🦀️.rs:240`), `bridgeLoadGroup` value (only emitted as SubjectRef path `bridgeLoadGroup` at `:300`; loop hardcodes all five groups `:271–305`), `fireCompartmentArea` / `fireCompartmentHeight` (`📸️snapshot/🦀️.rs:228–229`; fire checks at `:116–135` use opening factor & thermal inertia only). `AccidentalCase` is a flat record with impact **and** explosion inputs always present (`🦀️.rs:113–122`) — not a discriminated schema per kind. |
| 2 | Clause coverage | **FAIL** | Parts 1-1…1-7, 2–4 covered with derived limits. **Wind:** `c_pe,10` table conformance check exists (`💡️inferences/🦀️.rs:158–187`) but **`c_pe,1` is not validated** against `tabulated_cpe1` (`🦀️.rs:159,206` — only used in `max(c_pe10,c_pe1)` for pressure). **Bridge:** `bridgeLoadGroup` selection does not gate which EN 1991-2 Table 4.4a group is assessed — all five groups always run (`:271–305`). **Fire (when claimed):** compartment geometry fields exist but do not enter Annex A / NA formulae. EN 1991-1-2 correctly N/A when `fireClaimed=false` (`:116–117`). |
| 3 | Numerics | **PASS** | Hand recomputation within 0.5% — see `🗑️generated/verify-en1991/hand-numerics.txt`. Snow zone 2 @ 150 m → 850 Pa; µ=0.8 → 680 Pa; DE wind NA B.3 zone 2 cat 2 @ z=12 m → q_p=580 Pa, w=464 Pa; B1 DE q_k=2000 Pa; α_A(240 m²)=1.0; RC 0.2 m → g_k=5000 Pa. Rust tests align (`compliance-report/🦀️.rs:116–127`, `⚖️compliance/🦀️.rs:4–65`). |
| 4 | Applicability | **PASS** | Explicit `NotApplicable` for empty floors/roofs/wind, unclaimed fire/bridge/crane/silo/accidental (`💡️inferences/🦀️.rs:86–87,116–117,138–139,150–151,222–223,239–240,308–309,321–322`). |
| 5 | National annex | **PASS** | DE vs EN diverge in snow, bridge α_Q/α_q, crane horizontal, silo patch (`⚖️compliance/🦀️.rs:18–78`). Partitions DE 0.8 vs EN 1.0 kN/m² (`🦀️.rs:665–669`). `coastOrIsland` wired into `peak_velocity_pressure_pa` (`💡️inferences/🦀️.rs:205`). |
| 6 | Report quality | **PASS** | Entity paths use `[id=…]` (`💡️inferences/🦀️.rs:91,111,145,160,207,228`); resolve test passes (`compliance-report/🦀️.rs:67–88`). Localized en+de titles/explanations differ. Dual fail→pass remedy test on imposed + snow (`compliance-report/🦀️.rs:43–65`). |
| 7 | Examples | **PASS** | `de_office_compliant` / `multi_fail_noncompliant` decode DSL + evaluate (`📚️examples/*/🧪️tests/📚️example/🦀️.rs`). Compliant example uses tight DE LM1 UDL 3600 Pa (`📚️examples/🧬️subjects/🦀️.rs:13`). Non-compliant asserts ≥2 fails. |
| 7b | Inputs UX | **FAIL** | Structured editor via `render_document_editor` (`📥️inputs/🦀️.rs:27`). **Raw or identical en/de choice labels remain:** `constructionActivity` `formwork`/`landing`/`working_platform` (`field-meta/🦀️.rs:41`), `roofType` `cylindrical` (`:81`), `terrainCategory` value `"0"` → `"0"`/`"0"` (`:31`). **`bridgeLane` choices mislabel lanes as “Zone 1/2/3”** (`:44`). Most other enums fixed (annex, category incl. I/J/K, material, fire, silo). |
| 8 | Mutations & schema | **PASS** | `En1991Mutation` = **80** variants; `KINDS` = **80** (`🧬️mutations/🦀️.rs:173–254`); taxonomy fixture **80** rows (`🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`); oracle manifest contains all 80 kinds; `kinds_catalog_tests` wired (`🧬️mutations/🦀️.rs:507`). Snapshot facets typed (no bare `object` on subject interfaces in `📸️snapshot/🟦️.ts`). |
| 9 | Tests | **FAIL** | 70/70 green, no skips in runner. Python oracle ±0.5% (`compliance-report/🦀️.rs:266–293`). Third-party jsonschema validation runs when installed (`:295–309`) but **test exits 0 on `ImportError`** — escape hatch violates brief “skipped/ignored blocking”. **No scope-aware perturbation test** for editable leaves (CORRECTION 13:43). **No test iterating default snapshot leaves for field-meta en+de labels** (CORRECTION 13:27 #2). |
| 10 | Stubs | **FAIL** | No `todo!`/`unimplemented!` in evaluate path. **Editable fire compartment area/height and bridge lane width are decorative** — mutations exist, checks ignore values. Oracle manifest `_comment` still prose-references “32 kinds” (`🔮️oracles/🔣️.json:4,18`) though catalog now lists 80. Catalogue panel placeholder docstring is UI-only (`📌️panels/📚️catalogue/🦀️.rs:3`). |

---

## Prior Round-1 blocking items (re-check)

| R1 # | Topic | R2 | Evidence |
|------|-------|-----|----------|
| 1 | Thermal tautology (`required_delta_t`) | **FIXED** | Limit from `part_1_5::required_delta_t(...)` (`💡️inferences/🦀️.rs:212–215`); no free limit scalar. |
| 2 | `FloorArea.area` ignored | **FIXED** | Used in `imposed_qk_reduced_pa` / α_A / α_n (`:90–95`, `🦀️.rs:639–662`). |
| 3 | Wind zone / `c_pe` not validated | **PARTIAL** | `c_pe,10` vs Tables 7.1–7.4 (`💡️inferences/🦀️.rs:158–187`). **`c_pe,1` still user-only.** |
| 4 | `coast_or_island` ignored | **FIXED** | Passed to `peak_velocity_pressure_pa` (`💡️inferences/🦀️.rs:205`). |
| 5 | EN 1991-1-2 absent | **FIXED** (gated) | `part_1_2` checks when `fireClaimed` (`:116–135`); N/A otherwise. |
| 6 | Categories I/J/K | **FIXED** | Table values (`🦀️.rs:612,629`); localized choices (`field-meta/🦀️.rs:73`). |
| 7 | α_A / α_n | **FIXED** | Implemented (`🦀️.rs:638–662`); used in imposed check. |
| 8 | LM3 / LM4 | **FIXED** | Checks `:262–269`; bridge test asserts ids (`compliance-report/🦀️.rs:216–234`). |
| 9 | `[id=…]` paths | **FIXED** | Throughout entity SubjectRef/remedy paths; resolve test (`compliance-report/🦀️.rs:67–88`). |
| 10 | Dual remedy test | **FIXED** | `remedy_law_two_distinct_checks_flip_to_pass` (`compliance-report/🦀️.rs:43–65`). |
| — | Human en+de enum labels | **PARTIAL** | Many fixed; see 7b failures above. |
| — | KINDS / oracle / taxonomy sync | **FIXED** | 80 / 80 / 80; guard test runs. |
| — | Tight compliant bridge UDL | **FIXED** | `assumed_bridge_udl: 3600` (`📚️examples/🧬️subjects/🦀️.rs:13`). |
| — | Example DSL decode+evaluate | **FIXED** | Both example tests assert evaluate verdict. |
| — | LM1 span moment | **FIXED** | `assumed_m = tandem * span / 4` vs `req_m` (`💡️inferences/🦀️.rs:245–249`). |

---

## CORRECTION 13:27 (twelve causes)

| # | Cause | Result | Evidence |
|---|-------|--------|----------|
| 1 | Human en+de choice labels | **FAIL** | Raw/de=wire: `formwork`, `cylindrical`, `working_platform` (`field-meta/🦀️.rs:41,81`). |
| 2 | Every editable leaf has meta (+ test) | **FAIL** | Meta table is broad; **no automated test** walks default snapshot leaves asserting en+de label. |
| 3 | Structured editor (not JSON dump) | **PASS** | `render_document_editor` (`📥️inputs/🦀️.rs:27`). |
| 4 | `[id=…]` paths + resolve test | **PASS** | `compliance-report/🦀️.rs:67–88`. |
| 5 | ≥2 fail→pass remedy tests | **PASS** | `:43–65`. |
| 6 | Example decode + verdict tests | **PASS** | Example package tests. |
| 7 | Python oracle + third-party jsonschema | **PARTIAL** | Oracle PASS; jsonschema runs here but **allows silent skip** (`:306–307`). |
| 8 | Facets regenerated, no loose typing | **PASS** | Typed snapshot TS; taxonomy 80 rows. Guard helpers use `Record<string, unknown>` only in generated guard fns (framework pattern). |
| 9 | No tautologies / ignored fields | **FAIL** | Bridge group checks always dimensionless 1/0 (`💡️inferences/🦀️.rs:304`); five unread editable fields (audit). |
| 10 | No trivial tests | **PASS** | Numeric assertions present in compliance tests. |
| 11 | Semantic mutation verbs | **PASS** | `change-*`, `insert-*`, `remove-*` — no CRUD. |
| 12 | Dynamic localized issue text | **PASS** | en/de explanation strings differ with numeric detail (`💡️inferences/🦀️.rs:95,209,215`). |

---

## CORRECTION 13:43 (editable-field reads + perturbation test)

| Finding | Result |
|---------|--------|
| Scope-aware perturbation test exists | **FAIL** — no test in family (`🗑️generated/verify-en1991/ignored-fields-audit.txt`: `Perturbation test present: False`) |
| Every editable leaf read by ≥1 check | **FAIL** — static audit: unread top-level **`width`**, **`bridgeLaneWidth`**, **`bridgeLoadGroup`** (value), **`fireCompartmentArea`**, **`fireCompartmentHeight`** |
| Kind-specific leaves on discriminated schema | **FAIL** — `AccidentalCase` flat (`🦀️.rs:113–122`); impact vehicle fields present on explosion rows |
| `name`/`title` entity labels exempt | OK — only `id`-based entity labels |

Full static audit: `🗑️generated/verify-en1991/ignored-fields-audit.txt`

---

## Hand numerics (check 3)

See `🗑️generated/verify-en1991/hand-numerics.txt`. Summary: DE snow s_k(2,150 m)=850 Pa; roof s=680 Pa; q_p(2,2,12 m)=580 Pa; w=464 Pa; q_k(B1)=2000 Pa; g_k(RC,0.2 m)=5000 Pa.

---

## Taxonomy / mutations cross-check (impl claim)

| Source | Count | Notes |
|--------|------:|-------|
| `En1991Mutation` enum | 80 | Matches `KINDS` |
| Taxonomy fixture (`artifact: 🏋️en1991`) | 80 | Present in `🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` |
| Oracle manifest kind strings | 80 | All `KINDS` found |
| `kinds_match_the_enum_and_the_catalog` | runs | In 70-test suite |

---

## Blocking fix list

1. **`🧬️schema/💡️inferences/🧪️tests/` (new module)** — Add **scope-aware perturbation test** on `de_office_compliant()` (+ fire/bridge variants): for each editable leaf in the committed example (respecting `fireClaimed`, `bridgeClaimed`, `kind`, etc.), perturb value, re-run `evaluate()`, assert ≥1 check’s computed value or status changes; exempt descriptive entity labels only.

2. **`🧬️schema/💡️inferences/🦀️.rs` + `🦀️.rs` entity types** — **Read or remove** unread editable fields: wire `bridgeLaneWidth` into EN 1991-2 lane/tandem logic; wire `bridgeLoadGroup` so only the selected Table 4.4a group is assessed (not all five); wire `fireCompartmentArea`/`fireCompartmentHeight` into parametric fire / Annex E formulae when `fireClaimed`; use `width` in wind exposure or orography if within scope, or drop from editable subject if out of scope (do **not** narrow scope silently — implement the norm link).

3. **`🦀️.rs` `AccidentalCase` + facets/examples/mutations** — Replace flat accidental record with **discriminated** impact vs explosion variants so vehicle inputs appear only on impact rows and explosion inputs only on explosion rows; update field-meta and perturbation coverage.

4. **`✏️editor/🏷️field-meta/🦀️.rs`** — Fix remaining **human de labels**: `constructionActivity` (formwork, landing, working_platform), `roofType` cylindrical, `terrainCategory` `"0"` (sea/open water), **`bridgeLane`** (lane numbers, not “Zone 1/2/3”). Add **`field_meta_coverage` test** iterating default snapshot camelCase leaves (and list item leaves) asserting `label_en` + `label_de` present.

5. **`💡️inferences/🦀️.rs` `part_1_4`** — Add **`c_pe,1` table conformance check** mirroring `c_pe,10` (Tables 7.1–7.4 / 1 m² interpolation); fail with remedy targeting `windFaces[id=…].cPe1` when user coefficient disagrees with `tabulated_cpe1`.

6. **`compliance-report/🦀️.rs` `snapshot_json_validates_against_schema`** — Remove **`ImportError` → exit 0** skip; fail CI when third-party `jsonschema` is unavailable or when validation fails.

---

## Non-blocking observations

- Default snapshot still sets generous bridge assumed loads while `bridgeClaimed=false` — harmless but masks sensitivity until bridge is claimed.
- Wind check uses `max(|c_pe10|,|c_pe1|)` for pressure even when only `c_pe10` is conformance-checked — inconsistent until fix #5 lands.
- Oracle manifest narrative still mentions “32 kinds”; catalog content is correct at 80.
- `bridgeLoadGroup` fail remedy targets `assumedBridgeUdl` even when failure is LM3/LM4/tandem — remedy targeting could be sharper.
- EN 1991-1-2 remains N/A in default office workflow (`fireClaimed=false`); acceptable if product positions fire as opt-in, but fire fields remain editable in Inputs when not claimed.
