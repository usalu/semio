# Verify — VDI 3805 (`🏭️vdi3805`, Wave D Round 3 read-only)

VERDICT: FAIL (8 blocking)

**Runner:** `bun nx run @semio-tech/norm-vdi3805-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [0.879s] 255 tests run: 255 passed, 0 skipped** (log: `🗑️generated/verify-vdi3805/test-r3.txt`).

**Round history:** Round 1 = **FAIL (7 blocking)**. Round 2 = **FAIL (8 blocking)**. Round 3 = **FAIL (8 blocking)** — 255/255 execution is real and Round-2 mechanical fixes largely landed (sheet routing, typed Blatt 2–6 checks, writable remedies, field-meta walk, path-resolve test, distinct en/de copy), but **feature-complete compliance assessment is still not met**: no scope-aware perturbation test, duplicated header quantities, facet drift on accessories/components and root/diff GraphQL, many editable leaves still unread, and operative-sheet rules still use invented bounds rather than Blatt-sourced code lists.

---

## Round 1 blocker re-check

| # | Round-1 item | Round-3 | Evidence |
|---|----------------|---------|----------|
| 1 | `[id=]` path selectors | **FIXED** | Remedies use `catalog.products[id={article}]` (`💡️inferences/🦀️.rs:89–90`). Test `product_paths_use_id_selectors` (`🔬️compliance-report/🦀️.rs:181–185`). |
| 2 | Field meta / `empty_field_meta` | **FIXED** | `vdi3805_field_meta` TABLE covers correctionAsOf, index, accessories, components, geometry, pump/radiator/heat-gen (`✏️editor/🏷️field-meta/🦀️.rs:36–138`). Test `every_editable_leaf_has_en_de_field_meta` (`🔬️compliance-report/🦀️.rs:219–262`). |
| 3 | Generic operative identity-only Pass | **FIXED** | `check_operative_sheet` mandatory keys + `sheet_numeric_bounds` + curve monotonicity (`💡️inferences/🦀️.rs:908–1059`). `define_vdi_part` default arm calls `check_sheet_product` (`1480–1485`). **Non-blocking:** code-list values still not Blatt-sourced. |
| 4 | Edition profiles Legacy vs Current | **FIXED** | `check_edition_profiles` + writable leaf remedies (`💡️inferences/🦀️.rs:1259–1321`). Tests `edition_profile_legacy_missing_keys_fails`, `edition_profile_fail_remedy_is_applicable_on_attribute_leaf` (`🔬️compliance-report/🦀️.rs:136–146, 286–300`). |
| 5 | Records authoritative (Blatt 2–6) | **PARTIAL** | Sync + round-trip retained. **Still blocking:** hidden default `connection_type → "flange"` when 210 omits key (`🧬️schema/🦀️.rs:333`). |
| 6 | Python oracle + jsonschema | **PARTIAL** | Oracle extended to Blatt 3/5/6 + representative mandatory (`🐍️.py:103–132`). Rust host still runs **valve datasets only** (`⚖️compliance-vdi3805-1/🦀️.rs:37`); ±0.5 % numeric assert only on kvs for sheet-2 valve (`88–92`). jsonschema test passes on empty-accessories conforming snapshot (`109–117`). |
| 7 | Curve remedies scalar + applyRemedy | **FIXED** | Scalar `points[i].y`/`.x` remedies + `apply_remedy_flips_curve_monotonicity_fail_to_pass` (`🔬️compliance-report/🦀️.rs:149–177`). |

---

## Round 2 blocker re-check

| # | Round-2 item | Round-3 | Evidence |
|---|----------------|---------|----------|
| 1 | Sheet routing by `product.sheet` | **FIXED** | `check_sheet_product` matches `product.sheet.0` (`💡️inferences/🦀️.rs:1064–1082`). Test `sheet_routing_uses_product_sheet_even_when_attributes_generic` (`🔬️compliance-report/🦀️.rs:303–332`). |
| 2 | Operative Blatt rules beyond key presence | **PARTIAL** | Mandatory + `sheet_numeric_bounds` + curve monotonicity added (`908–1059`, `137–148`). **Still blocking:** ranges/code lists invented in Rust, not sourced from Blatt tables; `filter_class`, `type_code`, `connection_type` not validated as enumerated Blatt codes. |
| 3 | Typed fields + Part 1 integrity | **PARTIAL** | Radiator L/H/D, pump DN/head/power, heat-gen temps checked (`652–905`, `1086–1224`). **Still blocking:** `connectionType` on sheet-2 typed valve unread; `limits.*` unread; geometry bbox unread. |
| 4 | Writable remedies (not `records` root) | **FIXED** | Sync/edition/mandatory target attribute leaves. Tests `sync_fail_remedy_targets_writable_attribute_leaf`, `edition_profile_fail_remedy_is_applicable_on_attribute_leaf` (`🔬️compliance-report/🦀️.rs:265–300`). |
| 5 | Field meta for all editable leaves | **FIXED** | See Round-1 #2. |
| 6 | Facets regenerated (Product.id, GenericAttributes) | **FAIL** | Snapshot `📸️snapshot/🔗️.graphql` has `Product.id` + `GenericAttributes.entries` (`26–33`). **Still blocking:** root `🧬️schema/🔗️.graphql` + `🔺️diff/🔗️.graphql` retain `GenericAttributes { _placeholder: Boolean }` and `Product` without `id` (`22–26`); `accessories`/`components` are `[String!]!` in snapshot GraphQL/JSON/proto but Rust `AccessoryLink`/`CompositionLink` (`🦀️.rs:223–235`, `📸️snapshot/🔣️.json:424–434`, `📸️snapshot/🛰️.proto:62–63`). |
| 7 | Oracle multi-Blatt + path resolve | **PARTIAL** | `every_emitted_path_resolves` added (`🔬️compliance-report/🦀️.rs:197–216`). Oracle host still valve-only; no ±0.5 % for Φ/Q/η/Qn. |
| 8 | Localization `copy(x,x)` | **FIXED** | Distinct en/de throughout `evaluate()` (e.g. `💡️inferences/🦀️.rs:569–575`, `986–987`). |

---

## Check matrix (brief §Checks 1–10)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **FAIL** | Hierarchical snapshot with typed attributes + native records. **Blocking:** duplicated `manufacturerFile` vs `catalog.file` with intentional divergence (`🏭️change-manufacturer-file/🧪️tests/✏️renames-the-header-manufacturer-to-acme/🦀️.rs:39`); hidden `connection_type: "flange"` default (`🧬️schema/🦀️.rs:333`); `limits.*`, geometry bbox, index tags/dn/sheet, `connectionType` on sheet-2 valve unread (audit: `🗑️generated/verify-vdi3805/ignored-fields-audit.txt`). |
| 2 | Clause coverage | **FAIL** | Blatt 2–6 typed checks solid. Operative sheets: mandatory keys + invented `sheet_numeric_bounds` (`137–148`) — not Blatt-sourced code lists (e.g. `filter_class` on sheet 19, `type_code` enums). `check_operative_sheet_coverage` wired (`1359`) as N/A summary only. |
| 3 | Numerics | **PASS** | DN50 kvs 2.50 m³/h ↔ remedy `2.5/3600` m³/s (`🔬️compliance-report/🦀️.rs:31–41`, `🧬️schema/🦀️.rs:250–258`). DN 47 ∉ series (`nonconforming_valve_dataset`). recordCount mismatch (`🦀️.rs:1377–1378`). |
| 4 | Applicability | **PASS** | Reserved/historical/empty-catalogue/operative-without-product N/A gates + tests (`🔬️compliance-report/🦀️.rs:83–106, 188–193`). |
| 5 | National annex | **PASS** | VDI 3805 DE-origin; `ANNEX = De` (`🧬️schema/🦀️.rs:228`). |
| 6 | Report quality | **PASS** | en+de titles/explanations; `[id=]` paths; `every_emitted_path_resolves`; ≥2 fail→pass tests (kvs, dn, recordCount, curve, sync); applicable remedies on writable leaves. |
| 7 | Examples | **FAIL** | Only `conforming_valve_dataset` (sheet 2) + `nonconforming_valve_dataset` (`🦀️.rs:1277–1372`). No committed examples per operative Blatt for scope-aware perturbation (brief ADDENDUM 13:43). |
| 7b | Inputs UX | **PASS** | Structured editor + `vdi3805_field_meta` wired (`📥️inputs/🦀️.rs:20`); en+de labels + SI units + choice labels (`✏️editor/🏷️field-meta/🦀️.rs`). |
| 8 | Mutations & schema | **FAIL** | Semantic `add-`/`remove-`/`change-` mutations (`🧬️mutations/🦀️.rs`). Facet drift: accessories/components `string[]` vs `AccessoryLink`/`CompositionLink`; root/diff GraphQL stubs (see Round-2 #6). |
| 9 | Tests | **FAIL** | 255/255 passed, 0 skipped. **Blocking gaps:** no `every_editable_leaf_perturbation_changes_a_check` (peer: iso16757 `🔬️compliance-report/🦀️.rs:371`); oracle host valve-only; jsonschema passes only because default accessories/components are empty. |
| 10 | Stubs | **FAIL** | `GenericAttributes { _placeholder }` in `🧬️schema/🔗️.graphql:22` and `🔺️diff/🔗️.graphql:22`. Catalogue panel headline placeholder comment (`📌️panels/📚️catalogue/🦀️.rs:3`). No `todo!` in evaluate path. |

---

## CORRECTION 13:27 self-check (12 causes)

| # | Cause | Result |
|---|--------|--------|
| 1 | Human en+de choice labels | **PASS** (`✏️editor/🏷️field-meta/🦀️.rs:5–23`) |
| 2 | Every editable leaf has meta + test | **PASS** — meta walk test present; **read wiring still FAIL** (see #9, 13:43) |
| 3 | Structured editor, not JSON | **PASS** (`📥️inputs/🦀️.rs:20`) |
| 4 | `[id=]` paths + resolve test | **PASS** (`🔬️compliance-report/🦀️.rs:181–185, 197–216`) |
| 5 | ≥2 fail→pass remedy tests, writable targets | **PASS** (kvs, dn, recordCount, curve, sync, edition) |
| 6 | Example DSL verdict tests | **PASS** (`🔬️compliance-report/🦀️.rs:6–28`) |
| 7 | Oracle ±0.5 % + jsonschema | **PARTIAL** — jsonschema runs; oracle valve-only; ±0.5 % on kvs only |
| 8 | Facets regenerated, no stubs | **FAIL** — root/diff GraphQL stubs; accessories/components type mismatch across facets |
| 9 | No tautologies / ignored fields | **FAIL** — ≥18 ignored leaves in default valve; duplicate header |
| 10 | No trivially-true tests | **PASS** |
| 11 | Semantic mutation names | **PASS** — `change-manufacturer-file`, `add-product`, `remove-curve`, etc. |
| 12 | Localized dynamic text | **PASS** — distinct en/de |

---

## CORRECTION 13:43 findings

| Requirement | Result | Evidence |
|-------------|--------|----------|
| Scope-aware perturbation test (each editable leaf → ≥1 check changes) | **FAIL** | No `perturb_*` / `every_editable_leaf_perturbation_changes_a_check` in vdi3805 tree (peer iso16757 has it at `📇️iso16757/.../🔬️compliance-report/🦀️.rs:371`). |
| N/A-in-default leaves not exempt — committed example per Blatt | **FAIL** | Only sheet-2 valve examples in `📚️examples/`; tests build radiator/pump inline (`🔬️compliance-report/🦀️.rs:109–124, 303–332`) but no persisted per-Blatt fixtures for perturbation. |
| Duplicated quantities → one source of truth | **FAIL** | `manufacturerFile` and `catalog.file` are separate copies; mutation test **requires** them to diverge (`🏭️change-manufacturer-file/🧪️tests/✏️renames-the-header-manufacturer-to-acme/🦀️.rs:3, 39`); `validate_structure` reads `catalog.file` only (`🧬️schema/🦀️.rs:643–654`). |
| Static ignored-fields audit | **FAIL** | `🗑️generated/verify-vdi3805/ignored-fields-audit.txt` — limits×4, BSN×3, headerVersion, created, index tags/dn/sheet, geometry bbox×6, connectionType (sheet 2), extensions, accessories required/quantity. |

---

## Blocking fix list

1. **`🔬️compliance-report/🦀️.rs` + `📚️examples/`** — Add `every_editable_leaf_perturbation_changes_a_check` mirroring iso16757 (`perturb_dsl_leaf` + `report_signature` + scope-aware subjects). Commit one realistic conforming example per in-scope Blatt (3, 4, 5, 6, 16, 19, 53, 60 at minimum) under `📚️examples/`; perturb each applicable leaf there and assert ≥1 check computed value/status changes. Exempt only descriptive `name`/`title` entity labels.

2. **`🦀️.rs` + `🧬️schema/🦀️.rs` + `🏭️change-manufacturer-file/`** — Resolve `manufacturerFile` vs `catalog.file` to **one** authoritative header (remove duplicate or add sync Fail when they diverge). Delete the test assertion that catalog.file stays stale after `change-manufacturer-file` (`🏭️change-manufacturer-file/🧪️tests/✏️renames-the-header-manufacturer-to-acme/🦀️.rs:39`). Field meta must expose a single header path.

3. **`📸️snapshot/🔣️.json` + `📸️snapshot/🔗️.graphql` + `📸️snapshot/🛰️.proto` + `🧬️schema/🔗️.graphql` + `🔺️diff/🔗️.graphql`** — Regenerate all facets from Rust anchor: `accessories: [AccessoryLink!]!`, `components: [CompositionLink!]!`; replace `GenericAttributes { _placeholder }` with `{ entries: [GenericAttribute!]! }` in root + diff GraphQL; add `id: String!` to `Product` in root schema GraphQL. No `string[]` for link structs.

4. **`💡️inferences/🦀️.rs` + `🧬️schema/🦀️.rs`** — Remove hidden `connection_type → "flange"` default (`🧬️schema/🦀️.rs:333`); emit Fail when 210 omits mandatory connection_type. Add `check_valve` / typed-sheet rules validating `connectionType` against Blatt code list (same `CONNECTION` choices as field-meta).

5. **`💡️inferences/🦀️.rs`** — Wire ignored Part 1 / catalogue leaves into real checks: `limits.*` (security envelope), `index.entries[].tags` / `.dn` / `.sheet` consistency with products, `geometry[].bbox` / `connections` / `parameters` when `geometryRef` set, `accessories[].required` / `.quantity`, `components[].quantity`, `manufacturerFile.headerVersion` / `buildingSystemNumber` / `created` (or remove from editable surface if derived-only — not acceptable to leave editable+ignored).

6. **`💡️inferences/🦀️.rs:137–148, 908–1030`** — Replace invented `sheet_numeric_bounds` with Blatt-sourced tables/code lists per sheet (filter classes sheet 19, type codes, PN/connection enums from Blatt definitions). Mandatory-key Fail must validate **value domains**, not just presence.

7. **`⚖️compliance-vdi3805-1/🦀️.rs` + `🐍️.py`** — Run oracle against each committed per-Blatt example; compare Rust report check ids + numerics within ±0.5 % for Φ, n, Q, η, Qn, COP, mandatory keys — not pass/fail flags on valve-only snapshots.

8. **`🧬️schema/🧬️mutations/🟦️.ts`** — Replace `ExtensionBag { fields: Record<string, unknown> }` with generated typed map matching Rust/JSON schema (CORRECTION 13:27 #8 parity).

---

## Non-blocking observations

- Round 3 fixed all eight Round-2 mechanical items at least partially; 255/255 is a credible count.
- Blatt 2–6 typed validation, sync remedies, sheet routing, field-meta walk, and path-resolve tests are production-quality for the demo valve path.
- `check_operative_sheet_coverage` is now called from `evaluate()` (`1359`) — correctly N/A, not dead code.
- Impl md “remaining gaps: none” remains inaccurate.
- Catalogue panel placeholder is UX-only.
- Mutation oracle Python still references legacy `create-`/`delete-`/`update-` names in comments (`🏭️mutate-vdi3805-1/🐍️.py:33–73`) — non-blocking test mapping only.
