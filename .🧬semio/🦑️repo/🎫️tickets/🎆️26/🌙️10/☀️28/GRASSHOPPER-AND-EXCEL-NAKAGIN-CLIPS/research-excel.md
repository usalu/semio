# 📊 Excel research (Microsoft 365 Excel, bilingual EN/DE)

Scope: vocabulary, references, functions, errors and dynamic arrays for the teaching clips. Quiz sources read in full: `🎓️teaching/🏛️architecture/📊️excel/❓️quiz/🔣️.json` (tasks: workbook-terminology 40 items, spreadsheet-commands 50, copying-references 20).

Verification status: `[V]` = confirmed against a fetched or searched Microsoft source; `[K]` = working knowledge, not re-checked here; `[CHECK]` = confirm in the running app (German UI) before filming.

---

## 1. Anatomy

| Term (EN) | Term (DE) | Definition | Quiz usage | Status |
|---|---|---|---|---|
| Workbook | Arbeitsmappe | Complete file (`.xlsx`) with one or more sheets. | Quiz: "Complete file with one or more sheets." | `[V]` quiz |
| Worksheet | Arbeitsblatt (Tabellenblatt) | One named grid inside a workbook. | Quiz: "One named grid inside a workbook." Tab = "Register". | `[V]` quiz; `[K]` DE UI uses "Tabellenblatt" in menus |
| Sheet tab | Blattregister (Register) | The tab at the bottom that selects a sheet. | Quiz item "Costs tab" → DE "Register Kosten". | `[V]` quiz |
| Cell | Zelle | One intersection of row and column. | Quiz: "One intersection of row and column." | `[V]` quiz |
| Row | Zeile | Numbered horizontal sequence of cells (1, 2, 3 ...). | Quiz: "Numbered horizontal sequence of cells." | `[V]` quiz |
| Column | Spalte | Lettered vertical sequence (A, B, C ...). | Quiz: "Lettered vertical sequence of cells." | `[V]` quiz |
| Cell address | Zellbezug / Zelladresse | Position of a cell, e.g. `C4` (column letter + row number), often called "A1 notation". | Quiz "Selected C4". | `[V]` |
| Range | Bereich | Several cells as a group, e.g. `B2:D4`. | Quiz "Selection B2:D4". | `[V]` quiz |
| Name Box | Namenfeld | Field left of the formula bar; shows the active cell or range address and can jump to names. | `[K]` | `[K]` |
| Formula bar | Bearbeitungsleiste | Line above the grid that shows the cell's content (formula or value). | `[K]` | `[K]` |
| Value | Wert | What the cell stores (number, text, logical, error). | Quiz "Values paste": constant 24. | `[V]` quiz |
| Formula | Formel | Expression starting with `=`. Quiz: "Complete calculation expression starting with =." | `[V]` quiz | |
| Displayed value | Anzeige (angezeigter Wert) | What the cell shows after number format; stored value may differ. | Quiz "Percentage format": displays 25%, stored 0.25. | `[V]` quiz |
| Constant | Konstante | A typed value, not a formula. | Pasted values become constants. | `[K]` |
| Function | Funktion | Named built-in calculation, e.g. `SUM(B2:B4)`. | Quiz notes English names are notation. | `[V]` quiz |
| Operator | Operator | `+ - * / ^ &` (text join), comparisons `= < >`. | `[K]` | `[K]` |
| Reference | Bezug | Address of source cells used inside a formula. Quiz: "Address pointing to source cells in a formula." | `[V]` quiz | |
| Defined name | Name (definierter Name) | Human-readable name for a cell or range (e.g. `Flaeche`). Managed in Name Manager (Namens-Manager). | `[K]` | `[K]` |
| Structured table | Tabelle (Excel-Tabelle) | Formatted range with headers, created with Ctrl+T; supports structured references. | `[K]` | `[K]` |
| Structured reference | Strukturierter Verweis | Refers to table columns by name, e.g. `Tabelle1[Flaeche]`. | `[K]` | `[K]` |
| Dynamic array / spill | Dynamisches Arbeitsblatt / Überlauf | One formula fills several neighbouring cells; the output range is the spill range. | `[V]` Microsoft (see §6) | |
| Spill range operator `#` | Überlaufbereichsoperator `#` | `A1#` refers to the whole spill range of A1. | `[K]` (Microsoft spill page did not describe it in the fetch) | `[CHECK]` |
| Implicit intersection `@` | Impliziter Schnittpunkt `@` | Forces a single value from a range or array in a formula written for older Excel. | Mentioned on the Microsoft spill page as a related topic. | `[CHECK]` |

