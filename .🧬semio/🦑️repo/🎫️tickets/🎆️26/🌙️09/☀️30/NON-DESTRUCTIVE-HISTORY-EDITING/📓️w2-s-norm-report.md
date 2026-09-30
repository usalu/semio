# 📓️ W2-S-A norm — Schema/payload parity for `✏️s/🔌️plugins/📕️norm`

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-S-A (norm), 2026-09-30. Contract: `📋️design.md` §6, `🧭️plan.md`
"W2-S parity brief", coordinator addenda (annotations kept, `leafUncatalogued`, EN 1993 `section` label, negative witness,
norm document contract). Scope: every mutation aggregate, leaf, snapshot facet and fixture under `✏️s/🔌️plugins/📕️norm`.

## 1. Outcome

| Lint (`--under ✏️s/🔌️plugins/📕️norm`, cwd `🧪️test`) | before | after |
|---|---|---|
| `schema mutation-payloads` | 602 findings: 233 aggregate, 132 unmapped, 108 opaque, 107 invalid, 12 undescribed, 7 layout, 3 unresolved | **0 in every one of those classes**; 230/230 fixtures clean, the 1 negative witness is rejected. Only the newer class `unwitnessed` remains (356), which the coordinator reserved for the split wire-witness conversion. |
| `schema mutation-inputs` (collecting mode, incl. `leafUncatalogued`) | 370 at W2-R start; 64 `leafUncatalogued` + 1,257 nested after the first parity pass | **0 findings**, 928/928 inputs of 548 leaves, every nested record field labelled |
| `x-semio-ui` against manifest `$defs/InputUi` (Python `jsonschema`, leaf schemas + snapshot facets) | – | **1,855 annotations, 0 errors** |

Verdict: DONE for the brief's classes; `unwitnessed` deferred by instruction (not started).

## 2. Decisions

1. **Leaf root = `payload_value()`, camelCase everywhere.** Every norm leaf struct now carries `#[value(rename_all = "camelCase")]`
   and, where it derives the test serde twin, `#[cfg_attr(test, serde(rename_all = "camelCase"))]` — 400 Rust files. Fixtures, TS
   twins and doc comments moved in the same change (no compat).
2. **Records live once, in the snapshot facet.** Every record and enum a leaf reaches is a `$defs` entry of
   `🧬️schema/📸️snapshot/🔣️.json` (236 types over 15 artifacts), generated from the Rust type (value_derive rules: uints `minimum 0`,
   `Option` nullable and not required, `default` fields not required, unit enums as string enums with the Rust wire spelling,
   external/internal/adjacent data enums as `oneOf`). Leaves `$ref` them with the leaf's own `x-semio-ui` beside the `$ref`
   (the established pattern, 1,583 precedents). `definitions` became `$defs`.
3. **Aggregates follow the approved tagging rule:** internally tagged (en1990, din18599, en1997, en1998, en1999) = `oneOf` of leaf
   `$ref`s with `mutation: {const}` pinned in each leaf; externally tagged (the other 10 + results) = `{required: [Wire],
   additionalProperties: false, properties: {Wire: {$ref}}}`.
4. **Rust-side outliers fixed at the root, both wires aligned:** DIN V 18599 `MonthlyClimate` (payload-only record) → camelCase;
   VDI 3805 `VdiQuantityKind`, `ExtensionBag`, `EditionId`, `SchemaStatus`, `Domain`, `Diagnostic`, `Severity`,
   `EditionProfileChoice` → value camelCase (their serde twin and every fixture already were); ISO 16757's 48 catalogue records →
   serde camelCase (their value wire already was; the fixtures were serde-snake and are re-encoded); EN 1998 `En1998Site.enGroundType`
   / `enSpectrumType` → `#[value(default, skip_serializing_if = "String::is_empty")]` like their serde twin (empty under the DE annex).
5. **Annotations preserved and completed.** The generator merges the previous node's `x-semio-ui`, description and hard bounds into
   the Rust-derived structure (renamed properties carried over; option labels re-keyed onto the Rust spelling). Nested labels come
   from the W2-R tables (`names`, leaf inputs mapped onto record fields) and hand-written en/de tables with DIN/Eurocode
   terminology and SI stored units + engineering display units (`🧪️w2-s-norm-labels.py`). EN 1993 `section` inputs read
   "Cross-section"/"Querschnitt" explicitly (glossary `section` is now the neutral "Abschnitt").
6. **Orphans deleted, not re-homed** (read-only `git log --follow`): all 132 unmapped cases plus 10 cases whose kind name survived but
   whose payload and snapshots still encode the flat pre-652 model (en1990 ×2, din16798 ×1, en1996 ×6, en1998 ×1). Commit `…🚩️652`
   (09-26) replaced those flat leaves by index-/id-addressed leaves with other shapes and SI units, so no case was a rename.
   The 4 en1996 leaf test files that only `include_str!`ed those cases were never wired as modules and are deleted too.
7. **Negative witness:** din18599 `update-climate/refuses-a-negative-january-irradiance` is a range invariant →
   `MonthlyClimate.gHWM2.items.minimum = 0`.
8. **`$id`s on the catalogue scheme:** 30 EN 1990 + 22 EN 1991 + 12 EN 1996 leaves and 2 EN 1998 kind mismatches; artifact roots
   and facets of din16798, din4108, en1993, en1996, en1998 (`…/artifact.json`, `snapshot.json`, `diff.json`), en1999 diff, the
   en1995 text facet (its `…/en1995/mutations.json` collided with the aggregate); aggregate `$id`s `<scope>/mutations.json`;
   every touched document on draft-07.
