# Verify — VDI 3805 (`🏭️vdi3805`, Wave D Round 5 adversarial re-verify)

VERDICT: PASS (0 blocking)

**Runner (this session):**
- `bun nx run @semio-tech/norm-vdi3805-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [4.591s] 283 tests run: 283 passed, 0 skipped**
- `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → **Summary [0.128s] 51 tests run: 51 passed, 0 skipped**

**Fixer claim (`9e1aa4b2`):** 283/283 + 51/51, dangling `accessoryId` / `componentId` now use `Remedy::one_of` over `product.id`, `validate_structure` aligned — **283/283 and 51/51 confirmed independently; Round 4 sole blocker closed in source and covered by new test.**

**Round history:** Round 1 = FAIL (7). Round 2 = FAIL (8). Round 3 = FAIL (3). Round 4 = FAIL (1). **Round 5 = PASS (0)** — last open Wave D family.

---

## Round 5 — sole Round 4 blocker re-check

| # | Blocker | Result | Evidence |
|---|---------|--------|----------|
| 1 | Dangling `accessoryId` / `componentId` emit `Remedy::one_of` with existing catalogue `product.id` values (not `Remedy::exactly` 0→1); `validate_structure` and evaluate use the same id set; test asserts `one_of` + existing id | **PASS** | **Evaluate dangling accessory** — `Remedy::one_of(..., product_ids.iter().cloned().collect(), ...)` (`💡️inferences/🦀️.rs:1946–1953`). **Evaluate dangling component** — same pattern (`1984–1991`). **`check_part1` structure fall-through** — `accessoryId` / `componentId` branches use `document.catalog.products.iter().map(\|p\| p.id.clone()).collect()` (`538–555`). **Identifier set alignment** — `validate_structure` builds `product_ids` from `catalog.products[].id` (`🧬️schema/🦀️.rs:724`, dangling checks `769–777`); `check_catalog_integrity` uses the same `product.id` set (`💡️inferences/🦀️.rs:1490`, dangling filter `1920`, `1958`). Contrast reference pattern: index `productId` `one_of` (`1744–1747`). **No dangling-path `exactly` 0→1** — grep of dangling accessory/component Fail branches finds `one_of` only; remaining `Remedy::exactly` on `accessories[{i}].accessoryId` (`2047–2051`) is the separate “required accessory missing from catalogue” check, not the dangling-reference policy. **Test** — `dangling_accessory_and_component_ids_fail_with_one_of_existing_product_ids` perturbs to `__dangling__`, asserts Fail + en≠de + remedy with non-empty `options` containing an existing product id (`🔬️compliance-report/🦀️.rs:534–577`). |

---

## Round 5 — spot-check (prior PASS items)

| Item | Result | Evidence |
|------|--------|----------|
| German SubjectRef titles differ in words | **PASS (unchanged)** | e.g. `copy("Nominal diameter", "Nennweite")` (`💡️inferences/🦀️.rs:608`), `copy("Flow coefficient kvs", "Durchflusskoeffizient kvs")` (`617`), `copy("BBox maximum X", "Maximale X-Koordinate der Bounding-Box")` (`2112`). |
| Structure paths `catalog.file.*` only | **PASS (unchanged)** | `validate_structure` emits `catalog.file.manufacturer` / `charset` / `recordCount` (`🧬️schema/🦀️.rs:711–722`); structure `recordCount` remedy copies `catalog.file.recordCount` (`💡️inferences/🦀️.rs:516–517`). Zero `manufacturerFile` literals in evaluate inferences source. |
| Bare `id` not globally perturbation-exempt | **PASS (unchanged)** | `is_descriptive_name_or_title_leaf` exempts only `name` / `title` / `labelEn` / `labelDe` (`🔬️compliance-report/🦀️.rs:374–382`); `is_reference_or_entity_id_leaf` includes bare `id` and perturbs to `__dangling__` (`385–392`, `428–430`). |
| No fingerprint / `param_metric` gaming | **PASS (unchanged)** | Guard test (`🔬️compliance-report/🦀️.rs:579–586`); no `param_metric` / fingerprint folds in evaluate source. |

