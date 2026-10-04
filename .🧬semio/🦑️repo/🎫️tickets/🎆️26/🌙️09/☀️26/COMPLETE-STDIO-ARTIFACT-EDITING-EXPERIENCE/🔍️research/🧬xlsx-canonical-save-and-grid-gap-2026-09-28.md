# XLSX Canonical Save and Grid Gap

## Source Findings

The current XLSX exporter regenerates `xl/workbook.xml`, every `xl/worksheets/sheetN.xml`, and `xl/sharedStrings.xml` from the reduced semantic workbook on every save. `regenerate_workbook_parts` removes the original default-owned parts, writes generated XML, and only adds a root office-document relationship if one is absent. The importer resolves the root relationship, so an imported workbook with a custom main-part path can retain its original root relationship while export writes the changed semantic data into a different default main part. This source path needs an independent save/reopen regression; it is not covered by the unchanged-cell-draft repair.

`workbook_sheets_from_xml`, `shared_strings_from_xml`, and the worksheet helpers use literal element/prefix names. The semantic projection drops rich-string formatting, row/cell styles and unknown attributes, merged regions, validation, workbook settings, and other XML fields. Some parts such as styles remain as bytes, but their references on regenerated cells are lost. These are source findings, not a newly executed native result.

The current primary editor is a flat table with columns Sheet, Row, Column, and Value. Only existing cells can be edited: `xlsx_set_cell_emit` rejects missing cells. The view has no sheet tabs or real worksheet grid, and no direct blank-cell creation. Two-axis windowing makes the existing view bounded, but does not complete a spreadsheet editing experience.

## Required Implementation Cut

Use one canonical XML owner for each workbook/worksheet/shared-string part, with the semantic workbook as a derived projection. Preserve relationship-resolved paths, element/attribute order, namespaces, extension nodes, formatting, formula metadata, and all untouched XML. Reuse the DOCX canonical-XML pattern after its addressed mutation and retained preparation proof; do not add a regenerated semantic tree alongside independently editable XML authority. OPC metadata and archive retention remain part of the shared package work.

Author neutral custom-main-path and rich-cell fixtures first. A single value edit must retain untouched XML markers and custom paths in the saved ZIP; an independent ZIP/XML reader and Calamine must verify the resulting archive and displayed values. Include strict/transitional namespace aliases, rich shared strings, formulas with cached values, styles, merged cells, validations, and workbook settings. Native laws must exercise registered publication, inverse, undo/redo, save, and reopen.

The primary spreadsheet surface then needs localized sheet selection, A1 headers, a bounded two-dimensional grid including empty cells, formula/value editing, cell/range selection, clipboard operations, row/column/sheet structural actions, and visible formatting controls. Selection and viewport are ephemeral local state; artifact edits publish through addressed mutations with conflict checks. Large-range operations need progress and cancellation. All deeper XML/package fields stay reachable through validated Details alongside these natural controls.

## Validation Boundary

XLSX native current3 is still active and has not reported assertions. Its authored tests cover the preceding windowed viewer and unchanged draft/shared-string conflict repairs, not the canonical save/grid implementation described here. No format-complete or end-user-complete claim is justified.

## Neutral Fidelity Fixtures and Execution Assignment

Five schema-first fixtures now exist under the XLSX base subset `🧫️fixtures/🧬️canonical-xml-save`: transitional/default, transitional/aliased, strict/default, strict/aliased, and a custom main filename. Each preserves custom workbook and worksheet paths, nondefault sheet identity, workbook settings/defined names, rich shared strings, row/cell styling, formula source and cached value/attributes, error and Boolean cells, merge ranges, validation, hyperlinks, custom extension nodes, and unrelated binary bytes. A single A1 value edit has an independently authored expected worksheet XML.