9. **Snapshot facet `$defs` declare honest `x-semio-formats`** (JSON Schema plus exactly the twins that declare the type).

## 3. Verification (all foreground or gated; nothing claimed that was not run)

| Check | Result |
|---|---|
| `cargo check` 15 norm artifact crates (gated, shared target) | ok, 7 m 15 s; no warning in a file this WP touched |
| `cargo check -p semio-s-plugin-norm -p …-en1998 -p …-iso16757` (after the last Rust edit) | ok |
| `cargo test --lib` (private `target-nde-w2s-norm`, `CARGO_INCREMENTAL=0`) | din16798 85/85, din18599 114/114, din4108 98/98, en1991 80/80, en1992 99/99, en1993 160/160, en1994 79/79, en1995 178/178, en1996 146/146, en1997 98/98, en1998 76/76, en1999 79/79, iso16757 314/314, vdi3805 284/284, en1990 140/142 |
| W1-D payload round-trip law `semio_payload_law_*` | ok in all 15 crates |
| en1990 failures (2) | `default_snapshot_dsl_roundtrips` (3 ≠ 2) and `every_editable_leaf_changes_a_check…` (`variables[Q-snow].category` has no check influence): snapshot DSL and compliance evaluation, untouched by this WP (en1990 changed only leaf-struct attributes) |
| Norm document contract (`testNormDocumentContractOracle`) | **OK** — 6 native snapshots, 2 committed diffs; `tsc --noEmit --strict` over both artifacts' facet twins + the test: 0 errors |
| Generator idempotence (`report`, `fixtures`, `rust`, `ts`) | 0 documents / 0 fixtures / 0 Rust files / 0 TS files to write |
| Scoped `schema --under norm` contract diagnostics (parity class excluded) | 344 → 304: dialect 71 → 0, catalog-stale 69 → 31, export-incomplete 55 → 13, ref-unresolved 2 → 0, parser-missing 115 → 228 (see §5.2) |

## 4. Files

- **Generator + data (ticket root):** `🧪️w2-s-norm-rust.py` (Rust type reader), `🧪️w2-s-norm-parity.py` (`report|apply|dump`,
  `fixtures|fixtures-apply`, `rust|rust-apply`, `ts|ts-apply`), `🧪️w2-s-norm-labels.py` (ids, camelCase lists, fixture sources,
  stale-model cases, fills, bounds, labels, options), `🧪️w2-s-norm-inventory.py`.
- **Schemas:** 548 leaf schemas, 16 aggregate schemas, 15 snapshot facets, 5 artifact roots, 7 diff/text facets under norm.
- **Rust:** 400 files (leaf structs; `⚡️din18599/🦀️.rs` MonthlyClimate; `🏭️vdi3805/🦀️.rs`; `📇️iso16757/🦀️.rs`; `🫨️en1998/🦀️.rs`),
  13 iso16757 leaf-test doc comments.
- **Fixtures:** 188 re-encoded (iso16757 116 files incl. snapshots/diffs to the camelCase wire, din4108 62, en1993 12, en1996 6,
  din16798/en1990/din18599 1 each), 52 en1994 files normalised (unread `mKNm`/`vKN`/`nKN` dropped, required `fKN: 0.0` added),
  142 case directories deleted.
- **TS twins:** 19 files camelised (iso16757 snapshot + aggregate + 16 leaf `🦠️mutation/🟦️.ts`, vdi3805 aggregate), VDI
  `VdiQuantityKind` literals, din18599 `UseClass` literals, rewritten header docs; din18599 `🟦️.ts` (syntax error removed, exact
  `parseDin18599Fields`/`parseDin18599Artifact`), `📸️snapshot/🟦️.ts`, `🔺️diff/🟦️.ts` (current `Din18599Diff`).
- **Document contract:** `🧫️fixtures/🪪️document-contract/🔣️.json` (en1990 vector removed — it composes no child since 652; din18599
  vector on the current model, 6/2 committed), din18599 root + snapshot `climate` `x-semio-child-kind`, din18599 diff schema rebuilt.
- **Scratch:** `🗑️generated/w2s-norm/` (lint JSON, check/test logs, plans, helper probes).

## 5. Open items (outside this WP or deferred by instruction)

1. `unwitnessed` 356 leaves — the wire-witness conversion, to be split across executors (not started).
2. TS twin parser debt: 228 `schema-export-parser-missing`; 113 of them became visible because the snapshot records are now real
   `$defs` exports of catalogued scopes and the snapshot TS twins declare those types without `parse<Type>()`.
3. The schema catalog is stale repo-wide (`schema generate --check`); hashes only — every norm leaf and facet is already catalogued.
4. Python oracles and features of din16798, en1990, en1996, en1998 (`🧪️tests/mutate-*/🐍️.py`, `🥒️.feature`) still list the pre-652
   vocabularies and point at the deleted vectors; new vectors for the current kinds are owed (part of the witness conversion).
5. din4108 `🧫️fixtures` (28 cases, python-oracle vectors) snapshots predate `inclinationDeg`/`deltaUG`/`deltaUF`/`deltaUR`; the Rust
   mutate test uses `🎫️fixtures`, which decode.
6. en1995 `MemberRole`/`SupportType`: serde twin camelCase, value wire PascalCase (fixtures follow the value wire) — untouched.
7. Data-enum variant labels (ISO 16757 `CatalogueValue`/`GeometryNode`/`PartNumberRule`, VDI `SheetAttributes`) are not read by the
   reader and were not authored; ISO 16757 has one `DslValue` field described as any.
8. `semio-s-plugin-norm` tests were not run (its only change, `ChangeSelectedCheckIndex { index }`, is a wire no-op).