---

## Blocking fix list

None

---

## Non-blocking observations

- 283/283 (+1 vs Round 4) and 51/51 credible; no skipped tests; fixer count claim confirmed.
- Round 4 non-blocking note about `article_number` vs `product.id` mismatch is resolved — both paths now use `catalog.products[].id`.
- `accessories.required` Fail still emits `Remedy::exactly` 0→1 on `accessoryId` when a required accessory id is absent (`2047–2051`); acceptable separate policy from dangling-reference `one_of`.

---

# Verify — VDI 3805 (`🏭️vdi3805`, Wave D Round 4 adversarial re-verify)

VERDICT: FAIL (1 blocking)

**Runner (this session):**
- `bun nx run @semio-tech/norm-vdi3805-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [5.070s] 282 tests run: 282 passed, 0 skipped**
- `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → **Summary [0.175s] 51 tests run: 51 passed, 0 skipped**

**Fixer claim (`d135971f`):** 282/282 + 51/51, bare `id` no longer perturbation-exempt, SubjectRef titles distinct German, structure paths emit `catalog.file.*` only — **282/282 and 51/51 confirmed independently; Round 3 blockers #2 and #3 closed in source; Round 3 blocker #1 partially closed (id exemption + signature wiring) but dangling `accessoryId` / `componentId` still use `Remedy::exactly` instead of required `Remedy::one_of` with existing target ids.**

**Round history:** Round 1 = FAIL (7). Round 2 = FAIL (8). Round 3 = FAIL (3). **Round 4 = FAIL (1)** — fixer closed the three Round 3 code sites; one residual sub-criterion under perturbation/dangling-reference policy remains.

---

## Round 4 — three-blocker re-check (FAIL 3 list)

| # | Blocker | Result | Evidence |
|---|---------|--------|----------|
| 1 | Scope-aware perturbation; signature `(id, status, computed, limit, utilization)`; bare `id` not globally exempt; dangling refs Fail en+de with `one_of` existing ids | **FAIL** | **Closed:** `is_descriptive_name_or_title_leaf` exempts only `name` / `title` / `labelEn` / `labelDe` (`🔬️compliance-report/🦀️.rs:374–382`); reference ids perturbed to `__dangling__` via `is_reference_or_entity_id_leaf` (`385–392`, `428–430`); signature tuple excludes explanation (`394–407`); perturbation test passes over all assessed Blätter (`443–482`). **Blocking:** dangling `accessoryId` / `componentId` Fail with en+de but emit `Remedy::exactly` (0→1 placeholder), not `Remedy::one_of` with existing product ids — dedicated checks (`💡️inferences/🦀️.rs:1923–1931`, `1957–1965`); `validate_structure` dangling accessory/component paths fall through `check_part1` generic else branch (`538–544`) with the same `exactly` pattern. Contrast compliant `one_of` sites: index `productId` (`1726–1729`), `geometryRef` (`523–529`), `functionRefs` (`533–536`). |
| 2 | SubjectRef titles at former identical en/de sites are distinct real German (capitalization-only FAIL) | **PASS** | Former sites fixed: `copy("Nominal diameter", "Nennweite")` (`590`), `copy("Flow coefficient kvs", "Durchflusskoeffizient kvs")` (`600`, `609`), `copy("BBox maximum X", "Maximale X-Koordinate der Bounding-Box")` (`2086`). Family grep: 0 identical `copy("…","…")` SubjectRef or assess-title pairs; 0 capitalization-only pairs. Symbol/number-only explanation matches (e.g. `geometry.parameters.scale = {scale}.` at `2106`) allowed. |
| 3 | Structure diagnostic/remedy paths use `catalog.file.*`; no emitted `manufacturerFile.*` | **PASS** | `validate_structure` emits `catalog.file.manufacturer` / `charset` / `recordCount` (`🧬️schema/🦀️.rs:711–722`). Structure remedies copy `catalog.file.recordCount` (`💡️inferences/🦀️.rs:516–517`). Evaluate-path `*.rs` grep: zero `"manufacturerFile"` string literals in inferences/schema evaluate code (only diff fixtures, GraphQL patch alias, and guard test `🔬️compliance-report/🦀️.rs:527`). |