Calamine 0.36.1 source inspection shows it follows the root relationship directory but assumes `workbook.xml`, `sharedStrings.xml`, and `styles.xml` filenames within it. Accordingly it independently checks the four custom-directory namespace fixtures. The arbitrary `book.xml` case is checked with ZIP and Quick-XML without treating Calamine’s filename limitation as a product limitation. Native oracle execution remains pending; strict Ajv fixture validation passed in Office TypeScript current18.

The Office execution agent owns the single-authority XLSX schema/import/export/diff/projection/mutation migration. Root owns the independent fidelity laws and the shared Office schema harness. No dual persisted workbook/XML model or save-time regeneration fallback is accepted. Metadata declarations in the fixture are authored as conventional OPC declarations; exhaustive custom metadata XML preservation remains the separately recorded shared OPC authority work.

## 2026-10-03 Canonical Grid Checkpoint

The XLSX snapshot authority is now the schema identity, complete OPC package, and ordered canonical XML parts. The workbook remains a derived projection. Import retains resolved part paths and every parsed XML node; export writes the canonical OPC/XML authority and no longer regenerates workbook, worksheet, or shared-string XML from a second semantic owner. Diff, public facets, native and SQLite paths, mutation preparation, and the base/Strict/Transitional editor and viewer now consume that authority.

The natural worksheet surface is a sparse, windowed grid. It derives sheet names and occupied cells from canonical XML, renders bounded visible row and column windows without allocating a dense rectangle through the greatest address, and gives editors a revision-bound vacant-cell address. `insertCell` creates a missing row or cell in canonical `sheetData`; existing-cell mutations retain exact canonical addresses and inverses. The neutral sparse-grid and canonical-save fixtures cover blank sheets, missing rows, formulas, shared strings, rich/unknown XML, Strict and Transitional namespaces, aliases, and custom relationship paths. The independent ZIP, Quick-XML, and Calamine laws are mounted in the native suite.

Fresh native run `xlsx-grid-native-5.log` compiled and executed 145 tests: 139 passed, six failed, and one pre-existing skip was reported. Three failures were localized schema input labels for `insertCell`; two were empty Strict/Transitional history fixture catalogs; one was an exact allocator ledger mismatch during reconstruction. The schema now carries English and German worksheet, row, column, part, path, namespace, and revision labels. Strict and Transitional each have a hand-authored applied `setSnapshot` history case with their own namespace and relationship dialect. All nine added or changed neutral JSON documents parse successfully.

The allocator difference was 546 bytes. Source isolation found 544 bytes in the former standard-library relationship `BTreeMap` leaf and two bytes in `prologPosition = "99"` validation. XML position parsing now validates canonical decimal spelling directly from borrowed bytes and no longer allocates a temporary decimal string. The shared OPC authority has since moved relationship owners to `OpcRelationshipOwners`, a first-party sorted contiguous group owner adopted through the active decode allocation stage; reconstruction no longer guesses or charges standard-library B-tree layout. No XLSX native pass is claimed after native5; the exact-ledger and canonical editing laws still require a fresh run against the coherent explicit-owner source.

The non-Cargo package validations are green after these XLSX changes:

- `bun nx check @semio-tech/stdio-xlsx --skip-nx-cache`
- `bun nx test @semio-tech/stdio-xlsx --skip-nx-cache`: 11 tests passed, 97 expectations, zero failures; Ajv independently accepted the viewer and unchanged-draft fixtures.

The explicit relationship-owner source scan is also coherent: XLSX diff, canonical edits, import/export, subset checks, native/SQLite fixtures, and tests use the named `OpcRelationshipOwners` operations directly. A fresh uncached XLSX `check` passed after the shared cut; the Semio bridge check passed all 22 suites in 4m18s.

The next native XLSX run must start only after native codec, retained conversion, SQLite projection/reconstruction, and tests all compile against the explicit OPC relationship owner. It must re-execute the six native5 failures, the canonical ZIP/Quick-XML/Calamine save laws, editor save/undo, and the full 145-test catalog. The result remains open until that run is green.
