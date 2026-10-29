# XL-ENGINE report

Status: ready. `api.md` section 5 holds the final signatures and the cell map of the Nakagin workbook.

## Files (X = `🎓️teaching/🏛️architecture/📊️excel/🎬️clip`)

- `X/🧱️model/🐍️.py` stdlib-only engine: A1 references (`Ref`, `Range`, 3D spans, whole rows/columns, quoting), `translate`, `cycle_reference` (F4), tokenizer and parser (Excel precedence), evaluator with Tarjan recalculation order, circular detection, spill, `precedents`/`dependents`, 22 functions, `localize`/`canonicalize`, `display`, event-sourced `apply`.
- `X/🧱️model/🔣️.json` Nakagin workbook (7 sheets, 104 inputs bound to `🏢️nakagin` paths, capsule list from `slots()`), `X/🧱️model/🧬️schema/🔣️.schema.json`.
- `X/🧪️cases/🔣️.json` portable cases, `X/🧪️tests/🐍️.py` (63 tests), `X/🧪️tests/🟨️.cjs` (AJV + independent JS oracles).
- Ticket tools: `T/xl-engine/probe_formulas.py` is the oracle probe. `build_workbook.py` is retired (deleted): the workbook JSON is the source of truth and is edited directly. Generated `.xlsx` lands in `T/🗑️generated/xl-engine/Nakagin.xlsx`.

## German tab names (coordinator fix)

Sheet tabs are user data: JSON `title` {en, de} per sheet (canonical `name` stays English): Programm, Typ A-D, Klima, Energie. `localize(formula, "de", wb=None)` rewrites sheet references (`'Type A:Type D'!B13` to `'Typ A:Typ D'!B13`, `Program!B4` to `Programm!B4`, quoting recomputed), `canonicalize` is the inverse, `tab_name` / `sheet_titles` give the tab strip. Defined names stay as they are. New tests: tab labels, every workbook formula round trips through German, and a German `.xlsx` export (`Nakagin-de.xlsx`, German tab names and names) recalculated by `formulas` equals our values.

## Text constants are bilingual labels

Every string constant is now a `{en, de}` label (Typ A-D on Program A4:A7, B10:E10, A15, Energy A4:A7; tower A/B). The capsule list uses column `labels` maps (type Type A to Typ A, facade E to O, tower) and the id is a formula `=I2&"-"&IF(J2<10,"0","")&J2&"-"&K2&"-"&L2` so German ids read A-01-O-1. `workbook(language="de")` holds the German texts as cell values, so COUNTIF criteria, XLOOKUP and ids match inside the German workbook; `workbook()` (en) is unchanged. Tests: no string `value` constants remain (Python and JS), German values equal English numbers, and the German `.xlsx` export is recalculated by `formulas` against `evaluate(workbook(language="de"))`, strings included.

## Excel tables and structured references

