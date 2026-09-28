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