### 1.1 Column and row in mockups
- The quiz asks students to read headings: "Row heading 7", "Column heading C". A cell is named by its column letter first, then row number: C4 = column C, row 4.

---

## 2. Cell addresses, ranges and copying references

### 2.1 Reference types `[V]` quiz `copying-references`
- **Relative** (`=A1`): column and row both shift when copied.
- **Absolute** (`=$A$1`): column and row are fixed.
- **Mixed, column fixed** (`=$A1`): column fixed, row shifts.
- **Mixed, row fixed** (`=A$1`): row fixed, column shifts.
- Dollar signs only fix references when copying or filling. They do not stop structural edits such as moving or inserting cells (quiz prompt).
- Shortcut: F4 cycles through the four reference types on the selected reference `[K]` (Microsoft "switch references" help page returned 404 in this session, so `[CHECK]`).

### 2.2 Worked offset from the quiz `[V]`
Copy the formula from C2 to D4: one column right, two rows down (offset +1 column, +2 rows).

| Original | Result in D4 | Reason |
|---|---|---|
| `=A1` (relative) | `=B3` | +1 column, +2 rows |
| `=$A$1` (absolute) | `=$A$1` | fixed |
| `=$A1` (column fixed) | `=$A3` | row +2 |
| `=A$1` (row fixed) | `=B$1` | column +1 |

Teaching rule: calculate the offset (Δcolumn, Δrow) first, then apply it only to the relative parts.

### 2.3 Copy versus move versus values
- Copy-paste of a formula keeps the formula; references adjust as in §2.1.
- Paste Values (Werte einfügen) replaces the formula by its constant result. Later input changes do not recalculate the cell `[V]` quiz `command-values`.
- Percent format changes display only: 0.25 shown as 25%. The stored value stays 0.25 `[V]` quiz `command-format`.

---

## 3. References across sheets and workbooks

### 3.1 Sheet references `[K]`
- Syntax: `'Type A'!B3` (quotes needed when the sheet name has spaces or special characters). Unquoted: `Kosten!B3`.
- `[CHECK]` German UI shows the same syntax; the separator in formulas is `;` (see §5).

### 3.2 3D references (across several sheets) `[V]` Microsoft
Source: https://support.microsoft.com/en-us/excel/create-a-3-d-reference-to-the-same-cell-range-on-multiple-worksheets
- A 3D reference points to the same cell or range on multiple sheets.
- Syntax: `=SUM(Sheet1:Sheet3!B3)`; with spaces: `=SUM('Type A:Type C'!B3)`.
- Moving a sheet within the span changes the span; deleting an endpoint sheet removes its values.
- Not every function accepts a 3D reference; lookup and conditional functions do not, per the search summary of Microsoft documentation. `[CHECK]` for SUMIF, COUNTIF, XLOOKUP in M365 before filming.
- Teaching warning: `=SUM('Type A:Type C'!B3)` adds `B3` on three sheets; it is not a sum of the whole block.

### 3.3 Defined names and tables `[K]`
- Name a range (e.g. `Flaechen = B2:B4`) and use it in formulas: `=SUM(Flaechen)`.
- Table column references: `=SUM(Tabelle1[Flaeche])`. Tables expand when rows are added, so references stay correct.

---

## 4. Functions relevant to the clips