Program carries the table `Capsules` (H1:N141; canonical English name, German name `Kapseln` from the JSON `table.title`, headers are the bilingual labels). `localize`/`canonicalize` map `Capsules[Type]` to `Kapseln[Typ]` and back, bare `Capsules` to `Kapseln`; the German `.xlsx` export carries the table as `Kapseln`. Supported: `Capsules[Col]`, `Capsules[@Col]`, `[@Col]` inside the table, `Capsules[[#This Row],[Col]]`, `[#Headers]`, `[#All]`, `[#Data]`, `Capsules[[A]:[B]]`, bare `Capsules`; they work inside COUNTIF/COUNTIFS/SUMIF/SUMPRODUCT/XLOOKUP/ROWS/COLUMNS/INDEX, the dependency graph (precedents/dependents/order), `reference_spans(formula, wb, sheet, cell)` for coloured frames, `localize`/`canonicalize` (`Capsules[Typ]`, `[#Kopfzeilen]`, `[[#Diese Zeile];[Typ]]`) and events (insert/delete row or column grow, shrink or shift the table; `define-table`, `delete-table`; rename-sheet keeps it). The Program formulas, the area column and the id column now use structured references. The German special items (`#Kopfzeilen #Alle #Daten #Diese Zeile #Ergebnisse`) are only partly confirmed (an article confirms #Diese Zeile and `;`): check in the German UI.

The `formulas` library cannot parse structured references (`FormulaError` on `Capsules[Type]`), so the oracle compares against the expanded A1 form produced by an independent expander in the test (also for the German export); the structured form is written to `Nakagin-structured.xlsx` with an openpyxl table object and re-read by openpyxl (table ref and formula text preserved) but not recalculated.

## Number formats

Totals from 1,000 use thousands separators (Program D8, E8, B16, F13 and Energy E8 now `#,##0.0 "m²"`/`"m³"`; kWh/a, W and K·d were already `#,##0`); a test requires `#,##0` for every numeric cell of at least 1,000 and the ICU check covers `#,##0` and `#,##0.0` (182,901 / 182.901, 1,223.6 / 1.223,6).

## German area terms

Net floor area is `Nettogrundfläche` everywhere in German labels and headers (Program C3, F10, B14, A16 `Gesamte Nettogrundfläche`, Energy E3, table header, type sheets) and free floor area stays `Freie Fläche` (Type sheets A16). English texts and canonical formulas are unchanged. Widths grew where the longer word needs it (Program A 26, B 17, C 17, N 17; Energy E 17). The layout test now allows rows up to 30 because the program scene added copy/fill demo rows 21-27 to Program.

## Coordinator additions covered

ROWS/COLUMNS (ZEILEN/SPALTEN), COUNTA, set/clear cell, insert/delete row and column, delete/move/rename/add sheet (3D span shrinks, endpoint moves inwards when it leaves), paste values, defined names, German errors, unknown functions (SUMM, English SUM in German text) give #NAME?, `Table[Column]` tokens stay untouched in translate/localize and evaluate to #NAME?, F4 helper.

## German error names

Microsoft German support page confirms `#DIV/0!` (https://support.microsoft.com/de-DE/Excel/how-to-correct-a-div-0-error). `#BEZUG!`, `#WERT!`, `#NV`, `#ZAHL!` come from a German guide (https://www.weka.ch/themen/finanzen-controlling/controlling/excel-for-finance/article/fehlermeldungen-excel-schnell-erkennen-und-korrekt-handeln/), secondary source. `#ÜBERLAUF!` was not confirmed by any fetched source (the German Microsoft spill page returned 404): check in the German UI before filming.

## Oracles

- openpyxl: column letters, `range_boundaries`, `coordinate_from_string`, `quote_sheetname`, `Translator` (14 fixed + 400 random copy cases; off-grid cases must give `#REF!` where openpyxl raises).
- formulas 1.3.4: 60+ flagged formula cases and every formula cell (250+) of the Nakagin workbook written with openpyxl (incl. defined names, XLOOKUP, COUNTIFS, array SUMPRODUCT). `formulas` cannot parse 3D references: the oracle expands `SUM('Type A:Type D'!B13)` to four single refs; the 3D semantics themselves are tested by event cases. `formulas` is case-sensitive for `COUNTIF(...,"b*")` (Excel is not), so that case has no oracle flag.
- JS: AJV strict schema, 17 invalid mutations, bound inputs equal `🏢️nakagin/🔣️.json` leaves, independent copy translation, ICU `Intl.NumberFormat` en-US/de-DE.
- Nakagin cross-check: counts, areas, every envelope surface, H_T, H_V, load, annual demand, per m², HDD and building totals equal `🏢️nakagin` `totals()/heating()/envelope()/climate()`.

## Test result

```
Ran 84 tests in 34.287s  OK
Excel schema: authored workbook accepted; 18 invalid workbooks rejected.
Excel workbook: 108 inputs equal the shared Nakagin model values.
Excel A1: 10 copy translations reproduced by an independent implementation.
Excel display: 5 number formats equal ICU en-US and de-DE output.
```

## Notes and limits

- Verified after the `build_workbook.py` reruns: the references scene's widths (E 17, F 13 on all type sheets), the program scene's Program A17:B18 (building count and check) and the Occupants rows A17:B17 on the type sheets are present; the anatomy scene removed its own scratch rows. Nothing had to be restored.
- Layout: block in A:G, rows 1:16 on every sheet (fittings area in G); the 140-row capsule list sits in H:N. Type sheets share one layout for B-column formulas so `SUM('Type A:Type D'!B13)` is meaningful; Type B has three fittings (row 15 empty).
- Facade codes in the list are `N/E/S/W` (language neutral); storeys are 1-based capsule storeys, trees in the Nakagin model are 0-based.
- Circular cells evaluate to 0 (Excel behaviour); `circular_cells` reports them. Spilled children are written after their anchor; formulas that read spilled cells are ordered statically and may need the spill anchor first.
- After Nakagin data changes, edit `🔣️.json` by hand (bound values and counts) and run the tests: they fail if bindings drift.