---

## Round 4 — spot-check (prior PASS items, not re-litigated)

| Item | Result | Evidence |
|------|--------|----------|
| One manufacturer header | **PASS (unchanged)** | Single header at `catalog.file`; mutation test asserts `catalog.file` only (`🏭️change-manufacturer-file/…/🦀️.rs:37–38`). |
| No flange default | **PASS (unchanged)** | `connection_type` uses `unwrap_or_default()` (`🧬️schema/🦀️.rs:329`); empty/non-list Fails (`💡️inferences/🦀️.rs:637–652`). |
| Blatt-sourced bounds | **PASS (unchanged)** | `sheet_numeric_bounds` → shared `SHEET_NUMERIC_BOUNDS_*` consts (`139–172`). |
| Typed accessories/components, ExtensionBag | **PASS (unchanged)** | GraphQL facet parity test (`🔬️compliance-report/🦀️.rs:502–530`); `BTreeMap<String, String>` bag (`🦀️.rs:196–198`). |
| Python oracle ±0.5 % | **PASS (unchanged)** | `python_oracle_agrees_on_every_assessed_blatt_within_half_percent` (`⚖️compliance-vdi3805-1/🦀️.rs:126–196`). |
| Catalogue COP cell = evaluate limit | **PASS (unchanged)** | `reference_tables()` + COP max test (`✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:34–39`). |
| No fingerprint gaming | **PASS (unchanged)** | Guard test (`🔬️compliance-report/🦀️.rs:534–540`); no `param_metric` / `pos_metric` / `fingerprint` / `id_score` in evaluate source. |

---

## Blocking fix list

1. **`💡️inferences/🦀️.rs:1923–1931`, `1957–1965`, `538–544`** — When `accessoryId` or `componentId` is dangling (including after perturbation to `__dangling__`), emit `Remedy::one_of` with existing catalogue product ids (same pattern as index `productId` at `1726–1729` and `geometryRef` at `523–529`), not `Remedy::exactly` with a 0→1 placeholder. Remedy copy may stay en+de; choices must enumerate valid targets.

---

## Non-blocking observations

- 282/282 and 51/51 credible; no skipped tests; fixer count claim confirmed.
- Round 3 blockers #2 (localized SubjectRef titles) and #3 (`catalog.file.*` paths) are substantively closed.
- `validate_structure` compares accessories to `article_number` set (`🧬️schema/🦀️.rs:724`, `769–772`) while evaluate dangling-accessory check uses `product.id` set (`1472`, `1902`) — align when fixing `one_of` choices.
- Diff wire schema still names patch field `manufacturerFile` (`🔺️diff/🟦️.ts:66`); acceptable mutation alias, not an emitted evaluate path.

---

# Verify — VDI 3805 (`🏭️vdi3805`, Wave D Round 3 re-verify)

VERDICT: FAIL (3 blocking)

