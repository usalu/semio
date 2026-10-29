# Storyboard: Excel / Anatomy (`excel_anatomy`, `Beat1_Anatomy`)

File: `X/🦴️anatomy/🐍️.py`. Title: en "The Anatomy of a Workbook" · de "Aufbau einer Arbeitsmappe". Example: Nakagin Capsule Tower, "Type A" sheet. Screen split: left = Nakagin drawing (capsule axon, later tower), right = Excel window (`xl_window`). Both are on screen whenever a value is discussed.

## Learning goals

1. Name the parts of a workbook and read an address: workbook, worksheet, column, row, cell, range, Name Box, formula bar.
2. Distinguish a constant, a formula and the displayed value (stored 2.5, shown "2.50 m").
3. Write a first formula with references, operators (precedence, parentheses) and a function (SUM / SUMME, arguments, `,` versus `;`).
4. Explain recalculation and trace precedents and dependents.
5. Recognise the four classic errors (#DIV/0!, #NAME?, #REF!, #VALUE!) by their cause and repair them.

## Misconceptions addressed (research-excel.md section 7)

- A cell shows what its format shows: 2.5 may be shown as "2.50 m", the stored value stays 2.5 (misconception 1, section 6.4: the unit is a label, not a calculation).
- A formula starts with `=`; typing the same text without `=` is only text (feeds the #VALUE! case).
- Operator precedence: `2*B3+C3` is not `2*(B3+C3)`; the drawing shows which outline results.
- English `SUM(B9:B11,1.5)` is `SUMME(B9:B11;1,5)` in German Excel (separator and decimal comma).
- #NAME? is a spelling problem, #VALUE! a type problem, #REF! a deleted source, #DIV/0! an empty or zero divisor (section 7, additional).
- Pasted values are not shown here (that belongs to the references clip); only the dependency chain is shown.

## Fixed layout of the example sheet "Type A" (values come from the engine / `nakagin` model, never typed in the scene)

| Cell | Content | Meaning on the capsule |
| --- | --- | --- |
| A1 | text "Type A" | title |
| B2:D2 | text Width, Length, Height (Breite, Länge, Höhe) | column headings |
| A3 | text "Dimensions (m)" | label |
| B3, C3, D3 | constants 2.5 / 4.0 / 2.5 formatted `0.00 "m"` | width, length, height dimension lines |
| B5 | `=B3*C3` floor area (m²) | hatched capsule floor |
| B6 | `=2*(B3+C3)` perimeter (m) | floor outline |
| B7 | `=B3*C3*D3` volume (m³) | box volume tint |
| A9:A11, B9:B11 | fittings and their areas | three hatched fittings on the floor |
| B12 | `=SUM(B9:B11)` fittings area | the three hatches together |

Colour roles: width = DATA yellow, length = ARCH cyan, height = STRUCT beige (the same colour on the dimension line, the cell frame and the formula token); result = GOOD green; error = SERV red; text / labels = MUTED.

## Narration clauses (23)

Both languages are authored natively. "Teaching model" is said once (clause `hook`).

| # | Key | English | German | Tool side (workbook) | Building side (Nakagin) | Value and how it is drawn | Animation |
| - | --- | --- | --- | --- | --- | --- | --- |
| 1 | `hook` | In our teaching model the Nakagin tower has 140 capsules in four types. How much floor does one capsule give, and what happens when its width changes? | Im Lehrmodell hat der Nakagin-Turm 140 Kapseln in vier Typen. Wie viel Fläche bietet eine Kapsel, und was passiert, wenn sich ihre Breite ändert? | empty workbook frame fades in at the right, dimmed | tower axon with the four types in four tints, capsule count chip | 140 capsules (model count) as a chip; type tints | tower fades in, types tint one after the other |
| 2 | `workbook` | A workbook is the whole file, here Nakagin.xlsx: the calculation of the entire building. | Eine Arbeitsmappe ist die ganze Datei, hier Nakagin.xlsx: die Berechnung des ganzen Gebäudes. | window title bar with file name lights, rest of window visible | whole tower outlined | file name `Nakagin.xlsx` / `Nakagin.xlsx` | outline pulses around tower and window together |
| 3 | `sheets` | Inside it, worksheets sit as tabs: Program, one sheet per capsule type, Climate and Energy. | Darin liegen Arbeitsblätter als Register: Programm, je Kapseltyp ein Blatt, Klima und Energie. | sheet tabs appear at the bottom, Type A..D tabs coloured | each type shown on the tower in its tab colour | tab names from the workbook model in the active language | tabs slide in one after the other, tower tint follows tab |
| 4 | `grid` | One worksheet is a grid: columns have letters, rows have numbers. Type A describes one capsule. | Ein Arbeitsblatt ist ein Raster: Spalten haben Buchstaben, Zeilen Nummern. Typ A beschreibt eine Kapsel. | Type A sheet active; column letters A..F and row numbers 1..12 pulse; sheet content faded in | capsule A axon at 100 % scale, no dimensions yet | capsule A (its width, length, height from model) | headings pulse, capsule axon draws in |
| 5 | `address` | A cell is named by column and row. B3 is the width of the capsule: 2.5 metres. | Eine Zelle heißt nach Spalte und Zeile. B3 ist die Breite der Kapsel: 2,5 Meter. | click B3: cell pops out (scale 1.2), Name Box shows B3 | width dimension line lights on the capsule | 2.50 m (en) / 2,50 m (de), yellow dimension line + yellow cell frame | cell pop, dimension line grows |
| 6 | `range` | A range joins cells: B3:D3 holds width, length and height. | Ein Bereich fasst Zellen zusammen: B3:D3 enthält Breite, Länge und Höhe. | selection B3:D3 (green border), Name Box shows B3:D3 | three dimension lines light: width yellow, length cyan, height beige | 2.50 m / 4.00 m / 2.50 m, three colours | border stretches from B3 to D3, two more dimension lines light |
| 7 | `bars` | The Name Box shows the address; the formula bar shows what the cell contains. | Das Namenfeld zeigt die Adresse; die Bearbeitungsleiste zeigt den Inhalt der Zelle. | Name Box and formula bar highlighted with labels (Name Box / Namenfeld, formula bar / Bearbeitungsleiste); C3 selected: C3 / 4 | length dimension lights | C3, 4 (stored) | two callout labels, C3 selection moves |
| 8 | `constant` | Typed numbers are constants. Text such as Width is also a constant: Excel stores it as it is. | Eingetippte Zahlen sind Konstanten. Auch Text wie Breite ist eine Konstante: Excel speichert ihn unverändert. | B3 and B2 selected in turn; formula bar shows 2.5 and Width/Breite; grid cells tinted constant-grey, numbers right-aligned, text left-aligned | width line stays lit | 2.5 and text, alignment as the format rule | selection hops B2 → B3 |
| 9 | `display` | The cell shows 2.50 m, but it stores 2.5. The format changes the display, not the value. | Die Zelle zeigt 2,50 m, speichert aber 2,5. Das Format ändert die Anzeige, nicht den Wert. | split view: displayed value `2.50 m` above stored `2.5` (formula bar), format chip `0.00 "m"` | width dimension label reads 2.50 m | displayed vs stored, both from the engine `display()` | cell value text morphs between stored and displayed form |
| 10 | `formula` | A formula starts with an equals sign. Floor area is width times length: B3 times C3. | Eine Formel beginnt mit einem Gleichheitszeichen. Fläche ist Breite mal Länge: B3 mal C3. | B5 selected, formula bar types `=B3*C3`, cell shows 10.00 m² | capsule floor outline lights | floor area cell = engine value (B3*C3) | typing animation in the formula bar, result appears |
| 11 | `frames` | Excel colours each reference and frames its cell: the formula points at its sources. | Excel färbt jeden Bezug und umrahmt die Zelle: Die Formel zeigt auf ihre Quellen. | colour frames on B3 (yellow) and C3 (cyan), formula tokens `B3` and `C3` in the same colours | width line yellow, length line cyan | same colour on frame, token, dimension line | frames fade in, tokens recolour, lines pulse |
| 12 | `hatch` | The result is the floor of the capsule: 10 square metres, drawn as a hatched surface. | Das Ergebnis ist der Boden der Kapsel: 10 Quadratmeter, als schraffierte Fläche gezeichnet. | B5 shows the result in green | capsule floor hatched in green, area label over it | floor area, hatch spacing fixed, area tint = GOOD | hatch lines sweep in over the floor |
| 13 | `precedence` | Operators follow rules: multiplication comes before addition. For the perimeter we need brackets: 2 times (B3 plus C3). | Operatoren folgen Regeln: Punkt vor Strich. Für den Umfang brauchen wir Klammern: 2 mal (B3 plus C3). | B6 shows first the wrong `=2*B3+C3`, then `=2*(B3+C3)`; both results side by side | footprint outline: wrong result draws only one long side plus one short; the right result closes the whole outline | perimeter wrong vs right (both from the engine), red vs green outline | outline grows from wrong to right, formula text swaps with brackets highlighted |
| 14 | `function` | A function is a named calculation. SUM adds a range: the three fittings give their total area. | Eine Funktion ist eine benannte Berechnung. SUMME addiert einen Bereich: Die drei Einbauten ergeben ihre Gesamtfläche. | B12 `=SUM(B9:B11)` / `=SUMME(B9:B11)`; function name, parentheses and argument annotated | three fittings hatched on the capsule floor in three tints; total chip | fittings total from engine; each part its own hatch | the three hatches lie one after the other, then the total chip |
| 15 | `locale` | German Excel shows SUMME, separates arguments with a semicolon and uses a decimal comma. | Deutsches Excel zeigt SUMME, trennt Argumente mit Semikolon und nutzt das Dezimalkomma. | B12 formula toggles between English and German text with a second example `=SUM(B9:B11,1.5)` / `=SUMME(B9:B11;1,5)`; separator highlighted | capsule unchanged (the building does not care about the language) | the same formula in both languages from `localize()` | text swap with the differing tokens highlighted; in en narration it tells the German form, in de narration the English form |
| 16 | `recalc` | Change the width from 2.5 to 3.0 metres: every dependent cell recalculates, and the capsule follows. | Ändern wir die Breite von 2,5 auf 3,0 Meter: Alle abhängigen Zellen rechnen neu, und die Kapsel folgt. | B3 changes via ValueTracker, B5, B6, B7 update live | capsule widens, width line, hatch and outline follow | width 3.0 m (the scene edits the engine workbook through `apply`), area, perimeter, volume recomputed | ValueTracker ramp 2.5 → 3.0, cells and drawing redraw each frame |
| 17 | `trace` | Trace arrows show precedents, the cells a formula uses, and dependents, the cells that use this one. | Spurpfeile zeigen Vorgänger, die Zellen, die eine Formel nutzt, und Nachfolger, die Zellen, die diese Zelle nutzen. | B5 selected: arrows from B3 and C3 to B5; then B3 selected: arrows to B5, B6, B7 | the sources and results pulse on the capsule | precedents / dependents from the engine functions `precedents` and `dependents` | arrows draw, selection moves from B5 to B3 |
| 18 | `div0` | Typical errors have causes. An empty length makes a division by zero: #DIV/0!. | Typische Fehler haben Ursachen. Eine leere Länge führt zur Division durch null: #DIV/0!. | C3 cleared, a cell `=B5/C3` (area per metre of length) shows `#DIV/0!` (identical in German) in red | length dimension line dashed and red, hatch gone | error code from the engine | C3 content fades, error tag pops |
| 19 | `name` | A misspelled function gives #NAME?: SUMM is not a function. | Eine falsch geschriebene Funktion ergibt #NAME?: SUMM gibt es nicht. | B12 `=SUMM(B9:B11)` shows `#NAME?` (German `SUMM` also gives `#NAME?`) | fittings hatches go grey, total chip shows a question mark | error code from the engine | letter flips to the typo, error tag pops |
| 20 | `ref` | Delete a source and the reference breaks: #REF! in English, #BEZUG! in German. | Wird eine Quelle gelöscht, bricht der Bezug: #REF! im Englischen, #BEZUG! im Deutschen. | column D deleted, B7 shows `#REF!` / `#BEZUG!` | height dimension line is cut off and red, volume tint gone | error code localized by `display()` | column D slides out, error tag pops on B7 |
| 21 | `value` | Text where a number is expected gives #VALUE!, here the text "4 m" in the length cell. | Text, wo eine Zahl erwartet wird, ergibt #WERT!, hier der Text "4 m" in der Längenzelle. | C3 holds the text "4 m" (left-aligned, tag), B5 `=B3*C3` shows `#VALUE!` / `#WERT!` | length line shows a question mark, hatch gone | error code localized | the cell content is replaced by the text, error tag pops |
| 22 | `repair` | Repair the causes and the numbers return: 10 square metres, perimeter, volume, fittings. | Beheben wir die Ursachen, kehren die Zahlen zurück: 10 Quadratmeter, Umfang, Volumen, Einbauten. | C3 4.00 m again, errors vanish, values green again | capsule complete again, hatch back | all four values from the engine | errors fade, hatch and dimension lines return |
| 23 | `legend` | Remember the terms: workbook, worksheet, cell, reference, formula, function, Name Box and formula bar. | Merken Sie sich die Begriffe: Arbeitsmappe, Arbeitsblatt, Zelle, Bezug, Formel, Funktion, Namenfeld und Bearbeitungsleiste. | legend panel: eight terms en / de, each with a miniature of its location on the window | tower and capsule dimmed behind, the capsule keeps its hatch | terms with the English term first when `en`, German first when `de` | each term appears with a pointer to its place in the window |

## Notes for the implementation

- All numbers come from the engine (`workbook()`, `evaluate`, `display`, `localize`, `precedents`, `dependents`, `apply`); capsule dimensions and fittings from `nakagin.capsule_types()` / `program(t)`.
- Edits (width 3.0, deleted column, text "4 m") are applied as events with `apply(workbook, events)` and then evaluated; the scene never computes a cell itself.
- The width in clause 16 is a `ValueTracker` driving `always_redraw` of the cells and of the capsule drawing.
- Cells needed in the shared model JSON for `Type A` (my topic entry): B5, B6, B7, B9:B12 and one helper cell for the #DIV/0! demo (`=B5/C3`). Everything else comes from the capsule type data.
- Clause durations: 23 clauses at about 8 to 11 seconds = about 3.5 minutes.
