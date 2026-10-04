# XLSX Worksheet Grid and Vacancy Editing — 2026-10-03

## Scope

This cut completes the next XLSX end-user editing surface on top of the canonical OPC plus ordered XML-parts snapshot. Base, ECMA-376 Strict, and ECMA-376 Transitional editors and viewers now render real per-worksheet sparse grids. A blank coordinate can publish a revision-guarded `InsertCell` mutation without rebuilding a worksheet or replacing the semantic workbook projection.

The canonical ownership rule remains unchanged: `XlsxSnapshot { schema, opc, xml_parts }` is the sole persisted authority. `XlsxWorkbook`, sheets, cells, and grid extents are projections used by editor/viewer presentation and mutation preparation.

## Address and mutation model

`XlsxWorksheetAddress` identifies one canonical `sheetData` element by part path, XML node path, expanded namespace/local name, and revision. `XlsxCellVacancyAddress` adds a one-based row and zero-based column. Address validation enforces SpreadsheetML row and column bounds, validates canonical part and node paths, rejects occupied coordinates, and refuses stale worksheet revisions.

`InsertCell` is an atomic public mutation with Rust, JSON Schema, TypeScript, protobuf, GraphQL, text-grammar, binary-protocol, leaf-manifest, and aggregate-registry facets. It uses binary tag 11 and participates in canonical diff detection. The mutation catalog also declares its production dispatch and independent spreadsheet-reader requirement.

Application edits only the addressed worksheet XML:

- Existing rows receive the new `c` element at spreadsheet column order.
- Missing rows receive a new `row` at spreadsheet row order.
- Foreign elements and unknown siblings keep their relative positions.
- The existing namespace prefix is reused for new `row`, `c`, `f`, `v`, `is`, and `t` nodes.
- Formulas, cached values, styles, extensions, custom worksheet paths, and all unrelated OPC/XML fields are untouched.
- The exact inverse is a `SetSnapshot` carrying the pre-mutation canonical snapshot, so undo restores every authoritative field rather than synthesizing a semantic inverse.

Editor action preparation resolves an occupied coordinate to `SetCell` and a vacant coordinate to `InsertCell`. Existing cells bind to their cell revision; vacancies bind to the containing `sheetData` revision. Empty drafts at empty coordinates remain no-ops. Any structural edit stales previously captured vacancy addresses.

## Sparse grid UI

Each workbook renders as a tree section with one expandable item per worksheet. Each item contains a `TableWindowKit` indexed matrix with independently windowed rows and columns. Cell lookup uses a sparse `BTreeMap<(row, column), cell>`; the renderer never allocates a dense matrix up to the greatest used coordinate.

The visible logical extent is the used maximum plus one blank edit edge. A blank sheet exposes one row and one column, while SpreadsheetML maximum row and column bounds are capped without overflow. Column labels use A1 notation and row labels are one-based. Axis labels are localized in English and German. Viewer cells use the same sparse geometry and are read-only; editor cells carry stable controller action arguments.

Base, Strict, and Transitional share the grid renderer while retaining their own controller identities. Their TypeScript view facets now expose worksheet-grid metadata rather than a flat `sheet,row,column,value` record list.

## Neutral fixtures and independent laws

The language-neutral editor fixture `✏️editor/🧫️fixtures/📊️sheet-grid/🔣️.json` and its schema cover:

- sparse `A1` and `C3` cells with an editable `B2` vacancy;
- a completely blank worksheet;
- a formula cell with its editable formula draft and adjacent vacancy;
- EN/DE row and column labels.

The neutral viewer fixture `🧫️fixtures/🪟️viewer-cell-window/🔣️.json` describes a hosted sparse worksheet window, expected logical keys, values, vacancy edge, and localized axes.

Native laws cover blank-cell command emission, canonical XML application, save, Calamine reopen, and exact undo. A sparse-row law inserts between existing cells, proves formula/cached-value preservation, and proves old vacancy revisions become stale. Editor and viewer laws cover sparse/blank/formula grids, hosted row and column windows, read-only viewer cells, editable vacant cells, base/Strict/Transitional controller identities, and localization.

## Typed snapshot boundaries

XLSX native and SQLite snapshot boundaries now return `ValueError` directly. Native encoding has a measured preflight path and one controlled allocation stage. SQLite projection and reconstruction use the retained OPC and XML ownership helpers, cumulative work/value-byte checks, controlled reconstruction, and typed backing refusal. DSL errors convert with `TextError::from_value_error`; pack boundaries preserve `PackError::ValueRefusal`.

No existing `OpcPackage` API or ordering behavior changed. The separately added retained OPC family is additive and does not alter XLSX canonical snapshot authority.

## Validation

- JSON syntax for both grid fixtures, their schemas, the vacancy/insert-cell schemas, leaf manifest, and XLSX oracle catalog: passed with `jq`.
- `bun nx check @semio-tech/stdio-xlsx --skip-nx-cache`: passed after the editor and viewer migration.
- `bun nx test @semio-tech/stdio-xlsx --skip-nx-cache`: passed, 10 tests, 91 assertions, and both Ajv fixture checks. Runtime output reported the viewer fixture as two rows by two columns with locales `en,de`.
- `git diff --check -- '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx'`: passed.
- Native XLSX compilation and runtime laws remain queued. Multiple repository Cargo owners, including DOCX/PPTX and unrelated Flow/BREP work, held the shared queue; this agent did not start an overlapping Cargo process and makes no native-pass claim.