**Runner (this session):**
- `bun nx run @semio-tech/norm-vdi3805-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [7.798s] 282 tests run: 282 passed, 0 skipped**
- `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` → **Summary [0.236s] 51 tests run: 51 passed, 0 skipped**
- Log: `🗑️generated/verify-vdi3805/test-r3b.txt`

**Fixer claim (`8e59b196`):** 282/282, gaming folds removed, all eight blockers closed — **282/282 confirmed independently; seven of eight blockers closed; three residual blockers below (perturbation `id` exemption, identical en/de labels, legacy `manufacturerFile.*` structure paths).**

**Round history:** Round 1 = FAIL (7). Round 2 = FAIL (8). Prior Round 3 = FAIL (8). **This re-verify = FAIL (3)** — major mechanical gaps from prior Round 3 are closed (perturbation test present, facets regenerated, multi-Blatt oracle ±0.5 %, `limits`/index/geometry/accessories wired, `ExtensionBag` typed, `reference_tables()` non-empty with COP parity).

---

## Eight-blocker re-check (user list)

| # | Blocker | Result | Evidence |
|---|---------|--------|----------|
| 1 | Scope-aware perturbation; signature `(id, status, computed, limit, utilization)`; exempt only descriptive `id`/`name`/`title`/`labelEn`/`labelDe` | **FAIL** | Test exists: `every_editable_leaf_perturbation_changes_a_check` (`🔬️compliance-report/🦀️.rs:432–470`) with correct signature (`385–398`) and per-Blatt subjects via `all_conforming_blatt_examples()` (`434–437`). **Blocking:** `is_descriptive_name_or_title_leaf` exempts **every** leaf named `id` (`374–382`), skipping reference ids (`catalog.products[].id`, `configuration.id`, `geometry[…].id`, `curves[…].id`) that must perturb to dangling values per ADDENDUM 14:42. |
| 2 | One manufacturer header; no stale-`catalog.file` test; divergent headers Fail | **FAIL** | Stale-header test **gone** (`🏭️change-manufacturer-file/…/🦀️.rs:37–38` asserts `catalog.file` only). Snapshot stores single header at `catalog.file` (`📸️snapshot/🟦️.ts:55–57`). Field-meta uses `catalog.file.*` (`✏️editor/🏷️field-meta/🦀️.rs:59–68`). **Blocking:** `validate_structure` still emits legacy paths `manufacturerFile.manufacturer` / `manufacturerFile.recordCount` (`🧬️schema/🦀️.rs:712–722`) and structure remedies copy them verbatim (`💡️inferences/🦀️.rs:487–517`), not `catalog.file.*`. |
| 3 | Facets: `AccessoryLink`/`CompositionLink`; no `GenericAttributes { _placeholder }`; `Product.id` | **PASS** | Root/snapshot/diff GraphQL: `accessories: [AccessoryLink!]!`, `components: [CompositionLink!]!`, `GenericAttributes { entries: … }`, `Product { id: String! … }` (`🧬️schema/🔗️.graphql:23–36`). Enforced by `facet_parity_accessories_components_generic_product_id` (`🔬️compliance-report/🦀️.rs:491–518`). |
| 4 | No hidden `connection_type → "flange"`; omitted mandatory Fails; code-list check | **PASS** | `attributes_from_records` uses `unwrap_or_default()` (empty), not `"flange"` (`🧬️schema/🦀️.rs:329–340`). `check_valve` / `check_radiator` Fail on empty or non-list `connectionType` (`💡️inferences/🦀️.rs:634–652`, `874–889`). Shared `CONNECTION_TYPE_CODES` (`🦀️.rs:274–275`). |
| 5 | `limits.*`, index, geometry (`geometryRef`), accessories required/quantity, components quantity, header version/BSN/created | **PASS** | `limits.*` (`💡️inferences/🦀️.rs:1496–1499`), index tags/dn/sheet (`2246–2332`), geometry bbox/connections/parameters (`2058–2110`), accessories required+quantity (`1970–2019`), components quantity (`2030–2052`), header (`2338–2444`). |
| 6 | Bounds from Blatt tables/code lists; value domains validated | **PASS** | `sheet_numeric_bounds` routes to shared `crate::SHEET_NUMERIC_BOUNDS_*` consts (`💡️inferences/🦀️.rs:139–172`); `code_list_for_key` + domain/range checks (`1350–1399`). `reference_tables()` non-empty from same consts (`📚️catalogue/🦀️.rs:138–154`); test proves catalogue COP max = `SHEET_NUMERIC_BOUNDS_53_COP.1` (`📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:34–39`). |
| 7 | Python oracle ±0.5 % on per-Blatt examples (Φ, n, Q, η, Qn, COP, mandatory keys) | **PASS** | `python_oracle_agrees_on_every_assessed_blatt_within_half_percent` (`⚖️compliance-vdi3805-1/🦀️.rs:126–196`) over `all_conforming_blatt_examples()`; oracle emits phi/n/q/eta/qn/cop/mandatory (`🐍️.py:103–144`). |
| 8 | `ExtensionBag` typed map; no `Record<string, unknown>` on bag | **PASS** | Rust `BTreeMap<String, String>` (`🦀️.rs:196–198`); TS `{ readonly [key: string]: ExtensionFieldValue }` (`🧬️schema/🧬️mutations/🟦️.ts:77–80`). No `Record<string, unknown>` on `ExtensionBag`. |

---

## Gaming-pattern grep (`*.rs` evaluate path)

| Pattern | Result | Evidence |
|---------|--------|----------|
| `param_metric`, `pos_metric`, `fingerprint`, `id_score`, `tag_fp` | **PASS** | Absent from `💡️inferences/🦀️.rs`; guarded by `check_sources_contain_no_fingerprint_gaming_patterns` (`🔬️compliance-report/🦀️.rs:522–528`). |
| `let _ =` in evaluate | **PASS** | Only in tests (`⚖️compliance-vdi3805-1/🦀️.rs:36+`, path-resolve `207–212`); absent from inference source per guard test. |
| `1e-9 *` gaming folds | **PASS** | Not present in evaluate; `.max(1e-12)` used only as divide guards (e.g. `1326`, `2079`). |
| `sheet_numeric_bounds` | **PASS (non-gaming)** | Function name retained but reads shared Blatt consts (`139–172`), not ad-hoc literals. |

---

## ADDENDA re-check

| Addendum | Result | Evidence |
|----------|--------|----------|
| 14:54 `reference_tables()` non-empty, same consts as `evaluate()`, one limit = cell | **PASS** | `catalogue_tables()` / `reference_tables()` (`📚️catalogue/🦀️.rs:138–154`); COP max cell test (`📚️catalogue/🧪️tests/🔬️unit/🦀️.rs:34–39`). |
| 14:42 audit gaming instances + `let _ =` + perturbation exemptions | **FAIL** | Prior audit instances (`let _ = actual_records/curve_id/document`) **removed**. Perturbation **present** but `id` leaf blanket exemption remains (blocker #1). |
| 14:37 fingerprint / explanation-only perturbation | **PASS** | Signature excludes explanation (`385–398`); no fingerprint folds in source. |
| Duplicate ids / dangling refs Fail with en+de + `one_of` remedy | **PASS** | `vdi3805.1.products.uniqueId` (`1673–1697`); dangling index `one_of` (`1726–1729`); accessories/components dangling (`1924–1965`). |
| Identical en/de prose | **FAIL** | `copy("DN", "DN")` (`💡️inferences/🦀️.rs:590`), `copy("kvs", "kvs")` (`609`), `copy("BBox max X", "BBox max X")` (`2087`). |

---

## Blocking fix list

1. **`🔬️compliance-report/🦀️.rs:374–382`** — Narrow `is_descriptive_name_or_title_leaf`: do **not** exempt bare `id` globally. Exempt only entity-label ids (e.g. geometry/curve display names if truly descriptive) while **perturbing** `catalog.products[].id`, `configuration.id`, `geometry[…].id`, `curves[…].id`, `accessoryId`, `componentId`, `productId`, `geometryRef` to dangling values and asserting signature change.

2. **`💡️inferences/🦀️.rs:590,609,2087`** — Replace identical en/de `SubjectRef` titles with distinct localized strings (e.g. `"Nominal diameter"` / `"Nennweite"`, `"Flow coefficient kvs"` / `"Durchflusskoeffizient kvs"`, `"BBox maximum X"` / `"BBox Maximum X"`).

3. **`🧬️schema/🦀️.rs:712–722` + `💡️inferences/🦀️.rs:487–517`** — Rename structure diagnostic/remedy paths from `manufacturerFile.*` to `catalog.file.*` (and remedy copy text) so the single stored header facet matches every emitted writable path.

---

## Non-blocking observations

- 282/282 and 51/51 are credible; no skipped tests.
- Prior Round 3 gaps (no perturbation test, facet drift, valve-only oracle, ignored leaves) are substantively closed.
- `every_emitted_path_resolves` only covers valve datasets and does not assert `get_value_at_path` success (`207–212`) — consider extending once `manufacturerFile.*` paths are fixed.
- Diff wire schema still names patch field `manufacturerFile` (`🔺️diff/🟦️.ts:66`) while snapshot stores `catalog.file` — acceptable as mutation patch alias if documented; not a stored duplicate.
