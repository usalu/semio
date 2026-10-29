# Verify: excel / 📋️program / `Beat3_Program` (final, after fix round)

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`
Audit checked: `T/audit-excel-program.md`
Fresh frames (captured this run, 24 clauses x 2 languages, all 48 PNGs read):
`T/🗑️generated/frames/📊️excel/Beat3_Program/en/` and `.../de/`
Capture logs: `T/🗑️generated/cap-en-verify2.log`, `T/🗑️generated/cap-de-verify2.log`
Note: the en run printed a cp1252 UnicodeEncodeError on its final status line after `layout.md` was written; frames and layout are complete. The de run used `PYTHONIOENCODING=utf-8`.

## Verdict

NOT READY. No new blocker. All three audit blockers are fixed. Three majors remain (two are clipping, one is an overlap), and the audit's header-clipping major is still partly open in English.

## Audit blockers and majors: status

| audit item | status | frame that shows it |
| --- | --- | --- |
| Program D5:D8 / E8 show `###` (chart) | FIXED | chart en, de: Total column reads 314.6 m², 1,223.6 m² |
| Same `###` (update) | FIXED | update en, de: 1,223.6 m² readable |
| Volume dimension 2.50 m vs formula 2.10 m (blocker) | FIXED | volume en: "2.10 m" on the interior height, B7 red frame; de: "2,10 m" |
| No interior volume box, no B9/B7 reference frames (major) | FIXED | volume en, de: tinted box drawn; B9 blue and B7 red frames present |
| Program B3 "Anzahl" clipped in de (major) | FIXED | hook, rest, pull, countif de: "Anzahl" in full |
| countifs/towers de F10 "Nettofläche" clipped (major) | FIXED | towers de: "Nettofläche" in full |
| countifs en B10 "Type A" clipped (major) | FIXED for B10 | countifs en: B10 "Type A" in full |
| ... but the neighbour C10 "Type B" is now clipped to "Type..." | PARTLY | countifs en and towers en: C10 "Type..." |
| Trailing space invisible (major) | FIXED | typo en, de: red block marker after "Type B" in M3, label "mark = space" |
| Error red = Type D red (major) | FIXED | Type D is now purple in types, rest, typo, fix, recap; red only on FALSE/FALSCH badges |
| Tower tints clash with type colours (major) | FIXED | towers en, de: grey/white tower tints |
| Trace arrows strike the values (update, major) | FIXED | update en, de: arrows run under the count cells, no text struck |
| sumproduct operands off-window and unframed (major) | FIXED | sumproduct en, de: B4:B7 (blue) and C4:C7 (red) framed in window |

Fact findings: "In our teaching model" is now in the hook (en and de). The sheet label "Interior height" / "Innenhöhe" now matches the narration. Both fixed.

Didactic finding "the trailing-space cause is not visible": fixed by the red marker.

## NEW majors

1. **Table clause, header row hidden (en, de).** In `table` en and de, the expanded two-line formula bar covers the header row of the Program table: "Storey", "Facade", "Item" are half hidden (de: "Geschoss", "Fassade", "Platz", "Typ"). The same bar also covers column letters C to F in `countifs` and `towers` (en, de). Likely the same toolkit cause as the formula-bar overflow the audit assigned, but here it hides data headers. Fix: keep the formula bar to one line for this clause, or move the sheet window down so the header row sits below the bar.
2. **Towers de, row label clipped.** `towers` de: A13 "Summe" shows as "Sum..." (column A too narrow). Fix: widen column A or let the label wrap.
3. **Countifs/towers en, header clipped (PARTLY from audit item 7).** C10 "Type B" shows as "Type...". Fix: widen column C in the English sheets (the German sheet has the same width and shows "Typ B" in full).

## Minor items still open (not blocking)

- update en, de: "+1" and "-1" chips float far from B4 and B6 (unchanged).
- volume de: number format popup shows `0.0 "m³"` with a decimal point (unchanged).
- typo de: label "Marke = Leerzeichen" touches the chip "139 ≠ 140".
- recap en, de: term chips are INK, not their frame colours (unchanged).
- sumproduct en, de: tile labels use 8.74 m² while the sheet shows 8.7 m² (mixed precision).
- countifs en and towers de: formula bar overflow (assigned to toolkit, unchanged).

## Numbers

All numbers are consistent across frames and match the Nakagin model (`🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json`, read directly):
- Counts 36/35/34/35 = 140 (model values); tower counts 14/22 etc. consistent with the chart.
- Fitting areas: A 3.60, B 4.32, C 5.48, D 3.60 m²; free: A 5.14, B 4.42, C 3.26, D 5.14 m², shown as 5.1, 4.4, 3.3, 5.1 in both languages.
- Net area 2.30 x 3.80 = 8.74 m², shown as 8.7 m²; volume 18.354 m³, shown as 18.4 m³ with unit.
- Total 1,223.6 m² = 472.0 m² (tower A) + 751.6 m² (tower B); 139 = FALSE and 140 = TRUE after the typo and the fix.
- Every number carries a unit (m², m³, or a count).

German: Excel names (ZÄHLENWENN, ZÄHLENWENNS, XVERWEIS, SUMMENPRODUKT, SUMMEWENN, ZEILEN, UND, WENN), `;` separators, decimal commas, tabs (Programm, Typ A to D, Klima, Energie), status Bereit, WAHR/FALSCH: all correct.

## Must still change before READY

1. Widen column C in the English sheets so "Type B" is not clipped (countifs en, towers en, C10).
2. Widen or wrap column A in the German towers sheet so "Summe" is not clipped (towers de, A13).
3. Stop the expanded formula bar from hiding the Program header row and the column letters in the table, countifs and towers clauses (table en, de; countifs and towers en, de).

Items 1 to 3 are the only changes needed for READY. The minor items can follow.

Report: `T/verify-excel-program.md`
