# XLSX Canonical Authority Completion — 2026-10-03

## Scope

This implementation completes the XLSX migration from a regenerated semantic workbook authority to an OPC/XML authority. The persisted snapshot is now exactly:

- `schema`
- `opc`, containing ordered non-XML package parts, ordered content-type declarations, and ordered relationships
- `xml_parts`, containing every XML package part as an ordered path/content-type/document tuple

`XlsxWorkbook` remains a derived projection for editor/viewer presentation and handcrafted mutation inputs. It is not persisted and cannot replace the authoritative package representation.

## Authority and fidelity

The importer decodes the OPC package, parses every XML part into the XML document model, preserves every non-XML part in OPC order, and keeps custom paths and unknown XML content. Workbook, worksheet, shared-string, relationship, and content-type lookups are derived from the retained package instead of being reconstructed from a reduced workbook model.

The exporter validates the authoritative snapshot, serializes every retained XML document, reinserts those bytes into the OPC package, and encodes the package. Saving without a mutation therefore preserves XML field coverage, element order, relationship order, content-type order, custom part paths, and non-XML bytes subject only to the canonical XML/ZIP serializers.

The public Rust and TypeScript facets export `XlsxSnapshot`, `XlsxXmlPart`, `XlsxWorkbook`, `XlsxSheet`, `XlsxCell`, and `XlsxCellValue`, with the OPC package types available from TypeScript. The JSON, GraphQL, and protobuf snapshot/artifact/diff/mutation schemas now describe the same OPC plus XML-parts authority rather than the removed persisted workbook projection.

## Mutation and diff behavior

Cell, sheet, and shared-string mutations locate and edit the authoritative XML nodes in place. Namespace-aware traversal is used for worksheet cells and shared strings. Shared-string updates preserve existing aggregate attributes for value-only edits. Removing an unreferenced shared string updates an existing `uniqueCount` and shifts all later worksheet shared-string indexes without rewriting unrelated cells or XML fields.

The minimal workbook builder computes the shared-string reference `count` from cells and formula cached values and the `uniqueCount` from the shared-string table. Numeric cells accept both absent `t` and `t="n"`; other valid unknown/date cell kinds project conservatively as inline text rather than making package import fail.

`XlsxDiff` compares the authoritative OPC package and ordered XML-parts collection. Removed persisted-workbook diff types were deleted; handcrafted mutation codecs keep the derived workbook value types only where mutation payloads require them.

## Editor and viewer migration

Base, ECMA-376 strict, and ECMA-376 transitional editor/viewer paths now call `project_workbook()` from the canonical snapshot. The editor emits canonical XML mutations and the resulting save reads from the same XML authority.

The editor/viewer now presents each projected worksheet as a sparse, independently windowed row/column grid, including editable blank coordinates and blank-sheet creation. Revision-guarded vacancy addressing and canonical `InsertCell` behavior are recorded in `🧬xlsx-worksheet-grid-and-vacancy-editing-2026-10-03.md`. Range selection, clipboard, row/column structural commands, and formatting controls remain later interaction cuts.

## Neutral fixtures and independent laws

Five hand-authored neutral packages under `🧫️fixtures/🧬️canonical-xml-save` exercise custom XML, relationship/content-type ordering, extension markup, styles/theme/comments, macros and binary media, and formula/shared-string variants.

The native laws use three independent third-party oracles:

- `zip` inspects package entries and exact non-target bytes.
- `quick-xml` compares XML event streams independently of the repository XML serializer.
- `calamine` reopens supported workbooks and validates projected cell values.

The editor law edits `Data!A1` to `99.25` for every neutral fixture, asserts exact unaffected XML events and non-XML bytes, checks the independent Calamine value where supported, and reopens the output through the repository importer. Separate no-op laws cover canonical save stability, and a focused shared-string law proves reference reindexing after deletion.

## Validation

- `bun nx check @semio-tech/stdio-xlsx --skip-nx-cache`: passed.
- `bun nx test @semio-tech/stdio-xlsx --skip-nx-cache`: passed, 9 tests and 83 assertions. Runtime evidence included the viewer neutral projection (`rows=3`, `cols=3`, locales `en,de`) and unchanged draft fidelity fixture (`cases=10`, shared-string conflict detected).
- `git diff --check -- '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx'`: passed.
- `bun nx run @semio-tech/stdio-xlsx-rs:test --excludeTaskDependencies --skip-nx-cache --args='--features component-app-assembly --no-fail-fast --lib -- --nocapture'` with `CARGO_BUILD_JOBS=1` and the ticket-private `CARGO_TARGET_DIR`: inconclusive infrastructure timeout before test execution.

## Native result

The registered native command was started once through the project Nx target with one Cargo job. The repository owner wrapper redirected the requested ticket-private target directory to the shared Cargo cache. The command remained in `cargo test --no-run` for the full owner-command budget and exited after 16 minutes 3 seconds with status 101. Its output contained continuous owner/build heartbeats and no Rust compiler diagnostic, XLSX diagnostic, or executed test result. No second Cargo process was started.
