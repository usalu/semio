# XLSX Editor Feature Coverage Matrix (2026-11-27)

Sources reconciled: editor manifest `✏️s/…/📕️xlsx/…/🧱️base/✏️editor/🦀️.rs`, edit rules `🧬️mutations/🧭️edit-rules/🦀️.rs`, calculation `🧬️schema/🧮️calculation/🦀️.rs`, hub catalog actions (`🌎️hub/🧩️compositions/🗄️stdio/…/🔣️.json`), artifact definition `📕️xlsx/📜️artifact-definition.json`, acceptance row `🧪️tests/🎯️artifact-acceptance/🧫️fixtures/🔣️.json`, goal `RUNNING-XLSX`, earlier reports in this ticket.

Scope decision: the product surface is (a) the A1 grid with sheet tree and typed cell entry, (b) the Details pane that edits the workbook's XML parts (cell nodes incl. style index, shared strings, sheet insert/rename/remove, relationships, content types), (c) the generic shell actions Open File, Save File, Export Document, Undo/Redo, reopen from the local catalog. Out of scope (not exposed anywhere in the product): style/number-format editors, row/column insert/delete, sort, freeze, charts, comments, clipboard (shell declares `mod+c/v` but no xlsx handler exists), recalculation-on-open (caches are preserved on open; recomputed on every edit), functions outside the 19 implemented ones (`#NAME?`).

Status legend: PASS (end-to-end evidence through the browser app plus third-party oracle where relevant), FAIL, OPEN (not yet run).

| # | Feature | Expected behavior | Entry point | Status | Evidence |
|---|---|---|---|---|---|
| F1 | Open natural .xlsx (Transitional) | Picker open spawns the transitional editor; grid shows values | Actions rail Open File | OPEN | |
| F2 | Open Strict workbook | Strict editor shows values | Open File | OPEN | |
| F3 | Refuse malformed / foreign file | Localized refusal notice, document unchanged | Open File | OPEN | |
| F4 | Render values and types | text, number, boolean, formula text, inline/shared strings | grid | OPEN | |
| F5 | Sheet tree / switch sheets | one item per sheet; selecting shows its grid | sheet tree | OPEN | |
| F6 | Edit cell: number | typed number stored as `n` | grid input | OPEN | |
| F7 | Edit cell: boolean | `true`/`TRUE` stored as `b` | grid input | OPEN | |
| F8 | Edit cell: text, leading-zero `'007` | stored as string `007` | grid input | OPEN | |
| F9 | Edit cell: formula | `=expr` stored as `<f>`, cache computed | grid input | OPEN | |
| F10 | Insert into vacant cell / clear cell | vacancy insert; clearing removes unstyled cell | grid input | OPEN | |
| F11 | Recalculation of dependents | dependent caches updated in same command | grid edit | OPEN | |
| F12 | Cross-sheet formulas | `=SUM(Data!B2:B3)` evaluates | grid edit | OPEN | |
| F13 | Formula error values | `#DIV/0!`, `#NAME?`, `#REF!` cached | grid edit | OPEN | |
| F14 | Supported function set vs third-party oracle | 19 functions + operators match `formulas` lib | native fixture + browser | OPEN | |
| F15 | German locale entry/display | `1,5` → 1.5; displayed `1,5` | grid (de) | OPEN | |
| F16 | Undo / Redo | revert and reapply edit + caches | rail / `mod+z` | OPEN | |
| F17 | Cell style change (details) | `s` attribute edit → bold/number format | Details pane | OPEN | |
| F18 | Sheet rename (details) | tree + formulas follow rename | Details pane | OPEN | |
| F19 | Sheet insert / remove (details) | new empty sheet; removal | rail insert/removeSnapshotValue | OPEN | |
| F20 | Shared strings edit (details) | `<si>` edit changes referencing cells | Details pane | OPEN | |
| F21 | Save File (download .xlsx) | original name; valid package; values/types/formulas/styles/merges/widths preserved | Save File | OPEN | |
| F22 | Reopen from local catalog | edits persist | catalog button | OPEN | |
| F23 | Reload + reopen | edits persist across reload | page reload | OPEN | |
| F24 | Export Document archive | archive download, no failure, also after window close | Export Document | OPEN | |
| F25 | Shared formulas in imported files | dependents display/evaluate translated formula | open | OPEN | |
| F26 | Preservation of untouched formatting | fills, fonts, numFmts, merges, widths survive save | Save File | OPEN | |
| F27 | Third-party independent open of saved file | openpyxl + `formulas` recalculation agree | oracle | OPEN | |

## Defects found and fixed
(filled as work lands)

## Reproducible checks
(filled as work lands)
