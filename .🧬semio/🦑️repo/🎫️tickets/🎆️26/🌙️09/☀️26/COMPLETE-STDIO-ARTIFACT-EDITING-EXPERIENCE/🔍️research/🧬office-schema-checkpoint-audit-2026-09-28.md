# Office Schema Checkpoint Audit — 2026-09-28

## Scope

This is a read-only, static comparison of the DOCX, XLSX, and PPTX stdio artifact schema checkpoint. It covers the base snapshot and sparse-diff Rust, TypeScript, and JSON Schema contracts, then checks how strict and transitional roots reuse those contracts. Concurrent test-feature and OPC-validator changes were excluded because they do not alter the wire shapes reviewed here.

## Findings

### DOCX `null` cannot preserve the documented sparse-diff clear state

`DocxParagraphDiff.style` and `DocxStyleDiff.based_on` use nested options to model unchanged, set, and clear in Rust, and their public TypeScript/JSON Schema contracts advertise `null` as the clear value. The standard deserializer collapses an input `null` into the outer `None`; it therefore becomes indistinguishable from an omitted field rather than `Some(None)`.

The issue is explicitly described in the existing mutation proof: [`🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️mutations/📸️set-snapshot/🧪️tests/🅱️bolds-the-tower-run-of-the-opening-paragraph/🦀️.rs) lines 10 and 153 say that `Some(None)` cannot survive a JSON round trip. The schema contract remains contradictory:

- [`🔺️diff/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs) lines 115–120 define `style: Option<Option<String>>`, and lines 159–164 define `based_on: Option<Option<String>>`; neither has a custom deserializer.
- [`🔺️diff/🟦️.ts`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts) lines 14 and 20 expose `style?: string | null` and `basedOn?: string | null`.
- [`🔺️diff/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json) permits `null` for those properties.

PPTX already contains the required pattern. [`📽️pptx … 🔺️diff/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs) lines 160–181 apply `deserialize_double_option` to `Option<Option<u32>>`, preserving a present `null` for `font_size`.

Add the equivalent double-option deserializer for the two DOCX string fields, then add a JSON input proof that applies both `style: null` and `basedOn: null` and observes the corresponding clears. Until then, consumers may send JSON that validates against the published schema but silently produces no edit.

### DOCX and XLSX public artifact facets describe obsolete `entries` payloads

The base **public artifact** JSON Schema and TypeScript facets for DOCX and XLSX still expose `{ schema, entries: [{ name, data }] }`. Their actual Rust artifact roots contain the package and domain document/workbook instead. This is distinct from the base snapshot and diff leaves, which were checked separately below.

- DOCX: [`🧬️schema/🟦️.ts`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts) lines 1–9 and [`🧬️schema/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔣️.json) lines 6–25 define `entries`; [`🧬️schema/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs) lines 13–21 defines `schema`, `opc: OpcPackage`, and `document: DocxDocument`.
- XLSX: [`🧬️schema/🟦️.ts`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts) lines 1–9 and [`🧬️schema/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔣️.json) lines 6–25 define `entries`; [`🧬️schema/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs) lines 13–21 defines `schema`, `opc: OpcPackage`, and `workbook: XlsxWorkbook`.

The strict and transitional public roots inherit this defect rather than defining an independent shape: DOCX [`📏️strict/🧬️schema/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🔣️.json) lines 7–10 and [`🔄️transitional/🧬️schema/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🧬️schema/🔣️.json) lines 7–10 use `allOf` with the base facet. XLSX does the same in [`🔒️strict/🧬️schema/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🔣️.json) lines 7–10 and [`🌉️transitional/🧬️schema/🔣️.json`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🔣️.json) lines 7–10.

Regenerate these two base public facets from the current Rust artifact roots, then validate one artifact fixture through each base, strict, and transitional JSON Schema and TypeScript surface. The strict/transitional `$ref`/`allOf` reuse itself is correct and should remain.

## Verified Compatible Contracts

- **Snapshot optionality:** every base snapshot root requires only `schema`. Rust applies `#[value(default)]` to the optional input containers (`opc` and document/workbook/presentation portions), and the JSON Schema accepts their omission. The TypeScript snapshot interfaces represent the materialized output with those properties present. No input/output mismatch was established from the available code.
- **Sparse diff structure:** all three base diff roots omit `required` fields; their TypeScript properties are optional and the Rust fields are `Option` with absent values skipped during serialization. DOCX indexed/named triples, XLSX named triples, and PPTX indexed/named triples all use optional `removed`, `modified`, and `added` members. The one DOCX nested-null issue above is the exception.
- **Archive comments:** DOCX, XLSX, and PPTX snapshot OPC contracts use a required string comment; each diff uses `Option<String>` / `comment?: string`, with no `null`. Omission therefore means unchanged and an empty string can be assigned deliberately. This matches [`📜️docx … 🔺️diff/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs) lines 214–216, [`📕️xlsx … 🔺️diff/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs) lines 132–134, and [`📽️pptx … 🔺️diff/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs) lines 229–231.
- **Tagged enums:** DOCX blocks use `kind`; XLSX cell values use `kind`; PPTX deliberately uses `shapeKind`. In [`📽️pptx … 📸️snapshot/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs) lines 62–91, `shapeKind` identifies the `Placeholder` variant while its independent `kind: String` payload remains available. The TypeScript and JSON Schema variants require the same discriminator/payload arrangement. PPTX diffs retain the same shape discriminator and make changed placeholder payload fields optional, matching Rust `Option` fields.
- **`$defs` and subset reuse:** each base snapshot/diff JSON Schema contains self-contained `$defs`; diff schemas reuse their snapshot definitions instead of restating member objects. Strict and transitional Rust schema modules re-export their base types, while their JSON Schema roots compose the base artifact facet via `$ref`/`allOf`. No duplicated strict/transitional leaf schema drift was found. The DOCX/XLSX public-root issue is propagated reuse of an obsolete base facet, not a `$defs` composition error.

## Validation Performed

Static source and JSON Schema inspection covered all base/strict/transitional DOCX, XLSX, and PPTX schema paths named above. A bounded `bun nx run-many --target=check --projects=@semio-tech/stdio-docx,@semio-tech/stdio-xlsx,@semio-tech/stdio-pptx --parallel=3` was started but did not complete after more than four minutes and was interrupted; it provides no pass/fail result. No native build was run.