| Function (EN) | Function (DE) | Purpose | Quiz link | Status |
|---|---|---|---|---|
| SUM | SUMME | Adds numbers. | Quiz `command-sum`: 12+18+30 = 60 | `[V]` quiz; DE name `[V]` (quiz provides SUMME) |
| AVERAGE | MITTELWERT | Arithmetic mean. | Quiz `command-average`: 60/3 = 20 | `[V]` quiz (DE MITTELWERT in quiz prompt) |
| COUNT | ANZAHL | Counts numeric cells only. | Quiz `command-count`: text excluded | `[V]` quiz (DE ANZAHL in quiz prompt) |
| MIN | MIN | Smallest number. | Quiz `command-minimum`: 12 | `[V]` quiz |
| MAX | MAX | Largest number. | Quiz `command-maximum`: 30 | `[V]` quiz |
| SUMPRODUCT | SUMMENPRODUKT | Multiplies matching elements of arrays, then sums. | Ratio/product teaching | `[K]` DE name (common knowledge; `[CHECK]` in DE UI) |
| COUNTIF | ZÄHLENWENN | Counts cells meeting a criterion. | Filter/criteria teaching | `[K]` DE name |
| IF | WENN | Conditional value. | `[K]` | `[K]` DE name |
| ROUND | RUNDEN | Rounds to given digits. | `[K]` | `[K]` DE name |
| XLOOKUP | XVERWEIS | Looks up a value and returns a matching entry. Not available in Excel 2016 or 2019. | `[V]` Microsoft DE page search result (support.microsoft.com/de-de/excel/get-started/xlookup-function) | `[V]` |

Argument order (EN): `XLOOKUP(lookup_value, lookup_array, return_array, [if_not_found], [match_mode], [search_mode])`. `[K]`

Notes:
- Quiz `spreadsheet-commands` prompt: "English function names are notation; SUM, AVERAGE, COUNT, MIN and MAX correspond to SUMME, MITTELWERT, ANZAHL, MIN and MAX in German Excel." This is the authoritative pairing for the clips.
- Operations that sort or filter are not functions: Sort (Sortieren) and Filter (Filtern) act on rows, and filters keep original order while hiding rows (quiz `sort`, `filter`).

### 4.1 Sort and filter rules (quiz `spreadsheet-commands`)
- Sorting full rows keeps each record's values together `[V]` quiz.
- Filtering hides rows; it does not delete them and does not reorder them `[V]` quiz.

### 4.2 Transpose (Transponieren) `[V]` quiz
- Horizontal values 12, 18, 30 become a vertical three-row single-column selection, order kept.

---

## 5. German UI names and separator

- Workbook: Arbeitsmappe. Worksheet: Arbeitsblatt / Tabellenblatt. Sheet tab: Register. Cell: Zelle. Range: Bereich. Formula: Formel. Reference: Bezug. Function: Funktion. Name Box: Namenfeld. Formula bar: Bearbeitungsleiste. Fill handle: Ausfüllkästchen. Relative reference: relativer Bezug. Absolute reference: absoluter Bezug. Mixed reference: gemischter Bezug. Spill range: Überlaufbereich (`[CHECK]` exact wording).
- Argument separator: German Excel uses `;` in formulas, for example `=SUMME(B2:B4)` and `=WENN(A1>0;"ja";"nein")`. English uses `,`. Teaching: say "Semikolon" in DE captions.
- Decimal separator in DE Excel: comma (0,25). In formulas typed into DE Excel, use the local decimal comma.
- Function names used in clips: SUMME, MITTELWERT, ANZAHL, MIN, MAX (quiz); SUMMENPRODUKT, ZÄHLENWENN, WENN, RUNDEN, XVERWEIS (general German Excel names, `[K]`, XVERWEIS `[V]`).
- Translation tool: Microsoft's own "Functions Translator" add-in (Microsoft Garage) exists (search result, `[V]`).
- `[CHECK]` The German Microsoft "Excel-Funktionen Englisch Deutsch" table page was not reachable in this session. Verify all DE names above in the German M365 UI.

---

## 6. Errors, recalculation and dependencies

### 6.1 Error values `[K]` (classic set, verify wording in DE UI)
| Error | Meaning | DE name | Typical cause |
|---|---|---|---|
| `#REF!` | Invalid reference | `#BEZUG!` | Referenced cell was deleted |
| `#DIV/0!` | Division by zero | `#DIV/0!` | Divisor is 0 or empty |
| `#NAME?` | Unknown name | `#NAME?` | Misspelled function or undefined name |
| `#VALUE!` | Wrong type | `#WERT!` | Text used where number expected |
| `#N/A` | Value not found | `#NV` | XLOOKUP/MATCH with no match |
| `#SPILL!` | Spill blocked | `#ÜBERLAUF!` | Cells in the spill range are not empty `[V]` Microsoft spill page |
| Circular reference | Formula depends on itself | Zirkelbezug | Excel warns and may show 0 |

