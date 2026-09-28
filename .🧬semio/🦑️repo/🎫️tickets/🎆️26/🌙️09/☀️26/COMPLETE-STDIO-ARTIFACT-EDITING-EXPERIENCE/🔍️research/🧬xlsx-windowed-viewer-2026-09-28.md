# XLSX Windowed Viewer

The three XLSX viewer variants previously cloned every worksheet cell and rendered a complete table before the host could request a window. All variants now delegate to one shared read-only projection using independent logical row and column windows. The caller supplies active locale and host window requests. Only requested cells are formatted and materialized; EN/DE column and axis labels follow the active locale. The old eager viewer helper and duplicated strict/transitional value renderers were removed.

A neutral fixture addresses a window crossing an empty worksheet and a populated worksheet, with a nonzero column offset and Unicode values. Native laws compare projected keys, labels and values against independently decoded fixture arrays and verify no edit bindings are present. All three variant smoke laws now inspect the real Table/TableRow contract. These are authored assertions, not passing native results. Formatting and scoped diff checking passed.

This change bounds UI row/cell materialization. Computing total cells and locating a requested ordinal still scans worksheet headers, and a requested unusually large cell still needs the existing retained text-surface path. It is not proof of a complete bounded arbitrary-size workbook pipeline.

## Independent Cell Projection Oracle

The neutral fixture now has a strict adjacent JSON Schema. Review caught a test-only invalid zero-based worksheet row; authored native cells and expected window rows now use SpreadsheetML’s one-based row convention. The native law encodes a real minimal workbook and reads it through dev-only Calamine 0.36.1, then compares the reference reader’s exact visible coordinates/text to both the fixture and the mounted viewer projection in English and German. Empty sheets and Unicode remain represented. Range origins are included when converting Calamine-relative row/column coordinates.

The existing XLSX TypeScript test router now validates the fixture through Ajv and checks its row/column dimensions. Its current1 run is active; the Calamine native law has not run yet. No new executable target was added.

XLSX TypeScript package test current1 completed successfully in 13.0 seconds through the existing Bun/Nx target. Package build/export/type consumer checks and the strict Ajv viewer fixture contract passed. The new native Calamine/window projection assertion remains unrun.
