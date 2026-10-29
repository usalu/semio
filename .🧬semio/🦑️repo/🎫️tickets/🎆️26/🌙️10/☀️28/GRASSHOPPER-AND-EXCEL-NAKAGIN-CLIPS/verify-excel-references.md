# ✅ Verify: excel references (`Beat2_References`)

Scope: fresh keyframes in en and de (22 each), checked against `audit-excel-references.md`. Frames: `🗑️generated/frames/📊️excel/Beat2_References/{en,de}/<key>.png`. The scene source (`🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py`) was last changed at 21:11, before the capture at 23:07, so the frames reflect the current code.

## Verdict

**NOT READY.** The audit blocker (German sheet names) is fixed. Three of the seven audit majors are still open or only partly fixed. Stuck, sheet and deleted still show `###` in the Program Area column, and the sum row hides its value in fill2d and mixed. One minor count is hidden by the error icon in deleted.

Must change before READY:
1. Widen Program columns B and C in both languages so the area, total and `Summe m²` cells show their values (fixes stuck, sheet, deleted and the B9 total). Re-capture both languages.
2. Stop the error indicator in deleted B6 from hiding the count (`35`).
3. Check the 3D sum in deleted: 2,747 − 704 = 2,043, but the card says 2,042 W (en and de). Use unrounded values or display 2,043.

Minor items below can follow after these.

## Audit blocker and majors

| audit item | status | frame(s) |
| --- | --- | --- |
| Blocker: DE tab strip ignores `title.de` (`Type A` vs `'Typ A'!`) | FIXED | de sheet, stuck, threed, span, names, deleted, guard, recap, fixed, f4: tabs read Programm, Typ A–D, Klima, Energie, matching the formulas |
| Major: lights only highlights the current row | FIXED | en and de lights: all six faces lit at F9 |
| Major: stuck C6 shows `###` | NOT FIXED | en and de stuck: C4, C6, C8 and the D/E totals still show `###` |
| Major: deleted DE labels truncated (`#BEZ…`) | FIXED | de deleted: `#BEZUG!` complete in C6 and F4:F8 |
| Major: guard DE tag runs into `=` | FIXED | de guard: tag and formula separated |
| Major: repair tags read `F4` for G4 | FIXED | en and de repair: both rows tagged G4 (before/after; vorher/nachher) |
| Major: slide shows wrong values with no correct value | FIXED | en and de slide: card shows `= 100 W` and `≠ 120 W` next to the wrong values |
| Major: parentheses render as square brackets | FIXED | all frames: `W/(m²·K)`, `COUNTIFS(`, `ZÄHLENWENNS(`, `SUMME(`, `(theta_i-theta_e)` |

Audit's "stuck" and "sheet" issues are the same root cause as the new B9 finding below, so the column-width fix closes all of them.

## New majors

1. **Sum cell hidden (en and de).** `Sum m²` / `Summe…` B9 shows `###` in fill2d and mixed (en: `fill2d.png`, `mixed.png`; de: same keys). The total has no readable value and the unit is hidden. Same cause as stuck C4:C8. Fix with the column widths in item 1 above.

## Minor, still open

- deleted (en and de): warning icon covers the count in B6 (`3` visible, `35` hidden).
- hook (en and de): row 11 (`Transmission H_T`) cut at the window bottom. Cell texts truncated: `Einzelwohnung mit Bett, Na…`, `Rechte Seitenwa…`, `Rückwand zum K…`.
- All frames: top ~125 px empty above the window.
- sheet (en and de): thumbnail title `Nakagin.xlsx` overlapped by the search pill. DE header `Fläc…`, `Gesamt` and row label `Sum…` truncated.
- recap (de): the 3D row's rule text is set smaller than the others. The caption breaks as `Blatt-` / `und 3D-Bezüge`.
- Total thousands separators (`1223.6` / `1223,6`) could not be checked: totals show `###`.
- `von oben` / `von unten` labels are low-contrast on the dark stage.
- DE error tooltip (`Die Formel verweist auf eine nicht mehr vorhandene Zelle.`) still unverified in the German UI: mark `[CHECK]` before filming.
- Names are ASCII (`theta_i`, `theta_e`) in the workbook and narration, while the audit suggested θi/θe. Not changed.

## Fixed and checked (minor and German wording)

- f4: tags now `1st F4` … `4th F4` (de: `1. F4` …). The formula bar shows the dash in `=Klima!B2-Klima!B3`.
- span: the 3D bracket is visible from Type A to Type D, and Type B is shown greyed out after it is moved out.
- fill2d: the active cell's tower and type are labelled (`Tower B · Type D: 22`).
- names: the editor no longer overlaps the names card. Captions start with capitals (`Gleicher Wert, lesbar`).
- mixed: the formula editor stays inside the window.
- de wording: comma in `shift` (`kopiert, wird`), `Apostrophe` in sheet, `Das Blatt der Art C löschen`, `Das Dollarzeichen vergessen`, `Nach rechts und unten in einem Zug ausgefüllt`.

## Numbers and facts (engine and `nakagin` model)

- Program counts 36 / 35 / 34 / 35 (sum 140). Tower A 54 and Tower B 86 both match the fill2d table.
- Engine values match the frames: 10.0 m² roof, front wall 4.9 m², window 2.33 W/(m²·K), 20 K design difference. Losses 100, 120, 120, 120, 59, 62, 38 W.
- 3D heating load per type: 681 / 681 / 704 / 681 W; sum 2,747 W; without B 2,066 W (matches span).
- Without C: 2,042 W on the card. Subtracting the displayed C value (704 W) gives 2,043 W. This is the minor new issue above.

## Caveats

- The capture logs were written to file names that other agents also use in this ticket. The de run printed `UnicodeEncodeError` on its final summary line after writing all frames and `layout.md`. The frame and `layout.md` timestamps are the evidence. En frames: 23:07–23:21. De frames: 23:24–23:45.
- `layout.md` in en reports 51 strict findings in 15 of 22 frames. I did not triage these; this verdict rests on the PNGs, as the brief asks.
- Nothing was edited except this report. The capture wrote only under `🗑️generated/`.