- `[CHECK]` the DE error labels (`#BEZUG!`, `#WERT!`, `#NV`, `#ÜBERLAUF!`) in the German UI before filming.

### 6.2 Dynamic arrays and spill `[V]` Microsoft (fetched)
Source: https://support.microsoft.com/en-us/office/dynamic-array-formulas-and-spilled-array-behavior-205c6b06-03ba-4151-89a1-87a7eb36e531
- A formula that returns multiple values places them in neighbouring cells; you enter it once in the top-left cell and Excel sizes the range.
- Only the top-left cell is editable; other spill cells show the formula read-only, with a border around the range.
- #SPILL! means the output range is blocked by existing data; remove or move the blocking data.
- Legacy CSE array formulas return a fixed-size result and do not spill.

### 6.3 Recalculation and dependencies `[K]`
- Automatic calculation (Formeln > Berechnungsoptionen > Automatisch): changed inputs recalculate dependents.
- Precedents (Spuren zum Vorgänger): cells the active formula uses. Dependents (Spuren zum Nachfolger): cells that use the active cell. Shown with trace arrows.
- Pasted values break the dependency chain (quiz `command-values`).
- `[CHECK]` DE menu names for trace arrows.

### 6.4 Numbers and units `[K]`
- Number formats change display only. Units can be displayed through a custom format (e.g. `0.00 "m²"`), but the stored number stays unitless. Teaching point: the unit is a label, not a calculation.

---

## 7. Didactic misconceptions to address in the video (10)

1. A cell displays what its format shows; 25% is stored as 0.25 (quiz `command-format`).
2. Pasting values removes the formula; the result is frozen and no longer updates (quiz `command-values`).
3. Copying a relative reference shifts both column and row by the copy offset; compute the offset first (quiz copying-references).
4. `$` does not protect against inserting or moving cells; it only matters for copying (quiz copying-references prompt).
5. Sorting must keep whole rows together; sorting one column alone breaks the pairs (quiz `command-sort`).
6. Filtering hides rows and does not delete them, and the original order is retained (quiz `command-filter`).
7. COUNT counts numbers only; text labels are excluded (quiz `command-count`).
8. AVERAGE is total divided by count of numeric cells, not by the number of rows (quiz `command-average`).
9. A 3D reference `Sheet1:Sheet3!B3` adds one cell from three sheets, not a block sum; not all functions accept it.
10. A spill error (#SPILL!) means blocked output, not a broken formula; clear the blocking cells.

Additional common errors to mention: `;` versus `,` separators between DE and EN versions; confusing `#NAME?` (spelling) with `#VALUE!` (type).

---

## 8. Sources

- Microsoft Support, "Create a 3-D reference to the same cell range on multiple worksheets": https://support.microsoft.com/en-us/excel/create-a-3-d-reference-to-the-same-cell-range-on-multiple-worksheets (search result, page content confirmed in summary)
- Microsoft Support, "Dynamic array formulas and spilled array behavior": https://support.microsoft.com/en-us/office/dynamic-array-formulas-and-spilled-array-behavior-205c6b06-03ba-4151-89a1-87a7eb36e531 (fetched)
- Microsoft Support, "XLOOKUP function" (German): https://support.microsoft.com/de-de/excel/get-started/xlookup-function (search result)
- Microsoft Support, "Switch between relative, absolute and mixed references": the URL I tried returned 404; `[CHECK]` for the current page before citing.
- Microsoft Garage, Functions Translator add-in: https://www.microsoft.com/en-us/garage/profiles/functions-translator (search result)
- Quiz sources (repo): `🎓️teaching/🏛️architecture/📊️excel/❓️quiz/🔣️.json`.

Not verified in this session (flagged `[CHECK]`): exact German translation of every function (a Microsoft-authoritative table was not reached); spill operator `#` and implicit intersection `@` details; F4 cycling behaviour in current M365; German error labels; 3D reference support list for SUMIF/COUNTIF/XLOOKUP in current M365.
